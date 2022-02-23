use std::fs;
use std::path::Path;

use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};
use git2::build::RepoBuilder;
use regex::Regex;

use crate::domain::clone_repo_task::CloneRepoTaskError::{
    CouldNotCloneRepo, CouldNotDeleteExistingRepoDir, CouldNotExtractRepoName,
};

lazy_static! {
    static ref REPO_NAME_REGEX: Regex = Regex::new(r".*/(.*(\.))").unwrap();
}

pub struct CloneRepoTask {}

pub struct CloneRepoTaskResult {
    pub repo_path: String,
    pub git_repository: Repository,
}

impl CloneRepoTask {
    pub fn new() -> CloneRepoTask {
        return CloneRepoTask {};
    }

    pub fn execute(
        &self,
        url: &'static str,
        into_dir_path: &'static str,
        ssh_passphrase: &String,
        ssh_key_path: &String,
    ) -> Result<CloneRepoTaskResult, CloneRepoTaskError> {
        return self
            .extract_repo_name(url)
            .and_then(|data_holder_one| self.delete_repo_dir(into_dir_path, data_holder_one))
            .and_then(|data_holder_two| {
                self.clone_repo(url, data_holder_two, ssh_passphrase, ssh_key_path)
            });
    }

    fn extract_repo_name(&self, url: &str) -> Result<TempDataHolderOne, CloneRepoTaskError> {
        REPO_NAME_REGEX
            .captures(url)
            .ok_or(CouldNotExtractRepoName)
            .map(|captures| {
                captures
                    .get(1)
                    .map_or(url, |m| m.as_str())
                    .to_string()
                    .replace(".", "")
            })
            .map(|repo_name| TempDataHolderOne { repo_name })
    }

    fn delete_repo_dir(
        &self,
        into_dir_path: &'static str,
        first: TempDataHolderOne,
    ) -> Result<TempDataHolderTwo, CloneRepoTaskError> {
        let repo_name = first.repo_name;
        let formatted_repo_path = format!("{0}/{1}", into_dir_path, repo_name);
        let repo_path = Path::new(formatted_repo_path.as_str()).to_owned();
        let second = TempDataHolderTwo {
            formatted_repo_path,
        };

        if repo_path.exists() {
            fs::remove_dir_all(repo_path.as_path())
                .map_err(|_| CouldNotDeleteExistingRepoDir)
                .map(|_| second)
        } else {
            Ok(second)
        }
    }

    fn clone_repo(
        &self,
        url: &str,
        second: TempDataHolderTwo,
        ssh_passphrase: &String,
        ssh_key_path: &String,
    ) -> Result<CloneRepoTaskResult, CloneRepoTaskError> {
        let repo_path = Path::new(second.formatted_repo_path.as_str());
        let ssh_key_path = Path::new(ssh_key_path.as_str());
        let ssh_passphrase = if !ssh_passphrase.trim().is_empty() {
            Some(ssh_passphrase.as_str())
        } else {
            None
        };
        let mut callback = RemoteCallbacks::new();

        callback.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key(
                username_from_url.unwrap(),
                None,
                ssh_key_path,
                ssh_passphrase,
            )
        });

        let mut fo = FetchOptions::new();
        fo.remote_callbacks(callback);

        let mut builder = RepoBuilder::new();
        builder.fetch_options(fo);

        builder
            .clone(url, repo_path)
            .map_err(|_| CouldNotCloneRepo)
            .map(|repo| {
                CloneRepoTaskResult {
                    repo_path: second.formatted_repo_path,
                    git_repository: repo,
                }
            })
    }
}

struct TempDataHolderOne {
    repo_name: String,
}

struct TempDataHolderTwo {
    formatted_repo_path: String,
}

pub enum CloneRepoTaskError {
    CouldNotExtractRepoName,
    CouldNotDeleteExistingRepoDir,
    CouldNotCloneRepo,
}
