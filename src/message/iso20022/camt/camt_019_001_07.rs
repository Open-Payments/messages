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


// BusinessDay8 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BusinessDay8 {
	#[validate]
	#[serde(rename = "SysId")]
	pub sys_id: Vec<SystemIdentification2Choice>,
	#[validate]
	#[serde(rename = "BizDayOrErr")]
	pub biz_day_or_err: BusinessDayReportOrError10Choice,
}


// BusinessDay9 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BusinessDay9 {
	#[validate]
	#[serde(rename = "SysDt")]
	pub sys_dt: Option<DateAndDateTime2Choice>,
	#[validate]
	#[serde(rename = "SysSts")]
	pub sys_sts: Option<SystemStatus3>,
	#[validate]
	#[serde(rename = "SysInfPerCcy")]
	pub sys_inf_per_ccy: Option<Vec<SystemAvailabilityAndEvents3>>,
}


// BusinessDayReportOrError10Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BusinessDayReportOrError10Choice {
	#[validate]
	#[serde(rename = "BizDayInf")]
	pub biz_day_inf: Option<BusinessDay9>,
	#[validate]
	#[serde(rename = "BizErr")]
	pub biz_err: Option<Vec<ErrorHandling5>>,
}


// BusinessDayReportOrError9Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BusinessDayReportOrError9Choice {
	#[validate]
	#[serde(rename = "BizRpt")]
	pub biz_rpt: Option<Vec<BusinessDay8>>,
	#[validate]
	#[serde(rename = "OprlErr")]
	pub oprl_err: Option<Vec<ErrorHandling5>>,
}


// ClosureReason2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ClosureReason2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// CountryCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CountryCode {
	#[validate(pattern = "[A-Z]{2,2}")]
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// DateAndDateTime2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DateAndDateTime2Choice {
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm")]
	pub dt_tm: Option<String>,
}


// DateTimePeriod1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DateTimePeriod1 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: String,
}


// DateTimePeriod1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DateTimePeriod1Choice {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: Option<String>,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: Option<String>,
	#[validate]
	#[serde(rename = "DtTmRg")]
	pub dt_tm_rg: Option<DateTimePeriod1>,
}


// ErrorHandling3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ErrorHandling3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ErrorHandling5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ErrorHandling5 {
	#[validate]
	#[serde(rename = "Err")]
	pub err: ErrorHandling3Choice,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// ExternalEnquiryRequestType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalEnquiryRequestType1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalEnquiryRequestType1Code")]
	pub external_enquiry_request_type1_code: String,
}


// ExternalMarketInfrastructure1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalMarketInfrastructure1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 3)]
	#[serde(rename = "ExternalMarketInfrastructure1Code")]
	pub external_market_infrastructure1_code: String,
}


// ExternalPaymentControlRequestType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalPaymentControlRequestType1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalPaymentControlRequestType1Code")]
	pub external_payment_control_request_type1_code: String,
}


// ExternalSystemErrorHandling1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalSystemErrorHandling1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalSystemErrorHandling1Code")]
	pub external_system_error_handling1_code: String,
}


// ExternalSystemEventType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalSystemEventType1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalSystemEventType1Code")]
	pub external_system_event_type1_code: String,
}


// GenericIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
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


// ISOTime ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISOTime {
	#[serde(rename = "ISOTime")]
	pub iso_time: String,
}


// MarketInfrastructureIdentification1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MarketInfrastructureIdentification1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// Max140Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max140Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 140)]
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
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


// MessageHeader7 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MessageHeader7 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: Option<String>,
	#[validate]
	#[serde(rename = "ReqTp")]
	pub req_tp: Option<RequestType4Choice>,
	#[validate]
	#[serde(rename = "OrgnlBizQry")]
	pub orgnl_biz_qry: Option<OriginalBusinessQuery1>,
	#[serde(rename = "QryNm")]
	pub qry_nm: Option<String>,
}


// OriginalBusinessQuery1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OriginalBusinessQuery1 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "MsgNmId")]
	pub msg_nm_id: Option<String>,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: Option<String>,
}


// RequestType4Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RequestType4Choice {
	#[serde(rename = "PmtCtrl")]
	pub pmt_ctrl: Option<String>,
	#[serde(rename = "Enqry")]
	pub enqry: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification1>,
}


// ReturnBusinessDayInformationV07 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReturnBusinessDayInformationV07 {
	#[validate]
	#[serde(rename = "MsgHdr")]
	pub msg_hdr: MessageHeader7,
	#[validate]
	#[serde(rename = "RptOrErr")]
	pub rpt_or_err: BusinessDayReportOrError9Choice,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
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


// SystemAvailabilityAndEvents3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SystemAvailabilityAndEvents3 {
	#[serde(rename = "SysCcy")]
	pub sys_ccy: Option<String>,
	#[validate]
	#[serde(rename = "SsnPrd")]
	pub ssn_prd: Option<TimePeriod1>,
	#[validate]
	#[serde(rename = "Evt")]
	pub evt: Option<Vec<SystemEvent3>>,
	#[validate]
	#[serde(rename = "ClsrInf")]
	pub clsr_inf: Option<Vec<SystemClosure2>>,
}


// SystemClosure2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SystemClosure2 {
	#[validate]
	#[serde(rename = "Prd")]
	pub prd: Option<DateTimePeriod1Choice>,
	#[validate]
	#[serde(rename = "Rsn")]
	pub rsn: ClosureReason2Choice,
}


// SystemClosureReason1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SystemClosureReason1Code {
	#[validate(enumerate = ["BHOL", "SMTN", "NOOP", "RCVR", "ADTW"])]
	#[serde(rename = "SystemClosureReason1Code")]
	pub system_closure_reason1_code: String,
}


// SystemEvent3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SystemEvent3 {
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: SystemEventType4Choice,
	#[serde(rename = "SchdldTm")]
	pub schdld_tm: String,
	#[serde(rename = "FctvTm")]
	pub fctv_tm: Option<String>,
	#[serde(rename = "StartTm")]
	pub start_tm: Option<String>,
	#[serde(rename = "EndTm")]
	pub end_tm: Option<String>,
}


// SystemEventType4Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SystemEventType4Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification1>,
}


// SystemIdentification2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SystemIdentification2Choice {
	#[validate]
	#[serde(rename = "MktInfrstrctrId")]
	pub mkt_infrstrctr_id: Option<MarketInfrastructureIdentification1Choice>,
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
}


// SystemStatus2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SystemStatus2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification1>,
}


// SystemStatus2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SystemStatus2Code {
	#[validate(enumerate = ["SUSP", "ACTV", "CLSD", "CLSG"])]
	#[serde(rename = "SystemStatus2Code")]
	pub system_status2_code: String,
}


// SystemStatus3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SystemStatus3 {
	#[validate]
	#[serde(rename = "Sts")]
	pub sts: SystemStatus2Choice,
	#[validate]
	#[serde(rename = "VldtyTm")]
	pub vldty_tm: Option<DateTimePeriod1Choice>,
}


// TimePeriod1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TimePeriod1 {
	#[serde(rename = "FrTm")]
	pub fr_tm: String,
	#[serde(rename = "ToTm")]
	pub to_tm: String,
}
