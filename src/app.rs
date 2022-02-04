use axum::{http::Method, handler::Handler};
use std::net::SocketAddr;
use tower_http::cors::{any, CorsLayer};

use crate::routes::{api_router, static_handler};

pub struct Server {
    port: u16,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Server { port }
    }

    pub async fn run(&self) {
        let app = api_router()
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
