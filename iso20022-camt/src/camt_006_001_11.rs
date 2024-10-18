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


// AccountIdentification4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountIdentification4Choice {
	#[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
	pub iban: Option<IBAN2007Identifier>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<GenericAccountIdentification1>,
}

impl AccountIdentification4Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref iban_value) = self.iban { if !iban_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if self.active_currency_and_amount_simple_type < 0.000000 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.active_currency_code) {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.active_or_historic_currency_and_amount_simple_type < 0.000000 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.active_or_historic_currency_code) {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// Amount2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Amount2Choice {
	#[serde(rename = "AmtWthtCcy", skip_serializing_if = "Option::is_none")]
	pub amt_wtht_ccy: Option<f64>,
	#[serde(rename = "AmtWthCcy", skip_serializing_if = "Option::is_none")]
	pub amt_wth_ccy: Option<ActiveCurrencyAndAmount>,
}

impl Amount2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref amt_wth_ccy_value) = self.amt_wth_ccy { if !amt_wth_ccy_value.validate() { return false; } }
		return true
	}
}


// Amount3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Amount3Choice {
	#[serde(rename = "AmtWthCcy", skip_serializing_if = "Option::is_none")]
	pub amt_wth_ccy: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "AmtWthtCcy", skip_serializing_if = "Option::is_none")]
	pub amt_wtht_ccy: Option<f64>,
}

impl Amount3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref amt_wth_ccy_value) = self.amt_wth_ccy { if !amt_wth_ccy_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
		if !pattern.is_match(&self.any_bic_dec2014_identifier) {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
		if !pattern.is_match(&self.bicfi_dec2014_identifier) {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if !self.fin_instn_id.validate() { return false }
		if let Some(ref brnch_id_value) = self.brnch_id { if !brnch_id_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
		if let Some(ref lei_value) = self.lei { if !lei_value.validate() { return false; } }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref pstl_adr_value) = self.pstl_adr { if !pstl_adr_value.validate() { return false; } }
		return true
	}
}


// CancelledStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CancelledStatusReason1Code {
	#[default]
	#[serde(rename = "CANI")]
	CodeCANI,
	#[serde(rename = "CANS")]
	CodeCANS,
	#[serde(rename = "CSUB")]
	CodeCSUB,
}

impl CancelledStatusReason1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CashAccount43 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccount43 {
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
	#[serde(rename = "Ownr", skip_serializing_if = "Option::is_none")]
	pub ownr: Option<PartyIdentification272>,
	#[serde(rename = "Svcr", skip_serializing_if = "Option::is_none")]
	pub svcr: Option<BranchAndFinancialInstitutionIdentification8>,
}

impl CashAccount43 {
	pub fn validate(&self) -> bool {
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		if let Some(ref ccy_value) = self.ccy { if !ccy_value.validate() { return false; } }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref prxy_value) = self.prxy { if !prxy_value.validate() { return false; } }
		if let Some(ref ownr_value) = self.ownr { if !ownr_value.validate() { return false; } }
		if let Some(ref svcr_value) = self.svcr { if !svcr_value.validate() { return false; } }
		return true
	}
}


// CashAccountAndEntry5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountAndEntry5 {
	#[serde(rename = "Acct")]
	pub acct: CashAccount43,
	#[serde(rename = "Ntry", skip_serializing_if = "Option::is_none")]
	pub ntry: Option<CashEntry2>,
}

impl CashAccountAndEntry5 {
	pub fn validate(&self) -> bool {
		if !self.acct.validate() { return false }
		if let Some(ref ntry_value) = self.ntry { if !ntry_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// CashEntry2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashEntry2 {
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
	pub sts: Option<EntryStatus1Code>,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max35Text>,
	#[serde(rename = "StmtId", skip_serializing_if = "Option::is_none")]
	pub stmt_id: Option<Max35Text>,
	#[serde(rename = "AcctSvcrRef", skip_serializing_if = "Option::is_none")]
	pub acct_svcr_ref: Option<f64>,
	#[serde(rename = "AddtlNtryInf", skip_serializing_if = "Option::is_none")]
	pub addtl_ntry_inf: Option<Vec<Max140Text>>,
}

impl CashEntry2 {
	pub fn validate(&self) -> bool {
		if let Some(ref amt_value) = self.amt { if !amt_value.validate() { return false; } }
		if let Some(ref dt_value) = self.dt { if !dt_value.validate() { return false; } }
		if let Some(ref sts_value) = self.sts { if !sts_value.validate() { return false; } }
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
		if let Some(ref stmt_id_value) = self.stmt_id { if !stmt_id_value.validate() { return false; } }
		if let Some(ref addtl_ntry_inf_vec) = self.addtl_ntry_inf { for item in addtl_ntry_inf_vec { if !item.validate() { return false; } } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref clr_sys_id_value) = self.clr_sys_id { if !clr_sys_id_value.validate() { return false; } }
		if !self.mmb_id.validate() { return false }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref nm_prfx_value) = self.nm_prfx { if !nm_prfx_value.validate() { return false; } }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref phne_nb_value) = self.phne_nb { if !phne_nb_value.validate() { return false; } }
		if let Some(ref mob_nb_value) = self.mob_nb { if !mob_nb_value.validate() { return false; } }
		if let Some(ref fax_nb_value) = self.fax_nb { if !fax_nb_value.validate() { return false; } }
		if let Some(ref url_adr_value) = self.url_adr { if !url_adr_value.validate() { return false; } }
		if let Some(ref email_adr_value) = self.email_adr { if !email_adr_value.validate() { return false; } }
		if let Some(ref email_purp_value) = self.email_purp { if !email_purp_value.validate() { return false; } }
		if let Some(ref job_titl_value) = self.job_titl { if !job_titl_value.validate() { return false; } }
		if let Some(ref rspnsblty_value) = self.rspnsblty { if !rspnsblty_value.validate() { return false; } }
		if let Some(ref dept_value) = self.dept { if !dept_value.validate() { return false; } }
		if let Some(ref othr_vec) = self.othr { for item in othr_vec { if !item.validate() { return false; } } }
		if let Some(ref prefrd_mtd_value) = self.prefrd_mtd { if !prefrd_mtd_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.country_code) {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		return true
	}
}


// DateAndDateTime2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTime2Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<String>,
}

impl DateAndDateTime2Choice {
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref prvc_of_birth_value) = self.prvc_of_birth { if !prvc_of_birth_value.validate() { return false; } }
		if !self.city_of_birth.validate() { return false }
		if !self.ctry_of_birth.validate() { return false }
		return true
	}
}


// DateTimePeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod1 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: String,
}

impl DateTimePeriod1 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// DateTimePeriod1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod1Choice {
	#[serde(rename = "FrDtTm", skip_serializing_if = "Option::is_none")]
	pub fr_dt_tm: Option<String>,
	#[serde(rename = "ToDtTm", skip_serializing_if = "Option::is_none")]
	pub to_dt_tm: Option<String>,
	#[serde(rename = "DtTmRg", skip_serializing_if = "Option::is_none")]
	pub dt_tm_rg: Option<DateTimePeriod1>,
}

impl DateTimePeriod1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref dt_tm_rg_value) = self.dt_tm_rg { if !dt_tm_rg_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		return true
	}
}


// EntryStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EntryStatus1Code {
	#[default]
	#[serde(rename = "BOOK")]
	CodeBOOK,
	#[serde(rename = "PDNG")]
	CodePDNG,
	#[serde(rename = "FUTR")]
	CodeFUTR,
}

impl EntryStatus1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// EntryTypeIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EntryTypeIdentifier {
	#[serde(rename = "$value")]
	pub entry_type_identifier: String,
}

impl EntryTypeIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[BEOVW]{1,1}[0-9]{2,2}|DUM").unwrap();
		if !pattern.is_match(&self.entry_type_identifier) {
			return false
		}
		return true
	}
}


// ErrorHandling3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalSystemErrorHandling1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl ErrorHandling3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// ErrorHandling5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling5 {
	#[serde(rename = "Err")]
	pub err: ErrorHandling3Choice,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max140Text>,
}

impl ErrorHandling5 {
	pub fn validate(&self) -> bool {
		if !self.err.validate() { return false }
		if let Some(ref desc_value) = self.desc { if !desc_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
		if !pattern.is_match(&self.exact4_alpha_numeric_text) {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.external_account_identification1_code.chars().count() < 1 {
			return false
		}
		if self.external_account_identification1_code.chars().count() > 4 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.external_cash_account_type1_code.chars().count() < 1 {
			return false
		}
		if self.external_cash_account_type1_code.chars().count() > 4 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.external_clearing_system_identification1_code.chars().count() < 1 {
			return false
		}
		if self.external_clearing_system_identification1_code.chars().count() > 5 {
			return false
		}
		return true
	}
}


// ExternalEnquiryRequestType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalEnquiryRequestType1Code {
	#[serde(rename = "$value")]
	pub external_enquiry_request_type1_code: String,
}

impl ExternalEnquiryRequestType1Code {
	pub fn validate(&self) -> bool {
		if self.external_enquiry_request_type1_code.chars().count() < 1 {
			return false
		}
		if self.external_enquiry_request_type1_code.chars().count() > 4 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.external_financial_institution_identification1_code.chars().count() < 1 {
			return false
		}
		if self.external_financial_institution_identification1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// ExternalMarketInfrastructure1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalMarketInfrastructure1Code {
	#[serde(rename = "$value")]
	pub external_market_infrastructure1_code: String,
}

impl ExternalMarketInfrastructure1Code {
	pub fn validate(&self) -> bool {
		if self.external_market_infrastructure1_code.chars().count() < 1 {
			return false
		}
		if self.external_market_infrastructure1_code.chars().count() > 3 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.external_organisation_identification1_code.chars().count() < 1 {
			return false
		}
		if self.external_organisation_identification1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// ExternalPaymentControlRequestType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalPaymentControlRequestType1Code {
	#[serde(rename = "$value")]
	pub external_payment_control_request_type1_code: String,
}

impl ExternalPaymentControlRequestType1Code {
	pub fn validate(&self) -> bool {
		if self.external_payment_control_request_type1_code.chars().count() < 1 {
			return false
		}
		if self.external_payment_control_request_type1_code.chars().count() > 4 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.external_person_identification1_code.chars().count() < 1 {
			return false
		}
		if self.external_person_identification1_code.chars().count() > 4 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.external_proxy_account_type1_code.chars().count() < 1 {
			return false
		}
		if self.external_proxy_account_type1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// ExternalSystemErrorHandling1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalSystemErrorHandling1Code {
	#[serde(rename = "$value")]
	pub external_system_error_handling1_code: String,
}

impl ExternalSystemErrorHandling1Code {
	pub fn validate(&self) -> bool {
		if self.external_system_error_handling1_code.chars().count() < 1 {
			return false
		}
		if self.external_system_error_handling1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// FinalStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FinalStatus1Code {
	#[default]
	#[serde(rename = "STLD")]
	CodeSTLD,
	#[serde(rename = "RJTD")]
	CodeRJTD,
	#[serde(rename = "CAND")]
	CodeCAND,
	#[serde(rename = "FNLD")]
	CodeFNLD,
}

impl FinalStatus1Code {
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref bicfi_value) = self.bicfi { if !bicfi_value.validate() { return false; } }
		if let Some(ref clr_sys_mmb_id_value) = self.clr_sys_mmb_id { if !clr_sys_mmb_id_value.validate() { return false; } }
		if let Some(ref lei_value) = self.lei { if !lei_value.validate() { return false; } }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref pstl_adr_value) = self.pstl_adr { if !pstl_adr_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		return true
	}
}


// GenericIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.issr.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}").unwrap();
		if !pattern.is_match(&self.iban2007_identifier) {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
	}
}


// ImpliedCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ImpliedCurrencyAndAmount {
	#[serde(rename = "$value")]
	pub implied_currency_and_amount: f64,
}

impl ImpliedCurrencyAndAmount {
	pub fn validate(&self) -> bool {
		if self.implied_currency_and_amount < 0.000000 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.lei_identifier) {
			return false
		}
		return true
	}
}


// LongPaymentIdentification4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LongPaymentIdentification4 {
	#[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
	pub tx_id: Option<Max35Text>,
	#[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
	pub uetr: Option<UUIDv4Identifier>,
	#[serde(rename = "IntrBkSttlmAmt")]
	pub intr_bk_sttlm_amt: f64,
	#[serde(rename = "IntrBkSttlmDt")]
	pub intr_bk_sttlm_dt: String,
	#[serde(rename = "PmtMtd", skip_serializing_if = "Option::is_none")]
	pub pmt_mtd: Option<PaymentOrigin1Choice>,
	#[serde(rename = "InstgAgt")]
	pub instg_agt: BranchAndFinancialInstitutionIdentification8,
	#[serde(rename = "InstdAgt")]
	pub instd_agt: BranchAndFinancialInstitutionIdentification8,
	#[serde(rename = "NtryTp", skip_serializing_if = "Option::is_none")]
	pub ntry_tp: Option<EntryTypeIdentifier>,
	#[serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none")]
	pub end_to_end_id: Option<Max35Text>,
}

impl LongPaymentIdentification4 {
	pub fn validate(&self) -> bool {
		if let Some(ref tx_id_value) = self.tx_id { if !tx_id_value.validate() { return false; } }
		if let Some(ref uetr_value) = self.uetr { if !uetr_value.validate() { return false; } }
		if let Some(ref pmt_mtd_value) = self.pmt_mtd { if !pmt_mtd_value.validate() { return false; } }
		if !self.instg_agt.validate() { return false }
		if !self.instd_agt.validate() { return false }
		if let Some(ref ntry_tp_value) = self.ntry_tp { if !ntry_tp_value.validate() { return false; } }
		if let Some(ref end_to_end_id_value) = self.end_to_end_id { if !end_to_end_id_value.validate() { return false; } }
		return true
	}
}


// MarketInfrastructureIdentification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketInfrastructureIdentification1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalMarketInfrastructure1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl MarketInfrastructureIdentification1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// Max10Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max10Text {
	#[serde(rename = "$value")]
	pub max10_text: String,
}

impl Max10Text {
	pub fn validate(&self) -> bool {
		if self.max10_text.chars().count() < 1 {
			return false
		}
		if self.max10_text.chars().count() > 10 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.max128_text.chars().count() < 1 {
			return false
		}
		if self.max128_text.chars().count() > 128 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.max140_text.chars().count() < 1 {
			return false
		}
		if self.max140_text.chars().count() > 140 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[0-9]{1,15}").unwrap();
		if !pattern.is_match(&self.max15_numeric_text) {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.max16_text.chars().count() < 1 {
			return false
		}
		if self.max16_text.chars().count() > 16 {
			return false
		}
		return true
	}
}


// Max20000Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max20000Text {
	#[serde(rename = "$value")]
	pub max20000_text: String,
}

impl Max20000Text {
	pub fn validate(&self) -> bool {
		if self.max20000_text.chars().count() < 1 {
			return false
		}
		if self.max20000_text.chars().count() > 20000 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.max2048_text.chars().count() < 1 {
			return false
		}
		if self.max2048_text.chars().count() > 2048 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.max256_text.chars().count() < 1 {
			return false
		}
		if self.max256_text.chars().count() > 256 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.max34_text.chars().count() < 1 {
			return false
		}
		if self.max34_text.chars().count() > 34 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.max350_text.chars().count() < 1 {
			return false
		}
		if self.max350_text.chars().count() > 350 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.max35_text.chars().count() < 1 {
			return false
		}
		if self.max35_text.chars().count() > 35 {
			return false
		}
		return true
	}
}


// Max3NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max3NumericText {
	#[serde(rename = "$value")]
	pub max3_numeric_text: String,
}

impl Max3NumericText {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[0-9]{1,3}").unwrap();
		if !pattern.is_match(&self.max3_numeric_text) {
			return false
		}
		return true
	}
}


// Max4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "$value")]
	pub max4_alpha_numeric_text: String,
}

impl Max4AlphaNumericText {
	pub fn validate(&self) -> bool {
		if self.max4_alpha_numeric_text.chars().count() < 1 {
			return false
		}
		if self.max4_alpha_numeric_text.chars().count() > 4 {
			return false
		}
		let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
		if !pattern.is_match(&self.max4_alpha_numeric_text) {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.max4_text.chars().count() < 1 {
			return false
		}
		if self.max4_text.chars().count() > 4 {
			return false
		}
		return true
	}
}


// Max5NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max5NumericText {
	#[serde(rename = "$value")]
	pub max5_numeric_text: String,
}

impl Max5NumericText {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[0-9]{1,5}").unwrap();
		if !pattern.is_match(&self.max5_numeric_text) {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.max70_text.chars().count() < 1 {
			return false
		}
		if self.max70_text.chars().count() > 70 {
			return false
		}
		return true
	}
}


// MessageHeader8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader8 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
	#[serde(rename = "MsgPgntn", skip_serializing_if = "Option::is_none")]
	pub msg_pgntn: Option<Pagination1>,
	#[serde(rename = "OrgnlBizQry", skip_serializing_if = "Option::is_none")]
	pub orgnl_biz_qry: Option<OriginalBusinessQuery1>,
	#[serde(rename = "ReqTp", skip_serializing_if = "Option::is_none")]
	pub req_tp: Option<RequestType4Choice>,
	#[serde(rename = "QryNm", skip_serializing_if = "Option::is_none")]
	pub qry_nm: Option<Max35Text>,
}

impl MessageHeader8 {
	pub fn validate(&self) -> bool {
		if !self.msg_id.validate() { return false }
		if let Some(ref msg_pgntn_value) = self.msg_pgntn { if !msg_pgntn_value.validate() { return false; } }
		if let Some(ref orgnl_biz_qry_value) = self.orgnl_biz_qry { if !orgnl_biz_qry_value.validate() { return false; } }
		if let Some(ref req_tp_value) = self.req_tp { if !req_tp_value.validate() { return false; } }
		if let Some(ref qry_nm_value) = self.qry_nm { if !qry_nm_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
	}
}


// NumberAndSumOfTransactions2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NumberAndSumOfTransactions2 {
	#[serde(rename = "NbOfNtries", skip_serializing_if = "Option::is_none")]
	pub nb_of_ntries: Option<Max15NumericText>,
	#[serde(rename = "Sum", skip_serializing_if = "Option::is_none")]
	pub sum: Option<f64>,
	#[serde(rename = "TtlNetNtryAmt", skip_serializing_if = "Option::is_none")]
	pub ttl_net_ntry_amt: Option<f64>,
	#[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
}

impl NumberAndSumOfTransactions2 {
	pub fn validate(&self) -> bool {
		if let Some(ref nb_of_ntries_value) = self.nb_of_ntries { if !nb_of_ntries_value.validate() { return false; } }
		if let Some(ref cdt_dbt_ind_value) = self.cdt_dbt_ind { if !cdt_dbt_ind_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref any_bic_value) = self.any_bic { if !any_bic_value.validate() { return false; } }
		if let Some(ref lei_value) = self.lei { if !lei_value.validate() { return false; } }
		if let Some(ref othr_vec) = self.othr { for item in othr_vec { if !item.validate() { return false; } } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// OriginalBusinessQuery1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OriginalBusinessQuery1 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none")]
	pub msg_nm_id: Option<Max35Text>,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
}

impl OriginalBusinessQuery1 {
	pub fn validate(&self) -> bool {
		if !self.msg_id.validate() { return false }
		if let Some(ref msg_nm_id_value) = self.msg_nm_id { if !msg_nm_id_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if !self.chanl_tp.validate() { return false }
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
		return true
	}
}


// Pagination1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Pagination1 {
	#[serde(rename = "PgNb")]
	pub pg_nb: Max5NumericText,
	#[serde(rename = "LastPgInd")]
	pub last_pg_ind: bool,
}

impl Pagination1 {
	pub fn validate(&self) -> bool {
		if !self.pg_nb.validate() { return false }
		return true
	}
}


// Party50Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party50Choice {
	#[serde(rename = "Pty", skip_serializing_if = "Option::is_none")]
	pub pty: Option<PartyIdentification272>,
	#[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
	pub agt: Option<BranchAndFinancialInstitutionIdentification8>,
}

impl Party50Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref pty_value) = self.pty { if !pty_value.validate() { return false; } }
		if let Some(ref agt_value) = self.agt { if !agt_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref org_id_value) = self.org_id { if !org_id_value.validate() { return false; } }
		if let Some(ref prvt_id_value) = self.prvt_id { if !prvt_id_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref pstl_adr_value) = self.pstl_adr { if !pstl_adr_value.validate() { return false; } }
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
		if let Some(ref ctry_of_res_value) = self.ctry_of_res { if !ctry_of_res_value.validate() { return false; } }
		if let Some(ref ctct_dtls_value) = self.ctct_dtls { if !ctct_dtls_value.validate() { return false; } }
		return true
	}
}


// PaymentCommon6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentCommon6 {
	#[serde(rename = "PmtFr", skip_serializing_if = "Option::is_none")]
	pub pmt_fr: Option<System3>,
	#[serde(rename = "PmtTo", skip_serializing_if = "Option::is_none")]
	pub pmt_to: Option<System3>,
	#[serde(rename = "CmonSts", skip_serializing_if = "Option::is_none")]
	pub cmon_sts: Option<Vec<PaymentStatus6>>,
	#[serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none")]
	pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "NtryDt", skip_serializing_if = "Option::is_none")]
	pub ntry_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
	#[serde(rename = "PmtMtd", skip_serializing_if = "Option::is_none")]
	pub pmt_mtd: Option<PaymentOrigin1Choice>,
}

impl PaymentCommon6 {
	pub fn validate(&self) -> bool {
		if let Some(ref pmt_fr_value) = self.pmt_fr { if !pmt_fr_value.validate() { return false; } }
		if let Some(ref pmt_to_value) = self.pmt_to { if !pmt_to_value.validate() { return false; } }
		if let Some(ref cmon_sts_vec) = self.cmon_sts { for item in cmon_sts_vec { if !item.validate() { return false; } } }
		if let Some(ref reqd_exctn_dt_value) = self.reqd_exctn_dt { if !reqd_exctn_dt_value.validate() { return false; } }
		if let Some(ref ntry_dt_value) = self.ntry_dt { if !ntry_dt_value.validate() { return false; } }
		if let Some(ref cdt_dbt_ind_value) = self.cdt_dbt_ind { if !cdt_dbt_ind_value.validate() { return false; } }
		if let Some(ref pmt_mtd_value) = self.pmt_mtd { if !pmt_mtd_value.validate() { return false; } }
		return true
	}
}


// PaymentIdentification8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentIdentification8Choice {
	#[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
	pub tx_id: Option<Max35Text>,
	#[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
	pub uetr: Option<UUIDv4Identifier>,
	#[serde(rename = "QId", skip_serializing_if = "Option::is_none")]
	pub q_id: Option<QueueTransactionIdentification1>,
	#[serde(rename = "LngBizId", skip_serializing_if = "Option::is_none")]
	pub lng_biz_id: Option<LongPaymentIdentification4>,
	#[serde(rename = "ShrtBizId", skip_serializing_if = "Option::is_none")]
	pub shrt_biz_id: Option<ShortPaymentIdentification4>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<Max70Text>,
}

impl PaymentIdentification8Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref tx_id_value) = self.tx_id { if !tx_id_value.validate() { return false; } }
		if let Some(ref uetr_value) = self.uetr { if !uetr_value.validate() { return false; } }
		if let Some(ref q_id_value) = self.q_id { if !q_id_value.validate() { return false; } }
		if let Some(ref lng_biz_id_value) = self.lng_biz_id { if !lng_biz_id_value.validate() { return false; } }
		if let Some(ref shrt_biz_id_value) = self.shrt_biz_id { if !shrt_biz_id_value.validate() { return false; } }
		if let Some(ref prtry_id_value) = self.prtry_id { if !prtry_id_value.validate() { return false; } }
		return true
	}
}


// PaymentInstruction47 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentInstruction47 {
	#[serde(rename = "MsgId", skip_serializing_if = "Option::is_none")]
	pub msg_id: Option<Max35Text>,
	#[serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none")]
	pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
	pub sts: Option<Vec<PaymentStatus6>>,
	#[serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none")]
	pub instd_amt: Option<Amount3Choice>,
	#[serde(rename = "IntrBkSttlmAmt", skip_serializing_if = "Option::is_none")]
	pub intr_bk_sttlm_amt: Option<Amount2Choice>,
	#[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
	pub purp: Option<Max10Text>,
	#[serde(rename = "PmtMtd", skip_serializing_if = "Option::is_none")]
	pub pmt_mtd: Option<PaymentOrigin1Choice>,
	#[serde(rename = "Prty", skip_serializing_if = "Option::is_none")]
	pub prty: Option<Priority1Choice>,
	#[serde(rename = "PrcgVldtyTm", skip_serializing_if = "Option::is_none")]
	pub prcg_vldty_tm: Option<DateTimePeriod1Choice>,
	#[serde(rename = "InstrCpy", skip_serializing_if = "Option::is_none")]
	pub instr_cpy: Option<Max20000Text>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<PaymentType4Choice>,
	#[serde(rename = "GnrtdOrdr", skip_serializing_if = "Option::is_none")]
	pub gnrtd_ordr: Option<bool>,
	#[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
	pub tx_id: Option<Max35Text>,
	#[serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
	pub intr_bk_sttlm_dt: Option<String>,
	#[serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none")]
	pub end_to_end_id: Option<Max35Text>,
	#[serde(rename = "Pties", skip_serializing_if = "Option::is_none")]
	pub pties: Option<PaymentTransactionParty4>,
}

impl PaymentInstruction47 {
	pub fn validate(&self) -> bool {
		if let Some(ref msg_id_value) = self.msg_id { if !msg_id_value.validate() { return false; } }
		if let Some(ref reqd_exctn_dt_value) = self.reqd_exctn_dt { if !reqd_exctn_dt_value.validate() { return false; } }
		if let Some(ref sts_vec) = self.sts { for item in sts_vec { if !item.validate() { return false; } } }
		if let Some(ref instd_amt_value) = self.instd_amt { if !instd_amt_value.validate() { return false; } }
		if let Some(ref intr_bk_sttlm_amt_value) = self.intr_bk_sttlm_amt { if !intr_bk_sttlm_amt_value.validate() { return false; } }
		if let Some(ref purp_value) = self.purp { if !purp_value.validate() { return false; } }
		if let Some(ref pmt_mtd_value) = self.pmt_mtd { if !pmt_mtd_value.validate() { return false; } }
		if let Some(ref prty_value) = self.prty { if !prty_value.validate() { return false; } }
		if let Some(ref prcg_vldty_tm_value) = self.prcg_vldty_tm { if !prcg_vldty_tm_value.validate() { return false; } }
		if let Some(ref instr_cpy_value) = self.instr_cpy { if !instr_cpy_value.validate() { return false; } }
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		if let Some(ref tx_id_value) = self.tx_id { if !tx_id_value.validate() { return false; } }
		if let Some(ref end_to_end_id_value) = self.end_to_end_id { if !end_to_end_id_value.validate() { return false; } }
		if let Some(ref pties_value) = self.pties { if !pties_value.validate() { return false; } }
		return true
	}
}


// PaymentInstrument1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PaymentInstrument1Code {
	#[default]
	#[serde(rename = "BDT")]
	CodeBDT,
	#[serde(rename = "BCT")]
	CodeBCT,
	#[serde(rename = "CDT")]
	CodeCDT,
	#[serde(rename = "CCT")]
	CodeCCT,
	#[serde(rename = "CHK")]
	CodeCHK,
	#[serde(rename = "BKT")]
	CodeBKT,
	#[serde(rename = "DCP")]
	CodeDCP,
	#[serde(rename = "CCP")]
	CodeCCP,
	#[serde(rename = "RTI")]
	CodeRTI,
	#[serde(rename = "CAN")]
	CodeCAN,
}

impl PaymentInstrument1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// PaymentOrigin1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentOrigin1Choice {
	#[serde(rename = "FINMT", skip_serializing_if = "Option::is_none")]
	pub finmt: Option<Max3NumericText>,
	#[serde(rename = "XMLMsgNm", skip_serializing_if = "Option::is_none")]
	pub xml_msg_nm: Option<Max35Text>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
	#[serde(rename = "Instrm", skip_serializing_if = "Option::is_none")]
	pub instrm: Option<PaymentInstrument1Code>,
}

impl PaymentOrigin1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref finmt_value) = self.finmt { if !finmt_value.validate() { return false; } }
		if let Some(ref xml_msg_nm_value) = self.xml_msg_nm { if !xml_msg_nm_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		if let Some(ref instrm_value) = self.instrm { if !instrm_value.validate() { return false; } }
		return true
	}
}


// PaymentStatus6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentStatus6 {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PaymentStatusCode6Choice>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<DateAndDateTime2Choice>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<PaymentStatusReason1Choice>>,
}

impl PaymentStatus6 {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref dt_tm_value) = self.dt_tm { if !dt_tm_value.validate() { return false; } }
		if let Some(ref rsn_vec) = self.rsn { for item in rsn_vec { if !item.validate() { return false; } } }
		return true
	}
}


// PaymentStatusCode6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentStatusCode6Choice {
	#[serde(rename = "Pdg", skip_serializing_if = "Option::is_none")]
	pub pdg: Option<PendingStatus4Code>,
	#[serde(rename = "Fnl", skip_serializing_if = "Option::is_none")]
	pub fnl: Option<FinalStatus1Code>,
	#[serde(rename = "RTGS", skip_serializing_if = "Option::is_none")]
	pub rtgs: Option<Max4AlphaNumericText>,
	#[serde(rename = "Sttlm", skip_serializing_if = "Option::is_none")]
	pub sttlm: Option<Max4AlphaNumericText>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl PaymentStatusCode6Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref pdg_value) = self.pdg { if !pdg_value.validate() { return false; } }
		if let Some(ref fnl_value) = self.fnl { if !fnl_value.validate() { return false; } }
		if let Some(ref rtgs_value) = self.rtgs { if !rtgs_value.validate() { return false; } }
		if let Some(ref sttlm_value) = self.sttlm { if !sttlm_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// PaymentStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentStatusReason1Choice {
	#[serde(rename = "Umtchd", skip_serializing_if = "Option::is_none")]
	pub umtchd: Option<UnmatchedStatusReason1Code>,
	#[serde(rename = "Canc", skip_serializing_if = "Option::is_none")]
	pub canc: Option<CancelledStatusReason1Code>,
	#[serde(rename = "Sspd", skip_serializing_if = "Option::is_none")]
	pub sspd: Option<SuspendedStatusReason1Code>,
	#[serde(rename = "PdgFlngSttlm", skip_serializing_if = "Option::is_none")]
	pub pdg_flng_sttlm: Option<PendingFailingSettlement1Code>,
	#[serde(rename = "PdgSttlm", skip_serializing_if = "Option::is_none")]
	pub pdg_sttlm: Option<PendingSettlement2Code>,
	#[serde(rename = "PrtryRjctn", skip_serializing_if = "Option::is_none")]
	pub prtry_rjctn: Option<ProprietaryStatusJustification2>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl PaymentStatusReason1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref umtchd_value) = self.umtchd { if !umtchd_value.validate() { return false; } }
		if let Some(ref canc_value) = self.canc { if !canc_value.validate() { return false; } }
		if let Some(ref sspd_value) = self.sspd { if !sspd_value.validate() { return false; } }
		if let Some(ref pdg_flng_sttlm_value) = self.pdg_flng_sttlm { if !pdg_flng_sttlm_value.validate() { return false; } }
		if let Some(ref pdg_sttlm_value) = self.pdg_sttlm { if !pdg_sttlm_value.validate() { return false; } }
		if let Some(ref prtry_rjctn_value) = self.prtry_rjctn { if !prtry_rjctn_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// PaymentTransactionParty4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentTransactionParty4 {
	#[serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none")]
	pub instg_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none")]
	pub instd_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
	pub ultmt_dbtr: Option<Party50Choice>,
	#[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
	pub dbtr: Option<Party50Choice>,
	#[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "InstgRmbrsmntAgt", skip_serializing_if = "Option::is_none")]
	pub instg_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "InstdRmbrsmntAgt", skip_serializing_if = "Option::is_none")]
	pub instd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
	pub cdtr: Option<Party50Choice>,
	#[serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none")]
	pub ultmt_cdtr: Option<Party50Choice>,
}

impl PaymentTransactionParty4 {
	pub fn validate(&self) -> bool {
		if let Some(ref instg_agt_value) = self.instg_agt { if !instg_agt_value.validate() { return false; } }
		if let Some(ref instd_agt_value) = self.instd_agt { if !instd_agt_value.validate() { return false; } }
		if let Some(ref ultmt_dbtr_value) = self.ultmt_dbtr { if !ultmt_dbtr_value.validate() { return false; } }
		if let Some(ref dbtr_value) = self.dbtr { if !dbtr_value.validate() { return false; } }
		if let Some(ref dbtr_agt_value) = self.dbtr_agt { if !dbtr_agt_value.validate() { return false; } }
		if let Some(ref instg_rmbrsmnt_agt_value) = self.instg_rmbrsmnt_agt { if !instg_rmbrsmnt_agt_value.validate() { return false; } }
		if let Some(ref instd_rmbrsmnt_agt_value) = self.instd_rmbrsmnt_agt { if !instd_rmbrsmnt_agt_value.validate() { return false; } }
		if let Some(ref intrmy_agt1_value) = self.intrmy_agt1 { if !intrmy_agt1_value.validate() { return false; } }
		if let Some(ref intrmy_agt2_value) = self.intrmy_agt2 { if !intrmy_agt2_value.validate() { return false; } }
		if let Some(ref intrmy_agt3_value) = self.intrmy_agt3 { if !intrmy_agt3_value.validate() { return false; } }
		if let Some(ref cdtr_agt_value) = self.cdtr_agt { if !cdtr_agt_value.validate() { return false; } }
		if let Some(ref cdtr_value) = self.cdtr { if !cdtr_value.validate() { return false; } }
		if let Some(ref ultmt_cdtr_value) = self.ultmt_cdtr { if !ultmt_cdtr_value.validate() { return false; } }
		return true
	}
}


// PaymentType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PaymentType3Code {
	#[default]
	#[serde(rename = "CBS")]
	CodeCBS,
	#[serde(rename = "BCK")]
	CodeBCK,
	#[serde(rename = "BAL")]
	CodeBAL,
	#[serde(rename = "CLS")]
	CodeCLS,
	#[serde(rename = "CTR")]
	CodeCTR,
	#[serde(rename = "CBH")]
	CodeCBH,
	#[serde(rename = "CBP")]
	CodeCBP,
	#[serde(rename = "DPG")]
	CodeDPG,
	#[serde(rename = "DPN")]
	CodeDPN,
	#[serde(rename = "EXP")]
	CodeEXP,
	#[serde(rename = "TCH")]
	CodeTCH,
	#[serde(rename = "LMT")]
	CodeLMT,
	#[serde(rename = "LIQ")]
	CodeLIQ,
	#[serde(rename = "DPP")]
	CodeDPP,
	#[serde(rename = "DPH")]
	CodeDPH,
	#[serde(rename = "DPS")]
	CodeDPS,
	#[serde(rename = "STF")]
	CodeSTF,
	#[serde(rename = "TRP")]
	CodeTRP,
	#[serde(rename = "TCS")]
	CodeTCS,
	#[serde(rename = "LOA")]
	CodeLOA,
	#[serde(rename = "LOR")]
	CodeLOR,
	#[serde(rename = "TCP")]
	CodeTCP,
	#[serde(rename = "OND")]
	CodeOND,
	#[serde(rename = "MGL")]
	CodeMGL,
}

impl PaymentType3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// PaymentType4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentType4Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PaymentType3Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl PaymentType4Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// PendingFailingSettlement1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PendingFailingSettlement1Code {
	#[default]
	#[serde(rename = "AWMO")]
	CodeAWMO,
	#[serde(rename = "AWSH")]
	CodeAWSH,
	#[serde(rename = "LAAW")]
	CodeLAAW,
	#[serde(rename = "DOCY")]
	CodeDOCY,
	#[serde(rename = "CLAT")]
	CodeCLAT,
	#[serde(rename = "CERT")]
	CodeCERT,
	#[serde(rename = "MINO")]
	CodeMINO,
	#[serde(rename = "PHSE")]
	CodePHSE,
	#[serde(rename = "SBLO")]
	CodeSBLO,
	#[serde(rename = "DKNY")]
	CodeDKNY,
	#[serde(rename = "STCD")]
	CodeSTCD,
	#[serde(rename = "BENO")]
	CodeBENO,
	#[serde(rename = "LACK")]
	CodeLACK,
	#[serde(rename = "LATE")]
	CodeLATE,
	#[serde(rename = "CANR")]
	CodeCANR,
	#[serde(rename = "MLAT")]
	CodeMLAT,
	#[serde(rename = "OBJT")]
	CodeOBJT,
	#[serde(rename = "DOCC")]
	CodeDOCC,
	#[serde(rename = "BLOC")]
	CodeBLOC,
	#[serde(rename = "CHAS")]
	CodeCHAS,
	#[serde(rename = "NEWI")]
	CodeNEWI,
	#[serde(rename = "CLAC")]
	CodeCLAC,
	#[serde(rename = "PART")]
	CodePART,
	#[serde(rename = "CMON")]
	CodeCMON,
	#[serde(rename = "COLL")]
	CodeCOLL,
	#[serde(rename = "DEPO")]
	CodeDEPO,
	#[serde(rename = "FLIM")]
	CodeFLIM,
	#[serde(rename = "NOFX")]
	CodeNOFX,
	#[serde(rename = "INCA")]
	CodeINCA,
	#[serde(rename = "LINK")]
	CodeLINK,
	#[serde(rename = "BYIY")]
	CodeBYIY,
	#[serde(rename = "CAIS")]
	CodeCAIS,
	#[serde(rename = "LALO")]
	CodeLALO,
	#[serde(rename = "MONY")]
	CodeMONY,
	#[serde(rename = "NCON")]
	CodeNCON,
	#[serde(rename = "YCOL")]
	CodeYCOL,
	#[serde(rename = "REFS")]
	CodeREFS,
	#[serde(rename = "SDUT")]
	CodeSDUT,
	#[serde(rename = "CYCL")]
	CodeCYCL,
	#[serde(rename = "BATC")]
	CodeBATC,
	#[serde(rename = "GUAD")]
	CodeGUAD,
	#[serde(rename = "PREA")]
	CodePREA,
	#[serde(rename = "GLOB")]
	CodeGLOB,
	#[serde(rename = "CPEC")]
	CodeCPEC,
	#[serde(rename = "MUNO")]
	CodeMUNO,
}

impl PendingFailingSettlement1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// PendingSettlement2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PendingSettlement2Code {
	#[default]
	#[serde(rename = "AWMO")]
	CodeAWMO,
	#[serde(rename = "CAIS")]
	CodeCAIS,
	#[serde(rename = "REFU")]
	CodeREFU,
	#[serde(rename = "AWSH")]
	CodeAWSH,
	#[serde(rename = "PHSE")]
	CodePHSE,
	#[serde(rename = "TAMM")]
	CodeTAMM,
	#[serde(rename = "DOCY")]
	CodeDOCY,
	#[serde(rename = "DOCC")]
	CodeDOCC,
	#[serde(rename = "BLOC")]
	CodeBLOC,
	#[serde(rename = "CHAS")]
	CodeCHAS,
	#[serde(rename = "NEWI")]
	CodeNEWI,
	#[serde(rename = "CLAC")]
	CodeCLAC,
	#[serde(rename = "MUNO")]
	CodeMUNO,
	#[serde(rename = "GLOB")]
	CodeGLOB,
	#[serde(rename = "PREA")]
	CodePREA,
	#[serde(rename = "GUAD")]
	CodeGUAD,
	#[serde(rename = "PART")]
	CodePART,
	#[serde(rename = "NMAS")]
	CodeNMAS,
	#[serde(rename = "CMON")]
	CodeCMON,
	#[serde(rename = "YCOL")]
	CodeYCOL,
	#[serde(rename = "COLL")]
	CodeCOLL,
	#[serde(rename = "DEPO")]
	CodeDEPO,
	#[serde(rename = "FLIM")]
	CodeFLIM,
	#[serde(rename = "NOFX")]
	CodeNOFX,
	#[serde(rename = "INCA")]
	CodeINCA,
	#[serde(rename = "LINK")]
	CodeLINK,
	#[serde(rename = "FUTU")]
	CodeFUTU,
	#[serde(rename = "LACK")]
	CodeLACK,
	#[serde(rename = "LALO")]
	CodeLALO,
	#[serde(rename = "MONY")]
	CodeMONY,
	#[serde(rename = "NCON")]
	CodeNCON,
	#[serde(rename = "REFS")]
	CodeREFS,
	#[serde(rename = "SDUT")]
	CodeSDUT,
	#[serde(rename = "BATC")]
	CodeBATC,
	#[serde(rename = "CYCL")]
	CodeCYCL,
	#[serde(rename = "SBLO")]
	CodeSBLO,
	#[serde(rename = "CPEC")]
	CodeCPEC,
	#[serde(rename = "MINO")]
	CodeMINO,
	#[serde(rename = "PCAP")]
	CodePCAP,
}

impl PendingSettlement2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// PendingStatus4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PendingStatus4Code {
	#[default]
	#[serde(rename = "ACPD")]
	CodeACPD,
	#[serde(rename = "VALD")]
	CodeVALD,
	#[serde(rename = "MATD")]
	CodeMATD,
	#[serde(rename = "AUTD")]
	CodeAUTD,
	#[serde(rename = "INVD")]
	CodeINVD,
	#[serde(rename = "UMAC")]
	CodeUMAC,
	#[serde(rename = "STLE")]
	CodeSTLE,
	#[serde(rename = "STLM")]
	CodeSTLM,
	#[serde(rename = "SSPD")]
	CodeSSPD,
	#[serde(rename = "PCAN")]
	CodePCAN,
	#[serde(rename = "PSTL")]
	CodePSTL,
	#[serde(rename = "PFST")]
	CodePFST,
	#[serde(rename = "SMLR")]
	CodeSMLR,
	#[serde(rename = "RMLR")]
	CodeRMLR,
	#[serde(rename = "SRBL")]
	CodeSRBL,
	#[serde(rename = "AVLB")]
	CodeAVLB,
	#[serde(rename = "SRML")]
	CodeSRML,
}

impl PendingStatus4Code {
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref dt_and_plc_of_birth_value) = self.dt_and_plc_of_birth { if !dt_and_plc_of_birth_value.validate() { return false; } }
		if let Some(ref othr_vec) = self.othr { for item in othr_vec { if !item.validate() { return false; } } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
		if !pattern.is_match(&self.phone_number) {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref adr_tp_value) = self.adr_tp { if !adr_tp_value.validate() { return false; } }
		if let Some(ref care_of_value) = self.care_of { if !care_of_value.validate() { return false; } }
		if let Some(ref dept_value) = self.dept { if !dept_value.validate() { return false; } }
		if let Some(ref sub_dept_value) = self.sub_dept { if !sub_dept_value.validate() { return false; } }
		if let Some(ref strt_nm_value) = self.strt_nm { if !strt_nm_value.validate() { return false; } }
		if let Some(ref bldg_nb_value) = self.bldg_nb { if !bldg_nb_value.validate() { return false; } }
		if let Some(ref bldg_nm_value) = self.bldg_nm { if !bldg_nm_value.validate() { return false; } }
		if let Some(ref flr_value) = self.flr { if !flr_value.validate() { return false; } }
		if let Some(ref unit_nb_value) = self.unit_nb { if !unit_nb_value.validate() { return false; } }
		if let Some(ref pst_bx_value) = self.pst_bx { if !pst_bx_value.validate() { return false; } }
		if let Some(ref room_value) = self.room { if !room_value.validate() { return false; } }
		if let Some(ref pst_cd_value) = self.pst_cd { if !pst_cd_value.validate() { return false; } }
		if let Some(ref twn_nm_value) = self.twn_nm { if !twn_nm_value.validate() { return false; } }
		if let Some(ref twn_lctn_nm_value) = self.twn_lctn_nm { if !twn_lctn_nm_value.validate() { return false; } }
		if let Some(ref dstrct_nm_value) = self.dstrct_nm { if !dstrct_nm_value.validate() { return false; } }
		if let Some(ref ctry_sub_dvsn_value) = self.ctry_sub_dvsn { if !ctry_sub_dvsn_value.validate() { return false; } }
		if let Some(ref ctry_value) = self.ctry { if !ctry_value.validate() { return false; } }
		if let Some(ref adr_line_vec) = self.adr_line { for item in adr_line_vec { if !item.validate() { return false; } } }
		return true
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
	pub fn validate(&self) -> bool {
		return true
	}
}


// Priority1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Priority1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Priority5Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl Priority1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// Priority5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Priority5Code {
	#[default]
	#[serde(rename = "HIGH")]
	CodeHIGH,
	#[serde(rename = "LOWW")]
	CodeLOWW,
	#[serde(rename = "NORM")]
	CodeNORM,
	#[serde(rename = "URGT")]
	CodeURGT,
}

impl Priority5Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ProprietaryStatusJustification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryStatusJustification2 {
	#[serde(rename = "PrtryStsRsn")]
	pub prtry_sts_rsn: Max4AlphaNumericText,
	#[serde(rename = "Rsn")]
	pub rsn: Max256Text,
}

impl ProprietaryStatusJustification2 {
	pub fn validate(&self) -> bool {
		if !self.prtry_sts_rsn.validate() { return false }
		if !self.rsn.validate() { return false }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		if !self.id.validate() { return false }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// QueueTransactionIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct QueueTransactionIdentification1 {
	#[serde(rename = "QId")]
	pub q_id: Max16Text,
	#[serde(rename = "PosInQ")]
	pub pos_in_q: Max16Text,
}

impl QueueTransactionIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.q_id.validate() { return false }
		if !self.pos_in_q.validate() { return false }
		return true
	}
}


// RequestType4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestType4Choice {
	#[serde(rename = "PmtCtrl", skip_serializing_if = "Option::is_none")]
	pub pmt_ctrl: Option<ExternalPaymentControlRequestType1Code>,
	#[serde(rename = "Enqry", skip_serializing_if = "Option::is_none")]
	pub enqry: Option<ExternalEnquiryRequestType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification1>,
}

impl RequestType4Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref pmt_ctrl_value) = self.pmt_ctrl { if !pmt_ctrl_value.validate() { return false; } }
		if let Some(ref enqry_value) = self.enqry { if !enqry_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// ReturnTransactionV11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReturnTransactionV11 {
	#[serde(rename = "MsgHdr")]
	pub msg_hdr: MessageHeader8,
	#[serde(rename = "RptOrErr")]
	pub rpt_or_err: TransactionReportOrError7Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl ReturnTransactionV11 {
	pub fn validate(&self) -> bool {
		if !self.msg_hdr.validate() { return false }
		if !self.rpt_or_err.validate() { return false }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
	}
}


// SecuritiesTransactionReferences1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionReferences1 {
	#[serde(rename = "AcctOwnrTxId", skip_serializing_if = "Option::is_none")]
	pub acct_ownr_tx_id: Option<Max35Text>,
	#[serde(rename = "AcctSvcrTxId", skip_serializing_if = "Option::is_none")]
	pub acct_svcr_tx_id: Option<Max35Text>,
	#[serde(rename = "MktInfrstrctrTxId", skip_serializing_if = "Option::is_none")]
	pub mkt_infrstrctr_tx_id: Option<Max35Text>,
	#[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
	pub prcg_id: Option<Max35Text>,
}

impl SecuritiesTransactionReferences1 {
	pub fn validate(&self) -> bool {
		if let Some(ref acct_ownr_tx_id_value) = self.acct_ownr_tx_id { if !acct_ownr_tx_id_value.validate() { return false; } }
		if let Some(ref acct_svcr_tx_id_value) = self.acct_svcr_tx_id { if !acct_svcr_tx_id_value.validate() { return false; } }
		if let Some(ref mkt_infrstrctr_tx_id_value) = self.mkt_infrstrctr_tx_id { if !mkt_infrstrctr_tx_id_value.validate() { return false; } }
		if let Some(ref prcg_id_value) = self.prcg_id { if !prcg_id_value.validate() { return false; } }
		return true
	}
}


// ShortPaymentIdentification4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ShortPaymentIdentification4 {
	#[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
	pub tx_id: Option<Max35Text>,
	#[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
	pub uetr: Option<UUIDv4Identifier>,
	#[serde(rename = "IntrBkSttlmDt")]
	pub intr_bk_sttlm_dt: String,
	#[serde(rename = "InstgAgt")]
	pub instg_agt: BranchAndFinancialInstitutionIdentification8,
}

impl ShortPaymentIdentification4 {
	pub fn validate(&self) -> bool {
		if let Some(ref tx_id_value) = self.tx_id { if !tx_id_value.validate() { return false; } }
		if let Some(ref uetr_value) = self.uetr { if !uetr_value.validate() { return false; } }
		if !self.instg_agt.validate() { return false }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref plc_and_nm_value) = self.plc_and_nm { if !plc_and_nm_value.validate() { return false; } }
		if !self.envlp.validate() { return false }
		return true
	}
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// SuspendedStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SuspendedStatusReason1Code {
	#[default]
	#[serde(rename = "SUBY")]
	CodeSUBY,
	#[serde(rename = "SUBS")]
	CodeSUBS,
}

impl SuspendedStatusReason1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// System3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct System3 {
	#[serde(rename = "SysId", skip_serializing_if = "Option::is_none")]
	pub sys_id: Option<MarketInfrastructureIdentification1Choice>,
	#[serde(rename = "MmbId", skip_serializing_if = "Option::is_none")]
	pub mmb_id: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
	#[serde(rename = "AcctId", skip_serializing_if = "Option::is_none")]
	pub acct_id: Option<AccountIdentification4Choice>,
}

impl System3 {
	pub fn validate(&self) -> bool {
		if let Some(ref sys_id_value) = self.sys_id { if !sys_id_value.validate() { return false; } }
		if let Some(ref mmb_id_value) = self.mmb_id { if !mmb_id_value.validate() { return false; } }
		if let Some(ref ctry_value) = self.ctry { if !ctry_value.validate() { return false; } }
		if let Some(ref acct_id_value) = self.acct_id { if !acct_id_value.validate() { return false; } }
		return true
	}
}


// Transaction159 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Transaction159 {
	#[serde(rename = "PmtTo", skip_serializing_if = "Option::is_none")]
	pub pmt_to: Option<System3>,
	#[serde(rename = "PmtFr", skip_serializing_if = "Option::is_none")]
	pub pmt_fr: Option<System3>,
	#[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
	#[serde(rename = "Pmt", skip_serializing_if = "Option::is_none")]
	pub pmt: Option<PaymentInstruction47>,
	#[serde(rename = "AcctNtry", skip_serializing_if = "Option::is_none")]
	pub acct_ntry: Option<CashAccountAndEntry5>,
	#[serde(rename = "SctiesTxRefs", skip_serializing_if = "Option::is_none")]
	pub scties_tx_refs: Option<SecuritiesTransactionReferences1>,
}

impl Transaction159 {
	pub fn validate(&self) -> bool {
		if let Some(ref pmt_to_value) = self.pmt_to { if !pmt_to_value.validate() { return false; } }
		if let Some(ref pmt_fr_value) = self.pmt_fr { if !pmt_fr_value.validate() { return false; } }
		if let Some(ref cdt_dbt_ind_value) = self.cdt_dbt_ind { if !cdt_dbt_ind_value.validate() { return false; } }
		if let Some(ref pmt_value) = self.pmt { if !pmt_value.validate() { return false; } }
		if let Some(ref acct_ntry_value) = self.acct_ntry { if !acct_ntry_value.validate() { return false; } }
		if let Some(ref scties_tx_refs_value) = self.scties_tx_refs { if !scties_tx_refs_value.validate() { return false; } }
		return true
	}
}


// TransactionOrError6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionOrError6Choice {
	#[serde(rename = "Tx", skip_serializing_if = "Option::is_none")]
	pub tx: Option<Transaction159>,
	#[serde(rename = "BizErr", skip_serializing_if = "Option::is_none")]
	pub biz_err: Option<Vec<ErrorHandling5>>,
}

impl TransactionOrError6Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref tx_value) = self.tx { if !tx_value.validate() { return false; } }
		if let Some(ref biz_err_vec) = self.biz_err { for item in biz_err_vec { if !item.validate() { return false; } } }
		return true
	}
}


// TransactionReport8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionReport8 {
	#[serde(rename = "PmtId")]
	pub pmt_id: PaymentIdentification8Choice,
	#[serde(rename = "TxOrErr")]
	pub tx_or_err: TransactionOrError6Choice,
}

impl TransactionReport8 {
	pub fn validate(&self) -> bool {
		if !self.pmt_id.validate() { return false }
		if !self.tx_or_err.validate() { return false }
		return true
	}
}


// TransactionReportOrError7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionReportOrError7Choice {
	#[serde(rename = "BizRpt", skip_serializing_if = "Option::is_none")]
	pub biz_rpt: Option<Transactions11>,
	#[serde(rename = "OprlErr", skip_serializing_if = "Option::is_none")]
	pub oprl_err: Option<Vec<ErrorHandling5>>,
}

impl TransactionReportOrError7Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref biz_rpt_value) = self.biz_rpt { if !biz_rpt_value.validate() { return false; } }
		if let Some(ref oprl_err_vec) = self.oprl_err { for item in oprl_err_vec { if !item.validate() { return false; } } }
		return true
	}
}


// Transactions11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Transactions11 {
	#[serde(rename = "PmtCmonInf", skip_serializing_if = "Option::is_none")]
	pub pmt_cmon_inf: Option<PaymentCommon6>,
	#[serde(rename = "TxsSummry", skip_serializing_if = "Option::is_none")]
	pub txs_summry: Option<NumberAndSumOfTransactions2>,
	#[serde(rename = "TxRpt")]
	pub tx_rpt: Vec<TransactionReport8>,
}

impl Transactions11 {
	pub fn validate(&self) -> bool {
		if let Some(ref pmt_cmon_inf_value) = self.pmt_cmon_inf { if !pmt_cmon_inf_value.validate() { return false; } }
		if let Some(ref txs_summry_value) = self.txs_summry { if !txs_summry_value.validate() { return false; } }
		for item in &self.tx_rpt { if !item.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
		if !pattern.is_match(&self.uui_dv4_identifier) {
			return false
		}
		return true
	}
}


// UnmatchedStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum UnmatchedStatusReason1Code {
	#[default]
	#[serde(rename = "CMIS")]
	CodeCMIS,
	#[serde(rename = "DDAT")]
	CodeDDAT,
	#[serde(rename = "DELN")]
	CodeDELN,
	#[serde(rename = "DEPT")]
	CodeDEPT,
	#[serde(rename = "DMON")]
	CodeDMON,
	#[serde(rename = "DDEA")]
	CodeDDEA,
	#[serde(rename = "DQUA")]
	CodeDQUA,
	#[serde(rename = "CADE")]
	CodeCADE,
	#[serde(rename = "SETR")]
	CodeSETR,
	#[serde(rename = "DSEC")]
	CodeDSEC,
	#[serde(rename = "VASU")]
	CodeVASU,
	#[serde(rename = "DTRA")]
	CodeDTRA,
	#[serde(rename = "RSPR")]
	CodeRSPR,
	#[serde(rename = "REPO")]
	CodeREPO,
	#[serde(rename = "CLAT")]
	CodeCLAT,
	#[serde(rename = "RERT")]
	CodeRERT,
	#[serde(rename = "REPA")]
	CodeREPA,
	#[serde(rename = "REPP")]
	CodeREPP,
	#[serde(rename = "PHYS")]
	CodePHYS,
	#[serde(rename = "IIND")]
	CodeIIND,
	#[serde(rename = "FRAP")]
	CodeFRAP,
	#[serde(rename = "PLCE")]
	CodePLCE,
	#[serde(rename = "PODU")]
	CodePODU,
	#[serde(rename = "FORF")]
	CodeFORF,
	#[serde(rename = "REGD")]
	CodeREGD,
	#[serde(rename = "RTGS")]
	CodeRTGS,
	#[serde(rename = "ICAG")]
	CodeICAG,
	#[serde(rename = "CPCA")]
	CodeCPCA,
	#[serde(rename = "CHAR")]
	CodeCHAR,
	#[serde(rename = "IEXE")]
	CodeIEXE,
	#[serde(rename = "NCRR")]
	CodeNCRR,
	#[serde(rename = "NMAS")]
	CodeNMAS,
	#[serde(rename = "SAFE")]
	CodeSAFE,
	#[serde(rename = "DTRD")]
	CodeDTRD,
	#[serde(rename = "LATE")]
	CodeLATE,
	#[serde(rename = "TERM")]
	CodeTERM,
	#[serde(rename = "ICUS")]
	CodeICUS,
}

impl UnmatchedStatusReason1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct YesNoIndicator {
	#[serde(rename = "$value")]
	pub yes_no_indicator: bool,
}

impl YesNoIndicator {
	pub fn validate(&self) -> bool {
		return true
	}
}
