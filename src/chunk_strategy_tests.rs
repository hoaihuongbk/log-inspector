#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_file_strategy() {
        let strategy = ChunkStrategy::new();
        let size = strategy.calculate_optimal_chunk_size(5_000);
        assert_eq!(size, 5_000); // Should process as single chunk
    }

    #[test]
    fn test_medium_file_strategy() {
        let strategy = ChunkStrategy::new();
        let size = strategy.calculate_optimal_chunk_size(50_000);
        assert!(size >= 50 && size <= 200); // Should be in optimal range
    }

    #[test]
    fn test_large_file_strategy() {
        let strategy = ChunkStrategy::new();
        let size = strategy.calculate_optimal_chunk_size(1_000_000);
        assert_eq!(size, 200); // Should cap at maximum chunk size
    }
}
