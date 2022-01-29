use axum::{
    body::{boxed, Full},
    handler::Handler,
    Json,
    http::{header, StatusCode, Uri, Method},
    response::{IntoResponse, Response},
    routing::{get, Router},
};
use mime_guess;
use rust_embed::RustEmbed;
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, any};

use cargo_metadata::Metadata;
use crate::metadata;
use axum::extract::Path;
use crate::vision::g6::{MetadataTreeGraph};
use crate::metadata::readme;

pub struct Server {
    port: u16,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Server { port }
    }
    pub async fn run(&self) {
        let app = Router::new()
            .route("/api/profile/readme", get(profile_readme))
            .route("/api/metadata/:graph", get(metadata))
            .route("/", get(index_handler))
            .layer(
                CorsLayer::new()
                    // allow `GET` and `POST` when accessing the resource
                    .allow_methods(vec![Method::GET, Method::POST])
                    // allow requests from any origin
                    .allow_origin(any()),
            )
            .fallback(static_handler.into_service());

        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        println!("listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}


pub(crate) async fn metadata(Path(graph): Path<String>) -> impl IntoResponse {
    let output = metadata::exec().unwrap();
    let meta = MetadataTreeGraph {
        metadata: output
    };

    let res = match graph.as_str() {
        "graph" => {
            meta.graph()
        }
        "tree" => {
            meta.tree()
        }
        _ => {
            // /metadata/:any
            meta.meta()
        }
    };

    return Json(res);
}

pub(crate) async fn profile_readme() -> String {
    let profile_readme = readme().unwrap();
    return profile_readme;
}

async fn index_handler() -> impl IntoResponse {
    static_handler("/index.html".parse::<Uri>().unwrap()).await
}


async fn static_handler(uri: Uri) -> impl IntoResponse {
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
