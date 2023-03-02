use axum::{routing::get, Router};

#[no_mangle]
pub fn add_route(router: Router) -> Router {
    router.route("/c", get(|| async { "C" }))
}
