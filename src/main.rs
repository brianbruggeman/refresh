mod cli;
use tracing_subscriber::fmt::format::FmtSpan;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    initialize_logging();
    match cli::run().await {
        Ok(_) => tracing::info!("Success!"),
        Err(e) => tracing::error!("Error: {}", e),
    }
}

fn initialize_logging() {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_span_events(FmtSpan::CLOSE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
}
