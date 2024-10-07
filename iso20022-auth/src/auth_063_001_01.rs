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


// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "ActiveCurrencyAndAmount_SimpleType")]
	pub active_currency_and_amount_simple_type: f64,
}


// ActiveCurrencyAndAmount ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveCurrencyCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[validate(pattern = "[A-Z]{3,3}")]
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// AmountAndDirection102 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AmountAndDirection102 {
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
	#[serde(rename = "Sgn")]
	pub sgn: bool,
}


// CCPLiquidityStressTestingResultReportV01 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CCPLiquidityStressTestingResultReportV01 {
	#[validate]
	#[serde(rename = "LqdtyStrssTstRslt")]
	pub lqdty_strss_tst_rslt: Vec<LiquidityStressTestResult1>,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// CoverTwoDefaulters1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CoverTwoDefaulters1 {
	#[serde(rename = "Cover1Id")]
	pub cover1_id: String,
	#[serde(rename = "Cover2Id")]
	pub cover2_id: String,
}


// LEIIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[validate(pattern = "[A-Z0-9]{18,18}[0-9]{2,2}")]
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LiquidResourceInformation1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LiquidResourceInformation1 {
	#[serde(rename = "CntrPtyId")]
	pub cntr_pty_id: Option<String>,
	#[validate]
	#[serde(rename = "LqdRsrcVal")]
	pub lqd_rsrc_val: AmountAndDirection102,
	#[validate]
	#[serde(rename = "MktVal")]
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


// LiquidResources1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LiquidResources1 {
	#[validate]
	#[serde(rename = "CshDue")]
	pub csh_due: Vec<LiquidResourceInformation1>,
	#[validate]
	#[serde(rename = "FcltiesCmmtdLinesOfCdt")]
	pub fclties_cmmtd_lines_of_cdt: Option<Vec<LiquidResourceInformation1>>,
	#[validate]
	#[serde(rename = "FcltiesCmmtdRpAgrmts")]
	pub fclties_cmmtd_rp_agrmts: Option<Vec<LiquidResourceInformation1>>,
	#[validate]
	#[serde(rename = "FcltiesCmmtdFxSwps")]
	pub fclties_cmmtd_fx_swps: Option<Vec<LiquidResourceInformation1>>,
	#[validate]
	#[serde(rename = "FcltiesOthrCmmtd")]
	pub fclties_othr_cmmtd: Option<Vec<LiquidResourceInformation1>>,
	#[validate]
	#[serde(rename = "FcltiesUcmmtd")]
	pub fclties_ucmmtd: Option<Vec<LiquidResourceInformation1>>,
	#[validate]
	#[serde(rename = "FinInstrmsCCP")]
	pub fin_instrms_ccp: Option<Vec<LiquidResourceInformation1>>,
	#[validate]
	#[serde(rename = "FinInstrmsTrsrInvstmts")]
	pub fin_instrms_trsr_invstmts: Option<Vec<LiquidResourceInformation1>>,
	#[validate]
	#[serde(rename = "FinInstrmsDfltrsSttlmColl")]
	pub fin_instrms_dfltrs_sttlm_coll: Option<Vec<LiquidResourceInformation1>>,
	#[validate]
	#[serde(rename = "FinInstrmsDfltrsNonCshColl")]
	pub fin_instrms_dfltrs_non_csh_coll: Option<Vec<LiquidResourceInformation1>>,
}


// LiquidityRequiredAndAvailable1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LiquidityRequiredAndAvailable1 {
	#[validate]
	#[serde(rename = "LqdRsrcs")]
	pub lqd_rsrcs: LiquidResources1,
	#[serde(rename = "LqdtyHrzn")]
	pub lqdty_hrzn: String,
	#[validate]
	#[serde(rename = "StrssLqdRsrcRqrmnt")]
	pub strss_lqd_rsrc_rqrmnt: StressLiquidResourceRequirement1,
}


// LiquidityStressTestResult1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LiquidityStressTestResult1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[validate]
	#[serde(rename = "ScnroDfltrs")]
	pub scnro_dfltrs: CoverTwoDefaulters1,
	#[validate]
	#[serde(rename = "LqdtyReqrdAndAvlbl")]
	pub lqdty_reqrd_and_avlbl: Vec<LiquidityRequiredAndAvailable1>,
}


// Max256Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max256Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 256)]
	#[serde(rename = "Max256Text")]
	pub max256_text: String,
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


// PlusOrMinusIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "PlusOrMinusIndicator")]
	pub plus_or_minus_indicator: bool,
}


// SettlementDate6Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementDate6Code {
	#[validate(enumerate = ["TFIV", "TFOR", "TONE", "TTRE", "TTWO", "SAMD"])]
	#[serde(rename = "SettlementDate6Code")]
	pub settlement_date6_code: String,
}


// StressLiquidResourceRequirement1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct StressLiquidResourceRequirement1 {
	#[validate]
	#[serde(rename = "OprlOutflw")]
	pub oprl_outflw: AmountAndDirection102,
	#[validate]
	#[serde(rename = "VartnMrgnPmtOblgtn")]
	pub vartn_mrgn_pmt_oblgtn: AmountAndDirection102,
	#[validate]
	#[serde(rename = "SttlmOrDlvry")]
	pub sttlm_or_dlvry: AmountAndDirection102,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: AmountAndDirection102,
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


// TrueFalseIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}
