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


// ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType {
	#[serde(rename = "ActiveOrHistoricCurrencyAnd20DecimalAmount_SimpleType")]
	pub active_or_historic_currency_and20_decimal_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAnd20DecimalAmount ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd20DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyCode {
	#[validate(pattern = "[A-Z]{3,3}")]
	#[serde(rename = "ActiveOrHistoricCurrencyCode")]
	pub active_or_historic_currency_code: String,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[validate(pattern = "[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}")]
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// CollateralPortfolioCode6Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CollateralPortfolioCode6Choice {
	#[validate]
	#[serde(rename = "Prtfl")]
	pub prtfl: Option<PortfolioCode3Choice>,
	#[validate]
	#[serde(rename = "MrgnPrtflCd")]
	pub mrgn_prtfl_cd: Option<MarginPortfolio4>,
}


// CollateralisationType3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CollateralisationType3Code {
	#[validate(enumerate = ["FLCL", "OWCL", "OWC1", "OWC2", "OWP1", "OWP2", "PRCL", "PRC1", "PRC2", "UNCL"])]
	#[serde(rename = "CollateralisationType3Code")]
	pub collateralisation_type3_code: String,
}


// Counterparty45 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Counterparty45 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: PartyIdentification248Choice,
	#[validate]
	#[serde(rename = "Ntr")]
	pub ntr: Option<CounterpartyTradeNature15Choice>,
	#[serde(rename = "TradgCpcty")]
	pub tradg_cpcty: Option<String>,
	#[validate]
	#[serde(rename = "DrctnOrSd")]
	pub drctn_or_sd: Option<Direction4Choice>,
	#[serde(rename = "TradrLctn")]
	pub tradr_lctn: Option<String>,
	#[serde(rename = "BookgLctn")]
	pub bookg_lctn: Option<String>,
	#[validate]
	#[serde(rename = "RptgXmptn")]
	pub rptg_xmptn: Option<ReportingExemption1>,
}


// Counterparty46 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Counterparty46 {
	#[validate]
	#[serde(rename = "IdTp")]
	pub id_tp: Option<PartyIdentification248Choice>,
	#[validate]
	#[serde(rename = "Ntr")]
	pub ntr: Option<CounterpartyTradeNature15Choice>,
	#[serde(rename = "RptgOblgtn")]
	pub rptg_oblgtn: Option<bool>,
}


// CounterpartyTradeNature15Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CounterpartyTradeNature15Choice {
	#[validate]
	#[serde(rename = "FI")]
	pub fi: Option<FinancialInstitutionSector1>,
	#[validate]
	#[serde(rename = "NFI")]
	pub nfi: Option<NonFinancialInstitutionSector10>,
	#[serde(rename = "CntrlCntrPty")]
	pub cntrl_cntr_pty: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<String>,
}


// CountryCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CountryCode {
	#[validate(pattern = "[A-Z]{2,2}")]
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// DerivativesTradeMarginDataReportV02 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DerivativesTradeMarginDataReportV02 {
	#[validate]
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: TradeReportHeader4,
	#[validate]
	#[serde(rename = "TradData")]
	pub trad_data: TradeData61Choice,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// Direction2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Direction2 {
	#[serde(rename = "DrctnOfTheFrstLeg")]
	pub drctn_of_the_frst_leg: String,
	#[serde(rename = "DrctnOfTheScndLeg")]
	pub drctn_of_the_scnd_leg: Option<String>,
}


// Direction4Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Direction4Choice {
	#[validate]
	#[serde(rename = "Drctn")]
	pub drctn: Option<Direction2>,
	#[serde(rename = "CtrPtySd")]
	pub ctr_pty_sd: Option<String>,
}


// ExternalPartyRelationshipType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalPartyRelationshipType1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalPartyRelationshipType1Code")]
	pub external_party_relationship_type1_code: String,
}


// FinancialInstitutionSector1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstitutionSector1 {
	#[validate]
	#[serde(rename = "Sctr")]
	pub sctr: Vec<FinancialPartyClassification2Choice>,
	#[serde(rename = "ClrThrshld")]
	pub clr_thrshld: Option<bool>,
}


// FinancialPartyClassification2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialPartyClassification2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification175>,
}


// FinancialPartySectorType3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialPartySectorType3Code {
	#[validate(enumerate = ["AIFD", "CSDS", "CCPS", "CDTI", "INUN", "ORPI", "INVF", "REIN", "UCIT", "ASSU", "OTHR"])]
	#[serde(rename = "FinancialPartySectorType3Code")]
	pub financial_party_sector_type3_code: String,
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


// MarginCollateralReport5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MarginCollateralReport5 {
	#[validate]
	#[serde(rename = "CollPrtflCd")]
	pub coll_prtfl_cd: CollateralPortfolioCode6Choice,
	#[serde(rename = "CollstnCtgy")]
	pub collstn_ctgy: String,
	#[serde(rename = "TmStmp")]
	pub tm_stmp: Option<String>,
}


// MarginPortfolio4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MarginPortfolio4 {
	#[validate]
	#[serde(rename = "InitlMrgnPrtflCd")]
	pub initl_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
	#[validate]
	#[serde(rename = "VartnMrgnPrtflCd")]
	pub vartn_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
}


// MarginReportData9 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MarginReportData9 {
	#[serde(rename = "RptgTmStmp")]
	pub rptg_tm_stmp: Option<String>,
	#[validate]
	#[serde(rename = "CtrPtyId")]
	pub ctr_pty_id: TradeCounterpartyReport20,
	#[serde(rename = "EvtDt")]
	pub evt_dt: Option<String>,
	#[validate]
	#[serde(rename = "TxId")]
	pub tx_id: Option<UniqueTransactionIdentifier2Choice>,
	#[validate]
	#[serde(rename = "Coll")]
	pub coll: MarginCollateralReport5,
	#[validate]
	#[serde(rename = "PstdMrgnOrColl")]
	pub pstd_mrgn_or_coll: Option<PostedMarginOrCollateral6>,
	#[validate]
	#[serde(rename = "RcvdMrgnOrColl")]
	pub rcvd_mrgn_or_coll: Option<ReceivedMarginOrCollateral6>,
	#[serde(rename = "CtrPtyRatgTrggrInd")]
	pub ctr_pty_ratg_trggr_ind: Option<bool>,
	#[serde(rename = "CtrPtyRatgThrshldInd")]
	pub ctr_pty_ratg_thrshld_ind: Option<bool>,
	#[validate]
	#[serde(rename = "TechAttrbts")]
	pub tech_attrbts: Option<TechnicalAttributes6>,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// Max1000Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max1000Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 1000)]
	#[serde(rename = "Max1000Text")]
	pub max1000_text: String,
}


// Max100Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max100Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 100)]
	#[serde(rename = "Max100Text")]
	pub max100_text: String,
}


// Max105Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max105Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 105)]
	#[serde(rename = "Max105Text")]
	pub max105_text: String,
}


// Max140Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max140Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 140)]
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
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


// Max4Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max4Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "Max4Text")]
	pub max4_text: String,
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


// Max5NumericText ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max5NumericText {
	#[validate(pattern = "[0-9]{1,5}")]
	#[serde(rename = "Max5NumericText")]
	pub max5_numeric_text: String,
}


// Max72Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max72Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 72)]
	#[serde(rename = "Max72Text")]
	pub max72_text: String,
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


// NoReasonCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NoReasonCode {
	#[validate(enumerate = ["NORE"])]
	#[serde(rename = "NoReasonCode")]
	pub no_reason_code: String,
}


// NonFinancialInstitutionSector10 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NonFinancialInstitutionSector10 {
	#[validate]
	#[serde(rename = "Sctr")]
	pub sctr: Vec<GenericIdentification175>,
	#[serde(rename = "ClrThrshld")]
	pub clr_thrshld: Option<bool>,
	#[serde(rename = "DrctlyLkdActvty")]
	pub drctly_lkd_actvty: Option<bool>,
	#[serde(rename = "FdrlInstn")]
	pub fdrl_instn: Option<bool>,
}


// NotApplicable1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NotApplicable1Code {
	#[validate(max_length = 4)]
	#[validate(enumerate = ["NOAP"])]
	#[serde(rename = "NotApplicable1Code")]
	pub not_applicable1_code: String,
}


// Number ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
}


// OptionParty1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OptionParty1Code {
	#[validate(enumerate = ["SLLR", "BYER"])]
	#[serde(rename = "OptionParty1Code")]
	pub option_party1_code: String,
}


// OptionParty3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OptionParty3Code {
	#[validate(enumerate = ["MAKE", "TAKE"])]
	#[serde(rename = "OptionParty3Code")]
	pub option_party3_code: String,
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


// Pagination1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Pagination1 {
	#[serde(rename = "PgNb")]
	pub pg_nb: String,
	#[serde(rename = "LastPgInd")]
	pub last_pg_ind: bool,
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


// PortfolioCode3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PortfolioCode3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "NoPrtfl")]
	pub no_prtfl: Option<String>,
}


// PortfolioCode5Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PortfolioCode5Choice {
	#[validate]
	#[serde(rename = "Prtfl")]
	pub prtfl: Option<PortfolioIdentification3>,
	#[serde(rename = "NoPrtfl")]
	pub no_prtfl: Option<String>,
}


// PortfolioIdentification3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PortfolioIdentification3 {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "PrtflTxXmptn")]
	pub prtfl_tx_xmptn: Option<bool>,
}


// PostedMarginOrCollateral6 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PostedMarginOrCollateral6 {
	#[validate]
	#[serde(rename = "InitlMrgnPstdPreHrcut")]
	pub initl_mrgn_pstd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[validate]
	#[serde(rename = "InitlMrgnPstdPstHrcut")]
	pub initl_mrgn_pstd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[validate]
	#[serde(rename = "VartnMrgnPstdPreHrcut")]
	pub vartn_mrgn_pstd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[validate]
	#[serde(rename = "VartnMrgnPstdPstHrcut")]
	pub vartn_mrgn_pstd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[validate]
	#[serde(rename = "XcssCollPstd")]
	pub xcss_coll_pstd: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
}


// ReceivedMarginOrCollateral6 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReceivedMarginOrCollateral6 {
	#[validate]
	#[serde(rename = "InitlMrgnRcvdPreHrcut")]
	pub initl_mrgn_rcvd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[validate]
	#[serde(rename = "InitlMrgnRcvdPstHrcut")]
	pub initl_mrgn_rcvd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[validate]
	#[serde(rename = "VartnMrgnRcvdPreHrcut")]
	pub vartn_mrgn_rcvd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[validate]
	#[serde(rename = "VartnMrgnRcvdPstHrcut")]
	pub vartn_mrgn_rcvd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[validate]
	#[serde(rename = "XcssCollRcvd")]
	pub xcss_coll_rcvd: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
}


// ReportPeriodActivity1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReportPeriodActivity1Code {
	#[validate(enumerate = ["NOTX"])]
	#[serde(rename = "ReportPeriodActivity1Code")]
	pub report_period_activity1_code: String,
}


// ReportingExemption1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReportingExemption1 {
	#[serde(rename = "Rsn")]
	pub rsn: String,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
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


// TechnicalAttributes6 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TechnicalAttributes6 {
	#[serde(rename = "TechRcrdId")]
	pub tech_rcrd_id: Option<String>,
	#[serde(rename = "RptRctTmStmp")]
	pub rpt_rct_tm_stmp: Option<String>,
}


// TradeCounterpartyRelationship1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeCounterpartyRelationship1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// TradeCounterpartyRelationshipRecord1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeCounterpartyRelationshipRecord1 {
	#[serde(rename = "StartRltshPty")]
	pub start_rltsh_pty: String,
	#[serde(rename = "EndRltshPty")]
	pub end_rltsh_pty: String,
	#[validate]
	#[serde(rename = "RltshTp")]
	pub rltsh_tp: TradeCounterpartyRelationship1Choice,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// TradeCounterpartyReport20 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeCounterpartyReport20 {
	#[validate]
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: Counterparty45,
	#[validate]
	#[serde(rename = "OthrCtrPty")]
	pub othr_ctr_pty: Counterparty46,
	#[validate]
	#[serde(rename = "Brkr")]
	pub brkr: Option<OrganisationIdentification15Choice>,
	#[validate]
	#[serde(rename = "SubmitgAgt")]
	pub submitg_agt: Option<OrganisationIdentification15Choice>,
	#[validate]
	#[serde(rename = "ClrMmb")]
	pub clr_mmb: Option<PartyIdentification248Choice>,
	#[validate]
	#[serde(rename = "Bnfcry")]
	pub bnfcry: Option<Vec<PartyIdentification248Choice>>,
	#[validate]
	#[serde(rename = "NttyRspnsblForRpt")]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
	#[validate]
	#[serde(rename = "ExctnAgt")]
	pub exctn_agt: Option<Vec<OrganisationIdentification15Choice>>,
	#[validate]
	#[serde(rename = "RltshRcrd")]
	pub rltsh_rcrd: Option<Vec<TradeCounterpartyRelationshipRecord1>>,
}


// TradeCounterpartyType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeCounterpartyType1Code {
	#[validate(enumerate = ["BENE", "BROK", "CLEM", "EXEA", "OTHC", "REPC", "SBMA", "ERFR"])]
	#[serde(rename = "TradeCounterpartyType1Code")]
	pub trade_counterparty_type1_code: String,
}


// TradeData61Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeData61Choice {
	#[serde(rename = "DataSetActn")]
	pub data_set_actn: Option<String>,
	#[validate]
	#[serde(rename = "Rpt")]
	pub rpt: Option<Vec<TradeReport34Choice>>,
}


// TradeReport34Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeReport34Choice {
	#[validate]
	#[serde(rename = "New")]
	pub new: Option<MarginReportData9>,
	#[validate]
	#[serde(rename = "MrgnUpd")]
	pub mrgn_upd: Option<MarginReportData9>,
	#[validate]
	#[serde(rename = "Err")]
	pub err: Option<MarginReportData9>,
	#[validate]
	#[serde(rename = "Crrctn")]
	pub crrctn: Option<MarginReportData9>,
}


// TradeReportHeader4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeReportHeader4 {
	#[serde(rename = "RptExctnDt")]
	pub rpt_exctn_dt: Option<String>,
	#[validate]
	#[serde(rename = "MsgPgntn")]
	pub msg_pgntn: Option<Pagination1>,
	#[serde(rename = "NbRcrds")]
	pub nb_rcrds: f64,
	#[serde(rename = "CmptntAuthrty")]
	pub cmptnt_authrty: Option<Vec<String>>,
	#[validate]
	#[serde(rename = "NewTradRpstryIdr")]
	pub new_trad_rpstry_idr: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "RptgPurp")]
	pub rptg_purp: Option<Vec<String>>,
}


// TradingCapacity7Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradingCapacity7Code {
	#[validate(enumerate = ["AGEN", "PRIN"])]
	#[serde(rename = "TradingCapacity7Code")]
	pub trading_capacity7_code: String,
}


// TrueFalseIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}


// UTIIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct UTIIdentifier {
	#[validate(pattern = "[A-Z0-9]{18}[0-9]{2}[A-Z0-9]{0,32}")]
	#[serde(rename = "UTIIdentifier")]
	pub uti_identifier: String,
}


// UniqueTransactionIdentifier2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct UniqueTransactionIdentifier2Choice {
	#[serde(rename = "UnqTxIdr")]
	pub unq_tx_idr: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification175>,
}


// YesNoIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "YesNoIndicator")]
	pub yes_no_indicator: bool,
}
