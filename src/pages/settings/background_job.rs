#![allow(non_snake_case)]
use dioxus::prelude::*;
use serde::Deserialize;
use crate::hooks::{use_query, QueryState};
use crate::states::ApiError;

#[derive(Deserialize)]
pub enum WorkerState {
    Idle,
    #[serde(untagged)]
    Running {
        job: String
    }
}

#[derive(Deserialize)]
struct Job {
    uuid: String,
    interval: u32,
    name: String
}

#[inline_props]
fn WorkerList<'a>(cx: Scope, workers: &'a Vec<WorkerState>) -> Element {
    let rows = workers.iter().map(|worker| {
        match worker {
            WorkerState::Idle => {
                rsx! {
                    tr {
                        // key: "{job.uuid}",
                        td { "Idle" }
                        td { "" }
                    }
                }
            },
            WorkerState::Running { job } => {
                rsx! {
                    tr {
                        // key: "{job.uuid}",
                        td { "Running" }
                        td { "{job}" }
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
                    th { class: "w-1/12", "Status" }
                    th { class: "w-11/12", "Job" }
                }
            }
            tbody {
                rows
            }
        }
    }
}

fn Workers(cx: Scope) -> Element {
    let workers = use_query::<Vec<WorkerState>, ApiError>(cx, "/api/v1/job/workers");

    render! {
        div {
            p { class: "text-xl", "Worker" }
            match &workers.value {
                QueryState::Ok(w) => rsx! { WorkerList { workers: &w } },
                QueryState::Loading => rsx! { "Loading" },
                _ => rsx! { "Error" }
            }
        }
    }
}

#[inline_props]
fn CronjobList<'a>(cx: Scope, jobs: &'a Vec<Job>) -> Element {
    let rows = jobs.iter().map(|job| {
        rsx! {
            tr {
                key: "{job.uuid}",
                td { "{job.name}" }
                td { "{job.interval}" }
            }
        }
    });

    render! {
        table {
            class: "table bordered",
            thead {
                tr {
                    th { class: "w-11/12", "Name" }
                    th { class: "w-1/12", "Interval" }
                }
            }
            tbody {
                rows
            }
        }
    }
}

fn Cronjobs(cx: Scope) -> Element {
    let cronjobs = use_query::<Vec<Job>, ApiError>(cx, "/api/v1/job/cronjobs");

    render! {
        div {
            p { class: "text-xl", "Cronjob" }
            match &cronjobs.value {
                QueryState::Ok(j) => rsx! { CronjobList { jobs: &j } },
                QueryState::Loading => rsx! { "Loading" },
                _ => rsx! { "Error" }
            }
        }
    }
}

pub fn BackgroundJobs(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col w-full",
            div {
                class: "flex flex-row justify-between pb-2",
                p { class: "text-2xl", "Background jobs" }
            }
            Workers {}
            Cronjobs {}
        }
    }
}
