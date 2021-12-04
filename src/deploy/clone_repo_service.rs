use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::{env, fs};

use git2::build::RepoBuilder;
use git2::{Cred, FetchOptions, RemoteCallbacks};
use regex::Regex;

use crate::data::repo_info_repository::{RepoInfoEntity, RepoInfoRepository};
use crate::deploy::clone_repo_service::CloneRepoServiceError::{
    CouldNotCloneRepo, CouldNotDeleteExistingRepoDir, CouldNotFindHomePath, CouldNotSaveRepoInfo,
};

const HOME_VAR: &str = "HOME";

lazy_static! {
    static ref REPO_NAME_REGEX: Regex = Regex::new(r".*/(.*(\.))").unwrap();
}

pub struct CloneRepoService {
    repo_info_repo: Rc<RefCell<RepoInfoRepository>>,
}

struct First {
    home_path: String,
    repo_name: String,
}

struct Second {
    home_path: String,
    repo_name: String,
    formatted_repo_path: String,
}

impl CloneRepoService {
    pub fn new(repo_info_repo: Rc<RefCell<RepoInfoRepository>>) -> CloneRepoService {
        return CloneRepoService { repo_info_repo };
    }

    pub fn execute(
        self,
        url: &'static str,
        into_dir_path: &'static str,
        ssh_key_name: &'static str,
    ) -> Result<RepoInfoEntity, CloneRepoServiceError> {
        return self
            .find_home_path()
            .map(|home_path| self.extract_repo_name(url, home_path))
            .and_then(|first| self.delete_repo_dir(into_dir_path, first))
            .and_then(|second| self.clone_repo(url, ssh_key_name, second))
            .and_then(|path| self.save_repo_info(url, path));
    }

    fn find_home_path(&self) -> Result<String, CloneRepoServiceError> {
        env::var(HOME_VAR).map_err(|_| CouldNotFindHomePath)
    }

    fn extract_repo_name(&self, url: &str, home_path: String) -> First {
        let repo_name = REPO_NAME_REGEX
            .captures(url)
            .unwrap()
            .get(1)
            .map_or(url, |m| m.as_str())
            .to_string()
            .replace(".", "");

        First {
            home_path,
            repo_name,
        }
    }

    fn delete_repo_dir(
        &self,
        into_dir_path: &'static str,
        first: First,
    ) -> Result<Second, CloneRepoServiceError> {
        let repo_name = first.repo_name;
        let formatted_repo_path = format!("{0}/{1}", into_dir_path, repo_name);
        let repo_path = Path::new(formatted_repo_path.as_str()).to_owned();
        let second = Second {
            home_path: first.home_path,
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
        ssh_key_name: &str,
        second: Second,
    ) -> Result<String, CloneRepoServiceError> {
        let formatted_ssh_key_path = format!("{0}/.ssh/{1}", second.home_path, ssh_key_name);
        let repo_path = Path::new(second.formatted_repo_path.as_str());
        let ssh_key_path = Path::new(formatted_ssh_key_path.as_str());
        let mut callback = RemoteCallbacks::new();

        callback.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key(username_from_url.unwrap(), None, ssh_key_path, None)
        });

        let mut fo = FetchOptions::new();
        fo.remote_callbacks(callback);

        let mut builder = RepoBuilder::new();
        builder.fetch_options(fo);

        builder
            .clone(url, repo_path)
            .map_err(|_| CouldNotCloneRepo)
            .map(|_| second.formatted_repo_path)
    }

    fn save_repo_info(
        self,
        url: &str,
        formatted_repo_path: String,
    ) -> Result<RepoInfoEntity, CloneRepoServiceError> {
        self.repo_info_repo
            .borrow_mut()
            .save(String::from(url), RepoInfoEntity::new(formatted_repo_path))
            .ok_or(CouldNotSaveRepoInfo)
    }
}

#[derive(Debug)]
pub enum CloneRepoServiceError {
    CouldNotFindHomePath,
    CouldNotDeleteExistingRepoDir,
    CouldNotCloneRepo,
    CouldNotSaveRepoInfo,
}
