use crate::types::{Message, Model, Provider, Role};
use crate::Opts;
use async_openai::config::OpenAIConfig;
use async_openai::{types::CreateChatCompletionRequestArgs, Client};
use futures::{stream, Stream};
use std::pin::Pin;

#[derive(Clone, Debug)]
pub struct OpenAIOpts {
    config: OpenAIConfig,
}

#[derive(Clone)]
pub struct OpenAIProvider {
    client: Client<OpenAIConfig>,
}

impl Provider for OpenAIProvider {
    fn name(&self) -> &'static str {
        "openai"
    }

    async fn models(&self) -> Vec<Box<Model>> {
        // List the available models
        let all_models = self.client.models().list().await.unwrap();
        all_models
            .data
            .into_iter()
            .map(|model| {
                Box::new(Model {
                    code: format!("openai::{}", model.id),
                    // TODO: keep a whitelist for this
                    supports_images: true,
                    supports_function_calling: true,
                })
            })
            .collect()
    }

    fn invoke(
        &self,
        model: &Model,
        messages: Vec<Message>,
    ) -> Pin<Box<dyn Stream<Item = Message>>> {
        unimplemented!()
    }
}

pub fn create_openai_provider(opts: Opts) -> OpenAIProvider {
    let client = Client::with_config(opts.openai.config);
    OpenAIProvider { client }
}
