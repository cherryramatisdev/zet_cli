use colored::Colorize;
use anyhow::{bail, Result};

use crate::repo_schema::RepoSchema;

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

    RepoSchema::new().save()?;

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
