#![allow(non_snake_case)]
use dioxus::prelude::*;
use serde::Deserialize;
use fermi::use_set;
use crate::hooks::{use_query, QueryState};
use crate::states::ApiError;
use crate::components::modal::indexer::{Indexer, IndexerModalState, STATE};

#[derive(Deserialize, Debug)]
pub struct Indexer {
    id: u64,
    name: String,
    priority: u8
}

#[inline_props]
pub fn IndexerList<'a>(cx: Scope, indexers: &'a Vec<Indexer>, on_indexer_select: EventHandler<'a, u64>) -> Element {
    let rows = indexers.iter().map(|indexer| {
        rsx! {
            tr {
                key: "{indexer.id}",
                td { "{indexer.name}" }
                td { "{indexer.priority}" }
                td {
                    button {
                        class: "btn sm",
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
    let set_modal_status = use_set(cx, &STATE);
    let query = use_query::<Vec<Indexer>, ApiError>(cx, "/api/v1/indexers");

    render! {
        div {
            class: "flex flex-col w-full",
            div {
                class: "flex flex-row justify-between pb-2",
                p { class: "text-2xl", "Indexers" }
                p {
                    onclick: move |_| set_modal_status(IndexerModalState::New),
                    class: "btn solid sm primary", "New"
                }
            }
            match &query.value {
                QueryState::Ok(indexers) => rsx! {
                    IndexerList {
                        indexers: indexers,
                        on_indexer_select: move |id| set_modal_status(IndexerModalState::Id(id))
                    }
                },
                QueryState::Loading => rsx! { "Loading" },
                _ => rsx! { "Error" }
            }
            Indexer {}
        }
    }
}
