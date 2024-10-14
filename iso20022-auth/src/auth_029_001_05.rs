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


// AnyMIC1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AnyMIC1Code {
	#[serde(rename = "ANYM")]
	CodeANYM,

	#[default]
	UNKOWN
}


// BasketQuery1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BasketQuery1 {
	#[serde(rename = "Strr", skip_serializing_if = "Option::is_none")]
	pub strr: Option<LEIIdentifier>,
	#[serde(rename = "Idr", skip_serializing_if = "Option::is_none")]
	pub idr: Option<Max52Text>,
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
}


// CFIOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[serde(rename = "$value")]
	pub cfi_oct2015_identifier: String,
}


// CorporateSectorCriteria6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CorporateSectorCriteria6 {
	#[serde(rename = "FISctr", skip_serializing_if = "Option::is_none")]
	pub fi_sctr: Option<Vec<FinancialPartySectorType2Code>>,
	#[serde(rename = "NFISctr", skip_serializing_if = "Option::is_none")]
	pub nfi_sctr: Option<Vec<NonFinancialPartySector1Code>>,
	#[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
	pub not_rptd: Option<NotReported1Code>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// DateOrBlankQuery2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateOrBlankQuery2Choice {
	#[serde(rename = "Rg", skip_serializing_if = "Option::is_none")]
	pub rg: Option<DatePeriod1>,
	#[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
	pub not_rptd: Option<NotReported1Code>,
}


// DatePeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriod1 {
	#[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// DateTimeOrBlankQuery1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimeOrBlankQuery1Choice {
	#[serde(rename = "Rg", skip_serializing_if = "Option::is_none")]
	pub rg: Option<DateTimePeriod1>,
	#[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
	pub not_rptd: Option<NotReported1Code>,
}


// DateTimePeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod1 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: String,
}


// DayOfMonthNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DayOfMonthNumber {
	#[serde(rename = "$value")]
	pub day_of_month_number: f64,
}


// DerivativeEventType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum DerivativeEventType3Code {
	#[serde(rename = "ALOC")]
	CodeALOC,
	#[serde(rename = "CLRG")]
	CodeCLRG,
	#[serde(rename = "CLAL")]
	CodeCLAL,
	#[serde(rename = "COMP")]
	CodeCOMP,
	#[serde(rename = "CORP")]
	CodeCORP,
	#[serde(rename = "CREV")]
	CodeCREV,
	#[serde(rename = "ETRM")]
	CodeETRM,
	#[serde(rename = "EXER")]
	CodeEXER,
	#[serde(rename = "INCP")]
	CodeINCP,
	#[serde(rename = "NOVA")]
	CodeNOVA,
	#[serde(rename = "PTNG")]
	CodePTNG,
	#[serde(rename = "TRAD")]
	CodeTRAD,
	#[serde(rename = "UPDT")]
	CodeUPDT,

	#[default]
	UNKOWN
}


// DerivativesTradeReportQueryV05 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativesTradeReportQueryV05 {
	#[serde(rename = "RqstngAuthrty")]
	pub rqstng_authrty: PartyIdentification121Choice,
	#[serde(rename = "TradQryData")]
	pub trad_qry_data: TradeReportQuery18Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// FinancialInstrumentContractType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FinancialInstrumentContractType2Code {
	#[serde(rename = "CFDS")]
	CodeCFDS,
	#[serde(rename = "FRAS")]
	CodeFRAS,
	#[serde(rename = "FUTR")]
	CodeFUTR,
	#[serde(rename = "FORW")]
	CodeFORW,
	#[serde(rename = "OPTN")]
	CodeOPTN,
	#[serde(rename = "SPDB")]
	CodeSPDB,
	#[serde(rename = "SWAP")]
	CodeSWAP,
	#[serde(rename = "SWPT")]
	CodeSWPT,
	#[serde(rename = "OTHR")]
	CodeOTHR,

	#[default]
	UNKOWN
}


// FinancialPartySectorType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FinancialPartySectorType2Code {
	#[serde(rename = "AIFD")]
	CodeAIFD,
	#[serde(rename = "CSDS")]
	CodeCSDS,
	#[serde(rename = "CCPS")]
	CodeCCPS,
	#[serde(rename = "CDTI")]
	CodeCDTI,
	#[serde(rename = "INUN")]
	CodeINUN,
	#[serde(rename = "ORPI")]
	CodeORPI,
	#[serde(rename = "INVF")]
	CodeINVF,
	#[serde(rename = "REIN")]
	CodeREIN,
	#[serde(rename = "UCIT")]
	CodeUCIT,

	#[default]
	UNKOWN
}


// Frequency14Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Frequency14Code {
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "ADHO")]
	CodeADHO,

	#[default]
	UNKOWN
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


// GenericIdentification175 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification175 {
	#[serde(rename = "Id")]
	pub id: Max72Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}


// ISINOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "$value")]
	pub isin_oct2015_identifier: String,
}


// ISINQueryCriteria1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISINQueryCriteria1 {
	#[serde(rename = "Idr", skip_serializing_if = "Option::is_none")]
	pub idr: Option<Vec<ISINOct2015Identifier>>,
	#[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
	pub not_rptd: Option<NotReported1Code>,
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


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}


// LegalPersonIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LegalPersonIdentification1 {
	#[serde(rename = "Id")]
	pub id: OrganisationIdentification15Choice,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
}


// MICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MICIdentifier {
	#[serde(rename = "$value")]
	pub mic_identifier: String,
}


// Max1000Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max1000Text {
	#[serde(rename = "$value")]
	pub max1000_text: String,
}


// Max105Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max105Text {
	#[serde(rename = "$value")]
	pub max105_text: String,
}


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max16Text {
	#[serde(rename = "$value")]
	pub max16_text: String,
}


// Max25Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max25Text {
	#[serde(rename = "$value")]
	pub max25_text: String,
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


// Max500Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max500Text {
	#[serde(rename = "$value")]
	pub max500_text: String,
}


// Max52Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max52Text {
	#[serde(rename = "$value")]
	pub max52_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
}


// Max72Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max72Text {
	#[serde(rename = "$value")]
	pub max72_text: String,
}


// ModificationLevel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ModificationLevel1Code {
	#[serde(rename = "PSTN")]
	CodePSTN,
	#[serde(rename = "TCTN")]
	CodeTCTN,

	#[default]
	UNKOWN
}


// NameAndAddress5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<PostalAddress1>,
}


// NaturalPersonIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NaturalPersonIdentification2 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max105Text>,
	#[serde(rename = "Dmcl", skip_serializing_if = "Option::is_none")]
	pub dmcl: Option<Max500Text>,
}


// NaturalPersonIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NaturalPersonIdentification3 {
	#[serde(rename = "Id")]
	pub id: NaturalPersonIdentification2,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
}


// NonFinancialPartySector1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NonFinancialPartySector1Code {
	#[serde(rename = "WTER")]
	CodeWTER,
	#[serde(rename = "MING")]
	CodeMING,
	#[serde(rename = "MAFG")]
	CodeMAFG,
	#[serde(rename = "SPLY")]
	CodeSPLY,
	#[serde(rename = "CSTR")]
	CodeCSTR,
	#[serde(rename = "AGRI")]
	CodeAGRI,
	#[serde(rename = "ACAF")]
	CodeACAF,
	#[serde(rename = "EDUC")]
	CodeEDUC,
	#[serde(rename = "AEAR")]
	CodeAEAR,
	#[serde(rename = "FINA")]
	CodeFINA,
	#[serde(rename = "HHSW")]
	CodeHHSW,
	#[serde(rename = "INCO")]
	CodeINCO,
	#[serde(rename = "WRRM")]
	CodeWRRM,
	#[serde(rename = "OTSA")]
	CodeOTSA,
	#[serde(rename = "PSTA")]
	CodePSTA,
	#[serde(rename = "PADS")]
	CodePADS,
	#[serde(rename = "RESA")]
	CodeRESA,
	#[serde(rename = "TRAS")]
	CodeTRAS,
	#[serde(rename = "ASSA")]
	CodeASSA,
	#[serde(rename = "AHAE")]
	CodeAHAE,
	#[serde(rename = "AEOB")]
	CodeAEOB,

	#[default]
	UNKOWN
}


// NotAvailable1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NotAvailable1Code {
	#[serde(rename = "NTAV")]
	CodeNTAV,

	#[default]
	UNKOWN
}


// NotReported1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NotReported1Code {
	#[serde(rename = "NORP")]
	CodeNORP,

	#[default]
	UNKOWN
}


// Operation3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Operation3Code {
	#[serde(rename = "ANDD")]
	CodeANDD,
	#[serde(rename = "ORRR")]
	CodeORRR,

	#[default]
	UNKOWN
}


// OrganisationIdentification15Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification15Choice {
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<OrganisationIdentification38>,
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
}


// OrganisationIdentification38 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification38 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max105Text>,
	#[serde(rename = "Dmcl", skip_serializing_if = "Option::is_none")]
	pub dmcl: Option<Max500Text>,
}


// PartyIdentification121Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification121Choice {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
	#[serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none")]
	pub lgl_ntty_idr: Option<LEIIdentifier>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification1>,
}


// PartyIdentification248Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification248Choice {
	#[serde(rename = "Lgl", skip_serializing_if = "Option::is_none")]
	pub lgl: Option<LegalPersonIdentification1>,
	#[serde(rename = "Ntrl", skip_serializing_if = "Option::is_none")]
	pub ntrl: Option<NaturalPersonIdentification3>,
}


// PartyNatureType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PartyNatureType1Code {
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "NFIN")]
	CodeNFIN,
	#[serde(rename = "FIIN")]
	CodeFIIN,
	#[serde(rename = "CCPS")]
	CodeCCPS,

	#[default]
	UNKOWN
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


// ProductClassificationCriteria1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProductClassificationCriteria1 {
	#[serde(rename = "ClssfctnFinInstrm", skip_serializing_if = "Option::is_none")]
	pub clssfctn_fin_instrm: Option<Vec<CFIOct2015Identifier>>,
	#[serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none")]
	pub unq_pdct_idr: Option<Vec<Max52Text>>,
}


// ProductType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ProductType4Code {
	#[serde(rename = "CRDT")]
	CodeCRDT,
	#[serde(rename = "CURR")]
	CodeCURR,
	#[serde(rename = "EQUI")]
	CodeEQUI,
	#[serde(rename = "INTR")]
	CodeINTR,
	#[serde(rename = "COMM")]
	CodeCOMM,
	#[serde(rename = "OTHR")]
	CodeOTHR,

	#[default]
	UNKOWN
}


// SecuritiesTradeVenueCriteria1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTradeVenueCriteria1Choice {
	#[serde(rename = "MIC", skip_serializing_if = "Option::is_none")]
	pub mic: Option<Vec<MICIdentifier>>,
	#[serde(rename = "AnyMIC", skip_serializing_if = "Option::is_none")]
	pub any_mic: Option<AnyMIC1Code>,
}


// SecurityIdentification20Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification20Choice {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max25Text>,
}


// SecurityIdentificationQuery4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentificationQuery4Choice {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<Vec<ISINOct2015Identifier>>,
	#[serde(rename = "AltrntvInstrmId", skip_serializing_if = "Option::is_none")]
	pub altrntv_instrm_id: Option<Vec<Max52Text>>,
	#[serde(rename = "NotAvlbl", skip_serializing_if = "Option::is_none")]
	pub not_avlbl: Option<NotAvailable1Code>,
	#[serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none")]
	pub unq_pdct_idr: Option<Vec<Max52Text>>,
	#[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
	pub indx: Option<Vec<SecurityIdentification20Choice>>,
	#[serde(rename = "Bskt", skip_serializing_if = "Option::is_none")]
	pub bskt: Option<Vec<BasketQuery1>>,
	#[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
	pub not_rptd: Option<NotReported1Code>,
}


// SecurityIdentificationQueryCriteria1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentificationQueryCriteria1 {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<Vec<ISINOct2015Identifier>>,
	#[serde(rename = "AltrntvInstrmId", skip_serializing_if = "Option::is_none")]
	pub altrntv_instrm_id: Option<Vec<Max52Text>>,
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


// TradeAdditionalQueryCriteria9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeAdditionalQueryCriteria9 {
	#[serde(rename = "ActnTp", skip_serializing_if = "Option::is_none")]
	pub actn_tp: Option<Vec<TransactionOperationType8Code>>,
	#[serde(rename = "ExctnVn", skip_serializing_if = "Option::is_none")]
	pub exctn_vn: Option<SecuritiesTradeVenueCriteria1Choice>,
	#[serde(rename = "NtrOfCtrPty", skip_serializing_if = "Option::is_none")]
	pub ntr_of_ctr_pty: Option<PartyNatureType1Code>,
	#[serde(rename = "CorpSctr", skip_serializing_if = "Option::is_none")]
	pub corp_sctr: Option<CorporateSectorCriteria6>,
	#[serde(rename = "AsstClss", skip_serializing_if = "Option::is_none")]
	pub asst_clss: Option<Vec<ProductType4Code>>,
	#[serde(rename = "PdctClssfctn", skip_serializing_if = "Option::is_none")]
	pub pdct_clssfctn: Option<ProductClassificationCriteria1>,
	#[serde(rename = "Lvl", skip_serializing_if = "Option::is_none")]
	pub lvl: Option<ModificationLevel1Code>,
	#[serde(rename = "EvtTp", skip_serializing_if = "Option::is_none")]
	pub evt_tp: Option<Vec<DerivativeEventType3Code>>,
}


// TradeDateTimeQueryCriteria6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeDateTimeQueryCriteria6 {
	#[serde(rename = "RptgDtTm", skip_serializing_if = "Option::is_none")]
	pub rptg_dt_tm: Option<DateTimePeriod1>,
	#[serde(rename = "ExctnDtTm", skip_serializing_if = "Option::is_none")]
	pub exctn_dt_tm: Option<DateTimePeriod1>,
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<DateOrBlankQuery2Choice>,
	#[serde(rename = "FctvDt", skip_serializing_if = "Option::is_none")]
	pub fctv_dt: Option<DatePeriod1>,
	#[serde(rename = "ValtnDtTm", skip_serializing_if = "Option::is_none")]
	pub valtn_dt_tm: Option<DateTimePeriod1>,
	#[serde(rename = "XprtnDt", skip_serializing_if = "Option::is_none")]
	pub xprtn_dt: Option<DateOrBlankQuery2Choice>,
	#[serde(rename = "EarlyTermntnDt", skip_serializing_if = "Option::is_none")]
	pub early_termntn_dt: Option<DatePeriod1>,
	#[serde(rename = "CollTmStmp", skip_serializing_if = "Option::is_none")]
	pub coll_tm_stmp: Option<DateTimeOrBlankQuery1Choice>,
	#[serde(rename = "HstrclAsOfDt", skip_serializing_if = "Option::is_none")]
	pub hstrcl_as_of_dt: Option<String>,
}


// TradePartyIdentificationQuery10Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradePartyIdentificationQuery10Choice {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Vec<PartyIdentification248Choice>>,
	#[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
	pub not_rptd: Option<NotReported1Code>,
}


// TradePartyIdentificationQuery11Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradePartyIdentificationQuery11Choice {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Vec<OrganisationIdentification15Choice>>,
	#[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
	pub not_rptd: Option<NotReported1Code>,
}


// TradePartyQueryCriteria7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradePartyQueryCriteria7 {
	#[serde(rename = "Oprtr")]
	pub oprtr: Operation3Code,
	#[serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none")]
	pub rptg_ctr_pty: Option<TradePartyIdentificationQuery10Choice>,
	#[serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none")]
	pub othr_ctr_pty: Option<TradePartyIdentificationQuery10Choice>,
	#[serde(rename = "Bnfcry", skip_serializing_if = "Option::is_none")]
	pub bnfcry: Option<TradePartyIdentificationQuery10Choice>,
	#[serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none")]
	pub ntty_rspnsbl_for_rpt: Option<TradePartyIdentificationQuery11Choice>,
	#[serde(rename = "SubmitgAgt", skip_serializing_if = "Option::is_none")]
	pub submitg_agt: Option<TradePartyIdentificationQuery11Choice>,
	#[serde(rename = "Brkr", skip_serializing_if = "Option::is_none")]
	pub brkr: Option<TradePartyIdentificationQuery11Choice>,
	#[serde(rename = "CCP", skip_serializing_if = "Option::is_none")]
	pub ccp: Option<TradePartyIdentificationQuery11Choice>,
	#[serde(rename = "ClrMmb", skip_serializing_if = "Option::is_none")]
	pub clr_mmb: Option<TradePartyIdentificationQuery10Choice>,
}


// TradeQueryCriteria14 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeQueryCriteria14 {
	#[serde(rename = "TradLifeCyclHstry", skip_serializing_if = "Option::is_none")]
	pub trad_life_cycl_hstry: Option<bool>,
	#[serde(rename = "MrgnLifeCyclHstry", skip_serializing_if = "Option::is_none")]
	pub mrgn_life_cycl_hstry: Option<bool>,
	#[serde(rename = "OutsdngTradInd")]
	pub outsdng_trad_ind: bool,
	#[serde(rename = "TradPtyCrit", skip_serializing_if = "Option::is_none")]
	pub trad_pty_crit: Option<TradePartyQueryCriteria7>,
	#[serde(rename = "FinInstrmCrit", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_crit: Option<TradeSecurityIdentificationQueryCriteria3>,
	#[serde(rename = "TmCrit", skip_serializing_if = "Option::is_none")]
	pub tm_crit: Option<TradeDateTimeQueryCriteria6>,
	#[serde(rename = "OthrCrit", skip_serializing_if = "Option::is_none")]
	pub othr_crit: Option<TradeAdditionalQueryCriteria9>,
}


// TradeQueryExecutionFrequency3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeQueryExecutionFrequency3 {
	#[serde(rename = "FrqcyTp")]
	pub frqcy_tp: Frequency14Code,
	#[serde(rename = "DlvryDay", skip_serializing_if = "Option::is_none")]
	pub dlvry_day: Option<Vec<WeekDay3Code>>,
	#[serde(rename = "DayOfMnth", skip_serializing_if = "Option::is_none")]
	pub day_of_mnth: Option<Vec<DayOfMonthNumber>>,
}


// TradeRecurrentQuery7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeRecurrentQuery7 {
	#[serde(rename = "QryTp")]
	pub qry_tp: Max1000Text,
	#[serde(rename = "Frqcy")]
	pub frqcy: Vec<TradeQueryExecutionFrequency3>,
	#[serde(rename = "VldUntil")]
	pub vld_until: String,
}


// TradeReportQuery18Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeReportQuery18Choice {
	#[serde(rename = "AdHocQry", skip_serializing_if = "Option::is_none")]
	pub ad_hoc_qry: Option<TradeQueryCriteria14>,
	#[serde(rename = "RcrntQry", skip_serializing_if = "Option::is_none")]
	pub rcrnt_qry: Option<TradeRecurrentQuery7>,
}


// TradeSecurityIdentificationQueryCriteria3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeSecurityIdentificationQueryCriteria3 {
	#[serde(rename = "Oprtr")]
	pub oprtr: Operation3Code,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Vec<SecurityIdentificationQueryCriteria1>>,
	#[serde(rename = "CtrctTp", skip_serializing_if = "Option::is_none")]
	pub ctrct_tp: Option<Vec<FinancialInstrumentContractType2Code>>,
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<Vec<ISINQueryCriteria1>>,
	#[serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none")]
	pub unq_pdct_idr: Option<Vec<UPIQueryCriteria1>>,
	#[serde(rename = "UndrlygInstrmId", skip_serializing_if = "Option::is_none")]
	pub undrlyg_instrm_id: Option<Vec<SecurityIdentificationQuery4Choice>>,
}


// TransactionOperationType8Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TransactionOperationType8Code {
	#[serde(rename = "COMP")]
	CodeCOMP,
	#[serde(rename = "CORR")]
	CodeCORR,
	#[serde(rename = "EROR")]
	CodeEROR,
	#[serde(rename = "MODI")]
	CodeMODI,
	#[serde(rename = "NEWT")]
	CodeNEWT,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "POSC")]
	CodePOSC,
	#[serde(rename = "REVI")]
	CodeREVI,
	#[serde(rename = "TERM")]
	CodeTERM,
	#[serde(rename = "VALU")]
	CodeVALU,
	#[serde(rename = "MARU")]
	CodeMARU,

	#[default]
	UNKOWN
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}


// UPIQueryCriteria1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UPIQueryCriteria1 {
	#[serde(rename = "Idr", skip_serializing_if = "Option::is_none")]
	pub idr: Option<Vec<Max52Text>>,
	#[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
	pub not_rptd: Option<NotReported1Code>,
}


// WeekDay3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum WeekDay3Code {
	#[serde(rename = "ALLD")]
	CodeALLD,
	#[serde(rename = "XBHL")]
	CodeXBHL,
	#[serde(rename = "IBHL")]
	CodeIBHL,
	#[serde(rename = "FRID")]
	CodeFRID,
	#[serde(rename = "MOND")]
	CodeMOND,
	#[serde(rename = "SATD")]
	CodeSATD,
	#[serde(rename = "SUND")]
	CodeSUND,
	#[serde(rename = "THUD")]
	CodeTHUD,
	#[serde(rename = "TUED")]
	CodeTUED,
	#[serde(rename = "WEDD")]
	CodeWEDD,
	#[serde(rename = "WDAY")]
	CodeWDAY,
	#[serde(rename = "WEND")]
	CodeWEND,

	#[default]
	UNKOWN
}
