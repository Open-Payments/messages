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


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BaseOneRate {
	#[serde(rename = "$value")]
	pub base_one_rate: f64,
}


// CCPIncomeStatementAndCapitalAdequacyReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CCPIncomeStatementAndCapitalAdequacyReportV01 {
	#[serde(rename = "IncmStmt")]
	pub incm_stmt: IncomeStatement1,
	#[serde(rename = "CptlRqrmnts")]
	pub cptl_rqrmnts: CapitalRequirement1,
	#[serde(rename = "TtlCptl")]
	pub ttl_cptl: ActiveCurrencyAndAmount,
	#[serde(rename = "LqdFinRsrcs")]
	pub lqd_fin_rsrcs: ActiveCurrencyAndAmount,
	#[serde(rename = "HpthtclCptlMeasr")]
	pub hpthtcl_cptl_measr: Vec<HypotheticalCapitalMeasure1>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// CapitalRequirement1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CapitalRequirement1 {
	#[serde(rename = "WndgDwnOrRstrgRsk")]
	pub wndg_dwn_or_rstrg_rsk: ActiveCurrencyAndAmount,
	#[serde(rename = "OprlAndLglRsk")]
	pub oprl_and_lgl_rsk: ActiveCurrencyAndAmount,
	#[serde(rename = "CdtRsk")]
	pub cdt_rsk: ActiveCurrencyAndAmount,
	#[serde(rename = "CntrPtyRsk")]
	pub cntr_pty_rsk: ActiveCurrencyAndAmount,
	#[serde(rename = "MktRsk")]
	pub mkt_rsk: ActiveCurrencyAndAmount,
	#[serde(rename = "BizRsk")]
	pub biz_rsk: ActiveCurrencyAndAmount,
	#[serde(rename = "NtfctnBffr", skip_serializing_if = "Option::is_none")]
	pub ntfctn_bffr: Option<f64>,
}


// HypotheticalCapitalMeasure1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct HypotheticalCapitalMeasure1 {
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
	#[serde(rename = "DfltWtrfllId")]
	pub dflt_wtrfll_id: Max35Text,
}


// IncomeStatement1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IncomeStatement1 {
	#[serde(rename = "ClrFees")]
	pub clr_fees: ActiveCurrencyAndAmount,
	#[serde(rename = "OthrOprgRvn")]
	pub othr_oprg_rvn: ActiveCurrencyAndAmount,
	#[serde(rename = "OprgExpnss")]
	pub oprg_expnss: ActiveCurrencyAndAmount,
	#[serde(rename = "OprgPrftOrLoss")]
	pub oprg_prft_or_loss: AmountAndDirection102,
	#[serde(rename = "NetIntrstIncm")]
	pub net_intrst_incm: ActiveCurrencyAndAmount,
	#[serde(rename = "OthrNonOprgRvn")]
	pub othr_non_oprg_rvn: ActiveCurrencyAndAmount,
	#[serde(rename = "NonOprgExpnss")]
	pub non_oprg_expnss: ActiveCurrencyAndAmount,
	#[serde(rename = "PreTaxPrftOrLoss")]
	pub pre_tax_prft_or_loss: AmountAndDirection102,
	#[serde(rename = "PstTaxPrftOrLoss")]
	pub pst_tax_prft_or_loss: AmountAndDirection102,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}


// PlusOrMinusIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "$value")]
	pub plus_or_minus_indicator: bool,
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
