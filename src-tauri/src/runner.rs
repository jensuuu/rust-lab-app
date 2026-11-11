use serde::Serialize;

#[cfg(desktop)]
pub use crate::local::run;
#[cfg(mobile)]
pub use crate::remote::run;

#[derive(Serialize)]
pub struct RunResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

impl RunResult {
    pub(crate) fn failure(message: impl Into<String>) -> Self {
        Self {
            success: false,
            stdout: String::new(),
            stderr: message.into(),
        }
    }
}
