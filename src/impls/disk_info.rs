use sysinfo::Disk;

use crate::pb::DiskInfo;

impl From<&Disk> for DiskInfo {
    fn from(value: &Disk) -> Self {
        Self {
            name: String::from(value.name().to_str().unwrap()),
            file_system: value.file_system().to_str().unwrap_or_default().to_string(),
            mount_point: value.mount_point().to_string_lossy().to_string(),
            total_space: value.total_space() as u32,
            available_space: value.available_space() as u32,
        }
    }
}
