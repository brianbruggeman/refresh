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
            println!("Please set `GITHUB_TOKEN`, rerun with `--github-token` or enter below.");
            cmd.github_token = rpassword::prompt_password("GitHub token: ").unwrap();
        }
        let current_dir = env::current_dir()
                .expect("Failed to determine current directory")
                .to_str()
                .expect("Failed to convert path to string")
                .to_string();

        // When path is empty, use current directory
        if cmd.path.is_empty() {
            if current_dir == "refresh" {
                let git_path = Path::new(&current_dir).join(".git");
                // if refresh contains .git, use "repos"
                if git_path.exists() && git_path.is_dir() {
                    cmd.path = "repos".to_string();
                } else {
                    cmd.path = current_dir.clone();
                }
            }
            println!("Set path to: `{}`", cmd.path);
        }

        if cmd.org_name.is_empty() {
            cmd.org_name = Path::new(&cmd.path)
                .parent()
                .and_then(Path::file_name)
                .and_then(std::ffi::OsStr::to_str)
                .unwrap_or_default()
                .to_string();
        }

        cmd
    }
}
