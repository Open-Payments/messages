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

#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub mod iso20022 {
	use regex::Regex;
	use crate::common::*;
	#[cfg(feature = "derive_serde")]
	use serde::{Deserialize, Serialize};
	
	
	// ActiveCurrencyAndAmountSimpleType ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveCurrencyAndAmountSimpleType {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
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
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// ActiveCurrencyCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveCurrencyCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
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
	
	
	// Amount3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Amount3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlAmt", skip_serializing_if = "Option::is_none") )]
		pub orgnl_amt: Option<ActiveCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgAmt") )]
		pub rptg_amt: ActiveCurrencyAndAmount,
	}
	
	impl Amount3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref orgnl_amt_value) = self.orgnl_amt { if let Err(e) = orgnl_amt_value.validate() { return Err(e); } }
			if let Err(e) = self.rptg_amt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// AmountAndDirection102 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// CCPMemberRequirementsReportV01 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			for item in &self.intra_day_rqrmnt_amt { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref intra_day_mrgn_call_vec) = self.intra_day_mrgn_call { for item in intra_day_mrgn_call_vec { if let Err(e) = item.validate() { return Err(e); } } }
			for item in &self.end_of_day_rqrmnt { if let Err(e) = item.validate() { return Err(e); } }
			for item in &self.dflt_fnd_rqrmnt { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// DefaultFundRequirement1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct DefaultFundRequirement1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrMmbId") )]
		pub clr_mmb_id: GenericIdentification165,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SvcId", skip_serializing_if = "Option::is_none") )]
		pub svc_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveCurrencyAndAmount,
	}
	
	impl DefaultFundRequirement1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.clr_mmb_id.validate() { return Err(e); }
			if let Some(ref svc_id_value) = self.svc_id { if let Err(e) = svc_id_value.validate() { return Err(e); } }
			if let Err(e) = self.amt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// EndOfDayRequirement2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Err(e) = self.initl_mrgn_rqrmnts.validate() { return Err(e); }
			if let Err(e) = self.vartn_mrgn_rqrmnts.validate() { return Err(e); }
			if let Err(e) = self.mrgn_acct_id.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// GenericIdentification165 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GenericIdentification165 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max256Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<SchemeIdentificationType1Code>,
	}
	
	impl GenericIdentification165 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GenericIdentification36 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GenericIdentification36 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
		pub issr: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<Max35Text>,
	}
	
	impl GenericIdentification36 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Err(e) = self.issr.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ISODateTime ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISODateTime {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iso_date_time: String,
	}
	
	impl ISODateTime {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// InitialMarginExposure1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Err(e) = self.amt.validate() { return Err(e); }
			if let Err(e) = self.tp.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// InitialMarginRequirement1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct InitialMarginRequirement1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "InitlMrgnXpsr") )]
		pub initl_mrgn_xpsr: Vec<InitialMarginExposure1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cdt") )]
		pub cdt: ActiveCurrencyAndAmount,
	}
	
	impl InitialMarginRequirement1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			for item in &self.initl_mrgn_xpsr { if let Err(e) = item.validate() { return Err(e); } }
			if let Err(e) = self.cdt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// IntraDayMarginCall1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Err(e) = self.mrgn_acct_id.validate() { return Err(e); }
			if let Err(e) = self.intra_day_call.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// IntraDayRequirement1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
			if let Err(e) = self.intra_day_mrgn_call.validate() { return Err(e); }
			if let Err(e) = self.peak_initl_mrgn_lblty.validate() { return Err(e); }
			if let Err(e) = self.peak_vartn_mrgn_lblty.validate() { return Err(e); }
			if let Err(e) = self.aggt_peak_lblty.validate() { return Err(e); }
			if let Err(e) = self.mrgn_acct_id.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// MarginType2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct MarginType2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<MarginType2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification36>,
	}
	
	impl MarginType2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// MarginType2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// Max140Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max140Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
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
	
	
	// Max256Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max256Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max256_text: String,
	}
	
	impl Max256Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max256_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max256_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max256_text.chars().count() > 256 {
				return Err(ValidationError::new(1002, "max256_text exceeds the maximum length of 256".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max350Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max350Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
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
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max35Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
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
	
	
	// PlusOrMinusIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct PlusOrMinusIndicator {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub plus_or_minus_indicator: bool,
	}
	
	impl PlusOrMinusIndicator {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// SchemeIdentificationType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct SupplementaryData1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none") )]
		pub plc_and_nm: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Envlp") )]
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
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SupplementaryDataEnvelope1 {
	}
	
	impl SupplementaryDataEnvelope1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// TrueFalseIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct TrueFalseIndicator {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub true_false_indicator: bool,
	}
	
	impl TrueFalseIndicator {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
}