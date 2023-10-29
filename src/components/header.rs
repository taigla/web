#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::routes::Routes;

pub fn Header(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-column justify-between",
            Link { to: Routes::Home {}, class: "w-30", "Taigla" }
            div {
                class: "flex flex-column",
                Link { to: Routes::Home {}, class: "", "Home" }
                Link { to: Routes::Settings {}, class: "", "Settings" }
            }
            p { class: "w-30", "Taigla" }
        }
    }
}
