use crate::types::{Message, Model, Provider};
use crate::Opts;
use async_openai::config::OpenAIConfig;
use async_openai::{types::CreateChatCompletionRequestArgs, Client};
use futures::Stream;
use serde::Deserialize;
use std::pin::Pin;

#[derive(Clone, Debug, Deserialize)]
pub struct OpenAIOpts {
    config: OpenAIConfig,
}

pub struct OpenAIProvider {
    client: Client<OpenAIConfig>,
    models: Vec<Model>,
}

impl Provider for OpenAIProvider {
    async fn new(opts: Opts) -> Self
    where
        Self: Sized,
    {
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
        Self { client, models }
    }

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
