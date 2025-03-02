

use std::borrow::Cow;

use candid::{CandidType, Deserialize, Principal, Encode, Decode};
use ic_stable_structures::storable::{Storable, Bound};

pub type HyphaID = u32;
pub type Timestamp = u64;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum HyphaState {
    Open,
    Close,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Child {
    id: HyphaID,
    weight: u32,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum ChildrenRelation {
    Parallel,
    Serial,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HyphaChildren {
    pub children: Vec<Child>,
    pub children_relation: ChildrenRelation,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HyphaArgs {
    pub title: String,
    pub description: String,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Hypha {
    pub id: HyphaID,
    pub forest_id: Principal,

    pub title: String,
    pub description: String,
    
    pub tags: Vec<String>,

    pub creator: Principal,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub closed_at: Timestamp,
    pub state: HyphaState,

    pub parent: Option<HyphaID>,
    pub children: Option<HyphaChildren>,
}

impl Storable for Hypha {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Unbounded;
    
    fn to_bytes_checked(&self) -> Cow<[u8]> {
        let bytes = self.to_bytes();

        // TODO check the content size of hypha.

        bytes
    }
}
