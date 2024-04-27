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
}

impl Command {
    pub fn new() -> Self {
        let mut cmd = Command::parse();

        if cmd.github_token.is_empty() {
            // Prompt
            println!("Please set `GITHUB_TOKEN`, rerun with `--github-token` or enter below.");
            cmd.github_token = rpassword::prompt_password("GitHub token: ").unwrap();
        }

        // If path is None, set it to the current working directory
        if cmd.path.is_empty() {
            cmd.path = env::current_dir()
                .expect("Failed to determine current directory")
                .to_str()
                .expect("Failed to convert path to string")
                .to_string();
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
