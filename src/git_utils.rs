use anyhow::Result;
use git2::{IndexAddOption, Repository};

pub fn commit<'a>(message: String, path: Option<String>) -> Result<()> {
    let repo = Repository::open(path.unwrap_or(".".to_string()))?;
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
        &message,
        &tree,
        &[&parent_commit],
    )?;
    Ok(())
}
