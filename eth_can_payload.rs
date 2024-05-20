use bindings::canid_t;
use alloc::vec::*;
use alloc::borrow::ToOwned;

#[derive(PartialEq)]
#[derive(Clone)]

pub struct CanFrame {
 pub can_id: canid_t,
 pub can_dlc: u8,
 pub data: [u8; 8],
 /* private fields */
}

 impl CanFrame {
    pub fn get_can_id(&self) -> canid_t {
        self.can_id
    }
    pub fn get_can_dlc(&self) -> u8 {
        self.can_dlc
    }
    pub fn get_data(&self) -> &[u8; 8] {
        &self.data
    }
}

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub struct EtherType(pub u16);

impl EtherType {
    pub const IPV4: EtherType = Self(0x0800);
    pub const IPV6: EtherType = Self(0x86dd);
    pub const ARP: EtherType = Self(0x0806);
    pub const WAKE_ON_LAN: EtherType = Self(0x0842);
    pub const VLAN_TAGGED_FRAME: EtherType = Self(0x8100);
    pub const PROVIDER_BRIDGING: EtherType = Self(0x88A8);
    pub const VLAN_DOUBLE_TAGGED_FRAME: EtherType = Self(0x9100);
}
/// EthFrame struct
#[derive(PartialEq)]
#[derive(Clone)]
pub struct EthFrame {
    pub dst: [u8; 6],
    pub src: [u8; 6],
    pub ethertype: EtherType,
    pub data: [u8; 4],
}
#[derive(PartialEq)]
#[derive(Clone)]

pub struct Ethload {
    pub data : [u8;4]
}
//==========serialize============

fn serialize_ethload(ethload: &Ethload) -> Vec<u8> {
    let mut serialized_data = Vec::new();
    // Serialize data_eth (slice of u8)
    serialized_data.try_extend_from_slice(&ethload.data);
    serialized_data
}
//==========serialize============
  
/// eth_canpayload struct
#[derive(PartialEq)]
#[derive(Clone)]
pub struct EthCanpayLoad {
    ///dst_mac
    pub dst_mac: [u8; 6],
    ///src_mac
    pub src_mac: [u8; 6],
    ///ethertype
    pub ethertype: EtherType,
    ///data
    pub data: Ethload
}

impl EthCanpayLoad {
    pub fn to_eth_frame(payload: EthCanpayLoad) -> EthFrame {

        /*// Assuming data.len() is at least the size of the copied data (here, 4 bytes)
        let mut data_slice = [0; 4]; // Initialize data with zeroes
        data_slice.copy_from_slice(&frame.data.data.data[..4]); // Copy first 4 bytes of TCP data
 
        // Calculate the total length of the payload: IP header size + TCP header size + data size
        let payload_len =  data_slice.len() as u8;
 
        can_ethpayload {
            can_id: 0, // Set default value for CAN ID
            len: payload_len,
            flags: 0, // Set default value for flags (you can define flags if needed)
            data: Ethload {
                data: data_slice,
            },
        }*/
        let mut data = Vec::new();
        data.try_extend_from_slice(&payload.data.data);
        let eth_frame = EthFrame {
            dst: [0xDE, 0xAD, 0xBE, 0xEF, 0xCA, 0xFE],
            src: [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB],
            ethertype: payload.ethertype,
                data:payload.data.data,
                  
        };
        eth_frame
    }

    pub fn deserialize_eth_payload(buffer: &[u8]) -> Option<Self> {
       
        // Extract individual field slices using nom library
        let (dst_mac, rest) = buffer.split_at(6);
        let (src_mac, rest) = rest.split_at(6);
        let (ethertype, rest) =rest.split_at(2);
        let (data_can_bytes, _) = rest.split_at(8);
        let mut dst_arr = [0; 6];
        dst_arr.copy_from_slice(dst_mac);
        let mut eth_arr = [0; 2];
        eth_arr.copy_from_slice(ethertype);
        let mut src_arr = [0; 6];
        src_arr.copy_from_slice(src_mac);
        // Deserialize individual fields
        let data_can = Ethload::from_bytes(data_can_bytes)?;
        Some(Self {
            dst_mac: dst_arr, // Convert slice to owned array
            src_mac: src_arr,
            ethertype: EtherType(u16::from_be_bytes(eth_arr)), // Assuming conversion from u16
            data: data_can,
        })
    }

}

pub fn serialize_eth_can_payload(payload: &EthCanpayLoad) -> Vec<u8> {
    let mut serialized_data = Vec::new();
    // Serialize dst_mac
    serialized_data.try_extend_from_slice(&payload.dst_mac);
    // Serialize src_mac
    serialized_data.try_extend_from_slice(&payload.src_mac);
    // Serialize ethertype (assuming u16 representation)
    let ethertype_bytes = (payload.ethertype.0 as u16).to_be_bytes();
    serialized_data.try_extend_from_slice(&ethertype_bytes);
    // Serialize data (using Ethload serialization function)
    let ethload_bytes = serialize_ethload(&payload.data);
    serialized_data.try_extend_from_slice(&ethload_bytes);
    serialized_data
}

// ================================================ DESERIALIAZATION =====================================================

impl Ethload {
    pub fn from_bytes(buffer: &[u8]) -> Option<Ethload> {
        // Check if the buffer has enough data for all fields
        /*if buffer.len() < mem::size_of::<Ipv4Header>() + mem::size_of::<TcpHeader>() + mem::size_of::<[u8; 4]>() {
            return None;
        }*/
        // Extract individual field slices
        let (dst_mac, rest) = buffer.split_at(21);
        let (data, _) = rest.split_at(4);
        // Deserialize individual fields
        let data_vec = data.to_owned();
        let data_v1=vec_to_array(data_vec) ; 
        Some(Ethload {
            data:data_v1.unwrap(),
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