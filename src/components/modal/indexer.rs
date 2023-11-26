#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::{use_read, use_set, Atom};
use serde_json::{json, Value};
use serde::Deserialize;
use super::ModalWithTitle;
use crate::hooks::{use_taigla_api, use_query};
use crate::services::settings::SettingCommand;
use crate::api::{IndexerRow, QueryState};
use crate::components::ui::Input;

pub static STATE: Atom<IndexerModalState> = Atom(|_| IndexerModalState::Close);

#[derive(PartialEq)]
pub enum IndexerModalState {
    New,
    Id(u64),
    Close
}

#[derive(Deserialize)]
struct Indexer {
    name: String,
    url: String,
    api_key: Option<String>,
    priority: u8
}

#[inline_props]
fn Form<'a>(cx: Scope, indexer: Option<&'a Indexer>, on_update: EventHandler<'a, Value>, on_delete: EventHandler<'a, ()>) -> Element<'a> {
    let priority = if let Some(indexer) = cx.props.indexer { indexer.priority.to_string() } else { "".to_string() };
    let set_state = use_set(cx, &STATE);

    let submit = move |evt: Event<FormData>| {
        log::info!("{:?}", evt);
        let body = json!({
            "name": evt.data.values.get("name").unwrap().get(0).unwrap(),
            "url": evt.data.values.get("url").unwrap().get(0).unwrap(),
            "api_key": evt.data.values.get("api_key").unwrap().get(0).unwrap(),
            "priority": evt.data.values.get("priority").unwrap().get(0).unwrap().parse::<u8>().unwrap()
        });
        on_update.call(body);
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
                if indexer.is_some() {
                    rsx! {
                        p {
                            class: "btn btn-error",
                            onclick: move |_| on_delete.call(()),
                            "Delete"
                        }
                    }
                }
                p {
                    class: "btn",
                    onclick: move |_| set_state(IndexerModalState::Close),
                    "Close"
                }
                input {
                    class: "btn btn-primary",
                    r#type: "submit",
                    value: "Save"
                }
            }
        }
    }
}

#[inline_props]
fn ModalEditIndexer<'a>(cx: Scope, id: &'a u64) -> Element {
    let api = use_taigla_api(&cx);
    let indexer = use_query::<Indexer>(cx, &format!("/api/v1/indexers/{}", id));
    let set_state = use_set(cx, &STATE);
    let id = **id;
    let setting_handle = use_coroutine_handle::<SettingCommand>(cx).unwrap();

    let edit = move |v| {
        to_owned![api, id, set_state, setting_handle];
        cx.spawn(async move {
            let indexer = api.read().patch_indexer(id, v)
                .await;
            if let Ok(indexer) = indexer {
                set_state(IndexerModalState::Close);
                setting_handle.send(SettingCommand::UpdateIndexer(IndexerRow {
                    id: indexer.id,
                    name: indexer.name,
                    priority: indexer.priority
                }));
            }
        });
    };

    let delete = move |_| {
        to_owned![api, id, set_state, setting_handle];
        cx.spawn(async move {
            let result = api.read().delete_indexer(id)
                .await;
            if let Ok(_) = result {
                set_state(IndexerModalState::Close);
                setting_handle.send(SettingCommand::DeleteIndexer(id));
            }
        });
    };

    render! {
        match &indexer {
            QueryState::Ok(i) => rsx! { Form {
                indexer: i,
                on_update: edit,
                on_delete: delete
            } },
            QueryState::Loading => rsx! { "Loading" },
            _ => rsx! { "Error" }
        }
    }
}

#[inline_props]
fn ModalNewIndexer(cx: Scope) -> Element {
    let set_state = use_set(cx, &STATE);
    let api = use_taigla_api(&cx);
    let setting_handle = use_coroutine_handle::<SettingCommand>(cx).unwrap();

    let create = move |v| {
        to_owned![api, set_state, setting_handle];
        cx.spawn(async move {
            let indexer = api.read().post_indexer(v)
                .await;
            if let Ok(indexer) = indexer {
                set_state(IndexerModalState::Close);
                setting_handle.send(SettingCommand::AddIndexer(IndexerRow {
                    id: indexer.id,
                    name: indexer.name,
                    priority: indexer.priority
                }));
            }
        });
    };

    render! {
        Form {
            on_update: create,
            on_delete: move |_| {}
        }
    }
}

#[inline_props]
pub fn Indexer(cx: Scope) -> Element {
    let state = use_read(cx, &STATE);
    let set_state = use_set(cx, &STATE);

    render! {
        ModalWithTitle {
            visible: *state != IndexerModalState::Close,
            on_close: move |_| set_state(IndexerModalState::Close),
            title: "Indexer",
            match state {
                IndexerModalState::Id(id) => rsx! { ModalEditIndexer { id: id } },
                IndexerModalState::New => rsx! { ModalNewIndexer {} },
                _ => rsx! { "" }
            }
        }
    }
}
