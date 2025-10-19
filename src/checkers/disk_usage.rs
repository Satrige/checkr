mod config;
mod infra;
mod settings;

use super::{CheckLevel, CheckResult, CheckStatus, Checker};
use crate::checkers::disk_usage::infra::DiskSnapshot;
pub use config::*;
pub use infra::ProcDiskUsage;
pub use settings::DiskUsageSettings;

pub trait DiskUsageSource: Send + Sync {
    fn parse_values(&self) -> anyhow::Result<Vec<DiskSnapshot>>;
}

pub struct DiskUsageChecker<S: DiskUsageSource> {
    settings: DiskUsageSettings,
    name: String,
    source: S,
}

impl<S: DiskUsageSource> DiskUsageChecker<S> {
    pub fn new(settings: DiskUsageSettings, source: S) -> Self {
        DiskUsageChecker {
            settings,
            name: "disk_usage".to_string(),
            source,
        }
    }

    fn is_warning(&self, current_value: f32) -> bool {
        current_value > self.settings.warning_threshold
    }

    fn is_critical(&self, current_value: f32) -> bool {
        current_value > self.settings.critical_threshold
    }

    fn filter_by_level(
        &self,
        disk_snapshots: &[DiskSnapshot],
        lvl: CheckLevel,
    ) -> Vec<DiskSnapshot> {
        disk_snapshots
            .iter()
            .filter(|disk_snapshot| match lvl {
                CheckLevel::WARNING => self.is_warning(disk_snapshot.usage),
                CheckLevel::CRITICAL => self.is_critical(disk_snapshot.usage),
            })
            .cloned()
            .collect()
    }

    fn compile_descr(disk_snapshots: &[DiskSnapshot]) -> String {
        disk_snapshots
            .iter()
            .map(|disk_snapshot| {
                format!(
                    "Disk usage for {} is {:.1}%",
                    disk_snapshot.mount, disk_snapshot.usage
                )
            })
            .collect::<Vec<String>>()
            .join("; ")
    }
}

impl<S: DiskUsageSource> Checker for DiskUsageChecker<S> {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn is_enabled(&self) -> bool {
        self.settings.enabled
    }

    fn check(&self) -> anyhow::Result<CheckResult> {
        if !self.is_enabled() {
            return Ok(CheckResult::new(
                self.name.clone(),
                CheckStatus::DISABLED,
                None,
            ));
        }

        let load_values = self.source.parse_values()?;

        let critical_mounts = self.filter_by_level(&load_values, CheckLevel::CRITICAL);
        if critical_mounts.len() > 0 {
            return Ok(CheckResult::new(
                self.name.clone(),
                CheckStatus::CRITICAL,
                Some(Self::compile_descr(&critical_mounts)),
            ));
        }

        let warning_mounts = self.filter_by_level(&load_values, CheckLevel::WARNING);
        if warning_mounts.len() > 0 {
            return Ok(CheckResult::new(
                self.name.clone(),
                CheckStatus::WARNING,
                Some(Self::compile_descr(&warning_mounts)),
            ));
        }

        Ok(CheckResult::new(self.name.clone(), CheckStatus::OK, None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;

    // Simple fake source you can program with snapshots or an error
    #[derive(Clone, Default)]
    struct FakeDiskSource {
        snapshots: Vec<DiskSnapshot>,
        fail: bool,
    }

    impl DiskUsageSource for FakeDiskSource {
        fn parse_values(&self) -> anyhow::Result<Vec<DiskSnapshot>> {
            if self.fail {
                Err(anyhow!("boom"))
            } else {
                Ok(self.snapshots.clone())
            }
        }
    }

    fn settings(enabled: bool, warning: f32, critical: f32) -> DiskUsageSettings {
        DiskUsageSettings {
            enabled,
            warning_threshold: warning,
            critical_threshold: critical,
        }
    }

    fn snap(mount: &str, usage: f32) -> DiskSnapshot {
        DiskSnapshot {
            mount: mount.to_string(),
            usage,
        }
    }

    #[test]
    fn name_is_infra() {
        let checker = DiskUsageChecker::new(settings(true, 75.0, 90.0), FakeDiskSource::default());
        assert_eq!(checker.get_name(), "infra");
    }

    #[test]
    fn disabled_returns_disabled_status() {
        let checker = DiskUsageChecker::new(
            settings(false, 75.0, 90.0),
            FakeDiskSource {
                snapshots: vec![snap("/", 99.0)],
                fail: false,
            },
        );

        let res = checker.check().unwrap();
        assert_eq!(checker.is_enabled(), false);
        assert_eq!(res.result, CheckStatus::DISABLED);
        assert!(res.descr.is_none());
    }

    #[test]
    fn ok_when_all_below_warning_or_equal_to_warning() {
        let checker = DiskUsageChecker::new(
            settings(true, 75.0, 90.0),
            FakeDiskSource {
                snapshots: vec![snap("/", 40.0), snap("/home", 75.0)], // 75.0 == warning -> OK
                fail: false,
            },
        );

        let res = checker.check().unwrap();
        assert_eq!(res.result, CheckStatus::OK);
        assert!(res.descr.is_none());
    }

    #[test]
    fn warning_when_any_above_warning_and_none_critical() {
        let checker = DiskUsageChecker::new(
            settings(true, 75.0, 90.0),
            FakeDiskSource {
                snapshots: vec![
                    snap("/", 40.0),
                    snap("/home", 80.0), // warning
                    snap("/var", 89.9),  // warning (still < critical)
                ],
                fail: false,
            },
        );

        let res = checker.check().unwrap();
        assert_eq!(res.result, CheckStatus::WARNING);
        let descr = res.descr.unwrap();
        assert!(descr.contains("Disk usage for /home is 80.0%"));
        assert!(descr.contains("Disk usage for /var is 89.9%"));
        assert!(descr.contains("; "));
        assert!(!descr.contains("Disk usage for / is 40.0%"));
    }

    #[test]
    fn critical_when_any_above_critical() {
        let checker = DiskUsageChecker::new(
            settings(true, 75.0, 90.0),
            FakeDiskSource {
                snapshots: vec![
                    snap("/", 40.0),
                    snap("/home", 80.0), // warning
                    snap("/var", 95.0),  // critical
                ],
                fail: false,
            },
        );

        let res = checker.check().unwrap();
        assert_eq!(res.result, CheckStatus::CRITICAL);
        let descr = res.descr.unwrap();
        assert!(descr.contains("Disk usage for /var is 95.0%"));
        assert!(!descr.contains("/home"));
        assert!(!descr.contains(" / is 40.0%"));
    }

    #[test]
    fn filter_by_level_respects_thresholds_and_order() {
        let checker = DiskUsageChecker::new(settings(true, 70.0, 90.0), FakeDiskSource::default());

        let data = vec![
            snap("/", 69.9),     // below warning
            snap("/home", 70.0), // equal warning -> NOT warning (strict >)
            snap("/opt", 75.0),  // warning
            snap("/var", 90.0),  // equal critical -> NOT critical (strict >)
            snap("/data", 91.2), // critical
        ];

        let warns = checker.filter_by_level(&data, CheckLevel::WARNING);
        assert_eq!(warns.len(), 3);
        assert_eq!(warns[0].mount, "/opt");

        let crits = checker.filter_by_level(&data, CheckLevel::CRITICAL);
        assert_eq!(crits.len(), 1);
        assert_eq!(crits[0].mount, "/data");
    }

    #[test]
    fn compile_descr_formats_single_and_multiple() {
        let one = vec![snap("/one", 88.345)];
        let many = vec![snap("/a", 70.0), snap("/b", 92.2)];

        let s1 = DiskUsageChecker::<FakeDiskSource>::compile_descr(&one);
        assert_eq!(s1, "Disk usage for /one is 88.3%");

        let s2 = DiskUsageChecker::<FakeDiskSource>::compile_descr(&many);
        assert!(s2.contains("Disk usage for /a is 70.0%"));
        assert!(s2.contains("Disk usage for /b is 92.2%"));
        assert!(s2.contains("; "));
    }

    #[test]
    fn error_is_propagated() {
        let checker = DiskUsageChecker::new(
            settings(true, 75.0, 90.0),
            FakeDiskSource {
                snapshots: vec![],
                fail: true,
            },
        );

        let res = checker.check();
        assert!(res.is_err());
        let msg = format!("{:?}", res.err().unwrap());
        assert!(msg.contains("boom"));
    }
}
