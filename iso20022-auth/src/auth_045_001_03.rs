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


// AssetClassAndSubClassIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassAndSubClassIdentification2 {
	#[serde(rename = "AsstClss")]
	pub asst_clss: NonEquityAssetClass1Code,
	#[serde(rename = "DerivSubClss", skip_serializing_if = "Option::is_none")]
	pub deriv_sub_clss: Option<NonEquitySubClass1>,
	#[serde(rename = "FinInstrmClssfctn", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_clssfctn: Option<NonEquityInstrumentReportingClassification1Code>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "$value")]
	pub decimal_number: f64,
}


// FinancialInstrumentReportingNonEquityTradingActivityResultV03 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentReportingNonEquityTradingActivityResultV03 {
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: SecuritiesMarketReportHeader1,
	#[serde(rename = "NonEqtyTrnsprncyData")]
	pub non_eqty_trnsprncy_data: Vec<TransparencyDataReport20>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
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


// InstrumentAndSubClassIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InstrumentAndSubClassIdentification2 {
	#[serde(rename = "ISIN")]
	pub isin: ISINOct2015Identifier,
	#[serde(rename = "DerivSubClss", skip_serializing_if = "Option::is_none")]
	pub deriv_sub_clss: Option<NonEquitySubClass1>,
	#[serde(rename = "FinInstrmClssfctn", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_clssfctn: Option<NonEquityInstrumentReportingClassification1Code>,
}


// InstrumentOrSubClassIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InstrumentOrSubClassIdentification2Choice {
	#[serde(rename = "ISINAndSubClss", skip_serializing_if = "Option::is_none")]
	pub isin_and_sub_clss: Option<InstrumentAndSubClassIdentification2>,
	#[serde(rename = "AsstClssAndSubClss", skip_serializing_if = "Option::is_none")]
	pub asst_clss_and_sub_clss: Option<AssetClassAndSubClassIdentification2>,
}


// MICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MICIdentifier {
	#[serde(rename = "$value")]
	pub mic_identifier: String,
}


// Max1000Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max1000Text {
	#[serde(rename = "$value")]
	pub max1000_text: String,
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


// Max50Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max50Text {
	#[serde(rename = "$value")]
	pub max50_text: String,
}


// NonEquityAssetClass1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NonEquityAssetClass1Code {
	#[default]
	#[serde(rename = "SDRV")]
	CodeSDRV,
	#[serde(rename = "IRDV")]
	CodeIRDV,
	#[serde(rename = "FEXD")]
	CodeFEXD,
	#[serde(rename = "EQDV")]
	CodeEQDV,
	#[serde(rename = "EADV")]
	CodeEADV,
	#[serde(rename = "EMAL")]
	CodeEMAL,
	#[serde(rename = "CRDV")]
	CodeCRDV,
	#[serde(rename = "CFDS")]
	CodeCFDS,
	#[serde(rename = "COMD")]
	CodeCOMD,
	#[serde(rename = "C10D")]
	CodeC10D,
	#[serde(rename = "BOND")]
	CodeBOND,
	#[serde(rename = "ETCS")]
	CodeETCS,
	#[serde(rename = "ETNS")]
	CodeETNS,
	#[serde(rename = "SFPS")]
	CodeSFPS,
}


// NonEquityInstrumentReportingClassification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NonEquityInstrumentReportingClassification1Code {
	#[default]
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
}


// NonEquitySubClass1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NonEquitySubClass1 {
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max1000Text>,
	#[serde(rename = "SgmttnCrit")]
	pub sgmttn_crit: Vec<NonEquitySubClassSegmentationCriterion1>,
}


// NonEquitySubClassSegmentationCriteria1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NonEquitySubClassSegmentationCriteria1Code {
	#[default]
	#[serde(rename = "ASCL")]
	CodeASCL,
	#[serde(rename = "BSPD")]
	CodeBSPD,
	#[serde(rename = "CNC1")]
	CodeCNC1,
	#[serde(rename = "CNC2")]
	CodeCNC2,
	#[serde(rename = "NCCO")]
	CodeNCCO,
	#[serde(rename = "CTYP")]
	CodeCTYP,
	#[serde(rename = "NCCR")]
	CodeNCCR,
	#[serde(rename = "DCSL")]
	CodeDCSL,
	#[serde(rename = "DTYP")]
	CodeDTYP,
	#[serde(rename = "EQUT")]
	CodeEQUT,
	#[serde(rename = "FNC1")]
	CodeFNC1,
	#[serde(rename = "FNC2")]
	CodeFNC2,
	#[serde(rename = "FSPD")]
	CodeFSPD,
	#[serde(rename = "IIND")]
	CodeIIND,
	#[serde(rename = "IRTC")]
	CodeIRTC,
	#[serde(rename = "INC1")]
	CodeINC1,
	#[serde(rename = "INC2")]
	CodeINC2,
	#[serde(rename = "ISIN")]
	CodeISIN,
	#[serde(rename = "TTMO")]
	CodeTTMO,
	#[serde(rename = "PRMT")]
	CodePRMT,
	#[serde(rename = "SSRF")]
	CodeSSRF,
	#[serde(rename = "ISPT")]
	CodeISPT,
	#[serde(rename = "SRTC")]
	CodeSRTC,
	#[serde(rename = "SACL")]
	CodeSACL,
	#[serde(rename = "SBPD")]
	CodeSBPD,
	#[serde(rename = "TTMS")]
	CodeTTMS,
	#[serde(rename = "NCSW")]
	CodeNCSW,
	#[serde(rename = "TTMB")]
	CodeTTMB,
	#[serde(rename = "IOUB")]
	CodeIOUB,
	#[serde(rename = "TOUB")]
	CodeTOUB,
	#[serde(rename = "UISC")]
	CodeUISC,
	#[serde(rename = "UIDX")]
	CodeUIDX,
	#[serde(rename = "UINS")]
	CodeUINS,
	#[serde(rename = "UIRT")]
	CodeUIRT,
	#[serde(rename = "REOU")]
	CodeREOU,
	#[serde(rename = "UTYP")]
	CodeUTYP,
}


// NonEquitySubClassSegmentationCriterion1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NonEquitySubClassSegmentationCriterion1 {
	#[serde(rename = "CritNm")]
	pub crit_nm: NonEquitySubClassSegmentationCriteria1Code,
	#[serde(rename = "CritVal")]
	pub crit_val: Max1000Text,
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


// StatisticsTransparency2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatisticsTransparency2 {
	#[serde(rename = "TtlNbOfTxsExctd")]
	pub ttl_nb_of_txs_exctd: f64,
	#[serde(rename = "TtlVolOfTxsExctd")]
	pub ttl_vol_of_txs_exctd: f64,
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


// TonsOrCurrency2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TonsOrCurrency2Choice {
	#[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
	pub nb: Option<f64>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// TradingVenue2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TradingVenue2Code {
	#[default]
	#[serde(rename = "APPA")]
	CodeAPPA,
	#[serde(rename = "CTPS")]
	CodeCTPS,
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


// TransparencyDataReport20 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransparencyDataReport20 {
	#[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
	pub tech_rcrd_id: Option<Max35Text>,
	#[serde(rename = "Id")]
	pub id: InstrumentOrSubClassIdentification2Choice,
	#[serde(rename = "FullNm", skip_serializing_if = "Option::is_none")]
	pub full_nm: Option<Max350Text>,
	#[serde(rename = "TradgVn", skip_serializing_if = "Option::is_none")]
	pub tradg_vn: Option<MICIdentifier>,
	#[serde(rename = "RptgPrd", skip_serializing_if = "Option::is_none")]
	pub rptg_prd: Option<Period4Choice>,
	#[serde(rename = "Lqdty", skip_serializing_if = "Option::is_none")]
	pub lqdty: Option<bool>,
	#[serde(rename = "PreTradLrgInScaleThrshld", skip_serializing_if = "Option::is_none")]
	pub pre_trad_lrg_in_scale_thrshld: Option<TonsOrCurrency2Choice>,
	#[serde(rename = "PstTradLrgInScaleThrshld", skip_serializing_if = "Option::is_none")]
	pub pst_trad_lrg_in_scale_thrshld: Option<TonsOrCurrency2Choice>,
	#[serde(rename = "PreTradInstrmSzSpcfcThrshld", skip_serializing_if = "Option::is_none")]
	pub pre_trad_instrm_sz_spcfc_thrshld: Option<TonsOrCurrency2Choice>,
	#[serde(rename = "PstTradInstrmSzSpcfcThrshld", skip_serializing_if = "Option::is_none")]
	pub pst_trad_instrm_sz_spcfc_thrshld: Option<TonsOrCurrency2Choice>,
	#[serde(rename = "Sttstcs", skip_serializing_if = "Option::is_none")]
	pub sttstcs: Option<StatisticsTransparency2>,
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}
