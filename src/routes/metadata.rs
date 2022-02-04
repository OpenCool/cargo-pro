use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};

use crate::metadata;
use crate::vision::g6::MetadataTreeGraph;

pub fn router() -> Router {
    Router::new().route("/api/metadata/:graph", get(metadata))
}

pub(crate) async fn metadata(Path(graph): Path<String>) -> impl IntoResponse {
    let output = metadata::exec().unwrap();
    let meta = MetadataTreeGraph { metadata: output };

    let res = match graph.as_str() {
        "graph" => meta.graph(),
        "tree" => meta.tree(),
        _ => {
            // /metadata/:any
            meta.meta()
        }
    };

    return Json(res);
}
