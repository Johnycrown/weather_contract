use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
#[derive(CandidType, Deserialize)]
struct Weather{
    temperature: u32,
    pressure: u32,
    humidity: u32,
    wind_speed: u32,

}

impl Storable for Weather {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }


    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}


const MAXIMUM_VALUE_SIZE: u32 = 50;

impl BoundedStorable for Weather {
    const MAX_SIZE: u32 = MAXIMUM_VALUE_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

type Memory = VirtualMemory<DefaultMemoryImpl>;


thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));


    static WEATHER_MAP: RefCell<StableBTreeMap<u64, Weather, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );


    static PARTICIPATION_PERCENTAGE_MAP: RefCell<StableBTreeMap<u64, u64, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
        )
    );
}
