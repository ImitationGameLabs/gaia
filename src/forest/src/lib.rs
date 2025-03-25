
mod env;
mod service;
mod types;
mod hooks;

use env::CanisterEnvironment;
use types::*;
use service::ForestService;

use std::cell::RefCell;

use ic_cdk::{query, update};

thread_local! {
    pub static SERVICE: RefCell<ForestService> = RefCell::new(
        ForestService::new(Box::new(CanisterEnvironment{}))
    );
}

#[update]
fn open_hypha(args: HyphaArgs) -> HyphaID {
    SERVICE.with(|service| service.borrow_mut().new_hypha(args))
}

#[query]
fn get_hypha(id: HyphaID) -> Result<Hypha, String> {
    SERVICE.with(|service| service.borrow().get_hypha(id))
}

#[update]
fn push(args: PushArgs) -> Result<(), String> {
    SERVICE.with(|service: &RefCell<ForestService>| service.borrow_mut().push(args))
}

#[query]
fn fetch(args: FetchArgs) -> Result<Vec<u8>, String> {
    SERVICE.with(|service| service.borrow().fetch(args))
}