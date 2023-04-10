# Tauri + Laravel

This is a hacky way of getting IPC working between Laravel and Tauri.

It relies on an in-progress pull request in Tauri:
[https://github.com/tauri-apps/tauri/pull/5918](https://github.com/tauri-apps/tauri/pull/5918).

For more info on how to use unpublished Tauri changes, [check out this FAQ page](https://tauri.app/v1/guides/faq/#how-can-i-use-unpublished-tauri-changes)

## Setup
- Clone this repo
- Clone the Laravel repo from here: [https://github.com/austenc/laravel-tauri](https://github.com/austenc/laravel-tauri)
- Run `cd src/tauri && cargo install` in this repo
- Run the app with `cargo run --no-default-features`
- Inspect the console output to see the IPC messages

Beware:
- This is a prototype
- It relies on a work-in-progress PR from the Tauri team
- The PHP serve process is never killed when the app is closed
