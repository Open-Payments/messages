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


use regex::Regex;
use crate::common::*;
#[cfg(feature = "derive_serde")]
use serde::{Deserialize, Serialize};


// ActiveCurrencyAnd24Amount ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ActiveCurrencyAnd24Amount {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
	pub value: f64,
}

impl ActiveCurrencyAnd24Amount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ActiveCurrencyAndAmount ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ActiveCurrencyAndAmount {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
	pub value: f64,
}

impl ActiveCurrencyAndAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AssetClassDetailedSubProductType16Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum AssetClassDetailedSubProductType16Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FXCR") )]
	CodeFXCR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FXEM") )]
	CodeFXEM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FXMJ") )]
	CodeFXMJ,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FUEL") )]
	CodeFUEL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FOIL") )]
	CodeFOIL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GOIL") )]
	CodeGOIL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GSLN") )]
	CodeGSLN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GASP") )]
	CodeGASP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HEAT") )]
	CodeHEAT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IRON") )]
	CodeIRON,
	#[cfg_attr( feature = "derive_serde", serde(rename = "JTFL") )]
	CodeJTFL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "KERO") )]
	CodeKERO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LAMP") )]
	CodeLAMP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEAD") )]
	CodeLEAD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LLSO") )]
	CodeLLSO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LNGG") )]
	CodeLNGG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CORN") )]
	CodeCORN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MARS") )]
	CodeMARS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MWHT") )]
	CodeMWHT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MOLY") )]
	CodeMOLY,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NAPH") )]
	CodeNAPH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NBPG") )]
	CodeNBPG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NASC") )]
	CodeNASC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NCGG") )]
	CodeNCGG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NGLO") )]
	CodeNGLO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NICK") )]
	CodeNICK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OFFP") )]
	CodeOFFP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ALUM") )]
	CodeALUM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ALUA") )]
	CodeALUA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BAKK") )]
	CodeBAKK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BSLD") )]
	CodeBSLD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BDSL") )]
	CodeBDSL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BRNT") )]
	CodeBRNT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BRNX") )]
	CodeBRNX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CNDA") )]
	CodeCNDA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CERE") )]
	CodeCERE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CBLT") )]
	CodeCBLT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CCOA") )]
	CodeCCOA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "COND") )]
	CodeCOND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CSHP") )]
	CodeCSHP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "COPR") )]
	CodeCOPR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DSEL") )]
	CodeDSEL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DBCR") )]
	CodeDBCR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DUBA") )]
	CodeDUBA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ERUE") )]
	CodeERUE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ESPO") )]
	CodeESPO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ETHA") )]
	CodeETHA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EUAE") )]
	CodeEUAE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EUAA") )]
	CodeEUAA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FWHT") )]
	CodeFWHT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FITR") )]
	CodeFITR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PLDM") )]
	CodePLDM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PKLD") )]
	CodePKLD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PTNM") )]
	CodePTNM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "POTA") )]
	CodePOTA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RPSD") )]
	CodeRPSD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BRWN") )]
	CodeBRWN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RICE") )]
	CodeRICE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ROBU") )]
	CodeROBU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SLVR") )]
	CodeSLVR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SOYB") )]
	CodeSOYB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "STEL") )]
	CodeSTEL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TNKR") )]
	CodeTNKR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TAPI") )]
	CodeTAPI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TINN") )]
	CodeTINN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TTFG") )]
	CodeTTFG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "URAL") )]
	CodeURAL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WHSG") )]
	CodeWHSG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WTIO") )]
	CodeWTIO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ZINC") )]
	CodeZINC,
}

impl AssetClassDetailedSubProductType16Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AssetClassDetailedSubProductType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AssetClassDetailedSubProductType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<AssetClassDetailedSubProductType16Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification36>,
}

impl AssetClassDetailedSubProductType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// AssetHolding1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AssetHolding1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstHrcutVal") )]
	pub pst_hrcut_val: ActiveCurrencyAnd24Amount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AsstTp") )]
	pub asst_tp: AssetHolding1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollRqrmnt") )]
	pub coll_rqrmnt: CollateralAccountType3Code,
}

impl AssetHolding1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pst_hrcut_val.validate()?;
		self.asst_tp.validate()?;
		self.coll_rqrmnt.validate()?;
		Ok(())
	}
}


// AssetHolding1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AssetHolding1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Gold", skip_serializing_if = "Option::is_none") )]
	pub gold: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Trpty", skip_serializing_if = "Option::is_none") )]
	pub trpty: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Csh", skip_serializing_if = "Option::is_none") )]
	pub csh: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Scty", skip_serializing_if = "Option::is_none") )]
	pub scty: Option<SecurityIdentificationAndAmount1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Grnt", skip_serializing_if = "Option::is_none") )]
	pub grnt: Option<Guarantee1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none") )]
	pub cmmdty: Option<Commodity2>,
}

impl AssetHolding1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.gold { val.validate()? }
		if let Some(ref val) = self.trpty { val.validate()? }
		if let Some(ref val) = self.csh { val.validate()? }
		if let Some(ref val) = self.scty { val.validate()? }
		if let Some(ref val) = self.grnt { val.validate()? }
		if let Some(ref val) = self.cmmdty { val.validate()? }
		Ok(())
	}
}


// CCPCollateralReportV01 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CCPCollateralReportV01 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollAcctOwnr") )]
	pub coll_acct_ownr: Vec<CollateralAccount4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CCPCollateralReportV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.coll_acct_ownr { item.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// CollateralAccount4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CollateralAccount4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: GenericIdentification165,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AsstHldg") )]
	pub asst_hldg: Vec<AssetHolding1>,
}

impl CollateralAccount4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		for item in &self.asst_hldg { item.validate()? }
		Ok(())
	}
}


// CollateralAccountType3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum CollateralAccountType3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MGIN") )]
	CodeMGIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DFLT") )]
	CodeDFLT,
}

impl CollateralAccountType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Commodity2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Commodity2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktVal") )]
	pub mkt_val: ActiveCurrencyAnd24Amount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CmmdtyTp") )]
	pub cmmdty_tp: AssetClassDetailedSubProductType1Choice,
}

impl Commodity2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.mkt_val.validate()?;
		self.cmmdty_tp.validate()?;
		Ok(())
	}
}


// GenericIdentification165 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericIdentification165 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<SchemeIdentificationType1Code>,
}

impl GenericIdentification165 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 256 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 256".to_string()));
		}
		if let Some(ref val) = self.desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.schme_nm { val.validate()? }
		Ok(())
	}
}


// GenericIdentification168 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericIdentification168 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<String>,
}

impl GenericIdentification168 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 256 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 256".to_string()));
		}
		if let Some(ref val) = self.desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.schme_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "schme_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "schme_nm exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// GenericIdentification36 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericIdentification36 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
	pub issr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<String>,
}

impl GenericIdentification36 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if self.issr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
		}
		if self.issr.chars().count() > 35 {
			return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.schme_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "schme_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "schme_nm exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// Guarantee1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Guarantee1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prvdr") )]
	pub prvdr: PartyIdentification118Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveCurrencyAndAmount,
}

impl Guarantee1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.prvdr.validate()?;
		self.amt.validate()?;
		Ok(())
	}
}


// PartyIdentification118Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyIdentification118Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification168>,
}

impl PartyIdentification118Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// ProductType7Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ProductType7Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SVGN") )]
	CodeSVGN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EQUI") )]
	CodeEQUI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
}

impl ProductType7Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SchemeIdentificationType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum SchemeIdentificationType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MARG") )]
	CodeMARG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "COLL") )]
	CodeCOLL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "POSI") )]
	CodePOSI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLIM") )]
	CodeCLIM,
}

impl SchemeIdentificationType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SecurityIdentificationAndAmount1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecurityIdentificationAndAmount1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktVal") )]
	pub mkt_val: ActiveCurrencyAnd24Amount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmTp") )]
	pub fin_instrm_tp: ProductType7Code,
}

impl SecurityIdentificationAndAmount1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
		if !pattern.is_match(&self.id) {
			return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
		}
		self.mkt_val.validate()?;
		self.fin_instrm_tp.validate()?;
		Ok(())
	}
}


// SupplementaryData1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SupplementaryData1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none") )]
	pub plc_and_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Envlp") )]
	pub envlp: SupplementaryDataEnvelope1,
}

impl SupplementaryData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.plc_and_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "plc_and_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "plc_and_nm exceeds the maximum length of 350".to_string()));
			}
		}
		self.envlp.validate()?;
		Ok(())
	}
}


// SupplementaryDataEnvelope1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}
