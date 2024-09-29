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


// AddressType2Code ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AddressType2Code {
	#[serde(rename = "AddressType2Code")]
	pub address_type2_code: String,
}


// AddressType3Choice ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AddressType3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// BICFIDec2014Identifier ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BICFIDec2014Identifier {
	#[serde(rename = "BICFIDec2014Identifier")]
	pub bicfi_dec2014_identifier: String,
}


// BranchAndFinancialInstitutionIdentification6 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BranchAndFinancialInstitutionIdentification6 {
	#[serde(rename = "FinInstnId")]
	pub fin_instn_id: FinancialInstitutionIdentification18,
	#[serde(rename = "BrnchId")]
	pub brnch_id: Option<BranchData3>,
}


// BranchData3 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BranchData3 {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress24>,
}


// BusinessApplicationHeader5 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BusinessApplicationHeader5 {
	#[serde(rename = "CharSet")]
	pub char_set: Option<String>,
	#[serde(rename = "Fr")]
	pub fr: Party44Choice,
	#[serde(rename = "To")]
	pub to: Party44Choice,
	#[serde(rename = "BizMsgIdr")]
	pub biz_msg_idr: String,
	#[serde(rename = "MsgDefIdr")]
	pub msg_def_idr: String,
	#[serde(rename = "BizSvc")]
	pub biz_svc: Option<String>,
	#[serde(rename = "CreDt")]
	pub cre_dt: String,
	#[serde(rename = "CpyDplct")]
	pub cpy_dplct: Option<String>,
	#[serde(rename = "PssblDplct")]
	pub pssbl_dplct: Option<bool>,
	#[serde(rename = "Prty")]
	pub prty: Option<String>,
	#[serde(rename = "Sgntr")]
	pub sgntr: Option<SignatureEnvelope>,
}


// BusinessApplicationHeaderV02 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BusinessApplicationHeaderV02 {
	#[serde(rename = "CharSet")]
	pub char_set: Option<String>,
	#[serde(rename = "Fr")]
	pub fr: Party44Choice,
	#[serde(rename = "To")]
	pub to: Party44Choice,
	#[serde(rename = "BizMsgIdr")]
	pub biz_msg_idr: String,
	#[serde(rename = "MsgDefIdr")]
	pub msg_def_idr: String,
	#[serde(rename = "BizSvc")]
	pub biz_svc: Option<String>,
	#[serde(rename = "MktPrctc")]
	pub mkt_prctc: Option<ImplementationSpecification1>,
	#[serde(rename = "CreDt")]
	pub cre_dt: String,
	#[serde(rename = "BizPrcgDt")]
	pub biz_prcg_dt: Option<String>,
	#[serde(rename = "CpyDplct")]
	pub cpy_dplct: Option<String>,
	#[serde(rename = "PssblDplct")]
	pub pssbl_dplct: Option<bool>,
	#[serde(rename = "Prty")]
	pub prty: Option<String>,
	#[serde(rename = "Sgntr")]
	pub sgntr: Option<SignatureEnvelope>,
	#[serde(rename = "Rltd")]
	pub rltd: Option<Vec<BusinessApplicationHeader5>>,
}


// BusinessMessagePriorityCode ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct BusinessMessagePriorityCode {
	#[serde(rename = "BusinessMessagePriorityCode")]
	pub business_message_priority_code: String,
}


// ClearingSystemIdentification2Choice ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ClearingSystemIdentification2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ClearingSystemMemberIdentification2 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ClearingSystemMemberIdentification2 {
	#[serde(rename = "ClrSysId")]
	pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
	#[serde(rename = "MmbId")]
	pub mmb_id: String,
}


// Contact4 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Contact4 {
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
}


// CopyDuplicate1Code ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CopyDuplicate1Code {
	#[serde(rename = "CopyDuplicate1Code")]
	pub copy_duplicate1_code: String,
}


// CountryCode ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// DateAndPlaceOfBirth1 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct DateAndPlaceOfBirth1 {
	#[serde(rename = "BirthDt")]
	pub birth_dt: String,
	#[serde(rename = "PrvcOfBirth")]
	pub prvc_of_birth: Option<String>,
	#[serde(rename = "CityOfBirth")]
	pub city_of_birth: String,
	#[serde(rename = "CtryOfBirth")]
	pub ctry_of_birth: String,
}


// Exact4AlphaNumericText ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "Exact4AlphaNumericText")]
	pub exact4_alpha_numeric_text: String,
}


// ExternalClearingSystemIdentification1Code ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalClearingSystemIdentification1Code {
	#[serde(rename = "ExternalClearingSystemIdentification1Code")]
	pub external_clearing_system_identification1_code: String,
}


// ExternalFinancialInstitutionIdentification1Code ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalFinancialInstitutionIdentification1Code {
	#[serde(rename = "ExternalFinancialInstitutionIdentification1Code")]
	pub external_financial_institution_identification1_code: String,
}


// ExternalOrganisationIdentification1Code ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalOrganisationIdentification1Code {
	#[serde(rename = "ExternalOrganisationIdentification1Code")]
	pub external_organisation_identification1_code: String,
}


// ExternalPersonIdentification1Code ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ExternalPersonIdentification1Code {
	#[serde(rename = "ExternalPersonIdentification1Code")]
	pub external_person_identification1_code: String,
}


// FinancialIdentificationSchemeName1Choice ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FinancialIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// FinancialInstitutionIdentification18 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct FinancialInstitutionIdentification18 {
	#[serde(rename = "BICFI")]
	pub bicfi: Option<String>,
	#[serde(rename = "ClrSysMmbId")]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress24>,
	#[serde(rename = "Othr")]
	pub othr: Option<GenericFinancialIdentification1>,
}


// GenericFinancialIdentification1 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GenericFinancialIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<FinancialIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericIdentification30 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GenericIdentification30 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// GenericOrganisationIdentification1 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GenericOrganisationIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericPersonIdentification1 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GenericPersonIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// ISODate ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// ImplementationSpecification1 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ImplementationSpecification1 {
	#[serde(rename = "Regy")]
	pub regy: String,
	#[serde(rename = "Id")]
	pub id: String,
}


// LEIIdentifier ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// Max128Text ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max128Text {
	#[serde(rename = "Max128Text")]
	pub max128_text: String,
}


// Max140Text ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max140Text {
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max16Text ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max16Text {
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
}


// Max2048Text ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max2048Text {
	#[serde(rename = "Max2048Text")]
	pub max2048_text: String,
}


// Max350Text ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max350Text {
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max35Text {
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Max4Text ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max4Text {
	#[serde(rename = "Max4Text")]
	pub max4_text: String,
}


// Max70Text ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max70Text {
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// NamePrefix2Code ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct NamePrefix2Code {
	#[serde(rename = "NamePrefix2Code")]
	pub name_prefix2_code: String,
}


// OrganisationIdentification29 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OrganisationIdentification29 {
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<GenericOrganisationIdentification1>>,
}


// OrganisationIdentificationSchemeName1Choice ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OrganisationIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// OtherContact1 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct OtherContact1 {
	#[serde(rename = "ChanlTp")]
	pub chanl_tp: String,
	#[serde(rename = "Id")]
	pub id: Option<String>,
}


// Party38Choice ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Party38Choice {
	#[serde(rename = "OrgId")]
	pub org_id: Option<OrganisationIdentification29>,
	#[serde(rename = "PrvtId")]
	pub prvt_id: Option<PersonIdentification13>,
}


// Party44Choice ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Party44Choice {
	#[serde(rename = "OrgId")]
	pub org_id: Option<PartyIdentification135>,
	#[serde(rename = "FIId")]
	pub fi_id: Option<BranchAndFinancialInstitutionIdentification6>,
}


// PartyIdentification135 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PartyIdentification135 {
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress24>,
	#[serde(rename = "Id")]
	pub id: Option<Party38Choice>,
	#[serde(rename = "CtryOfRes")]
	pub ctry_of_res: Option<String>,
	#[serde(rename = "CtctDtls")]
	pub ctct_dtls: Option<Contact4>,
}


// PersonIdentification13 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PersonIdentification13 {
	#[serde(rename = "DtAndPlcOfBirth")]
	pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<GenericPersonIdentification1>>,
}


// PersonIdentificationSchemeName1Choice ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PersonIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// PhoneNumber ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PhoneNumber {
	#[serde(rename = "PhoneNumber")]
	pub phone_number: String,
}


// PostalAddress24 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PostalAddress24 {
	#[serde(rename = "AdrTp")]
	pub adr_tp: Option<AddressType3Choice>,
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
}


// PreferredContactMethod1Code ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct PreferredContactMethod1Code {
	#[serde(rename = "PreferredContactMethod1Code")]
	pub preferred_contact_method1_code: String,
}


// SignatureEnvelope ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SignatureEnvelope {
}


// UnicodeChartsCode ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct UnicodeChartsCode {
	#[serde(rename = "UnicodeChartsCode")]
	pub unicode_charts_code: String,
}


// YesNoIndicator ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct YesNoIndicator {
	#[serde(rename = "YesNoIndicator")]
	pub yes_no_indicator: bool,
}
