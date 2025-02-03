mod repo;
mod store;

use candid::Principal;
use ic_cdk::{query, caller};

#[query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[query]
fn whoami() -> Principal {
    caller()
}