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
	
	
	// CCPIncomeStatementAndCapitalAdequacyReportV01 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CCPIncomeStatementAndCapitalAdequacyReportV01 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "IncmStmt") )]
		pub incm_stmt: IncomeStatement1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CptlRqrmnts") )]
		pub cptl_rqrmnts: CapitalRequirement1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlCptl") )]
		pub ttl_cptl: ActiveCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LqdFinRsrcs") )]
		pub lqd_fin_rsrcs: ActiveCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HpthtclCptlMeasr") )]
		pub hpthtcl_cptl_measr: Vec<HypotheticalCapitalMeasure1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl CCPIncomeStatementAndCapitalAdequacyReportV01 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.incm_stmt.validate() { return Err(e); }
			if let Err(e) = self.cptl_rqrmnts.validate() { return Err(e); }
			if let Err(e) = self.ttl_cptl.validate() { return Err(e); }
			if let Err(e) = self.lqd_fin_rsrcs.validate() { return Err(e); }
			for item in &self.hpthtcl_cptl_measr { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// CapitalRequirement1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CapitalRequirement1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "WndgDwnOrRstrgRsk") )]
		pub wndg_dwn_or_rstrg_rsk: ActiveCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OprlAndLglRsk") )]
		pub oprl_and_lgl_rsk: ActiveCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtRsk") )]
		pub cdt_rsk: ActiveCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CntrPtyRsk") )]
		pub cntr_pty_rsk: ActiveCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MktRsk") )]
		pub mkt_rsk: ActiveCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BizRsk") )]
		pub biz_rsk: ActiveCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtfctnBffr", skip_serializing_if = "Option::is_none") )]
		pub ntfctn_bffr: Option<f64>,
	}
	
	impl CapitalRequirement1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.wndg_dwn_or_rstrg_rsk.validate() { return Err(e); }
			if let Err(e) = self.oprl_and_lgl_rsk.validate() { return Err(e); }
			if let Err(e) = self.cdt_rsk.validate() { return Err(e); }
			if let Err(e) = self.cntr_pty_rsk.validate() { return Err(e); }
			if let Err(e) = self.mkt_rsk.validate() { return Err(e); }
			if let Err(e) = self.biz_rsk.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// HypotheticalCapitalMeasure1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct HypotheticalCapitalMeasure1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DfltWtrfllId") )]
		pub dflt_wtrfll_id: String,
	}
	
	impl HypotheticalCapitalMeasure1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.amt.validate() { return Err(e); }
			if self.dflt_wtrfll_id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dflt_wtrfll_id is shorter than the minimum length of 1".to_string()));
			}
			if self.dflt_wtrfll_id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "dflt_wtrfll_id exceeds the maximum length of 35".to_string()));
			}
			Ok(())
		}
	}
	
	
	// IncomeStatement1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct IncomeStatement1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrFees") )]
		pub clr_fees: ActiveCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrOprgRvn") )]
		pub othr_oprg_rvn: ActiveCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OprgExpnss") )]
		pub oprg_expnss: ActiveCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OprgPrftOrLoss") )]
		pub oprg_prft_or_loss: AmountAndDirection102,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NetIntrstIncm") )]
		pub net_intrst_incm: ActiveCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrNonOprgRvn") )]
		pub othr_non_oprg_rvn: ActiveCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NonOprgExpnss") )]
		pub non_oprg_expnss: ActiveCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PreTaxPrftOrLoss") )]
		pub pre_tax_prft_or_loss: AmountAndDirection102,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstTaxPrftOrLoss") )]
		pub pst_tax_prft_or_loss: AmountAndDirection102,
	}
	
	impl IncomeStatement1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.clr_fees.validate() { return Err(e); }
			if let Err(e) = self.othr_oprg_rvn.validate() { return Err(e); }
			if let Err(e) = self.oprg_expnss.validate() { return Err(e); }
			if let Err(e) = self.oprg_prft_or_loss.validate() { return Err(e); }
			if let Err(e) = self.net_intrst_incm.validate() { return Err(e); }
			if let Err(e) = self.othr_non_oprg_rvn.validate() { return Err(e); }
			if let Err(e) = self.non_oprg_expnss.validate() { return Err(e); }
			if let Err(e) = self.pre_tax_prft_or_loss.validate() { return Err(e); }
			if let Err(e) = self.pst_tax_prft_or_loss.validate() { return Err(e); }
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