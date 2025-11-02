use std::io;
use std::process::Command;

#[derive(thiserror::Error, Debug)]
pub enum SsInvokeError {
    #[error("Failed to invoke ss command: {0}")]
    ErrorInvocation(#[from] io::Error),

    #[error("Incorrect status code of ss command: {0}")]
    WrongStatusCode(String),
}

pub struct SsRunner;

impl SsRunner {
    pub fn run() -> Result<String, SsInvokeError> {
        let output = Command::new("ss").args(["-H", "-tulnp"]).output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(SsInvokeError::WrongStatusCode(stderr.to_string()));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
