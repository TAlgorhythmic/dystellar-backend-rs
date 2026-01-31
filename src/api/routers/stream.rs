use std::{convert::Infallible, error::Error, fs};

use futures::{StreamExt, TryStreamExt};
use http_body_util::{combinators::BoxBody, BodyExt, StreamBody};
use hyper::{body::{Bytes, Frame, Incoming}, header::{CONTENT_DISPOSITION, CONTENT_LENGTH, CONTENT_TYPE}, Request, Response};
use tokio::fs::{try_exists, File};
use tokio_util::io::ReaderStream;

use crate::api::typedef::{BackendError, routing::nodes::Router};

async fn download(_: Request<Incoming>, path: String) -> Result<Response<BoxBody<Bytes, Infallible>>, BackendError> {
    if try_exists(&path).await.map_err(|err| BackendError::new(err.to_string().as_str(), 500))? {
        let file = File::open(&path).await.map_err(|err| BackendError::new(err.to_string().as_str(), 500))?;
        let metadata = file.metadata().await.map_err(|err| BackendError::new(err.to_string().as_str(), 500))?;
        if metadata.is_dir() {
            return Err(BackendError::new("No such file or directory (os error 2)", 404));
        }

        let length = metadata.len();
        let reader = ReaderStream::new(file).map(|s| {
            match s {
                Ok(b) => Ok::<Bytes, Infallible>(b),
                Err(_) => Ok::<Bytes, Infallible>(Bytes::new())
            }
        });

        let stream = StreamBody::new(reader.map_ok(Frame::data));

        let stream_boxed = BodyExt::boxed(stream);

        return Ok(Response::builder()
            .header(CONTENT_TYPE, "application/octet-stream")
            .header(CONTENT_LENGTH, length)
            .header(CONTENT_DISPOSITION, format!(r#"attachment; filename="{}""#, unsafe {path.get_unchecked(path.rfind('/').map(|u| u + 1).unwrap_or(0)..path.len())}))
            .status(200)
            .body(stream_boxed)
            .map_err(|err| BackendError::new(err.to_string().as_str(), 500))?
        );
    }

    Err(BackendError::new("No such file or directory (os error 2)", 404))
}

async fn serve(_: Request<Incoming>, mut path: String) -> Result<Response<BoxBody<Bytes, Infallible>>, BackendError> {
    if try_exists(&path).await.map_err(|err| BackendError::new(err.to_string().as_str(), 500))? {
        if path == "static" || path == "static/" {
            path = "static/index.html".to_string();
        }

        let file = File::open(&path).await.map_err(|err| BackendError::new(err.to_string().as_str(), 500))?;
        let metadata = file.metadata().await.map_err(|err| BackendError::new(err.to_string().as_str(), 500))?;
        if metadata.is_dir() {
            return Err(BackendError::new("No such file or directory (os error 2)", 404));
        }

        let reader = ReaderStream::new(file).map(|s| {
            match s {
                Ok(b) => Ok::<Bytes, Infallible>(b),
                Err(_) => Ok::<Bytes, Infallible>(Bytes::new())
            }
        });
        let stream = StreamBody::new(reader.map_ok(Frame::data));

        let stream_boxed = BodyExt::boxed(stream);

        return Ok(Response::builder()
            .header(CONTENT_TYPE, mime_guess::from_path(path).first_or_octet_stream().to_string())
            .status(200)
            .body(stream_boxed)
            .map_err(|err| BackendError::new(err.to_string().as_str(), 500))?
        );
    }

    Err(BackendError::new("No such file or directory (os error 2)", 404))
}

pub async fn register(router: &mut Router) -> Result<(), Box<dyn Error + Send + Sync>> {
    // Create dirs if missing
    let _ = fs::create_dir("repository");
    let _ = fs::create_dir("static");

    router.map("repository", "/download", download)?;
    router.map("static", "/", serve)?;

    Ok(())
}
