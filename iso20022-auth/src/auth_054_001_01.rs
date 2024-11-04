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


// CCPClearingMemberReportV01 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CCPClearingMemberReportV01 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrMmb") )]
	pub clr_mmb: Vec<ClearingMember1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CCPClearingMemberReportV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.clr_mmb { item.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// ClearingAccount1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ClearingAccount1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctTp") )]
	pub acct_tp: ClearingAccountType3Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollAcctOwnr") )]
	pub coll_acct_ownr: Vec<CollateralAccount5>,
}

impl ClearingAccount1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.acct_tp.validate()?;
		for item in &self.coll_acct_ownr { item.validate()? }
		Ok(())
	}
}


// ClearingAccountType3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ClearingAccountType3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NOSA") )]
	CodeNOSA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISEG") )]
	CodeISEG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HOUS") )]
	CodeHOUS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GOSA") )]
	CodeGOSA,
}

impl ClearingAccountType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ClearingMember1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ClearingMember1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: PartyIdentification118Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtQlty") )]
	pub cdt_qlty: CreditQuality1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtPrntId", skip_serializing_if = "Option::is_none") )]
	pub ultmt_prnt_id: Option<PartyIdentification118Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FutrsComssnMrchntInd") )]
	pub futrs_comssn_mrchnt_ind: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MmbshVldFr") )]
	pub mmbsh_vld_fr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MmbshVldTo", skip_serializing_if = "Option::is_none") )]
	pub mmbsh_vld_to: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SpnsrgClrMmbId", skip_serializing_if = "Option::is_none") )]
	pub spnsrg_clr_mmb_id: Option<PartyIdentification118Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrAcctOwnr") )]
	pub clr_acct_ownr: Vec<ClearingAccount1>,
}

impl ClearingMember1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		self.cdt_qlty.validate()?;
		if let Some(ref val) = self.ultmt_prnt_id { val.validate()? }
		if let Some(ref val) = self.spnsrg_clr_mmb_id { val.validate()? }
		for item in &self.clr_acct_ownr { item.validate()? }
		Ok(())
	}
}


// CollateralAccount5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CollateralAccount5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: PartyIdentification118Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdMrgnAcct") )]
	pub rltd_mrgn_acct: Vec<MarginAccount1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TitlTrfCollArrgmnt", skip_serializing_if = "Option::is_none") )]
	pub titl_trf_coll_arrgmnt: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollSgrtnByVal", skip_serializing_if = "Option::is_none") )]
	pub coll_sgrtn_by_val: Option<bool>,
}

impl CollateralAccount5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		for item in &self.rltd_mrgn_acct { item.validate()? }
		Ok(())
	}
}


// CreditQuality1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum CreditQuality1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DFIM") )]
	CodeDFIM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EXSP") )]
	CodeEXSP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HIGR") )]
	CodeHIGR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HISP") )]
	CodeHISP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INDF") )]
	CodeINDF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LMGR") )]
	CodeLMGR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NIGS") )]
	CodeNIGS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRIM") )]
	CodePRIM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SURI") )]
	CodeSURI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UMGR") )]
	CodeUMGR,
}

impl CreditQuality1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// GenericIdentification168 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericIdentification168 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<String>,
}

impl GenericIdentification168 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 256 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 256".to_string()));
		}
		if let Some(ref val) = self.desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.schme_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "schme_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "schme_nm exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// MarginAccount1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MarginAccount1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: PartyIdentification118Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PosAcct") )]
	pub pos_acct: Vec<PositionAccount1>,
}

impl MarginAccount1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		for item in &self.pos_acct { item.validate()? }
		Ok(())
	}
}


// PartyIdentification118Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyIdentification118Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification168>,
}

impl PartyIdentification118Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// PositionAccount1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PositionAccount1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: PartyIdentification118Choice,
}

impl PositionAccount1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
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
		self.envlp.validate()?;
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
