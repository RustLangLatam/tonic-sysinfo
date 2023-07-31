use crate::pb::sys_info_service_server::SysInfoService;
use crate::pb::{
    sys_info_check_request, sys_info_check_response::System as PbSystem, CpuInfo, MemInfo,
    SysInfoCheckRequest, SysInfoCheckResponse,
};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use sysinfo::{CpuExt, System, SystemExt};
use tonic::codegen::futures_core::Stream;
use tonic::{Request, Response, Status};

/// A service providing implementations of gRPC system information.
#[derive(Debug)]
pub struct SysInfoContext {
    sys: Arc<Mutex<System>>,
}

impl SysInfoContext {
    fn new() -> Self {
        // Please note that we use "new_all" to ensure that all list of
        // components, network interfaces, disks and users are already
        // filled!
        let sys = Arc::new(Mutex::new(System::new_all()));
        SysInfoContext { sys }
    }

    // fn refresh_system(&mut self) {
    //     self.sys.refresh_system()
    // }

    fn get_values(&self, data_inner: SysInfoCheckRequest) -> Result<PbSystem, Status> {
        let mut sys = self
            .sys
            .lock()
            .map_err(|_| Status::internal("Failed to acquire lock"))?;

        // First we update all information of our `System` struct.

        let mut info_response = PbSystem::from(&(*sys));

        if data_inner
            .info_type
            .contains(&sys_info_check_request::InfoType::MemInfo.into())
        {
            sys.refresh_memory();
            info_response.mem_info = Some(MemInfo::from(&(*sys)))
        };
        if data_inner
            .info_type
            .contains(&sys_info_check_request::InfoType::CpuInfo.into())
        {
            sys.refresh_cpu();
            let cpu_info_vec = sys.cpus().iter().map(Into::into).collect::<Vec<_>>();
            info_response.cpu_info = cpu_info_vec
        }

        if data_inner
            .info_type
            .contains(&sys_info_check_request::InfoType::DiskInfo.into())
        {
            sys.refresh_disks_list();
            let disk_info = sys.disks().iter().map(Into::into).collect::<Vec<_>>();
            info_response.disk_info = disk_info
        }
        Ok(info_response)
    }
}

#[tonic::async_trait]
impl SysInfoService for SysInfoContext {
    async fn check(
        &self,
        request: Request<SysInfoCheckRequest>,
    ) -> Result<Response<SysInfoCheckResponse>, Status> {
        let data_inner = request.into_inner();
        // Acquire the lock before mutating the state

        let info_response = self.get_values(data_inner)?;

        Ok(Response::new(SysInfoCheckResponse {
            info: Some(info_response),
        }))
    }

    type WatchStream =
        Pin<Box<dyn Stream<Item = Result<SysInfoCheckResponse, Status>> + Send + 'static>>;

    async fn watch(
        &self,
        request: Request<SysInfoCheckRequest>,
    ) -> Result<Response<Self::WatchStream>, Status> {
        let data_inner = request.into_inner();
        let info_system = self.get_values(data_inner).unwrap();

        let output = async_stream::try_stream! {

            // yield the current value
            // let status = crate::pb::sys_info_check_response::ServingStatus::from(*status_rx.borrow()) as i32;
            yield SysInfoCheckResponse { info: Some(info_system) };

            // #[allow(clippy::redundant_pattern_matching)]
            // while let Ok(_) = status_rx.changed().await {
            //     // let status = crate::pb::sys_info_check_response::ServingStatus::from(*status_rx.borrow()) as i32;
            //     yield SysInfoCheckResponse { status };
            // }
        };

        Ok(Response::new(Box::pin(output) as Self::WatchStream))
    }
}

#[cfg(test)]
mod tests {
    use crate::pb::sys_info_service_server::SysInfoService;
    use crate::pb::{sys_info_check_request, SysInfoCheckRequest};
    use crate::server::SysInfoContext;
    use tonic::Request;

    async fn make_test_service() -> SysInfoContext {
        let health_service = SysInfoContext::new();
        health_service
    }

    #[tokio::test]
    async fn test_service_check() {
        let service = make_test_service().await;

        // Overall server health
        let resp = service
            .check(Request::new(SysInfoCheckRequest {
                info_type: vec![
                    sys_info_check_request::InfoType::DiskInfo.into()
                ],
            }))
            .await;
        assert!(resp.is_ok());
    }
}
