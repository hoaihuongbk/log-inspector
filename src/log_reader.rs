use std::fs::File;
use std::io::Read;
use std::io::{self, BufRead, BufReader};
use std::path::Path; // Add this trait

pub struct LogReader {
    reader: BufReader<File>,
}

impl LogReader {
    pub fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        Ok(LogReader { reader })
    }

    pub fn read_chunks(&mut self, chunk_size: usize) -> io::Result<Vec<String>> {
        let mut chunks = Vec::new();
        let mut current_chunk = Vec::new();
        let lines = self.reader.by_ref().lines();

        for line_result in lines {
            let line = line_result?;
            current_chunk.push(line);

            if current_chunk.len() >= chunk_size {
                chunks.push(current_chunk.join("\n"));
                current_chunk.clear();
            }
        }

        if !current_chunk.is_empty() {
            chunks.push(current_chunk.join("\n"));
        }

        Ok(chunks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_chunks_empty_file() -> io::Result<()> {
        let temp_file = NamedTempFile::new()?;
        let mut reader = LogReader::new(temp_file.path())?;
        let chunks = reader.read_chunks(2)?;
        assert!(chunks.is_empty());
        Ok(())
    }

    #[test]
    fn test_read_chunks_single_line() -> io::Result<()> {
        let temp_file = NamedTempFile::new()?;
        write(temp_file.path(), "line1")?;

        let mut reader = LogReader::new(temp_file.path())?;
        let chunks = reader.read_chunks(2)?;

        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], "line1");
        Ok(())
    }

    #[test]
    fn test_read_chunks_multiple_lines() -> io::Result<()> {
        let temp_file = NamedTempFile::new()?;
        write(temp_file.path(), "line1\nline2\nline3\nline4")?;

        let mut reader = LogReader::new(temp_file.path())?;
        let chunks = reader.read_chunks(2)?;

        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0], "line1\nline2");
        assert_eq!(chunks[1], "line3\nline4");
        Ok(())
    }

    #[test]
    fn test_read_chunks_partial_chunk() -> io::Result<()> {
        let temp_file = NamedTempFile::new()?;
        write(temp_file.path(), "line1\nline2\nline3")?;

        let mut reader = LogReader::new(temp_file.path())?;
        let chunks = reader.read_chunks(2)?;

        assert_eq!(chunks.len(), 2);
        assert_eq!(chunks[0], "line1\nline2");
        assert_eq!(chunks[1], "line3");
        Ok(())
    }
}
