#![allow(non_snake_case)]
use dioxus::prelude::*;
use serde::de::DeserializeOwned;
use super::use_query_provider::{UseQueryProvider, use_query_provider};

enum QueryState<T> {
    Loading,
    Ok(T),
    Validating(T),
    // Network related error
    Error,
    // Error from the api, the server didn't response with an OK HTTP code
    UserError
}

pub struct UseQuery<T: DeserializeOwned> {
    value: QueryState<T>,
    mutate: u32
}

impl<T: DeserializeOwned> UseQuery<T> {
    pub fn new(provider: &UseQueryProvider, url: &str, scope_id: ScopeId) -> Self {
        let value = provider.add_listener(url, scope_id);

        Self {
            value: QueryState::Ok(serde_json::from_value::<T>(value).unwrap()),
            mutate: 0
        }
    }
}

pub fn use_query<'a, T: DeserializeOwned + 'static>(cx: &'a ScopeState, url: &str) -> &'a UseQuery<T> {
    let provider = use_query_provider(&cx);
    cx.use_hook(|| {
        log::info!("New use query");
        let query = UseQuery::<T>::new(&provider, url, cx.scope_id());

        cx.spawn({
            to_owned![url, provider];
            async move {
                provider.update(&url).await;
            }
        });
        query
    })
}
