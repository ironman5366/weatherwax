// This is the entrypoint for the server
use weatherwax::ai::types::Provider;

#[tokio::main]
async fn main() {
    let mut providers: Vec<&'static dyn Provider> = vec![];

    #[cfg(feature = "openai")]
    {
        use weatherwax::ai::providers::openai::OpenAIProvider;
        providers.push(&OpenAIProvider {});
    }
}
