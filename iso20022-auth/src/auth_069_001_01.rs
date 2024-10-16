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


// ActiveCurrencyAnd24AmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd24AmountSimpleType {
	#[serde(rename = "$value")]
	pub active_currency_and24_amount_simple_type: f64,
}


// ActiveCurrencyAnd24Amount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd24Amount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_currency_and_amount_simple_type: f64,
}


// ActiveCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "$value")]
	pub active_currency_code: String,
}


// CCPClearedProductReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CCPClearedProductReportV01 {
	#[serde(rename = "ClrdPdct")]
	pub clrd_pdct: Vec<ClearedProduct1>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// ClearedProduct1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearedProduct1 {
	#[serde(rename = "TradgVn")]
	pub tradg_vn: Vec<MICIdentifier>,
	#[serde(rename = "CCPPdctId")]
	pub ccp_pdct_id: GenericIdentification168,
	#[serde(rename = "UvrslPdctId", skip_serializing_if = "Option::is_none")]
	pub uvrsl_pdct_id: Option<GenericIdentification168>,
	#[serde(rename = "Pdct")]
	pub pdct: Product1Choice,
	#[serde(rename = "OpnIntrst")]
	pub opn_intrst: OpenInterest1,
	#[serde(rename = "TrdsClrd", skip_serializing_if = "Option::is_none")]
	pub trds_clrd: Option<f64>,
}


// ContractSize1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractSize1 {
	#[serde(rename = "LotSz")]
	pub lot_sz: PositiveNumber,
	#[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
	pub unit: Option<UnitOfMeasure5Choice>,
}


// DefinedAttributes1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DefinedAttributes1Choice {
	#[serde(rename = "QtyDfndAttrbts", skip_serializing_if = "Option::is_none")]
	pub qty_dfnd_attrbts: Option<FinancialInstrumentAttributes89>,
	#[serde(rename = "ValDfndAttrbts", skip_serializing_if = "Option::is_none")]
	pub val_dfnd_attrbts: Option<FinancialInstrumentAttributes90>,
}


// Derivative3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Derivative3 {
	#[serde(rename = "DerivClssfctn")]
	pub deriv_clssfctn: DerivativeClassification1,
	#[serde(rename = "DerivUndrlygLeg")]
	pub deriv_undrlyg_leg: Vec<DerivativeUnderlyingLeg1>,
	#[serde(rename = "OptnAttrbts", skip_serializing_if = "Option::is_none")]
	pub optn_attrbts: Option<Option14>,
}


// DerivativeClassification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativeClassification1 {
	#[serde(rename = "AsstClss")]
	pub asst_clss: Max35Text,
	#[serde(rename = "BasePdct", skip_serializing_if = "Option::is_none")]
	pub base_pdct: Option<Max35Text>,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<Max35Text>,
	#[serde(rename = "SubCmmdty", skip_serializing_if = "Option::is_none")]
	pub sub_cmmdty: Option<Max35Text>,
	#[serde(rename = "TxTp", skip_serializing_if = "Option::is_none")]
	pub tx_tp: Option<Max35Text>,
}


// DerivativeUnderlyingLeg1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativeUnderlyingLeg1 {
	#[serde(rename = "CtrctAttrbts")]
	pub ctrct_attrbts: FinancialInstrumentAttributes88,
	#[serde(rename = "DfndAttrbts", skip_serializing_if = "Option::is_none")]
	pub dfnd_attrbts: Option<DefinedAttributes1Choice>,
}


// ExoticOptionStyle1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ExoticOptionStyle1Code {
	#[default]
	#[serde(rename = "BINA")]
	CodeBINA,
	#[serde(rename = "DIGI")]
	CodeDIGI,
	#[serde(rename = "NOTO")]
	CodeNOTO,
	#[serde(rename = "VANI")]
	CodeVANI,

}


// FinancialInstrument59 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument59 {
	#[serde(rename = "Id")]
	pub id: ISINOct2015Identifier,
	#[serde(rename = "Issr")]
	pub issr: LEIIdentifier,
	#[serde(rename = "Sctr", skip_serializing_if = "Option::is_none")]
	pub sctr: Option<String>,
}


// FinancialInstrumentAttributes88 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentAttributes88 {
	#[serde(rename = "CtrctTerm", skip_serializing_if = "Option::is_none")]
	pub ctrct_term: Option<InterestRateContractTerm1>,
	#[serde(rename = "Stdstn", skip_serializing_if = "Option::is_none")]
	pub stdstn: Option<Vec<Standardisation1Code>>,
	#[serde(rename = "PmtFrqcy")]
	pub pmt_frqcy: Frequency11Code,
}


// FinancialInstrumentAttributes89 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentAttributes89 {
	#[serde(rename = "CtrctSz")]
	pub ctrct_sz: ContractSize1,
	#[serde(rename = "DlvryTp")]
	pub dlvry_tp: PhysicalTransferType4Code,
	#[serde(rename = "UndrlygId")]
	pub undrlyg_id: GenericIdentification165,
	#[serde(rename = "PricCcy")]
	pub pric_ccy: ActiveCurrencyCode,
}


// FinancialInstrumentAttributes90 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentAttributes90 {
	#[serde(rename = "Ntnl", skip_serializing_if = "Option::is_none")]
	pub ntnl: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "UnitVal")]
	pub unit_val: ActiveCurrencyAndAmount,
	#[serde(rename = "IndxId")]
	pub indx_id: GenericIdentification168,
	#[serde(rename = "IndxUnit")]
	pub indx_unit: Max35Text,
	#[serde(rename = "IntrstRateTerms", skip_serializing_if = "Option::is_none")]
	pub intrst_rate_terms: Option<InterestComputationMethod2Code>,
}


// Frequency11Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Frequency11Code {
	#[default]
	#[serde(rename = "YEAR")]
	CodeYEAR,
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "EXPI")]
	CodeEXPI,
	#[serde(rename = "OVNG")]
	CodeOVNG,
	#[serde(rename = "QURT")]
	CodeQURT,
	#[serde(rename = "MIAN")]
	CodeMIAN,
	#[serde(rename = "UPFR")]
	CodeUPFR,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "CRED")]
	CodeCRED,

}


// GeneralCollateral2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GeneralCollateral2 {
	#[serde(rename = "ElgblFinInstrmId")]
	pub elgbl_fin_instrm_id: Vec<Max35Text>,
}


// GenericIdentification165 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification165 {
	#[serde(rename = "Id")]
	pub id: Max256Text,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max140Text>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<SchemeIdentificationType1Code>,
}


// GenericIdentification168 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification168 {
	#[serde(rename = "Id")]
	pub id: Max256Text,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max140Text>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
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


// ISINOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "$value")]
	pub isin_oct2015_identifier: String,
}


// InterestComputationMethod2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InterestComputationMethod2Code {
	#[default]
	#[serde(rename = "A001")]
	CodeA001,
	#[serde(rename = "A002")]
	CodeA002,
	#[serde(rename = "A003")]
	CodeA003,
	#[serde(rename = "A004")]
	CodeA004,
	#[serde(rename = "A005")]
	CodeA005,
	#[serde(rename = "A006")]
	CodeA006,
	#[serde(rename = "A007")]
	CodeA007,
	#[serde(rename = "A008")]
	CodeA008,
	#[serde(rename = "A009")]
	CodeA009,
	#[serde(rename = "A010")]
	CodeA010,
	#[serde(rename = "A011")]
	CodeA011,
	#[serde(rename = "A012")]
	CodeA012,
	#[serde(rename = "A013")]
	CodeA013,
	#[serde(rename = "A014")]
	CodeA014,
	#[serde(rename = "NARR")]
	CodeNARR,

}


// InterestRateContractTerm1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRateContractTerm1 {
	#[serde(rename = "Unit")]
	pub unit: RateBasis1Code,
	#[serde(rename = "Val")]
	pub val: f64,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}


// MICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MICIdentifier {
	#[serde(rename = "$value")]
	pub mic_identifier: String,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
}


// Max256Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max256Text {
	#[serde(rename = "$value")]
	pub max256_text: String,
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


// NonNegativeNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NonNegativeNumber {
	#[serde(rename = "$value")]
	pub non_negative_number: f64,
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "$value")]
	pub number: f64,
}


// OpenInterest1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OpenInterest1 {
	#[serde(rename = "GrssNtnlAmt")]
	pub grss_ntnl_amt: ActiveCurrencyAnd24Amount,
	#[serde(rename = "NbOfLots", skip_serializing_if = "Option::is_none")]
	pub nb_of_lots: Option<PositiveNumber>,
}


// Option14 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Option14 {
	#[serde(rename = "XprtnStyle")]
	pub xprtn_style: Vec<OptionStyle5Code>,
	#[serde(rename = "OptnStyle", skip_serializing_if = "Option::is_none")]
	pub optn_style: Option<ExoticOptionStyle1Code>,
	#[serde(rename = "OptnTp", skip_serializing_if = "Option::is_none")]
	pub optn_tp: Option<OptionType1Code>,
	#[serde(rename = "BrrrInd", skip_serializing_if = "Option::is_none")]
	pub brrr_ind: Option<bool>,
	#[serde(rename = "EvtTp", skip_serializing_if = "Option::is_none")]
	pub evt_tp: Option<OptionEvent2>,
}


// OptionEvent2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OptionEvent2 {
	#[serde(rename = "Tp")]
	pub tp: OptionEventType1Choice,
	#[serde(rename = "Desc")]
	pub desc: Max35Text,
}


// OptionEventType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OptionEventType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<OptionEventType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}


// OptionEventType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OptionEventType1Code {
	#[default]
	#[serde(rename = "CLST")]
	CodeCLST,
	#[serde(rename = "CONF")]
	CodeCONF,
	#[serde(rename = "KNIN")]
	CodeKNIN,
	#[serde(rename = "KNOC")]
	CodeKNOC,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "TRIG")]
	CodeTRIG,

}


// OptionStyle5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OptionStyle5Code {
	#[default]
	#[serde(rename = "AMER")]
	CodeAMER,
	#[serde(rename = "ASIA")]
	CodeASIA,
	#[serde(rename = "BERM")]
	CodeBERM,
	#[serde(rename = "EURO")]
	CodeEURO,

}


// OptionType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OptionType1Code {
	#[default]
	#[serde(rename = "CALL")]
	CodeCALL,
	#[serde(rename = "PUTO")]
	CodePUTO,

}


// PhysicalTransferType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PhysicalTransferType4Code {
	#[default]
	#[serde(rename = "PHYS")]
	CodePHYS,
	#[serde(rename = "OPTL")]
	CodeOPTL,
	#[serde(rename = "CASH")]
	CodeCASH,

}


// PositiveNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositiveNumber {
	#[serde(rename = "$value")]
	pub positive_number: f64,
}


// Product1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Product1Choice {
	#[serde(rename = "Deriv", skip_serializing_if = "Option::is_none")]
	pub deriv: Option<Derivative3>,
	#[serde(rename = "SctiesFincgTx", skip_serializing_if = "Option::is_none")]
	pub scties_fincg_tx: Option<RepurchaseAgreement3>,
	#[serde(rename = "Scty", skip_serializing_if = "Option::is_none")]
	pub scty: Option<FinancialInstrument59>,
}


// ProductClassification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProductClassification1 {
	#[serde(rename = "AsstClss")]
	pub asst_clss: Max35Text,
	#[serde(rename = "BasePdct", skip_serializing_if = "Option::is_none")]
	pub base_pdct: Option<Max35Text>,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<Max35Text>,
	#[serde(rename = "SubCmmdty", skip_serializing_if = "Option::is_none")]
	pub sub_cmmdty: Option<Max35Text>,
	#[serde(rename = "TxTp", skip_serializing_if = "Option::is_none")]
	pub tx_tp: Option<Max35Text>,
}


// RateBasis1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RateBasis1Code {
	#[default]
	#[serde(rename = "DAYS")]
	CodeDAYS,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "YEAR")]
	CodeYEAR,

}


// RepurchaseAgreement3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RepurchaseAgreement3 {
	#[serde(rename = "PdctClssfctn")]
	pub pdct_clssfctn: ProductClassification1,
	#[serde(rename = "RpAgrmtTp")]
	pub rp_agrmt_tp: RepurchaseAgreementType1Choice,
	#[serde(rename = "TrptyAgt", skip_serializing_if = "Option::is_none")]
	pub trpty_agt: Option<LEIIdentifier>,
}


// RepurchaseAgreementType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RepurchaseAgreementType1Choice {
	#[serde(rename = "SpcfcColl", skip_serializing_if = "Option::is_none")]
	pub spcfc_coll: Option<SpecificCollateral2>,
	#[serde(rename = "GnlColl", skip_serializing_if = "Option::is_none")]
	pub gnl_coll: Option<GeneralCollateral2>,
}


// SNA2008SectorIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SNA2008SectorIdentifier {
	#[serde(rename = "$value")]
	pub sna2008_sector_identifier: String,
}


// SchemeIdentificationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SchemeIdentificationType1Code {
	#[default]
	#[serde(rename = "MARG")]
	CodeMARG,
	#[serde(rename = "COLL")]
	CodeCOLL,
	#[serde(rename = "POSI")]
	CodePOSI,
	#[serde(rename = "CLIM")]
	CodeCLIM,

}


// SpecificCollateral2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SpecificCollateral2 {
	#[serde(rename = "FinInstrmId")]
	pub fin_instrm_id: FinancialInstrument59,
}


// Standardisation1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Standardisation1Code {
	#[default]
	#[serde(rename = "FLEX")]
	CodeFLEX,
	#[serde(rename = "NSTA")]
	CodeNSTA,
	#[serde(rename = "STAN")]
	CodeSTAN,

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


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}


// UnitOfMeasure5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitOfMeasure5Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<UnitOfMeasure8Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}


// UnitOfMeasure8Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum UnitOfMeasure8Code {
	#[default]
	#[serde(rename = "KILO")]
	CodeKILO,
	#[serde(rename = "KMET")]
	CodeKMET,
	#[serde(rename = "KWDC")]
	CodeKWDC,
	#[serde(rename = "KWHO")]
	CodeKWHO,
	#[serde(rename = "KWHC")]
	CodeKWHC,
	#[serde(rename = "KMOC")]
	CodeKMOC,
	#[serde(rename = "KWMC")]
	CodeKWMC,
	#[serde(rename = "KWYC")]
	CodeKWYC,
	#[serde(rename = "LITR")]
	CodeLITR,
	#[serde(rename = "MWDC")]
	CodeMWDC,
	#[serde(rename = "MWHO")]
	CodeMWHO,
	#[serde(rename = "MWHC")]
	CodeMWHC,
	#[serde(rename = "MWMC")]
	CodeMWMC,
	#[serde(rename = "MMOC")]
	CodeMMOC,
	#[serde(rename = "MWYC")]
	CodeMWYC,
	#[serde(rename = "METR")]
	CodeMETR,
	#[serde(rename = "TONE")]
	CodeTONE,
	#[serde(rename = "MILE")]
	CodeMILE,
	#[serde(rename = "MILI")]
	CodeMILI,
	#[serde(rename = "MMET")]
	CodeMMET,
	#[serde(rename = "MIBA")]
	CodeMIBA,
	#[serde(rename = "MBTU")]
	CodeMBTU,
	#[serde(rename = "PIEC")]
	CodePIEC,
	#[serde(rename = "PUND")]
	CodePUND,
	#[serde(rename = "PWRD")]
	CodePWRD,
	#[serde(rename = "SHAS")]
	CodeSHAS,
	#[serde(rename = "SCMT")]
	CodeSCMT,
	#[serde(rename = "SQFO")]
	CodeSQFO,
	#[serde(rename = "SQIN")]
	CodeSQIN,
	#[serde(rename = "SQKI")]
	CodeSQKI,
	#[serde(rename = "SMET")]
	CodeSMET,
	#[serde(rename = "SQMI")]
	CodeSQMI,
	#[serde(rename = "SMIL")]
	CodeSMIL,
	#[serde(rename = "SQYA")]
	CodeSQYA,
	#[serde(rename = "THMS")]
	CodeTHMS,
	#[serde(rename = "TONS")]
	CodeTONS,
	#[serde(rename = "TOCD")]
	CodeTOCD,
	#[serde(rename = "OZTR")]
	CodeOZTR,
	#[serde(rename = "USGA")]
	CodeUSGA,
	#[serde(rename = "UCWT")]
	CodeUCWT,
	#[serde(rename = "USOU")]
	CodeUSOU,
	#[serde(rename = "USPI")]
	CodeUSPI,
	#[serde(rename = "USQA")]
	CodeUSQA,
	#[serde(rename = "YARD")]
	CodeYARD,
	#[serde(rename = "ACRE")]
	CodeACRE,
	#[serde(rename = "ALOW")]
	CodeALOW,
	#[serde(rename = "ACCY")]
	CodeACCY,
	#[serde(rename = "ARES")]
	CodeARES,
	#[serde(rename = "BARL")]
	CodeBARL,
	#[serde(rename = "BCUF")]
	CodeBCUF,
	#[serde(rename = "BDFT")]
	CodeBDFT,
	#[serde(rename = "BUSL")]
	CodeBUSL,
	#[serde(rename = "CELI")]
	CodeCELI,
	#[serde(rename = "CMET")]
	CodeCMET,
	#[serde(rename = "CEER")]
	CodeCEER,
	#[serde(rename = "CLRT")]
	CodeCLRT,
	#[serde(rename = "CBME")]
	CodeCBME,
	#[serde(rename = "DAYS")]
	CodeDAYS,
	#[serde(rename = "DGEU")]
	CodeDGEU,
	#[serde(rename = "DMET")]
	CodeDMET,
	#[serde(rename = "ENVC")]
	CodeENVC,
	#[serde(rename = "ENVO")]
	CodeENVO,
	#[serde(rename = "FOOT")]
	CodeFOOT,
	#[serde(rename = "GGEU")]
	CodeGGEU,
	#[serde(rename = "GBGA")]
	CodeGBGA,
	#[serde(rename = "GBOU")]
	CodeGBOU,
	#[serde(rename = "GBPI")]
	CodeGBPI,
	#[serde(rename = "GBQA")]
	CodeGBQA,
	#[serde(rename = "GRAM")]
	CodeGRAM,
	#[serde(rename = "HECT")]
	CodeHECT,
	#[serde(rename = "HUWG")]
	CodeHUWG,
	#[serde(rename = "INCH")]
	CodeINCH,
	#[serde(rename = "IPNT")]
	CodeIPNT,
	#[serde(rename = "FUTU")]
	CodeFUTU,
	#[serde(rename = "USTN")]
	CodeUSTN,

}
