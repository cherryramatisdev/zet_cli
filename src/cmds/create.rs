use std::io::BufRead;

use anyhow::bail;
use git2::{IndexAddOption, Repository};

use crate::repo_schema::{Entry, RepoSchema};

pub fn call() -> anyhow::Result<()> {
    let (header, content) = get_note_interactively()?;

    let schema_path = RepoSchema::get_schema_path()?;
    let mut schema = RepoSchema::get_config()?;

    let new_id = match schema.entries.last() {
        Some(entry) => entry.id + 1,
        None => 1,
    };

    std::fs::create_dir(format!("{}/{}", &schema_path, &new_id))?;
    std::fs::write(format!("{}/{}/README.md", &schema_path, &new_id), &content)?;

    schema.entries.push(Entry {
        id: new_id,
        title: header.clone(),
        created_at: chrono::Local::now().naive_local(),
        modified_at: None,
        dir_path: new_id.to_string(),
        entry_file: String::from("README.md"),
    });

    schema.save()?;

    let repo = Repository::open(schema_path)?;
    let head = repo.head()?;
    let parent_commit = repo.find_commit(head.target().unwrap())?;
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
    index.write()?;
    let tree_oid = index.write_tree()?;
    let tree = repo.find_tree(tree_oid)?;
    let signature = repo.signature()?;
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        format!("Add new note: {}", header).as_str(),
        &tree,
        &[&parent_commit],
    )?;

    Ok(())
}

fn get_note_interactively() -> Result<(String, String), anyhow::Error> {
    let file_path = get_rando_file_name(chrono::Local::now());
    std::process::Command::new("nvim")
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

fn get_rando_file_name(now: chrono::DateTime<chrono::Local>) -> String {
    format!("/tmp/notes_{}.md", now.format("%d%m%Y%H%M%S"))
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
        assert_eq!(result, "/tmp/notes_15052023103045.md");
    }
}
