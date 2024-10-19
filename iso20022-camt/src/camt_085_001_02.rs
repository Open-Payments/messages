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
	
	
	// AccountIdentification4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AccountIdentification4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "IBAN", skip_serializing_if = "Option::is_none") )]
		pub iban: Option<IBAN2007Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<GenericAccountIdentification1>,
	}
	
	impl AccountIdentification4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref iban_value) = self.iban { if let Err(e) = iban_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
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
	
	
	// AcknowledgedAcceptedStatus21Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AcknowledgedAcceptedStatus21Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
		pub no_spcfd_rsn: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<Vec<AcknowledgementReason9>>,
	}
	
	impl AcknowledgedAcceptedStatus21Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref no_spcfd_rsn_value) = self.no_spcfd_rsn { if let Err(e) = no_spcfd_rsn_value.validate() { return Err(e); } }
			if let Some(ref rsn_vec) = self.rsn { for item in rsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// AcknowledgementReason12Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AcknowledgementReason12Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<AcknowledgementReason5Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl AcknowledgementReason12Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AcknowledgementReason5Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AcknowledgementReason5Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ADEA") )]
		CodeADEA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SMPG") )]
		CodeSMPG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CDCY") )]
		CodeCDCY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CDRG") )]
		CodeCDRG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CDRE") )]
		CodeCDRE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NSTP") )]
		CodeNSTP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RQWV") )]
		CodeRQWV,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LATE") )]
		CodeLATE,
	}
	
	impl AcknowledgementReason5Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AcknowledgementReason9 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AcknowledgementReason9 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: AcknowledgementReason12Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_rsn_inf: Option<Max210Text>,
	}
	
	impl AcknowledgementReason9 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref addtl_rsn_inf_value) = self.addtl_rsn_inf { if let Err(e) = addtl_rsn_inf_value.validate() { return Err(e); } }
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
	
	
	// ActiveOrHistoricCurrencyCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// AddressType3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AddressType3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<AddressType2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl AddressType3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Amount2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Amount2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AmtWthtCcy", skip_serializing_if = "Option::is_none") )]
		pub amt_wtht_ccy: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AmtWthCcy", skip_serializing_if = "Option::is_none") )]
		pub amt_wth_ccy: Option<ActiveCurrencyAndAmount>,
	}
	
	impl Amount2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref amt_wth_ccy_value) = self.amt_wth_ccy { if let Err(e) = amt_wth_ccy_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AmountAndDirection5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AmountAndDirection5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none") )]
		pub cdt_dbt: Option<CreditDebitCode>,
	}
	
	impl AmountAndDirection5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.amt.validate() { return Err(e); }
			if let Some(ref cdt_dbt_value) = self.cdt_dbt { if let Err(e) = cdt_dbt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AmountAndQuantityBreakdown1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AmountAndQuantityBreakdown1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "LotNb", skip_serializing_if = "Option::is_none") )]
		pub lot_nb: Option<GenericIdentification37>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LotAmt", skip_serializing_if = "Option::is_none") )]
		pub lot_amt: Option<AmountAndDirection5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LotQty", skip_serializing_if = "Option::is_none") )]
		pub lot_qty: Option<FinancialInstrumentQuantity1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CshSubBalTp", skip_serializing_if = "Option::is_none") )]
		pub csh_sub_bal_tp: Option<GenericIdentification30>,
	}
	
	impl AmountAndQuantityBreakdown1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref lot_nb_value) = self.lot_nb { if let Err(e) = lot_nb_value.validate() { return Err(e); } }
			if let Some(ref lot_amt_value) = self.lot_amt { if let Err(e) = lot_amt_value.validate() { return Err(e); } }
			if let Some(ref lot_qty_value) = self.lot_qty { if let Err(e) = lot_qty_value.validate() { return Err(e); } }
			if let Some(ref csh_sub_bal_tp_value) = self.csh_sub_bal_tp { if let Err(e) = csh_sub_bal_tp_value.validate() { return Err(e); } }
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
	
	
	// BICFIDec2014Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct BICFIDec2014Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub bicfi_dec2014_identifier: String,
	}
	
	impl BICFIDec2014Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(&self.bicfi_dec2014_identifier) {
				return Err(ValidationError::new(1005, "bicfi_dec2014_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// BranchAndFinancialInstitutionIdentification8 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BranchAndFinancialInstitutionIdentification8 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstnId") )]
		pub fin_instn_id: FinancialInstitutionIdentification23,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BrnchId", skip_serializing_if = "Option::is_none") )]
		pub brnch_id: Option<BranchData5>,
	}
	
	impl BranchAndFinancialInstitutionIdentification8 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.fin_instn_id.validate() { return Err(e); }
			if let Some(ref brnch_id_value) = self.brnch_id { if let Err(e) = brnch_id_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BranchData5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BranchData5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<LEIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
		pub pstl_adr: Option<PostalAddress27>,
	}
	
	impl BranchData5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref pstl_adr_value) = self.pstl_adr { if let Err(e) = pstl_adr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CancellationReason19Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CancellationReason19Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<CancelledStatusReason13Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl CancellationReason19Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CancellationReason9 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CancellationReason9 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: CancellationReason19Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_rsn_inf: Option<Max210Text>,
	}
	
	impl CancellationReason9 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref addtl_rsn_inf_value) = self.addtl_rsn_inf { if let Err(e) = addtl_rsn_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CancellationStatus14Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CancellationStatus14Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
		pub no_spcfd_rsn: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<Vec<CancellationReason9>>,
	}
	
	impl CancellationStatus14Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref no_spcfd_rsn_value) = self.no_spcfd_rsn { if let Err(e) = no_spcfd_rsn_value.validate() { return Err(e); } }
			if let Some(ref rsn_vec) = self.rsn { for item in rsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// CancelledStatusReason13Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum CancelledStatusReason13Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CANI") )]
		CodeCANI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CANS") )]
		CodeCANS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CSUB") )]
		CodeCSUB,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CXLR") )]
		CodeCXLR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CANT") )]
		CodeCANT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CANZ") )]
		CodeCANZ,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CORP") )]
		CodeCORP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SCEX") )]
		CodeSCEX,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CTHP") )]
		CodeCTHP,
	}
	
	impl CancelledStatusReason13Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CashAccount40 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CashAccount40 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<AccountIdentification4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<CashAccountType2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy", skip_serializing_if = "Option::is_none") )]
		pub ccy: Option<ActiveOrHistoricCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prxy", skip_serializing_if = "Option::is_none") )]
		pub prxy: Option<ProxyAccountIdentification1>,
	}
	
	impl CashAccount40 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref ccy_value) = self.ccy { if let Err(e) = ccy_value.validate() { return Err(e); } }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref prxy_value) = self.prxy { if let Err(e) = prxy_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CashAccountType2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CashAccountType2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalCashAccountType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl CashAccountType2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CashBalanceType3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CashBalanceType3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalBalanceType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl CashBalanceType3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CashSubBalanceTypeAndQuantityBreakdown3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CashSubBalanceTypeAndQuantityBreakdown3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: CashBalanceType3Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QtyBrkdwn", skip_serializing_if = "Option::is_none") )]
		pub qty_brkdwn: Option<Vec<AmountAndQuantityBreakdown1>>,
	}
	
	impl CashSubBalanceTypeAndQuantityBreakdown3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Some(ref qty_brkdwn_vec) = self.qty_brkdwn { for item in qty_brkdwn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// ClearingSystemIdentification2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ClearingSystemIdentification2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalClearingSystemIdentification1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl ClearingSystemIdentification2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ClearingSystemMemberIdentification2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ClearingSystemMemberIdentification2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysId", skip_serializing_if = "Option::is_none") )]
		pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MmbId") )]
		pub mmb_id: Max35Text,
	}
	
	impl ClearingSystemMemberIdentification2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref clr_sys_id_value) = self.clr_sys_id { if let Err(e) = clr_sys_id_value.validate() { return Err(e); } }
			if let Err(e) = self.mmb_id.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// CopyDuplicate1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum CopyDuplicate1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CODU") )]
		CodeCODU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "COPY") )]
		CodeCOPY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DUPL") )]
		CodeDUPL,
	}
	
	impl CopyDuplicate1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	
	
	// CreditDebitCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// DateAndDateTime2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// DateTimePeriod1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// DocumentIdentification51 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct DocumentIdentification51 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
		pub cre_dt_tm: Option<DateAndDateTime2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CpyDplct", skip_serializing_if = "Option::is_none") )]
		pub cpy_dplct: Option<CopyDuplicate1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgOrgtr", skip_serializing_if = "Option::is_none") )]
		pub msg_orgtr: Option<PartyIdentification136>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgRcpt", skip_serializing_if = "Option::is_none") )]
		pub msg_rcpt: Option<PartyIdentification136>,
	}
	
	impl DocumentIdentification51 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref cre_dt_tm_value) = self.cre_dt_tm { if let Err(e) = cre_dt_tm_value.validate() { return Err(e); } }
			if let Some(ref cpy_dplct_value) = self.cpy_dplct { if let Err(e) = cpy_dplct_value.validate() { return Err(e); } }
			if let Some(ref msg_orgtr_value) = self.msg_orgtr { if let Err(e) = msg_orgtr_value.validate() { return Err(e); } }
			if let Some(ref msg_rcpt_value) = self.msg_rcpt { if let Err(e) = msg_rcpt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DocumentNumber5Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct DocumentNumber5Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtNb", skip_serializing_if = "Option::is_none") )]
		pub shrt_nb: Option<Exact3NumericText>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LngNb", skip_serializing_if = "Option::is_none") )]
		pub lng_nb: Option<ISO20022MessageIdentificationText>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryNb", skip_serializing_if = "Option::is_none") )]
		pub prtry_nb: Option<GenericIdentification36>,
	}
	
	impl DocumentNumber5Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref shrt_nb_value) = self.shrt_nb { if let Err(e) = shrt_nb_value.validate() { return Err(e); } }
			if let Some(ref lng_nb_value) = self.lng_nb { if let Err(e) = lng_nb_value.validate() { return Err(e); } }
			if let Some(ref prtry_nb_value) = self.prtry_nb { if let Err(e) = prtry_nb_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// EventFrequency7Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum EventFrequency7Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
		CodeYEAR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ADHO") )]
		CodeADHO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
		CodeMNTH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DAIL") )]
		CodeDAIL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INDA") )]
		CodeINDA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WEEK") )]
		CodeWEEK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SEMI") )]
		CodeSEMI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QUTR") )]
		CodeQUTR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TOMN") )]
		CodeTOMN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TOWK") )]
		CodeTOWK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TWMN") )]
		CodeTWMN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OVNG") )]
		CodeOVNG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ONDE") )]
		CodeONDE,
	}
	
	impl EventFrequency7Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Exact3NumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Exact3NumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub exact3_numeric_text: String,
	}
	
	impl Exact3NumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[0-9]{3}").unwrap();
			if !pattern.is_match(&self.exact3_numeric_text) {
				return Err(ValidationError::new(1005, "exact3_numeric_text does not match the required pattern".to_string()));
			}
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
	
	
	// Exact4NumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Exact4NumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub exact4_numeric_text: String,
	}
	
	impl Exact4NumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[0-9]{4}").unwrap();
			if !pattern.is_match(&self.exact4_numeric_text) {
				return Err(ValidationError::new(1005, "exact4_numeric_text does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Exact5NumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Exact5NumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub exact5_numeric_text: String,
	}
	
	impl Exact5NumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[0-9]{5}").unwrap();
			if !pattern.is_match(&self.exact5_numeric_text) {
				return Err(ValidationError::new(1005, "exact5_numeric_text does not match the required pattern".to_string()));
			}
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
	
	
	// ExternalBalanceType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalBalanceType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_balance_type1_code: String,
	}
	
	impl ExternalBalanceType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_balance_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_balance_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_balance_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_balance_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalCashAccountType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalCashAccountType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_cash_account_type1_code: String,
	}
	
	impl ExternalCashAccountType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_cash_account_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_cash_account_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_cash_account_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_cash_account_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalClearingSystemIdentification1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalClearingSystemIdentification1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_clearing_system_identification1_code: String,
	}
	
	impl ExternalClearingSystemIdentification1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_clearing_system_identification1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_clearing_system_identification1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_clearing_system_identification1_code.chars().count() > 5 {
				return Err(ValidationError::new(1002, "external_clearing_system_identification1_code exceeds the maximum length of 5".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalFinancialInstitutionIdentification1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalFinancialInstitutionIdentification1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_financial_institution_identification1_code: String,
	}
	
	impl ExternalFinancialInstitutionIdentification1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_financial_institution_identification1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_financial_institution_identification1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_financial_institution_identification1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_financial_institution_identification1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalProxyAccountType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalProxyAccountType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_proxy_account_type1_code: String,
	}
	
	impl ExternalProxyAccountType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_proxy_account_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_proxy_account_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_proxy_account_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_proxy_account_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// FailingReason3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum FailingReason3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "AWMO") )]
		CodeAWMO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BYIY") )]
		CodeBYIY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLAT") )]
		CodeCLAT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ADEA") )]
		CodeADEA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CANR") )]
		CodeCANR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CAIS") )]
		CodeCAIS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OBJT") )]
		CodeOBJT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AWSH") )]
		CodeAWSH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PHSE") )]
		CodePHSE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "STCD") )]
		CodeSTCD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DOCY") )]
		CodeDOCY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MLAT") )]
		CodeMLAT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DOCC") )]
		CodeDOCC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BLOC") )]
		CodeBLOC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CHAS") )]
		CodeCHAS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NEWI") )]
		CodeNEWI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLAC") )]
		CodeCLAC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MUNO") )]
		CodeMUNO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GLOB") )]
		CodeGLOB,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PREA") )]
		CodePREA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PART") )]
		CodePART,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NOFX") )]
		CodeNOFX,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CMON") )]
		CodeCMON,
		#[cfg_attr( feature = "derive_serde", serde(rename = "YCOL") )]
		CodeYCOL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "COLL") )]
		CodeCOLL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DEPO") )]
		CodeDEPO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FLIM") )]
		CodeFLIM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INCA") )]
		CodeINCA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LINK") )]
		CodeLINK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LACK") )]
		CodeLACK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LALO") )]
		CodeLALO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MONY") )]
		CodeMONY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NCON") )]
		CodeNCON,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REFS") )]
		CodeREFS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SDUT") )]
		CodeSDUT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BATC") )]
		CodeBATC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CYCL") )]
		CodeCYCL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SBLO") )]
		CodeSBLO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CPEC") )]
		CodeCPEC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MINO") )]
		CodeMINO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IAAD") )]
		CodeIAAD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PHCK") )]
		CodePHCK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BENO") )]
		CodeBENO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BOTH") )]
		CodeBOTH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLHT") )]
		CodeCLHT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DENO") )]
		CodeDENO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DISA") )]
		CodeDISA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DKNY") )]
		CodeDKNY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FROZ") )]
		CodeFROZ,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LAAW") )]
		CodeLAAW,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LATE") )]
		CodeLATE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LIQU") )]
		CodeLIQU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRCY") )]
		CodePRCY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REGT") )]
		CodeREGT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SETS") )]
		CodeSETS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CERT") )]
		CodeCERT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRSY") )]
		CodePRSY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INBC") )]
		CodeINBC,
	}
	
	impl FailingReason3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// FailingReason7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FailingReason7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: FailingReason7Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_rsn_inf: Option<Max210Text>,
	}
	
	impl FailingReason7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref addtl_rsn_inf_value) = self.addtl_rsn_inf { if let Err(e) = addtl_rsn_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FailingReason7Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FailingReason7Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<FailingReason3Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl FailingReason7Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FailingStatus9Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FailingStatus9Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
		pub no_spcfd_rsn: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<Vec<FailingReason7>>,
	}
	
	impl FailingStatus9Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref no_spcfd_rsn_value) = self.no_spcfd_rsn { if let Err(e) = no_spcfd_rsn_value.validate() { return Err(e); } }
			if let Some(ref rsn_vec) = self.rsn { for item in rsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// FinancialIdentificationSchemeName1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FinancialIdentificationSchemeName1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalFinancialInstitutionIdentification1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl FinancialIdentificationSchemeName1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FinancialInstitutionIdentification23 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FinancialInstitutionIdentification23 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BICFI", skip_serializing_if = "Option::is_none") )]
		pub bicfi: Option<BICFIDec2014Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none") )]
		pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<LEIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
		pub pstl_adr: Option<PostalAddress27>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<GenericFinancialIdentification1>,
	}
	
	impl FinancialInstitutionIdentification23 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref bicfi_value) = self.bicfi { if let Err(e) = bicfi_value.validate() { return Err(e); } }
			if let Some(ref clr_sys_mmb_id_value) = self.clr_sys_mmb_id { if let Err(e) = clr_sys_mmb_id_value.validate() { return Err(e); } }
			if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref pstl_adr_value) = self.pstl_adr { if let Err(e) = pstl_adr_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FinancialInstrumentQuantity1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			Ok(())
		}
	}
	
	
	// Frequency22Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Frequency22Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<EventFrequency7Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl Frequency22Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
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
	
	
	// GenericFinancialIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GenericFinancialIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<FinancialIdentificationSchemeName1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl GenericFinancialIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GenericIdentification30 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GenericIdentification30 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Exact4AlphaNumericText,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
		pub issr: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<Max35Text>,
	}
	
	impl GenericIdentification30 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Err(e) = self.issr.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
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
	
	
	// GenericIdentification37 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GenericIdentification37 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl GenericIdentification37 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
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
	
	
	// ISO20022MessageIdentificationText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISO20022MessageIdentificationText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iso20022_message_identification_text: String,
	}
	
	impl ISO20022MessageIdentificationText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[a-z]{4}\\.[0-9]{3}\\.[0-9]{3}\\.[0-9]{2}").unwrap();
			if !pattern.is_match(&self.iso20022_message_identification_text) {
				return Err(ValidationError::new(1005, "iso20022_message_identification_text does not match the required pattern".to_string()));
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
	
	
	// ImpliedCurrencyAndAmount ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ImpliedCurrencyAndAmount {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub implied_currency_and_amount: f64,
	}
	
	impl ImpliedCurrencyAndAmount {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.implied_currency_and_amount < 0.000000 {
				return Err(ValidationError::new(1003, "implied_currency_and_amount is less than the minimum value of 0.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// IntraBalanceMovementPendingReportV02 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct IntraBalanceMovementPendingReportV02 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<DocumentIdentification51>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pgntn") )]
		pub pgntn: Pagination1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptGnlDtls") )]
		pub rpt_gnl_dtls: IntraBalanceReport6,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CshAcct") )]
		pub csh_acct: CashAccount40,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CshAcctOwnr", skip_serializing_if = "Option::is_none") )]
		pub csh_acct_ownr: Option<SystemPartyIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CshAcctSvcr", skip_serializing_if = "Option::is_none") )]
		pub csh_acct_svcr: Option<BranchAndFinancialInstitutionIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Mvmnts", skip_serializing_if = "Option::is_none") )]
		pub mvmnts: Option<Vec<IntraBalancePending5>>,
	}
	
	impl IntraBalanceMovementPendingReportV02 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			if let Err(e) = self.pgntn.validate() { return Err(e); }
			if let Err(e) = self.rpt_gnl_dtls.validate() { return Err(e); }
			if let Err(e) = self.csh_acct.validate() { return Err(e); }
			if let Some(ref csh_acct_ownr_value) = self.csh_acct_ownr { if let Err(e) = csh_acct_ownr_value.validate() { return Err(e); } }
			if let Some(ref csh_acct_svcr_value) = self.csh_acct_svcr { if let Err(e) = csh_acct_svcr_value.validate() { return Err(e); } }
			if let Some(ref mvmnts_vec) = self.mvmnts { for item in mvmnts_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// IntraBalancePending5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct IntraBalancePending5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "StsAndRsn", skip_serializing_if = "Option::is_none") )]
		pub sts_and_rsn: Option<PendingStatusAndReason2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Mvmnt") )]
		pub mvmnt: Vec<IntraBalancePending6>,
	}
	
	impl IntraBalancePending5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref sts_and_rsn_value) = self.sts_and_rsn { if let Err(e) = sts_and_rsn_value.validate() { return Err(e); } }
			for item in &self.mvmnt { if let Err(e) = item.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// IntraBalancePending6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct IntraBalancePending6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "StsAndRsn", skip_serializing_if = "Option::is_none") )]
		pub sts_and_rsn: Option<PendingStatusAndReason2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctOwnrTxId") )]
		pub acct_ownr_tx_id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrTxId", skip_serializing_if = "Option::is_none") )]
		pub acct_svcr_tx_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MktInfrstrctrTxId", skip_serializing_if = "Option::is_none") )]
		pub mkt_infrstrctr_tx_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrcrTxId", skip_serializing_if = "Option::is_none") )]
		pub prcr_tx_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PoolId", skip_serializing_if = "Option::is_none") )]
		pub pool_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CorpActnEvtId", skip_serializing_if = "Option::is_none") )]
		pub corp_actn_evt_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BalFr") )]
		pub bal_fr: CashSubBalanceTypeAndQuantityBreakdown3,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BalTo") )]
		pub bal_to: CashSubBalanceTypeAndQuantityBreakdown3,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmAmt") )]
		pub sttlm_amt: Amount2Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntnddSttlmDt") )]
		pub intndd_sttlm_dt: DateAndDateTime2Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StsDt", skip_serializing_if = "Option::is_none") )]
		pub sts_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CshSubBalId", skip_serializing_if = "Option::is_none") )]
		pub csh_sub_bal_id: Option<GenericIdentification37>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Lnkgs", skip_serializing_if = "Option::is_none") )]
		pub lnkgs: Option<Vec<Linkages57>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prty", skip_serializing_if = "Option::is_none") )]
		pub prty: Option<PriorityNumeric4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgOrgtr", skip_serializing_if = "Option::is_none") )]
		pub msg_orgtr: Option<SystemPartyIdentification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
		pub cre_dt_tm: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstrPrcgAddtlDtls", skip_serializing_if = "Option::is_none") )]
		pub instr_prcg_addtl_dtls: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl IntraBalancePending6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref sts_and_rsn_value) = self.sts_and_rsn { if let Err(e) = sts_and_rsn_value.validate() { return Err(e); } }
			if let Err(e) = self.acct_ownr_tx_id.validate() { return Err(e); }
			if let Some(ref acct_svcr_tx_id_value) = self.acct_svcr_tx_id { if let Err(e) = acct_svcr_tx_id_value.validate() { return Err(e); } }
			if let Some(ref mkt_infrstrctr_tx_id_value) = self.mkt_infrstrctr_tx_id { if let Err(e) = mkt_infrstrctr_tx_id_value.validate() { return Err(e); } }
			if let Some(ref prcr_tx_id_value) = self.prcr_tx_id { if let Err(e) = prcr_tx_id_value.validate() { return Err(e); } }
			if let Some(ref pool_id_value) = self.pool_id { if let Err(e) = pool_id_value.validate() { return Err(e); } }
			if let Some(ref corp_actn_evt_id_value) = self.corp_actn_evt_id { if let Err(e) = corp_actn_evt_id_value.validate() { return Err(e); } }
			if let Err(e) = self.bal_fr.validate() { return Err(e); }
			if let Err(e) = self.bal_to.validate() { return Err(e); }
			if let Err(e) = self.sttlm_amt.validate() { return Err(e); }
			if let Err(e) = self.intndd_sttlm_dt.validate() { return Err(e); }
			if let Some(ref csh_sub_bal_id_value) = self.csh_sub_bal_id { if let Err(e) = csh_sub_bal_id_value.validate() { return Err(e); } }
			if let Some(ref lnkgs_vec) = self.lnkgs { for item in lnkgs_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref prty_value) = self.prty { if let Err(e) = prty_value.validate() { return Err(e); } }
			if let Some(ref msg_orgtr_value) = self.msg_orgtr { if let Err(e) = msg_orgtr_value.validate() { return Err(e); } }
			if let Some(ref instr_prcg_addtl_dtls_value) = self.instr_prcg_addtl_dtls { if let Err(e) = instr_prcg_addtl_dtls_value.validate() { return Err(e); } }
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// IntraBalanceReport6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct IntraBalanceReport6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptNb", skip_serializing_if = "Option::is_none") )]
		pub rpt_nb: Option<Number3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QryRef", skip_serializing_if = "Option::is_none") )]
		pub qry_ref: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptId", skip_serializing_if = "Option::is_none") )]
		pub rpt_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptDtTm", skip_serializing_if = "Option::is_none") )]
		pub rpt_dt_tm: Option<DateAndDateTime2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptPrd", skip_serializing_if = "Option::is_none") )]
		pub rpt_prd: Option<Period7Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Frqcy", skip_serializing_if = "Option::is_none") )]
		pub frqcy: Option<Frequency22Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UpdTp") )]
		pub upd_tp: UpdateType15Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ActvtyInd") )]
		pub actvty_ind: bool,
	}
	
	impl IntraBalanceReport6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref rpt_nb_value) = self.rpt_nb { if let Err(e) = rpt_nb_value.validate() { return Err(e); } }
			if let Some(ref qry_ref_value) = self.qry_ref { if let Err(e) = qry_ref_value.validate() { return Err(e); } }
			if let Some(ref rpt_id_value) = self.rpt_id { if let Err(e) = rpt_id_value.validate() { return Err(e); } }
			if let Some(ref rpt_dt_tm_value) = self.rpt_dt_tm { if let Err(e) = rpt_dt_tm_value.validate() { return Err(e); } }
			if let Some(ref rpt_prd_value) = self.rpt_prd { if let Err(e) = rpt_prd_value.validate() { return Err(e); } }
			if let Some(ref frqcy_value) = self.frqcy { if let Err(e) = frqcy_value.validate() { return Err(e); } }
			if let Err(e) = self.upd_tp.validate() { return Err(e); }
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
	
	
	// Linkages57 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Linkages57 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrcgPos", skip_serializing_if = "Option::is_none") )]
		pub prcg_pos: Option<ProcessingPosition7Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgNb", skip_serializing_if = "Option::is_none") )]
		pub msg_nb: Option<DocumentNumber5Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ref") )]
		pub ref_attr: References34Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RefOwnr", skip_serializing_if = "Option::is_none") )]
		pub ref_ownr: Option<PartyIdentification127Choice>,
	}
	
	impl Linkages57 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref prcg_pos_value) = self.prcg_pos { if let Err(e) = prcg_pos_value.validate() { return Err(e); } }
			if let Some(ref msg_nb_value) = self.msg_nb { if let Err(e) = msg_nb_value.validate() { return Err(e); } }
			if let Err(e) = self.ref_attr.validate() { return Err(e); }
			if let Some(ref ref_ownr_value) = self.ref_ownr { if let Err(e) = ref_ownr_value.validate() { return Err(e); } }
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
	
	
	// Max210Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max210Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max210_text: String,
	}
	
	impl Max210Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max210_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max210_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max210_text.chars().count() > 210 {
				return Err(ValidationError::new(1002, "max210_text exceeds the maximum length of 210".to_string()));
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
	
	
	// Max5NumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// NoReasonCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// Number3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Number3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Shrt", skip_serializing_if = "Option::is_none") )]
		pub shrt: Option<Exact3NumericText>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Lng", skip_serializing_if = "Option::is_none") )]
		pub lng: Option<Exact5NumericText>,
	}
	
	impl Number3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref shrt_value) = self.shrt { if let Err(e) = shrt_value.validate() { return Err(e); } }
			if let Some(ref lng_value) = self.lng { if let Err(e) = lng_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Pagination1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Pagination1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PgNb") )]
		pub pg_nb: Max5NumericText,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LastPgInd") )]
		pub last_pg_ind: bool,
	}
	
	impl Pagination1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.pg_nb.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// PartyIdentification120Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PartyIdentification120Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
		pub any_bic: Option<AnyBICDec2014Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
		pub prtry_id: Option<GenericIdentification36>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
		pub nm_and_adr: Option<NameAndAddress5>,
	}
	
	impl PartyIdentification120Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
			if let Some(ref prtry_id_value) = self.prtry_id { if let Err(e) = prtry_id_value.validate() { return Err(e); } }
			if let Some(ref nm_and_adr_value) = self.nm_and_adr { if let Err(e) = nm_and_adr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PartyIdentification127Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PartyIdentification127Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
		pub any_bic: Option<AnyBICDec2014Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
		pub prtry_id: Option<GenericIdentification36>,
	}
	
	impl PartyIdentification127Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
			if let Some(ref prtry_id_value) = self.prtry_id { if let Err(e) = prtry_id_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PartyIdentification136 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PartyIdentification136 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: PartyIdentification120Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<LEIIdentifier>,
	}
	
	impl PartyIdentification136 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PendingReason10Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum PendingReason10Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "AWMO") )]
		CodeAWMO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ADEA") )]
		CodeADEA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CAIS") )]
		CodeCAIS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REFU") )]
		CodeREFU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AWSH") )]
		CodeAWSH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PHSE") )]
		CodePHSE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TAMM") )]
		CodeTAMM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DOCY") )]
		CodeDOCY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DOCC") )]
		CodeDOCC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BLOC") )]
		CodeBLOC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CHAS") )]
		CodeCHAS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NEWI") )]
		CodeNEWI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLAC") )]
		CodeCLAC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MUNO") )]
		CodeMUNO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GLOB") )]
		CodeGLOB,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PREA") )]
		CodePREA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PART") )]
		CodePART,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NMAS") )]
		CodeNMAS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NOFX") )]
		CodeNOFX,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CMON") )]
		CodeCMON,
		#[cfg_attr( feature = "derive_serde", serde(rename = "YCOL") )]
		CodeYCOL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "COLL") )]
		CodeCOLL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DEPO") )]
		CodeDEPO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FLIM") )]
		CodeFLIM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INCA") )]
		CodeINCA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LINK") )]
		CodeLINK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FUTU") )]
		CodeFUTU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LACK") )]
		CodeLACK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LALO") )]
		CodeLALO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MONY") )]
		CodeMONY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NCON") )]
		CodeNCON,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REFS") )]
		CodeREFS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SDUT") )]
		CodeSDUT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BATC") )]
		CodeBATC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CYCL") )]
		CodeCYCL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SBLO") )]
		CodeSBLO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CPEC") )]
		CodeCPEC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MINO") )]
		CodeMINO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IAAD") )]
		CodeIAAD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PHCK") )]
		CodePHCK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BENO") )]
		CodeBENO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BOTH") )]
		CodeBOTH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLHT") )]
		CodeCLHT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DENO") )]
		CodeDENO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DISA") )]
		CodeDISA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DKNY") )]
		CodeDKNY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FROZ") )]
		CodeFROZ,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LAAW") )]
		CodeLAAW,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LATE") )]
		CodeLATE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LIQU") )]
		CodeLIQU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRCY") )]
		CodePRCY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REGT") )]
		CodeREGT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SETS") )]
		CodeSETS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CERT") )]
		CodeCERT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRSY") )]
		CodePRSY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INBC") )]
		CodeINBC,
	}
	
	impl PendingReason10Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PendingReason14 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PendingReason14 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: PendingReason26Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_rsn_inf: Option<Max210Text>,
	}
	
	impl PendingReason14 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref addtl_rsn_inf_value) = self.addtl_rsn_inf { if let Err(e) = addtl_rsn_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PendingReason26Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PendingReason26Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<PendingReason10Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl PendingReason26Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PendingStatus36Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PendingStatus36Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
		pub no_spcfd_rsn: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<Vec<PendingReason14>>,
	}
	
	impl PendingStatus36Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref no_spcfd_rsn_value) = self.no_spcfd_rsn { if let Err(e) = no_spcfd_rsn_value.validate() { return Err(e); } }
			if let Some(ref rsn_vec) = self.rsn { for item in rsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// PendingStatusAndReason2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PendingStatusAndReason2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrcgSts", skip_serializing_if = "Option::is_none") )]
		pub prcg_sts: Option<Vec<ProcessingStatus66Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmSts", skip_serializing_if = "Option::is_none") )]
		pub sttlm_sts: Option<Vec<SettlementStatus16Choice>>,
	}
	
	impl PendingStatusAndReason2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref prcg_sts_vec) = self.prcg_sts { for item in prcg_sts_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref sttlm_sts_vec) = self.sttlm_sts { for item in sttlm_sts_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// Period2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// Period7Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Period7Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrDtTmToDtTm", skip_serializing_if = "Option::is_none") )]
		pub fr_dt_tm_to_dt_tm: Option<DateTimePeriod1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrDtToDt", skip_serializing_if = "Option::is_none") )]
		pub fr_dt_to_dt: Option<Period2>,
	}
	
	impl Period7Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref fr_dt_tm_to_dt_tm_value) = self.fr_dt_tm_to_dt_tm { if let Err(e) = fr_dt_tm_to_dt_tm_value.validate() { return Err(e); } }
			if let Some(ref fr_dt_to_dt_value) = self.fr_dt_to_dt { if let Err(e) = fr_dt_to_dt_value.validate() { return Err(e); } }
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
	
	
	// PostalAddress27 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PostalAddress27 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdrTp", skip_serializing_if = "Option::is_none") )]
		pub adr_tp: Option<AddressType3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CareOf", skip_serializing_if = "Option::is_none") )]
		pub care_of: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dept", skip_serializing_if = "Option::is_none") )]
		pub dept: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubDept", skip_serializing_if = "Option::is_none") )]
		pub sub_dept: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StrtNm", skip_serializing_if = "Option::is_none") )]
		pub strt_nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNb", skip_serializing_if = "Option::is_none") )]
		pub bldg_nb: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNm", skip_serializing_if = "Option::is_none") )]
		pub bldg_nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Flr", skip_serializing_if = "Option::is_none") )]
		pub flr: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnitNb", skip_serializing_if = "Option::is_none") )]
		pub unit_nb: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstBx", skip_serializing_if = "Option::is_none") )]
		pub pst_bx: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Room", skip_serializing_if = "Option::is_none") )]
		pub room: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstCd", skip_serializing_if = "Option::is_none") )]
		pub pst_cd: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TwnNm", skip_serializing_if = "Option::is_none") )]
		pub twn_nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TwnLctnNm", skip_serializing_if = "Option::is_none") )]
		pub twn_lctn_nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none") )]
		pub dstrct_nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none") )]
		pub ctry_sub_dvsn: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
		pub ctry: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdrLine", skip_serializing_if = "Option::is_none") )]
		pub adr_line: Option<Vec<Max70Text>>,
	}
	
	impl PostalAddress27 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref adr_tp_value) = self.adr_tp { if let Err(e) = adr_tp_value.validate() { return Err(e); } }
			if let Some(ref care_of_value) = self.care_of { if let Err(e) = care_of_value.validate() { return Err(e); } }
			if let Some(ref dept_value) = self.dept { if let Err(e) = dept_value.validate() { return Err(e); } }
			if let Some(ref sub_dept_value) = self.sub_dept { if let Err(e) = sub_dept_value.validate() { return Err(e); } }
			if let Some(ref strt_nm_value) = self.strt_nm { if let Err(e) = strt_nm_value.validate() { return Err(e); } }
			if let Some(ref bldg_nb_value) = self.bldg_nb { if let Err(e) = bldg_nb_value.validate() { return Err(e); } }
			if let Some(ref bldg_nm_value) = self.bldg_nm { if let Err(e) = bldg_nm_value.validate() { return Err(e); } }
			if let Some(ref flr_value) = self.flr { if let Err(e) = flr_value.validate() { return Err(e); } }
			if let Some(ref unit_nb_value) = self.unit_nb { if let Err(e) = unit_nb_value.validate() { return Err(e); } }
			if let Some(ref pst_bx_value) = self.pst_bx { if let Err(e) = pst_bx_value.validate() { return Err(e); } }
			if let Some(ref room_value) = self.room { if let Err(e) = room_value.validate() { return Err(e); } }
			if let Some(ref pst_cd_value) = self.pst_cd { if let Err(e) = pst_cd_value.validate() { return Err(e); } }
			if let Some(ref twn_nm_value) = self.twn_nm { if let Err(e) = twn_nm_value.validate() { return Err(e); } }
			if let Some(ref twn_lctn_nm_value) = self.twn_lctn_nm { if let Err(e) = twn_lctn_nm_value.validate() { return Err(e); } }
			if let Some(ref dstrct_nm_value) = self.dstrct_nm { if let Err(e) = dstrct_nm_value.validate() { return Err(e); } }
			if let Some(ref ctry_sub_dvsn_value) = self.ctry_sub_dvsn { if let Err(e) = ctry_sub_dvsn_value.validate() { return Err(e); } }
			if let Some(ref ctry_value) = self.ctry { if let Err(e) = ctry_value.validate() { return Err(e); } }
			if let Some(ref adr_line_vec) = self.adr_line { for item in adr_line_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// PriorityNumeric4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PriorityNumeric4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nmrc", skip_serializing_if = "Option::is_none") )]
		pub nmrc: Option<Exact4NumericText>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl PriorityNumeric4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref nmrc_value) = self.nmrc { if let Err(e) = nmrc_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ProcessingPosition3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum ProcessingPosition3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "AFTE") )]
		CodeAFTE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WITH") )]
		CodeWITH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BEFO") )]
		CodeBEFO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INFO") )]
		CodeINFO,
	}
	
	impl ProcessingPosition3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ProcessingPosition7Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ProcessingPosition7Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ProcessingPosition3Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl ProcessingPosition7Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ProcessingStatus66Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ProcessingStatus66Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AckdAccptd", skip_serializing_if = "Option::is_none") )]
		pub ackd_accptd: Option<AcknowledgedAcceptedStatus21Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rpr", skip_serializing_if = "Option::is_none") )]
		pub rpr: Option<RejectionOrRepairStatus38Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Canc", skip_serializing_if = "Option::is_none") )]
		pub canc: Option<CancellationStatus14Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<ProprietaryStatusAndReason6>,
	}
	
	impl ProcessingStatus66Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ackd_accptd_value) = self.ackd_accptd { if let Err(e) = ackd_accptd_value.validate() { return Err(e); } }
			if let Some(ref rpr_value) = self.rpr { if let Err(e) = rpr_value.validate() { return Err(e); } }
			if let Some(ref canc_value) = self.canc { if let Err(e) = canc_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ProprietaryReason4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ProprietaryReason4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<GenericIdentification30>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_rsn_inf: Option<Max210Text>,
	}
	
	impl ProprietaryReason4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref rsn_value) = self.rsn { if let Err(e) = rsn_value.validate() { return Err(e); } }
			if let Some(ref addtl_rsn_inf_value) = self.addtl_rsn_inf { if let Err(e) = addtl_rsn_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ProprietaryStatusAndReason6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ProprietaryStatusAndReason6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtrySts") )]
		pub prtry_sts: GenericIdentification30,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryRsn", skip_serializing_if = "Option::is_none") )]
		pub prtry_rsn: Option<Vec<ProprietaryReason4>>,
	}
	
	impl ProprietaryStatusAndReason6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.prtry_sts.validate() { return Err(e); }
			if let Some(ref prtry_rsn_vec) = self.prtry_rsn { for item in prtry_rsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// ProxyAccountIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ProxyAccountIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<ProxyAccountType1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max2048Text,
	}
	
	impl ProxyAccountIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Err(e) = self.id.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// ProxyAccountType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ProxyAccountType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalProxyAccountType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl ProxyAccountType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// References34Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct References34Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesSttlmTxId", skip_serializing_if = "Option::is_none") )]
		pub scties_sttlm_tx_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntraPosMvmntId", skip_serializing_if = "Option::is_none") )]
		pub intra_pos_mvmnt_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntraBalMvmntId", skip_serializing_if = "Option::is_none") )]
		pub intra_bal_mvmnt_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrTxId", skip_serializing_if = "Option::is_none") )]
		pub acct_svcr_tx_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MktInfrstrctrTxId", skip_serializing_if = "Option::is_none") )]
		pub mkt_infrstrctr_tx_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PoolId", skip_serializing_if = "Option::is_none") )]
		pub pool_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrTxId", skip_serializing_if = "Option::is_none") )]
		pub othr_tx_id: Option<Max35Text>,
	}
	
	impl References34Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref scties_sttlm_tx_id_value) = self.scties_sttlm_tx_id { if let Err(e) = scties_sttlm_tx_id_value.validate() { return Err(e); } }
			if let Some(ref intra_pos_mvmnt_id_value) = self.intra_pos_mvmnt_id { if let Err(e) = intra_pos_mvmnt_id_value.validate() { return Err(e); } }
			if let Some(ref intra_bal_mvmnt_id_value) = self.intra_bal_mvmnt_id { if let Err(e) = intra_bal_mvmnt_id_value.validate() { return Err(e); } }
			if let Some(ref acct_svcr_tx_id_value) = self.acct_svcr_tx_id { if let Err(e) = acct_svcr_tx_id_value.validate() { return Err(e); } }
			if let Some(ref mkt_infrstrctr_tx_id_value) = self.mkt_infrstrctr_tx_id { if let Err(e) = mkt_infrstrctr_tx_id_value.validate() { return Err(e); } }
			if let Some(ref pool_id_value) = self.pool_id { if let Err(e) = pool_id_value.validate() { return Err(e); } }
			if let Some(ref othr_tx_id_value) = self.othr_tx_id { if let Err(e) = othr_tx_id_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// RejectionAndRepairReason32Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct RejectionAndRepairReason32Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<RejectionReason33Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl RejectionAndRepairReason32Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// RejectionOrRepairReason32 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct RejectionOrRepairReason32 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<Vec<RejectionAndRepairReason32Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_rsn_inf: Option<Max210Text>,
	}
	
	impl RejectionOrRepairReason32 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_vec) = self.cd { for item in cd_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref addtl_rsn_inf_value) = self.addtl_rsn_inf { if let Err(e) = addtl_rsn_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// RejectionOrRepairStatus38Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct RejectionOrRepairStatus38Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none") )]
		pub no_spcfd_rsn: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<Vec<RejectionOrRepairReason32>>,
	}
	
	impl RejectionOrRepairStatus38Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref no_spcfd_rsn_value) = self.no_spcfd_rsn { if let Err(e) = no_spcfd_rsn_value.validate() { return Err(e); } }
			if let Some(ref rsn_vec) = self.rsn { for item in rsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// RejectionReason33Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum RejectionReason33Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CASH") )]
		CodeCASH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ADEA") )]
		CodeADEA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DMON") )]
		CodeDMON,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NCRR") )]
		CodeNCRR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LATE") )]
		CodeLATE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INVL") )]
		CodeINVL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INVB") )]
		CodeINVB,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INVN") )]
		CodeINVN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VALR") )]
		CodeVALR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MONY") )]
		CodeMONY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CAEV") )]
		CodeCAEV,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DDAT") )]
		CodeDDAT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REFE") )]
		CodeREFE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DQUA") )]
		CodeDQUA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DSEC") )]
		CodeDSEC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MINO") )]
		CodeMINO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MUNO") )]
		CodeMUNO,
	}
	
	impl RejectionReason33Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// SettlementStatus16Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SettlementStatus16Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pdg", skip_serializing_if = "Option::is_none") )]
		pub pdg: Option<PendingStatus36Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Flng", skip_serializing_if = "Option::is_none") )]
		pub flng: Option<FailingStatus9Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<ProprietaryStatusAndReason6>,
	}
	
	impl SettlementStatus16Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref pdg_value) = self.pdg { if let Err(e) = pdg_value.validate() { return Err(e); } }
			if let Some(ref flng_value) = self.flng { if let Err(e) = flng_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// StatementUpdateType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum StatementUpdateType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "COMP") )]
		CodeCOMP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DELT") )]
		CodeDELT,
	}
	
	impl StatementUpdateType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// SupplementaryData1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SupplementaryData1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none") )]
		pub plc_and_nm: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Envlp") )]
		pub envlp: SupplementaryDataEnvelope1,
	}
	
	impl SupplementaryData1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref plc_and_nm_value) = self.plc_and_nm { if let Err(e) = plc_and_nm_value.validate() { return Err(e); } }
			if let Err(e) = self.envlp.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// SupplementaryDataEnvelope1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SupplementaryDataEnvelope1 {
	}
	
	impl SupplementaryDataEnvelope1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// SystemPartyIdentification8 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SystemPartyIdentification8 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: PartyIdentification136,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RspnsblPtyId", skip_serializing_if = "Option::is_none") )]
		pub rspnsbl_pty_id: Option<PartyIdentification136>,
	}
	
	impl SystemPartyIdentification8 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref rspnsbl_pty_id_value) = self.rspnsbl_pty_id { if let Err(e) = rspnsbl_pty_id_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// UpdateType15Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct UpdateType15Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<StatementUpdateType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification30>,
	}
	
	impl UpdateType15Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
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