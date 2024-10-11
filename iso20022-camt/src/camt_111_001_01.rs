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
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// AdjustmentCompensation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdjustmentCompensation1 {
	#[serde(rename = "InitlAmt", skip_serializing_if = "Option::is_none")]
	pub initl_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "DueChrgs", skip_serializing_if = "Option::is_none")]
	pub due_chrgs: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "AmtDue")]
	pub amt_due: ActiveCurrencyAndAmount,
	#[serde(rename = "CompstnAgt", skip_serializing_if = "Option::is_none")]
	pub compstn_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "CompstnAcct", skip_serializing_if = "Option::is_none")]
	pub compstn_acct: Option<CashAccount40>,
	#[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
	pub prd: Option<DatePeriod5>,
	#[serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none")]
	pub intrst_rate: Option<f64>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<String>,
}


// AmendmentInformationDetails14 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmendmentInformationDetails14 {
	#[serde(rename = "OrgnlMndtId", skip_serializing_if = "Option::is_none")]
	pub orgnl_mndt_id: Option<String>,
	#[serde(rename = "OrgnlCdtrSchmeId", skip_serializing_if = "Option::is_none")]
	pub orgnl_cdtr_schme_id: Option<PartyIdentification135>,
	#[serde(rename = "OrgnlCdtrAgt", skip_serializing_if = "Option::is_none")]
	pub orgnl_cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "OrgnlCdtrAgtAcct", skip_serializing_if = "Option::is_none")]
	pub orgnl_cdtr_agt_acct: Option<CashAccount40>,
	#[serde(rename = "OrgnlDbtr", skip_serializing_if = "Option::is_none")]
	pub orgnl_dbtr: Option<PartyIdentification135>,
	#[serde(rename = "OrgnlDbtrAcct", skip_serializing_if = "Option::is_none")]
	pub orgnl_dbtr_acct: Option<CashAccount40>,
	#[serde(rename = "OrgnlDbtrAgt", skip_serializing_if = "Option::is_none")]
	pub orgnl_dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
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


// AmountType4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountType4Choice {
	#[serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none")]
	pub instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "EqvtAmt", skip_serializing_if = "Option::is_none")]
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


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BaseOneRate {
	#[serde(rename = "BaseOneRate")]
	pub base_one_rate: f64,
}


// BookingConfirmation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BookingConfirmation1 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: String,
	#[serde(rename = "XchgRate", skip_serializing_if = "Option::is_none")]
	pub xchg_rate: Option<f64>,
	#[serde(rename = "Acct")]
	pub acct: CashAccount40,
	#[serde(rename = "BookgDt", skip_serializing_if = "Option::is_none")]
	pub bookg_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "ValDt")]
	pub val_dt: DateAndDateTime2Choice,
	#[serde(rename = "Refs")]
	pub refs: TransactionReferences6,
	#[serde(rename = "Chrgs", skip_serializing_if = "Option::is_none")]
	pub chrgs: Option<Charges6>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<String>,
}


// BranchAndFinancialInstitutionIdentification6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BranchAndFinancialInstitutionIdentification6 {
	#[serde(rename = "FinInstnId")]
	pub fin_instn_id: FinancialInstitutionIdentification18,
	#[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
	pub brnch_id: Option<BranchData3>,
}


// BranchData3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BranchData3 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<String>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress24>,
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


// ChargeIncludedIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ChargeIncludedIndicator {
	#[serde(rename = "ChargeIncludedIndicator")]
	pub charge_included_indicator: bool,
}


// ChargeType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ChargeType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification3>,
}


// Charges6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Charges6 {
	#[serde(rename = "TtlChrgsAndTaxAmt", skip_serializing_if = "Option::is_none")]
	pub ttl_chrgs_and_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Rcrd", skip_serializing_if = "Option::is_none")]
	pub rcrd: Option<Vec<ChargesRecord3>>,
}


// ChargesRecord3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ChargesRecord3 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt_ind: Option<String>,
	#[serde(rename = "ChrgInclInd", skip_serializing_if = "Option::is_none")]
	pub chrg_incl_ind: Option<bool>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<ChargeType3Choice>,
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "Br", skip_serializing_if = "Option::is_none")]
	pub br: Option<String>,
	#[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
	pub agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
	pub tax: Option<TaxCharges2>,
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


// CompensationResponse1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompensationResponse1 {
	#[serde(rename = "Grantd")]
	pub grantd: bool,
	#[serde(rename = "InitlAmt", skip_serializing_if = "Option::is_none")]
	pub initl_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "PdChrgs", skip_serializing_if = "Option::is_none")]
	pub pd_chrgs: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "AmtDue", skip_serializing_if = "Option::is_none")]
	pub amt_due: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "XpctdValDt", skip_serializing_if = "Option::is_none")]
	pub xpctd_val_dt: Option<String>,
	#[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
	pub prd: Option<DatePeriod2>,
	#[serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none")]
	pub intrst_rate: Option<f64>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<String>,
}


// Contact4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Contact4 {
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


// CreditTransferMandateData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditTransferMandateData1 {
	#[serde(rename = "MndtId", skip_serializing_if = "Option::is_none")]
	pub mndt_id: Option<String>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<MandateTypeInformation2>,
	#[serde(rename = "DtOfSgntr", skip_serializing_if = "Option::is_none")]
	pub dt_of_sgntr: Option<String>,
	#[serde(rename = "DtOfVrfctn", skip_serializing_if = "Option::is_none")]
	pub dt_of_vrfctn: Option<String>,
	#[serde(rename = "ElctrncSgntr", skip_serializing_if = "Option::is_none")]
	pub elctrnc_sgntr: Option<String>,
	#[serde(rename = "FrstPmtDt", skip_serializing_if = "Option::is_none")]
	pub frst_pmt_dt: Option<String>,
	#[serde(rename = "FnlPmtDt", skip_serializing_if = "Option::is_none")]
	pub fnl_pmt_dt: Option<String>,
	#[serde(rename = "Frqcy", skip_serializing_if = "Option::is_none")]
	pub frqcy: Option<Frequency36Choice>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<MandateSetupReason1Choice>,
}


// CreditorReferenceInformation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceInformation2 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<CreditorReferenceType2>,
	#[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
	pub ref_attr: Option<String>,
}


// CreditorReferenceType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// CreditorReferenceType2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceType2 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: CreditorReferenceType1Choice,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
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


// DebitAuthorisationConfirmation3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DebitAuthorisationConfirmation3 {
	#[serde(rename = "DbtAuthstn")]
	pub dbt_authstn: bool,
	#[serde(rename = "AmtToDbt", skip_serializing_if = "Option::is_none")]
	pub amt_to_dbt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Acct", skip_serializing_if = "Option::is_none")]
	pub acct: Option<CashAccount40>,
	#[serde(rename = "ValDtToDbt", skip_serializing_if = "Option::is_none")]
	pub val_dt_to_dbt: Option<String>,
	#[serde(rename = "CmonTxId", skip_serializing_if = "Option::is_none")]
	pub cmon_tx_id: Option<String>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<String>,
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
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<DiscountAmountType1Choice>,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}


// DiscountAmountType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DiscountAmountType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
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
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
	#[serde(rename = "LangCd", skip_serializing_if = "Option::is_none")]
	pub lang_cd: Option<String>,
	#[serde(rename = "Frmt")]
	pub frmt: DocumentFormat1Choice,
	#[serde(rename = "FileNm", skip_serializing_if = "Option::is_none")]
	pub file_nm: Option<String>,
	#[serde(rename = "DgtlSgntr", skip_serializing_if = "Option::is_none")]
	pub dgtl_sgntr: Option<PartyAndSignature3>,
	#[serde(rename = "Nclsr")]
	pub nclsr: String,
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


// DocumentFormat1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentFormat1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification1>,
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


// DocumentLineInformation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentLineInformation1 {
	#[serde(rename = "Id")]
	pub id: Vec<DocumentLineIdentification1>,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<String>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<RemittanceAmount3>,
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


// DocumentType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
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


// ExternalInvestigationStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalInvestigationStatus1Code {
	#[serde(rename = "ExternalInvestigationStatus1Code")]
	pub external_investigation_status1_code: String,
}


// ExternalInvestigationStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalInvestigationStatusReason1Code {
	#[serde(rename = "ExternalInvestigationStatusReason1Code")]
	pub external_investigation_status_reason1_code: String,
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


// ExternalStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalStatusReason1Code {
	#[serde(rename = "ExternalStatusReason1Code")]
	pub external_status_reason1_code: String,
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
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<DocumentType1Choice>,
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "IsseDt", skip_serializing_if = "Option::is_none")]
	pub isse_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "Frmt", skip_serializing_if = "Option::is_none")]
	pub frmt: Option<DocumentFormat1Choice>,
	#[serde(rename = "FileNm", skip_serializing_if = "Option::is_none")]
	pub file_nm: Option<String>,
	#[serde(rename = "NtwkRef", skip_serializing_if = "Option::is_none")]
	pub ntwk_ref: Option<String>,
	#[serde(rename = "FileLctnElctrncAdr", skip_serializing_if = "Option::is_none")]
	pub file_lctn_elctrnc_adr: Option<String>,
}


// FinancialIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// FinancialInstitutionIdentification18 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstitutionIdentification18 {
	#[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
	pub bicfi: Option<String>,
	#[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<String>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress24>,
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


// Garnishment3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Garnishment3 {
	#[serde(rename = "Tp")]
	pub tp: GarnishmentType1,
	#[serde(rename = "Grnshee", skip_serializing_if = "Option::is_none")]
	pub grnshee: Option<PartyIdentification135>,
	#[serde(rename = "GrnshmtAdmstr", skip_serializing_if = "Option::is_none")]
	pub grnshmt_admstr: Option<PartyIdentification135>,
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


// GenericIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification3 {
	#[serde(rename = "Id")]
	pub id: String,
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


// GenericOrganisationIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericOrganisationIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}


// GenericPersonIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericPersonIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
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
	#[serde(rename = "Orgtr", skip_serializing_if = "Option::is_none")]
	pub orgtr: Option<PartyIdentification135>,
	#[serde(rename = "Rsn")]
	pub rsn: InvestigationActionReason1Choice,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<String>>,
}


// InvestigationActionReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationActionReason1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// InvestigationData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationData2 {
	#[serde(rename = "OrgnlInvstgtnSeq", skip_serializing_if = "Option::is_none")]
	pub orgnl_invstgtn_seq: Option<f64>,
	#[serde(rename = "OrgnlInvstgtnRsn", skip_serializing_if = "Option::is_none")]
	pub orgnl_invstgtn_rsn: Option<InvestigationReason1Choice>,
	#[serde(rename = "OrgnlInvstgtnRsnSubTp", skip_serializing_if = "Option::is_none")]
	pub orgnl_invstgtn_rsn_sub_tp: Option<InvestigationReasonSubType1Choice>,
	#[serde(rename = "RspnData")]
	pub rspn_data: InvestigationDataRecord1Choice,
	#[serde(rename = "RltdInvstgtnData", skip_serializing_if = "Option::is_none")]
	pub rltd_invstgtn_data: Option<RelatedInvestigationData1>,
	#[serde(rename = "NclsdFile", skip_serializing_if = "Option::is_none")]
	pub nclsd_file: Option<Vec<Document12>>,
	#[serde(rename = "RltdFileData", skip_serializing_if = "Option::is_none")]
	pub rltd_file_data: Option<Vec<FileData1>>,
	#[serde(rename = "RspnOrgtr", skip_serializing_if = "Option::is_none")]
	pub rspn_orgtr: Option<Party40Choice>,
}


// InvestigationDataRecord1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationDataRecord1Choice {
	#[serde(rename = "DbtAuthstn", skip_serializing_if = "Option::is_none")]
	pub dbt_authstn: Option<DebitAuthorisationConfirmation3>,
	#[serde(rename = "Compstn", skip_serializing_if = "Option::is_none")]
	pub compstn: Option<CompensationResponse1>,
	#[serde(rename = "Valtn", skip_serializing_if = "Option::is_none")]
	pub valtn: Option<AdjustmentCompensation1>,
	#[serde(rename = "Conf", skip_serializing_if = "Option::is_none")]
	pub conf: Option<BookingConfirmation1>,
	#[serde(rename = "TxSts", skip_serializing_if = "Option::is_none")]
	pub tx_sts: Option<PaymentTransactionStatus1>,
	#[serde(rename = "TxData", skip_serializing_if = "Option::is_none")]
	pub tx_data: Option<Vec<TransactionAmendment1>>,
	#[serde(rename = "RspnNrrtv", skip_serializing_if = "Option::is_none")]
	pub rspn_nrrtv: Option<String>,
}


// InvestigationLocationData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationLocationData1 {
	#[serde(rename = "Mtd")]
	pub mtd: String,
	#[serde(rename = "ElctrncAdr", skip_serializing_if = "Option::is_none")]
	pub elctrnc_adr: Option<String>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
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
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// InvestigationReasonSubType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationReasonSubType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// InvestigationRequest3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationRequest3 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "RqstrInvstgtnId", skip_serializing_if = "Option::is_none")]
	pub rqstr_invstgtn_id: Option<String>,
	#[serde(rename = "RspndrInvstgtnId", skip_serializing_if = "Option::is_none")]
	pub rspndr_invstgtn_id: Option<String>,
	#[serde(rename = "EIR", skip_serializing_if = "Option::is_none")]
	pub eir: Option<String>,
	#[serde(rename = "ReqActn", skip_serializing_if = "Option::is_none")]
	pub req_actn: Option<InvestigationRequestAction1>,
	#[serde(rename = "InvstgtnTp")]
	pub invstgtn_tp: InvestigationType1Choice,
	#[serde(rename = "InvstgtnSubTp", skip_serializing_if = "Option::is_none")]
	pub invstgtn_sub_tp: Option<InvestigationSubType1Choice>,
	#[serde(rename = "UndrlygInstrm", skip_serializing_if = "Option::is_none")]
	pub undrlyg_instrm: Option<UnderlyingInvestigationInstrument1Choice>,
	#[serde(rename = "Undrlyg", skip_serializing_if = "Option::is_none")]
	pub undrlyg: Option<UnderlyingData2Choice>,
	#[serde(rename = "Rqstr")]
	pub rqstr: Party40Choice,
	#[serde(rename = "Rspndr")]
	pub rspndr: Party40Choice,
	#[serde(rename = "ReqOrgtr", skip_serializing_if = "Option::is_none")]
	pub req_orgtr: Option<Party40Choice>,
	#[serde(rename = "XpctdRspndr", skip_serializing_if = "Option::is_none")]
	pub xpctd_rspndr: Option<Party40Choice>,
	#[serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none")]
	pub svc_lvl: Option<Vec<InvestigationServiceLevel1Choice>>,
}


// InvestigationRequestAction1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationRequestAction1 {
	#[serde(rename = "Actn")]
	pub actn: InvestigationRequestAction1Choice,
	#[serde(rename = "ActnRsn", skip_serializing_if = "Option::is_none")]
	pub actn_rsn: Option<InvestigationActionReason1>,
}


// InvestigationRequestAction1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationRequestAction1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// InvestigationResponse3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationResponse3 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "RspndrInvstgtnId", skip_serializing_if = "Option::is_none")]
	pub rspndr_invstgtn_id: Option<String>,
	#[serde(rename = "InvstgtnSts")]
	pub invstgtn_sts: InvestigationStatus2,
	#[serde(rename = "NxtRspndr", skip_serializing_if = "Option::is_none")]
	pub nxt_rspndr: Option<Party40Choice>,
	#[serde(rename = "InvstgtnData", skip_serializing_if = "Option::is_none")]
	pub invstgtn_data: Option<Vec<InvestigationData2>>,
}


// InvestigationResponseV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationResponseV01 {
	#[serde(rename = "InvstgtnRspn")]
	pub invstgtn_rspn: InvestigationResponse3,
	#[serde(rename = "OrgnlInvstgtnReq")]
	pub orgnl_invstgtn_req: InvestigationRequest3,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// InvestigationServiceLevel1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationServiceLevel1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// InvestigationStatus2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationStatus2 {
	#[serde(rename = "Sts")]
	pub sts: String,
	#[serde(rename = "StsRsn", skip_serializing_if = "Option::is_none")]
	pub sts_rsn: Option<InvestigationStatusReason1Choice>,
}


// InvestigationStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationStatusReason1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// InvestigationSubType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationSubType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// InvestigationType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestigationType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
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
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// MandateClassification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MandateClassification1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
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
	#[serde(rename = "DrctDbtMndt", skip_serializing_if = "Option::is_none")]
	pub drct_dbt_mndt: Option<MandateRelatedInformation15>,
	#[serde(rename = "CdtTrfMndt", skip_serializing_if = "Option::is_none")]
	pub cdt_trf_mndt: Option<CreditTransferMandateData1>,
}


// MandateRelatedInformation15 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MandateRelatedInformation15 {
	#[serde(rename = "MndtId", skip_serializing_if = "Option::is_none")]
	pub mndt_id: Option<String>,
	#[serde(rename = "DtOfSgntr", skip_serializing_if = "Option::is_none")]
	pub dt_of_sgntr: Option<String>,
	#[serde(rename = "AmdmntInd", skip_serializing_if = "Option::is_none")]
	pub amdmnt_ind: Option<bool>,
	#[serde(rename = "AmdmntInfDtls", skip_serializing_if = "Option::is_none")]
	pub amdmnt_inf_dtls: Option<AmendmentInformationDetails14>,
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


// MandateTypeInformation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MandateTypeInformation2 {
	#[serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none")]
	pub svc_lvl: Option<ServiceLevel8Choice>,
	#[serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none")]
	pub lcl_instrm: Option<LocalInstrument2Choice>,
	#[serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none")]
	pub ctgy_purp: Option<CategoryPurpose1Choice>,
	#[serde(rename = "Clssfctn", skip_serializing_if = "Option::is_none")]
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


// Max52Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max52Text {
	#[serde(rename = "Max52Text")]
	pub max52_text: String,
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
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<String>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<String>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<GenericOrganisationIdentification1>>,
}


// OrganisationIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// OriginalGroupInformation29 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OriginalGroupInformation29 {
	#[serde(rename = "OrgnlMsgId")]
	pub orgnl_msg_id: String,
	#[serde(rename = "OrgnlMsgNmId")]
	pub orgnl_msg_nm_id: String,
	#[serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none")]
	pub orgnl_cre_dt_tm: Option<String>,
}


// OriginalTransactionReference35 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OriginalTransactionReference35 {
	#[serde(rename = "IntrBkSttlmAmt", skip_serializing_if = "Option::is_none")]
	pub intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<AmountType4Choice>,
	#[serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
	pub intr_bk_sttlm_dt: Option<String>,
	#[serde(rename = "ReqdColltnDt", skip_serializing_if = "Option::is_none")]
	pub reqd_colltn_dt: Option<String>,
	#[serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none")]
	pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "CdtrSchmeId", skip_serializing_if = "Option::is_none")]
	pub cdtr_schme_id: Option<PartyIdentification135>,
	#[serde(rename = "SttlmInf", skip_serializing_if = "Option::is_none")]
	pub sttlm_inf: Option<SettlementInstruction11>,
	#[serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none")]
	pub pmt_tp_inf: Option<PaymentTypeInformation27>,
	#[serde(rename = "PmtMtd", skip_serializing_if = "Option::is_none")]
	pub pmt_mtd: Option<String>,
	#[serde(rename = "MndtRltdInf", skip_serializing_if = "Option::is_none")]
	pub mndt_rltd_inf: Option<MandateRelatedData2Choice>,
	#[serde(rename = "RmtInf", skip_serializing_if = "Option::is_none")]
	pub rmt_inf: Option<RemittanceInformation21>,
	#[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
	pub ultmt_dbtr: Option<Party40Choice>,
	#[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
	pub dbtr: Option<Party40Choice>,
	#[serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none")]
	pub dbtr_acct: Option<CashAccount40>,
	#[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none")]
	pub dbtr_agt_acct: Option<CashAccount40>,
	#[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none")]
	pub cdtr_agt_acct: Option<CashAccount40>,
	#[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
	pub cdtr: Option<Party40Choice>,
	#[serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none")]
	pub cdtr_acct: Option<CashAccount40>,
	#[serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none")]
	pub ultmt_cdtr: Option<Party40Choice>,
	#[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
	pub purp: Option<Purpose2Choice>,
}


// OtherContact1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherContact1 {
	#[serde(rename = "ChanlTp")]
	pub chanl_tp: String,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
}


// Party38Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party38Choice {
	#[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
	pub org_id: Option<OrganisationIdentification29>,
	#[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
	pub prvt_id: Option<PersonIdentification13>,
}


// Party40Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party40Choice {
	#[serde(rename = "Pty", skip_serializing_if = "Option::is_none")]
	pub pty: Option<PartyIdentification135>,
	#[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
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
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress24>,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Party38Choice>,
	#[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
	pub ctry_of_res: Option<String>,
	#[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
	pub ctct_dtls: Option<Contact4>,
}


// PaymentMethod4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentMethod4Code {
	#[serde(rename = "PaymentMethod4Code")]
	pub payment_method4_code: String,
}


// PaymentTransactionStatus1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentTransactionStatus1 {
	#[serde(rename = "Sts")]
	pub sts: TransactionStatus1Choice,
	#[serde(rename = "StsRsnInf", skip_serializing_if = "Option::is_none")]
	pub sts_rsn_inf: Option<Vec<StatusReasonInformation12>>,
}


// PaymentTypeInformation27 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentTypeInformation27 {
	#[serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none")]
	pub instr_prty: Option<String>,
	#[serde(rename = "ClrChanl", skip_serializing_if = "Option::is_none")]
	pub clr_chanl: Option<String>,
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


// PersonIdentification13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentification13 {
	#[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
	pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<GenericPersonIdentification1>>,
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


// PostalAddress24 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress24 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<AddressType3Choice>,
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


// ProprietaryReference1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryReference1 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "Ref")]
	pub ref_attr: String,
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


// ReferredDocumentInformation7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReferredDocumentInformation7 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<ReferredDocumentType4>,
	#[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
	pub nb: Option<String>,
	#[serde(rename = "RltdDt", skip_serializing_if = "Option::is_none")]
	pub rltd_dt: Option<String>,
	#[serde(rename = "LineDtls", skip_serializing_if = "Option::is_none")]
	pub line_dtls: Option<Vec<DocumentLineInformation1>>,
}


// ReferredDocumentType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReferredDocumentType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// ReferredDocumentType4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReferredDocumentType4 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: ReferredDocumentType3Choice,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}


// RelatedInvestigationData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RelatedInvestigationData1 {
	#[serde(rename = "InvstgtnId", skip_serializing_if = "Option::is_none")]
	pub invstgtn_id: Option<String>,
	#[serde(rename = "Lctn", skip_serializing_if = "Option::is_none")]
	pub lctn: Option<Vec<InvestigationLocationData1>>,
}


// Remittance1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Remittance1 {
	#[serde(rename = "Ustrd", skip_serializing_if = "Option::is_none")]
	pub ustrd: Option<Vec<String>>,
	#[serde(rename = "Strd", skip_serializing_if = "Option::is_none")]
	pub strd: Option<Vec<StructuredRemittanceInformation16>>,
	#[serde(rename = "Rltd", skip_serializing_if = "Option::is_none")]
	pub rltd: Option<Vec<RemittanceLocation7>>,
}


// RemittanceAmount2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceAmount2 {
	#[serde(rename = "DuePyblAmt", skip_serializing_if = "Option::is_none")]
	pub due_pybl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "DscntApldAmt", skip_serializing_if = "Option::is_none")]
	pub dscnt_apld_amt: Option<Vec<DiscountAmountAndType1>>,
	#[serde(rename = "CdtNoteAmt", skip_serializing_if = "Option::is_none")]
	pub cdt_note_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none")]
	pub tax_amt: Option<Vec<TaxAmountAndType1>>,
	#[serde(rename = "AdjstmntAmtAndRsn", skip_serializing_if = "Option::is_none")]
	pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment1>>,
	#[serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none")]
	pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// RemittanceAmount3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceAmount3 {
	#[serde(rename = "DuePyblAmt", skip_serializing_if = "Option::is_none")]
	pub due_pybl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "DscntApldAmt", skip_serializing_if = "Option::is_none")]
	pub dscnt_apld_amt: Option<Vec<DiscountAmountAndType1>>,
	#[serde(rename = "CdtNoteAmt", skip_serializing_if = "Option::is_none")]
	pub cdt_note_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none")]
	pub tax_amt: Option<Vec<TaxAmountAndType1>>,
	#[serde(rename = "AdjstmntAmtAndRsn", skip_serializing_if = "Option::is_none")]
	pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment1>>,
	#[serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none")]
	pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// RemittanceInformation21 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceInformation21 {
	#[serde(rename = "Ustrd", skip_serializing_if = "Option::is_none")]
	pub ustrd: Option<Vec<String>>,
	#[serde(rename = "Strd", skip_serializing_if = "Option::is_none")]
	pub strd: Option<Vec<StructuredRemittanceInformation17>>,
}


// RemittanceLocation7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceLocation7 {
	#[serde(rename = "RmtId", skip_serializing_if = "Option::is_none")]
	pub rmt_id: Option<String>,
	#[serde(rename = "RmtLctnDtls", skip_serializing_if = "Option::is_none")]
	pub rmt_lctn_dtls: Option<Vec<RemittanceLocationData1>>,
}


// RemittanceLocationData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceLocationData1 {
	#[serde(rename = "Mtd")]
	pub mtd: String,
	#[serde(rename = "ElctrncAdr", skip_serializing_if = "Option::is_none")]
	pub elctrnc_adr: Option<String>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<NameAndAddress16>,
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


// SettlementInstruction11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementInstruction11 {
	#[serde(rename = "SttlmMtd")]
	pub sttlm_mtd: String,
	#[serde(rename = "SttlmAcct", skip_serializing_if = "Option::is_none")]
	pub sttlm_acct: Option<CashAccount40>,
	#[serde(rename = "ClrSys", skip_serializing_if = "Option::is_none")]
	pub clr_sys: Option<ClearingSystemIdentification3Choice>,
	#[serde(rename = "InstgRmbrsmntAgt", skip_serializing_if = "Option::is_none")]
	pub instg_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "InstgRmbrsmntAgtAcct", skip_serializing_if = "Option::is_none")]
	pub instg_rmbrsmnt_agt_acct: Option<CashAccount40>,
	#[serde(rename = "InstdRmbrsmntAgt", skip_serializing_if = "Option::is_none")]
	pub instd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "InstdRmbrsmntAgtAcct", skip_serializing_if = "Option::is_none")]
	pub instd_rmbrsmnt_agt_acct: Option<CashAccount40>,
	#[serde(rename = "ThrdRmbrsmntAgt", skip_serializing_if = "Option::is_none")]
	pub thrd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "ThrdRmbrsmntAgtAcct", skip_serializing_if = "Option::is_none")]
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


// StatusReason6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatusReason6Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// StatusReasonInformation12 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatusReasonInformation12 {
	#[serde(rename = "Orgtr", skip_serializing_if = "Option::is_none")]
	pub orgtr: Option<PartyIdentification135>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<StatusReason6Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<String>>,
}


// StructuredRemittanceInformation16 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StructuredRemittanceInformation16 {
	#[serde(rename = "RfrdDocInf", skip_serializing_if = "Option::is_none")]
	pub rfrd_doc_inf: Option<Vec<ReferredDocumentInformation7>>,
	#[serde(rename = "RfrdDocAmt", skip_serializing_if = "Option::is_none")]
	pub rfrd_doc_amt: Option<RemittanceAmount2>,
	#[serde(rename = "CdtrRefInf", skip_serializing_if = "Option::is_none")]
	pub cdtr_ref_inf: Option<CreditorReferenceInformation2>,
	#[serde(rename = "Invcr", skip_serializing_if = "Option::is_none")]
	pub invcr: Option<PartyIdentification135>,
	#[serde(rename = "Invcee", skip_serializing_if = "Option::is_none")]
	pub invcee: Option<PartyIdentification135>,
	#[serde(rename = "TaxRmt", skip_serializing_if = "Option::is_none")]
	pub tax_rmt: Option<TaxInformation7>,
	#[serde(rename = "GrnshmtRmt", skip_serializing_if = "Option::is_none")]
	pub grnshmt_rmt: Option<Garnishment3>,
	#[serde(rename = "AddtlRmtInf", skip_serializing_if = "Option::is_none")]
	pub addtl_rmt_inf: Option<Vec<String>>,
}


// StructuredRemittanceInformation17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StructuredRemittanceInformation17 {
	#[serde(rename = "RfrdDocInf", skip_serializing_if = "Option::is_none")]
	pub rfrd_doc_inf: Option<Vec<ReferredDocumentInformation7>>,
	#[serde(rename = "RfrdDocAmt", skip_serializing_if = "Option::is_none")]
	pub rfrd_doc_amt: Option<RemittanceAmount2>,
	#[serde(rename = "CdtrRefInf", skip_serializing_if = "Option::is_none")]
	pub cdtr_ref_inf: Option<CreditorReferenceInformation2>,
	#[serde(rename = "Invcr", skip_serializing_if = "Option::is_none")]
	pub invcr: Option<PartyIdentification135>,
	#[serde(rename = "Invcee", skip_serializing_if = "Option::is_none")]
	pub invcee: Option<PartyIdentification135>,
	#[serde(rename = "TaxRmt", skip_serializing_if = "Option::is_none")]
	pub tax_rmt: Option<TaxData1>,
	#[serde(rename = "GrnshmtRmt", skip_serializing_if = "Option::is_none")]
	pub grnshmt_rmt: Option<Garnishment3>,
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


// TaxAmount2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxAmount2 {
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "TaxblBaseAmt", skip_serializing_if = "Option::is_none")]
	pub taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none")]
	pub ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Dtls", skip_serializing_if = "Option::is_none")]
	pub dtls: Option<Vec<TaxRecordDetails2>>,
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


// TaxAmountAndType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxAmountAndType1 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<TaxAmountType1Choice>,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}


// TaxAmountType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxAmountType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// TaxAuthorisation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxAuthorisation1 {
	#[serde(rename = "Titl", skip_serializing_if = "Option::is_none")]
	pub titl: Option<String>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
}


// TaxCharges2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxCharges2 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
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


// TaxInformation7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxInformation7 {
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
	pub rcrd: Option<Vec<TaxRecord2>>,
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


// TaxPeriod2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxPeriod2 {
	#[serde(rename = "Yr", skip_serializing_if = "Option::is_none")]
	pub yr: Option<String>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<String>,
	#[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
	pub fr_to_dt: Option<DatePeriod2>,
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


// TaxRecord2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxRecord2 {
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
	pub prd: Option<TaxPeriod2>,
	#[serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none")]
	pub tax_amt: Option<TaxAmount2>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<String>,
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


// TaxRecordDetails2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxRecordDetails2 {
	#[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
	pub prd: Option<TaxPeriod2>,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
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


// TransactionAmendment1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionAmendment1 {
	#[serde(rename = "Pth", skip_serializing_if = "Option::is_none")]
	pub pth: Option<String>,
	#[serde(rename = "Rcrd")]
	pub rcrd: TransactionAmendment1Choice,
}


// TransactionAmendment1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionAmendment1Choice {
	#[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
	pub agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<String>,
	#[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
	pub bicfi: Option<String>,
	#[serde(rename = "CshAcct", skip_serializing_if = "Option::is_none")]
	pub csh_acct: Option<CashAccount40>,
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<String>,
	#[serde(rename = "Pty", skip_serializing_if = "Option::is_none")]
	pub pty: Option<PartyIdentification135>,
	#[serde(rename = "Rmt", skip_serializing_if = "Option::is_none")]
	pub rmt: Option<Remittance1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<String>,
}


// TransactionReferences6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionReferences6 {
	#[serde(rename = "MsgId", skip_serializing_if = "Option::is_none")]
	pub msg_id: Option<String>,
	#[serde(rename = "AcctSvcrRef", skip_serializing_if = "Option::is_none")]
	pub acct_svcr_ref: Option<String>,
	#[serde(rename = "PmtInfId", skip_serializing_if = "Option::is_none")]
	pub pmt_inf_id: Option<String>,
	#[serde(rename = "InstrId", skip_serializing_if = "Option::is_none")]
	pub instr_id: Option<String>,
	#[serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none")]
	pub end_to_end_id: Option<String>,
	#[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
	pub uetr: Option<String>,
	#[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
	pub tx_id: Option<String>,
	#[serde(rename = "MndtId", skip_serializing_if = "Option::is_none")]
	pub mndt_id: Option<String>,
	#[serde(rename = "ChqNb", skip_serializing_if = "Option::is_none")]
	pub chq_nb: Option<String>,
	#[serde(rename = "ClrSysRef", skip_serializing_if = "Option::is_none")]
	pub clr_sys_ref: Option<String>,
	#[serde(rename = "AcctOwnrTxId", skip_serializing_if = "Option::is_none")]
	pub acct_ownr_tx_id: Option<String>,
	#[serde(rename = "AcctSvcrTxId", skip_serializing_if = "Option::is_none")]
	pub acct_svcr_tx_id: Option<String>,
	#[serde(rename = "MktInfrstrctrTxId", skip_serializing_if = "Option::is_none")]
	pub mkt_infrstrctr_tx_id: Option<String>,
	#[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
	pub prcg_id: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Vec<ProprietaryReference1>>,
}


// TransactionStatus1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionStatus1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
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
	#[serde(rename = "Initn", skip_serializing_if = "Option::is_none")]
	pub initn: Option<UnderlyingPaymentInstruction8>,
	#[serde(rename = "IntrBk", skip_serializing_if = "Option::is_none")]
	pub intr_bk: Option<UnderlyingPaymentTransaction7>,
	#[serde(rename = "StmtNtry", skip_serializing_if = "Option::is_none")]
	pub stmt_ntry: Option<UnderlyingStatementEntry5>,
	#[serde(rename = "Acct", skip_serializing_if = "Option::is_none")]
	pub acct: Option<CashAccount40>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<GenericIdentification1>,
}


// UnderlyingGroupInformation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingGroupInformation1 {
	#[serde(rename = "OrgnlMsgId")]
	pub orgnl_msg_id: String,
	#[serde(rename = "OrgnlMsgNmId")]
	pub orgnl_msg_nm_id: String,
	#[serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none")]
	pub orgnl_cre_dt_tm: Option<String>,
	#[serde(rename = "OrgnlMsgDlvryChanl", skip_serializing_if = "Option::is_none")]
	pub orgnl_msg_dlvry_chanl: Option<String>,
}


// UnderlyingInvestigationInstrument1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingInvestigationInstrument1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// UnderlyingPaymentInstruction8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingPaymentInstruction8 {
	#[serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none")]
	pub orgnl_grp_inf: Option<UnderlyingGroupInformation1>,
	#[serde(rename = "OrgnlPmtInfId", skip_serializing_if = "Option::is_none")]
	pub orgnl_pmt_inf_id: Option<String>,
	#[serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none")]
	pub orgnl_instr_id: Option<String>,
	#[serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none")]
	pub orgnl_end_to_end_id: Option<String>,
	#[serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none")]
	pub orgnl_uetr: Option<String>,
	#[serde(rename = "OrgnlInstdAmt", skip_serializing_if = "Option::is_none")]
	pub orgnl_instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none")]
	pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "ReqdColltnDt", skip_serializing_if = "Option::is_none")]
	pub reqd_colltn_dt: Option<String>,
	#[serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none")]
	pub orgnl_tx_ref: Option<OriginalTransactionReference35>,
	#[serde(rename = "OrgnlSvcLvl", skip_serializing_if = "Option::is_none")]
	pub orgnl_svc_lvl: Option<ServiceLevel8Choice>,
}


// UnderlyingPaymentTransaction7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingPaymentTransaction7 {
	#[serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none")]
	pub orgnl_grp_inf: Option<UnderlyingGroupInformation1>,
	#[serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none")]
	pub orgnl_instr_id: Option<String>,
	#[serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none")]
	pub orgnl_end_to_end_id: Option<String>,
	#[serde(rename = "OrgnlTxId", skip_serializing_if = "Option::is_none")]
	pub orgnl_tx_id: Option<String>,
	#[serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none")]
	pub orgnl_uetr: Option<String>,
	#[serde(rename = "OrgnlIntrBkSttlmAmt", skip_serializing_if = "Option::is_none")]
	pub orgnl_intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "OrgnlIntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
	pub orgnl_intr_bk_sttlm_dt: Option<String>,
	#[serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none")]
	pub orgnl_tx_ref: Option<OriginalTransactionReference35>,
	#[serde(rename = "OrgnlSvcLvl", skip_serializing_if = "Option::is_none")]
	pub orgnl_svc_lvl: Option<ServiceLevel8Choice>,
}


// UnderlyingStatementEntry5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingStatementEntry5 {
	#[serde(rename = "OrgnlAcct", skip_serializing_if = "Option::is_none")]
	pub orgnl_acct: Option<CashAccount40>,
	#[serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none")]
	pub orgnl_grp_inf: Option<OriginalGroupInformation29>,
	#[serde(rename = "OrgnlStmtId", skip_serializing_if = "Option::is_none")]
	pub orgnl_stmt_id: Option<String>,
	#[serde(rename = "OrgnlNtryRef", skip_serializing_if = "Option::is_none")]
	pub orgnl_ntry_ref: Option<String>,
	#[serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none")]
	pub orgnl_uetr: Option<String>,
	#[serde(rename = "OrgnlNtryAmt", skip_serializing_if = "Option::is_none")]
	pub orgnl_ntry_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "OrgnlNtryValDt", skip_serializing_if = "Option::is_none")]
	pub orgnl_ntry_val_dt: Option<DateAndDateTime2Choice>,
}
