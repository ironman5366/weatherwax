use crate::error::{Result, Error};
use crate::types::{Message, Model, Provider, Role};
use crate::Opts;
use async_openai::config::OpenAIConfig;
use async_openai::error::OpenAIError;
use async_openai::types::{ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestFunctionMessageArgs, ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestToolMessageArgs, ChatCompletionRequestUserMessageArgs, Role as OAIRole};
use async_openai::types::{ChatCompletionRequestMessage, CreateChatCompletionStreamResponse, CreateChatCompletionRequestArgs};


use futures::Stream;
use serde::Deserialize;
use std::pin::Pin;
use async_openai::Client;
use tokio_stream::StreamExt as _;

#[derive(Clone, Debug, Deserialize)]
pub struct OpenAIOpts {
    api_key: String,
}

pub struct OpenAIProvider {
    client: Client<OpenAIConfig>,
    models: Vec<Model>,
}

impl TryInto<ChatCompletionRequestMessage> for Message {
    type Error = Error;

    fn try_into(self) -> Result<ChatCompletionRequestMessage> {
        match self.role {
            Role::System => ChatCompletionRequestSystemMessageArgs::default().content(self.content).build()?.into(),
            Role::User => ChatCompletionRequestUserMessageArgs::default().content(self.content).build()?.into(),
            Role::Assistant => ChatCompletionRequestAssistantMessageArgs::default().content(self.content).build()?.into(),
            Role::Function => ChatCompletionRequestToolMessageArgs::default().content(self.content).build()?.into(),
        }
    }
}

impl TryInto<OAIRole> for Role {
    type Error = Error;

    fn try_into(self) -> Result<OAIRole> {
        match self {
            Role::System => Ok(OAIRole::System),
            Role::User => Ok(OAIRole::User),
            Role::Assistant => Ok(OAIRole::Assistant),
            Role::Function => Ok(OAIRole::Tool),
        }
    }
}

impl TryFrom<CreateChatCompletionStreamResponse> for Message {
    type Error = Error;

    fn try_from(resp: CreateChatCompletionStreamResponse) -> Result<Message> {
        let oai_message =
            resp.choices
                .first()
                .ok_or(Error::OpenAIConversionError(
                    "No choices in OpenAI response".to_string(),
                ))?;
        let delta = &oai_message.delta;

        Ok(Message {
            role: delta.role.into(),
            content: delta
                .content
                .ok_or(Error::OpenAIConversionError(
                    "No content in OpenAI response".to_string(),
                ))?,
        })
    }
}

impl Provider for OpenAIProvider {
    async fn new(opts: Opts) -> Result<Self>
        where
            Self: Sized,
    {
        let client = Client::with_config(OpenAIConfig::new().with_api_key(opts.openai.api_key));

        // List the available models
        let api_models = client.models().list().await?;
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

        for model in &models {
            log::debug!("Loaded openai model: {:?}", model);
        }

        Ok(Self { client, models })
    }

    fn name(&self) -> &'static str {
        "openai"
    }

    fn models(&self) -> Vec<&Model> {
        self.models.iter().collect()
    }

    async fn invoke(
        &self,
        model: &Model,
        messages: Vec<Message>,
    ) -> Result<Pin<Box<dyn Stream<Item=Message> + Send>>> {
        let request = CreateChatCompletionRequestArgs::default()
            .model(&model.code)
            .messages(messages.into_iter().map(|m| m.try_into()?).collect())
            .build()?;

        let stream = self.client.chat().create_stream(request).await?;
        Ok(stream
            .map(|item| {
                let message: Message = item.try_into()?;
                Ok(message)
            })
            .into())
    }
}
