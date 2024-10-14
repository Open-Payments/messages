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


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_code: String,
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


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
}


// CFIOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[serde(rename = "$value")]
	pub cfi_oct2015_identifier: String,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// DatePeriod2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriod2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// DatePeriodSearch1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriodSearch1Choice {
	#[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt", skip_serializing_if = "Option::is_none")]
	pub to_dt: Option<String>,
	#[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
	pub fr_to_dt: Option<DatePeriod2>,
	#[serde(rename = "EQDt", skip_serializing_if = "Option::is_none")]
	pub eq_dt: Option<String>,
	#[serde(rename = "NEQDt", skip_serializing_if = "Option::is_none")]
	pub neq_dt: Option<String>,
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "$value")]
	pub exact4_alpha_numeric_text: String,
}


// ExternalFinancialInstrumentIdentificationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalFinancialInstrumentIdentificationType1Code {
	#[serde(rename = "$value")]
	pub external_financial_instrument_identification_type1_code: String,
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


// ISIN2021Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISIN2021Identifier {
	#[serde(rename = "$value")]
	pub isin2021_identifier: String,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}


// IdentificationSource3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IdentificationSource3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalFinancialInstrumentIdentificationType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
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


// MessageHeader1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader1 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
}


// NameAndAddress5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<PostalAddress1>,
}


// OtherIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Sfx", skip_serializing_if = "Option::is_none")]
	pub sfx: Option<Max16Text>,
	#[serde(rename = "Tp")]
	pub tp: IdentificationSource3Choice,
}


// PartyIdentification120Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification120Choice {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
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
	pub lei: Option<LEIIdentifier>,
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


// RequestedIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestedIndicator {
	#[serde(rename = "$value")]
	pub requested_indicator: bool,
}


// SecuritiesReturnCriteria1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesReturnCriteria1 {
	#[serde(rename = "FinInstrmId")]
	pub fin_instrm_id: bool,
	#[serde(rename = "ISOSctyLngNm")]
	pub iso_scty_lng_nm: bool,
	#[serde(rename = "ISOSctyShrtNm")]
	pub iso_scty_shrt_nm: bool,
	#[serde(rename = "ClssfctnFinInstrm")]
	pub clssfctn_fin_instrm: bool,
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: bool,
	#[serde(rename = "IsseDt")]
	pub isse_dt: bool,
	#[serde(rename = "IsseCcy")]
	pub isse_ccy: bool,
	#[serde(rename = "CtryOfIsse")]
	pub ctry_of_isse: bool,
	#[serde(rename = "SctySts")]
	pub scty_sts: bool,
	#[serde(rename = "InvstrCSD")]
	pub invstr_csd: bool,
	#[serde(rename = "IssrCSD")]
	pub issr_csd: bool,
	#[serde(rename = "TechIssrCSD")]
	pub tech_issr_csd: bool,
	#[serde(rename = "CSD")]
	pub csd: bool,
	#[serde(rename = "SctiesQtyTp")]
	pub scties_qty_tp: bool,
	#[serde(rename = "MinDnmtn")]
	pub min_dnmtn: bool,
	#[serde(rename = "MinMltplQty")]
	pub min_mltpl_qty: bool,
	#[serde(rename = "DevtgSttlmUnit")]
	pub devtg_sttlm_unit: bool,
}


// SecuritiesSearchCriteria4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesSearchCriteria4 {
	#[serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_id: Option<SecurityIdentification39>,
	#[serde(rename = "ClssfctnFinInstrm", skip_serializing_if = "Option::is_none")]
	pub clssfctn_fin_instrm: Option<CFIOct2015Identifier>,
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<DatePeriodSearch1Choice>,
	#[serde(rename = "IsseDt", skip_serializing_if = "Option::is_none")]
	pub isse_dt: Option<DatePeriodSearch1Choice>,
	#[serde(rename = "IsseCcy", skip_serializing_if = "Option::is_none")]
	pub isse_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "CtryOfIsse", skip_serializing_if = "Option::is_none")]
	pub ctry_of_isse: Option<CountryCode>,
	#[serde(rename = "SctySts", skip_serializing_if = "Option::is_none")]
	pub scty_sts: Option<SecurityStatus3Choice>,
	#[serde(rename = "MntngCSD", skip_serializing_if = "Option::is_none")]
	pub mntng_csd: Option<SystemPartyIdentification2Choice>,
	#[serde(rename = "InvstrCSD", skip_serializing_if = "Option::is_none")]
	pub invstr_csd: Option<SystemPartyIdentification2Choice>,
	#[serde(rename = "IssrCSD", skip_serializing_if = "Option::is_none")]
	pub issr_csd: Option<SystemPartyIdentification2Choice>,
	#[serde(rename = "TechIssrCSD", skip_serializing_if = "Option::is_none")]
	pub tech_issr_csd: Option<SystemPartyIdentification2Choice>,
	#[serde(rename = "CSD", skip_serializing_if = "Option::is_none")]
	pub csd: Option<SystemPartyIdentification2Choice>,
}


// SecurityIdentification39 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification39 {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISIN2021Identifier>,
	#[serde(rename = "OthrId", skip_serializing_if = "Option::is_none")]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max140Text>,
}


// SecurityQueryV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityQueryV01 {
	#[serde(rename = "MsgHdr", skip_serializing_if = "Option::is_none")]
	pub msg_hdr: Option<MessageHeader1>,
	#[serde(rename = "ReqTp", skip_serializing_if = "Option::is_none")]
	pub req_tp: Option<GenericIdentification1>,
	#[serde(rename = "SchCrit")]
	pub sch_crit: SecuritiesSearchCriteria4,
	#[serde(rename = "SmlSetRtrCrit", skip_serializing_if = "Option::is_none")]
	pub sml_set_rtr_crit: Option<SecuritiesReturnCriteria1>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// SecurityStatus2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SecurityStatus2Code {
	#[serde(rename = "ACTV")]
	CodeACTV,
	#[serde(rename = "INAC")]
	CodeINAC,
	#[serde(rename = "SUSP")]
	CodeSUSP,

	#[default]
	UNKOWN
}


// SecurityStatus3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityStatus3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<SecurityStatus2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
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


// SystemPartyIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemPartyIdentification2Choice {
	#[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
	pub org_id: Option<PartyIdentification136>,
	#[serde(rename = "CmbndId", skip_serializing_if = "Option::is_none")]
	pub cmbnd_id: Option<SystemPartyIdentification8>,
}


// SystemPartyIdentification8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemPartyIdentification8 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification136,
	#[serde(rename = "RspnsblPtyId", skip_serializing_if = "Option::is_none")]
	pub rspnsbl_pty_id: Option<PartyIdentification136>,
}
