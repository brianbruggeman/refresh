# refresh
[![CI](https://github.com/brianbruggeman/refresh/actions/workflows/ci.yml/badge.svg)](https://github.com/brianbruggeman/refresh/actions/workflows/ci.yml)[![CD](https://github.com/brianbruggeman/refresh/actions/workflows/cd.yml/badge.svg?branch=main)](https://github.com/brianbruggeman/refresh/actions/workflows/cd.yml)

This project is a simple way to pull all of the remote repositories from a GitHub organization and clone them to a local directory. This is useful for organizations that have many repositories and want to clone them all at once.  Additionally, it will keep all of the repositories up to date by pulling the latest changes from the remote repositories using fetch.  If any of the repos are on the main branch and it is possible to fast forward, then the main branch will be updated.  If the main branch is not up to date, then the script will identify which branches need updates and will take no action in that repository.

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