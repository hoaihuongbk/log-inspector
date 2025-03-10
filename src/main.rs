mod chunk_strategy;
mod config;
mod error_types;
mod log_inspector;
mod log_reader;
mod openai_client;

use config::Config;
use log_inspector::LogInspector;
use std::env;
use std::io;

use std::fs::metadata;
use std::path::Path;

fn get_file_size<P: AsRef<Path>>(path: P) -> io::Result<u64> {
    let metadata = metadata(path)?;
    Ok(metadata.len())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let log_path = args.get(1).expect("Please provide a log file path");

    // Load configuration from all possible sources
    let config = Config::load()?;

    println!("Starting log inspection for: {}", log_path);

    // Initialize reader and inspector
    // let mut reader = LogReader::new(log_path)?;
    // Initialize the OpenAI client and log inspector
    let inspector = LogInspector::new(config.openai_api_key, config.openai_host);

    let question = "What is the summary of this log?";
    let summary = inspector.analyze(log_path, question).await?;
    println!("{}", summary);

    Ok(())
}
