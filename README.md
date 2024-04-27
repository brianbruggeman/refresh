# refresh
[![CI](https://github.com/brianbruggeman/refresh/actions/workflows/ci.yml/badge.svg)](https://github.com/brianbruggeman/refresh/actions/workflows/ci.yml)[![CD](https://github.com/brianbruggeman/refresh/actions/workflows/cd.yml/badge.svg?branch=main)](https://github.com/brianbruggeman/refresh/actions/workflows/cd.yml)

Simple tool to keep remote repositories locally fresh from a GitHub organization.


## Quickstart

```bash
cargo install --git https://github.com/brianbruggeman/refresh
```

## Usage

```bash
refresh --help
```

## Example crons

Clone new repos once a week on Sunday at 12a.  This will clone new repos into the root path and
pull the latest changes from the remote repositories.
```bash
0 0 * * SUN /usr/local/bin/refresh --all --github-org=... --github-token=... --path=...
```

Fetch and pull the latest changes from the remote repositories for the repos found within
the root path where the repositories are collected.  This will run Monday through Friday at 12a.
```bash
0 0 * * MON-FRI /usr/local/bin/refresh --github-org=... --github-token=... --path=...
```

## Future roadmap items

- [ ] remove the need for a github token within the crons
- [ ] make this more configurable for better automation with a refresh.toml
- [ ] add a way for exclusions as a glob
- [ ] support deeper hierarchy roots than just the flat root
- [ ] add support for branch/version control on clones/refreshes
- [ ] add support for a dry-run mode
- [ ] add support for controlling which repos are refreshed
- [ ] add support for refreshing within a repo
- [ ] add support for optionally updating main even when the repo is dirty
- [ ] add an install script for latest release for arch and platform
- [ ] add a way for refresh to auto-update itself
