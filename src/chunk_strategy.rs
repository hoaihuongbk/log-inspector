pub struct ChunkStrategy {
    max_tokens_per_chunk: usize,
    overlap_lines: usize,
    context_window: usize,
    estimated_line_size: usize,
    max_lines_per_chunk: usize,
    min_chunk_size: usize,
}

impl ChunkStrategy {
    pub fn new() -> Self {
        ChunkStrategy {
            max_tokens_per_chunk: 2500, // Safe margin for GPT-3.5
            overlap_lines: 5,           // Overlap lines for context
            context_window: 1000,       // Characters to analyze for chunk boundaries
            estimated_line_size: 100, // Average line length in bytes
            max_lines_per_chunk: 200, // Maximum lines per chunk
            min_chunk_size: 10_000, // Minimum chunk size in bytes (10KB)
        }
    }

    pub fn calculate_optimal_chunk_size(&self, file_size: usize) -> usize {
        if file_size < self.min_chunk_size {
            return file_size;
        }

        // For larger files, calculate lines based on file size
        // let estimated_line_size = 100; // Average line length in bytes
        let total_lines = file_size / self.estimated_line_size;
        // Return number of lines, capped at 200
        let optimal_lines = (total_lines / 10).min(self.max_lines_per_chunk);

        // Convert lines back to bytes
        optimal_lines * self.estimated_line_size
    }

    pub fn find_chunk_boundary(&self, content: &str) -> usize {
        let window_end = content.len().min(self.context_window);
        let search_text = &content[..window_end];

        // Look for natural break points
        let patterns = [
            "\n[0-9]{4}-[0-9]{2}-[0-9]{2}", // DateTime
            "\nERROR:",
            "\nWARNING:",
            "\n[0-9]{10}", // Timestamp
        ];

        for pattern in patterns {
            if let Some(pos) = search_text.rfind(pattern) {
                return pos + self.overlap_lines;
            }
        }

        window_end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_file_strategy() {
        let strategy = ChunkStrategy::new();
        let size = strategy.calculate_optimal_chunk_size(5_000);
        assert_eq!(size, 5_000);
    }

    #[test]
    fn test_medium_file_strategy() {
        let strategy = ChunkStrategy::new();
        let size = strategy.calculate_optimal_chunk_size(50_000);
        assert!(size >= 50 * 100 && size <= 200 * 100);
    }

    #[test]
    fn test_large_file_strategy() {
        let strategy = ChunkStrategy::new();
        let size = strategy.calculate_optimal_chunk_size(1_000_000);
        assert_eq!(size, 200 * 100);
    }
}