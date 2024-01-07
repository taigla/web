use std::hash::Hasher;
use std::{collections::HashMap, hash::Hash};
use std::collections::hash_map::DefaultHasher;
use crate::redux::{Store, Reducer, SimpleHashable, Effect};
use crate::api::{Token, TaiglaApi};
pub use api::*;

mod api;

#[derive(Hash)]
pub enum TaiglaData {
    Token,
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
    ApiEvent(api::ApiEvent),
    SetToken(String)
}

pub struct TaiglaStore {
    pub cache: HashMap<api::ApiData, api::InnerRequestState>,
    pub token: Token,
    pub api: TaiglaApi
}

impl TaiglaStore {
    pub fn new() -> Self {
        let token = Token::default();
        Self {
            cache: HashMap::new(),
            token: token.clone(),
            api: TaiglaApi::new("http://localhost:1234/", token)
        }
    }

    pub fn token(&self) -> (TaiglaData, Token) {
        (TaiglaData::Token, self.token.clone())
    }
}

impl Store for TaiglaStore {
    type Event = TaiglaEvent;

    fn handle(&mut self, event: Self::Event) -> Effect<Self> {
        match event {
            TaiglaEvent::ApiEvent(e) => return e.reduce(self),
            TaiglaEvent::SetToken(token) => self.token.set(&token)
        }
        Effect::NONE
    }
}
