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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AbnormalValuesData4 {
	#[serde(rename = "CtrPtyId")]
	pub ctr_pty_id: CounterpartyData92,
	#[serde(rename = "NbOfDerivsRptd")]
	pub nb_of_derivs_rptd: f64,
	#[serde(rename = "NbOfDerivsRptdWthOtlrs")]
	pub nb_of_derivs_rptd_wth_otlrs: f64,
	#[serde(rename = "TxDtls", skip_serializing_if = "Option::is_none")]
	pub tx_dtls: Option<Vec<AbnormalValuesTransactionData2>>,
}


// AbnormalValuesTransactionData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AbnormalValuesTransactionData2 {
	#[serde(rename = "TxId")]
	pub tx_id: TradeTransactionIdentification24,
	#[serde(rename = "NtnlAmt", skip_serializing_if = "Option::is_none")]
	pub ntnl_amt: Option<NotionalAmountLegs5>,
	#[serde(rename = "NtnlQty", skip_serializing_if = "Option::is_none")]
	pub ntnl_qty: Option<NotionalQuantityLegs5>,
}


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "$value")]
	pub active_currency_code: String,
}


// ActiveOrHistoricCurrencyAnd19DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd19DecimalAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_and19_decimal_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAnd19DecimalAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd19DecimalAmount {
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


// AgreementType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgreementType2Choice {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<ExternalAgreementType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max50Text>,
}


// AmountAndDirection106 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection106 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAnd19DecimalAmount,
	#[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
	pub sgn: Option<bool>,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
}


// CollateralPortfolioCode5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralPortfolioCode5Choice {
	#[serde(rename = "Prtfl", skip_serializing_if = "Option::is_none")]
	pub prtfl: Option<PortfolioCode3Choice>,
	#[serde(rename = "MrgnPrtflCd", skip_serializing_if = "Option::is_none")]
	pub mrgn_prtfl_cd: Option<MarginPortfolio3>,
}


// CounterpartyData92 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyData92 {
	#[serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none")]
	pub rptg_ctr_pty: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "RptSubmitgNtty", skip_serializing_if = "Option::is_none")]
	pub rpt_submitg_ntty: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none")]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// DateAndDateTime2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTime2Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<String>,
}


// DerivativeEventType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum DerivativeEventType3Code {
	#[default]
	#[serde(rename = "ALOC")]
	CodeALOC,
	#[serde(rename = "CLRG")]
	CodeCLRG,
	#[serde(rename = "CLAL")]
	CodeCLAL,
	#[serde(rename = "COMP")]
	CodeCOMP,
	#[serde(rename = "CORP")]
	CodeCORP,
	#[serde(rename = "CREV")]
	CodeCREV,
	#[serde(rename = "ETRM")]
	CodeETRM,
	#[serde(rename = "EXER")]
	CodeEXER,
	#[serde(rename = "INCP")]
	CodeINCP,
	#[serde(rename = "NOVA")]
	CodeNOVA,
	#[serde(rename = "PTNG")]
	CodePTNG,
	#[serde(rename = "TRAD")]
	CodeTRAD,
	#[serde(rename = "UPDT")]
	CodeUPDT,
}


// DerivativesTradeWarningsReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativesTradeWarningsReportV01 {
	#[serde(rename = "WrnngsSttstcs")]
	pub wrnngs_sttstcs: StatisticsPerCounterparty16Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// DetailedAbnormalValuesStatistics4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DetailedAbnormalValuesStatistics4Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[serde(rename = "Rpt", skip_serializing_if = "Option::is_none")]
	pub rpt: Option<DetailedTransactionStatistics28>,
}


// DetailedMissingMarginInformationStatistics4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DetailedMissingMarginInformationStatistics4Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[serde(rename = "Rpt", skip_serializing_if = "Option::is_none")]
	pub rpt: Option<DetailedTransactionStatistics26>,
}


// DetailedMissingValuationsStatistics4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DetailedMissingValuationsStatistics4Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[serde(rename = "Rpt", skip_serializing_if = "Option::is_none")]
	pub rpt: Option<DetailedTransactionStatistics27>,
}


// DetailedStatisticsPerCounterparty17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DetailedTransactionStatistics28 {
	#[serde(rename = "NbOfDerivsRptd")]
	pub nb_of_derivs_rptd: f64,
	#[serde(rename = "NbOfDerivsRptdWthOtlrs")]
	pub nb_of_derivs_rptd_wth_otlrs: f64,
	#[serde(rename = "Wrnngs")]
	pub wrnngs: Vec<AbnormalValuesData4>,
}


// ExternalAgreementType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAgreementType1Code {
	#[serde(rename = "$value")]
	pub external_agreement_type1_code: String,
}


// ExternalUnitOfMeasure1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalUnitOfMeasure1Code {
	#[serde(rename = "$value")]
	pub external_unit_of_measure1_code: String,
}


// Frequency19Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Frequency19Code {
	#[default]
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "YEAR")]
	CodeYEAR,
	#[serde(rename = "ADHO")]
	CodeADHO,
	#[serde(rename = "EXPI")]
	CodeEXPI,
	#[serde(rename = "MIAN")]
	CodeMIAN,
	#[serde(rename = "QURT")]
	CodeQURT,
	#[serde(rename = "HOUL")]
	CodeHOUL,
	#[serde(rename = "ODMD")]
	CodeODMD,
}


// GenericIdentification175 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification175 {
	#[serde(rename = "Id")]
	pub id: Max72Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
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


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}


// LegalPersonIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LegalPersonIdentification1 {
	#[serde(rename = "Id")]
	pub id: OrganisationIdentification15Choice,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
}


// LongFraction19DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LongFraction19DecimalNumber {
	#[serde(rename = "$value")]
	pub long_fraction19_decimal_number: f64,
}


// MarginPortfolio3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarginPortfolio3 {
	#[serde(rename = "InitlMrgnPrtflCd")]
	pub initl_mrgn_prtfl_cd: PortfolioCode5Choice,
	#[serde(rename = "VartnMrgnPrtflCd", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
}


// MasterAgreement8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MasterAgreement8 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<AgreementType2Choice>,
	#[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
	pub vrsn: Option<Max50Text>,
	#[serde(rename = "OthrMstrAgrmtDtls", skip_serializing_if = "Option::is_none")]
	pub othr_mstr_agrmt_dtls: Option<Max350Text>,
}


// Max105Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max105Text {
	#[serde(rename = "$value")]
	pub max105_text: String,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
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


// Max500Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max500Text {
	#[serde(rename = "$value")]
	pub max500_text: String,
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


// Max72Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max72Text {
	#[serde(rename = "$value")]
	pub max72_text: String,
}


// MissingMarginData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MissingMarginData2 {
	#[serde(rename = "CtrPtyId")]
	pub ctr_pty_id: CounterpartyData92,
	#[serde(rename = "NbOfOutsdngDerivs")]
	pub nb_of_outsdng_derivs: f64,
	#[serde(rename = "NbOfOutsdngDerivsWthNoMrgnInf")]
	pub nb_of_outsdng_derivs_wth_no_mrgn_inf: f64,
	#[serde(rename = "NbOfOutsdngDerivsWthOutdtdMrgnInf")]
	pub nb_of_outsdng_derivs_wth_outdtd_mrgn_inf: f64,
	#[serde(rename = "TxDtls", skip_serializing_if = "Option::is_none")]
	pub tx_dtls: Option<Vec<MissingMarginTransactionData2>>,
}


// MissingMarginTransactionData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MissingMarginTransactionData2 {
	#[serde(rename = "TxId")]
	pub tx_id: TradeTransactionIdentification24,
	#[serde(rename = "CollTmStmp", skip_serializing_if = "Option::is_none")]
	pub coll_tm_stmp: Option<String>,
}


// MissingValuationsData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MissingValuationsData2 {
	#[serde(rename = "CtrPtyId")]
	pub ctr_pty_id: CounterpartyData92,
	#[serde(rename = "NbOfOutsdngDerivs")]
	pub nb_of_outsdng_derivs: f64,
	#[serde(rename = "NbOfOutsdngDerivsWthNoValtn")]
	pub nb_of_outsdng_derivs_wth_no_valtn: f64,
	#[serde(rename = "NbOfOutsdngDerivsWthOutdtdValtn")]
	pub nb_of_outsdng_derivs_wth_outdtd_valtn: f64,
	#[serde(rename = "TxDtls", skip_serializing_if = "Option::is_none")]
	pub tx_dtls: Option<Vec<MissingValuationsTransactionData2>>,
}


// MissingValuationsTransactionData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MissingValuationsTransactionData2 {
	#[serde(rename = "TxId")]
	pub tx_id: TradeTransactionIdentification24,
	#[serde(rename = "ValtnAmt", skip_serializing_if = "Option::is_none")]
	pub valtn_amt: Option<AmountAndDirection106>,
	#[serde(rename = "ValtnTmStmp", skip_serializing_if = "Option::is_none")]
	pub valtn_tm_stmp: Option<DateAndDateTime2Choice>,
}


// NaturalPersonIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NaturalPersonIdentification2 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max105Text>,
	#[serde(rename = "Dmcl", skip_serializing_if = "Option::is_none")]
	pub dmcl: Option<Max500Text>,
}


// NaturalPersonIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NaturalPersonIdentification3 {
	#[serde(rename = "Id")]
	pub id: NaturalPersonIdentification2,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
}


// NotApplicable1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NotApplicable1Code {
	#[default]
	#[serde(rename = "NOAP")]
	CodeNOAP,
}


// NotionalAmount5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalAmount5 {
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<AmountAndDirection106>,
	#[serde(rename = "SchdlPrd", skip_serializing_if = "Option::is_none")]
	pub schdl_prd: Option<Vec<Schedule11>>,
}


// NotionalAmount6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalAmount6 {
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<AmountAndDirection106>,
	#[serde(rename = "SchdlPrd", skip_serializing_if = "Option::is_none")]
	pub schdl_prd: Option<Vec<Schedule11>>,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveOrHistoricCurrencyCode>,
}


// NotionalAmountLegs5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalAmountLegs5 {
	#[serde(rename = "FrstLeg", skip_serializing_if = "Option::is_none")]
	pub frst_leg: Option<NotionalAmount5>,
	#[serde(rename = "ScndLeg", skip_serializing_if = "Option::is_none")]
	pub scnd_leg: Option<NotionalAmount6>,
}


// NotionalQuantity9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalQuantity9 {
	#[serde(rename = "TtlQty", skip_serializing_if = "Option::is_none")]
	pub ttl_qty: Option<f64>,
	#[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
	pub unit_of_measr: Option<UnitOfMeasure8Choice>,
	#[serde(rename = "Dtls", skip_serializing_if = "Option::is_none")]
	pub dtls: Option<QuantityOrTerm1Choice>,
}


// NotionalQuantityLegs5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalQuantityLegs5 {
	#[serde(rename = "FrstLeg", skip_serializing_if = "Option::is_none")]
	pub frst_leg: Option<NotionalQuantity9>,
	#[serde(rename = "ScndLeg", skip_serializing_if = "Option::is_none")]
	pub scnd_leg: Option<NotionalQuantity9>,
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "$value")]
	pub number: f64,
}


// OrganisationIdentification15Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification15Choice {
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<OrganisationIdentification38>,
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
}


// OrganisationIdentification38 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification38 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max105Text>,
	#[serde(rename = "Dmcl", skip_serializing_if = "Option::is_none")]
	pub dmcl: Option<Max500Text>,
}


// PartyIdentification248Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification248Choice {
	#[serde(rename = "Lgl", skip_serializing_if = "Option::is_none")]
	pub lgl: Option<LegalPersonIdentification1>,
	#[serde(rename = "Ntrl", skip_serializing_if = "Option::is_none")]
	pub ntrl: Option<NaturalPersonIdentification3>,
}


// PlusOrMinusIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "$value")]
	pub plus_or_minus_indicator: bool,
}


// PortfolioCode3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioCode3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Max52Text>,
	#[serde(rename = "NoPrtfl", skip_serializing_if = "Option::is_none")]
	pub no_prtfl: Option<NotApplicable1Code>,
}


// PortfolioCode5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioCode5Choice {
	#[serde(rename = "Prtfl", skip_serializing_if = "Option::is_none")]
	pub prtfl: Option<PortfolioIdentification3>,
	#[serde(rename = "NoPrtfl", skip_serializing_if = "Option::is_none")]
	pub no_prtfl: Option<NotApplicable1Code>,
}


// PortfolioIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioIdentification3 {
	#[serde(rename = "Cd")]
	pub cd: Max52Text,
	#[serde(rename = "PrtflTxXmptn", skip_serializing_if = "Option::is_none")]
	pub prtfl_tx_xmptn: Option<bool>,
}


// QuantityOrTerm1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct QuantityOrTerm1Choice {
	#[serde(rename = "SchdlPrd", skip_serializing_if = "Option::is_none")]
	pub schdl_prd: Option<Vec<Schedule10>>,
	#[serde(rename = "Term", skip_serializing_if = "Option::is_none")]
	pub term: Option<QuantityTerm1>,
}


// QuantityTerm1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct QuantityTerm1 {
	#[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
	pub qty: Option<f64>,
	#[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
	pub unit_of_measr: Option<UnitOfMeasure8Choice>,
	#[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
	pub val: Option<f64>,
	#[serde(rename = "TmUnit", skip_serializing_if = "Option::is_none")]
	pub tm_unit: Option<Frequency19Code>,
}


// ReportPeriodActivity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ReportPeriodActivity1Code {
	#[default]
	#[serde(rename = "NOTX")]
	CodeNOTX,
}


// Schedule10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Schedule10 {
	#[serde(rename = "Qty")]
	pub qty: f64,
	#[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
	pub unit_of_measr: Option<UnitOfMeasure8Choice>,
	#[serde(rename = "UadjstdFctvDt")]
	pub uadjstd_fctv_dt: String,
	#[serde(rename = "UadjstdEndDt", skip_serializing_if = "Option::is_none")]
	pub uadjstd_end_dt: Option<String>,
}


// Schedule11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Schedule11 {
	#[serde(rename = "UadjstdFctvDt")]
	pub uadjstd_fctv_dt: String,
	#[serde(rename = "UadjstdEndDt", skip_serializing_if = "Option::is_none")]
	pub uadjstd_end_dt: Option<String>,
	#[serde(rename = "Amt")]
	pub amt: AmountAndDirection106,
}


// StatisticsPerCounterparty16Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatisticsPerCounterparty16Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[serde(rename = "Rpt", skip_serializing_if = "Option::is_none")]
	pub rpt: Option<DetailedStatisticsPerCounterparty17>,
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


// TradeTransactionIdentification24 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeTransactionIdentification24 {
	#[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
	pub tech_rcrd_id: Option<Max140Text>,
	#[serde(rename = "ActnTp", skip_serializing_if = "Option::is_none")]
	pub actn_tp: Option<TransactionOperationType10Code>,
	#[serde(rename = "RptgTmStmp", skip_serializing_if = "Option::is_none")]
	pub rptg_tm_stmp: Option<String>,
	#[serde(rename = "DerivEvtTp", skip_serializing_if = "Option::is_none")]
	pub deriv_evt_tp: Option<DerivativeEventType3Code>,
	#[serde(rename = "DerivEvtTmStmp", skip_serializing_if = "Option::is_none")]
	pub deriv_evt_tm_stmp: Option<DateAndDateTime2Choice>,
	#[serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none")]
	pub othr_ctr_pty: Option<PartyIdentification248Choice>,
	#[serde(rename = "UnqIdr", skip_serializing_if = "Option::is_none")]
	pub unq_idr: Option<UniqueTransactionIdentifier2Choice>,
	#[serde(rename = "MstrAgrmt", skip_serializing_if = "Option::is_none")]
	pub mstr_agrmt: Option<MasterAgreement8>,
	#[serde(rename = "CollPrtflCd", skip_serializing_if = "Option::is_none")]
	pub coll_prtfl_cd: Option<CollateralPortfolioCode5Choice>,
}


// TransactionOperationType10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TransactionOperationType10Code {
	#[default]
	#[serde(rename = "COMP")]
	CodeCOMP,
	#[serde(rename = "CORR")]
	CodeCORR,
	#[serde(rename = "EROR")]
	CodeEROR,
	#[serde(rename = "MODI")]
	CodeMODI,
	#[serde(rename = "NEWT")]
	CodeNEWT,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "POSC")]
	CodePOSC,
	#[serde(rename = "REVI")]
	CodeREVI,
	#[serde(rename = "TERM")]
	CodeTERM,
	#[serde(rename = "VALU")]
	CodeVALU,
	#[serde(rename = "MARU")]
	CodeMARU,
	#[serde(rename = "PRTO")]
	CodePRTO,
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}


// UTIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UTIIdentifier {
	#[serde(rename = "$value")]
	pub uti_identifier: String,
}


// UniqueTransactionIdentifier2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UniqueTransactionIdentifier2Choice {
	#[serde(rename = "UnqTxIdr", skip_serializing_if = "Option::is_none")]
	pub unq_tx_idr: Option<UTIIdentifier>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification175>,
}


// UnitOfMeasure8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitOfMeasure8Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalUnitOfMeasure1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification175>,
}
