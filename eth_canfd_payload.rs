/// Different imports : 
use bindings::{canfd_frame, canid_t};
use crate::prelude::*;
use alloc::borrow::ToOwned;
#[derive(PartialEq)]
#[derive(Clone)]
pub struct canfdFrame {
    pub can_id: canid_t,
    pub len: u8,
    pub flags: u8,
    pub data : [u8;46]
    /* private fields */
}

impl canfdFrame {
    pub fn get_can_id(&self) -> canid_t {
        self.can_id
    }
    pub fn get_len(&self) -> u8 {
        self.len
    }
    pub fn get_flags(&self) -> u8 {
        self.flags
    }
    pub fn get_data(&self) -> &[u8; 46] {
        &self.data
    }
    pub fn deserialize_canfd_payload(buffer: &[u8]) -> Option<Self> {
       
        // Extract individual field slices using nom library
        let (can_id, rest) = buffer.split_at(4);
        let (len, rest) = rest.split_at(1);
        let (flags, rest) =rest.split_at(1);
        let (data, _) = rest.split_at(46);
        let mut can_id_arr = [0; 4];
        can_id_arr.copy_from_slice(can_id);
        let mut len_arr = [0; 1];
        len_arr.copy_from_slice(len);
        let mut flags_arr = [0; 1];
        flags_arr.copy_from_slice(flags);
        let data_vec = data.to_owned();
        let data_v1=vec_to_array1(data_vec) ; 
        // Deserialize individual fields
       // let data = Ethloadfd::from_bytes(data)?;
        //let can_id=u32::from_be_bytes(can_id);
        let can_id_vec = can_id.to_owned();
        let can_id_v1=vec_to_array(can_id_vec) ; 
        let can_id = u32::from_be_bytes(can_id_arr) ; 
        let len = u8:: from_be_bytes(len_arr);
        let flags= u8::from_be_bytes(flags_arr);
        //let data_vec= data.to_o
        Some(Self {
            can_id,
            len:len,
            flags:flags,
            data: data_v1.unwrap(),
        })
    }
}



#[derive(PartialEq)]
#[derive(Clone)]
pub struct EtherTypefd(pub u16);

impl EtherTypefd {
    pub const IPV4: EtherTypefd = Self(0x0800);
    pub const IPV6: EtherTypefd = Self(0x86dd);
    pub const ARP: EtherTypefd = Self(0x0806);
    pub const WAKE_ON_LAN: EtherTypefd = Self(0x0842);
    pub const VLAN_TAGGED_FRAME: EtherTypefd = Self(0x8100);
    pub const PROVIDER_BRIDGING: EtherTypefd = Self(0x88A8);
    pub const VLAN_DOUBLE_TAGGED_FRAME: EtherTypefd = Self(0x9100);
}
/// TCP_Frame struct
#[derive(PartialEq)]
#[derive(Clone)]
pub struct TcpFrame {
    pub tcphdr: TcpHeader,
    pub data: [ u8; 4],
}

/// Ipv4Frame struct
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Ipv4Frame {
    pub header: Ipv4Header,
    pub datatcp: TcpFrame,
}

/// EthFrame struct
#[derive(PartialEq)]
#[derive(Clone)]
pub struct EthFrame {
    pub dst: [u8; 6],
    pub src: [u8; 6],
    pub ethertype: EtherTypefd,
    pub data: Ipv4Frame,
}


impl EthFrame {
    pub fn get_ip_header(&self) -> Option<&Ipv4Header> {
        if self.ethertype == EtherTypefd::IPV4 {
            Some(&self.data.header)
        } else {
            None
        }
    }
    pub fn get_tcp_header(&self) -> Option<&TcpHeader> {
        if self.ethertype == EtherTypefd::IPV4 {
            Some(&self.data.datatcp.tcphdr)
        } else {
            None
        }
    }
}

/// Ipv4Header struct
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub struct Ipv4Header {
    pub version: u8, // 0x04
    pub len: u8,
    pub to_s: u8,
    pub total_len: u16,
    pub id: u16, //
    pub flags: u8, //3b
    pub frag_offset: u8, //13b
    pub ttl: u8,  //
    pub protocol: u8, // 0x06,   tcp-->6 u8
    pub checksum: u16, //
    pub src: [u8; 4], //
    pub dst: [u8; 4], //
}


//============serialize Ipv4 Header ============
impl Ipv4Header {
    pub fn serialized_size(&self) -> usize {
        1 + 1 + 1 + 2 + 2 + 1 + 1 + 1 + 1 + 2 + 4 + 4
    }
}

fn serialize_ip_header(ip_header: &Ipv4Header) -> Vec<u8> {
    let mut serialized_data = Vec::new();
    // Serialize each field of Ipv4Header (u8, u16, etc.) using to_be_bytes()
    serialized_data.try_push(ip_header.version);
    serialized_data.try_push(ip_header.len);
    serialized_data.try_push(ip_header.to_s);
    serialized_data.try_extend_from_slice(&ip_header.total_len.to_be_bytes());
    serialized_data.try_extend_from_slice(&ip_header.id.to_be_bytes());
    serialized_data.try_push(ip_header.flags);
    serialized_data.try_push(ip_header.frag_offset);
    serialized_data.try_push(ip_header.ttl);
    serialized_data.try_push(ip_header.protocol);
    serialized_data.try_extend_from_slice(&ip_header.checksum.to_be_bytes());
    serialized_data.try_extend_from_slice(&ip_header.src);
    serialized_data.try_extend_from_slice(&ip_header.dst);

    serialized_data
}
//============serialize============





/// TcpHeader struct
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub struct TcpHeader {
    pub src_port: u16,
    pub dst_port: u16,
    pub seq: u32,
    pub ack: u32,
    pub offset: u8, //4b
    pub reserved: u8, //4b
    pub flags: u8,
    pub window: u16,
    pub checksum: u16,
    pub urgent_ptr: u16,
}
//==========serialize============
impl TcpHeader {
    fn serialized_size(&self) -> usize {
      // Fixed-size fields
      let fixed_size = 2 + 2 + 4 + 4 + 1 + 1 + 1 + 2 + 2 + 2; // src_port, dst_port, seq, ack, offset, reserved, flags, window, checksum, urgent_ptr
      fixed_size
    }
    //==========serialize TCP header ==============
    fn serialize_tcp_header(tcp_header: &TcpHeader) -> Vec<u8> {
        let mut serialized_data = Vec::new();
    
        // Serialize each field of TcpHeader (u16, u32, etc.) using to_be_bytes()
        serialized_data.try_extend_from_slice(&tcp_header.src_port.to_be_bytes());
        serialized_data.try_extend_from_slice(&tcp_header.dst_port.to_be_bytes());
        serialized_data.try_extend_from_slice(&tcp_header.seq.to_be_bytes());
        serialized_data.try_extend_from_slice(&tcp_header.ack.to_be_bytes());
        serialized_data.try_push(tcp_header.offset); // u8, no need for to_be_bytes()
        serialized_data.try_push(tcp_header.reserved); // u8, no need for to_be_bytes()
        serialized_data.try_push(tcp_header.flags); // u8, no need for to_be_bytes()
        serialized_data.try_extend_from_slice(&tcp_header.window.to_be_bytes());
        serialized_data.try_extend_from_slice(&tcp_header.checksum.to_be_bytes());
        serialized_data.try_extend_from_slice(&tcp_header.urgent_ptr.to_be_bytes());
    
        serialized_data
    }
}
//==========serialize============


/// Ethloadfd struct
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Ethloadfd {
    /// iphdr
   pub iphdr : Ipv4Header ,
   /// tcphdr 
   pub tcphdr : TcpHeader ,
   /// data 
   pub data: [u8;46],
}
//==========serialize Ethloadfd ============


fn serialize_ethload(ethload: &Ethloadfd) -> Vec<u8> {
    let mut serialized_data = Vec::new();
    // Serialize iphdr (Ipv4Header)
    serialized_data.try_extend_from_slice(&serialize_ip_header(&ethload.iphdr));
    // Serialize tcphdr (TcpHeader)
    serialized_data.try_extend_from_slice(&serialize_tcp_header(&ethload.tcphdr));
    // Serialize data_eth (slice of u8)
    serialized_data.try_extend_from_slice(&ethload.data);
    serialized_data
}
//==========serialize============
  








/// eth_canfdpayload struct
#[derive(PartialEq)]
#[derive(Clone)]
pub struct EthCanfdpayLoad {
    ///dst_mac
    pub dst_mac: [u8; 6],
    ///src_mac
    pub src_mac: [u8; 6],
    ///ethertype
    pub ethertype: EtherTypefd,
    ///data
    pub data: Ethloadfd
}

//==========serialize============

//==========serialize============






impl EthCanfdpayLoad {
    pub fn to_eth_frame(payload:  &canfdFrame) -> Self {
        let ip_header = Ipv4Header {
            version: 4,
            len: 20, // Assuming minimum header length
            to_s: 1, // ToS (Type of Service) field example
            total_len: 50, // Assuming total length (including data)
            id: 100, // Example ID
            flags: 0, // Flags field example (no flags set)
            frag_offset: 0, // Fragment offset (no fragmentation)
            ttl: 64, // Time to live example
            protocol: 6, // TCP protocol
            checksum: 0xABCD, // Mock checksum value
            src: [192, 168, 1, 10],
            dst: [10, 0, 0, 1],
        };
        let tcp_header = TcpHeader {
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
        };
        let mut data = Vec::new();
        data.try_extend_from_slice(&payload.data);
        let eth_frame = EthCanfdpayLoad {
            dst_mac: [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
            src_mac: [0x00, 0x34, 0x56, 0x78, 0x90, 0xAB],
            ethertype: EtherTypefd::IPV4,
            data: Ethloadfd {
                iphdr: ip_header,
                tcphdr: tcp_header,
                data:payload.data,
                
            },        
        };
        eth_frame
    }
        pub fn deserialize_eth_canfd_payload(buffer: &[u8] ) -> Option<EthCanfdpayLoad>
        {   // Extract individual field slices using nom library
            let (dst_mac, rest) = buffer.split_at(6);
            let (src_mac, rest) = rest.split_at(6);
            let (ethertype, rest) =rest.split_at(2);
            let (data, _) = rest.split_at(88);
            let mut dst_arr = [0; 6];
            dst_arr.copy_from_slice(dst_mac);

            let mut src_arr = [0; 6];
            src_arr.copy_from_slice(src_mac);
    
            let mut eth_arr = [0; 2];
            eth_arr.copy_from_slice(ethertype);
    
            

            let mut data_arr = [0; 88];
            data_arr.copy_from_slice(data);
            let data_vec = data.to_owned();
           // let data_v1=vec_to_array1(data_vec) ; 
            // Deserialize individual fields
            let data = Ethloadfd::from_bytes(data)?;
            //let can_id=u32::from_be_bytes(can_id);
            //let can_id_vec = can_id.to_owned();
            //let can_id_v1=vec_to_array(can_id_vec) ; 
           // let can_id = u32::from_be_bytes(can_id_arr) ; 
            //let len = u8:: from_be_bytes(len_arr);
          //  let flags= u8::from_be_bytes(flags_arr);
            //let data_vec= data.to_o
            Some(Self {
                dst_mac: dst_arr, // Convert slice to owned array
                src_mac: src_arr,
                ethertype: EtherTypefd(u16::from_be_bytes(eth_arr)), // Assuming conversion from u16
                data: data,

            })  
        }

        
    
    
    
}
    pub fn serialize_eth_canfd_payload(payload: &EthCanfdpayLoad) -> Vec<u8> {
        let mut serialized_data = Vec::new();
        // Serialize dst_mac
        serialized_data.try_extend_from_slice(&payload.dst_mac);
        // Serialize src_mac
        serialized_data.try_extend_from_slice(&payload.src_mac);
        // Serialize ethertype (assuming u16 representation)
        let ethertype_bytes = (payload.ethertype.0 as u16).to_be_bytes();
        serialized_data.try_extend_from_slice(&ethertype_bytes);
        // Serialize data (using Ethloadfd serialization function)
        let ethload_bytes = serialize_ethload(&payload.data);
        serialized_data.try_extend_from_slice(&ethload_bytes);
        serialized_data
    }








/*fn deserialize_canfd_frame(data: &[u8]) -> canfdFrame {
    let mut deserializer = serde_cbor::Deserializer::from_slice(data);
    let frame: canfdFrame = serde_cbor::Deserialize::deserialize(&mut deserializer).unwrap();
    frame
}*/




pub fn serialize_tcp_header(tcp_header: &TcpHeader) -> Vec<u8> {
    let mut serialized_data = Vec::new();

    // Serialize each field of TcpHeader (u16, u32, etc.) using to_be_bytes()
    serialized_data.try_extend_from_slice(&tcp_header.src_port.to_be_bytes());
    serialized_data.try_extend_from_slice(&tcp_header.dst_port.to_be_bytes());
    serialized_data.try_extend_from_slice(&tcp_header.seq.to_be_bytes());
    serialized_data.try_extend_from_slice(&tcp_header.ack.to_be_bytes());
    serialized_data.try_push(tcp_header.offset); // u8, no need for to_be_bytes()
    serialized_data.try_push(tcp_header.reserved); // u8, no need for to_be_bytes()
    serialized_data.try_push(tcp_header.flags); // u8, no need for to_be_bytes()
    serialized_data.try_extend_from_slice(&tcp_header.window.to_be_bytes());
    serialized_data.try_extend_from_slice(&tcp_header.checksum.to_be_bytes());
    serialized_data.try_extend_from_slice(&tcp_header.urgent_ptr.to_be_bytes());

    serialized_data
}



// ================================================ DESERIALIAZATION =====================================================





impl Ethloadfd {
    pub fn from_bytes(buffer: &[u8]) -> Option<Ethloadfd> {
       
        // Extract individual field slices
        //pr_info!("{}", buffer.len());
        let (iphdr_bytes, rest) = buffer.split_at(21);
        let (tcphdr_bytes, rest) = rest.split_at(21);
        let (data, _) = rest.split_at(46);//46

        // Deserialize individual fields
        let iphdr = Ipv4Header::deserialize_ip_header(iphdr_bytes)?;
        let tcphdr = TcpHeader::deserialize_tcp_header(tcphdr_bytes)?;
        let data_vec = data.to_owned();
        let data_v1=vec_to_array1(data_vec) ; 


        Some(Ethloadfd{
            iphdr,
            tcphdr,
            data:data_v1.unwrap(),
        })
    }
}


impl Ipv4Header {
    pub fn deserialize_ip_header(buffer: &[u8]) -> Option<Ipv4Header> {
            
        
            // Extract individual field values
            let (version, rest) = buffer.split_at(1);
            let (len, rest) = rest.split_at(1);
            let (tos, rest) = rest.split_at(1);
            let (total_len, rest) = rest.split_at(2);
            let (id, rest) = rest.split_at(2);
            let (flags, rest) = rest.split_at(1);
            let (frag_offset, rest) = rest.split_at(1);
            let (ttl, rest) = rest.split_at(1);
            let (protocol, rest) = rest.split_at(1);
            let (checksum, rest) = rest.split_at(2);
            let (src,rest) = rest.split_at(4);
            let (dst,_)=rest.split_at(4) ; 
    
            let mut total_len_arr = [0; 2];
            total_len_arr.copy_from_slice(total_len);
            let mut id_arr = [0; 2];
            id_arr.copy_from_slice(id);
            let mut checksum_arr = [0; 2];
            checksum_arr.copy_from_slice(checksum);
    
            let src_vec = src.to_owned();
            let src_v1=vec_to_array(src_vec) ;  
            let dst_vec = dst.to_owned();
            let dst_v1=vec_to_array(dst_vec) ; 
    
            // Convert slices to fixed-size values (avoid unnecessary copies)
            let total_len = u16::from_be_bytes(total_len_arr);
            let checksum = u16::from_be_bytes(checksum_arr) ; 
            let id = u16::from_be_bytes(id_arr);
            let ttl = ttl[0]; // Single byte for TTL
       
    
            // Create the Ipv4Header struct
            Some(Ipv4Header {
                version: version[0],
                len: len[0],
                to_s: tos[0],
                total_len,
                id,
                flags: flags[0],
                frag_offset: frag_offset[0],
                ttl,
                protocol: protocol[0],
                checksum,
                src:src_v1.unwrap(),
                dst:dst_v1.unwrap(),
            })
        }
    }
impl TcpHeader {
    pub fn deserialize_tcp_header(buffer: &[u8]) -> Option<TcpHeader> {
    
        // Extract individual field slices
        let (src_port, rest) = buffer.split_at(2);
        let (dst_port, rest) = rest.split_at(2);
        let (seq, rest) = rest.split_at(4);
        let (ack, rest) = rest.split_at(4);
        let (offset, rest) = rest.split_at(1);
        let (reserved,rest) =rest.split_at(1) ; 
        let (flags, rest) = rest.split_at(1);
        let (window, rest) = rest.split_at(2);
        let (checksum, rest) = rest.split_at(2);
        let (urgent_ptr, _) = rest.split_at(2);
        
        // Convert slices to fixed-size arrays before conversion
        let mut src_port_arr = [0; 2];
        src_port_arr.copy_from_slice(src_port);
        let mut dst_port_arr = [0; 2];
        dst_port_arr.copy_from_slice(dst_port);
        let mut seq_arr = [0; 4];
        seq_arr.copy_from_slice(seq);
        let mut ack_arr = [0; 4];
        ack_arr.copy_from_slice(ack);
        let mut window_arr = [0; 2];
        window_arr.copy_from_slice(window);
        let mut checksum_arr = [0; 2];
        checksum_arr.copy_from_slice(checksum);
        let mut urgent_ptr_arr = [0; 2];
        urgent_ptr_arr.copy_from_slice(urgent_ptr);
        
        // Extract individual fields and convert from network byte order
        let src_port = u16::from_be_bytes(src_port_arr);
        let dst_port = u16::from_be_bytes(dst_port_arr);
        let seq = u32::from_be_bytes(seq_arr);
        let ack = u32::from_be_bytes(ack_arr);
        let offset = (offset[0] >> 4) & 0x0F;
        let reserved = offset & 0x0F;
        let flags = flags[0];
        let window = u16::from_be_bytes(window_arr);
        let checksum = u16::from_be_bytes(checksum_arr);
        let urgent_ptr = u16::from_be_bytes(urgent_ptr_arr);
        
        // Create the TcpHeader struct
        Some(TcpHeader{
            src_port,
            dst_port,
            seq,
            ack,
            offset,
            reserved,
            flags,
             window,
                checksum,
                urgent_ptr,
            })
        }
    }    
    

fn vec_to_array1(slice_data: &[u8]) -> core::result::Result<[u8; 46], &'static str> {
    // Check if the slice has exactly 4 elements
    if slice_data.len() != 46{
        return Err("Error: Slice must contain exactly 46 elements");
    }
    
    // Safely convert the slice to a fixed-size array
    unsafe {
        // This is safe because we checked the slice length beforehand
        let array: [u8; 46] = *(slice_data.as_ptr() as *const [u8; 46]);
        Ok(array)
    }
}
fn vec_to_array(slice_data: &[u8]) -> core::result::Result<[u8; 4], &'static str> {
    // Check if the slice has exactly 4 elements
    if slice_data.len() != 4{
        return Err("Error: Slice must contain exactly 4 elements");
    }
    
    // Safely convert the slice to a fixed-size array
    unsafe {
        // This is safe because we checked the slice length beforehand
        let array: [u8; 4] = *(slice_data.as_ptr() as *const [u8; 4]);
        Ok(array)
    }
}
fn vec_to_array2(slice_data: &[u8]) -> core::result::Result<[u8; 1], &'static str> {
    // Check if the slice has exactly1 elements
    if slice_data.len() != 1{
        return Err("Error: Slice must contain exactly 1 elements");
    }
    
    // Safely convert the slice to a fixed-size array
    unsafe {
        // This is safe because we checked the slice length beforehand
        let array: [u8; 1] = *(slice_data.as_ptr() as *const [u8; 1]);
        Ok(array)
    }
}
