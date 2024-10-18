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


// AmountAndDirection86 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection86 {
	#[serde(rename = "Amt")]
	pub amt: f64,
	#[serde(rename = "Sgn")]
	pub sgn: bool,
}

impl AmountAndDirection86 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CCPDailyCashFlowsReportV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CCPDailyCashFlowsReportV02 {
	#[serde(rename = "CncntrtnAgt")]
	pub cncntrtn_agt: Vec<ConcentrationAgent1>,
	#[serde(rename = "SttlmAgt")]
	pub sttlm_agt: Vec<SettlementAgent2>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CCPDailyCashFlowsReportV02 {
	pub fn validate(&self) -> bool {
		for item in &self.cncntrtn_agt { if !item.validate() { return false; } }
		for item in &self.sttlm_agt { if !item.validate() { return false; } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
	}
}


// ConcentrationAccount1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
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
	pub late_pmt_conf: Max10NumericText,
}

impl ConcentrationAccount1 {
	pub fn validate(&self) -> bool {
		if !self.in_flow.validate() { return false }
		if !self.out_flow.validate() { return false }
		if !self.end_of_day.validate() { return false }
		if !self.peak_cdt.validate() { return false }
		if !self.peak_dbt.validate() { return false }
		if !self.late_pmt_conf.validate() { return false }
		return true
	}
}


// ConcentrationAgent1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConcentrationAgent1 {
	#[serde(rename = "Id")]
	pub id: LEIIdentifier,
	#[serde(rename = "Acct")]
	pub acct: Vec<ConcentrationAccount1>,
}

impl ConcentrationAgent1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		for item in &self.acct { if !item.validate() { return false; } }
		return true
	}
}


// Flows1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Flows1 {
	#[serde(rename = "PmtBkFlows")]
	pub pmt_bk_flows: AmountAndDirection102,
	#[serde(rename = "InvstmtFlows")]
	pub invstmt_flows: AmountAndDirection102,
}

impl Flows1 {
	pub fn validate(&self) -> bool {
		if !self.pmt_bk_flows.validate() { return false }
		if !self.invstmt_flows.validate() { return false }
		return true
	}
}


// ImpliedCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ImpliedCurrencyAndAmount {
	#[serde(rename = "$value")]
	pub implied_currency_and_amount: f64,
}

impl ImpliedCurrencyAndAmount {
	pub fn validate(&self) -> bool {
		if self.implied_currency_and_amount < 0.000000 {
			return false
		}
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


// Max10NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max10NumericText {
	#[serde(rename = "$value")]
	pub max10_numeric_text: String,
}

impl Max10NumericText {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[0-9]{1,10}").unwrap();
		if !pattern.is_match(&self.max10_numeric_text) {
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


// PaymentAccount4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentAccount4 {
	#[serde(rename = "Ccy")]
	pub ccy: ActiveCurrencyCode,
	#[serde(rename = "NetPmt")]
	pub net_pmt: AmountAndDirection86,
	#[serde(rename = "GrssCdts")]
	pub grss_cdts: f64,
	#[serde(rename = "GrssDbts")]
	pub grss_dbts: f64,
	#[serde(rename = "LatePmtConf")]
	pub late_pmt_conf: Max10NumericText,
}

impl PaymentAccount4 {
	pub fn validate(&self) -> bool {
		if !self.ccy.validate() { return false }
		if !self.net_pmt.validate() { return false }
		if !self.late_pmt_conf.validate() { return false }
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


// SettlementAgent2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementAgent2 {
	#[serde(rename = "Id")]
	pub id: LEIIdentifier,
	#[serde(rename = "Acct")]
	pub acct: Vec<PaymentAccount4>,
}

impl SettlementAgent2 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		for item in &self.acct { if !item.validate() { return false; } }
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
