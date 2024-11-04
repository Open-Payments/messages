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


// ActiveCurrencyAnd13DecimalAmount ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ActiveCurrencyAnd13DecimalAmount {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
	pub value: f64,
}

impl ActiveCurrencyAnd13DecimalAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ActiveOrHistoricCurrencyAnd13DecimalAmount ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmount {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
	pub value: f64,
}

impl ActiveOrHistoricCurrencyAnd13DecimalAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ActiveOrHistoricCurrencyAndAmount ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ActiveOrHistoricCurrencyAndAmount {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
	pub value: f64,
}

impl ActiveOrHistoricCurrencyAndAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AdditionalReference3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AdditionalReference3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ref") )]
	pub ref_attr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefIssr", skip_serializing_if = "Option::is_none") )]
	pub ref_issr: Option<PartyIdentification2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgNm", skip_serializing_if = "Option::is_none") )]
	pub msg_nm: Option<String>,
}

impl AdditionalReference3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.ref_attr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "ref_attr is shorter than the minimum length of 1".to_string()));
		}
		if self.ref_attr.chars().count() > 35 {
			return Err(ValidationError::new(1002, "ref_attr exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.ref_issr { val.validate()? }
		if let Some(ref val) = self.msg_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "msg_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "msg_nm exceeds the maximum length of 35".to_string()));
			}
		}
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


// AlternateSecurityIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AlternateSecurityIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DmstIdSrc", skip_serializing_if = "Option::is_none") )]
	pub dmst_id_src: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryIdSrc", skip_serializing_if = "Option::is_none") )]
	pub prtry_id_src: Option<String>,
}

impl AlternateSecurityIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.dmst_id_src {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "dmst_id_src does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prtry_id_src {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry_id_src is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry_id_src exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// CalculationBasis2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum CalculationBasis2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "AVER") )]
	CodeAVER,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DAIL") )]
	CodeDAIL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
	CodeMNTH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
	CodeYEAR,
}

impl CalculationBasis2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Charge15 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Charge15 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<ChargeType9Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XtndedTp", skip_serializing_if = "Option::is_none") )]
	pub xtnded_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
	pub rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClctnBsis", skip_serializing_if = "Option::is_none") )]
	pub clctn_bsis: Option<CalculationBasis2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XtndedClctnBsis", skip_serializing_if = "Option::is_none") )]
	pub xtnded_clctn_bsis: Option<String>,
}

impl Charge15 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.xtnded_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "xtnded_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "xtnded_tp exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.amt { val.validate()? }
		if let Some(ref val) = self.clctn_bsis { val.validate()? }
		if let Some(ref val) = self.xtnded_clctn_bsis {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "xtnded_clctn_bsis is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "xtnded_clctn_bsis exceeds the maximum length of 350".to_string()));
			}
		}
		Ok(())
	}
}


// ChargeType9Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ChargeType9Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MANF") )]
	CodeMANF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BEND") )]
	CodeBEND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FEND") )]
	CodeFEND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ADVI") )]
	CodeADVI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CUST") )]
	CodeCUST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUBL") )]
	CodePUBL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACCT") )]
	CodeACCT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EQUL") )]
	CodeEQUL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PENA") )]
	CodePENA,
}

impl ChargeType9Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DateAndDateTime1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DateAndDateTime1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtTm", skip_serializing_if = "Option::is_none") )]
	pub dt_tm: Option<String>,
}

impl DateAndDateTime1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DateAndDateTimeChoice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DateAndDateTimeChoice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtTm", skip_serializing_if = "Option::is_none") )]
	pub dt_tm: Option<String>,
}

impl DateAndDateTimeChoice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DateOrDateTimePeriodChoice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DateOrDateTimePeriodChoice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<DatePeriodDetails>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtTm", skip_serializing_if = "Option::is_none") )]
	pub dt_tm: Option<DateTimePeriodDetails>,
}

impl DateOrDateTimePeriodChoice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dt { val.validate()? }
		if let Some(ref val) = self.dt_tm { val.validate()? }
		Ok(())
	}
}


// DatePeriodDetails ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DatePeriodDetails {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt") )]
	pub fr_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt") )]
	pub to_dt: String,
}

impl DatePeriodDetails {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DateTimePeriodDetails ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DateTimePeriodDetails {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDtTm") )]
	pub fr_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDtTm") )]
	pub to_dt_tm: String,
}

impl DateTimePeriodDetails {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DistributionPolicy1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum DistributionPolicy1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DIST") )]
	CodeDIST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACCU") )]
	CodeACCU,
}

impl DistributionPolicy1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// EUCapitalGain2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum EUCapitalGain2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "EUSI") )]
	CodeEUSI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EUSO") )]
	CodeEUSO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UKWN") )]
	CodeUKWN,
}

impl EUCapitalGain2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// EUDividendStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum EUDividendStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DIVI") )]
	CodeDIVI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DIVO") )]
	CodeDIVO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UKWN") )]
	CodeUKWN,
}

impl EUDividendStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// EventFrequency1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum EventFrequency1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
	CodeYEAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SEMI") )]
	CodeSEMI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QUTR") )]
	CodeQUTR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TOMN") )]
	CodeTOMN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
	CodeMNTH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TWMN") )]
	CodeTWMN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TOWK") )]
	CodeTOWK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WEEK") )]
	CodeWEEK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DAIL") )]
	CodeDAIL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ADHO") )]
	CodeADHO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INDA") )]
	CodeINDA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OVNG") )]
	CodeOVNG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ONDE") )]
	CodeONDE,
}

impl EventFrequency1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Extension1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Extension1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PlcAndNm") )]
	pub plc_and_nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Txt") )]
	pub txt: String,
}

impl Extension1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.plc_and_nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "plc_and_nm is shorter than the minimum length of 1".to_string()));
		}
		if self.plc_and_nm.chars().count() > 350 {
			return Err(ValidationError::new(1002, "plc_and_nm exceeds the maximum length of 350".to_string()));
		}
		if self.txt.chars().count() < 1 {
			return Err(ValidationError::new(1001, "txt is shorter than the minimum length of 1".to_string()));
		}
		if self.txt.chars().count() > 350 {
			return Err(ValidationError::new(1002, "txt exceeds the maximum length of 350".to_string()));
		}
		Ok(())
	}
}


// FinancialInstrument8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrument8 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: Vec<SecurityIdentification3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryId", skip_serializing_if = "Option::is_none") )]
	pub splmtry_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DnmtnCcy", skip_serializing_if = "Option::is_none") )]
	pub dnmtn_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClssTp", skip_serializing_if = "Option::is_none") )]
	pub clss_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesForm", skip_serializing_if = "Option::is_none") )]
	pub scties_form: Option<FormOfSecurity1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DstrbtnPlcy", skip_serializing_if = "Option::is_none") )]
	pub dstrbtn_plcy: Option<DistributionPolicy1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DualFndInd") )]
	pub dual_fnd_ind: bool,
}

impl FinancialInstrument8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.id { item.validate()? }
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.splmtry_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "splmtry_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "splmtry_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.dnmtn_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "dnmtn_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.clss_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "clss_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "clss_tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.scties_form { val.validate()? }
		if let Some(ref val) = self.dstrbtn_plcy { val.validate()? }
		Ok(())
	}
}


// FinancialInstrumentQuantity1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrumentQuantity1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Unit") )]
	pub unit: f64,
}

impl FinancialInstrumentQuantity1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FormOfSecurity1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum FormOfSecurity1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "BEAR") )]
	CodeBEAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REGD") )]
	CodeREGD,
}

impl FormOfSecurity1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// GenericIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.schme_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "schme_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "schme_nm exceeds the maximum length of 35".to_string()));
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
		Ok(())
	}
}


// MessageIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MessageIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
	pub cre_dt_tm: String,
}

impl MessageIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// NameAndAddress5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NameAndAddress5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Adr", skip_serializing_if = "Option::is_none") )]
	pub adr: Option<PostalAddress1>,
}

impl NameAndAddress5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 350 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
		}
		if let Some(ref val) = self.adr { val.validate()? }
		Ok(())
	}
}


// Pagination ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Pagination {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PgNb") )]
	pub pg_nb: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LastPgInd") )]
	pub last_pg_ind: bool,
}

impl Pagination {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{1,5}").unwrap();
		if !pattern.is_match(&self.pg_nb) {
			return Err(ValidationError::new(1005, "pg_nb does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// PartyIdentification2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyIdentification2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "BICOrBEI", skip_serializing_if = "Option::is_none") )]
	pub bic_or_bei: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
	pub prtry_id: Option<GenericIdentification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
	pub nm_and_adr: Option<NameAndAddress5>,
}

impl PartyIdentification2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.bic_or_bei {
			let pattern = Regex::new("[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "bic_or_bei does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prtry_id { val.validate()? }
		if let Some(ref val) = self.nm_and_adr { val.validate()? }
		Ok(())
	}
}


// PerformanceFactors1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PerformanceFactors1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CorpActnFctr", skip_serializing_if = "Option::is_none") )]
	pub corp_actn_fctr: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CmltvCorpActnFctr", skip_serializing_if = "Option::is_none") )]
	pub cmltv_corp_actn_fctr: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcmltnPrd", skip_serializing_if = "Option::is_none") )]
	pub acmltn_prd: Option<DatePeriodDetails>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NrmlPrfrmnc", skip_serializing_if = "Option::is_none") )]
	pub nrml_prfrmnc: Option<f64>,
}

impl PerformanceFactors1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.acmltn_prd { val.validate()? }
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
		if let Some(ref val) = self.adr_tp { val.validate()? }
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


// PriceMethod1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum PriceMethod1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FORW") )]
	CodeFORW,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HIST") )]
	CodeHIST,
}

impl PriceMethod1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PriceReport3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PriceReport3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricValtnDtls") )]
	pub pric_valtn_dtls: Vec<PriceValuation4>,
}

impl PriceReport3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.pric_valtn_dtls { item.validate()? }
		Ok(())
	}
}


// PriceReportCancellationV04 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PriceReportCancellationV04 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: MessageIdentification1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PoolRef", skip_serializing_if = "Option::is_none") )]
	pub pool_ref: Option<AdditionalReference3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsRef", skip_serializing_if = "Option::is_none") )]
	pub prvs_ref: Option<AdditionalReference3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgPgntn") )]
	pub msg_pgntn: Pagination,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricRptId") )]
	pub pric_rpt_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CxlId") )]
	pub cxl_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CxlRsn", skip_serializing_if = "Option::is_none") )]
	pub cxl_rsn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XpctdPricCrrctnDt", skip_serializing_if = "Option::is_none") )]
	pub xpctd_pric_crrctn_dt: Option<DateAndDateTime1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CmpltPricCxl") )]
	pub cmplt_pric_cxl: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CancPricValtnDtls", skip_serializing_if = "Option::is_none") )]
	pub canc_pric_valtn_dtls: Option<Vec<PriceReport3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Xtnsn", skip_serializing_if = "Option::is_none") )]
	pub xtnsn: Option<Vec<Extension1>>,
}

impl PriceReportCancellationV04 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.msg_id.validate()?;
		if let Some(ref val) = self.pool_ref { val.validate()? }
		if let Some(ref val) = self.prvs_ref { val.validate()? }
		self.msg_pgntn.validate()?;
		if self.pric_rpt_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "pric_rpt_id is shorter than the minimum length of 1".to_string()));
		}
		if self.pric_rpt_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "pric_rpt_id exceeds the maximum length of 35".to_string()));
		}
		if self.cxl_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "cxl_id is shorter than the minimum length of 1".to_string()));
		}
		if self.cxl_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "cxl_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.cxl_rsn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cxl_rsn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "cxl_rsn exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.xpctd_pric_crrctn_dt { val.validate()? }
		if let Some(ref vec) = self.canc_pric_valtn_dtls { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.xtnsn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PriceType2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PriceType2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Strd") )]
	pub strd: TypeOfPrice6Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<String>,
}

impl PriceType2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.strd.validate()?;
		if let Some(ref val) = self.addtl_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 350".to_string()));
			}
		}
		Ok(())
	}
}


// PriceValuation4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PriceValuation4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValtnDtTm", skip_serializing_if = "Option::is_none") )]
	pub valtn_dt_tm: Option<DateAndDateTimeChoice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NAVDtTm") )]
	pub nav_dt_tm: DateAndDateTimeChoice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmDtls") )]
	pub fin_instrm_dtls: FinancialInstrument8,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FndMgmtCpny", skip_serializing_if = "Option::is_none") )]
	pub fnd_mgmt_cpny: Option<PartyIdentification2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNAV", skip_serializing_if = "Option::is_none") )]
	pub ttl_nav: Option<Vec<ActiveOrHistoricCurrencyAndAmount>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlUnitsNb", skip_serializing_if = "Option::is_none") )]
	pub ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NxtValtnDtTm", skip_serializing_if = "Option::is_none") )]
	pub nxt_valtn_dt_tm: Option<DateAndDateTimeChoice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsValtnDtTm", skip_serializing_if = "Option::is_none") )]
	pub prvs_valtn_dt_tm: Option<DateAndDateTimeChoice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValtnTp") )]
	pub valtn_tp: ValuationTiming1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValtnFrqcy", skip_serializing_if = "Option::is_none") )]
	pub valtn_frqcy: Option<EventFrequency1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OffclValtnInd") )]
	pub offcl_valtn_ind: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SspdInd") )]
	pub sspd_ind: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricDtls", skip_serializing_if = "Option::is_none") )]
	pub pric_dtls: Option<Vec<UnitPrice15>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValtnSttstcs", skip_serializing_if = "Option::is_none") )]
	pub valtn_sttstcs: Option<Vec<ValuationStatistics3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrfrmncDtls", skip_serializing_if = "Option::is_none") )]
	pub prfrmnc_dtls: Option<PerformanceFactors1>,
}

impl PriceValuation4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.valtn_dt_tm { val.validate()? }
		self.nav_dt_tm.validate()?;
		self.fin_instrm_dtls.validate()?;
		if let Some(ref val) = self.fnd_mgmt_cpny { val.validate()? }
		if let Some(ref vec) = self.ttl_nav { for item in vec { item.validate()? } }
		if let Some(ref val) = self.ttl_units_nb { val.validate()? }
		if let Some(ref val) = self.nxt_valtn_dt_tm { val.validate()? }
		if let Some(ref val) = self.prvs_valtn_dt_tm { val.validate()? }
		self.valtn_tp.validate()?;
		if let Some(ref val) = self.valtn_frqcy { val.validate()? }
		if let Some(ref vec) = self.pric_dtls { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.valtn_sttstcs { for item in vec { item.validate()? } }
		if let Some(ref val) = self.prfrmnc_dtls { val.validate()? }
		Ok(())
	}
}


// PriceValue1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PriceValue1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveCurrencyAnd13DecimalAmount,
}

impl PriceValue1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		Ok(())
	}
}


// PriceValue5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PriceValue5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAnd13DecimalAmount,
}

impl PriceValue5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		Ok(())
	}
}


// PriceValueChange1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PriceValueChange1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmtSgn", skip_serializing_if = "Option::is_none") )]
	pub amt_sgn: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
	pub rate: Option<f64>,
}

impl PriceValueChange1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.amt { val.validate()? }
		Ok(())
	}
}


// SecurityIdentification3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecurityIdentification3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
	pub isin: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SEDOL", skip_serializing_if = "Option::is_none") )]
	pub sedol: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CUSIP", skip_serializing_if = "Option::is_none") )]
	pub cusip: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RIC", skip_serializing_if = "Option::is_none") )]
	pub ric: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TckrSymb", skip_serializing_if = "Option::is_none") )]
	pub tckr_symb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Blmbrg", skip_serializing_if = "Option::is_none") )]
	pub blmbrg: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CTA", skip_serializing_if = "Option::is_none") )]
	pub cta: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QUICK", skip_serializing_if = "Option::is_none") )]
	pub quick: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Wrtppr", skip_serializing_if = "Option::is_none") )]
	pub wrtppr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dtch", skip_serializing_if = "Option::is_none") )]
	pub dtch: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Vlrn", skip_serializing_if = "Option::is_none") )]
	pub vlrn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SCVM", skip_serializing_if = "Option::is_none") )]
	pub scvm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Belgn", skip_serializing_if = "Option::is_none") )]
	pub belgn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cmon", skip_serializing_if = "Option::is_none") )]
	pub cmon: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPrtryId", skip_serializing_if = "Option::is_none") )]
	pub othr_prtry_id: Option<AlternateSecurityIdentification1>,
}

impl SecurityIdentification3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.isin {
			let pattern = Regex::new("[A-Z0-9]{12,12}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ric {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ric is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ric exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.tckr_symb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tckr_symb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tckr_symb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.blmbrg {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "blmbrg is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "blmbrg exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.cta {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cta is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "cta exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.cmon {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cmon is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 12 {
				return Err(ValidationError::new(1002, "cmon exceeds the maximum length of 12".to_string()));
			}
		}
		if let Some(ref val) = self.othr_prtry_id { val.validate()? }
		Ok(())
	}
}


// StatisticsByPredefinedTimePeriods2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct StatisticsByPredefinedTimePeriods2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "HghstPricVal12Mnths", skip_serializing_if = "Option::is_none") )]
	pub hghst_pric_val12_mnths: Option<PriceValue5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LwstPricVal12Mnths", skip_serializing_if = "Option::is_none") )]
	pub lwst_pric_val12_mnths: Option<PriceValue5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OneYrPricChng", skip_serializing_if = "Option::is_none") )]
	pub one_yr_pric_chng: Option<PriceValueChange1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ThreeYrPricChng", skip_serializing_if = "Option::is_none") )]
	pub three_yr_pric_chng: Option<PriceValueChange1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FiveYrPricChng", skip_serializing_if = "Option::is_none") )]
	pub five_yr_pric_chng: Option<PriceValueChange1>,
}

impl StatisticsByPredefinedTimePeriods2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.hghst_pric_val12_mnths { val.validate()? }
		if let Some(ref val) = self.lwst_pric_val12_mnths { val.validate()? }
		if let Some(ref val) = self.one_yr_pric_chng { val.validate()? }
		if let Some(ref val) = self.three_yr_pric_chng { val.validate()? }
		if let Some(ref val) = self.five_yr_pric_chng { val.validate()? }
		Ok(())
	}
}


// StatisticsByUserDefinedTimePeriod2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct StatisticsByUserDefinedTimePeriod2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prd") )]
	pub prd: DateOrDateTimePeriodChoice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HghstPricVal", skip_serializing_if = "Option::is_none") )]
	pub hghst_pric_val: Option<PriceValue5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LwstPricVal", skip_serializing_if = "Option::is_none") )]
	pub lwst_pric_val: Option<PriceValue5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricChng", skip_serializing_if = "Option::is_none") )]
	pub pric_chng: Option<PriceValueChange1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Yld", skip_serializing_if = "Option::is_none") )]
	pub yld: Option<f64>,
}

impl StatisticsByUserDefinedTimePeriod2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.prd.validate()?;
		if let Some(ref val) = self.hghst_pric_val { val.validate()? }
		if let Some(ref val) = self.lwst_pric_val { val.validate()? }
		if let Some(ref val) = self.pric_chng { val.validate()? }
		Ok(())
	}
}


// Tax17 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Tax17 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<TaxType12Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XtndedTp", skip_serializing_if = "Option::is_none") )]
	pub xtnded_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<Vec<ActiveOrHistoricCurrencyAnd13DecimalAmount>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
	pub rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
	pub ctry: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxClctnDtls", skip_serializing_if = "Option::is_none") )]
	pub tax_clctn_dtls: Option<TaxCalculationInformation4>,
}

impl Tax17 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.xtnded_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "xtnded_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "xtnded_tp exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref vec) = self.amt { for item in vec { item.validate()? } }
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.ctry) {
			return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.tax_clctn_dtls { val.validate()? }
		Ok(())
	}
}


// TaxCalculationInformation4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TaxCalculationInformation4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "EUCptlGn", skip_serializing_if = "Option::is_none") )]
	pub eu_cptl_gn: Option<EUCapitalGain2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XtndedEUCptlGn", skip_serializing_if = "Option::is_none") )]
	pub xtnded_eu_cptl_gn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PctgOfDebtClm", skip_serializing_if = "Option::is_none") )]
	pub pctg_of_debt_clm: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PctgGrdfthdDebt", skip_serializing_if = "Option::is_none") )]
	pub pctg_grdfthd_debt: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxblIncmPerDvdd", skip_serializing_if = "Option::is_none") )]
	pub taxbl_incm_per_dvdd: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EUDvddSts", skip_serializing_if = "Option::is_none") )]
	pub eu_dvdd_sts: Option<EUDividendStatus1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XtndedEUDvddSts", skip_serializing_if = "Option::is_none") )]
	pub xtnded_eu_dvdd_sts: Option<String>,
}

impl TaxCalculationInformation4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.eu_cptl_gn { val.validate()? }
		if let Some(ref val) = self.xtnded_eu_cptl_gn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "xtnded_eu_cptl_gn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "xtnded_eu_cptl_gn exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.taxbl_incm_per_dvdd { val.validate()? }
		if let Some(ref val) = self.eu_dvdd_sts { val.validate()? }
		if let Some(ref val) = self.xtnded_eu_dvdd_sts {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "xtnded_eu_dvdd_sts is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "xtnded_eu_dvdd_sts exceeds the maximum length of 350".to_string()));
			}
		}
		Ok(())
	}
}


// TaxType12Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TaxType12Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "INPO") )]
	CodeINPO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EUTR") )]
	CodeEUTR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AKT1") )]
	CodeAKT1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AKT2") )]
	CodeAKT2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ZWIS") )]
	CodeZWIS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MIET") )]
	CodeMIET,
}

impl TaxType12Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TaxableIncomePerShareCalculated2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TaxableIncomePerShareCalculated2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "TSIY") )]
	CodeTSIY,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TSIN") )]
	CodeTSIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UKWN") )]
	CodeUKWN,
}

impl TaxableIncomePerShareCalculated2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TypeOfPrice6Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TypeOfPrice6Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "BIDE") )]
	CodeBIDE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OFFR") )]
	CodeOFFR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NAVL") )]
	CodeNAVL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CREA") )]
	CodeCREA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CANC") )]
	CodeCANC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INTE") )]
	CodeINTE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SWNG") )]
	CodeSWNG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MIDD") )]
	CodeMIDD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RINV") )]
	CodeRINV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SWIC") )]
	CodeSWIC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DDVR") )]
	CodeDDVR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACTU") )]
	CodeACTU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NAUP") )]
	CodeNAUP,
}

impl TypeOfPrice6Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TypeOfPrice9Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TypeOfPrice9Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "BIDE") )]
	CodeBIDE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OFFR") )]
	CodeOFFR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NAVL") )]
	CodeNAVL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CREA") )]
	CodeCREA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CANC") )]
	CodeCANC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INTE") )]
	CodeINTE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SWNG") )]
	CodeSWNG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MIDD") )]
	CodeMIDD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RINV") )]
	CodeRINV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SWIC") )]
	CodeSWIC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DDVR") )]
	CodeDDVR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACTU") )]
	CodeACTU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NAUP") )]
	CodeNAUP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GUAR") )]
	CodeGUAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ENAV") )]
	CodeENAV,
}

impl TypeOfPrice9Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// UnitPrice15 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UnitPrice15 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<TypeOfPrice9Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XtndedTp", skip_serializing_if = "Option::is_none") )]
	pub xtnded_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricMtd", skip_serializing_if = "Option::is_none") )]
	pub pric_mtd: Option<PriceMethod1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValInInvstmtCcy") )]
	pub val_in_invstmt_ccy: Vec<PriceValue1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValInAltrntvCcy", skip_serializing_if = "Option::is_none") )]
	pub val_in_altrntv_ccy: Option<Vec<PriceValue1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ForExctnInd") )]
	pub for_exctn_ind: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CumDvddInd") )]
	pub cum_dvdd_ind: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClctnBsis", skip_serializing_if = "Option::is_none") )]
	pub clctn_bsis: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EstmtdPricInd") )]
	pub estmtd_pric_ind: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfDaysAcrd", skip_serializing_if = "Option::is_none") )]
	pub nb_of_days_acrd: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxblIncmPerShr", skip_serializing_if = "Option::is_none") )]
	pub taxbl_incm_per_shr: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxblIncmPerShrClctd", skip_serializing_if = "Option::is_none") )]
	pub taxbl_incm_per_shr_clctd: Option<TaxableIncomePerShareCalculated2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XtndedTaxblIncmPerShrClctd", skip_serializing_if = "Option::is_none") )]
	pub xtnded_taxbl_incm_per_shr_clctd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxblIncmPerDvdd", skip_serializing_if = "Option::is_none") )]
	pub taxbl_incm_per_dvdd: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EUDvddSts", skip_serializing_if = "Option::is_none") )]
	pub eu_dvdd_sts: Option<EUDividendStatus1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XtndedEUDvddSts", skip_serializing_if = "Option::is_none") )]
	pub xtnded_eu_dvdd_sts: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgDtls", skip_serializing_if = "Option::is_none") )]
	pub chrg_dtls: Option<Vec<Charge15>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxLbltyDtls", skip_serializing_if = "Option::is_none") )]
	pub tax_lblty_dtls: Option<Vec<Tax17>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRfndDtls", skip_serializing_if = "Option::is_none") )]
	pub tax_rfnd_dtls: Option<Vec<Tax17>>,
}

impl UnitPrice15 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.xtnded_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "xtnded_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "xtnded_tp exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.pric_mtd { val.validate()? }
		for item in &self.val_in_invstmt_ccy { item.validate()? }
		if let Some(ref vec) = self.val_in_altrntv_ccy { for item in vec { item.validate()? } }
		if let Some(ref val) = self.taxbl_incm_per_shr { val.validate()? }
		if let Some(ref val) = self.taxbl_incm_per_shr_clctd { val.validate()? }
		if let Some(ref val) = self.xtnded_taxbl_incm_per_shr_clctd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "xtnded_taxbl_incm_per_shr_clctd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "xtnded_taxbl_incm_per_shr_clctd exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.taxbl_incm_per_dvdd { val.validate()? }
		if let Some(ref val) = self.eu_dvdd_sts { val.validate()? }
		if let Some(ref val) = self.xtnded_eu_dvdd_sts {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "xtnded_eu_dvdd_sts is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "xtnded_eu_dvdd_sts exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref vec) = self.chrg_dtls { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.tax_lblty_dtls { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.tax_rfnd_dtls { for item in vec { item.validate()? } }
		Ok(())
	}
}


// ValuationStatistics3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ValuationStatistics3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricTpChngBsis") )]
	pub pric_tp_chng_bsis: PriceType2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricChng") )]
	pub pric_chng: PriceValueChange1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Yld", skip_serializing_if = "Option::is_none") )]
	pub yld: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ByPrdfndTmPrds", skip_serializing_if = "Option::is_none") )]
	pub by_prdfnd_tm_prds: Option<StatisticsByPredefinedTimePeriods2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ByUsrDfndTmPrd", skip_serializing_if = "Option::is_none") )]
	pub by_usr_dfnd_tm_prd: Option<Vec<StatisticsByUserDefinedTimePeriod2>>,
}

impl ValuationStatistics3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.ccy) {
			return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
		}
		self.pric_tp_chng_bsis.validate()?;
		self.pric_chng.validate()?;
		if let Some(ref val) = self.by_prdfnd_tm_prds { val.validate()? }
		if let Some(ref vec) = self.by_usr_dfnd_tm_prd { for item in vec { item.validate()? } }
		Ok(())
	}
}


// ValuationTiming1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ValuationTiming1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "EXCP") )]
	CodeEXCP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USUA") )]
	CodeUSUA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PATC") )]
	CodePATC,
}

impl ValuationTiming1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}
