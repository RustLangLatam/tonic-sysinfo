use crate::pb::sys_info_service_server::SysInfoService;
use crate::pb::{
    sys_info_check_request, sys_info_check_response::System as PbSystem, MemInfo,
    SysInfoCheckRequest, SysInfoCheckResponse,
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
    fn get_values(data_inner: SysInfoCheckRequest, sys: &mut System) -> Result<PbSystem, Status> {
        let mut info_response = PbSystem::from(sys.deref());

        if data_inner
            .info_type
            .contains(&sys_info_check_request::InfoType::MemInfo.into())
        {
            sys.refresh_memory();
            info_response.mem_info = Some(MemInfo::from(sys.deref()))
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

        let output = async_stream::try_stream! {

            let mut sys = System::new();
            let info = Self::get_values(data_inner.clone(), sys.borrow_mut()).unwrap();

            yield SysInfoCheckResponse { info: Some(info) };

            let mut interval = time::interval(Duration::from_secs(1));

            loop {
                // sleep not to overload our system.
                let _tick = interval.tick().await;

                let info = Self::get_values(data_inner.clone(), sys.borrow_mut()).unwrap();

                yield SysInfoCheckResponse { info: Some(info) };

            }

        };

        Ok(Response::new(Box::pin(output) as Self::WatchStream))
    }
}

#[cfg(test)]
mod tests {
    use crate::pb::sys_info_service_server::SysInfoService;
    use crate::pb::{sys_info_check_request, SysInfoCheckRequest};
    use crate::server::SysInfoContext;
    use tokio_stream::StreamExt;
    use tonic::Request;

    async fn make_test_service() -> SysInfoContext {
        let health_service = SysInfoContext {};
        health_service
    }

    #[tokio::test]
    async fn test_service_check() {
        let service = make_test_service().await;
        println!("#######");

        // Overall server health
        let resp = service
            .check(Request::new(SysInfoCheckRequest {
                info_type: vec![sys_info_check_request::InfoType::DiskInfo.into()],
            }))
            .await;
        assert!(resp.is_ok());

        println!("{:#?}", resp.unwrap());
    }

    #[tokio::test]
    async fn test_service_watch() {
        let service = make_test_service().await;
        println!("#######");

        // Overall server health
        let response = service
            .watch(Request::new(SysInfoCheckRequest {
                info_type: vec![sys_info_check_request::InfoType::DiskInfo.into()],
            }))
            .await
            .unwrap();
        // assert!(response.is_ok());

        let mut response_stream = response.into_inner();

        while let Some(response) = response_stream.next().await {
            println!("####### {:?}", response)
        }
    }
}
