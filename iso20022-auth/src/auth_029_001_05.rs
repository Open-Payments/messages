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


// AnyMIC1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyMIC1Code {
	#[serde(rename = "AnyMIC1Code")]
	pub any_mic1_code: String,
}


// BasketQuery1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BasketQuery1 {
	#[serde(rename = "Strr", skip_serializing_if = "Option::is_none")]
	pub strr: Option<String>,
	#[serde(rename = "Idr", skip_serializing_if = "Option::is_none")]
	pub idr: Option<String>,
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<String>,
}


// CFIOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[serde(rename = "CFIOct2015Identifier")]
	pub cfi_oct2015_identifier: String,
}


// CorporateSectorCriteria6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CorporateSectorCriteria6 {
	#[serde(rename = "FISctr", skip_serializing_if = "Option::is_none")]
	pub fi_sctr: Option<Vec<String>>,
	#[serde(rename = "NFISctr", skip_serializing_if = "Option::is_none")]
	pub nfi_sctr: Option<Vec<String>>,
	#[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
	pub not_rptd: Option<String>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// DateOrBlankQuery2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateOrBlankQuery2Choice {
	#[serde(rename = "Rg", skip_serializing_if = "Option::is_none")]
	pub rg: Option<DatePeriod1>,
	#[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
	pub not_rptd: Option<String>,
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
	pub not_rptd: Option<String>,
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
	#[serde(rename = "DayOfMonthNumber")]
	pub day_of_month_number: f64,
}


// DerivativeEventType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativeEventType3Code {
	#[serde(rename = "DerivativeEventType3Code")]
	pub derivative_event_type3_code: String,
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
pub struct FinancialInstrumentContractType2Code {
	#[serde(rename = "FinancialInstrumentContractType2Code")]
	pub financial_instrument_contract_type2_code: String,
}


// FinancialPartySectorType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialPartySectorType2Code {
	#[serde(rename = "FinancialPartySectorType2Code")]
	pub financial_party_sector_type2_code: String,
}


// Frequency14Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Frequency14Code {
	#[serde(rename = "Frequency14Code")]
	pub frequency14_code: String,
}


// GenericIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}


// GenericIdentification175 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification175 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}


// ISINOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "ISINOct2015Identifier")]
	pub isin_oct2015_identifier: String,
}


// ISINQueryCriteria1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISINQueryCriteria1 {
	#[serde(rename = "Idr", skip_serializing_if = "Option::is_none")]
	pub idr: Option<Vec<String>>,
	#[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
	pub not_rptd: Option<String>,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LegalPersonIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LegalPersonIdentification1 {
	#[serde(rename = "Id")]
	pub id: OrganisationIdentification15Choice,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<String>,
}


// MICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MICIdentifier {
	#[serde(rename = "MICIdentifier")]
	pub mic_identifier: String,
}


// Max1000Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max1000Text {
	#[serde(rename = "Max1000Text")]
	pub max1000_text: String,
}


// Max105Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max105Text {
	#[serde(rename = "Max105Text")]
	pub max105_text: String,
}


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max16Text {
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
}


// Max25Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max25Text {
	#[serde(rename = "Max25Text")]
	pub max25_text: String,
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


// Max500Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max500Text {
	#[serde(rename = "Max500Text")]
	pub max500_text: String,
}


// Max52Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max52Text {
	#[serde(rename = "Max52Text")]
	pub max52_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// Max72Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max72Text {
	#[serde(rename = "Max72Text")]
	pub max72_text: String,
}


// ModificationLevel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ModificationLevel1Code {
	#[serde(rename = "ModificationLevel1Code")]
	pub modification_level1_code: String,
}


// NameAndAddress5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<PostalAddress1>,
}


// NaturalPersonIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NaturalPersonIdentification2 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
	#[serde(rename = "Dmcl", skip_serializing_if = "Option::is_none")]
	pub dmcl: Option<String>,
}


// NaturalPersonIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NaturalPersonIdentification3 {
	#[serde(rename = "Id")]
	pub id: NaturalPersonIdentification2,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<String>,
}


// NonFinancialPartySector1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NonFinancialPartySector1Code {
	#[serde(rename = "NonFinancialPartySector1Code")]
	pub non_financial_party_sector1_code: String,
}


// NotAvailable1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotAvailable1Code {
	#[serde(rename = "NotAvailable1Code")]
	pub not_available1_code: String,
}


// NotReported1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotReported1Code {
	#[serde(rename = "NotReported1Code")]
	pub not_reported1_code: String,
}


// Operation3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Operation3Code {
	#[serde(rename = "Operation3Code")]
	pub operation3_code: String,
}


// OrganisationIdentification15Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification15Choice {
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<String>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<OrganisationIdentification38>,
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<String>,
}


// OrganisationIdentification38 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification38 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
	#[serde(rename = "Dmcl", skip_serializing_if = "Option::is_none")]
	pub dmcl: Option<String>,
}


// PartyIdentification121Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification121Choice {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<String>,
	#[serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none")]
	pub lgl_ntty_idr: Option<String>,
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
pub struct PartyNatureType1Code {
	#[serde(rename = "PartyNatureType1Code")]
	pub party_nature_type1_code: String,
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


// ProductClassificationCriteria1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProductClassificationCriteria1 {
	#[serde(rename = "ClssfctnFinInstrm", skip_serializing_if = "Option::is_none")]
	pub clssfctn_fin_instrm: Option<Vec<String>>,
	#[serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none")]
	pub unq_pdct_idr: Option<Vec<String>>,
}


// ProductType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProductType4Code {
	#[serde(rename = "ProductType4Code")]
	pub product_type4_code: String,
}


// SecuritiesTradeVenueCriteria1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTradeVenueCriteria1Choice {
	#[serde(rename = "MIC", skip_serializing_if = "Option::is_none")]
	pub mic: Option<Vec<String>>,
	#[serde(rename = "AnyMIC", skip_serializing_if = "Option::is_none")]
	pub any_mic: Option<String>,
}


// SecurityIdentification20Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification20Choice {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<String>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
}


// SecurityIdentificationQuery4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentificationQuery4Choice {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<Vec<String>>,
	#[serde(rename = "AltrntvInstrmId", skip_serializing_if = "Option::is_none")]
	pub altrntv_instrm_id: Option<Vec<String>>,
	#[serde(rename = "NotAvlbl", skip_serializing_if = "Option::is_none")]
	pub not_avlbl: Option<String>,
	#[serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none")]
	pub unq_pdct_idr: Option<Vec<String>>,
	#[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
	pub indx: Option<Vec<SecurityIdentification20Choice>>,
	#[serde(rename = "Bskt", skip_serializing_if = "Option::is_none")]
	pub bskt: Option<Vec<BasketQuery1>>,
	#[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
	pub not_rptd: Option<String>,
}


// SecurityIdentificationQueryCriteria1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentificationQueryCriteria1 {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<Vec<String>>,
	#[serde(rename = "AltrntvInstrmId", skip_serializing_if = "Option::is_none")]
	pub altrntv_instrm_id: Option<Vec<String>>,
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


// TradeAdditionalQueryCriteria9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeAdditionalQueryCriteria9 {
	#[serde(rename = "ActnTp", skip_serializing_if = "Option::is_none")]
	pub actn_tp: Option<Vec<String>>,
	#[serde(rename = "ExctnVn", skip_serializing_if = "Option::is_none")]
	pub exctn_vn: Option<SecuritiesTradeVenueCriteria1Choice>,
	#[serde(rename = "NtrOfCtrPty", skip_serializing_if = "Option::is_none")]
	pub ntr_of_ctr_pty: Option<String>,
	#[serde(rename = "CorpSctr", skip_serializing_if = "Option::is_none")]
	pub corp_sctr: Option<CorporateSectorCriteria6>,
	#[serde(rename = "AsstClss", skip_serializing_if = "Option::is_none")]
	pub asst_clss: Option<Vec<String>>,
	#[serde(rename = "PdctClssfctn", skip_serializing_if = "Option::is_none")]
	pub pdct_clssfctn: Option<ProductClassificationCriteria1>,
	#[serde(rename = "Lvl", skip_serializing_if = "Option::is_none")]
	pub lvl: Option<String>,
	#[serde(rename = "EvtTp", skip_serializing_if = "Option::is_none")]
	pub evt_tp: Option<Vec<String>>,
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
	pub not_rptd: Option<String>,
}


// TradePartyIdentificationQuery11Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradePartyIdentificationQuery11Choice {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Vec<OrganisationIdentification15Choice>>,
	#[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
	pub not_rptd: Option<String>,
}


// TradePartyQueryCriteria7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradePartyQueryCriteria7 {
	#[serde(rename = "Oprtr")]
	pub oprtr: String,
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
	pub frqcy_tp: String,
	#[serde(rename = "DlvryDay", skip_serializing_if = "Option::is_none")]
	pub dlvry_day: Option<Vec<String>>,
	#[serde(rename = "DayOfMnth", skip_serializing_if = "Option::is_none")]
	pub day_of_mnth: Option<Vec<f64>>,
}


// TradeRecurrentQuery7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeRecurrentQuery7 {
	#[serde(rename = "QryTp")]
	pub qry_tp: String,
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
	pub oprtr: String,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Vec<SecurityIdentificationQueryCriteria1>>,
	#[serde(rename = "CtrctTp", skip_serializing_if = "Option::is_none")]
	pub ctrct_tp: Option<Vec<String>>,
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<Vec<ISINQueryCriteria1>>,
	#[serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none")]
	pub unq_pdct_idr: Option<Vec<UPIQueryCriteria1>>,
	#[serde(rename = "UndrlygInstrmId", skip_serializing_if = "Option::is_none")]
	pub undrlyg_instrm_id: Option<Vec<SecurityIdentificationQuery4Choice>>,
}


// TransactionOperationType8Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionOperationType8Code {
	#[serde(rename = "TransactionOperationType8Code")]
	pub transaction_operation_type8_code: String,
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}


// UPIQueryCriteria1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UPIQueryCriteria1 {
	#[serde(rename = "Idr", skip_serializing_if = "Option::is_none")]
	pub idr: Option<Vec<String>>,
	#[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
	pub not_rptd: Option<String>,
}


// WeekDay3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct WeekDay3Code {
	#[serde(rename = "WeekDay3Code")]
	pub week_day3_code: String,
}
