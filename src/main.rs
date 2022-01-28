#[macro_use]
extern crate anyhow;

use crate::app::Server;

mod app;
mod db;
mod routes;
mod metadata;
mod vision;

#[tokio::main]
async fn main() {
    let server = Server::new(8080);
    server.run().await;
}
