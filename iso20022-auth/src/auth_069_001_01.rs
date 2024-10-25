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


// CCPClearedProductReportV01 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CCPClearedProductReportV01 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrdPdct") )]
	pub clrd_pdct: Vec<ClearedProduct1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CCPClearedProductReportV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.clrd_pdct { if let Err(e) = item.validate() { return Err(e); } }
		if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// ClearedProduct1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ClearedProduct1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgVn") )]
	pub tradg_vn: Vec<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CCPPdctId") )]
	pub ccp_pdct_id: GenericIdentification168,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UvrslPdctId", skip_serializing_if = "Option::is_none") )]
	pub uvrsl_pdct_id: Option<GenericIdentification168>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pdct") )]
	pub pdct: Product1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OpnIntrst") )]
	pub opn_intrst: OpenInterest1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrdsClrd", skip_serializing_if = "Option::is_none") )]
	pub trds_clrd: Option<f64>,
}

impl ClearedProduct1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.tradg_vn {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(&item) {
				return Err(ValidationError::new(1005, "tradg_vn does not match the required pattern".to_string()));
			}
		}
		if let Err(e) = self.ccp_pdct_id.validate() { return Err(e); }
		if let Some(ref val) = self.uvrsl_pdct_id { if let Err(e) = val.validate() { return Err(e); } }
		if let Err(e) = self.pdct.validate() { return Err(e); }
		if let Err(e) = self.opn_intrst.validate() { return Err(e); }
		if let Some(ref val) = self.trds_clrd {
			if *val < 0.000000 {
				return Err(ValidationError::new(1003, "trds_clrd is less than the minimum value of 0.000000".to_string()));
			}
		}
		Ok(())
	}
}


// ContractSize1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ContractSize1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "LotSz") )]
	pub lot_sz: f64,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Unit", skip_serializing_if = "Option::is_none") )]
	pub unit: Option<UnitOfMeasure5Choice>,
}

impl ContractSize1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.lot_sz < 1.000000 {
			return Err(ValidationError::new(1003, "lot_sz is less than the minimum value of 1.000000".to_string()));
		}
		if let Some(ref val) = self.unit { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// DefinedAttributes1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DefinedAttributes1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "QtyDfndAttrbts", skip_serializing_if = "Option::is_none") )]
	pub qty_dfnd_attrbts: Option<FinancialInstrumentAttributes89>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValDfndAttrbts", skip_serializing_if = "Option::is_none") )]
	pub val_dfnd_attrbts: Option<FinancialInstrumentAttributes90>,
}

impl DefinedAttributes1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.qty_dfnd_attrbts { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.val_dfnd_attrbts { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// Derivative3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Derivative3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DerivClssfctn") )]
	pub deriv_clssfctn: DerivativeClassification1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DerivUndrlygLeg") )]
	pub deriv_undrlyg_leg: Vec<DerivativeUnderlyingLeg1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OptnAttrbts", skip_serializing_if = "Option::is_none") )]
	pub optn_attrbts: Option<Option14>,
}

impl Derivative3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.deriv_clssfctn.validate() { return Err(e); }
		for item in &self.deriv_undrlyg_leg { if let Err(e) = item.validate() { return Err(e); } }
		if let Some(ref val) = self.optn_attrbts { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// DerivativeClassification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DerivativeClassification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AsstClss") )]
	pub asst_clss: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct", skip_serializing_if = "Option::is_none") )]
	pub base_pdct: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
	pub sub_pdct: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubCmmdty", skip_serializing_if = "Option::is_none") )]
	pub sub_cmmdty: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxTp", skip_serializing_if = "Option::is_none") )]
	pub tx_tp: Option<String>,
}

impl DerivativeClassification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.asst_clss.chars().count() < 1 {
			return Err(ValidationError::new(1001, "asst_clss is shorter than the minimum length of 1".to_string()));
		}
		if self.asst_clss.chars().count() > 35 {
			return Err(ValidationError::new(1002, "asst_clss exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.base_pdct {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "base_pdct is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "base_pdct exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.sub_pdct {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sub_pdct is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "sub_pdct exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.sub_cmmdty {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sub_cmmdty is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "sub_cmmdty exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.tx_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tx_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tx_tp exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// DerivativeUnderlyingLeg1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DerivativeUnderlyingLeg1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctAttrbts") )]
	pub ctrct_attrbts: FinancialInstrumentAttributes88,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DfndAttrbts", skip_serializing_if = "Option::is_none") )]
	pub dfnd_attrbts: Option<DefinedAttributes1Choice>,
}

impl DerivativeUnderlyingLeg1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.ctrct_attrbts.validate() { return Err(e); }
		if let Some(ref val) = self.dfnd_attrbts { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// ExoticOptionStyle1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ExoticOptionStyle1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "BINA") )]
	CodeBINA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DIGI") )]
	CodeDIGI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NOTO") )]
	CodeNOTO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VANI") )]
	CodeVANI,
}

impl ExoticOptionStyle1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FinancialInstrument59 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrument59 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
	pub issr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sctr", skip_serializing_if = "Option::is_none") )]
	pub sctr: Option<String>,
}

impl FinancialInstrument59 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
		if !pattern.is_match(&self.id) {
			return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
		}
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.issr) {
			return Err(ValidationError::new(1005, "issr does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// FinancialInstrumentAttributes88 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrumentAttributes88 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctTerm", skip_serializing_if = "Option::is_none") )]
	pub ctrct_term: Option<InterestRateContractTerm1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Stdstn", skip_serializing_if = "Option::is_none") )]
	pub stdstn: Option<Vec<Standardisation1Code>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtFrqcy") )]
	pub pmt_frqcy: Frequency11Code,
}

impl FinancialInstrumentAttributes88 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ctrct_term { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.stdstn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Err(e) = self.pmt_frqcy.validate() { return Err(e); }
		Ok(())
	}
}


// FinancialInstrumentAttributes89 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrumentAttributes89 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctSz") )]
	pub ctrct_sz: ContractSize1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DlvryTp") )]
	pub dlvry_tp: PhysicalTransferType4Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygId") )]
	pub undrlyg_id: GenericIdentification165,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricCcy") )]
	pub pric_ccy: String,
}

impl FinancialInstrumentAttributes89 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.ctrct_sz.validate() { return Err(e); }
		if let Err(e) = self.dlvry_tp.validate() { return Err(e); }
		if let Err(e) = self.undrlyg_id.validate() { return Err(e); }
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.pric_ccy) {
			return Err(ValidationError::new(1005, "pric_ccy does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// FinancialInstrumentAttributes90 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrumentAttributes90 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ntnl", skip_serializing_if = "Option::is_none") )]
	pub ntnl: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitVal") )]
	pub unit_val: ActiveCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IndxId") )]
	pub indx_id: GenericIdentification168,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IndxUnit") )]
	pub indx_unit: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstRateTerms", skip_serializing_if = "Option::is_none") )]
	pub intrst_rate_terms: Option<InterestComputationMethod2Code>,
}

impl FinancialInstrumentAttributes90 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ntnl { if let Err(e) = val.validate() { return Err(e); } }
		if let Err(e) = self.unit_val.validate() { return Err(e); }
		if let Err(e) = self.indx_id.validate() { return Err(e); }
		if self.indx_unit.chars().count() < 1 {
			return Err(ValidationError::new(1001, "indx_unit is shorter than the minimum length of 1".to_string()));
		}
		if self.indx_unit.chars().count() > 35 {
			return Err(ValidationError::new(1002, "indx_unit exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.intrst_rate_terms { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// Frequency11Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum Frequency11Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
	CodeYEAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DAIL") )]
	CodeDAIL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
	CodeMNTH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EXPI") )]
	CodeEXPI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OVNG") )]
	CodeOVNG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QURT") )]
	CodeQURT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MIAN") )]
	CodeMIAN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UPFR") )]
	CodeUPFR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WEEK") )]
	CodeWEEK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRED") )]
	CodeCRED,
}

impl Frequency11Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// GeneralCollateral2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GeneralCollateral2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ElgblFinInstrmId") )]
	pub elgbl_fin_instrm_id: Vec<String>,
}

impl GeneralCollateral2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.elgbl_fin_instrm_id {
			if item.chars().count() < 1 {
				return Err(ValidationError::new(1001, "elgbl_fin_instrm_id is shorter than the minimum length of 1".to_string()));
			}
			if item.chars().count() > 35 {
				return Err(ValidationError::new(1002, "elgbl_fin_instrm_id exceeds the maximum length of 35".to_string()));
			}
		}
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
		if let Some(ref val) = self.schme_nm { if let Err(e) = val.validate() { return Err(e); } }
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


// InterestComputationMethod2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum InterestComputationMethod2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "A001") )]
	CodeA001,
	#[cfg_attr( feature = "derive_serde", serde(rename = "A002") )]
	CodeA002,
	#[cfg_attr( feature = "derive_serde", serde(rename = "A003") )]
	CodeA003,
	#[cfg_attr( feature = "derive_serde", serde(rename = "A004") )]
	CodeA004,
	#[cfg_attr( feature = "derive_serde", serde(rename = "A005") )]
	CodeA005,
	#[cfg_attr( feature = "derive_serde", serde(rename = "A006") )]
	CodeA006,
	#[cfg_attr( feature = "derive_serde", serde(rename = "A007") )]
	CodeA007,
	#[cfg_attr( feature = "derive_serde", serde(rename = "A008") )]
	CodeA008,
	#[cfg_attr( feature = "derive_serde", serde(rename = "A009") )]
	CodeA009,
	#[cfg_attr( feature = "derive_serde", serde(rename = "A010") )]
	CodeA010,
	#[cfg_attr( feature = "derive_serde", serde(rename = "A011") )]
	CodeA011,
	#[cfg_attr( feature = "derive_serde", serde(rename = "A012") )]
	CodeA012,
	#[cfg_attr( feature = "derive_serde", serde(rename = "A013") )]
	CodeA013,
	#[cfg_attr( feature = "derive_serde", serde(rename = "A014") )]
	CodeA014,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NARR") )]
	CodeNARR,
}

impl InterestComputationMethod2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InterestRateContractTerm1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InterestRateContractTerm1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Unit") )]
	pub unit: RateBasis1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
	pub val: f64,
}

impl InterestRateContractTerm1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.unit.validate() { return Err(e); }
		Ok(())
	}
}


// OpenInterest1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OpenInterest1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrssNtnlAmt") )]
	pub grss_ntnl_amt: ActiveCurrencyAnd24Amount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfLots", skip_serializing_if = "Option::is_none") )]
	pub nb_of_lots: Option<f64>,
}

impl OpenInterest1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.grss_ntnl_amt.validate() { return Err(e); }
		if let Some(ref val) = self.nb_of_lots {
			if *val < 1.000000 {
				return Err(ValidationError::new(1003, "nb_of_lots is less than the minimum value of 1.000000".to_string()));
			}
		}
		Ok(())
	}
}


// Option14 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Option14 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "XprtnStyle") )]
	pub xprtn_style: Vec<OptionStyle5Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OptnStyle", skip_serializing_if = "Option::is_none") )]
	pub optn_style: Option<ExoticOptionStyle1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OptnTp", skip_serializing_if = "Option::is_none") )]
	pub optn_tp: Option<OptionType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BrrrInd", skip_serializing_if = "Option::is_none") )]
	pub brrr_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EvtTp", skip_serializing_if = "Option::is_none") )]
	pub evt_tp: Option<OptionEvent2>,
}

impl Option14 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.xprtn_style { if let Err(e) = item.validate() { return Err(e); } }
		if let Some(ref val) = self.optn_style { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.optn_tp { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.evt_tp { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// OptionEvent2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OptionEvent2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: OptionEventType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc") )]
	pub desc: String,
}

impl OptionEvent2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.tp.validate() { return Err(e); }
		if self.desc.chars().count() < 1 {
			return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
		}
		if self.desc.chars().count() > 35 {
			return Err(ValidationError::new(1002, "desc exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// OptionEventType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OptionEventType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<OptionEventType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification36>,
}

impl OptionEventType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// OptionEventType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum OptionEventType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLST") )]
	CodeCLST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CONF") )]
	CodeCONF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "KNIN") )]
	CodeKNIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "KNOC") )]
	CodeKNOC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRIG") )]
	CodeTRIG,
}

impl OptionEventType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OptionStyle5Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum OptionStyle5Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "AMER") )]
	CodeAMER,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ASIA") )]
	CodeASIA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BERM") )]
	CodeBERM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EURO") )]
	CodeEURO,
}

impl OptionStyle5Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OptionType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum OptionType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CALL") )]
	CodeCALL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUTO") )]
	CodePUTO,
}

impl OptionType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PhysicalTransferType4Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum PhysicalTransferType4Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "PHYS") )]
	CodePHYS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OPTL") )]
	CodeOPTL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CASH") )]
	CodeCASH,
}

impl PhysicalTransferType4Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Product1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Product1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Deriv", skip_serializing_if = "Option::is_none") )]
	pub deriv: Option<Derivative3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesFincgTx", skip_serializing_if = "Option::is_none") )]
	pub scties_fincg_tx: Option<RepurchaseAgreement3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Scty", skip_serializing_if = "Option::is_none") )]
	pub scty: Option<FinancialInstrument59>,
}

impl Product1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.deriv { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.scties_fincg_tx { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.scty { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// ProductClassification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ProductClassification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AsstClss") )]
	pub asst_clss: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct", skip_serializing_if = "Option::is_none") )]
	pub base_pdct: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
	pub sub_pdct: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubCmmdty", skip_serializing_if = "Option::is_none") )]
	pub sub_cmmdty: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxTp", skip_serializing_if = "Option::is_none") )]
	pub tx_tp: Option<String>,
}

impl ProductClassification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.asst_clss.chars().count() < 1 {
			return Err(ValidationError::new(1001, "asst_clss is shorter than the minimum length of 1".to_string()));
		}
		if self.asst_clss.chars().count() > 35 {
			return Err(ValidationError::new(1002, "asst_clss exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.base_pdct {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "base_pdct is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "base_pdct exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.sub_pdct {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sub_pdct is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "sub_pdct exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.sub_cmmdty {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sub_cmmdty is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "sub_cmmdty exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.tx_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tx_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tx_tp exceeds the maximum length of 35".to_string()));
			}
		}
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


// RepurchaseAgreement3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RepurchaseAgreement3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PdctClssfctn") )]
	pub pdct_clssfctn: ProductClassification1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RpAgrmtTp") )]
	pub rp_agrmt_tp: RepurchaseAgreementType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrptyAgt", skip_serializing_if = "Option::is_none") )]
	pub trpty_agt: Option<String>,
}

impl RepurchaseAgreement3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.pdct_clssfctn.validate() { return Err(e); }
		if let Err(e) = self.rp_agrmt_tp.validate() { return Err(e); }
		if let Some(ref val) = self.trpty_agt {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "trpty_agt does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// RepurchaseAgreementType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RepurchaseAgreementType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SpcfcColl", skip_serializing_if = "Option::is_none") )]
	pub spcfc_coll: Option<SpecificCollateral2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GnlColl", skip_serializing_if = "Option::is_none") )]
	pub gnl_coll: Option<GeneralCollateral2>,
}

impl RepurchaseAgreementType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.spcfc_coll { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.gnl_coll { if let Err(e) = val.validate() { return Err(e); } }
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


// SpecificCollateral2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SpecificCollateral2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmId") )]
	pub fin_instrm_id: FinancialInstrument59,
}

impl SpecificCollateral2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.fin_instrm_id.validate() { return Err(e); }
		Ok(())
	}
}


// Standardisation1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum Standardisation1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FLEX") )]
	CodeFLEX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NSTA") )]
	CodeNSTA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "STAN") )]
	CodeSTAN,
}

impl Standardisation1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
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
		if let Err(e) = self.envlp.validate() { return Err(e); }
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


// UnitOfMeasure5Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UnitOfMeasure5Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<UnitOfMeasure8Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification36>,
}

impl UnitOfMeasure5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// UnitOfMeasure8Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum UnitOfMeasure8Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "KILO") )]
	CodeKILO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "KMET") )]
	CodeKMET,
	#[cfg_attr( feature = "derive_serde", serde(rename = "KWDC") )]
	CodeKWDC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "KWHO") )]
	CodeKWHO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "KWHC") )]
	CodeKWHC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "KMOC") )]
	CodeKMOC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "KWMC") )]
	CodeKWMC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "KWYC") )]
	CodeKWYC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LITR") )]
	CodeLITR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MWDC") )]
	CodeMWDC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MWHO") )]
	CodeMWHO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MWHC") )]
	CodeMWHC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MWMC") )]
	CodeMWMC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MMOC") )]
	CodeMMOC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MWYC") )]
	CodeMWYC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "METR") )]
	CodeMETR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TONE") )]
	CodeTONE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MILE") )]
	CodeMILE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MILI") )]
	CodeMILI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MMET") )]
	CodeMMET,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MIBA") )]
	CodeMIBA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MBTU") )]
	CodeMBTU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PIEC") )]
	CodePIEC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUND") )]
	CodePUND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PWRD") )]
	CodePWRD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SHAS") )]
	CodeSHAS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SCMT") )]
	CodeSCMT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SQFO") )]
	CodeSQFO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SQIN") )]
	CodeSQIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SQKI") )]
	CodeSQKI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SMET") )]
	CodeSMET,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SQMI") )]
	CodeSQMI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SMIL") )]
	CodeSMIL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SQYA") )]
	CodeSQYA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "THMS") )]
	CodeTHMS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TONS") )]
	CodeTONS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TOCD") )]
	CodeTOCD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OZTR") )]
	CodeOZTR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USGA") )]
	CodeUSGA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UCWT") )]
	CodeUCWT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USOU") )]
	CodeUSOU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USPI") )]
	CodeUSPI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USQA") )]
	CodeUSQA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YARD") )]
	CodeYARD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACRE") )]
	CodeACRE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ALOW") )]
	CodeALOW,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACCY") )]
	CodeACCY,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ARES") )]
	CodeARES,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BARL") )]
	CodeBARL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BCUF") )]
	CodeBCUF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BDFT") )]
	CodeBDFT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BUSL") )]
	CodeBUSL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CELI") )]
	CodeCELI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CMET") )]
	CodeCMET,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CEER") )]
	CodeCEER,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLRT") )]
	CodeCLRT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CBME") )]
	CodeCBME,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DAYS") )]
	CodeDAYS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DGEU") )]
	CodeDGEU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DMET") )]
	CodeDMET,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ENVC") )]
	CodeENVC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ENVO") )]
	CodeENVO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FOOT") )]
	CodeFOOT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GGEU") )]
	CodeGGEU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GBGA") )]
	CodeGBGA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GBOU") )]
	CodeGBOU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GBPI") )]
	CodeGBPI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GBQA") )]
	CodeGBQA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GRAM") )]
	CodeGRAM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HECT") )]
	CodeHECT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HUWG") )]
	CodeHUWG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INCH") )]
	CodeINCH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IPNT") )]
	CodeIPNT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FUTU") )]
	CodeFUTU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USTN") )]
	CodeUSTN,
}

impl UnitOfMeasure8Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}
