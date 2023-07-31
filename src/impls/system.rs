use sysinfo::SystemExt;
use crate::pb::sys_info_check_response;

impl From<&sysinfo::System> for sys_info_check_response::System {
    fn from(value: &sysinfo::System) -> Self {
        Self {
            mem_info: Some(value.into()),
            cpu_info: value.cpus().iter().map(Into::into).collect::<Vec<_>>(),
            disk_info: value.disks().iter().map(Into::into).collect::<Vec<_>>(),
        }
    }
}