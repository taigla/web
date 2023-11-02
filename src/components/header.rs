#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::routes::Routes;

static LINKS: &'static [(&str, Routes)] = &[
    ("Home", Routes::Home {}),
    ("Settings", Routes::Settings {})
];

pub fn Header(cx: Scope) -> Element {
    render! {
        div {
                div {
                class: "flex flex-row justify-between",
                Link { to: Routes::Home {}, class: "w-30", "Taigla" }
                LinkList {}
                p { class: "w-30", "Profile" }
            }
            Outlet::<Routes> {}
        }
    }
}

fn LinkList(cx: Scope) -> Element {
    let links = LINKS.iter().cloned().map(|(name, link)| {
        return rsx! {
            Link { key: "{name}", to: link, name }
        };
    });

    render! {
        div {
            class: "flex flex-row",
            links
        }
    }
}
