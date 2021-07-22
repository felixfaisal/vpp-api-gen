/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
use typenum::{U10, U24, U256, U32, U64};
use crate::ip_types::*; 
use crate::ethernet_types::*; 
use crate::interface_types::*; 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpBridgeDomain { 
	pub bd_id : u32, 
	pub rd_id : u32, 
	pub flags : GbpBridgeDomainFlags, 
	pub bvi_sw_if_index : InterfaceIndex, 
	pub uu_fwd_sw_if_index : InterfaceIndex, 
	pub bm_flood_sw_if_index : InterfaceIndex, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpRouteDomain { 
	pub rd_id : u32, 
	pub ip4_table_id : u32, 
	pub ip6_table_id : u32, 
	pub ip4_uu_sw_if_index : InterfaceIndex, 
	pub ip6_uu_sw_if_index : InterfaceIndex, 
	pub scope : GbpScope, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpEndpointTun { 
	pub src : Address, 
	pub dst : Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpEndpoint { 
	pub sw_if_index : InterfaceIndex, 
	pub sclass : u16, 
	pub flags : GbpEndpointFlags, 
	pub mac : MacAddress, 
	pub tun : GbpEndpointTun, 
	pub n_ips : u8, 
	pub ips : Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpEndpointRetention { 
	pub remote_ep_timeout : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpEndpointGroup { 
	pub vnid : u32, 
	pub sclass : u16, 
	pub bd_id : u32, 
	pub rd_id : u32, 
	pub uplink_sw_if_index : InterfaceIndex, 
	pub retention : GbpEndpointRetention, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpRecirc { 
	pub sw_if_index : InterfaceIndex, 
	pub sclass : u16, 
	pub is_ext : bool, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpSubnet { 
	pub rd_id : u32, 
	pub sw_if_index : InterfaceIndex, 
	pub sclass : u16, 
	pub typ : GbpSubnetType, 
	pub prefix : Prefix, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpNextHop { 
	pub ip : Address, 
	pub mac : MacAddress, 
	pub bd_id : u32, 
	pub rd_id : u32, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpNextHopSet { 
	pub hash_mode : GbpHashMode, 
	pub n_nhs : u8, 
	pub nhs : GbpNextHop, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpRule { 
	pub action : GbpRuleAction, 
	pub nh_set : GbpNextHopSet, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpContract { 
	pub scope : GbpScope, 
	pub sclass : u16, 
	pub dclass : u16, 
	pub acl_index : u32, 
	pub n_ether_types : u8, 
	pub allowed_ethertypes : u16, 
	pub n_rules : u8, 
	pub rules : GbpRule, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpVxlanTunnel { 
	pub vni : u32, 
	pub mode : GbpVxlanTunnelMode, 
	pub bd_rd_id : u32, 
	pub src : Ip4Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpExtItf { 
	pub sw_if_index : InterfaceIndex, 
	pub bd_id : u32, 
	pub rd_id : u32, 
	pub flags : GbpExtItfFlags, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum GbpBridgeDomainFlags { 
	 GBP_BD_API_FLAG_NONE=0, 
	 GBP_BD_API_FLAG_DO_NOT_LEARN=1, 
	 GBP_BD_API_FLAG_UU_FWD_DROP=2, 
	 GBP_BD_API_FLAG_MCAST_DROP=4, 
	 GBP_BD_API_FLAG_UCAST_ARP=8, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum GbpEndpointFlags { 
	 GBP_API_ENDPOINT_FLAG_NONE=0, 
	 GBP_API_ENDPOINT_FLAG_BOUNCE=1, 
	 GBP_API_ENDPOINT_FLAG_REMOTE=2, 
	 GBP_API_ENDPOINT_FLAG_LEARNT=4, 
	 GBP_API_ENDPOINT_FLAG_EXTERNAL=8, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum GbpSubnetType { 
	 GBP_API_SUBNET_TRANSPORT=1, 
	 GBP_API_SUBNET_STITCHED_INTERNAL=2, 
	 GBP_API_SUBNET_STITCHED_EXTERNAL=3, 
	 GBP_API_SUBNET_L3_OUT=4, 
	 GBP_API_SUBNET_ANON_L3_OUT=5, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum GbpHashMode { 
	 GBP_API_HASH_MODE_SRC_IP=1, 
	 GBP_API_HASH_MODE_DST_IP=2, 
	 GBP_API_HASH_MODE_SYMMETRIC=3, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum GbpRuleAction { 
	 GBP_API_RULE_PERMIT=1, 
	 GBP_API_RULE_DENY=2, 
	 GBP_API_RULE_REDIRECT=3, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum GbpVxlanTunnelMode { 
	 GBP_VXLAN_TUNNEL_MODE_L2=1, 
	 GBP_VXLAN_TUNNEL_MODE_L3=2, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u32)]
pub enum GbpExtItfFlags { 
	 GBP_API_EXT_ITF_F_NONE=0, 
	 GBP_API_EXT_ITF_F_ANON=1, 
} 
pub type GbpScope=u16; 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpBridgeDomainAdd { 
	pub client_index : u32, 
	pub context : u32, 
	pub bd : GbpBridgeDomain, 
} 
impl GbpBridgeDomainAdd { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_bridge_domain_add_8454bfdf") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpBridgeDomainAddReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl GbpBridgeDomainAddReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_bridge_domain_add_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpBridgeDomainDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub bd_id : u32, 
} 
impl GbpBridgeDomainDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_bridge_domain_del_c25fdce6") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpBridgeDomainDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl GbpBridgeDomainDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_bridge_domain_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpBridgeDomainDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl GbpBridgeDomainDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_bridge_domain_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpBridgeDomainDumpReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl GbpBridgeDomainDumpReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_bridge_domain_dump_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpBridgeDomainDetails { 
	pub context : u32, 
	pub bd : GbpBridgeDomain, 
} 
impl GbpBridgeDomainDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_bridge_domain_details_2acd15f9") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpRouteDomainAdd { 
	pub client_index : u32, 
	pub context : u32, 
	pub rd : GbpRouteDomain, 
} 
impl GbpRouteDomainAdd { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_route_domain_add_2d0afe38") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpRouteDomainAddReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl GbpRouteDomainAddReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_route_domain_add_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpRouteDomainDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub rd_id : u32, 
} 
impl GbpRouteDomainDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_route_domain_del_bee4edcd") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpRouteDomainDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl GbpRouteDomainDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_route_domain_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpRouteDomainDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl GbpRouteDomainDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_route_domain_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpRouteDomainDumpReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl GbpRouteDomainDumpReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_route_domain_dump_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpRouteDomainDetails { 
	pub context : u32, 
	pub rd : GbpRouteDomain, 
} 
impl GbpRouteDomainDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_route_domain_details_8ab11375") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpEndpointAdd { 
	pub client_index : u32, 
	pub context : u32, 
	pub endpoint : GbpEndpoint, 
} 
impl GbpEndpointAdd { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_endpoint_add_9ce16d5a") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpEndpointAddReply { 
	pub context : u32, 
	pub retval : i32, 
	pub handle : u32, 
} 
impl GbpEndpointAddReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_endpoint_add_reply_1dd3ff3e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpEndpointDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub handle : u32, 
} 
impl GbpEndpointDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_endpoint_del_b93cd566") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpEndpointDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl GbpEndpointDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_endpoint_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpEndpointDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl GbpEndpointDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_endpoint_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpEndpointDetails { 
	pub context : u32, 
	pub age : f64, 
	pub handle : u32, 
	pub endpoint : GbpEndpoint, 
} 
impl GbpEndpointDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_endpoint_details_08aecb60") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpEndpointGroupAdd { 
	pub client_index : u32, 
	pub context : u32, 
	pub epg : GbpEndpointGroup, 
} 
impl GbpEndpointGroupAdd { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_endpoint_group_add_8e0f4054") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpEndpointGroupAddReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl GbpEndpointGroupAddReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_endpoint_group_add_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpEndpointGroupDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub sclass : u16, 
} 
impl GbpEndpointGroupDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_endpoint_group_del_3436b8b7") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpEndpointGroupDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl GbpEndpointGroupDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_endpoint_group_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpEndpointGroupDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl GbpEndpointGroupDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_endpoint_group_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpEndpointGroupDetails { 
	pub context : u32, 
	pub epg : GbpEndpointGroup, 
} 
impl GbpEndpointGroupDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_endpoint_group_details_8f38292c") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpRecircAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub recirc : GbpRecirc, 
} 
impl GbpRecircAddDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_recirc_add_del_718c69c3") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpRecircAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl GbpRecircAddDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_recirc_add_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpRecircDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl GbpRecircDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_recirc_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpRecircDetails { 
	pub context : u32, 
	pub recirc : GbpRecirc, 
} 
impl GbpRecircDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_recirc_details_66ecc42e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpSubnetAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub subnet : GbpSubnet, 
} 
impl GbpSubnetAddDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_subnet_add_del_888aca35") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpSubnetAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl GbpSubnetAddDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_subnet_add_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpSubnetDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl GbpSubnetDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_subnet_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpSubnetDetails { 
	pub context : u32, 
	pub subnet : GbpSubnet, 
} 
impl GbpSubnetDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_subnet_details_4ed84156") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpContractAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub contract : GbpContract, 
} 
impl GbpContractAddDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_contract_add_del_553e275b") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpContractAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
	pub stats_index : u32, 
} 
impl GbpContractAddDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_contract_add_del_reply_1992deab") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpContractDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl GbpContractDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_contract_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpContractDetails { 
	pub context : u32, 
	pub contract : GbpContract, 
} 
impl GbpContractDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_contract_details_2a18db6e") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpVxlanTunnelAdd { 
	pub client_index : u32, 
	pub context : u32, 
	pub tunnel : GbpVxlanTunnel, 
} 
impl GbpVxlanTunnelAdd { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_vxlan_tunnel_add_3e070b35") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpVxlanTunnelAddReply { 
	pub context : u32, 
	pub retval : i32, 
	pub sw_if_index : InterfaceIndex, 
} 
impl GbpVxlanTunnelAddReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_vxlan_tunnel_add_reply_5383d31f") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpVxlanTunnelDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub vni : u32, 
} 
impl GbpVxlanTunnelDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_vxlan_tunnel_del_8d1f2fe9") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpVxlanTunnelDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl GbpVxlanTunnelDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_vxlan_tunnel_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpVxlanTunnelDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl GbpVxlanTunnelDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_vxlan_tunnel_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpVxlanTunnelDetails { 
	pub context : u32, 
	pub tunnel : GbpVxlanTunnel, 
} 
impl GbpVxlanTunnelDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_vxlan_tunnel_details_65c6c818") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpExtItfAddDel { 
	pub client_index : u32, 
	pub context : u32, 
	pub is_add : bool, 
	pub ext_itf : GbpExtItf, 
} 
impl GbpExtItfAddDel { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_ext_itf_add_del_12ed5700") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpExtItfAddDelReply { 
	pub context : u32, 
	pub retval : i32, 
} 
impl GbpExtItfAddDelReply { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_ext_itf_add_del_reply_e8d4e804") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpExtItfDump { 
	pub client_index : u32, 
	pub context : u32, 
} 
impl GbpExtItfDump { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_ext_itf_dump_51077d14") 
	 } 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct GbpExtItfDetails { 
	pub context : u32, 
	pub ext_itf : GbpExtItf, 
} 
impl GbpExtItfDetails { 
	 pub fn get_message_id() -> String { 
	 	 String::from("gbp_ext_itf_details_408a45c0") 
	 } 
} 