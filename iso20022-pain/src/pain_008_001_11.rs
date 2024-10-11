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
	pub iban: Option<String>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<GenericAccountIdentification1>,
}


// AccountSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// ActiveOrHistoricCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
	#[serde(rename = "ActiveOrHistoricCurrencyAndAmount_SimpleType")]
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


// AdviceType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdviceType1 {
	#[serde(rename = "CdtAdvc", skip_serializing_if = "Option::is_none")]
	pub cdt_advc: Option<AdviceType1Choice>,
	#[serde(rename = "DbtAdvc", skip_serializing_if = "Option::is_none")]
	pub dbt_advc: Option<AdviceType1Choice>,
}


// AdviceType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdviceType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// AdviceType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdviceType1Code {
	#[serde(rename = "AdviceType1Code")]
	pub advice_type1_code: String,
}


// AmendmentInformationDetails15 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmendmentInformationDetails15 {
	#[serde(rename = "OrgnlMndtId", skip_serializing_if = "Option::is_none")]
	pub orgnl_mndt_id: Option<String>,
	#[serde(rename = "OrgnlCdtrSchmeId", skip_serializing_if = "Option::is_none")]
	pub orgnl_cdtr_schme_id: Option<PartyIdentification272>,
	#[serde(rename = "OrgnlCdtrAgt", skip_serializing_if = "Option::is_none")]
	pub orgnl_cdtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "OrgnlCdtrAgtAcct", skip_serializing_if = "Option::is_none")]
	pub orgnl_cdtr_agt_acct: Option<CashAccount40>,
	#[serde(rename = "OrgnlDbtr", skip_serializing_if = "Option::is_none")]
	pub orgnl_dbtr: Option<PartyIdentification272>,
	#[serde(rename = "OrgnlDbtrAcct", skip_serializing_if = "Option::is_none")]
	pub orgnl_dbtr_acct: Option<CashAccount40>,
	#[serde(rename = "OrgnlDbtrAgt", skip_serializing_if = "Option::is_none")]
	pub orgnl_dbtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "OrgnlDbtrAgtAcct", skip_serializing_if = "Option::is_none")]
	pub orgnl_dbtr_agt_acct: Option<CashAccount40>,
	#[serde(rename = "OrgnlFnlColltnDt", skip_serializing_if = "Option::is_none")]
	pub orgnl_fnl_colltn_dt: Option<String>,
	#[serde(rename = "OrgnlFrqcy", skip_serializing_if = "Option::is_none")]
	pub orgnl_frqcy: Option<Frequency36Choice>,
	#[serde(rename = "OrgnlRsn", skip_serializing_if = "Option::is_none")]
	pub orgnl_rsn: Option<MandateSetupReason1Choice>,
	#[serde(rename = "OrgnlTrckgDays", skip_serializing_if = "Option::is_none")]
	pub orgnl_trckg_days: Option<String>,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// Authorisation1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Authorisation1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// Authorisation1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Authorisation1Code {
	#[serde(rename = "Authorisation1Code")]
	pub authorisation1_code: String,
}


// BICFIDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BICFIDec2014Identifier {
	#[serde(rename = "BICFIDec2014Identifier")]
	pub bicfi_dec2014_identifier: String,
}


// BatchBookingIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BatchBookingIndicator {
	#[serde(rename = "BatchBookingIndicator")]
	pub batch_booking_indicator: bool,
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


// CashAccount40 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccount40 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<AccountIdentification4Choice>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<CashAccountType2Choice>,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<String>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
	#[serde(rename = "Prxy", skip_serializing_if = "Option::is_none")]
	pub prxy: Option<ProxyAccountIdentification1>,
}


// CashAccountType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// CategoryPurpose1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CategoryPurpose1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// ChargeBearerType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ChargeBearerType1Code {
	#[serde(rename = "ChargeBearerType1Code")]
	pub charge_bearer_type1_code: String,
}


// ClearingSystemIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemIdentification2Choice {
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


// CreditorReferenceInformation3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceInformation3 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<CreditorReferenceType3>,
	#[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
	pub ref_attr: Option<String>,
}


// CreditorReferenceType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// CreditorReferenceType3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceType3 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: CreditorReferenceType2Choice,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}


// CustomerDirectDebitInitiationV11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CustomerDirectDebitInitiationV11 {
	#[serde(rename = "GrpHdr")]
	pub grp_hdr: GroupHeader118,
	#[serde(rename = "PmtInf")]
	pub pmt_inf: Vec<PaymentInstruction45>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
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


// DateAndType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndType1 {
	#[serde(rename = "Tp")]
	pub tp: DateType2Choice,
	#[serde(rename = "Dt")]
	pub dt: String,
}


// DatePeriod2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriod2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// DateType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "DecimalNumber")]
	pub decimal_number: f64,
}


// DirectDebitTransaction12 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DirectDebitTransaction12 {
	#[serde(rename = "MndtRltdInf", skip_serializing_if = "Option::is_none")]
	pub mndt_rltd_inf: Option<MandateRelatedInformation16>,
	#[serde(rename = "CdtrSchmeId", skip_serializing_if = "Option::is_none")]
	pub cdtr_schme_id: Option<PartyIdentification272>,
	#[serde(rename = "PreNtfctnId", skip_serializing_if = "Option::is_none")]
	pub pre_ntfctn_id: Option<String>,
	#[serde(rename = "PreNtfctnDt", skip_serializing_if = "Option::is_none")]
	pub pre_ntfctn_dt: Option<String>,
}


// DirectDebitTransactionInformation32 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DirectDebitTransactionInformation32 {
	#[serde(rename = "PmtId")]
	pub pmt_id: PaymentIdentification6,
	#[serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none")]
	pub pmt_tp_inf: Option<PaymentTypeInformation29>,
	#[serde(rename = "InstdAmt")]
	pub instd_amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "ChrgBr", skip_serializing_if = "Option::is_none")]
	pub chrg_br: Option<String>,
	#[serde(rename = "DrctDbtTx", skip_serializing_if = "Option::is_none")]
	pub drct_dbt_tx: Option<DirectDebitTransaction12>,
	#[serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none")]
	pub ultmt_cdtr: Option<PartyIdentification272>,
	#[serde(rename = "DbtrAgt")]
	pub dbtr_agt: BranchAndFinancialInstitutionIdentification8,
	#[serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none")]
	pub dbtr_agt_acct: Option<CashAccount40>,
	#[serde(rename = "Dbtr")]
	pub dbtr: PartyIdentification272,
	#[serde(rename = "DbtrAcct")]
	pub dbtr_acct: CashAccount40,
	#[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
	pub ultmt_dbtr: Option<PartyIdentification272>,
	#[serde(rename = "InstrForCdtrAgt", skip_serializing_if = "Option::is_none")]
	pub instr_for_cdtr_agt: Option<String>,
	#[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
	pub purp: Option<Purpose2Choice>,
	#[serde(rename = "RgltryRptg", skip_serializing_if = "Option::is_none")]
	pub rgltry_rptg: Option<Vec<RegulatoryReporting3>>,
	#[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
	pub tax: Option<TaxData1>,
	#[serde(rename = "RltdRmtInf", skip_serializing_if = "Option::is_none")]
	pub rltd_rmt_inf: Option<Vec<RemittanceLocation8>>,
	#[serde(rename = "RmtInf", skip_serializing_if = "Option::is_none")]
	pub rmt_inf: Option<RemittanceInformation22>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// DocumentAdjustment1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentAdjustment1 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt_ind: Option<String>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<String>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<String>,
}


// DocumentAmount1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentAmount1 {
	#[serde(rename = "Tp")]
	pub tp: DocumentAmountType1Choice,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}


// DocumentAmountType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentAmountType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// DocumentLineIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentLineIdentification1 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<DocumentLineType1>,
	#[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
	pub nb: Option<String>,
	#[serde(rename = "RltdDt", skip_serializing_if = "Option::is_none")]
	pub rltd_dt: Option<String>,
}


// DocumentLineInformation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentLineInformation2 {
	#[serde(rename = "Id")]
	pub id: Vec<DocumentLineIdentification1>,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<String>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<RemittanceAmount4>,
}


// DocumentLineType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentLineType1 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: DocumentLineType1Choice,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}


// DocumentLineType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentLineType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// DocumentType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentType1 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: DocumentType2Choice,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}


// DocumentType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// Exact2NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact2NumericText {
	#[serde(rename = "Exact2NumericText")]
	pub exact2_numeric_text: String,
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


// ExternalCashAccountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalCashAccountType1Code {
	#[serde(rename = "ExternalCashAccountType1Code")]
	pub external_cash_account_type1_code: String,
}


// ExternalCategoryPurpose1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalCategoryPurpose1Code {
	#[serde(rename = "ExternalCategoryPurpose1Code")]
	pub external_category_purpose1_code: String,
}


// ExternalClearingSystemIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalClearingSystemIdentification1Code {
	#[serde(rename = "ExternalClearingSystemIdentification1Code")]
	pub external_clearing_system_identification1_code: String,
}


// ExternalCreditorReferenceType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalCreditorReferenceType1Code {
	#[serde(rename = "ExternalCreditorReferenceType1Code")]
	pub external_creditor_reference_type1_code: String,
}


// ExternalDateType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalDateType1Code {
	#[serde(rename = "ExternalDateType1Code")]
	pub external_date_type1_code: String,
}


// ExternalDocumentAmountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalDocumentAmountType1Code {
	#[serde(rename = "ExternalDocumentAmountType1Code")]
	pub external_document_amount_type1_code: String,
}


// ExternalDocumentLineType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalDocumentLineType1Code {
	#[serde(rename = "ExternalDocumentLineType1Code")]
	pub external_document_line_type1_code: String,
}


// ExternalDocumentType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalDocumentType1Code {
	#[serde(rename = "ExternalDocumentType1Code")]
	pub external_document_type1_code: String,
}


// ExternalFinancialInstitutionIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalFinancialInstitutionIdentification1Code {
	#[serde(rename = "ExternalFinancialInstitutionIdentification1Code")]
	pub external_financial_institution_identification1_code: String,
}


// ExternalGarnishmentType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalGarnishmentType1Code {
	#[serde(rename = "ExternalGarnishmentType1Code")]
	pub external_garnishment_type1_code: String,
}


// ExternalLocalInstrument1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalLocalInstrument1Code {
	#[serde(rename = "ExternalLocalInstrument1Code")]
	pub external_local_instrument1_code: String,
}


// ExternalMandateSetupReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalMandateSetupReason1Code {
	#[serde(rename = "ExternalMandateSetupReason1Code")]
	pub external_mandate_setup_reason1_code: String,
}


// ExternalOrganisationIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalOrganisationIdentification1Code {
	#[serde(rename = "ExternalOrganisationIdentification1Code")]
	pub external_organisation_identification1_code: String,
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


// ExternalPurpose1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPurpose1Code {
	#[serde(rename = "ExternalPurpose1Code")]
	pub external_purpose1_code: String,
}


// ExternalServiceLevel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalServiceLevel1Code {
	#[serde(rename = "ExternalServiceLevel1Code")]
	pub external_service_level1_code: String,
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


// Frequency36Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Frequency36Choice {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<String>,
	#[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
	pub prd: Option<FrequencyPeriod1>,
	#[serde(rename = "PtInTm", skip_serializing_if = "Option::is_none")]
	pub pt_in_tm: Option<FrequencyAndMoment1>,
}


// Frequency6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Frequency6Code {
	#[serde(rename = "Frequency6Code")]
	pub frequency6_code: String,
}


// FrequencyAndMoment1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FrequencyAndMoment1 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "PtInTm")]
	pub pt_in_tm: String,
}


// FrequencyPeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FrequencyPeriod1 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "CntPerPrd")]
	pub cnt_per_prd: f64,
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
	pub ref_nb: Option<String>,
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none")]
	pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "FmlyMdclInsrncInd", skip_serializing_if = "Option::is_none")]
	pub fmly_mdcl_insrnc_ind: Option<bool>,
	#[serde(rename = "MplyeeTermntnInd", skip_serializing_if = "Option::is_none")]
	pub mplyee_termntn_ind: Option<bool>,
}


// GarnishmentType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GarnishmentType1 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: GarnishmentType1Choice,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}


// GarnishmentType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GarnishmentType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
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


// GroupHeader118 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GroupHeader118 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
	#[serde(rename = "Authstn", skip_serializing_if = "Option::is_none")]
	pub authstn: Option<Vec<Authorisation1Choice>>,
	#[serde(rename = "NbOfTxs")]
	pub nb_of_txs: String,
	#[serde(rename = "CtrlSum", skip_serializing_if = "Option::is_none")]
	pub ctrl_sum: Option<f64>,
	#[serde(rename = "InitgPty")]
	pub initg_pty: PartyIdentification272,
	#[serde(rename = "FwdgAgt", skip_serializing_if = "Option::is_none")]
	pub fwdg_agt: Option<BranchAndFinancialInstitutionIdentification8>,
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


// ISOYear ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISOYear {
	#[serde(rename = "ISOYear")]
	pub iso_year: String,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LocalInstrument2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LocalInstrument2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// MandateRelatedInformation16 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MandateRelatedInformation16 {
	#[serde(rename = "MndtId", skip_serializing_if = "Option::is_none")]
	pub mndt_id: Option<String>,
	#[serde(rename = "DtOfSgntr", skip_serializing_if = "Option::is_none")]
	pub dt_of_sgntr: Option<String>,
	#[serde(rename = "AmdmntInd", skip_serializing_if = "Option::is_none")]
	pub amdmnt_ind: Option<bool>,
	#[serde(rename = "AmdmntInfDtls", skip_serializing_if = "Option::is_none")]
	pub amdmnt_inf_dtls: Option<AmendmentInformationDetails15>,
	#[serde(rename = "ElctrncSgntr", skip_serializing_if = "Option::is_none")]
	pub elctrnc_sgntr: Option<String>,
	#[serde(rename = "FrstColltnDt", skip_serializing_if = "Option::is_none")]
	pub frst_colltn_dt: Option<String>,
	#[serde(rename = "FnlColltnDt", skip_serializing_if = "Option::is_none")]
	pub fnl_colltn_dt: Option<String>,
	#[serde(rename = "Frqcy", skip_serializing_if = "Option::is_none")]
	pub frqcy: Option<Frequency36Choice>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<MandateSetupReason1Choice>,
	#[serde(rename = "TrckgDays", skip_serializing_if = "Option::is_none")]
	pub trckg_days: Option<String>,
}


// MandateSetupReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MandateSetupReason1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// Max1025Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max1025Text {
	#[serde(rename = "Max1025Text")]
	pub max1025_text: String,
}


// Max10Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max10Text {
	#[serde(rename = "Max10Text")]
	pub max10_text: String,
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


// Max15NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max15NumericText {
	#[serde(rename = "Max15NumericText")]
	pub max15_numeric_text: String,
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


// NameAndAddress18 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress18 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Adr")]
	pub adr: PostalAddress27,
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


// PaymentIdentification6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentIdentification6 {
	#[serde(rename = "InstrId", skip_serializing_if = "Option::is_none")]
	pub instr_id: Option<String>,
	#[serde(rename = "EndToEndId")]
	pub end_to_end_id: String,
	#[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
	pub uetr: Option<String>,
}


// PaymentInstruction45 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentInstruction45 {
	#[serde(rename = "PmtInfId")]
	pub pmt_inf_id: String,
	#[serde(rename = "PmtMtd")]
	pub pmt_mtd: String,
	#[serde(rename = "ReqdAdvcTp", skip_serializing_if = "Option::is_none")]
	pub reqd_advc_tp: Option<AdviceType1>,
	#[serde(rename = "BtchBookg", skip_serializing_if = "Option::is_none")]
	pub btch_bookg: Option<bool>,
	#[serde(rename = "NbOfTxs", skip_serializing_if = "Option::is_none")]
	pub nb_of_txs: Option<String>,
	#[serde(rename = "CtrlSum", skip_serializing_if = "Option::is_none")]
	pub ctrl_sum: Option<f64>,
	#[serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none")]
	pub pmt_tp_inf: Option<PaymentTypeInformation29>,
	#[serde(rename = "ReqdColltnDt")]
	pub reqd_colltn_dt: String,
	#[serde(rename = "Cdtr")]
	pub cdtr: PartyIdentification272,
	#[serde(rename = "CdtrAcct")]
	pub cdtr_acct: CashAccount40,
	#[serde(rename = "CdtrAgt")]
	pub cdtr_agt: BranchAndFinancialInstitutionIdentification8,
	#[serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none")]
	pub cdtr_agt_acct: Option<CashAccount40>,
	#[serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none")]
	pub ultmt_cdtr: Option<PartyIdentification272>,
	#[serde(rename = "ChrgBr", skip_serializing_if = "Option::is_none")]
	pub chrg_br: Option<String>,
	#[serde(rename = "ChrgsAcct", skip_serializing_if = "Option::is_none")]
	pub chrgs_acct: Option<CashAccount40>,
	#[serde(rename = "ChrgsAcctAgt", skip_serializing_if = "Option::is_none")]
	pub chrgs_acct_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "CdtrSchmeId", skip_serializing_if = "Option::is_none")]
	pub cdtr_schme_id: Option<PartyIdentification272>,
	#[serde(rename = "DrctDbtTxInf")]
	pub drct_dbt_tx_inf: Vec<DirectDebitTransactionInformation32>,
}


// PaymentMethod2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentMethod2Code {
	#[serde(rename = "PaymentMethod2Code")]
	pub payment_method2_code: String,
}


// PaymentTypeInformation29 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentTypeInformation29 {
	#[serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none")]
	pub instr_prty: Option<String>,
	#[serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none")]
	pub svc_lvl: Option<Vec<ServiceLevel8Choice>>,
	#[serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none")]
	pub lcl_instrm: Option<LocalInstrument2Choice>,
	#[serde(rename = "SeqTp", skip_serializing_if = "Option::is_none")]
	pub seq_tp: Option<String>,
	#[serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none")]
	pub ctgy_purp: Option<CategoryPurpose1Choice>,
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
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


// Priority2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Priority2Code {
	#[serde(rename = "Priority2Code")]
	pub priority2_code: String,
}


// ProxyAccountIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProxyAccountIdentification1 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<ProxyAccountType1Choice>,
	#[serde(rename = "Id")]
	pub id: String,
}


// ProxyAccountType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProxyAccountType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// Purpose2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Purpose2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// ReferredDocumentInformation8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReferredDocumentInformation8 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<DocumentType1>,
	#[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
	pub nb: Option<String>,
	#[serde(rename = "RltdDt", skip_serializing_if = "Option::is_none")]
	pub rltd_dt: Option<DateAndType1>,
	#[serde(rename = "LineDtls", skip_serializing_if = "Option::is_none")]
	pub line_dtls: Option<Vec<DocumentLineInformation2>>,
}


// RegulatoryAuthority2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegulatoryAuthority2 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<String>,
}


// RegulatoryReporting3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegulatoryReporting3 {
	#[serde(rename = "DbtCdtRptgInd", skip_serializing_if = "Option::is_none")]
	pub dbt_cdt_rptg_ind: Option<String>,
	#[serde(rename = "Authrty", skip_serializing_if = "Option::is_none")]
	pub authrty: Option<RegulatoryAuthority2>,
	#[serde(rename = "Dtls", skip_serializing_if = "Option::is_none")]
	pub dtls: Option<Vec<StructuredRegulatoryReporting3>>,
}


// RegulatoryReportingType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegulatoryReportingType1Code {
	#[serde(rename = "RegulatoryReportingType1Code")]
	pub regulatory_reporting_type1_code: String,
}


// RemittanceAmount4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceAmount4 {
	#[serde(rename = "RmtAmtAndTp", skip_serializing_if = "Option::is_none")]
	pub rmt_amt_and_tp: Option<Vec<DocumentAmount1>>,
	#[serde(rename = "AdjstmntAmtAndRsn", skip_serializing_if = "Option::is_none")]
	pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment1>>,
}


// RemittanceInformation22 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceInformation22 {
	#[serde(rename = "Ustrd", skip_serializing_if = "Option::is_none")]
	pub ustrd: Option<Vec<String>>,
	#[serde(rename = "Strd", skip_serializing_if = "Option::is_none")]
	pub strd: Option<Vec<StructuredRemittanceInformation18>>,
}


// RemittanceLocation8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceLocation8 {
	#[serde(rename = "RmtId", skip_serializing_if = "Option::is_none")]
	pub rmt_id: Option<String>,
	#[serde(rename = "RmtLctnDtls", skip_serializing_if = "Option::is_none")]
	pub rmt_lctn_dtls: Option<Vec<RemittanceLocationData2>>,
}


// RemittanceLocationData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceLocationData2 {
	#[serde(rename = "Mtd")]
	pub mtd: String,
	#[serde(rename = "ElctrncAdr", skip_serializing_if = "Option::is_none")]
	pub elctrnc_adr: Option<String>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<NameAndAddress18>,
}


// RemittanceLocationMethod2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceLocationMethod2Code {
	#[serde(rename = "RemittanceLocationMethod2Code")]
	pub remittance_location_method2_code: String,
}


// SequenceType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SequenceType3Code {
	#[serde(rename = "SequenceType3Code")]
	pub sequence_type3_code: String,
}


// ServiceLevel8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ServiceLevel8Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// StructuredRegulatoryReporting3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StructuredRegulatoryReporting3 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<String>,
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<String>,
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Inf", skip_serializing_if = "Option::is_none")]
	pub inf: Option<Vec<String>>,
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
	pub addtl_rmt_inf: Option<Vec<String>>,
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


// TaxAuthorisation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxAuthorisation1 {
	#[serde(rename = "Titl", skip_serializing_if = "Option::is_none")]
	pub titl: Option<String>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
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
	pub admstn_zone: Option<String>,
	#[serde(rename = "RefNb", skip_serializing_if = "Option::is_none")]
	pub ref_nb: Option<String>,
	#[serde(rename = "Mtd", skip_serializing_if = "Option::is_none")]
	pub mtd: Option<String>,
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


// TaxParty1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxParty1 {
	#[serde(rename = "TaxId", skip_serializing_if = "Option::is_none")]
	pub tax_id: Option<String>,
	#[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
	pub regn_id: Option<String>,
	#[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
	pub tax_tp: Option<String>,
}


// TaxParty2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxParty2 {
	#[serde(rename = "TaxId", skip_serializing_if = "Option::is_none")]
	pub tax_id: Option<String>,
	#[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
	pub regn_id: Option<String>,
	#[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
	pub tax_tp: Option<String>,
	#[serde(rename = "Authstn", skip_serializing_if = "Option::is_none")]
	pub authstn: Option<TaxAuthorisation1>,
}


// TaxPeriod3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxPeriod3 {
	#[serde(rename = "Yr", skip_serializing_if = "Option::is_none")]
	pub yr: Option<String>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<String>,
	#[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
	pub fr_to_dt: Option<DatePeriod2>,
}


// TaxRecord3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxRecord3 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<String>,
	#[serde(rename = "Ctgy", skip_serializing_if = "Option::is_none")]
	pub ctgy: Option<String>,
	#[serde(rename = "CtgyDtls", skip_serializing_if = "Option::is_none")]
	pub ctgy_dtls: Option<String>,
	#[serde(rename = "DbtrSts", skip_serializing_if = "Option::is_none")]
	pub dbtr_sts: Option<String>,
	#[serde(rename = "CertId", skip_serializing_if = "Option::is_none")]
	pub cert_id: Option<String>,
	#[serde(rename = "FrmsCd", skip_serializing_if = "Option::is_none")]
	pub frms_cd: Option<String>,
	#[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
	pub prd: Option<TaxPeriod3>,
	#[serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none")]
	pub tax_amt: Option<TaxAmount3>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<String>,
}


// TaxRecordDetails3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxRecordDetails3 {
	#[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
	pub prd: Option<TaxPeriod3>,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}


// TaxRecordPeriod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxRecordPeriod1Code {
	#[serde(rename = "TaxRecordPeriod1Code")]
	pub tax_record_period1_code: String,
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}


// UUIDv4Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UUIDv4Identifier {
	#[serde(rename = "UUIDv4Identifier")]
	pub uui_dv4_identifier: String,
}
