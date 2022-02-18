use std::{env, fs};
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};
use git2::build::RepoBuilder;
use regex::Regex;

use crate::data::repo_info_repository::{GitRepoInfoEntity, GitRepoInfoRepository};
use crate::domain::clone_repo_task::CloneRepoServiceError::{
    CouldNotCloneRepo, CouldNotDeleteExistingRepoDir, CouldNotExtractRepoName,
    CouldNotFindHomePath, CouldNotSaveRepoInfo,
};

const HOME_VAR: &str = "HOME";

lazy_static! {
    static ref REPO_NAME_REGEX: Regex = Regex::new(r".*/(.*(\.))").unwrap();
}

pub struct CloneRepoTask {
    repo_info_repo: Rc<RefCell<GitRepoInfoRepository>>,
}

struct TempDataHolderOne {
    repo_name: String,
}

struct TempDataHolderSecond {
    repo_name: String,
    formatted_repo_path: String,
}

struct TempDataHolderThird {
    formatted_repo_path: String,
    repository: Repository,
}

impl CloneRepoTask {
    pub fn new(repo_info_repo: Rc<RefCell<GitRepoInfoRepository>>) -> CloneRepoTask {
        return CloneRepoTask { repo_info_repo };
    }

    pub fn execute(
        &self,
        url: &'static str,
        into_dir_path: &'static str,
        ssh_passphrase: &String,
        ssh_key_path: &String,
    ) -> Result<GitRepoInfoEntity, CloneRepoServiceError> {
        return self
            .extract_repo_name(url)
            .and_then(|first| self.delete_repo_dir(into_dir_path, first))
            .and_then(|second| self.clone_repo(url, second, ssh_passphrase, ssh_key_path))
            .and_then(|third| self.save_repo_info(url, third));
    }

    fn extract_repo_name(&self, url: &str) -> Result<TempDataHolderOne, CloneRepoServiceError> {
        REPO_NAME_REGEX
            .captures(url)
            .ok_or(CouldNotExtractRepoName)
            .map(|captures| {
                captures
                    .get(1)
                    .map_or(url, |m| m.as_str())
                    .to_string()
                    .replace(".", "")
            }).map(|repo_name| TempDataHolderOne { repo_name })
    }

    fn delete_repo_dir(
        &self,
        into_dir_path: &'static str,
        first: TempDataHolderOne,
    ) -> Result<TempDataHolderSecond, CloneRepoServiceError> {
        let repo_name = first.repo_name;
        let formatted_repo_path = format!("{0}/{1}", into_dir_path, repo_name);
        let repo_path = Path::new(formatted_repo_path.as_str()).to_owned();
        let second = TempDataHolderSecond {
            repo_name,
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
        second: TempDataHolderSecond,
        ssh_passphrase: &String,
        ssh_key_path: &String,
    ) -> Result<TempDataHolderThird, CloneRepoServiceError> {
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
                TempDataHolderThird {
                    formatted_repo_path: second.formatted_repo_path,
                    repository: repo,
                }
            })
    }

    fn save_repo_info(
        &self,
        url: &str,
        third: TempDataHolderThird,
    ) -> Result<GitRepoInfoEntity, CloneRepoServiceError> {
        self.repo_info_repo
            .borrow_mut()
            .save(
                String::from(url),
                GitRepoInfoEntity::new(third.formatted_repo_path, third.repository),
            )
            .ok_or(CouldNotSaveRepoInfo)
    }
}

#[derive(Debug)]
pub enum CloneRepoServiceError {
    CouldNotExtractRepoName,
    CouldNotFindHomePath,
    CouldNotDeleteExistingRepoDir,
    CouldNotCloneRepo,
    CouldNotSaveRepoInfo,
}
