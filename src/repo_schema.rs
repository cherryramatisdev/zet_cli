use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub id: u16,
    pub title: String,
    pub created_at: chrono::NaiveDateTime,
    pub modified_at: Option<chrono::NaiveDateTime>,
    pub dir_path: String,
    pub entry_file: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RepoSchema {
    pub author: String,
    pub repo_url: Option<String>,
    pub entries: Vec<Entry>,
}

impl RepoSchema {
    pub fn new() -> Self {
        let author = get_input("Author name of the repo: ", true);
        let repo_url = get_input("Repo URL (if you dont have one, press enter): ", false);

        Self {
            author,
            repo_url: if repo_url.is_empty() {
                None
            } else {
                Some(repo_url)
            },
            entries: vec![],
        }
    }

    pub fn save(&self) -> Result<()> {
        let schema = self.to_json_string()?;
        let index_path = RepoSchema::get_schema_index_path()?;
        std::fs::write(index_path, &schema)?;

        Ok(())
    }

    pub fn get_schema_path() -> Result<String> {
        let dir_path = match std::env::var("ZET_CURRENT") {
            Ok(path) => format!("{}", path),
            Err(_) => {
                let pwd = std::env::current_dir()?;

                pwd.to_string_lossy().into_owned()
            }
        };

        Ok(dir_path)
    }

    fn get_schema_index_path() -> Result<String> {
        let dir_path = RepoSchema::get_schema_path()?;

        Ok(format!("{}/index.json", dir_path))
    }

    pub fn get_config() -> Result<Self> {
        let schema_path = RepoSchema::get_schema_index_path()?;

        if let Ok(false) = std::fs::exists(&schema_path) {
            bail!("Please run the command on a valid repo location");
        }

        let content = std::fs::read_to_string(&schema_path)?;

        let schema: RepoSchema = serde_json::from_str(&content)?;

        Ok(schema)
    }

    pub fn to_json_string(&self) -> Result<String> {
        let json = serde_json::to_string(self)?;

        Ok(json)
    }
}

/// Gets input from the user with optional required flag
///
/// # Arguments
/// * `prompt` - The message to display to the user
/// * `required` - Whether the input is mandatory (cannot be empty)
///
/// # Returns
/// The user's input as a String
fn get_input(prompt: &str, required: bool) -> String {
    loop {
        print!("{}", prompt);
        std::io::stdout().flush().unwrap(); // Ensure prompt displays immediately

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim().to_string();

        // If not required, or if required and not empty, return the input
        if !required || (required && !input.is_empty()) {
            return input;
        }

        println!("Error: This field is required. Please enter a value.");
    }
}
