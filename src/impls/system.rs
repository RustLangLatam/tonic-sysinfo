use crate::pb::SystemInfoResponse;

impl From<&sysinfo::System> for SystemInfoResponse {
    fn from(value: &sysinfo::System) -> Self {
        Self {
            memory_info: Some(value.into()),
            cpu_info: value.cpus().iter().map(Into::into).collect::<Vec<_>>(),
            disk_info: vec![],
        }
    }
}
