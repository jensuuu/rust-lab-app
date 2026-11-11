// Tauri's mobile entry point macro generates `#[no_mangle]` functions.
#![cfg_attr(mobile, allow(unsafe_code))]

mod book;
mod commands;
mod lessons;
#[cfg(desktop)]
mod local;
mod progress;
#[cfg(mobile)]
mod remote;
mod runner;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::list_lessons,
            commands::list_book,
            commands::get_book_section,
            commands::search_book,
            commands::get_progress,
            commands::run_code,
            commands::check_exercise,
        ])
        .run(tauri::generate_context!())
        .expect("failed to start Rust Academy");
}
