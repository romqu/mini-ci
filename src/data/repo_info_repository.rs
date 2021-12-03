use std::collections::HashMap;

pub struct RepoInfoEntity {
    path: String,
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
    pub fn new(db: HashMap<String, RepoInfoEntity>) -> RepoInfoRepository {
        RepoInfoRepository { db }
    }

    pub fn save(&mut self, key: String, entity: RepoInfoEntity) -> Option<RepoInfoEntity> {
        self.db.insert(key, entity)
    }

    pub fn get(&self, key: &'static str) -> Option<&RepoInfoEntity> {
        self.db.get(key)
    }
}
