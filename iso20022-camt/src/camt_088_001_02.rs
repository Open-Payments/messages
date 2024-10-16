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


// AddressType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AddressType2Code {
	#[default]
	#[serde(rename = "ADDR")]
	CodeADDR,
	#[serde(rename = "PBOX")]
	CodePBOX,
	#[serde(rename = "HOME")]
	CodeHOME,
	#[serde(rename = "BIZZ")]
	CodeBIZZ,
	#[serde(rename = "MLTO")]
	CodeMLTO,
	#[serde(rename = "DLVY")]
	CodeDLVY,

}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
}


// ClearingSystemIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemIdentification2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalClearingSystemIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// ExternalClearingSystemIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalClearingSystemIdentification1Code {
	#[serde(rename = "$value")]
	pub external_clearing_system_identification1_code: String,
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


// ISOTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISOTime {
	#[serde(rename = "$value")]
	pub iso_time: String,
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


// Max10NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max10NumericText {
	#[serde(rename = "$value")]
	pub max10_numeric_text: String,
}


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max16Text {
	#[serde(rename = "$value")]
	pub max16_text: String,
}


// Max34Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max34Text {
	#[serde(rename = "$value")]
	pub max34_text: String,
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


// Max5NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max5NumericText {
	#[serde(rename = "$value")]
	pub max5_numeric_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
}


// NameAndAddress8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress8 {
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<PostalAddress1>,
	#[serde(rename = "AltrntvIdr", skip_serializing_if = "Option::is_none")]
	pub altrntv_idr: Option<Vec<Max35Text>>,
}


// NetObligation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NetObligation2 {
	#[serde(rename = "OblgtnId")]
	pub oblgtn_id: Max35Text,
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
	#[serde(rename = "PtcptNetgId")]
	pub ptcpt_netg_id: NettingIdentification2Choice,
	#[serde(rename = "OblgtnDrctn")]
	pub oblgtn_drctn: PaymentReceipt1Code,
	#[serde(rename = "CtrPtyNetgId")]
	pub ctr_pty_netg_id: NettingIdentification2Choice,
	#[serde(rename = "NetSvcCtrPtyId", skip_serializing_if = "Option::is_none")]
	pub net_svc_ctr_pty_id: Option<PartyIdentification242Choice>,
	#[serde(rename = "CtrPtySttlmInstrs", skip_serializing_if = "Option::is_none")]
	pub ctr_pty_sttlm_instrs: Option<SettlementParties120>,
	#[serde(rename = "TxsNb", skip_serializing_if = "Option::is_none")]
	pub txs_nb: Option<Max10NumericText>,
}


// NetReportData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NetReportData2 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
	#[serde(rename = "NetgCutOffTm")]
	pub netg_cut_off_tm: String,
	#[serde(rename = "RptDt")]
	pub rpt_dt: String,
	#[serde(rename = "ValDt")]
	pub val_dt: String,
	#[serde(rename = "RptTp", skip_serializing_if = "Option::is_none")]
	pub rpt_tp: Option<Max35Text>,
	#[serde(rename = "NetRptSvcr", skip_serializing_if = "Option::is_none")]
	pub net_rpt_svcr: Option<PartyIdentification242Choice>,
	#[serde(rename = "NetSvcTp", skip_serializing_if = "Option::is_none")]
	pub net_svc_tp: Option<Max35Text>,
	#[serde(rename = "MsgPgntn", skip_serializing_if = "Option::is_none")]
	pub msg_pgntn: Option<Pagination1>,
}


// NetReportV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NetReportV02 {
	#[serde(rename = "NetRptData")]
	pub net_rpt_data: NetReportData2,
	#[serde(rename = "NetSvcPtcptId")]
	pub net_svc_ptcpt_id: PartyIdentification242Choice,
	#[serde(rename = "NetSvcCtrPtyId", skip_serializing_if = "Option::is_none")]
	pub net_svc_ctr_pty_id: Option<PartyIdentification242Choice>,
	#[serde(rename = "NetOblgtn")]
	pub net_oblgtn: Vec<NetObligation2>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// NettingIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NettingIdentification2Choice {
	#[serde(rename = "TradPty", skip_serializing_if = "Option::is_none")]
	pub trad_pty: Option<PartyIdentification242Choice>,
	#[serde(rename = "NetgGrpId", skip_serializing_if = "Option::is_none")]
	pub netg_grp_id: Option<Max35Text>,
}


// Pagination1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Pagination1 {
	#[serde(rename = "PgNb")]
	pub pg_nb: Max5NumericText,
	#[serde(rename = "LastPgInd")]
	pub last_pg_ind: bool,
}


// PartyIdentification242Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification242Choice {
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress8>,
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<PartyIdentification265>,
	#[serde(rename = "PtyId", skip_serializing_if = "Option::is_none")]
	pub pty_id: Option<PartyIdentification266>,
}


// PartyIdentification265 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification265 {
	#[serde(rename = "AnyBIC")]
	pub any_bic: AnyBICDec2014Identifier,
	#[serde(rename = "AltrntvIdr", skip_serializing_if = "Option::is_none")]
	pub altrntv_idr: Option<Vec<Max35Text>>,
}


// PartyIdentification266 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification266 {
	#[serde(rename = "PtyNm", skip_serializing_if = "Option::is_none")]
	pub pty_nm: Option<Max34Text>,
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<PartyIdentification265>,
	#[serde(rename = "AcctNb", skip_serializing_if = "Option::is_none")]
	pub acct_nb: Option<Max34Text>,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<Max105Text>,
	#[serde(rename = "ClrSysId", skip_serializing_if = "Option::is_none")]
	pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
	#[serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none")]
	pub lgl_ntty_idr: Option<LEIIdentifier>,
}


// PaymentReceipt1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PaymentReceipt1Code {
	#[default]
	#[serde(rename = "PAYM")]
	CodePAYM,
	#[serde(rename = "RECE")]
	CodeRECE,
	#[serde(rename = "NONE")]
	CodeNONE,

}


// PostalAddress1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress1 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<AddressType2Code>,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
	pub adr_line: Option<Vec<Max70Text>>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<Max70Text>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<Max16Text>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<Max16Text>,
	#[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
	pub twn_nm: Option<Max35Text>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<Max35Text>,
	#[serde(rename = "Ctry")]
	pub ctry: CountryCode,
}


// SettlementParties120 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementParties120 {
	#[serde(rename = "DlvryAgt", skip_serializing_if = "Option::is_none")]
	pub dlvry_agt: Option<PartyIdentification242Choice>,
	#[serde(rename = "Intrmy", skip_serializing_if = "Option::is_none")]
	pub intrmy: Option<PartyIdentification242Choice>,
	#[serde(rename = "RcvgAgt")]
	pub rcvg_agt: PartyIdentification242Choice,
	#[serde(rename = "BnfcryInstn", skip_serializing_if = "Option::is_none")]
	pub bnfcry_instn: Option<PartyIdentification242Choice>,
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


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "$value")]
	pub yes_no_indicator: bool,
}
