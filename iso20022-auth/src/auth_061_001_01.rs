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


// ActiveCurrencyAnd24AmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyAnd24AmountSimpleType {
	#[serde(rename = "$value")]
	pub active_currency_and24_amount_simple_type: f64,
}

impl ActiveCurrencyAnd24AmountSimpleType {
	pub fn validate(&self) -> bool {
		if self.active_currency_and24_amount_simple_type < 0.000000 {
			return false
		}
		return true
	}
}


// ActiveCurrencyAnd24Amount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd24Amount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveCurrencyAnd24Amount {
	pub fn validate(&self) -> bool {
		return true
	}
}


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


// CCPInvestmentsReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CCPInvestmentsReportV01 {
	#[serde(rename = "Invstmt")]
	pub invstmt: Vec<Investment1Choice>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CCPInvestmentsReportV01 {
	pub fn validate(&self) -> bool {
		for item in &self.invstmt { if !item.validate() { return false; } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
	}
}


// Deposit1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Deposit1 {
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: String,
	#[serde(rename = "Val")]
	pub val: ActiveCurrencyAndAmount,
	#[serde(rename = "CtrPtyId")]
	pub ctr_pty_id: LEIIdentifier,
}

impl Deposit1 {
	pub fn validate(&self) -> bool {
		if !self.val.validate() { return false }
		if !self.ctr_pty_id.validate() { return false }
		return true
	}
}


// FinancialInstrument59 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument59 {
	#[serde(rename = "Id")]
	pub id: ISINOct2015Identifier,
	#[serde(rename = "Issr")]
	pub issr: LEIIdentifier,
	#[serde(rename = "Sctr", skip_serializing_if = "Option::is_none")]
	pub sctr: Option<String>,
}

impl FinancialInstrument59 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.issr.validate() { return false }
		return true
	}
}


// GeneralCollateral3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GeneralCollateral3 {
	#[serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_id: Option<Vec<FinancialInstrument59>>,
	#[serde(rename = "ElgblFinInstrmId", skip_serializing_if = "Option::is_none")]
	pub elgbl_fin_instrm_id: Option<Vec<ISINOct2015Identifier>>,
}

impl GeneralCollateral3 {
	pub fn validate(&self) -> bool {
		if let Some(ref fin_instrm_id_vec) = self.fin_instrm_id { for item in fin_instrm_id_vec { if !item.validate() { return false; } } }
		if let Some(ref elgbl_fin_instrm_id_vec) = self.elgbl_fin_instrm_id { for item in elgbl_fin_instrm_id_vec { if !item.validate() { return false; } } }
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


// Investment1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Investment1Choice {
	#[serde(rename = "UscrdCshDpst", skip_serializing_if = "Option::is_none")]
	pub uscrd_csh_dpst: Option<Deposit1>,
	#[serde(rename = "CntrlBkDpst", skip_serializing_if = "Option::is_none")]
	pub cntrl_bk_dpst: Option<Deposit1>,
	#[serde(rename = "RpAgrmt", skip_serializing_if = "Option::is_none")]
	pub rp_agrmt: Option<RepurchaseAgreement2>,
	#[serde(rename = "OthrInvstmts", skip_serializing_if = "Option::is_none")]
	pub othr_invstmts: Option<OtherInvestment1>,
	#[serde(rename = "OutrghtInvstmt", skip_serializing_if = "Option::is_none")]
	pub outrght_invstmt: Option<SecurityIdentificationAndAmount1>,
}

impl Investment1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref uscrd_csh_dpst_value) = self.uscrd_csh_dpst { if !uscrd_csh_dpst_value.validate() { return false; } }
		if let Some(ref cntrl_bk_dpst_value) = self.cntrl_bk_dpst { if !cntrl_bk_dpst_value.validate() { return false; } }
		if let Some(ref rp_agrmt_value) = self.rp_agrmt { if !rp_agrmt_value.validate() { return false; } }
		if let Some(ref othr_invstmts_value) = self.othr_invstmts { if !othr_invstmts_value.validate() { return false; } }
		if let Some(ref outrght_invstmt_value) = self.outrght_invstmt { if !outrght_invstmt_value.validate() { return false; } }
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


// OtherInvestment1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherInvestment1 {
	#[serde(rename = "Desc")]
	pub desc: Max140Text,
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
}

impl OtherInvestment1 {
	pub fn validate(&self) -> bool {
		if !self.desc.validate() { return false }
		if !self.amt.validate() { return false }
		return true
	}
}


// ProductType7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ProductType7Code {
	#[default]
	#[serde(rename = "SVGN")]
	CodeSVGN,
	#[serde(rename = "EQUI")]
	CodeEQUI,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}

impl ProductType7Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// RepurchaseAgreement2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RepurchaseAgreement2 {
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: String,
	#[serde(rename = "ScndLegPric")]
	pub scnd_leg_pric: ActiveCurrencyAndAmount,
	#[serde(rename = "CollMktVal")]
	pub coll_mkt_val: ActiveCurrencyAndAmount,
	#[serde(rename = "CtrPty")]
	pub ctr_pty: LEIIdentifier,
	#[serde(rename = "RpAgrmtTp")]
	pub rp_agrmt_tp: RepurchaseAgreementType3Choice,
	#[serde(rename = "TrptyAgtId", skip_serializing_if = "Option::is_none")]
	pub trpty_agt_id: Option<LEIIdentifier>,
}

impl RepurchaseAgreement2 {
	pub fn validate(&self) -> bool {
		if !self.scnd_leg_pric.validate() { return false }
		if !self.coll_mkt_val.validate() { return false }
		if !self.ctr_pty.validate() { return false }
		if !self.rp_agrmt_tp.validate() { return false }
		if let Some(ref trpty_agt_id_value) = self.trpty_agt_id { if !trpty_agt_id_value.validate() { return false; } }
		return true
	}
}


// RepurchaseAgreementType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RepurchaseAgreementType3Choice {
	#[serde(rename = "SpcfcColl", skip_serializing_if = "Option::is_none")]
	pub spcfc_coll: Option<SpecificCollateral2>,
	#[serde(rename = "GnlColl", skip_serializing_if = "Option::is_none")]
	pub gnl_coll: Option<GeneralCollateral3>,
}

impl RepurchaseAgreementType3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref spcfc_coll_value) = self.spcfc_coll { if !spcfc_coll_value.validate() { return false; } }
		if let Some(ref gnl_coll_value) = self.gnl_coll { if !gnl_coll_value.validate() { return false; } }
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


// SecurityIdentificationAndAmount1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentificationAndAmount1 {
	#[serde(rename = "Id")]
	pub id: ISINOct2015Identifier,
	#[serde(rename = "MktVal")]
	pub mkt_val: ActiveCurrencyAnd24Amount,
	#[serde(rename = "FinInstrmTp")]
	pub fin_instrm_tp: ProductType7Code,
}

impl SecurityIdentificationAndAmount1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.mkt_val.validate() { return false }
		if !self.fin_instrm_tp.validate() { return false }
		return true
	}
}


// SpecificCollateral2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SpecificCollateral2 {
	#[serde(rename = "FinInstrmId")]
	pub fin_instrm_id: FinancialInstrument59,
}

impl SpecificCollateral2 {
	pub fn validate(&self) -> bool {
		if !self.fin_instrm_id.validate() { return false }
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
