use axum::debug_handler;
use axum::response::sse::{Event, Sse};
use std::time::Duration;

use crate::error::Error::{ModelNotFoundError, NoModelAvailableError};
use crate::error::Result;
use crate::types::{Invocation, ModelsByCode, ProviderModel};
use crate::ServerState;
use axum::extract::{Json, State};
use futures::stream::Stream;
use tokio_stream::StreamExt;

fn resolve_model(models: &ModelsByCode, model_code: Option<String>) -> Result<&ProviderModel> {
    // At some point this will likely involve defaults and heuristics like cheapest model or
    // best model. Right now we just do model by code if it was provided, and the first model if it isn't

    match model_code {
        None => {
            // Grab the first model from the available models
            let (_, model) = models.iter().next().ok_or(NoModelAvailableError)?;
            Ok(model)
        }
        Some(model_code) => Ok(models
            .get(&model_code)
            .ok_or(ModelNotFoundError(model_code))?),
    }
}

#[debug_handler]
pub async fn invoke(
    State(state): State<ServerState>,
    Json(invocation): Json<Invocation>,
) -> Sse<impl Stream<Item = Result<Event>>> {
    let models = &state.models;
    // Get the invocation from the body
    let provider_model = resolve_model(models, invocation.model).unwrap();

    let stream = provider_model
        .provider
        .invoke(&provider_model.model, invocation.messages)
        .map(|message| Ok(Event::default().json_data(&message)?));

    Sse::new(stream).keep_alive(
        // Send a keep-alive every 15 seconds
        axum::response::sse::KeepAlive::new().interval(Duration::from_secs(15)),
    )
}
