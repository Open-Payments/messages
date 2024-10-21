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

#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub mod iso20022 {
	use regex::Regex;
	use crate::common::*;
	#[cfg(feature = "derive_serde")]
	use serde::{Deserialize, Serialize};
	
	
	// AddressType2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AddressType2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ADDR") )]
		CodeADDR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PBOX") )]
		CodePBOX,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HOME") )]
		CodeHOME,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BIZZ") )]
		CodeBIZZ,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MLTO") )]
		CodeMLTO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DLVY") )]
		CodeDLVY,
	}
	
	impl AddressType2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AnyBICDec2014Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// BenchmarkCancellation1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BenchmarkCancellation1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none") )]
		pub tech_rcrd_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: SecurityIdentification19,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl BenchmarkCancellation1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tech_rcrd_id_value) = self.tech_rcrd_id { if let Err(e) = tech_rcrd_id_value.validate() { return Err(e); } }
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// BenchmarkCreate1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BenchmarkCreate1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none") )]
		pub tech_rcrd_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: SecurityIdentification19,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr") )]
		pub othr: BenchmarkDetail1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Admstr") )]
		pub admstr: PartyIdentification136,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NdrsngPty", skip_serializing_if = "Option::is_none") )]
		pub ndrsng_pty: Option<PartyIdentification136>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sts", skip_serializing_if = "Option::is_none") )]
		pub sts: Option<StatusDetail1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TechVldtyPrd", skip_serializing_if = "Option::is_none") )]
		pub tech_vldty_prd: Option<Period4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl BenchmarkCreate1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tech_rcrd_id_value) = self.tech_rcrd_id { if let Err(e) = tech_rcrd_id_value.validate() { return Err(e); } }
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Err(e) = self.othr.validate() { return Err(e); }
			if let Err(e) = self.admstr.validate() { return Err(e); }
			if let Some(ref ndrsng_pty_value) = self.ndrsng_pty { if let Err(e) = ndrsng_pty_value.validate() { return Err(e); } }
			if let Some(ref sts_value) = self.sts { if let Err(e) = sts_value.validate() { return Err(e); } }
			if let Some(ref tech_vldty_prd_value) = self.tech_vldty_prd { if let Err(e) = tech_vldty_prd_value.validate() { return Err(e); } }
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// BenchmarkCurveName2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum BenchmarkCurveName2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "WIBO") )]
		CodeWIBO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TREA") )]
		CodeTREA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TIBO") )]
		CodeTIBO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TLBO") )]
		CodeTLBO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SWAP") )]
		CodeSWAP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "STBO") )]
		CodeSTBO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRBO") )]
		CodePRBO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PFAN") )]
		CodePFAN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NIBO") )]
		CodeNIBO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MAAA") )]
		CodeMAAA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MOSP") )]
		CodeMOSP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LIBO") )]
		CodeLIBO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LIBI") )]
		CodeLIBI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "JIBA") )]
		CodeJIBA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ISDA") )]
		CodeISDA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GCFR") )]
		CodeGCFR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FUSW") )]
		CodeFUSW,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EUCH") )]
		CodeEUCH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EUUS") )]
		CodeEUUS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EURI") )]
		CodeEURI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EONS") )]
		CodeEONS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EONA") )]
		CodeEONA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CIBO") )]
		CodeCIBO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CDOR") )]
		CodeCDOR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BUBO") )]
		CodeBUBO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BBSW") )]
		CodeBBSW,
	}
	
	impl BenchmarkCurveName2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// BenchmarkDetail1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BenchmarkDetail1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FullNm") )]
		pub full_nm: Max350Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Indx", skip_serializing_if = "Option::is_none") )]
		pub indx: Option<BenchmarkCurveName2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cmnt", skip_serializing_if = "Option::is_none") )]
		pub cmnt: Option<Max20000Text>,
	}
	
	impl BenchmarkDetail1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.full_nm.validate() { return Err(e); }
			if let Some(ref indx_value) = self.indx { if let Err(e) = indx_value.validate() { return Err(e); } }
			if let Some(ref cmnt_value) = self.cmnt { if let Err(e) = cmnt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BenchmarkReport1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BenchmarkReport1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cret", skip_serializing_if = "Option::is_none") )]
		pub cret: Option<BenchmarkCreate1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Upd", skip_serializing_if = "Option::is_none") )]
		pub upd: Option<BenchmarkUpdate1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cxl", skip_serializing_if = "Option::is_none") )]
		pub cxl: Option<BenchmarkCancellation1>,
	}
	
	impl BenchmarkReport1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cret_value) = self.cret { if let Err(e) = cret_value.validate() { return Err(e); } }
			if let Some(ref upd_value) = self.upd { if let Err(e) = upd_value.validate() { return Err(e); } }
			if let Some(ref cxl_value) = self.cxl { if let Err(e) = cxl_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BenchmarkUpdate1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BenchmarkUpdate1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none") )]
		pub tech_rcrd_id: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: SecurityIdentification19,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsId", skip_serializing_if = "Option::is_none") )]
		pub prvs_id: Option<SecurityIdentification19>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr") )]
		pub othr: BenchmarkDetail1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Admstr") )]
		pub admstr: PartyIdentification136,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NdrsngPty", skip_serializing_if = "Option::is_none") )]
		pub ndrsng_pty: Option<PartyIdentification136>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sts", skip_serializing_if = "Option::is_none") )]
		pub sts: Option<StatusDetail1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TechVldtyPrd", skip_serializing_if = "Option::is_none") )]
		pub tech_vldty_prd: Option<Period4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl BenchmarkUpdate1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tech_rcrd_id_value) = self.tech_rcrd_id { if let Err(e) = tech_rcrd_id_value.validate() { return Err(e); } }
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref prvs_id_value) = self.prvs_id { if let Err(e) = prvs_id_value.validate() { return Err(e); } }
			if let Err(e) = self.othr.validate() { return Err(e); }
			if let Err(e) = self.admstr.validate() { return Err(e); }
			if let Some(ref ndrsng_pty_value) = self.ndrsng_pty { if let Err(e) = ndrsng_pty_value.validate() { return Err(e); } }
			if let Some(ref sts_value) = self.sts { if let Err(e) = sts_value.validate() { return Err(e); } }
			if let Some(ref tech_vldty_prd_value) = self.tech_vldty_prd { if let Err(e) = tech_vldty_prd_value.validate() { return Err(e); } }
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// CountryCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// ExternalAuthorityIdentification1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalAuthorityIdentification1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_authority_identification1_code: String,
	}
	
	impl ExternalAuthorityIdentification1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_authority_identification1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_authority_identification1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_authority_identification1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_authority_identification1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalFinancialInstrumentIdentificationType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalFinancialInstrumentIdentificationType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_financial_instrument_identification_type1_code: String,
	}
	
	impl ExternalFinancialInstrumentIdentificationType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_financial_instrument_identification_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_financial_instrument_identification_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_financial_instrument_identification_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_financial_instrument_identification_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// FinancialBenchmarkReportV01 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FinancialBenchmarkReportV01 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BchmkData") )]
		pub bchmk_data: Vec<BenchmarkReport1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl FinancialBenchmarkReportV01 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			for item in &self.bchmk_data { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// GenericIdentification36 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GenericIdentification36 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
		pub issr: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<Max35Text>,
	}
	
	impl GenericIdentification36 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Err(e) = self.issr.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ISINOct2015Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISINOct2015Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
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
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// IdentificationSource3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct IdentificationSource3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalFinancialInstrumentIdentificationType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl IdentificationSource3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// LEIIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// Max10Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max10Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max10_text: String,
	}
	
	impl Max10Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max10_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max10_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max10_text.chars().count() > 10 {
				return Err(ValidationError::new(1002, "max10_text exceeds the maximum length of 10".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max140Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// Max16Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max16Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max16_text: String,
	}
	
	impl Max16Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max16_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max16_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max16_text.chars().count() > 16 {
				return Err(ValidationError::new(1002, "max16_text exceeds the maximum length of 16".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max20000Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max20000Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max20000_text: String,
	}
	
	impl Max20000Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max20000_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max20000_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max20000_text.chars().count() > 20000 {
				return Err(ValidationError::new(1002, "max20000_text exceeds the maximum length of 20000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max350Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// Max70Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max70Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
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
	
	
	// NameAndAddress5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct NameAndAddress5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
		pub nm: Max350Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Adr", skip_serializing_if = "Option::is_none") )]
		pub adr: Option<PostalAddress1>,
	}
	
	impl NameAndAddress5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.nm.validate() { return Err(e); }
			if let Some(ref adr_value) = self.adr { if let Err(e) = adr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// OtherIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct OtherIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sfx", skip_serializing_if = "Option::is_none") )]
		pub sfx: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: IdentificationSource3Choice,
	}
	
	impl OtherIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref sfx_value) = self.sfx { if let Err(e) = sfx_value.validate() { return Err(e); } }
			if let Err(e) = self.tp.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// PartyIdentification120Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PartyIdentification120Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
		pub any_bic: Option<AnyBICDec2014Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
		pub prtry_id: Option<GenericIdentification36>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
		pub nm_and_adr: Option<NameAndAddress5>,
	}
	
	impl PartyIdentification120Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
			if let Some(ref prtry_id_value) = self.prtry_id { if let Err(e) = prtry_id_value.validate() { return Err(e); } }
			if let Some(ref nm_and_adr_value) = self.nm_and_adr { if let Err(e) = nm_and_adr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PartyIdentification136 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PartyIdentification136 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: PartyIdentification120Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<LEIIdentifier>,
	}
	
	impl PartyIdentification136 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Period2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Period2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt") )]
		pub fr_dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt") )]
		pub to_dt: String,
	}
	
	impl Period2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Period4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Period4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
		pub dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt", skip_serializing_if = "Option::is_none") )]
		pub fr_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt", skip_serializing_if = "Option::is_none") )]
		pub to_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrDtToDt", skip_serializing_if = "Option::is_none") )]
		pub fr_dt_to_dt: Option<Period2>,
	}
	
	impl Period4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref fr_dt_to_dt_value) = self.fr_dt_to_dt { if let Err(e) = fr_dt_to_dt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PostalAddress1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PostalAddress1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdrTp", skip_serializing_if = "Option::is_none") )]
		pub adr_tp: Option<AddressType2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdrLine", skip_serializing_if = "Option::is_none") )]
		pub adr_line: Option<Vec<Max70Text>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StrtNm", skip_serializing_if = "Option::is_none") )]
		pub strt_nm: Option<Max70Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNb", skip_serializing_if = "Option::is_none") )]
		pub bldg_nb: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstCd", skip_serializing_if = "Option::is_none") )]
		pub pst_cd: Option<Max16Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TwnNm", skip_serializing_if = "Option::is_none") )]
		pub twn_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none") )]
		pub ctry_sub_dvsn: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
		pub ctry: CountryCode,
	}
	
	impl PostalAddress1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref adr_tp_value) = self.adr_tp { if let Err(e) = adr_tp_value.validate() { return Err(e); } }
			if let Some(ref adr_line_vec) = self.adr_line { for item in adr_line_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref strt_nm_value) = self.strt_nm { if let Err(e) = strt_nm_value.validate() { return Err(e); } }
			if let Some(ref bldg_nb_value) = self.bldg_nb { if let Err(e) = bldg_nb_value.validate() { return Err(e); } }
			if let Some(ref pst_cd_value) = self.pst_cd { if let Err(e) = pst_cd_value.validate() { return Err(e); } }
			if let Some(ref twn_nm_value) = self.twn_nm { if let Err(e) = twn_nm_value.validate() { return Err(e); } }
			if let Some(ref ctry_sub_dvsn_value) = self.ctry_sub_dvsn { if let Err(e) = ctry_sub_dvsn_value.validate() { return Err(e); } }
			if let Err(e) = self.ctry.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// SecurityIdentification19 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SecurityIdentification19 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
		pub isin: Option<ISINOct2015Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrId", skip_serializing_if = "Option::is_none") )]
		pub othr_id: Option<Vec<OtherIdentification1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<Max140Text>,
	}
	
	impl SecurityIdentification19 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref isin_value) = self.isin { if let Err(e) = isin_value.validate() { return Err(e); } }
			if let Some(ref othr_id_vec) = self.othr_id { for item in othr_id_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// StatusDetail1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct StatusDetail1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
		pub ctry: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CmptntAuthrty") )]
		pub cmptnt_authrty: SupervisingAuthorityIdentification1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sts", skip_serializing_if = "Option::is_none") )]
		pub sts: Option<Max10Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StsRsn") )]
		pub sts_rsn: Max10Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ActvtyPrd", skip_serializing_if = "Option::is_none") )]
		pub actvty_prd: Option<Period4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cmnt", skip_serializing_if = "Option::is_none") )]
		pub cmnt: Option<Max20000Text>,
	}
	
	impl StatusDetail1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ctry_value) = self.ctry { if let Err(e) = ctry_value.validate() { return Err(e); } }
			if let Err(e) = self.cmptnt_authrty.validate() { return Err(e); }
			if let Some(ref sts_value) = self.sts { if let Err(e) = sts_value.validate() { return Err(e); } }
			if let Err(e) = self.sts_rsn.validate() { return Err(e); }
			if let Some(ref actvty_prd_value) = self.actvty_prd { if let Err(e) = actvty_prd_value.validate() { return Err(e); } }
			if let Some(ref cmnt_value) = self.cmnt { if let Err(e) = cmnt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SupervisingAuthorityIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SupervisingAuthorityIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<SupervisingAuthorityIdentification1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<LEIIdentifier>,
	}
	
	impl SupervisingAuthorityIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SupervisingAuthorityIdentification1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SupervisingAuthorityIdentification1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
		pub prtry_id: Option<ExternalAuthorityIdentification1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FullNm", skip_serializing_if = "Option::is_none") )]
		pub full_nm: Option<Max350Text>,
	}
	
	impl SupervisingAuthorityIdentification1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref prtry_id_value) = self.prtry_id { if let Err(e) = prtry_id_value.validate() { return Err(e); } }
			if let Some(ref full_nm_value) = self.full_nm { if let Err(e) = full_nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SupplementaryData1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	pub struct SupplementaryDataEnvelope1 {
	}
	
	impl SupplementaryDataEnvelope1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
}