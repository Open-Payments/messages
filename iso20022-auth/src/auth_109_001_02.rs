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


// ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_and20_decimal_amount_simple_type: f64,
}

impl ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType {
	pub fn validate(&self) -> bool {
		if self.active_or_historic_currency_and20_decimal_amount_simple_type < 0.000000 {
			return false
		}
		return true
	}
}


// ActiveOrHistoricCurrencyAnd20DecimalAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd20DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveOrHistoricCurrencyAnd20DecimalAmount {
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


// CollateralPortfolioCode6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralPortfolioCode6Choice {
	#[serde(rename = "Prtfl", skip_serializing_if = "Option::is_none")]
	pub prtfl: Option<PortfolioCode3Choice>,
	#[serde(rename = "MrgnPrtflCd", skip_serializing_if = "Option::is_none")]
	pub mrgn_prtfl_cd: Option<MarginPortfolio4>,
}

impl CollateralPortfolioCode6Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref prtfl_value) = self.prtfl { if !prtfl_value.validate() { return false; } }
		if let Some(ref mrgn_prtfl_cd_value) = self.mrgn_prtfl_cd { if !mrgn_prtfl_cd_value.validate() { return false; } }
		return true
	}
}


// CollateralisationType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CollateralisationType3Code {
	#[default]
	#[serde(rename = "FLCL")]
	CodeFLCL,
	#[serde(rename = "OWCL")]
	CodeOWCL,
	#[serde(rename = "OWC1")]
	CodeOWC1,
	#[serde(rename = "OWC2")]
	CodeOWC2,
	#[serde(rename = "OWP1")]
	CodeOWP1,
	#[serde(rename = "OWP2")]
	CodeOWP2,
	#[serde(rename = "PRCL")]
	CodePRCL,
	#[serde(rename = "PRC1")]
	CodePRC1,
	#[serde(rename = "PRC2")]
	CodePRC2,
	#[serde(rename = "UNCL")]
	CodeUNCL,
}

impl CollateralisationType3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ContractModification8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractModification8 {
	#[serde(rename = "ActnTp")]
	pub actn_tp: TransactionOperationType11Code,
}

impl ContractModification8 {
	pub fn validate(&self) -> bool {
		if !self.actn_tp.validate() { return false }
		return true
	}
}


// Counterparty45 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Counterparty45 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification248Choice,
	#[serde(rename = "Ntr", skip_serializing_if = "Option::is_none")]
	pub ntr: Option<CounterpartyTradeNature15Choice>,
	#[serde(rename = "TradgCpcty", skip_serializing_if = "Option::is_none")]
	pub tradg_cpcty: Option<TradingCapacity7Code>,
	#[serde(rename = "DrctnOrSd", skip_serializing_if = "Option::is_none")]
	pub drctn_or_sd: Option<Direction4Choice>,
	#[serde(rename = "TradrLctn", skip_serializing_if = "Option::is_none")]
	pub tradr_lctn: Option<CountryCode>,
	#[serde(rename = "BookgLctn", skip_serializing_if = "Option::is_none")]
	pub bookg_lctn: Option<CountryCode>,
	#[serde(rename = "RptgXmptn", skip_serializing_if = "Option::is_none")]
	pub rptg_xmptn: Option<ReportingExemption1>,
}

impl Counterparty45 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref ntr_value) = self.ntr { if !ntr_value.validate() { return false; } }
		if let Some(ref tradg_cpcty_value) = self.tradg_cpcty { if !tradg_cpcty_value.validate() { return false; } }
		if let Some(ref drctn_or_sd_value) = self.drctn_or_sd { if !drctn_or_sd_value.validate() { return false; } }
		if let Some(ref tradr_lctn_value) = self.tradr_lctn { if !tradr_lctn_value.validate() { return false; } }
		if let Some(ref bookg_lctn_value) = self.bookg_lctn { if !bookg_lctn_value.validate() { return false; } }
		if let Some(ref rptg_xmptn_value) = self.rptg_xmptn { if !rptg_xmptn_value.validate() { return false; } }
		return true
	}
}


// Counterparty46 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Counterparty46 {
	#[serde(rename = "IdTp", skip_serializing_if = "Option::is_none")]
	pub id_tp: Option<PartyIdentification248Choice>,
	#[serde(rename = "Ntr", skip_serializing_if = "Option::is_none")]
	pub ntr: Option<CounterpartyTradeNature15Choice>,
	#[serde(rename = "RptgOblgtn", skip_serializing_if = "Option::is_none")]
	pub rptg_oblgtn: Option<bool>,
}

impl Counterparty46 {
	pub fn validate(&self) -> bool {
		if let Some(ref id_tp_value) = self.id_tp { if !id_tp_value.validate() { return false; } }
		if let Some(ref ntr_value) = self.ntr { if !ntr_value.validate() { return false; } }
		return true
	}
}


// CounterpartyTradeNature15Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyTradeNature15Choice {
	#[serde(rename = "FI", skip_serializing_if = "Option::is_none")]
	pub fi: Option<FinancialInstitutionSector1>,
	#[serde(rename = "NFI", skip_serializing_if = "Option::is_none")]
	pub nfi: Option<NonFinancialInstitutionSector10>,
	#[serde(rename = "CntrlCntrPty", skip_serializing_if = "Option::is_none")]
	pub cntrl_cntr_pty: Option<NoReasonCode>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<NoReasonCode>,
}

impl CounterpartyTradeNature15Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref fi_value) = self.fi { if !fi_value.validate() { return false; } }
		if let Some(ref nfi_value) = self.nfi { if !nfi_value.validate() { return false; } }
		if let Some(ref cntrl_cntr_pty_value) = self.cntrl_cntr_pty { if !cntrl_cntr_pty_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
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


// DerivativesTradeMarginDataTransactionStateReportV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativesTradeMarginDataTransactionStateReportV02 {
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: TradeReportHeader4,
	#[serde(rename = "TradData")]
	pub trad_data: TradeData62Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl DerivativesTradeMarginDataTransactionStateReportV02 {
	pub fn validate(&self) -> bool {
		if !self.rpt_hdr.validate() { return false }
		if !self.trad_data.validate() { return false }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
	}
}


// Direction2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Direction2 {
	#[serde(rename = "DrctnOfTheFrstLeg")]
	pub drctn_of_the_frst_leg: OptionParty3Code,
	#[serde(rename = "DrctnOfTheScndLeg", skip_serializing_if = "Option::is_none")]
	pub drctn_of_the_scnd_leg: Option<OptionParty3Code>,
}

impl Direction2 {
	pub fn validate(&self) -> bool {
		if !self.drctn_of_the_frst_leg.validate() { return false }
		if let Some(ref drctn_of_the_scnd_leg_value) = self.drctn_of_the_scnd_leg { if !drctn_of_the_scnd_leg_value.validate() { return false; } }
		return true
	}
}


// Direction4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Direction4Choice {
	#[serde(rename = "Drctn", skip_serializing_if = "Option::is_none")]
	pub drctn: Option<Direction2>,
	#[serde(rename = "CtrPtySd", skip_serializing_if = "Option::is_none")]
	pub ctr_pty_sd: Option<OptionParty1Code>,
}

impl Direction4Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref drctn_value) = self.drctn { if !drctn_value.validate() { return false; } }
		if let Some(ref ctr_pty_sd_value) = self.ctr_pty_sd { if !ctr_pty_sd_value.validate() { return false; } }
		return true
	}
}


// ExternalPartyRelationshipType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalPartyRelationshipType1Code {
	#[serde(rename = "$value")]
	pub external_party_relationship_type1_code: String,
}

impl ExternalPartyRelationshipType1Code {
	pub fn validate(&self) -> bool {
		if self.external_party_relationship_type1_code.chars().count() < 1 {
			return false
		}
		if self.external_party_relationship_type1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// FinancialInstitutionSector1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstitutionSector1 {
	#[serde(rename = "Sctr")]
	pub sctr: Vec<FinancialPartyClassification2Choice>,
	#[serde(rename = "ClrThrshld", skip_serializing_if = "Option::is_none")]
	pub clr_thrshld: Option<bool>,
}

impl FinancialInstitutionSector1 {
	pub fn validate(&self) -> bool {
		for item in &self.sctr { if !item.validate() { return false; } }
		return true
	}
}


// FinancialPartyClassification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialPartyClassification2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<FinancialPartySectorType3Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification175>,
}

impl FinancialPartyClassification2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// FinancialPartySectorType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FinancialPartySectorType3Code {
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
	#[serde(rename = "ASSU")]
	CodeASSU,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}

impl FinancialPartySectorType3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// GenericIdentification175 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification175 {
	#[serde(rename = "Id")]
	pub id: Max72Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericIdentification175 {
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


// LegalPersonIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LegalPersonIdentification1 {
	#[serde(rename = "Id")]
	pub id: OrganisationIdentification15Choice,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
}

impl LegalPersonIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref ctry_value) = self.ctry { if !ctry_value.validate() { return false; } }
		return true
	}
}


// MarginCollateralReport5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarginCollateralReport5 {
	#[serde(rename = "CollPrtflCd")]
	pub coll_prtfl_cd: CollateralPortfolioCode6Choice,
	#[serde(rename = "CollstnCtgy")]
	pub collstn_ctgy: CollateralisationType3Code,
	#[serde(rename = "TmStmp", skip_serializing_if = "Option::is_none")]
	pub tm_stmp: Option<String>,
}

impl MarginCollateralReport5 {
	pub fn validate(&self) -> bool {
		if !self.coll_prtfl_cd.validate() { return false }
		if !self.collstn_ctgy.validate() { return false }
		return true
	}
}


// MarginPortfolio4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarginPortfolio4 {
	#[serde(rename = "InitlMrgnPrtflCd", skip_serializing_if = "Option::is_none")]
	pub initl_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
	#[serde(rename = "VartnMrgnPrtflCd", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
}

impl MarginPortfolio4 {
	pub fn validate(&self) -> bool {
		if let Some(ref initl_mrgn_prtfl_cd_value) = self.initl_mrgn_prtfl_cd { if !initl_mrgn_prtfl_cd_value.validate() { return false; } }
		if let Some(ref vartn_mrgn_prtfl_cd_value) = self.vartn_mrgn_prtfl_cd { if !vartn_mrgn_prtfl_cd_value.validate() { return false; } }
		return true
	}
}


// MarginReportData10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarginReportData10 {
	#[serde(rename = "RptgTmStmp", skip_serializing_if = "Option::is_none")]
	pub rptg_tm_stmp: Option<String>,
	#[serde(rename = "CtrPtyId")]
	pub ctr_pty_id: TradeCounterpartyReport20,
	#[serde(rename = "EvtDt", skip_serializing_if = "Option::is_none")]
	pub evt_dt: Option<String>,
	#[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
	pub tx_id: Option<UniqueTransactionIdentifier2Choice>,
	#[serde(rename = "Coll")]
	pub coll: MarginCollateralReport5,
	#[serde(rename = "PstdMrgnOrColl", skip_serializing_if = "Option::is_none")]
	pub pstd_mrgn_or_coll: Option<PostedMarginOrCollateral6>,
	#[serde(rename = "RcvdMrgnOrColl", skip_serializing_if = "Option::is_none")]
	pub rcvd_mrgn_or_coll: Option<ReceivedMarginOrCollateral6>,
	#[serde(rename = "CtrPtyRatgTrggrInd", skip_serializing_if = "Option::is_none")]
	pub ctr_pty_ratg_trggr_ind: Option<bool>,
	#[serde(rename = "CtrPtyRatgThrshldInd", skip_serializing_if = "Option::is_none")]
	pub ctr_pty_ratg_thrshld_ind: Option<bool>,
	#[serde(rename = "CtrctMod", skip_serializing_if = "Option::is_none")]
	pub ctrct_mod: Option<ContractModification8>,
	#[serde(rename = "TechAttrbts", skip_serializing_if = "Option::is_none")]
	pub tech_attrbts: Option<TechnicalAttributes6>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl MarginReportData10 {
	pub fn validate(&self) -> bool {
		if !self.ctr_pty_id.validate() { return false }
		if let Some(ref tx_id_value) = self.tx_id { if !tx_id_value.validate() { return false; } }
		if !self.coll.validate() { return false }
		if let Some(ref pstd_mrgn_or_coll_value) = self.pstd_mrgn_or_coll { if !pstd_mrgn_or_coll_value.validate() { return false; } }
		if let Some(ref rcvd_mrgn_or_coll_value) = self.rcvd_mrgn_or_coll { if !rcvd_mrgn_or_coll_value.validate() { return false; } }
		if let Some(ref ctrct_mod_value) = self.ctrct_mod { if !ctrct_mod_value.validate() { return false; } }
		if let Some(ref tech_attrbts_value) = self.tech_attrbts { if !tech_attrbts_value.validate() { return false; } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
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


// Max100Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max100Text {
	#[serde(rename = "$value")]
	pub max100_text: String,
}

impl Max100Text {
	pub fn validate(&self) -> bool {
		if self.max100_text.chars().count() < 1 {
			return false
		}
		if self.max100_text.chars().count() > 100 {
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


// Max4Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max4Text {
	#[serde(rename = "$value")]
	pub max4_text: String,
}

impl Max4Text {
	pub fn validate(&self) -> bool {
		if self.max4_text.chars().count() < 1 {
			return false
		}
		if self.max4_text.chars().count() > 4 {
			return false
		}
		return true
	}
}


// Max500Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max500Text {
	#[serde(rename = "$value")]
	pub max500_text: String,
}

impl Max500Text {
	pub fn validate(&self) -> bool {
		if self.max500_text.chars().count() < 1 {
			return false
		}
		if self.max500_text.chars().count() > 500 {
			return false
		}
		return true
	}
}


// Max52Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max52Text {
	#[serde(rename = "$value")]
	pub max52_text: String,
}

impl Max52Text {
	pub fn validate(&self) -> bool {
		if self.max52_text.chars().count() < 1 {
			return false
		}
		if self.max52_text.chars().count() > 52 {
			return false
		}
		return true
	}
}


// Max5NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max5NumericText {
	#[serde(rename = "$value")]
	pub max5_numeric_text: String,
}

impl Max5NumericText {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[0-9]{1,5}").unwrap();
		if !pattern.is_match(&self.max5_numeric_text) {
			return false
		}
		return true
	}
}


// Max72Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max72Text {
	#[serde(rename = "$value")]
	pub max72_text: String,
}

impl Max72Text {
	pub fn validate(&self) -> bool {
		if self.max72_text.chars().count() < 1 {
			return false
		}
		if self.max72_text.chars().count() > 72 {
			return false
		}
		return true
	}
}


// NaturalPersonIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NaturalPersonIdentification2 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max105Text>,
	#[serde(rename = "Dmcl", skip_serializing_if = "Option::is_none")]
	pub dmcl: Option<Max500Text>,
}

impl NaturalPersonIdentification2 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref dmcl_value) = self.dmcl { if !dmcl_value.validate() { return false; } }
		return true
	}
}


// NaturalPersonIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NaturalPersonIdentification3 {
	#[serde(rename = "Id")]
	pub id: NaturalPersonIdentification2,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
}

impl NaturalPersonIdentification3 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref ctry_value) = self.ctry { if !ctry_value.validate() { return false; } }
		return true
	}
}


// NoReasonCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NoReasonCode {
	#[default]
	#[serde(rename = "NORE")]
	CodeNORE,
}

impl NoReasonCode {
	pub fn validate(&self) -> bool {
		return true
	}
}


// NonFinancialInstitutionSector10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NonFinancialInstitutionSector10 {
	#[serde(rename = "Sctr")]
	pub sctr: Vec<GenericIdentification175>,
	#[serde(rename = "ClrThrshld", skip_serializing_if = "Option::is_none")]
	pub clr_thrshld: Option<bool>,
	#[serde(rename = "DrctlyLkdActvty", skip_serializing_if = "Option::is_none")]
	pub drctly_lkd_actvty: Option<bool>,
	#[serde(rename = "FdrlInstn", skip_serializing_if = "Option::is_none")]
	pub fdrl_instn: Option<bool>,
}

impl NonFinancialInstitutionSector10 {
	pub fn validate(&self) -> bool {
		for item in &self.sctr { if !item.validate() { return false; } }
		return true
	}
}


// NotApplicable1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NotApplicable1Code {
	#[default]
	#[serde(rename = "NOAP")]
	CodeNOAP,
}

impl NotApplicable1Code {
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


// OptionParty1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OptionParty1Code {
	#[default]
	#[serde(rename = "SLLR")]
	CodeSLLR,
	#[serde(rename = "BYER")]
	CodeBYER,
}

impl OptionParty1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// OptionParty3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OptionParty3Code {
	#[default]
	#[serde(rename = "MAKE")]
	CodeMAKE,
	#[serde(rename = "TAKE")]
	CodeTAKE,
}

impl OptionParty3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// OrganisationIdentification15Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification15Choice {
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<OrganisationIdentification38>,
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
}

impl OrganisationIdentification15Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref lei_value) = self.lei { if !lei_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		if let Some(ref any_bic_value) = self.any_bic { if !any_bic_value.validate() { return false; } }
		return true
	}
}


// OrganisationIdentification38 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification38 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max105Text>,
	#[serde(rename = "Dmcl", skip_serializing_if = "Option::is_none")]
	pub dmcl: Option<Max500Text>,
}

impl OrganisationIdentification38 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref dmcl_value) = self.dmcl { if !dmcl_value.validate() { return false; } }
		return true
	}
}


// Pagination1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Pagination1 {
	#[serde(rename = "PgNb")]
	pub pg_nb: Max5NumericText,
	#[serde(rename = "LastPgInd")]
	pub last_pg_ind: bool,
}

impl Pagination1 {
	pub fn validate(&self) -> bool {
		if !self.pg_nb.validate() { return false }
		return true
	}
}


// PartyIdentification248Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification248Choice {
	#[serde(rename = "Lgl", skip_serializing_if = "Option::is_none")]
	pub lgl: Option<LegalPersonIdentification1>,
	#[serde(rename = "Ntrl", skip_serializing_if = "Option::is_none")]
	pub ntrl: Option<NaturalPersonIdentification3>,
}

impl PartyIdentification248Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref lgl_value) = self.lgl { if !lgl_value.validate() { return false; } }
		if let Some(ref ntrl_value) = self.ntrl { if !ntrl_value.validate() { return false; } }
		return true
	}
}


// PortfolioCode3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioCode3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Max52Text>,
	#[serde(rename = "NoPrtfl", skip_serializing_if = "Option::is_none")]
	pub no_prtfl: Option<NotApplicable1Code>,
}

impl PortfolioCode3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref no_prtfl_value) = self.no_prtfl { if !no_prtfl_value.validate() { return false; } }
		return true
	}
}


// PortfolioCode5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioCode5Choice {
	#[serde(rename = "Prtfl", skip_serializing_if = "Option::is_none")]
	pub prtfl: Option<PortfolioIdentification3>,
	#[serde(rename = "NoPrtfl", skip_serializing_if = "Option::is_none")]
	pub no_prtfl: Option<NotApplicable1Code>,
}

impl PortfolioCode5Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref prtfl_value) = self.prtfl { if !prtfl_value.validate() { return false; } }
		if let Some(ref no_prtfl_value) = self.no_prtfl { if !no_prtfl_value.validate() { return false; } }
		return true
	}
}


// PortfolioIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioIdentification3 {
	#[serde(rename = "Cd")]
	pub cd: Max52Text,
	#[serde(rename = "PrtflTxXmptn", skip_serializing_if = "Option::is_none")]
	pub prtfl_tx_xmptn: Option<bool>,
}

impl PortfolioIdentification3 {
	pub fn validate(&self) -> bool {
		if !self.cd.validate() { return false }
		return true
	}
}


// PostedMarginOrCollateral6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostedMarginOrCollateral6 {
	#[serde(rename = "InitlMrgnPstdPreHrcut", skip_serializing_if = "Option::is_none")]
	pub initl_mrgn_pstd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "InitlMrgnPstdPstHrcut", skip_serializing_if = "Option::is_none")]
	pub initl_mrgn_pstd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "VartnMrgnPstdPreHrcut", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_pstd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "VartnMrgnPstdPstHrcut", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_pstd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "XcssCollPstd", skip_serializing_if = "Option::is_none")]
	pub xcss_coll_pstd: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
}

impl PostedMarginOrCollateral6 {
	pub fn validate(&self) -> bool {
		if let Some(ref initl_mrgn_pstd_pre_hrcut_value) = self.initl_mrgn_pstd_pre_hrcut { if !initl_mrgn_pstd_pre_hrcut_value.validate() { return false; } }
		if let Some(ref initl_mrgn_pstd_pst_hrcut_value) = self.initl_mrgn_pstd_pst_hrcut { if !initl_mrgn_pstd_pst_hrcut_value.validate() { return false; } }
		if let Some(ref vartn_mrgn_pstd_pre_hrcut_value) = self.vartn_mrgn_pstd_pre_hrcut { if !vartn_mrgn_pstd_pre_hrcut_value.validate() { return false; } }
		if let Some(ref vartn_mrgn_pstd_pst_hrcut_value) = self.vartn_mrgn_pstd_pst_hrcut { if !vartn_mrgn_pstd_pst_hrcut_value.validate() { return false; } }
		if let Some(ref xcss_coll_pstd_value) = self.xcss_coll_pstd { if !xcss_coll_pstd_value.validate() { return false; } }
		return true
	}
}


// ReceivedMarginOrCollateral6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReceivedMarginOrCollateral6 {
	#[serde(rename = "InitlMrgnRcvdPreHrcut", skip_serializing_if = "Option::is_none")]
	pub initl_mrgn_rcvd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "InitlMrgnRcvdPstHrcut", skip_serializing_if = "Option::is_none")]
	pub initl_mrgn_rcvd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "VartnMrgnRcvdPreHrcut", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_rcvd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "VartnMrgnRcvdPstHrcut", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_rcvd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "XcssCollRcvd", skip_serializing_if = "Option::is_none")]
	pub xcss_coll_rcvd: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
}

impl ReceivedMarginOrCollateral6 {
	pub fn validate(&self) -> bool {
		if let Some(ref initl_mrgn_rcvd_pre_hrcut_value) = self.initl_mrgn_rcvd_pre_hrcut { if !initl_mrgn_rcvd_pre_hrcut_value.validate() { return false; } }
		if let Some(ref initl_mrgn_rcvd_pst_hrcut_value) = self.initl_mrgn_rcvd_pst_hrcut { if !initl_mrgn_rcvd_pst_hrcut_value.validate() { return false; } }
		if let Some(ref vartn_mrgn_rcvd_pre_hrcut_value) = self.vartn_mrgn_rcvd_pre_hrcut { if !vartn_mrgn_rcvd_pre_hrcut_value.validate() { return false; } }
		if let Some(ref vartn_mrgn_rcvd_pst_hrcut_value) = self.vartn_mrgn_rcvd_pst_hrcut { if !vartn_mrgn_rcvd_pst_hrcut_value.validate() { return false; } }
		if let Some(ref xcss_coll_rcvd_value) = self.xcss_coll_rcvd { if !xcss_coll_rcvd_value.validate() { return false; } }
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


// ReportingExemption1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportingExemption1 {
	#[serde(rename = "Rsn")]
	pub rsn: Max4Text,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max1000Text>,
}

impl ReportingExemption1 {
	pub fn validate(&self) -> bool {
		if !self.rsn.validate() { return false }
		if let Some(ref desc_value) = self.desc { if !desc_value.validate() { return false; } }
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


// TechnicalAttributes6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TechnicalAttributes6 {
	#[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
	pub tech_rcrd_id: Option<Max140Text>,
	#[serde(rename = "RptRctTmStmp", skip_serializing_if = "Option::is_none")]
	pub rpt_rct_tm_stmp: Option<String>,
}

impl TechnicalAttributes6 {
	pub fn validate(&self) -> bool {
		if let Some(ref tech_rcrd_id_value) = self.tech_rcrd_id { if !tech_rcrd_id_value.validate() { return false; } }
		return true
	}
}


// TradeCounterpartyRelationship1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeCounterpartyRelationship1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalPartyRelationshipType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max100Text>,
}

impl TradeCounterpartyRelationship1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// TradeCounterpartyRelationshipRecord1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeCounterpartyRelationshipRecord1 {
	#[serde(rename = "StartRltshPty")]
	pub start_rltsh_pty: TradeCounterpartyType1Code,
	#[serde(rename = "EndRltshPty")]
	pub end_rltsh_pty: TradeCounterpartyType1Code,
	#[serde(rename = "RltshTp")]
	pub rltsh_tp: TradeCounterpartyRelationship1Choice,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max1000Text>,
}

impl TradeCounterpartyRelationshipRecord1 {
	pub fn validate(&self) -> bool {
		if !self.start_rltsh_pty.validate() { return false }
		if !self.end_rltsh_pty.validate() { return false }
		if !self.rltsh_tp.validate() { return false }
		if let Some(ref desc_value) = self.desc { if !desc_value.validate() { return false; } }
		return true
	}
}


// TradeCounterpartyReport20 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeCounterpartyReport20 {
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: Counterparty45,
	#[serde(rename = "OthrCtrPty")]
	pub othr_ctr_pty: Counterparty46,
	#[serde(rename = "Brkr", skip_serializing_if = "Option::is_none")]
	pub brkr: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "SubmitgAgt", skip_serializing_if = "Option::is_none")]
	pub submitg_agt: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "ClrMmb", skip_serializing_if = "Option::is_none")]
	pub clr_mmb: Option<PartyIdentification248Choice>,
	#[serde(rename = "Bnfcry", skip_serializing_if = "Option::is_none")]
	pub bnfcry: Option<Vec<PartyIdentification248Choice>>,
	#[serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none")]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "ExctnAgt", skip_serializing_if = "Option::is_none")]
	pub exctn_agt: Option<Vec<OrganisationIdentification15Choice>>,
	#[serde(rename = "RltshRcrd", skip_serializing_if = "Option::is_none")]
	pub rltsh_rcrd: Option<Vec<TradeCounterpartyRelationshipRecord1>>,
}

impl TradeCounterpartyReport20 {
	pub fn validate(&self) -> bool {
		if !self.rptg_ctr_pty.validate() { return false }
		if !self.othr_ctr_pty.validate() { return false }
		if let Some(ref brkr_value) = self.brkr { if !brkr_value.validate() { return false; } }
		if let Some(ref submitg_agt_value) = self.submitg_agt { if !submitg_agt_value.validate() { return false; } }
		if let Some(ref clr_mmb_value) = self.clr_mmb { if !clr_mmb_value.validate() { return false; } }
		if let Some(ref bnfcry_vec) = self.bnfcry { for item in bnfcry_vec { if !item.validate() { return false; } } }
		if let Some(ref ntty_rspnsbl_for_rpt_value) = self.ntty_rspnsbl_for_rpt { if !ntty_rspnsbl_for_rpt_value.validate() { return false; } }
		if let Some(ref exctn_agt_vec) = self.exctn_agt { for item in exctn_agt_vec { if !item.validate() { return false; } } }
		if let Some(ref rltsh_rcrd_vec) = self.rltsh_rcrd { for item in rltsh_rcrd_vec { if !item.validate() { return false; } } }
		return true
	}
}


// TradeCounterpartyType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TradeCounterpartyType1Code {
	#[default]
	#[serde(rename = "BENE")]
	CodeBENE,
	#[serde(rename = "BROK")]
	CodeBROK,
	#[serde(rename = "CLEM")]
	CodeCLEM,
	#[serde(rename = "EXEA")]
	CodeEXEA,
	#[serde(rename = "OTHC")]
	CodeOTHC,
	#[serde(rename = "REPC")]
	CodeREPC,
	#[serde(rename = "SBMA")]
	CodeSBMA,
	#[serde(rename = "ERFR")]
	CodeERFR,
}

impl TradeCounterpartyType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// TradeData62Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeData62Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[serde(rename = "Stat", skip_serializing_if = "Option::is_none")]
	pub stat: Option<Vec<MarginReportData10>>,
}

impl TradeData62Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref data_set_actn_value) = self.data_set_actn { if !data_set_actn_value.validate() { return false; } }
		if let Some(ref stat_vec) = self.stat { for item in stat_vec { if !item.validate() { return false; } } }
		return true
	}
}


// TradeReportHeader4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeReportHeader4 {
	#[serde(rename = "RptExctnDt", skip_serializing_if = "Option::is_none")]
	pub rpt_exctn_dt: Option<String>,
	#[serde(rename = "MsgPgntn", skip_serializing_if = "Option::is_none")]
	pub msg_pgntn: Option<Pagination1>,
	#[serde(rename = "NbRcrds")]
	pub nb_rcrds: f64,
	#[serde(rename = "CmptntAuthrty", skip_serializing_if = "Option::is_none")]
	pub cmptnt_authrty: Option<Vec<Max100Text>>,
	#[serde(rename = "NewTradRpstryIdr", skip_serializing_if = "Option::is_none")]
	pub new_trad_rpstry_idr: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "RptgPurp", skip_serializing_if = "Option::is_none")]
	pub rptg_purp: Option<Vec<Max100Text>>,
}

impl TradeReportHeader4 {
	pub fn validate(&self) -> bool {
		if let Some(ref msg_pgntn_value) = self.msg_pgntn { if !msg_pgntn_value.validate() { return false; } }
		if let Some(ref cmptnt_authrty_vec) = self.cmptnt_authrty { for item in cmptnt_authrty_vec { if !item.validate() { return false; } } }
		if let Some(ref new_trad_rpstry_idr_value) = self.new_trad_rpstry_idr { if !new_trad_rpstry_idr_value.validate() { return false; } }
		if let Some(ref rptg_purp_vec) = self.rptg_purp { for item in rptg_purp_vec { if !item.validate() { return false; } } }
		return true
	}
}


// TradingCapacity7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TradingCapacity7Code {
	#[default]
	#[serde(rename = "AGEN")]
	CodeAGEN,
	#[serde(rename = "PRIN")]
	CodePRIN,
}

impl TradingCapacity7Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// TransactionOperationType11Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TransactionOperationType11Code {
	#[default]
	#[serde(rename = "CORR")]
	CodeCORR,
	#[serde(rename = "MARU")]
	CodeMARU,
	#[serde(rename = "NEWT")]
	CodeNEWT,
	#[serde(rename = "EROR")]
	CodeEROR,
}

impl TransactionOperationType11Code {
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


// UTIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UTIIdentifier {
	#[serde(rename = "$value")]
	pub uti_identifier: String,
}

impl UTIIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{18}[0-9]{2}[A-Z0-9]{0,32}").unwrap();
		if !pattern.is_match(&self.uti_identifier) {
			return false
		}
		return true
	}
}


// UniqueTransactionIdentifier2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UniqueTransactionIdentifier2Choice {
	#[serde(rename = "UnqTxIdr", skip_serializing_if = "Option::is_none")]
	pub unq_tx_idr: Option<UTIIdentifier>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification175>,
}

impl UniqueTransactionIdentifier2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref unq_tx_idr_value) = self.unq_tx_idr { if !unq_tx_idr_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct YesNoIndicator {
	#[serde(rename = "$value")]
	pub yes_no_indicator: bool,
}

impl YesNoIndicator {
	pub fn validate(&self) -> bool {
		return true
	}
}
