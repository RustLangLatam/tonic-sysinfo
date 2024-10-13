use tokio_stream::StreamExt;
use tonic::Request;
use tonic_sysinfo::pb::system_info_service_server::SystemInfoService;
use tonic_sysinfo::pb::SystemInfoRequest;
use tonic_sysinfo::SysInfoContext;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = SysInfoContext {};

    // Overall server health
    let response = service
        .watch_system_info(Request::new(SystemInfoRequest {
            include_memory_info: false,
            include_disk_info: true,
            include_cpu_info: false,
            refresh_interval: 10,
        }))
        .await
        .unwrap();
    // assert!(response.is_ok());

    let mut response_stream = response.into_inner();

    while let Some(response) = response_stream.next().await {
        match response {
            Ok(info) => {
                if let Some(ref mem_info) = info.memory_info {
                    println!("Total Memory: {}", mem_info.total);
                    println!("Free Memory: {}", mem_info.free);
                    println!("Available Memory: {}", mem_info.available);
                    println!("Total Swap Space: {}", mem_info.swap_total);
                    println!("Free Swap Space: {}", mem_info.swap_free);
                } else {
                    println!("Memory information not available.");
                }
                println!("\n");

                if !info.cpu_info.is_empty() {
                    for cpu_info in info.cpu_info {
                        println!("CPU Name: {}", cpu_info.name);
                        println!("CPU Usage: {}", cpu_info.usage);
                        println!("Frequency: {}", cpu_info.frequency);
                        println!("Vendor ID: {}", cpu_info.vendor_id);
                        println!("Brand: {}", cpu_info.brand);
                        println!("--------------------------------");
                    }
                } else {
                    println!("CPU information not available.");
                }
                println!("\n");

                if !info.disk_info.is_empty() {
                    for disk_info in info.disk_info {
                        println!("Disk Name: {}", disk_info.name);
                        println!("File System: {}", disk_info.file_system);
                        println!("Mount Point: {}", disk_info.mount_point);
                        println!(
                            "Total Space: {} GB",
                            disk_info.total_space as f64 / 1_000_000_000.0
                        );
                        println!(
                            "Available Space: {} GB",
                            disk_info.available_space as f64 / 1_000_000_000.0
                        );
                        println!("--------------------------------");
                    }
                } else {
                    println!("Disk information not available.");
                }
                println!("\n");
            }
            Err(e) => println!("Error in the stream: {:?}", e),
        }
    }

    Ok(())
}
