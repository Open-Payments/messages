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


// CCPClearingMemberReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CCPClearingMemberReportV01 {
	#[serde(rename = "ClrMmb")]
	pub clr_mmb: Vec<ClearingMember1>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CCPClearingMemberReportV01 {
	pub fn validate(&self) -> bool {
		for item in &self.clr_mmb { if !item.validate() { return false; } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
	}
}


// ClearingAccount1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingAccount1 {
	#[serde(rename = "AcctTp")]
	pub acct_tp: ClearingAccountType3Code,
	#[serde(rename = "CollAcctOwnr")]
	pub coll_acct_ownr: Vec<CollateralAccount5>,
}

impl ClearingAccount1 {
	pub fn validate(&self) -> bool {
		if !self.acct_tp.validate() { return false }
		for item in &self.coll_acct_ownr { if !item.validate() { return false; } }
		return true
	}
}


// ClearingAccountType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ClearingAccountType3Code {
	#[default]
	#[serde(rename = "NOSA")]
	CodeNOSA,
	#[serde(rename = "ISEG")]
	CodeISEG,
	#[serde(rename = "HOUS")]
	CodeHOUS,
	#[serde(rename = "GOSA")]
	CodeGOSA,
}

impl ClearingAccountType3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ClearingMember1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingMember1 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification118Choice,
	#[serde(rename = "CdtQlty")]
	pub cdt_qlty: CreditQuality1Code,
	#[serde(rename = "UltmtPrntId", skip_serializing_if = "Option::is_none")]
	pub ultmt_prnt_id: Option<PartyIdentification118Choice>,
	#[serde(rename = "FutrsComssnMrchntInd")]
	pub futrs_comssn_mrchnt_ind: bool,
	#[serde(rename = "MmbshVldFr")]
	pub mmbsh_vld_fr: String,
	#[serde(rename = "MmbshVldTo", skip_serializing_if = "Option::is_none")]
	pub mmbsh_vld_to: Option<String>,
	#[serde(rename = "SpnsrgClrMmbId", skip_serializing_if = "Option::is_none")]
	pub spnsrg_clr_mmb_id: Option<PartyIdentification118Choice>,
	#[serde(rename = "ClrAcctOwnr")]
	pub clr_acct_ownr: Vec<ClearingAccount1>,
}

impl ClearingMember1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.cdt_qlty.validate() { return false }
		if let Some(ref ultmt_prnt_id_value) = self.ultmt_prnt_id { if !ultmt_prnt_id_value.validate() { return false; } }
		if let Some(ref spnsrg_clr_mmb_id_value) = self.spnsrg_clr_mmb_id { if !spnsrg_clr_mmb_id_value.validate() { return false; } }
		for item in &self.clr_acct_ownr { if !item.validate() { return false; } }
		return true
	}
}


// CollateralAccount5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralAccount5 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification118Choice,
	#[serde(rename = "RltdMrgnAcct")]
	pub rltd_mrgn_acct: Vec<MarginAccount1>,
	#[serde(rename = "TitlTrfCollArrgmnt", skip_serializing_if = "Option::is_none")]
	pub titl_trf_coll_arrgmnt: Option<bool>,
	#[serde(rename = "CollSgrtnByVal", skip_serializing_if = "Option::is_none")]
	pub coll_sgrtn_by_val: Option<bool>,
}

impl CollateralAccount5 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		for item in &self.rltd_mrgn_acct { if !item.validate() { return false; } }
		return true
	}
}


// CreditQuality1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CreditQuality1Code {
	#[default]
	#[serde(rename = "DFIM")]
	CodeDFIM,
	#[serde(rename = "EXSP")]
	CodeEXSP,
	#[serde(rename = "HIGR")]
	CodeHIGR,
	#[serde(rename = "HISP")]
	CodeHISP,
	#[serde(rename = "INDF")]
	CodeINDF,
	#[serde(rename = "LMGR")]
	CodeLMGR,
	#[serde(rename = "NIGS")]
	CodeNIGS,
	#[serde(rename = "PRIM")]
	CodePRIM,
	#[serde(rename = "SURI")]
	CodeSURI,
	#[serde(rename = "UMGR")]
	CodeUMGR,
}

impl CreditQuality1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// GenericIdentification168 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification168 {
	#[serde(rename = "Id")]
	pub id: Max256Text,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max140Text>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
}

impl GenericIdentification168 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref desc_value) = self.desc { if !desc_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
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


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}

impl LEIIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.lei_identifier) {
			return false
		}
		return true
	}
}


// MarginAccount1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarginAccount1 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification118Choice,
	#[serde(rename = "PosAcct")]
	pub pos_acct: Vec<PositionAccount1>,
}

impl MarginAccount1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		for item in &self.pos_acct { if !item.validate() { return false; } }
		return true
	}
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
}

impl Max140Text {
	pub fn validate(&self) -> bool {
		if self.max140_text.chars().count() < 1 {
			return false
		}
		if self.max140_text.chars().count() > 140 {
			return false
		}
		return true
	}
}


// Max256Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max256Text {
	#[serde(rename = "$value")]
	pub max256_text: String,
}

impl Max256Text {
	pub fn validate(&self) -> bool {
		if self.max256_text.chars().count() < 1 {
			return false
		}
		if self.max256_text.chars().count() > 256 {
			return false
		}
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


// PartyIdentification118Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification118Choice {
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification168>,
}

impl PartyIdentification118Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref lei_value) = self.lei { if !lei_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// PositionAccount1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionAccount1 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification118Choice,
}

impl PositionAccount1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
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
