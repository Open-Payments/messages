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


// AccountIdentificationAndName7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AccountIdentificationAndName7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: CashAccountIdentification8Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
}

impl AccountIdentificationAndName7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// AccountSchemeName1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AccountSchemeName1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl AccountSchemeName1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
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


// AdditionalInformation15 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AdditionalInformation15 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InfTp") )]
	pub inf_tp: GenericIdentification36,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InfVal") )]
	pub inf_val: String,
}

impl AdditionalInformation15 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.inf_tp.validate()?;
		if self.inf_val.chars().count() < 1 {
			return Err(ValidationError::new(1001, "inf_val is shorter than the minimum length of 1".to_string()));
		}
		if self.inf_val.chars().count() > 350 {
			return Err(ValidationError::new(1002, "inf_val exceeds the maximum length of 350".to_string()));
		}
		Ok(())
	}
}


// AdditionalProductInformation3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AdditionalProductInformation3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmTxCostsExAnteUK", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_tx_costs_ex_ante_uk: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmTxCostsExPstUK", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_tx_costs_ex_pst_uk: Option<f64>,
}

impl AdditionalProductInformation3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AdditionalReference10 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AdditionalReference10 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ref") )]
	pub ref_attr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefIssr", skip_serializing_if = "Option::is_none") )]
	pub ref_issr: Option<PartyIdentification139>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgNm", skip_serializing_if = "Option::is_none") )]
	pub msg_nm: Option<String>,
}

impl AdditionalReference10 {
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


// AnnualChargePaymentType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum AnnualChargePaymentType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CAPL") )]
	CodeCAPL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INCO") )]
	CodeINCO,
}

impl AnnualChargePaymentType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AssessmentOfValueRequiredUnderCOLLUKType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum AssessmentOfValueRequiredUnderCOLLUKType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "YSCO") )]
	CodeYSCO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NSCO") )]
	CodeNSCO,
}

impl AssessmentOfValueRequiredUnderCOLLUKType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// BusinessDayConvention1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum BusinessDayConvention1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FWNG") )]
	CodeFWNG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PREC") )]
	CodePREC,
}

impl BusinessDayConvention1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CashAccount205 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CashAccount205 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy", skip_serializing_if = "Option::is_none") )]
	pub ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmryAcct", skip_serializing_if = "Option::is_none") )]
	pub pmry_acct: Option<CashAccount206>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ScndryAcct", skip_serializing_if = "Option::is_none") )]
	pub scndry_acct: Option<CashAccount206>,
}

impl CashAccount205 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pmry_acct { val.validate()? }
		if let Some(ref val) = self.scndry_acct { val.validate()? }
		Ok(())
	}
}


// CashAccount206 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CashAccount206 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctId") )]
	pub acct_id: AccountIdentificationAndName7,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Svcr", skip_serializing_if = "Option::is_none") )]
	pub svcr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctTpDesc", skip_serializing_if = "Option::is_none") )]
	pub acct_tp_desc: Option<String>,
}

impl CashAccount206 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.acct_id.validate()?;
		if let Some(ref val) = self.svcr {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "svcr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.acct_tp_desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_tp_desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acct_tp_desc exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// CashAccountIdentification8Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CashAccountIdentification8Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<GenericAccountIdentification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IBAN", skip_serializing_if = "Option::is_none") )]
	pub iban: Option<String>,
}

impl CashAccountIdentification8Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.othr { val.validate()? }
		if let Some(ref val) = self.iban {
			let pattern = Regex::new("[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "iban does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// ChargeType8Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ChargeType8Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<InvestmentFundMiFIDFee2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl ChargeType8Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// ContactAttributes5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ContactAttributes5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
	pub pstl_adr: Option<PostalAddress1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PhneNb", skip_serializing_if = "Option::is_none") )]
	pub phne_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FaxNb", skip_serializing_if = "Option::is_none") )]
	pub fax_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none") )]
	pub email_adr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "URLAdr", skip_serializing_if = "Option::is_none") )]
	pub url_adr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
	pub any_bic: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
}

impl ContactAttributes5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 350 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
		}
		if let Some(ref val) = self.pstl_adr { val.validate()? }
		if let Some(ref val) = self.phne_nb {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "phne_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.fax_nb {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "fax_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.email_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "email_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "email_adr exceeds the maximum length of 256".to_string()));
			}
		}
		if let Some(ref val) = self.url_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "url_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 2048 {
				return Err(ValidationError::new(1002, "url_adr exceeds the maximum length of 2048".to_string()));
			}
		}
		if let Some(ref val) = self.any_bic {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// ContactAttributes6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ContactAttributes6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
	pub pstl_adr: Option<PostalAddress1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PhneNb", skip_serializing_if = "Option::is_none") )]
	pub phne_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FaxNb", skip_serializing_if = "Option::is_none") )]
	pub fax_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none") )]
	pub email_adr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "URLAdr", skip_serializing_if = "Option::is_none") )]
	pub url_adr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
	pub any_bic: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
}

impl ContactAttributes6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.pstl_adr { val.validate()? }
		if let Some(ref val) = self.phne_nb {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "phne_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.fax_nb {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "fax_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.email_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "email_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "email_adr exceeds the maximum length of 256".to_string()));
			}
		}
		if let Some(ref val) = self.url_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "url_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 2048 {
				return Err(ValidationError::new(1002, "url_adr exceeds the maximum length of 2048".to_string()));
			}
		}
		if let Some(ref val) = self.any_bic {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// CostsAndCharges2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CostsAndCharges2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExAnteRefDt", skip_serializing_if = "Option::is_none") )]
	pub ex_ante_ref_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IndvCostOrChrg") )]
	pub indv_cost_or_chrg: Vec<IndividualCostOrCharge2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<AdditionalInformation15>,
}

impl CostsAndCharges2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.indv_cost_or_chrg { item.validate()? }
		if let Some(ref val) = self.addtl_inf { val.validate()? }
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


// DistributionStrategy1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DistributionStrategy1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExctnOnly", skip_serializing_if = "Option::is_none") )]
	pub exctn_only: Option<DistributionStrategy1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExctnWthApprprtnssTstOrNonAdvsdSvcs", skip_serializing_if = "Option::is_none") )]
	pub exctn_wth_apprprtnss_tst_or_non_advsd_svcs: Option<DistributionStrategy1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstmtAdvc", skip_serializing_if = "Option::is_none") )]
	pub invstmt_advc: Option<DistributionStrategy1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtflMgmt", skip_serializing_if = "Option::is_none") )]
	pub prtfl_mgmt: Option<DistributionStrategy1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<OtherDistributionStrategy1>,
}

impl DistributionStrategy1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.exctn_only { val.validate()? }
		if let Some(ref val) = self.exctn_wth_apprprtnss_tst_or_non_advsd_svcs { val.validate()? }
		if let Some(ref val) = self.invstmt_advc { val.validate()? }
		if let Some(ref val) = self.prtfl_mgmt { val.validate()? }
		if let Some(ref val) = self.othr { val.validate()? }
		Ok(())
	}
}


// DistributionStrategy1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DistributionStrategy1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<InvestorType3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl DistributionStrategy1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// DividendPolicy1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum DividendPolicy1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CASH") )]
	CodeCASH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UNIT") )]
	CodeUNIT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BOTH") )]
	CodeBOTH,
}

impl DividendPolicy1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// EMTDataReportingVFMUKType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum EMTDataReportingVFMUKType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "YSCO") )]
	CodeYSCO,
}

impl EMTDataReportingVFMUKType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// EUSavingsDirective1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum EUSavingsDirective1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "EUSI") )]
	CodeEUSI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EUSO") )]
	CodeEUSO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VARI") )]
	CodeVARI,
}

impl EUSavingsDirective1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// EventFrequency5Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum EventFrequency5Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
	CodeYEAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SEMI") )]
	CodeSEMI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QUTR") )]
	CodeQUTR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
	CodeMNTH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WEEK") )]
	CodeWEEK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DAIL") )]
	CodeDAIL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLOS") )]
	CodeCLOS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TOMN") )]
	CodeTOMN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TOWK") )]
	CodeTOWK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TWMN") )]
	CodeTWMN,
}

impl EventFrequency5Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// EventFrequency8Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum EventFrequency8Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ADHO") )]
	CodeADHO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
	CodeYEAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DAIL") )]
	CodeDAIL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FOMN") )]
	CodeFOMN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TOMN") )]
	CodeTOMN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TOWK") )]
	CodeTOWK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TYEA") )]
	CodeTYEA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INDA") )]
	CodeINDA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
	CodeMNTH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ONDE") )]
	CodeONDE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OVNG") )]
	CodeOVNG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QUTR") )]
	CodeQUTR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SEMI") )]
	CodeSEMI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TWMN") )]
	CodeTWMN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WEEK") )]
	CodeWEEK,
}

impl EventFrequency8Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ExPostCostCalculationBasis1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ExPostCostCalculationBasis1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<ExPostCostCalculationBasis1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl ExPostCostCalculationBasis1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// ExPostCostCalculationBasis1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ExPostCostCalculationBasis1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FIXB") )]
	CodeFIXB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ROLL") )]
	CodeROLL,
}

impl ExPostCostCalculationBasis1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ExtendedParty13 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ExtendedParty13 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyRole") )]
	pub pty_role: GenericIdentification36,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPtyDtls") )]
	pub othr_pty_dtls: ContactAttributes5,
}

impl ExtendedParty13 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pty_role.validate()?;
		self.othr_pty_dtls.validate()?;
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


// FinancialInstrument96 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrument96 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PhysBrScties", skip_serializing_if = "Option::is_none") )]
	pub phys_br_scties: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DmtrlsdBrScties", skip_serializing_if = "Option::is_none") )]
	pub dmtrlsd_br_scties: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PhysRegdScties", skip_serializing_if = "Option::is_none") )]
	pub phys_regd_scties: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DmtrlsdRegdScties", skip_serializing_if = "Option::is_none") )]
	pub dmtrlsd_regd_scties: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DstrbtnPlcy", skip_serializing_if = "Option::is_none") )]
	pub dstrbtn_plcy: Option<DistributionPolicy1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DvddPlcy", skip_serializing_if = "Option::is_none") )]
	pub dvdd_plcy: Option<DividendPolicy1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DvddFrqcy", skip_serializing_if = "Option::is_none") )]
	pub dvdd_frqcy: Option<EventFrequency5Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RinvstmtFrqcy", skip_serializing_if = "Option::is_none") )]
	pub rinvstmt_frqcy: Option<EventFrequency5Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrntEndLd", skip_serializing_if = "Option::is_none") )]
	pub frnt_end_ld: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BckEndLd", skip_serializing_if = "Option::is_none") )]
	pub bck_end_ld: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SwtchFee", skip_serializing_if = "Option::is_none") )]
	pub swtch_fee: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EUSvgsDrctv", skip_serializing_if = "Option::is_none") )]
	pub eu_svgs_drctv: Option<EUSavingsDirective1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LnchDt", skip_serializing_if = "Option::is_none") )]
	pub lnch_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FndEndDt", skip_serializing_if = "Option::is_none") )]
	pub fnd_end_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none") )]
	pub termntn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitlOfferEndDt", skip_serializing_if = "Option::is_none") )]
	pub initl_offer_end_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SspnsnStartDt", skip_serializing_if = "Option::is_none") )]
	pub sspnsn_start_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SspnsnEndDt", skip_serializing_if = "Option::is_none") )]
	pub sspnsn_end_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none") )]
	pub mtrty_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MayBeTermntdEarly", skip_serializing_if = "Option::is_none") )]
	pub may_be_termntd_early: Option<TargetMarket1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClsdEndFnd", skip_serializing_if = "Option::is_none") )]
	pub clsd_end_fnd: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Equlstn", skip_serializing_if = "Option::is_none") )]
	pub equlstn: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxEffcntPdctElgbl", skip_serializing_if = "Option::is_none") )]
	pub tax_effcnt_pdct_elgbl: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Authrsd", skip_serializing_if = "Option::is_none") )]
	pub authrsd: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RDRCmplnt", skip_serializing_if = "Option::is_none") )]
	pub rdr_cmplnt: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MgmtFeeSrc", skip_serializing_if = "Option::is_none") )]
	pub mgmt_fee_src: Option<AnnualChargePaymentType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrfrmncFee", skip_serializing_if = "Option::is_none") )]
	pub prfrmnc_fee: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}

impl FinancialInstrument96 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dstrbtn_plcy { val.validate()? }
		if let Some(ref val) = self.dvdd_plcy { val.validate()? }
		if let Some(ref val) = self.dvdd_frqcy { val.validate()? }
		if let Some(ref val) = self.rinvstmt_frqcy { val.validate()? }
		if let Some(ref val) = self.eu_svgs_drctv { val.validate()? }
		if let Some(ref val) = self.may_be_termntd_early { val.validate()? }
		if let Some(ref val) = self.mgmt_fee_src { val.validate()? }
		if let Some(ref vec) = self.addtl_inf { for item in vec { item.validate()? } }
		Ok(())
	}
}


// Forms1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Forms1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ApplForm") )]
	pub appl_form: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SgntrTp") )]
	pub sgntr_tp: SignatureType1Code,
}

impl Forms1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.sgntr_tp.validate()?;
		Ok(())
	}
}


// Frequency20Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Frequency20Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<EventFrequency8Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl Frequency20Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// FundOrderType10Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum FundOrderType10Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SUBS") )]
	CodeSUBS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RDIV") )]
	CodeRDIV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REDM") )]
	CodeREDM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RGSV") )]
	CodeRGSV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WIDP") )]
	CodeWIDP,
}

impl FundOrderType10Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FundOrderType5Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FundOrderType5Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<FundOrderType10Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification36>,
}

impl FundOrderType5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// FundParties1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FundParties1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Guarntr", skip_serializing_if = "Option::is_none") )]
	pub guarntr: Option<ContactAttributes5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Audtr", skip_serializing_if = "Option::is_none") )]
	pub audtr: Option<ContactAttributes5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Trstee", skip_serializing_if = "Option::is_none") )]
	pub trstee: Option<ContactAttributes5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPty", skip_serializing_if = "Option::is_none") )]
	pub othr_pty: Option<Vec<ExtendedParty13>>,
}

impl FundParties1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.guarntr { val.validate()? }
		if let Some(ref val) = self.audtr { val.validate()? }
		if let Some(ref val) = self.trstee { val.validate()? }
		if let Some(ref vec) = self.othr_pty { for item in vec { item.validate()? } }
		Ok(())
	}
}


// FundPaymentType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FundPaymentType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<FundPaymentType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification36>,
}

impl FundPaymentType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// FundPaymentType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum FundPaymentType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DRAF") )]
	CodeDRAF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CACC") )]
	CodeCACC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CHEQ") )]
	CodeCHEQ,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRDT") )]
	CodeCRDT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DDEB") )]
	CodeDDEB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CARD") )]
	CodeCARD,
}

impl FundPaymentType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FundReferenceDataReport5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FundReferenceDataReport5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Vrsn", skip_serializing_if = "Option::is_none") )]
	pub vrsn: Option<MarketPracticeVersion1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AuthrsdPrxy", skip_serializing_if = "Option::is_none") )]
	pub authrsd_prxy: Option<ContactAttributes6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GnlRefDt") )]
	pub gnl_ref_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrgtMktInd", skip_serializing_if = "Option::is_none") )]
	pub trgt_mkt_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExAnteInd", skip_serializing_if = "Option::is_none") )]
	pub ex_ante_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExPstInd", skip_serializing_if = "Option::is_none") )]
	pub ex_pst_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctyId") )]
	pub scty_id: SecurityIdentification47,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FndPties", skip_serializing_if = "Option::is_none") )]
	pub fnd_pties: Option<FundParties1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MainFndOrdrDsk", skip_serializing_if = "Option::is_none") )]
	pub main_fnd_ordr_dsk: Option<OrderDesk1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FndMgmtCpny", skip_serializing_if = "Option::is_none") )]
	pub fnd_mgmt_cpny: Option<ContactAttributes5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FndDtls", skip_serializing_if = "Option::is_none") )]
	pub fnd_dtls: Option<FinancialInstrument96>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValtnDealgChrtcs", skip_serializing_if = "Option::is_none") )]
	pub valtn_dealg_chrtcs: Option<ValuationDealingProcessingCharacteristics3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstmtRstrctns", skip_serializing_if = "Option::is_none") )]
	pub invstmt_rstrctns: Option<InvestmentRestrictions3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SbcptPrcgChrtcs", skip_serializing_if = "Option::is_none") )]
	pub sbcpt_prcg_chrtcs: Option<ProcessingCharacteristics11>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RedPrcgChrtcs", skip_serializing_if = "Option::is_none") )]
	pub red_prcg_chrtcs: Option<ProcessingCharacteristics12>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SwtchPrcgChrtcs", skip_serializing_if = "Option::is_none") )]
	pub swtch_prcg_chrtcs: Option<ProcessingCharacteristics9>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PlanChrtcs", skip_serializing_if = "Option::is_none") )]
	pub plan_chrtcs: Option<Vec<InvestmentPlanCharacteristics1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtInstrm", skip_serializing_if = "Option::is_none") )]
	pub pmt_instrm: Option<Vec<PaymentInstrument16>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshSttlmDtls", skip_serializing_if = "Option::is_none") )]
	pub csh_sttlm_dtls: Option<Vec<CashAccount205>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LclMktAnx", skip_serializing_if = "Option::is_none") )]
	pub lcl_mkt_anx: Option<Vec<LocalMarketAnnex6>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrgtMkt", skip_serializing_if = "Option::is_none") )]
	pub trgt_mkt: Option<TargetMarket4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DstrbtnStrtgy", skip_serializing_if = "Option::is_none") )]
	pub dstrbtn_strtgy: Option<DistributionStrategy1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CostsAndChrgs", skip_serializing_if = "Option::is_none") )]
	pub costs_and_chrgs: Option<Vec<CostsAndCharges2>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInfUKMkt", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf_uk_mkt: Option<AdditionalProductInformation3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValForMny", skip_serializing_if = "Option::is_none") )]
	pub val_for_mny: Option<ValueForMoney1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Xtnsn", skip_serializing_if = "Option::is_none") )]
	pub xtnsn: Option<Vec<Extension1>>,
}

impl FundReferenceDataReport5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.vrsn { val.validate()? }
		if let Some(ref val) = self.authrsd_prxy { val.validate()? }
		self.scty_id.validate()?;
		if let Some(ref val) = self.fnd_pties { val.validate()? }
		if let Some(ref val) = self.main_fnd_ordr_dsk { val.validate()? }
		if let Some(ref val) = self.fnd_mgmt_cpny { val.validate()? }
		if let Some(ref val) = self.fnd_dtls { val.validate()? }
		if let Some(ref val) = self.valtn_dealg_chrtcs { val.validate()? }
		if let Some(ref val) = self.invstmt_rstrctns { val.validate()? }
		if let Some(ref val) = self.sbcpt_prcg_chrtcs { val.validate()? }
		if let Some(ref val) = self.red_prcg_chrtcs { val.validate()? }
		if let Some(ref val) = self.swtch_prcg_chrtcs { val.validate()? }
		if let Some(ref vec) = self.plan_chrtcs { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.pmt_instrm { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.csh_sttlm_dtls { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.lcl_mkt_anx { for item in vec { item.validate()? } }
		if let Some(ref val) = self.trgt_mkt { val.validate()? }
		if let Some(ref val) = self.dstrbtn_strtgy { val.validate()? }
		if let Some(ref vec) = self.costs_and_chrgs { for item in vec { item.validate()? } }
		if let Some(ref val) = self.addtl_inf_uk_mkt { val.validate()? }
		if let Some(ref val) = self.val_for_mny { val.validate()? }
		if let Some(ref vec) = self.xtnsn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// FundReferenceDataReportV07 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FundReferenceDataReportV07 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: MessageIdentification1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsRef", skip_serializing_if = "Option::is_none") )]
	pub prvs_ref: Option<Vec<AdditionalReference10>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdRef", skip_serializing_if = "Option::is_none") )]
	pub rltd_ref: Option<AdditionalReference10>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FndRefDataRptId", skip_serializing_if = "Option::is_none") )]
	pub fnd_ref_data_rpt_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rpt") )]
	pub rpt: Vec<FundReferenceDataReport5>,
}

impl FundReferenceDataReportV07 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.msg_id.validate()?;
		if let Some(ref vec) = self.prvs_ref { for item in vec { item.validate()? } }
		if let Some(ref val) = self.rltd_ref { val.validate()? }
		if let Some(ref val) = self.fnd_ref_data_rpt_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "fnd_ref_data_rpt_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "fnd_ref_data_rpt_id exceeds the maximum length of 35".to_string()));
			}
		}
		for item in &self.rpt { item.validate()? }
		Ok(())
	}
}


// GenericAccountIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericAccountIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<AccountSchemeName1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericAccountIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 34 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 34".to_string()));
		}
		if let Some(ref val) = self.schme_nm { val.validate()? }
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


// GenericIdentification3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericIdentification3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericIdentification3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
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


// GenericIdentification36 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericIdentification36 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
	pub issr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<String>,
}

impl GenericIdentification36 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if self.issr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
		}
		if self.issr.chars().count() > 35 {
			return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
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


// GenericIdentification47 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericIdentification47 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
	pub issr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<String>,
}

impl GenericIdentification47 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
		if !pattern.is_match(&self.id) {
			return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
		}
		if self.issr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
		}
		if self.issr.chars().count() > 4 {
			return Err(ValidationError::new(1002, "issr exceeds the maximum length of 4".to_string()));
		}
		let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
		if !pattern.is_match(&self.issr) {
			return Err(ValidationError::new(1005, "issr does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.schme_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "schme_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "schme_nm exceeds the maximum length of 4".to_string()));
			}
			let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "schme_nm does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// GovernanceProcess1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GovernanceProcess1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<GovernanceProcessType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl GovernanceProcess1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// GovernanceProcessType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum GovernanceProcessType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "BMIF") )]
	CodeBMIF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NINF") )]
	CodeNINF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CMIF") )]
	CodeCMIF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AMIF") )]
	CodeAMIF,
}

impl GovernanceProcessType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// HoldingTransferable1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum HoldingTransferable1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRAL") )]
	CodeTRAL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRNA") )]
	CodeTRNA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RFOD") )]
	CodeRFOD,
}

impl HoldingTransferable1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// IdentificationSource3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct IdentificationSource3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl IdentificationSource3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
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


// IndividualCostOrCharge2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct IndividualCostOrCharge2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CostTp") )]
	pub cost_tp: ChargeType8Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExAnteOrExPst") )]
	pub ex_ante_or_ex_pst: IntendedOrActual2Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sgn", skip_serializing_if = "Option::is_none") )]
	pub sgn: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
	pub rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefPrd", skip_serializing_if = "Option::is_none") )]
	pub ref_prd: Option<Period15>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<AdditionalInformation15>,
}

impl IndividualCostOrCharge2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cost_tp.validate()?;
		self.ex_ante_or_ex_pst.validate()?;
		if let Some(ref val) = self.amt { val.validate()? }
		if let Some(ref val) = self.ref_prd { val.validate()? }
		if let Some(ref val) = self.addtl_inf { val.validate()? }
		Ok(())
	}
}


// IntendedOrActual2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum IntendedOrActual2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ANTE") )]
	CodeANTE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "POST") )]
	CodePOST,
}

impl IntendedOrActual2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InvestmentFundMiFIDFee2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum InvestmentFundMiFIDFee2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "BORF") )]
	CodeBORF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DIS2") )]
	CodeDIS2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FES3") )]
	CodeFES3,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FEND") )]
	CodeFEND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FES2") )]
	CodeFES2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GOC1") )]
	CodeGOC1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GOCS") )]
	CodeGOCS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INCF") )]
	CodeINCF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INCS") )]
	CodeINCS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MNF1") )]
	CodeMNF1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MANS") )]
	CodeMANS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NET2") )]
	CodeNET2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NESF") )]
	CodeNESF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NETO") )]
	CodeNETO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NRAM") )]
	CodeNRAM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OOEA") )]
	CodeOOEA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OOSF") )]
	CodeOOSF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OOSS") )]
	CodeOOSS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BENS") )]
	CodeBENS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ENAC") )]
	CodeENAC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ENFX") )]
	CodeENFX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EXAC") )]
	CodeEXAC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ENBX") )]
	CodeENBX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BEND") )]
	CodeBEND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PENO") )]
	CodePENO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTES") )]
	CodeOTES,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OCAS") )]
	CodeOCAS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RPSS") )]
	CodeRPSS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRS1") )]
	CodeTRS1,
}

impl InvestmentFundMiFIDFee2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InvestmentFundPlanType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InvestmentFundPlanType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<InvestmentFundPlanType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification36>,
}

impl InvestmentFundPlanType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// InvestmentFundPlanType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum InvestmentFundPlanType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "INVP") )]
	CodeINVP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SWIP") )]
	CodeSWIP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WTHP") )]
	CodeWTHP,
}

impl InvestmentFundPlanType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InvestmentNeed2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InvestmentNeed2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<InvestmentNeed2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl InvestmentNeed2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// InvestmentNeed2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum InvestmentNeed2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NSPE") )]
	CodeNSPE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISLB") )]
	CodeISLB,
}

impl InvestmentNeed2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InvestmentPlanCharacteristics1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InvestmentPlanCharacteristics1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PlanTp") )]
	pub plan_tp: InvestmentFundPlanType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Frqcy", skip_serializing_if = "Option::is_none") )]
	pub frqcy: Option<Frequency20Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfInstlmts", skip_serializing_if = "Option::is_none") )]
	pub ttl_nb_of_instlmts: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Qty", skip_serializing_if = "Option::is_none") )]
	pub qty: Option<UnitsOrAmount1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PlanConttn", skip_serializing_if = "Option::is_none") )]
	pub plan_conttn: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSbcpt", skip_serializing_if = "Option::is_none") )]
	pub addtl_sbcpt: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSbcptFctn", skip_serializing_if = "Option::is_none") )]
	pub addtl_sbcpt_fctn: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}

impl InvestmentPlanCharacteristics1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.plan_tp.validate()?;
		if let Some(ref val) = self.frqcy { val.validate()? }
		if let Some(ref val) = self.qty { val.validate()? }
		if let Some(ref vec) = self.addtl_inf { for item in vec { item.validate()? } }
		Ok(())
	}
}


// InvestmentRestrictions3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InvestmentRestrictions3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinInitlSbcptAmt", skip_serializing_if = "Option::is_none") )]
	pub min_initl_sbcpt_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinInitlSbcptUnits", skip_serializing_if = "Option::is_none") )]
	pub min_initl_sbcpt_units: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinSbsqntSbcptAmt", skip_serializing_if = "Option::is_none") )]
	pub min_sbsqnt_sbcpt_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinSbsqntSbcptUnits", skip_serializing_if = "Option::is_none") )]
	pub min_sbsqnt_sbcpt_units: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MaxRedAmt", skip_serializing_if = "Option::is_none") )]
	pub max_red_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MaxRedUnits", skip_serializing_if = "Option::is_none") )]
	pub max_red_units: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinRedPctg", skip_serializing_if = "Option::is_none") )]
	pub min_red_pctg: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrRedRstrctns", skip_serializing_if = "Option::is_none") )]
	pub othr_red_rstrctns: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinSwtchSbcptAmt", skip_serializing_if = "Option::is_none") )]
	pub min_swtch_sbcpt_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinSwtchSbcptUnits", skip_serializing_if = "Option::is_none") )]
	pub min_swtch_sbcpt_units: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MaxSwtchRedAmt", skip_serializing_if = "Option::is_none") )]
	pub max_swtch_red_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MaxSwtchRedUnits", skip_serializing_if = "Option::is_none") )]
	pub max_swtch_red_units: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrSwtchRstrctns", skip_serializing_if = "Option::is_none") )]
	pub othr_swtch_rstrctns: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinHldgAmt", skip_serializing_if = "Option::is_none") )]
	pub min_hldg_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinHldgUnits", skip_serializing_if = "Option::is_none") )]
	pub min_hldg_units: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinHldgPrd", skip_serializing_if = "Option::is_none") )]
	pub min_hldg_prd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HldgTrfbl", skip_serializing_if = "Option::is_none") )]
	pub hldg_trfbl: Option<HoldingTransferable1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}

impl InvestmentRestrictions3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.min_initl_sbcpt_amt { val.validate()? }
		if let Some(ref val) = self.min_sbsqnt_sbcpt_amt { val.validate()? }
		if let Some(ref val) = self.max_red_amt { val.validate()? }
		if let Some(ref val) = self.othr_red_rstrctns {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "othr_red_rstrctns is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "othr_red_rstrctns exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.min_swtch_sbcpt_amt { val.validate()? }
		if let Some(ref val) = self.max_swtch_red_amt { val.validate()? }
		if let Some(ref val) = self.othr_swtch_rstrctns {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "othr_swtch_rstrctns is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "othr_swtch_rstrctns exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.min_hldg_amt { val.validate()? }
		if let Some(ref val) = self.min_hldg_prd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "min_hldg_prd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "min_hldg_prd exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.hldg_trfbl { val.validate()? }
		if let Some(ref vec) = self.addtl_inf { for item in vec { item.validate()? } }
		Ok(())
	}
}


// InvestorKnowledge1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InvestorKnowledge1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "BsicInvstr", skip_serializing_if = "Option::is_none") )]
	pub bsic_invstr: Option<TargetMarket1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InfrmdInvstr", skip_serializing_if = "Option::is_none") )]
	pub infrmd_invstr: Option<TargetMarket1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdvncdInvstr", skip_serializing_if = "Option::is_none") )]
	pub advncd_invstr: Option<TargetMarket1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExprtInvstrDE", skip_serializing_if = "Option::is_none") )]
	pub exprt_invstr_de: Option<TargetMarket1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<OtherTargetMarketInvestorKnowledge1>>,
}

impl InvestorKnowledge1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.bsic_invstr { val.validate()? }
		if let Some(ref val) = self.infrmd_invstr { val.validate()? }
		if let Some(ref val) = self.advncd_invstr { val.validate()? }
		if let Some(ref val) = self.exprt_invstr_de { val.validate()? }
		if let Some(ref vec) = self.othr { for item in vec { item.validate()? } }
		Ok(())
	}
}


// InvestorRequirements4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InvestorRequirements4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RtrPrflPrsrvtn", skip_serializing_if = "Option::is_none") )]
	pub rtr_prfl_prsrvtn: Option<TargetMarket1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RtrPrflGrwth", skip_serializing_if = "Option::is_none") )]
	pub rtr_prfl_grwth: Option<TargetMarket1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RtrPrflIncm", skip_serializing_if = "Option::is_none") )]
	pub rtr_prfl_incm: Option<TargetMarket1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RtrPrflHdgg", skip_serializing_if = "Option::is_none") )]
	pub rtr_prfl_hdgg: Option<TargetMarket1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OptnOrLvrgdRtrPrfl", skip_serializing_if = "Option::is_none") )]
	pub optn_or_lvrgd_rtr_prfl: Option<TargetMarket1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RtrPrflPnsnSchmeDE", skip_serializing_if = "Option::is_none") )]
	pub rtr_prfl_pnsn_schme_de: Option<TargetMarket1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinHldgPrd", skip_serializing_if = "Option::is_none") )]
	pub min_hldg_prd: Option<TimeHorizon2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SstnbltyPrefs", skip_serializing_if = "Option::is_none") )]
	pub sstnblty_prefs: Option<SustainabilityPreferences2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrSpcfcInvstmtNeed", skip_serializing_if = "Option::is_none") )]
	pub othr_spcfc_invstmt_need: Option<InvestmentNeed2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<OtherInvestmentNeed1>>,
}

impl InvestorRequirements4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rtr_prfl_prsrvtn { val.validate()? }
		if let Some(ref val) = self.rtr_prfl_grwth { val.validate()? }
		if let Some(ref val) = self.rtr_prfl_incm { val.validate()? }
		if let Some(ref val) = self.rtr_prfl_hdgg { val.validate()? }
		if let Some(ref val) = self.optn_or_lvrgd_rtr_prfl { val.validate()? }
		if let Some(ref val) = self.rtr_prfl_pnsn_schme_de { val.validate()? }
		if let Some(ref val) = self.min_hldg_prd { val.validate()? }
		if let Some(ref val) = self.sstnblty_prefs { val.validate()? }
		if let Some(ref val) = self.othr_spcfc_invstmt_need { val.validate()? }
		if let Some(ref vec) = self.othr { for item in vec { item.validate()? } }
		Ok(())
	}
}


// InvestorType2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InvestorType2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstrTpRtl", skip_serializing_if = "Option::is_none") )]
	pub invstr_tp_rtl: Option<TargetMarket1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstrTpPrfssnl", skip_serializing_if = "Option::is_none") )]
	pub invstr_tp_prfssnl: Option<TargetMarket5Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstrTpElgblCtrPty", skip_serializing_if = "Option::is_none") )]
	pub invstr_tp_elgbl_ctr_pty: Option<TargetMarket3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<OtherTargetMarketInvestor1>>,
}

impl InvestorType2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.invstr_tp_rtl { val.validate()? }
		if let Some(ref val) = self.invstr_tp_prfssnl { val.validate()? }
		if let Some(ref val) = self.invstr_tp_elgbl_ctr_pty { val.validate()? }
		if let Some(ref vec) = self.othr { for item in vec { item.validate()? } }
		Ok(())
	}
}


// InvestorType2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum InvestorType2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "BOT3") )]
	CodeBOT3,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EPRO") )]
	CodeEPRO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRF2") )]
	CodePRF2,
}

impl InvestorType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InvestorType3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum InvestorType3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "RETL") )]
	CodeRETL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRF2") )]
	CodePRF2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NEI1") )]
	CodeNEI1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BOT2") )]
	CodeBOT2,
}

impl InvestorType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InvestorType4Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum InvestorType4Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "BOT3") )]
	CodeBOT3,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NPRF") )]
	CodeNPRF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRF3") )]
	CodePRF3,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRF4") )]
	CodePRF4,
}

impl InvestorType4Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// LocalMarketAnnex6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct LocalMarketAnnex6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
	pub ctry: Vec<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LclOrdrDsk") )]
	pub lcl_ordr_dsk: OrderDesk1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SbcptPrcgChrtcs", skip_serializing_if = "Option::is_none") )]
	pub sbcpt_prcg_chrtcs: Option<ProcessingCharacteristics11>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RedPrcgChrtcs", skip_serializing_if = "Option::is_none") )]
	pub red_prcg_chrtcs: Option<ProcessingCharacteristics10>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SwtchPrcgChrtcs", skip_serializing_if = "Option::is_none") )]
	pub swtch_prcg_chrtcs: Option<ProcessingCharacteristics9>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshSttlmDtls", skip_serializing_if = "Option::is_none") )]
	pub csh_sttlm_dtls: Option<Vec<CashAccount205>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}

impl LocalMarketAnnex6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(&item) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
		self.lcl_ordr_dsk.validate()?;
		if let Some(ref val) = self.sbcpt_prcg_chrtcs { val.validate()? }
		if let Some(ref val) = self.red_prcg_chrtcs { val.validate()? }
		if let Some(ref val) = self.swtch_prcg_chrtcs { val.validate()? }
		if let Some(ref vec) = self.csh_sttlm_dtls { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.addtl_inf { for item in vec { item.validate()? } }
		Ok(())
	}
}


// LossBearing2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct LossBearing2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NoCptlLoss", skip_serializing_if = "Option::is_none") )]
	pub no_cptl_loss: Option<TargetMarket1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LtdCptlLoss", skip_serializing_if = "Option::is_none") )]
	pub ltd_cptl_loss: Option<TargetMarket1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LtdCptlLossLvl", skip_serializing_if = "Option::is_none") )]
	pub ltd_cptl_loss_lvl: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NoCptlGrnt", skip_serializing_if = "Option::is_none") )]
	pub no_cptl_grnt: Option<TargetMarket1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LossByndCptl", skip_serializing_if = "Option::is_none") )]
	pub loss_bynd_cptl: Option<TargetMarket1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<OtherTargetMarketLossBearing1>>,
}

impl LossBearing2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.no_cptl_loss { val.validate()? }
		if let Some(ref val) = self.ltd_cptl_loss { val.validate()? }
		if let Some(ref val) = self.no_cptl_grnt { val.validate()? }
		if let Some(ref val) = self.loss_bynd_cptl { val.validate()? }
		if let Some(ref vec) = self.othr { for item in vec { item.validate()? } }
		Ok(())
	}
}


// MainFundOrderDeskLocation1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MainFundOrderDeskLocation1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
	pub ctry: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TmZoneOffSet") )]
	pub tm_zone_off_set: UTCOffset1,
}

impl MainFundOrderDeskLocation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.ctry) {
			return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
		}
		self.tm_zone_off_set.validate()?;
		Ok(())
	}
}


// MarketPracticeVersion1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MarketPracticeVersion1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nb", skip_serializing_if = "Option::is_none") )]
	pub nb: Option<String>,
}

impl MarketPracticeVersion1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 35 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nb exceeds the maximum length of 35".to_string()));
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


// NotionalOrUnitBased1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NotionalOrUnitBased1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<NotionalOrUnitBased1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl NotionalOrUnitBased1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// NotionalOrUnitBased1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum NotionalOrUnitBased1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "UNIT") )]
	CodeUNIT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NOTI") )]
	CodeNOTI,
}

impl NotionalOrUnitBased1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OrderDesk1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OrderDesk1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrdrDsk", skip_serializing_if = "Option::is_none") )]
	pub ordr_dsk: Option<ContactAttributes5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClsrDts", skip_serializing_if = "Option::is_none") )]
	pub clsr_dts: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}

impl OrderDesk1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ordr_dsk { val.validate()? }
		if let Some(ref vec) = self.addtl_inf { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OtherDistributionStrategy1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OtherDistributionStrategy1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DstrbtnStrtgyTp", skip_serializing_if = "Option::is_none") )]
	pub dstrbtn_strtgy_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Trgt", skip_serializing_if = "Option::is_none") )]
	pub trgt: Option<DistributionStrategy1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<AdditionalInformation15>,
}

impl OtherDistributionStrategy1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dstrbtn_strtgy_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dstrbtn_strtgy_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "dstrbtn_strtgy_tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.trgt { val.validate()? }
		if let Some(ref val) = self.addtl_inf { val.validate()? }
		Ok(())
	}
}


// OtherIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OtherIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sfx", skip_serializing_if = "Option::is_none") )]
	pub sfx: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: IdentificationSource3Choice,
}

impl OtherIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.sfx {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sfx is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "sfx exceeds the maximum length of 16".to_string()));
			}
		}
		self.tp.validate()?;
		Ok(())
	}
}


// OtherInvestmentNeed1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OtherInvestmentNeed1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClntObjctvsAndNeedsTp", skip_serializing_if = "Option::is_none") )]
	pub clnt_objctvs_and_needs_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Trgt", skip_serializing_if = "Option::is_none") )]
	pub trgt: Option<TargetMarket1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<AdditionalInformation15>,
}

impl OtherInvestmentNeed1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.clnt_objctvs_and_needs_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "clnt_objctvs_and_needs_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "clnt_objctvs_and_needs_tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.trgt { val.validate()? }
		if let Some(ref val) = self.addtl_inf { val.validate()? }
		Ok(())
	}
}


// OtherReviewRelatedToValueAndOrChargesUKType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum OtherReviewRelatedToValueAndOrChargesUKType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "REVA") )]
	CodeREVA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REVO") )]
	CodeREVO,
}

impl OtherReviewRelatedToValueAndOrChargesUKType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OtherTargetMarket1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OtherTargetMarket1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrgtMktTp") )]
	pub trgt_mkt_tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<AdditionalInformation15>,
}

impl OtherTargetMarket1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.trgt_mkt_tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "trgt_mkt_tp is shorter than the minimum length of 1".to_string()));
		}
		if self.trgt_mkt_tp.chars().count() > 350 {
			return Err(ValidationError::new(1002, "trgt_mkt_tp exceeds the maximum length of 350".to_string()));
		}
		if let Some(ref val) = self.addtl_inf { val.validate()? }
		Ok(())
	}
}


// OtherTargetMarketInvestor1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OtherTargetMarketInvestor1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstrTp", skip_serializing_if = "Option::is_none") )]
	pub invstr_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Trgt", skip_serializing_if = "Option::is_none") )]
	pub trgt: Option<TargetMarket3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<AdditionalInformation15>,
}

impl OtherTargetMarketInvestor1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.invstr_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "invstr_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "invstr_tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.trgt { val.validate()? }
		if let Some(ref val) = self.addtl_inf { val.validate()? }
		Ok(())
	}
}


// OtherTargetMarketInvestorKnowledge1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OtherTargetMarketInvestorKnowledge1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstrKnwldgTp", skip_serializing_if = "Option::is_none") )]
	pub invstr_knwldg_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Trgt", skip_serializing_if = "Option::is_none") )]
	pub trgt: Option<TargetMarket1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<AdditionalInformation15>,
}

impl OtherTargetMarketInvestorKnowledge1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.invstr_knwldg_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "invstr_knwldg_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "invstr_knwldg_tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.trgt { val.validate()? }
		if let Some(ref val) = self.addtl_inf { val.validate()? }
		Ok(())
	}
}


// OtherTargetMarketLossBearing1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OtherTargetMarketLossBearing1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AbltyToBearLossesTp", skip_serializing_if = "Option::is_none") )]
	pub ablty_to_bear_losses_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Trgt", skip_serializing_if = "Option::is_none") )]
	pub trgt: Option<TargetMarket1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<AdditionalInformation15>,
}

impl OtherTargetMarketLossBearing1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ablty_to_bear_losses_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ablty_to_bear_losses_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ablty_to_bear_losses_tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.trgt { val.validate()? }
		if let Some(ref val) = self.addtl_inf { val.validate()? }
		Ok(())
	}
}


// OtherTargetMarketRiskTolerance1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OtherTargetMarketRiskTolerance1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RskTlrnceTp", skip_serializing_if = "Option::is_none") )]
	pub rsk_tlrnce_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Trgt", skip_serializing_if = "Option::is_none") )]
	pub trgt: Option<TargetMarket1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<AdditionalInformation15>,
}

impl OtherTargetMarketRiskTolerance1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rsk_tlrnce_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rsk_tlrnce_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "rsk_tlrnce_tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.trgt { val.validate()? }
		if let Some(ref val) = self.addtl_inf { val.validate()? }
		Ok(())
	}
}


// OutcomeOfCOLLAssessmentOfValueUKType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum OutcomeOfCOLLAssessmentOfValueUKType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "COL1") )]
	CodeCOL1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "COL2") )]
	CodeCOL2,
}

impl OutcomeOfCOLLAssessmentOfValueUKType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OutcomeOfPRINValueAssessmentOrReviewUKType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum OutcomeOfPRINValueAssessmentOrReviewUKType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRI2") )]
	CodePRI2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRI1") )]
	CodePRI1,
}

impl OutcomeOfPRINValueAssessmentOrReviewUKType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PartyIdentification125Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyIdentification125Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
	pub any_bic: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
	pub prtry_id: Option<GenericIdentification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
	pub nm_and_adr: Option<NameAndAddress5>,
}

impl PartyIdentification125Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.any_bic {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prtry_id { val.validate()? }
		if let Some(ref val) = self.nm_and_adr { val.validate()? }
		Ok(())
	}
}


// PartyIdentification139 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyIdentification139 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pty") )]
	pub pty: PartyIdentification125Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
}

impl PartyIdentification139 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pty.validate()?;
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// PaymentInstrument16 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentInstrument16 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrdrTp") )]
	pub ordr_tp: FundOrderType5Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrmTp") )]
	pub instrm_tp: FundPaymentType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}

impl PaymentInstrument16 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.ordr_tp.validate()?;
		self.instrm_tp.validate()?;
		if let Some(ref vec) = self.addtl_inf { for item in vec { item.validate()? } }
		Ok(())
	}
}


// Period15 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Period15 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt") )]
	pub start_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndDt") )]
	pub end_dt: String,
}

impl Period15 {
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


// ProcessingCharacteristics10 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ProcessingCharacteristics10 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealgCcyAccptd", skip_serializing_if = "Option::is_none") )]
	pub dealg_ccy_accptd: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RedAuthstn", skip_serializing_if = "Option::is_none") )]
	pub red_authstn: Option<Forms1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmtInd", skip_serializing_if = "Option::is_none") )]
	pub amt_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitsInd", skip_serializing_if = "Option::is_none") )]
	pub units_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rndg", skip_serializing_if = "Option::is_none") )]
	pub rndg: Option<RoundingDirection2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PctgInd", skip_serializing_if = "Option::is_none") )]
	pub pctg_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MainFndOrdrDskLctn", skip_serializing_if = "Option::is_none") )]
	pub main_fnd_ordr_dsk_lctn: Option<MainFundOrderDeskLocation1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealgFrqcy", skip_serializing_if = "Option::is_none") )]
	pub dealg_frqcy: Option<EventFrequency5Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealgFrqcyDesc", skip_serializing_if = "Option::is_none") )]
	pub dealg_frqcy_desc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealgCutOffTm", skip_serializing_if = "Option::is_none") )]
	pub dealg_cut_off_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealgCutOffTmFrame", skip_serializing_if = "Option::is_none") )]
	pub dealg_cut_off_tm_frame: Option<TimeFrame9>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealConfTm", skip_serializing_if = "Option::is_none") )]
	pub deal_conf_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealConfTmFrame", skip_serializing_if = "Option::is_none") )]
	pub deal_conf_tm_frame: Option<TimeFrame8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LtdPrd", skip_serializing_if = "Option::is_none") )]
	pub ltd_prd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmCycl", skip_serializing_if = "Option::is_none") )]
	pub sttlm_cycl: Option<TimeFrame8Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}

impl ProcessingCharacteristics10 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.dealg_ccy_accptd {
			for item in vec {
				let pattern = Regex::new("[A-Z]{3,3}").unwrap();
				if !pattern.is_match(&item) {
					return Err(ValidationError::new(1005, "dealg_ccy_accptd does not match the required pattern".to_string()));
				}
			}
		}
		if let Some(ref val) = self.red_authstn { val.validate()? }
		if let Some(ref val) = self.rndg { val.validate()? }
		if let Some(ref val) = self.main_fnd_ordr_dsk_lctn { val.validate()? }
		if let Some(ref val) = self.dealg_frqcy { val.validate()? }
		if let Some(ref val) = self.dealg_frqcy_desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dealg_frqcy_desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "dealg_frqcy_desc exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.dealg_cut_off_tm_frame { val.validate()? }
		if let Some(ref val) = self.deal_conf_tm_frame { val.validate()? }
		if let Some(ref val) = self.ltd_prd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ltd_prd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "ltd_prd exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.sttlm_cycl { val.validate()? }
		if let Some(ref vec) = self.addtl_inf { for item in vec { item.validate()? } }
		Ok(())
	}
}


// ProcessingCharacteristics11 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ProcessingCharacteristics11 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealgCcyAccptd", skip_serializing_if = "Option::is_none") )]
	pub dealg_ccy_accptd: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitlInvstmtAppl", skip_serializing_if = "Option::is_none") )]
	pub initl_invstmt_appl: Option<Forms1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SbsqntInvstmtAppl", skip_serializing_if = "Option::is_none") )]
	pub sbsqnt_invstmt_appl: Option<Forms1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmtInd", skip_serializing_if = "Option::is_none") )]
	pub amt_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitsInd", skip_serializing_if = "Option::is_none") )]
	pub units_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rndg", skip_serializing_if = "Option::is_none") )]
	pub rndg: Option<RoundingDirection2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MainFndOrdrDskLctn", skip_serializing_if = "Option::is_none") )]
	pub main_fnd_ordr_dsk_lctn: Option<MainFundOrderDeskLocation1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealgFrqcy", skip_serializing_if = "Option::is_none") )]
	pub dealg_frqcy: Option<EventFrequency5Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealgFrqcyDesc", skip_serializing_if = "Option::is_none") )]
	pub dealg_frqcy_desc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealgCutOffTm", skip_serializing_if = "Option::is_none") )]
	pub dealg_cut_off_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealgCutOffTmFrame", skip_serializing_if = "Option::is_none") )]
	pub dealg_cut_off_tm_frame: Option<TimeFrame9>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealConfTm", skip_serializing_if = "Option::is_none") )]
	pub deal_conf_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealConfTmFrame", skip_serializing_if = "Option::is_none") )]
	pub deal_conf_tm_frame: Option<TimeFrame11>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LtdPrd", skip_serializing_if = "Option::is_none") )]
	pub ltd_prd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmCycl", skip_serializing_if = "Option::is_none") )]
	pub sttlm_cycl: Option<TimeFrame7Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}

impl ProcessingCharacteristics11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.dealg_ccy_accptd {
			for item in vec {
				let pattern = Regex::new("[A-Z]{3,3}").unwrap();
				if !pattern.is_match(&item) {
					return Err(ValidationError::new(1005, "dealg_ccy_accptd does not match the required pattern".to_string()));
				}
			}
		}
		if let Some(ref val) = self.initl_invstmt_appl { val.validate()? }
		if let Some(ref val) = self.sbsqnt_invstmt_appl { val.validate()? }
		if let Some(ref val) = self.rndg { val.validate()? }
		if let Some(ref val) = self.main_fnd_ordr_dsk_lctn { val.validate()? }
		if let Some(ref val) = self.dealg_frqcy { val.validate()? }
		if let Some(ref val) = self.dealg_frqcy_desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dealg_frqcy_desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "dealg_frqcy_desc exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.dealg_cut_off_tm_frame { val.validate()? }
		if let Some(ref val) = self.deal_conf_tm_frame { val.validate()? }
		if let Some(ref val) = self.ltd_prd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ltd_prd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "ltd_prd exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.sttlm_cycl { val.validate()? }
		if let Some(ref vec) = self.addtl_inf { for item in vec { item.validate()? } }
		Ok(())
	}
}


// ProcessingCharacteristics12 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ProcessingCharacteristics12 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealgCcyAccptd", skip_serializing_if = "Option::is_none") )]
	pub dealg_ccy_accptd: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RedAuthstn", skip_serializing_if = "Option::is_none") )]
	pub red_authstn: Option<Forms1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmtInd", skip_serializing_if = "Option::is_none") )]
	pub amt_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitsInd", skip_serializing_if = "Option::is_none") )]
	pub units_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rndg", skip_serializing_if = "Option::is_none") )]
	pub rndg: Option<RoundingDirection2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PctgInd", skip_serializing_if = "Option::is_none") )]
	pub pctg_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MainFndOrdrDskLctn", skip_serializing_if = "Option::is_none") )]
	pub main_fnd_ordr_dsk_lctn: Option<MainFundOrderDeskLocation1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealgFrqcy", skip_serializing_if = "Option::is_none") )]
	pub dealg_frqcy: Option<EventFrequency5Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealgFrqcyDesc", skip_serializing_if = "Option::is_none") )]
	pub dealg_frqcy_desc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealgCutOffTm", skip_serializing_if = "Option::is_none") )]
	pub dealg_cut_off_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealgCutOffTmFrame", skip_serializing_if = "Option::is_none") )]
	pub dealg_cut_off_tm_frame: Option<TimeFrame9>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealConfTm", skip_serializing_if = "Option::is_none") )]
	pub deal_conf_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealConfTmFrame", skip_serializing_if = "Option::is_none") )]
	pub deal_conf_tm_frame: Option<TimeFrame10>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LtdPrd", skip_serializing_if = "Option::is_none") )]
	pub ltd_prd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmCycl", skip_serializing_if = "Option::is_none") )]
	pub sttlm_cycl: Option<TimeFrame8Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}

impl ProcessingCharacteristics12 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.dealg_ccy_accptd {
			for item in vec {
				let pattern = Regex::new("[A-Z]{3,3}").unwrap();
				if !pattern.is_match(&item) {
					return Err(ValidationError::new(1005, "dealg_ccy_accptd does not match the required pattern".to_string()));
				}
			}
		}
		if let Some(ref val) = self.red_authstn { val.validate()? }
		if let Some(ref val) = self.rndg { val.validate()? }
		if let Some(ref val) = self.main_fnd_ordr_dsk_lctn { val.validate()? }
		if let Some(ref val) = self.dealg_frqcy { val.validate()? }
		if let Some(ref val) = self.dealg_frqcy_desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dealg_frqcy_desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "dealg_frqcy_desc exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.dealg_cut_off_tm_frame { val.validate()? }
		if let Some(ref val) = self.deal_conf_tm_frame { val.validate()? }
		if let Some(ref val) = self.ltd_prd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ltd_prd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "ltd_prd exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.sttlm_cycl { val.validate()? }
		if let Some(ref vec) = self.addtl_inf { for item in vec { item.validate()? } }
		Ok(())
	}
}


// ProcessingCharacteristics9 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ProcessingCharacteristics9 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealgCcyAccptd", skip_serializing_if = "Option::is_none") )]
	pub dealg_ccy_accptd: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SwtchAuthstn", skip_serializing_if = "Option::is_none") )]
	pub swtch_authstn: Option<Forms1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmtInd", skip_serializing_if = "Option::is_none") )]
	pub amt_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitsInd", skip_serializing_if = "Option::is_none") )]
	pub units_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rndg", skip_serializing_if = "Option::is_none") )]
	pub rndg: Option<RoundingDirection2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MainFndOrdrDskLctn", skip_serializing_if = "Option::is_none") )]
	pub main_fnd_ordr_dsk_lctn: Option<MainFundOrderDeskLocation1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealgFrqcy", skip_serializing_if = "Option::is_none") )]
	pub dealg_frqcy: Option<EventFrequency5Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealgFrqcyDesc", skip_serializing_if = "Option::is_none") )]
	pub dealg_frqcy_desc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealgCutOffTm", skip_serializing_if = "Option::is_none") )]
	pub dealg_cut_off_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealgCutOffTmFrame", skip_serializing_if = "Option::is_none") )]
	pub dealg_cut_off_tm_frame: Option<TimeFrame9>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealConfTm", skip_serializing_if = "Option::is_none") )]
	pub deal_conf_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealConfTmFrame", skip_serializing_if = "Option::is_none") )]
	pub deal_conf_tm_frame: Option<TimeFrame8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LtdPrd", skip_serializing_if = "Option::is_none") )]
	pub ltd_prd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmCycl", skip_serializing_if = "Option::is_none") )]
	pub sttlm_cycl: Option<TimeFrame8Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}

impl ProcessingCharacteristics9 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.dealg_ccy_accptd {
			for item in vec {
				let pattern = Regex::new("[A-Z]{3,3}").unwrap();
				if !pattern.is_match(&item) {
					return Err(ValidationError::new(1005, "dealg_ccy_accptd does not match the required pattern".to_string()));
				}
			}
		}
		if let Some(ref val) = self.swtch_authstn { val.validate()? }
		if let Some(ref val) = self.rndg { val.validate()? }
		if let Some(ref val) = self.main_fnd_ordr_dsk_lctn { val.validate()? }
		if let Some(ref val) = self.dealg_frqcy { val.validate()? }
		if let Some(ref val) = self.dealg_frqcy_desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dealg_frqcy_desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "dealg_frqcy_desc exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.dealg_cut_off_tm_frame { val.validate()? }
		if let Some(ref val) = self.deal_conf_tm_frame { val.validate()? }
		if let Some(ref val) = self.ltd_prd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ltd_prd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "ltd_prd exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.sttlm_cycl { val.validate()? }
		if let Some(ref vec) = self.addtl_inf { for item in vec { item.validate()? } }
		Ok(())
	}
}


// ProductStructure1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ProductStructure1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<ProductStructure1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl ProductStructure1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// ProductStructure1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ProductStructure1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "BOND") )]
	CodeBOND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NUMM") )]
	CodeNUMM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UCMM") )]
	CodeUCMM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EXTC") )]
	CodeEXTC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UCIT") )]
	CodeUCIT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SSEC") )]
	CodeSSEC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SFUN") )]
	CodeSFUN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NUCI") )]
	CodeNUCI,
}

impl ProductStructure1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// QuotationType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct QuotationType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<QuotationType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl QuotationType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// QuotationType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum QuotationType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACTU") )]
	CodeACTU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRCT") )]
	CodePRCT,
}

impl QuotationType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ReferToFundOrderDesk1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ReferToFundOrderDesk1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "RFOD") )]
	CodeRFOD,
}

impl ReferToFundOrderDesk1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RiskLevel1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum RiskLevel1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "HIGH") )]
	CodeHIGH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LOWW") )]
	CodeLOWW,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MEDM") )]
	CodeMEDM,
}

impl RiskLevel1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RiskTolerance1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RiskTolerance1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RskTlrncePRIIPSMthdlgy", skip_serializing_if = "Option::is_none") )]
	pub rsk_tlrnce_priips_mthdlgy: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RskTlrnceUCITSMthdlgy", skip_serializing_if = "Option::is_none") )]
	pub rsk_tlrnce_ucits_mthdlgy: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RskTlrnceIntl", skip_serializing_if = "Option::is_none") )]
	pub rsk_tlrnce_intl: Option<RiskLevel1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RskTlrnceForNonPRIIPSAndNonUCITSES", skip_serializing_if = "Option::is_none") )]
	pub rsk_tlrnce_for_non_priips_and_non_ucitses: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NotForInvstrsWthTheLwstRskTlrnceDE", skip_serializing_if = "Option::is_none") )]
	pub not_for_invstrs_wth_the_lwst_rsk_tlrnce_de: Option<TargetMarket2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<OtherTargetMarketRiskTolerance1>>,
}

impl RiskTolerance1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rsk_tlrnce_intl { val.validate()? }
		if let Some(ref val) = self.not_for_invstrs_wth_the_lwst_rsk_tlrnce_de { val.validate()? }
		if let Some(ref vec) = self.othr { for item in vec { item.validate()? } }
		Ok(())
	}
}


// RoundingDirection2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum RoundingDirection2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "RDUP") )]
	CodeRDUP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RDWN") )]
	CodeRDWN,
}

impl RoundingDirection2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SecurityClassificationType2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecurityClassificationType2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CFI", skip_serializing_if = "Option::is_none") )]
	pub cfi: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AltrnClssfctn", skip_serializing_if = "Option::is_none") )]
	pub altrn_clssfctn: Option<GenericIdentification3>,
}

impl SecurityClassificationType2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cfi {
			let pattern = Regex::new("[A-Z]{6,6}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "cfi does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.altrn_clssfctn { val.validate()? }
		Ok(())
	}
}


// SecurityIdentification40 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecurityIdentification40 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrId", skip_serializing_if = "Option::is_none") )]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
	pub isin: Option<String>,
}

impl SecurityIdentification40 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.othr_id { for item in vec { item.validate()? } }
		if let Some(ref val) = self.desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.isin {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// SecurityIdentification47 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecurityIdentification47 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: SecurityIdentification40,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none") )]
	pub shrt_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClssTp", skip_serializing_if = "Option::is_none") )]
	pub clss_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UmbrllNm", skip_serializing_if = "Option::is_none") )]
	pub umbrll_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NewUmbrll", skip_serializing_if = "Option::is_none") )]
	pub new_umbrll: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none") )]
	pub clssfctn_tp: Option<SecurityClassificationType2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BaseCcy", skip_serializing_if = "Option::is_none") )]
	pub base_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfDmcl", skip_serializing_if = "Option::is_none") )]
	pub ctry_of_dmcl: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegdDstrbtnCtry", skip_serializing_if = "Option::is_none") )]
	pub regd_dstrbtn_ctry: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PdctTp", skip_serializing_if = "Option::is_none") )]
	pub pdct_tp: Option<ProductStructure1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<ContactAttributes5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IssrPdctGovncPrc", skip_serializing_if = "Option::is_none") )]
	pub issr_pdct_govnc_prc: Option<GovernanceProcess1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PdctCtgy", skip_serializing_if = "Option::is_none") )]
	pub pdct_ctgy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PdctCtgyDE", skip_serializing_if = "Option::is_none") )]
	pub pdct_ctgy_de: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlOrUnitBased", skip_serializing_if = "Option::is_none") )]
	pub ntnl_or_unit_based: Option<NotionalOrUnitBased1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QtnTp", skip_serializing_if = "Option::is_none") )]
	pub qtn_tp: Option<QuotationType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LvrgdOrCntngntLblty", skip_serializing_if = "Option::is_none") )]
	pub lvrgd_or_cntngnt_lblty: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NoRtrcssnInd", skip_serializing_if = "Option::is_none") )]
	pub no_rtrcssn_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExPstCostClctnBsis", skip_serializing_if = "Option::is_none") )]
	pub ex_pst_cost_clctn_bsis: Option<ExPostCostCalculationBasis1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}

impl SecurityIdentification47 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 350 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
		}
		if let Some(ref val) = self.shrt_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "shrt_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "shrt_nm exceeds the maximum length of 35".to_string()));
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
		if let Some(ref val) = self.umbrll_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "umbrll_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "umbrll_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.clssfctn_tp { val.validate()? }
		if let Some(ref val) = self.base_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "base_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ctry_of_dmcl {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry_of_dmcl does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.regd_dstrbtn_ctry {
			for item in vec {
				let pattern = Regex::new("[A-Z]{2,2}").unwrap();
				if !pattern.is_match(&item) {
					return Err(ValidationError::new(1005, "regd_dstrbtn_ctry does not match the required pattern".to_string()));
				}
			}
		}
		if let Some(ref val) = self.pdct_tp { val.validate()? }
		if let Some(ref val) = self.issr { val.validate()? }
		if let Some(ref val) = self.issr_pdct_govnc_prc { val.validate()? }
		if let Some(ref val) = self.pdct_ctgy {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pdct_ctgy is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "pdct_ctgy exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.pdct_ctgy_de {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pdct_ctgy_de is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "pdct_ctgy_de exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.ntnl_or_unit_based { val.validate()? }
		if let Some(ref val) = self.qtn_tp { val.validate()? }
		if let Some(ref val) = self.ex_pst_cost_clctn_bsis { val.validate()? }
		if let Some(ref vec) = self.addtl_inf { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SignatureType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum SignatureType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ORIG") )]
	CodeORIG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DIGI") )]
	CodeDIGI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ELEC") )]
	CodeELEC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NONE") )]
	CodeNONE,
}

impl SignatureType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SustainabilityPreferences2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum SustainabilityPreferences2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NEUT") )]
	CodeNEUT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YSCO") )]
	CodeYSCO,
}

impl SustainabilityPreferences2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TargetMarket1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TargetMarket1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<TargetMarket1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl TargetMarket1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// TargetMarket1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TargetMarket1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "YSCO") )]
	CodeYSCO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NEUT") )]
	CodeNEUT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NSCO") )]
	CodeNSCO,
}

impl TargetMarket1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TargetMarket2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TargetMarket2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NEUT") )]
	CodeNEUT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YSCO") )]
	CodeYSCO,
}

impl TargetMarket2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TargetMarket3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TargetMarket3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<InvestorType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<TargetMarket1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl TargetMarket3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.othr { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// TargetMarket3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TargetMarket3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "YSCO") )]
	CodeYSCO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NSCO") )]
	CodeNSCO,
}

impl TargetMarket3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TargetMarket4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TargetMarket4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefDt", skip_serializing_if = "Option::is_none") )]
	pub ref_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstrTp", skip_serializing_if = "Option::is_none") )]
	pub invstr_tp: Option<InvestorType2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "KnwldgAndOrExprnc", skip_serializing_if = "Option::is_none") )]
	pub knwldg_and_or_exprnc: Option<InvestorKnowledge1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AbltyToBearLosses", skip_serializing_if = "Option::is_none") )]
	pub ablty_to_bear_losses: Option<LossBearing2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RskTlrnce", skip_serializing_if = "Option::is_none") )]
	pub rsk_tlrnce: Option<RiskTolerance1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClntObjctvsAndNeeds", skip_serializing_if = "Option::is_none") )]
	pub clnt_objctvs_and_needs: Option<InvestorRequirements4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<OtherTargetMarket1>>,
}

impl TargetMarket4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.invstr_tp { val.validate()? }
		if let Some(ref val) = self.knwldg_and_or_exprnc { val.validate()? }
		if let Some(ref val) = self.ablty_to_bear_losses { val.validate()? }
		if let Some(ref val) = self.rsk_tlrnce { val.validate()? }
		if let Some(ref val) = self.clnt_objctvs_and_needs { val.validate()? }
		if let Some(ref vec) = self.othr { for item in vec { item.validate()? } }
		Ok(())
	}
}


// TargetMarket5Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TargetMarket5Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<InvestorType4Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<TargetMarket1Code>,
}

impl TargetMarket5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.othr { val.validate()? }
		Ok(())
	}
}


// TimeFrame10 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TimeFrame10 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrTmFrameDesc", skip_serializing_if = "Option::is_none") )]
	pub othr_tm_frame_desc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TPlus", skip_serializing_if = "Option::is_none") )]
	pub t_plus: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NonWorkgDayAdjstmnt", skip_serializing_if = "Option::is_none") )]
	pub non_workg_day_adjstmnt: Option<BusinessDayConvention1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefrToOrdrDsk", skip_serializing_if = "Option::is_none") )]
	pub refr_to_ordr_dsk: Option<ReferToFundOrderDesk1Code>,
}

impl TimeFrame10 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.othr_tm_frame_desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "othr_tm_frame_desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "othr_tm_frame_desc exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.non_workg_day_adjstmnt { val.validate()? }
		if let Some(ref val) = self.refr_to_ordr_dsk { val.validate()? }
		Ok(())
	}
}


// TimeFrame11 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TimeFrame11 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrTmFrameDesc", skip_serializing_if = "Option::is_none") )]
	pub othr_tm_frame_desc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TPlus", skip_serializing_if = "Option::is_none") )]
	pub t_plus: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NonWorkgDayAdjstmnt", skip_serializing_if = "Option::is_none") )]
	pub non_workg_day_adjstmnt: Option<BusinessDayConvention1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefrToOrdrDsk", skip_serializing_if = "Option::is_none") )]
	pub refr_to_ordr_dsk: Option<ReferToFundOrderDesk1Code>,
}

impl TimeFrame11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.othr_tm_frame_desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "othr_tm_frame_desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "othr_tm_frame_desc exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.non_workg_day_adjstmnt { val.validate()? }
		if let Some(ref val) = self.refr_to_ordr_dsk { val.validate()? }
		Ok(())
	}
}


// TimeFrame2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TimeFrame2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "HOLD") )]
	CodeHOLD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LONG") )]
	CodeLONG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MEDM") )]
	CodeMEDM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SHOR") )]
	CodeSHOR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VSHT") )]
	CodeVSHT,
}

impl TimeFrame2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TimeFrame7Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TimeFrame7Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TPlus", skip_serializing_if = "Option::is_none") )]
	pub t_plus: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prepmt", skip_serializing_if = "Option::is_none") )]
	pub prepmt: Option<bool>,
}

impl TimeFrame7Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TimeFrame8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TimeFrame8 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrTmFrameDesc", skip_serializing_if = "Option::is_none") )]
	pub othr_tm_frame_desc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TPlus", skip_serializing_if = "Option::is_none") )]
	pub t_plus: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NonWorkgDayAdjstmnt", skip_serializing_if = "Option::is_none") )]
	pub non_workg_day_adjstmnt: Option<BusinessDayConvention1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefrToOrdrDsk", skip_serializing_if = "Option::is_none") )]
	pub refr_to_ordr_dsk: Option<ReferToFundOrderDesk1Code>,
}

impl TimeFrame8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.othr_tm_frame_desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "othr_tm_frame_desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "othr_tm_frame_desc exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.non_workg_day_adjstmnt { val.validate()? }
		if let Some(ref val) = self.refr_to_ordr_dsk { val.validate()? }
		Ok(())
	}
}


// TimeFrame8Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TimeFrame8Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TPlus", skip_serializing_if = "Option::is_none") )]
	pub t_plus: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RPlus", skip_serializing_if = "Option::is_none") )]
	pub r_plus: Option<f64>,
}

impl TimeFrame8Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TimeFrame9 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TimeFrame9 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrTmFrameDesc", skip_serializing_if = "Option::is_none") )]
	pub othr_tm_frame_desc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TMns", skip_serializing_if = "Option::is_none") )]
	pub t_mns: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NonWorkgDayAdjstmnt", skip_serializing_if = "Option::is_none") )]
	pub non_workg_day_adjstmnt: Option<BusinessDayConvention1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefrToOrdrDsk", skip_serializing_if = "Option::is_none") )]
	pub refr_to_ordr_dsk: Option<ReferToFundOrderDesk1Code>,
}

impl TimeFrame9 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.othr_tm_frame_desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "othr_tm_frame_desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "othr_tm_frame_desc exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.non_workg_day_adjstmnt { val.validate()? }
		if let Some(ref val) = self.refr_to_ordr_dsk { val.validate()? }
		Ok(())
	}
}


// TimeFrame9Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TimeFrame9Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<TimeFrame2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl TimeFrame9Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// TimeHorizon2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TimeHorizon2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfYrs", skip_serializing_if = "Option::is_none") )]
	pub nb_of_yrs: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TmFrame", skip_serializing_if = "Option::is_none") )]
	pub tm_frame: Option<TimeFrame9Choice>,
}

impl TimeHorizon2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tm_frame { val.validate()? }
		Ok(())
	}
}


// UTCOffset1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UTCOffset1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sgn") )]
	pub sgn: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfHrs") )]
	pub nb_of_hrs: String,
}

impl UTCOffset1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// UnitsOrAmount1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UnitsOrAmount1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Unit", skip_serializing_if = "Option::is_none") )]
	pub unit: Option<f64>,
}

impl UnitsOrAmount1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.amt { val.validate()? }
		Ok(())
	}
}


// ValuationDealingProcessingCharacteristics3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ValuationDealingProcessingCharacteristics3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValtnFrqcy", skip_serializing_if = "Option::is_none") )]
	pub valtn_frqcy: Option<EventFrequency5Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValtnFrqcyDesc", skip_serializing_if = "Option::is_none") )]
	pub valtn_frqcy_desc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValtnTm", skip_serializing_if = "Option::is_none") )]
	pub valtn_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DcmlstnUnits", skip_serializing_if = "Option::is_none") )]
	pub dcmlstn_units: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DcmlstnPric", skip_serializing_if = "Option::is_none") )]
	pub dcmlstn_pric: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DualFndInd", skip_serializing_if = "Option::is_none") )]
	pub dual_fnd_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricMtd", skip_serializing_if = "Option::is_none") )]
	pub pric_mtd: Option<PriceMethod1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricCcy", skip_serializing_if = "Option::is_none") )]
	pub pric_ccy: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}

impl ValuationDealingProcessingCharacteristics3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.valtn_frqcy { val.validate()? }
		if let Some(ref val) = self.valtn_frqcy_desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "valtn_frqcy_desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "valtn_frqcy_desc exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.pric_mtd { val.validate()? }
		if let Some(ref vec) = self.pric_ccy {
			for item in vec {
				let pattern = Regex::new("[A-Z]{3,3}").unwrap();
				if !pattern.is_match(&item) {
					return Err(ValidationError::new(1005, "pric_ccy does not match the required pattern".to_string()));
				}
			}
		}
		if let Some(ref vec) = self.addtl_inf { for item in vec { item.validate()? } }
		Ok(())
	}
}


// ValueForMoney1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ValueForMoney1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "EMTDataRptgVFMUK", skip_serializing_if = "Option::is_none") )]
	pub emt_data_rptg_vfmuk: Option<EMTDataReportingVFMUKType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AssmntOfValReqrdUdrCOLLUK", skip_serializing_if = "Option::is_none") )]
	pub assmnt_of_val_reqrd_udr_colluk: Option<AssessmentOfValueRequiredUnderCOLLUKType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OutcmOfCOLLAssmntOfValUK", skip_serializing_if = "Option::is_none") )]
	pub outcm_of_coll_assmnt_of_val_uk: Option<OutcomeOfCOLLAssessmentOfValueUKType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OutcmOfPRINValAssmntOrRvwUK", skip_serializing_if = "Option::is_none") )]
	pub outcm_of_prin_val_assmnt_or_rvw_uk: Option<OutcomeOfPRINValueAssessmentOrReviewUKType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrRvwRltdToValAndOrChrgsUK", skip_serializing_if = "Option::is_none") )]
	pub othr_rvw_rltd_to_val_and_or_chrgs_uk: Option<OtherReviewRelatedToValueAndOrChargesUKType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrthrInfUK", skip_serializing_if = "Option::is_none") )]
	pub frthr_inf_uk: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RvwDtUK", skip_serializing_if = "Option::is_none") )]
	pub rvw_dt_uk: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RvwNxtDueUK", skip_serializing_if = "Option::is_none") )]
	pub rvw_nxt_due_uk: Option<String>,
}

impl ValueForMoney1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.emt_data_rptg_vfmuk { val.validate()? }
		if let Some(ref val) = self.assmnt_of_val_reqrd_udr_colluk { val.validate()? }
		if let Some(ref val) = self.outcm_of_coll_assmnt_of_val_uk { val.validate()? }
		if let Some(ref val) = self.outcm_of_prin_val_assmnt_or_rvw_uk { val.validate()? }
		if let Some(ref val) = self.othr_rvw_rltd_to_val_and_or_chrgs_uk { val.validate()? }
		if let Some(ref val) = self.frthr_inf_uk {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "frthr_inf_uk is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "frthr_inf_uk exceeds the maximum length of 350".to_string()));
			}
		}
		Ok(())
	}
}
