pub mod ai;

pub use ai::*;
use std::collections::HashMap;
use std::sync::Arc;

pub mod error;

use crate::invoke::invoke;
use crate::types::{ModelsByCode, Provider, ProviderModel, ProviderPtr};
use axum::routing::post;
use axum::Router;
use error::Result;
use log;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Opts {
    host: String,

    // Provider-specific options
    #[cfg(feature = "openai")]
    openai: providers::openai::OpenAIOpts,
}

#[derive(Debug)]
pub(crate) struct ServerState {
    opts: Opts,
    models: ModelsByCode,
}

fn get_provider_models(providers: Vec<ProviderPtr>) -> HashMap<String, ProviderModel> {
    let mut models = HashMap::new();

    for provider in providers {
        for model in provider.models() {
            models.insert(
                model.code.clone(),
                ProviderModel {
                    provider: provider.clone(),
                    model: model.clone(),
                },
            );
        }
    }

    models
}

pub async fn serve(providers: Vec<ProviderPtr>, opts: Opts) -> Result<()> {
    let provider_len = providers.len();

    let models = get_provider_models(providers);

    log::info!(
        "Loaded {} models across {} providers",
        models.len(),
        provider_len
    );

    let state = Arc::new(ServerState {
        opts: opts.clone(),
        models,
    });

    let app = Router::new()
        .route("/invoke", post(invoke))
        .with_state(state);

    log::info!("listening on http://{}", opts.host.clone());

    let listener = tokio::net::TcpListener::bind(opts.host).await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}
