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


// AssetClassSubProductType19Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum AssetClassSubProductType19Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DLVR") )]
	CodeDLVR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NDLV") )]
	CodeNDLV,
}

impl AssetClassSubProductType19Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// BenchmarkCurveName2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum BenchmarkCurveName2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "WIBO") )]
	CodeWIBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TREA") )]
	CodeTREA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TIBO") )]
	CodeTIBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TLBO") )]
	CodeTLBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SWAP") )]
	CodeSWAP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "STBO") )]
	CodeSTBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRBO") )]
	CodePRBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PFAN") )]
	CodePFAN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NIBO") )]
	CodeNIBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MAAA") )]
	CodeMAAA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MOSP") )]
	CodeMOSP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LIBO") )]
	CodeLIBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LIBI") )]
	CodeLIBI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "JIBA") )]
	CodeJIBA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISDA") )]
	CodeISDA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GCFR") )]
	CodeGCFR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FUSW") )]
	CodeFUSW,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EUCH") )]
	CodeEUCH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EUUS") )]
	CodeEUUS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EURI") )]
	CodeEURI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EONS") )]
	CodeEONS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EONA") )]
	CodeEONA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CIBO") )]
	CodeCIBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CDOR") )]
	CodeCDOR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BUBO") )]
	CodeBUBO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BBSW") )]
	CodeBBSW,
}

impl BenchmarkCurveName2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// BenchmarkCurveName5Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BenchmarkCurveName5Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Indx", skip_serializing_if = "Option::is_none") )]
	pub indx: Option<BenchmarkCurveName2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
}

impl BenchmarkCurveName5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.indx { val.validate()? }
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 25 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 25".to_string()));
			}
		}
		Ok(())
	}
}


// BondDerivative2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BondDerivative2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
	pub issr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none") )]
	pub mtrty_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IssncDt", skip_serializing_if = "Option::is_none") )]
	pub issnc_dt: Option<String>,
}

impl BondDerivative2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.issr) {
			return Err(ValidationError::new(1005, "issr does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// BondType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum BondType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "EUSB") )]
	CodeEUSB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OEPB") )]
	CodeOEPB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CVTB") )]
	CodeCVTB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRPB") )]
	CodeCRPB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CVDB") )]
	CodeCVDB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
}

impl BondType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CommodityDerivative2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CommodityDerivative2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Frght", skip_serializing_if = "Option::is_none") )]
	pub frght: Option<CommodityDerivative5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nrgy", skip_serializing_if = "Option::is_none") )]
	pub nrgy: Option<CommodityDerivative6>,
}

impl CommodityDerivative2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.frght { val.validate()? }
		if let Some(ref val) = self.nrgy { val.validate()? }
		Ok(())
	}
}


// CommodityDerivative4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CommodityDerivative4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClssSpcfc", skip_serializing_if = "Option::is_none") )]
	pub clss_spcfc: Option<CommodityDerivative2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlCcy") )]
	pub ntnl_ccy: String,
}

impl CommodityDerivative4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.clss_spcfc { val.validate()? }
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.ntnl_ccy) {
			return Err(ValidationError::new(1005, "ntnl_ccy does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// CommodityDerivative5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CommodityDerivative5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sz") )]
	pub sz: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AvrgTmChrtr") )]
	pub avrg_tm_chrtr: String,
}

impl CommodityDerivative5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.sz.chars().count() < 1 {
			return Err(ValidationError::new(1001, "sz is shorter than the minimum length of 1".to_string()));
		}
		if self.sz.chars().count() > 25 {
			return Err(ValidationError::new(1002, "sz exceeds the maximum length of 25".to_string()));
		}
		if self.avrg_tm_chrtr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "avrg_tm_chrtr is shorter than the minimum length of 1".to_string()));
		}
		if self.avrg_tm_chrtr.chars().count() > 25 {
			return Err(ValidationError::new(1002, "avrg_tm_chrtr exceeds the maximum length of 25".to_string()));
		}
		Ok(())
	}
}


// CommodityDerivative6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CommodityDerivative6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmLctn") )]
	pub sttlm_lctn: String,
}

impl CommodityDerivative6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.sttlm_lctn.chars().count() < 1 {
			return Err(ValidationError::new(1001, "sttlm_lctn is shorter than the minimum length of 1".to_string()));
		}
		if self.sttlm_lctn.chars().count() > 25 {
			return Err(ValidationError::new(1002, "sttlm_lctn exceeds the maximum length of 25".to_string()));
		}
		Ok(())
	}
}


// ContractForDifference2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ContractForDifference2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygTp") )]
	pub undrlyg_tp: UnderlyingContractForDifferenceType3Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlCcy1", skip_serializing_if = "Option::is_none") )]
	pub ntnl_ccy1: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlCcy2", skip_serializing_if = "Option::is_none") )]
	pub ntnl_ccy2: Option<String>,
}

impl ContractForDifference2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.undrlyg_tp.validate()?;
		if let Some(ref val) = self.ntnl_ccy1 {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ntnl_ccy1 does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ntnl_ccy2 {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ntnl_ccy2 does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// CreditDefaultSwapDerivative5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CreditDefaultSwapDerivative5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygCdtDfltSwpId", skip_serializing_if = "Option::is_none") )]
	pub undrlyg_cdt_dflt_swp_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygCdtDfltSwpIndx") )]
	pub undrlyg_cdt_dflt_swp_indx: CreditDefaultSwapIndex3,
}

impl CreditDefaultSwapDerivative5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.undrlyg_cdt_dflt_swp_id {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "undrlyg_cdt_dflt_swp_id does not match the required pattern".to_string()));
			}
		}
		self.undrlyg_cdt_dflt_swp_indx.validate()?;
		Ok(())
	}
}


// CreditDefaultSwapDerivative6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CreditDefaultSwapDerivative6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygCdtDfltSwpId", skip_serializing_if = "Option::is_none") )]
	pub undrlyg_cdt_dflt_swp_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OblgtnId") )]
	pub oblgtn_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SnglNm") )]
	pub sngl_nm: CreditDefaultSwapSingleName2,
}

impl CreditDefaultSwapDerivative6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.undrlyg_cdt_dflt_swp_id {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "undrlyg_cdt_dflt_swp_id does not match the required pattern".to_string()));
			}
		}
		let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
		if !pattern.is_match(&self.oblgtn_id) {
			return Err(ValidationError::new(1005, "oblgtn_id does not match the required pattern".to_string()));
		}
		self.sngl_nm.validate()?;
		Ok(())
	}
}


// CreditDefaultSwapIndex3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CreditDefaultSwapIndex3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygIndxId", skip_serializing_if = "Option::is_none") )]
	pub undrlyg_indx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygIndxNm", skip_serializing_if = "Option::is_none") )]
	pub undrlyg_indx_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Srs", skip_serializing_if = "Option::is_none") )]
	pub srs: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Vrsn", skip_serializing_if = "Option::is_none") )]
	pub vrsn: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RollMnth", skip_serializing_if = "Option::is_none") )]
	pub roll_mnth: Option<Vec<f64>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NxtRollDt", skip_serializing_if = "Option::is_none") )]
	pub nxt_roll_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlCcy") )]
	pub ntnl_ccy: String,
}

impl CreditDefaultSwapIndex3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.undrlyg_indx_id {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "undrlyg_indx_id does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.undrlyg_indx_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "undrlyg_indx_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 25 {
				return Err(ValidationError::new(1002, "undrlyg_indx_nm exceeds the maximum length of 25".to_string()));
			}
		}
		if let Some(ref vec) = self.roll_mnth {
			for item in vec {
				if *item < 1.000000 {
					return Err(ValidationError::new(1003, "roll_mnth is less than the minimum value of 1.000000".to_string()));
				}
				if *item > 12.000000 {
					return Err(ValidationError::new(1004, "roll_mnth exceeds the maximum value of 12.000000".to_string()));
				}
			}
		}
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.ntnl_ccy) {
			return Err(ValidationError::new(1005, "ntnl_ccy does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// CreditDefaultSwapSingleName2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CreditDefaultSwapSingleName2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SvrgnIssr") )]
	pub svrgn_issr: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefPty", skip_serializing_if = "Option::is_none") )]
	pub ref_pty: Option<DerivativePartyIdentification1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlCcy") )]
	pub ntnl_ccy: String,
}

impl CreditDefaultSwapSingleName2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ref_pty { val.validate()? }
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.ntnl_ccy) {
			return Err(ValidationError::new(1005, "ntnl_ccy does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// CreditDefaultSwapsDerivative4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CreditDefaultSwapsDerivative4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SnglNmCdtDfltSwp", skip_serializing_if = "Option::is_none") )]
	pub sngl_nm_cdt_dflt_swp: Option<CreditDefaultSwapSingleName2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDfltSwpIndx", skip_serializing_if = "Option::is_none") )]
	pub cdt_dflt_swp_indx: Option<CreditDefaultSwapIndex3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SnglNmCdtDfltSwpDeriv", skip_serializing_if = "Option::is_none") )]
	pub sngl_nm_cdt_dflt_swp_deriv: Option<CreditDefaultSwapDerivative6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDfltSwpIndxDeriv", skip_serializing_if = "Option::is_none") )]
	pub cdt_dflt_swp_indx_deriv: Option<CreditDefaultSwapDerivative5>,
}

impl CreditDefaultSwapsDerivative4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.sngl_nm_cdt_dflt_swp { val.validate()? }
		if let Some(ref val) = self.cdt_dflt_swp_indx { val.validate()? }
		if let Some(ref val) = self.sngl_nm_cdt_dflt_swp_deriv { val.validate()? }
		if let Some(ref val) = self.cdt_dflt_swp_indx_deriv { val.validate()? }
		Ok(())
	}
}


// DebtInstrument5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DebtInstrument5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: BondType1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IssncDt") )]
	pub issnc_dt: String,
}

impl DebtInstrument5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tp.validate()?;
		Ok(())
	}
}


// Derivative3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Derivative3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none") )]
	pub cmmdty: Option<CommodityDerivative4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none") )]
	pub intrst_rate: Option<InterestRateDerivative5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FX", skip_serializing_if = "Option::is_none") )]
	pub fx: Option<ForeignExchangeDerivative2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Eqty", skip_serializing_if = "Option::is_none") )]
	pub eqty: Option<EquityDerivative2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctForDiff", skip_serializing_if = "Option::is_none") )]
	pub ctrct_for_diff: Option<ContractForDifference2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdt", skip_serializing_if = "Option::is_none") )]
	pub cdt: Option<CreditDefaultSwapsDerivative4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EmssnAllwnc", skip_serializing_if = "Option::is_none") )]
	pub emssn_allwnc: Option<EmissionAllowanceProductType1Code>,
}

impl Derivative3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cmmdty { val.validate()? }
		if let Some(ref val) = self.intrst_rate { val.validate()? }
		if let Some(ref val) = self.fx { val.validate()? }
		if let Some(ref val) = self.eqty { val.validate()? }
		if let Some(ref val) = self.ctrct_for_diff { val.validate()? }
		if let Some(ref val) = self.cdt { val.validate()? }
		if let Some(ref val) = self.emssn_allwnc { val.validate()? }
		Ok(())
	}
}


// DerivativePartyIdentification1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DerivativePartyIdentification1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
	pub ctry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none") )]
	pub ctry_sub_dvsn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
}

impl DerivativePartyIdentification1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ctry_sub_dvsn {
			let pattern = Regex::new("[A-Z]{2,2}\\-[0-9A-Z]{1,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry_sub_dvsn does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// EmissionAllowanceProductType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum EmissionAllowanceProductType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "EUAA") )]
	CodeEUAA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EUAE") )]
	CodeEUAE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ERUE") )]
	CodeERUE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CERE") )]
	CodeCERE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
}

impl EmissionAllowanceProductType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// EquityDerivative2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct EquityDerivative2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygTp") )]
	pub undrlyg_tp: EquityDerivative3Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Param", skip_serializing_if = "Option::is_none") )]
	pub param: Option<EquityReturnParameter1Code>,
}

impl EquityDerivative2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.undrlyg_tp.validate()?;
		if let Some(ref val) = self.param { val.validate()? }
		Ok(())
	}
}


// EquityDerivative3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct EquityDerivative3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Bskt", skip_serializing_if = "Option::is_none") )]
	pub bskt: Option<UnderlyingEquityType3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Indx", skip_serializing_if = "Option::is_none") )]
	pub indx: Option<UnderlyingEquityType4Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SnglNm", skip_serializing_if = "Option::is_none") )]
	pub sngl_nm: Option<UnderlyingEquityType5Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<UnderlyingEquityType6Code>,
}

impl EquityDerivative3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.bskt { val.validate()? }
		if let Some(ref val) = self.indx { val.validate()? }
		if let Some(ref val) = self.sngl_nm { val.validate()? }
		if let Some(ref val) = self.othr { val.validate()? }
		Ok(())
	}
}


// EquityReturnParameter1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum EquityReturnParameter1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRDV") )]
	CodePRDV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRVA") )]
	CodePRVA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRVO") )]
	CodePRVO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRBP") )]
	CodePRBP,
}

impl EquityReturnParameter1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FinancialInstrumentContractType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum FinancialInstrumentContractType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CFDS") )]
	CodeCFDS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FORW") )]
	CodeFORW,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FRAS") )]
	CodeFRAS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FUTR") )]
	CodeFUTR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OPTN") )]
	CodeOPTN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SPDB") )]
	CodeSPDB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SWAP") )]
	CodeSWAP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SWPT") )]
	CodeSWPT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FONS") )]
	CodeFONS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PSWP") )]
	CodePSWP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FFAS") )]
	CodeFFAS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FWOS") )]
	CodeFWOS,
}

impl FinancialInstrumentContractType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FinancialInstrumentReportingNonEquityTransparencyDataReportV03 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrumentReportingNonEquityTransparencyDataReportV03 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptHdr") )]
	pub rpt_hdr: SecuritiesMarketReportHeader1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NonEqtyTrnsprncyData") )]
	pub non_eqty_trnsprncy_data: Vec<TransparencyDataReport21>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl FinancialInstrumentReportingNonEquityTransparencyDataReportV03 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rpt_hdr.validate()?;
		for item in &self.non_eqty_trnsprncy_data { item.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// FloatingInterestRate8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FloatingInterestRate8 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefRate") )]
	pub ref_rate: BenchmarkCurveName5Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Term", skip_serializing_if = "Option::is_none") )]
	pub term: Option<InterestRateContractTerm2>,
}

impl FloatingInterestRate8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.ref_rate.validate()?;
		if let Some(ref val) = self.term { val.validate()? }
		Ok(())
	}
}


// ForeignExchangeDerivative2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ForeignExchangeDerivative2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctSubTp") )]
	pub ctrct_sub_tp: AssetClassSubProductType19Code,
}

impl ForeignExchangeDerivative2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.ctrct_sub_tp.validate()?;
		Ok(())
	}
}


// InflationIndex1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InflationIndex1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
	pub isin: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
}

impl InflationIndex1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.isin {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 25 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 25".to_string()));
			}
		}
		Ok(())
	}
}


// InterestRateContractTerm2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InterestRateContractTerm2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Unit") )]
	pub unit: RateBasis1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
	pub val: f64,
}

impl InterestRateContractTerm2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.unit.validate()?;
		Ok(())
	}
}


// InterestRateDerivative2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InterestRateDerivative2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SwpRltd", skip_serializing_if = "Option::is_none") )]
	pub swp_rltd: Option<SwapType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<UnderlyingInterestRateType3Code>,
}

impl InterestRateDerivative2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.swp_rltd { val.validate()? }
		if let Some(ref val) = self.othr { val.validate()? }
		Ok(())
	}
}


// InterestRateDerivative5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InterestRateDerivative5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygTp") )]
	pub undrlyg_tp: InterestRateDerivative2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygBd", skip_serializing_if = "Option::is_none") )]
	pub undrlyg_bd: Option<BondDerivative2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SwptnNtnlCcy", skip_serializing_if = "Option::is_none") )]
	pub swptn_ntnl_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygSwpMtrtyDt", skip_serializing_if = "Option::is_none") )]
	pub undrlyg_swp_mtrty_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InfltnIndx", skip_serializing_if = "Option::is_none") )]
	pub infltn_indx: Option<InflationIndex1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstRateRef") )]
	pub intrst_rate_ref: FloatingInterestRate8,
}

impl InterestRateDerivative5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.undrlyg_tp.validate()?;
		if let Some(ref val) = self.undrlyg_bd { val.validate()? }
		if let Some(ref val) = self.swptn_ntnl_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "swptn_ntnl_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.infltn_indx { val.validate()? }
		self.intrst_rate_ref.validate()?;
		Ok(())
	}
}


// NonEquityInstrumentReportingClassification1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum NonEquityInstrumentReportingClassification1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SFPS") )]
	CodeSFPS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SDRV") )]
	CodeSDRV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DERV") )]
	CodeDERV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EMAL") )]
	CodeEMAL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BOND") )]
	CodeBOND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ETCS") )]
	CodeETCS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ETNS") )]
	CodeETNS,
}

impl NonEquityInstrumentReportingClassification1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Period2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Period2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt") )]
	pub fr_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt") )]
	pub to_dt: String,
}

impl Period2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Period4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Period4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt", skip_serializing_if = "Option::is_none") )]
	pub fr_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt", skip_serializing_if = "Option::is_none") )]
	pub to_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDtToDt", skip_serializing_if = "Option::is_none") )]
	pub fr_dt_to_dt: Option<Period2>,
}

impl Period4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.fr_dt_to_dt { val.validate()? }
		Ok(())
	}
}


// RateBasis1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum RateBasis1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DAYS") )]
	CodeDAYS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
	CodeMNTH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WEEK") )]
	CodeWEEK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
	CodeYEAR,
}

impl RateBasis1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SecuritiesMarketReportHeader1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesMarketReportHeader1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgNtty") )]
	pub rptg_ntty: TradingVenueIdentification1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgPrd") )]
	pub rptg_prd: Period4Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubmissnDtTm", skip_serializing_if = "Option::is_none") )]
	pub submissn_dt_tm: Option<String>,
}

impl SecuritiesMarketReportHeader1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rptg_ntty.validate()?;
		self.rptg_prd.validate()?;
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


// SwapType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum SwapType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "OSSC") )]
	CodeOSSC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XFSC") )]
	CodeXFSC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XFMC") )]
	CodeXFMC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XXSC") )]
	CodeXXSC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XXMC") )]
	CodeXXMC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IFMC") )]
	CodeIFMC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FFSC") )]
	CodeFFSC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FFMC") )]
	CodeFFMC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IFSC") )]
	CodeIFSC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OSMC") )]
	CodeOSMC,
}

impl SwapType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TradingVenue2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TradingVenue2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "APPA") )]
	CodeAPPA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CTPS") )]
	CodeCTPS,
}

impl TradingVenue2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TradingVenueIdentification1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradingVenueIdentification1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktIdCd", skip_serializing_if = "Option::is_none") )]
	pub mkt_id_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NtlCmptntAuthrty", skip_serializing_if = "Option::is_none") )]
	pub ntl_cmptnt_authrty: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<TradingVenueIdentification2>,
}

impl TradingVenueIdentification1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mkt_id_cd {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "mkt_id_cd does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ntl_cmptnt_authrty {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ntl_cmptnt_authrty does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.othr { val.validate()? }
		Ok(())
	}
}


// TradingVenueIdentification2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradingVenueIdentification2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: TradingVenue2Code,
}

impl TradingVenueIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 50 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 50".to_string()));
		}
		self.tp.validate()?;
		Ok(())
	}
}


// TransparencyDataReport21 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TransparencyDataReport21 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none") )]
	pub tech_rcrd_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FullNm", skip_serializing_if = "Option::is_none") )]
	pub full_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgVn", skip_serializing_if = "Option::is_none") )]
	pub tradg_vn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgDt", skip_serializing_if = "Option::is_none") )]
	pub rptg_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none") )]
	pub mtrty_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmClssfctn") )]
	pub fin_instrm_clssfctn: NonEquityInstrumentReportingClassification1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygInstrmAsstClss", skip_serializing_if = "Option::is_none") )]
	pub undrlyg_instrm_asst_clss: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DerivCtrctTp", skip_serializing_if = "Option::is_none") )]
	pub deriv_ctrct_tp: Option<FinancialInstrumentContractType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Bd", skip_serializing_if = "Option::is_none") )]
	pub bd: Option<DebtInstrument5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EmssnAllwncTp", skip_serializing_if = "Option::is_none") )]
	pub emssn_allwnc_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Deriv", skip_serializing_if = "Option::is_none") )]
	pub deriv: Option<Derivative3Choice>,
}

impl TransparencyDataReport21 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tech_rcrd_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tech_rcrd_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tech_rcrd_id exceeds the maximum length of 35".to_string()));
			}
		}
		let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
		if !pattern.is_match(&self.id) {
			return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.full_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "full_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "full_nm exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.tradg_vn {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "tradg_vn does not match the required pattern".to_string()));
			}
		}
		self.fin_instrm_clssfctn.validate()?;
		if let Some(ref val) = self.undrlyg_instrm_asst_clss {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "undrlyg_instrm_asst_clss is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "undrlyg_instrm_asst_clss exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.deriv_ctrct_tp { val.validate()? }
		if let Some(ref val) = self.bd { val.validate()? }
		if let Some(ref val) = self.emssn_allwnc_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "emssn_allwnc_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "emssn_allwnc_tp exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.deriv { val.validate()? }
		Ok(())
	}
}


// UnderlyingContractForDifferenceType3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum UnderlyingContractForDifferenceType3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "BOND") )]
	CodeBOND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "COMM") )]
	CodeCOMM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CURR") )]
	CodeCURR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EMAL") )]
	CodeEMAL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EQUI") )]
	CodeEQUI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FTEQ") )]
	CodeFTEQ,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OPEQ") )]
	CodeOPEQ,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
}

impl UnderlyingContractForDifferenceType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// UnderlyingEquityType3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum UnderlyingEquityType3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "BSKT") )]
	CodeBSKT,
}

impl UnderlyingEquityType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// UnderlyingEquityType4Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum UnderlyingEquityType4Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "STIX") )]
	CodeSTIX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DIVI") )]
	CodeDIVI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VOLI") )]
	CodeVOLI,
}

impl UnderlyingEquityType4Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// UnderlyingEquityType5Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum UnderlyingEquityType5Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ETFS") )]
	CodeETFS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SHRS") )]
	CodeSHRS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DVSE") )]
	CodeDVSE,
}

impl UnderlyingEquityType5Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// UnderlyingEquityType6Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum UnderlyingEquityType6Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "BSKT") )]
	CodeBSKT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DIVI") )]
	CodeDIVI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ETFS") )]
	CodeETFS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SHRS") )]
	CodeSHRS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DVSE") )]
	CodeDVSE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "STIX") )]
	CodeSTIX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VOLI") )]
	CodeVOLI,
}

impl UnderlyingEquityType6Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// UnderlyingInterestRateType3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum UnderlyingInterestRateType3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "BOND") )]
	CodeBOND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BNDF") )]
	CodeBNDF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INTR") )]
	CodeINTR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IFUT") )]
	CodeIFUT,
}

impl UnderlyingInterestRateType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}
