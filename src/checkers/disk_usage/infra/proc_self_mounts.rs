use std::{fs, path::PathBuf};

use super::super::ParseError;
use super::MountEntry;

const DEFAULT_MOUNTS_PATH: &str = "/proc/self/mounts";

pub struct ProcSelfMounts {
    moutns_path: PathBuf,
}

impl ProcSelfMounts {
    pub fn new() -> Self {
        Self {
            moutns_path: PathBuf::from(DEFAULT_MOUNTS_PATH),
        }
    }

    pub fn parse_mounts(&self) -> Result<Vec<MountEntry>, ParseError> {
        let mounts_info = self.read_mounts()?;

        let mut mount_entries = Vec::new();

        for line in mounts_info.lines() {
            let mut parts = line.split_whitespace();

            let source = parts.next().unwrap_or("").to_string();
            let target = parts.next().unwrap_or("").to_string();
            let fs_type = parts.next().unwrap_or("").to_string();

            if source.is_empty() || target.is_empty() || fs_type.is_empty() {
                return Err(ParseError::DiskUsageError(format!(
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

    fn read_mounts(&self) -> Result<String, ParseError> {
        fs::read_to_string(&self.moutns_path).map_err(|e| {
            ParseError::DiskUsageError(format!("{}: {}", self.moutns_path.display(), e.to_string()))
        })
    }
}
