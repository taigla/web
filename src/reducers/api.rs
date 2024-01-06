use dioxus::prelude::*;
use crate::{redux::{Reducer, use_slice, use_dispatcher, ReduxSlice, Effect, ReduxDispatcher}, reducers::TaiglaEvent};
use super::{TaiglaStore, TaiglaData};

#[derive(Clone, PartialEq, Eq)]
pub enum RequestState {
    NotFetch,
    Loading
}

impl RequestState {
    fn should_refetch(&self) -> bool {
        matches!(self, RequestState::NotFetch)
    }
}

#[derive(Hash, PartialEq, Eq)]
pub enum ApiData {
    Version,
    TrendingMovie,
    Movie(u32)
}

pub enum ApiEvent {
    GetVersion,
    CacheData(ApiData, RequestState)
}

impl Into<super::TaiglaEvent> for ApiEvent {
    fn into(self) -> super::TaiglaEvent {
        super::TaiglaEvent::ApiEvent(self)
    }
}

impl Reducer<TaiglaStore> for ApiEvent {
    fn reduce(self, store: &mut TaiglaStore) -> Effect<TaiglaStore> {
        let token = store.token.clone();
        match self {
            ApiEvent::GetVersion => return Effect::future(|dispatcher: ReduxDispatcher<TaiglaStore>| {
                Box::pin(async move {
                   log::info!("{:?}", token);
                    // let token = store.token.clone();
                    // log::info!("{:?}", token);
                })
            }),
            ApiEvent::CacheData(key, data) => (),
        }
        Effect::NONE
    }
}

impl TaiglaStore {
    fn get_api_version(&self) -> (TaiglaData, RequestState) {
        let value = self.cache.get(&ApiData::Version)
            .unwrap_or(&RequestState::NotFetch);
        (TaiglaData::Api(ApiData::Version), value.clone())
    }
}

pub fn use_get_version(cx: &ScopeState) -> &ReduxSlice<RequestState> {
    let value = use_slice(cx, TaiglaStore::get_api_version);
    let dispatcher = use_dispatcher::<TaiglaStore>(cx);

    use_effect(cx, (value.read().borrow().as_ref(),), |(value,)| {
        if value.should_refetch() {
            dispatcher.dispatch(TaiglaEvent::ApiEvent(ApiEvent::GetVersion));
        }
        async move {}
    });

    value
}
