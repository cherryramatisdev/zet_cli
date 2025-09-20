use std::io::{BufRead, IsTerminal, Read};

use anyhow::{bail, Result};

use crate::git_utils;
use crate::repo_schema::{Entry, RepoSchema};

pub fn call() -> Result<()> {
    // NOTE: If it's running on a pipeline
    let (header, content) = if !std::io::stdin().is_terminal() {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        (String::from("Piped content"), buffer)
    } else {
        get_note_interactively()?
    };

    let schema_path = RepoSchema::get_schema_path()?;
    let schema = create_note(header.clone(), content, None)?;

    schema.save()?;
    git_utils::commit(format!("Add new note: {}", header), Some(schema_path))?;

    Ok(())
}

fn get_note_interactively() -> Result<(String, String), anyhow::Error> {
    let file_path = get_rando_file_name(chrono::Local::now());
    std::process::Command::new(RepoSchema::get_editor())
        .arg(&file_path)
        .status()?;
    let file = std::fs::File::open(&file_path)?;
    let mut reader = std::io::BufReader::new(file);
    let mut header = String::new();
    reader.read_line(&mut header)?;

    // TODO: maybe a proper parser here, its fine for now though
    // Also, if this error happen, i kinda want to not lose the note and reuse it to allow editing
    // it
    if !header.contains("#") {
        bail!("Format error: Your note should have a top level title (#) on the first line.");
    }

    let header = header.replace("#", "");
    let header = header.trim().to_string();

    let content = std::fs::read_to_string(&file_path)?;

    Ok((header, content))
}

pub fn get_rando_file_name(now: chrono::DateTime<chrono::Local>) -> String {
    format!("/tmp/notes_{}.md", now.format("%d%m%Y%H%M%S"))
}

pub fn create_note(
    header: String,
    content: String,
    modified: Option<chrono::DateTime<chrono::Local>>,
) -> Result<RepoSchema> {
    let schema_path = RepoSchema::get_schema_path()?;
    let mut schema = RepoSchema::get_config()?;

    let new_id = if modified.is_some() {
        // NOTE: If the modified is passed, we assume the id exists, is there a way to encode this
        // on the type system?
        schema
            .entries
            .iter()
            .find(|en| en.title == header)
            .unwrap()
            .id
    } else {
        let id = match schema.entries.last() {
            Some(entry) => entry.id + 1,
            None => 1,
        };

        std::fs::create_dir(format!("{}/{}", &schema_path, &id))?;
        std::fs::write(format!("{}/{}/README.md", &schema_path, &id), &content)?;

        id
    };

    if modified.is_some() {
        if let Some(entry) = schema.entries.iter_mut().find(|e| e.title == header) {
            entry.modified_at = modified.map(|m| m.naive_local())
        };
    } else {
        schema.entries.push(Entry {
            id: new_id,
            title: header,
            created_at: chrono::Local::now().naive_local(),
            modified_at: None,
            dir_path: new_id.to_string(),
            entry_file: String::from("README.md"),
        })
    }

    Ok(schema)
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn get_rando_file_name_test() {
        let mock_time = chrono::Local
            .with_ymd_and_hms(2023, 5, 15, 10, 30, 45)
            .unwrap();
        let result = get_rando_file_name(mock_time);
        assert_eq!(result, "/tmp/notes_5052023103045.md");
    }
}
