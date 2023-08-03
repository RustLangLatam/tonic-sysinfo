use crate::pb::sys_info_service_server::{SysInfoService, SysInfoServiceServer};
use crate::pb::{
    sys_info_check_response::System as PbSystem, MemInfo, SysInfoCheckRequest, SysInfoCheckResponse,
};
use std::borrow::BorrowMut;
use std::ops::Deref;
use std::pin::Pin;
use std::time::Duration;
use sysinfo::{System, SystemExt};
use tokio::time;
use tonic::codegen::futures_core::Stream;
use tonic::{Request, Response, Status};

/// A service providing implementations of gRPC system information.
#[derive(Debug)]
pub struct SysInfoContext {}

impl SysInfoContext {
    pub fn service() -> SysInfoServiceServer<SysInfoContext> {
        SysInfoServiceServer::new(SysInfoContext {})
    }

    fn get_values(data_inner: SysInfoCheckRequest, sys: &mut System) -> Result<PbSystem, Status> {
        let mut info_response = PbSystem::from(sys.deref());

        if data_inner.mem_info {
            sys.refresh_memory();
            info_response.mem_info = Some(MemInfo::from(sys.deref()))
        };
        if data_inner.cpu_info {
            sys.refresh_cpu();
            let cpu_info_vec = sys.cpus().iter().map(Into::into).collect::<Vec<_>>();
            info_response.cpu_info = cpu_info_vec
        }

        if data_inner.disk_info {
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
        let mut sys = System::new();

        let info = Self::get_values(data_inner, sys.borrow_mut()).unwrap();

        Ok(Response::new(SysInfoCheckResponse { info: Some(info) }))
    }

    type WatchStream =
        Pin<Box<dyn Stream<Item = Result<SysInfoCheckResponse, Status>> + Send + 'static>>;

    async fn watch(
        &self,
        request: Request<SysInfoCheckRequest>,
    ) -> Result<Response<Self::WatchStream>, Status> {
        let data_inner = request.into_inner();

        let interval_parm = if data_inner.interval == 0 {
            1
        } else {
            data_inner.interval
        } as u64;

        let interval_sec = Duration::from_secs(interval_parm);

        let output = async_stream::try_stream! {

            let mut sys = System::new();
            let info = Self::get_values(data_inner.clone(), sys.borrow_mut()).unwrap();

            yield SysInfoCheckResponse { info: Some(info) };

            let mut interval = time::interval(interval_sec);

            loop {
                // sleep not to overload our system.
                let _tick = interval.tick().await;

                let info = Self::get_values(data_inner.clone(), sys.borrow_mut()).unwrap();

                yield SysInfoCheckResponse { info: Some(info) };

                // std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
            }

        };

        Ok(Response::new(Box::pin(output) as Self::WatchStream))
    }
}

#[cfg(test)]
mod tests {
    use crate::pb::sys_info_service_server::SysInfoService;
    use crate::pb::SysInfoCheckRequest;
    use crate::server::SysInfoContext;
    use tonic::Request;

    async fn make_test_service() -> SysInfoContext {
        let health_service = SysInfoContext {};
        health_service
    }

    #[tokio::test]
    async fn test_service_check() {
        let service = make_test_service().await;

        // Overall server health
        let resp = service
            .check(Request::new(SysInfoCheckRequest {
                disk_info: true,
                ..Default::default()
            }))
            .await;
        assert!(resp.is_ok());
    }

    #[tokio::test]
    async fn test_service_watch() {
        let service = make_test_service().await;

        // Overall server health
        let response = service
            .watch(Request::new(SysInfoCheckRequest {
                disk_info: true,
                ..Default::default()
            }))
            .await;
        assert!(response.is_ok());
    }
}
