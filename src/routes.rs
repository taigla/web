use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::pages::{
    Home,
    Settings,
    settings::{Indexers, Invites, RequestProfiles, Users, BackgroundJobs},
    auth::Login
};
use crate::components::{Header, SettingsNavbar, LoginRequired};

#[derive(Clone, Routable)]
pub enum Routes {
    #[layout(LoginRequired)]
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
                    #[route("/background-jobs")]
                    BackgroundJobs {},
                #[end_nest]
            #[end_layout]
        #[end_layout]
    #[end_layout]
    #[nest("/auth")]
        #[route("/login")]
        Login {},
    // #[end_nest]
}
