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


// CashInForecast6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CashInForecast6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshSttlmDt") )]
	pub csh_sttlm_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubTtlAmt", skip_serializing_if = "Option::is_none") )]
	pub sub_ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubTtlUnitsNb", skip_serializing_if = "Option::is_none") )]
	pub sub_ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XcptnlCshFlowInd", skip_serializing_if = "Option::is_none") )]
	pub xcptnl_csh_flow_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlBal", skip_serializing_if = "Option::is_none") )]
	pub addtl_bal: Option<FundBalance1>,
}

impl CashInForecast6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.sub_ttl_amt { val.validate()? }
		if let Some(ref val) = self.sub_ttl_units_nb { val.validate()? }
		if let Some(ref val) = self.addtl_bal { val.validate()? }
		Ok(())
	}
}


// CashInOutForecast7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CashInOutForecast7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshSttlmDt", skip_serializing_if = "Option::is_none") )]
	pub csh_sttlm_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}

impl CashInOutForecast7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		Ok(())
	}
}


// CashOutForecast6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CashOutForecast6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshSttlmDt") )]
	pub csh_sttlm_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubTtlAmt", skip_serializing_if = "Option::is_none") )]
	pub sub_ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubTtlUnitsNb", skip_serializing_if = "Option::is_none") )]
	pub sub_ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XcptnlCshFlowInd", skip_serializing_if = "Option::is_none") )]
	pub xcptnl_csh_flow_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlBal", skip_serializing_if = "Option::is_none") )]
	pub addtl_bal: Option<FundBalance1>,
}

impl CashOutForecast6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.sub_ttl_amt { val.validate()? }
		if let Some(ref val) = self.sub_ttl_units_nb { val.validate()? }
		if let Some(ref val) = self.addtl_bal { val.validate()? }
		Ok(())
	}
}


// CurrencyDesignation1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CurrencyDesignation1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CcyDsgnt", skip_serializing_if = "Option::is_none") )]
	pub ccy_dsgnt: Option<CurrencyDesignation1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Lctn", skip_serializing_if = "Option::is_none") )]
	pub lctn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<String>,
}

impl CurrencyDesignation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ccy_dsgnt { val.validate()? }
		if let Some(ref val) = self.lctn {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lctn does not match the required pattern".to_string()));
			}
		}
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


// CurrencyDesignation1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum CurrencyDesignation1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ONSH") )]
	CodeONSH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OFFS") )]
	CodeOFFS,
}

impl CurrencyDesignation1Code {
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


// FinancialInstrument9 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrument9 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: SecurityIdentification3Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryId", skip_serializing_if = "Option::is_none") )]
	pub splmtry_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdNAVCcy", skip_serializing_if = "Option::is_none") )]
	pub reqd_nav_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClssTp", skip_serializing_if = "Option::is_none") )]
	pub clss_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesForm", skip_serializing_if = "Option::is_none") )]
	pub scties_form: Option<FormOfSecurity1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DstrbtnPlcy", skip_serializing_if = "Option::is_none") )]
	pub dstrbtn_plcy: Option<DistributionPolicy1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DualFndInd") )]
	pub dual_fnd_ind: bool,
}

impl FinancialInstrument9 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
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
		if let Some(ref val) = self.reqd_nav_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "reqd_nav_ccy does not match the required pattern".to_string()));
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


// FlowDirectionType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum FlowDirectionType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "INCG") )]
	CodeINCG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OUTG") )]
	CodeOUTG,
}

impl FlowDirectionType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ForeignExchangeTerms19 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ForeignExchangeTerms19 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitCcy") )]
	pub unit_ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QtdCcy") )]
	pub qtd_ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XchgRate") )]
	pub xchg_rate: f64,
}

impl ForeignExchangeTerms19 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.unit_ccy) {
			return Err(ValidationError::new(1005, "unit_ccy does not match the required pattern".to_string()));
		}
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.qtd_ccy) {
			return Err(ValidationError::new(1005, "qtd_ccy does not match the required pattern".to_string()));
		}
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


// Fund2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Fund2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none") )]
	pub lgl_ntty_idr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<OtherIdentification4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy", skip_serializing_if = "Option::is_none") )]
	pub ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradDtTm", skip_serializing_if = "Option::is_none") )]
	pub trad_dt_tm: Option<DateAndDateTimeChoice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsTradDtTm", skip_serializing_if = "Option::is_none") )]
	pub prvs_trad_dt_tm: Option<DateAndDateTimeChoice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNAV", skip_serializing_if = "Option::is_none") )]
	pub ttl_nav: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsTtlNAV", skip_serializing_if = "Option::is_none") )]
	pub prvs_ttl_nav: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlUnitsNb", skip_serializing_if = "Option::is_none") )]
	pub ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsTtlUnitsNb", skip_serializing_if = "Option::is_none") )]
	pub prvs_ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PctgOfFndTtlNAV", skip_serializing_if = "Option::is_none") )]
	pub pctg_of_fnd_ttl_nav: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshInFcstDtls", skip_serializing_if = "Option::is_none") )]
	pub csh_in_fcst_dtls: Option<Vec<CashInOutForecast7>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshOutFcstDtls", skip_serializing_if = "Option::is_none") )]
	pub csh_out_fcst_dtls: Option<Vec<CashInOutForecast7>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetCshFcstDtls", skip_serializing_if = "Option::is_none") )]
	pub net_csh_fcst_dtls: Option<Vec<NetCashForecast5>>,
}

impl Fund2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.lgl_ntty_idr {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lgl_ntty_idr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.id { val.validate()? }
		if let Some(ref val) = self.ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.trad_dt_tm { val.validate()? }
		if let Some(ref val) = self.prvs_trad_dt_tm { val.validate()? }
		if let Some(ref val) = self.ttl_nav { val.validate()? }
		if let Some(ref val) = self.prvs_ttl_nav { val.validate()? }
		if let Some(ref val) = self.ttl_units_nb { val.validate()? }
		if let Some(ref val) = self.prvs_ttl_units_nb { val.validate()? }
		if let Some(ref vec) = self.csh_in_fcst_dtls { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.csh_out_fcst_dtls { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.net_csh_fcst_dtls { for item in vec { item.validate()? } }
		Ok(())
	}
}


// FundBalance1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FundBalance1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlUnitsFrUnitOrdrs", skip_serializing_if = "Option::is_none") )]
	pub ttl_units_fr_unit_ordrs: Option<FinancialInstrumentQuantity1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlUnitsFrCshOrdrs", skip_serializing_if = "Option::is_none") )]
	pub ttl_units_fr_csh_ordrs: Option<FinancialInstrumentQuantity1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlCshFrUnitOrdrs", skip_serializing_if = "Option::is_none") )]
	pub ttl_csh_fr_unit_ordrs: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlCshFrCshOrdrs", skip_serializing_if = "Option::is_none") )]
	pub ttl_csh_fr_csh_ordrs: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl FundBalance1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ttl_units_fr_unit_ordrs { val.validate()? }
		if let Some(ref val) = self.ttl_units_fr_csh_ordrs { val.validate()? }
		if let Some(ref val) = self.ttl_csh_fr_unit_ordrs { val.validate()? }
		if let Some(ref val) = self.ttl_csh_fr_csh_ordrs { val.validate()? }
		Ok(())
	}
}


// FundCashForecast7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FundCashForecast7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradDtTm") )]
	pub trad_dt_tm: DateAndDateTimeChoice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsTradDtTm", skip_serializing_if = "Option::is_none") )]
	pub prvs_trad_dt_tm: Option<DateAndDateTimeChoice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmDtls") )]
	pub fin_instrm_dtls: FinancialInstrument9,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNAV", skip_serializing_if = "Option::is_none") )]
	pub ttl_nav: Option<Vec<ActiveOrHistoricCurrencyAndAmount>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsTtlNAV", skip_serializing_if = "Option::is_none") )]
	pub prvs_ttl_nav: Option<Vec<ActiveOrHistoricCurrencyAndAmount>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlUnitsNb", skip_serializing_if = "Option::is_none") )]
	pub ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsTtlUnitsNb", skip_serializing_if = "Option::is_none") )]
	pub prvs_ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNAVChngRate", skip_serializing_if = "Option::is_none") )]
	pub ttl_nav_chng_rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstmtCcy", skip_serializing_if = "Option::is_none") )]
	pub invstmt_ccy: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CcySts", skip_serializing_if = "Option::is_none") )]
	pub ccy_sts: Option<CurrencyDesignation1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XcptnlNetCshFlowInd") )]
	pub xcptnl_net_csh_flow_ind: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pric", skip_serializing_if = "Option::is_none") )]
	pub pric: Option<UnitPrice19>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FXRate", skip_serializing_if = "Option::is_none") )]
	pub fx_rate: Option<ForeignExchangeTerms19>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PctgOfShrClssTtlNAV", skip_serializing_if = "Option::is_none") )]
	pub pctg_of_shr_clss_ttl_nav: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshInFcstDtls", skip_serializing_if = "Option::is_none") )]
	pub csh_in_fcst_dtls: Option<Vec<CashInForecast6>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshOutFcstDtls", skip_serializing_if = "Option::is_none") )]
	pub csh_out_fcst_dtls: Option<Vec<CashOutForecast6>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetCshFcstDtls", skip_serializing_if = "Option::is_none") )]
	pub net_csh_fcst_dtls: Option<Vec<NetCashForecast4>>,
}

impl FundCashForecast7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		self.trad_dt_tm.validate()?;
		if let Some(ref val) = self.prvs_trad_dt_tm { val.validate()? }
		self.fin_instrm_dtls.validate()?;
		if let Some(ref vec) = self.ttl_nav { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.prvs_ttl_nav { for item in vec { item.validate()? } }
		if let Some(ref val) = self.ttl_units_nb { val.validate()? }
		if let Some(ref val) = self.prvs_ttl_units_nb { val.validate()? }
		if let Some(ref vec) = self.invstmt_ccy {
			for item in vec {
				let pattern = Regex::new("[A-Z]{3,3}").unwrap();
				if !pattern.is_match(&item) {
					return Err(ValidationError::new(1005, "invstmt_ccy does not match the required pattern".to_string()));
				}
			}
		}
		if let Some(ref val) = self.ccy_sts { val.validate()? }
		if let Some(ref val) = self.pric { val.validate()? }
		if let Some(ref val) = self.fx_rate { val.validate()? }
		if let Some(ref vec) = self.csh_in_fcst_dtls { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.csh_out_fcst_dtls { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.net_csh_fcst_dtls { for item in vec { item.validate()? } }
		Ok(())
	}
}


// FundConfirmedCashForecastReportV04 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FundConfirmedCashForecastReportV04 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: MessageIdentification1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PoolRef", skip_serializing_if = "Option::is_none") )]
	pub pool_ref: Option<AdditionalReference3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsRef", skip_serializing_if = "Option::is_none") )]
	pub prvs_ref: Option<Vec<AdditionalReference3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdRef", skip_serializing_if = "Option::is_none") )]
	pub rltd_ref: Option<Vec<AdditionalReference3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgPgntn") )]
	pub msg_pgntn: Pagination,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FndOrSubFndDtls", skip_serializing_if = "Option::is_none") )]
	pub fnd_or_sub_fnd_dtls: Option<Vec<Fund2>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FndCshFcstDtls", skip_serializing_if = "Option::is_none") )]
	pub fnd_csh_fcst_dtls: Option<Vec<FundCashForecast7>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CnsltdNetCshFcst", skip_serializing_if = "Option::is_none") )]
	pub cnsltd_net_csh_fcst: Option<NetCashForecast3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Xtnsn", skip_serializing_if = "Option::is_none") )]
	pub xtnsn: Option<Vec<Extension1>>,
}

impl FundConfirmedCashForecastReportV04 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.msg_id.validate()?;
		if let Some(ref val) = self.pool_ref { val.validate()? }
		if let Some(ref vec) = self.prvs_ref { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.rltd_ref { for item in vec { item.validate()? } }
		self.msg_pgntn.validate()?;
		if let Some(ref vec) = self.fnd_or_sub_fnd_dtls { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.fnd_csh_fcst_dtls { for item in vec { item.validate()? } }
		if let Some(ref val) = self.cnsltd_net_csh_fcst { val.validate()? }
		if let Some(ref vec) = self.xtnsn { for item in vec { item.validate()? } }
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


// IdentificationSource5Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct IdentificationSource5Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DmstIdSrc", skip_serializing_if = "Option::is_none") )]
	pub dmst_id_src: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryIdSrc", skip_serializing_if = "Option::is_none") )]
	pub prtry_id_src: Option<String>,
}

impl IdentificationSource5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
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


// NetCashForecast3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NetCashForecast3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetAmt", skip_serializing_if = "Option::is_none") )]
	pub net_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetUnitsNb", skip_serializing_if = "Option::is_none") )]
	pub net_units_nb: Option<FinancialInstrumentQuantity1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FlowDrctn") )]
	pub flow_drctn: FlowDirectionType1Code,
}

impl NetCashForecast3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.net_amt { val.validate()? }
		if let Some(ref val) = self.net_units_nb { val.validate()? }
		self.flow_drctn.validate()?;
		Ok(())
	}
}


// NetCashForecast4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NetCashForecast4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshSttlmDt") )]
	pub csh_sttlm_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetAmt", skip_serializing_if = "Option::is_none") )]
	pub net_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetUnitsNb", skip_serializing_if = "Option::is_none") )]
	pub net_units_nb: Option<FinancialInstrumentQuantity1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FlowDrctn") )]
	pub flow_drctn: FlowDirectionType1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlBal", skip_serializing_if = "Option::is_none") )]
	pub addtl_bal: Option<FundBalance1>,
}

impl NetCashForecast4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.net_amt { val.validate()? }
		if let Some(ref val) = self.net_units_nb { val.validate()? }
		self.flow_drctn.validate()?;
		if let Some(ref val) = self.addtl_bal { val.validate()? }
		Ok(())
	}
}


// NetCashForecast5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NetCashForecast5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshSttlmDt", skip_serializing_if = "Option::is_none") )]
	pub csh_sttlm_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetAmt", skip_serializing_if = "Option::is_none") )]
	pub net_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetUnitsNb", skip_serializing_if = "Option::is_none") )]
	pub net_units_nb: Option<FinancialInstrumentQuantity1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FlowDrctn") )]
	pub flow_drctn: FlowDirectionType1Code,
}

impl NetCashForecast5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.net_amt { val.validate()? }
		if let Some(ref val) = self.net_units_nb { val.validate()? }
		self.flow_drctn.validate()?;
		Ok(())
	}
}


// OtherIdentification4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OtherIdentification4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: IdentificationSource5Choice,
}

impl OtherIdentification4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		self.tp.validate()?;
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


// TypeOfPrice10Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TypeOfPrice10Code {
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
}

impl TypeOfPrice10Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// UnitPrice19 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UnitPrice19 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricTp") )]
	pub pric_tp: UnitPriceType2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
	pub val: PriceValue1,
}

impl UnitPrice19 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pric_tp.validate()?;
		self.val.validate()?;
		Ok(())
	}
}


// UnitPriceType2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UnitPriceType2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<TypeOfPrice10Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification47>,
}

impl UnitPriceType2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}
