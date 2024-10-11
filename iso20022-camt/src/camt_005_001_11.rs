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


// AccountCashEntryReturnCriteria3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountCashEntryReturnCriteria3 {
	#[serde(rename = "NtryRefInd", skip_serializing_if = "Option::is_none")]
	pub ntry_ref_ind: Option<bool>,
	#[serde(rename = "AcctTpInd", skip_serializing_if = "Option::is_none")]
	pub acct_tp_ind: Option<bool>,
	#[serde(rename = "NtryAmtInd", skip_serializing_if = "Option::is_none")]
	pub ntry_amt_ind: Option<bool>,
	#[serde(rename = "AcctCcyInd", skip_serializing_if = "Option::is_none")]
	pub acct_ccy_ind: Option<bool>,
	#[serde(rename = "NtryStsInd", skip_serializing_if = "Option::is_none")]
	pub ntry_sts_ind: Option<bool>,
	#[serde(rename = "NtryDtInd", skip_serializing_if = "Option::is_none")]
	pub ntry_dt_ind: Option<bool>,
	#[serde(rename = "AcctSvcrInd", skip_serializing_if = "Option::is_none")]
	pub acct_svcr_ind: Option<bool>,
	#[serde(rename = "AcctOwnrInd", skip_serializing_if = "Option::is_none")]
	pub acct_ownr_ind: Option<bool>,
}


// AccountIdentification4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountIdentification4Choice {
	#[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
	pub iban: Option<String>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<GenericAccountIdentification1>,
}


// AccountIdentificationSearchCriteria2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountIdentificationSearchCriteria2Choice {
	#[serde(rename = "EQ", skip_serializing_if = "Option::is_none")]
	pub eq: Option<AccountIdentification4Choice>,
	#[serde(rename = "CTTxt", skip_serializing_if = "Option::is_none")]
	pub ct_txt: Option<String>,
	#[serde(rename = "NCTTxt", skip_serializing_if = "Option::is_none")]
	pub nct_txt: Option<String>,
}


// AccountSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// ActiveAmountRange3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveAmountRange3Choice {
	#[serde(rename = "ImpldCcyAndAmtRg", skip_serializing_if = "Option::is_none")]
	pub impld_ccy_and_amt_rg: Option<ImpliedCurrencyAndAmountRange1>,
	#[serde(rename = "CcyAndAmtRg", skip_serializing_if = "Option::is_none")]
	pub ccy_and_amt_rg: Option<ActiveCurrencyAndAmountRange3>,
}


// ActiveCurrencyAndAmountRange3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmountRange3 {
	#[serde(rename = "Amt")]
	pub amt: ImpliedCurrencyAmountRange1Choice,
	#[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt_ind: Option<String>,
	#[serde(rename = "Ccy")]
	pub ccy: String,
}


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// ActiveOrHistoricAmountRange2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricAmountRange2Choice {
	#[serde(rename = "ImpldCcyAndAmtRg", skip_serializing_if = "Option::is_none")]
	pub impld_ccy_and_amt_rg: Option<ImpliedCurrencyAndAmountRange1>,
	#[serde(rename = "CcyAndAmtRg", skip_serializing_if = "Option::is_none")]
	pub ccy_and_amt_rg: Option<ActiveOrHistoricCurrencyAndAmountRange2>,
}


// ActiveOrHistoricCurrencyAndAmountRange2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmountRange2 {
	#[serde(rename = "Amt")]
	pub amt: ImpliedCurrencyAmountRange1Choice,
	#[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt_ind: Option<String>,
	#[serde(rename = "Ccy")]
	pub ccy: String,
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
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// AmountRangeBoundary1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountRangeBoundary1 {
	#[serde(rename = "BdryAmt")]
	pub bdry_amt: f64,
	#[serde(rename = "Incl")]
	pub incl: bool,
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
	pub id: Option<String>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<String>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress27>,
}


// CashAccountEntrySearch8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountEntrySearch8 {
	#[serde(rename = "AcctId", skip_serializing_if = "Option::is_none")]
	pub acct_id: Option<Vec<AccountIdentificationSearchCriteria2Choice>>,
	#[serde(rename = "NtryAmt", skip_serializing_if = "Option::is_none")]
	pub ntry_amt: Option<Vec<ActiveOrHistoricAmountRange2Choice>>,
	#[serde(rename = "NtryAmtCcy", skip_serializing_if = "Option::is_none")]
	pub ntry_amt_ccy: Option<Vec<String>>,
	#[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt_ind: Option<String>,
	#[serde(rename = "NtrySts", skip_serializing_if = "Option::is_none")]
	pub ntry_sts: Option<Vec<String>>,
	#[serde(rename = "NtryDt", skip_serializing_if = "Option::is_none")]
	pub ntry_dt: Option<Vec<DateAndDateTimeSearch3Choice>>,
	#[serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none")]
	pub acct_ownr: Option<PartyIdentification272>,
	#[serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none")]
	pub acct_svcr: Option<BranchAndFinancialInstitutionIdentification8>,
}


// CashPaymentStatus2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashPaymentStatus2Code {
	#[serde(rename = "CashPaymentStatus2Code")]
	pub cash_payment_status2_code: String,
}


// ClearingSystemIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemIdentification2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// ClearingSystemIdentification3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemIdentification3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// ClearingSystemMemberIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemMemberIdentification2 {
	#[serde(rename = "ClrSysId", skip_serializing_if = "Option::is_none")]
	pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
	#[serde(rename = "MmbId")]
	pub mmb_id: String,
}


// Contact13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Contact13 {
	#[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
	pub nm_prfx: Option<String>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
	#[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
	pub phne_nb: Option<String>,
	#[serde(rename = "MobNb", skip_serializing_if = "Option::is_none")]
	pub mob_nb: Option<String>,
	#[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
	pub fax_nb: Option<String>,
	#[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
	pub url_adr: Option<String>,
	#[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
	pub email_adr: Option<String>,
	#[serde(rename = "EmailPurp", skip_serializing_if = "Option::is_none")]
	pub email_purp: Option<String>,
	#[serde(rename = "JobTitl", skip_serializing_if = "Option::is_none")]
	pub job_titl: Option<String>,
	#[serde(rename = "Rspnsblty", skip_serializing_if = "Option::is_none")]
	pub rspnsblty: Option<String>,
	#[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
	pub dept: Option<String>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<OtherContact1>>,
	#[serde(rename = "PrefrdMtd", skip_serializing_if = "Option::is_none")]
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


// DateAndDateTimeSearch3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTimeSearch3Choice {
	#[serde(rename = "DtTmSch", skip_serializing_if = "Option::is_none")]
	pub dt_tm_sch: Option<DateTimePeriod1Choice>,
	#[serde(rename = "DtSch", skip_serializing_if = "Option::is_none")]
	pub dt_sch: Option<DatePeriodSearch1Choice>,
}


// DateAndPlaceOfBirth1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndPlaceOfBirth1 {
	#[serde(rename = "BirthDt")]
	pub birth_dt: String,
	#[serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none")]
	pub prvc_of_birth: Option<String>,
	#[serde(rename = "CityOfBirth")]
	pub city_of_birth: String,
	#[serde(rename = "CtryOfBirth")]
	pub ctry_of_birth: String,
}


// DatePeriod2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriod2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// DatePeriodSearch1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriodSearch1Choice {
	#[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt", skip_serializing_if = "Option::is_none")]
	pub to_dt: Option<String>,
	#[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
	pub fr_to_dt: Option<DatePeriod2>,
	#[serde(rename = "EQDt", skip_serializing_if = "Option::is_none")]
	pub eq_dt: Option<String>,
	#[serde(rename = "NEQDt", skip_serializing_if = "Option::is_none")]
	pub neq_dt: Option<String>,
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


// EntryStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EntryStatus1Code {
	#[serde(rename = "EntryStatus1Code")]
	pub entry_status1_code: String,
}


// EntryTypeIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EntryTypeIdentifier {
	#[serde(rename = "EntryTypeIdentifier")]
	pub entry_type_identifier: String,
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "Exact4AlphaNumericText")]
	pub exact4_alpha_numeric_text: String,
}


// ExternalAccountIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAccountIdentification1Code {
	#[serde(rename = "ExternalAccountIdentification1Code")]
	pub external_account_identification1_code: String,
}


// ExternalCashClearingSystem1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalCashClearingSystem1Code {
	#[serde(rename = "ExternalCashClearingSystem1Code")]
	pub external_cash_clearing_system1_code: String,
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


// FinalStatusCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinalStatusCode {
	#[serde(rename = "FinalStatusCode")]
	pub final_status_code: String,
}


// FinancialIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// FinancialInstitutionIdentification23 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstitutionIdentification23 {
	#[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
	pub bicfi: Option<String>,
	#[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<String>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress27>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<GenericFinancialIdentification1>,
}


// FromToAmountRange1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FromToAmountRange1 {
	#[serde(rename = "FrAmt")]
	pub fr_amt: AmountRangeBoundary1,
	#[serde(rename = "ToAmt")]
	pub to_amt: AmountRangeBoundary1,
}


// GenericAccountIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericAccountIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<AccountSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}


// GenericFinancialIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericFinancialIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<FinancialIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}


// GenericIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}


// GenericIdentification30 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification30 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<String>,
}


// GenericOrganisationIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericOrganisationIdentification3 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}


// GenericPersonIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericPersonIdentification2 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}


// GetTransactionV11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GetTransactionV11 {
	#[serde(rename = "MsgHdr")]
	pub msg_hdr: MessageHeader9,
	#[serde(rename = "TxQryDef", skip_serializing_if = "Option::is_none")]
	pub tx_qry_def: Option<TransactionQuery8>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
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


// ImpliedCurrencyAmountRange1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImpliedCurrencyAmountRange1Choice {
	#[serde(rename = "FrAmt", skip_serializing_if = "Option::is_none")]
	pub fr_amt: Option<AmountRangeBoundary1>,
	#[serde(rename = "ToAmt", skip_serializing_if = "Option::is_none")]
	pub to_amt: Option<AmountRangeBoundary1>,
	#[serde(rename = "FrToAmt", skip_serializing_if = "Option::is_none")]
	pub fr_to_amt: Option<FromToAmountRange1>,
	#[serde(rename = "EQAmt", skip_serializing_if = "Option::is_none")]
	pub eq_amt: Option<f64>,
	#[serde(rename = "NEQAmt", skip_serializing_if = "Option::is_none")]
	pub neq_amt: Option<f64>,
}


// ImpliedCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImpliedCurrencyAndAmount {
	#[serde(rename = "ImpliedCurrencyAndAmount")]
	pub implied_currency_and_amount: f64,
}


// ImpliedCurrencyAndAmountRange1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImpliedCurrencyAndAmountRange1 {
	#[serde(rename = "Amt")]
	pub amt: ImpliedCurrencyAmountRange1Choice,
	#[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt_ind: Option<String>,
}


// Instruction1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Instruction1Code {
	#[serde(rename = "Instruction1Code")]
	pub instruction1_code: String,
}


// InstructionStatusReturnCriteria1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InstructionStatusReturnCriteria1 {
	#[serde(rename = "PmtInstrStsInd")]
	pub pmt_instr_sts_ind: bool,
	#[serde(rename = "PmtInstrStsDtTmInd", skip_serializing_if = "Option::is_none")]
	pub pmt_instr_sts_dt_tm_ind: Option<bool>,
	#[serde(rename = "PmtInstrStsRsnInd", skip_serializing_if = "Option::is_none")]
	pub pmt_instr_sts_rsn_ind: Option<bool>,
}


// InstructionStatusSearch5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InstructionStatusSearch5 {
	#[serde(rename = "PmtInstrSts", skip_serializing_if = "Option::is_none")]
	pub pmt_instr_sts: Option<PaymentStatusCodeSearch2Choice>,
	#[serde(rename = "PmtInstrStsDtTm", skip_serializing_if = "Option::is_none")]
	pub pmt_instr_sts_dt_tm: Option<DateTimePeriod1Choice>,
	#[serde(rename = "PrtryStsRsn", skip_serializing_if = "Option::is_none")]
	pub prtry_sts_rsn: Option<String>,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LongPaymentIdentification4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LongPaymentIdentification4 {
	#[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
	pub tx_id: Option<String>,
	#[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
	pub uetr: Option<String>,
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
	pub ntry_tp: Option<String>,
	#[serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none")]
	pub end_to_end_id: Option<String>,
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


// Max3NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max3NumericText {
	#[serde(rename = "Max3NumericText")]
	pub max3_numeric_text: String,
}


// Max4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "Max4AlphaNumericText")]
	pub max4_alpha_numeric_text: String,
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


// MessageHeader9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader9 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
	#[serde(rename = "ReqTp", skip_serializing_if = "Option::is_none")]
	pub req_tp: Option<RequestType4Choice>,
}


// NamePrefix2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NamePrefix2Code {
	#[serde(rename = "NamePrefix2Code")]
	pub name_prefix2_code: String,
}


// OrganisationIdentification39 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification39 {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<String>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<String>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<GenericOrganisationIdentification3>>,
}


// OrganisationIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// OtherContact1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherContact1 {
	#[serde(rename = "ChanlTp")]
	pub chanl_tp: String,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
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
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress27>,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Party52Choice>,
	#[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
	pub ctry_of_res: Option<String>,
	#[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
	pub ctct_dtls: Option<Contact13>,
}


// PaymentIdentification8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentIdentification8Choice {
	#[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
	pub tx_id: Option<String>,
	#[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
	pub uetr: Option<String>,
	#[serde(rename = "QId", skip_serializing_if = "Option::is_none")]
	pub q_id: Option<QueueTransactionIdentification1>,
	#[serde(rename = "LngBizId", skip_serializing_if = "Option::is_none")]
	pub lng_biz_id: Option<LongPaymentIdentification4>,
	#[serde(rename = "ShrtBizId", skip_serializing_if = "Option::is_none")]
	pub shrt_biz_id: Option<ShortPaymentIdentification4>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<String>,
}


// PaymentInstrument1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentInstrument1Code {
	#[serde(rename = "PaymentInstrument1Code")]
	pub payment_instrument1_code: String,
}


// PaymentOrigin1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentOrigin1Choice {
	#[serde(rename = "FINMT", skip_serializing_if = "Option::is_none")]
	pub finmt: Option<String>,
	#[serde(rename = "XMLMsgNm", skip_serializing_if = "Option::is_none")]
	pub xml_msg_nm: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
	#[serde(rename = "Instrm", skip_serializing_if = "Option::is_none")]
	pub instrm: Option<String>,
}


// PaymentReturnCriteria4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentReturnCriteria4 {
	#[serde(rename = "MsgIdInd", skip_serializing_if = "Option::is_none")]
	pub msg_id_ind: Option<bool>,
	#[serde(rename = "ReqdExctnDtInd", skip_serializing_if = "Option::is_none")]
	pub reqd_exctn_dt_ind: Option<bool>,
	#[serde(rename = "InstrInd", skip_serializing_if = "Option::is_none")]
	pub instr_ind: Option<bool>,
	#[serde(rename = "InstrStsRtrCrit", skip_serializing_if = "Option::is_none")]
	pub instr_sts_rtr_crit: Option<InstructionStatusReturnCriteria1>,
	#[serde(rename = "InstdAmtInd", skip_serializing_if = "Option::is_none")]
	pub instd_amt_ind: Option<bool>,
	#[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt_ind: Option<bool>,
	#[serde(rename = "IntrBkSttlmAmtInd", skip_serializing_if = "Option::is_none")]
	pub intr_bk_sttlm_amt_ind: Option<bool>,
	#[serde(rename = "PrtyInd", skip_serializing_if = "Option::is_none")]
	pub prty_ind: Option<bool>,
	#[serde(rename = "PrcgVldtyTmInd", skip_serializing_if = "Option::is_none")]
	pub prcg_vldty_tm_ind: Option<bool>,
	#[serde(rename = "PurpInd", skip_serializing_if = "Option::is_none")]
	pub purp_ind: Option<bool>,
	#[serde(rename = "InstrCpyInd", skip_serializing_if = "Option::is_none")]
	pub instr_cpy_ind: Option<bool>,
	#[serde(rename = "PmtMTInd", skip_serializing_if = "Option::is_none")]
	pub pmt_mt_ind: Option<bool>,
	#[serde(rename = "PmtTpInd", skip_serializing_if = "Option::is_none")]
	pub pmt_tp_ind: Option<bool>,
	#[serde(rename = "TxIdInd", skip_serializing_if = "Option::is_none")]
	pub tx_id_ind: Option<bool>,
	#[serde(rename = "IntrBkSttlmDtInd", skip_serializing_if = "Option::is_none")]
	pub intr_bk_sttlm_dt_ind: Option<bool>,
	#[serde(rename = "EndToEndIdInd", skip_serializing_if = "Option::is_none")]
	pub end_to_end_id_ind: Option<bool>,
	#[serde(rename = "PmtMtdInd", skip_serializing_if = "Option::is_none")]
	pub pmt_mtd_ind: Option<bool>,
	#[serde(rename = "DbtrInd", skip_serializing_if = "Option::is_none")]
	pub dbtr_ind: Option<bool>,
	#[serde(rename = "DbtrAgtInd", skip_serializing_if = "Option::is_none")]
	pub dbtr_agt_ind: Option<bool>,
	#[serde(rename = "InstgRmbrsmntAgtInd", skip_serializing_if = "Option::is_none")]
	pub instg_rmbrsmnt_agt_ind: Option<bool>,
	#[serde(rename = "InstdRmbrsmntAgtInd", skip_serializing_if = "Option::is_none")]
	pub instd_rmbrsmnt_agt_ind: Option<bool>,
	#[serde(rename = "IntrmyInd", skip_serializing_if = "Option::is_none")]
	pub intrmy_ind: Option<bool>,
	#[serde(rename = "CdtrAgtInd", skip_serializing_if = "Option::is_none")]
	pub cdtr_agt_ind: Option<bool>,
	#[serde(rename = "CdtrInd", skip_serializing_if = "Option::is_none")]
	pub cdtr_ind: Option<bool>,
}


// PaymentSearch10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentSearch10 {
	#[serde(rename = "MsgId", skip_serializing_if = "Option::is_none")]
	pub msg_id: Option<Vec<String>>,
	#[serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none")]
	pub reqd_exctn_dt: Option<Vec<DateAndDateTimeSearch3Choice>>,
	#[serde(rename = "PmtId", skip_serializing_if = "Option::is_none")]
	pub pmt_id: Option<Vec<PaymentIdentification8Choice>>,
	#[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
	pub sts: Option<Vec<InstructionStatusSearch5>>,
	#[serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none")]
	pub instd_amt: Option<Vec<ActiveOrHistoricAmountRange2Choice>>,
	#[serde(rename = "InstdAmtCcy", skip_serializing_if = "Option::is_none")]
	pub instd_amt_ccy: Option<Vec<String>>,
	#[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt_ind: Option<String>,
	#[serde(rename = "IntrBkSttlmAmt", skip_serializing_if = "Option::is_none")]
	pub intr_bk_sttlm_amt: Option<Vec<ActiveAmountRange3Choice>>,
	#[serde(rename = "IntrBkSttlmAmtCcy", skip_serializing_if = "Option::is_none")]
	pub intr_bk_sttlm_amt_ccy: Option<Vec<String>>,
	#[serde(rename = "PmtMtd", skip_serializing_if = "Option::is_none")]
	pub pmt_mtd: Option<Vec<PaymentOrigin1Choice>>,
	#[serde(rename = "PmtTp", skip_serializing_if = "Option::is_none")]
	pub pmt_tp: Option<Vec<PaymentType4Choice>>,
	#[serde(rename = "Prty", skip_serializing_if = "Option::is_none")]
	pub prty: Option<Vec<Priority1Choice>>,
	#[serde(rename = "PrcgVldtyTm", skip_serializing_if = "Option::is_none")]
	pub prcg_vldty_tm: Option<Vec<DateTimePeriod1Choice>>,
	#[serde(rename = "Instr", skip_serializing_if = "Option::is_none")]
	pub instr: Option<Vec<String>>,
	#[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
	pub tx_id: Option<Vec<String>>,
	#[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
	pub uetr: Option<Vec<String>>,
	#[serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
	pub intr_bk_sttlm_dt: Option<Vec<String>>,
	#[serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none")]
	pub end_to_end_id: Option<Vec<String>>,
	#[serde(rename = "Pties", skip_serializing_if = "Option::is_none")]
	pub pties: Option<PaymentTransactionParty4>,
}


// PaymentStatusCodeSearch2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentStatusCodeSearch2Choice {
	#[serde(rename = "PdgSts", skip_serializing_if = "Option::is_none")]
	pub pdg_sts: Option<String>,
	#[serde(rename = "FnlSts", skip_serializing_if = "Option::is_none")]
	pub fnl_sts: Option<String>,
	#[serde(rename = "PdgAndFnlSts", skip_serializing_if = "Option::is_none")]
	pub pdg_and_fnl_sts: Option<String>,
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
pub struct PaymentType3Code {
	#[serde(rename = "PaymentType3Code")]
	pub payment_type3_code: String,
}


// PaymentType4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentType4Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// PendingStatus4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingStatus4Code {
	#[serde(rename = "PendingStatus4Code")]
	pub pending_status4_code: String,
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
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
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
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<AddressType3Choice>,
	#[serde(rename = "CareOf", skip_serializing_if = "Option::is_none")]
	pub care_of: Option<String>,
	#[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
	pub dept: Option<String>,
	#[serde(rename = "SubDept", skip_serializing_if = "Option::is_none")]
	pub sub_dept: Option<String>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<String>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<String>,
	#[serde(rename = "BldgNm", skip_serializing_if = "Option::is_none")]
	pub bldg_nm: Option<String>,
	#[serde(rename = "Flr", skip_serializing_if = "Option::is_none")]
	pub flr: Option<String>,
	#[serde(rename = "UnitNb", skip_serializing_if = "Option::is_none")]
	pub unit_nb: Option<String>,
	#[serde(rename = "PstBx", skip_serializing_if = "Option::is_none")]
	pub pst_bx: Option<String>,
	#[serde(rename = "Room", skip_serializing_if = "Option::is_none")]
	pub room: Option<String>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<String>,
	#[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
	pub twn_nm: Option<String>,
	#[serde(rename = "TwnLctnNm", skip_serializing_if = "Option::is_none")]
	pub twn_lctn_nm: Option<String>,
	#[serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none")]
	pub dstrct_nm: Option<String>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<String>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<String>,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
	pub adr_line: Option<Vec<String>>,
}


// PreferredContactMethod2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PreferredContactMethod2Code {
	#[serde(rename = "PreferredContactMethod2Code")]
	pub preferred_contact_method2_code: String,
}


// Priority1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Priority1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// Priority5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Priority5Code {
	#[serde(rename = "Priority5Code")]
	pub priority5_code: String,
}


// QueryType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct QueryType2Code {
	#[serde(rename = "QueryType2Code")]
	pub query_type2_code: String,
}


// QueueTransactionIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct QueueTransactionIdentification1 {
	#[serde(rename = "QId")]
	pub q_id: String,
	#[serde(rename = "PosInQ")]
	pub pos_in_q: String,
}


// ReportIndicator1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportIndicator1Code {
	#[serde(rename = "ReportIndicator1Code")]
	pub report_indicator1_code: String,
}


// RequestType4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestType4Choice {
	#[serde(rename = "PmtCtrl", skip_serializing_if = "Option::is_none")]
	pub pmt_ctrl: Option<String>,
	#[serde(rename = "Enqry", skip_serializing_if = "Option::is_none")]
	pub enqry: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification1>,
}


// RequestedIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestedIndicator {
	#[serde(rename = "RequestedIndicator")]
	pub requested_indicator: bool,
}


// ShortPaymentIdentification4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ShortPaymentIdentification4 {
	#[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
	pub tx_id: Option<String>,
	#[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
	pub uetr: Option<String>,
	#[serde(rename = "IntrBkSttlmDt")]
	pub intr_bk_sttlm_dt: String,
	#[serde(rename = "InstgAgt")]
	pub instg_agt: BranchAndFinancialInstitutionIdentification8,
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
	pub plc_and_nm: Option<String>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}


// SystemReturnCriteria2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemReturnCriteria2 {
	#[serde(rename = "SysIdInd", skip_serializing_if = "Option::is_none")]
	pub sys_id_ind: Option<bool>,
	#[serde(rename = "MmbIdInd", skip_serializing_if = "Option::is_none")]
	pub mmb_id_ind: Option<bool>,
	#[serde(rename = "CtryIdInd", skip_serializing_if = "Option::is_none")]
	pub ctry_id_ind: Option<bool>,
	#[serde(rename = "AcctIdInd", skip_serializing_if = "Option::is_none")]
	pub acct_id_ind: Option<bool>,
}


// SystemSearch5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemSearch5 {
	#[serde(rename = "SysId", skip_serializing_if = "Option::is_none")]
	pub sys_id: Option<Vec<ClearingSystemIdentification3Choice>>,
	#[serde(rename = "MmbId", skip_serializing_if = "Option::is_none")]
	pub mmb_id: Option<Vec<BranchAndFinancialInstitutionIdentification8>>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<String>,
	#[serde(rename = "AcctId", skip_serializing_if = "Option::is_none")]
	pub acct_id: Option<AccountIdentification4Choice>,
}


// TransactionCriteria11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionCriteria11 {
	#[serde(rename = "NewQryNm", skip_serializing_if = "Option::is_none")]
	pub new_qry_nm: Option<String>,
	#[serde(rename = "SchCrit", skip_serializing_if = "Option::is_none")]
	pub sch_crit: Option<Vec<TransactionSearchCriteria11>>,
	#[serde(rename = "StmtRpt", skip_serializing_if = "Option::is_none")]
	pub stmt_rpt: Option<String>,
	#[serde(rename = "RtrCrit", skip_serializing_if = "Option::is_none")]
	pub rtr_crit: Option<TransactionReturnCriteria5>,
}


// TransactionCriteria8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionCriteria8Choice {
	#[serde(rename = "QryNm", skip_serializing_if = "Option::is_none")]
	pub qry_nm: Option<String>,
	#[serde(rename = "NewCrit", skip_serializing_if = "Option::is_none")]
	pub new_crit: Option<TransactionCriteria11>,
}


// TransactionQuery8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionQuery8 {
	#[serde(rename = "QryTp", skip_serializing_if = "Option::is_none")]
	pub qry_tp: Option<String>,
	#[serde(rename = "TxCrit", skip_serializing_if = "Option::is_none")]
	pub tx_crit: Option<TransactionCriteria8Choice>,
}


// TransactionReturnCriteria5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionReturnCriteria5 {
	#[serde(rename = "PmtToRtrCrit", skip_serializing_if = "Option::is_none")]
	pub pmt_to_rtr_crit: Option<SystemReturnCriteria2>,
	#[serde(rename = "PmtFrRtrCrit", skip_serializing_if = "Option::is_none")]
	pub pmt_fr_rtr_crit: Option<SystemReturnCriteria2>,
	#[serde(rename = "AcctCshNtryRtrCrit", skip_serializing_if = "Option::is_none")]
	pub acct_csh_ntry_rtr_crit: Option<AccountCashEntryReturnCriteria3>,
	#[serde(rename = "PmtRtrCrit", skip_serializing_if = "Option::is_none")]
	pub pmt_rtr_crit: Option<PaymentReturnCriteria4>,
}


// TransactionSearchCriteria11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionSearchCriteria11 {
	#[serde(rename = "PmtTo", skip_serializing_if = "Option::is_none")]
	pub pmt_to: Option<Vec<SystemSearch5>>,
	#[serde(rename = "PmtFr", skip_serializing_if = "Option::is_none")]
	pub pmt_fr: Option<Vec<SystemSearch5>>,
	#[serde(rename = "PmtSch", skip_serializing_if = "Option::is_none")]
	pub pmt_sch: Option<PaymentSearch10>,
	#[serde(rename = "AcctNtrySch", skip_serializing_if = "Option::is_none")]
	pub acct_ntry_sch: Option<CashAccountEntrySearch8>,
}


// UUIDv4Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UUIDv4Identifier {
	#[serde(rename = "UUIDv4Identifier")]
	pub uui_dv4_identifier: String,
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "YesNoIndicator")]
	pub yes_no_indicator: bool,
}
