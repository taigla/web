use dioxus::prelude::*;
use serde::de::DeserializeOwned;
use crate::api::QueryState;
use super::use_taigla_api;

// Make a simple get request and return the result
pub fn use_query<'a, T: DeserializeOwned + 'static>(cx: &'a ScopeState, url: &str) -> &'a QueryState<T> {
    let api = use_taigla_api(cx);
    let data = use_state(cx, || QueryState::<T>::Loading);
    let url = url.to_string();

    use_effect(cx, (&url,), |_| {
        to_owned![api, data, url];
        async move {
            let response = api.read().get::<T>(&url).await;
            match response {
                Ok(k) => data.set(QueryState::Ok(k)),
                Err(e) => data.set(QueryState::Err(e))
            }
        }
    });
    data.get()
}
