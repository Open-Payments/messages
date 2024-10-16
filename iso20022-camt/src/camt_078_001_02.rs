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


// AccountIdentificationSearchCriteria2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountIdentificationSearchCriteria2Choice {
	#[serde(rename = "EQ", skip_serializing_if = "Option::is_none")]
	pub eq: Option<AccountIdentification4Choice>,
	#[serde(rename = "CTTxt", skip_serializing_if = "Option::is_none")]
	pub ct_txt: Option<Max35Text>,
	#[serde(rename = "NCTTxt", skip_serializing_if = "Option::is_none")]
	pub nct_txt: Option<Max35Text>,
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


// AmountAndDirection5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection5 {
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
	#[serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt: Option<CreditDebitCode>,
}


// AmountAndQuantityBreakdown1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndQuantityBreakdown1 {
	#[serde(rename = "LotNb", skip_serializing_if = "Option::is_none")]
	pub lot_nb: Option<GenericIdentification37>,
	#[serde(rename = "LotAmt", skip_serializing_if = "Option::is_none")]
	pub lot_amt: Option<AmountAndDirection5>,
	#[serde(rename = "LotQty", skip_serializing_if = "Option::is_none")]
	pub lot_qty: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "CshSubBalTp", skip_serializing_if = "Option::is_none")]
	pub csh_sub_bal_tp: Option<GenericIdentification30>,
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
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
}


// BICFIDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
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


// CashBalanceType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashBalanceType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalBalanceType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// CashSubBalanceTypeAndQuantityBreakdown3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashSubBalanceTypeAndQuantityBreakdown3 {
	#[serde(rename = "Tp")]
	pub tp: CashBalanceType3Choice,
	#[serde(rename = "QtyBrkdwn", skip_serializing_if = "Option::is_none")]
	pub qty_brkdwn: Option<Vec<AmountAndQuantityBreakdown1>>,
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


// CopyDuplicate1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CopyDuplicate1Code {
	#[default]
	#[serde(rename = "CODU")]
	CodeCODU,
	#[serde(rename = "COPY")]
	CodeCOPY,
	#[serde(rename = "DUPL")]
	CodeDUPL,

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


// DateAndDateTimeSearch5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTimeSearch5Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<DatePeriodSearch1Choice>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<DateTimeSearch2Choice>,
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


// DateTimeSearch2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimeSearch2Choice {
	#[serde(rename = "FrDtTm", skip_serializing_if = "Option::is_none")]
	pub fr_dt_tm: Option<String>,
	#[serde(rename = "ToDtTm", skip_serializing_if = "Option::is_none")]
	pub to_dt_tm: Option<String>,
	#[serde(rename = "FrToDtTm", skip_serializing_if = "Option::is_none")]
	pub fr_to_dt_tm: Option<DateTimePeriod1>,
	#[serde(rename = "EQDtTm", skip_serializing_if = "Option::is_none")]
	pub eq_dt_tm: Option<String>,
	#[serde(rename = "NEQDtTm", skip_serializing_if = "Option::is_none")]
	pub neq_dt_tm: Option<String>,
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "$value")]
	pub decimal_number: f64,
}


// DocumentIdentification51 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentIdentification51 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<DateAndDateTime2Choice>,
	#[serde(rename = "CpyDplct", skip_serializing_if = "Option::is_none")]
	pub cpy_dplct: Option<CopyDuplicate1Code>,
	#[serde(rename = "MsgOrgtr", skip_serializing_if = "Option::is_none")]
	pub msg_orgtr: Option<PartyIdentification136>,
	#[serde(rename = "MsgRcpt", skip_serializing_if = "Option::is_none")]
	pub msg_rcpt: Option<PartyIdentification136>,
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "$value")]
	pub exact4_alpha_numeric_text: String,
}


// Exact4NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact4NumericText {
	#[serde(rename = "$value")]
	pub exact4_numeric_text: String,
}


// ExternalAccountIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAccountIdentification1Code {
	#[serde(rename = "$value")]
	pub external_account_identification1_code: String,
}


// ExternalBalanceType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalBalanceType1Code {
	#[serde(rename = "$value")]
	pub external_balance_type1_code: String,
}


// ExternalClearingSystemIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalClearingSystemIdentification1Code {
	#[serde(rename = "$value")]
	pub external_clearing_system_identification1_code: String,
}


// ExternalFinancialInstitutionIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalFinancialInstitutionIdentification1Code {
	#[serde(rename = "$value")]
	pub external_financial_institution_identification1_code: String,
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


// FinancialInstrumentQuantity1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentQuantity1Choice {
	#[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
	pub unit: Option<f64>,
	#[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
	pub face_amt: Option<f64>,
	#[serde(rename = "AmtsdVal", skip_serializing_if = "Option::is_none")]
	pub amtsd_val: Option<f64>,
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


// GenericIdentification36 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification36 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Issr")]
	pub issr: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
}


// GenericIdentification37 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification37 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
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
	#[serde(rename = "$value")]
	pub implied_currency_and_amount: f64,
}


// IntraBalanceMovementQueryV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IntraBalanceMovementQueryV02 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<DocumentIdentification51>,
	#[serde(rename = "QryDef")]
	pub qry_def: IntraBalanceQueryDefinition11,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// IntraBalanceQueryCriteria11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IntraBalanceQueryCriteria11 {
	#[serde(rename = "Refs", skip_serializing_if = "Option::is_none")]
	pub refs: Option<Vec<References36Choice>>,
	#[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
	pub sts: Option<IntraBalanceQueryStatus3>,
	#[serde(rename = "CshAcct", skip_serializing_if = "Option::is_none")]
	pub csh_acct: Option<Vec<AccountIdentificationSearchCriteria2Choice>>,
	#[serde(rename = "CshAcctOwnr", skip_serializing_if = "Option::is_none")]
	pub csh_acct_ownr: Option<Vec<SystemPartyIdentification8>>,
	#[serde(rename = "CshAcctSvcr", skip_serializing_if = "Option::is_none")]
	pub csh_acct_svcr: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "BalTp", skip_serializing_if = "Option::is_none")]
	pub bal_tp: Option<Vec<IntraBalanceType3>>,
	#[serde(rename = "CshSubBalId", skip_serializing_if = "Option::is_none")]
	pub csh_sub_bal_id: Option<Vec<GenericIdentification37>>,
	#[serde(rename = "SttlmAmt", skip_serializing_if = "Option::is_none")]
	pub sttlm_amt: Option<ImpliedCurrencyAmountRange1Choice>,
	#[serde(rename = "SttldAmt", skip_serializing_if = "Option::is_none")]
	pub sttld_amt: Option<ImpliedCurrencyAmountRange1Choice>,
	#[serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none")]
	pub sttlm_ccy: Option<Vec<ActiveOrHistoricCurrencyCode>>,
	#[serde(rename = "IntnddSttlmDt", skip_serializing_if = "Option::is_none")]
	pub intndd_sttlm_dt: Option<DateAndDateTimeSearch5Choice>,
	#[serde(rename = "FctvSttlmDt", skip_serializing_if = "Option::is_none")]
	pub fctv_sttlm_dt: Option<DateAndDateTimeSearch5Choice>,
	#[serde(rename = "Prty", skip_serializing_if = "Option::is_none")]
	pub prty: Option<Vec<PriorityNumeric4Choice>>,
	#[serde(rename = "MsgOrgtr", skip_serializing_if = "Option::is_none")]
	pub msg_orgtr: Option<Vec<SystemPartyIdentification8>>,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<DateAndDateTimeSearch5Choice>,
}


// IntraBalanceQueryDefinition11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IntraBalanceQueryDefinition11 {
	#[serde(rename = "QryTp")]
	pub qry_tp: MovementResponseType1Code,
	#[serde(rename = "SchCrit")]
	pub sch_crit: IntraBalanceQueryCriteria11,
}


// IntraBalanceQueryStatus3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IntraBalanceQueryStatus3 {
	#[serde(rename = "Tp")]
	pub tp: IntraBalanceStatusType2,
	#[serde(rename = "DtPrd", skip_serializing_if = "Option::is_none")]
	pub dt_prd: Option<DateAndDateTimeSearch5Choice>,
}


// IntraBalanceStatusType2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IntraBalanceStatusType2 {
	#[serde(rename = "PrcgSts", skip_serializing_if = "Option::is_none")]
	pub prcg_sts: Option<Vec<ProcessingStatus68Choice>>,
	#[serde(rename = "SttlmSts", skip_serializing_if = "Option::is_none")]
	pub sttlm_sts: Option<Vec<SettlementStatus26Choice>>,
	#[serde(rename = "Sttld", skip_serializing_if = "Option::is_none")]
	pub sttld: Option<ProprietaryReason4>,
}


// IntraBalanceType3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IntraBalanceType3 {
	#[serde(rename = "BalFr", skip_serializing_if = "Option::is_none")]
	pub bal_fr: Option<CashSubBalanceTypeAndQuantityBreakdown3>,
	#[serde(rename = "BalTo", skip_serializing_if = "Option::is_none")]
	pub bal_to: Option<CashSubBalanceTypeAndQuantityBreakdown3>,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
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


// Max210Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max210Text {
	#[serde(rename = "$value")]
	pub max210_text: String,
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


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
}


// MovementResponseType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum MovementResponseType1Code {
	#[default]
	#[serde(rename = "FULL")]
	CodeFULL,
	#[serde(rename = "STTS")]
	CodeSTTS,

}


// NameAndAddress5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<PostalAddress1>,
}


// PartyIdentification120Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification120Choice {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification36>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
}


// PartyIdentification136 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification136 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification120Choice,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
}


// PostalAddress1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress1 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<AddressType2Code>,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
	pub adr_line: Option<Vec<Max70Text>>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<Max70Text>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<Max16Text>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<Max16Text>,
	#[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
	pub twn_nm: Option<Max35Text>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<Max35Text>,
	#[serde(rename = "Ctry")]
	pub ctry: CountryCode,
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


// PriorityNumeric4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriorityNumeric4Choice {
	#[serde(rename = "Nmrc", skip_serializing_if = "Option::is_none")]
	pub nmrc: Option<Exact4NumericText>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// ProcessingStatus68Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProcessingStatus68Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<TransactionProcessingStatus3Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// ProprietaryReason4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryReason4 {
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<GenericIdentification30>,
	#[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
	pub addtl_rsn_inf: Option<Max210Text>,
}


// References36Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct References36Choice {
	#[serde(rename = "AcctOwnrTxId", skip_serializing_if = "Option::is_none")]
	pub acct_ownr_tx_id: Option<Max35Text>,
	#[serde(rename = "AcctSvcrTxId", skip_serializing_if = "Option::is_none")]
	pub acct_svcr_tx_id: Option<Max35Text>,
	#[serde(rename = "MktInfrstrctrTxId", skip_serializing_if = "Option::is_none")]
	pub mkt_infrstrctr_tx_id: Option<Max35Text>,
	#[serde(rename = "PrcrTxId", skip_serializing_if = "Option::is_none")]
	pub prcr_tx_id: Option<Max35Text>,
	#[serde(rename = "PoolId", skip_serializing_if = "Option::is_none")]
	pub pool_id: Option<Max35Text>,
	#[serde(rename = "CorpActnEvtId", skip_serializing_if = "Option::is_none")]
	pub corp_actn_evt_id: Option<Max35Text>,
}


// SecuritiesSettlementStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SecuritiesSettlementStatus1Code {
	#[default]
	#[serde(rename = "PEND")]
	CodePEND,
	#[serde(rename = "PENF")]
	CodePENF,

}


// SettlementStatus26Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementStatus26Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<SecuritiesSettlementStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
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


// SystemPartyIdentification8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemPartyIdentification8 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification136,
	#[serde(rename = "RspnsblPtyId", skip_serializing_if = "Option::is_none")]
	pub rspnsbl_pty_id: Option<PartyIdentification136>,
}


// TransactionProcessingStatus3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TransactionProcessingStatus3Code {
	#[default]
	#[serde(rename = "CAND")]
	CodeCAND,
	#[serde(rename = "PACK")]
	CodePACK,
	#[serde(rename = "REJT")]
	CodeREJT,
	#[serde(rename = "REPR")]
	CodeREPR,

}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "$value")]
	pub yes_no_indicator: bool,
}
