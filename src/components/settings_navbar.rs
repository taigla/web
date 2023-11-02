#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::routes::Routes;

static LINKS: &'static [(&str, Routes)] = &[
    ("User", Routes::Users {}),
    ("Invite", Routes::Invites {}),
    ("Indexer", Routes::Indexers {}),
    ("Request profile", Routes::RequestProfiles {})
];

pub fn SettingsNavbar(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-row",
            LinkList {}
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
            class: "flex flex-col w-20",
            links
        }
    }
}
