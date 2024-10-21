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

pub mod iso20022 {
	use regex::Regex;
	use crate::common::*;
	#[cfg(feature = "derive_serde")]
	use serde::{Deserialize, Serialize};
	
	
	// ActiveCurrencyCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ActiveOrHistoricCurrencyCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveOrHistoricCurrencyCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_or_historic_currency_code: String,
	}
	
	impl ActiveOrHistoricCurrencyCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(&self.active_or_historic_currency_code) {
				return Err(ValidationError::new(1005, "active_or_historic_currency_code does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
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
		pub nm: Option<Max25Text>,
	}
	
	impl BenchmarkCurveName5Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref indx_value) = self.indx { if let Err(e) = indx_value.validate() { return Err(e); } }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
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
		pub issr: LEIIdentifier,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none") )]
		pub mtrty_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IssncDt", skip_serializing_if = "Option::is_none") )]
		pub issnc_dt: Option<String>,
	}
	
	impl BondDerivative2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.issr.validate() { return Err(e); }
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
			if let Some(ref frght_value) = self.frght { if let Err(e) = frght_value.validate() { return Err(e); } }
			if let Some(ref nrgy_value) = self.nrgy { if let Err(e) = nrgy_value.validate() { return Err(e); } }
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
		pub ntnl_ccy: ActiveOrHistoricCurrencyCode,
	}
	
	impl CommodityDerivative4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref clss_spcfc_value) = self.clss_spcfc { if let Err(e) = clss_spcfc_value.validate() { return Err(e); } }
			if let Err(e) = self.ntnl_ccy.validate() { return Err(e); }
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
		pub sz: Max25Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AvrgTmChrtr") )]
		pub avrg_tm_chrtr: Max25Text,
	}
	
	impl CommodityDerivative5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.sz.validate() { return Err(e); }
			if let Err(e) = self.avrg_tm_chrtr.validate() { return Err(e); }
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
		pub sttlm_lctn: Max25Text,
	}
	
	impl CommodityDerivative6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.sttlm_lctn.validate() { return Err(e); }
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
		pub ntnl_ccy1: Option<ActiveOrHistoricCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlCcy2", skip_serializing_if = "Option::is_none") )]
		pub ntnl_ccy2: Option<ActiveOrHistoricCurrencyCode>,
	}
	
	impl ContractForDifference2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.undrlyg_tp.validate() { return Err(e); }
			if let Some(ref ntnl_ccy1_value) = self.ntnl_ccy1 { if let Err(e) = ntnl_ccy1_value.validate() { return Err(e); } }
			if let Some(ref ntnl_ccy2_value) = self.ntnl_ccy2 { if let Err(e) = ntnl_ccy2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CountryCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct CountryCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub country_code: String,
	}
	
	impl CountryCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(&self.country_code) {
				return Err(ValidationError::new(1005, "country_code does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// CountrySubDivisionCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct CountrySubDivisionCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub country_sub_division_code: String,
	}
	
	impl CountrySubDivisionCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{2,2}\\-[0-9A-Z]{1,3}").unwrap();
			if !pattern.is_match(&self.country_sub_division_code) {
				return Err(ValidationError::new(1005, "country_sub_division_code does not match the required pattern".to_string()));
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
		pub undrlyg_cdt_dflt_swp_id: Option<ISINOct2015Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygCdtDfltSwpIndx") )]
		pub undrlyg_cdt_dflt_swp_indx: CreditDefaultSwapIndex3,
	}
	
	impl CreditDefaultSwapDerivative5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref undrlyg_cdt_dflt_swp_id_value) = self.undrlyg_cdt_dflt_swp_id { if let Err(e) = undrlyg_cdt_dflt_swp_id_value.validate() { return Err(e); } }
			if let Err(e) = self.undrlyg_cdt_dflt_swp_indx.validate() { return Err(e); }
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
		pub undrlyg_cdt_dflt_swp_id: Option<ISINOct2015Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OblgtnId") )]
		pub oblgtn_id: ISINOct2015Identifier,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SnglNm") )]
		pub sngl_nm: CreditDefaultSwapSingleName2,
	}
	
	impl CreditDefaultSwapDerivative6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref undrlyg_cdt_dflt_swp_id_value) = self.undrlyg_cdt_dflt_swp_id { if let Err(e) = undrlyg_cdt_dflt_swp_id_value.validate() { return Err(e); } }
			if let Err(e) = self.oblgtn_id.validate() { return Err(e); }
			if let Err(e) = self.sngl_nm.validate() { return Err(e); }
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
		pub undrlyg_indx_id: Option<ISINOct2015Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygIndxNm", skip_serializing_if = "Option::is_none") )]
		pub undrlyg_indx_nm: Option<Max25Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Srs", skip_serializing_if = "Option::is_none") )]
		pub srs: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Vrsn", skip_serializing_if = "Option::is_none") )]
		pub vrsn: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RollMnth", skip_serializing_if = "Option::is_none") )]
		pub roll_mnth: Option<Vec<RestrictedMonthExact2Number>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NxtRollDt", skip_serializing_if = "Option::is_none") )]
		pub nxt_roll_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlCcy") )]
		pub ntnl_ccy: ActiveOrHistoricCurrencyCode,
	}
	
	impl CreditDefaultSwapIndex3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref undrlyg_indx_id_value) = self.undrlyg_indx_id { if let Err(e) = undrlyg_indx_id_value.validate() { return Err(e); } }
			if let Some(ref undrlyg_indx_nm_value) = self.undrlyg_indx_nm { if let Err(e) = undrlyg_indx_nm_value.validate() { return Err(e); } }
			if let Some(ref roll_mnth_vec) = self.roll_mnth { for item in roll_mnth_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Err(e) = self.ntnl_ccy.validate() { return Err(e); }
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
		pub ntnl_ccy: ActiveOrHistoricCurrencyCode,
	}
	
	impl CreditDefaultSwapSingleName2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ref_pty_value) = self.ref_pty { if let Err(e) = ref_pty_value.validate() { return Err(e); } }
			if let Err(e) = self.ntnl_ccy.validate() { return Err(e); }
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
			if let Some(ref sngl_nm_cdt_dflt_swp_value) = self.sngl_nm_cdt_dflt_swp { if let Err(e) = sngl_nm_cdt_dflt_swp_value.validate() { return Err(e); } }
			if let Some(ref cdt_dflt_swp_indx_value) = self.cdt_dflt_swp_indx { if let Err(e) = cdt_dflt_swp_indx_value.validate() { return Err(e); } }
			if let Some(ref sngl_nm_cdt_dflt_swp_deriv_value) = self.sngl_nm_cdt_dflt_swp_deriv { if let Err(e) = sngl_nm_cdt_dflt_swp_deriv_value.validate() { return Err(e); } }
			if let Some(ref cdt_dflt_swp_indx_deriv_value) = self.cdt_dflt_swp_indx_deriv { if let Err(e) = cdt_dflt_swp_indx_deriv_value.validate() { return Err(e); } }
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
			if let Err(e) = self.tp.validate() { return Err(e); }
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
			if let Some(ref cmmdty_value) = self.cmmdty { if let Err(e) = cmmdty_value.validate() { return Err(e); } }
			if let Some(ref intrst_rate_value) = self.intrst_rate { if let Err(e) = intrst_rate_value.validate() { return Err(e); } }
			if let Some(ref fx_value) = self.fx { if let Err(e) = fx_value.validate() { return Err(e); } }
			if let Some(ref eqty_value) = self.eqty { if let Err(e) = eqty_value.validate() { return Err(e); } }
			if let Some(ref ctrct_for_diff_value) = self.ctrct_for_diff { if let Err(e) = ctrct_for_diff_value.validate() { return Err(e); } }
			if let Some(ref cdt_value) = self.cdt { if let Err(e) = cdt_value.validate() { return Err(e); } }
			if let Some(ref emssn_allwnc_value) = self.emssn_allwnc { if let Err(e) = emssn_allwnc_value.validate() { return Err(e); } }
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
		pub ctry: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none") )]
		pub ctry_sub_dvsn: Option<CountrySubDivisionCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<LEIIdentifier>,
	}
	
	impl DerivativePartyIdentification1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ctry_value) = self.ctry { if let Err(e) = ctry_value.validate() { return Err(e); } }
			if let Some(ref ctry_sub_dvsn_value) = self.ctry_sub_dvsn { if let Err(e) = ctry_sub_dvsn_value.validate() { return Err(e); } }
			if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
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
			if let Err(e) = self.undrlyg_tp.validate() { return Err(e); }
			if let Some(ref param_value) = self.param { if let Err(e) = param_value.validate() { return Err(e); } }
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
			if let Some(ref bskt_value) = self.bskt { if let Err(e) = bskt_value.validate() { return Err(e); } }
			if let Some(ref indx_value) = self.indx { if let Err(e) = indx_value.validate() { return Err(e); } }
			if let Some(ref sngl_nm_value) = self.sngl_nm { if let Err(e) = sngl_nm_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
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
	
	
	// ExternalEmissionAllowanceSubProductType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalEmissionAllowanceSubProductType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_emission_allowance_sub_product_type1_code: String,
	}
	
	impl ExternalEmissionAllowanceSubProductType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_emission_allowance_sub_product_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_emission_allowance_sub_product_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_emission_allowance_sub_product_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_emission_allowance_sub_product_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalProductType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalProductType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_product_type1_code: String,
	}
	
	impl ExternalProductType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_product_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_product_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_product_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_product_type1_code exceeds the maximum length of 4".to_string()));
			}
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
			if let Err(e) = self.rpt_hdr.validate() { return Err(e); }
			for item in &self.non_eqty_trnsprncy_data { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
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
			if let Err(e) = self.ref_rate.validate() { return Err(e); }
			if let Some(ref term_value) = self.term { if let Err(e) = term_value.validate() { return Err(e); } }
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
			if let Err(e) = self.ctrct_sub_tp.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// ISINOct2015Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ISODate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISODate {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iso_date: String,
	}
	
	impl ISODate {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ISODateTime ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISODateTime {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iso_date_time: String,
	}
	
	impl ISODateTime {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
		pub isin: Option<ISINOct2015Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max25Text>,
	}
	
	impl InflationIndex1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref isin_value) = self.isin { if let Err(e) = isin_value.validate() { return Err(e); } }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
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
			if let Err(e) = self.unit.validate() { return Err(e); }
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
			if let Some(ref swp_rltd_value) = self.swp_rltd { if let Err(e) = swp_rltd_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
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
		pub swptn_ntnl_ccy: Option<ActiveCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygSwpMtrtyDt", skip_serializing_if = "Option::is_none") )]
		pub undrlyg_swp_mtrty_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InfltnIndx", skip_serializing_if = "Option::is_none") )]
		pub infltn_indx: Option<InflationIndex1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstRateRef") )]
		pub intrst_rate_ref: FloatingInterestRate8,
	}
	
	impl InterestRateDerivative5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.undrlyg_tp.validate() { return Err(e); }
			if let Some(ref undrlyg_bd_value) = self.undrlyg_bd { if let Err(e) = undrlyg_bd_value.validate() { return Err(e); } }
			if let Some(ref swptn_ntnl_ccy_value) = self.swptn_ntnl_ccy { if let Err(e) = swptn_ntnl_ccy_value.validate() { return Err(e); } }
			if let Some(ref infltn_indx_value) = self.infltn_indx { if let Err(e) = infltn_indx_value.validate() { return Err(e); } }
			if let Err(e) = self.intrst_rate_ref.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// LEIIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// MICIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct MICIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub mic_identifier: String,
	}
	
	impl MICIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(&self.mic_identifier) {
				return Err(ValidationError::new(1005, "mic_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max25Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max25Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max25_text: String,
	}
	
	impl Max25Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max25_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max25_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max25_text.chars().count() > 25 {
				return Err(ValidationError::new(1002, "max25_text exceeds the maximum length of 25".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max350Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max3Number ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max3Number {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max3_number: f64,
	}
	
	impl Max3Number {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Max50Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max50Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max50_text: String,
	}
	
	impl Max50Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max50_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max50_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max50_text.chars().count() > 50 {
				return Err(ValidationError::new(1002, "max50_text exceeds the maximum length of 50".to_string()));
			}
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
	
	
	// Number ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Number {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub number: f64,
	}
	
	impl Number {
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
			if let Some(ref fr_dt_to_dt_value) = self.fr_dt_to_dt { if let Err(e) = fr_dt_to_dt_value.validate() { return Err(e); } }
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
	
	
	// RestrictedMonthExact2Number ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct RestrictedMonthExact2Number {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub restricted_month_exact2_number: f64,
	}
	
	impl RestrictedMonthExact2Number {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.restricted_month_exact2_number < 1.000000 {
				return Err(ValidationError::new(1003, "restricted_month_exact2_number is less than the minimum value of 1.000000".to_string()));
			}
			if self.restricted_month_exact2_number > 12.000000 {
				return Err(ValidationError::new(1004, "restricted_month_exact2_number exceeds the maximum value of 12.000000".to_string()));
			}
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
			if let Err(e) = self.rptg_ntty.validate() { return Err(e); }
			if let Err(e) = self.rptg_prd.validate() { return Err(e); }
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
		pub mkt_id_cd: Option<MICIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtlCmptntAuthrty", skip_serializing_if = "Option::is_none") )]
		pub ntl_cmptnt_authrty: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<TradingVenueIdentification2>,
	}
	
	impl TradingVenueIdentification1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref mkt_id_cd_value) = self.mkt_id_cd { if let Err(e) = mkt_id_cd_value.validate() { return Err(e); } }
			if let Some(ref ntl_cmptnt_authrty_value) = self.ntl_cmptnt_authrty { if let Err(e) = ntl_cmptnt_authrty_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
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
		pub id: Max50Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: TradingVenue2Code,
	}
	
	impl TradingVenueIdentification2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Err(e) = self.tp.validate() { return Err(e); }
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
		pub tech_rcrd_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: ISINOct2015Identifier,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FullNm", skip_serializing_if = "Option::is_none") )]
		pub full_nm: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradgVn", skip_serializing_if = "Option::is_none") )]
		pub tradg_vn: Option<MICIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgDt", skip_serializing_if = "Option::is_none") )]
		pub rptg_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none") )]
		pub mtrty_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmClssfctn") )]
		pub fin_instrm_clssfctn: NonEquityInstrumentReportingClassification1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygInstrmAsstClss", skip_serializing_if = "Option::is_none") )]
		pub undrlyg_instrm_asst_clss: Option<ExternalProductType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DerivCtrctTp", skip_serializing_if = "Option::is_none") )]
		pub deriv_ctrct_tp: Option<FinancialInstrumentContractType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Bd", skip_serializing_if = "Option::is_none") )]
		pub bd: Option<DebtInstrument5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EmssnAllwncTp", skip_serializing_if = "Option::is_none") )]
		pub emssn_allwnc_tp: Option<ExternalEmissionAllowanceSubProductType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Deriv", skip_serializing_if = "Option::is_none") )]
		pub deriv: Option<Derivative3Choice>,
	}
	
	impl TransparencyDataReport21 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tech_rcrd_id_value) = self.tech_rcrd_id { if let Err(e) = tech_rcrd_id_value.validate() { return Err(e); } }
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref full_nm_value) = self.full_nm { if let Err(e) = full_nm_value.validate() { return Err(e); } }
			if let Some(ref tradg_vn_value) = self.tradg_vn { if let Err(e) = tradg_vn_value.validate() { return Err(e); } }
			if let Err(e) = self.fin_instrm_clssfctn.validate() { return Err(e); }
			if let Some(ref undrlyg_instrm_asst_clss_value) = self.undrlyg_instrm_asst_clss { if let Err(e) = undrlyg_instrm_asst_clss_value.validate() { return Err(e); } }
			if let Some(ref deriv_ctrct_tp_value) = self.deriv_ctrct_tp { if let Err(e) = deriv_ctrct_tp_value.validate() { return Err(e); } }
			if let Some(ref bd_value) = self.bd { if let Err(e) = bd_value.validate() { return Err(e); } }
			if let Some(ref emssn_allwnc_tp_value) = self.emssn_allwnc_tp { if let Err(e) = emssn_allwnc_tp_value.validate() { return Err(e); } }
			if let Some(ref deriv_value) = self.deriv { if let Err(e) = deriv_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TrueFalseIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct TrueFalseIndicator {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub true_false_indicator: bool,
	}
	
	impl TrueFalseIndicator {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	
}