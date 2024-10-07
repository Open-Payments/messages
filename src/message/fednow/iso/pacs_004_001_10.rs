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
use serde_valid::Validate;


// AccountIdentification4Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountIdentification4Choice {
	#[serde(rename = "IBAN")]
	pub iban: Option<String>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<GenericAccountIdentification1>,
}


// AccountSchemeName1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "ActiveCurrencyAndAmount_SimpleType")]
	pub active_currency_and_amount_simple_type: f64,
}


// ActiveCurrencyAndAmount ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveCurrencyCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[validate(pattern = "[A-Z]{3,3}")]
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// ActiveOrHistoricCurrencyAndAmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
	#[serde(rename = "ActiveOrHistoricCurrencyAndAmount_SimpleType")]
	pub active_or_historic_currency_and_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAndAmount ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyCode {
	#[validate(pattern = "[A-Z]{3,3}")]
	#[serde(rename = "ActiveOrHistoricCurrencyCode")]
	pub active_or_historic_currency_code: String,
}


// AddressType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AddressType2Code {
	#[validate(enumerate = ["ADDR", "PBOX", "HOME", "BIZZ", "MLTO", "DLVY"])]
	#[serde(rename = "AddressType2Code")]
	pub address_type2_code: String,
}


// AddressType3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AddressType3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// AmendmentInformationDetails13 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AmendmentInformationDetails13 {
	#[serde(rename = "OrgnlMndtId")]
	pub orgnl_mndt_id: Option<String>,
	#[validate]
	#[serde(rename = "OrgnlCdtrSchmeId")]
	pub orgnl_cdtr_schme_id: Option<PartyIdentification135>,
	#[validate]
	#[serde(rename = "OrgnlCdtrAgt")]
	pub orgnl_cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[validate]
	#[serde(rename = "OrgnlCdtrAgtAcct")]
	pub orgnl_cdtr_agt_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "OrgnlDbtr")]
	pub orgnl_dbtr: Option<PartyIdentification135>,
	#[validate]
	#[serde(rename = "OrgnlDbtrAcct")]
	pub orgnl_dbtr_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "OrgnlDbtrAgt")]
	pub orgnl_dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[validate]
	#[serde(rename = "OrgnlDbtrAgtAcct")]
	pub orgnl_dbtr_agt_acct: Option<CashAccount38>,
	#[serde(rename = "OrgnlFnlColltnDt")]
	pub orgnl_fnl_colltn_dt: Option<String>,
	#[validate]
	#[serde(rename = "OrgnlFrqcy")]
	pub orgnl_frqcy: Option<Frequency36Choice>,
	#[validate]
	#[serde(rename = "OrgnlRsn")]
	pub orgnl_rsn: Option<MandateSetupReason1Choice>,
	#[serde(rename = "OrgnlTrckgDays")]
	pub orgnl_trckg_days: Option<String>,
}


// AmountType4Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AmountType4Choice {
	#[validate]
	#[serde(rename = "InstdAmt")]
	pub instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "EqvtAmt")]
	pub eqvt_amt: Option<EquivalentAmount2>,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[validate(pattern = "[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}")]
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// Authorisation1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Authorisation1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// Authorisation1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Authorisation1Code {
	#[validate(enumerate = ["AUTH", "FDET", "FSUM", "ILEV"])]
	#[serde(rename = "Authorisation1Code")]
	pub authorisation1_code: String,
}


// BICFIDec2014Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BICFIDec2014Identifier {
	#[validate(pattern = "[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}")]
	#[serde(rename = "BICFIDec2014Identifier")]
	pub bicfi_dec2014_identifier: String,
}


// BaseOneRate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BaseOneRate {
	#[serde(rename = "BaseOneRate")]
	pub base_one_rate: f64,
}


// BatchBookingIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BatchBookingIndicator {
	#[serde(rename = "BatchBookingIndicator")]
	pub batch_booking_indicator: bool,
}


// BranchAndFinancialInstitutionIdentification6 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BranchAndFinancialInstitutionIdentification6 {
	#[validate]
	#[serde(rename = "FinInstnId")]
	pub fin_instn_id: FinancialInstitutionIdentification18,
	#[validate]
	#[serde(rename = "BrnchId")]
	pub brnch_id: Option<BranchData3>,
}


// BranchData3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BranchData3 {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[validate]
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress24>,
}


// CashAccount38 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CashAccount38 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: AccountIdentification4Choice,
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: Option<CashAccountType2Choice>,
	#[serde(rename = "Ccy")]
	pub ccy: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[validate]
	#[serde(rename = "Prxy")]
	pub prxy: Option<ProxyAccountIdentification1>,
}


// CashAccountType2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CashAccountType2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// CategoryPurpose1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CategoryPurpose1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ChargeBearerType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ChargeBearerType1Code {
	#[validate(enumerate = ["DEBT", "CRED", "SHAR", "SLEV"])]
	#[serde(rename = "ChargeBearerType1Code")]
	pub charge_bearer_type1_code: String,
}


// Charges7 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Charges7 {
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[validate]
	#[serde(rename = "Agt")]
	pub agt: BranchAndFinancialInstitutionIdentification6,
}


// ClearingChannel2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ClearingChannel2Code {
	#[validate(enumerate = ["RTGS", "RTNS", "MPNS", "BOOK"])]
	#[serde(rename = "ClearingChannel2Code")]
	pub clearing_channel2_code: String,
}


// ClearingSystemIdentification2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ClearingSystemIdentification2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ClearingSystemIdentification3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ClearingSystemIdentification3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ClearingSystemMemberIdentification2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ClearingSystemMemberIdentification2 {
	#[validate]
	#[serde(rename = "ClrSysId")]
	pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
	#[serde(rename = "MmbId")]
	pub mmb_id: String,
}


// Contact4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
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
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<OtherContact1>>,
	#[serde(rename = "PrefrdMtd")]
	pub prefrd_mtd: Option<String>,
}


// CountryCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CountryCode {
	#[validate(pattern = "[A-Z]{2,2}")]
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// CreditDebitCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CreditDebitCode {
	#[validate(enumerate = ["CRDT", "DBIT"])]
	#[serde(rename = "CreditDebitCode")]
	pub credit_debit_code: String,
}


// CreditTransferMandateData1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CreditTransferMandateData1 {
	#[serde(rename = "MndtId")]
	pub mndt_id: Option<String>,
	#[validate]
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
	#[validate]
	#[serde(rename = "Frqcy")]
	pub frqcy: Option<Frequency36Choice>,
	#[validate]
	#[serde(rename = "Rsn")]
	pub rsn: Option<MandateSetupReason1Choice>,
}


// CreditTransferTransaction45 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CreditTransferTransaction45 {
	#[validate]
	#[serde(rename = "UltmtDbtr")]
	pub ultmt_dbtr: Option<PartyIdentification135>,
	#[validate]
	#[serde(rename = "InitgPty")]
	pub initg_pty: Option<PartyIdentification135>,
	#[validate]
	#[serde(rename = "Dbtr")]
	pub dbtr: PartyIdentification135,
	#[validate]
	#[serde(rename = "DbtrAcct")]
	pub dbtr_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "DbtrAgt")]
	pub dbtr_agt: BranchAndFinancialInstitutionIdentification6,
	#[validate]
	#[serde(rename = "DbtrAgtAcct")]
	pub dbtr_agt_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "PrvsInstgAgt1")]
	pub prvs_instg_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
	#[validate]
	#[serde(rename = "PrvsInstgAgt1Acct")]
	pub prvs_instg_agt1_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "PrvsInstgAgt2")]
	pub prvs_instg_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
	#[validate]
	#[serde(rename = "PrvsInstgAgt2Acct")]
	pub prvs_instg_agt2_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "PrvsInstgAgt3")]
	pub prvs_instg_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
	#[validate]
	#[serde(rename = "PrvsInstgAgt3Acct")]
	pub prvs_instg_agt3_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "IntrmyAgt1")]
	pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
	#[validate]
	#[serde(rename = "IntrmyAgt1Acct")]
	pub intrmy_agt1_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "IntrmyAgt2")]
	pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
	#[validate]
	#[serde(rename = "IntrmyAgt2Acct")]
	pub intrmy_agt2_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "IntrmyAgt3")]
	pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
	#[validate]
	#[serde(rename = "IntrmyAgt3Acct")]
	pub intrmy_agt3_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "CdtrAgt")]
	pub cdtr_agt: BranchAndFinancialInstitutionIdentification6,
	#[validate]
	#[serde(rename = "CdtrAgtAcct")]
	pub cdtr_agt_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "Cdtr")]
	pub cdtr: PartyIdentification135,
	#[validate]
	#[serde(rename = "CdtrAcct")]
	pub cdtr_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "UltmtCdtr")]
	pub ultmt_cdtr: Option<PartyIdentification135>,
	#[validate]
	#[serde(rename = "InstrForCdtrAgt")]
	pub instr_for_cdtr_agt: Option<Vec<InstructionForCreditorAgent3>>,
	#[validate]
	#[serde(rename = "InstrForNxtAgt")]
	pub instr_for_nxt_agt: Option<Vec<InstructionForNextAgent1>>,
	#[validate]
	#[serde(rename = "Tax")]
	pub tax: Option<TaxInformation8>,
	#[validate]
	#[serde(rename = "RmtInf")]
	pub rmt_inf: Option<RemittanceInformation16>,
	#[validate]
	#[serde(rename = "InstdAmt")]
	pub instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// CreditorReferenceInformation2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CreditorReferenceInformation2 {
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: Option<CreditorReferenceType2>,
	#[serde(rename = "Ref")]
	pub ref_attr: Option<String>,
}


// CreditorReferenceType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CreditorReferenceType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// CreditorReferenceType2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CreditorReferenceType2 {
	#[validate]
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: CreditorReferenceType1Choice,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// DateAndDateTime2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DateAndDateTime2Choice {
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm")]
	pub dt_tm: Option<String>,
}


// DateAndPlaceOfBirth1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
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
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DatePeriod2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// DecimalNumber ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "DecimalNumber")]
	pub decimal_number: f64,
}


// DiscountAmountAndType1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DiscountAmountAndType1 {
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: Option<DiscountAmountType1Choice>,
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}


// DiscountAmountType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DiscountAmountType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// DocumentAdjustment1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DocumentAdjustment1 {
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: Option<String>,
	#[serde(rename = "Rsn")]
	pub rsn: Option<String>,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<String>,
}


// DocumentLineIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DocumentLineIdentification1 {
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: Option<DocumentLineType1>,
	#[serde(rename = "Nb")]
	pub nb: Option<String>,
	#[serde(rename = "RltdDt")]
	pub rltd_dt: Option<String>,
}


// DocumentLineInformation1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DocumentLineInformation1 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: Vec<DocumentLineIdentification1>,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: Option<RemittanceAmount3>,
}


// DocumentLineType1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DocumentLineType1 {
	#[validate]
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: DocumentLineType1Choice,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// DocumentLineType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DocumentLineType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// DocumentType3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DocumentType3Code {
	#[validate(enumerate = ["RADM", "RPIN", "FXDR", "DISP", "PUOR", "SCOR"])]
	#[serde(rename = "DocumentType3Code")]
	pub document_type3_code: String,
}


// DocumentType6Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DocumentType6Code {
	#[validate(enumerate = ["MSIN", "CNFA", "DNFA", "CINV", "CREN", "DEBN", "HIRI", "SBIN", "CMCN", "SOAC", "DISP", "BOLD", "VCHR", "AROI", "TSUT", "PUOR"])]
	#[serde(rename = "DocumentType6Code")]
	pub document_type6_code: String,
}


// EquivalentAmount2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EquivalentAmount2 {
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CcyOfTrf")]
	pub ccy_of_trf: String,
}


// Exact2NumericText ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Exact2NumericText {
	#[validate(pattern = "[0-9]{2}")]
	#[serde(rename = "Exact2NumericText")]
	pub exact2_numeric_text: String,
}


// Exact4AlphaNumericText ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[validate(pattern = "[a-zA-Z0-9]{4}")]
	#[serde(rename = "Exact4AlphaNumericText")]
	pub exact4_alpha_numeric_text: String,
}


// ExternalAccountIdentification1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalAccountIdentification1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalAccountIdentification1Code")]
	pub external_account_identification1_code: String,
}


// ExternalCashAccountType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalCashAccountType1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalCashAccountType1Code")]
	pub external_cash_account_type1_code: String,
}


// ExternalCashClearingSystem1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalCashClearingSystem1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 3)]
	#[serde(rename = "ExternalCashClearingSystem1Code")]
	pub external_cash_clearing_system1_code: String,
}


// ExternalCategoryPurpose1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalCategoryPurpose1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalCategoryPurpose1Code")]
	pub external_category_purpose1_code: String,
}


// ExternalClearingSystemIdentification1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalClearingSystemIdentification1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 5)]
	#[serde(rename = "ExternalClearingSystemIdentification1Code")]
	pub external_clearing_system_identification1_code: String,
}


// ExternalCreditorAgentInstruction1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalCreditorAgentInstruction1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalCreditorAgentInstruction1Code")]
	pub external_creditor_agent_instruction1_code: String,
}


// ExternalDiscountAmountType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalDiscountAmountType1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalDiscountAmountType1Code")]
	pub external_discount_amount_type1_code: String,
}


// ExternalDocumentLineType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalDocumentLineType1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalDocumentLineType1Code")]
	pub external_document_line_type1_code: String,
}


// ExternalFinancialInstitutionIdentification1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalFinancialInstitutionIdentification1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalFinancialInstitutionIdentification1Code")]
	pub external_financial_institution_identification1_code: String,
}


// ExternalGarnishmentType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalGarnishmentType1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalGarnishmentType1Code")]
	pub external_garnishment_type1_code: String,
}


// ExternalLocalInstrument1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalLocalInstrument1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 35)]
	#[serde(rename = "ExternalLocalInstrument1Code")]
	pub external_local_instrument1_code: String,
}


// ExternalMandateSetupReason1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalMandateSetupReason1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalMandateSetupReason1Code")]
	pub external_mandate_setup_reason1_code: String,
}


// ExternalOrganisationIdentification1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalOrganisationIdentification1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalOrganisationIdentification1Code")]
	pub external_organisation_identification1_code: String,
}


// ExternalPersonIdentification1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalPersonIdentification1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalPersonIdentification1Code")]
	pub external_person_identification1_code: String,
}


// ExternalProxyAccountType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalProxyAccountType1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalProxyAccountType1Code")]
	pub external_proxy_account_type1_code: String,
}


// ExternalPurpose1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalPurpose1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalPurpose1Code")]
	pub external_purpose1_code: String,
}


// ExternalReturnReason1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalReturnReason1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalReturnReason1Code")]
	pub external_return_reason1_code: String,
}


// ExternalServiceLevel1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalServiceLevel1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalServiceLevel1Code")]
	pub external_service_level1_code: String,
}


// ExternalTaxAmountType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalTaxAmountType1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalTaxAmountType1Code")]
	pub external_tax_amount_type1_code: String,
}


// FinancialIdentificationSchemeName1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// FinancialInstitutionIdentification18 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstitutionIdentification18 {
	#[serde(rename = "BICFI")]
	pub bicfi: Option<String>,
	#[validate]
	#[serde(rename = "ClrSysMmbId")]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[validate]
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress24>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<GenericFinancialIdentification1>,
}


// Frequency36Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Frequency36Choice {
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[validate]
	#[serde(rename = "Prd")]
	pub prd: Option<FrequencyPeriod1>,
	#[validate]
	#[serde(rename = "PtInTm")]
	pub pt_in_tm: Option<FrequencyAndMoment1>,
}


// Frequency6Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Frequency6Code {
	#[validate(enumerate = ["YEAR", "MNTH", "QURT", "MIAN", "WEEK", "DAIL", "ADHO", "INDA", "FRTN"])]
	#[serde(rename = "Frequency6Code")]
	pub frequency6_code: String,
}


// FrequencyAndMoment1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FrequencyAndMoment1 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "PtInTm")]
	pub pt_in_tm: String,
}


// FrequencyPeriod1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FrequencyPeriod1 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "CntPerPrd")]
	pub cnt_per_prd: f64,
}


// Garnishment3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Garnishment3 {
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: GarnishmentType1,
	#[validate]
	#[serde(rename = "Grnshee")]
	pub grnshee: Option<PartyIdentification135>,
	#[validate]
	#[serde(rename = "GrnshmtAdmstr")]
	pub grnshmt_admstr: Option<PartyIdentification135>,
	#[serde(rename = "RefNb")]
	pub ref_nb: Option<String>,
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[validate]
	#[serde(rename = "RmtdAmt")]
	pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "FmlyMdclInsrncInd")]
	pub fmly_mdcl_insrnc_ind: Option<bool>,
	#[serde(rename = "MplyeeTermntnInd")]
	pub mplyee_termntn_ind: Option<bool>,
}


// GarnishmentType1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GarnishmentType1 {
	#[validate]
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: GarnishmentType1Choice,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GarnishmentType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GarnishmentType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// GenericAccountIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericAccountIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[validate]
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<AccountSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericFinancialIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericFinancialIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[validate]
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<FinancialIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericIdentification30 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification30 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// GenericOrganisationIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericOrganisationIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[validate]
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericPersonIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericPersonIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[validate]
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GroupHeader90 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GroupHeader90 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
	#[validate]
	#[serde(rename = "Authstn")]
	pub authstn: Option<Vec<Authorisation1Choice>>,
	#[serde(rename = "BtchBookg")]
	pub btch_bookg: Option<bool>,
	#[serde(rename = "NbOfTxs")]
	pub nb_of_txs: String,
	#[serde(rename = "CtrlSum")]
	pub ctrl_sum: Option<f64>,
	#[serde(rename = "GrpRtr")]
	pub grp_rtr: Option<bool>,
	#[validate]
	#[serde(rename = "TtlRtrdIntrBkSttlmAmt")]
	pub ttl_rtrd_intr_bk_sttlm_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "IntrBkSttlmDt")]
	pub intr_bk_sttlm_dt: Option<String>,
	#[validate]
	#[serde(rename = "SttlmInf")]
	pub sttlm_inf: SettlementInstruction7,
	#[validate]
	#[serde(rename = "InstgAgt")]
	pub instg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[validate]
	#[serde(rename = "InstdAgt")]
	pub instd_agt: Option<BranchAndFinancialInstitutionIdentification6>,
}


// IBAN2007Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct IBAN2007Identifier {
	#[validate(pattern = "[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}")]
	#[serde(rename = "IBAN2007Identifier")]
	pub iban2007_identifier: String,
}


// ISODate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// Instruction4Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Instruction4Code {
	#[validate(enumerate = ["PHOA", "TELA"])]
	#[serde(rename = "Instruction4Code")]
	pub instruction4_code: String,
}


// InstructionForCreditorAgent3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InstructionForCreditorAgent3 {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "InstrInf")]
	pub instr_inf: Option<String>,
}


// InstructionForNextAgent1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InstructionForNextAgent1 {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "InstrInf")]
	pub instr_inf: Option<String>,
}


// LEIIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[validate(pattern = "[A-Z0-9]{18,18}[0-9]{2,2}")]
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LocalInstrument2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LocalInstrument2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// MandateClassification1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MandateClassification1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// MandateClassification1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MandateClassification1Code {
	#[validate(enumerate = ["FIXE", "USGB", "VARI"])]
	#[serde(rename = "MandateClassification1Code")]
	pub mandate_classification1_code: String,
}


// MandateRelatedData1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MandateRelatedData1Choice {
	#[validate]
	#[serde(rename = "DrctDbtMndt")]
	pub drct_dbt_mndt: Option<MandateRelatedInformation14>,
	#[validate]
	#[serde(rename = "CdtTrfMndt")]
	pub cdt_trf_mndt: Option<CreditTransferMandateData1>,
}


// MandateRelatedInformation14 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MandateRelatedInformation14 {
	#[serde(rename = "MndtId")]
	pub mndt_id: Option<String>,
	#[serde(rename = "DtOfSgntr")]
	pub dt_of_sgntr: Option<String>,
	#[serde(rename = "AmdmntInd")]
	pub amdmnt_ind: Option<bool>,
	#[validate]
	#[serde(rename = "AmdmntInfDtls")]
	pub amdmnt_inf_dtls: Option<AmendmentInformationDetails13>,
	#[serde(rename = "ElctrncSgntr")]
	pub elctrnc_sgntr: Option<String>,
	#[serde(rename = "FrstColltnDt")]
	pub frst_colltn_dt: Option<String>,
	#[serde(rename = "FnlColltnDt")]
	pub fnl_colltn_dt: Option<String>,
	#[validate]
	#[serde(rename = "Frqcy")]
	pub frqcy: Option<Frequency36Choice>,
	#[validate]
	#[serde(rename = "Rsn")]
	pub rsn: Option<MandateSetupReason1Choice>,
	#[serde(rename = "TrckgDays")]
	pub trckg_days: Option<String>,
}


// MandateSetupReason1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MandateSetupReason1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// MandateTypeInformation2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MandateTypeInformation2 {
	#[validate]
	#[serde(rename = "SvcLvl")]
	pub svc_lvl: Option<ServiceLevel8Choice>,
	#[validate]
	#[serde(rename = "LclInstrm")]
	pub lcl_instrm: Option<LocalInstrument2Choice>,
	#[validate]
	#[serde(rename = "CtgyPurp")]
	pub ctgy_purp: Option<CategoryPurpose1Choice>,
	#[validate]
	#[serde(rename = "Clssfctn")]
	pub clssfctn: Option<MandateClassification1Choice>,
}


// Max1025Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max1025Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 1025)]
	#[serde(rename = "Max1025Text")]
	pub max1025_text: String,
}


// Max105Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max105Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 105)]
	#[serde(rename = "Max105Text")]
	pub max105_text: String,
}


// Max10KBinary ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max10KBinary {
	#[validate(min_length = 1)]
	#[validate(max_length = 10240)]
	#[serde(rename = "Max10KBinary")]
	pub max10_k_binary: String,
}


// Max128Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max128Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 128)]
	#[serde(rename = "Max128Text")]
	pub max128_text: String,
}


// Max140Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max140Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 140)]
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max15NumericText ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max15NumericText {
	#[validate(pattern = "[0-9]{1,15}")]
	#[serde(rename = "Max15NumericText")]
	pub max15_numeric_text: String,
}


// Max16Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max16Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 16)]
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
}


// Max2048Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max2048Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 2048)]
	#[serde(rename = "Max2048Text")]
	pub max2048_text: String,
}


// Max34Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max34Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 34)]
	#[serde(rename = "Max34Text")]
	pub max34_text: String,
}


// Max350Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max350Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 350)]
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max35Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 35)]
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Max4Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max4Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "Max4Text")]
	pub max4_text: String,
}


// Max70Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max70Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 70)]
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// NamePrefix2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NamePrefix2Code {
	#[validate(enumerate = ["DOCT", "MADM", "MISS", "MIST", "MIKS"])]
	#[serde(rename = "NamePrefix2Code")]
	pub name_prefix2_code: String,
}


// Number ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
}


// OrganisationIdentification29 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OrganisationIdentification29 {
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<GenericOrganisationIdentification1>>,
}


// OrganisationIdentificationSchemeName1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OrganisationIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// OriginalGroupHeader18 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OriginalGroupHeader18 {
	#[serde(rename = "OrgnlMsgId")]
	pub orgnl_msg_id: String,
	#[serde(rename = "OrgnlMsgNmId")]
	pub orgnl_msg_nm_id: String,
	#[serde(rename = "OrgnlCreDtTm")]
	pub orgnl_cre_dt_tm: Option<String>,
	#[validate]
	#[serde(rename = "RtrRsnInf")]
	pub rtr_rsn_inf: Option<Vec<PaymentReturnReason6>>,
}


// OriginalGroupInformation29 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OriginalGroupInformation29 {
	#[serde(rename = "OrgnlMsgId")]
	pub orgnl_msg_id: String,
	#[serde(rename = "OrgnlMsgNmId")]
	pub orgnl_msg_nm_id: String,
	#[serde(rename = "OrgnlCreDtTm")]
	pub orgnl_cre_dt_tm: Option<String>,
}


// OriginalTransactionReference32 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OriginalTransactionReference32 {
	#[validate]
	#[serde(rename = "IntrBkSttlmAmt")]
	pub intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: Option<AmountType4Choice>,
	#[serde(rename = "IntrBkSttlmDt")]
	pub intr_bk_sttlm_dt: Option<String>,
	#[serde(rename = "ReqdColltnDt")]
	pub reqd_colltn_dt: Option<String>,
	#[validate]
	#[serde(rename = "ReqdExctnDt")]
	pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[validate]
	#[serde(rename = "CdtrSchmeId")]
	pub cdtr_schme_id: Option<PartyIdentification135>,
	#[validate]
	#[serde(rename = "SttlmInf")]
	pub sttlm_inf: Option<SettlementInstruction7>,
	#[validate]
	#[serde(rename = "PmtTpInf")]
	pub pmt_tp_inf: Option<PaymentTypeInformation27>,
	#[serde(rename = "PmtMtd")]
	pub pmt_mtd: Option<String>,
	#[validate]
	#[serde(rename = "MndtRltdInf")]
	pub mndt_rltd_inf: Option<MandateRelatedData1Choice>,
	#[validate]
	#[serde(rename = "RmtInf")]
	pub rmt_inf: Option<RemittanceInformation16>,
	#[validate]
	#[serde(rename = "UltmtDbtr")]
	pub ultmt_dbtr: Option<Party40Choice>,
	#[validate]
	#[serde(rename = "Dbtr")]
	pub dbtr: Option<Party40Choice>,
	#[validate]
	#[serde(rename = "DbtrAcct")]
	pub dbtr_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "DbtrAgt")]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[validate]
	#[serde(rename = "DbtrAgtAcct")]
	pub dbtr_agt_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "CdtrAgt")]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[validate]
	#[serde(rename = "CdtrAgtAcct")]
	pub cdtr_agt_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "Cdtr")]
	pub cdtr: Option<Party40Choice>,
	#[validate]
	#[serde(rename = "CdtrAcct")]
	pub cdtr_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "UltmtCdtr")]
	pub ultmt_cdtr: Option<Party40Choice>,
	#[validate]
	#[serde(rename = "Purp")]
	pub purp: Option<Purpose2Choice>,
	#[validate]
	#[serde(rename = "UndrlygCstmrCdtTrf")]
	pub undrlyg_cstmr_cdt_trf: Option<CreditTransferTransaction45>,
}


// OtherContact1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OtherContact1 {
	#[serde(rename = "ChanlTp")]
	pub chanl_tp: String,
	#[serde(rename = "Id")]
	pub id: Option<String>,
}


// Party38Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Party38Choice {
	#[validate]
	#[serde(rename = "OrgId")]
	pub org_id: Option<OrganisationIdentification29>,
	#[validate]
	#[serde(rename = "PrvtId")]
	pub prvt_id: Option<PersonIdentification13>,
}


// Party40Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Party40Choice {
	#[validate]
	#[serde(rename = "Pty")]
	pub pty: Option<PartyIdentification135>,
	#[validate]
	#[serde(rename = "Agt")]
	pub agt: Option<BranchAndFinancialInstitutionIdentification6>,
}


// PartyIdentification135 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification135 {
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[validate]
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress24>,
	#[validate]
	#[serde(rename = "Id")]
	pub id: Option<Party38Choice>,
	#[serde(rename = "CtryOfRes")]
	pub ctry_of_res: Option<String>,
	#[validate]
	#[serde(rename = "CtctDtls")]
	pub ctct_dtls: Option<Contact4>,
}


// PaymentMethod4Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PaymentMethod4Code {
	#[validate(enumerate = ["CHK", "TRF", "DD", "TRA"])]
	#[serde(rename = "PaymentMethod4Code")]
	pub payment_method4_code: String,
}


// PaymentReturnReason6 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PaymentReturnReason6 {
	#[validate]
	#[serde(rename = "Orgtr")]
	pub orgtr: Option<PartyIdentification135>,
	#[validate]
	#[serde(rename = "Rsn")]
	pub rsn: Option<ReturnReason5Choice>,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<Vec<String>>,
}


// PaymentReturnV10 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PaymentReturnV10 {
	#[validate]
	#[serde(rename = "GrpHdr")]
	pub grp_hdr: GroupHeader90,
	#[validate]
	#[serde(rename = "OrgnlGrpInf")]
	pub orgnl_grp_inf: Option<OriginalGroupHeader18>,
	#[validate]
	#[serde(rename = "TxInf")]
	pub tx_inf: Option<Vec<PaymentTransaction118>>,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// PaymentTransaction118 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PaymentTransaction118 {
	#[serde(rename = "RtrId")]
	pub rtr_id: Option<String>,
	#[validate]
	#[serde(rename = "OrgnlGrpInf")]
	pub orgnl_grp_inf: Option<OriginalGroupInformation29>,
	#[serde(rename = "OrgnlInstrId")]
	pub orgnl_instr_id: Option<String>,
	#[serde(rename = "OrgnlEndToEndId")]
	pub orgnl_end_to_end_id: Option<String>,
	#[serde(rename = "OrgnlTxId")]
	pub orgnl_tx_id: Option<String>,
	#[serde(rename = "OrgnlUETR")]
	pub orgnl_uetr: Option<String>,
	#[serde(rename = "OrgnlClrSysRef")]
	pub orgnl_clr_sys_ref: Option<String>,
	#[validate]
	#[serde(rename = "OrgnlIntrBkSttlmAmt")]
	pub orgnl_intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "OrgnlIntrBkSttlmDt")]
	pub orgnl_intr_bk_sttlm_dt: Option<String>,
	#[validate]
	#[serde(rename = "RtrdIntrBkSttlmAmt")]
	pub rtrd_intr_bk_sttlm_amt: ActiveCurrencyAndAmount,
	#[serde(rename = "IntrBkSttlmDt")]
	pub intr_bk_sttlm_dt: Option<String>,
	#[serde(rename = "SttlmPrty")]
	pub sttlm_prty: Option<String>,
	#[validate]
	#[serde(rename = "SttlmTmIndctn")]
	pub sttlm_tm_indctn: Option<SettlementDateTimeIndication1>,
	#[validate]
	#[serde(rename = "RtrdInstdAmt")]
	pub rtrd_instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "XchgRate")]
	pub xchg_rate: Option<f64>,
	#[validate]
	#[serde(rename = "CompstnAmt")]
	pub compstn_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "ChrgBr")]
	pub chrg_br: Option<String>,
	#[validate]
	#[serde(rename = "ChrgsInf")]
	pub chrgs_inf: Option<Vec<Charges7>>,
	#[serde(rename = "ClrSysRef")]
	pub clr_sys_ref: Option<String>,
	#[validate]
	#[serde(rename = "InstgAgt")]
	pub instg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[validate]
	#[serde(rename = "InstdAgt")]
	pub instd_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[validate]
	#[serde(rename = "RtrChain")]
	pub rtr_chain: Option<TransactionParties8>,
	#[validate]
	#[serde(rename = "RtrRsnInf")]
	pub rtr_rsn_inf: Option<Vec<PaymentReturnReason6>>,
	#[validate]
	#[serde(rename = "OrgnlTxRef")]
	pub orgnl_tx_ref: Option<OriginalTransactionReference32>,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// PaymentTypeInformation27 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PaymentTypeInformation27 {
	#[serde(rename = "InstrPrty")]
	pub instr_prty: Option<String>,
	#[serde(rename = "ClrChanl")]
	pub clr_chanl: Option<String>,
	#[validate]
	#[serde(rename = "SvcLvl")]
	pub svc_lvl: Option<Vec<ServiceLevel8Choice>>,
	#[validate]
	#[serde(rename = "LclInstrm")]
	pub lcl_instrm: Option<LocalInstrument2Choice>,
	#[serde(rename = "SeqTp")]
	pub seq_tp: Option<String>,
	#[validate]
	#[serde(rename = "CtgyPurp")]
	pub ctgy_purp: Option<CategoryPurpose1Choice>,
}


// PercentageRate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// PersonIdentification13 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PersonIdentification13 {
	#[validate]
	#[serde(rename = "DtAndPlcOfBirth")]
	pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<GenericPersonIdentification1>>,
}


// PersonIdentificationSchemeName1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PersonIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// PhoneNumber ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PhoneNumber {
	#[validate(pattern = "\\+[0-9]{1,3}-[0-9()+\\-]{1,30}")]
	#[serde(rename = "PhoneNumber")]
	pub phone_number: String,
}


// PostalAddress24 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PostalAddress24 {
	#[validate]
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
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PreferredContactMethod1Code {
	#[validate(enumerate = ["LETT", "MAIL", "PHON", "FAXX", "CELL"])]
	#[serde(rename = "PreferredContactMethod1Code")]
	pub preferred_contact_method1_code: String,
}


// Priority2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Priority2Code {
	#[validate(enumerate = ["HIGH", "NORM"])]
	#[serde(rename = "Priority2Code")]
	pub priority2_code: String,
}


// Priority3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Priority3Code {
	#[validate(enumerate = ["URGT", "HIGH", "NORM"])]
	#[serde(rename = "Priority3Code")]
	pub priority3_code: String,
}


// ProxyAccountIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ProxyAccountIdentification1 {
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: Option<ProxyAccountType1Choice>,
	#[serde(rename = "Id")]
	pub id: String,
}


// ProxyAccountType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ProxyAccountType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// Purpose2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Purpose2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ReferredDocumentInformation7 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReferredDocumentInformation7 {
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: Option<ReferredDocumentType4>,
	#[serde(rename = "Nb")]
	pub nb: Option<String>,
	#[serde(rename = "RltdDt")]
	pub rltd_dt: Option<String>,
	#[validate]
	#[serde(rename = "LineDtls")]
	pub line_dtls: Option<Vec<DocumentLineInformation1>>,
}


// ReferredDocumentType3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReferredDocumentType3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ReferredDocumentType4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReferredDocumentType4 {
	#[validate]
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: ReferredDocumentType3Choice,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// RemittanceAmount2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RemittanceAmount2 {
	#[validate]
	#[serde(rename = "DuePyblAmt")]
	pub due_pybl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "DscntApldAmt")]
	pub dscnt_apld_amt: Option<Vec<DiscountAmountAndType1>>,
	#[validate]
	#[serde(rename = "CdtNoteAmt")]
	pub cdt_note_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "TaxAmt")]
	pub tax_amt: Option<Vec<TaxAmountAndType1>>,
	#[validate]
	#[serde(rename = "AdjstmntAmtAndRsn")]
	pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment1>>,
	#[validate]
	#[serde(rename = "RmtdAmt")]
	pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// RemittanceAmount3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RemittanceAmount3 {
	#[validate]
	#[serde(rename = "DuePyblAmt")]
	pub due_pybl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "DscntApldAmt")]
	pub dscnt_apld_amt: Option<Vec<DiscountAmountAndType1>>,
	#[validate]
	#[serde(rename = "CdtNoteAmt")]
	pub cdt_note_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "TaxAmt")]
	pub tax_amt: Option<Vec<TaxAmountAndType1>>,
	#[validate]
	#[serde(rename = "AdjstmntAmtAndRsn")]
	pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment1>>,
	#[validate]
	#[serde(rename = "RmtdAmt")]
	pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// RemittanceInformation16 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RemittanceInformation16 {
	#[serde(rename = "Ustrd")]
	pub ustrd: Option<Vec<String>>,
	#[validate]
	#[serde(rename = "Strd")]
	pub strd: Option<Vec<StructuredRemittanceInformation16>>,
}


// ReturnReason5Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReturnReason5Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// SequenceType3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SequenceType3Code {
	#[validate(enumerate = ["FRST", "RCUR", "FNAL", "OOFF", "RPRE"])]
	#[serde(rename = "SequenceType3Code")]
	pub sequence_type3_code: String,
}


// ServiceLevel8Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ServiceLevel8Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// SettlementDateTimeIndication1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementDateTimeIndication1 {
	#[serde(rename = "DbtDtTm")]
	pub dbt_dt_tm: Option<String>,
	#[serde(rename = "CdtDtTm")]
	pub cdt_dt_tm: Option<String>,
}


// SettlementInstruction7 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementInstruction7 {
	#[serde(rename = "SttlmMtd")]
	pub sttlm_mtd: String,
	#[validate]
	#[serde(rename = "SttlmAcct")]
	pub sttlm_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "ClrSys")]
	pub clr_sys: Option<ClearingSystemIdentification3Choice>,
	#[validate]
	#[serde(rename = "InstgRmbrsmntAgt")]
	pub instg_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[validate]
	#[serde(rename = "InstgRmbrsmntAgtAcct")]
	pub instg_rmbrsmnt_agt_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "InstdRmbrsmntAgt")]
	pub instd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[validate]
	#[serde(rename = "InstdRmbrsmntAgtAcct")]
	pub instd_rmbrsmnt_agt_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "ThrdRmbrsmntAgt")]
	pub thrd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[validate]
	#[serde(rename = "ThrdRmbrsmntAgtAcct")]
	pub thrd_rmbrsmnt_agt_acct: Option<CashAccount38>,
}


// SettlementMethod1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementMethod1Code {
	#[validate(enumerate = ["INDA", "INGA", "COVE", "CLRG"])]
	#[serde(rename = "SettlementMethod1Code")]
	pub settlement_method1_code: String,
}


// StructuredRemittanceInformation16 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct StructuredRemittanceInformation16 {
	#[validate]
	#[serde(rename = "RfrdDocInf")]
	pub rfrd_doc_inf: Option<Vec<ReferredDocumentInformation7>>,
	#[validate]
	#[serde(rename = "RfrdDocAmt")]
	pub rfrd_doc_amt: Option<RemittanceAmount2>,
	#[validate]
	#[serde(rename = "CdtrRefInf")]
	pub cdtr_ref_inf: Option<CreditorReferenceInformation2>,
	#[validate]
	#[serde(rename = "Invcr")]
	pub invcr: Option<PartyIdentification135>,
	#[validate]
	#[serde(rename = "Invcee")]
	pub invcee: Option<PartyIdentification135>,
	#[validate]
	#[serde(rename = "TaxRmt")]
	pub tax_rmt: Option<TaxInformation7>,
	#[validate]
	#[serde(rename = "GrnshmtRmt")]
	pub grnshmt_rmt: Option<Garnishment3>,
	#[serde(rename = "AddtlRmtInf")]
	pub addtl_rmt_inf: Option<Vec<String>>,
}


// SupplementaryData1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Option<String>,
	#[validate]
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}


// TaxAmount2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TaxAmount2 {
	#[serde(rename = "Rate")]
	pub rate: Option<f64>,
	#[validate]
	#[serde(rename = "TaxblBaseAmt")]
	pub taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "TtlAmt")]
	pub ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "Dtls")]
	pub dtls: Option<Vec<TaxRecordDetails2>>,
}


// TaxAmountAndType1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TaxAmountAndType1 {
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: Option<TaxAmountType1Choice>,
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}


// TaxAmountType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TaxAmountType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// TaxAuthorisation1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TaxAuthorisation1 {
	#[serde(rename = "Titl")]
	pub titl: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
}


// TaxInformation7 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TaxInformation7 {
	#[validate]
	#[serde(rename = "Cdtr")]
	pub cdtr: Option<TaxParty1>,
	#[validate]
	#[serde(rename = "Dbtr")]
	pub dbtr: Option<TaxParty2>,
	#[validate]
	#[serde(rename = "UltmtDbtr")]
	pub ultmt_dbtr: Option<TaxParty2>,
	#[serde(rename = "AdmstnZone")]
	pub admstn_zone: Option<String>,
	#[serde(rename = "RefNb")]
	pub ref_nb: Option<String>,
	#[serde(rename = "Mtd")]
	pub mtd: Option<String>,
	#[validate]
	#[serde(rename = "TtlTaxblBaseAmt")]
	pub ttl_taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "TtlTaxAmt")]
	pub ttl_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "SeqNb")]
	pub seq_nb: Option<f64>,
	#[validate]
	#[serde(rename = "Rcrd")]
	pub rcrd: Option<Vec<TaxRecord2>>,
}


// TaxInformation8 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TaxInformation8 {
	#[validate]
	#[serde(rename = "Cdtr")]
	pub cdtr: Option<TaxParty1>,
	#[validate]
	#[serde(rename = "Dbtr")]
	pub dbtr: Option<TaxParty2>,
	#[serde(rename = "AdmstnZone")]
	pub admstn_zone: Option<String>,
	#[serde(rename = "RefNb")]
	pub ref_nb: Option<String>,
	#[serde(rename = "Mtd")]
	pub mtd: Option<String>,
	#[validate]
	#[serde(rename = "TtlTaxblBaseAmt")]
	pub ttl_taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "TtlTaxAmt")]
	pub ttl_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "SeqNb")]
	pub seq_nb: Option<f64>,
	#[validate]
	#[serde(rename = "Rcrd")]
	pub rcrd: Option<Vec<TaxRecord2>>,
}


// TaxParty1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TaxParty1 {
	#[serde(rename = "TaxId")]
	pub tax_id: Option<String>,
	#[serde(rename = "RegnId")]
	pub regn_id: Option<String>,
	#[serde(rename = "TaxTp")]
	pub tax_tp: Option<String>,
}


// TaxParty2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TaxParty2 {
	#[serde(rename = "TaxId")]
	pub tax_id: Option<String>,
	#[serde(rename = "RegnId")]
	pub regn_id: Option<String>,
	#[serde(rename = "TaxTp")]
	pub tax_tp: Option<String>,
	#[validate]
	#[serde(rename = "Authstn")]
	pub authstn: Option<TaxAuthorisation1>,
}


// TaxPeriod2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TaxPeriod2 {
	#[serde(rename = "Yr")]
	pub yr: Option<String>,
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[validate]
	#[serde(rename = "FrToDt")]
	pub fr_to_dt: Option<DatePeriod2>,
}


// TaxRecord2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TaxRecord2 {
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
	#[validate]
	#[serde(rename = "Prd")]
	pub prd: Option<TaxPeriod2>,
	#[validate]
	#[serde(rename = "TaxAmt")]
	pub tax_amt: Option<TaxAmount2>,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<String>,
}


// TaxRecordDetails2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TaxRecordDetails2 {
	#[validate]
	#[serde(rename = "Prd")]
	pub prd: Option<TaxPeriod2>,
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}


// TaxRecordPeriod1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TaxRecordPeriod1Code {
	#[validate(enumerate = ["MM01", "MM02", "MM03", "MM04", "MM05", "MM06", "MM07", "MM08", "MM09", "MM10", "MM11", "MM12", "QTR1", "QTR2", "QTR3", "QTR4", "HLF1", "HLF2"])]
	#[serde(rename = "TaxRecordPeriod1Code")]
	pub tax_record_period1_code: String,
}


// TransactionParties8 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TransactionParties8 {
	#[validate]
	#[serde(rename = "UltmtDbtr")]
	pub ultmt_dbtr: Option<Party40Choice>,
	#[validate]
	#[serde(rename = "Dbtr")]
	pub dbtr: Party40Choice,
	#[validate]
	#[serde(rename = "DbtrAcct")]
	pub dbtr_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "InitgPty")]
	pub initg_pty: Option<Party40Choice>,
	#[validate]
	#[serde(rename = "DbtrAgt")]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[validate]
	#[serde(rename = "DbtrAgtAcct")]
	pub dbtr_agt_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "PrvsInstgAgt1")]
	pub prvs_instg_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
	#[validate]
	#[serde(rename = "PrvsInstgAgt1Acct")]
	pub prvs_instg_agt1_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "PrvsInstgAgt2")]
	pub prvs_instg_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
	#[validate]
	#[serde(rename = "PrvsInstgAgt2Acct")]
	pub prvs_instg_agt2_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "PrvsInstgAgt3")]
	pub prvs_instg_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
	#[validate]
	#[serde(rename = "PrvsInstgAgt3Acct")]
	pub prvs_instg_agt3_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "IntrmyAgt1")]
	pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
	#[validate]
	#[serde(rename = "IntrmyAgt1Acct")]
	pub intrmy_agt1_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "IntrmyAgt2")]
	pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
	#[validate]
	#[serde(rename = "IntrmyAgt2Acct")]
	pub intrmy_agt2_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "IntrmyAgt3")]
	pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
	#[validate]
	#[serde(rename = "IntrmyAgt3Acct")]
	pub intrmy_agt3_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "CdtrAgt")]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[validate]
	#[serde(rename = "CdtrAgtAcct")]
	pub cdtr_agt_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "Cdtr")]
	pub cdtr: Party40Choice,
	#[validate]
	#[serde(rename = "CdtrAcct")]
	pub cdtr_acct: Option<CashAccount38>,
	#[validate]
	#[serde(rename = "UltmtCdtr")]
	pub ultmt_cdtr: Option<Party40Choice>,
}


// TrueFalseIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}


// UUIDv4Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct UUIDv4Identifier {
	#[validate(pattern = "[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}")]
	#[serde(rename = "UUIDv4Identifier")]
	pub uui_dv4_identifier: String,
}
