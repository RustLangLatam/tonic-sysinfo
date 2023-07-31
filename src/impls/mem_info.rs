use sysinfo::SystemExt;
use crate::pb::MemInfo;

impl From<&sysinfo::System> for MemInfo {
    fn from(value: &sysinfo::System) -> Self {
        Self {
            mem_total: value.total_memory(),
            mem_free: value.free_memory(),
            mem_available: value.available_memory(),
            swap_total: value.total_swap(),
            swap_free: value.free_swap(),
        }
    }
}