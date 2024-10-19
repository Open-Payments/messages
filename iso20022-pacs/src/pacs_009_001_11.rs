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

use serde::{Deserialize, Serialize};
use regex::Regex;
use crate::validationerror::*;
// AccountIdentification4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountIdentification4Choice {
	#[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
	pub iban: Option<IBAN2007Identifier>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalAccountIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl AccountSchemeName1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveCurrencyAndAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveOrHistoricCurrencyAndAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AddressType2Code {
	#[default]
	#[serde(rename = "ADDR")]
	CodeADDR,
	#[serde(rename = "PBOX")]
	CodePBOX,
	#[serde(rename = "HOME")]
	CodeHOME,
	#[serde(rename = "BIZZ")]
	CodeBIZZ,
	#[serde(rename = "MLTO")]
	CodeMLTO,
	#[serde(rename = "DLVY")]
	CodeDLVY,
}

impl AddressType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AddressType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<AddressType2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BICFIDec2014Identifier {
	#[serde(rename = "$value")]
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


// BatchBookingIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BatchBookingIndicator {
	#[serde(rename = "$value")]
	pub batch_booking_indicator: bool,
}

impl BatchBookingIndicator {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// BranchAndFinancialInstitutionIdentification8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BranchAndFinancialInstitutionIdentification8 {
	#[serde(rename = "FinInstnId")]
	pub fin_instn_id: FinancialInstitutionIdentification23,
	#[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BranchData5 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max35Text>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
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


// CashAccount40 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccount40 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<AccountIdentification4Choice>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<CashAccountType2Choice>,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max70Text>,
	#[serde(rename = "Prxy", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalCashAccountType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CategoryPurpose1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalCategoryPurpose1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl CategoryPurpose1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ClearingChannel2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ClearingChannel2Code {
	#[default]
	#[serde(rename = "RTGS")]
	CodeRTGS,
	#[serde(rename = "RTNS")]
	CodeRTNS,
	#[serde(rename = "MPNS")]
	CodeMPNS,
	#[serde(rename = "BOOK")]
	CodeBOOK,
}

impl ClearingChannel2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ClearingSystemIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemIdentification2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalClearingSystemIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl ClearingSystemIdentification2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ClearingSystemIdentification3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemIdentification3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalCashClearingSystem1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl ClearingSystemIdentification3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ClearingSystemMemberIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemMemberIdentification2 {
	#[serde(rename = "ClrSysId", skip_serializing_if = "Option::is_none")]
	pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
	#[serde(rename = "MmbId")]
	pub mmb_id: Max35Text,
}

impl ClearingSystemMemberIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref clr_sys_id_value) = self.clr_sys_id { if let Err(e) = clr_sys_id_value.validate() { return Err(e); } }
		if let Err(e) = self.mmb_id.validate() { return Err(e); }
		Ok(())
	}
}


// Contact13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Contact13 {
	#[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
	pub nm_prfx: Option<NamePrefix2Code>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
	pub phne_nb: Option<PhoneNumber>,
	#[serde(rename = "MobNb", skip_serializing_if = "Option::is_none")]
	pub mob_nb: Option<PhoneNumber>,
	#[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
	pub fax_nb: Option<PhoneNumber>,
	#[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
	pub url_adr: Option<Max2048Text>,
	#[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
	pub email_adr: Option<Max256Text>,
	#[serde(rename = "EmailPurp", skip_serializing_if = "Option::is_none")]
	pub email_purp: Option<Max35Text>,
	#[serde(rename = "JobTitl", skip_serializing_if = "Option::is_none")]
	pub job_titl: Option<Max35Text>,
	#[serde(rename = "Rspnsblty", skip_serializing_if = "Option::is_none")]
	pub rspnsblty: Option<Max35Text>,
	#[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
	pub dept: Option<Max70Text>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<OtherContact1>>,
	#[serde(rename = "PrefrdMtd", skip_serializing_if = "Option::is_none")]
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


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CountryCode {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CreditDebitCode {
	#[default]
	#[serde(rename = "CRDT")]
	CodeCRDT,
	#[serde(rename = "DBIT")]
	CodeDBIT,
}

impl CreditDebitCode {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CreditTransferTransaction62 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditTransferTransaction62 {
	#[serde(rename = "PmtId")]
	pub pmt_id: PaymentIdentification13,
	#[serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none")]
	pub pmt_tp_inf: Option<PaymentTypeInformation28>,
	#[serde(rename = "IntrBkSttlmAmt")]
	pub intr_bk_sttlm_amt: ActiveCurrencyAndAmount,
	#[serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
	pub intr_bk_sttlm_dt: Option<String>,
	#[serde(rename = "SttlmPrty", skip_serializing_if = "Option::is_none")]
	pub sttlm_prty: Option<Priority3Code>,
	#[serde(rename = "SttlmTmIndctn", skip_serializing_if = "Option::is_none")]
	pub sttlm_tm_indctn: Option<SettlementDateTimeIndication1>,
	#[serde(rename = "SttlmTmReq", skip_serializing_if = "Option::is_none")]
	pub sttlm_tm_req: Option<SettlementTimeRequest2>,
	#[serde(rename = "PrvsInstgAgt1", skip_serializing_if = "Option::is_none")]
	pub prvs_instg_agt1: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "PrvsInstgAgt1Acct", skip_serializing_if = "Option::is_none")]
	pub prvs_instg_agt1_acct: Option<CashAccount40>,
	#[serde(rename = "PrvsInstgAgt2", skip_serializing_if = "Option::is_none")]
	pub prvs_instg_agt2: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "PrvsInstgAgt2Acct", skip_serializing_if = "Option::is_none")]
	pub prvs_instg_agt2_acct: Option<CashAccount40>,
	#[serde(rename = "PrvsInstgAgt3", skip_serializing_if = "Option::is_none")]
	pub prvs_instg_agt3: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "PrvsInstgAgt3Acct", skip_serializing_if = "Option::is_none")]
	pub prvs_instg_agt3_acct: Option<CashAccount40>,
	#[serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none")]
	pub instg_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none")]
	pub instd_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "IntrmyAgt1Acct", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt1_acct: Option<CashAccount40>,
	#[serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "IntrmyAgt2Acct", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt2_acct: Option<CashAccount40>,
	#[serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "IntrmyAgt3Acct", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt3_acct: Option<CashAccount40>,
	#[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
	pub ultmt_dbtr: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "Dbtr")]
	pub dbtr: BranchAndFinancialInstitutionIdentification8,
	#[serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none")]
	pub dbtr_acct: Option<CashAccount40>,
	#[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none")]
	pub dbtr_agt_acct: Option<CashAccount40>,
	#[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none")]
	pub cdtr_agt_acct: Option<CashAccount40>,
	#[serde(rename = "Cdtr")]
	pub cdtr: BranchAndFinancialInstitutionIdentification8,
	#[serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none")]
	pub cdtr_acct: Option<CashAccount40>,
	#[serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none")]
	pub ultmt_cdtr: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "InstrForCdtrAgt", skip_serializing_if = "Option::is_none")]
	pub instr_for_cdtr_agt: Option<Vec<InstructionForCreditorAgent3>>,
	#[serde(rename = "InstrForNxtAgt", skip_serializing_if = "Option::is_none")]
	pub instr_for_nxt_agt: Option<Vec<InstructionForNextAgent1>>,
	#[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
	pub purp: Option<Purpose2Choice>,
	#[serde(rename = "RmtInf", skip_serializing_if = "Option::is_none")]
	pub rmt_inf: Option<RemittanceInformation2>,
	#[serde(rename = "UndrlygAllcn", skip_serializing_if = "Option::is_none")]
	pub undrlyg_allcn: Option<Vec<TransactionAllocation1>>,
	#[serde(rename = "UndrlygCstmrCdtTrf", skip_serializing_if = "Option::is_none")]
	pub undrlyg_cstmr_cdt_trf: Option<CreditTransferTransaction63>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CreditTransferTransaction62 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.pmt_id.validate() { return Err(e); }
		if let Some(ref pmt_tp_inf_value) = self.pmt_tp_inf { if let Err(e) = pmt_tp_inf_value.validate() { return Err(e); } }
		if let Err(e) = self.intr_bk_sttlm_amt.validate() { return Err(e); }
		if let Some(ref sttlm_prty_value) = self.sttlm_prty { if let Err(e) = sttlm_prty_value.validate() { return Err(e); } }
		if let Some(ref sttlm_tm_indctn_value) = self.sttlm_tm_indctn { if let Err(e) = sttlm_tm_indctn_value.validate() { return Err(e); } }
		if let Some(ref sttlm_tm_req_value) = self.sttlm_tm_req { if let Err(e) = sttlm_tm_req_value.validate() { return Err(e); } }
		if let Some(ref prvs_instg_agt1_value) = self.prvs_instg_agt1 { if let Err(e) = prvs_instg_agt1_value.validate() { return Err(e); } }
		if let Some(ref prvs_instg_agt1_acct_value) = self.prvs_instg_agt1_acct { if let Err(e) = prvs_instg_agt1_acct_value.validate() { return Err(e); } }
		if let Some(ref prvs_instg_agt2_value) = self.prvs_instg_agt2 { if let Err(e) = prvs_instg_agt2_value.validate() { return Err(e); } }
		if let Some(ref prvs_instg_agt2_acct_value) = self.prvs_instg_agt2_acct { if let Err(e) = prvs_instg_agt2_acct_value.validate() { return Err(e); } }
		if let Some(ref prvs_instg_agt3_value) = self.prvs_instg_agt3 { if let Err(e) = prvs_instg_agt3_value.validate() { return Err(e); } }
		if let Some(ref prvs_instg_agt3_acct_value) = self.prvs_instg_agt3_acct { if let Err(e) = prvs_instg_agt3_acct_value.validate() { return Err(e); } }
		if let Some(ref instg_agt_value) = self.instg_agt { if let Err(e) = instg_agt_value.validate() { return Err(e); } }
		if let Some(ref instd_agt_value) = self.instd_agt { if let Err(e) = instd_agt_value.validate() { return Err(e); } }
		if let Some(ref intrmy_agt1_value) = self.intrmy_agt1 { if let Err(e) = intrmy_agt1_value.validate() { return Err(e); } }
		if let Some(ref intrmy_agt1_acct_value) = self.intrmy_agt1_acct { if let Err(e) = intrmy_agt1_acct_value.validate() { return Err(e); } }
		if let Some(ref intrmy_agt2_value) = self.intrmy_agt2 { if let Err(e) = intrmy_agt2_value.validate() { return Err(e); } }
		if let Some(ref intrmy_agt2_acct_value) = self.intrmy_agt2_acct { if let Err(e) = intrmy_agt2_acct_value.validate() { return Err(e); } }
		if let Some(ref intrmy_agt3_value) = self.intrmy_agt3 { if let Err(e) = intrmy_agt3_value.validate() { return Err(e); } }
		if let Some(ref intrmy_agt3_acct_value) = self.intrmy_agt3_acct { if let Err(e) = intrmy_agt3_acct_value.validate() { return Err(e); } }
		if let Some(ref ultmt_dbtr_value) = self.ultmt_dbtr { if let Err(e) = ultmt_dbtr_value.validate() { return Err(e); } }
		if let Err(e) = self.dbtr.validate() { return Err(e); }
		if let Some(ref dbtr_acct_value) = self.dbtr_acct { if let Err(e) = dbtr_acct_value.validate() { return Err(e); } }
		if let Some(ref dbtr_agt_value) = self.dbtr_agt { if let Err(e) = dbtr_agt_value.validate() { return Err(e); } }
		if let Some(ref dbtr_agt_acct_value) = self.dbtr_agt_acct { if let Err(e) = dbtr_agt_acct_value.validate() { return Err(e); } }
		if let Some(ref cdtr_agt_value) = self.cdtr_agt { if let Err(e) = cdtr_agt_value.validate() { return Err(e); } }
		if let Some(ref cdtr_agt_acct_value) = self.cdtr_agt_acct { if let Err(e) = cdtr_agt_acct_value.validate() { return Err(e); } }
		if let Err(e) = self.cdtr.validate() { return Err(e); }
		if let Some(ref cdtr_acct_value) = self.cdtr_acct { if let Err(e) = cdtr_acct_value.validate() { return Err(e); } }
		if let Some(ref ultmt_cdtr_value) = self.ultmt_cdtr { if let Err(e) = ultmt_cdtr_value.validate() { return Err(e); } }
		if let Some(ref instr_for_cdtr_agt_vec) = self.instr_for_cdtr_agt { for item in instr_for_cdtr_agt_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref instr_for_nxt_agt_vec) = self.instr_for_nxt_agt { for item in instr_for_nxt_agt_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref purp_value) = self.purp { if let Err(e) = purp_value.validate() { return Err(e); } }
		if let Some(ref rmt_inf_value) = self.rmt_inf { if let Err(e) = rmt_inf_value.validate() { return Err(e); } }
		if let Some(ref undrlyg_allcn_vec) = self.undrlyg_allcn { for item in undrlyg_allcn_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref undrlyg_cstmr_cdt_trf_value) = self.undrlyg_cstmr_cdt_trf { if let Err(e) = undrlyg_cstmr_cdt_trf_value.validate() { return Err(e); } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// CreditTransferTransaction63 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditTransferTransaction63 {
	#[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
	pub ultmt_dbtr: Option<PartyIdentification272>,
	#[serde(rename = "InitgPty", skip_serializing_if = "Option::is_none")]
	pub initg_pty: Option<PartyIdentification272>,
	#[serde(rename = "Dbtr")]
	pub dbtr: PartyIdentification272,
	#[serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none")]
	pub dbtr_acct: Option<CashAccount40>,
	#[serde(rename = "DbtrAgt")]
	pub dbtr_agt: BranchAndFinancialInstitutionIdentification8,
	#[serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none")]
	pub dbtr_agt_acct: Option<CashAccount40>,
	#[serde(rename = "PrvsInstgAgt1", skip_serializing_if = "Option::is_none")]
	pub prvs_instg_agt1: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "PrvsInstgAgt1Acct", skip_serializing_if = "Option::is_none")]
	pub prvs_instg_agt1_acct: Option<CashAccount40>,
	#[serde(rename = "PrvsInstgAgt2", skip_serializing_if = "Option::is_none")]
	pub prvs_instg_agt2: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "PrvsInstgAgt2Acct", skip_serializing_if = "Option::is_none")]
	pub prvs_instg_agt2_acct: Option<CashAccount40>,
	#[serde(rename = "PrvsInstgAgt3", skip_serializing_if = "Option::is_none")]
	pub prvs_instg_agt3: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "PrvsInstgAgt3Acct", skip_serializing_if = "Option::is_none")]
	pub prvs_instg_agt3_acct: Option<CashAccount40>,
	#[serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "IntrmyAgt1Acct", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt1_acct: Option<CashAccount40>,
	#[serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "IntrmyAgt2Acct", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt2_acct: Option<CashAccount40>,
	#[serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "IntrmyAgt3Acct", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt3_acct: Option<CashAccount40>,
	#[serde(rename = "CdtrAgt")]
	pub cdtr_agt: BranchAndFinancialInstitutionIdentification8,
	#[serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none")]
	pub cdtr_agt_acct: Option<CashAccount40>,
	#[serde(rename = "Cdtr")]
	pub cdtr: PartyIdentification272,
	#[serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none")]
	pub cdtr_acct: Option<CashAccount40>,
	#[serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none")]
	pub ultmt_cdtr: Option<PartyIdentification272>,
	#[serde(rename = "InstrForCdtrAgt", skip_serializing_if = "Option::is_none")]
	pub instr_for_cdtr_agt: Option<Vec<InstructionForCreditorAgent3>>,
	#[serde(rename = "InstrForNxtAgt", skip_serializing_if = "Option::is_none")]
	pub instr_for_nxt_agt: Option<Vec<InstructionForNextAgent1>>,
	#[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
	pub tax: Option<TaxData1>,
	#[serde(rename = "RmtInf", skip_serializing_if = "Option::is_none")]
	pub rmt_inf: Option<RemittanceInformation22>,
	#[serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none")]
	pub instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl CreditTransferTransaction63 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref ultmt_dbtr_value) = self.ultmt_dbtr { if let Err(e) = ultmt_dbtr_value.validate() { return Err(e); } }
		if let Some(ref initg_pty_value) = self.initg_pty { if let Err(e) = initg_pty_value.validate() { return Err(e); } }
		if let Err(e) = self.dbtr.validate() { return Err(e); }
		if let Some(ref dbtr_acct_value) = self.dbtr_acct { if let Err(e) = dbtr_acct_value.validate() { return Err(e); } }
		if let Err(e) = self.dbtr_agt.validate() { return Err(e); }
		if let Some(ref dbtr_agt_acct_value) = self.dbtr_agt_acct { if let Err(e) = dbtr_agt_acct_value.validate() { return Err(e); } }
		if let Some(ref prvs_instg_agt1_value) = self.prvs_instg_agt1 { if let Err(e) = prvs_instg_agt1_value.validate() { return Err(e); } }
		if let Some(ref prvs_instg_agt1_acct_value) = self.prvs_instg_agt1_acct { if let Err(e) = prvs_instg_agt1_acct_value.validate() { return Err(e); } }
		if let Some(ref prvs_instg_agt2_value) = self.prvs_instg_agt2 { if let Err(e) = prvs_instg_agt2_value.validate() { return Err(e); } }
		if let Some(ref prvs_instg_agt2_acct_value) = self.prvs_instg_agt2_acct { if let Err(e) = prvs_instg_agt2_acct_value.validate() { return Err(e); } }
		if let Some(ref prvs_instg_agt3_value) = self.prvs_instg_agt3 { if let Err(e) = prvs_instg_agt3_value.validate() { return Err(e); } }
		if let Some(ref prvs_instg_agt3_acct_value) = self.prvs_instg_agt3_acct { if let Err(e) = prvs_instg_agt3_acct_value.validate() { return Err(e); } }
		if let Some(ref intrmy_agt1_value) = self.intrmy_agt1 { if let Err(e) = intrmy_agt1_value.validate() { return Err(e); } }
		if let Some(ref intrmy_agt1_acct_value) = self.intrmy_agt1_acct { if let Err(e) = intrmy_agt1_acct_value.validate() { return Err(e); } }
		if let Some(ref intrmy_agt2_value) = self.intrmy_agt2 { if let Err(e) = intrmy_agt2_value.validate() { return Err(e); } }
		if let Some(ref intrmy_agt2_acct_value) = self.intrmy_agt2_acct { if let Err(e) = intrmy_agt2_acct_value.validate() { return Err(e); } }
		if let Some(ref intrmy_agt3_value) = self.intrmy_agt3 { if let Err(e) = intrmy_agt3_value.validate() { return Err(e); } }
		if let Some(ref intrmy_agt3_acct_value) = self.intrmy_agt3_acct { if let Err(e) = intrmy_agt3_acct_value.validate() { return Err(e); } }
		if let Err(e) = self.cdtr_agt.validate() { return Err(e); }
		if let Some(ref cdtr_agt_acct_value) = self.cdtr_agt_acct { if let Err(e) = cdtr_agt_acct_value.validate() { return Err(e); } }
		if let Err(e) = self.cdtr.validate() { return Err(e); }
		if let Some(ref cdtr_acct_value) = self.cdtr_acct { if let Err(e) = cdtr_acct_value.validate() { return Err(e); } }
		if let Some(ref ultmt_cdtr_value) = self.ultmt_cdtr { if let Err(e) = ultmt_cdtr_value.validate() { return Err(e); } }
		if let Some(ref instr_for_cdtr_agt_vec) = self.instr_for_cdtr_agt { for item in instr_for_cdtr_agt_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref instr_for_nxt_agt_vec) = self.instr_for_nxt_agt { for item in instr_for_nxt_agt_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref tax_value) = self.tax { if let Err(e) = tax_value.validate() { return Err(e); } }
		if let Some(ref rmt_inf_value) = self.rmt_inf { if let Err(e) = rmt_inf_value.validate() { return Err(e); } }
		if let Some(ref instd_amt_value) = self.instd_amt { if let Err(e) = instd_amt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CreditorReferenceInformation3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceInformation3 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<CreditorReferenceType3>,
	#[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalCreditorReferenceType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceType3 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: CreditorReferenceType2Choice,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndPlaceOfBirth1 {
	#[serde(rename = "BirthDt")]
	pub birth_dt: String,
	#[serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none")]
	pub prvc_of_birth: Option<Max35Text>,
	#[serde(rename = "CityOfBirth")]
	pub city_of_birth: Max35Text,
	#[serde(rename = "CtryOfBirth")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndType1 {
	#[serde(rename = "Tp")]
	pub tp: DateType2Choice,
	#[serde(rename = "Dt")]
	pub dt: String,
}

impl DateAndType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.tp.validate() { return Err(e); }
		Ok(())
	}
}


// DatePeriod2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriod2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}

impl DatePeriod2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DateType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalDateType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl DateType2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DecimalNumber {
	#[serde(rename = "$value")]
	pub decimal_number: f64,
}

impl DecimalNumber {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DocumentAdjustment1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentAdjustment1 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Max4Text>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentAmount1 {
	#[serde(rename = "Tp")]
	pub tp: DocumentAmountType1Choice,
	#[serde(rename = "Amt")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentAmountType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalDocumentAmountType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentLineIdentification1 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<DocumentLineType1>,
	#[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
	pub nb: Option<Max35Text>,
	#[serde(rename = "RltdDt", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentLineInformation2 {
	#[serde(rename = "Id")]
	pub id: Vec<DocumentLineIdentification1>,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max2048Text>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentLineType1 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: DocumentLineType1Choice,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentLineType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalDocumentLineType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentType1 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: DocumentType2Choice,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalDocumentType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl DocumentType2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalAccountIdentification1Code {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalCashAccountType1Code {
	#[serde(rename = "$value")]
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


// ExternalCashClearingSystem1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalCashClearingSystem1Code {
	#[serde(rename = "$value")]
	pub external_cash_clearing_system1_code: String,
}

impl ExternalCashClearingSystem1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_cash_clearing_system1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_cash_clearing_system1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_cash_clearing_system1_code.chars().count() > 3 {
			return Err(ValidationError::new(1002, "external_cash_clearing_system1_code exceeds the maximum length of 3".to_string()));
		}
		Ok(())
	}
}


// ExternalCategoryPurpose1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalCategoryPurpose1Code {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalClearingSystemIdentification1Code {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalCreditorAgentInstruction1Code {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalCreditorReferenceType1Code {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalDateType1Code {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalDocumentAmountType1Code {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalDocumentLineType1Code {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalDocumentType1Code {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalFinancialInstitutionIdentification1Code {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalGarnishmentType1Code {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalLocalInstrument1Code {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalOrganisationIdentification1Code {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalPersonIdentification1Code {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalProxyAccountType1Code {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalPurpose1Code {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalServiceLevel1Code {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalFinancialInstitutionIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl FinancialIdentificationSchemeName1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// FinancialInstitutionCreditTransferV11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstitutionCreditTransferV11 {
	#[serde(rename = "GrpHdr")]
	pub grp_hdr: GroupHeader113,
	#[serde(rename = "CdtTrfTxInf")]
	pub cdt_trf_tx_inf: Vec<CreditTransferTransaction62>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl FinancialInstitutionCreditTransferV11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.grp_hdr.validate() { return Err(e); }
		for item in &self.cdt_trf_tx_inf { if let Err(e) = item.validate() { return Err(e); } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// FinancialInstitutionIdentification23 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstitutionIdentification23 {
	#[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
	pub bicfi: Option<BICFIDec2014Identifier>,
	#[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress27>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
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


// Garnishment4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Garnishment4 {
	#[serde(rename = "Tp")]
	pub tp: GarnishmentType1,
	#[serde(rename = "Grnshee", skip_serializing_if = "Option::is_none")]
	pub grnshee: Option<PartyIdentification272>,
	#[serde(rename = "GrnshmtAdmstr", skip_serializing_if = "Option::is_none")]
	pub grnshmt_admstr: Option<PartyIdentification272>,
	#[serde(rename = "RefNb", skip_serializing_if = "Option::is_none")]
	pub ref_nb: Option<Max140Text>,
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none")]
	pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "FmlyMdclInsrncInd", skip_serializing_if = "Option::is_none")]
	pub fmly_mdcl_insrnc_ind: Option<bool>,
	#[serde(rename = "MplyeeTermntnInd", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GarnishmentType1 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: GarnishmentType1Choice,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GarnishmentType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalGarnishmentType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl GarnishmentType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// GenericAccountIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericAccountIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max34Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<AccountSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericFinancialIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<FinancialIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification30 {
	#[serde(rename = "Id")]
	pub id: Exact4AlphaNumericText,
	#[serde(rename = "Issr")]
	pub issr: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
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


// GenericOrganisationIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericOrganisationIdentification3 {
	#[serde(rename = "Id")]
	pub id: Max256Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericPersonIdentification2 {
	#[serde(rename = "Id")]
	pub id: Max256Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
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


// GroupHeader113 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GroupHeader113 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
	#[serde(rename = "BtchBookg", skip_serializing_if = "Option::is_none")]
	pub btch_bookg: Option<bool>,
	#[serde(rename = "NbOfTxs")]
	pub nb_of_txs: Max15NumericText,
	#[serde(rename = "CtrlSum", skip_serializing_if = "Option::is_none")]
	pub ctrl_sum: Option<f64>,
	#[serde(rename = "TtlIntrBkSttlmAmt", skip_serializing_if = "Option::is_none")]
	pub ttl_intr_bk_sttlm_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
	pub intr_bk_sttlm_dt: Option<String>,
	#[serde(rename = "SttlmInf")]
	pub sttlm_inf: SettlementInstruction15,
	#[serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none")]
	pub pmt_tp_inf: Option<PaymentTypeInformation28>,
	#[serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none")]
	pub instg_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none")]
	pub instd_agt: Option<BranchAndFinancialInstitutionIdentification8>,
}

impl GroupHeader113 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.msg_id.validate() { return Err(e); }
		if let Err(e) = self.nb_of_txs.validate() { return Err(e); }
		if let Some(ref ttl_intr_bk_sttlm_amt_value) = self.ttl_intr_bk_sttlm_amt { if let Err(e) = ttl_intr_bk_sttlm_amt_value.validate() { return Err(e); } }
		if let Err(e) = self.sttlm_inf.validate() { return Err(e); }
		if let Some(ref pmt_tp_inf_value) = self.pmt_tp_inf { if let Err(e) = pmt_tp_inf_value.validate() { return Err(e); } }
		if let Some(ref instg_agt_value) = self.instg_agt { if let Err(e) = instg_agt_value.validate() { return Err(e); } }
		if let Some(ref instd_agt_value) = self.instd_agt { if let Err(e) = instd_agt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// IBAN2007Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct IBAN2007Identifier {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}

impl ISODate {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}

impl ISODateTime {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ISOTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISOTime {
	#[serde(rename = "$value")]
	pub iso_time: String,
}

impl ISOTime {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ISOYear ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISOYear {
	#[serde(rename = "$value")]
	pub iso_year: String,
}

impl ISOYear {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Instruction4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Instruction4Code {
	#[default]
	#[serde(rename = "PHOA")]
	CodePHOA,
	#[serde(rename = "TELA")]
	CodeTELA,
}

impl Instruction4Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InstructionForCreditorAgent3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InstructionForCreditorAgent3 {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalCreditorAgentInstruction1Code>,
	#[serde(rename = "InstrInf", skip_serializing_if = "Option::is_none")]
	pub instr_inf: Option<Max140Text>,
}

impl InstructionForCreditorAgent3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref instr_inf_value) = self.instr_inf { if let Err(e) = instr_inf_value.validate() { return Err(e); } }
		Ok(())
	}
}


// InstructionForNextAgent1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InstructionForNextAgent1 {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Instruction4Code>,
	#[serde(rename = "InstrInf", skip_serializing_if = "Option::is_none")]
	pub instr_inf: Option<Max140Text>,
}

impl InstructionForNextAgent1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref instr_inf_value) = self.instr_inf { if let Err(e) = instr_inf_value.validate() { return Err(e); } }
		Ok(())
	}
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
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


// LocalInstrument2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LocalInstrument2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalLocalInstrument1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl LocalInstrument2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Max128Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max128Text {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max140Text {
	#[serde(rename = "$value")]
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


// Max15NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max15NumericText {
	#[serde(rename = "$value")]
	pub max15_numeric_text: String,
}

impl Max15NumericText {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{1,15}").unwrap();
		if !pattern.is_match(&self.max15_numeric_text) {
			return Err(ValidationError::new(1005, "max15_numeric_text does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max16Text {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max2048Text {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max256Text {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max34Text {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max350Text {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max35Text {
	#[serde(rename = "$value")]
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


// Max4Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max4Text {
	#[serde(rename = "$value")]
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


// Max52Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max52Text {
	#[serde(rename = "$value")]
	pub max52_text: String,
}

impl Max52Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max52_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max52_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max52_text.chars().count() > 52 {
			return Err(ValidationError::new(1002, "max52_text exceeds the maximum length of 52".to_string()));
		}
		Ok(())
	}
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max70Text {
	#[serde(rename = "$value")]
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


// NamePrefix2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NamePrefix2Code {
	#[default]
	#[serde(rename = "DOCT")]
	CodeDOCT,
	#[serde(rename = "MADM")]
	CodeMADM,
	#[serde(rename = "MISS")]
	CodeMISS,
	#[serde(rename = "MIST")]
	CodeMIST,
	#[serde(rename = "MIKS")]
	CodeMIKS,
}

impl NamePrefix2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Number {
	#[serde(rename = "$value")]
	pub number: f64,
}

impl Number {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OrganisationIdentification39 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification39 {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalOrganisationIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl OrganisationIdentificationSchemeName1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// OtherContact1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherContact1 {
	#[serde(rename = "ChanlTp")]
	pub chanl_tp: Max4Text,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max128Text>,
}

impl OtherContact1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.chanl_tp.validate() { return Err(e); }
		if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Party52Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party52Choice {
	#[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
	pub org_id: Option<OrganisationIdentification39>,
	#[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification272 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress27>,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Party52Choice>,
	#[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
	pub ctry_of_res: Option<CountryCode>,
	#[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
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


// PaymentIdentification13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentIdentification13 {
	#[serde(rename = "InstrId", skip_serializing_if = "Option::is_none")]
	pub instr_id: Option<Max35Text>,
	#[serde(rename = "EndToEndId")]
	pub end_to_end_id: Max35Text,
	#[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
	pub tx_id: Option<Max35Text>,
	#[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
	pub uetr: Option<UUIDv4Identifier>,
	#[serde(rename = "ClrSysRef", skip_serializing_if = "Option::is_none")]
	pub clr_sys_ref: Option<Max35Text>,
}

impl PaymentIdentification13 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref instr_id_value) = self.instr_id { if let Err(e) = instr_id_value.validate() { return Err(e); } }
		if let Err(e) = self.end_to_end_id.validate() { return Err(e); }
		if let Some(ref tx_id_value) = self.tx_id { if let Err(e) = tx_id_value.validate() { return Err(e); } }
		if let Some(ref uetr_value) = self.uetr { if let Err(e) = uetr_value.validate() { return Err(e); } }
		if let Some(ref clr_sys_ref_value) = self.clr_sys_ref { if let Err(e) = clr_sys_ref_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PaymentTypeInformation28 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentTypeInformation28 {
	#[serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none")]
	pub instr_prty: Option<Priority2Code>,
	#[serde(rename = "ClrChanl", skip_serializing_if = "Option::is_none")]
	pub clr_chanl: Option<ClearingChannel2Code>,
	#[serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none")]
	pub svc_lvl: Option<Vec<ServiceLevel8Choice>>,
	#[serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none")]
	pub lcl_instrm: Option<LocalInstrument2Choice>,
	#[serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none")]
	pub ctgy_purp: Option<CategoryPurpose1Choice>,
}

impl PaymentTypeInformation28 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref instr_prty_value) = self.instr_prty { if let Err(e) = instr_prty_value.validate() { return Err(e); } }
		if let Some(ref clr_chanl_value) = self.clr_chanl { if let Err(e) = clr_chanl_value.validate() { return Err(e); } }
		if let Some(ref svc_lvl_vec) = self.svc_lvl { for item in svc_lvl_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref lcl_instrm_value) = self.lcl_instrm { if let Err(e) = lcl_instrm_value.validate() { return Err(e); } }
		if let Some(ref ctgy_purp_value) = self.ctgy_purp { if let Err(e) = ctgy_purp_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PercentageRate {
	#[serde(rename = "$value")]
	pub percentage_rate: f64,
}

impl PercentageRate {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PersonIdentification18 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentification18 {
	#[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
	pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalPersonIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl PersonIdentificationSchemeName1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PhoneNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PhoneNumber {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress27 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<AddressType3Choice>,
	#[serde(rename = "CareOf", skip_serializing_if = "Option::is_none")]
	pub care_of: Option<Max140Text>,
	#[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
	pub dept: Option<Max70Text>,
	#[serde(rename = "SubDept", skip_serializing_if = "Option::is_none")]
	pub sub_dept: Option<Max70Text>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<Max140Text>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<Max16Text>,
	#[serde(rename = "BldgNm", skip_serializing_if = "Option::is_none")]
	pub bldg_nm: Option<Max140Text>,
	#[serde(rename = "Flr", skip_serializing_if = "Option::is_none")]
	pub flr: Option<Max70Text>,
	#[serde(rename = "UnitNb", skip_serializing_if = "Option::is_none")]
	pub unit_nb: Option<Max16Text>,
	#[serde(rename = "PstBx", skip_serializing_if = "Option::is_none")]
	pub pst_bx: Option<Max16Text>,
	#[serde(rename = "Room", skip_serializing_if = "Option::is_none")]
	pub room: Option<Max70Text>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<Max16Text>,
	#[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
	pub twn_nm: Option<Max140Text>,
	#[serde(rename = "TwnLctnNm", skip_serializing_if = "Option::is_none")]
	pub twn_lctn_nm: Option<Max140Text>,
	#[serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none")]
	pub dstrct_nm: Option<Max140Text>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<Max35Text>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PreferredContactMethod2Code {
	#[default]
	#[serde(rename = "MAIL")]
	CodeMAIL,
	#[serde(rename = "FAXX")]
	CodeFAXX,
	#[serde(rename = "LETT")]
	CodeLETT,
	#[serde(rename = "CELL")]
	CodeCELL,
	#[serde(rename = "ONLI")]
	CodeONLI,
	#[serde(rename = "PHON")]
	CodePHON,
}

impl PreferredContactMethod2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Priority2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Priority2Code {
	#[default]
	#[serde(rename = "HIGH")]
	CodeHIGH,
	#[serde(rename = "NORM")]
	CodeNORM,
}

impl Priority2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Priority3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Priority3Code {
	#[default]
	#[serde(rename = "URGT")]
	CodeURGT,
	#[serde(rename = "HIGH")]
	CodeHIGH,
	#[serde(rename = "NORM")]
	CodeNORM,
}

impl Priority3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ProxyAccountIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProxyAccountIdentification1 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<ProxyAccountType1Choice>,
	#[serde(rename = "Id")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProxyAccountType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalProxyAccountType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Purpose2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalPurpose1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl Purpose2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// References74Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct References74Choice {
	#[serde(rename = "SctiesSttlmTxId", skip_serializing_if = "Option::is_none")]
	pub scties_sttlm_tx_id: Option<Max35Text>,
	#[serde(rename = "IntraPosMvmntId", skip_serializing_if = "Option::is_none")]
	pub intra_pos_mvmnt_id: Option<Max35Text>,
	#[serde(rename = "IntraBalMvmntId", skip_serializing_if = "Option::is_none")]
	pub intra_bal_mvmnt_id: Option<Max35Text>,
	#[serde(rename = "AcctSvcrTxId", skip_serializing_if = "Option::is_none")]
	pub acct_svcr_tx_id: Option<Max35Text>,
	#[serde(rename = "MktInfrstrctrTxId", skip_serializing_if = "Option::is_none")]
	pub mkt_infrstrctr_tx_id: Option<Max35Text>,
	#[serde(rename = "CtrPtyMktInfrstrctrTxId", skip_serializing_if = "Option::is_none")]
	pub ctr_pty_mkt_infrstrctr_tx_id: Option<Max35Text>,
	#[serde(rename = "PoolId", skip_serializing_if = "Option::is_none")]
	pub pool_id: Option<Max35Text>,
	#[serde(rename = "CmonId", skip_serializing_if = "Option::is_none")]
	pub cmon_id: Option<Max35Text>,
	#[serde(rename = "TradId", skip_serializing_if = "Option::is_none")]
	pub trad_id: Option<Max52Text>,
	#[serde(rename = "OthrTxId", skip_serializing_if = "Option::is_none")]
	pub othr_tx_id: Option<Max35Text>,
}

impl References74Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref scties_sttlm_tx_id_value) = self.scties_sttlm_tx_id { if let Err(e) = scties_sttlm_tx_id_value.validate() { return Err(e); } }
		if let Some(ref intra_pos_mvmnt_id_value) = self.intra_pos_mvmnt_id { if let Err(e) = intra_pos_mvmnt_id_value.validate() { return Err(e); } }
		if let Some(ref intra_bal_mvmnt_id_value) = self.intra_bal_mvmnt_id { if let Err(e) = intra_bal_mvmnt_id_value.validate() { return Err(e); } }
		if let Some(ref acct_svcr_tx_id_value) = self.acct_svcr_tx_id { if let Err(e) = acct_svcr_tx_id_value.validate() { return Err(e); } }
		if let Some(ref mkt_infrstrctr_tx_id_value) = self.mkt_infrstrctr_tx_id { if let Err(e) = mkt_infrstrctr_tx_id_value.validate() { return Err(e); } }
		if let Some(ref ctr_pty_mkt_infrstrctr_tx_id_value) = self.ctr_pty_mkt_infrstrctr_tx_id { if let Err(e) = ctr_pty_mkt_infrstrctr_tx_id_value.validate() { return Err(e); } }
		if let Some(ref pool_id_value) = self.pool_id { if let Err(e) = pool_id_value.validate() { return Err(e); } }
		if let Some(ref cmon_id_value) = self.cmon_id { if let Err(e) = cmon_id_value.validate() { return Err(e); } }
		if let Some(ref trad_id_value) = self.trad_id { if let Err(e) = trad_id_value.validate() { return Err(e); } }
		if let Some(ref othr_tx_id_value) = self.othr_tx_id { if let Err(e) = othr_tx_id_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ReferredDocumentInformation8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReferredDocumentInformation8 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<DocumentType1>,
	#[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
	pub nb: Option<Max35Text>,
	#[serde(rename = "RltdDt", skip_serializing_if = "Option::is_none")]
	pub rltd_dt: Option<DateAndType1>,
	#[serde(rename = "LineDtls", skip_serializing_if = "Option::is_none")]
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


// RemittanceAmount4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceAmount4 {
	#[serde(rename = "RmtAmtAndTp", skip_serializing_if = "Option::is_none")]
	pub rmt_amt_and_tp: Option<Vec<DocumentAmount1>>,
	#[serde(rename = "AdjstmntAmtAndRsn", skip_serializing_if = "Option::is_none")]
	pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment1>>,
}

impl RemittanceAmount4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref rmt_amt_and_tp_vec) = self.rmt_amt_and_tp { for item in rmt_amt_and_tp_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref adjstmnt_amt_and_rsn_vec) = self.adjstmnt_amt_and_rsn { for item in adjstmnt_amt_and_rsn_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// RemittanceInformation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceInformation2 {
	#[serde(rename = "Ustrd", skip_serializing_if = "Option::is_none")]
	pub ustrd: Option<Vec<Max140Text>>,
}

impl RemittanceInformation2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref ustrd_vec) = self.ustrd { for item in ustrd_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// RemittanceInformation22 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceInformation22 {
	#[serde(rename = "Ustrd", skip_serializing_if = "Option::is_none")]
	pub ustrd: Option<Vec<Max140Text>>,
	#[serde(rename = "Strd", skip_serializing_if = "Option::is_none")]
	pub strd: Option<Vec<StructuredRemittanceInformation18>>,
}

impl RemittanceInformation22 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref ustrd_vec) = self.ustrd { for item in ustrd_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref strd_vec) = self.strd { for item in strd_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// ServiceLevel8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ServiceLevel8Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalServiceLevel1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl ServiceLevel8Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SettlementDateTimeIndication1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementDateTimeIndication1 {
	#[serde(rename = "DbtDtTm", skip_serializing_if = "Option::is_none")]
	pub dbt_dt_tm: Option<String>,
	#[serde(rename = "CdtDtTm", skip_serializing_if = "Option::is_none")]
	pub cdt_dt_tm: Option<String>,
}

impl SettlementDateTimeIndication1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SettlementInstruction15 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementInstruction15 {
	#[serde(rename = "SttlmMtd")]
	pub sttlm_mtd: SettlementMethod1Code,
	#[serde(rename = "SttlmAcct", skip_serializing_if = "Option::is_none")]
	pub sttlm_acct: Option<CashAccount40>,
	#[serde(rename = "ClrSys", skip_serializing_if = "Option::is_none")]
	pub clr_sys: Option<ClearingSystemIdentification3Choice>,
	#[serde(rename = "InstgRmbrsmntAgt", skip_serializing_if = "Option::is_none")]
	pub instg_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "InstgRmbrsmntAgtAcct", skip_serializing_if = "Option::is_none")]
	pub instg_rmbrsmnt_agt_acct: Option<CashAccount40>,
	#[serde(rename = "InstdRmbrsmntAgt", skip_serializing_if = "Option::is_none")]
	pub instd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "InstdRmbrsmntAgtAcct", skip_serializing_if = "Option::is_none")]
	pub instd_rmbrsmnt_agt_acct: Option<CashAccount40>,
	#[serde(rename = "ThrdRmbrsmntAgt", skip_serializing_if = "Option::is_none")]
	pub thrd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "ThrdRmbrsmntAgtAcct", skip_serializing_if = "Option::is_none")]
	pub thrd_rmbrsmnt_agt_acct: Option<CashAccount40>,
}

impl SettlementInstruction15 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.sttlm_mtd.validate() { return Err(e); }
		if let Some(ref sttlm_acct_value) = self.sttlm_acct { if let Err(e) = sttlm_acct_value.validate() { return Err(e); } }
		if let Some(ref clr_sys_value) = self.clr_sys { if let Err(e) = clr_sys_value.validate() { return Err(e); } }
		if let Some(ref instg_rmbrsmnt_agt_value) = self.instg_rmbrsmnt_agt { if let Err(e) = instg_rmbrsmnt_agt_value.validate() { return Err(e); } }
		if let Some(ref instg_rmbrsmnt_agt_acct_value) = self.instg_rmbrsmnt_agt_acct { if let Err(e) = instg_rmbrsmnt_agt_acct_value.validate() { return Err(e); } }
		if let Some(ref instd_rmbrsmnt_agt_value) = self.instd_rmbrsmnt_agt { if let Err(e) = instd_rmbrsmnt_agt_value.validate() { return Err(e); } }
		if let Some(ref instd_rmbrsmnt_agt_acct_value) = self.instd_rmbrsmnt_agt_acct { if let Err(e) = instd_rmbrsmnt_agt_acct_value.validate() { return Err(e); } }
		if let Some(ref thrd_rmbrsmnt_agt_value) = self.thrd_rmbrsmnt_agt { if let Err(e) = thrd_rmbrsmnt_agt_value.validate() { return Err(e); } }
		if let Some(ref thrd_rmbrsmnt_agt_acct_value) = self.thrd_rmbrsmnt_agt_acct { if let Err(e) = thrd_rmbrsmnt_agt_acct_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SettlementMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SettlementMethod1Code {
	#[default]
	#[serde(rename = "INDA")]
	CodeINDA,
	#[serde(rename = "INGA")]
	CodeINGA,
	#[serde(rename = "COVE")]
	CodeCOVE,
	#[serde(rename = "CLRG")]
	CodeCLRG,
}

impl SettlementMethod1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SettlementTimeRequest2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementTimeRequest2 {
	#[serde(rename = "CLSTm", skip_serializing_if = "Option::is_none")]
	pub cls_tm: Option<String>,
	#[serde(rename = "TillTm", skip_serializing_if = "Option::is_none")]
	pub till_tm: Option<String>,
	#[serde(rename = "FrTm", skip_serializing_if = "Option::is_none")]
	pub fr_tm: Option<String>,
	#[serde(rename = "RjctTm", skip_serializing_if = "Option::is_none")]
	pub rjct_tm: Option<String>,
}

impl SettlementTimeRequest2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// StructuredRemittanceInformation18 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StructuredRemittanceInformation18 {
	#[serde(rename = "RfrdDocInf", skip_serializing_if = "Option::is_none")]
	pub rfrd_doc_inf: Option<Vec<ReferredDocumentInformation8>>,
	#[serde(rename = "RfrdDocAmt", skip_serializing_if = "Option::is_none")]
	pub rfrd_doc_amt: Option<RemittanceAmount4>,
	#[serde(rename = "CdtrRefInf", skip_serializing_if = "Option::is_none")]
	pub cdtr_ref_inf: Option<CreditorReferenceInformation3>,
	#[serde(rename = "Invcr", skip_serializing_if = "Option::is_none")]
	pub invcr: Option<PartyIdentification272>,
	#[serde(rename = "Invcee", skip_serializing_if = "Option::is_none")]
	pub invcee: Option<PartyIdentification272>,
	#[serde(rename = "TaxRmt", skip_serializing_if = "Option::is_none")]
	pub tax_rmt: Option<TaxData1>,
	#[serde(rename = "GrnshmtRmt", skip_serializing_if = "Option::is_none")]
	pub grnshmt_rmt: Option<Garnishment4>,
	#[serde(rename = "AddtlRmtInf", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
	pub plc_and_nm: Option<Max350Text>,
	#[serde(rename = "Envlp")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TaxAmount3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxAmount3 {
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "TaxblBaseAmt", skip_serializing_if = "Option::is_none")]
	pub taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none")]
	pub ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Dtls", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxAuthorisation1 {
	#[serde(rename = "Titl", skip_serializing_if = "Option::is_none")]
	pub titl: Option<Max35Text>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxData1 {
	#[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
	pub cdtr: Option<TaxParty1>,
	#[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
	pub dbtr: Option<TaxParty2>,
	#[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
	pub ultmt_dbtr: Option<TaxParty2>,
	#[serde(rename = "AdmstnZone", skip_serializing_if = "Option::is_none")]
	pub admstn_zone: Option<Max35Text>,
	#[serde(rename = "RefNb", skip_serializing_if = "Option::is_none")]
	pub ref_nb: Option<Max140Text>,
	#[serde(rename = "Mtd", skip_serializing_if = "Option::is_none")]
	pub mtd: Option<Max35Text>,
	#[serde(rename = "TtlTaxblBaseAmt", skip_serializing_if = "Option::is_none")]
	pub ttl_taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TtlTaxAmt", skip_serializing_if = "Option::is_none")]
	pub ttl_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
	pub seq_nb: Option<f64>,
	#[serde(rename = "Rcrd", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxParty1 {
	#[serde(rename = "TaxId", skip_serializing_if = "Option::is_none")]
	pub tax_id: Option<Max35Text>,
	#[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
	pub regn_id: Option<Max35Text>,
	#[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxParty2 {
	#[serde(rename = "TaxId", skip_serializing_if = "Option::is_none")]
	pub tax_id: Option<Max35Text>,
	#[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
	pub regn_id: Option<Max35Text>,
	#[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
	pub tax_tp: Option<Max35Text>,
	#[serde(rename = "Authstn", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxPeriod3 {
	#[serde(rename = "Yr", skip_serializing_if = "Option::is_none")]
	pub yr: Option<String>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<TaxRecordPeriod1Code>,
	#[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
	pub fr_to_dt: Option<DatePeriod2>,
}

impl TaxPeriod3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Some(ref fr_to_dt_value) = self.fr_to_dt { if let Err(e) = fr_to_dt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// TaxRecord3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxRecord3 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<Max35Text>,
	#[serde(rename = "Ctgy", skip_serializing_if = "Option::is_none")]
	pub ctgy: Option<Max35Text>,
	#[serde(rename = "CtgyDtls", skip_serializing_if = "Option::is_none")]
	pub ctgy_dtls: Option<Max35Text>,
	#[serde(rename = "DbtrSts", skip_serializing_if = "Option::is_none")]
	pub dbtr_sts: Option<Max35Text>,
	#[serde(rename = "CertId", skip_serializing_if = "Option::is_none")]
	pub cert_id: Option<Max35Text>,
	#[serde(rename = "FrmsCd", skip_serializing_if = "Option::is_none")]
	pub frms_cd: Option<Max35Text>,
	#[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
	pub prd: Option<TaxPeriod3>,
	#[serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none")]
	pub tax_amt: Option<TaxAmount3>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxRecordDetails3 {
	#[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
	pub prd: Option<TaxPeriod3>,
	#[serde(rename = "Amt")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TaxRecordPeriod1Code {
	#[default]
	#[serde(rename = "MM01")]
	CodeMM01,
	#[serde(rename = "MM02")]
	CodeMM02,
	#[serde(rename = "MM03")]
	CodeMM03,
	#[serde(rename = "MM04")]
	CodeMM04,
	#[serde(rename = "MM05")]
	CodeMM05,
	#[serde(rename = "MM06")]
	CodeMM06,
	#[serde(rename = "MM07")]
	CodeMM07,
	#[serde(rename = "MM08")]
	CodeMM08,
	#[serde(rename = "MM09")]
	CodeMM09,
	#[serde(rename = "MM10")]
	CodeMM10,
	#[serde(rename = "MM11")]
	CodeMM11,
	#[serde(rename = "MM12")]
	CodeMM12,
	#[serde(rename = "QTR1")]
	CodeQTR1,
	#[serde(rename = "QTR2")]
	CodeQTR2,
	#[serde(rename = "QTR3")]
	CodeQTR3,
	#[serde(rename = "QTR4")]
	CodeQTR4,
	#[serde(rename = "HLF1")]
	CodeHLF1,
	#[serde(rename = "HLF2")]
	CodeHLF2,
}

impl TaxRecordPeriod1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TransactionAllocation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionAllocation1 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: CreditDebitCode,
	#[serde(rename = "Acct")]
	pub acct: CashAccount40,
	#[serde(rename = "Purp")]
	pub purp: Purpose2Choice,
	#[serde(rename = "Ref")]
	pub ref_attr: Max35Text,
	#[serde(rename = "RltdRefs", skip_serializing_if = "Option::is_none")]
	pub rltd_refs: Option<Vec<References74Choice>>,
}

impl TransactionAllocation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.amt.validate() { return Err(e); }
		if let Err(e) = self.cdt_dbt_ind.validate() { return Err(e); }
		if let Err(e) = self.acct.validate() { return Err(e); }
		if let Err(e) = self.purp.validate() { return Err(e); }
		if let Err(e) = self.ref_attr.validate() { return Err(e); }
		if let Some(ref rltd_refs_vec) = self.rltd_refs { for item in rltd_refs_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}

impl TrueFalseIndicator {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// UUIDv4Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UUIDv4Identifier {
	#[serde(rename = "$value")]
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
