use axum::{routing::get, Router};


use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use std::net::SocketAddr;

mod api;
mod model;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let (_handle, model) = model::Model::spawn();

    let app = Router::new()
        .route(api::text::ENDPOINT, get(api::text::text))
        .with_state(model)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000)); // TODO specify port on env
    tracing::info!("Listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
