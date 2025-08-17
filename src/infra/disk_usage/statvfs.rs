use libc::statvfs;
use std::{ffi::CString, io, os::unix::ffi::OsStrExt, path::Path};

use crate::{domain::errors::ParseError, infra::DiskUsageStats};

pub struct StatfsData;

impl StatfsData {
    pub fn get_disk_usage_for(mount_path: &Path) -> Result<DiskUsageStats, ParseError> {
        let statvfs_buffer = Self::get_statvfs_data(mount_path)?;

        Ok(Self::calc_disk_usage_for(&statvfs_buffer))
    }

    fn get_statvfs_data(mount_path: &Path) -> Result<statvfs, ParseError> {
        // Convert Path to CString for libc usage
        let c_path = CString::new(mount_path.as_os_str().as_bytes()).map_err(|err| {
            ParseError::DiskUsageError(format!(
                "Statvfs: path contains interior NUL: {}",
                err.to_string()
            ))
        })?;

        // Fill statfs buffer with zeroes
        let mut statvfs_buffer: statvfs = unsafe { std::mem::zeroed() };
        let statvfs_status_code =
            unsafe { statvfs(c_path.as_ptr(), &mut statvfs_buffer as *mut statvfs) };

        if statvfs_status_code != 0 {
            return Err(ParseError::DiskUsageError(format!(
                "Statvfs: Can't get statvfs info: {} | Status code: {}",
                io::Error::last_os_error().to_string(),
                statvfs_status_code
            )));
        }

        Ok(statvfs_buffer)
    }

    fn calc_disk_usage_for(statvfs_buffer: &statvfs) -> DiskUsageStats {
        let block_size = if statvfs_buffer.f_frsize != 0 {
            statvfs_buffer.f_frsize as u64
        } else {
            statvfs_buffer.f_bsize as u64
        };

        let total = (statvfs_buffer.f_blocks as u64).saturating_mul(block_size);
        let free = (statvfs_buffer.f_bfree as u64).saturating_mul(block_size);
        let used = total.saturating_sub(free);
        let available = (statvfs_buffer.f_bavail as u64).saturating_mul(block_size);
        let percent_used = if total == 0 {
            0.0
        } else {
            (used as f64 / total as f64 * 100.0) as f32
        };

        DiskUsageStats {
            total,
            free,
            available,
            used,
            percent_used,
        }
    }
}
