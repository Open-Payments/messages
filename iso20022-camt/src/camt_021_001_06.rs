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
	
	
	// ErrorHandling3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ErrorHandling3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalSystemErrorHandling1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl ErrorHandling3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ErrorHandling5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ErrorHandling5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Err") )]
		pub err: ErrorHandling3Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<Max140Text>,
	}
	
	impl ErrorHandling5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.err.validate() { return Err(e); }
			if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ExternalEnquiryRequestType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalEnquiryRequestType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_enquiry_request_type1_code: String,
	}
	
	impl ExternalEnquiryRequestType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_enquiry_request_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_enquiry_request_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_enquiry_request_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_enquiry_request_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalPaymentControlRequestType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalPaymentControlRequestType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_payment_control_request_type1_code: String,
	}
	
	impl ExternalPaymentControlRequestType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_payment_control_request_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_payment_control_request_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_payment_control_request_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_payment_control_request_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalSystemErrorHandling1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalSystemErrorHandling1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_system_error_handling1_code: String,
	}
	
	impl ExternalSystemErrorHandling1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_system_error_handling1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_system_error_handling1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_system_error_handling1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_system_error_handling1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// GeneralBusinessInformation1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GeneralBusinessInformation1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Qlfr", skip_serializing_if = "Option::is_none") )]
		pub qlfr: Option<InformationQualifierType1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sbjt", skip_serializing_if = "Option::is_none") )]
		pub sbjt: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SbjtDtls", skip_serializing_if = "Option::is_none") )]
		pub sbjt_dtls: Option<Max350Text>,
	}
	
	impl GeneralBusinessInformation1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref qlfr_value) = self.qlfr { if let Err(e) = qlfr_value.validate() { return Err(e); } }
			if let Some(ref sbjt_value) = self.sbjt { if let Err(e) = sbjt_value.validate() { return Err(e); } }
			if let Some(ref sbjt_dtls_value) = self.sbjt_dtls { if let Err(e) = sbjt_dtls_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GeneralBusinessOrError7Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GeneralBusinessOrError7Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "OprlErr", skip_serializing_if = "Option::is_none") )]
		pub oprl_err: Option<Vec<ErrorHandling5>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BizRpt", skip_serializing_if = "Option::is_none") )]
		pub biz_rpt: Option<Vec<GeneralBusinessReport6>>,
	}
	
	impl GeneralBusinessOrError7Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref oprl_err_vec) = self.oprl_err { for item in oprl_err_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref biz_rpt_vec) = self.biz_rpt { for item in biz_rpt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// GeneralBusinessOrError8Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GeneralBusinessOrError8Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BizErr", skip_serializing_if = "Option::is_none") )]
		pub biz_err: Option<Vec<ErrorHandling5>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GnlBiz", skip_serializing_if = "Option::is_none") )]
		pub gnl_biz: Option<GeneralBusinessInformation1>,
	}
	
	impl GeneralBusinessOrError8Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref biz_err_vec) = self.biz_err { for item in biz_err_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref gnl_biz_value) = self.gnl_biz { if let Err(e) = gnl_biz_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GeneralBusinessReport6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GeneralBusinessReport6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BizInfRef") )]
		pub biz_inf_ref: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GnlBizOrErr") )]
		pub gnl_biz_or_err: GeneralBusinessOrError8Choice,
	}
	
	impl GeneralBusinessReport6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.biz_inf_ref.validate() { return Err(e); }
			if let Err(e) = self.gnl_biz_or_err.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// GenericIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GenericIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl GenericIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
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
	
	
	// InformationQualifierType1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct InformationQualifierType1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "IsFrmtd", skip_serializing_if = "Option::is_none") )]
		pub is_frmtd: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prty", skip_serializing_if = "Option::is_none") )]
		pub prty: Option<Priority1Code>,
	}
	
	impl InformationQualifierType1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref prty_value) = self.prty { if let Err(e) = prty_value.validate() { return Err(e); } }
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
	
	
	// MessageHeader7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct MessageHeader7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
		pub msg_id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
		pub cre_dt_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ReqTp", skip_serializing_if = "Option::is_none") )]
		pub req_tp: Option<RequestType4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlBizQry", skip_serializing_if = "Option::is_none") )]
		pub orgnl_biz_qry: Option<OriginalBusinessQuery1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QryNm", skip_serializing_if = "Option::is_none") )]
		pub qry_nm: Option<Max35Text>,
	}
	
	impl MessageHeader7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.msg_id.validate() { return Err(e); }
			if let Some(ref req_tp_value) = self.req_tp { if let Err(e) = req_tp_value.validate() { return Err(e); } }
			if let Some(ref orgnl_biz_qry_value) = self.orgnl_biz_qry { if let Err(e) = orgnl_biz_qry_value.validate() { return Err(e); } }
			if let Some(ref qry_nm_value) = self.qry_nm { if let Err(e) = qry_nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// OriginalBusinessQuery1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct OriginalBusinessQuery1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
		pub msg_id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none") )]
		pub msg_nm_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
		pub cre_dt_tm: Option<String>,
	}
	
	impl OriginalBusinessQuery1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.msg_id.validate() { return Err(e); }
			if let Some(ref msg_nm_id_value) = self.msg_nm_id { if let Err(e) = msg_nm_id_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Priority1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum Priority1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "HIGH") )]
		CodeHIGH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NORM") )]
		CodeNORM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LOWW") )]
		CodeLOWW,
	}
	
	impl Priority1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// RequestType4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct RequestType4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtCtrl", skip_serializing_if = "Option::is_none") )]
		pub pmt_ctrl: Option<ExternalPaymentControlRequestType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Enqry", skip_serializing_if = "Option::is_none") )]
		pub enqry: Option<ExternalEnquiryRequestType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification1>,
	}
	
	impl RequestType4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref pmt_ctrl_value) = self.pmt_ctrl { if let Err(e) = pmt_ctrl_value.validate() { return Err(e); } }
			if let Some(ref enqry_value) = self.enqry { if let Err(e) = enqry_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ReturnGeneralBusinessInformationV06 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ReturnGeneralBusinessInformationV06 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgHdr") )]
		pub msg_hdr: MessageHeader7,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptOrErr") )]
		pub rpt_or_err: GeneralBusinessOrError7Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl ReturnGeneralBusinessInformationV06 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.msg_hdr.validate() { return Err(e); }
			if let Err(e) = self.rpt_or_err.validate() { return Err(e); }
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
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
	
	
	// YesNoIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct YesNoIndicator {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub yes_no_indicator: bool,
	}
	
	impl YesNoIndicator {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
}