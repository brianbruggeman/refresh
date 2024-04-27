use std::path::{Path, PathBuf};
use std::borrow::Cow;

use path_absolutize::Absolutize;

use super::Repo;

pub fn compare_repos(remote_repos: &[Repo], local_repos: &[Repo]) -> Vec<Repo> {
    if remote_repos.len() == 0 {
        return Vec::new();
    } else if local_repos.len() == 0 {
        tracing::info!("No local repos found");
        return remote_repos.to_vec();
    }
    let mut repos_to_clone = Vec::new();
    for remote_repo in remote_repos {
        if !local_repos
            .iter()
            .any(|local_repo| local_repo.name == remote_repo.name)
        {
            repos_to_clone.push(remote_repo.clone());
        }
    }
    repos_to_clone
}

pub fn combine_repos(local_repos: &[Repo], new_repos: &[Repo]) -> Vec<Repo> {
    let mut repos = Vec::new();
    for repo in local_repos {
        repos.push(repo.clone());
    }
    for repo in new_repos {
        repos.push(repo.clone());
    }
    repos
}

pub fn true_path(path: impl AsRef<Path>) -> PathBuf {
    fn inner(path: &Path) -> PathBuf {
        // Expand `~` to home directory
        let tilde_expanded = match path.to_str() {
            Some(str_path) => match shellexpand::tilde(str_path) {
                Cow::Borrowed(s) => Cow::Borrowed(Path::new(s)),
                Cow::Owned(s) => Cow::Owned(PathBuf::from(s)),
            },
            None => Cow::Borrowed(path),
        };

        // Get absolute path
        let abs_path = match tilde_expanded.absolutize() {
            Ok(path) => path,
            Err(_) => tilde_expanded,
        };

        // Canonicalize to resolve all symbolic links if it exists
        match abs_path.canonicalize() {
            Ok(path) => path,
            Err(_) => abs_path.to_path_buf(),
        }
    }
    inner(path.as_ref())
}