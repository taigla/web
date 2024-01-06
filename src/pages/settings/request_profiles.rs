#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::prelude::*;
use fermi::use_set;
use crate::components::modal::request_profile::{RequestProfile, RequestProfileModalState, STATE};
use crate::services::settings::{REQUEST_PROFILE_LIST_STORE, SettingCommand};
use crate::api::{QueryState, RequestProfileRow};

#[component]
pub fn RequestProfileList<'a>(cx: Scope, request_profiles: &'a Vec<RequestProfileRow>, on_request_profile_select: EventHandler<'a, u64>) -> Element {
    let rows = request_profiles.iter().map(|request_profile| {
        rsx! {
            tr {
                key: "{request_profile.id}",
                td { "{request_profile.name}" }
                td {
                    style: "padding-top: 0; padding-bottom: 0;",
                    button {
                        class: "btn btn-sm",
                        onclick: move |_| on_request_profile_select.call(request_profile.id),
                        "Edit"
                    }
                }
            }
        }
    });

    render! {
        table {
            class: "table bordered",
            thead {
                tr {
                    th { class: "w-11/12", "Name" }
                    th { class: "w-1/12", "" }
                }
            }
            tbody {
                rows
            }
        }
    }
}

pub fn RequestProfiles(cx: Scope) -> Element {
    let set_modal_state = use_set(cx, &STATE);
    let request_profiles = use_read(cx, &REQUEST_PROFILE_LIST_STORE);
    let setting_handle = use_coroutine_handle::<SettingCommand>(cx);

    use_memo(cx, (), |_| {
        setting_handle.map(|h| h.send(SettingCommand::FetchRequestProfilesList));
    });

    render! {
        div {
            class: "flex flex-col w-full",
            div {
                class: "flex flex-row justify-between pb-2",
                p { class: "text-3xl", "Request profiles" }
                button {
                    onclick: move |_| set_modal_state(RequestProfileModalState::New),
                    class: "btn btn-primary",
                    "New"
                }
            }
            match &request_profiles {
                QueryState::Ok(request_profiles) => rsx! {
                    RequestProfileList {
                        request_profiles: request_profiles,
                        on_request_profile_select: move |id| set_modal_state(RequestProfileModalState::Id(id))
                    }
                },
                QueryState::Loading => rsx! { "Loading" },
                _ => rsx! { "Error" }
            }
            RequestProfile {}
        }
    }
}
