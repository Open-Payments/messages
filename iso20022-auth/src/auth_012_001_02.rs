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

use serde::{Deserialize, Serialize};
use regex::Regex;
use crate::validationerror::*;


// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveCurrencyAndAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "$value")]
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


// BrokeredDeal1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BrokeredDeal1Code {
	#[default]
	#[serde(rename = "BILA")]
	CodeBILA,
	#[serde(rename = "BROK")]
	CodeBROK,
}

impl BrokeredDeal1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CFIOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CFIOct2015Identifier {
	#[serde(rename = "$value")]
	pub cfi_oct2015_identifier: String,
}

impl CFIOct2015Identifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{6,6}").unwrap();
		if !pattern.is_match(&self.cfi_oct2015_identifier) {
			return Err(ValidationError::new(1005, "cfi_oct2015_identifier does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// Collateral18 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Collateral18 {
	#[serde(rename = "Valtn")]
	pub valtn: SecuredCollateral2Choice,
	#[serde(rename = "Hrcut", skip_serializing_if = "Option::is_none")]
	pub hrcut: Option<f64>,
	#[serde(rename = "SpclCollInd", skip_serializing_if = "Option::is_none")]
	pub spcl_coll_ind: Option<SpecialCollateral2Code>,
}

impl Collateral18 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.valtn.validate() { return Err(e); }
		if let Some(ref spcl_coll_ind_value) = self.spcl_coll_ind { if let Err(e) = spcl_coll_ind_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CollateralPool1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CollateralPool1Code {
	#[default]
	#[serde(rename = "NOPL")]
	CodeNOPL,
	#[serde(rename = "POOL")]
	CodePOOL,
}

impl CollateralPool1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CollateralValuation6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralValuation6 {
	#[serde(rename = "NmnlAmt", skip_serializing_if = "Option::is_none")]
	pub nmnl_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "ISIN")]
	pub isin: ISINOct2015Identifier,
}

impl CollateralValuation6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref nmnl_amt_value) = self.nmnl_amt { if let Err(e) = nmnl_amt_value.validate() { return Err(e); } }
		if let Err(e) = self.isin.validate() { return Err(e); }
		Ok(())
	}
}


// CollateralValuation7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralValuation7 {
	#[serde(rename = "PoolSts")]
	pub pool_sts: CollateralPool1Code,
	#[serde(rename = "Tp")]
	pub tp: CFIOct2015Identifier,
	#[serde(rename = "Sctr")]
	pub sctr: String,
	#[serde(rename = "NmnlAmt", skip_serializing_if = "Option::is_none")]
	pub nmnl_amt: Option<ActiveCurrencyAndAmount>,
}

impl CollateralValuation7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.pool_sts.validate() { return Err(e); }
		if let Err(e) = self.tp.validate() { return Err(e); }
		if let Some(ref nmnl_amt_value) = self.nmnl_amt { if let Err(e) = nmnl_amt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CounterpartyIdentification3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyIdentification3Choice {
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "SctrAndLctn", skip_serializing_if = "Option::is_none")]
	pub sctr_and_lctn: Option<SectorAndLocation1>,
	#[serde(rename = "NmAndLctn", skip_serializing_if = "Option::is_none")]
	pub nm_and_lctn: Option<NameAndLocation1>,
}

impl CounterpartyIdentification3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
		if let Some(ref sctr_and_lctn_value) = self.sctr_and_lctn { if let Err(e) = sctr_and_lctn_value.validate() { return Err(e); } }
		if let Some(ref nm_and_lctn_value) = self.nm_and_lctn { if let Err(e) = nm_and_lctn_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}

impl CountryCode {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.country_code) {
			return Err(ValidationError::new(1005, "country_code does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// DateAndDateTimeChoice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTimeChoice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<String>,
}

impl DateAndDateTimeChoice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DateTimePeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod1 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: String,
}

impl DateTimePeriod1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FloatingRateNote2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FloatingRateNote2 {
	#[serde(rename = "RefRateIndx")]
	pub ref_rate_indx: ISINOct2015Identifier,
	#[serde(rename = "BsisPtSprd")]
	pub bsis_pt_sprd: f64,
}

impl FloatingRateNote2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.ref_rate_indx.validate() { return Err(e); }
		Ok(())
	}
}


// ISINOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "$value")]
	pub isin_oct2015_identifier: String,
}

impl ISINOct2015Identifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
		if !pattern.is_match(&self.isin_oct2015_identifier) {
			return Err(ValidationError::new(1005, "isin_oct2015_identifier does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}

impl ISODate {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}

impl ISODateTime {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InterestRateType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InterestRateType1Code {
	#[default]
	#[serde(rename = "FIXE")]
	CodeFIXE,
	#[serde(rename = "VARI")]
	CodeVARI,
}

impl InterestRateType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}

impl LEIIdentifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.lei_identifier) {
			return Err(ValidationError::new(1005, "lei_identifier does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// Max105Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max105Text {
	#[serde(rename = "$value")]
	pub max105_text: String,
}

impl Max105Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max105_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max105_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max105_text.chars().count() > 105 {
			return Err(ValidationError::new(1002, "max105_text exceeds the maximum length of 105".to_string()));
		}
		Ok(())
	}
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max350Text {
	#[serde(rename = "$value")]
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


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
}

impl Max70Text {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.max70_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max70_text is shorter than the minimum length of 1".to_string()));
		}
		if self.max70_text.chars().count() > 70 {
			return Err(ValidationError::new(1002, "max70_text exceeds the maximum length of 70".to_string()));
		}
		Ok(())
	}
}


// MoneyMarketReportHeader1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MoneyMarketReportHeader1 {
	#[serde(rename = "RptgAgt")]
	pub rptg_agt: LEIIdentifier,
	#[serde(rename = "RefPrd")]
	pub ref_prd: DateTimePeriod1,
}

impl MoneyMarketReportHeader1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.rptg_agt.validate() { return Err(e); }
		if let Err(e) = self.ref_prd.validate() { return Err(e); }
		Ok(())
	}
}


// MoneyMarketSecuredMarketStatisticalReportV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MoneyMarketSecuredMarketStatisticalReportV02 {
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: MoneyMarketReportHeader1,
	#[serde(rename = "ScrdMktRpt")]
	pub scrd_mkt_rpt: SecuredMarketReport4Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl MoneyMarketSecuredMarketStatisticalReportV02 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.rpt_hdr.validate() { return Err(e); }
		if let Err(e) = self.scrd_mkt_rpt.validate() { return Err(e); }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// MoneyMarketTransactionType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum MoneyMarketTransactionType1Code {
	#[default]
	#[serde(rename = "BORR")]
	CodeBORR,
	#[serde(rename = "LEND")]
	CodeLEND,
}

impl MoneyMarketTransactionType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// NameAndLocation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndLocation1 {
	#[serde(rename = "Nm")]
	pub nm: Max70Text,
	#[serde(rename = "Lctn")]
	pub lctn: CountryCode,
}

impl NameAndLocation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.nm.validate() { return Err(e); }
		if let Err(e) = self.lctn.validate() { return Err(e); }
		Ok(())
	}
}


// NovationStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NovationStatus1Code {
	#[default]
	#[serde(rename = "NONO")]
	CodeNONO,
	#[serde(rename = "NOVA")]
	CodeNOVA,
}

impl NovationStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Number {
	#[serde(rename = "$value")]
	pub number: f64,
}

impl Number {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PercentageRate {
	#[serde(rename = "$value")]
	pub percentage_rate: f64,
}

impl PercentageRate {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ReportPeriodActivity3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ReportPeriodActivity3Code {
	#[default]
	#[serde(rename = "NOTX")]
	CodeNOTX,
	#[serde(rename = "NORA")]
	CodeNORA,
}

impl ReportPeriodActivity3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SNA2008SectorIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SNA2008SectorIdentifier {
	#[serde(rename = "$value")]
	pub sna2008_sector_identifier: String,
}

impl SNA2008SectorIdentifier {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SectorAndLocation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SectorAndLocation1 {
	#[serde(rename = "Sctr")]
	pub sctr: String,
	#[serde(rename = "Lctn")]
	pub lctn: CountryCode,
}

impl SectorAndLocation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.lctn.validate() { return Err(e); }
		Ok(())
	}
}


// SecuredCollateral2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuredCollateral2Choice {
	#[serde(rename = "SnglColl", skip_serializing_if = "Option::is_none")]
	pub sngl_coll: Option<CollateralValuation6>,
	#[serde(rename = "MltplColl", skip_serializing_if = "Option::is_none")]
	pub mltpl_coll: Option<Vec<CollateralValuation6>>,
	#[serde(rename = "PoolColl", skip_serializing_if = "Option::is_none")]
	pub pool_coll: Option<CollateralValuation6>,
	#[serde(rename = "OthrColl", skip_serializing_if = "Option::is_none")]
	pub othr_coll: Option<Vec<CollateralValuation7>>,
}

impl SecuredCollateral2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref sngl_coll_value) = self.sngl_coll { if let Err(e) = sngl_coll_value.validate() { return Err(e); } }
		if let Some(ref mltpl_coll_vec) = self.mltpl_coll { for item in mltpl_coll_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref pool_coll_value) = self.pool_coll { if let Err(e) = pool_coll_value.validate() { return Err(e); } }
		if let Some(ref othr_coll_vec) = self.othr_coll { for item in othr_coll_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// SecuredMarketReport4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuredMarketReport4Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity3Code>,
	#[serde(rename = "Tx", skip_serializing_if = "Option::is_none")]
	pub tx: Option<Vec<SecuredMarketTransaction4>>,
}

impl SecuredMarketReport4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref data_set_actn_value) = self.data_set_actn { if let Err(e) = data_set_actn_value.validate() { return Err(e); } }
		if let Some(ref tx_vec) = self.tx { for item in tx_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// SecuredMarketTransaction4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuredMarketTransaction4 {
	#[serde(rename = "RptdTxSts")]
	pub rptd_tx_sts: TransactionOperationType1Code,
	#[serde(rename = "NvtnSts", skip_serializing_if = "Option::is_none")]
	pub nvtn_sts: Option<NovationStatus1Code>,
	#[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
	pub brnch_id: Option<LEIIdentifier>,
	#[serde(rename = "UnqTxIdr", skip_serializing_if = "Option::is_none")]
	pub unq_tx_idr: Option<Max105Text>,
	#[serde(rename = "PrtryTxId")]
	pub prtry_tx_id: Max105Text,
	#[serde(rename = "RltdPrtryTxId", skip_serializing_if = "Option::is_none")]
	pub rltd_prtry_tx_id: Option<Max105Text>,
	#[serde(rename = "CtrPtyPrtryTxId", skip_serializing_if = "Option::is_none")]
	pub ctr_pty_prtry_tx_id: Option<Max105Text>,
	#[serde(rename = "CtrPtyId")]
	pub ctr_pty_id: CounterpartyIdentification3Choice,
	#[serde(rename = "TrptyAgtId", skip_serializing_if = "Option::is_none")]
	pub trpty_agt_id: Option<LEIIdentifier>,
	#[serde(rename = "TradDt")]
	pub trad_dt: DateAndDateTimeChoice,
	#[serde(rename = "SttlmDt")]
	pub sttlm_dt: String,
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: String,
	#[serde(rename = "TxTp")]
	pub tx_tp: MoneyMarketTransactionType1Code,
	#[serde(rename = "TxNmnlAmt")]
	pub tx_nmnl_amt: ActiveCurrencyAndAmount,
	#[serde(rename = "RateTp")]
	pub rate_tp: InterestRateType1Code,
	#[serde(rename = "DealRate", skip_serializing_if = "Option::is_none")]
	pub deal_rate: Option<f64>,
	#[serde(rename = "FltgRateRpAgrmt", skip_serializing_if = "Option::is_none")]
	pub fltg_rate_rp_agrmt: Option<FloatingRateNote2>,
	#[serde(rename = "BrkrdDeal", skip_serializing_if = "Option::is_none")]
	pub brkrd_deal: Option<BrokeredDeal1Code>,
	#[serde(rename = "Coll")]
	pub coll: Collateral18,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl SecuredMarketTransaction4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.rptd_tx_sts.validate() { return Err(e); }
		if let Some(ref nvtn_sts_value) = self.nvtn_sts { if let Err(e) = nvtn_sts_value.validate() { return Err(e); } }
		if let Some(ref brnch_id_value) = self.brnch_id { if let Err(e) = brnch_id_value.validate() { return Err(e); } }
		if let Some(ref unq_tx_idr_value) = self.unq_tx_idr { if let Err(e) = unq_tx_idr_value.validate() { return Err(e); } }
		if let Err(e) = self.prtry_tx_id.validate() { return Err(e); }
		if let Some(ref rltd_prtry_tx_id_value) = self.rltd_prtry_tx_id { if let Err(e) = rltd_prtry_tx_id_value.validate() { return Err(e); } }
		if let Some(ref ctr_pty_prtry_tx_id_value) = self.ctr_pty_prtry_tx_id { if let Err(e) = ctr_pty_prtry_tx_id_value.validate() { return Err(e); } }
		if let Err(e) = self.ctr_pty_id.validate() { return Err(e); }
		if let Some(ref trpty_agt_id_value) = self.trpty_agt_id { if let Err(e) = trpty_agt_id_value.validate() { return Err(e); } }
		if let Err(e) = self.trad_dt.validate() { return Err(e); }
		if let Err(e) = self.tx_tp.validate() { return Err(e); }
		if let Err(e) = self.tx_nmnl_amt.validate() { return Err(e); }
		if let Err(e) = self.rate_tp.validate() { return Err(e); }
		if let Some(ref fltg_rate_rp_agrmt_value) = self.fltg_rate_rp_agrmt { if let Err(e) = fltg_rate_rp_agrmt_value.validate() { return Err(e); } }
		if let Some(ref brkrd_deal_value) = self.brkrd_deal { if let Err(e) = brkrd_deal_value.validate() { return Err(e); } }
		if let Err(e) = self.coll.validate() { return Err(e); }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// SpecialCollateral2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SpecialCollateral2Code {
	#[default]
	#[serde(rename = "GENE")]
	CodeGENE,
	#[serde(rename = "SPEC")]
	CodeSPEC,
	#[serde(rename = "MRRP")]
	CodeMRRP,
}

impl SpecialCollateral2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
	pub plc_and_nm: Option<Max350Text>,
	#[serde(rename = "Envlp")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TransactionOperationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TransactionOperationType1Code {
	#[default]
	#[serde(rename = "AMND")]
	CodeAMND,
	#[serde(rename = "CANC")]
	CodeCANC,
	#[serde(rename = "CORR")]
	CodeCORR,
	#[serde(rename = "NEWT")]
	CodeNEWT,
}

impl TransactionOperationType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}
