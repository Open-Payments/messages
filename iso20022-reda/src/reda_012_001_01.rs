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


// ActiveCurrencyAnd13DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyAnd13DecimalAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_currency_and13_decimal_amount_simple_type: f64,
}

impl ActiveCurrencyAnd13DecimalAmountSimpleType {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.active_currency_and13_decimal_amount_simple_type < 0.000000 {
			return Err(ValidationError::new(1003, "active_currency_and13_decimal_amount_simple_type is less than the minimum value of 0.000000".to_string()));
		}
		Ok(())
	}
}


// ActiveCurrencyAnd13DecimalAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd13DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveCurrencyAnd13DecimalAmount {
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


// ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_and13_decimal_amount_simple_type: f64,
}

impl ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.active_or_historic_currency_and13_decimal_amount_simple_type < 0.000000 {
			return Err(ValidationError::new(1003, "active_or_historic_currency_and13_decimal_amount_simple_type is less than the minimum value of 0.000000".to_string()));
		}
		Ok(())
	}
}


// ActiveOrHistoricCurrencyAnd13DecimalAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveOrHistoricCurrencyAnd13DecimalAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.active_or_historic_currency_code) {
			return Err(ValidationError::new(1005, "active_or_historic_currency_code does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// AddressType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AddressType1Code {
	#[default]
	#[serde(rename = "HOME")]
	CodeHOME,
	#[serde(rename = "BIZZ")]
	CodeBIZZ,
}

impl AddressType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AddressType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AddressType2Code {
	#[default]
	#[serde(rename = "ADDR")]
	CodeADDR,
	#[serde(rename = "PBOX")]
	CodePBOX,
	#[serde(rename = "HOME")]
	CodeHOME,
	#[serde(rename = "BIZZ")]
	CodeBIZZ,
	#[serde(rename = "MLTO")]
	CodeMLTO,
	#[serde(rename = "DLVY")]
	CodeDLVY,
}

impl AddressType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AmountOrPercentageRange1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountOrPercentageRange1 {
	#[serde(rename = "Opr", skip_serializing_if = "Option::is_none")]
	pub opr: Option<Operation1Code>,
	#[serde(rename = "Term", skip_serializing_if = "Option::is_none")]
	pub term: Option<Vec<Term1>>,
}

impl AmountOrPercentageRange1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref opr_value) = self.opr { if let Err(e) = opr_value.validate() { return Err(e); } }
		if let Some(ref term_vec) = self.term { for item in term_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
}

impl AnyBICDec2014Identifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
		if !pattern.is_match(&self.any_bic_dec2014_identifier) {
			return Err(ValidationError::new(1005, "any_bic_dec2014_identifier does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// Appearance1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Appearance1Code {
	#[default]
	#[serde(rename = "DELI")]
	CodeDELI,
	#[serde(rename = "NDEL")]
	CodeNDEL,
	#[serde(rename = "LIMI")]
	CodeLIMI,
	#[serde(rename = "BENT")]
	CodeBENT,
	#[serde(rename = "DFBE")]
	CodeDFBE,
	#[serde(rename = "DLBE")]
	CodeDLBE,
	#[serde(rename = "TMPG")]
	CodeTMPG,
	#[serde(rename = "GLOB")]
	CodeGLOB,
}

impl Appearance1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Appearance3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Appearance3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Appearance1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl Appearance3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// AssignmentMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssignmentMethod1Code {
	#[default]
	#[serde(rename = "RAND")]
	CodeRAND,
	#[serde(rename = "PROR")]
	CodePROR,
}

impl AssignmentMethod1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AssignmentMethod2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssignmentMethod2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<AssignmentMethod1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl AssignmentMethod2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BaseOneRate {
	#[serde(rename = "$value")]
	pub base_one_rate: f64,
}

impl BaseOneRate {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// BenchmarkCurve6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkCurve6 {
	#[serde(rename = "Sprd", skip_serializing_if = "Option::is_none")]
	pub sprd: Option<f64>,
	#[serde(rename = "BchmkId", skip_serializing_if = "Option::is_none")]
	pub bchmk_id: Option<SecurityIdentification39>,
	#[serde(rename = "BchmkPric", skip_serializing_if = "Option::is_none")]
	pub bchmk_pric: Option<Price8>,
	#[serde(rename = "BchmkCrvCcy", skip_serializing_if = "Option::is_none")]
	pub bchmk_crv_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "BchmkCrvNm", skip_serializing_if = "Option::is_none")]
	pub bchmk_crv_nm: Option<BenchmarkCurveName7Choice>,
	#[serde(rename = "BchmkCrvPt", skip_serializing_if = "Option::is_none")]
	pub bchmk_crv_pt: Option<Max256Text>,
}

impl BenchmarkCurve6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref bchmk_id_value) = self.bchmk_id { if let Err(e) = bchmk_id_value.validate() { return Err(e); } }
		if let Some(ref bchmk_pric_value) = self.bchmk_pric { if let Err(e) = bchmk_pric_value.validate() { return Err(e); } }
		if let Some(ref bchmk_crv_ccy_value) = self.bchmk_crv_ccy { if let Err(e) = bchmk_crv_ccy_value.validate() { return Err(e); } }
		if let Some(ref bchmk_crv_nm_value) = self.bchmk_crv_nm { if let Err(e) = bchmk_crv_nm_value.validate() { return Err(e); } }
		if let Some(ref bchmk_crv_pt_value) = self.bchmk_crv_pt { if let Err(e) = bchmk_crv_pt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// BenchmarkCurveName1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BenchmarkCurveName1Code {
	#[default]
	#[serde(rename = "MAAA")]
	CodeMAAA,
	#[serde(rename = "FUSW")]
	CodeFUSW,
	#[serde(rename = "LIBI")]
	CodeLIBI,
	#[serde(rename = "LIBO")]
	CodeLIBO,
	#[serde(rename = "SWAP")]
	CodeSWAP,
	#[serde(rename = "TREA")]
	CodeTREA,
	#[serde(rename = "EURI")]
	CodeEURI,
	#[serde(rename = "PFAN")]
	CodePFAN,
}

impl BenchmarkCurveName1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// BenchmarkCurveName7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkCurveName7Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<BenchmarkCurveName1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl BenchmarkCurveName7Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// BusinessError4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BusinessError4 {
	#[serde(rename = "FinInstrmId")]
	pub fin_instrm_id: SecurityIdentification39,
	#[serde(rename = "BizErr")]
	pub biz_err: Vec<ErrorHandling5>,
}

impl BusinessError4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.fin_instrm_id.validate() { return Err(e); }
		for item in &self.biz_err { if let Err(e) = item.validate() { return Err(e); } }
		Ok(())
	}
}


// CFIOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CFIOct2015Identifier {
	#[serde(rename = "$value")]
	pub cfi_oct2015_identifier: String,
}

impl CFIOct2015Identifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{6,6}").unwrap();
		if !pattern.is_match(&self.cfi_oct2015_identifier) {
			return Err(ValidationError::new(1005, "cfi_oct2015_identifier does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// CalculationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CalculationType1Code {
	#[default]
	#[serde(rename = "AFTX")]
	CodeAFTX,
	#[serde(rename = "ANNU")]
	CodeANNU,
	#[serde(rename = "ISSU")]
	CodeISSU,
	#[serde(rename = "AVMA")]
	CodeAVMA,
	#[serde(rename = "BOOK")]
	CodeBOOK,
	#[serde(rename = "YTNC")]
	CodeYTNC,
	#[serde(rename = "CHCL")]
	CodeCHCL,
	#[serde(rename = "CLOS")]
	CodeCLOS,
	#[serde(rename = "CMPD")]
	CodeCMPD,
	#[serde(rename = "CUYI")]
	CodeCUYI,
	#[serde(rename = "TRGR")]
	CodeTRGR,
	#[serde(rename = "GVEQ")]
	CodeGVEQ,
	#[serde(rename = "FLAS")]
	CodeFLAS,
	#[serde(rename = "NVFL")]
	CodeNVFL,
	#[serde(rename = "LSCL")]
	CodeLSCL,
	#[serde(rename = "LSMT")]
	CodeLSMT,
	#[serde(rename = "LSQR")]
	CodeLSQR,
	#[serde(rename = "LSYR")]
	CodeLSYR,
	#[serde(rename = "LGAL")]
	CodeLGAL,
	#[serde(rename = "MARK")]
	CodeMARK,
	#[serde(rename = "YTMA")]
	CodeYTMA,
	#[serde(rename = "NXRF")]
	CodeNXRF,
	#[serde(rename = "PNAV")]
	CodePNAV,
	#[serde(rename = "NXPT")]
	CodeNXPT,
	#[serde(rename = "PRCL")]
	CodePRCL,
	#[serde(rename = "PRYL")]
	CodePRYL,
	#[serde(rename = "SEMI")]
	CodeSEMI,
	#[serde(rename = "SHLF")]
	CodeSHLF,
	#[serde(rename = "SPLL")]
	CodeSPLL,
	#[serde(rename = "TXQV")]
	CodeTXQV,
	#[serde(rename = "TTDT")]
	CodeTTDT,
	#[serde(rename = "TRYL")]
	CodeTRYL,
	#[serde(rename = "WRST")]
	CodeWRST,
}

impl CalculationType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CalculationType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CalculationType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<CalculationType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl CalculationType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CallType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CallType1Code {
	#[default]
	#[serde(rename = "LOTT")]
	CodeLOTT,
	#[serde(rename = "PRTA")]
	CodePRTA,
}

impl CallType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CallType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CallType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<CallType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl CallType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ClassificationType2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClassificationType2 {
	#[serde(rename = "ClssfctnFinInstrm", skip_serializing_if = "Option::is_none")]
	pub clssfctn_fin_instrm: Option<CFIOct2015Identifier>,
	#[serde(rename = "FinInstrmPdctTpCd", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_pdct_tp_cd: Option<ExternalFinancialInstrumentProductType1Code>,
	#[serde(rename = "AltrnClssfctn", skip_serializing_if = "Option::is_none")]
	pub altrn_clssfctn: Option<Vec<GenericIdentification36>>,
}

impl ClassificationType2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref clssfctn_fin_instrm_value) = self.clssfctn_fin_instrm { if let Err(e) = clssfctn_fin_instrm_value.validate() { return Err(e); } }
		if let Some(ref fin_instrm_pdct_tp_cd_value) = self.fin_instrm_pdct_tp_cd { if let Err(e) = fin_instrm_pdct_tp_cd_value.validate() { return Err(e); } }
		if let Some(ref altrn_clssfctn_vec) = self.altrn_clssfctn { for item in altrn_clssfctn_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// CommonFinancialInstrumentAttributes11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommonFinancialInstrumentAttributes11 {
	#[serde(rename = "SctySts", skip_serializing_if = "Option::is_none")]
	pub scty_sts: Option<SecurityStatus3Choice>,
	#[serde(rename = "FinInstrmNm", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_nm: Option<Vec<FinancialInstrumentName2>>,
	#[serde(rename = "DnmtnCcy", skip_serializing_if = "Option::is_none")]
	pub dnmtn_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "CertNb", skip_serializing_if = "Option::is_none")]
	pub cert_nb: Option<Max35Text>,
	#[serde(rename = "CtrctVrsnNb", skip_serializing_if = "Option::is_none")]
	pub ctrct_vrsn_nb: Option<f64>,
	#[serde(rename = "CpnAttchdNb", skip_serializing_if = "Option::is_none")]
	pub cpn_attchd_nb: Option<Max3NumericText>,
	#[serde(rename = "TaxLotNb", skip_serializing_if = "Option::is_none")]
	pub tax_lot_nb: Option<Max15NumericText>,
	#[serde(rename = "PoolNb", skip_serializing_if = "Option::is_none")]
	pub pool_nb: Option<Max15NumericText>,
	#[serde(rename = "CvrdInd", skip_serializing_if = "Option::is_none")]
	pub cvrd_ind: Option<bool>,
	#[serde(rename = "LglRstrctns", skip_serializing_if = "Option::is_none")]
	pub lgl_rstrctns: Option<LegalRestrictions4Choice>,
	#[serde(rename = "PosLmt", skip_serializing_if = "Option::is_none")]
	pub pos_lmt: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "NearTermPosLmt", skip_serializing_if = "Option::is_none")]
	pub near_term_pos_lmt: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "ListgDt", skip_serializing_if = "Option::is_none")]
	pub listg_dt: Option<String>,
	#[serde(rename = "RcrdDt", skip_serializing_if = "Option::is_none")]
	pub rcrd_dt: Option<String>,
	#[serde(rename = "XpryDt", skip_serializing_if = "Option::is_none")]
	pub xpry_dt: Option<String>,
	#[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
	pub purp: Option<Max256Text>,
	#[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
	pub clssfctn_tp: Option<ClassificationType2>,
	#[serde(rename = "Issnc", skip_serializing_if = "Option::is_none")]
	pub issnc: Option<Issuance6>,
	#[serde(rename = "TradgMkt", skip_serializing_if = "Option::is_none")]
	pub tradg_mkt: Option<Vec<TradingParameters2>>,
	#[serde(rename = "SprdAndBchmkCrv", skip_serializing_if = "Option::is_none")]
	pub sprd_and_bchmk_crv: Option<Vec<BenchmarkCurve6>>,
	#[serde(rename = "PutTp", skip_serializing_if = "Option::is_none")]
	pub put_tp: Option<PutType3Choice>,
	#[serde(rename = "CallTp", skip_serializing_if = "Option::is_none")]
	pub call_tp: Option<CallType3Choice>,
	#[serde(rename = "FngbInd", skip_serializing_if = "Option::is_none")]
	pub fngb_ind: Option<bool>,
	#[serde(rename = "Cnfdtl", skip_serializing_if = "Option::is_none")]
	pub cnfdtl: Option<bool>,
	#[serde(rename = "PrvtPlcmnt", skip_serializing_if = "Option::is_none")]
	pub prvt_plcmnt: Option<bool>,
	#[serde(rename = "ConvtblInd", skip_serializing_if = "Option::is_none")]
	pub convtbl_ind: Option<bool>,
	#[serde(rename = "ConvsPrd", skip_serializing_if = "Option::is_none")]
	pub convs_prd: Option<DateTimePeriod1>,
	#[serde(rename = "ConvsRatioNmrtr", skip_serializing_if = "Option::is_none")]
	pub convs_ratio_nmrtr: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "ConvsRatioDnmtr", skip_serializing_if = "Option::is_none")]
	pub convs_ratio_dnmtr: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "PmryPlcOfDpst", skip_serializing_if = "Option::is_none")]
	pub pmry_plc_of_dpst: Option<PartyIdentification136>,
	#[serde(rename = "TradgMtd", skip_serializing_if = "Option::is_none")]
	pub tradg_mtd: Option<UnitOrFaceAmount1Choice>,
	#[serde(rename = "TEFRARule", skip_serializing_if = "Option::is_none")]
	pub tefra_rule: Option<TEFRARules3Choice>,
	#[serde(rename = "SrNb", skip_serializing_if = "Option::is_none")]
	pub sr_nb: Option<Max16Text>,
	#[serde(rename = "Clss", skip_serializing_if = "Option::is_none")]
	pub clss: Option<Max16Text>,
	#[serde(rename = "WhldgTaxRgm", skip_serializing_if = "Option::is_none")]
	pub whldg_tax_rgm: Option<Vec<SecurityWithHoldingTax1>>,
	#[serde(rename = "PmtSts", skip_serializing_if = "Option::is_none")]
	pub pmt_sts: Option<SecuritiesPaymentStatus5Choice>,
	#[serde(rename = "InitlPhysForm", skip_serializing_if = "Option::is_none")]
	pub initl_phys_form: Option<InitialPhysicalForm4Choice>,
	#[serde(rename = "AftrXchgPhysForm", skip_serializing_if = "Option::is_none")]
	pub aftr_xchg_phys_form: Option<InitialPhysicalForm3Choice>,
	#[serde(rename = "CmonSfkpr", skip_serializing_if = "Option::is_none")]
	pub cmon_sfkpr: Option<PartyIdentification177Choice>,
	#[serde(rename = "RedTp", skip_serializing_if = "Option::is_none")]
	pub red_tp: Option<MaturityRedemptionType3Choice>,
	#[serde(rename = "RedPmtCcy", skip_serializing_if = "Option::is_none")]
	pub red_pmt_ccy: Option<ActiveCurrencyCode>,
	#[serde(rename = "Rstrctn", skip_serializing_if = "Option::is_none")]
	pub rstrctn: Option<Vec<SecurityRestriction3>>,
	#[serde(rename = "FinInstrmIdVldty", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_id_vldty: Option<Vec<FinancialInstrumentIdentificationValidity3>>,
	#[serde(rename = "SttlmInf", skip_serializing_if = "Option::is_none")]
	pub sttlm_inf: Option<Vec<SettlementInformation17>>,
	#[serde(rename = "FinInstrmForm", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_form: Option<FinancialInstrumentForm2>,
	#[serde(rename = "CtctNm", skip_serializing_if = "Option::is_none")]
	pub ctct_nm: Option<Organisation38>,
	#[serde(rename = "LeadMgr", skip_serializing_if = "Option::is_none")]
	pub lead_mgr: Option<Organisation38>,
	#[serde(rename = "PrncplPngAgt", skip_serializing_if = "Option::is_none")]
	pub prncpl_png_agt: Option<Organisation38>,
	#[serde(rename = "PngAgt", skip_serializing_if = "Option::is_none")]
	pub png_agt: Option<Organisation38>,
	#[serde(rename = "Dpstry", skip_serializing_if = "Option::is_none")]
	pub dpstry: Option<Organisation38>,
	#[serde(rename = "UndrlygRsk", skip_serializing_if = "Option::is_none")]
	pub undrlyg_rsk: Option<Organisation38>,
	#[serde(rename = "SctyCSDLk", skip_serializing_if = "Option::is_none")]
	pub scty_csd_lk: Option<Vec<SecurityCSDLink7>>,
}

impl CommonFinancialInstrumentAttributes11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref scty_sts_value) = self.scty_sts { if let Err(e) = scty_sts_value.validate() { return Err(e); } }
		if let Some(ref fin_instrm_nm_vec) = self.fin_instrm_nm { for item in fin_instrm_nm_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref dnmtn_ccy_value) = self.dnmtn_ccy { if let Err(e) = dnmtn_ccy_value.validate() { return Err(e); } }
		if let Some(ref cert_nb_value) = self.cert_nb { if let Err(e) = cert_nb_value.validate() { return Err(e); } }
		if let Some(ref cpn_attchd_nb_value) = self.cpn_attchd_nb { if let Err(e) = cpn_attchd_nb_value.validate() { return Err(e); } }
		if let Some(ref tax_lot_nb_value) = self.tax_lot_nb { if let Err(e) = tax_lot_nb_value.validate() { return Err(e); } }
		if let Some(ref pool_nb_value) = self.pool_nb { if let Err(e) = pool_nb_value.validate() { return Err(e); } }
		if let Some(ref lgl_rstrctns_value) = self.lgl_rstrctns { if let Err(e) = lgl_rstrctns_value.validate() { return Err(e); } }
		if let Some(ref pos_lmt_value) = self.pos_lmt { if let Err(e) = pos_lmt_value.validate() { return Err(e); } }
		if let Some(ref near_term_pos_lmt_value) = self.near_term_pos_lmt { if let Err(e) = near_term_pos_lmt_value.validate() { return Err(e); } }
		if let Some(ref purp_value) = self.purp { if let Err(e) = purp_value.validate() { return Err(e); } }
		if let Some(ref clssfctn_tp_value) = self.clssfctn_tp { if let Err(e) = clssfctn_tp_value.validate() { return Err(e); } }
		if let Some(ref issnc_value) = self.issnc { if let Err(e) = issnc_value.validate() { return Err(e); } }
		if let Some(ref tradg_mkt_vec) = self.tradg_mkt { for item in tradg_mkt_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref sprd_and_bchmk_crv_vec) = self.sprd_and_bchmk_crv { for item in sprd_and_bchmk_crv_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref put_tp_value) = self.put_tp { if let Err(e) = put_tp_value.validate() { return Err(e); } }
		if let Some(ref call_tp_value) = self.call_tp { if let Err(e) = call_tp_value.validate() { return Err(e); } }
		if let Some(ref convs_prd_value) = self.convs_prd { if let Err(e) = convs_prd_value.validate() { return Err(e); } }
		if let Some(ref convs_ratio_nmrtr_value) = self.convs_ratio_nmrtr { if let Err(e) = convs_ratio_nmrtr_value.validate() { return Err(e); } }
		if let Some(ref convs_ratio_dnmtr_value) = self.convs_ratio_dnmtr { if let Err(e) = convs_ratio_dnmtr_value.validate() { return Err(e); } }
		if let Some(ref pmry_plc_of_dpst_value) = self.pmry_plc_of_dpst { if let Err(e) = pmry_plc_of_dpst_value.validate() { return Err(e); } }
		if let Some(ref tradg_mtd_value) = self.tradg_mtd { if let Err(e) = tradg_mtd_value.validate() { return Err(e); } }
		if let Some(ref tefra_rule_value) = self.tefra_rule { if let Err(e) = tefra_rule_value.validate() { return Err(e); } }
		if let Some(ref sr_nb_value) = self.sr_nb { if let Err(e) = sr_nb_value.validate() { return Err(e); } }
		if let Some(ref clss_value) = self.clss { if let Err(e) = clss_value.validate() { return Err(e); } }
		if let Some(ref whldg_tax_rgm_vec) = self.whldg_tax_rgm { for item in whldg_tax_rgm_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref pmt_sts_value) = self.pmt_sts { if let Err(e) = pmt_sts_value.validate() { return Err(e); } }
		if let Some(ref initl_phys_form_value) = self.initl_phys_form { if let Err(e) = initl_phys_form_value.validate() { return Err(e); } }
		if let Some(ref aftr_xchg_phys_form_value) = self.aftr_xchg_phys_form { if let Err(e) = aftr_xchg_phys_form_value.validate() { return Err(e); } }
		if let Some(ref cmon_sfkpr_value) = self.cmon_sfkpr { if let Err(e) = cmon_sfkpr_value.validate() { return Err(e); } }
		if let Some(ref red_tp_value) = self.red_tp { if let Err(e) = red_tp_value.validate() { return Err(e); } }
		if let Some(ref red_pmt_ccy_value) = self.red_pmt_ccy { if let Err(e) = red_pmt_ccy_value.validate() { return Err(e); } }
		if let Some(ref rstrctn_vec) = self.rstrctn { for item in rstrctn_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref fin_instrm_id_vldty_vec) = self.fin_instrm_id_vldty { for item in fin_instrm_id_vldty_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref sttlm_inf_vec) = self.sttlm_inf { for item in sttlm_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref fin_instrm_form_value) = self.fin_instrm_form { if let Err(e) = fin_instrm_form_value.validate() { return Err(e); } }
		if let Some(ref ctct_nm_value) = self.ctct_nm { if let Err(e) = ctct_nm_value.validate() { return Err(e); } }
		if let Some(ref lead_mgr_value) = self.lead_mgr { if let Err(e) = lead_mgr_value.validate() { return Err(e); } }
		if let Some(ref prncpl_png_agt_value) = self.prncpl_png_agt { if let Err(e) = prncpl_png_agt_value.validate() { return Err(e); } }
		if let Some(ref png_agt_value) = self.png_agt { if let Err(e) = png_agt_value.validate() { return Err(e); } }
		if let Some(ref dpstry_value) = self.dpstry { if let Err(e) = dpstry_value.validate() { return Err(e); } }
		if let Some(ref undrlyg_rsk_value) = self.undrlyg_rsk { if let Err(e) = undrlyg_rsk_value.validate() { return Err(e); } }
		if let Some(ref scty_csd_lk_vec) = self.scty_csd_lk { for item in scty_csd_lk_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// CommunicationAddress3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommunicationAddress3 {
	#[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
	pub email: Option<Max256Text>,
	#[serde(rename = "Phne", skip_serializing_if = "Option::is_none")]
	pub phne: Option<PhoneNumber>,
	#[serde(rename = "Mob", skip_serializing_if = "Option::is_none")]
	pub mob: Option<PhoneNumber>,
	#[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
	pub fax_nb: Option<PhoneNumber>,
	#[serde(rename = "TlxAdr", skip_serializing_if = "Option::is_none")]
	pub tlx_adr: Option<Max35Text>,
	#[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
	pub url_adr: Option<Max256Text>,
}

impl CommunicationAddress3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref email_value) = self.email { if let Err(e) = email_value.validate() { return Err(e); } }
		if let Some(ref phne_value) = self.phne { if let Err(e) = phne_value.validate() { return Err(e); } }
		if let Some(ref mob_value) = self.mob { if let Err(e) = mob_value.validate() { return Err(e); } }
		if let Some(ref fax_nb_value) = self.fax_nb { if let Err(e) = fax_nb_value.validate() { return Err(e); } }
		if let Some(ref tlx_adr_value) = self.tlx_adr { if let Err(e) = tlx_adr_value.validate() { return Err(e); } }
		if let Some(ref url_adr_value) = self.url_adr { if let Err(e) = url_adr_value.validate() { return Err(e); } }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.country_code) {
			return Err(ValidationError::new(1005, "country_code does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// DateAndDateTime2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTime2Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<String>,
}

impl DateAndDateTime2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DateTimePeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod1 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: String,
}

impl DateTimePeriod1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DateTimePeriod1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod1Choice {
	#[serde(rename = "FrDtTm", skip_serializing_if = "Option::is_none")]
	pub fr_dt_tm: Option<String>,
	#[serde(rename = "ToDtTm", skip_serializing_if = "Option::is_none")]
	pub to_dt_tm: Option<String>,
	#[serde(rename = "DtTmRg", skip_serializing_if = "Option::is_none")]
	pub dt_tm_rg: Option<DateTimePeriod1>,
}

impl DateTimePeriod1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref dt_tm_rg_value) = self.dt_tm_rg { if let Err(e) = dt_tm_rg_value.validate() { return Err(e); } }
		Ok(())
	}
}


// DateTimePeriod2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod2 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm", skip_serializing_if = "Option::is_none")]
	pub to_dt_tm: Option<String>,
}

impl DateTimePeriod2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Debt5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Debt5 {
	#[serde(rename = "PmtCcy", skip_serializing_if = "Option::is_none")]
	pub pmt_ccy: Option<ActiveCurrencyCode>,
	#[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
	pub face_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "PmtFrqcy", skip_serializing_if = "Option::is_none")]
	pub pmt_frqcy: Option<Frequency35Choice>,
	#[serde(rename = "IntrstFxgDt", skip_serializing_if = "Option::is_none")]
	pub intrst_fxg_dt: Option<String>,
	#[serde(rename = "DtdDt", skip_serializing_if = "Option::is_none")]
	pub dtd_dt: Option<String>,
	#[serde(rename = "FrstPmtDt", skip_serializing_if = "Option::is_none")]
	pub frst_pmt_dt: Option<String>,
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "NxtCpnDt", skip_serializing_if = "Option::is_none")]
	pub nxt_cpn_dt: Option<String>,
	#[serde(rename = "PutblDt", skip_serializing_if = "Option::is_none")]
	pub putbl_dt: Option<String>,
	#[serde(rename = "NxtCllblDt", skip_serializing_if = "Option::is_none")]
	pub nxt_cllbl_dt: Option<String>,
	#[serde(rename = "NxtFctrDt", skip_serializing_if = "Option::is_none")]
	pub nxt_fctr_dt: Option<String>,
	#[serde(rename = "XprtnDt", skip_serializing_if = "Option::is_none")]
	pub xprtn_dt: Option<String>,
	#[serde(rename = "PmtDrctnInd", skip_serializing_if = "Option::is_none")]
	pub pmt_drctn_ind: Option<bool>,
	#[serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none")]
	pub intrst_rate: Option<f64>,
	#[serde(rename = "NxtIntrstRate", skip_serializing_if = "Option::is_none")]
	pub nxt_intrst_rate: Option<f64>,
	#[serde(rename = "OddCpnInd", skip_serializing_if = "Option::is_none")]
	pub odd_cpn_ind: Option<bool>,
	#[serde(rename = "CllblInd", skip_serializing_if = "Option::is_none")]
	pub cllbl_ind: Option<bool>,
	#[serde(rename = "CPPrgm", skip_serializing_if = "Option::is_none")]
	pub cp_prgm: Option<f64>,
	#[serde(rename = "CPRegnTp", skip_serializing_if = "Option::is_none")]
	pub cp_regn_tp: Option<Max350Text>,
	#[serde(rename = "IntrstAcrlDt", skip_serializing_if = "Option::is_none")]
	pub intrst_acrl_dt: Option<String>,
	#[serde(rename = "PutblInd", skip_serializing_if = "Option::is_none")]
	pub putbl_ind: Option<bool>,
	#[serde(rename = "PreFnddInd", skip_serializing_if = "Option::is_none")]
	pub pre_fndd_ind: Option<bool>,
	#[serde(rename = "EscrwdInd", skip_serializing_if = "Option::is_none")]
	pub escrwd_ind: Option<bool>,
	#[serde(rename = "PerptlInd", skip_serializing_if = "Option::is_none")]
	pub perptl_ind: Option<bool>,
	#[serde(rename = "SubrdntdInd", skip_serializing_if = "Option::is_none")]
	pub subrdntd_ind: Option<bool>,
	#[serde(rename = "XtndblInd", skip_serializing_if = "Option::is_none")]
	pub xtndbl_ind: Option<bool>,
	#[serde(rename = "XtndblPrd", skip_serializing_if = "Option::is_none")]
	pub xtndbl_prd: Option<DateTimePeriod1Choice>,
	#[serde(rename = "VarblRateInd", skip_serializing_if = "Option::is_none")]
	pub varbl_rate_ind: Option<bool>,
	#[serde(rename = "OverAlltmtAmt", skip_serializing_if = "Option::is_none")]
	pub over_alltmt_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "OverAlltmtRate", skip_serializing_if = "Option::is_none")]
	pub over_alltmt_rate: Option<f64>,
	#[serde(rename = "AmtsblInd", skip_serializing_if = "Option::is_none")]
	pub amtsbl_ind: Option<bool>,
	#[serde(rename = "IntrstClctnMtd", skip_serializing_if = "Option::is_none")]
	pub intrst_clctn_mtd: Option<Max70Text>,
	#[serde(rename = "CptlsdIntrst", skip_serializing_if = "Option::is_none")]
	pub cptlsd_intrst: Option<DistributionPolicy2Choice>,
	#[serde(rename = "ActlDnmtnAmt", skip_serializing_if = "Option::is_none")]
	pub actl_dnmtn_amt: Option<Vec<ActiveCurrencyAndAmount>>,
	#[serde(rename = "CurFctr", skip_serializing_if = "Option::is_none")]
	pub cur_fctr: Option<f64>,
	#[serde(rename = "NxtFctr", skip_serializing_if = "Option::is_none")]
	pub nxt_fctr: Option<f64>,
	#[serde(rename = "PrvsFctr", skip_serializing_if = "Option::is_none")]
	pub prvs_fctr: Option<f64>,
	#[serde(rename = "Pcs", skip_serializing_if = "Option::is_none")]
	pub pcs: Option<f64>,
	#[serde(rename = "PlsMax", skip_serializing_if = "Option::is_none")]
	pub pls_max: Option<f64>,
	#[serde(rename = "PlsPerMln", skip_serializing_if = "Option::is_none")]
	pub pls_per_mln: Option<f64>,
	#[serde(rename = "PlsPerLot", skip_serializing_if = "Option::is_none")]
	pub pls_per_lot: Option<f64>,
	#[serde(rename = "PlsPerTrad", skip_serializing_if = "Option::is_none")]
	pub pls_per_trad: Option<f64>,
	#[serde(rename = "CstPrePmtPnltyInd", skip_serializing_if = "Option::is_none")]
	pub cst_pre_pmt_pnlty_ind: Option<bool>,
	#[serde(rename = "LotId", skip_serializing_if = "Option::is_none")]
	pub lot_id: Option<Max35Text>,
	#[serde(rename = "CstPrePmtYld", skip_serializing_if = "Option::is_none")]
	pub cst_pre_pmt_yld: Option<f64>,
	#[serde(rename = "WghtdAvrgCpn", skip_serializing_if = "Option::is_none")]
	pub wghtd_avrg_cpn: Option<f64>,
	#[serde(rename = "WghtdAvrgLife", skip_serializing_if = "Option::is_none")]
	pub wghtd_avrg_life: Option<f64>,
	#[serde(rename = "WghtdAvrgLn", skip_serializing_if = "Option::is_none")]
	pub wghtd_avrg_ln: Option<f64>,
	#[serde(rename = "WghtdAvrgMtrty", skip_serializing_if = "Option::is_none")]
	pub wghtd_avrg_mtrty: Option<f64>,
	#[serde(rename = "InsrdInd", skip_serializing_if = "Option::is_none")]
	pub insrd_ind: Option<bool>,
	#[serde(rename = "BkQlfdInd", skip_serializing_if = "Option::is_none")]
	pub bk_qlfd_ind: Option<bool>,
	#[serde(rename = "YldClctn", skip_serializing_if = "Option::is_none")]
	pub yld_clctn: Option<Vec<YieldCalculation6>>,
	#[serde(rename = "IntrstTp", skip_serializing_if = "Option::is_none")]
	pub intrst_tp: Option<InterestType3Code>,
	#[serde(rename = "InstrmStrTp", skip_serializing_if = "Option::is_none")]
	pub instrm_str_tp: Option<InstrumentSubStructureType2Choice>,
	#[serde(rename = "GblTp", skip_serializing_if = "Option::is_none")]
	pub gbl_tp: Option<GlobalNote2Choice>,
	#[serde(rename = "PotntlEuroSysElgblty", skip_serializing_if = "Option::is_none")]
	pub potntl_euro_sys_elgblty: Option<bool>,
	#[serde(rename = "Geogcs", skip_serializing_if = "Option::is_none")]
	pub geogcs: Option<Max35Text>,
	#[serde(rename = "YldRg", skip_serializing_if = "Option::is_none")]
	pub yld_rg: Option<AmountOrPercentageRange1>,
	#[serde(rename = "CpnRg", skip_serializing_if = "Option::is_none")]
	pub cpn_rg: Option<AmountOrPercentageRange1>,
	#[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
	pub purp: Option<Max256Text>,
	#[serde(rename = "AltrntvMinTaxInd", skip_serializing_if = "Option::is_none")]
	pub altrntv_min_tax_ind: Option<bool>,
	#[serde(rename = "AutoRinvstmt", skip_serializing_if = "Option::is_none")]
	pub auto_rinvstmt: Option<f64>,
	#[serde(rename = "Hrcut", skip_serializing_if = "Option::is_none")]
	pub hrcut: Option<f64>,
	#[serde(rename = "TxConds", skip_serializing_if = "Option::is_none")]
	pub tx_conds: Option<TradeTransactionCondition7Choice>,
	#[serde(rename = "LookBck", skip_serializing_if = "Option::is_none")]
	pub look_bck: Option<f64>,
	#[serde(rename = "MaxSbstitn", skip_serializing_if = "Option::is_none")]
	pub max_sbstitn: Option<f64>,
	#[serde(rename = "MinIncrmt", skip_serializing_if = "Option::is_none")]
	pub min_incrmt: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "MinQty", skip_serializing_if = "Option::is_none")]
	pub min_qty: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "Pdctn", skip_serializing_if = "Option::is_none")]
	pub pdctn: Option<Max35Text>,
	#[serde(rename = "RstrctdInd", skip_serializing_if = "Option::is_none")]
	pub rstrctd_ind: Option<bool>,
	#[serde(rename = "PricFrqcy", skip_serializing_if = "Option::is_none")]
	pub pric_frqcy: Option<Frequency35Choice>,
	#[serde(rename = "Sctr", skip_serializing_if = "Option::is_none")]
	pub sctr: Option<Max35Text>,
	#[serde(rename = "SbstitnFrqcy", skip_serializing_if = "Option::is_none")]
	pub sbstitn_frqcy: Option<Frequency35Choice>,
	#[serde(rename = "SbstitnLft", skip_serializing_if = "Option::is_none")]
	pub sbstitn_lft: Option<f64>,
	#[serde(rename = "WhlPoolInd", skip_serializing_if = "Option::is_none")]
	pub whl_pool_ind: Option<bool>,
	#[serde(rename = "PricSrc", skip_serializing_if = "Option::is_none")]
	pub pric_src: Option<Max35Text>,
	#[serde(rename = "PricRg", skip_serializing_if = "Option::is_none")]
	pub pric_rg: Option<AmountOrPercentageRange1>,
}

impl Debt5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref pmt_ccy_value) = self.pmt_ccy { if let Err(e) = pmt_ccy_value.validate() { return Err(e); } }
		if let Some(ref face_amt_value) = self.face_amt { if let Err(e) = face_amt_value.validate() { return Err(e); } }
		if let Some(ref pmt_frqcy_value) = self.pmt_frqcy { if let Err(e) = pmt_frqcy_value.validate() { return Err(e); } }
		if let Some(ref cp_regn_tp_value) = self.cp_regn_tp { if let Err(e) = cp_regn_tp_value.validate() { return Err(e); } }
		if let Some(ref xtndbl_prd_value) = self.xtndbl_prd { if let Err(e) = xtndbl_prd_value.validate() { return Err(e); } }
		if let Some(ref over_alltmt_amt_value) = self.over_alltmt_amt { if let Err(e) = over_alltmt_amt_value.validate() { return Err(e); } }
		if let Some(ref intrst_clctn_mtd_value) = self.intrst_clctn_mtd { if let Err(e) = intrst_clctn_mtd_value.validate() { return Err(e); } }
		if let Some(ref cptlsd_intrst_value) = self.cptlsd_intrst { if let Err(e) = cptlsd_intrst_value.validate() { return Err(e); } }
		if let Some(ref actl_dnmtn_amt_vec) = self.actl_dnmtn_amt { for item in actl_dnmtn_amt_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref lot_id_value) = self.lot_id { if let Err(e) = lot_id_value.validate() { return Err(e); } }
		if let Some(ref yld_clctn_vec) = self.yld_clctn { for item in yld_clctn_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref intrst_tp_value) = self.intrst_tp { if let Err(e) = intrst_tp_value.validate() { return Err(e); } }
		if let Some(ref instrm_str_tp_value) = self.instrm_str_tp { if let Err(e) = instrm_str_tp_value.validate() { return Err(e); } }
		if let Some(ref gbl_tp_value) = self.gbl_tp { if let Err(e) = gbl_tp_value.validate() { return Err(e); } }
		if let Some(ref geogcs_value) = self.geogcs { if let Err(e) = geogcs_value.validate() { return Err(e); } }
		if let Some(ref yld_rg_value) = self.yld_rg { if let Err(e) = yld_rg_value.validate() { return Err(e); } }
		if let Some(ref cpn_rg_value) = self.cpn_rg { if let Err(e) = cpn_rg_value.validate() { return Err(e); } }
		if let Some(ref purp_value) = self.purp { if let Err(e) = purp_value.validate() { return Err(e); } }
		if let Some(ref tx_conds_value) = self.tx_conds { if let Err(e) = tx_conds_value.validate() { return Err(e); } }
		if let Some(ref min_incrmt_value) = self.min_incrmt { if let Err(e) = min_incrmt_value.validate() { return Err(e); } }
		if let Some(ref min_qty_value) = self.min_qty { if let Err(e) = min_qty_value.validate() { return Err(e); } }
		if let Some(ref pdctn_value) = self.pdctn { if let Err(e) = pdctn_value.validate() { return Err(e); } }
		if let Some(ref pric_frqcy_value) = self.pric_frqcy { if let Err(e) = pric_frqcy_value.validate() { return Err(e); } }
		if let Some(ref sctr_value) = self.sctr { if let Err(e) = sctr_value.validate() { return Err(e); } }
		if let Some(ref sbstitn_frqcy_value) = self.sbstitn_frqcy { if let Err(e) = sbstitn_frqcy_value.validate() { return Err(e); } }
		if let Some(ref pric_src_value) = self.pric_src { if let Err(e) = pric_src_value.validate() { return Err(e); } }
		if let Some(ref pric_rg_value) = self.pric_rg { if let Err(e) = pric_rg_value.validate() { return Err(e); } }
		Ok(())
	}
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DecimalNumber {
	#[serde(rename = "$value")]
	pub decimal_number: f64,
}

impl DecimalNumber {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Derivative4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Derivative4 {
	#[serde(rename = "Futr", skip_serializing_if = "Option::is_none")]
	pub futr: Option<Future4>,
	#[serde(rename = "Optn", skip_serializing_if = "Option::is_none")]
	pub optn: Option<Option15>,
}

impl Derivative4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref futr_value) = self.futr { if let Err(e) = futr_value.validate() { return Err(e); } }
		if let Some(ref optn_value) = self.optn { if let Err(e) = optn_value.validate() { return Err(e); } }
		Ok(())
	}
}


// DistributionPolicy1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum DistributionPolicy1Code {
	#[default]
	#[serde(rename = "DIST")]
	CodeDIST,
	#[serde(rename = "ACCU")]
	CodeACCU,
}

impl DistributionPolicy1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DistributionPolicy2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DistributionPolicy2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<DistributionPolicy1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl DistributionPolicy2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Equity3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Equity3 {
	#[serde(rename = "PrefToIncm")]
	pub pref_to_incm: PreferenceToIncome5Choice,
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "NonPdAmt", skip_serializing_if = "Option::is_none")]
	pub non_pd_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "ParVal", skip_serializing_if = "Option::is_none")]
	pub par_val: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "VtngRghtsPerShr", skip_serializing_if = "Option::is_none")]
	pub vtng_rghts_per_shr: Option<f64>,
}

impl Equity3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.pref_to_incm.validate() { return Err(e); }
		if let Some(ref non_pd_amt_value) = self.non_pd_amt { if let Err(e) = non_pd_amt_value.validate() { return Err(e); } }
		if let Some(ref par_val_value) = self.par_val { if let Err(e) = par_val_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ErrorHandling3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalSystemErrorHandling1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl ErrorHandling3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ErrorHandling5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling5 {
	#[serde(rename = "Err")]
	pub err: ErrorHandling3Choice,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max140Text>,
}

impl ErrorHandling5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.err.validate() { return Err(e); }
		if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "$value")]
	pub exact4_alpha_numeric_text: String,
}

impl Exact4AlphaNumericText {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
		if !pattern.is_match(&self.exact4_alpha_numeric_text) {
			return Err(ValidationError::new(1005, "exact4_alpha_numeric_text does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// ExternalFinancialInstrumentIdentificationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalFinancialInstrumentIdentificationType1Code {
	#[serde(rename = "$value")]
	pub external_financial_instrument_identification_type1_code: String,
}

impl ExternalFinancialInstrumentIdentificationType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_financial_instrument_identification_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_financial_instrument_identification_type1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_financial_instrument_identification_type1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_financial_instrument_identification_type1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalFinancialInstrumentProductType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalFinancialInstrumentProductType1Code {
	#[serde(rename = "$value")]
	pub external_financial_instrument_product_type1_code: String,
}

impl ExternalFinancialInstrumentProductType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_financial_instrument_product_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_financial_instrument_product_type1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_financial_instrument_product_type1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_financial_instrument_product_type1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalSystemErrorHandling1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalSystemErrorHandling1Code {
	#[serde(rename = "$value")]
	pub external_system_error_handling1_code: String,
}

impl ExternalSystemErrorHandling1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_system_error_handling1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_system_error_handling1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_system_error_handling1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_system_error_handling1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// FinancialInstrument97 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument97 {
	#[serde(rename = "Eqty", skip_serializing_if = "Option::is_none")]
	pub eqty: Option<Equity3>,
	#[serde(rename = "Warrt", skip_serializing_if = "Option::is_none")]
	pub warrt: Option<Warrant4>,
	#[serde(rename = "Debt", skip_serializing_if = "Option::is_none")]
	pub debt: Option<Debt5>,
	#[serde(rename = "Deriv", skip_serializing_if = "Option::is_none")]
	pub deriv: Option<Derivative4>,
}

impl FinancialInstrument97 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref eqty_value) = self.eqty { if let Err(e) = eqty_value.validate() { return Err(e); } }
		if let Some(ref warrt_value) = self.warrt { if let Err(e) = warrt_value.validate() { return Err(e); } }
		if let Some(ref debt_value) = self.debt { if let Err(e) = debt_value.validate() { return Err(e); } }
		if let Some(ref deriv_value) = self.deriv { if let Err(e) = deriv_value.validate() { return Err(e); } }
		Ok(())
	}
}


// FinancialInstrumentForm2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentForm2 {
	#[serde(rename = "BookgApprnc", skip_serializing_if = "Option::is_none")]
	pub bookg_apprnc: Option<Appearance3Choice>,
	#[serde(rename = "LglForm", skip_serializing_if = "Option::is_none")]
	pub lgl_form: Option<FormOfSecurity8Choice>,
}

impl FinancialInstrumentForm2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref bookg_apprnc_value) = self.bookg_apprnc { if let Err(e) = bookg_apprnc_value.validate() { return Err(e); } }
		if let Some(ref lgl_form_value) = self.lgl_form { if let Err(e) = lgl_form_value.validate() { return Err(e); } }
		Ok(())
	}
}


// FinancialInstrumentIdentificationValidity3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentIdentificationValidity3 {
	#[serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_id: Option<SecurityIdentification39>,
	#[serde(rename = "ISINVldFr", skip_serializing_if = "Option::is_none")]
	pub isin_vld_fr: Option<String>,
}

impl FinancialInstrumentIdentificationValidity3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref fin_instrm_id_value) = self.fin_instrm_id { if let Err(e) = fin_instrm_id_value.validate() { return Err(e); } }
		Ok(())
	}
}


// FinancialInstrumentName2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentName2 {
	#[serde(rename = "ISOShrtNm", skip_serializing_if = "Option::is_none")]
	pub iso_shrt_nm: Option<Max35Text>,
	#[serde(rename = "ISOLngNm", skip_serializing_if = "Option::is_none")]
	pub iso_lng_nm: Option<Max350Text>,
	#[serde(rename = "VldFr", skip_serializing_if = "Option::is_none")]
	pub vld_fr: Option<DateAndDateTime2Choice>,
}

impl FinancialInstrumentName2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref iso_shrt_nm_value) = self.iso_shrt_nm { if let Err(e) = iso_shrt_nm_value.validate() { return Err(e); } }
		if let Some(ref iso_lng_nm_value) = self.iso_lng_nm { if let Err(e) = iso_lng_nm_value.validate() { return Err(e); } }
		if let Some(ref vld_fr_value) = self.vld_fr { if let Err(e) = vld_fr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// FinancialInstrumentQuantity1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentQuantity1Choice {
	#[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
	pub unit: Option<f64>,
	#[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
	pub face_amt: Option<f64>,
	#[serde(rename = "AmtsdVal", skip_serializing_if = "Option::is_none")]
	pub amtsd_val: Option<f64>,
}

impl FinancialInstrumentQuantity1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FormOfSecurity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FormOfSecurity1Code {
	#[default]
	#[serde(rename = "BEAR")]
	CodeBEAR,
	#[serde(rename = "REGD")]
	CodeREGD,
}

impl FormOfSecurity1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FormOfSecurity8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FormOfSecurity8Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<FormOfSecurity1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl FormOfSecurity8Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Frequency35Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Frequency35Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Frequency5Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl Frequency35Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Frequency5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Frequency5Code {
	#[default]
	#[serde(rename = "YEAR")]
	CodeYEAR,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "QURT")]
	CodeQURT,
	#[serde(rename = "MIAN")]
	CodeMIAN,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "ADHO")]
	CodeADHO,
	#[serde(rename = "INDA")]
	CodeINDA,
	#[serde(rename = "OVNG")]
	CodeOVNG,
	#[serde(rename = "TEND")]
	CodeTEND,
}

impl Frequency5Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Future4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Future4 {
	#[serde(rename = "CtrctSz", skip_serializing_if = "Option::is_none")]
	pub ctrct_sz: Option<f64>,
	#[serde(rename = "ExrcPric", skip_serializing_if = "Option::is_none")]
	pub exrc_pric: Option<Price8>,
	#[serde(rename = "FutrDt", skip_serializing_if = "Option::is_none")]
	pub futr_dt: Option<String>,
	#[serde(rename = "MinSz", skip_serializing_if = "Option::is_none")]
	pub min_sz: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
	pub unit_of_measr: Option<UnitOfMeasure7Choice>,
	#[serde(rename = "TmUnit", skip_serializing_if = "Option::is_none")]
	pub tm_unit: Option<TimeUnit3Choice>,
	#[serde(rename = "AddtlUndrlygAttrbts", skip_serializing_if = "Option::is_none")]
	pub addtl_undrlyg_attrbts: Option<Vec<UnderlyingAttributes4>>,
}

impl Future4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref exrc_pric_value) = self.exrc_pric { if let Err(e) = exrc_pric_value.validate() { return Err(e); } }
		if let Some(ref min_sz_value) = self.min_sz { if let Err(e) = min_sz_value.validate() { return Err(e); } }
		if let Some(ref unit_of_measr_value) = self.unit_of_measr { if let Err(e) = unit_of_measr_value.validate() { return Err(e); } }
		if let Some(ref tm_unit_value) = self.tm_unit { if let Err(e) = tm_unit_value.validate() { return Err(e); } }
		if let Some(ref addtl_undrlyg_attrbts_vec) = self.addtl_undrlyg_attrbts { for item in addtl_undrlyg_attrbts_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// GenericIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
		if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// GenericIdentification13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification13 {
	#[serde(rename = "Id")]
	pub id: Max4AlphaNumericText,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
	#[serde(rename = "Issr")]
	pub issr: Max35Text,
}

impl GenericIdentification13 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
		if let Err(e) = self.issr.validate() { return Err(e); }
		Ok(())
	}
}


// GenericIdentification30 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification30 {
	#[serde(rename = "Id")]
	pub id: Exact4AlphaNumericText,
	#[serde(rename = "Issr")]
	pub issr: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
}

impl GenericIdentification30 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Err(e) = self.issr.validate() { return Err(e); }
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


// GlobalNote1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum GlobalNote1Code {
	#[default]
	#[serde(rename = "NGNO")]
	CodeNGNO,
	#[serde(rename = "CGNO")]
	CodeCGNO,
}

impl GlobalNote1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// GlobalNote2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GlobalNote2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<GlobalNote1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl GlobalNote2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ISIN2021Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISIN2021Identifier {
	#[serde(rename = "$value")]
	pub isin2021_identifier: String,
}

impl ISIN2021Identifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
		if !pattern.is_match(&self.isin2021_identifier) {
			return Err(ValidationError::new(1005, "isin2021_identifier does not match the required pattern".to_string()));
		}
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ISOYearMonth ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISOYearMonth {
	#[serde(rename = "$value")]
	pub iso_year_month: String,
}

impl ISOYearMonth {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// IdentificationSource3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IdentificationSource3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalFinancialInstrumentIdentificationType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl IdentificationSource3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ImpliedCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ImpliedCurrencyAndAmount {
	#[serde(rename = "$value")]
	pub implied_currency_and_amount: f64,
}

impl ImpliedCurrencyAndAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.implied_currency_and_amount < 0.000000 {
			return Err(ValidationError::new(1003, "implied_currency_and_amount is less than the minimum value of 0.000000".to_string()));
		}
		Ok(())
	}
}


// InitialPhysicalForm1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InitialPhysicalForm1Code {
	#[default]
	#[serde(rename = "GTGT")]
	CodeGTGT,
	#[serde(rename = "GPGP")]
	CodeGPGP,
	#[serde(rename = "DERN")]
	CodeDERN,
}

impl InitialPhysicalForm1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InitialPhysicalForm2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InitialPhysicalForm2Code {
	#[default]
	#[serde(rename = "GPGP")]
	CodeGPGP,
	#[serde(rename = "DERN")]
	CodeDERN,
}

impl InitialPhysicalForm2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InitialPhysicalForm3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InitialPhysicalForm3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InitialPhysicalForm2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl InitialPhysicalForm3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// InitialPhysicalForm4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InitialPhysicalForm4Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InitialPhysicalForm1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl InitialPhysicalForm4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// InstrumentSubStructureType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InstrumentSubStructureType1Code {
	#[default]
	#[serde(rename = "ABSE")]
	CodeABSE,
	#[serde(rename = "AIRT")]
	CodeAIRT,
	#[serde(rename = "AUTT")]
	CodeAUTT,
	#[serde(rename = "CBOB")]
	CodeCBOB,
	#[serde(rename = "CDOB")]
	CodeCDOB,
	#[serde(rename = "CLNO")]
	CodeCLNO,
	#[serde(rename = "CLOB")]
	CodeCLOB,
	#[serde(rename = "CMBS")]
	CodeCMBS,
	#[serde(rename = "CSMR")]
	CodeCSMR,
	#[serde(rename = "CRCT")]
	CodeCRCT,
	#[serde(rename = "HELO")]
	CodeHELO,
	#[serde(rename = "LPNO")]
	CodeLPNO,
	#[serde(rename = "PFAB")]
	CodePFAB,
	#[serde(rename = "PYRT")]
	CodePYRT,
	#[serde(rename = "REPK")]
	CodeREPK,
	#[serde(rename = "RMBS")]
	CodeRMBS,
	#[serde(rename = "SCBO")]
	CodeSCBO,
	#[serde(rename = "STRB")]
	CodeSTRB,
	#[serde(rename = "STUT")]
	CodeSTUT,
	#[serde(rename = "WBSE")]
	CodeWBSE,
}

impl InstrumentSubStructureType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InstrumentSubStructureType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InstrumentSubStructureType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InstrumentSubStructureType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl InstrumentSubStructureType2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// InterestType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InterestType3Code {
	#[default]
	#[serde(rename = "ZCPN")]
	CodeZCPN,
	#[serde(rename = "FIXD")]
	CodeFIXD,
	#[serde(rename = "FLRN")]
	CodeFLRN,
	#[serde(rename = "DUAL")]
	CodeDUAL,
	#[serde(rename = "INDE")]
	CodeINDE,
	#[serde(rename = "DSCO")]
	CodeDSCO,
}

impl InterestType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InvestorRestrictionType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InvestorRestrictionType1Code {
	#[default]
	#[serde(rename = "LERE")]
	CodeLERE,
	#[serde(rename = "CITI")]
	CodeCITI,
	#[serde(rename = "INDV")]
	CodeINDV,
}

impl InvestorRestrictionType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InvestorRestrictionType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestorRestrictionType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InvestorRestrictionType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl InvestorRestrictionType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// InvestorType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InvestorType1Code {
	#[default]
	#[serde(rename = "RETL")]
	CodeRETL,
	#[serde(rename = "PROF")]
	CodePROF,
	#[serde(rename = "STAF")]
	CodeSTAF,
	#[serde(rename = "PPER")]
	CodePPER,
}

impl InvestorType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InvestorType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestorType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InvestorType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl InvestorType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Issuance6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Issuance6 {
	#[serde(rename = "IssePlc", skip_serializing_if = "Option::is_none")]
	pub isse_plc: Option<MICIdentifier>,
	#[serde(rename = "CtryOfIsse", skip_serializing_if = "Option::is_none")]
	pub ctry_of_isse: Option<CountryCode>,
	#[serde(rename = "IsseDt", skip_serializing_if = "Option::is_none")]
	pub isse_dt: Option<String>,
	#[serde(rename = "AnncmntDt", skip_serializing_if = "Option::is_none")]
	pub anncmnt_dt: Option<String>,
	#[serde(rename = "IssrOrg", skip_serializing_if = "Option::is_none")]
	pub issr_org: Option<Organisation38>,
	#[serde(rename = "IsseNmnlAmt", skip_serializing_if = "Option::is_none")]
	pub isse_nmnl_amt: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "FullIssdAmt", skip_serializing_if = "Option::is_none")]
	pub full_issd_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "IsseSz", skip_serializing_if = "Option::is_none")]
	pub isse_sz: Option<f64>,
	#[serde(rename = "IssePric", skip_serializing_if = "Option::is_none")]
	pub isse_pric: Option<PriceValue1>,
	#[serde(rename = "IssncDstrbtn", skip_serializing_if = "Option::is_none")]
	pub issnc_dstrbtn: Option<SecuritiesTransactionType31Choice>,
	#[serde(rename = "GovngLaw", skip_serializing_if = "Option::is_none")]
	pub govng_law: Option<Vec<Jurisdiction1>>,
}

impl Issuance6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref isse_plc_value) = self.isse_plc { if let Err(e) = isse_plc_value.validate() { return Err(e); } }
		if let Some(ref ctry_of_isse_value) = self.ctry_of_isse { if let Err(e) = ctry_of_isse_value.validate() { return Err(e); } }
		if let Some(ref issr_org_value) = self.issr_org { if let Err(e) = issr_org_value.validate() { return Err(e); } }
		if let Some(ref isse_nmnl_amt_value) = self.isse_nmnl_amt { if let Err(e) = isse_nmnl_amt_value.validate() { return Err(e); } }
		if let Some(ref full_issd_amt_value) = self.full_issd_amt { if let Err(e) = full_issd_amt_value.validate() { return Err(e); } }
		if let Some(ref isse_pric_value) = self.isse_pric { if let Err(e) = isse_pric_value.validate() { return Err(e); } }
		if let Some(ref issnc_dstrbtn_value) = self.issnc_dstrbtn { if let Err(e) = issnc_dstrbtn_value.validate() { return Err(e); } }
		if let Some(ref govng_law_vec) = self.govng_law { for item in govng_law_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// IssuanceAccount2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IssuanceAccount2 {
	#[serde(rename = "IssncAcct")]
	pub issnc_acct: SecuritiesAccount19,
	#[serde(rename = "PmryAcctInd")]
	pub pmry_acct_ind: bool,
}

impl IssuanceAccount2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.issnc_acct.validate() { return Err(e); }
		Ok(())
	}
}


// Jurisdiction1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Jurisdiction1 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max70Text>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
}

impl Jurisdiction1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
		if let Some(ref ctry_value) = self.ctry { if let Err(e) = ctry_value.validate() { return Err(e); } }
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


// LegalRestrictions1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum LegalRestrictions1Code {
	#[default]
	#[serde(rename = "USLE")]
	CodeUSLE,
	#[serde(rename = "NORE")]
	CodeNORE,
	#[serde(rename = "REST")]
	CodeREST,
}

impl LegalRestrictions1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// LegalRestrictions2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum LegalRestrictions2Code {
	#[default]
	#[serde(rename = "JURO")]
	CodeJURO,
	#[serde(rename = "PPLA")]
	CodePPLA,
	#[serde(rename = "ACRI")]
	CodeACRI,
	#[serde(rename = "MARG")]
	CodeMARG,
	#[serde(rename = "PRIV")]
	CodePRIV,
}

impl LegalRestrictions2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// LegalRestrictions4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LegalRestrictions4Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<LegalRestrictions1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl LegalRestrictions4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// LegalRestrictions5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LegalRestrictions5Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<LegalRestrictions2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl LegalRestrictions5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
		if !pattern.is_match(&self.mic_identifier) {
			return Err(ValidationError::new(1005, "mic_identifier does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// MaturityRedemptionType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum MaturityRedemptionType1Code {
	#[default]
	#[serde(rename = "FRED")]
	CodeFRED,
	#[serde(rename = "PRNR")]
	CodePRNR,
	#[serde(rename = "PRWR")]
	CodePRWR,
	#[serde(rename = "RNDM")]
	CodeRNDM,
	#[serde(rename = "PRRA")]
	CodePRRA,
	#[serde(rename = "CALL")]
	CodeCALL,
	#[serde(rename = "PUUT")]
	CodePUUT,
}

impl MaturityRedemptionType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// MaturityRedemptionType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MaturityRedemptionType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<MaturityRedemptionType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl MaturityRedemptionType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
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


// Max15NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max15NumericText {
	#[serde(rename = "$value")]
	pub max15_numeric_text: String,
}

impl Max15NumericText {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{1,15}").unwrap();
		if !pattern.is_match(&self.max15_numeric_text) {
			return Err(ValidationError::new(1005, "max15_numeric_text does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max16Text {
	#[serde(rename = "$value")]
	pub max16_text: String,
}

impl Max16Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max16_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max16_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max16_text.chars().count() > 16 {
			return Err(ValidationError::new(1002, "max16_text exceeds the maximum length of 16".to_string()));
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


// Max3NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max3NumericText {
	#[serde(rename = "$value")]
	pub max3_numeric_text: String,
}

impl Max3NumericText {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{1,3}").unwrap();
		if !pattern.is_match(&self.max3_numeric_text) {
			return Err(ValidationError::new(1005, "max3_numeric_text does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// Max4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "$value")]
	pub max4_alpha_numeric_text: String,
}

impl Max4AlphaNumericText {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max4_alpha_numeric_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max4_alpha_numeric_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max4_alpha_numeric_text.chars().count() > 4 {
			return Err(ValidationError::new(1002, "max4_alpha_numeric_text exceeds the maximum length of 4".to_string()));
		}
		let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
		if !pattern.is_match(&self.max4_alpha_numeric_text) {
			return Err(ValidationError::new(1005, "max4_alpha_numeric_text does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// Max5NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max5NumericText {
	#[serde(rename = "$value")]
	pub max5_numeric_text: String,
}

impl Max5NumericText {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{1,5}").unwrap();
		if !pattern.is_match(&self.max5_numeric_text) {
			return Err(ValidationError::new(1005, "max5_numeric_text does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
}

impl Max70Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max70_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max70_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max70_text.chars().count() > 70 {
			return Err(ValidationError::new(1002, "max70_text exceeds the maximum length of 70".to_string()));
		}
		Ok(())
	}
}


// MessageHeader12 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader12 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
	#[serde(rename = "OrgnlBizInstr", skip_serializing_if = "Option::is_none")]
	pub orgnl_biz_instr: Option<OriginalBusinessInstruction1>,
}

impl MessageHeader12 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.msg_id.validate() { return Err(e); }
		if let Some(ref orgnl_biz_instr_value) = self.orgnl_biz_instr { if let Err(e) = orgnl_biz_instr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// NameAndAddress4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress4 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max350Text>,
	#[serde(rename = "Adr")]
	pub adr: PostalAddress1,
}

impl NameAndAddress4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
		if let Err(e) = self.adr.validate() { return Err(e); }
		Ok(())
	}
}


// NameAndAddress5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<PostalAddress1>,
}

impl NameAndAddress5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.nm.validate() { return Err(e); }
		if let Some(ref adr_value) = self.adr { if let Err(e) = adr_value.validate() { return Err(e); } }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Operation1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Operation1Code {
	#[default]
	#[serde(rename = "TILL")]
	CodeTILL,
	#[serde(rename = "ORRR")]
	CodeORRR,
	#[serde(rename = "ANDD")]
	CodeANDD,
}

impl Operation1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Operator1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Operator1Code {
	#[default]
	#[serde(rename = "SMAL")]
	CodeSMAL,
	#[serde(rename = "SMEQ")]
	CodeSMEQ,
	#[serde(rename = "GREA")]
	CodeGREA,
	#[serde(rename = "GREQ")]
	CodeGREQ,
	#[serde(rename = "EQAL")]
	CodeEQAL,
}

impl Operator1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Option15 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Option15 {
	#[serde(rename = "OptnSttlmStyle", skip_serializing_if = "Option::is_none")]
	pub optn_sttlm_style: Option<SettleStyle2Choice>,
	#[serde(rename = "ConvsDt", skip_serializing_if = "Option::is_none")]
	pub convs_dt: Option<String>,
	#[serde(rename = "StrkPric", skip_serializing_if = "Option::is_none")]
	pub strk_pric: Option<Price8>,
	#[serde(rename = "MinExrcblQty", skip_serializing_if = "Option::is_none")]
	pub min_exrcbl_qty: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "ConvsPrd", skip_serializing_if = "Option::is_none")]
	pub convs_prd: Option<DateTimePeriod1Choice>,
	#[serde(rename = "OptnStyle", skip_serializing_if = "Option::is_none")]
	pub optn_style: Option<OptionStyle1Choice>,
	#[serde(rename = "OptnTp", skip_serializing_if = "Option::is_none")]
	pub optn_tp: Option<OptionType8Choice>,
	#[serde(rename = "StrkVal", skip_serializing_if = "Option::is_none")]
	pub strk_val: Option<f64>,
	#[serde(rename = "StrkMltplr", skip_serializing_if = "Option::is_none")]
	pub strk_mltplr: Option<f64>,
	#[serde(rename = "InstrmAssgnmtMtd", skip_serializing_if = "Option::is_none")]
	pub instrm_assgnmt_mtd: Option<AssignmentMethod2Choice>,
	#[serde(rename = "VrsnNb", skip_serializing_if = "Option::is_none")]
	pub vrsn_nb: Option<f64>,
	#[serde(rename = "XpryLctn", skip_serializing_if = "Option::is_none")]
	pub xpry_lctn: Option<Max4AlphaNumericText>,
	#[serde(rename = "Stdstn", skip_serializing_if = "Option::is_none")]
	pub stdstn: Option<Standardisation3Choice>,
	#[serde(rename = "TradgPtyRole", skip_serializing_if = "Option::is_none")]
	pub tradg_pty_role: Option<OptionParty3Choice>,
	#[serde(rename = "CtrctSz", skip_serializing_if = "Option::is_none")]
	pub ctrct_sz: Option<f64>,
	#[serde(rename = "AddtlUndrlygAttrbts", skip_serializing_if = "Option::is_none")]
	pub addtl_undrlyg_attrbts: Option<Vec<UnderlyingAttributes4>>,
}

impl Option15 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref optn_sttlm_style_value) = self.optn_sttlm_style { if let Err(e) = optn_sttlm_style_value.validate() { return Err(e); } }
		if let Some(ref strk_pric_value) = self.strk_pric { if let Err(e) = strk_pric_value.validate() { return Err(e); } }
		if let Some(ref min_exrcbl_qty_value) = self.min_exrcbl_qty { if let Err(e) = min_exrcbl_qty_value.validate() { return Err(e); } }
		if let Some(ref convs_prd_value) = self.convs_prd { if let Err(e) = convs_prd_value.validate() { return Err(e); } }
		if let Some(ref optn_style_value) = self.optn_style { if let Err(e) = optn_style_value.validate() { return Err(e); } }
		if let Some(ref optn_tp_value) = self.optn_tp { if let Err(e) = optn_tp_value.validate() { return Err(e); } }
		if let Some(ref instrm_assgnmt_mtd_value) = self.instrm_assgnmt_mtd { if let Err(e) = instrm_assgnmt_mtd_value.validate() { return Err(e); } }
		if let Some(ref xpry_lctn_value) = self.xpry_lctn { if let Err(e) = xpry_lctn_value.validate() { return Err(e); } }
		if let Some(ref stdstn_value) = self.stdstn { if let Err(e) = stdstn_value.validate() { return Err(e); } }
		if let Some(ref tradg_pty_role_value) = self.tradg_pty_role { if let Err(e) = tradg_pty_role_value.validate() { return Err(e); } }
		if let Some(ref addtl_undrlyg_attrbts_vec) = self.addtl_undrlyg_attrbts { for item in addtl_undrlyg_attrbts_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// OptionParty1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OptionParty1Code {
	#[default]
	#[serde(rename = "SLLR")]
	CodeSLLR,
	#[serde(rename = "BYER")]
	CodeBYER,
}

impl OptionParty1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OptionParty3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OptionParty3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Vec<OptionParty1Code>>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl OptionParty3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_vec) = self.cd { for item in cd_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// OptionStyle1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OptionStyle1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<OptionStyle1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification13>,
}

impl OptionStyle1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// OptionStyle1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OptionStyle1Code {
	#[default]
	#[serde(rename = "AMER")]
	CodeAMER,
	#[serde(rename = "EURO")]
	CodeEURO,
	#[serde(rename = "BERM")]
	CodeBERM,
	#[serde(rename = "ASIA")]
	CodeASIA,
	#[serde(rename = "CANA")]
	CodeCANA,
}

impl OptionStyle1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
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

impl OptionType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OptionType8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OptionType8Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Vec<OptionType1Code>>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl OptionType8Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_vec) = self.cd { for item in cd_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Organisation38 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Organisation38 {
	#[serde(rename = "Nm")]
	pub nm: Max140Text,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<PartyIdentification177Choice>,
	#[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
	pub purp: Option<Max35Text>,
	#[serde(rename = "TaxtnCtry", skip_serializing_if = "Option::is_none")]
	pub taxtn_ctry: Option<CountryCode>,
	#[serde(rename = "RegnCtry", skip_serializing_if = "Option::is_none")]
	pub regn_ctry: Option<CountryCode>,
	#[serde(rename = "RegnDt", skip_serializing_if = "Option::is_none")]
	pub regn_dt: Option<String>,
	#[serde(rename = "TaxIdNb", skip_serializing_if = "Option::is_none")]
	pub tax_id_nb: Option<Max35Text>,
	#[serde(rename = "NtlRegnNb", skip_serializing_if = "Option::is_none")]
	pub ntl_regn_nb: Option<Max35Text>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Vec<PostalAddress3>,
	#[serde(rename = "PmryComAdr", skip_serializing_if = "Option::is_none")]
	pub pmry_com_adr: Option<CommunicationAddress3>,
	#[serde(rename = "ScndryComAdr", skip_serializing_if = "Option::is_none")]
	pub scndry_com_adr: Option<CommunicationAddress3>,
}

impl Organisation38 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.nm.validate() { return Err(e); }
		if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
		if let Some(ref purp_value) = self.purp { if let Err(e) = purp_value.validate() { return Err(e); } }
		if let Some(ref taxtn_ctry_value) = self.taxtn_ctry { if let Err(e) = taxtn_ctry_value.validate() { return Err(e); } }
		if let Some(ref regn_ctry_value) = self.regn_ctry { if let Err(e) = regn_ctry_value.validate() { return Err(e); } }
		if let Some(ref tax_id_nb_value) = self.tax_id_nb { if let Err(e) = tax_id_nb_value.validate() { return Err(e); } }
		if let Some(ref ntl_regn_nb_value) = self.ntl_regn_nb { if let Err(e) = ntl_regn_nb_value.validate() { return Err(e); } }
		for item in &self.pstl_adr { if let Err(e) = item.validate() { return Err(e); } }
		if let Some(ref pmry_com_adr_value) = self.pmry_com_adr { if let Err(e) = pmry_com_adr_value.validate() { return Err(e); } }
		if let Some(ref scndry_com_adr_value) = self.scndry_com_adr { if let Err(e) = scndry_com_adr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// OriginalBusinessInstruction1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OriginalBusinessInstruction1 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none")]
	pub msg_nm_id: Option<Max35Text>,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
}

impl OriginalBusinessInstruction1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.msg_id.validate() { return Err(e); }
		if let Some(ref msg_nm_id_value) = self.msg_nm_id { if let Err(e) = msg_nm_id_value.validate() { return Err(e); } }
		Ok(())
	}
}


// OtherIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Sfx", skip_serializing_if = "Option::is_none")]
	pub sfx: Option<Max16Text>,
	#[serde(rename = "Tp")]
	pub tp: IdentificationSource3Choice,
}

impl OtherIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref sfx_value) = self.sfx { if let Err(e) = sfx_value.validate() { return Err(e); } }
		if let Err(e) = self.tp.validate() { return Err(e); }
		Ok(())
	}
}


// Pagination1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Pagination1 {
	#[serde(rename = "PgNb")]
	pub pg_nb: Max5NumericText,
	#[serde(rename = "LastPgInd")]
	pub last_pg_ind: bool,
}

impl Pagination1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.pg_nb.validate() { return Err(e); }
		Ok(())
	}
}


// PartyIdentification120Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification120Choice {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification36>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
}

impl PartyIdentification120Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
		if let Some(ref prtry_id_value) = self.prtry_id { if let Err(e) = prtry_id_value.validate() { return Err(e); } }
		if let Some(ref nm_and_adr_value) = self.nm_and_adr { if let Err(e) = nm_and_adr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PartyIdentification136 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification136 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification120Choice,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
}

impl PartyIdentification136 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PartyIdentification177Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification177Choice {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification1>,
}

impl PartyIdentification177Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
		if let Some(ref prtry_id_value) = self.prtry_id { if let Err(e) = prtry_id_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PaymentDirectionIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PaymentDirectionIndicator {
	#[serde(rename = "$value")]
	pub payment_direction_indicator: bool,
}

impl PaymentDirectionIndicator {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PercentageRate {
	#[serde(rename = "$value")]
	pub percentage_rate: f64,
}

impl PercentageRate {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PhoneNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PhoneNumber {
	#[serde(rename = "$value")]
	pub phone_number: String,
}

impl PhoneNumber {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
		if !pattern.is_match(&self.phone_number) {
			return Err(ValidationError::new(1005, "phone_number does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// PostalAddress1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress1 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<AddressType2Code>,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
	pub adr_line: Option<Vec<Max70Text>>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<Max70Text>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<Max16Text>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<Max16Text>,
	#[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
	pub twn_nm: Option<Max35Text>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<Max35Text>,
	#[serde(rename = "Ctry")]
	pub ctry: CountryCode,
}

impl PostalAddress1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref adr_tp_value) = self.adr_tp { if let Err(e) = adr_tp_value.validate() { return Err(e); } }
		if let Some(ref adr_line_vec) = self.adr_line { for item in adr_line_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref strt_nm_value) = self.strt_nm { if let Err(e) = strt_nm_value.validate() { return Err(e); } }
		if let Some(ref bldg_nb_value) = self.bldg_nb { if let Err(e) = bldg_nb_value.validate() { return Err(e); } }
		if let Some(ref pst_cd_value) = self.pst_cd { if let Err(e) = pst_cd_value.validate() { return Err(e); } }
		if let Some(ref twn_nm_value) = self.twn_nm { if let Err(e) = twn_nm_value.validate() { return Err(e); } }
		if let Some(ref ctry_sub_dvsn_value) = self.ctry_sub_dvsn { if let Err(e) = ctry_sub_dvsn_value.validate() { return Err(e); } }
		if let Err(e) = self.ctry.validate() { return Err(e); }
		Ok(())
	}
}


// PostalAddress3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress3 {
	#[serde(rename = "AdrTp")]
	pub adr_tp: AddressType1Code,
	#[serde(rename = "MlngInd")]
	pub mlng_ind: bool,
	#[serde(rename = "RegnAdrInd")]
	pub regn_adr_ind: bool,
	#[serde(rename = "NmAndAdr")]
	pub nm_and_adr: NameAndAddress4,
}

impl PostalAddress3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.adr_tp.validate() { return Err(e); }
		if let Err(e) = self.nm_and_adr.validate() { return Err(e); }
		Ok(())
	}
}


// PreferenceToIncome1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PreferenceToIncome1Code {
	#[default]
	#[serde(rename = "ORDN")]
	CodeORDN,
	#[serde(rename = "PFRD")]
	CodePFRD,
}

impl PreferenceToIncome1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PreferenceToIncome5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PreferenceToIncome5Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PreferenceToIncome1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl PreferenceToIncome5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Price8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Price8 {
	#[serde(rename = "ValTp", skip_serializing_if = "Option::is_none")]
	pub val_tp: Option<PriceValueType3Code>,
	#[serde(rename = "Val")]
	pub val: PriceRateOrAmount3Choice,
	#[serde(rename = "PricTp", skip_serializing_if = "Option::is_none")]
	pub pric_tp: Option<TypeOfPrice1Code>,
}

impl Price8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val_tp_value) = self.val_tp { if let Err(e) = val_tp_value.validate() { return Err(e); } }
		if let Err(e) = self.val.validate() { return Err(e); }
		if let Some(ref pric_tp_value) = self.pric_tp { if let Err(e) = pric_tp_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PriceRateOrAmount3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceRateOrAmount3Choice {
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
}

impl PriceRateOrAmount3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PriceValue1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceValue1 {
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAnd13DecimalAmount,
}

impl PriceValue1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.amt.validate() { return Err(e); }
		Ok(())
	}
}


// PriceValueType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PriceValueType3Code {
	#[default]
	#[serde(rename = "DISC")]
	CodeDISC,
	#[serde(rename = "PREM")]
	CodePREM,
	#[serde(rename = "PARV")]
	CodePARV,
	#[serde(rename = "YIEL")]
	CodeYIEL,
	#[serde(rename = "SPRE")]
	CodeSPRE,
	#[serde(rename = "PEUN")]
	CodePEUN,
	#[serde(rename = "ABSO")]
	CodeABSO,
	#[serde(rename = "TEDP")]
	CodeTEDP,
	#[serde(rename = "TEDY")]
	CodeTEDY,
	#[serde(rename = "FICT")]
	CodeFICT,
	#[serde(rename = "VACT")]
	CodeVACT,
}

impl PriceValueType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PutType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PutType1Code {
	#[default]
	#[serde(rename = "MAND")]
	CodeMAND,
	#[serde(rename = "OPTI")]
	CodeOPTI,
	#[serde(rename = "TWOS")]
	CodeTWOS,
}

impl PutType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PutType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PutType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PutType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl PutType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// RateAndAmountFormat1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RateAndAmountFormat1Choice {
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
	pub not_spcfd_rate: Option<RateType12FormatChoice>,
}

impl RateAndAmountFormat1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
		if let Some(ref not_spcfd_rate_value) = self.not_spcfd_rate { if let Err(e) = not_spcfd_rate_value.validate() { return Err(e); } }
		Ok(())
	}
}


// RateOrAbsoluteValue1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RateOrAbsoluteValue1Choice {
	#[serde(rename = "RateVal", skip_serializing_if = "Option::is_none")]
	pub rate_val: Option<f64>,
	#[serde(rename = "AbsVal", skip_serializing_if = "Option::is_none")]
	pub abs_val: Option<f64>,
}

impl RateOrAbsoluteValue1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RateType12Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RateType12Code {
	#[default]
	#[serde(rename = "OPEN")]
	CodeOPEN,
	#[serde(rename = "UKWN")]
	CodeUKWN,
	#[serde(rename = "NILP")]
	CodeNILP,
}

impl RateType12Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RateType12FormatChoice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RateType12FormatChoice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<RateType12Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification13>,
}

impl RateType12FormatChoice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// RestrictionType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RestrictionType1Code {
	#[default]
	#[serde(rename = "SELR")]
	CodeSELR,
	#[serde(rename = "BUYR")]
	CodeBUYR,
	#[serde(rename = "PLAR")]
	CodePLAR,
	#[serde(rename = "HOLR")]
	CodeHOLR,
	#[serde(rename = "VOTR")]
	CodeVOTR,
}

impl RestrictionType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SecuritiesAccount19 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesAccount19 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<GenericIdentification30>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max70Text>,
}

impl SecuritiesAccount19 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SecuritiesPaymentStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SecuritiesPaymentStatus1Code {
	#[default]
	#[serde(rename = "FULL")]
	CodeFULL,
	#[serde(rename = "NILL")]
	CodeNILL,
	#[serde(rename = "PART")]
	CodePART,
}

impl SecuritiesPaymentStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SecuritiesPaymentStatus5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesPaymentStatus5Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<SecuritiesPaymentStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl SecuritiesPaymentStatus5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SecuritiesTransactionType11Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SecuritiesTransactionType11Code {
	#[default]
	#[serde(rename = "NSYN")]
	CodeNSYN,
	#[serde(rename = "SYND")]
	CodeSYND,
}

impl SecuritiesTransactionType11Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SecuritiesTransactionType31Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionType31Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<SecuritiesTransactionType11Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl SecuritiesTransactionType31Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SecurityAttributes11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityAttributes11 {
	#[serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_id: Option<Vec<SecurityIdentification39>>,
	#[serde(rename = "FinInstrmTp", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_tp: Option<FinancialInstrument97>,
	#[serde(rename = "FinInstrmAttrbts", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_attrbts: Option<Vec<CommonFinancialInstrumentAttributes11>>,
}

impl SecurityAttributes11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref fin_instrm_id_vec) = self.fin_instrm_id { for item in fin_instrm_id_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref fin_instrm_tp_value) = self.fin_instrm_tp { if let Err(e) = fin_instrm_tp_value.validate() { return Err(e); } }
		if let Some(ref fin_instrm_attrbts_vec) = self.fin_instrm_attrbts { for item in fin_instrm_attrbts_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// SecurityCSDLink7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityCSDLink7 {
	#[serde(rename = "VldFr")]
	pub vld_fr: DateAndDateTime2Choice,
	#[serde(rename = "VldTo", skip_serializing_if = "Option::is_none")]
	pub vld_to: Option<DateAndDateTime2Choice>,
	#[serde(rename = "SctyMntnc", skip_serializing_if = "Option::is_none")]
	pub scty_mntnc: Option<bool>,
	#[serde(rename = "IssrCSD", skip_serializing_if = "Option::is_none")]
	pub issr_csd: Option<SystemPartyIdentification2Choice>,
	#[serde(rename = "InvstrCSD", skip_serializing_if = "Option::is_none")]
	pub invstr_csd: Option<SystemPartyIdentification2Choice>,
	#[serde(rename = "TechIssrCSD", skip_serializing_if = "Option::is_none")]
	pub tech_issr_csd: Option<SystemPartyIdentification2Choice>,
	#[serde(rename = "IssncAcct", skip_serializing_if = "Option::is_none")]
	pub issnc_acct: Option<Vec<IssuanceAccount2>>,
}

impl SecurityCSDLink7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.vld_fr.validate() { return Err(e); }
		if let Some(ref vld_to_value) = self.vld_to { if let Err(e) = vld_to_value.validate() { return Err(e); } }
		if let Some(ref issr_csd_value) = self.issr_csd { if let Err(e) = issr_csd_value.validate() { return Err(e); } }
		if let Some(ref invstr_csd_value) = self.invstr_csd { if let Err(e) = invstr_csd_value.validate() { return Err(e); } }
		if let Some(ref tech_issr_csd_value) = self.tech_issr_csd { if let Err(e) = tech_issr_csd_value.validate() { return Err(e); } }
		if let Some(ref issnc_acct_vec) = self.issnc_acct { for item in issnc_acct_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// SecurityIdentification39 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification39 {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISIN2021Identifier>,
	#[serde(rename = "OthrId", skip_serializing_if = "Option::is_none")]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max140Text>,
}

impl SecurityIdentification39 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref isin_value) = self.isin { if let Err(e) = isin_value.validate() { return Err(e); } }
		if let Some(ref othr_id_vec) = self.othr_id { for item in othr_id_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SecurityOrBusinessError4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityOrBusinessError4Choice {
	#[serde(rename = "SctyRpt", skip_serializing_if = "Option::is_none")]
	pub scty_rpt: Option<Vec<SecurityAttributes11>>,
	#[serde(rename = "BizErr", skip_serializing_if = "Option::is_none")]
	pub biz_err: Option<Vec<BusinessError4>>,
}

impl SecurityOrBusinessError4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref scty_rpt_vec) = self.scty_rpt { for item in scty_rpt_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref biz_err_vec) = self.biz_err { for item in biz_err_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// SecurityOrOperationalError4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityOrOperationalError4Choice {
	#[serde(rename = "SctyRptOrBizErr", skip_serializing_if = "Option::is_none")]
	pub scty_rpt_or_biz_err: Option<SecurityOrBusinessError4Choice>,
	#[serde(rename = "OprlErr", skip_serializing_if = "Option::is_none")]
	pub oprl_err: Option<Vec<ErrorHandling5>>,
}

impl SecurityOrOperationalError4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref scty_rpt_or_biz_err_value) = self.scty_rpt_or_biz_err { if let Err(e) = scty_rpt_or_biz_err_value.validate() { return Err(e); } }
		if let Some(ref oprl_err_vec) = self.oprl_err { for item in oprl_err_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// SecurityReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityReportV01 {
	#[serde(rename = "MsgHdr", skip_serializing_if = "Option::is_none")]
	pub msg_hdr: Option<MessageHeader12>,
	#[serde(rename = "Pgntn")]
	pub pgntn: Pagination1,
	#[serde(rename = "SctyRptOrErr")]
	pub scty_rpt_or_err: SecurityOrOperationalError4Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl SecurityReportV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref msg_hdr_value) = self.msg_hdr { if let Err(e) = msg_hdr_value.validate() { return Err(e); } }
		if let Err(e) = self.pgntn.validate() { return Err(e); }
		if let Err(e) = self.scty_rpt_or_err.validate() { return Err(e); }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// SecurityRestriction3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityRestriction3 {
	#[serde(rename = "FctvPrd", skip_serializing_if = "Option::is_none")]
	pub fctv_prd: Option<DateTimePeriod2>,
	#[serde(rename = "RstrctnTp", skip_serializing_if = "Option::is_none")]
	pub rstrctn_tp: Option<SecurityRestrictionType2Choice>,
	#[serde(rename = "LglRstrctnTp", skip_serializing_if = "Option::is_none")]
	pub lgl_rstrctn_tp: Option<LegalRestrictions5Choice>,
	#[serde(rename = "InvstrRstrctnTp", skip_serializing_if = "Option::is_none")]
	pub invstr_rstrctn_tp: Option<Vec<InvestorRestrictionType3Choice>>,
	#[serde(rename = "InvstrTp", skip_serializing_if = "Option::is_none")]
	pub invstr_tp: Option<Vec<InvestorType3Choice>>,
}

impl SecurityRestriction3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref fctv_prd_value) = self.fctv_prd { if let Err(e) = fctv_prd_value.validate() { return Err(e); } }
		if let Some(ref rstrctn_tp_value) = self.rstrctn_tp { if let Err(e) = rstrctn_tp_value.validate() { return Err(e); } }
		if let Some(ref lgl_rstrctn_tp_value) = self.lgl_rstrctn_tp { if let Err(e) = lgl_rstrctn_tp_value.validate() { return Err(e); } }
		if let Some(ref invstr_rstrctn_tp_vec) = self.invstr_rstrctn_tp { for item in invstr_rstrctn_tp_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref invstr_tp_vec) = self.invstr_tp { for item in invstr_tp_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// SecurityRestrictionType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityRestrictionType2Choice {
	#[serde(rename = "RstrctnTp", skip_serializing_if = "Option::is_none")]
	pub rstrctn_tp: Option<RestrictionType1Code>,
	#[serde(rename = "PrtryRstrctn", skip_serializing_if = "Option::is_none")]
	pub prtry_rstrctn: Option<GenericIdentification30>,
}

impl SecurityRestrictionType2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref rstrctn_tp_value) = self.rstrctn_tp { if let Err(e) = rstrctn_tp_value.validate() { return Err(e); } }
		if let Some(ref prtry_rstrctn_value) = self.prtry_rstrctn { if let Err(e) = prtry_rstrctn_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SecurityStatus2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SecurityStatus2Code {
	#[default]
	#[serde(rename = "ACTV")]
	CodeACTV,
	#[serde(rename = "INAC")]
	CodeINAC,
	#[serde(rename = "SUSP")]
	CodeSUSP,
}

impl SecurityStatus2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SecurityStatus3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityStatus3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<SecurityStatus2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl SecurityStatus3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SecurityWithHoldingTax1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityWithHoldingTax1 {
	#[serde(rename = "WhldgTaxVal")]
	pub whldg_tax_val: RateAndAmountFormat1Choice,
	#[serde(rename = "Ctry")]
	pub ctry: CountryCode,
}

impl SecurityWithHoldingTax1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.whldg_tax_val.validate() { return Err(e); }
		if let Err(e) = self.ctry.validate() { return Err(e); }
		Ok(())
	}
}


// SettleStyle1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SettleStyle1Code {
	#[default]
	#[serde(rename = "SETC")]
	CodeSETC,
	#[serde(rename = "SETO")]
	CodeSETO,
}

impl SettleStyle1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SettleStyle2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettleStyle2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Vec<SettleStyle1Code>>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl SettleStyle2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_vec) = self.cd { for item in cd_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SettlementInformation17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementInformation17 {
	#[serde(rename = "SctiesQtyTp", skip_serializing_if = "Option::is_none")]
	pub scties_qty_tp: Option<SettlementUnitType3Choice>,
	#[serde(rename = "CtrctSttlmMnth", skip_serializing_if = "Option::is_none")]
	pub ctrct_sttlm_mnth: Option<String>,
	#[serde(rename = "MinDnmtn", skip_serializing_if = "Option::is_none")]
	pub min_dnmtn: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "MinMltplQty", skip_serializing_if = "Option::is_none")]
	pub min_mltpl_qty: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "DevtgSttlmUnit", skip_serializing_if = "Option::is_none")]
	pub devtg_sttlm_unit: Option<Vec<FinancialInstrumentQuantity1Choice>>,
}

impl SettlementInformation17 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref scties_qty_tp_value) = self.scties_qty_tp { if let Err(e) = scties_qty_tp_value.validate() { return Err(e); } }
		if let Some(ref min_dnmtn_value) = self.min_dnmtn { if let Err(e) = min_dnmtn_value.validate() { return Err(e); } }
		if let Some(ref min_mltpl_qty_value) = self.min_mltpl_qty { if let Err(e) = min_mltpl_qty_value.validate() { return Err(e); } }
		if let Some(ref devtg_sttlm_unit_vec) = self.devtg_sttlm_unit { for item in devtg_sttlm_unit_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// SettlementType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SettlementType1Code {
	#[default]
	#[serde(rename = "PRIN")]
	CodePRIN,
	#[serde(rename = "NETO")]
	CodeNETO,
}

impl SettlementType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SettlementType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<SettlementType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl SettlementType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SettlementUnitType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SettlementUnitType1Code {
	#[default]
	#[serde(rename = "FAMT")]
	CodeFAMT,
	#[serde(rename = "UNIT")]
	CodeUNIT,
}

impl SettlementUnitType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SettlementUnitType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementUnitType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<SettlementUnitType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl SettlementUnitType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
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

impl Standardisation1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Standardisation3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Standardisation3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Vec<Standardisation1Code>>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl Standardisation3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_vec) = self.cd { for item in cd_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
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


// SystemPartyIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemPartyIdentification2Choice {
	#[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
	pub org_id: Option<PartyIdentification136>,
	#[serde(rename = "CmbndId", skip_serializing_if = "Option::is_none")]
	pub cmbnd_id: Option<SystemPartyIdentification8>,
}

impl SystemPartyIdentification2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref org_id_value) = self.org_id { if let Err(e) = org_id_value.validate() { return Err(e); } }
		if let Some(ref cmbnd_id_value) = self.cmbnd_id { if let Err(e) = cmbnd_id_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SystemPartyIdentification8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemPartyIdentification8 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification136,
	#[serde(rename = "RspnsblPtyId", skip_serializing_if = "Option::is_none")]
	pub rspnsbl_pty_id: Option<PartyIdentification136>,
}

impl SystemPartyIdentification8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref rspnsbl_pty_id_value) = self.rspnsbl_pty_id { if let Err(e) = rspnsbl_pty_id_value.validate() { return Err(e); } }
		Ok(())
	}
}


// TEFRARules1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TEFRARules1Code {
	#[default]
	#[serde(rename = "RULC")]
	CodeRULC,
	#[serde(rename = "RULD")]
	CodeRULD,
}

impl TEFRARules1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TEFRARules3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TEFRARules3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<TEFRARules1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl TEFRARules3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Term1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Term1 {
	#[serde(rename = "Oprtr")]
	pub oprtr: Operator1Code,
	#[serde(rename = "Val")]
	pub val: RateOrAbsoluteValue1Choice,
}

impl Term1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.oprtr.validate() { return Err(e); }
		if let Err(e) = self.val.validate() { return Err(e); }
		Ok(())
	}
}


// TimeUnit1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TimeUnit1Code {
	#[default]
	#[serde(rename = "DAYC")]
	CodeDAYC,
	#[serde(rename = "HOUR")]
	CodeHOUR,
	#[serde(rename = "MINU")]
	CodeMINU,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "SECO")]
	CodeSECO,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "YEAR")]
	CodeYEAR,
}

impl TimeUnit1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TimeUnit3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeUnit3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<TimeUnit1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl TimeUnit3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// TradeTransactionCondition2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TradeTransactionCondition2Code {
	#[default]
	#[serde(rename = "SPCC")]
	CodeSPCC,
	#[serde(rename = "SECN")]
	CodeSECN,
	#[serde(rename = "SEBN")]
	CodeSEBN,
	#[serde(rename = "SCBN")]
	CodeSCBN,
	#[serde(rename = "SCRT")]
	CodeSCRT,
	#[serde(rename = "SERT")]
	CodeSERT,
	#[serde(rename = "SCCR")]
	CodeSCCR,
	#[serde(rename = "SECR")]
	CodeSECR,
	#[serde(rename = "CAST")]
	CodeCAST,
	#[serde(rename = "SPPR")]
	CodeSPPR,
	#[serde(rename = "SPCU")]
	CodeSPCU,
	#[serde(rename = "SPEX")]
	CodeSPEX,
	#[serde(rename = "GTDL")]
	CodeGTDL,
}

impl TradeTransactionCondition2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TradeTransactionCondition7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeTransactionCondition7Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<TradeTransactionCondition2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl TradeTransactionCondition7Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// TradingParameters2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingParameters2 {
	#[serde(rename = "MktId", skip_serializing_if = "Option::is_none")]
	pub mkt_id: Option<MICIdentifier>,
	#[serde(rename = "RndLot", skip_serializing_if = "Option::is_none")]
	pub rnd_lot: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "TradLotSz", skip_serializing_if = "Option::is_none")]
	pub trad_lot_sz: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "ScndryPlcOfListg", skip_serializing_if = "Option::is_none")]
	pub scndry_plc_of_listg: Option<Vec<MICIdentifier>>,
	#[serde(rename = "MinTraddNmnlQty", skip_serializing_if = "Option::is_none")]
	pub min_tradd_nmnl_qty: Option<UnitOrFaceAmount1Choice>,
	#[serde(rename = "MaxTraddNmnlQty", skip_serializing_if = "Option::is_none")]
	pub max_tradd_nmnl_qty: Option<UnitOrFaceAmount1Choice>,
	#[serde(rename = "MinTradgPricgIncrmt", skip_serializing_if = "Option::is_none")]
	pub min_tradg_pricg_incrmt: Option<f64>,
	#[serde(rename = "PmryPlcOfListgId", skip_serializing_if = "Option::is_none")]
	pub pmry_plc_of_listg_id: Option<MICIdentifier>,
}

impl TradingParameters2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref mkt_id_value) = self.mkt_id { if let Err(e) = mkt_id_value.validate() { return Err(e); } }
		if let Some(ref rnd_lot_value) = self.rnd_lot { if let Err(e) = rnd_lot_value.validate() { return Err(e); } }
		if let Some(ref trad_lot_sz_value) = self.trad_lot_sz { if let Err(e) = trad_lot_sz_value.validate() { return Err(e); } }
		if let Some(ref scndry_plc_of_listg_vec) = self.scndry_plc_of_listg { for item in scndry_plc_of_listg_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref min_tradd_nmnl_qty_value) = self.min_tradd_nmnl_qty { if let Err(e) = min_tradd_nmnl_qty_value.validate() { return Err(e); } }
		if let Some(ref max_tradd_nmnl_qty_value) = self.max_tradd_nmnl_qty { if let Err(e) = max_tradd_nmnl_qty_value.validate() { return Err(e); } }
		if let Some(ref pmry_plc_of_listg_id_value) = self.pmry_plc_of_listg_id { if let Err(e) = pmry_plc_of_listg_id_value.validate() { return Err(e); } }
		Ok(())
	}
}


// TypeOfPrice1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypeOfPrice1Code {
	#[default]
	#[serde(rename = "AVER")]
	CodeAVER,
	#[serde(rename = "AVOV")]
	CodeAVOV,
	#[serde(rename = "COMB")]
	CodeCOMB,
	#[serde(rename = "GREX")]
	CodeGREX,
	#[serde(rename = "LIMI")]
	CodeLIMI,
	#[serde(rename = "NET2")]
	CodeNET2,
	#[serde(rename = "NDIS")]
	CodeNDIS,
	#[serde(rename = "NET1")]
	CodeNET1,
	#[serde(rename = "NUND")]
	CodeNUND,
	#[serde(rename = "NOGR")]
	CodeNOGR,
	#[serde(rename = "PARV")]
	CodePARV,
	#[serde(rename = "RDAV")]
	CodeRDAV,
	#[serde(rename = "STOP")]
	CodeSTOP,
}

impl TypeOfPrice1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// UnderlyingAttributes4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingAttributes4 {
	#[serde(rename = "AllcnPctg", skip_serializing_if = "Option::is_none")]
	pub allcn_pctg: Option<f64>,
	#[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
	pub qty: Option<UnitOrFaceAmount1Choice>,
	#[serde(rename = "SttlmTp", skip_serializing_if = "Option::is_none")]
	pub sttlm_tp: Option<SettlementType3Choice>,
	#[serde(rename = "CshAmt", skip_serializing_if = "Option::is_none")]
	pub csh_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "CshTp", skip_serializing_if = "Option::is_none")]
	pub csh_tp: Option<Max35Text>,
	#[serde(rename = "Pric", skip_serializing_if = "Option::is_none")]
	pub pric: Option<Price8>,
	#[serde(rename = "DrtyPric", skip_serializing_if = "Option::is_none")]
	pub drty_pric: Option<Price8>,
	#[serde(rename = "EndPric", skip_serializing_if = "Option::is_none")]
	pub end_pric: Option<Price8>,
	#[serde(rename = "StartVal", skip_serializing_if = "Option::is_none")]
	pub start_val: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "CurVal", skip_serializing_if = "Option::is_none")]
	pub cur_val: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "EndVal", skip_serializing_if = "Option::is_none")]
	pub end_val: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "AdjstdQty", skip_serializing_if = "Option::is_none")]
	pub adjstd_qty: Option<UnitOrFaceAmount1Choice>,
	#[serde(rename = "XchgRate", skip_serializing_if = "Option::is_none")]
	pub xchg_rate: Option<f64>,
	#[serde(rename = "CapVal", skip_serializing_if = "Option::is_none")]
	pub cap_val: Option<ActiveCurrencyAndAmount>,
}

impl UnderlyingAttributes4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref qty_value) = self.qty { if let Err(e) = qty_value.validate() { return Err(e); } }
		if let Some(ref sttlm_tp_value) = self.sttlm_tp { if let Err(e) = sttlm_tp_value.validate() { return Err(e); } }
		if let Some(ref csh_amt_value) = self.csh_amt { if let Err(e) = csh_amt_value.validate() { return Err(e); } }
		if let Some(ref csh_tp_value) = self.csh_tp { if let Err(e) = csh_tp_value.validate() { return Err(e); } }
		if let Some(ref pric_value) = self.pric { if let Err(e) = pric_value.validate() { return Err(e); } }
		if let Some(ref drty_pric_value) = self.drty_pric { if let Err(e) = drty_pric_value.validate() { return Err(e); } }
		if let Some(ref end_pric_value) = self.end_pric { if let Err(e) = end_pric_value.validate() { return Err(e); } }
		if let Some(ref start_val_value) = self.start_val { if let Err(e) = start_val_value.validate() { return Err(e); } }
		if let Some(ref cur_val_value) = self.cur_val { if let Err(e) = cur_val_value.validate() { return Err(e); } }
		if let Some(ref end_val_value) = self.end_val { if let Err(e) = end_val_value.validate() { return Err(e); } }
		if let Some(ref adjstd_qty_value) = self.adjstd_qty { if let Err(e) = adjstd_qty_value.validate() { return Err(e); } }
		if let Some(ref cap_val_value) = self.cap_val { if let Err(e) = cap_val_value.validate() { return Err(e); } }
		Ok(())
	}
}


// UnitOfMeasure7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitOfMeasure7Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<UnitOfMeasure9Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl UnitOfMeasure7Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// UnitOfMeasure9Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum UnitOfMeasure9Code {
	#[default]
	#[serde(rename = "BAGG")]
	CodeBAGG,
	#[serde(rename = "BALE")]
	CodeBALE,
	#[serde(rename = "BOTL")]
	CodeBOTL,
	#[serde(rename = "BOXX")]
	CodeBOXX,
	#[serde(rename = "CRTN")]
	CodeCRTN,
	#[serde(rename = "CELI")]
	CodeCELI,
	#[serde(rename = "CMET")]
	CodeCMET,
	#[serde(rename = "CNTR")]
	CodeCNTR,
	#[serde(rename = "CRAT")]
	CodeCRAT,
	#[serde(rename = "CBIN")]
	CodeCBIN,
	#[serde(rename = "CBME")]
	CodeCBME,
	#[serde(rename = "CBML")]
	CodeCBML,
	#[serde(rename = "PIEC")]
	CodePIEC,
	#[serde(rename = "FOOT")]
	CodeFOOT,
	#[serde(rename = "GBFO")]
	CodeGBFO,
	#[serde(rename = "GBGA")]
	CodeGBGA,
	#[serde(rename = "GBPI")]
	CodeGBPI,
	#[serde(rename = "GBQA")]
	CodeGBQA,
	#[serde(rename = "GBTN")]
	CodeGBTN,
	#[serde(rename = "GRAM")]
	CodeGRAM,
	#[serde(rename = "INCH")]
	CodeINCH,
	#[serde(rename = "KILO")]
	CodeKILO,
	#[serde(rename = "KMET")]
	CodeKMET,
	#[serde(rename = "LITR")]
	CodeLITR,
	#[serde(rename = "METR")]
	CodeMETR,
	#[serde(rename = "TONE")]
	CodeTONE,
	#[serde(rename = "MILE")]
	CodeMILE,
	#[serde(rename = "MMET")]
	CodeMMET,
	#[serde(rename = "MILI")]
	CodeMILI,
	#[serde(rename = "PUND")]
	CodePUND,
	#[serde(rename = "USOU")]
	CodeUSOU,
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
	#[serde(rename = "USBA")]
	CodeUSBA,
	#[serde(rename = "USFO")]
	CodeUSFO,
	#[serde(rename = "USGA")]
	CodeUSGA,
	#[serde(rename = "USPI")]
	CodeUSPI,
	#[serde(rename = "USQA")]
	CodeUSQA,
	#[serde(rename = "USTN")]
	CodeUSTN,
	#[serde(rename = "YARD")]
	CodeYARD,
	#[serde(rename = "GBOU")]
	CodeGBOU,
	#[serde(rename = "ACRE")]
	CodeACRE,
	#[serde(rename = "ARES")]
	CodeARES,
	#[serde(rename = "HECT")]
	CodeHECT,
}

impl UnitOfMeasure9Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// UnitOrFaceAmount1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitOrFaceAmount1Choice {
	#[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
	pub unit: Option<f64>,
	#[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
	pub face_amt: Option<ActiveCurrencyAndAmount>,
}

impl UnitOrFaceAmount1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref face_amt_value) = self.face_amt { if let Err(e) = face_amt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Warrant4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Warrant4 {
	#[serde(rename = "Mltplr", skip_serializing_if = "Option::is_none")]
	pub mltplr: Option<f64>,
	#[serde(rename = "SbcptPric", skip_serializing_if = "Option::is_none")]
	pub sbcpt_pric: Option<Price8>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<WarrantStyle3Choice>,
	#[serde(rename = "WarrtAgt", skip_serializing_if = "Option::is_none")]
	pub warrt_agt: Option<Vec<Organisation38>>,
}

impl Warrant4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref sbcpt_pric_value) = self.sbcpt_pric { if let Err(e) = sbcpt_pric_value.validate() { return Err(e); } }
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Some(ref warrt_agt_vec) = self.warrt_agt { for item in warrt_agt_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// WarrantStyle1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum WarrantStyle1Code {
	#[default]
	#[serde(rename = "AMER")]
	CodeAMER,
	#[serde(rename = "EURO")]
	CodeEURO,
	#[serde(rename = "BERM")]
	CodeBERM,
}

impl WarrantStyle1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// WarrantStyle3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct WarrantStyle3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<WarrantStyle1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl WarrantStyle3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct YesNoIndicator {
	#[serde(rename = "$value")]
	pub yes_no_indicator: bool,
}

impl YesNoIndicator {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// YieldCalculation6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YieldCalculation6 {
	#[serde(rename = "Val")]
	pub val: f64,
	#[serde(rename = "ClctnTp", skip_serializing_if = "Option::is_none")]
	pub clctn_tp: Option<CalculationType3Choice>,
	#[serde(rename = "RedPric", skip_serializing_if = "Option::is_none")]
	pub red_pric: Option<Price8>,
	#[serde(rename = "ValDt")]
	pub val_dt: String,
	#[serde(rename = "ValPrd")]
	pub val_prd: DateTimePeriod1Choice,
	#[serde(rename = "ClctnDt")]
	pub clctn_dt: String,
}

impl YieldCalculation6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref clctn_tp_value) = self.clctn_tp { if let Err(e) = clctn_tp_value.validate() { return Err(e); } }
		if let Some(ref red_pric_value) = self.red_pric { if let Err(e) = red_pric_value.validate() { return Err(e); } }
		if let Err(e) = self.val_prd.validate() { return Err(e); }
		Ok(())
	}
}
