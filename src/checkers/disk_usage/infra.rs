mod proc_self_mounts;
mod statvfs;

use super::DiskUsageSource;
use once_cell::sync::Lazy;
use proc_self_mounts::*;
use statvfs::*;
use std::collections::HashSet;

#[derive(Clone)]
pub struct DiskSnapshot {
    pub mount: String,
    pub usage: f32,
}

// TODO: Cover only 95% of the cases. Need to enhance in further versions
static SKIP_FS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    [
        "proc",
        "sysfs",
        "tmpfs",
        "devpts",
        "devtmpfs",
        "mqueue",
        "pstore",
        "securityfs",
        "configfs",
        "bpf",
        "tracefs",
        "debugfs",
        "hugetlbfs",
        "ramfs",
        "selinuxfs",
        "binfmt_misc",
        "fusectl",
        "autofs",
        "cgroup",
        "cgroup2",
        "overlay",
        "squashfs",
    ]
    .into_iter()
    .collect()
});

pub struct ProcDiskUsage;

impl ProcDiskUsage {
    fn filter_mount_entries(mount_entries: Vec<MountEntry>) -> Vec<MountEntry> {
        mount_entries
            .into_iter()
            .filter(|mount_entry| !SKIP_FS.contains(mount_entry.fs_type.as_str()))
            .collect() // here we will allocate a new vector, using filtered vals
    }

    fn get_mount_entries() -> anyhow::Result<Vec<MountEntry>> {
        let proc_self_mounts = ProcSelfMounts::new();

        let mount_entries = proc_self_mounts.parse_mounts()?;

        Ok(Self::filter_mount_entries(mount_entries))
    }
}

impl DiskUsageSource for ProcDiskUsage {
    fn parse_values(&self) -> anyhow::Result<Vec<DiskSnapshot>> {
        let snapshots = Self::get_mount_entries()?
            .iter()
            .map(|mount_entry| {
                StatfsData::get_disk_usage_for(&mount_entry.target).map(|usage| DiskSnapshot {
                    mount: mount_entry.target.display().to_string(),
                    usage: usage.percent_used,
                })
            })
            .collect::<Result<Vec<DiskSnapshot>, StatFsError>>()?;

        Ok(snapshots)
    }
}
