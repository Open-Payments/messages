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
use regex::Regex;


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "$value")]
	pub active_currency_code: String,
}

impl ActiveCurrencyCode {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.active_currency_code) {
			return false
		}
		return true
	}
}


// CalendarData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CalendarData1 {
	#[serde(rename = "SysDt")]
	pub sys_dt: String,
	#[serde(rename = "SysSts")]
	pub sys_sts: SystemStatus3Choice,
}

impl CalendarData1 {
	pub fn validate(&self) -> bool {
		if !self.sys_sts.validate() { return false }
		return true
	}
}


// CalendarOrBusinessError1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CalendarOrBusinessError1Choice {
	#[serde(rename = "CalData", skip_serializing_if = "Option::is_none")]
	pub cal_data: Option<Vec<CalendarData1>>,
	#[serde(rename = "BizErr", skip_serializing_if = "Option::is_none")]
	pub biz_err: Option<Vec<ErrorHandling4>>,
}

impl CalendarOrBusinessError1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cal_data_vec) = self.cal_data { for item in cal_data_vec { if !item.validate() { return false; } } }
		if let Some(ref biz_err_vec) = self.biz_err { for item in biz_err_vec { if !item.validate() { return false; } } }
		return true
	}
}


// CalendarReport1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CalendarReport1 {
	#[serde(rename = "Svc", skip_serializing_if = "Option::is_none")]
	pub svc: Option<SystemAndCurrency1>,
	#[serde(rename = "CalOrErr")]
	pub cal_or_err: CalendarOrBusinessError1Choice,
}

impl CalendarReport1 {
	pub fn validate(&self) -> bool {
		if let Some(ref svc_value) = self.svc { if !svc_value.validate() { return false; } }
		if !self.cal_or_err.validate() { return false }
		return true
	}
}


// CalendarReportOrError1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CalendarReportOrError1Choice {
	#[serde(rename = "CalRpt", skip_serializing_if = "Option::is_none")]
	pub cal_rpt: Option<CalendarReport1>,
	#[serde(rename = "OprlErr", skip_serializing_if = "Option::is_none")]
	pub oprl_err: Option<Vec<ErrorHandling4>>,
}

impl CalendarReportOrError1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cal_rpt_value) = self.cal_rpt { if !cal_rpt_value.validate() { return false; } }
		if let Some(ref oprl_err_vec) = self.oprl_err { for item in oprl_err_vec { if !item.validate() { return false; } } }
		return true
	}
}


// CalendarReportV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CalendarReportV02 {
	#[serde(rename = "MsgHdr")]
	pub msg_hdr: MessageHeader11,
	#[serde(rename = "RptOrErr")]
	pub rpt_or_err: CalendarReportOrError1Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CalendarReportV02 {
	pub fn validate(&self) -> bool {
		if !self.msg_hdr.validate() { return false }
		if !self.rpt_or_err.validate() { return false }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
	}
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}

impl CountryCode {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.country_code) {
			return false
		}
		return true
	}
}


// ErrorHandling1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ErrorHandling1Code {
	#[default]
	#[serde(rename = "X020")]
	CodeX020,
	#[serde(rename = "X030")]
	CodeX030,
	#[serde(rename = "X050")]
	CodeX050,
}

impl ErrorHandling1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ErrorHandling2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ErrorHandling1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl ErrorHandling2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// ErrorHandling4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling4 {
	#[serde(rename = "Err")]
	pub err: ErrorHandling2Choice,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max140Text>,
}

impl ErrorHandling4 {
	pub fn validate(&self) -> bool {
		if !self.err.validate() { return false }
		if let Some(ref desc_value) = self.desc { if !desc_value.validate() { return false; } }
		return true
	}
}


// ExternalEnquiryRequestType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalEnquiryRequestType1Code {
	#[serde(rename = "$value")]
	pub external_enquiry_request_type1_code: String,
}

impl ExternalEnquiryRequestType1Code {
	pub fn validate(&self) -> bool {
		if self.external_enquiry_request_type1_code.chars().count() < 1 {
			return false
		}
		if self.external_enquiry_request_type1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// ExternalMarketInfrastructure1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalMarketInfrastructure1Code {
	#[serde(rename = "$value")]
	pub external_market_infrastructure1_code: String,
}

impl ExternalMarketInfrastructure1Code {
	pub fn validate(&self) -> bool {
		if self.external_market_infrastructure1_code.chars().count() < 1 {
			return false
		}
		if self.external_market_infrastructure1_code.chars().count() > 3 {
			return false
		}
		return true
	}
}


// ExternalPaymentControlRequestType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalPaymentControlRequestType1Code {
	#[serde(rename = "$value")]
	pub external_payment_control_request_type1_code: String,
}

impl ExternalPaymentControlRequestType1Code {
	pub fn validate(&self) -> bool {
		if self.external_payment_control_request_type1_code.chars().count() < 1 {
			return false
		}
		if self.external_payment_control_request_type1_code.chars().count() > 4 {
			return false
		}
		return true
	}
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

impl GenericIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		return true
	}
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}

impl ISODate {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}

impl ISODateTime {
	pub fn validate(&self) -> bool {
		return true
	}
}


// MarketInfrastructureIdentification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketInfrastructureIdentification1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalMarketInfrastructure1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl MarketInfrastructureIdentification1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
}

impl Max140Text {
	pub fn validate(&self) -> bool {
		if self.max140_text.chars().count() < 1 {
			return false
		}
		if self.max140_text.chars().count() > 140 {
			return false
		}
		return true
	}
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}

impl Max350Text {
	pub fn validate(&self) -> bool {
		if self.max350_text.chars().count() < 1 {
			return false
		}
		if self.max350_text.chars().count() > 350 {
			return false
		}
		return true
	}
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}

impl Max35Text {
	pub fn validate(&self) -> bool {
		if self.max35_text.chars().count() < 1 {
			return false
		}
		if self.max35_text.chars().count() > 35 {
			return false
		}
		return true
	}
}


// MessageHeader11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader11 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
	#[serde(rename = "ReqTp", skip_serializing_if = "Option::is_none")]
	pub req_tp: Option<RequestType4Choice>,
	#[serde(rename = "OrgnlBizQry", skip_serializing_if = "Option::is_none")]
	pub orgnl_biz_qry: Option<OriginalBusinessQuery1>,
}

impl MessageHeader11 {
	pub fn validate(&self) -> bool {
		if !self.msg_id.validate() { return false }
		if let Some(ref req_tp_value) = self.req_tp { if !req_tp_value.validate() { return false; } }
		if let Some(ref orgnl_biz_qry_value) = self.orgnl_biz_qry { if !orgnl_biz_qry_value.validate() { return false; } }
		return true
	}
}


// OriginalBusinessQuery1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OriginalBusinessQuery1 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none")]
	pub msg_nm_id: Option<Max35Text>,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
}

impl OriginalBusinessQuery1 {
	pub fn validate(&self) -> bool {
		if !self.msg_id.validate() { return false }
		if let Some(ref msg_nm_id_value) = self.msg_nm_id { if !msg_nm_id_value.validate() { return false; } }
		return true
	}
}


// RequestType4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestType4Choice {
	#[serde(rename = "PmtCtrl", skip_serializing_if = "Option::is_none")]
	pub pmt_ctrl: Option<ExternalPaymentControlRequestType1Code>,
	#[serde(rename = "Enqry", skip_serializing_if = "Option::is_none")]
	pub enqry: Option<ExternalEnquiryRequestType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification1>,
}

impl RequestType4Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref pmt_ctrl_value) = self.pmt_ctrl { if !pmt_ctrl_value.validate() { return false; } }
		if let Some(ref enqry_value) = self.enqry { if !enqry_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
	pub plc_and_nm: Option<Max350Text>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}

impl SupplementaryData1 {
	pub fn validate(&self) -> bool {
		if let Some(ref plc_and_nm_value) = self.plc_and_nm { if !plc_and_nm_value.validate() { return false; } }
		if !self.envlp.validate() { return false }
		return true
	}
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// SystemAndCurrency1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemAndCurrency1 {
	#[serde(rename = "SysId")]
	pub sys_id: SystemIdentification2Choice,
	#[serde(rename = "SysCcy", skip_serializing_if = "Option::is_none")]
	pub sys_ccy: Option<ActiveCurrencyCode>,
}

impl SystemAndCurrency1 {
	pub fn validate(&self) -> bool {
		if !self.sys_id.validate() { return false }
		if let Some(ref sys_ccy_value) = self.sys_ccy { if !sys_ccy_value.validate() { return false; } }
		return true
	}
}


// SystemIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemIdentification2Choice {
	#[serde(rename = "MktInfrstrctrId", skip_serializing_if = "Option::is_none")]
	pub mkt_infrstrctr_id: Option<MarketInfrastructureIdentification1Choice>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
}

impl SystemIdentification2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref mkt_infrstrctr_id_value) = self.mkt_infrstrctr_id { if !mkt_infrstrctr_id_value.validate() { return false; } }
		if let Some(ref ctry_value) = self.ctry { if !ctry_value.validate() { return false; } }
		return true
	}
}


// SystemStatus3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemStatus3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<SystemStatus3Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification1>,
}

impl SystemStatus3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// SystemStatus3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SystemStatus3Code {
	#[default]
	#[serde(rename = "ACTV")]
	CodeACTV,
	#[serde(rename = "CLSD")]
	CodeCLSD,
	#[serde(rename = "RMPS")]
	CodeRMPS,
}

impl SystemStatus3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}
