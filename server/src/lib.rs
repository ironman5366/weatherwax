mod config;
mod error;
mod providers;
mod types;

use axum::{
    response::sse::{Event, Sse},
    routing::get,
    Router,
};
use error::Result;
use std::time::Duration;
use tokio_stream::StreamExt as _;

use futures::stream::{self, Stream};

async fn invoke() -> Sse<impl Stream<Item = Result<Event>>> {
    let stream = stream::iter(vec![
        Ok(Event::default().data("hello")),
        Ok(Event::default().data("world")),
    ])
    .throttle(Duration::from_secs(3));

    Sse::new(stream).keep_alive(
        // Send a keep-alive every 15 seconds
        axum::response::sse::KeepAlive::new().interval(Duration::from_secs(15)),
    )
}

async fn serve() {
    let app = Router::new().route("/invoke", get(invoke));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
