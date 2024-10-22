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
	
	
	// AgreementType2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AgreementType2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl AgreementType2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tp {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 4 {
					return Err(ValidationError::new(1002, "tp exceeds the maximum length of 4".to_string()));
				}
			}
			if let Some(ref val) = self.prtry {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 50 {
					return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 50".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// DetailedReportStatistics5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DetailedReportStatistics5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfRpts") )]
		pub ttl_nb_of_rpts: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfRptsAccptd") )]
		pub ttl_nb_of_rpts_accptd: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfRptsRjctd") )]
		pub ttl_nb_of_rpts_rjctd: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfRptsRjctdPerErr", skip_serializing_if = "Option::is_none") )]
		pub nb_of_rpts_rjctd_per_err: Option<Vec<NumberOfTransactionsPerValidationRule5>>,
	}
	
	impl DetailedReportStatistics5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(&self.ttl_nb_of_rpts) {
				return Err(ValidationError::new(1005, "ttl_nb_of_rpts does not match the required pattern".to_string()));
			}
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(&self.ttl_nb_of_rpts_accptd) {
				return Err(ValidationError::new(1005, "ttl_nb_of_rpts_accptd does not match the required pattern".to_string()));
			}
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(&self.ttl_nb_of_rpts_rjctd) {
				return Err(ValidationError::new(1005, "ttl_nb_of_rpts_rjctd does not match the required pattern".to_string()));
			}
			if let Some(ref vec) = self.nb_of_rpts_rjctd_per_err { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// DetailedTransactionStatistics13 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DetailedTransactionStatistics13 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfTxs") )]
		pub ttl_nb_of_txs: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfTxsAccptd") )]
		pub ttl_nb_of_txs_accptd: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfTxsRjctd") )]
		pub ttl_nb_of_txs_rjctd: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxsRjctnsRsn", skip_serializing_if = "Option::is_none") )]
		pub txs_rjctns_rsn: Option<Vec<RejectionReason53>>,
	}
	
	impl DetailedTransactionStatistics13 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(&self.ttl_nb_of_txs) {
				return Err(ValidationError::new(1005, "ttl_nb_of_txs does not match the required pattern".to_string()));
			}
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(&self.ttl_nb_of_txs_accptd) {
				return Err(ValidationError::new(1005, "ttl_nb_of_txs_accptd does not match the required pattern".to_string()));
			}
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(&self.ttl_nb_of_txs_rjctd) {
				return Err(ValidationError::new(1005, "ttl_nb_of_txs_rjctd does not match the required pattern".to_string()));
			}
			if let Some(ref vec) = self.txs_rjctns_rsn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// DetailedTransactionStatistics2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DetailedTransactionStatistics2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
		pub data_set_actn: Option<ReportPeriodActivity1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtldSttstcs", skip_serializing_if = "Option::is_none") )]
		pub dtld_sttstcs: Option<DetailedTransactionStatistics13>,
	}
	
	impl DetailedTransactionStatistics2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.data_set_actn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.dtld_sttstcs { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// GenericValidationRuleIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericValidationRuleIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<ValidationRuleSchemeName1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<String>,
	}
	
	impl GenericValidationRuleIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if self.id.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
			if let Some(ref val) = self.desc {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 350 {
					return Err(ValidationError::new(1002, "desc exceeds the maximum length of 350".to_string()));
				}
			}
			if let Some(ref val) = self.schme_nm { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// MasterAgreement7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MasterAgreement7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: AgreementType2Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Vrsn", skip_serializing_if = "Option::is_none") )]
		pub vrsn: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrMstrAgrmtDtls", skip_serializing_if = "Option::is_none") )]
		pub othr_mstr_agrmt_dtls: Option<String>,
	}
	
	impl MasterAgreement7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Some(ref val) = self.vrsn {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "vrsn is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 50 {
					return Err(ValidationError::new(1002, "vrsn exceeds the maximum length of 50".to_string()));
				}
			}
			if let Some(ref val) = self.othr_mstr_agrmt_dtls {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "othr_mstr_agrmt_dtls is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 350 {
					return Err(ValidationError::new(1002, "othr_mstr_agrmt_dtls exceeds the maximum length of 350".to_string()));
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
	
	
	// NumberOfTransactionsPerValidationRule5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct NumberOfTransactionsPerValidationRule5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtldNb") )]
		pub dtld_nb: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptSts") )]
		pub rpt_sts: Vec<RejectionReason45>,
	}
	
	impl NumberOfTransactionsPerValidationRule5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(&self.dtld_nb) {
				return Err(ValidationError::new(1005, "dtld_nb does not match the required pattern".to_string()));
			}
			for item in &self.rpt_sts { if let Err(e) = item.validate() { return Err(e); } }
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
	
	
	// RejectionReason45 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RejectionReason45 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgRptId") )]
		pub msg_rpt_id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sts") )]
		pub sts: ReportingMessageStatus1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtldVldtnRule", skip_serializing_if = "Option::is_none") )]
		pub dtld_vldtn_rule: Option<GenericValidationRuleIdentification1>,
	}
	
	impl RejectionReason45 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.msg_rpt_id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "msg_rpt_id is shorter than the minimum length of 1".to_string()));
			}
			if self.msg_rpt_id.chars().count() > 140 {
				return Err(ValidationError::new(1002, "msg_rpt_id exceeds the maximum length of 140".to_string()));
			}
			if let Err(e) = self.sts.validate() { return Err(e); }
			if let Some(ref val) = self.dtld_vldtn_rule { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// RejectionReason53 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RejectionReason53 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxId") )]
		pub tx_id: TransactionIdentification3Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sts") )]
		pub sts: ReportingMessageStatus1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtldVldtnRule", skip_serializing_if = "Option::is_none") )]
		pub dtld_vldtn_rule: Option<Vec<GenericValidationRuleIdentification1>>,
	}
	
	impl RejectionReason53 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tx_id.validate() { return Err(e); }
			if let Err(e) = self.sts.validate() { return Err(e); }
			if let Some(ref vec) = self.dtld_vldtn_rule { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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
	
	
	// ReportingMessageStatus1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum ReportingMessageStatus1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ACPT") )]
		CodeACPT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ACTC") )]
		CodeACTC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PART") )]
		CodePART,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RCVD") )]
		CodeRCVD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RJCT") )]
		CodeRJCT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RMDR") )]
		CodeRMDR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WARN") )]
		CodeWARN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INCF") )]
		CodeINCF,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CRPT") )]
		CodeCRPT,
	}
	
	impl ReportingMessageStatus1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// SecuritiesFinancingReportingTransactionStatusAdviceV02 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SecuritiesFinancingReportingTransactionStatusAdviceV02 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxRptStsAndRsn") )]
		pub tx_rpt_sts_and_rsn: Vec<TradeData35Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl SecuritiesFinancingReportingTransactionStatusAdviceV02 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			for item in &self.tx_rpt_sts_and_rsn { if let Err(e) = item.validate() { return Err(e); } }
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
	
	
	// TradeData29 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeData29 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptSttstcs") )]
		pub rpt_sttstcs: Vec<DetailedReportStatistics5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxSttstcs") )]
		pub tx_sttstcs: Vec<DetailedTransactionStatistics2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl TradeData29 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			for item in &self.rpt_sttstcs { if let Err(e) = item.validate() { return Err(e); } }
			for item in &self.tx_sttstcs { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TradeData35Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeData35Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
		pub data_set_actn: Option<ReportPeriodActivity1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rpt", skip_serializing_if = "Option::is_none") )]
		pub rpt: Option<Vec<TradeData29>>,
	}
	
	impl TradeData35Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.data_set_actn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.rpt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TradeTransactionIdentification16 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeTransactionIdentification16 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none") )]
		pub tech_rcrd_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgCtrPty") )]
		pub rptg_ctr_pty: OrganisationIdentification15Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrCtrPty") )]
		pub othr_ctr_pty: PartyIdentification236Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none") )]
		pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CollPrtflId", skip_serializing_if = "Option::is_none") )]
		pub coll_prtfl_id: Option<String>,
	}
	
	impl TradeTransactionIdentification16 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tech_rcrd_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "tech_rcrd_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 140 {
					return Err(ValidationError::new(1002, "tech_rcrd_id exceeds the maximum length of 140".to_string()));
				}
			}
			if let Err(e) = self.rptg_ctr_pty.validate() { return Err(e); }
			if let Err(e) = self.othr_ctr_pty.validate() { return Err(e); }
			if let Some(ref val) = self.ntty_rspnsbl_for_rpt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.coll_prtfl_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "coll_prtfl_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 52 {
					return Err(ValidationError::new(1002, "coll_prtfl_id exceeds the maximum length of 52".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// TradeTransactionIdentification17 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeTransactionIdentification17 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none") )]
		pub tech_rcrd_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgCtrPty") )]
		pub rptg_ctr_pty: OrganisationIdentification15Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptSubmitgNtty") )]
		pub rpt_submitg_ntty: OrganisationIdentification15Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none") )]
		pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
	}
	
	impl TradeTransactionIdentification17 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tech_rcrd_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "tech_rcrd_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 140 {
					return Err(ValidationError::new(1002, "tech_rcrd_id exceeds the maximum length of 140".to_string()));
				}
			}
			if let Err(e) = self.rptg_ctr_pty.validate() { return Err(e); }
			if let Err(e) = self.rpt_submitg_ntty.validate() { return Err(e); }
			if let Some(ref val) = self.ntty_rspnsbl_for_rpt { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TradeTransactionIdentification20 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeTransactionIdentification20 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none") )]
		pub tech_rcrd_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgCtrPty") )]
		pub rptg_ctr_pty: OrganisationIdentification15Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrCtrPty") )]
		pub othr_ctr_pty: PartyIdentification236Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none") )]
		pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnqTradIdr", skip_serializing_if = "Option::is_none") )]
		pub unq_trad_idr: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MstrAgrmt", skip_serializing_if = "Option::is_none") )]
		pub mstr_agrmt: Option<MasterAgreement7>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AgtLndr", skip_serializing_if = "Option::is_none") )]
		pub agt_lndr: Option<OrganisationIdentification15Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TrptyAgt", skip_serializing_if = "Option::is_none") )]
		pub trpty_agt: Option<OrganisationIdentification15Choice>,
	}
	
	impl TradeTransactionIdentification20 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tech_rcrd_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "tech_rcrd_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 140 {
					return Err(ValidationError::new(1002, "tech_rcrd_id exceeds the maximum length of 140".to_string()));
				}
			}
			if let Err(e) = self.rptg_ctr_pty.validate() { return Err(e); }
			if let Err(e) = self.othr_ctr_pty.validate() { return Err(e); }
			if let Some(ref val) = self.ntty_rspnsbl_for_rpt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.unq_trad_idr {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "unq_trad_idr is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 52 {
					return Err(ValidationError::new(1002, "unq_trad_idr exceeds the maximum length of 52".to_string()));
				}
			}
			if let Some(ref val) = self.mstr_agrmt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.agt_lndr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.trpty_agt { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TransactionIdentification3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TransactionIdentification3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tx", skip_serializing_if = "Option::is_none") )]
		pub tx: Option<TradeTransactionIdentification20>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MrgnRptg", skip_serializing_if = "Option::is_none") )]
		pub mrgn_rptg: Option<TradeTransactionIdentification16>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CollReuse", skip_serializing_if = "Option::is_none") )]
		pub coll_reuse: Option<TradeTransactionIdentification17>,
	}
	
	impl TransactionIdentification3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tx { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.mrgn_rptg { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.coll_reuse { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ValidationRuleSchemeName1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ValidationRuleSchemeName1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl ValidationRuleSchemeName1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cd {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 4 {
					return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
				}
			}
			if let Some(ref val) = self.prtry {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
				}
			}
			Ok(())
		}
	}
	
}