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


// ContactDetails4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ContactDetails4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PhneNb") )]
	pub phne_nb: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EmailAdr") )]
	pub email_adr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Fctn") )]
	pub fctn: String,
}

impl ContactDetails4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 140 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
		}
		let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
		if !pattern.is_match(&self.phne_nb) {
			return Err(ValidationError::new(1005, "phne_nb does not match the required pattern".to_string()));
		}
		if self.email_adr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "email_adr is shorter than the minimum length of 1".to_string()));
		}
		if self.email_adr.chars().count() > 2048 {
			return Err(ValidationError::new(1002, "email_adr exceeds the maximum length of 2048".to_string()));
		}
		if self.fctn.chars().count() < 1 {
			return Err(ValidationError::new(1001, "fctn is shorter than the minimum length of 1".to_string()));
		}
		if self.fctn.chars().count() > 140 {
			return Err(ValidationError::new(1002, "fctn exceeds the maximum length of 140".to_string()));
		}
		Ok(())
	}
}


// InternalisationData1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InternalisationData1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Aggt") )]
	pub aggt: InternalisationData2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FaildRate") )]
	pub faild_rate: InternalisationDataRate1,
}

impl InternalisationData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.aggt.validate()?;
		self.faild_rate.validate()?;
		Ok(())
	}
}


// InternalisationData2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InternalisationData2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sttld") )]
	pub sttld: InternalisationDataVolume1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Faild") )]
	pub faild: InternalisationDataVolume1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ttl") )]
	pub ttl: InternalisationDataVolume1,
}

impl InternalisationData2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.sttld.validate()?;
		self.faild.validate()?;
		self.ttl.validate()?;
		Ok(())
	}
}


// InternalisationDataRate1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InternalisationDataRate1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "VolPctg") )]
	pub vol_pctg: f64,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
	pub val: f64,
}

impl InternalisationDataRate1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InternalisationDataVolume1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InternalisationDataVolume1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Vol") )]
	pub vol: f64,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
	pub val: f64,
}

impl InternalisationDataVolume1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.vol < 0.000000 {
			return Err(ValidationError::new(1003, "vol is less than the minimum value of 0.000000".to_string()));
		}
		if self.val < 0.000000 {
			return Err(ValidationError::new(1003, "val is less than the minimum value of 0.000000".to_string()));
		}
		Ok(())
	}
}


// IssuerCSDIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct IssuerCSDIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrstTwoCharsInstrmId") )]
	pub frst_two_chars_instrm_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
	pub ctry: Option<String>,
}

impl IssuerCSDIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		let pattern = Regex::new("[A-Z]{2}").unwrap();
		if !pattern.is_match(&self.frst_two_chars_instrm_id) {
			return Err(ValidationError::new(1005, "frst_two_chars_instrm_id does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// IssuerCSDReport1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct IssuerCSDReport1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: IssuerCSDIdentification1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OvrllTtl") )]
	pub ovrll_ttl: InternalisationData1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrm") )]
	pub fin_instrm: SettlementInternaliserFinancialInstrument1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxTp") )]
	pub tx_tp: SettlementInternaliserTransactionType1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClntTp") )]
	pub clnt_tp: SettlementInternaliserClientType1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlCshTrf") )]
	pub ttl_csh_trf: InternalisationData1,
}

impl IssuerCSDReport1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		self.ovrll_ttl.validate()?;
		self.fin_instrm.validate()?;
		self.tx_tp.validate()?;
		self.clnt_tp.validate()?;
		self.ttl_csh_trf.validate()?;
		Ok(())
	}
}


// SettlementInternaliser1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementInternaliser1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: SettlementInternaliserIdentification1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OvrllTtl") )]
	pub ovrll_ttl: InternalisationData1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrm") )]
	pub fin_instrm: SettlementInternaliserFinancialInstrument1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxTp") )]
	pub tx_tp: SettlementInternaliserTransactionType1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClntTp") )]
	pub clnt_tp: SettlementInternaliserClientType1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlCshTrf") )]
	pub ttl_csh_trf: InternalisationData1,
}

impl SettlementInternaliser1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		self.ovrll_ttl.validate()?;
		self.fin_instrm.validate()?;
		self.tx_tp.validate()?;
		self.clnt_tp.validate()?;
		self.ttl_csh_trf.validate()?;
		Ok(())
	}
}


// SettlementInternaliserClientType1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementInternaliserClientType1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prfssnl") )]
	pub prfssnl: InternalisationData1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rtl") )]
	pub rtl: InternalisationData1,
}

impl SettlementInternaliserClientType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.prfssnl.validate()?;
		self.rtl.validate()?;
		Ok(())
	}
}


// SettlementInternaliserFinancialInstrument1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementInternaliserFinancialInstrument1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Eqty") )]
	pub eqty: InternalisationData1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SvrgnDebt") )]
	pub svrgn_debt: InternalisationData1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Bd") )]
	pub bd: InternalisationData1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrTrfblScties") )]
	pub othr_trfbl_scties: InternalisationData1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XchgTradgFnds") )]
	pub xchg_tradg_fnds: InternalisationData1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CllctvInvstmtUdrtkgs") )]
	pub cllctv_invstmt_udrtkgs: InternalisationData1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MnyMktInstrm") )]
	pub mny_mkt_instrm: InternalisationData1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EmssnAllwnc") )]
	pub emssn_allwnc: InternalisationData1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrFinInstrms") )]
	pub othr_fin_instrms: InternalisationData1,
}

impl SettlementInternaliserFinancialInstrument1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.eqty.validate()?;
		self.svrgn_debt.validate()?;
		self.bd.validate()?;
		self.othr_trfbl_scties.validate()?;
		self.xchg_tradg_fnds.validate()?;
		self.cllctv_invstmt_udrtkgs.validate()?;
		self.mny_mkt_instrm.validate()?;
		self.emssn_allwnc.validate()?;
		self.othr_fin_instrms.validate()?;
		Ok(())
	}
}


// SettlementInternaliserIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementInternaliserIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI") )]
	pub lei: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RspnsblPrsn") )]
	pub rspnsbl_prsn: ContactDetails4,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
	pub ctry: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BrnchId", skip_serializing_if = "Option::is_none") )]
	pub brnch_id: Option<String>,
}

impl SettlementInternaliserIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.lei) {
			return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
		}
		self.rspnsbl_prsn.validate()?;
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.ctry) {
			return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.brnch_id {
			let pattern = Regex::new("[A-Z]{2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "brnch_id does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// SettlementInternaliserReportHeader1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementInternaliserReportHeader1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
	pub cre_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgDt") )]
	pub rptg_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptSts") )]
	pub rpt_sts: TransactionOperationType4Code,
}

impl SettlementInternaliserReportHeader1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.ccy) {
			return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
		}
		self.rpt_sts.validate()?;
		Ok(())
	}
}


// SettlementInternaliserReportV01 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementInternaliserReportV01 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptHdr") )]
	pub rpt_hdr: SettlementInternaliserReportHeader1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmIntlr") )]
	pub sttlm_intlr: SettlementInternaliser1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IssrCSD") )]
	pub issr_csd: Vec<IssuerCSDReport1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl SettlementInternaliserReportV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rpt_hdr.validate()?;
		self.sttlm_intlr.validate()?;
		for item in &self.issr_csd { item.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// SettlementInternaliserTransactionType1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementInternaliserTransactionType1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesBuyOrSell") )]
	pub scties_buy_or_sell: InternalisationData1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollMgmtOpr") )]
	pub coll_mgmt_opr: InternalisationData1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesLndgOrBrrwg") )]
	pub scties_lndg_or_brrwg: InternalisationData1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RpAgrmt") )]
	pub rp_agrmt: InternalisationData1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrTxs") )]
	pub othr_txs: InternalisationData1,
}

impl SettlementInternaliserTransactionType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.scties_buy_or_sell.validate()?;
		self.coll_mgmt_opr.validate()?;
		self.scties_lndg_or_brrwg.validate()?;
		self.rp_agrmt.validate()?;
		self.othr_txs.validate()?;
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


// TransactionOperationType4Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TransactionOperationType4Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NEWT") )]
	CodeNEWT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AMND") )]
	CodeAMND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CANC") )]
	CodeCANC,
}

impl TransactionOperationType4Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}
