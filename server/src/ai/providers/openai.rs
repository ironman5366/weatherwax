use crate::types::{Message, Model, Provider, Role};
use crate::Opts;
use async_openai::config::OpenAIConfig;
use async_openai::{types::CreateChatCompletionRequestArgs, Client};
use futures::{stream, Stream};
use serde::{Deserialize, Serialize};
use std::pin::Pin;

#[derive(Clone, Debug, Deserialize)]
pub struct OpenAIOpts {
    config: OpenAIConfig,
}

#[derive(Clone)]
pub struct OpenAIProvider {
    client: Client<OpenAIConfig>,
    models: Vec<Model>,
}

impl Provider for OpenAIProvider {
    fn name(&self) -> &'static str {
        "openai"
    }

    fn models(&self) -> Vec<&Model> {
        self.models.iter().collect()
    }

    fn invoke(
        &self,
        model: &Model,
        messages: Vec<Message>,
    ) -> Pin<Box<dyn Stream<Item = Message> + Send>> {
        unimplemented!()
    }
}

pub async fn create_openai_provider(opts: Opts) -> OpenAIProvider {
    let client = Client::with_config(opts.openai.config);
    // List the available models
    let api_models = client.models().list().await.unwrap();
    let models = api_models
        .data
        .into_iter()
        .map(|model| {
            Model {
                code: format!("openai::{}", model.id),
                // TODO: keep a whitelist for this
                supports_images: true,
                supports_function_calling: true,
            }
        })
        .collect();
    OpenAIProvider { client, models }
}
