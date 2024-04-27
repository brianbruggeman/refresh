use std::fs;
use std::path::Path;

use crate::core::true_path;

use super::Repo;

pub async fn find_local_repos(path: impl AsRef<Path>) -> anyhow::Result<Vec<Repo>> {
    if !path.as_ref().exists() {
        return Ok(Vec::new());
    }
    let path = true_path(path);
    tracing::debug!("Searching under: {:?}", path);
    let paths = fs::read_dir(path).expect("Could not read folder");
    let mut repos = Vec::new();
    for path in paths {
        let path = path.expect("Path was an error").path();
        let git_path = path.join(".git");
        if path.is_dir() && git_path.exists() && git_path.is_dir() {
            let repo = Repo {
                name: path
                    .file_name()
                    .expect("Could not get filename")
                    .to_string_lossy()
                    .to_string(),
                ssh_url: "".to_string(),
            };
            repos.push(repo);
        }
    }
    tracing::debug!("Found: {} repos", &repos.len());
    Ok(repos)
}
