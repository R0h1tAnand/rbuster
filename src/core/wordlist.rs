//! Wordlist loader with streaming support for memory efficiency

use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

/// Load entire wordlist into memory
pub async fn load_wordlist(path: &Path) -> std::io::Result<Vec<String>> {
    let file = File::open(path).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut entries = Vec::new();

    while let Some(line) = lines.next_line().await? {
        let trimmed = line.trim();
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            entries.push(trimmed.to_string());
        }
    }

    Ok(entries)
}
