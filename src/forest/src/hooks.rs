use crate::env::CanisterEnvironment;
use crate::SERVICE;
use ic_cdk::init;
use crate::service::ForestService;

#[init]
fn init() {
    ic_cdk::setup();

    let mut init_service = ForestService::default();
    init_service.env = Box::new(CanisterEnvironment {});

    SERVICE.with(|service| *service.borrow_mut() = init_service);
}
