#![allow(non_snake_case)]
use dioxus::prelude::*;
use serde::{de::DeserializeOwned, Serialize};
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

pub struct UseQuery<T: DeserializeOwned> {
    pub value: QueryState<T>,
    value_hash: ValueHash,
    registry_entry: RegistryEntry,
    query_provider: UseQueryProvider,
    scope_id: ScopeId
}

impl<T: DeserializeOwned> UseQuery<T> {
    fn check_update(&mut self) {
        let readable_entry = self.registry_entry
            .read()
            .unwrap();
        if self.value_hash != readable_entry.hash {
            log::info!("Update data");
            let query_value = readable_entry
                .value
                .clone();

            self.value = match query_value {
                QueryValue::Ok(v) => QueryState::Ok(serde_json::from_value::<T>(v).unwrap()),
                QueryValue::Validating(v) => QueryState::Validating(serde_json::from_value::<T>(v).unwrap()),
                QueryValue::Error => QueryState::Error,
                QueryValue::Loading => QueryState::Loading,
                QueryValue::NotFetch => QueryState::Loading
            };
            self.value_hash = readable_entry.hash;
        }
    }

    pub fn mutate<U: Serialize>(&self, new_value: U) {
        self.query_provider.mutate(&self.registry_entry, serde_json::to_value(new_value).unwrap());
    }
}

impl<T: DeserializeOwned> Drop for UseQuery<T> {
    fn drop(&mut self) {
        self.query_provider.remove_listener(&self.registry_entry, self.scope_id);
    }
}

pub fn use_query<'a, T: DeserializeOwned + 'static>(cx: &'a ScopeState, url: &str) -> &'a UseQuery<T> {
    let provider = use_query_provider(&cx);

    let hook = cx.use_hook(|| {
        let registry_entry = provider.add_listener(url, cx.scope_id());
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
            QueryValue::Loading => QueryState::Loading,
            QueryValue::NotFetch => {
                let mut writable_entry = registry_entry
                    .write()
                    .unwrap();
                writable_entry.value = QueryValue::Loading;
                cx.spawn({
                    to_owned![url, provider];
                    async move {
                        provider.update(&url).await;
                    }
                });
                QueryState::Loading
            }
        };

        let query = UseQuery::<T> {
            value: value,
            value_hash: hash,
            registry_entry: registry_entry,
            query_provider: provider.clone(),
            scope_id: cx.scope_id()
        };
        query
    });
    hook.check_update();
    hook
}
