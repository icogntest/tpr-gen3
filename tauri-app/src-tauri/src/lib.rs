use std::sync::OnceLock;
use tauri::{AppHandle, Manager};
use tauri_plugin_shell::{process::Command, ShellExt};

// From: https://github.com/tauri-apps/tauri/discussions/6309#discussioncomment-10295527
static APP_HANDLE: OnceLock<AppHandle> = OnceLock::new();

fn app_handle<'a>() -> &'a AppHandle {
    APP_HANDLE.get().unwrap()
}

// // Enjoy:
// fn foo() {
//   let app_handle = app_handle();
// }

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context = tauri::generate_context!();

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .build(context)
        .unwrap();

    APP_HANDLE.set(app.app_handle().to_owned()).unwrap();

    let app_handle = app_handle();

    let website_dir = app_handle
        .path()
        .resolve("resources/website", tauri::path::BaseDirectory::Resource)
        .unwrap();

    // let sidecar_command = app_handle.shell().sidecar("bin/node_v20_17_0").unwrap();
    let sidecar_command = app_handle.shell().sidecar("node_v20_17_0").unwrap();
    let (mut rx, mut _child) = sidecar_command
        .args(["server.js"])
        .current_dir(website_dir)
        .spawn()
        .expect("failed to spawn sidecar");

    // tauri::async_runtime::spawn(async move {
    //     while let Some(event)
    // })

    app.run(|_, _| {});
}
