#![allow(non_snake_case)]
use dioxus::prelude::*;
use serde::de::DeserializeOwned;
use super::use_query_provider::{UseQueryProvider, use_query_provider};

enum QueryState<T> {
    Loading,
    Ok(T),
    Validating(T),
    Err,
}

pub struct UseQuery<T: DeserializeOwned> {
    value: QueryState<T>,
    mutate: u32,
    provider: UseQueryProvider
}

impl<T: DeserializeOwned> UseQuery<T> {
    pub fn new(provider: UseQueryProvider, url: &str, scope_id: ScopeId) -> Self {
        provider.add_listener(url, scope_id);
        Self {
            value: QueryState::Loading,
            mutate: 0,
            provider
        }
    }
}

pub fn use_query<'a, T: DeserializeOwned + 'static>(cx: &'a ScopeState, url: &str) -> &'a UseQuery<T> {
    let provider = use_query_provider(&cx);
    cx.use_hook(|| {
        log::info!("New use query");
        UseQuery::<T>::new(provider, url, cx.scope_id())
    })
}
