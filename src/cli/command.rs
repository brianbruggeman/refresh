use std::env;
use std::path::Path;

use clap::Parser;

#[derive(Parser)]
pub struct Command {
    /// The name of the GitHub organization
    #[clap(short, long, env = "GITHUB_ORG", default_value = "")]
    pub org_name: String,

    /// The GitHub token to use for authentication
    #[clap(short, long, env = "GITHUB_TOKEN", default_value = "")]
    pub github_token: String,

    /// The path to the directory where the repos are stored
    #[clap(short, long, default_value = "")]
    pub path: String,

    /// When set to true, this will pull all repos in the organization
    /// When set to false, this will only fetch and update the repos that have been previously cloned
    #[clap(short, long)]
    pub all: bool,
}

impl Command {
    pub fn new() -> Self {
        let mut cmd = Command::parse();

        if cmd.github_token.is_empty() {
            // Prompt
            tracing::error!("Please set `GITHUB_TOKEN`, rerun with `--github-token` or enter below.");
            cmd.github_token = rpassword::prompt_password("GitHub token: ").unwrap();
        }
        let current_dir = env::current_dir()
                .expect("Failed to determine current directory")
                .to_str()
                .expect("Failed to convert path to string")
                .to_string();

        // When path is empty, use current directory
        if cmd.path.is_empty() {
            cmd.path = current_dir.clone();
            tracing::debug!("Using current directory: `{}`", cmd.path);
            let current_path =  Path::new(&current_dir);
            let filename = current_path.file_name().unwrap().to_str().unwrap();
            let git_path = current_path.join(".git");
            if current_path.exists() && filename == "refresh" && current_path.is_dir() && git_path.exists() && git_path.is_dir() {
                // if refresh contains .git, use "repos"
                tracing::debug!("Found `refresh` repo,  using: `repos`");
                cmd.path = current_path.join("repos").to_str().unwrap().to_string();
            }
            tracing::debug!("Set path to: `{}`", cmd.path);
        } else {
            tracing::debug!("Using provided directory: `{}`", cmd.path);
        }

        if cmd.org_name.is_empty() {
            tracing::debug!("Org is empty");
            let path = Path::new(&cmd.path);
            let filename = path.file_name().unwrap().to_str().unwrap();
            cmd.org_name = filename.to_string();
        }
        if cmd.org_name == "mine" {
            cmd.org_name = whoami::username();
        }
        tracing::debug!("Set org to: `{}`", cmd.org_name);

        cmd
    }
}
