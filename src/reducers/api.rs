use dioxus::prelude::*;
use serde::de::DeserializeOwned;
use crate::{redux::{Reducer, use_slice, use_dispatcher, ReduxSlice, Effect, ReduxDispatcher}, reducers::TaiglaEvent};
use super::{TaiglaStore, TaiglaData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InnerRequestState {
    NotFetch,
    Loading,
    Ok(serde_json::Value)
}

impl InnerRequestState {
    fn should_refetch(&self) -> bool {
        matches!(self, InnerRequestState::NotFetch)
    }
}

#[derive(Clone, Debug)]
pub enum RequestState<T: DeserializeOwned> {
    NotFetch,
    Loading,
    Ok(T)
}

impl<T: DeserializeOwned> Into<RequestState<T>> for InnerRequestState {
    fn into(self) -> RequestState<T> {
        match self {
            InnerRequestState::NotFetch => RequestState::NotFetch,
            InnerRequestState::Loading => RequestState::Loading,
            InnerRequestState::Ok(v) => {
                RequestState::Ok(serde_json::from_value::<T>(v).unwrap())
            }
        }
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
    CacheData(ApiData, InnerRequestState)
}

impl Into<super::TaiglaEvent> for ApiEvent {
    fn into(self) -> super::TaiglaEvent {
        super::TaiglaEvent::ApiEvent(self)
    }
}

impl Reducer<TaiglaStore> for ApiEvent {
    fn reduce(self, store: &mut TaiglaStore) -> Effect<TaiglaStore> {
        let api = store.api.clone();
        match self {
            ApiEvent::GetVersion => return Effect::future(|dispatcher: ReduxDispatcher<TaiglaStore>| {
                Box::pin(async move {
                    let response = match api.get_json("/api/v1/version").await {
                        Ok(v) => InnerRequestState::Ok(v),
                        Err(_) => panic!("Failed to get request")
                    };
                    dispatcher.dispatch(ApiEvent::CacheData(ApiData::Version, response))
                })
            }),
            ApiEvent::CacheData(key, data) => {
                store.cache.insert(key, data);
            },
        }
        Effect::NONE
    }
}

impl TaiglaStore {
    fn get_api_version(&self) -> (TaiglaData, InnerRequestState) {
        let value = self.cache.get(&ApiData::Version)
            .unwrap_or(&InnerRequestState::NotFetch);
        (TaiglaData::Api(ApiData::Version), value.clone())
    }
}

pub fn use_get_version(cx: &ScopeState) -> RequestState<crate::api::Version> {
    let value = use_slice(cx, TaiglaStore::get_api_version);
    let dispatcher = use_dispatcher::<TaiglaStore>(cx);

    use_effect(cx, (value.read().borrow().as_ref(),), |(value,)| {
        if value.should_refetch() {
            dispatcher.dispatch(TaiglaEvent::ApiEvent(ApiEvent::GetVersion));
        }
        async move {}
    });

    let request_state = *value.read().borrow().clone();
    request_state.into()
}
