#![allow(non_snake_case)]
use dioxus::prelude::*;
use serde::de::DeserializeOwned;
use super::use_query_provider::{UseQueryProvider, use_query_provider, QueryValue, RegistryEntry, ValueHash};

#[derive(Debug)]
pub enum QueryState<T> {
    Loading,
    Ok(T),
    Validating(T),
    // Network related error
    Error,
    // Error from the api, the server didn't response with an OK HTTP code
    UserError
}

#[derive(Debug)]
pub struct UseQuery<T: DeserializeOwned> {
    pub value: QueryState<T>,
    pub mutate: u32,
    value_hash: ValueHash,
    registry_entry: RegistryEntry
}

impl<T: DeserializeOwned> UseQuery<T> {
    pub fn new(provider: &UseQueryProvider, url: &str, scope_id: ScopeId) -> Self {
        let registry_entry = provider.add_listener(url, scope_id);
        let readable_entry = registry_entry
            .read()
            .unwrap();
        let query_value = readable_entry
            .value
            .clone();
        let hash = readable_entry.hash;
        drop(readable_entry);

        let value = match query_value {
            QueryValue::Ok(v) => QueryState::Ok(serde_json::from_value::<T>(v).unwrap()),
            QueryValue::Validating(v) => QueryState::Validating(serde_json::from_value::<T>(v).unwrap()),
            QueryValue::Error => QueryState::Error,
            QueryValue::Loading => QueryState::Loading
        };
        Self {
            value: value,
            mutate: 0,
            value_hash: hash,
            registry_entry: registry_entry
        }
    }

    fn check_update(&mut self) {
        let readable_entry = self.registry_entry
            .read()
            .unwrap();
        if self.value_hash != readable_entry.hash {
            let query_value = readable_entry
                .value
                .clone();

            self.value = match query_value {
                QueryValue::Ok(v) => QueryState::Ok(serde_json::from_value::<T>(v).unwrap()),
                QueryValue::Validating(v) => QueryState::Validating(serde_json::from_value::<T>(v).unwrap()),
                QueryValue::Error => QueryState::Error,
                QueryValue::Loading => QueryState::Loading
            };
            self.value_hash = readable_entry.hash;
        }
    }
}

pub fn use_query<'a, T: DeserializeOwned + 'static>(cx: &'a ScopeState, url: &str) -> &'a UseQuery<T> {
    let provider = use_query_provider(&cx);

    let hook = cx.use_hook(|| {
        log::info!("New use query");
        let query = UseQuery::<T>::new(&provider, url, cx.scope_id());

        cx.spawn({
            to_owned![url, provider];
            async move {
                provider.update(&url).await;
            }
        });
        query
    });
    hook.check_update();
    hook
}
