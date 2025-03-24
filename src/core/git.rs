use std::fs;
use std::path::Path;
use std::process::Command;

use futures::future::join_all;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::task;

use super::{true_path, Repo};

pub async fn clone_repos(repos: &[Repo], path: impl AsRef<Path>, github_token: &str) -> anyhow::Result<()> {
    if repos.is_empty() {
        tracing::debug!("No repos to clone");
        return Ok(());
    }
    let path = path.as_ref();
    if !path.exists() {
        tracing::debug!("Creating directory: {:?}", path);
        fs::create_dir_all(path)?;
    }
    let path = true_path(path);

    let total_repos = repos.len();
    tracing::info!("Cloning {total_repos} repos");
    let pb = ProgressBar::new(total_repos as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg}\n[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
            .progress_chars("=>-"),
    );

    let futures = repos.iter().map(|repo| {
        let repo_clone = repo.clone();
        let token_clone = github_token.to_owned();
        let path = path.clone();
        let pb = pb.clone();
        // Spawn a new asynchronous task for each repository
        task::spawn(async move {
            let token = token_clone.clone();
            clone_repo(repo_clone, path, &token, pb).await
        })
    });

    // Collect all futures into a Vec
    let futures_collected: Vec<_> = futures.collect();

    // Wait for all spawned tasks to complete
    let results = join_all(futures_collected).await;

    // Check results
    results.into_iter().flatten().collect::<anyhow::Result<Vec<_>>>()?;

    pb.finish_with_message("Cloning completed");
    Ok(())
}

pub async fn clone_repo(repo: Repo, path: impl AsRef<Path>, github_token: &str, pb: ProgressBar) -> anyhow::Result<()> {
    // git clone <url>
    git(&["clone", &repo.ssh_url, &repo.name], path.as_ref(), github_token).await?;
    pb.set_message(format!("Cloned: `{}` into `{}`", repo.name, path.as_ref().display()));
    pb.inc(1);
    Ok(())
}

pub async fn fetch_repo(repo_path: impl AsRef<Path>, github_token: &str, pb: &ProgressBar) -> anyhow::Result<()> {
    // git fetch --all
    git(&["fetch", "--all"], repo_path.as_ref(), github_token).await?;
    pb.set_message(format!("Fetched: `{}`", repo_path.as_ref().display()));
    Ok(())
}

// Executes a git command with Command
pub async fn git(args: &[impl AsRef<str>], path: impl AsRef<Path>, github_token: impl AsRef<str>) -> anyhow::Result<String> {
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
        Err(anyhow::anyhow!("Failed to execute git command: `git {}` - {}", args.join(" "), String::from_utf8_lossy(&output.stderr)))
    }
}

pub async fn has_upstream(repo_path: impl AsRef<Path>) -> anyhow::Result<bool> {
    // git rev-parse --abbrev-ref --symbolic-full-name @{u}
    match git(&["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"], repo_path, "").await {
        Ok(output) => Ok(!output.is_empty()),
        Err(_) => Ok(false),
    }
}

pub async fn is_dirty(repo_path: impl AsRef<Path>) -> anyhow::Result<bool> {
    // git status --porcelain
    match git(&["status", "--porcelain"], repo_path, "").await {
        Ok(output) => Ok(!output.is_empty()),
        Err(_) => Ok(false),
    }
}

pub async fn is_main_branch(repo_path: impl AsRef<Path>) -> anyhow::Result<bool> {
    // git rev-parse --abbrev-ref HEAD
    match git(&["rev-parse", "--abbrev-ref", "HEAD"], repo_path, "").await {
        Ok(output) => Ok(output.trim() == "main"),
        Err(_) => Ok(false),
    }
}

pub async fn pull_repo(repo_path: impl AsRef<Path>, github_token: &str, pb: &ProgressBar) -> anyhow::Result<()> {
    // git pull
    git(&["pull"], repo_path.as_ref(), github_token).await?;
    pb.set_message(format!("Pulled: `{}`", repo_path.as_ref().display()));
    Ok(())
}

pub async fn update_repos(repos: &[Repo], path: impl AsRef<Path>, github_token: &str) -> anyhow::Result<()> {
    if repos.is_empty() {
        tracing::debug!("No repos to update");
        return Ok(());
    }
    let path = true_path(path);
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    let total_repos = repos.len();
    let pb = ProgressBar::new(total_repos as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg}\nUpdating: [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")?
            .progress_chars("=>-"),
    );

    let futures = repos.iter().map(|repo| {
        let repo_path = path.join(&repo.name);
        let token_clone = github_token.to_owned();
        let pb = pb.clone();
        let repo_name = repo.name.clone();

        // Spawn a new asynchronous task for each repository
        task::spawn(async move {
            let token = token_clone.clone();

            // Fetch the repository
            if has_upstream(&repo_path).await? {
                if let Err(why) = fetch_repo(&repo_path, &token, &pb).await {
                    tracing::error!("Failed to fetch repo: `{}` - {}", repo_name, why);
                    return Err(why);
                }
            }

            // Check if the repository is on `main` branch and not dirty
            if is_main_branch(&repo_path).await? && !is_dirty(&repo_path).await? && has_upstream(&repo_path).await? {
                // Pull the repository
                if let Err(why) = pull_repo(&repo_path, &token, &pb).await {
                    tracing::error!("Failed to pull repo: `{}` - {}", repo_name, why);
                    return Err(why);
                }
            }
            pb.inc(1);

            Ok(())
        })
    });

    // Collect all futures into a Vec
    let futures_collected: Vec<_> = futures.collect();

    // Wait for all spawned tasks to complete
    let results = join_all(futures_collected).await;

    // Check results
    results.into_iter().flatten().collect::<anyhow::Result<Vec<_>>>()?;

    pb.finish_with_message("Updating completed");
    Ok(())
}
