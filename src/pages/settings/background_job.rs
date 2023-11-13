#![allow(non_snake_case)]
use dioxus::prelude::*;
use fermi::prelude::use_read;
use crate::services::settings::{WORKER_LIST_STORE, SettingCommand, CRONJOB_LIST_STORE};
use crate::api::{QueryState, WorkerState, Job};

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
    let setting_handle = use_coroutine_handle::<SettingCommand>(cx);
    let workers = use_read(cx, &WORKER_LIST_STORE);

    use_memo(cx, (), |_| {
        setting_handle.map(|h| h.send(SettingCommand::FetchWorkersList));
    });

    render! {
        div {
            p { class: "text-xl", "Worker" }
            match &workers {
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
    let setting_handle = use_coroutine_handle::<SettingCommand>(cx);
    let cronjobs = use_read(cx, &CRONJOB_LIST_STORE);

    use_memo(cx, (), |_| {
        setting_handle.map(|h| h.send(SettingCommand::FetchCronjobsList));
    });

    render! {
        div {
            p { class: "text-xl", "Cronjob" }
            match &cronjobs {
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
                p { class: "text-3xl", "Background jobs" }
            }
            Workers {}
            Cronjobs {}
        }
    }
}
