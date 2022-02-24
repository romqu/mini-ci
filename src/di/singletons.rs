use std::lazy::SyncOnceCell;

use crate::domain::deploy_service::DeployService;

pub static DEPLOY_SERVICE_CELL: SyncOnceCell<DeployService> = SyncOnceCell::new();


