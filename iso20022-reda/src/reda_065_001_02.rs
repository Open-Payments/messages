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


// CalendarData1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CalendarData1 {
	#[serde(rename = "SysDt")]
	pub sys_dt: String,
	#[serde(rename = "SysSts")]
	pub sys_sts: SystemStatus3Choice,
}


// CalendarOrBusinessError1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CalendarOrBusinessError1Choice {
	#[serde(rename = "CalData")]
	pub cal_data: Option<Vec<CalendarData1>>,
	#[serde(rename = "BizErr")]
	pub biz_err: Option<Vec<ErrorHandling4>>,
}


// CalendarReport1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CalendarReport1 {
	#[serde(rename = "Svc")]
	pub svc: Option<SystemAndCurrency1>,
	#[serde(rename = "CalOrErr")]
	pub cal_or_err: CalendarOrBusinessError1Choice,
}


// CalendarReportOrError1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CalendarReportOrError1Choice {
	#[serde(rename = "CalRpt")]
	pub cal_rpt: Option<CalendarReport1>,
	#[serde(rename = "OprlErr")]
	pub oprl_err: Option<Vec<ErrorHandling4>>,
}


// CalendarReportV02 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CalendarReportV02 {
	#[serde(rename = "MsgHdr")]
	pub msg_hdr: MessageHeader11,
	#[serde(rename = "RptOrErr")]
	pub rpt_or_err: CalendarReportOrError1Choice,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// CountryCode ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// ErrorHandling1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling1Code {
	#[serde(rename = "ErrorHandling1Code")]
	pub error_handling1_code: String,
}


// ErrorHandling2Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ErrorHandling4 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling4 {
	#[serde(rename = "Err")]
	pub err: ErrorHandling2Choice,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// ExternalEnquiryRequestType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalEnquiryRequestType1Code {
	#[serde(rename = "ExternalEnquiryRequestType1Code")]
	pub external_enquiry_request_type1_code: String,
}


// ExternalMarketInfrastructure1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalMarketInfrastructure1Code {
	#[serde(rename = "ExternalMarketInfrastructure1Code")]
	pub external_market_infrastructure1_code: String,
}


// ExternalPaymentControlRequestType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPaymentControlRequestType1Code {
	#[serde(rename = "ExternalPaymentControlRequestType1Code")]
	pub external_payment_control_request_type1_code: String,
}


// GenericIdentification1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification1 {
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


// MarketInfrastructureIdentification1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketInfrastructureIdentification1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
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


// MessageHeader11 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader11 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: Option<String>,
	#[serde(rename = "ReqTp")]
	pub req_tp: Option<RequestType4Choice>,
	#[serde(rename = "OrgnlBizQry")]
	pub orgnl_biz_qry: Option<OriginalBusinessQuery1>,
}


// OriginalBusinessQuery1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OriginalBusinessQuery1 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "MsgNmId")]
	pub msg_nm_id: Option<String>,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: Option<String>,
}


// RequestType4Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestType4Choice {
	#[serde(rename = "PmtCtrl")]
	pub pmt_ctrl: Option<String>,
	#[serde(rename = "Enqry")]
	pub enqry: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification1>,
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


// SystemAndCurrency1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemAndCurrency1 {
	#[serde(rename = "SysId")]
	pub sys_id: SystemIdentification2Choice,
	#[serde(rename = "SysCcy")]
	pub sys_ccy: Option<String>,
}


// SystemIdentification2Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemIdentification2Choice {
	#[serde(rename = "MktInfrstrctrId")]
	pub mkt_infrstrctr_id: Option<MarketInfrastructureIdentification1Choice>,
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
}


// SystemStatus3Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemStatus3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification1>,
}


// SystemStatus3Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemStatus3Code {
	#[serde(rename = "SystemStatus3Code")]
	pub system_status3_code: String,
}
