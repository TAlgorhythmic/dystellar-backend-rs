use std::{collections::HashMap, convert::Infallible, sync::{Arc, LazyLock}, time::Duration};

use chrono::{Days, Utc};
use http_body_util::{combinators::BoxBody, Full};
use hyper::{body::{Bytes, Incoming}, Request, Response};
use json::object;
use tokio::sync::Mutex;

use crate::api::{control::{microsoft_lifecycle::{login_minecraft, login_minecraft_existing}, storage::query::{create_new_player, set_index}}, routers::{users::TOKENS, ROUTER}, typedef::{routing::Method, BackendError, MicrosoftTokens, SigninState}, utils::{get_body_json, get_body_url_args, response_json, HttpTransaction}};

static PENDING: LazyLock<Arc<Mutex<HashMap<Box<str>, SigninState>>>> = LazyLock::new(|| {Arc::new(Mutex::new(HashMap::new()))});

/**
* Endpoint used to create a session for oauth2 microsoft authentication, it's necessary to call
* this before logging in to microsoft in the frontend, otherwise when microsoft redirects to
* callback the backend won't find the state, and will result in an error.
*
* Method: POST
*
* Expects: body {
*   uuid: <provided uuid>
* }
* if no errors return: body {
*    ok: true
* }
*/
async fn loginsession(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>, BackendError> {
    let body = get_body_json(HttpTransaction::Req(req)).await?;
    let uuidopt = body["uuid"].as_str();
    if uuidopt.is_none() {
        return Err(BackendError::new("Malformed body", 400));
    }

    let uuid = uuidopt.unwrap().to_owned();

    let pend = &PENDING;

    let mut guard = pend.lock().await;

    guard.insert(uuid.clone().into(), SigninState::new());

    tokio::task::spawn(async move {
        tokio::time::sleep(Duration::from_secs(220)).await;
        
        let mut guard = pend.lock().await;
        guard.remove(uuid.as_str());
    });
    Ok(response_json(object! { ok: true }))
}

/**
* Endpoint used to fetch microsoft and minecraft account tokens, from an existing access_token or
* refresh_token
*
* Method: POST
*
* Expects: body {
*   access_token: string,
*   refresh_token: string,
*   expires_in: i64
* }
* if no errors return: body {
*    ok: true,
*    uuid: <minecraft account uuid>,
*    minecraft_token: <minecraft exchanged token>,
*    access_token: <microsoft oauth2 access_token, for later logins>,
*    refresh_token: <microsoft oauth2 refresh_token, for later logins in case the access_token is expired>,
*    expires_in: <time before expiring, provided by microsoft>
* }
*/
async fn login_existing(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>, BackendError> {
    let body = get_body_json(HttpTransaction::Req(req)).await?;

    let opt_access_token = body["access_token"].as_str();
    let opt_refresh_token = body["refresh_token"].as_str();
    if opt_access_token.is_none() || opt_refresh_token.is_none() {
        return Err(BackendError::new("Malformed request body", 400));
    }

    let tokens = MicrosoftTokens::new(opt_access_token.unwrap().into(), opt_refresh_token.unwrap().into());
    let user_credentials = login_minecraft_existing(tokens).await?;

    let _ = set_index(&user_credentials.name, &user_credentials.get_uuid());

    let tokens = TOKENS.clone();
    let mut tokens_map = tokens.lock().await;
    let cl = user_credentials.get_uuid().clone();
    tokens_map.insert(user_credentials.mc_token.clone(), (cl, Utc::now().checked_add_days(Days::new(1)).unwrap()));

    Ok(response_json(object! {
        ok: true,
        uuid: user_credentials.get_uuid().as_ref(),
        minecraft_token: user_credentials.get_minecraft_token().as_ref(),
        access_token: user_credentials.get_access_token().as_ref(),
        refresh_token: user_credentials.get_refresh_token().as_ref(),
        expires_in: *user_credentials.get_expiration()
    }))
}

/**
* Endpoint used to check login state as well as logging in with microsoft for the first time
* (without access_token/refresh_token)
*
* Method: GET
* Content-Type: urlencoded
*
* Expects: body: uuid=<state uuid>
* if no errors return: body {
*    ok: true,
*    authenticated: false,
*    uuid: <minecraft account uuid>,
*    minecraft_token: <minecraft exchanged token>,
*    access_token: <microsoft oauth2 access_token, for later logins>,
*    refresh_token: <microsoft oauth2 refresh_token, for later logins in case the access_token is expired>,
*    expires_in: <time before expiring, provided by microsoft>
* }
* if callback hasn't been called yet: body {
*    ok: true,
*    authenticated: false
* }
*/
async fn login(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>, BackendError> {
    let pend = &PENDING;
    let args = get_body_url_args(&req).await?;

    let arg = args.get("uuid");
    if arg.is_none() {
        return Err(BackendError::new("Invalid params", 400));
    }

    let mut guard = pend.lock().await;
    let uuid = arg.unwrap();
    let state = guard.get(uuid);

    if state.is_none() {
        return Err(BackendError::new("Login session expired.", 400));
    }

    let res = state.unwrap();
    let codeopt = res.get_code();
    if codeopt.is_none() {
        return Ok(response_json(object! { ok: true, authenticated: false }));
    }

    let code = codeopt.as_deref().unwrap();
    let session = login_minecraft(code).await?;
    guard.remove(uuid);
    
    // Try to create new player if it doesn't exist.
    if let Err(err) = create_new_player(session.get_uuid().as_ref(), session.name.as_ref()) {
        println!("Failed to create user in the database: {err}");
        return Err(BackendError::new("Backend internal error.", 500));
    }

    let _ = set_index(&session.name, &session.get_uuid());

    let tokens = TOKENS.clone();
    let mut tokens_map = tokens.lock().await;
    tokens_map.insert(session.mc_token.clone(), (uuid.clone(), Utc::now().checked_add_days(Days::new(1)).unwrap()));
    
    Ok(response_json(object! {
        ok: true,
        authenticated: true,
        uuid: session.get_uuid().as_ref(),
        minecraft_token: session.get_minecraft_token().as_ref(),
        access_token: session.get_access_token().as_ref(),
        refresh_token: session.get_refresh_token().as_ref(),
        expires_in: *session.get_expiration()
    }))
}

/**
* This endpoint will only be called by microsoft when the user finishes the oauth2 login to their
* microsoft account. It will update the login state of the user and set the unique code to later
* exchange for microsft tokens.
*
* Method: GET
* Content-Type: urlencoded
*
* Expects: body: state=<state uuid>&code=<code generated by microsoft>
* if no errors it will return a generic message saying that everything is okay
*/
async fn callback(req: Request<Incoming>) -> Result<Response<BoxBody<Bytes, Infallible>>, BackendError> {
    let args = get_body_url_args(&req).await?;

    let arg0 = args.get("code");
    let arg1 = args.get("state");

    if arg0.is_none() || arg1.is_none() {
        return Err(BackendError::new("Invalid url params.", 400));
    }

    let pend = &PENDING;
    let code = arg0.unwrap();
    let uuid = arg1.unwrap();
    
    let mut guard = pend.lock().await;
    let opt = guard.get_mut(uuid);
    if opt.is_none() {
        return Err(BackendError::new("Invalid state.", 400));
    }

    let signin_state: &mut SigninState = opt.unwrap();
    signin_state.set_authenticated(true);
    signin_state.set_code(code);

    Ok(response_json(object! { ok: true, msg: "Login successful! You can now close this tab." }))
}

pub async fn register() {
    let mut router = ROUTER.lock().await;

    router.endpoint(
        Method::Get, 
        "/api/microsoft/callback",
        Box::new(|req| {Box::pin(callback(req))})
    ).expect("Failed to register microsoft callback endpoint");

    router.endpoint(
        Method::Post,
        "/api/microsoft/login_existing",
        Box::new(|req| {Box::pin(login_existing(req))})
    ).expect("Failed to register login_existing endpoint");

    router.endpoint(
        Method::Get,
        "/api/microsoft/login",
        Box::new(|req| {Box::pin(login(req))})
    ).expect("Failed to register microsoft login endpoint");

    router.endpoint(
        Method::Post,
        "/api/microsoft/loginsession",
        Box::new(|req| {Box::pin(loginsession(req))})
    ).expect("Failed to register microsoft login session endpoint");
}
