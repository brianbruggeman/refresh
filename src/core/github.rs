use super::Repo;

pub async fn fetch_repo_list(org_name: &str, github_token: &str) -> anyhow::Result<Vec<Repo>> {
    let client = reqwest::Client::new();
    let user = whoami::username();
    let org_or_user = match org_name == user {
        true => "users",
        false => "orgs",
    };
    let mut url = format!("https://api.github.com/{org_or_user}/{org_name}/repos");
    let mut repos = Vec::new();

    loop {
        tracing::debug!("Fetching repos from: {}", url);
        let response = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", github_token))
            .header("Accept", "application/vnd.github+json")
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header("User-Agent", "refresh")
            .send()
            .await;
        match response {
            Ok(resp) => {
                if resp.status().is_success() {
                    let headers = resp.headers().clone();
                    match &resp.json::<Vec<Repo>>().await {
                        Ok(new_repos) => {
                            let link = headers.get("Link");
                            if let Some(link) = link {
                                let link = link.to_str().unwrap();
                                let next_link = link.split(',').find(|link| link.contains("rel=\"next\""));
                                if let Some(next_link) = next_link {
                                    let next_link = next_link.split(';').next().unwrap().trim();
                                    next_link
                                        .trim_start_matches('<')
                                        .trim_end_matches('>')
                                        .to_owned()
                                        .clone_into(&mut url);
                                } else {
                                    break;
                                }
                            } else {
                                break;
                            }
                            repos.extend(new_repos.iter().cloned());
                        }
                        Err(why) => {
                            return Err(anyhow::anyhow!("Failed to parse response for org repos: `{org_name}`. {}", why));
                        }
                    }
                } else {
                    return Err(anyhow::anyhow!(
                        "Failed to fetch repos for org: `{org_name}`. {}.\nPerhaps use a different `GITHUB_ORG` or `--org-name`?",
                        resp.status()
                    ));
                }
            }
            Err(why) => {
                return Err(anyhow::anyhow!("Failed to send request for org repos: `{org_name}`. {}.", why));
            }
        };
    }
    tracing::info!("Fetched: {} repos", &repos.len());
    Ok(repos)
}
