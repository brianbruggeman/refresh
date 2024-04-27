# refresh

This project is a simple way to pull all of the remote repositories from a GitHub organization and clone them to a local directory. This is useful for organizations that have many repositories and want to clone them all at once.  Additionally, it will keep all of the repositories up to date by pulling the latest changes from the remote repositories using fetch.  If any of the repos are on the main branch and it is possible to fast forward, then the main branch will be updated.  If the main branch is not up to date, then the script will identify which branches need updates and will take no action in that repository.

## Quickstart

```bash
cargo install --git https://path/to/refresh.git
```

## Usage

```bash
refresh --help
```
