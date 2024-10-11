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


// AccountIdentification26 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountIdentification26 {
	#[serde(rename = "Prtry")]
	pub prtry: SimpleIdentificationInformation4,
}


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


// AnyBICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICIdentifier {
	#[serde(rename = "AnyBICIdentifier")]
	pub any_bic_identifier: String,
}


// CFIOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[serde(rename = "CFIOct2015Identifier")]
	pub cfi_oct2015_identifier: String,
}


// ClassificationType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClassificationType1Choice {
	#[serde(rename = "ClssfctnFinInstrm", skip_serializing_if = "Option::is_none")]
	pub clssfctn_fin_instrm: Option<String>,
	#[serde(rename = "AltrnClssfctn", skip_serializing_if = "Option::is_none")]
	pub altrn_clssfctn: Option<GenericIdentification1>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// EffectiveDate1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EffectiveDate1 {
	#[serde(rename = "FctvDt")]
	pub fctv_dt: String,
	#[serde(rename = "FctvDtParam", skip_serializing_if = "Option::is_none")]
	pub fctv_dt_param: Option<String>,
}


// ExternalEffectiveDateParameter1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalEffectiveDateParameter1Code {
	#[serde(rename = "ExternalEffectiveDateParameter1Code")]
	pub external_effective_date_parameter1_code: String,
}


// ExternalMarketArea1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalMarketArea1Code {
	#[serde(rename = "ExternalMarketArea1Code")]
	pub external_market_area1_code: String,
}


// ExternalSecuritiesPurpose1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalSecuritiesPurpose1Code {
	#[serde(rename = "ExternalSecuritiesPurpose1Code")]
	pub external_securities_purpose1_code: String,
}


// GenericIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// MarketIdentification87 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketIdentification87 {
	#[serde(rename = "Ctry")]
	pub ctry: String,
	#[serde(rename = "ClssfctnTp")]
	pub clssfctn_tp: ClassificationType1Choice,
	#[serde(rename = "SttlmPurp", skip_serializing_if = "Option::is_none")]
	pub sttlm_purp: Option<Purpose3Choice>,
}


// MarketIdentificationOrCashPurpose1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketIdentificationOrCashPurpose1Choice {
	#[serde(rename = "SttlmInstrMktId", skip_serializing_if = "Option::is_none")]
	pub sttlm_instr_mkt_id: Option<MarketIdentification87>,
	#[serde(rename = "CshSSIPurp", skip_serializing_if = "Option::is_none")]
	pub csh_ssi_purp: Option<Vec<String>>,
}


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max16Text {
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
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


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// NameAndAddress5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<PostalAddress1>,
}


// PartyIdentification63 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification63 {
	#[serde(rename = "PtyId")]
	pub pty_id: PartyIdentification75Choice,
	#[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
	pub prcg_id: Option<String>,
}


// PartyIdentification75Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification75Choice {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<String>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<String>,
}


// PartyOrCurrency1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyOrCurrency1Choice {
	#[serde(rename = "Dpstry", skip_serializing_if = "Option::is_none")]
	pub dpstry: Option<PartyIdentification63>,
	#[serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none")]
	pub sttlm_ccy: Option<String>,
}


// PostalAddress1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress1 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<String>,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
	pub adr_line: Option<Vec<String>>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<String>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<String>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<String>,
	#[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
	pub twn_nm: Option<String>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<String>,
	#[serde(rename = "Ctry")]
	pub ctry: String,
}


// Purpose3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Purpose3Choice {
	#[serde(rename = "SctiesPurpCd", skip_serializing_if = "Option::is_none")]
	pub scties_purp_cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification1>,
}


// SimpleIdentificationInformation4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SimpleIdentificationInformation4 {
	#[serde(rename = "Id")]
	pub id: String,
}


// StandingSettlementInstructionCancellationV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StandingSettlementInstructionCancellationV01 {
	#[serde(rename = "MsgRefId")]
	pub msg_ref_id: String,
	#[serde(rename = "FctvDtDtls", skip_serializing_if = "Option::is_none")]
	pub fctv_dt_dtls: Option<EffectiveDate1>,
	#[serde(rename = "AcctId")]
	pub acct_id: Vec<AccountIdentification26>,
	#[serde(rename = "MktId")]
	pub mkt_id: MarketIdentificationOrCashPurpose1Choice,
	#[serde(rename = "SttlmDtls")]
	pub sttlm_dtls: PartyOrCurrency1Choice,
	#[serde(rename = "PrvsMsgRef")]
	pub prvs_msg_ref: String,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
	pub plc_and_nm: Option<String>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}
