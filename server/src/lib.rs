pub mod ai;

pub use ai::*;
use std::collections::HashMap;

pub mod error;

use crate::invoke::invoke;
use crate::types::{ModelsByCode, Provider, ProviderModel};
use axum::routing::post;
use axum::{routing::get, Router};
use error::Result;
use futures::stream::Stream;
use log;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex, RwLock};
use tokio_stream::StreamExt as _;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Opts {
    host: String,

    // Provider-specific options
    #[cfg(feature = "openai")]
    openai: providers::openai::OpenAIOpts,
}

#[derive(Clone, Debug)]
pub(crate) struct ServerState {
    opts: Opts,
    models: ModelsByCode,
}

fn get_provider_models(providers: Vec<&'static dyn Provider>) -> HashMap<String, ProviderModel> {
    let mut models = HashMap::new();

    for provider in providers {
        for model in provider.models() {
            models.insert(
                model.code.clone(),
                ProviderModel {
                    provider,
                    model: model.clone(),
                },
            );
        }
    }
    models
}

pub async fn serve(providers: Vec<&'static dyn Provider>, opts: Opts) -> Result<()> {
    let provider_len = providers.len();

    let models = get_provider_models(providers);

    log::info!(
        "Loaded {} models across {} providers",
        models.len(),
        provider_len
    );

    let state = ServerState {
        opts: opts.clone(),
        models,
    };

    let thread_safe_state = Arc::new(Mutex::new(state));

    let app = Router::new()
        .route("/invoke", post(invoke))
        .with_state(thread_safe_state);

    log::info!("listening on http://{}", opts.host.clone());

    let listener = tokio::net::TcpListener::bind(opts.host).await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}
