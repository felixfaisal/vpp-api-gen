/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SetIpfixExporter { 
	pub client_index : u32, 
	pub context : u32, 
	pub collector_address : Address, 
	pub collector_port : u16, 
	pub src_address : Address, 
	pub vrf_id : u32, 
	pub path_mtu : u32, 
	pub template_interval : u32, 
	pub udp_checksum : bool, 
} 
impl SetIpfixExporter { 
	 pub fn get_message_id() -> String { 
	 	 String::from("set_ipfix_exporter_69284e07") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SetIpfixExporterReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl SetIpfixExporterReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("set_ipfix_exporter_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpfixExporterDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl IpfixExporterDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipfix_exporter_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpfixExporterDetails { 
	pub context : u32, 
	pub collector_address : Address, 
	pub collector_port : u16, 
	pub src_address : Address, 
	pub vrf_id : u32, 
	pub path_mtu : u32, 
	pub template_interval : u32, 
	pub udp_checksum : bool, 
} 
impl IpfixExporterDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipfix_exporter_details_11e07413") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SetIpfixClassifyStream { 
	pub client_index : u32, 
	pub context : u32, 
	pub domain_id : u32, 
	pub src_port : u16, 
} 
impl SetIpfixClassifyStream { 
	 pub fn get_message_id() -> String { 
	 	 String::from("set_ipfix_classify_stream_c9cbe053") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct SetIpfixClassifyStreamReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl SetIpfixClassifyStreamReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("set_ipfix_classify_stream_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpfixClassifyStreamDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl IpfixClassifyStreamDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipfix_classify_stream_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpfixClassifyStreamDetails { 
	pub context : u32, 
	pub domain_id : u32, 
	pub src_port : u16, 
} 
impl IpfixClassifyStreamDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipfix_classify_stream_details_2903539d") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpfixClassifyTableAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub table_id : u32, 
	pub ip_version : AddressFamily, 
	pub transport_protocol : IpProto, 
	pub is_add : bool, 
} 
impl IpfixClassifyTableAddDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipfix_classify_table_add_del_3e449bb9") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpfixClassifyTableAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl IpfixClassifyTableAddDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipfix_classify_table_add_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpfixClassifyTableDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl IpfixClassifyTableDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipfix_classify_table_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpfixClassifyTableDetails { 
	pub context : u32, 
	pub table_id : u32, 
	pub ip_version : AddressFamily, 
	pub transport_protocol : IpProto, 
} 
impl IpfixClassifyTableDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipfix_classify_table_details_1af8c28c") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpfixFlush { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl IpfixFlush { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipfix_flush_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct IpfixFlushReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl IpfixFlushReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("ipfix_flush_reply_e8d4e804") 
	 } 
} 
