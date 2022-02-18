use std::collections::HashMap;

use cmd_lib::FunChildren;

pub struct DeployInfoEntity {
    pub ssh_git_url: &'static str,
    pub command_builders: Vec<fn(String) -> std::io::Result<FunChildren>>,
}

pub struct DeployInfoRepository {
    db: HashMap<String, DeployInfoEntity>,
}

impl DeployInfoRepository {
    pub fn new(db: HashMap<String, DeployInfoEntity>) -> DeployInfoRepository {
        DeployInfoRepository { db }
    }

    pub fn save(&mut self, key: String, entity: DeployInfoEntity) -> Option<DeployInfoEntity> {
        self.db.insert(key, entity)
    }

    pub fn get(&self, key: &'static str) -> Option<&DeployInfoEntity> {
        self.db.get(key)
    }
}
