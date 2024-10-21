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
	
	
	// AccountIdentification4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// AccountSwitchDetails1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AccountSwitchDetails1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnqRefNb") )]
		pub unq_ref_nb: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RtgUnqRefNb") )]
		pub rtg_unq_ref_nb: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SwtchRcvdDtTm", skip_serializing_if = "Option::is_none") )]
		pub swtch_rcvd_dt_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SwtchDt", skip_serializing_if = "Option::is_none") )]
		pub swtch_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SwtchTp") )]
		pub swtch_tp: SwitchType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SwtchSts", skip_serializing_if = "Option::is_none") )]
		pub swtch_sts: Option<SwitchStatus1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BalTrfWndw", skip_serializing_if = "Option::is_none") )]
		pub bal_trf_wndw: Option<BalanceTransferWindow1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rspn", skip_serializing_if = "Option::is_none") )]
		pub rspn: Option<Vec<ResponseDetails1>>,
	}
	
	impl AccountSwitchDetails1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.unq_ref_nb.validate() { return Err(e); }
			if let Err(e) = self.rtg_unq_ref_nb.validate() { return Err(e); }
			if let Err(e) = self.swtch_tp.validate() { return Err(e); }
			if let Some(ref swtch_sts_value) = self.swtch_sts { if let Err(e) = swtch_sts_value.validate() { return Err(e); } }
			if let Some(ref bal_trf_wndw_value) = self.bal_trf_wndw { if let Err(e) = bal_trf_wndw_value.validate() { return Err(e); } }
			if let Some(ref rspn_vec) = self.rspn { for item in rspn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// AccountSwitchInformationRequestV05 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AccountSwitchInformationRequestV05 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
		pub msg_id: MessageIdentification1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSwtchDtls") )]
		pub acct_swtch_dtls: AccountSwitchDetails1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NewAcct") )]
		pub new_acct: NewAccount4,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OdAcct") )]
		pub od_acct: CashAccount43,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BalTrf", skip_serializing_if = "Option::is_none") )]
		pub bal_trf: Option<Vec<BalanceTransfer5>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl AccountSwitchInformationRequestV05 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.msg_id.validate() { return Err(e); }
			if let Err(e) = self.acct_swtch_dtls.validate() { return Err(e); }
			if let Err(e) = self.new_acct.validate() { return Err(e); }
			if let Err(e) = self.od_acct.validate() { return Err(e); }
			if let Some(ref bal_trf_vec) = self.bal_trf { for item in bal_trf_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// ActiveCurrencyAndAmountSimpleType ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// AddressType3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// AnyBICDec2014Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// BalanceTransfer5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BalanceTransfer5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BalTrfRef", skip_serializing_if = "Option::is_none") )]
		pub bal_trf_ref: Option<BalanceTransferReference1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BalTrfMtd", skip_serializing_if = "Option::is_none") )]
		pub bal_trf_mtd: Option<SettlementMethod5Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BalTrfFndgLmt", skip_serializing_if = "Option::is_none") )]
		pub bal_trf_fndg_lmt: Option<BalanceTransferFundingLimit1>,
	}
	
	impl BalanceTransfer5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref bal_trf_ref_value) = self.bal_trf_ref { if let Err(e) = bal_trf_ref_value.validate() { return Err(e); } }
			if let Some(ref bal_trf_mtd_value) = self.bal_trf_mtd { if let Err(e) = bal_trf_mtd_value.validate() { return Err(e); } }
			if let Some(ref bal_trf_fndg_lmt_value) = self.bal_trf_fndg_lmt { if let Err(e) = bal_trf_fndg_lmt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BalanceTransferFundingLimit1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BalanceTransferFundingLimit1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CcyAmt") )]
		pub ccy_amt: ActiveCurrencyAndAmount,
	}
	
	impl BalanceTransferFundingLimit1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.ccy_amt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// BalanceTransferReference1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BalanceTransferReference1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BalTrfRef") )]
		pub bal_trf_ref: Max35Text,
	}
	
	impl BalanceTransferReference1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.bal_trf_ref.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// BalanceTransferWindow1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum BalanceTransferWindow1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "DAYH") )]
		CodeDAYH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EARL") )]
		CodeEARL,
	}
	
	impl BalanceTransferWindow1Code {
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// CashAccount40 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// CashAccount43 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CashAccount43 {
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
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ownr", skip_serializing_if = "Option::is_none") )]
		pub ownr: Option<PartyIdentification272>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Svcr", skip_serializing_if = "Option::is_none") )]
		pub svcr: Option<BranchAndFinancialInstitutionIdentification8>,
	}
	
	impl CashAccount43 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref ccy_value) = self.ccy { if let Err(e) = ccy_value.validate() { return Err(e); } }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref prxy_value) = self.prxy { if let Err(e) = prxy_value.validate() { return Err(e); } }
			if let Some(ref ownr_value) = self.ownr { if let Err(e) = ownr_value.validate() { return Err(e); } }
			if let Some(ref svcr_value) = self.svcr { if let Err(e) = svcr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CashAccountType2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// CategoryPurpose1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CategoryPurpose1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalCategoryPurpose1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl CategoryPurpose1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ChargeBearerType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Cheque19 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Cheque19 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ChqTp", skip_serializing_if = "Option::is_none") )]
		pub chq_tp: Option<ChequeType2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ChqNb", skip_serializing_if = "Option::is_none") )]
		pub chq_nb: Option<Max35Text>,
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
		pub frms_cd: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MemoFld", skip_serializing_if = "Option::is_none") )]
		pub memo_fld: Option<Vec<Max35Text>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RgnlClrZone", skip_serializing_if = "Option::is_none") )]
		pub rgnl_clr_zone: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtLctn", skip_serializing_if = "Option::is_none") )]
		pub prt_lctn: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sgntr", skip_serializing_if = "Option::is_none") )]
		pub sgntr: Option<Vec<Max70Text>>,
	}
	
	impl Cheque19 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref chq_tp_value) = self.chq_tp { if let Err(e) = chq_tp_value.validate() { return Err(e); } }
			if let Some(ref chq_nb_value) = self.chq_nb { if let Err(e) = chq_nb_value.validate() { return Err(e); } }
			if let Some(ref chq_fr_value) = self.chq_fr { if let Err(e) = chq_fr_value.validate() { return Err(e); } }
			if let Some(ref dlvry_mtd_value) = self.dlvry_mtd { if let Err(e) = dlvry_mtd_value.validate() { return Err(e); } }
			if let Some(ref dlvr_to_value) = self.dlvr_to { if let Err(e) = dlvr_to_value.validate() { return Err(e); } }
			if let Some(ref instr_prty_value) = self.instr_prty { if let Err(e) = instr_prty_value.validate() { return Err(e); } }
			if let Some(ref frms_cd_value) = self.frms_cd { if let Err(e) = frms_cd_value.validate() { return Err(e); } }
			if let Some(ref memo_fld_vec) = self.memo_fld { for item in memo_fld_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref rgnl_clr_zone_value) = self.rgnl_clr_zone { if let Err(e) = rgnl_clr_zone_value.validate() { return Err(e); } }
			if let Some(ref prt_lctn_value) = self.prt_lctn { if let Err(e) = prt_lctn_value.validate() { return Err(e); } }
			if let Some(ref sgntr_vec) = self.sgntr { for item in sgntr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// ChequeDelivery1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	pub struct ChequeDeliveryMethod1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ChequeDelivery1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl ChequeDeliveryMethod1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ChequeType2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// CitizenshipInformation1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CitizenshipInformation1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ntlty") )]
		pub ntlty: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MnrInd", skip_serializing_if = "Option::is_none") )]
		pub mnr_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt", skip_serializing_if = "Option::is_none") )]
		pub start_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EndDt", skip_serializing_if = "Option::is_none") )]
		pub end_dt: Option<String>,
	}
	
	impl CitizenshipInformation1 {
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// CommunicationAddress3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CommunicationAddress3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Email", skip_serializing_if = "Option::is_none") )]
		pub email: Option<Max256Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Phne", skip_serializing_if = "Option::is_none") )]
		pub phne: Option<PhoneNumber>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Mob", skip_serializing_if = "Option::is_none") )]
		pub mob: Option<PhoneNumber>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FaxNb", skip_serializing_if = "Option::is_none") )]
		pub fax_nb: Option<PhoneNumber>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TlxAdr", skip_serializing_if = "Option::is_none") )]
		pub tlx_adr: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "URLAdr", skip_serializing_if = "Option::is_none") )]
		pub url_adr: Option<Max256Text>,
	}
	
	impl CommunicationAddress3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref email_value) = self.email { if let Err(e) = email_value.validate() { return Err(e); } }
			if let Some(ref phne_value) = self.phne { if let Err(e) = phne_value.validate() { return Err(e); } }
			if let Some(ref mob_value) = self.mob { if let Err(e) = mob_value.validate() { return Err(e); } }
			if let Some(ref fax_nb_value) = self.fax_nb { if let Err(e) = fax_nb_value.validate() { return Err(e); } }
			if let Some(ref tlx_adr_value) = self.tlx_adr { if let Err(e) = tlx_adr_value.validate() { return Err(e); } }
			if let Some(ref url_adr_value) = self.url_adr { if let Err(e) = url_adr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Contact13 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Contact13 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none") )]
		pub nm_prfx: Option<NamePrefix2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PhneNb", skip_serializing_if = "Option::is_none") )]
		pub phne_nb: Option<PhoneNumber>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MobNb", skip_serializing_if = "Option::is_none") )]
		pub mob_nb: Option<PhoneNumber>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FaxNb", skip_serializing_if = "Option::is_none") )]
		pub fax_nb: Option<PhoneNumber>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "URLAdr", skip_serializing_if = "Option::is_none") )]
		pub url_adr: Option<Max2048Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none") )]
		pub email_adr: Option<Max256Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EmailPurp", skip_serializing_if = "Option::is_none") )]
		pub email_purp: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "JobTitl", skip_serializing_if = "Option::is_none") )]
		pub job_titl: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rspnsblty", skip_serializing_if = "Option::is_none") )]
		pub rspnsblty: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dept", skip_serializing_if = "Option::is_none") )]
		pub dept: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<Vec<OtherContact1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrefrdMtd", skip_serializing_if = "Option::is_none") )]
		pub prefrd_mtd: Option<PreferredContactMethod2Code>,
	}
	
	impl Contact13 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref nm_prfx_value) = self.nm_prfx { if let Err(e) = nm_prfx_value.validate() { return Err(e); } }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref phne_nb_value) = self.phne_nb { if let Err(e) = phne_nb_value.validate() { return Err(e); } }
			if let Some(ref mob_nb_value) = self.mob_nb { if let Err(e) = mob_nb_value.validate() { return Err(e); } }
			if let Some(ref fax_nb_value) = self.fax_nb { if let Err(e) = fax_nb_value.validate() { return Err(e); } }
			if let Some(ref url_adr_value) = self.url_adr { if let Err(e) = url_adr_value.validate() { return Err(e); } }
			if let Some(ref email_adr_value) = self.email_adr { if let Err(e) = email_adr_value.validate() { return Err(e); } }
			if let Some(ref email_purp_value) = self.email_purp { if let Err(e) = email_purp_value.validate() { return Err(e); } }
			if let Some(ref job_titl_value) = self.job_titl { if let Err(e) = job_titl_value.validate() { return Err(e); } }
			if let Some(ref rspnsblty_value) = self.rspnsblty { if let Err(e) = rspnsblty_value.validate() { return Err(e); } }
			if let Some(ref dept_value) = self.dept { if let Err(e) = dept_value.validate() { return Err(e); } }
			if let Some(ref othr_vec) = self.othr { for item in othr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref prefrd_mtd_value) = self.prefrd_mtd { if let Err(e) = prefrd_mtd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CountryAndResidentialStatusType1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CountryAndResidentialStatusType1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
		pub ctry: CountryCode,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ResdtlSts") )]
		pub resdtl_sts: ResidentialStatus1Code,
	}
	
	impl CountryAndResidentialStatusType1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.ctry.validate() { return Err(e); }
			if let Err(e) = self.resdtl_sts.validate() { return Err(e); }
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
	
	
	// CreditDebitCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// CreditTransferTransaction59 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CreditTransferTransaction59 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtId") )]
		pub pmt_id: PaymentIdentification6,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none") )]
		pub pmt_tp_inf: Option<PaymentTypeInformation26>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRateMrkr", skip_serializing_if = "Option::is_none") )]
		pub tax_rate_mrkr: Option<TaxRateMarker1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgBr", skip_serializing_if = "Option::is_none") )]
		pub chrg_br: Option<ChargeBearerType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ChqInstr", skip_serializing_if = "Option::is_none") )]
		pub chq_instr: Option<Cheque19>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Frqcy", skip_serializing_if = "Option::is_none") )]
		pub frqcy: Option<Frequency1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TrfInstr", skip_serializing_if = "Option::is_none") )]
		pub trf_instr: Option<TransferInstruction1>,
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
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr", skip_serializing_if = "Option::is_none") )]
		pub cdtr: Option<PartyIdentification272>,
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
		pub rltd_rmt_inf: Option<Vec<RemittanceLocation9>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RmtInf", skip_serializing_if = "Option::is_none") )]
		pub rmt_inf: Option<RemittanceInformation22>,
	}
	
	impl CreditTransferTransaction59 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.pmt_id.validate() { return Err(e); }
			if let Some(ref pmt_tp_inf_value) = self.pmt_tp_inf { if let Err(e) = pmt_tp_inf_value.validate() { return Err(e); } }
			if let Some(ref tax_rate_mrkr_value) = self.tax_rate_mrkr { if let Err(e) = tax_rate_mrkr_value.validate() { return Err(e); } }
			if let Err(e) = self.amt.validate() { return Err(e); }
			if let Some(ref chrg_br_value) = self.chrg_br { if let Err(e) = chrg_br_value.validate() { return Err(e); } }
			if let Some(ref chq_instr_value) = self.chq_instr { if let Err(e) = chq_instr_value.validate() { return Err(e); } }
			if let Some(ref frqcy_value) = self.frqcy { if let Err(e) = frqcy_value.validate() { return Err(e); } }
			if let Some(ref trf_instr_value) = self.trf_instr { if let Err(e) = trf_instr_value.validate() { return Err(e); } }
			if let Some(ref ultmt_dbtr_value) = self.ultmt_dbtr { if let Err(e) = ultmt_dbtr_value.validate() { return Err(e); } }
			if let Some(ref intrmy_agt1_value) = self.intrmy_agt1 { if let Err(e) = intrmy_agt1_value.validate() { return Err(e); } }
			if let Some(ref intrmy_agt2_value) = self.intrmy_agt2 { if let Err(e) = intrmy_agt2_value.validate() { return Err(e); } }
			if let Some(ref intrmy_agt3_value) = self.intrmy_agt3 { if let Err(e) = intrmy_agt3_value.validate() { return Err(e); } }
			if let Err(e) = self.cdtr_agt.validate() { return Err(e); }
			if let Some(ref cdtr_value) = self.cdtr { if let Err(e) = cdtr_value.validate() { return Err(e); } }
			if let Some(ref cdtr_acct_value) = self.cdtr_acct { if let Err(e) = cdtr_acct_value.validate() { return Err(e); } }
			if let Some(ref ultmt_cdtr_value) = self.ultmt_cdtr { if let Err(e) = ultmt_cdtr_value.validate() { return Err(e); } }
			if let Some(ref instr_for_cdtr_agt_vec) = self.instr_for_cdtr_agt { for item in instr_for_cdtr_agt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref purp_value) = self.purp { if let Err(e) = purp_value.validate() { return Err(e); } }
			if let Some(ref rgltry_rptg_vec) = self.rgltry_rptg { for item in rgltry_rptg_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref tax_value) = self.tax { if let Err(e) = tax_value.validate() { return Err(e); } }
			if let Some(ref rltd_rmt_inf_vec) = self.rltd_rmt_inf { for item in rltd_rmt_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref rmt_inf_value) = self.rmt_inf { if let Err(e) = rmt_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CreditorReferenceInformation3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CreditorReferenceInformation3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<CreditorReferenceType3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ref", skip_serializing_if = "Option::is_none") )]
		pub ref_attr: Option<Max35Text>,
	}
	
	impl CreditorReferenceInformation3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref ref_attr_value) = self.ref_attr { if let Err(e) = ref_attr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CreditorReferenceType2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CreditorReferenceType2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalCreditorReferenceType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl CreditorReferenceType2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CreditorReferenceType3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CreditorReferenceType3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdOrPrtry") )]
		pub cd_or_prtry: CreditorReferenceType2Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl CreditorReferenceType3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd_or_prtry.validate() { return Err(e); }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DateAndPlaceOfBirth1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DateAndPlaceOfBirth1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BirthDt") )]
		pub birth_dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none") )]
		pub prvc_of_birth: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CityOfBirth") )]
		pub city_of_birth: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfBirth") )]
		pub ctry_of_birth: CountryCode,
	}
	
	impl DateAndPlaceOfBirth1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref prvc_of_birth_value) = self.prvc_of_birth { if let Err(e) = prvc_of_birth_value.validate() { return Err(e); } }
			if let Err(e) = self.city_of_birth.validate() { return Err(e); }
			if let Err(e) = self.ctry_of_birth.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// DateAndType1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DateAndType1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: DateType2Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt") )]
		pub dt: String,
	}
	
	impl DateAndType1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// DatePeriod2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// DateType2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DateType2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalDateType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl DateType2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DocumentAdjustment1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DocumentAdjustment1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveOrHistoricCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none") )]
		pub cdt_dbt_ind: Option<CreditDebitCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<Max4Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<Max140Text>,
	}
	
	impl DocumentAdjustment1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.amt.validate() { return Err(e); }
			if let Some(ref cdt_dbt_ind_value) = self.cdt_dbt_ind { if let Err(e) = cdt_dbt_ind_value.validate() { return Err(e); } }
			if let Some(ref rsn_value) = self.rsn { if let Err(e) = rsn_value.validate() { return Err(e); } }
			if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DocumentAmount1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DocumentAmount1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: DocumentAmountType1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveOrHistoricCurrencyAndAmount,
	}
	
	impl DocumentAmount1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Err(e) = self.amt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// DocumentAmountType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DocumentAmountType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalDocumentAmountType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl DocumentAmountType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DocumentLineIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DocumentLineIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<DocumentLineType1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nb", skip_serializing_if = "Option::is_none") )]
		pub nb: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RltdDt", skip_serializing_if = "Option::is_none") )]
		pub rltd_dt: Option<String>,
	}
	
	impl DocumentLineIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref nb_value) = self.nb { if let Err(e) = nb_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DocumentLineInformation2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DocumentLineInformation2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Vec<DocumentLineIdentification1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<Max2048Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
		pub amt: Option<RemittanceAmount4>,
	}
	
	impl DocumentLineInformation2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			for item in &self.id { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
			if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DocumentLineType1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DocumentLineType1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdOrPrtry") )]
		pub cd_or_prtry: DocumentLineType1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl DocumentLineType1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd_or_prtry.validate() { return Err(e); }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DocumentLineType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DocumentLineType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalDocumentLineType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl DocumentLineType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DocumentType1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DocumentType1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdOrPrtry") )]
		pub cd_or_prtry: DocumentType2Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl DocumentType1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd_or_prtry.validate() { return Err(e); }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DocumentType2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DocumentType2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalDocumentType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl DocumentType2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// EndPoint1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct EndPoint1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfPmts", skip_serializing_if = "Option::is_none") )]
		pub nb_of_pmts: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LastPmtDt", skip_serializing_if = "Option::is_none") )]
		pub last_pmt_dt: Option<String>,
	}
	
	impl EndPoint1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref nb_of_pmts_value) = self.nb_of_pmts { if let Err(e) = nb_of_pmts_value.validate() { return Err(e); } }
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
	
	
	// ExternalAccountIdentification1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ExternalCashAccountType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ExternalCategoryPurpose1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalCategoryPurpose1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_category_purpose1_code: String,
	}
	
	impl ExternalCategoryPurpose1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_category_purpose1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_category_purpose1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_category_purpose1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_category_purpose1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalClearingSystemIdentification1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ExternalCreditorAgentInstruction1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalCreditorAgentInstruction1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_creditor_agent_instruction1_code: String,
	}
	
	impl ExternalCreditorAgentInstruction1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_creditor_agent_instruction1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_creditor_agent_instruction1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_creditor_agent_instruction1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_creditor_agent_instruction1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalCreditorReferenceType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalCreditorReferenceType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_creditor_reference_type1_code: String,
	}
	
	impl ExternalCreditorReferenceType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_creditor_reference_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_creditor_reference_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_creditor_reference_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_creditor_reference_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalDateType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalDateType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_date_type1_code: String,
	}
	
	impl ExternalDateType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_date_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_date_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_date_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_date_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalDocumentAmountType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalDocumentAmountType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_document_amount_type1_code: String,
	}
	
	impl ExternalDocumentAmountType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_document_amount_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_document_amount_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_document_amount_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_document_amount_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalDocumentLineType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalDocumentLineType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_document_line_type1_code: String,
	}
	
	impl ExternalDocumentLineType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_document_line_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_document_line_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_document_line_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_document_line_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalDocumentType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalDocumentType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_document_type1_code: String,
	}
	
	impl ExternalDocumentType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_document_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_document_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_document_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_document_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalFinancialInstitutionIdentification1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ExternalGarnishmentType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalGarnishmentType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_garnishment_type1_code: String,
	}
	
	impl ExternalGarnishmentType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_garnishment_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_garnishment_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_garnishment_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_garnishment_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalLocalInstrument1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalLocalInstrument1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_local_instrument1_code: String,
	}
	
	impl ExternalLocalInstrument1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_local_instrument1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_local_instrument1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_local_instrument1_code.chars().count() > 35 {
				return Err(ValidationError::new(1002, "external_local_instrument1_code exceeds the maximum length of 35".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalOrganisationIdentification1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalOrganisationIdentification1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_organisation_identification1_code: String,
	}
	
	impl ExternalOrganisationIdentification1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_organisation_identification1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_organisation_identification1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_organisation_identification1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_organisation_identification1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalPersonIdentification1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalPersonIdentification1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_person_identification1_code: String,
	}
	
	impl ExternalPersonIdentification1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_person_identification1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_person_identification1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_person_identification1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_person_identification1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalProxyAccountType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ExternalPurpose1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalPurpose1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_purpose1_code: String,
	}
	
	impl ExternalPurpose1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_purpose1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_purpose1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_purpose1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_purpose1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalServiceLevel1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalServiceLevel1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_service_level1_code: String,
	}
	
	impl ExternalServiceLevel1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_service_level1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_service_level1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_service_level1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_service_level1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// FinancialIdentificationSchemeName1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Frequency1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Frequency1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Seq", skip_serializing_if = "Option::is_none") )]
		pub seq: Option<Max3NumericText>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt") )]
		pub start_dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EndPtChc") )]
		pub end_pt_chc: EndPoint1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdFrqcyPttrn", skip_serializing_if = "Option::is_none") )]
		pub reqd_frqcy_pttrn: Option<Frequency37Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NonWorkgDayAdjstmnt", skip_serializing_if = "Option::is_none") )]
		pub non_workg_day_adjstmnt: Option<BusinessDayConvention1Code>,
	}
	
	impl Frequency1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref seq_value) = self.seq { if let Err(e) = seq_value.validate() { return Err(e); } }
			if let Err(e) = self.end_pt_chc.validate() { return Err(e); }
			if let Some(ref reqd_frqcy_pttrn_value) = self.reqd_frqcy_pttrn { if let Err(e) = reqd_frqcy_pttrn_value.validate() { return Err(e); } }
			if let Some(ref non_workg_day_adjstmnt_value) = self.non_workg_day_adjstmnt { if let Err(e) = non_workg_day_adjstmnt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Frequency10Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Frequency37Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Frequency37Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<Frequency10Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl Frequency37Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Garnishment4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Garnishment4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: GarnishmentType1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Grnshee", skip_serializing_if = "Option::is_none") )]
		pub grnshee: Option<PartyIdentification272>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GrnshmtAdmstr", skip_serializing_if = "Option::is_none") )]
		pub grnshmt_admstr: Option<PartyIdentification272>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RefNb", skip_serializing_if = "Option::is_none") )]
		pub ref_nb: Option<Max140Text>,
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
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Some(ref grnshee_value) = self.grnshee { if let Err(e) = grnshee_value.validate() { return Err(e); } }
			if let Some(ref grnshmt_admstr_value) = self.grnshmt_admstr { if let Err(e) = grnshmt_admstr_value.validate() { return Err(e); } }
			if let Some(ref ref_nb_value) = self.ref_nb { if let Err(e) = ref_nb_value.validate() { return Err(e); } }
			if let Some(ref rmtd_amt_value) = self.rmtd_amt { if let Err(e) = rmtd_amt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GarnishmentType1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GarnishmentType1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdOrPrtry") )]
		pub cd_or_prtry: GarnishmentType1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl GarnishmentType1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd_or_prtry.validate() { return Err(e); }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GarnishmentType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GarnishmentType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalGarnishmentType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl GarnishmentType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Gender1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum Gender1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "FEMA") )]
		CodeFEMA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MALE") )]
		CodeMALE,
	}
	
	impl Gender1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// GenericIdentification44 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericIdentification44 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: OtherIdentification1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IsseDt", skip_serializing_if = "Option::is_none") )]
		pub isse_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XpryDt", skip_serializing_if = "Option::is_none") )]
		pub xpry_dt: Option<String>,
	}
	
	impl GenericIdentification44 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Err(e) = self.tp.validate() { return Err(e); }
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
	
	
	// GenericOrganisationIdentification3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericOrganisationIdentification3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max256Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl GenericOrganisationIdentification3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GenericPersonIdentification2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericPersonIdentification2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max256Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl GenericPersonIdentification2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// IBAN2007Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ISOYear ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISOYear {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iso_year: String,
	}
	
	impl ISOYear {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// IndividualPerson44 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct IndividualPerson44 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CurNm") )]
		pub cur_nm: IndividualPersonNameLong2,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsNm", skip_serializing_if = "Option::is_none") )]
		pub prvs_nm: Option<Vec<IndividualPersonNameLong2>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Gndr", skip_serializing_if = "Option::is_none") )]
		pub gndr: Option<Gender1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Lang", skip_serializing_if = "Option::is_none") )]
		pub lang: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BirthDt", skip_serializing_if = "Option::is_none") )]
		pub birth_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfBirth", skip_serializing_if = "Option::is_none") )]
		pub ctry_of_birth: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none") )]
		pub prvc_of_birth: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CityOfBirth", skip_serializing_if = "Option::is_none") )]
		pub city_of_birth: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxtnCtry", skip_serializing_if = "Option::is_none") )]
		pub taxtn_ctry: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtryAndResdtlSts", skip_serializing_if = "Option::is_none") )]
		pub ctry_and_resdtl_sts: Option<CountryAndResidentialStatusType1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SclSctyNb", skip_serializing_if = "Option::is_none") )]
		pub scl_scty_nb: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
		pub pstl_adr: Option<Vec<PostalAddress27>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtznshInf", skip_serializing_if = "Option::is_none") )]
		pub ctznsh_inf: Option<Vec<CitizenshipInformation1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmryComAdr", skip_serializing_if = "Option::is_none") )]
		pub pmry_com_adr: Option<CommunicationAddress3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ScndryComAdr", skip_serializing_if = "Option::is_none") )]
		pub scndry_com_adr: Option<CommunicationAddress3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrId", skip_serializing_if = "Option::is_none") )]
		pub othr_id: Option<Vec<GenericIdentification44>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrDtls", skip_serializing_if = "Option::is_none") )]
		pub othr_dtls: Option<Vec<TransferInstruction1>>,
	}
	
	impl IndividualPerson44 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cur_nm.validate() { return Err(e); }
			if let Some(ref prvs_nm_vec) = self.prvs_nm { for item in prvs_nm_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref gndr_value) = self.gndr { if let Err(e) = gndr_value.validate() { return Err(e); } }
			if let Some(ref ctry_of_birth_value) = self.ctry_of_birth { if let Err(e) = ctry_of_birth_value.validate() { return Err(e); } }
			if let Some(ref prvc_of_birth_value) = self.prvc_of_birth { if let Err(e) = prvc_of_birth_value.validate() { return Err(e); } }
			if let Some(ref city_of_birth_value) = self.city_of_birth { if let Err(e) = city_of_birth_value.validate() { return Err(e); } }
			if let Some(ref taxtn_ctry_value) = self.taxtn_ctry { if let Err(e) = taxtn_ctry_value.validate() { return Err(e); } }
			if let Some(ref ctry_and_resdtl_sts_value) = self.ctry_and_resdtl_sts { if let Err(e) = ctry_and_resdtl_sts_value.validate() { return Err(e); } }
			if let Some(ref scl_scty_nb_value) = self.scl_scty_nb { if let Err(e) = scl_scty_nb_value.validate() { return Err(e); } }
			if let Some(ref pstl_adr_vec) = self.pstl_adr { for item in pstl_adr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref ctznsh_inf_vec) = self.ctznsh_inf { for item in ctznsh_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref pmry_com_adr_value) = self.pmry_com_adr { if let Err(e) = pmry_com_adr_value.validate() { return Err(e); } }
			if let Some(ref scndry_com_adr_value) = self.scndry_com_adr { if let Err(e) = scndry_com_adr_value.validate() { return Err(e); } }
			if let Some(ref othr_id_vec) = self.othr_id { for item in othr_id_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref othr_dtls_vec) = self.othr_dtls { for item in othr_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// IndividualPersonNameLong2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct IndividualPersonNameLong2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none") )]
		pub nm_prfx: Option<NamePrefix2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Srnm") )]
		pub srnm: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GvnNm", skip_serializing_if = "Option::is_none") )]
		pub gvn_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MddlNm", skip_serializing_if = "Option::is_none") )]
		pub mddl_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Initls", skip_serializing_if = "Option::is_none") )]
		pub initls: Option<Max6Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NmSfx", skip_serializing_if = "Option::is_none") )]
		pub nm_sfx: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt", skip_serializing_if = "Option::is_none") )]
		pub start_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EndDt", skip_serializing_if = "Option::is_none") )]
		pub end_dt: Option<String>,
	}
	
	impl IndividualPersonNameLong2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref nm_prfx_value) = self.nm_prfx { if let Err(e) = nm_prfx_value.validate() { return Err(e); } }
			if let Err(e) = self.srnm.validate() { return Err(e); }
			if let Some(ref gvn_nm_value) = self.gvn_nm { if let Err(e) = gvn_nm_value.validate() { return Err(e); } }
			if let Some(ref mddl_nm_value) = self.mddl_nm { if let Err(e) = mddl_nm_value.validate() { return Err(e); } }
			if let Some(ref initls_value) = self.initls { if let Err(e) = initls_value.validate() { return Err(e); } }
			if let Some(ref nm_sfx_value) = self.nm_sfx { if let Err(e) = nm_sfx_value.validate() { return Err(e); } }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// InstructionForCreditorAgent3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct InstructionForCreditorAgent3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalCreditorAgentInstruction1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstrInf", skip_serializing_if = "Option::is_none") )]
		pub instr_inf: Option<Max140Text>,
	}
	
	impl InstructionForCreditorAgent3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref instr_inf_value) = self.instr_inf { if let Err(e) = instr_inf_value.validate() { return Err(e); } }
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
	
	
	// LanguageCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct LanguageCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub language_code: String,
	}
	
	impl LanguageCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// LocalInstrument2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct LocalInstrument2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalLocalInstrument1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl LocalInstrument2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Max10Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max10Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max10_text: String,
	}
	
	impl Max10Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max10_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max10_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max10_text.chars().count() > 10 {
				return Err(ValidationError::new(1002, "max10_text exceeds the maximum length of 10".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max128Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max128Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max128_text: String,
	}
	
	impl Max128Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max128_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max128_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max128_text.chars().count() > 128 {
				return Err(ValidationError::new(1002, "max128_text exceeds the maximum length of 128".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max140Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max2048Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max3NumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max3NumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max3_numeric_text: String,
	}
	
	impl Max3NumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[0-9]{1,3}").unwrap();
			if !pattern.is_match(&self.max3_numeric_text) {
				return Err(ValidationError::new(1005, "max3_numeric_text does not match the required pattern".to_string()));
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
	
	
	// Max4Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max4Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max4_text: String,
	}
	
	impl Max4Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max4_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max4_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max4_text.chars().count() > 4 {
				return Err(ValidationError::new(1002, "max4_text exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max6Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max6Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max6_text: String,
	}
	
	impl Max6Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max6_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max6_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max6_text.chars().count() > 6 {
				return Err(ValidationError::new(1002, "max6_text exceeds the maximum length of 6".to_string()));
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
	
	
	// NameAndAddress18 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct NameAndAddress18 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
		pub nm: Max140Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Adr") )]
		pub adr: PostalAddress27,
	}
	
	impl NameAndAddress18 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.nm.validate() { return Err(e); }
			if let Err(e) = self.adr.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// NamePrefix2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// NationalityCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct NationalityCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub nationality_code: String,
	}
	
	impl NationalityCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// NewAccount4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct NewAccount4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Acct") )]
		pub acct: CashAccount43,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AcctPty") )]
		pub acct_pty: Vec<IndividualPerson44>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Org", skip_serializing_if = "Option::is_none") )]
		pub org: Option<Organisation43>,
	}
	
	impl NewAccount4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.acct.validate() { return Err(e); }
			for item in &self.acct_pty { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref org_value) = self.org { if let Err(e) = org_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Number ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Organisation43 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Organisation43 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FullLglNm") )]
		pub full_lgl_nm: Max350Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradgNm", skip_serializing_if = "Option::is_none") )]
		pub tradg_nm: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgLglSts", skip_serializing_if = "Option::is_none") )]
		pub org_lgl_sts: Option<OrganisationLegalStatus1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EstblishdDt", skip_serializing_if = "Option::is_none") )]
		pub estblishd_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RegnNb", skip_serializing_if = "Option::is_none") )]
		pub regn_nb: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RegnCtry", skip_serializing_if = "Option::is_none") )]
		pub regn_ctry: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RegnDt", skip_serializing_if = "Option::is_none") )]
		pub regn_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxtnIdNb", skip_serializing_if = "Option::is_none") )]
		pub taxtn_id_nb: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxtnCtry", skip_serializing_if = "Option::is_none") )]
		pub taxtn_ctry: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfOpr", skip_serializing_if = "Option::is_none") )]
		pub ctry_of_opr: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BrdRsltnInd", skip_serializing_if = "Option::is_none") )]
		pub brd_rsltn_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BizAdr", skip_serializing_if = "Option::is_none") )]
		pub biz_adr: Option<PostalAddress27>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OprlAdr", skip_serializing_if = "Option::is_none") )]
		pub oprl_adr: Option<PostalAddress27>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LglAdr", skip_serializing_if = "Option::is_none") )]
		pub lgl_adr: Option<PostalAddress27>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RprtvOffcr", skip_serializing_if = "Option::is_none") )]
		pub rprtv_offcr: Option<Vec<PartyIdentification272>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TrsrMgr", skip_serializing_if = "Option::is_none") )]
		pub trsr_mgr: Option<PartyIdentification272>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MainMndtHldr", skip_serializing_if = "Option::is_none") )]
		pub main_mndt_hldr: Option<Vec<PartyIdentification272>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sndr", skip_serializing_if = "Option::is_none") )]
		pub sndr: Option<Vec<PartyIdentification272>>,
	}
	
	impl Organisation43 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.full_lgl_nm.validate() { return Err(e); }
			if let Some(ref tradg_nm_value) = self.tradg_nm { if let Err(e) = tradg_nm_value.validate() { return Err(e); } }
			if let Some(ref org_lgl_sts_value) = self.org_lgl_sts { if let Err(e) = org_lgl_sts_value.validate() { return Err(e); } }
			if let Some(ref regn_nb_value) = self.regn_nb { if let Err(e) = regn_nb_value.validate() { return Err(e); } }
			if let Some(ref regn_ctry_value) = self.regn_ctry { if let Err(e) = regn_ctry_value.validate() { return Err(e); } }
			if let Some(ref taxtn_id_nb_value) = self.taxtn_id_nb { if let Err(e) = taxtn_id_nb_value.validate() { return Err(e); } }
			if let Some(ref taxtn_ctry_value) = self.taxtn_ctry { if let Err(e) = taxtn_ctry_value.validate() { return Err(e); } }
			if let Some(ref ctry_of_opr_value) = self.ctry_of_opr { if let Err(e) = ctry_of_opr_value.validate() { return Err(e); } }
			if let Some(ref biz_adr_value) = self.biz_adr { if let Err(e) = biz_adr_value.validate() { return Err(e); } }
			if let Some(ref oprl_adr_value) = self.oprl_adr { if let Err(e) = oprl_adr_value.validate() { return Err(e); } }
			if let Some(ref lgl_adr_value) = self.lgl_adr { if let Err(e) = lgl_adr_value.validate() { return Err(e); } }
			if let Some(ref rprtv_offcr_vec) = self.rprtv_offcr { for item in rprtv_offcr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref trsr_mgr_value) = self.trsr_mgr { if let Err(e) = trsr_mgr_value.validate() { return Err(e); } }
			if let Some(ref main_mndt_hldr_vec) = self.main_mndt_hldr { for item in main_mndt_hldr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref sndr_vec) = self.sndr { for item in sndr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// OrganisationIdentification39 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OrganisationIdentification39 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
		pub any_bic: Option<AnyBICDec2014Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<LEIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<Vec<GenericOrganisationIdentification3>>,
	}
	
	impl OrganisationIdentification39 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
			if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
			if let Some(ref othr_vec) = self.othr { for item in othr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// OrganisationIdentificationSchemeName1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OrganisationIdentificationSchemeName1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalOrganisationIdentification1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl OrganisationIdentificationSchemeName1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// OrganisationLegalStatus1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum OrganisationLegalStatus1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CIOC") )]
		CodeCIOC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CHAR") )]
		CodeCHAR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CICC") )]
		CodeCICC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GENP") )]
		CodeGENP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IAPS") )]
		CodeIAPS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LLPP") )]
		CodeLLPP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PCLG") )]
		CodePCLG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LIMP") )]
		CodeLIMP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PCLS") )]
		CodePCLS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PCLC") )]
		CodePCLC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SOLE") )]
		CodeSOLE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UNLC") )]
		CodeUNLC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UNLT") )]
		CodeUNLT,
	}
	
	impl OrganisationLegalStatus1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// OtherContact1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OtherContact1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ChanlTp") )]
		pub chanl_tp: Max4Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<Max128Text>,
	}
	
	impl OtherContact1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.chanl_tp.validate() { return Err(e); }
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// OtherIdentification1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OtherIdentification1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<PersonIdentificationType5Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification47>,
	}
	
	impl OtherIdentification1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Party52Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Party52Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgId", skip_serializing_if = "Option::is_none") )]
		pub org_id: Option<OrganisationIdentification39>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrvtId", skip_serializing_if = "Option::is_none") )]
		pub prvt_id: Option<PersonIdentification18>,
	}
	
	impl Party52Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref org_id_value) = self.org_id { if let Err(e) = org_id_value.validate() { return Err(e); } }
			if let Some(ref prvt_id_value) = self.prvt_id { if let Err(e) = prvt_id_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PartyIdentification272 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PartyIdentification272 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
		pub pstl_adr: Option<PostalAddress27>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<Party52Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none") )]
		pub ctry_of_res: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none") )]
		pub ctct_dtls: Option<Contact13>,
	}
	
	impl PartyIdentification272 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref pstl_adr_value) = self.pstl_adr { if let Err(e) = pstl_adr_value.validate() { return Err(e); } }
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			if let Some(ref ctry_of_res_value) = self.ctry_of_res { if let Err(e) = ctry_of_res_value.validate() { return Err(e); } }
			if let Some(ref ctct_dtls_value) = self.ctct_dtls { if let Err(e) = ctct_dtls_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaymentIdentification6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaymentIdentification6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstrId", skip_serializing_if = "Option::is_none") )]
		pub instr_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EndToEndId") )]
		pub end_to_end_id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UETR", skip_serializing_if = "Option::is_none") )]
		pub uetr: Option<UUIDv4Identifier>,
	}
	
	impl PaymentIdentification6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref instr_id_value) = self.instr_id { if let Err(e) = instr_id_value.validate() { return Err(e); } }
			if let Err(e) = self.end_to_end_id.validate() { return Err(e); }
			if let Some(ref uetr_value) = self.uetr { if let Err(e) = uetr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaymentTypeInformation26 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if let Some(ref instr_prty_value) = self.instr_prty { if let Err(e) = instr_prty_value.validate() { return Err(e); } }
			if let Some(ref svc_lvl_vec) = self.svc_lvl { for item in svc_lvl_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref lcl_instrm_value) = self.lcl_instrm { if let Err(e) = lcl_instrm_value.validate() { return Err(e); } }
			if let Some(ref ctgy_purp_value) = self.ctgy_purp { if let Err(e) = ctgy_purp_value.validate() { return Err(e); } }
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
	
	
	// PersonIdentification18 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PersonIdentification18 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none") )]
		pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<Vec<GenericPersonIdentification2>>,
	}
	
	impl PersonIdentification18 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref dt_and_plc_of_birth_value) = self.dt_and_plc_of_birth { if let Err(e) = dt_and_plc_of_birth_value.validate() { return Err(e); } }
			if let Some(ref othr_vec) = self.othr { for item in othr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// PersonIdentificationSchemeName1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PersonIdentificationSchemeName1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalPersonIdentification1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl PersonIdentificationSchemeName1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PersonIdentificationType5Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum PersonIdentificationType5Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "AREG") )]
		CodeAREG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CPFA") )]
		CodeCPFA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DRLC") )]
		CodeDRLC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EMID") )]
		CodeEMID,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IDCD") )]
		CodeIDCD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NRIN") )]
		CodeNRIN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PASS") )]
		CodePASS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "POCD") )]
		CodePOCD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SOCS") )]
		CodeSOCS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SRSA") )]
		CodeSRSA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GUNL") )]
		CodeGUNL,
	}
	
	impl PersonIdentificationType5Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PhoneNumber ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// PostalAddress27 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// PreferredContactMethod2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Purpose2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Purpose2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalPurpose1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl Purpose2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ReferredDocumentInformation8 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ReferredDocumentInformation8 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<DocumentType1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nb", skip_serializing_if = "Option::is_none") )]
		pub nb: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RltdDt", skip_serializing_if = "Option::is_none") )]
		pub rltd_dt: Option<DateAndType1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LineDtls", skip_serializing_if = "Option::is_none") )]
		pub line_dtls: Option<Vec<DocumentLineInformation2>>,
	}
	
	impl ReferredDocumentInformation8 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref nb_value) = self.nb { if let Err(e) = nb_value.validate() { return Err(e); } }
			if let Some(ref rltd_dt_value) = self.rltd_dt { if let Err(e) = rltd_dt_value.validate() { return Err(e); } }
			if let Some(ref line_dtls_vec) = self.line_dtls { for item in line_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// RegulatoryAuthority2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RegulatoryAuthority2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
		pub ctry: Option<CountryCode>,
	}
	
	impl RegulatoryAuthority2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref ctry_value) = self.ctry { if let Err(e) = ctry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// RegulatoryReporting3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if let Some(ref dbt_cdt_rptg_ind_value) = self.dbt_cdt_rptg_ind { if let Err(e) = dbt_cdt_rptg_ind_value.validate() { return Err(e); } }
			if let Some(ref authrty_value) = self.authrty { if let Err(e) = authrty_value.validate() { return Err(e); } }
			if let Some(ref dtls_vec) = self.dtls { for item in dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// RegulatoryReportingType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	pub struct RemittanceAmount4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RmtAmtAndTp", skip_serializing_if = "Option::is_none") )]
		pub rmt_amt_and_tp: Option<Vec<DocumentAmount1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdjstmntAmtAndRsn", skip_serializing_if = "Option::is_none") )]
		pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment1>>,
	}
	
	impl RemittanceAmount4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref rmt_amt_and_tp_vec) = self.rmt_amt_and_tp { for item in rmt_amt_and_tp_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref adjstmnt_amt_and_rsn_vec) = self.adjstmnt_amt_and_rsn { for item in adjstmnt_amt_and_rsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// RemittanceInformation22 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RemittanceInformation22 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ustrd", skip_serializing_if = "Option::is_none") )]
		pub ustrd: Option<Vec<Max140Text>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Strd", skip_serializing_if = "Option::is_none") )]
		pub strd: Option<Vec<StructuredRemittanceInformation18>>,
	}
	
	impl RemittanceInformation22 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ustrd_vec) = self.ustrd { for item in ustrd_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref strd_vec) = self.strd { for item in strd_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// RemittanceLocation9 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RemittanceLocation9 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RmtId", skip_serializing_if = "Option::is_none") )]
		pub rmt_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RmtLctnMtd", skip_serializing_if = "Option::is_none") )]
		pub rmt_lctn_mtd: Option<RemittanceLocationMethod2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RmtLctnElctrncAdr", skip_serializing_if = "Option::is_none") )]
		pub rmt_lctn_elctrnc_adr: Option<Max2048Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RmtLctnPstlAdr", skip_serializing_if = "Option::is_none") )]
		pub rmt_lctn_pstl_adr: Option<NameAndAddress18>,
	}
	
	impl RemittanceLocation9 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref rmt_id_value) = self.rmt_id { if let Err(e) = rmt_id_value.validate() { return Err(e); } }
			if let Some(ref rmt_lctn_mtd_value) = self.rmt_lctn_mtd { if let Err(e) = rmt_lctn_mtd_value.validate() { return Err(e); } }
			if let Some(ref rmt_lctn_elctrnc_adr_value) = self.rmt_lctn_elctrnc_adr { if let Err(e) = rmt_lctn_elctrnc_adr_value.validate() { return Err(e); } }
			if let Some(ref rmt_lctn_pstl_adr_value) = self.rmt_lctn_pstl_adr { if let Err(e) = rmt_lctn_pstl_adr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// RemittanceLocationMethod2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ResidentialStatus1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum ResidentialStatus1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "RESI") )]
		CodeRESI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRES") )]
		CodePRES,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NRES") )]
		CodeNRES,
	}
	
	impl ResidentialStatus1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ResponseDetails1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ResponseDetails1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RspnCd") )]
		pub rspn_cd: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlDtls", skip_serializing_if = "Option::is_none") )]
		pub addtl_dtls: Option<Max350Text>,
	}
	
	impl ResponseDetails1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rspn_cd.validate() { return Err(e); }
			if let Some(ref addtl_dtls_value) = self.addtl_dtls { if let Err(e) = addtl_dtls_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ServiceLevel8Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ServiceLevel8Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalServiceLevel1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl ServiceLevel8Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SettlementMethod5Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SettlementMethod5Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cdt", skip_serializing_if = "Option::is_none") )]
		pub cdt: Option<CreditTransferTransaction59>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dbt", skip_serializing_if = "Option::is_none") )]
		pub dbt: Option<CreditTransferTransaction59>,
	}
	
	impl SettlementMethod5Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cdt_value) = self.cdt { if let Err(e) = cdt_value.validate() { return Err(e); } }
			if let Some(ref dbt_value) = self.dbt { if let Err(e) = dbt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// StructuredRegulatoryReporting3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct StructuredRegulatoryReporting3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
		pub dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
		pub ctry: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<Max10Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
		pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Inf", skip_serializing_if = "Option::is_none") )]
		pub inf: Option<Vec<Max35Text>>,
	}
	
	impl StructuredRegulatoryReporting3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref ctry_value) = self.ctry { if let Err(e) = ctry_value.validate() { return Err(e); } }
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
			if let Some(ref inf_vec) = self.inf { for item in inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// StructuredRemittanceInformation18 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
		pub addtl_rmt_inf: Option<Vec<Max140Text>>,
	}
	
	impl StructuredRemittanceInformation18 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref rfrd_doc_inf_vec) = self.rfrd_doc_inf { for item in rfrd_doc_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref rfrd_doc_amt_value) = self.rfrd_doc_amt { if let Err(e) = rfrd_doc_amt_value.validate() { return Err(e); } }
			if let Some(ref cdtr_ref_inf_value) = self.cdtr_ref_inf { if let Err(e) = cdtr_ref_inf_value.validate() { return Err(e); } }
			if let Some(ref invcr_value) = self.invcr { if let Err(e) = invcr_value.validate() { return Err(e); } }
			if let Some(ref invcee_value) = self.invcee { if let Err(e) = invcee_value.validate() { return Err(e); } }
			if let Some(ref tax_rmt_value) = self.tax_rmt { if let Err(e) = tax_rmt_value.validate() { return Err(e); } }
			if let Some(ref grnshmt_rmt_value) = self.grnshmt_rmt { if let Err(e) = grnshmt_rmt_value.validate() { return Err(e); } }
			if let Some(ref addtl_rmt_inf_vec) = self.addtl_rmt_inf { for item in addtl_rmt_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
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
	
	
	// SwitchStatus1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum SwitchStatus1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ACPT") )]
		CodeACPT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BTRQ") )]
		CodeBTRQ,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BTRS") )]
		CodeBTRS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "COMP") )]
		CodeCOMP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REDT") )]
		CodeREDT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REDE") )]
		CodeREDE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REJT") )]
		CodeREJT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REQU") )]
		CodeREQU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TMTN") )]
		CodeTMTN,
	}
	
	impl SwitchStatus1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// SwitchType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum SwitchType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "FULL") )]
		CodeFULL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PART") )]
		CodePART,
	}
	
	impl SwitchType1Code {
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
			if let Some(ref taxbl_base_amt_value) = self.taxbl_base_amt { if let Err(e) = taxbl_base_amt_value.validate() { return Err(e); } }
			if let Some(ref ttl_amt_value) = self.ttl_amt { if let Err(e) = ttl_amt_value.validate() { return Err(e); } }
			if let Some(ref dtls_vec) = self.dtls { for item in dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TaxAuthorisation1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxAuthorisation1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Titl", skip_serializing_if = "Option::is_none") )]
		pub titl: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max140Text>,
	}
	
	impl TaxAuthorisation1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref titl_value) = self.titl { if let Err(e) = titl_value.validate() { return Err(e); } }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TaxData1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxData1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr", skip_serializing_if = "Option::is_none") )]
		pub cdtr: Option<TaxParty1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr", skip_serializing_if = "Option::is_none") )]
		pub dbtr: Option<TaxParty2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
		pub ultmt_dbtr: Option<TaxParty2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdmstnZone", skip_serializing_if = "Option::is_none") )]
		pub admstn_zone: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RefNb", skip_serializing_if = "Option::is_none") )]
		pub ref_nb: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Mtd", skip_serializing_if = "Option::is_none") )]
		pub mtd: Option<Max35Text>,
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
			if let Some(ref cdtr_value) = self.cdtr { if let Err(e) = cdtr_value.validate() { return Err(e); } }
			if let Some(ref dbtr_value) = self.dbtr { if let Err(e) = dbtr_value.validate() { return Err(e); } }
			if let Some(ref ultmt_dbtr_value) = self.ultmt_dbtr { if let Err(e) = ultmt_dbtr_value.validate() { return Err(e); } }
			if let Some(ref admstn_zone_value) = self.admstn_zone { if let Err(e) = admstn_zone_value.validate() { return Err(e); } }
			if let Some(ref ref_nb_value) = self.ref_nb { if let Err(e) = ref_nb_value.validate() { return Err(e); } }
			if let Some(ref mtd_value) = self.mtd { if let Err(e) = mtd_value.validate() { return Err(e); } }
			if let Some(ref ttl_taxbl_base_amt_value) = self.ttl_taxbl_base_amt { if let Err(e) = ttl_taxbl_base_amt_value.validate() { return Err(e); } }
			if let Some(ref ttl_tax_amt_value) = self.ttl_tax_amt { if let Err(e) = ttl_tax_amt_value.validate() { return Err(e); } }
			if let Some(ref rcrd_vec) = self.rcrd { for item in rcrd_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TaxParty1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxParty1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxId", skip_serializing_if = "Option::is_none") )]
		pub tax_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RegnId", skip_serializing_if = "Option::is_none") )]
		pub regn_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxTp", skip_serializing_if = "Option::is_none") )]
		pub tax_tp: Option<Max35Text>,
	}
	
	impl TaxParty1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tax_id_value) = self.tax_id { if let Err(e) = tax_id_value.validate() { return Err(e); } }
			if let Some(ref regn_id_value) = self.regn_id { if let Err(e) = regn_id_value.validate() { return Err(e); } }
			if let Some(ref tax_tp_value) = self.tax_tp { if let Err(e) = tax_tp_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TaxParty2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxParty2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxId", skip_serializing_if = "Option::is_none") )]
		pub tax_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RegnId", skip_serializing_if = "Option::is_none") )]
		pub regn_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxTp", skip_serializing_if = "Option::is_none") )]
		pub tax_tp: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Authstn", skip_serializing_if = "Option::is_none") )]
		pub authstn: Option<TaxAuthorisation1>,
	}
	
	impl TaxParty2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tax_id_value) = self.tax_id { if let Err(e) = tax_id_value.validate() { return Err(e); } }
			if let Some(ref regn_id_value) = self.regn_id { if let Err(e) = regn_id_value.validate() { return Err(e); } }
			if let Some(ref tax_tp_value) = self.tax_tp { if let Err(e) = tax_tp_value.validate() { return Err(e); } }
			if let Some(ref authstn_value) = self.authstn { if let Err(e) = authstn_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TaxPeriod3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref fr_to_dt_value) = self.fr_to_dt { if let Err(e) = fr_to_dt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TaxRateMarker1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum TaxRateMarker1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ALPR") )]
		CodeALPR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ALIT") )]
		CodeALIT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GRSS") )]
		CodeGRSS,
	}
	
	impl TaxRateMarker1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// TaxRecord3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxRecord3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctgy", skip_serializing_if = "Option::is_none") )]
		pub ctgy: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtgyDtls", skip_serializing_if = "Option::is_none") )]
		pub ctgy_dtls: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrSts", skip_serializing_if = "Option::is_none") )]
		pub dbtr_sts: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CertId", skip_serializing_if = "Option::is_none") )]
		pub cert_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrmsCd", skip_serializing_if = "Option::is_none") )]
		pub frms_cd: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prd", skip_serializing_if = "Option::is_none") )]
		pub prd: Option<TaxPeriod3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none") )]
		pub tax_amt: Option<TaxAmount3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
		pub addtl_inf: Option<Max140Text>,
	}
	
	impl TaxRecord3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref ctgy_value) = self.ctgy { if let Err(e) = ctgy_value.validate() { return Err(e); } }
			if let Some(ref ctgy_dtls_value) = self.ctgy_dtls { if let Err(e) = ctgy_dtls_value.validate() { return Err(e); } }
			if let Some(ref dbtr_sts_value) = self.dbtr_sts { if let Err(e) = dbtr_sts_value.validate() { return Err(e); } }
			if let Some(ref cert_id_value) = self.cert_id { if let Err(e) = cert_id_value.validate() { return Err(e); } }
			if let Some(ref frms_cd_value) = self.frms_cd { if let Err(e) = frms_cd_value.validate() { return Err(e); } }
			if let Some(ref prd_value) = self.prd { if let Err(e) = prd_value.validate() { return Err(e); } }
			if let Some(ref tax_amt_value) = self.tax_amt { if let Err(e) = tax_amt_value.validate() { return Err(e); } }
			if let Some(ref addtl_inf_value) = self.addtl_inf { if let Err(e) = addtl_inf_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TaxRecordDetails3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TaxRecordDetails3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prd", skip_serializing_if = "Option::is_none") )]
		pub prd: Option<TaxPeriod3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveOrHistoricCurrencyAndAmount,
	}
	
	impl TaxRecordDetails3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref prd_value) = self.prd { if let Err(e) = prd_value.validate() { return Err(e); } }
			if let Err(e) = self.amt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// TaxRecordPeriod1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// TransferInstruction1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TransferInstruction1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TrfInd", skip_serializing_if = "Option::is_none") )]
		pub trf_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max256Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StartDtTm", skip_serializing_if = "Option::is_none") )]
		pub start_dt_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt", skip_serializing_if = "Option::is_none") )]
		pub start_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<Max350Text>,
	}
	
	impl TransferInstruction1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TrueFalseIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct TrueFalseIndicator {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub true_false_indicator: bool,
	}
	
	impl TrueFalseIndicator {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// UUIDv4Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct UUIDv4Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub uui_dv4_identifier: String,
	}
	
	impl UUIDv4Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(&self.uui_dv4_identifier) {
				return Err(ValidationError::new(1005, "uui_dv4_identifier does not match the required pattern".to_string()));
			}
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