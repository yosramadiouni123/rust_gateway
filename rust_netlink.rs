// SPDX-License-Identifier: GPL-2.0
use kernel::{
    file::{flags, File, Operations},
    io_buffer::{IoBufferReader, IoBufferWriter},
    miscdev,
    prelude::*,
    sync::{smutex::Mutex, Arc, ArcBorrow},
    Module,
    net::*,
    str::CString,
    delay::coarse_sleep,
};
use core::time::Duration;

module! {
    type: RustClientTest,
    name: "RUST_ETHERNET_CLIENT",
    license: "GPL",
    params: {
        devices: u32 {
            default: 1,
            permissions: 0o644,
            description: "Number of virtual devices",
        },
    },
}

pub fn connect(address: &SocketAddr) -> Result<TcpStream> {
    let socket = Socket::new(AddressFamily::Inet, SockType::Stream, IpProtocol::Tcp)?;
    socket.connect(address, 0)?; 
    pr_info!("RUST_NETLINK CONNECT FUNCTION IS BEING CALLED "); 
    coarse_sleep(Duration::from_secs(1)) ;
    Ok(TcpStream {sock:unsafe{socket.as_inner()}})
}

pub struct RustClientTest {
    stream: TcpStream,
}


impl kernel::Module for RustClientTest {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        let remote_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::ANY, 8080)) ;
        pr_info!("---INFO--- CREATED SOCKET SUCCESSFULLY !") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        
        let stream = connect(&remote_addr)?;
        pr_info!("---INFO--- CONNECTED TO PORT 8080 SUCCESSFULLY , BROADCAST MODE ENABLED ") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        // Example number to send 
        
        let raw_data:[u8;52]= [
            //can_id
            0x00, 0x00, 0x22 , 0x33,
            //can_len
            0x44,
            //flags
            0x11,
            //data
            0xAA, 0x00, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x00, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 
            0x33, 0x44, 0x55, 0x66, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x00,0x22, 0x33, 0x44, 0x55, 0x00,  

        ];          
       
        let data_vec=raw_data; 
       
        pr_info!("---INFO--- PUSHING DATA (CANFD FRAME) TO THE GATEWAY ! ") ;
        coarse_sleep(Duration::from_secs(1)) ;
        send_data(&stream, data_vec)? ; 

        Ok(Self { stream })
    }
}



pub fn send_data(stream: &TcpStream, data: [u8; 52]) -> Result<usize> {
    // Ensure the data vector has exactly 13 elements
    
    
    let mut buffer = [0u8; 52];
    for (i, &item) in data.iter().enumerate() {
        if i >= 52 {
            break; // Prevent index out of bounds
        }
        buffer[i] = item;
    }

    pr_info!("RUST_CLIENT : SEND_DATA FUNCTION IS BEING CALLED") ; 
    coarse_sleep(Duration::from_secs(1)) ;
    // Write the data vector to the stream
    stream.write(&buffer,true)

    // Return the number of bytes written
   
}
