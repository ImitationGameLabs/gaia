
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, StableCell};

use std::cell::RefCell;
use crate::env::{EmptyEnvironment, CanisterEnvironment, Environment};
use crate::types::*;

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    pub static SERVICE: RefCell<ForestService> = RefCell::default();
}

pub struct ForestService {
    pub env: Box<dyn Environment>,

    // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
    // return a memory that can be used by stable structures.
    //
    // MemoryManager::init(DefaultMemoryImpl::default())
    pub memory_manager: MemoryManager<DefaultMemoryImpl>, 

    // TODO: Maps sha1 to object
    pub objects: StableBTreeMap<String, String, Memory>,
    pub hyphae: StableBTreeMap<HyphaID, Hypha, Memory>,

    pub next_hypha_id: StableCell<HyphaID, Memory>,
}

impl Default for ForestService {
    fn default() -> Self {
        let memory_manager = MemoryManager::init(DefaultMemoryImpl::default());
        
        let objects = StableBTreeMap::init(
               memory_manager.get(MemoryId::new(0))
        );

        let hyphae = StableBTreeMap::init(
            memory_manager.get(MemoryId::new(1))
        );

        // FIXME unwrap
        let next_hypha_id = StableCell::init(
            memory_manager.get(MemoryId::new(2)),
            0
        ).unwrap();

        Self {
            env: Box::new(CanisterEnvironment {}),
            memory_manager,
            objects,
            hyphae,
            next_hypha_id,
        }
    }
}


impl ForestService {
    pub fn new_hypha(&mut self, args: HyphaArgs) -> HyphaID {
        let id = self.get_next_hypha_id();

        let now = self.env.now();

        let hypha = Hypha {
            id,
            forest_id: self.env.canister_id(),
            title: args.title,
            description: args.description,
            comments: vec![],
            tags: vec![],
            creator: self.env.caller(),
            created_at: now,
            updated_at: now,
            closed_at: now,
            state: HyphaState::Open,
            parent: None,
            children: None,
        };

        self.hyphae.insert(id, hypha);

        id
    }

    pub fn get_hypha(&self, id: HyphaID) -> Result<Hypha, String> {
        self.hyphae.get(&id).ok_or(String::from("Not found"))
    }

    fn get_next_hypha_id(&mut self) -> HyphaID {
        let id = *self.next_hypha_id.get();
        let _ = self.next_hypha_id.set(id + 1);
        id
    }
}