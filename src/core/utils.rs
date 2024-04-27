use super::Repo;

pub fn compare_repos(remote_repos: &[Repo], local_repos: &[Repo]) -> Vec<Repo> {
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
