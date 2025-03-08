use crate::chunk_strategy::ChunkStrategy;
use std::fs::File;
use std::io::Read;
use std::io::{self};

pub struct LogReader {
    file_path: String,
    strategy: ChunkStrategy,
    optimal_chunk_size: usize,
}

impl LogReader {
    pub fn new(file_path: &str) -> io::Result<Self> {
        // let file = File::open(path)?;
        // let reader = BufReader::new(file);
        // Ok(LogReader { reader })

        let metadata = std::fs::metadata(file_path)?;
        let file_size = metadata.len() as usize;
        let strategy = ChunkStrategy::new();
        let optimal_chunk_size = strategy.calculate_optimal_chunk_size(file_size);

        Ok(LogReader {
            file_path: file_path.to_string(),
            strategy,
            optimal_chunk_size,
        })
    }

    pub fn read_chunks(&mut self) -> io::Result<Vec<String>> {
        // let mut chunks = Vec::new();
        // let mut current_chunk = Vec::new();
        // let lines = self.reader.by_ref().lines();

        let mut file = File::open(&self.file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let mut chunks = Vec::new();
        let mut start = 0;

        while start < content.len() {
            let remaining = content.len() - start;
            if remaining <= self.optimal_chunk_size {
                chunks.push(content[start..].to_string());
                break;
            }

            let boundary = self.strategy.find_chunk_boundary(&content[start..]);
            let chunk = content[start..start + boundary].to_string();
            chunks.push(chunk);
            start += boundary;
        }

        Ok(chunks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_test_log(content: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file
    }

    #[test]
    fn test_large_file_multiple_chunks() {
        // Create a larger log file content (>10KB to trigger chunking)
        let mut content = String::new();
        for i in 0..200 {
            content.push_str(&format!("2024-01-10 10:15:{:02} INFO: Log entry {}\n\
                                     2024-01-10 10:15:{:02} ERROR: Error message {}\n\
                                     2024-01-10 10:15:{:02} WARNING: Warning message {}\n",
                                      i, i, i, i, i, i));
        }

        let file = create_test_log(&content);
        let mut reader = LogReader::new(file.path().to_str().unwrap()).unwrap();
        let chunks = reader.read_chunks().unwrap();

        assert!(chunks.len() > 1);
        assert!(chunks.iter().all(|chunk| !chunk.is_empty()));
    }
}
