use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CheckStatus {
    OK,
    ERROR,
    WARNING,
    CRITICAL,
    DISABLED,
}

#[derive(Serialize)]
pub struct CheckResult {
    name: String,
    result: CheckStatus,
    descr: Option<String>,
}

impl CheckResult {
    pub fn new(name: String, result: CheckStatus, descr: Option<String>) -> Self {
        CheckResult {
            name,
            result,
            descr,
        }
    }
}

pub trait Checker: Send + Sync {
    fn get_name(&self) -> &str;

    fn is_enabled(&self) -> bool;

    fn check(&self) -> anyhow::Result<CheckResult>;
}
