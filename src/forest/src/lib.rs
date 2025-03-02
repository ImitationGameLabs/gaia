mod repo;
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

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[update]
// #[candid::candid_method]
fn open_hypha(args: HyphaArgs) -> HyphaID {
    SERVICE.with(|service| service.borrow_mut().new_hypha(args))
}

#[query]
// #[candid::candid_method]
fn get_hypha(id: HyphaID) -> Result<Hypha, String> {
    SERVICE.with(|service| service.borrow_mut().get_hypha(id))
}