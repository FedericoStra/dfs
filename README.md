# dfs

[![GitHub](https://img.shields.io/badge/github-FedericoStra/dfs-master?logo=github)](https://github.com/FedericoStra/dfs)
[![Crates.io](https://img.shields.io/crates/v/dfs?logo=rust)](https://crates.io/crates/dfs)
[![docs.rs](https://img.shields.io/docsrs/dfs?logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCA1MTIgNTEyIj48cGF0aCBkPSJNNDg4LjYgMjUwLjJMMzkyIDIxNFYxMDUuNWMwLTE1LTkuMy0yOC40LTIzLjQtMzMuN2wtMTAwLTM3LjVjLTguMS0zLjEtMTcuMS0zLjEtMjUuMyAwbC0xMDAgMzcuNWMtMTQuMSA1LjMtMjMuNCAxOC43LTIzLjQgMzMuN1YyMTRsLTk2LjYgMzYuMkM5LjMgMjU1LjUgMCAyNjguOSAwIDI4My45VjM5NGMwIDEzLjYgNy43IDI2LjEgMTkuOSAzMi4ybDEwMCA1MGMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAzLjktNTIgMTAzLjkgNTJjMTAuMSA1LjEgMjIuMSA1LjEgMzIuMiAwbDEwMC01MGMxMi4yLTYuMSAxOS45LTE4LjYgMTkuOS0zMi4yVjI4My45YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43ek0zNTggMjE0LjhsLTg1IDMxLjl2LTY4LjJsODUtMzd2NzMuM3pNMTU0IDEwNC4xbDEwMi0zOC4yIDEwMiAzOC4ydi42bC0xMDIgNDEuNC0xMDItNDEuNHYtLjZ6bTg0IDI5MS4xbC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnptMjQwIDExMmwtODUgNDIuNXYtNzkuMWw4NS0zOC44djc1LjR6bTAtMTEybC0xMDIgNDEuNC0xMDItNDEuNHYtLjZsMTAyLTM4LjIgMTAyIDM4LjJ2LjZ6IiBzdHlsZT0iZmlsbDojZmZmZmZmIj48L3BhdGg+PC9zdmc+Cg==)](https://docs.rs/dfs)
[![MIT license](https://img.shields.io/crates/l/dfs)](https://github.com/FedericoStra/dfs/blob/master/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/FedericoStra/dfs/rust.yml)](https://github.com/FedericoStra/dfs/actions/workflows/rust.yml)
![Lines of code](https://tokei.rs/b1/github/FedericoStra/dfs?category=code)

> Dotfiles synchronizer

This is a work in progress, at a very early stage of development.

```
dfs help
```

## Structure of the available commands

This is a sketch of the intended structure and does not reflect the actual implementation at the moment:

- config
    + init
    + show
- status
- profile
    + list
    + new
    + remove
    + activate
    + deactivate
- link
- unlink
- add
- remove

## How to

### List available profiles

```bash
dfs profile list
```

### Create a new profile

```bash
dfs profile new <name>
```

### Activate a profile

This will add symlinks for every tracked file.
Add `--force` to allow overwriting files which are not symlinks.

```bash
dfs profile activate <name>
```

### Selectively link/unlink files from the profile

`link` creates a symlink for the specified paths if they are tracked by the current profile.
`unlink` materializes (converts from symlinks to actual files) the specified tracked paths.

```bash
dfs link <path>...
dfs unlink <path>...
```
