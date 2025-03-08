mod chunk_strategy;
mod chunk_strategy_tests;
mod error_types;
mod log_inspector;
mod log_reader;
mod openai_client;

use chunk_strategy::ChunkStrategy;
use dotenv::dotenv;
use log_inspector::LogInspector;
use log_reader::LogReader;
use openai_client::OpenAIClient;
use std::env;
use std::io;
use std::path::PathBuf;

use std::fs::metadata;
use std::path::Path;

fn get_file_size<P: AsRef<Path>>(path: P) -> io::Result<u64> {
    let metadata = metadata(path)?;
    Ok(metadata.len())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let host = env::var("OPENAI_HOST").unwrap_or_else(|_| "https://api.openai.com".to_string());

    // println!("API Key: {}", api_key);
    // println!("Host: {}", host);
    println!("Starting log inspection...");

    // let log_path = "example.log";
    let args: Vec<String> = env::args().collect();
    let log_path = args.get(1).expect("Please provide a log file path");

    let file_size = get_file_size(log_path)?;
    println!("Log file size: {} bytes", file_size);

    // Initialize the log reader and chunk strategy
    let strategy = ChunkStrategy::new();
    let chunk_size = strategy.calculate_optimal_chunk_size(file_size as usize);
    let mut reader = LogReader::new(log_path)?;
    let chunks = reader.read_chunks(chunk_size)?;

    // Initialize the OpenAI client and log inspector
    let inspector = LogInspector::new(api_key, host);

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
