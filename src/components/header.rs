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
                class: "fixed top-0 w-full navbar bg-base-100 z-50",
                div {
                    class: "navbar-start",
                    Link { to: Routes::Home {}, class: "flex flex-row items-center",
                        img { class: "mr-1 rounded-none", src: "/favicon-32.png" }
                        p { class:"font-semibold", "Taigla" }
                    }
                }
                div {
                    class: "navbar-center",
                    LinkList {}
                }
                div {
                    class: "navbar-end",
                    p { "Profile" }
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
                Routes::Home { .. } => "!text-primary",
                _ => ""
            },
            _ => match &link {
                Routes::Home { .. } => "",
                l => if l.is_child_of(&route) { "!text-primary" } else { "" }
            }
        };

        return rsx! {
            li {
                Link { key: "{name}", class: "{active} text-sm", to: link, name }
            }
        };
    });

    render! {
        ul {
            class: "menu menu-horizontal",
            links
        }
    }
}
