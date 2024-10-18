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

use serde::{Deserialize, Serialize};
use regex::Regex;


// DataModification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum DataModification1Code {
	#[default]
	#[serde(rename = "INSE")]
	CodeINSE,
	#[serde(rename = "UPDT")]
	CodeUPDT,
	#[serde(rename = "DELT")]
	CodeDELT,
}

impl DataModification1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "$value")]
	pub exact4_alpha_numeric_text: String,
}

impl Exact4AlphaNumericText {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
		if !pattern.is_match(&self.exact4_alpha_numeric_text) {
			return false
		}
		return true
	}
}


// GenericIdentification30 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification30 {
	#[serde(rename = "Id")]
	pub id: Exact4AlphaNumericText,
	#[serde(rename = "Issr")]
	pub issr: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
}

impl GenericIdentification30 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.issr.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		return true
	}
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}

impl ISODate {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}

impl ISODateTime {
	pub fn validate(&self) -> bool {
		return true
	}
}


// MarketSpecificAttribute1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketSpecificAttribute1 {
	#[serde(rename = "Nm")]
	pub nm: Max35Text,
	#[serde(rename = "Val")]
	pub val: Max350Text,
}

impl MarketSpecificAttribute1 {
	pub fn validate(&self) -> bool {
		if !self.nm.validate() { return false }
		if !self.val.validate() { return false }
		return true
	}
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}

impl Max350Text {
	pub fn validate(&self) -> bool {
		if self.max350_text.chars().count() < 1 {
			return false
		}
		if self.max350_text.chars().count() > 350 {
			return false
		}
		return true
	}
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}

impl Max35Text {
	pub fn validate(&self) -> bool {
		if self.max35_text.chars().count() < 1 {
			return false
		}
		if self.max35_text.chars().count() > 35 {
			return false
		}
		return true
	}
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
}

impl Max70Text {
	pub fn validate(&self) -> bool {
		if self.max70_text.chars().count() < 1 {
			return false
		}
		if self.max70_text.chars().count() > 70 {
			return false
		}
		return true
	}
}


// MessageHeader1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader1 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
}

impl MessageHeader1 {
	pub fn validate(&self) -> bool {
		if !self.msg_id.validate() { return false }
		return true
	}
}


// SecuritiesAccount19 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesAccount19 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<GenericIdentification30>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max70Text>,
}

impl SecuritiesAccount19 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		return true
	}
}


// SecuritiesAccountModification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesAccountModification2 {
	#[serde(rename = "ScpIndctn")]
	pub scp_indctn: DataModification1Code,
	#[serde(rename = "ReqdMod")]
	pub reqd_mod: SecuritiesAccountModification2Choice,
}

impl SecuritiesAccountModification2 {
	pub fn validate(&self) -> bool {
		if !self.scp_indctn.validate() { return false }
		if !self.reqd_mod.validate() { return false }
		return true
	}
}


// SecuritiesAccountModification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesAccountModification2Choice {
	#[serde(rename = "SysSctiesAcct", skip_serializing_if = "Option::is_none")]
	pub sys_scties_acct: Option<SystemSecuritiesAccount5>,
	#[serde(rename = "SysRstrctn", skip_serializing_if = "Option::is_none")]
	pub sys_rstrctn: Option<SystemRestriction1>,
	#[serde(rename = "MktSpcfcAttr", skip_serializing_if = "Option::is_none")]
	pub mkt_spcfc_attr: Option<MarketSpecificAttribute1>,
}

impl SecuritiesAccountModification2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref sys_scties_acct_value) = self.sys_scties_acct { if !sys_scties_acct_value.validate() { return false; } }
		if let Some(ref sys_rstrctn_value) = self.sys_rstrctn { if !sys_rstrctn_value.validate() { return false; } }
		if let Some(ref mkt_spcfc_attr_value) = self.mkt_spcfc_attr { if !mkt_spcfc_attr_value.validate() { return false; } }
		return true
	}
}


// SecuritiesAccountModificationRequestV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesAccountModificationRequestV01 {
	#[serde(rename = "MsgHdr", skip_serializing_if = "Option::is_none")]
	pub msg_hdr: Option<MessageHeader1>,
	#[serde(rename = "AcctId")]
	pub acct_id: SecuritiesAccount19,
	#[serde(rename = "Mod")]
	pub mod_attr: Vec<SecuritiesAccountModification2>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl SecuritiesAccountModificationRequestV01 {
	pub fn validate(&self) -> bool {
		if let Some(ref msg_hdr_value) = self.msg_hdr { if !msg_hdr_value.validate() { return false; } }
		if !self.acct_id.validate() { return false }
		for item in &self.mod_attr { if !item.validate() { return false; } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
	}
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
	pub plc_and_nm: Option<Max350Text>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}

impl SupplementaryData1 {
	pub fn validate(&self) -> bool {
		if let Some(ref plc_and_nm_value) = self.plc_and_nm { if !plc_and_nm_value.validate() { return false; } }
		if !self.envlp.validate() { return false }
		return true
	}
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// SystemRestriction1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemRestriction1 {
	#[serde(rename = "VldFr")]
	pub vld_fr: String,
	#[serde(rename = "VldTo", skip_serializing_if = "Option::is_none")]
	pub vld_to: Option<String>,
	#[serde(rename = "Tp")]
	pub tp: Max35Text,
}

impl SystemRestriction1 {
	pub fn validate(&self) -> bool {
		if !self.tp.validate() { return false }
		return true
	}
}


// SystemSecuritiesAccount5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemSecuritiesAccount5 {
	#[serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none")]
	pub clsg_dt: Option<String>,
	#[serde(rename = "HldInd", skip_serializing_if = "Option::is_none")]
	pub hld_ind: Option<bool>,
	#[serde(rename = "NegPos", skip_serializing_if = "Option::is_none")]
	pub neg_pos: Option<bool>,
	#[serde(rename = "EndInvstrFlg", skip_serializing_if = "Option::is_none")]
	pub end_invstr_flg: Option<Exact4AlphaNumericText>,
	#[serde(rename = "PricgSchme", skip_serializing_if = "Option::is_none")]
	pub pricg_schme: Option<Exact4AlphaNumericText>,
}

impl SystemSecuritiesAccount5 {
	pub fn validate(&self) -> bool {
		if let Some(ref end_invstr_flg_value) = self.end_invstr_flg { if !end_invstr_flg_value.validate() { return false; } }
		if let Some(ref pricg_schme_value) = self.pricg_schme { if !pricg_schme_value.validate() { return false; } }
		return true
	}
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}

impl TrueFalseIndicator {
	pub fn validate(&self) -> bool {
		return true
	}
}
