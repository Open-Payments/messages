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


// ActiveCurrencyAnd24AmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd24AmountSimpleType {
	#[serde(rename = "ActiveCurrencyAnd24Amount_SimpleType")]
	pub active_currency_and24_amount_simple_type: f64,
}


// ActiveCurrencyAnd24Amount ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd24Amount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "ActiveCurrencyAndAmount_SimpleType")]
	pub active_currency_and_amount_simple_type: f64,
}


// ActiveCurrencyAndAmount ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveCurrencyCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[validate(pattern = "[A-Z]{3,3}")]
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// CCPClearedProductReportV01 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CCPClearedProductReportV01 {
	#[validate]
	#[serde(rename = "ClrdPdct")]
	pub clrd_pdct: Vec<ClearedProduct1>,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// ClearedProduct1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ClearedProduct1 {
	#[serde(rename = "TradgVn")]
	pub tradg_vn: Vec<String>,
	#[validate]
	#[serde(rename = "CCPPdctId")]
	pub ccp_pdct_id: GenericIdentification168,
	#[validate]
	#[serde(rename = "UvrslPdctId")]
	pub uvrsl_pdct_id: Option<GenericIdentification168>,
	#[validate]
	#[serde(rename = "Pdct")]
	pub pdct: Product1Choice,
	#[validate]
	#[serde(rename = "OpnIntrst")]
	pub opn_intrst: OpenInterest1,
	#[serde(rename = "TrdsClrd")]
	pub trds_clrd: Option<f64>,
}


// ContractSize1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ContractSize1 {
	#[serde(rename = "LotSz")]
	pub lot_sz: f64,
	#[validate]
	#[serde(rename = "Unit")]
	pub unit: Option<UnitOfMeasure5Choice>,
}


// DefinedAttributes1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DefinedAttributes1Choice {
	#[validate]
	#[serde(rename = "QtyDfndAttrbts")]
	pub qty_dfnd_attrbts: Option<FinancialInstrumentAttributes89>,
	#[validate]
	#[serde(rename = "ValDfndAttrbts")]
	pub val_dfnd_attrbts: Option<FinancialInstrumentAttributes90>,
}


// Derivative3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Derivative3 {
	#[validate]
	#[serde(rename = "DerivClssfctn")]
	pub deriv_clssfctn: DerivativeClassification1,
	#[validate]
	#[serde(rename = "DerivUndrlygLeg")]
	pub deriv_undrlyg_leg: Vec<DerivativeUnderlyingLeg1>,
	#[validate]
	#[serde(rename = "OptnAttrbts")]
	pub optn_attrbts: Option<Option14>,
}


// DerivativeClassification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DerivativeClassification1 {
	#[serde(rename = "AsstClss")]
	pub asst_clss: String,
	#[serde(rename = "BasePdct")]
	pub base_pdct: Option<String>,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "SubCmmdty")]
	pub sub_cmmdty: Option<String>,
	#[serde(rename = "TxTp")]
	pub tx_tp: Option<String>,
}


// DerivativeUnderlyingLeg1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DerivativeUnderlyingLeg1 {
	#[validate]
	#[serde(rename = "CtrctAttrbts")]
	pub ctrct_attrbts: FinancialInstrumentAttributes88,
	#[validate]
	#[serde(rename = "DfndAttrbts")]
	pub dfnd_attrbts: Option<DefinedAttributes1Choice>,
}


// ExoticOptionStyle1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExoticOptionStyle1Code {
	#[validate(enumerate = ["BINA", "DIGI", "NOTO", "VANI"])]
	#[serde(rename = "ExoticOptionStyle1Code")]
	pub exotic_option_style1_code: String,
}


// FinancialInstrument59 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstrument59 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "Sctr")]
	pub sctr: Option<String>,
}


// FinancialInstrumentAttributes88 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstrumentAttributes88 {
	#[validate]
	#[serde(rename = "CtrctTerm")]
	pub ctrct_term: Option<InterestRateContractTerm1>,
	#[serde(rename = "Stdstn")]
	pub stdstn: Option<Vec<String>>,
	#[serde(rename = "PmtFrqcy")]
	pub pmt_frqcy: String,
}


// FinancialInstrumentAttributes89 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstrumentAttributes89 {
	#[validate]
	#[serde(rename = "CtrctSz")]
	pub ctrct_sz: ContractSize1,
	#[serde(rename = "DlvryTp")]
	pub dlvry_tp: String,
	#[validate]
	#[serde(rename = "UndrlygId")]
	pub undrlyg_id: GenericIdentification165,
	#[serde(rename = "PricCcy")]
	pub pric_ccy: String,
}


// FinancialInstrumentAttributes90 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstrumentAttributes90 {
	#[validate]
	#[serde(rename = "Ntnl")]
	pub ntnl: Option<ActiveCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "UnitVal")]
	pub unit_val: ActiveCurrencyAndAmount,
	#[validate]
	#[serde(rename = "IndxId")]
	pub indx_id: GenericIdentification168,
	#[serde(rename = "IndxUnit")]
	pub indx_unit: String,
	#[serde(rename = "IntrstRateTerms")]
	pub intrst_rate_terms: Option<String>,
}


// Frequency11Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Frequency11Code {
	#[validate(enumerate = ["YEAR", "DAIL", "MNTH", "EXPI", "OVNG", "QURT", "MIAN", "UPFR", "WEEK", "CRED"])]
	#[serde(rename = "Frequency11Code")]
	pub frequency11_code: String,
}


// GeneralCollateral2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GeneralCollateral2 {
	#[serde(rename = "ElgblFinInstrmId")]
	pub elgbl_fin_instrm_id: Vec<String>,
}


// GenericIdentification165 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification165 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// GenericIdentification168 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification168 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// GenericIdentification36 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification36 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// ISINOct2015Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[validate(pattern = "[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}")]
	#[serde(rename = "ISINOct2015Identifier")]
	pub isin_oct2015_identifier: String,
}


// InterestComputationMethod2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InterestComputationMethod2Code {
	#[validate(enumerate = ["A001", "A002", "A003", "A004", "A005", "A006", "A007", "A008", "A009", "A010", "A011", "A012", "A013", "A014", "NARR"])]
	#[serde(rename = "InterestComputationMethod2Code")]
	pub interest_computation_method2_code: String,
}


// InterestRateContractTerm1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InterestRateContractTerm1 {
	#[serde(rename = "Unit")]
	pub unit: String,
	#[serde(rename = "Val")]
	pub val: f64,
}


// LEIIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[validate(pattern = "[A-Z0-9]{18,18}[0-9]{2,2}")]
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// MICIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MICIdentifier {
	#[validate(pattern = "[A-Z0-9]{4,4}")]
	#[serde(rename = "MICIdentifier")]
	pub mic_identifier: String,
}


// Max140Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max140Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 140)]
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max256Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max256Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 256)]
	#[serde(rename = "Max256Text")]
	pub max256_text: String,
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


// NonNegativeNumber ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NonNegativeNumber {
	#[serde(rename = "NonNegativeNumber")]
	pub non_negative_number: f64,
}


// Number ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
}


// OpenInterest1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OpenInterest1 {
	#[validate]
	#[serde(rename = "GrssNtnlAmt")]
	pub grss_ntnl_amt: ActiveCurrencyAnd24Amount,
	#[serde(rename = "NbOfLots")]
	pub nb_of_lots: Option<f64>,
}


// Option14 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Option14 {
	#[serde(rename = "XprtnStyle")]
	pub xprtn_style: Vec<String>,
	#[serde(rename = "OptnStyle")]
	pub optn_style: Option<String>,
	#[serde(rename = "OptnTp")]
	pub optn_tp: Option<String>,
	#[serde(rename = "BrrrInd")]
	pub brrr_ind: Option<bool>,
	#[validate]
	#[serde(rename = "EvtTp")]
	pub evt_tp: Option<OptionEvent2>,
}


// OptionEvent2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OptionEvent2 {
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: OptionEventType1Choice,
	#[serde(rename = "Desc")]
	pub desc: String,
}


// OptionEventType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OptionEventType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification36>,
}


// OptionEventType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OptionEventType1Code {
	#[validate(enumerate = ["CLST", "CONF", "KNIN", "KNOC", "OTHR", "TRIG"])]
	#[serde(rename = "OptionEventType1Code")]
	pub option_event_type1_code: String,
}


// OptionStyle5Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OptionStyle5Code {
	#[validate(enumerate = ["AMER", "ASIA", "BERM", "EURO"])]
	#[serde(rename = "OptionStyle5Code")]
	pub option_style5_code: String,
}


// OptionType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OptionType1Code {
	#[validate(enumerate = ["CALL", "PUTO"])]
	#[serde(rename = "OptionType1Code")]
	pub option_type1_code: String,
}


// PhysicalTransferType4Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PhysicalTransferType4Code {
	#[validate(enumerate = ["PHYS", "OPTL", "CASH"])]
	#[serde(rename = "PhysicalTransferType4Code")]
	pub physical_transfer_type4_code: String,
}


// PositiveNumber ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PositiveNumber {
	#[serde(rename = "PositiveNumber")]
	pub positive_number: f64,
}


// Product1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Product1Choice {
	#[validate]
	#[serde(rename = "Deriv")]
	pub deriv: Option<Derivative3>,
	#[validate]
	#[serde(rename = "SctiesFincgTx")]
	pub scties_fincg_tx: Option<RepurchaseAgreement3>,
	#[validate]
	#[serde(rename = "Scty")]
	pub scty: Option<FinancialInstrument59>,
}


// ProductClassification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ProductClassification1 {
	#[serde(rename = "AsstClss")]
	pub asst_clss: String,
	#[serde(rename = "BasePdct")]
	pub base_pdct: Option<String>,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "SubCmmdty")]
	pub sub_cmmdty: Option<String>,
	#[serde(rename = "TxTp")]
	pub tx_tp: Option<String>,
}


// RateBasis1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RateBasis1Code {
	#[validate(enumerate = ["DAYS", "MNTH", "WEEK", "YEAR"])]
	#[serde(rename = "RateBasis1Code")]
	pub rate_basis1_code: String,
}


// RepurchaseAgreement3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RepurchaseAgreement3 {
	#[validate]
	#[serde(rename = "PdctClssfctn")]
	pub pdct_clssfctn: ProductClassification1,
	#[validate]
	#[serde(rename = "RpAgrmtTp")]
	pub rp_agrmt_tp: RepurchaseAgreementType1Choice,
	#[serde(rename = "TrptyAgt")]
	pub trpty_agt: Option<String>,
}


// RepurchaseAgreementType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RepurchaseAgreementType1Choice {
	#[validate]
	#[serde(rename = "SpcfcColl")]
	pub spcfc_coll: Option<SpecificCollateral2>,
	#[validate]
	#[serde(rename = "GnlColl")]
	pub gnl_coll: Option<GeneralCollateral2>,
}


// SNA2008SectorIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SNA2008SectorIdentifier {
	#[serde(rename = "SNA2008SectorIdentifier")]
	pub sna2008_sector_identifier: String,
}


// SchemeIdentificationType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SchemeIdentificationType1Code {
	#[validate(enumerate = ["MARG", "COLL", "POSI", "CLIM"])]
	#[serde(rename = "SchemeIdentificationType1Code")]
	pub scheme_identification_type1_code: String,
}


// SpecificCollateral2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SpecificCollateral2 {
	#[validate]
	#[serde(rename = "FinInstrmId")]
	pub fin_instrm_id: FinancialInstrument59,
}


// Standardisation1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Standardisation1Code {
	#[validate(enumerate = ["FLEX", "NSTA", "STAN"])]
	#[serde(rename = "Standardisation1Code")]
	pub standardisation1_code: String,
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


// TrueFalseIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}


// UnitOfMeasure5Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct UnitOfMeasure5Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification36>,
}


// UnitOfMeasure8Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct UnitOfMeasure8Code {
	#[validate(enumerate = ["KILO", "KMET", "KWDC", "KWHO", "KWHC", "KMOC", "KWMC", "KWYC", "LITR", "MWDC", "MWHO", "MWHC", "MWMC", "MMOC", "MWYC", "METR", "TONE", "MILE", "MILI", "MMET", "MIBA", "MBTU", "PIEC", "PUND", "PWRD", "SHAS", "SCMT", "SQFO", "SQIN", "SQKI", "SMET", "SQMI", "SMIL", "SQYA", "THMS", "TONS", "TOCD", "OZTR", "USGA", "UCWT", "USOU", "USPI", "USQA", "YARD", "ACRE", "ALOW", "ACCY", "ARES", "BARL", "BCUF", "BDFT", "BUSL", "CELI", "CMET", "CEER", "CLRT", "CBME", "DAYS", "DGEU", "DMET", "ENVC", "ENVO", "FOOT", "GGEU", "GBGA", "GBOU", "GBPI", "GBQA", "GRAM", "HECT", "HUWG", "INCH", "IPNT", "FUTU", "USTN"])]
	#[serde(rename = "UnitOfMeasure8Code")]
	pub unit_of_measure8_code: String,
}
