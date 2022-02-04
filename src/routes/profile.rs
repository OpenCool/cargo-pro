use crate::metadata::readme;
use axum::{routing::get, Router};

pub fn router() -> Router {
    Router::new().route("/api/profile/readme", get(profile_readme))
}

pub(crate) async fn profile_readme() -> String {
    let profile_readme = readme().unwrap();
    return profile_readme;
}
