# git-global

[![Crates.io](https://img.shields.io/crates/v/git-global.svg)](https://crates.io/crates/git-global)
[![Crates.io](https://img.shields.io/crates/d/git-global.svg)](https://crates.io/crates/git-global)

Use `git-global` to keep track of all the git repositories on your machine.

This is a Rust program that you can install with `cargo install git-global`.
(To obtain `cargo` and Rust, see https://rustup.rs.) Once installed, you gain
an extra git subcommand that you can run from anywhere to check up on all your
git repos: `git global`.

Use `git global <subcommand>` to:

* `git global [status]`: show `git status -s` for all your git repos (the
  default subcommand)
* `git global info`: show information about git-global itself (configuration,
  number of known repos, etc.)
* `git global list`: show all git repos that git-global knows about
* `git global scan`: search your filesystem for git repos and update the cache

## Configuration

To change the behavior of `git-global`, you can do so with --- wait for it
--- [git's global
configuration](https://git-scm.com/book/en/v2/Customizing-Git-Git-Configuration)!

To set the root directory for repo discovery to something other than your home
directory:
```
git config --global global.basedir /some/path
```

To add patterns to exclude while walking directories:
```
git config --global global.ignore .cargo,.vim,Library
```

## Ideas

The following are some ideas I've had about future subcommands and features:

* `git global unstaged`: show all repos that have unstaged changes
* `git global staged`: show all repos that have staged changes
* `git global stashed`: show all repos that have stashed changes
* `git global dirty`: show all repos that have changes of any kind
* `git global branched`: show all repos not on `master` (TODO: or a different
  default branch in .gitconfig)
* `git global duplicates`: show repos that are checked out to multiple places
* `git global remotes`: show all remotes (TODO: why? maybe filter by hostname?)

* `git global add <path>`: add a git repo to the cache that would not be found in a scan
* `git global ignore <path>`: ignore a git repo and remove it from the cache
* `git global ignored`: show which git repos are currently being ignored
* `git global monitor`: launch a daemon to watch git dirs with inotify
* `git global pull`: pull down changes from default tracking branch for clean repos

* stream results to `STDOUT` as the come in (from `git global status`, for
  example, so we don't have to wait until they're all collected)
* use `locate .git` if the DB is populated, instead of walking the filesystem
* make a `Subcommand` trait
* do concurrency generically, not just for the `status` subcommand

## Release Notes

* 0.2.0 (2019-03-18)
  * Include untracked files in status output.
  * Expand documentation and package metadata.
  * Update and change several dependencies.
  * Add some tests.
  * Several public API changes, such as:
    * Rename `GitGlobalConfig` to `Config`.
    * Rename `GitGlobalResult` to `Report`.
    * Move `get_repos` `find_repos`, and `cache_repos` functions to `Config`.
    * Split the `core` module into `config`, `repo`, and `report`.
  * Merge bug fix for scanning directories when nothing is configured to be
    ignored ([#1](https://github.com/peap/git-global/pull/1)).
* 0.1.0 (2017-01-31)
  * Initial release with these subcommands: help, info, list, scan, status.
