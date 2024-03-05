use axum::response::sse::{Event, Sse};
use std::time::Duration;
use tokio_stream::StreamExt as _;

use futures::stream::{self, Stream};

pub async fn invoke() -> Sse<impl Stream<Item = crate::error::Result<Event>>> {
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
