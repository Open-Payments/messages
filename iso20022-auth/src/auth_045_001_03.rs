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


// AssetClassAndSubClassIdentification2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AssetClassAndSubClassIdentification2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AsstClss") )]
	pub asst_clss: NonEquityAssetClass1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DerivSubClss", skip_serializing_if = "Option::is_none") )]
	pub deriv_sub_clss: Option<NonEquitySubClass1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmClssfctn", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_clssfctn: Option<NonEquityInstrumentReportingClassification1Code>,
}

impl AssetClassAndSubClassIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.asst_clss.validate()?;
		if let Some(ref val) = self.deriv_sub_clss { val.validate()? }
		if let Some(ref val) = self.fin_instrm_clssfctn { val.validate()? }
		Ok(())
	}
}


// FinancialInstrumentReportingNonEquityTradingActivityResultV03 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrumentReportingNonEquityTradingActivityResultV03 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptHdr") )]
	pub rpt_hdr: SecuritiesMarketReportHeader1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NonEqtyTrnsprncyData") )]
	pub non_eqty_trnsprncy_data: Vec<TransparencyDataReport20>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl FinancialInstrumentReportingNonEquityTradingActivityResultV03 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rpt_hdr.validate()?;
		for item in &self.non_eqty_trnsprncy_data { item.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// InstrumentAndSubClassIdentification2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InstrumentAndSubClassIdentification2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN") )]
	pub isin: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DerivSubClss", skip_serializing_if = "Option::is_none") )]
	pub deriv_sub_clss: Option<NonEquitySubClass1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmClssfctn", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_clssfctn: Option<NonEquityInstrumentReportingClassification1Code>,
}

impl InstrumentAndSubClassIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
		if !pattern.is_match(&self.isin) {
			return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.deriv_sub_clss { val.validate()? }
		if let Some(ref val) = self.fin_instrm_clssfctn { val.validate()? }
		Ok(())
	}
}


// InstrumentOrSubClassIdentification2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InstrumentOrSubClassIdentification2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISINAndSubClss", skip_serializing_if = "Option::is_none") )]
	pub isin_and_sub_clss: Option<InstrumentAndSubClassIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AsstClssAndSubClss", skip_serializing_if = "Option::is_none") )]
	pub asst_clss_and_sub_clss: Option<AssetClassAndSubClassIdentification2>,
}

impl InstrumentOrSubClassIdentification2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.isin_and_sub_clss { val.validate()? }
		if let Some(ref val) = self.asst_clss_and_sub_clss { val.validate()? }
		Ok(())
	}
}


// NonEquityAssetClass1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum NonEquityAssetClass1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SDRV") )]
	CodeSDRV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IRDV") )]
	CodeIRDV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FEXD") )]
	CodeFEXD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EQDV") )]
	CodeEQDV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EADV") )]
	CodeEADV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EMAL") )]
	CodeEMAL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRDV") )]
	CodeCRDV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CFDS") )]
	CodeCFDS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "COMD") )]
	CodeCOMD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "C10D") )]
	CodeC10D,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BOND") )]
	CodeBOND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ETCS") )]
	CodeETCS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ETNS") )]
	CodeETNS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SFPS") )]
	CodeSFPS,
}

impl NonEquityAssetClass1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// NonEquityInstrumentReportingClassification1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum NonEquityInstrumentReportingClassification1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SFPS") )]
	CodeSFPS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SDRV") )]
	CodeSDRV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DERV") )]
	CodeDERV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EMAL") )]
	CodeEMAL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BOND") )]
	CodeBOND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ETCS") )]
	CodeETCS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ETNS") )]
	CodeETNS,
}

impl NonEquityInstrumentReportingClassification1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// NonEquitySubClass1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NonEquitySubClass1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SgmttnCrit") )]
	pub sgmttn_crit: Vec<NonEquitySubClassSegmentationCriterion1>,
}

impl NonEquitySubClass1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 1000 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 1000".to_string()));
			}
		}
		for item in &self.sgmttn_crit { item.validate()? }
		Ok(())
	}
}


// NonEquitySubClassSegmentationCriteria1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum NonEquitySubClassSegmentationCriteria1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ASCL") )]
	CodeASCL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BSPD") )]
	CodeBSPD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CNC1") )]
	CodeCNC1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CNC2") )]
	CodeCNC2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NCCO") )]
	CodeNCCO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CTYP") )]
	CodeCTYP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NCCR") )]
	CodeNCCR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DCSL") )]
	CodeDCSL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DTYP") )]
	CodeDTYP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EQUT") )]
	CodeEQUT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FNC1") )]
	CodeFNC1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FNC2") )]
	CodeFNC2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FSPD") )]
	CodeFSPD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IIND") )]
	CodeIIND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IRTC") )]
	CodeIRTC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INC1") )]
	CodeINC1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INC2") )]
	CodeINC2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN") )]
	CodeISIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TTMO") )]
	CodeTTMO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRMT") )]
	CodePRMT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SSRF") )]
	CodeSSRF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISPT") )]
	CodeISPT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SRTC") )]
	CodeSRTC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SACL") )]
	CodeSACL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SBPD") )]
	CodeSBPD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TTMS") )]
	CodeTTMS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NCSW") )]
	CodeNCSW,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TTMB") )]
	CodeTTMB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IOUB") )]
	CodeIOUB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TOUB") )]
	CodeTOUB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UISC") )]
	CodeUISC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UIDX") )]
	CodeUIDX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UINS") )]
	CodeUINS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UIRT") )]
	CodeUIRT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REOU") )]
	CodeREOU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UTYP") )]
	CodeUTYP,
}

impl NonEquitySubClassSegmentationCriteria1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// NonEquitySubClassSegmentationCriterion1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NonEquitySubClassSegmentationCriterion1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CritNm") )]
	pub crit_nm: NonEquitySubClassSegmentationCriteria1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CritVal") )]
	pub crit_val: String,
}

impl NonEquitySubClassSegmentationCriterion1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.crit_nm.validate()?;
		if self.crit_val.chars().count() < 1 {
			return Err(ValidationError::new(1001, "crit_val is shorter than the minimum length of 1".to_string()));
		}
		if self.crit_val.chars().count() > 1000 {
			return Err(ValidationError::new(1002, "crit_val exceeds the maximum length of 1000".to_string()));
		}
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


// Period4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Period4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt", skip_serializing_if = "Option::is_none") )]
	pub fr_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt", skip_serializing_if = "Option::is_none") )]
	pub to_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDtToDt", skip_serializing_if = "Option::is_none") )]
	pub fr_dt_to_dt: Option<Period2>,
}

impl Period4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.fr_dt_to_dt { val.validate()? }
		Ok(())
	}
}


// SecuritiesMarketReportHeader1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesMarketReportHeader1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgNtty") )]
	pub rptg_ntty: TradingVenueIdentification1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgPrd") )]
	pub rptg_prd: Period4Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubmissnDtTm", skip_serializing_if = "Option::is_none") )]
	pub submissn_dt_tm: Option<String>,
}

impl SecuritiesMarketReportHeader1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rptg_ntty.validate()?;
		self.rptg_prd.validate()?;
		Ok(())
	}
}


// StatisticsTransparency2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct StatisticsTransparency2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfTxsExctd") )]
	pub ttl_nb_of_txs_exctd: f64,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlVolOfTxsExctd") )]
	pub ttl_vol_of_txs_exctd: f64,
}

impl StatisticsTransparency2 {
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


// TonsOrCurrency2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TonsOrCurrency2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nb", skip_serializing_if = "Option::is_none") )]
	pub nb: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl TonsOrCurrency2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.amt { val.validate()? }
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


// TransparencyDataReport20 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TransparencyDataReport20 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none") )]
	pub tech_rcrd_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: InstrumentOrSubClassIdentification2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FullNm", skip_serializing_if = "Option::is_none") )]
	pub full_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgVn", skip_serializing_if = "Option::is_none") )]
	pub tradg_vn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgPrd", skip_serializing_if = "Option::is_none") )]
	pub rptg_prd: Option<Period4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Lqdty", skip_serializing_if = "Option::is_none") )]
	pub lqdty: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PreTradLrgInScaleThrshld", skip_serializing_if = "Option::is_none") )]
	pub pre_trad_lrg_in_scale_thrshld: Option<TonsOrCurrency2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstTradLrgInScaleThrshld", skip_serializing_if = "Option::is_none") )]
	pub pst_trad_lrg_in_scale_thrshld: Option<TonsOrCurrency2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PreTradInstrmSzSpcfcThrshld", skip_serializing_if = "Option::is_none") )]
	pub pre_trad_instrm_sz_spcfc_thrshld: Option<TonsOrCurrency2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstTradInstrmSzSpcfcThrshld", skip_serializing_if = "Option::is_none") )]
	pub pst_trad_instrm_sz_spcfc_thrshld: Option<TonsOrCurrency2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sttstcs", skip_serializing_if = "Option::is_none") )]
	pub sttstcs: Option<StatisticsTransparency2>,
}

impl TransparencyDataReport20 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tech_rcrd_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tech_rcrd_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tech_rcrd_id exceeds the maximum length of 35".to_string()));
			}
		}
		self.id.validate()?;
		if let Some(ref val) = self.full_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "full_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "full_nm exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.tradg_vn {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "tradg_vn does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.rptg_prd { val.validate()? }
		if let Some(ref val) = self.pre_trad_lrg_in_scale_thrshld { val.validate()? }
		if let Some(ref val) = self.pst_trad_lrg_in_scale_thrshld { val.validate()? }
		if let Some(ref val) = self.pre_trad_instrm_sz_spcfc_thrshld { val.validate()? }
		if let Some(ref val) = self.pst_trad_instrm_sz_spcfc_thrshld { val.validate()? }
		if let Some(ref val) = self.sttstcs { val.validate()? }
		Ok(())
	}
}
