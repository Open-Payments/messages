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
use serde_valid::Validate;


// ActiveCurrencyCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[validate(pattern = "[A-Z]{3,3}")]
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// Contact9 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
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
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CountryCode {
	#[validate(pattern = "[A-Z]{2,2}")]
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// DatePeriod2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DatePeriod2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// ISODate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// LEIIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[validate(pattern = "[A-Z0-9]{18,18}[0-9]{2,2}")]
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// Max140Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max140Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 140)]
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max2048Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max2048Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 2048)]
	#[serde(rename = "Max2048Text")]
	pub max2048_text: String,
}


// Max20PositiveDecimalNumber ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max20PositiveDecimalNumber {
	#[serde(rename = "Max20PositiveDecimalNumber")]
	pub max20_positive_decimal_number: f64,
}


// Max20PositiveNumber ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max20PositiveNumber {
	#[serde(rename = "Max20PositiveNumber")]
	pub max20_positive_number: f64,
}


// Max256Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max256Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 256)]
	#[serde(rename = "Max256Text")]
	pub max256_text: String,
}


// Max2Fraction1NonNegativeNumber ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max2Fraction1NonNegativeNumber {
	#[serde(rename = "Max2Fraction1NonNegativeNumber")]
	pub max2_fraction1_non_negative_number: f64,
}


// Max350Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max350Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 350)]
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max35Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 35)]
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// PercentageRate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// PhoneNumber ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PhoneNumber {
	#[validate(pattern = "\\+[0-9]{1,3}-[0-9()+\\-]{1,30}")]
	#[serde(rename = "PhoneNumber")]
	pub phone_number: String,
}


// SecuritiesSettlementSystemIdentification2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
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
	#[validate]
	#[serde(rename = "RspnsblPty")]
	pub rspnsbl_pty: Option<Vec<Contact9>>,
}


// SettlementDataRate1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementDataRate1Choice {
	#[serde(rename = "NbOfInstrs")]
	pub nb_of_instrs: Option<f64>,
	#[serde(rename = "ValOfInstrs")]
	pub val_of_instrs: Option<f64>,
}


// SettlementDataRate2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementDataRate2 {
	#[serde(rename = "Vol")]
	pub vol: f64,
	#[serde(rename = "Val")]
	pub val: f64,
}


// SettlementDataVolume2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementDataVolume2 {
	#[serde(rename = "Vol")]
	pub vol: f64,
	#[serde(rename = "Val")]
	pub val: f64,
}


// SettlementFailsAnnualReportV01 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementFailsAnnualReportV01 {
	#[validate]
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: SettlementFailsReportHeader2,
	#[validate]
	#[serde(rename = "AnlAggt")]
	pub anl_aggt: SettlementFailsData4,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// SettlementFailsData4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementFailsData4 {
	#[validate]
	#[serde(rename = "Ttl")]
	pub ttl: SettlementTotalData1,
	#[validate]
	#[serde(rename = "FailrRsn")]
	pub failr_rsn: SettlementFailureReason3,
	#[validate]
	#[serde(rename = "ElgblForDrgtn")]
	pub elgbl_for_drgtn: SettlementFailsDerogation1,
}


// SettlementFailsDerogation1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementFailsDerogation1 {
	#[serde(rename = "ElgbltyInd")]
	pub elgblty_ind: bool,
	#[validate]
	#[serde(rename = "Justfn")]
	pub justfn: Option<SettlementFailsJustification1>,
}


// SettlementFailsJustification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementFailsJustification1 {
	#[serde(rename = "Val")]
	pub val: f64,
	#[validate]
	#[serde(rename = "Rate")]
	pub rate: SettlementDataRate1Choice,
}


// SettlementFailsReportHeader2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementFailsReportHeader2 {
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
	#[validate]
	#[serde(rename = "RptgPrd")]
	pub rptg_prd: DatePeriod2,
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "RptSts")]
	pub rpt_sts: String,
	#[validate]
	#[serde(rename = "SctiesSttlmSys")]
	pub scties_sttlm_sys: SecuritiesSettlementSystemIdentification2,
}


// SettlementFailureReason2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementFailureReason2 {
	#[serde(rename = "MainRsns")]
	pub main_rsns: String,
	#[serde(rename = "EffcncyImprvmt")]
	pub effcncy_imprvmt: String,
}


// SettlementFailureReason3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementFailureReason3 {
	#[serde(rename = "AvrgDrtn")]
	pub avrg_drtn: Option<f64>,
	#[validate]
	#[serde(rename = "Desc")]
	pub desc: Vec<SettlementFailureReason2>,
}


// SettlementTotalData1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementTotalData1 {
	#[validate]
	#[serde(rename = "Sttld")]
	pub sttld: SettlementDataVolume2,
	#[validate]
	#[serde(rename = "Faild")]
	pub faild: SettlementDataVolume2,
	#[validate]
	#[serde(rename = "Ttl")]
	pub ttl: SettlementDataVolume2,
	#[validate]
	#[serde(rename = "FaildRate")]
	pub faild_rate: SettlementDataRate2,
}


// SupplementaryData1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Option<String>,
	#[validate]
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}


// TransactionOperationType4Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TransactionOperationType4Code {
	#[validate(enumerate = ["NEWT", "AMND", "CANC"])]
	#[serde(rename = "TransactionOperationType4Code")]
	pub transaction_operation_type4_code: String,
}


// TrueFalseIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}
