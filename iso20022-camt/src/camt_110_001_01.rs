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


// AdditionalRequestData1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdditionalRequestData1Choice {
	#[serde(rename = "ReqdDbtAuthstn")]
	pub reqd_dbt_authstn: Option<DebitAuthorisation3>,
	#[serde(rename = "ReqdCompstn")]
	pub reqd_compstn: Option<CompensationRequest1>,
	#[serde(rename = "ReqdValtn")]
	pub reqd_valtn: Option<AdjustmentRequest1>,
	#[serde(rename = "ReqNrrtv")]
	pub req_nrrtv: Option<String>,
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


// AdjustmentRequest1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdjustmentRequest1 {
	#[serde(rename = "Prd")]
	pub prd: Option<DatePeriod5>,
}


// AmendmentInformationDetails14 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmendmentInformationDetails14 {
	#[serde(rename = "OrgnlMndtId")]
	pub orgnl_mndt_id: Option<String>,
	#[serde(rename = "OrgnlCdtrSchmeId")]
	pub orgnl_cdtr_schme_id: Option<PartyIdentification135>,
	#[serde(rename = "OrgnlCdtrAgt")]
	pub orgnl_cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "OrgnlCdtrAgtAcct")]
	pub orgnl_cdtr_agt_acct: Option<CashAccount40>,
	#[serde(rename = "OrgnlDbtr")]
	pub orgnl_dbtr: Option<PartyIdentification135>,
	#[serde(rename = "OrgnlDbtrAcct")]
	pub orgnl_dbtr_acct: Option<CashAccount40>,
	#[serde(rename = "OrgnlDbtrAgt")]
	pub orgnl_dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "OrgnlDbtrAgtAcct")]
	pub orgnl_dbtr_agt_acct: Option<CashAccount40>,
	#[serde(rename = "OrgnlFnlColltnDt")]
	pub orgnl_fnl_colltn_dt: Option<String>,
	#[serde(rename = "OrgnlFrqcy")]
	pub orgnl_frqcy: Option<Frequency36Choice>,
	#[serde(rename = "OrgnlRsn")]
	pub orgnl_rsn: Option<MandateSetupReason1Choice>,
	#[serde(rename = "OrgnlTrckgDays")]
	pub orgnl_trckg_days: Option<String>,
}


// AmountType4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountType4Choice {
	#[serde(rename = "InstdAmt")]
	pub instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "EqvtAmt")]
	pub eqvt_amt: Option<EquivalentAmount2>,
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


// BranchAndFinancialInstitutionIdentification6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BranchAndFinancialInstitutionIdentification6 {
	#[serde(rename = "FinInstnId")]
	pub fin_instn_id: FinancialInstitutionIdentification18,
	#[serde(rename = "BrnchId")]
	pub brnch_id: Option<BranchData3>,
}


// BranchData3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BranchData3 {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress24>,
}


// CancellationReason33Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CancellationReason33Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
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


// CashAccountType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountType2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// CategoryPurpose1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CategoryPurpose1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ClearingChannel2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingChannel2Code {
	#[serde(rename = "ClearingChannel2Code")]
	pub clearing_channel2_code: String,
}


// ClearingSystemIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemIdentification2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ClearingSystemIdentification3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemIdentification3Choice {
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


// CompensationRequest1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompensationRequest1 {
	#[serde(rename = "CompstnAcct")]
	pub compstn_acct: Option<CashAccount40>,
	#[serde(rename = "Prd")]
	pub prd: DatePeriod2,
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "XpctdValDt")]
	pub xpctd_val_dt: Option<String>,
	#[serde(rename = "IntrstRate")]
	pub intrst_rate: Option<f64>,
	#[serde(rename = "Rsn")]
	pub rsn: Option<Vec<String>>,
}


// Contact4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Contact4 {
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


// CreditTransferMandateData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditTransferMandateData1 {
	#[serde(rename = "MndtId")]
	pub mndt_id: Option<String>,
	#[serde(rename = "Tp")]
	pub tp: Option<MandateTypeInformation2>,
	#[serde(rename = "DtOfSgntr")]
	pub dt_of_sgntr: Option<String>,
	#[serde(rename = "DtOfVrfctn")]
	pub dt_of_vrfctn: Option<String>,
	#[serde(rename = "ElctrncSgntr")]
	pub elctrnc_sgntr: Option<String>,
	#[serde(rename = "FrstPmtDt")]
	pub frst_pmt_dt: Option<String>,
	#[serde(rename = "FnlPmtDt")]
	pub fnl_pmt_dt: Option<String>,
	#[serde(rename = "Frqcy")]
	pub frqcy: Option<Frequency36Choice>,
	#[serde(rename = "Rsn")]
	pub rsn: Option<MandateSetupReason1Choice>,
}


// CreditorReferenceInformation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceInformation2 {
	#[serde(rename = "Tp")]
	pub tp: Option<CreditorReferenceType2>,
	#[serde(rename = "Ref")]
	pub ref_attr: Option<String>,
}


// CreditorReferenceType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// CreditorReferenceType2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceType2 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: CreditorReferenceType1Choice,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
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


// DatePeriod2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriod2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// DatePeriod5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriod5 {
	#[serde(rename = "CurValDt")]
	pub cur_val_dt: String,
	#[serde(rename = "ReqdValDt")]
	pub reqd_val_dt: String,
}


// DebitAuthorisation3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DebitAuthorisation3 {
	#[serde(rename = "CxlRsn")]
	pub cxl_rsn: CancellationReason33Choice,
	#[serde(rename = "AmtToDbt")]
	pub amt_to_dbt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Acct")]
	pub acct: Option<CashAccount40>,
	#[serde(rename = "ValDtToDbt")]
	pub val_dt_to_dbt: Option<String>,
	#[serde(rename = "AddtlCxlRsnInf")]
	pub addtl_cxl_rsn_inf: Option<Vec<String>>,
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "DecimalNumber")]
	pub decimal_number: f64,
}


// DiscountAmountAndType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DiscountAmountAndType1 {
	#[serde(rename = "Tp")]
	pub tp: Option<DiscountAmountType1Choice>,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}


// DiscountAmountType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DiscountAmountType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// Document12 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Document12 {
	#[serde(rename = "Tp")]
	pub tp: DocumentType1Choice,
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "IsseDt")]
	pub isse_dt: DateAndDateTime2Choice,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "LangCd")]
	pub lang_cd: Option<String>,
	#[serde(rename = "Frmt")]
	pub frmt: DocumentFormat1Choice,
	#[serde(rename = "FileNm")]
	pub file_nm: Option<String>,
	#[serde(rename = "DgtlSgntr")]
	pub dgtl_sgntr: Option<PartyAndSignature3>,
	#[serde(rename = "Nclsr")]
	pub nclsr: String,
}


// DocumentAdjustment1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentAdjustment1 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: Option<String>,
	#[serde(rename = "Rsn")]
	pub rsn: Option<String>,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<String>,
}


// DocumentFormat1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentFormat1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification1>,
}


// DocumentLineIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentLineIdentification1 {
	#[serde(rename = "Tp")]
	pub tp: Option<DocumentLineType1>,
	#[serde(rename = "Nb")]
	pub nb: Option<String>,
	#[serde(rename = "RltdDt")]
	pub rltd_dt: Option<String>,
}


// DocumentLineInformation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentLineInformation1 {
	#[serde(rename = "Id")]
	pub id: Vec<DocumentLineIdentification1>,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
	#[serde(rename = "Amt")]
	pub amt: Option<RemittanceAmount3>,
}


// DocumentLineType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentLineType1 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: DocumentLineType1Choice,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// DocumentLineType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentLineType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// DocumentType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification1>,
}


// DocumentType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentType3Code {
	#[serde(rename = "DocumentType3Code")]
	pub document_type3_code: String,
}


// DocumentType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentType6Code {
	#[serde(rename = "DocumentType6Code")]
	pub document_type6_code: String,
}


// EquivalentAmount2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EquivalentAmount2 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CcyOfTrf")]
	pub ccy_of_trf: String,
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


// ExternalCancellationReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalCancellationReason1Code {
	#[serde(rename = "ExternalCancellationReason1Code")]
	pub external_cancellation_reason1_code: String,
}


// ExternalCashAccountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalCashAccountType1Code {
	#[serde(rename = "ExternalCashAccountType1Code")]
	pub external_cash_account_type1_code: String,
}


// ExternalCashClearingSystem1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalCashClearingSystem1Code {
	#[serde(rename = "ExternalCashClearingSystem1Code")]
	pub external_cash_clearing_system1_code: String,
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


// ExternalDiscountAmountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalDiscountAmountType1Code {
	#[serde(rename = "ExternalDiscountAmountType1Code")]
	pub external_discount_amount_type1_code: String,
}


// ExternalDocumentFormat1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalDocumentFormat1Code {
	#[serde(rename = "ExternalDocumentFormat1Code")]
	pub external_document_format1_code: String,
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


// ExternalInvestigationAction1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalInvestigationAction1Code {
	#[serde(rename = "ExternalInvestigationAction1Code")]
	pub external_investigation_action1_code: String,
}


// ExternalInvestigationActionReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalInvestigationActionReason1Code {
	#[serde(rename = "ExternalInvestigationActionReason1Code")]
	pub external_investigation_action_reason1_code: String,
}


// ExternalInvestigationInstrument1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalInvestigationInstrument1Code {
	#[serde(rename = "ExternalInvestigationInstrument1Code")]
	pub external_investigation_instrument1_code: String,
}


// ExternalInvestigationReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalInvestigationReason1Code {
	#[serde(rename = "ExternalInvestigationReason1Code")]
	pub external_investigation_reason1_code: String,
}


// ExternalInvestigationReasonSubType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalInvestigationReasonSubType1Code {
	#[serde(rename = "ExternalInvestigationReasonSubType1Code")]
	pub external_investigation_reason_sub_type1_code: String,
}


// ExternalInvestigationServiceLevel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalInvestigationServiceLevel1Code {
	#[serde(rename = "ExternalInvestigationServiceLevel1Code")]
	pub external_investigation_service_level1_code: String,
}


// ExternalInvestigationSubType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalInvestigationSubType1Code {
	#[serde(rename = "ExternalInvestigationSubType1Code")]
	pub external_investigation_sub_type1_code: String,
}


// ExternalInvestigationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalInvestigationType1Code {
	#[serde(rename = "ExternalInvestigationType1Code")]
	pub external_investigation_type1_code: String,
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


// ExternalTaxAmountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalTaxAmountType1Code {
	#[serde(rename = "ExternalTaxAmountType1Code")]
	pub external_tax_amount_type1_code: String,
}


// FileData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FileData1 {
	#[serde(rename = "Tp")]
	pub tp: Option<DocumentType1Choice>,
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "IsseDt")]
	pub isse_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "Frmt")]
	pub frmt: Option<DocumentFormat1Choice>,
	#[serde(rename = "FileNm")]
	pub file_nm: Option<String>,
	#[serde(rename = "NtwkRef")]
	pub ntwk_ref: Option<String>,
	#[serde(rename = "FileLctnElctrncAdr")]
	pub file_lctn_elctrnc_adr: Option<String>,
}


// FinancialIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// FinancialInstitutionIdentification18 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstitutionIdentification18 {
	#[serde(rename = "BICFI")]
	pub bicfi: Option<String>,
	#[serde(rename = "ClrSysMmbId")]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress24>,
	#[serde(rename = "Othr")]
	pub othr: Option<GenericFinancialIdentification1>,
}


// Frequency36Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Frequency36Choice {
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "Prd")]
	pub prd: Option<FrequencyPeriod1>,
	#[serde(rename = "PtInTm")]
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


// Garnishment3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Garnishment3 {
	#[serde(rename = "Tp")]
	pub tp: GarnishmentType1,
	#[serde(rename = "Grnshee")]
	pub grnshee: Option<PartyIdentification135>,
	#[serde(rename = "GrnshmtAdmstr")]
	pub grnshmt_admstr: Option<PartyIdentification135>,
	#[serde(rename = "RefNb")]
	pub ref_nb: Option<String>,
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "RmtdAmt")]
	pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "FmlyMdclInsrncInd")]
	pub fmly_mdcl_insrnc_ind: Option<bool>,
	#[serde(rename = "MplyeeTermntnInd")]
	pub mplyee_termntn_ind: Option<bool>,
}


// GarnishmentType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GarnishmentType1 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: GarnishmentType1Choice,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GarnishmentType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GarnishmentType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
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


// GenericOrganisationIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericOrganisationIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericPersonIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericPersonIdentification1 {
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


// ISOYear ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISOYear {
	#[serde(rename = "ISOYear")]
	pub iso_year: String,
}


// InvestigationActionReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationActionReason1 {
	#[serde(rename = "Orgtr")]
	pub orgtr: Option<PartyIdentification135>,
	#[serde(rename = "Rsn")]
	pub rsn: InvestigationActionReason1Choice,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<Vec<String>>,
}


// InvestigationActionReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationActionReason1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// InvestigationLocationData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationLocationData1 {
	#[serde(rename = "Mtd")]
	pub mtd: String,
	#[serde(rename = "ElctrncAdr")]
	pub elctrnc_adr: Option<String>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<NameAndAddress16>,
}


// InvestigationLocationMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationLocationMethod1Code {
	#[serde(rename = "InvestigationLocationMethod1Code")]
	pub investigation_location_method1_code: String,
}


// InvestigationReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationReason1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// InvestigationReason2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationReason2 {
	#[serde(rename = "Seq")]
	pub seq: Option<f64>,
	#[serde(rename = "Rsn")]
	pub rsn: InvestigationReason1Choice,
	#[serde(rename = "RsnSubTp")]
	pub rsn_sub_tp: Option<InvestigationReasonSubType1Choice>,
	#[serde(rename = "AddtlReqData")]
	pub addtl_req_data: Option<AdditionalRequestData1Choice>,
	#[serde(rename = "RltdInvstgtnData")]
	pub rltd_invstgtn_data: Option<RelatedInvestigationData1>,
	#[serde(rename = "NclsdFile")]
	pub nclsd_file: Option<Vec<Document12>>,
	#[serde(rename = "RltdFileData")]
	pub rltd_file_data: Option<Vec<FileData1>>,
}


// InvestigationReasonSubType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationReasonSubType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// InvestigationRequest2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationRequest2 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "RqstrInvstgtnId")]
	pub rqstr_invstgtn_id: Option<String>,
	#[serde(rename = "RspndrInvstgtnId")]
	pub rspndr_invstgtn_id: Option<String>,
	#[serde(rename = "EIR")]
	pub eir: Option<String>,
	#[serde(rename = "ReqActn")]
	pub req_actn: Option<InvestigationRequestAction1>,
	#[serde(rename = "InvstgtnTp")]
	pub invstgtn_tp: InvestigationType1Choice,
	#[serde(rename = "InvstgtnSubTp")]
	pub invstgtn_sub_tp: Option<InvestigationSubType1Choice>,
	#[serde(rename = "UndrlygInstrm")]
	pub undrlyg_instrm: Option<UnderlyingInvestigationInstrument1Choice>,
	#[serde(rename = "Undrlyg")]
	pub undrlyg: UnderlyingData2Choice,
	#[serde(rename = "Rqstr")]
	pub rqstr: Party40Choice,
	#[serde(rename = "Rspndr")]
	pub rspndr: Party40Choice,
	#[serde(rename = "ReqOrgtr")]
	pub req_orgtr: Option<Party40Choice>,
	#[serde(rename = "XpctdRspndr")]
	pub xpctd_rspndr: Option<Party40Choice>,
	#[serde(rename = "SvcLvl")]
	pub svc_lvl: Option<Vec<InvestigationServiceLevel1Choice>>,
}


// InvestigationRequestAction1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationRequestAction1 {
	#[serde(rename = "Actn")]
	pub actn: InvestigationRequestAction1Choice,
	#[serde(rename = "ActnRsn")]
	pub actn_rsn: Option<InvestigationActionReason1>,
}


// InvestigationRequestAction1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationRequestAction1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// InvestigationRequestV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationRequestV01 {
	#[serde(rename = "InvstgtnReq")]
	pub invstgtn_req: InvestigationRequest2,
	#[serde(rename = "InvstgtnData")]
	pub invstgtn_data: Vec<InvestigationReason2>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// InvestigationServiceLevel1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationServiceLevel1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// InvestigationSubType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationSubType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// InvestigationType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LanguageCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LanguageCode {
	#[serde(rename = "LanguageCode")]
	pub language_code: String,
}


// LocalInstrument2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LocalInstrument2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// MandateClassification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MandateClassification1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// MandateClassification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MandateClassification1Code {
	#[serde(rename = "MandateClassification1Code")]
	pub mandate_classification1_code: String,
}


// MandateRelatedData2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MandateRelatedData2Choice {
	#[serde(rename = "DrctDbtMndt")]
	pub drct_dbt_mndt: Option<MandateRelatedInformation15>,
	#[serde(rename = "CdtTrfMndt")]
	pub cdt_trf_mndt: Option<CreditTransferMandateData1>,
}


// MandateRelatedInformation15 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MandateRelatedInformation15 {
	#[serde(rename = "MndtId")]
	pub mndt_id: Option<String>,
	#[serde(rename = "DtOfSgntr")]
	pub dt_of_sgntr: Option<String>,
	#[serde(rename = "AmdmntInd")]
	pub amdmnt_ind: Option<bool>,
	#[serde(rename = "AmdmntInfDtls")]
	pub amdmnt_inf_dtls: Option<AmendmentInformationDetails14>,
	#[serde(rename = "ElctrncSgntr")]
	pub elctrnc_sgntr: Option<String>,
	#[serde(rename = "FrstColltnDt")]
	pub frst_colltn_dt: Option<String>,
	#[serde(rename = "FnlColltnDt")]
	pub fnl_colltn_dt: Option<String>,
	#[serde(rename = "Frqcy")]
	pub frqcy: Option<Frequency36Choice>,
	#[serde(rename = "Rsn")]
	pub rsn: Option<MandateSetupReason1Choice>,
	#[serde(rename = "TrckgDays")]
	pub trckg_days: Option<String>,
}


// MandateSetupReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MandateSetupReason1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// MandateTypeInformation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MandateTypeInformation2 {
	#[serde(rename = "SvcLvl")]
	pub svc_lvl: Option<ServiceLevel8Choice>,
	#[serde(rename = "LclInstrm")]
	pub lcl_instrm: Option<LocalInstrument2Choice>,
	#[serde(rename = "CtgyPurp")]
	pub ctgy_purp: Option<CategoryPurpose1Choice>,
	#[serde(rename = "Clssfctn")]
	pub clssfctn: Option<MandateClassification1Choice>,
}


// Max1025Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max1025Text {
	#[serde(rename = "Max1025Text")]
	pub max1025_text: String,
}


// Max105Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max105Text {
	#[serde(rename = "Max105Text")]
	pub max105_text: String,
}


// Max10KBinary ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max10KBinary {
	#[serde(rename = "Max10KBinary")]
	pub max10_k_binary: String,
}


// Max10MbBinary ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max10MbBinary {
	#[serde(rename = "Max10MbBinary")]
	pub max10_mb_binary: String,
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


// Max3Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max3Number {
	#[serde(rename = "Max3Number")]
	pub max3_number: f64,
}


// Max4Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4Text {
	#[serde(rename = "Max4Text")]
	pub max4_text: String,
}


// Max500Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max500Text {
	#[serde(rename = "Max500Text")]
	pub max500_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// NameAndAddress16 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress16 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Adr")]
	pub adr: PostalAddress24,
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


// OrganisationIdentification29 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification29 {
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<GenericOrganisationIdentification1>>,
}


// OrganisationIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// OriginalGroupInformation29 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OriginalGroupInformation29 {
	#[serde(rename = "OrgnlMsgId")]
	pub orgnl_msg_id: String,
	#[serde(rename = "OrgnlMsgNmId")]
	pub orgnl_msg_nm_id: String,
	#[serde(rename = "OrgnlCreDtTm")]
	pub orgnl_cre_dt_tm: Option<String>,
}


// OriginalTransactionReference35 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OriginalTransactionReference35 {
	#[serde(rename = "IntrBkSttlmAmt")]
	pub intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Amt")]
	pub amt: Option<AmountType4Choice>,
	#[serde(rename = "IntrBkSttlmDt")]
	pub intr_bk_sttlm_dt: Option<String>,
	#[serde(rename = "ReqdColltnDt")]
	pub reqd_colltn_dt: Option<String>,
	#[serde(rename = "ReqdExctnDt")]
	pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "CdtrSchmeId")]
	pub cdtr_schme_id: Option<PartyIdentification135>,
	#[serde(rename = "SttlmInf")]
	pub sttlm_inf: Option<SettlementInstruction11>,
	#[serde(rename = "PmtTpInf")]
	pub pmt_tp_inf: Option<PaymentTypeInformation27>,
	#[serde(rename = "PmtMtd")]
	pub pmt_mtd: Option<String>,
	#[serde(rename = "MndtRltdInf")]
	pub mndt_rltd_inf: Option<MandateRelatedData2Choice>,
	#[serde(rename = "RmtInf")]
	pub rmt_inf: Option<RemittanceInformation21>,
	#[serde(rename = "UltmtDbtr")]
	pub ultmt_dbtr: Option<Party40Choice>,
	#[serde(rename = "Dbtr")]
	pub dbtr: Option<Party40Choice>,
	#[serde(rename = "DbtrAcct")]
	pub dbtr_acct: Option<CashAccount40>,
	#[serde(rename = "DbtrAgt")]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "DbtrAgtAcct")]
	pub dbtr_agt_acct: Option<CashAccount40>,
	#[serde(rename = "CdtrAgt")]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "CdtrAgtAcct")]
	pub cdtr_agt_acct: Option<CashAccount40>,
	#[serde(rename = "Cdtr")]
	pub cdtr: Option<Party40Choice>,
	#[serde(rename = "CdtrAcct")]
	pub cdtr_acct: Option<CashAccount40>,
	#[serde(rename = "UltmtCdtr")]
	pub ultmt_cdtr: Option<Party40Choice>,
	#[serde(rename = "Purp")]
	pub purp: Option<Purpose2Choice>,
}


// OtherContact1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherContact1 {
	#[serde(rename = "ChanlTp")]
	pub chanl_tp: String,
	#[serde(rename = "Id")]
	pub id: Option<String>,
}


// Party38Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party38Choice {
	#[serde(rename = "OrgId")]
	pub org_id: Option<OrganisationIdentification29>,
	#[serde(rename = "PrvtId")]
	pub prvt_id: Option<PersonIdentification13>,
}


// Party40Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party40Choice {
	#[serde(rename = "Pty")]
	pub pty: Option<PartyIdentification135>,
	#[serde(rename = "Agt")]
	pub agt: Option<BranchAndFinancialInstitutionIdentification6>,
}


// PartyAndSignature3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyAndSignature3 {
	#[serde(rename = "Pty")]
	pub pty: PartyIdentification135,
	#[serde(rename = "Sgntr")]
	pub sgntr: SkipPayload,
}


// PartyIdentification135 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification135 {
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress24>,
	#[serde(rename = "Id")]
	pub id: Option<Party38Choice>,
	#[serde(rename = "CtryOfRes")]
	pub ctry_of_res: Option<String>,
	#[serde(rename = "CtctDtls")]
	pub ctct_dtls: Option<Contact4>,
}


// PaymentMethod4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentMethod4Code {
	#[serde(rename = "PaymentMethod4Code")]
	pub payment_method4_code: String,
}


// PaymentTypeInformation27 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentTypeInformation27 {
	#[serde(rename = "InstrPrty")]
	pub instr_prty: Option<String>,
	#[serde(rename = "ClrChanl")]
	pub clr_chanl: Option<String>,
	#[serde(rename = "SvcLvl")]
	pub svc_lvl: Option<Vec<ServiceLevel8Choice>>,
	#[serde(rename = "LclInstrm")]
	pub lcl_instrm: Option<LocalInstrument2Choice>,
	#[serde(rename = "SeqTp")]
	pub seq_tp: Option<String>,
	#[serde(rename = "CtgyPurp")]
	pub ctgy_purp: Option<CategoryPurpose1Choice>,
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// PersonIdentification13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentification13 {
	#[serde(rename = "DtAndPlcOfBirth")]
	pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<GenericPersonIdentification1>>,
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


// PostalAddress24 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress24 {
	#[serde(rename = "AdrTp")]
	pub adr_tp: Option<AddressType3Choice>,
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


// PreferredContactMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PreferredContactMethod1Code {
	#[serde(rename = "PreferredContactMethod1Code")]
	pub preferred_contact_method1_code: String,
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


// Purpose2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Purpose2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ReferredDocumentInformation7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReferredDocumentInformation7 {
	#[serde(rename = "Tp")]
	pub tp: Option<ReferredDocumentType4>,
	#[serde(rename = "Nb")]
	pub nb: Option<String>,
	#[serde(rename = "RltdDt")]
	pub rltd_dt: Option<String>,
	#[serde(rename = "LineDtls")]
	pub line_dtls: Option<Vec<DocumentLineInformation1>>,
}


// ReferredDocumentType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReferredDocumentType3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ReferredDocumentType4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReferredDocumentType4 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: ReferredDocumentType3Choice,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// RelatedInvestigationData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RelatedInvestigationData1 {
	#[serde(rename = "InvstgtnId")]
	pub invstgtn_id: Option<String>,
	#[serde(rename = "Lctn")]
	pub lctn: Option<Vec<InvestigationLocationData1>>,
}


// RemittanceAmount2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceAmount2 {
	#[serde(rename = "DuePyblAmt")]
	pub due_pybl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "DscntApldAmt")]
	pub dscnt_apld_amt: Option<Vec<DiscountAmountAndType1>>,
	#[serde(rename = "CdtNoteAmt")]
	pub cdt_note_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TaxAmt")]
	pub tax_amt: Option<Vec<TaxAmountAndType1>>,
	#[serde(rename = "AdjstmntAmtAndRsn")]
	pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment1>>,
	#[serde(rename = "RmtdAmt")]
	pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// RemittanceAmount3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceAmount3 {
	#[serde(rename = "DuePyblAmt")]
	pub due_pybl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "DscntApldAmt")]
	pub dscnt_apld_amt: Option<Vec<DiscountAmountAndType1>>,
	#[serde(rename = "CdtNoteAmt")]
	pub cdt_note_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TaxAmt")]
	pub tax_amt: Option<Vec<TaxAmountAndType1>>,
	#[serde(rename = "AdjstmntAmtAndRsn")]
	pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment1>>,
	#[serde(rename = "RmtdAmt")]
	pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// RemittanceInformation21 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceInformation21 {
	#[serde(rename = "Ustrd")]
	pub ustrd: Option<Vec<String>>,
	#[serde(rename = "Strd")]
	pub strd: Option<Vec<StructuredRemittanceInformation17>>,
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
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// SettlementInstruction11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementInstruction11 {
	#[serde(rename = "SttlmMtd")]
	pub sttlm_mtd: String,
	#[serde(rename = "SttlmAcct")]
	pub sttlm_acct: Option<CashAccount40>,
	#[serde(rename = "ClrSys")]
	pub clr_sys: Option<ClearingSystemIdentification3Choice>,
	#[serde(rename = "InstgRmbrsmntAgt")]
	pub instg_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "InstgRmbrsmntAgtAcct")]
	pub instg_rmbrsmnt_agt_acct: Option<CashAccount40>,
	#[serde(rename = "InstdRmbrsmntAgt")]
	pub instd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "InstdRmbrsmntAgtAcct")]
	pub instd_rmbrsmnt_agt_acct: Option<CashAccount40>,
	#[serde(rename = "ThrdRmbrsmntAgt")]
	pub thrd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "ThrdRmbrsmntAgtAcct")]
	pub thrd_rmbrsmnt_agt_acct: Option<CashAccount40>,
}


// SettlementMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementMethod1Code {
	#[serde(rename = "SettlementMethod1Code")]
	pub settlement_method1_code: String,
}


// SkipPayload ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SkipPayload {
}


// StructuredRemittanceInformation17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StructuredRemittanceInformation17 {
	#[serde(rename = "RfrdDocInf")]
	pub rfrd_doc_inf: Option<Vec<ReferredDocumentInformation7>>,
	#[serde(rename = "RfrdDocAmt")]
	pub rfrd_doc_amt: Option<RemittanceAmount2>,
	#[serde(rename = "CdtrRefInf")]
	pub cdtr_ref_inf: Option<CreditorReferenceInformation2>,
	#[serde(rename = "Invcr")]
	pub invcr: Option<PartyIdentification135>,
	#[serde(rename = "Invcee")]
	pub invcee: Option<PartyIdentification135>,
	#[serde(rename = "TaxRmt")]
	pub tax_rmt: Option<TaxData1>,
	#[serde(rename = "GrnshmtRmt")]
	pub grnshmt_rmt: Option<Garnishment3>,
	#[serde(rename = "AddtlRmtInf")]
	pub addtl_rmt_inf: Option<Vec<String>>,
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


// TaxAmount3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxAmount3 {
	#[serde(rename = "Rate")]
	pub rate: Option<f64>,
	#[serde(rename = "TaxblBaseAmt")]
	pub taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TtlAmt")]
	pub ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Dtls")]
	pub dtls: Option<Vec<TaxRecordDetails3>>,
}


// TaxAmountAndType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxAmountAndType1 {
	#[serde(rename = "Tp")]
	pub tp: Option<TaxAmountType1Choice>,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}


// TaxAmountType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxAmountType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// TaxAuthorisation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxAuthorisation1 {
	#[serde(rename = "Titl")]
	pub titl: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
}


// TaxData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxData1 {
	#[serde(rename = "Cdtr")]
	pub cdtr: Option<TaxParty1>,
	#[serde(rename = "Dbtr")]
	pub dbtr: Option<TaxParty2>,
	#[serde(rename = "UltmtDbtr")]
	pub ultmt_dbtr: Option<TaxParty2>,
	#[serde(rename = "AdmstnZone")]
	pub admstn_zone: Option<String>,
	#[serde(rename = "RefNb")]
	pub ref_nb: Option<String>,
	#[serde(rename = "Mtd")]
	pub mtd: Option<String>,
	#[serde(rename = "TtlTaxblBaseAmt")]
	pub ttl_taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TtlTaxAmt")]
	pub ttl_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "SeqNb")]
	pub seq_nb: Option<f64>,
	#[serde(rename = "Rcrd")]
	pub rcrd: Option<Vec<TaxRecord3>>,
}


// TaxParty1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxParty1 {
	#[serde(rename = "TaxId")]
	pub tax_id: Option<String>,
	#[serde(rename = "RegnId")]
	pub regn_id: Option<String>,
	#[serde(rename = "TaxTp")]
	pub tax_tp: Option<String>,
}


// TaxParty2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxParty2 {
	#[serde(rename = "TaxId")]
	pub tax_id: Option<String>,
	#[serde(rename = "RegnId")]
	pub regn_id: Option<String>,
	#[serde(rename = "TaxTp")]
	pub tax_tp: Option<String>,
	#[serde(rename = "Authstn")]
	pub authstn: Option<TaxAuthorisation1>,
}


// TaxPeriod3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxPeriod3 {
	#[serde(rename = "Yr")]
	pub yr: Option<String>,
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "FrToDt")]
	pub fr_to_dt: Option<DatePeriod2>,
}


// TaxRecord3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxRecord3 {
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "Ctgy")]
	pub ctgy: Option<String>,
	#[serde(rename = "CtgyDtls")]
	pub ctgy_dtls: Option<String>,
	#[serde(rename = "DbtrSts")]
	pub dbtr_sts: Option<String>,
	#[serde(rename = "CertId")]
	pub cert_id: Option<String>,
	#[serde(rename = "FrmsCd")]
	pub frms_cd: Option<String>,
	#[serde(rename = "Prd")]
	pub prd: Option<TaxPeriod3>,
	#[serde(rename = "TaxAmt")]
	pub tax_amt: Option<TaxAmount3>,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<String>,
}


// TaxRecordDetails3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxRecordDetails3 {
	#[serde(rename = "Prd")]
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


// UnderlyingData2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingData2Choice {
	#[serde(rename = "Initn")]
	pub initn: Option<UnderlyingPaymentInstruction8>,
	#[serde(rename = "IntrBk")]
	pub intr_bk: Option<UnderlyingPaymentTransaction7>,
	#[serde(rename = "StmtNtry")]
	pub stmt_ntry: Option<UnderlyingStatementEntry5>,
	#[serde(rename = "Acct")]
	pub acct: Option<CashAccount40>,
	#[serde(rename = "Othr")]
	pub othr: Option<GenericIdentification1>,
}


// UnderlyingGroupInformation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingGroupInformation1 {
	#[serde(rename = "OrgnlMsgId")]
	pub orgnl_msg_id: String,
	#[serde(rename = "OrgnlMsgNmId")]
	pub orgnl_msg_nm_id: String,
	#[serde(rename = "OrgnlCreDtTm")]
	pub orgnl_cre_dt_tm: Option<String>,
	#[serde(rename = "OrgnlMsgDlvryChanl")]
	pub orgnl_msg_dlvry_chanl: Option<String>,
}


// UnderlyingInvestigationInstrument1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingInvestigationInstrument1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// UnderlyingPaymentInstruction8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingPaymentInstruction8 {
	#[serde(rename = "OrgnlGrpInf")]
	pub orgnl_grp_inf: Option<UnderlyingGroupInformation1>,
	#[serde(rename = "OrgnlPmtInfId")]
	pub orgnl_pmt_inf_id: Option<String>,
	#[serde(rename = "OrgnlInstrId")]
	pub orgnl_instr_id: Option<String>,
	#[serde(rename = "OrgnlEndToEndId")]
	pub orgnl_end_to_end_id: Option<String>,
	#[serde(rename = "OrgnlUETR")]
	pub orgnl_uetr: Option<String>,
	#[serde(rename = "OrgnlInstdAmt")]
	pub orgnl_instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "ReqdExctnDt")]
	pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "ReqdColltnDt")]
	pub reqd_colltn_dt: Option<String>,
	#[serde(rename = "OrgnlTxRef")]
	pub orgnl_tx_ref: Option<OriginalTransactionReference35>,
	#[serde(rename = "OrgnlSvcLvl")]
	pub orgnl_svc_lvl: Option<ServiceLevel8Choice>,
}


// UnderlyingPaymentTransaction7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingPaymentTransaction7 {
	#[serde(rename = "OrgnlGrpInf")]
	pub orgnl_grp_inf: Option<UnderlyingGroupInformation1>,
	#[serde(rename = "OrgnlInstrId")]
	pub orgnl_instr_id: Option<String>,
	#[serde(rename = "OrgnlEndToEndId")]
	pub orgnl_end_to_end_id: Option<String>,
	#[serde(rename = "OrgnlTxId")]
	pub orgnl_tx_id: Option<String>,
	#[serde(rename = "OrgnlUETR")]
	pub orgnl_uetr: Option<String>,
	#[serde(rename = "OrgnlIntrBkSttlmAmt")]
	pub orgnl_intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "OrgnlIntrBkSttlmDt")]
	pub orgnl_intr_bk_sttlm_dt: Option<String>,
	#[serde(rename = "OrgnlTxRef")]
	pub orgnl_tx_ref: Option<OriginalTransactionReference35>,
	#[serde(rename = "OrgnlSvcLvl")]
	pub orgnl_svc_lvl: Option<ServiceLevel8Choice>,
}


// UnderlyingStatementEntry5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingStatementEntry5 {
	#[serde(rename = "OrgnlAcct")]
	pub orgnl_acct: Option<CashAccount40>,
	#[serde(rename = "OrgnlGrpInf")]
	pub orgnl_grp_inf: Option<OriginalGroupInformation29>,
	#[serde(rename = "OrgnlStmtId")]
	pub orgnl_stmt_id: Option<String>,
	#[serde(rename = "OrgnlNtryRef")]
	pub orgnl_ntry_ref: Option<String>,
	#[serde(rename = "OrgnlUETR")]
	pub orgnl_uetr: Option<String>,
	#[serde(rename = "OrgnlNtryAmt")]
	pub orgnl_ntry_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "OrgnlNtryValDt")]
	pub orgnl_ntry_val_dt: Option<DateAndDateTime2Choice>,
}
