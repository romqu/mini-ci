use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct RepoInfoEntity {
    pub path: String,
}

impl RepoInfoEntity {
    pub fn new(path: String) -> RepoInfoEntity {
        RepoInfoEntity { path }
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
