use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::runner::RunResult;

const COMPILE_TIME_LIMIT: Duration = Duration::from_secs(60);
const RUN_TIME_LIMIT: Duration = Duration::from_secs(5);
const POLL_INTERVAL: Duration = Duration::from_millis(20);

pub fn run(code: &str) -> RunResult {
    let workspace = create_workspace();

    let result = match workspace.as_ref() {
        Ok(directory) => compile_and_run(directory, code),
        Err(error) => RunResult::failure(format!("Could not prepare workspace: {error}")),
    };

    if let Ok(directory) = workspace {
        let _ = fs::remove_dir_all(directory);
    }

    result
}

fn compile_and_run(directory: &Path, code: &str) -> RunResult {
    let source = directory.join("main.rs");
    let executable = directory.join(format!("program{}", env::consts::EXE_SUFFIX));

    if let Err(error) = fs::write(&source, code) {
        return RunResult::failure(format!("Could not save your code: {error}"));
    }

    match compile(&source, &executable) {
        Ok(output) if output.success => execute(&executable),
        Ok(output) => RunResult {
            success: false,
            stdout: String::new(),
            stderr: output.stderr,
        },
        Err(error) => RunResult::failure(compiler_error_message(&error)),
    }
}

fn compile(source: &Path, executable: &Path) -> io::Result<ProcessOutput> {
    let mut command = Command::new("rustc");
    command
        .arg("--edition")
        .arg("2024")
        .arg("--color")
        .arg("never")
        .arg("-o")
        .arg(executable)
        .arg(source);

    run_with_time_limit(command, COMPILE_TIME_LIMIT)
}

fn execute(executable: &Path) -> RunResult {
    match run_with_time_limit(Command::new(executable), RUN_TIME_LIMIT) {
        Ok(output) => RunResult {
            success: output.success,
            stdout: output.stdout,
            stderr: output.stderr,
        },
        Err(error) => RunResult::failure(format!("Could not start your program: {error}")),
    }
}

fn compiler_error_message(error: &io::Error) -> String {
    if error.kind() == io::ErrorKind::NotFound {
        "The Rust compiler (rustc) was not found. Install Rust from https://rustup.rs and restart the app."
            .to_string()
    } else {
        format!("Could not start the Rust compiler: {error}")
    }
}

struct ProcessOutput {
    success: bool,
    stdout: String,
    stderr: String,
}

fn run_with_time_limit(mut command: Command, limit: Duration) -> io::Result<ProcessOutput> {
    let mut child = command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let stdout_reader = read_in_background(child.stdout.take());
    let stderr_reader = read_in_background(child.stderr.take());
    let started = Instant::now();

    let exit_status = loop {
        if let Some(status) = child.try_wait()? {
            break Some(status);
        }

        if started.elapsed() >= limit {
            child.kill()?;
            child.wait()?;
            break None;
        }

        thread::sleep(POLL_INTERVAL);
    };

    let stdout = stdout_reader.join().unwrap_or_default();
    let mut stderr = stderr_reader.join().unwrap_or_default();

    let success = match exit_status {
        Some(status) => status.success(),
        None => {
            stderr.push_str(&format!(
                "\nStopped: the program ran longer than {} seconds.",
                limit.as_secs()
            ));
            false
        }
    };

    Ok(ProcessOutput {
        success,
        stdout,
        stderr,
    })
}

fn read_in_background<R>(stream: Option<R>) -> JoinHandle<String>
where
    R: Read + Send + 'static,
{
    thread::spawn(move || {
        let mut bytes = Vec::new();

        if let Some(mut stream) = stream {
            let _ = stream.read_to_end(&mut bytes);
        }

        String::from_utf8_lossy(&bytes).into_owned()
    })
}

fn create_workspace() -> io::Result<PathBuf> {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos())
        .unwrap_or_default();
    let directory = env::temp_dir().join(format!("rust-academy-{}-{nanos}", std::process::id()));

    fs::create_dir_all(&directory)?;
    Ok(directory)
}
