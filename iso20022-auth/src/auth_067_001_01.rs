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

#![allow(unused_imports)]

pub mod iso20022 {
	use regex::Regex;
	use crate::common::*;
	#[cfg(feature = "derive_serde")]
	use serde::{Deserialize, Serialize};
	
	
	// ActiveCurrencyAnd24AmountSimpleType ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveCurrencyAnd24AmountSimpleType {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
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
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// ActiveCurrencyAndAmountSimpleType ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveCurrencyAndAmountSimpleType {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
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
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// ActiveCurrencyCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveCurrencyCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
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
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AssetClassDetailedSubProductType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<AssetClassDetailedSubProductType16Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification36>,
	}
	
	impl AssetClassDetailedSubProductType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetHolding1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Err(e) = self.pst_hrcut_val.validate() { return Err(e); }
			if let Err(e) = self.asst_tp.validate() { return Err(e); }
			if let Err(e) = self.coll_rqrmnt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// AssetHolding1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Some(ref gold_value) = self.gold { if let Err(e) = gold_value.validate() { return Err(e); } }
			if let Some(ref trpty_value) = self.trpty { if let Err(e) = trpty_value.validate() { return Err(e); } }
			if let Some(ref csh_value) = self.csh { if let Err(e) = csh_value.validate() { return Err(e); } }
			if let Some(ref scty_value) = self.scty { if let Err(e) = scty_value.validate() { return Err(e); } }
			if let Some(ref grnt_value) = self.grnt { if let Err(e) = grnt_value.validate() { return Err(e); } }
			if let Some(ref cmmdty_value) = self.cmmdty { if let Err(e) = cmmdty_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CCPCollateralReportV01 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CCPCollateralReportV01 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CollAcctOwnr") )]
		pub coll_acct_ownr: Vec<CollateralAccount4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl CCPCollateralReportV01 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			for item in &self.coll_acct_ownr { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// CollateralAccount4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CollateralAccount4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: GenericIdentification165,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AsstHldg") )]
		pub asst_hldg: Vec<AssetHolding1>,
	}
	
	impl CollateralAccount4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			for item in &self.asst_hldg { if let Err(e) = item.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CollateralAccountType3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Commodity2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MktVal") )]
		pub mkt_val: ActiveCurrencyAnd24Amount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CmmdtyTp") )]
		pub cmmdty_tp: AssetClassDetailedSubProductType1Choice,
	}
	
	impl Commodity2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.mkt_val.validate() { return Err(e); }
			if let Err(e) = self.cmmdty_tp.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// GenericIdentification165 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GenericIdentification165 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max256Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<SchemeIdentificationType1Code>,
	}
	
	impl GenericIdentification165 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GenericIdentification168 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GenericIdentification168 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max256Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
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
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GenericIdentification36 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
		pub issr: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
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
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Guarantee1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prvdr") )]
		pub prvdr: PartyIdentification118Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
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
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISINOct2015Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
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
	
	
	// LEIIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct LEIIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
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
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max140Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
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
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max256Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
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
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max350Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
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
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max35Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
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
	
	
	// PartyIdentification118Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PartyIdentification118Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<LEIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
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
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SecurityIdentificationAndAmount1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: ISINOct2015Identifier,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MktVal") )]
		pub mkt_val: ActiveCurrencyAnd24Amount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmTp") )]
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
	
	
	// SupplementaryData1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SupplementaryData1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none") )]
		pub plc_and_nm: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Envlp") )]
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
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SupplementaryDataEnvelope1 {
	}
	
	impl SupplementaryDataEnvelope1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
}