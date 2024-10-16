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


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
}


// AnyMIC1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AnyMIC1Code {
	#[default]
	#[serde(rename = "ANYM")]
	CodeANYM,

}


// CollateralType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CollateralType6Code {
	#[default]
	#[serde(rename = "GBBK")]
	CodeGBBK,
	#[serde(rename = "BOND")]
	CodeBOND,
	#[serde(rename = "CASH")]
	CodeCASH,
	#[serde(rename = "COMM")]
	CodeCOMM,
	#[serde(rename = "INSU")]
	CodeINSU,
	#[serde(rename = "LCRE")]
	CodeLCRE,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "PHYS")]
	CodePHYS,
	#[serde(rename = "SECU")]
	CodeSECU,
	#[serde(rename = "STCF")]
	CodeSTCF,

}


// CorporateSectorCriteria5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CorporateSectorCriteria5 {
	#[serde(rename = "FISctr", skip_serializing_if = "Option::is_none")]
	pub fi_sctr: Option<Vec<FinancialPartySectorType2Code>>,
	#[serde(rename = "NFISctr", skip_serializing_if = "Option::is_none")]
	pub nfi_sctr: Option<Vec<NACEDomainIdentifier>>,
	#[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
	pub not_rptd: Option<NotReported1Code>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// DateOrBlankQuery2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateOrBlankQuery2Choice {
	#[serde(rename = "Rg", skip_serializing_if = "Option::is_none")]
	pub rg: Option<DatePeriod1>,
	#[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
	pub not_rptd: Option<NotReported1Code>,
}


// DatePeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriod1 {
	#[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
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
	#[serde(rename = "$value")]
	pub day_of_month_number: f64,
}


// ExposureType10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ExposureType10Code {
	#[default]
	#[serde(rename = "SBSC")]
	CodeSBSC,
	#[serde(rename = "MGLD")]
	CodeMGLD,
	#[serde(rename = "SLEB")]
	CodeSLEB,
	#[serde(rename = "REPO")]
	CodeREPO,

}


// FinancialPartySectorType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FinancialPartySectorType2Code {
	#[default]
	#[serde(rename = "AIFD")]
	CodeAIFD,
	#[serde(rename = "CSDS")]
	CodeCSDS,
	#[serde(rename = "CCPS")]
	CodeCCPS,
	#[serde(rename = "CDTI")]
	CodeCDTI,
	#[serde(rename = "INUN")]
	CodeINUN,
	#[serde(rename = "ORPI")]
	CodeORPI,
	#[serde(rename = "INVF")]
	CodeINVF,
	#[serde(rename = "REIN")]
	CodeREIN,
	#[serde(rename = "UCIT")]
	CodeUCIT,

}


// Frequency14Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Frequency14Code {
	#[default]
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "ADHO")]
	CodeADHO,

}


// GenericIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
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


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max16Text {
	#[serde(rename = "$value")]
	pub max16_text: String,
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


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
}


// NACEDomainIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NACEDomainIdentifier {
	#[serde(rename = "$value")]
	pub nace_domain_identifier: String,
}


// NameAndAddress5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<PostalAddress1>,
}


// NotReported1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NotReported1Code {
	#[default]
	#[serde(rename = "NORP")]
	CodeNORP,

}


// Operation3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Operation3Code {
	#[default]
	#[serde(rename = "ANDD")]
	CodeANDD,
	#[serde(rename = "ORRR")]
	CodeORRR,

}


// PartyIdentification121Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification121Choice {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
	#[serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none")]
	pub lgl_ntty_idr: Option<LEIIdentifier>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification1>,
}


// PartyNatureType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PartyNatureType1Code {
	#[default]
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "NFIN")]
	CodeNFIN,
	#[serde(rename = "FIIN")]
	CodeFIIN,
	#[serde(rename = "CCPS")]
	CodeCCPS,

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


// SecuritiesFinancingReportingTransactionQueryV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesFinancingReportingTransactionQueryV02 {
	#[serde(rename = "RqstngAuthrty")]
	pub rqstng_authrty: PartyIdentification121Choice,
	#[serde(rename = "TradQryData")]
	pub trad_qry_data: TradeReportQuery13Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// SecuritiesTradeVenueCriteria1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTradeVenueCriteria1Choice {
	#[serde(rename = "MIC", skip_serializing_if = "Option::is_none")]
	pub mic: Option<Vec<MICIdentifier>>,
	#[serde(rename = "AnyMIC", skip_serializing_if = "Option::is_none")]
	pub any_mic: Option<AnyMIC1Code>,
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


// TradeAdditionalQueryCriteria7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeAdditionalQueryCriteria7 {
	#[serde(rename = "ActnTp", skip_serializing_if = "Option::is_none")]
	pub actn_tp: Option<Vec<TransactionOperationType6Code>>,
	#[serde(rename = "ExctnVn", skip_serializing_if = "Option::is_none")]
	pub exctn_vn: Option<SecuritiesTradeVenueCriteria1Choice>,
	#[serde(rename = "NtrOfCtrPty", skip_serializing_if = "Option::is_none")]
	pub ntr_of_ctr_pty: Option<Vec<PartyNatureType1Code>>,
	#[serde(rename = "CorpSctr", skip_serializing_if = "Option::is_none")]
	pub corp_sctr: Option<Vec<CorporateSectorCriteria5>>,
}


// TradeDateTimeQueryCriteria2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeDateTimeQueryCriteria2 {
	#[serde(rename = "RptgDtTm", skip_serializing_if = "Option::is_none")]
	pub rptg_dt_tm: Option<DateTimePeriod1>,
	#[serde(rename = "ExctnDtTm", skip_serializing_if = "Option::is_none")]
	pub exctn_dt_tm: Option<DateTimePeriod1>,
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<DateOrBlankQuery2Choice>,
	#[serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none")]
	pub termntn_dt: Option<DateOrBlankQuery2Choice>,
}


// TradePartyIdentificationQuery8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradePartyIdentificationQuery8 {
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<Vec<LEIIdentifier>>,
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<Vec<AnyBICDec2014Identifier>>,
	#[serde(rename = "ClntId", skip_serializing_if = "Option::is_none")]
	pub clnt_id: Option<Vec<Max50Text>>,
	#[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
	pub not_rptd: Option<NotReported1Code>,
}


// TradePartyIdentificationQuery9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradePartyIdentificationQuery9 {
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<Vec<LEIIdentifier>>,
	#[serde(rename = "CtryCd", skip_serializing_if = "Option::is_none")]
	pub ctry_cd: Option<Vec<CountryCode>>,
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<Vec<AnyBICDec2014Identifier>>,
	#[serde(rename = "ClntId", skip_serializing_if = "Option::is_none")]
	pub clnt_id: Option<Vec<Max50Text>>,
	#[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
	pub not_rptd: Option<NotReported1Code>,
}


// TradePartyQueryCriteria5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradePartyQueryCriteria5 {
	#[serde(rename = "Oprtr")]
	pub oprtr: Operation3Code,
	#[serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none")]
	pub rptg_ctr_pty: Option<TradePartyIdentificationQuery8>,
	#[serde(rename = "RptgCtrPtyBrnch", skip_serializing_if = "Option::is_none")]
	pub rptg_ctr_pty_brnch: Option<TradePartyIdentificationQuery9>,
	#[serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none")]
	pub othr_ctr_pty: Option<TradePartyIdentificationQuery8>,
	#[serde(rename = "OthrCtrPtyBrnch", skip_serializing_if = "Option::is_none")]
	pub othr_ctr_pty_brnch: Option<TradePartyIdentificationQuery9>,
	#[serde(rename = "Bnfcry", skip_serializing_if = "Option::is_none")]
	pub bnfcry: Option<TradePartyIdentificationQuery8>,
	#[serde(rename = "SubmitgAgt", skip_serializing_if = "Option::is_none")]
	pub submitg_agt: Option<TradePartyIdentificationQuery8>,
	#[serde(rename = "Brkr", skip_serializing_if = "Option::is_none")]
	pub brkr: Option<TradePartyIdentificationQuery8>,
	#[serde(rename = "CCP", skip_serializing_if = "Option::is_none")]
	pub ccp: Option<TradePartyIdentificationQuery8>,
	#[serde(rename = "AgtLndr", skip_serializing_if = "Option::is_none")]
	pub agt_lndr: Option<TradePartyIdentificationQuery8>,
	#[serde(rename = "TrptyAgt", skip_serializing_if = "Option::is_none")]
	pub trpty_agt: Option<TradePartyIdentificationQuery8>,
}


// TradeQueryCriteria10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeQueryCriteria10 {
	#[serde(rename = "TradLifeCyclHstry")]
	pub trad_life_cycl_hstry: bool,
	#[serde(rename = "OutsdngTradInd")]
	pub outsdng_trad_ind: bool,
	#[serde(rename = "TradPtyCrit", skip_serializing_if = "Option::is_none")]
	pub trad_pty_crit: Option<TradePartyQueryCriteria5>,
	#[serde(rename = "TradTpCrit", skip_serializing_if = "Option::is_none")]
	pub trad_tp_crit: Option<TradeTypeQueryCriteria2>,
	#[serde(rename = "TmCrit", skip_serializing_if = "Option::is_none")]
	pub tm_crit: Option<TradeDateTimeQueryCriteria2>,
	#[serde(rename = "OthrCrit", skip_serializing_if = "Option::is_none")]
	pub othr_crit: Option<TradeAdditionalQueryCriteria7>,
}


// TradeQueryExecutionFrequency3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeQueryExecutionFrequency3 {
	#[serde(rename = "FrqcyTp")]
	pub frqcy_tp: Frequency14Code,
	#[serde(rename = "DlvryDay", skip_serializing_if = "Option::is_none")]
	pub dlvry_day: Option<Vec<WeekDay3Code>>,
	#[serde(rename = "DayOfMnth", skip_serializing_if = "Option::is_none")]
	pub day_of_mnth: Option<Vec<DayOfMonthNumber>>,
}


// TradeRecurrentQuery5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeRecurrentQuery5 {
	#[serde(rename = "QryTp")]
	pub qry_tp: Max1000Text,
	#[serde(rename = "Frqcy")]
	pub frqcy: TradeQueryExecutionFrequency3,
	#[serde(rename = "VldUntil")]
	pub vld_until: String,
}


// TradeReportQuery13Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeReportQuery13Choice {
	#[serde(rename = "AdHocQry", skip_serializing_if = "Option::is_none")]
	pub ad_hoc_qry: Option<TradeQueryCriteria10>,
	#[serde(rename = "RcrntQry", skip_serializing_if = "Option::is_none")]
	pub rcrnt_qry: Option<TradeRecurrentQuery5>,
}


// TradeTypeQueryCriteria2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeTypeQueryCriteria2 {
	#[serde(rename = "Oprtr")]
	pub oprtr: Operation3Code,
	#[serde(rename = "SctiesFincgTxTp", skip_serializing_if = "Option::is_none")]
	pub scties_fincg_tx_tp: Option<Vec<ExposureType10Code>>,
	#[serde(rename = "CollCmpntTp", skip_serializing_if = "Option::is_none")]
	pub coll_cmpnt_tp: Option<Vec<CollateralType6Code>>,
}


// TransactionOperationType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TransactionOperationType6Code {
	#[default]
	#[serde(rename = "REUU")]
	CodeREUU,
	#[serde(rename = "COLU")]
	CodeCOLU,
	#[serde(rename = "CORR")]
	CodeCORR,
	#[serde(rename = "ETRM")]
	CodeETRM,
	#[serde(rename = "VALU")]
	CodeVALU,
	#[serde(rename = "POSC")]
	CodePOSC,
	#[serde(rename = "NEWT")]
	CodeNEWT,
	#[serde(rename = "MODI")]
	CodeMODI,
	#[serde(rename = "MARU")]
	CodeMARU,
	#[serde(rename = "EROR")]
	CodeEROR,

}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}


// WeekDay3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum WeekDay3Code {
	#[default]
	#[serde(rename = "ALLD")]
	CodeALLD,
	#[serde(rename = "XBHL")]
	CodeXBHL,
	#[serde(rename = "IBHL")]
	CodeIBHL,
	#[serde(rename = "FRID")]
	CodeFRID,
	#[serde(rename = "MOND")]
	CodeMOND,
	#[serde(rename = "SATD")]
	CodeSATD,
	#[serde(rename = "SUND")]
	CodeSUND,
	#[serde(rename = "THUD")]
	CodeTHUD,
	#[serde(rename = "TUED")]
	CodeTUED,
	#[serde(rename = "WEDD")]
	CodeWEDD,
	#[serde(rename = "WDAY")]
	CodeWDAY,
	#[serde(rename = "WEND")]
	CodeWEND,

}
