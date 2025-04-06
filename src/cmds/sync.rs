use anyhow::{bail, Result};
use git2::Repository;

use crate::repo_schema::RepoSchema;

pub fn call() -> Result<()> {
    let schema = RepoSchema::get_config()?;
    let schema_path = RepoSchema::get_schema_path()?;

    if schema.repo_url.is_none() {
        bail!(format!(
            "First you need to configure a remote url to the git repo at: {}",
            schema_path
        ));
    }

    let repo = Repository::open(&schema_path)?;

    let head = repo.head()?;
    let branch_name = head
        .shorthand()
        .ok_or_else(|| anyhow::anyhow!("Failed to get branch name from HEAD"))?;

    // TODO: use libgit2 for the pushing as well;
    std::process::Command::new("git")
        .arg("-C")
        .arg(schema_path)
        .arg("push")
        .arg("origin")
        .arg(branch_name)
        .status()?;

    Ok(())
}
