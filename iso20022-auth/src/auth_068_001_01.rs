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


// ActiveCurrencyAnd24AmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd24AmountSimpleType {
	#[serde(rename = "ActiveCurrencyAnd24Amount_SimpleType")]
	pub active_currency_and24_amount_simple_type: f64,
}


// ActiveCurrencyAnd24Amount ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd24Amount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


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


// CCPAccountPositionReportV01 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CCPAccountPositionReportV01 {
	#[serde(rename = "Prtfl")]
	pub prtfl: Vec<PositionAccount2>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// EndOfDayRequirement1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EndOfDayRequirement1 {
	#[serde(rename = "InitlMrgnRqrmnt")]
	pub initl_mrgn_rqrmnt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "VartnMrgnRqrmnt")]
	pub vartn_mrgn_rqrmnt: Option<AmountAndDirection102>,
}


// Fraction5DecimalNumber ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Fraction5DecimalNumber {
	#[serde(rename = "Fraction5DecimalNumber")]
	pub fraction5_decimal_number: f64,
}


// GenericIdentification165 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification165 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// Max140Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max256Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max256Text {
	#[serde(rename = "Max256Text")]
	pub max256_text: String,
}


// Max350Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// NonNegativeFraction5DecimalNumber ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NonNegativeFraction5DecimalNumber {
	#[serde(rename = "NonNegativeFraction5DecimalNumber")]
	pub non_negative_fraction5_decimal_number: f64,
}


// PlusOrMinusIndicator ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "PlusOrMinusIndicator")]
	pub plus_or_minus_indicator: bool,
}


// Position1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Position1 {
	#[serde(rename = "PdctId")]
	pub pdct_id: String,
	#[serde(rename = "RskRqrmnt")]
	pub rsk_rqrmnt: Option<EndOfDayRequirement1>,
	#[serde(rename = "GrssNtnl")]
	pub grss_ntnl: ActiveCurrencyAnd24Amount,
	#[serde(rename = "NetNtnl")]
	pub net_ntnl: AmountAndDirection102,
	#[serde(rename = "GrssDltaEqvtVal")]
	pub grss_dlta_eqvt_val: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "NetDltaEqvtVal")]
	pub net_dlta_eqvt_val: Option<AmountAndDirection102>,
	#[serde(rename = "GrssDltaEqvtQty")]
	pub grss_dlta_eqvt_qty: Option<f64>,
	#[serde(rename = "NetDltaEqvtQty")]
	pub net_dlta_eqvt_qty: Option<f64>,
	#[serde(rename = "GrssMktVal")]
	pub grss_mkt_val: ActiveCurrencyAndAmount,
}


// PositionAccount2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionAccount2 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification165,
	#[serde(rename = "Pos")]
	pub pos: Vec<Position1>,
}


// SchemeIdentificationType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SchemeIdentificationType1Code {
	#[serde(rename = "SchemeIdentificationType1Code")]
	pub scheme_identification_type1_code: String,
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