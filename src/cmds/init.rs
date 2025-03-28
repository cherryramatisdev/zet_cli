use std::io::Write;

use colored::Colorize;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    id: u16,
    title: String,
    created_at: chrono::NaiveDateTime,
    modified_at: chrono::NaiveDateTime,
    dir_path: String,
    entry_file: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct RepoSchema {
    author: String,
    repo_url: Option<String>,
    entries: Vec<Entry>,
}

impl RepoSchema {
    fn new() -> Self {
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

    fn to_json_string(&self) -> Result<String> {
        let json = serde_json::to_string(self)?;

        Ok(json)
    }
}

pub fn call() -> Result<()> {
    let entries = std::fs::read_dir(".")?;

    if entries.count() > 0 {
        bail!("Your directory has content in it! Please run the binary in a clean directory");
    }

    if which::which("git").is_err() {
        bail!("Please install the `git` binary in your system at: https://github.com/git-guides/install-git");
    }

    if which::which("gh").is_err() {
        bail!("Please install the `gh` binary in your system at: https://cli.github.com");
    }

    let schema = RepoSchema::new().to_json_string()?;

    std::fs::write("index.json", &schema)?;

    let pwd = std::env::current_dir()?;
    let shell = std::env::var("SHELL")?;
    let shell = if shell.contains("bash") {
        "~/.bashrc"
    } else if shell.contains("fish") {
        "~/.config/fish/config.fish"
    } else if shell.contains("zsh") {
        "~/.zshrc"
    } else {
        "<your shell config file>"
    };

    println!("{}", format!(
        r#"
Done! repo created

Run this command to globally refer to your repo when running zet commands:

echo "export ZET_CURRENT={}" >> {}
        "#,
        pwd.to_str().unwrap(),
        shell
    ).green().bold());

    Ok(())
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
