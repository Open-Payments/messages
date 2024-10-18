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


// SettlementDataRate1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementDataRate1Choice {
	#[serde(rename = "NbOfInstrs", skip_serializing_if = "Option::is_none")]
	pub nb_of_instrs: Option<f64>,
	#[serde(rename = "ValOfInstrs", skip_serializing_if = "Option::is_none")]
	pub val_of_instrs: Option<f64>,
}

impl SettlementDataRate1Choice {
	pub fn validate(&self) -> bool {
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


// SettlementFailsAnnualReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsAnnualReportV01 {
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: SettlementFailsReportHeader2,
	#[serde(rename = "AnlAggt")]
	pub anl_aggt: SettlementFailsData4,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl SettlementFailsAnnualReportV01 {
	pub fn validate(&self) -> bool {
		if !self.rpt_hdr.validate() { return false }
		if !self.anl_aggt.validate() { return false }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
	}
}


// SettlementFailsData4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsData4 {
	#[serde(rename = "Ttl")]
	pub ttl: SettlementTotalData1,
	#[serde(rename = "FailrRsn")]
	pub failr_rsn: SettlementFailureReason3,
	#[serde(rename = "ElgblForDrgtn")]
	pub elgbl_for_drgtn: SettlementFailsDerogation1,
}

impl SettlementFailsData4 {
	pub fn validate(&self) -> bool {
		if !self.ttl.validate() { return false }
		if !self.failr_rsn.validate() { return false }
		if !self.elgbl_for_drgtn.validate() { return false }
		return true
	}
}


// SettlementFailsDerogation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsDerogation1 {
	#[serde(rename = "ElgbltyInd")]
	pub elgblty_ind: bool,
	#[serde(rename = "Justfn", skip_serializing_if = "Option::is_none")]
	pub justfn: Option<SettlementFailsJustification1>,
}

impl SettlementFailsDerogation1 {
	pub fn validate(&self) -> bool {
		if let Some(ref justfn_value) = self.justfn { if !justfn_value.validate() { return false; } }
		return true
	}
}


// SettlementFailsJustification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFailsJustification1 {
	#[serde(rename = "Val")]
	pub val: f64,
	#[serde(rename = "Rate")]
	pub rate: SettlementDataRate1Choice,
}

impl SettlementFailsJustification1 {
	pub fn validate(&self) -> bool {
		if !self.rate.validate() { return false }
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
