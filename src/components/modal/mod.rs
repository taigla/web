use dioxus::prelude::*;

pub mod indexer;
pub mod invite;
pub mod request_profile;

#[allow(non_snake_case)]
#[inline_props]
pub fn Modal<'a>(cx: Scope, visible: bool, children: Element<'a>) -> Element {
    if *visible {
        render! {
            div {
                class: "modal modal-open absolute top-0 left-0 bottom-0 right-0 flex",
                div {
                    class: "modal-backdrop"
                }
                div {
                    class: "w-full md:w-3/5 m-auto modal-box max-w-none",
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
                class: "flex flex-row pb-3 justify-between",
                p { class: "text-2xl", "{title}" }
                button { class: "btn btn-sm btn-circle btn-ghost", "✕" }
            }
            children
        }
    }
}
