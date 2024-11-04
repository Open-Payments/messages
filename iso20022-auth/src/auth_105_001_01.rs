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


// ActiveOrHistoricCurrencyAnd20DecimalAmount ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ActiveOrHistoricCurrencyAnd20DecimalAmount {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
	pub value: f64,
}

impl ActiveOrHistoricCurrencyAnd20DecimalAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ActiveOrHistoricCurrencyAndAmount ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ActiveOrHistoricCurrencyAndAmount {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
	pub value: f64,
}

impl ActiveOrHistoricCurrencyAndAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AmountAndDirection107 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AmountAndDirection107 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAnd20DecimalAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sgn", skip_serializing_if = "Option::is_none") )]
	pub sgn: Option<bool>,
}

impl AmountAndDirection107 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		Ok(())
	}
}


// AmountAndDirection53 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AmountAndDirection53 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sgn", skip_serializing_if = "Option::is_none") )]
	pub sgn: Option<bool>,
}

impl AmountAndDirection53 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		Ok(())
	}
}


// CollateralData33 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CollateralData33 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetXpsrCollstnInd", skip_serializing_if = "Option::is_none") )]
	pub net_xpsr_collstn_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CmpntTp", skip_serializing_if = "Option::is_none") )]
	pub cmpnt_tp: Option<CollateralType6Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshCollCcy", skip_serializing_if = "Option::is_none") )]
	pub csh_coll_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricCcy", skip_serializing_if = "Option::is_none") )]
	pub pric_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Qlty", skip_serializing_if = "Option::is_none") )]
	pub qlty: Option<CollateralQualityType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mtrty", skip_serializing_if = "Option::is_none") )]
	pub mtrty: Option<ContractTerm6Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IssrJursdctn", skip_serializing_if = "Option::is_none") )]
	pub issr_jursdctn: Option<IssuerJurisdiction1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<SecuritiesLendingType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradRpstry", skip_serializing_if = "Option::is_none") )]
	pub trad_rpstry: Option<OrganisationIdentification15Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RcncltnFlg", skip_serializing_if = "Option::is_none") )]
	pub rcncltn_flg: Option<ReconciliationFlag2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RinvstdCsh", skip_serializing_if = "Option::is_none") )]
	pub rinvstd_csh: Option<ReinvestedCashTypeAndAmount2>,
}

impl CollateralData33 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cmpnt_tp { val.validate()? }
		if let Some(ref val) = self.csh_coll_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "csh_coll_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pric_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "pric_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.qlty { val.validate()? }
		if let Some(ref val) = self.mtrty { val.validate()? }
		if let Some(ref val) = self.issr_jursdctn { val.validate()? }
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.trad_rpstry { val.validate()? }
		if let Some(ref val) = self.rcncltn_flg { val.validate()? }
		if let Some(ref val) = self.rinvstd_csh { val.validate()? }
		Ok(())
	}
}


// CollateralQualityType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum CollateralQualityType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "INVG") )]
	CodeINVG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NIVG") )]
	CodeNIVG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NOTR") )]
	CodeNOTR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NOAP") )]
	CodeNOAP,
}

impl CollateralQualityType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CollateralRole1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum CollateralRole1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "GIVE") )]
	CodeGIVE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TAKE") )]
	CodeTAKE,
}

impl CollateralRole1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CollateralType6Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum CollateralType6Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "GBBK") )]
	CodeGBBK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BOND") )]
	CodeBOND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CASH") )]
	CodeCASH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "COMM") )]
	CodeCOMM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INSU") )]
	CodeINSU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LCRE") )]
	CodeLCRE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PHYS") )]
	CodePHYS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SECU") )]
	CodeSECU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "STCF") )]
	CodeSTCF,
}

impl CollateralType6Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ContractTerm6Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ContractTerm6Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Opn", skip_serializing_if = "Option::is_none") )]
	pub opn: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Fxd", skip_serializing_if = "Option::is_none") )]
	pub fxd: Option<TimeToMaturity2Choice>,
}

impl ContractTerm6Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.fxd { val.validate()? }
		Ok(())
	}
}


// CounterpartyData86 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CounterpartyData86 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none") )]
	pub rptg_ctr_pty: Option<CounterpartyIdentification10>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none") )]
	pub othr_ctr_pty: Option<OrganisationIdentification15Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrptyAgt", skip_serializing_if = "Option::is_none") )]
	pub trpty_agt: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AgtLndr", skip_serializing_if = "Option::is_none") )]
	pub agt_lndr: Option<bool>,
}

impl CounterpartyData86 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rptg_ctr_pty { val.validate()? }
		if let Some(ref val) = self.othr_ctr_pty { val.validate()? }
		Ok(())
	}
}


// CounterpartyIdentification10 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CounterpartyIdentification10 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<OrganisationIdentification15Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sd", skip_serializing_if = "Option::is_none") )]
	pub sd: Option<CollateralRole1Code>,
}

impl CounterpartyIdentification10 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id { val.validate()? }
		if let Some(ref val) = self.sd { val.validate()? }
		Ok(())
	}
}


// ExposureMetrics4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ExposureMetrics4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrncplAmt", skip_serializing_if = "Option::is_none") )]
	pub prncpl_amt: Option<PrincipalAmount3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LnVal", skip_serializing_if = "Option::is_none") )]
	pub ln_val: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktVal", skip_serializing_if = "Option::is_none") )]
	pub mkt_val: Option<AmountAndDirection53>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OutsdngMrgnLnAmt", skip_serializing_if = "Option::is_none") )]
	pub outsdng_mrgn_ln_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtMktValAmt", skip_serializing_if = "Option::is_none") )]
	pub shrt_mkt_val_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MrgnLn", skip_serializing_if = "Option::is_none") )]
	pub mrgn_ln: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshCollAmt", skip_serializing_if = "Option::is_none") )]
	pub csh_coll_amt: Option<AmountAndDirection53>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollMktVal", skip_serializing_if = "Option::is_none") )]
	pub coll_mkt_val: Option<AmountAndDirection53>,
}

impl ExposureMetrics4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.prncpl_amt { val.validate()? }
		if let Some(ref val) = self.ln_val { val.validate()? }
		if let Some(ref val) = self.mkt_val { val.validate()? }
		if let Some(ref val) = self.outsdng_mrgn_ln_amt { val.validate()? }
		if let Some(ref val) = self.shrt_mkt_val_amt { val.validate()? }
		if let Some(ref val) = self.mrgn_ln { val.validate()? }
		if let Some(ref val) = self.csh_coll_amt { val.validate()? }
		if let Some(ref val) = self.coll_mkt_val { val.validate()? }
		Ok(())
	}
}


// ExposureMetrics5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ExposureMetrics5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshCollAmt", skip_serializing_if = "Option::is_none") )]
	pub csh_coll_amt: Option<AmountAndDirection53>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollMktVal", skip_serializing_if = "Option::is_none") )]
	pub coll_mkt_val: Option<AmountAndDirection53>,
}

impl ExposureMetrics5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.csh_coll_amt { val.validate()? }
		if let Some(ref val) = self.coll_mkt_val { val.validate()? }
		Ok(())
	}
}


// ExposureMetrics6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ExposureMetrics6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstdMrgnOrColl", skip_serializing_if = "Option::is_none") )]
	pub pstd_mrgn_or_coll: Option<PostedMarginOrCollateral4>,
}

impl ExposureMetrics6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pstd_mrgn_or_coll { val.validate()? }
		Ok(())
	}
}


// ExposureType10Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ExposureType10Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SBSC") )]
	CodeSBSC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MGLD") )]
	CodeMGLD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SLEB") )]
	CodeSLEB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REPO") )]
	CodeREPO,
}

impl ExposureType10Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// GenericIdentification175 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericIdentification175 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericIdentification175 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 72 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 72".to_string()));
		}
		if let Some(ref val) = self.schme_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "schme_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "schme_nm exceeds the maximum length of 35".to_string()));
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
		Ok(())
	}
}


// IssuerJurisdiction1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct IssuerJurisdiction1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryCd", skip_serializing_if = "Option::is_none") )]
	pub ctry_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<String>,
}

impl IssuerJurisdiction1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ctry_cd {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry_cd does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.othr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "othr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "othr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// LoanData134 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct LoanData134 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctTp", skip_serializing_if = "Option::is_none") )]
	pub ctrct_tp: Option<ExposureType10Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Clrd", skip_serializing_if = "Option::is_none") )]
	pub clrd: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtflCd", skip_serializing_if = "Option::is_none") )]
	pub prtfl_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgVn", skip_serializing_if = "Option::is_none") )]
	pub tradg_vn: Option<TradingVenueType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MstrAgrmtTp", skip_serializing_if = "Option::is_none") )]
	pub mstr_agrmt_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none") )]
	pub mtrty_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GnlColl", skip_serializing_if = "Option::is_none") )]
	pub gnl_coll: Option<SpecialCollateral1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Term", skip_serializing_if = "Option::is_none") )]
	pub term: Option<ContractTerm6Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rates", skip_serializing_if = "Option::is_none") )]
	pub rates: Option<Rates1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrncplAmtCcy", skip_serializing_if = "Option::is_none") )]
	pub prncpl_amt_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricCcy", skip_serializing_if = "Option::is_none") )]
	pub pric_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Scty", skip_serializing_if = "Option::is_none") )]
	pub scty: Option<Security49>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OutsdngMrgnLnCcy", skip_serializing_if = "Option::is_none") )]
	pub outsdng_mrgn_ln_ccy: Option<String>,
}

impl LoanData134 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ctrct_tp { val.validate()? }
		if let Some(ref val) = self.prtfl_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtfl_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 52 {
				return Err(ValidationError::new(1002, "prtfl_cd exceeds the maximum length of 52".to_string()));
			}
		}
		if let Some(ref val) = self.tradg_vn { val.validate()? }
		if let Some(ref val) = self.mstr_agrmt_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mstr_agrmt_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "mstr_agrmt_tp exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.gnl_coll { val.validate()? }
		if let Some(ref val) = self.term { val.validate()? }
		if let Some(ref val) = self.rates { val.validate()? }
		if let Some(ref val) = self.prncpl_amt_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "prncpl_amt_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pric_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "pric_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.scty { val.validate()? }
		if let Some(ref val) = self.outsdng_mrgn_ln_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "outsdng_mrgn_ln_ccy does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// MaturityTerm2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MaturityTerm2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Unit") )]
	pub unit: RateBasis1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
	pub val: f64,
}

impl MaturityTerm2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.unit.validate()?;
		Ok(())
	}
}


// NamedPosition3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NamedPosition3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefDt") )]
	pub ref_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GnlInf", skip_serializing_if = "Option::is_none") )]
	pub gnl_inf: Option<Vec<PositionSet16>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ln", skip_serializing_if = "Option::is_none") )]
	pub ln: Option<Vec<PositionSet17>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Coll", skip_serializing_if = "Option::is_none") )]
	pub coll: Option<Vec<PositionSet18>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mrgn", skip_serializing_if = "Option::is_none") )]
	pub mrgn: Option<Vec<PositionSet20>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Reuse", skip_serializing_if = "Option::is_none") )]
	pub reuse: Option<Vec<PositionSet19>>,
}

impl NamedPosition3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.gnl_inf { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.ln { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.coll { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.mrgn { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.reuse { for item in vec { item.validate()? } }
		Ok(())
	}
}


// NoReasonCode ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum NoReasonCode {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NORE") )]
	CodeNORE,
}

impl NoReasonCode {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OrganisationIdentification15Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OrganisationIdentification15Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<OrganisationIdentification38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
	pub any_bic: Option<String>,
}

impl OrganisationIdentification15Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.othr { val.validate()? }
		if let Some(ref val) = self.any_bic {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// OrganisationIdentification38 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OrganisationIdentification38 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: GenericIdentification175,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dmcl", skip_serializing_if = "Option::is_none") )]
	pub dmcl: Option<String>,
}

impl OrganisationIdentification38 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 105 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 105".to_string()));
			}
		}
		if let Some(ref val) = self.dmcl {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dmcl is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 500 {
				return Err(ValidationError::new(1002, "dmcl exceeds the maximum length of 500".to_string()));
			}
		}
		Ok(())
	}
}


// PositionSet16 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PositionSet16 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dmnsns") )]
	pub dmnsns: PositionSetDimensions14,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mtrcs") )]
	pub mtrcs: PositionSetMetrics7,
}

impl PositionSet16 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.dmnsns.validate()?;
		self.mtrcs.validate()?;
		Ok(())
	}
}


// PositionSet17 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PositionSet17 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dmnsns") )]
	pub dmnsns: PositionSetDimensions14,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mtrcs") )]
	pub mtrcs: PositionSetMetrics13,
}

impl PositionSet17 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.dmnsns.validate()?;
		self.mtrcs.validate()?;
		Ok(())
	}
}


// PositionSet18 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PositionSet18 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dmnsns") )]
	pub dmnsns: PositionSetDimensions14,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mtrcs") )]
	pub mtrcs: PositionSetMetrics12,
}

impl PositionSet18 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.dmnsns.validate()?;
		self.mtrcs.validate()?;
		Ok(())
	}
}


// PositionSet19 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PositionSet19 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dmnsns") )]
	pub dmnsns: PositionSetDimensions12,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mtrcs") )]
	pub mtrcs: PositionSetMetrics11,
}

impl PositionSet19 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.dmnsns.validate()?;
		self.mtrcs.validate()?;
		Ok(())
	}
}


// PositionSet20 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PositionSet20 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dmnsns") )]
	pub dmnsns: PositionSetDimensions15,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mtrcs") )]
	pub mtrcs: PositionSetMetrics10,
}

impl PositionSet20 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.dmnsns.validate()?;
		self.mtrcs.validate()?;
		Ok(())
	}
}


// PositionSetDimensions12 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PositionSetDimensions12 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none") )]
	pub rptg_ctr_pty: Option<OrganisationIdentification15Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollData", skip_serializing_if = "Option::is_none") )]
	pub coll_data: Option<CollateralData33>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OtlrsIncl", skip_serializing_if = "Option::is_none") )]
	pub otlrs_incl: Option<bool>,
}

impl PositionSetDimensions12 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rptg_ctr_pty { val.validate()? }
		if let Some(ref val) = self.coll_data { val.validate()? }
		Ok(())
	}
}


// PositionSetDimensions14 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PositionSetDimensions14 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtyData", skip_serializing_if = "Option::is_none") )]
	pub ctr_pty_data: Option<CounterpartyData86>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LnData", skip_serializing_if = "Option::is_none") )]
	pub ln_data: Option<LoanData134>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollData", skip_serializing_if = "Option::is_none") )]
	pub coll_data: Option<CollateralData33>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OtlrsIncl", skip_serializing_if = "Option::is_none") )]
	pub otlrs_incl: Option<bool>,
}

impl PositionSetDimensions14 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ctr_pty_data { val.validate()? }
		if let Some(ref val) = self.ln_data { val.validate()? }
		if let Some(ref val) = self.coll_data { val.validate()? }
		Ok(())
	}
}


// PositionSetDimensions15 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PositionSetDimensions15 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none") )]
	pub rptg_ctr_pty: Option<OrganisationIdentification15Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none") )]
	pub othr_ctr_pty: Option<OrganisationIdentification15Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollPrtflId", skip_serializing_if = "Option::is_none") )]
	pub coll_prtfl_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OtlrsIncl", skip_serializing_if = "Option::is_none") )]
	pub otlrs_incl: Option<bool>,
}

impl PositionSetDimensions15 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rptg_ctr_pty { val.validate()? }
		if let Some(ref val) = self.othr_ctr_pty { val.validate()? }
		if let Some(ref val) = self.coll_prtfl_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "coll_prtfl_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 52 {
				return Err(ValidationError::new(1002, "coll_prtfl_id exceeds the maximum length of 52".to_string()));
			}
		}
		Ok(())
	}
}


// PositionSetMetrics10 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PositionSetMetrics10 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "VolMtrcs", skip_serializing_if = "Option::is_none") )]
	pub vol_mtrcs: Option<ExposureMetrics6>,
}

impl PositionSetMetrics10 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.vol_mtrcs { val.validate()? }
		Ok(())
	}
}


// PositionSetMetrics11 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PositionSetMetrics11 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "VolMtrcs", skip_serializing_if = "Option::is_none") )]
	pub vol_mtrcs: Option<VolumeMetrics4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshRinvstmtRate", skip_serializing_if = "Option::is_none") )]
	pub csh_rinvstmt_rate: Option<f64>,
}

impl PositionSetMetrics11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.vol_mtrcs { val.validate()? }
		Ok(())
	}
}


// PositionSetMetrics12 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PositionSetMetrics12 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "VolMtrcs", skip_serializing_if = "Option::is_none") )]
	pub vol_mtrcs: Option<VolumeMetrics6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HrcutOrMrgn", skip_serializing_if = "Option::is_none") )]
	pub hrcut_or_mrgn: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QtyOrNmnlAmt", skip_serializing_if = "Option::is_none") )]
	pub qty_or_nmnl_amt: Option<QuantityNominalValue2Choice>,
}

impl PositionSetMetrics12 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.vol_mtrcs { val.validate()? }
		if let Some(ref val) = self.qty_or_nmnl_amt { val.validate()? }
		Ok(())
	}
}


// PositionSetMetrics13 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PositionSetMetrics13 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "VolMtrcs") )]
	pub vol_mtrcs: VolumeMetrics5,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PricMtrcs", skip_serializing_if = "Option::is_none") )]
	pub pric_mtrcs: Option<PriceMetrics3>,
}

impl PositionSetMetrics13 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.vol_mtrcs.validate()?;
		if let Some(ref val) = self.pric_mtrcs { val.validate()? }
		Ok(())
	}
}


// PositionSetMetrics7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PositionSetMetrics7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "VolMtrcs") )]
	pub vol_mtrcs: VolumeMetrics5,
}

impl PositionSetMetrics7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.vol_mtrcs.validate()?;
		Ok(())
	}
}


// PositionSetReport3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PositionSetReport3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rpt", skip_serializing_if = "Option::is_none") )]
	pub rpt: Option<NamedPosition3>,
}

impl PositionSetReport3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.data_set_actn { val.validate()? }
		if let Some(ref val) = self.rpt { val.validate()? }
		Ok(())
	}
}


// PostedMarginOrCollateral4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PostedMarginOrCollateral4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitlMrgnPstd", skip_serializing_if = "Option::is_none") )]
	pub initl_mrgn_pstd: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VartnMrgnPstd", skip_serializing_if = "Option::is_none") )]
	pub vartn_mrgn_pstd: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XcssCollPstd", skip_serializing_if = "Option::is_none") )]
	pub xcss_coll_pstd: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl PostedMarginOrCollateral4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.initl_mrgn_pstd { val.validate()? }
		if let Some(ref val) = self.vartn_mrgn_pstd { val.validate()? }
		if let Some(ref val) = self.xcss_coll_pstd { val.validate()? }
		Ok(())
	}
}


// PriceMetrics3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PriceMetrics3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rates", skip_serializing_if = "Option::is_none") )]
	pub rates: Option<Rates3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LndgFee", skip_serializing_if = "Option::is_none") )]
	pub lndg_fee: Option<f64>,
}

impl PriceMetrics3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rates { val.validate()? }
		Ok(())
	}
}


// PriceStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum PriceStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "PNDG") )]
	CodePNDG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NOAP") )]
	CodeNOAP,
}

impl PriceStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PrincipalAmount3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PrincipalAmount3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValDtAmt", skip_serializing_if = "Option::is_none") )]
	pub val_dt_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MtrtyDtAmt", skip_serializing_if = "Option::is_none") )]
	pub mtrty_dt_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl PrincipalAmount3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.val_dt_amt { val.validate()? }
		if let Some(ref val) = self.mtrty_dt_amt { val.validate()? }
		Ok(())
	}
}


// QuantityNominalValue2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct QuantityNominalValue2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Qty", skip_serializing_if = "Option::is_none") )]
	pub qty: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmnlVal", skip_serializing_if = "Option::is_none") )]
	pub nmnl_val: Option<AmountAndDirection53>,
}

impl QuantityNominalValue2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nmnl_val { val.validate()? }
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


// Rates1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Rates1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Fxd", skip_serializing_if = "Option::is_none") )]
	pub fxd: Option<NoReasonCode>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Fltg", skip_serializing_if = "Option::is_none") )]
	pub fltg: Option<String>,
}

impl Rates1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.fxd { val.validate()? }
		if let Some(ref val) = self.fltg {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "fltg is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "fltg exceeds the maximum length of 4".to_string()));
			}
		}
		Ok(())
	}
}


// Rates3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Rates3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Fxd", skip_serializing_if = "Option::is_none") )]
	pub fxd: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Fltg", skip_serializing_if = "Option::is_none") )]
	pub fltg: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BuySellBck", skip_serializing_if = "Option::is_none") )]
	pub buy_sell_bck: Option<SecuritiesTransactionPrice18Choice>,
}

impl Rates3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.buy_sell_bck { val.validate()? }
		Ok(())
	}
}


// ReconciliationFlag2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ReconciliationFlag2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptTp", skip_serializing_if = "Option::is_none") )]
	pub rpt_tp: Option<TradeRepositoryReportingType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BothCtrPtiesRptg", skip_serializing_if = "Option::is_none") )]
	pub both_ctr_pties_rptg: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PairdSts", skip_serializing_if = "Option::is_none") )]
	pub paird_sts: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LnRcncltnSts", skip_serializing_if = "Option::is_none") )]
	pub ln_rcncltn_sts: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollRcncltnSts", skip_serializing_if = "Option::is_none") )]
	pub coll_rcncltn_sts: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModSts", skip_serializing_if = "Option::is_none") )]
	pub mod_sts: Option<bool>,
}

impl ReconciliationFlag2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rpt_tp { val.validate()? }
		Ok(())
	}
}


// ReinvestedCashTypeAndAmount2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ReinvestedCashTypeAndAmount2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: ReinvestmentType1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RinvstdCshCcy") )]
	pub rinvstd_csh_ccy: String,
}

impl ReinvestedCashTypeAndAmount2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tp.validate()?;
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.rinvstd_csh_ccy) {
			return Err(ValidationError::new(1005, "rinvstd_csh_ccy does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// ReinvestmentType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ReinvestmentType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OCMP") )]
	CodeOCMP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MMFT") )]
	CodeMMFT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REPM") )]
	CodeREPM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SDPU") )]
	CodeSDPU,
}

impl ReinvestmentType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ReportPeriodActivity1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ReportPeriodActivity1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NOTX") )]
	CodeNOTX,
}

impl ReportPeriodActivity1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ReuseValue1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ReuseValue1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Actl", skip_serializing_if = "Option::is_none") )]
	pub actl: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Estmtd", skip_serializing_if = "Option::is_none") )]
	pub estmtd: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl ReuseValue1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.actl { val.validate()? }
		if let Some(ref val) = self.estmtd { val.validate()? }
		Ok(())
	}
}


// SecuritiesFinancingReportingPositionSetReportV01 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesFinancingReportingPositionSetReportV01 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AggtdPoss") )]
	pub aggtd_poss: PositionSetReport3Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl SecuritiesFinancingReportingPositionSetReportV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.aggtd_poss.validate()?;
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SecuritiesLendingType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesLendingType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl SecuritiesLendingType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// SecuritiesTransactionPrice18Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesTransactionPrice18Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MntryVal", skip_serializing_if = "Option::is_none") )]
	pub mntry_val: Option<AmountAndDirection107>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pctg", skip_serializing_if = "Option::is_none") )]
	pub pctg: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dcml", skip_serializing_if = "Option::is_none") )]
	pub dcml: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BsisPts", skip_serializing_if = "Option::is_none") )]
	pub bsis_pts: Option<f64>,
}

impl SecuritiesTransactionPrice18Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mntry_val { val.validate()? }
		Ok(())
	}
}


// SecuritiesTransactionPrice19Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesTransactionPrice19Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MntryVal", skip_serializing_if = "Option::is_none") )]
	pub mntry_val: Option<AmountAndDirection107>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Unit", skip_serializing_if = "Option::is_none") )]
	pub unit: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pctg", skip_serializing_if = "Option::is_none") )]
	pub pctg: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Yld", skip_serializing_if = "Option::is_none") )]
	pub yld: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dcml", skip_serializing_if = "Option::is_none") )]
	pub dcml: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PdgPric", skip_serializing_if = "Option::is_none") )]
	pub pdg_pric: Option<PriceStatus1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<SecuritiesTransactionPrice5>,
}

impl SecuritiesTransactionPrice19Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mntry_val { val.validate()? }
		if let Some(ref val) = self.pdg_pric { val.validate()? }
		if let Some(ref val) = self.othr { val.validate()? }
		Ok(())
	}
}


// SecuritiesTransactionPrice5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesTransactionPrice5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val", skip_serializing_if = "Option::is_none") )]
	pub val: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<String>,
}

impl SecuritiesTransactionPrice5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// Security49 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Security49 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none") )]
	pub clssfctn_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QtyOrNmnlVal", skip_serializing_if = "Option::is_none") )]
	pub qty_or_nmnl_val: Option<QuantityNominalValue2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitPric", skip_serializing_if = "Option::is_none") )]
	pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktVal", skip_serializing_if = "Option::is_none") )]
	pub mkt_val: Option<AmountAndDirection53>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Qlty", skip_serializing_if = "Option::is_none") )]
	pub qlty: Option<CollateralQualityType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mtrty", skip_serializing_if = "Option::is_none") )]
	pub mtrty: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<SecurityIssuer4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<Vec<SecuritiesLendingType3Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExclsvArrgmnt", skip_serializing_if = "Option::is_none") )]
	pub exclsv_arrgmnt: Option<bool>,
}

impl Security49 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.clssfctn_tp {
			let pattern = Regex::new("[A-Z]{6,6}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "clssfctn_tp does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.qty_or_nmnl_val { val.validate()? }
		if let Some(ref val) = self.unit_pric { val.validate()? }
		if let Some(ref val) = self.mkt_val { val.validate()? }
		if let Some(ref val) = self.qlty { val.validate()? }
		if let Some(ref val) = self.issr { val.validate()? }
		if let Some(ref vec) = self.tp { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SecurityIssuer4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecurityIssuer4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<OrganisationIdentification15Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "JursdctnCtry") )]
	pub jursdctn_ctry: String,
}

impl SecurityIssuer4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id { val.validate()? }
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.jursdctn_ctry) {
			return Err(ValidationError::new(1005, "jursdctn_ctry does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// SpecialCollateral1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum SpecialCollateral1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "GENE") )]
	CodeGENE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SPEC") )]
	CodeSPEC,
}

impl SpecialCollateral1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SpecialPurpose2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum SpecialPurpose2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "BLNK") )]
	CodeBLNK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NTAV") )]
	CodeNTAV,
}

impl SpecialPurpose2Code {
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


// TimeToMaturity2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TimeToMaturity2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prd", skip_serializing_if = "Option::is_none") )]
	pub prd: Option<TimeToMaturityPeriod2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Spcl", skip_serializing_if = "Option::is_none") )]
	pub spcl: Option<SpecialPurpose2Code>,
}

impl TimeToMaturity2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.prd { val.validate()? }
		if let Some(ref val) = self.spcl { val.validate()? }
		Ok(())
	}
}


// TimeToMaturityPeriod2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TimeToMaturityPeriod2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Start", skip_serializing_if = "Option::is_none") )]
	pub start: Option<MaturityTerm2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "End", skip_serializing_if = "Option::is_none") )]
	pub end: Option<MaturityTerm2>,
}

impl TimeToMaturityPeriod2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.start { val.validate()? }
		if let Some(ref val) = self.end { val.validate()? }
		Ok(())
	}
}


// TradeMarket2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TradeMarket2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DMST") )]
	CodeDMST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FRGN") )]
	CodeFRGN,
}

impl TradeMarket2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TradeRepositoryReportingType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TradeRepositoryReportingType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SWOS") )]
	CodeSWOS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TWOS") )]
	CodeTWOS,
}

impl TradeRepositoryReportingType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TradingVenueType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradingVenueType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OnVn", skip_serializing_if = "Option::is_none") )]
	pub on_vn: Option<TradeMarket2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OffVn", skip_serializing_if = "Option::is_none") )]
	pub off_vn: Option<NoReasonCode>,
}

impl TradingVenueType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.on_vn { val.validate()? }
		if let Some(ref val) = self.off_vn { val.validate()? }
		Ok(())
	}
}


// VolumeMetrics4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct VolumeMetrics4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReuseVal", skip_serializing_if = "Option::is_none") )]
	pub reuse_val: Option<ReuseValue1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RinvstdCshAmt", skip_serializing_if = "Option::is_none") )]
	pub rinvstd_csh_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl VolumeMetrics4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.reuse_val { val.validate()? }
		if let Some(ref val) = self.rinvstd_csh_amt { val.validate()? }
		Ok(())
	}
}


// VolumeMetrics5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct VolumeMetrics5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxs", skip_serializing_if = "Option::is_none") )]
	pub nb_of_txs: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Xpsr", skip_serializing_if = "Option::is_none") )]
	pub xpsr: Option<ExposureMetrics4>,
}

impl VolumeMetrics5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nb_of_txs {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "nb_of_txs does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.xpsr { val.validate()? }
		Ok(())
	}
}


// VolumeMetrics6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct VolumeMetrics6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Postv", skip_serializing_if = "Option::is_none") )]
	pub postv: Option<ExposureMetrics5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Neg", skip_serializing_if = "Option::is_none") )]
	pub neg: Option<ExposureMetrics5>,
}

impl VolumeMetrics6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.postv { val.validate()? }
		if let Some(ref val) = self.neg { val.validate()? }
		Ok(())
	}
}
