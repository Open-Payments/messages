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
use crate::validationerror::*;


// ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType {
	#[serde(rename = "$value")]
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd20DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveOrHistoricCurrencyAnd20DecimalAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ActiveOrHistoricCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_and_amount_simple_type: f64,
}

impl ActiveOrHistoricCurrencyAndAmountSimpleType {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.active_or_historic_currency_and_amount_simple_type < 0.000000 {
			return Err(ValidationError::new(1003, "active_or_historic_currency_and_amount_simple_type is less than the minimum value of 0.000000".to_string()));
		}
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.active_or_historic_currency_code) {
			return Err(ValidationError::new(1005, "active_or_historic_currency_code does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// AmountAndDirection107 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection107 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAnd20DecimalAmount,
	#[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
	pub sgn: Option<bool>,
}

impl AmountAndDirection107 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.amt.validate() { return Err(e); }
		Ok(())
	}
}


// AmountAndDirection53 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection53 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
	pub sgn: Option<bool>,
}

impl AmountAndDirection53 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.amt.validate() { return Err(e); }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
		if !pattern.is_match(&self.any_bic_dec2014_identifier) {
			return Err(ValidationError::new(1005, "any_bic_dec2014_identifier does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BaseOneRate {
	#[serde(rename = "$value")]
	pub base_one_rate: f64,
}

impl BaseOneRate {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CFIOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CFIOct2015Identifier {
	#[serde(rename = "$value")]
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


// CollateralData33 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralData33 {
	#[serde(rename = "NetXpsrCollstnInd", skip_serializing_if = "Option::is_none")]
	pub net_xpsr_collstn_ind: Option<bool>,
	#[serde(rename = "CmpntTp", skip_serializing_if = "Option::is_none")]
	pub cmpnt_tp: Option<CollateralType6Code>,
	#[serde(rename = "CshCollCcy", skip_serializing_if = "Option::is_none")]
	pub csh_coll_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "PricCcy", skip_serializing_if = "Option::is_none")]
	pub pric_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "Qlty", skip_serializing_if = "Option::is_none")]
	pub qlty: Option<CollateralQualityType1Code>,
	#[serde(rename = "Mtrty", skip_serializing_if = "Option::is_none")]
	pub mtrty: Option<ContractTerm6Choice>,
	#[serde(rename = "IssrJursdctn", skip_serializing_if = "Option::is_none")]
	pub issr_jursdctn: Option<IssuerJurisdiction1Choice>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<SecuritiesLendingType3Choice>,
	#[serde(rename = "TradRpstry", skip_serializing_if = "Option::is_none")]
	pub trad_rpstry: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "RcncltnFlg", skip_serializing_if = "Option::is_none")]
	pub rcncltn_flg: Option<ReconciliationFlag2>,
	#[serde(rename = "RinvstdCsh", skip_serializing_if = "Option::is_none")]
	pub rinvstd_csh: Option<ReinvestedCashTypeAndAmount2>,
}

impl CollateralData33 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cmpnt_tp_value) = self.cmpnt_tp { if let Err(e) = cmpnt_tp_value.validate() { return Err(e); } }
		if let Some(ref csh_coll_ccy_value) = self.csh_coll_ccy { if let Err(e) = csh_coll_ccy_value.validate() { return Err(e); } }
		if let Some(ref pric_ccy_value) = self.pric_ccy { if let Err(e) = pric_ccy_value.validate() { return Err(e); } }
		if let Some(ref qlty_value) = self.qlty { if let Err(e) = qlty_value.validate() { return Err(e); } }
		if let Some(ref mtrty_value) = self.mtrty { if let Err(e) = mtrty_value.validate() { return Err(e); } }
		if let Some(ref issr_jursdctn_value) = self.issr_jursdctn { if let Err(e) = issr_jursdctn_value.validate() { return Err(e); } }
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		if let Some(ref trad_rpstry_value) = self.trad_rpstry { if let Err(e) = trad_rpstry_value.validate() { return Err(e); } }
		if let Some(ref rcncltn_flg_value) = self.rcncltn_flg { if let Err(e) = rcncltn_flg_value.validate() { return Err(e); } }
		if let Some(ref rinvstd_csh_value) = self.rinvstd_csh { if let Err(e) = rinvstd_csh_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CollateralQualityType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CollateralQualityType1Code {
	#[default]
	#[serde(rename = "INVG")]
	CodeINVG,
	#[serde(rename = "NIVG")]
	CodeNIVG,
	#[serde(rename = "NOTR")]
	CodeNOTR,
	#[serde(rename = "NOAP")]
	CodeNOAP,
}

impl CollateralQualityType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CollateralRole1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CollateralRole1Code {
	#[default]
	#[serde(rename = "GIVE")]
	CodeGIVE,
	#[serde(rename = "TAKE")]
	CodeTAKE,
}

impl CollateralRole1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ContractTerm6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractTerm6Choice {
	#[serde(rename = "Opn", skip_serializing_if = "Option::is_none")]
	pub opn: Option<bool>,
	#[serde(rename = "Fxd", skip_serializing_if = "Option::is_none")]
	pub fxd: Option<TimeToMaturity2Choice>,
}

impl ContractTerm6Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref fxd_value) = self.fxd { if let Err(e) = fxd_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CounterpartyData86 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyData86 {
	#[serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none")]
	pub rptg_ctr_pty: Option<CounterpartyIdentification10>,
	#[serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none")]
	pub othr_ctr_pty: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "TrptyAgt", skip_serializing_if = "Option::is_none")]
	pub trpty_agt: Option<bool>,
	#[serde(rename = "AgtLndr", skip_serializing_if = "Option::is_none")]
	pub agt_lndr: Option<bool>,
}

impl CounterpartyData86 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref rptg_ctr_pty_value) = self.rptg_ctr_pty { if let Err(e) = rptg_ctr_pty_value.validate() { return Err(e); } }
		if let Some(ref othr_ctr_pty_value) = self.othr_ctr_pty { if let Err(e) = othr_ctr_pty_value.validate() { return Err(e); } }
		Ok(())
	}
}


// CounterpartyIdentification10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyIdentification10 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "Sd", skip_serializing_if = "Option::is_none")]
	pub sd: Option<CollateralRole1Code>,
}

impl CounterpartyIdentification10 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
		if let Some(ref sd_value) = self.sd { if let Err(e) = sd_value.validate() { return Err(e); } }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.country_code) {
			return Err(ValidationError::new(1005, "country_code does not match the required pattern".to_string()));
		}
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ExposureMetrics4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExposureMetrics4 {
	#[serde(rename = "PrncplAmt", skip_serializing_if = "Option::is_none")]
	pub prncpl_amt: Option<PrincipalAmount3>,
	#[serde(rename = "LnVal", skip_serializing_if = "Option::is_none")]
	pub ln_val: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "MktVal", skip_serializing_if = "Option::is_none")]
	pub mkt_val: Option<AmountAndDirection53>,
	#[serde(rename = "OutsdngMrgnLnAmt", skip_serializing_if = "Option::is_none")]
	pub outsdng_mrgn_ln_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "ShrtMktValAmt", skip_serializing_if = "Option::is_none")]
	pub shrt_mkt_val_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "MrgnLn", skip_serializing_if = "Option::is_none")]
	pub mrgn_ln: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "CshCollAmt", skip_serializing_if = "Option::is_none")]
	pub csh_coll_amt: Option<AmountAndDirection53>,
	#[serde(rename = "CollMktVal", skip_serializing_if = "Option::is_none")]
	pub coll_mkt_val: Option<AmountAndDirection53>,
}

impl ExposureMetrics4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref prncpl_amt_value) = self.prncpl_amt { if let Err(e) = prncpl_amt_value.validate() { return Err(e); } }
		if let Some(ref ln_val_value) = self.ln_val { if let Err(e) = ln_val_value.validate() { return Err(e); } }
		if let Some(ref mkt_val_value) = self.mkt_val { if let Err(e) = mkt_val_value.validate() { return Err(e); } }
		if let Some(ref outsdng_mrgn_ln_amt_value) = self.outsdng_mrgn_ln_amt { if let Err(e) = outsdng_mrgn_ln_amt_value.validate() { return Err(e); } }
		if let Some(ref shrt_mkt_val_amt_value) = self.shrt_mkt_val_amt { if let Err(e) = shrt_mkt_val_amt_value.validate() { return Err(e); } }
		if let Some(ref mrgn_ln_value) = self.mrgn_ln { if let Err(e) = mrgn_ln_value.validate() { return Err(e); } }
		if let Some(ref csh_coll_amt_value) = self.csh_coll_amt { if let Err(e) = csh_coll_amt_value.validate() { return Err(e); } }
		if let Some(ref coll_mkt_val_value) = self.coll_mkt_val { if let Err(e) = coll_mkt_val_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ExposureMetrics5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExposureMetrics5 {
	#[serde(rename = "CshCollAmt", skip_serializing_if = "Option::is_none")]
	pub csh_coll_amt: Option<AmountAndDirection53>,
	#[serde(rename = "CollMktVal", skip_serializing_if = "Option::is_none")]
	pub coll_mkt_val: Option<AmountAndDirection53>,
}

impl ExposureMetrics5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref csh_coll_amt_value) = self.csh_coll_amt { if let Err(e) = csh_coll_amt_value.validate() { return Err(e); } }
		if let Some(ref coll_mkt_val_value) = self.coll_mkt_val { if let Err(e) = coll_mkt_val_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ExposureMetrics6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExposureMetrics6 {
	#[serde(rename = "PstdMrgnOrColl", skip_serializing_if = "Option::is_none")]
	pub pstd_mrgn_or_coll: Option<PostedMarginOrCollateral4>,
}

impl ExposureMetrics6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref pstd_mrgn_or_coll_value) = self.pstd_mrgn_or_coll { if let Err(e) = pstd_mrgn_or_coll_value.validate() { return Err(e); } }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ExternalAgreementType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalAgreementType1Code {
	#[serde(rename = "$value")]
	pub external_agreement_type1_code: String,
}

impl ExternalAgreementType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_agreement_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_agreement_type1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_agreement_type1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_agreement_type1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalRatesAndTenors1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalRatesAndTenors1Code {
	#[serde(rename = "$value")]
	pub external_rates_and_tenors1_code: String,
}

impl ExternalRatesAndTenors1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_rates_and_tenors1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_rates_and_tenors1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_rates_and_tenors1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_rates_and_tenors1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// ExternalSecuritiesLendingType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalSecuritiesLendingType1Code {
	#[serde(rename = "$value")]
	pub external_securities_lending_type1_code: String,
}

impl ExternalSecuritiesLendingType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.external_securities_lending_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_securities_lending_type1_code is shorter than the minimum length of 1".to_string()));
		}
		if self.external_securities_lending_type1_code.chars().count() > 4 {
			return Err(ValidationError::new(1002, "external_securities_lending_type1_code exceeds the maximum length of 4".to_string()));
		}
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
		if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
		if !pattern.is_match(&self.isin_oct2015_identifier) {
			return Err(ValidationError::new(1005, "isin_oct2015_identifier does not match the required pattern".to_string()));
		}
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// IssuerJurisdiction1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IssuerJurisdiction1Choice {
	#[serde(rename = "CtryCd", skip_serializing_if = "Option::is_none")]
	pub ctry_cd: Option<CountryCode>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Max35Text>,
}

impl IssuerJurisdiction1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref ctry_cd_value) = self.ctry_cd { if let Err(e) = ctry_cd_value.validate() { return Err(e); } }
		if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.lei_identifier) {
			return Err(ValidationError::new(1005, "lei_identifier does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// LoanData134 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LoanData134 {
	#[serde(rename = "CtrctTp", skip_serializing_if = "Option::is_none")]
	pub ctrct_tp: Option<ExposureType10Code>,
	#[serde(rename = "Clrd", skip_serializing_if = "Option::is_none")]
	pub clrd: Option<bool>,
	#[serde(rename = "PrtflCd", skip_serializing_if = "Option::is_none")]
	pub prtfl_cd: Option<Max52Text>,
	#[serde(rename = "TradgVn", skip_serializing_if = "Option::is_none")]
	pub tradg_vn: Option<TradingVenueType1Choice>,
	#[serde(rename = "MstrAgrmtTp", skip_serializing_if = "Option::is_none")]
	pub mstr_agrmt_tp: Option<ExternalAgreementType1Code>,
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "GnlColl", skip_serializing_if = "Option::is_none")]
	pub gnl_coll: Option<SpecialCollateral1Code>,
	#[serde(rename = "Term", skip_serializing_if = "Option::is_none")]
	pub term: Option<ContractTerm6Choice>,
	#[serde(rename = "Rates", skip_serializing_if = "Option::is_none")]
	pub rates: Option<Rates1Choice>,
	#[serde(rename = "PrncplAmtCcy", skip_serializing_if = "Option::is_none")]
	pub prncpl_amt_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "PricCcy", skip_serializing_if = "Option::is_none")]
	pub pric_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "Scty", skip_serializing_if = "Option::is_none")]
	pub scty: Option<Security49>,
	#[serde(rename = "OutsdngMrgnLnCcy", skip_serializing_if = "Option::is_none")]
	pub outsdng_mrgn_ln_ccy: Option<ActiveOrHistoricCurrencyCode>,
}

impl LoanData134 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref ctrct_tp_value) = self.ctrct_tp { if let Err(e) = ctrct_tp_value.validate() { return Err(e); } }
		if let Some(ref prtfl_cd_value) = self.prtfl_cd { if let Err(e) = prtfl_cd_value.validate() { return Err(e); } }
		if let Some(ref tradg_vn_value) = self.tradg_vn { if let Err(e) = tradg_vn_value.validate() { return Err(e); } }
		if let Some(ref mstr_agrmt_tp_value) = self.mstr_agrmt_tp { if let Err(e) = mstr_agrmt_tp_value.validate() { return Err(e); } }
		if let Some(ref gnl_coll_value) = self.gnl_coll { if let Err(e) = gnl_coll_value.validate() { return Err(e); } }
		if let Some(ref term_value) = self.term { if let Err(e) = term_value.validate() { return Err(e); } }
		if let Some(ref rates_value) = self.rates { if let Err(e) = rates_value.validate() { return Err(e); } }
		if let Some(ref prncpl_amt_ccy_value) = self.prncpl_amt_ccy { if let Err(e) = prncpl_amt_ccy_value.validate() { return Err(e); } }
		if let Some(ref pric_ccy_value) = self.pric_ccy { if let Err(e) = pric_ccy_value.validate() { return Err(e); } }
		if let Some(ref scty_value) = self.scty { if let Err(e) = scty_value.validate() { return Err(e); } }
		if let Some(ref outsdng_mrgn_ln_ccy_value) = self.outsdng_mrgn_ln_ccy { if let Err(e) = outsdng_mrgn_ln_ccy_value.validate() { return Err(e); } }
		Ok(())
	}
}


// LongFraction19DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LongFraction19DecimalNumber {
	#[serde(rename = "$value")]
	pub long_fraction19_decimal_number: f64,
}

impl LongFraction19DecimalNumber {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// MaturityTerm2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MaturityTerm2 {
	#[serde(rename = "Unit")]
	pub unit: RateBasis1Code,
	#[serde(rename = "Val")]
	pub val: f64,
}

impl MaturityTerm2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.unit.validate() { return Err(e); }
		Ok(())
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


// Max15NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max15NumericText {
	#[serde(rename = "$value")]
	pub max15_numeric_text: String,
}

impl Max15NumericText {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{1,15}").unwrap();
		if !pattern.is_match(&self.max15_numeric_text) {
			return Err(ValidationError::new(1005, "max15_numeric_text does not match the required pattern".to_string()));
		}
		Ok(())
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max35Text {
	#[serde(rename = "$value")]
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


// Max3Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max3Number {
	#[serde(rename = "$value")]
	pub max3_number: f64,
}

impl Max3Number {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
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
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max52Text {
	#[serde(rename = "$value")]
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


// Max72Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max72Text {
	#[serde(rename = "$value")]
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


// NamedPosition3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NamedPosition3 {
	#[serde(rename = "RefDt")]
	pub ref_dt: String,
	#[serde(rename = "GnlInf", skip_serializing_if = "Option::is_none")]
	pub gnl_inf: Option<Vec<PositionSet16>>,
	#[serde(rename = "Ln", skip_serializing_if = "Option::is_none")]
	pub ln: Option<Vec<PositionSet17>>,
	#[serde(rename = "Coll", skip_serializing_if = "Option::is_none")]
	pub coll: Option<Vec<PositionSet18>>,
	#[serde(rename = "Mrgn", skip_serializing_if = "Option::is_none")]
	pub mrgn: Option<Vec<PositionSet20>>,
	#[serde(rename = "Reuse", skip_serializing_if = "Option::is_none")]
	pub reuse: Option<Vec<PositionSet19>>,
}

impl NamedPosition3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref gnl_inf_vec) = self.gnl_inf { for item in gnl_inf_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref ln_vec) = self.ln { for item in ln_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref coll_vec) = self.coll { for item in coll_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref mrgn_vec) = self.mrgn { for item in mrgn_vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref reuse_vec) = self.reuse { for item in reuse_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
		if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
		if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
		if let Some(ref dmcl_value) = self.dmcl { if let Err(e) = dmcl_value.validate() { return Err(e); } }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PlusOrMinusIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "$value")]
	pub plus_or_minus_indicator: bool,
}

impl PlusOrMinusIndicator {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PositionSet16 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSet16 {
	#[serde(rename = "Dmnsns")]
	pub dmnsns: PositionSetDimensions14,
	#[serde(rename = "Mtrcs")]
	pub mtrcs: PositionSetMetrics7,
}

impl PositionSet16 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.dmnsns.validate() { return Err(e); }
		if let Err(e) = self.mtrcs.validate() { return Err(e); }
		Ok(())
	}
}


// PositionSet17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSet17 {
	#[serde(rename = "Dmnsns")]
	pub dmnsns: PositionSetDimensions14,
	#[serde(rename = "Mtrcs")]
	pub mtrcs: PositionSetMetrics13,
}

impl PositionSet17 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.dmnsns.validate() { return Err(e); }
		if let Err(e) = self.mtrcs.validate() { return Err(e); }
		Ok(())
	}
}


// PositionSet18 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSet18 {
	#[serde(rename = "Dmnsns")]
	pub dmnsns: PositionSetDimensions14,
	#[serde(rename = "Mtrcs")]
	pub mtrcs: PositionSetMetrics12,
}

impl PositionSet18 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.dmnsns.validate() { return Err(e); }
		if let Err(e) = self.mtrcs.validate() { return Err(e); }
		Ok(())
	}
}


// PositionSet19 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSet19 {
	#[serde(rename = "Dmnsns")]
	pub dmnsns: PositionSetDimensions12,
	#[serde(rename = "Mtrcs")]
	pub mtrcs: PositionSetMetrics11,
}

impl PositionSet19 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.dmnsns.validate() { return Err(e); }
		if let Err(e) = self.mtrcs.validate() { return Err(e); }
		Ok(())
	}
}


// PositionSet20 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSet20 {
	#[serde(rename = "Dmnsns")]
	pub dmnsns: PositionSetDimensions15,
	#[serde(rename = "Mtrcs")]
	pub mtrcs: PositionSetMetrics10,
}

impl PositionSet20 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.dmnsns.validate() { return Err(e); }
		if let Err(e) = self.mtrcs.validate() { return Err(e); }
		Ok(())
	}
}


// PositionSetDimensions12 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetDimensions12 {
	#[serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none")]
	pub rptg_ctr_pty: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "CollData", skip_serializing_if = "Option::is_none")]
	pub coll_data: Option<CollateralData33>,
	#[serde(rename = "OtlrsIncl", skip_serializing_if = "Option::is_none")]
	pub otlrs_incl: Option<bool>,
}

impl PositionSetDimensions12 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref rptg_ctr_pty_value) = self.rptg_ctr_pty { if let Err(e) = rptg_ctr_pty_value.validate() { return Err(e); } }
		if let Some(ref coll_data_value) = self.coll_data { if let Err(e) = coll_data_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PositionSetDimensions14 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetDimensions14 {
	#[serde(rename = "CtrPtyData", skip_serializing_if = "Option::is_none")]
	pub ctr_pty_data: Option<CounterpartyData86>,
	#[serde(rename = "LnData", skip_serializing_if = "Option::is_none")]
	pub ln_data: Option<LoanData134>,
	#[serde(rename = "CollData", skip_serializing_if = "Option::is_none")]
	pub coll_data: Option<CollateralData33>,
	#[serde(rename = "OtlrsIncl", skip_serializing_if = "Option::is_none")]
	pub otlrs_incl: Option<bool>,
}

impl PositionSetDimensions14 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref ctr_pty_data_value) = self.ctr_pty_data { if let Err(e) = ctr_pty_data_value.validate() { return Err(e); } }
		if let Some(ref ln_data_value) = self.ln_data { if let Err(e) = ln_data_value.validate() { return Err(e); } }
		if let Some(ref coll_data_value) = self.coll_data { if let Err(e) = coll_data_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PositionSetDimensions15 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetDimensions15 {
	#[serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none")]
	pub rptg_ctr_pty: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none")]
	pub othr_ctr_pty: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "CollPrtflId", skip_serializing_if = "Option::is_none")]
	pub coll_prtfl_id: Option<Max52Text>,
	#[serde(rename = "OtlrsIncl", skip_serializing_if = "Option::is_none")]
	pub otlrs_incl: Option<bool>,
}

impl PositionSetDimensions15 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref rptg_ctr_pty_value) = self.rptg_ctr_pty { if let Err(e) = rptg_ctr_pty_value.validate() { return Err(e); } }
		if let Some(ref othr_ctr_pty_value) = self.othr_ctr_pty { if let Err(e) = othr_ctr_pty_value.validate() { return Err(e); } }
		if let Some(ref coll_prtfl_id_value) = self.coll_prtfl_id { if let Err(e) = coll_prtfl_id_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PositionSetMetrics10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetMetrics10 {
	#[serde(rename = "VolMtrcs", skip_serializing_if = "Option::is_none")]
	pub vol_mtrcs: Option<ExposureMetrics6>,
}

impl PositionSetMetrics10 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vol_mtrcs_value) = self.vol_mtrcs { if let Err(e) = vol_mtrcs_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PositionSetMetrics11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetMetrics11 {
	#[serde(rename = "VolMtrcs", skip_serializing_if = "Option::is_none")]
	pub vol_mtrcs: Option<VolumeMetrics4>,
	#[serde(rename = "CshRinvstmtRate", skip_serializing_if = "Option::is_none")]
	pub csh_rinvstmt_rate: Option<f64>,
}

impl PositionSetMetrics11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vol_mtrcs_value) = self.vol_mtrcs { if let Err(e) = vol_mtrcs_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PositionSetMetrics12 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetMetrics12 {
	#[serde(rename = "VolMtrcs", skip_serializing_if = "Option::is_none")]
	pub vol_mtrcs: Option<VolumeMetrics6>,
	#[serde(rename = "HrcutOrMrgn", skip_serializing_if = "Option::is_none")]
	pub hrcut_or_mrgn: Option<f64>,
	#[serde(rename = "QtyOrNmnlAmt", skip_serializing_if = "Option::is_none")]
	pub qty_or_nmnl_amt: Option<QuantityNominalValue2Choice>,
}

impl PositionSetMetrics12 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vol_mtrcs_value) = self.vol_mtrcs { if let Err(e) = vol_mtrcs_value.validate() { return Err(e); } }
		if let Some(ref qty_or_nmnl_amt_value) = self.qty_or_nmnl_amt { if let Err(e) = qty_or_nmnl_amt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PositionSetMetrics13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetMetrics13 {
	#[serde(rename = "VolMtrcs")]
	pub vol_mtrcs: VolumeMetrics5,
	#[serde(rename = "PricMtrcs", skip_serializing_if = "Option::is_none")]
	pub pric_mtrcs: Option<PriceMetrics3>,
}

impl PositionSetMetrics13 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.vol_mtrcs.validate() { return Err(e); }
		if let Some(ref pric_mtrcs_value) = self.pric_mtrcs { if let Err(e) = pric_mtrcs_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PositionSetMetrics7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetMetrics7 {
	#[serde(rename = "VolMtrcs")]
	pub vol_mtrcs: VolumeMetrics5,
}

impl PositionSetMetrics7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.vol_mtrcs.validate() { return Err(e); }
		Ok(())
	}
}


// PositionSetReport3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetReport3Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[serde(rename = "Rpt", skip_serializing_if = "Option::is_none")]
	pub rpt: Option<NamedPosition3>,
}

impl PositionSetReport3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref data_set_actn_value) = self.data_set_actn { if let Err(e) = data_set_actn_value.validate() { return Err(e); } }
		if let Some(ref rpt_value) = self.rpt { if let Err(e) = rpt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PostedMarginOrCollateral4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostedMarginOrCollateral4 {
	#[serde(rename = "InitlMrgnPstd", skip_serializing_if = "Option::is_none")]
	pub initl_mrgn_pstd: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "VartnMrgnPstd", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_pstd: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "XcssCollPstd", skip_serializing_if = "Option::is_none")]
	pub xcss_coll_pstd: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl PostedMarginOrCollateral4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref initl_mrgn_pstd_value) = self.initl_mrgn_pstd { if let Err(e) = initl_mrgn_pstd_value.validate() { return Err(e); } }
		if let Some(ref vartn_mrgn_pstd_value) = self.vartn_mrgn_pstd { if let Err(e) = vartn_mrgn_pstd_value.validate() { return Err(e); } }
		if let Some(ref xcss_coll_pstd_value) = self.xcss_coll_pstd { if let Err(e) = xcss_coll_pstd_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PriceMetrics3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceMetrics3 {
	#[serde(rename = "Rates", skip_serializing_if = "Option::is_none")]
	pub rates: Option<Rates3>,
	#[serde(rename = "LndgFee", skip_serializing_if = "Option::is_none")]
	pub lndg_fee: Option<f64>,
}

impl PriceMetrics3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref rates_value) = self.rates { if let Err(e) = rates_value.validate() { return Err(e); } }
		Ok(())
	}
}


// PriceStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PriceStatus1Code {
	#[default]
	#[serde(rename = "PNDG")]
	CodePNDG,
	#[serde(rename = "NOAP")]
	CodeNOAP,
}

impl PriceStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PrincipalAmount3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PrincipalAmount3 {
	#[serde(rename = "ValDtAmt", skip_serializing_if = "Option::is_none")]
	pub val_dt_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "MtrtyDtAmt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl PrincipalAmount3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val_dt_amt_value) = self.val_dt_amt { if let Err(e) = val_dt_amt_value.validate() { return Err(e); } }
		if let Some(ref mtrty_dt_amt_value) = self.mtrty_dt_amt { if let Err(e) = mtrty_dt_amt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// QuantityNominalValue2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct QuantityNominalValue2Choice {
	#[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
	pub qty: Option<f64>,
	#[serde(rename = "NmnlVal", skip_serializing_if = "Option::is_none")]
	pub nmnl_val: Option<AmountAndDirection53>,
}

impl QuantityNominalValue2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref nmnl_val_value) = self.nmnl_val { if let Err(e) = nmnl_val_value.validate() { return Err(e); } }
		Ok(())
	}
}


// RateBasis1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RateBasis1Code {
	#[default]
	#[serde(rename = "DAYS")]
	CodeDAYS,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "YEAR")]
	CodeYEAR,
}

impl RateBasis1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Rates1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Rates1Choice {
	#[serde(rename = "Fxd", skip_serializing_if = "Option::is_none")]
	pub fxd: Option<NoReasonCode>,
	#[serde(rename = "Fltg", skip_serializing_if = "Option::is_none")]
	pub fltg: Option<ExternalRatesAndTenors1Code>,
}

impl Rates1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref fxd_value) = self.fxd { if let Err(e) = fxd_value.validate() { return Err(e); } }
		if let Some(ref fltg_value) = self.fltg { if let Err(e) = fltg_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Rates3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Rates3 {
	#[serde(rename = "Fxd", skip_serializing_if = "Option::is_none")]
	pub fxd: Option<f64>,
	#[serde(rename = "Fltg", skip_serializing_if = "Option::is_none")]
	pub fltg: Option<f64>,
	#[serde(rename = "BuySellBck", skip_serializing_if = "Option::is_none")]
	pub buy_sell_bck: Option<SecuritiesTransactionPrice18Choice>,
}

impl Rates3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref buy_sell_bck_value) = self.buy_sell_bck { if let Err(e) = buy_sell_bck_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ReconciliationFlag2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReconciliationFlag2 {
	#[serde(rename = "RptTp", skip_serializing_if = "Option::is_none")]
	pub rpt_tp: Option<TradeRepositoryReportingType1Code>,
	#[serde(rename = "BothCtrPtiesRptg", skip_serializing_if = "Option::is_none")]
	pub both_ctr_pties_rptg: Option<bool>,
	#[serde(rename = "PairdSts", skip_serializing_if = "Option::is_none")]
	pub paird_sts: Option<bool>,
	#[serde(rename = "LnRcncltnSts", skip_serializing_if = "Option::is_none")]
	pub ln_rcncltn_sts: Option<bool>,
	#[serde(rename = "CollRcncltnSts", skip_serializing_if = "Option::is_none")]
	pub coll_rcncltn_sts: Option<bool>,
	#[serde(rename = "ModSts", skip_serializing_if = "Option::is_none")]
	pub mod_sts: Option<bool>,
}

impl ReconciliationFlag2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref rpt_tp_value) = self.rpt_tp { if let Err(e) = rpt_tp_value.validate() { return Err(e); } }
		Ok(())
	}
}


// ReinvestedCashTypeAndAmount2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReinvestedCashTypeAndAmount2 {
	#[serde(rename = "Tp")]
	pub tp: ReinvestmentType1Code,
	#[serde(rename = "RinvstdCshCcy")]
	pub rinvstd_csh_ccy: ActiveOrHistoricCurrencyCode,
}

impl ReinvestedCashTypeAndAmount2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.tp.validate() { return Err(e); }
		if let Err(e) = self.rinvstd_csh_ccy.validate() { return Err(e); }
		Ok(())
	}
}


// ReinvestmentType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ReinvestmentType1Code {
	#[default]
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "OCMP")]
	CodeOCMP,
	#[serde(rename = "MMFT")]
	CodeMMFT,
	#[serde(rename = "REPM")]
	CodeREPM,
	#[serde(rename = "SDPU")]
	CodeSDPU,
}

impl ReinvestmentType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ReuseValue1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReuseValue1Choice {
	#[serde(rename = "Actl", skip_serializing_if = "Option::is_none")]
	pub actl: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Estmtd", skip_serializing_if = "Option::is_none")]
	pub estmtd: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl ReuseValue1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref actl_value) = self.actl { if let Err(e) = actl_value.validate() { return Err(e); } }
		if let Some(ref estmtd_value) = self.estmtd { if let Err(e) = estmtd_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SecuritiesFinancingReportingPositionSetReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesFinancingReportingPositionSetReportV01 {
	#[serde(rename = "AggtdPoss")]
	pub aggtd_poss: PositionSetReport3Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl SecuritiesFinancingReportingPositionSetReportV01 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.aggtd_poss.validate() { return Err(e); }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// SecuritiesLendingType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesLendingType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalSecuritiesLendingType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl SecuritiesLendingType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
		if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SecuritiesTransactionPrice18Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice18Choice {
	#[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
	pub mntry_val: Option<AmountAndDirection107>,
	#[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
	pub pctg: Option<f64>,
	#[serde(rename = "Dcml", skip_serializing_if = "Option::is_none")]
	pub dcml: Option<f64>,
	#[serde(rename = "BsisPts", skip_serializing_if = "Option::is_none")]
	pub bsis_pts: Option<f64>,
}

impl SecuritiesTransactionPrice18Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref mntry_val_value) = self.mntry_val { if let Err(e) = mntry_val_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SecuritiesTransactionPrice19Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice19Choice {
	#[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
	pub mntry_val: Option<AmountAndDirection107>,
	#[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
	pub unit: Option<f64>,
	#[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
	pub pctg: Option<f64>,
	#[serde(rename = "Yld", skip_serializing_if = "Option::is_none")]
	pub yld: Option<f64>,
	#[serde(rename = "Dcml", skip_serializing_if = "Option::is_none")]
	pub dcml: Option<f64>,
	#[serde(rename = "PdgPric", skip_serializing_if = "Option::is_none")]
	pub pdg_pric: Option<PriceStatus1Code>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<SecuritiesTransactionPrice5>,
}

impl SecuritiesTransactionPrice19Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref mntry_val_value) = self.mntry_val { if let Err(e) = mntry_val_value.validate() { return Err(e); } }
		if let Some(ref pdg_pric_value) = self.pdg_pric { if let Err(e) = pdg_pric_value.validate() { return Err(e); } }
		if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// SecuritiesTransactionPrice5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice5 {
	#[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
	pub val: Option<f64>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<Max35Text>,
}

impl SecuritiesTransactionPrice5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
		Ok(())
	}
}


// Security49 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Security49 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ISINOct2015Identifier>,
	#[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
	pub clssfctn_tp: Option<CFIOct2015Identifier>,
	#[serde(rename = "QtyOrNmnlVal", skip_serializing_if = "Option::is_none")]
	pub qty_or_nmnl_val: Option<QuantityNominalValue2Choice>,
	#[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
	pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
	#[serde(rename = "MktVal", skip_serializing_if = "Option::is_none")]
	pub mkt_val: Option<AmountAndDirection53>,
	#[serde(rename = "Qlty", skip_serializing_if = "Option::is_none")]
	pub qlty: Option<CollateralQualityType1Code>,
	#[serde(rename = "Mtrty", skip_serializing_if = "Option::is_none")]
	pub mtrty: Option<String>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<SecurityIssuer4>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<Vec<SecuritiesLendingType3Choice>>,
	#[serde(rename = "ExclsvArrgmnt", skip_serializing_if = "Option::is_none")]
	pub exclsv_arrgmnt: Option<bool>,
}

impl Security49 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
		if let Some(ref clssfctn_tp_value) = self.clssfctn_tp { if let Err(e) = clssfctn_tp_value.validate() { return Err(e); } }
		if let Some(ref qty_or_nmnl_val_value) = self.qty_or_nmnl_val { if let Err(e) = qty_or_nmnl_val_value.validate() { return Err(e); } }
		if let Some(ref unit_pric_value) = self.unit_pric { if let Err(e) = unit_pric_value.validate() { return Err(e); } }
		if let Some(ref mkt_val_value) = self.mkt_val { if let Err(e) = mkt_val_value.validate() { return Err(e); } }
		if let Some(ref qlty_value) = self.qlty { if let Err(e) = qlty_value.validate() { return Err(e); } }
		if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
		if let Some(ref tp_vec) = self.tp { for item in tp_vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// SecurityIssuer4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIssuer4 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "JursdctnCtry")]
	pub jursdctn_ctry: CountryCode,
}

impl SecurityIssuer4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
		if let Err(e) = self.jursdctn_ctry.validate() { return Err(e); }
		Ok(())
	}
}


// SpecialCollateral1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SpecialCollateral1Code {
	#[default]
	#[serde(rename = "GENE")]
	CodeGENE,
	#[serde(rename = "SPEC")]
	CodeSPEC,
}

impl SpecialCollateral1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SpecialPurpose2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SpecialPurpose2Code {
	#[default]
	#[serde(rename = "BLNK")]
	CodeBLNK,
	#[serde(rename = "NTAV")]
	CodeNTAV,
}

impl SpecialPurpose2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref plc_and_nm_value) = self.plc_and_nm { if let Err(e) = plc_and_nm_value.validate() { return Err(e); } }
		if let Err(e) = self.envlp.validate() { return Err(e); }
		Ok(())
	}
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TimeToMaturity2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeToMaturity2Choice {
	#[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
	pub prd: Option<TimeToMaturityPeriod2>,
	#[serde(rename = "Spcl", skip_serializing_if = "Option::is_none")]
	pub spcl: Option<SpecialPurpose2Code>,
}

impl TimeToMaturity2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref prd_value) = self.prd { if let Err(e) = prd_value.validate() { return Err(e); } }
		if let Some(ref spcl_value) = self.spcl { if let Err(e) = spcl_value.validate() { return Err(e); } }
		Ok(())
	}
}


// TimeToMaturityPeriod2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeToMaturityPeriod2 {
	#[serde(rename = "Start", skip_serializing_if = "Option::is_none")]
	pub start: Option<MaturityTerm2>,
	#[serde(rename = "End", skip_serializing_if = "Option::is_none")]
	pub end: Option<MaturityTerm2>,
}

impl TimeToMaturityPeriod2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref start_value) = self.start { if let Err(e) = start_value.validate() { return Err(e); } }
		if let Some(ref end_value) = self.end { if let Err(e) = end_value.validate() { return Err(e); } }
		Ok(())
	}
}


// TradeMarket2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TradeMarket2Code {
	#[default]
	#[serde(rename = "DMST")]
	CodeDMST,
	#[serde(rename = "FRGN")]
	CodeFRGN,
}

impl TradeMarket2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TradeRepositoryReportingType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TradeRepositoryReportingType1Code {
	#[default]
	#[serde(rename = "SWOS")]
	CodeSWOS,
	#[serde(rename = "TWOS")]
	CodeTWOS,
}

impl TradeRepositoryReportingType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TradingVenueType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingVenueType1Choice {
	#[serde(rename = "OnVn", skip_serializing_if = "Option::is_none")]
	pub on_vn: Option<TradeMarket2Code>,
	#[serde(rename = "OffVn", skip_serializing_if = "Option::is_none")]
	pub off_vn: Option<NoReasonCode>,
}

impl TradingVenueType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref on_vn_value) = self.on_vn { if let Err(e) = on_vn_value.validate() { return Err(e); } }
		if let Some(ref off_vn_value) = self.off_vn { if let Err(e) = off_vn_value.validate() { return Err(e); } }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// VolumeMetrics4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct VolumeMetrics4 {
	#[serde(rename = "ReuseVal", skip_serializing_if = "Option::is_none")]
	pub reuse_val: Option<ReuseValue1Choice>,
	#[serde(rename = "RinvstdCshAmt", skip_serializing_if = "Option::is_none")]
	pub rinvstd_csh_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl VolumeMetrics4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref reuse_val_value) = self.reuse_val { if let Err(e) = reuse_val_value.validate() { return Err(e); } }
		if let Some(ref rinvstd_csh_amt_value) = self.rinvstd_csh_amt { if let Err(e) = rinvstd_csh_amt_value.validate() { return Err(e); } }
		Ok(())
	}
}


// VolumeMetrics5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct VolumeMetrics5 {
	#[serde(rename = "NbOfTxs", skip_serializing_if = "Option::is_none")]
	pub nb_of_txs: Option<Max15NumericText>,
	#[serde(rename = "Xpsr", skip_serializing_if = "Option::is_none")]
	pub xpsr: Option<ExposureMetrics4>,
}

impl VolumeMetrics5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref nb_of_txs_value) = self.nb_of_txs { if let Err(e) = nb_of_txs_value.validate() { return Err(e); } }
		if let Some(ref xpsr_value) = self.xpsr { if let Err(e) = xpsr_value.validate() { return Err(e); } }
		Ok(())
	}
}


// VolumeMetrics6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct VolumeMetrics6 {
	#[serde(rename = "Postv", skip_serializing_if = "Option::is_none")]
	pub postv: Option<ExposureMetrics5>,
	#[serde(rename = "Neg", skip_serializing_if = "Option::is_none")]
	pub neg: Option<ExposureMetrics5>,
}

impl VolumeMetrics6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref postv_value) = self.postv { if let Err(e) = postv_value.validate() { return Err(e); } }
		if let Some(ref neg_value) = self.neg { if let Err(e) = neg_value.validate() { return Err(e); } }
		Ok(())
	}
}
