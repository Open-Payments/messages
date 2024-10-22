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
	
	
	// AmountAndDirection86 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AmountAndDirection86 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sgn") )]
		pub sgn: bool,
	}
	
	impl AmountAndDirection86 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.amt < 0.000000 {
				return Err(ValidationError::new(1003, "amt is less than the minimum value of 0.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// CCPDailyCashFlowsReportV02 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CCPDailyCashFlowsReportV02 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CncntrtnAgt") )]
		pub cncntrtn_agt: Vec<ConcentrationAgent1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmAgt") )]
		pub sttlm_agt: Vec<SettlementAgent2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl CCPDailyCashFlowsReportV02 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			for item in &self.cncntrtn_agt { if let Err(e) = item.validate() { return Err(e); } }
			for item in &self.sttlm_agt { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// ConcentrationAccount1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ConcentrationAccount1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "InFlow") )]
		pub in_flow: Flows1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OutFlow") )]
		pub out_flow: Flows1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EndOfDay") )]
		pub end_of_day: AmountAndDirection102,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PeakCdt") )]
		pub peak_cdt: ActiveCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PeakDbt") )]
		pub peak_dbt: ActiveCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LatePmtConf") )]
		pub late_pmt_conf: String,
	}
	
	impl ConcentrationAccount1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.in_flow.validate() { return Err(e); }
			if let Err(e) = self.out_flow.validate() { return Err(e); }
			if let Err(e) = self.end_of_day.validate() { return Err(e); }
			if let Err(e) = self.peak_cdt.validate() { return Err(e); }
			if let Err(e) = self.peak_dbt.validate() { return Err(e); }
			let pattern = Regex::new("[0-9]{1,10}").unwrap();
			if !pattern.is_match(&self.late_pmt_conf) {
				return Err(ValidationError::new(1005, "late_pmt_conf does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ConcentrationAgent1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ConcentrationAgent1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Acct") )]
		pub acct: Vec<ConcentrationAccount1>,
	}
	
	impl ConcentrationAgent1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(&self.id) {
				return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
			}
			for item in &self.acct { if let Err(e) = item.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Flows1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Flows1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtBkFlows") )]
		pub pmt_bk_flows: AmountAndDirection102,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InvstmtFlows") )]
		pub invstmt_flows: AmountAndDirection102,
	}
	
	impl Flows1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.pmt_bk_flows.validate() { return Err(e); }
			if let Err(e) = self.invstmt_flows.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// PaymentAccount4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaymentAccount4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
		pub ccy: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NetPmt") )]
		pub net_pmt: AmountAndDirection86,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GrssCdts") )]
		pub grss_cdts: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GrssDbts") )]
		pub grss_dbts: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LatePmtConf") )]
		pub late_pmt_conf: String,
	}
	
	impl PaymentAccount4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(&self.ccy) {
				return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
			}
			if let Err(e) = self.net_pmt.validate() { return Err(e); }
			let pattern = Regex::new("[0-9]{1,10}").unwrap();
			if !pattern.is_match(&self.late_pmt_conf) {
				return Err(ValidationError::new(1005, "late_pmt_conf does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// SettlementAgent2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SettlementAgent2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Acct") )]
		pub acct: Vec<PaymentAccount4>,
	}
	
	impl SettlementAgent2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(&self.id) {
				return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
			}
			for item in &self.acct { if let Err(e) = item.validate() { return Err(e); } }
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