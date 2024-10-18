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


// BusinessDayCriteria2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BusinessDayCriteria2 {
	#[serde(rename = "NewQryNm", skip_serializing_if = "Option::is_none")]
	pub new_qry_nm: Option<Max35Text>,
	#[serde(rename = "SchCrit", skip_serializing_if = "Option::is_none")]
	pub sch_crit: Option<Vec<BusinessDaySearchCriteria2>>,
	#[serde(rename = "RtrCrit", skip_serializing_if = "Option::is_none")]
	pub rtr_crit: Option<BusinessDayReturnCriteria2>,
}

impl BusinessDayCriteria2 {
	pub fn validate(&self) -> bool {
		if let Some(ref new_qry_nm_value) = self.new_qry_nm { if !new_qry_nm_value.validate() { return false; } }
		if let Some(ref sch_crit_vec) = self.sch_crit { for item in sch_crit_vec { if !item.validate() { return false; } } }
		if let Some(ref rtr_crit_value) = self.rtr_crit { if !rtr_crit_value.validate() { return false; } }
		return true
	}
}


// BusinessDayCriteria3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BusinessDayCriteria3Choice {
	#[serde(rename = "QryNm", skip_serializing_if = "Option::is_none")]
	pub qry_nm: Option<Max35Text>,
	#[serde(rename = "NewCrit", skip_serializing_if = "Option::is_none")]
	pub new_crit: Option<BusinessDayCriteria2>,
}

impl BusinessDayCriteria3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref qry_nm_value) = self.qry_nm { if !qry_nm_value.validate() { return false; } }
		if let Some(ref new_crit_value) = self.new_crit { if !new_crit_value.validate() { return false; } }
		return true
	}
}


// BusinessDayQuery2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BusinessDayQuery2 {
	#[serde(rename = "QryTp", skip_serializing_if = "Option::is_none")]
	pub qry_tp: Option<QueryType2Code>,
	#[serde(rename = "Crit", skip_serializing_if = "Option::is_none")]
	pub crit: Option<BusinessDayCriteria3Choice>,
}

impl BusinessDayQuery2 {
	pub fn validate(&self) -> bool {
		if let Some(ref qry_tp_value) = self.qry_tp { if !qry_tp_value.validate() { return false; } }
		if let Some(ref crit_value) = self.crit { if !crit_value.validate() { return false; } }
		return true
	}
}


// BusinessDayReturnCriteria2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BusinessDayReturnCriteria2 {
	#[serde(rename = "SysDtInd", skip_serializing_if = "Option::is_none")]
	pub sys_dt_ind: Option<bool>,
	#[serde(rename = "SysStsInd", skip_serializing_if = "Option::is_none")]
	pub sys_sts_ind: Option<bool>,
	#[serde(rename = "SysCcyInd", skip_serializing_if = "Option::is_none")]
	pub sys_ccy_ind: Option<bool>,
	#[serde(rename = "ClsrPrdInd", skip_serializing_if = "Option::is_none")]
	pub clsr_prd_ind: Option<bool>,
	#[serde(rename = "EvtInd", skip_serializing_if = "Option::is_none")]
	pub evt_ind: Option<bool>,
	#[serde(rename = "SsnPrdInd", skip_serializing_if = "Option::is_none")]
	pub ssn_prd_ind: Option<bool>,
	#[serde(rename = "EvtTpInd", skip_serializing_if = "Option::is_none")]
	pub evt_tp_ind: Option<bool>,
}

impl BusinessDayReturnCriteria2 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// BusinessDaySearchCriteria2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BusinessDaySearchCriteria2 {
	#[serde(rename = "SysDt", skip_serializing_if = "Option::is_none")]
	pub sys_dt: Option<String>,
	#[serde(rename = "SysId", skip_serializing_if = "Option::is_none")]
	pub sys_id: Option<Vec<SystemIdentification2Choice>>,
	#[serde(rename = "SysCcy", skip_serializing_if = "Option::is_none")]
	pub sys_ccy: Option<Vec<ActiveCurrencyCode>>,
	#[serde(rename = "EvtTp", skip_serializing_if = "Option::is_none")]
	pub evt_tp: Option<SystemEventType2Choice>,
	#[serde(rename = "ClsrPrd", skip_serializing_if = "Option::is_none")]
	pub clsr_prd: Option<DateTimePeriod1Choice>,
}

impl BusinessDaySearchCriteria2 {
	pub fn validate(&self) -> bool {
		if let Some(ref sys_id_vec) = self.sys_id { for item in sys_id_vec { if !item.validate() { return false; } } }
		if let Some(ref sys_ccy_vec) = self.sys_ccy { for item in sys_ccy_vec { if !item.validate() { return false; } } }
		if let Some(ref evt_tp_value) = self.evt_tp { if !evt_tp_value.validate() { return false; } }
		if let Some(ref clsr_prd_value) = self.clsr_prd { if !clsr_prd_value.validate() { return false; } }
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


// DateTimePeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod1 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: String,
}

impl DateTimePeriod1 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// DateTimePeriod1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod1Choice {
	#[serde(rename = "FrDtTm", skip_serializing_if = "Option::is_none")]
	pub fr_dt_tm: Option<String>,
	#[serde(rename = "ToDtTm", skip_serializing_if = "Option::is_none")]
	pub to_dt_tm: Option<String>,
	#[serde(rename = "DtTmRg", skip_serializing_if = "Option::is_none")]
	pub dt_tm_rg: Option<DateTimePeriod1>,
}

impl DateTimePeriod1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref dt_tm_rg_value) = self.dt_tm_rg { if !dt_tm_rg_value.validate() { return false; } }
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


// GetBusinessDayInformationV05 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GetBusinessDayInformationV05 {
	#[serde(rename = "MsgHdr")]
	pub msg_hdr: MessageHeader9,
	#[serde(rename = "BizDayInfQryDef", skip_serializing_if = "Option::is_none")]
	pub biz_day_inf_qry_def: Option<BusinessDayQuery2>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl GetBusinessDayInformationV05 {
	pub fn validate(&self) -> bool {
		if !self.msg_hdr.validate() { return false }
		if let Some(ref biz_day_inf_qry_def_value) = self.biz_day_inf_qry_def { if !biz_day_inf_qry_def_value.validate() { return false; } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
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


// MessageHeader9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader9 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
	#[serde(rename = "ReqTp", skip_serializing_if = "Option::is_none")]
	pub req_tp: Option<RequestType4Choice>,
}

impl MessageHeader9 {
	pub fn validate(&self) -> bool {
		if !self.msg_id.validate() { return false }
		if let Some(ref req_tp_value) = self.req_tp { if !req_tp_value.validate() { return false; } }
		return true
	}
}


// QueryType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum QueryType2Code {
	#[default]
	#[serde(rename = "ALLL")]
	CodeALLL,
	#[serde(rename = "CHNG")]
	CodeCHNG,
	#[serde(rename = "MODF")]
	CodeMODF,
	#[serde(rename = "DELD")]
	CodeDELD,
}

impl QueryType2Code {
	pub fn validate(&self) -> bool {
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


// RequestedIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RequestedIndicator {
	#[serde(rename = "$value")]
	pub requested_indicator: bool,
}

impl RequestedIndicator {
	pub fn validate(&self) -> bool {
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


// SystemEventType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemEventType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<SystemEventType2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification1>,
}

impl SystemEventType2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// SystemEventType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SystemEventType2Code {
	#[default]
	#[serde(rename = "LVCO")]
	CodeLVCO,
	#[serde(rename = "LVCC")]
	CodeLVCC,
	#[serde(rename = "LVRT")]
	CodeLVRT,
	#[serde(rename = "EUSU")]
	CodeEUSU,
	#[serde(rename = "STSU")]
	CodeSTSU,
	#[serde(rename = "LWSU")]
	CodeLWSU,
	#[serde(rename = "EUCO")]
	CodeEUCO,
	#[serde(rename = "FIRE")]
	CodeFIRE,
	#[serde(rename = "STDY")]
	CodeSTDY,
	#[serde(rename = "LTNC")]
	CodeLTNC,
	#[serde(rename = "CRCO")]
	CodeCRCO,
	#[serde(rename = "RECC")]
	CodeRECC,
	#[serde(rename = "LTGC")]
	CodeLTGC,
	#[serde(rename = "LTDC")]
	CodeLTDC,
	#[serde(rename = "CUSC")]
	CodeCUSC,
	#[serde(rename = "IBKC")]
	CodeIBKC,
	#[serde(rename = "SYSC")]
	CodeSYSC,
	#[serde(rename = "SSSC")]
	CodeSSSC,
	#[serde(rename = "REOP")]
	CodeREOP,
	#[serde(rename = "PCOT")]
	CodePCOT,
	#[serde(rename = "NPCT")]
	CodeNPCT,
	#[serde(rename = "ESTF")]
	CodeESTF,
}

impl SystemEventType2Code {
	pub fn validate(&self) -> bool {
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
