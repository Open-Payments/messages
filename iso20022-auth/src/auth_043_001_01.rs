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


// BenchmarkCurveName2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum BenchmarkCurveName2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "WIBO") )]
	CodeWIBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TREA") )]
	CodeTREA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TIBO") )]
	CodeTIBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TLBO") )]
	CodeTLBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SWAP") )]
	CodeSWAP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "STBO") )]
	CodeSTBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRBO") )]
	CodePRBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PFAN") )]
	CodePFAN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NIBO") )]
	CodeNIBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MAAA") )]
	CodeMAAA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MOSP") )]
	CodeMOSP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LIBO") )]
	CodeLIBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LIBI") )]
	CodeLIBI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "JIBA") )]
	CodeJIBA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISDA") )]
	CodeISDA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GCFR") )]
	CodeGCFR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FUSW") )]
	CodeFUSW,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EUCH") )]
	CodeEUCH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EUUS") )]
	CodeEUUS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EURI") )]
	CodeEURI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EONS") )]
	CodeEONS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EONA") )]
	CodeEONA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CIBO") )]
	CodeCIBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CDOR") )]
	CodeCDOR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BUBO") )]
	CodeBUBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BBSW") )]
	CodeBBSW,
}

impl BenchmarkCurveName2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FinancialInstrument46Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrument46Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
	pub isin: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Indx", skip_serializing_if = "Option::is_none") )]
	pub indx: Option<BenchmarkCurveName2Code>,
}

impl FinancialInstrument46Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.isin {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.indx { val.validate()? }
		Ok(())
	}
}


// FinancialInstrumentReportingReferenceDataIndexReportV01 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrumentReportingReferenceDataIndexReportV01 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptHdr") )]
	pub rpt_hdr: SecuritiesMarketReportHeader1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IndxData") )]
	pub indx_data: Vec<SecuritiesIndexReport1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl FinancialInstrumentReportingReferenceDataIndexReportV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rpt_hdr.validate()?;
		for item in &self.indx_data { item.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// Period2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Period2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt") )]
	pub fr_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt") )]
	pub to_dt: String,
}

impl Period2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Period4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Period4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt", skip_serializing_if = "Option::is_none") )]
	pub fr_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt", skip_serializing_if = "Option::is_none") )]
	pub to_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDtToDt", skip_serializing_if = "Option::is_none") )]
	pub fr_dt_to_dt: Option<Period2>,
}

impl Period4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.fr_dt_to_dt { val.validate()? }
		Ok(())
	}
}


// SecuritiesIndexReport1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesIndexReport1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none") )]
	pub tech_rcrd_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RqstngNtty", skip_serializing_if = "Option::is_none") )]
	pub rqstng_ntty: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Indx") )]
	pub indx: FinancialInstrument46Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldtyPrd", skip_serializing_if = "Option::is_none") )]
	pub vldty_prd: Option<Period4Choice>,
}

impl SecuritiesIndexReport1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tech_rcrd_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tech_rcrd_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tech_rcrd_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.rqstng_ntty {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "rqstng_ntty does not match the required pattern".to_string()));
			}
		}
		self.indx.validate()?;
		if let Some(ref val) = self.vldty_prd { val.validate()? }
		Ok(())
	}
}


// SecuritiesMarketReportHeader1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesMarketReportHeader1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgNtty") )]
	pub rptg_ntty: TradingVenueIdentification1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgPrd") )]
	pub rptg_prd: Period4Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubmissnDtTm", skip_serializing_if = "Option::is_none") )]
	pub submissn_dt_tm: Option<String>,
}

impl SecuritiesMarketReportHeader1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rptg_ntty.validate()?;
		self.rptg_prd.validate()?;
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


// TradingVenue2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TradingVenue2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "APPA") )]
	CodeAPPA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CTPS") )]
	CodeCTPS,
}

impl TradingVenue2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TradingVenueIdentification1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradingVenueIdentification1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktIdCd", skip_serializing_if = "Option::is_none") )]
	pub mkt_id_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NtlCmptntAuthrty", skip_serializing_if = "Option::is_none") )]
	pub ntl_cmptnt_authrty: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<TradingVenueIdentification2>,
}

impl TradingVenueIdentification1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mkt_id_cd {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "mkt_id_cd does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ntl_cmptnt_authrty {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ntl_cmptnt_authrty does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.othr { val.validate()? }
		Ok(())
	}
}


// TradingVenueIdentification2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradingVenueIdentification2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: TradingVenue2Code,
}

impl TradingVenueIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 50 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 50".to_string()));
		}
		self.tp.validate()?;
		Ok(())
	}
}
