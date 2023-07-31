use crate::pb::CpuInfo;
use sysinfo::{Cpu, CpuExt};

impl From<&Cpu> for CpuInfo {
    fn from(value: &Cpu) -> Self {
        Self {
            cpu_name: value.name().to_owned(),
            cpu_usage: value.cpu_usage(),
            frequency: value.frequency(),
            vendor_id: value.vendor_id().to_owned(),
            brand: value.brand().to_owned(),
        }
    }
}
