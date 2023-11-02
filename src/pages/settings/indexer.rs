#![allow(non_snake_case)]
use dioxus::prelude::*;

pub fn Form(cx: Scope) -> Element {
    render! {
        form {
            input {}
        }
    }
}

#[inline_props]
pub fn Indexer(cx: Scope, id: String) -> Element {
    render! {
        "indexer"
        Form {}
    }
}
