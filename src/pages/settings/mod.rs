#![allow(non_snake_case)]
use dioxus::prelude::*;

pub use indexers::Indexers;
pub use invites::Invites;
pub use request_profiles::RequestProfiles;
pub use users::Users;

mod indexers;
mod invites;
mod request_profiles;
mod users;

pub fn Settings(cx: Scope) -> Element {
    render! {
        "settings"
    }
}
