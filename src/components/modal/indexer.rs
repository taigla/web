#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::{use_read, use_set, Atom};
use serde_json::json;
use serde::Deserialize;
use super::Modal;
use crate::hooks::{use_taigla_api, use_swr::{use_swr, State, Error as SwrError}};

pub static STATE: Atom<IndexerModalState> = Atom(|_| IndexerModalState::Close);

#[derive(PartialEq)]
pub enum IndexerModalState {
    New,
    Id(u64),
    Close
}

#[derive(Deserialize)]
struct Indexer {
    id: u64,
    name: String,
    url: String,
    api_key: Option<String>,
    priority: u8
}

#[inline_props]
fn Input<'a>(cx: Scope, name: &'a str, lbl: Option<&'a str>, default_value: Option<&'a str>) -> Element<'a> {
    render! {
        Fragment {
            if let Some(lbl) = *lbl {
                rsx! { label { class: "col-span-12 md:col-span-3", lbl } }
            }
            rsx! { input { class: "input col-span-12 md:col-span-9", initial_value: *default_value, name: *name } }
        }
    }
}

#[inline_props]
fn Form<'a>(cx: Scope, indexer: Option<&'a Indexer>) -> Element<'a> {
    let api = use_taigla_api(&cx);
    let priority = if let Some(indexer) = cx.props.indexer { indexer.priority.to_string() } else { "".to_string() };
    let id = if let Some(indexer) = indexer { indexer.id }  else { 0 };
    let set_state = use_set(cx, &STATE);

    let submit = move |evt: Event<FormData>| {
        to_owned![api, id, set_state];
        cx.spawn(async move {
            log::info!("{:?}", evt);
            let body = json!({
                "name": evt.data.values.get("name").unwrap().get(0).unwrap(),
                "url": evt.data.values.get("url").unwrap().get(0).unwrap(),
                "api_key": evt.data.values.get("api_key").unwrap().get(0).unwrap(),
                "priority": evt.data.values.get("priority").unwrap().get(0).unwrap().parse::<u8>().unwrap()
            });
            if id == 0 {
                let _ = api.read().post("/api/v1/indexers")
                    .json(&body)
                    .send()
                    .await
                    .unwrap()
                    .json::<Indexer>()
                    .await
                    .unwrap();
            } else {
                let _ = api.read().patch(&format!("/api/v1/indexers/{id}"))
                    .json(&body)
                    .send()
                    .await
                    .unwrap()
                    .json::<Indexer>()
                    .await
                    .unwrap();
            }
            set_state(IndexerModalState::Close);
        })
    };

    render! {
        form {
            class: "grid grid-cols-12 gap-y-5 items-center",
            onsubmit: submit,
            Input {
                lbl: "Name:",
                name: "name",
                default_value: if let Some(indexer) = indexer { &indexer.name } else { "" }
            }
            Input {
                lbl: "Url:",
                name: "url",
                default_value: if let Some(indexer) = indexer { &indexer.url } else { "" }
            }
            Input {
                lbl: "Api key:",
                name: "api_key",
                default_value: if let Some(indexer) = indexer { indexer.api_key.as_ref().map(|v| v.as_str()).unwrap_or("") } else { "" }
            }
            Input {
                lbl: "Priority:",
                name: "priority",
                default_value: "{priority}"
            }
            div {
                class: "flex flex-row justify-end col-span-12 gap-2",
                p {
                    class: "btn solid md",
                    onclick: move |_| set_state(IndexerModalState::Close),
                    "Close"
                }
                input {
                    class: "btn solid primary md",
                    r#type: "submit",
                    value: "Save"
                }
            }
        }
    }
}

#[inline_props]
fn ModalContent<'a>(cx: Scope, state: &'a IndexerModalState) -> Element<'a> {
    let url = if let IndexerModalState::Id(id) = *state { format!("/api/v1/indexers/{}", id) } else { "".to_string() };
    let indexer = use_swr::<Indexer>(cx, &url);

    render! {
        div {
            class: "flex flex-col p-6",
            p { class: "text-2xl", "Indexer" }
            match indexer {
                State::Ok(i) => rsx! { Form { indexer: i } },
                State::Loading => rsx! { div { class: "loader bw sm", div { class: "spin" } } },
                State::Error(SwrError::EmptyUrl) => rsx! { Form { } },
                State::Error(_) => rsx! { "Error while fetching indexer" }
            }
        }
    }
}

#[inline_props]
pub fn Indexer(cx: Scope) -> Element {
    let state = use_read(cx, &STATE);
    // let write

    render! {
        Modal {
            visible: *state != IndexerModalState::Close,
            ModalContent {
                state: state
            }
        }
    }
}
