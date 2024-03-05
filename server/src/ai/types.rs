use crate::Opts;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Invocation {
    model: Option<String>,
    messages: Vec<Message>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

pub struct Model {
    code: String,
    supports_function_calling: bool,
    supports_images: bool,
}

pub trait Provider {
    fn new(opts: Opts) -> Self
    where
        Self: Sized;

    fn models(&self) -> Vec<Model>;

    fn invoke(&self, model: Model, messages: Vec<Message>) {}
}
