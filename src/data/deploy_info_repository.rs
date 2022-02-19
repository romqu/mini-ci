use std::collections::HashMap;

use cmd_lib::FunChildren;
use git2::Repository;

pub struct DeployInfoEntity {
    pub ssh_git_url: &'static str,
    pub command_builders: Vec<fn(String) -> std::io::Result<FunChildren>>,
    pub path: String,
    pub git_repository: Repository,
}

pub struct DeployInfoRepository {
    cache: HashMap<String, DeployInfoEntity>,
}

impl DeployInfoRepository {
    pub fn new(cache: HashMap<String, DeployInfoEntity>) -> DeployInfoRepository {
        DeployInfoRepository { cache }
    }

    pub fn save(&mut self, key: String, entity: DeployInfoEntity) -> Option<DeployInfoEntity> {
        self.cache.insert(key, entity)
    }

    pub fn get(&self, key: &'static str) -> Option<&DeployInfoEntity> {
        self.cache.get(key)
    }
}
