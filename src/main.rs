mod chunk_strategy;
mod config;
mod error_types;
mod log_inspector;
mod log_reader;
mod openai_client;

use config::Config;
use log_inspector::LogInspector;
use log_reader::LogReader;
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
    let mut reader = LogReader::new(log_path)?;
    // Initialize the OpenAI client and log inspector
    let inspector = LogInspector::new(config.openai_api_key, config.openai_host);

    // Process each chunk
    let chunks = reader.read_chunks()?;
    for (i, chunk) in chunks.iter().enumerate() {
        println!("\n=== Processing chunk {} ===", i + 1);

        let error_codes = inspector.error_classify(chunk).await?;
        println!("Error Codes: {}", error_codes);

        let summary = inspector.summarize(chunk).await?;
        println!("Summary: {}", summary);
    }

    Ok(())
}
