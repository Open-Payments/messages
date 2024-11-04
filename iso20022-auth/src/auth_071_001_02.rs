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


// CashReuseData1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CashReuseData1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RinvstdCsh") )]
	pub rinvstd_csh: Vec<ReinvestedCashTypeAndAmount1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshRinvstmtRate") )]
	pub csh_rinvstmt_rate: f64,
}

impl CashReuseData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.rinvstd_csh { item.validate()? }
		Ok(())
	}
}


// CollateralType19 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CollateralType19 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Scty", skip_serializing_if = "Option::is_none") )]
	pub scty: Option<Vec<SecurityReuseData1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Csh", skip_serializing_if = "Option::is_none") )]
	pub csh: Option<Vec<CashReuseData1>>,
}

impl CollateralType19 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.scty { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.csh { for item in vec { item.validate()? } }
		Ok(())
	}
}


// CounterpartyData87 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CounterpartyData87 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptSubmitgNtty") )]
	pub rpt_submitg_ntty: OrganisationIdentification15Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgCtrPty") )]
	pub rptg_ctr_pty: OrganisationIdentification15Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none") )]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
}

impl CounterpartyData87 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rpt_submitg_ntty.validate()?;
		self.rptg_ctr_pty.validate()?;
		if let Some(ref val) = self.ntty_rspnsbl_for_rpt { val.validate()? }
		Ok(())
	}
}


// FundingSource3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FundingSource3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: FundingSourceType1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktVal") )]
	pub mkt_val: AmountAndDirection53,
}

impl FundingSource3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tp.validate()?;
		self.mkt_val.validate()?;
		Ok(())
	}
}


// FundingSourceType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum FundingSourceType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SECL") )]
	CodeSECL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FREE") )]
	CodeFREE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BSHS") )]
	CodeBSHS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CSHS") )]
	CodeCSHS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REPO") )]
	CodeREPO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UBOR") )]
	CodeUBOR,
}

impl FundingSourceType1Code {
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


// ReinvestedCashTypeAndAmount1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ReinvestedCashTypeAndAmount1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: ReinvestmentType1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RinvstdCshAmt") )]
	pub rinvstd_csh_amt: ActiveOrHistoricCurrencyAndAmount,
}

impl ReinvestedCashTypeAndAmount1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tp.validate()?;
		self.rinvstd_csh_amt.validate()?;
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


// ReuseDataReport6Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ReuseDataReport6Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "New", skip_serializing_if = "Option::is_none") )]
	pub new: Option<ReuseDataReportNew6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Err", skip_serializing_if = "Option::is_none") )]
	pub err: Option<ReuseDataReportError5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Crrctn", skip_serializing_if = "Option::is_none") )]
	pub crrctn: Option<ReuseDataReportCorrection14>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollReuseUpd", skip_serializing_if = "Option::is_none") )]
	pub coll_reuse_upd: Option<ReuseDataReportCorrection14>,
}

impl ReuseDataReport6Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.new { val.validate()? }
		if let Some(ref val) = self.err { val.validate()? }
		if let Some(ref val) = self.crrctn { val.validate()? }
		if let Some(ref val) = self.coll_reuse_upd { val.validate()? }
		Ok(())
	}
}


// ReuseDataReportCorrection14 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ReuseDataReportCorrection14 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none") )]
	pub tech_rcrd_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgDtTm") )]
	pub rptg_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPty") )]
	pub ctr_pty: CounterpartyData87,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollCmpnt", skip_serializing_if = "Option::is_none") )]
	pub coll_cmpnt: Option<Vec<CollateralType19>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EvtDay") )]
	pub evt_day: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FndgSrc", skip_serializing_if = "Option::is_none") )]
	pub fndg_src: Option<Vec<FundingSource3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl ReuseDataReportCorrection14 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tech_rcrd_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tech_rcrd_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "tech_rcrd_id exceeds the maximum length of 140".to_string()));
			}
		}
		self.ctr_pty.validate()?;
		if let Some(ref vec) = self.coll_cmpnt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.fndg_src { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// ReuseDataReportError5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ReuseDataReportError5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none") )]
	pub tech_rcrd_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgDtTm") )]
	pub rptg_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPty") )]
	pub ctr_pty: CounterpartyData87,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl ReuseDataReportError5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tech_rcrd_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tech_rcrd_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "tech_rcrd_id exceeds the maximum length of 140".to_string()));
			}
		}
		self.ctr_pty.validate()?;
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// ReuseDataReportNew6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ReuseDataReportNew6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none") )]
	pub tech_rcrd_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgDtTm") )]
	pub rptg_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPty") )]
	pub ctr_pty: CounterpartyData87,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollCmpnt", skip_serializing_if = "Option::is_none") )]
	pub coll_cmpnt: Option<Vec<CollateralType19>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EvtDay") )]
	pub evt_day: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FndgSrc", skip_serializing_if = "Option::is_none") )]
	pub fndg_src: Option<Vec<FundingSource3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl ReuseDataReportNew6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tech_rcrd_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tech_rcrd_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "tech_rcrd_id exceeds the maximum length of 140".to_string()));
			}
		}
		self.ctr_pty.validate()?;
		if let Some(ref vec) = self.coll_cmpnt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.fndg_src { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
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


// SecuritiesFinancingReportingTransactionReusedCollateralDataReportV02 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesFinancingReportingTransactionReusedCollateralDataReportV02 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradData") )]
	pub trad_data: TradeData36Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl SecuritiesFinancingReportingTransactionReusedCollateralDataReportV02 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.trad_data.validate()?;
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SecurityReuseData1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecurityReuseData1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN") )]
	pub isin: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReuseVal") )]
	pub reuse_val: ReuseValue1Choice,
}

impl SecurityReuseData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
		if !pattern.is_match(&self.isin) {
			return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
		}
		self.reuse_val.validate()?;
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


// TradeData36Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradeData36Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rpt", skip_serializing_if = "Option::is_none") )]
	pub rpt: Option<Vec<ReuseDataReport6Choice>>,
}

impl TradeData36Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.data_set_actn { val.validate()? }
		if let Some(ref vec) = self.rpt { for item in vec { item.validate()? } }
		Ok(())
	}
}
