/* Autogenerated data. Do not edit */
#![allow(non_camel_case_types)]
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use vpp_api_encoding::typ::*;
use vpp_api_transport::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum SrPolicyOp { 
	 SR_POLICY_OP_API_NONE=0, 
	 SR_POLICY_OP_API_ADD=1, 
	 SR_POLICY_OP_API_DEL=2, 
	 SR_POLICY_OP_API_MOD=3, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum SrBehavior { 
	 SR_BEHAVIOR_API_END=1, 
	 SR_BEHAVIOR_API_X=2, 
	 SR_BEHAVIOR_API_T=3, 
	 SR_BEHAVIOR_API_D_FIRST=4, 
	 SR_BEHAVIOR_API_DX2=5, 
	 SR_BEHAVIOR_API_DX6=6, 
	 SR_BEHAVIOR_API_DX4=7, 
	 SR_BEHAVIOR_API_DT6=8, 
	 SR_BEHAVIOR_API_DT4=9, 
	 SR_BEHAVIOR_API_LAST=10, 
} 
#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)] 
#[repr(u8)]
pub enum SrSteer { 
	 SR_STEER_API_L2=2, 
	 SR_STEER_API_IPV4=4, 
	 SR_STEER_API_IPV6=6, 
} 