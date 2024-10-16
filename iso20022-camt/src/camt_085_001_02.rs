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


// AccountSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalAccountIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// AcknowledgedAcceptedStatus21Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AcknowledgedAcceptedStatus21Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<AcknowledgementReason9>>,
}


// AcknowledgementReason12Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AcknowledgementReason12Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<AcknowledgementReason5Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// AcknowledgementReason5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AcknowledgementReason5Code {
	#[default]
	#[serde(rename = "ADEA")]
	CodeADEA,
	#[serde(rename = "SMPG")]
	CodeSMPG,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "CDCY")]
	CodeCDCY,
	#[serde(rename = "CDRG")]
	CodeCDRG,
	#[serde(rename = "CDRE")]
	CodeCDRE,
	#[serde(rename = "NSTP")]
	CodeNSTP,
	#[serde(rename = "RQWV")]
	CodeRQWV,
	#[serde(rename = "LATE")]
	CodeLATE,
}


// AcknowledgementReason9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AcknowledgementReason9 {
	#[serde(rename = "Cd")]
	pub cd: AcknowledgementReason12Choice,
	#[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
	pub addtl_rsn_inf: Option<Max210Text>,
}


// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
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
#[serde(transparent)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "$value")]
	pub active_currency_code: String,
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
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


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
}


// BICFIDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
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


// CancellationReason19Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CancellationReason19Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<CancelledStatusReason13Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// CancellationReason9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CancellationReason9 {
	#[serde(rename = "Cd")]
	pub cd: CancellationReason19Choice,
	#[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
	pub addtl_rsn_inf: Option<Max210Text>,
}


// CancellationStatus14Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CancellationStatus14Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<CancellationReason9>>,
}


// CancelledStatusReason13Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CancelledStatusReason13Code {
	#[default]
	#[serde(rename = "CANI")]
	CodeCANI,
	#[serde(rename = "CANS")]
	CodeCANS,
	#[serde(rename = "CSUB")]
	CodeCSUB,
	#[serde(rename = "CXLR")]
	CodeCXLR,
	#[serde(rename = "CANT")]
	CodeCANT,
	#[serde(rename = "CANZ")]
	CodeCANZ,
	#[serde(rename = "CORP")]
	CodeCORP,
	#[serde(rename = "SCEX")]
	CodeSCEX,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "CTHP")]
	CodeCTHP,
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


// CashAccountType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalCashAccountType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
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
#[serde(transparent)]
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


// DateTimePeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod1 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: String,
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
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


// DocumentNumber5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentNumber5Choice {
	#[serde(rename = "ShrtNb", skip_serializing_if = "Option::is_none")]
	pub shrt_nb: Option<Exact3NumericText>,
	#[serde(rename = "LngNb", skip_serializing_if = "Option::is_none")]
	pub lng_nb: Option<ISO20022MessageIdentificationText>,
	#[serde(rename = "PrtryNb", skip_serializing_if = "Option::is_none")]
	pub prtry_nb: Option<GenericIdentification36>,
}


// EventFrequency7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EventFrequency7Code {
	#[default]
	#[serde(rename = "YEAR")]
	CodeYEAR,
	#[serde(rename = "ADHO")]
	CodeADHO,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "INDA")]
	CodeINDA,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "SEMI")]
	CodeSEMI,
	#[serde(rename = "QUTR")]
	CodeQUTR,
	#[serde(rename = "TOMN")]
	CodeTOMN,
	#[serde(rename = "TOWK")]
	CodeTOWK,
	#[serde(rename = "TWMN")]
	CodeTWMN,
	#[serde(rename = "OVNG")]
	CodeOVNG,
	#[serde(rename = "ONDE")]
	CodeONDE,
}


// Exact3NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Exact3NumericText {
	#[serde(rename = "$value")]
	pub exact3_numeric_text: String,
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "$value")]
	pub exact4_alpha_numeric_text: String,
}


// Exact4NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Exact4NumericText {
	#[serde(rename = "$value")]
	pub exact4_numeric_text: String,
}


// Exact5NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Exact5NumericText {
	#[serde(rename = "$value")]
	pub exact5_numeric_text: String,
}


// ExternalAccountIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalAccountIdentification1Code {
	#[serde(rename = "$value")]
	pub external_account_identification1_code: String,
}


// ExternalBalanceType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalBalanceType1Code {
	#[serde(rename = "$value")]
	pub external_balance_type1_code: String,
}


// ExternalCashAccountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalCashAccountType1Code {
	#[serde(rename = "$value")]
	pub external_cash_account_type1_code: String,
}


// ExternalClearingSystemIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalClearingSystemIdentification1Code {
	#[serde(rename = "$value")]
	pub external_clearing_system_identification1_code: String,
}


// ExternalFinancialInstitutionIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalFinancialInstitutionIdentification1Code {
	#[serde(rename = "$value")]
	pub external_financial_institution_identification1_code: String,
}


// ExternalProxyAccountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalProxyAccountType1Code {
	#[serde(rename = "$value")]
	pub external_proxy_account_type1_code: String,
}


// FailingReason3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FailingReason3Code {
	#[default]
	#[serde(rename = "AWMO")]
	CodeAWMO,
	#[serde(rename = "BYIY")]
	CodeBYIY,
	#[serde(rename = "CLAT")]
	CodeCLAT,
	#[serde(rename = "ADEA")]
	CodeADEA,
	#[serde(rename = "CANR")]
	CodeCANR,
	#[serde(rename = "CAIS")]
	CodeCAIS,
	#[serde(rename = "OBJT")]
	CodeOBJT,
	#[serde(rename = "AWSH")]
	CodeAWSH,
	#[serde(rename = "PHSE")]
	CodePHSE,
	#[serde(rename = "STCD")]
	CodeSTCD,
	#[serde(rename = "DOCY")]
	CodeDOCY,
	#[serde(rename = "MLAT")]
	CodeMLAT,
	#[serde(rename = "DOCC")]
	CodeDOCC,
	#[serde(rename = "BLOC")]
	CodeBLOC,
	#[serde(rename = "CHAS")]
	CodeCHAS,
	#[serde(rename = "NEWI")]
	CodeNEWI,
	#[serde(rename = "CLAC")]
	CodeCLAC,
	#[serde(rename = "MUNO")]
	CodeMUNO,
	#[serde(rename = "GLOB")]
	CodeGLOB,
	#[serde(rename = "PREA")]
	CodePREA,
	#[serde(rename = "PART")]
	CodePART,
	#[serde(rename = "NOFX")]
	CodeNOFX,
	#[serde(rename = "CMON")]
	CodeCMON,
	#[serde(rename = "YCOL")]
	CodeYCOL,
	#[serde(rename = "COLL")]
	CodeCOLL,
	#[serde(rename = "DEPO")]
	CodeDEPO,
	#[serde(rename = "FLIM")]
	CodeFLIM,
	#[serde(rename = "INCA")]
	CodeINCA,
	#[serde(rename = "LINK")]
	CodeLINK,
	#[serde(rename = "LACK")]
	CodeLACK,
	#[serde(rename = "LALO")]
	CodeLALO,
	#[serde(rename = "MONY")]
	CodeMONY,
	#[serde(rename = "NCON")]
	CodeNCON,
	#[serde(rename = "REFS")]
	CodeREFS,
	#[serde(rename = "SDUT")]
	CodeSDUT,
	#[serde(rename = "BATC")]
	CodeBATC,
	#[serde(rename = "CYCL")]
	CodeCYCL,
	#[serde(rename = "SBLO")]
	CodeSBLO,
	#[serde(rename = "CPEC")]
	CodeCPEC,
	#[serde(rename = "MINO")]
	CodeMINO,
	#[serde(rename = "IAAD")]
	CodeIAAD,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "PHCK")]
	CodePHCK,
	#[serde(rename = "BENO")]
	CodeBENO,
	#[serde(rename = "BOTH")]
	CodeBOTH,
	#[serde(rename = "CLHT")]
	CodeCLHT,
	#[serde(rename = "DENO")]
	CodeDENO,
	#[serde(rename = "DISA")]
	CodeDISA,
	#[serde(rename = "DKNY")]
	CodeDKNY,
	#[serde(rename = "FROZ")]
	CodeFROZ,
	#[serde(rename = "LAAW")]
	CodeLAAW,
	#[serde(rename = "LATE")]
	CodeLATE,
	#[serde(rename = "LIQU")]
	CodeLIQU,
	#[serde(rename = "PRCY")]
	CodePRCY,
	#[serde(rename = "REGT")]
	CodeREGT,
	#[serde(rename = "SETS")]
	CodeSETS,
	#[serde(rename = "CERT")]
	CodeCERT,
	#[serde(rename = "PRSY")]
	CodePRSY,
	#[serde(rename = "INBC")]
	CodeINBC,
}


// FailingReason7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FailingReason7 {
	#[serde(rename = "Cd")]
	pub cd: FailingReason7Choice,
	#[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
	pub addtl_rsn_inf: Option<Max210Text>,
}


// FailingReason7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FailingReason7Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<FailingReason3Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// FailingStatus9Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FailingStatus9Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<FailingReason7>>,
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


// Frequency22Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Frequency22Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<EventFrequency7Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
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
#[serde(transparent)]
pub struct IBAN2007Identifier {
	#[serde(rename = "$value")]
	pub iban2007_identifier: String,
}


// ISO20022MessageIdentificationText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISO20022MessageIdentificationText {
	#[serde(rename = "$value")]
	pub iso20022_message_identification_text: String,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}


// ImpliedCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ImpliedCurrencyAndAmount {
	#[serde(rename = "$value")]
	pub implied_currency_and_amount: f64,
}


// IntraBalanceMovementPendingReportV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IntraBalanceMovementPendingReportV02 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<DocumentIdentification51>,
	#[serde(rename = "Pgntn")]
	pub pgntn: Pagination1,
	#[serde(rename = "RptGnlDtls")]
	pub rpt_gnl_dtls: IntraBalanceReport6,
	#[serde(rename = "CshAcct")]
	pub csh_acct: CashAccount40,
	#[serde(rename = "CshAcctOwnr", skip_serializing_if = "Option::is_none")]
	pub csh_acct_ownr: Option<SystemPartyIdentification8>,
	#[serde(rename = "CshAcctSvcr", skip_serializing_if = "Option::is_none")]
	pub csh_acct_svcr: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "Mvmnts", skip_serializing_if = "Option::is_none")]
	pub mvmnts: Option<Vec<IntraBalancePending5>>,
}


// IntraBalancePending5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IntraBalancePending5 {
	#[serde(rename = "StsAndRsn", skip_serializing_if = "Option::is_none")]
	pub sts_and_rsn: Option<PendingStatusAndReason2>,
	#[serde(rename = "Mvmnt")]
	pub mvmnt: Vec<IntraBalancePending6>,
}


// IntraBalancePending6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IntraBalancePending6 {
	#[serde(rename = "StsAndRsn", skip_serializing_if = "Option::is_none")]
	pub sts_and_rsn: Option<PendingStatusAndReason2>,
	#[serde(rename = "AcctOwnrTxId")]
	pub acct_ownr_tx_id: Max35Text,
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
	#[serde(rename = "BalFr")]
	pub bal_fr: CashSubBalanceTypeAndQuantityBreakdown3,
	#[serde(rename = "BalTo")]
	pub bal_to: CashSubBalanceTypeAndQuantityBreakdown3,
	#[serde(rename = "SttlmAmt")]
	pub sttlm_amt: Amount2Choice,
	#[serde(rename = "IntnddSttlmDt")]
	pub intndd_sttlm_dt: DateAndDateTime2Choice,
	#[serde(rename = "StsDt", skip_serializing_if = "Option::is_none")]
	pub sts_dt: Option<String>,
	#[serde(rename = "CshSubBalId", skip_serializing_if = "Option::is_none")]
	pub csh_sub_bal_id: Option<GenericIdentification37>,
	#[serde(rename = "Lnkgs", skip_serializing_if = "Option::is_none")]
	pub lnkgs: Option<Vec<Linkages57>>,
	#[serde(rename = "Prty", skip_serializing_if = "Option::is_none")]
	pub prty: Option<PriorityNumeric4Choice>,
	#[serde(rename = "MsgOrgtr", skip_serializing_if = "Option::is_none")]
	pub msg_orgtr: Option<SystemPartyIdentification8>,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
	#[serde(rename = "InstrPrcgAddtlDtls", skip_serializing_if = "Option::is_none")]
	pub instr_prcg_addtl_dtls: Option<Max350Text>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// IntraBalanceReport6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IntraBalanceReport6 {
	#[serde(rename = "RptNb", skip_serializing_if = "Option::is_none")]
	pub rpt_nb: Option<Number3Choice>,
	#[serde(rename = "QryRef", skip_serializing_if = "Option::is_none")]
	pub qry_ref: Option<Max35Text>,
	#[serde(rename = "RptId", skip_serializing_if = "Option::is_none")]
	pub rpt_id: Option<Max35Text>,
	#[serde(rename = "RptDtTm", skip_serializing_if = "Option::is_none")]
	pub rpt_dt_tm: Option<DateAndDateTime2Choice>,
	#[serde(rename = "RptPrd", skip_serializing_if = "Option::is_none")]
	pub rpt_prd: Option<Period7Choice>,
	#[serde(rename = "Frqcy", skip_serializing_if = "Option::is_none")]
	pub frqcy: Option<Frequency22Choice>,
	#[serde(rename = "UpdTp")]
	pub upd_tp: UpdateType15Choice,
	#[serde(rename = "ActvtyInd")]
	pub actvty_ind: bool,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}


// Linkages57 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Linkages57 {
	#[serde(rename = "PrcgPos", skip_serializing_if = "Option::is_none")]
	pub prcg_pos: Option<ProcessingPosition7Choice>,
	#[serde(rename = "MsgNb", skip_serializing_if = "Option::is_none")]
	pub msg_nb: Option<DocumentNumber5Choice>,
	#[serde(rename = "Ref")]
	pub ref_attr: References34Choice,
	#[serde(rename = "RefOwnr", skip_serializing_if = "Option::is_none")]
	pub ref_ownr: Option<PartyIdentification127Choice>,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
}


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max16Text {
	#[serde(rename = "$value")]
	pub max16_text: String,
}


// Max2048Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max2048Text {
	#[serde(rename = "$value")]
	pub max2048_text: String,
}


// Max210Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max210Text {
	#[serde(rename = "$value")]
	pub max210_text: String,
}


// Max34Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max34Text {
	#[serde(rename = "$value")]
	pub max34_text: String,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}


// Max5NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max5NumericText {
	#[serde(rename = "$value")]
	pub max5_numeric_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
}


// NameAndAddress5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<PostalAddress1>,
}


// NoReasonCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NoReasonCode {
	#[default]
	#[serde(rename = "NORE")]
	CodeNORE,
}


// Number3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number3Choice {
	#[serde(rename = "Shrt", skip_serializing_if = "Option::is_none")]
	pub shrt: Option<Exact3NumericText>,
	#[serde(rename = "Lng", skip_serializing_if = "Option::is_none")]
	pub lng: Option<Exact5NumericText>,
}


// Pagination1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Pagination1 {
	#[serde(rename = "PgNb")]
	pub pg_nb: Max5NumericText,
	#[serde(rename = "LastPgInd")]
	pub last_pg_ind: bool,
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


// PartyIdentification127Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification127Choice {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification36>,
}


// PartyIdentification136 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification136 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification120Choice,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
}


// PendingReason10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PendingReason10Code {
	#[default]
	#[serde(rename = "AWMO")]
	CodeAWMO,
	#[serde(rename = "ADEA")]
	CodeADEA,
	#[serde(rename = "CAIS")]
	CodeCAIS,
	#[serde(rename = "REFU")]
	CodeREFU,
	#[serde(rename = "AWSH")]
	CodeAWSH,
	#[serde(rename = "PHSE")]
	CodePHSE,
	#[serde(rename = "TAMM")]
	CodeTAMM,
	#[serde(rename = "DOCY")]
	CodeDOCY,
	#[serde(rename = "DOCC")]
	CodeDOCC,
	#[serde(rename = "BLOC")]
	CodeBLOC,
	#[serde(rename = "CHAS")]
	CodeCHAS,
	#[serde(rename = "NEWI")]
	CodeNEWI,
	#[serde(rename = "CLAC")]
	CodeCLAC,
	#[serde(rename = "MUNO")]
	CodeMUNO,
	#[serde(rename = "GLOB")]
	CodeGLOB,
	#[serde(rename = "PREA")]
	CodePREA,
	#[serde(rename = "PART")]
	CodePART,
	#[serde(rename = "NMAS")]
	CodeNMAS,
	#[serde(rename = "NOFX")]
	CodeNOFX,
	#[serde(rename = "CMON")]
	CodeCMON,
	#[serde(rename = "YCOL")]
	CodeYCOL,
	#[serde(rename = "COLL")]
	CodeCOLL,
	#[serde(rename = "DEPO")]
	CodeDEPO,
	#[serde(rename = "FLIM")]
	CodeFLIM,
	#[serde(rename = "INCA")]
	CodeINCA,
	#[serde(rename = "LINK")]
	CodeLINK,
	#[serde(rename = "FUTU")]
	CodeFUTU,
	#[serde(rename = "LACK")]
	CodeLACK,
	#[serde(rename = "LALO")]
	CodeLALO,
	#[serde(rename = "MONY")]
	CodeMONY,
	#[serde(rename = "NCON")]
	CodeNCON,
	#[serde(rename = "REFS")]
	CodeREFS,
	#[serde(rename = "SDUT")]
	CodeSDUT,
	#[serde(rename = "BATC")]
	CodeBATC,
	#[serde(rename = "CYCL")]
	CodeCYCL,
	#[serde(rename = "SBLO")]
	CodeSBLO,
	#[serde(rename = "CPEC")]
	CodeCPEC,
	#[serde(rename = "MINO")]
	CodeMINO,
	#[serde(rename = "IAAD")]
	CodeIAAD,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "PHCK")]
	CodePHCK,
	#[serde(rename = "BENO")]
	CodeBENO,
	#[serde(rename = "BOTH")]
	CodeBOTH,
	#[serde(rename = "CLHT")]
	CodeCLHT,
	#[serde(rename = "DENO")]
	CodeDENO,
	#[serde(rename = "DISA")]
	CodeDISA,
	#[serde(rename = "DKNY")]
	CodeDKNY,
	#[serde(rename = "FROZ")]
	CodeFROZ,
	#[serde(rename = "LAAW")]
	CodeLAAW,
	#[serde(rename = "LATE")]
	CodeLATE,
	#[serde(rename = "LIQU")]
	CodeLIQU,
	#[serde(rename = "PRCY")]
	CodePRCY,
	#[serde(rename = "REGT")]
	CodeREGT,
	#[serde(rename = "SETS")]
	CodeSETS,
	#[serde(rename = "CERT")]
	CodeCERT,
	#[serde(rename = "PRSY")]
	CodePRSY,
	#[serde(rename = "INBC")]
	CodeINBC,
}


// PendingReason14 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingReason14 {
	#[serde(rename = "Cd")]
	pub cd: PendingReason26Choice,
	#[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
	pub addtl_rsn_inf: Option<Max210Text>,
}


// PendingReason26Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingReason26Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PendingReason10Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// PendingStatus36Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingStatus36Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<PendingReason14>>,
}


// PendingStatusAndReason2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingStatusAndReason2 {
	#[serde(rename = "PrcgSts", skip_serializing_if = "Option::is_none")]
	pub prcg_sts: Option<Vec<ProcessingStatus66Choice>>,
	#[serde(rename = "SttlmSts", skip_serializing_if = "Option::is_none")]
	pub sttlm_sts: Option<Vec<SettlementStatus16Choice>>,
}


// Period2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// Period7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period7Choice {
	#[serde(rename = "FrDtTmToDtTm", skip_serializing_if = "Option::is_none")]
	pub fr_dt_tm_to_dt_tm: Option<DateTimePeriod1>,
	#[serde(rename = "FrDtToDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt_to_dt: Option<Period2>,
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


// ProcessingPosition3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ProcessingPosition3Code {
	#[default]
	#[serde(rename = "AFTE")]
	CodeAFTE,
	#[serde(rename = "WITH")]
	CodeWITH,
	#[serde(rename = "BEFO")]
	CodeBEFO,
	#[serde(rename = "INFO")]
	CodeINFO,
}


// ProcessingPosition7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProcessingPosition7Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ProcessingPosition3Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// ProcessingStatus66Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProcessingStatus66Choice {
	#[serde(rename = "AckdAccptd", skip_serializing_if = "Option::is_none")]
	pub ackd_accptd: Option<AcknowledgedAcceptedStatus21Choice>,
	#[serde(rename = "Rpr", skip_serializing_if = "Option::is_none")]
	pub rpr: Option<RejectionOrRepairStatus38Choice>,
	#[serde(rename = "Canc", skip_serializing_if = "Option::is_none")]
	pub canc: Option<CancellationStatus14Choice>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<ProprietaryStatusAndReason6>,
}


// ProprietaryReason4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryReason4 {
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<GenericIdentification30>,
	#[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
	pub addtl_rsn_inf: Option<Max210Text>,
}


// ProprietaryStatusAndReason6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryStatusAndReason6 {
	#[serde(rename = "PrtrySts")]
	pub prtry_sts: GenericIdentification30,
	#[serde(rename = "PrtryRsn", skip_serializing_if = "Option::is_none")]
	pub prtry_rsn: Option<Vec<ProprietaryReason4>>,
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


// References34Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct References34Choice {
	#[serde(rename = "SctiesSttlmTxId", skip_serializing_if = "Option::is_none")]
	pub scties_sttlm_tx_id: Option<Max35Text>,
	#[serde(rename = "IntraPosMvmntId", skip_serializing_if = "Option::is_none")]
	pub intra_pos_mvmnt_id: Option<Max35Text>,
	#[serde(rename = "IntraBalMvmntId", skip_serializing_if = "Option::is_none")]
	pub intra_bal_mvmnt_id: Option<Max35Text>,
	#[serde(rename = "AcctSvcrTxId", skip_serializing_if = "Option::is_none")]
	pub acct_svcr_tx_id: Option<Max35Text>,
	#[serde(rename = "MktInfrstrctrTxId", skip_serializing_if = "Option::is_none")]
	pub mkt_infrstrctr_tx_id: Option<Max35Text>,
	#[serde(rename = "PoolId", skip_serializing_if = "Option::is_none")]
	pub pool_id: Option<Max35Text>,
	#[serde(rename = "OthrTxId", skip_serializing_if = "Option::is_none")]
	pub othr_tx_id: Option<Max35Text>,
}


// RejectionAndRepairReason32Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RejectionAndRepairReason32Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<RejectionReason33Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// RejectionOrRepairReason32 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RejectionOrRepairReason32 {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Vec<RejectionAndRepairReason32Choice>>,
	#[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
	pub addtl_rsn_inf: Option<Max210Text>,
}


// RejectionOrRepairStatus38Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RejectionOrRepairStatus38Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<RejectionOrRepairReason32>>,
}


// RejectionReason33Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RejectionReason33Code {
	#[default]
	#[serde(rename = "CASH")]
	CodeCASH,
	#[serde(rename = "ADEA")]
	CodeADEA,
	#[serde(rename = "DMON")]
	CodeDMON,
	#[serde(rename = "NCRR")]
	CodeNCRR,
	#[serde(rename = "LATE")]
	CodeLATE,
	#[serde(rename = "INVL")]
	CodeINVL,
	#[serde(rename = "INVB")]
	CodeINVB,
	#[serde(rename = "INVN")]
	CodeINVN,
	#[serde(rename = "VALR")]
	CodeVALR,
	#[serde(rename = "MONY")]
	CodeMONY,
	#[serde(rename = "CAEV")]
	CodeCAEV,
	#[serde(rename = "DDAT")]
	CodeDDAT,
	#[serde(rename = "REFE")]
	CodeREFE,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "DQUA")]
	CodeDQUA,
	#[serde(rename = "DSEC")]
	CodeDSEC,
	#[serde(rename = "MINO")]
	CodeMINO,
	#[serde(rename = "MUNO")]
	CodeMUNO,
}


// SettlementStatus16Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementStatus16Choice {
	#[serde(rename = "Pdg", skip_serializing_if = "Option::is_none")]
	pub pdg: Option<PendingStatus36Choice>,
	#[serde(rename = "Flng", skip_serializing_if = "Option::is_none")]
	pub flng: Option<FailingStatus9Choice>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<ProprietaryStatusAndReason6>,
}


// StatementUpdateType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum StatementUpdateType1Code {
	#[default]
	#[serde(rename = "COMP")]
	CodeCOMP,
	#[serde(rename = "DELT")]
	CodeDELT,
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


// UpdateType15Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateType15Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<StatementUpdateType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct YesNoIndicator {
	#[serde(rename = "$value")]
	pub yes_no_indicator: bool,
}
