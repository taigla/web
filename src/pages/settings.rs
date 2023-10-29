#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::components::Header;

pub fn Settings(cx: Scope) -> Element {
    render! {
        Header {}
        "settings"
    }
}
