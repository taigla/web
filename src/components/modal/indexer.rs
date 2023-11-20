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

pub fn Form(cx: Scope) -> Element {
    render! {
        form {
            class: "grid grid-cols-12 gap-y-5",
            Input { lbl: "Name:" }
            Input { lbl: "Url:" }
            Input { lbl: "Api key:" }
            Input { lbl: "Priority:" }
        }
    }
}

#[inline_props]
pub fn Indexer(cx: Scope, id: String) -> Element {
    render! {
        Modal {
            visible: true,
            div {
                class: "flex flex-col p-6",
                p { class: "text-2xl", "Indexer" }
                Form {}
            }
        }
    }
}
