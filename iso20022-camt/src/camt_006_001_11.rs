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
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountIdentification4Choice {
	#[serde(rename = "IBAN")]
	pub iban: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<GenericAccountIdentification1>,
}


// AccountSchemeName1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "ActiveCurrencyAndAmount_SimpleType")]
	pub active_currency_and_amount_simple_type: f64,
}


// ActiveCurrencyAndAmount ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveCurrencyCode ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// ActiveOrHistoricCurrencyAndAmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
	#[serde(rename = "ActiveOrHistoricCurrencyAndAmount_SimpleType")]
	pub active_or_historic_currency_and_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAndAmount ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "ActiveOrHistoricCurrencyCode")]
	pub active_or_historic_currency_code: String,
}


// AddressType2Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType2Code {
	#[serde(rename = "AddressType2Code")]
	pub address_type2_code: String,
}


// AddressType3Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// Amount2Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Amount2Choice {
	#[serde(rename = "AmtWthtCcy")]
	pub amt_wtht_ccy: Option<f64>,
	#[serde(rename = "AmtWthCcy")]
	pub amt_wth_ccy: Option<ActiveCurrencyAndAmount>,
}


// Amount3Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Amount3Choice {
	#[serde(rename = "AmtWthCcy")]
	pub amt_wth_ccy: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "AmtWthtCcy")]
	pub amt_wtht_ccy: Option<f64>,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// BICFIDec2014Identifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BICFIDec2014Identifier {
	#[serde(rename = "BICFIDec2014Identifier")]
	pub bicfi_dec2014_identifier: String,
}


// BranchAndFinancialInstitutionIdentification8 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BranchAndFinancialInstitutionIdentification8 {
	#[serde(rename = "FinInstnId")]
	pub fin_instn_id: FinancialInstitutionIdentification23,
	#[serde(rename = "BrnchId")]
	pub brnch_id: Option<BranchData5>,
}


// BranchData5 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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


// CancelledStatusReason1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CancelledStatusReason1Code {
	#[serde(rename = "CancelledStatusReason1Code")]
	pub cancelled_status_reason1_code: String,
}


// CashAccount43 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccount43 {
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
	#[serde(rename = "Ownr")]
	pub ownr: Option<PartyIdentification272>,
	#[serde(rename = "Svcr")]
	pub svcr: Option<BranchAndFinancialInstitutionIdentification8>,
}


// CashAccountAndEntry5 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountAndEntry5 {
	#[serde(rename = "Acct")]
	pub acct: CashAccount43,
	#[serde(rename = "Ntry")]
	pub ntry: Option<CashEntry2>,
}


// CashAccountType2Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountType2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// CashEntry2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashEntry2 {
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Dt")]
	pub dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "Sts")]
	pub sts: Option<String>,
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "StmtId")]
	pub stmt_id: Option<String>,
	#[serde(rename = "AcctSvcrRef")]
	pub acct_svcr_ref: Option<f64>,
	#[serde(rename = "AddtlNtryInf")]
	pub addtl_ntry_inf: Option<Vec<String>>,
}


// ClearingSystemIdentification2Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemIdentification2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ClearingSystemMemberIdentification2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemMemberIdentification2 {
	#[serde(rename = "ClrSysId")]
	pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
	#[serde(rename = "MmbId")]
	pub mmb_id: String,
}


// Contact13 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// CreditDebitCode ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditDebitCode {
	#[serde(rename = "CreditDebitCode")]
	pub credit_debit_code: String,
}


// DateAndDateTime2Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTime2Choice {
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm")]
	pub dt_tm: Option<String>,
}


// DateAndPlaceOfBirth1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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


// DateTimePeriod1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod1 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: String,
}


// DateTimePeriod1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod1Choice {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: Option<String>,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: Option<String>,
	#[serde(rename = "DtTmRg")]
	pub dt_tm_rg: Option<DateTimePeriod1>,
}


// DecimalNumber ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "DecimalNumber")]
	pub decimal_number: f64,
}


// EntryStatus1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EntryStatus1Code {
	#[serde(rename = "EntryStatus1Code")]
	pub entry_status1_code: String,
}


// EntryTypeIdentifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EntryTypeIdentifier {
	#[serde(rename = "EntryTypeIdentifier")]
	pub entry_type_identifier: String,
}


// ErrorHandling3Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ErrorHandling5 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling5 {
	#[serde(rename = "Err")]
	pub err: ErrorHandling3Choice,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// Exact4AlphaNumericText ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "Exact4AlphaNumericText")]
	pub exact4_alpha_numeric_text: String,
}


// ExternalAccountIdentification1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAccountIdentification1Code {
	#[serde(rename = "ExternalAccountIdentification1Code")]
	pub external_account_identification1_code: String,
}


// ExternalCashAccountType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalCashAccountType1Code {
	#[serde(rename = "ExternalCashAccountType1Code")]
	pub external_cash_account_type1_code: String,
}


// ExternalClearingSystemIdentification1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalClearingSystemIdentification1Code {
	#[serde(rename = "ExternalClearingSystemIdentification1Code")]
	pub external_clearing_system_identification1_code: String,
}


// ExternalEnquiryRequestType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalEnquiryRequestType1Code {
	#[serde(rename = "ExternalEnquiryRequestType1Code")]
	pub external_enquiry_request_type1_code: String,
}


// ExternalFinancialInstitutionIdentification1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalFinancialInstitutionIdentification1Code {
	#[serde(rename = "ExternalFinancialInstitutionIdentification1Code")]
	pub external_financial_institution_identification1_code: String,
}


// ExternalMarketInfrastructure1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalMarketInfrastructure1Code {
	#[serde(rename = "ExternalMarketInfrastructure1Code")]
	pub external_market_infrastructure1_code: String,
}


// ExternalOrganisationIdentification1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalOrganisationIdentification1Code {
	#[serde(rename = "ExternalOrganisationIdentification1Code")]
	pub external_organisation_identification1_code: String,
}


// ExternalPaymentControlRequestType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPaymentControlRequestType1Code {
	#[serde(rename = "ExternalPaymentControlRequestType1Code")]
	pub external_payment_control_request_type1_code: String,
}


// ExternalPersonIdentification1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPersonIdentification1Code {
	#[serde(rename = "ExternalPersonIdentification1Code")]
	pub external_person_identification1_code: String,
}


// ExternalProxyAccountType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalProxyAccountType1Code {
	#[serde(rename = "ExternalProxyAccountType1Code")]
	pub external_proxy_account_type1_code: String,
}


// ExternalSystemErrorHandling1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalSystemErrorHandling1Code {
	#[serde(rename = "ExternalSystemErrorHandling1Code")]
	pub external_system_error_handling1_code: String,
}


// FinalStatus1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinalStatus1Code {
	#[serde(rename = "FinalStatus1Code")]
	pub final_status1_code: String,
}


// FinancialIdentificationSchemeName1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// FinancialInstitutionIdentification23 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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


// GenericAccountIdentification1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericAccountIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<AccountSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericFinancialIdentification1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericFinancialIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<FinancialIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericIdentification1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericIdentification30 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification30 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// GenericOrganisationIdentification3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericOrganisationIdentification3 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericPersonIdentification2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericPersonIdentification2 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// IBAN2007Identifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IBAN2007Identifier {
	#[serde(rename = "IBAN2007Identifier")]
	pub iban2007_identifier: String,
}


// ISODate ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// ImpliedCurrencyAndAmount ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImpliedCurrencyAndAmount {
	#[serde(rename = "ImpliedCurrencyAndAmount")]
	pub implied_currency_and_amount: f64,
}


// LEIIdentifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LongPaymentIdentification4 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LongPaymentIdentification4 {
	#[serde(rename = "TxId")]
	pub tx_id: Option<String>,
	#[serde(rename = "UETR")]
	pub uetr: Option<String>,
	#[serde(rename = "IntrBkSttlmAmt")]
	pub intr_bk_sttlm_amt: f64,
	#[serde(rename = "IntrBkSttlmDt")]
	pub intr_bk_sttlm_dt: String,
	#[serde(rename = "PmtMtd")]
	pub pmt_mtd: Option<PaymentOrigin1Choice>,
	#[serde(rename = "InstgAgt")]
	pub instg_agt: BranchAndFinancialInstitutionIdentification8,
	#[serde(rename = "InstdAgt")]
	pub instd_agt: BranchAndFinancialInstitutionIdentification8,
	#[serde(rename = "NtryTp")]
	pub ntry_tp: Option<String>,
	#[serde(rename = "EndToEndId")]
	pub end_to_end_id: Option<String>,
}


// MarketInfrastructureIdentification1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketInfrastructureIdentification1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// Max10Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max10Text {
	#[serde(rename = "Max10Text")]
	pub max10_text: String,
}


// Max128Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max128Text {
	#[serde(rename = "Max128Text")]
	pub max128_text: String,
}


// Max140Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max15NumericText ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max15NumericText {
	#[serde(rename = "Max15NumericText")]
	pub max15_numeric_text: String,
}


// Max16Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max16Text {
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
}


// Max20000Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max20000Text {
	#[serde(rename = "Max20000Text")]
	pub max20000_text: String,
}


// Max2048Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max2048Text {
	#[serde(rename = "Max2048Text")]
	pub max2048_text: String,
}


// Max256Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max256Text {
	#[serde(rename = "Max256Text")]
	pub max256_text: String,
}


// Max34Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max34Text {
	#[serde(rename = "Max34Text")]
	pub max34_text: String,
}


// Max350Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Max3NumericText ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max3NumericText {
	#[serde(rename = "Max3NumericText")]
	pub max3_numeric_text: String,
}


// Max4AlphaNumericText ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "Max4AlphaNumericText")]
	pub max4_alpha_numeric_text: String,
}


// Max4Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4Text {
	#[serde(rename = "Max4Text")]
	pub max4_text: String,
}


// Max5NumericText ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max5NumericText {
	#[serde(rename = "Max5NumericText")]
	pub max5_numeric_text: String,
}


// Max70Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// MessageHeader8 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader8 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: Option<String>,
	#[serde(rename = "MsgPgntn")]
	pub msg_pgntn: Option<Pagination1>,
	#[serde(rename = "OrgnlBizQry")]
	pub orgnl_biz_qry: Option<OriginalBusinessQuery1>,
	#[serde(rename = "ReqTp")]
	pub req_tp: Option<RequestType4Choice>,
	#[serde(rename = "QryNm")]
	pub qry_nm: Option<String>,
}


// NamePrefix2Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NamePrefix2Code {
	#[serde(rename = "NamePrefix2Code")]
	pub name_prefix2_code: String,
}


// Number ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
}


// NumberAndSumOfTransactions2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NumberAndSumOfTransactions2 {
	#[serde(rename = "NbOfNtries")]
	pub nb_of_ntries: Option<String>,
	#[serde(rename = "Sum")]
	pub sum: Option<f64>,
	#[serde(rename = "TtlNetNtryAmt")]
	pub ttl_net_ntry_amt: Option<f64>,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: Option<String>,
}


// OrganisationIdentification39 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification39 {
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<GenericOrganisationIdentification3>>,
}


// OrganisationIdentificationSchemeName1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// OriginalBusinessQuery1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OriginalBusinessQuery1 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "MsgNmId")]
	pub msg_nm_id: Option<String>,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: Option<String>,
}


// OtherContact1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherContact1 {
	#[serde(rename = "ChanlTp")]
	pub chanl_tp: String,
	#[serde(rename = "Id")]
	pub id: Option<String>,
}


// Pagination1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Pagination1 {
	#[serde(rename = "PgNb")]
	pub pg_nb: String,
	#[serde(rename = "LastPgInd")]
	pub last_pg_ind: bool,
}


// Party50Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party50Choice {
	#[serde(rename = "Pty")]
	pub pty: Option<PartyIdentification272>,
	#[serde(rename = "Agt")]
	pub agt: Option<BranchAndFinancialInstitutionIdentification8>,
}


// Party52Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party52Choice {
	#[serde(rename = "OrgId")]
	pub org_id: Option<OrganisationIdentification39>,
	#[serde(rename = "PrvtId")]
	pub prvt_id: Option<PersonIdentification18>,
}


// PartyIdentification272 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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


// PaymentCommon6 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentCommon6 {
	#[serde(rename = "PmtFr")]
	pub pmt_fr: Option<System3>,
	#[serde(rename = "PmtTo")]
	pub pmt_to: Option<System3>,
	#[serde(rename = "CmonSts")]
	pub cmon_sts: Option<Vec<PaymentStatus6>>,
	#[serde(rename = "ReqdExctnDt")]
	pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "NtryDt")]
	pub ntry_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: Option<String>,
	#[serde(rename = "PmtMtd")]
	pub pmt_mtd: Option<PaymentOrigin1Choice>,
}


// PaymentIdentification8Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentIdentification8Choice {
	#[serde(rename = "TxId")]
	pub tx_id: Option<String>,
	#[serde(rename = "UETR")]
	pub uetr: Option<String>,
	#[serde(rename = "QId")]
	pub q_id: Option<QueueTransactionIdentification1>,
	#[serde(rename = "LngBizId")]
	pub lng_biz_id: Option<LongPaymentIdentification4>,
	#[serde(rename = "ShrtBizId")]
	pub shrt_biz_id: Option<ShortPaymentIdentification4>,
	#[serde(rename = "PrtryId")]
	pub prtry_id: Option<String>,
}


// PaymentInstruction47 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentInstruction47 {
	#[serde(rename = "MsgId")]
	pub msg_id: Option<String>,
	#[serde(rename = "ReqdExctnDt")]
	pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "Sts")]
	pub sts: Option<Vec<PaymentStatus6>>,
	#[serde(rename = "InstdAmt")]
	pub instd_amt: Option<Amount3Choice>,
	#[serde(rename = "IntrBkSttlmAmt")]
	pub intr_bk_sttlm_amt: Option<Amount2Choice>,
	#[serde(rename = "Purp")]
	pub purp: Option<String>,
	#[serde(rename = "PmtMtd")]
	pub pmt_mtd: Option<PaymentOrigin1Choice>,
	#[serde(rename = "Prty")]
	pub prty: Option<Priority1Choice>,
	#[serde(rename = "PrcgVldtyTm")]
	pub prcg_vldty_tm: Option<DateTimePeriod1Choice>,
	#[serde(rename = "InstrCpy")]
	pub instr_cpy: Option<String>,
	#[serde(rename = "Tp")]
	pub tp: Option<PaymentType4Choice>,
	#[serde(rename = "GnrtdOrdr")]
	pub gnrtd_ordr: Option<bool>,
	#[serde(rename = "TxId")]
	pub tx_id: Option<String>,
	#[serde(rename = "IntrBkSttlmDt")]
	pub intr_bk_sttlm_dt: Option<String>,
	#[serde(rename = "EndToEndId")]
	pub end_to_end_id: Option<String>,
	#[serde(rename = "Pties")]
	pub pties: Option<PaymentTransactionParty4>,
}


// PaymentInstrument1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentInstrument1Code {
	#[serde(rename = "PaymentInstrument1Code")]
	pub payment_instrument1_code: String,
}


// PaymentOrigin1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentOrigin1Choice {
	#[serde(rename = "FINMT")]
	pub finmt: Option<String>,
	#[serde(rename = "XMLMsgNm")]
	pub xml_msg_nm: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
	#[serde(rename = "Instrm")]
	pub instrm: Option<String>,
}


// PaymentStatus6 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentStatus6 {
	#[serde(rename = "Cd")]
	pub cd: Option<PaymentStatusCode6Choice>,
	#[serde(rename = "DtTm")]
	pub dt_tm: Option<DateAndDateTime2Choice>,
	#[serde(rename = "Rsn")]
	pub rsn: Option<Vec<PaymentStatusReason1Choice>>,
}


// PaymentStatusCode6Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentStatusCode6Choice {
	#[serde(rename = "Pdg")]
	pub pdg: Option<String>,
	#[serde(rename = "Fnl")]
	pub fnl: Option<String>,
	#[serde(rename = "RTGS")]
	pub rtgs: Option<String>,
	#[serde(rename = "Sttlm")]
	pub sttlm: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// PaymentStatusReason1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentStatusReason1Choice {
	#[serde(rename = "Umtchd")]
	pub umtchd: Option<String>,
	#[serde(rename = "Canc")]
	pub canc: Option<String>,
	#[serde(rename = "Sspd")]
	pub sspd: Option<String>,
	#[serde(rename = "PdgFlngSttlm")]
	pub pdg_flng_sttlm: Option<String>,
	#[serde(rename = "PdgSttlm")]
	pub pdg_sttlm: Option<String>,
	#[serde(rename = "PrtryRjctn")]
	pub prtry_rjctn: Option<ProprietaryStatusJustification2>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// PaymentTransactionParty4 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentTransactionParty4 {
	#[serde(rename = "InstgAgt")]
	pub instg_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "InstdAgt")]
	pub instd_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "UltmtDbtr")]
	pub ultmt_dbtr: Option<Party50Choice>,
	#[serde(rename = "Dbtr")]
	pub dbtr: Option<Party50Choice>,
	#[serde(rename = "DbtrAgt")]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "InstgRmbrsmntAgt")]
	pub instg_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "InstdRmbrsmntAgt")]
	pub instd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "IntrmyAgt1")]
	pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "IntrmyAgt2")]
	pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "IntrmyAgt3")]
	pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "CdtrAgt")]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "Cdtr")]
	pub cdtr: Option<Party50Choice>,
	#[serde(rename = "UltmtCdtr")]
	pub ultmt_cdtr: Option<Party50Choice>,
}


// PaymentType3Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentType3Code {
	#[serde(rename = "PaymentType3Code")]
	pub payment_type3_code: String,
}


// PaymentType4Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentType4Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// PendingFailingSettlement1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingFailingSettlement1Code {
	#[serde(rename = "PendingFailingSettlement1Code")]
	pub pending_failing_settlement1_code: String,
}


// PendingSettlement2Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingSettlement2Code {
	#[serde(rename = "PendingSettlement2Code")]
	pub pending_settlement2_code: String,
}


// PendingStatus4Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingStatus4Code {
	#[serde(rename = "PendingStatus4Code")]
	pub pending_status4_code: String,
}


// PersonIdentification18 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentification18 {
	#[serde(rename = "DtAndPlcOfBirth")]
	pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<GenericPersonIdentification2>>,
}


// PersonIdentificationSchemeName1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// PhoneNumber ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PhoneNumber {
	#[serde(rename = "PhoneNumber")]
	pub phone_number: String,
}


// PostalAddress27 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PreferredContactMethod2Code {
	#[serde(rename = "PreferredContactMethod2Code")]
	pub preferred_contact_method2_code: String,
}


// Priority1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Priority1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// Priority5Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Priority5Code {
	#[serde(rename = "Priority5Code")]
	pub priority5_code: String,
}


// ProprietaryStatusJustification2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryStatusJustification2 {
	#[serde(rename = "PrtryStsRsn")]
	pub prtry_sts_rsn: String,
	#[serde(rename = "Rsn")]
	pub rsn: String,
}


// ProxyAccountIdentification1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProxyAccountIdentification1 {
	#[serde(rename = "Tp")]
	pub tp: Option<ProxyAccountType1Choice>,
	#[serde(rename = "Id")]
	pub id: String,
}


// ProxyAccountType1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProxyAccountType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// QueueTransactionIdentification1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct QueueTransactionIdentification1 {
	#[serde(rename = "QId")]
	pub q_id: String,
	#[serde(rename = "PosInQ")]
	pub pos_in_q: String,
}


// RequestType4Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestType4Choice {
	#[serde(rename = "PmtCtrl")]
	pub pmt_ctrl: Option<String>,
	#[serde(rename = "Enqry")]
	pub enqry: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification1>,
}


// ReturnTransactionV11 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReturnTransactionV11 {
	#[serde(rename = "MsgHdr")]
	pub msg_hdr: MessageHeader8,
	#[serde(rename = "RptOrErr")]
	pub rpt_or_err: TransactionReportOrError7Choice,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// SecuritiesTransactionReferences1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionReferences1 {
	#[serde(rename = "AcctOwnrTxId")]
	pub acct_ownr_tx_id: Option<String>,
	#[serde(rename = "AcctSvcrTxId")]
	pub acct_svcr_tx_id: Option<String>,
	#[serde(rename = "MktInfrstrctrTxId")]
	pub mkt_infrstrctr_tx_id: Option<String>,
	#[serde(rename = "PrcgId")]
	pub prcg_id: Option<String>,
}


// ShortPaymentIdentification4 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ShortPaymentIdentification4 {
	#[serde(rename = "TxId")]
	pub tx_id: Option<String>,
	#[serde(rename = "UETR")]
	pub uetr: Option<String>,
	#[serde(rename = "IntrBkSttlmDt")]
	pub intr_bk_sttlm_dt: String,
	#[serde(rename = "InstgAgt")]
	pub instg_agt: BranchAndFinancialInstitutionIdentification8,
}


// SupplementaryData1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Option<String>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}


// SuspendedStatusReason1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SuspendedStatusReason1Code {
	#[serde(rename = "SuspendedStatusReason1Code")]
	pub suspended_status_reason1_code: String,
}


// System3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct System3 {
	#[serde(rename = "SysId")]
	pub sys_id: Option<MarketInfrastructureIdentification1Choice>,
	#[serde(rename = "MmbId")]
	pub mmb_id: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
	#[serde(rename = "AcctId")]
	pub acct_id: Option<AccountIdentification4Choice>,
}


// Transaction159 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Transaction159 {
	#[serde(rename = "PmtTo")]
	pub pmt_to: Option<System3>,
	#[serde(rename = "PmtFr")]
	pub pmt_fr: Option<System3>,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: Option<String>,
	#[serde(rename = "Pmt")]
	pub pmt: Option<PaymentInstruction47>,
	#[serde(rename = "AcctNtry")]
	pub acct_ntry: Option<CashAccountAndEntry5>,
	#[serde(rename = "SctiesTxRefs")]
	pub scties_tx_refs: Option<SecuritiesTransactionReferences1>,
}


// TransactionOrError6Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionOrError6Choice {
	#[serde(rename = "Tx")]
	pub tx: Option<Transaction159>,
	#[serde(rename = "BizErr")]
	pub biz_err: Option<Vec<ErrorHandling5>>,
}


// TransactionReport8 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionReport8 {
	#[serde(rename = "PmtId")]
	pub pmt_id: PaymentIdentification8Choice,
	#[serde(rename = "TxOrErr")]
	pub tx_or_err: TransactionOrError6Choice,
}


// TransactionReportOrError7Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionReportOrError7Choice {
	#[serde(rename = "BizRpt")]
	pub biz_rpt: Option<Transactions11>,
	#[serde(rename = "OprlErr")]
	pub oprl_err: Option<Vec<ErrorHandling5>>,
}


// Transactions11 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Transactions11 {
	#[serde(rename = "PmtCmonInf")]
	pub pmt_cmon_inf: Option<PaymentCommon6>,
	#[serde(rename = "TxsSummry")]
	pub txs_summry: Option<NumberAndSumOfTransactions2>,
	#[serde(rename = "TxRpt")]
	pub tx_rpt: Vec<TransactionReport8>,
}


// TrueFalseIndicator ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}


// UUIDv4Identifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UUIDv4Identifier {
	#[serde(rename = "UUIDv4Identifier")]
	pub uui_dv4_identifier: String,
}


// UnmatchedStatusReason1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnmatchedStatusReason1Code {
	#[serde(rename = "UnmatchedStatusReason1Code")]
	pub unmatched_status_reason1_code: String,
}


// YesNoIndicator ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "YesNoIndicator")]
	pub yes_no_indicator: bool,
}
