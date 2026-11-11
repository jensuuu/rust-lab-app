use std::fs;
use std::path::PathBuf;

use tauri::{AppHandle, Manager};

pub fn load(app: &AppHandle) -> Result<Vec<u32>, String> {
    let path = progress_file(app)?;

    if !path.exists() {
        return Ok(Vec::new());
    }

    let text = fs::read_to_string(path).map_err(|error| error.to_string())?;
    Ok(serde_json::from_str(&text).unwrap_or_default())
}

pub fn mark_completed(app: &AppHandle, lesson_id: u32) -> Result<(), String> {
    let mut completed = load(app)?;

    if completed.contains(&lesson_id) {
        return Ok(());
    }

    completed.push(lesson_id);
    completed.sort_unstable();
    save(app, &completed)
}

fn save(app: &AppHandle, completed: &[u32]) -> Result<(), String> {
    let path = progress_file(app)?;
    let text = serde_json::to_string(completed).map_err(|error| error.to_string())?;
    fs::write(path, text).map_err(|error| error.to_string())
}

fn progress_file(app: &AppHandle) -> Result<PathBuf, String> {
    let directory = app
        .path()
        .app_data_dir()
        .map_err(|error| error.to_string())?;
    fs::create_dir_all(&directory).map_err(|error| error.to_string())?;
    Ok(directory.join("progress.json"))
}
