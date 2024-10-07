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


// AddressType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AddressType2Code {
	#[validate(enumerate = ["ADDR", "PBOX", "HOME", "BIZZ", "MLTO", "DLVY"])]
	#[serde(rename = "AddressType2Code")]
	pub address_type2_code: String,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[validate(pattern = "[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}")]
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// AnyMIC1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AnyMIC1Code {
	#[validate(enumerate = ["ANYM"])]
	#[serde(rename = "AnyMIC1Code")]
	pub any_mic1_code: String,
}


// BasketQuery1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BasketQuery1 {
	#[serde(rename = "Strr")]
	pub strr: Option<String>,
	#[serde(rename = "Idr")]
	pub idr: Option<String>,
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
}


// CFIOct2015Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[validate(pattern = "[A-Z]{6,6}")]
	#[serde(rename = "CFIOct2015Identifier")]
	pub cfi_oct2015_identifier: String,
}


// CorporateSectorCriteria6 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CorporateSectorCriteria6 {
	#[serde(rename = "FISctr")]
	pub fi_sctr: Option<Vec<String>>,
	#[serde(rename = "NFISctr")]
	pub nfi_sctr: Option<Vec<String>>,
	#[serde(rename = "NotRptd")]
	pub not_rptd: Option<String>,
}


// CountryCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CountryCode {
	#[validate(pattern = "[A-Z]{2,2}")]
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// DateOrBlankQuery2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DateOrBlankQuery2Choice {
	#[validate]
	#[serde(rename = "Rg")]
	pub rg: Option<DatePeriod1>,
	#[serde(rename = "NotRptd")]
	pub not_rptd: Option<String>,
}


// DatePeriod1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DatePeriod1 {
	#[serde(rename = "FrDt")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// DateTimeOrBlankQuery1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DateTimeOrBlankQuery1Choice {
	#[validate]
	#[serde(rename = "Rg")]
	pub rg: Option<DateTimePeriod1>,
	#[serde(rename = "NotRptd")]
	pub not_rptd: Option<String>,
}


// DateTimePeriod1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DateTimePeriod1 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: String,
}


// DayOfMonthNumber ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DayOfMonthNumber {
	#[serde(rename = "DayOfMonthNumber")]
	pub day_of_month_number: f64,
}


// DerivativeEventType3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DerivativeEventType3Code {
	#[validate(enumerate = ["ALOC", "CLRG", "CLAL", "COMP", "CORP", "CREV", "ETRM", "EXER", "INCP", "NOVA", "PTNG", "TRAD", "UPDT"])]
	#[serde(rename = "DerivativeEventType3Code")]
	pub derivative_event_type3_code: String,
}


// DerivativesTradeReportQueryV05 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DerivativesTradeReportQueryV05 {
	#[validate]
	#[serde(rename = "RqstngAuthrty")]
	pub rqstng_authrty: PartyIdentification121Choice,
	#[validate]
	#[serde(rename = "TradQryData")]
	pub trad_qry_data: TradeReportQuery18Choice,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// FinancialInstrumentContractType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstrumentContractType2Code {
	#[validate(enumerate = ["CFDS", "FRAS", "FUTR", "FORW", "OPTN", "SPDB", "SWAP", "SWPT", "OTHR"])]
	#[serde(rename = "FinancialInstrumentContractType2Code")]
	pub financial_instrument_contract_type2_code: String,
}


// FinancialPartySectorType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialPartySectorType2Code {
	#[validate(enumerate = ["AIFD", "CSDS", "CCPS", "CDTI", "INUN", "ORPI", "INVF", "REIN", "UCIT"])]
	#[serde(rename = "FinancialPartySectorType2Code")]
	pub financial_party_sector_type2_code: String,
}


// Frequency14Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Frequency14Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[validate(enumerate = ["DAIL", "WEEK", "MNTH", "ADHO"])]
	#[serde(rename = "Frequency14Code")]
	pub frequency14_code: String,
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


// GenericIdentification175 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification175 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// ISINOct2015Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[validate(pattern = "[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}")]
	#[serde(rename = "ISINOct2015Identifier")]
	pub isin_oct2015_identifier: String,
}


// ISINQueryCriteria1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISINQueryCriteria1 {
	#[serde(rename = "Idr")]
	pub idr: Option<Vec<String>>,
	#[serde(rename = "NotRptd")]
	pub not_rptd: Option<String>,
}


// ISODate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// LEIIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[validate(pattern = "[A-Z0-9]{18,18}[0-9]{2,2}")]
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LegalPersonIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LegalPersonIdentification1 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: OrganisationIdentification15Choice,
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
}


// MICIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MICIdentifier {
	#[validate(pattern = "[A-Z0-9]{4,4}")]
	#[serde(rename = "MICIdentifier")]
	pub mic_identifier: String,
}


// Max1000Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max1000Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 1000)]
	#[serde(rename = "Max1000Text")]
	pub max1000_text: String,
}


// Max105Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max105Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 105)]
	#[serde(rename = "Max105Text")]
	pub max105_text: String,
}


// Max16Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max16Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 16)]
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
}


// Max25Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max25Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 25)]
	#[serde(rename = "Max25Text")]
	pub max25_text: String,
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


// Max500Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max500Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 500)]
	#[serde(rename = "Max500Text")]
	pub max500_text: String,
}


// Max52Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max52Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 52)]
	#[serde(rename = "Max52Text")]
	pub max52_text: String,
}


// Max70Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max70Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 70)]
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// Max72Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max72Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 72)]
	#[serde(rename = "Max72Text")]
	pub max72_text: String,
}


// ModificationLevel1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ModificationLevel1Code {
	#[validate(enumerate = ["PSTN", "TCTN"])]
	#[serde(rename = "ModificationLevel1Code")]
	pub modification_level1_code: String,
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


// NaturalPersonIdentification2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NaturalPersonIdentification2 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Dmcl")]
	pub dmcl: Option<String>,
}


// NaturalPersonIdentification3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NaturalPersonIdentification3 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: NaturalPersonIdentification2,
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
}


// NonFinancialPartySector1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NonFinancialPartySector1Code {
	#[validate(enumerate = ["WTER", "MING", "MAFG", "SPLY", "CSTR", "AGRI", "ACAF", "EDUC", "AEAR", "FINA", "HHSW", "INCO", "WRRM", "OTSA", "PSTA", "PADS", "RESA", "TRAS", "ASSA", "AHAE", "AEOB"])]
	#[serde(rename = "NonFinancialPartySector1Code")]
	pub non_financial_party_sector1_code: String,
}


// NotAvailable1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NotAvailable1Code {
	#[validate(enumerate = ["NTAV"])]
	#[serde(rename = "NotAvailable1Code")]
	pub not_available1_code: String,
}


// NotReported1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NotReported1Code {
	#[validate(enumerate = ["NORP"])]
	#[serde(rename = "NotReported1Code")]
	pub not_reported1_code: String,
}


// Operation3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Operation3Code {
	#[validate(enumerate = ["ANDD", "ORRR"])]
	#[serde(rename = "Operation3Code")]
	pub operation3_code: String,
}


// OrganisationIdentification15Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OrganisationIdentification15Choice {
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<OrganisationIdentification38>,
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
}


// OrganisationIdentification38 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OrganisationIdentification38 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Dmcl")]
	pub dmcl: Option<String>,
}


// PartyIdentification121Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification121Choice {
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
	#[serde(rename = "LglNttyIdr")]
	pub lgl_ntty_idr: Option<String>,
	#[validate]
	#[serde(rename = "NmAndAdr")]
	pub nm_and_adr: Option<NameAndAddress5>,
	#[validate]
	#[serde(rename = "PrtryId")]
	pub prtry_id: Option<GenericIdentification1>,
}


// PartyIdentification248Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification248Choice {
	#[validate]
	#[serde(rename = "Lgl")]
	pub lgl: Option<LegalPersonIdentification1>,
	#[validate]
	#[serde(rename = "Ntrl")]
	pub ntrl: Option<NaturalPersonIdentification3>,
}


// PartyNatureType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyNatureType1Code {
	#[validate(enumerate = ["OTHR", "NFIN", "FIIN", "CCPS"])]
	#[serde(rename = "PartyNatureType1Code")]
	pub party_nature_type1_code: String,
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


// ProductClassificationCriteria1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ProductClassificationCriteria1 {
	#[serde(rename = "ClssfctnFinInstrm")]
	pub clssfctn_fin_instrm: Option<Vec<String>>,
	#[serde(rename = "UnqPdctIdr")]
	pub unq_pdct_idr: Option<Vec<String>>,
}


// ProductType4Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ProductType4Code {
	#[validate(enumerate = ["CRDT", "CURR", "EQUI", "INTR", "COMM", "OTHR"])]
	#[serde(rename = "ProductType4Code")]
	pub product_type4_code: String,
}


// SecuritiesTradeVenueCriteria1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesTradeVenueCriteria1Choice {
	#[serde(rename = "MIC")]
	pub mic: Option<Vec<String>>,
	#[serde(rename = "AnyMIC")]
	pub any_mic: Option<String>,
}


// SecurityIdentification20Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityIdentification20Choice {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
}


// SecurityIdentificationQuery4Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityIdentificationQuery4Choice {
	#[serde(rename = "ISIN")]
	pub isin: Option<Vec<String>>,
	#[serde(rename = "AltrntvInstrmId")]
	pub altrntv_instrm_id: Option<Vec<String>>,
	#[serde(rename = "NotAvlbl")]
	pub not_avlbl: Option<String>,
	#[serde(rename = "UnqPdctIdr")]
	pub unq_pdct_idr: Option<Vec<String>>,
	#[validate]
	#[serde(rename = "Indx")]
	pub indx: Option<Vec<SecurityIdentification20Choice>>,
	#[validate]
	#[serde(rename = "Bskt")]
	pub bskt: Option<Vec<BasketQuery1>>,
	#[serde(rename = "NotRptd")]
	pub not_rptd: Option<String>,
}


// SecurityIdentificationQueryCriteria1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityIdentificationQueryCriteria1 {
	#[serde(rename = "ISIN")]
	pub isin: Option<Vec<String>>,
	#[serde(rename = "AltrntvInstrmId")]
	pub altrntv_instrm_id: Option<Vec<String>>,
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


// TradeAdditionalQueryCriteria9 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeAdditionalQueryCriteria9 {
	#[serde(rename = "ActnTp")]
	pub actn_tp: Option<Vec<String>>,
	#[validate]
	#[serde(rename = "ExctnVn")]
	pub exctn_vn: Option<SecuritiesTradeVenueCriteria1Choice>,
	#[serde(rename = "NtrOfCtrPty")]
	pub ntr_of_ctr_pty: Option<String>,
	#[validate]
	#[serde(rename = "CorpSctr")]
	pub corp_sctr: Option<CorporateSectorCriteria6>,
	#[serde(rename = "AsstClss")]
	pub asst_clss: Option<Vec<String>>,
	#[validate]
	#[serde(rename = "PdctClssfctn")]
	pub pdct_clssfctn: Option<ProductClassificationCriteria1>,
	#[serde(rename = "Lvl")]
	pub lvl: Option<String>,
	#[serde(rename = "EvtTp")]
	pub evt_tp: Option<Vec<String>>,
}


// TradeDateTimeQueryCriteria6 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeDateTimeQueryCriteria6 {
	#[validate]
	#[serde(rename = "RptgDtTm")]
	pub rptg_dt_tm: Option<DateTimePeriod1>,
	#[validate]
	#[serde(rename = "ExctnDtTm")]
	pub exctn_dt_tm: Option<DateTimePeriod1>,
	#[validate]
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: Option<DateOrBlankQuery2Choice>,
	#[validate]
	#[serde(rename = "FctvDt")]
	pub fctv_dt: Option<DatePeriod1>,
	#[validate]
	#[serde(rename = "ValtnDtTm")]
	pub valtn_dt_tm: Option<DateTimePeriod1>,
	#[validate]
	#[serde(rename = "XprtnDt")]
	pub xprtn_dt: Option<DateOrBlankQuery2Choice>,
	#[validate]
	#[serde(rename = "EarlyTermntnDt")]
	pub early_termntn_dt: Option<DatePeriod1>,
	#[validate]
	#[serde(rename = "CollTmStmp")]
	pub coll_tm_stmp: Option<DateTimeOrBlankQuery1Choice>,
	#[serde(rename = "HstrclAsOfDt")]
	pub hstrcl_as_of_dt: Option<String>,
}


// TradePartyIdentificationQuery10Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradePartyIdentificationQuery10Choice {
	#[validate]
	#[serde(rename = "Id")]
	pub id: Option<Vec<PartyIdentification248Choice>>,
	#[serde(rename = "NotRptd")]
	pub not_rptd: Option<String>,
}


// TradePartyIdentificationQuery11Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradePartyIdentificationQuery11Choice {
	#[validate]
	#[serde(rename = "Id")]
	pub id: Option<Vec<OrganisationIdentification15Choice>>,
	#[serde(rename = "NotRptd")]
	pub not_rptd: Option<String>,
}


// TradePartyQueryCriteria7 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradePartyQueryCriteria7 {
	#[serde(rename = "Oprtr")]
	pub oprtr: String,
	#[validate]
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: Option<TradePartyIdentificationQuery10Choice>,
	#[validate]
	#[serde(rename = "OthrCtrPty")]
	pub othr_ctr_pty: Option<TradePartyIdentificationQuery10Choice>,
	#[validate]
	#[serde(rename = "Bnfcry")]
	pub bnfcry: Option<TradePartyIdentificationQuery10Choice>,
	#[validate]
	#[serde(rename = "NttyRspnsblForRpt")]
	pub ntty_rspnsbl_for_rpt: Option<TradePartyIdentificationQuery11Choice>,
	#[validate]
	#[serde(rename = "SubmitgAgt")]
	pub submitg_agt: Option<TradePartyIdentificationQuery11Choice>,
	#[validate]
	#[serde(rename = "Brkr")]
	pub brkr: Option<TradePartyIdentificationQuery11Choice>,
	#[validate]
	#[serde(rename = "CCP")]
	pub ccp: Option<TradePartyIdentificationQuery11Choice>,
	#[validate]
	#[serde(rename = "ClrMmb")]
	pub clr_mmb: Option<TradePartyIdentificationQuery10Choice>,
}


// TradeQueryCriteria14 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeQueryCriteria14 {
	#[serde(rename = "TradLifeCyclHstry")]
	pub trad_life_cycl_hstry: Option<bool>,
	#[serde(rename = "MrgnLifeCyclHstry")]
	pub mrgn_life_cycl_hstry: Option<bool>,
	#[serde(rename = "OutsdngTradInd")]
	pub outsdng_trad_ind: bool,
	#[validate]
	#[serde(rename = "TradPtyCrit")]
	pub trad_pty_crit: Option<TradePartyQueryCriteria7>,
	#[validate]
	#[serde(rename = "FinInstrmCrit")]
	pub fin_instrm_crit: Option<TradeSecurityIdentificationQueryCriteria3>,
	#[validate]
	#[serde(rename = "TmCrit")]
	pub tm_crit: Option<TradeDateTimeQueryCriteria6>,
	#[validate]
	#[serde(rename = "OthrCrit")]
	pub othr_crit: Option<TradeAdditionalQueryCriteria9>,
}


// TradeQueryExecutionFrequency3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeQueryExecutionFrequency3 {
	#[serde(rename = "FrqcyTp")]
	pub frqcy_tp: String,
	#[serde(rename = "DlvryDay")]
	pub dlvry_day: Option<Vec<String>>,
	#[serde(rename = "DayOfMnth")]
	pub day_of_mnth: Option<Vec<f64>>,
}


// TradeRecurrentQuery7 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeRecurrentQuery7 {
	#[serde(rename = "QryTp")]
	pub qry_tp: String,
	#[validate]
	#[serde(rename = "Frqcy")]
	pub frqcy: Vec<TradeQueryExecutionFrequency3>,
	#[serde(rename = "VldUntil")]
	pub vld_until: String,
}


// TradeReportQuery18Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeReportQuery18Choice {
	#[validate]
	#[serde(rename = "AdHocQry")]
	pub ad_hoc_qry: Option<TradeQueryCriteria14>,
	#[validate]
	#[serde(rename = "RcrntQry")]
	pub rcrnt_qry: Option<TradeRecurrentQuery7>,
}


// TradeSecurityIdentificationQueryCriteria3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeSecurityIdentificationQueryCriteria3 {
	#[serde(rename = "Oprtr")]
	pub oprtr: String,
	#[validate]
	#[serde(rename = "Id")]
	pub id: Option<Vec<SecurityIdentificationQueryCriteria1>>,
	#[serde(rename = "CtrctTp")]
	pub ctrct_tp: Option<Vec<String>>,
	#[validate]
	#[serde(rename = "ISIN")]
	pub isin: Option<Vec<ISINQueryCriteria1>>,
	#[validate]
	#[serde(rename = "UnqPdctIdr")]
	pub unq_pdct_idr: Option<Vec<UPIQueryCriteria1>>,
	#[validate]
	#[serde(rename = "UndrlygInstrmId")]
	pub undrlyg_instrm_id: Option<Vec<SecurityIdentificationQuery4Choice>>,
}


// TransactionOperationType8Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TransactionOperationType8Code {
	#[validate(enumerate = ["COMP", "CORR", "EROR", "MODI", "NEWT", "OTHR", "POSC", "REVI", "TERM", "VALU", "MARU"])]
	#[serde(rename = "TransactionOperationType8Code")]
	pub transaction_operation_type8_code: String,
}


// TrueFalseIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}


// UPIQueryCriteria1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct UPIQueryCriteria1 {
	#[serde(rename = "Idr")]
	pub idr: Option<Vec<String>>,
	#[serde(rename = "NotRptd")]
	pub not_rptd: Option<String>,
}


// WeekDay3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct WeekDay3Code {
	#[validate(enumerate = ["ALLD", "XBHL", "IBHL", "FRID", "MOND", "SATD", "SUND", "THUD", "TUED", "WEDD", "WDAY", "WEND"])]
	#[serde(rename = "WeekDay3Code")]
	pub week_day3_code: String,
}
