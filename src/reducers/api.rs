use dioxus::prelude::*;
use std::rc::Rc;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use crate::{redux::{Reducer, use_slice, use_dispatcher, Effect, ReduxDispatcher}, reducers::TaiglaEvent};
use super::{TaiglaStore, TaiglaData};

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct ApiError {
    pub err_code: String,
    pub description: String
}

impl ApiError {
    pub fn new(err_code: &str, description: &str) -> Self {
        Self {
            err_code: err_code.to_string(),
            description: description.to_string()
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InnerRequestState {
    NotFetch,
    Loading,
    Ok(serde_json::Value),
    Err(ApiError)
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
    Ok(T),
    Err(ApiError)
}

impl<T: DeserializeOwned> Into<RequestState<T>> for InnerRequestState {
    fn into(self) -> RequestState<T> {
        match self {
            InnerRequestState::NotFetch => RequestState::NotFetch,
            InnerRequestState::Loading => RequestState::Loading,
            InnerRequestState::Ok(v) => {
                if let Ok(v) = serde_json::from_value::<T>(v) {
                    RequestState::Ok(v)
                } else {
                    RequestState::Err(ApiError::new("SerdeErr", "Failed to parse response"))
                }
            },
            InnerRequestState::Err(e) => RequestState::Err(e)
        }
    }
}

#[derive(Hash, PartialEq, Eq)]
pub enum ApiData {
    Version,
    TrendingMovie,
    Movie(u32),
    Indexers,
    Indexer(u64)
}

pub enum ApiEvent {
    GetVersion,
    GetIndexers,
    GetIndexer(u64),
    UpdateIndexer(crate::api::Indexer),
    AddIndexer(crate::api::IndexerCreate),
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
            ApiEvent::GetVersion => return Effect::future(move |dispatcher: ReduxDispatcher<TaiglaStore>| {
                Box::pin(async move {
                    let response = match api.get_json("/api/v1/version").await {
                        Ok(v) => InnerRequestState::Ok(v),
                        Err(_) => panic!("Failed to get request")
                    };
                    dispatcher.dispatch(ApiEvent::CacheData(ApiData::Version, response))
                })
            }),
            ApiEvent::GetIndexers => return Effect::future(move |dispatcher: ReduxDispatcher<TaiglaStore>| {
                Box::pin(async move {
                    dispatcher.dispatch(ApiEvent::CacheData(ApiData::Indexers, InnerRequestState::Loading));
                    let response = match api.get_json("/api/v1/indexers").await {
                        Ok(v) => InnerRequestState::Ok(v),
                        Err(_) => InnerRequestState::Err(ApiError::new("NetworkError", "Network error"))
                    };
                    dispatcher.dispatch(ApiEvent::CacheData(ApiData::Indexers, response))
                })
            }),
            ApiEvent::GetIndexer(id) => return Effect::future(move |dispatcher: ReduxDispatcher<TaiglaStore>| {
                Box::pin(async move {
                    dispatcher.dispatch(ApiEvent::CacheData(ApiData::Indexer(id), InnerRequestState::Loading));
                    let response = match api.get_json(&format!("/api/v1/indexers/{id}")).await {
                        Ok(v) => InnerRequestState::Ok(v),
                        Err(_) => panic!("Failed to get request")
                    };
                    dispatcher.dispatch(ApiEvent::CacheData(ApiData::Indexer(id), response))
                })
            }),
            ApiEvent::UpdateIndexer(indexer) => return Effect::future(move |dispatcher: ReduxDispatcher<TaiglaStore>| {
                Box::pin(async move {
                    let _ = match api.patch_json(&format!("/api/v1/indexers/{}", indexer.id), &indexer).await {
                        Ok(v) => InnerRequestState::Ok(v),
                        Err(_) => panic!("Failed to get request")
                    };
                    dispatcher.dispatch(ApiEvent::GetIndexer(indexer.id));
                    dispatcher.dispatch(ApiEvent::GetIndexers)
                })
            }),
            ApiEvent::AddIndexer(indexer) => return Effect::future(move |dispatcher: ReduxDispatcher<TaiglaStore>| {
                Box::pin(async move {
                    let _ = match api.post_json(&format!("/api/v1/indexers"), &indexer).await {
                        Ok(v) => InnerRequestState::Ok(v),
                        Err(_) => panic!("Failed to get request")
                    };
                    dispatcher.dispatch(ApiEvent::GetIndexers);
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

    fn indexers(&self) -> (TaiglaData, InnerRequestState) {
        let value = self.cache.get(&ApiData::Indexers)
            .unwrap_or(&InnerRequestState::NotFetch);
        (TaiglaData::Api(ApiData::Indexers), value.clone())
    }

    fn indexer(id: u64) -> impl Fn(&Self) -> (TaiglaData, InnerRequestState) {
        move |store| {
            let value = store.cache.get(&ApiData::Indexer(id))
                .unwrap_or(&InnerRequestState::NotFetch);
            (TaiglaData::Api(ApiData::Indexer(id)), value.clone())
        }
    }
}

pub fn use_get_version() -> Signal<RequestState<crate::api::Version>> {
    let slice = use_slice(TaiglaStore::get_api_version);
    let value = use_signal(|| Into::<RequestState<crate::api::Version>>::into(*slice.read().borrow().clone()));
    // let dispatcher = use_dispatcher::<TaiglaStore>();

    // use_effect(|| {
    //     let new_value = slice.read().borrow().as_ref();
    //     if new_value.should_refetch() {
    //         dispatcher.dispatch(TaiglaEvent::ApiEvent(ApiEvent::GetVersion));
    //     }
    //     value.set(new_value.into());
    // });

    value
}

// pub fn use_get_indexers() -> RequestState<Vec<crate::api::IndexerRow>> {
//     let slice = use_slice(TaiglaStore::indexers);
//     let value = use_signal(|| Into::<RequestState<Vec<crate::api::IndexerRow>>>::into(*slice.read().borrow().clone()));
//     let dispatcher = use_dispatcher::<TaiglaStore>();

//     use_effect(|| {
//         if new_value.should_refetch() {
//             dispatcher.dispatch(TaiglaEvent::ApiEvent(ApiEvent::GetIndexers));
//         }
//         value.set(new_value.into());
//     });

//     &value
// }

// pub fn use_get_indexer(id: u64) -> RequestState<crate::api::Indexer> {
//     let slice = use_slice(TaiglaStore::indexer(id));
//     let value = use_state(cx, || Into::<RequestState<crate::api::Indexer>>::into(*slice.read().borrow().clone()));
//     let dispatcher = use_dispatcher::<TaiglaStore>();

//     use_effect(cx, (slice.read().borrow().as_ref(),), |(new_value,)| {
//         if new_value.should_refetch() {
//             dispatcher.dispatch(TaiglaEvent::ApiEvent(ApiEvent::GetIndexer(id)));
//         }
//         value.set(new_value.into());
//         async move {}
//     });

//     &value
// }

pub fn use_update_indexer_mutation() -> Rc<impl Fn(crate::api::Indexer) -> ()> {
    let dispatcher = use_dispatcher::<TaiglaStore>();

    use_hook(|| {
        to_owned![dispatcher];
        Rc::new(move |indexer: crate::api::Indexer| {
            dispatcher.dispatch(ApiEvent::UpdateIndexer(indexer))
        })
    })
}

pub fn use_add_indexer_mutation() -> Rc<impl Fn(crate::api::IndexerCreate) -> ()> {
    let dispatcher = use_dispatcher::<TaiglaStore>();

    use_hook(|| {
        to_owned![dispatcher];
        Rc::new(move |indexer: crate::api::IndexerCreate| {
            dispatcher.dispatch(ApiEvent::AddIndexer(indexer))
        })
    })
}

