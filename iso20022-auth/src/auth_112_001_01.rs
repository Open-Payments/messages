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


// AssetClassDetailedSubProductType16Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType16Code {
	#[default]
	#[serde(rename = "FXCR")]
	CodeFXCR,
	#[serde(rename = "FXEM")]
	CodeFXEM,
	#[serde(rename = "FXMJ")]
	CodeFXMJ,
	#[serde(rename = "FUEL")]
	CodeFUEL,
	#[serde(rename = "FOIL")]
	CodeFOIL,
	#[serde(rename = "GOIL")]
	CodeGOIL,
	#[serde(rename = "GSLN")]
	CodeGSLN,
	#[serde(rename = "GASP")]
	CodeGASP,
	#[serde(rename = "HEAT")]
	CodeHEAT,
	#[serde(rename = "IRON")]
	CodeIRON,
	#[serde(rename = "JTFL")]
	CodeJTFL,
	#[serde(rename = "KERO")]
	CodeKERO,
	#[serde(rename = "LAMP")]
	CodeLAMP,
	#[serde(rename = "LEAD")]
	CodeLEAD,
	#[serde(rename = "LLSO")]
	CodeLLSO,
	#[serde(rename = "LNGG")]
	CodeLNGG,
	#[serde(rename = "CORN")]
	CodeCORN,
	#[serde(rename = "MARS")]
	CodeMARS,
	#[serde(rename = "MWHT")]
	CodeMWHT,
	#[serde(rename = "MOLY")]
	CodeMOLY,
	#[serde(rename = "NAPH")]
	CodeNAPH,
	#[serde(rename = "NBPG")]
	CodeNBPG,
	#[serde(rename = "NASC")]
	CodeNASC,
	#[serde(rename = "NCGG")]
	CodeNCGG,
	#[serde(rename = "NGLO")]
	CodeNGLO,
	#[serde(rename = "NICK")]
	CodeNICK,
	#[serde(rename = "OFFP")]
	CodeOFFP,
	#[serde(rename = "ALUM")]
	CodeALUM,
	#[serde(rename = "ALUA")]
	CodeALUA,
	#[serde(rename = "BAKK")]
	CodeBAKK,
	#[serde(rename = "BSLD")]
	CodeBSLD,
	#[serde(rename = "BDSL")]
	CodeBDSL,
	#[serde(rename = "BRNT")]
	CodeBRNT,
	#[serde(rename = "BRNX")]
	CodeBRNX,
	#[serde(rename = "CNDA")]
	CodeCNDA,
	#[serde(rename = "CERE")]
	CodeCERE,
	#[serde(rename = "CBLT")]
	CodeCBLT,
	#[serde(rename = "CCOA")]
	CodeCCOA,
	#[serde(rename = "COND")]
	CodeCOND,
	#[serde(rename = "CSHP")]
	CodeCSHP,
	#[serde(rename = "COPR")]
	CodeCOPR,
	#[serde(rename = "DSEL")]
	CodeDSEL,
	#[serde(rename = "DBCR")]
	CodeDBCR,
	#[serde(rename = "DUBA")]
	CodeDUBA,
	#[serde(rename = "ERUE")]
	CodeERUE,
	#[serde(rename = "ESPO")]
	CodeESPO,
	#[serde(rename = "ETHA")]
	CodeETHA,
	#[serde(rename = "EUAE")]
	CodeEUAE,
	#[serde(rename = "EUAA")]
	CodeEUAA,
	#[serde(rename = "FWHT")]
	CodeFWHT,
	#[serde(rename = "FITR")]
	CodeFITR,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "PLDM")]
	CodePLDM,
	#[serde(rename = "PKLD")]
	CodePKLD,
	#[serde(rename = "PTNM")]
	CodePTNM,
	#[serde(rename = "POTA")]
	CodePOTA,
	#[serde(rename = "RPSD")]
	CodeRPSD,
	#[serde(rename = "BRWN")]
	CodeBRWN,
	#[serde(rename = "RICE")]
	CodeRICE,
	#[serde(rename = "ROBU")]
	CodeROBU,
	#[serde(rename = "SLVR")]
	CodeSLVR,
	#[serde(rename = "SOYB")]
	CodeSOYB,
	#[serde(rename = "STEL")]
	CodeSTEL,
	#[serde(rename = "TNKR")]
	CodeTNKR,
	#[serde(rename = "TAPI")]
	CodeTAPI,
	#[serde(rename = "TINN")]
	CodeTINN,
	#[serde(rename = "TTFG")]
	CodeTTFG,
	#[serde(rename = "URAL")]
	CodeURAL,
	#[serde(rename = "WHSG")]
	CodeWHSG,
	#[serde(rename = "WTIO")]
	CodeWTIO,
	#[serde(rename = "ZINC")]
	CodeZINC,
}


// AssetClassDetailedSubProductType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<AssetClassDetailedSubProductType16Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}


// AssetHolding3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetHolding3 {
	#[serde(rename = "PstHrcutVal")]
	pub pst_hrcut_val: ActiveCurrencyAnd24Amount,
	#[serde(rename = "AsstTp")]
	pub asst_tp: AssetHolding3Choice,
	#[serde(rename = "CollRqrmnt")]
	pub coll_rqrmnt: CollateralAccountType3Code,
}


// AssetHolding3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetHolding3Choice {
	#[serde(rename = "Gold", skip_serializing_if = "Option::is_none")]
	pub gold: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Trpty", skip_serializing_if = "Option::is_none")]
	pub trpty: Option<TripartyCollateralAndAmount1>,
	#[serde(rename = "Csh", skip_serializing_if = "Option::is_none")]
	pub csh: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Scty", skip_serializing_if = "Option::is_none")]
	pub scty: Option<SecurityIdentificationAndAmount1>,
	#[serde(rename = "Grnt", skip_serializing_if = "Option::is_none")]
	pub grnt: Option<Guarantee1>,
	#[serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none")]
	pub cmmdty: Option<Commodity2>,
}


// CCPInteroperabilityReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CCPInteroperabilityReportV01 {
	#[serde(rename = "IntrprbltyCCP")]
	pub intrprblty_ccp: Vec<InteroperabilityCCP1>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// CollateralAccountType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CollateralAccountType3Code {
	#[default]
	#[serde(rename = "MGIN")]
	CodeMGIN,
	#[serde(rename = "DFLT")]
	CodeDFLT,
}


// CollateralType22Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralType22Choice {
	#[serde(rename = "GnlColl", skip_serializing_if = "Option::is_none")]
	pub gnl_coll: Option<GeneralCollateral4>,
	#[serde(rename = "SpcfcColl", skip_serializing_if = "Option::is_none")]
	pub spcfc_coll: Option<SpecificCollateral3>,
}


// Commodity2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Commodity2 {
	#[serde(rename = "MktVal")]
	pub mkt_val: ActiveCurrencyAnd24Amount,
	#[serde(rename = "CmmdtyTp")]
	pub cmmdty_tp: AssetClassDetailedSubProductType1Choice,
}


// FinancialInstrument104 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument104 {
	#[serde(rename = "Id")]
	pub id: ISINOct2015Identifier,
	#[serde(rename = "Issr")]
	pub issr: LEIIdentifier,
}


// GeneralCollateral4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GeneralCollateral4 {
	#[serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_id: Option<Vec<FinancialInstrument104>>,
	#[serde(rename = "MktVal")]
	pub mkt_val: ActiveCurrencyAnd24Amount,
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


// Guarantee1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Guarantee1 {
	#[serde(rename = "Prvdr")]
	pub prvdr: PartyIdentification118Choice,
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
}


// ISINOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "$value")]
	pub isin_oct2015_identifier: String,
}


// InteroperabilityCCP1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InteroperabilityCCP1 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification168,
	#[serde(rename = "TtlInitlMrgn")]
	pub ttl_initl_mrgn: Vec<ActiveCurrencyAndAmount>,
	#[serde(rename = "TrdsClrd", skip_serializing_if = "Option::is_none")]
	pub trds_clrd: Option<f64>,
	#[serde(rename = "GrssNtnlAmt")]
	pub grss_ntnl_amt: Vec<ActiveCurrencyAnd24Amount>,
	#[serde(rename = "AsstHldg")]
	pub asst_hldg: Vec<AssetHolding3>,
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


// PartyIdentification118Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification118Choice {
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification168>,
}


// ProductType7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ProductType7Code {
	#[default]
	#[serde(rename = "SVGN")]
	CodeSVGN,
	#[serde(rename = "EQUI")]
	CodeEQUI,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}


// SecurityIdentificationAndAmount1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentificationAndAmount1 {
	#[serde(rename = "Id")]
	pub id: ISINOct2015Identifier,
	#[serde(rename = "MktVal")]
	pub mkt_val: ActiveCurrencyAnd24Amount,
	#[serde(rename = "FinInstrmTp")]
	pub fin_instrm_tp: ProductType7Code,
}


// SpecificCollateral3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SpecificCollateral3 {
	#[serde(rename = "FinInstrmId")]
	pub fin_instrm_id: FinancialInstrument104,
	#[serde(rename = "MktVal")]
	pub mkt_val: ActiveCurrencyAnd24Amount,
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


// TripartyCollateralAndAmount1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TripartyCollateralAndAmount1 {
	#[serde(rename = "Trpty")]
	pub trpty: ActiveCurrencyAndAmount,
	#[serde(rename = "CollTp")]
	pub coll_tp: CollateralType22Choice,
}
