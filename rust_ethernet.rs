use kernel::{
    file::{flags, File},
    io_buffer::{IoBufferReader},
    miscdev,
    prelude::*,
    sync::{smutex::Mutex, Arc, ArcBorrow},
    Module,
};
use kernel::str::CString;
use kernel::delay::coarse_sleep ; 
use core::time::Duration;

/* 
use kernel::tcp::{EthFrame , canfd_ethpayload , TCP_Frame , Ipv4Frame , Ipv4Header , TcpHeader ,EtherType} ; 
use kernel::tcp::{serialize_canfd_ethpayload};
use alloc::vec::*; 
*/

module! {
    type: RustClientTest,
    name: "RUST_CAN",
    license: "GPL",
    params: {
        devices: u32 {
            default: 1,
            permissions: 0o644,
            description: "Number of virtual devices",
        },
    },
}


use kernel::net::*;
use core::*;


pub fn connect(address: &SocketAddr) -> Result<TcpStream> {
    let socket = Socket::new(AddressFamily::Inet, SockType::Stream, IpProtocol::Tcp)?;
    socket.connect(address, 0)?; 
    pr_info!("RUST_NETLINK CONNECT FUNCTION IS BEING CALLED "); 
    coarse_sleep(Duration::from_secs(1)) ;
    Ok(TcpStream {sock:unsafe{socket.as_inner()}})
}


pub fn send_data(stream: &TcpStream, data: [u8; 13]) -> Result<usize> {
    // Ensure the data vector has exactly 13 elements
    
    
    let mut buffer = [0u8; 60];
    for (i, &item) in data.iter().enumerate() {
        if i >= 60 {
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

pub struct RustClientTest {
    stream: TcpStream,
}

impl kernel::Module for RustClientTest {
    fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
        let remote_addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::ANY, 8080)) ;
        pr_info!("---INFO--- CREATED SOCKET SUCCCESSFULLY !") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        
        let stream = connect(&remote_addr)?;
        pr_info!("---INFO--- CONNECTED TO PORT 8080 SUCCESSFULLY , BROADCAST MODE ENABLED ") ; 
        coarse_sleep(Duration::from_secs(1)) ;
        // Example number to send 
        
        let raw_data:[u8;13]= [
            //can_id
            0x00, 0x11, 0x22 , 0x33,
            //can_dlc
            0x44,
            //data
            0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x22, 0x33,
        ];          
       
        let data_vec=raw_data; 
       
        pr_info!("---INFO--- PUSHING DATA (CAN FRAME) TO THE GATEWAY ! ") ;
        coarse_sleep(Duration::from_secs(1)) ;
        send_data(&stream, data_vec)? ; 

        Ok(Self { stream })
    }
}

