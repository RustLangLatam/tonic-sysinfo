use std::{borrow::BorrowMut, ops::Deref, pin::Pin, time::Duration};

use futures_util::Stream;
use sysinfo::{Disks, System};
use tokio::time;
use tonic::{Request, Response, Status};

use crate::pb::{
    system_info_service_server::{SystemInfoService, SystemInfoServiceServer},
    MemoryInfo, SystemInfoRequest, SystemInfoResponse,
};

/// A service providing implementations of gRPC system information.
#[derive(Debug)]
pub struct SysInfoContext {}

impl SysInfoContext {
    /// Creates a new instance of the `SystemInfoServiceServer`.
    pub fn service() -> SystemInfoServiceServer<SysInfoContext> {
        SystemInfoServiceServer::new(SysInfoContext {})
    }

    /// Retrieves system information based on the provided request.
    ///
    /// This function takes a `SystemInfoRequest` and a mutable reference to a `System` object.
    /// It returns a `SystemInfoResponse` containing the requested system information.
    fn get_system_info_values(
        request: SystemInfoRequest,
        system: &mut System,
    ) -> Result<SystemInfoResponse, Status> {
        // Initialize the response with the system information.
        let mut response = SystemInfoResponse::from(system.deref());

        // If memory information is requested, refresh the system's memory information and add it to the response.
        if request.include_memory_info {
            system.refresh_memory();
            response.memory_info = Some(MemoryInfo::from(system.deref()));
        }

        // If CPU information is requested, refresh the system's CPU information and add it to the response.
        if request.include_cpu_info {
            system.refresh_cpu_all();
            let cpu_info = system.cpus().iter().map(Into::into).collect::<Vec<_>>();
            response.cpu_info = cpu_info;
        }

        // If disk information is requested, refresh the system's disk information and add it to the response.
        if request.include_disk_info {
            let disks = Disks::new_with_refreshed_list();
            let disk_info = disks.iter().map(Into::into).collect::<Vec<_>>();
            response.disk_info = disk_info;
        }

        Ok(response)
    }
}

#[tonic::async_trait]
impl SystemInfoService for SysInfoContext {
    /// Retrieves system information based on the provided request.
    ///
    /// This function takes a `Request` containing a `SystemInfoRequest` and returns a `Response` containing a `SystemInfoResponse`.
    async fn get_system_info(
        &self,
        request: Request<SystemInfoRequest>,
    ) -> Result<Response<SystemInfoResponse>, Status> {
        // Extract the request data from the request.
        let request_data = request.into_inner();

        // Create a new system object.
        let mut system = System::new();

        // Retrieve the system information based on the request.
        let response = Self::get_system_info_values(request_data, system.borrow_mut())?;

        // Return the response.
        Ok(Response::new(response))
    }

    /// Type alias for the watch system info stream.
    type WatchSystemInfoStream =
    Pin<Box<dyn Stream<Item = Result<SystemInfoResponse, Status>> + Send + 'static>>;

    /// Watches system information based on the provided request.
    ///
    /// This function takes a `Request` containing a `SystemInfoRequest` and returns a `Response` containing a stream of `SystemInfoResponse` objects.
    async fn watch_system_info(
        &self,
        request: Request<SystemInfoRequest>,
    ) -> Result<Response<Self::WatchSystemInfoStream>, Status> {
        // Extract the request data from the request.
        let request_data = request.into_inner();

        // Determine the interval at which to refresh the system information.
        let interval = if request_data.refresh_interval == 0 {
            Duration::from_secs(1)
        } else {
            Duration::from_secs(request_data.refresh_interval as u64)
        };

        // Create a stream that yields system information at the specified interval.
        let output = async_stream::try_stream! {
            // Create a new system object.
            let mut system = System::new();

            // Yield the initial system information.
            yield Self::get_system_info_values(request_data.clone(), system.borrow_mut())?;

            // Create an interval stream that ticks at the specified interval.
            let mut interval_stream = time::interval(interval);

            // Loop indefinitely, yielding system information at each interval.
            loop {
                let _ = interval_stream.tick().await;
                yield Self::get_system_info_values(request_data.clone(), system.borrow_mut())?;
            }
        };

        // Return the response containing the stream.
        Ok(Response::new(
            Box::pin(output) as Self::WatchSystemInfoStream
        ))
    }
}

#[cfg(test)]
mod tests {
    use tonic::Request;

    use crate::pb::system_info_service_server::SystemInfoService;
    use crate::pb::SystemInfoRequest;
    use crate::server::SysInfoContext;

    async fn make_test_service() -> SysInfoContext {
        let health_service = SysInfoContext {};
        health_service
    }

    #[tokio::test]
    async fn test_service_check() {
        let service = make_test_service().await;

        // Overall server health
        let resp = service
            .get_system_info(Request::new(SystemInfoRequest {
                include_disk_info: true,
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
            .watch_system_info(Request::new(SystemInfoRequest {
                include_disk_info: true,
                ..Default::default()
            }))
            .await;
        assert!(response.is_ok());
    }
}
