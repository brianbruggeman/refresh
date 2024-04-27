use std::fs;
use std::path::Path;

use super::Repo;

pub async fn find_local_repos(path: impl AsRef<Path>) -> anyhow::Result<Vec<Repo>> {
    if !path.as_ref().exists() {
        return Ok(Vec::new());
    }
    let paths = fs::read_dir(path.as_ref()).expect("Could not read folder");
    let mut repos = Vec::new();
    for path in paths {
        let path = path.expect("Path was an error").path();
        if path.is_dir() {
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
    Ok(repos)
}
