use crate::types::{Message, Provider};

pub struct Opts {
    api_key: String,
}

pub struct OpenAIProvider {}

impl Provider for OpenAIProvider {
    fn new() -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn generate(&self, messages: Vec<Message>) {
        unimplemented!()
    }
}
