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


// AccountIdentification26 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountIdentification26 {
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: SimpleIdentificationInformation4,
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


// AnyBICIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AnyBICIdentifier {
	#[validate(pattern = "[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}")]
	#[serde(rename = "AnyBICIdentifier")]
	pub any_bic_identifier: String,
}


// BICFIIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BICFIIdentifier {
	#[validate(pattern = "[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}")]
	#[serde(rename = "BICFIIdentifier")]
	pub bicfi_identifier: String,
}


// CFIOct2015Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[validate(pattern = "[A-Z]{6,6}")]
	#[serde(rename = "CFIOct2015Identifier")]
	pub cfi_oct2015_identifier: String,
}


// CashParties24 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CashParties24 {
	#[validate]
	#[serde(rename = "Cdtr")]
	pub cdtr: PartyIdentificationAndAccount96,
	#[validate]
	#[serde(rename = "CdtrAgt")]
	pub cdtr_agt: PartyIdentificationAndAccount97,
	#[validate]
	#[serde(rename = "Intrmy")]
	pub intrmy: Option<PartyIdentificationAndAccount97>,
	#[validate]
	#[serde(rename = "Intrmy2")]
	pub intrmy2: Option<PartyIdentificationAndAccount97>,
}


// ClassificationType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ClassificationType1Choice {
	#[serde(rename = "ClssfctnFinInstrm")]
	pub clssfctn_fin_instrm: Option<String>,
	#[validate]
	#[serde(rename = "AltrnClssfctn")]
	pub altrn_clssfctn: Option<GenericIdentification1>,
}


// CountryCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CountryCode {
	#[validate(pattern = "[A-Z]{2,2}")]
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// EffectiveDate1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EffectiveDate1 {
	#[serde(rename = "FctvDt")]
	pub fctv_dt: String,
	#[serde(rename = "FctvDtParam")]
	pub fctv_dt_param: Option<String>,
}


// Exact4AlphaNumericText ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[validate(pattern = "[a-zA-Z0-9]{4}")]
	#[serde(rename = "Exact4AlphaNumericText")]
	pub exact4_alpha_numeric_text: String,
}


// ExternalEffectiveDateParameter1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalEffectiveDateParameter1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalEffectiveDateParameter1Code")]
	pub external_effective_date_parameter1_code: String,
}


// ExternalMarketArea1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalMarketArea1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalMarketArea1Code")]
	pub external_market_area1_code: String,
}


// ExternalSecuritiesPurpose1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalSecuritiesPurpose1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalSecuritiesPurpose1Code")]
	pub external_securities_purpose1_code: String,
}


// GenericIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericIdentification30 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification30 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// GenericIdentification36 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification36 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// GenericIdentification49 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification49 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "IdTp")]
	pub id_tp: String,
}


// ISODate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// MarketIdentification87 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MarketIdentification87 {
	#[serde(rename = "Ctry")]
	pub ctry: String,
	#[validate]
	#[serde(rename = "ClssfctnTp")]
	pub clssfctn_tp: ClassificationType1Choice,
	#[validate]
	#[serde(rename = "SttlmPurp")]
	pub sttlm_purp: Option<Purpose3Choice>,
}


// MarketIdentificationOrCashPurpose1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MarketIdentificationOrCashPurpose1Choice {
	#[validate]
	#[serde(rename = "SttlmInstrMktId")]
	pub sttlm_instr_mkt_id: Option<MarketIdentification87>,
	#[serde(rename = "CshSSIPurp")]
	pub csh_ssi_purp: Option<Vec<String>>,
}


// Max16Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max16Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 16)]
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
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


// Max70Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max70Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 70)]
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// NameAndAddress5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[validate]
	#[serde(rename = "Adr")]
	pub adr: Option<PostalAddress1>,
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


// PartyIdentification44 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification44 {
	#[serde(rename = "AnyBIC")]
	pub any_bic: String,
	#[serde(rename = "AltrntvIdr")]
	pub altrntv_idr: Option<Vec<String>>,
}


// PartyIdentification62 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification62 {
	#[serde(rename = "BICFI")]
	pub bicfi: Option<String>,
	#[validate]
	#[serde(rename = "PrtryId")]
	pub prtry_id: Option<GenericIdentification1>,
	#[validate]
	#[serde(rename = "NmAndAdr")]
	pub nm_and_adr: Option<NameAndAddress5>,
}


// PartyIdentification63 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification63 {
	#[validate]
	#[serde(rename = "PtyId")]
	pub pty_id: PartyIdentification75Choice,
	#[serde(rename = "PrcgId")]
	pub prcg_id: Option<String>,
}


// PartyIdentification64 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification64 {
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
	#[validate]
	#[serde(rename = "PrtryId")]
	pub prtry_id: Option<GenericIdentification1>,
	#[validate]
	#[serde(rename = "NmAndAdr")]
	pub nm_and_adr: Option<NameAndAddress5>,
}


// PartyIdentification71Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification71Choice {
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
	#[validate]
	#[serde(rename = "PrtryId")]
	pub prtry_id: Option<GenericIdentification36>,
	#[validate]
	#[serde(rename = "NmAndAdr")]
	pub nm_and_adr: Option<NameAndAddress5>,
}


// PartyIdentification75Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification75Choice {
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
	#[validate]
	#[serde(rename = "NmAndAdr")]
	pub nm_and_adr: Option<NameAndAddress5>,
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
}


// PartyIdentification99Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification99Choice {
	#[validate]
	#[serde(rename = "NmAndAdr")]
	pub nm_and_adr: Option<NameAndAddress8>,
	#[validate]
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<PartyIdentification44>,
}


// PartyIdentificationAndAccount95 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentificationAndAccount95 {
	#[validate]
	#[serde(rename = "PtyId")]
	pub pty_id: PartyIdentification71Choice,
	#[validate]
	#[serde(rename = "AcctId")]
	pub acct_id: Option<SecuritiesAccount22>,
	#[serde(rename = "PrcgId")]
	pub prcg_id: Option<String>,
}


// PartyIdentificationAndAccount96 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentificationAndAccount96 {
	#[validate]
	#[serde(rename = "PtyId")]
	pub pty_id: PartyIdentification64,
	#[validate]
	#[serde(rename = "AcctId")]
	pub acct_id: AccountIdentification26,
}


// PartyIdentificationAndAccount97 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentificationAndAccount97 {
	#[validate]
	#[serde(rename = "PtyId")]
	pub pty_id: PartyIdentification62,
	#[validate]
	#[serde(rename = "AcctId")]
	pub acct_id: Option<AccountIdentification26>,
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


// Purpose3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Purpose3Choice {
	#[serde(rename = "SctiesPurpCd")]
	pub scties_purp_cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification1>,
}


// SecuritiesAccount22 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesAccount22 {
	#[serde(rename = "Id")]
	pub id: String,
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: Option<GenericIdentification30>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
}


// SecuritiesOrCash1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesOrCash1Choice {
	#[validate]
	#[serde(rename = "SctiesDtls")]
	pub scties_dtls: Option<SettlementParties35>,
	#[validate]
	#[serde(rename = "CshPtiesDtls")]
	pub csh_pties_dtls: Option<CashParties24>,
}


// SettlementParties32 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementParties32 {
	#[validate]
	#[serde(rename = "Dpstry")]
	pub dpstry: PartyIdentification63,
	#[validate]
	#[serde(rename = "Pty1")]
	pub pty1: Option<PartyIdentificationAndAccount95>,
	#[validate]
	#[serde(rename = "Pty2")]
	pub pty2: Option<PartyIdentificationAndAccount95>,
	#[validate]
	#[serde(rename = "Pty3")]
	pub pty3: Option<PartyIdentificationAndAccount95>,
	#[validate]
	#[serde(rename = "Pty4")]
	pub pty4: Option<PartyIdentificationAndAccount95>,
	#[validate]
	#[serde(rename = "Pty5")]
	pub pty5: Option<PartyIdentificationAndAccount95>,
}


// SettlementParties35 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementParties35 {
	#[validate]
	#[serde(rename = "StgSttlmPties")]
	pub stg_sttlm_pties: SettlementParties32,
	#[validate]
	#[serde(rename = "LclMktId")]
	pub lcl_mkt_id: Option<Vec<GenericIdentification49>>,
	#[validate]
	#[serde(rename = "RegnDtls")]
	pub regn_dtls: Option<PartyIdentification99Choice>,
}


// SimpleIdentificationInformation4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SimpleIdentificationInformation4 {
	#[serde(rename = "Id")]
	pub id: String,
}


// StandingSettlementInstructionV01 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct StandingSettlementInstructionV01 {
	#[serde(rename = "MsgRefId")]
	pub msg_ref_id: String,
	#[validate]
	#[serde(rename = "FctvDtDtls")]
	pub fctv_dt_dtls: Option<EffectiveDate1>,
	#[validate]
	#[serde(rename = "AcctId")]
	pub acct_id: Vec<AccountIdentification26>,
	#[validate]
	#[serde(rename = "MktId")]
	pub mkt_id: MarketIdentificationOrCashPurpose1Choice,
	#[serde(rename = "SttlmCcy")]
	pub sttlm_ccy: Option<String>,
	#[validate]
	#[serde(rename = "SttlmDtls")]
	pub sttlm_dtls: SecuritiesOrCash1Choice,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
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
