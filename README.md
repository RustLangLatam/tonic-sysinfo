Aquí tienes un archivo `README.md` que puedes usar para documentar tu servicio gRPC de monitoreo del sistema usando Tonic en Rust:

---

# System Info Service

This project provides a basic gRPC service built using Tonic in Rust to monitor system resources such as memory, CPU, and disk usage. The service offers two endpoints:

1. `get_system_info`: Retrieves system information upon request.
2. `watch_system_info`: Provides real-time monitoring of system information, streaming updates at regular intervals.

## Prerequisites

- Rust (>=1.80.0)
- `Tonic` crate for building gRPC services
- `sysinfo` crate to retrieve system information
- `tokio` for asynchronous programming
- `async-stream` for streaming responses

### Dependencies

Ensure you include the following in your `Cargo.toml`:

```toml
[dependencies]
tonic = "0.6"
tokio = { version = "1", features = ["full"] }
futures-util = "0.3"
sysinfo = "0.22"
async-stream = "0.3"
```

## System Info Service

The `SysInfoContext` struct implements the gRPC service using Tonic. It provides two key endpoints for monitoring system resources.

### `get_system_info`

This endpoint allows a client to request system information (memory, CPU, disk usage). You can specify which information to include through the `SystemInfoRequest`.

- **Parameters**:
    - `include_memory_info`: Boolean flag to include memory information.
    - `include_cpu_info`: Boolean flag to include CPU information.
    - `include_disk_info`: Boolean flag to include disk information.

- **Response**: A `SystemInfoResponse` that contains the requested data.

### `watch_system_info`

This endpoint streams system information at regular intervals, allowing real-time monitoring of system resources. The client can specify the refresh interval in the request.

- **Parameters**:
    - `include_memory_info`: Boolean flag to include memory information.
    - `include_cpu_info`: Boolean flag to include CPU information.
    - `include_disk_info`: Boolean flag to include disk information.
    - `refresh_interval`: Interval in seconds to refresh the information stream.

- **Response**: A stream of `SystemInfoResponse` objects, updated at the specified interval.

## Usage

### Step 1: Set Up the Server

You can start the gRPC server using the `SysInfoContext::service()` method. This sets up the service to listen for incoming gRPC requests.

```rust
use tonic::transport::Server;
use sysinfo_service::SysInfoContext;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let sys_info_service = SysInfoContext::service();

    println!("SystemInfoService listening on {}", addr);

    Server::builder()
        .add_service(sys_info_service)
        .serve(addr)
        .await?;

    Ok(())
}
```

### Step 2: Client Request

To make requests, you can use a gRPC client or any tool that can communicate via gRPC (such as `grpcurl` or your own Rust client). Below is an example request using a Rust client:

```rust
use tonic::transport::Channel;
use sysinfo_service::system_info_service_client::SystemInfoServiceClient;
use sysinfo_service::SystemInfoRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SystemInfoServiceClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(SystemInfoRequest {
        include_memory_info: true,
        include_cpu_info: true,
        include_disk_info: true,
        refresh_interval: 5,  // for watch_system_info
    });

    // For get_system_info
    let response = client.get_system_info(request.clone()).await?;
    println!("System Info: {:?}", response.into_inner());

    // For watch_system_info (Streaming)
    let mut stream = client.watch_system_info(request).await?.into_inner();
    while let Some(system_info) = stream.message().await? {
        println!("System Info Stream: {:?}", system_info);
    }

    Ok(())
}
```

## License

This project is licensed under the APACHE License.

---