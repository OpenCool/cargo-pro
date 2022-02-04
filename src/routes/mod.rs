use axum::{
    body::{boxed, Full},
    http::{header, StatusCode, Uri},
    routing::get,
    response::{IntoResponse, Response},
    routing::Router,
};
use rust_embed::RustEmbed;

mod git;
mod metadata;
mod profile;

pub fn api_router() -> Router {
    // This is the order that the modules were authored in.
    git::router()
        .merge(metadata::router())
        .merge(profile::router())
        .route("/", get(index_handler))
}

async fn index_handler() -> impl IntoResponse {
    static_handler("/index.html".parse::<Uri>().unwrap()).await
}

pub async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();
    if path.starts_with("dist/") {
        path = path.replace("dist/", "");
    }
    StaticFile(path)
}

#[derive(RustEmbed)]
#[folder = "dist"]
struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();
        match Asset::get(path.as_str()) {
            Some(content) => {
                let body = boxed(Full::from(content.data));
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                Response::builder()
                    .header(header::CONTENT_TYPE, mime.as_ref())
                    .body(body)
                    .unwrap()
            }
            None => Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(boxed(Full::from("404")))
                .unwrap(),
        }
    }
}
