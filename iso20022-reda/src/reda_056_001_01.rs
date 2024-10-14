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
	#[serde(rename = "$value")]
	pub active_currency_code: String,
}


// AddressType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AddressType2Code {
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

	#[default]
	UNKOWN
}


// AnyBICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICIdentifier {
	#[serde(rename = "$value")]
	pub any_bic_identifier: String,
}


// BICFIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BICFIIdentifier {
	#[serde(rename = "$value")]
	pub bicfi_identifier: String,
}


// CFIOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[serde(rename = "$value")]
	pub cfi_oct2015_identifier: String,
}


// CashParties24 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashParties24 {
	#[serde(rename = "Cdtr")]
	pub cdtr: PartyIdentificationAndAccount96,
	#[serde(rename = "CdtrAgt")]
	pub cdtr_agt: PartyIdentificationAndAccount97,
	#[serde(rename = "Intrmy", skip_serializing_if = "Option::is_none")]
	pub intrmy: Option<PartyIdentificationAndAccount97>,
	#[serde(rename = "Intrmy2", skip_serializing_if = "Option::is_none")]
	pub intrmy2: Option<PartyIdentificationAndAccount97>,
}


// ClassificationType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClassificationType1Choice {
	#[serde(rename = "ClssfctnFinInstrm", skip_serializing_if = "Option::is_none")]
	pub clssfctn_fin_instrm: Option<CFIOct2015Identifier>,
	#[serde(rename = "AltrnClssfctn", skip_serializing_if = "Option::is_none")]
	pub altrn_clssfctn: Option<GenericIdentification1>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// EffectiveDate1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EffectiveDate1 {
	#[serde(rename = "FctvDt")]
	pub fctv_dt: String,
	#[serde(rename = "FctvDtParam", skip_serializing_if = "Option::is_none")]
	pub fctv_dt_param: Option<ExternalEffectiveDateParameter1Code>,
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "$value")]
	pub exact4_alpha_numeric_text: String,
}


// ExternalEffectiveDateParameter1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalEffectiveDateParameter1Code {
	#[serde(rename = "$value")]
	pub external_effective_date_parameter1_code: String,
}


// ExternalMarketArea1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalMarketArea1Code {
	#[serde(rename = "$value")]
	pub external_market_area1_code: String,
}


// ExternalSecuritiesPurpose1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalSecuritiesPurpose1Code {
	#[serde(rename = "$value")]
	pub external_securities_purpose1_code: String,
}


// GenericIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}


// GenericIdentification30 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification30 {
	#[serde(rename = "Id")]
	pub id: Exact4AlphaNumericText,
	#[serde(rename = "Issr")]
	pub issr: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
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


// GenericIdentification49 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification49 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "IdTp")]
	pub id_tp: Max35Text,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}


// MarketIdentification87 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketIdentification87 {
	#[serde(rename = "Ctry")]
	pub ctry: CountryCode,
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
	pub csh_ssi_purp: Option<Vec<ExternalMarketArea1Code>>,
}


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max16Text {
	#[serde(rename = "$value")]
	pub max16_text: String,
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


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
}


// NameAndAddress5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<PostalAddress1>,
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


// PartyIdentification44 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification44 {
	#[serde(rename = "AnyBIC")]
	pub any_bic: AnyBICIdentifier,
	#[serde(rename = "AltrntvIdr", skip_serializing_if = "Option::is_none")]
	pub altrntv_idr: Option<Vec<Max35Text>>,
}


// PartyIdentification62 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification62 {
	#[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
	pub bicfi: Option<BICFIIdentifier>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification1>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
}


// PartyIdentification63 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification63 {
	#[serde(rename = "PtyId")]
	pub pty_id: PartyIdentification75Choice,
	#[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
	pub prcg_id: Option<Max35Text>,
}


// PartyIdentification64 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification64 {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICIdentifier>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification1>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
}


// PartyIdentification71Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification71Choice {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICIdentifier>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification36>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
}


// PartyIdentification75Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification75Choice {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICIdentifier>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
}


// PartyIdentification99Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification99Choice {
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress8>,
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<PartyIdentification44>,
}


// PartyIdentificationAndAccount95 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentificationAndAccount95 {
	#[serde(rename = "PtyId")]
	pub pty_id: PartyIdentification71Choice,
	#[serde(rename = "AcctId", skip_serializing_if = "Option::is_none")]
	pub acct_id: Option<SecuritiesAccount22>,
	#[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
	pub prcg_id: Option<Max35Text>,
}


// PartyIdentificationAndAccount96 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentificationAndAccount96 {
	#[serde(rename = "PtyId")]
	pub pty_id: PartyIdentification64,
	#[serde(rename = "AcctId")]
	pub acct_id: AccountIdentification26,
}


// PartyIdentificationAndAccount97 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentificationAndAccount97 {
	#[serde(rename = "PtyId")]
	pub pty_id: PartyIdentification62,
	#[serde(rename = "AcctId", skip_serializing_if = "Option::is_none")]
	pub acct_id: Option<AccountIdentification26>,
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


// Purpose3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Purpose3Choice {
	#[serde(rename = "SctiesPurpCd", skip_serializing_if = "Option::is_none")]
	pub scties_purp_cd: Option<ExternalSecuritiesPurpose1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification1>,
}


// SecuritiesAccount22 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesAccount22 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<GenericIdentification30>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max70Text>,
}


// SecuritiesOrCash1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesOrCash1Choice {
	#[serde(rename = "SctiesDtls", skip_serializing_if = "Option::is_none")]
	pub scties_dtls: Option<SettlementParties35>,
	#[serde(rename = "CshPtiesDtls", skip_serializing_if = "Option::is_none")]
	pub csh_pties_dtls: Option<CashParties24>,
}


// SettlementParties32 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementParties32 {
	#[serde(rename = "Dpstry")]
	pub dpstry: PartyIdentification63,
	#[serde(rename = "Pty1", skip_serializing_if = "Option::is_none")]
	pub pty1: Option<PartyIdentificationAndAccount95>,
	#[serde(rename = "Pty2", skip_serializing_if = "Option::is_none")]
	pub pty2: Option<PartyIdentificationAndAccount95>,
	#[serde(rename = "Pty3", skip_serializing_if = "Option::is_none")]
	pub pty3: Option<PartyIdentificationAndAccount95>,
	#[serde(rename = "Pty4", skip_serializing_if = "Option::is_none")]
	pub pty4: Option<PartyIdentificationAndAccount95>,
	#[serde(rename = "Pty5", skip_serializing_if = "Option::is_none")]
	pub pty5: Option<PartyIdentificationAndAccount95>,
}


// SettlementParties35 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementParties35 {
	#[serde(rename = "StgSttlmPties")]
	pub stg_sttlm_pties: SettlementParties32,
	#[serde(rename = "LclMktId", skip_serializing_if = "Option::is_none")]
	pub lcl_mkt_id: Option<Vec<GenericIdentification49>>,
	#[serde(rename = "RegnDtls", skip_serializing_if = "Option::is_none")]
	pub regn_dtls: Option<PartyIdentification99Choice>,
}


// SimpleIdentificationInformation4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SimpleIdentificationInformation4 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
}


// StandingSettlementInstructionV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StandingSettlementInstructionV01 {
	#[serde(rename = "MsgRefId")]
	pub msg_ref_id: Max35Text,
	#[serde(rename = "FctvDtDtls", skip_serializing_if = "Option::is_none")]
	pub fctv_dt_dtls: Option<EffectiveDate1>,
	#[serde(rename = "AcctId")]
	pub acct_id: Vec<AccountIdentification26>,
	#[serde(rename = "MktId")]
	pub mkt_id: MarketIdentificationOrCashPurpose1Choice,
	#[serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none")]
	pub sttlm_ccy: Option<ActiveCurrencyCode>,
	#[serde(rename = "SttlmDtls")]
	pub sttlm_dtls: SecuritiesOrCash1Choice,
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
