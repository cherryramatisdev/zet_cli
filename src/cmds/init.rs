use anyhow::{bail, Result};
use colored::Colorize;
use git2::{IndexAddOption, Repository};

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

    let repo = Repository::init(".")?;
    let schema = RepoSchema::new();

    schema.save()?;

    init_repo(repo)?;

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

    println!(
        "{}",
        format!(
            r#"
Done! repo created

Run this command to globally refer to your repo when running zet commands:

echo "export ZET_CURRENT={}" >> {}
        "#,
            pwd.to_str().unwrap(),
            shell
        )
        .green()
        .bold()
    );

    Ok(())
}

fn init_repo(repo: Repository) -> Result<(), anyhow::Error> {
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
        "Initial commit",
        &tree,
        &[],
    )?;
    Ok(())
}
