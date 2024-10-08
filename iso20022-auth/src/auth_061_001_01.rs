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


// CCPInvestmentsReportV01 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CCPInvestmentsReportV01 {
	#[serde(rename = "Invstmt")]
	pub invstmt: Vec<Investment1Choice>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// Deposit1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Deposit1 {
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: String,
	#[serde(rename = "Val")]
	pub val: ActiveCurrencyAndAmount,
	#[serde(rename = "CtrPtyId")]
	pub ctr_pty_id: String,
}


// FinancialInstrument59 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument59 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "Sctr")]
	pub sctr: Option<String>,
}


// GeneralCollateral3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GeneralCollateral3 {
	#[serde(rename = "FinInstrmId")]
	pub fin_instrm_id: Option<Vec<FinancialInstrument59>>,
	#[serde(rename = "ElgblFinInstrmId")]
	pub elgbl_fin_instrm_id: Option<Vec<String>>,
}


// ISINOct2015Identifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "ISINOct2015Identifier")]
	pub isin_oct2015_identifier: String,
}


// ISODate ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// Investment1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Investment1Choice {
	#[serde(rename = "UscrdCshDpst")]
	pub uscrd_csh_dpst: Option<Deposit1>,
	#[serde(rename = "CntrlBkDpst")]
	pub cntrl_bk_dpst: Option<Deposit1>,
	#[serde(rename = "RpAgrmt")]
	pub rp_agrmt: Option<RepurchaseAgreement2>,
	#[serde(rename = "OthrInvstmts")]
	pub othr_invstmts: Option<OtherInvestment1>,
	#[serde(rename = "OutrghtInvstmt")]
	pub outrght_invstmt: Option<SecurityIdentificationAndAmount1>,
}


// LEIIdentifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// Max140Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max350Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// OtherInvestment1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherInvestment1 {
	#[serde(rename = "Desc")]
	pub desc: String,
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
}


// ProductType7Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProductType7Code {
	#[serde(rename = "ProductType7Code")]
	pub product_type7_code: String,
}


// RepurchaseAgreement2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RepurchaseAgreement2 {
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: String,
	#[serde(rename = "ScndLegPric")]
	pub scnd_leg_pric: ActiveCurrencyAndAmount,
	#[serde(rename = "CollMktVal")]
	pub coll_mkt_val: ActiveCurrencyAndAmount,
	#[serde(rename = "CtrPty")]
	pub ctr_pty: String,
	#[serde(rename = "RpAgrmtTp")]
	pub rp_agrmt_tp: RepurchaseAgreementType3Choice,
	#[serde(rename = "TrptyAgtId")]
	pub trpty_agt_id: Option<String>,
}


// RepurchaseAgreementType3Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RepurchaseAgreementType3Choice {
	#[serde(rename = "SpcfcColl")]
	pub spcfc_coll: Option<SpecificCollateral2>,
	#[serde(rename = "GnlColl")]
	pub gnl_coll: Option<GeneralCollateral3>,
}


// SNA2008SectorIdentifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SNA2008SectorIdentifier {
	#[serde(rename = "SNA2008SectorIdentifier")]
	pub sna2008_sector_identifier: String,
}


// SecurityIdentificationAndAmount1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentificationAndAmount1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "MktVal")]
	pub mkt_val: ActiveCurrencyAnd24Amount,
	#[serde(rename = "FinInstrmTp")]
	pub fin_instrm_tp: String,
}


// SpecificCollateral2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SpecificCollateral2 {
	#[serde(rename = "FinInstrmId")]
	pub fin_instrm_id: FinancialInstrument59,
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
