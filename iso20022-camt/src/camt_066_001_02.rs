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
	pub cdt_dbt: Option<String>,
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


// CashBalanceType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashBalanceType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
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


// CopyDuplicate1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CopyDuplicate1Code {
	#[serde(rename = "CopyDuplicate1Code")]
	pub copy_duplicate1_code: String,
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
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<String>,
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "DecimalNumber")]
	pub decimal_number: f64,
}


// DocumentNumber5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentNumber5Choice {
	#[serde(rename = "ShrtNb", skip_serializing_if = "Option::is_none")]
	pub shrt_nb: Option<String>,
	#[serde(rename = "LngNb", skip_serializing_if = "Option::is_none")]
	pub lng_nb: Option<String>,
	#[serde(rename = "PrtryNb", skip_serializing_if = "Option::is_none")]
	pub prtry_nb: Option<GenericIdentification36>,
}


// Exact3NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact3NumericText {
	#[serde(rename = "Exact3NumericText")]
	pub exact3_numeric_text: String,
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "Exact4AlphaNumericText")]
	pub exact4_alpha_numeric_text: String,
}


// Exact4NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact4NumericText {
	#[serde(rename = "Exact4NumericText")]
	pub exact4_numeric_text: String,
}


// ExternalAccountIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAccountIdentification1Code {
	#[serde(rename = "ExternalAccountIdentification1Code")]
	pub external_account_identification1_code: String,
}


// ExternalBalanceType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalBalanceType1Code {
	#[serde(rename = "ExternalBalanceType1Code")]
	pub external_balance_type1_code: String,
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


// ExternalFinancialInstitutionIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalFinancialInstitutionIdentification1Code {
	#[serde(rename = "ExternalFinancialInstitutionIdentification1Code")]
	pub external_financial_institution_identification1_code: String,
}


// ExternalProxyAccountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalProxyAccountType1Code {
	#[serde(rename = "ExternalProxyAccountType1Code")]
	pub external_proxy_account_type1_code: String,
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


// GenericIdentification36 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification36 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<String>,
}


// GenericIdentification37 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification37 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}


// IBAN2007Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IBAN2007Identifier {
	#[serde(rename = "IBAN2007Identifier")]
	pub iban2007_identifier: String,
}


// ISO20022MessageIdentificationText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISO20022MessageIdentificationText {
	#[serde(rename = "ISO20022MessageIdentificationText")]
	pub iso20022_message_identification_text: String,
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


// ImpliedCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImpliedCurrencyAndAmount {
	#[serde(rename = "ImpliedCurrencyAndAmount")]
	pub implied_currency_and_amount: f64,
}


// IntraBalance5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IntraBalance5 {
	#[serde(rename = "SttlmAmt")]
	pub sttlm_amt: Amount2Choice,
	#[serde(rename = "SttlmDt")]
	pub sttlm_dt: DateAndDateTime2Choice,
	#[serde(rename = "BalFr")]
	pub bal_fr: CashSubBalanceTypeAndQuantityBreakdown3,
	#[serde(rename = "BalTo")]
	pub bal_to: CashSubBalanceTypeAndQuantityBreakdown3,
	#[serde(rename = "CshSubBalId", skip_serializing_if = "Option::is_none")]
	pub csh_sub_bal_id: Option<GenericIdentification37>,
	#[serde(rename = "Prty", skip_serializing_if = "Option::is_none")]
	pub prty: Option<PriorityNumeric4Choice>,
	#[serde(rename = "InstrPrcgAddtlDtls", skip_serializing_if = "Option::is_none")]
	pub instr_prcg_addtl_dtls: Option<String>,
}


// IntraBalanceMovementInstructionV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IntraBalanceMovementInstructionV02 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<TransactionAndDocumentIdentification6>,
	#[serde(rename = "CorpActnEvtId", skip_serializing_if = "Option::is_none")]
	pub corp_actn_evt_id: Option<String>,
	#[serde(rename = "NbCounts", skip_serializing_if = "Option::is_none")]
	pub nb_counts: Option<NumberCount1Choice>,
	#[serde(rename = "Lnkgs", skip_serializing_if = "Option::is_none")]
	pub lnkgs: Option<Vec<Linkages57>>,
	#[serde(rename = "CshAcct")]
	pub csh_acct: CashAccount40,
	#[serde(rename = "CshAcctOwnr", skip_serializing_if = "Option::is_none")]
	pub csh_acct_ownr: Option<SystemPartyIdentification8>,
	#[serde(rename = "CshAcctSvcr", skip_serializing_if = "Option::is_none")]
	pub csh_acct_svcr: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "IntraBal")]
	pub intra_bal: IntraBalance5,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
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


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// NameAndAddress5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<PostalAddress1>,
}


// NumberCount1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NumberCount1Choice {
	#[serde(rename = "CurInstrNb", skip_serializing_if = "Option::is_none")]
	pub cur_instr_nb: Option<String>,
	#[serde(rename = "TtlNb", skip_serializing_if = "Option::is_none")]
	pub ttl_nb: Option<TotalNumber1>,
}


// PartyIdentification120Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification120Choice {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<String>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification36>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
}


// PartyIdentification127Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification127Choice {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<String>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification36>,
}


// PartyIdentification136 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification136 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification120Choice,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<String>,
}


// PostalAddress1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress1 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<String>,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
	pub adr_line: Option<Vec<String>>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<String>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<String>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<String>,
	#[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
	pub twn_nm: Option<String>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<String>,
	#[serde(rename = "Ctry")]
	pub ctry: String,
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


// PriorityNumeric4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriorityNumeric4Choice {
	#[serde(rename = "Nmrc", skip_serializing_if = "Option::is_none")]
	pub nmrc: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// ProcessingPosition3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProcessingPosition3Code {
	#[serde(rename = "ProcessingPosition3Code")]
	pub processing_position3_code: String,
}


// ProcessingPosition7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProcessingPosition7Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
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


// References34Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct References34Choice {
	#[serde(rename = "SctiesSttlmTxId", skip_serializing_if = "Option::is_none")]
	pub scties_sttlm_tx_id: Option<String>,
	#[serde(rename = "IntraPosMvmntId", skip_serializing_if = "Option::is_none")]
	pub intra_pos_mvmnt_id: Option<String>,
	#[serde(rename = "IntraBalMvmntId", skip_serializing_if = "Option::is_none")]
	pub intra_bal_mvmnt_id: Option<String>,
	#[serde(rename = "AcctSvcrTxId", skip_serializing_if = "Option::is_none")]
	pub acct_svcr_tx_id: Option<String>,
	#[serde(rename = "MktInfrstrctrTxId", skip_serializing_if = "Option::is_none")]
	pub mkt_infrstrctr_tx_id: Option<String>,
	#[serde(rename = "PoolId", skip_serializing_if = "Option::is_none")]
	pub pool_id: Option<String>,
	#[serde(rename = "OthrTxId", skip_serializing_if = "Option::is_none")]
	pub othr_tx_id: Option<String>,
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


// SystemPartyIdentification8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemPartyIdentification8 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification136,
	#[serde(rename = "RspnsblPtyId", skip_serializing_if = "Option::is_none")]
	pub rspnsbl_pty_id: Option<PartyIdentification136>,
}


// TotalNumber1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TotalNumber1 {
	#[serde(rename = "CurInstrNb")]
	pub cur_instr_nb: String,
	#[serde(rename = "TtlOfLkdInstrs")]
	pub ttl_of_lkd_instrs: String,
}


// TransactionAndDocumentIdentification6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionAndDocumentIdentification6 {
	#[serde(rename = "TxId")]
	pub tx_id: String,
	#[serde(rename = "DocId", skip_serializing_if = "Option::is_none")]
	pub doc_id: Option<String>,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<DateAndDateTime2Choice>,
	#[serde(rename = "CpyDplct", skip_serializing_if = "Option::is_none")]
	pub cpy_dplct: Option<String>,
	#[serde(rename = "MsgOrgtr", skip_serializing_if = "Option::is_none")]
	pub msg_orgtr: Option<PartyIdentification136>,
	#[serde(rename = "MsgRcpt", skip_serializing_if = "Option::is_none")]
	pub msg_rcpt: Option<PartyIdentification136>,
}
