mod config;
mod log_inspector;

use config::Config;
use log_inspector::LogInspector;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let log_path = args.get(1).expect("Please provide a log file path");

    // Load configuration from all possible sources
    let config = Config::load()?;

    // Initialize log inspector
    let inspector = LogInspector::new(config.openai_api_key, config.openai_host);

    let question = "What is the summary of this log?";
    let summary = inspector.analyze(log_path, question).await?;
    println!("{}", summary);

    Ok(())
}
