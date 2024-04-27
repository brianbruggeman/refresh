use super::Command;

use refresh::core;

pub async fn run() -> anyhow::Result<()> {
    let command = Command::new();
    let remote_repos = match command.all {
        true => core::fetch_repo_list(&command.org_name, &command.github_token).await?,
        false => Vec::new(),
    };
    let local_repos = core::find_local_repos(&command.path).await?;
    let repos_to_clone = core::compare_repos(&remote_repos, &local_repos);
    core::clone_repos(&repos_to_clone, &command.path, &command.github_token).await?;
    let combined_repos = core::combine_repos(&local_repos, &repos_to_clone);
    core::update_repos(&combined_repos, &command.path, &command.github_token).await?;
    Ok(())
}
