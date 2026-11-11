use serde::Serialize;
use tauri::AppHandle;

use crate::book::{self, ChapterSummary, SearchHit, Section};
use crate::lessons::{self, Lesson};
use crate::progress;
use crate::runner::{self, RunResult};

const SEARCH_RESULT_LIMIT: usize = 40;

#[derive(Serialize)]
pub struct CheckResult {
    passed: bool,
    run: RunResult,
    expected: String,
}

#[tauri::command]
pub fn list_lessons() -> Vec<Lesson> {
    lessons::all().to_vec()
}

#[tauri::command]
pub fn list_book() -> Vec<ChapterSummary> {
    book::outline()
}

#[tauri::command]
pub fn get_book_section(chapter_id: usize, section_id: usize) -> Result<&'static Section, String> {
    book::section(chapter_id, section_id).ok_or_else(|| "That section does not exist".to_string())
}

#[tauri::command]
pub fn search_book(query: String) -> Vec<SearchHit> {
    book::search(&query, SEARCH_RESULT_LIMIT)
}

#[tauri::command]
pub fn get_progress(app: AppHandle) -> Result<Vec<u32>, String> {
    progress::load(&app)
}

#[tauri::command]
pub async fn run_code(code: String) -> Result<RunResult, String> {
    run_in_background(code).await
}

#[tauri::command]
pub async fn check_exercise(
    app: AppHandle,
    lesson_id: u32,
    code: String,
) -> Result<CheckResult, String> {
    let lesson = lessons::find(lesson_id).ok_or("Unknown lesson")?;
    let run = run_in_background(code).await?;
    let expected = lesson.exercise.expected_output;
    let passed = run.success && normalize(&run.stdout) == normalize(expected);

    if passed {
        progress::mark_completed(&app, lesson_id)?;
    }

    Ok(CheckResult {
        passed,
        run,
        expected: expected.to_string(),
    })
}

async fn run_in_background(code: String) -> Result<RunResult, String> {
    tauri::async_runtime::spawn_blocking(move || runner::run(&code))
        .await
        .map_err(|error| error.to_string())
}

fn normalize(text: &str) -> String {
    text.lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}
