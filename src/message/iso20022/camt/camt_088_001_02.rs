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
use serde_valid::Validate;


// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "ActiveCurrencyAndAmount_SimpleType")]
	pub active_currency_and_amount_simple_type: f64,
}


// ActiveCurrencyAndAmount ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveCurrencyCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[validate(pattern = "[A-Z]{3,3}")]
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// AddressType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AddressType2Code {
	#[validate(enumerate = ["ADDR", "PBOX", "HOME", "BIZZ", "MLTO", "DLVY"])]
	#[serde(rename = "AddressType2Code")]
	pub address_type2_code: String,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[validate(pattern = "[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}")]
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// ClearingSystemIdentification2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ClearingSystemIdentification2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// CountryCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CountryCode {
	#[validate(pattern = "[A-Z]{2,2}")]
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// ExternalClearingSystemIdentification1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalClearingSystemIdentification1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 5)]
	#[serde(rename = "ExternalClearingSystemIdentification1Code")]
	pub external_clearing_system_identification1_code: String,
}


// ISODate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// ISOTime ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISOTime {
	#[serde(rename = "ISOTime")]
	pub iso_time: String,
}


// LEIIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[validate(pattern = "[A-Z0-9]{18,18}[0-9]{2,2}")]
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// Max105Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max105Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 105)]
	#[serde(rename = "Max105Text")]
	pub max105_text: String,
}


// Max10NumericText ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max10NumericText {
	#[validate(pattern = "[0-9]{1,10}")]
	#[serde(rename = "Max10NumericText")]
	pub max10_numeric_text: String,
}


// Max16Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max16Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 16)]
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
}


// Max34Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max34Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 34)]
	#[serde(rename = "Max34Text")]
	pub max34_text: String,
}


// Max350Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max350Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 350)]
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max35Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 35)]
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Max5NumericText ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max5NumericText {
	#[validate(pattern = "[0-9]{1,5}")]
	#[serde(rename = "Max5NumericText")]
	pub max5_numeric_text: String,
}


// Max70Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max70Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 70)]
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// NameAndAddress8 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NameAndAddress8 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[validate]
	#[serde(rename = "Adr")]
	pub adr: Option<PostalAddress1>,
	#[serde(rename = "AltrntvIdr")]
	pub altrntv_idr: Option<Vec<String>>,
}


// NetObligation2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NetObligation2 {
	#[serde(rename = "OblgtnId")]
	pub oblgtn_id: String,
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
	#[validate]
	#[serde(rename = "PtcptNetgId")]
	pub ptcpt_netg_id: NettingIdentification2Choice,
	#[serde(rename = "OblgtnDrctn")]
	pub oblgtn_drctn: String,
	#[validate]
	#[serde(rename = "CtrPtyNetgId")]
	pub ctr_pty_netg_id: NettingIdentification2Choice,
	#[validate]
	#[serde(rename = "NetSvcCtrPtyId")]
	pub net_svc_ctr_pty_id: Option<PartyIdentification242Choice>,
	#[validate]
	#[serde(rename = "CtrPtySttlmInstrs")]
	pub ctr_pty_sttlm_instrs: Option<SettlementParties120>,
	#[serde(rename = "TxsNb")]
	pub txs_nb: Option<String>,
}


// NetReportData2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NetReportData2 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
	#[serde(rename = "NetgCutOffTm")]
	pub netg_cut_off_tm: String,
	#[serde(rename = "RptDt")]
	pub rpt_dt: String,
	#[serde(rename = "ValDt")]
	pub val_dt: String,
	#[serde(rename = "RptTp")]
	pub rpt_tp: Option<String>,
	#[validate]
	#[serde(rename = "NetRptSvcr")]
	pub net_rpt_svcr: Option<PartyIdentification242Choice>,
	#[serde(rename = "NetSvcTp")]
	pub net_svc_tp: Option<String>,
	#[validate]
	#[serde(rename = "MsgPgntn")]
	pub msg_pgntn: Option<Pagination1>,
}


// NetReportV02 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NetReportV02 {
	#[validate]
	#[serde(rename = "NetRptData")]
	pub net_rpt_data: NetReportData2,
	#[validate]
	#[serde(rename = "NetSvcPtcptId")]
	pub net_svc_ptcpt_id: PartyIdentification242Choice,
	#[validate]
	#[serde(rename = "NetSvcCtrPtyId")]
	pub net_svc_ctr_pty_id: Option<PartyIdentification242Choice>,
	#[validate]
	#[serde(rename = "NetOblgtn")]
	pub net_oblgtn: Vec<NetObligation2>,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// NettingIdentification2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NettingIdentification2Choice {
	#[validate]
	#[serde(rename = "TradPty")]
	pub trad_pty: Option<PartyIdentification242Choice>,
	#[serde(rename = "NetgGrpId")]
	pub netg_grp_id: Option<String>,
}


// Pagination1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Pagination1 {
	#[serde(rename = "PgNb")]
	pub pg_nb: String,
	#[serde(rename = "LastPgInd")]
	pub last_pg_ind: bool,
}


// PartyIdentification242Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification242Choice {
	#[validate]
	#[serde(rename = "NmAndAdr")]
	pub nm_and_adr: Option<NameAndAddress8>,
	#[validate]
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<PartyIdentification265>,
	#[validate]
	#[serde(rename = "PtyId")]
	pub pty_id: Option<PartyIdentification266>,
}


// PartyIdentification265 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification265 {
	#[serde(rename = "AnyBIC")]
	pub any_bic: String,
	#[serde(rename = "AltrntvIdr")]
	pub altrntv_idr: Option<Vec<String>>,
}


// PartyIdentification266 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification266 {
	#[serde(rename = "PtyNm")]
	pub pty_nm: Option<String>,
	#[validate]
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<PartyIdentification265>,
	#[serde(rename = "AcctNb")]
	pub acct_nb: Option<String>,
	#[serde(rename = "Adr")]
	pub adr: Option<String>,
	#[validate]
	#[serde(rename = "ClrSysId")]
	pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
	#[serde(rename = "LglNttyIdr")]
	pub lgl_ntty_idr: Option<String>,
}


// PaymentReceipt1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PaymentReceipt1Code {
	#[validate(enumerate = ["PAYM", "RECE", "NONE"])]
	#[serde(rename = "PaymentReceipt1Code")]
	pub payment_receipt1_code: String,
}


// PostalAddress1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PostalAddress1 {
	#[serde(rename = "AdrTp")]
	pub adr_tp: Option<String>,
	#[serde(rename = "AdrLine")]
	pub adr_line: Option<Vec<String>>,
	#[serde(rename = "StrtNm")]
	pub strt_nm: Option<String>,
	#[serde(rename = "BldgNb")]
	pub bldg_nb: Option<String>,
	#[serde(rename = "PstCd")]
	pub pst_cd: Option<String>,
	#[serde(rename = "TwnNm")]
	pub twn_nm: Option<String>,
	#[serde(rename = "CtrySubDvsn")]
	pub ctry_sub_dvsn: Option<String>,
	#[serde(rename = "Ctry")]
	pub ctry: String,
}


// SettlementParties120 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementParties120 {
	#[validate]
	#[serde(rename = "DlvryAgt")]
	pub dlvry_agt: Option<PartyIdentification242Choice>,
	#[validate]
	#[serde(rename = "Intrmy")]
	pub intrmy: Option<PartyIdentification242Choice>,
	#[validate]
	#[serde(rename = "RcvgAgt")]
	pub rcvg_agt: PartyIdentification242Choice,
	#[validate]
	#[serde(rename = "BnfcryInstn")]
	pub bnfcry_instn: Option<PartyIdentification242Choice>,
}


// SupplementaryData1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Option<String>,
	#[validate]
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}


// YesNoIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "YesNoIndicator")]
	pub yes_no_indicator: bool,
}
