#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::components::modal::indexer::Indexer;
use crate::reducers::{use_get_indexers, RequestState, IndexerModalState, TaiglaEvent, TaiglaStore};
use crate::redux::use_dispatcher;
use crate::api::IndexerRow;

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
    let indexers = use_get_indexers(cx);
    let dispatcher = use_dispatcher::<TaiglaStore>(cx);

    render! {
        div {
            class: "flex flex-col w-full",
            div {
                class: "flex flex-row justify-between pb-2",
                p { class: "text-3xl", "Indexers" }
                p {
                    onclick: move |_| dispatcher.dispatch(TaiglaEvent::SetIndexerModalState(IndexerModalState::New)),
                    class: "btn btn-primary", "New"
                }
            }
            match &indexers {
                RequestState::Ok(indexers) | RequestState::Validating(indexers) => rsx! {
                    IndexerList {
                        indexers: &indexers,
                        on_indexer_select: move |id| dispatcher.dispatch(TaiglaEvent::SetIndexerModalState(IndexerModalState::Id(id)))
                    }
                },
                RequestState::Loading => rsx! { "Loading" },
                _ => rsx! { "Error" }
            }
            Indexer {
                on_close: move |_| dispatcher.dispatch(TaiglaEvent::SetIndexerModalState(IndexerModalState::Close))
            }
        }
    }
}
