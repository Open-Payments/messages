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


// Contact9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Contact9 {
	#[serde(rename = "Nm")]
	pub nm: Max140Text,
	#[serde(rename = "PhneNb")]
	pub phne_nb: PhoneNumber,
	#[serde(rename = "EmailAdr")]
	pub email_adr: Max256Text,
	#[serde(rename = "Fctn", skip_serializing_if = "Option::is_none")]
	pub fctn: Option<Max140Text>,
}

impl Contact9 {
	pub fn validate(&self) -> bool {
		if !self.nm.validate() { return false }
		if !self.phne_nb.validate() { return false }
		if !self.email_adr.validate() { return false }
		if let Some(ref fctn_value) = self.fctn { if !fctn_value.validate() { return false; } }
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


// DatePeriod2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriod2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}

impl DatePeriod2 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ExternalFinancialInstrumentIdentificationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalFinancialInstrumentIdentificationType1Code {
	#[serde(rename = "$value")]
	pub external_financial_instrument_identification_type1_code: String,
}

impl ExternalFinancialInstrumentIdentificationType1Code {
	pub fn validate(&self) -> bool {
		if self.external_financial_instrument_identification_type1_code.chars().count() < 1 {
			return false
		}
		if self.external_financial_instrument_identification_type1_code.chars().count() > 4 {
			return false
		}
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


// IdentificationSource3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IdentificationSource3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalFinancialInstrumentIdentificationType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl IdentificationSource3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
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


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
}

impl Max140Text {
	pub fn validate(&self) -> bool {
		if self.max140_text.chars().count() < 1 {
			return false
		}
		if self.max140_text.chars().count() > 140 {
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


// Max2048Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max2048Text {
	#[serde(rename = "$value")]
	pub max2048_text: String,
}

impl Max2048Text {
	pub fn validate(&self) -> bool {
		if self.max2048_text.chars().count() < 1 {
			return false
		}
		if self.max2048_text.chars().count() > 2048 {
			return false
		}
		return true
	}
}


// Max20PositiveDecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max20PositiveDecimalNumber {
	#[serde(rename = "$value")]
	pub max20_positive_decimal_number: f64,
}

impl Max20PositiveDecimalNumber {
	pub fn validate(&self) -> bool {
		if self.max20_positive_decimal_number < 0.000000 {
			return false
		}
		return true
	}
}


// Max20PositiveNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max20PositiveNumber {
	#[serde(rename = "$value")]
	pub max20_positive_number: f64,
}

impl Max20PositiveNumber {
	pub fn validate(&self) -> bool {
		if self.max20_positive_number < 0.000000 {
			return false
		}
		return true
	}
}


// Max256Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max256Text {
	#[serde(rename = "$value")]
	pub max256_text: String,
}

impl Max256Text {
	pub fn validate(&self) -> bool {
		if self.max256_text.chars().count() < 1 {
			return false
		}
		if self.max256_text.chars().count() > 256 {
			return false
		}
		return true
	}
}


// Max2Fraction1NonNegativeNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max2Fraction1NonNegativeNumber {
	#[serde(rename = "$value")]
	pub max2_fraction1_non_negative_number: f64,
}

impl Max2Fraction1NonNegativeNumber {
	pub fn validate(&self) -> bool {
		if self.max2_fraction1_non_negative_number < 0.000000 {
			return false
		}
		if self.max2_fraction1_non_negative_number > 9.900000 {
			return false
		}
		return true
	}
}


// Max2NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max2NumericText {
	#[serde(rename = "$value")]
	pub max2_numeric_text: String,
}

impl Max2NumericText {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[0-9]{1,2}").unwrap();
		if !pattern.is_match(&self.max2_numeric_text) {
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


// OtherIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Sfx", skip_serializing_if = "Option::is_none")]
	pub sfx: Option<Max16Text>,
	#[serde(rename = "Tp")]
	pub tp: IdentificationSource3Choice,
}

impl OtherIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref sfx_value) = self.sfx { if !sfx_value.validate() { return false; } }
		if !self.tp.validate() { return false }
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


// PhoneNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PhoneNumber {
	#[serde(rename = "$value")]
	pub phone_number: String,
}

impl PhoneNumber {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
		if !pattern.is_match(&self.phone_number) {
			return false
		}
		return true
	}
}


// ReportPeriodActivity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ReportPeriodActivity1Code {
	#[default]
	#[serde(rename = "NOTX")]
	CodeNOTX,
}

impl ReportPeriodActivity1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// SecuritiesSettlementSystemIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesSettlementSystemIdentification2 {
	#[serde(rename = "SysId")]
	pub sys_id: Max35Text,
	#[serde(rename = "SysNm", skip_serializing_if = "Option::is_none")]
	pub sys_nm: Option<Max140Text>,
	#[serde(rename = "CtryOfJursdctn", skip_serializing_if = "Option::is_none")]
	pub ctry_of_jursdctn: Option<CountryCode>,
	#[serde(rename = "CSDLglNm", skip_serializing_if = "Option::is_none")]
	pub csd_lgl_nm: Option<Max140Text>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "RspnsblPty", skip_serializing_if = "Option::is_none")]
	pub rspnsbl_pty: Option<Vec<Contact9>>,
}

impl SecuritiesSettlementSystemIdentification2 {
	pub fn validate(&self) -> bool {
		if !self.sys_id.validate() { return false }
		if let Some(ref sys_nm_value) = self.sys_nm { if !sys_nm_value.validate() { return false; } }
		if let Some(ref ctry_of_jursdctn_value) = self.ctry_of_jursdctn { if !ctry_of_jursdctn_value.validate() { return false; } }
		if let Some(ref csd_lgl_nm_value) = self.csd_lgl_nm { if !csd_lgl_nm_value.validate() { return false; } }
		if let Some(ref lei_value) = self.lei { if !lei_value.validate() { return false; } }
		if let Some(ref rspnsbl_pty_vec) = self.rspnsbl_pty { for item in rspnsbl_pty_vec { if !item.validate() { return false; } } }
		return true
	}
}


// SecurityIdentification19 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification19 {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
	#[serde(rename = "OthrId", skip_serializing_if = "Option::is_none")]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max140Text>,
}

impl SecurityIdentification19 {
	pub fn validate(&self) -> bool {
		if let Some(ref isin_value) = self.isin { if !isin_value.validate() { return false; } }
		if let Some(ref othr_id_vec) = self.othr_id { for item in othr_id_vec { if !item.validate() { return false; } } }
		if let Some(ref desc_value) = self.desc { if !desc_value.validate() { return false; } }
		return true
	}
}


// SettlementDailyFailureReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementDailyFailureReason1Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[serde(rename = "Data", skip_serializing_if = "Option::is_none")]
	pub data: Option<SettlementDailyFailureReason3>,
}

impl SettlementDailyFailureReason1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref data_set_actn_value) = self.data_set_actn { if !data_set_actn_value.validate() { return false; } }
		if let Some(ref data_value) = self.data { if !data_value.validate() { return false; } }
		return true
	}
}


// SettlementDailyFailureReason3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementDailyFailureReason3 {
	#[serde(rename = "FaildScties")]
	pub faild_scties: SettlementTotalData1Choice,
	#[serde(rename = "FaildCsh")]
	pub faild_csh: SettlementTotalData1Choice,
}

impl SettlementDailyFailureReason3 {
	pub fn validate(&self) -> bool {
		if !self.faild_scties.validate() { return false }
		if !self.faild_csh.validate() { return false }
		return true
	}
}


// SettlementDataRate2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementDataRate2 {
	#[serde(rename = "Vol")]
	pub vol: f64,
	#[serde(rename = "Val")]
	pub val: f64,
}

impl SettlementDataRate2 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// SettlementDataVolume2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementDataVolume2 {
	#[serde(rename = "Vol")]
	pub vol: f64,
	#[serde(rename = "Val")]
	pub val: f64,
}

impl SettlementDataVolume2 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// SettlementFailsCurrency2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsCurrency2 {
	#[serde(rename = "Ccy")]
	pub ccy: ActiveCurrencyCode,
	#[serde(rename = "Data")]
	pub data: SettlementTotalData1,
}

impl SettlementFailsCurrency2 {
	pub fn validate(&self) -> bool {
		if !self.ccy.validate() { return false }
		if !self.data.validate() { return false }
		return true
	}
}


// SettlementFailsDailyCSD1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsDailyCSD1Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[serde(rename = "Data", skip_serializing_if = "Option::is_none")]
	pub data: Option<SettlementFailsDailyCSD3>,
}

impl SettlementFailsDailyCSD1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref data_set_actn_value) = self.data_set_actn { if !data_set_actn_value.validate() { return false; } }
		if let Some(ref data_value) = self.data { if !data_value.validate() { return false; } }
		return true
	}
}


// SettlementFailsDailyCSD3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsDailyCSD3 {
	#[serde(rename = "IntraCSD")]
	pub intra_csd: SettlementFailsDailyInstructionType1Choice,
	#[serde(rename = "CrossCSD")]
	pub cross_csd: SettlementFailsDailyInstructionType1Choice,
}

impl SettlementFailsDailyCSD3 {
	pub fn validate(&self) -> bool {
		if !self.intra_csd.validate() { return false }
		if !self.cross_csd.validate() { return false }
		return true
	}
}


// SettlementFailsDailyData3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsDailyData3 {
	#[serde(rename = "RptgDt")]
	pub rptg_dt: String,
	#[serde(rename = "DalyRcrd")]
	pub daly_rcrd: SettlementFailsDailyInstrument3,
}

impl SettlementFailsDailyData3 {
	pub fn validate(&self) -> bool {
		if !self.daly_rcrd.validate() { return false }
		return true
	}
}


// SettlementFailsDailyInstructionType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsDailyInstructionType1Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[serde(rename = "Data", skip_serializing_if = "Option::is_none")]
	pub data: Option<SettlementFailsDailyInstructionType3>,
}

impl SettlementFailsDailyInstructionType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref data_set_actn_value) = self.data_set_actn { if !data_set_actn_value.validate() { return false; } }
		if let Some(ref data_value) = self.data { if !data_value.validate() { return false; } }
		return true
	}
}


// SettlementFailsDailyInstructionType3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsDailyInstructionType3 {
	#[serde(rename = "DlvryVrssPmt")]
	pub dlvry_vrss_pmt: SettlementDailyFailureReason1Choice,
	#[serde(rename = "DlvryWthPmt")]
	pub dlvry_wth_pmt: SettlementDailyFailureReason1Choice,
	#[serde(rename = "PmtFreeOfDlvry")]
	pub pmt_free_of_dlvry: SettlementDailyFailureReason1Choice,
	#[serde(rename = "FreeOfPmt")]
	pub free_of_pmt: SettlementDailyFailureReason1Choice,
}

impl SettlementFailsDailyInstructionType3 {
	pub fn validate(&self) -> bool {
		if !self.dlvry_vrss_pmt.validate() { return false }
		if !self.dlvry_wth_pmt.validate() { return false }
		if !self.pmt_free_of_dlvry.validate() { return false }
		if !self.free_of_pmt.validate() { return false }
		return true
	}
}


// SettlementFailsDailyInstrument3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsDailyInstrument3 {
	#[serde(rename = "Eqty")]
	pub eqty: SettlementFailsDailyTransactionType1Choice,
	#[serde(rename = "SvrgnDebt")]
	pub svrgn_debt: SettlementFailsDailyTransactionType1Choice,
	#[serde(rename = "Bd")]
	pub bd: SettlementFailsDailyTransactionType1Choice,
	#[serde(rename = "OthrTrfblScties")]
	pub othr_trfbl_scties: SettlementFailsDailyTransactionType1Choice,
	#[serde(rename = "XchgTraddFnds")]
	pub xchg_tradd_fnds: SettlementFailsDailyTransactionType1Choice,
	#[serde(rename = "CllctvInvstmtUdrtkgs")]
	pub cllctv_invstmt_udrtkgs: SettlementFailsDailyTransactionType1Choice,
	#[serde(rename = "MnyMktInstrm")]
	pub mny_mkt_instrm: SettlementFailsDailyTransactionType1Choice,
	#[serde(rename = "EmssnAllwnc")]
	pub emssn_allwnc: SettlementFailsDailyTransactionType1Choice,
	#[serde(rename = "Othr")]
	pub othr: SettlementFailsDailyTransactionType1Choice,
}

impl SettlementFailsDailyInstrument3 {
	pub fn validate(&self) -> bool {
		if !self.eqty.validate() { return false }
		if !self.svrgn_debt.validate() { return false }
		if !self.bd.validate() { return false }
		if !self.othr_trfbl_scties.validate() { return false }
		if !self.xchg_tradd_fnds.validate() { return false }
		if !self.cllctv_invstmt_udrtkgs.validate() { return false }
		if !self.mny_mkt_instrm.validate() { return false }
		if !self.emssn_allwnc.validate() { return false }
		if !self.othr.validate() { return false }
		return true
	}
}


// SettlementFailsDailyTransactionType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsDailyTransactionType1Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[serde(rename = "Data", skip_serializing_if = "Option::is_none")]
	pub data: Option<SettlementFailsDailyTransactionType3>,
}

impl SettlementFailsDailyTransactionType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref data_set_actn_value) = self.data_set_actn { if !data_set_actn_value.validate() { return false; } }
		if let Some(ref data_value) = self.data { if !data_value.validate() { return false; } }
		return true
	}
}


// SettlementFailsDailyTransactionType3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsDailyTransactionType3 {
	#[serde(rename = "SctiesBuyOrSell")]
	pub scties_buy_or_sell: SettlementFailsDailyCSD1Choice,
	#[serde(rename = "CollMgmtOpr")]
	pub coll_mgmt_opr: SettlementFailsDailyCSD1Choice,
	#[serde(rename = "SctiesLndgOrBrrwg")]
	pub scties_lndg_or_brrwg: SettlementFailsDailyCSD1Choice,
	#[serde(rename = "RpAgrmt")]
	pub rp_agrmt: SettlementFailsDailyCSD1Choice,
	#[serde(rename = "Othr")]
	pub othr: SettlementFailsDailyCSD1Choice,
}

impl SettlementFailsDailyTransactionType3 {
	pub fn validate(&self) -> bool {
		if !self.scties_buy_or_sell.validate() { return false }
		if !self.coll_mgmt_opr.validate() { return false }
		if !self.scties_lndg_or_brrwg.validate() { return false }
		if !self.rp_agrmt.validate() { return false }
		if !self.othr.validate() { return false }
		return true
	}
}


// SettlementFailsData3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsData3 {
	#[serde(rename = "Ttl")]
	pub ttl: SettlementTotalData1,
	#[serde(rename = "PtcptInFail", skip_serializing_if = "Option::is_none")]
	pub ptcpt_in_fail: Option<SettlementFailsParticipantRange1>,
	#[serde(rename = "FlsPerCcy", skip_serializing_if = "Option::is_none")]
	pub fls_per_ccy: Option<Vec<SettlementFailsCurrency2>>,
	#[serde(rename = "FlsPerFinInstrmTp", skip_serializing_if = "Option::is_none")]
	pub fls_per_fin_instrm_tp: Option<SettlementFailsInstrument2>,
	#[serde(rename = "SctiesInFail", skip_serializing_if = "Option::is_none")]
	pub scties_in_fail: Option<SettlementFailsSecuritiesRange1>,
	#[serde(rename = "FlsPerTxTp", skip_serializing_if = "Option::is_none")]
	pub fls_per_tx_tp: Option<SettlementFailsTransactionType2>,
	#[serde(rename = "TtlSttlmPnlties", skip_serializing_if = "Option::is_none")]
	pub ttl_sttlm_pnlties: Option<SettlementDataVolume2>,
	#[serde(rename = "FailrRsn")]
	pub failr_rsn: SettlementFailureReason3,
}

impl SettlementFailsData3 {
	pub fn validate(&self) -> bool {
		if !self.ttl.validate() { return false }
		if let Some(ref ptcpt_in_fail_value) = self.ptcpt_in_fail { if !ptcpt_in_fail_value.validate() { return false; } }
		if let Some(ref fls_per_ccy_vec) = self.fls_per_ccy { for item in fls_per_ccy_vec { if !item.validate() { return false; } } }
		if let Some(ref fls_per_fin_instrm_tp_value) = self.fls_per_fin_instrm_tp { if !fls_per_fin_instrm_tp_value.validate() { return false; } }
		if let Some(ref scties_in_fail_value) = self.scties_in_fail { if !scties_in_fail_value.validate() { return false; } }
		if let Some(ref fls_per_tx_tp_value) = self.fls_per_tx_tp { if !fls_per_tx_tp_value.validate() { return false; } }
		if let Some(ref ttl_sttlm_pnlties_value) = self.ttl_sttlm_pnlties { if !ttl_sttlm_pnlties_value.validate() { return false; } }
		if !self.failr_rsn.validate() { return false }
		return true
	}
}


// SettlementFailsInstrument2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsInstrument2 {
	#[serde(rename = "Eqty")]
	pub eqty: SettlementTotalData1Choice,
	#[serde(rename = "SvrgnDebt")]
	pub svrgn_debt: SettlementTotalData1Choice,
	#[serde(rename = "Bd")]
	pub bd: SettlementTotalData1Choice,
	#[serde(rename = "OthrTrfblScties")]
	pub othr_trfbl_scties: SettlementTotalData1Choice,
	#[serde(rename = "XchgTraddFnds")]
	pub xchg_tradd_fnds: SettlementTotalData1Choice,
	#[serde(rename = "CllctvInvstmtUdrtkgs")]
	pub cllctv_invstmt_udrtkgs: SettlementTotalData1Choice,
	#[serde(rename = "MnyMktInstrm")]
	pub mny_mkt_instrm: SettlementTotalData1Choice,
	#[serde(rename = "EmssnAllwnc")]
	pub emssn_allwnc: SettlementTotalData1Choice,
	#[serde(rename = "Othr")]
	pub othr: SettlementTotalData1Choice,
}

impl SettlementFailsInstrument2 {
	pub fn validate(&self) -> bool {
		if !self.eqty.validate() { return false }
		if !self.svrgn_debt.validate() { return false }
		if !self.bd.validate() { return false }
		if !self.othr_trfbl_scties.validate() { return false }
		if !self.xchg_tradd_fnds.validate() { return false }
		if !self.cllctv_invstmt_udrtkgs.validate() { return false }
		if !self.mny_mkt_instrm.validate() { return false }
		if !self.emssn_allwnc.validate() { return false }
		if !self.othr.validate() { return false }
		return true
	}
}


// SettlementFailsMonthlyReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsMonthlyReportV01 {
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: SettlementFailsReportHeader2,
	#[serde(rename = "MnthlyAggt")]
	pub mnthly_aggt: SettlementFailsData3,
	#[serde(rename = "DalyData")]
	pub daly_data: Vec<SettlementFailsDailyData3>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl SettlementFailsMonthlyReportV01 {
	pub fn validate(&self) -> bool {
		if !self.rpt_hdr.validate() { return false }
		if !self.mnthly_aggt.validate() { return false }
		for item in &self.daly_data { if !item.validate() { return false; } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
	}
}


// SettlementFailsParticipant1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsParticipant1 {
	#[serde(rename = "LEI")]
	pub lei: LEIIdentifier,
	#[serde(rename = "Rank")]
	pub rank: Max2NumericText,
	#[serde(rename = "Aggt")]
	pub aggt: SettlementTotalData1,
}

impl SettlementFailsParticipant1 {
	pub fn validate(&self) -> bool {
		if !self.lei.validate() { return false }
		if !self.rank.validate() { return false }
		if !self.aggt.validate() { return false }
		return true
	}
}


// SettlementFailsParticipantRange1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsParticipantRange1 {
	#[serde(rename = "HghstInVol")]
	pub hghst_in_vol: Vec<SettlementFailsParticipant1>,
	#[serde(rename = "HghstInVal")]
	pub hghst_in_val: Vec<SettlementFailsParticipant1>,
}

impl SettlementFailsParticipantRange1 {
	pub fn validate(&self) -> bool {
		for item in &self.hghst_in_vol { if !item.validate() { return false; } }
		for item in &self.hghst_in_val { if !item.validate() { return false; } }
		return true
	}
}


// SettlementFailsReportHeader2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsReportHeader2 {
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
	#[serde(rename = "RptgPrd")]
	pub rptg_prd: DatePeriod2,
	#[serde(rename = "Ccy")]
	pub ccy: ActiveCurrencyCode,
	#[serde(rename = "RptSts")]
	pub rpt_sts: TransactionOperationType4Code,
	#[serde(rename = "SctiesSttlmSys")]
	pub scties_sttlm_sys: SecuritiesSettlementSystemIdentification2,
}

impl SettlementFailsReportHeader2 {
	pub fn validate(&self) -> bool {
		if !self.rptg_prd.validate() { return false }
		if !self.ccy.validate() { return false }
		if !self.rpt_sts.validate() { return false }
		if !self.scties_sttlm_sys.validate() { return false }
		return true
	}
}


// SettlementFailsSecurities1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsSecurities1 {
	#[serde(rename = "FinInstrmId")]
	pub fin_instrm_id: SecurityIdentification19,
	#[serde(rename = "Rank")]
	pub rank: Max2NumericText,
}

impl SettlementFailsSecurities1 {
	pub fn validate(&self) -> bool {
		if !self.fin_instrm_id.validate() { return false }
		if !self.rank.validate() { return false }
		return true
	}
}


// SettlementFailsSecuritiesRange1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsSecuritiesRange1 {
	#[serde(rename = "HghstInVol")]
	pub hghst_in_vol: Vec<SettlementFailsSecurities1>,
	#[serde(rename = "HghstInVal")]
	pub hghst_in_val: Vec<SettlementFailsSecurities1>,
}

impl SettlementFailsSecuritiesRange1 {
	pub fn validate(&self) -> bool {
		for item in &self.hghst_in_vol { if !item.validate() { return false; } }
		for item in &self.hghst_in_val { if !item.validate() { return false; } }
		return true
	}
}


// SettlementFailsTransactionType2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsTransactionType2 {
	#[serde(rename = "SctiesBuyOrSell")]
	pub scties_buy_or_sell: SettlementTotalData1Choice,
	#[serde(rename = "CollMgmtOpr")]
	pub coll_mgmt_opr: SettlementTotalData1Choice,
	#[serde(rename = "SctiesLndgOrBrrwg")]
	pub scties_lndg_or_brrwg: SettlementTotalData1Choice,
	#[serde(rename = "RpAgrmt")]
	pub rp_agrmt: SettlementTotalData1Choice,
	#[serde(rename = "Othr")]
	pub othr: SettlementTotalData1Choice,
}

impl SettlementFailsTransactionType2 {
	pub fn validate(&self) -> bool {
		if !self.scties_buy_or_sell.validate() { return false }
		if !self.coll_mgmt_opr.validate() { return false }
		if !self.scties_lndg_or_brrwg.validate() { return false }
		if !self.rp_agrmt.validate() { return false }
		if !self.othr.validate() { return false }
		return true
	}
}


// SettlementFailureReason2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailureReason2 {
	#[serde(rename = "MainRsns")]
	pub main_rsns: Max2048Text,
	#[serde(rename = "EffcncyImprvmt")]
	pub effcncy_imprvmt: Max2048Text,
}

impl SettlementFailureReason2 {
	pub fn validate(&self) -> bool {
		if !self.main_rsns.validate() { return false }
		if !self.effcncy_imprvmt.validate() { return false }
		return true
	}
}


// SettlementFailureReason3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailureReason3 {
	#[serde(rename = "AvrgDrtn", skip_serializing_if = "Option::is_none")]
	pub avrg_drtn: Option<f64>,
	#[serde(rename = "Desc")]
	pub desc: Vec<SettlementFailureReason2>,
}

impl SettlementFailureReason3 {
	pub fn validate(&self) -> bool {
		for item in &self.desc { if !item.validate() { return false; } }
		return true
	}
}


// SettlementTotalData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementTotalData1 {
	#[serde(rename = "Sttld")]
	pub sttld: SettlementDataVolume2,
	#[serde(rename = "Faild")]
	pub faild: SettlementDataVolume2,
	#[serde(rename = "Ttl")]
	pub ttl: SettlementDataVolume2,
	#[serde(rename = "FaildRate")]
	pub faild_rate: SettlementDataRate2,
}

impl SettlementTotalData1 {
	pub fn validate(&self) -> bool {
		if !self.sttld.validate() { return false }
		if !self.faild.validate() { return false }
		if !self.ttl.validate() { return false }
		if !self.faild_rate.validate() { return false }
		return true
	}
}


// SettlementTotalData1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementTotalData1Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[serde(rename = "Data", skip_serializing_if = "Option::is_none")]
	pub data: Option<SettlementTotalData1>,
}

impl SettlementTotalData1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref data_set_actn_value) = self.data_set_actn { if !data_set_actn_value.validate() { return false; } }
		if let Some(ref data_value) = self.data { if !data_value.validate() { return false; } }
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


// TransactionOperationType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TransactionOperationType4Code {
	#[default]
	#[serde(rename = "NEWT")]
	CodeNEWT,
	#[serde(rename = "AMND")]
	CodeAMND,
	#[serde(rename = "CANC")]
	CodeCANC,
}

impl TransactionOperationType4Code {
	pub fn validate(&self) -> bool {
		return true
	}
}
