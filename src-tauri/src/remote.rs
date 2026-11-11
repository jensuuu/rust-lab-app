//! Phones cannot run `rustc`, so on mobile the code is compiled and run
//! by the Rust Playground (https://play.rust-lang.org) instead.

use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::runner::RunResult;

const ENDPOINT: &str = "https://play.rust-lang.org/execute";
const REQUEST_TIME_LIMIT: Duration = Duration::from_secs(60);
const CARGO_PROGRESS_PREFIXES: [&str; 3] = ["Compiling", "Finished", "Running"];

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Request<'a> {
    channel: &'a str,
    mode: &'a str,
    edition: &'a str,
    crate_type: &'a str,
    tests: bool,
    backtrace: bool,
    code: &'a str,
}

#[derive(Deserialize)]
struct Response {
    success: bool,
    stdout: String,
    stderr: String,
}

pub fn run(code: &str) -> RunResult {
    let request = Request {
        channel: "stable",
        mode: "debug",
        edition: "2024",
        crate_type: "bin",
        tests: false,
        backtrace: false,
        code,
    };

    let agent = ureq::AgentBuilder::new()
        .timeout(REQUEST_TIME_LIMIT)
        .user_agent("rust-academy")
        .build();

    match agent.post(ENDPOINT).send_json(&request) {
        Ok(reply) => match reply.into_json::<Response>() {
            Ok(body) => RunResult {
                success: body.success,
                stdout: body.stdout,
                stderr: strip_cargo_progress(&body.stderr),
            },
            Err(error) => RunResult::failure(format!(
                "Unexpected reply from the Rust Playground: {error}"
            )),
        },
        Err(ureq::Error::Status(code, _)) => RunResult::failure(format!(
            "The Rust Playground returned an error (HTTP {code}). Try again in a moment."
        )),
        Err(ureq::Error::Transport(_)) => RunResult::failure(
            "Could not reach the Rust Playground. Check your internet connection and try again.",
        ),
    }
}

/// The playground reports cargo's own progress lines on stderr. They are noise
/// next to the program output, so only compiler messages are kept.
fn strip_cargo_progress(stderr: &str) -> String {
    stderr
        .lines()
        .filter(|line| {
            let line = line.trim_start();
            !CARGO_PROGRESS_PREFIXES
                .iter()
                .any(|prefix| line.starts_with(prefix))
        })
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}
