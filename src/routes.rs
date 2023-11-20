use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::pages::{
    Home,
    Settings,
    settings::{Indexers, Invites, RequestProfiles, Users},
    auth::Login
};
use crate::components::{Header, SettingsNavbar};

#[derive(Clone, Routable)]
pub enum Routes {
    #[layout(Header)]
        #[route("/")]
        #[redirect("/:.._segments", |_segments: Vec<String>| Routes::Home {})]
        Home {},
        #[layout(SettingsNavbar)]
            #[nest("/settings")]
                #[route("/")]
                Settings {},
                #[route("/indexers")]
                Indexers {},
                #[route("/invites")]
                Invites {},
                #[route("/request-profiles")]
                RequestProfiles {},
                #[route("/users")]
                Users {},
            #[end_nest]
        #[end_layout]
    #[end_layout]
    #[nest("/auth")]
        #[route("/login")]
        Login {},
}
