use anyhow::{bail, Result};
use colored::Colorize;
use regex::Regex;
use std::io::BufRead;

use crate::{error_management::err_print, repo_schema::{Entry, RepoSchema}};

pub fn call(path: String) -> Result<()> {
    let dirs: Vec<String> = std::fs::read_dir(&path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let dir_format = Regex::new(r"\d+$").unwrap();

            // Check if it's a directory with a numeric ending
            if !entry.file_type().ok()?.is_dir()
                || !dir_format.is_match(entry.file_name().to_str()?)
            {
                return None;
            }

            // Check if "README.md" exists inside the directory
            let readme_path = entry.path().join("README.md");
            if !readme_path.exists() {
                return None;
            }

            Some(entry.file_name().to_string_lossy().into_owned())
        })
        .collect();

    if dirs.len() == 0 {
        bail!("You dont have any directories in the correct format being `<numeric_id>/README.md`");
    }

    let mut schema = RepoSchema::get_config()?;
    let schema_path = RepoSchema::get_schema_path()?;

    for dir in &dirs {
        let src_path = format!("{}/{}", &path, &dir);
        let id = schema.entries.last().map_or(1, |entry| entry.id + 1);

        let file = std::fs::File::open(format!("{}/README.md", &src_path))?;
        let mut reader = std::io::BufReader::new(file);
        let mut header = String::new();
        reader.read_line(&mut header)?;

        if !header.contains("#") {
            err_print(format!("Format error on note [{}]: Your note should have a top level title (#) on the first line.", &src_path));
            continue
        }

        let header = header.replace("#", "");
        let header = header.trim().to_string();

        std::process::Command::new("cp")
            .arg("-r")
            .arg(&src_path)
            .arg(format!("./{}", id))
            .output()?;

        schema.entries.push(Entry {
            id,
            title: header,
            created_at: chrono::Local::now().naive_local(),
            modified_at: None,
            dir_path: format!("{}/{}", schema_path, id.to_string()),
            entry_file: String::from("README.md"),
        });

        schema.save()?;
    }

    println!("{}", "Imported succesfully!".to_string().green().bold());

    Ok(())
}
