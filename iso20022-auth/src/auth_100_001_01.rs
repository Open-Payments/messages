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
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// Contact9 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Contact9 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "PhneNb")]
	pub phne_nb: String,
	#[serde(rename = "EmailAdr")]
	pub email_adr: String,
	#[serde(rename = "Fctn")]
	pub fctn: Option<String>,
}


// CountryCode ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// DatePeriod2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriod2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// ExternalFinancialInstrumentIdentificationType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalFinancialInstrumentIdentificationType1Code {
	#[serde(rename = "ExternalFinancialInstrumentIdentificationType1Code")]
	pub external_financial_instrument_identification_type1_code: String,
}


// ISINOct2015Identifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "ISINOct2015Identifier")]
	pub isin_oct2015_identifier: String,
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


// IdentificationSource3Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IdentificationSource3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// LEIIdentifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// Max140Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max16Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max16Text {
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
}


// Max2048Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max2048Text {
	#[serde(rename = "Max2048Text")]
	pub max2048_text: String,
}


// Max20PositiveDecimalNumber ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max20PositiveDecimalNumber {
	#[serde(rename = "Max20PositiveDecimalNumber")]
	pub max20_positive_decimal_number: f64,
}


// Max20PositiveNumber ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max20PositiveNumber {
	#[serde(rename = "Max20PositiveNumber")]
	pub max20_positive_number: f64,
}


// Max256Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max256Text {
	#[serde(rename = "Max256Text")]
	pub max256_text: String,
}


// Max2Fraction1NonNegativeNumber ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max2Fraction1NonNegativeNumber {
	#[serde(rename = "Max2Fraction1NonNegativeNumber")]
	pub max2_fraction1_non_negative_number: f64,
}


// Max2NumericText ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max2NumericText {
	#[serde(rename = "Max2NumericText")]
	pub max2_numeric_text: String,
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


// OtherIdentification1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Sfx")]
	pub sfx: Option<String>,
	#[serde(rename = "Tp")]
	pub tp: IdentificationSource3Choice,
}


// PercentageRate ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// PhoneNumber ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PhoneNumber {
	#[serde(rename = "PhoneNumber")]
	pub phone_number: String,
}


// ReportPeriodActivity1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportPeriodActivity1Code {
	#[serde(rename = "ReportPeriodActivity1Code")]
	pub report_period_activity1_code: String,
}


// SecuritiesSettlementSystemIdentification2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesSettlementSystemIdentification2 {
	#[serde(rename = "SysId")]
	pub sys_id: String,
	#[serde(rename = "SysNm")]
	pub sys_nm: Option<String>,
	#[serde(rename = "CtryOfJursdctn")]
	pub ctry_of_jursdctn: Option<String>,
	#[serde(rename = "CSDLglNm")]
	pub csd_lgl_nm: Option<String>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "RspnsblPty")]
	pub rspnsbl_pty: Option<Vec<Contact9>>,
}


// SecurityIdentification19 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification19 {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[serde(rename = "OthrId")]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// SettlementDailyFailureReason1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementDailyFailureReason1Choice {
	#[serde(rename = "DataSetActn")]
	pub data_set_actn: Option<String>,
	#[serde(rename = "Data")]
	pub data: Option<SettlementDailyFailureReason3>,
}


// SettlementDailyFailureReason3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementDailyFailureReason3 {
	#[serde(rename = "FaildScties")]
	pub faild_scties: SettlementTotalData1Choice,
	#[serde(rename = "FaildCsh")]
	pub faild_csh: SettlementTotalData1Choice,
}


// SettlementDataRate2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementDataRate2 {
	#[serde(rename = "Vol")]
	pub vol: f64,
	#[serde(rename = "Val")]
	pub val: f64,
}


// SettlementDataVolume2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementDataVolume2 {
	#[serde(rename = "Vol")]
	pub vol: f64,
	#[serde(rename = "Val")]
	pub val: f64,
}


// SettlementFailsCurrency2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsCurrency2 {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "Data")]
	pub data: SettlementTotalData1,
}


// SettlementFailsDailyCSD1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsDailyCSD1Choice {
	#[serde(rename = "DataSetActn")]
	pub data_set_actn: Option<String>,
	#[serde(rename = "Data")]
	pub data: Option<SettlementFailsDailyCSD3>,
}


// SettlementFailsDailyCSD3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsDailyCSD3 {
	#[serde(rename = "IntraCSD")]
	pub intra_csd: SettlementFailsDailyInstructionType1Choice,
	#[serde(rename = "CrossCSD")]
	pub cross_csd: SettlementFailsDailyInstructionType1Choice,
}


// SettlementFailsDailyData3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsDailyData3 {
	#[serde(rename = "RptgDt")]
	pub rptg_dt: String,
	#[serde(rename = "DalyRcrd")]
	pub daly_rcrd: SettlementFailsDailyInstrument3,
}


// SettlementFailsDailyInstructionType1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsDailyInstructionType1Choice {
	#[serde(rename = "DataSetActn")]
	pub data_set_actn: Option<String>,
	#[serde(rename = "Data")]
	pub data: Option<SettlementFailsDailyInstructionType3>,
}


// SettlementFailsDailyInstructionType3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsDailyInstructionType3 {
	#[serde(rename = "DlvryVrssPmt")]
	pub dlvry_vrss_pmt: SettlementDailyFailureReason1Choice,
	#[serde(rename = "DlvryWthPmt")]
	pub dlvry_wth_pmt: SettlementDailyFailureReason1Choice,
	#[serde(rename = "PmtFreeOfDlvry")]
	pub pmt_free_of_dlvry: SettlementDailyFailureReason1Choice,
	#[serde(rename = "FreeOfPmt")]
	pub free_of_pmt: SettlementDailyFailureReason1Choice,
}


// SettlementFailsDailyInstrument3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsDailyInstrument3 {
	#[serde(rename = "Eqty")]
	pub eqty: SettlementFailsDailyTransactionType1Choice,
	#[serde(rename = "SvrgnDebt")]
	pub svrgn_debt: SettlementFailsDailyTransactionType1Choice,
	#[serde(rename = "Bd")]
	pub bd: SettlementFailsDailyTransactionType1Choice,
	#[serde(rename = "OthrTrfblScties")]
	pub othr_trfbl_scties: SettlementFailsDailyTransactionType1Choice,
	#[serde(rename = "XchgTraddFnds")]
	pub xchg_tradd_fnds: SettlementFailsDailyTransactionType1Choice,
	#[serde(rename = "CllctvInvstmtUdrtkgs")]
	pub cllctv_invstmt_udrtkgs: SettlementFailsDailyTransactionType1Choice,
	#[serde(rename = "MnyMktInstrm")]
	pub mny_mkt_instrm: SettlementFailsDailyTransactionType1Choice,
	#[serde(rename = "EmssnAllwnc")]
	pub emssn_allwnc: SettlementFailsDailyTransactionType1Choice,
	#[serde(rename = "Othr")]
	pub othr: SettlementFailsDailyTransactionType1Choice,
}


// SettlementFailsDailyTransactionType1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsDailyTransactionType1Choice {
	#[serde(rename = "DataSetActn")]
	pub data_set_actn: Option<String>,
	#[serde(rename = "Data")]
	pub data: Option<SettlementFailsDailyTransactionType3>,
}


// SettlementFailsDailyTransactionType3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsDailyTransactionType3 {
	#[serde(rename = "SctiesBuyOrSell")]
	pub scties_buy_or_sell: SettlementFailsDailyCSD1Choice,
	#[serde(rename = "CollMgmtOpr")]
	pub coll_mgmt_opr: SettlementFailsDailyCSD1Choice,
	#[serde(rename = "SctiesLndgOrBrrwg")]
	pub scties_lndg_or_brrwg: SettlementFailsDailyCSD1Choice,
	#[serde(rename = "RpAgrmt")]
	pub rp_agrmt: SettlementFailsDailyCSD1Choice,
	#[serde(rename = "Othr")]
	pub othr: SettlementFailsDailyCSD1Choice,
}


// SettlementFailsData3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsData3 {
	#[serde(rename = "Ttl")]
	pub ttl: SettlementTotalData1,
	#[serde(rename = "PtcptInFail")]
	pub ptcpt_in_fail: Option<SettlementFailsParticipantRange1>,
	#[serde(rename = "FlsPerCcy")]
	pub fls_per_ccy: Option<Vec<SettlementFailsCurrency2>>,
	#[serde(rename = "FlsPerFinInstrmTp")]
	pub fls_per_fin_instrm_tp: Option<SettlementFailsInstrument2>,
	#[serde(rename = "SctiesInFail")]
	pub scties_in_fail: Option<SettlementFailsSecuritiesRange1>,
	#[serde(rename = "FlsPerTxTp")]
	pub fls_per_tx_tp: Option<SettlementFailsTransactionType2>,
	#[serde(rename = "TtlSttlmPnlties")]
	pub ttl_sttlm_pnlties: Option<SettlementDataVolume2>,
	#[serde(rename = "FailrRsn")]
	pub failr_rsn: SettlementFailureReason3,
}


// SettlementFailsInstrument2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsInstrument2 {
	#[serde(rename = "Eqty")]
	pub eqty: SettlementTotalData1Choice,
	#[serde(rename = "SvrgnDebt")]
	pub svrgn_debt: SettlementTotalData1Choice,
	#[serde(rename = "Bd")]
	pub bd: SettlementTotalData1Choice,
	#[serde(rename = "OthrTrfblScties")]
	pub othr_trfbl_scties: SettlementTotalData1Choice,
	#[serde(rename = "XchgTraddFnds")]
	pub xchg_tradd_fnds: SettlementTotalData1Choice,
	#[serde(rename = "CllctvInvstmtUdrtkgs")]
	pub cllctv_invstmt_udrtkgs: SettlementTotalData1Choice,
	#[serde(rename = "MnyMktInstrm")]
	pub mny_mkt_instrm: SettlementTotalData1Choice,
	#[serde(rename = "EmssnAllwnc")]
	pub emssn_allwnc: SettlementTotalData1Choice,
	#[serde(rename = "Othr")]
	pub othr: SettlementTotalData1Choice,
}


// SettlementFailsMonthlyReportV01 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsMonthlyReportV01 {
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: SettlementFailsReportHeader2,
	#[serde(rename = "MnthlyAggt")]
	pub mnthly_aggt: SettlementFailsData3,
	#[serde(rename = "DalyData")]
	pub daly_data: Vec<SettlementFailsDailyData3>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// SettlementFailsParticipant1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsParticipant1 {
	#[serde(rename = "LEI")]
	pub lei: String,
	#[serde(rename = "Rank")]
	pub rank: String,
	#[serde(rename = "Aggt")]
	pub aggt: SettlementTotalData1,
}


// SettlementFailsParticipantRange1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsParticipantRange1 {
	#[serde(rename = "HghstInVol")]
	pub hghst_in_vol: Vec<SettlementFailsParticipant1>,
	#[serde(rename = "HghstInVal")]
	pub hghst_in_val: Vec<SettlementFailsParticipant1>,
}


// SettlementFailsReportHeader2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsReportHeader2 {
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
	#[serde(rename = "RptgPrd")]
	pub rptg_prd: DatePeriod2,
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "RptSts")]
	pub rpt_sts: String,
	#[serde(rename = "SctiesSttlmSys")]
	pub scties_sttlm_sys: SecuritiesSettlementSystemIdentification2,
}


// SettlementFailsSecurities1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsSecurities1 {
	#[serde(rename = "FinInstrmId")]
	pub fin_instrm_id: SecurityIdentification19,
	#[serde(rename = "Rank")]
	pub rank: String,
}


// SettlementFailsSecuritiesRange1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsSecuritiesRange1 {
	#[serde(rename = "HghstInVol")]
	pub hghst_in_vol: Vec<SettlementFailsSecurities1>,
	#[serde(rename = "HghstInVal")]
	pub hghst_in_val: Vec<SettlementFailsSecurities1>,
}


// SettlementFailsTransactionType2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsTransactionType2 {
	#[serde(rename = "SctiesBuyOrSell")]
	pub scties_buy_or_sell: SettlementTotalData1Choice,
	#[serde(rename = "CollMgmtOpr")]
	pub coll_mgmt_opr: SettlementTotalData1Choice,
	#[serde(rename = "SctiesLndgOrBrrwg")]
	pub scties_lndg_or_brrwg: SettlementTotalData1Choice,
	#[serde(rename = "RpAgrmt")]
	pub rp_agrmt: SettlementTotalData1Choice,
	#[serde(rename = "Othr")]
	pub othr: SettlementTotalData1Choice,
}


// SettlementFailureReason2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailureReason2 {
	#[serde(rename = "MainRsns")]
	pub main_rsns: String,
	#[serde(rename = "EffcncyImprvmt")]
	pub effcncy_imprvmt: String,
}


// SettlementFailureReason3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailureReason3 {
	#[serde(rename = "AvrgDrtn")]
	pub avrg_drtn: Option<f64>,
	#[serde(rename = "Desc")]
	pub desc: Vec<SettlementFailureReason2>,
}


// SettlementTotalData1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementTotalData1 {
	#[serde(rename = "Sttld")]
	pub sttld: SettlementDataVolume2,
	#[serde(rename = "Faild")]
	pub faild: SettlementDataVolume2,
	#[serde(rename = "Ttl")]
	pub ttl: SettlementDataVolume2,
	#[serde(rename = "FaildRate")]
	pub faild_rate: SettlementDataRate2,
}


// SettlementTotalData1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementTotalData1Choice {
	#[serde(rename = "DataSetActn")]
	pub data_set_actn: Option<String>,
	#[serde(rename = "Data")]
	pub data: Option<SettlementTotalData1>,
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


// TransactionOperationType4Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionOperationType4Code {
	#[serde(rename = "TransactionOperationType4Code")]
	pub transaction_operation_type4_code: String,
}