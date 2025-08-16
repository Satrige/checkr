use std::path::PathBuf;

pub struct MountEntry {
    pub source: String,
    pub target: PathBuf,
    pub fs_type: String,
}

pub struct DiskUsageStats {
    pub total: u64,
    pub free: u64,
    pub available: u64,
    pub used: u64,
    pub pct_used: f32,
}
