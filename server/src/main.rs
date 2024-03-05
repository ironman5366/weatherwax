mod types;

use axum::{
    routing::get,
    Router,
    response::sse::{Event, Sse}
};
use std::{convert::{Infallible}};
use std::time::Duration;
use tokio_stream::StreamExt as _;

use futures::stream::{self, Stream};

async fn invoke(
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::iter(vec![
        Ok(Event::default().data("hello")),
        Ok(Event::default().data("world")),
    ]).throttle(Duration::from_secs(3));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
    )
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/invoke", get(invoke));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
