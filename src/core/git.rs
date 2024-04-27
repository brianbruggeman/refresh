use futures::future::join_all;
use std::fs;
use std::path::Path;
use std::process::Command;
use tokio::task;

use super::Repo;

pub async fn clone_repos(
    repos: &[Repo],
    path: impl AsRef<Path>,
    github_token: &str,
) -> anyhow::Result<()> {
    let path = path.as_ref();
    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    let futures = repos.iter().map(|repo| {
        let repo_clone = repo.clone();
        let token_clone = github_token.to_owned();
        let path = path.to_path_buf();
        // Spawn a new asynchronous task for each repository
        task::spawn(async move {
            let token = token_clone.clone();
            clone_repo(repo_clone, path, &token).await
        })
    });

    // Collect all futures into a Vec
    let futures_collected: Vec<_> = futures.collect();

    // Wait for all spawned tasks to complete
    let results = join_all(futures_collected).await;

    // Check results
    results
        .into_iter()
        .flatten()
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok(())
}

pub async fn clone_repo(
    repo: Repo,
    path: impl AsRef<Path>,
    github_token: &str,
) -> anyhow::Result<()> {
    // git clone <url>
    git(&["clone", &repo.ssh_url, &repo.name], path, github_token).await
}

// Executes a git command with Command
pub async fn git(
    args: &[impl AsRef<str>],
    path: impl AsRef<Path>,
    github_token: impl AsRef<str>,
) -> anyhow::Result<()> {
    // Setup command to execute git command
    let args = args.iter().map(|arg| arg.as_ref()).collect::<Vec<_>>();
    let output = Command::new("git")
        .args(&args)
        .current_dir(path)
        // .env("GIT_TERMINAL_PROMPT", "0") // Disable terminal prompt for credentials
        // .env("GIT_ASKPASS", "echo") // Use echo to avoid providing a password
        // .env("GIT_SSH_COMMAND", format!("ssh -o UserKnownHostsFile=/dev/null -o StrictHostKeyChecking=no").as_str())
        .env("GITHUB_TOKEN", github_token.as_ref())
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!(
            "Failed to execute git command: `git {}` - {}",
            args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

pub async fn fetch_repo(repo_path: impl AsRef<Path>, github_token: &str) -> anyhow::Result<()> {
    // git fetch --all
    git(&["fetch", "--all"], repo_path, github_token).await
}

pub async fn pull_repo(repo_path: impl AsRef<Path>, github_token: &str) -> anyhow::Result<()> {
    // git pull
    git(&["pull"], repo_path, github_token).await
}

pub async fn update_repos(repos: &[Repo]) -> anyhow::Result<()> {
    for repo in repos {
        let path = Path::new(&repo.name);
        if !path.exists() {
            let url = &repo.ssh_url;
            let _output = Command::new("git")
                .arg("clone")
                .arg(url)
                .arg(&repo.name)
                .output()?;
        }
    }
    Ok(())
}
