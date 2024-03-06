use crate::error::Result;
use crate::Opts;
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use async_trait::async_trait;

#[derive(Debug, Serialize, Deserialize)]
pub struct Invocation {
    pub model: Option<String>,
    pub messages: Vec<Message>,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,

    // OpenAI calls these "tools", most providers call them functions
    #[serde(alias = "tool")]
    Function,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Clone, Debug)]
pub struct Model {
    pub code: String,
    pub supports_function_calling: bool,
    pub supports_images: bool,
}

#[async_trait]
pub trait Provider: Send + Sync {
    async fn new(opts: Opts) -> Result<Self>
        where
            Self: Sized;
    fn name(&self) -> &'static str;

    fn models(&self) -> Vec<&Model>;

    async fn invoke(
        &self,
        model: &Model,
        messages: Vec<Message>,
    ) -> Result<Pin<Box<dyn Stream<Item=Result<Message>> + Send>>>;
}

impl Debug for dyn Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Provider")
            .field("name", &self.name())
            .finish()
    }
}

pub type ProviderPtr = Arc<dyn Provider + Send + Sync>;

pub(crate) struct ProviderModel {
    pub provider: ProviderPtr,
    pub model: Model,
}

impl Debug for ProviderModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProviderModel")
            //.field("provider", &self.provider.name())
            .field("model", &self.model.code)
            .finish()
    }
}

pub(crate) type ModelsByCode = HashMap<String, ProviderModel>;
