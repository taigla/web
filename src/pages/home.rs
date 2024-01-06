#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::redux::{use_slice, use_dispatcher};
use crate::reducers::{TaiglaStore, ApiEvent, use_get_version};

#[component]
pub fn Home(cx: Scope) -> Element {
    let dispatcher = use_dispatcher::<TaiglaStore>(cx);
    let version = use_get_version(cx);

    let onclick = move |_| dispatcher.dispatch(ApiEvent::GetVersion.into());
    render! {
        "Home"
        button {
            onclick: onclick,
            "New Task"
        }
    }
}
