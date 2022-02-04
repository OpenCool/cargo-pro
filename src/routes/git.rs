use axum::{response::IntoResponse, routing::get, Json, Router};
use git2::Repository;
use std::env;

use crate::git::repository::Repo;

pub fn router() -> Router {
    Router::new().route("/api/git/logs", get(git_logs))
}

pub(crate) async fn git_logs() -> impl IntoResponse {
    let current_dir = env::current_dir().unwrap_or(".".into());
    let dis_repo = Repository::discover(&current_dir).unwrap();
    let rep = Repo::new(&dis_repo).unwrap();
    return Json(rep.cache.logs);
}
