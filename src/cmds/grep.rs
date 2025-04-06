use anyhow::Result;

use crate::repo_schema::RepoSchema;

pub fn call(term: String) -> Result<()> {
    let schema_path = RepoSchema::get_schema_path()?;

    std::process::Command::new("git")
        .current_dir(&schema_path)
        .arg("grep")
        .arg(&term)
        .arg("--")
        .arg("*.md")
        .status()?;

    Ok(())
}
