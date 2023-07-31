use crate::pb::DiskInfo;
use sysinfo::{Disk, DiskExt};

impl From<&Disk> for DiskInfo {
    fn from(value: &Disk) -> Self {
        Self {
            name: String::from(value.name().to_str().unwrap()),
            file_system: String::from_utf8(value.file_system().to_vec()).unwrap(),
            mount_point: value.mount_point().to_string_lossy().to_string(),
            total_space: value.total_space(),
            available_space: value.available_space(),
        }
    }
}

// Function to format space in GB
fn format_space_in_gb(space: u64) -> f64 {
    space as f64 / (1024.0 * 1024.0 * 1024.0)
}