mod repo;
mod env;
mod store;
mod service;
mod types;
mod hooks;

use candid::Principal;
use ic_cdk::{query, update, caller};
use types::*;
use service::SERVICE;



#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[query]
fn whoami() -> Principal {
    caller()
}

#[update]
// #[candid::candid_method]
fn open_hypha(args: HyphaArgs) -> HyphaID {
    SERVICE.with(|service| service.borrow_mut().open_hypha(args))
}

#[query]
// #[candid::candid_method]
fn get_hypha(id: HyphaID) -> Result<Hypha, String> {
    SERVICE.with(|service| service.borrow_mut().get_hypha(id))
}