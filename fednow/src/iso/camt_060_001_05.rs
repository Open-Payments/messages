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


// AccountReportingRequestV05 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountReportingRequestV05 {
	#[serde(rename = "GrpHdr")]
	pub grp_hdr: GroupHeader77,
	#[serde(rename = "RptgReq")]
	pub rptg_req: Vec<ReportingRequest5>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl AccountReportingRequestV05 {
	pub fn validate(&self) -> bool {
		if !self.grp_hdr.validate() { return false }
		for item in &self.rptg_req { if !item.validate() { return false; } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
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


// BalanceSubType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BalanceSubType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalBalanceSubType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl BalanceSubType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// BalanceType10Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BalanceType10Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalBalanceType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl BalanceType10Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// BalanceType13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BalanceType13 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: BalanceType10Choice,
	#[serde(rename = "SubTp", skip_serializing_if = "Option::is_none")]
	pub sub_tp: Option<BalanceSubType1Choice>,
}

impl BalanceType13 {
	pub fn validate(&self) -> bool {
		if !self.cd_or_prtry.validate() { return false }
		if let Some(ref sub_tp_value) = self.sub_tp { if !sub_tp_value.validate() { return false; } }
		return true
	}
}


// BranchAndFinancialInstitutionIdentification6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BranchAndFinancialInstitutionIdentification6 {
	#[serde(rename = "FinInstnId")]
	pub fin_instn_id: FinancialInstitutionIdentification18,
	#[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
	pub brnch_id: Option<BranchData3>,
}

impl BranchAndFinancialInstitutionIdentification6 {
	pub fn validate(&self) -> bool {
		if !self.fin_instn_id.validate() { return false }
		if let Some(ref brnch_id_value) = self.brnch_id { if !brnch_id_value.validate() { return false; } }
		return true
	}
}


// BranchData3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BranchData3 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max35Text>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress24>,
}

impl BranchData3 {
	pub fn validate(&self) -> bool {
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
		if let Some(ref lei_value) = self.lei { if !lei_value.validate() { return false; } }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref pstl_adr_value) = self.pstl_adr { if !pstl_adr_value.validate() { return false; } }
		return true
	}
}


// CashAccount38 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccount38 {
	#[serde(rename = "Id")]
	pub id: AccountIdentification4Choice,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<CashAccountType2Choice>,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max70Text>,
	#[serde(rename = "Prxy", skip_serializing_if = "Option::is_none")]
	pub prxy: Option<ProxyAccountIdentification1>,
}

impl CashAccount38 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		if let Some(ref ccy_value) = self.ccy { if !ccy_value.validate() { return false; } }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref prxy_value) = self.prxy { if !prxy_value.validate() { return false; } }
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


// Contact4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Contact4 {
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
	#[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
	pub email_adr: Option<Max2048Text>,
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
	pub prefrd_mtd: Option<PreferredContactMethod1Code>,
}

impl Contact4 {
	pub fn validate(&self) -> bool {
		if let Some(ref nm_prfx_value) = self.nm_prfx { if !nm_prfx_value.validate() { return false; } }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref phne_nb_value) = self.phne_nb { if !phne_nb_value.validate() { return false; } }
		if let Some(ref mob_nb_value) = self.mob_nb { if !mob_nb_value.validate() { return false; } }
		if let Some(ref fax_nb_value) = self.fax_nb { if !fax_nb_value.validate() { return false; } }
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


// DatePeriodDetails1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriodDetails1 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt", skip_serializing_if = "Option::is_none")]
	pub to_dt: Option<String>,
}

impl DatePeriodDetails1 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// EntryStatus1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EntryStatus1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalEntryStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl EntryStatus1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
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


// ExternalBalanceSubType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalBalanceSubType1Code {
	#[serde(rename = "$value")]
	pub external_balance_sub_type1_code: String,
}

impl ExternalBalanceSubType1Code {
	pub fn validate(&self) -> bool {
		if self.external_balance_sub_type1_code.chars().count() < 1 {
			return false
		}
		if self.external_balance_sub_type1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// ExternalBalanceType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalBalanceType1Code {
	#[serde(rename = "$value")]
	pub external_balance_type1_code: String,
}

impl ExternalBalanceType1Code {
	pub fn validate(&self) -> bool {
		if self.external_balance_type1_code.chars().count() < 1 {
			return false
		}
		if self.external_balance_type1_code.chars().count() > 4 {
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


// ExternalEntryStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalEntryStatus1Code {
	#[serde(rename = "$value")]
	pub external_entry_status1_code: String,
}

impl ExternalEntryStatus1Code {
	pub fn validate(&self) -> bool {
		if self.external_entry_status1_code.chars().count() < 1 {
			return false
		}
		if self.external_entry_status1_code.chars().count() > 4 {
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


// FinancialInstitutionIdentification18 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstitutionIdentification18 {
	#[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
	pub bicfi: Option<BICFIDec2014Identifier>,
	#[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress24>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<GenericFinancialIdentification1>,
}

impl FinancialInstitutionIdentification18 {
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


// FloorLimitType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FloorLimitType1Code {
	#[default]
	#[serde(rename = "CRED")]
	CodeCRED,
	#[serde(rename = "DEBT")]
	CodeDEBT,
	#[serde(rename = "BOTH")]
	CodeBOTH,
}

impl FloorLimitType1Code {
	pub fn validate(&self) -> bool {
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


// GenericOrganisationIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericOrganisationIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericOrganisationIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		return true
	}
}


// GenericPersonIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericPersonIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericPersonIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		return true
	}
}


// GroupHeader77 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GroupHeader77 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
	#[serde(rename = "MsgSndr", skip_serializing_if = "Option::is_none")]
	pub msg_sndr: Option<Party40Choice>,
}

impl GroupHeader77 {
	pub fn validate(&self) -> bool {
		if !self.msg_id.validate() { return false }
		if let Some(ref msg_sndr_value) = self.msg_sndr { if !msg_sndr_value.validate() { return false; } }
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


// ISOTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISOTime {
	#[serde(rename = "$value")]
	pub iso_time: String,
}

impl ISOTime {
	pub fn validate(&self) -> bool {
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


// Limit2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Limit2 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: FloorLimitType1Code,
}

impl Limit2 {
	pub fn validate(&self) -> bool {
		if !self.amt.validate() { return false }
		if !self.cdt_dbt_ind.validate() { return false }
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


// MessageNameIdentificationFRS1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MessageNameIdentificationFRS1 {
	#[serde(rename = "$value")]
	pub message_name_identification_frs_1: String,
}

impl MessageNameIdentificationFRS1 {
	pub fn validate(&self) -> bool {
		if self.message_name_identification_frs_1.chars().count() < 15 {
			return false
		}
		if self.message_name_identification_frs_1.chars().count() > 15 {
			return false
		}
		let pattern = Regex::new("[a-z]{4,4}[.]{1,1}[0-9]{3,3}[.]{1,1}001[.]{1,1}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.message_name_identification_frs_1) {
			return false
		}
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


// OrganisationIdentification29 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification29 {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<GenericOrganisationIdentification1>>,
}

impl OrganisationIdentification29 {
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


// Party38Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party38Choice {
	#[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
	pub org_id: Option<OrganisationIdentification29>,
	#[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
	pub prvt_id: Option<PersonIdentification13>,
}

impl Party38Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref org_id_value) = self.org_id { if !org_id_value.validate() { return false; } }
		if let Some(ref prvt_id_value) = self.prvt_id { if !prvt_id_value.validate() { return false; } }
		return true
	}
}


// Party40Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party40Choice {
	#[serde(rename = "Pty", skip_serializing_if = "Option::is_none")]
	pub pty: Option<PartyIdentification135>,
	#[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
	pub agt: Option<BranchAndFinancialInstitutionIdentification6>,
}

impl Party40Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref pty_value) = self.pty { if !pty_value.validate() { return false; } }
		if let Some(ref agt_value) = self.agt { if !agt_value.validate() { return false; } }
		return true
	}
}


// PartyIdentification135 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification135 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress24>,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Party38Choice>,
	#[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
	pub ctry_of_res: Option<CountryCode>,
	#[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
	pub ctct_dtls: Option<Contact4>,
}

impl PartyIdentification135 {
	pub fn validate(&self) -> bool {
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref pstl_adr_value) = self.pstl_adr { if !pstl_adr_value.validate() { return false; } }
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
		if let Some(ref ctry_of_res_value) = self.ctry_of_res { if !ctry_of_res_value.validate() { return false; } }
		if let Some(ref ctct_dtls_value) = self.ctct_dtls { if !ctct_dtls_value.validate() { return false; } }
		return true
	}
}


// PersonIdentification13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentification13 {
	#[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
	pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<GenericPersonIdentification1>>,
}

impl PersonIdentification13 {
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


// PostalAddress24 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress24 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<AddressType3Choice>,
	#[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
	pub dept: Option<Max70Text>,
	#[serde(rename = "SubDept", skip_serializing_if = "Option::is_none")]
	pub sub_dept: Option<Max70Text>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<Max70Text>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<Max16Text>,
	#[serde(rename = "BldgNm", skip_serializing_if = "Option::is_none")]
	pub bldg_nm: Option<Max35Text>,
	#[serde(rename = "Flr", skip_serializing_if = "Option::is_none")]
	pub flr: Option<Max70Text>,
	#[serde(rename = "PstBx", skip_serializing_if = "Option::is_none")]
	pub pst_bx: Option<Max16Text>,
	#[serde(rename = "Room", skip_serializing_if = "Option::is_none")]
	pub room: Option<Max70Text>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<Max16Text>,
	#[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
	pub twn_nm: Option<Max35Text>,
	#[serde(rename = "TwnLctnNm", skip_serializing_if = "Option::is_none")]
	pub twn_lctn_nm: Option<Max35Text>,
	#[serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none")]
	pub dstrct_nm: Option<Max35Text>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<Max35Text>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
	pub adr_line: Option<Vec<Max70Text>>,
}

impl PostalAddress24 {
	pub fn validate(&self) -> bool {
		if let Some(ref adr_tp_value) = self.adr_tp { if !adr_tp_value.validate() { return false; } }
		if let Some(ref dept_value) = self.dept { if !dept_value.validate() { return false; } }
		if let Some(ref sub_dept_value) = self.sub_dept { if !sub_dept_value.validate() { return false; } }
		if let Some(ref strt_nm_value) = self.strt_nm { if !strt_nm_value.validate() { return false; } }
		if let Some(ref bldg_nb_value) = self.bldg_nb { if !bldg_nb_value.validate() { return false; } }
		if let Some(ref bldg_nm_value) = self.bldg_nm { if !bldg_nm_value.validate() { return false; } }
		if let Some(ref flr_value) = self.flr { if !flr_value.validate() { return false; } }
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


// PreferredContactMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PreferredContactMethod1Code {
	#[default]
	#[serde(rename = "LETT")]
	CodeLETT,
	#[serde(rename = "MAIL")]
	CodeMAIL,
	#[serde(rename = "PHON")]
	CodePHON,
	#[serde(rename = "FAXX")]
	CodeFAXX,
	#[serde(rename = "CELL")]
	CodeCELL,
}

impl PreferredContactMethod1Code {
	pub fn validate(&self) -> bool {
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


// QueryType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum QueryType3Code {
	#[default]
	#[serde(rename = "ALLL")]
	CodeALLL,
	#[serde(rename = "CHNG")]
	CodeCHNG,
	#[serde(rename = "MODF")]
	CodeMODF,
}

impl QueryType3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ReportingPeriod2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportingPeriod2 {
	#[serde(rename = "FrToDt")]
	pub fr_to_dt: DatePeriodDetails1,
	#[serde(rename = "FrToTm", skip_serializing_if = "Option::is_none")]
	pub fr_to_tm: Option<TimePeriodDetails1>,
	#[serde(rename = "Tp")]
	pub tp: QueryType3Code,
}

impl ReportingPeriod2 {
	pub fn validate(&self) -> bool {
		if !self.fr_to_dt.validate() { return false }
		if let Some(ref fr_to_tm_value) = self.fr_to_tm { if !fr_to_tm_value.validate() { return false; } }
		if !self.tp.validate() { return false }
		return true
	}
}


// ReportingRequest5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportingRequest5 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max35Text>,
	#[serde(rename = "ReqdMsgNmId")]
	pub reqd_msg_nm_id: MessageNameIdentificationFRS1,
	#[serde(rename = "Acct", skip_serializing_if = "Option::is_none")]
	pub acct: Option<CashAccount38>,
	#[serde(rename = "AcctOwnr")]
	pub acct_ownr: Party40Choice,
	#[serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none")]
	pub acct_svcr: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "RptgPrd", skip_serializing_if = "Option::is_none")]
	pub rptg_prd: Option<ReportingPeriod2>,
	#[serde(rename = "RptgSeq", skip_serializing_if = "Option::is_none")]
	pub rptg_seq: Option<SequenceRange1Choice>,
	#[serde(rename = "ReqdTxTp", skip_serializing_if = "Option::is_none")]
	pub reqd_tx_tp: Option<TransactionType2>,
	#[serde(rename = "ReqdBalTp", skip_serializing_if = "Option::is_none")]
	pub reqd_bal_tp: Option<Vec<BalanceType13>>,
}

impl ReportingRequest5 {
	pub fn validate(&self) -> bool {
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
		if !self.reqd_msg_nm_id.validate() { return false }
		if let Some(ref acct_value) = self.acct { if !acct_value.validate() { return false; } }
		if !self.acct_ownr.validate() { return false }
		if let Some(ref acct_svcr_value) = self.acct_svcr { if !acct_svcr_value.validate() { return false; } }
		if let Some(ref rptg_prd_value) = self.rptg_prd { if !rptg_prd_value.validate() { return false; } }
		if let Some(ref rptg_seq_value) = self.rptg_seq { if !rptg_seq_value.validate() { return false; } }
		if let Some(ref reqd_tx_tp_value) = self.reqd_tx_tp { if !reqd_tx_tp_value.validate() { return false; } }
		if let Some(ref reqd_bal_tp_vec) = self.reqd_bal_tp { for item in reqd_bal_tp_vec { if !item.validate() { return false; } } }
		return true
	}
}


// SequenceRange1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SequenceRange1 {
	#[serde(rename = "FrSeq")]
	pub fr_seq: Max35Text,
	#[serde(rename = "ToSeq")]
	pub to_seq: Max35Text,
}

impl SequenceRange1 {
	pub fn validate(&self) -> bool {
		if !self.fr_seq.validate() { return false }
		if !self.to_seq.validate() { return false }
		return true
	}
}


// SequenceRange1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SequenceRange1Choice {
	#[serde(rename = "FrSeq", skip_serializing_if = "Option::is_none")]
	pub fr_seq: Option<Max35Text>,
	#[serde(rename = "ToSeq", skip_serializing_if = "Option::is_none")]
	pub to_seq: Option<Max35Text>,
	#[serde(rename = "FrToSeq", skip_serializing_if = "Option::is_none")]
	pub fr_to_seq: Option<Vec<SequenceRange1>>,
	#[serde(rename = "EQSeq", skip_serializing_if = "Option::is_none")]
	pub eq_seq: Option<Vec<Max35Text>>,
	#[serde(rename = "NEQSeq", skip_serializing_if = "Option::is_none")]
	pub neq_seq: Option<Vec<Max35Text>>,
}

impl SequenceRange1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref fr_seq_value) = self.fr_seq { if !fr_seq_value.validate() { return false; } }
		if let Some(ref to_seq_value) = self.to_seq { if !to_seq_value.validate() { return false; } }
		if let Some(ref fr_to_seq_vec) = self.fr_to_seq { for item in fr_to_seq_vec { if !item.validate() { return false; } } }
		if let Some(ref eq_seq_vec) = self.eq_seq { for item in eq_seq_vec { if !item.validate() { return false; } } }
		if let Some(ref neq_seq_vec) = self.neq_seq { for item in neq_seq_vec { if !item.validate() { return false; } } }
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


// TimePeriodDetails1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimePeriodDetails1 {
	#[serde(rename = "FrTm")]
	pub fr_tm: String,
	#[serde(rename = "ToTm", skip_serializing_if = "Option::is_none")]
	pub to_tm: Option<String>,
}

impl TimePeriodDetails1 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// TransactionType2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionType2 {
	#[serde(rename = "Sts")]
	pub sts: EntryStatus1Choice,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: CreditDebitCode,
	#[serde(rename = "FlrLmt", skip_serializing_if = "Option::is_none")]
	pub flr_lmt: Option<Vec<Limit2>>,
}

impl TransactionType2 {
	pub fn validate(&self) -> bool {
		if !self.sts.validate() { return false }
		if !self.cdt_dbt_ind.validate() { return false }
		if let Some(ref flr_lmt_vec) = self.flr_lmt { for item in flr_lmt_vec { if !item.validate() { return false; } } }
		return true
	}
}
