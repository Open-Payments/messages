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

#![allow(unused_imports)]
use regex::Regex;

#[cfg(feature = "derive_serde")]
use serde::{Deserialize, Serialize};
use open_payments_common::ValidationError;

#[cfg(feature = "derive_samplify")]
use samplify_rs::Sampleable;


// AcceptedReason7Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AcceptedReason7Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification36>,
}

impl AcceptedReason7Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// AcceptedReason8Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AcceptedReason8Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<AcceptedReason7Choice>,
}

impl AcceptedReason8Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.no_spcfd_rsn { val.validate()? }
		if let Some(ref val) = self.rsn { val.validate()? }
		Ok(())
	}
}


// AcceptedStatusReason7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AcceptedStatusReason7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn") )]
	pub rsn: AcceptedReason8Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_rsn_inf: Option<String>,
}

impl AcceptedStatusReason7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rsn.validate()?;
		if let Some(ref val) = self.addtl_rsn_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_rsn_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 210 {
				return Err(ValidationError::new(1002, "addtl_rsn_inf exceeds the maximum length of 210".to_string()));
			}
		}
		Ok(())
	}
}


// AccountIdentification26 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountIdentification26 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry") )]
	pub prtry: SimpleIdentificationInformation4,
}

impl AccountIdentification26 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.prtry.validate()?;
		Ok(())
	}
}


// AccountIdentificationAndName7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// ActivationHeader3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ActivationHeader3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
	pub cre_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgOrgtr", skip_serializing_if = "Option::is_none") )]
	pub msg_orgtr: Option<RTPPartyIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgRcpt", skip_serializing_if = "Option::is_none") )]
	pub msg_rcpt: Option<RTPPartyIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitgPty") )]
	pub initg_pty: RTPPartyIdentification2,
}

impl ActivationHeader3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.msg_orgtr { val.validate()? }
		if let Some(ref val) = self.msg_rcpt { val.validate()? }
		self.initg_pty.validate()?;
		Ok(())
	}
}


// ActivationStatus3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ActivationStatus3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlBizInstr", skip_serializing_if = "Option::is_none") )]
	pub orgnl_biz_instr: Option<OriginalBusinessInstruction1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts") )]
	pub sts: ServiceStatus1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsRsn", skip_serializing_if = "Option::is_none") )]
	pub sts_rsn: Option<DebtorActivationStatusReason3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlActvtnRef", skip_serializing_if = "Option::is_none") )]
	pub orgnl_actvtn_ref: Option<OriginalActivation3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FctvActvtnDt", skip_serializing_if = "Option::is_none") )]
	pub fctv_actvtn_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl ActivationStatus3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_biz_instr { val.validate()? }
		self.sts.validate()?;
		if let Some(ref val) = self.sts_rsn { val.validate()? }
		if let Some(ref val) = self.orgnl_actvtn_ref { val.validate()? }
		if let Some(ref val) = self.fctv_actvtn_dt { val.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// ActiveCurrencyAnd13DecimalAmount ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// ActiveOrHistoricCurrencyAnd13DecimalAmount ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// AdditionalInformation15 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// AdditionalReference3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// AddressType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum AddressType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "HOME") )]
	CodeHOME,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BIZZ") )]
	CodeBIZZ,
}

impl AddressType1Code {
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// AddressType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AddressType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<AddressType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl AddressType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// AlternateSecurityIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// AmountOrPercentageRange1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AmountOrPercentageRange1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Opr", skip_serializing_if = "Option::is_none") )]
	pub opr: Option<Operation1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Term", skip_serializing_if = "Option::is_none") )]
	pub term: Option<Vec<Term1>>,
}

impl AmountOrPercentageRange1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.opr { val.validate()? }
		if let Some(ref vec) = self.term { for item in vec { item.validate()? } }
		Ok(())
	}
}


// AnnualChargePaymentType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// Appearance1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum Appearance1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DELI") )]
	CodeDELI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NDEL") )]
	CodeNDEL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LIMI") )]
	CodeLIMI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BENT") )]
	CodeBENT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DFBE") )]
	CodeDFBE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DLBE") )]
	CodeDLBE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TMPG") )]
	CodeTMPG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GLOB") )]
	CodeGLOB,
}

impl Appearance1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Appearance3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Appearance3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<Appearance1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl Appearance3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// AssessmentOfValueRequiredUnderCOLLUKType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// AssignmentMethod1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum AssignmentMethod1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "RAND") )]
	CodeRAND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PROR") )]
	CodePROR,
}

impl AssignmentMethod1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AssignmentMethod2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AssignmentMethod2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<AssignmentMethod1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl AssignmentMethod2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// AuditTrail1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AuditTrail1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FldNm") )]
	pub fld_nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OdFldVal") )]
	pub od_fld_val: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NewFldVal") )]
	pub new_fld_val: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OprTmStmp") )]
	pub opr_tm_stmp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstgUsr") )]
	pub instg_usr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ApprvgUsr", skip_serializing_if = "Option::is_none") )]
	pub apprvg_usr: Option<String>,
}

impl AuditTrail1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.fld_nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "fld_nm is shorter than the minimum length of 1".to_string()));
		}
		if self.fld_nm.chars().count() > 35 {
			return Err(ValidationError::new(1002, "fld_nm exceeds the maximum length of 35".to_string()));
		}
		if self.od_fld_val.chars().count() < 1 {
			return Err(ValidationError::new(1001, "od_fld_val is shorter than the minimum length of 1".to_string()));
		}
		if self.od_fld_val.chars().count() > 350 {
			return Err(ValidationError::new(1002, "od_fld_val exceeds the maximum length of 350".to_string()));
		}
		if self.new_fld_val.chars().count() < 1 {
			return Err(ValidationError::new(1001, "new_fld_val is shorter than the minimum length of 1".to_string()));
		}
		if self.new_fld_val.chars().count() > 350 {
			return Err(ValidationError::new(1002, "new_fld_val exceeds the maximum length of 350".to_string()));
		}
		if self.instg_usr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "instg_usr is shorter than the minimum length of 1".to_string()));
		}
		if self.instg_usr.chars().count() > 256 {
			return Err(ValidationError::new(1002, "instg_usr exceeds the maximum length of 256".to_string()));
		}
		if let Some(ref val) = self.apprvg_usr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "apprvg_usr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "apprvg_usr exceeds the maximum length of 256".to_string()));
			}
		}
		Ok(())
	}
}


// AuditTrailOrBusinessError6Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AuditTrailOrBusinessError6Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AudtTrl", skip_serializing_if = "Option::is_none") )]
	pub audt_trl: Option<Vec<AuditTrail1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BizErr", skip_serializing_if = "Option::is_none") )]
	pub biz_err: Option<Vec<ErrorHandling5>>,
}

impl AuditTrailOrBusinessError6Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.audt_trl { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.biz_err { for item in vec { item.validate()? } }
		Ok(())
	}
}


// BenchmarkCurve6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct BenchmarkCurve6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sprd", skip_serializing_if = "Option::is_none") )]
	pub sprd: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BchmkId", skip_serializing_if = "Option::is_none") )]
	pub bchmk_id: Option<SecurityIdentification39>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BchmkPric", skip_serializing_if = "Option::is_none") )]
	pub bchmk_pric: Option<Price8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BchmkCrvCcy", skip_serializing_if = "Option::is_none") )]
	pub bchmk_crv_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BchmkCrvNm", skip_serializing_if = "Option::is_none") )]
	pub bchmk_crv_nm: Option<BenchmarkCurveName7Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BchmkCrvPt", skip_serializing_if = "Option::is_none") )]
	pub bchmk_crv_pt: Option<String>,
}

impl BenchmarkCurve6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.bchmk_id { val.validate()? }
		if let Some(ref val) = self.bchmk_pric { val.validate()? }
		if let Some(ref val) = self.bchmk_crv_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "bchmk_crv_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.bchmk_crv_nm { val.validate()? }
		if let Some(ref val) = self.bchmk_crv_pt {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "bchmk_crv_pt is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "bchmk_crv_pt exceeds the maximum length of 256".to_string()));
			}
		}
		Ok(())
	}
}


// BenchmarkCurveName1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum BenchmarkCurveName1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MAAA") )]
	CodeMAAA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FUSW") )]
	CodeFUSW,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LIBI") )]
	CodeLIBI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LIBO") )]
	CodeLIBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SWAP") )]
	CodeSWAP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TREA") )]
	CodeTREA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EURI") )]
	CodeEURI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PFAN") )]
	CodePFAN,
}

impl BenchmarkCurveName1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// BenchmarkCurveName7Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct BenchmarkCurveName7Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<BenchmarkCurveName1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl BenchmarkCurveName7Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// BusinessDayConvention1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// BusinessError4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct BusinessError4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmId") )]
	pub fin_instrm_id: SecurityIdentification39,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BizErr") )]
	pub biz_err: Vec<ErrorHandling5>,
}

impl BusinessError4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.fin_instrm_id.validate()?;
		for item in &self.biz_err { item.validate()? }
		Ok(())
	}
}


// CalculationBasis2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// CalculationType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum CalculationType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "AFTX") )]
	CodeAFTX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ANNU") )]
	CodeANNU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISSU") )]
	CodeISSU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AVMA") )]
	CodeAVMA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BOOK") )]
	CodeBOOK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YTNC") )]
	CodeYTNC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CHCL") )]
	CodeCHCL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLOS") )]
	CodeCLOS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CMPD") )]
	CodeCMPD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CUYI") )]
	CodeCUYI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRGR") )]
	CodeTRGR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GVEQ") )]
	CodeGVEQ,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FLAS") )]
	CodeFLAS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NVFL") )]
	CodeNVFL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LSCL") )]
	CodeLSCL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LSMT") )]
	CodeLSMT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LSQR") )]
	CodeLSQR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LSYR") )]
	CodeLSYR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LGAL") )]
	CodeLGAL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MARK") )]
	CodeMARK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YTMA") )]
	CodeYTMA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NXRF") )]
	CodeNXRF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PNAV") )]
	CodePNAV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NXPT") )]
	CodeNXPT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRCL") )]
	CodePRCL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRYL") )]
	CodePRYL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SEMI") )]
	CodeSEMI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SHLF") )]
	CodeSHLF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SPLL") )]
	CodeSPLL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TXQV") )]
	CodeTXQV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TTDT") )]
	CodeTTDT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRYL") )]
	CodeTRYL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WRST") )]
	CodeWRST,
}

impl CalculationType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CalculationType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CalculationType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<CalculationType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl CalculationType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// CalendarData1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CalendarData1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SysDt") )]
	pub sys_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SysSts") )]
	pub sys_sts: SystemStatus3Choice,
}

impl CalendarData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.sys_sts.validate()?;
		Ok(())
	}
}


// CalendarOrBusinessError1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CalendarOrBusinessError1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CalData", skip_serializing_if = "Option::is_none") )]
	pub cal_data: Option<Vec<CalendarData1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BizErr", skip_serializing_if = "Option::is_none") )]
	pub biz_err: Option<Vec<ErrorHandling4>>,
}

impl CalendarOrBusinessError1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.cal_data { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.biz_err { for item in vec { item.validate()? } }
		Ok(())
	}
}


// CalendarReport1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CalendarReport1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Svc", skip_serializing_if = "Option::is_none") )]
	pub svc: Option<SystemAndCurrency1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CalOrErr") )]
	pub cal_or_err: CalendarOrBusinessError1Choice,
}

impl CalendarReport1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.svc { val.validate()? }
		self.cal_or_err.validate()?;
		Ok(())
	}
}


// CalendarReportOrError1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CalendarReportOrError1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CalRpt", skip_serializing_if = "Option::is_none") )]
	pub cal_rpt: Option<CalendarReport1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OprlErr", skip_serializing_if = "Option::is_none") )]
	pub oprl_err: Option<Vec<ErrorHandling4>>,
}

impl CalendarReportOrError1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cal_rpt { val.validate()? }
		if let Some(ref vec) = self.oprl_err { for item in vec { item.validate()? } }
		Ok(())
	}
}


// CalendarSearchCriteria1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CalendarSearchCriteria1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Yr", skip_serializing_if = "Option::is_none") )]
	pub yr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mnth", skip_serializing_if = "Option::is_none") )]
	pub mnth: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Svc", skip_serializing_if = "Option::is_none") )]
	pub svc: Option<SystemAndCurrency1>,
}

impl CalendarSearchCriteria1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.svc { val.validate()? }
		Ok(())
	}
}


// CallType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum CallType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "LOTT") )]
	CodeLOTT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRTA") )]
	CodePRTA,
}

impl CallType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CallType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CallType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<CallType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl CallType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// CashAccount205 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// CashParties24 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CashParties24 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr") )]
	pub cdtr: PartyIdentificationAndAccount96,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt") )]
	pub cdtr_agt: PartyIdentificationAndAccount97,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Intrmy", skip_serializing_if = "Option::is_none") )]
	pub intrmy: Option<PartyIdentificationAndAccount97>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Intrmy2", skip_serializing_if = "Option::is_none") )]
	pub intrmy2: Option<PartyIdentificationAndAccount97>,
}

impl CashParties24 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cdtr.validate()?;
		self.cdtr_agt.validate()?;
		if let Some(ref val) = self.intrmy { val.validate()? }
		if let Some(ref val) = self.intrmy2 { val.validate()? }
		Ok(())
	}
}


// Charge15 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// ChargeType8Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// ChargeType9Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// ClassificationType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ClassificationType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClssfctnFinInstrm", skip_serializing_if = "Option::is_none") )]
	pub clssfctn_fin_instrm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AltrnClssfctn", skip_serializing_if = "Option::is_none") )]
	pub altrn_clssfctn: Option<GenericIdentification1>,
}

impl ClassificationType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.clssfctn_fin_instrm {
			let pattern = Regex::new("[A-Z]{6,6}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "clssfctn_fin_instrm does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.altrn_clssfctn { val.validate()? }
		Ok(())
	}
}


// ClassificationType2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ClassificationType2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClssfctnFinInstrm", skip_serializing_if = "Option::is_none") )]
	pub clssfctn_fin_instrm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmPdctTpCd", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_pdct_tp_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AltrnClssfctn", skip_serializing_if = "Option::is_none") )]
	pub altrn_clssfctn: Option<Vec<GenericIdentification36>>,
}

impl ClassificationType2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.clssfctn_fin_instrm {
			let pattern = Regex::new("[A-Z]{6,6}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "clssfctn_fin_instrm does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.fin_instrm_pdct_tp_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "fin_instrm_pdct_tp_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "fin_instrm_pdct_tp_cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref vec) = self.altrn_clssfctn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// ClearingSystemIdentification2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// CodeOrProprietary1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CodeOrProprietary1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification13>,
}

impl CodeOrProprietary1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// CommonFinancialInstrumentAttributes10 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CommonFinancialInstrumentAttributes10 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctySts", skip_serializing_if = "Option::is_none") )]
	pub scty_sts: Option<SecurityStatus3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISOSctyLngNm", skip_serializing_if = "Option::is_none") )]
	pub iso_scty_lng_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISOSctyShrtNm", skip_serializing_if = "Option::is_none") )]
	pub iso_scty_shrt_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmVldFr", skip_serializing_if = "Option::is_none") )]
	pub nm_vld_fr: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DnmtnCcy") )]
	pub dnmtn_ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CertNb", skip_serializing_if = "Option::is_none") )]
	pub cert_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctVrsnNb", skip_serializing_if = "Option::is_none") )]
	pub ctrct_vrsn_nb: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CpnAttchdNb", skip_serializing_if = "Option::is_none") )]
	pub cpn_attchd_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxLotNb", skip_serializing_if = "Option::is_none") )]
	pub tax_lot_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PoolNb", skip_serializing_if = "Option::is_none") )]
	pub pool_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CvrdInd", skip_serializing_if = "Option::is_none") )]
	pub cvrd_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglRstrctns", skip_serializing_if = "Option::is_none") )]
	pub lgl_rstrctns: Option<LegalRestrictions4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PosLmt", skip_serializing_if = "Option::is_none") )]
	pub pos_lmt: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NearTermPosLmt", skip_serializing_if = "Option::is_none") )]
	pub near_term_pos_lmt: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ListgDt", skip_serializing_if = "Option::is_none") )]
	pub listg_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RcrdDt", skip_serializing_if = "Option::is_none") )]
	pub rcrd_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XpryDt", skip_serializing_if = "Option::is_none") )]
	pub xpry_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
	pub purp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none") )]
	pub clssfctn_tp: Option<ClassificationType2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issnc", skip_serializing_if = "Option::is_none") )]
	pub issnc: Option<Issuance5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgMkt", skip_serializing_if = "Option::is_none") )]
	pub tradg_mkt: Option<Vec<TradingParameters2>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SprdAndBchmkCrv", skip_serializing_if = "Option::is_none") )]
	pub sprd_and_bchmk_crv: Option<Vec<BenchmarkCurve6>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PutTp", skip_serializing_if = "Option::is_none") )]
	pub put_tp: Option<PutType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CallTp", skip_serializing_if = "Option::is_none") )]
	pub call_tp: Option<CallType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FngbInd", skip_serializing_if = "Option::is_none") )]
	pub fngb_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cnfdtl", skip_serializing_if = "Option::is_none") )]
	pub cnfdtl: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvtPlcmnt", skip_serializing_if = "Option::is_none") )]
	pub prvt_plcmnt: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ConvtblInd", skip_serializing_if = "Option::is_none") )]
	pub convtbl_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ConvsPrd", skip_serializing_if = "Option::is_none") )]
	pub convs_prd: Option<DateTimePeriod1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ConvsRatioNmrtr", skip_serializing_if = "Option::is_none") )]
	pub convs_ratio_nmrtr: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ConvsRatioDnmtr", skip_serializing_if = "Option::is_none") )]
	pub convs_ratio_dnmtr: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmryPlcOfDpst", skip_serializing_if = "Option::is_none") )]
	pub pmry_plc_of_dpst: Option<PartyIdentification136>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgMtd", skip_serializing_if = "Option::is_none") )]
	pub tradg_mtd: Option<UnitOrFaceAmount1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TEFRARule", skip_serializing_if = "Option::is_none") )]
	pub tefra_rule: Option<TEFRARules3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SrNb", skip_serializing_if = "Option::is_none") )]
	pub sr_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Clss", skip_serializing_if = "Option::is_none") )]
	pub clss: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WhldgTaxRgm", skip_serializing_if = "Option::is_none") )]
	pub whldg_tax_rgm: Option<Vec<SecurityWithHoldingTax1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtSts", skip_serializing_if = "Option::is_none") )]
	pub pmt_sts: Option<SecuritiesPaymentStatus5Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitlPhysForm", skip_serializing_if = "Option::is_none") )]
	pub initl_phys_form: Option<InitialPhysicalForm4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AftrXchgPhysForm", skip_serializing_if = "Option::is_none") )]
	pub aftr_xchg_phys_form: Option<InitialPhysicalForm3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CmonSfkpr", skip_serializing_if = "Option::is_none") )]
	pub cmon_sfkpr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RedTp", skip_serializing_if = "Option::is_none") )]
	pub red_tp: Option<MaturityRedemptionType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RedPmtCcy", skip_serializing_if = "Option::is_none") )]
	pub red_pmt_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rstrctn", skip_serializing_if = "Option::is_none") )]
	pub rstrctn: Option<Vec<SecurityRestriction3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmInf", skip_serializing_if = "Option::is_none") )]
	pub sttlm_inf: Option<Vec<SettlementInformation17>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmForm", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_form: Option<FinancialInstrumentForm2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtctNm", skip_serializing_if = "Option::is_none") )]
	pub ctct_nm: Option<Organisation38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LeadMgr", skip_serializing_if = "Option::is_none") )]
	pub lead_mgr: Option<Organisation38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrncplPngAgt", skip_serializing_if = "Option::is_none") )]
	pub prncpl_png_agt: Option<Organisation38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PngAgt", skip_serializing_if = "Option::is_none") )]
	pub png_agt: Option<Organisation38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dpstry", skip_serializing_if = "Option::is_none") )]
	pub dpstry: Option<Organisation38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygRsk", skip_serializing_if = "Option::is_none") )]
	pub undrlyg_rsk: Option<Organisation38>,
}

impl CommonFinancialInstrumentAttributes10 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.scty_sts { val.validate()? }
		if let Some(ref val) = self.iso_scty_lng_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "iso_scty_lng_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "iso_scty_lng_nm exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.iso_scty_shrt_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "iso_scty_shrt_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "iso_scty_shrt_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.nm_vld_fr { val.validate()? }
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.dnmtn_ccy) {
			return Err(ValidationError::new(1005, "dnmtn_ccy does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.cert_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cert_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "cert_nb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.cpn_attchd_nb {
			let pattern = Regex::new("[0-9]{1,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "cpn_attchd_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.tax_lot_nb {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "tax_lot_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pool_nb {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "pool_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.lgl_rstrctns { val.validate()? }
		if let Some(ref val) = self.pos_lmt { val.validate()? }
		if let Some(ref val) = self.near_term_pos_lmt { val.validate()? }
		if let Some(ref val) = self.purp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "purp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "purp exceeds the maximum length of 256".to_string()));
			}
		}
		if let Some(ref val) = self.clssfctn_tp { val.validate()? }
		if let Some(ref val) = self.issnc { val.validate()? }
		if let Some(ref vec) = self.tradg_mkt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.sprd_and_bchmk_crv { for item in vec { item.validate()? } }
		if let Some(ref val) = self.put_tp { val.validate()? }
		if let Some(ref val) = self.call_tp { val.validate()? }
		if let Some(ref val) = self.convs_prd { val.validate()? }
		if let Some(ref val) = self.convs_ratio_nmrtr { val.validate()? }
		if let Some(ref val) = self.convs_ratio_dnmtr { val.validate()? }
		if let Some(ref val) = self.pmry_plc_of_dpst { val.validate()? }
		if let Some(ref val) = self.tradg_mtd { val.validate()? }
		if let Some(ref val) = self.tefra_rule { val.validate()? }
		if let Some(ref val) = self.sr_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sr_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "sr_nb exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.clss {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "clss is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "clss exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref vec) = self.whldg_tax_rgm { for item in vec { item.validate()? } }
		if let Some(ref val) = self.pmt_sts { val.validate()? }
		if let Some(ref val) = self.initl_phys_form { val.validate()? }
		if let Some(ref val) = self.aftr_xchg_phys_form { val.validate()? }
		if let Some(ref val) = self.cmon_sfkpr {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "cmon_sfkpr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.red_tp { val.validate()? }
		if let Some(ref val) = self.red_pmt_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "red_pmt_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.rstrctn { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.sttlm_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.fin_instrm_form { val.validate()? }
		if let Some(ref val) = self.ctct_nm { val.validate()? }
		if let Some(ref val) = self.lead_mgr { val.validate()? }
		if let Some(ref val) = self.prncpl_png_agt { val.validate()? }
		if let Some(ref val) = self.png_agt { val.validate()? }
		if let Some(ref val) = self.dpstry { val.validate()? }
		if let Some(ref val) = self.undrlyg_rsk { val.validate()? }
		Ok(())
	}
}


// CommonFinancialInstrumentAttributes11 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CommonFinancialInstrumentAttributes11 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctySts", skip_serializing_if = "Option::is_none") )]
	pub scty_sts: Option<SecurityStatus3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmNm", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_nm: Option<Vec<FinancialInstrumentName2>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DnmtnCcy", skip_serializing_if = "Option::is_none") )]
	pub dnmtn_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CertNb", skip_serializing_if = "Option::is_none") )]
	pub cert_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctVrsnNb", skip_serializing_if = "Option::is_none") )]
	pub ctrct_vrsn_nb: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CpnAttchdNb", skip_serializing_if = "Option::is_none") )]
	pub cpn_attchd_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxLotNb", skip_serializing_if = "Option::is_none") )]
	pub tax_lot_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PoolNb", skip_serializing_if = "Option::is_none") )]
	pub pool_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CvrdInd", skip_serializing_if = "Option::is_none") )]
	pub cvrd_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglRstrctns", skip_serializing_if = "Option::is_none") )]
	pub lgl_rstrctns: Option<LegalRestrictions4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PosLmt", skip_serializing_if = "Option::is_none") )]
	pub pos_lmt: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NearTermPosLmt", skip_serializing_if = "Option::is_none") )]
	pub near_term_pos_lmt: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ListgDt", skip_serializing_if = "Option::is_none") )]
	pub listg_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RcrdDt", skip_serializing_if = "Option::is_none") )]
	pub rcrd_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XpryDt", skip_serializing_if = "Option::is_none") )]
	pub xpry_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
	pub purp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none") )]
	pub clssfctn_tp: Option<ClassificationType2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issnc", skip_serializing_if = "Option::is_none") )]
	pub issnc: Option<Issuance6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgMkt", skip_serializing_if = "Option::is_none") )]
	pub tradg_mkt: Option<Vec<TradingParameters2>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SprdAndBchmkCrv", skip_serializing_if = "Option::is_none") )]
	pub sprd_and_bchmk_crv: Option<Vec<BenchmarkCurve6>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PutTp", skip_serializing_if = "Option::is_none") )]
	pub put_tp: Option<PutType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CallTp", skip_serializing_if = "Option::is_none") )]
	pub call_tp: Option<CallType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FngbInd", skip_serializing_if = "Option::is_none") )]
	pub fngb_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cnfdtl", skip_serializing_if = "Option::is_none") )]
	pub cnfdtl: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvtPlcmnt", skip_serializing_if = "Option::is_none") )]
	pub prvt_plcmnt: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ConvtblInd", skip_serializing_if = "Option::is_none") )]
	pub convtbl_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ConvsPrd", skip_serializing_if = "Option::is_none") )]
	pub convs_prd: Option<DateTimePeriod1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ConvsRatioNmrtr", skip_serializing_if = "Option::is_none") )]
	pub convs_ratio_nmrtr: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ConvsRatioDnmtr", skip_serializing_if = "Option::is_none") )]
	pub convs_ratio_dnmtr: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmryPlcOfDpst", skip_serializing_if = "Option::is_none") )]
	pub pmry_plc_of_dpst: Option<PartyIdentification136>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgMtd", skip_serializing_if = "Option::is_none") )]
	pub tradg_mtd: Option<UnitOrFaceAmount1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TEFRARule", skip_serializing_if = "Option::is_none") )]
	pub tefra_rule: Option<TEFRARules3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SrNb", skip_serializing_if = "Option::is_none") )]
	pub sr_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Clss", skip_serializing_if = "Option::is_none") )]
	pub clss: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WhldgTaxRgm", skip_serializing_if = "Option::is_none") )]
	pub whldg_tax_rgm: Option<Vec<SecurityWithHoldingTax1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtSts", skip_serializing_if = "Option::is_none") )]
	pub pmt_sts: Option<SecuritiesPaymentStatus5Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitlPhysForm", skip_serializing_if = "Option::is_none") )]
	pub initl_phys_form: Option<InitialPhysicalForm4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AftrXchgPhysForm", skip_serializing_if = "Option::is_none") )]
	pub aftr_xchg_phys_form: Option<InitialPhysicalForm3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CmonSfkpr", skip_serializing_if = "Option::is_none") )]
	pub cmon_sfkpr: Option<PartyIdentification177Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RedTp", skip_serializing_if = "Option::is_none") )]
	pub red_tp: Option<MaturityRedemptionType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RedPmtCcy", skip_serializing_if = "Option::is_none") )]
	pub red_pmt_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rstrctn", skip_serializing_if = "Option::is_none") )]
	pub rstrctn: Option<Vec<SecurityRestriction3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmIdVldty", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_id_vldty: Option<Vec<FinancialInstrumentIdentificationValidity3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmInf", skip_serializing_if = "Option::is_none") )]
	pub sttlm_inf: Option<Vec<SettlementInformation17>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmForm", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_form: Option<FinancialInstrumentForm2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtctNm", skip_serializing_if = "Option::is_none") )]
	pub ctct_nm: Option<Organisation38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LeadMgr", skip_serializing_if = "Option::is_none") )]
	pub lead_mgr: Option<Organisation38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrncplPngAgt", skip_serializing_if = "Option::is_none") )]
	pub prncpl_png_agt: Option<Organisation38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PngAgt", skip_serializing_if = "Option::is_none") )]
	pub png_agt: Option<Organisation38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dpstry", skip_serializing_if = "Option::is_none") )]
	pub dpstry: Option<Organisation38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygRsk", skip_serializing_if = "Option::is_none") )]
	pub undrlyg_rsk: Option<Organisation38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctyCSDLk", skip_serializing_if = "Option::is_none") )]
	pub scty_csd_lk: Option<Vec<SecurityCSDLink7>>,
}

impl CommonFinancialInstrumentAttributes11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.scty_sts { val.validate()? }
		if let Some(ref vec) = self.fin_instrm_nm { for item in vec { item.validate()? } }
		if let Some(ref val) = self.dnmtn_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "dnmtn_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.cert_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cert_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "cert_nb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.cpn_attchd_nb {
			let pattern = Regex::new("[0-9]{1,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "cpn_attchd_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.tax_lot_nb {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "tax_lot_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pool_nb {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "pool_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.lgl_rstrctns { val.validate()? }
		if let Some(ref val) = self.pos_lmt { val.validate()? }
		if let Some(ref val) = self.near_term_pos_lmt { val.validate()? }
		if let Some(ref val) = self.purp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "purp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "purp exceeds the maximum length of 256".to_string()));
			}
		}
		if let Some(ref val) = self.clssfctn_tp { val.validate()? }
		if let Some(ref val) = self.issnc { val.validate()? }
		if let Some(ref vec) = self.tradg_mkt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.sprd_and_bchmk_crv { for item in vec { item.validate()? } }
		if let Some(ref val) = self.put_tp { val.validate()? }
		if let Some(ref val) = self.call_tp { val.validate()? }
		if let Some(ref val) = self.convs_prd { val.validate()? }
		if let Some(ref val) = self.convs_ratio_nmrtr { val.validate()? }
		if let Some(ref val) = self.convs_ratio_dnmtr { val.validate()? }
		if let Some(ref val) = self.pmry_plc_of_dpst { val.validate()? }
		if let Some(ref val) = self.tradg_mtd { val.validate()? }
		if let Some(ref val) = self.tefra_rule { val.validate()? }
		if let Some(ref val) = self.sr_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sr_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "sr_nb exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.clss {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "clss is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "clss exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref vec) = self.whldg_tax_rgm { for item in vec { item.validate()? } }
		if let Some(ref val) = self.pmt_sts { val.validate()? }
		if let Some(ref val) = self.initl_phys_form { val.validate()? }
		if let Some(ref val) = self.aftr_xchg_phys_form { val.validate()? }
		if let Some(ref val) = self.cmon_sfkpr { val.validate()? }
		if let Some(ref val) = self.red_tp { val.validate()? }
		if let Some(ref val) = self.red_pmt_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "red_pmt_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.rstrctn { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.fin_instrm_id_vldty { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.sttlm_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.fin_instrm_form { val.validate()? }
		if let Some(ref val) = self.ctct_nm { val.validate()? }
		if let Some(ref val) = self.lead_mgr { val.validate()? }
		if let Some(ref val) = self.prncpl_png_agt { val.validate()? }
		if let Some(ref val) = self.png_agt { val.validate()? }
		if let Some(ref val) = self.dpstry { val.validate()? }
		if let Some(ref val) = self.undrlyg_rsk { val.validate()? }
		if let Some(ref vec) = self.scty_csd_lk { for item in vec { item.validate()? } }
		Ok(())
	}
}


// CommonFinancialInstrumentAttributes12 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CommonFinancialInstrumentAttributes12 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctySts", skip_serializing_if = "Option::is_none") )]
	pub scty_sts: Option<SecurityStatus3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISOSctyLngNm", skip_serializing_if = "Option::is_none") )]
	pub iso_scty_lng_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISOSctyShrtNm", skip_serializing_if = "Option::is_none") )]
	pub iso_scty_shrt_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmVldFr", skip_serializing_if = "Option::is_none") )]
	pub nm_vld_fr: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DnmtnCcy", skip_serializing_if = "Option::is_none") )]
	pub dnmtn_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CertNb", skip_serializing_if = "Option::is_none") )]
	pub cert_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctVrsnNb", skip_serializing_if = "Option::is_none") )]
	pub ctrct_vrsn_nb: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CpnAttchdNb", skip_serializing_if = "Option::is_none") )]
	pub cpn_attchd_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxLotNb", skip_serializing_if = "Option::is_none") )]
	pub tax_lot_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PoolNb", skip_serializing_if = "Option::is_none") )]
	pub pool_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CvrdInd", skip_serializing_if = "Option::is_none") )]
	pub cvrd_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglRstrctns", skip_serializing_if = "Option::is_none") )]
	pub lgl_rstrctns: Option<LegalRestrictions4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PosLmt", skip_serializing_if = "Option::is_none") )]
	pub pos_lmt: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NearTermPosLmt", skip_serializing_if = "Option::is_none") )]
	pub near_term_pos_lmt: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ListgDt", skip_serializing_if = "Option::is_none") )]
	pub listg_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RcrdDt", skip_serializing_if = "Option::is_none") )]
	pub rcrd_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XpryDt", skip_serializing_if = "Option::is_none") )]
	pub xpry_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
	pub purp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none") )]
	pub clssfctn_tp: Option<ClassificationType2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issnc", skip_serializing_if = "Option::is_none") )]
	pub issnc: Option<Issuance5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgMkt", skip_serializing_if = "Option::is_none") )]
	pub tradg_mkt: Option<Vec<TradingParameters2>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SprdAndBchmkCrv", skip_serializing_if = "Option::is_none") )]
	pub sprd_and_bchmk_crv: Option<Vec<BenchmarkCurve6>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PutTp", skip_serializing_if = "Option::is_none") )]
	pub put_tp: Option<PutType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CallTp", skip_serializing_if = "Option::is_none") )]
	pub call_tp: Option<CallType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FngbInd", skip_serializing_if = "Option::is_none") )]
	pub fngb_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cnfdtl", skip_serializing_if = "Option::is_none") )]
	pub cnfdtl: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvtPlcmnt", skip_serializing_if = "Option::is_none") )]
	pub prvt_plcmnt: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ConvtblInd", skip_serializing_if = "Option::is_none") )]
	pub convtbl_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ConvsPrd", skip_serializing_if = "Option::is_none") )]
	pub convs_prd: Option<DateTimePeriod1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ConvsRatioNmrtr", skip_serializing_if = "Option::is_none") )]
	pub convs_ratio_nmrtr: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ConvsRatioDnmtr", skip_serializing_if = "Option::is_none") )]
	pub convs_ratio_dnmtr: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmryPlcOfDpst", skip_serializing_if = "Option::is_none") )]
	pub pmry_plc_of_dpst: Option<PartyIdentification136>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgMtd", skip_serializing_if = "Option::is_none") )]
	pub tradg_mtd: Option<UnitOrFaceAmount1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TEFRARule", skip_serializing_if = "Option::is_none") )]
	pub tefra_rule: Option<TEFRARules3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SrNb", skip_serializing_if = "Option::is_none") )]
	pub sr_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Clss", skip_serializing_if = "Option::is_none") )]
	pub clss: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WhldgTaxRgm", skip_serializing_if = "Option::is_none") )]
	pub whldg_tax_rgm: Option<Vec<SecurityWithHoldingTax1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtSts", skip_serializing_if = "Option::is_none") )]
	pub pmt_sts: Option<SecuritiesPaymentStatus5Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitlPhysForm", skip_serializing_if = "Option::is_none") )]
	pub initl_phys_form: Option<InitialPhysicalForm4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AftrXchgPhysForm", skip_serializing_if = "Option::is_none") )]
	pub aftr_xchg_phys_form: Option<InitialPhysicalForm3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CmonSfkpr", skip_serializing_if = "Option::is_none") )]
	pub cmon_sfkpr: Option<PartyIdentification177Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RedTp", skip_serializing_if = "Option::is_none") )]
	pub red_tp: Option<MaturityRedemptionType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RedPmtCcy", skip_serializing_if = "Option::is_none") )]
	pub red_pmt_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rstrctn", skip_serializing_if = "Option::is_none") )]
	pub rstrctn: Option<Vec<SecurityRestriction3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_id: Option<SecurityIdentification39>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmInf", skip_serializing_if = "Option::is_none") )]
	pub sttlm_inf: Option<Vec<SettlementInformation17>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmForm", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_form: Option<FinancialInstrumentForm2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtctNm", skip_serializing_if = "Option::is_none") )]
	pub ctct_nm: Option<Organisation38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LeadMgr", skip_serializing_if = "Option::is_none") )]
	pub lead_mgr: Option<Organisation38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrncplPngAgt", skip_serializing_if = "Option::is_none") )]
	pub prncpl_png_agt: Option<Organisation38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PngAgt", skip_serializing_if = "Option::is_none") )]
	pub png_agt: Option<Organisation38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dpstry", skip_serializing_if = "Option::is_none") )]
	pub dpstry: Option<Organisation38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygRsk", skip_serializing_if = "Option::is_none") )]
	pub undrlyg_rsk: Option<Organisation38>,
}

impl CommonFinancialInstrumentAttributes12 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.scty_sts { val.validate()? }
		if let Some(ref val) = self.iso_scty_lng_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "iso_scty_lng_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "iso_scty_lng_nm exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.iso_scty_shrt_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "iso_scty_shrt_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "iso_scty_shrt_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.nm_vld_fr { val.validate()? }
		if let Some(ref val) = self.dnmtn_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "dnmtn_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.cert_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cert_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "cert_nb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.cpn_attchd_nb {
			let pattern = Regex::new("[0-9]{1,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "cpn_attchd_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.tax_lot_nb {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "tax_lot_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pool_nb {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "pool_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.lgl_rstrctns { val.validate()? }
		if let Some(ref val) = self.pos_lmt { val.validate()? }
		if let Some(ref val) = self.near_term_pos_lmt { val.validate()? }
		if let Some(ref val) = self.purp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "purp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "purp exceeds the maximum length of 256".to_string()));
			}
		}
		if let Some(ref val) = self.clssfctn_tp { val.validate()? }
		if let Some(ref val) = self.issnc { val.validate()? }
		if let Some(ref vec) = self.tradg_mkt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.sprd_and_bchmk_crv { for item in vec { item.validate()? } }
		if let Some(ref val) = self.put_tp { val.validate()? }
		if let Some(ref val) = self.call_tp { val.validate()? }
		if let Some(ref val) = self.convs_prd { val.validate()? }
		if let Some(ref val) = self.convs_ratio_nmrtr { val.validate()? }
		if let Some(ref val) = self.convs_ratio_dnmtr { val.validate()? }
		if let Some(ref val) = self.pmry_plc_of_dpst { val.validate()? }
		if let Some(ref val) = self.tradg_mtd { val.validate()? }
		if let Some(ref val) = self.tefra_rule { val.validate()? }
		if let Some(ref val) = self.sr_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sr_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "sr_nb exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.clss {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "clss is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "clss exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref vec) = self.whldg_tax_rgm { for item in vec { item.validate()? } }
		if let Some(ref val) = self.pmt_sts { val.validate()? }
		if let Some(ref val) = self.initl_phys_form { val.validate()? }
		if let Some(ref val) = self.aftr_xchg_phys_form { val.validate()? }
		if let Some(ref val) = self.cmon_sfkpr { val.validate()? }
		if let Some(ref val) = self.red_tp { val.validate()? }
		if let Some(ref val) = self.red_pmt_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "red_pmt_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.rstrctn { for item in vec { item.validate()? } }
		if let Some(ref val) = self.fin_instrm_id { val.validate()? }
		if let Some(ref vec) = self.sttlm_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.fin_instrm_form { val.validate()? }
		if let Some(ref val) = self.ctct_nm { val.validate()? }
		if let Some(ref val) = self.lead_mgr { val.validate()? }
		if let Some(ref val) = self.prncpl_png_agt { val.validate()? }
		if let Some(ref val) = self.png_agt { val.validate()? }
		if let Some(ref val) = self.dpstry { val.validate()? }
		if let Some(ref val) = self.undrlyg_rsk { val.validate()? }
		Ok(())
	}
}


// CommunicationAddress3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CommunicationAddress3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Email", skip_serializing_if = "Option::is_none") )]
	pub email: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Phne", skip_serializing_if = "Option::is_none") )]
	pub phne: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mob", skip_serializing_if = "Option::is_none") )]
	pub mob: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FaxNb", skip_serializing_if = "Option::is_none") )]
	pub fax_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TlxAdr", skip_serializing_if = "Option::is_none") )]
	pub tlx_adr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "URLAdr", skip_serializing_if = "Option::is_none") )]
	pub url_adr: Option<String>,
}

impl CommunicationAddress3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.email {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "email is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "email exceeds the maximum length of 256".to_string()));
			}
		}
		if let Some(ref val) = self.phne {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "phne does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.mob {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "mob does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.fax_nb {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "fax_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.tlx_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tlx_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tlx_adr exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.url_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "url_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "url_adr exceeds the maximum length of 256".to_string()));
			}
		}
		Ok(())
	}
}


// Contact13 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Contact13 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none") )]
	pub nm_prfx: Option<NamePrefix2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PhneNb", skip_serializing_if = "Option::is_none") )]
	pub phne_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MobNb", skip_serializing_if = "Option::is_none") )]
	pub mob_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FaxNb", skip_serializing_if = "Option::is_none") )]
	pub fax_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "URLAdr", skip_serializing_if = "Option::is_none") )]
	pub url_adr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none") )]
	pub email_adr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EmailPurp", skip_serializing_if = "Option::is_none") )]
	pub email_purp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "JobTitl", skip_serializing_if = "Option::is_none") )]
	pub job_titl: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rspnsblty", skip_serializing_if = "Option::is_none") )]
	pub rspnsblty: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dept", skip_serializing_if = "Option::is_none") )]
	pub dept: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<OtherContact1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrefrdMtd", skip_serializing_if = "Option::is_none") )]
	pub prefrd_mtd: Option<PreferredContactMethod2Code>,
}

impl Contact13 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm_prfx { val.validate()? }
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.phne_nb {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "phne_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.mob_nb {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "mob_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.fax_nb {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "fax_nb does not match the required pattern".to_string()));
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
		if let Some(ref val) = self.email_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "email_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "email_adr exceeds the maximum length of 256".to_string()));
			}
		}
		if let Some(ref val) = self.email_purp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "email_purp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "email_purp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.job_titl {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "job_titl is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "job_titl exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.rspnsblty {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rspnsblty is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "rspnsblty exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.dept {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dept is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "dept exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref vec) = self.othr { for item in vec { item.validate()? } }
		if let Some(ref val) = self.prefrd_mtd { val.validate()? }
		Ok(())
	}
}


// Contact14 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Contact14 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none") )]
	pub nm_prfx: Option<NamePrefix2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PhneNb", skip_serializing_if = "Option::is_none") )]
	pub phne_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MobNb", skip_serializing_if = "Option::is_none") )]
	pub mob_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FaxNb", skip_serializing_if = "Option::is_none") )]
	pub fax_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "URLAdr", skip_serializing_if = "Option::is_none") )]
	pub url_adr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none") )]
	pub email_adr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EmailPurp", skip_serializing_if = "Option::is_none") )]
	pub email_purp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "JobTitl", skip_serializing_if = "Option::is_none") )]
	pub job_titl: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rspnsblty", skip_serializing_if = "Option::is_none") )]
	pub rspnsblty: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dept", skip_serializing_if = "Option::is_none") )]
	pub dept: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<OtherContact1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrefrdMtd", skip_serializing_if = "Option::is_none") )]
	pub prefrd_mtd: Option<PreferredContactMethod2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldFr", skip_serializing_if = "Option::is_none") )]
	pub vld_fr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldTo", skip_serializing_if = "Option::is_none") )]
	pub vld_to: Option<String>,
}

impl Contact14 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm_prfx { val.validate()? }
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.phne_nb {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "phne_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.mob_nb {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "mob_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.fax_nb {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "fax_nb does not match the required pattern".to_string()));
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
		if let Some(ref val) = self.email_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "email_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "email_adr exceeds the maximum length of 256".to_string()));
			}
		}
		if let Some(ref val) = self.email_purp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "email_purp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "email_purp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.job_titl {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "job_titl is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "job_titl exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.rspnsblty {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rspnsblty is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "rspnsblty exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.dept {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dept is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "dept exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref vec) = self.othr { for item in vec { item.validate()? } }
		if let Some(ref val) = self.prefrd_mtd { val.validate()? }
		Ok(())
	}
}


// ContactAttributes5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// ContractReference1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ContractReference1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<DocumentType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ref") )]
	pub ref_attr: String,
}

impl ContractReference1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if self.ref_attr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "ref_attr is shorter than the minimum length of 1".to_string()));
		}
		if self.ref_attr.chars().count() > 500 {
			return Err(ValidationError::new(1002, "ref_attr exceeds the maximum length of 500".to_string()));
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// CreditorEnrolment5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CreditorEnrolment5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Enrlmnt") )]
	pub enrlmnt: CreditorServiceEnrolment1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrTradgNm", skip_serializing_if = "Option::is_none") )]
	pub cdtr_tradg_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr") )]
	pub cdtr: RTPPartyIdentification2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<RTPPartyIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MrchntCtgyCd") )]
	pub mrchnt_ctgy_cd: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrLogo", skip_serializing_if = "Option::is_none") )]
	pub cdtr_logo: Option<String>,
}

impl CreditorEnrolment5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.enrlmnt.validate()?;
		if let Some(ref val) = self.cdtr_tradg_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cdtr_tradg_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "cdtr_tradg_nm exceeds the maximum length of 140".to_string()));
			}
		}
		self.cdtr.validate()?;
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		let pattern = Regex::new("[0-9]{4,4}").unwrap();
		if !pattern.is_match(&self.mrchnt_ctgy_cd) {
			return Err(ValidationError::new(1005, "mrchnt_ctgy_cd does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.cdtr_logo {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cdtr_logo is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 10240 {
				return Err(ValidationError::new(1002, "cdtr_logo exceeds the maximum length of 10240".to_string()));
			}
		}
		Ok(())
	}
}


// CreditorEnrolment6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CreditorEnrolment6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Enrlmnt", skip_serializing_if = "Option::is_none") )]
	pub enrlmnt: Option<CreditorServiceEnrolment1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrTradgNm", skip_serializing_if = "Option::is_none") )]
	pub cdtr_tradg_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr") )]
	pub cdtr: RTPPartyIdentification2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<RTPPartyIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MrchntCtgyCd", skip_serializing_if = "Option::is_none") )]
	pub mrchnt_ctgy_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrLogo", skip_serializing_if = "Option::is_none") )]
	pub cdtr_logo: Option<String>,
}

impl CreditorEnrolment6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.enrlmnt { val.validate()? }
		if let Some(ref val) = self.cdtr_tradg_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cdtr_tradg_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "cdtr_tradg_nm exceeds the maximum length of 140".to_string()));
			}
		}
		self.cdtr.validate()?;
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		if let Some(ref val) = self.mrchnt_ctgy_cd {
			let pattern = Regex::new("[0-9]{4,4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "mrchnt_ctgy_cd does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.cdtr_logo {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cdtr_logo is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 10240 {
				return Err(ValidationError::new(1002, "cdtr_logo exceeds the maximum length of 10240".to_string()));
			}
		}
		Ok(())
	}
}


// CreditorEnrolmentAmendment5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CreditorEnrolmentAmendment5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlBizInstr", skip_serializing_if = "Option::is_none") )]
	pub orgnl_biz_instr: Option<OriginalBusinessInstruction1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmdmntRsn", skip_serializing_if = "Option::is_none") )]
	pub amdmnt_rsn: Option<CreditorEnrolmentAmendmentReason3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amdmnt") )]
	pub amdmnt: CreditorEnrolmentAmendment6,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlEnrlmnt") )]
	pub orgnl_enrlmnt: OriginalEnrolment3Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CreditorEnrolmentAmendment5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_biz_instr { val.validate()? }
		if let Some(ref val) = self.amdmnt_rsn { val.validate()? }
		self.amdmnt.validate()?;
		self.orgnl_enrlmnt.validate()?;
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// CreditorEnrolmentAmendment6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CreditorEnrolmentAmendment6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrEnrlmnt", skip_serializing_if = "Option::is_none") )]
	pub cdtr_enrlmnt: Option<CreditorEnrolment6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ActvtnData", skip_serializing_if = "Option::is_none") )]
	pub actvtn_data: Option<CreditorInvoice5>,
}

impl CreditorEnrolmentAmendment6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cdtr_enrlmnt { val.validate()? }
		if let Some(ref val) = self.actvtn_data { val.validate()? }
		Ok(())
	}
}


// CreditorEnrolmentAmendmentReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CreditorEnrolmentAmendmentReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl CreditorEnrolmentAmendmentReason1Choice {
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


// CreditorEnrolmentAmendmentReason3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CreditorEnrolmentAmendmentReason3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Orgtr", skip_serializing_if = "Option::is_none") )]
	pub orgtr: Option<RTPPartyIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn") )]
	pub rsn: CreditorEnrolmentAmendmentReason1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<String>>,
}

impl CreditorEnrolmentAmendmentReason3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgtr { val.validate()? }
		self.rsn.validate()?;
		if let Some(ref vec) = self.addtl_inf {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 105 {
					return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 105".to_string()));
				}
			}
		}
		Ok(())
	}
}


// CreditorEnrolmentCancellation3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CreditorEnrolmentCancellation3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlBizInstr", skip_serializing_if = "Option::is_none") )]
	pub orgnl_biz_instr: Option<OriginalBusinessInstruction1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CxlRsn", skip_serializing_if = "Option::is_none") )]
	pub cxl_rsn: Option<CreditorEnrolmentCancellationReason3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlEnrlmnt") )]
	pub orgnl_enrlmnt: OriginalEnrolment3Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CreditorEnrolmentCancellation3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_biz_instr { val.validate()? }
		if let Some(ref val) = self.cxl_rsn { val.validate()? }
		self.orgnl_enrlmnt.validate()?;
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// CreditorEnrolmentCancellationReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CreditorEnrolmentCancellationReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl CreditorEnrolmentCancellationReason1Choice {
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


// CreditorEnrolmentCancellationReason3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CreditorEnrolmentCancellationReason3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Orgtr", skip_serializing_if = "Option::is_none") )]
	pub orgtr: Option<RTPPartyIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn") )]
	pub rsn: CreditorEnrolmentCancellationReason1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<String>>,
}

impl CreditorEnrolmentCancellationReason3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgtr { val.validate()? }
		self.rsn.validate()?;
		if let Some(ref vec) = self.addtl_inf {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 105 {
					return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 105".to_string()));
				}
			}
		}
		Ok(())
	}
}


// CreditorEnrolmentStatusReason2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CreditorEnrolmentStatusReason2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl CreditorEnrolmentStatusReason2Choice {
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


// CreditorEnrolmentStatusReason3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CreditorEnrolmentStatusReason3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Orgtr", skip_serializing_if = "Option::is_none") )]
	pub orgtr: Option<RTPPartyIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn") )]
	pub rsn: CreditorEnrolmentStatusReason2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<String>>,
}

impl CreditorEnrolmentStatusReason3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgtr { val.validate()? }
		self.rsn.validate()?;
		if let Some(ref vec) = self.addtl_inf {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 105 {
					return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 105".to_string()));
				}
			}
		}
		Ok(())
	}
}


// CreditorInvoice5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CreditorInvoice5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "LtdPresntmntInd", skip_serializing_if = "Option::is_none") )]
	pub ltd_presntmnt_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CstmrIdTp", skip_serializing_if = "Option::is_none") )]
	pub cstmr_id_tp: Option<CustomerTypeRequest2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctFrmtTp", skip_serializing_if = "Option::is_none") )]
	pub ctrct_frmt_tp: Option<Vec<DocumentFormat2Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctRefTp", skip_serializing_if = "Option::is_none") )]
	pub ctrct_ref_tp: Option<Vec<DocumentType1Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrInstr", skip_serializing_if = "Option::is_none") )]
	pub cdtr_instr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ActvtnReqDlvryPty", skip_serializing_if = "Option::is_none") )]
	pub actvtn_req_dlvry_pty: Option<RTPPartyIdentification2>,
}

impl CreditorInvoice5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cstmr_id_tp { val.validate()? }
		if let Some(ref vec) = self.ctrct_frmt_tp { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.ctrct_ref_tp { for item in vec { item.validate()? } }
		if let Some(ref val) = self.cdtr_instr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cdtr_instr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 500 {
				return Err(ValidationError::new(1002, "cdtr_instr exceeds the maximum length of 500".to_string()));
			}
		}
		if let Some(ref val) = self.actvtn_req_dlvry_pty { val.validate()? }
		Ok(())
	}
}


// CreditorInvoice6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CreditorInvoice6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "LtdPresntmntInd") )]
	pub ltd_presntmnt_ind: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CstmrIdTp", skip_serializing_if = "Option::is_none") )]
	pub cstmr_id_tp: Option<CustomerTypeRequest2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctFrmtTp", skip_serializing_if = "Option::is_none") )]
	pub ctrct_frmt_tp: Option<Vec<DocumentFormat2Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctRefTp", skip_serializing_if = "Option::is_none") )]
	pub ctrct_ref_tp: Option<Vec<DocumentType1Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrInstr", skip_serializing_if = "Option::is_none") )]
	pub cdtr_instr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ActvtnReqDlvryPty") )]
	pub actvtn_req_dlvry_pty: RTPPartyIdentification2,
}

impl CreditorInvoice6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cstmr_id_tp { val.validate()? }
		if let Some(ref vec) = self.ctrct_frmt_tp { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.ctrct_ref_tp { for item in vec { item.validate()? } }
		if let Some(ref val) = self.cdtr_instr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cdtr_instr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 500 {
				return Err(ValidationError::new(1002, "cdtr_instr exceeds the maximum length of 500".to_string()));
			}
		}
		self.actvtn_req_dlvry_pty.validate()?;
		Ok(())
	}
}


// CreditorServiceEnrolment1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CreditorServiceEnrolment1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "EnrlmntStartDt", skip_serializing_if = "Option::is_none") )]
	pub enrlmnt_start_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EnrlmntEndDt", skip_serializing_if = "Option::is_none") )]
	pub enrlmnt_end_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Vsblty", skip_serializing_if = "Option::is_none") )]
	pub vsblty: Option<Visibilty1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SvcActvtnAllwd") )]
	pub svc_actvtn_allwd: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SvcDescLk", skip_serializing_if = "Option::is_none") )]
	pub svc_desc_lk: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrSvcActvtnLk", skip_serializing_if = "Option::is_none") )]
	pub cdtr_svc_actvtn_lk: Option<String>,
}

impl CreditorServiceEnrolment1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.enrlmnt_start_dt { val.validate()? }
		if let Some(ref val) = self.enrlmnt_end_dt { val.validate()? }
		if let Some(ref val) = self.vsblty { val.validate()? }
		if let Some(ref val) = self.svc_desc_lk {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "svc_desc_lk is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 2048 {
				return Err(ValidationError::new(1002, "svc_desc_lk exceeds the maximum length of 2048".to_string()));
			}
		}
		if let Some(ref val) = self.cdtr_svc_actvtn_lk {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cdtr_svc_actvtn_lk is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 2048 {
				return Err(ValidationError::new(1002, "cdtr_svc_actvtn_lk exceeds the maximum length of 2048".to_string()));
			}
		}
		Ok(())
	}
}


// CustomerTypeRequest2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CustomerTypeRequest2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Reqd") )]
	pub reqd: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgTp", skip_serializing_if = "Option::is_none") )]
	pub org_tp: Option<OrganisationType2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvtTp", skip_serializing_if = "Option::is_none") )]
	pub prvt_tp: Option<PersonType2>,
}

impl CustomerTypeRequest2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.org_tp { val.validate()? }
		if let Some(ref val) = self.prvt_tp { val.validate()? }
		Ok(())
	}
}


// CutOff1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CutOff1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CutOffUpdId") )]
	pub cut_off_upd_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CutOffTm") )]
	pub cut_off_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValDtOffset") )]
	pub val_dt_offset: String,
}

impl CutOff1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.cut_off_upd_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "cut_off_upd_id is shorter than the minimum length of 1".to_string()));
		}
		if self.cut_off_upd_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "cut_off_upd_id exceeds the maximum length of 35".to_string()));
		}
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.ccy) {
			return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
		}
		let pattern = Regex::new("0|-1|-2").unwrap();
		if !pattern.is_match(&self.val_dt_offset) {
			return Err(ValidationError::new(1005, "val_dt_offset does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// CutOffData2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CutOffData2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtcptId") )]
	pub ptcpt_id: PartyIdentification242Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetgCutOffDtls") )]
	pub netg_cut_off_dtls: Vec<NettingCutOff2>,
}

impl CutOffData2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.ptcpt_id.validate()?;
		for item in &self.netg_cut_off_dtls { item.validate()? }
		Ok(())
	}
}


// DataModification1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum DataModification1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "INSE") )]
	CodeINSE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UPDT") )]
	CodeUPDT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DELT") )]
	CodeDELT,
}

impl DataModification1Code {
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// DateAndDateTime2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DateAndDateTime2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtTm", skip_serializing_if = "Option::is_none") )]
	pub dt_tm: Option<String>,
}

impl DateAndDateTime2Choice {
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// DateAndDateTimeSearch4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DateAndDateTimeSearch4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtTm", skip_serializing_if = "Option::is_none") )]
	pub dt_tm: Option<DateTimeSearch2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<DatePeriodSearch1Choice>,
}

impl DateAndDateTimeSearch4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dt_tm { val.validate()? }
		if let Some(ref val) = self.dt { val.validate()? }
		Ok(())
	}
}


// DateAndPlaceOfBirth1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DateAndPlaceOfBirth1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "BirthDt") )]
	pub birth_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none") )]
	pub prvc_of_birth: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CityOfBirth") )]
	pub city_of_birth: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfBirth") )]
	pub ctry_of_birth: String,
}

impl DateAndPlaceOfBirth1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.prvc_of_birth {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prvc_of_birth is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prvc_of_birth exceeds the maximum length of 35".to_string()));
			}
		}
		if self.city_of_birth.chars().count() < 1 {
			return Err(ValidationError::new(1001, "city_of_birth is shorter than the minimum length of 1".to_string()));
		}
		if self.city_of_birth.chars().count() > 35 {
			return Err(ValidationError::new(1002, "city_of_birth exceeds the maximum length of 35".to_string()));
		}
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.ctry_of_birth) {
			return Err(ValidationError::new(1005, "ctry_of_birth does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// DateOrDateTimePeriodChoice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// DatePeriod2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DatePeriod2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt") )]
	pub fr_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt") )]
	pub to_dt: String,
}

impl DatePeriod2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DatePeriod3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DatePeriod3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt", skip_serializing_if = "Option::is_none") )]
	pub fr_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt", skip_serializing_if = "Option::is_none") )]
	pub to_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrToDt", skip_serializing_if = "Option::is_none") )]
	pub fr_to_dt: Option<DatePeriod2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
}

impl DatePeriod3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.fr_to_dt { val.validate()? }
		Ok(())
	}
}


// DatePeriodDetails ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// DatePeriodSearch1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DatePeriodSearch1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt", skip_serializing_if = "Option::is_none") )]
	pub fr_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt", skip_serializing_if = "Option::is_none") )]
	pub to_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrToDt", skip_serializing_if = "Option::is_none") )]
	pub fr_to_dt: Option<DatePeriod2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EQDt", skip_serializing_if = "Option::is_none") )]
	pub eq_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NEQDt", skip_serializing_if = "Option::is_none") )]
	pub neq_dt: Option<String>,
}

impl DatePeriodSearch1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.fr_to_dt { val.validate()? }
		Ok(())
	}
}


// DateTimePeriod1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DateTimePeriod1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDtTm") )]
	pub fr_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDtTm") )]
	pub to_dt_tm: String,
}

impl DateTimePeriod1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DateTimePeriod1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DateTimePeriod1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDtTm", skip_serializing_if = "Option::is_none") )]
	pub fr_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDtTm", skip_serializing_if = "Option::is_none") )]
	pub to_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtTmRg", skip_serializing_if = "Option::is_none") )]
	pub dt_tm_rg: Option<DateTimePeriod1>,
}

impl DateTimePeriod1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dt_tm_rg { val.validate()? }
		Ok(())
	}
}


// DateTimePeriod2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DateTimePeriod2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDtTm") )]
	pub fr_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDtTm", skip_serializing_if = "Option::is_none") )]
	pub to_dt_tm: Option<String>,
}

impl DateTimePeriod2 {
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// DateTimeSearch2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DateTimeSearch2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDtTm", skip_serializing_if = "Option::is_none") )]
	pub fr_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDtTm", skip_serializing_if = "Option::is_none") )]
	pub to_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrToDtTm", skip_serializing_if = "Option::is_none") )]
	pub fr_to_dt_tm: Option<DateTimePeriod1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EQDtTm", skip_serializing_if = "Option::is_none") )]
	pub eq_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NEQDtTm", skip_serializing_if = "Option::is_none") )]
	pub neq_dt_tm: Option<String>,
}

impl DateTimeSearch2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.fr_to_dt_tm { val.validate()? }
		Ok(())
	}
}


// Debt5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Debt5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtCcy", skip_serializing_if = "Option::is_none") )]
	pub pmt_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none") )]
	pub face_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtFrqcy", skip_serializing_if = "Option::is_none") )]
	pub pmt_frqcy: Option<Frequency35Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFxgDt", skip_serializing_if = "Option::is_none") )]
	pub intrst_fxg_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtdDt", skip_serializing_if = "Option::is_none") )]
	pub dtd_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrstPmtDt", skip_serializing_if = "Option::is_none") )]
	pub frst_pmt_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none") )]
	pub mtrty_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NxtCpnDt", skip_serializing_if = "Option::is_none") )]
	pub nxt_cpn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PutblDt", skip_serializing_if = "Option::is_none") )]
	pub putbl_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NxtCllblDt", skip_serializing_if = "Option::is_none") )]
	pub nxt_cllbl_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NxtFctrDt", skip_serializing_if = "Option::is_none") )]
	pub nxt_fctr_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XprtnDt", skip_serializing_if = "Option::is_none") )]
	pub xprtn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtDrctnInd", skip_serializing_if = "Option::is_none") )]
	pub pmt_drctn_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none") )]
	pub intrst_rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NxtIntrstRate", skip_serializing_if = "Option::is_none") )]
	pub nxt_intrst_rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OddCpnInd", skip_serializing_if = "Option::is_none") )]
	pub odd_cpn_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CllblInd", skip_serializing_if = "Option::is_none") )]
	pub cllbl_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CPPrgm", skip_serializing_if = "Option::is_none") )]
	pub cp_prgm: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CPRegnTp", skip_serializing_if = "Option::is_none") )]
	pub cp_regn_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstAcrlDt", skip_serializing_if = "Option::is_none") )]
	pub intrst_acrl_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PutblInd", skip_serializing_if = "Option::is_none") )]
	pub putbl_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PreFnddInd", skip_serializing_if = "Option::is_none") )]
	pub pre_fndd_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EscrwdInd", skip_serializing_if = "Option::is_none") )]
	pub escrwd_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PerptlInd", skip_serializing_if = "Option::is_none") )]
	pub perptl_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubrdntdInd", skip_serializing_if = "Option::is_none") )]
	pub subrdntd_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XtndblInd", skip_serializing_if = "Option::is_none") )]
	pub xtndbl_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XtndblPrd", skip_serializing_if = "Option::is_none") )]
	pub xtndbl_prd: Option<DateTimePeriod1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VarblRateInd", skip_serializing_if = "Option::is_none") )]
	pub varbl_rate_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OverAlltmtAmt", skip_serializing_if = "Option::is_none") )]
	pub over_alltmt_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OverAlltmtRate", skip_serializing_if = "Option::is_none") )]
	pub over_alltmt_rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmtsblInd", skip_serializing_if = "Option::is_none") )]
	pub amtsbl_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstClctnMtd", skip_serializing_if = "Option::is_none") )]
	pub intrst_clctn_mtd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CptlsdIntrst", skip_serializing_if = "Option::is_none") )]
	pub cptlsd_intrst: Option<DistributionPolicy2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ActlDnmtnAmt", skip_serializing_if = "Option::is_none") )]
	pub actl_dnmtn_amt: Option<Vec<ActiveCurrencyAndAmount>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CurFctr", skip_serializing_if = "Option::is_none") )]
	pub cur_fctr: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NxtFctr", skip_serializing_if = "Option::is_none") )]
	pub nxt_fctr: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsFctr", skip_serializing_if = "Option::is_none") )]
	pub prvs_fctr: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pcs", skip_serializing_if = "Option::is_none") )]
	pub pcs: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PlsMax", skip_serializing_if = "Option::is_none") )]
	pub pls_max: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PlsPerMln", skip_serializing_if = "Option::is_none") )]
	pub pls_per_mln: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PlsPerLot", skip_serializing_if = "Option::is_none") )]
	pub pls_per_lot: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PlsPerTrad", skip_serializing_if = "Option::is_none") )]
	pub pls_per_trad: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CstPrePmtPnltyInd", skip_serializing_if = "Option::is_none") )]
	pub cst_pre_pmt_pnlty_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LotId", skip_serializing_if = "Option::is_none") )]
	pub lot_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CstPrePmtYld", skip_serializing_if = "Option::is_none") )]
	pub cst_pre_pmt_yld: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WghtdAvrgCpn", skip_serializing_if = "Option::is_none") )]
	pub wghtd_avrg_cpn: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WghtdAvrgLife", skip_serializing_if = "Option::is_none") )]
	pub wghtd_avrg_life: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WghtdAvrgLn", skip_serializing_if = "Option::is_none") )]
	pub wghtd_avrg_ln: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WghtdAvrgMtrty", skip_serializing_if = "Option::is_none") )]
	pub wghtd_avrg_mtrty: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InsrdInd", skip_serializing_if = "Option::is_none") )]
	pub insrd_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BkQlfdInd", skip_serializing_if = "Option::is_none") )]
	pub bk_qlfd_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YldClctn", skip_serializing_if = "Option::is_none") )]
	pub yld_clctn: Option<Vec<YieldCalculation6>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstTp", skip_serializing_if = "Option::is_none") )]
	pub intrst_tp: Option<InterestType3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrmStrTp", skip_serializing_if = "Option::is_none") )]
	pub instrm_str_tp: Option<InstrumentSubStructureType2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GblTp", skip_serializing_if = "Option::is_none") )]
	pub gbl_tp: Option<GlobalNote2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PotntlEuroSysElgblty", skip_serializing_if = "Option::is_none") )]
	pub potntl_euro_sys_elgblty: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Geogcs", skip_serializing_if = "Option::is_none") )]
	pub geogcs: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YldRg", skip_serializing_if = "Option::is_none") )]
	pub yld_rg: Option<AmountOrPercentageRange1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CpnRg", skip_serializing_if = "Option::is_none") )]
	pub cpn_rg: Option<AmountOrPercentageRange1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
	pub purp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AltrntvMinTaxInd", skip_serializing_if = "Option::is_none") )]
	pub altrntv_min_tax_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AutoRinvstmt", skip_serializing_if = "Option::is_none") )]
	pub auto_rinvstmt: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Hrcut", skip_serializing_if = "Option::is_none") )]
	pub hrcut: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxConds", skip_serializing_if = "Option::is_none") )]
	pub tx_conds: Option<TradeTransactionCondition7Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LookBck", skip_serializing_if = "Option::is_none") )]
	pub look_bck: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MaxSbstitn", skip_serializing_if = "Option::is_none") )]
	pub max_sbstitn: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinIncrmt", skip_serializing_if = "Option::is_none") )]
	pub min_incrmt: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinQty", skip_serializing_if = "Option::is_none") )]
	pub min_qty: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pdctn", skip_serializing_if = "Option::is_none") )]
	pub pdctn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RstrctdInd", skip_serializing_if = "Option::is_none") )]
	pub rstrctd_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricFrqcy", skip_serializing_if = "Option::is_none") )]
	pub pric_frqcy: Option<Frequency35Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sctr", skip_serializing_if = "Option::is_none") )]
	pub sctr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SbstitnFrqcy", skip_serializing_if = "Option::is_none") )]
	pub sbstitn_frqcy: Option<Frequency35Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SbstitnLft", skip_serializing_if = "Option::is_none") )]
	pub sbstitn_lft: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WhlPoolInd", skip_serializing_if = "Option::is_none") )]
	pub whl_pool_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricSrc", skip_serializing_if = "Option::is_none") )]
	pub pric_src: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricRg", skip_serializing_if = "Option::is_none") )]
	pub pric_rg: Option<AmountOrPercentageRange1>,
}

impl Debt5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pmt_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "pmt_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.face_amt { val.validate()? }
		if let Some(ref val) = self.pmt_frqcy { val.validate()? }
		if let Some(ref val) = self.cp_regn_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cp_regn_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "cp_regn_tp exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.xtndbl_prd { val.validate()? }
		if let Some(ref val) = self.over_alltmt_amt { val.validate()? }
		if let Some(ref val) = self.intrst_clctn_mtd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "intrst_clctn_mtd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "intrst_clctn_mtd exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.cptlsd_intrst { val.validate()? }
		if let Some(ref vec) = self.actl_dnmtn_amt { for item in vec { item.validate()? } }
		if let Some(ref val) = self.lot_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "lot_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "lot_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.yld_clctn { for item in vec { item.validate()? } }
		if let Some(ref val) = self.intrst_tp { val.validate()? }
		if let Some(ref val) = self.instrm_str_tp { val.validate()? }
		if let Some(ref val) = self.gbl_tp { val.validate()? }
		if let Some(ref val) = self.geogcs {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "geogcs is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "geogcs exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.yld_rg { val.validate()? }
		if let Some(ref val) = self.cpn_rg { val.validate()? }
		if let Some(ref val) = self.purp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "purp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "purp exceeds the maximum length of 256".to_string()));
			}
		}
		if let Some(ref val) = self.tx_conds { val.validate()? }
		if let Some(ref val) = self.min_incrmt { val.validate()? }
		if let Some(ref val) = self.min_qty { val.validate()? }
		if let Some(ref val) = self.pdctn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pdctn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "pdctn exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.pric_frqcy { val.validate()? }
		if let Some(ref val) = self.sctr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sctr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "sctr exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.sbstitn_frqcy { val.validate()? }
		if let Some(ref val) = self.pric_src {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pric_src is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "pric_src exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.pric_rg { val.validate()? }
		Ok(())
	}
}


// DebtorActivation5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DebtorActivation5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrActvtnId", skip_serializing_if = "Option::is_none") )]
	pub dbtr_actvtn_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DispNm", skip_serializing_if = "Option::is_none") )]
	pub disp_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<RTPPartyIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr") )]
	pub dbtr: RTPPartyIdentification2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrSolPrvdr") )]
	pub dbtr_sol_prvdr: RTPPartyIdentification2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CstmrId", skip_serializing_if = "Option::is_none") )]
	pub cstmr_id: Option<Vec<Party53Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctFrmtTp", skip_serializing_if = "Option::is_none") )]
	pub ctrct_frmt_tp: Option<Vec<DocumentFormat2Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctRef", skip_serializing_if = "Option::is_none") )]
	pub ctrct_ref: Option<Vec<ContractReference1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr") )]
	pub cdtr: RTPPartyIdentification2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<RTPPartyIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ActvtnReqDlvryPty", skip_serializing_if = "Option::is_none") )]
	pub actvtn_req_dlvry_pty: Option<RTPPartyIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt", skip_serializing_if = "Option::is_none") )]
	pub start_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndDt", skip_serializing_if = "Option::is_none") )]
	pub end_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DdctdActvtnCd", skip_serializing_if = "Option::is_none") )]
	pub ddctd_actvtn_cd: Option<String>,
}

impl DebtorActivation5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dbtr_actvtn_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dbtr_actvtn_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "dbtr_actvtn_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.disp_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "disp_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "disp_nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		self.dbtr.validate()?;
		self.dbtr_sol_prvdr.validate()?;
		if let Some(ref vec) = self.cstmr_id { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.ctrct_frmt_tp { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.ctrct_ref { for item in vec { item.validate()? } }
		self.cdtr.validate()?;
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		if let Some(ref val) = self.actvtn_req_dlvry_pty { val.validate()? }
		if let Some(ref val) = self.start_dt { val.validate()? }
		if let Some(ref val) = self.end_dt { val.validate()? }
		if let Some(ref val) = self.ddctd_actvtn_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ddctd_actvtn_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ddctd_actvtn_cd exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// DebtorActivation6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DebtorActivation6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrActvtnId", skip_serializing_if = "Option::is_none") )]
	pub dbtr_actvtn_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DispNm", skip_serializing_if = "Option::is_none") )]
	pub disp_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<RTPPartyIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr", skip_serializing_if = "Option::is_none") )]
	pub dbtr: Option<RTPPartyIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrSolPrvdr", skip_serializing_if = "Option::is_none") )]
	pub dbtr_sol_prvdr: Option<RTPPartyIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CstmrId", skip_serializing_if = "Option::is_none") )]
	pub cstmr_id: Option<Vec<Party53Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctFrmtTp", skip_serializing_if = "Option::is_none") )]
	pub ctrct_frmt_tp: Option<Vec<DocumentFormat2Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctRef", skip_serializing_if = "Option::is_none") )]
	pub ctrct_ref: Option<Vec<ContractReference1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr", skip_serializing_if = "Option::is_none") )]
	pub cdtr: Option<RTPPartyIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<RTPPartyIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ActvtnReqDlvryPty", skip_serializing_if = "Option::is_none") )]
	pub actvtn_req_dlvry_pty: Option<RTPPartyIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt", skip_serializing_if = "Option::is_none") )]
	pub start_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndDt", skip_serializing_if = "Option::is_none") )]
	pub end_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DdctdActvtnCd", skip_serializing_if = "Option::is_none") )]
	pub ddctd_actvtn_cd: Option<String>,
}

impl DebtorActivation6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dbtr_actvtn_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dbtr_actvtn_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "dbtr_actvtn_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.disp_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "disp_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "disp_nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.dbtr { val.validate()? }
		if let Some(ref val) = self.dbtr_sol_prvdr { val.validate()? }
		if let Some(ref vec) = self.cstmr_id { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.ctrct_frmt_tp { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.ctrct_ref { for item in vec { item.validate()? } }
		if let Some(ref val) = self.cdtr { val.validate()? }
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		if let Some(ref val) = self.actvtn_req_dlvry_pty { val.validate()? }
		if let Some(ref val) = self.start_dt { val.validate()? }
		if let Some(ref val) = self.end_dt { val.validate()? }
		if let Some(ref val) = self.ddctd_actvtn_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ddctd_actvtn_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ddctd_actvtn_cd exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// DebtorActivationAmendment5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DebtorActivationAmendment5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlBizInstr", skip_serializing_if = "Option::is_none") )]
	pub orgnl_biz_instr: Option<OriginalBusinessInstruction1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmdmntRsn", skip_serializing_if = "Option::is_none") )]
	pub amdmnt_rsn: Option<DebtorActivationAmendmentReason3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amdmnt") )]
	pub amdmnt: DebtorActivationAmendment6,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlActvtn") )]
	pub orgnl_actvtn: OriginalActivation3Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl DebtorActivationAmendment5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_biz_instr { val.validate()? }
		if let Some(ref val) = self.amdmnt_rsn { val.validate()? }
		self.amdmnt.validate()?;
		self.orgnl_actvtn.validate()?;
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// DebtorActivationAmendment6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DebtorActivationAmendment6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrActvtn", skip_serializing_if = "Option::is_none") )]
	pub dbtr_actvtn: Option<DebtorActivation6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ElctrncInvcData", skip_serializing_if = "Option::is_none") )]
	pub elctrnc_invc_data: Option<ElectronicInvoice1>,
}

impl DebtorActivationAmendment6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dbtr_actvtn { val.validate()? }
		if let Some(ref val) = self.elctrnc_invc_data { val.validate()? }
		Ok(())
	}
}


// DebtorActivationAmendmentReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DebtorActivationAmendmentReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl DebtorActivationAmendmentReason1Choice {
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


// DebtorActivationAmendmentReason3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DebtorActivationAmendmentReason3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Orgtr", skip_serializing_if = "Option::is_none") )]
	pub orgtr: Option<RTPPartyIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn") )]
	pub rsn: DebtorActivationAmendmentReason1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<String>>,
}

impl DebtorActivationAmendmentReason3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgtr { val.validate()? }
		self.rsn.validate()?;
		if let Some(ref vec) = self.addtl_inf {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 105 {
					return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 105".to_string()));
				}
			}
		}
		Ok(())
	}
}


// DebtorActivationCancellation3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DebtorActivationCancellation3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlBizInstr", skip_serializing_if = "Option::is_none") )]
	pub orgnl_biz_instr: Option<OriginalBusinessInstruction1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CxlRsn", skip_serializing_if = "Option::is_none") )]
	pub cxl_rsn: Option<DebtorActivationCancellationReason3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlActvtn") )]
	pub orgnl_actvtn: OriginalActivation3Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl DebtorActivationCancellation3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_biz_instr { val.validate()? }
		if let Some(ref val) = self.cxl_rsn { val.validate()? }
		self.orgnl_actvtn.validate()?;
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// DebtorActivationCancellationReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DebtorActivationCancellationReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl DebtorActivationCancellationReason1Choice {
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


// DebtorActivationCancellationReason3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DebtorActivationCancellationReason3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Orgtr", skip_serializing_if = "Option::is_none") )]
	pub orgtr: Option<RTPPartyIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn") )]
	pub rsn: DebtorActivationCancellationReason1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<String>>,
}

impl DebtorActivationCancellationReason3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgtr { val.validate()? }
		self.rsn.validate()?;
		if let Some(ref vec) = self.addtl_inf {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 105 {
					return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 105".to_string()));
				}
			}
		}
		Ok(())
	}
}


// DebtorActivationStatusReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DebtorActivationStatusReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl DebtorActivationStatusReason1Choice {
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


// DebtorActivationStatusReason3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DebtorActivationStatusReason3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Orgtr", skip_serializing_if = "Option::is_none") )]
	pub orgtr: Option<RTPPartyIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn") )]
	pub rsn: DebtorActivationStatusReason1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<String>>,
}

impl DebtorActivationStatusReason3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgtr { val.validate()? }
		self.rsn.validate()?;
		if let Some(ref vec) = self.addtl_inf {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 105 {
					return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 105".to_string()));
				}
			}
		}
		Ok(())
	}
}


// Derivative4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Derivative4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Futr", skip_serializing_if = "Option::is_none") )]
	pub futr: Option<Future4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Optn", skip_serializing_if = "Option::is_none") )]
	pub optn: Option<Option15>,
}

impl Derivative4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.futr { val.validate()? }
		if let Some(ref val) = self.optn { val.validate()? }
		Ok(())
	}
}


// DistributionPolicy1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// DistributionPolicy2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DistributionPolicy2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<DistributionPolicy1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl DistributionPolicy2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// DistributionStrategy1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// DocumentFormat2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DocumentFormat2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification1>,
}

impl DocumentFormat2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// DocumentType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DocumentType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification1>,
}

impl DocumentType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// EMTDataReportingVFMUKType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// EUCapitalGain2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// EUSavingsDirective1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// EffectiveDate1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct EffectiveDate1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FctvDt") )]
	pub fctv_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FctvDtParam", skip_serializing_if = "Option::is_none") )]
	pub fctv_dt_param: Option<String>,
}

impl EffectiveDate1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.fctv_dt_param {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "fctv_dt_param is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "fctv_dt_param exceeds the maximum length of 4".to_string()));
			}
		}
		Ok(())
	}
}


// ElectronicInvoice1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ElectronicInvoice1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PresntmntTp") )]
	pub presntmnt_tp: PresentmentType1Code,
}

impl ElectronicInvoice1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.presntmnt_tp.validate()?;
		Ok(())
	}
}


// EnrolmentHeader3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct EnrolmentHeader3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
	pub cre_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgOrgtr", skip_serializing_if = "Option::is_none") )]
	pub msg_orgtr: Option<RTPPartyIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgRcpt", skip_serializing_if = "Option::is_none") )]
	pub msg_rcpt: Option<RTPPartyIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitgPty") )]
	pub initg_pty: RTPPartyIdentification2,
}

impl EnrolmentHeader3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.msg_orgtr { val.validate()? }
		if let Some(ref val) = self.msg_rcpt { val.validate()? }
		self.initg_pty.validate()?;
		Ok(())
	}
}


// EnrolmentStatus3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct EnrolmentStatus3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlBizInstr", skip_serializing_if = "Option::is_none") )]
	pub orgnl_biz_instr: Option<OriginalBusinessInstruction1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts") )]
	pub sts: ServiceStatus1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsRsn", skip_serializing_if = "Option::is_none") )]
	pub sts_rsn: Option<CreditorEnrolmentStatusReason3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlEnrlmntRef", skip_serializing_if = "Option::is_none") )]
	pub orgnl_enrlmnt_ref: Option<OriginalEnrolment3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FctvEnrlmntDt", skip_serializing_if = "Option::is_none") )]
	pub fctv_enrlmnt_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl EnrolmentStatus3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_biz_instr { val.validate()? }
		self.sts.validate()?;
		if let Some(ref val) = self.sts_rsn { val.validate()? }
		if let Some(ref val) = self.orgnl_enrlmnt_ref { val.validate()? }
		if let Some(ref val) = self.fctv_enrlmnt_dt { val.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// Equity3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Equity3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrefToIncm") )]
	pub pref_to_incm: PreferenceToIncome5Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none") )]
	pub mtrty_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NonPdAmt", skip_serializing_if = "Option::is_none") )]
	pub non_pd_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ParVal", skip_serializing_if = "Option::is_none") )]
	pub par_val: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VtngRghtsPerShr", skip_serializing_if = "Option::is_none") )]
	pub vtng_rghts_per_shr: Option<f64>,
}

impl Equity3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pref_to_incm.validate()?;
		if let Some(ref val) = self.non_pd_amt { val.validate()? }
		if let Some(ref val) = self.par_val { val.validate()? }
		Ok(())
	}
}


// ErrorHandling1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum ErrorHandling1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "X020") )]
	CodeX020,
	#[cfg_attr( feature = "derive_serde", serde(rename = "X030") )]
	CodeX030,
	#[cfg_attr( feature = "derive_serde", serde(rename = "X050") )]
	CodeX050,
}

impl ErrorHandling1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ErrorHandling2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ErrorHandling2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<ErrorHandling1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ErrorHandling2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
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


// ErrorHandling3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ErrorHandling3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ErrorHandling3Choice {
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


// ErrorHandling4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ErrorHandling4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Err") )]
	pub err: ErrorHandling2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
}

impl ErrorHandling4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.err.validate()?;
		if let Some(ref val) = self.desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// ErrorHandling5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ErrorHandling5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Err") )]
	pub err: ErrorHandling3Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
}

impl ErrorHandling5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.err.validate()?;
		if let Some(ref val) = self.desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// EventFrequency1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// EventFrequency5Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// FinancialInstrument71 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FinancialInstrument71 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: SecurityIdentification19,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none") )]
	pub shrt_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryId", skip_serializing_if = "Option::is_none") )]
	pub splmtry_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClssTp", skip_serializing_if = "Option::is_none") )]
	pub clss_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesForm", skip_serializing_if = "Option::is_none") )]
	pub scties_form: Option<FormOfSecurity1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DstrbtnPlcy", skip_serializing_if = "Option::is_none") )]
	pub dstrbtn_plcy: Option<DistributionPolicy1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PdctGrp", skip_serializing_if = "Option::is_none") )]
	pub pdct_grp: Option<String>,
}

impl FinancialInstrument71 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		if let Some(ref val) = self.shrt_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "shrt_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "shrt_nm exceeds the maximum length of 35".to_string()));
			}
		}
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
		if let Some(ref val) = self.pdct_grp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pdct_grp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "pdct_grp exceeds the maximum length of 140".to_string()));
			}
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// FinancialInstrument96 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// FinancialInstrument97 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FinancialInstrument97 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Eqty", skip_serializing_if = "Option::is_none") )]
	pub eqty: Option<Equity3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Warrt", skip_serializing_if = "Option::is_none") )]
	pub warrt: Option<Warrant4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Debt", skip_serializing_if = "Option::is_none") )]
	pub debt: Option<Debt5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Deriv", skip_serializing_if = "Option::is_none") )]
	pub deriv: Option<Derivative4>,
}

impl FinancialInstrument97 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.eqty { val.validate()? }
		if let Some(ref val) = self.warrt { val.validate()? }
		if let Some(ref val) = self.debt { val.validate()? }
		if let Some(ref val) = self.deriv { val.validate()? }
		Ok(())
	}
}


// FinancialInstrumentForm2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FinancialInstrumentForm2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "BookgApprnc", skip_serializing_if = "Option::is_none") )]
	pub bookg_apprnc: Option<Appearance3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglForm", skip_serializing_if = "Option::is_none") )]
	pub lgl_form: Option<FormOfSecurity8Choice>,
}

impl FinancialInstrumentForm2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.bookg_apprnc { val.validate()? }
		if let Some(ref val) = self.lgl_form { val.validate()? }
		Ok(())
	}
}


// FinancialInstrumentIdentificationValidity3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FinancialInstrumentIdentificationValidity3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_id: Option<SecurityIdentification39>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISINVldFr", skip_serializing_if = "Option::is_none") )]
	pub isin_vld_fr: Option<String>,
}

impl FinancialInstrumentIdentificationValidity3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.fin_instrm_id { val.validate()? }
		Ok(())
	}
}


// FinancialInstrumentName2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FinancialInstrumentName2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISOShrtNm", skip_serializing_if = "Option::is_none") )]
	pub iso_shrt_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISOLngNm", skip_serializing_if = "Option::is_none") )]
	pub iso_lng_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldFr", skip_serializing_if = "Option::is_none") )]
	pub vld_fr: Option<DateAndDateTime2Choice>,
}

impl FinancialInstrumentName2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.iso_shrt_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "iso_shrt_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "iso_shrt_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.iso_lng_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "iso_lng_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "iso_lng_nm exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.vld_fr { val.validate()? }
		Ok(())
	}
}


// FinancialInstrumentQuantity1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FinancialInstrumentQuantity1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Unit") )]
	pub unit: f64,
}

impl FinancialInstrumentQuantity1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FinancialInstrumentQuantity1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FinancialInstrumentQuantity1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Unit", skip_serializing_if = "Option::is_none") )]
	pub unit: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none") )]
	pub face_amt: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmtsdVal", skip_serializing_if = "Option::is_none") )]
	pub amtsd_val: Option<f64>,
}

impl FinancialInstrumentQuantity1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.face_amt {
			if *val < 0.000000 {
				return Err(ValidationError::new(1003, "face_amt is less than the minimum value of 0.000000".to_string()));
			}
		}
		if let Some(ref val) = self.amtsd_val {
			if *val < 0.000000 {
				return Err(ValidationError::new(1003, "amtsd_val is less than the minimum value of 0.000000".to_string()));
			}
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// FormOfSecurity8Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FormOfSecurity8Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<FormOfSecurity1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl FormOfSecurity8Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// Forms1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// Frequency35Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Frequency35Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<Frequency5Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl Frequency35Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// Frequency5Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum Frequency5Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
	CodeYEAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
	CodeMNTH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QURT") )]
	CodeQURT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MIAN") )]
	CodeMIAN,
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
	#[cfg_attr( feature = "derive_serde", serde(rename = "TEND") )]
	CodeTEND,
}

impl Frequency5Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FundOrderType10Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// FundParameters4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FundParameters4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NoCrit", skip_serializing_if = "Option::is_none") )]
	pub no_crit: Option<NoCriteria1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Params", skip_serializing_if = "Option::is_none") )]
	pub params: Option<FundParameters5>,
}

impl FundParameters4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.no_crit { val.validate()? }
		if let Some(ref val) = self.params { val.validate()? }
		Ok(())
	}
}


// FundParameters5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FundParameters5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmDtls", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_dtls: Option<Vec<FinancialInstrument71>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FndMgmtCpny", skip_serializing_if = "Option::is_none") )]
	pub fnd_mgmt_cpny: Option<Vec<PartyIdentification139>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtFr", skip_serializing_if = "Option::is_none") )]
	pub dt_fr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfDmcl", skip_serializing_if = "Option::is_none") )]
	pub ctry_of_dmcl: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegdDstrbtnCtry", skip_serializing_if = "Option::is_none") )]
	pub regd_dstrbtn_ctry: Option<Vec<String>>,
}

impl FundParameters5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.fin_instrm_dtls { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.fnd_mgmt_cpny { for item in vec { item.validate()? } }
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
		Ok(())
	}
}


// FundParties1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// Future4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Future4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctSz", skip_serializing_if = "Option::is_none") )]
	pub ctrct_sz: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExrcPric", skip_serializing_if = "Option::is_none") )]
	pub exrc_pric: Option<Price8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FutrDt", skip_serializing_if = "Option::is_none") )]
	pub futr_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinSz", skip_serializing_if = "Option::is_none") )]
	pub min_sz: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none") )]
	pub unit_of_measr: Option<UnitOfMeasure7Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TmUnit", skip_serializing_if = "Option::is_none") )]
	pub tm_unit: Option<TimeUnit3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlUndrlygAttrbts", skip_serializing_if = "Option::is_none") )]
	pub addtl_undrlyg_attrbts: Option<Vec<UnderlyingAttributes4>>,
}

impl Future4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.exrc_pric { val.validate()? }
		if let Some(ref val) = self.min_sz { val.validate()? }
		if let Some(ref val) = self.unit_of_measr { val.validate()? }
		if let Some(ref val) = self.tm_unit { val.validate()? }
		if let Some(ref vec) = self.addtl_undrlyg_attrbts { for item in vec { item.validate()? } }
		Ok(())
	}
}


// GenericAccountIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// GenericIdentification13 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GenericIdentification13 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
	pub issr: String,
}

impl GenericIdentification13 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 4 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 4".to_string()));
		}
		let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
		if !pattern.is_match(&self.id) {
			return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.schme_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "schme_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "schme_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if self.issr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
		}
		if self.issr.chars().count() > 35 {
			return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// GenericIdentification30 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GenericIdentification30 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
	pub issr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<String>,
}

impl GenericIdentification30 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
		if !pattern.is_match(&self.id) {
			return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
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


// GenericIdentification36 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// GenericIdentification49 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GenericIdentification49 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IdTp") )]
	pub id_tp: String,
}

impl GenericIdentification49 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if self.id_tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id_tp is shorter than the minimum length of 1".to_string()));
		}
		if self.id_tp.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id_tp exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// GenericOrganisationIdentification3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GenericOrganisationIdentification3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericOrganisationIdentification3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 256 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 256".to_string()));
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


// GenericOrganisationType1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GenericOrganisationType1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Reqd") )]
	pub reqd: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm") )]
	pub schme_nm: OrganisationIdentificationSchemeName1Choice,
}

impl GenericOrganisationType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.schme_nm.validate()?;
		Ok(())
	}
}


// GenericPersonIdentification2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GenericPersonIdentification2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericPersonIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 256 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 256".to_string()));
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


// GenericPersonType1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GenericPersonType1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Reqd") )]
	pub reqd: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm") )]
	pub schme_nm: PersonIdentificationSchemeName1Choice,
}

impl GenericPersonType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.schme_nm.validate()?;
		Ok(())
	}
}


// GlobalNote1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum GlobalNote1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NGNO") )]
	CodeNGNO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CGNO") )]
	CodeCGNO,
}

impl GlobalNote1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// GlobalNote2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GlobalNote2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<GlobalNote1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl GlobalNote2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// GovernanceProcess1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// InitialPhysicalForm1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum InitialPhysicalForm1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "GTGT") )]
	CodeGTGT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GPGP") )]
	CodeGPGP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DERN") )]
	CodeDERN,
}

impl InitialPhysicalForm1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InitialPhysicalForm2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum InitialPhysicalForm2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "GPGP") )]
	CodeGPGP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DERN") )]
	CodeDERN,
}

impl InitialPhysicalForm2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InitialPhysicalForm3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct InitialPhysicalForm3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<InitialPhysicalForm2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl InitialPhysicalForm3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// InitialPhysicalForm4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct InitialPhysicalForm4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<InitialPhysicalForm1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl InitialPhysicalForm4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// InstrumentSubStructureType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum InstrumentSubStructureType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ABSE") )]
	CodeABSE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AIRT") )]
	CodeAIRT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AUTT") )]
	CodeAUTT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CBOB") )]
	CodeCBOB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CDOB") )]
	CodeCDOB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLNO") )]
	CodeCLNO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLOB") )]
	CodeCLOB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CMBS") )]
	CodeCMBS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CSMR") )]
	CodeCSMR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRCT") )]
	CodeCRCT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HELO") )]
	CodeHELO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LPNO") )]
	CodeLPNO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PFAB") )]
	CodePFAB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PYRT") )]
	CodePYRT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REPK") )]
	CodeREPK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RMBS") )]
	CodeRMBS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SCBO") )]
	CodeSCBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "STRB") )]
	CodeSTRB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "STUT") )]
	CodeSTUT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WBSE") )]
	CodeWBSE,
}

impl InstrumentSubStructureType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InstrumentSubStructureType2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct InstrumentSubStructureType2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<InstrumentSubStructureType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl InstrumentSubStructureType2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// IntendedOrActual2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// InterestType3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum InterestType3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ZCPN") )]
	CodeZCPN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FIXD") )]
	CodeFIXD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FLRN") )]
	CodeFLRN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DUAL") )]
	CodeDUAL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INDE") )]
	CodeINDE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DSCO") )]
	CodeDSCO,
}

impl InterestType3Code {
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// InvestorRestrictionType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum InvestorRestrictionType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "LERE") )]
	CodeLERE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CITI") )]
	CodeCITI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INDV") )]
	CodeINDV,
}

impl InvestorRestrictionType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InvestorRestrictionType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct InvestorRestrictionType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<InvestorRestrictionType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl InvestorRestrictionType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// InvestorType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum InvestorType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "RETL") )]
	CodeRETL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PROF") )]
	CodePROF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "STAF") )]
	CodeSTAF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PPER") )]
	CodePPER,
}

impl InvestorType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InvestorType2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// InvestorType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct InvestorType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<InvestorType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl InvestorType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// InvestorType3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// Issuance5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Issuance5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "IssePlc", skip_serializing_if = "Option::is_none") )]
	pub isse_plc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfIsse", skip_serializing_if = "Option::is_none") )]
	pub ctry_of_isse: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IsseDt", skip_serializing_if = "Option::is_none") )]
	pub isse_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnncmntDt", skip_serializing_if = "Option::is_none") )]
	pub anncmnt_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISINVldFr", skip_serializing_if = "Option::is_none") )]
	pub isin_vld_fr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IssrOrg", skip_serializing_if = "Option::is_none") )]
	pub issr_org: Option<Organisation38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IsseNmnlAmt", skip_serializing_if = "Option::is_none") )]
	pub isse_nmnl_amt: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FullIssdAmt", skip_serializing_if = "Option::is_none") )]
	pub full_issd_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IsseSz", skip_serializing_if = "Option::is_none") )]
	pub isse_sz: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IssePric", skip_serializing_if = "Option::is_none") )]
	pub isse_pric: Option<PriceValue1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IssncDstrbtn", skip_serializing_if = "Option::is_none") )]
	pub issnc_dstrbtn: Option<SecuritiesTransactionType31Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GovngLaw", skip_serializing_if = "Option::is_none") )]
	pub govng_law: Option<Vec<Jurisdiction1>>,
}

impl Issuance5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.isse_plc {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "isse_plc does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ctry_of_isse {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry_of_isse does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.issr_org { val.validate()? }
		if let Some(ref val) = self.isse_nmnl_amt { val.validate()? }
		if let Some(ref val) = self.full_issd_amt { val.validate()? }
		if let Some(ref val) = self.isse_pric { val.validate()? }
		if let Some(ref val) = self.issnc_dstrbtn { val.validate()? }
		if let Some(ref vec) = self.govng_law { for item in vec { item.validate()? } }
		Ok(())
	}
}


// Issuance6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Issuance6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "IssePlc", skip_serializing_if = "Option::is_none") )]
	pub isse_plc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfIsse", skip_serializing_if = "Option::is_none") )]
	pub ctry_of_isse: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IsseDt", skip_serializing_if = "Option::is_none") )]
	pub isse_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnncmntDt", skip_serializing_if = "Option::is_none") )]
	pub anncmnt_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IssrOrg", skip_serializing_if = "Option::is_none") )]
	pub issr_org: Option<Organisation38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IsseNmnlAmt", skip_serializing_if = "Option::is_none") )]
	pub isse_nmnl_amt: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FullIssdAmt", skip_serializing_if = "Option::is_none") )]
	pub full_issd_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IsseSz", skip_serializing_if = "Option::is_none") )]
	pub isse_sz: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IssePric", skip_serializing_if = "Option::is_none") )]
	pub isse_pric: Option<PriceValue1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IssncDstrbtn", skip_serializing_if = "Option::is_none") )]
	pub issnc_dstrbtn: Option<SecuritiesTransactionType31Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GovngLaw", skip_serializing_if = "Option::is_none") )]
	pub govng_law: Option<Vec<Jurisdiction1>>,
}

impl Issuance6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.isse_plc {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "isse_plc does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ctry_of_isse {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry_of_isse does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.issr_org { val.validate()? }
		if let Some(ref val) = self.isse_nmnl_amt { val.validate()? }
		if let Some(ref val) = self.full_issd_amt { val.validate()? }
		if let Some(ref val) = self.isse_pric { val.validate()? }
		if let Some(ref val) = self.issnc_dstrbtn { val.validate()? }
		if let Some(ref vec) = self.govng_law { for item in vec { item.validate()? } }
		Ok(())
	}
}


// IssuanceAccount2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct IssuanceAccount2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "IssncAcct") )]
	pub issnc_acct: SecuritiesAccount19,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmryAcctInd") )]
	pub pmry_acct_ind: bool,
}

impl IssuanceAccount2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.issnc_acct.validate()?;
		Ok(())
	}
}


// Jurisdiction1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Jurisdiction1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
	pub ctry: Option<String>,
}

impl Jurisdiction1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// LegalRestrictions1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum LegalRestrictions1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "USLE") )]
	CodeUSLE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NORE") )]
	CodeNORE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REST") )]
	CodeREST,
}

impl LegalRestrictions1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// LegalRestrictions2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum LegalRestrictions2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "JURO") )]
	CodeJURO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PPLA") )]
	CodePPLA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACRI") )]
	CodeACRI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MARG") )]
	CodeMARG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRIV") )]
	CodePRIV,
}

impl LegalRestrictions2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// LegalRestrictions4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct LegalRestrictions4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<LegalRestrictions1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl LegalRestrictions4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// LegalRestrictions5Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct LegalRestrictions5Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<LegalRestrictions2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl LegalRestrictions5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// LocalMarketAnnex6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// LockStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum LockStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "LOCK") )]
	CodeLOCK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ULCK") )]
	CodeULCK,
}

impl LockStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// LossBearing2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// MarketIdentification87 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MarketIdentification87 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
	pub ctry: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClssfctnTp") )]
	pub clssfctn_tp: ClassificationType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmPurp", skip_serializing_if = "Option::is_none") )]
	pub sttlm_purp: Option<Purpose3Choice>,
}

impl MarketIdentification87 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.ctry) {
			return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
		}
		self.clssfctn_tp.validate()?;
		if let Some(ref val) = self.sttlm_purp { val.validate()? }
		Ok(())
	}
}


// MarketIdentificationOrCashPurpose1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MarketIdentificationOrCashPurpose1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmInstrMktId", skip_serializing_if = "Option::is_none") )]
	pub sttlm_instr_mkt_id: Option<MarketIdentification87>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshSSIPurp", skip_serializing_if = "Option::is_none") )]
	pub csh_ssi_purp: Option<Vec<String>>,
}

impl MarketIdentificationOrCashPurpose1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.sttlm_instr_mkt_id { val.validate()? }
		if let Some(ref vec) = self.csh_ssi_purp {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "csh_ssi_purp is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 4 {
					return Err(ValidationError::new(1002, "csh_ssi_purp exceeds the maximum length of 4".to_string()));
				}
			}
		}
		Ok(())
	}
}


// MarketInfrastructureIdentification1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MarketInfrastructureIdentification1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl MarketInfrastructureIdentification1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 3 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 3".to_string()));
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


// MarketPracticeVersion1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// MarketSpecificAttribute1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MarketSpecificAttribute1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
	pub val: String,
}

impl MarketSpecificAttribute1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 35 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 35".to_string()));
		}
		if self.val.chars().count() < 1 {
			return Err(ValidationError::new(1001, "val is shorter than the minimum length of 1".to_string()));
		}
		if self.val.chars().count() > 350 {
			return Err(ValidationError::new(1002, "val exceeds the maximum length of 350".to_string()));
		}
		Ok(())
	}
}


// MaturityRedemptionType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum MaturityRedemptionType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FRED") )]
	CodeFRED,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRNR") )]
	CodePRNR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRWR") )]
	CodePRWR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RNDM") )]
	CodeRNDM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRRA") )]
	CodePRRA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CALL") )]
	CodeCALL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUUT") )]
	CodePUUT,
}

impl MaturityRedemptionType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// MaturityRedemptionType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MaturityRedemptionType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<MaturityRedemptionType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl MaturityRedemptionType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// MessageHeader1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MessageHeader1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
	pub cre_dt_tm: Option<String>,
}

impl MessageHeader1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// MessageHeader11 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MessageHeader11 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
	pub cre_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqTp", skip_serializing_if = "Option::is_none") )]
	pub req_tp: Option<RequestType4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlBizQry", skip_serializing_if = "Option::is_none") )]
	pub orgnl_biz_qry: Option<OriginalBusinessQuery1>,
}

impl MessageHeader11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.req_tp { val.validate()? }
		if let Some(ref val) = self.orgnl_biz_qry { val.validate()? }
		Ok(())
	}
}


// MessageHeader12 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MessageHeader12 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
	pub cre_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlBizInstr", skip_serializing_if = "Option::is_none") )]
	pub orgnl_biz_instr: Option<OriginalBusinessInstruction1>,
}

impl MessageHeader12 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.orgnl_biz_instr { val.validate()? }
		Ok(())
	}
}


// MessageHeader2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MessageHeader2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
	pub cre_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqTp", skip_serializing_if = "Option::is_none") )]
	pub req_tp: Option<RequestType2Choice>,
}

impl MessageHeader2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.req_tp { val.validate()? }
		Ok(())
	}
}


// MessageHeader3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MessageHeader3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
	pub cre_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqTp", skip_serializing_if = "Option::is_none") )]
	pub req_tp: Option<RequestType2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlBizQry", skip_serializing_if = "Option::is_none") )]
	pub orgnl_biz_qry: Option<OriginalBusinessQuery1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QryNm", skip_serializing_if = "Option::is_none") )]
	pub qry_nm: Option<String>,
}

impl MessageHeader3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.req_tp { val.validate()? }
		if let Some(ref val) = self.orgnl_biz_qry { val.validate()? }
		if let Some(ref val) = self.qry_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "qry_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "qry_nm exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// MessageHeader9 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MessageHeader9 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
	pub cre_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqTp", skip_serializing_if = "Option::is_none") )]
	pub req_tp: Option<RequestType4Choice>,
}

impl MessageHeader9 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.req_tp { val.validate()? }
		Ok(())
	}
}


// MessageIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// NameAndAddress4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct NameAndAddress4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Adr") )]
	pub adr: PostalAddress1,
}

impl NameAndAddress4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
			}
		}
		self.adr.validate()?;
		Ok(())
	}
}


// NameAndAddress5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// NameAndAddress8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
		if let Some(ref val) = self.adr { val.validate()? }
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


// NamePrefix2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum NamePrefix2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DOCT") )]
	CodeDOCT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MADM") )]
	CodeMADM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MISS") )]
	CodeMISS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MIST") )]
	CodeMIST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MIKS") )]
	CodeMIKS,
}

impl NamePrefix2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// NettingCutOff2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct NettingCutOff2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetgId") )]
	pub netg_id: NettingIdentification2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NewCutOff") )]
	pub new_cut_off: Vec<CutOff1>,
}

impl NettingCutOff2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.netg_id.validate()?;
		for item in &self.new_cut_off { item.validate()? }
		Ok(())
	}
}


// NettingCutOffReportData2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct NettingCutOffReportData2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
	pub cre_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptTp") )]
	pub rpt_tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ActvtnDt") )]
	pub actvtn_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetSvcPtcptId", skip_serializing_if = "Option::is_none") )]
	pub net_svc_ptcpt_id: Option<PartyIdentification242Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptSvcr", skip_serializing_if = "Option::is_none") )]
	pub rpt_svcr: Option<PartyIdentification242Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetSvcTp", skip_serializing_if = "Option::is_none") )]
	pub net_svc_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgPgntn", skip_serializing_if = "Option::is_none") )]
	pub msg_pgntn: Option<Pagination1>,
}

impl NettingCutOffReportData2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if self.rpt_tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "rpt_tp is shorter than the minimum length of 1".to_string()));
		}
		if self.rpt_tp.chars().count() > 4 {
			return Err(ValidationError::new(1002, "rpt_tp exceeds the maximum length of 4".to_string()));
		}
		if let Some(ref val) = self.net_svc_ptcpt_id { val.validate()? }
		if let Some(ref val) = self.rpt_svcr { val.validate()? }
		if let Some(ref val) = self.net_svc_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "net_svc_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "net_svc_tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.msg_pgntn { val.validate()? }
		Ok(())
	}
}


// NettingIdentification2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct NettingIdentification2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradPty", skip_serializing_if = "Option::is_none") )]
	pub trad_pty: Option<PartyIdentification242Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetgGrpId", skip_serializing_if = "Option::is_none") )]
	pub netg_grp_id: Option<String>,
}

impl NettingIdentification2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.trad_pty { val.validate()? }
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


// NoCriteria1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum NoCriteria1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NOCR") )]
	CodeNOCR,
}

impl NoCriteria1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// NoReasonCode ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum NoReasonCode {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NORE") )]
	CodeNORE,
}

impl NoReasonCode {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// NotionalOrUnitBased1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// Operation1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum Operation1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "TILL") )]
	CodeTILL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ORRR") )]
	CodeORRR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ANDD") )]
	CodeANDD,
}

impl Operation1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Operator1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum Operator1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SMAL") )]
	CodeSMAL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SMEQ") )]
	CodeSMEQ,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GREA") )]
	CodeGREA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GREQ") )]
	CodeGREQ,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EQAL") )]
	CodeEQAL,
}

impl Operator1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Option15 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Option15 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OptnSttlmStyle", skip_serializing_if = "Option::is_none") )]
	pub optn_sttlm_style: Option<SettleStyle2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ConvsDt", skip_serializing_if = "Option::is_none") )]
	pub convs_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StrkPric", skip_serializing_if = "Option::is_none") )]
	pub strk_pric: Option<Price8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinExrcblQty", skip_serializing_if = "Option::is_none") )]
	pub min_exrcbl_qty: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ConvsPrd", skip_serializing_if = "Option::is_none") )]
	pub convs_prd: Option<DateTimePeriod1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OptnStyle", skip_serializing_if = "Option::is_none") )]
	pub optn_style: Option<OptionStyle1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OptnTp", skip_serializing_if = "Option::is_none") )]
	pub optn_tp: Option<OptionType8Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StrkVal", skip_serializing_if = "Option::is_none") )]
	pub strk_val: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StrkMltplr", skip_serializing_if = "Option::is_none") )]
	pub strk_mltplr: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrmAssgnmtMtd", skip_serializing_if = "Option::is_none") )]
	pub instrm_assgnmt_mtd: Option<AssignmentMethod2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VrsnNb", skip_serializing_if = "Option::is_none") )]
	pub vrsn_nb: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XpryLctn", skip_serializing_if = "Option::is_none") )]
	pub xpry_lctn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Stdstn", skip_serializing_if = "Option::is_none") )]
	pub stdstn: Option<Standardisation3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgPtyRole", skip_serializing_if = "Option::is_none") )]
	pub tradg_pty_role: Option<OptionParty3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctSz", skip_serializing_if = "Option::is_none") )]
	pub ctrct_sz: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlUndrlygAttrbts", skip_serializing_if = "Option::is_none") )]
	pub addtl_undrlyg_attrbts: Option<Vec<UnderlyingAttributes4>>,
}

impl Option15 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.optn_sttlm_style { val.validate()? }
		if let Some(ref val) = self.strk_pric { val.validate()? }
		if let Some(ref val) = self.min_exrcbl_qty { val.validate()? }
		if let Some(ref val) = self.convs_prd { val.validate()? }
		if let Some(ref val) = self.optn_style { val.validate()? }
		if let Some(ref val) = self.optn_tp { val.validate()? }
		if let Some(ref val) = self.instrm_assgnmt_mtd { val.validate()? }
		if let Some(ref val) = self.xpry_lctn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "xpry_lctn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "xpry_lctn exceeds the maximum length of 4".to_string()));
			}
			let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "xpry_lctn does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.stdstn { val.validate()? }
		if let Some(ref val) = self.tradg_pty_role { val.validate()? }
		if let Some(ref vec) = self.addtl_undrlyg_attrbts { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OptionParty1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum OptionParty1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SLLR") )]
	CodeSLLR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BYER") )]
	CodeBYER,
}

impl OptionParty1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OptionParty3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OptionParty3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<Vec<OptionParty1Code>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl OptionParty3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.cd { for item in vec { item.validate()? } }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// OptionStyle1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OptionStyle1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<OptionStyle1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification13>,
}

impl OptionStyle1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// OptionStyle1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum OptionStyle1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "AMER") )]
	CodeAMER,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EURO") )]
	CodeEURO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BERM") )]
	CodeBERM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ASIA") )]
	CodeASIA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CANA") )]
	CodeCANA,
}

impl OptionStyle1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OptionType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum OptionType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CALL") )]
	CodeCALL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUTO") )]
	CodePUTO,
}

impl OptionType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OptionType8Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OptionType8Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<Vec<OptionType1Code>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl OptionType8Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.cd { for item in vec { item.validate()? } }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// OrderDesk1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// Organisation38 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Organisation38 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<PartyIdentification177Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
	pub purp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxtnCtry", skip_serializing_if = "Option::is_none") )]
	pub taxtn_ctry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegnCtry", skip_serializing_if = "Option::is_none") )]
	pub regn_ctry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegnDt", skip_serializing_if = "Option::is_none") )]
	pub regn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxIdNb", skip_serializing_if = "Option::is_none") )]
	pub tax_id_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NtlRegnNb", skip_serializing_if = "Option::is_none") )]
	pub ntl_regn_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr") )]
	pub pstl_adr: Vec<PostalAddress3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmryComAdr", skip_serializing_if = "Option::is_none") )]
	pub pmry_com_adr: Option<CommunicationAddress3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ScndryComAdr", skip_serializing_if = "Option::is_none") )]
	pub scndry_com_adr: Option<CommunicationAddress3>,
}

impl Organisation38 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 140 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
		}
		if let Some(ref val) = self.id { val.validate()? }
		if let Some(ref val) = self.purp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "purp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "purp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.taxtn_ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "taxtn_ctry does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.regn_ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "regn_ctry does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.tax_id_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tax_id_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tax_id_nb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ntl_regn_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ntl_regn_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ntl_regn_nb exceeds the maximum length of 35".to_string()));
			}
		}
		for item in &self.pstl_adr { item.validate()? }
		if let Some(ref val) = self.pmry_com_adr { val.validate()? }
		if let Some(ref val) = self.scndry_com_adr { val.validate()? }
		Ok(())
	}
}


// OrganisationIdentification40 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OrganisationIdentification40 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
	pub any_bic: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none") )]
	pub email_adr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<GenericOrganisationIdentification3>>,
}

impl OrganisationIdentification40 {
	pub fn validate(&self) -> Result<(), ValidationError> {
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
		if let Some(ref val) = self.email_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "email_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "email_adr exceeds the maximum length of 256".to_string()));
			}
		}
		if let Some(ref vec) = self.othr { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OrganisationIdentificationSchemeName1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OrganisationIdentificationSchemeName1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl OrganisationIdentificationSchemeName1Choice {
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


// OrganisationType2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OrganisationType2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
	pub any_bic: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none") )]
	pub email_adr: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<GenericOrganisationType1>>,
}

impl OrganisationType2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.othr { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OriginalActivation3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OriginalActivation3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlDbtrId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_dbtr_id: Option<Party53Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlActvtnData", skip_serializing_if = "Option::is_none") )]
	pub orgnl_actvtn_data: Option<DebtorActivation5>,
}

impl OriginalActivation3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_dbtr_id { val.validate()? }
		if let Some(ref val) = self.orgnl_actvtn_data { val.validate()? }
		Ok(())
	}
}


// OriginalBusinessInstruction1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OriginalBusinessInstruction1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none") )]
	pub msg_nm_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
	pub cre_dt_tm: Option<String>,
}

impl OriginalBusinessInstruction1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.msg_nm_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "msg_nm_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "msg_nm_id exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// OriginalBusinessQuery1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OriginalBusinessQuery1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none") )]
	pub msg_nm_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
	pub cre_dt_tm: Option<String>,
}

impl OriginalBusinessQuery1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.msg_nm_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "msg_nm_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "msg_nm_id exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// OriginalEnrolment3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OriginalEnrolment3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCdtrId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_cdtr_id: Option<Party53Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlEnrlmntData", skip_serializing_if = "Option::is_none") )]
	pub orgnl_enrlmnt_data: Option<CreditorEnrolment5>,
}

impl OriginalEnrolment3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_cdtr_id { val.validate()? }
		if let Some(ref val) = self.orgnl_enrlmnt_data { val.validate()? }
		Ok(())
	}
}


// OtherContact1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OtherContact1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChanlTp") )]
	pub chanl_tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
}

impl OtherContact1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.chanl_tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "chanl_tp is shorter than the minimum length of 1".to_string()));
		}
		if self.chanl_tp.chars().count() > 4 {
			return Err(ValidationError::new(1002, "chanl_tp exceeds the maximum length of 4".to_string()));
		}
		if let Some(ref val) = self.id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 128 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 128".to_string()));
			}
		}
		Ok(())
	}
}


// OtherDistributionStrategy1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// Pagination ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// Pagination1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// Party53Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Party53Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgId", skip_serializing_if = "Option::is_none") )]
	pub org_id: Option<OrganisationIdentification40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvtId", skip_serializing_if = "Option::is_none") )]
	pub prvt_id: Option<PersonIdentification20>,
}

impl Party53Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.org_id { val.validate()? }
		if let Some(ref val) = self.prvt_id { val.validate()? }
		Ok(())
	}
}


// PartyAuditTrail2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyAuditTrail2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rcrd") )]
	pub rcrd: Vec<UpdateLogPartyRecord2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OprTmStmp") )]
	pub opr_tm_stmp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstgUsr") )]
	pub instg_usr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ApprvgUsr", skip_serializing_if = "Option::is_none") )]
	pub apprvg_usr: Option<String>,
}

impl PartyAuditTrail2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.rcrd { item.validate()? }
		if self.instg_usr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "instg_usr is shorter than the minimum length of 1".to_string()));
		}
		if self.instg_usr.chars().count() > 256 {
			return Err(ValidationError::new(1002, "instg_usr exceeds the maximum length of 256".to_string()));
		}
		if let Some(ref val) = self.apprvg_usr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "apprvg_usr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "apprvg_usr exceeds the maximum length of 256".to_string()));
			}
		}
		Ok(())
	}
}


// PartyAuditTrailOrError3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyAuditTrailOrError3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyAudtTrlRpt", skip_serializing_if = "Option::is_none") )]
	pub pty_audt_trl_rpt: Option<Vec<PartyAuditTrailReport4>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OprlErr", skip_serializing_if = "Option::is_none") )]
	pub oprl_err: Option<Vec<ErrorHandling5>>,
}

impl PartyAuditTrailOrError3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.pty_audt_trl_rpt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.oprl_err { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PartyAuditTrailOrError4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyAuditTrailOrError4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AudtTrl", skip_serializing_if = "Option::is_none") )]
	pub audt_trl: Option<Vec<PartyAuditTrail2>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BizErr", skip_serializing_if = "Option::is_none") )]
	pub biz_err: Option<Vec<ErrorHandling5>>,
}

impl PartyAuditTrailOrError4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.audt_trl { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.biz_err { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PartyAuditTrailReport4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyAuditTrailReport4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyAudtTrlOrErr") )]
	pub pty_audt_trl_or_err: PartyAuditTrailOrError4Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtPrd", skip_serializing_if = "Option::is_none") )]
	pub dt_prd: Option<DatePeriod3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyId") )]
	pub pty_id: SystemPartyIdentification8,
}

impl PartyAuditTrailReport4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pty_audt_trl_or_err.validate()?;
		if let Some(ref val) = self.dt_prd { val.validate()? }
		self.pty_id.validate()?;
		Ok(())
	}
}


// PartyAuditTrailSearchCriteria2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyAuditTrailSearchCriteria2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyId", skip_serializing_if = "Option::is_none") )]
	pub pty_id: Option<SystemPartyIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtPrd", skip_serializing_if = "Option::is_none") )]
	pub dt_prd: Option<DatePeriodSearch1Choice>,
}

impl PartyAuditTrailSearchCriteria2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pty_id { val.validate()? }
		if let Some(ref val) = self.dt_prd { val.validate()? }
		Ok(())
	}
}


// PartyDataReturnCriteria2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyDataReturnCriteria2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OpngDt", skip_serializing_if = "Option::is_none") )]
	pub opng_dt: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none") )]
	pub clsg_dt: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyId", skip_serializing_if = "Option::is_none") )]
	pub pty_id: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RspnsblPtyId", skip_serializing_if = "Option::is_none") )]
	pub rspnsbl_pty_id: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RstrctnId", skip_serializing_if = "Option::is_none") )]
	pub rstrctn_id: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RstrctdOnDt", skip_serializing_if = "Option::is_none") )]
	pub rstrctd_on_dt: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none") )]
	pub shrt_nm: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Adr", skip_serializing_if = "Option::is_none") )]
	pub adr: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechAdr", skip_serializing_if = "Option::is_none") )]
	pub tech_adr: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none") )]
	pub ctct_dtls: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ResTp", skip_serializing_if = "Option::is_none") )]
	pub res_tp: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LckSts", skip_serializing_if = "Option::is_none") )]
	pub lck_sts: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktSpcfcAttr", skip_serializing_if = "Option::is_none") )]
	pub mkt_spcfc_attr: Option<bool>,
}

impl PartyDataReturnCriteria2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PartyDataSearchCriteria2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyDataSearchCriteria2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OpngDt", skip_serializing_if = "Option::is_none") )]
	pub opng_dt: Option<DatePeriodSearch1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none") )]
	pub clsg_dt: Option<DatePeriodSearch1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<SystemPartyType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RspnsblPtyId", skip_serializing_if = "Option::is_none") )]
	pub rspnsbl_pty_id: Option<PartyIdentification136>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyId", skip_serializing_if = "Option::is_none") )]
	pub pty_id: Option<PartyIdentification136>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RstrctnId", skip_serializing_if = "Option::is_none") )]
	pub rstrctn_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RstrctnIsseDt", skip_serializing_if = "Option::is_none") )]
	pub rstrctn_isse_dt: Option<DateAndDateTimeSearch4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ResTp", skip_serializing_if = "Option::is_none") )]
	pub res_tp: Option<ResidenceType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LckSts", skip_serializing_if = "Option::is_none") )]
	pub lck_sts: Option<PartyLockStatus1>,
}

impl PartyDataSearchCriteria2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.opng_dt { val.validate()? }
		if let Some(ref val) = self.clsg_dt { val.validate()? }
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.rspnsbl_pty_id { val.validate()? }
		if let Some(ref val) = self.pty_id { val.validate()? }
		if let Some(ref val) = self.rstrctn_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rstrctn_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "rstrctn_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.rstrctn_isse_dt { val.validate()? }
		if let Some(ref val) = self.res_tp { val.validate()? }
		if let Some(ref val) = self.lck_sts { val.validate()? }
		Ok(())
	}
}


// PartyIdentification120Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyIdentification120Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
	pub any_bic: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
	pub prtry_id: Option<GenericIdentification36>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
	pub nm_and_adr: Option<NameAndAddress5>,
}

impl PartyIdentification120Choice {
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


// PartyIdentification125Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// PartyIdentification136 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyIdentification136 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: PartyIdentification120Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
}

impl PartyIdentification136 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// PartyIdentification139 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// PartyIdentification177Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyIdentification177Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
	pub any_bic: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
	pub prtry_id: Option<GenericIdentification1>,
}

impl PartyIdentification177Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.any_bic {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prtry_id { val.validate()? }
		Ok(())
	}
}


// PartyIdentification242Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
		if let Some(ref val) = self.nm_and_adr { val.validate()? }
		if let Some(ref val) = self.any_bic { val.validate()? }
		if let Some(ref val) = self.pty_id { val.validate()? }
		Ok(())
	}
}


// PartyIdentification265 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
		if let Some(ref val) = self.any_bic { val.validate()? }
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
		if let Some(ref val) = self.clr_sys_id { val.validate()? }
		if let Some(ref val) = self.lgl_ntty_idr {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lgl_ntty_idr does not match the required pattern".to_string()));
			}
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// PartyIdentification44 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyIdentification44 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC") )]
	pub any_bic: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AltrntvIdr", skip_serializing_if = "Option::is_none") )]
	pub altrntv_idr: Option<Vec<String>>,
}

impl PartyIdentification44 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}").unwrap();
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


// PartyIdentification62 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyIdentification62 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "BICFI", skip_serializing_if = "Option::is_none") )]
	pub bicfi: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
	pub prtry_id: Option<GenericIdentification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
	pub nm_and_adr: Option<NameAndAddress5>,
}

impl PartyIdentification62 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.bicfi {
			let pattern = Regex::new("[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "bicfi does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prtry_id { val.validate()? }
		if let Some(ref val) = self.nm_and_adr { val.validate()? }
		Ok(())
	}
}


// PartyIdentification63 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyIdentification63 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyId") )]
	pub pty_id: PartyIdentification75Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrcgId", skip_serializing_if = "Option::is_none") )]
	pub prcg_id: Option<String>,
}

impl PartyIdentification63 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pty_id.validate()?;
		if let Some(ref val) = self.prcg_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prcg_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prcg_id exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// PartyIdentification64 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyIdentification64 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
	pub any_bic: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
	pub prtry_id: Option<GenericIdentification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
	pub nm_and_adr: Option<NameAndAddress5>,
}

impl PartyIdentification64 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.any_bic {
			let pattern = Regex::new("[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prtry_id { val.validate()? }
		if let Some(ref val) = self.nm_and_adr { val.validate()? }
		Ok(())
	}
}


// PartyIdentification71Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyIdentification71Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
	pub any_bic: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
	pub prtry_id: Option<GenericIdentification36>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
	pub nm_and_adr: Option<NameAndAddress5>,
}

impl PartyIdentification71Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.any_bic {
			let pattern = Regex::new("[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prtry_id { val.validate()? }
		if let Some(ref val) = self.nm_and_adr { val.validate()? }
		Ok(())
	}
}


// PartyIdentification75Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyIdentification75Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
	pub any_bic: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
	pub nm_and_adr: Option<NameAndAddress5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
	pub ctry: Option<String>,
}

impl PartyIdentification75Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.any_bic {
			let pattern = Regex::new("[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.nm_and_adr { val.validate()? }
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// PartyIdentification99Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyIdentification99Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
	pub nm_and_adr: Option<NameAndAddress8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
	pub any_bic: Option<PartyIdentification44>,
}

impl PartyIdentification99Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm_and_adr { val.validate()? }
		if let Some(ref val) = self.any_bic { val.validate()? }
		Ok(())
	}
}


// PartyIdentificationAndAccount95 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyIdentificationAndAccount95 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyId") )]
	pub pty_id: PartyIdentification71Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctId", skip_serializing_if = "Option::is_none") )]
	pub acct_id: Option<SecuritiesAccount22>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrcgId", skip_serializing_if = "Option::is_none") )]
	pub prcg_id: Option<String>,
}

impl PartyIdentificationAndAccount95 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pty_id.validate()?;
		if let Some(ref val) = self.acct_id { val.validate()? }
		if let Some(ref val) = self.prcg_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prcg_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prcg_id exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// PartyIdentificationAndAccount96 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyIdentificationAndAccount96 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyId") )]
	pub pty_id: PartyIdentification64,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctId") )]
	pub acct_id: AccountIdentification26,
}

impl PartyIdentificationAndAccount96 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pty_id.validate()?;
		self.acct_id.validate()?;
		Ok(())
	}
}


// PartyIdentificationAndAccount97 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyIdentificationAndAccount97 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyId") )]
	pub pty_id: PartyIdentification62,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctId", skip_serializing_if = "Option::is_none") )]
	pub acct_id: Option<AccountIdentification26>,
}

impl PartyIdentificationAndAccount97 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pty_id.validate()?;
		if let Some(ref val) = self.acct_id { val.validate()? }
		Ok(())
	}
}


// PartyLockStatus1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyLockStatus1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldFr", skip_serializing_if = "Option::is_none") )]
	pub vld_fr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts") )]
	pub sts: LockStatus1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LckRsn", skip_serializing_if = "Option::is_none") )]
	pub lck_rsn: Option<Vec<String>>,
}

impl PartyLockStatus1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.sts.validate()?;
		if let Some(ref vec) = self.lck_rsn {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "lck_rsn is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 35 {
					return Err(ValidationError::new(1002, "lck_rsn exceeds the maximum length of 35".to_string()));
				}
			}
		}
		Ok(())
	}
}


// PartyName3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyName3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldFr", skip_serializing_if = "Option::is_none") )]
	pub vld_fr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none") )]
	pub shrt_nm: Option<String>,
}

impl PartyName3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.shrt_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "shrt_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "shrt_nm exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// PartyName4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyName4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldFr", skip_serializing_if = "Option::is_none") )]
	pub vld_fr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none") )]
	pub shrt_nm: Option<String>,
}

impl PartyName4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
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
		Ok(())
	}
}


// PartyOrBusinessError4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyOrBusinessError4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SysPty", skip_serializing_if = "Option::is_none") )]
	pub sys_pty: Option<SystemParty6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BizErr", skip_serializing_if = "Option::is_none") )]
	pub biz_err: Option<Vec<ErrorHandling4>>,
}

impl PartyOrBusinessError4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.sys_pty { val.validate()? }
		if let Some(ref vec) = self.biz_err { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PartyOrCurrency1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyOrCurrency1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dpstry", skip_serializing_if = "Option::is_none") )]
	pub dpstry: Option<PartyIdentification63>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none") )]
	pub sttlm_ccy: Option<String>,
}

impl PartyOrCurrency1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dpstry { val.validate()? }
		if let Some(ref val) = self.sttlm_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "sttlm_ccy does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// PartyOrOperationalError4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyOrOperationalError4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyRpt", skip_serializing_if = "Option::is_none") )]
	pub pty_rpt: Option<Vec<PartyReport4>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OprlErr", skip_serializing_if = "Option::is_none") )]
	pub oprl_err: Option<Vec<ErrorHandling5>>,
}

impl PartyOrOperationalError4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.pty_rpt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.oprl_err { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PartyReferenceDataChange3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyReferenceDataChange3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyId") )]
	pub pty_id: SystemPartyIdentification8,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rcrd") )]
	pub rcrd: Vec<UpdateLogPartyRecord2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OprTmStmp") )]
	pub opr_tm_stmp: String,
}

impl PartyReferenceDataChange3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pty_id.validate()?;
		for item in &self.rcrd { item.validate()? }
		Ok(())
	}
}


// PartyReport4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyReport4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyId") )]
	pub pty_id: SystemPartyIdentification8,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyOrErr") )]
	pub pty_or_err: PartyOrBusinessError4Choice,
}

impl PartyReport4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pty_id.validate()?;
		self.pty_or_err.validate()?;
		Ok(())
	}
}


// PartyStatement3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyStatement3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SysDt") )]
	pub sys_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Chng", skip_serializing_if = "Option::is_none") )]
	pub chng: Option<Vec<PartyReferenceDataChange3>>,
}

impl PartyStatement3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.chng { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PartyStatus2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyStatus2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts") )]
	pub sts: Status6Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsRsn", skip_serializing_if = "Option::is_none") )]
	pub sts_rsn: Option<Vec<StatusReasonInformation10>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SysPtyId", skip_serializing_if = "Option::is_none") )]
	pub sys_pty_id: Option<SystemPartyIdentification8>,
}

impl PartyStatus2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.sts.validate()?;
		if let Some(ref vec) = self.sts_rsn { for item in vec { item.validate()? } }
		if let Some(ref val) = self.sys_pty_id { val.validate()? }
		Ok(())
	}
}


// PaymentInstrument16 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// PendingProcessingReason8Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PendingProcessingReason8Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification36>,
}

impl PendingProcessingReason8Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// PendingProcessingReason9Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PendingProcessingReason9Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<Vec<PendingProcessingReason8Choice>>,
}

impl PendingProcessingReason9Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.no_spcfd_rsn { val.validate()? }
		if let Some(ref vec) = self.rsn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PendingProcessingStatusReason1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PendingProcessingStatusReason1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn") )]
	pub rsn: PendingProcessingReason9Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_rsn_inf: Option<String>,
}

impl PendingProcessingStatusReason1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rsn.validate()?;
		if let Some(ref val) = self.addtl_rsn_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_rsn_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 210 {
				return Err(ValidationError::new(1002, "addtl_rsn_inf exceeds the maximum length of 210".to_string()));
			}
		}
		Ok(())
	}
}


// PerformanceFactors1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// Period15 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// PersonIdentification20 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PersonIdentification20 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none") )]
	pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none") )]
	pub email_adr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<GenericPersonIdentification2>>,
}

impl PersonIdentification20 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dt_and_plc_of_birth { val.validate()? }
		if let Some(ref val) = self.email_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "email_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "email_adr exceeds the maximum length of 256".to_string()));
			}
		}
		if let Some(ref vec) = self.othr { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PersonIdentificationSchemeName1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PersonIdentificationSchemeName1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl PersonIdentificationSchemeName1Choice {
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


// PersonType2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PersonType2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none") )]
	pub dt_and_plc_of_birth: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none") )]
	pub email_adr: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<GenericPersonType1>>,
}

impl PersonType2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.othr { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PostalAddress1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// PostalAddress27 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PostalAddress27 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdrTp", skip_serializing_if = "Option::is_none") )]
	pub adr_tp: Option<AddressType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CareOf", skip_serializing_if = "Option::is_none") )]
	pub care_of: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dept", skip_serializing_if = "Option::is_none") )]
	pub dept: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubDept", skip_serializing_if = "Option::is_none") )]
	pub sub_dept: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StrtNm", skip_serializing_if = "Option::is_none") )]
	pub strt_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNb", skip_serializing_if = "Option::is_none") )]
	pub bldg_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNm", skip_serializing_if = "Option::is_none") )]
	pub bldg_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Flr", skip_serializing_if = "Option::is_none") )]
	pub flr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitNb", skip_serializing_if = "Option::is_none") )]
	pub unit_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstBx", skip_serializing_if = "Option::is_none") )]
	pub pst_bx: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Room", skip_serializing_if = "Option::is_none") )]
	pub room: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstCd", skip_serializing_if = "Option::is_none") )]
	pub pst_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TwnNm", skip_serializing_if = "Option::is_none") )]
	pub twn_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TwnLctnNm", skip_serializing_if = "Option::is_none") )]
	pub twn_lctn_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none") )]
	pub dstrct_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none") )]
	pub ctry_sub_dvsn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
	pub ctry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdrLine", skip_serializing_if = "Option::is_none") )]
	pub adr_line: Option<Vec<String>>,
}

impl PostalAddress27 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.adr_tp { val.validate()? }
		if let Some(ref val) = self.care_of {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "care_of is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "care_of exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.dept {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dept is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "dept exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.sub_dept {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sub_dept is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "sub_dept exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.strt_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "strt_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "strt_nm exceeds the maximum length of 140".to_string()));
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
		if let Some(ref val) = self.bldg_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "bldg_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "bldg_nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.flr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "flr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "flr exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.unit_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "unit_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "unit_nb exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.pst_bx {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pst_bx is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "pst_bx exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.room {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "room is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "room exceeds the maximum length of 70".to_string()));
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
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "twn_nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.twn_lctn_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "twn_lctn_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "twn_lctn_nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.dstrct_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dstrct_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "dstrct_nm exceeds the maximum length of 140".to_string()));
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
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
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
		Ok(())
	}
}


// PostalAddress28 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PostalAddress28 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdrTp", skip_serializing_if = "Option::is_none") )]
	pub adr_tp: Option<AddressType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CareOf", skip_serializing_if = "Option::is_none") )]
	pub care_of: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dept", skip_serializing_if = "Option::is_none") )]
	pub dept: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubDept", skip_serializing_if = "Option::is_none") )]
	pub sub_dept: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StrtNm", skip_serializing_if = "Option::is_none") )]
	pub strt_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNb", skip_serializing_if = "Option::is_none") )]
	pub bldg_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNm", skip_serializing_if = "Option::is_none") )]
	pub bldg_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Flr", skip_serializing_if = "Option::is_none") )]
	pub flr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitNb", skip_serializing_if = "Option::is_none") )]
	pub unit_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstBx", skip_serializing_if = "Option::is_none") )]
	pub pst_bx: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Room", skip_serializing_if = "Option::is_none") )]
	pub room: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstCd", skip_serializing_if = "Option::is_none") )]
	pub pst_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TwnNm", skip_serializing_if = "Option::is_none") )]
	pub twn_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TwnLctnNm", skip_serializing_if = "Option::is_none") )]
	pub twn_lctn_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none") )]
	pub dstrct_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none") )]
	pub ctry_sub_dvsn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
	pub ctry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdrLine", skip_serializing_if = "Option::is_none") )]
	pub adr_line: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldFr", skip_serializing_if = "Option::is_none") )]
	pub vld_fr: Option<String>,
}

impl PostalAddress28 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.adr_tp { val.validate()? }
		if let Some(ref val) = self.care_of {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "care_of is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "care_of exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.dept {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dept is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "dept exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.sub_dept {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sub_dept is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "sub_dept exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.strt_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "strt_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "strt_nm exceeds the maximum length of 140".to_string()));
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
		if let Some(ref val) = self.bldg_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "bldg_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "bldg_nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.flr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "flr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "flr exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.unit_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "unit_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "unit_nb exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.pst_bx {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pst_bx is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "pst_bx exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.room {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "room is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "room exceeds the maximum length of 70".to_string()));
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
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "twn_nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.twn_lctn_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "twn_lctn_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "twn_lctn_nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.dstrct_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dstrct_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "dstrct_nm exceeds the maximum length of 140".to_string()));
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
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
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
		Ok(())
	}
}


// PostalAddress3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PostalAddress3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdrTp") )]
	pub adr_tp: AddressType1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MlngInd") )]
	pub mlng_ind: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegnAdrInd") )]
	pub regn_adr_ind: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr") )]
	pub nm_and_adr: NameAndAddress4,
}

impl PostalAddress3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.adr_tp.validate()?;
		self.nm_and_adr.validate()?;
		Ok(())
	}
}


// PreferenceToIncome1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum PreferenceToIncome1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ORDN") )]
	CodeORDN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PFRD") )]
	CodePFRD,
}

impl PreferenceToIncome1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PreferenceToIncome5Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PreferenceToIncome5Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<PreferenceToIncome1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl PreferenceToIncome5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// PreferredContactMethod2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum PreferredContactMethod2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MAIL") )]
	CodeMAIL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FAXX") )]
	CodeFAXX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LETT") )]
	CodeLETT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CELL") )]
	CodeCELL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ONLI") )]
	CodeONLI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PHON") )]
	CodePHON,
}

impl PreferredContactMethod2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PresentmentType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum PresentmentType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FULL") )]
	CodeFULL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PAYD") )]
	CodePAYD,
}

impl PresentmentType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Price8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Price8 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValTp", skip_serializing_if = "Option::is_none") )]
	pub val_tp: Option<PriceValueType3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
	pub val: PriceRateOrAmount3Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricTp", skip_serializing_if = "Option::is_none") )]
	pub pric_tp: Option<TypeOfPrice1Code>,
}

impl Price8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.val_tp { val.validate()? }
		self.val.validate()?;
		if let Some(ref val) = self.pric_tp { val.validate()? }
		Ok(())
	}
}


// PriceMethod1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// PriceRateOrAmount3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PriceRateOrAmount3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
	pub rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
}

impl PriceRateOrAmount3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.amt { val.validate()? }
		Ok(())
	}
}


// PriceReport3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// PriceReportFunction1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum PriceReportFunction1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "REPL") )]
	CodeREPL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NEWP") )]
	CodeNEWP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PART") )]
	CodePART,
}

impl PriceReportFunction1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PriceType2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// PriceValueType3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum PriceValueType3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DISC") )]
	CodeDISC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PREM") )]
	CodePREM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PARV") )]
	CodePARV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YIEL") )]
	CodeYIEL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SPRE") )]
	CodeSPRE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PEUN") )]
	CodePEUN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ABSO") )]
	CodeABSO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TEDP") )]
	CodeTEDP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TEDY") )]
	CodeTEDY,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FICT") )]
	CodeFICT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VACT") )]
	CodeVACT,
}

impl PriceValueType3Code {
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// ProcessingStatus43Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ProcessingStatus43Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rcvd", skip_serializing_if = "Option::is_none") )]
	pub rcvd: Option<ReceivedStatusReason1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Accptd", skip_serializing_if = "Option::is_none") )]
	pub accptd: Option<AcceptedStatusReason7>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PdgPrcg", skip_serializing_if = "Option::is_none") )]
	pub pdg_prcg: Option<PendingProcessingStatusReason1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rjctd", skip_serializing_if = "Option::is_none") )]
	pub rjctd: Option<RejectedStatusReason12>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtrySts", skip_serializing_if = "Option::is_none") )]
	pub prtry_sts: Option<ProprietaryStatusAndReason5>,
}

impl ProcessingStatus43Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rcvd { val.validate()? }
		if let Some(ref val) = self.accptd { val.validate()? }
		if let Some(ref val) = self.pdg_prcg { val.validate()? }
		if let Some(ref val) = self.rjctd { val.validate()? }
		if let Some(ref val) = self.prtry_sts { val.validate()? }
		Ok(())
	}
}


// ProcessingStatus72Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ProcessingStatus72Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AckdAccptd", skip_serializing_if = "Option::is_none") )]
	pub ackd_accptd: Option<Reason4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PdgPrcg", skip_serializing_if = "Option::is_none") )]
	pub pdg_prcg: Option<Reason18Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rjctd", skip_serializing_if = "Option::is_none") )]
	pub rjctd: Option<Reason18Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cmpltd", skip_serializing_if = "Option::is_none") )]
	pub cmpltd: Option<Reason4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<ProprietaryStatusAndReason6>,
}

impl ProcessingStatus72Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ackd_accptd { val.validate()? }
		if let Some(ref val) = self.pdg_prcg { val.validate()? }
		if let Some(ref val) = self.rjctd { val.validate()? }
		if let Some(ref val) = self.cmpltd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// ProductStructure1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// ProprietaryReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ProprietaryReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<Vec<GenericIdentification36>>,
}

impl ProprietaryReason1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.no_spcfd_rsn { val.validate()? }
		if let Some(ref vec) = self.rsn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// ProprietaryReason4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ProprietaryReason4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<GenericIdentification30>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_rsn_inf: Option<String>,
}

impl ProprietaryReason4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rsn { val.validate()? }
		if let Some(ref val) = self.addtl_rsn_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_rsn_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 210 {
				return Err(ValidationError::new(1002, "addtl_rsn_inf exceeds the maximum length of 210".to_string()));
			}
		}
		Ok(())
	}
}


// ProprietaryStatusAndReason5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ProprietaryStatusAndReason5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts") )]
	pub sts: GenericIdentification36,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn") )]
	pub rsn: ProprietaryReason1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_rsn_inf: Option<String>,
}

impl ProprietaryStatusAndReason5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.sts.validate()?;
		self.rsn.validate()?;
		if let Some(ref val) = self.addtl_rsn_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_rsn_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 210 {
				return Err(ValidationError::new(1002, "addtl_rsn_inf exceeds the maximum length of 210".to_string()));
			}
		}
		Ok(())
	}
}


// ProprietaryStatusAndReason6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ProprietaryStatusAndReason6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtrySts") )]
	pub prtry_sts: GenericIdentification30,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryRsn", skip_serializing_if = "Option::is_none") )]
	pub prtry_rsn: Option<Vec<ProprietaryReason4>>,
}

impl ProprietaryStatusAndReason6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.prtry_sts.validate()?;
		if let Some(ref vec) = self.prtry_rsn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// Purpose3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Purpose3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesPurpCd", skip_serializing_if = "Option::is_none") )]
	pub scties_purp_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification1>,
}

impl Purpose3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.scties_purp_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "scties_purp_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "scties_purp_cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// PutType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum PutType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MAND") )]
	CodeMAND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OPTI") )]
	CodeOPTI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TWOS") )]
	CodeTWOS,
}

impl PutType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PutType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PutType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<PutType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl PutType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// QuotationType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// RTPPartyIdentification2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RTPPartyIdentification2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
	pub pstl_adr: Option<PostalAddress27>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<Party53Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none") )]
	pub ctry_of_res: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none") )]
	pub ctct_dtls: Option<Contact13>,
}

impl RTPPartyIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.pstl_adr { val.validate()? }
		if let Some(ref val) = self.id { val.validate()? }
		if let Some(ref val) = self.ctry_of_res {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry_of_res does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ctct_dtls { val.validate()? }
		Ok(())
	}
}


// RateAndAmountFormat1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RateAndAmountFormat1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
	pub rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none") )]
	pub not_spcfd_rate: Option<RateType12FormatChoice>,
}

impl RateAndAmountFormat1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.amt { val.validate()? }
		if let Some(ref val) = self.not_spcfd_rate { val.validate()? }
		Ok(())
	}
}


// RateOrAbsoluteValue1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RateOrAbsoluteValue1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RateVal", skip_serializing_if = "Option::is_none") )]
	pub rate_val: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AbsVal", skip_serializing_if = "Option::is_none") )]
	pub abs_val: Option<f64>,
}

impl RateOrAbsoluteValue1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RateType12Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum RateType12Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "OPEN") )]
	CodeOPEN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UKWN") )]
	CodeUKWN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NILP") )]
	CodeNILP,
}

impl RateType12Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RateType12FormatChoice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RateType12FormatChoice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<RateType12Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification13>,
}

impl RateType12FormatChoice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// Reason18Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Reason18Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<Vec<ProprietaryReason4>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
	pub no_spcfd_rsn: Option<NoReasonCode>,
}

impl Reason18Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.rsn { for item in vec { item.validate()? } }
		if let Some(ref val) = self.no_spcfd_rsn { val.validate()? }
		Ok(())
	}
}


// Reason4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Reason4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<Vec<ProprietaryReason4>>,
}

impl Reason4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.rsn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// ReceivedReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ReceivedReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<ReceivedReason2Choice>,
}

impl ReceivedReason1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.no_spcfd_rsn { val.validate()? }
		if let Some(ref val) = self.rsn { val.validate()? }
		Ok(())
	}
}


// ReceivedReason2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ReceivedReason2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification36>,
}

impl ReceivedReason2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// ReceivedStatusReason1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ReceivedStatusReason1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn") )]
	pub rsn: ReceivedReason1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_rsn_inf: Option<String>,
}

impl ReceivedStatusReason1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rsn.validate()?;
		if let Some(ref val) = self.addtl_rsn_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_rsn_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 210 {
				return Err(ValidationError::new(1002, "addtl_rsn_inf exceeds the maximum length of 210".to_string()));
			}
		}
		Ok(())
	}
}


// ReferToFundOrderDesk1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// RejectedReason7Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RejectedReason7Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification36>,
}

impl RejectedReason7Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// RejectedReason8Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RejectedReason8Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<Vec<RejectedReason7Choice>>,
}

impl RejectedReason8Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.no_spcfd_rsn { val.validate()? }
		if let Some(ref vec) = self.rsn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// RejectedStatusReason12 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RejectedStatusReason12 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn") )]
	pub rsn: RejectedReason8Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_rsn_inf: Option<String>,
}

impl RejectedStatusReason12 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rsn.validate()?;
		if let Some(ref val) = self.addtl_rsn_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_rsn_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 210 {
				return Err(ValidationError::new(1002, "addtl_rsn_inf exceeds the maximum length of 210".to_string()));
			}
		}
		Ok(())
	}
}


// RequestData2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RequestData2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqTp") )]
	pub req_tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdActvtnDt") )]
	pub reqd_actvtn_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqSvcr", skip_serializing_if = "Option::is_none") )]
	pub req_svcr: Option<PartyIdentification242Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetSvcPtcptId") )]
	pub net_svc_ptcpt_id: PartyIdentification242Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetSvcTp", skip_serializing_if = "Option::is_none") )]
	pub net_svc_tp: Option<String>,
}

impl RequestData2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if self.req_tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "req_tp is shorter than the minimum length of 1".to_string()));
		}
		if self.req_tp.chars().count() > 4 {
			return Err(ValidationError::new(1002, "req_tp exceeds the maximum length of 4".to_string()));
		}
		if let Some(ref val) = self.req_svcr { val.validate()? }
		self.net_svc_ptcpt_id.validate()?;
		if let Some(ref val) = self.net_svc_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "net_svc_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "net_svc_tp exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// RequestType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum RequestType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "RT01") )]
	CodeRT01,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RT02") )]
	CodeRT02,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RT03") )]
	CodeRT03,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RT04") )]
	CodeRT04,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RT05") )]
	CodeRT05,
}

impl RequestType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RequestType2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RequestType2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtCtrl", skip_serializing_if = "Option::is_none") )]
	pub pmt_ctrl: Option<RequestType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Enqry", skip_serializing_if = "Option::is_none") )]
	pub enqry: Option<RequestType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification1>,
}

impl RequestType2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pmt_ctrl { val.validate()? }
		if let Some(ref val) = self.enqry { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// RequestType2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum RequestType2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "RT11") )]
	CodeRT11,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RT12") )]
	CodeRT12,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RT13") )]
	CodeRT13,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RT14") )]
	CodeRT14,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RT15") )]
	CodeRT15,
}

impl RequestType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RequestType4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RequestType4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtCtrl", skip_serializing_if = "Option::is_none") )]
	pub pmt_ctrl: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Enqry", skip_serializing_if = "Option::is_none") )]
	pub enqry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification1>,
}

impl RequestType4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pmt_ctrl {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pmt_ctrl is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "pmt_ctrl exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.enqry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "enqry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "enqry exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// ResidenceType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum ResidenceType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DMST") )]
	CodeDMST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FRGN") )]
	CodeFRGN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MXED") )]
	CodeMXED,
}

impl ResidenceType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Restriction1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Restriction1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RstrctnTp") )]
	pub rstrctn_tp: CodeOrProprietary1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldFr") )]
	pub vld_fr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldUntil", skip_serializing_if = "Option::is_none") )]
	pub vld_until: Option<String>,
}

impl Restriction1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rstrctn_tp.validate()?;
		Ok(())
	}
}


// RestrictionType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum RestrictionType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SELR") )]
	CodeSELR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BUYR") )]
	CodeBUYR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PLAR") )]
	CodePLAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HOLR") )]
	CodeHOLR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VOTR") )]
	CodeVOTR,
}

impl RestrictionType1Code {
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// SecuritiesAccount19 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecuritiesAccount19 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<GenericIdentification30>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
}

impl SecuritiesAccount19 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 70".to_string()));
			}
		}
		Ok(())
	}
}


// SecuritiesAccount22 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecuritiesAccount22 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<GenericIdentification30>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
}

impl SecuritiesAccount22 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 70".to_string()));
			}
		}
		Ok(())
	}
}


// SecuritiesAccountAuditTrailOrOperationalError3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecuritiesAccountAuditTrailOrOperationalError3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesAcctAudtTrlRpt", skip_serializing_if = "Option::is_none") )]
	pub scties_acct_audt_trl_rpt: Option<Vec<SecuritiesAccountAuditTrailReport3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OprlErr", skip_serializing_if = "Option::is_none") )]
	pub oprl_err: Option<Vec<ErrorHandling5>>,
}

impl SecuritiesAccountAuditTrailOrOperationalError3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.scties_acct_audt_trl_rpt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.oprl_err { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SecuritiesAccountAuditTrailReport3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecuritiesAccountAuditTrailReport3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesAcctAudtTrlOrErr") )]
	pub scties_acct_audt_trl_or_err: AuditTrailOrBusinessError6Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtPrd", skip_serializing_if = "Option::is_none") )]
	pub dt_prd: Option<DatePeriodSearch1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesAcctId") )]
	pub scties_acct_id: SecuritiesAccount19,
}

impl SecuritiesAccountAuditTrailReport3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.scties_acct_audt_trl_or_err.validate()?;
		if let Some(ref val) = self.dt_prd { val.validate()? }
		self.scties_acct_id.validate()?;
		Ok(())
	}
}


// SecuritiesAccountAuditTrailSearchCriteria3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecuritiesAccountAuditTrailSearchCriteria3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesAcctId", skip_serializing_if = "Option::is_none") )]
	pub scties_acct_id: Option<SecuritiesAccount19>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtPrd", skip_serializing_if = "Option::is_none") )]
	pub dt_prd: Option<DatePeriodSearch1Choice>,
}

impl SecuritiesAccountAuditTrailSearchCriteria3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.scties_acct_id { val.validate()? }
		if let Some(ref val) = self.dt_prd { val.validate()? }
		Ok(())
	}
}


// SecuritiesAccountModification2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecuritiesAccountModification2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ScpIndctn") )]
	pub scp_indctn: DataModification1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdMod") )]
	pub reqd_mod: SecuritiesAccountModification2Choice,
}

impl SecuritiesAccountModification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.scp_indctn.validate()?;
		self.reqd_mod.validate()?;
		Ok(())
	}
}


// SecuritiesAccountModification2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecuritiesAccountModification2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SysSctiesAcct", skip_serializing_if = "Option::is_none") )]
	pub sys_scties_acct: Option<SystemSecuritiesAccount5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SysRstrctn", skip_serializing_if = "Option::is_none") )]
	pub sys_rstrctn: Option<SystemRestriction1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktSpcfcAttr", skip_serializing_if = "Option::is_none") )]
	pub mkt_spcfc_attr: Option<MarketSpecificAttribute1>,
}

impl SecuritiesAccountModification2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.sys_scties_acct { val.validate()? }
		if let Some(ref val) = self.sys_rstrctn { val.validate()? }
		if let Some(ref val) = self.mkt_spcfc_attr { val.validate()? }
		Ok(())
	}
}


// SecuritiesAccountOrBusinessError3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecuritiesAccountOrBusinessError3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesAcct", skip_serializing_if = "Option::is_none") )]
	pub scties_acct: Option<SystemSecuritiesAccount6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BizErr", skip_serializing_if = "Option::is_none") )]
	pub biz_err: Option<Vec<ErrorHandling5>>,
}

impl SecuritiesAccountOrBusinessError3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.scties_acct { val.validate()? }
		if let Some(ref vec) = self.biz_err { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SecuritiesAccountOrOperationalError3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecuritiesAccountOrOperationalError3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesAcctRpt", skip_serializing_if = "Option::is_none") )]
	pub scties_acct_rpt: Option<Vec<SecuritiesAccountReport3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OprlErr", skip_serializing_if = "Option::is_none") )]
	pub oprl_err: Option<Vec<ErrorHandling5>>,
}

impl SecuritiesAccountOrOperationalError3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.scties_acct_rpt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.oprl_err { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SecuritiesAccountReferenceDataChange2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecuritiesAccountReferenceDataChange2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesAcctId") )]
	pub scties_acct_id: SecuritiesAccount19,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FldNm") )]
	pub fld_nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OdFldVal") )]
	pub od_fld_val: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NewFldVal") )]
	pub new_fld_val: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OprTmStmp") )]
	pub opr_tm_stmp: String,
}

impl SecuritiesAccountReferenceDataChange2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.scties_acct_id.validate()?;
		if self.fld_nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "fld_nm is shorter than the minimum length of 1".to_string()));
		}
		if self.fld_nm.chars().count() > 35 {
			return Err(ValidationError::new(1002, "fld_nm exceeds the maximum length of 35".to_string()));
		}
		if self.od_fld_val.chars().count() < 1 {
			return Err(ValidationError::new(1001, "od_fld_val is shorter than the minimum length of 1".to_string()));
		}
		if self.od_fld_val.chars().count() > 350 {
			return Err(ValidationError::new(1002, "od_fld_val exceeds the maximum length of 350".to_string()));
		}
		if self.new_fld_val.chars().count() < 1 {
			return Err(ValidationError::new(1001, "new_fld_val is shorter than the minimum length of 1".to_string()));
		}
		if self.new_fld_val.chars().count() > 350 {
			return Err(ValidationError::new(1002, "new_fld_val exceeds the maximum length of 350".to_string()));
		}
		Ok(())
	}
}


// SecuritiesAccountReport3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecuritiesAccountReport3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesAcctId") )]
	pub scties_acct_id: SecuritiesAccount19,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesAcctOrErr") )]
	pub scties_acct_or_err: SecuritiesAccountOrBusinessError3Choice,
}

impl SecuritiesAccountReport3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.scties_acct_id.validate()?;
		self.scties_acct_or_err.validate()?;
		Ok(())
	}
}


// SecuritiesAccountReturnCriteria1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecuritiesAccountReturnCriteria1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctId", skip_serializing_if = "Option::is_none") )]
	pub acct_id: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyId", skip_serializing_if = "Option::is_none") )]
	pub pty_id: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyTp", skip_serializing_if = "Option::is_none") )]
	pub pty_tp: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none") )]
	pub acct_svcr: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctTp", skip_serializing_if = "Option::is_none") )]
	pub acct_tp: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OpngDt", skip_serializing_if = "Option::is_none") )]
	pub opng_dt: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none") )]
	pub clsg_dt: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndInvstrFlg", skip_serializing_if = "Option::is_none") )]
	pub end_invstr_flg: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricgSchme", skip_serializing_if = "Option::is_none") )]
	pub pricg_schme: Option<bool>,
}

impl SecuritiesAccountReturnCriteria1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SecuritiesAccountSearchCriteria2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecuritiesAccountSearchCriteria2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctId", skip_serializing_if = "Option::is_none") )]
	pub acct_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none") )]
	pub acct_svcr: Option<PartyIdentification136>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none") )]
	pub acct_ownr: Option<SystemPartyIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyTp", skip_serializing_if = "Option::is_none") )]
	pub pty_tp: Option<SystemPartyType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OpngDt", skip_serializing_if = "Option::is_none") )]
	pub opng_dt: Option<DatePeriodSearch1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none") )]
	pub clsg_dt: Option<DatePeriodSearch1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctTp", skip_serializing_if = "Option::is_none") )]
	pub acct_tp: Option<SystemSecuritiesAccountType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndInvstrFlg", skip_serializing_if = "Option::is_none") )]
	pub end_invstr_flg: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricgSchme", skip_serializing_if = "Option::is_none") )]
	pub pricg_schme: Option<String>,
}

impl SecuritiesAccountSearchCriteria2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.acct_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acct_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.acct_svcr { val.validate()? }
		if let Some(ref val) = self.acct_ownr { val.validate()? }
		if let Some(ref val) = self.pty_tp { val.validate()? }
		if let Some(ref val) = self.opng_dt { val.validate()? }
		if let Some(ref val) = self.clsg_dt { val.validate()? }
		if let Some(ref val) = self.acct_tp { val.validate()? }
		if let Some(ref val) = self.end_invstr_flg {
			let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "end_invstr_flg does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pricg_schme {
			let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "pricg_schme does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// SecuritiesAccountStatement2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecuritiesAccountStatement2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SysDt") )]
	pub sys_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Chng", skip_serializing_if = "Option::is_none") )]
	pub chng: Option<Vec<SecuritiesAccountReferenceDataChange2>>,
}

impl SecuritiesAccountStatement2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.chng { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SecuritiesAccountStatus2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecuritiesAccountStatus2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdSctiesAcct", skip_serializing_if = "Option::is_none") )]
	pub rltd_scties_acct: Option<SecuritiesAccount19>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts") )]
	pub sts: Status6Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsRsn", skip_serializing_if = "Option::is_none") )]
	pub sts_rsn: Option<Vec<StatusReasonInformation10>>,
}

impl SecuritiesAccountStatus2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rltd_scties_acct { val.validate()? }
		self.sts.validate()?;
		if let Some(ref vec) = self.sts_rsn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SecuritiesAuditTrailOrOperationalError4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecuritiesAuditTrailOrOperationalError4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesAudtTrlRpt", skip_serializing_if = "Option::is_none") )]
	pub scties_audt_trl_rpt: Option<Vec<SecuritiesAuditTrailReport4>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OprlErr", skip_serializing_if = "Option::is_none") )]
	pub oprl_err: Option<Vec<ErrorHandling5>>,
}

impl SecuritiesAuditTrailOrOperationalError4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.scties_audt_trl_rpt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.oprl_err { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SecuritiesAuditTrailReport4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecuritiesAuditTrailReport4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesAudtTrlOrErr") )]
	pub scties_audt_trl_or_err: AuditTrailOrBusinessError6Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtPrd", skip_serializing_if = "Option::is_none") )]
	pub dt_prd: Option<DatePeriodSearch1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmId") )]
	pub fin_instrm_id: SecurityIdentification39,
}

impl SecuritiesAuditTrailReport4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.scties_audt_trl_or_err.validate()?;
		if let Some(ref val) = self.dt_prd { val.validate()? }
		self.fin_instrm_id.validate()?;
		Ok(())
	}
}


// SecuritiesAuditTrailSearchCriteria4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecuritiesAuditTrailSearchCriteria4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_id: Option<SecurityIdentification39>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtPrd", skip_serializing_if = "Option::is_none") )]
	pub dt_prd: Option<DatePeriodSearch1Choice>,
}

impl SecuritiesAuditTrailSearchCriteria4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.fin_instrm_id { val.validate()? }
		if let Some(ref val) = self.dt_prd { val.validate()? }
		Ok(())
	}
}


// SecuritiesOrCash1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecuritiesOrCash1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesDtls", skip_serializing_if = "Option::is_none") )]
	pub scties_dtls: Option<SettlementParties35>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshPtiesDtls", skip_serializing_if = "Option::is_none") )]
	pub csh_pties_dtls: Option<CashParties24>,
}

impl SecuritiesOrCash1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.scties_dtls { val.validate()? }
		if let Some(ref val) = self.csh_pties_dtls { val.validate()? }
		Ok(())
	}
}


// SecuritiesPaymentStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum SecuritiesPaymentStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FULL") )]
	CodeFULL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NILL") )]
	CodeNILL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PART") )]
	CodePART,
}

impl SecuritiesPaymentStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SecuritiesPaymentStatus5Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecuritiesPaymentStatus5Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<SecuritiesPaymentStatus1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl SecuritiesPaymentStatus5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// SecuritiesReferenceDataChange3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecuritiesReferenceDataChange3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmId") )]
	pub fin_instrm_id: SecurityIdentification39,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FldNm") )]
	pub fld_nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OdFldVal") )]
	pub od_fld_val: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NewFldVal") )]
	pub new_fld_val: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OprTmStmp") )]
	pub opr_tm_stmp: String,
}

impl SecuritiesReferenceDataChange3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.fin_instrm_id.validate()?;
		if self.fld_nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "fld_nm is shorter than the minimum length of 1".to_string()));
		}
		if self.fld_nm.chars().count() > 35 {
			return Err(ValidationError::new(1002, "fld_nm exceeds the maximum length of 35".to_string()));
		}
		if self.od_fld_val.chars().count() < 1 {
			return Err(ValidationError::new(1001, "od_fld_val is shorter than the minimum length of 1".to_string()));
		}
		if self.od_fld_val.chars().count() > 350 {
			return Err(ValidationError::new(1002, "od_fld_val exceeds the maximum length of 350".to_string()));
		}
		if self.new_fld_val.chars().count() < 1 {
			return Err(ValidationError::new(1001, "new_fld_val is shorter than the minimum length of 1".to_string()));
		}
		if self.new_fld_val.chars().count() > 350 {
			return Err(ValidationError::new(1002, "new_fld_val exceeds the maximum length of 350".to_string()));
		}
		Ok(())
	}
}


// SecuritiesReturnCriteria1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecuritiesReturnCriteria1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmId") )]
	pub fin_instrm_id: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISOSctyLngNm") )]
	pub iso_scty_lng_nm: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISOSctyShrtNm") )]
	pub iso_scty_shrt_nm: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClssfctnFinInstrm") )]
	pub clssfctn_fin_instrm: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MtrtyDt") )]
	pub mtrty_dt: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IsseDt") )]
	pub isse_dt: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IsseCcy") )]
	pub isse_ccy: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfIsse") )]
	pub ctry_of_isse: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctySts") )]
	pub scty_sts: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstrCSD") )]
	pub invstr_csd: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IssrCSD") )]
	pub issr_csd: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechIssrCSD") )]
	pub tech_issr_csd: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CSD") )]
	pub csd: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesQtyTp") )]
	pub scties_qty_tp: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinDnmtn") )]
	pub min_dnmtn: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinMltplQty") )]
	pub min_mltpl_qty: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DevtgSttlmUnit") )]
	pub devtg_sttlm_unit: bool,
}

impl SecuritiesReturnCriteria1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SecuritiesSearchCriteria4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecuritiesSearchCriteria4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_id: Option<SecurityIdentification39>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClssfctnFinInstrm", skip_serializing_if = "Option::is_none") )]
	pub clssfctn_fin_instrm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none") )]
	pub mtrty_dt: Option<DatePeriodSearch1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IsseDt", skip_serializing_if = "Option::is_none") )]
	pub isse_dt: Option<DatePeriodSearch1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IsseCcy", skip_serializing_if = "Option::is_none") )]
	pub isse_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfIsse", skip_serializing_if = "Option::is_none") )]
	pub ctry_of_isse: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctySts", skip_serializing_if = "Option::is_none") )]
	pub scty_sts: Option<SecurityStatus3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MntngCSD", skip_serializing_if = "Option::is_none") )]
	pub mntng_csd: Option<SystemPartyIdentification2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstrCSD", skip_serializing_if = "Option::is_none") )]
	pub invstr_csd: Option<SystemPartyIdentification2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IssrCSD", skip_serializing_if = "Option::is_none") )]
	pub issr_csd: Option<SystemPartyIdentification2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechIssrCSD", skip_serializing_if = "Option::is_none") )]
	pub tech_issr_csd: Option<SystemPartyIdentification2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CSD", skip_serializing_if = "Option::is_none") )]
	pub csd: Option<SystemPartyIdentification2Choice>,
}

impl SecuritiesSearchCriteria4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.fin_instrm_id { val.validate()? }
		if let Some(ref val) = self.clssfctn_fin_instrm {
			let pattern = Regex::new("[A-Z]{6,6}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "clssfctn_fin_instrm does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.mtrty_dt { val.validate()? }
		if let Some(ref val) = self.isse_dt { val.validate()? }
		if let Some(ref val) = self.isse_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "isse_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ctry_of_isse {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry_of_isse does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.scty_sts { val.validate()? }
		if let Some(ref val) = self.mntng_csd { val.validate()? }
		if let Some(ref val) = self.invstr_csd { val.validate()? }
		if let Some(ref val) = self.issr_csd { val.validate()? }
		if let Some(ref val) = self.tech_issr_csd { val.validate()? }
		if let Some(ref val) = self.csd { val.validate()? }
		Ok(())
	}
}


// SecuritiesTransactionType11Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum SecuritiesTransactionType11Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NSYN") )]
	CodeNSYN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SYND") )]
	CodeSYND,
}

impl SecuritiesTransactionType11Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SecuritiesTransactionType31Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecuritiesTransactionType31Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<SecuritiesTransactionType11Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl SecuritiesTransactionType31Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// SecuritiesUpdateReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecuritiesUpdateReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl SecuritiesUpdateReason1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// SecurityAttributes10 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecurityAttributes10 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmId") )]
	pub fin_instrm_id: SecurityIdentification39,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmTp", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_tp: Option<Vec<FinancialInstrument97>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmAttrbts", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_attrbts: Option<Vec<CommonFinancialInstrumentAttributes10>>,
}

impl SecurityAttributes10 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.fin_instrm_id.validate()?;
		if let Some(ref vec) = self.fin_instrm_tp { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.fin_instrm_attrbts { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SecurityAttributes11 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecurityAttributes11 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_id: Option<Vec<SecurityIdentification39>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmTp", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_tp: Option<FinancialInstrument97>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmAttrbts", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_attrbts: Option<Vec<CommonFinancialInstrumentAttributes11>>,
}

impl SecurityAttributes11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.fin_instrm_id { for item in vec { item.validate()? } }
		if let Some(ref val) = self.fin_instrm_tp { val.validate()? }
		if let Some(ref vec) = self.fin_instrm_attrbts { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SecurityAttributes12 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecurityAttributes12 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmTp", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_tp: Option<Vec<FinancialInstrument97>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmAttrbts", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_attrbts: Option<Vec<CommonFinancialInstrumentAttributes12>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl SecurityAttributes12 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.fin_instrm_tp { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.fin_instrm_attrbts { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SecurityCSDLink7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecurityCSDLink7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldFr") )]
	pub vld_fr: DateAndDateTime2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldTo", skip_serializing_if = "Option::is_none") )]
	pub vld_to: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctyMntnc", skip_serializing_if = "Option::is_none") )]
	pub scty_mntnc: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IssrCSD", skip_serializing_if = "Option::is_none") )]
	pub issr_csd: Option<SystemPartyIdentification2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstrCSD", skip_serializing_if = "Option::is_none") )]
	pub invstr_csd: Option<SystemPartyIdentification2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechIssrCSD", skip_serializing_if = "Option::is_none") )]
	pub tech_issr_csd: Option<SystemPartyIdentification2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IssncAcct", skip_serializing_if = "Option::is_none") )]
	pub issnc_acct: Option<Vec<IssuanceAccount2>>,
}

impl SecurityCSDLink7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.vld_fr.validate()?;
		if let Some(ref val) = self.vld_to { val.validate()? }
		if let Some(ref val) = self.issr_csd { val.validate()? }
		if let Some(ref val) = self.invstr_csd { val.validate()? }
		if let Some(ref val) = self.tech_issr_csd { val.validate()? }
		if let Some(ref vec) = self.issnc_acct { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SecurityClassificationType2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// SecurityIdentification19 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecurityIdentification19 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
	pub isin: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrId", skip_serializing_if = "Option::is_none") )]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
}

impl SecurityIdentification19 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.isin {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.othr_id { for item in vec { item.validate()? } }
		if let Some(ref val) = self.desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// SecurityIdentification39 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecurityIdentification39 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
	pub isin: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrId", skip_serializing_if = "Option::is_none") )]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
}

impl SecurityIdentification39 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.isin {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.othr_id { for item in vec { item.validate()? } }
		if let Some(ref val) = self.desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// SecurityIdentification3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// SecurityIdentification40 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// SecurityOrBusinessError4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecurityOrBusinessError4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctyRpt", skip_serializing_if = "Option::is_none") )]
	pub scty_rpt: Option<Vec<SecurityAttributes11>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BizErr", skip_serializing_if = "Option::is_none") )]
	pub biz_err: Option<Vec<BusinessError4>>,
}

impl SecurityOrBusinessError4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.scty_rpt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.biz_err { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SecurityOrOperationalError4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecurityOrOperationalError4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctyRptOrBizErr", skip_serializing_if = "Option::is_none") )]
	pub scty_rpt_or_biz_err: Option<SecurityOrBusinessError4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OprlErr", skip_serializing_if = "Option::is_none") )]
	pub oprl_err: Option<Vec<ErrorHandling5>>,
}

impl SecurityOrOperationalError4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.scty_rpt_or_biz_err { val.validate()? }
		if let Some(ref vec) = self.oprl_err { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SecurityRestriction3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecurityRestriction3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FctvPrd", skip_serializing_if = "Option::is_none") )]
	pub fctv_prd: Option<DateTimePeriod2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RstrctnTp", skip_serializing_if = "Option::is_none") )]
	pub rstrctn_tp: Option<SecurityRestrictionType2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglRstrctnTp", skip_serializing_if = "Option::is_none") )]
	pub lgl_rstrctn_tp: Option<LegalRestrictions5Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstrRstrctnTp", skip_serializing_if = "Option::is_none") )]
	pub invstr_rstrctn_tp: Option<Vec<InvestorRestrictionType3Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstrTp", skip_serializing_if = "Option::is_none") )]
	pub invstr_tp: Option<Vec<InvestorType3Choice>>,
}

impl SecurityRestriction3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.fctv_prd { val.validate()? }
		if let Some(ref val) = self.rstrctn_tp { val.validate()? }
		if let Some(ref val) = self.lgl_rstrctn_tp { val.validate()? }
		if let Some(ref vec) = self.invstr_rstrctn_tp { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.invstr_tp { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SecurityRestrictionType2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecurityRestrictionType2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RstrctnTp", skip_serializing_if = "Option::is_none") )]
	pub rstrctn_tp: Option<RestrictionType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryRstrctn", skip_serializing_if = "Option::is_none") )]
	pub prtry_rstrctn: Option<GenericIdentification30>,
}

impl SecurityRestrictionType2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rstrctn_tp { val.validate()? }
		if let Some(ref val) = self.prtry_rstrctn { val.validate()? }
		Ok(())
	}
}


// SecurityStatement3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecurityStatement3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SysDt") )]
	pub sys_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Chng", skip_serializing_if = "Option::is_none") )]
	pub chng: Option<Vec<SecuritiesReferenceDataChange3>>,
}

impl SecurityStatement3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.chng { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SecurityStatus2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum SecurityStatus2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACTV") )]
	CodeACTV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INAC") )]
	CodeINAC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SUSP") )]
	CodeSUSP,
}

impl SecurityStatus2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SecurityStatus3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecurityStatus3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<SecurityStatus2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl SecurityStatus3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// SecurityWithHoldingTax1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SecurityWithHoldingTax1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "WhldgTaxVal") )]
	pub whldg_tax_val: RateAndAmountFormat1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
	pub ctry: String,
}

impl SecurityWithHoldingTax1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.whldg_tax_val.validate()?;
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.ctry) {
			return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// ServiceRequestStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum ServiceRequestStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACPT") )]
	CodeACPT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RJCT") )]
	CodeRJCT,
}

impl ServiceRequestStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ServiceStatus1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ServiceStatus1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<ServiceRequestStatus1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ServiceStatus1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
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


// SettleStyle1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum SettleStyle1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SETC") )]
	CodeSETC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SETO") )]
	CodeSETO,
}

impl SettleStyle1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SettleStyle2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SettleStyle2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<Vec<SettleStyle1Code>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl SettleStyle2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.cd { for item in vec { item.validate()? } }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// SettlementInformation17 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SettlementInformation17 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesQtyTp", skip_serializing_if = "Option::is_none") )]
	pub scties_qty_tp: Option<SettlementUnitType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctSttlmMnth", skip_serializing_if = "Option::is_none") )]
	pub ctrct_sttlm_mnth: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinDnmtn", skip_serializing_if = "Option::is_none") )]
	pub min_dnmtn: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinMltplQty", skip_serializing_if = "Option::is_none") )]
	pub min_mltpl_qty: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DevtgSttlmUnit", skip_serializing_if = "Option::is_none") )]
	pub devtg_sttlm_unit: Option<Vec<FinancialInstrumentQuantity1Choice>>,
}

impl SettlementInformation17 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.scties_qty_tp { val.validate()? }
		if let Some(ref val) = self.min_dnmtn { val.validate()? }
		if let Some(ref val) = self.min_mltpl_qty { val.validate()? }
		if let Some(ref vec) = self.devtg_sttlm_unit { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SettlementParties32 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SettlementParties32 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dpstry") )]
	pub dpstry: PartyIdentification63,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pty1", skip_serializing_if = "Option::is_none") )]
	pub pty1: Option<PartyIdentificationAndAccount95>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pty2", skip_serializing_if = "Option::is_none") )]
	pub pty2: Option<PartyIdentificationAndAccount95>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pty3", skip_serializing_if = "Option::is_none") )]
	pub pty3: Option<PartyIdentificationAndAccount95>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pty4", skip_serializing_if = "Option::is_none") )]
	pub pty4: Option<PartyIdentificationAndAccount95>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pty5", skip_serializing_if = "Option::is_none") )]
	pub pty5: Option<PartyIdentificationAndAccount95>,
}

impl SettlementParties32 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.dpstry.validate()?;
		if let Some(ref val) = self.pty1 { val.validate()? }
		if let Some(ref val) = self.pty2 { val.validate()? }
		if let Some(ref val) = self.pty3 { val.validate()? }
		if let Some(ref val) = self.pty4 { val.validate()? }
		if let Some(ref val) = self.pty5 { val.validate()? }
		Ok(())
	}
}


// SettlementParties35 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SettlementParties35 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "StgSttlmPties") )]
	pub stg_sttlm_pties: SettlementParties32,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LclMktId", skip_serializing_if = "Option::is_none") )]
	pub lcl_mkt_id: Option<Vec<GenericIdentification49>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegnDtls", skip_serializing_if = "Option::is_none") )]
	pub regn_dtls: Option<PartyIdentification99Choice>,
}

impl SettlementParties35 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.stg_sttlm_pties.validate()?;
		if let Some(ref vec) = self.lcl_mkt_id { for item in vec { item.validate()? } }
		if let Some(ref val) = self.regn_dtls { val.validate()? }
		Ok(())
	}
}


// SettlementType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum SettlementType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRIN") )]
	CodePRIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NETO") )]
	CodeNETO,
}

impl SettlementType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SettlementType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SettlementType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<SettlementType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl SettlementType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// SettlementUnitType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum SettlementUnitType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FAMT") )]
	CodeFAMT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UNIT") )]
	CodeUNIT,
}

impl SettlementUnitType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SettlementUnitType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SettlementUnitType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<SettlementUnitType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl SettlementUnitType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// SignatureType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// SimpleIdentificationInformation4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SimpleIdentificationInformation4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
}

impl SimpleIdentificationInformation4 {
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


// Standardisation1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum Standardisation1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FLEX") )]
	CodeFLEX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NSTA") )]
	CodeNSTA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "STAN") )]
	CodeSTAN,
}

impl Standardisation1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Standardisation3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Standardisation3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<Vec<Standardisation1Code>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl Standardisation3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.cd { for item in vec { item.validate()? } }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// StatisticsByPredefinedTimePeriods2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// Status6Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum Status6Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "REJT") )]
	CodeREJT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "COMP") )]
	CodeCOMP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QUED") )]
	CodeQUED,
}

impl Status6Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// StatusReason6Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct StatusReason6Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl StatusReason6Choice {
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


// StatusReasonInformation10 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct StatusReasonInformation10 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn") )]
	pub rsn: StatusReason6Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<String>,
}

impl StatusReasonInformation10 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rsn.validate()?;
		if let Some(ref val) = self.addtl_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// SupplementaryData1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// SystemAndCurrency1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SystemAndCurrency1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SysId") )]
	pub sys_id: SystemIdentification2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SysCcy", skip_serializing_if = "Option::is_none") )]
	pub sys_ccy: Option<String>,
}

impl SystemAndCurrency1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.sys_id.validate()?;
		if let Some(ref val) = self.sys_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "sys_ccy does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// SystemIdentification2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SystemIdentification2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktInfrstrctrId", skip_serializing_if = "Option::is_none") )]
	pub mkt_infrstrctr_id: Option<MarketInfrastructureIdentification1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
	pub ctry: Option<String>,
}

impl SystemIdentification2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mkt_infrstrctr_id { val.validate()? }
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// SystemParty2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SystemParty2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OpngDt", skip_serializing_if = "Option::is_none") )]
	pub opng_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none") )]
	pub clsg_dt: Option<String>,
}

impl SystemParty2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SystemParty6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SystemParty6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyId", skip_serializing_if = "Option::is_none") )]
	pub pty_id: Option<SystemPartyIdentification9>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Adr", skip_serializing_if = "Option::is_none") )]
	pub adr: Option<PostalAddress28>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none") )]
	pub ctct_dtls: Option<Vec<Contact14>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OpngDt", skip_serializing_if = "Option::is_none") )]
	pub opng_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none") )]
	pub clsg_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<SystemPartyType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechAdr", skip_serializing_if = "Option::is_none") )]
	pub tech_adr: Option<Vec<TechnicalIdentification2Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktSpcfcAttr", skip_serializing_if = "Option::is_none") )]
	pub mkt_spcfc_attr: Option<Vec<MarketSpecificAttribute1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<PartyName4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ResTp", skip_serializing_if = "Option::is_none") )]
	pub res_tp: Option<ResidenceType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LckSts", skip_serializing_if = "Option::is_none") )]
	pub lck_sts: Option<PartyLockStatus1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rstrctn", skip_serializing_if = "Option::is_none") )]
	pub rstrctn: Option<Vec<SystemRestriction1>>,
}

impl SystemParty6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pty_id { val.validate()? }
		if let Some(ref val) = self.adr { val.validate()? }
		if let Some(ref vec) = self.ctct_dtls { for item in vec { item.validate()? } }
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref vec) = self.tech_adr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.mkt_spcfc_attr { for item in vec { item.validate()? } }
		if let Some(ref val) = self.nm { val.validate()? }
		if let Some(ref val) = self.res_tp { val.validate()? }
		if let Some(ref val) = self.lck_sts { val.validate()? }
		if let Some(ref vec) = self.rstrctn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SystemParty7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SystemParty7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyId") )]
	pub pty_id: SystemPartyIdentification9,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Adr", skip_serializing_if = "Option::is_none") )]
	pub adr: Option<Vec<PostalAddress28>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none") )]
	pub ctct_dtls: Option<Vec<Contact14>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OpngDt", skip_serializing_if = "Option::is_none") )]
	pub opng_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none") )]
	pub clsg_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: SystemPartyType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechAdr", skip_serializing_if = "Option::is_none") )]
	pub tech_adr: Option<Vec<TechnicalIdentification2Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktSpcfcAttr", skip_serializing_if = "Option::is_none") )]
	pub mkt_spcfc_attr: Option<Vec<MarketSpecificAttribute1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<PartyName4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ResTp", skip_serializing_if = "Option::is_none") )]
	pub res_tp: Option<ResidenceType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LckSts", skip_serializing_if = "Option::is_none") )]
	pub lck_sts: Option<PartyLockStatus1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rstrctn", skip_serializing_if = "Option::is_none") )]
	pub rstrctn: Option<Vec<SystemRestriction1>>,
}

impl SystemParty7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pty_id.validate()?;
		if let Some(ref vec) = self.adr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.ctct_dtls { for item in vec { item.validate()? } }
		self.tp.validate()?;
		if let Some(ref vec) = self.tech_adr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.mkt_spcfc_attr { for item in vec { item.validate()? } }
		if let Some(ref val) = self.nm { val.validate()? }
		if let Some(ref val) = self.res_tp { val.validate()? }
		if let Some(ref val) = self.lck_sts { val.validate()? }
		if let Some(ref vec) = self.rstrctn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SystemPartyIdentification10 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SystemPartyIdentification10 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldFr", skip_serializing_if = "Option::is_none") )]
	pub vld_fr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<PartyIdentification136>,
}

impl SystemPartyIdentification10 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id { val.validate()? }
		Ok(())
	}
}


// SystemPartyIdentification2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SystemPartyIdentification2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgId", skip_serializing_if = "Option::is_none") )]
	pub org_id: Option<PartyIdentification136>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CmbndId", skip_serializing_if = "Option::is_none") )]
	pub cmbnd_id: Option<SystemPartyIdentification8>,
}

impl SystemPartyIdentification2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.org_id { val.validate()? }
		if let Some(ref val) = self.cmbnd_id { val.validate()? }
		Ok(())
	}
}


// SystemPartyIdentification8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SystemPartyIdentification8 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: PartyIdentification136,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RspnsblPtyId", skip_serializing_if = "Option::is_none") )]
	pub rspnsbl_pty_id: Option<PartyIdentification136>,
}

impl SystemPartyIdentification8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		if let Some(ref val) = self.rspnsbl_pty_id { val.validate()? }
		Ok(())
	}
}


// SystemPartyIdentification9 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SystemPartyIdentification9 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: PartyIdentification136,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RspnsblPtyId", skip_serializing_if = "Option::is_none") )]
	pub rspnsbl_pty_id: Option<PartyIdentification136>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldFr", skip_serializing_if = "Option::is_none") )]
	pub vld_fr: Option<String>,
}

impl SystemPartyIdentification9 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		if let Some(ref val) = self.rspnsbl_pty_id { val.validate()? }
		Ok(())
	}
}


// SystemPartyModification3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SystemPartyModification3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ScpIndctn") )]
	pub scp_indctn: DataModification1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdMod") )]
	pub reqd_mod: SystemPartyModification3Choice,
}

impl SystemPartyModification3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.scp_indctn.validate()?;
		self.reqd_mod.validate()?;
		Ok(())
	}
}


// SystemPartyModification3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SystemPartyModification3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SysPtyDt", skip_serializing_if = "Option::is_none") )]
	pub sys_pty_dt: Option<SystemParty2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyId", skip_serializing_if = "Option::is_none") )]
	pub pty_id: Option<SystemPartyIdentification10>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyNm", skip_serializing_if = "Option::is_none") )]
	pub pty_nm: Option<PartyName3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none") )]
	pub ctct_dtls: Option<Contact14>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechAdr", skip_serializing_if = "Option::is_none") )]
	pub tech_adr: Option<TechnicalIdentification2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyAdr", skip_serializing_if = "Option::is_none") )]
	pub pty_adr: Option<PostalAddress28>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ResTp", skip_serializing_if = "Option::is_none") )]
	pub res_tp: Option<ResidenceType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LckSts", skip_serializing_if = "Option::is_none") )]
	pub lck_sts: Option<PartyLockStatus1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SysRstrctn", skip_serializing_if = "Option::is_none") )]
	pub sys_rstrctn: Option<SystemRestriction1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktSpcfcAttr", skip_serializing_if = "Option::is_none") )]
	pub mkt_spcfc_attr: Option<MarketSpecificAttribute1>,
}

impl SystemPartyModification3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.sys_pty_dt { val.validate()? }
		if let Some(ref val) = self.pty_id { val.validate()? }
		if let Some(ref val) = self.pty_nm { val.validate()? }
		if let Some(ref val) = self.ctct_dtls { val.validate()? }
		if let Some(ref val) = self.tech_adr { val.validate()? }
		if let Some(ref val) = self.pty_adr { val.validate()? }
		if let Some(ref val) = self.res_tp { val.validate()? }
		if let Some(ref val) = self.lck_sts { val.validate()? }
		if let Some(ref val) = self.sys_rstrctn { val.validate()? }
		if let Some(ref val) = self.mkt_spcfc_attr { val.validate()? }
		Ok(())
	}
}


// SystemPartyType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SystemPartyType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl SystemPartyType1Choice {
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


// SystemRestriction1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SystemRestriction1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldFr") )]
	pub vld_fr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldTo", skip_serializing_if = "Option::is_none") )]
	pub vld_to: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: String,
}

impl SystemRestriction1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
		}
		if self.tp.chars().count() > 35 {
			return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// SystemSecuritiesAccount5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SystemSecuritiesAccount5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none") )]
	pub clsg_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HldInd", skip_serializing_if = "Option::is_none") )]
	pub hld_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NegPos", skip_serializing_if = "Option::is_none") )]
	pub neg_pos: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndInvstrFlg", skip_serializing_if = "Option::is_none") )]
	pub end_invstr_flg: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricgSchme", skip_serializing_if = "Option::is_none") )]
	pub pricg_schme: Option<String>,
}

impl SystemSecuritiesAccount5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.end_invstr_flg {
			let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "end_invstr_flg does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pricg_schme {
			let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "pricg_schme does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// SystemSecuritiesAccount6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SystemSecuritiesAccount6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OpngDt", skip_serializing_if = "Option::is_none") )]
	pub opng_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none") )]
	pub clsg_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HldInd", skip_serializing_if = "Option::is_none") )]
	pub hld_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NegPos", skip_serializing_if = "Option::is_none") )]
	pub neg_pos: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<SystemSecuritiesAccountType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctOwnr") )]
	pub acct_ownr: SystemPartyIdentification8,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyTp", skip_serializing_if = "Option::is_none") )]
	pub pty_tp: Option<SystemPartyType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktSpcfcAttr", skip_serializing_if = "Option::is_none") )]
	pub mkt_spcfc_attr: Option<Vec<MarketSpecificAttribute1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rstrctn", skip_serializing_if = "Option::is_none") )]
	pub rstrctn: Option<Vec<SystemRestriction1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndInvstrFlg", skip_serializing_if = "Option::is_none") )]
	pub end_invstr_flg: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricgSchme", skip_serializing_if = "Option::is_none") )]
	pub pricg_schme: Option<String>,
}

impl SystemSecuritiesAccount6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		self.acct_ownr.validate()?;
		if let Some(ref val) = self.pty_tp { val.validate()? }
		if let Some(ref vec) = self.mkt_spcfc_attr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.rstrctn { for item in vec { item.validate()? } }
		if let Some(ref val) = self.end_invstr_flg {
			let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "end_invstr_flg does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pricg_schme {
			let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "pricg_schme does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// SystemSecuritiesAccount7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SystemSecuritiesAccount7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctOwnr") )]
	pub acct_ownr: SystemPartyIdentification8,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: SystemSecuritiesAccountType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OpngDt") )]
	pub opng_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none") )]
	pub clsg_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HldInd") )]
	pub hld_ind: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NegPos") )]
	pub neg_pos: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktSpcfcAttr", skip_serializing_if = "Option::is_none") )]
	pub mkt_spcfc_attr: Option<Vec<MarketSpecificAttribute1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rstrctn", skip_serializing_if = "Option::is_none") )]
	pub rstrctn: Option<Vec<SystemRestriction1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndInvstrFlg", skip_serializing_if = "Option::is_none") )]
	pub end_invstr_flg: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricgSchme", skip_serializing_if = "Option::is_none") )]
	pub pricg_schme: Option<String>,
}

impl SystemSecuritiesAccount7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.acct_ownr.validate()?;
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		self.tp.validate()?;
		if let Some(ref vec) = self.mkt_spcfc_attr { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.rstrctn { for item in vec { item.validate()? } }
		if let Some(ref val) = self.end_invstr_flg {
			let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "end_invstr_flg does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pricg_schme {
			let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "pricg_schme does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// SystemSecuritiesAccountType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SystemSecuritiesAccountType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<SystemSecuritiesAccountType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl SystemSecuritiesAccountType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// SystemSecuritiesAccountType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum SystemSecuritiesAccountType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CSDP") )]
	CodeCSDP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CSDM") )]
	CodeCSDM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ICSA") )]
	CodeICSA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TOFF") )]
	CodeTOFF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CSDO") )]
	CodeCSDO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISSA") )]
	CodeISSA,
}

impl SystemSecuritiesAccountType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SystemStatus3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SystemStatus3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<SystemStatus3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification1>,
}

impl SystemStatus3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// SystemStatus3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum SystemStatus3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACTV") )]
	CodeACTV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLSD") )]
	CodeCLSD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RMPS") )]
	CodeRMPS,
}

impl SystemStatus3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TEFRARules1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum TEFRARules1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "RULC") )]
	CodeRULC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RULD") )]
	CodeRULD,
}

impl TEFRARules1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TEFRARules3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TEFRARules3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<TEFRARules1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl TEFRARules3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// TargetMarket1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// Tax17 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// TechnicalIdentification2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TechnicalIdentification2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "BICFI", skip_serializing_if = "Option::is_none") )]
	pub bicfi: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechAdr", skip_serializing_if = "Option::is_none") )]
	pub tech_adr: Option<String>,
}

impl TechnicalIdentification2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.bicfi {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "bicfi does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.tech_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tech_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 256 {
				return Err(ValidationError::new(1002, "tech_adr exceeds the maximum length of 256".to_string()));
			}
		}
		Ok(())
	}
}


// Term1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Term1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Oprtr") )]
	pub oprtr: Operator1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
	pub val: RateOrAbsoluteValue1Choice,
}

impl Term1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.oprtr.validate()?;
		self.val.validate()?;
		Ok(())
	}
}


// TimeFrame10 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// TimeUnit1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum TimeUnit1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DAYC") )]
	CodeDAYC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HOUR") )]
	CodeHOUR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MINU") )]
	CodeMINU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
	CodeMNTH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SECO") )]
	CodeSECO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WEEK") )]
	CodeWEEK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
	CodeYEAR,
}

impl TimeUnit1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TimeUnit3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TimeUnit3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<TimeUnit1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl TimeUnit3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// TradeTransactionCondition2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum TradeTransactionCondition2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SPCC") )]
	CodeSPCC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SECN") )]
	CodeSECN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SEBN") )]
	CodeSEBN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SCBN") )]
	CodeSCBN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SCRT") )]
	CodeSCRT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SERT") )]
	CodeSERT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SCCR") )]
	CodeSCCR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SECR") )]
	CodeSECR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CAST") )]
	CodeCAST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SPPR") )]
	CodeSPPR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SPCU") )]
	CodeSPCU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SPEX") )]
	CodeSPEX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GTDL") )]
	CodeGTDL,
}

impl TradeTransactionCondition2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TradeTransactionCondition7Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TradeTransactionCondition7Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<TradeTransactionCondition2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl TradeTransactionCondition7Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// TradingParameters2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TradingParameters2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktId", skip_serializing_if = "Option::is_none") )]
	pub mkt_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RndLot", skip_serializing_if = "Option::is_none") )]
	pub rnd_lot: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradLotSz", skip_serializing_if = "Option::is_none") )]
	pub trad_lot_sz: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ScndryPlcOfListg", skip_serializing_if = "Option::is_none") )]
	pub scndry_plc_of_listg: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinTraddNmnlQty", skip_serializing_if = "Option::is_none") )]
	pub min_tradd_nmnl_qty: Option<UnitOrFaceAmount1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MaxTraddNmnlQty", skip_serializing_if = "Option::is_none") )]
	pub max_tradd_nmnl_qty: Option<UnitOrFaceAmount1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinTradgPricgIncrmt", skip_serializing_if = "Option::is_none") )]
	pub min_tradg_pricg_incrmt: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmryPlcOfListgId", skip_serializing_if = "Option::is_none") )]
	pub pmry_plc_of_listg_id: Option<String>,
}

impl TradingParameters2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mkt_id {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "mkt_id does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.rnd_lot { val.validate()? }
		if let Some(ref val) = self.trad_lot_sz { val.validate()? }
		if let Some(ref vec) = self.scndry_plc_of_listg {
			for item in vec {
				let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
				if !pattern.is_match(&item) {
					return Err(ValidationError::new(1005, "scndry_plc_of_listg does not match the required pattern".to_string()));
				}
			}
		}
		if let Some(ref val) = self.min_tradd_nmnl_qty { val.validate()? }
		if let Some(ref val) = self.max_tradd_nmnl_qty { val.validate()? }
		if let Some(ref val) = self.pmry_plc_of_listg_id {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "pmry_plc_of_listg_id does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// TypeOfPrice1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum TypeOfPrice1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "AVER") )]
	CodeAVER,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AVOV") )]
	CodeAVOV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "COMB") )]
	CodeCOMB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GREX") )]
	CodeGREX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LIMI") )]
	CodeLIMI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NET2") )]
	CodeNET2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NDIS") )]
	CodeNDIS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NET1") )]
	CodeNET1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NUND") )]
	CodeNUND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NOGR") )]
	CodeNOGR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PARV") )]
	CodePARV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RDAV") )]
	CodeRDAV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "STOP") )]
	CodeSTOP,
}

impl TypeOfPrice1Code {
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// UTCOffset1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// UnderlyingAttributes4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct UnderlyingAttributes4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AllcnPctg", skip_serializing_if = "Option::is_none") )]
	pub allcn_pctg: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Qty", skip_serializing_if = "Option::is_none") )]
	pub qty: Option<UnitOrFaceAmount1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmTp", skip_serializing_if = "Option::is_none") )]
	pub sttlm_tp: Option<SettlementType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshAmt", skip_serializing_if = "Option::is_none") )]
	pub csh_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshTp", skip_serializing_if = "Option::is_none") )]
	pub csh_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pric", skip_serializing_if = "Option::is_none") )]
	pub pric: Option<Price8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DrtyPric", skip_serializing_if = "Option::is_none") )]
	pub drty_pric: Option<Price8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndPric", skip_serializing_if = "Option::is_none") )]
	pub end_pric: Option<Price8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StartVal", skip_serializing_if = "Option::is_none") )]
	pub start_val: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CurVal", skip_serializing_if = "Option::is_none") )]
	pub cur_val: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndVal", skip_serializing_if = "Option::is_none") )]
	pub end_val: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdjstdQty", skip_serializing_if = "Option::is_none") )]
	pub adjstd_qty: Option<UnitOrFaceAmount1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XchgRate", skip_serializing_if = "Option::is_none") )]
	pub xchg_rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CapVal", skip_serializing_if = "Option::is_none") )]
	pub cap_val: Option<ActiveCurrencyAndAmount>,
}

impl UnderlyingAttributes4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.qty { val.validate()? }
		if let Some(ref val) = self.sttlm_tp { val.validate()? }
		if let Some(ref val) = self.csh_amt { val.validate()? }
		if let Some(ref val) = self.csh_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "csh_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "csh_tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.pric { val.validate()? }
		if let Some(ref val) = self.drty_pric { val.validate()? }
		if let Some(ref val) = self.end_pric { val.validate()? }
		if let Some(ref val) = self.start_val { val.validate()? }
		if let Some(ref val) = self.cur_val { val.validate()? }
		if let Some(ref val) = self.end_val { val.validate()? }
		if let Some(ref val) = self.adjstd_qty { val.validate()? }
		if let Some(ref val) = self.cap_val { val.validate()? }
		Ok(())
	}
}


// UnitOfMeasure7Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct UnitOfMeasure7Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<UnitOfMeasure9Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl UnitOfMeasure7Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// UnitOfMeasure9Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum UnitOfMeasure9Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "BAGG") )]
	CodeBAGG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BALE") )]
	CodeBALE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BOTL") )]
	CodeBOTL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BOXX") )]
	CodeBOXX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRTN") )]
	CodeCRTN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CELI") )]
	CodeCELI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CMET") )]
	CodeCMET,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CNTR") )]
	CodeCNTR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRAT") )]
	CodeCRAT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CBIN") )]
	CodeCBIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CBME") )]
	CodeCBME,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CBML") )]
	CodeCBML,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PIEC") )]
	CodePIEC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FOOT") )]
	CodeFOOT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GBFO") )]
	CodeGBFO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GBGA") )]
	CodeGBGA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GBPI") )]
	CodeGBPI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GBQA") )]
	CodeGBQA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GBTN") )]
	CodeGBTN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GRAM") )]
	CodeGRAM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INCH") )]
	CodeINCH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "KILO") )]
	CodeKILO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "KMET") )]
	CodeKMET,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LITR") )]
	CodeLITR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "METR") )]
	CodeMETR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TONE") )]
	CodeTONE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MILE") )]
	CodeMILE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MMET") )]
	CodeMMET,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MILI") )]
	CodeMILI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUND") )]
	CodePUND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USOU") )]
	CodeUSOU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SCMT") )]
	CodeSCMT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SQFO") )]
	CodeSQFO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SQIN") )]
	CodeSQIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SQKI") )]
	CodeSQKI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SMET") )]
	CodeSMET,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SQMI") )]
	CodeSQMI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SMIL") )]
	CodeSMIL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SQYA") )]
	CodeSQYA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USBA") )]
	CodeUSBA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USFO") )]
	CodeUSFO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USGA") )]
	CodeUSGA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USPI") )]
	CodeUSPI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USQA") )]
	CodeUSQA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USTN") )]
	CodeUSTN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YARD") )]
	CodeYARD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GBOU") )]
	CodeGBOU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACRE") )]
	CodeACRE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ARES") )]
	CodeARES,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HECT") )]
	CodeHECT,
}

impl UnitOfMeasure9Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// UnitOrFaceAmount1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct UnitOrFaceAmount1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Unit", skip_serializing_if = "Option::is_none") )]
	pub unit: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none") )]
	pub face_amt: Option<ActiveCurrencyAndAmount>,
}

impl UnitOrFaceAmount1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.face_amt { val.validate()? }
		Ok(())
	}
}


// UnitPrice15 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// UnitsOrAmount1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// UpdateLogAddress2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct UpdateLogAddress2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Od") )]
	pub od: PostalAddress28,
	#[cfg_attr( feature = "derive_serde", serde(rename = "New") )]
	pub new: PostalAddress28,
}

impl UpdateLogAddress2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.od.validate()?;
		self.new.validate()?;
		Ok(())
	}
}


// UpdateLogContact2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct UpdateLogContact2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Od") )]
	pub od: Contact14,
	#[cfg_attr( feature = "derive_serde", serde(rename = "New") )]
	pub new: Contact14,
}

impl UpdateLogContact2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.od.validate()?;
		self.new.validate()?;
		Ok(())
	}
}


// UpdateLogDate1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct UpdateLogDate1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Od") )]
	pub od: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "New") )]
	pub new: String,
}

impl UpdateLogDate1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// UpdateLogMarketSpecificAttribute1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct UpdateLogMarketSpecificAttribute1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Od") )]
	pub od: MarketSpecificAttribute1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "New") )]
	pub new: MarketSpecificAttribute1,
}

impl UpdateLogMarketSpecificAttribute1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.od.validate()?;
		self.new.validate()?;
		Ok(())
	}
}


// UpdateLogPartyLockStatus1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct UpdateLogPartyLockStatus1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Od") )]
	pub od: PartyLockStatus1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "New") )]
	pub new: PartyLockStatus1,
}

impl UpdateLogPartyLockStatus1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.od.validate()?;
		self.new.validate()?;
		Ok(())
	}
}


// UpdateLogPartyName1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct UpdateLogPartyName1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Od") )]
	pub od: PartyName4,
	#[cfg_attr( feature = "derive_serde", serde(rename = "New") )]
	pub new: PartyName4,
}

impl UpdateLogPartyName1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.od.validate()?;
		self.new.validate()?;
		Ok(())
	}
}


// UpdateLogPartyRecord2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct UpdateLogPartyRecord2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Adr", skip_serializing_if = "Option::is_none") )]
	pub adr: Option<UpdateLogAddress2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none") )]
	pub ctct_dtls: Option<UpdateLogContact2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OpngDt", skip_serializing_if = "Option::is_none") )]
	pub opng_dt: Option<UpdateLogDate1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none") )]
	pub clsg_dt: Option<UpdateLogDate1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<UpdateLogSystemPartyType1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechAdr", skip_serializing_if = "Option::is_none") )]
	pub tech_adr: Option<UpdateLogTechnicalAddress1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktSpcfcAttr", skip_serializing_if = "Option::is_none") )]
	pub mkt_spcfc_attr: Option<UpdateLogMarketSpecificAttribute1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<UpdateLogPartyName1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ResTp", skip_serializing_if = "Option::is_none") )]
	pub res_tp: Option<UpdateLogResidenceType1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LckSts", skip_serializing_if = "Option::is_none") )]
	pub lck_sts: Option<UpdateLogPartyLockStatus1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rstrctn", skip_serializing_if = "Option::is_none") )]
	pub rstrctn: Option<UpdateLogRestriction1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<UpdateLogProprietary1>>,
}

impl UpdateLogPartyRecord2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.adr { val.validate()? }
		if let Some(ref val) = self.ctct_dtls { val.validate()? }
		if let Some(ref val) = self.opng_dt { val.validate()? }
		if let Some(ref val) = self.clsg_dt { val.validate()? }
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.tech_adr { val.validate()? }
		if let Some(ref val) = self.mkt_spcfc_attr { val.validate()? }
		if let Some(ref val) = self.nm { val.validate()? }
		if let Some(ref val) = self.res_tp { val.validate()? }
		if let Some(ref val) = self.lck_sts { val.validate()? }
		if let Some(ref val) = self.rstrctn { val.validate()? }
		if let Some(ref vec) = self.othr { for item in vec { item.validate()? } }
		Ok(())
	}
}


// UpdateLogProprietary1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct UpdateLogProprietary1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FldNm") )]
	pub fld_nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OdFldVal") )]
	pub od_fld_val: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NewFldVal") )]
	pub new_fld_val: String,
}

impl UpdateLogProprietary1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.fld_nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "fld_nm is shorter than the minimum length of 1".to_string()));
		}
		if self.fld_nm.chars().count() > 35 {
			return Err(ValidationError::new(1002, "fld_nm exceeds the maximum length of 35".to_string()));
		}
		if self.od_fld_val.chars().count() < 1 {
			return Err(ValidationError::new(1001, "od_fld_val is shorter than the minimum length of 1".to_string()));
		}
		if self.od_fld_val.chars().count() > 350 {
			return Err(ValidationError::new(1002, "od_fld_val exceeds the maximum length of 350".to_string()));
		}
		if self.new_fld_val.chars().count() < 1 {
			return Err(ValidationError::new(1001, "new_fld_val is shorter than the minimum length of 1".to_string()));
		}
		if self.new_fld_val.chars().count() > 350 {
			return Err(ValidationError::new(1002, "new_fld_val exceeds the maximum length of 350".to_string()));
		}
		Ok(())
	}
}


// UpdateLogResidenceType1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct UpdateLogResidenceType1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Od") )]
	pub od: ResidenceType1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "New") )]
	pub new: ResidenceType1Code,
}

impl UpdateLogResidenceType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.od.validate()?;
		self.new.validate()?;
		Ok(())
	}
}


// UpdateLogRestriction1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct UpdateLogRestriction1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Od") )]
	pub od: Restriction1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "New") )]
	pub new: Restriction1,
}

impl UpdateLogRestriction1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.od.validate()?;
		self.new.validate()?;
		Ok(())
	}
}


// UpdateLogSystemPartyType1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct UpdateLogSystemPartyType1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Od") )]
	pub od: SystemPartyType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "New") )]
	pub new: SystemPartyType1Choice,
}

impl UpdateLogSystemPartyType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.od.validate()?;
		self.new.validate()?;
		Ok(())
	}
}


// UpdateLogTechnicalAddress1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct UpdateLogTechnicalAddress1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Od") )]
	pub od: TechnicalIdentification2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "New") )]
	pub new: TechnicalIdentification2Choice,
}

impl UpdateLogTechnicalAddress1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.od.validate()?;
		self.new.validate()?;
		Ok(())
	}
}


// UpdateType35Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct UpdateType35Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Add", skip_serializing_if = "Option::is_none") )]
	pub add: Option<SecurityAttributes12>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Del", skip_serializing_if = "Option::is_none") )]
	pub del: Option<SecurityAttributes12>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Modfy", skip_serializing_if = "Option::is_none") )]
	pub modfy: Option<SecurityAttributes12>,
}

impl UpdateType35Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.add { val.validate()? }
		if let Some(ref val) = self.del { val.validate()? }
		if let Some(ref val) = self.modfy { val.validate()? }
		Ok(())
	}
}


// UpdateType36Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct UpdateType36Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "UpdTp", skip_serializing_if = "Option::is_none") )]
	pub upd_tp: Option<Vec<UpdateType35Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rplc", skip_serializing_if = "Option::is_none") )]
	pub rplc: Option<SecurityAttributes12>,
}

impl UpdateType36Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.upd_tp { for item in vec { item.validate()? } }
		if let Some(ref val) = self.rplc { val.validate()? }
		Ok(())
	}
}


// ValuationDealingProcessingCharacteristics3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// ValuationStatistics3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// ValueForMoney1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
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


// Visibilty1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Visibilty1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt", skip_serializing_if = "Option::is_none") )]
	pub start_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndDt", skip_serializing_if = "Option::is_none") )]
	pub end_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LtdVsblty", skip_serializing_if = "Option::is_none") )]
	pub ltd_vsblty: Option<bool>,
}

impl Visibilty1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.start_dt { val.validate()? }
		if let Some(ref val) = self.end_dt { val.validate()? }
		Ok(())
	}
}


// Warrant4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Warrant4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mltplr", skip_serializing_if = "Option::is_none") )]
	pub mltplr: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SbcptPric", skip_serializing_if = "Option::is_none") )]
	pub sbcpt_pric: Option<Price8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<WarrantStyle3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WarrtAgt", skip_serializing_if = "Option::is_none") )]
	pub warrt_agt: Option<Vec<Organisation38>>,
}

impl Warrant4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.sbcpt_pric { val.validate()? }
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref vec) = self.warrt_agt { for item in vec { item.validate()? } }
		Ok(())
	}
}


// WarrantStyle1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum WarrantStyle1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "AMER") )]
	CodeAMER,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EURO") )]
	CodeEURO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BERM") )]
	CodeBERM,
}

impl WarrantStyle1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// WarrantStyle3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct WarrantStyle3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<WarrantStyle1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl WarrantStyle3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// YieldCalculation6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct YieldCalculation6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
	pub val: f64,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClctnTp", skip_serializing_if = "Option::is_none") )]
	pub clctn_tp: Option<CalculationType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RedPric", skip_serializing_if = "Option::is_none") )]
	pub red_pric: Option<Price8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValDt") )]
	pub val_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValPrd") )]
	pub val_prd: DateTimePeriod1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClctnDt") )]
	pub clctn_dt: String,
}

impl YieldCalculation6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.clctn_tp { val.validate()? }
		if let Some(ref val) = self.red_pric { val.validate()? }
		self.val_prd.validate()?;
		Ok(())
	}
}