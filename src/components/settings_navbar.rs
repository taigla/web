#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::routes::Routes;

static LINKS: &'static [(&str, Routes)] = &[
    ("User", Routes::Users {}),
    ("Invite", Routes::Invites {}),
    ("Indexer", Routes::Indexers {}),
    ("Request profile", Routes::RequestProfiles {}),
    ("Background jobs", Routes::BackgroundJobs {})
];

pub fn SettingsNavbar(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-row",
            LinkList {}
            div {
                class: "w-full overflow-y-scroll h-screen pt-20 pb-2 px-24",
                Outlet::<Routes> {}
            }
        }
    }
}

fn LinkList(cx: Scope) -> Element {
    let route = use_route::<Routes>(&cx).unwrap();
    let links = LINKS.iter().cloned().map(|(name, link)| {
        let active = if link.is_child_of(&route) { "text-primary active" } else { "" };
        return rsx! {
            li {
                Link { key: "{name}", class: "{active}", to: link, name }
            }
        };
    });

    render! {
        ul {
            class: "menu menu-vertical pt-20 min-w-max",
            links
        }
    }
}
