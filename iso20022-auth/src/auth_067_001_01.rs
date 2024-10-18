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
use regex::Regex;


// ActiveCurrencyAnd24AmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyAnd24AmountSimpleType {
	#[serde(rename = "$value")]
	pub active_currency_and24_amount_simple_type: f64,
}

impl ActiveCurrencyAnd24AmountSimpleType {
	pub fn validate(&self) -> bool {
		if self.active_currency_and24_amount_simple_type < 0.000000 {
			return false
		}
		return true
	}
}


// ActiveCurrencyAnd24Amount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd24Amount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveCurrencyAnd24Amount {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_currency_and_amount_simple_type: f64,
}

impl ActiveCurrencyAndAmountSimpleType {
	pub fn validate(&self) -> bool {
		if self.active_currency_and_amount_simple_type < 0.000000 {
			return false
		}
		return true
	}
}


// ActiveCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveCurrencyAndAmount {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "$value")]
	pub active_currency_code: String,
}

impl ActiveCurrencyCode {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.active_currency_code) {
			return false
		}
		return true
	}
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

impl AssetClassDetailedSubProductType16Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassDetailedSubProductType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<AssetClassDetailedSubProductType16Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}

impl AssetClassDetailedSubProductType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// AssetHolding1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetHolding1 {
	#[serde(rename = "PstHrcutVal")]
	pub pst_hrcut_val: ActiveCurrencyAnd24Amount,
	#[serde(rename = "AsstTp")]
	pub asst_tp: AssetHolding1Choice,
	#[serde(rename = "CollRqrmnt")]
	pub coll_rqrmnt: CollateralAccountType3Code,
}

impl AssetHolding1 {
	pub fn validate(&self) -> bool {
		if !self.pst_hrcut_val.validate() { return false }
		if !self.asst_tp.validate() { return false }
		if !self.coll_rqrmnt.validate() { return false }
		return true
	}
}


// AssetHolding1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetHolding1Choice {
	#[serde(rename = "Gold", skip_serializing_if = "Option::is_none")]
	pub gold: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Trpty", skip_serializing_if = "Option::is_none")]
	pub trpty: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Csh", skip_serializing_if = "Option::is_none")]
	pub csh: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Scty", skip_serializing_if = "Option::is_none")]
	pub scty: Option<SecurityIdentificationAndAmount1>,
	#[serde(rename = "Grnt", skip_serializing_if = "Option::is_none")]
	pub grnt: Option<Guarantee1>,
	#[serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none")]
	pub cmmdty: Option<Commodity2>,
}

impl AssetHolding1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref gold_value) = self.gold { if !gold_value.validate() { return false; } }
		if let Some(ref trpty_value) = self.trpty { if !trpty_value.validate() { return false; } }
		if let Some(ref csh_value) = self.csh { if !csh_value.validate() { return false; } }
		if let Some(ref scty_value) = self.scty { if !scty_value.validate() { return false; } }
		if let Some(ref grnt_value) = self.grnt { if !grnt_value.validate() { return false; } }
		if let Some(ref cmmdty_value) = self.cmmdty { if !cmmdty_value.validate() { return false; } }
		return true
	}
}


// CCPCollateralReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CCPCollateralReportV01 {
	#[serde(rename = "CollAcctOwnr")]
	pub coll_acct_ownr: Vec<CollateralAccount4>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CCPCollateralReportV01 {
	pub fn validate(&self) -> bool {
		for item in &self.coll_acct_ownr { if !item.validate() { return false; } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
	}
}


// CollateralAccount4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralAccount4 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification165,
	#[serde(rename = "AsstHldg")]
	pub asst_hldg: Vec<AssetHolding1>,
}

impl CollateralAccount4 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		for item in &self.asst_hldg { if !item.validate() { return false; } }
		return true
	}
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

impl CollateralAccountType3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Commodity2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Commodity2 {
	#[serde(rename = "MktVal")]
	pub mkt_val: ActiveCurrencyAnd24Amount,
	#[serde(rename = "CmmdtyTp")]
	pub cmmdty_tp: AssetClassDetailedSubProductType1Choice,
}

impl Commodity2 {
	pub fn validate(&self) -> bool {
		if !self.mkt_val.validate() { return false }
		if !self.cmmdty_tp.validate() { return false }
		return true
	}
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

impl GenericIdentification165 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref desc_value) = self.desc { if !desc_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		return true
	}
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

impl GenericIdentification168 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref desc_value) = self.desc { if !desc_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		return true
	}
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

impl GenericIdentification36 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.issr.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		return true
	}
}


// Guarantee1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Guarantee1 {
	#[serde(rename = "Prvdr")]
	pub prvdr: PartyIdentification118Choice,
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
}

impl Guarantee1 {
	pub fn validate(&self) -> bool {
		if !self.prvdr.validate() { return false }
		if !self.amt.validate() { return false }
		return true
	}
}


// ISINOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "$value")]
	pub isin_oct2015_identifier: String,
}

impl ISINOct2015Identifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
		if !pattern.is_match(&self.isin_oct2015_identifier) {
			return false
		}
		return true
	}
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}

impl LEIIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.lei_identifier) {
			return false
		}
		return true
	}
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
}

impl Max140Text {
	pub fn validate(&self) -> bool {
		if self.max140_text.chars().count() < 1 {
			return false
		}
		if self.max140_text.chars().count() > 140 {
			return false
		}
		return true
	}
}


// Max256Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max256Text {
	#[serde(rename = "$value")]
	pub max256_text: String,
}

impl Max256Text {
	pub fn validate(&self) -> bool {
		if self.max256_text.chars().count() < 1 {
			return false
		}
		if self.max256_text.chars().count() > 256 {
			return false
		}
		return true
	}
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}

impl Max350Text {
	pub fn validate(&self) -> bool {
		if self.max350_text.chars().count() < 1 {
			return false
		}
		if self.max350_text.chars().count() > 350 {
			return false
		}
		return true
	}
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}

impl Max35Text {
	pub fn validate(&self) -> bool {
		if self.max35_text.chars().count() < 1 {
			return false
		}
		if self.max35_text.chars().count() > 35 {
			return false
		}
		return true
	}
}


// PartyIdentification118Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification118Choice {
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification168>,
}

impl PartyIdentification118Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref lei_value) = self.lei { if !lei_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
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

impl ProductType7Code {
	pub fn validate(&self) -> bool {
		return true
	}
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

impl SchemeIdentificationType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
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

impl SecurityIdentificationAndAmount1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.mkt_val.validate() { return false }
		if !self.fin_instrm_tp.validate() { return false }
		return true
	}
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
	pub plc_and_nm: Option<Max350Text>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}

impl SupplementaryData1 {
	pub fn validate(&self) -> bool {
		if let Some(ref plc_and_nm_value) = self.plc_and_nm { if !plc_and_nm_value.validate() { return false; } }
		if !self.envlp.validate() { return false }
		return true
	}
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> bool {
		return true
	}
}
