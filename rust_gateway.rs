// SPDX-License-Identifier: GPL-2.0

//! Rust echo server sample.

use kernel::{
    kasync::executor::{workqueue::Executor as WqExecutor, AutoStopHandle, Executor},
    kasync::net::{TcpListener, TcpStream},
    net::{self, Ipv4Addr, SocketAddr, SocketAddrV4},
    prelude::*,
    spawn_task,
    sync::{Arc, ArcBorrow},
    eth_canfd_payload ::* ,
    //eth_can_payload ::* ,
    delay::coarse_sleep ,
    
};
use core::time::Duration;
use kernel::net::*;


async fn echo_server(stream: TcpStream) -> Result {
    let mut buff = [0u8; 52 ];
    loop {
        let n = stream.read(&mut buff).await?;
        pr_info!("RECEIVING CANFD FRAME FROM THE NETLINK CLIENT ! ") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        if n == 0 {
            return Ok(());
        }

        pr_info!("-------------------------------") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("start the conversion from CANFD to Ethernet \n");
        coarse_sleep(Duration::from_secs(1)) ;
        //pr_info!("buffer is {:?}", buff );
        //coarse_sleep(Duration::from_secs(1)) ;
        //pr_info!("-------------------------------") ; 
        //coarse_sleep(Duration::from_secs(1)) ;
        let canfd = canfdFrame ::deserialize_canfd_payload(&buff).unwrap();
        pr_info!("DONE DESERIALIZING THE CANFD FRAME ")  ; 
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("PREPARING FOR A CONVERSION ") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        let payload_data = EthCanfdpayLoad::to_eth_frame(&canfd) ; 
        pr_info!("DONE THE CONVERSION FROM CANFD FRAME INTO ETHERNET FRAME ") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        //pr_info!("-------------------------------") ; 
        //coarse_sleep(Duration::from_secs(1)) ;
        //pr_info!("-------------------------------") ; 
        //coarse_sleep(Duration::from_secs(1)) ;
       // pr_info!("-------------------------------") ;
        pr_info!("PREPARING TO SEND TO THE ETHERNET DEVICE ") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        let remote_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::ANY, 7070)) ;
        pr_info!("---INFO--- CREATED GATEWAY SOCKET SUCCCESSFULLY !") ;
        coarse_sleep(Duration::from_secs(1)) ; 
        let stream1 = connect(&remote_addr)?;
        // Example number to send 
        let buf1=serialize_eth_canfd_payload(&payload_data) ; 
        pr_info!("len  {:}", buf1.len());

        pr_info!("DONE SERIALIZING THE ETHERNET FRAME , SENDING TO THE ETHERNET DEVICE") ;
        coarse_sleep(Duration::from_secs(1)) ;  
        send_data(&stream1, buf1)? ; //test
        pr_info!("---INFO--- SEND DATA FUNCTION IS BEING CALLED !") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("---INFO--- SENDING TO THE ETHERNET DEVICE ! ") ;
        coarse_sleep(Duration::from_secs(1)) ; 

        /*pr_info!("-- Eth Canfd Frame --");
        pr_info!("  Destination MAC: {:?}", payload.dst_mac);
        pr_info!("  Source MAC: {:?}", payload.src_mac);
        pr_info!("  Ethertype: {:?}", payload.ethertype.0);
        pr_info!("-------------------------------") ; 
        pr_info!("-------------------------------") ; 
        pr_info!("start receinving CAN frame \n");
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("buffer is {:?}", buff );
        coarse_sleep(Duration::from_secs(1)) ;*/

  
        /*let mock = EthCanfdpayLoad {
            dst_mac: [0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE],
            src_mac: [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB],
            ethertype: EtherTypefd::IPV4,
            data: Ethloadfd {
                iphdr: Ipv4Header {
                    version: 4,
                    len: 20, // Assuming minimum header length
                    to_s: 1, // ToS (Type of Service) field example
                    total_len: 50, // Assuming total length (including data)
                    id: 100, // Example ID
                    flags: 0, // Flags field example (no flags set)
                    frag_offset: 0, // Fragment offset (no fragmentation)
                    ttl: 56, // Time to live example
                    protocol: 6, // TCP protocol
                    checksum: 0xABCD, // Mock checksum value
                    src: [192, 168, 1, 10],
                    dst: [10, 0, 0, 1],
                },
                tcphdr: TcpHeader {
                    src_port: 8080,
                    dst_port: 4433,
                    seq: 1234567890,
                    ack: 987654321,
                    offset: 5, // Assuming header length (5 words)
                    reserved: 0, // Reserved bits (set to 0)
                    flags: 0, // Flags field example (no flags set)
                    window: 1024, // Example window size
                    checksum: 0x1234, // Mock checksum value
                    urgent_ptr: 0, // Urgent pointer (not used here)
                },
                data: [0xCA, 0xFE, 0xBA, 0xBE], // Example data
            }
        } ; */
    
        /*pr_info!("-- Eth Canfd Frame --");
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("  Destination MAC: {:?}", payload.dst);
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("  Source MAC: {:?}", payload.src);
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("  Ethertype: {:?}", payload.ethertype.0);
        coarse_sleep(Duration::from_secs(1)) ;

        // Print IP Header details
        pr_info!("-- IP Header --");
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("  Version: {}", payload.data.header.version);
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("  Header Length: {} bytes", payload.data.header.len * 4);
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("  Type of Service: {}", payload.data.header.to_s);
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("  Total Length: {} bytes", payload.data.header.total_len);
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("  Identification: {}", payload.data.header.id);
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("  Flags: {:x}", payload.data.header.flags);
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("  Fragment Offset: {} bytes", payload.data.header.frag_offset & 0x1f);
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("  Time to Live: {} hops", payload.data.header.ttl);
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("-------------------------------") ; 
         // Print TCP  Header details
         pr_info!("-- TCP Header --");
         coarse_sleep(Duration::from_secs(1)) ;
         pr_info!("  Source port: {}", payload.data.datatcp.tcphdr.src_port);
         coarse_sleep(Duration::from_secs(1)) ;
         pr_info!("  Destination port: {}", payload.data.datatcp.tcphdr.dst_port);
         coarse_sleep(Duration::from_secs(1)) ;
         pr_info!("  Seq: {}", payload.data.datatcp.tcphdr.seq);
         coarse_sleep(Duration::from_secs(1)) ;
         pr_info!("  Ack: {} ", payload.data.datatcp.tcphdr.ack);
         coarse_sleep(Duration::from_secs(1)) ;
         pr_info!("  Offset: {}", payload.data.datatcp.tcphdr.offset);
         coarse_sleep(Duration::from_secs(1)) ;
         pr_info!("  Reserved: {}", payload.data.datatcp.tcphdr.reserved);
         coarse_sleep(Duration::from_secs(1)) ;
         pr_info!("  Flags: {:x} ", payload.data.datatcp.tcphdr.flags);
         coarse_sleep(Duration::from_secs(1)) ;
         pr_info!("  Window: {} ", payload.data.datatcp.tcphdr.window);
         coarse_sleep(Duration::from_secs(1)) ;
         pr_info!("  Checksum: {} ", payload.data.datatcp.tcphdr.checksum);
         coarse_sleep(Duration::from_secs(1)) ;
         pr_info!("  Urgent_ptr: {} ", payload.data.datatcp.tcphdr.urgent_ptr);
         coarse_sleep(Duration::from_secs(1)) ;
         pr_info!("-------------------------------") ; 
        coarse_sleep(Duration::from_secs(1)) ;*/
        // Print data details
        //pr_info!("  Data: {} ", payload.data.data);
        //coarse_sleep(Duration::from_secs(1)) ;
      


        /*pr_info!("start the conversion from CAN to Ethernet \n");
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("buffer is {:?}", buff );
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("-------------------------------") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        let payload = EthCanpayLoad ::deserialize_eth_canfd_payload(&buff).unwrap();
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("DONE DESERIALIZING THE CAN FRAME ")  ; 
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("PREPARING FOR A CONVERSION ") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("DONE THE CONVERSION FROM CAN FRAME INTO ETHERNET FRAME ") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("-- Eth Can Frame --");
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("  Destination MAC: {:?}", payload.dst_mac);
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("  Source MAC: {:?}", payload.src_mac);
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("  Ethertype: {:?}", payload.ethertype.0);
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("-------------------------------") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("-------------------------------") ;*/
        //stream.write_all(&buff[..n]).await?;
        
        /*pr_info!("-------------------------------") ;
        coarse_sleep(Duration::from_secs(1)) ;
        pr_err!("Frame length exceeds buffer size. Panicking...");
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("-------------------------------") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("-------------------------------") ;
        
        pr_info!("-------------------------------") ;
        coarse_sleep(Duration::from_secs(1)) ;
        pr_err!(" Panicking...");
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("-------------------------------") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        pr_info!("-------------------------------") ;*/
    
   
    //stream.write_all(&buff[..n]).await?;
    }
}


pub fn send_data(stream: &net::TcpStream, data: Vec<u8>) -> Result<usize> {
    // Ensure the data vector has exactly 52 elements
    let mut buffer = [0u8; 102 ];
    for (i, &item) in data.iter().enumerate() {
        buffer[i] = item;
    }
    // Write the data vector to the stream
    stream.write(&buffer,true)
    // Return the number of bytes written
   
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
    pr_info!(" listening") ;
    spawn_task!(ex, accept_loop(listener, ex.into()))?;
    Ok(())
}


pub fn connect(address: &SocketAddr) -> Result<net::TcpStream> {
    let socket = Socket::new(AddressFamily::Inet, SockType::Stream, IpProtocol::Tcp)?;
    socket.connect(address, 0)?; 
    Ok(net::TcpStream {sock:unsafe{socket.as_inner()}})
}


struct RustEchoServer {
    _handle: AutoStopHandle<dyn Executor>,
}

impl kernel::Module for RustEchoServer {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        let handle = WqExecutor::try_new(kernel::workqueue::system())?;
        pr_info!("************************LET'S GET STARTED********************************\n");
        start_listener(handle.executor())?;
        //echo_server(stream);
        Ok(Self {
            _handle: handle.into(),
        })
    }
}

module! {
    type: RustEchoServer,
    name: "rust_gateway",
    author: "Rust for Linux Contributors",
    description: "Rust gateway",
    license: "GPL",
}
