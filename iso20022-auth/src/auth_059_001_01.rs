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


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BaseOneRate {
	#[serde(rename = "$value")]
	pub base_one_rate: f64,
}

impl BaseOneRate {
	pub fn validate(&self) -> bool {
		return true
	}
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

impl CCPIncomeStatementAndCapitalAdequacyReportV01 {
	pub fn validate(&self) -> bool {
		if !self.incm_stmt.validate() { return false }
		if !self.cptl_rqrmnts.validate() { return false }
		if !self.ttl_cptl.validate() { return false }
		if !self.lqd_fin_rsrcs.validate() { return false }
		for item in &self.hpthtcl_cptl_measr { if !item.validate() { return false; } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
	}
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

impl CapitalRequirement1 {
	pub fn validate(&self) -> bool {
		if !self.wndg_dwn_or_rstrg_rsk.validate() { return false }
		if !self.oprl_and_lgl_rsk.validate() { return false }
		if !self.cdt_rsk.validate() { return false }
		if !self.cntr_pty_rsk.validate() { return false }
		if !self.mkt_rsk.validate() { return false }
		if !self.biz_rsk.validate() { return false }
		return true
	}
}


// HypotheticalCapitalMeasure1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct HypotheticalCapitalMeasure1 {
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
	#[serde(rename = "DfltWtrfllId")]
	pub dflt_wtrfll_id: Max35Text,
}

impl HypotheticalCapitalMeasure1 {
	pub fn validate(&self) -> bool {
		if !self.amt.validate() { return false }
		if !self.dflt_wtrfll_id.validate() { return false }
		return true
	}
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

impl IncomeStatement1 {
	pub fn validate(&self) -> bool {
		if !self.clr_fees.validate() { return false }
		if !self.othr_oprg_rvn.validate() { return false }
		if !self.oprg_expnss.validate() { return false }
		if !self.oprg_prft_or_loss.validate() { return false }
		if !self.net_intrst_incm.validate() { return false }
		if !self.othr_non_oprg_rvn.validate() { return false }
		if !self.non_oprg_expnss.validate() { return false }
		if !self.pre_tax_prft_or_loss.validate() { return false }
		if !self.pst_tax_prft_or_loss.validate() { return false }
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
