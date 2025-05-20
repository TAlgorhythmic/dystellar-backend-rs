static CLIENT_ID: &str = env!("CLIENT_ID");
static CLIENT_SECRET: &str = env!("CLIENT_SECRET");

pub fn login_minecraft() -> Result<> {
    let auth_res = post_urlencoded("https://login.live.com/oauth20_token.srf", format!("client_id={CLIENT_ID}&client_secret={CLIENT_SECRET}&code={code}&grant_type=authorization_code&redirect_uri={redirect}")).await?;
}
