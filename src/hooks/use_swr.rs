use dioxus::prelude::*;
use serde::de::DeserializeOwned;
use crate::hooks::use_taigla_api;

pub enum Error {
    EmptyUrl
}

pub enum State<T> {
    Loading,
    Ok(T),
    Error(Error)
}

pub fn use_swr<'a, T: 'static + DeserializeOwned>(cx: &'a ScopeState, url: &str) -> &'a State<T> {
    let data = use_state::<State<T>>(cx, || State::Loading);
    let taigla_api = use_taigla_api(cx);
    let url = url.to_string();

    use_effect(cx, (&url,), |(url,)| {
        to_owned![taigla_api, data];
        async move {
            if url.len() == 0 {
                data.set(State::Error(Error::EmptyUrl));
                return;
            }
            let response = taigla_api.read().get(&url)
                .send()
                .await
                .unwrap()
                .json::<T>()
                .await
                .expect("Failed to parse user list");
            data.set(State::Ok(response));
        }
    });

    data.get()
}
