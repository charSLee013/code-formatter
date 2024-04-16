use tokio::{fs::File, io::AsyncReadExt};

/// Reads the content of a file asynchronously and returns it as a String.
pub async fn read_file_content(path: &std::path::Path) -> std::io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

/// Formats the content for output based on the language short code.
pub fn format_content(content: &str, language: &str) -> String {
    format!("```{}\n{}\n```", language, content)
}

#[cfg(test)]
mod tests;
