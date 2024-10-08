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


// AbnormalValuesData4 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AbnormalValuesData4 {
	#[serde(rename = "CtrPtyId")]
	pub ctr_pty_id: CounterpartyData92,
	#[serde(rename = "NbOfDerivsRptd")]
	pub nb_of_derivs_rptd: f64,
	#[serde(rename = "NbOfDerivsRptdWthOtlrs")]
	pub nb_of_derivs_rptd_wth_otlrs: f64,
	#[serde(rename = "TxDtls")]
	pub tx_dtls: Option<Vec<AbnormalValuesTransactionData2>>,
}


// AbnormalValuesTransactionData2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AbnormalValuesTransactionData2 {
	#[serde(rename = "TxId")]
	pub tx_id: TradeTransactionIdentification24,
	#[serde(rename = "NtnlAmt")]
	pub ntnl_amt: Option<NotionalAmountLegs5>,
	#[serde(rename = "NtnlQty")]
	pub ntnl_qty: Option<NotionalQuantityLegs5>,
}


// ActiveCurrencyCode ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// ActiveOrHistoricCurrencyAnd19DecimalAmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd19DecimalAmountSimpleType {
	#[serde(rename = "ActiveOrHistoricCurrencyAnd19DecimalAmount_SimpleType")]
	pub active_or_historic_currency_and19_decimal_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAnd19DecimalAmount ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd19DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "ActiveOrHistoricCurrencyCode")]
	pub active_or_historic_currency_code: String,
}


// AgreementType2Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgreementType2Choice {
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// AmountAndDirection106 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection106 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAnd19DecimalAmount,
	#[serde(rename = "Sgn")]
	pub sgn: Option<bool>,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// CollateralPortfolioCode5Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralPortfolioCode5Choice {
	#[serde(rename = "Prtfl")]
	pub prtfl: Option<PortfolioCode3Choice>,
	#[serde(rename = "MrgnPrtflCd")]
	pub mrgn_prtfl_cd: Option<MarginPortfolio3>,
}


// CounterpartyData92 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyData92 {
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "RptSubmitgNtty")]
	pub rpt_submitg_ntty: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "NttyRspnsblForRpt")]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
}


// CountryCode ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// DateAndDateTime2Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTime2Choice {
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm")]
	pub dt_tm: Option<String>,
}


// DerivativeEventType3Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativeEventType3Code {
	#[serde(rename = "DerivativeEventType3Code")]
	pub derivative_event_type3_code: String,
}


// DerivativesTradeWarningsReportV01 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativesTradeWarningsReportV01 {
	#[serde(rename = "WrnngsSttstcs")]
	pub wrnngs_sttstcs: StatisticsPerCounterparty16Choice,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// DetailedAbnormalValuesStatistics4Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DetailedAbnormalValuesStatistics4Choice {
	#[serde(rename = "DataSetActn")]
	pub data_set_actn: Option<String>,
	#[serde(rename = "Rpt")]
	pub rpt: Option<DetailedTransactionStatistics28>,
}


// DetailedMissingMarginInformationStatistics4Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DetailedMissingMarginInformationStatistics4Choice {
	#[serde(rename = "DataSetActn")]
	pub data_set_actn: Option<String>,
	#[serde(rename = "Rpt")]
	pub rpt: Option<DetailedTransactionStatistics26>,
}


// DetailedMissingValuationsStatistics4Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DetailedMissingValuationsStatistics4Choice {
	#[serde(rename = "DataSetActn")]
	pub data_set_actn: Option<String>,
	#[serde(rename = "Rpt")]
	pub rpt: Option<DetailedTransactionStatistics27>,
}


// DetailedStatisticsPerCounterparty17 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DetailedStatisticsPerCounterparty17 {
	#[serde(rename = "RefDt")]
	pub ref_dt: String,
	#[serde(rename = "MssngValtn")]
	pub mssng_valtn: DetailedMissingValuationsStatistics4Choice,
	#[serde(rename = "MssngMrgnInf")]
	pub mssng_mrgn_inf: DetailedMissingMarginInformationStatistics4Choice,
	#[serde(rename = "AbnrmlVals")]
	pub abnrml_vals: DetailedAbnormalValuesStatistics4Choice,
}


// DetailedTransactionStatistics26 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DetailedTransactionStatistics26 {
	#[serde(rename = "NbOfOutsdngDerivs")]
	pub nb_of_outsdng_derivs: f64,
	#[serde(rename = "NbOfOutsdngDerivsWthNoMrgnInf")]
	pub nb_of_outsdng_derivs_wth_no_mrgn_inf: f64,
	#[serde(rename = "NbOfOutsdngDerivsWthOutdtdMrgnInf")]
	pub nb_of_outsdng_derivs_wth_outdtd_mrgn_inf: f64,
	#[serde(rename = "Wrnngs")]
	pub wrnngs: Vec<MissingMarginData2>,
}


// DetailedTransactionStatistics27 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DetailedTransactionStatistics27 {
	#[serde(rename = "NbOfOutsdngDerivs")]
	pub nb_of_outsdng_derivs: f64,
	#[serde(rename = "NbOfOutsdngDerivsWthNoValtn")]
	pub nb_of_outsdng_derivs_wth_no_valtn: f64,
	#[serde(rename = "NbOfOutsdngDerivsWthOutdtdValtn")]
	pub nb_of_outsdng_derivs_wth_outdtd_valtn: f64,
	#[serde(rename = "Wrnngs")]
	pub wrnngs: Vec<MissingValuationsData2>,
}


// DetailedTransactionStatistics28 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DetailedTransactionStatistics28 {
	#[serde(rename = "NbOfDerivsRptd")]
	pub nb_of_derivs_rptd: f64,
	#[serde(rename = "NbOfDerivsRptdWthOtlrs")]
	pub nb_of_derivs_rptd_wth_otlrs: f64,
	#[serde(rename = "Wrnngs")]
	pub wrnngs: Vec<AbnormalValuesData4>,
}


// ExternalAgreementType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAgreementType1Code {
	#[serde(rename = "ExternalAgreementType1Code")]
	pub external_agreement_type1_code: String,
}


// ExternalUnitOfMeasure1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalUnitOfMeasure1Code {
	#[serde(rename = "ExternalUnitOfMeasure1Code")]
	pub external_unit_of_measure1_code: String,
}


// Frequency19Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Frequency19Code {
	#[serde(rename = "Frequency19Code")]
	pub frequency19_code: String,
}


// GenericIdentification175 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification175 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// ISODate ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// LEIIdentifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LegalPersonIdentification1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LegalPersonIdentification1 {
	#[serde(rename = "Id")]
	pub id: OrganisationIdentification15Choice,
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
}


// LongFraction19DecimalNumber ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LongFraction19DecimalNumber {
	#[serde(rename = "LongFraction19DecimalNumber")]
	pub long_fraction19_decimal_number: f64,
}


// MarginPortfolio3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarginPortfolio3 {
	#[serde(rename = "InitlMrgnPrtflCd")]
	pub initl_mrgn_prtfl_cd: PortfolioCode5Choice,
	#[serde(rename = "VartnMrgnPrtflCd")]
	pub vartn_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
}


// MasterAgreement8 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MasterAgreement8 {
	#[serde(rename = "Tp")]
	pub tp: Option<AgreementType2Choice>,
	#[serde(rename = "Vrsn")]
	pub vrsn: Option<String>,
	#[serde(rename = "OthrMstrAgrmtDtls")]
	pub othr_mstr_agrmt_dtls: Option<String>,
}


// Max105Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max105Text {
	#[serde(rename = "Max105Text")]
	pub max105_text: String,
}


// Max140Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max350Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Max3Number ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max3Number {
	#[serde(rename = "Max3Number")]
	pub max3_number: f64,
}


// Max500Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max500Text {
	#[serde(rename = "Max500Text")]
	pub max500_text: String,
}


// Max50Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max50Text {
	#[serde(rename = "Max50Text")]
	pub max50_text: String,
}


// Max52Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max52Text {
	#[serde(rename = "Max52Text")]
	pub max52_text: String,
}


// Max72Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max72Text {
	#[serde(rename = "Max72Text")]
	pub max72_text: String,
}


// MissingMarginData2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MissingMarginData2 {
	#[serde(rename = "CtrPtyId")]
	pub ctr_pty_id: CounterpartyData92,
	#[serde(rename = "NbOfOutsdngDerivs")]
	pub nb_of_outsdng_derivs: f64,
	#[serde(rename = "NbOfOutsdngDerivsWthNoMrgnInf")]
	pub nb_of_outsdng_derivs_wth_no_mrgn_inf: f64,
	#[serde(rename = "NbOfOutsdngDerivsWthOutdtdMrgnInf")]
	pub nb_of_outsdng_derivs_wth_outdtd_mrgn_inf: f64,
	#[serde(rename = "TxDtls")]
	pub tx_dtls: Option<Vec<MissingMarginTransactionData2>>,
}


// MissingMarginTransactionData2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MissingMarginTransactionData2 {
	#[serde(rename = "TxId")]
	pub tx_id: TradeTransactionIdentification24,
	#[serde(rename = "CollTmStmp")]
	pub coll_tm_stmp: Option<String>,
}


// MissingValuationsData2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MissingValuationsData2 {
	#[serde(rename = "CtrPtyId")]
	pub ctr_pty_id: CounterpartyData92,
	#[serde(rename = "NbOfOutsdngDerivs")]
	pub nb_of_outsdng_derivs: f64,
	#[serde(rename = "NbOfOutsdngDerivsWthNoValtn")]
	pub nb_of_outsdng_derivs_wth_no_valtn: f64,
	#[serde(rename = "NbOfOutsdngDerivsWthOutdtdValtn")]
	pub nb_of_outsdng_derivs_wth_outdtd_valtn: f64,
	#[serde(rename = "TxDtls")]
	pub tx_dtls: Option<Vec<MissingValuationsTransactionData2>>,
}


// MissingValuationsTransactionData2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MissingValuationsTransactionData2 {
	#[serde(rename = "TxId")]
	pub tx_id: TradeTransactionIdentification24,
	#[serde(rename = "ValtnAmt")]
	pub valtn_amt: Option<AmountAndDirection106>,
	#[serde(rename = "ValtnTmStmp")]
	pub valtn_tm_stmp: Option<DateAndDateTime2Choice>,
}


// NaturalPersonIdentification2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NaturalPersonIdentification2 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Dmcl")]
	pub dmcl: Option<String>,
}


// NaturalPersonIdentification3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NaturalPersonIdentification3 {
	#[serde(rename = "Id")]
	pub id: NaturalPersonIdentification2,
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
}


// NotApplicable1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotApplicable1Code {
	#[serde(rename = "NotApplicable1Code")]
	pub not_applicable1_code: String,
}


// NotionalAmount5 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalAmount5 {
	#[serde(rename = "Amt")]
	pub amt: Option<AmountAndDirection106>,
	#[serde(rename = "SchdlPrd")]
	pub schdl_prd: Option<Vec<Schedule11>>,
}


// NotionalAmount6 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalAmount6 {
	#[serde(rename = "Amt")]
	pub amt: Option<AmountAndDirection106>,
	#[serde(rename = "SchdlPrd")]
	pub schdl_prd: Option<Vec<Schedule11>>,
	#[serde(rename = "Ccy")]
	pub ccy: Option<String>,
}


// NotionalAmountLegs5 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalAmountLegs5 {
	#[serde(rename = "FrstLeg")]
	pub frst_leg: Option<NotionalAmount5>,
	#[serde(rename = "ScndLeg")]
	pub scnd_leg: Option<NotionalAmount6>,
}


// NotionalQuantity9 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalQuantity9 {
	#[serde(rename = "TtlQty")]
	pub ttl_qty: Option<f64>,
	#[serde(rename = "UnitOfMeasr")]
	pub unit_of_measr: Option<UnitOfMeasure8Choice>,
	#[serde(rename = "Dtls")]
	pub dtls: Option<QuantityOrTerm1Choice>,
}


// NotionalQuantityLegs5 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalQuantityLegs5 {
	#[serde(rename = "FrstLeg")]
	pub frst_leg: Option<NotionalQuantity9>,
	#[serde(rename = "ScndLeg")]
	pub scnd_leg: Option<NotionalQuantity9>,
}


// Number ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
}


// OrganisationIdentification15Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification15Choice {
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<OrganisationIdentification38>,
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
}


// OrganisationIdentification38 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification38 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Dmcl")]
	pub dmcl: Option<String>,
}


// PartyIdentification248Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification248Choice {
	#[serde(rename = "Lgl")]
	pub lgl: Option<LegalPersonIdentification1>,
	#[serde(rename = "Ntrl")]
	pub ntrl: Option<NaturalPersonIdentification3>,
}


// PlusOrMinusIndicator ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "PlusOrMinusIndicator")]
	pub plus_or_minus_indicator: bool,
}


// PortfolioCode3Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioCode3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "NoPrtfl")]
	pub no_prtfl: Option<String>,
}


// PortfolioCode5Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioCode5Choice {
	#[serde(rename = "Prtfl")]
	pub prtfl: Option<PortfolioIdentification3>,
	#[serde(rename = "NoPrtfl")]
	pub no_prtfl: Option<String>,
}


// PortfolioIdentification3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioIdentification3 {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "PrtflTxXmptn")]
	pub prtfl_tx_xmptn: Option<bool>,
}


// QuantityOrTerm1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct QuantityOrTerm1Choice {
	#[serde(rename = "SchdlPrd")]
	pub schdl_prd: Option<Vec<Schedule10>>,
	#[serde(rename = "Term")]
	pub term: Option<QuantityTerm1>,
}


// QuantityTerm1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct QuantityTerm1 {
	#[serde(rename = "Qty")]
	pub qty: Option<f64>,
	#[serde(rename = "UnitOfMeasr")]
	pub unit_of_measr: Option<UnitOfMeasure8Choice>,
	#[serde(rename = "Val")]
	pub val: Option<f64>,
	#[serde(rename = "TmUnit")]
	pub tm_unit: Option<String>,
}


// ReportPeriodActivity1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportPeriodActivity1Code {
	#[serde(rename = "ReportPeriodActivity1Code")]
	pub report_period_activity1_code: String,
}


// Schedule10 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Schedule10 {
	#[serde(rename = "Qty")]
	pub qty: f64,
	#[serde(rename = "UnitOfMeasr")]
	pub unit_of_measr: Option<UnitOfMeasure8Choice>,
	#[serde(rename = "UadjstdFctvDt")]
	pub uadjstd_fctv_dt: String,
	#[serde(rename = "UadjstdEndDt")]
	pub uadjstd_end_dt: Option<String>,
}


// Schedule11 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Schedule11 {
	#[serde(rename = "UadjstdFctvDt")]
	pub uadjstd_fctv_dt: String,
	#[serde(rename = "UadjstdEndDt")]
	pub uadjstd_end_dt: Option<String>,
	#[serde(rename = "Amt")]
	pub amt: AmountAndDirection106,
}


// StatisticsPerCounterparty16Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatisticsPerCounterparty16Choice {
	#[serde(rename = "DataSetActn")]
	pub data_set_actn: Option<String>,
	#[serde(rename = "Rpt")]
	pub rpt: Option<DetailedStatisticsPerCounterparty17>,
}


// SupplementaryData1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Option<String>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}


// TradeTransactionIdentification24 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeTransactionIdentification24 {
	#[serde(rename = "TechRcrdId")]
	pub tech_rcrd_id: Option<String>,
	#[serde(rename = "ActnTp")]
	pub actn_tp: Option<String>,
	#[serde(rename = "RptgTmStmp")]
	pub rptg_tm_stmp: Option<String>,
	#[serde(rename = "DerivEvtTp")]
	pub deriv_evt_tp: Option<String>,
	#[serde(rename = "DerivEvtTmStmp")]
	pub deriv_evt_tm_stmp: Option<DateAndDateTime2Choice>,
	#[serde(rename = "OthrCtrPty")]
	pub othr_ctr_pty: Option<PartyIdentification248Choice>,
	#[serde(rename = "UnqIdr")]
	pub unq_idr: Option<UniqueTransactionIdentifier2Choice>,
	#[serde(rename = "MstrAgrmt")]
	pub mstr_agrmt: Option<MasterAgreement8>,
	#[serde(rename = "CollPrtflCd")]
	pub coll_prtfl_cd: Option<CollateralPortfolioCode5Choice>,
}


// TransactionOperationType10Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionOperationType10Code {
	#[serde(rename = "TransactionOperationType10Code")]
	pub transaction_operation_type10_code: String,
}


// TrueFalseIndicator ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}


// UTIIdentifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UTIIdentifier {
	#[serde(rename = "UTIIdentifier")]
	pub uti_identifier: String,
}


// UniqueTransactionIdentifier2Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UniqueTransactionIdentifier2Choice {
	#[serde(rename = "UnqTxIdr")]
	pub unq_tx_idr: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification175>,
}


// UnitOfMeasure8Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitOfMeasure8Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification175>,
}
