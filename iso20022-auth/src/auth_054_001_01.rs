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


// CCPClearingMemberReportV01 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CCPClearingMemberReportV01 {
	#[serde(rename = "ClrMmb")]
	pub clr_mmb: Vec<ClearingMember1>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// ClearingAccount1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingAccount1 {
	#[serde(rename = "AcctTp")]
	pub acct_tp: String,
	#[serde(rename = "CollAcctOwnr")]
	pub coll_acct_ownr: Vec<CollateralAccount5>,
}


// ClearingAccountType3Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingAccountType3Code {
	#[serde(rename = "ClearingAccountType3Code")]
	pub clearing_account_type3_code: String,
}


// ClearingMember1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingMember1 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification118Choice,
	#[serde(rename = "CdtQlty")]
	pub cdt_qlty: String,
	#[serde(rename = "UltmtPrntId")]
	pub ultmt_prnt_id: Option<PartyIdentification118Choice>,
	#[serde(rename = "FutrsComssnMrchntInd")]
	pub futrs_comssn_mrchnt_ind: bool,
	#[serde(rename = "MmbshVldFr")]
	pub mmbsh_vld_fr: String,
	#[serde(rename = "MmbshVldTo")]
	pub mmbsh_vld_to: Option<String>,
	#[serde(rename = "SpnsrgClrMmbId")]
	pub spnsrg_clr_mmb_id: Option<PartyIdentification118Choice>,
	#[serde(rename = "ClrAcctOwnr")]
	pub clr_acct_ownr: Vec<ClearingAccount1>,
}


// CollateralAccount5 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralAccount5 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification118Choice,
	#[serde(rename = "RltdMrgnAcct")]
	pub rltd_mrgn_acct: Vec<MarginAccount1>,
	#[serde(rename = "TitlTrfCollArrgmnt")]
	pub titl_trf_coll_arrgmnt: Option<bool>,
	#[serde(rename = "CollSgrtnByVal")]
	pub coll_sgrtn_by_val: Option<bool>,
}


// CreditQuality1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditQuality1Code {
	#[serde(rename = "CreditQuality1Code")]
	pub credit_quality1_code: String,
}


// GenericIdentification168 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification168 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// ISODate ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// LEIIdentifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// MarginAccount1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarginAccount1 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification118Choice,
	#[serde(rename = "PosAcct")]
	pub pos_acct: Vec<PositionAccount1>,
}


// Max140Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max256Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max256Text {
	#[serde(rename = "Max256Text")]
	pub max256_text: String,
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


// PartyIdentification118Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification118Choice {
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification168>,
}


// PositionAccount1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionAccount1 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification118Choice,
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


// TrueFalseIndicator ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}
