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
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "ActiveOrHistoricCurrencyCode")]
	pub active_or_historic_currency_code: String,
}


// AssetClassSubProductType19Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType19Code {
	#[serde(rename = "AssetClassSubProductType19Code")]
	pub asset_class_sub_product_type19_code: String,
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


// BondDerivative2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BondDerivative2 {
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "IssncDt")]
	pub issnc_dt: Option<String>,
}


// BondType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BondType1Code {
	#[serde(rename = "BondType1Code")]
	pub bond_type1_code: String,
}


// CommodityDerivative2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommodityDerivative2Choice {
	#[serde(rename = "Frght")]
	pub frght: Option<CommodityDerivative5>,
	#[serde(rename = "Nrgy")]
	pub nrgy: Option<CommodityDerivative6>,
}


// CommodityDerivative4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommodityDerivative4 {
	#[serde(rename = "ClssSpcfc")]
	pub clss_spcfc: Option<CommodityDerivative2Choice>,
	#[serde(rename = "NtnlCcy")]
	pub ntnl_ccy: String,
}


// CommodityDerivative5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommodityDerivative5 {
	#[serde(rename = "Sz")]
	pub sz: String,
	#[serde(rename = "AvrgTmChrtr")]
	pub avrg_tm_chrtr: String,
}


// CommodityDerivative6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommodityDerivative6 {
	#[serde(rename = "SttlmLctn")]
	pub sttlm_lctn: String,
}


// ContractForDifference2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractForDifference2 {
	#[serde(rename = "UndrlygTp")]
	pub undrlyg_tp: String,
	#[serde(rename = "NtnlCcy1")]
	pub ntnl_ccy1: Option<String>,
	#[serde(rename = "NtnlCcy2")]
	pub ntnl_ccy2: Option<String>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// CountrySubDivisionCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountrySubDivisionCode {
	#[serde(rename = "CountrySubDivisionCode")]
	pub country_sub_division_code: String,
}


// CreditDefaultSwapDerivative5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditDefaultSwapDerivative5 {
	#[serde(rename = "UndrlygCdtDfltSwpId")]
	pub undrlyg_cdt_dflt_swp_id: Option<String>,
	#[serde(rename = "UndrlygCdtDfltSwpIndx")]
	pub undrlyg_cdt_dflt_swp_indx: CreditDefaultSwapIndex3,
}


// CreditDefaultSwapDerivative6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditDefaultSwapDerivative6 {
	#[serde(rename = "UndrlygCdtDfltSwpId")]
	pub undrlyg_cdt_dflt_swp_id: Option<String>,
	#[serde(rename = "OblgtnId")]
	pub oblgtn_id: String,
	#[serde(rename = "SnglNm")]
	pub sngl_nm: CreditDefaultSwapSingleName2,
}


// CreditDefaultSwapIndex3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditDefaultSwapIndex3 {
	#[serde(rename = "UndrlygIndxId")]
	pub undrlyg_indx_id: Option<String>,
	#[serde(rename = "UndrlygIndxNm")]
	pub undrlyg_indx_nm: Option<String>,
	#[serde(rename = "Srs")]
	pub srs: Option<f64>,
	#[serde(rename = "Vrsn")]
	pub vrsn: Option<f64>,
	#[serde(rename = "RollMnth")]
	pub roll_mnth: Option<Vec<f64>>,
	#[serde(rename = "NxtRollDt")]
	pub nxt_roll_dt: Option<String>,
	#[serde(rename = "NtnlCcy")]
	pub ntnl_ccy: String,
}


// CreditDefaultSwapSingleName2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditDefaultSwapSingleName2 {
	#[serde(rename = "SvrgnIssr")]
	pub svrgn_issr: bool,
	#[serde(rename = "RefPty")]
	pub ref_pty: Option<DerivativePartyIdentification1Choice>,
	#[serde(rename = "NtnlCcy")]
	pub ntnl_ccy: String,
}


// CreditDefaultSwapsDerivative4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditDefaultSwapsDerivative4Choice {
	#[serde(rename = "SnglNmCdtDfltSwp")]
	pub sngl_nm_cdt_dflt_swp: Option<CreditDefaultSwapSingleName2>,
	#[serde(rename = "CdtDfltSwpIndx")]
	pub cdt_dflt_swp_indx: Option<CreditDefaultSwapIndex3>,
	#[serde(rename = "SnglNmCdtDfltSwpDeriv")]
	pub sngl_nm_cdt_dflt_swp_deriv: Option<CreditDefaultSwapDerivative6>,
	#[serde(rename = "CdtDfltSwpIndxDeriv")]
	pub cdt_dflt_swp_indx_deriv: Option<CreditDefaultSwapDerivative5>,
}


// DebtInstrument5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DebtInstrument5 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "IssncDt")]
	pub issnc_dt: String,
}


// Derivative3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Derivative3Choice {
	#[serde(rename = "Cmmdty")]
	pub cmmdty: Option<CommodityDerivative4>,
	#[serde(rename = "IntrstRate")]
	pub intrst_rate: Option<InterestRateDerivative5>,
	#[serde(rename = "FX")]
	pub fx: Option<ForeignExchangeDerivative2>,
	#[serde(rename = "Eqty")]
	pub eqty: Option<EquityDerivative2>,
	#[serde(rename = "CtrctForDiff")]
	pub ctrct_for_diff: Option<ContractForDifference2>,
	#[serde(rename = "Cdt")]
	pub cdt: Option<CreditDefaultSwapsDerivative4Choice>,
	#[serde(rename = "EmssnAllwnc")]
	pub emssn_allwnc: Option<String>,
}


// DerivativePartyIdentification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativePartyIdentification1Choice {
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
	#[serde(rename = "CtrySubDvsn")]
	pub ctry_sub_dvsn: Option<String>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
}


// EmissionAllowanceProductType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EmissionAllowanceProductType1Code {
	#[serde(rename = "EmissionAllowanceProductType1Code")]
	pub emission_allowance_product_type1_code: String,
}


// EquityDerivative2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EquityDerivative2 {
	#[serde(rename = "UndrlygTp")]
	pub undrlyg_tp: EquityDerivative3Choice,
	#[serde(rename = "Param")]
	pub param: Option<String>,
}


// EquityDerivative3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EquityDerivative3Choice {
	#[serde(rename = "Bskt")]
	pub bskt: Option<String>,
	#[serde(rename = "Indx")]
	pub indx: Option<String>,
	#[serde(rename = "SnglNm")]
	pub sngl_nm: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<String>,
}


// EquityReturnParameter1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EquityReturnParameter1Code {
	#[serde(rename = "EquityReturnParameter1Code")]
	pub equity_return_parameter1_code: String,
}


// ExternalEmissionAllowanceSubProductType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalEmissionAllowanceSubProductType1Code {
	#[serde(rename = "ExternalEmissionAllowanceSubProductType1Code")]
	pub external_emission_allowance_sub_product_type1_code: String,
}


// ExternalProductType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalProductType1Code {
	#[serde(rename = "ExternalProductType1Code")]
	pub external_product_type1_code: String,
}


// FinancialInstrumentContractType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentContractType1Code {
	#[serde(rename = "FinancialInstrumentContractType1Code")]
	pub financial_instrument_contract_type1_code: String,
}


// FinancialInstrumentReportingNonEquityTransparencyDataReportV03 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentReportingNonEquityTransparencyDataReportV03 {
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: SecuritiesMarketReportHeader1,
	#[serde(rename = "NonEqtyTrnsprncyData")]
	pub non_eqty_trnsprncy_data: Vec<TransparencyDataReport21>,
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


// ForeignExchangeDerivative2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ForeignExchangeDerivative2 {
	#[serde(rename = "CtrctSubTp")]
	pub ctrct_sub_tp: String,
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


// InflationIndex1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InflationIndex1Choice {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
}


// InterestRateContractTerm2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRateContractTerm2 {
	#[serde(rename = "Unit")]
	pub unit: String,
	#[serde(rename = "Val")]
	pub val: f64,
}


// InterestRateDerivative2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRateDerivative2Choice {
	#[serde(rename = "SwpRltd")]
	pub swp_rltd: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<String>,
}


// InterestRateDerivative5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRateDerivative5 {
	#[serde(rename = "UndrlygTp")]
	pub undrlyg_tp: InterestRateDerivative2Choice,
	#[serde(rename = "UndrlygBd")]
	pub undrlyg_bd: Option<BondDerivative2>,
	#[serde(rename = "SwptnNtnlCcy")]
	pub swptn_ntnl_ccy: Option<String>,
	#[serde(rename = "UndrlygSwpMtrtyDt")]
	pub undrlyg_swp_mtrty_dt: Option<String>,
	#[serde(rename = "InfltnIndx")]
	pub infltn_indx: Option<InflationIndex1Choice>,
	#[serde(rename = "IntrstRateRef")]
	pub intrst_rate_ref: FloatingInterestRate8,
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


// Max25Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max25Text {
	#[serde(rename = "Max25Text")]
	pub max25_text: String,
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


// NonEquityInstrumentReportingClassification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NonEquityInstrumentReportingClassification1Code {
	#[serde(rename = "NonEquityInstrumentReportingClassification1Code")]
	pub non_equity_instrument_reporting_classification1_code: String,
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
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
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "FrDt")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt")]
	pub to_dt: Option<String>,
	#[serde(rename = "FrDtToDt")]
	pub fr_dt_to_dt: Option<Period2>,
}


// RateBasis1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RateBasis1Code {
	#[serde(rename = "RateBasis1Code")]
	pub rate_basis1_code: String,
}


// RestrictedMonthExact2Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RestrictedMonthExact2Number {
	// #[serde(rename = "RestrictedMonthExact2Number")]
	pub restricted_month_exact2_number: f64,
}


// SecuritiesMarketReportHeader1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesMarketReportHeader1 {
	#[serde(rename = "RptgNtty")]
	pub rptg_ntty: TradingVenueIdentification1Choice,
	#[serde(rename = "RptgPrd")]
	pub rptg_prd: Period4Choice,
	#[serde(rename = "SubmissnDtTm")]
	pub submissn_dt_tm: Option<String>,
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


// SwapType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SwapType1Code {
	#[serde(rename = "SwapType1Code")]
	pub swap_type1_code: String,
}


// TradingVenue2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingVenue2Code {
	#[serde(rename = "TradingVenue2Code")]
	pub trading_venue2_code: String,
}


// TradingVenueIdentification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingVenueIdentification1Choice {
	#[serde(rename = "MktIdCd")]
	pub mkt_id_cd: Option<String>,
	#[serde(rename = "NtlCmptntAuthrty")]
	pub ntl_cmptnt_authrty: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<TradingVenueIdentification2>,
}


// TradingVenueIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingVenueIdentification2 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Tp")]
	pub tp: String,
}


// TransparencyDataReport21 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransparencyDataReport21 {
	#[serde(rename = "TechRcrdId")]
	pub tech_rcrd_id: Option<String>,
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "FullNm")]
	pub full_nm: Option<String>,
	#[serde(rename = "TradgVn")]
	pub tradg_vn: Option<String>,
	#[serde(rename = "RptgDt")]
	pub rptg_dt: Option<String>,
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "FinInstrmClssfctn")]
	pub fin_instrm_clssfctn: String,
	#[serde(rename = "UndrlygInstrmAsstClss")]
	pub undrlyg_instrm_asst_clss: Option<String>,
	#[serde(rename = "DerivCtrctTp")]
	pub deriv_ctrct_tp: Option<String>,
	#[serde(rename = "Bd")]
	pub bd: Option<DebtInstrument5>,
	#[serde(rename = "EmssnAllwncTp")]
	pub emssn_allwnc_tp: Option<String>,
	#[serde(rename = "Deriv")]
	pub deriv: Option<Derivative3Choice>,
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}


// UnderlyingContractForDifferenceType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingContractForDifferenceType3Code {
	#[serde(rename = "UnderlyingContractForDifferenceType3Code")]
	pub underlying_contract_for_difference_type3_code: String,
}


// UnderlyingEquityType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingEquityType3Code {
	#[serde(rename = "UnderlyingEquityType3Code")]
	pub underlying_equity_type3_code: String,
}


// UnderlyingEquityType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingEquityType4Code {
	#[serde(rename = "UnderlyingEquityType4Code")]
	pub underlying_equity_type4_code: String,
}


// UnderlyingEquityType5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingEquityType5Code {
	#[serde(rename = "UnderlyingEquityType5Code")]
	pub underlying_equity_type5_code: String,
}


// UnderlyingEquityType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingEquityType6Code {
	#[serde(rename = "UnderlyingEquityType6Code")]
	pub underlying_equity_type6_code: String,
}


// UnderlyingInterestRateType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingInterestRateType3Code {
	#[serde(rename = "UnderlyingInterestRateType3Code")]
	pub underlying_interest_rate_type3_code: String,
}
