use std::path::Path;

use crate::{domain::errors::ParseError, infra::DiskUsageStats};

pub fn get_disk_usage_for(path: &Path) -> Result<DiskUsageStats, ParseError> {}

}
