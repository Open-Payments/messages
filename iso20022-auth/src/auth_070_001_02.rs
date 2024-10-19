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
// ActiveOrHistoricCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_and_amount_simple_type: f64,
}

impl ActiveOrHistoricCurrencyAndAmountSimpleType {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.active_or_historic_currency_and_amount_simple_type < 0.000000 {
			return Err(ValidationError::new(1003, "active_or_historic_currency_and_amount_simple_type is less than the minimum value of 0.000000".to_string()));
		}
		Ok(())
	}
}


// ActiveOrHistoricCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveOrHistoricCurrencyAndAmount {
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


// CollateralMarginCorrection6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralMarginCorrection6 {
	#[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
	pub tech_rcrd_id: Option<Max140Text>,
	#[serde(rename = "RptgDtTm")]
	pub rptg_dt_tm: String,
	#[serde(rename = "EvtDt")]
	pub evt_dt: String,
	#[serde(rename = "CtrPty")]
	pub ctr_pty: Counterparty39,
	#[serde(rename = "CollPrtflId")]
	pub coll_prtfl_id: Max52Text,
	#[serde(rename = "PstdMrgnOrColl", skip_serializing_if = "Option::is_none")]
	pub pstd_mrgn_or_coll: Option<PostedMarginOrCollateral4>,
	#[serde(rename = "RcvdMrgnOrColl", skip_serializing_if = "Option::is_none")]
	pub rcvd_mrgn_or_coll: Option<ReceivedMarginOrCollateral4>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CollateralMarginCorrection6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref tech_rcrd_id_value) = self.tech_rcrd_id { if let Err(e) = tech_rcrd_id_value.validate() { return Err(e); } }
		if let Err(e) = self.ctr_pty.validate() { return Err(e); }
		if let Err(e) = self.coll_prtfl_id.validate() { return Err(e); }
		if let Some(ref pstd_mrgn_or_coll_value) = self.pstd_mrgn_or_coll { if let Err(e) = pstd_mrgn_or_coll_value.validate() { return Err(e); } }
		if let Some(ref rcvd_mrgn_or_coll_value) = self.rcvd_mrgn_or_coll { if let Err(e) = rcvd_mrgn_or_coll_value.validate() { return Err(e); } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// CollateralMarginError4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralMarginError4 {
	#[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
	pub tech_rcrd_id: Option<Max140Text>,
	#[serde(rename = "RptgDtTm")]
	pub rptg_dt_tm: String,
	#[serde(rename = "CtrPty")]
	pub ctr_pty: Counterparty39,
	#[serde(rename = "CollPrtflId")]
	pub coll_prtfl_id: Max52Text,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CollateralMarginError4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref tech_rcrd_id_value) = self.tech_rcrd_id { if let Err(e) = tech_rcrd_id_value.validate() { return Err(e); } }
		if let Err(e) = self.ctr_pty.validate() { return Err(e); }
		if let Err(e) = self.coll_prtfl_id.validate() { return Err(e); }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// CollateralMarginMarginUpdate5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralMarginMarginUpdate5 {
	#[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
	pub tech_rcrd_id: Option<Max140Text>,
	#[serde(rename = "RptgDtTm")]
	pub rptg_dt_tm: String,
	#[serde(rename = "EvtDt")]
	pub evt_dt: String,
	#[serde(rename = "CtrPty", skip_serializing_if = "Option::is_none")]
	pub ctr_pty: Option<Counterparty39>,
	#[serde(rename = "CollPrtflId")]
	pub coll_prtfl_id: Max52Text,
	#[serde(rename = "PstdMrgnOrColl", skip_serializing_if = "Option::is_none")]
	pub pstd_mrgn_or_coll: Option<PostedMarginOrCollateral4>,
	#[serde(rename = "RcvdMrgnOrColl", skip_serializing_if = "Option::is_none")]
	pub rcvd_mrgn_or_coll: Option<ReceivedMarginOrCollateral4>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CollateralMarginMarginUpdate5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref tech_rcrd_id_value) = self.tech_rcrd_id { if let Err(e) = tech_rcrd_id_value.validate() { return Err(e); } }
		if let Some(ref ctr_pty_value) = self.ctr_pty { if let Err(e) = ctr_pty_value.validate() { return Err(e); } }
		if let Err(e) = self.coll_prtfl_id.validate() { return Err(e); }
		if let Some(ref pstd_mrgn_or_coll_value) = self.pstd_mrgn_or_coll { if let Err(e) = pstd_mrgn_or_coll_value.validate() { return Err(e); } }
		if let Some(ref rcvd_mrgn_or_coll_value) = self.rcvd_mrgn_or_coll { if let Err(e) = rcvd_mrgn_or_coll_value.validate() { return Err(e); } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// Counterparty39 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Counterparty39 {
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: OrganisationIdentification15Choice,
	#[serde(rename = "OthrCtrPty")]
	pub othr_ctr_pty: PartyIdentification236Choice,
	#[serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none")]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "RptSubmitgNtty", skip_serializing_if = "Option::is_none")]
	pub rpt_submitg_ntty: Option<OrganisationIdentification15Choice>,
}

impl Counterparty39 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.rptg_ctr_pty.validate() { return Err(e); }
		if let Err(e) = self.othr_ctr_pty.validate() { return Err(e); }
		if let Some(ref ntty_rspnsbl_for_rpt_value) = self.ntty_rspnsbl_for_rpt { if let Err(e) = ntty_rspnsbl_for_rpt_value.validate() { return Err(e); } }
		if let Some(ref rpt_submitg_ntty_value) = self.rpt_submitg_ntty { if let Err(e) = rpt_submitg_ntty_value.validate() { return Err(e); } }
		Ok(())
	}
}


// GenericIdentification175 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification175 {
	#[serde(rename = "Id")]
	pub id: Max72Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericIdentification175 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
		if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
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


// Max105Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max105Text {
	#[serde(rename = "$value")]
	pub max105_text: String,
}

impl Max105Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max105_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max105_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max105_text.chars().count() > 105 {
			return Err(ValidationError::new(1002, "max105_text exceeds the maximum length of 105".to_string()));
		}
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


// Max500Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max500Text {
	#[serde(rename = "$value")]
	pub max500_text: String,
}

impl Max500Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max500_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max500_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max500_text.chars().count() > 500 {
			return Err(ValidationError::new(1002, "max500_text exceeds the maximum length of 500".to_string()));
		}
		Ok(())
	}
}


// Max52Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max52Text {
	#[serde(rename = "$value")]
	pub max52_text: String,
}

impl Max52Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max52_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max52_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max52_text.chars().count() > 52 {
			return Err(ValidationError::new(1002, "max52_text exceeds the maximum length of 52".to_string()));
		}
		Ok(())
	}
}


// Max72Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max72Text {
	#[serde(rename = "$value")]
	pub max72_text: String,
}

impl Max72Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max72_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max72_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max72_text.chars().count() > 72 {
			return Err(ValidationError::new(1002, "max72_text exceeds the maximum length of 72".to_string()));
		}
		Ok(())
	}
}


// NaturalPersonIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NaturalPersonIdentification2 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max105Text>,
	#[serde(rename = "Dmcl", skip_serializing_if = "Option::is_none")]
	pub dmcl: Option<Max500Text>,
}

impl NaturalPersonIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
		if let Some(ref dmcl_value) = self.dmcl { if let Err(e) = dmcl_value.validate() { return Err(e); } }
		Ok(())
	}
}


// OrganisationIdentification15Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification15Choice {
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<OrganisationIdentification38>,
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
}

impl OrganisationIdentification15Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
		if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
		if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
		Ok(())
	}
}


// OrganisationIdentification38 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification38 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max105Text>,
	#[serde(rename = "Dmcl", skip_serializing_if = "Option::is_none")]
	pub dmcl: Option<Max500Text>,
}

impl OrganisationIdentification38 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
		if let Some(ref dmcl_value) = self.dmcl { if let Err(e) = dmcl_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PartyIdentification236Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification236Choice {
	#[serde(rename = "Lgl", skip_serializing_if = "Option::is_none")]
	pub lgl: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "Ntrl", skip_serializing_if = "Option::is_none")]
	pub ntrl: Option<NaturalPersonIdentification2>,
}

impl PartyIdentification236Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref lgl_value) = self.lgl { if let Err(e) = lgl_value.validate() { return Err(e); } }
		if let Some(ref ntrl_value) = self.ntrl { if let Err(e) = ntrl_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PostedMarginOrCollateral4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostedMarginOrCollateral4 {
	#[serde(rename = "InitlMrgnPstd", skip_serializing_if = "Option::is_none")]
	pub initl_mrgn_pstd: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "VartnMrgnPstd", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_pstd: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "XcssCollPstd", skip_serializing_if = "Option::is_none")]
	pub xcss_coll_pstd: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl PostedMarginOrCollateral4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref initl_mrgn_pstd_value) = self.initl_mrgn_pstd { if let Err(e) = initl_mrgn_pstd_value.validate() { return Err(e); } }
		if let Some(ref vartn_mrgn_pstd_value) = self.vartn_mrgn_pstd { if let Err(e) = vartn_mrgn_pstd_value.validate() { return Err(e); } }
		if let Some(ref xcss_coll_pstd_value) = self.xcss_coll_pstd { if let Err(e) = xcss_coll_pstd_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ReceivedMarginOrCollateral4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReceivedMarginOrCollateral4 {
	#[serde(rename = "InitlMrgnRcvd", skip_serializing_if = "Option::is_none")]
	pub initl_mrgn_rcvd: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "VartnMrgnRcvd", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_rcvd: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "XcssCollRcvd", skip_serializing_if = "Option::is_none")]
	pub xcss_coll_rcvd: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl ReceivedMarginOrCollateral4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref initl_mrgn_rcvd_value) = self.initl_mrgn_rcvd { if let Err(e) = initl_mrgn_rcvd_value.validate() { return Err(e); } }
		if let Some(ref vartn_mrgn_rcvd_value) = self.vartn_mrgn_rcvd { if let Err(e) = vartn_mrgn_rcvd_value.validate() { return Err(e); } }
		if let Some(ref xcss_coll_rcvd_value) = self.xcss_coll_rcvd { if let Err(e) = xcss_coll_rcvd_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ReportPeriodActivity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ReportPeriodActivity1Code {
	#[default]
	#[serde(rename = "NOTX")]
	CodeNOTX,
}

impl ReportPeriodActivity1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SecuritiesFinancingReportingTransactionMarginDataReportV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesFinancingReportingTransactionMarginDataReportV02 {
	#[serde(rename = "TradData")]
	pub trad_data: TradeData39Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl SecuritiesFinancingReportingTransactionMarginDataReportV02 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.trad_data.validate() { return Err(e); }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
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


// TradeData39Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeData39Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[serde(rename = "Rpt", skip_serializing_if = "Option::is_none")]
	pub rpt: Option<Vec<TradeReport21Choice>>,
}

impl TradeData39Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref data_set_actn_value) = self.data_set_actn { if let Err(e) = data_set_actn_value.validate() { return Err(e); } }
		if let Some(ref rpt_vec) = self.rpt { for item in rpt_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// TradeReport21Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeReport21Choice {
	#[serde(rename = "New", skip_serializing_if = "Option::is_none")]
	pub new: Option<CollateralMarginCorrection6>,
	#[serde(rename = "Err", skip_serializing_if = "Option::is_none")]
	pub err: Option<CollateralMarginError4>,
	#[serde(rename = "Crrctn", skip_serializing_if = "Option::is_none")]
	pub crrctn: Option<CollateralMarginCorrection6>,
	#[serde(rename = "TradUpd", skip_serializing_if = "Option::is_none")]
	pub trad_upd: Option<CollateralMarginMarginUpdate5>,
}

impl TradeReport21Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref new_value) = self.new { if let Err(e) = new_value.validate() { return Err(e); } }
		if let Some(ref err_value) = self.err { if let Err(e) = err_value.validate() { return Err(e); } }
		if let Some(ref crrctn_value) = self.crrctn { if let Err(e) = crrctn_value.validate() { return Err(e); } }
		if let Some(ref trad_upd_value) = self.trad_upd { if let Err(e) = trad_upd_value.validate() { return Err(e); } }
		Ok(())
	}
}
