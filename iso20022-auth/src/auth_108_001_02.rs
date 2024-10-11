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


// ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType {
	#[serde(rename = "ActiveOrHistoricCurrencyAnd20DecimalAmount_SimpleType")]
	pub active_or_historic_currency_and20_decimal_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAnd20DecimalAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd20DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "ActiveOrHistoricCurrencyCode")]
	pub active_or_historic_currency_code: String,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// CollateralPortfolioCode6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralPortfolioCode6Choice {
	#[serde(rename = "Prtfl", skip_serializing_if = "Option::is_none")]
	pub prtfl: Option<PortfolioCode3Choice>,
	#[serde(rename = "MrgnPrtflCd", skip_serializing_if = "Option::is_none")]
	pub mrgn_prtfl_cd: Option<MarginPortfolio4>,
}


// CollateralisationType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralisationType3Code {
	#[serde(rename = "CollateralisationType3Code")]
	pub collateralisation_type3_code: String,
}


// Counterparty45 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Counterparty45 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification248Choice,
	#[serde(rename = "Ntr", skip_serializing_if = "Option::is_none")]
	pub ntr: Option<CounterpartyTradeNature15Choice>,
	#[serde(rename = "TradgCpcty", skip_serializing_if = "Option::is_none")]
	pub tradg_cpcty: Option<String>,
	#[serde(rename = "DrctnOrSd", skip_serializing_if = "Option::is_none")]
	pub drctn_or_sd: Option<Direction4Choice>,
	#[serde(rename = "TradrLctn", skip_serializing_if = "Option::is_none")]
	pub tradr_lctn: Option<String>,
	#[serde(rename = "BookgLctn", skip_serializing_if = "Option::is_none")]
	pub bookg_lctn: Option<String>,
	#[serde(rename = "RptgXmptn", skip_serializing_if = "Option::is_none")]
	pub rptg_xmptn: Option<ReportingExemption1>,
}


// Counterparty46 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Counterparty46 {
	#[serde(rename = "IdTp", skip_serializing_if = "Option::is_none")]
	pub id_tp: Option<PartyIdentification248Choice>,
	#[serde(rename = "Ntr", skip_serializing_if = "Option::is_none")]
	pub ntr: Option<CounterpartyTradeNature15Choice>,
	#[serde(rename = "RptgOblgtn", skip_serializing_if = "Option::is_none")]
	pub rptg_oblgtn: Option<bool>,
}


// CounterpartyTradeNature15Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyTradeNature15Choice {
	#[serde(rename = "FI", skip_serializing_if = "Option::is_none")]
	pub fi: Option<FinancialInstitutionSector1>,
	#[serde(rename = "NFI", skip_serializing_if = "Option::is_none")]
	pub nfi: Option<NonFinancialInstitutionSector10>,
	#[serde(rename = "CntrlCntrPty", skip_serializing_if = "Option::is_none")]
	pub cntrl_cntr_pty: Option<String>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<String>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// DerivativesTradeMarginDataReportV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativesTradeMarginDataReportV02 {
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: TradeReportHeader4,
	#[serde(rename = "TradData")]
	pub trad_data: TradeData61Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// Direction2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Direction2 {
	#[serde(rename = "DrctnOfTheFrstLeg")]
	pub drctn_of_the_frst_leg: String,
	#[serde(rename = "DrctnOfTheScndLeg", skip_serializing_if = "Option::is_none")]
	pub drctn_of_the_scnd_leg: Option<String>,
}


// Direction4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Direction4Choice {
	#[serde(rename = "Drctn", skip_serializing_if = "Option::is_none")]
	pub drctn: Option<Direction2>,
	#[serde(rename = "CtrPtySd", skip_serializing_if = "Option::is_none")]
	pub ctr_pty_sd: Option<String>,
}


// ExternalPartyRelationshipType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPartyRelationshipType1Code {
	#[serde(rename = "ExternalPartyRelationshipType1Code")]
	pub external_party_relationship_type1_code: String,
}


// FinancialInstitutionSector1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstitutionSector1 {
	#[serde(rename = "Sctr")]
	pub sctr: Vec<FinancialPartyClassification2Choice>,
	#[serde(rename = "ClrThrshld", skip_serializing_if = "Option::is_none")]
	pub clr_thrshld: Option<bool>,
}


// FinancialPartyClassification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialPartyClassification2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification175>,
}


// FinancialPartySectorType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialPartySectorType3Code {
	#[serde(rename = "FinancialPartySectorType3Code")]
	pub financial_party_sector_type3_code: String,
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


// MarginCollateralReport5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarginCollateralReport5 {
	#[serde(rename = "CollPrtflCd")]
	pub coll_prtfl_cd: CollateralPortfolioCode6Choice,
	#[serde(rename = "CollstnCtgy")]
	pub collstn_ctgy: String,
	#[serde(rename = "TmStmp", skip_serializing_if = "Option::is_none")]
	pub tm_stmp: Option<String>,
}


// MarginPortfolio4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarginPortfolio4 {
	#[serde(rename = "InitlMrgnPrtflCd", skip_serializing_if = "Option::is_none")]
	pub initl_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
	#[serde(rename = "VartnMrgnPrtflCd", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
}


// MarginReportData9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarginReportData9 {
	#[serde(rename = "RptgTmStmp", skip_serializing_if = "Option::is_none")]
	pub rptg_tm_stmp: Option<String>,
	#[serde(rename = "CtrPtyId")]
	pub ctr_pty_id: TradeCounterpartyReport20,
	#[serde(rename = "EvtDt", skip_serializing_if = "Option::is_none")]
	pub evt_dt: Option<String>,
	#[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
	pub tx_id: Option<UniqueTransactionIdentifier2Choice>,
	#[serde(rename = "Coll")]
	pub coll: MarginCollateralReport5,
	#[serde(rename = "PstdMrgnOrColl", skip_serializing_if = "Option::is_none")]
	pub pstd_mrgn_or_coll: Option<PostedMarginOrCollateral6>,
	#[serde(rename = "RcvdMrgnOrColl", skip_serializing_if = "Option::is_none")]
	pub rcvd_mrgn_or_coll: Option<ReceivedMarginOrCollateral6>,
	#[serde(rename = "CtrPtyRatgTrggrInd", skip_serializing_if = "Option::is_none")]
	pub ctr_pty_ratg_trggr_ind: Option<bool>,
	#[serde(rename = "CtrPtyRatgThrshldInd", skip_serializing_if = "Option::is_none")]
	pub ctr_pty_ratg_thrshld_ind: Option<bool>,
	#[serde(rename = "TechAttrbts", skip_serializing_if = "Option::is_none")]
	pub tech_attrbts: Option<TechnicalAttributes6>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// Max1000Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max1000Text {
	#[serde(rename = "Max1000Text")]
	pub max1000_text: String,
}


// Max100Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max100Text {
	#[serde(rename = "Max100Text")]
	pub max100_text: String,
}


// Max105Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max105Text {
	#[serde(rename = "Max105Text")]
	pub max105_text: String,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
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


// Max4Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4Text {
	#[serde(rename = "Max4Text")]
	pub max4_text: String,
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


// Max5NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max5NumericText {
	#[serde(rename = "Max5NumericText")]
	pub max5_numeric_text: String,
}


// Max72Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max72Text {
	#[serde(rename = "Max72Text")]
	pub max72_text: String,
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


// NoReasonCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NoReasonCode {
	#[serde(rename = "NoReasonCode")]
	pub no_reason_code: String,
}


// NonFinancialInstitutionSector10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NonFinancialInstitutionSector10 {
	#[serde(rename = "Sctr")]
	pub sctr: Vec<GenericIdentification175>,
	#[serde(rename = "ClrThrshld", skip_serializing_if = "Option::is_none")]
	pub clr_thrshld: Option<bool>,
	#[serde(rename = "DrctlyLkdActvty", skip_serializing_if = "Option::is_none")]
	pub drctly_lkd_actvty: Option<bool>,
	#[serde(rename = "FdrlInstn", skip_serializing_if = "Option::is_none")]
	pub fdrl_instn: Option<bool>,
}


// NotApplicable1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotApplicable1Code {
	#[serde(rename = "NotApplicable1Code")]
	pub not_applicable1_code: String,
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
}


// OptionParty1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OptionParty1Code {
	#[serde(rename = "OptionParty1Code")]
	pub option_party1_code: String,
}


// OptionParty3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OptionParty3Code {
	#[serde(rename = "OptionParty3Code")]
	pub option_party3_code: String,
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


// Pagination1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Pagination1 {
	#[serde(rename = "PgNb")]
	pub pg_nb: String,
	#[serde(rename = "LastPgInd")]
	pub last_pg_ind: bool,
}


// PartyIdentification248Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification248Choice {
	#[serde(rename = "Lgl", skip_serializing_if = "Option::is_none")]
	pub lgl: Option<LegalPersonIdentification1>,
	#[serde(rename = "Ntrl", skip_serializing_if = "Option::is_none")]
	pub ntrl: Option<NaturalPersonIdentification3>,
}


// PortfolioCode3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioCode3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "NoPrtfl", skip_serializing_if = "Option::is_none")]
	pub no_prtfl: Option<String>,
}


// PortfolioCode5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioCode5Choice {
	#[serde(rename = "Prtfl", skip_serializing_if = "Option::is_none")]
	pub prtfl: Option<PortfolioIdentification3>,
	#[serde(rename = "NoPrtfl", skip_serializing_if = "Option::is_none")]
	pub no_prtfl: Option<String>,
}


// PortfolioIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioIdentification3 {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "PrtflTxXmptn", skip_serializing_if = "Option::is_none")]
	pub prtfl_tx_xmptn: Option<bool>,
}


// PostedMarginOrCollateral6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostedMarginOrCollateral6 {
	#[serde(rename = "InitlMrgnPstdPreHrcut", skip_serializing_if = "Option::is_none")]
	pub initl_mrgn_pstd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "InitlMrgnPstdPstHrcut", skip_serializing_if = "Option::is_none")]
	pub initl_mrgn_pstd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "VartnMrgnPstdPreHrcut", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_pstd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "VartnMrgnPstdPstHrcut", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_pstd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "XcssCollPstd", skip_serializing_if = "Option::is_none")]
	pub xcss_coll_pstd: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
}


// ReceivedMarginOrCollateral6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReceivedMarginOrCollateral6 {
	#[serde(rename = "InitlMrgnRcvdPreHrcut", skip_serializing_if = "Option::is_none")]
	pub initl_mrgn_rcvd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "InitlMrgnRcvdPstHrcut", skip_serializing_if = "Option::is_none")]
	pub initl_mrgn_rcvd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "VartnMrgnRcvdPreHrcut", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_rcvd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "VartnMrgnRcvdPstHrcut", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_rcvd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "XcssCollRcvd", skip_serializing_if = "Option::is_none")]
	pub xcss_coll_rcvd: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
}


// ReportPeriodActivity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportPeriodActivity1Code {
	#[serde(rename = "ReportPeriodActivity1Code")]
	pub report_period_activity1_code: String,
}


// ReportingExemption1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportingExemption1 {
	#[serde(rename = "Rsn")]
	pub rsn: String,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<String>,
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


// TechnicalAttributes6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TechnicalAttributes6 {
	#[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
	pub tech_rcrd_id: Option<String>,
	#[serde(rename = "RptRctTmStmp", skip_serializing_if = "Option::is_none")]
	pub rpt_rct_tm_stmp: Option<String>,
}


// TradeCounterpartyRelationship1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeCounterpartyRelationship1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// TradeCounterpartyRelationshipRecord1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeCounterpartyRelationshipRecord1 {
	#[serde(rename = "StartRltshPty")]
	pub start_rltsh_pty: String,
	#[serde(rename = "EndRltshPty")]
	pub end_rltsh_pty: String,
	#[serde(rename = "RltshTp")]
	pub rltsh_tp: TradeCounterpartyRelationship1Choice,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<String>,
}


// TradeCounterpartyReport20 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeCounterpartyReport20 {
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: Counterparty45,
	#[serde(rename = "OthrCtrPty")]
	pub othr_ctr_pty: Counterparty46,
	#[serde(rename = "Brkr", skip_serializing_if = "Option::is_none")]
	pub brkr: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "SubmitgAgt", skip_serializing_if = "Option::is_none")]
	pub submitg_agt: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "ClrMmb", skip_serializing_if = "Option::is_none")]
	pub clr_mmb: Option<PartyIdentification248Choice>,
	#[serde(rename = "Bnfcry", skip_serializing_if = "Option::is_none")]
	pub bnfcry: Option<Vec<PartyIdentification248Choice>>,
	#[serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none")]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "ExctnAgt", skip_serializing_if = "Option::is_none")]
	pub exctn_agt: Option<Vec<OrganisationIdentification15Choice>>,
	#[serde(rename = "RltshRcrd", skip_serializing_if = "Option::is_none")]
	pub rltsh_rcrd: Option<Vec<TradeCounterpartyRelationshipRecord1>>,
}


// TradeCounterpartyType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeCounterpartyType1Code {
	#[serde(rename = "TradeCounterpartyType1Code")]
	pub trade_counterparty_type1_code: String,
}


// TradeData61Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeData61Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<String>,
	#[serde(rename = "Rpt", skip_serializing_if = "Option::is_none")]
	pub rpt: Option<Vec<TradeReport34Choice>>,
}


// TradeReport34Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeReport34Choice {
	#[serde(rename = "New", skip_serializing_if = "Option::is_none")]
	pub new: Option<MarginReportData9>,
	#[serde(rename = "MrgnUpd", skip_serializing_if = "Option::is_none")]
	pub mrgn_upd: Option<MarginReportData9>,
	#[serde(rename = "Err", skip_serializing_if = "Option::is_none")]
	pub err: Option<MarginReportData9>,
	#[serde(rename = "Crrctn", skip_serializing_if = "Option::is_none")]
	pub crrctn: Option<MarginReportData9>,
}


// TradeReportHeader4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeReportHeader4 {
	#[serde(rename = "RptExctnDt", skip_serializing_if = "Option::is_none")]
	pub rpt_exctn_dt: Option<String>,
	#[serde(rename = "MsgPgntn", skip_serializing_if = "Option::is_none")]
	pub msg_pgntn: Option<Pagination1>,
	#[serde(rename = "NbRcrds")]
	pub nb_rcrds: f64,
	#[serde(rename = "CmptntAuthrty", skip_serializing_if = "Option::is_none")]
	pub cmptnt_authrty: Option<Vec<String>>,
	#[serde(rename = "NewTradRpstryIdr", skip_serializing_if = "Option::is_none")]
	pub new_trad_rpstry_idr: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "RptgPurp", skip_serializing_if = "Option::is_none")]
	pub rptg_purp: Option<Vec<String>>,
}


// TradingCapacity7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingCapacity7Code {
	#[serde(rename = "TradingCapacity7Code")]
	pub trading_capacity7_code: String,
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}


// UTIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UTIIdentifier {
	#[serde(rename = "UTIIdentifier")]
	pub uti_identifier: String,
}


// UniqueTransactionIdentifier2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UniqueTransactionIdentifier2Choice {
	#[serde(rename = "UnqTxIdr", skip_serializing_if = "Option::is_none")]
	pub unq_tx_idr: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification175>,
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "YesNoIndicator")]
	pub yes_no_indicator: bool,
}
