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


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "ActiveOrHistoricCurrencyCode")]
	pub active_or_historic_currency_code: String,
}


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BaseOneRate {
	#[serde(rename = "BaseOneRate")]
	pub base_one_rate: f64,
}


// CurrencyExchange20 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CurrencyExchange20 {
	#[serde(rename = "XchgRate")]
	pub xchg_rate: f64,
	#[serde(rename = "QtdCcy")]
	pub qtd_ccy: String,
	#[serde(rename = "QtnDt")]
	pub qtn_dt: String,
	#[serde(rename = "LwLmt")]
	pub lw_lmt: Option<ExchangeRateOrPercentage1Choice>,
	#[serde(rename = "HghLmt")]
	pub hgh_lmt: Option<ExchangeRateOrPercentage1Choice>,
}


// CurrencyExchangeReport4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CurrencyExchangeReport4 {
	#[serde(rename = "CcyRef")]
	pub ccy_ref: CurrencySourceTarget1,
	#[serde(rename = "CcyXchgOrErr")]
	pub ccy_xchg_or_err: ExchangeRateReportOrError4Choice,
}


// CurrencySourceTarget1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CurrencySourceTarget1 {
	#[serde(rename = "SrcCcy")]
	pub src_ccy: String,
	#[serde(rename = "TrgtCcy")]
	pub trgt_ccy: String,
}


// ErrorHandling1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ErrorHandling1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling1Code {
	#[serde(rename = "ErrorHandling1Code")]
	pub error_handling1_code: String,
}


// ErrorHandling3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling3 {
	#[serde(rename = "Err")]
	pub err: ErrorHandling1Choice,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// ExchangeRateOrPercentage1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExchangeRateOrPercentage1Choice {
	#[serde(rename = "Rate")]
	pub rate: Option<f64>,
	#[serde(rename = "Pctg")]
	pub pctg: Option<f64>,
}


// ExchangeRateReportOrError3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExchangeRateReportOrError3Choice {
	#[serde(rename = "CcyXchgRpt")]
	pub ccy_xchg_rpt: Option<Vec<CurrencyExchangeReport4>>,
	#[serde(rename = "OprlErr")]
	pub oprl_err: Option<Vec<ErrorHandling3>>,
}


// ExchangeRateReportOrError4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExchangeRateReportOrError4Choice {
	#[serde(rename = "BizErr")]
	pub biz_err: Option<Vec<ErrorHandling3>>,
	#[serde(rename = "CcyXchg")]
	pub ccy_xchg: Option<CurrencyExchange20>,
}


// ExternalEnquiryRequestType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalEnquiryRequestType1Code {
	#[serde(rename = "ExternalEnquiryRequestType1Code")]
	pub external_enquiry_request_type1_code: String,
}


// ExternalPaymentControlRequestType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPaymentControlRequestType1Code {
	#[serde(rename = "ExternalPaymentControlRequestType1Code")]
	pub external_payment_control_request_type1_code: String,
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


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
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


// Max4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "Max4AlphaNumericText")]
	pub max4_alpha_numeric_text: String,
}


// MessageHeader7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader7 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: Option<String>,
	#[serde(rename = "ReqTp")]
	pub req_tp: Option<RequestType4Choice>,
	#[serde(rename = "OrgnlBizQry")]
	pub orgnl_biz_qry: Option<OriginalBusinessQuery1>,
	#[serde(rename = "QryNm")]
	pub qry_nm: Option<String>,
}


// OriginalBusinessQuery1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OriginalBusinessQuery1 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "MsgNmId")]
	pub msg_nm_id: Option<String>,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: Option<String>,
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// RequestType4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestType4Choice {
	#[serde(rename = "PmtCtrl")]
	pub pmt_ctrl: Option<String>,
	#[serde(rename = "Enqry")]
	pub enqry: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification1>,
}


// ReturnCurrencyExchangeRateV05 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReturnCurrencyExchangeRateV05 {
	#[serde(rename = "MsgHdr")]
	pub msg_hdr: MessageHeader7,
	#[serde(rename = "RptOrErr")]
	pub rpt_or_err: ExchangeRateReportOrError3Choice,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
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
