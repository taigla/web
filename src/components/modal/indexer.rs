#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::{use_set, Atom};
use serde::Deserialize;
use super::ModalWithTitle;
use crate::hooks::use_taigla_api;
use crate::reducers::{use_get_indexer, RequestState, use_add_indexer_mutation, use_update_indexer_mutation, IndexerModalState, TaiglaStore, TaiglaEvent};
use crate::redux::{use_slice, use_dispatcher};
use crate::services::settings::SettingCommand;
use crate::api::{Indexer, IndexerCreate};
use crate::components::ui::Input;

pub static STATE: Atom<IndexerModalState> = Atom(|_| IndexerModalState::Close);

#[derive(Deserialize)]
struct IndexerForm {
    name: String,
    url: String,
    api_key: String,
    priority: u8
}

#[component]
fn Form<'a>(cx: Scope, indexer: Option<&'a Indexer>, on_update: EventHandler<'a, IndexerForm>, on_delete: EventHandler<'a, ()>) -> Element<'a> {
    let priority = if let Some(indexer) = cx.props.indexer { indexer.priority.to_string() } else { "".to_string() };

    let submit = move |evt: Event<FormData>| {
        tracing::info!("{:?}", evt);
        let body = IndexerForm {
            name: evt.data.values.get("name").unwrap().get(0).unwrap().to_string(),
            url: evt.data.values.get("url").unwrap().get(0).unwrap().to_string(),
            api_key: evt.data.values.get("api_key").unwrap().get(0).unwrap().to_string(),
            priority: evt.data.values.get("priority").unwrap().get(0).unwrap().parse::<u8>().unwrap()
        };
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
                input {
                    class: "btn btn-primary",
                    r#type: "submit",
                    value: "Save"
                }
            }
        }
    }
}

#[component]
fn ModalEditIndexer(cx: Scope, id: u64) -> Element {
    let api = use_taigla_api(&cx);
    let indexer = use_get_indexer(cx, *id);
    let update_indexer = use_update_indexer_mutation(cx);
    let set_state = use_set(cx, &STATE);
    let setting_handle = use_coroutine_handle::<SettingCommand>(cx).unwrap();
    let dispatcher = use_dispatcher::<TaiglaStore>(cx);

    let edit = move |v: IndexerForm| {
        to_owned![id, update_indexer, dispatcher];
        cx.spawn(async move {
            let indexer = Indexer {
                name: v.name,
                id: id,
                url: v.url,
                api_key: Some(v.api_key),
                priority: v.priority
            };
            update_indexer(indexer);
            dispatcher.dispatch(TaiglaEvent::SetIndexerModalState(IndexerModalState::Close));
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
            RequestState::Ok(i) => rsx! { Form {
                indexer: i,
                on_update: edit,
                on_delete: delete
            } },
            RequestState::Loading => rsx! { "Loading" },
            _ => rsx! { "Error" }
        }
    }
}

#[component]
fn ModalNewIndexer(cx: Scope) -> Element {
    let add_indexer = use_add_indexer_mutation(cx);
    let dispatcher = use_dispatcher::<TaiglaStore>(cx);

    let create = move |v: IndexerForm| {
        to_owned![add_indexer, dispatcher];
        cx.spawn(async move {
            let indexer = IndexerCreate {
                name: v.name,
                url: v.url,
                api_key: v.api_key,
                priority: v.priority
            };
            add_indexer(indexer);
            dispatcher.dispatch(TaiglaEvent::SetIndexerModalState(IndexerModalState::Close));
        });
    };

    render! {
        Form {
            on_update: create,
            on_delete: move |_| {}
        }
    }
}

#[component]
pub fn Indexer<'a>(cx: Scope, on_close: EventHandler<'a, ()>) -> Element {
    let state = use_slice(cx, TaiglaStore::indexer_modal_state);

    render! {
        ModalWithTitle {
            visible: **state.read().borrow() != IndexerModalState::Close,
            on_close: move |_| on_close.call(()),
            title: "Indexer",
            match **state.read().borrow() {
                IndexerModalState::Id(id) => rsx! { ModalEditIndexer { id: id } },
                IndexerModalState::New => rsx! { ModalNewIndexer {} },
                _ => rsx! { "" }
            }
        }
    }
}
