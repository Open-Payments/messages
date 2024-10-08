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


// AddressType2Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType2Code {
	#[serde(rename = "AddressType2Code")]
	pub address_type2_code: String,
}


// AddressType3Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// BICFIDec2014Identifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BICFIDec2014Identifier {
	#[serde(rename = "BICFIDec2014Identifier")]
	pub bicfi_dec2014_identifier: String,
}


// CodeOrProprietary1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CodeOrProprietary1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification13>,
}


// Contact14 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Contact14 {
	#[serde(rename = "NmPrfx")]
	pub nm_prfx: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "PhneNb")]
	pub phne_nb: Option<String>,
	#[serde(rename = "MobNb")]
	pub mob_nb: Option<String>,
	#[serde(rename = "FaxNb")]
	pub fax_nb: Option<String>,
	#[serde(rename = "URLAdr")]
	pub url_adr: Option<String>,
	#[serde(rename = "EmailAdr")]
	pub email_adr: Option<String>,
	#[serde(rename = "EmailPurp")]
	pub email_purp: Option<String>,
	#[serde(rename = "JobTitl")]
	pub job_titl: Option<String>,
	#[serde(rename = "Rspnsblty")]
	pub rspnsblty: Option<String>,
	#[serde(rename = "Dept")]
	pub dept: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<OtherContact1>>,
	#[serde(rename = "PrefrdMtd")]
	pub prefrd_mtd: Option<String>,
	#[serde(rename = "VldFr")]
	pub vld_fr: Option<String>,
	#[serde(rename = "VldTo")]
	pub vld_to: Option<String>,
}


// CountryCode ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// Exact4AlphaNumericText ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "Exact4AlphaNumericText")]
	pub exact4_alpha_numeric_text: String,
}


// ExternalSystemPartyType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalSystemPartyType1Code {
	#[serde(rename = "ExternalSystemPartyType1Code")]
	pub external_system_party_type1_code: String,
}


// GenericIdentification13 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification13 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: String,
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


// GenericIdentification36 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification36 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// ISODate ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// LEIIdentifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LockStatus1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LockStatus1Code {
	#[serde(rename = "LockStatus1Code")]
	pub lock_status1_code: String,
}


// MarketSpecificAttribute1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketSpecificAttribute1 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Val")]
	pub val: String,
}


// Max128Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max128Text {
	#[serde(rename = "Max128Text")]
	pub max128_text: String,
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


// Max2048Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max2048Text {
	#[serde(rename = "Max2048Text")]
	pub max2048_text: String,
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


// Max4AlphaNumericText ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "Max4AlphaNumericText")]
	pub max4_alpha_numeric_text: String,
}


// Max4Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4Text {
	#[serde(rename = "Max4Text")]
	pub max4_text: String,
}


// Max70Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// MessageHeader1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader1 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: Option<String>,
}


// NameAndAddress5 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Adr")]
	pub adr: Option<PostalAddress1>,
}


// NamePrefix2Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NamePrefix2Code {
	#[serde(rename = "NamePrefix2Code")]
	pub name_prefix2_code: String,
}


// OtherContact1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherContact1 {
	#[serde(rename = "ChanlTp")]
	pub chanl_tp: String,
	#[serde(rename = "Id")]
	pub id: Option<String>,
}


// PartyActivityAdviceV02 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyActivityAdviceV02 {
	#[serde(rename = "MsgHdr")]
	pub msg_hdr: Option<MessageHeader1>,
	#[serde(rename = "PtyActvty")]
	pub pty_actvty: PartyStatement3,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// PartyIdentification120Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification120Choice {
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
	#[serde(rename = "PrtryId")]
	pub prtry_id: Option<GenericIdentification36>,
	#[serde(rename = "NmAndAdr")]
	pub nm_and_adr: Option<NameAndAddress5>,
}


// PartyIdentification136 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification136 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification120Choice,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
}


// PartyLockStatus1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyLockStatus1 {
	#[serde(rename = "VldFr")]
	pub vld_fr: Option<String>,
	#[serde(rename = "Sts")]
	pub sts: String,
	#[serde(rename = "LckRsn")]
	pub lck_rsn: Option<Vec<String>>,
}


// PartyName4 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyName4 {
	#[serde(rename = "VldFr")]
	pub vld_fr: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "ShrtNm")]
	pub shrt_nm: Option<String>,
}


// PartyReferenceDataChange3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyReferenceDataChange3 {
	#[serde(rename = "PtyId")]
	pub pty_id: SystemPartyIdentification8,
	#[serde(rename = "Rcrd")]
	pub rcrd: Vec<UpdateLogPartyRecord2Choice>,
	#[serde(rename = "OprTmStmp")]
	pub opr_tm_stmp: String,
}


// PartyStatement3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyStatement3 {
	#[serde(rename = "SysDt")]
	pub sys_dt: String,
	#[serde(rename = "Chng")]
	pub chng: Option<Vec<PartyReferenceDataChange3>>,
}


// PhoneNumber ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PhoneNumber {
	#[serde(rename = "PhoneNumber")]
	pub phone_number: String,
}


// PostalAddress1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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


// PostalAddress28 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress28 {
	#[serde(rename = "AdrTp")]
	pub adr_tp: Option<AddressType3Choice>,
	#[serde(rename = "CareOf")]
	pub care_of: Option<String>,
	#[serde(rename = "Dept")]
	pub dept: Option<String>,
	#[serde(rename = "SubDept")]
	pub sub_dept: Option<String>,
	#[serde(rename = "StrtNm")]
	pub strt_nm: Option<String>,
	#[serde(rename = "BldgNb")]
	pub bldg_nb: Option<String>,
	#[serde(rename = "BldgNm")]
	pub bldg_nm: Option<String>,
	#[serde(rename = "Flr")]
	pub flr: Option<String>,
	#[serde(rename = "UnitNb")]
	pub unit_nb: Option<String>,
	#[serde(rename = "PstBx")]
	pub pst_bx: Option<String>,
	#[serde(rename = "Room")]
	pub room: Option<String>,
	#[serde(rename = "PstCd")]
	pub pst_cd: Option<String>,
	#[serde(rename = "TwnNm")]
	pub twn_nm: Option<String>,
	#[serde(rename = "TwnLctnNm")]
	pub twn_lctn_nm: Option<String>,
	#[serde(rename = "DstrctNm")]
	pub dstrct_nm: Option<String>,
	#[serde(rename = "CtrySubDvsn")]
	pub ctry_sub_dvsn: Option<String>,
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
	#[serde(rename = "AdrLine")]
	pub adr_line: Option<Vec<String>>,
	#[serde(rename = "VldFr")]
	pub vld_fr: Option<String>,
}


// PreferredContactMethod2Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PreferredContactMethod2Code {
	#[serde(rename = "PreferredContactMethod2Code")]
	pub preferred_contact_method2_code: String,
}


// ResidenceType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ResidenceType1Code {
	#[serde(rename = "ResidenceType1Code")]
	pub residence_type1_code: String,
}


// Restriction1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Restriction1 {
	#[serde(rename = "RstrctnTp")]
	pub rstrctn_tp: CodeOrProprietary1Choice,
	#[serde(rename = "VldFr")]
	pub vld_fr: String,
	#[serde(rename = "VldUntil")]
	pub vld_until: Option<String>,
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


// SystemPartyIdentification8 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemPartyIdentification8 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification136,
	#[serde(rename = "RspnsblPtyId")]
	pub rspnsbl_pty_id: Option<PartyIdentification136>,
}


// SystemPartyType1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemPartyType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// TechnicalIdentification2Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TechnicalIdentification2Choice {
	#[serde(rename = "BICFI")]
	pub bicfi: Option<String>,
	#[serde(rename = "TechAdr")]
	pub tech_adr: Option<String>,
}


// UpdateLogAddress2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateLogAddress2 {
	#[serde(rename = "Od")]
	pub od: PostalAddress28,
	#[serde(rename = "New")]
	pub new: PostalAddress28,
}


// UpdateLogContact2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateLogContact2 {
	#[serde(rename = "Od")]
	pub od: Contact14,
	#[serde(rename = "New")]
	pub new: Contact14,
}


// UpdateLogDate1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateLogDate1 {
	#[serde(rename = "Od")]
	pub od: String,
	#[serde(rename = "New")]
	pub new: String,
}


// UpdateLogMarketSpecificAttribute1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateLogMarketSpecificAttribute1 {
	#[serde(rename = "Od")]
	pub od: MarketSpecificAttribute1,
	#[serde(rename = "New")]
	pub new: MarketSpecificAttribute1,
}


// UpdateLogPartyLockStatus1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateLogPartyLockStatus1 {
	#[serde(rename = "Od")]
	pub od: PartyLockStatus1,
	#[serde(rename = "New")]
	pub new: PartyLockStatus1,
}


// UpdateLogPartyName1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateLogPartyName1 {
	#[serde(rename = "Od")]
	pub od: PartyName4,
	#[serde(rename = "New")]
	pub new: PartyName4,
}


// UpdateLogPartyRecord2Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateLogPartyRecord2Choice {
	#[serde(rename = "Adr")]
	pub adr: Option<UpdateLogAddress2>,
	#[serde(rename = "CtctDtls")]
	pub ctct_dtls: Option<UpdateLogContact2>,
	#[serde(rename = "OpngDt")]
	pub opng_dt: Option<UpdateLogDate1>,
	#[serde(rename = "ClsgDt")]
	pub clsg_dt: Option<UpdateLogDate1>,
	#[serde(rename = "Tp")]
	pub tp: Option<UpdateLogSystemPartyType1>,
	#[serde(rename = "TechAdr")]
	pub tech_adr: Option<UpdateLogTechnicalAddress1>,
	#[serde(rename = "MktSpcfcAttr")]
	pub mkt_spcfc_attr: Option<UpdateLogMarketSpecificAttribute1>,
	#[serde(rename = "Nm")]
	pub nm: Option<UpdateLogPartyName1>,
	#[serde(rename = "ResTp")]
	pub res_tp: Option<UpdateLogResidenceType1>,
	#[serde(rename = "LckSts")]
	pub lck_sts: Option<UpdateLogPartyLockStatus1>,
	#[serde(rename = "Rstrctn")]
	pub rstrctn: Option<UpdateLogRestriction1>,
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<UpdateLogProprietary1>>,
}


// UpdateLogProprietary1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateLogProprietary1 {
	#[serde(rename = "FldNm")]
	pub fld_nm: String,
	#[serde(rename = "OdFldVal")]
	pub od_fld_val: String,
	#[serde(rename = "NewFldVal")]
	pub new_fld_val: String,
}


// UpdateLogResidenceType1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateLogResidenceType1 {
	#[serde(rename = "Od")]
	pub od: String,
	#[serde(rename = "New")]
	pub new: String,
}


// UpdateLogRestriction1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateLogRestriction1 {
	#[serde(rename = "Od")]
	pub od: Restriction1,
	#[serde(rename = "New")]
	pub new: Restriction1,
}


// UpdateLogSystemPartyType1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateLogSystemPartyType1 {
	#[serde(rename = "Od")]
	pub od: SystemPartyType1Choice,
	#[serde(rename = "New")]
	pub new: SystemPartyType1Choice,
}


// UpdateLogTechnicalAddress1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateLogTechnicalAddress1 {
	#[serde(rename = "Od")]
	pub od: TechnicalIdentification2Choice,
	#[serde(rename = "New")]
	pub new: TechnicalIdentification2Choice,
}
