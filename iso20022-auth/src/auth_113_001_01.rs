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


use regex::Regex;
use crate::common::*;
#[cfg(feature = "derive_serde")]
use serde::{Deserialize, Serialize};


// ActiveCurrencyAnd13DecimalAmount ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ActiveCurrencyAnd13DecimalAmount {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
	pub value: f64,
}

impl ActiveCurrencyAnd13DecimalAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


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


// AmountAndDirection53 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AmountAndDirection53 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sgn", skip_serializing_if = "Option::is_none") )]
	pub sgn: Option<bool>,
}

impl AmountAndDirection53 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		Ok(())
	}
}


// AmountAndDirection61 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AmountAndDirection61 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveCurrencyAnd13DecimalAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sgn", skip_serializing_if = "Option::is_none") )]
	pub sgn: Option<bool>,
}

impl AmountAndDirection61 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		Ok(())
	}
}


// AuctionData2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AuctionData2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgPhs", skip_serializing_if = "Option::is_none") )]
	pub tradg_phs: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IndctvAuctnPric", skip_serializing_if = "Option::is_none") )]
	pub indctv_auctn_pric: Option<SecuritiesTransactionPrice21Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IndctvAuctnVol", skip_serializing_if = "Option::is_none") )]
	pub indctv_auctn_vol: Option<FinancialInstrumentQuantity25Choice>,
}

impl AuctionData2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tradg_phs {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tradg_phs is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 50 {
				return Err(ValidationError::new(1002, "tradg_phs exceeds the maximum length of 50".to_string()));
			}
		}
		if let Some(ref val) = self.indctv_auctn_pric { val.validate()? }
		if let Some(ref val) = self.indctv_auctn_vol { val.validate()? }
		Ok(())
	}
}


// CancelOrderReport1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CancelOrderReport1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptId") )]
	pub rpt_id: String,
}

impl CancelOrderReport1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.rpt_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "rpt_id is shorter than the minimum length of 1".to_string()));
		}
		if self.rpt_id.chars().count() > 140 {
			return Err(ValidationError::new(1002, "rpt_id exceeds the maximum length of 140".to_string()));
		}
		Ok(())
	}
}


// DateTimePeriod1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DateTimePeriod1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDtTm") )]
	pub fr_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDtTm") )]
	pub to_dt_tm: String,
}

impl DateTimePeriod1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ExecutingParty2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ExecutingParty2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prsn", skip_serializing_if = "Option::is_none") )]
	pub prsn: Option<GenericPersonIdentification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Algo", skip_serializing_if = "Option::is_none") )]
	pub algo: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Clnt", skip_serializing_if = "Option::is_none") )]
	pub clnt: Option<NoReasonCode>,
}

impl ExecutingParty2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.prsn { val.validate()? }
		if let Some(ref val) = self.algo {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "algo is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 50 {
				return Err(ValidationError::new(1002, "algo exceeds the maximum length of 50".to_string()));
			}
		}
		if let Some(ref val) = self.clnt { val.validate()? }
		Ok(())
	}
}


// FinancialInstrument99Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrument99Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StrtgyInstrms", skip_serializing_if = "Option::is_none") )]
	pub strtgy_instrms: Option<Vec<String>>,
}

impl FinancialInstrument99Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.strtgy_instrms {
			for item in vec {
				let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
				if !pattern.is_match(&item) {
					return Err(ValidationError::new(1005, "strtgy_instrms does not match the required pattern".to_string()));
				}
			}
		}
		Ok(())
	}
}


// FinancialInstrumentQuantity25Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrumentQuantity25Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Unit", skip_serializing_if = "Option::is_none") )]
	pub unit: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmnlVal", skip_serializing_if = "Option::is_none") )]
	pub nmnl_val: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MntryVal", skip_serializing_if = "Option::is_none") )]
	pub mntry_val: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl FinancialInstrumentQuantity25Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nmnl_val { val.validate()? }
		if let Some(ref val) = self.mntry_val { val.validate()? }
		Ok(())
	}
}


// GenericIdentification30 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericIdentification30 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
	pub issr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<String>,
}

impl GenericIdentification30 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
		if !pattern.is_match(&self.id) {
			return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
		}
		if self.issr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
		}
		if self.issr.chars().count() > 35 {
			return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.schme_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "schme_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "schme_nm exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// GenericPersonIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericPersonIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericPersonIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.schme_nm { val.validate()? }
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


// MinimumExecutable1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MinimumExecutable1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sz", skip_serializing_if = "Option::is_none") )]
	pub sz: Option<FinancialInstrumentQuantity25Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrstExctnOnly", skip_serializing_if = "Option::is_none") )]
	pub frst_exctn_only: Option<bool>,
}

impl MinimumExecutable1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.sz { val.validate()? }
		Ok(())
	}
}


// NewOrderReport2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NewOrderReport2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptId") )]
	pub rpt_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ordr") )]
	pub ordr: Vec<OrderData3>,
}

impl NewOrderReport2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.rpt_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "rpt_id is shorter than the minimum length of 1".to_string()));
		}
		if self.rpt_id.chars().count() > 140 {
			return Err(ValidationError::new(1002, "rpt_id exceeds the maximum length of 140".to_string()));
		}
		for item in &self.ordr { item.validate()? }
		Ok(())
	}
}


// NoReasonCode ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum NoReasonCode {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NORE") )]
	CodeNORE,
}

impl NoReasonCode {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OrderBookReportV01 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OrderBookReportV01 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptHdr") )]
	pub rpt_hdr: SecuritiesMarketReportHeader3,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrdrRpt") )]
	pub ordr_rpt: Vec<OrderReport2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl OrderBookReportV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rpt_hdr.validate()?;
		for item in &self.ordr_rpt { item.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OrderClassification2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OrderClassification2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrdrTp", skip_serializing_if = "Option::is_none") )]
	pub ordr_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrdrTpClssfctn", skip_serializing_if = "Option::is_none") )]
	pub ordr_tp_clssfctn: Option<OrderType3Code>,
}

impl OrderClassification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ordr_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ordr_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 50 {
				return Err(ValidationError::new(1002, "ordr_tp exceeds the maximum length of 50".to_string()));
			}
		}
		if let Some(ref val) = self.ordr_tp_clssfctn { val.validate()? }
		Ok(())
	}
}


// OrderData3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OrderData3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrdrIdData") )]
	pub ordr_id_data: OrderIdentification2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AuctnData", skip_serializing_if = "Option::is_none") )]
	pub auctn_data: Option<AuctionData2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrdrData", skip_serializing_if = "Option::is_none") )]
	pub ordr_data: Option<OrderData4>,
}

impl OrderData3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.ordr_id_data.validate()?;
		if let Some(ref val) = self.auctn_data { val.validate()? }
		if let Some(ref val) = self.ordr_data { val.validate()? }
		Ok(())
	}
}


// OrderData4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OrderData4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubmitgNtty", skip_serializing_if = "Option::is_none") )]
	pub submitg_ntty: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DrctElctrncAccs", skip_serializing_if = "Option::is_none") )]
	pub drct_elctrnc_accs: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClntId", skip_serializing_if = "Option::is_none") )]
	pub clnt_id: Option<PersonOrOrganisation4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstmtDcsnPrsn", skip_serializing_if = "Option::is_none") )]
	pub invstmt_dcsn_prsn: Option<ExecutingParty2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExctgPrsn", skip_serializing_if = "Option::is_none") )]
	pub exctg_prsn: Option<ExecutingParty2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NonExctgBrkr", skip_serializing_if = "Option::is_none") )]
	pub non_exctg_brkr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgCpcty", skip_serializing_if = "Option::is_none") )]
	pub tradg_cpcty: Option<RegulatoryTradingCapacity1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LqdtyPrvsnActvty", skip_serializing_if = "Option::is_none") )]
	pub lqdty_prvsn_actvty: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrdrClssfctn", skip_serializing_if = "Option::is_none") )]
	pub ordr_clssfctn: Option<OrderClassification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrdrPrics", skip_serializing_if = "Option::is_none") )]
	pub ordr_prics: Option<OrderPriceData2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrData", skip_serializing_if = "Option::is_none") )]
	pub instr_data: Option<OrderInstructionData2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxData", skip_serializing_if = "Option::is_none") )]
	pub tx_data: Option<TransactionData3>,
}

impl OrderData4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.submitg_ntty {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "submitg_ntty does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.clnt_id { val.validate()? }
		if let Some(ref val) = self.invstmt_dcsn_prsn { val.validate()? }
		if let Some(ref val) = self.exctg_prsn { val.validate()? }
		if let Some(ref val) = self.non_exctg_brkr {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "non_exctg_brkr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.tradg_cpcty { val.validate()? }
		if let Some(ref val) = self.ordr_clssfctn { val.validate()? }
		if let Some(ref val) = self.ordr_prics { val.validate()? }
		if let Some(ref val) = self.instr_data { val.validate()? }
		if let Some(ref val) = self.tx_data { val.validate()? }
		Ok(())
	}
}


// OrderEventType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OrderEventType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<OrderEventType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl OrderEventType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// OrderEventType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum OrderEventType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CAME") )]
	CodeCAME,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CAMO") )]
	CodeCAMO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CHME") )]
	CodeCHME,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CHMO") )]
	CodeCHMO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EXPI") )]
	CodeEXPI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FILL") )]
	CodeFILL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NEWO") )]
	CodeNEWO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PARF") )]
	CodePARF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REMA") )]
	CodeREMA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REMO") )]
	CodeREMO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REMH") )]
	CodeREMH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REME") )]
	CodeREME,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRIG") )]
	CodeTRIG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RFQS") )]
	CodeRFQS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RFQR") )]
	CodeRFQR,
}

impl OrderEventType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OrderIdentification2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OrderIdentification2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrdrBookId") )]
	pub ordr_book_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SeqNb") )]
	pub seq_nb: f64,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prty", skip_serializing_if = "Option::is_none") )]
	pub prty: Option<OrderPriority1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TmStmp") )]
	pub tm_stmp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradVn") )]
	pub trad_vn: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrm") )]
	pub fin_instrm: FinancialInstrument99Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrdrId", skip_serializing_if = "Option::is_none") )]
	pub ordr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtOfRct", skip_serializing_if = "Option::is_none") )]
	pub dt_of_rct: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldtyPrd", skip_serializing_if = "Option::is_none") )]
	pub vldty_prd: Option<ValidityPeriod1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrdrRstrctn", skip_serializing_if = "Option::is_none") )]
	pub ordr_rstrctn: Option<Vec<OrderRestriction1Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldtyDtTm", skip_serializing_if = "Option::is_none") )]
	pub vldty_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EvtTp", skip_serializing_if = "Option::is_none") )]
	pub evt_tp: Option<OrderEventType1Choice>,
}

impl OrderIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.ordr_book_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "ordr_book_id is shorter than the minimum length of 1".to_string()));
		}
		if self.ordr_book_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "ordr_book_id exceeds the maximum length of 35".to_string()));
		}
		if self.seq_nb < 1.000000 {
			return Err(ValidationError::new(1003, "seq_nb is less than the minimum value of 1.000000".to_string()));
		}
		if let Some(ref val) = self.prty { val.validate()? }
		let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
		if !pattern.is_match(&self.trad_vn) {
			return Err(ValidationError::new(1005, "trad_vn does not match the required pattern".to_string()));
		}
		self.fin_instrm.validate()?;
		if let Some(ref val) = self.ordr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ordr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 50 {
				return Err(ValidationError::new(1002, "ordr_id exceeds the maximum length of 50".to_string()));
			}
		}
		if let Some(ref val) = self.vldty_prd { val.validate()? }
		if let Some(ref vec) = self.ordr_rstrctn { for item in vec { item.validate()? } }
		if let Some(ref val) = self.evt_tp { val.validate()? }
		Ok(())
	}
}


// OrderInstructionData2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OrderInstructionData2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "BuySellInd", skip_serializing_if = "Option::is_none") )]
	pub buy_sell_ind: Option<Side6Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrdrVldtySts", skip_serializing_if = "Option::is_none") )]
	pub ordr_vldty_sts: Option<OrderStatus10Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrdrSts", skip_serializing_if = "Option::is_none") )]
	pub ordr_sts: Option<Vec<OrderStatus11Code>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitlQty", skip_serializing_if = "Option::is_none") )]
	pub initl_qty: Option<FinancialInstrumentQuantity25Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmngQty", skip_serializing_if = "Option::is_none") )]
	pub rmng_qty: Option<FinancialInstrumentQuantity25Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DispdQty", skip_serializing_if = "Option::is_none") )]
	pub dispd_qty: Option<FinancialInstrumentQuantity25Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinAccptblQty", skip_serializing_if = "Option::is_none") )]
	pub min_accptbl_qty: Option<FinancialInstrumentQuantity25Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinExctbl", skip_serializing_if = "Option::is_none") )]
	pub min_exctbl: Option<MinimumExecutable1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PssvOnlyInd", skip_serializing_if = "Option::is_none") )]
	pub pssv_only_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SlfExctnPrvntn", skip_serializing_if = "Option::is_none") )]
	pub slf_exctn_prvntn: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RtgStrtgy", skip_serializing_if = "Option::is_none") )]
	pub rtg_strtgy: Option<String>,
}

impl OrderInstructionData2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.buy_sell_ind { val.validate()? }
		if let Some(ref val) = self.ordr_vldty_sts { val.validate()? }
		if let Some(ref vec) = self.ordr_sts { for item in vec { item.validate()? } }
		if let Some(ref val) = self.initl_qty { val.validate()? }
		if let Some(ref val) = self.rmng_qty { val.validate()? }
		if let Some(ref val) = self.dispd_qty { val.validate()? }
		if let Some(ref val) = self.min_accptbl_qty { val.validate()? }
		if let Some(ref val) = self.min_exctbl { val.validate()? }
		if let Some(ref val) = self.rtg_strtgy {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rtg_strtgy is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 50 {
				return Err(ValidationError::new(1002, "rtg_strtgy exceeds the maximum length of 50".to_string()));
			}
		}
		Ok(())
	}
}


// OrderPriceData2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OrderPriceData2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "LmtPric", skip_serializing_if = "Option::is_none") )]
	pub lmt_pric: Option<SecuritiesTransactionPrice2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StopPric", skip_serializing_if = "Option::is_none") )]
	pub stop_pric: Option<SecuritiesTransactionPrice2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlLmtPric", skip_serializing_if = "Option::is_none") )]
	pub addtl_lmt_pric: Option<SecuritiesTransactionPrice2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PggdPric", skip_serializing_if = "Option::is_none") )]
	pub pggd_pric: Option<SecuritiesTransactionPrice2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CcyScndLeg", skip_serializing_if = "Option::is_none") )]
	pub ccy_scnd_leg: Option<String>,
}

impl OrderPriceData2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.lmt_pric { val.validate()? }
		if let Some(ref val) = self.stop_pric { val.validate()? }
		if let Some(ref val) = self.addtl_lmt_pric { val.validate()? }
		if let Some(ref val) = self.pggd_pric { val.validate()? }
		if let Some(ref val) = self.ccy_scnd_leg {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ccy_scnd_leg does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// OrderPriority1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OrderPriority1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TmStmp", skip_serializing_if = "Option::is_none") )]
	pub tm_stmp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sz", skip_serializing_if = "Option::is_none") )]
	pub sz: Option<f64>,
}

impl OrderPriority1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.sz {
			if *val < 1.000000 {
				return Err(ValidationError::new(1003, "sz is less than the minimum value of 1.000000".to_string()));
			}
		}
		Ok(())
	}
}


// OrderReport2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OrderReport2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "New", skip_serializing_if = "Option::is_none") )]
	pub new: Option<NewOrderReport2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cxl", skip_serializing_if = "Option::is_none") )]
	pub cxl: Option<CancelOrderReport1>,
}

impl OrderReport2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.new { val.validate()? }
		if let Some(ref val) = self.cxl { val.validate()? }
		Ok(())
	}
}


// OrderRestriction1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OrderRestriction1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrdrRstrctnCd", skip_serializing_if = "Option::is_none") )]
	pub ordr_rstrctn_cd: Option<OrderRestrictionType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl OrderRestriction1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ordr_rstrctn_cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// OrderRestrictionType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum OrderRestrictionType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SESR") )]
	CodeSESR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VFAR") )]
	CodeVFAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VFCR") )]
	CodeVFCR,
}

impl OrderRestrictionType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OrderStatus10Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum OrderStatus10Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACTI") )]
	CodeACTI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INAC") )]
	CodeINAC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SUSP") )]
	CodeSUSP,
}

impl OrderStatus10Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OrderStatus11Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum OrderStatus11Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FIRM") )]
	CodeFIRM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IMPL") )]
	CodeIMPL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INDI") )]
	CodeINDI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ROUT") )]
	CodeROUT,
}

impl OrderStatus11Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OrderType3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum OrderType3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "LMTO") )]
	CodeLMTO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "STOP") )]
	CodeSTOP,
}

impl OrderType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Pagination1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Pagination1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PgNb") )]
	pub pg_nb: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LastPgInd") )]
	pub last_pg_ind: bool,
}

impl Pagination1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{1,5}").unwrap();
		if !pattern.is_match(&self.pg_nb) {
			return Err(ValidationError::new(1005, "pg_nb does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// PartyExceptionType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum PartyExceptionType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "AGGR") )]
	CodeAGGR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PNAL") )]
	CodePNAL,
}

impl PartyExceptionType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PassiveOrAgressiveType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum PassiveOrAgressiveType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "AGRE") )]
	CodeAGRE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PASV") )]
	CodePASV,
}

impl PassiveOrAgressiveType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Period11Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Period11Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt", skip_serializing_if = "Option::is_none") )]
	pub fr_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt", skip_serializing_if = "Option::is_none") )]
	pub to_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrToDt", skip_serializing_if = "Option::is_none") )]
	pub fr_to_dt: Option<Period2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrToDtTm", skip_serializing_if = "Option::is_none") )]
	pub fr_to_dt_tm: Option<DateTimePeriod1>,
}

impl Period11Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.fr_to_dt { val.validate()? }
		if let Some(ref val) = self.fr_to_dt_tm { val.validate()? }
		Ok(())
	}
}


// Period2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Period2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt") )]
	pub fr_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt") )]
	pub to_dt: String,
}

impl Period2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PersonIdentificationSchemeName1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PersonIdentificationSchemeName1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl PersonIdentificationSchemeName1Choice {
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


// PersonOrOrganisation4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PersonOrOrganisation4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prsn", skip_serializing_if = "Option::is_none") )]
	pub prsn: Option<GenericPersonIdentification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XcptnId", skip_serializing_if = "Option::is_none") )]
	pub xcptn_id: Option<PartyExceptionType1Code>,
}

impl PersonOrOrganisation4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prsn { val.validate()? }
		if let Some(ref val) = self.xcptn_id { val.validate()? }
		Ok(())
	}
}


// PriceStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum PriceStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "PNDG") )]
	CodePNDG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NOAP") )]
	CodeNOAP,
}

impl PriceStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RegulatoryTradingCapacity1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum RegulatoryTradingCapacity1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MTCH") )]
	CodeMTCH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DEAL") )]
	CodeDEAL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AOTC") )]
	CodeAOTC,
}

impl RegulatoryTradingCapacity1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SecuritiesMarketReportHeader3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesMarketReportHeader3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgNtty") )]
	pub rptg_ntty: TradingVenueIdentification1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgPrd") )]
	pub rptg_prd: Period11Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
	pub isin: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubmissnDtTm", skip_serializing_if = "Option::is_none") )]
	pub submissn_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgPgntn", skip_serializing_if = "Option::is_none") )]
	pub msg_pgntn: Option<Pagination1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbRcrds", skip_serializing_if = "Option::is_none") )]
	pub nb_rcrds: Option<f64>,
}

impl SecuritiesMarketReportHeader3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rptg_ntty.validate()?;
		self.rptg_prd.validate()?;
		if let Some(ref vec) = self.isin {
			for item in vec {
				let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
				if !pattern.is_match(&item) {
					return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
				}
			}
		}
		if let Some(ref val) = self.msg_pgntn { val.validate()? }
		Ok(())
	}
}


// SecuritiesTransactionPrice1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesTransactionPrice1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pdg") )]
	pub pdg: PriceStatus1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy", skip_serializing_if = "Option::is_none") )]
	pub ccy: Option<String>,
}

impl SecuritiesTransactionPrice1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pdg.validate()?;
		if let Some(ref val) = self.ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// SecuritiesTransactionPrice21Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesTransactionPrice21Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MntryVal", skip_serializing_if = "Option::is_none") )]
	pub mntry_val: Option<AmountAndDirection53>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pctg", skip_serializing_if = "Option::is_none") )]
	pub pctg: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Yld", skip_serializing_if = "Option::is_none") )]
	pub yld: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BsisPts", skip_serializing_if = "Option::is_none") )]
	pub bsis_pts: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmnlVal", skip_serializing_if = "Option::is_none") )]
	pub nmnl_val: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl SecuritiesTransactionPrice21Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mntry_val { val.validate()? }
		if let Some(ref val) = self.nmnl_val { val.validate()? }
		Ok(())
	}
}


// SecuritiesTransactionPrice2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesTransactionPrice2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MntryVal", skip_serializing_if = "Option::is_none") )]
	pub mntry_val: Option<AmountAndDirection61>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pctg", skip_serializing_if = "Option::is_none") )]
	pub pctg: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Yld", skip_serializing_if = "Option::is_none") )]
	pub yld: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BsisPts", skip_serializing_if = "Option::is_none") )]
	pub bsis_pts: Option<f64>,
}

impl SecuritiesTransactionPrice2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mntry_val { val.validate()? }
		Ok(())
	}
}


// SecuritiesTransactionPrice4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesTransactionPrice4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pric", skip_serializing_if = "Option::is_none") )]
	pub pric: Option<SecuritiesTransactionPrice2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NoPric", skip_serializing_if = "Option::is_none") )]
	pub no_pric: Option<SecuritiesTransactionPrice1>,
}

impl SecuritiesTransactionPrice4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pric { val.validate()? }
		if let Some(ref val) = self.no_pric { val.validate()? }
		Ok(())
	}
}


// Side6Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum Side6Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "BUYI") )]
	CodeBUYI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SELL") )]
	CodeSELL,
}

impl Side6Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
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
		self.envlp.validate()?;
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


// TradingVenue2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TradingVenue2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "APPA") )]
	CodeAPPA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CTPS") )]
	CodeCTPS,
}

impl TradingVenue2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TradingVenueIdentification1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradingVenueIdentification1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktIdCd", skip_serializing_if = "Option::is_none") )]
	pub mkt_id_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NtlCmptntAuthrty", skip_serializing_if = "Option::is_none") )]
	pub ntl_cmptnt_authrty: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<TradingVenueIdentification2>,
}

impl TradingVenueIdentification1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mkt_id_cd {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "mkt_id_cd does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ntl_cmptnt_authrty {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ntl_cmptnt_authrty does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.othr { val.validate()? }
		Ok(())
	}
}


// TradingVenueIdentification2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradingVenueIdentification2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: TradingVenue2Code,
}

impl TradingVenueIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 50 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 50".to_string()));
		}
		self.tp.validate()?;
		Ok(())
	}
}


// TransactionData3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TransactionData3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxPric", skip_serializing_if = "Option::is_none") )]
	pub tx_pric: Option<SecuritiesTransactionPrice4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TraddQty", skip_serializing_if = "Option::is_none") )]
	pub tradd_qty: Option<FinancialInstrumentQuantity25Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PssvOrAggrssvInd", skip_serializing_if = "Option::is_none") )]
	pub pssv_or_aggrssv_ind: Option<PassiveOrAgressiveType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StrtgyLkdOrdrId", skip_serializing_if = "Option::is_none") )]
	pub strtgy_lkd_ordr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxId", skip_serializing_if = "Option::is_none") )]
	pub tx_id: Option<String>,
}

impl TransactionData3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tx_pric { val.validate()? }
		if let Some(ref val) = self.tradd_qty { val.validate()? }
		if let Some(ref val) = self.pssv_or_aggrssv_ind { val.validate()? }
		if let Some(ref val) = self.strtgy_lkd_ordr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "strtgy_lkd_ordr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 50 {
				return Err(ValidationError::new(1002, "strtgy_lkd_ordr_id exceeds the maximum length of 50".to_string()));
			}
		}
		if let Some(ref val) = self.tx_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tx_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 52 {
				return Err(ValidationError::new(1002, "tx_id exceeds the maximum length of 52".to_string()));
			}
		}
		Ok(())
	}
}


// ValidityPeriod1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ValidityPeriod1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldtyPrdCd", skip_serializing_if = "Option::is_none") )]
	pub vldty_prd_cd: Option<ValidityPeriodType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl ValidityPeriod1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.vldty_prd_cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// ValidityPeriodType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ValidityPeriodType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FOKV") )]
	CodeFOKV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GADV") )]
	CodeGADV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GASV") )]
	CodeGASV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GATV") )]
	CodeGATV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DAVY") )]
	CodeDAVY,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GTCV") )]
	CodeGTCV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GTDV") )]
	CodeGTDV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GTSV") )]
	CodeGTSV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GTTV") )]
	CodeGTTV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IOCV") )]
	CodeIOCV,
}

impl ValidityPeriodType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}
