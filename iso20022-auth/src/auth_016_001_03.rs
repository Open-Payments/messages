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
	#[serde(rename = "ActiveCurrencyAnd13DecimalAmount_SimpleType")]
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


// AmountAndDirection53 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection53 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "Sgn")]
	pub sgn: Option<bool>,
}


// AmountAndDirection61 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection61 {
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAnd13DecimalAmount,
	#[serde(rename = "Sgn")]
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
	#[serde(rename = "Intrst")]
	pub intrst: Option<DerivativeInterest2>,
	#[serde(rename = "FX")]
	pub fx: Option<DerivativeForeignExchange2>,
	#[serde(rename = "Both")]
	pub both: Option<AssetClassAttributes1>,
}


// BasketDescription3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BasketDescription3 {
	#[serde(rename = "ISIN")]
	pub isin: Option<Vec<String>>,
	#[serde(rename = "Indx")]
	pub indx: Option<Vec<FinancialInstrument58>>,
}


// BenchmarkCurveName2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkCurveName2Code {
	#[serde(rename = "BenchmarkCurveName2Code")]
	pub benchmark_curve_name2_code: String,
}


// BenchmarkCurveName5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkCurveName5Choice {
	#[serde(rename = "Indx")]
	pub indx: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
}


// CFIOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[serde(rename = "CFIOct2015Identifier")]
	pub cfi_oct2015_identifier: String,
}


// CancelledStatusReason15Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CancelledStatusReason15Code {
	#[serde(rename = "CancelledStatusReason15Code")]
	pub cancelled_status_reason15_code: String,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// DTI2021Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DTI2021Identifier {
	#[serde(rename = "DTI2021Identifier")]
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
	#[serde(rename = "DecimalNumber")]
	pub decimal_number: f64,
}


// DerivativeForeignExchange2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativeForeignExchange2 {
	#[serde(rename = "OthrNtnlCcy")]
	pub othr_ntnl_ccy: String,
}


// DerivativeInstrument6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativeInstrument6 {
	#[serde(rename = "XpryDt")]
	pub xpry_dt: Option<String>,
	#[serde(rename = "PricMltplr")]
	pub pric_mltplr: f64,
	#[serde(rename = "UndrlygInstrm")]
	pub undrlyg_instrm: UnderlyingIdentification2Choice,
	#[serde(rename = "OptnTp")]
	pub optn_tp: Option<String>,
	#[serde(rename = "StrkPric")]
	pub strk_pric: Option<SecuritiesTransactionPrice4Choice>,
	#[serde(rename = "OptnExrcStyle")]
	pub optn_exrc_style: Option<String>,
	#[serde(rename = "DlvryTp")]
	pub dlvry_tp: String,
	#[serde(rename = "AsstClssSpcfcAttrbts")]
	pub asst_clss_spcfc_attrbts: Option<AssetClassAttributes1Choice>,
}


// DerivativeInterest2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativeInterest2 {
	#[serde(rename = "OthrNtnlCcy")]
	pub othr_ntnl_ccy: String,
}


// DigitalTokenAmount2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DigitalTokenAmount2 {
	#[serde(rename = "Idr")]
	pub idr: String,
	#[serde(rename = "Unit")]
	pub unit: Option<f64>,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// ExecutingParty1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExecutingParty1Choice {
	#[serde(rename = "Prsn")]
	pub prsn: Option<PersonIdentification12>,
	#[serde(rename = "Algo")]
	pub algo: Option<String>,
	#[serde(rename = "Clnt")]
	pub clnt: Option<String>,
}


// ExternalAuthorityExchangeReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAuthorityExchangeReason1Code {
	#[serde(rename = "ExternalAuthorityExchangeReason1Code")]
	pub external_authority_exchange_reason1_code: String,
}


// ExternalFinancialInstrumentIdentificationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalFinancialInstrumentIdentificationType1Code {
	#[serde(rename = "ExternalFinancialInstrumentIdentificationType1Code")]
	pub external_financial_instrument_identification_type1_code: String,
}


// ExternalPersonIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPersonIdentification1Code {
	#[serde(rename = "ExternalPersonIdentification1Code")]
	pub external_person_identification1_code: String,
}


// FinancialInstrument58 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument58 {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: FloatingInterestRate8,
}


// FinancialInstrumentAttributes5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentAttributes5Choice {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "AltrnId")]
	pub altrn_id: Option<SecurityIdentification19>,
	#[serde(rename = "Othr")]
	pub othr: Option<SecurityInstrumentDescription22>,
}


// FinancialInstrumentIdentification6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentIdentification6Choice {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[serde(rename = "Indx")]
	pub indx: Option<FinancialInstrument58>,
}


// FinancialInstrumentIdentification7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentIdentification7Choice {
	#[serde(rename = "Sngl")]
	pub sngl: Option<FinancialInstrumentIdentification6Choice>,
	#[serde(rename = "Bskt")]
	pub bskt: Option<BasketDescription3>,
}


// FinancialInstrumentQuantity25Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentQuantity25Choice {
	#[serde(rename = "Unit")]
	pub unit: Option<f64>,
	#[serde(rename = "NmnlVal")]
	pub nmnl_val: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "MntryVal")]
	pub mntry_val: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// FinancialInstrumentReportingTransactionReportV03 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentReportingTransactionReportV03 {
	#[serde(rename = "Tx")]
	pub tx: Vec<ReportingTransactionType3Choice>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// FloatingInterestRate8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FloatingInterestRate8 {
	#[serde(rename = "RefRate")]
	pub ref_rate: BenchmarkCurveName5Choice,
	#[serde(rename = "Term")]
	pub term: Option<InterestRateContractTerm2>,
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


// ISINOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "ISINOct2015Identifier")]
	pub isin_oct2015_identifier: String,
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


// IdentificationSource3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IdentificationSource3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ImpliedCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImpliedCurrencyAndAmount {
	#[serde(rename = "ImpliedCurrencyAndAmount")]
	pub implied_currency_and_amount: f64,
}


// InterestRateContractTerm2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRateContractTerm2 {
	#[serde(rename = "Unit")]
	pub unit: String,
	#[serde(rename = "Val")]
	pub val: f64,
}


// InternalPartyRole1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InternalPartyRole1Code {
	#[serde(rename = "InternalPartyRole1Code")]
	pub internal_party_role1_code: String,
}


// InvestmentParty1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentParty1Choice {
	#[serde(rename = "Prsn")]
	pub prsn: Option<PersonIdentification12>,
	#[serde(rename = "Algo")]
	pub algo: Option<String>,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// MICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MICIdentifier {
	#[serde(rename = "MICIdentifier")]
	pub mic_identifier: String,
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


// Max25Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max25Text {
	#[serde(rename = "Max25Text")]
	pub max25_text: String,
}


// Max30DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max30DecimalNumber {
	#[serde(rename = "Max30DecimalNumber")]
	pub max30_decimal_number: f64,
}


// Max30Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max30Text {
	#[serde(rename = "Max30Text")]
	pub max30_text: String,
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


// Max50Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max50Text {
	#[serde(rename = "Max50Text")]
	pub max50_text: String,
}


// Max52Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max52Text {
	#[serde(rename = "Max52Text")]
	pub max52_text: String,
}


// NoReasonCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NoReasonCode {
	#[serde(rename = "NoReasonCode")]
	pub no_reason_code: String,
}


// NonNegativeDecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NonNegativeDecimalNumber {
	#[serde(rename = "NonNegativeDecimalNumber")]
	pub non_negative_decimal_number: f64,
}


// OptionStyle7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OptionStyle7Code {
	#[serde(rename = "OptionStyle7Code")]
	pub option_style7_code: String,
}


// OptionType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OptionType2Code {
	#[serde(rename = "OptionType2Code")]
	pub option_type2_code: String,
}


// OtherIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Sfx")]
	pub sfx: Option<String>,
	#[serde(rename = "Tp")]
	pub tp: IdentificationSource3Choice,
}


// PartyIdentification76 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification76 {
	#[serde(rename = "Id")]
	pub id: PersonOrOrganisation1Choice,
	#[serde(rename = "CtryOfBrnch")]
	pub ctry_of_brnch: Option<String>,
}


// PartyIdentification79 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification79 {
	#[serde(rename = "AcctOwnr")]
	pub acct_ownr: Vec<PartyIdentification76>,
	#[serde(rename = "DcsnMakr")]
	pub dcsn_makr: Option<Vec<PersonOrOrganisation2Choice>>,
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// PersonIdentification10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentification10 {
	#[serde(rename = "FrstNm")]
	pub frst_nm: String,
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "BirthDt")]
	pub birth_dt: String,
	#[serde(rename = "Othr")]
	pub othr: GenericPersonIdentification1,
}


// PersonIdentification12 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentification12 {
	#[serde(rename = "CtryOfBrnch")]
	pub ctry_of_brnch: String,
	#[serde(rename = "Othr")]
	pub othr: GenericPersonIdentification1,
}


// PersonIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// PersonOrOrganisation1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonOrOrganisation1Choice {
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "MIC")]
	pub mic: Option<String>,
	#[serde(rename = "Prsn")]
	pub prsn: Option<PersonIdentification10>,
	#[serde(rename = "Intl")]
	pub intl: Option<String>,
}


// PersonOrOrganisation2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonOrOrganisation2Choice {
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "Prsn")]
	pub prsn: Option<PersonIdentification10>,
}


// PhysicalTransferType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PhysicalTransferType4Code {
	#[serde(rename = "PhysicalTransferType4Code")]
	pub physical_transfer_type4_code: String,
}


// PlusOrMinusIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "PlusOrMinusIndicator")]
	pub plus_or_minus_indicator: bool,
}


// PriceStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceStatus1Code {
	#[serde(rename = "PriceStatus1Code")]
	pub price_status1_code: String,
}


// RateBasis1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RateBasis1Code {
	#[serde(rename = "RateBasis1Code")]
	pub rate_basis1_code: String,
}


// RecordTechnicalData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RecordTechnicalData2 {
	#[serde(rename = "RctDtTm")]
	pub rct_dt_tm: String,
	#[serde(rename = "CxlRsn")]
	pub cxl_rsn: String,
}


// RecordTechnicalData5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RecordTechnicalData5 {
	#[serde(rename = "RctDtTm")]
	pub rct_dt_tm: String,
	#[serde(rename = "XchgRsn")]
	pub xchg_rsn: Vec<String>,
}


// RegulatoryTradingCapacity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegulatoryTradingCapacity1Code {
	#[serde(rename = "RegulatoryTradingCapacity1Code")]
	pub regulatory_trading_capacity1_code: String,
}


// ReportingTransactionType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportingTransactionType3Choice {
	#[serde(rename = "New")]
	pub new: Option<SecuritiesTransactionReport7>,
	#[serde(rename = "Cxl")]
	pub cxl: Option<SecuritiesTransactionReport2>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// ReportingWaiverType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportingWaiverType1Code {
	#[serde(rename = "ReportingWaiverType1Code")]
	pub reporting_waiver_type1_code: String,
}


// ReportingWaiverType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportingWaiverType3Code {
	#[serde(rename = "ReportingWaiverType3Code")]
	pub reporting_waiver_type3_code: String,
}


// SecuritiesTransaction3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransaction3 {
	#[serde(rename = "TradDt")]
	pub trad_dt: String,
	#[serde(rename = "TradgCpcty")]
	pub tradg_cpcty: String,
	#[serde(rename = "Qty")]
	pub qty: FinancialInstrumentQuantity25Choice,
	#[serde(rename = "DgtlTknQty")]
	pub dgtl_tkn_qty: Option<Vec<DigitalTokenAmount2>>,
	#[serde(rename = "DerivNtnlChng")]
	pub deriv_ntnl_chng: Option<String>,
	#[serde(rename = "Pric")]
	pub pric: SecuritiesTransactionPrice22Choice,
	#[serde(rename = "NetAmt")]
	pub net_amt: Option<f64>,
	#[serde(rename = "TradVn")]
	pub trad_vn: String,
	#[serde(rename = "CtryOfBrnch")]
	pub ctry_of_brnch: Option<String>,
	#[serde(rename = "UpFrntPmt")]
	pub up_frnt_pmt: Option<AmountAndDirection53>,
	#[serde(rename = "TradPlcMtchgId")]
	pub trad_plc_mtchg_id: Option<String>,
	#[serde(rename = "CmplxTradCmpntId")]
	pub cmplx_trad_cmpnt_id: Option<String>,
}


// SecuritiesTransactionIndicator2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionIndicator2 {
	#[serde(rename = "WvrInd")]
	pub wvr_ind: Option<Vec<String>>,
	#[serde(rename = "ShrtSellgInd")]
	pub shrt_sellg_ind: Option<String>,
	#[serde(rename = "OTCPstTradInd")]
	pub otc_pst_trad_ind: Option<Vec<String>>,
	#[serde(rename = "RskRdcgTx")]
	pub rsk_rdcg_tx: Option<bool>,
	#[serde(rename = "SctiesFincgTxInd")]
	pub scties_fincg_tx_ind: bool,
}


// SecuritiesTransactionPrice1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice1 {
	#[serde(rename = "Pdg")]
	pub pdg: String,
	#[serde(rename = "Ccy")]
	pub ccy: Option<String>,
}


// SecuritiesTransactionPrice22Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice22Choice {
	#[serde(rename = "Pric")]
	pub pric: Option<SecuritiesTransactionPrice2Choice>,
	#[serde(rename = "DgtlTknPric")]
	pub dgtl_tkn_pric: Option<SecuritiesTransactionPrice7>,
	#[serde(rename = "NoPric")]
	pub no_pric: Option<SecuritiesTransactionPrice6>,
}


// SecuritiesTransactionPrice2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice2Choice {
	#[serde(rename = "MntryVal")]
	pub mntry_val: Option<AmountAndDirection61>,
	#[serde(rename = "Pctg")]
	pub pctg: Option<f64>,
	#[serde(rename = "Yld")]
	pub yld: Option<f64>,
	#[serde(rename = "BsisPts")]
	pub bsis_pts: Option<f64>,
}


// SecuritiesTransactionPrice4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice4Choice {
	#[serde(rename = "Pric")]
	pub pric: Option<SecuritiesTransactionPrice2Choice>,
	#[serde(rename = "NoPric")]
	pub no_pric: Option<SecuritiesTransactionPrice1>,
}


// SecuritiesTransactionPrice6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice6 {
	#[serde(rename = "Pdg")]
	pub pdg: String,
	#[serde(rename = "Ccy")]
	pub ccy: Option<String>,
	#[serde(rename = "DgtlTkn")]
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
	pub tx_id: String,
	#[serde(rename = "ExctgPty")]
	pub exctg_pty: String,
	#[serde(rename = "SubmitgPty")]
	pub submitg_pty: String,
	#[serde(rename = "TechAttrbts")]
	pub tech_attrbts: Option<RecordTechnicalData2>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// SecuritiesTransactionReport7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionReport7 {
	#[serde(rename = "TxId")]
	pub tx_id: String,
	#[serde(rename = "ExctgPty")]
	pub exctg_pty: String,
	#[serde(rename = "InvstmtPtyInd")]
	pub invstmt_pty_ind: bool,
	#[serde(rename = "SubmitgPty")]
	pub submitg_pty: String,
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
	#[serde(rename = "InvstmtDcsnPrsn")]
	pub invstmt_dcsn_prsn: Option<InvestmentParty1Choice>,
	#[serde(rename = "ExctgPrsn")]
	pub exctg_prsn: ExecutingParty1Choice,
	#[serde(rename = "AddtlAttrbts")]
	pub addtl_attrbts: SecuritiesTransactionIndicator2,
	#[serde(rename = "TechAttrbts")]
	pub tech_attrbts: Option<RecordTechnicalData5>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// SecuritiesTransactionTransmission2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionTransmission2 {
	#[serde(rename = "TrnsmssnInd")]
	pub trnsmssn_ind: bool,
	#[serde(rename = "TrnsmttgBuyr")]
	pub trnsmttg_buyr: Option<String>,
	#[serde(rename = "TrnsmttgSellr")]
	pub trnsmttg_sellr: Option<String>,
}


// SecurityIdentification19 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification19 {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[serde(rename = "OthrId")]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// SecurityInstrumentDescription22 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityInstrumentDescription22 {
	#[serde(rename = "FinInstrmGnlAttrbts")]
	pub fin_instrm_gnl_attrbts: SecurityInstrumentDescription23,
	#[serde(rename = "DebtInstrmAttrbts")]
	pub debt_instrm_attrbts: Option<DebtInstrument4>,
	#[serde(rename = "DerivInstrmAttrbts")]
	pub deriv_instrm_attrbts: DerivativeInstrument6,
}


// SecurityInstrumentDescription23 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityInstrumentDescription23 {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "OthrId")]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[serde(rename = "FullNm")]
	pub full_nm: String,
	#[serde(rename = "ClssfctnTp")]
	pub clssfctn_tp: String,
	#[serde(rename = "NtnlCcy")]
	pub ntnl_ccy: Option<String>,
}


// Side5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Side5Code {
	#[serde(rename = "Side5Code")]
	pub side5_code: String,
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


// SwapLegIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SwapLegIdentification2 {
	#[serde(rename = "SwpIn")]
	pub swp_in: Option<FinancialInstrumentIdentification7Choice>,
	#[serde(rename = "SwpOut")]
	pub swp_out: Option<FinancialInstrumentIdentification7Choice>,
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}


// UnderlyingIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingIdentification2Choice {
	#[serde(rename = "Swp")]
	pub swp: Option<SwapLegIdentification2>,
	#[serde(rename = "Othr")]
	pub othr: Option<FinancialInstrumentIdentification7Choice>,
}


// VariationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct VariationType1Code {
	#[serde(rename = "VariationType1Code")]
	pub variation_type1_code: String,
}
