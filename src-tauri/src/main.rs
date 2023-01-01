#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use std::{env::set_current_dir, process::Command};

fn main() {
    set_current_dir("/Users/austen/Code/arsenal").expect("Unable to find path.");
    let server = Command::new("php")
        .args(["-S", "localhost:5150", "-t", "public"])
        .spawn()
        .expect("Could not start server.");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
