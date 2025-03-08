mod chunk_strategy;
mod error_types;
mod log_inspector;
mod log_reader;
mod openai_client;
mod config;

use chunk_strategy::ChunkStrategy;
use dotenv::dotenv;
use log_inspector::LogInspector;
use log_reader::LogReader;
use config::Config;
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

    let file_size = get_file_size(log_path)?;
    println!("Log file size: {} bytes", file_size);

    // Initialize the log reader and chunk strategy
    let strategy = ChunkStrategy::new();
    let chunk_size = strategy.calculate_optimal_chunk_size(file_size as usize);
    let mut reader = LogReader::new(log_path)?;
    let chunks = reader.read_chunks(chunk_size)?;

    // Initialize the OpenAI client and log inspector
    let inspector = LogInspector::new(config.openai_api_key, config.openai_host);

    // Process each chunk
    for (i, chunk) in chunks.iter().enumerate() {
        println!("\n=== Processing chunk {} ===", i + 1);

        let error_codes = inspector.error_classify(chunk).await?;
        println!("Error Codes: {}", error_codes);

        let summary = inspector.summarize(chunk).await?;
        println!("Summary: {}", summary);
    }

    Ok(())
}
