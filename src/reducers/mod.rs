use std::hash::Hasher;
use std::{collections::HashMap, hash::Hash};
use std::collections::hash_map::DefaultHasher;
use crate::redux::{Store, Reducer, SimpleHashable, Effect};
use crate::api::{Token, TaiglaApi};
pub use api::*;

mod api;

#[derive(PartialEq, Clone)]
pub enum IndexerModalState {
    New,
    Id(u64),
    Close
}

#[derive(Hash)]
pub enum TaiglaData {
    Token,
    Api(api::ApiData),
    IndexerModalState
}

impl SimpleHashable for TaiglaData {
    fn simple_hash(&self) -> crate::redux::SimpleHash {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

pub enum TaiglaEvent {
    ApiEvent(api::ApiEvent),
    SetToken(String),
    SetIndexerModalState(IndexerModalState)
}

pub struct TaiglaStore {
    pub cache: HashMap<api::ApiData, api::InnerRequestState>,
    pub token: Token,
    pub api: TaiglaApi,
    pub indexer_modal_state: IndexerModalState
}

impl TaiglaStore {
    pub fn new() -> Self {
        let token = Token::default();
        Self {
            cache: HashMap::new(),
            token: token.clone(),
            api: TaiglaApi::new("http://localhost:1234/", token),
            indexer_modal_state: IndexerModalState::Close
        }
    }

    pub fn token(&self) -> (TaiglaData, Token) {
        (TaiglaData::Token, self.token.clone())
    }

    pub fn indexer_modal_state(&self) -> (TaiglaData, IndexerModalState) {
        (TaiglaData::IndexerModalState, self.indexer_modal_state.clone())
    }
}

impl Store for TaiglaStore {
    type Event = TaiglaEvent;

    fn handle(&mut self, event: Self::Event) -> Effect<Self> {
        match event {
            TaiglaEvent::ApiEvent(e) => return e.reduce(self),
            TaiglaEvent::SetToken(token) => self.token.set(&token),
            TaiglaEvent::SetIndexerModalState(state) => self.indexer_modal_state = state
        }
        Effect::NONE
    }
}
