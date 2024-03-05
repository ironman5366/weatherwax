use crate::types::{Model, Provider};
use async_openai::{config::OpenAIConfig, Client};

pub struct OpenAIOpts(OpenAIConfig);

pub struct OpenAIProvider {
    opts: OpenAIOpts,
    client: Client<OpenAIConfig>,
}

impl Provider for OpenAIProvider {
    fn new(opts: crate::Opts) -> Self {
        Self {
            opts: opts.openai_opts,
            client: Client::with_config(opts.openai_opts.0),
        }
    }

    fn models(&self) -> Vec<Model> {
        todo!()
    }

    fn invoke(&self, model: Model<Self::ModelDetails>) {
        todo!()
    }
}
