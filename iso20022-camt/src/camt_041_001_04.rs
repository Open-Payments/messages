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

pub mod iso20022 {
	use regex::Regex;
	use crate::common::*;
	#[cfg(feature = "derive_serde")]
	use serde::{Deserialize, Serialize};
	
	
	// ActiveCurrencyAnd13DecimalAmountSimpleType ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ActiveCurrencyCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ActiveOrHistoricCurrencyAndAmountSimpleType ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_or_historic_currency_and_amount_simple_type: f64,
	}
	
	impl ActiveOrHistoricCurrencyAndAmountSimpleType {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.active_or_historic_currency_and_amount_simple_type < 0.000000 {
				return Err(ValidationError::new(1003, "active_or_historic_currency_and_amount_simple_type is less than the minimum value of 0.000000".to_string()));
			}
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
	
	
	// ActiveOrHistoricCurrencyCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveOrHistoricCurrencyCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_or_historic_currency_code: String,
	}
	
	impl ActiveOrHistoricCurrencyCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(&self.active_or_historic_currency_code) {
				return Err(ValidationError::new(1005, "active_or_historic_currency_code does not match the required pattern".to_string()));
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
	pub struct AdditionalReference3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ref") )]
		pub ref_attr: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RefIssr", skip_serializing_if = "Option::is_none") )]
		pub ref_issr: Option<PartyIdentification2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgNm", skip_serializing_if = "Option::is_none") )]
		pub msg_nm: Option<Max35Text>,
	}
	
	impl AdditionalReference3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.ref_attr.validate() { return Err(e); }
			if let Some(ref ref_issr_value) = self.ref_issr { if let Err(e) = ref_issr_value.validate() { return Err(e); } }
			if let Some(ref msg_nm_value) = self.msg_nm { if let Err(e) = msg_nm_value.validate() { return Err(e); } }
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
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DmstIdSrc", skip_serializing_if = "Option::is_none") )]
		pub dmst_id_src: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryIdSrc", skip_serializing_if = "Option::is_none") )]
		pub prtry_id_src: Option<Max35Text>,
	}
	
	impl AlternateSecurityIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref dmst_id_src_value) = self.dmst_id_src { if let Err(e) = dmst_id_src_value.validate() { return Err(e); } }
			if let Some(ref prtry_id_src_value) = self.prtry_id_src { if let Err(e) = prtry_id_src_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AnyBICIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct AnyBICIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub any_bic_identifier: String,
	}
	
	impl AnyBICIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(&self.any_bic_identifier) {
				return Err(ValidationError::new(1005, "any_bic_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// BaseOneRate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct BaseOneRate {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub base_one_rate: f64,
	}
	
	impl BaseOneRate {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// BelgianIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct BelgianIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub belgian_identifier: String,
	}
	
	impl BelgianIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// BloombergIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct BloombergIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub bloomberg_identifier: String,
	}
	
	impl BloombergIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.bloomberg_identifier.chars().count() < 1 {
			return Err(ValidationError::new(1001, "bloomberg_identifier is shorter than the minimum length of 1".to_string()));
			}
			if self.bloomberg_identifier.chars().count() > 35 {
				return Err(ValidationError::new(1002, "bloomberg_identifier exceeds the maximum length of 35".to_string()));
			}
			Ok(())
		}
	}
	
	
	// CUSIPIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct CUSIPIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub cusip_identifier: String,
	}
	
	impl CUSIPIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
			if let Some(ref sub_ttl_amt_value) = self.sub_ttl_amt { if let Err(e) = sub_ttl_amt_value.validate() { return Err(e); } }
			if let Some(ref sub_ttl_units_nb_value) = self.sub_ttl_units_nb { if let Err(e) = sub_ttl_units_nb_value.validate() { return Err(e); } }
			if let Some(ref addtl_bal_value) = self.addtl_bal { if let Err(e) = addtl_bal_value.validate() { return Err(e); } }
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
			if let Err(e) = self.amt.validate() { return Err(e); }
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
			if let Some(ref sub_ttl_amt_value) = self.sub_ttl_amt { if let Err(e) = sub_ttl_amt_value.validate() { return Err(e); } }
			if let Some(ref sub_ttl_units_nb_value) = self.sub_ttl_units_nb { if let Err(e) = sub_ttl_units_nb_value.validate() { return Err(e); } }
			if let Some(ref addtl_bal_value) = self.addtl_bal { if let Err(e) = addtl_bal_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ConsolidatedTapeAssociationIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ConsolidatedTapeAssociationIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub consolidated_tape_association_identifier: String,
	}
	
	impl ConsolidatedTapeAssociationIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.consolidated_tape_association_identifier.chars().count() < 1 {
			return Err(ValidationError::new(1001, "consolidated_tape_association_identifier is shorter than the minimum length of 1".to_string()));
			}
			if self.consolidated_tape_association_identifier.chars().count() > 35 {
				return Err(ValidationError::new(1002, "consolidated_tape_association_identifier exceeds the maximum length of 35".to_string()));
			}
			Ok(())
		}
	}
	
	
	// CountryCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
		pub lctn: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<Max350Text>,
	}
	
	impl CurrencyDesignation1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ccy_dsgnt_value) = self.ccy_dsgnt { if let Err(e) = ccy_dsgnt_value.validate() { return Err(e); } }
			if let Some(ref lctn_value) = self.lctn { if let Err(e) = lctn_value.validate() { return Err(e); } }
			if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
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
	
	
	// DecimalNumber ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// DutchIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct DutchIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub dutch_identifier: String,
	}
	
	impl DutchIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// EuroclearClearstreamIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct EuroclearClearstreamIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub euroclear_clearstream_identifier: String,
	}
	
	impl EuroclearClearstreamIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.euroclear_clearstream_identifier.chars().count() < 1 {
			return Err(ValidationError::new(1001, "euroclear_clearstream_identifier is shorter than the minimum length of 1".to_string()));
			}
			if self.euroclear_clearstream_identifier.chars().count() > 12 {
				return Err(ValidationError::new(1002, "euroclear_clearstream_identifier exceeds the maximum length of 12".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Exact4AlphaNumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Extension1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
		pub nm: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryId", skip_serializing_if = "Option::is_none") )]
		pub splmtry_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdNAVCcy", skip_serializing_if = "Option::is_none") )]
		pub reqd_nav_ccy: Option<ActiveOrHistoricCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClssTp", skip_serializing_if = "Option::is_none") )]
		pub clss_tp: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesForm", skip_serializing_if = "Option::is_none") )]
		pub scties_form: Option<FormOfSecurity1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DstrbtnPlcy", skip_serializing_if = "Option::is_none") )]
		pub dstrbtn_plcy: Option<DistributionPolicy1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DualFndInd") )]
		pub dual_fnd_ind: bool,
	}
	
	impl FinancialInstrument9 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref splmtry_id_value) = self.splmtry_id { if let Err(e) = splmtry_id_value.validate() { return Err(e); } }
			if let Some(ref reqd_nav_ccy_value) = self.reqd_nav_ccy { if let Err(e) = reqd_nav_ccy_value.validate() { return Err(e); } }
			if let Some(ref clss_tp_value) = self.clss_tp { if let Err(e) = clss_tp_value.validate() { return Err(e); } }
			if let Some(ref scties_form_value) = self.scties_form { if let Err(e) = scties_form_value.validate() { return Err(e); } }
			if let Some(ref dstrbtn_plcy_value) = self.dstrbtn_plcy { if let Err(e) = dstrbtn_plcy_value.validate() { return Err(e); } }
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
		pub unit_ccy: ActiveCurrencyCode,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QtdCcy") )]
		pub qtd_ccy: ActiveCurrencyCode,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XchgRate") )]
		pub xchg_rate: f64,
	}
	
	impl ForeignExchangeTerms19 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.unit_ccy.validate() { return Err(e); }
			if let Err(e) = self.qtd_ccy.validate() { return Err(e); }
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
		pub nm: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none") )]
		pub lgl_ntty_idr: Option<LEIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<OtherIdentification4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy", skip_serializing_if = "Option::is_none") )]
		pub ccy: Option<ActiveOrHistoricCurrencyCode>,
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
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref lgl_ntty_idr_value) = self.lgl_ntty_idr { if let Err(e) = lgl_ntty_idr_value.validate() { return Err(e); } }
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			if let Some(ref ccy_value) = self.ccy { if let Err(e) = ccy_value.validate() { return Err(e); } }
			if let Some(ref trad_dt_tm_value) = self.trad_dt_tm { if let Err(e) = trad_dt_tm_value.validate() { return Err(e); } }
			if let Some(ref prvs_trad_dt_tm_value) = self.prvs_trad_dt_tm { if let Err(e) = prvs_trad_dt_tm_value.validate() { return Err(e); } }
			if let Some(ref ttl_nav_value) = self.ttl_nav { if let Err(e) = ttl_nav_value.validate() { return Err(e); } }
			if let Some(ref prvs_ttl_nav_value) = self.prvs_ttl_nav { if let Err(e) = prvs_ttl_nav_value.validate() { return Err(e); } }
			if let Some(ref ttl_units_nb_value) = self.ttl_units_nb { if let Err(e) = ttl_units_nb_value.validate() { return Err(e); } }
			if let Some(ref prvs_ttl_units_nb_value) = self.prvs_ttl_units_nb { if let Err(e) = prvs_ttl_units_nb_value.validate() { return Err(e); } }
			if let Some(ref csh_in_fcst_dtls_vec) = self.csh_in_fcst_dtls { for item in csh_in_fcst_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref csh_out_fcst_dtls_vec) = self.csh_out_fcst_dtls { for item in csh_out_fcst_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref net_csh_fcst_dtls_vec) = self.net_csh_fcst_dtls { for item in net_csh_fcst_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
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
			if let Some(ref ttl_units_fr_unit_ordrs_value) = self.ttl_units_fr_unit_ordrs { if let Err(e) = ttl_units_fr_unit_ordrs_value.validate() { return Err(e); } }
			if let Some(ref ttl_units_fr_csh_ordrs_value) = self.ttl_units_fr_csh_ordrs { if let Err(e) = ttl_units_fr_csh_ordrs_value.validate() { return Err(e); } }
			if let Some(ref ttl_csh_fr_unit_ordrs_value) = self.ttl_csh_fr_unit_ordrs { if let Err(e) = ttl_csh_fr_unit_ordrs_value.validate() { return Err(e); } }
			if let Some(ref ttl_csh_fr_csh_ordrs_value) = self.ttl_csh_fr_csh_ordrs { if let Err(e) = ttl_csh_fr_csh_ordrs_value.validate() { return Err(e); } }
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
		pub id: Max35Text,
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
		pub invstmt_ccy: Option<Vec<ActiveOrHistoricCurrencyCode>>,
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
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Err(e) = self.trad_dt_tm.validate() { return Err(e); }
			if let Some(ref prvs_trad_dt_tm_value) = self.prvs_trad_dt_tm { if let Err(e) = prvs_trad_dt_tm_value.validate() { return Err(e); } }
			if let Err(e) = self.fin_instrm_dtls.validate() { return Err(e); }
			if let Some(ref ttl_nav_vec) = self.ttl_nav { for item in ttl_nav_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref prvs_ttl_nav_vec) = self.prvs_ttl_nav { for item in prvs_ttl_nav_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref ttl_units_nb_value) = self.ttl_units_nb { if let Err(e) = ttl_units_nb_value.validate() { return Err(e); } }
			if let Some(ref prvs_ttl_units_nb_value) = self.prvs_ttl_units_nb { if let Err(e) = prvs_ttl_units_nb_value.validate() { return Err(e); } }
			if let Some(ref invstmt_ccy_vec) = self.invstmt_ccy { for item in invstmt_ccy_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref ccy_sts_value) = self.ccy_sts { if let Err(e) = ccy_sts_value.validate() { return Err(e); } }
			if let Some(ref pric_value) = self.pric { if let Err(e) = pric_value.validate() { return Err(e); } }
			if let Some(ref fx_rate_value) = self.fx_rate { if let Err(e) = fx_rate_value.validate() { return Err(e); } }
			if let Some(ref csh_in_fcst_dtls_vec) = self.csh_in_fcst_dtls { for item in csh_in_fcst_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref csh_out_fcst_dtls_vec) = self.csh_out_fcst_dtls { for item in csh_out_fcst_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref net_csh_fcst_dtls_vec) = self.net_csh_fcst_dtls { for item in net_csh_fcst_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
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
			if let Err(e) = self.msg_id.validate() { return Err(e); }
			if let Some(ref pool_ref_value) = self.pool_ref { if let Err(e) = pool_ref_value.validate() { return Err(e); } }
			if let Some(ref prvs_ref_vec) = self.prvs_ref { for item in prvs_ref_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref rltd_ref_vec) = self.rltd_ref { for item in rltd_ref_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Err(e) = self.msg_pgntn.validate() { return Err(e); }
			if let Some(ref fnd_or_sub_fnd_dtls_vec) = self.fnd_or_sub_fnd_dtls { for item in fnd_or_sub_fnd_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref fnd_csh_fcst_dtls_vec) = self.fnd_csh_fcst_dtls { for item in fnd_csh_fcst_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref cnsltd_net_csh_fcst_value) = self.cnsltd_net_csh_fcst { if let Err(e) = cnsltd_net_csh_fcst_value.validate() { return Err(e); } }
			if let Some(ref xtnsn_vec) = self.xtnsn { for item in xtnsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
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
	
	
	// GenericIdentification47 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ISINIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISINIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub isin_identifier: String,
	}
	
	impl ISINIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9]{12,12}").unwrap();
			if !pattern.is_match(&self.isin_identifier) {
				return Err(ValidationError::new(1005, "isin_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ISODate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// IdentificationSource5Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct IdentificationSource5Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DmstIdSrc", skip_serializing_if = "Option::is_none") )]
		pub dmst_id_src: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryIdSrc", skip_serializing_if = "Option::is_none") )]
		pub prtry_id_src: Option<Max35Text>,
	}
	
	impl IdentificationSource5Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref dmst_id_src_value) = self.dmst_id_src { if let Err(e) = dmst_id_src_value.validate() { return Err(e); } }
			if let Some(ref prtry_id_src_value) = self.prtry_id_src { if let Err(e) = prtry_id_src_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// LEIIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max16Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max350Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max5NumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max5NumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max5_numeric_text: String,
	}
	
	impl Max5NumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[0-9]{1,5}").unwrap();
			if !pattern.is_match(&self.max5_numeric_text) {
				return Err(ValidationError::new(1005, "max5_numeric_text does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max70Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if let Some(ref net_amt_value) = self.net_amt { if let Err(e) = net_amt_value.validate() { return Err(e); } }
			if let Some(ref net_units_nb_value) = self.net_units_nb { if let Err(e) = net_units_nb_value.validate() { return Err(e); } }
			if let Err(e) = self.flow_drctn.validate() { return Err(e); }
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
			if let Some(ref net_amt_value) = self.net_amt { if let Err(e) = net_amt_value.validate() { return Err(e); } }
			if let Some(ref net_units_nb_value) = self.net_units_nb { if let Err(e) = net_units_nb_value.validate() { return Err(e); } }
			if let Err(e) = self.flow_drctn.validate() { return Err(e); }
			if let Some(ref addtl_bal_value) = self.addtl_bal { if let Err(e) = addtl_bal_value.validate() { return Err(e); } }
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
			if let Some(ref net_amt_value) = self.net_amt { if let Err(e) = net_amt_value.validate() { return Err(e); } }
			if let Some(ref net_units_nb_value) = self.net_units_nb { if let Err(e) = net_units_nb_value.validate() { return Err(e); } }
			if let Err(e) = self.flow_drctn.validate() { return Err(e); }
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
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: IdentificationSource5Choice,
	}
	
	impl OtherIdentification4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Err(e) = self.tp.validate() { return Err(e); }
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
		pub pg_nb: Max5NumericText,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LastPgInd") )]
		pub last_pg_ind: bool,
	}
	
	impl Pagination {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.pg_nb.validate() { return Err(e); }
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
		pub bic_or_bei: Option<AnyBICIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
		pub prtry_id: Option<GenericIdentification1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
		pub nm_and_adr: Option<NameAndAddress5>,
	}
	
	impl PartyIdentification2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref bic_or_bei_value) = self.bic_or_bei { if let Err(e) = bic_or_bei_value.validate() { return Err(e); } }
			if let Some(ref prtry_id_value) = self.prtry_id { if let Err(e) = prtry_id_value.validate() { return Err(e); } }
			if let Some(ref nm_and_adr_value) = self.nm_and_adr { if let Err(e) = nm_and_adr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PercentageRate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if let Err(e) = self.amt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// QUICKIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct QUICKIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub quick_identifier: String,
	}
	
	impl QUICKIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// RICIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct RICIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub ric_identifier: String,
	}
	
	impl RICIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.ric_identifier.chars().count() < 1 {
			return Err(ValidationError::new(1001, "ric_identifier is shorter than the minimum length of 1".to_string()));
			}
			if self.ric_identifier.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ric_identifier exceeds the maximum length of 35".to_string()));
			}
			Ok(())
		}
	}
	
	
	// SEDOLIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct SEDOLIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub sedol_identifier: String,
	}
	
	impl SEDOLIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
		pub isin: Option<ISINIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SEDOL", skip_serializing_if = "Option::is_none") )]
		pub sedol: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CUSIP", skip_serializing_if = "Option::is_none") )]
		pub cusip: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RIC", skip_serializing_if = "Option::is_none") )]
		pub ric: Option<RICIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TckrSymb", skip_serializing_if = "Option::is_none") )]
		pub tckr_symb: Option<TickerIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Blmbrg", skip_serializing_if = "Option::is_none") )]
		pub blmbrg: Option<BloombergIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CTA", skip_serializing_if = "Option::is_none") )]
		pub cta: Option<ConsolidatedTapeAssociationIdentifier>,
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
		pub cmon: Option<EuroclearClearstreamIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPrtryId", skip_serializing_if = "Option::is_none") )]
		pub othr_prtry_id: Option<AlternateSecurityIdentification1>,
	}
	
	impl SecurityIdentification3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref isin_value) = self.isin { if let Err(e) = isin_value.validate() { return Err(e); } }
			if let Some(ref ric_value) = self.ric { if let Err(e) = ric_value.validate() { return Err(e); } }
			if let Some(ref tckr_symb_value) = self.tckr_symb { if let Err(e) = tckr_symb_value.validate() { return Err(e); } }
			if let Some(ref blmbrg_value) = self.blmbrg { if let Err(e) = blmbrg_value.validate() { return Err(e); } }
			if let Some(ref cta_value) = self.cta { if let Err(e) = cta_value.validate() { return Err(e); } }
			if let Some(ref cmon_value) = self.cmon { if let Err(e) = cmon_value.validate() { return Err(e); } }
			if let Some(ref othr_prtry_id_value) = self.othr_prtry_id { if let Err(e) = othr_prtry_id_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SicovamIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct SicovamIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub sicovam_identifier: String,
	}
	
	impl SicovamIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// TickerIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct TickerIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub ticker_identifier: String,
	}
	
	impl TickerIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.ticker_identifier.chars().count() < 1 {
			return Err(ValidationError::new(1001, "ticker_identifier is shorter than the minimum length of 1".to_string()));
			}
			if self.ticker_identifier.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ticker_identifier exceeds the maximum length of 35".to_string()));
			}
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
			if let Err(e) = self.pric_tp.validate() { return Err(e); }
			if let Err(e) = self.val.validate() { return Err(e); }
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
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ValorenIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ValorenIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub valoren_identifier: String,
	}
	
	impl ValorenIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// WertpapierIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct WertpapierIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub wertpapier_identifier: String,
	}
	
	impl WertpapierIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// YesNoIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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