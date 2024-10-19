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


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BaseOneRate {
	#[serde(rename = "$value")]
	pub base_one_rate: f64,
}

impl BaseOneRate {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.incm_stmt.validate() { return Err(e); }
		if let Err(e) = self.cptl_rqrmnts.validate() { return Err(e); }
		if let Err(e) = self.ttl_cptl.validate() { return Err(e); }
		if let Err(e) = self.lqd_fin_rsrcs.validate() { return Err(e); }
		for item in &self.hpthtcl_cptl_measr { if let Err(e) = item.validate() { return Err(e); } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.wndg_dwn_or_rstrg_rsk.validate() { return Err(e); }
		if let Err(e) = self.oprl_and_lgl_rsk.validate() { return Err(e); }
		if let Err(e) = self.cdt_rsk.validate() { return Err(e); }
		if let Err(e) = self.cntr_pty_rsk.validate() { return Err(e); }
		if let Err(e) = self.mkt_rsk.validate() { return Err(e); }
		if let Err(e) = self.biz_rsk.validate() { return Err(e); }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.amt.validate() { return Err(e); }
		if let Err(e) = self.dflt_wtrfll_id.validate() { return Err(e); }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.clr_fees.validate() { return Err(e); }
		if let Err(e) = self.othr_oprg_rvn.validate() { return Err(e); }
		if let Err(e) = self.oprg_expnss.validate() { return Err(e); }
		if let Err(e) = self.oprg_prft_or_loss.validate() { return Err(e); }
		if let Err(e) = self.net_intrst_incm.validate() { return Err(e); }
		if let Err(e) = self.othr_non_oprg_rvn.validate() { return Err(e); }
		if let Err(e) = self.non_oprg_expnss.validate() { return Err(e); }
		if let Err(e) = self.pre_tax_prft_or_loss.validate() { return Err(e); }
		if let Err(e) = self.pst_tax_prft_or_loss.validate() { return Err(e); }
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


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}

impl Max35Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max35_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max35_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max35_text.chars().count() > 35 {
			return Err(ValidationError::new(1002, "max35_text exceeds the maximum length of 35".to_string()));
		}
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
