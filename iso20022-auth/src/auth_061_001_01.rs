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


// ActiveCurrencyAnd24Amount ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ActiveCurrencyAnd24Amount {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
	pub value: f64,
}

impl ActiveCurrencyAnd24Amount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


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


// CCPInvestmentsReportV01 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CCPInvestmentsReportV01 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Invstmt") )]
	pub invstmt: Vec<Investment1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CCPInvestmentsReportV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.invstmt { if let Err(e) = item.validate() { return Err(e); } }
		if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// Deposit1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Deposit1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MtrtyDt") )]
	pub mtrty_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
	pub val: ActiveCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtyId") )]
	pub ctr_pty_id: String,
}

impl Deposit1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.val.validate() { return Err(e); }
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.ctr_pty_id) {
			return Err(ValidationError::new(1005, "ctr_pty_id does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// FinancialInstrument59 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrument59 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
	pub issr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sctr", skip_serializing_if = "Option::is_none") )]
	pub sctr: Option<String>,
}

impl FinancialInstrument59 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
		if !pattern.is_match(&self.id) {
			return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
		}
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.issr) {
			return Err(ValidationError::new(1005, "issr does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// GeneralCollateral3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GeneralCollateral3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_id: Option<Vec<FinancialInstrument59>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ElgblFinInstrmId", skip_serializing_if = "Option::is_none") )]
	pub elgbl_fin_instrm_id: Option<Vec<String>>,
}

impl GeneralCollateral3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.fin_instrm_id { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref vec) = self.elgbl_fin_instrm_id {
			for item in vec {
				let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
				if !pattern.is_match(&item) {
					return Err(ValidationError::new(1005, "elgbl_fin_instrm_id does not match the required pattern".to_string()));
				}
			}
		}
		Ok(())
	}
}


// Investment1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Investment1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "UscrdCshDpst", skip_serializing_if = "Option::is_none") )]
	pub uscrd_csh_dpst: Option<Deposit1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CntrlBkDpst", skip_serializing_if = "Option::is_none") )]
	pub cntrl_bk_dpst: Option<Deposit1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RpAgrmt", skip_serializing_if = "Option::is_none") )]
	pub rp_agrmt: Option<RepurchaseAgreement2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrInvstmts", skip_serializing_if = "Option::is_none") )]
	pub othr_invstmts: Option<OtherInvestment1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OutrghtInvstmt", skip_serializing_if = "Option::is_none") )]
	pub outrght_invstmt: Option<SecurityIdentificationAndAmount1>,
}

impl Investment1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.uscrd_csh_dpst { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.cntrl_bk_dpst { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.rp_agrmt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.othr_invstmts { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.outrght_invstmt { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// OtherInvestment1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OtherInvestment1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc") )]
	pub desc: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveCurrencyAndAmount,
}

impl OtherInvestment1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.desc.chars().count() < 1 {
			return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
		}
		if self.desc.chars().count() > 140 {
			return Err(ValidationError::new(1002, "desc exceeds the maximum length of 140".to_string()));
		}
		if let Err(e) = self.amt.validate() { return Err(e); }
		Ok(())
	}
}


// ProductType7Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ProductType7Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SVGN") )]
	CodeSVGN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EQUI") )]
	CodeEQUI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
}

impl ProductType7Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RepurchaseAgreement2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RepurchaseAgreement2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MtrtyDt") )]
	pub mtrty_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ScndLegPric") )]
	pub scnd_leg_pric: ActiveCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollMktVal") )]
	pub coll_mkt_val: ActiveCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPty") )]
	pub ctr_pty: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RpAgrmtTp") )]
	pub rp_agrmt_tp: RepurchaseAgreementType3Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrptyAgtId", skip_serializing_if = "Option::is_none") )]
	pub trpty_agt_id: Option<String>,
}

impl RepurchaseAgreement2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.scnd_leg_pric.validate() { return Err(e); }
		if let Err(e) = self.coll_mkt_val.validate() { return Err(e); }
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.ctr_pty) {
			return Err(ValidationError::new(1005, "ctr_pty does not match the required pattern".to_string()));
		}
		if let Err(e) = self.rp_agrmt_tp.validate() { return Err(e); }
		if let Some(ref val) = self.trpty_agt_id {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "trpty_agt_id does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// RepurchaseAgreementType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RepurchaseAgreementType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SpcfcColl", skip_serializing_if = "Option::is_none") )]
	pub spcfc_coll: Option<SpecificCollateral2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GnlColl", skip_serializing_if = "Option::is_none") )]
	pub gnl_coll: Option<GeneralCollateral3>,
}

impl RepurchaseAgreementType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.spcfc_coll { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.gnl_coll { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// SecurityIdentificationAndAmount1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecurityIdentificationAndAmount1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktVal") )]
	pub mkt_val: ActiveCurrencyAnd24Amount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmTp") )]
	pub fin_instrm_tp: ProductType7Code,
}

impl SecurityIdentificationAndAmount1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
		if !pattern.is_match(&self.id) {
			return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
		}
		if let Err(e) = self.mkt_val.validate() { return Err(e); }
		if let Err(e) = self.fin_instrm_tp.validate() { return Err(e); }
		Ok(())
	}
}


// SpecificCollateral2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SpecificCollateral2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmId") )]
	pub fin_instrm_id: FinancialInstrument59,
}

impl SpecificCollateral2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.fin_instrm_id.validate() { return Err(e); }
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
