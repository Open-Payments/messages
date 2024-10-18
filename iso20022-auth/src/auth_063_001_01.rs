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


// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_currency_and_amount_simple_type: f64,
}

impl ActiveCurrencyAndAmountSimpleType {
	pub fn validate(&self) -> bool {
		if self.active_currency_and_amount_simple_type < 0.000000 {
			return false
		}
		return true
	}
}


// ActiveCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveCurrencyAndAmount {
	pub fn validate(&self) -> bool {
		return true
	}
}


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


// AmountAndDirection102 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection102 {
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
	#[serde(rename = "Sgn")]
	pub sgn: bool,
}

impl AmountAndDirection102 {
	pub fn validate(&self) -> bool {
		if !self.amt.validate() { return false }
		return true
	}
}


// CCPLiquidityStressTestingResultReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CCPLiquidityStressTestingResultReportV01 {
	#[serde(rename = "LqdtyStrssTstRslt")]
	pub lqdty_strss_tst_rslt: Vec<LiquidityStressTestResult1>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CCPLiquidityStressTestingResultReportV01 {
	pub fn validate(&self) -> bool {
		for item in &self.lqdty_strss_tst_rslt { if !item.validate() { return false; } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
	}
}


// CoverTwoDefaulters1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CoverTwoDefaulters1 {
	#[serde(rename = "Cover1Id")]
	pub cover1_id: LEIIdentifier,
	#[serde(rename = "Cover2Id")]
	pub cover2_id: LEIIdentifier,
}

impl CoverTwoDefaulters1 {
	pub fn validate(&self) -> bool {
		if !self.cover1_id.validate() { return false }
		if !self.cover2_id.validate() { return false }
		return true
	}
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}

impl LEIIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.lei_identifier) {
			return false
		}
		return true
	}
}


// LiquidResourceInformation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LiquidResourceInformation1 {
	#[serde(rename = "CntrPtyId", skip_serializing_if = "Option::is_none")]
	pub cntr_pty_id: Option<Max35Text>,
	#[serde(rename = "LqdRsrcVal")]
	pub lqd_rsrc_val: AmountAndDirection102,
	#[serde(rename = "MktVal", skip_serializing_if = "Option::is_none")]
	pub mkt_val: Option<AmountAndDirection102>,
	#[serde(rename = "Scrd")]
	pub scrd: bool,
	#[serde(rename = "AsstNcmbrd")]
	pub asst_ncmbrd: bool,
	#[serde(rename = "QlfygRsrc")]
	pub qlfyg_rsrc: bool,
	#[serde(rename = "AgcyArrgmnts")]
	pub agcy_arrgmnts: bool,
}

impl LiquidResourceInformation1 {
	pub fn validate(&self) -> bool {
		if let Some(ref cntr_pty_id_value) = self.cntr_pty_id { if !cntr_pty_id_value.validate() { return false; } }
		if !self.lqd_rsrc_val.validate() { return false }
		if let Some(ref mkt_val_value) = self.mkt_val { if !mkt_val_value.validate() { return false; } }
		return true
	}
}


// LiquidResources1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LiquidResources1 {
	#[serde(rename = "CshDue")]
	pub csh_due: Vec<LiquidResourceInformation1>,
	#[serde(rename = "FcltiesCmmtdLinesOfCdt", skip_serializing_if = "Option::is_none")]
	pub fclties_cmmtd_lines_of_cdt: Option<Vec<LiquidResourceInformation1>>,
	#[serde(rename = "FcltiesCmmtdRpAgrmts", skip_serializing_if = "Option::is_none")]
	pub fclties_cmmtd_rp_agrmts: Option<Vec<LiquidResourceInformation1>>,
	#[serde(rename = "FcltiesCmmtdFxSwps", skip_serializing_if = "Option::is_none")]
	pub fclties_cmmtd_fx_swps: Option<Vec<LiquidResourceInformation1>>,
	#[serde(rename = "FcltiesOthrCmmtd", skip_serializing_if = "Option::is_none")]
	pub fclties_othr_cmmtd: Option<Vec<LiquidResourceInformation1>>,
	#[serde(rename = "FcltiesUcmmtd", skip_serializing_if = "Option::is_none")]
	pub fclties_ucmmtd: Option<Vec<LiquidResourceInformation1>>,
	#[serde(rename = "FinInstrmsCCP", skip_serializing_if = "Option::is_none")]
	pub fin_instrms_ccp: Option<Vec<LiquidResourceInformation1>>,
	#[serde(rename = "FinInstrmsTrsrInvstmts", skip_serializing_if = "Option::is_none")]
	pub fin_instrms_trsr_invstmts: Option<Vec<LiquidResourceInformation1>>,
	#[serde(rename = "FinInstrmsDfltrsSttlmColl", skip_serializing_if = "Option::is_none")]
	pub fin_instrms_dfltrs_sttlm_coll: Option<Vec<LiquidResourceInformation1>>,
	#[serde(rename = "FinInstrmsDfltrsNonCshColl", skip_serializing_if = "Option::is_none")]
	pub fin_instrms_dfltrs_non_csh_coll: Option<Vec<LiquidResourceInformation1>>,
}

impl LiquidResources1 {
	pub fn validate(&self) -> bool {
		for item in &self.csh_due { if !item.validate() { return false; } }
		if let Some(ref fclties_cmmtd_lines_of_cdt_vec) = self.fclties_cmmtd_lines_of_cdt { for item in fclties_cmmtd_lines_of_cdt_vec { if !item.validate() { return false; } } }
		if let Some(ref fclties_cmmtd_rp_agrmts_vec) = self.fclties_cmmtd_rp_agrmts { for item in fclties_cmmtd_rp_agrmts_vec { if !item.validate() { return false; } } }
		if let Some(ref fclties_cmmtd_fx_swps_vec) = self.fclties_cmmtd_fx_swps { for item in fclties_cmmtd_fx_swps_vec { if !item.validate() { return false; } } }
		if let Some(ref fclties_othr_cmmtd_vec) = self.fclties_othr_cmmtd { for item in fclties_othr_cmmtd_vec { if !item.validate() { return false; } } }
		if let Some(ref fclties_ucmmtd_vec) = self.fclties_ucmmtd { for item in fclties_ucmmtd_vec { if !item.validate() { return false; } } }
		if let Some(ref fin_instrms_ccp_vec) = self.fin_instrms_ccp { for item in fin_instrms_ccp_vec { if !item.validate() { return false; } } }
		if let Some(ref fin_instrms_trsr_invstmts_vec) = self.fin_instrms_trsr_invstmts { for item in fin_instrms_trsr_invstmts_vec { if !item.validate() { return false; } } }
		if let Some(ref fin_instrms_dfltrs_sttlm_coll_vec) = self.fin_instrms_dfltrs_sttlm_coll { for item in fin_instrms_dfltrs_sttlm_coll_vec { if !item.validate() { return false; } } }
		if let Some(ref fin_instrms_dfltrs_non_csh_coll_vec) = self.fin_instrms_dfltrs_non_csh_coll { for item in fin_instrms_dfltrs_non_csh_coll_vec { if !item.validate() { return false; } } }
		return true
	}
}


// LiquidityRequiredAndAvailable1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LiquidityRequiredAndAvailable1 {
	#[serde(rename = "LqdRsrcs")]
	pub lqd_rsrcs: LiquidResources1,
	#[serde(rename = "LqdtyHrzn")]
	pub lqdty_hrzn: SettlementDate6Code,
	#[serde(rename = "StrssLqdRsrcRqrmnt")]
	pub strss_lqd_rsrc_rqrmnt: StressLiquidResourceRequirement1,
}

impl LiquidityRequiredAndAvailable1 {
	pub fn validate(&self) -> bool {
		if !self.lqd_rsrcs.validate() { return false }
		if !self.lqdty_hrzn.validate() { return false }
		if !self.strss_lqd_rsrc_rqrmnt.validate() { return false }
		return true
	}
}


// LiquidityStressTestResult1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LiquidityStressTestResult1 {
	#[serde(rename = "Id")]
	pub id: Max256Text,
	#[serde(rename = "ScnroDfltrs")]
	pub scnro_dfltrs: CoverTwoDefaulters1,
	#[serde(rename = "LqdtyReqrdAndAvlbl")]
	pub lqdty_reqrd_and_avlbl: Vec<LiquidityRequiredAndAvailable1>,
}

impl LiquidityStressTestResult1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.scnro_dfltrs.validate() { return false }
		for item in &self.lqdty_reqrd_and_avlbl { if !item.validate() { return false; } }
		return true
	}
}


// Max256Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max256Text {
	#[serde(rename = "$value")]
	pub max256_text: String,
}

impl Max256Text {
	pub fn validate(&self) -> bool {
		if self.max256_text.chars().count() < 1 {
			return false
		}
		if self.max256_text.chars().count() > 256 {
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


// PlusOrMinusIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "$value")]
	pub plus_or_minus_indicator: bool,
}

impl PlusOrMinusIndicator {
	pub fn validate(&self) -> bool {
		return true
	}
}


// SettlementDate6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SettlementDate6Code {
	#[default]
	#[serde(rename = "TFIV")]
	CodeTFIV,
	#[serde(rename = "TFOR")]
	CodeTFOR,
	#[serde(rename = "TONE")]
	CodeTONE,
	#[serde(rename = "TTRE")]
	CodeTTRE,
	#[serde(rename = "TTWO")]
	CodeTTWO,
	#[serde(rename = "SAMD")]
	CodeSAMD,
}

impl SettlementDate6Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// StressLiquidResourceRequirement1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StressLiquidResourceRequirement1 {
	#[serde(rename = "OprlOutflw")]
	pub oprl_outflw: AmountAndDirection102,
	#[serde(rename = "VartnMrgnPmtOblgtn")]
	pub vartn_mrgn_pmt_oblgtn: AmountAndDirection102,
	#[serde(rename = "SttlmOrDlvry")]
	pub sttlm_or_dlvry: AmountAndDirection102,
	#[serde(rename = "Othr")]
	pub othr: AmountAndDirection102,
}

impl StressLiquidResourceRequirement1 {
	pub fn validate(&self) -> bool {
		if !self.oprl_outflw.validate() { return false }
		if !self.vartn_mrgn_pmt_oblgtn.validate() { return false }
		if !self.sttlm_or_dlvry.validate() { return false }
		if !self.othr.validate() { return false }
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


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}

impl TrueFalseIndicator {
	pub fn validate(&self) -> bool {
		return true
	}
}
