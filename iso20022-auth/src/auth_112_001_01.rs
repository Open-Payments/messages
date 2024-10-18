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
use crate::validationerror::*;


// ActiveCurrencyAnd24AmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyAnd24AmountSimpleType {
	#[serde(rename = "$value")]
	pub active_currency_and24_amount_simple_type: f64,
}

impl ActiveCurrencyAnd24AmountSimpleType {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.active_currency_and24_amount_simple_type < 0.000000 {
			return Err(ValidationError::new(1003, "active_currency_and24_amount_simple_type is less than the minimum value of 0.000000".to_string()));
		}
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.active_currency_and_amount_simple_type < 0.000000 {
			return Err(ValidationError::new(1003, "active_currency_and_amount_simple_type is less than the minimum value of 0.000000".to_string()));
		}
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.active_currency_code) {
			return Err(ValidationError::new(1005, "active_currency_code does not match the required pattern".to_string()));
		}
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
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

impl AssetHolding3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.pst_hrcut_val.validate() { return Err(e); }
		if let Err(e) = self.asst_tp.validate() { return Err(e); }
		if let Err(e) = self.coll_rqrmnt.validate() { return Err(e); }
		Ok(())
	}
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

impl AssetHolding3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref gold_value) = self.gold { if let Err(e) = gold_value.validate() { return Err(e); } }
		if let Some(ref trpty_value) = self.trpty { if let Err(e) = trpty_value.validate() { return Err(e); } }
		if let Some(ref csh_value) = self.csh { if let Err(e) = csh_value.validate() { return Err(e); } }
		if let Some(ref scty_value) = self.scty { if let Err(e) = scty_value.validate() { return Err(e); } }
		if let Some(ref grnt_value) = self.grnt { if let Err(e) = grnt_value.validate() { return Err(e); } }
		if let Some(ref cmmdty_value) = self.cmmdty { if let Err(e) = cmmdty_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CCPInteroperabilityReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CCPInteroperabilityReportV01 {
	#[serde(rename = "IntrprbltyCCP")]
	pub intrprblty_ccp: Vec<InteroperabilityCCP1>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CCPInteroperabilityReportV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.intrprblty_ccp { if let Err(e) = item.validate() { return Err(e); } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CollateralType22Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralType22Choice {
	#[serde(rename = "GnlColl", skip_serializing_if = "Option::is_none")]
	pub gnl_coll: Option<GeneralCollateral4>,
	#[serde(rename = "SpcfcColl", skip_serializing_if = "Option::is_none")]
	pub spcfc_coll: Option<SpecificCollateral3>,
}

impl CollateralType22Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref gnl_coll_value) = self.gnl_coll { if let Err(e) = gnl_coll_value.validate() { return Err(e); } }
		if let Some(ref spcfc_coll_value) = self.spcfc_coll { if let Err(e) = spcfc_coll_value.validate() { return Err(e); } }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.mkt_val.validate() { return Err(e); }
		if let Err(e) = self.cmmdty_tp.validate() { return Err(e); }
		Ok(())
	}
}


// FinancialInstrument104 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument104 {
	#[serde(rename = "Id")]
	pub id: ISINOct2015Identifier,
	#[serde(rename = "Issr")]
	pub issr: LEIIdentifier,
}

impl FinancialInstrument104 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Err(e) = self.issr.validate() { return Err(e); }
		Ok(())
	}
}


// GeneralCollateral4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GeneralCollateral4 {
	#[serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_id: Option<Vec<FinancialInstrument104>>,
	#[serde(rename = "MktVal")]
	pub mkt_val: ActiveCurrencyAnd24Amount,
}

impl GeneralCollateral4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref fin_instrm_id_vec) = self.fin_instrm_id { for item in fin_instrm_id_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Err(e) = self.mkt_val.validate() { return Err(e); }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
		if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
		if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Err(e) = self.issr.validate() { return Err(e); }
		if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.prvdr.validate() { return Err(e); }
		if let Err(e) = self.amt.validate() { return Err(e); }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
		if !pattern.is_match(&self.isin_oct2015_identifier) {
			return Err(ValidationError::new(1005, "isin_oct2015_identifier does not match the required pattern".to_string()));
		}
		Ok(())
	}
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

impl InteroperabilityCCP1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		for item in &self.ttl_initl_mrgn { if let Err(e) = item.validate() { return Err(e); } }
		for item in &self.grss_ntnl_amt { if let Err(e) = item.validate() { return Err(e); } }
		for item in &self.asst_hldg { if let Err(e) = item.validate() { return Err(e); } }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.lei_identifier) {
			return Err(ValidationError::new(1005, "lei_identifier does not match the required pattern".to_string()));
		}
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max140_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max140_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max140_text.chars().count() > 140 {
			return Err(ValidationError::new(1002, "max140_text exceeds the maximum length of 140".to_string()));
		}
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max256_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max256_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max256_text.chars().count() > 256 {
			return Err(ValidationError::new(1002, "max256_text exceeds the maximum length of 256".to_string()));
		}
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max350_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max350_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max350_text.chars().count() > 350 {
			return Err(ValidationError::new(1002, "max350_text exceeds the maximum length of 350".to_string()));
		}
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max35_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max35_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max35_text.chars().count() > 35 {
			return Err(ValidationError::new(1002, "max35_text exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// NonNegativeNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NonNegativeNumber {
	#[serde(rename = "$value")]
	pub non_negative_number: f64,
}

impl NonNegativeNumber {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.non_negative_number < 0.000000 {
			return Err(ValidationError::new(1003, "non_negative_number is less than the minimum value of 0.000000".to_string()));
		}
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Err(e) = self.mkt_val.validate() { return Err(e); }
		if let Err(e) = self.fin_instrm_tp.validate() { return Err(e); }
		Ok(())
	}
}


// SpecificCollateral3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SpecificCollateral3 {
	#[serde(rename = "FinInstrmId")]
	pub fin_instrm_id: FinancialInstrument104,
	#[serde(rename = "MktVal")]
	pub mkt_val: ActiveCurrencyAnd24Amount,
}

impl SpecificCollateral3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.fin_instrm_id.validate() { return Err(e); }
		if let Err(e) = self.mkt_val.validate() { return Err(e); }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref plc_and_nm_value) = self.plc_and_nm { if let Err(e) = plc_and_nm_value.validate() { return Err(e); } }
		if let Err(e) = self.envlp.validate() { return Err(e); }
		Ok(())
	}
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TripartyCollateralAndAmount1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TripartyCollateralAndAmount1 {
	#[serde(rename = "Trpty")]
	pub trpty: ActiveCurrencyAndAmount,
	#[serde(rename = "CollTp")]
	pub coll_tp: CollateralType22Choice,
}

impl TripartyCollateralAndAmount1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.trpty.validate() { return Err(e); }
		if let Err(e) = self.coll_tp.validate() { return Err(e); }
		Ok(())
	}
}
