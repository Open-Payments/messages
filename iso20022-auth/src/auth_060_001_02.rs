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
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "ActiveCurrencyAndAmount_SimpleType")]
	pub active_currency_and_amount_simple_type: f64,
}


// ActiveCurrencyAndAmount ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveCurrencyCode ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// AmountAndDirection102 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection102 {
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
	#[serde(rename = "Sgn")]
	pub sgn: bool,
}


// AmountAndDirection86 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection86 {
	#[serde(rename = "Amt")]
	pub amt: f64,
	#[serde(rename = "Sgn")]
	pub sgn: bool,
}


// CCPDailyCashFlowsReportV02 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CCPDailyCashFlowsReportV02 {
	#[serde(rename = "CncntrtnAgt")]
	pub cncntrtn_agt: Vec<ConcentrationAgent1>,
	#[serde(rename = "SttlmAgt")]
	pub sttlm_agt: Vec<SettlementAgent2>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// ConcentrationAccount1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConcentrationAccount1 {
	#[serde(rename = "InFlow")]
	pub in_flow: Flows1,
	#[serde(rename = "OutFlow")]
	pub out_flow: Flows1,
	#[serde(rename = "EndOfDay")]
	pub end_of_day: AmountAndDirection102,
	#[serde(rename = "PeakCdt")]
	pub peak_cdt: ActiveCurrencyAndAmount,
	#[serde(rename = "PeakDbt")]
	pub peak_dbt: ActiveCurrencyAndAmount,
	#[serde(rename = "LatePmtConf")]
	pub late_pmt_conf: String,
}


// ConcentrationAgent1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConcentrationAgent1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Acct")]
	pub acct: Vec<ConcentrationAccount1>,
}


// Flows1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Flows1 {
	#[serde(rename = "PmtBkFlows")]
	pub pmt_bk_flows: AmountAndDirection102,
	#[serde(rename = "InvstmtFlows")]
	pub invstmt_flows: AmountAndDirection102,
}


// ImpliedCurrencyAndAmount ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImpliedCurrencyAndAmount {
	#[serde(rename = "ImpliedCurrencyAndAmount")]
	pub implied_currency_and_amount: f64,
}


// LEIIdentifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// Max10NumericText ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max10NumericText {
	#[serde(rename = "Max10NumericText")]
	pub max10_numeric_text: String,
}


// Max350Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// PaymentAccount4 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentAccount4 {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "NetPmt")]
	pub net_pmt: AmountAndDirection86,
	#[serde(rename = "GrssCdts")]
	pub grss_cdts: f64,
	#[serde(rename = "GrssDbts")]
	pub grss_dbts: f64,
	#[serde(rename = "LatePmtConf")]
	pub late_pmt_conf: String,
}


// PlusOrMinusIndicator ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "PlusOrMinusIndicator")]
	pub plus_or_minus_indicator: bool,
}


// SettlementAgent2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementAgent2 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Acct")]
	pub acct: Vec<PaymentAccount4>,
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