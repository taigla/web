use dioxus::prelude::*;

#[component]
#[allow(non_snake_case)]
pub fn Select<'a>(cx: Scope, name: &'a str, lbl: Option<&'a str>, default_value: Option<&'a str>, options: Vec<&'a str>) -> Element<'a> {
    let selected = default_value.unwrap_or("");
    let child = options.iter().map(|opt| {
        rsx! {
            option {selected: *opt == selected , *opt }
        }
    });

    render! {
        Fragment {
            if let Some(lbl) = *lbl {
                rsx! { label { class: "col-span-12 md:col-span-3", lbl } }
            }
            rsx! {
                select {
                    class: "select select-bordered col-span-12 md:col-span-9",
                    name: *name,
                    child
                }
            }
        }
    }
}
