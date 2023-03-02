use axum::{routing::get, Router};
use libloading::{Library, Symbol};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/a", get(|| async { "A" }))
        .route("/b", get(|| async { "B" }));

    let dylib = unsafe { Library::new("./target/debug/libdylib.so").expect("load library") };
    let add_route: Symbol<fn(router: Router) -> Router> =
        unsafe { dylib.get(b"add_route") }.expect("load symbol");

    let router = add_route(router);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
