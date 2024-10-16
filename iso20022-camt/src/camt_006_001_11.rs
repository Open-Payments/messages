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


// AccountIdentification4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountIdentification4Choice {
	#[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
	pub iban: Option<IBAN2007Identifier>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<GenericAccountIdentification1>,
}


// AccountSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalAccountIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_currency_and_amount_simple_type: f64,
}


// ActiveCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "$value")]
	pub active_currency_code: String,
}


// ActiveOrHistoricCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_and_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_code: String,
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


// AddressType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<AddressType2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// Amount2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Amount2Choice {
	#[serde(rename = "AmtWthtCcy", skip_serializing_if = "Option::is_none")]
	pub amt_wtht_ccy: Option<f64>,
	#[serde(rename = "AmtWthCcy", skip_serializing_if = "Option::is_none")]
	pub amt_wth_ccy: Option<ActiveCurrencyAndAmount>,
}


// Amount3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Amount3Choice {
	#[serde(rename = "AmtWthCcy", skip_serializing_if = "Option::is_none")]
	pub amt_wth_ccy: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "AmtWthtCcy", skip_serializing_if = "Option::is_none")]
	pub amt_wtht_ccy: Option<f64>,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
}


// BICFIDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BICFIDec2014Identifier {
	#[serde(rename = "$value")]
	pub bicfi_dec2014_identifier: String,
}


// BranchAndFinancialInstitutionIdentification8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BranchAndFinancialInstitutionIdentification8 {
	#[serde(rename = "FinInstnId")]
	pub fin_instn_id: FinancialInstitutionIdentification23,
	#[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
	pub brnch_id: Option<BranchData5>,
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


// CashAccountAndEntry5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountAndEntry5 {
	#[serde(rename = "Acct")]
	pub acct: CashAccount43,
	#[serde(rename = "Ntry", skip_serializing_if = "Option::is_none")]
	pub ntry: Option<CashEntry2>,
}


// CashAccountType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalCashAccountType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
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


// ClearingSystemIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemIdentification2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalClearingSystemIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// ClearingSystemMemberIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemMemberIdentification2 {
	#[serde(rename = "ClrSysId", skip_serializing_if = "Option::is_none")]
	pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
	#[serde(rename = "MmbId")]
	pub mmb_id: Max35Text,
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


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
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


// DateAndDateTime2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTime2Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<String>,
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


// DateTimePeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod1 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: String,
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


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DecimalNumber {
	#[serde(rename = "$value")]
	pub decimal_number: f64,
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


// EntryTypeIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EntryTypeIdentifier {
	#[serde(rename = "$value")]
	pub entry_type_identifier: String,
}


// ErrorHandling3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalSystemErrorHandling1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// ErrorHandling5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling5 {
	#[serde(rename = "Err")]
	pub err: ErrorHandling3Choice,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max140Text>,
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "$value")]
	pub exact4_alpha_numeric_text: String,
}


// ExternalAccountIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalAccountIdentification1Code {
	#[serde(rename = "$value")]
	pub external_account_identification1_code: String,
}


// ExternalCashAccountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalCashAccountType1Code {
	#[serde(rename = "$value")]
	pub external_cash_account_type1_code: String,
}


// ExternalClearingSystemIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalClearingSystemIdentification1Code {
	#[serde(rename = "$value")]
	pub external_clearing_system_identification1_code: String,
}


// ExternalEnquiryRequestType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalEnquiryRequestType1Code {
	#[serde(rename = "$value")]
	pub external_enquiry_request_type1_code: String,
}


// ExternalFinancialInstitutionIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalFinancialInstitutionIdentification1Code {
	#[serde(rename = "$value")]
	pub external_financial_institution_identification1_code: String,
}


// ExternalMarketInfrastructure1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalMarketInfrastructure1Code {
	#[serde(rename = "$value")]
	pub external_market_infrastructure1_code: String,
}


// ExternalOrganisationIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalOrganisationIdentification1Code {
	#[serde(rename = "$value")]
	pub external_organisation_identification1_code: String,
}


// ExternalPaymentControlRequestType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalPaymentControlRequestType1Code {
	#[serde(rename = "$value")]
	pub external_payment_control_request_type1_code: String,
}


// ExternalPersonIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalPersonIdentification1Code {
	#[serde(rename = "$value")]
	pub external_person_identification1_code: String,
}


// ExternalProxyAccountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalProxyAccountType1Code {
	#[serde(rename = "$value")]
	pub external_proxy_account_type1_code: String,
}


// ExternalSystemErrorHandling1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalSystemErrorHandling1Code {
	#[serde(rename = "$value")]
	pub external_system_error_handling1_code: String,
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


// FinancialIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalFinancialInstitutionIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
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


// IBAN2007Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct IBAN2007Identifier {
	#[serde(rename = "$value")]
	pub iban2007_identifier: String,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}


// ImpliedCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ImpliedCurrencyAndAmount {
	#[serde(rename = "$value")]
	pub implied_currency_and_amount: f64,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
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


// MarketInfrastructureIdentification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketInfrastructureIdentification1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalMarketInfrastructure1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// Max10Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max10Text {
	#[serde(rename = "$value")]
	pub max10_text: String,
}


// Max128Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max128Text {
	#[serde(rename = "$value")]
	pub max128_text: String,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
}


// Max15NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max15NumericText {
	#[serde(rename = "$value")]
	pub max15_numeric_text: String,
}


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max16Text {
	#[serde(rename = "$value")]
	pub max16_text: String,
}


// Max20000Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max20000Text {
	#[serde(rename = "$value")]
	pub max20000_text: String,
}


// Max2048Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max2048Text {
	#[serde(rename = "$value")]
	pub max2048_text: String,
}


// Max256Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max256Text {
	#[serde(rename = "$value")]
	pub max256_text: String,
}


// Max34Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max34Text {
	#[serde(rename = "$value")]
	pub max34_text: String,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}


// Max3NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max3NumericText {
	#[serde(rename = "$value")]
	pub max3_numeric_text: String,
}


// Max4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "$value")]
	pub max4_alpha_numeric_text: String,
}


// Max4Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max4Text {
	#[serde(rename = "$value")]
	pub max4_text: String,
}


// Max5NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max5NumericText {
	#[serde(rename = "$value")]
	pub max5_numeric_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
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


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Number {
	#[serde(rename = "$value")]
	pub number: f64,
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


// OrganisationIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalOrganisationIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
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


// OtherContact1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherContact1 {
	#[serde(rename = "ChanlTp")]
	pub chanl_tp: Max4Text,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max128Text>,
}


// Pagination1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Pagination1 {
	#[serde(rename = "PgNb")]
	pub pg_nb: Max5NumericText,
	#[serde(rename = "LastPgInd")]
	pub last_pg_ind: bool,
}


// Party50Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party50Choice {
	#[serde(rename = "Pty", skip_serializing_if = "Option::is_none")]
	pub pty: Option<PartyIdentification272>,
	#[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
	pub agt: Option<BranchAndFinancialInstitutionIdentification8>,
}


// Party52Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party52Choice {
	#[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
	pub org_id: Option<OrganisationIdentification39>,
	#[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
	pub prvt_id: Option<PersonIdentification18>,
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


// PaymentType4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentType4Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PaymentType3Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
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


// PersonIdentification18 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentification18 {
	#[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
	pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<GenericPersonIdentification2>>,
}


// PersonIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalPersonIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// PhoneNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PhoneNumber {
	#[serde(rename = "$value")]
	pub phone_number: String,
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


// Priority1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Priority1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Priority5Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
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


// ProprietaryStatusJustification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryStatusJustification2 {
	#[serde(rename = "PrtryStsRsn")]
	pub prtry_sts_rsn: Max4AlphaNumericText,
	#[serde(rename = "Rsn")]
	pub rsn: Max256Text,
}


// ProxyAccountIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProxyAccountIdentification1 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<ProxyAccountType1Choice>,
	#[serde(rename = "Id")]
	pub id: Max2048Text,
}


// ProxyAccountType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProxyAccountType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalProxyAccountType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// QueueTransactionIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct QueueTransactionIdentification1 {
	#[serde(rename = "QId")]
	pub q_id: Max16Text,
	#[serde(rename = "PosInQ")]
	pub pos_in_q: Max16Text,
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


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
	pub plc_and_nm: Option<Max350Text>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
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


// TransactionOrError6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionOrError6Choice {
	#[serde(rename = "Tx", skip_serializing_if = "Option::is_none")]
	pub tx: Option<Transaction159>,
	#[serde(rename = "BizErr", skip_serializing_if = "Option::is_none")]
	pub biz_err: Option<Vec<ErrorHandling5>>,
}


// TransactionReport8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionReport8 {
	#[serde(rename = "PmtId")]
	pub pmt_id: PaymentIdentification8Choice,
	#[serde(rename = "TxOrErr")]
	pub tx_or_err: TransactionOrError6Choice,
}


// TransactionReportOrError7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionReportOrError7Choice {
	#[serde(rename = "BizRpt", skip_serializing_if = "Option::is_none")]
	pub biz_rpt: Option<Transactions11>,
	#[serde(rename = "OprlErr", skip_serializing_if = "Option::is_none")]
	pub oprl_err: Option<Vec<ErrorHandling5>>,
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


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}


// UUIDv4Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UUIDv4Identifier {
	#[serde(rename = "$value")]
	pub uui_dv4_identifier: String,
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


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct YesNoIndicator {
	#[serde(rename = "$value")]
	pub yes_no_indicator: bool,
}
