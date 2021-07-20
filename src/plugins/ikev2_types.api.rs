/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2Id { 
	pub typ : u8, 
	pub data_len : u8, 
	pub data : String, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2Ts { 
	pub sa_index : u32, 
	pub child_sa_index : u32, 
	pub is_local : bool, 
	pub protocol_id : u8, 
	pub start_port : u16, 
	pub end_port : u16, 
	pub start_addr : Address, 
	pub end_addr : Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2Auth { 
	pub method : u8, 
	pub hex : u8, 
	pub data_len : u32, 
	pub data : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2Responder { 
	pub sw_if_index : InterfaceIndex, 
	pub addr : Address, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2IkeTransforms { 
	pub crypto_alg : u8, 
	pub crypto_key_size : u32, 
	pub integ_alg : u8, 
	pub dh_group : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2EspTransforms { 
	pub crypto_alg : u8, 
	pub crypto_key_size : u32, 
	pub integ_alg : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2Profile { 
	pub name : String, 
	pub loc_id : Ikev2Id, 
	pub rem_id : Ikev2Id, 
	pub loc_ts : Ikev2Ts, 
	pub rem_ts : Ikev2Ts, 
	pub responder : Ikev2Responder, 
	pub ike_ts : Ikev2IkeTransforms, 
	pub esp_ts : Ikev2EspTransforms, 
	pub lifetime : u64, 
	pub lifetime_maxdata : u64, 
	pub lifetime_jitter : u32, 
	pub handover : u32, 
	pub ipsec_over_udp_port : u16, 
	pub tun_itf : u32, 
	pub udp_encap : bool, 
	pub natt_disabled : bool, 
	pub auth : Ikev2Auth, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2SaTransform { 
	pub transform_type : u8, 
	pub transform_id : u16, 
	pub key_len : u16, 
	pub key_trunc : u16, 
	pub block_size : u16, 
	pub dh_group : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2Keys { 
	pub sk_d : u8, 
	pub sk_d_len : u8, 
	pub sk_ai : u8, 
	pub sk_ai_len : u8, 
	pub sk_ar : u8, 
	pub sk_ar_len : u8, 
	pub sk_ei : u8, 
	pub sk_ei_len : u8, 
	pub sk_er : u8, 
	pub sk_er_len : u8, 
	pub sk_pi : u8, 
	pub sk_pi_len : u8, 
	pub sk_pr : u8, 
	pub sk_pr_len : u8, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2ChildSa { 
	pub sa_index : u32, 
	pub child_sa_index : u32, 
	pub i_spi : u32, 
	pub r_spi : u32, 
	pub keys : Ikev2Keys, 
	pub encryption : Ikev2SaTransform, 
	pub integrity : Ikev2SaTransform, 
	pub esn : Ikev2SaTransform, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2SaStats { 
	pub n_keepalives : u16, 
	pub n_rekey_req : u16, 
	pub n_sa_init_req : u16, 
	pub n_sa_auth_req : u16, 
	pub n_retransmit : u16, 
	pub n_init_sa_retransmit : u16, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct Ikev2Sa { 
	pub sa_index : u32, 
	pub profile_index : u32, 
	pub ispi : u64, 
	pub rspi : u64, 
	pub iaddr : Address, 
	pub raddr : Address, 
	pub keys : Ikev2Keys, 
	pub i_id : Ikev2Id, 
	pub r_id : Ikev2Id, 
	pub encryption : Ikev2SaTransform, 
	pub integrity : Ikev2SaTransform, 
	pub prf : Ikev2SaTransform, 
	pub dh : Ikev2SaTransform, 
	pub stats : Ikev2SaStats, 
} 
#[derive(Debug, Clone, Serialize, Deserialize)] 
union address_union { 
	 ip4 : Ip4Address, 
	 ip6 : Ip6Address, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum AddressFamily { 
	 ADDRESS_IP4=0, 
	 ADDRESS_IP6=1, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum IpFeatureLocation { 
	 IP_API_FEATURE_INPUT=0, 
	 IP_API_FEATURE_OUTPUT=1, 
	 IP_API_FEATURE_LOCAL=2, 
	 IP_API_FEATURE_PUNT=3, 
	 IP_API_FEATURE_DROP=4, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum IpEcn { 
	 IP_API_ECN_NONE=0, 
	 IP_API_ECN_ECT0=1, 
	 IP_API_ECN_ECT1=2, 
	 IP_API_ECN_CE=3, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum IpDscp { 
	 IP_API_DSCP_CS0=0, 
	 IP_API_DSCP_CS1=8, 
	 IP_API_DSCP_AF11=10, 
	 IP_API_DSCP_AF12=12, 
	 IP_API_DSCP_AF13=14, 
	 IP_API_DSCP_CS2=16, 
	 IP_API_DSCP_AF21=18, 
	 IP_API_DSCP_AF22=20, 
	 IP_API_DSCP_AF23=22, 
	 IP_API_DSCP_CS3=24, 
	 IP_API_DSCP_AF31=26, 
	 IP_API_DSCP_AF32=28, 
	 IP_API_DSCP_AF33=30, 
	 IP_API_DSCP_CS4=32, 
	 IP_API_DSCP_AF41=34, 
	 IP_API_DSCP_AF42=36, 
	 IP_API_DSCP_AF43=38, 
	 IP_API_DSCP_CS5=40, 
	 IP_API_DSCP_EF=46, 
	 IP_API_DSCP_CS6=48, 
	 IP_API_DSCP_CS7=50, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum IpProto { 
	 IP_API_PROTO_HOPOPT=0, 
	 IP_API_PROTO_ICMP=1, 
	 IP_API_PROTO_IGMP=2, 
	 IP_API_PROTO_TCP=6, 
	 IP_API_PROTO_UDP=17, 
	 IP_API_PROTO_GRE=47, 
	 IP_API_PROTO_ESP=50, 
	 IP_API_PROTO_AH=51, 
	 IP_API_PROTO_ICMP6=58, 
	 IP_API_PROTO_EIGRP=88, 
	 IP_API_PROTO_OSPF=89, 
	 IP_API_PROTO_SCTP=132, 
	 IP_API_PROTO_RESERVED=255, 
} 
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