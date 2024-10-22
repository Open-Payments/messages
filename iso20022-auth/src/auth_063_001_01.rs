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
			if let Err(e) = self.amt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// CCPLiquidityStressTestingResultReportV01 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CCPLiquidityStressTestingResultReportV01 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "LqdtyStrssTstRslt") )]
		pub lqdty_strss_tst_rslt: Vec<LiquidityStressTestResult1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl CCPLiquidityStressTestingResultReportV01 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			for item in &self.lqdty_strss_tst_rslt { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// CoverTwoDefaulters1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CoverTwoDefaulters1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cover1Id") )]
		pub cover1_id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cover2Id") )]
		pub cover2_id: String,
	}
	
	impl CoverTwoDefaulters1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(&self.cover1_id) {
				return Err(ValidationError::new(1005, "cover1_id does not match the required pattern".to_string()));
			}
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(&self.cover2_id) {
				return Err(ValidationError::new(1005, "cover2_id does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// LiquidResourceInformation1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct LiquidResourceInformation1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CntrPtyId", skip_serializing_if = "Option::is_none") )]
		pub cntr_pty_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LqdRsrcVal") )]
		pub lqd_rsrc_val: AmountAndDirection102,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MktVal", skip_serializing_if = "Option::is_none") )]
		pub mkt_val: Option<AmountAndDirection102>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Scrd") )]
		pub scrd: bool,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AsstNcmbrd") )]
		pub asst_ncmbrd: bool,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QlfygRsrc") )]
		pub qlfyg_rsrc: bool,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AgcyArrgmnts") )]
		pub agcy_arrgmnts: bool,
	}
	
	impl LiquidResourceInformation1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cntr_pty_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "cntr_pty_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "cntr_pty_id exceeds the maximum length of 35".to_string()));
				}
			}
			if let Err(e) = self.lqd_rsrc_val.validate() { return Err(e); }
			if let Some(ref val) = self.mkt_val { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// LiquidResources1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct LiquidResources1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CshDue") )]
		pub csh_due: Vec<LiquidResourceInformation1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FcltiesCmmtdLinesOfCdt", skip_serializing_if = "Option::is_none") )]
		pub fclties_cmmtd_lines_of_cdt: Option<Vec<LiquidResourceInformation1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FcltiesCmmtdRpAgrmts", skip_serializing_if = "Option::is_none") )]
		pub fclties_cmmtd_rp_agrmts: Option<Vec<LiquidResourceInformation1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FcltiesCmmtdFxSwps", skip_serializing_if = "Option::is_none") )]
		pub fclties_cmmtd_fx_swps: Option<Vec<LiquidResourceInformation1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FcltiesOthrCmmtd", skip_serializing_if = "Option::is_none") )]
		pub fclties_othr_cmmtd: Option<Vec<LiquidResourceInformation1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FcltiesUcmmtd", skip_serializing_if = "Option::is_none") )]
		pub fclties_ucmmtd: Option<Vec<LiquidResourceInformation1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmsCCP", skip_serializing_if = "Option::is_none") )]
		pub fin_instrms_ccp: Option<Vec<LiquidResourceInformation1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmsTrsrInvstmts", skip_serializing_if = "Option::is_none") )]
		pub fin_instrms_trsr_invstmts: Option<Vec<LiquidResourceInformation1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmsDfltrsSttlmColl", skip_serializing_if = "Option::is_none") )]
		pub fin_instrms_dfltrs_sttlm_coll: Option<Vec<LiquidResourceInformation1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmsDfltrsNonCshColl", skip_serializing_if = "Option::is_none") )]
		pub fin_instrms_dfltrs_non_csh_coll: Option<Vec<LiquidResourceInformation1>>,
	}
	
	impl LiquidResources1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			for item in &self.csh_due { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref vec) = self.fclties_cmmtd_lines_of_cdt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.fclties_cmmtd_rp_agrmts { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.fclties_cmmtd_fx_swps { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.fclties_othr_cmmtd { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.fclties_ucmmtd { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.fin_instrms_ccp { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.fin_instrms_trsr_invstmts { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.fin_instrms_dfltrs_sttlm_coll { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.fin_instrms_dfltrs_non_csh_coll { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// LiquidityRequiredAndAvailable1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct LiquidityRequiredAndAvailable1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "LqdRsrcs") )]
		pub lqd_rsrcs: LiquidResources1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LqdtyHrzn") )]
		pub lqdty_hrzn: SettlementDate6Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StrssLqdRsrcRqrmnt") )]
		pub strss_lqd_rsrc_rqrmnt: StressLiquidResourceRequirement1,
	}
	
	impl LiquidityRequiredAndAvailable1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.lqd_rsrcs.validate() { return Err(e); }
			if let Err(e) = self.lqdty_hrzn.validate() { return Err(e); }
			if let Err(e) = self.strss_lqd_rsrc_rqrmnt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// LiquidityStressTestResult1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct LiquidityStressTestResult1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ScnroDfltrs") )]
		pub scnro_dfltrs: CoverTwoDefaulters1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LqdtyReqrdAndAvlbl") )]
		pub lqdty_reqrd_and_avlbl: Vec<LiquidityRequiredAndAvailable1>,
	}
	
	impl LiquidityStressTestResult1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if self.id.chars().count() > 256 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 256".to_string()));
			}
			if let Err(e) = self.scnro_dfltrs.validate() { return Err(e); }
			for item in &self.lqdty_reqrd_and_avlbl { if let Err(e) = item.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SettlementDate6Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum SettlementDate6Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "TFIV") )]
		CodeTFIV,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TFOR") )]
		CodeTFOR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TONE") )]
		CodeTONE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TTRE") )]
		CodeTTRE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TTWO") )]
		CodeTTWO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SAMD") )]
		CodeSAMD,
	}
	
	impl SettlementDate6Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// StressLiquidResourceRequirement1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct StressLiquidResourceRequirement1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "OprlOutflw") )]
		pub oprl_outflw: AmountAndDirection102,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VartnMrgnPmtOblgtn") )]
		pub vartn_mrgn_pmt_oblgtn: AmountAndDirection102,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmOrDlvry") )]
		pub sttlm_or_dlvry: AmountAndDirection102,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr") )]
		pub othr: AmountAndDirection102,
	}
	
	impl StressLiquidResourceRequirement1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.oprl_outflw.validate() { return Err(e); }
			if let Err(e) = self.vartn_mrgn_pmt_oblgtn.validate() { return Err(e); }
			if let Err(e) = self.sttlm_or_dlvry.validate() { return Err(e); }
			if let Err(e) = self.othr.validate() { return Err(e); }
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
	
}