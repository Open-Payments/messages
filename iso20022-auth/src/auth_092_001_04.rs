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
	
	
	// CollateralPortfolioCode5Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CollateralPortfolioCode5Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtfl", skip_serializing_if = "Option::is_none") )]
		pub prtfl: Option<PortfolioCode3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MrgnPrtflCd", skip_serializing_if = "Option::is_none") )]
		pub mrgn_prtfl_cd: Option<MarginPortfolio3>,
	}
	
	impl CollateralPortfolioCode5Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.prtfl { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.mrgn_prtfl_cd { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CounterpartyData92 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CounterpartyData92 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none") )]
		pub rptg_ctr_pty: Option<OrganisationIdentification15Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptSubmitgNtty", skip_serializing_if = "Option::is_none") )]
		pub rpt_submitg_ntty: Option<OrganisationIdentification15Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none") )]
		pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
	}
	
	impl CounterpartyData92 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.rptg_ctr_pty { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.rpt_submitg_ntty { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ntty_rspnsbl_for_rpt { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DateAndDateTime2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DateAndDateTime2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
		pub dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtTm", skip_serializing_if = "Option::is_none") )]
		pub dt_tm: Option<String>,
	}
	
	impl DateAndDateTime2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// DerivativeEventType3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum DerivativeEventType3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ALOC") )]
		CodeALOC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLRG") )]
		CodeCLRG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLAL") )]
		CodeCLAL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "COMP") )]
		CodeCOMP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CORP") )]
		CodeCORP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CREV") )]
		CodeCREV,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ETRM") )]
		CodeETRM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EXER") )]
		CodeEXER,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INCP") )]
		CodeINCP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NOVA") )]
		CodeNOVA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PTNG") )]
		CodePTNG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TRAD") )]
		CodeTRAD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UPDT") )]
		CodeUPDT,
	}
	
	impl DerivativeEventType3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// DerivativesTradeRejectionStatisticalReportV04 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DerivativesTradeRejectionStatisticalReportV04 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RjctnSttstcs") )]
		pub rjctn_sttstcs: StatisticsPerCounterparty18Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl DerivativesTradeRejectionStatisticalReportV04 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rjctn_sttstcs.validate() { return Err(e); }
			if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// DetailedReportStatistics7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DetailedReportStatistics7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfRpts") )]
		pub ttl_nb_of_rpts: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfRptsAccptd") )]
		pub ttl_nb_of_rpts_accptd: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfRptsRjctd") )]
		pub ttl_nb_of_rpts_rjctd: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfRptsRjctdPerErr", skip_serializing_if = "Option::is_none") )]
		pub nb_of_rpts_rjctd_per_err: Option<Vec<NumberOfTransactionsPerValidationRule6>>,
	}
	
	impl DetailedReportStatistics7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.ttl_nb_of_rpts < 0.000000 {
				return Err(ValidationError::new(1003, "ttl_nb_of_rpts is less than the minimum value of 0.000000".to_string()));
			}
			if self.ttl_nb_of_rpts_accptd < 0.000000 {
				return Err(ValidationError::new(1003, "ttl_nb_of_rpts_accptd is less than the minimum value of 0.000000".to_string()));
			}
			if self.ttl_nb_of_rpts_rjctd < 0.000000 {
				return Err(ValidationError::new(1003, "ttl_nb_of_rpts_rjctd is less than the minimum value of 0.000000".to_string()));
			}
			if let Some(ref vec) = self.nb_of_rpts_rjctd_per_err { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// DetailedStatisticsPerCounterparty19 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DetailedStatisticsPerCounterparty19 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RefDt") )]
		pub ref_dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfRpts") )]
		pub ttl_nb_of_rpts: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfRptsAccptd") )]
		pub ttl_nb_of_rpts_accptd: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfRptsRjctd") )]
		pub ttl_nb_of_rpts_rjctd: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfTxs") )]
		pub ttl_nb_of_txs: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfTxsAccptd") )]
		pub ttl_nb_of_txs_accptd: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfTxsRjctd") )]
		pub ttl_nb_of_txs_rjctd: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlCrrctdRjctns", skip_serializing_if = "Option::is_none") )]
		pub ttl_crrctd_rjctns: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RjctnSttstcs") )]
		pub rjctn_sttstcs: Vec<RejectionStatistics9>,
	}
	
	impl DetailedStatisticsPerCounterparty19 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.ttl_nb_of_rpts < 0.000000 {
				return Err(ValidationError::new(1003, "ttl_nb_of_rpts is less than the minimum value of 0.000000".to_string()));
			}
			if self.ttl_nb_of_rpts_accptd < 0.000000 {
				return Err(ValidationError::new(1003, "ttl_nb_of_rpts_accptd is less than the minimum value of 0.000000".to_string()));
			}
			if self.ttl_nb_of_rpts_rjctd < 0.000000 {
				return Err(ValidationError::new(1003, "ttl_nb_of_rpts_rjctd is less than the minimum value of 0.000000".to_string()));
			}
			if self.ttl_nb_of_txs < 0.000000 {
				return Err(ValidationError::new(1003, "ttl_nb_of_txs is less than the minimum value of 0.000000".to_string()));
			}
			if self.ttl_nb_of_txs_accptd < 0.000000 {
				return Err(ValidationError::new(1003, "ttl_nb_of_txs_accptd is less than the minimum value of 0.000000".to_string()));
			}
			if self.ttl_nb_of_txs_rjctd < 0.000000 {
				return Err(ValidationError::new(1003, "ttl_nb_of_txs_rjctd is less than the minimum value of 0.000000".to_string()));
			}
			if let Some(ref val) = self.ttl_crrctd_rjctns {
				if *val < 0.000000 {
					return Err(ValidationError::new(1003, "ttl_crrctd_rjctns is less than the minimum value of 0.000000".to_string()));
				}
			}
			for item in &self.rjctn_sttstcs { if let Err(e) = item.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DetailedTransactionStatistics30 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DetailedTransactionStatistics30 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfTxs") )]
		pub ttl_nb_of_txs: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfTxsAccptd") )]
		pub ttl_nb_of_txs_accptd: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfTxsRjctd") )]
		pub ttl_nb_of_txs_rjctd: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlCrrctdRjctns", skip_serializing_if = "Option::is_none") )]
		pub ttl_crrctd_rjctns: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxsRjctnsRsn", skip_serializing_if = "Option::is_none") )]
		pub txs_rjctns_rsn: Option<Vec<RejectionReason71>>,
	}
	
	impl DetailedTransactionStatistics30 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.ttl_nb_of_txs < 0.000000 {
				return Err(ValidationError::new(1003, "ttl_nb_of_txs is less than the minimum value of 0.000000".to_string()));
			}
			if self.ttl_nb_of_txs_accptd < 0.000000 {
				return Err(ValidationError::new(1003, "ttl_nb_of_txs_accptd is less than the minimum value of 0.000000".to_string()));
			}
			if self.ttl_nb_of_txs_rjctd < 0.000000 {
				return Err(ValidationError::new(1003, "ttl_nb_of_txs_rjctd is less than the minimum value of 0.000000".to_string()));
			}
			if let Some(ref val) = self.ttl_crrctd_rjctns {
				if *val < 0.000000 {
					return Err(ValidationError::new(1003, "ttl_crrctd_rjctns is less than the minimum value of 0.000000".to_string()));
				}
			}
			if let Some(ref vec) = self.txs_rjctns_rsn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// DetailedTransactionStatistics7Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DetailedTransactionStatistics7Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
		pub data_set_actn: Option<ReportPeriodActivity1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtldSttstcs", skip_serializing_if = "Option::is_none") )]
		pub dtld_sttstcs: Option<DetailedTransactionStatistics30>,
	}
	
	impl DetailedTransactionStatistics7Choice {
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
	
	
	// LegalPersonIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct LegalPersonIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: OrganisationIdentification15Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
		pub ctry: Option<String>,
	}
	
	impl LegalPersonIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref val) = self.ctry {
				let pattern = Regex::new("[A-Z]{2,2}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// MarginPortfolio3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MarginPortfolio3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "InitlMrgnPrtflCd") )]
		pub initl_mrgn_prtfl_cd: PortfolioCode5Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VartnMrgnPrtflCd", skip_serializing_if = "Option::is_none") )]
		pub vartn_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
	}
	
	impl MarginPortfolio3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.initl_mrgn_prtfl_cd.validate() { return Err(e); }
			if let Some(ref val) = self.vartn_mrgn_prtfl_cd { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// MasterAgreement8 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MasterAgreement8 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<AgreementType2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Vrsn", skip_serializing_if = "Option::is_none") )]
		pub vrsn: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrMstrAgrmtDtls", skip_serializing_if = "Option::is_none") )]
		pub othr_mstr_agrmt_dtls: Option<String>,
	}
	
	impl MasterAgreement8 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tp { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// NaturalPersonIdentification3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct NaturalPersonIdentification3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: NaturalPersonIdentification2,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
		pub ctry: Option<String>,
	}
	
	impl NaturalPersonIdentification3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref val) = self.ctry {
				let pattern = Regex::new("[A-Z]{2,2}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// NotApplicable1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum NotApplicable1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "NOAP") )]
		CodeNOAP,
	}
	
	impl NotApplicable1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// NumberOfTransactionsPerValidationRule6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct NumberOfTransactionsPerValidationRule6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtldNb") )]
		pub dtld_nb: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptSts") )]
		pub rpt_sts: Vec<RejectionReason70>,
	}
	
	impl NumberOfTransactionsPerValidationRule6 {
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
	
	
	// PartyIdentification248Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PartyIdentification248Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Lgl", skip_serializing_if = "Option::is_none") )]
		pub lgl: Option<LegalPersonIdentification1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ntrl", skip_serializing_if = "Option::is_none") )]
		pub ntrl: Option<NaturalPersonIdentification3>,
	}
	
	impl PartyIdentification248Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.lgl { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ntrl { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PortfolioCode3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PortfolioCode3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoPrtfl", skip_serializing_if = "Option::is_none") )]
		pub no_prtfl: Option<NotApplicable1Code>,
	}
	
	impl PortfolioCode3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cd {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 52 {
					return Err(ValidationError::new(1002, "cd exceeds the maximum length of 52".to_string()));
				}
			}
			if let Some(ref val) = self.no_prtfl { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PortfolioCode5Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PortfolioCode5Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtfl", skip_serializing_if = "Option::is_none") )]
		pub prtfl: Option<PortfolioIdentification3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoPrtfl", skip_serializing_if = "Option::is_none") )]
		pub no_prtfl: Option<NotApplicable1Code>,
	}
	
	impl PortfolioCode5Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.prtfl { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.no_prtfl { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PortfolioIdentification3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PortfolioIdentification3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtflTxXmptn", skip_serializing_if = "Option::is_none") )]
		pub prtfl_tx_xmptn: Option<bool>,
	}
	
	impl PortfolioIdentification3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.cd.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if self.cd.chars().count() > 52 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 52".to_string()));
			}
			Ok(())
		}
	}
	
	
	// RejectionReason70 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RejectionReason70 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgRptId") )]
		pub msg_rpt_id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sts") )]
		pub sts: ReportingMessageStatus2Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtldVldtnRule", skip_serializing_if = "Option::is_none") )]
		pub dtld_vldtn_rule: Option<GenericValidationRuleIdentification1>,
	}
	
	impl RejectionReason70 {
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
	
	
	// RejectionReason71 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RejectionReason71 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxId") )]
		pub tx_id: TradeTransactionIdentification24,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sts") )]
		pub sts: ReportingMessageStatus2Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtldVldtnRule", skip_serializing_if = "Option::is_none") )]
		pub dtld_vldtn_rule: Option<Vec<GenericValidationRuleIdentification1>>,
	}
	
	impl RejectionReason71 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tx_id.validate() { return Err(e); }
			if let Err(e) = self.sts.validate() { return Err(e); }
			if let Some(ref vec) = self.dtld_vldtn_rule { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// RejectionStatistics9 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RejectionStatistics9 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtyId") )]
		pub ctr_pty_id: CounterpartyData92,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptSttstcs") )]
		pub rpt_sttstcs: DetailedReportStatistics7,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DerivSttstcs") )]
		pub deriv_sttstcs: DetailedTransactionStatistics7Choice,
	}
	
	impl RejectionStatistics9 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.ctr_pty_id.validate() { return Err(e); }
			if let Err(e) = self.rpt_sttstcs.validate() { return Err(e); }
			if let Err(e) = self.deriv_sttstcs.validate() { return Err(e); }
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
	
	
	// ReportingMessageStatus2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum ReportingMessageStatus2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ACPT") )]
		CodeACPT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RJCT") )]
		CodeRJCT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INCF") )]
		CodeINCF,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CRPT") )]
		CodeCRPT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NAUT") )]
		CodeNAUT,
	}
	
	impl ReportingMessageStatus2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// StatisticsPerCounterparty18Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct StatisticsPerCounterparty18Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
		pub data_set_actn: Option<ReportPeriodActivity1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rpt", skip_serializing_if = "Option::is_none") )]
		pub rpt: Option<DetailedStatisticsPerCounterparty19>,
	}
	
	impl StatisticsPerCounterparty18Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.data_set_actn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.rpt { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// TradeTransactionIdentification24 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeTransactionIdentification24 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none") )]
		pub tech_rcrd_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ActnTp", skip_serializing_if = "Option::is_none") )]
		pub actn_tp: Option<TransactionOperationType10Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgTmStmp", skip_serializing_if = "Option::is_none") )]
		pub rptg_tm_stmp: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DerivEvtTp", skip_serializing_if = "Option::is_none") )]
		pub deriv_evt_tp: Option<DerivativeEventType3Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DerivEvtTmStmp", skip_serializing_if = "Option::is_none") )]
		pub deriv_evt_tm_stmp: Option<DateAndDateTime2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none") )]
		pub othr_ctr_pty: Option<PartyIdentification248Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnqIdr", skip_serializing_if = "Option::is_none") )]
		pub unq_idr: Option<UniqueTransactionIdentifier2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MstrAgrmt", skip_serializing_if = "Option::is_none") )]
		pub mstr_agrmt: Option<MasterAgreement8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CollPrtflCd", skip_serializing_if = "Option::is_none") )]
		pub coll_prtfl_cd: Option<CollateralPortfolioCode5Choice>,
	}
	
	impl TradeTransactionIdentification24 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tech_rcrd_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "tech_rcrd_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 140 {
					return Err(ValidationError::new(1002, "tech_rcrd_id exceeds the maximum length of 140".to_string()));
				}
			}
			if let Some(ref val) = self.actn_tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.deriv_evt_tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.deriv_evt_tm_stmp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.othr_ctr_pty { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.unq_idr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.mstr_agrmt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.coll_prtfl_cd { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TransactionOperationType10Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum TransactionOperationType10Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "COMP") )]
		CodeCOMP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CORR") )]
		CodeCORR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EROR") )]
		CodeEROR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MODI") )]
		CodeMODI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NEWT") )]
		CodeNEWT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "POSC") )]
		CodePOSC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REVI") )]
		CodeREVI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TERM") )]
		CodeTERM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VALU") )]
		CodeVALU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MARU") )]
		CodeMARU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRTO") )]
		CodePRTO,
	}
	
	impl TransactionOperationType10Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// UniqueTransactionIdentifier2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct UniqueTransactionIdentifier2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnqTxIdr", skip_serializing_if = "Option::is_none") )]
		pub unq_tx_idr: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification175>,
	}
	
	impl UniqueTransactionIdentifier2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.unq_tx_idr {
				let pattern = Regex::new("[A-Z0-9]{18}[0-9]{2}[A-Z0-9]{0,32}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "unq_tx_idr does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
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