use anyhow::Result;
use colored::Colorize;
use regex::Regex;

use crate::repo_schema::RepoSchema;

pub fn call() -> Result<()> {
    let schema_path = RepoSchema::get_schema_path()?;
    let schema = RepoSchema::get_config()?;

    for entry in schema.entries {
        let tags = extract_tags(format!(
            "{}/{}/{}",
            schema_path, entry.dir_path, entry.entry_file
        ))?;
        eprintln!(
            "{} - {} {}",
            entry.id,
            entry.title,
            tags.join(" ").bright_black().bold()
        );
    }

    Ok(())
}

fn extract_tags(file_path: String) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(file_path)?;
    let re = Regex::new(r"#\w+").unwrap(); // `\w` = letters, numbers, underscore

    // Search entire file
    let tags: Vec<String> = re
        .find_iter(&content)
        .map(|m| m.as_str().to_string())
        .collect();

    Ok(tags)
}
