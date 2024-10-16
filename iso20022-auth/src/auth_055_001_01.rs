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


// Amount3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Amount3 {
	#[serde(rename = "OrgnlAmt", skip_serializing_if = "Option::is_none")]
	pub orgnl_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "RptgAmt")]
	pub rptg_amt: ActiveCurrencyAndAmount,
}


// AmountAndDirection102 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection102 {
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
	#[serde(rename = "Sgn")]
	pub sgn: bool,
}


// CCPMemberRequirementsReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CCPMemberRequirementsReportV01 {
	#[serde(rename = "IntraDayRqrmntAmt")]
	pub intra_day_rqrmnt_amt: Vec<IntraDayRequirement1>,
	#[serde(rename = "IntraDayMrgnCall", skip_serializing_if = "Option::is_none")]
	pub intra_day_mrgn_call: Option<Vec<IntraDayMarginCall1>>,
	#[serde(rename = "EndOfDayRqrmnt")]
	pub end_of_day_rqrmnt: Vec<EndOfDayRequirement2>,
	#[serde(rename = "DfltFndRqrmnt")]
	pub dflt_fnd_rqrmnt: Vec<DefaultFundRequirement1>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// DefaultFundRequirement1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DefaultFundRequirement1 {
	#[serde(rename = "ClrMmbId")]
	pub clr_mmb_id: GenericIdentification165,
	#[serde(rename = "SvcId", skip_serializing_if = "Option::is_none")]
	pub svc_id: Option<Max35Text>,
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
}


// EndOfDayRequirement2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EndOfDayRequirement2 {
	#[serde(rename = "InitlMrgnRqrmnts")]
	pub initl_mrgn_rqrmnts: InitialMarginRequirement1,
	#[serde(rename = "VartnMrgnRqrmnts")]
	pub vartn_mrgn_rqrmnts: AmountAndDirection102,
	#[serde(rename = "MrgnAcctId")]
	pub mrgn_acct_id: GenericIdentification165,
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


// GenericIdentification36 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification36 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Issr")]
	pub issr: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}


// InitialMarginExposure1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InitialMarginExposure1 {
	#[serde(rename = "Amt")]
	pub amt: Amount3,
	#[serde(rename = "Tp")]
	pub tp: MarginType2Choice,
	#[serde(rename = "CoreInd")]
	pub core_ind: bool,
}


// InitialMarginRequirement1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InitialMarginRequirement1 {
	#[serde(rename = "InitlMrgnXpsr")]
	pub initl_mrgn_xpsr: Vec<InitialMarginExposure1>,
	#[serde(rename = "Cdt")]
	pub cdt: ActiveCurrencyAndAmount,
}


// IntraDayMarginCall1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IntraDayMarginCall1 {
	#[serde(rename = "MrgnAcctId")]
	pub mrgn_acct_id: GenericIdentification165,
	#[serde(rename = "IntraDayCall")]
	pub intra_day_call: ActiveCurrencyAndAmount,
	#[serde(rename = "TmStmp")]
	pub tm_stmp: String,
}


// IntraDayRequirement1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IntraDayRequirement1 {
	#[serde(rename = "IntraDayMrgnCall")]
	pub intra_day_mrgn_call: ActiveCurrencyAndAmount,
	#[serde(rename = "PeakInitlMrgnLblty")]
	pub peak_initl_mrgn_lblty: ActiveCurrencyAndAmount,
	#[serde(rename = "PeakVartnMrgnLblty")]
	pub peak_vartn_mrgn_lblty: ActiveCurrencyAndAmount,
	#[serde(rename = "AggtPeakLblty")]
	pub aggt_peak_lblty: ActiveCurrencyAndAmount,
	#[serde(rename = "MrgnAcctId")]
	pub mrgn_acct_id: GenericIdentification165,
}


// MarginType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarginType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<MarginType2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}


// MarginType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum MarginType2Code {
	#[default]
	#[serde(rename = "ADFM")]
	CodeADFM,
	#[serde(rename = "COMA")]
	CodeCOMA,
	#[serde(rename = "CEMA")]
	CodeCEMA,
	#[serde(rename = "SEMA")]
	CodeSEMA,
	#[serde(rename = "SCMA")]
	CodeSCMA,
	#[serde(rename = "UFMA")]
	CodeUFMA,
	#[serde(rename = "MARM")]
	CodeMARM,
	#[serde(rename = "SORM")]
	CodeSORM,
	#[serde(rename = "WWRM")]
	CodeWWRM,
	#[serde(rename = "BARM")]
	CodeBARM,
	#[serde(rename = "LIRM")]
	CodeLIRM,
	#[serde(rename = "CRAM")]
	CodeCRAM,
	#[serde(rename = "CVMA")]
	CodeCVMA,
	#[serde(rename = "SPMA")]
	CodeSPMA,
	#[serde(rename = "JTDR")]
	CodeJTDR,
	#[serde(rename = "DRAO")]
	CodeDRAO,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
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
