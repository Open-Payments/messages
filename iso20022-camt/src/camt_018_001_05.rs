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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// BusinessDayCriteria2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BusinessDayCriteria2 {
	#[serde(rename = "NewQryNm")]
	pub new_qry_nm: Option<String>,
	#[serde(rename = "SchCrit")]
	pub sch_crit: Option<Vec<BusinessDaySearchCriteria2>>,
	#[serde(rename = "RtrCrit")]
	pub rtr_crit: Option<BusinessDayReturnCriteria2>,
}


// BusinessDayCriteria3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BusinessDayCriteria3Choice {
	#[serde(rename = "QryNm")]
	pub qry_nm: Option<String>,
	#[serde(rename = "NewCrit")]
	pub new_crit: Option<BusinessDayCriteria2>,
}


// BusinessDayQuery2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BusinessDayQuery2 {
	#[serde(rename = "QryTp")]
	pub qry_tp: Option<String>,
	#[serde(rename = "Crit")]
	pub crit: Option<BusinessDayCriteria3Choice>,
}


// BusinessDayReturnCriteria2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BusinessDayReturnCriteria2 {
	#[serde(rename = "SysDtInd")]
	pub sys_dt_ind: Option<bool>,
	#[serde(rename = "SysStsInd")]
	pub sys_sts_ind: Option<bool>,
	#[serde(rename = "SysCcyInd")]
	pub sys_ccy_ind: Option<bool>,
	#[serde(rename = "ClsrPrdInd")]
	pub clsr_prd_ind: Option<bool>,
	#[serde(rename = "EvtInd")]
	pub evt_ind: Option<bool>,
	#[serde(rename = "SsnPrdInd")]
	pub ssn_prd_ind: Option<bool>,
	#[serde(rename = "EvtTpInd")]
	pub evt_tp_ind: Option<bool>,
}


// BusinessDaySearchCriteria2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BusinessDaySearchCriteria2 {
	#[serde(rename = "SysDt")]
	pub sys_dt: Option<String>,
	#[serde(rename = "SysId")]
	pub sys_id: Option<Vec<SystemIdentification2Choice>>,
	#[serde(rename = "SysCcy")]
	pub sys_ccy: Option<Vec<String>>,
	#[serde(rename = "EvtTp")]
	pub evt_tp: Option<SystemEventType2Choice>,
	#[serde(rename = "ClsrPrd")]
	pub clsr_prd: Option<DateTimePeriod1Choice>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// DateTimePeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod1 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: String,
}


// DateTimePeriod1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod1Choice {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: Option<String>,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: Option<String>,
	#[serde(rename = "DtTmRg")]
	pub dt_tm_rg: Option<DateTimePeriod1>,
}


// ExternalEnquiryRequestType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalEnquiryRequestType1Code {
	#[serde(rename = "ExternalEnquiryRequestType1Code")]
	pub external_enquiry_request_type1_code: String,
}


// ExternalMarketInfrastructure1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalMarketInfrastructure1Code {
	#[serde(rename = "ExternalMarketInfrastructure1Code")]
	pub external_market_infrastructure1_code: String,
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


// GetBusinessDayInformationV05 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GetBusinessDayInformationV05 {
	#[serde(rename = "MsgHdr")]
	pub msg_hdr: MessageHeader9,
	#[serde(rename = "BizDayInfQryDef")]
	pub biz_day_inf_qry_def: Option<BusinessDayQuery2>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
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


// MarketInfrastructureIdentification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketInfrastructureIdentification1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
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


// MessageHeader9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader9 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: Option<String>,
	#[serde(rename = "ReqTp")]
	pub req_tp: Option<RequestType4Choice>,
}


// QueryType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct QueryType2Code {
	#[serde(rename = "QueryType2Code")]
	pub query_type2_code: String,
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


// RequestedIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestedIndicator {
	#[serde(rename = "RequestedIndicator")]
	pub requested_indicator: bool,
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


// SystemEventType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemEventType2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification1>,
}


// SystemEventType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemEventType2Code {
	#[serde(rename = "SystemEventType2Code")]
	pub system_event_type2_code: String,
}


// SystemIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemIdentification2Choice {
	#[serde(rename = "MktInfrstrctrId")]
	pub mkt_infrstrctr_id: Option<MarketInfrastructureIdentification1Choice>,
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
}
