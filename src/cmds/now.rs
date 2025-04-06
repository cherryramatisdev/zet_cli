use anyhow::Result;
use std::fs::OpenOptions;
use std::io::Write;

use crate::{git_utils, repo_schema::RepoSchema};

use super::create::{create_note, get_rando_file_name};

pub fn call() -> Result<()> {
    let now = chrono::Local::now();
    let header = now.format("%d %B %Y");

    let schema_path = RepoSchema::get_schema_path()?;
    let config = RepoSchema::get_config()?;
    let entry = config
        .entries
        .iter()
        .find(|entry| entry.title == header.to_string());

    let schema = if let Some(entry) = entry {
        let path = format!("{}/{}", entry.dir_path, entry.entry_file);

        std::process::Command::new("nvim").arg(&path).status()?;

        let content = std::fs::read_to_string(&path)?;

        create_note(header.to_string(), content, Some(chrono::Local::now()))
    } else {
        let path = get_rando_file_name(now);

        write_line_to_file(&path, format!("# {}", header).as_str())?;

        std::process::Command::new("nvim").arg(&path).status()?;

        let content = std::fs::read_to_string(&path)?;

        create_note(header.to_string(), content, None)
    }?;

    schema.save()?;

    git_utils::commit(format!("Add new note: {}", header), Some(schema_path))?;

    Ok(())
}

fn write_line_to_file(file_path: &str, line: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)?;

    writeln!(file, "{}", line)?;
    Ok(())
}
