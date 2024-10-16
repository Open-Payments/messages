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


// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_currency_and_amount_simple_type: f64,
}


// ActiveCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "$value")]
	pub active_currency_code: String,
}


// AmountAndDirection102 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection102 {
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
	#[serde(rename = "Sgn")]
	pub sgn: bool,
}


// CCPLiquidityStressTestingResultReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CCPLiquidityStressTestingResultReportV01 {
	#[serde(rename = "LqdtyStrssTstRslt")]
	pub lqdty_strss_tst_rslt: Vec<LiquidityStressTestResult1>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// CoverTwoDefaulters1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CoverTwoDefaulters1 {
	#[serde(rename = "Cover1Id")]
	pub cover1_id: LEIIdentifier,
	#[serde(rename = "Cover2Id")]
	pub cover2_id: LEIIdentifier,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
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


// Max256Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max256Text {
	#[serde(rename = "$value")]
	pub max256_text: String,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}


// PlusOrMinusIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "$value")]
	pub plus_or_minus_indicator: bool,
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


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
	pub plc_and_nm: Option<Max350Text>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}
