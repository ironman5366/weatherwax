use crate::error::{Result, Error};
use crate::types::{Message, Model, Provider, Role};
use crate::Opts;
use async_openai::config::OpenAIConfig;
use async_openai::types::{ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestFunctionMessageArgs, ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestToolMessageArgs, ChatCompletionRequestUserMessageArgs, Role as OAIRole};
use async_openai::types::{ChatCompletionRequestMessage, CreateChatCompletionStreamResponse, CreateChatCompletionRequestArgs};


use futures::Stream;
use serde::Deserialize;
use std::pin::{Pin};
use async_openai::Client;
use async_trait::async_trait;
use tokio_stream::StreamExt as _;
use crate::error::Error::OpenAIConversionError;

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
            Role::System => Ok(ChatCompletionRequestSystemMessageArgs::default().content(self.content).build()?.into()),
            Role::User => Ok(ChatCompletionRequestUserMessageArgs::default().content(self.content).build()?.into()),
            Role::Assistant => Ok(ChatCompletionRequestAssistantMessageArgs::default().content(self.content).build()?.into()),
            Role::Function => Ok(ChatCompletionRequestToolMessageArgs::default().content(self.content).build()?.into()),
        }
    }
}

impl Into<OAIRole> for Role {
    fn into(self) -> OAIRole {
        match self {
            Role::System => OAIRole::System,
            Role::User => OAIRole::User,
            Role::Assistant => OAIRole::Assistant,
            Role::Function => OAIRole::Tool,
        }
    }
}

impl From<OAIRole> for Role {
    fn from(role: OAIRole) -> Self {
        match role {
            OAIRole::System => Role::System,
            OAIRole::User => Role::User,
            OAIRole::Assistant => Role::Assistant,
            OAIRole::Tool => Role::Function,
            OAIRole::Function => Role::Function,
        }
    }
}

impl TryFrom<CreateChatCompletionStreamResponse> for Message {
    type Error = Error;

    fn try_from(resp: CreateChatCompletionStreamResponse) -> Result<Message> {
        let oai_message =
            resp.choices
                .first()
                .ok_or(OpenAIConversionError(
                    "No choices in OpenAI response".to_string(),
                ))?;

        let delta = &oai_message.delta;

        Ok(Message {
            role: delta.role.ok_or(OpenAIConversionError("No role in OAI message".to_string()))?.into(),
            content: delta
                .content
                .clone()
                .ok_or(OpenAIConversionError(
                    "No content in OpenAI response".to_string(),
                ))?,
        })
    }
}

#[async_trait]
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
    ) -> Result<Pin<Box<dyn Stream<Item=Result<Message>> + Send>>> {
        let oai_messages: Vec<ChatCompletionRequestMessage> = messages
            .into_iter()
            .map(|message| message.try_into())
            .collect::<Result<Vec<ChatCompletionRequestMessage>>>()?;

        let request = CreateChatCompletionRequestArgs::default()
            .model(&model.code)
            .messages(oai_messages)
            .build()?;

        let stream = self.client.chat().create_stream(request).await?;
        let message_stream = stream.map(|oai_message| {
            let message: Message = oai_message?.try_into()?;
            Ok(message)
        });
        Ok(Box::pin(message_stream))
    }
}
