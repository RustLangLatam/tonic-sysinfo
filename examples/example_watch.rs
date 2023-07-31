use tokio_stream::StreamExt;
use tonic::Request;
use tonic_sysinfo::pb::{sys_info_check_request, SysInfoCheckRequest};
use tonic_sysinfo::pb::sys_info_service_server::SysInfoService;
use tonic_sysinfo::server::SysInfoContext;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let service = SysInfoContext{};

    // Overall server health
    let response = service
        .watch(Request::new(SysInfoCheckRequest {
            info_type: vec![sys_info_check_request::InfoType::MemInfo.into()],
        }))
        .await.unwrap();
    // assert!(response.is_ok());

    let mut response_stream = response.into_inner();

    while let Some(response) = response_stream.next().await {
        match response {
            Ok(info) => {
                let info = info.info.unwrap();

                if let Some(ref mem_info) = info.mem_info {
                    println!("Total Memory: {}", mem_info.mem_total);
                    println!("Free Memory: {}", mem_info.mem_free);
                    println!("Available Memory: {}", mem_info.mem_available);
                    println!("Total Swap Space: {}", mem_info.swap_total);
                    println!("Free Swap Space: {}", mem_info.swap_free);

                } else {
                    println!("Memory information not available.");
                }
                println!("\n");

                if !info.cpu_info.is_empty() {
                    for cpu_info in info.cpu_info {
                        println!("CPU Name: {}", cpu_info.cpu_name);
                        println!("CPU Usage: {}", cpu_info.cpu_usage);
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
                        println!("Total Space: {} GB", disk_info.total_space as f64 / 1_000_000_000.0);
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