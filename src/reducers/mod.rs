use std::hash::Hasher;
use std::{collections::HashMap, hash::Hash};
use std::collections::hash_map::DefaultHasher;
use crate::redux::{Store, Reducer, SimpleHashable, Effect};
pub use api::*;

mod api;

#[derive(Hash)]
pub enum TaiglaData {
    Api(api::ApiData)
}

impl SimpleHashable for TaiglaData {
    fn simple_hash(&self) -> crate::redux::SimpleHash {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

pub enum TaiglaEvent {
    ApiEvent(api::ApiEvent)
}

pub struct TaiglaStore {
    pub cache: HashMap<api::ApiData, api::RequestState>,
    pub token: Option<String>
}

impl TaiglaStore {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            token: None
        }
    }
}

impl Store for TaiglaStore {
    type Event = TaiglaEvent;

    fn handle(&mut self, event: Self::Event) -> Effect<Self> {
        match event {
            TaiglaEvent::ApiEvent(e) => e.reduce(self)
        }
    }
}
