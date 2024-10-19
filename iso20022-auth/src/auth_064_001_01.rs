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


// AvailableFinancialResourcesAmount1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AvailableFinancialResourcesAmount1 {
	#[serde(rename = "TtlInitlMrgn")]
	pub ttl_initl_mrgn: ActiveCurrencyAndAmount,
	#[serde(rename = "TtlPrfnddDfltFnd")]
	pub ttl_prfndd_dflt_fnd: ActiveCurrencyAndAmount,
	#[serde(rename = "CCPSkinInTheGame")]
	pub ccp_skin_in_the_game: Vec<ReportingAssetBreakdown1>,
	#[serde(rename = "OthrDfltFndCntrbtn")]
	pub othr_dflt_fnd_cntrbtn: ActiveCurrencyAndAmount,
	#[serde(rename = "UfnddMmbCmmtmnt")]
	pub ufndd_mmb_cmmtmnt: ActiveCurrencyAndAmount,
	#[serde(rename = "UfnddThrdPtyCmmtmnt")]
	pub ufndd_thrd_pty_cmmtmnt: ActiveCurrencyAndAmount,
}

impl AvailableFinancialResourcesAmount1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.ttl_initl_mrgn.validate() { return Err(e); }
		if let Err(e) = self.ttl_prfndd_dflt_fnd.validate() { return Err(e); }
		for item in &self.ccp_skin_in_the_game { if let Err(e) = item.validate() { return Err(e); } }
		if let Err(e) = self.othr_dflt_fnd_cntrbtn.validate() { return Err(e); }
		if let Err(e) = self.ufndd_mmb_cmmtmnt.validate() { return Err(e); }
		if let Err(e) = self.ufndd_thrd_pty_cmmtmnt.validate() { return Err(e); }
		Ok(())
	}
}


// CCPAvailableFinancialResourcesReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CCPAvailableFinancialResourcesReportV01 {
	#[serde(rename = "AvlblFinRsrcsAmt")]
	pub avlbl_fin_rsrcs_amt: AvailableFinancialResourcesAmount1,
	#[serde(rename = "OthrPrfnddRsrcs", skip_serializing_if = "Option::is_none")]
	pub othr_prfndd_rsrcs: Option<ReportingAssetBreakdown1>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CCPAvailableFinancialResourcesReportV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.avlbl_fin_rsrcs_amt.validate() { return Err(e); }
		if let Some(ref othr_prfndd_rsrcs_value) = self.othr_prfndd_rsrcs { if let Err(e) = othr_prfndd_rsrcs_value.validate() { return Err(e); } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
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


// ProductType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ProductType6Code {
	#[default]
	#[serde(rename = "BOND")]
	CodeBOND,
	#[serde(rename = "CASH")]
	CodeCASH,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "EQUI")]
	CodeEQUI,
}

impl ProductType6Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ReportingAssetBreakdown1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportingAssetBreakdown1 {
	#[serde(rename = "RptgAsstTp")]
	pub rptg_asst_tp: ProductType6Code,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max350Text>,
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
}

impl ReportingAssetBreakdown1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.rptg_asst_tp.validate() { return Err(e); }
		if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
		if let Err(e) = self.amt.validate() { return Err(e); }
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
