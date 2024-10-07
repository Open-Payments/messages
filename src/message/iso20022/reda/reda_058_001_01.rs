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


// AcceptedReason7Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AcceptedReason7Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification36>,
}


// AcceptedReason8Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AcceptedReason8Choice {
	#[serde(rename = "NoSpcfdRsn")]
	pub no_spcfd_rsn: Option<String>,
	#[validate]
	#[serde(rename = "Rsn")]
	pub rsn: Option<AcceptedReason7Choice>,
}


// AcceptedStatusReason7 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AcceptedStatusReason7 {
	#[validate]
	#[serde(rename = "Rsn")]
	pub rsn: AcceptedReason8Choice,
	#[serde(rename = "AddtlRsnInf")]
	pub addtl_rsn_inf: Option<String>,
}


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


// CFIOct2015Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[validate(pattern = "[A-Z]{6,6}")]
	#[serde(rename = "CFIOct2015Identifier")]
	pub cfi_oct2015_identifier: String,
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


// ExternalAcceptedReason1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalAcceptedReason1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalAcceptedReason1Code")]
	pub external_accepted_reason1_code: String,
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


// ExternalPendingProcessingReason1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalPendingProcessingReason1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalPendingProcessingReason1Code")]
	pub external_pending_processing_reason1_code: String,
}


// ExternalReceivedReason1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalReceivedReason1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalReceivedReason1Code")]
	pub external_received_reason1_code: String,
}


// ExternalRejectedReason1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalRejectedReason1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalRejectedReason1Code")]
	pub external_rejected_reason1_code: String,
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


// Max210Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max210Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 210)]
	#[serde(rename = "Max210Text")]
	pub max210_text: String,
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


// NoReasonCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NoReasonCode {
	#[validate(enumerate = ["NORE"])]
	#[serde(rename = "NoReasonCode")]
	pub no_reason_code: String,
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


// PartyOrCurrency1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyOrCurrency1Choice {
	#[validate]
	#[serde(rename = "Dpstry")]
	pub dpstry: Option<PartyIdentification63>,
	#[serde(rename = "SttlmCcy")]
	pub sttlm_ccy: Option<String>,
}


// PendingProcessingReason8Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PendingProcessingReason8Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification36>,
}


// PendingProcessingReason9Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PendingProcessingReason9Choice {
	#[serde(rename = "NoSpcfdRsn")]
	pub no_spcfd_rsn: Option<String>,
	#[validate]
	#[serde(rename = "Rsn")]
	pub rsn: Option<Vec<PendingProcessingReason8Choice>>,
}


// PendingProcessingStatusReason1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PendingProcessingStatusReason1 {
	#[validate]
	#[serde(rename = "Rsn")]
	pub rsn: PendingProcessingReason9Choice,
	#[serde(rename = "AddtlRsnInf")]
	pub addtl_rsn_inf: Option<String>,
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


// ProcessingStatus43Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ProcessingStatus43Choice {
	#[validate]
	#[serde(rename = "Rcvd")]
	pub rcvd: Option<ReceivedStatusReason1>,
	#[validate]
	#[serde(rename = "Accptd")]
	pub accptd: Option<AcceptedStatusReason7>,
	#[validate]
	#[serde(rename = "PdgPrcg")]
	pub pdg_prcg: Option<PendingProcessingStatusReason1>,
	#[validate]
	#[serde(rename = "Rjctd")]
	pub rjctd: Option<RejectedStatusReason12>,
	#[validate]
	#[serde(rename = "PrtrySts")]
	pub prtry_sts: Option<ProprietaryStatusAndReason5>,
}


// ProprietaryReason1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ProprietaryReason1Choice {
	#[serde(rename = "NoSpcfdRsn")]
	pub no_spcfd_rsn: Option<String>,
	#[validate]
	#[serde(rename = "Rsn")]
	pub rsn: Option<Vec<GenericIdentification36>>,
}


// ProprietaryStatusAndReason5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ProprietaryStatusAndReason5 {
	#[validate]
	#[serde(rename = "Sts")]
	pub sts: GenericIdentification36,
	#[validate]
	#[serde(rename = "Rsn")]
	pub rsn: ProprietaryReason1Choice,
	#[serde(rename = "AddtlRsnInf")]
	pub addtl_rsn_inf: Option<String>,
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


// ReceivedReason1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReceivedReason1Choice {
	#[serde(rename = "NoSpcfdRsn")]
	pub no_spcfd_rsn: Option<String>,
	#[validate]
	#[serde(rename = "Rsn")]
	pub rsn: Option<ReceivedReason2Choice>,
}


// ReceivedReason2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReceivedReason2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification36>,
}


// ReceivedStatusReason1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReceivedStatusReason1 {
	#[validate]
	#[serde(rename = "Rsn")]
	pub rsn: ReceivedReason1Choice,
	#[serde(rename = "AddtlRsnInf")]
	pub addtl_rsn_inf: Option<String>,
}


// RejectedReason7Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RejectedReason7Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification36>,
}


// RejectedReason8Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RejectedReason8Choice {
	#[serde(rename = "NoSpcfdRsn")]
	pub no_spcfd_rsn: Option<String>,
	#[validate]
	#[serde(rename = "Rsn")]
	pub rsn: Option<Vec<RejectedReason7Choice>>,
}


// RejectedStatusReason12 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RejectedStatusReason12 {
	#[validate]
	#[serde(rename = "Rsn")]
	pub rsn: RejectedReason8Choice,
	#[serde(rename = "AddtlRsnInf")]
	pub addtl_rsn_inf: Option<String>,
}


// SimpleIdentificationInformation4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SimpleIdentificationInformation4 {
	#[serde(rename = "Id")]
	pub id: String,
}


// StandingSettlementInstructionStatusAdviceV01 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct StandingSettlementInstructionStatusAdviceV01 {
	#[validate]
	#[serde(rename = "FctvDtDtls")]
	pub fctv_dt_dtls: Option<EffectiveDate1>,
	#[validate]
	#[serde(rename = "AcctId")]
	pub acct_id: Vec<AccountIdentification26>,
	#[validate]
	#[serde(rename = "MktId")]
	pub mkt_id: MarketIdentificationOrCashPurpose1Choice,
	#[validate]
	#[serde(rename = "SttlmDtls")]
	pub sttlm_dtls: PartyOrCurrency1Choice,
	#[serde(rename = "RltdMsgRef")]
	pub rltd_msg_ref: String,
	#[validate]
	#[serde(rename = "PrcgSts")]
	pub prcg_sts: ProcessingStatus43Choice,
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
