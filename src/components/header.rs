#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::routes::Routes;
use crate::api::Token;

static LINKS: &'static [(&str, Routes)] = &[
    ("Home", Routes::Home {}),
    ("Settings", Routes::Settings {})
];

#[inline_props]
pub fn Dropdown<'a>(
    cx: Scope,
    options: Vec<&'static str>,
    on_click: EventHandler<'a, usize>,
    children: Element<'a>
) -> Element<'a> {
    let options = options.iter().enumerate().map(|(index, option)| {
        rsx! {
            li { a { onclick: move |_| on_click.call(index), *option } }
        }
    });

    render! {
        div {
            class: "dropdown dropdown-end",
            tabindex: 0,
            children
            ul {
                tabindex: 0,
                class: "mt-3 z-[1] p-2 shadow menu bg-base-200 dropdown-content rounded-box w-52",
                options
            }
        }
    }
}

pub fn Header(cx: Scope) -> Element {
    let navigator = use_navigator(cx);
    let token = use_shared_state::<Token>(cx).unwrap();

    let on_downdown_item_pressed = move |index| {
        match index {
            0 => { navigator.push(Routes::UserPreferences {}); },
            1 => { token.read().remove(); },
            _ => {}
        }
    };

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
                    Dropdown {
                        options: vec!["Settings", "Logout"],
                        on_click: on_downdown_item_pressed,
                        div {
                            class: "btn btn-ghost btn-circle avatar placeholder",
                            div {
                                class: "bg-neutral-focus text-neutral-content rounded-full w-10",
                                "U"
                            }
                        }
                    }
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
