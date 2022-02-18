use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use git2::Repository;

pub struct GitRepoInfoEntity {
    pub path: String,
    pub git_repository: Repository,
}

impl GitRepoInfoEntity {
    pub fn new(path: String, git_repository: Repository) -> GitRepoInfoEntity {
        GitRepoInfoEntity { path, git_repository }
    }
}

pub struct GitRepoInfoRepository<> {
    db: HashMap<String, GitRepoInfoEntity>,
}

impl GitRepoInfoRepository {
    pub fn new(db: HashMap<String, GitRepoInfoEntity>) -> GitRepoInfoRepository {
        GitRepoInfoRepository { db }
    }

    pub fn save(&mut self, key: String, entity: GitRepoInfoEntity) -> Option<GitRepoInfoEntity> {
        self.db.insert(key, entity)
    }

    pub fn get(&self, key: &'static str) -> Option<&GitRepoInfoEntity> {
        self.db.get(key)
    }
}
