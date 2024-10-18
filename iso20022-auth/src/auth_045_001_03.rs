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


// ActiveOrHistoricCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_and_amount_simple_type: f64,
}

impl ActiveOrHistoricCurrencyAndAmountSimpleType {
	pub fn validate(&self) -> bool {
		if self.active_or_historic_currency_and_amount_simple_type < 0.000000 {
			return false
		}
		return true
	}
}


// ActiveOrHistoricCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveOrHistoricCurrencyAndAmount {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_code: String,
}

impl ActiveOrHistoricCurrencyCode {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.active_or_historic_currency_code) {
			return false
		}
		return true
	}
}


// AssetClassAndSubClassIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassAndSubClassIdentification2 {
	#[serde(rename = "AsstClss")]
	pub asst_clss: NonEquityAssetClass1Code,
	#[serde(rename = "DerivSubClss", skip_serializing_if = "Option::is_none")]
	pub deriv_sub_clss: Option<NonEquitySubClass1>,
	#[serde(rename = "FinInstrmClssfctn", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_clssfctn: Option<NonEquityInstrumentReportingClassification1Code>,
}

impl AssetClassAndSubClassIdentification2 {
	pub fn validate(&self) -> bool {
		if !self.asst_clss.validate() { return false }
		if let Some(ref deriv_sub_clss_value) = self.deriv_sub_clss { if !deriv_sub_clss_value.validate() { return false; } }
		if let Some(ref fin_instrm_clssfctn_value) = self.fin_instrm_clssfctn { if !fin_instrm_clssfctn_value.validate() { return false; } }
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


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DecimalNumber {
	#[serde(rename = "$value")]
	pub decimal_number: f64,
}

impl DecimalNumber {
	pub fn validate(&self) -> bool {
		return true
	}
}


// FinancialInstrumentReportingNonEquityTradingActivityResultV03 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentReportingNonEquityTradingActivityResultV03 {
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: SecuritiesMarketReportHeader1,
	#[serde(rename = "NonEqtyTrnsprncyData")]
	pub non_eqty_trnsprncy_data: Vec<TransparencyDataReport20>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl FinancialInstrumentReportingNonEquityTradingActivityResultV03 {
	pub fn validate(&self) -> bool {
		if !self.rpt_hdr.validate() { return false }
		for item in &self.non_eqty_trnsprncy_data { if !item.validate() { return false; } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
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


// InstrumentAndSubClassIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InstrumentAndSubClassIdentification2 {
	#[serde(rename = "ISIN")]
	pub isin: ISINOct2015Identifier,
	#[serde(rename = "DerivSubClss", skip_serializing_if = "Option::is_none")]
	pub deriv_sub_clss: Option<NonEquitySubClass1>,
	#[serde(rename = "FinInstrmClssfctn", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_clssfctn: Option<NonEquityInstrumentReportingClassification1Code>,
}

impl InstrumentAndSubClassIdentification2 {
	pub fn validate(&self) -> bool {
		if !self.isin.validate() { return false }
		if let Some(ref deriv_sub_clss_value) = self.deriv_sub_clss { if !deriv_sub_clss_value.validate() { return false; } }
		if let Some(ref fin_instrm_clssfctn_value) = self.fin_instrm_clssfctn { if !fin_instrm_clssfctn_value.validate() { return false; } }
		return true
	}
}


// InstrumentOrSubClassIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InstrumentOrSubClassIdentification2Choice {
	#[serde(rename = "ISINAndSubClss", skip_serializing_if = "Option::is_none")]
	pub isin_and_sub_clss: Option<InstrumentAndSubClassIdentification2>,
	#[serde(rename = "AsstClssAndSubClss", skip_serializing_if = "Option::is_none")]
	pub asst_clss_and_sub_clss: Option<AssetClassAndSubClassIdentification2>,
}

impl InstrumentOrSubClassIdentification2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref isin_and_sub_clss_value) = self.isin_and_sub_clss { if !isin_and_sub_clss_value.validate() { return false; } }
		if let Some(ref asst_clss_and_sub_clss_value) = self.asst_clss_and_sub_clss { if !asst_clss_and_sub_clss_value.validate() { return false; } }
		return true
	}
}


// MICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MICIdentifier {
	#[serde(rename = "$value")]
	pub mic_identifier: String,
}

impl MICIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
		if !pattern.is_match(&self.mic_identifier) {
			return false
		}
		return true
	}
}


// Max1000Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max1000Text {
	#[serde(rename = "$value")]
	pub max1000_text: String,
}

impl Max1000Text {
	pub fn validate(&self) -> bool {
		if self.max1000_text.chars().count() < 1 {
			return false
		}
		if self.max1000_text.chars().count() > 1000 {
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


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}

impl Max35Text {
	pub fn validate(&self) -> bool {
		if self.max35_text.chars().count() < 1 {
			return false
		}
		if self.max35_text.chars().count() > 35 {
			return false
		}
		return true
	}
}


// Max50Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max50Text {
	#[serde(rename = "$value")]
	pub max50_text: String,
}

impl Max50Text {
	pub fn validate(&self) -> bool {
		if self.max50_text.chars().count() < 1 {
			return false
		}
		if self.max50_text.chars().count() > 50 {
			return false
		}
		return true
	}
}


// NonEquityAssetClass1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NonEquityAssetClass1Code {
	#[default]
	#[serde(rename = "SDRV")]
	CodeSDRV,
	#[serde(rename = "IRDV")]
	CodeIRDV,
	#[serde(rename = "FEXD")]
	CodeFEXD,
	#[serde(rename = "EQDV")]
	CodeEQDV,
	#[serde(rename = "EADV")]
	CodeEADV,
	#[serde(rename = "EMAL")]
	CodeEMAL,
	#[serde(rename = "CRDV")]
	CodeCRDV,
	#[serde(rename = "CFDS")]
	CodeCFDS,
	#[serde(rename = "COMD")]
	CodeCOMD,
	#[serde(rename = "C10D")]
	CodeC10D,
	#[serde(rename = "BOND")]
	CodeBOND,
	#[serde(rename = "ETCS")]
	CodeETCS,
	#[serde(rename = "ETNS")]
	CodeETNS,
	#[serde(rename = "SFPS")]
	CodeSFPS,
}

impl NonEquityAssetClass1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// NonEquityInstrumentReportingClassification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NonEquityInstrumentReportingClassification1Code {
	#[default]
	#[serde(rename = "SFPS")]
	CodeSFPS,
	#[serde(rename = "SDRV")]
	CodeSDRV,
	#[serde(rename = "DERV")]
	CodeDERV,
	#[serde(rename = "EMAL")]
	CodeEMAL,
	#[serde(rename = "BOND")]
	CodeBOND,
	#[serde(rename = "ETCS")]
	CodeETCS,
	#[serde(rename = "ETNS")]
	CodeETNS,
}

impl NonEquityInstrumentReportingClassification1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// NonEquitySubClass1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NonEquitySubClass1 {
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max1000Text>,
	#[serde(rename = "SgmttnCrit")]
	pub sgmttn_crit: Vec<NonEquitySubClassSegmentationCriterion1>,
}

impl NonEquitySubClass1 {
	pub fn validate(&self) -> bool {
		if let Some(ref desc_value) = self.desc { if !desc_value.validate() { return false; } }
		for item in &self.sgmttn_crit { if !item.validate() { return false; } }
		return true
	}
}


// NonEquitySubClassSegmentationCriteria1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NonEquitySubClassSegmentationCriteria1Code {
	#[default]
	#[serde(rename = "ASCL")]
	CodeASCL,
	#[serde(rename = "BSPD")]
	CodeBSPD,
	#[serde(rename = "CNC1")]
	CodeCNC1,
	#[serde(rename = "CNC2")]
	CodeCNC2,
	#[serde(rename = "NCCO")]
	CodeNCCO,
	#[serde(rename = "CTYP")]
	CodeCTYP,
	#[serde(rename = "NCCR")]
	CodeNCCR,
	#[serde(rename = "DCSL")]
	CodeDCSL,
	#[serde(rename = "DTYP")]
	CodeDTYP,
	#[serde(rename = "EQUT")]
	CodeEQUT,
	#[serde(rename = "FNC1")]
	CodeFNC1,
	#[serde(rename = "FNC2")]
	CodeFNC2,
	#[serde(rename = "FSPD")]
	CodeFSPD,
	#[serde(rename = "IIND")]
	CodeIIND,
	#[serde(rename = "IRTC")]
	CodeIRTC,
	#[serde(rename = "INC1")]
	CodeINC1,
	#[serde(rename = "INC2")]
	CodeINC2,
	#[serde(rename = "ISIN")]
	CodeISIN,
	#[serde(rename = "TTMO")]
	CodeTTMO,
	#[serde(rename = "PRMT")]
	CodePRMT,
	#[serde(rename = "SSRF")]
	CodeSSRF,
	#[serde(rename = "ISPT")]
	CodeISPT,
	#[serde(rename = "SRTC")]
	CodeSRTC,
	#[serde(rename = "SACL")]
	CodeSACL,
	#[serde(rename = "SBPD")]
	CodeSBPD,
	#[serde(rename = "TTMS")]
	CodeTTMS,
	#[serde(rename = "NCSW")]
	CodeNCSW,
	#[serde(rename = "TTMB")]
	CodeTTMB,
	#[serde(rename = "IOUB")]
	CodeIOUB,
	#[serde(rename = "TOUB")]
	CodeTOUB,
	#[serde(rename = "UISC")]
	CodeUISC,
	#[serde(rename = "UIDX")]
	CodeUIDX,
	#[serde(rename = "UINS")]
	CodeUINS,
	#[serde(rename = "UIRT")]
	CodeUIRT,
	#[serde(rename = "REOU")]
	CodeREOU,
	#[serde(rename = "UTYP")]
	CodeUTYP,
}

impl NonEquitySubClassSegmentationCriteria1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// NonEquitySubClassSegmentationCriterion1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NonEquitySubClassSegmentationCriterion1 {
	#[serde(rename = "CritNm")]
	pub crit_nm: NonEquitySubClassSegmentationCriteria1Code,
	#[serde(rename = "CritVal")]
	pub crit_val: Max1000Text,
}

impl NonEquitySubClassSegmentationCriterion1 {
	pub fn validate(&self) -> bool {
		if !self.crit_nm.validate() { return false }
		if !self.crit_val.validate() { return false }
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


// Period2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}

impl Period2 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Period4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period4Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt", skip_serializing_if = "Option::is_none")]
	pub to_dt: Option<String>,
	#[serde(rename = "FrDtToDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt_to_dt: Option<Period2>,
}

impl Period4Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref fr_dt_to_dt_value) = self.fr_dt_to_dt { if !fr_dt_to_dt_value.validate() { return false; } }
		return true
	}
}


// SecuritiesMarketReportHeader1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesMarketReportHeader1 {
	#[serde(rename = "RptgNtty")]
	pub rptg_ntty: TradingVenueIdentification1Choice,
	#[serde(rename = "RptgPrd")]
	pub rptg_prd: Period4Choice,
	#[serde(rename = "SubmissnDtTm", skip_serializing_if = "Option::is_none")]
	pub submissn_dt_tm: Option<String>,
}

impl SecuritiesMarketReportHeader1 {
	pub fn validate(&self) -> bool {
		if !self.rptg_ntty.validate() { return false }
		if !self.rptg_prd.validate() { return false }
		return true
	}
}


// StatisticsTransparency2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatisticsTransparency2 {
	#[serde(rename = "TtlNbOfTxsExctd")]
	pub ttl_nb_of_txs_exctd: f64,
	#[serde(rename = "TtlVolOfTxsExctd")]
	pub ttl_vol_of_txs_exctd: f64,
}

impl StatisticsTransparency2 {
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


// TonsOrCurrency2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TonsOrCurrency2Choice {
	#[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
	pub nb: Option<f64>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl TonsOrCurrency2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref amt_value) = self.amt { if !amt_value.validate() { return false; } }
		return true
	}
}


// TradingVenue2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TradingVenue2Code {
	#[default]
	#[serde(rename = "APPA")]
	CodeAPPA,
	#[serde(rename = "CTPS")]
	CodeCTPS,
}

impl TradingVenue2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// TradingVenueIdentification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingVenueIdentification1Choice {
	#[serde(rename = "MktIdCd", skip_serializing_if = "Option::is_none")]
	pub mkt_id_cd: Option<MICIdentifier>,
	#[serde(rename = "NtlCmptntAuthrty", skip_serializing_if = "Option::is_none")]
	pub ntl_cmptnt_authrty: Option<CountryCode>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<TradingVenueIdentification2>,
}

impl TradingVenueIdentification1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref mkt_id_cd_value) = self.mkt_id_cd { if !mkt_id_cd_value.validate() { return false; } }
		if let Some(ref ntl_cmptnt_authrty_value) = self.ntl_cmptnt_authrty { if !ntl_cmptnt_authrty_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
	}
}


// TradingVenueIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingVenueIdentification2 {
	#[serde(rename = "Id")]
	pub id: Max50Text,
	#[serde(rename = "Tp")]
	pub tp: TradingVenue2Code,
}

impl TradingVenueIdentification2 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.tp.validate() { return false }
		return true
	}
}


// TransparencyDataReport20 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransparencyDataReport20 {
	#[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
	pub tech_rcrd_id: Option<Max35Text>,
	#[serde(rename = "Id")]
	pub id: InstrumentOrSubClassIdentification2Choice,
	#[serde(rename = "FullNm", skip_serializing_if = "Option::is_none")]
	pub full_nm: Option<Max350Text>,
	#[serde(rename = "TradgVn", skip_serializing_if = "Option::is_none")]
	pub tradg_vn: Option<MICIdentifier>,
	#[serde(rename = "RptgPrd", skip_serializing_if = "Option::is_none")]
	pub rptg_prd: Option<Period4Choice>,
	#[serde(rename = "Lqdty", skip_serializing_if = "Option::is_none")]
	pub lqdty: Option<bool>,
	#[serde(rename = "PreTradLrgInScaleThrshld", skip_serializing_if = "Option::is_none")]
	pub pre_trad_lrg_in_scale_thrshld: Option<TonsOrCurrency2Choice>,
	#[serde(rename = "PstTradLrgInScaleThrshld", skip_serializing_if = "Option::is_none")]
	pub pst_trad_lrg_in_scale_thrshld: Option<TonsOrCurrency2Choice>,
	#[serde(rename = "PreTradInstrmSzSpcfcThrshld", skip_serializing_if = "Option::is_none")]
	pub pre_trad_instrm_sz_spcfc_thrshld: Option<TonsOrCurrency2Choice>,
	#[serde(rename = "PstTradInstrmSzSpcfcThrshld", skip_serializing_if = "Option::is_none")]
	pub pst_trad_instrm_sz_spcfc_thrshld: Option<TonsOrCurrency2Choice>,
	#[serde(rename = "Sttstcs", skip_serializing_if = "Option::is_none")]
	pub sttstcs: Option<StatisticsTransparency2>,
}

impl TransparencyDataReport20 {
	pub fn validate(&self) -> bool {
		if let Some(ref tech_rcrd_id_value) = self.tech_rcrd_id { if !tech_rcrd_id_value.validate() { return false; } }
		if !self.id.validate() { return false }
		if let Some(ref full_nm_value) = self.full_nm { if !full_nm_value.validate() { return false; } }
		if let Some(ref tradg_vn_value) = self.tradg_vn { if !tradg_vn_value.validate() { return false; } }
		if let Some(ref rptg_prd_value) = self.rptg_prd { if !rptg_prd_value.validate() { return false; } }
		if let Some(ref pre_trad_lrg_in_scale_thrshld_value) = self.pre_trad_lrg_in_scale_thrshld { if !pre_trad_lrg_in_scale_thrshld_value.validate() { return false; } }
		if let Some(ref pst_trad_lrg_in_scale_thrshld_value) = self.pst_trad_lrg_in_scale_thrshld { if !pst_trad_lrg_in_scale_thrshld_value.validate() { return false; } }
		if let Some(ref pre_trad_instrm_sz_spcfc_thrshld_value) = self.pre_trad_instrm_sz_spcfc_thrshld { if !pre_trad_instrm_sz_spcfc_thrshld_value.validate() { return false; } }
		if let Some(ref pst_trad_instrm_sz_spcfc_thrshld_value) = self.pst_trad_instrm_sz_spcfc_thrshld { if !pst_trad_instrm_sz_spcfc_thrshld_value.validate() { return false; } }
		if let Some(ref sttstcs_value) = self.sttstcs { if !sttstcs_value.validate() { return false; } }
		return true
	}
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}

impl TrueFalseIndicator {
	pub fn validate(&self) -> bool {
		return true
	}
}
