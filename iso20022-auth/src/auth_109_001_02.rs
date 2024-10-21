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

pub mod iso20022 {
	use regex::Regex;
	use crate::common::*;
	#[cfg(feature = "derive_serde")]
	use serde::{Deserialize, Serialize};
	
	
	// ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_or_historic_currency_and20_decimal_amount_simple_type: f64,
	}
	
	impl ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.active_or_historic_currency_and20_decimal_amount_simple_type < 0.000000 {
				return Err(ValidationError::new(1003, "active_or_historic_currency_and20_decimal_amount_simple_type is less than the minimum value of 0.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
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
	
	
	// ActiveOrHistoricCurrencyCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveOrHistoricCurrencyCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_or_historic_currency_code: String,
	}
	
	impl ActiveOrHistoricCurrencyCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(&self.active_or_historic_currency_code) {
				return Err(ValidationError::new(1005, "active_or_historic_currency_code does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// AnyBICDec2014Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct AnyBICDec2014Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub any_bic_dec2014_identifier: String,
	}
	
	impl AnyBICDec2014Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(&self.any_bic_dec2014_identifier) {
				return Err(ValidationError::new(1005, "any_bic_dec2014_identifier does not match the required pattern".to_string()));
			}
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
			if let Some(ref prtfl_value) = self.prtfl { if let Err(e) = prtfl_value.validate() { return Err(e); } }
			if let Some(ref mrgn_prtfl_cd_value) = self.mrgn_prtfl_cd { if let Err(e) = mrgn_prtfl_cd_value.validate() { return Err(e); } }
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
	
	
	// ContractModification8 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ContractModification8 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ActnTp") )]
		pub actn_tp: TransactionOperationType11Code,
	}
	
	impl ContractModification8 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.actn_tp.validate() { return Err(e); }
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
		pub tradr_lctn: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BookgLctn", skip_serializing_if = "Option::is_none") )]
		pub bookg_lctn: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgXmptn", skip_serializing_if = "Option::is_none") )]
		pub rptg_xmptn: Option<ReportingExemption1>,
	}
	
	impl Counterparty45 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref ntr_value) = self.ntr { if let Err(e) = ntr_value.validate() { return Err(e); } }
			if let Some(ref tradg_cpcty_value) = self.tradg_cpcty { if let Err(e) = tradg_cpcty_value.validate() { return Err(e); } }
			if let Some(ref drctn_or_sd_value) = self.drctn_or_sd { if let Err(e) = drctn_or_sd_value.validate() { return Err(e); } }
			if let Some(ref tradr_lctn_value) = self.tradr_lctn { if let Err(e) = tradr_lctn_value.validate() { return Err(e); } }
			if let Some(ref bookg_lctn_value) = self.bookg_lctn { if let Err(e) = bookg_lctn_value.validate() { return Err(e); } }
			if let Some(ref rptg_xmptn_value) = self.rptg_xmptn { if let Err(e) = rptg_xmptn_value.validate() { return Err(e); } }
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
			if let Some(ref id_tp_value) = self.id_tp { if let Err(e) = id_tp_value.validate() { return Err(e); } }
			if let Some(ref ntr_value) = self.ntr { if let Err(e) = ntr_value.validate() { return Err(e); } }
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
			if let Some(ref fi_value) = self.fi { if let Err(e) = fi_value.validate() { return Err(e); } }
			if let Some(ref nfi_value) = self.nfi { if let Err(e) = nfi_value.validate() { return Err(e); } }
			if let Some(ref cntrl_cntr_pty_value) = self.cntrl_cntr_pty { if let Err(e) = cntrl_cntr_pty_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CountryCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct CountryCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
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
	
	
	// DerivativesTradeMarginDataTransactionStateReportV02 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DerivativesTradeMarginDataTransactionStateReportV02 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptHdr") )]
		pub rpt_hdr: TradeReportHeader4,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradData") )]
		pub trad_data: TradeData62Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl DerivativesTradeMarginDataTransactionStateReportV02 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rpt_hdr.validate() { return Err(e); }
			if let Err(e) = self.trad_data.validate() { return Err(e); }
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
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
			if let Some(ref drctn_of_the_scnd_leg_value) = self.drctn_of_the_scnd_leg { if let Err(e) = drctn_of_the_scnd_leg_value.validate() { return Err(e); } }
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
			if let Some(ref drctn_value) = self.drctn { if let Err(e) = drctn_value.validate() { return Err(e); } }
			if let Some(ref ctr_pty_sd_value) = self.ctr_pty_sd { if let Err(e) = ctr_pty_sd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ExternalPartyRelationshipType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalPartyRelationshipType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_party_relationship_type1_code: String,
	}
	
	impl ExternalPartyRelationshipType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_party_relationship_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_party_relationship_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_party_relationship_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_party_relationship_type1_code exceeds the maximum length of 4".to_string()));
			}
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
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
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
		pub id: Max72Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl GenericIdentification175 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ISODate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISODate {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iso_date: String,
	}
	
	impl ISODate {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ISODateTime ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISODateTime {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iso_date_time: String,
	}
	
	impl ISODateTime {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// LEIIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct LEIIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
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
		pub ctry: Option<CountryCode>,
	}
	
	impl LegalPersonIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref ctry_value) = self.ctry { if let Err(e) = ctry_value.validate() { return Err(e); } }
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
			if let Some(ref initl_mrgn_prtfl_cd_value) = self.initl_mrgn_prtfl_cd { if let Err(e) = initl_mrgn_prtfl_cd_value.validate() { return Err(e); } }
			if let Some(ref vartn_mrgn_prtfl_cd_value) = self.vartn_mrgn_prtfl_cd { if let Err(e) = vartn_mrgn_prtfl_cd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// MarginReportData10 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MarginReportData10 {
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
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctMod", skip_serializing_if = "Option::is_none") )]
		pub ctrct_mod: Option<ContractModification8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TechAttrbts", skip_serializing_if = "Option::is_none") )]
		pub tech_attrbts: Option<TechnicalAttributes6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl MarginReportData10 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.ctr_pty_id.validate() { return Err(e); }
			if let Some(ref tx_id_value) = self.tx_id { if let Err(e) = tx_id_value.validate() { return Err(e); } }
			if let Err(e) = self.coll.validate() { return Err(e); }
			if let Some(ref pstd_mrgn_or_coll_value) = self.pstd_mrgn_or_coll { if let Err(e) = pstd_mrgn_or_coll_value.validate() { return Err(e); } }
			if let Some(ref rcvd_mrgn_or_coll_value) = self.rcvd_mrgn_or_coll { if let Err(e) = rcvd_mrgn_or_coll_value.validate() { return Err(e); } }
			if let Some(ref ctrct_mod_value) = self.ctrct_mod { if let Err(e) = ctrct_mod_value.validate() { return Err(e); } }
			if let Some(ref tech_attrbts_value) = self.tech_attrbts { if let Err(e) = tech_attrbts_value.validate() { return Err(e); } }
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// Max1000Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max1000Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max1000_text: String,
	}
	
	impl Max1000Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max1000_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max1000_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max1000_text.chars().count() > 1000 {
				return Err(ValidationError::new(1002, "max1000_text exceeds the maximum length of 1000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max100Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max100Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max100_text: String,
	}
	
	impl Max100Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max100_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max100_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max100_text.chars().count() > 100 {
				return Err(ValidationError::new(1002, "max100_text exceeds the maximum length of 100".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max105Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max105Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
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
	
	
	// Max140Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max140Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max140_text: String,
	}
	
	impl Max140Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max140_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max140_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max140_text.chars().count() > 140 {
				return Err(ValidationError::new(1002, "max140_text exceeds the maximum length of 140".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max350Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max350Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
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
	
	
	// Max35Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max35Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max35_text: String,
	}
	
	impl Max35Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max35_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max35_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max35_text.chars().count() > 35 {
				return Err(ValidationError::new(1002, "max35_text exceeds the maximum length of 35".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max4Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max4Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max4_text: String,
	}
	
	impl Max4Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max4_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max4_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max4_text.chars().count() > 4 {
				return Err(ValidationError::new(1002, "max4_text exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max500Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max500Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max500_text: String,
	}
	
	impl Max500Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max500_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max500_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max500_text.chars().count() > 500 {
				return Err(ValidationError::new(1002, "max500_text exceeds the maximum length of 500".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max52Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max52Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max52_text: String,
	}
	
	impl Max52Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max52_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max52_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max52_text.chars().count() > 52 {
				return Err(ValidationError::new(1002, "max52_text exceeds the maximum length of 52".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max5NumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max5NumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max5_numeric_text: String,
	}
	
	impl Max5NumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[0-9]{1,5}").unwrap();
			if !pattern.is_match(&self.max5_numeric_text) {
				return Err(ValidationError::new(1005, "max5_numeric_text does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max72Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max72Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max72_text: String,
	}
	
	impl Max72Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max72_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max72_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max72_text.chars().count() > 72 {
				return Err(ValidationError::new(1002, "max72_text exceeds the maximum length of 72".to_string()));
			}
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
		pub nm: Option<Max105Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dmcl", skip_serializing_if = "Option::is_none") )]
		pub dmcl: Option<Max500Text>,
	}
	
	impl NaturalPersonIdentification2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref dmcl_value) = self.dmcl { if let Err(e) = dmcl_value.validate() { return Err(e); } }
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
		pub ctry: Option<CountryCode>,
	}
	
	impl NaturalPersonIdentification3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref ctry_value) = self.ctry { if let Err(e) = ctry_value.validate() { return Err(e); } }
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
	
	
	// Number ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Number {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub number: f64,
	}
	
	impl Number {
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
		pub lei: Option<LEIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<OrganisationIdentification38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
		pub any_bic: Option<AnyBICDec2014Identifier>,
	}
	
	impl OrganisationIdentification15Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
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
		pub nm: Option<Max105Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dmcl", skip_serializing_if = "Option::is_none") )]
		pub dmcl: Option<Max500Text>,
	}
	
	impl OrganisationIdentification38 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref dmcl_value) = self.dmcl { if let Err(e) = dmcl_value.validate() { return Err(e); } }
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
		pub pg_nb: Max5NumericText,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LastPgInd") )]
		pub last_pg_ind: bool,
	}
	
	impl Pagination1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.pg_nb.validate() { return Err(e); }
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
			if let Some(ref lgl_value) = self.lgl { if let Err(e) = lgl_value.validate() { return Err(e); } }
			if let Some(ref ntrl_value) = self.ntrl { if let Err(e) = ntrl_value.validate() { return Err(e); } }
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
		pub cd: Option<Max52Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoPrtfl", skip_serializing_if = "Option::is_none") )]
		pub no_prtfl: Option<NotApplicable1Code>,
	}
	
	impl PortfolioCode3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref no_prtfl_value) = self.no_prtfl { if let Err(e) = no_prtfl_value.validate() { return Err(e); } }
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
			if let Some(ref prtfl_value) = self.prtfl { if let Err(e) = prtfl_value.validate() { return Err(e); } }
			if let Some(ref no_prtfl_value) = self.no_prtfl { if let Err(e) = no_prtfl_value.validate() { return Err(e); } }
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
		pub cd: Max52Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtflTxXmptn", skip_serializing_if = "Option::is_none") )]
		pub prtfl_tx_xmptn: Option<bool>,
	}
	
	impl PortfolioIdentification3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
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
			if let Some(ref initl_mrgn_pstd_pre_hrcut_value) = self.initl_mrgn_pstd_pre_hrcut { if let Err(e) = initl_mrgn_pstd_pre_hrcut_value.validate() { return Err(e); } }
			if let Some(ref initl_mrgn_pstd_pst_hrcut_value) = self.initl_mrgn_pstd_pst_hrcut { if let Err(e) = initl_mrgn_pstd_pst_hrcut_value.validate() { return Err(e); } }
			if let Some(ref vartn_mrgn_pstd_pre_hrcut_value) = self.vartn_mrgn_pstd_pre_hrcut { if let Err(e) = vartn_mrgn_pstd_pre_hrcut_value.validate() { return Err(e); } }
			if let Some(ref vartn_mrgn_pstd_pst_hrcut_value) = self.vartn_mrgn_pstd_pst_hrcut { if let Err(e) = vartn_mrgn_pstd_pst_hrcut_value.validate() { return Err(e); } }
			if let Some(ref xcss_coll_pstd_value) = self.xcss_coll_pstd { if let Err(e) = xcss_coll_pstd_value.validate() { return Err(e); } }
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
			if let Some(ref initl_mrgn_rcvd_pre_hrcut_value) = self.initl_mrgn_rcvd_pre_hrcut { if let Err(e) = initl_mrgn_rcvd_pre_hrcut_value.validate() { return Err(e); } }
			if let Some(ref initl_mrgn_rcvd_pst_hrcut_value) = self.initl_mrgn_rcvd_pst_hrcut { if let Err(e) = initl_mrgn_rcvd_pst_hrcut_value.validate() { return Err(e); } }
			if let Some(ref vartn_mrgn_rcvd_pre_hrcut_value) = self.vartn_mrgn_rcvd_pre_hrcut { if let Err(e) = vartn_mrgn_rcvd_pre_hrcut_value.validate() { return Err(e); } }
			if let Some(ref vartn_mrgn_rcvd_pst_hrcut_value) = self.vartn_mrgn_rcvd_pst_hrcut { if let Err(e) = vartn_mrgn_rcvd_pst_hrcut_value.validate() { return Err(e); } }
			if let Some(ref xcss_coll_rcvd_value) = self.xcss_coll_rcvd { if let Err(e) = xcss_coll_rcvd_value.validate() { return Err(e); } }
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
		pub rsn: Max4Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<Max1000Text>,
	}
	
	impl ReportingExemption1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rsn.validate() { return Err(e); }
			if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
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
		pub plc_and_nm: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Envlp") )]
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
		pub tech_rcrd_id: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptRctTmStmp", skip_serializing_if = "Option::is_none") )]
		pub rpt_rct_tm_stmp: Option<String>,
	}
	
	impl TechnicalAttributes6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tech_rcrd_id_value) = self.tech_rcrd_id { if let Err(e) = tech_rcrd_id_value.validate() { return Err(e); } }
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
		pub cd: Option<ExternalPartyRelationshipType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max100Text>,
	}
	
	impl TradeCounterpartyRelationship1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
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
		pub desc: Option<Max1000Text>,
	}
	
	impl TradeCounterpartyRelationshipRecord1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.start_rltsh_pty.validate() { return Err(e); }
			if let Err(e) = self.end_rltsh_pty.validate() { return Err(e); }
			if let Err(e) = self.rltsh_tp.validate() { return Err(e); }
			if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
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
			if let Some(ref brkr_value) = self.brkr { if let Err(e) = brkr_value.validate() { return Err(e); } }
			if let Some(ref submitg_agt_value) = self.submitg_agt { if let Err(e) = submitg_agt_value.validate() { return Err(e); } }
			if let Some(ref clr_mmb_value) = self.clr_mmb { if let Err(e) = clr_mmb_value.validate() { return Err(e); } }
			if let Some(ref bnfcry_vec) = self.bnfcry { for item in bnfcry_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref ntty_rspnsbl_for_rpt_value) = self.ntty_rspnsbl_for_rpt { if let Err(e) = ntty_rspnsbl_for_rpt_value.validate() { return Err(e); } }
			if let Some(ref exctn_agt_vec) = self.exctn_agt { for item in exctn_agt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref rltsh_rcrd_vec) = self.rltsh_rcrd { for item in rltsh_rcrd_vec { if let Err(e) = item.validate() { return Err(e); } } }
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
	
	
	// TradeData62Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeData62Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
		pub data_set_actn: Option<ReportPeriodActivity1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Stat", skip_serializing_if = "Option::is_none") )]
		pub stat: Option<Vec<MarginReportData10>>,
	}
	
	impl TradeData62Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref data_set_actn_value) = self.data_set_actn { if let Err(e) = data_set_actn_value.validate() { return Err(e); } }
			if let Some(ref stat_vec) = self.stat { for item in stat_vec { if let Err(e) = item.validate() { return Err(e); } } }
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
		pub cmptnt_authrty: Option<Vec<Max100Text>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NewTradRpstryIdr", skip_serializing_if = "Option::is_none") )]
		pub new_trad_rpstry_idr: Option<OrganisationIdentification15Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgPurp", skip_serializing_if = "Option::is_none") )]
		pub rptg_purp: Option<Vec<Max100Text>>,
	}
	
	impl TradeReportHeader4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref msg_pgntn_value) = self.msg_pgntn { if let Err(e) = msg_pgntn_value.validate() { return Err(e); } }
			if let Some(ref cmptnt_authrty_vec) = self.cmptnt_authrty { for item in cmptnt_authrty_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref new_trad_rpstry_idr_value) = self.new_trad_rpstry_idr { if let Err(e) = new_trad_rpstry_idr_value.validate() { return Err(e); } }
			if let Some(ref rptg_purp_vec) = self.rptg_purp { for item in rptg_purp_vec { if let Err(e) = item.validate() { return Err(e); } } }
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
	
	
	// TransactionOperationType11Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum TransactionOperationType11Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CORR") )]
		CodeCORR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MARU") )]
		CodeMARU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NEWT") )]
		CodeNEWT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EROR") )]
		CodeEROR,
	}
	
	impl TransactionOperationType11Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// TrueFalseIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct TrueFalseIndicator {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub true_false_indicator: bool,
	}
	
	impl TrueFalseIndicator {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// UTIIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct UTIIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub uti_identifier: String,
	}
	
	impl UTIIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9]{18}[0-9]{2}[A-Z0-9]{0,32}").unwrap();
			if !pattern.is_match(&self.uti_identifier) {
				return Err(ValidationError::new(1005, "uti_identifier does not match the required pattern".to_string()));
			}
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
		pub unq_tx_idr: Option<UTIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification175>,
	}
	
	impl UniqueTransactionIdentifier2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref unq_tx_idr_value) = self.unq_tx_idr { if let Err(e) = unq_tx_idr_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// YesNoIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct YesNoIndicator {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub yes_no_indicator: bool,
	}
	
	impl YesNoIndicator {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
}