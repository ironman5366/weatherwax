use crate::Opts;
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

pub struct Model {
    pub code: String,
    pub supports_function_calling: bool,
    pub supports_images: bool,
}

pub trait Provider {
    fn new(opts: Opts) -> Self
    where
        Self: Sized;

    fn models(&self) -> Vec<Model>;

    fn invoke(
        &self,
        model: &Model,
        messages: Vec<Message>,
    ) -> Pin<Box<dyn Stream<Item = Message> + Send>>;
}

pub(crate) struct ProviderModel<'a> {
    pub provider: &'a dyn Provider,
    pub model: &'a Model,
}

pub(crate) type ModelsByCode<'a> = HashMap<String, ProviderModel<'a>>;
