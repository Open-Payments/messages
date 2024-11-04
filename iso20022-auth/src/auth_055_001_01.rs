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


// Amount3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Amount3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlAmt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgAmt") )]
	pub rptg_amt: ActiveCurrencyAndAmount,
}

impl Amount3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_amt { val.validate()? }
		self.rptg_amt.validate()?;
		Ok(())
	}
}


// AmountAndDirection102 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AmountAndDirection102 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sgn") )]
	pub sgn: bool,
}

impl AmountAndDirection102 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		Ok(())
	}
}


// CCPMemberRequirementsReportV01 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CCPMemberRequirementsReportV01 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntraDayRqrmntAmt") )]
	pub intra_day_rqrmnt_amt: Vec<IntraDayRequirement1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntraDayMrgnCall", skip_serializing_if = "Option::is_none") )]
	pub intra_day_mrgn_call: Option<Vec<IntraDayMarginCall1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndOfDayRqrmnt") )]
	pub end_of_day_rqrmnt: Vec<EndOfDayRequirement2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DfltFndRqrmnt") )]
	pub dflt_fnd_rqrmnt: Vec<DefaultFundRequirement1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CCPMemberRequirementsReportV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.intra_day_rqrmnt_amt { item.validate()? }
		if let Some(ref vec) = self.intra_day_mrgn_call { for item in vec { item.validate()? } }
		for item in &self.end_of_day_rqrmnt { item.validate()? }
		for item in &self.dflt_fnd_rqrmnt { item.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// DefaultFundRequirement1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DefaultFundRequirement1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrMmbId") )]
	pub clr_mmb_id: GenericIdentification165,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SvcId", skip_serializing_if = "Option::is_none") )]
	pub svc_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveCurrencyAndAmount,
}

impl DefaultFundRequirement1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.clr_mmb_id.validate()?;
		if let Some(ref val) = self.svc_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "svc_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "svc_id exceeds the maximum length of 35".to_string()));
			}
		}
		self.amt.validate()?;
		Ok(())
	}
}


// EndOfDayRequirement2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct EndOfDayRequirement2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitlMrgnRqrmnts") )]
	pub initl_mrgn_rqrmnts: InitialMarginRequirement1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VartnMrgnRqrmnts") )]
	pub vartn_mrgn_rqrmnts: AmountAndDirection102,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MrgnAcctId") )]
	pub mrgn_acct_id: GenericIdentification165,
}

impl EndOfDayRequirement2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.initl_mrgn_rqrmnts.validate()?;
		self.vartn_mrgn_rqrmnts.validate()?;
		self.mrgn_acct_id.validate()?;
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


// InitialMarginExposure1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InitialMarginExposure1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: Amount3,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: MarginType2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CoreInd") )]
	pub core_ind: bool,
}

impl InitialMarginExposure1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		self.tp.validate()?;
		Ok(())
	}
}


// InitialMarginRequirement1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InitialMarginRequirement1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitlMrgnXpsr") )]
	pub initl_mrgn_xpsr: Vec<InitialMarginExposure1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdt") )]
	pub cdt: ActiveCurrencyAndAmount,
}

impl InitialMarginRequirement1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.initl_mrgn_xpsr { item.validate()? }
		self.cdt.validate()?;
		Ok(())
	}
}


// IntraDayMarginCall1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct IntraDayMarginCall1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MrgnAcctId") )]
	pub mrgn_acct_id: GenericIdentification165,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntraDayCall") )]
	pub intra_day_call: ActiveCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TmStmp") )]
	pub tm_stmp: String,
}

impl IntraDayMarginCall1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.mrgn_acct_id.validate()?;
		self.intra_day_call.validate()?;
		Ok(())
	}
}


// IntraDayRequirement1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct IntraDayRequirement1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntraDayMrgnCall") )]
	pub intra_day_mrgn_call: ActiveCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PeakInitlMrgnLblty") )]
	pub peak_initl_mrgn_lblty: ActiveCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PeakVartnMrgnLblty") )]
	pub peak_vartn_mrgn_lblty: ActiveCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AggtPeakLblty") )]
	pub aggt_peak_lblty: ActiveCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MrgnAcctId") )]
	pub mrgn_acct_id: GenericIdentification165,
}

impl IntraDayRequirement1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.intra_day_mrgn_call.validate()?;
		self.peak_initl_mrgn_lblty.validate()?;
		self.peak_vartn_mrgn_lblty.validate()?;
		self.aggt_peak_lblty.validate()?;
		self.mrgn_acct_id.validate()?;
		Ok(())
	}
}


// MarginType2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MarginType2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<MarginType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification36>,
}

impl MarginType2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// MarginType2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum MarginType2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ADFM") )]
	CodeADFM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "COMA") )]
	CodeCOMA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CEMA") )]
	CodeCEMA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SEMA") )]
	CodeSEMA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SCMA") )]
	CodeSCMA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UFMA") )]
	CodeUFMA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MARM") )]
	CodeMARM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SORM") )]
	CodeSORM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WWRM") )]
	CodeWWRM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BARM") )]
	CodeBARM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LIRM") )]
	CodeLIRM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRAM") )]
	CodeCRAM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CVMA") )]
	CodeCVMA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SPMA") )]
	CodeSPMA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "JTDR") )]
	CodeJTDR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DRAO") )]
	CodeDRAO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
}

impl MarginType2Code {
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
