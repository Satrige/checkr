use std::{fs, path::PathBuf};

const DEFAULT_MOUNTS_PATH: &str = "/proc/self/mounts";

#[derive(thiserror::Error, Debug)]
pub enum SelfMountsError {
    #[error("Can't read mounts file: {0}")]
    ReadError(String),

    #[error("Wrong mounts file format: {0}")]
    ParseError(String),
}

#[derive(Debug)]
pub struct MountEntry {
    pub source: String,
    pub target: PathBuf,
    pub fs_type: String,
}

pub struct ProcSelfMounts {
    moutns_path: PathBuf,
}

impl ProcSelfMounts {
    pub fn new() -> Self {
        Self {
            moutns_path: PathBuf::from(DEFAULT_MOUNTS_PATH),
        }
    }

    pub fn parse_mounts(&self) -> Result<Vec<MountEntry>, SelfMountsError> {
        let mounts_info = self.read_mounts()?;

        let mut mount_entries = Vec::new();

        for line in mounts_info.lines() {
            let mut parts = line.split_whitespace();

            let source = parts.next().unwrap_or("").to_string();
            let target = parts.next().unwrap_or("").to_string();
            let fs_type = parts.next().unwrap_or("").to_string();

            if source.is_empty() || target.is_empty() || fs_type.is_empty() {
                return Err(SelfMountsError::ParseError(format!(
                    "{}: Invalid line: {}",
                    self.moutns_path.display(),
                    line
                )));
            }

            mount_entries.push(MountEntry {
                source,
                target: PathBuf::from(target),
                fs_type,
            });
        }

        Ok(mount_entries)
    }

    fn read_mounts(&self) -> Result<String, SelfMountsError> {
        fs::read_to_string(&self.moutns_path).map_err(|e| {
            SelfMountsError::ReadError(format!("{}: {}", self.moutns_path.display(), e.to_string()))
        })
    }
}
