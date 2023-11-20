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
        Fragment {
            div {
                class: "p-2.5 fixed top-0 w-full",
                div {
                    class: "flex flex-row justify-between items-center",
                    Link { to: Routes::Home {}, class: "w-40 flex flex-row items-center",
                        img { class: "mr-1 rounded-none", src: "/favicon-32.png" }
                        p { class:"font-semibold", "Taigla" }
                    }
                    LinkList {}
                    p { class: "w-40", "Profile" }
                }
            }
            Outlet::<Routes> {}
        }
    }
}

fn LinkList(cx: Scope) -> Element {
    let route = use_route::<Routes>(&cx).unwrap();
    let links = LINKS.iter().cloned().map(|(name, link)| {
        let active = match &route {
            Routes::Home { .. } => match link {
                Routes::Home { .. } => "text-primary",
                _ => ""
            },
            _ => match &link {
                Routes::Home { .. } => "",
                l => if l.is_child_of(&route) { "text-primary" } else { "" }
            }
        };

        return rsx! {
            Link { key: "{name}", class: "px-2 {active}", to: link, name }
        };
    });

    render! {
        div {
            class: "flex flex-row",
            links
        }
    }
}
