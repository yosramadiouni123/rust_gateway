/// Different imports : 
use bindings::canid_t;
use crate::prelude::*;
use alloc::borrow::ToOwned;





/// This struct defines some of the fields of the canfd_frame
/// Could look at a potential bindgen use 
/// This struct defines some of the fields of the canfd_frame
/// Could look at a potential bindgen use 
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
}

/*fn serialize_canfd_frame(frame: &canfdFrame) -> Vec<u8> {
    let mut serialized_data = Vec::new();
    let mut serializer = serde_cbor::Serializer::new(&mut serialized_data);
    frame.serialize(&mut serializer).unwrap();
    serialized_data

}*/

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
    pub data: TcpFrame,
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
            Some(&self.data.data.tcphdr)
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
   pub data: [u8;4],
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
    pub fn to_eth_frame(payload: EthCanfdpayLoad) -> EthFrame {
        let ip_header = Ipv4Header {
            version: payload.data.iphdr.version,
            len: payload.data.iphdr.len,
            to_s: payload.data.iphdr.to_s,
            total_len: payload.data.iphdr.total_len,
            id: payload.data.iphdr.id,
            flags: payload.data.iphdr.flags,
            frag_offset: payload.data.iphdr.frag_offset,
            ttl: payload.data.iphdr.ttl,
            protocol: payload.data.iphdr.protocol,
            checksum: payload.data.iphdr.checksum,
            src: payload.data.iphdr.src,
            dst: payload.data.iphdr.dst,
        };
        let tcp_header = TcpHeader {
            src_port: payload.data.tcphdr.src_port,
            dst_port: payload.data.tcphdr.dst_port,
            seq: payload.data.tcphdr.seq,
            ack: payload.data.tcphdr.ack,
            offset: payload.data.tcphdr.offset,
            reserved: payload.data.tcphdr.reserved,
            flags: payload.data.tcphdr.flags,
            window: payload.data.tcphdr.window,
            checksum: payload.data.tcphdr.checksum,
            urgent_ptr: payload.data.tcphdr.urgent_ptr,
        };
        let mut data = Vec::new();
        data.try_extend_from_slice(&payload.data.data);
        let eth_frame = EthFrame {
            dst: [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
            src: [0x00, 0x34, 0x56, 0x78, 0x90, 0xAB],
            ethertype: payload.ethertype,
            data: Ipv4Frame {
                header: ip_header,
                data: TcpFrame {
                    tcphdr: tcp_header,
                    data:payload.data.data,
                },
            },        
        };
        eth_frame
    }

    pub fn deserialize_eth_payload(buffer: &[u8]) -> Option<Self> {
       
        // Extract individual field slices using nom library
        let (dst_mac, rest) = buffer.split_at(6);
        let (src_mac, rest) = rest.split_at(6);
        let (ethertype, rest) =rest.split_at(2);
        let (data_can_bytes, _) = rest.split_at(46);
        let mut dst_arr = [0; 6];
        dst_arr.copy_from_slice(dst_mac);
        let mut eth_arr = [0; 2];
        eth_arr.copy_from_slice(ethertype);
        let mut src_arr = [0; 6];
        src_arr.copy_from_slice(src_mac);
        // Deserialize individual fields
        let data_can = Ethloadfd::from_bytes(data_can_bytes)?;
        Some(Self {
            dst_mac: dst_arr, // Convert slice to owned array
            src_mac: src_arr,
            ethertype: EtherTypefd(u16::from_be_bytes(eth_arr)), // Assuming conversion from u16
            data: data_can,
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
        // Check if the buffer has enough data for all fields
        /*if buffer.len() < mem::size_of::<Ipv4Header>() + mem::size_of::<TcpHeader>() + mem::size_of::<[u8; 4]>() {
            return None;
        }*/

        // Extract individual field slices
        pr_info!("{}", buffer.len());
        let (iphdr_bytes, rest) = buffer.split_at(21);
        let (tcphdr_bytes, rest) = rest.split_at(21);
        let (data, _) = rest.split_at(4);

        // Deserialize individual fields
        let iphdr = Ipv4Header::deserialize_ip_header(iphdr_bytes)?;
        let tcphdr = TcpHeader::deserialize_tcp_header(tcphdr_bytes)?;
        let data_vec = data.to_owned();
        let data_v1=vec_to_array(data_vec) ; 


        Some(Ethloadfd{
            iphdr,
            tcphdr,
            data:data_v1.unwrap(),
        })
    }
}


impl Ipv4Header {
    pub fn deserialize_ip_header(buffer: &[u8]) -> Option<Ipv4Header> {
            // Check if the buffer has enough data (expected size)
            /*if buffer.len() < mem::size_of::<Ipv4Header>() {
                return None;
            }*/
        
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
    

    fn vec_to_array(slice_data: &[u8]) -> core::result::Result<[u8; 4], &'static str> {
        // Check if the slice has exactly 4 elements
        if slice_data.len() != 4 {
            return Err("Error: Slice must contain exactly 4 elements");
        }
    
        // Safely convert the slice to a fixed-size array
        unsafe {
            // This is safe because we checked the slice length beforehand
            let array: [u8; 4] = *(slice_data.as_ptr() as *const [u8; 4]);
            Ok(array)
        }
    }
