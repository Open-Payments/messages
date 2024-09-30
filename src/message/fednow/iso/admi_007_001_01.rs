// FedNow Message Parsing Library
// https://github.com/Open-Payments/messages
//
// This library is designed to parse FedNow message formats based on ISO 20022 standards.
// It handles various message types, including administrative notifications, payment status reports, 
// customer credit transfers, and more, using Serde for efficient serialization and deserialization.
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


// AddressType2Code ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct AddressType2Code {
	#[serde(rename = "AddressType2Code")]
	pub address_type2_code: String,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct AnyBICDec2014Identifier {
	#[validate(pattern = "[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}")]
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// CountryCode ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct CountryCode {
	#[validate(pattern = "[A-Z]{2,2}")]
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// GenericIdentification36 ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct GenericIdentification36 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// ISODateTime ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// LEIIdentifier ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct LEIIdentifier {
	#[validate(pattern = "[A-Z0-9]{18,18}[0-9]{2,2}")]
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// Max140Text ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct Max140Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 140)]
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max16Text ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct Max16Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 16)]
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
}


// Max350Text ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct Max350Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 350)]
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct Max35Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 35)]
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Max4AlphaNumericText ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct Max4AlphaNumericText {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[validate(pattern = "[a-zA-Z0-9]{1,4}")]
	#[serde(rename = "Max4AlphaNumericText")]
	pub max4_alpha_numeric_text: String,
}


// Max70Text ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct Max70Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 70)]
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// MessageHeader10 ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct MessageHeader10 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: Option<String>,
	#[serde(rename = "QryNm")]
	pub qry_nm: Option<String>,
}


// MessageReference1 ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct MessageReference1 {
	#[serde(rename = "Ref")]
	pub ref_attr: String,
	#[serde(rename = "MsgNm")]
	pub msg_nm: Option<String>,
	#[serde(rename = "RefIssr")]
	pub ref_issr: Option<PartyIdentification136>,
}


// NameAndAddress5 ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Adr")]
	pub adr: Option<PostalAddress1>,
}


// PartyIdentification120Choice ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct PartyIdentification120Choice {
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
	#[serde(rename = "PrtryId")]
	pub prtry_id: Option<GenericIdentification36>,
	#[serde(rename = "NmAndAdr")]
	pub nm_and_adr: Option<NameAndAddress5>,
}


// PartyIdentification136 ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct PartyIdentification136 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification120Choice,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
}


// PostalAddress1 ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
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


// ReceiptAcknowledgementReport2 ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct ReceiptAcknowledgementReport2 {
	#[serde(rename = "RltdRef")]
	pub rltd_ref: MessageReference1,
	#[serde(rename = "ReqHdlg")]
	pub req_hdlg: RequestHandling2,
}


// ReceiptAcknowledgementV01 ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct ReceiptAcknowledgementV01 {
	#[serde(rename = "MsgId")]
	pub msg_id: MessageHeader10,
	#[serde(rename = "Rpt")]
	pub rpt: Vec<ReceiptAcknowledgementReport2>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// RequestHandling2 ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct RequestHandling2 {
	#[serde(rename = "StsCd")]
	pub sts_cd: String,
	#[serde(rename = "StsDtTm")]
	pub sts_dt_tm: Option<String>,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// SupplementaryData1 ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Option<String>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Validate, Deserialize, Serialize, PartialEq)]
pub struct SupplementaryDataEnvelope1 {
}
