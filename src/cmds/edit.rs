use anyhow::{bail, Result};

use crate::{
    git_utils,
    repo_schema::{find_entry_by_term, RepoSchema},
};

pub fn call(term: String) -> Result<()> {
    let schema_path = RepoSchema::get_schema_path()?;
    let mut schema = RepoSchema::get_config()?;

    let entry = find_entry_by_term(&mut schema, term)?;

    match entry {
        Some(entry) => {
            let path = format!("{}/{}/{}", schema_path, entry.dir_path, entry.entry_file);
            std::process::Command::new("nvim").arg(&path).status()?;

            entry.modified_at = Some(chrono::Local::now().naive_local());

            schema.save()?;

            git_utils::commit(format!("Changed note: {}", &path), Some(schema_path))?;

            Ok(())
        }
        None => bail!("Could not find the zettel"),
    }
}
