use std::{convert::Infallible, error::Error, fs, sync::{LazyLock, Mutex}};

use futures::{StreamExt, TryStreamExt};
use http_body_util::{combinators::BoxBody, BodyExt, StreamBody};
use hyper::{body::{Bytes, Frame, Incoming}, header::{CONTENT_DISPOSITION, CONTENT_LENGTH, CONTENT_TYPE}, Request, Response};
use tokio::fs::{try_exists, File};
use tokio_util::io::ReaderStream;

use crate::api::{routers::ROUTER, typedef::{BackendError, Mod}};

async fn download(_: Request<Incoming>, path: String) -> Result<Response<BoxBody<Bytes, Infallible>>, BackendError> {
    if try_exists(&path).await.map_err(|err| BackendError::new(err.to_string().as_str(), 500))? {
        let file = File::open(&path).await.map_err(|err| BackendError::new(err.to_string().as_str(), 500))?;
        let length = file.metadata().await.map_err(|err| BackendError::new(err.to_string().as_str(), 500))?.len();
        let reader = ReaderStream::new(file).map(|s| {
            match s {
                Ok(b) => Ok::<Bytes, Infallible>(b),
                Err(_) => Ok::<Bytes, Infallible>(Bytes::new())
            }
        });

        let stream = StreamBody::new(reader.map_ok(Frame::data));

        let stream_boxed = BodyExt::boxed(stream);

        println!("{path}");

        return Ok(Response::builder()
            .header(CONTENT_TYPE, "application/octet-stream")
            .header(CONTENT_LENGTH, length)
            .header(CONTENT_DISPOSITION, format!(r#"attachment; filename="{}""#, unsafe {path.get_unchecked(path.rfind('/').map(|u| u + 1).unwrap_or(0)..path.len())}))
            .status(200)
            .body(stream_boxed)
            .map_err(|err| BackendError::new(err.to_string().as_str(), 500))?
        );
    }

    Err(BackendError::new("No such file or directory", 404))
}

pub async fn register() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut router = ROUTER.lock().await;

    // Create dirs if missing
    let _ = fs::create_dir("repository");
    router.map("repository", "/download", Box::new(|req, path| {Box::pin(download(req, path))}))?;

    Ok(())
}
