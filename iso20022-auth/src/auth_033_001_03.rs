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


// AssetClassSubProductType19Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType19Code {
	#[serde(rename = "DLVR")]
	CodeDLVR,
	#[serde(rename = "NDLV")]
	CodeNDLV,

	#[default]
	UNKOWN
}


// BenchmarkCurveName2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BenchmarkCurveName2Code {
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

	#[default]
	UNKOWN
}


// BenchmarkCurveName5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkCurveName5Choice {
	#[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
	pub indx: Option<BenchmarkCurveName2Code>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max25Text>,
}


// BondDerivative2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BondDerivative2 {
	#[serde(rename = "Issr")]
	pub issr: LEIIdentifier,
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "IssncDt", skip_serializing_if = "Option::is_none")]
	pub issnc_dt: Option<String>,
}


// BondType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BondType1Code {
	#[serde(rename = "EUSB")]
	CodeEUSB,
	#[serde(rename = "OEPB")]
	CodeOEPB,
	#[serde(rename = "CVTB")]
	CodeCVTB,
	#[serde(rename = "CRPB")]
	CodeCRPB,
	#[serde(rename = "CVDB")]
	CodeCVDB,
	#[serde(rename = "OTHR")]
	CodeOTHR,

	#[default]
	UNKOWN
}


// CommodityDerivative2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommodityDerivative2Choice {
	#[serde(rename = "Frght", skip_serializing_if = "Option::is_none")]
	pub frght: Option<CommodityDerivative5>,
	#[serde(rename = "Nrgy", skip_serializing_if = "Option::is_none")]
	pub nrgy: Option<CommodityDerivative6>,
}


// CommodityDerivative4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommodityDerivative4 {
	#[serde(rename = "ClssSpcfc", skip_serializing_if = "Option::is_none")]
	pub clss_spcfc: Option<CommodityDerivative2Choice>,
	#[serde(rename = "NtnlCcy")]
	pub ntnl_ccy: ActiveOrHistoricCurrencyCode,
}


// CommodityDerivative5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommodityDerivative5 {
	#[serde(rename = "Sz")]
	pub sz: Max25Text,
	#[serde(rename = "AvrgTmChrtr")]
	pub avrg_tm_chrtr: Max25Text,
}


// CommodityDerivative6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommodityDerivative6 {
	#[serde(rename = "SttlmLctn")]
	pub sttlm_lctn: Max25Text,
}


// ContractForDifference2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractForDifference2 {
	#[serde(rename = "UndrlygTp")]
	pub undrlyg_tp: UnderlyingContractForDifferenceType3Code,
	#[serde(rename = "NtnlCcy1", skip_serializing_if = "Option::is_none")]
	pub ntnl_ccy1: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "NtnlCcy2", skip_serializing_if = "Option::is_none")]
	pub ntnl_ccy2: Option<ActiveOrHistoricCurrencyCode>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// CountrySubDivisionCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountrySubDivisionCode {
	#[serde(rename = "$value")]
	pub country_sub_division_code: String,
}


// CreditDefaultSwapDerivative5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditDefaultSwapDerivative5 {
	#[serde(rename = "UndrlygCdtDfltSwpId", skip_serializing_if = "Option::is_none")]
	pub undrlyg_cdt_dflt_swp_id: Option<ISINOct2015Identifier>,
	#[serde(rename = "UndrlygCdtDfltSwpIndx")]
	pub undrlyg_cdt_dflt_swp_indx: CreditDefaultSwapIndex3,
}


// CreditDefaultSwapDerivative6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditDefaultSwapDerivative6 {
	#[serde(rename = "UndrlygCdtDfltSwpId", skip_serializing_if = "Option::is_none")]
	pub undrlyg_cdt_dflt_swp_id: Option<ISINOct2015Identifier>,
	#[serde(rename = "OblgtnId")]
	pub oblgtn_id: ISINOct2015Identifier,
	#[serde(rename = "SnglNm")]
	pub sngl_nm: CreditDefaultSwapSingleName2,
}


// CreditDefaultSwapIndex3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditDefaultSwapIndex3 {
	#[serde(rename = "UndrlygIndxId", skip_serializing_if = "Option::is_none")]
	pub undrlyg_indx_id: Option<ISINOct2015Identifier>,
	#[serde(rename = "UndrlygIndxNm", skip_serializing_if = "Option::is_none")]
	pub undrlyg_indx_nm: Option<Max25Text>,
	#[serde(rename = "Srs", skip_serializing_if = "Option::is_none")]
	pub srs: Option<f64>,
	#[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
	pub vrsn: Option<f64>,
	#[serde(rename = "RollMnth", skip_serializing_if = "Option::is_none")]
	pub roll_mnth: Option<Vec<RestrictedMonthExact2Number>>,
	#[serde(rename = "NxtRollDt", skip_serializing_if = "Option::is_none")]
	pub nxt_roll_dt: Option<String>,
	#[serde(rename = "NtnlCcy")]
	pub ntnl_ccy: ActiveOrHistoricCurrencyCode,
}


// CreditDefaultSwapSingleName2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditDefaultSwapSingleName2 {
	#[serde(rename = "SvrgnIssr")]
	pub svrgn_issr: bool,
	#[serde(rename = "RefPty", skip_serializing_if = "Option::is_none")]
	pub ref_pty: Option<DerivativePartyIdentification1Choice>,
	#[serde(rename = "NtnlCcy")]
	pub ntnl_ccy: ActiveOrHistoricCurrencyCode,
}


// CreditDefaultSwapsDerivative4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditDefaultSwapsDerivative4Choice {
	#[serde(rename = "SnglNmCdtDfltSwp", skip_serializing_if = "Option::is_none")]
	pub sngl_nm_cdt_dflt_swp: Option<CreditDefaultSwapSingleName2>,
	#[serde(rename = "CdtDfltSwpIndx", skip_serializing_if = "Option::is_none")]
	pub cdt_dflt_swp_indx: Option<CreditDefaultSwapIndex3>,
	#[serde(rename = "SnglNmCdtDfltSwpDeriv", skip_serializing_if = "Option::is_none")]
	pub sngl_nm_cdt_dflt_swp_deriv: Option<CreditDefaultSwapDerivative6>,
	#[serde(rename = "CdtDfltSwpIndxDeriv", skip_serializing_if = "Option::is_none")]
	pub cdt_dflt_swp_indx_deriv: Option<CreditDefaultSwapDerivative5>,
}


// DebtInstrument5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DebtInstrument5 {
	#[serde(rename = "Tp")]
	pub tp: BondType1Code,
	#[serde(rename = "IssncDt")]
	pub issnc_dt: String,
}


// Derivative3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Derivative3Choice {
	#[serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none")]
	pub cmmdty: Option<CommodityDerivative4>,
	#[serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none")]
	pub intrst_rate: Option<InterestRateDerivative5>,
	#[serde(rename = "FX", skip_serializing_if = "Option::is_none")]
	pub fx: Option<ForeignExchangeDerivative2>,
	#[serde(rename = "Eqty", skip_serializing_if = "Option::is_none")]
	pub eqty: Option<EquityDerivative2>,
	#[serde(rename = "CtrctForDiff", skip_serializing_if = "Option::is_none")]
	pub ctrct_for_diff: Option<ContractForDifference2>,
	#[serde(rename = "Cdt", skip_serializing_if = "Option::is_none")]
	pub cdt: Option<CreditDefaultSwapsDerivative4Choice>,
	#[serde(rename = "EmssnAllwnc", skip_serializing_if = "Option::is_none")]
	pub emssn_allwnc: Option<EmissionAllowanceProductType1Code>,
}


// DerivativePartyIdentification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativePartyIdentification1Choice {
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<CountrySubDivisionCode>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
}


// EmissionAllowanceProductType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EmissionAllowanceProductType1Code {
	#[serde(rename = "EUAA")]
	CodeEUAA,
	#[serde(rename = "EUAE")]
	CodeEUAE,
	#[serde(rename = "ERUE")]
	CodeERUE,
	#[serde(rename = "CERE")]
	CodeCERE,
	#[serde(rename = "OTHR")]
	CodeOTHR,

	#[default]
	UNKOWN
}


// EquityDerivative2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EquityDerivative2 {
	#[serde(rename = "UndrlygTp")]
	pub undrlyg_tp: EquityDerivative3Choice,
	#[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
	pub param: Option<EquityReturnParameter1Code>,
}


// EquityDerivative3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EquityDerivative3Choice {
	#[serde(rename = "Bskt", skip_serializing_if = "Option::is_none")]
	pub bskt: Option<UnderlyingEquityType3Code>,
	#[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
	pub indx: Option<UnderlyingEquityType4Code>,
	#[serde(rename = "SnglNm", skip_serializing_if = "Option::is_none")]
	pub sngl_nm: Option<UnderlyingEquityType5Code>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<UnderlyingEquityType6Code>,
}


// EquityReturnParameter1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EquityReturnParameter1Code {
	#[serde(rename = "PRDV")]
	CodePRDV,
	#[serde(rename = "PRVA")]
	CodePRVA,
	#[serde(rename = "PRVO")]
	CodePRVO,
	#[serde(rename = "PRBP")]
	CodePRBP,

	#[default]
	UNKOWN
}


// ExternalEmissionAllowanceSubProductType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalEmissionAllowanceSubProductType1Code {
	#[serde(rename = "$value")]
	pub external_emission_allowance_sub_product_type1_code: String,
}


// ExternalProductType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalProductType1Code {
	#[serde(rename = "$value")]
	pub external_product_type1_code: String,
}


// FinancialInstrumentContractType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FinancialInstrumentContractType1Code {
	#[serde(rename = "CFDS")]
	CodeCFDS,
	#[serde(rename = "FORW")]
	CodeFORW,
	#[serde(rename = "FRAS")]
	CodeFRAS,
	#[serde(rename = "FUTR")]
	CodeFUTR,
	#[serde(rename = "OPTN")]
	CodeOPTN,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "SPDB")]
	CodeSPDB,
	#[serde(rename = "SWAP")]
	CodeSWAP,
	#[serde(rename = "SWPT")]
	CodeSWPT,
	#[serde(rename = "FONS")]
	CodeFONS,
	#[serde(rename = "PSWP")]
	CodePSWP,
	#[serde(rename = "FFAS")]
	CodeFFAS,
	#[serde(rename = "FWOS")]
	CodeFWOS,

	#[default]
	UNKOWN
}


// FinancialInstrumentReportingNonEquityTransparencyDataReportV03 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentReportingNonEquityTransparencyDataReportV03 {
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: SecuritiesMarketReportHeader1,
	#[serde(rename = "NonEqtyTrnsprncyData")]
	pub non_eqty_trnsprncy_data: Vec<TransparencyDataReport21>,
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


// ForeignExchangeDerivative2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ForeignExchangeDerivative2 {
	#[serde(rename = "CtrctSubTp")]
	pub ctrct_sub_tp: AssetClassSubProductType19Code,
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


// InflationIndex1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InflationIndex1Choice {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max25Text>,
}


// InterestRateContractTerm2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRateContractTerm2 {
	#[serde(rename = "Unit")]
	pub unit: RateBasis1Code,
	#[serde(rename = "Val")]
	pub val: f64,
}


// InterestRateDerivative2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRateDerivative2Choice {
	#[serde(rename = "SwpRltd", skip_serializing_if = "Option::is_none")]
	pub swp_rltd: Option<SwapType1Code>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<UnderlyingInterestRateType3Code>,
}


// InterestRateDerivative5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRateDerivative5 {
	#[serde(rename = "UndrlygTp")]
	pub undrlyg_tp: InterestRateDerivative2Choice,
	#[serde(rename = "UndrlygBd", skip_serializing_if = "Option::is_none")]
	pub undrlyg_bd: Option<BondDerivative2>,
	#[serde(rename = "SwptnNtnlCcy", skip_serializing_if = "Option::is_none")]
	pub swptn_ntnl_ccy: Option<ActiveCurrencyCode>,
	#[serde(rename = "UndrlygSwpMtrtyDt", skip_serializing_if = "Option::is_none")]
	pub undrlyg_swp_mtrty_dt: Option<String>,
	#[serde(rename = "InfltnIndx", skip_serializing_if = "Option::is_none")]
	pub infltn_indx: Option<InflationIndex1Choice>,
	#[serde(rename = "IntrstRateRef")]
	pub intrst_rate_ref: FloatingInterestRate8,
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


// Max25Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max25Text {
	#[serde(rename = "$value")]
	pub max25_text: String,
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


// NonEquityInstrumentReportingClassification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NonEquityInstrumentReportingClassification1Code {
	#[serde(rename = "SFPS")]
	CodeSFPS,
	#[serde(rename = "SDRV")]
	CodeSDRV,
	#[serde(rename = "DERV")]
	CodeDERV,
	#[serde(rename = "EMAL")]
	CodeEMAL,
	#[serde(rename = "BOND")]
	CodeBOND,
	#[serde(rename = "ETCS")]
	CodeETCS,
	#[serde(rename = "ETNS")]
	CodeETNS,

	#[default]
	UNKOWN
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "$value")]
	pub number: f64,
}


// Period2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// Period4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period4Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt", skip_serializing_if = "Option::is_none")]
	pub to_dt: Option<String>,
	#[serde(rename = "FrDtToDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt_to_dt: Option<Period2>,
}


// RateBasis1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RateBasis1Code {
	#[serde(rename = "DAYS")]
	CodeDAYS,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "YEAR")]
	CodeYEAR,

	#[default]
	UNKOWN
}


// RestrictedMonthExact2Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RestrictedMonthExact2Number {
	#[serde(rename = "$value")]
	pub restricted_month_exact2_number: f64,
}


// SecuritiesMarketReportHeader1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesMarketReportHeader1 {
	#[serde(rename = "RptgNtty")]
	pub rptg_ntty: TradingVenueIdentification1Choice,
	#[serde(rename = "RptgPrd")]
	pub rptg_prd: Period4Choice,
	#[serde(rename = "SubmissnDtTm", skip_serializing_if = "Option::is_none")]
	pub submissn_dt_tm: Option<String>,
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


// SwapType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SwapType1Code {
	#[serde(rename = "OSSC")]
	CodeOSSC,
	#[serde(rename = "XFSC")]
	CodeXFSC,
	#[serde(rename = "XFMC")]
	CodeXFMC,
	#[serde(rename = "XXSC")]
	CodeXXSC,
	#[serde(rename = "XXMC")]
	CodeXXMC,
	#[serde(rename = "IFMC")]
	CodeIFMC,
	#[serde(rename = "FFSC")]
	CodeFFSC,
	#[serde(rename = "FFMC")]
	CodeFFMC,
	#[serde(rename = "IFSC")]
	CodeIFSC,
	#[serde(rename = "OSMC")]
	CodeOSMC,

	#[default]
	UNKOWN
}


// TradingVenue2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TradingVenue2Code {
	#[serde(rename = "APPA")]
	CodeAPPA,
	#[serde(rename = "CTPS")]
	CodeCTPS,

	#[default]
	UNKOWN
}


// TradingVenueIdentification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingVenueIdentification1Choice {
	#[serde(rename = "MktIdCd", skip_serializing_if = "Option::is_none")]
	pub mkt_id_cd: Option<MICIdentifier>,
	#[serde(rename = "NtlCmptntAuthrty", skip_serializing_if = "Option::is_none")]
	pub ntl_cmptnt_authrty: Option<CountryCode>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<TradingVenueIdentification2>,
}


// TradingVenueIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingVenueIdentification2 {
	#[serde(rename = "Id")]
	pub id: Max50Text,
	#[serde(rename = "Tp")]
	pub tp: TradingVenue2Code,
}


// TransparencyDataReport21 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransparencyDataReport21 {
	#[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
	pub tech_rcrd_id: Option<Max35Text>,
	#[serde(rename = "Id")]
	pub id: ISINOct2015Identifier,
	#[serde(rename = "FullNm", skip_serializing_if = "Option::is_none")]
	pub full_nm: Option<Max350Text>,
	#[serde(rename = "TradgVn", skip_serializing_if = "Option::is_none")]
	pub tradg_vn: Option<MICIdentifier>,
	#[serde(rename = "RptgDt", skip_serializing_if = "Option::is_none")]
	pub rptg_dt: Option<String>,
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "FinInstrmClssfctn")]
	pub fin_instrm_clssfctn: NonEquityInstrumentReportingClassification1Code,
	#[serde(rename = "UndrlygInstrmAsstClss", skip_serializing_if = "Option::is_none")]
	pub undrlyg_instrm_asst_clss: Option<ExternalProductType1Code>,
	#[serde(rename = "DerivCtrctTp", skip_serializing_if = "Option::is_none")]
	pub deriv_ctrct_tp: Option<FinancialInstrumentContractType1Code>,
	#[serde(rename = "Bd", skip_serializing_if = "Option::is_none")]
	pub bd: Option<DebtInstrument5>,
	#[serde(rename = "EmssnAllwncTp", skip_serializing_if = "Option::is_none")]
	pub emssn_allwnc_tp: Option<ExternalEmissionAllowanceSubProductType1Code>,
	#[serde(rename = "Deriv", skip_serializing_if = "Option::is_none")]
	pub deriv: Option<Derivative3Choice>,
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}


// UnderlyingContractForDifferenceType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum UnderlyingContractForDifferenceType3Code {
	#[serde(rename = "BOND")]
	CodeBOND,
	#[serde(rename = "COMM")]
	CodeCOMM,
	#[serde(rename = "CURR")]
	CodeCURR,
	#[serde(rename = "EMAL")]
	CodeEMAL,
	#[serde(rename = "EQUI")]
	CodeEQUI,
	#[serde(rename = "FTEQ")]
	CodeFTEQ,
	#[serde(rename = "OPEQ")]
	CodeOPEQ,
	#[serde(rename = "OTHR")]
	CodeOTHR,

	#[default]
	UNKOWN
}


// UnderlyingEquityType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum UnderlyingEquityType3Code {
	#[serde(rename = "BSKT")]
	CodeBSKT,

	#[default]
	UNKOWN
}


// UnderlyingEquityType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum UnderlyingEquityType4Code {
	#[serde(rename = "STIX")]
	CodeSTIX,
	#[serde(rename = "DIVI")]
	CodeDIVI,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "VOLI")]
	CodeVOLI,

	#[default]
	UNKOWN
}


// UnderlyingEquityType5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum UnderlyingEquityType5Code {
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "ETFS")]
	CodeETFS,
	#[serde(rename = "SHRS")]
	CodeSHRS,
	#[serde(rename = "DVSE")]
	CodeDVSE,

	#[default]
	UNKOWN
}


// UnderlyingEquityType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum UnderlyingEquityType6Code {
	#[serde(rename = "BSKT")]
	CodeBSKT,
	#[serde(rename = "DIVI")]
	CodeDIVI,
	#[serde(rename = "ETFS")]
	CodeETFS,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "SHRS")]
	CodeSHRS,
	#[serde(rename = "DVSE")]
	CodeDVSE,
	#[serde(rename = "STIX")]
	CodeSTIX,
	#[serde(rename = "VOLI")]
	CodeVOLI,

	#[default]
	UNKOWN
}


// UnderlyingInterestRateType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum UnderlyingInterestRateType3Code {
	#[serde(rename = "BOND")]
	CodeBOND,
	#[serde(rename = "BNDF")]
	CodeBNDF,
	#[serde(rename = "INTR")]
	CodeINTR,
	#[serde(rename = "IFUT")]
	CodeIFUT,

	#[default]
	UNKOWN
}
