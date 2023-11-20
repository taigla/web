use dioxus::prelude::*;
pub use indexer::Indexer;

mod indexer;

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
