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


// CCPAccountPositionReportV01 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CCPAccountPositionReportV01 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtfl") )]
	pub prtfl: Vec<PositionAccount2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CCPAccountPositionReportV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.prtfl { item.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// EndOfDayRequirement1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct EndOfDayRequirement1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitlMrgnRqrmnt", skip_serializing_if = "Option::is_none") )]
	pub initl_mrgn_rqrmnt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VartnMrgnRqrmnt", skip_serializing_if = "Option::is_none") )]
	pub vartn_mrgn_rqrmnt: Option<AmountAndDirection102>,
}

impl EndOfDayRequirement1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.initl_mrgn_rqrmnt { val.validate()? }
		if let Some(ref val) = self.vartn_mrgn_rqrmnt { val.validate()? }
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


// Position1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Position1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PdctId") )]
	pub pdct_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RskRqrmnt", skip_serializing_if = "Option::is_none") )]
	pub rsk_rqrmnt: Option<EndOfDayRequirement1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrssNtnl") )]
	pub grss_ntnl: ActiveCurrencyAnd24Amount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetNtnl") )]
	pub net_ntnl: AmountAndDirection102,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrssDltaEqvtVal", skip_serializing_if = "Option::is_none") )]
	pub grss_dlta_eqvt_val: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetDltaEqvtVal", skip_serializing_if = "Option::is_none") )]
	pub net_dlta_eqvt_val: Option<AmountAndDirection102>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrssDltaEqvtQty", skip_serializing_if = "Option::is_none") )]
	pub grss_dlta_eqvt_qty: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetDltaEqvtQty", skip_serializing_if = "Option::is_none") )]
	pub net_dlta_eqvt_qty: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrssMktVal") )]
	pub grss_mkt_val: ActiveCurrencyAndAmount,
}

impl Position1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.pdct_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "pdct_id is shorter than the minimum length of 1".to_string()));
		}
		if self.pdct_id.chars().count() > 256 {
			return Err(ValidationError::new(1002, "pdct_id exceeds the maximum length of 256".to_string()));
		}
		if let Some(ref val) = self.rsk_rqrmnt { val.validate()? }
		self.grss_ntnl.validate()?;
		self.net_ntnl.validate()?;
		if let Some(ref val) = self.grss_dlta_eqvt_val { val.validate()? }
		if let Some(ref val) = self.net_dlta_eqvt_val { val.validate()? }
		self.grss_mkt_val.validate()?;
		Ok(())
	}
}


// PositionAccount2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PositionAccount2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: GenericIdentification165,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pos") )]
	pub pos: Vec<Position1>,
}

impl PositionAccount2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		for item in &self.pos { item.validate()? }
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
