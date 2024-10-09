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


// AmountOrRate1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountOrRate1Choice {
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Rate")]
	pub rate: Option<f64>,
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


// ChargeType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ChargeType3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification3>,
}


// Charges16 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Charges16 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "Agt")]
	pub agt: BranchAndFinancialInstitutionIdentification8,
	#[serde(rename = "Tp")]
	pub tp: Option<ChargeType3Choice>,
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


// CreditorPaymentActivationRequestStatusReportV11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorPaymentActivationRequestStatusReportV11 {
	#[serde(rename = "GrpHdr")]
	pub grp_hdr: GroupHeader111,
	#[serde(rename = "OrgnlGrpInfAndSts")]
	pub orgnl_grp_inf_and_sts: OriginalGroupInformation32,
	#[serde(rename = "OrgnlPmtInfAndSts")]
	pub orgnl_pmt_inf_and_sts: Option<Vec<OriginalPaymentInstruction47>>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// CreditorReferenceInformation3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceInformation3 {
	#[serde(rename = "Tp")]
	pub tp: Option<CreditorReferenceType3>,
	#[serde(rename = "Ref")]
	pub ref_attr: Option<String>,
}


// CreditorReferenceType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceType2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// CreditorReferenceType3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceType3 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: CreditorReferenceType2Choice,
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
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "DecimalNumber")]
	pub decimal_number: f64,
}


// Document15 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Document15 {
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
	pub dgtl_sgntr: Option<PartyAndSignature4>,
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
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
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


// DocumentLineInformation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentLineInformation2 {
	#[serde(rename = "Id")]
	pub id: Vec<DocumentLineIdentification1>,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
	#[serde(rename = "Amt")]
	pub amt: Option<RemittanceAmount4>,
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


// DocumentType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentType1 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: DocumentType2Choice,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// DocumentType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification1>,
}


// DocumentType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentType2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
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


// ExternalChargeType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalChargeType1Code {
	#[serde(rename = "ExternalChargeType1Code")]
	pub external_charge_type1_code: String,
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


// ExternalPaymentGroupStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPaymentGroupStatus1Code {
	#[serde(rename = "ExternalPaymentGroupStatus1Code")]
	pub external_payment_group_status1_code: String,
}


// ExternalPaymentTransactionStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPaymentTransactionStatus1Code {
	#[serde(rename = "ExternalPaymentTransactionStatus1Code")]
	pub external_payment_transaction_status1_code: String,
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


// ExternalServiceLevel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalServiceLevel1Code {
	#[serde(rename = "ExternalServiceLevel1Code")]
	pub external_service_level1_code: String,
}


// ExternalStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalStatusReason1Code {
	#[serde(rename = "ExternalStatusReason1Code")]
	pub external_status_reason1_code: String,
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


// Garnishment4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Garnishment4 {
	#[serde(rename = "Tp")]
	pub tp: GarnishmentType1,
	#[serde(rename = "Grnshee")]
	pub grnshee: Option<PartyIdentification272>,
	#[serde(rename = "GrnshmtAdmstr")]
	pub grnshmt_admstr: Option<PartyIdentification272>,
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


// GenericIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification3 {
	#[serde(rename = "Id")]
	pub id: String,
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


// GroupHeader111 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GroupHeader111 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
	#[serde(rename = "InitgPty")]
	pub initg_pty: PartyIdentification272,
	#[serde(rename = "FwdgAgt")]
	pub fwdg_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "DbtrAgt")]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "CdtrAgt")]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
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


// NumberOfTransactionsPerStatus5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NumberOfTransactionsPerStatus5 {
	#[serde(rename = "DtldNbOfTxs")]
	pub dtld_nb_of_txs: String,
	#[serde(rename = "DtldSts")]
	pub dtld_sts: String,
	#[serde(rename = "DtldCtrlSum")]
	pub dtld_ctrl_sum: Option<f64>,
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


// OriginalGroupInformation32 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OriginalGroupInformation32 {
	#[serde(rename = "OrgnlMsgId")]
	pub orgnl_msg_id: String,
	#[serde(rename = "OrgnlMsgNmId")]
	pub orgnl_msg_nm_id: String,
	#[serde(rename = "OrgnlCreDtTm")]
	pub orgnl_cre_dt_tm: Option<String>,
	#[serde(rename = "OrgnlNbOfTxs")]
	pub orgnl_nb_of_txs: Option<String>,
	#[serde(rename = "OrgnlCtrlSum")]
	pub orgnl_ctrl_sum: Option<f64>,
	#[serde(rename = "GrpSts")]
	pub grp_sts: Option<String>,
	#[serde(rename = "StsRsnInf")]
	pub sts_rsn_inf: Option<Vec<StatusReasonInformation14>>,
	#[serde(rename = "NbOfTxsPerSts")]
	pub nb_of_txs_per_sts: Option<Vec<NumberOfTransactionsPerStatus5>>,
}


// OriginalPaymentInstruction47 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OriginalPaymentInstruction47 {
	#[serde(rename = "OrgnlPmtInfId")]
	pub orgnl_pmt_inf_id: String,
	#[serde(rename = "OrgnlNbOfTxs")]
	pub orgnl_nb_of_txs: Option<String>,
	#[serde(rename = "OrgnlCtrlSum")]
	pub orgnl_ctrl_sum: Option<f64>,
	#[serde(rename = "PmtInfSts")]
	pub pmt_inf_sts: Option<String>,
	#[serde(rename = "StsRsnInf")]
	pub sts_rsn_inf: Option<Vec<StatusReasonInformation14>>,
	#[serde(rename = "NbOfTxsPerSts")]
	pub nb_of_txs_per_sts: Option<Vec<NumberOfTransactionsPerStatus5>>,
	#[serde(rename = "TxInfAndSts")]
	pub tx_inf_and_sts: Option<Vec<PaymentTransaction150>>,
}


// OriginalTransactionReference40 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OriginalTransactionReference40 {
	#[serde(rename = "Amt")]
	pub amt: Option<AmountType4Choice>,
	#[serde(rename = "ReqdExctnDt")]
	pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "XpryDt")]
	pub xpry_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "PmtCond")]
	pub pmt_cond: Option<PaymentCondition2>,
	#[serde(rename = "PmtTpInf")]
	pub pmt_tp_inf: Option<PaymentTypeInformation29>,
	#[serde(rename = "PmtMtd")]
	pub pmt_mtd: Option<String>,
	#[serde(rename = "MndtRltdInf")]
	pub mndt_rltd_inf: Option<CreditTransferMandateData1>,
	#[serde(rename = "RmtInf")]
	pub rmt_inf: Option<RemittanceInformation22>,
	#[serde(rename = "NclsdFile")]
	pub nclsd_file: Option<Vec<Document15>>,
	#[serde(rename = "UltmtDbtr")]
	pub ultmt_dbtr: Option<PartyIdentification272>,
	#[serde(rename = "Dbtr")]
	pub dbtr: Option<PartyIdentification272>,
	#[serde(rename = "DbtrAcct")]
	pub dbtr_acct: Option<CashAccount40>,
	#[serde(rename = "DbtrAgt")]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "DbtrAgtAcct")]
	pub dbtr_agt_acct: Option<CashAccount40>,
	#[serde(rename = "CdtrAgt")]
	pub cdtr_agt: BranchAndFinancialInstitutionIdentification8,
	#[serde(rename = "CdtrAgtAcct")]
	pub cdtr_agt_acct: Option<CashAccount40>,
	#[serde(rename = "Cdtr")]
	pub cdtr: PartyIdentification272,
	#[serde(rename = "CdtrAcct")]
	pub cdtr_acct: Option<CashAccount40>,
	#[serde(rename = "UltmtCdtr")]
	pub ultmt_cdtr: Option<PartyIdentification272>,
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


// PartyAndSignature4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyAndSignature4 {
	#[serde(rename = "Pty")]
	pub pty: PartyIdentification272,
	#[serde(rename = "Sgntr")]
	pub sgntr: SkipPayload,
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


// PaymentCondition2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentCondition2 {
	#[serde(rename = "AmtModAllwd")]
	pub amt_mod_allwd: Option<bool>,
	#[serde(rename = "EarlyPmtAllwd")]
	pub early_pmt_allwd: Option<bool>,
	#[serde(rename = "DelyPnlty")]
	pub dely_pnlty: Option<String>,
	#[serde(rename = "ImdtPmtRbt")]
	pub imdt_pmt_rbt: Option<AmountOrRate1Choice>,
	#[serde(rename = "GrntedPmtReqd")]
	pub grnted_pmt_reqd: Option<bool>,
}


// PaymentConditionStatus2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentConditionStatus2 {
	#[serde(rename = "AccptdAmt")]
	pub accptd_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "GrntedPmt")]
	pub grnted_pmt: Option<bool>,
	#[serde(rename = "EarlyPmt")]
	pub early_pmt: Option<bool>,
}


// PaymentMethod4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentMethod4Code {
	#[serde(rename = "PaymentMethod4Code")]
	pub payment_method4_code: String,
}


// PaymentTransaction150 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentTransaction150 {
	#[serde(rename = "StsId")]
	pub sts_id: Option<String>,
	#[serde(rename = "OrgnlInstrId")]
	pub orgnl_instr_id: Option<String>,
	#[serde(rename = "OrgnlEndToEndId")]
	pub orgnl_end_to_end_id: Option<String>,
	#[serde(rename = "OrgnlUETR")]
	pub orgnl_uetr: Option<String>,
	#[serde(rename = "TxSts")]
	pub tx_sts: Option<String>,
	#[serde(rename = "StsRsnInf")]
	pub sts_rsn_inf: Option<Vec<StatusReasonInformation14>>,
	#[serde(rename = "PmtCondSts")]
	pub pmt_cond_sts: Option<PaymentConditionStatus2>,
	#[serde(rename = "ChrgsInf")]
	pub chrgs_inf: Option<Vec<Charges16>>,
	#[serde(rename = "DbtrDcsnDtTm")]
	pub dbtr_dcsn_dt_tm: Option<String>,
	#[serde(rename = "AccptncDtTm")]
	pub accptnc_dt_tm: Option<String>,
	#[serde(rename = "AcctSvcrRef")]
	pub acct_svcr_ref: Option<String>,
	#[serde(rename = "ClrSysRef")]
	pub clr_sys_ref: Option<String>,
	#[serde(rename = "OrgnlTxRef")]
	pub orgnl_tx_ref: Option<OriginalTransactionReference40>,
	#[serde(rename = "NclsdFile")]
	pub nclsd_file: Option<Vec<Document15>>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// PaymentTypeInformation29 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentTypeInformation29 {
	#[serde(rename = "InstrPrty")]
	pub instr_prty: Option<String>,
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


// ReferredDocumentInformation8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReferredDocumentInformation8 {
	#[serde(rename = "Tp")]
	pub tp: Option<DocumentType1>,
	#[serde(rename = "Nb")]
	pub nb: Option<String>,
	#[serde(rename = "RltdDt")]
	pub rltd_dt: Option<DateAndType1>,
	#[serde(rename = "LineDtls")]
	pub line_dtls: Option<Vec<DocumentLineInformation2>>,
}


// RemittanceAmount4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceAmount4 {
	#[serde(rename = "RmtAmtAndTp")]
	pub rmt_amt_and_tp: Option<Vec<DocumentAmount1>>,
	#[serde(rename = "AdjstmntAmtAndRsn")]
	pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment1>>,
}


// RemittanceInformation22 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceInformation22 {
	#[serde(rename = "Ustrd")]
	pub ustrd: Option<Vec<String>>,
	#[serde(rename = "Strd")]
	pub strd: Option<Vec<StructuredRemittanceInformation18>>,
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


// SkipPayload ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SkipPayload {
}


// StatusReason6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatusReason6Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// StatusReasonInformation14 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatusReasonInformation14 {
	#[serde(rename = "Orgtr")]
	pub orgtr: Option<PartyIdentification272>,
	#[serde(rename = "Rsn")]
	pub rsn: Option<StatusReason6Choice>,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<Vec<String>>,
}


// StructuredRemittanceInformation18 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StructuredRemittanceInformation18 {
	#[serde(rename = "RfrdDocInf")]
	pub rfrd_doc_inf: Option<Vec<ReferredDocumentInformation8>>,
	#[serde(rename = "RfrdDocAmt")]
	pub rfrd_doc_amt: Option<RemittanceAmount4>,
	#[serde(rename = "CdtrRefInf")]
	pub cdtr_ref_inf: Option<CreditorReferenceInformation3>,
	#[serde(rename = "Invcr")]
	pub invcr: Option<PartyIdentification272>,
	#[serde(rename = "Invcee")]
	pub invcee: Option<PartyIdentification272>,
	#[serde(rename = "TaxRmt")]
	pub tax_rmt: Option<TaxData1>,
	#[serde(rename = "GrnshmtRmt")]
	pub grnshmt_rmt: Option<Garnishment4>,
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
