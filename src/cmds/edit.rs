use anyhow::{bail, Result};

use crate::repo_schema::{find_entry_by_term, RepoSchema};

pub fn call(term: String) -> Result<()> {
    let schema_path = RepoSchema::get_schema_path()?;
    let mut schema = RepoSchema::get_config()?;

    let entry = find_entry_by_term(&mut schema, term)?;

    match entry {
        Some(entry) => {
            std::process::Command::new("nvim")
                .arg(format!(
                    "{}/{}/{}",
                    schema_path, entry.dir_path, entry.entry_file
                ))
                .status()?;

            entry.modified_at = Some(chrono::Local::now().naive_local());

            schema.save()?;

            Ok(())
        }
        None => bail!("Could not find the zettel"),
    }
}
