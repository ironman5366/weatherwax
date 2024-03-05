pub mod ai;

pub use ai::*;

pub mod error;

use crate::invoke::invoke;
use crate::types::Provider;
use axum::{routing::get, Router};
use futures::stream::Stream;
use std::sync::Arc;
use tokio_stream::StreamExt as _;

#[derive(Copy, Clone, Debug)]
pub struct Opts {
    host: &'static str,

    // Provider-specific options
    #[cfg(feature = "openai")]
    openai_opts: providers::openai::Opts,
}

pub struct State {
    opts: Opts,
}

async fn serve(opts: Opts) {
    let state = Arc::new(State { opts });
    let app = Router::new()
        .route("/invoke", get(invoke))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(opts.host).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
