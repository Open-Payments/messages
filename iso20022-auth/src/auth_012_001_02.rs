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


// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_currency_and_amount_simple_type: f64,
}

impl ActiveCurrencyAndAmountSimpleType {
	pub fn validate(&self) -> bool {
		if self.active_currency_and_amount_simple_type < 0.000000 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.active_currency_code) {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{6,6}").unwrap();
		if !pattern.is_match(&self.cfi_oct2015_identifier) {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if !self.valtn.validate() { return false }
		if let Some(ref spcl_coll_ind_value) = self.spcl_coll_ind { if !spcl_coll_ind_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref nmnl_amt_value) = self.nmnl_amt { if !nmnl_amt_value.validate() { return false; } }
		if !self.isin.validate() { return false }
		return true
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
	pub fn validate(&self) -> bool {
		if !self.pool_sts.validate() { return false }
		if !self.tp.validate() { return false }
		if let Some(ref nmnl_amt_value) = self.nmnl_amt { if !nmnl_amt_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref lei_value) = self.lei { if !lei_value.validate() { return false; } }
		if let Some(ref sctr_and_lctn_value) = self.sctr_and_lctn { if !sctr_and_lctn_value.validate() { return false; } }
		if let Some(ref nm_and_lctn_value) = self.nm_and_lctn { if !nm_and_lctn_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.country_code) {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if !self.ref_rate_indx.validate() { return false }
		return true
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
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
		if !pattern.is_match(&self.isin_oct2015_identifier) {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.lei_identifier) {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.max105_text.chars().count() < 1 {
			return false
		}
		if self.max105_text.chars().count() > 105 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.max350_text.chars().count() < 1 {
			return false
		}
		if self.max350_text.chars().count() > 350 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if self.max70_text.chars().count() < 1 {
			return false
		}
		if self.max70_text.chars().count() > 70 {
			return false
		}
		return true
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
	pub fn validate(&self) -> bool {
		if !self.rptg_agt.validate() { return false }
		if !self.ref_prd.validate() { return false }
		return true
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
	pub fn validate(&self) -> bool {
		if !self.rpt_hdr.validate() { return false }
		if !self.scrd_mkt_rpt.validate() { return false }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if !self.nm.validate() { return false }
		if !self.lctn.validate() { return false }
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if !self.lctn.validate() { return false }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref sngl_coll_value) = self.sngl_coll { if !sngl_coll_value.validate() { return false; } }
		if let Some(ref mltpl_coll_vec) = self.mltpl_coll { for item in mltpl_coll_vec { if !item.validate() { return false; } } }
		if let Some(ref pool_coll_value) = self.pool_coll { if !pool_coll_value.validate() { return false; } }
		if let Some(ref othr_coll_vec) = self.othr_coll { for item in othr_coll_vec { if !item.validate() { return false; } } }
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref data_set_actn_value) = self.data_set_actn { if !data_set_actn_value.validate() { return false; } }
		if let Some(ref tx_vec) = self.tx { for item in tx_vec { if !item.validate() { return false; } } }
		return true
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
	pub fn validate(&self) -> bool {
		if !self.rptd_tx_sts.validate() { return false }
		if let Some(ref nvtn_sts_value) = self.nvtn_sts { if !nvtn_sts_value.validate() { return false; } }
		if let Some(ref brnch_id_value) = self.brnch_id { if !brnch_id_value.validate() { return false; } }
		if let Some(ref unq_tx_idr_value) = self.unq_tx_idr { if !unq_tx_idr_value.validate() { return false; } }
		if !self.prtry_tx_id.validate() { return false }
		if let Some(ref rltd_prtry_tx_id_value) = self.rltd_prtry_tx_id { if !rltd_prtry_tx_id_value.validate() { return false; } }
		if let Some(ref ctr_pty_prtry_tx_id_value) = self.ctr_pty_prtry_tx_id { if !ctr_pty_prtry_tx_id_value.validate() { return false; } }
		if !self.ctr_pty_id.validate() { return false }
		if let Some(ref trpty_agt_id_value) = self.trpty_agt_id { if !trpty_agt_id_value.validate() { return false; } }
		if !self.trad_dt.validate() { return false }
		if !self.tx_tp.validate() { return false }
		if !self.tx_nmnl_amt.validate() { return false }
		if !self.rate_tp.validate() { return false }
		if let Some(ref fltg_rate_rp_agrmt_value) = self.fltg_rate_rp_agrmt { if !fltg_rate_rp_agrmt_value.validate() { return false; } }
		if let Some(ref brkrd_deal_value) = self.brkrd_deal { if !brkrd_deal_value.validate() { return false; } }
		if !self.coll.validate() { return false }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
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
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		if let Some(ref plc_and_nm_value) = self.plc_and_nm { if !plc_and_nm_value.validate() { return false; } }
		if !self.envlp.validate() { return false }
		return true
	}
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		return true
	}
}
