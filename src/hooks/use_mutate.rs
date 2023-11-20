use dioxus::prelude::*;
use serde::Serialize;
use super::use_query_provider::{UseQueryProvider, use_query_provider, QueryValue, RegistryEntry};

#[derive(Clone)]
pub struct UseMutate {
    query_provider: UseQueryProvider,
    entry: RegistryEntry
}

impl UseMutate {
    pub fn mutate<T: Serialize>(&self, new_value: T) {
        let value = serde_json::to_value(new_value).unwrap();
        self.query_provider.mutate(&self.entry, QueryValue::Ok(value));
    }

    pub fn mutate_silent<T: Serialize>(&self, new_value: T) {
        let value = serde_json::to_value(new_value).unwrap();
        self.query_provider.mutate_silent(&self.entry, QueryValue::Ok(value));
    }
}

pub fn use_mutate<'a>(cx: &'a ScopeState, url: &str) -> &'a UseMutate {
    let provider = use_query_provider(cx);
    cx.use_hook(|| {
        let entry = provider.get_registry_entry(url);
        UseMutate {
            query_provider: provider,
            entry: entry
        }
    })
}
