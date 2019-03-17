//! Configuration of git-global.

use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use app_dirs::{app_dir, get_app_dir, AppDataType, AppInfo};
use dirs::home_dir;
use git2;
use walkdir::{DirEntry, WalkDir};

use core::Repo;

const APP: AppInfo = AppInfo {
    name: "git-global",
    author: "peap",
};
const CACHE_FILE: &'static str = "repos.txt";
const SETTING_BASEDIR: &'static str = "global.basedir";
const SETTING_IGNORED: &'static str = "global.ignore";

/// A container for git-global configuration options.
pub struct GitGlobalConfig {
    pub basedir: String,
    pub ignored_patterns: Vec<String>,
    pub cache_file: PathBuf,
}

impl GitGlobalConfig {
    pub fn new() -> GitGlobalConfig {
        let home_dir = home_dir()
            .expect("Could not determine home directory.")
            .to_str()
            .expect("Could not convert home directory path to string.")
            .to_string();
        let (basedir, patterns) = match git2::Config::open_default() {
            Ok(config) => (
                config.get_string(SETTING_BASEDIR).unwrap_or(home_dir),
                config
                    .get_string(SETTING_IGNORED)
                    .unwrap_or(String::new())
                    .split(",")
                    .map(|p| p.trim().to_string())
                    .collect(),
            ),
            Err(_) => (home_dir, Vec::new()),
        };
        let cache_file =
            match get_app_dir(AppDataType::UserCache, &APP, "cache") {
                Ok(mut dir) => {
                    dir.push(CACHE_FILE);
                    dir
                }
                Err(_) => panic!("TODO: work without XDG"),
            };
        GitGlobalConfig {
            basedir: basedir,
            ignored_patterns: patterns,
            cache_file: cache_file,
        }
    }

    /// Returns `true` if this directory entry should be included in scans.
    pub fn filter(&self, entry: &DirEntry) -> bool {
        let entry_path = entry.path().to_str().expect("DirEntry without path.");

        self.ignored_patterns
            .iter()
            .filter(|p| p != &"")
            .fold(true, |acc, pattern| acc && !entry_path.contains(pattern))
    }

    /// Returns boolean indicating if the cache file exists.
    pub fn has_cache(&self) -> bool {
        self.cache_file.as_path().exists()
    }

    /// Writes the given repo paths to the cache file.
    pub fn cache_repos(&self, repos: &Vec<Repo>) {
        if !self.cache_file.as_path().exists() {
            // Try to create the cache directory if the cache *file* doesn't
            // exist; app_dir() handles an existing directory just fine.
            match app_dir(AppDataType::UserCache, &APP, "cache") {
                Ok(_) => (),
                Err(e) => panic!("Could not create cache directory: {}", e),
            }
        }
        let mut f = File::create(&self.cache_file)
            .expect("Could not create cache file.");
        for repo in repos.iter() {
            match writeln!(f, "{}", repo.path()) {
                Ok(_) => (),
                Err(e) => panic!("Problem writing cache file: {}", e),
            }
        }
    }

    /// Returns the list of repos found in the cache file.
    pub fn get_cached_repos(&self) -> Vec<Repo> {
        let mut repos = Vec::new();
        if self.cache_file.as_path().exists() {
            let f = File::open(&self.cache_file)
                .expect("Could not open cache file.");
            let reader = BufReader::new(f);
            for line in reader.lines() {
                match line {
                    Ok(repo_path) => repos.push(Repo::new(repo_path)),
                    Err(_) => (), // TODO: handle errors
                }
            }
        }
        repos
    }
}

/// Walks the configured base directory, looking for git repos.
pub fn find_repos(config: &GitGlobalConfig) -> Vec<Repo> {
    let mut repos = Vec::new();
    let basedir = &config.basedir;

    println!(
        "Scanning for git repos under {}; this may take a while...",
        basedir
    );
    for entry in WalkDir::new(basedir)
        .into_iter()
        .filter_entry(|e| config.filter(e))
    {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_dir() && entry.file_name() == ".git" {
                    let parent_path = entry
                        .path()
                        .parent()
                        .expect("Could not determine parent.");
                    match parent_path.to_str() {
                        Some(path) => {
                            repos.push(Repo::new(path.to_string()));
                        }
                        None => (),
                    }
                }
            }
            Err(_) => (),
        }
    }
    repos.sort_by(|a, b| a.path().cmp(&b.path()));
    repos
}

/// Caches repo list to disk, in the XDG cache directory for git-global.
pub fn cache_repos(config: &mut GitGlobalConfig, repos: &Vec<Repo>) {
    config.cache_repos(repos);
}

/// Returns all known git repos, populating the cache first, if necessary.
pub fn get_repos(config: &mut GitGlobalConfig) -> Vec<Repo> {
    if !config.has_cache() {
        let repos = find_repos(config);
        cache_repos(config, &repos);
        repos
    } else {
        config.get_cached_repos()
    }
}