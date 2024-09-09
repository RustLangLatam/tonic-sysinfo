use sysinfo::Cpu;

use crate::pb::CpuInfo;

impl From<&Cpu> for CpuInfo {
    fn from(value: &Cpu) -> Self {
        Self {
            name: value.name().to_owned(),
            frequency: value.frequency(),
            vendor_id: value.vendor_id().to_owned(),
            brand: value.brand().to_owned(),
            usage: value.cpu_usage(),
        }
    }
}
