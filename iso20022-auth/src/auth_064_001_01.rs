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


// AvailableFinancialResourcesAmount1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AvailableFinancialResourcesAmount1 {
	#[serde(rename = "TtlInitlMrgn")]
	pub ttl_initl_mrgn: ActiveCurrencyAndAmount,
	#[serde(rename = "TtlPrfnddDfltFnd")]
	pub ttl_prfndd_dflt_fnd: ActiveCurrencyAndAmount,
	#[serde(rename = "CCPSkinInTheGame")]
	pub ccp_skin_in_the_game: Vec<ReportingAssetBreakdown1>,
	#[serde(rename = "OthrDfltFndCntrbtn")]
	pub othr_dflt_fnd_cntrbtn: ActiveCurrencyAndAmount,
	#[serde(rename = "UfnddMmbCmmtmnt")]
	pub ufndd_mmb_cmmtmnt: ActiveCurrencyAndAmount,
	#[serde(rename = "UfnddThrdPtyCmmtmnt")]
	pub ufndd_thrd_pty_cmmtmnt: ActiveCurrencyAndAmount,
}


// CCPAvailableFinancialResourcesReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CCPAvailableFinancialResourcesReportV01 {
	#[serde(rename = "AvlblFinRsrcsAmt")]
	pub avlbl_fin_rsrcs_amt: AvailableFinancialResourcesAmount1,
	#[serde(rename = "OthrPrfnddRsrcs", skip_serializing_if = "Option::is_none")]
	pub othr_prfndd_rsrcs: Option<ReportingAssetBreakdown1>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}


// ProductType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ProductType6Code {
	#[default]
	#[serde(rename = "BOND")]
	CodeBOND,
	#[serde(rename = "CASH")]
	CodeCASH,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "EQUI")]
	CodeEQUI,

}


// ReportingAssetBreakdown1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportingAssetBreakdown1 {
	#[serde(rename = "RptgAsstTp")]
	pub rptg_asst_tp: ProductType6Code,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max350Text>,
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
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
