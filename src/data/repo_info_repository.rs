use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use git2::Repository;

pub struct RepoInfoEntity {
    pub path: String,
    pub repository: Repository
}

impl RepoInfoEntity {
    pub fn new(path: String, repository: Repository) -> RepoInfoEntity {
        RepoInfoEntity { path, repository }
    }
}

pub struct RepoInfoRepository {
    db: HashMap<String, RepoInfoEntity>,
}

impl RepoInfoRepository {
    pub fn new(db: HashMap<String, RepoInfoEntity>) -> Rc<RefCell<RepoInfoRepository>> {
        Rc::new(RefCell::new(RepoInfoRepository { db }))
    }

    pub fn save(&mut self, key: String, entity: RepoInfoEntity) -> Option<RepoInfoEntity> {
        self.db.insert(key, entity)
    }

    pub fn get(&self, key: &'static str) -> Option<&RepoInfoEntity> {
        self.db.get(key)
    }
}
