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
	
	
	// AnyMIC1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AnyMIC1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ANYM") )]
		CodeANYM,
	}
	
	impl AnyMIC1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// BasketQuery1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BasketQuery1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Strr", skip_serializing_if = "Option::is_none") )]
		pub strr: Option<LEIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Idr", skip_serializing_if = "Option::is_none") )]
		pub idr: Option<Max52Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
		pub isin: Option<ISINOct2015Identifier>,
	}
	
	impl BasketQuery1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref strr_value) = self.strr { if let Err(e) = strr_value.validate() { return Err(e); } }
			if let Some(ref idr_value) = self.idr { if let Err(e) = idr_value.validate() { return Err(e); } }
			if let Some(ref isin_value) = self.isin { if let Err(e) = isin_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CFIOct2015Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct CFIOct2015Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub cfi_oct2015_identifier: String,
	}
	
	impl CFIOct2015Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{6,6}").unwrap();
			if !pattern.is_match(&self.cfi_oct2015_identifier) {
				return Err(ValidationError::new(1005, "cfi_oct2015_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// CorporateSectorCriteria6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CorporateSectorCriteria6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FISctr", skip_serializing_if = "Option::is_none") )]
		pub fi_sctr: Option<Vec<FinancialPartySectorType2Code>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NFISctr", skip_serializing_if = "Option::is_none") )]
		pub nfi_sctr: Option<Vec<NonFinancialPartySector1Code>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NotRptd", skip_serializing_if = "Option::is_none") )]
		pub not_rptd: Option<NotReported1Code>,
	}
	
	impl CorporateSectorCriteria6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref fi_sctr_vec) = self.fi_sctr { for item in fi_sctr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref nfi_sctr_vec) = self.nfi_sctr { for item in nfi_sctr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref not_rptd_value) = self.not_rptd { if let Err(e) = not_rptd_value.validate() { return Err(e); } }
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
	
	
	// DateOrBlankQuery2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct DateOrBlankQuery2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rg", skip_serializing_if = "Option::is_none") )]
		pub rg: Option<DatePeriod1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NotRptd", skip_serializing_if = "Option::is_none") )]
		pub not_rptd: Option<NotReported1Code>,
	}
	
	impl DateOrBlankQuery2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref rg_value) = self.rg { if let Err(e) = rg_value.validate() { return Err(e); } }
			if let Some(ref not_rptd_value) = self.not_rptd { if let Err(e) = not_rptd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DatePeriod1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct DatePeriod1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt", skip_serializing_if = "Option::is_none") )]
		pub fr_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt") )]
		pub to_dt: String,
	}
	
	impl DatePeriod1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// DateTimeOrBlankQuery1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct DateTimeOrBlankQuery1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rg", skip_serializing_if = "Option::is_none") )]
		pub rg: Option<DateTimePeriod1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NotRptd", skip_serializing_if = "Option::is_none") )]
		pub not_rptd: Option<NotReported1Code>,
	}
	
	impl DateTimeOrBlankQuery1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref rg_value) = self.rg { if let Err(e) = rg_value.validate() { return Err(e); } }
			if let Some(ref not_rptd_value) = self.not_rptd { if let Err(e) = not_rptd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DateTimePeriod1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct DateTimePeriod1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrDtTm") )]
		pub fr_dt_tm: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ToDtTm") )]
		pub to_dt_tm: String,
	}
	
	impl DateTimePeriod1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// DayOfMonthNumber ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct DayOfMonthNumber {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub day_of_month_number: f64,
	}
	
	impl DayOfMonthNumber {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.day_of_month_number < 1.000000 {
				return Err(ValidationError::new(1003, "day_of_month_number is less than the minimum value of 1.000000".to_string()));
			}
			if self.day_of_month_number > 31.000000 {
				return Err(ValidationError::new(1004, "day_of_month_number exceeds the maximum value of 31.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// DerivativeEventType3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum DerivativeEventType3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ALOC") )]
		CodeALOC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLRG") )]
		CodeCLRG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLAL") )]
		CodeCLAL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "COMP") )]
		CodeCOMP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CORP") )]
		CodeCORP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CREV") )]
		CodeCREV,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ETRM") )]
		CodeETRM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EXER") )]
		CodeEXER,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INCP") )]
		CodeINCP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NOVA") )]
		CodeNOVA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PTNG") )]
		CodePTNG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TRAD") )]
		CodeTRAD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UPDT") )]
		CodeUPDT,
	}
	
	impl DerivativeEventType3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// DerivativesTradeReportQueryV05 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct DerivativesTradeReportQueryV05 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RqstngAuthrty") )]
		pub rqstng_authrty: PartyIdentification121Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradQryData") )]
		pub trad_qry_data: TradeReportQuery18Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl DerivativesTradeReportQueryV05 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rqstng_authrty.validate() { return Err(e); }
			if let Err(e) = self.trad_qry_data.validate() { return Err(e); }
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// FinancialInstrumentContractType2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum FinancialInstrumentContractType2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CFDS") )]
		CodeCFDS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FRAS") )]
		CodeFRAS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FUTR") )]
		CodeFUTR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FORW") )]
		CodeFORW,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OPTN") )]
		CodeOPTN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SPDB") )]
		CodeSPDB,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SWAP") )]
		CodeSWAP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SWPT") )]
		CodeSWPT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
	}
	
	impl FinancialInstrumentContractType2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// FinancialPartySectorType2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum FinancialPartySectorType2Code {
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
	}
	
	impl FinancialPartySectorType2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Frequency14Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum Frequency14Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "DAIL") )]
		CodeDAIL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WEEK") )]
		CodeWEEK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
		CodeMNTH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ADHO") )]
		CodeADHO,
	}
	
	impl Frequency14Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// GenericIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GenericIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl GenericIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GenericIdentification175 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// ISINQueryCriteria1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ISINQueryCriteria1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Idr", skip_serializing_if = "Option::is_none") )]
		pub idr: Option<Vec<ISINOct2015Identifier>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NotRptd", skip_serializing_if = "Option::is_none") )]
		pub not_rptd: Option<NotReported1Code>,
	}
	
	impl ISINQueryCriteria1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref idr_vec) = self.idr { for item in idr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref not_rptd_value) = self.not_rptd { if let Err(e) = not_rptd_value.validate() { return Err(e); } }
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
	
	
	// ISODateTime ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// MICIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct MICIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub mic_identifier: String,
	}
	
	impl MICIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(&self.mic_identifier) {
				return Err(ValidationError::new(1005, "mic_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max1000Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// Max105Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// Max25Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max25Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max25_text: String,
	}
	
	impl Max25Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max25_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max25_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max25_text.chars().count() > 25 {
				return Err(ValidationError::new(1002, "max25_text exceeds the maximum length of 25".to_string()));
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
	
	
	// Max500Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// Max72Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// ModificationLevel1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum ModificationLevel1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "PSTN") )]
		CodePSTN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TCTN") )]
		CodeTCTN,
	}
	
	impl ModificationLevel1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	
	
	// NaturalPersonIdentification2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// NonFinancialPartySector1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum NonFinancialPartySector1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "WTER") )]
		CodeWTER,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MING") )]
		CodeMING,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MAFG") )]
		CodeMAFG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SPLY") )]
		CodeSPLY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CSTR") )]
		CodeCSTR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AGRI") )]
		CodeAGRI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ACAF") )]
		CodeACAF,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EDUC") )]
		CodeEDUC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AEAR") )]
		CodeAEAR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FINA") )]
		CodeFINA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HHSW") )]
		CodeHHSW,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INCO") )]
		CodeINCO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WRRM") )]
		CodeWRRM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTSA") )]
		CodeOTSA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PSTA") )]
		CodePSTA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PADS") )]
		CodePADS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RESA") )]
		CodeRESA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TRAS") )]
		CodeTRAS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ASSA") )]
		CodeASSA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AHAE") )]
		CodeAHAE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AEOB") )]
		CodeAEOB,
	}
	
	impl NonFinancialPartySector1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// NotAvailable1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum NotAvailable1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "NTAV") )]
		CodeNTAV,
	}
	
	impl NotAvailable1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// NotReported1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum NotReported1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "NORP") )]
		CodeNORP,
	}
	
	impl NotReported1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Operation3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum Operation3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ANDD") )]
		CodeANDD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ORRR") )]
		CodeORRR,
	}
	
	impl Operation3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// OrganisationIdentification15Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// PartyIdentification121Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PartyIdentification121Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
		pub any_bic: Option<AnyBICDec2014Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none") )]
		pub lgl_ntty_idr: Option<LEIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
		pub nm_and_adr: Option<NameAndAddress5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
		pub prtry_id: Option<GenericIdentification1>,
	}
	
	impl PartyIdentification121Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
			if let Some(ref lgl_ntty_idr_value) = self.lgl_ntty_idr { if let Err(e) = lgl_ntty_idr_value.validate() { return Err(e); } }
			if let Some(ref nm_and_adr_value) = self.nm_and_adr { if let Err(e) = nm_and_adr_value.validate() { return Err(e); } }
			if let Some(ref prtry_id_value) = self.prtry_id { if let Err(e) = prtry_id_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PartyIdentification248Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// PartyNatureType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum PartyNatureType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NFIN") )]
		CodeNFIN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FIIN") )]
		CodeFIIN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CCPS") )]
		CodeCCPS,
	}
	
	impl PartyNatureType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	
	
	// ProductClassificationCriteria1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ProductClassificationCriteria1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClssfctnFinInstrm", skip_serializing_if = "Option::is_none") )]
		pub clssfctn_fin_instrm: Option<Vec<CFIOct2015Identifier>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none") )]
		pub unq_pdct_idr: Option<Vec<Max52Text>>,
	}
	
	impl ProductClassificationCriteria1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref clssfctn_fin_instrm_vec) = self.clssfctn_fin_instrm { for item in clssfctn_fin_instrm_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref unq_pdct_idr_vec) = self.unq_pdct_idr { for item in unq_pdct_idr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// ProductType4Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum ProductType4Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CRDT") )]
		CodeCRDT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CURR") )]
		CodeCURR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EQUI") )]
		CodeEQUI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INTR") )]
		CodeINTR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "COMM") )]
		CodeCOMM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
	}
	
	impl ProductType4Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// SecuritiesTradeVenueCriteria1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SecuritiesTradeVenueCriteria1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MIC", skip_serializing_if = "Option::is_none") )]
		pub mic: Option<Vec<MICIdentifier>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyMIC", skip_serializing_if = "Option::is_none") )]
		pub any_mic: Option<AnyMIC1Code>,
	}
	
	impl SecuritiesTradeVenueCriteria1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref mic_vec) = self.mic { for item in mic_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref any_mic_value) = self.any_mic { if let Err(e) = any_mic_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SecurityIdentification20Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SecurityIdentification20Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
		pub isin: Option<ISINOct2015Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max25Text>,
	}
	
	impl SecurityIdentification20Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref isin_value) = self.isin { if let Err(e) = isin_value.validate() { return Err(e); } }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SecurityIdentificationQuery4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SecurityIdentificationQuery4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
		pub isin: Option<Vec<ISINOct2015Identifier>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AltrntvInstrmId", skip_serializing_if = "Option::is_none") )]
		pub altrntv_instrm_id: Option<Vec<Max52Text>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NotAvlbl", skip_serializing_if = "Option::is_none") )]
		pub not_avlbl: Option<NotAvailable1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none") )]
		pub unq_pdct_idr: Option<Vec<Max52Text>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Indx", skip_serializing_if = "Option::is_none") )]
		pub indx: Option<Vec<SecurityIdentification20Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Bskt", skip_serializing_if = "Option::is_none") )]
		pub bskt: Option<Vec<BasketQuery1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NotRptd", skip_serializing_if = "Option::is_none") )]
		pub not_rptd: Option<NotReported1Code>,
	}
	
	impl SecurityIdentificationQuery4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref isin_vec) = self.isin { for item in isin_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref altrntv_instrm_id_vec) = self.altrntv_instrm_id { for item in altrntv_instrm_id_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref not_avlbl_value) = self.not_avlbl { if let Err(e) = not_avlbl_value.validate() { return Err(e); } }
			if let Some(ref unq_pdct_idr_vec) = self.unq_pdct_idr { for item in unq_pdct_idr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref indx_vec) = self.indx { for item in indx_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref bskt_vec) = self.bskt { for item in bskt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref not_rptd_value) = self.not_rptd { if let Err(e) = not_rptd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SecurityIdentificationQueryCriteria1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SecurityIdentificationQueryCriteria1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
		pub isin: Option<Vec<ISINOct2015Identifier>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AltrntvInstrmId", skip_serializing_if = "Option::is_none") )]
		pub altrntv_instrm_id: Option<Vec<Max52Text>>,
	}
	
	impl SecurityIdentificationQueryCriteria1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref isin_vec) = self.isin { for item in isin_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref altrntv_instrm_id_vec) = self.altrntv_instrm_id { for item in altrntv_instrm_id_vec { if let Err(e) = item.validate() { return Err(e); } } }
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
	
	
	// TradeAdditionalQueryCriteria9 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct TradeAdditionalQueryCriteria9 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ActnTp", skip_serializing_if = "Option::is_none") )]
		pub actn_tp: Option<Vec<TransactionOperationType8Code>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ExctnVn", skip_serializing_if = "Option::is_none") )]
		pub exctn_vn: Option<SecuritiesTradeVenueCriteria1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtrOfCtrPty", skip_serializing_if = "Option::is_none") )]
		pub ntr_of_ctr_pty: Option<PartyNatureType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CorpSctr", skip_serializing_if = "Option::is_none") )]
		pub corp_sctr: Option<CorporateSectorCriteria6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AsstClss", skip_serializing_if = "Option::is_none") )]
		pub asst_clss: Option<Vec<ProductType4Code>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PdctClssfctn", skip_serializing_if = "Option::is_none") )]
		pub pdct_clssfctn: Option<ProductClassificationCriteria1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Lvl", skip_serializing_if = "Option::is_none") )]
		pub lvl: Option<ModificationLevel1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EvtTp", skip_serializing_if = "Option::is_none") )]
		pub evt_tp: Option<Vec<DerivativeEventType3Code>>,
	}
	
	impl TradeAdditionalQueryCriteria9 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref actn_tp_vec) = self.actn_tp { for item in actn_tp_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref exctn_vn_value) = self.exctn_vn { if let Err(e) = exctn_vn_value.validate() { return Err(e); } }
			if let Some(ref ntr_of_ctr_pty_value) = self.ntr_of_ctr_pty { if let Err(e) = ntr_of_ctr_pty_value.validate() { return Err(e); } }
			if let Some(ref corp_sctr_value) = self.corp_sctr { if let Err(e) = corp_sctr_value.validate() { return Err(e); } }
			if let Some(ref asst_clss_vec) = self.asst_clss { for item in asst_clss_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref pdct_clssfctn_value) = self.pdct_clssfctn { if let Err(e) = pdct_clssfctn_value.validate() { return Err(e); } }
			if let Some(ref lvl_value) = self.lvl { if let Err(e) = lvl_value.validate() { return Err(e); } }
			if let Some(ref evt_tp_vec) = self.evt_tp { for item in evt_tp_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TradeDateTimeQueryCriteria6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct TradeDateTimeQueryCriteria6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgDtTm", skip_serializing_if = "Option::is_none") )]
		pub rptg_dt_tm: Option<DateTimePeriod1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ExctnDtTm", skip_serializing_if = "Option::is_none") )]
		pub exctn_dt_tm: Option<DateTimePeriod1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none") )]
		pub mtrty_dt: Option<DateOrBlankQuery2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FctvDt", skip_serializing_if = "Option::is_none") )]
		pub fctv_dt: Option<DatePeriod1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ValtnDtTm", skip_serializing_if = "Option::is_none") )]
		pub valtn_dt_tm: Option<DateTimePeriod1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XprtnDt", skip_serializing_if = "Option::is_none") )]
		pub xprtn_dt: Option<DateOrBlankQuery2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EarlyTermntnDt", skip_serializing_if = "Option::is_none") )]
		pub early_termntn_dt: Option<DatePeriod1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CollTmStmp", skip_serializing_if = "Option::is_none") )]
		pub coll_tm_stmp: Option<DateTimeOrBlankQuery1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HstrclAsOfDt", skip_serializing_if = "Option::is_none") )]
		pub hstrcl_as_of_dt: Option<String>,
	}
	
	impl TradeDateTimeQueryCriteria6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref rptg_dt_tm_value) = self.rptg_dt_tm { if let Err(e) = rptg_dt_tm_value.validate() { return Err(e); } }
			if let Some(ref exctn_dt_tm_value) = self.exctn_dt_tm { if let Err(e) = exctn_dt_tm_value.validate() { return Err(e); } }
			if let Some(ref mtrty_dt_value) = self.mtrty_dt { if let Err(e) = mtrty_dt_value.validate() { return Err(e); } }
			if let Some(ref fctv_dt_value) = self.fctv_dt { if let Err(e) = fctv_dt_value.validate() { return Err(e); } }
			if let Some(ref valtn_dt_tm_value) = self.valtn_dt_tm { if let Err(e) = valtn_dt_tm_value.validate() { return Err(e); } }
			if let Some(ref xprtn_dt_value) = self.xprtn_dt { if let Err(e) = xprtn_dt_value.validate() { return Err(e); } }
			if let Some(ref early_termntn_dt_value) = self.early_termntn_dt { if let Err(e) = early_termntn_dt_value.validate() { return Err(e); } }
			if let Some(ref coll_tm_stmp_value) = self.coll_tm_stmp { if let Err(e) = coll_tm_stmp_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TradePartyIdentificationQuery10Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct TradePartyIdentificationQuery10Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<Vec<PartyIdentification248Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NotRptd", skip_serializing_if = "Option::is_none") )]
		pub not_rptd: Option<NotReported1Code>,
	}
	
	impl TradePartyIdentificationQuery10Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref id_vec) = self.id { for item in id_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref not_rptd_value) = self.not_rptd { if let Err(e) = not_rptd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TradePartyIdentificationQuery11Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct TradePartyIdentificationQuery11Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<Vec<OrganisationIdentification15Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NotRptd", skip_serializing_if = "Option::is_none") )]
		pub not_rptd: Option<NotReported1Code>,
	}
	
	impl TradePartyIdentificationQuery11Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref id_vec) = self.id { for item in id_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref not_rptd_value) = self.not_rptd { if let Err(e) = not_rptd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TradePartyQueryCriteria7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct TradePartyQueryCriteria7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Oprtr") )]
		pub oprtr: Operation3Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none") )]
		pub rptg_ctr_pty: Option<TradePartyIdentificationQuery10Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none") )]
		pub othr_ctr_pty: Option<TradePartyIdentificationQuery10Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Bnfcry", skip_serializing_if = "Option::is_none") )]
		pub bnfcry: Option<TradePartyIdentificationQuery10Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none") )]
		pub ntty_rspnsbl_for_rpt: Option<TradePartyIdentificationQuery11Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubmitgAgt", skip_serializing_if = "Option::is_none") )]
		pub submitg_agt: Option<TradePartyIdentificationQuery11Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Brkr", skip_serializing_if = "Option::is_none") )]
		pub brkr: Option<TradePartyIdentificationQuery11Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CCP", skip_serializing_if = "Option::is_none") )]
		pub ccp: Option<TradePartyIdentificationQuery11Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrMmb", skip_serializing_if = "Option::is_none") )]
		pub clr_mmb: Option<TradePartyIdentificationQuery10Choice>,
	}
	
	impl TradePartyQueryCriteria7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.oprtr.validate() { return Err(e); }
			if let Some(ref rptg_ctr_pty_value) = self.rptg_ctr_pty { if let Err(e) = rptg_ctr_pty_value.validate() { return Err(e); } }
			if let Some(ref othr_ctr_pty_value) = self.othr_ctr_pty { if let Err(e) = othr_ctr_pty_value.validate() { return Err(e); } }
			if let Some(ref bnfcry_value) = self.bnfcry { if let Err(e) = bnfcry_value.validate() { return Err(e); } }
			if let Some(ref ntty_rspnsbl_for_rpt_value) = self.ntty_rspnsbl_for_rpt { if let Err(e) = ntty_rspnsbl_for_rpt_value.validate() { return Err(e); } }
			if let Some(ref submitg_agt_value) = self.submitg_agt { if let Err(e) = submitg_agt_value.validate() { return Err(e); } }
			if let Some(ref brkr_value) = self.brkr { if let Err(e) = brkr_value.validate() { return Err(e); } }
			if let Some(ref ccp_value) = self.ccp { if let Err(e) = ccp_value.validate() { return Err(e); } }
			if let Some(ref clr_mmb_value) = self.clr_mmb { if let Err(e) = clr_mmb_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TradeQueryCriteria14 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct TradeQueryCriteria14 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradLifeCyclHstry", skip_serializing_if = "Option::is_none") )]
		pub trad_life_cycl_hstry: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MrgnLifeCyclHstry", skip_serializing_if = "Option::is_none") )]
		pub mrgn_life_cycl_hstry: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OutsdngTradInd") )]
		pub outsdng_trad_ind: bool,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradPtyCrit", skip_serializing_if = "Option::is_none") )]
		pub trad_pty_crit: Option<TradePartyQueryCriteria7>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmCrit", skip_serializing_if = "Option::is_none") )]
		pub fin_instrm_crit: Option<TradeSecurityIdentificationQueryCriteria3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TmCrit", skip_serializing_if = "Option::is_none") )]
		pub tm_crit: Option<TradeDateTimeQueryCriteria6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrCrit", skip_serializing_if = "Option::is_none") )]
		pub othr_crit: Option<TradeAdditionalQueryCriteria9>,
	}
	
	impl TradeQueryCriteria14 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref trad_pty_crit_value) = self.trad_pty_crit { if let Err(e) = trad_pty_crit_value.validate() { return Err(e); } }
			if let Some(ref fin_instrm_crit_value) = self.fin_instrm_crit { if let Err(e) = fin_instrm_crit_value.validate() { return Err(e); } }
			if let Some(ref tm_crit_value) = self.tm_crit { if let Err(e) = tm_crit_value.validate() { return Err(e); } }
			if let Some(ref othr_crit_value) = self.othr_crit { if let Err(e) = othr_crit_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TradeQueryExecutionFrequency3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct TradeQueryExecutionFrequency3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrqcyTp") )]
		pub frqcy_tp: Frequency14Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DlvryDay", skip_serializing_if = "Option::is_none") )]
		pub dlvry_day: Option<Vec<WeekDay3Code>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DayOfMnth", skip_serializing_if = "Option::is_none") )]
		pub day_of_mnth: Option<Vec<f64>>,
	}
	
	impl TradeQueryExecutionFrequency3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.frqcy_tp.validate() { return Err(e); }
			if let Some(ref dlvry_day_vec) = self.dlvry_day { for item in dlvry_day_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TradeRecurrentQuery7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct TradeRecurrentQuery7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "QryTp") )]
		pub qry_tp: Max1000Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Frqcy") )]
		pub frqcy: Vec<TradeQueryExecutionFrequency3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VldUntil") )]
		pub vld_until: String,
	}
	
	impl TradeRecurrentQuery7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.qry_tp.validate() { return Err(e); }
			for item in &self.frqcy { if let Err(e) = item.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TradeReportQuery18Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct TradeReportQuery18Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AdHocQry", skip_serializing_if = "Option::is_none") )]
		pub ad_hoc_qry: Option<TradeQueryCriteria14>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RcrntQry", skip_serializing_if = "Option::is_none") )]
		pub rcrnt_qry: Option<TradeRecurrentQuery7>,
	}
	
	impl TradeReportQuery18Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ad_hoc_qry_value) = self.ad_hoc_qry { if let Err(e) = ad_hoc_qry_value.validate() { return Err(e); } }
			if let Some(ref rcrnt_qry_value) = self.rcrnt_qry { if let Err(e) = rcrnt_qry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TradeSecurityIdentificationQueryCriteria3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct TradeSecurityIdentificationQueryCriteria3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Oprtr") )]
		pub oprtr: Operation3Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<Vec<SecurityIdentificationQueryCriteria1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctTp", skip_serializing_if = "Option::is_none") )]
		pub ctrct_tp: Option<Vec<FinancialInstrumentContractType2Code>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
		pub isin: Option<Vec<ISINQueryCriteria1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none") )]
		pub unq_pdct_idr: Option<Vec<UPIQueryCriteria1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygInstrmId", skip_serializing_if = "Option::is_none") )]
		pub undrlyg_instrm_id: Option<Vec<SecurityIdentificationQuery4Choice>>,
	}
	
	impl TradeSecurityIdentificationQueryCriteria3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.oprtr.validate() { return Err(e); }
			if let Some(ref id_vec) = self.id { for item in id_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref ctrct_tp_vec) = self.ctrct_tp { for item in ctrct_tp_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref isin_vec) = self.isin { for item in isin_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref unq_pdct_idr_vec) = self.unq_pdct_idr { for item in unq_pdct_idr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref undrlyg_instrm_id_vec) = self.undrlyg_instrm_id { for item in undrlyg_instrm_id_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TransactionOperationType8Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum TransactionOperationType8Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "COMP") )]
		CodeCOMP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CORR") )]
		CodeCORR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EROR") )]
		CodeEROR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MODI") )]
		CodeMODI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NEWT") )]
		CodeNEWT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "POSC") )]
		CodePOSC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REVI") )]
		CodeREVI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TERM") )]
		CodeTERM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VALU") )]
		CodeVALU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MARU") )]
		CodeMARU,
	}
	
	impl TransactionOperationType8Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// TrueFalseIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// UPIQueryCriteria1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct UPIQueryCriteria1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Idr", skip_serializing_if = "Option::is_none") )]
		pub idr: Option<Vec<Max52Text>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NotRptd", skip_serializing_if = "Option::is_none") )]
		pub not_rptd: Option<NotReported1Code>,
	}
	
	impl UPIQueryCriteria1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref idr_vec) = self.idr { for item in idr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref not_rptd_value) = self.not_rptd { if let Err(e) = not_rptd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// WeekDay3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum WeekDay3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ALLD") )]
		CodeALLD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XBHL") )]
		CodeXBHL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IBHL") )]
		CodeIBHL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FRID") )]
		CodeFRID,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MOND") )]
		CodeMOND,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SATD") )]
		CodeSATD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SUND") )]
		CodeSUND,
		#[cfg_attr( feature = "derive_serde", serde(rename = "THUD") )]
		CodeTHUD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TUED") )]
		CodeTUED,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WEDD") )]
		CodeWEDD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WDAY") )]
		CodeWDAY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WEND") )]
		CodeWEND,
	}
	
	impl WeekDay3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
}