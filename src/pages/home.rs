#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::query::use_get_user;

#[component]
pub fn Home() -> Element {
    let mut counter = use_signal(|| 0);
    let user = use_get_user(1);
    let user2 = use_get_user(1);
    tracing::info!("value: {:?}", user.read().unwrap());

    let onclick = move |_| {
        counter += 1;
    };

    rsx! {
        "Homee"
        "{counter}"
        button {
            class: "btn",
            onclick: onclick,
            "New Task"
        }
    }
}
