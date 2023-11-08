use dioxus::prelude::*;

pub mod indexer;

#[allow(non_snake_case)]
#[inline_props]
pub fn Modal<'a>(cx: Scope, visible: bool, children: Element<'a>) -> Element {
    if *visible {
        render! {
            div {
                class: "absolute top-0 left-0 bottom-0 right-0 flex",
                div {
                    class: "absolute top-0 left-0 bottom-0 right-0 bg-bw-50 opacity-50"
                }
                div {
                    class: "w-full md:w-3/5 bg-bw-100 m-auto rounded-lg z-50",
                    children
                }
            }
        }
    } else {
        None
    }
}

#[allow(non_snake_case)]
#[inline_props]
pub fn ModalWithTitle<'a>(cx: Scope, visible: bool, title: &'a str, children: Element<'a>) -> Element {
    render! {
        Modal {
            visible: *visible,
            div {
                class: "flex flex-col p-6",
                p { class: "text-2xl", "{title}" }
                button { class: "absolute right-4 top-3", value: "x"}
                children
            }
        }
    }
}
