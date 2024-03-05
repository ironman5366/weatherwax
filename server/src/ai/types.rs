use crate::Opts;
use futures::Stream;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Invocation {}

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

pub trait Provider {
    fn new(opts: Opts) -> Self
    where
        Self: Sized;

    fn generate(&self, messages: Vec<Message>);
}
