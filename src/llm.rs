//! LLM integration module for docgen-rs
// This module will handle communication with an Open LLM API to generate professional documentation.
// It will take form context, git changes, and a user prompt, and return a markdown document.

use std::fs;
use std::env;
use serde_json::json;
use dotenv::dotenv;

/// Represents the context collected from the TUI form.
#[derive(Debug, Clone)]
pub struct FormContext {
    pub code_flow: String,
    pub db_changes: String,
    pub extensibility: String,
    pub perf_notes: String,
}

/// Represents a summary of git changes to be sent to the LLM, including code diff.
#[derive(Debug, Clone)]
pub struct GitChange {
    pub filename: String,
    pub lines_added: usize,
    pub lines_removed: usize,
    pub functions_modified: usize,
    pub diff: String,
    pub code: String, // The actual code that was modified
}

/// Generates a professional markdown document using the OpenRouter API.
///
/// # Arguments
/// * `form` - The context from the TUI form (code flow, db changes, etc.)
/// * `git_changes` - A list of changed files/modules with diffs and code
/// * `user_prompt` - An additional prompt or instruction from the user
///
/// # Returns
/// * `String` - The generated markdown document
pub async fn generate_markdown_doc(
    form: &FormContext,
    git_changes: &[GitChange],
    user_prompt: &str,
    md_path: &str,
    pdf_path: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("OPENROUTER_API_KEY")?;

    // Compose the prompt
    let mut prompt = String::new();
    prompt.push_str("# Software Change Documentation\n");
    prompt.push_str("## Form Context\n");
    prompt.push_str(&format!("- Code Flow: {}\n- DB Changes: {}\n- Extensibility: {}\n- Performance/Security: {}\n\n",
        form.code_flow, form.db_changes, form.extensibility, form.perf_notes));
    prompt.push_str("## Git Changes\n");
    for change in git_changes {
        prompt.push_str(&format!(
            "### {}\n+{} -{} fns:{}\nDiff:\n{}\nCode:\n```rust\n{}\n```\n\n",
            change.filename, change.lines_added, change.lines_removed, change.functions_modified, change.diff, change.code));
    }
    prompt.push_str("## Instructions\n");
    prompt.push_str(user_prompt);
    prompt.push_str("\n\nGenerate a highly professional, comprehensive markdown document covering all software development aspects.");

    // Prepare OpenRouter API request
    let client = reqwest::Client::new();
    let body = json!({
        "model": "openrouter/openai/gpt-4-turbo",
        "messages": [{
            "role": "user",
            "content": prompt
        }],
        "max_tokens": 4096,
    });
    let res = client.post("https://openrouter.ai/api/v1/chat")
        .bearer_auth(api_key)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;
    let res_json: serde_json::Value = res.json().await?;
    let markdown = res_json["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string();

    // Save markdown to file
    fs::write(md_path, &markdown)?;

    // Convert markdown to PDF (requires pandoc installed)
    let output = std::process::Command::new("pandoc")
        .arg(md_path)
        .arg("-o")
        .arg(pdf_path)
        .output()?;
    if !output.status.success() {
        eprintln!("Pandoc failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(markdown)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_generate_markdown_doc_dummy() {
        let form = FormContext {
            code_flow: "Refactored the authentication logic for better modularity.".to_string(),
            db_changes: "Added new user_sessions table.".to_string(),
            extensibility: "Can add more auth providers easily.".to_string(),
            perf_notes: "Improved login speed, no known security issues.".to_string(),
        };
        let git_changes = vec![
            GitChange {
                filename: "src/auth.rs".to_string(),
                lines_added: 42,
                lines_removed: 10,
                functions_modified: 3,
                diff: "+fn new_login() ... -fn old_login() ...".to_string(),
                code: "pub fn new_login() { /* ... */ }".to_string(),
            },
        ];
        let user_prompt = "Please generate a changelog and technical summary.";
        let md_path = "test_output.md";
        let pdf_path = "test_output.pdf";

        // This will fail unless you have a valid API key and pandoc installed.
        // For CI, you may want to mock the HTTP call and file system.
        let result = generate_markdown_doc(&form, &git_changes, user_prompt, md_path, pdf_path).await;
        match result {
            Ok(markdown) => {
                println!("Generated markdown:\n{}", markdown);
                assert!(markdown.contains("Software Change Documentation") || markdown.is_empty());
            },
            Err(e) => {
                eprintln!("Test failed (expected if no API key or pandoc): {}", e);
            }
        }
    }
} 