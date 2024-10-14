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
	#[serde(rename = "$value")]
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


// ActiveOrHistoricCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_and_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_code: String,
}


// AmountAndDirection107 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection107 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAnd20DecimalAmount,
	#[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
	pub sgn: Option<bool>,
}


// AmountAndDirection53 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection53 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
	pub sgn: Option<bool>,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
}


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BaseOneRate {
	#[serde(rename = "$value")]
	pub base_one_rate: f64,
}


// CFIOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[serde(rename = "$value")]
	pub cfi_oct2015_identifier: String,
}


// CollateralData33 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralData33 {
	#[serde(rename = "NetXpsrCollstnInd", skip_serializing_if = "Option::is_none")]
	pub net_xpsr_collstn_ind: Option<bool>,
	#[serde(rename = "CmpntTp", skip_serializing_if = "Option::is_none")]
	pub cmpnt_tp: Option<CollateralType6Code>,
	#[serde(rename = "CshCollCcy", skip_serializing_if = "Option::is_none")]
	pub csh_coll_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "PricCcy", skip_serializing_if = "Option::is_none")]
	pub pric_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "Qlty", skip_serializing_if = "Option::is_none")]
	pub qlty: Option<CollateralQualityType1Code>,
	#[serde(rename = "Mtrty", skip_serializing_if = "Option::is_none")]
	pub mtrty: Option<ContractTerm6Choice>,
	#[serde(rename = "IssrJursdctn", skip_serializing_if = "Option::is_none")]
	pub issr_jursdctn: Option<IssuerJurisdiction1Choice>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<SecuritiesLendingType3Choice>,
	#[serde(rename = "TradRpstry", skip_serializing_if = "Option::is_none")]
	pub trad_rpstry: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "RcncltnFlg", skip_serializing_if = "Option::is_none")]
	pub rcncltn_flg: Option<ReconciliationFlag2>,
	#[serde(rename = "RinvstdCsh", skip_serializing_if = "Option::is_none")]
	pub rinvstd_csh: Option<ReinvestedCashTypeAndAmount2>,
}


// CollateralQualityType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CollateralQualityType1Code {
	#[serde(rename = "INVG")]
	CodeINVG,
	#[serde(rename = "NIVG")]
	CodeNIVG,
	#[serde(rename = "NOTR")]
	CodeNOTR,
	#[serde(rename = "NOAP")]
	CodeNOAP,

	#[default]
	UNKOWN
}


// CollateralRole1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CollateralRole1Code {
	#[serde(rename = "GIVE")]
	CodeGIVE,
	#[serde(rename = "TAKE")]
	CodeTAKE,

	#[default]
	UNKOWN
}


// CollateralType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CollateralType6Code {
	#[serde(rename = "GBBK")]
	CodeGBBK,
	#[serde(rename = "BOND")]
	CodeBOND,
	#[serde(rename = "CASH")]
	CodeCASH,
	#[serde(rename = "COMM")]
	CodeCOMM,
	#[serde(rename = "INSU")]
	CodeINSU,
	#[serde(rename = "LCRE")]
	CodeLCRE,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "PHYS")]
	CodePHYS,
	#[serde(rename = "SECU")]
	CodeSECU,
	#[serde(rename = "STCF")]
	CodeSTCF,

	#[default]
	UNKOWN
}


// ContractTerm6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractTerm6Choice {
	#[serde(rename = "Opn", skip_serializing_if = "Option::is_none")]
	pub opn: Option<bool>,
	#[serde(rename = "Fxd", skip_serializing_if = "Option::is_none")]
	pub fxd: Option<TimeToMaturity2Choice>,
}


// CounterpartyData86 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyData86 {
	#[serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none")]
	pub rptg_ctr_pty: Option<CounterpartyIdentification10>,
	#[serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none")]
	pub othr_ctr_pty: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "TrptyAgt", skip_serializing_if = "Option::is_none")]
	pub trpty_agt: Option<bool>,
	#[serde(rename = "AgtLndr", skip_serializing_if = "Option::is_none")]
	pub agt_lndr: Option<bool>,
}


// CounterpartyIdentification10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyIdentification10 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "Sd", skip_serializing_if = "Option::is_none")]
	pub sd: Option<CollateralRole1Code>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "$value")]
	pub decimal_number: f64,
}


// ExposureMetrics4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExposureMetrics4 {
	#[serde(rename = "PrncplAmt", skip_serializing_if = "Option::is_none")]
	pub prncpl_amt: Option<PrincipalAmount3>,
	#[serde(rename = "LnVal", skip_serializing_if = "Option::is_none")]
	pub ln_val: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "MktVal", skip_serializing_if = "Option::is_none")]
	pub mkt_val: Option<AmountAndDirection53>,
	#[serde(rename = "OutsdngMrgnLnAmt", skip_serializing_if = "Option::is_none")]
	pub outsdng_mrgn_ln_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "ShrtMktValAmt", skip_serializing_if = "Option::is_none")]
	pub shrt_mkt_val_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "MrgnLn", skip_serializing_if = "Option::is_none")]
	pub mrgn_ln: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "CshCollAmt", skip_serializing_if = "Option::is_none")]
	pub csh_coll_amt: Option<AmountAndDirection53>,
	#[serde(rename = "CollMktVal", skip_serializing_if = "Option::is_none")]
	pub coll_mkt_val: Option<AmountAndDirection53>,
}


// ExposureMetrics5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExposureMetrics5 {
	#[serde(rename = "CshCollAmt", skip_serializing_if = "Option::is_none")]
	pub csh_coll_amt: Option<AmountAndDirection53>,
	#[serde(rename = "CollMktVal", skip_serializing_if = "Option::is_none")]
	pub coll_mkt_val: Option<AmountAndDirection53>,
}


// ExposureMetrics6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExposureMetrics6 {
	#[serde(rename = "PstdMrgnOrColl", skip_serializing_if = "Option::is_none")]
	pub pstd_mrgn_or_coll: Option<PostedMarginOrCollateral4>,
}


// ExposureType10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ExposureType10Code {
	#[serde(rename = "SBSC")]
	CodeSBSC,
	#[serde(rename = "MGLD")]
	CodeMGLD,
	#[serde(rename = "SLEB")]
	CodeSLEB,
	#[serde(rename = "REPO")]
	CodeREPO,

	#[default]
	UNKOWN
}


// ExternalAgreementType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAgreementType1Code {
	#[serde(rename = "$value")]
	pub external_agreement_type1_code: String,
}


// ExternalRatesAndTenors1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalRatesAndTenors1Code {
	#[serde(rename = "$value")]
	pub external_rates_and_tenors1_code: String,
}


// ExternalSecuritiesLendingType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalSecuritiesLendingType1Code {
	#[serde(rename = "$value")]
	pub external_securities_lending_type1_code: String,
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


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}


// IssuerJurisdiction1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IssuerJurisdiction1Choice {
	#[serde(rename = "CtryCd", skip_serializing_if = "Option::is_none")]
	pub ctry_cd: Option<CountryCode>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Max35Text>,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}


// LoanData134 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LoanData134 {
	#[serde(rename = "CtrctTp", skip_serializing_if = "Option::is_none")]
	pub ctrct_tp: Option<ExposureType10Code>,
	#[serde(rename = "Clrd", skip_serializing_if = "Option::is_none")]
	pub clrd: Option<bool>,
	#[serde(rename = "PrtflCd", skip_serializing_if = "Option::is_none")]
	pub prtfl_cd: Option<Max52Text>,
	#[serde(rename = "TradgVn", skip_serializing_if = "Option::is_none")]
	pub tradg_vn: Option<TradingVenueType1Choice>,
	#[serde(rename = "MstrAgrmtTp", skip_serializing_if = "Option::is_none")]
	pub mstr_agrmt_tp: Option<ExternalAgreementType1Code>,
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "GnlColl", skip_serializing_if = "Option::is_none")]
	pub gnl_coll: Option<SpecialCollateral1Code>,
	#[serde(rename = "Term", skip_serializing_if = "Option::is_none")]
	pub term: Option<ContractTerm6Choice>,
	#[serde(rename = "Rates", skip_serializing_if = "Option::is_none")]
	pub rates: Option<Rates1Choice>,
	#[serde(rename = "PrncplAmtCcy", skip_serializing_if = "Option::is_none")]
	pub prncpl_amt_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "PricCcy", skip_serializing_if = "Option::is_none")]
	pub pric_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "Scty", skip_serializing_if = "Option::is_none")]
	pub scty: Option<Security49>,
	#[serde(rename = "OutsdngMrgnLnCcy", skip_serializing_if = "Option::is_none")]
	pub outsdng_mrgn_ln_ccy: Option<ActiveOrHistoricCurrencyCode>,
}


// LongFraction19DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LongFraction19DecimalNumber {
	#[serde(rename = "$value")]
	pub long_fraction19_decimal_number: f64,
}


// MaturityTerm2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MaturityTerm2 {
	#[serde(rename = "Unit")]
	pub unit: RateBasis1Code,
	#[serde(rename = "Val")]
	pub val: f64,
}


// Max105Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max105Text {
	#[serde(rename = "$value")]
	pub max105_text: String,
}


// Max15NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max15NumericText {
	#[serde(rename = "$value")]
	pub max15_numeric_text: String,
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


// Max3Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max3Number {
	#[serde(rename = "$value")]
	pub max3_number: f64,
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


// Max72Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max72Text {
	#[serde(rename = "$value")]
	pub max72_text: String,
}


// NamedPosition3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NamedPosition3 {
	#[serde(rename = "RefDt")]
	pub ref_dt: String,
	#[serde(rename = "GnlInf", skip_serializing_if = "Option::is_none")]
	pub gnl_inf: Option<Vec<PositionSet16>>,
	#[serde(rename = "Ln", skip_serializing_if = "Option::is_none")]
	pub ln: Option<Vec<PositionSet17>>,
	#[serde(rename = "Coll", skip_serializing_if = "Option::is_none")]
	pub coll: Option<Vec<PositionSet18>>,
	#[serde(rename = "Mrgn", skip_serializing_if = "Option::is_none")]
	pub mrgn: Option<Vec<PositionSet20>>,
	#[serde(rename = "Reuse", skip_serializing_if = "Option::is_none")]
	pub reuse: Option<Vec<PositionSet19>>,
}


// NoReasonCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NoReasonCode {
	#[serde(rename = "NORE")]
	CodeNORE,

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


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "$value")]
	pub percentage_rate: f64,
}


// PlusOrMinusIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "$value")]
	pub plus_or_minus_indicator: bool,
}


// PositionSet16 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSet16 {
	#[serde(rename = "Dmnsns")]
	pub dmnsns: PositionSetDimensions14,
	#[serde(rename = "Mtrcs")]
	pub mtrcs: PositionSetMetrics7,
}


// PositionSet17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSet17 {
	#[serde(rename = "Dmnsns")]
	pub dmnsns: PositionSetDimensions14,
	#[serde(rename = "Mtrcs")]
	pub mtrcs: PositionSetMetrics13,
}


// PositionSet18 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSet18 {
	#[serde(rename = "Dmnsns")]
	pub dmnsns: PositionSetDimensions14,
	#[serde(rename = "Mtrcs")]
	pub mtrcs: PositionSetMetrics12,
}


// PositionSet19 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSet19 {
	#[serde(rename = "Dmnsns")]
	pub dmnsns: PositionSetDimensions12,
	#[serde(rename = "Mtrcs")]
	pub mtrcs: PositionSetMetrics11,
}


// PositionSet20 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSet20 {
	#[serde(rename = "Dmnsns")]
	pub dmnsns: PositionSetDimensions15,
	#[serde(rename = "Mtrcs")]
	pub mtrcs: PositionSetMetrics10,
}


// PositionSetDimensions12 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetDimensions12 {
	#[serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none")]
	pub rptg_ctr_pty: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "CollData", skip_serializing_if = "Option::is_none")]
	pub coll_data: Option<CollateralData33>,
	#[serde(rename = "OtlrsIncl", skip_serializing_if = "Option::is_none")]
	pub otlrs_incl: Option<bool>,
}


// PositionSetDimensions14 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetDimensions14 {
	#[serde(rename = "CtrPtyData", skip_serializing_if = "Option::is_none")]
	pub ctr_pty_data: Option<CounterpartyData86>,
	#[serde(rename = "LnData", skip_serializing_if = "Option::is_none")]
	pub ln_data: Option<LoanData134>,
	#[serde(rename = "CollData", skip_serializing_if = "Option::is_none")]
	pub coll_data: Option<CollateralData33>,
	#[serde(rename = "OtlrsIncl", skip_serializing_if = "Option::is_none")]
	pub otlrs_incl: Option<bool>,
}


// PositionSetDimensions15 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetDimensions15 {
	#[serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none")]
	pub rptg_ctr_pty: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none")]
	pub othr_ctr_pty: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "CollPrtflId", skip_serializing_if = "Option::is_none")]
	pub coll_prtfl_id: Option<Max52Text>,
	#[serde(rename = "OtlrsIncl", skip_serializing_if = "Option::is_none")]
	pub otlrs_incl: Option<bool>,
}


// PositionSetMetrics10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetMetrics10 {
	#[serde(rename = "VolMtrcs", skip_serializing_if = "Option::is_none")]
	pub vol_mtrcs: Option<ExposureMetrics6>,
}


// PositionSetMetrics11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetMetrics11 {
	#[serde(rename = "VolMtrcs", skip_serializing_if = "Option::is_none")]
	pub vol_mtrcs: Option<VolumeMetrics4>,
	#[serde(rename = "CshRinvstmtRate", skip_serializing_if = "Option::is_none")]
	pub csh_rinvstmt_rate: Option<f64>,
}


// PositionSetMetrics12 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetMetrics12 {
	#[serde(rename = "VolMtrcs", skip_serializing_if = "Option::is_none")]
	pub vol_mtrcs: Option<VolumeMetrics6>,
	#[serde(rename = "HrcutOrMrgn", skip_serializing_if = "Option::is_none")]
	pub hrcut_or_mrgn: Option<f64>,
	#[serde(rename = "QtyOrNmnlAmt", skip_serializing_if = "Option::is_none")]
	pub qty_or_nmnl_amt: Option<QuantityNominalValue2Choice>,
}


// PositionSetMetrics13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetMetrics13 {
	#[serde(rename = "VolMtrcs")]
	pub vol_mtrcs: VolumeMetrics5,
	#[serde(rename = "PricMtrcs", skip_serializing_if = "Option::is_none")]
	pub pric_mtrcs: Option<PriceMetrics3>,
}


// PositionSetMetrics7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetMetrics7 {
	#[serde(rename = "VolMtrcs")]
	pub vol_mtrcs: VolumeMetrics5,
}


// PositionSetReport3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetReport3Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[serde(rename = "Rpt", skip_serializing_if = "Option::is_none")]
	pub rpt: Option<NamedPosition3>,
}


// PostedMarginOrCollateral4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostedMarginOrCollateral4 {
	#[serde(rename = "InitlMrgnPstd", skip_serializing_if = "Option::is_none")]
	pub initl_mrgn_pstd: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "VartnMrgnPstd", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_pstd: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "XcssCollPstd", skip_serializing_if = "Option::is_none")]
	pub xcss_coll_pstd: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// PriceMetrics3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceMetrics3 {
	#[serde(rename = "Rates", skip_serializing_if = "Option::is_none")]
	pub rates: Option<Rates3>,
	#[serde(rename = "LndgFee", skip_serializing_if = "Option::is_none")]
	pub lndg_fee: Option<f64>,
}


// PriceStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PriceStatus1Code {
	#[serde(rename = "PNDG")]
	CodePNDG,
	#[serde(rename = "NOAP")]
	CodeNOAP,

	#[default]
	UNKOWN
}


// PrincipalAmount3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PrincipalAmount3 {
	#[serde(rename = "ValDtAmt", skip_serializing_if = "Option::is_none")]
	pub val_dt_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "MtrtyDtAmt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// QuantityNominalValue2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct QuantityNominalValue2Choice {
	#[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
	pub qty: Option<f64>,
	#[serde(rename = "NmnlVal", skip_serializing_if = "Option::is_none")]
	pub nmnl_val: Option<AmountAndDirection53>,
}


// RateBasis1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RateBasis1Code {
	#[serde(rename = "DAYS")]
	CodeDAYS,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "YEAR")]
	CodeYEAR,

	#[default]
	UNKOWN
}


// Rates1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Rates1Choice {
	#[serde(rename = "Fxd", skip_serializing_if = "Option::is_none")]
	pub fxd: Option<NoReasonCode>,
	#[serde(rename = "Fltg", skip_serializing_if = "Option::is_none")]
	pub fltg: Option<ExternalRatesAndTenors1Code>,
}


// Rates3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Rates3 {
	#[serde(rename = "Fxd", skip_serializing_if = "Option::is_none")]
	pub fxd: Option<f64>,
	#[serde(rename = "Fltg", skip_serializing_if = "Option::is_none")]
	pub fltg: Option<f64>,
	#[serde(rename = "BuySellBck", skip_serializing_if = "Option::is_none")]
	pub buy_sell_bck: Option<SecuritiesTransactionPrice18Choice>,
}


// ReconciliationFlag2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReconciliationFlag2 {
	#[serde(rename = "RptTp", skip_serializing_if = "Option::is_none")]
	pub rpt_tp: Option<TradeRepositoryReportingType1Code>,
	#[serde(rename = "BothCtrPtiesRptg", skip_serializing_if = "Option::is_none")]
	pub both_ctr_pties_rptg: Option<bool>,
	#[serde(rename = "PairdSts", skip_serializing_if = "Option::is_none")]
	pub paird_sts: Option<bool>,
	#[serde(rename = "LnRcncltnSts", skip_serializing_if = "Option::is_none")]
	pub ln_rcncltn_sts: Option<bool>,
	#[serde(rename = "CollRcncltnSts", skip_serializing_if = "Option::is_none")]
	pub coll_rcncltn_sts: Option<bool>,
	#[serde(rename = "ModSts", skip_serializing_if = "Option::is_none")]
	pub mod_sts: Option<bool>,
}


// ReinvestedCashTypeAndAmount2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReinvestedCashTypeAndAmount2 {
	#[serde(rename = "Tp")]
	pub tp: ReinvestmentType1Code,
	#[serde(rename = "RinvstdCshCcy")]
	pub rinvstd_csh_ccy: ActiveOrHistoricCurrencyCode,
}


// ReinvestmentType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ReinvestmentType1Code {
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "OCMP")]
	CodeOCMP,
	#[serde(rename = "MMFT")]
	CodeMMFT,
	#[serde(rename = "REPM")]
	CodeREPM,
	#[serde(rename = "SDPU")]
	CodeSDPU,

	#[default]
	UNKOWN
}


// ReportPeriodActivity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ReportPeriodActivity1Code {
	#[serde(rename = "NOTX")]
	CodeNOTX,

	#[default]
	UNKOWN
}


// ReuseValue1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReuseValue1Choice {
	#[serde(rename = "Actl", skip_serializing_if = "Option::is_none")]
	pub actl: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Estmtd", skip_serializing_if = "Option::is_none")]
	pub estmtd: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// SecuritiesFinancingReportingPositionSetReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesFinancingReportingPositionSetReportV01 {
	#[serde(rename = "AggtdPoss")]
	pub aggtd_poss: PositionSetReport3Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// SecuritiesLendingType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesLendingType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalSecuritiesLendingType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// SecuritiesTransactionPrice18Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice18Choice {
	#[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
	pub mntry_val: Option<AmountAndDirection107>,
	#[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
	pub pctg: Option<f64>,
	#[serde(rename = "Dcml", skip_serializing_if = "Option::is_none")]
	pub dcml: Option<f64>,
	#[serde(rename = "BsisPts", skip_serializing_if = "Option::is_none")]
	pub bsis_pts: Option<f64>,
}


// SecuritiesTransactionPrice19Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice19Choice {
	#[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
	pub mntry_val: Option<AmountAndDirection107>,
	#[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
	pub unit: Option<f64>,
	#[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
	pub pctg: Option<f64>,
	#[serde(rename = "Yld", skip_serializing_if = "Option::is_none")]
	pub yld: Option<f64>,
	#[serde(rename = "Dcml", skip_serializing_if = "Option::is_none")]
	pub dcml: Option<f64>,
	#[serde(rename = "PdgPric", skip_serializing_if = "Option::is_none")]
	pub pdg_pric: Option<PriceStatus1Code>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<SecuritiesTransactionPrice5>,
}


// SecuritiesTransactionPrice5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice5 {
	#[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
	pub val: Option<f64>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<Max35Text>,
}


// Security49 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Security49 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ISINOct2015Identifier>,
	#[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
	pub clssfctn_tp: Option<CFIOct2015Identifier>,
	#[serde(rename = "QtyOrNmnlVal", skip_serializing_if = "Option::is_none")]
	pub qty_or_nmnl_val: Option<QuantityNominalValue2Choice>,
	#[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
	pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
	#[serde(rename = "MktVal", skip_serializing_if = "Option::is_none")]
	pub mkt_val: Option<AmountAndDirection53>,
	#[serde(rename = "Qlty", skip_serializing_if = "Option::is_none")]
	pub qlty: Option<CollateralQualityType1Code>,
	#[serde(rename = "Mtrty", skip_serializing_if = "Option::is_none")]
	pub mtrty: Option<String>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<SecurityIssuer4>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<Vec<SecuritiesLendingType3Choice>>,
	#[serde(rename = "ExclsvArrgmnt", skip_serializing_if = "Option::is_none")]
	pub exclsv_arrgmnt: Option<bool>,
}


// SecurityIssuer4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIssuer4 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "JursdctnCtry")]
	pub jursdctn_ctry: CountryCode,
}


// SpecialCollateral1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SpecialCollateral1Code {
	#[serde(rename = "GENE")]
	CodeGENE,
	#[serde(rename = "SPEC")]
	CodeSPEC,

	#[default]
	UNKOWN
}


// SpecialPurpose2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SpecialPurpose2Code {
	#[serde(rename = "BLNK")]
	CodeBLNK,
	#[serde(rename = "NTAV")]
	CodeNTAV,

	#[default]
	UNKOWN
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


// TimeToMaturity2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeToMaturity2Choice {
	#[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
	pub prd: Option<TimeToMaturityPeriod2>,
	#[serde(rename = "Spcl", skip_serializing_if = "Option::is_none")]
	pub spcl: Option<SpecialPurpose2Code>,
}


// TimeToMaturityPeriod2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeToMaturityPeriod2 {
	#[serde(rename = "Start", skip_serializing_if = "Option::is_none")]
	pub start: Option<MaturityTerm2>,
	#[serde(rename = "End", skip_serializing_if = "Option::is_none")]
	pub end: Option<MaturityTerm2>,
}


// TradeMarket2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TradeMarket2Code {
	#[serde(rename = "DMST")]
	CodeDMST,
	#[serde(rename = "FRGN")]
	CodeFRGN,

	#[default]
	UNKOWN
}


// TradeRepositoryReportingType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TradeRepositoryReportingType1Code {
	#[serde(rename = "SWOS")]
	CodeSWOS,
	#[serde(rename = "TWOS")]
	CodeTWOS,

	#[default]
	UNKOWN
}


// TradingVenueType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingVenueType1Choice {
	#[serde(rename = "OnVn", skip_serializing_if = "Option::is_none")]
	pub on_vn: Option<TradeMarket2Code>,
	#[serde(rename = "OffVn", skip_serializing_if = "Option::is_none")]
	pub off_vn: Option<NoReasonCode>,
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}


// VolumeMetrics4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct VolumeMetrics4 {
	#[serde(rename = "ReuseVal", skip_serializing_if = "Option::is_none")]
	pub reuse_val: Option<ReuseValue1Choice>,
	#[serde(rename = "RinvstdCshAmt", skip_serializing_if = "Option::is_none")]
	pub rinvstd_csh_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// VolumeMetrics5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct VolumeMetrics5 {
	#[serde(rename = "NbOfTxs", skip_serializing_if = "Option::is_none")]
	pub nb_of_txs: Option<Max15NumericText>,
	#[serde(rename = "Xpsr", skip_serializing_if = "Option::is_none")]
	pub xpsr: Option<ExposureMetrics4>,
}


// VolumeMetrics6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct VolumeMetrics6 {
	#[serde(rename = "Postv", skip_serializing_if = "Option::is_none")]
	pub postv: Option<ExposureMetrics5>,
	#[serde(rename = "Neg", skip_serializing_if = "Option::is_none")]
	pub neg: Option<ExposureMetrics5>,
}
