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


// BrokeredDeal1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum BrokeredDeal1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "BILA") )]
	CodeBILA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BROK") )]
	CodeBROK,
}

impl BrokeredDeal1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CounterpartyIdentification3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CounterpartyIdentification3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctrAndLctn", skip_serializing_if = "Option::is_none") )]
	pub sctr_and_lctn: Option<SectorAndLocation1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndLctn", skip_serializing_if = "Option::is_none") )]
	pub nm_and_lctn: Option<NameAndLocation1>,
}

impl CounterpartyIdentification3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.sctr_and_lctn { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.nm_and_lctn { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// DateAndDateTimeChoice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DateAndDateTimeChoice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtTm", skip_serializing_if = "Option::is_none") )]
	pub dt_tm: Option<String>,
}

impl DateAndDateTimeChoice {
	pub fn validate(&self) -> Result<(), ValidationError> {
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


// FinancialInstrumentProductType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum FinancialInstrumentProductType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CEOD") )]
	CodeCEOD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "COPR") )]
	CodeCOPR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ABCP") )]
	CodeABCP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FRNT") )]
	CodeFRNT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CACM") )]
	CodeCACM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DPST") )]
	CodeDPST,
}

impl FinancialInstrumentProductType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FloatingRateNote2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FloatingRateNote2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefRateIndx") )]
	pub ref_rate_indx: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BsisPtSprd") )]
	pub bsis_pt_sprd: f64,
}

impl FloatingRateNote2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
		if !pattern.is_match(&self.ref_rate_indx) {
			return Err(ValidationError::new(1005, "ref_rate_indx does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// InterestRateType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum InterestRateType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FIXE") )]
	CodeFIXE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VARI") )]
	CodeVARI,
}

impl InterestRateType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// MoneyMarketReportHeader1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MoneyMarketReportHeader1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgAgt") )]
	pub rptg_agt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefPrd") )]
	pub ref_prd: DateTimePeriod1,
}

impl MoneyMarketReportHeader1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.rptg_agt) {
			return Err(ValidationError::new(1005, "rptg_agt does not match the required pattern".to_string()));
		}
		if let Err(e) = self.ref_prd.validate() { return Err(e); }
		Ok(())
	}
}


// MoneyMarketTransactionType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum MoneyMarketTransactionType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "BORR") )]
	CodeBORR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEND") )]
	CodeLEND,
}

impl MoneyMarketTransactionType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// MoneyMarketUnsecuredMarketStatisticalReportV02 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MoneyMarketUnsecuredMarketStatisticalReportV02 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptHdr") )]
	pub rpt_hdr: MoneyMarketReportHeader1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UscrdMktRpt") )]
	pub uscrd_mkt_rpt: UnsecuredMarketReport4Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl MoneyMarketUnsecuredMarketStatisticalReportV02 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.rpt_hdr.validate() { return Err(e); }
		if let Err(e) = self.uscrd_mkt_rpt.validate() { return Err(e); }
		if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// NameAndLocation1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NameAndLocation1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Lctn") )]
	pub lctn: String,
}

impl NameAndLocation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 70 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 70".to_string()));
		}
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.lctn) {
			return Err(ValidationError::new(1005, "lctn does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// NovationStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum NovationStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NONO") )]
	CodeNONO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NOVA") )]
	CodeNOVA,
}

impl NovationStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Option12 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Option12 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: OptionType1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtOrPrd") )]
	pub dt_or_prd: OptionDateOrPeriod1Choice,
}

impl Option12 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.tp.validate() { return Err(e); }
		if let Err(e) = self.dt_or_prd.validate() { return Err(e); }
		Ok(())
	}
}


// OptionDateOrPeriod1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OptionDateOrPeriod1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "EarlstExrcDt", skip_serializing_if = "Option::is_none") )]
	pub earlst_exrc_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NtcePrd", skip_serializing_if = "Option::is_none") )]
	pub ntce_prd: Option<f64>,
}

impl OptionDateOrPeriod1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OptionType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum OptionType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CALL") )]
	CodeCALL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUTO") )]
	CodePUTO,
}

impl OptionType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ReportPeriodActivity3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ReportPeriodActivity3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NOTX") )]
	CodeNOTX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NORA") )]
	CodeNORA,
}

impl ReportPeriodActivity3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SectorAndLocation1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SectorAndLocation1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sctr") )]
	pub sctr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Lctn") )]
	pub lctn: String,
}

impl SectorAndLocation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.lctn) {
			return Err(ValidationError::new(1005, "lctn does not match the required pattern".to_string()));
		}
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


// TransactionOperationType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TransactionOperationType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "AMND") )]
	CodeAMND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CANC") )]
	CodeCANC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CORR") )]
	CodeCORR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NEWT") )]
	CodeNEWT,
}

impl TransactionOperationType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// UnsecuredMarketReport4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UnsecuredMarketReport4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
	pub data_set_actn: Option<ReportPeriodActivity3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tx", skip_serializing_if = "Option::is_none") )]
	pub tx: Option<Vec<UnsecuredMarketTransaction4>>,
}

impl UnsecuredMarketReport4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.data_set_actn { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.tx { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// UnsecuredMarketTransaction4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UnsecuredMarketTransaction4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptdTxSts") )]
	pub rptd_tx_sts: TransactionOperationType1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NvtnSts", skip_serializing_if = "Option::is_none") )]
	pub nvtn_sts: Option<NovationStatus1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BrnchId", skip_serializing_if = "Option::is_none") )]
	pub brnch_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnqTxIdr", skip_serializing_if = "Option::is_none") )]
	pub unq_tx_idr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryTxId") )]
	pub prtry_tx_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdPrtryTxId", skip_serializing_if = "Option::is_none") )]
	pub rltd_prtry_tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtyPrtryTxId", skip_serializing_if = "Option::is_none") )]
	pub ctr_pty_prtry_tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtyId") )]
	pub ctr_pty_id: CounterpartyIdentification3Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradDt") )]
	pub trad_dt: DateAndDateTimeChoice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmDt") )]
	pub sttlm_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MtrtyDt") )]
	pub mtrty_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxTp") )]
	pub tx_tp: MoneyMarketTransactionType1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrmTp") )]
	pub instrm_tp: FinancialInstrumentProductType1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxNmnlAmt") )]
	pub tx_nmnl_amt: ActiveCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealPric") )]
	pub deal_pric: f64,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RateTp") )]
	pub rate_tp: InterestRateType1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealRate", skip_serializing_if = "Option::is_none") )]
	pub deal_rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FltgRateNote", skip_serializing_if = "Option::is_none") )]
	pub fltg_rate_note: Option<FloatingRateNote2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BrkrdDeal", skip_serializing_if = "Option::is_none") )]
	pub brkrd_deal: Option<BrokeredDeal1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CallPutOptn", skip_serializing_if = "Option::is_none") )]
	pub call_put_optn: Option<Vec<Option12>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl UnsecuredMarketTransaction4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.rptd_tx_sts.validate() { return Err(e); }
		if let Some(ref val) = self.nvtn_sts { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.brnch_id {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "brnch_id does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.unq_tx_idr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "unq_tx_idr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 105 {
				return Err(ValidationError::new(1002, "unq_tx_idr exceeds the maximum length of 105".to_string()));
			}
		}
		if self.prtry_tx_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "prtry_tx_id is shorter than the minimum length of 1".to_string()));
		}
		if self.prtry_tx_id.chars().count() > 105 {
			return Err(ValidationError::new(1002, "prtry_tx_id exceeds the maximum length of 105".to_string()));
		}
		if let Some(ref val) = self.rltd_prtry_tx_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rltd_prtry_tx_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 105 {
				return Err(ValidationError::new(1002, "rltd_prtry_tx_id exceeds the maximum length of 105".to_string()));
			}
		}
		if let Some(ref val) = self.ctr_pty_prtry_tx_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ctr_pty_prtry_tx_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 105 {
				return Err(ValidationError::new(1002, "ctr_pty_prtry_tx_id exceeds the maximum length of 105".to_string()));
			}
		}
		if let Err(e) = self.ctr_pty_id.validate() { return Err(e); }
		if let Err(e) = self.trad_dt.validate() { return Err(e); }
		if let Err(e) = self.tx_tp.validate() { return Err(e); }
		if let Err(e) = self.instrm_tp.validate() { return Err(e); }
		if let Err(e) = self.tx_nmnl_amt.validate() { return Err(e); }
		if let Err(e) = self.rate_tp.validate() { return Err(e); }
		if let Some(ref val) = self.fltg_rate_note { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.brkrd_deal { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.call_put_optn { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}
