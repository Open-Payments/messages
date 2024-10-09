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
	#[serde(rename = "IBAN")]
	pub iban: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<GenericAccountIdentification1>,
}


// AccountOrBusinessError6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountOrBusinessError6Choice {
	#[serde(rename = "Acct")]
	pub acct: Option<CashAccountData1>,
	#[serde(rename = "BizErr")]
	pub biz_err: Option<Vec<ErrorHandling5>>,
}


// AccountOrOperationalError6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountOrOperationalError6Choice {
	#[serde(rename = "AcctRpt")]
	pub acct_rpt: Option<Vec<AccountReport35>>,
	#[serde(rename = "OprlErr")]
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
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "ActiveCurrencyAndAmount_SimpleType")]
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
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "ActiveOrHistoricCurrencyCode")]
	pub active_or_historic_currency_code: String,
}


// AddressType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType2Code {
	#[serde(rename = "AddressType2Code")]
	pub address_type2_code: String,
}


// AddressType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// Amount2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Amount2Choice {
	#[serde(rename = "AmtWthtCcy")]
	pub amt_wtht_ccy: Option<f64>,
	#[serde(rename = "AmtWthCcy")]
	pub amt_wth_ccy: Option<ActiveCurrencyAndAmount>,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// BICFIDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BICFIDec2014Identifier {
	#[serde(rename = "BICFIDec2014Identifier")]
	pub bicfi_dec2014_identifier: String,
}


// BalanceRestrictionType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BalanceRestrictionType1 {
	#[serde(rename = "Tp")]
	pub tp: GenericIdentification1,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
	#[serde(rename = "PrcgTp")]
	pub prcg_tp: Option<ProcessingType1Choice>,
}


// BalanceStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BalanceStatus1Code {
	#[serde(rename = "BalanceStatus1Code")]
	pub balance_status1_code: String,
}


// BalanceType11Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BalanceType11Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// BalanceType9Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BalanceType9Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// BilateralLimit4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BilateralLimit4 {
	#[serde(rename = "CtrPtyId")]
	pub ctr_pty_id: BranchAndFinancialInstitutionIdentification8,
	#[serde(rename = "LmtAmt")]
	pub lmt_amt: Amount2Choice,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: String,
	#[serde(rename = "BilBal")]
	pub bil_bal: Option<Vec<CashBalance11>>,
}


// BranchAndFinancialInstitutionIdentification8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BranchAndFinancialInstitutionIdentification8 {
	#[serde(rename = "FinInstnId")]
	pub fin_instn_id: FinancialInstitutionIdentification23,
	#[serde(rename = "BrnchId")]
	pub brnch_id: Option<BranchData5>,
}


// BranchData5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BranchData5 {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress27>,
}


// CashAccount40 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccount40 {
	#[serde(rename = "Id")]
	pub id: Option<AccountIdentification4Choice>,
	#[serde(rename = "Tp")]
	pub tp: Option<CashAccountType2Choice>,
	#[serde(rename = "Ccy")]
	pub ccy: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Prxy")]
	pub prxy: Option<ProxyAccountIdentification1>,
}


// CashAccountData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountData1 {
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Tp")]
	pub tp: Option<CashAccountType2Choice>,
	#[serde(rename = "Ccy")]
	pub ccy: Option<String>,
	#[serde(rename = "Prxy")]
	pub prxy: Option<ProxyAccountIdentification1>,
	#[serde(rename = "CurMulLmt")]
	pub cur_mul_lmt: Option<Limit5>,
	#[serde(rename = "Ownr")]
	pub ownr: Option<PartyIdentification272>,
	#[serde(rename = "Svcr")]
	pub svcr: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "MulBal")]
	pub mul_bal: Option<Vec<CashBalance13>>,
	#[serde(rename = "CurBilLmt")]
	pub cur_bil_lmt: Option<Vec<BilateralLimit4>>,
	#[serde(rename = "StgOrdr")]
	pub stg_ordr: Option<Vec<StandingOrder11>>,
}


// CashAccountType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountType2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// CashBalance11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashBalance11 {
	#[serde(rename = "Amt")]
	pub amt: f64,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: String,
	#[serde(rename = "Tp")]
	pub tp: Option<BalanceType9Choice>,
	#[serde(rename = "Sts")]
	pub sts: Option<String>,
	#[serde(rename = "ValDt")]
	pub val_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "NbOfPmts")]
	pub nb_of_pmts: Option<f64>,
}


// CashBalance13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashBalance13 {
	#[serde(rename = "Amt")]
	pub amt: f64,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: String,
	#[serde(rename = "Tp")]
	pub tp: Option<BalanceType11Choice>,
	#[serde(rename = "Sts")]
	pub sts: Option<String>,
	#[serde(rename = "ValDt")]
	pub val_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "PrcgDt")]
	pub prcg_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "NbOfPmts")]
	pub nb_of_pmts: Option<f64>,
	#[serde(rename = "RstrctnTp")]
	pub rstrctn_tp: Option<BalanceRestrictionType1>,
}


// ClearingSystemIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemIdentification2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ClearingSystemMemberIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemMemberIdentification2 {
	#[serde(rename = "ClrSysId")]
	pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
	#[serde(rename = "MmbId")]
	pub mmb_id: String,
}


// Contact13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Contact13 {
	#[serde(rename = "NmPrfx")]
	pub nm_prfx: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "PhneNb")]
	pub phne_nb: Option<String>,
	#[serde(rename = "MobNb")]
	pub mob_nb: Option<String>,
	#[serde(rename = "FaxNb")]
	pub fax_nb: Option<String>,
	#[serde(rename = "URLAdr")]
	pub url_adr: Option<String>,
	#[serde(rename = "EmailAdr")]
	pub email_adr: Option<String>,
	#[serde(rename = "EmailPurp")]
	pub email_purp: Option<String>,
	#[serde(rename = "JobTitl")]
	pub job_titl: Option<String>,
	#[serde(rename = "Rspnsblty")]
	pub rspnsblty: Option<String>,
	#[serde(rename = "Dept")]
	pub dept: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<OtherContact1>>,
	#[serde(rename = "PrefrdMtd")]
	pub prefrd_mtd: Option<String>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// CreditDebitCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditDebitCode {
	#[serde(rename = "CreditDebitCode")]
	pub credit_debit_code: String,
}


// DateAndDateTime2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTime2Choice {
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm")]
	pub dt_tm: Option<String>,
}


// DateAndPlaceOfBirth1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndPlaceOfBirth1 {
	#[serde(rename = "BirthDt")]
	pub birth_dt: String,
	#[serde(rename = "PrvcOfBirth")]
	pub prvc_of_birth: Option<String>,
	#[serde(rename = "CityOfBirth")]
	pub city_of_birth: String,
	#[serde(rename = "CtryOfBirth")]
	pub ctry_of_birth: String,
}


// DatePeriod3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriod3 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: Option<String>,
}


// ErrorHandling3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ErrorHandling5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling5 {
	#[serde(rename = "Err")]
	pub err: ErrorHandling3Choice,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// EventType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EventType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "Exact4AlphaNumericText")]
	pub exact4_alpha_numeric_text: String,
}


// ExecutionType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExecutionType1Choice {
	#[serde(rename = "Tm")]
	pub tm: Option<String>,
	#[serde(rename = "Evt")]
	pub evt: Option<EventType1Choice>,
}


// ExternalAccountIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAccountIdentification1Code {
	#[serde(rename = "ExternalAccountIdentification1Code")]
	pub external_account_identification1_code: String,
}


// ExternalCashAccountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalCashAccountType1Code {
	#[serde(rename = "ExternalCashAccountType1Code")]
	pub external_cash_account_type1_code: String,
}


// ExternalClearingSystemIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalClearingSystemIdentification1Code {
	#[serde(rename = "ExternalClearingSystemIdentification1Code")]
	pub external_clearing_system_identification1_code: String,
}


// ExternalEnquiryRequestType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalEnquiryRequestType1Code {
	#[serde(rename = "ExternalEnquiryRequestType1Code")]
	pub external_enquiry_request_type1_code: String,
}


// ExternalFinancialInstitutionIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalFinancialInstitutionIdentification1Code {
	#[serde(rename = "ExternalFinancialInstitutionIdentification1Code")]
	pub external_financial_institution_identification1_code: String,
}


// ExternalOrganisationIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalOrganisationIdentification1Code {
	#[serde(rename = "ExternalOrganisationIdentification1Code")]
	pub external_organisation_identification1_code: String,
}


// ExternalPaymentControlRequestType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPaymentControlRequestType1Code {
	#[serde(rename = "ExternalPaymentControlRequestType1Code")]
	pub external_payment_control_request_type1_code: String,
}


// ExternalPersonIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPersonIdentification1Code {
	#[serde(rename = "ExternalPersonIdentification1Code")]
	pub external_person_identification1_code: String,
}


// ExternalProxyAccountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalProxyAccountType1Code {
	#[serde(rename = "ExternalProxyAccountType1Code")]
	pub external_proxy_account_type1_code: String,
}


// ExternalSystemBalanceType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalSystemBalanceType1Code {
	#[serde(rename = "ExternalSystemBalanceType1Code")]
	pub external_system_balance_type1_code: String,
}


// ExternalSystemErrorHandling1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalSystemErrorHandling1Code {
	#[serde(rename = "ExternalSystemErrorHandling1Code")]
	pub external_system_error_handling1_code: String,
}


// ExternalSystemEventType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalSystemEventType1Code {
	#[serde(rename = "ExternalSystemEventType1Code")]
	pub external_system_event_type1_code: String,
}


// FinancialIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// FinancialInstitutionIdentification23 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstitutionIdentification23 {
	#[serde(rename = "BICFI")]
	pub bicfi: Option<String>,
	#[serde(rename = "ClrSysMmbId")]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress27>,
	#[serde(rename = "Othr")]
	pub othr: Option<GenericFinancialIdentification1>,
}


// Frequency2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Frequency2Code {
	#[serde(rename = "Frequency2Code")]
	pub frequency2_code: String,
}


// GenericAccountIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericAccountIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<AccountSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericFinancialIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericFinancialIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<FinancialIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericIdentification30 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification30 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// GenericOrganisationIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericOrganisationIdentification3 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericPersonIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericPersonIdentification2 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// IBAN2007Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IBAN2007Identifier {
	#[serde(rename = "IBAN2007Identifier")]
	pub iban2007_identifier: String,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// ISOTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISOTime {
	#[serde(rename = "ISOTime")]
	pub iso_time: String,
}


// ImpliedCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImpliedCurrencyAndAmount {
	#[serde(rename = "ImpliedCurrencyAndAmount")]
	pub implied_currency_and_amount: f64,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// Limit5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Limit5 {
	#[serde(rename = "Amt")]
	pub amt: Amount2Choice,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: String,
}


// Max128Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max128Text {
	#[serde(rename = "Max128Text")]
	pub max128_text: String,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max16Text {
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
}


// Max2048Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max2048Text {
	#[serde(rename = "Max2048Text")]
	pub max2048_text: String,
}


// Max256Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max256Text {
	#[serde(rename = "Max256Text")]
	pub max256_text: String,
}


// Max34Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max34Text {
	#[serde(rename = "Max34Text")]
	pub max34_text: String,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Max4Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4Text {
	#[serde(rename = "Max4Text")]
	pub max4_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// MessageHeader7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader7 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: Option<String>,
	#[serde(rename = "ReqTp")]
	pub req_tp: Option<RequestType4Choice>,
	#[serde(rename = "OrgnlBizQry")]
	pub orgnl_biz_qry: Option<OriginalBusinessQuery1>,
	#[serde(rename = "QryNm")]
	pub qry_nm: Option<String>,
}


// NamePrefix2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NamePrefix2Code {
	#[serde(rename = "NamePrefix2Code")]
	pub name_prefix2_code: String,
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
}


// OrganisationIdentification39 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification39 {
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<GenericOrganisationIdentification3>>,
}


// OrganisationIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// OriginalBusinessQuery1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OriginalBusinessQuery1 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "MsgNmId")]
	pub msg_nm_id: Option<String>,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: Option<String>,
}


// OtherContact1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherContact1 {
	#[serde(rename = "ChanlTp")]
	pub chanl_tp: String,
	#[serde(rename = "Id")]
	pub id: Option<String>,
}


// Party52Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party52Choice {
	#[serde(rename = "OrgId")]
	pub org_id: Option<OrganisationIdentification39>,
	#[serde(rename = "PrvtId")]
	pub prvt_id: Option<PersonIdentification18>,
}


// PartyIdentification272 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification272 {
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress27>,
	#[serde(rename = "Id")]
	pub id: Option<Party52Choice>,
	#[serde(rename = "CtryOfRes")]
	pub ctry_of_res: Option<String>,
	#[serde(rename = "CtctDtls")]
	pub ctct_dtls: Option<Contact13>,
}


// PersonIdentification18 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentification18 {
	#[serde(rename = "DtAndPlcOfBirth")]
	pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<GenericPersonIdentification2>>,
}


// PersonIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// PhoneNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PhoneNumber {
	#[serde(rename = "PhoneNumber")]
	pub phone_number: String,
}


// PostalAddress27 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress27 {
	#[serde(rename = "AdrTp")]
	pub adr_tp: Option<AddressType3Choice>,
	#[serde(rename = "CareOf")]
	pub care_of: Option<String>,
	#[serde(rename = "Dept")]
	pub dept: Option<String>,
	#[serde(rename = "SubDept")]
	pub sub_dept: Option<String>,
	#[serde(rename = "StrtNm")]
	pub strt_nm: Option<String>,
	#[serde(rename = "BldgNb")]
	pub bldg_nb: Option<String>,
	#[serde(rename = "BldgNm")]
	pub bldg_nm: Option<String>,
	#[serde(rename = "Flr")]
	pub flr: Option<String>,
	#[serde(rename = "UnitNb")]
	pub unit_nb: Option<String>,
	#[serde(rename = "PstBx")]
	pub pst_bx: Option<String>,
	#[serde(rename = "Room")]
	pub room: Option<String>,
	#[serde(rename = "PstCd")]
	pub pst_cd: Option<String>,
	#[serde(rename = "TwnNm")]
	pub twn_nm: Option<String>,
	#[serde(rename = "TwnLctnNm")]
	pub twn_lctn_nm: Option<String>,
	#[serde(rename = "DstrctNm")]
	pub dstrct_nm: Option<String>,
	#[serde(rename = "CtrySubDvsn")]
	pub ctry_sub_dvsn: Option<String>,
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
	#[serde(rename = "AdrLine")]
	pub adr_line: Option<Vec<String>>,
}


// PreferredContactMethod2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PreferredContactMethod2Code {
	#[serde(rename = "PreferredContactMethod2Code")]
	pub preferred_contact_method2_code: String,
}


// ProcessingType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProcessingType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ProcessingType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProcessingType1Code {
	#[serde(rename = "ProcessingType1Code")]
	pub processing_type1_code: String,
}


// ProxyAccountIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProxyAccountIdentification1 {
	#[serde(rename = "Tp")]
	pub tp: Option<ProxyAccountType1Choice>,
	#[serde(rename = "Id")]
	pub id: String,
}


// ProxyAccountType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProxyAccountType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// RequestType4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestType4Choice {
	#[serde(rename = "PmtCtrl")]
	pub pmt_ctrl: Option<String>,
	#[serde(rename = "Enqry")]
	pub enqry: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification1>,
}


// ReturnAccountV10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReturnAccountV10 {
	#[serde(rename = "MsgHdr")]
	pub msg_hdr: MessageHeader7,
	#[serde(rename = "RptOrErr")]
	pub rpt_or_err: AccountOrOperationalError6Choice,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// StandingOrder11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StandingOrder11 {
	#[serde(rename = "Amt")]
	pub amt: Amount2Choice,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: String,
	#[serde(rename = "Ccy")]
	pub ccy: Option<String>,
	#[serde(rename = "Tp")]
	pub tp: Option<StandingOrderType1Choice>,
	#[serde(rename = "AssoctdPoolAcct")]
	pub assoctd_pool_acct: Option<AccountIdentification4Choice>,
	#[serde(rename = "Ref")]
	pub ref_attr: Option<String>,
	#[serde(rename = "Frqcy")]
	pub frqcy: Option<String>,
	#[serde(rename = "VldtyPrd")]
	pub vldty_prd: Option<DatePeriod3>,
	#[serde(rename = "SysMmb")]
	pub sys_mmb: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "RspnsblPty")]
	pub rspnsbl_pty: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "LkSetId")]
	pub lk_set_id: Option<String>,
	#[serde(rename = "LkSetOrdrId")]
	pub lk_set_ordr_id: Option<String>,
	#[serde(rename = "LkSetOrdrSeq")]
	pub lk_set_ordr_seq: Option<f64>,
	#[serde(rename = "ExctnTp")]
	pub exctn_tp: Option<ExecutionType1Choice>,
	#[serde(rename = "Cdtr")]
	pub cdtr: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "CdtrAcct")]
	pub cdtr_acct: Option<CashAccount40>,
	#[serde(rename = "Dbtr")]
	pub dbtr: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "DbtrAcct")]
	pub dbtr_acct: Option<CashAccount40>,
	#[serde(rename = "TtlsPerStgOrdr")]
	pub ttls_per_stg_ordr: Option<StandingOrderTotalAmount1>,
	#[serde(rename = "ZeroSweepInd")]
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
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification1>,
}


// StandingOrderType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StandingOrderType1Code {
	#[serde(rename = "StandingOrderType1Code")]
	pub standing_order_type1_code: String,
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Option<String>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}


// SystemBalanceType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemBalanceType2Code {
	#[serde(rename = "SystemBalanceType2Code")]
	pub system_balance_type2_code: String,
}


// TotalAmountAndCurrency1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TotalAmountAndCurrency1 {
	#[serde(rename = "TtlAmt")]
	pub ttl_amt: f64,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: Option<String>,
	#[serde(rename = "Ccy")]
	pub ccy: Option<String>,
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}
