
use ic_stable_structures::memory_manager::{MemoryId, VirtualMemory, MemoryManager};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap, StableCell};

use crate::env::Environment;
use crate::types::*;

type Memory = VirtualMemory<DefaultMemoryImpl>;

const OBJECTS_MEMORY_ID: MemoryId = MemoryId::new(0);
const HYPHAE_MEMORY_ID: MemoryId = MemoryId::new(1);
const NEXT_HYPHA_ID_MEMORY_ID: MemoryId = MemoryId::new(2);

pub struct ForestService {
    pub env: Box<dyn Environment>,

    // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
    // return a memory that can be used by stable structures.
    //
    // MemoryManager::init(DefaultMemoryImpl::default())
    pub memory_manager: MemoryManager<DefaultMemoryImpl>, 

    // TODO: Maps sha1 to object
    pub objects: StableBTreeMap<String, String, Memory>,
    // pub refs: StableBTreeMap<String, String, Memory>,

    // A new form of issues
    pub hyphae: StableBTreeMap<HyphaID, Hypha, Memory>,
    pub next_hypha_id: StableCell<HyphaID, Memory>,

    // TODO Merges
}

impl ForestService {
    pub fn new(env: Box<dyn Environment>) -> Self {
        let memory_manager = MemoryManager::init(DefaultMemoryImpl::default());
        
        let objects = StableBTreeMap::init(
               memory_manager.get(OBJECTS_MEMORY_ID)
        );

        let hyphae = StableBTreeMap::init(
            memory_manager.get(HYPHAE_MEMORY_ID)
        );

        // FIXME unwrap
        let next_hypha_id = StableCell::init(
            memory_manager.get(NEXT_HYPHA_ID_MEMORY_ID),
            0
        ).unwrap();

        Self {
            env,
            memory_manager,
            objects,
            hyphae,
            next_hypha_id,
        }
    }

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