// Open Payment Message Parsing Library
// https://github.com/Open-Payments/messages
//
// This library is designed to parse message formats based on the ISO 20022 standards,
// including but not limited to FedNow messages. It supports various financial message types,
// such as customer credit transfers, payment status reports, administrative notifications, 
// and other ISO 20022 messages, using Serde for efficient serialization and deserialization.
//
// Copyright (c) 2024 Open Payments by Harishankar Narayanan
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// You may obtain a copy of this library at
// https://github.com/Open-Payments/messages


use regex::Regex;
use crate::common::*;
#[cfg(feature = "derive_serde")]
use serde::{Deserialize, Serialize};


// ActiveCurrencyAndAmount ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ActiveCurrencyAndAmount {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
	pub value: f64,
}

impl ActiveCurrencyAndAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AddressType2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum AddressType2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ADDR") )]
	CodeADDR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PBOX") )]
	CodePBOX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HOME") )]
	CodeHOME,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BIZZ") )]
	CodeBIZZ,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MLTO") )]
	CodeMLTO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DLVY") )]
	CodeDLVY,
}

impl AddressType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ClearingSystemIdentification2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ClearingSystemIdentification2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ClearingSystemIdentification2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 5 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 5".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// NameAndAddress8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NameAndAddress8 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Adr", skip_serializing_if = "Option::is_none") )]
	pub adr: Option<PostalAddress1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AltrntvIdr", skip_serializing_if = "Option::is_none") )]
	pub altrntv_idr: Option<Vec<String>>,
}

impl NameAndAddress8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 350 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
		}
		if let Some(ref val) = self.adr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.altrntv_idr {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "altrntv_idr is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 35 {
					return Err(ValidationError::new(1002, "altrntv_idr exceeds the maximum length of 35".to_string()));
				}
			}
		}
		Ok(())
	}
}


// NetObligation2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NetObligation2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OblgtnId") )]
	pub oblgtn_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtcptNetgId") )]
	pub ptcpt_netg_id: NettingIdentification2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OblgtnDrctn") )]
	pub oblgtn_drctn: PaymentReceipt1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtyNetgId") )]
	pub ctr_pty_netg_id: NettingIdentification2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetSvcCtrPtyId", skip_serializing_if = "Option::is_none") )]
	pub net_svc_ctr_pty_id: Option<PartyIdentification242Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtySttlmInstrs", skip_serializing_if = "Option::is_none") )]
	pub ctr_pty_sttlm_instrs: Option<SettlementParties120>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxsNb", skip_serializing_if = "Option::is_none") )]
	pub txs_nb: Option<String>,
}

impl NetObligation2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.oblgtn_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "oblgtn_id is shorter than the minimum length of 1".to_string()));
		}
		if self.oblgtn_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "oblgtn_id exceeds the maximum length of 35".to_string()));
		}
		if let Err(e) = self.amt.validate() { return Err(e); }
		if let Err(e) = self.ptcpt_netg_id.validate() { return Err(e); }
		if let Err(e) = self.oblgtn_drctn.validate() { return Err(e); }
		if let Err(e) = self.ctr_pty_netg_id.validate() { return Err(e); }
		if let Some(ref val) = self.net_svc_ctr_pty_id { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.ctr_pty_sttlm_instrs { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.txs_nb {
			let pattern = Regex::new("[0-9]{1,10}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "txs_nb does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// NetReportData2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NetReportData2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
	pub cre_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetgCutOffTm") )]
	pub netg_cut_off_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptDt") )]
	pub rpt_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValDt") )]
	pub val_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptTp", skip_serializing_if = "Option::is_none") )]
	pub rpt_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetRptSvcr", skip_serializing_if = "Option::is_none") )]
	pub net_rpt_svcr: Option<PartyIdentification242Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetSvcTp", skip_serializing_if = "Option::is_none") )]
	pub net_svc_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgPgntn", skip_serializing_if = "Option::is_none") )]
	pub msg_pgntn: Option<Pagination1>,
}

impl NetReportData2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.rpt_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rpt_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "rpt_tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.net_rpt_svcr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.net_svc_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "net_svc_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "net_svc_tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.msg_pgntn { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// NetReportV02 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NetReportV02 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetRptData") )]
	pub net_rpt_data: NetReportData2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetSvcPtcptId") )]
	pub net_svc_ptcpt_id: PartyIdentification242Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetSvcCtrPtyId", skip_serializing_if = "Option::is_none") )]
	pub net_svc_ctr_pty_id: Option<PartyIdentification242Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetOblgtn") )]
	pub net_oblgtn: Vec<NetObligation2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl NetReportV02 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.net_rpt_data.validate() { return Err(e); }
		if let Err(e) = self.net_svc_ptcpt_id.validate() { return Err(e); }
		if let Some(ref val) = self.net_svc_ctr_pty_id { if let Err(e) = val.validate() { return Err(e); } }
		for item in &self.net_oblgtn { if let Err(e) = item.validate() { return Err(e); } }
		if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// NettingIdentification2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NettingIdentification2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradPty", skip_serializing_if = "Option::is_none") )]
	pub trad_pty: Option<PartyIdentification242Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetgGrpId", skip_serializing_if = "Option::is_none") )]
	pub netg_grp_id: Option<String>,
}

impl NettingIdentification2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.trad_pty { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.netg_grp_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "netg_grp_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "netg_grp_id exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// Pagination1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Pagination1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PgNb") )]
	pub pg_nb: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LastPgInd") )]
	pub last_pg_ind: bool,
}

impl Pagination1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{1,5}").unwrap();
		if !pattern.is_match(&self.pg_nb) {
			return Err(ValidationError::new(1005, "pg_nb does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// PartyIdentification242Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyIdentification242Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
	pub nm_and_adr: Option<NameAndAddress8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
	pub any_bic: Option<PartyIdentification265>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyId", skip_serializing_if = "Option::is_none") )]
	pub pty_id: Option<PartyIdentification266>,
}

impl PartyIdentification242Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm_and_adr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.any_bic { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.pty_id { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// PartyIdentification265 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyIdentification265 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC") )]
	pub any_bic: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AltrntvIdr", skip_serializing_if = "Option::is_none") )]
	pub altrntv_idr: Option<Vec<String>>,
}

impl PartyIdentification265 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
		if !pattern.is_match(&self.any_bic) {
			return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
		}
		if let Some(ref vec) = self.altrntv_idr {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "altrntv_idr is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 35 {
					return Err(ValidationError::new(1002, "altrntv_idr exceeds the maximum length of 35".to_string()));
				}
			}
		}
		Ok(())
	}
}


// PartyIdentification266 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyIdentification266 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyNm", skip_serializing_if = "Option::is_none") )]
	pub pty_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
	pub any_bic: Option<PartyIdentification265>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctNb", skip_serializing_if = "Option::is_none") )]
	pub acct_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Adr", skip_serializing_if = "Option::is_none") )]
	pub adr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysId", skip_serializing_if = "Option::is_none") )]
	pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none") )]
	pub lgl_ntty_idr: Option<String>,
}

impl PartyIdentification266 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pty_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pty_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 34 {
				return Err(ValidationError::new(1002, "pty_nm exceeds the maximum length of 34".to_string()));
			}
		}
		if let Some(ref val) = self.any_bic { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.acct_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 34 {
				return Err(ValidationError::new(1002, "acct_nb exceeds the maximum length of 34".to_string()));
			}
		}
		if let Some(ref val) = self.adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 105 {
				return Err(ValidationError::new(1002, "adr exceeds the maximum length of 105".to_string()));
			}
		}
		if let Some(ref val) = self.clr_sys_id { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.lgl_ntty_idr {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "lgl_ntty_idr does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// PaymentReceipt1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum PaymentReceipt1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "PAYM") )]
	CodePAYM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RECE") )]
	CodeRECE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NONE") )]
	CodeNONE,
}

impl PaymentReceipt1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PostalAddress1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PostalAddress1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdrTp", skip_serializing_if = "Option::is_none") )]
	pub adr_tp: Option<AddressType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdrLine", skip_serializing_if = "Option::is_none") )]
	pub adr_line: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StrtNm", skip_serializing_if = "Option::is_none") )]
	pub strt_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNb", skip_serializing_if = "Option::is_none") )]
	pub bldg_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstCd", skip_serializing_if = "Option::is_none") )]
	pub pst_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TwnNm", skip_serializing_if = "Option::is_none") )]
	pub twn_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none") )]
	pub ctry_sub_dvsn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
	pub ctry: String,
}

impl PostalAddress1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.adr_tp { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.adr_line {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "adr_line is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 70 {
					return Err(ValidationError::new(1002, "adr_line exceeds the maximum length of 70".to_string()));
				}
			}
		}
		if let Some(ref val) = self.strt_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "strt_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "strt_nm exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.bldg_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "bldg_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "bldg_nb exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.pst_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pst_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "pst_cd exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.twn_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "twn_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "twn_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ctry_sub_dvsn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ctry_sub_dvsn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ctry_sub_dvsn exceeds the maximum length of 35".to_string()));
			}
		}
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.ctry) {
			return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// SettlementParties120 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementParties120 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DlvryAgt", skip_serializing_if = "Option::is_none") )]
	pub dlvry_agt: Option<PartyIdentification242Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Intrmy", skip_serializing_if = "Option::is_none") )]
	pub intrmy: Option<PartyIdentification242Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RcvgAgt") )]
	pub rcvg_agt: PartyIdentification242Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BnfcryInstn", skip_serializing_if = "Option::is_none") )]
	pub bnfcry_instn: Option<PartyIdentification242Choice>,
}

impl SettlementParties120 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dlvry_agt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.intrmy { if let Err(e) = val.validate() { return Err(e); } }
		if let Err(e) = self.rcvg_agt.validate() { return Err(e); }
		if let Some(ref val) = self.bnfcry_instn { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// SupplementaryData1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SupplementaryData1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none") )]
	pub plc_and_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Envlp") )]
	pub envlp: SupplementaryDataEnvelope1,
}

impl SupplementaryData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.plc_and_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "plc_and_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "plc_and_nm exceeds the maximum length of 350".to_string()));
			}
		}
		if let Err(e) = self.envlp.validate() { return Err(e); }
		Ok(())
	}
}


// SupplementaryDataEnvelope1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}
