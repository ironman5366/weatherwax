use config::{Config, Environment};
use dotenvy::dotenv;
use std::sync::Arc;
use weatherwax::ai::types::Provider;
use weatherwax::types::ProviderPtr;
use weatherwax::{serve, Opts};

#[tokio::main]
async fn main() {
    // We don't care if this succeeds or not - it's perfectly fine to not have a .env file. Load before env_logger
    // in case the user wants to specifyRUST_LOG in the .env file
    let _ = dotenv();
    env_logger::init();

    log::info!("Starting Weatherwax");

    // Ignore cargo, it doesn't understand that this need to be mut for the cfg
    #[allow(unused_mut)]
        let mut providers: Vec<ProviderPtr> = vec![];

    let config = Config::builder()
        .set_default("host", "0.0.0.0:8000")
        .expect("Invalid host")
        // Because of the __ separator you can specify for ex. openai api key as OPENAI__API_KEY
        .add_source(Environment::default().separator("__"))
        .build()
        .expect("Failed to build config");

    let opts: Opts = config
        .try_deserialize()
        .expect("Failed to deserialize config");

    #[cfg(feature = "openai")]
    {
        log::info!("Loading OpenAI provider...");
        use weatherwax::ai::providers::openai::OpenAIProvider;
        let openai_provider = OpenAIProvider::new(opts.clone()).await.expect("Couldn't load OpenAI provider");
        providers.push(Arc::new(openai_provider));
    }

    serve(providers, opts).await.unwrap();
}
