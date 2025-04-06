use anyhow::{bail, Result};

use crate::repo_schema::{find_entry_by_term, RepoSchema};

pub fn call(term: String) -> Result<()> {
    let mut schema = RepoSchema::get_config()?;

    let entry = find_entry_by_term(&mut schema, term)?;

    match entry {
        Some(entry) => {
            std::process::Command::new("nvim")
                .arg(format!("{}/{}", entry.dir_path, entry.entry_file))
                .status()?;

            entry.modified_at = Some(chrono::Local::now().naive_local());

            schema.save()?;

            Ok(())
        }
        None => bail!("Could not find the zettel"),
    }
}
