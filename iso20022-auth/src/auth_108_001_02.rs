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


// ActiveOrHistoricCurrencyAnd20DecimalAmount ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ActiveOrHistoricCurrencyAnd20DecimalAmount {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
	pub value: f64,
}

impl ActiveOrHistoricCurrencyAnd20DecimalAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CollateralPortfolioCode6Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CollateralPortfolioCode6Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtfl", skip_serializing_if = "Option::is_none") )]
	pub prtfl: Option<PortfolioCode3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MrgnPrtflCd", skip_serializing_if = "Option::is_none") )]
	pub mrgn_prtfl_cd: Option<MarginPortfolio4>,
}

impl CollateralPortfolioCode6Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.prtfl { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.mrgn_prtfl_cd { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// CollateralisationType3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum CollateralisationType3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FLCL") )]
	CodeFLCL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OWCL") )]
	CodeOWCL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OWC1") )]
	CodeOWC1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OWC2") )]
	CodeOWC2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OWP1") )]
	CodeOWP1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OWP2") )]
	CodeOWP2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRCL") )]
	CodePRCL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRC1") )]
	CodePRC1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRC2") )]
	CodePRC2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UNCL") )]
	CodeUNCL,
}

impl CollateralisationType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Counterparty45 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Counterparty45 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: PartyIdentification248Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ntr", skip_serializing_if = "Option::is_none") )]
	pub ntr: Option<CounterpartyTradeNature15Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgCpcty", skip_serializing_if = "Option::is_none") )]
	pub tradg_cpcty: Option<TradingCapacity7Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DrctnOrSd", skip_serializing_if = "Option::is_none") )]
	pub drctn_or_sd: Option<Direction4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradrLctn", skip_serializing_if = "Option::is_none") )]
	pub tradr_lctn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BookgLctn", skip_serializing_if = "Option::is_none") )]
	pub bookg_lctn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgXmptn", skip_serializing_if = "Option::is_none") )]
	pub rptg_xmptn: Option<ReportingExemption1>,
}

impl Counterparty45 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref val) = self.ntr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.tradg_cpcty { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.drctn_or_sd { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.tradr_lctn {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "tradr_lctn does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.bookg_lctn {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "bookg_lctn does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.rptg_xmptn { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// Counterparty46 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Counterparty46 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "IdTp", skip_serializing_if = "Option::is_none") )]
	pub id_tp: Option<PartyIdentification248Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ntr", skip_serializing_if = "Option::is_none") )]
	pub ntr: Option<CounterpartyTradeNature15Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgOblgtn", skip_serializing_if = "Option::is_none") )]
	pub rptg_oblgtn: Option<bool>,
}

impl Counterparty46 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id_tp { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.ntr { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// CounterpartyTradeNature15Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CounterpartyTradeNature15Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FI", skip_serializing_if = "Option::is_none") )]
	pub fi: Option<FinancialInstitutionSector1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NFI", skip_serializing_if = "Option::is_none") )]
	pub nfi: Option<NonFinancialInstitutionSector10>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CntrlCntrPty", skip_serializing_if = "Option::is_none") )]
	pub cntrl_cntr_pty: Option<NoReasonCode>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<NoReasonCode>,
}

impl CounterpartyTradeNature15Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.fi { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.nfi { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.cntrl_cntr_pty { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.othr { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// DerivativesTradeMarginDataReportV02 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DerivativesTradeMarginDataReportV02 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptHdr") )]
	pub rpt_hdr: TradeReportHeader4,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradData") )]
	pub trad_data: TradeData61Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl DerivativesTradeMarginDataReportV02 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.rpt_hdr.validate() { return Err(e); }
		if let Err(e) = self.trad_data.validate() { return Err(e); }
		if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// Direction2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Direction2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DrctnOfTheFrstLeg") )]
	pub drctn_of_the_frst_leg: OptionParty3Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DrctnOfTheScndLeg", skip_serializing_if = "Option::is_none") )]
	pub drctn_of_the_scnd_leg: Option<OptionParty3Code>,
}

impl Direction2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.drctn_of_the_frst_leg.validate() { return Err(e); }
		if let Some(ref val) = self.drctn_of_the_scnd_leg { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// Direction4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Direction4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Drctn", skip_serializing_if = "Option::is_none") )]
	pub drctn: Option<Direction2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtySd", skip_serializing_if = "Option::is_none") )]
	pub ctr_pty_sd: Option<OptionParty1Code>,
}

impl Direction4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.drctn { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.ctr_pty_sd { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// FinancialInstitutionSector1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstitutionSector1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sctr") )]
	pub sctr: Vec<FinancialPartyClassification2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrThrshld", skip_serializing_if = "Option::is_none") )]
	pub clr_thrshld: Option<bool>,
}

impl FinancialInstitutionSector1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.sctr { if let Err(e) = item.validate() { return Err(e); } }
		Ok(())
	}
}


// FinancialPartyClassification2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialPartyClassification2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<FinancialPartySectorType3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification175>,
}

impl FinancialPartyClassification2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// FinancialPartySectorType3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum FinancialPartySectorType3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "AIFD") )]
	CodeAIFD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CSDS") )]
	CodeCSDS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CCPS") )]
	CodeCCPS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CDTI") )]
	CodeCDTI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INUN") )]
	CodeINUN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ORPI") )]
	CodeORPI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INVF") )]
	CodeINVF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REIN") )]
	CodeREIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UCIT") )]
	CodeUCIT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ASSU") )]
	CodeASSU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
}

impl FinancialPartySectorType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// GenericIdentification175 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericIdentification175 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericIdentification175 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 72 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 72".to_string()));
		}
		if let Some(ref val) = self.schme_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "schme_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "schme_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// LegalPersonIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct LegalPersonIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: OrganisationIdentification15Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
	pub ctry: Option<String>,
}

impl LegalPersonIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// MarginCollateralReport5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MarginCollateralReport5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollPrtflCd") )]
	pub coll_prtfl_cd: CollateralPortfolioCode6Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollstnCtgy") )]
	pub collstn_ctgy: CollateralisationType3Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TmStmp", skip_serializing_if = "Option::is_none") )]
	pub tm_stmp: Option<String>,
}

impl MarginCollateralReport5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.coll_prtfl_cd.validate() { return Err(e); }
		if let Err(e) = self.collstn_ctgy.validate() { return Err(e); }
		Ok(())
	}
}


// MarginPortfolio4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MarginPortfolio4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitlMrgnPrtflCd", skip_serializing_if = "Option::is_none") )]
	pub initl_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VartnMrgnPrtflCd", skip_serializing_if = "Option::is_none") )]
	pub vartn_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
}

impl MarginPortfolio4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.initl_mrgn_prtfl_cd { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.vartn_mrgn_prtfl_cd { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// MarginReportData9 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MarginReportData9 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgTmStmp", skip_serializing_if = "Option::is_none") )]
	pub rptg_tm_stmp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtyId") )]
	pub ctr_pty_id: TradeCounterpartyReport20,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EvtDt", skip_serializing_if = "Option::is_none") )]
	pub evt_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxId", skip_serializing_if = "Option::is_none") )]
	pub tx_id: Option<UniqueTransactionIdentifier2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Coll") )]
	pub coll: MarginCollateralReport5,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstdMrgnOrColl", skip_serializing_if = "Option::is_none") )]
	pub pstd_mrgn_or_coll: Option<PostedMarginOrCollateral6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RcvdMrgnOrColl", skip_serializing_if = "Option::is_none") )]
	pub rcvd_mrgn_or_coll: Option<ReceivedMarginOrCollateral6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtyRatgTrggrInd", skip_serializing_if = "Option::is_none") )]
	pub ctr_pty_ratg_trggr_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtyRatgThrshldInd", skip_serializing_if = "Option::is_none") )]
	pub ctr_pty_ratg_thrshld_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechAttrbts", skip_serializing_if = "Option::is_none") )]
	pub tech_attrbts: Option<TechnicalAttributes6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl MarginReportData9 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.ctr_pty_id.validate() { return Err(e); }
		if let Some(ref val) = self.tx_id { if let Err(e) = val.validate() { return Err(e); } }
		if let Err(e) = self.coll.validate() { return Err(e); }
		if let Some(ref val) = self.pstd_mrgn_or_coll { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.rcvd_mrgn_or_coll { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.tech_attrbts { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// NaturalPersonIdentification2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NaturalPersonIdentification2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: GenericIdentification175,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dmcl", skip_serializing_if = "Option::is_none") )]
	pub dmcl: Option<String>,
}

impl NaturalPersonIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 105 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 105".to_string()));
			}
		}
		if let Some(ref val) = self.dmcl {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dmcl is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 500 {
				return Err(ValidationError::new(1002, "dmcl exceeds the maximum length of 500".to_string()));
			}
		}
		Ok(())
	}
}


// NaturalPersonIdentification3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NaturalPersonIdentification3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: NaturalPersonIdentification2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
	pub ctry: Option<String>,
}

impl NaturalPersonIdentification3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// NoReasonCode ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum NoReasonCode {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NORE") )]
	CodeNORE,
}

impl NoReasonCode {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// NonFinancialInstitutionSector10 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NonFinancialInstitutionSector10 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sctr") )]
	pub sctr: Vec<GenericIdentification175>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrThrshld", skip_serializing_if = "Option::is_none") )]
	pub clr_thrshld: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DrctlyLkdActvty", skip_serializing_if = "Option::is_none") )]
	pub drctly_lkd_actvty: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FdrlInstn", skip_serializing_if = "Option::is_none") )]
	pub fdrl_instn: Option<bool>,
}

impl NonFinancialInstitutionSector10 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.sctr { if let Err(e) = item.validate() { return Err(e); } }
		Ok(())
	}
}


// NotApplicable1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum NotApplicable1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NOAP") )]
	CodeNOAP,
}

impl NotApplicable1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OptionParty1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum OptionParty1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SLLR") )]
	CodeSLLR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BYER") )]
	CodeBYER,
}

impl OptionParty1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OptionParty3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum OptionParty3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MAKE") )]
	CodeMAKE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TAKE") )]
	CodeTAKE,
}

impl OptionParty3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OrganisationIdentification15Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OrganisationIdentification15Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<OrganisationIdentification38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
	pub any_bic: Option<String>,
}

impl OrganisationIdentification15Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.othr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.any_bic {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// OrganisationIdentification38 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OrganisationIdentification38 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: GenericIdentification175,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dmcl", skip_serializing_if = "Option::is_none") )]
	pub dmcl: Option<String>,
}

impl OrganisationIdentification38 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 105 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 105".to_string()));
			}
		}
		if let Some(ref val) = self.dmcl {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dmcl is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 500 {
				return Err(ValidationError::new(1002, "dmcl exceeds the maximum length of 500".to_string()));
			}
		}
		Ok(())
	}
}


// Pagination1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Pagination1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PgNb") )]
	pub pg_nb: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LastPgInd") )]
	pub last_pg_ind: bool,
}

impl Pagination1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{1,5}").unwrap();
		if !pattern.is_match(&self.pg_nb) {
			return Err(ValidationError::new(1005, "pg_nb does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// PartyIdentification248Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyIdentification248Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Lgl", skip_serializing_if = "Option::is_none") )]
	pub lgl: Option<LegalPersonIdentification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ntrl", skip_serializing_if = "Option::is_none") )]
	pub ntrl: Option<NaturalPersonIdentification3>,
}

impl PartyIdentification248Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.lgl { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.ntrl { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// PortfolioCode3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PortfolioCode3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NoPrtfl", skip_serializing_if = "Option::is_none") )]
	pub no_prtfl: Option<NotApplicable1Code>,
}

impl PortfolioCode3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 52 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 52".to_string()));
			}
		}
		if let Some(ref val) = self.no_prtfl { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// PortfolioCode5Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PortfolioCode5Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtfl", skip_serializing_if = "Option::is_none") )]
	pub prtfl: Option<PortfolioIdentification3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NoPrtfl", skip_serializing_if = "Option::is_none") )]
	pub no_prtfl: Option<NotApplicable1Code>,
}

impl PortfolioCode5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.prtfl { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.no_prtfl { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// PortfolioIdentification3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PortfolioIdentification3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
	pub cd: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtflTxXmptn", skip_serializing_if = "Option::is_none") )]
	pub prtfl_tx_xmptn: Option<bool>,
}

impl PortfolioIdentification3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.cd.chars().count() < 1 {
			return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
		}
		if self.cd.chars().count() > 52 {
			return Err(ValidationError::new(1002, "cd exceeds the maximum length of 52".to_string()));
		}
		Ok(())
	}
}


// PostedMarginOrCollateral6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PostedMarginOrCollateral6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitlMrgnPstdPreHrcut", skip_serializing_if = "Option::is_none") )]
	pub initl_mrgn_pstd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitlMrgnPstdPstHrcut", skip_serializing_if = "Option::is_none") )]
	pub initl_mrgn_pstd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VartnMrgnPstdPreHrcut", skip_serializing_if = "Option::is_none") )]
	pub vartn_mrgn_pstd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VartnMrgnPstdPstHrcut", skip_serializing_if = "Option::is_none") )]
	pub vartn_mrgn_pstd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XcssCollPstd", skip_serializing_if = "Option::is_none") )]
	pub xcss_coll_pstd: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
}

impl PostedMarginOrCollateral6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.initl_mrgn_pstd_pre_hrcut { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.initl_mrgn_pstd_pst_hrcut { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.vartn_mrgn_pstd_pre_hrcut { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.vartn_mrgn_pstd_pst_hrcut { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.xcss_coll_pstd { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// ReceivedMarginOrCollateral6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ReceivedMarginOrCollateral6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitlMrgnRcvdPreHrcut", skip_serializing_if = "Option::is_none") )]
	pub initl_mrgn_rcvd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitlMrgnRcvdPstHrcut", skip_serializing_if = "Option::is_none") )]
	pub initl_mrgn_rcvd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VartnMrgnRcvdPreHrcut", skip_serializing_if = "Option::is_none") )]
	pub vartn_mrgn_rcvd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VartnMrgnRcvdPstHrcut", skip_serializing_if = "Option::is_none") )]
	pub vartn_mrgn_rcvd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XcssCollRcvd", skip_serializing_if = "Option::is_none") )]
	pub xcss_coll_rcvd: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
}

impl ReceivedMarginOrCollateral6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.initl_mrgn_rcvd_pre_hrcut { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.initl_mrgn_rcvd_pst_hrcut { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.vartn_mrgn_rcvd_pre_hrcut { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.vartn_mrgn_rcvd_pst_hrcut { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.xcss_coll_rcvd { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// ReportPeriodActivity1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ReportPeriodActivity1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "NOTX") )]
	CodeNOTX,
}

impl ReportPeriodActivity1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ReportingExemption1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ReportingExemption1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn") )]
	pub rsn: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
}

impl ReportingExemption1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.rsn.chars().count() < 1 {
			return Err(ValidationError::new(1001, "rsn is shorter than the minimum length of 1".to_string()));
		}
		if self.rsn.chars().count() > 4 {
			return Err(ValidationError::new(1002, "rsn exceeds the maximum length of 4".to_string()));
		}
		if let Some(ref val) = self.desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 1000 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 1000".to_string()));
			}
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


// TechnicalAttributes6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TechnicalAttributes6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none") )]
	pub tech_rcrd_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptRctTmStmp", skip_serializing_if = "Option::is_none") )]
	pub rpt_rct_tm_stmp: Option<String>,
}

impl TechnicalAttributes6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tech_rcrd_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tech_rcrd_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "tech_rcrd_id exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// TradeCounterpartyRelationship1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradeCounterpartyRelationship1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl TradeCounterpartyRelationship1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 100 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 100".to_string()));
			}
		}
		Ok(())
	}
}


// TradeCounterpartyRelationshipRecord1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradeCounterpartyRelationshipRecord1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "StartRltshPty") )]
	pub start_rltsh_pty: TradeCounterpartyType1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndRltshPty") )]
	pub end_rltsh_pty: TradeCounterpartyType1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltshTp") )]
	pub rltsh_tp: TradeCounterpartyRelationship1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
}

impl TradeCounterpartyRelationshipRecord1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.start_rltsh_pty.validate() { return Err(e); }
		if let Err(e) = self.end_rltsh_pty.validate() { return Err(e); }
		if let Err(e) = self.rltsh_tp.validate() { return Err(e); }
		if let Some(ref val) = self.desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 1000 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 1000".to_string()));
			}
		}
		Ok(())
	}
}


// TradeCounterpartyReport20 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradeCounterpartyReport20 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgCtrPty") )]
	pub rptg_ctr_pty: Counterparty45,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrCtrPty") )]
	pub othr_ctr_pty: Counterparty46,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Brkr", skip_serializing_if = "Option::is_none") )]
	pub brkr: Option<OrganisationIdentification15Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubmitgAgt", skip_serializing_if = "Option::is_none") )]
	pub submitg_agt: Option<OrganisationIdentification15Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrMmb", skip_serializing_if = "Option::is_none") )]
	pub clr_mmb: Option<PartyIdentification248Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Bnfcry", skip_serializing_if = "Option::is_none") )]
	pub bnfcry: Option<Vec<PartyIdentification248Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none") )]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExctnAgt", skip_serializing_if = "Option::is_none") )]
	pub exctn_agt: Option<Vec<OrganisationIdentification15Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltshRcrd", skip_serializing_if = "Option::is_none") )]
	pub rltsh_rcrd: Option<Vec<TradeCounterpartyRelationshipRecord1>>,
}

impl TradeCounterpartyReport20 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.rptg_ctr_pty.validate() { return Err(e); }
		if let Err(e) = self.othr_ctr_pty.validate() { return Err(e); }
		if let Some(ref val) = self.brkr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.submitg_agt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.clr_mmb { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.bnfcry { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref val) = self.ntty_rspnsbl_for_rpt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.exctn_agt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref vec) = self.rltsh_rcrd { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// TradeCounterpartyType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TradeCounterpartyType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "BENE") )]
	CodeBENE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BROK") )]
	CodeBROK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLEM") )]
	CodeCLEM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EXEA") )]
	CodeEXEA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHC") )]
	CodeOTHC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REPC") )]
	CodeREPC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SBMA") )]
	CodeSBMA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ERFR") )]
	CodeERFR,
}

impl TradeCounterpartyType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TradeData61Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradeData61Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rpt", skip_serializing_if = "Option::is_none") )]
	pub rpt: Option<Vec<TradeReport34Choice>>,
}

impl TradeData61Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.data_set_actn { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.rpt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// TradeReport34Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradeReport34Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "New", skip_serializing_if = "Option::is_none") )]
	pub new: Option<MarginReportData9>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MrgnUpd", skip_serializing_if = "Option::is_none") )]
	pub mrgn_upd: Option<MarginReportData9>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Err", skip_serializing_if = "Option::is_none") )]
	pub err: Option<MarginReportData9>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Crrctn", skip_serializing_if = "Option::is_none") )]
	pub crrctn: Option<MarginReportData9>,
}

impl TradeReport34Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.new { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.mrgn_upd { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.err { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.crrctn { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// TradeReportHeader4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradeReportHeader4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptExctnDt", skip_serializing_if = "Option::is_none") )]
	pub rpt_exctn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgPgntn", skip_serializing_if = "Option::is_none") )]
	pub msg_pgntn: Option<Pagination1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbRcrds") )]
	pub nb_rcrds: f64,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CmptntAuthrty", skip_serializing_if = "Option::is_none") )]
	pub cmptnt_authrty: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NewTradRpstryIdr", skip_serializing_if = "Option::is_none") )]
	pub new_trad_rpstry_idr: Option<OrganisationIdentification15Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgPurp", skip_serializing_if = "Option::is_none") )]
	pub rptg_purp: Option<Vec<String>>,
}

impl TradeReportHeader4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.msg_pgntn { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.cmptnt_authrty {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "cmptnt_authrty is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 100 {
					return Err(ValidationError::new(1002, "cmptnt_authrty exceeds the maximum length of 100".to_string()));
				}
			}
		}
		if let Some(ref val) = self.new_trad_rpstry_idr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.rptg_purp {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "rptg_purp is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 100 {
					return Err(ValidationError::new(1002, "rptg_purp exceeds the maximum length of 100".to_string()));
				}
			}
		}
		Ok(())
	}
}


// TradingCapacity7Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TradingCapacity7Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "AGEN") )]
	CodeAGEN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRIN") )]
	CodePRIN,
}

impl TradingCapacity7Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// UniqueTransactionIdentifier2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UniqueTransactionIdentifier2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnqTxIdr", skip_serializing_if = "Option::is_none") )]
	pub unq_tx_idr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification175>,
}

impl UniqueTransactionIdentifier2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.unq_tx_idr {
			let pattern = Regex::new("[A-Z0-9]{18}[0-9]{2}[A-Z0-9]{0,32}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "unq_tx_idr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prtry { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}
