// SPDX-License-Identifier: GPL-2.0

use kernel::{
    kasync::executor::{workqueue::Executor as WqExecutor, AutoStopHandle, Executor},
    kasync::net::{TcpListener, TcpStream},
    net::{self, Ipv4Addr, SocketAddr, SocketAddrV4},
    prelude::*,
    spawn_task,
    sync::{Arc, ArcBorrow},
    eth_canfd_payload ::* ,
    //rust_gateway ::* ,
    delay::coarse_sleep ,

};
use core::time::Duration;
use core::*;
use kernel::net::*;

async fn echo_server(stream: TcpStream) -> Result {
    let mut buf = [0u8; 52];
    loop {
        let n = stream.read(&mut buf).await?;
        pr_info!("RECEIVING DATA FROM THE GATEWAY MODULE : MODE CANFD->ETHERNET") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("DESERIALIZING THE CONVERTED ETHERNET FRAME") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        let ethernet = EthCanfdpayLoad:: deserialize_eth_payload(&buf).unwrap();
        pr_info!("ETHERNET FRAME INFORMATION : ");
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("  Destination MAC: {:?}", ethernet.data.iphdr.dst);
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("  Source MAC: {:?}", ethernet.data.iphdr.src);
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("  Ethertype: {:?}", ethernet.ethertype.0);
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("\n  - CANFD Payload:");
        coarse_sleep(Duration::from_millis(500));  
        pr_info!("- IP Header:");
        coarse_sleep(Duration::from_millis(500));  
        pr_info!("- Version: {:?}", ethernet.data.iphdr.version);
        coarse_sleep(Duration::from_millis(500));
        pr_info!("- Source IP: {}.{}.{}.{}",
        ethernet.data.iphdr.src[0], ethernet.data.iphdr.src[1], ethernet.data.iphdr.src[2], ethernet.data.iphdr.src[3]);
        pr_info!("- Destination IP: {}.{}.{}.{}",
        ethernet.data.iphdr.dst[0], ethernet.data.iphdr.dst[1],ethernet.data.iphdr.dst[2], ethernet.data.iphdr.dst[3]);  
        pr_info!("- TCP Header:");
        coarse_sleep(Duration::from_millis(500));  
        pr_info!("- Source Port: 0x{:0x}", ethernet.data.tcphdr.src_port);
        coarse_sleep(Duration::from_millis(500));  
        pr_info!("- Destination Port: 0x{:0x}", ethernet.data.tcphdr.dst_port);
        coarse_sleep(Duration::from_millis(500));  
        pr_info!("- Sequence Number: {}", ethernet.data.tcphdr.seq);
        coarse_sleep(Duration::from_millis(500));  
        pr_info!("- Acknowledgment Number: {}", ethernet.data.tcphdr.ack);
        coarse_sleep(Duration::from_millis(500));  

    // Improved output for TCP flags:
        pr_info!("- Flags: {:?}", ethernet.data.tcphdr.flags); // Use Debug trait for detailed flag information
        coarse_sleep(Duration::from_millis(500)); 
    // Additional TCP header fields (consider including only relevant ones):
        pr_info!("- Data Offset: {}", ethernet.data.tcphdr.offset);
        coarse_sleep(Duration::from_millis(500)); 
        pr_info!("- Window: {}", ethernet.data.tcphdr.window);
        coarse_sleep(Duration::from_millis(500)); 
        pr_info!("- Checksum: {:x}", ethernet.data.tcphdr.checksum); // Hexadecimal format for checksum
        coarse_sleep(Duration::from_millis(500)); 
        pr_info!("- Payload");
        for i in 0..5 {
            if i < ethernet.data.data.len() {
            pr_info!("  - Byte {}: {:02X}", i, ethernet.data.data[i]);
            coarse_sleep(Duration::from_millis(200)); 
            }   else {
            pr_info!("  - Byte {}: (no data)", i);
            }
        } 
        pr_info!("--------------------------------------------------------------") ; 
        
    return Ok(());







        /*if n == 0 {
            return Ok(());
        }

        let remote_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::ANY, 8000)) ;
        let stream1 = connect(&remote_addr)?;
        // Example number to send 
        let buf1=buf.try_to_vec()?; 
        send_data(&stream1, buf1)? ; 

        stream.write_all(&buf[..n]).await?;*/
        
       


    }
}








async fn accept_loop(listener: TcpListener, executor: Arc<impl Executor>) {
    loop {
        if let Ok(stream) = listener.accept().await {
            let _ = spawn_task!(executor.as_arc_borrow(), echo_server(stream));
        }
    }
}
fn start_listener(ex: ArcBorrow<'_, impl Executor + Send + Sync + 'static>) -> Result {
    let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::ANY, 8080));
    let listener = TcpListener::try_new(net::init_ns(), &addr)?;
    pr_info!("LIstening") ;
    spawn_task!(ex, accept_loop(listener, ex.into()))?;
    Ok(())
}

struct RustEchoServer {
    _handle: AutoStopHandle<dyn Executor>,
}

impl kernel::Module for RustEchoServer {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        let handle = WqExecutor::try_new(kernel::workqueue::system())?;
        pr_info!("Listening") ; 
        start_listener(handle.executor())?;
        Ok(Self {
            _handle: handle.into(),
        })
    }
}

fn array_to_vec(arr: &[u8; 52]) -> Vec<u8> {
    let mut vec = Vec::new();
    for &item in arr.iter() {
        vec.try_push(item);
    }
    vec
}

module! {
    type: RustEchoServer,
    name: "RUST_ETHERNET_DEVICE",
    author: "Rust for Linux Contributors",
    description: "Rust tcp echo sample",
    license: "GPL v2",
}
