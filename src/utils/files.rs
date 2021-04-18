use super::Result;
use std::fs;

#[derive(Default)]
pub struct ReadFileOptions {
    pub delimiter: String,
    pub padding: String,
}

pub fn read_file(filename: String, options: ReadFileOptions) -> Result<Vec<String>> {
    let content = fs::read_to_string(filename).or_else(|e| Err(e.to_string()))?;

    Ok(content
        .replace(&options.padding, "")
        .split(&options.delimiter)
        .map(|r| r.to_string())
        .collect())
}
