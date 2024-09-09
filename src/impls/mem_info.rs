use crate::pb::MemoryInfo;

impl From<&sysinfo::System> for MemoryInfo {
    fn from(value: &sysinfo::System) -> Self {
        Self {
            total: value.total_memory() as u32,
            free: value.free_memory() as u32,
            swap_total: value.total_swap() as u32,
            swap_free: value.free_swap() as u32,
            available: value.available_memory() as u32,
        }
    }
}
