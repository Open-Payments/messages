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


// Exact4AlphaNumericText ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "Exact4AlphaNumericText")]
	pub exact4_alpha_numeric_text: String,
}


// ExternalFinancialInstrumentIdentificationType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalFinancialInstrumentIdentificationType1Code {
	#[serde(rename = "ExternalFinancialInstrumentIdentificationType1Code")]
	pub external_financial_instrument_identification_type1_code: String,
}


// GenericIdentification30 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification30 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// ISIN2021Identifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISIN2021Identifier {
	#[serde(rename = "ISIN2021Identifier")]
	pub isin2021_identifier: String,
}


// ISODateTime ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// IdentificationSource3Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IdentificationSource3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// Max140Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max16Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max16Text {
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
}


// Max210Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max210Text {
	#[serde(rename = "Max210Text")]
	pub max210_text: String,
}


// Max350Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// MessageHeader12 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader12 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: Option<String>,
	#[serde(rename = "OrgnlBizInstr")]
	pub orgnl_biz_instr: Option<OriginalBusinessInstruction1>,
}


// NoReasonCode ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NoReasonCode {
	#[serde(rename = "NoReasonCode")]
	pub no_reason_code: String,
}


// OriginalBusinessInstruction1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OriginalBusinessInstruction1 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "MsgNmId")]
	pub msg_nm_id: Option<String>,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: Option<String>,
}


// OtherIdentification1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Sfx")]
	pub sfx: Option<String>,
	#[serde(rename = "Tp")]
	pub tp: IdentificationSource3Choice,
}


// ProcessingStatus72Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProcessingStatus72Choice {
	#[serde(rename = "AckdAccptd")]
	pub ackd_accptd: Option<Reason4>,
	#[serde(rename = "PdgPrcg")]
	pub pdg_prcg: Option<Reason18Choice>,
	#[serde(rename = "Rjctd")]
	pub rjctd: Option<Reason18Choice>,
	#[serde(rename = "Cmpltd")]
	pub cmpltd: Option<Reason4>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<ProprietaryStatusAndReason6>,
}


// ProprietaryReason4 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryReason4 {
	#[serde(rename = "Rsn")]
	pub rsn: Option<GenericIdentification30>,
	#[serde(rename = "AddtlRsnInf")]
	pub addtl_rsn_inf: Option<String>,
}


// ProprietaryStatusAndReason6 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryStatusAndReason6 {
	#[serde(rename = "PrtrySts")]
	pub prtry_sts: GenericIdentification30,
	#[serde(rename = "PrtryRsn")]
	pub prtry_rsn: Option<Vec<ProprietaryReason4>>,
}


// Reason18Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Reason18Choice {
	#[serde(rename = "Rsn")]
	pub rsn: Option<Vec<ProprietaryReason4>>,
	#[serde(rename = "NoSpcfdRsn")]
	pub no_spcfd_rsn: Option<String>,
}


// Reason4 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Reason4 {
	#[serde(rename = "Rsn")]
	pub rsn: Option<Vec<ProprietaryReason4>>,
}


// SecurityCreationStatusAdviceV01 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityCreationStatusAdviceV01 {
	#[serde(rename = "MsgHdr")]
	pub msg_hdr: Option<MessageHeader12>,
	#[serde(rename = "FinInstrmId")]
	pub fin_instrm_id: Option<SecurityIdentification39>,
	#[serde(rename = "PrcgSts")]
	pub prcg_sts: ProcessingStatus72Choice,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// SecurityIdentification39 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification39 {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[serde(rename = "OthrId")]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// SupplementaryData1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Option<String>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}
