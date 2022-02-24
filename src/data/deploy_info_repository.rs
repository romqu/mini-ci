use std::collections::hash_map::OccupiedError;
use std::collections::HashMap;

use cmd_lib::FunChildren;
use git2::Repository;

pub struct DeployInfoEntity {
    pub ssh_git_url: &'static str,
    pub command_builders: Vec<fn(String) -> std::io::Result<FunChildren>>,
    pub repo_path: String,
    pub git_repository: Repository,
}

pub struct DeployInfoRepository {
    cache: HashMap<String, DeployInfoEntity>,
}

impl DeployInfoRepository {
    pub fn new(cache: HashMap<String, DeployInfoEntity>) -> DeployInfoRepository {
        DeployInfoRepository { cache }
    }

    pub fn save(
        &mut self,
        key: String,
        entity: DeployInfoEntity,
    ) -> Result<&mut DeployInfoEntity, OccupiedError<'_, String, DeployInfoEntity>> {
        self.cache.try_insert(key, entity)
    }

    pub fn get(&self, key: &String) -> Option<&DeployInfoEntity> {
        self.cache.get(key)
    }
}
