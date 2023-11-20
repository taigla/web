#![allow(non_snake_case)]
use dioxus::prelude::*;
use super::Modal;

#[inline_props]
fn Input<'a>(cx: Scope, lbl: Option<&'a str>) -> Element {
    render! {
        Fragment {
            if let Some(lbl) = lbl {
                rsx! { label { class: "col-span-12 md:col-span-3", "{lbl}" } }
            }
            rsx! { input { class: "input col-span-12 md:col-span-9" } }
        }
    }
}

pub fn Form<'a>(cx: Scope, close: EventHandler<'a, ()>) -> Element<'a> {
    render! {
        form {
            class: "grid grid-cols-12 gap-y-5",
            Input { lbl: "Name:" }
            Input { lbl: "Url:" }
            Input { lbl: "Api key:" }
            Input { lbl: "Priority:" }
            div {
                class: "flex flex-row justify-end col-span-12 gap-2",
                p {
                    class: "btn solid md",
                    onclick: move |_| {log::info!("press")},
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
pub fn Indexer<'a>(cx: Scope, id: String, visible: &'a bool, close: EventHandler<'a, ()>) -> Element<'a> {
    render! {
        Modal {
            visible: **visible,
            div {
                class: "flex flex-col p-6",
                p { class: "text-2xl", "Indexer" }
                // Form {
                //     close: 
                // }
            }
        }
    }
}
