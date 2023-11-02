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
                class: "w-full overflow-y-scroll h-screen pt-16",
                Outlet::<Routes> {}
            }
        }
    }
}

fn LinkList(cx: Scope) -> Element {
    let route = use_route::<Routes>(&cx).unwrap();
    let links = LINKS.iter().cloned().map(|(name, link)| {
        let active = if link.is_child_of(&route) { "text-accent" } else { "" };
        return rsx! {
            Link { key: "{name}", class: "py-1 px-2 rounded-md hover:bg-gray-500 {active}", to: link, name }
        };
    });

    render! {
        div {
            class: "flex flex-col p-2 pt-16 min-w-max",
            links
        }
    }
}
