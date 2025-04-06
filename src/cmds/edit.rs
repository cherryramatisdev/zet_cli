use anyhow::{bail, Result};

use crate::repo_schema::RepoSchema;

pub fn call(term: String) -> Result<()> {
    let schema_path = RepoSchema::get_schema_path()?;
    let mut schema = RepoSchema::get_config()?;

    if term.chars().all(|ch| ch.is_numeric()) {
        let term_id = term.parse::<u16>()?;

        match schema.entries.iter_mut().find(|entry| entry.id == term_id) {
            Some(entry) => {
                std::process::Command::new("nvim")
                    .arg(format!("{}/{}/README.md", schema_path, term))
                    .status()?;

                entry.modified_at = Some(chrono::Local::now().naive_local());

                return Ok(());
            }
            None => bail!("Could not find the zettel"),
        }
    }

    match schema
        .entries
        .iter_mut()
        .find(|entry| entry.title.contains(term.as_str()))
    {
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
