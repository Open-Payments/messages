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


// AddressType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType2Code {
	#[serde(rename = "AddressType2Code")]
	pub address_type2_code: String,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// AnyMIC1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyMIC1Code {
	#[serde(rename = "AnyMIC1Code")]
	pub any_mic1_code: String,
}


// CollateralType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralType6Code {
	#[serde(rename = "CollateralType6Code")]
	pub collateral_type6_code: String,
}


// CorporateSectorCriteria5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CorporateSectorCriteria5 {
	#[serde(rename = "FISctr")]
	pub fi_sctr: Option<Vec<String>>,
	#[serde(rename = "NFISctr")]
	pub nfi_sctr: Option<Vec<String>>,
	#[serde(rename = "NotRptd")]
	pub not_rptd: Option<String>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// DateOrBlankQuery2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateOrBlankQuery2Choice {
	#[serde(rename = "Rg")]
	pub rg: Option<DatePeriod1>,
	#[serde(rename = "NotRptd")]
	pub not_rptd: Option<String>,
}


// DatePeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriod1 {
	#[serde(rename = "FrDt")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// DateTimePeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod1 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: String,
}


// DayOfMonthNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DayOfMonthNumber {
	#[serde(rename = "DayOfMonthNumber")]
	pub day_of_month_number: f64,
}


// ExposureType10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExposureType10Code {
	#[serde(rename = "ExposureType10Code")]
	pub exposure_type10_code: String,
}


// FinancialPartySectorType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialPartySectorType2Code {
	#[serde(rename = "FinancialPartySectorType2Code")]
	pub financial_party_sector_type2_code: String,
}


// Frequency14Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Frequency14Code {
	#[serde(rename = "Frequency14Code")]
	pub frequency14_code: String,
}


// GenericIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
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


// Max1000Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max1000Text {
	#[serde(rename = "Max1000Text")]
	pub max1000_text: String,
}


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max16Text {
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
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


// Max50Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max50Text {
	#[serde(rename = "Max50Text")]
	pub max50_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// NACEDomainIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NACEDomainIdentifier {
	#[serde(rename = "NACEDomainIdentifier")]
	pub nace_domain_identifier: String,
}


// NameAndAddress5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Adr")]
	pub adr: Option<PostalAddress1>,
}


// NotReported1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotReported1Code {
	#[serde(rename = "NotReported1Code")]
	pub not_reported1_code: String,
}


// Operation3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Operation3Code {
	#[serde(rename = "Operation3Code")]
	pub operation3_code: String,
}


// PartyIdentification121Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification121Choice {
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
	#[serde(rename = "LglNttyIdr")]
	pub lgl_ntty_idr: Option<String>,
	#[serde(rename = "NmAndAdr")]
	pub nm_and_adr: Option<NameAndAddress5>,
	#[serde(rename = "PrtryId")]
	pub prtry_id: Option<GenericIdentification1>,
}


// PartyNatureType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyNatureType1Code {
	#[serde(rename = "PartyNatureType1Code")]
	pub party_nature_type1_code: String,
}


// PostalAddress1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress1 {
	#[serde(rename = "AdrTp")]
	pub adr_tp: Option<String>,
	#[serde(rename = "AdrLine")]
	pub adr_line: Option<Vec<String>>,
	#[serde(rename = "StrtNm")]
	pub strt_nm: Option<String>,
	#[serde(rename = "BldgNb")]
	pub bldg_nb: Option<String>,
	#[serde(rename = "PstCd")]
	pub pst_cd: Option<String>,
	#[serde(rename = "TwnNm")]
	pub twn_nm: Option<String>,
	#[serde(rename = "CtrySubDvsn")]
	pub ctry_sub_dvsn: Option<String>,
	#[serde(rename = "Ctry")]
	pub ctry: String,
}


// SecuritiesFinancingReportingTransactionQueryV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesFinancingReportingTransactionQueryV02 {
	#[serde(rename = "RqstngAuthrty")]
	pub rqstng_authrty: PartyIdentification121Choice,
	#[serde(rename = "TradQryData")]
	pub trad_qry_data: TradeReportQuery13Choice,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// SecuritiesTradeVenueCriteria1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTradeVenueCriteria1Choice {
	#[serde(rename = "MIC")]
	pub mic: Option<Vec<String>>,
	#[serde(rename = "AnyMIC")]
	pub any_mic: Option<String>,
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


// TradeAdditionalQueryCriteria7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeAdditionalQueryCriteria7 {
	#[serde(rename = "ActnTp")]
	pub actn_tp: Option<Vec<String>>,
	#[serde(rename = "ExctnVn")]
	pub exctn_vn: Option<SecuritiesTradeVenueCriteria1Choice>,
	#[serde(rename = "NtrOfCtrPty")]
	pub ntr_of_ctr_pty: Option<Vec<String>>,
	#[serde(rename = "CorpSctr")]
	pub corp_sctr: Option<Vec<CorporateSectorCriteria5>>,
}


// TradeDateTimeQueryCriteria2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeDateTimeQueryCriteria2 {
	#[serde(rename = "RptgDtTm")]
	pub rptg_dt_tm: Option<DateTimePeriod1>,
	#[serde(rename = "ExctnDtTm")]
	pub exctn_dt_tm: Option<DateTimePeriod1>,
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: Option<DateOrBlankQuery2Choice>,
	#[serde(rename = "TermntnDt")]
	pub termntn_dt: Option<DateOrBlankQuery2Choice>,
}


// TradePartyIdentificationQuery8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradePartyIdentificationQuery8 {
	#[serde(rename = "LEI")]
	pub lei: Option<Vec<String>>,
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<Vec<String>>,
	#[serde(rename = "ClntId")]
	pub clnt_id: Option<Vec<String>>,
	#[serde(rename = "NotRptd")]
	pub not_rptd: Option<String>,
}


// TradePartyIdentificationQuery9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradePartyIdentificationQuery9 {
	#[serde(rename = "LEI")]
	pub lei: Option<Vec<String>>,
	#[serde(rename = "CtryCd")]
	pub ctry_cd: Option<Vec<String>>,
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<Vec<String>>,
	#[serde(rename = "ClntId")]
	pub clnt_id: Option<Vec<String>>,
	#[serde(rename = "NotRptd")]
	pub not_rptd: Option<String>,
}


// TradePartyQueryCriteria5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradePartyQueryCriteria5 {
	#[serde(rename = "Oprtr")]
	pub oprtr: String,
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: Option<TradePartyIdentificationQuery8>,
	#[serde(rename = "RptgCtrPtyBrnch")]
	pub rptg_ctr_pty_brnch: Option<TradePartyIdentificationQuery9>,
	#[serde(rename = "OthrCtrPty")]
	pub othr_ctr_pty: Option<TradePartyIdentificationQuery8>,
	#[serde(rename = "OthrCtrPtyBrnch")]
	pub othr_ctr_pty_brnch: Option<TradePartyIdentificationQuery9>,
	#[serde(rename = "Bnfcry")]
	pub bnfcry: Option<TradePartyIdentificationQuery8>,
	#[serde(rename = "SubmitgAgt")]
	pub submitg_agt: Option<TradePartyIdentificationQuery8>,
	#[serde(rename = "Brkr")]
	pub brkr: Option<TradePartyIdentificationQuery8>,
	#[serde(rename = "CCP")]
	pub ccp: Option<TradePartyIdentificationQuery8>,
	#[serde(rename = "AgtLndr")]
	pub agt_lndr: Option<TradePartyIdentificationQuery8>,
	#[serde(rename = "TrptyAgt")]
	pub trpty_agt: Option<TradePartyIdentificationQuery8>,
}


// TradeQueryCriteria10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeQueryCriteria10 {
	#[serde(rename = "TradLifeCyclHstry")]
	pub trad_life_cycl_hstry: bool,
	#[serde(rename = "OutsdngTradInd")]
	pub outsdng_trad_ind: bool,
	#[serde(rename = "TradPtyCrit")]
	pub trad_pty_crit: Option<TradePartyQueryCriteria5>,
	#[serde(rename = "TradTpCrit")]
	pub trad_tp_crit: Option<TradeTypeQueryCriteria2>,
	#[serde(rename = "TmCrit")]
	pub tm_crit: Option<TradeDateTimeQueryCriteria2>,
	#[serde(rename = "OthrCrit")]
	pub othr_crit: Option<TradeAdditionalQueryCriteria7>,
}


// TradeQueryExecutionFrequency3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeQueryExecutionFrequency3 {
	#[serde(rename = "FrqcyTp")]
	pub frqcy_tp: String,
	#[serde(rename = "DlvryDay")]
	pub dlvry_day: Option<Vec<String>>,
	#[serde(rename = "DayOfMnth")]
	pub day_of_mnth: Option<Vec<f64>>,
}


// TradeRecurrentQuery5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeRecurrentQuery5 {
	#[serde(rename = "QryTp")]
	pub qry_tp: String,
	#[serde(rename = "Frqcy")]
	pub frqcy: TradeQueryExecutionFrequency3,
	#[serde(rename = "VldUntil")]
	pub vld_until: String,
}


// TradeReportQuery13Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeReportQuery13Choice {
	#[serde(rename = "AdHocQry")]
	pub ad_hoc_qry: Option<TradeQueryCriteria10>,
	#[serde(rename = "RcrntQry")]
	pub rcrnt_qry: Option<TradeRecurrentQuery5>,
}


// TradeTypeQueryCriteria2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeTypeQueryCriteria2 {
	#[serde(rename = "Oprtr")]
	pub oprtr: String,
	#[serde(rename = "SctiesFincgTxTp")]
	pub scties_fincg_tx_tp: Option<Vec<String>>,
	#[serde(rename = "CollCmpntTp")]
	pub coll_cmpnt_tp: Option<Vec<String>>,
}


// TransactionOperationType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionOperationType6Code {
	#[serde(rename = "TransactionOperationType6Code")]
	pub transaction_operation_type6_code: String,
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}


// WeekDay3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct WeekDay3Code {
	#[serde(rename = "WeekDay3Code")]
	pub week_day3_code: String,
}
