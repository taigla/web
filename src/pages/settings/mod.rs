#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::routes::Routes;

pub use indexers::Indexers;
pub use invites::Invites;
pub use request_profiles::RequestProfiles;
pub use users::Users;
pub use background_job::BackgroundJobs;

mod indexers;
mod invites;
mod request_profiles;
mod users;
mod background_job;

pub fn Settings(cx: Scope) -> Element {
    let navigator = use_navigator(cx);

    navigator.replace(Routes::Users {});
    return render! {""}
}
