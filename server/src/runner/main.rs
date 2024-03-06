// This is the entrypoint for the server
use config::Config;
use weatherwax::ai::types::Provider;
use weatherwax::{serve, Opts};

#[tokio::main]
async fn main() {
    // Ignore cargo, it doesn't understand that this need to be mut for the cfg
    #[allow(unused_mut)]
    let mut providers: Vec<&(dyn Provider + Send + Sync)> = vec![];

    let config = Config::builder()
        .set_default("host", "0.0.0.0:8000")
        .expect("Invalid host")
        .add_source(config::Environment::default())
        .build()
        .expect("Failed to build config");

    let opts: Opts = config
        .try_deserialize()
        .expect("Failed to deserialize config");

    #[cfg(feature = "openai")]
    {
        use weatherwax::ai::providers::openai::OpenAIProvider;
        providers.push(&OpenAIProvider::new(opts));
    }

    serve(providers, opts).await.unwrap();
}
