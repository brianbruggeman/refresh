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
    git(&["clone", &repo.ssh_url, &repo.name], path, github_token).await?;
    Ok(())
}

pub async fn is_dirty(repo_path: impl AsRef<Path>) -> anyhow::Result<bool> {
    // git status --porcelain
    let output = git(&["status", "--porcelain"], repo_path, "").await?;
    Ok(!output.is_empty())
}

pub async fn is_main_branch(repo_path: impl AsRef<Path>) -> anyhow::Result<bool> {
    // git rev-parse --abbrev-ref HEAD
    let output = git(&["rev-parse", "--abbrev-ref", "HEAD"], repo_path, "").await?;
    Ok(output.trim() == "main")
}

// Executes a git command with Command
pub async fn git(
    args: &[impl AsRef<str>],
    path: impl AsRef<Path>,
    github_token: impl AsRef<str>,
) -> anyhow::Result<String> {
    // Setup command to execute git command
    let args = args.iter().map(|arg| arg.as_ref()).collect::<Vec<_>>();
    let output = Command::new("git")
        .args(&args)
        .current_dir(path)
        .env("GITHUB_TOKEN", github_token.as_ref())
        .output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
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
    git(&["fetch", "--all"], repo_path, github_token).await?;
    Ok(())
}

pub async fn pull_repo(repo_path: impl AsRef<Path>, github_token: &str) -> anyhow::Result<()> {
    // git pull
    git(&["pull"], repo_path, github_token).await?;
    Ok(())
}

pub async fn update_repos(repos: &[Repo], path: impl AsRef<Path>, github_token: &str) -> anyhow::Result<()> {
    let path = path.as_ref();
    if !path.exists() {
        fs::create_dir_all(path)?;
    }

    let futures = repos.iter().map(|repo| {
        let repo_path = path.join(&repo.name);
        let token_clone = github_token.to_owned();

        // Spawn a new asynchronous task for each repository
        task::spawn(async move {
            let token = token_clone.clone();

            // Fetch the repository
            fetch_repo(&repo_path, &token).await?;

            // Check if the repository is on `main` branch and not dirty
            if is_main_branch(&repo_path).await? && !is_dirty(&repo_path).await? {
                // Pull the repository
                pull_repo(&repo_path, &token).await?;
            }

            Ok(())
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

