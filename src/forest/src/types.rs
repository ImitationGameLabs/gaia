

use std::borrow::Cow;

use candid::{CandidType, Deserialize, Principal, Encode, Decode};
use ic_stable_structures::storable::{Storable, Bound};

const MAX_VALUE_SIZE: u32 = 10000;

pub type HyphaID = u32;
pub type Timestamp = u64;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Comment {
    pub content: String,
    pub creator: Principal,
    pub create_at: Timestamp,
}

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
pub struct Hypha {
    pub id: HyphaID,
    pub forest_id: Principal,

    pub title: String,
    pub description: String,
    
    pub comments: Vec<Comment>,
    pub tags: Vec<String>,

    pub creator: Principal,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub closed_at: Timestamp,
    pub state: HyphaState,

    pub parent: Option<HyphaID>,
    pub children: Option<HyphaChildren>,
}


#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HyphaArgs {
    pub title: String,
    pub description: String,
}

impl Storable for Hypha {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: MAX_VALUE_SIZE,
        is_fixed_size: false,
    };
    
    fn to_bytes_checked(&self) -> Cow<[u8]> {
        let bytes = self.to_bytes();
        if let ic_stable_structures::storable::Bound::Bounded {
            max_size,
            is_fixed_size,
        } = Self::BOUND
        {
            if is_fixed_size {
                std::assert_eq!(
                    bytes.len(),
                    max_size as usize,
                    "expected a fixed-size element with length {} bytes, but found {} bytes",
                    max_size,
                    bytes.len()
                );
            } else {
                assert!(
                    bytes.len() <= max_size as usize,
                    "expected an element with length <= {} bytes, but found {} bytes",
                    max_size,
                    bytes.len()
                );
            }
        }
        bytes
    }
}
