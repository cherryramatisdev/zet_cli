use anyhow::{bail, Result};
use bat::PrettyPrinter;

use crate::repo_schema::{find_entry_by_term, RepoSchema};

pub fn call(term: String) -> Result<()> {
    let schema_path = RepoSchema::get_schema_path()?;
    let mut schema = RepoSchema::get_config()?;

    let entry = find_entry_by_term(&mut schema, term)?;

    match entry {
        Some(entry) => {
            let content = std::fs::read(format!(
                "{}/{}/{}",
                schema_path, entry.dir_path, entry.entry_file
            ))?;

            PrettyPrinter::new()
                .input_from_bytes(&content[..])
                .language("markdown")
                .print()?;

            Ok(())
        }
        None => bail!("Could not find the zettel"),
    }
}
