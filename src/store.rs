use crate::reducers::auth::{AuthSlice, AuthAction};
use crate::redux::{Reducer, Store};

pub enum StoreEvent {
    AuthAction(AuthAction),
}

impl Into<StoreEvent> for AuthAction {
    fn into(self) -> StoreEvent {
        StoreEvent::AuthAction(self)
    }
}

pub struct TaiglaStore {
    // #[redux(EnumEventAuthSlice)]
    // Use to auto generate the match statement for this value adn the enum of store event
    pub auth: AuthSlice
}

impl TaiglaStore {
    pub fn new() -> Self {
        Self {
            auth: AuthSlice::new()
        }
    }
}

impl Store for TaiglaStore {
    type Event = StoreEvent;

    fn handle(&mut self, event: StoreEvent) {
        match event {
            StoreEvent::AuthAction(e) => e.reduce(&mut self.auth)
        }
    }
}
