pub mod ai;

pub use ai::*;

pub mod error;

use crate::invoke::invoke;
use crate::types::Provider;
use axum::{routing::get, Router};
use error::Result;
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_stream::StreamExt as _;

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Opts {
    host: &'static str,

    // Provider-specific options
    #[cfg(feature = "openai")]
    openai: providers::openai::OpenAIOpts,
}

pub struct State {
    opts: Opts,
}

pub async fn serve(providers: Vec<&dyn Provider>, opts: Opts) -> Result<()> {
    let state = Arc::new(State { opts });
    let app = Router::new()
        .route("/invoke", get(invoke))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(opts.host).await.unwrap();
    axum::serve(listener, app).await?;
    Ok(())
}
