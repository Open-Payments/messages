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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd24AmountSimpleType {
	#[serde(rename = "$value")]
	pub active_currency_and24_amount_simple_type: f64,
}


// ActiveCurrencyAnd24Amount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd24Amount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


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


// CCPInvestmentsReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CCPInvestmentsReportV01 {
	#[serde(rename = "Invstmt")]
	pub invstmt: Vec<Investment1Choice>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// Deposit1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Deposit1 {
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: String,
	#[serde(rename = "Val")]
	pub val: ActiveCurrencyAndAmount,
	#[serde(rename = "CtrPtyId")]
	pub ctr_pty_id: LEIIdentifier,
}


// FinancialInstrument59 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument59 {
	#[serde(rename = "Id")]
	pub id: ISINOct2015Identifier,
	#[serde(rename = "Issr")]
	pub issr: LEIIdentifier,
	#[serde(rename = "Sctr", skip_serializing_if = "Option::is_none")]
	pub sctr: Option<String>,
}


// GeneralCollateral3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GeneralCollateral3 {
	#[serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_id: Option<Vec<FinancialInstrument59>>,
	#[serde(rename = "ElgblFinInstrmId", skip_serializing_if = "Option::is_none")]
	pub elgbl_fin_instrm_id: Option<Vec<ISINOct2015Identifier>>,
}


// ISINOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "$value")]
	pub isin_oct2015_identifier: String,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}


// Investment1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Investment1Choice {
	#[serde(rename = "UscrdCshDpst", skip_serializing_if = "Option::is_none")]
	pub uscrd_csh_dpst: Option<Deposit1>,
	#[serde(rename = "CntrlBkDpst", skip_serializing_if = "Option::is_none")]
	pub cntrl_bk_dpst: Option<Deposit1>,
	#[serde(rename = "RpAgrmt", skip_serializing_if = "Option::is_none")]
	pub rp_agrmt: Option<RepurchaseAgreement2>,
	#[serde(rename = "OthrInvstmts", skip_serializing_if = "Option::is_none")]
	pub othr_invstmts: Option<OtherInvestment1>,
	#[serde(rename = "OutrghtInvstmt", skip_serializing_if = "Option::is_none")]
	pub outrght_invstmt: Option<SecurityIdentificationAndAmount1>,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}


// OtherInvestment1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherInvestment1 {
	#[serde(rename = "Desc")]
	pub desc: Max140Text,
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
}


// ProductType7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ProductType7Code {
	#[default]
	#[serde(rename = "SVGN")]
	CodeSVGN,
	#[serde(rename = "EQUI")]
	CodeEQUI,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}


// RepurchaseAgreement2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RepurchaseAgreement2 {
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: String,
	#[serde(rename = "ScndLegPric")]
	pub scnd_leg_pric: ActiveCurrencyAndAmount,
	#[serde(rename = "CollMktVal")]
	pub coll_mkt_val: ActiveCurrencyAndAmount,
	#[serde(rename = "CtrPty")]
	pub ctr_pty: LEIIdentifier,
	#[serde(rename = "RpAgrmtTp")]
	pub rp_agrmt_tp: RepurchaseAgreementType3Choice,
	#[serde(rename = "TrptyAgtId", skip_serializing_if = "Option::is_none")]
	pub trpty_agt_id: Option<LEIIdentifier>,
}


// RepurchaseAgreementType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RepurchaseAgreementType3Choice {
	#[serde(rename = "SpcfcColl", skip_serializing_if = "Option::is_none")]
	pub spcfc_coll: Option<SpecificCollateral2>,
	#[serde(rename = "GnlColl", skip_serializing_if = "Option::is_none")]
	pub gnl_coll: Option<GeneralCollateral3>,
}


// SNA2008SectorIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SNA2008SectorIdentifier {
	#[serde(rename = "$value")]
	pub sna2008_sector_identifier: String,
}


// SecurityIdentificationAndAmount1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentificationAndAmount1 {
	#[serde(rename = "Id")]
	pub id: ISINOct2015Identifier,
	#[serde(rename = "MktVal")]
	pub mkt_val: ActiveCurrencyAnd24Amount,
	#[serde(rename = "FinInstrmTp")]
	pub fin_instrm_tp: ProductType7Code,
}


// SpecificCollateral2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SpecificCollateral2 {
	#[serde(rename = "FinInstrmId")]
	pub fin_instrm_id: FinancialInstrument59,
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
