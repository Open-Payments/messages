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


// AddressType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AddressType2Code {
	#[default]
	#[serde(rename = "ADDR")]
	CodeADDR,
	#[serde(rename = "PBOX")]
	CodePBOX,
	#[serde(rename = "HOME")]
	CodeHOME,
	#[serde(rename = "BIZZ")]
	CodeBIZZ,
	#[serde(rename = "MLTO")]
	CodeMLTO,
	#[serde(rename = "DLVY")]
	CodeDLVY,
}

impl AddressType2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
}

impl AnyBICDec2014Identifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
		if !pattern.is_match(&self.any_bic_dec2014_identifier) {
			return false
		}
		return true
	}
}


// AnyMIC1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AnyMIC1Code {
	#[default]
	#[serde(rename = "ANYM")]
	CodeANYM,
}

impl AnyMIC1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CollateralType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CollateralType6Code {
	#[default]
	#[serde(rename = "GBBK")]
	CodeGBBK,
	#[serde(rename = "BOND")]
	CodeBOND,
	#[serde(rename = "CASH")]
	CodeCASH,
	#[serde(rename = "COMM")]
	CodeCOMM,
	#[serde(rename = "INSU")]
	CodeINSU,
	#[serde(rename = "LCRE")]
	CodeLCRE,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "PHYS")]
	CodePHYS,
	#[serde(rename = "SECU")]
	CodeSECU,
	#[serde(rename = "STCF")]
	CodeSTCF,
}

impl CollateralType6Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CorporateSectorCriteria5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CorporateSectorCriteria5 {
	#[serde(rename = "FISctr", skip_serializing_if = "Option::is_none")]
	pub fi_sctr: Option<Vec<FinancialPartySectorType2Code>>,
	#[serde(rename = "NFISctr", skip_serializing_if = "Option::is_none")]
	pub nfi_sctr: Option<Vec<NACEDomainIdentifier>>,
	#[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
	pub not_rptd: Option<NotReported1Code>,
}

impl CorporateSectorCriteria5 {
	pub fn validate(&self) -> bool {
		if let Some(ref fi_sctr_vec) = self.fi_sctr { for item in fi_sctr_vec { if !item.validate() { return false; } } }
		if let Some(ref nfi_sctr_vec) = self.nfi_sctr { for item in nfi_sctr_vec { if !item.validate() { return false; } } }
		if let Some(ref not_rptd_value) = self.not_rptd { if !not_rptd_value.validate() { return false; } }
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


// DateOrBlankQuery2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateOrBlankQuery2Choice {
	#[serde(rename = "Rg", skip_serializing_if = "Option::is_none")]
	pub rg: Option<DatePeriod1>,
	#[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
	pub not_rptd: Option<NotReported1Code>,
}

impl DateOrBlankQuery2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref rg_value) = self.rg { if !rg_value.validate() { return false; } }
		if let Some(ref not_rptd_value) = self.not_rptd { if !not_rptd_value.validate() { return false; } }
		return true
	}
}


// DatePeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriod1 {
	#[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}

impl DatePeriod1 {
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


// DayOfMonthNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DayOfMonthNumber {
	#[serde(rename = "$value")]
	pub day_of_month_number: f64,
}

impl DayOfMonthNumber {
	pub fn validate(&self) -> bool {
		if self.day_of_month_number < 1.000000 {
			return false
		}
		if self.day_of_month_number > 31.000000 {
			return false
		}
		return true
	}
}


// ExposureType10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ExposureType10Code {
	#[default]
	#[serde(rename = "SBSC")]
	CodeSBSC,
	#[serde(rename = "MGLD")]
	CodeMGLD,
	#[serde(rename = "SLEB")]
	CodeSLEB,
	#[serde(rename = "REPO")]
	CodeREPO,
}

impl ExposureType10Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// FinancialPartySectorType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FinancialPartySectorType2Code {
	#[default]
	#[serde(rename = "AIFD")]
	CodeAIFD,
	#[serde(rename = "CSDS")]
	CodeCSDS,
	#[serde(rename = "CCPS")]
	CodeCCPS,
	#[serde(rename = "CDTI")]
	CodeCDTI,
	#[serde(rename = "INUN")]
	CodeINUN,
	#[serde(rename = "ORPI")]
	CodeORPI,
	#[serde(rename = "INVF")]
	CodeINVF,
	#[serde(rename = "REIN")]
	CodeREIN,
	#[serde(rename = "UCIT")]
	CodeUCIT,
}

impl FinancialPartySectorType2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Frequency14Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Frequency14Code {
	#[default]
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "ADHO")]
	CodeADHO,
}

impl Frequency14Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// GenericIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
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


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max16Text {
	#[serde(rename = "$value")]
	pub max16_text: String,
}

impl Max16Text {
	pub fn validate(&self) -> bool {
		if self.max16_text.chars().count() < 1 {
			return false
		}
		if self.max16_text.chars().count() > 16 {
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


// NACEDomainIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NACEDomainIdentifier {
	#[serde(rename = "$value")]
	pub nace_domain_identifier: String,
}

impl NACEDomainIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-U]{1,1}").unwrap();
		if !pattern.is_match(&self.nace_domain_identifier) {
			return false
		}
		return true
	}
}


// NameAndAddress5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<PostalAddress1>,
}

impl NameAndAddress5 {
	pub fn validate(&self) -> bool {
		if !self.nm.validate() { return false }
		if let Some(ref adr_value) = self.adr { if !adr_value.validate() { return false; } }
		return true
	}
}


// NotReported1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NotReported1Code {
	#[default]
	#[serde(rename = "NORP")]
	CodeNORP,
}

impl NotReported1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Operation3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Operation3Code {
	#[default]
	#[serde(rename = "ANDD")]
	CodeANDD,
	#[serde(rename = "ORRR")]
	CodeORRR,
}

impl Operation3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// PartyIdentification121Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification121Choice {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
	#[serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none")]
	pub lgl_ntty_idr: Option<LEIIdentifier>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification1>,
}

impl PartyIdentification121Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref any_bic_value) = self.any_bic { if !any_bic_value.validate() { return false; } }
		if let Some(ref lgl_ntty_idr_value) = self.lgl_ntty_idr { if !lgl_ntty_idr_value.validate() { return false; } }
		if let Some(ref nm_and_adr_value) = self.nm_and_adr { if !nm_and_adr_value.validate() { return false; } }
		if let Some(ref prtry_id_value) = self.prtry_id { if !prtry_id_value.validate() { return false; } }
		return true
	}
}


// PartyNatureType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PartyNatureType1Code {
	#[default]
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "NFIN")]
	CodeNFIN,
	#[serde(rename = "FIIN")]
	CodeFIIN,
	#[serde(rename = "CCPS")]
	CodeCCPS,
}

impl PartyNatureType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// PostalAddress1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress1 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<AddressType2Code>,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
	pub adr_line: Option<Vec<Max70Text>>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<Max70Text>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<Max16Text>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<Max16Text>,
	#[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
	pub twn_nm: Option<Max35Text>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<Max35Text>,
	#[serde(rename = "Ctry")]
	pub ctry: CountryCode,
}

impl PostalAddress1 {
	pub fn validate(&self) -> bool {
		if let Some(ref adr_tp_value) = self.adr_tp { if !adr_tp_value.validate() { return false; } }
		if let Some(ref adr_line_vec) = self.adr_line { for item in adr_line_vec { if !item.validate() { return false; } } }
		if let Some(ref strt_nm_value) = self.strt_nm { if !strt_nm_value.validate() { return false; } }
		if let Some(ref bldg_nb_value) = self.bldg_nb { if !bldg_nb_value.validate() { return false; } }
		if let Some(ref pst_cd_value) = self.pst_cd { if !pst_cd_value.validate() { return false; } }
		if let Some(ref twn_nm_value) = self.twn_nm { if !twn_nm_value.validate() { return false; } }
		if let Some(ref ctry_sub_dvsn_value) = self.ctry_sub_dvsn { if !ctry_sub_dvsn_value.validate() { return false; } }
		if !self.ctry.validate() { return false }
		return true
	}
}


// SecuritiesFinancingReportingTransactionQueryV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesFinancingReportingTransactionQueryV02 {
	#[serde(rename = "RqstngAuthrty")]
	pub rqstng_authrty: PartyIdentification121Choice,
	#[serde(rename = "TradQryData")]
	pub trad_qry_data: TradeReportQuery13Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl SecuritiesFinancingReportingTransactionQueryV02 {
	pub fn validate(&self) -> bool {
		if !self.rqstng_authrty.validate() { return false }
		if !self.trad_qry_data.validate() { return false }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
	}
}


// SecuritiesTradeVenueCriteria1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTradeVenueCriteria1Choice {
	#[serde(rename = "MIC", skip_serializing_if = "Option::is_none")]
	pub mic: Option<Vec<MICIdentifier>>,
	#[serde(rename = "AnyMIC", skip_serializing_if = "Option::is_none")]
	pub any_mic: Option<AnyMIC1Code>,
}

impl SecuritiesTradeVenueCriteria1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref mic_vec) = self.mic { for item in mic_vec { if !item.validate() { return false; } } }
		if let Some(ref any_mic_value) = self.any_mic { if !any_mic_value.validate() { return false; } }
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


// TradeAdditionalQueryCriteria7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeAdditionalQueryCriteria7 {
	#[serde(rename = "ActnTp", skip_serializing_if = "Option::is_none")]
	pub actn_tp: Option<Vec<TransactionOperationType6Code>>,
	#[serde(rename = "ExctnVn", skip_serializing_if = "Option::is_none")]
	pub exctn_vn: Option<SecuritiesTradeVenueCriteria1Choice>,
	#[serde(rename = "NtrOfCtrPty", skip_serializing_if = "Option::is_none")]
	pub ntr_of_ctr_pty: Option<Vec<PartyNatureType1Code>>,
	#[serde(rename = "CorpSctr", skip_serializing_if = "Option::is_none")]
	pub corp_sctr: Option<Vec<CorporateSectorCriteria5>>,
}

impl TradeAdditionalQueryCriteria7 {
	pub fn validate(&self) -> bool {
		if let Some(ref actn_tp_vec) = self.actn_tp { for item in actn_tp_vec { if !item.validate() { return false; } } }
		if let Some(ref exctn_vn_value) = self.exctn_vn { if !exctn_vn_value.validate() { return false; } }
		if let Some(ref ntr_of_ctr_pty_vec) = self.ntr_of_ctr_pty { for item in ntr_of_ctr_pty_vec { if !item.validate() { return false; } } }
		if let Some(ref corp_sctr_vec) = self.corp_sctr { for item in corp_sctr_vec { if !item.validate() { return false; } } }
		return true
	}
}


// TradeDateTimeQueryCriteria2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeDateTimeQueryCriteria2 {
	#[serde(rename = "RptgDtTm", skip_serializing_if = "Option::is_none")]
	pub rptg_dt_tm: Option<DateTimePeriod1>,
	#[serde(rename = "ExctnDtTm", skip_serializing_if = "Option::is_none")]
	pub exctn_dt_tm: Option<DateTimePeriod1>,
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<DateOrBlankQuery2Choice>,
	#[serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none")]
	pub termntn_dt: Option<DateOrBlankQuery2Choice>,
}

impl TradeDateTimeQueryCriteria2 {
	pub fn validate(&self) -> bool {
		if let Some(ref rptg_dt_tm_value) = self.rptg_dt_tm { if !rptg_dt_tm_value.validate() { return false; } }
		if let Some(ref exctn_dt_tm_value) = self.exctn_dt_tm { if !exctn_dt_tm_value.validate() { return false; } }
		if let Some(ref mtrty_dt_value) = self.mtrty_dt { if !mtrty_dt_value.validate() { return false; } }
		if let Some(ref termntn_dt_value) = self.termntn_dt { if !termntn_dt_value.validate() { return false; } }
		return true
	}
}


// TradePartyIdentificationQuery8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradePartyIdentificationQuery8 {
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<Vec<LEIIdentifier>>,
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<Vec<AnyBICDec2014Identifier>>,
	#[serde(rename = "ClntId", skip_serializing_if = "Option::is_none")]
	pub clnt_id: Option<Vec<Max50Text>>,
	#[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
	pub not_rptd: Option<NotReported1Code>,
}

impl TradePartyIdentificationQuery8 {
	pub fn validate(&self) -> bool {
		if let Some(ref lei_vec) = self.lei { for item in lei_vec { if !item.validate() { return false; } } }
		if let Some(ref any_bic_vec) = self.any_bic { for item in any_bic_vec { if !item.validate() { return false; } } }
		if let Some(ref clnt_id_vec) = self.clnt_id { for item in clnt_id_vec { if !item.validate() { return false; } } }
		if let Some(ref not_rptd_value) = self.not_rptd { if !not_rptd_value.validate() { return false; } }
		return true
	}
}


// TradePartyIdentificationQuery9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradePartyIdentificationQuery9 {
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<Vec<LEIIdentifier>>,
	#[serde(rename = "CtryCd", skip_serializing_if = "Option::is_none")]
	pub ctry_cd: Option<Vec<CountryCode>>,
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<Vec<AnyBICDec2014Identifier>>,
	#[serde(rename = "ClntId", skip_serializing_if = "Option::is_none")]
	pub clnt_id: Option<Vec<Max50Text>>,
	#[serde(rename = "NotRptd", skip_serializing_if = "Option::is_none")]
	pub not_rptd: Option<NotReported1Code>,
}

impl TradePartyIdentificationQuery9 {
	pub fn validate(&self) -> bool {
		if let Some(ref lei_vec) = self.lei { for item in lei_vec { if !item.validate() { return false; } } }
		if let Some(ref ctry_cd_vec) = self.ctry_cd { for item in ctry_cd_vec { if !item.validate() { return false; } } }
		if let Some(ref any_bic_vec) = self.any_bic { for item in any_bic_vec { if !item.validate() { return false; } } }
		if let Some(ref clnt_id_vec) = self.clnt_id { for item in clnt_id_vec { if !item.validate() { return false; } } }
		if let Some(ref not_rptd_value) = self.not_rptd { if !not_rptd_value.validate() { return false; } }
		return true
	}
}


// TradePartyQueryCriteria5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradePartyQueryCriteria5 {
	#[serde(rename = "Oprtr")]
	pub oprtr: Operation3Code,
	#[serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none")]
	pub rptg_ctr_pty: Option<TradePartyIdentificationQuery8>,
	#[serde(rename = "RptgCtrPtyBrnch", skip_serializing_if = "Option::is_none")]
	pub rptg_ctr_pty_brnch: Option<TradePartyIdentificationQuery9>,
	#[serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none")]
	pub othr_ctr_pty: Option<TradePartyIdentificationQuery8>,
	#[serde(rename = "OthrCtrPtyBrnch", skip_serializing_if = "Option::is_none")]
	pub othr_ctr_pty_brnch: Option<TradePartyIdentificationQuery9>,
	#[serde(rename = "Bnfcry", skip_serializing_if = "Option::is_none")]
	pub bnfcry: Option<TradePartyIdentificationQuery8>,
	#[serde(rename = "SubmitgAgt", skip_serializing_if = "Option::is_none")]
	pub submitg_agt: Option<TradePartyIdentificationQuery8>,
	#[serde(rename = "Brkr", skip_serializing_if = "Option::is_none")]
	pub brkr: Option<TradePartyIdentificationQuery8>,
	#[serde(rename = "CCP", skip_serializing_if = "Option::is_none")]
	pub ccp: Option<TradePartyIdentificationQuery8>,
	#[serde(rename = "AgtLndr", skip_serializing_if = "Option::is_none")]
	pub agt_lndr: Option<TradePartyIdentificationQuery8>,
	#[serde(rename = "TrptyAgt", skip_serializing_if = "Option::is_none")]
	pub trpty_agt: Option<TradePartyIdentificationQuery8>,
}

impl TradePartyQueryCriteria5 {
	pub fn validate(&self) -> bool {
		if !self.oprtr.validate() { return false }
		if let Some(ref rptg_ctr_pty_value) = self.rptg_ctr_pty { if !rptg_ctr_pty_value.validate() { return false; } }
		if let Some(ref rptg_ctr_pty_brnch_value) = self.rptg_ctr_pty_brnch { if !rptg_ctr_pty_brnch_value.validate() { return false; } }
		if let Some(ref othr_ctr_pty_value) = self.othr_ctr_pty { if !othr_ctr_pty_value.validate() { return false; } }
		if let Some(ref othr_ctr_pty_brnch_value) = self.othr_ctr_pty_brnch { if !othr_ctr_pty_brnch_value.validate() { return false; } }
		if let Some(ref bnfcry_value) = self.bnfcry { if !bnfcry_value.validate() { return false; } }
		if let Some(ref submitg_agt_value) = self.submitg_agt { if !submitg_agt_value.validate() { return false; } }
		if let Some(ref brkr_value) = self.brkr { if !brkr_value.validate() { return false; } }
		if let Some(ref ccp_value) = self.ccp { if !ccp_value.validate() { return false; } }
		if let Some(ref agt_lndr_value) = self.agt_lndr { if !agt_lndr_value.validate() { return false; } }
		if let Some(ref trpty_agt_value) = self.trpty_agt { if !trpty_agt_value.validate() { return false; } }
		return true
	}
}


// TradeQueryCriteria10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeQueryCriteria10 {
	#[serde(rename = "TradLifeCyclHstry")]
	pub trad_life_cycl_hstry: bool,
	#[serde(rename = "OutsdngTradInd")]
	pub outsdng_trad_ind: bool,
	#[serde(rename = "TradPtyCrit", skip_serializing_if = "Option::is_none")]
	pub trad_pty_crit: Option<TradePartyQueryCriteria5>,
	#[serde(rename = "TradTpCrit", skip_serializing_if = "Option::is_none")]
	pub trad_tp_crit: Option<TradeTypeQueryCriteria2>,
	#[serde(rename = "TmCrit", skip_serializing_if = "Option::is_none")]
	pub tm_crit: Option<TradeDateTimeQueryCriteria2>,
	#[serde(rename = "OthrCrit", skip_serializing_if = "Option::is_none")]
	pub othr_crit: Option<TradeAdditionalQueryCriteria7>,
}

impl TradeQueryCriteria10 {
	pub fn validate(&self) -> bool {
		if let Some(ref trad_pty_crit_value) = self.trad_pty_crit { if !trad_pty_crit_value.validate() { return false; } }
		if let Some(ref trad_tp_crit_value) = self.trad_tp_crit { if !trad_tp_crit_value.validate() { return false; } }
		if let Some(ref tm_crit_value) = self.tm_crit { if !tm_crit_value.validate() { return false; } }
		if let Some(ref othr_crit_value) = self.othr_crit { if !othr_crit_value.validate() { return false; } }
		return true
	}
}


// TradeQueryExecutionFrequency3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeQueryExecutionFrequency3 {
	#[serde(rename = "FrqcyTp")]
	pub frqcy_tp: Frequency14Code,
	#[serde(rename = "DlvryDay", skip_serializing_if = "Option::is_none")]
	pub dlvry_day: Option<Vec<WeekDay3Code>>,
	#[serde(rename = "DayOfMnth", skip_serializing_if = "Option::is_none")]
	pub day_of_mnth: Option<Vec<f64>>,
}

impl TradeQueryExecutionFrequency3 {
	pub fn validate(&self) -> bool {
		if !self.frqcy_tp.validate() { return false }
		if let Some(ref dlvry_day_vec) = self.dlvry_day { for item in dlvry_day_vec { if !item.validate() { return false; } } }
		return true
	}
}


// TradeRecurrentQuery5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeRecurrentQuery5 {
	#[serde(rename = "QryTp")]
	pub qry_tp: Max1000Text,
	#[serde(rename = "Frqcy")]
	pub frqcy: TradeQueryExecutionFrequency3,
	#[serde(rename = "VldUntil")]
	pub vld_until: String,
}

impl TradeRecurrentQuery5 {
	pub fn validate(&self) -> bool {
		if !self.qry_tp.validate() { return false }
		if !self.frqcy.validate() { return false }
		return true
	}
}


// TradeReportQuery13Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeReportQuery13Choice {
	#[serde(rename = "AdHocQry", skip_serializing_if = "Option::is_none")]
	pub ad_hoc_qry: Option<TradeQueryCriteria10>,
	#[serde(rename = "RcrntQry", skip_serializing_if = "Option::is_none")]
	pub rcrnt_qry: Option<TradeRecurrentQuery5>,
}

impl TradeReportQuery13Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref ad_hoc_qry_value) = self.ad_hoc_qry { if !ad_hoc_qry_value.validate() { return false; } }
		if let Some(ref rcrnt_qry_value) = self.rcrnt_qry { if !rcrnt_qry_value.validate() { return false; } }
		return true
	}
}


// TradeTypeQueryCriteria2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeTypeQueryCriteria2 {
	#[serde(rename = "Oprtr")]
	pub oprtr: Operation3Code,
	#[serde(rename = "SctiesFincgTxTp", skip_serializing_if = "Option::is_none")]
	pub scties_fincg_tx_tp: Option<Vec<ExposureType10Code>>,
	#[serde(rename = "CollCmpntTp", skip_serializing_if = "Option::is_none")]
	pub coll_cmpnt_tp: Option<Vec<CollateralType6Code>>,
}

impl TradeTypeQueryCriteria2 {
	pub fn validate(&self) -> bool {
		if !self.oprtr.validate() { return false }
		if let Some(ref scties_fincg_tx_tp_vec) = self.scties_fincg_tx_tp { for item in scties_fincg_tx_tp_vec { if !item.validate() { return false; } } }
		if let Some(ref coll_cmpnt_tp_vec) = self.coll_cmpnt_tp { for item in coll_cmpnt_tp_vec { if !item.validate() { return false; } } }
		return true
	}
}


// TransactionOperationType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TransactionOperationType6Code {
	#[default]
	#[serde(rename = "REUU")]
	CodeREUU,
	#[serde(rename = "COLU")]
	CodeCOLU,
	#[serde(rename = "CORR")]
	CodeCORR,
	#[serde(rename = "ETRM")]
	CodeETRM,
	#[serde(rename = "VALU")]
	CodeVALU,
	#[serde(rename = "POSC")]
	CodePOSC,
	#[serde(rename = "NEWT")]
	CodeNEWT,
	#[serde(rename = "MODI")]
	CodeMODI,
	#[serde(rename = "MARU")]
	CodeMARU,
	#[serde(rename = "EROR")]
	CodeEROR,
}

impl TransactionOperationType6Code {
	pub fn validate(&self) -> bool {
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


// WeekDay3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum WeekDay3Code {
	#[default]
	#[serde(rename = "ALLD")]
	CodeALLD,
	#[serde(rename = "XBHL")]
	CodeXBHL,
	#[serde(rename = "IBHL")]
	CodeIBHL,
	#[serde(rename = "FRID")]
	CodeFRID,
	#[serde(rename = "MOND")]
	CodeMOND,
	#[serde(rename = "SATD")]
	CodeSATD,
	#[serde(rename = "SUND")]
	CodeSUND,
	#[serde(rename = "THUD")]
	CodeTHUD,
	#[serde(rename = "TUED")]
	CodeTUED,
	#[serde(rename = "WEDD")]
	CodeWEDD,
	#[serde(rename = "WDAY")]
	CodeWDAY,
	#[serde(rename = "WEND")]
	CodeWEND,
}

impl WeekDay3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}
