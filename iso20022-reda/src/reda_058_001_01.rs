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


// AcceptedReason7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AcceptedReason7Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalAcceptedReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}


// AcceptedReason8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AcceptedReason8Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<AcceptedReason7Choice>,
}


// AcceptedStatusReason7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AcceptedStatusReason7 {
	#[serde(rename = "Rsn")]
	pub rsn: AcceptedReason8Choice,
	#[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
	pub addtl_rsn_inf: Option<Max210Text>,
}


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


// CFIOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[serde(rename = "$value")]
	pub cfi_oct2015_identifier: String,
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


// ExternalAcceptedReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAcceptedReason1Code {
	#[serde(rename = "$value")]
	pub external_accepted_reason1_code: String,
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


// ExternalPendingProcessingReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPendingProcessingReason1Code {
	#[serde(rename = "$value")]
	pub external_pending_processing_reason1_code: String,
}


// ExternalReceivedReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalReceivedReason1Code {
	#[serde(rename = "$value")]
	pub external_received_reason1_code: String,
}


// ExternalRejectedReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalRejectedReason1Code {
	#[serde(rename = "$value")]
	pub external_rejected_reason1_code: String,
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


// Max210Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max210Text {
	#[serde(rename = "$value")]
	pub max210_text: String,
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


// NoReasonCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NoReasonCode {
	#[serde(rename = "NORE")]
	CodeNORE,

	#[default]
	UNKOWN
}


// PartyIdentification63 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification63 {
	#[serde(rename = "PtyId")]
	pub pty_id: PartyIdentification75Choice,
	#[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
	pub prcg_id: Option<Max35Text>,
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


// PartyOrCurrency1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyOrCurrency1Choice {
	#[serde(rename = "Dpstry", skip_serializing_if = "Option::is_none")]
	pub dpstry: Option<PartyIdentification63>,
	#[serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none")]
	pub sttlm_ccy: Option<ActiveCurrencyCode>,
}


// PendingProcessingReason8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingProcessingReason8Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalPendingProcessingReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}


// PendingProcessingReason9Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingProcessingReason9Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<PendingProcessingReason8Choice>>,
}


// PendingProcessingStatusReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingProcessingStatusReason1 {
	#[serde(rename = "Rsn")]
	pub rsn: PendingProcessingReason9Choice,
	#[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
	pub addtl_rsn_inf: Option<Max210Text>,
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


// ProcessingStatus43Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProcessingStatus43Choice {
	#[serde(rename = "Rcvd", skip_serializing_if = "Option::is_none")]
	pub rcvd: Option<ReceivedStatusReason1>,
	#[serde(rename = "Accptd", skip_serializing_if = "Option::is_none")]
	pub accptd: Option<AcceptedStatusReason7>,
	#[serde(rename = "PdgPrcg", skip_serializing_if = "Option::is_none")]
	pub pdg_prcg: Option<PendingProcessingStatusReason1>,
	#[serde(rename = "Rjctd", skip_serializing_if = "Option::is_none")]
	pub rjctd: Option<RejectedStatusReason12>,
	#[serde(rename = "PrtrySts", skip_serializing_if = "Option::is_none")]
	pub prtry_sts: Option<ProprietaryStatusAndReason5>,
}


// ProprietaryReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryReason1Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<GenericIdentification36>>,
}


// ProprietaryStatusAndReason5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryStatusAndReason5 {
	#[serde(rename = "Sts")]
	pub sts: GenericIdentification36,
	#[serde(rename = "Rsn")]
	pub rsn: ProprietaryReason1Choice,
	#[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
	pub addtl_rsn_inf: Option<Max210Text>,
}


// Purpose3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Purpose3Choice {
	#[serde(rename = "SctiesPurpCd", skip_serializing_if = "Option::is_none")]
	pub scties_purp_cd: Option<ExternalSecuritiesPurpose1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification1>,
}


// ReceivedReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReceivedReason1Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<ReceivedReason2Choice>,
}


// ReceivedReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReceivedReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalReceivedReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}


// ReceivedStatusReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReceivedStatusReason1 {
	#[serde(rename = "Rsn")]
	pub rsn: ReceivedReason1Choice,
	#[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
	pub addtl_rsn_inf: Option<Max210Text>,
}


// RejectedReason7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RejectedReason7Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalRejectedReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}


// RejectedReason8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RejectedReason8Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<RejectedReason7Choice>>,
}


// RejectedStatusReason12 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RejectedStatusReason12 {
	#[serde(rename = "Rsn")]
	pub rsn: RejectedReason8Choice,
	#[serde(rename = "AddtlRsnInf", skip_serializing_if = "Option::is_none")]
	pub addtl_rsn_inf: Option<Max210Text>,
}


// SimpleIdentificationInformation4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SimpleIdentificationInformation4 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
}


// StandingSettlementInstructionStatusAdviceV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StandingSettlementInstructionStatusAdviceV01 {
	#[serde(rename = "FctvDtDtls", skip_serializing_if = "Option::is_none")]
	pub fctv_dt_dtls: Option<EffectiveDate1>,
	#[serde(rename = "AcctId")]
	pub acct_id: Vec<AccountIdentification26>,
	#[serde(rename = "MktId")]
	pub mkt_id: MarketIdentificationOrCashPurpose1Choice,
	#[serde(rename = "SttlmDtls")]
	pub sttlm_dtls: PartyOrCurrency1Choice,
	#[serde(rename = "RltdMsgRef")]
	pub rltd_msg_ref: Max35Text,
	#[serde(rename = "PrcgSts")]
	pub prcg_sts: ProcessingStatus43Choice,
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
