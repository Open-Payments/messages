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


// AcceptanceResult6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AcceptanceResult6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Accptd") )]
	pub accptd: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RjctRsn", skip_serializing_if = "Option::is_none") )]
	pub rjct_rsn: Option<MandateReason1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRjctRsnInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_rjct_rsn_inf: Option<Vec<String>>,
}

impl AcceptanceResult6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rjct_rsn { val.validate()? }
		if let Some(ref vec) = self.addtl_rjct_rsn_inf {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_rjct_rsn_inf is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 105 {
					return Err(ValidationError::new(1002, "addtl_rjct_rsn_inf exceeds the maximum length of 105".to_string()));
				}
			}
		}
		Ok(())
	}
}


// AccountIdentification4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AccountIdentification4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "IBAN", skip_serializing_if = "Option::is_none") )]
	pub iban: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<GenericAccountIdentification1>,
}

impl AccountIdentification4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.iban {
			let pattern = Regex::new("[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "iban does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.othr { val.validate()? }
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


// AdviceType1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AdviceType1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtAdvc", skip_serializing_if = "Option::is_none") )]
	pub cdt_advc: Option<AdviceType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtAdvc", skip_serializing_if = "Option::is_none") )]
	pub dbt_advc: Option<AdviceType1Choice>,
}

impl AdviceType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cdt_advc { val.validate()? }
		if let Some(ref val) = self.dbt_advc { val.validate()? }
		Ok(())
	}
}


// AdviceType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AdviceType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<AdviceType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl AdviceType1Choice {
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


// AdviceType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum AdviceType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ADWD") )]
	CodeADWD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ADND") )]
	CodeADND,
}

impl AdviceType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AmendmentInformationDetails15 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AmendmentInformationDetails15 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMndtId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_mndt_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCdtrSchmeId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_cdtr_schme_id: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_cdtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCdtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub orgnl_cdtr_agt_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlDbtr", skip_serializing_if = "Option::is_none") )]
	pub orgnl_dbtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlDbtrAcct", skip_serializing_if = "Option::is_none") )]
	pub orgnl_dbtr_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlDbtrAgt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_dbtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlDbtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub orgnl_dbtr_agt_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlFnlColltnDt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_fnl_colltn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlFrqcy", skip_serializing_if = "Option::is_none") )]
	pub orgnl_frqcy: Option<Frequency36Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlRsn", skip_serializing_if = "Option::is_none") )]
	pub orgnl_rsn: Option<MandateSetupReason1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTrckgDays", skip_serializing_if = "Option::is_none") )]
	pub orgnl_trckg_days: Option<String>,
}

impl AmendmentInformationDetails15 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_mndt_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_mndt_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_mndt_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_cdtr_schme_id { val.validate()? }
		if let Some(ref val) = self.orgnl_cdtr_agt { val.validate()? }
		if let Some(ref val) = self.orgnl_cdtr_agt_acct { val.validate()? }
		if let Some(ref val) = self.orgnl_dbtr { val.validate()? }
		if let Some(ref val) = self.orgnl_dbtr_acct { val.validate()? }
		if let Some(ref val) = self.orgnl_dbtr_agt { val.validate()? }
		if let Some(ref val) = self.orgnl_dbtr_agt_acct { val.validate()? }
		if let Some(ref val) = self.orgnl_frqcy { val.validate()? }
		if let Some(ref val) = self.orgnl_rsn { val.validate()? }
		if let Some(ref val) = self.orgnl_trckg_days {
			let pattern = Regex::new("[0-9]{2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_trckg_days does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// AmountOrRate1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AmountOrRate1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
	pub rate: Option<f64>,
}

impl AmountOrRate1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.amt { val.validate()? }
		Ok(())
	}
}


// AmountType4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AmountType4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none") )]
	pub instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EqvtAmt", skip_serializing_if = "Option::is_none") )]
	pub eqvt_amt: Option<EquivalentAmount2>,
}

impl AmountType4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.instd_amt { val.validate()? }
		if let Some(ref val) = self.eqvt_amt { val.validate()? }
		Ok(())
	}
}


// AuthenticationChannel1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct AuthenticationChannel1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl AuthenticationChannel1Choice {
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


// Authorisation1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Authorisation1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<Authorisation1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl Authorisation1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 128 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 128".to_string()));
			}
		}
		Ok(())
	}
}


// Authorisation1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum Authorisation1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "AUTH") )]
	CodeAUTH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FDET") )]
	CodeFDET,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FSUM") )]
	CodeFSUM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ILEV") )]
	CodeILEV,
}

impl Authorisation1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// BranchAndFinancialInstitutionIdentification8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct BranchAndFinancialInstitutionIdentification8 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstnId") )]
	pub fin_instn_id: FinancialInstitutionIdentification23,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BrnchId", skip_serializing_if = "Option::is_none") )]
	pub brnch_id: Option<BranchData5>,
}

impl BranchAndFinancialInstitutionIdentification8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.fin_instn_id.validate()?;
		if let Some(ref val) = self.brnch_id { val.validate()? }
		Ok(())
	}
}


// BranchData5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct BranchData5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
	pub pstl_adr: Option<PostalAddress27>,
}

impl BranchData5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.pstl_adr { val.validate()? }
		Ok(())
	}
}


// CashAccount40 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CashAccount40 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<AccountIdentification4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<CashAccountType2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy", skip_serializing_if = "Option::is_none") )]
	pub ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prxy", skip_serializing_if = "Option::is_none") )]
	pub prxy: Option<ProxyAccountIdentification1>,
}

impl CashAccount40 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id { val.validate()? }
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.prxy { val.validate()? }
		Ok(())
	}
}


// CashAccountType2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CashAccountType2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl CashAccountType2Choice {
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


// CategoryPurpose1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CategoryPurpose1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl CategoryPurpose1Choice {
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


// ChargeBearerType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum ChargeBearerType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DEBT") )]
	CodeDEBT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRED") )]
	CodeCRED,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SHAR") )]
	CodeSHAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SLEV") )]
	CodeSLEV,
}

impl ChargeBearerType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ChargeType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ChargeType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification3>,
}

impl ChargeType3Choice {
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


// Charges16 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Charges16 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Agt") )]
	pub agt: BranchAndFinancialInstitutionIdentification8,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<ChargeType3Choice>,
}

impl Charges16 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		self.agt.validate()?;
		if let Some(ref val) = self.tp { val.validate()? }
		Ok(())
	}
}


// Cheque19 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Cheque19 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChqTp", skip_serializing_if = "Option::is_none") )]
	pub chq_tp: Option<ChequeType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChqNb", skip_serializing_if = "Option::is_none") )]
	pub chq_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChqFr", skip_serializing_if = "Option::is_none") )]
	pub chq_fr: Option<NameAndAddress18>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DlvryMtd", skip_serializing_if = "Option::is_none") )]
	pub dlvry_mtd: Option<ChequeDeliveryMethod1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DlvrTo", skip_serializing_if = "Option::is_none") )]
	pub dlvr_to: Option<NameAndAddress18>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none") )]
	pub instr_prty: Option<Priority2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChqMtrtyDt", skip_serializing_if = "Option::is_none") )]
	pub chq_mtrty_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrmsCd", skip_serializing_if = "Option::is_none") )]
	pub frms_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MemoFld", skip_serializing_if = "Option::is_none") )]
	pub memo_fld: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RgnlClrZone", skip_serializing_if = "Option::is_none") )]
	pub rgnl_clr_zone: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtLctn", skip_serializing_if = "Option::is_none") )]
	pub prt_lctn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sgntr", skip_serializing_if = "Option::is_none") )]
	pub sgntr: Option<Vec<String>>,
}

impl Cheque19 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.chq_tp { val.validate()? }
		if let Some(ref val) = self.chq_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "chq_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "chq_nb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.chq_fr { val.validate()? }
		if let Some(ref val) = self.dlvry_mtd { val.validate()? }
		if let Some(ref val) = self.dlvr_to { val.validate()? }
		if let Some(ref val) = self.instr_prty { val.validate()? }
		if let Some(ref val) = self.frms_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "frms_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "frms_cd exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.memo_fld {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "memo_fld is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 35 {
					return Err(ValidationError::new(1002, "memo_fld exceeds the maximum length of 35".to_string()));
				}
			}
		}
		if let Some(ref val) = self.rgnl_clr_zone {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rgnl_clr_zone is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "rgnl_clr_zone exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.prt_lctn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prt_lctn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prt_lctn exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.sgntr {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "sgntr is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 70 {
					return Err(ValidationError::new(1002, "sgntr exceeds the maximum length of 70".to_string()));
				}
			}
		}
		Ok(())
	}
}


// ChequeDelivery1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum ChequeDelivery1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MLDB") )]
	CodeMLDB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MLCD") )]
	CodeMLCD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MLFA") )]
	CodeMLFA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRDB") )]
	CodeCRDB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRCD") )]
	CodeCRCD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRFA") )]
	CodeCRFA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUDB") )]
	CodePUDB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUCD") )]
	CodePUCD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUFA") )]
	CodePUFA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RGDB") )]
	CodeRGDB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RGCD") )]
	CodeRGCD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RGFA") )]
	CodeRGFA,
}

impl ChequeDelivery1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ChequeDeliveryMethod1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ChequeDeliveryMethod1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<ChequeDelivery1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ChequeDeliveryMethod1Choice {
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


// ChequeType2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum ChequeType2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CCHQ") )]
	CodeCCHQ,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CCCH") )]
	CodeCCCH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BCHQ") )]
	CodeBCHQ,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DRFT") )]
	CodeDRFT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ELDR") )]
	CodeELDR,
}

impl ChequeType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ClearingChannel2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum ClearingChannel2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "RTGS") )]
	CodeRTGS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RTNS") )]
	CodeRTNS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MPNS") )]
	CodeMPNS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BOOK") )]
	CodeBOOK,
}

impl ClearingChannel2Code {
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


// ClearingSystemIdentification3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ClearingSystemIdentification3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ClearingSystemIdentification3Choice {
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


// ClearingSystemMemberIdentification2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ClearingSystemMemberIdentification2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysId", skip_serializing_if = "Option::is_none") )]
	pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MmbId") )]
	pub mmb_id: String,
}

impl ClearingSystemMemberIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.clr_sys_id { val.validate()? }
		if self.mmb_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "mmb_id is shorter than the minimum length of 1".to_string()));
		}
		if self.mmb_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "mmb_id exceeds the maximum length of 35".to_string()));
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


// CreditDebitCode ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum CreditDebitCode {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRDT") )]
	CodeCRDT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DBIT") )]
	CodeDBIT,
}

impl CreditDebitCode {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CreditTransferMandateData1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CreditTransferMandateData1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtId", skip_serializing_if = "Option::is_none") )]
	pub mndt_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<MandateTypeInformation2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtOfSgntr", skip_serializing_if = "Option::is_none") )]
	pub dt_of_sgntr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtOfVrfctn", skip_serializing_if = "Option::is_none") )]
	pub dt_of_vrfctn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ElctrncSgntr", skip_serializing_if = "Option::is_none") )]
	pub elctrnc_sgntr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrstPmtDt", skip_serializing_if = "Option::is_none") )]
	pub frst_pmt_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FnlPmtDt", skip_serializing_if = "Option::is_none") )]
	pub fnl_pmt_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Frqcy", skip_serializing_if = "Option::is_none") )]
	pub frqcy: Option<Frequency36Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<MandateSetupReason1Choice>,
}

impl CreditTransferMandateData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mndt_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mndt_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mndt_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.elctrnc_sgntr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "elctrnc_sgntr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 10240 {
				return Err(ValidationError::new(1002, "elctrnc_sgntr exceeds the maximum length of 10240".to_string()));
			}
		}
		if let Some(ref val) = self.frqcy { val.validate()? }
		if let Some(ref val) = self.rsn { val.validate()? }
		Ok(())
	}
}


// CreditTransferTransaction61 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CreditTransferTransaction61 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtId") )]
	pub pmt_id: PaymentIdentification6,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none") )]
	pub pmt_tp_inf: Option<PaymentTypeInformation26>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: AmountType4Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XchgRateInf", skip_serializing_if = "Option::is_none") )]
	pub xchg_rate_inf: Option<ExchangeRate1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgBr", skip_serializing_if = "Option::is_none") )]
	pub chrg_br: Option<ChargeBearerType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtRltdInf", skip_serializing_if = "Option::is_none") )]
	pub mndt_rltd_inf: Option<CreditTransferMandateData1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChqInstr", skip_serializing_if = "Option::is_none") )]
	pub chq_instr: Option<Cheque19>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt1Acct", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt1_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt2Acct", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt2_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt3Acct", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt3_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr", skip_serializing_if = "Option::is_none") )]
	pub cdtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrForCdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub instr_for_cdtr_agt: Option<Vec<InstructionForCreditorAgent3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrForDbtrAgt", skip_serializing_if = "Option::is_none") )]
	pub instr_for_dbtr_agt: Option<InstructionForDebtorAgent1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
	pub purp: Option<Purpose2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RgltryRptg", skip_serializing_if = "Option::is_none") )]
	pub rgltry_rptg: Option<Vec<RegulatoryReporting3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tax", skip_serializing_if = "Option::is_none") )]
	pub tax: Option<TaxData1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdRmtInf", skip_serializing_if = "Option::is_none") )]
	pub rltd_rmt_inf: Option<Vec<RemittanceLocation8>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtInf", skip_serializing_if = "Option::is_none") )]
	pub rmt_inf: Option<RemittanceInformation22>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CreditTransferTransaction61 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pmt_id.validate()?;
		if let Some(ref val) = self.pmt_tp_inf { val.validate()? }
		self.amt.validate()?;
		if let Some(ref val) = self.xchg_rate_inf { val.validate()? }
		if let Some(ref val) = self.chrg_br { val.validate()? }
		if let Some(ref val) = self.mndt_rltd_inf { val.validate()? }
		if let Some(ref val) = self.chq_instr { val.validate()? }
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.intrmy_agt1 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt1_acct { val.validate()? }
		if let Some(ref val) = self.intrmy_agt2 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt2_acct { val.validate()? }
		if let Some(ref val) = self.intrmy_agt3 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt3_acct { val.validate()? }
		if let Some(ref val) = self.cdtr_agt { val.validate()? }
		if let Some(ref val) = self.cdtr_agt_acct { val.validate()? }
		if let Some(ref val) = self.cdtr { val.validate()? }
		if let Some(ref val) = self.cdtr_acct { val.validate()? }
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		if let Some(ref vec) = self.instr_for_cdtr_agt { for item in vec { item.validate()? } }
		if let Some(ref val) = self.instr_for_dbtr_agt { val.validate()? }
		if let Some(ref val) = self.purp { val.validate()? }
		if let Some(ref vec) = self.rgltry_rptg { for item in vec { item.validate()? } }
		if let Some(ref val) = self.tax { val.validate()? }
		if let Some(ref vec) = self.rltd_rmt_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.rmt_inf { val.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// CreditTransferTransaction65 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CreditTransferTransaction65 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtId") )]
	pub pmt_id: PaymentIdentification6,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none") )]
	pub pmt_tp_inf: Option<PaymentTypeInformation29>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtCond", skip_serializing_if = "Option::is_none") )]
	pub pmt_cond: Option<PaymentCondition2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none") )]
	pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: AmountType4Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgBr", skip_serializing_if = "Option::is_none") )]
	pub chrg_br: Option<ChargeBearerType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtRltdInf", skip_serializing_if = "Option::is_none") )]
	pub mndt_rltd_inf: Option<CreditTransferMandateData1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChqInstr", skip_serializing_if = "Option::is_none") )]
	pub chq_instr: Option<Cheque19>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt") )]
	pub cdtr_agt: BranchAndFinancialInstitutionIdentification8,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr") )]
	pub cdtr: PartyIdentification272,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrForCdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub instr_for_cdtr_agt: Option<Vec<InstructionForCreditorAgent3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
	pub purp: Option<Purpose2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RgltryRptg", skip_serializing_if = "Option::is_none") )]
	pub rgltry_rptg: Option<Vec<RegulatoryReporting3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tax", skip_serializing_if = "Option::is_none") )]
	pub tax: Option<TaxData1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdRmtInf", skip_serializing_if = "Option::is_none") )]
	pub rltd_rmt_inf: Option<Vec<RemittanceLocation8>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtInf", skip_serializing_if = "Option::is_none") )]
	pub rmt_inf: Option<RemittanceInformation22>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NclsdFile", skip_serializing_if = "Option::is_none") )]
	pub nclsd_file: Option<Vec<Document15>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CreditTransferTransaction65 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pmt_id.validate()?;
		if let Some(ref val) = self.pmt_tp_inf { val.validate()? }
		if let Some(ref val) = self.pmt_cond { val.validate()? }
		if let Some(ref val) = self.reqd_exctn_dt { val.validate()? }
		self.amt.validate()?;
		if let Some(ref val) = self.chrg_br { val.validate()? }
		if let Some(ref val) = self.mndt_rltd_inf { val.validate()? }
		if let Some(ref val) = self.chq_instr { val.validate()? }
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.intrmy_agt1 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt2 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt3 { val.validate()? }
		self.cdtr_agt.validate()?;
		if let Some(ref val) = self.cdtr_agt_acct { val.validate()? }
		self.cdtr.validate()?;
		if let Some(ref val) = self.cdtr_acct { val.validate()? }
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		if let Some(ref vec) = self.instr_for_cdtr_agt { for item in vec { item.validate()? } }
		if let Some(ref val) = self.purp { val.validate()? }
		if let Some(ref vec) = self.rgltry_rptg { for item in vec { item.validate()? } }
		if let Some(ref val) = self.tax { val.validate()? }
		if let Some(ref vec) = self.rltd_rmt_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.rmt_inf { val.validate()? }
		if let Some(ref vec) = self.nclsd_file { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// CreditorReferenceInformation3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CreditorReferenceInformation3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<CreditorReferenceType3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ref", skip_serializing_if = "Option::is_none") )]
	pub ref_attr: Option<String>,
}

impl CreditorReferenceInformation3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.ref_attr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ref_attr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ref_attr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// CreditorReferenceType2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CreditorReferenceType2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl CreditorReferenceType2Choice {
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


// CreditorReferenceType3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CreditorReferenceType3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdOrPrtry") )]
	pub cd_or_prtry: CreditorReferenceType2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl CreditorReferenceType3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd_or_prtry.validate()?;
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


// CurrencyExchange13 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct CurrencyExchange13 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SrcCcy") )]
	pub src_ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrgtCcy") )]
	pub trgt_ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XchgRate") )]
	pub xchg_rate: f64,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitCcy", skip_serializing_if = "Option::is_none") )]
	pub unit_ccy: Option<String>,
}

impl CurrencyExchange13 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.src_ccy) {
			return Err(ValidationError::new(1005, "src_ccy does not match the required pattern".to_string()));
		}
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.trgt_ccy) {
			return Err(ValidationError::new(1005, "trgt_ccy does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.unit_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "unit_ccy does not match the required pattern".to_string()));
			}
		}
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


// DateAndType1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DateAndType1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: DateType2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt") )]
	pub dt: String,
}

impl DateAndType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tp.validate()?;
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


// DatePeriod3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DatePeriod3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt") )]
	pub fr_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt", skip_serializing_if = "Option::is_none") )]
	pub to_dt: Option<String>,
}

impl DatePeriod3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DateType2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DateType2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl DateType2Choice {
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


// DirectDebitTransaction12 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DirectDebitTransaction12 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtRltdInf", skip_serializing_if = "Option::is_none") )]
	pub mndt_rltd_inf: Option<MandateRelatedInformation16>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrSchmeId", skip_serializing_if = "Option::is_none") )]
	pub cdtr_schme_id: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PreNtfctnId", skip_serializing_if = "Option::is_none") )]
	pub pre_ntfctn_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PreNtfctnDt", skip_serializing_if = "Option::is_none") )]
	pub pre_ntfctn_dt: Option<String>,
}

impl DirectDebitTransaction12 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mndt_rltd_inf { val.validate()? }
		if let Some(ref val) = self.cdtr_schme_id { val.validate()? }
		if let Some(ref val) = self.pre_ntfctn_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pre_ntfctn_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "pre_ntfctn_id exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// DirectDebitTransactionInformation32 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DirectDebitTransactionInformation32 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtId") )]
	pub pmt_id: PaymentIdentification6,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none") )]
	pub pmt_tp_inf: Option<PaymentTypeInformation29>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAmt") )]
	pub instd_amt: ActiveOrHistoricCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgBr", skip_serializing_if = "Option::is_none") )]
	pub chrg_br: Option<ChargeBearerType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DrctDbtTx", skip_serializing_if = "Option::is_none") )]
	pub drct_dbt_tx: Option<DirectDebitTransaction12>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt") )]
	pub dbtr_agt: BranchAndFinancialInstitutionIdentification8,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr") )]
	pub dbtr: PartyIdentification272,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct") )]
	pub dbtr_acct: CashAccount40,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrForCdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub instr_for_cdtr_agt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
	pub purp: Option<Purpose2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RgltryRptg", skip_serializing_if = "Option::is_none") )]
	pub rgltry_rptg: Option<Vec<RegulatoryReporting3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tax", skip_serializing_if = "Option::is_none") )]
	pub tax: Option<TaxData1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdRmtInf", skip_serializing_if = "Option::is_none") )]
	pub rltd_rmt_inf: Option<Vec<RemittanceLocation8>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtInf", skip_serializing_if = "Option::is_none") )]
	pub rmt_inf: Option<RemittanceInformation22>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl DirectDebitTransactionInformation32 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pmt_id.validate()?;
		if let Some(ref val) = self.pmt_tp_inf { val.validate()? }
		self.instd_amt.validate()?;
		if let Some(ref val) = self.chrg_br { val.validate()? }
		if let Some(ref val) = self.drct_dbt_tx { val.validate()? }
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		self.dbtr_agt.validate()?;
		if let Some(ref val) = self.dbtr_agt_acct { val.validate()? }
		self.dbtr.validate()?;
		self.dbtr_acct.validate()?;
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.instr_for_cdtr_agt {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "instr_for_cdtr_agt is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "instr_for_cdtr_agt exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.purp { val.validate()? }
		if let Some(ref vec) = self.rgltry_rptg { for item in vec { item.validate()? } }
		if let Some(ref val) = self.tax { val.validate()? }
		if let Some(ref vec) = self.rltd_rmt_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.rmt_inf { val.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// Document15 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Document15 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: DocumentType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IsseDt") )]
	pub isse_dt: DateAndDateTime2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LangCd", skip_serializing_if = "Option::is_none") )]
	pub lang_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Frmt") )]
	pub frmt: DocumentFormat1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FileNm", skip_serializing_if = "Option::is_none") )]
	pub file_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DgtlSgntr", skip_serializing_if = "Option::is_none") )]
	pub dgtl_sgntr: Option<PartyAndSignature4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nclsr") )]
	pub nclsr: String,
}

impl Document15 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tp.validate()?;
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		self.isse_dt.validate()?;
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
		}
		self.frmt.validate()?;
		if let Some(ref val) = self.file_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "file_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "file_nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.dgtl_sgntr { val.validate()? }
		if self.nclsr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nclsr is shorter than the minimum length of 1".to_string()));
		}
		if self.nclsr.chars().count() > 10485760 {
			return Err(ValidationError::new(1002, "nclsr exceeds the maximum length of 10485760".to_string()));
		}
		Ok(())
	}
}


// DocumentAdjustment1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DocumentAdjustment1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none") )]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<String>,
}

impl DocumentAdjustment1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		if let Some(ref val) = self.cdt_dbt_ind { val.validate()? }
		if let Some(ref val) = self.rsn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rsn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "rsn exceeds the maximum length of 4".to_string()));
			}
		}
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


// DocumentAmount1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DocumentAmount1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: DocumentAmountType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}

impl DocumentAmount1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tp.validate()?;
		self.amt.validate()?;
		Ok(())
	}
}


// DocumentAmountType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DocumentAmountType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl DocumentAmountType1Choice {
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


// DocumentFormat1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DocumentFormat1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification1>,
}

impl DocumentFormat1Choice {
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


// DocumentLineIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DocumentLineIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<DocumentLineType1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nb", skip_serializing_if = "Option::is_none") )]
	pub nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdDt", skip_serializing_if = "Option::is_none") )]
	pub rltd_dt: Option<String>,
}

impl DocumentLineIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
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


// DocumentLineInformation2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DocumentLineInformation2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: Vec<DocumentLineIdentification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<RemittanceAmount4>,
}

impl DocumentLineInformation2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.id { item.validate()? }
		if let Some(ref val) = self.desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 2048 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 2048".to_string()));
			}
		}
		if let Some(ref val) = self.amt { val.validate()? }
		Ok(())
	}
}


// DocumentLineType1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DocumentLineType1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdOrPrtry") )]
	pub cd_or_prtry: DocumentLineType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl DocumentLineType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd_or_prtry.validate()?;
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


// DocumentLineType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DocumentLineType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl DocumentLineType1Choice {
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


// DocumentType1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DocumentType1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdOrPrtry") )]
	pub cd_or_prtry: DocumentType2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl DocumentType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd_or_prtry.validate()?;
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


// DocumentType2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct DocumentType2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl DocumentType2Choice {
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


// EquivalentAmount2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct EquivalentAmount2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CcyOfTrf") )]
	pub ccy_of_trf: String,
}

impl EquivalentAmount2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.ccy_of_trf) {
			return Err(ValidationError::new(1005, "ccy_of_trf does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// ExchangeRate1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ExchangeRate1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitCcy", skip_serializing_if = "Option::is_none") )]
	pub unit_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XchgRate", skip_serializing_if = "Option::is_none") )]
	pub xchg_rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RateTp", skip_serializing_if = "Option::is_none") )]
	pub rate_tp: Option<ExchangeRateType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctId", skip_serializing_if = "Option::is_none") )]
	pub ctrct_id: Option<String>,
}

impl ExchangeRate1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.unit_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "unit_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.rate_tp { val.validate()? }
		if let Some(ref val) = self.ctrct_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ctrct_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ctrct_id exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// ExchangeRateType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum ExchangeRateType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SPOT") )]
	CodeSPOT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SALE") )]
	CodeSALE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AGRD") )]
	CodeAGRD,
}

impl ExchangeRateType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FinancialIdentificationSchemeName1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FinancialIdentificationSchemeName1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl FinancialIdentificationSchemeName1Choice {
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


// FinancialInstitutionIdentification23 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FinancialInstitutionIdentification23 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "BICFI", skip_serializing_if = "Option::is_none") )]
	pub bicfi: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none") )]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
	pub pstl_adr: Option<PostalAddress27>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<GenericFinancialIdentification1>,
}

impl FinancialInstitutionIdentification23 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.bicfi {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "bicfi does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.clr_sys_mmb_id { val.validate()? }
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.pstl_adr { val.validate()? }
		if let Some(ref val) = self.othr { val.validate()? }
		Ok(())
	}
}


// Frequency10Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum Frequency10Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NEVR") )]
	CodeNEVR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
	CodeYEAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RATE") )]
	CodeRATE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MIAN") )]
	CodeMIAN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QURT") )]
	CodeQURT,
}

impl Frequency10Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Frequency36Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Frequency36Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<Frequency6Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prd", skip_serializing_if = "Option::is_none") )]
	pub prd: Option<FrequencyPeriod1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtInTm", skip_serializing_if = "Option::is_none") )]
	pub pt_in_tm: Option<FrequencyAndMoment1>,
}

impl Frequency36Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.prd { val.validate()? }
		if let Some(ref val) = self.pt_in_tm { val.validate()? }
		Ok(())
	}
}


// Frequency37Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Frequency37Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<Frequency10Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl Frequency37Choice {
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


// Frequency6Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum Frequency6Code {
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
	#[cfg_attr( feature = "derive_serde", serde(rename = "FRTN") )]
	CodeFRTN,
}

impl Frequency6Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FrequencyAndMoment1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FrequencyAndMoment1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: Frequency6Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtInTm") )]
	pub pt_in_tm: String,
}

impl FrequencyAndMoment1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tp.validate()?;
		let pattern = Regex::new("[0-9]{2}").unwrap();
		if !pattern.is_match(&self.pt_in_tm) {
			return Err(ValidationError::new(1005, "pt_in_tm does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// FrequencyPeriod1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct FrequencyPeriod1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: Frequency6Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CntPerPrd") )]
	pub cnt_per_prd: f64,
}

impl FrequencyPeriod1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tp.validate()?;
		Ok(())
	}
}


// Garnishment4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Garnishment4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: GarnishmentType1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Grnshee", skip_serializing_if = "Option::is_none") )]
	pub grnshee: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrnshmtAdmstr", skip_serializing_if = "Option::is_none") )]
	pub grnshmt_admstr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefNb", skip_serializing_if = "Option::is_none") )]
	pub ref_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none") )]
	pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FmlyMdclInsrncInd", skip_serializing_if = "Option::is_none") )]
	pub fmly_mdcl_insrnc_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MplyeeTermntnInd", skip_serializing_if = "Option::is_none") )]
	pub mplyee_termntn_ind: Option<bool>,
}

impl Garnishment4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tp.validate()?;
		if let Some(ref val) = self.grnshee { val.validate()? }
		if let Some(ref val) = self.grnshmt_admstr { val.validate()? }
		if let Some(ref val) = self.ref_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ref_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "ref_nb exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.rmtd_amt { val.validate()? }
		Ok(())
	}
}


// GarnishmentType1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GarnishmentType1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdOrPrtry") )]
	pub cd_or_prtry: GarnishmentType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GarnishmentType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd_or_prtry.validate()?;
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


// GarnishmentType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GarnishmentType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl GarnishmentType1Choice {
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


// GenericFinancialIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GenericFinancialIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<FinancialIdentificationSchemeName1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericFinancialIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
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


// GroupHeader110 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GroupHeader110 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
	pub cre_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Authstn", skip_serializing_if = "Option::is_none") )]
	pub authstn: Option<Vec<Authorisation1Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitgPty", skip_serializing_if = "Option::is_none") )]
	pub initg_pty: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none") )]
	pub instg_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none") )]
	pub instd_agt: Option<BranchAndFinancialInstitutionIdentification8>,
}

impl GroupHeader110 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref vec) = self.authstn { for item in vec { item.validate()? } }
		if let Some(ref val) = self.initg_pty { val.validate()? }
		if let Some(ref val) = self.instg_agt { val.validate()? }
		if let Some(ref val) = self.instd_agt { val.validate()? }
		Ok(())
	}
}


// GroupHeader111 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GroupHeader111 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
	pub cre_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitgPty") )]
	pub initg_pty: PartyIdentification272,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FwdgAgt", skip_serializing_if = "Option::is_none") )]
	pub fwdg_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
}

impl GroupHeader111 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		self.initg_pty.validate()?;
		if let Some(ref val) = self.fwdg_agt { val.validate()? }
		if let Some(ref val) = self.dbtr_agt { val.validate()? }
		if let Some(ref val) = self.cdtr_agt { val.validate()? }
		Ok(())
	}
}


// GroupHeader112 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GroupHeader112 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
	pub cre_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxs") )]
	pub nb_of_txs: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrlSum", skip_serializing_if = "Option::is_none") )]
	pub ctrl_sum: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitgPty") )]
	pub initg_pty: PartyIdentification272,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FwdgAgt", skip_serializing_if = "Option::is_none") )]
	pub fwdg_agt: Option<BranchAndFinancialInstitutionIdentification8>,
}

impl GroupHeader112 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		let pattern = Regex::new("[0-9]{1,15}").unwrap();
		if !pattern.is_match(&self.nb_of_txs) {
			return Err(ValidationError::new(1005, "nb_of_txs does not match the required pattern".to_string()));
		}
		self.initg_pty.validate()?;
		if let Some(ref val) = self.fwdg_agt { val.validate()? }
		Ok(())
	}
}


// GroupHeader114 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GroupHeader114 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
	pub cre_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Authstn", skip_serializing_if = "Option::is_none") )]
	pub authstn: Option<Vec<Authorisation1Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxs") )]
	pub nb_of_txs: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrlSum", skip_serializing_if = "Option::is_none") )]
	pub ctrl_sum: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitgPty") )]
	pub initg_pty: PartyIdentification272,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FwdgAgt", skip_serializing_if = "Option::is_none") )]
	pub fwdg_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitnSrc", skip_serializing_if = "Option::is_none") )]
	pub initn_src: Option<PaymentInitiationSource1>,
}

impl GroupHeader114 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref vec) = self.authstn { for item in vec { item.validate()? } }
		let pattern = Regex::new("[0-9]{1,15}").unwrap();
		if !pattern.is_match(&self.nb_of_txs) {
			return Err(ValidationError::new(1005, "nb_of_txs does not match the required pattern".to_string()));
		}
		self.initg_pty.validate()?;
		if let Some(ref val) = self.fwdg_agt { val.validate()? }
		if let Some(ref val) = self.initn_src { val.validate()? }
		Ok(())
	}
}


// GroupHeader118 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GroupHeader118 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
	pub cre_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Authstn", skip_serializing_if = "Option::is_none") )]
	pub authstn: Option<Vec<Authorisation1Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxs") )]
	pub nb_of_txs: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrlSum", skip_serializing_if = "Option::is_none") )]
	pub ctrl_sum: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitgPty") )]
	pub initg_pty: PartyIdentification272,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FwdgAgt", skip_serializing_if = "Option::is_none") )]
	pub fwdg_agt: Option<BranchAndFinancialInstitutionIdentification8>,
}

impl GroupHeader118 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref vec) = self.authstn { for item in vec { item.validate()? } }
		let pattern = Regex::new("[0-9]{1,15}").unwrap();
		if !pattern.is_match(&self.nb_of_txs) {
			return Err(ValidationError::new(1005, "nb_of_txs does not match the required pattern".to_string()));
		}
		self.initg_pty.validate()?;
		if let Some(ref val) = self.fwdg_agt { val.validate()? }
		Ok(())
	}
}


// GroupHeader124 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GroupHeader124 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
	pub cre_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Authstn", skip_serializing_if = "Option::is_none") )]
	pub authstn: Option<Vec<Authorisation1Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxs") )]
	pub nb_of_txs: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrlSum", skip_serializing_if = "Option::is_none") )]
	pub ctrl_sum: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrpRvsl", skip_serializing_if = "Option::is_none") )]
	pub grp_rvsl: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitgPty", skip_serializing_if = "Option::is_none") )]
	pub initg_pty: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FwdgAgt", skip_serializing_if = "Option::is_none") )]
	pub fwdg_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
}

impl GroupHeader124 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref vec) = self.authstn { for item in vec { item.validate()? } }
		let pattern = Regex::new("[0-9]{1,15}").unwrap();
		if !pattern.is_match(&self.nb_of_txs) {
			return Err(ValidationError::new(1005, "nb_of_txs does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.initg_pty { val.validate()? }
		if let Some(ref val) = self.fwdg_agt { val.validate()? }
		if let Some(ref val) = self.dbtr_agt { val.validate()? }
		if let Some(ref val) = self.cdtr_agt { val.validate()? }
		Ok(())
	}
}


// GroupHeader128 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct GroupHeader128 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
	pub cre_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitgPty", skip_serializing_if = "Option::is_none") )]
	pub initg_pty: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FwdgAgt", skip_serializing_if = "Option::is_none") )]
	pub fwdg_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
}

impl GroupHeader128 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.initg_pty { val.validate()? }
		if let Some(ref val) = self.fwdg_agt { val.validate()? }
		if let Some(ref val) = self.dbtr_agt { val.validate()? }
		if let Some(ref val) = self.cdtr_agt { val.validate()? }
		Ok(())
	}
}


// InstructionForCreditorAgent3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct InstructionForCreditorAgent3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrInf", skip_serializing_if = "Option::is_none") )]
	pub instr_inf: Option<String>,
}

impl InstructionForCreditorAgent3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.instr_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "instr_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "instr_inf exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// InstructionForDebtorAgent1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct InstructionForDebtorAgent1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrInf", skip_serializing_if = "Option::is_none") )]
	pub instr_inf: Option<String>,
}

impl InstructionForDebtorAgent1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.instr_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "instr_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "instr_inf exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// LocalInstrument2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct LocalInstrument2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl LocalInstrument2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 35".to_string()));
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


// Mandate20 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Mandate20 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtId") )]
	pub mndt_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtReqId", skip_serializing_if = "Option::is_none") )]
	pub mndt_req_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Authntcn", skip_serializing_if = "Option::is_none") )]
	pub authntcn: Option<MandateAuthentication1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<MandateTypeInformation2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ocrncs", skip_serializing_if = "Option::is_none") )]
	pub ocrncs: Option<MandateOccurrences5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrckgInd") )]
	pub trckg_ind: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrstColltnAmt", skip_serializing_if = "Option::is_none") )]
	pub frst_colltn_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ColltnAmt", skip_serializing_if = "Option::is_none") )]
	pub colltn_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MaxAmt", skip_serializing_if = "Option::is_none") )]
	pub max_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Adjstmnt", skip_serializing_if = "Option::is_none") )]
	pub adjstmnt: Option<MandateAdjustment1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<MandateSetupReason1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrSchmeId", skip_serializing_if = "Option::is_none") )]
	pub cdtr_schme_id: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr") )]
	pub cdtr: PartyIdentification272,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr") )]
	pub dbtr: PartyIdentification272,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt") )]
	pub dbtr_agt: BranchAndFinancialInstitutionIdentification8,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtRef", skip_serializing_if = "Option::is_none") )]
	pub mndt_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RfrdDoc", skip_serializing_if = "Option::is_none") )]
	pub rfrd_doc: Option<Vec<ReferredMandateDocument2>>,
}

impl Mandate20 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.mndt_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "mndt_id is shorter than the minimum length of 1".to_string()));
		}
		if self.mndt_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "mndt_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.mndt_req_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mndt_req_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mndt_req_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.authntcn { val.validate()? }
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.ocrncs { val.validate()? }
		if let Some(ref val) = self.frst_colltn_amt { val.validate()? }
		if let Some(ref val) = self.colltn_amt { val.validate()? }
		if let Some(ref val) = self.max_amt { val.validate()? }
		if let Some(ref val) = self.adjstmnt { val.validate()? }
		if let Some(ref val) = self.rsn { val.validate()? }
		if let Some(ref val) = self.cdtr_schme_id { val.validate()? }
		self.cdtr.validate()?;
		if let Some(ref val) = self.cdtr_acct { val.validate()? }
		if let Some(ref val) = self.cdtr_agt { val.validate()? }
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		self.dbtr.validate()?;
		if let Some(ref val) = self.dbtr_acct { val.validate()? }
		self.dbtr_agt.validate()?;
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.mndt_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mndt_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mndt_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.rfrd_doc { for item in vec { item.validate()? } }
		Ok(())
	}
}


// Mandate21 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Mandate21 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtId") )]
	pub mndt_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtReqId", skip_serializing_if = "Option::is_none") )]
	pub mndt_req_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Authntcn", skip_serializing_if = "Option::is_none") )]
	pub authntcn: Option<MandateAuthentication1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<MandateTypeInformation2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ocrncs", skip_serializing_if = "Option::is_none") )]
	pub ocrncs: Option<MandateOccurrences5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrckgInd") )]
	pub trckg_ind: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrstColltnAmt", skip_serializing_if = "Option::is_none") )]
	pub frst_colltn_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ColltnAmt", skip_serializing_if = "Option::is_none") )]
	pub colltn_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MaxAmt", skip_serializing_if = "Option::is_none") )]
	pub max_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Adjstmnt", skip_serializing_if = "Option::is_none") )]
	pub adjstmnt: Option<MandateAdjustment1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<MandateSetupReason1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrSchmeId", skip_serializing_if = "Option::is_none") )]
	pub cdtr_schme_id: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr", skip_serializing_if = "Option::is_none") )]
	pub cdtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr", skip_serializing_if = "Option::is_none") )]
	pub dbtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtRef", skip_serializing_if = "Option::is_none") )]
	pub mndt_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RfrdDoc", skip_serializing_if = "Option::is_none") )]
	pub rfrd_doc: Option<Vec<ReferredMandateDocument2>>,
}

impl Mandate21 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.mndt_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "mndt_id is shorter than the minimum length of 1".to_string()));
		}
		if self.mndt_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "mndt_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.mndt_req_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mndt_req_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mndt_req_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.authntcn { val.validate()? }
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.ocrncs { val.validate()? }
		if let Some(ref val) = self.frst_colltn_amt { val.validate()? }
		if let Some(ref val) = self.colltn_amt { val.validate()? }
		if let Some(ref val) = self.max_amt { val.validate()? }
		if let Some(ref val) = self.adjstmnt { val.validate()? }
		if let Some(ref val) = self.rsn { val.validate()? }
		if let Some(ref val) = self.cdtr_schme_id { val.validate()? }
		if let Some(ref val) = self.cdtr { val.validate()? }
		if let Some(ref val) = self.cdtr_acct { val.validate()? }
		if let Some(ref val) = self.cdtr_agt { val.validate()? }
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		if let Some(ref val) = self.dbtr { val.validate()? }
		if let Some(ref val) = self.dbtr_acct { val.validate()? }
		if let Some(ref val) = self.dbtr_agt { val.validate()? }
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.mndt_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mndt_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mndt_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.rfrd_doc { for item in vec { item.validate()? } }
		Ok(())
	}
}


// Mandate22 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Mandate22 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtId", skip_serializing_if = "Option::is_none") )]
	pub mndt_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtReqId", skip_serializing_if = "Option::is_none") )]
	pub mndt_req_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Authntcn", skip_serializing_if = "Option::is_none") )]
	pub authntcn: Option<MandateAuthentication1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<MandateTypeInformation2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ocrncs", skip_serializing_if = "Option::is_none") )]
	pub ocrncs: Option<MandateOccurrences5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrckgInd") )]
	pub trckg_ind: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrstColltnAmt", skip_serializing_if = "Option::is_none") )]
	pub frst_colltn_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ColltnAmt", skip_serializing_if = "Option::is_none") )]
	pub colltn_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MaxAmt", skip_serializing_if = "Option::is_none") )]
	pub max_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Adjstmnt", skip_serializing_if = "Option::is_none") )]
	pub adjstmnt: Option<MandateAdjustment1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<MandateSetupReason1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrSchmeId", skip_serializing_if = "Option::is_none") )]
	pub cdtr_schme_id: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr") )]
	pub cdtr: PartyIdentification272,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr") )]
	pub dbtr: PartyIdentification272,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt") )]
	pub dbtr_agt: BranchAndFinancialInstitutionIdentification8,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtRef", skip_serializing_if = "Option::is_none") )]
	pub mndt_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RfrdDoc", skip_serializing_if = "Option::is_none") )]
	pub rfrd_doc: Option<Vec<ReferredMandateDocument2>>,
}

impl Mandate22 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mndt_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mndt_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mndt_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.mndt_req_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mndt_req_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mndt_req_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.authntcn { val.validate()? }
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.ocrncs { val.validate()? }
		if let Some(ref val) = self.frst_colltn_amt { val.validate()? }
		if let Some(ref val) = self.colltn_amt { val.validate()? }
		if let Some(ref val) = self.max_amt { val.validate()? }
		if let Some(ref val) = self.adjstmnt { val.validate()? }
		if let Some(ref val) = self.rsn { val.validate()? }
		if let Some(ref val) = self.cdtr_schme_id { val.validate()? }
		self.cdtr.validate()?;
		if let Some(ref val) = self.cdtr_acct { val.validate()? }
		if let Some(ref val) = self.cdtr_agt { val.validate()? }
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		self.dbtr.validate()?;
		if let Some(ref val) = self.dbtr_acct { val.validate()? }
		self.dbtr_agt.validate()?;
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.mndt_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mndt_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mndt_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.rfrd_doc { for item in vec { item.validate()? } }
		Ok(())
	}
}


// Mandate23 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Mandate23 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtId", skip_serializing_if = "Option::is_none") )]
	pub mndt_id: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtReqId") )]
	pub mndt_req_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Authntcn", skip_serializing_if = "Option::is_none") )]
	pub authntcn: Option<MandateAuthentication1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<MandateTypeInformation2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ocrncs", skip_serializing_if = "Option::is_none") )]
	pub ocrncs: Option<MandateOccurrences5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrckgInd") )]
	pub trckg_ind: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrstColltnAmt", skip_serializing_if = "Option::is_none") )]
	pub frst_colltn_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ColltnAmt", skip_serializing_if = "Option::is_none") )]
	pub colltn_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MaxAmt", skip_serializing_if = "Option::is_none") )]
	pub max_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Adjstmnt", skip_serializing_if = "Option::is_none") )]
	pub adjstmnt: Option<MandateAdjustment1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<MandateSetupReason1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrSchmeId", skip_serializing_if = "Option::is_none") )]
	pub cdtr_schme_id: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr") )]
	pub cdtr: PartyIdentification272,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr") )]
	pub dbtr: PartyIdentification272,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt") )]
	pub dbtr_agt: BranchAndFinancialInstitutionIdentification8,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtRef", skip_serializing_if = "Option::is_none") )]
	pub mndt_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RfrdDoc", skip_serializing_if = "Option::is_none") )]
	pub rfrd_doc: Option<Vec<ReferredMandateDocument2>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl Mandate23 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.mndt_id {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "mndt_id is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 35 {
					return Err(ValidationError::new(1002, "mndt_id exceeds the maximum length of 35".to_string()));
				}
			}
		}
		if self.mndt_req_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "mndt_req_id is shorter than the minimum length of 1".to_string()));
		}
		if self.mndt_req_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "mndt_req_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.authntcn { val.validate()? }
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.ocrncs { val.validate()? }
		if let Some(ref val) = self.frst_colltn_amt { val.validate()? }
		if let Some(ref val) = self.colltn_amt { val.validate()? }
		if let Some(ref val) = self.max_amt { val.validate()? }
		if let Some(ref val) = self.adjstmnt { val.validate()? }
		if let Some(ref val) = self.rsn { val.validate()? }
		if let Some(ref val) = self.cdtr_schme_id { val.validate()? }
		self.cdtr.validate()?;
		if let Some(ref val) = self.cdtr_acct { val.validate()? }
		if let Some(ref val) = self.cdtr_agt { val.validate()? }
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		self.dbtr.validate()?;
		if let Some(ref val) = self.dbtr_acct { val.validate()? }
		self.dbtr_agt.validate()?;
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.mndt_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mndt_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mndt_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.rfrd_doc { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// MandateAcceptance8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MandateAcceptance8 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgInf", skip_serializing_if = "Option::is_none") )]
	pub orgnl_msg_inf: Option<OriginalMessageInformation1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AccptncRslt") )]
	pub accptnc_rslt: AcceptanceResult6,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMndt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_mndt: Option<OriginalMandate11Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl MandateAcceptance8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_msg_inf { val.validate()? }
		self.accptnc_rslt.validate()?;
		if let Some(ref val) = self.orgnl_mndt { val.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// MandateAdjustment1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MandateAdjustment1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtAdjstmntRuleInd") )]
	pub dt_adjstmnt_rule_ind: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctgy", skip_serializing_if = "Option::is_none") )]
	pub ctgy: Option<Frequency37Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
	pub rate: Option<f64>,
}

impl MandateAdjustment1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ctgy { val.validate()? }
		if let Some(ref val) = self.amt { val.validate()? }
		Ok(())
	}
}


// MandateAmendment8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MandateAmendment8 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgInf", skip_serializing_if = "Option::is_none") )]
	pub orgnl_msg_inf: Option<OriginalMessageInformation1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmdmntRsn") )]
	pub amdmnt_rsn: MandateAmendmentReason3,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mndt") )]
	pub mndt: Mandate21,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMndt") )]
	pub orgnl_mndt: OriginalMandate10Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl MandateAmendment8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_msg_inf { val.validate()? }
		self.amdmnt_rsn.validate()?;
		self.mndt.validate()?;
		self.orgnl_mndt.validate()?;
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// MandateAmendmentReason3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MandateAmendmentReason3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Orgtr", skip_serializing_if = "Option::is_none") )]
	pub orgtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn") )]
	pub rsn: MandateReason1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<String>>,
}

impl MandateAmendmentReason3 {
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


// MandateAuthentication1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MandateAuthentication1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgAuthntcnCd", skip_serializing_if = "Option::is_none") )]
	pub msg_authntcn_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Chanl", skip_serializing_if = "Option::is_none") )]
	pub chanl: Option<AuthenticationChannel1Choice>,
}

impl MandateAuthentication1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.msg_authntcn_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "msg_authntcn_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "msg_authntcn_cd exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.chanl { val.validate()? }
		Ok(())
	}
}


// MandateCancellation8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MandateCancellation8 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgInf", skip_serializing_if = "Option::is_none") )]
	pub orgnl_msg_inf: Option<OriginalMessageInformation1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CxlRsn") )]
	pub cxl_rsn: MandateCancellationReason2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMndt") )]
	pub orgnl_mndt: OriginalMandate10Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl MandateCancellation8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_msg_inf { val.validate()? }
		self.cxl_rsn.validate()?;
		self.orgnl_mndt.validate()?;
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// MandateCancellationReason2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MandateCancellationReason2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Orgtr", skip_serializing_if = "Option::is_none") )]
	pub orgtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn") )]
	pub rsn: MandateReason1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<String>>,
}

impl MandateCancellationReason2 {
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


// MandateClassification1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MandateClassification1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<MandateClassification1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl MandateClassification1Choice {
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


// MandateClassification1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum MandateClassification1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FIXE") )]
	CodeFIXE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USGB") )]
	CodeUSGB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VARI") )]
	CodeVARI,
}

impl MandateClassification1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// MandateCopy4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MandateCopy4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgInf", skip_serializing_if = "Option::is_none") )]
	pub orgnl_msg_inf: Option<OriginalMessageInformation1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMndt") )]
	pub orgnl_mndt: OriginalMandate10Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtSts", skip_serializing_if = "Option::is_none") )]
	pub mndt_sts: Option<MandateStatus1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl MandateCopy4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_msg_inf { val.validate()? }
		self.orgnl_mndt.validate()?;
		if let Some(ref val) = self.mndt_sts { val.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// MandateOccurrences5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MandateOccurrences5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SeqTp") )]
	pub seq_tp: SequenceType2Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Frqcy", skip_serializing_if = "Option::is_none") )]
	pub frqcy: Option<Frequency36Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Drtn", skip_serializing_if = "Option::is_none") )]
	pub drtn: Option<DatePeriod3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrstColltnDt", skip_serializing_if = "Option::is_none") )]
	pub frst_colltn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FnlColltnDt", skip_serializing_if = "Option::is_none") )]
	pub fnl_colltn_dt: Option<String>,
}

impl MandateOccurrences5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.seq_tp.validate()?;
		if let Some(ref val) = self.frqcy { val.validate()? }
		if let Some(ref val) = self.drtn { val.validate()? }
		Ok(())
	}
}


// MandateReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MandateReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl MandateReason1Choice {
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


// MandateRelatedData3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MandateRelatedData3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DrctDbtMndt", skip_serializing_if = "Option::is_none") )]
	pub drct_dbt_mndt: Option<MandateRelatedInformation16>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtTrfMndt", skip_serializing_if = "Option::is_none") )]
	pub cdt_trf_mndt: Option<CreditTransferMandateData1>,
}

impl MandateRelatedData3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.drct_dbt_mndt { val.validate()? }
		if let Some(ref val) = self.cdt_trf_mndt { val.validate()? }
		Ok(())
	}
}


// MandateRelatedInformation16 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MandateRelatedInformation16 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtId", skip_serializing_if = "Option::is_none") )]
	pub mndt_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtOfSgntr", skip_serializing_if = "Option::is_none") )]
	pub dt_of_sgntr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmdmntInd", skip_serializing_if = "Option::is_none") )]
	pub amdmnt_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmdmntInfDtls", skip_serializing_if = "Option::is_none") )]
	pub amdmnt_inf_dtls: Option<AmendmentInformationDetails15>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ElctrncSgntr", skip_serializing_if = "Option::is_none") )]
	pub elctrnc_sgntr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrstColltnDt", skip_serializing_if = "Option::is_none") )]
	pub frst_colltn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FnlColltnDt", skip_serializing_if = "Option::is_none") )]
	pub fnl_colltn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Frqcy", skip_serializing_if = "Option::is_none") )]
	pub frqcy: Option<Frequency36Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<MandateSetupReason1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrckgDays", skip_serializing_if = "Option::is_none") )]
	pub trckg_days: Option<String>,
}

impl MandateRelatedInformation16 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mndt_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mndt_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mndt_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.amdmnt_inf_dtls { val.validate()? }
		if let Some(ref val) = self.elctrnc_sgntr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "elctrnc_sgntr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 1025 {
				return Err(ValidationError::new(1002, "elctrnc_sgntr exceeds the maximum length of 1025".to_string()));
			}
		}
		if let Some(ref val) = self.frqcy { val.validate()? }
		if let Some(ref val) = self.rsn { val.validate()? }
		if let Some(ref val) = self.trckg_days {
			let pattern = Regex::new("[0-9]{2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "trckg_days does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// MandateSetupReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MandateSetupReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl MandateSetupReason1Choice {
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
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 70".to_string()));
			}
		}
		Ok(())
	}
}


// MandateStatus1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MandateStatus1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl MandateStatus1Choice {
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


// MandateSuspension4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MandateSuspension4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SspnsnReqId") )]
	pub sspnsn_req_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgInf", skip_serializing_if = "Option::is_none") )]
	pub orgnl_msg_inf: Option<OriginalMessageInformation1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SspnsnRsn") )]
	pub sspnsn_rsn: MandateSuspensionReason3,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMndt") )]
	pub orgnl_mndt: OriginalMandate10Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl MandateSuspension4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.sspnsn_req_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "sspnsn_req_id is shorter than the minimum length of 1".to_string()));
		}
		if self.sspnsn_req_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "sspnsn_req_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.orgnl_msg_inf { val.validate()? }
		self.sspnsn_rsn.validate()?;
		self.orgnl_mndt.validate()?;
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// MandateSuspensionReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MandateSuspensionReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl MandateSuspensionReason1Choice {
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


// MandateSuspensionReason3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MandateSuspensionReason3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Orgtr", skip_serializing_if = "Option::is_none") )]
	pub orgtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn") )]
	pub rsn: MandateSuspensionReason1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<String>>,
}

impl MandateSuspensionReason3 {
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


// MandateTypeInformation2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct MandateTypeInformation2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none") )]
	pub svc_lvl: Option<ServiceLevel8Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none") )]
	pub lcl_instrm: Option<LocalInstrument2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none") )]
	pub ctgy_purp: Option<CategoryPurpose1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Clssfctn", skip_serializing_if = "Option::is_none") )]
	pub clssfctn: Option<MandateClassification1Choice>,
}

impl MandateTypeInformation2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.svc_lvl { val.validate()? }
		if let Some(ref val) = self.lcl_instrm { val.validate()? }
		if let Some(ref val) = self.ctgy_purp { val.validate()? }
		if let Some(ref val) = self.clssfctn { val.validate()? }
		Ok(())
	}
}


// NameAndAddress18 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct NameAndAddress18 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Adr") )]
	pub adr: PostalAddress27,
}

impl NameAndAddress18 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 140 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
		}
		self.adr.validate()?;
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


// NumberOfTransactionsPerStatus5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct NumberOfTransactionsPerStatus5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtldNbOfTxs") )]
	pub dtld_nb_of_txs: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtldSts") )]
	pub dtld_sts: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtldCtrlSum", skip_serializing_if = "Option::is_none") )]
	pub dtld_ctrl_sum: Option<f64>,
}

impl NumberOfTransactionsPerStatus5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{1,15}").unwrap();
		if !pattern.is_match(&self.dtld_nb_of_txs) {
			return Err(ValidationError::new(1005, "dtld_nb_of_txs does not match the required pattern".to_string()));
		}
		if self.dtld_sts.chars().count() < 1 {
			return Err(ValidationError::new(1001, "dtld_sts is shorter than the minimum length of 1".to_string()));
		}
		if self.dtld_sts.chars().count() > 4 {
			return Err(ValidationError::new(1002, "dtld_sts exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// OrganisationIdentification39 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OrganisationIdentification39 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
	pub any_bic: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<GenericOrganisationIdentification3>>,
}

impl OrganisationIdentification39 {
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


// OriginalGroupHeader20 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OriginalGroupHeader20 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgId") )]
	pub orgnl_msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgNmId") )]
	pub orgnl_msg_nm_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none") )]
	pub orgnl_cre_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RvslRsnInf", skip_serializing_if = "Option::is_none") )]
	pub rvsl_rsn_inf: Option<Vec<PaymentReversalReason10>>,
}

impl OriginalGroupHeader20 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.orgnl_msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_msg_id exceeds the maximum length of 35".to_string()));
		}
		if self.orgnl_msg_nm_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_msg_nm_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_msg_nm_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_msg_nm_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref vec) = self.rvsl_rsn_inf { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OriginalGroupHeader22 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OriginalGroupHeader22 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgId") )]
	pub orgnl_msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgNmId") )]
	pub orgnl_msg_nm_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none") )]
	pub orgnl_cre_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlNbOfTxs", skip_serializing_if = "Option::is_none") )]
	pub orgnl_nb_of_txs: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCtrlSum", skip_serializing_if = "Option::is_none") )]
	pub orgnl_ctrl_sum: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrpSts", skip_serializing_if = "Option::is_none") )]
	pub grp_sts: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsRsnInf", skip_serializing_if = "Option::is_none") )]
	pub sts_rsn_inf: Option<Vec<StatusReasonInformation14>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxsPerSts", skip_serializing_if = "Option::is_none") )]
	pub nb_of_txs_per_sts: Option<Vec<NumberOfTransactionsPerStatus5>>,
}

impl OriginalGroupHeader22 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.orgnl_msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_msg_id exceeds the maximum length of 35".to_string()));
		}
		if self.orgnl_msg_nm_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_msg_nm_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_msg_nm_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_msg_nm_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.orgnl_nb_of_txs {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_nb_of_txs does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.grp_sts {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "grp_sts is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "grp_sts exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref vec) = self.sts_rsn_inf { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.nb_of_txs_per_sts { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OriginalGroupInformation32 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OriginalGroupInformation32 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgId") )]
	pub orgnl_msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgNmId") )]
	pub orgnl_msg_nm_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none") )]
	pub orgnl_cre_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlNbOfTxs", skip_serializing_if = "Option::is_none") )]
	pub orgnl_nb_of_txs: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCtrlSum", skip_serializing_if = "Option::is_none") )]
	pub orgnl_ctrl_sum: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrpSts", skip_serializing_if = "Option::is_none") )]
	pub grp_sts: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsRsnInf", skip_serializing_if = "Option::is_none") )]
	pub sts_rsn_inf: Option<Vec<StatusReasonInformation14>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxsPerSts", skip_serializing_if = "Option::is_none") )]
	pub nb_of_txs_per_sts: Option<Vec<NumberOfTransactionsPerStatus5>>,
}

impl OriginalGroupInformation32 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.orgnl_msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_msg_id exceeds the maximum length of 35".to_string()));
		}
		if self.orgnl_msg_nm_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_msg_nm_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_msg_nm_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_msg_nm_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.orgnl_nb_of_txs {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_nb_of_txs does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.grp_sts {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "grp_sts is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "grp_sts exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref vec) = self.sts_rsn_inf { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.nb_of_txs_per_sts { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OriginalMandate10Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OriginalMandate10Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMndtId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_mndt_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMndt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_mndt: Option<Mandate20>,
}

impl OriginalMandate10Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_mndt_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_mndt_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_mndt_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_mndt { val.validate()? }
		Ok(())
	}
}


// OriginalMandate11Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OriginalMandate11Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMndtId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_mndt_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMndt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_mndt: Option<Mandate22>,
}

impl OriginalMandate11Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_mndt_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_mndt_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_mndt_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_mndt { val.validate()? }
		Ok(())
	}
}


// OriginalMessageInformation1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OriginalMessageInformation1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgNmId") )]
	pub msg_nm_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
	pub cre_dt_tm: Option<String>,
}

impl OriginalMessageInformation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if self.msg_nm_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_nm_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_nm_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_nm_id exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// OriginalPaymentInstruction47 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OriginalPaymentInstruction47 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlPmtInfId") )]
	pub orgnl_pmt_inf_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlNbOfTxs", skip_serializing_if = "Option::is_none") )]
	pub orgnl_nb_of_txs: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCtrlSum", skip_serializing_if = "Option::is_none") )]
	pub orgnl_ctrl_sum: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtInfSts", skip_serializing_if = "Option::is_none") )]
	pub pmt_inf_sts: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsRsnInf", skip_serializing_if = "Option::is_none") )]
	pub sts_rsn_inf: Option<Vec<StatusReasonInformation14>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxsPerSts", skip_serializing_if = "Option::is_none") )]
	pub nb_of_txs_per_sts: Option<Vec<NumberOfTransactionsPerStatus5>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxInfAndSts", skip_serializing_if = "Option::is_none") )]
	pub tx_inf_and_sts: Option<Vec<PaymentTransaction150>>,
}

impl OriginalPaymentInstruction47 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.orgnl_pmt_inf_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_pmt_inf_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_pmt_inf_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_pmt_inf_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.orgnl_nb_of_txs {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_nb_of_txs does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pmt_inf_sts {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pmt_inf_sts is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "pmt_inf_sts exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref vec) = self.sts_rsn_inf { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.nb_of_txs_per_sts { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.tx_inf_and_sts { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OriginalPaymentInstruction50 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OriginalPaymentInstruction50 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RvslPmtInfId", skip_serializing_if = "Option::is_none") )]
	pub rvsl_pmt_inf_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlPmtInfId") )]
	pub orgnl_pmt_inf_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlNbOfTxs", skip_serializing_if = "Option::is_none") )]
	pub orgnl_nb_of_txs: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCtrlSum", skip_serializing_if = "Option::is_none") )]
	pub orgnl_ctrl_sum: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BtchBookg", skip_serializing_if = "Option::is_none") )]
	pub btch_bookg: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtInfRvsl", skip_serializing_if = "Option::is_none") )]
	pub pmt_inf_rvsl: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RvslRsnInf", skip_serializing_if = "Option::is_none") )]
	pub rvsl_rsn_inf: Option<Vec<PaymentReversalReason10>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxInf", skip_serializing_if = "Option::is_none") )]
	pub tx_inf: Option<Vec<PaymentTransaction156>>,
}

impl OriginalPaymentInstruction50 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rvsl_pmt_inf_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rvsl_pmt_inf_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "rvsl_pmt_inf_id exceeds the maximum length of 35".to_string()));
			}
		}
		if self.orgnl_pmt_inf_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_pmt_inf_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_pmt_inf_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_pmt_inf_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.orgnl_nb_of_txs {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_nb_of_txs does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.rvsl_rsn_inf { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.tx_inf { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OriginalPaymentInstruction51 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OriginalPaymentInstruction51 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlPmtInfId") )]
	pub orgnl_pmt_inf_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlNbOfTxs", skip_serializing_if = "Option::is_none") )]
	pub orgnl_nb_of_txs: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCtrlSum", skip_serializing_if = "Option::is_none") )]
	pub orgnl_ctrl_sum: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtInfSts", skip_serializing_if = "Option::is_none") )]
	pub pmt_inf_sts: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsRsnInf", skip_serializing_if = "Option::is_none") )]
	pub sts_rsn_inf: Option<Vec<StatusReasonInformation14>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxsPerSts", skip_serializing_if = "Option::is_none") )]
	pub nb_of_txs_per_sts: Option<Vec<NumberOfTransactionsPerStatus5>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxInfAndSts", skip_serializing_if = "Option::is_none") )]
	pub tx_inf_and_sts: Option<Vec<PaymentTransaction160>>,
}

impl OriginalPaymentInstruction51 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.orgnl_pmt_inf_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_pmt_inf_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_pmt_inf_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_pmt_inf_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.orgnl_nb_of_txs {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_nb_of_txs does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pmt_inf_sts {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pmt_inf_sts is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "pmt_inf_sts exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref vec) = self.sts_rsn_inf { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.nb_of_txs_per_sts { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.tx_inf_and_sts { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OriginalTransactionReference40 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OriginalTransactionReference40 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<AmountType4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none") )]
	pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XpryDt", skip_serializing_if = "Option::is_none") )]
	pub xpry_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtCond", skip_serializing_if = "Option::is_none") )]
	pub pmt_cond: Option<PaymentCondition2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none") )]
	pub pmt_tp_inf: Option<PaymentTypeInformation29>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtMtd", skip_serializing_if = "Option::is_none") )]
	pub pmt_mtd: Option<PaymentMethod4Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtRltdInf", skip_serializing_if = "Option::is_none") )]
	pub mndt_rltd_inf: Option<CreditTransferMandateData1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtInf", skip_serializing_if = "Option::is_none") )]
	pub rmt_inf: Option<RemittanceInformation22>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NclsdFile", skip_serializing_if = "Option::is_none") )]
	pub nclsd_file: Option<Vec<Document15>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr", skip_serializing_if = "Option::is_none") )]
	pub dbtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt") )]
	pub cdtr_agt: BranchAndFinancialInstitutionIdentification8,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr") )]
	pub cdtr: PartyIdentification272,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<PartyIdentification272>,
}

impl OriginalTransactionReference40 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.amt { val.validate()? }
		if let Some(ref val) = self.reqd_exctn_dt { val.validate()? }
		if let Some(ref val) = self.xpry_dt { val.validate()? }
		if let Some(ref val) = self.pmt_cond { val.validate()? }
		if let Some(ref val) = self.pmt_tp_inf { val.validate()? }
		if let Some(ref val) = self.pmt_mtd { val.validate()? }
		if let Some(ref val) = self.mndt_rltd_inf { val.validate()? }
		if let Some(ref val) = self.rmt_inf { val.validate()? }
		if let Some(ref vec) = self.nclsd_file { for item in vec { item.validate()? } }
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.dbtr { val.validate()? }
		if let Some(ref val) = self.dbtr_acct { val.validate()? }
		if let Some(ref val) = self.dbtr_agt { val.validate()? }
		if let Some(ref val) = self.dbtr_agt_acct { val.validate()? }
		self.cdtr_agt.validate()?;
		if let Some(ref val) = self.cdtr_agt_acct { val.validate()? }
		self.cdtr.validate()?;
		if let Some(ref val) = self.cdtr_acct { val.validate()? }
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		Ok(())
	}
}


// OriginalTransactionReference42 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct OriginalTransactionReference42 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmAmt", skip_serializing_if = "Option::is_none") )]
	pub intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<AmountType4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none") )]
	pub intr_bk_sttlm_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdColltnDt", skip_serializing_if = "Option::is_none") )]
	pub reqd_colltn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none") )]
	pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrSchmeId", skip_serializing_if = "Option::is_none") )]
	pub cdtr_schme_id: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmInf", skip_serializing_if = "Option::is_none") )]
	pub sttlm_inf: Option<SettlementInstruction15>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none") )]
	pub pmt_tp_inf: Option<PaymentTypeInformation27>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtMtd", skip_serializing_if = "Option::is_none") )]
	pub pmt_mtd: Option<PaymentMethod4Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtRltdInf", skip_serializing_if = "Option::is_none") )]
	pub mndt_rltd_inf: Option<MandateRelatedData3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtInf", skip_serializing_if = "Option::is_none") )]
	pub rmt_inf: Option<RemittanceInformation22>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<Party50Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr", skip_serializing_if = "Option::is_none") )]
	pub dbtr: Option<Party50Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr", skip_serializing_if = "Option::is_none") )]
	pub cdtr: Option<Party50Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<Party50Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
	pub purp: Option<Purpose2Choice>,
}

impl OriginalTransactionReference42 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.intr_bk_sttlm_amt { val.validate()? }
		if let Some(ref val) = self.amt { val.validate()? }
		if let Some(ref val) = self.reqd_exctn_dt { val.validate()? }
		if let Some(ref val) = self.cdtr_schme_id { val.validate()? }
		if let Some(ref val) = self.sttlm_inf { val.validate()? }
		if let Some(ref val) = self.pmt_tp_inf { val.validate()? }
		if let Some(ref val) = self.pmt_mtd { val.validate()? }
		if let Some(ref val) = self.mndt_rltd_inf { val.validate()? }
		if let Some(ref val) = self.rmt_inf { val.validate()? }
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.dbtr { val.validate()? }
		if let Some(ref val) = self.dbtr_acct { val.validate()? }
		if let Some(ref val) = self.dbtr_agt { val.validate()? }
		if let Some(ref val) = self.dbtr_agt_acct { val.validate()? }
		if let Some(ref val) = self.cdtr_agt { val.validate()? }
		if let Some(ref val) = self.cdtr_agt_acct { val.validate()? }
		if let Some(ref val) = self.cdtr { val.validate()? }
		if let Some(ref val) = self.cdtr_acct { val.validate()? }
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		if let Some(ref val) = self.purp { val.validate()? }
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


// Party50Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Party50Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pty", skip_serializing_if = "Option::is_none") )]
	pub pty: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Agt", skip_serializing_if = "Option::is_none") )]
	pub agt: Option<BranchAndFinancialInstitutionIdentification8>,
}

impl Party50Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pty { val.validate()? }
		if let Some(ref val) = self.agt { val.validate()? }
		Ok(())
	}
}


// Party52Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Party52Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgId", skip_serializing_if = "Option::is_none") )]
	pub org_id: Option<OrganisationIdentification39>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvtId", skip_serializing_if = "Option::is_none") )]
	pub prvt_id: Option<PersonIdentification18>,
}

impl Party52Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.org_id { val.validate()? }
		if let Some(ref val) = self.prvt_id { val.validate()? }
		Ok(())
	}
}


// PartyAndSignature4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyAndSignature4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pty") )]
	pub pty: PartyIdentification272,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sgntr") )]
	pub sgntr: SkipPayload,
}

impl PartyAndSignature4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pty.validate()?;
		self.sgntr.validate()?;
		Ok(())
	}
}


// PartyIdentification272 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PartyIdentification272 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
	pub pstl_adr: Option<PostalAddress27>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<Party52Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none") )]
	pub ctry_of_res: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none") )]
	pub ctct_dtls: Option<Contact13>,
}

impl PartyIdentification272 {
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


// PaymentCondition2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PaymentCondition2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmtModAllwd", skip_serializing_if = "Option::is_none") )]
	pub amt_mod_allwd: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EarlyPmtAllwd", skip_serializing_if = "Option::is_none") )]
	pub early_pmt_allwd: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DelyPnlty", skip_serializing_if = "Option::is_none") )]
	pub dely_pnlty: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ImdtPmtRbt", skip_serializing_if = "Option::is_none") )]
	pub imdt_pmt_rbt: Option<AmountOrRate1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrntedPmtReqd", skip_serializing_if = "Option::is_none") )]
	pub grnted_pmt_reqd: Option<bool>,
}

impl PaymentCondition2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dely_pnlty {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dely_pnlty is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "dely_pnlty exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.imdt_pmt_rbt { val.validate()? }
		Ok(())
	}
}


// PaymentConditionStatus2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PaymentConditionStatus2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AccptdAmt", skip_serializing_if = "Option::is_none") )]
	pub accptd_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrntedPmt", skip_serializing_if = "Option::is_none") )]
	pub grnted_pmt: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EarlyPmt", skip_serializing_if = "Option::is_none") )]
	pub early_pmt: Option<bool>,
}

impl PaymentConditionStatus2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.accptd_amt { val.validate()? }
		Ok(())
	}
}


// PaymentIdentification6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PaymentIdentification6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrId", skip_serializing_if = "Option::is_none") )]
	pub instr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndToEndId") )]
	pub end_to_end_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UETR", skip_serializing_if = "Option::is_none") )]
	pub uetr: Option<String>,
}

impl PaymentIdentification6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.instr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "instr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "instr_id exceeds the maximum length of 35".to_string()));
			}
		}
		if self.end_to_end_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "end_to_end_id is shorter than the minimum length of 1".to_string()));
		}
		if self.end_to_end_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "end_to_end_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.uetr {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "uetr does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// PaymentInitiationSource1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PaymentInitiationSource1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prvdr", skip_serializing_if = "Option::is_none") )]
	pub prvdr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Vrsn", skip_serializing_if = "Option::is_none") )]
	pub vrsn: Option<String>,
}

impl PaymentInitiationSource1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 140 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
		}
		if let Some(ref val) = self.prvdr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prvdr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prvdr exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.vrsn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "vrsn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "vrsn exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// PaymentInstruction44 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PaymentInstruction44 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtInfId") )]
	pub pmt_inf_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtMtd") )]
	pub pmt_mtd: PaymentMethod3Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdAdvcTp", skip_serializing_if = "Option::is_none") )]
	pub reqd_advc_tp: Option<AdviceType1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BtchBookg", skip_serializing_if = "Option::is_none") )]
	pub btch_bookg: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxs", skip_serializing_if = "Option::is_none") )]
	pub nb_of_txs: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrlSum", skip_serializing_if = "Option::is_none") )]
	pub ctrl_sum: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none") )]
	pub pmt_tp_inf: Option<PaymentTypeInformation26>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdExctnDt") )]
	pub reqd_exctn_dt: DateAndDateTime2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PoolgAdjstmntDt", skip_serializing_if = "Option::is_none") )]
	pub poolg_adjstmnt_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr") )]
	pub dbtr: PartyIdentification272,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct") )]
	pub dbtr_acct: CashAccount40,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt") )]
	pub dbtr_agt: BranchAndFinancialInstitutionIdentification8,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrForDbtrAgt", skip_serializing_if = "Option::is_none") )]
	pub instr_for_dbtr_agt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgBr", skip_serializing_if = "Option::is_none") )]
	pub chrg_br: Option<ChargeBearerType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgsAcct", skip_serializing_if = "Option::is_none") )]
	pub chrgs_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgsAcctAgt", skip_serializing_if = "Option::is_none") )]
	pub chrgs_acct_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtTrfTxInf") )]
	pub cdt_trf_tx_inf: Vec<CreditTransferTransaction61>,
}

impl PaymentInstruction44 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.pmt_inf_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "pmt_inf_id is shorter than the minimum length of 1".to_string()));
		}
		if self.pmt_inf_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "pmt_inf_id exceeds the maximum length of 35".to_string()));
		}
		self.pmt_mtd.validate()?;
		if let Some(ref val) = self.reqd_advc_tp { val.validate()? }
		if let Some(ref val) = self.nb_of_txs {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "nb_of_txs does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pmt_tp_inf { val.validate()? }
		self.reqd_exctn_dt.validate()?;
		self.dbtr.validate()?;
		self.dbtr_acct.validate()?;
		self.dbtr_agt.validate()?;
		if let Some(ref val) = self.dbtr_agt_acct { val.validate()? }
		if let Some(ref val) = self.instr_for_dbtr_agt {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "instr_for_dbtr_agt is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "instr_for_dbtr_agt exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.chrg_br { val.validate()? }
		if let Some(ref val) = self.chrgs_acct { val.validate()? }
		if let Some(ref val) = self.chrgs_acct_agt { val.validate()? }
		for item in &self.cdt_trf_tx_inf { item.validate()? }
		Ok(())
	}
}


// PaymentInstruction45 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PaymentInstruction45 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtInfId") )]
	pub pmt_inf_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtMtd") )]
	pub pmt_mtd: PaymentMethod2Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdAdvcTp", skip_serializing_if = "Option::is_none") )]
	pub reqd_advc_tp: Option<AdviceType1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BtchBookg", skip_serializing_if = "Option::is_none") )]
	pub btch_bookg: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxs", skip_serializing_if = "Option::is_none") )]
	pub nb_of_txs: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrlSum", skip_serializing_if = "Option::is_none") )]
	pub ctrl_sum: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none") )]
	pub pmt_tp_inf: Option<PaymentTypeInformation29>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdColltnDt") )]
	pub reqd_colltn_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr") )]
	pub cdtr: PartyIdentification272,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct") )]
	pub cdtr_acct: CashAccount40,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt") )]
	pub cdtr_agt: BranchAndFinancialInstitutionIdentification8,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgBr", skip_serializing_if = "Option::is_none") )]
	pub chrg_br: Option<ChargeBearerType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgsAcct", skip_serializing_if = "Option::is_none") )]
	pub chrgs_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgsAcctAgt", skip_serializing_if = "Option::is_none") )]
	pub chrgs_acct_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrSchmeId", skip_serializing_if = "Option::is_none") )]
	pub cdtr_schme_id: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DrctDbtTxInf") )]
	pub drct_dbt_tx_inf: Vec<DirectDebitTransactionInformation32>,
}

impl PaymentInstruction45 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.pmt_inf_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "pmt_inf_id is shorter than the minimum length of 1".to_string()));
		}
		if self.pmt_inf_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "pmt_inf_id exceeds the maximum length of 35".to_string()));
		}
		self.pmt_mtd.validate()?;
		if let Some(ref val) = self.reqd_advc_tp { val.validate()? }
		if let Some(ref val) = self.nb_of_txs {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "nb_of_txs does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pmt_tp_inf { val.validate()? }
		self.cdtr.validate()?;
		self.cdtr_acct.validate()?;
		self.cdtr_agt.validate()?;
		if let Some(ref val) = self.cdtr_agt_acct { val.validate()? }
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		if let Some(ref val) = self.chrg_br { val.validate()? }
		if let Some(ref val) = self.chrgs_acct { val.validate()? }
		if let Some(ref val) = self.chrgs_acct_agt { val.validate()? }
		if let Some(ref val) = self.cdtr_schme_id { val.validate()? }
		for item in &self.drct_dbt_tx_inf { item.validate()? }
		Ok(())
	}
}


// PaymentInstruction46 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PaymentInstruction46 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtInfId", skip_serializing_if = "Option::is_none") )]
	pub pmt_inf_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtMtd") )]
	pub pmt_mtd: PaymentMethod7Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdAdvcTp", skip_serializing_if = "Option::is_none") )]
	pub reqd_advc_tp: Option<AdviceType1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none") )]
	pub pmt_tp_inf: Option<PaymentTypeInformation29>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none") )]
	pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XpryDt", skip_serializing_if = "Option::is_none") )]
	pub xpry_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtCond", skip_serializing_if = "Option::is_none") )]
	pub pmt_cond: Option<PaymentCondition2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr") )]
	pub dbtr: PartyIdentification272,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt") )]
	pub dbtr_agt: BranchAndFinancialInstitutionIdentification8,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgBr", skip_serializing_if = "Option::is_none") )]
	pub chrg_br: Option<ChargeBearerType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtTrfTx") )]
	pub cdt_trf_tx: Vec<CreditTransferTransaction65>,
}

impl PaymentInstruction46 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pmt_inf_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pmt_inf_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "pmt_inf_id exceeds the maximum length of 35".to_string()));
			}
		}
		self.pmt_mtd.validate()?;
		if let Some(ref val) = self.reqd_advc_tp { val.validate()? }
		if let Some(ref val) = self.pmt_tp_inf { val.validate()? }
		if let Some(ref val) = self.reqd_exctn_dt { val.validate()? }
		if let Some(ref val) = self.xpry_dt { val.validate()? }
		if let Some(ref val) = self.pmt_cond { val.validate()? }
		self.dbtr.validate()?;
		if let Some(ref val) = self.dbtr_acct { val.validate()? }
		self.dbtr_agt.validate()?;
		if let Some(ref val) = self.dbtr_agt_acct { val.validate()? }
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.chrg_br { val.validate()? }
		for item in &self.cdt_trf_tx { item.validate()? }
		Ok(())
	}
}


// PaymentMethod2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum PaymentMethod2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DD") )]
	CodeDD,
}

impl PaymentMethod2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PaymentMethod3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum PaymentMethod3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CHK") )]
	CodeCHK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRF") )]
	CodeTRF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRA") )]
	CodeTRA,
}

impl PaymentMethod3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PaymentMethod4Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum PaymentMethod4Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CHK") )]
	CodeCHK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRF") )]
	CodeTRF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DD") )]
	CodeDD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRA") )]
	CodeTRA,
}

impl PaymentMethod4Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PaymentMethod7Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum PaymentMethod7Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CHK") )]
	CodeCHK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRF") )]
	CodeTRF,
}

impl PaymentMethod7Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PaymentReversalReason10 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PaymentReversalReason10 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Orgtr", skip_serializing_if = "Option::is_none") )]
	pub orgtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<ReversalReason4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<String>>,
}

impl PaymentReversalReason10 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgtr { val.validate()? }
		if let Some(ref val) = self.rsn { val.validate()? }
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


// PaymentTransaction150 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PaymentTransaction150 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsId", skip_serializing_if = "Option::is_none") )]
	pub sts_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_instr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_end_to_end_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none") )]
	pub orgnl_uetr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxSts", skip_serializing_if = "Option::is_none") )]
	pub tx_sts: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsRsnInf", skip_serializing_if = "Option::is_none") )]
	pub sts_rsn_inf: Option<Vec<StatusReasonInformation14>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtCondSts", skip_serializing_if = "Option::is_none") )]
	pub pmt_cond_sts: Option<PaymentConditionStatus2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgsInf", skip_serializing_if = "Option::is_none") )]
	pub chrgs_inf: Option<Vec<Charges16>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrDcsnDtTm", skip_serializing_if = "Option::is_none") )]
	pub dbtr_dcsn_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AccptncDtTm", skip_serializing_if = "Option::is_none") )]
	pub accptnc_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrRef", skip_serializing_if = "Option::is_none") )]
	pub acct_svcr_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysRef", skip_serializing_if = "Option::is_none") )]
	pub clr_sys_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none") )]
	pub orgnl_tx_ref: Option<OriginalTransactionReference40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NclsdFile", skip_serializing_if = "Option::is_none") )]
	pub nclsd_file: Option<Vec<Document15>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl PaymentTransaction150 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.sts_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sts_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "sts_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_instr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_instr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_instr_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_end_to_end_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_end_to_end_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_end_to_end_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_uetr {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_uetr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.tx_sts {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tx_sts is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "tx_sts exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref vec) = self.sts_rsn_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.pmt_cond_sts { val.validate()? }
		if let Some(ref vec) = self.chrgs_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.acct_svcr_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_svcr_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acct_svcr_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.clr_sys_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "clr_sys_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "clr_sys_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_tx_ref { val.validate()? }
		if let Some(ref vec) = self.nclsd_file { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PaymentTransaction156 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PaymentTransaction156 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RvslId", skip_serializing_if = "Option::is_none") )]
	pub rvsl_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_instr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_end_to_end_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none") )]
	pub orgnl_uetr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlInstdAmt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RvsdInstdAmt", skip_serializing_if = "Option::is_none") )]
	pub rvsd_instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgBr", skip_serializing_if = "Option::is_none") )]
	pub chrg_br: Option<ChargeBearerType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RvslRsnInf", skip_serializing_if = "Option::is_none") )]
	pub rvsl_rsn_inf: Option<Vec<PaymentReversalReason10>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none") )]
	pub orgnl_tx_ref: Option<OriginalTransactionReference42>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl PaymentTransaction156 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rvsl_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rvsl_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "rvsl_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_instr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_instr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_instr_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_end_to_end_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_end_to_end_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_end_to_end_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_uetr {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_uetr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_instd_amt { val.validate()? }
		if let Some(ref val) = self.rvsd_instd_amt { val.validate()? }
		if let Some(ref val) = self.chrg_br { val.validate()? }
		if let Some(ref vec) = self.rvsl_rsn_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.orgnl_tx_ref { val.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PaymentTransaction160 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PaymentTransaction160 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsId", skip_serializing_if = "Option::is_none") )]
	pub sts_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_instr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_end_to_end_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none") )]
	pub orgnl_uetr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxSts", skip_serializing_if = "Option::is_none") )]
	pub tx_sts: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsRsnInf", skip_serializing_if = "Option::is_none") )]
	pub sts_rsn_inf: Option<Vec<StatusReasonInformation14>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgsInf", skip_serializing_if = "Option::is_none") )]
	pub chrgs_inf: Option<Vec<Charges16>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrckrData", skip_serializing_if = "Option::is_none") )]
	pub trckr_data: Option<TrackerData7>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AccptncDtTm", skip_serializing_if = "Option::is_none") )]
	pub accptnc_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrRef", skip_serializing_if = "Option::is_none") )]
	pub acct_svcr_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysRef", skip_serializing_if = "Option::is_none") )]
	pub clr_sys_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none") )]
	pub orgnl_tx_ref: Option<OriginalTransactionReference42>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl PaymentTransaction160 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.sts_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sts_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "sts_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_instr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_instr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_instr_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_end_to_end_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_end_to_end_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_end_to_end_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_uetr {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_uetr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.tx_sts {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tx_sts is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "tx_sts exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref vec) = self.sts_rsn_inf { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.chrgs_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.trckr_data { val.validate()? }
		if let Some(ref val) = self.acct_svcr_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_svcr_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acct_svcr_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.clr_sys_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "clr_sys_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "clr_sys_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_tx_ref { val.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PaymentTypeInformation26 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PaymentTypeInformation26 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none") )]
	pub instr_prty: Option<Priority2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none") )]
	pub svc_lvl: Option<Vec<ServiceLevel8Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none") )]
	pub lcl_instrm: Option<LocalInstrument2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none") )]
	pub ctgy_purp: Option<CategoryPurpose1Choice>,
}

impl PaymentTypeInformation26 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.instr_prty { val.validate()? }
		if let Some(ref vec) = self.svc_lvl { for item in vec { item.validate()? } }
		if let Some(ref val) = self.lcl_instrm { val.validate()? }
		if let Some(ref val) = self.ctgy_purp { val.validate()? }
		Ok(())
	}
}


// PaymentTypeInformation27 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PaymentTypeInformation27 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none") )]
	pub instr_prty: Option<Priority2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrChanl", skip_serializing_if = "Option::is_none") )]
	pub clr_chanl: Option<ClearingChannel2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none") )]
	pub svc_lvl: Option<Vec<ServiceLevel8Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none") )]
	pub lcl_instrm: Option<LocalInstrument2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SeqTp", skip_serializing_if = "Option::is_none") )]
	pub seq_tp: Option<SequenceType3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none") )]
	pub ctgy_purp: Option<CategoryPurpose1Choice>,
}

impl PaymentTypeInformation27 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.instr_prty { val.validate()? }
		if let Some(ref val) = self.clr_chanl { val.validate()? }
		if let Some(ref vec) = self.svc_lvl { for item in vec { item.validate()? } }
		if let Some(ref val) = self.lcl_instrm { val.validate()? }
		if let Some(ref val) = self.seq_tp { val.validate()? }
		if let Some(ref val) = self.ctgy_purp { val.validate()? }
		Ok(())
	}
}


// PaymentTypeInformation29 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PaymentTypeInformation29 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none") )]
	pub instr_prty: Option<Priority2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none") )]
	pub svc_lvl: Option<Vec<ServiceLevel8Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none") )]
	pub lcl_instrm: Option<LocalInstrument2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SeqTp", skip_serializing_if = "Option::is_none") )]
	pub seq_tp: Option<SequenceType3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none") )]
	pub ctgy_purp: Option<CategoryPurpose1Choice>,
}

impl PaymentTypeInformation29 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.instr_prty { val.validate()? }
		if let Some(ref vec) = self.svc_lvl { for item in vec { item.validate()? } }
		if let Some(ref val) = self.lcl_instrm { val.validate()? }
		if let Some(ref val) = self.seq_tp { val.validate()? }
		if let Some(ref val) = self.ctgy_purp { val.validate()? }
		Ok(())
	}
}


// PersonIdentification18 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct PersonIdentification18 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none") )]
	pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<GenericPersonIdentification2>>,
}

impl PersonIdentification18 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dt_and_plc_of_birth { val.validate()? }
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


// Priority2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum Priority2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "HIGH") )]
	CodeHIGH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NORM") )]
	CodeNORM,
}

impl Priority2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ProxyAccountIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ProxyAccountIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<ProxyAccountType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
}

impl ProxyAccountIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 2048 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 2048".to_string()));
		}
		Ok(())
	}
}


// ProxyAccountType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ProxyAccountType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ProxyAccountType1Choice {
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


// Purpose2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct Purpose2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl Purpose2Choice {
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


// ReferredDocumentInformation8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ReferredDocumentInformation8 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<DocumentType1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nb", skip_serializing_if = "Option::is_none") )]
	pub nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdDt", skip_serializing_if = "Option::is_none") )]
	pub rltd_dt: Option<DateAndType1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LineDtls", skip_serializing_if = "Option::is_none") )]
	pub line_dtls: Option<Vec<DocumentLineInformation2>>,
}

impl ReferredDocumentInformation8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.rltd_dt { val.validate()? }
		if let Some(ref vec) = self.line_dtls { for item in vec { item.validate()? } }
		Ok(())
	}
}


// ReferredMandateDocument2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ReferredMandateDocument2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<DocumentType1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nb", skip_serializing_if = "Option::is_none") )]
	pub nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrRef", skip_serializing_if = "Option::is_none") )]
	pub cdtr_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdDt", skip_serializing_if = "Option::is_none") )]
	pub rltd_dt: Option<DateAndType1>,
}

impl ReferredMandateDocument2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.cdtr_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cdtr_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "cdtr_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.rltd_dt { val.validate()? }
		Ok(())
	}
}


// RegulatoryAuthority2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RegulatoryAuthority2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
	pub ctry: Option<String>,
}

impl RegulatoryAuthority2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
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


// RegulatoryReporting3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RegulatoryReporting3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtCdtRptgInd", skip_serializing_if = "Option::is_none") )]
	pub dbt_cdt_rptg_ind: Option<RegulatoryReportingType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Authrty", skip_serializing_if = "Option::is_none") )]
	pub authrty: Option<RegulatoryAuthority2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dtls", skip_serializing_if = "Option::is_none") )]
	pub dtls: Option<Vec<StructuredRegulatoryReporting3>>,
}

impl RegulatoryReporting3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dbt_cdt_rptg_ind { val.validate()? }
		if let Some(ref val) = self.authrty { val.validate()? }
		if let Some(ref vec) = self.dtls { for item in vec { item.validate()? } }
		Ok(())
	}
}


// RegulatoryReportingType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum RegulatoryReportingType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRED") )]
	CodeCRED,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DEBT") )]
	CodeDEBT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BOTH") )]
	CodeBOTH,
}

impl RegulatoryReportingType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RemittanceAmount4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RemittanceAmount4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtAmtAndTp", skip_serializing_if = "Option::is_none") )]
	pub rmt_amt_and_tp: Option<Vec<DocumentAmount1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdjstmntAmtAndRsn", skip_serializing_if = "Option::is_none") )]
	pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment1>>,
}

impl RemittanceAmount4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.rmt_amt_and_tp { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.adjstmnt_amt_and_rsn { for item in vec { item.validate()? } }
		Ok(())
	}
}


// RemittanceInformation22 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RemittanceInformation22 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ustrd", skip_serializing_if = "Option::is_none") )]
	pub ustrd: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Strd", skip_serializing_if = "Option::is_none") )]
	pub strd: Option<Vec<StructuredRemittanceInformation18>>,
}

impl RemittanceInformation22 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.ustrd {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "ustrd is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 140 {
					return Err(ValidationError::new(1002, "ustrd exceeds the maximum length of 140".to_string()));
				}
			}
		}
		if let Some(ref vec) = self.strd { for item in vec { item.validate()? } }
		Ok(())
	}
}


// RemittanceLocation8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RemittanceLocation8 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtId", skip_serializing_if = "Option::is_none") )]
	pub rmt_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtLctnDtls", skip_serializing_if = "Option::is_none") )]
	pub rmt_lctn_dtls: Option<Vec<RemittanceLocationData2>>,
}

impl RemittanceLocation8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rmt_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rmt_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "rmt_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.rmt_lctn_dtls { for item in vec { item.validate()? } }
		Ok(())
	}
}


// RemittanceLocationData2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct RemittanceLocationData2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mtd") )]
	pub mtd: RemittanceLocationMethod2Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ElctrncAdr", skip_serializing_if = "Option::is_none") )]
	pub elctrnc_adr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
	pub pstl_adr: Option<NameAndAddress18>,
}

impl RemittanceLocationData2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.mtd.validate()?;
		if let Some(ref val) = self.elctrnc_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "elctrnc_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 2048 {
				return Err(ValidationError::new(1002, "elctrnc_adr exceeds the maximum length of 2048".to_string()));
			}
		}
		if let Some(ref val) = self.pstl_adr { val.validate()? }
		Ok(())
	}
}


// RemittanceLocationMethod2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum RemittanceLocationMethod2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FAXI") )]
	CodeFAXI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EDIC") )]
	CodeEDIC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "URID") )]
	CodeURID,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EMAL") )]
	CodeEMAL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "POST") )]
	CodePOST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SMSM") )]
	CodeSMSM,
}

impl RemittanceLocationMethod2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ReversalReason4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ReversalReason4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ReversalReason4Choice {
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


// SequenceType2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum SequenceType2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "RCUR") )]
	CodeRCUR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OOFF") )]
	CodeOOFF,
}

impl SequenceType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SequenceType3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum SequenceType3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FRST") )]
	CodeFRST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RCUR") )]
	CodeRCUR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FNAL") )]
	CodeFNAL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OOFF") )]
	CodeOOFF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RPRE") )]
	CodeRPRE,
}

impl SequenceType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ServiceLevel8Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct ServiceLevel8Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ServiceLevel8Choice {
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


// SettlementInstruction15 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SettlementInstruction15 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmMtd") )]
	pub sttlm_mtd: SettlementMethod1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmAcct", skip_serializing_if = "Option::is_none") )]
	pub sttlm_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSys", skip_serializing_if = "Option::is_none") )]
	pub clr_sys: Option<ClearingSystemIdentification3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstgRmbrsmntAgt", skip_serializing_if = "Option::is_none") )]
	pub instg_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstgRmbrsmntAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub instg_rmbrsmnt_agt_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdRmbrsmntAgt", skip_serializing_if = "Option::is_none") )]
	pub instd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdRmbrsmntAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub instd_rmbrsmnt_agt_acct: Option<CashAccount40>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ThrdRmbrsmntAgt", skip_serializing_if = "Option::is_none") )]
	pub thrd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ThrdRmbrsmntAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub thrd_rmbrsmnt_agt_acct: Option<CashAccount40>,
}

impl SettlementInstruction15 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.sttlm_mtd.validate()?;
		if let Some(ref val) = self.sttlm_acct { val.validate()? }
		if let Some(ref val) = self.clr_sys { val.validate()? }
		if let Some(ref val) = self.instg_rmbrsmnt_agt { val.validate()? }
		if let Some(ref val) = self.instg_rmbrsmnt_agt_acct { val.validate()? }
		if let Some(ref val) = self.instd_rmbrsmnt_agt { val.validate()? }
		if let Some(ref val) = self.instd_rmbrsmnt_agt_acct { val.validate()? }
		if let Some(ref val) = self.thrd_rmbrsmnt_agt { val.validate()? }
		if let Some(ref val) = self.thrd_rmbrsmnt_agt_acct { val.validate()? }
		Ok(())
	}
}


// SettlementMethod1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum SettlementMethod1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "INDA") )]
	CodeINDA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INGA") )]
	CodeINGA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "COVE") )]
	CodeCOVE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLRG") )]
	CodeCLRG,
}

impl SettlementMethod1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SkipPayload ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct SkipPayload {
}

impl SkipPayload {
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


// StatusReasonInformation14 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct StatusReasonInformation14 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Orgtr", skip_serializing_if = "Option::is_none") )]
	pub orgtr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<StatusReason6Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<String>>,
}

impl StatusReasonInformation14 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgtr { val.validate()? }
		if let Some(ref val) = self.rsn { val.validate()? }
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


// StructuredRegulatoryReporting3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct StructuredRegulatoryReporting3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
	pub ctry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Inf", skip_serializing_if = "Option::is_none") )]
	pub inf: Option<Vec<String>>,
}

impl StructuredRegulatoryReporting3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 10 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 10".to_string()));
			}
		}
		if let Some(ref val) = self.amt { val.validate()? }
		if let Some(ref vec) = self.inf {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "inf is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 35 {
					return Err(ValidationError::new(1002, "inf exceeds the maximum length of 35".to_string()));
				}
			}
		}
		Ok(())
	}
}


// StructuredRemittanceInformation18 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct StructuredRemittanceInformation18 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RfrdDocInf", skip_serializing_if = "Option::is_none") )]
	pub rfrd_doc_inf: Option<Vec<ReferredDocumentInformation8>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RfrdDocAmt", skip_serializing_if = "Option::is_none") )]
	pub rfrd_doc_amt: Option<RemittanceAmount4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrRefInf", skip_serializing_if = "Option::is_none") )]
	pub cdtr_ref_inf: Option<CreditorReferenceInformation3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Invcr", skip_serializing_if = "Option::is_none") )]
	pub invcr: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Invcee", skip_serializing_if = "Option::is_none") )]
	pub invcee: Option<PartyIdentification272>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRmt", skip_serializing_if = "Option::is_none") )]
	pub tax_rmt: Option<TaxData1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrnshmtRmt", skip_serializing_if = "Option::is_none") )]
	pub grnshmt_rmt: Option<Garnishment4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRmtInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_rmt_inf: Option<Vec<String>>,
}

impl StructuredRemittanceInformation18 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.rfrd_doc_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.rfrd_doc_amt { val.validate()? }
		if let Some(ref val) = self.cdtr_ref_inf { val.validate()? }
		if let Some(ref val) = self.invcr { val.validate()? }
		if let Some(ref val) = self.invcee { val.validate()? }
		if let Some(ref val) = self.tax_rmt { val.validate()? }
		if let Some(ref val) = self.grnshmt_rmt { val.validate()? }
		if let Some(ref vec) = self.addtl_rmt_inf {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_rmt_inf is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 140 {
					return Err(ValidationError::new(1002, "addtl_rmt_inf exceeds the maximum length of 140".to_string()));
				}
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


// TaxAmount3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TaxAmount3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
	pub rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxblBaseAmt", skip_serializing_if = "Option::is_none") )]
	pub taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none") )]
	pub ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dtls", skip_serializing_if = "Option::is_none") )]
	pub dtls: Option<Vec<TaxRecordDetails3>>,
}

impl TaxAmount3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.taxbl_base_amt { val.validate()? }
		if let Some(ref val) = self.ttl_amt { val.validate()? }
		if let Some(ref vec) = self.dtls { for item in vec { item.validate()? } }
		Ok(())
	}
}


// TaxAuthorisation1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TaxAuthorisation1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Titl", skip_serializing_if = "Option::is_none") )]
	pub titl: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
}

impl TaxAuthorisation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.titl {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "titl is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "titl exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// TaxData1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TaxData1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr", skip_serializing_if = "Option::is_none") )]
	pub cdtr: Option<TaxParty1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr", skip_serializing_if = "Option::is_none") )]
	pub dbtr: Option<TaxParty2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<TaxParty2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdmstnZone", skip_serializing_if = "Option::is_none") )]
	pub admstn_zone: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefNb", skip_serializing_if = "Option::is_none") )]
	pub ref_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mtd", skip_serializing_if = "Option::is_none") )]
	pub mtd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlTaxblBaseAmt", skip_serializing_if = "Option::is_none") )]
	pub ttl_taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlTaxAmt", skip_serializing_if = "Option::is_none") )]
	pub ttl_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SeqNb", skip_serializing_if = "Option::is_none") )]
	pub seq_nb: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rcrd", skip_serializing_if = "Option::is_none") )]
	pub rcrd: Option<Vec<TaxRecord3>>,
}

impl TaxData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cdtr { val.validate()? }
		if let Some(ref val) = self.dbtr { val.validate()? }
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.admstn_zone {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "admstn_zone is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "admstn_zone exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ref_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ref_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "ref_nb exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.mtd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mtd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mtd exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ttl_taxbl_base_amt { val.validate()? }
		if let Some(ref val) = self.ttl_tax_amt { val.validate()? }
		if let Some(ref vec) = self.rcrd { for item in vec { item.validate()? } }
		Ok(())
	}
}


// TaxParty1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TaxParty1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxId", skip_serializing_if = "Option::is_none") )]
	pub tax_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegnId", skip_serializing_if = "Option::is_none") )]
	pub regn_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxTp", skip_serializing_if = "Option::is_none") )]
	pub tax_tp: Option<String>,
}

impl TaxParty1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tax_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tax_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tax_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.regn_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "regn_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "regn_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.tax_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tax_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tax_tp exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// TaxParty2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TaxParty2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxId", skip_serializing_if = "Option::is_none") )]
	pub tax_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegnId", skip_serializing_if = "Option::is_none") )]
	pub regn_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxTp", skip_serializing_if = "Option::is_none") )]
	pub tax_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Authstn", skip_serializing_if = "Option::is_none") )]
	pub authstn: Option<TaxAuthorisation1>,
}

impl TaxParty2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tax_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tax_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tax_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.regn_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "regn_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "regn_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.tax_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tax_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tax_tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.authstn { val.validate()? }
		Ok(())
	}
}


// TaxPeriod3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TaxPeriod3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Yr", skip_serializing_if = "Option::is_none") )]
	pub yr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<TaxRecordPeriod1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrToDt", skip_serializing_if = "Option::is_none") )]
	pub fr_to_dt: Option<DatePeriod2>,
}

impl TaxPeriod3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.fr_to_dt { val.validate()? }
		Ok(())
	}
}


// TaxRecord3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TaxRecord3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctgy", skip_serializing_if = "Option::is_none") )]
	pub ctgy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtgyDtls", skip_serializing_if = "Option::is_none") )]
	pub ctgy_dtls: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrSts", skip_serializing_if = "Option::is_none") )]
	pub dbtr_sts: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CertId", skip_serializing_if = "Option::is_none") )]
	pub cert_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrmsCd", skip_serializing_if = "Option::is_none") )]
	pub frms_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prd", skip_serializing_if = "Option::is_none") )]
	pub prd: Option<TaxPeriod3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none") )]
	pub tax_amt: Option<TaxAmount3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<String>,
}

impl TaxRecord3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ctgy {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ctgy is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ctgy exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ctgy_dtls {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ctgy_dtls is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ctgy_dtls exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.dbtr_sts {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dbtr_sts is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "dbtr_sts exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.cert_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cert_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "cert_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.frms_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "frms_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "frms_cd exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.prd { val.validate()? }
		if let Some(ref val) = self.tax_amt { val.validate()? }
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


// TaxRecordDetails3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TaxRecordDetails3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prd", skip_serializing_if = "Option::is_none") )]
	pub prd: Option<TaxPeriod3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}

impl TaxRecordDetails3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.prd { val.validate()? }
		self.amt.validate()?;
		Ok(())
	}
}


// TaxRecordPeriod1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum TaxRecordPeriod1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM01") )]
	CodeMM01,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM02") )]
	CodeMM02,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM03") )]
	CodeMM03,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM04") )]
	CodeMM04,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM05") )]
	CodeMM05,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM06") )]
	CodeMM06,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM07") )]
	CodeMM07,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM08") )]
	CodeMM08,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM09") )]
	CodeMM09,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM10") )]
	CodeMM10,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM11") )]
	CodeMM11,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM12") )]
	CodeMM12,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QTR1") )]
	CodeQTR1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QTR2") )]
	CodeQTR2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QTR3") )]
	CodeQTR3,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QTR4") )]
	CodeQTR4,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HLF1") )]
	CodeHLF1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HLF2") )]
	CodeHLF2,
}

impl TaxRecordPeriod1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TrackerData7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TrackerData7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ConfdDt") )]
	pub confd_dt: DateAndDateTime2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ConfdAmt") )]
	pub confd_amt: ActiveCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrckrRcrd") )]
	pub trckr_rcrd: Vec<TrackerRecord5>,
}

impl TrackerData7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.confd_dt.validate()?;
		self.confd_amt.validate()?;
		for item in &self.trckr_rcrd { item.validate()? }
		Ok(())
	}
}


// TrackerRecord5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub struct TrackerRecord5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Agt") )]
	pub agt: BranchAndFinancialInstitutionIdentification8,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgBr", skip_serializing_if = "Option::is_none") )]
	pub chrg_br: Option<ChargeBearerType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgsAmt", skip_serializing_if = "Option::is_none") )]
	pub chrgs_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XchgRateData", skip_serializing_if = "Option::is_none") )]
	pub xchg_rate_data: Option<CurrencyExchange13>,
}

impl TrackerRecord5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.agt.validate()?;
		if let Some(ref val) = self.chrg_br { val.validate()? }
		if let Some(ref val) = self.chrgs_amt { val.validate()? }
		if let Some(ref val) = self.xchg_rate_data { val.validate()? }
		Ok(())
	}
}