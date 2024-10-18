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


// ActiveCurrencyAnd24AmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyAnd24AmountSimpleType {
	#[serde(rename = "$value")]
	pub active_currency_and24_amount_simple_type: f64,
}

impl ActiveCurrencyAnd24AmountSimpleType {
	pub fn validate(&self) -> bool {
		if self.active_currency_and24_amount_simple_type < 0.000000 {
			return false
		}
		return true
	}
}


// ActiveCurrencyAnd24Amount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd24Amount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveCurrencyAnd24Amount {
	pub fn validate(&self) -> bool {
		return true
	}
}


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


// CCPAccountPositionReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CCPAccountPositionReportV01 {
	#[serde(rename = "Prtfl")]
	pub prtfl: Vec<PositionAccount2>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CCPAccountPositionReportV01 {
	pub fn validate(&self) -> bool {
		for item in &self.prtfl { if !item.validate() { return false; } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
	}
}


// EndOfDayRequirement1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EndOfDayRequirement1 {
	#[serde(rename = "InitlMrgnRqrmnt", skip_serializing_if = "Option::is_none")]
	pub initl_mrgn_rqrmnt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "VartnMrgnRqrmnt", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_rqrmnt: Option<AmountAndDirection102>,
}

impl EndOfDayRequirement1 {
	pub fn validate(&self) -> bool {
		if let Some(ref initl_mrgn_rqrmnt_value) = self.initl_mrgn_rqrmnt { if !initl_mrgn_rqrmnt_value.validate() { return false; } }
		if let Some(ref vartn_mrgn_rqrmnt_value) = self.vartn_mrgn_rqrmnt { if !vartn_mrgn_rqrmnt_value.validate() { return false; } }
		return true
	}
}


// Fraction5DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Fraction5DecimalNumber {
	#[serde(rename = "$value")]
	pub fraction5_decimal_number: f64,
}

impl Fraction5DecimalNumber {
	pub fn validate(&self) -> bool {
		return true
	}
}


// GenericIdentification165 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification165 {
	#[serde(rename = "Id")]
	pub id: Max256Text,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max140Text>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<SchemeIdentificationType1Code>,
}

impl GenericIdentification165 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref desc_value) = self.desc { if !desc_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		return true
	}
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
}

impl Max140Text {
	pub fn validate(&self) -> bool {
		if self.max140_text.chars().count() < 1 {
			return false
		}
		if self.max140_text.chars().count() > 140 {
			return false
		}
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


// NonNegativeFraction5DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NonNegativeFraction5DecimalNumber {
	#[serde(rename = "$value")]
	pub non_negative_fraction5_decimal_number: f64,
}

impl NonNegativeFraction5DecimalNumber {
	pub fn validate(&self) -> bool {
		if self.non_negative_fraction5_decimal_number < 0.000000 {
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


// Position1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Position1 {
	#[serde(rename = "PdctId")]
	pub pdct_id: Max256Text,
	#[serde(rename = "RskRqrmnt", skip_serializing_if = "Option::is_none")]
	pub rsk_rqrmnt: Option<EndOfDayRequirement1>,
	#[serde(rename = "GrssNtnl")]
	pub grss_ntnl: ActiveCurrencyAnd24Amount,
	#[serde(rename = "NetNtnl")]
	pub net_ntnl: AmountAndDirection102,
	#[serde(rename = "GrssDltaEqvtVal", skip_serializing_if = "Option::is_none")]
	pub grss_dlta_eqvt_val: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "NetDltaEqvtVal", skip_serializing_if = "Option::is_none")]
	pub net_dlta_eqvt_val: Option<AmountAndDirection102>,
	#[serde(rename = "GrssDltaEqvtQty", skip_serializing_if = "Option::is_none")]
	pub grss_dlta_eqvt_qty: Option<f64>,
	#[serde(rename = "NetDltaEqvtQty", skip_serializing_if = "Option::is_none")]
	pub net_dlta_eqvt_qty: Option<f64>,
	#[serde(rename = "GrssMktVal")]
	pub grss_mkt_val: ActiveCurrencyAndAmount,
}

impl Position1 {
	pub fn validate(&self) -> bool {
		if !self.pdct_id.validate() { return false }
		if let Some(ref rsk_rqrmnt_value) = self.rsk_rqrmnt { if !rsk_rqrmnt_value.validate() { return false; } }
		if !self.grss_ntnl.validate() { return false }
		if !self.net_ntnl.validate() { return false }
		if let Some(ref grss_dlta_eqvt_val_value) = self.grss_dlta_eqvt_val { if !grss_dlta_eqvt_val_value.validate() { return false; } }
		if let Some(ref net_dlta_eqvt_val_value) = self.net_dlta_eqvt_val { if !net_dlta_eqvt_val_value.validate() { return false; } }
		if !self.grss_mkt_val.validate() { return false }
		return true
	}
}


// PositionAccount2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionAccount2 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification165,
	#[serde(rename = "Pos")]
	pub pos: Vec<Position1>,
}

impl PositionAccount2 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		for item in &self.pos { if !item.validate() { return false; } }
		return true
	}
}


// SchemeIdentificationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SchemeIdentificationType1Code {
	#[default]
	#[serde(rename = "MARG")]
	CodeMARG,
	#[serde(rename = "COLL")]
	CodeCOLL,
	#[serde(rename = "POSI")]
	CodePOSI,
	#[serde(rename = "CLIM")]
	CodeCLIM,
}

impl SchemeIdentificationType1Code {
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
