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


// ActiveCurrencyAnd13DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd13DecimalAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_currency_and13_decimal_amount_simple_type: f64,
}


// ActiveCurrencyAnd13DecimalAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd13DecimalAmount {
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


// ActiveOrHistoricCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
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
	#[serde(rename = "$value")]
	pub active_or_historic_currency_code: String,
}


// AmountAndDirection53 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection53 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
	pub sgn: Option<bool>,
}


// AmountAndDirection61 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection61 {
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAnd13DecimalAmount,
	#[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
	pub sgn: Option<bool>,
}


// AssetClassAttributes1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassAttributes1 {
	#[serde(rename = "Intrst")]
	pub intrst: DerivativeInterest2,
	#[serde(rename = "FX")]
	pub fx: DerivativeForeignExchange2,
}


// AssetClassAttributes1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassAttributes1Choice {
	#[serde(rename = "Intrst", skip_serializing_if = "Option::is_none")]
	pub intrst: Option<DerivativeInterest2>,
	#[serde(rename = "FX", skip_serializing_if = "Option::is_none")]
	pub fx: Option<DerivativeForeignExchange2>,
	#[serde(rename = "Both", skip_serializing_if = "Option::is_none")]
	pub both: Option<AssetClassAttributes1>,
}


// BasketDescription3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BasketDescription3 {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<Vec<ISINOct2015Identifier>>,
	#[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
	pub indx: Option<Vec<FinancialInstrument58>>,
}


// BenchmarkCurveName2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BenchmarkCurveName2Code {
	#[default]
	#[serde(rename = "WIBO")]
	CodeWIBO,
	#[serde(rename = "TREA")]
	CodeTREA,
	#[serde(rename = "TIBO")]
	CodeTIBO,
	#[serde(rename = "TLBO")]
	CodeTLBO,
	#[serde(rename = "SWAP")]
	CodeSWAP,
	#[serde(rename = "STBO")]
	CodeSTBO,
	#[serde(rename = "PRBO")]
	CodePRBO,
	#[serde(rename = "PFAN")]
	CodePFAN,
	#[serde(rename = "NIBO")]
	CodeNIBO,
	#[serde(rename = "MAAA")]
	CodeMAAA,
	#[serde(rename = "MOSP")]
	CodeMOSP,
	#[serde(rename = "LIBO")]
	CodeLIBO,
	#[serde(rename = "LIBI")]
	CodeLIBI,
	#[serde(rename = "JIBA")]
	CodeJIBA,
	#[serde(rename = "ISDA")]
	CodeISDA,
	#[serde(rename = "GCFR")]
	CodeGCFR,
	#[serde(rename = "FUSW")]
	CodeFUSW,
	#[serde(rename = "EUCH")]
	CodeEUCH,
	#[serde(rename = "EUUS")]
	CodeEUUS,
	#[serde(rename = "EURI")]
	CodeEURI,
	#[serde(rename = "EONS")]
	CodeEONS,
	#[serde(rename = "EONA")]
	CodeEONA,
	#[serde(rename = "CIBO")]
	CodeCIBO,
	#[serde(rename = "CDOR")]
	CodeCDOR,
	#[serde(rename = "BUBO")]
	CodeBUBO,
	#[serde(rename = "BBSW")]
	CodeBBSW,
}


// BenchmarkCurveName5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkCurveName5Choice {
	#[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
	pub indx: Option<BenchmarkCurveName2Code>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max25Text>,
}


// CFIOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[serde(rename = "$value")]
	pub cfi_oct2015_identifier: String,
}


// CancelledStatusReason15Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CancelledStatusReason15Code {
	#[default]
	#[serde(rename = "CANI")]
	CodeCANI,
	#[serde(rename = "CSUB")]
	CodeCSUB,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// DTI2021Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DTI2021Identifier {
	#[serde(rename = "$value")]
	pub dti2021_identifier: String,
}


// DebtInstrument4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DebtInstrument4 {
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: String,
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "$value")]
	pub decimal_number: f64,
}


// DerivativeForeignExchange2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativeForeignExchange2 {
	#[serde(rename = "OthrNtnlCcy")]
	pub othr_ntnl_ccy: ActiveOrHistoricCurrencyCode,
}


// DerivativeInstrument6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativeInstrument6 {
	#[serde(rename = "XpryDt", skip_serializing_if = "Option::is_none")]
	pub xpry_dt: Option<String>,
	#[serde(rename = "PricMltplr")]
	pub pric_mltplr: f64,
	#[serde(rename = "UndrlygInstrm")]
	pub undrlyg_instrm: UnderlyingIdentification2Choice,
	#[serde(rename = "OptnTp", skip_serializing_if = "Option::is_none")]
	pub optn_tp: Option<OptionType2Code>,
	#[serde(rename = "StrkPric", skip_serializing_if = "Option::is_none")]
	pub strk_pric: Option<SecuritiesTransactionPrice4Choice>,
	#[serde(rename = "OptnExrcStyle", skip_serializing_if = "Option::is_none")]
	pub optn_exrc_style: Option<OptionStyle7Code>,
	#[serde(rename = "DlvryTp")]
	pub dlvry_tp: PhysicalTransferType4Code,
	#[serde(rename = "AsstClssSpcfcAttrbts", skip_serializing_if = "Option::is_none")]
	pub asst_clss_spcfc_attrbts: Option<AssetClassAttributes1Choice>,
}


// DerivativeInterest2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativeInterest2 {
	#[serde(rename = "OthrNtnlCcy")]
	pub othr_ntnl_ccy: ActiveOrHistoricCurrencyCode,
}


// DigitalTokenAmount2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DigitalTokenAmount2 {
	#[serde(rename = "Idr")]
	pub idr: DTI2021Identifier,
	#[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
	pub unit: Option<f64>,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max30Text>,
}


// ExecutingParty1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExecutingParty1Choice {
	#[serde(rename = "Prsn", skip_serializing_if = "Option::is_none")]
	pub prsn: Option<PersonIdentification12>,
	#[serde(rename = "Algo", skip_serializing_if = "Option::is_none")]
	pub algo: Option<Max50Text>,
	#[serde(rename = "Clnt", skip_serializing_if = "Option::is_none")]
	pub clnt: Option<NoReasonCode>,
}


// ExternalAuthorityExchangeReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAuthorityExchangeReason1Code {
	#[serde(rename = "$value")]
	pub external_authority_exchange_reason1_code: String,
}


// ExternalFinancialInstrumentIdentificationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalFinancialInstrumentIdentificationType1Code {
	#[serde(rename = "$value")]
	pub external_financial_instrument_identification_type1_code: String,
}


// ExternalPersonIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPersonIdentification1Code {
	#[serde(rename = "$value")]
	pub external_person_identification1_code: String,
}


// FinancialInstrument58 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument58 {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
	#[serde(rename = "Nm")]
	pub nm: FloatingInterestRate8,
}


// FinancialInstrumentAttributes5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentAttributes5Choice {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ISINOct2015Identifier>,
	#[serde(rename = "AltrnId", skip_serializing_if = "Option::is_none")]
	pub altrn_id: Option<SecurityIdentification19>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<SecurityInstrumentDescription22>,
}


// FinancialInstrumentIdentification6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentIdentification6Choice {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
	#[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
	pub indx: Option<FinancialInstrument58>,
}


// FinancialInstrumentIdentification7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentIdentification7Choice {
	#[serde(rename = "Sngl", skip_serializing_if = "Option::is_none")]
	pub sngl: Option<FinancialInstrumentIdentification6Choice>,
	#[serde(rename = "Bskt", skip_serializing_if = "Option::is_none")]
	pub bskt: Option<BasketDescription3>,
}


// FinancialInstrumentQuantity25Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentQuantity25Choice {
	#[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
	pub unit: Option<f64>,
	#[serde(rename = "NmnlVal", skip_serializing_if = "Option::is_none")]
	pub nmnl_val: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
	pub mntry_val: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// FinancialInstrumentReportingTransactionReportV03 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentReportingTransactionReportV03 {
	#[serde(rename = "Tx")]
	pub tx: Vec<ReportingTransactionType3Choice>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// FloatingInterestRate8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FloatingInterestRate8 {
	#[serde(rename = "RefRate")]
	pub ref_rate: BenchmarkCurveName5Choice,
	#[serde(rename = "Term", skip_serializing_if = "Option::is_none")]
	pub term: Option<InterestRateContractTerm2>,
}


// GenericPersonIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericPersonIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}


// ISINOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "$value")]
	pub isin_oct2015_identifier: String,
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


// IdentificationSource3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IdentificationSource3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalFinancialInstrumentIdentificationType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// ImpliedCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImpliedCurrencyAndAmount {
	#[serde(rename = "$value")]
	pub implied_currency_and_amount: f64,
}


// InterestRateContractTerm2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRateContractTerm2 {
	#[serde(rename = "Unit")]
	pub unit: RateBasis1Code,
	#[serde(rename = "Val")]
	pub val: f64,
}


// InternalPartyRole1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InternalPartyRole1Code {
	#[default]
	#[serde(rename = "INTC")]
	CodeINTC,
}


// InvestmentParty1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentParty1Choice {
	#[serde(rename = "Prsn", skip_serializing_if = "Option::is_none")]
	pub prsn: Option<PersonIdentification12>,
	#[serde(rename = "Algo", skip_serializing_if = "Option::is_none")]
	pub algo: Option<Max50Text>,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}


// MICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MICIdentifier {
	#[serde(rename = "$value")]
	pub mic_identifier: String,
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


// Max25Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max25Text {
	#[serde(rename = "$value")]
	pub max25_text: String,
}


// Max30DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max30DecimalNumber {
	#[serde(rename = "$value")]
	pub max30_decimal_number: f64,
}


// Max30Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max30Text {
	#[serde(rename = "$value")]
	pub max30_text: String,
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


// Max3Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max3Number {
	#[serde(rename = "$value")]
	pub max3_number: f64,
}


// Max50Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max50Text {
	#[serde(rename = "$value")]
	pub max50_text: String,
}


// Max52Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max52Text {
	#[serde(rename = "$value")]
	pub max52_text: String,
}


// NoReasonCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NoReasonCode {
	#[default]
	#[serde(rename = "NORE")]
	CodeNORE,
}


// NonNegativeDecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NonNegativeDecimalNumber {
	#[serde(rename = "$value")]
	pub non_negative_decimal_number: f64,
}


// OptionStyle7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OptionStyle7Code {
	#[default]
	#[serde(rename = "AMER")]
	CodeAMER,
	#[serde(rename = "ASIA")]
	CodeASIA,
	#[serde(rename = "BERM")]
	CodeBERM,
	#[serde(rename = "EURO")]
	CodeEURO,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}


// OptionType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OptionType2Code {
	#[default]
	#[serde(rename = "CALL")]
	CodeCALL,
	#[serde(rename = "PUTO")]
	CodePUTO,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}


// OtherIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Sfx", skip_serializing_if = "Option::is_none")]
	pub sfx: Option<Max16Text>,
	#[serde(rename = "Tp")]
	pub tp: IdentificationSource3Choice,
}


// PartyIdentification76 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification76 {
	#[serde(rename = "Id")]
	pub id: PersonOrOrganisation1Choice,
	#[serde(rename = "CtryOfBrnch", skip_serializing_if = "Option::is_none")]
	pub ctry_of_brnch: Option<CountryCode>,
}


// PartyIdentification79 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification79 {
	#[serde(rename = "AcctOwnr")]
	pub acct_ownr: Vec<PartyIdentification76>,
	#[serde(rename = "DcsnMakr", skip_serializing_if = "Option::is_none")]
	pub dcsn_makr: Option<Vec<PersonOrOrganisation2Choice>>,
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "$value")]
	pub percentage_rate: f64,
}


// PersonIdentification10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentification10 {
	#[serde(rename = "FrstNm")]
	pub frst_nm: Max140Text,
	#[serde(rename = "Nm")]
	pub nm: Max140Text,
	#[serde(rename = "BirthDt")]
	pub birth_dt: String,
	#[serde(rename = "Othr")]
	pub othr: GenericPersonIdentification1,
}


// PersonIdentification12 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentification12 {
	#[serde(rename = "CtryOfBrnch")]
	pub ctry_of_brnch: CountryCode,
	#[serde(rename = "Othr")]
	pub othr: GenericPersonIdentification1,
}


// PersonIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalPersonIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// PersonOrOrganisation1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonOrOrganisation1Choice {
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "MIC", skip_serializing_if = "Option::is_none")]
	pub mic: Option<MICIdentifier>,
	#[serde(rename = "Prsn", skip_serializing_if = "Option::is_none")]
	pub prsn: Option<PersonIdentification10>,
	#[serde(rename = "Intl", skip_serializing_if = "Option::is_none")]
	pub intl: Option<InternalPartyRole1Code>,
}


// PersonOrOrganisation2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonOrOrganisation2Choice {
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Prsn", skip_serializing_if = "Option::is_none")]
	pub prsn: Option<PersonIdentification10>,
}


// PhysicalTransferType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PhysicalTransferType4Code {
	#[default]
	#[serde(rename = "PHYS")]
	CodePHYS,
	#[serde(rename = "OPTL")]
	CodeOPTL,
	#[serde(rename = "CASH")]
	CodeCASH,
}


// PlusOrMinusIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "$value")]
	pub plus_or_minus_indicator: bool,
}


// PriceStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PriceStatus1Code {
	#[default]
	#[serde(rename = "PNDG")]
	CodePNDG,
	#[serde(rename = "NOAP")]
	CodeNOAP,
}


// RateBasis1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RateBasis1Code {
	#[default]
	#[serde(rename = "DAYS")]
	CodeDAYS,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "YEAR")]
	CodeYEAR,
}


// RecordTechnicalData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RecordTechnicalData2 {
	#[serde(rename = "RctDtTm")]
	pub rct_dt_tm: String,
	#[serde(rename = "CxlRsn")]
	pub cxl_rsn: CancelledStatusReason15Code,
}


// RecordTechnicalData5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RecordTechnicalData5 {
	#[serde(rename = "RctDtTm")]
	pub rct_dt_tm: String,
	#[serde(rename = "XchgRsn")]
	pub xchg_rsn: Vec<ExternalAuthorityExchangeReason1Code>,
}


// RegulatoryTradingCapacity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RegulatoryTradingCapacity1Code {
	#[default]
	#[serde(rename = "MTCH")]
	CodeMTCH,
	#[serde(rename = "DEAL")]
	CodeDEAL,
	#[serde(rename = "AOTC")]
	CodeAOTC,
}


// ReportingTransactionType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportingTransactionType3Choice {
	#[serde(rename = "New", skip_serializing_if = "Option::is_none")]
	pub new: Option<SecuritiesTransactionReport7>,
	#[serde(rename = "Cxl", skip_serializing_if = "Option::is_none")]
	pub cxl: Option<SecuritiesTransactionReport2>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// ReportingWaiverType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ReportingWaiverType1Code {
	#[default]
	#[serde(rename = "OILQ")]
	CodeOILQ,
	#[serde(rename = "NLIQ")]
	CodeNLIQ,
	#[serde(rename = "PRIC")]
	CodePRIC,
	#[serde(rename = "ILQD")]
	CodeILQD,
	#[serde(rename = "RFPT")]
	CodeRFPT,
	#[serde(rename = "SIZE")]
	CodeSIZE,
}


// ReportingWaiverType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ReportingWaiverType3Code {
	#[default]
	#[serde(rename = "BENC")]
	CodeBENC,
	#[serde(rename = "ACTX")]
	CodeACTX,
	#[serde(rename = "ILQD")]
	CodeILQD,
	#[serde(rename = "SIZE")]
	CodeSIZE,
	#[serde(rename = "CANC")]
	CodeCANC,
	#[serde(rename = "AMND")]
	CodeAMND,
	#[serde(rename = "SDIV")]
	CodeSDIV,
	#[serde(rename = "RPRI")]
	CodeRPRI,
	#[serde(rename = "DUPL")]
	CodeDUPL,
	#[serde(rename = "LRGS")]
	CodeLRGS,
	#[serde(rename = "TNCP")]
	CodeTNCP,
	#[serde(rename = "TPAC")]
	CodeTPAC,
	#[serde(rename = "XFPH")]
	CodeXFPH,
}


// SecuritiesTransaction3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransaction3 {
	#[serde(rename = "TradDt")]
	pub trad_dt: String,
	#[serde(rename = "TradgCpcty")]
	pub tradg_cpcty: RegulatoryTradingCapacity1Code,
	#[serde(rename = "Qty")]
	pub qty: FinancialInstrumentQuantity25Choice,
	#[serde(rename = "DgtlTknQty", skip_serializing_if = "Option::is_none")]
	pub dgtl_tkn_qty: Option<Vec<DigitalTokenAmount2>>,
	#[serde(rename = "DerivNtnlChng", skip_serializing_if = "Option::is_none")]
	pub deriv_ntnl_chng: Option<VariationType1Code>,
	#[serde(rename = "Pric")]
	pub pric: SecuritiesTransactionPrice22Choice,
	#[serde(rename = "NetAmt", skip_serializing_if = "Option::is_none")]
	pub net_amt: Option<f64>,
	#[serde(rename = "TradVn")]
	pub trad_vn: MICIdentifier,
	#[serde(rename = "CtryOfBrnch", skip_serializing_if = "Option::is_none")]
	pub ctry_of_brnch: Option<CountryCode>,
	#[serde(rename = "UpFrntPmt", skip_serializing_if = "Option::is_none")]
	pub up_frnt_pmt: Option<AmountAndDirection53>,
	#[serde(rename = "TradPlcMtchgId", skip_serializing_if = "Option::is_none")]
	pub trad_plc_mtchg_id: Option<Max52Text>,
	#[serde(rename = "CmplxTradCmpntId", skip_serializing_if = "Option::is_none")]
	pub cmplx_trad_cmpnt_id: Option<Max35Text>,
}


// SecuritiesTransactionIndicator2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionIndicator2 {
	#[serde(rename = "WvrInd", skip_serializing_if = "Option::is_none")]
	pub wvr_ind: Option<Vec<ReportingWaiverType1Code>>,
	#[serde(rename = "ShrtSellgInd", skip_serializing_if = "Option::is_none")]
	pub shrt_sellg_ind: Option<Side5Code>,
	#[serde(rename = "OTCPstTradInd", skip_serializing_if = "Option::is_none")]
	pub otc_pst_trad_ind: Option<Vec<ReportingWaiverType3Code>>,
	#[serde(rename = "RskRdcgTx", skip_serializing_if = "Option::is_none")]
	pub rsk_rdcg_tx: Option<bool>,
	#[serde(rename = "SctiesFincgTxInd")]
	pub scties_fincg_tx_ind: bool,
}


// SecuritiesTransactionPrice1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice1 {
	#[serde(rename = "Pdg")]
	pub pdg: PriceStatus1Code,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveOrHistoricCurrencyCode>,
}


// SecuritiesTransactionPrice22Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice22Choice {
	#[serde(rename = "Pric", skip_serializing_if = "Option::is_none")]
	pub pric: Option<SecuritiesTransactionPrice2Choice>,
	#[serde(rename = "DgtlTknPric", skip_serializing_if = "Option::is_none")]
	pub dgtl_tkn_pric: Option<SecuritiesTransactionPrice7>,
	#[serde(rename = "NoPric", skip_serializing_if = "Option::is_none")]
	pub no_pric: Option<SecuritiesTransactionPrice6>,
}


// SecuritiesTransactionPrice2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice2Choice {
	#[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
	pub mntry_val: Option<AmountAndDirection61>,
	#[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
	pub pctg: Option<f64>,
	#[serde(rename = "Yld", skip_serializing_if = "Option::is_none")]
	pub yld: Option<f64>,
	#[serde(rename = "BsisPts", skip_serializing_if = "Option::is_none")]
	pub bsis_pts: Option<f64>,
}


// SecuritiesTransactionPrice4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice4Choice {
	#[serde(rename = "Pric", skip_serializing_if = "Option::is_none")]
	pub pric: Option<SecuritiesTransactionPrice2Choice>,
	#[serde(rename = "NoPric", skip_serializing_if = "Option::is_none")]
	pub no_pric: Option<SecuritiesTransactionPrice1>,
}


// SecuritiesTransactionPrice6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice6 {
	#[serde(rename = "Pdg")]
	pub pdg: PriceStatus1Code,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "DgtlTkn", skip_serializing_if = "Option::is_none")]
	pub dgtl_tkn: Option<Vec<DigitalTokenAmount2>>,
}


// SecuritiesTransactionPrice7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice7 {
	#[serde(rename = "MntryVal")]
	pub mntry_val: AmountAndDirection61,
	#[serde(rename = "DgtlTknQty")]
	pub dgtl_tkn_qty: DigitalTokenAmount2,
}


// SecuritiesTransactionReport2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionReport2 {
	#[serde(rename = "TxId")]
	pub tx_id: Max52Text,
	#[serde(rename = "ExctgPty")]
	pub exctg_pty: LEIIdentifier,
	#[serde(rename = "SubmitgPty")]
	pub submitg_pty: LEIIdentifier,
	#[serde(rename = "TechAttrbts", skip_serializing_if = "Option::is_none")]
	pub tech_attrbts: Option<RecordTechnicalData2>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// SecuritiesTransactionReport7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionReport7 {
	#[serde(rename = "TxId")]
	pub tx_id: Max52Text,
	#[serde(rename = "ExctgPty")]
	pub exctg_pty: LEIIdentifier,
	#[serde(rename = "InvstmtPtyInd")]
	pub invstmt_pty_ind: bool,
	#[serde(rename = "SubmitgPty")]
	pub submitg_pty: LEIIdentifier,
	#[serde(rename = "Buyr")]
	pub buyr: PartyIdentification79,
	#[serde(rename = "Sellr")]
	pub sellr: PartyIdentification79,
	#[serde(rename = "OrdrTrnsmssn")]
	pub ordr_trnsmssn: SecuritiesTransactionTransmission2,
	#[serde(rename = "Tx")]
	pub tx: SecuritiesTransaction3,
	#[serde(rename = "FinInstrm")]
	pub fin_instrm: FinancialInstrumentAttributes5Choice,
	#[serde(rename = "InvstmtDcsnPrsn", skip_serializing_if = "Option::is_none")]
	pub invstmt_dcsn_prsn: Option<InvestmentParty1Choice>,
	#[serde(rename = "ExctgPrsn")]
	pub exctg_prsn: ExecutingParty1Choice,
	#[serde(rename = "AddtlAttrbts")]
	pub addtl_attrbts: SecuritiesTransactionIndicator2,
	#[serde(rename = "TechAttrbts", skip_serializing_if = "Option::is_none")]
	pub tech_attrbts: Option<RecordTechnicalData5>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// SecuritiesTransactionTransmission2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionTransmission2 {
	#[serde(rename = "TrnsmssnInd")]
	pub trnsmssn_ind: bool,
	#[serde(rename = "TrnsmttgBuyr", skip_serializing_if = "Option::is_none")]
	pub trnsmttg_buyr: Option<LEIIdentifier>,
	#[serde(rename = "TrnsmttgSellr", skip_serializing_if = "Option::is_none")]
	pub trnsmttg_sellr: Option<LEIIdentifier>,
}


// SecurityIdentification19 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification19 {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
	#[serde(rename = "OthrId", skip_serializing_if = "Option::is_none")]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max140Text>,
}


// SecurityInstrumentDescription22 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityInstrumentDescription22 {
	#[serde(rename = "FinInstrmGnlAttrbts")]
	pub fin_instrm_gnl_attrbts: SecurityInstrumentDescription23,
	#[serde(rename = "DebtInstrmAttrbts", skip_serializing_if = "Option::is_none")]
	pub debt_instrm_attrbts: Option<DebtInstrument4>,
	#[serde(rename = "DerivInstrmAttrbts")]
	pub deriv_instrm_attrbts: DerivativeInstrument6,
}


// SecurityInstrumentDescription23 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityInstrumentDescription23 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ISINOct2015Identifier>,
	#[serde(rename = "OthrId", skip_serializing_if = "Option::is_none")]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[serde(rename = "FullNm")]
	pub full_nm: Max350Text,
	#[serde(rename = "ClssfctnTp")]
	pub clssfctn_tp: CFIOct2015Identifier,
	#[serde(rename = "NtnlCcy", skip_serializing_if = "Option::is_none")]
	pub ntnl_ccy: Option<ActiveOrHistoricCurrencyCode>,
}


// Side5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Side5Code {
	#[default]
	#[serde(rename = "SESH")]
	CodeSESH,
	#[serde(rename = "SELL")]
	CodeSELL,
	#[serde(rename = "SSEX")]
	CodeSSEX,
	#[serde(rename = "UNDI")]
	CodeUNDI,
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


// SwapLegIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SwapLegIdentification2 {
	#[serde(rename = "SwpIn", skip_serializing_if = "Option::is_none")]
	pub swp_in: Option<FinancialInstrumentIdentification7Choice>,
	#[serde(rename = "SwpOut", skip_serializing_if = "Option::is_none")]
	pub swp_out: Option<FinancialInstrumentIdentification7Choice>,
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}


// UnderlyingIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingIdentification2Choice {
	#[serde(rename = "Swp", skip_serializing_if = "Option::is_none")]
	pub swp: Option<SwapLegIdentification2>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<FinancialInstrumentIdentification7Choice>,
}


// VariationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum VariationType1Code {
	#[default]
	#[serde(rename = "DECR")]
	CodeDECR,
	#[serde(rename = "INCR")]
	CodeINCR,
}
