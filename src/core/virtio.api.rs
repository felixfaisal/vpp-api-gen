/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum IfStatusFlags { 
	 IF_STATUS_API_FLAG_ADMIN_UP=1, 
	 IF_STATUS_API_FLAG_LINK_UP=2, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum MtuProto { 
	 MTU_PROTO_API_L3=0, 
	 MTU_PROTO_API_IP4=1, 
	 MTU_PROTO_API_IP6=2, 
	 MTU_PROTO_API_MPLS=3, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum LinkDuplex { 
	 LINK_DUPLEX_API_UNKNOWN=0, 
	 LINK_DUPLEX_API_HALF=1, 
	 LINK_DUPLEX_API_FULL=2, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum SubIfFlags { 
	 SUB_IF_API_FLAG_NO_TAGS=1, 
	 SUB_IF_API_FLAG_ONE_TAG=2, 
	 SUB_IF_API_FLAG_TWO_TAGS=4, 
	 SUB_IF_API_FLAG_DOT1AD=8, 
	 SUB_IF_API_FLAG_EXACT_MATCH=16, 
	 SUB_IF_API_FLAG_DEFAULT=32, 
	 SUB_IF_API_FLAG_OUTER_VLAN_ID_ANY=64, 
	 SUB_IF_API_FLAG_INNER_VLAN_ID_ANY=128, 
	 SUB_IF_API_FLAG_MASK_VNET=254, 
	 SUB_IF_API_FLAG_DOT1AH=256, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum RxMode { 
	 RX_MODE_API_UNKNOWN=0, 
	 RX_MODE_API_POLLING=1, 
	 RX_MODE_API_INTERRUPT=2, 
	 RX_MODE_API_ADAPTIVE=3, 
	 RX_MODE_API_DEFAULT=4, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum IfType { 
	 IF_API_TYPE_HARDWARE=0, 
	 IF_API_TYPE_SUB=1, 
	 IF_API_TYPE_P2P=2, 
	 IF_API_TYPE_PIPE=3, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum Direction { 
	 RX=0, 
	 TX=1, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum VirtioFlags { 
	 VIRTIO_API_FLAG_GSO=1, 
	 VIRTIO_API_FLAG_CSUM_OFFLOAD=2, 
	 VIRTIO_API_FLAG_GRO_COALESCE=4, 
	 VIRTIO_API_FLAG_PACKED=8, 
	 VIRTIO_API_FLAG_IN_ORDER=16, 
	 VIRTIO_API_FLAG_BUFFERING=32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VirtioPciCreate { 
	pub client_index : u32, 
	pub context : u32, 
	pub pci_addr : PciAddress, 
	pub use_random_mac : bool, 
	pub mac_address : MacAddress, 
	pub gso_enabled : bool, 
	pub checksum_offload_enabled : bool, 
	pub features : u64, 
} 
impl VirtioPciCreate { 
	 pub fn get_message_id() -> String { 
	 	 String::from("virtio_pci_create_a9f1370c") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VirtioPciCreateReply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl VirtioPciCreateReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("virtio_pci_create_reply_5383d31f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VirtioPciCreateV2 { 
	pub client_index : u32, 
	pub context : u32, 
	pub pci_addr : PciAddress, 
	pub use_random_mac : bool, 
	pub mac_address : MacAddress, 
	pub virtio_flags : VirtioFlags, 
	pub features : u64, 
} 
impl VirtioPciCreateV2 { 
	 pub fn get_message_id() -> String { 
	 	 String::from("virtio_pci_create_v2_5d096e1a") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VirtioPciCreateV2Reply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl VirtioPciCreateV2Reply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("virtio_pci_create_v2_reply_5383d31f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VirtioPciDelete { 
	pub client_index : u32, 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl VirtioPciDelete { 
	 pub fn get_message_id() -> String { 
	 	 String::from("virtio_pci_delete_f9e6675e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct VirtioPciDeleteReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl VirtioPciDeleteReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("virtio_pci_delete_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SwInterfaceVirtioPciDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl SwInterfaceVirtioPciDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sw_interface_virtio_pci_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SwInterfaceVirtioPciDetails { 
	pub context : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub pci_addr : PciAddress, 
	pub mac_addr : MacAddress, 
	pub tx_ring_sz : u16, 
	pub rx_ring_sz : u16, 
	pub features : u64, 
} 
impl SwInterfaceVirtioPciDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("sw_interface_virtio_pci_details_16187f3a") 
	 } 
} 