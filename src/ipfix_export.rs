/*
   Autogenerated Data, Do not Edit! 
   Author: @felixfaisal 
*/
#![allow(non_camel_case_types)]
use vpp_macros::Message; 
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum::{U10, U24, U256, U32, U64};
use crate::ip_types::*; 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(set_ipfix_exporter_69284e07)] 
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
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(set_ipfix_exporter_reply_e8d4e804)] 
pub struct SetIpfixExporterReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipfix_exporter_dump_51077d14)] 
pub struct IpfixExporterDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipfix_exporter_details_11e07413)] 
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
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(set_ipfix_classify_stream_c9cbe053)] 
pub struct SetIpfixClassifyStream { 
	pub client_index : u32, 
	pub context : u32, 
	pub domain_id : u32, 
	pub src_port : u16, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(set_ipfix_classify_stream_reply_e8d4e804)] 
pub struct SetIpfixClassifyStreamReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipfix_classify_stream_dump_51077d14)] 
pub struct IpfixClassifyStreamDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipfix_classify_stream_details_2903539d)] 
pub struct IpfixClassifyStreamDetails { 
	pub context : u32, 
	pub domain_id : u32, 
	pub src_port : u16, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipfix_classify_table_add_del_3e449bb9)] 
pub struct IpfixClassifyTableAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub table_id : u32, 
	pub ip_version : AddressFamily, 
	pub transport_protocol : IpProto, 
	pub is_add : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipfix_classify_table_add_del_reply_e8d4e804)] 
pub struct IpfixClassifyTableAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipfix_classify_table_dump_51077d14)] 
pub struct IpfixClassifyTableDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipfix_classify_table_details_1af8c28c)] 
pub struct IpfixClassifyTableDetails { 
	pub context : u32, 
	pub table_id : u32, 
	pub ip_version : AddressFamily, 
	pub transport_protocol : IpProto, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipfix_flush_51077d14)] 
pub struct IpfixFlush { 
	pub client_index : u32, 
	pub context : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize, Message)] 
#[message_name_and_crc(ipfix_flush_reply_e8d4e804)] 
pub struct IpfixFlushReply { 
	pub context : u32, 
	pub retval : i32, 
} 
