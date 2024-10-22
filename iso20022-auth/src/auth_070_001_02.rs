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
	
	
	// CollateralMarginCorrection6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CollateralMarginCorrection6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none") )]
		pub tech_rcrd_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgDtTm") )]
		pub rptg_dt_tm: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EvtDt") )]
		pub evt_dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPty") )]
		pub ctr_pty: Counterparty39,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CollPrtflId") )]
		pub coll_prtfl_id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstdMrgnOrColl", skip_serializing_if = "Option::is_none") )]
		pub pstd_mrgn_or_coll: Option<PostedMarginOrCollateral4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RcvdMrgnOrColl", skip_serializing_if = "Option::is_none") )]
		pub rcvd_mrgn_or_coll: Option<ReceivedMarginOrCollateral4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl CollateralMarginCorrection6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tech_rcrd_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "tech_rcrd_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 140 {
					return Err(ValidationError::new(1002, "tech_rcrd_id exceeds the maximum length of 140".to_string()));
				}
			}
			if let Err(e) = self.ctr_pty.validate() { return Err(e); }
			if self.coll_prtfl_id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "coll_prtfl_id is shorter than the minimum length of 1".to_string()));
			}
			if self.coll_prtfl_id.chars().count() > 52 {
				return Err(ValidationError::new(1002, "coll_prtfl_id exceeds the maximum length of 52".to_string()));
			}
			if let Some(ref val) = self.pstd_mrgn_or_coll { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.rcvd_mrgn_or_coll { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// CollateralMarginError4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CollateralMarginError4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none") )]
		pub tech_rcrd_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgDtTm") )]
		pub rptg_dt_tm: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPty") )]
		pub ctr_pty: Counterparty39,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CollPrtflId") )]
		pub coll_prtfl_id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl CollateralMarginError4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tech_rcrd_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "tech_rcrd_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 140 {
					return Err(ValidationError::new(1002, "tech_rcrd_id exceeds the maximum length of 140".to_string()));
				}
			}
			if let Err(e) = self.ctr_pty.validate() { return Err(e); }
			if self.coll_prtfl_id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "coll_prtfl_id is shorter than the minimum length of 1".to_string()));
			}
			if self.coll_prtfl_id.chars().count() > 52 {
				return Err(ValidationError::new(1002, "coll_prtfl_id exceeds the maximum length of 52".to_string()));
			}
			if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// CollateralMarginMarginUpdate5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CollateralMarginMarginUpdate5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none") )]
		pub tech_rcrd_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgDtTm") )]
		pub rptg_dt_tm: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EvtDt") )]
		pub evt_dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPty", skip_serializing_if = "Option::is_none") )]
		pub ctr_pty: Option<Counterparty39>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CollPrtflId") )]
		pub coll_prtfl_id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstdMrgnOrColl", skip_serializing_if = "Option::is_none") )]
		pub pstd_mrgn_or_coll: Option<PostedMarginOrCollateral4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RcvdMrgnOrColl", skip_serializing_if = "Option::is_none") )]
		pub rcvd_mrgn_or_coll: Option<ReceivedMarginOrCollateral4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl CollateralMarginMarginUpdate5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tech_rcrd_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "tech_rcrd_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 140 {
					return Err(ValidationError::new(1002, "tech_rcrd_id exceeds the maximum length of 140".to_string()));
				}
			}
			if let Some(ref val) = self.ctr_pty { if let Err(e) = val.validate() { return Err(e); } }
			if self.coll_prtfl_id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "coll_prtfl_id is shorter than the minimum length of 1".to_string()));
			}
			if self.coll_prtfl_id.chars().count() > 52 {
				return Err(ValidationError::new(1002, "coll_prtfl_id exceeds the maximum length of 52".to_string()));
			}
			if let Some(ref val) = self.pstd_mrgn_or_coll { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.rcvd_mrgn_or_coll { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// Counterparty39 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Counterparty39 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgCtrPty") )]
		pub rptg_ctr_pty: OrganisationIdentification15Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrCtrPty") )]
		pub othr_ctr_pty: PartyIdentification236Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none") )]
		pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptSubmitgNtty", skip_serializing_if = "Option::is_none") )]
		pub rpt_submitg_ntty: Option<OrganisationIdentification15Choice>,
	}
	
	impl Counterparty39 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rptg_ctr_pty.validate() { return Err(e); }
			if let Err(e) = self.othr_ctr_pty.validate() { return Err(e); }
			if let Some(ref val) = self.ntty_rspnsbl_for_rpt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.rpt_submitg_ntty { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// NaturalPersonIdentification2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct NaturalPersonIdentification2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: GenericIdentification175,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dmcl", skip_serializing_if = "Option::is_none") )]
		pub dmcl: Option<String>,
	}
	
	impl NaturalPersonIdentification2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
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
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.othr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.any_bic {
				let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
				if !pattern.is_match(&val) {
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
			if let Err(e) = self.id.validate() { return Err(e); }
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
	
	
	// PartyIdentification236Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PartyIdentification236Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Lgl", skip_serializing_if = "Option::is_none") )]
		pub lgl: Option<OrganisationIdentification15Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ntrl", skip_serializing_if = "Option::is_none") )]
		pub ntrl: Option<NaturalPersonIdentification2>,
	}
	
	impl PartyIdentification236Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.lgl { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ntrl { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.initl_mrgn_pstd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.vartn_mrgn_pstd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.xcss_coll_pstd { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ReceivedMarginOrCollateral4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ReceivedMarginOrCollateral4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "InitlMrgnRcvd", skip_serializing_if = "Option::is_none") )]
		pub initl_mrgn_rcvd: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VartnMrgnRcvd", skip_serializing_if = "Option::is_none") )]
		pub vartn_mrgn_rcvd: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XcssCollRcvd", skip_serializing_if = "Option::is_none") )]
		pub xcss_coll_rcvd: Option<ActiveOrHistoricCurrencyAndAmount>,
	}
	
	impl ReceivedMarginOrCollateral4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.initl_mrgn_rcvd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.vartn_mrgn_rcvd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.xcss_coll_rcvd { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// SecuritiesFinancingReportingTransactionMarginDataReportV02 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SecuritiesFinancingReportingTransactionMarginDataReportV02 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradData") )]
		pub trad_data: TradeData39Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl SecuritiesFinancingReportingTransactionMarginDataReportV02 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.trad_data.validate() { return Err(e); }
			if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
	
	
	// TradeData39Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeData39Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
		pub data_set_actn: Option<ReportPeriodActivity1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rpt", skip_serializing_if = "Option::is_none") )]
		pub rpt: Option<Vec<TradeReport21Choice>>,
	}
	
	impl TradeData39Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.data_set_actn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.rpt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TradeReport21Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeReport21Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "New", skip_serializing_if = "Option::is_none") )]
		pub new: Option<CollateralMarginCorrection6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Err", skip_serializing_if = "Option::is_none") )]
		pub err: Option<CollateralMarginError4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Crrctn", skip_serializing_if = "Option::is_none") )]
		pub crrctn: Option<CollateralMarginCorrection6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradUpd", skip_serializing_if = "Option::is_none") )]
		pub trad_upd: Option<CollateralMarginMarginUpdate5>,
	}
	
	impl TradeReport21Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.new { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.err { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.crrctn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.trad_upd { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
}