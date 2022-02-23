use std::any::Any;
use std::cell::RefCell;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::thread;
use std::thread::JoinHandle;

use cmd_lib::*;
use git2::{Branch, BranchType, ObjectType};
use git2::build::CheckoutBuilder;

use crate::data::deploy_info_repository::DeployInfoEntity;
use crate::DeployInfoRepository;
use crate::domain::deploy_service::DeployServiceError::{
    CouldNotCheckoutBranch, CouldNotGetBranch, CouldNotGetRepoInfo,
};
use crate::domain::init_service::DeployInfo;
use crate::entrypoint::github_push_event_dto::GithubPushEventDto;

pub struct DeployService {
    deploy_info_repo: Rc<RefCell<DeployInfoRepository>>,
}

impl DeployService {
    pub fn new(deploy_info_repo: Rc<RefCell<DeployInfoRepository>>) -> DeployService {
        return DeployService { deploy_info_repo };
    }

    pub fn execute(&self, dto: GithubPushEventDto) -> Result<JoinHandle<()>, DeployServiceError> {
        return self
            .deploy_info_repo
            .borrow()
            .get(&dto.repository.ssh_url)
            .ok_or(CouldNotGetRepoInfo)
            .and_then(|deploy_info| Self::get_branch(dto, deploy_info))
            .and_then(|temp_data_holder| Self::checkout_branch(temp_data_holder))
            .map(|repo_info| Self::execute_deploy_commands(repo_info));
    }

    fn get_branch(
        dto: GithubPushEventDto,
        deploy_info: &DeployInfoEntity,
    ) -> Result<TempDataHolderOne, DeployServiceError> {
        let refs = dto.ref_field;

        refs.rfind("/")
            .ok_or(CouldNotGetBranch)
            .and_then(|position| {
                let branch_name = &refs[position..];
                let formatted_branch_name = format!("origin/{}", branch_name);

                deploy_info
                    .git_repository
                    .find_branch(formatted_branch_name.as_str(), BranchType::Remote)
                    .map(|branch| {
                        TempDataHolderOne {
                            branch,
                            deploy_info,
                            refs,
                        }
                    })
                    .map_err(|_| CouldNotGetBranch)
            })
    }

    fn checkout_branch(first: TempDataHolderOne) -> Result<&DeployInfoEntity, DeployServiceError> {
        let deploy_info = first.deploy_info;
        let git_repository = &deploy_info.git_repository;
        let branch = first.branch;
        let refs = first.refs;

        branch
            .get()
            .peel_to_tree()
            .and_then(|tree| deploy_info.git_repository.find_tree(tree.id()))
            .and_then(|tree| git_repository.find_object(tree.id(), Option::Some(ObjectType::Tree)))
            .and_then(|git_object| {
                git_repository.checkout_tree(&git_object, Some(CheckoutBuilder::default().force()))
            })
            .and_then(|_| git_repository.set_head(refs.as_str()))
            .map_err(|_| CouldNotCheckoutBranch)
            .map(|_| deploy_info)
    }

    fn execute_deploy_commands(deploy_info: &DeployInfoEntity) -> JoinHandle<()> {
        let path = &deploy_info.repo_path;
        let path_copy = path.clone();

        thread::spawn(move || {
            let commands = vec![
                spawn_with_output!(cd ${path_copy} | docker-compose build --build-arg ENVPROFILE=dev),
                spawn_with_output!(docker-compose -f ${path_copy}/docker-compose.yml up --force-recreate --no-deps -d api),
            ];

            for command in commands {
                command.unwrap().wait_with_pipe(&mut |pipe| {
                    BufReader::new(pipe)
                        .lines()
                        .filter_map(|line| line.ok())
                        .for_each(|line| println!("{}", line));
                });
            }
        })
    }
}

struct TempDataHolderOne<'a> {
    branch: Branch<'a>,
    deploy_info: &'a DeployInfoEntity,
    refs: String,
}

#[derive(Debug)]
pub enum DeployServiceError {
    CouldNotGetBranch,
    CouldNotGetRepoInfo,
    CouldNotCheckoutBranch,
}
