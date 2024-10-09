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


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// AddressType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType2Code {
	#[serde(rename = "AddressType2Code")]
	pub address_type2_code: String,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// ClearingSystemIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemIdentification2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// CutOff1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CutOff1 {
	#[serde(rename = "CutOffUpdId")]
	pub cut_off_upd_id: String,
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "CutOffTm")]
	pub cut_off_tm: String,
	#[serde(rename = "ValDtOffset")]
	pub val_dt_offset: String,
}


// DateOffsetText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateOffsetText {
	#[serde(rename = "DateOffsetText")]
	pub date_offset_text: String,
}


// ExternalClearingSystemIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalClearingSystemIdentification1Code {
	#[serde(rename = "ExternalClearingSystemIdentification1Code")]
	pub external_clearing_system_identification1_code: String,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// ISOTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISOTime {
	#[serde(rename = "ISOTime")]
	pub iso_time: String,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// Max105Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max105Text {
	#[serde(rename = "Max105Text")]
	pub max105_text: String,
}


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max16Text {
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
}


// Max34Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max34Text {
	#[serde(rename = "Max34Text")]
	pub max34_text: String,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Max4Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4Text {
	#[serde(rename = "Max4Text")]
	pub max4_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// NameAndAddress8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress8 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Adr")]
	pub adr: Option<PostalAddress1>,
	#[serde(rename = "AltrntvIdr")]
	pub altrntv_idr: Option<Vec<String>>,
}


// NettingCutOff2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NettingCutOff2 {
	#[serde(rename = "NetgId")]
	pub netg_id: NettingIdentification2Choice,
	#[serde(rename = "NewCutOff")]
	pub new_cut_off: Vec<CutOff1>,
}


// NettingCutOffReferenceDataUpdateRequestV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NettingCutOffReferenceDataUpdateRequestV02 {
	#[serde(rename = "ReqData")]
	pub req_data: RequestData2,
	#[serde(rename = "NetgCutOffReq")]
	pub netg_cut_off_req: Vec<NettingCutOff2>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// NettingIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NettingIdentification2Choice {
	#[serde(rename = "TradPty")]
	pub trad_pty: Option<PartyIdentification242Choice>,
	#[serde(rename = "NetgGrpId")]
	pub netg_grp_id: Option<String>,
}


// PartyIdentification242Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification242Choice {
	#[serde(rename = "NmAndAdr")]
	pub nm_and_adr: Option<NameAndAddress8>,
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<PartyIdentification265>,
	#[serde(rename = "PtyId")]
	pub pty_id: Option<PartyIdentification266>,
}


// PartyIdentification265 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification265 {
	#[serde(rename = "AnyBIC")]
	pub any_bic: String,
	#[serde(rename = "AltrntvIdr")]
	pub altrntv_idr: Option<Vec<String>>,
}


// PartyIdentification266 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification266 {
	#[serde(rename = "PtyNm")]
	pub pty_nm: Option<String>,
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<PartyIdentification265>,
	#[serde(rename = "AcctNb")]
	pub acct_nb: Option<String>,
	#[serde(rename = "Adr")]
	pub adr: Option<String>,
	#[serde(rename = "ClrSysId")]
	pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
	#[serde(rename = "LglNttyIdr")]
	pub lgl_ntty_idr: Option<String>,
}


// PostalAddress1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
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


// RequestData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestData2 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "ReqTp")]
	pub req_tp: String,
	#[serde(rename = "ReqdActvtnDt")]
	pub reqd_actvtn_dt: String,
	#[serde(rename = "ReqSvcr")]
	pub req_svcr: Option<PartyIdentification242Choice>,
	#[serde(rename = "NetSvcPtcptId")]
	pub net_svc_ptcpt_id: PartyIdentification242Choice,
	#[serde(rename = "NetSvcTp")]
	pub net_svc_tp: Option<String>,
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Option<String>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}
