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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType2Code {
	#[serde(rename = "AddressType2Code")]
	pub address_type2_code: String,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// BenchmarkCancellation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkCancellation1 {
	#[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
	pub tech_rcrd_id: Option<String>,
	#[serde(rename = "Id")]
	pub id: SecurityIdentification19,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// BenchmarkCreate1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkCreate1 {
	#[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
	pub tech_rcrd_id: Option<String>,
	#[serde(rename = "Id")]
	pub id: SecurityIdentification19,
	#[serde(rename = "Othr")]
	pub othr: BenchmarkDetail1,
	#[serde(rename = "Admstr")]
	pub admstr: PartyIdentification136,
	#[serde(rename = "NdrsngPty", skip_serializing_if = "Option::is_none")]
	pub ndrsng_pty: Option<PartyIdentification136>,
	#[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
	pub sts: Option<StatusDetail1>,
	#[serde(rename = "TechVldtyPrd", skip_serializing_if = "Option::is_none")]
	pub tech_vldty_prd: Option<Period4Choice>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// BenchmarkCurveName2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkCurveName2Code {
	#[serde(rename = "BenchmarkCurveName2Code")]
	pub benchmark_curve_name2_code: String,
}


// BenchmarkDetail1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkDetail1 {
	#[serde(rename = "FullNm")]
	pub full_nm: String,
	#[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
	pub indx: Option<String>,
	#[serde(rename = "Cmnt", skip_serializing_if = "Option::is_none")]
	pub cmnt: Option<String>,
}


// BenchmarkReport1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkReport1Choice {
	#[serde(rename = "Cret", skip_serializing_if = "Option::is_none")]
	pub cret: Option<BenchmarkCreate1>,
	#[serde(rename = "Upd", skip_serializing_if = "Option::is_none")]
	pub upd: Option<BenchmarkUpdate1>,
	#[serde(rename = "Cxl", skip_serializing_if = "Option::is_none")]
	pub cxl: Option<BenchmarkCancellation1>,
}


// BenchmarkUpdate1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkUpdate1 {
	#[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
	pub tech_rcrd_id: Option<String>,
	#[serde(rename = "Id")]
	pub id: SecurityIdentification19,
	#[serde(rename = "PrvsId", skip_serializing_if = "Option::is_none")]
	pub prvs_id: Option<SecurityIdentification19>,
	#[serde(rename = "Othr")]
	pub othr: BenchmarkDetail1,
	#[serde(rename = "Admstr")]
	pub admstr: PartyIdentification136,
	#[serde(rename = "NdrsngPty", skip_serializing_if = "Option::is_none")]
	pub ndrsng_pty: Option<PartyIdentification136>,
	#[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
	pub sts: Option<StatusDetail1>,
	#[serde(rename = "TechVldtyPrd", skip_serializing_if = "Option::is_none")]
	pub tech_vldty_prd: Option<Period4Choice>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// ExternalAuthorityIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAuthorityIdentification1Code {
	#[serde(rename = "ExternalAuthorityIdentification1Code")]
	pub external_authority_identification1_code: String,
}


// ExternalFinancialInstrumentIdentificationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalFinancialInstrumentIdentificationType1Code {
	#[serde(rename = "ExternalFinancialInstrumentIdentificationType1Code")]
	pub external_financial_instrument_identification_type1_code: String,
}


// FinancialBenchmarkReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialBenchmarkReportV01 {
	#[serde(rename = "BchmkData")]
	pub bchmk_data: Vec<BenchmarkReport1Choice>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// GenericIdentification36 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification36 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<String>,
}


// ISINOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "ISINOct2015Identifier")]
	pub isin_oct2015_identifier: String,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// IdentificationSource3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IdentificationSource3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// Max10Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max10Text {
	#[serde(rename = "Max10Text")]
	pub max10_text: String,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max16Text {
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
}


// Max20000Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max20000Text {
	#[serde(rename = "Max20000Text")]
	pub max20000_text: String,
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


// OtherIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Sfx", skip_serializing_if = "Option::is_none")]
	pub sfx: Option<String>,
	#[serde(rename = "Tp")]
	pub tp: IdentificationSource3Choice,
}


// PartyIdentification120Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification120Choice {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<String>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification36>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
}


// PartyIdentification136 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification136 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification120Choice,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<String>,
}


// Period2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// Period4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period4Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt", skip_serializing_if = "Option::is_none")]
	pub to_dt: Option<String>,
	#[serde(rename = "FrDtToDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt_to_dt: Option<Period2>,
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


// SecurityIdentification19 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification19 {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<String>,
	#[serde(rename = "OthrId", skip_serializing_if = "Option::is_none")]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<String>,
}


// StatusDetail1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatusDetail1 {
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<String>,
	#[serde(rename = "CmptntAuthrty")]
	pub cmptnt_authrty: SupervisingAuthorityIdentification1,
	#[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
	pub sts: Option<String>,
	#[serde(rename = "StsRsn")]
	pub sts_rsn: String,
	#[serde(rename = "ActvtyPrd", skip_serializing_if = "Option::is_none")]
	pub actvty_prd: Option<Period4Choice>,
	#[serde(rename = "Cmnt", skip_serializing_if = "Option::is_none")]
	pub cmnt: Option<String>,
}


// SupervisingAuthorityIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupervisingAuthorityIdentification1 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<SupervisingAuthorityIdentification1Choice>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<String>,
}


// SupervisingAuthorityIdentification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupervisingAuthorityIdentification1Choice {
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<String>,
	#[serde(rename = "FullNm", skip_serializing_if = "Option::is_none")]
	pub full_nm: Option<String>,
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
