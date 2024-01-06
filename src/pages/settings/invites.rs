#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::prelude::*;
use crate::services::settings::{INVITE_LIST_STORE, SettingCommand};
use crate::api::{QueryState, Invite};
use crate::components::modal::invite::{InviteModalState, STATE, Invite};

#[component]
pub fn InviteList<'a>(cx: Scope, invites: &'a Vec<Invite>) -> Element {
    let rows = invites.iter().map(|invite| {
        rsx! {
            tr {
                key: "{invite.key}",
                td { "{invite.name}" }
                td { "{invite.key}" }
                td { "{invite.remaining_use}" }
            }
        }
    });

    render! {
        table {
            class: "table bordered",
            thead {
                tr {
                    th { class: "w-7/12", "Name" }
                    th { class: "w-4/12", "Key" }
                    th { class: "w-1/12", "Reamaining use" }
                }
            }
            tbody {
                rows
            }
        }
    }
}

pub fn Invites(cx: Scope) -> Element {
    let set_modal_state = use_set(cx, &STATE);
    let invites = use_read(cx, &INVITE_LIST_STORE);
    let setting_handle = use_coroutine_handle::<SettingCommand>(cx);

    use_memo(cx, (), |_| {
        setting_handle.map(|h| h.send(SettingCommand::FetchInvitesList));
    });

    render! {
        div {
            class: "flex flex-col w-full",
            div {
                class: "flex flex-row justify-between pb-2",
                p { class: "text-3xl", "Invites" }
                p {
                    onclick: move |_| set_modal_state(InviteModalState::New),
                    class: "btn btn-primary", "New"
                }
            }
            match &invites {
                QueryState::Ok(Invites) => rsx! {
                    InviteList {
                        invites: Invites,
                    }
                },
                QueryState::Loading => rsx! { "Loading" },
                _ => rsx! { "Error" }
            }
            Invite {}
        }
    }
}
