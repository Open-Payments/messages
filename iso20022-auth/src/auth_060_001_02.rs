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
use crate::validationerror::*;
// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_currency_and_amount_simple_type: f64,
}

impl ActiveCurrencyAndAmountSimpleType {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.active_currency_and_amount_simple_type < 0.000000 {
			return Err(ValidationError::new(1003, "active_currency_and_amount_simple_type is less than the minimum value of 0.000000".to_string()));
		}
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.active_currency_code) {
			return Err(ValidationError::new(1005, "active_currency_code does not match the required pattern".to_string()));
		}
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.amt.validate() { return Err(e); }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.cncntrtn_agt { if let Err(e) = item.validate() { return Err(e); } }
		for item in &self.sttlm_agt { if let Err(e) = item.validate() { return Err(e); } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.in_flow.validate() { return Err(e); }
		if let Err(e) = self.out_flow.validate() { return Err(e); }
		if let Err(e) = self.end_of_day.validate() { return Err(e); }
		if let Err(e) = self.peak_cdt.validate() { return Err(e); }
		if let Err(e) = self.peak_dbt.validate() { return Err(e); }
		if let Err(e) = self.late_pmt_conf.validate() { return Err(e); }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		for item in &self.acct { if let Err(e) = item.validate() { return Err(e); } }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.pmt_bk_flows.validate() { return Err(e); }
		if let Err(e) = self.invstmt_flows.validate() { return Err(e); }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.implied_currency_and_amount < 0.000000 {
			return Err(ValidationError::new(1003, "implied_currency_and_amount is less than the minimum value of 0.000000".to_string()));
		}
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.lei_identifier) {
			return Err(ValidationError::new(1005, "lei_identifier does not match the required pattern".to_string()));
		}
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{1,10}").unwrap();
		if !pattern.is_match(&self.max10_numeric_text) {
			return Err(ValidationError::new(1005, "max10_numeric_text does not match the required pattern".to_string()));
		}
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max350_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max350_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max350_text.chars().count() > 350 {
			return Err(ValidationError::new(1002, "max350_text exceeds the maximum length of 350".to_string()));
		}
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.ccy.validate() { return Err(e); }
		if let Err(e) = self.net_pmt.validate() { return Err(e); }
		if let Err(e) = self.late_pmt_conf.validate() { return Err(e); }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		for item in &self.acct { if let Err(e) = item.validate() { return Err(e); } }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref plc_and_nm_value) = self.plc_and_nm { if let Err(e) = plc_and_nm_value.validate() { return Err(e); } }
		if let Err(e) = self.envlp.validate() { return Err(e); }
		Ok(())
	}
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}
