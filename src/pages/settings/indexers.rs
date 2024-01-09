#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::prelude::*;
use fermi::use_set;
use crate::components::modal::indexer::{Indexer, IndexerModalState, STATE};
use crate::reducers::{use_get_indexers, RequestState};
use crate::services::settings::{INDEXER_LIST_STORE, SettingCommand};
use crate::api::{QueryState, IndexerRow};

#[component]
pub fn IndexerList<'a>(cx: Scope, indexers: &'a Vec<IndexerRow>, on_indexer_select: EventHandler<'a, u64>) -> Element {
    let rows = indexers.iter().map(|indexer| {
        rsx! {
            tr {
                class: "hover",
                key: "{indexer.id}",
                td { "{indexer.name}" }
                td { "{indexer.priority}" }
                td {
                    style: "padding-top: 0; padding-bottom: 0;",
                    button {
                        class: "btn btn-sm",
                        onclick: move |_| on_indexer_select.call(indexer.id),
                        "Edit"
                    }
                }
            }
        }
    });

    render! {
        table {
            class: "table bordered",
            thead {
                tr {
                    th { class: "w-10/12", "Name" }
                    th { class: "w-1/12", "Priority" }
                    th { class: "w-1/12", "" }
                }
            }
            tbody {
                rows
            }
        }
    }
}

pub fn Indexers(cx: Scope) -> Element {
    let set_modal_state = use_set(cx, &STATE);
    let indexers = use_get_indexers(cx);
    let selected_indexer = use_state::<IndexerModalState>(cx, || IndexerModalState::Close);

    render! {
        div {
            class: "flex flex-col w-full",
            div {
                class: "flex flex-row justify-between pb-2",
                p { class: "text-3xl", "Indexers" }
                p {
                    onclick: move |_| selected_indexer.set(IndexerModalState::New),
                    class: "btn btn-primary", "New"
                }
            }
            match &indexers {
                RequestState::Ok(indexers) => rsx! {
                    IndexerList {
                        indexers: &indexers,
                        on_indexer_select: move |id| selected_indexer.set(IndexerModalState::Id(id))
                    }
                },
                _ => rsx! { "Error" }
            }
            Indexer {
                state: selected_indexer,
                on_close: move |_| selected_indexer.set(IndexerModalState::Close)
            }
        }
    }
}
