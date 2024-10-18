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


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_code: String,
}

impl ActiveOrHistoricCurrencyCode {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.active_or_historic_currency_code) {
			return false
		}
		return true
	}
}


// AssetClassSubProductType19Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType19Code {
	#[default]
	#[serde(rename = "DLVR")]
	CodeDLVR,
	#[serde(rename = "NDLV")]
	CodeNDLV,
}

impl AssetClassSubProductType19Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// BenchmarkCurveName2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BenchmarkCurveName2Code {
	#[default]
	#[serde(rename = "WIBO")]
	CodeWIBO,
	#[serde(rename = "TREA")]
	CodeTREA,
	#[serde(rename = "TIBO")]
	CodeTIBO,
	#[serde(rename = "TLBO")]
	CodeTLBO,
	#[serde(rename = "SWAP")]
	CodeSWAP,
	#[serde(rename = "STBO")]
	CodeSTBO,
	#[serde(rename = "PRBO")]
	CodePRBO,
	#[serde(rename = "PFAN")]
	CodePFAN,
	#[serde(rename = "NIBO")]
	CodeNIBO,
	#[serde(rename = "MAAA")]
	CodeMAAA,
	#[serde(rename = "MOSP")]
	CodeMOSP,
	#[serde(rename = "LIBO")]
	CodeLIBO,
	#[serde(rename = "LIBI")]
	CodeLIBI,
	#[serde(rename = "JIBA")]
	CodeJIBA,
	#[serde(rename = "ISDA")]
	CodeISDA,
	#[serde(rename = "GCFR")]
	CodeGCFR,
	#[serde(rename = "FUSW")]
	CodeFUSW,
	#[serde(rename = "EUCH")]
	CodeEUCH,
	#[serde(rename = "EUUS")]
	CodeEUUS,
	#[serde(rename = "EURI")]
	CodeEURI,
	#[serde(rename = "EONS")]
	CodeEONS,
	#[serde(rename = "EONA")]
	CodeEONA,
	#[serde(rename = "CIBO")]
	CodeCIBO,
	#[serde(rename = "CDOR")]
	CodeCDOR,
	#[serde(rename = "BUBO")]
	CodeBUBO,
	#[serde(rename = "BBSW")]
	CodeBBSW,
}

impl BenchmarkCurveName2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// BenchmarkCurveName5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkCurveName5Choice {
	#[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
	pub indx: Option<BenchmarkCurveName2Code>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max25Text>,
}

impl BenchmarkCurveName5Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref indx_value) = self.indx { if !indx_value.validate() { return false; } }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		return true
	}
}


// BondDerivative2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BondDerivative2 {
	#[serde(rename = "Issr")]
	pub issr: LEIIdentifier,
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "IssncDt", skip_serializing_if = "Option::is_none")]
	pub issnc_dt: Option<String>,
}

impl BondDerivative2 {
	pub fn validate(&self) -> bool {
		if !self.issr.validate() { return false }
		return true
	}
}


// BondType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BondType1Code {
	#[default]
	#[serde(rename = "EUSB")]
	CodeEUSB,
	#[serde(rename = "OEPB")]
	CodeOEPB,
	#[serde(rename = "CVTB")]
	CodeCVTB,
	#[serde(rename = "CRPB")]
	CodeCRPB,
	#[serde(rename = "CVDB")]
	CodeCVDB,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}

impl BondType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CommodityDerivative2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommodityDerivative2Choice {
	#[serde(rename = "Frght", skip_serializing_if = "Option::is_none")]
	pub frght: Option<CommodityDerivative5>,
	#[serde(rename = "Nrgy", skip_serializing_if = "Option::is_none")]
	pub nrgy: Option<CommodityDerivative6>,
}

impl CommodityDerivative2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref frght_value) = self.frght { if !frght_value.validate() { return false; } }
		if let Some(ref nrgy_value) = self.nrgy { if !nrgy_value.validate() { return false; } }
		return true
	}
}


// CommodityDerivative4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommodityDerivative4 {
	#[serde(rename = "ClssSpcfc", skip_serializing_if = "Option::is_none")]
	pub clss_spcfc: Option<CommodityDerivative2Choice>,
	#[serde(rename = "NtnlCcy")]
	pub ntnl_ccy: ActiveOrHistoricCurrencyCode,
}

impl CommodityDerivative4 {
	pub fn validate(&self) -> bool {
		if let Some(ref clss_spcfc_value) = self.clss_spcfc { if !clss_spcfc_value.validate() { return false; } }
		if !self.ntnl_ccy.validate() { return false }
		return true
	}
}


// CommodityDerivative5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommodityDerivative5 {
	#[serde(rename = "Sz")]
	pub sz: Max25Text,
	#[serde(rename = "AvrgTmChrtr")]
	pub avrg_tm_chrtr: Max25Text,
}

impl CommodityDerivative5 {
	pub fn validate(&self) -> bool {
		if !self.sz.validate() { return false }
		if !self.avrg_tm_chrtr.validate() { return false }
		return true
	}
}


// CommodityDerivative6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommodityDerivative6 {
	#[serde(rename = "SttlmLctn")]
	pub sttlm_lctn: Max25Text,
}

impl CommodityDerivative6 {
	pub fn validate(&self) -> bool {
		if !self.sttlm_lctn.validate() { return false }
		return true
	}
}


// ContractForDifference2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractForDifference2 {
	#[serde(rename = "UndrlygTp")]
	pub undrlyg_tp: UnderlyingContractForDifferenceType3Code,
	#[serde(rename = "NtnlCcy1", skip_serializing_if = "Option::is_none")]
	pub ntnl_ccy1: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "NtnlCcy2", skip_serializing_if = "Option::is_none")]
	pub ntnl_ccy2: Option<ActiveOrHistoricCurrencyCode>,
}

impl ContractForDifference2 {
	pub fn validate(&self) -> bool {
		if !self.undrlyg_tp.validate() { return false }
		if let Some(ref ntnl_ccy1_value) = self.ntnl_ccy1 { if !ntnl_ccy1_value.validate() { return false; } }
		if let Some(ref ntnl_ccy2_value) = self.ntnl_ccy2 { if !ntnl_ccy2_value.validate() { return false; } }
		return true
	}
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}

impl CountryCode {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.country_code) {
			return false
		}
		return true
	}
}


// CountrySubDivisionCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CountrySubDivisionCode {
	#[serde(rename = "$value")]
	pub country_sub_division_code: String,
}

impl CountrySubDivisionCode {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{2,2}\\-[0-9A-Z]{1,3}").unwrap();
		if !pattern.is_match(&self.country_sub_division_code) {
			return false
		}
		return true
	}
}


// CreditDefaultSwapDerivative5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditDefaultSwapDerivative5 {
	#[serde(rename = "UndrlygCdtDfltSwpId", skip_serializing_if = "Option::is_none")]
	pub undrlyg_cdt_dflt_swp_id: Option<ISINOct2015Identifier>,
	#[serde(rename = "UndrlygCdtDfltSwpIndx")]
	pub undrlyg_cdt_dflt_swp_indx: CreditDefaultSwapIndex3,
}

impl CreditDefaultSwapDerivative5 {
	pub fn validate(&self) -> bool {
		if let Some(ref undrlyg_cdt_dflt_swp_id_value) = self.undrlyg_cdt_dflt_swp_id { if !undrlyg_cdt_dflt_swp_id_value.validate() { return false; } }
		if !self.undrlyg_cdt_dflt_swp_indx.validate() { return false }
		return true
	}
}


// CreditDefaultSwapDerivative6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditDefaultSwapDerivative6 {
	#[serde(rename = "UndrlygCdtDfltSwpId", skip_serializing_if = "Option::is_none")]
	pub undrlyg_cdt_dflt_swp_id: Option<ISINOct2015Identifier>,
	#[serde(rename = "OblgtnId")]
	pub oblgtn_id: ISINOct2015Identifier,
	#[serde(rename = "SnglNm")]
	pub sngl_nm: CreditDefaultSwapSingleName2,
}

impl CreditDefaultSwapDerivative6 {
	pub fn validate(&self) -> bool {
		if let Some(ref undrlyg_cdt_dflt_swp_id_value) = self.undrlyg_cdt_dflt_swp_id { if !undrlyg_cdt_dflt_swp_id_value.validate() { return false; } }
		if !self.oblgtn_id.validate() { return false }
		if !self.sngl_nm.validate() { return false }
		return true
	}
}


// CreditDefaultSwapIndex3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditDefaultSwapIndex3 {
	#[serde(rename = "UndrlygIndxId", skip_serializing_if = "Option::is_none")]
	pub undrlyg_indx_id: Option<ISINOct2015Identifier>,
	#[serde(rename = "UndrlygIndxNm", skip_serializing_if = "Option::is_none")]
	pub undrlyg_indx_nm: Option<Max25Text>,
	#[serde(rename = "Srs", skip_serializing_if = "Option::is_none")]
	pub srs: Option<f64>,
	#[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
	pub vrsn: Option<f64>,
	#[serde(rename = "RollMnth", skip_serializing_if = "Option::is_none")]
	pub roll_mnth: Option<Vec<RestrictedMonthExact2Number>>,
	#[serde(rename = "NxtRollDt", skip_serializing_if = "Option::is_none")]
	pub nxt_roll_dt: Option<String>,
	#[serde(rename = "NtnlCcy")]
	pub ntnl_ccy: ActiveOrHistoricCurrencyCode,
}

impl CreditDefaultSwapIndex3 {
	pub fn validate(&self) -> bool {
		if let Some(ref undrlyg_indx_id_value) = self.undrlyg_indx_id { if !undrlyg_indx_id_value.validate() { return false; } }
		if let Some(ref undrlyg_indx_nm_value) = self.undrlyg_indx_nm { if !undrlyg_indx_nm_value.validate() { return false; } }
		if let Some(ref roll_mnth_vec) = self.roll_mnth { for item in roll_mnth_vec { if !item.validate() { return false; } } }
		if !self.ntnl_ccy.validate() { return false }
		return true
	}
}


// CreditDefaultSwapSingleName2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditDefaultSwapSingleName2 {
	#[serde(rename = "SvrgnIssr")]
	pub svrgn_issr: bool,
	#[serde(rename = "RefPty", skip_serializing_if = "Option::is_none")]
	pub ref_pty: Option<DerivativePartyIdentification1Choice>,
	#[serde(rename = "NtnlCcy")]
	pub ntnl_ccy: ActiveOrHistoricCurrencyCode,
}

impl CreditDefaultSwapSingleName2 {
	pub fn validate(&self) -> bool {
		if let Some(ref ref_pty_value) = self.ref_pty { if !ref_pty_value.validate() { return false; } }
		if !self.ntnl_ccy.validate() { return false }
		return true
	}
}


// CreditDefaultSwapsDerivative4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditDefaultSwapsDerivative4Choice {
	#[serde(rename = "SnglNmCdtDfltSwp", skip_serializing_if = "Option::is_none")]
	pub sngl_nm_cdt_dflt_swp: Option<CreditDefaultSwapSingleName2>,
	#[serde(rename = "CdtDfltSwpIndx", skip_serializing_if = "Option::is_none")]
	pub cdt_dflt_swp_indx: Option<CreditDefaultSwapIndex3>,
	#[serde(rename = "SnglNmCdtDfltSwpDeriv", skip_serializing_if = "Option::is_none")]
	pub sngl_nm_cdt_dflt_swp_deriv: Option<CreditDefaultSwapDerivative6>,
	#[serde(rename = "CdtDfltSwpIndxDeriv", skip_serializing_if = "Option::is_none")]
	pub cdt_dflt_swp_indx_deriv: Option<CreditDefaultSwapDerivative5>,
}

impl CreditDefaultSwapsDerivative4Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref sngl_nm_cdt_dflt_swp_value) = self.sngl_nm_cdt_dflt_swp { if !sngl_nm_cdt_dflt_swp_value.validate() { return false; } }
		if let Some(ref cdt_dflt_swp_indx_value) = self.cdt_dflt_swp_indx { if !cdt_dflt_swp_indx_value.validate() { return false; } }
		if let Some(ref sngl_nm_cdt_dflt_swp_deriv_value) = self.sngl_nm_cdt_dflt_swp_deriv { if !sngl_nm_cdt_dflt_swp_deriv_value.validate() { return false; } }
		if let Some(ref cdt_dflt_swp_indx_deriv_value) = self.cdt_dflt_swp_indx_deriv { if !cdt_dflt_swp_indx_deriv_value.validate() { return false; } }
		return true
	}
}


// DebtInstrument5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DebtInstrument5 {
	#[serde(rename = "Tp")]
	pub tp: BondType1Code,
	#[serde(rename = "IssncDt")]
	pub issnc_dt: String,
}

impl DebtInstrument5 {
	pub fn validate(&self) -> bool {
		if !self.tp.validate() { return false }
		return true
	}
}


// Derivative3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Derivative3Choice {
	#[serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none")]
	pub cmmdty: Option<CommodityDerivative4>,
	#[serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none")]
	pub intrst_rate: Option<InterestRateDerivative5>,
	#[serde(rename = "FX", skip_serializing_if = "Option::is_none")]
	pub fx: Option<ForeignExchangeDerivative2>,
	#[serde(rename = "Eqty", skip_serializing_if = "Option::is_none")]
	pub eqty: Option<EquityDerivative2>,
	#[serde(rename = "CtrctForDiff", skip_serializing_if = "Option::is_none")]
	pub ctrct_for_diff: Option<ContractForDifference2>,
	#[serde(rename = "Cdt", skip_serializing_if = "Option::is_none")]
	pub cdt: Option<CreditDefaultSwapsDerivative4Choice>,
	#[serde(rename = "EmssnAllwnc", skip_serializing_if = "Option::is_none")]
	pub emssn_allwnc: Option<EmissionAllowanceProductType1Code>,
}

impl Derivative3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cmmdty_value) = self.cmmdty { if !cmmdty_value.validate() { return false; } }
		if let Some(ref intrst_rate_value) = self.intrst_rate { if !intrst_rate_value.validate() { return false; } }
		if let Some(ref fx_value) = self.fx { if !fx_value.validate() { return false; } }
		if let Some(ref eqty_value) = self.eqty { if !eqty_value.validate() { return false; } }
		if let Some(ref ctrct_for_diff_value) = self.ctrct_for_diff { if !ctrct_for_diff_value.validate() { return false; } }
		if let Some(ref cdt_value) = self.cdt { if !cdt_value.validate() { return false; } }
		if let Some(ref emssn_allwnc_value) = self.emssn_allwnc { if !emssn_allwnc_value.validate() { return false; } }
		return true
	}
}


// DerivativePartyIdentification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativePartyIdentification1Choice {
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<CountrySubDivisionCode>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
}

impl DerivativePartyIdentification1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref ctry_value) = self.ctry { if !ctry_value.validate() { return false; } }
		if let Some(ref ctry_sub_dvsn_value) = self.ctry_sub_dvsn { if !ctry_sub_dvsn_value.validate() { return false; } }
		if let Some(ref lei_value) = self.lei { if !lei_value.validate() { return false; } }
		return true
	}
}


// EmissionAllowanceProductType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EmissionAllowanceProductType1Code {
	#[default]
	#[serde(rename = "EUAA")]
	CodeEUAA,
	#[serde(rename = "EUAE")]
	CodeEUAE,
	#[serde(rename = "ERUE")]
	CodeERUE,
	#[serde(rename = "CERE")]
	CodeCERE,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}

impl EmissionAllowanceProductType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// EquityDerivative2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EquityDerivative2 {
	#[serde(rename = "UndrlygTp")]
	pub undrlyg_tp: EquityDerivative3Choice,
	#[serde(rename = "Param", skip_serializing_if = "Option::is_none")]
	pub param: Option<EquityReturnParameter1Code>,
}

impl EquityDerivative2 {
	pub fn validate(&self) -> bool {
		if !self.undrlyg_tp.validate() { return false }
		if let Some(ref param_value) = self.param { if !param_value.validate() { return false; } }
		return true
	}
}


// EquityDerivative3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EquityDerivative3Choice {
	#[serde(rename = "Bskt", skip_serializing_if = "Option::is_none")]
	pub bskt: Option<UnderlyingEquityType3Code>,
	#[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
	pub indx: Option<UnderlyingEquityType4Code>,
	#[serde(rename = "SnglNm", skip_serializing_if = "Option::is_none")]
	pub sngl_nm: Option<UnderlyingEquityType5Code>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<UnderlyingEquityType6Code>,
}

impl EquityDerivative3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref bskt_value) = self.bskt { if !bskt_value.validate() { return false; } }
		if let Some(ref indx_value) = self.indx { if !indx_value.validate() { return false; } }
		if let Some(ref sngl_nm_value) = self.sngl_nm { if !sngl_nm_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
	}
}


// EquityReturnParameter1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EquityReturnParameter1Code {
	#[default]
	#[serde(rename = "PRDV")]
	CodePRDV,
	#[serde(rename = "PRVA")]
	CodePRVA,
	#[serde(rename = "PRVO")]
	CodePRVO,
	#[serde(rename = "PRBP")]
	CodePRBP,
}

impl EquityReturnParameter1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ExternalEmissionAllowanceSubProductType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalEmissionAllowanceSubProductType1Code {
	#[serde(rename = "$value")]
	pub external_emission_allowance_sub_product_type1_code: String,
}

impl ExternalEmissionAllowanceSubProductType1Code {
	pub fn validate(&self) -> bool {
		if self.external_emission_allowance_sub_product_type1_code.chars().count() < 1 {
			return false
		}
		if self.external_emission_allowance_sub_product_type1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// ExternalProductType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalProductType1Code {
	#[serde(rename = "$value")]
	pub external_product_type1_code: String,
}

impl ExternalProductType1Code {
	pub fn validate(&self) -> bool {
		if self.external_product_type1_code.chars().count() < 1 {
			return false
		}
		if self.external_product_type1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// FinancialInstrumentContractType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FinancialInstrumentContractType1Code {
	#[default]
	#[serde(rename = "CFDS")]
	CodeCFDS,
	#[serde(rename = "FORW")]
	CodeFORW,
	#[serde(rename = "FRAS")]
	CodeFRAS,
	#[serde(rename = "FUTR")]
	CodeFUTR,
	#[serde(rename = "OPTN")]
	CodeOPTN,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "SPDB")]
	CodeSPDB,
	#[serde(rename = "SWAP")]
	CodeSWAP,
	#[serde(rename = "SWPT")]
	CodeSWPT,
	#[serde(rename = "FONS")]
	CodeFONS,
	#[serde(rename = "PSWP")]
	CodePSWP,
	#[serde(rename = "FFAS")]
	CodeFFAS,
	#[serde(rename = "FWOS")]
	CodeFWOS,
}

impl FinancialInstrumentContractType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// FinancialInstrumentReportingNonEquityTransparencyDataReportV03 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentReportingNonEquityTransparencyDataReportV03 {
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: SecuritiesMarketReportHeader1,
	#[serde(rename = "NonEqtyTrnsprncyData")]
	pub non_eqty_trnsprncy_data: Vec<TransparencyDataReport21>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl FinancialInstrumentReportingNonEquityTransparencyDataReportV03 {
	pub fn validate(&self) -> bool {
		if !self.rpt_hdr.validate() { return false }
		for item in &self.non_eqty_trnsprncy_data { if !item.validate() { return false; } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
	}
}


// FloatingInterestRate8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FloatingInterestRate8 {
	#[serde(rename = "RefRate")]
	pub ref_rate: BenchmarkCurveName5Choice,
	#[serde(rename = "Term", skip_serializing_if = "Option::is_none")]
	pub term: Option<InterestRateContractTerm2>,
}

impl FloatingInterestRate8 {
	pub fn validate(&self) -> bool {
		if !self.ref_rate.validate() { return false }
		if let Some(ref term_value) = self.term { if !term_value.validate() { return false; } }
		return true
	}
}


// ForeignExchangeDerivative2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ForeignExchangeDerivative2 {
	#[serde(rename = "CtrctSubTp")]
	pub ctrct_sub_tp: AssetClassSubProductType19Code,
}

impl ForeignExchangeDerivative2 {
	pub fn validate(&self) -> bool {
		if !self.ctrct_sub_tp.validate() { return false }
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


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}

impl ISODate {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}

impl ISODateTime {
	pub fn validate(&self) -> bool {
		return true
	}
}


// InflationIndex1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InflationIndex1Choice {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max25Text>,
}

impl InflationIndex1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref isin_value) = self.isin { if !isin_value.validate() { return false; } }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		return true
	}
}


// InterestRateContractTerm2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRateContractTerm2 {
	#[serde(rename = "Unit")]
	pub unit: RateBasis1Code,
	#[serde(rename = "Val")]
	pub val: f64,
}

impl InterestRateContractTerm2 {
	pub fn validate(&self) -> bool {
		if !self.unit.validate() { return false }
		return true
	}
}


// InterestRateDerivative2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRateDerivative2Choice {
	#[serde(rename = "SwpRltd", skip_serializing_if = "Option::is_none")]
	pub swp_rltd: Option<SwapType1Code>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<UnderlyingInterestRateType3Code>,
}

impl InterestRateDerivative2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref swp_rltd_value) = self.swp_rltd { if !swp_rltd_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
	}
}


// InterestRateDerivative5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRateDerivative5 {
	#[serde(rename = "UndrlygTp")]
	pub undrlyg_tp: InterestRateDerivative2Choice,
	#[serde(rename = "UndrlygBd", skip_serializing_if = "Option::is_none")]
	pub undrlyg_bd: Option<BondDerivative2>,
	#[serde(rename = "SwptnNtnlCcy", skip_serializing_if = "Option::is_none")]
	pub swptn_ntnl_ccy: Option<ActiveCurrencyCode>,
	#[serde(rename = "UndrlygSwpMtrtyDt", skip_serializing_if = "Option::is_none")]
	pub undrlyg_swp_mtrty_dt: Option<String>,
	#[serde(rename = "InfltnIndx", skip_serializing_if = "Option::is_none")]
	pub infltn_indx: Option<InflationIndex1Choice>,
	#[serde(rename = "IntrstRateRef")]
	pub intrst_rate_ref: FloatingInterestRate8,
}

impl InterestRateDerivative5 {
	pub fn validate(&self) -> bool {
		if !self.undrlyg_tp.validate() { return false }
		if let Some(ref undrlyg_bd_value) = self.undrlyg_bd { if !undrlyg_bd_value.validate() { return false; } }
		if let Some(ref swptn_ntnl_ccy_value) = self.swptn_ntnl_ccy { if !swptn_ntnl_ccy_value.validate() { return false; } }
		if let Some(ref infltn_indx_value) = self.infltn_indx { if !infltn_indx_value.validate() { return false; } }
		if !self.intrst_rate_ref.validate() { return false }
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


// MICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MICIdentifier {
	#[serde(rename = "$value")]
	pub mic_identifier: String,
}

impl MICIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
		if !pattern.is_match(&self.mic_identifier) {
			return false
		}
		return true
	}
}


// Max25Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max25Text {
	#[serde(rename = "$value")]
	pub max25_text: String,
}

impl Max25Text {
	pub fn validate(&self) -> bool {
		if self.max25_text.chars().count() < 1 {
			return false
		}
		if self.max25_text.chars().count() > 25 {
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


// Max3Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max3Number {
	#[serde(rename = "$value")]
	pub max3_number: f64,
}

impl Max3Number {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Max50Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max50Text {
	#[serde(rename = "$value")]
	pub max50_text: String,
}

impl Max50Text {
	pub fn validate(&self) -> bool {
		if self.max50_text.chars().count() < 1 {
			return false
		}
		if self.max50_text.chars().count() > 50 {
			return false
		}
		return true
	}
}


// NonEquityInstrumentReportingClassification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NonEquityInstrumentReportingClassification1Code {
	#[default]
	#[serde(rename = "SFPS")]
	CodeSFPS,
	#[serde(rename = "SDRV")]
	CodeSDRV,
	#[serde(rename = "DERV")]
	CodeDERV,
	#[serde(rename = "EMAL")]
	CodeEMAL,
	#[serde(rename = "BOND")]
	CodeBOND,
	#[serde(rename = "ETCS")]
	CodeETCS,
	#[serde(rename = "ETNS")]
	CodeETNS,
}

impl NonEquityInstrumentReportingClassification1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Number {
	#[serde(rename = "$value")]
	pub number: f64,
}

impl Number {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Period2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}

impl Period2 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Period4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period4Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt", skip_serializing_if = "Option::is_none")]
	pub to_dt: Option<String>,
	#[serde(rename = "FrDtToDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt_to_dt: Option<Period2>,
}

impl Period4Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref fr_dt_to_dt_value) = self.fr_dt_to_dt { if !fr_dt_to_dt_value.validate() { return false; } }
		return true
	}
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

impl RateBasis1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// RestrictedMonthExact2Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RestrictedMonthExact2Number {
	#[serde(rename = "$value")]
	pub restricted_month_exact2_number: f64,
}

impl RestrictedMonthExact2Number {
	pub fn validate(&self) -> bool {
		if self.restricted_month_exact2_number < 1.000000 {
			return false
		}
		if self.restricted_month_exact2_number > 12.000000 {
			return false
		}
		return true
	}
}


// SecuritiesMarketReportHeader1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesMarketReportHeader1 {
	#[serde(rename = "RptgNtty")]
	pub rptg_ntty: TradingVenueIdentification1Choice,
	#[serde(rename = "RptgPrd")]
	pub rptg_prd: Period4Choice,
	#[serde(rename = "SubmissnDtTm", skip_serializing_if = "Option::is_none")]
	pub submissn_dt_tm: Option<String>,
}

impl SecuritiesMarketReportHeader1 {
	pub fn validate(&self) -> bool {
		if !self.rptg_ntty.validate() { return false }
		if !self.rptg_prd.validate() { return false }
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


// SwapType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SwapType1Code {
	#[default]
	#[serde(rename = "OSSC")]
	CodeOSSC,
	#[serde(rename = "XFSC")]
	CodeXFSC,
	#[serde(rename = "XFMC")]
	CodeXFMC,
	#[serde(rename = "XXSC")]
	CodeXXSC,
	#[serde(rename = "XXMC")]
	CodeXXMC,
	#[serde(rename = "IFMC")]
	CodeIFMC,
	#[serde(rename = "FFSC")]
	CodeFFSC,
	#[serde(rename = "FFMC")]
	CodeFFMC,
	#[serde(rename = "IFSC")]
	CodeIFSC,
	#[serde(rename = "OSMC")]
	CodeOSMC,
}

impl SwapType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// TradingVenue2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TradingVenue2Code {
	#[default]
	#[serde(rename = "APPA")]
	CodeAPPA,
	#[serde(rename = "CTPS")]
	CodeCTPS,
}

impl TradingVenue2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// TradingVenueIdentification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingVenueIdentification1Choice {
	#[serde(rename = "MktIdCd", skip_serializing_if = "Option::is_none")]
	pub mkt_id_cd: Option<MICIdentifier>,
	#[serde(rename = "NtlCmptntAuthrty", skip_serializing_if = "Option::is_none")]
	pub ntl_cmptnt_authrty: Option<CountryCode>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<TradingVenueIdentification2>,
}

impl TradingVenueIdentification1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref mkt_id_cd_value) = self.mkt_id_cd { if !mkt_id_cd_value.validate() { return false; } }
		if let Some(ref ntl_cmptnt_authrty_value) = self.ntl_cmptnt_authrty { if !ntl_cmptnt_authrty_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
	}
}


// TradingVenueIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingVenueIdentification2 {
	#[serde(rename = "Id")]
	pub id: Max50Text,
	#[serde(rename = "Tp")]
	pub tp: TradingVenue2Code,
}

impl TradingVenueIdentification2 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.tp.validate() { return false }
		return true
	}
}


// TransparencyDataReport21 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransparencyDataReport21 {
	#[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
	pub tech_rcrd_id: Option<Max35Text>,
	#[serde(rename = "Id")]
	pub id: ISINOct2015Identifier,
	#[serde(rename = "FullNm", skip_serializing_if = "Option::is_none")]
	pub full_nm: Option<Max350Text>,
	#[serde(rename = "TradgVn", skip_serializing_if = "Option::is_none")]
	pub tradg_vn: Option<MICIdentifier>,
	#[serde(rename = "RptgDt", skip_serializing_if = "Option::is_none")]
	pub rptg_dt: Option<String>,
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "FinInstrmClssfctn")]
	pub fin_instrm_clssfctn: NonEquityInstrumentReportingClassification1Code,
	#[serde(rename = "UndrlygInstrmAsstClss", skip_serializing_if = "Option::is_none")]
	pub undrlyg_instrm_asst_clss: Option<ExternalProductType1Code>,
	#[serde(rename = "DerivCtrctTp", skip_serializing_if = "Option::is_none")]
	pub deriv_ctrct_tp: Option<FinancialInstrumentContractType1Code>,
	#[serde(rename = "Bd", skip_serializing_if = "Option::is_none")]
	pub bd: Option<DebtInstrument5>,
	#[serde(rename = "EmssnAllwncTp", skip_serializing_if = "Option::is_none")]
	pub emssn_allwnc_tp: Option<ExternalEmissionAllowanceSubProductType1Code>,
	#[serde(rename = "Deriv", skip_serializing_if = "Option::is_none")]
	pub deriv: Option<Derivative3Choice>,
}

impl TransparencyDataReport21 {
	pub fn validate(&self) -> bool {
		if let Some(ref tech_rcrd_id_value) = self.tech_rcrd_id { if !tech_rcrd_id_value.validate() { return false; } }
		if !self.id.validate() { return false }
		if let Some(ref full_nm_value) = self.full_nm { if !full_nm_value.validate() { return false; } }
		if let Some(ref tradg_vn_value) = self.tradg_vn { if !tradg_vn_value.validate() { return false; } }
		if !self.fin_instrm_clssfctn.validate() { return false }
		if let Some(ref undrlyg_instrm_asst_clss_value) = self.undrlyg_instrm_asst_clss { if !undrlyg_instrm_asst_clss_value.validate() { return false; } }
		if let Some(ref deriv_ctrct_tp_value) = self.deriv_ctrct_tp { if !deriv_ctrct_tp_value.validate() { return false; } }
		if let Some(ref bd_value) = self.bd { if !bd_value.validate() { return false; } }
		if let Some(ref emssn_allwnc_tp_value) = self.emssn_allwnc_tp { if !emssn_allwnc_tp_value.validate() { return false; } }
		if let Some(ref deriv_value) = self.deriv { if !deriv_value.validate() { return false; } }
		return true
	}
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}

impl TrueFalseIndicator {
	pub fn validate(&self) -> bool {
		return true
	}
}


// UnderlyingContractForDifferenceType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum UnderlyingContractForDifferenceType3Code {
	#[default]
	#[serde(rename = "BOND")]
	CodeBOND,
	#[serde(rename = "COMM")]
	CodeCOMM,
	#[serde(rename = "CURR")]
	CodeCURR,
	#[serde(rename = "EMAL")]
	CodeEMAL,
	#[serde(rename = "EQUI")]
	CodeEQUI,
	#[serde(rename = "FTEQ")]
	CodeFTEQ,
	#[serde(rename = "OPEQ")]
	CodeOPEQ,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}

impl UnderlyingContractForDifferenceType3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// UnderlyingEquityType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum UnderlyingEquityType3Code {
	#[default]
	#[serde(rename = "BSKT")]
	CodeBSKT,
}

impl UnderlyingEquityType3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// UnderlyingEquityType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum UnderlyingEquityType4Code {
	#[default]
	#[serde(rename = "STIX")]
	CodeSTIX,
	#[serde(rename = "DIVI")]
	CodeDIVI,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "VOLI")]
	CodeVOLI,
}

impl UnderlyingEquityType4Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// UnderlyingEquityType5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum UnderlyingEquityType5Code {
	#[default]
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "ETFS")]
	CodeETFS,
	#[serde(rename = "SHRS")]
	CodeSHRS,
	#[serde(rename = "DVSE")]
	CodeDVSE,
}

impl UnderlyingEquityType5Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// UnderlyingEquityType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum UnderlyingEquityType6Code {
	#[default]
	#[serde(rename = "BSKT")]
	CodeBSKT,
	#[serde(rename = "DIVI")]
	CodeDIVI,
	#[serde(rename = "ETFS")]
	CodeETFS,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "SHRS")]
	CodeSHRS,
	#[serde(rename = "DVSE")]
	CodeDVSE,
	#[serde(rename = "STIX")]
	CodeSTIX,
	#[serde(rename = "VOLI")]
	CodeVOLI,
}

impl UnderlyingEquityType6Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// UnderlyingInterestRateType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum UnderlyingInterestRateType3Code {
	#[default]
	#[serde(rename = "BOND")]
	CodeBOND,
	#[serde(rename = "BNDF")]
	CodeBNDF,
	#[serde(rename = "INTR")]
	CodeINTR,
	#[serde(rename = "IFUT")]
	CodeIFUT,
}

impl UnderlyingInterestRateType3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}
