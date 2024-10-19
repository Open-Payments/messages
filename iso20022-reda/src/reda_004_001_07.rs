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

pub mod iso20022 {
	use regex::Regex;
	use crate::common::*;
	#[cfg(feature = "derive_serde")]
	use serde::{Deserialize, Serialize};
	
	
	// AccountIdentificationAndName7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AccountIdentificationAndName7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: CashAccountIdentification8Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max35Text>,
	}
	
	impl AccountIdentificationAndName7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AccountSchemeName1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AccountSchemeName1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalAccountIdentification1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl AccountSchemeName1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ActiveCurrencyAnd13DecimalAmountSimpleType ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveCurrencyAnd13DecimalAmountSimpleType {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_currency_and13_decimal_amount_simple_type: f64,
	}
	
	impl ActiveCurrencyAnd13DecimalAmountSimpleType {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.active_currency_and13_decimal_amount_simple_type < 0.000000 {
				return Err(ValidationError::new(1003, "active_currency_and13_decimal_amount_simple_type is less than the minimum value of 0.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ActiveCurrencyAnd13DecimalAmount ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// ActiveCurrencyAndAmountSimpleType ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveCurrencyAndAmountSimpleType {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_currency_and_amount_simple_type: f64,
	}
	
	impl ActiveCurrencyAndAmountSimpleType {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.active_currency_and_amount_simple_type < 0.000000 {
				return Err(ValidationError::new(1003, "active_currency_and_amount_simple_type is less than the minimum value of 0.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ActiveCurrencyAndAmount ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// ActiveCurrencyCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveCurrencyCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_currency_code: String,
	}
	
	impl ActiveCurrencyCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(&self.active_currency_code) {
				return Err(ValidationError::new(1005, "active_currency_code does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// AdditionalInformation15 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AdditionalInformation15 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "InfTp") )]
		pub inf_tp: GenericIdentification36,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InfVal") )]
		pub inf_val: Max350Text,
	}
	
	impl AdditionalInformation15 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.inf_tp.validate() { return Err(e); }
			if let Err(e) = self.inf_val.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// AdditionalProductInformation3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AdditionalReference10 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ref") )]
		pub ref_attr: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RefIssr", skip_serializing_if = "Option::is_none") )]
		pub ref_issr: Option<PartyIdentification139>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgNm", skip_serializing_if = "Option::is_none") )]
		pub msg_nm: Option<Max35Text>,
	}
	
	impl AdditionalReference10 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.ref_attr.validate() { return Err(e); }
			if let Some(ref ref_issr_value) = self.ref_issr { if let Err(e) = ref_issr_value.validate() { return Err(e); } }
			if let Some(ref msg_nm_value) = self.msg_nm { if let Err(e) = msg_nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AddressType2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// AnyBICDec2014Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct AnyBICDec2014Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub any_bic_dec2014_identifier: String,
	}
	
	impl AnyBICDec2014Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(&self.any_bic_dec2014_identifier) {
				return Err(ValidationError::new(1005, "any_bic_dec2014_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// AssessmentOfValueRequiredUnderCOLLUKType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// CFIOct2015Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct CFIOct2015Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub cfi_oct2015_identifier: String,
	}
	
	impl CFIOct2015Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{6,6}").unwrap();
			if !pattern.is_match(&self.cfi_oct2015_identifier) {
				return Err(ValidationError::new(1005, "cfi_oct2015_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// CashAccount205 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CashAccount205 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy", skip_serializing_if = "Option::is_none") )]
		pub ccy: Option<ActiveCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmryAcct", skip_serializing_if = "Option::is_none") )]
		pub pmry_acct: Option<CashAccount206>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ScndryAcct", skip_serializing_if = "Option::is_none") )]
		pub scndry_acct: Option<CashAccount206>,
	}
	
	impl CashAccount205 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ccy_value) = self.ccy { if let Err(e) = ccy_value.validate() { return Err(e); } }
			if let Some(ref pmry_acct_value) = self.pmry_acct { if let Err(e) = pmry_acct_value.validate() { return Err(e); } }
			if let Some(ref scndry_acct_value) = self.scndry_acct { if let Err(e) = scndry_acct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CashAccount206 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CashAccount206 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctId") )]
		pub acct_id: AccountIdentificationAndName7,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Svcr", skip_serializing_if = "Option::is_none") )]
		pub svcr: Option<AnyBICDec2014Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctTpDesc", skip_serializing_if = "Option::is_none") )]
		pub acct_tp_desc: Option<Max35Text>,
	}
	
	impl CashAccount206 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.acct_id.validate() { return Err(e); }
			if let Some(ref svcr_value) = self.svcr { if let Err(e) = svcr_value.validate() { return Err(e); } }
			if let Some(ref acct_tp_desc_value) = self.acct_tp_desc { if let Err(e) = acct_tp_desc_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CashAccountIdentification8Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CashAccountIdentification8Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<GenericAccountIdentification1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IBAN", skip_serializing_if = "Option::is_none") )]
		pub iban: Option<IBAN2007Identifier>,
	}
	
	impl CashAccountIdentification8Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			if let Some(ref iban_value) = self.iban { if let Err(e) = iban_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ChargeType8Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ChargeType8Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<InvestmentFundMiFIDFee2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl ChargeType8Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ContactAttributes5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ContactAttributes5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
		pub nm: Max350Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
		pub pstl_adr: Option<PostalAddress1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PhneNb", skip_serializing_if = "Option::is_none") )]
		pub phne_nb: Option<PhoneNumber>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FaxNb", skip_serializing_if = "Option::is_none") )]
		pub fax_nb: Option<PhoneNumber>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none") )]
		pub email_adr: Option<Max256Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "URLAdr", skip_serializing_if = "Option::is_none") )]
		pub url_adr: Option<Max2048Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
		pub any_bic: Option<AnyBICDec2014Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<LEIIdentifier>,
	}
	
	impl ContactAttributes5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.nm.validate() { return Err(e); }
			if let Some(ref pstl_adr_value) = self.pstl_adr { if let Err(e) = pstl_adr_value.validate() { return Err(e); } }
			if let Some(ref phne_nb_value) = self.phne_nb { if let Err(e) = phne_nb_value.validate() { return Err(e); } }
			if let Some(ref fax_nb_value) = self.fax_nb { if let Err(e) = fax_nb_value.validate() { return Err(e); } }
			if let Some(ref email_adr_value) = self.email_adr { if let Err(e) = email_adr_value.validate() { return Err(e); } }
			if let Some(ref url_adr_value) = self.url_adr { if let Err(e) = url_adr_value.validate() { return Err(e); } }
			if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
			if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ContactAttributes6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ContactAttributes6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
		pub pstl_adr: Option<PostalAddress1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PhneNb", skip_serializing_if = "Option::is_none") )]
		pub phne_nb: Option<PhoneNumber>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FaxNb", skip_serializing_if = "Option::is_none") )]
		pub fax_nb: Option<PhoneNumber>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none") )]
		pub email_adr: Option<Max256Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "URLAdr", skip_serializing_if = "Option::is_none") )]
		pub url_adr: Option<Max2048Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
		pub any_bic: Option<AnyBICDec2014Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<LEIIdentifier>,
	}
	
	impl ContactAttributes6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref pstl_adr_value) = self.pstl_adr { if let Err(e) = pstl_adr_value.validate() { return Err(e); } }
			if let Some(ref phne_nb_value) = self.phne_nb { if let Err(e) = phne_nb_value.validate() { return Err(e); } }
			if let Some(ref fax_nb_value) = self.fax_nb { if let Err(e) = fax_nb_value.validate() { return Err(e); } }
			if let Some(ref email_adr_value) = self.email_adr { if let Err(e) = email_adr_value.validate() { return Err(e); } }
			if let Some(ref url_adr_value) = self.url_adr { if let Err(e) = url_adr_value.validate() { return Err(e); } }
			if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
			if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CostsAndCharges2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			for item in &self.indv_cost_or_chrg { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CountryCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct CountryCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub country_code: String,
	}
	
	impl CountryCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(&self.country_code) {
				return Err(ValidationError::new(1005, "country_code does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// DecimalNumber ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct DecimalNumber {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub decimal_number: f64,
	}
	
	impl DecimalNumber {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// DistributionPolicy1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref exctn_only_value) = self.exctn_only { if let Err(e) = exctn_only_value.validate() { return Err(e); } }
			if let Some(ref exctn_wth_apprprtnss_tst_or_non_advsd_svcs_value) = self.exctn_wth_apprprtnss_tst_or_non_advsd_svcs { if let Err(e) = exctn_wth_apprprtnss_tst_or_non_advsd_svcs_value.validate() { return Err(e); } }
			if let Some(ref invstmt_advc_value) = self.invstmt_advc { if let Err(e) = invstmt_advc_value.validate() { return Err(e); } }
			if let Some(ref prtfl_mgmt_value) = self.prtfl_mgmt { if let Err(e) = prtfl_mgmt_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DistributionStrategy1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct DistributionStrategy1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<InvestorType3Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl DistributionStrategy1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DividendPolicy1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ExPostCostCalculationBasis1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExPostCostCalculationBasis1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl ExPostCostCalculationBasis1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ExPostCostCalculationBasis1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// Exact4AlphaNumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Exact4AlphaNumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub exact4_alpha_numeric_text: String,
	}
	
	impl Exact4AlphaNumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
			if !pattern.is_match(&self.exact4_alpha_numeric_text) {
				return Err(ValidationError::new(1005, "exact4_alpha_numeric_text does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExtendedParty13 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ExtendedParty13 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PtyRole") )]
		pub pty_role: GenericIdentification36,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPtyDtls") )]
		pub othr_pty_dtls: ContactAttributes5,
	}
	
	impl ExtendedParty13 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.pty_role.validate() { return Err(e); }
			if let Err(e) = self.othr_pty_dtls.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// Extension1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Extension1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PlcAndNm") )]
		pub plc_and_nm: Max350Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Txt") )]
		pub txt: Max350Text,
	}
	
	impl Extension1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.plc_and_nm.validate() { return Err(e); }
			if let Err(e) = self.txt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// ExternalAccountIdentification1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalAccountIdentification1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_account_identification1_code: String,
	}
	
	impl ExternalAccountIdentification1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_account_identification1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_account_identification1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_account_identification1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_account_identification1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalFinancialInstrumentIdentificationType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalFinancialInstrumentIdentificationType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_financial_instrument_identification_type1_code: String,
	}
	
	impl ExternalFinancialInstrumentIdentificationType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_financial_instrument_identification_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_financial_instrument_identification_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_financial_instrument_identification_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_financial_instrument_identification_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// FinancialInstrument96 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref dstrbtn_plcy_value) = self.dstrbtn_plcy { if let Err(e) = dstrbtn_plcy_value.validate() { return Err(e); } }
			if let Some(ref dvdd_plcy_value) = self.dvdd_plcy { if let Err(e) = dvdd_plcy_value.validate() { return Err(e); } }
			if let Some(ref dvdd_frqcy_value) = self.dvdd_frqcy { if let Err(e) = dvdd_frqcy_value.validate() { return Err(e); } }
			if let Some(ref rinvstmt_frqcy_value) = self.rinvstmt_frqcy { if let Err(e) = rinvstmt_frqcy_value.validate() { return Err(e); } }
			if let Some(ref eu_svgs_drctv_value) = self.eu_svgs_drctv { if let Err(e) = eu_svgs_drctv_value.validate() { return Err(e); } }
			if let Some(ref may_be_termntd_early_value) = self.may_be_termntd_early { if let Err(e) = may_be_termntd_early_value.validate() { return Err(e); } }
			if let Some(ref mgmt_fee_src_value) = self.mgmt_fee_src { if let Err(e) = mgmt_fee_src_value.validate() { return Err(e); } }
			if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// Forms1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Forms1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ApplForm") )]
		pub appl_form: bool,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SgntrTp") )]
		pub sgntr_tp: SignatureType1Code,
	}
	
	impl Forms1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.sgntr_tp.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// Frequency20Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Frequency20Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<EventFrequency8Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl Frequency20Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FundOrderType10Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FundOrderType5Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<FundOrderType10Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification36>,
	}
	
	impl FundOrderType5Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FundParties1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref guarntr_value) = self.guarntr { if let Err(e) = guarntr_value.validate() { return Err(e); } }
			if let Some(ref audtr_value) = self.audtr { if let Err(e) = audtr_value.validate() { return Err(e); } }
			if let Some(ref trstee_value) = self.trstee { if let Err(e) = trstee_value.validate() { return Err(e); } }
			if let Some(ref othr_pty_vec) = self.othr_pty { for item in othr_pty_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// FundPaymentType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FundPaymentType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<FundPaymentType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification36>,
	}
	
	impl FundPaymentType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FundPaymentType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FundReferenceDataReport5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<Max35Text>,
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
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			if let Some(ref vrsn_value) = self.vrsn { if let Err(e) = vrsn_value.validate() { return Err(e); } }
			if let Some(ref authrsd_prxy_value) = self.authrsd_prxy { if let Err(e) = authrsd_prxy_value.validate() { return Err(e); } }
			if let Err(e) = self.scty_id.validate() { return Err(e); }
			if let Some(ref fnd_pties_value) = self.fnd_pties { if let Err(e) = fnd_pties_value.validate() { return Err(e); } }
			if let Some(ref main_fnd_ordr_dsk_value) = self.main_fnd_ordr_dsk { if let Err(e) = main_fnd_ordr_dsk_value.validate() { return Err(e); } }
			if let Some(ref fnd_mgmt_cpny_value) = self.fnd_mgmt_cpny { if let Err(e) = fnd_mgmt_cpny_value.validate() { return Err(e); } }
			if let Some(ref fnd_dtls_value) = self.fnd_dtls { if let Err(e) = fnd_dtls_value.validate() { return Err(e); } }
			if let Some(ref valtn_dealg_chrtcs_value) = self.valtn_dealg_chrtcs { if let Err(e) = valtn_dealg_chrtcs_value.validate() { return Err(e); } }
			if let Some(ref invstmt_rstrctns_value) = self.invstmt_rstrctns { if let Err(e) = invstmt_rstrctns_value.validate() { return Err(e); } }
			if let Some(ref sbcpt_prcg_chrtcs_value) = self.sbcpt_prcg_chrtcs { if let Err(e) = sbcpt_prcg_chrtcs_value.validate() { return Err(e); } }
			if let Some(ref red_prcg_chrtcs_value) = self.red_prcg_chrtcs { if let Err(e) = red_prcg_chrtcs_value.validate() { return Err(e); } }
			if let Some(ref swtch_prcg_chrtcs_value) = self.swtch_prcg_chrtcs { if let Err(e) = swtch_prcg_chrtcs_value.validate() { return Err(e); } }
			if let Some(ref plan_chrtcs_vec) = self.plan_chrtcs { for item in plan_chrtcs_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref pmt_instrm_vec) = self.pmt_instrm { for item in pmt_instrm_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref csh_sttlm_dtls_vec) = self.csh_sttlm_dtls { for item in csh_sttlm_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref lcl_mkt_anx_vec) = self.lcl_mkt_anx { for item in lcl_mkt_anx_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref trgt_mkt_value) = self.trgt_mkt { if let Err(e) = trgt_mkt_value.validate() { return Err(e); } }
			if let Some(ref dstrbtn_strtgy_value) = self.dstrbtn_strtgy { if let Err(e) = dstrbtn_strtgy_value.validate() { return Err(e); } }
			if let Some(ref costs_and_chrgs_vec) = self.costs_and_chrgs { for item in costs_and_chrgs_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref addtl_inf_uk_mkt_value) = self.addtl_inf_uk_mkt { if let Err(e) = addtl_inf_uk_mkt_value.validate() { return Err(e); } }
			if let Some(ref val_for_mny_value) = self.val_for_mny { if let Err(e) = val_for_mny_value.validate() { return Err(e); } }
			if let Some(ref xtnsn_vec) = self.xtnsn { for item in xtnsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// FundReferenceDataReportV07 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FundReferenceDataReportV07 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
		pub msg_id: MessageIdentification1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsRef", skip_serializing_if = "Option::is_none") )]
		pub prvs_ref: Option<Vec<AdditionalReference10>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RltdRef", skip_serializing_if = "Option::is_none") )]
		pub rltd_ref: Option<AdditionalReference10>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FndRefDataRptId", skip_serializing_if = "Option::is_none") )]
		pub fnd_ref_data_rpt_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rpt") )]
		pub rpt: Vec<FundReferenceDataReport5>,
	}
	
	impl FundReferenceDataReportV07 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.msg_id.validate() { return Err(e); }
			if let Some(ref prvs_ref_vec) = self.prvs_ref { for item in prvs_ref_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref rltd_ref_value) = self.rltd_ref { if let Err(e) = rltd_ref_value.validate() { return Err(e); } }
			if let Some(ref fnd_ref_data_rpt_id_value) = self.fnd_ref_data_rpt_id { if let Err(e) = fnd_ref_data_rpt_id_value.validate() { return Err(e); } }
			for item in &self.rpt { if let Err(e) = item.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GenericAccountIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GenericAccountIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max34Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<AccountSchemeName1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl GenericAccountIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GenericIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GenericIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl GenericIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GenericIdentification3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GenericIdentification3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl GenericIdentification3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GenericIdentification36 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GenericIdentification36 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
		pub issr: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<Max35Text>,
	}
	
	impl GenericIdentification36 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Err(e) = self.issr.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GenericIdentification47 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GenericIdentification47 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Exact4AlphaNumericText,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
		pub issr: Max4AlphaNumericText,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<Max4AlphaNumericText>,
	}
	
	impl GenericIdentification47 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Err(e) = self.issr.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GovernanceProcess1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GovernanceProcess1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<GovernanceProcessType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl GovernanceProcess1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GovernanceProcessType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// IBAN2007Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct IBAN2007Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iban2007_identifier: String,
	}
	
	impl IBAN2007Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}").unwrap();
			if !pattern.is_match(&self.iban2007_identifier) {
				return Err(ValidationError::new(1005, "iban2007_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ISINOct2015Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISINOct2015Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub isin_oct2015_identifier: String,
	}
	
	impl ISINOct2015Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(&self.isin_oct2015_identifier) {
				return Err(ValidationError::new(1005, "isin_oct2015_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ISODate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISODate {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iso_date: String,
	}
	
	impl ISODate {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ISODateTime ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISODateTime {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iso_date_time: String,
	}
	
	impl ISODateTime {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ISOTime ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISOTime {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iso_time: String,
	}
	
	impl ISOTime {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ISOYearMonth ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISOYearMonth {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iso_year_month: String,
	}
	
	impl ISOYearMonth {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// IdentificationSource3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct IdentificationSource3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalFinancialInstrumentIdentificationType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl IdentificationSource3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// IndividualCostOrCharge2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Err(e) = self.cost_tp.validate() { return Err(e); }
			if let Err(e) = self.ex_ante_or_ex_pst.validate() { return Err(e); }
			if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
			if let Some(ref ref_prd_value) = self.ref_prd { if let Err(e) = ref_prd_value.validate() { return Err(e); } }
			if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// IntendedOrActual2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct InvestmentFundPlanType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<InvestmentFundPlanType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification36>,
	}
	
	impl InvestmentFundPlanType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// InvestmentFundPlanType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct InvestmentNeed2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<InvestmentNeed2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl InvestmentNeed2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// InvestmentNeed2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Err(e) = self.plan_tp.validate() { return Err(e); }
			if let Some(ref frqcy_value) = self.frqcy { if let Err(e) = frqcy_value.validate() { return Err(e); } }
			if let Some(ref qty_value) = self.qty { if let Err(e) = qty_value.validate() { return Err(e); } }
			if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// InvestmentRestrictions3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
		pub othr_red_rstrctns: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MinSwtchSbcptAmt", skip_serializing_if = "Option::is_none") )]
		pub min_swtch_sbcpt_amt: Option<ActiveCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MinSwtchSbcptUnits", skip_serializing_if = "Option::is_none") )]
		pub min_swtch_sbcpt_units: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MaxSwtchRedAmt", skip_serializing_if = "Option::is_none") )]
		pub max_swtch_red_amt: Option<ActiveCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MaxSwtchRedUnits", skip_serializing_if = "Option::is_none") )]
		pub max_swtch_red_units: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrSwtchRstrctns", skip_serializing_if = "Option::is_none") )]
		pub othr_swtch_rstrctns: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MinHldgAmt", skip_serializing_if = "Option::is_none") )]
		pub min_hldg_amt: Option<ActiveCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MinHldgUnits", skip_serializing_if = "Option::is_none") )]
		pub min_hldg_units: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MinHldgPrd", skip_serializing_if = "Option::is_none") )]
		pub min_hldg_prd: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HldgTrfbl", skip_serializing_if = "Option::is_none") )]
		pub hldg_trfbl: Option<HoldingTransferable1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<Vec<AdditionalInformation15>>,
	}
	
	impl InvestmentRestrictions3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref min_initl_sbcpt_amt_value) = self.min_initl_sbcpt_amt { if let Err(e) = min_initl_sbcpt_amt_value.validate() { return Err(e); } }
			if let Some(ref min_sbsqnt_sbcpt_amt_value) = self.min_sbsqnt_sbcpt_amt { if let Err(e) = min_sbsqnt_sbcpt_amt_value.validate() { return Err(e); } }
			if let Some(ref max_red_amt_value) = self.max_red_amt { if let Err(e) = max_red_amt_value.validate() { return Err(e); } }
			if let Some(ref othr_red_rstrctns_value) = self.othr_red_rstrctns { if let Err(e) = othr_red_rstrctns_value.validate() { return Err(e); } }
			if let Some(ref min_swtch_sbcpt_amt_value) = self.min_swtch_sbcpt_amt { if let Err(e) = min_swtch_sbcpt_amt_value.validate() { return Err(e); } }
			if let Some(ref max_swtch_red_amt_value) = self.max_swtch_red_amt { if let Err(e) = max_swtch_red_amt_value.validate() { return Err(e); } }
			if let Some(ref othr_swtch_rstrctns_value) = self.othr_swtch_rstrctns { if let Err(e) = othr_swtch_rstrctns_value.validate() { return Err(e); } }
			if let Some(ref min_hldg_amt_value) = self.min_hldg_amt { if let Err(e) = min_hldg_amt_value.validate() { return Err(e); } }
			if let Some(ref min_hldg_prd_value) = self.min_hldg_prd { if let Err(e) = min_hldg_prd_value.validate() { return Err(e); } }
			if let Some(ref hldg_trfbl_value) = self.hldg_trfbl { if let Err(e) = hldg_trfbl_value.validate() { return Err(e); } }
			if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// InvestorKnowledge1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref bsic_invstr_value) = self.bsic_invstr { if let Err(e) = bsic_invstr_value.validate() { return Err(e); } }
			if let Some(ref infrmd_invstr_value) = self.infrmd_invstr { if let Err(e) = infrmd_invstr_value.validate() { return Err(e); } }
			if let Some(ref advncd_invstr_value) = self.advncd_invstr { if let Err(e) = advncd_invstr_value.validate() { return Err(e); } }
			if let Some(ref exprt_invstr_de_value) = self.exprt_invstr_de { if let Err(e) = exprt_invstr_de_value.validate() { return Err(e); } }
			if let Some(ref othr_vec) = self.othr { for item in othr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// InvestorRequirements4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref rtr_prfl_prsrvtn_value) = self.rtr_prfl_prsrvtn { if let Err(e) = rtr_prfl_prsrvtn_value.validate() { return Err(e); } }
			if let Some(ref rtr_prfl_grwth_value) = self.rtr_prfl_grwth { if let Err(e) = rtr_prfl_grwth_value.validate() { return Err(e); } }
			if let Some(ref rtr_prfl_incm_value) = self.rtr_prfl_incm { if let Err(e) = rtr_prfl_incm_value.validate() { return Err(e); } }
			if let Some(ref rtr_prfl_hdgg_value) = self.rtr_prfl_hdgg { if let Err(e) = rtr_prfl_hdgg_value.validate() { return Err(e); } }
			if let Some(ref optn_or_lvrgd_rtr_prfl_value) = self.optn_or_lvrgd_rtr_prfl { if let Err(e) = optn_or_lvrgd_rtr_prfl_value.validate() { return Err(e); } }
			if let Some(ref rtr_prfl_pnsn_schme_de_value) = self.rtr_prfl_pnsn_schme_de { if let Err(e) = rtr_prfl_pnsn_schme_de_value.validate() { return Err(e); } }
			if let Some(ref min_hldg_prd_value) = self.min_hldg_prd { if let Err(e) = min_hldg_prd_value.validate() { return Err(e); } }
			if let Some(ref sstnblty_prefs_value) = self.sstnblty_prefs { if let Err(e) = sstnblty_prefs_value.validate() { return Err(e); } }
			if let Some(ref othr_spcfc_invstmt_need_value) = self.othr_spcfc_invstmt_need { if let Err(e) = othr_spcfc_invstmt_need_value.validate() { return Err(e); } }
			if let Some(ref othr_vec) = self.othr { for item in othr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// InvestorType2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref invstr_tp_rtl_value) = self.invstr_tp_rtl { if let Err(e) = invstr_tp_rtl_value.validate() { return Err(e); } }
			if let Some(ref invstr_tp_prfssnl_value) = self.invstr_tp_prfssnl { if let Err(e) = invstr_tp_prfssnl_value.validate() { return Err(e); } }
			if let Some(ref invstr_tp_elgbl_ctr_pty_value) = self.invstr_tp_elgbl_ctr_pty { if let Err(e) = invstr_tp_elgbl_ctr_pty_value.validate() { return Err(e); } }
			if let Some(ref othr_vec) = self.othr { for item in othr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// InvestorType2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// LEIIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct LEIIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub lei_identifier: String,
	}
	
	impl LEIIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(&self.lei_identifier) {
				return Err(ValidationError::new(1005, "lei_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// LocalMarketAnnex6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct LocalMarketAnnex6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
		pub ctry: Vec<CountryCode>,
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
			for item in &self.ctry { if let Err(e) = item.validate() { return Err(e); } }
			if let Err(e) = self.lcl_ordr_dsk.validate() { return Err(e); }
			if let Some(ref sbcpt_prcg_chrtcs_value) = self.sbcpt_prcg_chrtcs { if let Err(e) = sbcpt_prcg_chrtcs_value.validate() { return Err(e); } }
			if let Some(ref red_prcg_chrtcs_value) = self.red_prcg_chrtcs { if let Err(e) = red_prcg_chrtcs_value.validate() { return Err(e); } }
			if let Some(ref swtch_prcg_chrtcs_value) = self.swtch_prcg_chrtcs { if let Err(e) = swtch_prcg_chrtcs_value.validate() { return Err(e); } }
			if let Some(ref csh_sttlm_dtls_vec) = self.csh_sttlm_dtls { for item in csh_sttlm_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// LossBearing2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref no_cptl_loss_value) = self.no_cptl_loss { if let Err(e) = no_cptl_loss_value.validate() { return Err(e); } }
			if let Some(ref ltd_cptl_loss_value) = self.ltd_cptl_loss { if let Err(e) = ltd_cptl_loss_value.validate() { return Err(e); } }
			if let Some(ref no_cptl_grnt_value) = self.no_cptl_grnt { if let Err(e) = no_cptl_grnt_value.validate() { return Err(e); } }
			if let Some(ref loss_bynd_cptl_value) = self.loss_bynd_cptl { if let Err(e) = loss_bynd_cptl_value.validate() { return Err(e); } }
			if let Some(ref othr_vec) = self.othr { for item in othr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// MainFundOrderDeskLocation1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct MainFundOrderDeskLocation1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
		pub ctry: CountryCode,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TmZoneOffSet") )]
		pub tm_zone_off_set: UTCOffset1,
	}
	
	impl MainFundOrderDeskLocation1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.ctry.validate() { return Err(e); }
			if let Err(e) = self.tm_zone_off_set.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// MarketPracticeVersion1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct MarketPracticeVersion1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
		pub nm: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
		pub dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nb", skip_serializing_if = "Option::is_none") )]
		pub nb: Option<Max35Text>,
	}
	
	impl MarketPracticeVersion1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.nm.validate() { return Err(e); }
			if let Some(ref nb_value) = self.nb { if let Err(e) = nb_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Max140Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max140Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max140_text: String,
	}
	
	impl Max140Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max140_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max140_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max140_text.chars().count() > 140 {
				return Err(ValidationError::new(1002, "max140_text exceeds the maximum length of 140".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max16Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max16Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max16_text: String,
	}
	
	impl Max16Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max16_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max16_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max16_text.chars().count() > 16 {
				return Err(ValidationError::new(1002, "max16_text exceeds the maximum length of 16".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max1Number ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max1Number {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max1_number: f64,
	}
	
	impl Max1Number {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Max2048Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max2048Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max2048_text: String,
	}
	
	impl Max2048Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max2048_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max2048_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max2048_text.chars().count() > 2048 {
				return Err(ValidationError::new(1002, "max2048_text exceeds the maximum length of 2048".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max256Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max256Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max256_text: String,
	}
	
	impl Max256Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max256_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max256_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max256_text.chars().count() > 256 {
				return Err(ValidationError::new(1002, "max256_text exceeds the maximum length of 256".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max34Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max34Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max34_text: String,
	}
	
	impl Max34Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max34_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max34_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max34_text.chars().count() > 34 {
				return Err(ValidationError::new(1002, "max34_text exceeds the maximum length of 34".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max350Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max350Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max350_text: String,
	}
	
	impl Max350Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max350_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max350_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max350_text.chars().count() > 350 {
				return Err(ValidationError::new(1002, "max350_text exceeds the maximum length of 350".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max35Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max35Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max35_text: String,
	}
	
	impl Max35Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max35_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max35_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max35_text.chars().count() > 35 {
				return Err(ValidationError::new(1002, "max35_text exceeds the maximum length of 35".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max4AlphaNumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max4AlphaNumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max4_alpha_numeric_text: String,
	}
	
	impl Max4AlphaNumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max4_alpha_numeric_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max4_alpha_numeric_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max4_alpha_numeric_text.chars().count() > 4 {
				return Err(ValidationError::new(1002, "max4_alpha_numeric_text exceeds the maximum length of 4".to_string()));
			}
			let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
			if !pattern.is_match(&self.max4_alpha_numeric_text) {
				return Err(ValidationError::new(1005, "max4_alpha_numeric_text does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max70Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max70Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max70_text: String,
	}
	
	impl Max70Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max70_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max70_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max70_text.chars().count() > 70 {
				return Err(ValidationError::new(1002, "max70_text exceeds the maximum length of 70".to_string()));
			}
			Ok(())
		}
	}
	
	
	// MessageIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct MessageIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
		pub cre_dt_tm: String,
	}
	
	impl MessageIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// NameAndAddress5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct NameAndAddress5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
		pub nm: Max350Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Adr", skip_serializing_if = "Option::is_none") )]
		pub adr: Option<PostalAddress1>,
	}
	
	impl NameAndAddress5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.nm.validate() { return Err(e); }
			if let Some(ref adr_value) = self.adr { if let Err(e) = adr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// NotionalOrUnitBased1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct NotionalOrUnitBased1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<NotionalOrUnitBased1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl NotionalOrUnitBased1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// NotionalOrUnitBased1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// Number ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Number {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub number: f64,
	}
	
	impl Number {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// OrderDesk1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref ordr_dsk_value) = self.ordr_dsk { if let Err(e) = ordr_dsk_value.validate() { return Err(e); } }
			if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// OtherDistributionStrategy1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct OtherDistributionStrategy1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DstrbtnStrtgyTp", skip_serializing_if = "Option::is_none") )]
		pub dstrbtn_strtgy_tp: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Trgt", skip_serializing_if = "Option::is_none") )]
		pub trgt: Option<DistributionStrategy1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<AdditionalInformation15>,
	}
	
	impl OtherDistributionStrategy1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref dstrbtn_strtgy_tp_value) = self.dstrbtn_strtgy_tp { if let Err(e) = dstrbtn_strtgy_tp_value.validate() { return Err(e); } }
			if let Some(ref trgt_value) = self.trgt { if let Err(e) = trgt_value.validate() { return Err(e); } }
			if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// OtherIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct OtherIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sfx", skip_serializing_if = "Option::is_none") )]
		pub sfx: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: IdentificationSource3Choice,
	}
	
	impl OtherIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref sfx_value) = self.sfx { if let Err(e) = sfx_value.validate() { return Err(e); } }
			if let Err(e) = self.tp.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// OtherInvestmentNeed1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct OtherInvestmentNeed1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClntObjctvsAndNeedsTp", skip_serializing_if = "Option::is_none") )]
		pub clnt_objctvs_and_needs_tp: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Trgt", skip_serializing_if = "Option::is_none") )]
		pub trgt: Option<TargetMarket1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<AdditionalInformation15>,
	}
	
	impl OtherInvestmentNeed1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref clnt_objctvs_and_needs_tp_value) = self.clnt_objctvs_and_needs_tp { if let Err(e) = clnt_objctvs_and_needs_tp_value.validate() { return Err(e); } }
			if let Some(ref trgt_value) = self.trgt { if let Err(e) = trgt_value.validate() { return Err(e); } }
			if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// OtherReviewRelatedToValueAndOrChargesUKType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct OtherTargetMarket1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TrgtMktTp") )]
		pub trgt_mkt_tp: Max350Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<AdditionalInformation15>,
	}
	
	impl OtherTargetMarket1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.trgt_mkt_tp.validate() { return Err(e); }
			if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// OtherTargetMarketInvestor1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct OtherTargetMarketInvestor1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "InvstrTp", skip_serializing_if = "Option::is_none") )]
		pub invstr_tp: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Trgt", skip_serializing_if = "Option::is_none") )]
		pub trgt: Option<TargetMarket3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<AdditionalInformation15>,
	}
	
	impl OtherTargetMarketInvestor1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref invstr_tp_value) = self.invstr_tp { if let Err(e) = invstr_tp_value.validate() { return Err(e); } }
			if let Some(ref trgt_value) = self.trgt { if let Err(e) = trgt_value.validate() { return Err(e); } }
			if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// OtherTargetMarketInvestorKnowledge1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct OtherTargetMarketInvestorKnowledge1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "InvstrKnwldgTp", skip_serializing_if = "Option::is_none") )]
		pub invstr_knwldg_tp: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Trgt", skip_serializing_if = "Option::is_none") )]
		pub trgt: Option<TargetMarket1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<AdditionalInformation15>,
	}
	
	impl OtherTargetMarketInvestorKnowledge1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref invstr_knwldg_tp_value) = self.invstr_knwldg_tp { if let Err(e) = invstr_knwldg_tp_value.validate() { return Err(e); } }
			if let Some(ref trgt_value) = self.trgt { if let Err(e) = trgt_value.validate() { return Err(e); } }
			if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// OtherTargetMarketLossBearing1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct OtherTargetMarketLossBearing1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AbltyToBearLossesTp", skip_serializing_if = "Option::is_none") )]
		pub ablty_to_bear_losses_tp: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Trgt", skip_serializing_if = "Option::is_none") )]
		pub trgt: Option<TargetMarket1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<AdditionalInformation15>,
	}
	
	impl OtherTargetMarketLossBearing1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ablty_to_bear_losses_tp_value) = self.ablty_to_bear_losses_tp { if let Err(e) = ablty_to_bear_losses_tp_value.validate() { return Err(e); } }
			if let Some(ref trgt_value) = self.trgt { if let Err(e) = trgt_value.validate() { return Err(e); } }
			if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// OtherTargetMarketRiskTolerance1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct OtherTargetMarketRiskTolerance1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RskTlrnceTp", skip_serializing_if = "Option::is_none") )]
		pub rsk_tlrnce_tp: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Trgt", skip_serializing_if = "Option::is_none") )]
		pub trgt: Option<TargetMarket1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<AdditionalInformation15>,
	}
	
	impl OtherTargetMarketRiskTolerance1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref rsk_tlrnce_tp_value) = self.rsk_tlrnce_tp { if let Err(e) = rsk_tlrnce_tp_value.validate() { return Err(e); } }
			if let Some(ref trgt_value) = self.trgt { if let Err(e) = trgt_value.validate() { return Err(e); } }
			if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// OutcomeOfCOLLAssessmentOfValueUKType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PartyIdentification125Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
		pub any_bic: Option<AnyBICDec2014Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
		pub prtry_id: Option<GenericIdentification1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
		pub nm_and_adr: Option<NameAndAddress5>,
	}
	
	impl PartyIdentification125Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
			if let Some(ref prtry_id_value) = self.prtry_id { if let Err(e) = prtry_id_value.validate() { return Err(e); } }
			if let Some(ref nm_and_adr_value) = self.nm_and_adr { if let Err(e) = nm_and_adr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PartyIdentification139 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PartyIdentification139 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pty") )]
		pub pty: PartyIdentification125Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<LEIIdentifier>,
	}
	
	impl PartyIdentification139 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.pty.validate() { return Err(e); }
			if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaymentInstrument16 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Err(e) = self.ordr_tp.validate() { return Err(e); }
			if let Err(e) = self.instrm_tp.validate() { return Err(e); }
			if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// PercentageRate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct PercentageRate {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub percentage_rate: f64,
	}
	
	impl PercentageRate {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Period15 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// PhoneNumber ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct PhoneNumber {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub phone_number: String,
	}
	
	impl PhoneNumber {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(&self.phone_number) {
				return Err(ValidationError::new(1005, "phone_number does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// PlusOrMinusIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct PlusOrMinusIndicator {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub plus_or_minus_indicator: bool,
	}
	
	impl PlusOrMinusIndicator {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PostalAddress1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PostalAddress1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdrTp", skip_serializing_if = "Option::is_none") )]
		pub adr_tp: Option<AddressType2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdrLine", skip_serializing_if = "Option::is_none") )]
		pub adr_line: Option<Vec<Max70Text>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StrtNm", skip_serializing_if = "Option::is_none") )]
		pub strt_nm: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNb", skip_serializing_if = "Option::is_none") )]
		pub bldg_nb: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstCd", skip_serializing_if = "Option::is_none") )]
		pub pst_cd: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TwnNm", skip_serializing_if = "Option::is_none") )]
		pub twn_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none") )]
		pub ctry_sub_dvsn: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
		pub ctry: CountryCode,
	}
	
	impl PostalAddress1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref adr_tp_value) = self.adr_tp { if let Err(e) = adr_tp_value.validate() { return Err(e); } }
			if let Some(ref adr_line_vec) = self.adr_line { for item in adr_line_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref strt_nm_value) = self.strt_nm { if let Err(e) = strt_nm_value.validate() { return Err(e); } }
			if let Some(ref bldg_nb_value) = self.bldg_nb { if let Err(e) = bldg_nb_value.validate() { return Err(e); } }
			if let Some(ref pst_cd_value) = self.pst_cd { if let Err(e) = pst_cd_value.validate() { return Err(e); } }
			if let Some(ref twn_nm_value) = self.twn_nm { if let Err(e) = twn_nm_value.validate() { return Err(e); } }
			if let Some(ref ctry_sub_dvsn_value) = self.ctry_sub_dvsn { if let Err(e) = ctry_sub_dvsn_value.validate() { return Err(e); } }
			if let Err(e) = self.ctry.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// PriceMethod1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ProcessingCharacteristics10 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DealgCcyAccptd", skip_serializing_if = "Option::is_none") )]
		pub dealg_ccy_accptd: Option<Vec<ActiveCurrencyCode>>,
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
		pub dealg_frqcy_desc: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DealgCutOffTm", skip_serializing_if = "Option::is_none") )]
		pub dealg_cut_off_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DealgCutOffTmFrame", skip_serializing_if = "Option::is_none") )]
		pub dealg_cut_off_tm_frame: Option<TimeFrame9>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DealConfTm", skip_serializing_if = "Option::is_none") )]
		pub deal_conf_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DealConfTmFrame", skip_serializing_if = "Option::is_none") )]
		pub deal_conf_tm_frame: Option<TimeFrame8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LtdPrd", skip_serializing_if = "Option::is_none") )]
		pub ltd_prd: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmCycl", skip_serializing_if = "Option::is_none") )]
		pub sttlm_cycl: Option<TimeFrame8Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<Vec<AdditionalInformation15>>,
	}
	
	impl ProcessingCharacteristics10 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref dealg_ccy_accptd_vec) = self.dealg_ccy_accptd { for item in dealg_ccy_accptd_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref red_authstn_value) = self.red_authstn { if let Err(e) = red_authstn_value.validate() { return Err(e); } }
			if let Some(ref rndg_value) = self.rndg { if let Err(e) = rndg_value.validate() { return Err(e); } }
			if let Some(ref main_fnd_ordr_dsk_lctn_value) = self.main_fnd_ordr_dsk_lctn { if let Err(e) = main_fnd_ordr_dsk_lctn_value.validate() { return Err(e); } }
			if let Some(ref dealg_frqcy_value) = self.dealg_frqcy { if let Err(e) = dealg_frqcy_value.validate() { return Err(e); } }
			if let Some(ref dealg_frqcy_desc_value) = self.dealg_frqcy_desc { if let Err(e) = dealg_frqcy_desc_value.validate() { return Err(e); } }
			if let Some(ref dealg_cut_off_tm_frame_value) = self.dealg_cut_off_tm_frame { if let Err(e) = dealg_cut_off_tm_frame_value.validate() { return Err(e); } }
			if let Some(ref deal_conf_tm_frame_value) = self.deal_conf_tm_frame { if let Err(e) = deal_conf_tm_frame_value.validate() { return Err(e); } }
			if let Some(ref ltd_prd_value) = self.ltd_prd { if let Err(e) = ltd_prd_value.validate() { return Err(e); } }
			if let Some(ref sttlm_cycl_value) = self.sttlm_cycl { if let Err(e) = sttlm_cycl_value.validate() { return Err(e); } }
			if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// ProcessingCharacteristics11 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ProcessingCharacteristics11 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DealgCcyAccptd", skip_serializing_if = "Option::is_none") )]
		pub dealg_ccy_accptd: Option<Vec<ActiveCurrencyCode>>,
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
		pub dealg_frqcy_desc: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DealgCutOffTm", skip_serializing_if = "Option::is_none") )]
		pub dealg_cut_off_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DealgCutOffTmFrame", skip_serializing_if = "Option::is_none") )]
		pub dealg_cut_off_tm_frame: Option<TimeFrame9>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DealConfTm", skip_serializing_if = "Option::is_none") )]
		pub deal_conf_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DealConfTmFrame", skip_serializing_if = "Option::is_none") )]
		pub deal_conf_tm_frame: Option<TimeFrame11>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LtdPrd", skip_serializing_if = "Option::is_none") )]
		pub ltd_prd: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmCycl", skip_serializing_if = "Option::is_none") )]
		pub sttlm_cycl: Option<TimeFrame7Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<Vec<AdditionalInformation15>>,
	}
	
	impl ProcessingCharacteristics11 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref dealg_ccy_accptd_vec) = self.dealg_ccy_accptd { for item in dealg_ccy_accptd_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref initl_invstmt_appl_value) = self.initl_invstmt_appl { if let Err(e) = initl_invstmt_appl_value.validate() { return Err(e); } }
			if let Some(ref sbsqnt_invstmt_appl_value) = self.sbsqnt_invstmt_appl { if let Err(e) = sbsqnt_invstmt_appl_value.validate() { return Err(e); } }
			if let Some(ref rndg_value) = self.rndg { if let Err(e) = rndg_value.validate() { return Err(e); } }
			if let Some(ref main_fnd_ordr_dsk_lctn_value) = self.main_fnd_ordr_dsk_lctn { if let Err(e) = main_fnd_ordr_dsk_lctn_value.validate() { return Err(e); } }
			if let Some(ref dealg_frqcy_value) = self.dealg_frqcy { if let Err(e) = dealg_frqcy_value.validate() { return Err(e); } }
			if let Some(ref dealg_frqcy_desc_value) = self.dealg_frqcy_desc { if let Err(e) = dealg_frqcy_desc_value.validate() { return Err(e); } }
			if let Some(ref dealg_cut_off_tm_frame_value) = self.dealg_cut_off_tm_frame { if let Err(e) = dealg_cut_off_tm_frame_value.validate() { return Err(e); } }
			if let Some(ref deal_conf_tm_frame_value) = self.deal_conf_tm_frame { if let Err(e) = deal_conf_tm_frame_value.validate() { return Err(e); } }
			if let Some(ref ltd_prd_value) = self.ltd_prd { if let Err(e) = ltd_prd_value.validate() { return Err(e); } }
			if let Some(ref sttlm_cycl_value) = self.sttlm_cycl { if let Err(e) = sttlm_cycl_value.validate() { return Err(e); } }
			if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// ProcessingCharacteristics12 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ProcessingCharacteristics12 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DealgCcyAccptd", skip_serializing_if = "Option::is_none") )]
		pub dealg_ccy_accptd: Option<Vec<ActiveCurrencyCode>>,
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
		pub dealg_frqcy_desc: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DealgCutOffTm", skip_serializing_if = "Option::is_none") )]
		pub dealg_cut_off_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DealgCutOffTmFrame", skip_serializing_if = "Option::is_none") )]
		pub dealg_cut_off_tm_frame: Option<TimeFrame9>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DealConfTm", skip_serializing_if = "Option::is_none") )]
		pub deal_conf_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DealConfTmFrame", skip_serializing_if = "Option::is_none") )]
		pub deal_conf_tm_frame: Option<TimeFrame10>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LtdPrd", skip_serializing_if = "Option::is_none") )]
		pub ltd_prd: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmCycl", skip_serializing_if = "Option::is_none") )]
		pub sttlm_cycl: Option<TimeFrame8Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<Vec<AdditionalInformation15>>,
	}
	
	impl ProcessingCharacteristics12 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref dealg_ccy_accptd_vec) = self.dealg_ccy_accptd { for item in dealg_ccy_accptd_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref red_authstn_value) = self.red_authstn { if let Err(e) = red_authstn_value.validate() { return Err(e); } }
			if let Some(ref rndg_value) = self.rndg { if let Err(e) = rndg_value.validate() { return Err(e); } }
			if let Some(ref main_fnd_ordr_dsk_lctn_value) = self.main_fnd_ordr_dsk_lctn { if let Err(e) = main_fnd_ordr_dsk_lctn_value.validate() { return Err(e); } }
			if let Some(ref dealg_frqcy_value) = self.dealg_frqcy { if let Err(e) = dealg_frqcy_value.validate() { return Err(e); } }
			if let Some(ref dealg_frqcy_desc_value) = self.dealg_frqcy_desc { if let Err(e) = dealg_frqcy_desc_value.validate() { return Err(e); } }
			if let Some(ref dealg_cut_off_tm_frame_value) = self.dealg_cut_off_tm_frame { if let Err(e) = dealg_cut_off_tm_frame_value.validate() { return Err(e); } }
			if let Some(ref deal_conf_tm_frame_value) = self.deal_conf_tm_frame { if let Err(e) = deal_conf_tm_frame_value.validate() { return Err(e); } }
			if let Some(ref ltd_prd_value) = self.ltd_prd { if let Err(e) = ltd_prd_value.validate() { return Err(e); } }
			if let Some(ref sttlm_cycl_value) = self.sttlm_cycl { if let Err(e) = sttlm_cycl_value.validate() { return Err(e); } }
			if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// ProcessingCharacteristics9 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ProcessingCharacteristics9 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DealgCcyAccptd", skip_serializing_if = "Option::is_none") )]
		pub dealg_ccy_accptd: Option<Vec<ActiveCurrencyCode>>,
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
		pub dealg_frqcy_desc: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DealgCutOffTm", skip_serializing_if = "Option::is_none") )]
		pub dealg_cut_off_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DealgCutOffTmFrame", skip_serializing_if = "Option::is_none") )]
		pub dealg_cut_off_tm_frame: Option<TimeFrame9>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DealConfTm", skip_serializing_if = "Option::is_none") )]
		pub deal_conf_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DealConfTmFrame", skip_serializing_if = "Option::is_none") )]
		pub deal_conf_tm_frame: Option<TimeFrame8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LtdPrd", skip_serializing_if = "Option::is_none") )]
		pub ltd_prd: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmCycl", skip_serializing_if = "Option::is_none") )]
		pub sttlm_cycl: Option<TimeFrame8Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<Vec<AdditionalInformation15>>,
	}
	
	impl ProcessingCharacteristics9 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref dealg_ccy_accptd_vec) = self.dealg_ccy_accptd { for item in dealg_ccy_accptd_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref swtch_authstn_value) = self.swtch_authstn { if let Err(e) = swtch_authstn_value.validate() { return Err(e); } }
			if let Some(ref rndg_value) = self.rndg { if let Err(e) = rndg_value.validate() { return Err(e); } }
			if let Some(ref main_fnd_ordr_dsk_lctn_value) = self.main_fnd_ordr_dsk_lctn { if let Err(e) = main_fnd_ordr_dsk_lctn_value.validate() { return Err(e); } }
			if let Some(ref dealg_frqcy_value) = self.dealg_frqcy { if let Err(e) = dealg_frqcy_value.validate() { return Err(e); } }
			if let Some(ref dealg_frqcy_desc_value) = self.dealg_frqcy_desc { if let Err(e) = dealg_frqcy_desc_value.validate() { return Err(e); } }
			if let Some(ref dealg_cut_off_tm_frame_value) = self.dealg_cut_off_tm_frame { if let Err(e) = dealg_cut_off_tm_frame_value.validate() { return Err(e); } }
			if let Some(ref deal_conf_tm_frame_value) = self.deal_conf_tm_frame { if let Err(e) = deal_conf_tm_frame_value.validate() { return Err(e); } }
			if let Some(ref ltd_prd_value) = self.ltd_prd { if let Err(e) = ltd_prd_value.validate() { return Err(e); } }
			if let Some(ref sttlm_cycl_value) = self.sttlm_cycl { if let Err(e) = sttlm_cycl_value.validate() { return Err(e); } }
			if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// ProductStructure1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ProductStructure1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ProductStructure1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl ProductStructure1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ProductStructure1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct QuotationType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<QuotationType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl QuotationType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// QuotationType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref rsk_tlrnce_intl_value) = self.rsk_tlrnce_intl { if let Err(e) = rsk_tlrnce_intl_value.validate() { return Err(e); } }
			if let Some(ref not_for_invstrs_wth_the_lwst_rsk_tlrnce_de_value) = self.not_for_invstrs_wth_the_lwst_rsk_tlrnce_de { if let Err(e) = not_for_invstrs_wth_the_lwst_rsk_tlrnce_de_value.validate() { return Err(e); } }
			if let Some(ref othr_vec) = self.othr { for item in othr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// RoundingDirection2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SecurityClassificationType2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CFI", skip_serializing_if = "Option::is_none") )]
		pub cfi: Option<CFIOct2015Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AltrnClssfctn", skip_serializing_if = "Option::is_none") )]
		pub altrn_clssfctn: Option<GenericIdentification3>,
	}
	
	impl SecurityClassificationType2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cfi_value) = self.cfi { if let Err(e) = cfi_value.validate() { return Err(e); } }
			if let Some(ref altrn_clssfctn_value) = self.altrn_clssfctn { if let Err(e) = altrn_clssfctn_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SecurityIdentification40 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SecurityIdentification40 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrId", skip_serializing_if = "Option::is_none") )]
		pub othr_id: Option<Vec<OtherIdentification1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
		pub isin: Option<ISINOct2015Identifier>,
	}
	
	impl SecurityIdentification40 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref othr_id_vec) = self.othr_id { for item in othr_id_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
			if let Some(ref isin_value) = self.isin { if let Err(e) = isin_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SecurityIdentification47 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SecurityIdentification47 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: SecurityIdentification40,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
		pub nm: Max350Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none") )]
		pub shrt_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClssTp", skip_serializing_if = "Option::is_none") )]
		pub clss_tp: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UmbrllNm", skip_serializing_if = "Option::is_none") )]
		pub umbrll_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NewUmbrll", skip_serializing_if = "Option::is_none") )]
		pub new_umbrll: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none") )]
		pub clssfctn_tp: Option<SecurityClassificationType2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BaseCcy", skip_serializing_if = "Option::is_none") )]
		pub base_ccy: Option<ActiveCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfDmcl", skip_serializing_if = "Option::is_none") )]
		pub ctry_of_dmcl: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RegdDstrbtnCtry", skip_serializing_if = "Option::is_none") )]
		pub regd_dstrbtn_ctry: Option<Vec<CountryCode>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PdctTp", skip_serializing_if = "Option::is_none") )]
		pub pdct_tp: Option<ProductStructure1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<ContactAttributes5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IssrPdctGovncPrc", skip_serializing_if = "Option::is_none") )]
		pub issr_pdct_govnc_prc: Option<GovernanceProcess1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PdctCtgy", skip_serializing_if = "Option::is_none") )]
		pub pdct_ctgy: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PdctCtgyDE", skip_serializing_if = "Option::is_none") )]
		pub pdct_ctgy_de: Option<Max140Text>,
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
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Err(e) = self.nm.validate() { return Err(e); }
			if let Some(ref shrt_nm_value) = self.shrt_nm { if let Err(e) = shrt_nm_value.validate() { return Err(e); } }
			if let Some(ref clss_tp_value) = self.clss_tp { if let Err(e) = clss_tp_value.validate() { return Err(e); } }
			if let Some(ref umbrll_nm_value) = self.umbrll_nm { if let Err(e) = umbrll_nm_value.validate() { return Err(e); } }
			if let Some(ref clssfctn_tp_value) = self.clssfctn_tp { if let Err(e) = clssfctn_tp_value.validate() { return Err(e); } }
			if let Some(ref base_ccy_value) = self.base_ccy { if let Err(e) = base_ccy_value.validate() { return Err(e); } }
			if let Some(ref ctry_of_dmcl_value) = self.ctry_of_dmcl { if let Err(e) = ctry_of_dmcl_value.validate() { return Err(e); } }
			if let Some(ref regd_dstrbtn_ctry_vec) = self.regd_dstrbtn_ctry { for item in regd_dstrbtn_ctry_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref pdct_tp_value) = self.pdct_tp { if let Err(e) = pdct_tp_value.validate() { return Err(e); } }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			if let Some(ref issr_pdct_govnc_prc_value) = self.issr_pdct_govnc_prc { if let Err(e) = issr_pdct_govnc_prc_value.validate() { return Err(e); } }
			if let Some(ref pdct_ctgy_value) = self.pdct_ctgy { if let Err(e) = pdct_ctgy_value.validate() { return Err(e); } }
			if let Some(ref pdct_ctgy_de_value) = self.pdct_ctgy_de { if let Err(e) = pdct_ctgy_de_value.validate() { return Err(e); } }
			if let Some(ref ntnl_or_unit_based_value) = self.ntnl_or_unit_based { if let Err(e) = ntnl_or_unit_based_value.validate() { return Err(e); } }
			if let Some(ref qtn_tp_value) = self.qtn_tp { if let Err(e) = qtn_tp_value.validate() { return Err(e); } }
			if let Some(ref ex_pst_cost_clctn_bsis_value) = self.ex_pst_cost_clctn_bsis { if let Err(e) = ex_pst_cost_clctn_bsis_value.validate() { return Err(e); } }
			if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// SignatureType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct TargetMarket1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<TargetMarket1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl TargetMarket1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TargetMarket1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TargetMarket3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref invstr_tp_value) = self.invstr_tp { if let Err(e) = invstr_tp_value.validate() { return Err(e); } }
			if let Some(ref knwldg_and_or_exprnc_value) = self.knwldg_and_or_exprnc { if let Err(e) = knwldg_and_or_exprnc_value.validate() { return Err(e); } }
			if let Some(ref ablty_to_bear_losses_value) = self.ablty_to_bear_losses { if let Err(e) = ablty_to_bear_losses_value.validate() { return Err(e); } }
			if let Some(ref rsk_tlrnce_value) = self.rsk_tlrnce { if let Err(e) = rsk_tlrnce_value.validate() { return Err(e); } }
			if let Some(ref clnt_objctvs_and_needs_value) = self.clnt_objctvs_and_needs { if let Err(e) = clnt_objctvs_and_needs_value.validate() { return Err(e); } }
			if let Some(ref othr_vec) = self.othr { for item in othr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TargetMarket5Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct TargetMarket5Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<InvestorType4Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<TargetMarket1Code>,
	}
	
	impl TargetMarket5Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TimeFrame10 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct TimeFrame10 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrTmFrameDesc", skip_serializing_if = "Option::is_none") )]
		pub othr_tm_frame_desc: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TPlus", skip_serializing_if = "Option::is_none") )]
		pub t_plus: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NonWorkgDayAdjstmnt", skip_serializing_if = "Option::is_none") )]
		pub non_workg_day_adjstmnt: Option<BusinessDayConvention1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RefrToOrdrDsk", skip_serializing_if = "Option::is_none") )]
		pub refr_to_ordr_dsk: Option<ReferToFundOrderDesk1Code>,
	}
	
	impl TimeFrame10 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref othr_tm_frame_desc_value) = self.othr_tm_frame_desc { if let Err(e) = othr_tm_frame_desc_value.validate() { return Err(e); } }
			if let Some(ref non_workg_day_adjstmnt_value) = self.non_workg_day_adjstmnt { if let Err(e) = non_workg_day_adjstmnt_value.validate() { return Err(e); } }
			if let Some(ref refr_to_ordr_dsk_value) = self.refr_to_ordr_dsk { if let Err(e) = refr_to_ordr_dsk_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TimeFrame11 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct TimeFrame11 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrTmFrameDesc", skip_serializing_if = "Option::is_none") )]
		pub othr_tm_frame_desc: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TPlus", skip_serializing_if = "Option::is_none") )]
		pub t_plus: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NonWorkgDayAdjstmnt", skip_serializing_if = "Option::is_none") )]
		pub non_workg_day_adjstmnt: Option<BusinessDayConvention1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RefrToOrdrDsk", skip_serializing_if = "Option::is_none") )]
		pub refr_to_ordr_dsk: Option<ReferToFundOrderDesk1Code>,
	}
	
	impl TimeFrame11 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref othr_tm_frame_desc_value) = self.othr_tm_frame_desc { if let Err(e) = othr_tm_frame_desc_value.validate() { return Err(e); } }
			if let Some(ref non_workg_day_adjstmnt_value) = self.non_workg_day_adjstmnt { if let Err(e) = non_workg_day_adjstmnt_value.validate() { return Err(e); } }
			if let Some(ref refr_to_ordr_dsk_value) = self.refr_to_ordr_dsk { if let Err(e) = refr_to_ordr_dsk_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TimeFrame2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct TimeFrame8 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrTmFrameDesc", skip_serializing_if = "Option::is_none") )]
		pub othr_tm_frame_desc: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TPlus", skip_serializing_if = "Option::is_none") )]
		pub t_plus: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NonWorkgDayAdjstmnt", skip_serializing_if = "Option::is_none") )]
		pub non_workg_day_adjstmnt: Option<BusinessDayConvention1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RefrToOrdrDsk", skip_serializing_if = "Option::is_none") )]
		pub refr_to_ordr_dsk: Option<ReferToFundOrderDesk1Code>,
	}
	
	impl TimeFrame8 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref othr_tm_frame_desc_value) = self.othr_tm_frame_desc { if let Err(e) = othr_tm_frame_desc_value.validate() { return Err(e); } }
			if let Some(ref non_workg_day_adjstmnt_value) = self.non_workg_day_adjstmnt { if let Err(e) = non_workg_day_adjstmnt_value.validate() { return Err(e); } }
			if let Some(ref refr_to_ordr_dsk_value) = self.refr_to_ordr_dsk { if let Err(e) = refr_to_ordr_dsk_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TimeFrame8Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct TimeFrame9 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrTmFrameDesc", skip_serializing_if = "Option::is_none") )]
		pub othr_tm_frame_desc: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TMns", skip_serializing_if = "Option::is_none") )]
		pub t_mns: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NonWorkgDayAdjstmnt", skip_serializing_if = "Option::is_none") )]
		pub non_workg_day_adjstmnt: Option<BusinessDayConvention1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RefrToOrdrDsk", skip_serializing_if = "Option::is_none") )]
		pub refr_to_ordr_dsk: Option<ReferToFundOrderDesk1Code>,
	}
	
	impl TimeFrame9 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref othr_tm_frame_desc_value) = self.othr_tm_frame_desc { if let Err(e) = othr_tm_frame_desc_value.validate() { return Err(e); } }
			if let Some(ref non_workg_day_adjstmnt_value) = self.non_workg_day_adjstmnt { if let Err(e) = non_workg_day_adjstmnt_value.validate() { return Err(e); } }
			if let Some(ref refr_to_ordr_dsk_value) = self.refr_to_ordr_dsk { if let Err(e) = refr_to_ordr_dsk_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TimeFrame9Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct TimeFrame9Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<TimeFrame2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl TimeFrame9Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TimeHorizon2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct TimeHorizon2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfYrs", skip_serializing_if = "Option::is_none") )]
		pub nb_of_yrs: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TmFrame", skip_serializing_if = "Option::is_none") )]
		pub tm_frame: Option<TimeFrame9Choice>,
	}
	
	impl TimeHorizon2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tm_frame_value) = self.tm_frame { if let Err(e) = tm_frame_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// UTCOffset1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct UnitsOrAmount1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
		pub amt: Option<ActiveCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Unit", skip_serializing_if = "Option::is_none") )]
		pub unit: Option<f64>,
	}
	
	impl UnitsOrAmount1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ValuationDealingProcessingCharacteristics3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ValuationDealingProcessingCharacteristics3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ValtnFrqcy", skip_serializing_if = "Option::is_none") )]
		pub valtn_frqcy: Option<EventFrequency5Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ValtnFrqcyDesc", skip_serializing_if = "Option::is_none") )]
		pub valtn_frqcy_desc: Option<Max350Text>,
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
		pub pric_ccy: Option<Vec<ActiveCurrencyCode>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<Vec<AdditionalInformation15>>,
	}
	
	impl ValuationDealingProcessingCharacteristics3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref valtn_frqcy_value) = self.valtn_frqcy { if let Err(e) = valtn_frqcy_value.validate() { return Err(e); } }
			if let Some(ref valtn_frqcy_desc_value) = self.valtn_frqcy_desc { if let Err(e) = valtn_frqcy_desc_value.validate() { return Err(e); } }
			if let Some(ref pric_mtd_value) = self.pric_mtd { if let Err(e) = pric_mtd_value.validate() { return Err(e); } }
			if let Some(ref pric_ccy_vec) = self.pric_ccy { for item in pric_ccy_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// ValueForMoney1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
		pub frthr_inf_uk: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RvwDtUK", skip_serializing_if = "Option::is_none") )]
		pub rvw_dt_uk: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RvwNxtDueUK", skip_serializing_if = "Option::is_none") )]
		pub rvw_nxt_due_uk: Option<String>,
	}
	
	impl ValueForMoney1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref emt_data_rptg_vfmuk_value) = self.emt_data_rptg_vfmuk { if let Err(e) = emt_data_rptg_vfmuk_value.validate() { return Err(e); } }
			if let Some(ref assmnt_of_val_reqrd_udr_colluk_value) = self.assmnt_of_val_reqrd_udr_colluk { if let Err(e) = assmnt_of_val_reqrd_udr_colluk_value.validate() { return Err(e); } }
			if let Some(ref outcm_of_coll_assmnt_of_val_uk_value) = self.outcm_of_coll_assmnt_of_val_uk { if let Err(e) = outcm_of_coll_assmnt_of_val_uk_value.validate() { return Err(e); } }
			if let Some(ref outcm_of_prin_val_assmnt_or_rvw_uk_value) = self.outcm_of_prin_val_assmnt_or_rvw_uk { if let Err(e) = outcm_of_prin_val_assmnt_or_rvw_uk_value.validate() { return Err(e); } }
			if let Some(ref othr_rvw_rltd_to_val_and_or_chrgs_uk_value) = self.othr_rvw_rltd_to_val_and_or_chrgs_uk { if let Err(e) = othr_rvw_rltd_to_val_and_or_chrgs_uk_value.validate() { return Err(e); } }
			if let Some(ref frthr_inf_uk_value) = self.frthr_inf_uk { if let Err(e) = frthr_inf_uk_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// YesNoIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct YesNoIndicator {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub yes_no_indicator: bool,
	}
	
	impl YesNoIndicator {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
}