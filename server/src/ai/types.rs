use crate::Opts;
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Debug;
use std::pin::Pin;

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

pub trait Provider: Send + Sync {
    fn name(&self) -> &'static str;

    fn new(opts: Opts) -> Self
    where
        Self: Sized;

    fn models(&self) -> Vec<&Model>;

    fn invoke(
        &self,
        model: &Model,
        messages: Vec<Message>,
    ) -> Pin<Box<dyn Stream<Item = Message> + Send>>;
}

impl Debug for dyn Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Provider")
            .field("name", &self.name())
            .finish()
    }
}

#[derive(Clone)]
pub(crate) struct ProviderModel<'a> {
    pub provider: &'a (dyn Provider + Send + Sync),
    pub model: &'a Model,
}

impl Debug for ProviderModel<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ProviderModel")
            .field("provider", &self.provider.name())
            .field("model", &self.model.code)
            .finish()
    }
}

pub(crate) type ModelsByCode<'a> = HashMap<String, ProviderModel<'a>>;
