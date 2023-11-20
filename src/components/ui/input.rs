use dioxus::prelude::*;

#[inline_props]
#[allow(non_snake_case)]
pub fn Input<'a>(cx: Scope, name: &'a str, lbl: Option<&'a str>, default_value: Option<&'a str>) -> Element<'a> {
    render! {
        Fragment {
            if let Some(lbl) = *lbl {
                rsx! { label { class: "col-span-12 md:col-span-3", lbl } }
            }
            rsx! { input { class: "input input-bordered col-span-12 md:col-span-9", initial_value: *default_value, name: *name } }
        }
    }
}
