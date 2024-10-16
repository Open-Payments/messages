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


// ActiveOrHistoricCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_and_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_code: String,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
}


// CollateralMarginNew10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralMarginNew10 {
	#[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
	pub tech_rcrd_id: Option<Max140Text>,
	#[serde(rename = "RptgDtTm")]
	pub rptg_dt_tm: String,
	#[serde(rename = "EvtDt")]
	pub evt_dt: String,
	#[serde(rename = "CtrPty")]
	pub ctr_pty: Counterparty39,
	#[serde(rename = "CollPrtflId")]
	pub coll_prtfl_id: Max52Text,
	#[serde(rename = "PstdMrgnOrColl", skip_serializing_if = "Option::is_none")]
	pub pstd_mrgn_or_coll: Option<PostedMarginOrCollateral4>,
	#[serde(rename = "RcvdMrgnOrColl", skip_serializing_if = "Option::is_none")]
	pub rcvd_mrgn_or_coll: Option<ReceivedMarginOrCollateral4>,
	#[serde(rename = "RcncltnFlg", skip_serializing_if = "Option::is_none")]
	pub rcncltn_flg: Option<ReconciliationFlag2>,
	#[serde(rename = "CtrctMod")]
	pub ctrct_mod: ContractModification3,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// ContractModification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractModification3 {
	#[serde(rename = "ActnTp")]
	pub actn_tp: TransactionOperationType6Code,
	#[serde(rename = "Lvl", skip_serializing_if = "Option::is_none")]
	pub lvl: Option<ModificationLevel1Code>,
}


// Counterparty39 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Counterparty39 {
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: OrganisationIdentification15Choice,
	#[serde(rename = "OthrCtrPty")]
	pub othr_ctr_pty: PartyIdentification236Choice,
	#[serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none")]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "RptSubmitgNtty", skip_serializing_if = "Option::is_none")]
	pub rpt_submitg_ntty: Option<OrganisationIdentification15Choice>,
}


// GenericIdentification175 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification175 {
	#[serde(rename = "Id")]
	pub id: Max72Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}


// Max105Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max105Text {
	#[serde(rename = "$value")]
	pub max105_text: String,
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


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}


// Max500Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max500Text {
	#[serde(rename = "$value")]
	pub max500_text: String,
}


// Max52Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max52Text {
	#[serde(rename = "$value")]
	pub max52_text: String,
}


// Max72Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max72Text {
	#[serde(rename = "$value")]
	pub max72_text: String,
}


// ModificationLevel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ModificationLevel1Code {
	#[default]
	#[serde(rename = "PSTN")]
	CodePSTN,
	#[serde(rename = "TCTN")]
	CodeTCTN,

}


// NaturalPersonIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NaturalPersonIdentification2 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max105Text>,
	#[serde(rename = "Dmcl", skip_serializing_if = "Option::is_none")]
	pub dmcl: Option<Max500Text>,
}


// OrganisationIdentification15Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification15Choice {
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<OrganisationIdentification38>,
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
}


// OrganisationIdentification38 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification38 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max105Text>,
	#[serde(rename = "Dmcl", skip_serializing_if = "Option::is_none")]
	pub dmcl: Option<Max500Text>,
}


// PartyIdentification236Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification236Choice {
	#[serde(rename = "Lgl", skip_serializing_if = "Option::is_none")]
	pub lgl: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "Ntrl", skip_serializing_if = "Option::is_none")]
	pub ntrl: Option<NaturalPersonIdentification2>,
}


// PostedMarginOrCollateral4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostedMarginOrCollateral4 {
	#[serde(rename = "InitlMrgnPstd", skip_serializing_if = "Option::is_none")]
	pub initl_mrgn_pstd: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "VartnMrgnPstd", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_pstd: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "XcssCollPstd", skip_serializing_if = "Option::is_none")]
	pub xcss_coll_pstd: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// ReceivedMarginOrCollateral4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReceivedMarginOrCollateral4 {
	#[serde(rename = "InitlMrgnRcvd", skip_serializing_if = "Option::is_none")]
	pub initl_mrgn_rcvd: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "VartnMrgnRcvd", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_rcvd: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "XcssCollRcvd", skip_serializing_if = "Option::is_none")]
	pub xcss_coll_rcvd: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// ReconciliationFlag2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReconciliationFlag2 {
	#[serde(rename = "RptTp", skip_serializing_if = "Option::is_none")]
	pub rpt_tp: Option<TradeRepositoryReportingType1Code>,
	#[serde(rename = "BothCtrPtiesRptg", skip_serializing_if = "Option::is_none")]
	pub both_ctr_pties_rptg: Option<bool>,
	#[serde(rename = "PairdSts", skip_serializing_if = "Option::is_none")]
	pub paird_sts: Option<bool>,
	#[serde(rename = "LnRcncltnSts", skip_serializing_if = "Option::is_none")]
	pub ln_rcncltn_sts: Option<bool>,
	#[serde(rename = "CollRcncltnSts", skip_serializing_if = "Option::is_none")]
	pub coll_rcncltn_sts: Option<bool>,
	#[serde(rename = "ModSts", skip_serializing_if = "Option::is_none")]
	pub mod_sts: Option<bool>,
}


// ReportPeriodActivity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ReportPeriodActivity1Code {
	#[default]
	#[serde(rename = "NOTX")]
	CodeNOTX,

}


// SecuritiesFinancingReportingMarginDataTransactionStateReportV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesFinancingReportingMarginDataTransactionStateReportV02 {
	#[serde(rename = "TradData")]
	pub trad_data: TradeData38Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
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


// TradeData38Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeData38Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[serde(rename = "Stat", skip_serializing_if = "Option::is_none")]
	pub stat: Option<Vec<CollateralMarginNew10>>,
}


// TradeRepositoryReportingType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TradeRepositoryReportingType1Code {
	#[default]
	#[serde(rename = "SWOS")]
	CodeSWOS,
	#[serde(rename = "TWOS")]
	CodeTWOS,

}


// TransactionOperationType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TransactionOperationType6Code {
	#[default]
	#[serde(rename = "REUU")]
	CodeREUU,
	#[serde(rename = "COLU")]
	CodeCOLU,
	#[serde(rename = "CORR")]
	CodeCORR,
	#[serde(rename = "ETRM")]
	CodeETRM,
	#[serde(rename = "VALU")]
	CodeVALU,
	#[serde(rename = "POSC")]
	CodePOSC,
	#[serde(rename = "NEWT")]
	CodeNEWT,
	#[serde(rename = "MODI")]
	CodeMODI,
	#[serde(rename = "MARU")]
	CodeMARU,
	#[serde(rename = "EROR")]
	CodeEROR,

}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}
