#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::{use_read, use_set, Atom};
use super::ModalWithTitle;
use crate::hooks::use_taigla_api;
use crate::services::settings::SettingCommand;
use crate::api::InviteCreate;
use crate::components::ui::Input;

pub static STATE: Atom<InviteModalState> = Atom(|_| InviteModalState::Close);

#[derive(PartialEq)]
pub enum InviteModalState {
    New,
    Close
}

#[inline_props]
fn Form<'a>(cx: Scope, on_update: EventHandler<'a, InviteCreate>) -> Element<'a> {
    let set_state = use_set(cx, &STATE);

    let submit = move |evt: Event<FormData>| {
        let invite = InviteCreate {
            name: evt.data.values.get("name").unwrap().get(0).unwrap().to_string(),
            remaining_use: evt.data.values.get("remaining_use").unwrap().get(0).unwrap().parse::<u32>().unwrap()
        };
        on_update.call(invite);
    };

    render! {
        form {
            class: "grid grid-cols-12 gap-y-5 items-center",
            onsubmit: submit,
            Input {
                lbl: "Name:",
                name: "name",
            }
            Input {
                lbl: "Number of use (-1 for unlimited):",
                name: "remaining_use",
                default_value: "1"
            }
            div {
                class: "flex flex-row justify-end col-span-12 gap-2",
                p {
                    class: "btn",
                    onclick: move |_| set_state(InviteModalState::Close),
                    "Close"
                }
                input {
                    class: "btn btn-primary",
                    r#type: "submit",
                    value: "Save"
                }
            }
        }
    }
}

#[inline_props]
pub fn Invite(cx: Scope) -> Element {
    let state = use_read(cx, &STATE);
    let set_state = use_set(cx, &STATE);
    let api = use_taigla_api(cx);
    let setting_handle = use_coroutine_handle::<SettingCommand>(cx).unwrap();

    let create = move |invite| {
        to_owned![api, set_state, setting_handle];
        cx.spawn(async move {
            let invite = api.read().post_invite(invite)
                .await;
            log::info!("{:?}", invite);
            if let Ok(_) = invite {
                set_state(InviteModalState::Close);
                setting_handle.send(SettingCommand::FetchInvitesList);
            }
        });
    };

    render! {
        ModalWithTitle {
            visible: *state != InviteModalState::Close,
            on_close: move |_| set_state(InviteModalState::Close),
            title: "Invite",
            match state {
                InviteModalState::New => rsx! { Form {
                    on_update: create
                } },
                _ => rsx! { "" }
            }
        }
    }
}
