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


// AccountOrBusinessError6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountOrBusinessError6Choice {
	#[serde(rename = "Acct", skip_serializing_if = "Option::is_none")]
	pub acct: Option<CashAccountData1>,
	#[serde(rename = "BizErr", skip_serializing_if = "Option::is_none")]
	pub biz_err: Option<Vec<ErrorHandling5>>,
}


// AccountOrOperationalError6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountOrOperationalError6Choice {
	#[serde(rename = "AcctRpt", skip_serializing_if = "Option::is_none")]
	pub acct_rpt: Option<Vec<AccountReport35>>,
	#[serde(rename = "OprlErr", skip_serializing_if = "Option::is_none")]
	pub oprl_err: Option<Vec<ErrorHandling5>>,
}


// AccountReport35 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountReport35 {
	#[serde(rename = "AcctId")]
	pub acct_id: AccountIdentification4Choice,
	#[serde(rename = "AcctOrErr")]
	pub acct_or_err: AccountOrBusinessError6Choice,
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
pub struct ActiveCurrencyCode {
	#[serde(rename = "$value")]
	pub active_currency_code: String,
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
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


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
}


// BICFIDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BICFIDec2014Identifier {
	#[serde(rename = "$value")]
	pub bicfi_dec2014_identifier: String,
}


// BalanceRestrictionType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BalanceRestrictionType1 {
	#[serde(rename = "Tp")]
	pub tp: GenericIdentification1,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max140Text>,
	#[serde(rename = "PrcgTp", skip_serializing_if = "Option::is_none")]
	pub prcg_tp: Option<ProcessingType1Choice>,
}


// BalanceStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BalanceStatus1Code {
	#[default]
	#[serde(rename = "PDNG")]
	CodePDNG,
	#[serde(rename = "STLD")]
	CodeSTLD,

}


// BalanceType11Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BalanceType11Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalSystemBalanceType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// BalanceType9Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BalanceType9Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<SystemBalanceType2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// BilateralLimit4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BilateralLimit4 {
	#[serde(rename = "CtrPtyId")]
	pub ctr_pty_id: BranchAndFinancialInstitutionIdentification8,
	#[serde(rename = "LmtAmt")]
	pub lmt_amt: Amount2Choice,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: CreditDebitCode,
	#[serde(rename = "BilBal", skip_serializing_if = "Option::is_none")]
	pub bil_bal: Option<Vec<CashBalance11>>,
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


// CashAccountData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountData1 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max70Text>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<CashAccountType2Choice>,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "Prxy", skip_serializing_if = "Option::is_none")]
	pub prxy: Option<ProxyAccountIdentification1>,
	#[serde(rename = "CurMulLmt", skip_serializing_if = "Option::is_none")]
	pub cur_mul_lmt: Option<Limit5>,
	#[serde(rename = "Ownr", skip_serializing_if = "Option::is_none")]
	pub ownr: Option<PartyIdentification272>,
	#[serde(rename = "Svcr", skip_serializing_if = "Option::is_none")]
	pub svcr: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "MulBal", skip_serializing_if = "Option::is_none")]
	pub mul_bal: Option<Vec<CashBalance13>>,
	#[serde(rename = "CurBilLmt", skip_serializing_if = "Option::is_none")]
	pub cur_bil_lmt: Option<Vec<BilateralLimit4>>,
	#[serde(rename = "StgOrdr", skip_serializing_if = "Option::is_none")]
	pub stg_ordr: Option<Vec<StandingOrder11>>,
}


// CashAccountType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalCashAccountType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// CashBalance11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashBalance11 {
	#[serde(rename = "Amt")]
	pub amt: f64,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: CreditDebitCode,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<BalanceType9Choice>,
	#[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
	pub sts: Option<BalanceStatus1Code>,
	#[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
	pub val_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "NbOfPmts", skip_serializing_if = "Option::is_none")]
	pub nb_of_pmts: Option<f64>,
}


// CashBalance13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashBalance13 {
	#[serde(rename = "Amt")]
	pub amt: f64,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: CreditDebitCode,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<BalanceType11Choice>,
	#[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
	pub sts: Option<BalanceStatus1Code>,
	#[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
	pub val_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "PrcgDt", skip_serializing_if = "Option::is_none")]
	pub prcg_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "NbOfPmts", skip_serializing_if = "Option::is_none")]
	pub nb_of_pmts: Option<f64>,
	#[serde(rename = "RstrctnTp", skip_serializing_if = "Option::is_none")]
	pub rstrctn_tp: Option<BalanceRestrictionType1>,
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


// DatePeriod3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriod3 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt", skip_serializing_if = "Option::is_none")]
	pub to_dt: Option<String>,
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


// EventType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EventType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalSystemEventType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "$value")]
	pub exact4_alpha_numeric_text: String,
}


// ExecutionType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExecutionType1Choice {
	#[serde(rename = "Tm", skip_serializing_if = "Option::is_none")]
	pub tm: Option<String>,
	#[serde(rename = "Evt", skip_serializing_if = "Option::is_none")]
	pub evt: Option<EventType1Choice>,
}


// ExternalAccountIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAccountIdentification1Code {
	#[serde(rename = "$value")]
	pub external_account_identification1_code: String,
}


// ExternalCashAccountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalCashAccountType1Code {
	#[serde(rename = "$value")]
	pub external_cash_account_type1_code: String,
}


// ExternalClearingSystemIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalClearingSystemIdentification1Code {
	#[serde(rename = "$value")]
	pub external_clearing_system_identification1_code: String,
}


// ExternalEnquiryRequestType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalEnquiryRequestType1Code {
	#[serde(rename = "$value")]
	pub external_enquiry_request_type1_code: String,
}


// ExternalFinancialInstitutionIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalFinancialInstitutionIdentification1Code {
	#[serde(rename = "$value")]
	pub external_financial_institution_identification1_code: String,
}


// ExternalOrganisationIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalOrganisationIdentification1Code {
	#[serde(rename = "$value")]
	pub external_organisation_identification1_code: String,
}


// ExternalPaymentControlRequestType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPaymentControlRequestType1Code {
	#[serde(rename = "$value")]
	pub external_payment_control_request_type1_code: String,
}


// ExternalPersonIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPersonIdentification1Code {
	#[serde(rename = "$value")]
	pub external_person_identification1_code: String,
}


// ExternalProxyAccountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalProxyAccountType1Code {
	#[serde(rename = "$value")]
	pub external_proxy_account_type1_code: String,
}


// ExternalSystemBalanceType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalSystemBalanceType1Code {
	#[serde(rename = "$value")]
	pub external_system_balance_type1_code: String,
}


// ExternalSystemErrorHandling1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalSystemErrorHandling1Code {
	#[serde(rename = "$value")]
	pub external_system_error_handling1_code: String,
}


// ExternalSystemEventType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalSystemEventType1Code {
	#[serde(rename = "$value")]
	pub external_system_event_type1_code: String,
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


// Frequency2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Frequency2Code {
	#[default]
	#[serde(rename = "YEAR")]
	CodeYEAR,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "QURT")]
	CodeQURT,
	#[serde(rename = "MIAN")]
	CodeMIAN,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "ADHO")]
	CodeADHO,
	#[serde(rename = "INDA")]
	CodeINDA,
	#[serde(rename = "OVNG")]
	CodeOVNG,

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
pub struct IBAN2007Identifier {
	#[serde(rename = "$value")]
	pub iban2007_identifier: String,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}


// ISOTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISOTime {
	#[serde(rename = "$value")]
	pub iso_time: String,
}


// ImpliedCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImpliedCurrencyAndAmount {
	#[serde(rename = "$value")]
	pub implied_currency_and_amount: f64,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}


// Limit5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Limit5 {
	#[serde(rename = "Amt")]
	pub amt: Amount2Choice,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: CreditDebitCode,
}


// Max128Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max128Text {
	#[serde(rename = "$value")]
	pub max128_text: String,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
}


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max16Text {
	#[serde(rename = "$value")]
	pub max16_text: String,
}


// Max2048Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max2048Text {
	#[serde(rename = "$value")]
	pub max2048_text: String,
}


// Max256Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max256Text {
	#[serde(rename = "$value")]
	pub max256_text: String,
}


// Max34Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max34Text {
	#[serde(rename = "$value")]
	pub max34_text: String,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}


// Max4Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4Text {
	#[serde(rename = "$value")]
	pub max4_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
}


// MessageHeader7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader7 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
	#[serde(rename = "ReqTp", skip_serializing_if = "Option::is_none")]
	pub req_tp: Option<RequestType4Choice>,
	#[serde(rename = "OrgnlBizQry", skip_serializing_if = "Option::is_none")]
	pub orgnl_biz_qry: Option<OriginalBusinessQuery1>,
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
pub struct Number {
	#[serde(rename = "$value")]
	pub number: f64,
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


// ProcessingType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProcessingType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ProcessingType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// ProcessingType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ProcessingType1Code {
	#[default]
	#[serde(rename = "RJCT")]
	CodeRJCT,
	#[serde(rename = "CVHD")]
	CodeCVHD,
	#[serde(rename = "RSVT")]
	CodeRSVT,
	#[serde(rename = "BLCK")]
	CodeBLCK,
	#[serde(rename = "EARM")]
	CodeEARM,
	#[serde(rename = "EFAC")]
	CodeEFAC,
	#[serde(rename = "DLVR")]
	CodeDLVR,
	#[serde(rename = "COLD")]
	CodeCOLD,
	#[serde(rename = "CSDB")]
	CodeCSDB,

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


// ReturnAccountV10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReturnAccountV10 {
	#[serde(rename = "MsgHdr")]
	pub msg_hdr: MessageHeader7,
	#[serde(rename = "RptOrErr")]
	pub rpt_or_err: AccountOrOperationalError6Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// StandingOrder11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StandingOrder11 {
	#[serde(rename = "Amt")]
	pub amt: Amount2Choice,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: CreditDebitCode,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveCurrencyCode>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<StandingOrderType1Choice>,
	#[serde(rename = "AssoctdPoolAcct", skip_serializing_if = "Option::is_none")]
	pub assoctd_pool_acct: Option<AccountIdentification4Choice>,
	#[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
	pub ref_attr: Option<Max35Text>,
	#[serde(rename = "Frqcy", skip_serializing_if = "Option::is_none")]
	pub frqcy: Option<Frequency2Code>,
	#[serde(rename = "VldtyPrd", skip_serializing_if = "Option::is_none")]
	pub vldty_prd: Option<DatePeriod3>,
	#[serde(rename = "SysMmb", skip_serializing_if = "Option::is_none")]
	pub sys_mmb: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "RspnsblPty", skip_serializing_if = "Option::is_none")]
	pub rspnsbl_pty: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "LkSetId", skip_serializing_if = "Option::is_none")]
	pub lk_set_id: Option<Max35Text>,
	#[serde(rename = "LkSetOrdrId", skip_serializing_if = "Option::is_none")]
	pub lk_set_ordr_id: Option<Max35Text>,
	#[serde(rename = "LkSetOrdrSeq", skip_serializing_if = "Option::is_none")]
	pub lk_set_ordr_seq: Option<f64>,
	#[serde(rename = "ExctnTp", skip_serializing_if = "Option::is_none")]
	pub exctn_tp: Option<ExecutionType1Choice>,
	#[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
	pub cdtr: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none")]
	pub cdtr_acct: Option<CashAccount40>,
	#[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
	pub dbtr: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none")]
	pub dbtr_acct: Option<CashAccount40>,
	#[serde(rename = "TtlsPerStgOrdr", skip_serializing_if = "Option::is_none")]
	pub ttls_per_stg_ordr: Option<StandingOrderTotalAmount1>,
	#[serde(rename = "ZeroSweepInd", skip_serializing_if = "Option::is_none")]
	pub zero_sweep_ind: Option<bool>,
}


// StandingOrderTotalAmount1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StandingOrderTotalAmount1 {
	#[serde(rename = "SetPrdfndOrdr")]
	pub set_prdfnd_ordr: TotalAmountAndCurrency1,
	#[serde(rename = "PdgPrdfndOrdr")]
	pub pdg_prdfnd_ordr: TotalAmountAndCurrency1,
	#[serde(rename = "SetStgOrdr")]
	pub set_stg_ordr: TotalAmountAndCurrency1,
	#[serde(rename = "PdgStgOrdr")]
	pub pdg_stg_ordr: TotalAmountAndCurrency1,
}


// StandingOrderType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StandingOrderType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<StandingOrderType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification1>,
}


// StandingOrderType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum StandingOrderType1Code {
	#[default]
	#[serde(rename = "USTO")]
	CodeUSTO,
	#[serde(rename = "PSTO")]
	CodePSTO,

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


// SystemBalanceType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SystemBalanceType2Code {
	#[default]
	#[serde(rename = "OPNG")]
	CodeOPNG,
	#[serde(rename = "INTM")]
	CodeINTM,
	#[serde(rename = "CLSG")]
	CodeCLSG,
	#[serde(rename = "BOOK")]
	CodeBOOK,
	#[serde(rename = "CRRT")]
	CodeCRRT,
	#[serde(rename = "PDNG")]
	CodePDNG,
	#[serde(rename = "LRLD")]
	CodeLRLD,
	#[serde(rename = "AVLB")]
	CodeAVLB,
	#[serde(rename = "LTSF")]
	CodeLTSF,
	#[serde(rename = "CRDT")]
	CodeCRDT,
	#[serde(rename = "EAST")]
	CodeEAST,
	#[serde(rename = "PYMT")]
	CodePYMT,
	#[serde(rename = "BLCK")]
	CodeBLCK,
	#[serde(rename = "XPCD")]
	CodeXPCD,
	#[serde(rename = "DLOD")]
	CodeDLOD,
	#[serde(rename = "XCRD")]
	CodeXCRD,
	#[serde(rename = "XDBT")]
	CodeXDBT,
	#[serde(rename = "ADJT")]
	CodeADJT,
	#[serde(rename = "PRAV")]
	CodePRAV,
	#[serde(rename = "DBIT")]
	CodeDBIT,
	#[serde(rename = "THRE")]
	CodeTHRE,
	#[serde(rename = "NOTE")]
	CodeNOTE,
	#[serde(rename = "FSET")]
	CodeFSET,
	#[serde(rename = "BLOC")]
	CodeBLOC,
	#[serde(rename = "OTHB")]
	CodeOTHB,
	#[serde(rename = "CUST")]
	CodeCUST,
	#[serde(rename = "FORC")]
	CodeFORC,
	#[serde(rename = "COLC")]
	CodeCOLC,
	#[serde(rename = "FUND")]
	CodeFUND,
	#[serde(rename = "PIPO")]
	CodePIPO,
	#[serde(rename = "XCHG")]
	CodeXCHG,
	#[serde(rename = "CCPS")]
	CodeCCPS,
	#[serde(rename = "TOHB")]
	CodeTOHB,
	#[serde(rename = "COHB")]
	CodeCOHB,
	#[serde(rename = "DOHB")]
	CodeDOHB,
	#[serde(rename = "TPBL")]
	CodeTPBL,
	#[serde(rename = "CPBL")]
	CodeCPBL,
	#[serde(rename = "DPBL")]
	CodeDPBL,
	#[serde(rename = "FUTB")]
	CodeFUTB,
	#[serde(rename = "REJB")]
	CodeREJB,
	#[serde(rename = "FCOL")]
	CodeFCOL,
	#[serde(rename = "FCOU")]
	CodeFCOU,
	#[serde(rename = "SCOL")]
	CodeSCOL,
	#[serde(rename = "SCOU")]
	CodeSCOU,
	#[serde(rename = "CUSA")]
	CodeCUSA,
	#[serde(rename = "XCHC")]
	CodeXCHC,
	#[serde(rename = "XCHN")]
	CodeXCHN,
	#[serde(rename = "DSET")]
	CodeDSET,
	#[serde(rename = "LACK")]
	CodeLACK,
	#[serde(rename = "NSET")]
	CodeNSET,
	#[serde(rename = "OTCC")]
	CodeOTCC,
	#[serde(rename = "OTCG")]
	CodeOTCG,
	#[serde(rename = "OTCN")]
	CodeOTCN,
	#[serde(rename = "SAPD")]
	CodeSAPD,
	#[serde(rename = "SAPC")]
	CodeSAPC,
	#[serde(rename = "REPD")]
	CodeREPD,
	#[serde(rename = "REPC")]
	CodeREPC,
	#[serde(rename = "BSCD")]
	CodeBSCD,
	#[serde(rename = "BSCC")]
	CodeBSCC,
	#[serde(rename = "SAPP")]
	CodeSAPP,
	#[serde(rename = "IRLT")]
	CodeIRLT,
	#[serde(rename = "IRDR")]
	CodeIRDR,
	#[serde(rename = "DWRD")]
	CodeDWRD,
	#[serde(rename = "ADWR")]
	CodeADWR,
	#[serde(rename = "AIDR")]
	CodeAIDR,

}


// TotalAmountAndCurrency1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TotalAmountAndCurrency1 {
	#[serde(rename = "TtlAmt")]
	pub ttl_amt: f64,
	#[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveCurrencyCode>,
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}
