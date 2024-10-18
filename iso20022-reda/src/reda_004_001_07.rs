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


// AccountIdentificationAndName7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountIdentificationAndName7 {
	#[serde(rename = "Id")]
	pub id: CashAccountIdentification8Choice,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max35Text>,
}

impl AccountIdentificationAndName7 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		return true
	}
}


// AccountSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalAccountIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl AccountSchemeName1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// ActiveCurrencyAnd13DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyAnd13DecimalAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_currency_and13_decimal_amount_simple_type: f64,
}

impl ActiveCurrencyAnd13DecimalAmountSimpleType {
	pub fn validate(&self) -> bool {
		if self.active_currency_and13_decimal_amount_simple_type < 0.000000 {
			return false
		}
		return true
	}
}


// ActiveCurrencyAnd13DecimalAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd13DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveCurrencyAnd13DecimalAmount {
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


// AdditionalInformation15 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdditionalInformation15 {
	#[serde(rename = "InfTp")]
	pub inf_tp: GenericIdentification36,
	#[serde(rename = "InfVal")]
	pub inf_val: Max350Text,
}

impl AdditionalInformation15 {
	pub fn validate(&self) -> bool {
		if !self.inf_tp.validate() { return false }
		if !self.inf_val.validate() { return false }
		return true
	}
}


// AdditionalProductInformation3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdditionalProductInformation3 {
	#[serde(rename = "FinInstrmTxCostsExAnteUK", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_tx_costs_ex_ante_uk: Option<f64>,
	#[serde(rename = "FinInstrmTxCostsExPstUK", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_tx_costs_ex_pst_uk: Option<f64>,
}

impl AdditionalProductInformation3 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AdditionalReference10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdditionalReference10 {
	#[serde(rename = "Ref")]
	pub ref_attr: Max35Text,
	#[serde(rename = "RefIssr", skip_serializing_if = "Option::is_none")]
	pub ref_issr: Option<PartyIdentification139>,
	#[serde(rename = "MsgNm", skip_serializing_if = "Option::is_none")]
	pub msg_nm: Option<Max35Text>,
}

impl AdditionalReference10 {
	pub fn validate(&self) -> bool {
		if !self.ref_attr.validate() { return false }
		if let Some(ref ref_issr_value) = self.ref_issr { if !ref_issr_value.validate() { return false; } }
		if let Some(ref msg_nm_value) = self.msg_nm { if !msg_nm_value.validate() { return false; } }
		return true
	}
}


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


// AnnualChargePaymentType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AnnualChargePaymentType1Code {
	#[default]
	#[serde(rename = "CAPL")]
	CodeCAPL,
	#[serde(rename = "INCO")]
	CodeINCO,
}

impl AnnualChargePaymentType1Code {
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


// AssessmentOfValueRequiredUnderCOLLUKType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssessmentOfValueRequiredUnderCOLLUKType1Code {
	#[default]
	#[serde(rename = "YSCO")]
	CodeYSCO,
	#[serde(rename = "NSCO")]
	CodeNSCO,
}

impl AssessmentOfValueRequiredUnderCOLLUKType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// BusinessDayConvention1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BusinessDayConvention1Code {
	#[default]
	#[serde(rename = "FWNG")]
	CodeFWNG,
	#[serde(rename = "PREC")]
	CodePREC,
}

impl BusinessDayConvention1Code {
	pub fn validate(&self) -> bool {
		return true
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
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{6,6}").unwrap();
		if !pattern.is_match(&self.cfi_oct2015_identifier) {
			return false
		}
		return true
	}
}


// CashAccount205 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccount205 {
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveCurrencyCode>,
	#[serde(rename = "PmryAcct", skip_serializing_if = "Option::is_none")]
	pub pmry_acct: Option<CashAccount206>,
	#[serde(rename = "ScndryAcct", skip_serializing_if = "Option::is_none")]
	pub scndry_acct: Option<CashAccount206>,
}

impl CashAccount205 {
	pub fn validate(&self) -> bool {
		if let Some(ref ccy_value) = self.ccy { if !ccy_value.validate() { return false; } }
		if let Some(ref pmry_acct_value) = self.pmry_acct { if !pmry_acct_value.validate() { return false; } }
		if let Some(ref scndry_acct_value) = self.scndry_acct { if !scndry_acct_value.validate() { return false; } }
		return true
	}
}


// CashAccount206 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccount206 {
	#[serde(rename = "AcctId")]
	pub acct_id: AccountIdentificationAndName7,
	#[serde(rename = "Svcr", skip_serializing_if = "Option::is_none")]
	pub svcr: Option<AnyBICDec2014Identifier>,
	#[serde(rename = "AcctTpDesc", skip_serializing_if = "Option::is_none")]
	pub acct_tp_desc: Option<Max35Text>,
}

impl CashAccount206 {
	pub fn validate(&self) -> bool {
		if !self.acct_id.validate() { return false }
		if let Some(ref svcr_value) = self.svcr { if !svcr_value.validate() { return false; } }
		if let Some(ref acct_tp_desc_value) = self.acct_tp_desc { if !acct_tp_desc_value.validate() { return false; } }
		return true
	}
}


// CashAccountIdentification8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountIdentification8Choice {
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<GenericAccountIdentification1>,
	#[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
	pub iban: Option<IBAN2007Identifier>,
}

impl CashAccountIdentification8Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		if let Some(ref iban_value) = self.iban { if !iban_value.validate() { return false; } }
		return true
	}
}


// ChargeType8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ChargeType8Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InvestmentFundMiFIDFee2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl ChargeType8Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// ContactAttributes5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContactAttributes5 {
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress1>,
	#[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
	pub phne_nb: Option<PhoneNumber>,
	#[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
	pub fax_nb: Option<PhoneNumber>,
	#[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
	pub email_adr: Option<Max256Text>,
	#[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
	pub url_adr: Option<Max2048Text>,
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
}

impl ContactAttributes5 {
	pub fn validate(&self) -> bool {
		if !self.nm.validate() { return false }
		if let Some(ref pstl_adr_value) = self.pstl_adr { if !pstl_adr_value.validate() { return false; } }
		if let Some(ref phne_nb_value) = self.phne_nb { if !phne_nb_value.validate() { return false; } }
		if let Some(ref fax_nb_value) = self.fax_nb { if !fax_nb_value.validate() { return false; } }
		if let Some(ref email_adr_value) = self.email_adr { if !email_adr_value.validate() { return false; } }
		if let Some(ref url_adr_value) = self.url_adr { if !url_adr_value.validate() { return false; } }
		if let Some(ref any_bic_value) = self.any_bic { if !any_bic_value.validate() { return false; } }
		if let Some(ref lei_value) = self.lei { if !lei_value.validate() { return false; } }
		return true
	}
}


// ContactAttributes6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContactAttributes6 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max350Text>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress1>,
	#[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
	pub phne_nb: Option<PhoneNumber>,
	#[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
	pub fax_nb: Option<PhoneNumber>,
	#[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
	pub email_adr: Option<Max256Text>,
	#[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
	pub url_adr: Option<Max2048Text>,
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
}

impl ContactAttributes6 {
	pub fn validate(&self) -> bool {
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref pstl_adr_value) = self.pstl_adr { if !pstl_adr_value.validate() { return false; } }
		if let Some(ref phne_nb_value) = self.phne_nb { if !phne_nb_value.validate() { return false; } }
		if let Some(ref fax_nb_value) = self.fax_nb { if !fax_nb_value.validate() { return false; } }
		if let Some(ref email_adr_value) = self.email_adr { if !email_adr_value.validate() { return false; } }
		if let Some(ref url_adr_value) = self.url_adr { if !url_adr_value.validate() { return false; } }
		if let Some(ref any_bic_value) = self.any_bic { if !any_bic_value.validate() { return false; } }
		if let Some(ref lei_value) = self.lei { if !lei_value.validate() { return false; } }
		return true
	}
}


// CostsAndCharges2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CostsAndCharges2 {
	#[serde(rename = "ExAnteRefDt", skip_serializing_if = "Option::is_none")]
	pub ex_ante_ref_dt: Option<String>,
	#[serde(rename = "IndvCostOrChrg")]
	pub indv_cost_or_chrg: Vec<IndividualCostOrCharge2>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<AdditionalInformation15>,
}

impl CostsAndCharges2 {
	pub fn validate(&self) -> bool {
		for item in &self.indv_cost_or_chrg { if !item.validate() { return false; } }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if !addtl_inf_value.validate() { return false; } }
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


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DecimalNumber {
	#[serde(rename = "$value")]
	pub decimal_number: f64,
}

impl DecimalNumber {
	pub fn validate(&self) -> bool {
		return true
	}
}


// DistributionPolicy1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum DistributionPolicy1Code {
	#[default]
	#[serde(rename = "DIST")]
	CodeDIST,
	#[serde(rename = "ACCU")]
	CodeACCU,
}

impl DistributionPolicy1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// DistributionStrategy1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DistributionStrategy1 {
	#[serde(rename = "ExctnOnly", skip_serializing_if = "Option::is_none")]
	pub exctn_only: Option<DistributionStrategy1Choice>,
	#[serde(rename = "ExctnWthApprprtnssTstOrNonAdvsdSvcs", skip_serializing_if = "Option::is_none")]
	pub exctn_wth_apprprtnss_tst_or_non_advsd_svcs: Option<DistributionStrategy1Choice>,
	#[serde(rename = "InvstmtAdvc", skip_serializing_if = "Option::is_none")]
	pub invstmt_advc: Option<DistributionStrategy1Choice>,
	#[serde(rename = "PrtflMgmt", skip_serializing_if = "Option::is_none")]
	pub prtfl_mgmt: Option<DistributionStrategy1Choice>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<OtherDistributionStrategy1>,
}

impl DistributionStrategy1 {
	pub fn validate(&self) -> bool {
		if let Some(ref exctn_only_value) = self.exctn_only { if !exctn_only_value.validate() { return false; } }
		if let Some(ref exctn_wth_apprprtnss_tst_or_non_advsd_svcs_value) = self.exctn_wth_apprprtnss_tst_or_non_advsd_svcs { if !exctn_wth_apprprtnss_tst_or_non_advsd_svcs_value.validate() { return false; } }
		if let Some(ref invstmt_advc_value) = self.invstmt_advc { if !invstmt_advc_value.validate() { return false; } }
		if let Some(ref prtfl_mgmt_value) = self.prtfl_mgmt { if !prtfl_mgmt_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
	}
}


// DistributionStrategy1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DistributionStrategy1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InvestorType3Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl DistributionStrategy1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// DividendPolicy1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum DividendPolicy1Code {
	#[default]
	#[serde(rename = "CASH")]
	CodeCASH,
	#[serde(rename = "UNIT")]
	CodeUNIT,
	#[serde(rename = "BOTH")]
	CodeBOTH,
}

impl DividendPolicy1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// EMTDataReportingVFMUKType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EMTDataReportingVFMUKType1Code {
	#[default]
	#[serde(rename = "YSCO")]
	CodeYSCO,
}

impl EMTDataReportingVFMUKType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// EUSavingsDirective1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EUSavingsDirective1Code {
	#[default]
	#[serde(rename = "EUSI")]
	CodeEUSI,
	#[serde(rename = "EUSO")]
	CodeEUSO,
	#[serde(rename = "VARI")]
	CodeVARI,
}

impl EUSavingsDirective1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// EventFrequency5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EventFrequency5Code {
	#[default]
	#[serde(rename = "YEAR")]
	CodeYEAR,
	#[serde(rename = "SEMI")]
	CodeSEMI,
	#[serde(rename = "QUTR")]
	CodeQUTR,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "CLOS")]
	CodeCLOS,
	#[serde(rename = "TOMN")]
	CodeTOMN,
	#[serde(rename = "TOWK")]
	CodeTOWK,
	#[serde(rename = "TWMN")]
	CodeTWMN,
}

impl EventFrequency5Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// EventFrequency8Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EventFrequency8Code {
	#[default]
	#[serde(rename = "ADHO")]
	CodeADHO,
	#[serde(rename = "YEAR")]
	CodeYEAR,
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "FOMN")]
	CodeFOMN,
	#[serde(rename = "TOMN")]
	CodeTOMN,
	#[serde(rename = "TOWK")]
	CodeTOWK,
	#[serde(rename = "TYEA")]
	CodeTYEA,
	#[serde(rename = "INDA")]
	CodeINDA,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "ONDE")]
	CodeONDE,
	#[serde(rename = "OVNG")]
	CodeOVNG,
	#[serde(rename = "QUTR")]
	CodeQUTR,
	#[serde(rename = "SEMI")]
	CodeSEMI,
	#[serde(rename = "TWMN")]
	CodeTWMN,
	#[serde(rename = "WEEK")]
	CodeWEEK,
}

impl EventFrequency8Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ExPostCostCalculationBasis1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExPostCostCalculationBasis1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExPostCostCalculationBasis1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl ExPostCostCalculationBasis1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// ExPostCostCalculationBasis1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ExPostCostCalculationBasis1Code {
	#[default]
	#[serde(rename = "FIXB")]
	CodeFIXB,
	#[serde(rename = "ROLL")]
	CodeROLL,
}

impl ExPostCostCalculationBasis1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "$value")]
	pub exact4_alpha_numeric_text: String,
}

impl Exact4AlphaNumericText {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
		if !pattern.is_match(&self.exact4_alpha_numeric_text) {
			return false
		}
		return true
	}
}


// ExtendedParty13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExtendedParty13 {
	#[serde(rename = "PtyRole")]
	pub pty_role: GenericIdentification36,
	#[serde(rename = "OthrPtyDtls")]
	pub othr_pty_dtls: ContactAttributes5,
}

impl ExtendedParty13 {
	pub fn validate(&self) -> bool {
		if !self.pty_role.validate() { return false }
		if !self.othr_pty_dtls.validate() { return false }
		return true
	}
}


// Extension1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Extension1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Max350Text,
	#[serde(rename = "Txt")]
	pub txt: Max350Text,
}

impl Extension1 {
	pub fn validate(&self) -> bool {
		if !self.plc_and_nm.validate() { return false }
		if !self.txt.validate() { return false }
		return true
	}
}


// ExternalAccountIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalAccountIdentification1Code {
	#[serde(rename = "$value")]
	pub external_account_identification1_code: String,
}

impl ExternalAccountIdentification1Code {
	pub fn validate(&self) -> bool {
		if self.external_account_identification1_code.chars().count() < 1 {
			return false
		}
		if self.external_account_identification1_code.chars().count() > 4 {
			return false
		}
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


// FinancialInstrument96 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument96 {
	#[serde(rename = "PhysBrScties", skip_serializing_if = "Option::is_none")]
	pub phys_br_scties: Option<bool>,
	#[serde(rename = "DmtrlsdBrScties", skip_serializing_if = "Option::is_none")]
	pub dmtrlsd_br_scties: Option<bool>,
	#[serde(rename = "PhysRegdScties", skip_serializing_if = "Option::is_none")]
	pub phys_regd_scties: Option<bool>,
	#[serde(rename = "DmtrlsdRegdScties", skip_serializing_if = "Option::is_none")]
	pub dmtrlsd_regd_scties: Option<bool>,
	#[serde(rename = "DstrbtnPlcy", skip_serializing_if = "Option::is_none")]
	pub dstrbtn_plcy: Option<DistributionPolicy1Code>,
	#[serde(rename = "DvddPlcy", skip_serializing_if = "Option::is_none")]
	pub dvdd_plcy: Option<DividendPolicy1Code>,
	#[serde(rename = "DvddFrqcy", skip_serializing_if = "Option::is_none")]
	pub dvdd_frqcy: Option<EventFrequency5Code>,
	#[serde(rename = "RinvstmtFrqcy", skip_serializing_if = "Option::is_none")]
	pub rinvstmt_frqcy: Option<EventFrequency5Code>,
	#[serde(rename = "FrntEndLd", skip_serializing_if = "Option::is_none")]
	pub frnt_end_ld: Option<bool>,
	#[serde(rename = "BckEndLd", skip_serializing_if = "Option::is_none")]
	pub bck_end_ld: Option<bool>,
	#[serde(rename = "SwtchFee", skip_serializing_if = "Option::is_none")]
	pub swtch_fee: Option<bool>,
	#[serde(rename = "EUSvgsDrctv", skip_serializing_if = "Option::is_none")]
	pub eu_svgs_drctv: Option<EUSavingsDirective1Code>,
	#[serde(rename = "LnchDt", skip_serializing_if = "Option::is_none")]
	pub lnch_dt: Option<String>,
	#[serde(rename = "FndEndDt", skip_serializing_if = "Option::is_none")]
	pub fnd_end_dt: Option<String>,
	#[serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none")]
	pub termntn_dt: Option<String>,
	#[serde(rename = "InitlOfferEndDt", skip_serializing_if = "Option::is_none")]
	pub initl_offer_end_dt: Option<String>,
	#[serde(rename = "SspnsnStartDt", skip_serializing_if = "Option::is_none")]
	pub sspnsn_start_dt: Option<String>,
	#[serde(rename = "SspnsnEndDt", skip_serializing_if = "Option::is_none")]
	pub sspnsn_end_dt: Option<String>,
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "MayBeTermntdEarly", skip_serializing_if = "Option::is_none")]
	pub may_be_termntd_early: Option<TargetMarket1Code>,
	#[serde(rename = "ClsdEndFnd", skip_serializing_if = "Option::is_none")]
	pub clsd_end_fnd: Option<bool>,
	#[serde(rename = "Equlstn", skip_serializing_if = "Option::is_none")]
	pub equlstn: Option<bool>,
	#[serde(rename = "TaxEffcntPdctElgbl", skip_serializing_if = "Option::is_none")]
	pub tax_effcnt_pdct_elgbl: Option<bool>,
	#[serde(rename = "Authrsd", skip_serializing_if = "Option::is_none")]
	pub authrsd: Option<bool>,
	#[serde(rename = "RDRCmplnt", skip_serializing_if = "Option::is_none")]
	pub rdr_cmplnt: Option<bool>,
	#[serde(rename = "MgmtFeeSrc", skip_serializing_if = "Option::is_none")]
	pub mgmt_fee_src: Option<AnnualChargePaymentType1Code>,
	#[serde(rename = "PrfrmncFee", skip_serializing_if = "Option::is_none")]
	pub prfrmnc_fee: Option<bool>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}

impl FinancialInstrument96 {
	pub fn validate(&self) -> bool {
		if let Some(ref dstrbtn_plcy_value) = self.dstrbtn_plcy { if !dstrbtn_plcy_value.validate() { return false; } }
		if let Some(ref dvdd_plcy_value) = self.dvdd_plcy { if !dvdd_plcy_value.validate() { return false; } }
		if let Some(ref dvdd_frqcy_value) = self.dvdd_frqcy { if !dvdd_frqcy_value.validate() { return false; } }
		if let Some(ref rinvstmt_frqcy_value) = self.rinvstmt_frqcy { if !rinvstmt_frqcy_value.validate() { return false; } }
		if let Some(ref eu_svgs_drctv_value) = self.eu_svgs_drctv { if !eu_svgs_drctv_value.validate() { return false; } }
		if let Some(ref may_be_termntd_early_value) = self.may_be_termntd_early { if !may_be_termntd_early_value.validate() { return false; } }
		if let Some(ref mgmt_fee_src_value) = self.mgmt_fee_src { if !mgmt_fee_src_value.validate() { return false; } }
		if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if !item.validate() { return false; } } }
		return true
	}
}


// Forms1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Forms1 {
	#[serde(rename = "ApplForm")]
	pub appl_form: bool,
	#[serde(rename = "SgntrTp")]
	pub sgntr_tp: SignatureType1Code,
}

impl Forms1 {
	pub fn validate(&self) -> bool {
		if !self.sgntr_tp.validate() { return false }
		return true
	}
}


// Frequency20Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Frequency20Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<EventFrequency8Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl Frequency20Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// FundOrderType10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FundOrderType10Code {
	#[default]
	#[serde(rename = "SUBS")]
	CodeSUBS,
	#[serde(rename = "RDIV")]
	CodeRDIV,
	#[serde(rename = "REDM")]
	CodeREDM,
	#[serde(rename = "RGSV")]
	CodeRGSV,
	#[serde(rename = "WIDP")]
	CodeWIDP,
}

impl FundOrderType10Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// FundOrderType5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundOrderType5Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<FundOrderType10Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}

impl FundOrderType5Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// FundParties1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundParties1 {
	#[serde(rename = "Guarntr", skip_serializing_if = "Option::is_none")]
	pub guarntr: Option<ContactAttributes5>,
	#[serde(rename = "Audtr", skip_serializing_if = "Option::is_none")]
	pub audtr: Option<ContactAttributes5>,
	#[serde(rename = "Trstee", skip_serializing_if = "Option::is_none")]
	pub trstee: Option<ContactAttributes5>,
	#[serde(rename = "OthrPty", skip_serializing_if = "Option::is_none")]
	pub othr_pty: Option<Vec<ExtendedParty13>>,
}

impl FundParties1 {
	pub fn validate(&self) -> bool {
		if let Some(ref guarntr_value) = self.guarntr { if !guarntr_value.validate() { return false; } }
		if let Some(ref audtr_value) = self.audtr { if !audtr_value.validate() { return false; } }
		if let Some(ref trstee_value) = self.trstee { if !trstee_value.validate() { return false; } }
		if let Some(ref othr_pty_vec) = self.othr_pty { for item in othr_pty_vec { if !item.validate() { return false; } } }
		return true
	}
}


// FundPaymentType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundPaymentType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<FundPaymentType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}

impl FundPaymentType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// FundPaymentType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FundPaymentType1Code {
	#[default]
	#[serde(rename = "DRAF")]
	CodeDRAF,
	#[serde(rename = "CACC")]
	CodeCACC,
	#[serde(rename = "CHEQ")]
	CodeCHEQ,
	#[serde(rename = "CRDT")]
	CodeCRDT,
	#[serde(rename = "DDEB")]
	CodeDDEB,
	#[serde(rename = "CARD")]
	CodeCARD,
}

impl FundPaymentType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// FundReferenceDataReport5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundReferenceDataReport5 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max35Text>,
	#[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
	pub vrsn: Option<MarketPracticeVersion1>,
	#[serde(rename = "AuthrsdPrxy", skip_serializing_if = "Option::is_none")]
	pub authrsd_prxy: Option<ContactAttributes6>,
	#[serde(rename = "GnlRefDt")]
	pub gnl_ref_dt: String,
	#[serde(rename = "TrgtMktInd", skip_serializing_if = "Option::is_none")]
	pub trgt_mkt_ind: Option<bool>,
	#[serde(rename = "ExAnteInd", skip_serializing_if = "Option::is_none")]
	pub ex_ante_ind: Option<bool>,
	#[serde(rename = "ExPstInd", skip_serializing_if = "Option::is_none")]
	pub ex_pst_ind: Option<bool>,
	#[serde(rename = "SctyId")]
	pub scty_id: SecurityIdentification47,
	#[serde(rename = "FndPties", skip_serializing_if = "Option::is_none")]
	pub fnd_pties: Option<FundParties1>,
	#[serde(rename = "MainFndOrdrDsk", skip_serializing_if = "Option::is_none")]
	pub main_fnd_ordr_dsk: Option<OrderDesk1>,
	#[serde(rename = "FndMgmtCpny", skip_serializing_if = "Option::is_none")]
	pub fnd_mgmt_cpny: Option<ContactAttributes5>,
	#[serde(rename = "FndDtls", skip_serializing_if = "Option::is_none")]
	pub fnd_dtls: Option<FinancialInstrument96>,
	#[serde(rename = "ValtnDealgChrtcs", skip_serializing_if = "Option::is_none")]
	pub valtn_dealg_chrtcs: Option<ValuationDealingProcessingCharacteristics3>,
	#[serde(rename = "InvstmtRstrctns", skip_serializing_if = "Option::is_none")]
	pub invstmt_rstrctns: Option<InvestmentRestrictions3>,
	#[serde(rename = "SbcptPrcgChrtcs", skip_serializing_if = "Option::is_none")]
	pub sbcpt_prcg_chrtcs: Option<ProcessingCharacteristics11>,
	#[serde(rename = "RedPrcgChrtcs", skip_serializing_if = "Option::is_none")]
	pub red_prcg_chrtcs: Option<ProcessingCharacteristics12>,
	#[serde(rename = "SwtchPrcgChrtcs", skip_serializing_if = "Option::is_none")]
	pub swtch_prcg_chrtcs: Option<ProcessingCharacteristics9>,
	#[serde(rename = "PlanChrtcs", skip_serializing_if = "Option::is_none")]
	pub plan_chrtcs: Option<Vec<InvestmentPlanCharacteristics1>>,
	#[serde(rename = "PmtInstrm", skip_serializing_if = "Option::is_none")]
	pub pmt_instrm: Option<Vec<PaymentInstrument16>>,
	#[serde(rename = "CshSttlmDtls", skip_serializing_if = "Option::is_none")]
	pub csh_sttlm_dtls: Option<Vec<CashAccount205>>,
	#[serde(rename = "LclMktAnx", skip_serializing_if = "Option::is_none")]
	pub lcl_mkt_anx: Option<Vec<LocalMarketAnnex6>>,
	#[serde(rename = "TrgtMkt", skip_serializing_if = "Option::is_none")]
	pub trgt_mkt: Option<TargetMarket4>,
	#[serde(rename = "DstrbtnStrtgy", skip_serializing_if = "Option::is_none")]
	pub dstrbtn_strtgy: Option<DistributionStrategy1>,
	#[serde(rename = "CostsAndChrgs", skip_serializing_if = "Option::is_none")]
	pub costs_and_chrgs: Option<Vec<CostsAndCharges2>>,
	#[serde(rename = "AddtlInfUKMkt", skip_serializing_if = "Option::is_none")]
	pub addtl_inf_uk_mkt: Option<AdditionalProductInformation3>,
	#[serde(rename = "ValForMny", skip_serializing_if = "Option::is_none")]
	pub val_for_mny: Option<ValueForMoney1>,
	#[serde(rename = "Xtnsn", skip_serializing_if = "Option::is_none")]
	pub xtnsn: Option<Vec<Extension1>>,
}

impl FundReferenceDataReport5 {
	pub fn validate(&self) -> bool {
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
		if let Some(ref vrsn_value) = self.vrsn { if !vrsn_value.validate() { return false; } }
		if let Some(ref authrsd_prxy_value) = self.authrsd_prxy { if !authrsd_prxy_value.validate() { return false; } }
		if !self.scty_id.validate() { return false }
		if let Some(ref fnd_pties_value) = self.fnd_pties { if !fnd_pties_value.validate() { return false; } }
		if let Some(ref main_fnd_ordr_dsk_value) = self.main_fnd_ordr_dsk { if !main_fnd_ordr_dsk_value.validate() { return false; } }
		if let Some(ref fnd_mgmt_cpny_value) = self.fnd_mgmt_cpny { if !fnd_mgmt_cpny_value.validate() { return false; } }
		if let Some(ref fnd_dtls_value) = self.fnd_dtls { if !fnd_dtls_value.validate() { return false; } }
		if let Some(ref valtn_dealg_chrtcs_value) = self.valtn_dealg_chrtcs { if !valtn_dealg_chrtcs_value.validate() { return false; } }
		if let Some(ref invstmt_rstrctns_value) = self.invstmt_rstrctns { if !invstmt_rstrctns_value.validate() { return false; } }
		if let Some(ref sbcpt_prcg_chrtcs_value) = self.sbcpt_prcg_chrtcs { if !sbcpt_prcg_chrtcs_value.validate() { return false; } }
		if let Some(ref red_prcg_chrtcs_value) = self.red_prcg_chrtcs { if !red_prcg_chrtcs_value.validate() { return false; } }
		if let Some(ref swtch_prcg_chrtcs_value) = self.swtch_prcg_chrtcs { if !swtch_prcg_chrtcs_value.validate() { return false; } }
		if let Some(ref plan_chrtcs_vec) = self.plan_chrtcs { for item in plan_chrtcs_vec { if !item.validate() { return false; } } }
		if let Some(ref pmt_instrm_vec) = self.pmt_instrm { for item in pmt_instrm_vec { if !item.validate() { return false; } } }
		if let Some(ref csh_sttlm_dtls_vec) = self.csh_sttlm_dtls { for item in csh_sttlm_dtls_vec { if !item.validate() { return false; } } }
		if let Some(ref lcl_mkt_anx_vec) = self.lcl_mkt_anx { for item in lcl_mkt_anx_vec { if !item.validate() { return false; } } }
		if let Some(ref trgt_mkt_value) = self.trgt_mkt { if !trgt_mkt_value.validate() { return false; } }
		if let Some(ref dstrbtn_strtgy_value) = self.dstrbtn_strtgy { if !dstrbtn_strtgy_value.validate() { return false; } }
		if let Some(ref costs_and_chrgs_vec) = self.costs_and_chrgs { for item in costs_and_chrgs_vec { if !item.validate() { return false; } } }
		if let Some(ref addtl_inf_uk_mkt_value) = self.addtl_inf_uk_mkt { if !addtl_inf_uk_mkt_value.validate() { return false; } }
		if let Some(ref val_for_mny_value) = self.val_for_mny { if !val_for_mny_value.validate() { return false; } }
		if let Some(ref xtnsn_vec) = self.xtnsn { for item in xtnsn_vec { if !item.validate() { return false; } } }
		return true
	}
}


// FundReferenceDataReportV07 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundReferenceDataReportV07 {
	#[serde(rename = "MsgId")]
	pub msg_id: MessageIdentification1,
	#[serde(rename = "PrvsRef", skip_serializing_if = "Option::is_none")]
	pub prvs_ref: Option<Vec<AdditionalReference10>>,
	#[serde(rename = "RltdRef", skip_serializing_if = "Option::is_none")]
	pub rltd_ref: Option<AdditionalReference10>,
	#[serde(rename = "FndRefDataRptId", skip_serializing_if = "Option::is_none")]
	pub fnd_ref_data_rpt_id: Option<Max35Text>,
	#[serde(rename = "Rpt")]
	pub rpt: Vec<FundReferenceDataReport5>,
}

impl FundReferenceDataReportV07 {
	pub fn validate(&self) -> bool {
		if !self.msg_id.validate() { return false }
		if let Some(ref prvs_ref_vec) = self.prvs_ref { for item in prvs_ref_vec { if !item.validate() { return false; } } }
		if let Some(ref rltd_ref_value) = self.rltd_ref { if !rltd_ref_value.validate() { return false; } }
		if let Some(ref fnd_ref_data_rpt_id_value) = self.fnd_ref_data_rpt_id { if !fnd_ref_data_rpt_id_value.validate() { return false; } }
		for item in &self.rpt { if !item.validate() { return false; } }
		return true
	}
}


// GenericAccountIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericAccountIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max34Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<AccountSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericAccountIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
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


// GenericIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification3 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericIdentification3 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		return true
	}
}


// GenericIdentification36 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification36 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Issr")]
	pub issr: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
}

impl GenericIdentification36 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.issr.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		return true
	}
}


// GenericIdentification47 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification47 {
	#[serde(rename = "Id")]
	pub id: Exact4AlphaNumericText,
	#[serde(rename = "Issr")]
	pub issr: Max4AlphaNumericText,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max4AlphaNumericText>,
}

impl GenericIdentification47 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.issr.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		return true
	}
}


// GovernanceProcess1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GovernanceProcess1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<GovernanceProcessType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl GovernanceProcess1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// GovernanceProcessType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum GovernanceProcessType1Code {
	#[default]
	#[serde(rename = "BMIF")]
	CodeBMIF,
	#[serde(rename = "NINF")]
	CodeNINF,
	#[serde(rename = "CMIF")]
	CodeCMIF,
	#[serde(rename = "AMIF")]
	CodeAMIF,
}

impl GovernanceProcessType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// HoldingTransferable1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum HoldingTransferable1Code {
	#[default]
	#[serde(rename = "TRAL")]
	CodeTRAL,
	#[serde(rename = "TRNA")]
	CodeTRNA,
	#[serde(rename = "RFOD")]
	CodeRFOD,
}

impl HoldingTransferable1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// IBAN2007Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct IBAN2007Identifier {
	#[serde(rename = "$value")]
	pub iban2007_identifier: String,
}

impl IBAN2007Identifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}").unwrap();
		if !pattern.is_match(&self.iban2007_identifier) {
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


// ISOTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISOTime {
	#[serde(rename = "$value")]
	pub iso_time: String,
}

impl ISOTime {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ISOYearMonth ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISOYearMonth {
	#[serde(rename = "$value")]
	pub iso_year_month: String,
}

impl ISOYearMonth {
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


// IndividualCostOrCharge2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndividualCostOrCharge2 {
	#[serde(rename = "CostTp")]
	pub cost_tp: ChargeType8Choice,
	#[serde(rename = "ExAnteOrExPst")]
	pub ex_ante_or_ex_pst: IntendedOrActual2Code,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
	#[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
	pub sgn: Option<bool>,
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "RefPrd", skip_serializing_if = "Option::is_none")]
	pub ref_prd: Option<Period15>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<AdditionalInformation15>,
}

impl IndividualCostOrCharge2 {
	pub fn validate(&self) -> bool {
		if !self.cost_tp.validate() { return false }
		if !self.ex_ante_or_ex_pst.validate() { return false }
		if let Some(ref amt_value) = self.amt { if !amt_value.validate() { return false; } }
		if let Some(ref ref_prd_value) = self.ref_prd { if !ref_prd_value.validate() { return false; } }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if !addtl_inf_value.validate() { return false; } }
		return true
	}
}


// IntendedOrActual2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum IntendedOrActual2Code {
	#[default]
	#[serde(rename = "ANTE")]
	CodeANTE,
	#[serde(rename = "POST")]
	CodePOST,
}

impl IntendedOrActual2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// InvestmentFundMiFIDFee2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InvestmentFundMiFIDFee2Code {
	#[default]
	#[serde(rename = "BORF")]
	CodeBORF,
	#[serde(rename = "DIS2")]
	CodeDIS2,
	#[serde(rename = "FES3")]
	CodeFES3,
	#[serde(rename = "FEND")]
	CodeFEND,
	#[serde(rename = "FES2")]
	CodeFES2,
	#[serde(rename = "GOC1")]
	CodeGOC1,
	#[serde(rename = "GOCS")]
	CodeGOCS,
	#[serde(rename = "INCF")]
	CodeINCF,
	#[serde(rename = "INCS")]
	CodeINCS,
	#[serde(rename = "MNF1")]
	CodeMNF1,
	#[serde(rename = "MANS")]
	CodeMANS,
	#[serde(rename = "NET2")]
	CodeNET2,
	#[serde(rename = "NESF")]
	CodeNESF,
	#[serde(rename = "NETO")]
	CodeNETO,
	#[serde(rename = "NRAM")]
	CodeNRAM,
	#[serde(rename = "OOEA")]
	CodeOOEA,
	#[serde(rename = "OOSF")]
	CodeOOSF,
	#[serde(rename = "OOSS")]
	CodeOOSS,
	#[serde(rename = "BENS")]
	CodeBENS,
	#[serde(rename = "ENAC")]
	CodeENAC,
	#[serde(rename = "ENFX")]
	CodeENFX,
	#[serde(rename = "EXAC")]
	CodeEXAC,
	#[serde(rename = "ENBX")]
	CodeENBX,
	#[serde(rename = "BEND")]
	CodeBEND,
	#[serde(rename = "PENO")]
	CodePENO,
	#[serde(rename = "OTES")]
	CodeOTES,
	#[serde(rename = "OCAS")]
	CodeOCAS,
	#[serde(rename = "RPSS")]
	CodeRPSS,
	#[serde(rename = "TRS1")]
	CodeTRS1,
}

impl InvestmentFundMiFIDFee2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// InvestmentFundPlanType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentFundPlanType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InvestmentFundPlanType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}

impl InvestmentFundPlanType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// InvestmentFundPlanType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InvestmentFundPlanType1Code {
	#[default]
	#[serde(rename = "INVP")]
	CodeINVP,
	#[serde(rename = "SWIP")]
	CodeSWIP,
	#[serde(rename = "WTHP")]
	CodeWTHP,
}

impl InvestmentFundPlanType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// InvestmentNeed2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentNeed2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InvestmentNeed2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl InvestmentNeed2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// InvestmentNeed2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InvestmentNeed2Code {
	#[default]
	#[serde(rename = "NSPE")]
	CodeNSPE,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "ISLB")]
	CodeISLB,
}

impl InvestmentNeed2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// InvestmentPlanCharacteristics1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentPlanCharacteristics1 {
	#[serde(rename = "PlanTp")]
	pub plan_tp: InvestmentFundPlanType1Choice,
	#[serde(rename = "Frqcy", skip_serializing_if = "Option::is_none")]
	pub frqcy: Option<Frequency20Choice>,
	#[serde(rename = "TtlNbOfInstlmts", skip_serializing_if = "Option::is_none")]
	pub ttl_nb_of_instlmts: Option<f64>,
	#[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
	pub qty: Option<UnitsOrAmount1Choice>,
	#[serde(rename = "PlanConttn", skip_serializing_if = "Option::is_none")]
	pub plan_conttn: Option<bool>,
	#[serde(rename = "AddtlSbcpt", skip_serializing_if = "Option::is_none")]
	pub addtl_sbcpt: Option<bool>,
	#[serde(rename = "AddtlSbcptFctn", skip_serializing_if = "Option::is_none")]
	pub addtl_sbcpt_fctn: Option<bool>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}

impl InvestmentPlanCharacteristics1 {
	pub fn validate(&self) -> bool {
		if !self.plan_tp.validate() { return false }
		if let Some(ref frqcy_value) = self.frqcy { if !frqcy_value.validate() { return false; } }
		if let Some(ref qty_value) = self.qty { if !qty_value.validate() { return false; } }
		if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if !item.validate() { return false; } } }
		return true
	}
}


// InvestmentRestrictions3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentRestrictions3 {
	#[serde(rename = "MinInitlSbcptAmt", skip_serializing_if = "Option::is_none")]
	pub min_initl_sbcpt_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "MinInitlSbcptUnits", skip_serializing_if = "Option::is_none")]
	pub min_initl_sbcpt_units: Option<f64>,
	#[serde(rename = "MinSbsqntSbcptAmt", skip_serializing_if = "Option::is_none")]
	pub min_sbsqnt_sbcpt_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "MinSbsqntSbcptUnits", skip_serializing_if = "Option::is_none")]
	pub min_sbsqnt_sbcpt_units: Option<f64>,
	#[serde(rename = "MaxRedAmt", skip_serializing_if = "Option::is_none")]
	pub max_red_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "MaxRedUnits", skip_serializing_if = "Option::is_none")]
	pub max_red_units: Option<f64>,
	#[serde(rename = "MinRedPctg", skip_serializing_if = "Option::is_none")]
	pub min_red_pctg: Option<f64>,
	#[serde(rename = "OthrRedRstrctns", skip_serializing_if = "Option::is_none")]
	pub othr_red_rstrctns: Option<Max350Text>,
	#[serde(rename = "MinSwtchSbcptAmt", skip_serializing_if = "Option::is_none")]
	pub min_swtch_sbcpt_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "MinSwtchSbcptUnits", skip_serializing_if = "Option::is_none")]
	pub min_swtch_sbcpt_units: Option<f64>,
	#[serde(rename = "MaxSwtchRedAmt", skip_serializing_if = "Option::is_none")]
	pub max_swtch_red_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "MaxSwtchRedUnits", skip_serializing_if = "Option::is_none")]
	pub max_swtch_red_units: Option<f64>,
	#[serde(rename = "OthrSwtchRstrctns", skip_serializing_if = "Option::is_none")]
	pub othr_swtch_rstrctns: Option<Max350Text>,
	#[serde(rename = "MinHldgAmt", skip_serializing_if = "Option::is_none")]
	pub min_hldg_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "MinHldgUnits", skip_serializing_if = "Option::is_none")]
	pub min_hldg_units: Option<f64>,
	#[serde(rename = "MinHldgPrd", skip_serializing_if = "Option::is_none")]
	pub min_hldg_prd: Option<Max70Text>,
	#[serde(rename = "HldgTrfbl", skip_serializing_if = "Option::is_none")]
	pub hldg_trfbl: Option<HoldingTransferable1Code>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}

impl InvestmentRestrictions3 {
	pub fn validate(&self) -> bool {
		if let Some(ref min_initl_sbcpt_amt_value) = self.min_initl_sbcpt_amt { if !min_initl_sbcpt_amt_value.validate() { return false; } }
		if let Some(ref min_sbsqnt_sbcpt_amt_value) = self.min_sbsqnt_sbcpt_amt { if !min_sbsqnt_sbcpt_amt_value.validate() { return false; } }
		if let Some(ref max_red_amt_value) = self.max_red_amt { if !max_red_amt_value.validate() { return false; } }
		if let Some(ref othr_red_rstrctns_value) = self.othr_red_rstrctns { if !othr_red_rstrctns_value.validate() { return false; } }
		if let Some(ref min_swtch_sbcpt_amt_value) = self.min_swtch_sbcpt_amt { if !min_swtch_sbcpt_amt_value.validate() { return false; } }
		if let Some(ref max_swtch_red_amt_value) = self.max_swtch_red_amt { if !max_swtch_red_amt_value.validate() { return false; } }
		if let Some(ref othr_swtch_rstrctns_value) = self.othr_swtch_rstrctns { if !othr_swtch_rstrctns_value.validate() { return false; } }
		if let Some(ref min_hldg_amt_value) = self.min_hldg_amt { if !min_hldg_amt_value.validate() { return false; } }
		if let Some(ref min_hldg_prd_value) = self.min_hldg_prd { if !min_hldg_prd_value.validate() { return false; } }
		if let Some(ref hldg_trfbl_value) = self.hldg_trfbl { if !hldg_trfbl_value.validate() { return false; } }
		if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if !item.validate() { return false; } } }
		return true
	}
}


// InvestorKnowledge1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestorKnowledge1 {
	#[serde(rename = "BsicInvstr", skip_serializing_if = "Option::is_none")]
	pub bsic_invstr: Option<TargetMarket1Code>,
	#[serde(rename = "InfrmdInvstr", skip_serializing_if = "Option::is_none")]
	pub infrmd_invstr: Option<TargetMarket1Code>,
	#[serde(rename = "AdvncdInvstr", skip_serializing_if = "Option::is_none")]
	pub advncd_invstr: Option<TargetMarket1Code>,
	#[serde(rename = "ExprtInvstrDE", skip_serializing_if = "Option::is_none")]
	pub exprt_invstr_de: Option<TargetMarket1Code>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<OtherTargetMarketInvestorKnowledge1>>,
}

impl InvestorKnowledge1 {
	pub fn validate(&self) -> bool {
		if let Some(ref bsic_invstr_value) = self.bsic_invstr { if !bsic_invstr_value.validate() { return false; } }
		if let Some(ref infrmd_invstr_value) = self.infrmd_invstr { if !infrmd_invstr_value.validate() { return false; } }
		if let Some(ref advncd_invstr_value) = self.advncd_invstr { if !advncd_invstr_value.validate() { return false; } }
		if let Some(ref exprt_invstr_de_value) = self.exprt_invstr_de { if !exprt_invstr_de_value.validate() { return false; } }
		if let Some(ref othr_vec) = self.othr { for item in othr_vec { if !item.validate() { return false; } } }
		return true
	}
}


// InvestorRequirements4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestorRequirements4 {
	#[serde(rename = "RtrPrflPrsrvtn", skip_serializing_if = "Option::is_none")]
	pub rtr_prfl_prsrvtn: Option<TargetMarket1Code>,
	#[serde(rename = "RtrPrflGrwth", skip_serializing_if = "Option::is_none")]
	pub rtr_prfl_grwth: Option<TargetMarket1Code>,
	#[serde(rename = "RtrPrflIncm", skip_serializing_if = "Option::is_none")]
	pub rtr_prfl_incm: Option<TargetMarket1Code>,
	#[serde(rename = "RtrPrflHdgg", skip_serializing_if = "Option::is_none")]
	pub rtr_prfl_hdgg: Option<TargetMarket1Code>,
	#[serde(rename = "OptnOrLvrgdRtrPrfl", skip_serializing_if = "Option::is_none")]
	pub optn_or_lvrgd_rtr_prfl: Option<TargetMarket1Code>,
	#[serde(rename = "RtrPrflPnsnSchmeDE", skip_serializing_if = "Option::is_none")]
	pub rtr_prfl_pnsn_schme_de: Option<TargetMarket1Code>,
	#[serde(rename = "MinHldgPrd", skip_serializing_if = "Option::is_none")]
	pub min_hldg_prd: Option<TimeHorizon2Choice>,
	#[serde(rename = "SstnbltyPrefs", skip_serializing_if = "Option::is_none")]
	pub sstnblty_prefs: Option<SustainabilityPreferences2Code>,
	#[serde(rename = "OthrSpcfcInvstmtNeed", skip_serializing_if = "Option::is_none")]
	pub othr_spcfc_invstmt_need: Option<InvestmentNeed2Choice>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<OtherInvestmentNeed1>>,
}

impl InvestorRequirements4 {
	pub fn validate(&self) -> bool {
		if let Some(ref rtr_prfl_prsrvtn_value) = self.rtr_prfl_prsrvtn { if !rtr_prfl_prsrvtn_value.validate() { return false; } }
		if let Some(ref rtr_prfl_grwth_value) = self.rtr_prfl_grwth { if !rtr_prfl_grwth_value.validate() { return false; } }
		if let Some(ref rtr_prfl_incm_value) = self.rtr_prfl_incm { if !rtr_prfl_incm_value.validate() { return false; } }
		if let Some(ref rtr_prfl_hdgg_value) = self.rtr_prfl_hdgg { if !rtr_prfl_hdgg_value.validate() { return false; } }
		if let Some(ref optn_or_lvrgd_rtr_prfl_value) = self.optn_or_lvrgd_rtr_prfl { if !optn_or_lvrgd_rtr_prfl_value.validate() { return false; } }
		if let Some(ref rtr_prfl_pnsn_schme_de_value) = self.rtr_prfl_pnsn_schme_de { if !rtr_prfl_pnsn_schme_de_value.validate() { return false; } }
		if let Some(ref min_hldg_prd_value) = self.min_hldg_prd { if !min_hldg_prd_value.validate() { return false; } }
		if let Some(ref sstnblty_prefs_value) = self.sstnblty_prefs { if !sstnblty_prefs_value.validate() { return false; } }
		if let Some(ref othr_spcfc_invstmt_need_value) = self.othr_spcfc_invstmt_need { if !othr_spcfc_invstmt_need_value.validate() { return false; } }
		if let Some(ref othr_vec) = self.othr { for item in othr_vec { if !item.validate() { return false; } } }
		return true
	}
}


// InvestorType2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestorType2 {
	#[serde(rename = "InvstrTpRtl", skip_serializing_if = "Option::is_none")]
	pub invstr_tp_rtl: Option<TargetMarket1Code>,
	#[serde(rename = "InvstrTpPrfssnl", skip_serializing_if = "Option::is_none")]
	pub invstr_tp_prfssnl: Option<TargetMarket5Choice>,
	#[serde(rename = "InvstrTpElgblCtrPty", skip_serializing_if = "Option::is_none")]
	pub invstr_tp_elgbl_ctr_pty: Option<TargetMarket3Code>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<OtherTargetMarketInvestor1>>,
}

impl InvestorType2 {
	pub fn validate(&self) -> bool {
		if let Some(ref invstr_tp_rtl_value) = self.invstr_tp_rtl { if !invstr_tp_rtl_value.validate() { return false; } }
		if let Some(ref invstr_tp_prfssnl_value) = self.invstr_tp_prfssnl { if !invstr_tp_prfssnl_value.validate() { return false; } }
		if let Some(ref invstr_tp_elgbl_ctr_pty_value) = self.invstr_tp_elgbl_ctr_pty { if !invstr_tp_elgbl_ctr_pty_value.validate() { return false; } }
		if let Some(ref othr_vec) = self.othr { for item in othr_vec { if !item.validate() { return false; } } }
		return true
	}
}


// InvestorType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InvestorType2Code {
	#[default]
	#[serde(rename = "BOT3")]
	CodeBOT3,
	#[serde(rename = "EPRO")]
	CodeEPRO,
	#[serde(rename = "PRF2")]
	CodePRF2,
}

impl InvestorType2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// InvestorType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InvestorType3Code {
	#[default]
	#[serde(rename = "RETL")]
	CodeRETL,
	#[serde(rename = "PRF2")]
	CodePRF2,
	#[serde(rename = "NEI1")]
	CodeNEI1,
	#[serde(rename = "BOT2")]
	CodeBOT2,
}

impl InvestorType3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// InvestorType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InvestorType4Code {
	#[default]
	#[serde(rename = "BOT3")]
	CodeBOT3,
	#[serde(rename = "NPRF")]
	CodeNPRF,
	#[serde(rename = "PRF3")]
	CodePRF3,
	#[serde(rename = "PRF4")]
	CodePRF4,
}

impl InvestorType4Code {
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


// LocalMarketAnnex6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LocalMarketAnnex6 {
	#[serde(rename = "Ctry")]
	pub ctry: Vec<CountryCode>,
	#[serde(rename = "LclOrdrDsk")]
	pub lcl_ordr_dsk: OrderDesk1,
	#[serde(rename = "SbcptPrcgChrtcs", skip_serializing_if = "Option::is_none")]
	pub sbcpt_prcg_chrtcs: Option<ProcessingCharacteristics11>,
	#[serde(rename = "RedPrcgChrtcs", skip_serializing_if = "Option::is_none")]
	pub red_prcg_chrtcs: Option<ProcessingCharacteristics10>,
	#[serde(rename = "SwtchPrcgChrtcs", skip_serializing_if = "Option::is_none")]
	pub swtch_prcg_chrtcs: Option<ProcessingCharacteristics9>,
	#[serde(rename = "CshSttlmDtls", skip_serializing_if = "Option::is_none")]
	pub csh_sttlm_dtls: Option<Vec<CashAccount205>>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}

impl LocalMarketAnnex6 {
	pub fn validate(&self) -> bool {
		for item in &self.ctry { if !item.validate() { return false; } }
		if !self.lcl_ordr_dsk.validate() { return false }
		if let Some(ref sbcpt_prcg_chrtcs_value) = self.sbcpt_prcg_chrtcs { if !sbcpt_prcg_chrtcs_value.validate() { return false; } }
		if let Some(ref red_prcg_chrtcs_value) = self.red_prcg_chrtcs { if !red_prcg_chrtcs_value.validate() { return false; } }
		if let Some(ref swtch_prcg_chrtcs_value) = self.swtch_prcg_chrtcs { if !swtch_prcg_chrtcs_value.validate() { return false; } }
		if let Some(ref csh_sttlm_dtls_vec) = self.csh_sttlm_dtls { for item in csh_sttlm_dtls_vec { if !item.validate() { return false; } } }
		if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if !item.validate() { return false; } } }
		return true
	}
}


// LossBearing2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LossBearing2 {
	#[serde(rename = "NoCptlLoss", skip_serializing_if = "Option::is_none")]
	pub no_cptl_loss: Option<TargetMarket1Code>,
	#[serde(rename = "LtdCptlLoss", skip_serializing_if = "Option::is_none")]
	pub ltd_cptl_loss: Option<TargetMarket1Code>,
	#[serde(rename = "LtdCptlLossLvl", skip_serializing_if = "Option::is_none")]
	pub ltd_cptl_loss_lvl: Option<f64>,
	#[serde(rename = "NoCptlGrnt", skip_serializing_if = "Option::is_none")]
	pub no_cptl_grnt: Option<TargetMarket1Code>,
	#[serde(rename = "LossByndCptl", skip_serializing_if = "Option::is_none")]
	pub loss_bynd_cptl: Option<TargetMarket1Code>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<OtherTargetMarketLossBearing1>>,
}

impl LossBearing2 {
	pub fn validate(&self) -> bool {
		if let Some(ref no_cptl_loss_value) = self.no_cptl_loss { if !no_cptl_loss_value.validate() { return false; } }
		if let Some(ref ltd_cptl_loss_value) = self.ltd_cptl_loss { if !ltd_cptl_loss_value.validate() { return false; } }
		if let Some(ref no_cptl_grnt_value) = self.no_cptl_grnt { if !no_cptl_grnt_value.validate() { return false; } }
		if let Some(ref loss_bynd_cptl_value) = self.loss_bynd_cptl { if !loss_bynd_cptl_value.validate() { return false; } }
		if let Some(ref othr_vec) = self.othr { for item in othr_vec { if !item.validate() { return false; } } }
		return true
	}
}


// MainFundOrderDeskLocation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MainFundOrderDeskLocation1 {
	#[serde(rename = "Ctry")]
	pub ctry: CountryCode,
	#[serde(rename = "TmZoneOffSet")]
	pub tm_zone_off_set: UTCOffset1,
}

impl MainFundOrderDeskLocation1 {
	pub fn validate(&self) -> bool {
		if !self.ctry.validate() { return false }
		if !self.tm_zone_off_set.validate() { return false }
		return true
	}
}


// MarketPracticeVersion1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketPracticeVersion1 {
	#[serde(rename = "Nm")]
	pub nm: Max35Text,
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
	pub nb: Option<Max35Text>,
}

impl MarketPracticeVersion1 {
	pub fn validate(&self) -> bool {
		if !self.nm.validate() { return false }
		if let Some(ref nb_value) = self.nb { if !nb_value.validate() { return false; } }
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


// Max1Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max1Number {
	#[serde(rename = "$value")]
	pub max1_number: f64,
}

impl Max1Number {
	pub fn validate(&self) -> bool {
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


// Max34Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max34Text {
	#[serde(rename = "$value")]
	pub max34_text: String,
}

impl Max34Text {
	pub fn validate(&self) -> bool {
		if self.max34_text.chars().count() < 1 {
			return false
		}
		if self.max34_text.chars().count() > 34 {
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


// Max4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "$value")]
	pub max4_alpha_numeric_text: String,
}

impl Max4AlphaNumericText {
	pub fn validate(&self) -> bool {
		if self.max4_alpha_numeric_text.chars().count() < 1 {
			return false
		}
		if self.max4_alpha_numeric_text.chars().count() > 4 {
			return false
		}
		let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
		if !pattern.is_match(&self.max4_alpha_numeric_text) {
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


// MessageIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
}

impl MessageIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
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


// NotionalOrUnitBased1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalOrUnitBased1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<NotionalOrUnitBased1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl NotionalOrUnitBased1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// NotionalOrUnitBased1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NotionalOrUnitBased1Code {
	#[default]
	#[serde(rename = "UNIT")]
	CodeUNIT,
	#[serde(rename = "NOTI")]
	CodeNOTI,
}

impl NotionalOrUnitBased1Code {
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


// OrderDesk1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrderDesk1 {
	#[serde(rename = "OrdrDsk", skip_serializing_if = "Option::is_none")]
	pub ordr_dsk: Option<ContactAttributes5>,
	#[serde(rename = "ClsrDts", skip_serializing_if = "Option::is_none")]
	pub clsr_dts: Option<Vec<String>>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}

impl OrderDesk1 {
	pub fn validate(&self) -> bool {
		if let Some(ref ordr_dsk_value) = self.ordr_dsk { if !ordr_dsk_value.validate() { return false; } }
		if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if !item.validate() { return false; } } }
		return true
	}
}


// OtherDistributionStrategy1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherDistributionStrategy1 {
	#[serde(rename = "DstrbtnStrtgyTp", skip_serializing_if = "Option::is_none")]
	pub dstrbtn_strtgy_tp: Option<Max35Text>,
	#[serde(rename = "Trgt", skip_serializing_if = "Option::is_none")]
	pub trgt: Option<DistributionStrategy1Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<AdditionalInformation15>,
}

impl OtherDistributionStrategy1 {
	pub fn validate(&self) -> bool {
		if let Some(ref dstrbtn_strtgy_tp_value) = self.dstrbtn_strtgy_tp { if !dstrbtn_strtgy_tp_value.validate() { return false; } }
		if let Some(ref trgt_value) = self.trgt { if !trgt_value.validate() { return false; } }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if !addtl_inf_value.validate() { return false; } }
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


// OtherInvestmentNeed1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherInvestmentNeed1 {
	#[serde(rename = "ClntObjctvsAndNeedsTp", skip_serializing_if = "Option::is_none")]
	pub clnt_objctvs_and_needs_tp: Option<Max35Text>,
	#[serde(rename = "Trgt", skip_serializing_if = "Option::is_none")]
	pub trgt: Option<TargetMarket1Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<AdditionalInformation15>,
}

impl OtherInvestmentNeed1 {
	pub fn validate(&self) -> bool {
		if let Some(ref clnt_objctvs_and_needs_tp_value) = self.clnt_objctvs_and_needs_tp { if !clnt_objctvs_and_needs_tp_value.validate() { return false; } }
		if let Some(ref trgt_value) = self.trgt { if !trgt_value.validate() { return false; } }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if !addtl_inf_value.validate() { return false; } }
		return true
	}
}


// OtherReviewRelatedToValueAndOrChargesUKType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OtherReviewRelatedToValueAndOrChargesUKType1Code {
	#[default]
	#[serde(rename = "REVA")]
	CodeREVA,
	#[serde(rename = "REVO")]
	CodeREVO,
}

impl OtherReviewRelatedToValueAndOrChargesUKType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// OtherTargetMarket1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherTargetMarket1 {
	#[serde(rename = "TrgtMktTp")]
	pub trgt_mkt_tp: Max350Text,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<AdditionalInformation15>,
}

impl OtherTargetMarket1 {
	pub fn validate(&self) -> bool {
		if !self.trgt_mkt_tp.validate() { return false }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if !addtl_inf_value.validate() { return false; } }
		return true
	}
}


// OtherTargetMarketInvestor1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherTargetMarketInvestor1 {
	#[serde(rename = "InvstrTp", skip_serializing_if = "Option::is_none")]
	pub invstr_tp: Option<Max35Text>,
	#[serde(rename = "Trgt", skip_serializing_if = "Option::is_none")]
	pub trgt: Option<TargetMarket3Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<AdditionalInformation15>,
}

impl OtherTargetMarketInvestor1 {
	pub fn validate(&self) -> bool {
		if let Some(ref invstr_tp_value) = self.invstr_tp { if !invstr_tp_value.validate() { return false; } }
		if let Some(ref trgt_value) = self.trgt { if !trgt_value.validate() { return false; } }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if !addtl_inf_value.validate() { return false; } }
		return true
	}
}


// OtherTargetMarketInvestorKnowledge1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherTargetMarketInvestorKnowledge1 {
	#[serde(rename = "InvstrKnwldgTp", skip_serializing_if = "Option::is_none")]
	pub invstr_knwldg_tp: Option<Max35Text>,
	#[serde(rename = "Trgt", skip_serializing_if = "Option::is_none")]
	pub trgt: Option<TargetMarket1Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<AdditionalInformation15>,
}

impl OtherTargetMarketInvestorKnowledge1 {
	pub fn validate(&self) -> bool {
		if let Some(ref invstr_knwldg_tp_value) = self.invstr_knwldg_tp { if !invstr_knwldg_tp_value.validate() { return false; } }
		if let Some(ref trgt_value) = self.trgt { if !trgt_value.validate() { return false; } }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if !addtl_inf_value.validate() { return false; } }
		return true
	}
}


// OtherTargetMarketLossBearing1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherTargetMarketLossBearing1 {
	#[serde(rename = "AbltyToBearLossesTp", skip_serializing_if = "Option::is_none")]
	pub ablty_to_bear_losses_tp: Option<Max35Text>,
	#[serde(rename = "Trgt", skip_serializing_if = "Option::is_none")]
	pub trgt: Option<TargetMarket1Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<AdditionalInformation15>,
}

impl OtherTargetMarketLossBearing1 {
	pub fn validate(&self) -> bool {
		if let Some(ref ablty_to_bear_losses_tp_value) = self.ablty_to_bear_losses_tp { if !ablty_to_bear_losses_tp_value.validate() { return false; } }
		if let Some(ref trgt_value) = self.trgt { if !trgt_value.validate() { return false; } }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if !addtl_inf_value.validate() { return false; } }
		return true
	}
}


// OtherTargetMarketRiskTolerance1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherTargetMarketRiskTolerance1 {
	#[serde(rename = "RskTlrnceTp", skip_serializing_if = "Option::is_none")]
	pub rsk_tlrnce_tp: Option<Max35Text>,
	#[serde(rename = "Trgt", skip_serializing_if = "Option::is_none")]
	pub trgt: Option<TargetMarket1Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<AdditionalInformation15>,
}

impl OtherTargetMarketRiskTolerance1 {
	pub fn validate(&self) -> bool {
		if let Some(ref rsk_tlrnce_tp_value) = self.rsk_tlrnce_tp { if !rsk_tlrnce_tp_value.validate() { return false; } }
		if let Some(ref trgt_value) = self.trgt { if !trgt_value.validate() { return false; } }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if !addtl_inf_value.validate() { return false; } }
		return true
	}
}


// OutcomeOfCOLLAssessmentOfValueUKType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OutcomeOfCOLLAssessmentOfValueUKType1Code {
	#[default]
	#[serde(rename = "COL1")]
	CodeCOL1,
	#[serde(rename = "COL2")]
	CodeCOL2,
}

impl OutcomeOfCOLLAssessmentOfValueUKType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// OutcomeOfPRINValueAssessmentOrReviewUKType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OutcomeOfPRINValueAssessmentOrReviewUKType1Code {
	#[default]
	#[serde(rename = "PRI2")]
	CodePRI2,
	#[serde(rename = "PRI1")]
	CodePRI1,
}

impl OutcomeOfPRINValueAssessmentOrReviewUKType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// PartyIdentification125Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification125Choice {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification1>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
}

impl PartyIdentification125Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref any_bic_value) = self.any_bic { if !any_bic_value.validate() { return false; } }
		if let Some(ref prtry_id_value) = self.prtry_id { if !prtry_id_value.validate() { return false; } }
		if let Some(ref nm_and_adr_value) = self.nm_and_adr { if !nm_and_adr_value.validate() { return false; } }
		return true
	}
}


// PartyIdentification139 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification139 {
	#[serde(rename = "Pty")]
	pub pty: PartyIdentification125Choice,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
}

impl PartyIdentification139 {
	pub fn validate(&self) -> bool {
		if !self.pty.validate() { return false }
		if let Some(ref lei_value) = self.lei { if !lei_value.validate() { return false; } }
		return true
	}
}


// PaymentInstrument16 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentInstrument16 {
	#[serde(rename = "OrdrTp")]
	pub ordr_tp: FundOrderType5Choice,
	#[serde(rename = "InstrmTp")]
	pub instrm_tp: FundPaymentType1Choice,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}

impl PaymentInstrument16 {
	pub fn validate(&self) -> bool {
		if !self.ordr_tp.validate() { return false }
		if !self.instrm_tp.validate() { return false }
		if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if !item.validate() { return false; } } }
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


// Period15 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period15 {
	#[serde(rename = "StartDt")]
	pub start_dt: String,
	#[serde(rename = "EndDt")]
	pub end_dt: String,
}

impl Period15 {
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


// PlusOrMinusIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "$value")]
	pub plus_or_minus_indicator: bool,
}

impl PlusOrMinusIndicator {
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


// PriceMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PriceMethod1Code {
	#[default]
	#[serde(rename = "FORW")]
	CodeFORW,
	#[serde(rename = "HIST")]
	CodeHIST,
}

impl PriceMethod1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ProcessingCharacteristics10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProcessingCharacteristics10 {
	#[serde(rename = "DealgCcyAccptd", skip_serializing_if = "Option::is_none")]
	pub dealg_ccy_accptd: Option<Vec<ActiveCurrencyCode>>,
	#[serde(rename = "RedAuthstn", skip_serializing_if = "Option::is_none")]
	pub red_authstn: Option<Forms1>,
	#[serde(rename = "AmtInd", skip_serializing_if = "Option::is_none")]
	pub amt_ind: Option<bool>,
	#[serde(rename = "UnitsInd", skip_serializing_if = "Option::is_none")]
	pub units_ind: Option<bool>,
	#[serde(rename = "Rndg", skip_serializing_if = "Option::is_none")]
	pub rndg: Option<RoundingDirection2Code>,
	#[serde(rename = "PctgInd", skip_serializing_if = "Option::is_none")]
	pub pctg_ind: Option<bool>,
	#[serde(rename = "MainFndOrdrDskLctn", skip_serializing_if = "Option::is_none")]
	pub main_fnd_ordr_dsk_lctn: Option<MainFundOrderDeskLocation1>,
	#[serde(rename = "DealgFrqcy", skip_serializing_if = "Option::is_none")]
	pub dealg_frqcy: Option<EventFrequency5Code>,
	#[serde(rename = "DealgFrqcyDesc", skip_serializing_if = "Option::is_none")]
	pub dealg_frqcy_desc: Option<Max350Text>,
	#[serde(rename = "DealgCutOffTm", skip_serializing_if = "Option::is_none")]
	pub dealg_cut_off_tm: Option<String>,
	#[serde(rename = "DealgCutOffTmFrame", skip_serializing_if = "Option::is_none")]
	pub dealg_cut_off_tm_frame: Option<TimeFrame9>,
	#[serde(rename = "DealConfTm", skip_serializing_if = "Option::is_none")]
	pub deal_conf_tm: Option<String>,
	#[serde(rename = "DealConfTmFrame", skip_serializing_if = "Option::is_none")]
	pub deal_conf_tm_frame: Option<TimeFrame8>,
	#[serde(rename = "LtdPrd", skip_serializing_if = "Option::is_none")]
	pub ltd_prd: Option<Max350Text>,
	#[serde(rename = "SttlmCycl", skip_serializing_if = "Option::is_none")]
	pub sttlm_cycl: Option<TimeFrame8Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}

impl ProcessingCharacteristics10 {
	pub fn validate(&self) -> bool {
		if let Some(ref dealg_ccy_accptd_vec) = self.dealg_ccy_accptd { for item in dealg_ccy_accptd_vec { if !item.validate() { return false; } } }
		if let Some(ref red_authstn_value) = self.red_authstn { if !red_authstn_value.validate() { return false; } }
		if let Some(ref rndg_value) = self.rndg { if !rndg_value.validate() { return false; } }
		if let Some(ref main_fnd_ordr_dsk_lctn_value) = self.main_fnd_ordr_dsk_lctn { if !main_fnd_ordr_dsk_lctn_value.validate() { return false; } }
		if let Some(ref dealg_frqcy_value) = self.dealg_frqcy { if !dealg_frqcy_value.validate() { return false; } }
		if let Some(ref dealg_frqcy_desc_value) = self.dealg_frqcy_desc { if !dealg_frqcy_desc_value.validate() { return false; } }
		if let Some(ref dealg_cut_off_tm_frame_value) = self.dealg_cut_off_tm_frame { if !dealg_cut_off_tm_frame_value.validate() { return false; } }
		if let Some(ref deal_conf_tm_frame_value) = self.deal_conf_tm_frame { if !deal_conf_tm_frame_value.validate() { return false; } }
		if let Some(ref ltd_prd_value) = self.ltd_prd { if !ltd_prd_value.validate() { return false; } }
		if let Some(ref sttlm_cycl_value) = self.sttlm_cycl { if !sttlm_cycl_value.validate() { return false; } }
		if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if !item.validate() { return false; } } }
		return true
	}
}


// ProcessingCharacteristics11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProcessingCharacteristics11 {
	#[serde(rename = "DealgCcyAccptd", skip_serializing_if = "Option::is_none")]
	pub dealg_ccy_accptd: Option<Vec<ActiveCurrencyCode>>,
	#[serde(rename = "InitlInvstmtAppl", skip_serializing_if = "Option::is_none")]
	pub initl_invstmt_appl: Option<Forms1>,
	#[serde(rename = "SbsqntInvstmtAppl", skip_serializing_if = "Option::is_none")]
	pub sbsqnt_invstmt_appl: Option<Forms1>,
	#[serde(rename = "AmtInd", skip_serializing_if = "Option::is_none")]
	pub amt_ind: Option<bool>,
	#[serde(rename = "UnitsInd", skip_serializing_if = "Option::is_none")]
	pub units_ind: Option<bool>,
	#[serde(rename = "Rndg", skip_serializing_if = "Option::is_none")]
	pub rndg: Option<RoundingDirection2Code>,
	#[serde(rename = "MainFndOrdrDskLctn", skip_serializing_if = "Option::is_none")]
	pub main_fnd_ordr_dsk_lctn: Option<MainFundOrderDeskLocation1>,
	#[serde(rename = "DealgFrqcy", skip_serializing_if = "Option::is_none")]
	pub dealg_frqcy: Option<EventFrequency5Code>,
	#[serde(rename = "DealgFrqcyDesc", skip_serializing_if = "Option::is_none")]
	pub dealg_frqcy_desc: Option<Max350Text>,
	#[serde(rename = "DealgCutOffTm", skip_serializing_if = "Option::is_none")]
	pub dealg_cut_off_tm: Option<String>,
	#[serde(rename = "DealgCutOffTmFrame", skip_serializing_if = "Option::is_none")]
	pub dealg_cut_off_tm_frame: Option<TimeFrame9>,
	#[serde(rename = "DealConfTm", skip_serializing_if = "Option::is_none")]
	pub deal_conf_tm: Option<String>,
	#[serde(rename = "DealConfTmFrame", skip_serializing_if = "Option::is_none")]
	pub deal_conf_tm_frame: Option<TimeFrame11>,
	#[serde(rename = "LtdPrd", skip_serializing_if = "Option::is_none")]
	pub ltd_prd: Option<Max350Text>,
	#[serde(rename = "SttlmCycl", skip_serializing_if = "Option::is_none")]
	pub sttlm_cycl: Option<TimeFrame7Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}

impl ProcessingCharacteristics11 {
	pub fn validate(&self) -> bool {
		if let Some(ref dealg_ccy_accptd_vec) = self.dealg_ccy_accptd { for item in dealg_ccy_accptd_vec { if !item.validate() { return false; } } }
		if let Some(ref initl_invstmt_appl_value) = self.initl_invstmt_appl { if !initl_invstmt_appl_value.validate() { return false; } }
		if let Some(ref sbsqnt_invstmt_appl_value) = self.sbsqnt_invstmt_appl { if !sbsqnt_invstmt_appl_value.validate() { return false; } }
		if let Some(ref rndg_value) = self.rndg { if !rndg_value.validate() { return false; } }
		if let Some(ref main_fnd_ordr_dsk_lctn_value) = self.main_fnd_ordr_dsk_lctn { if !main_fnd_ordr_dsk_lctn_value.validate() { return false; } }
		if let Some(ref dealg_frqcy_value) = self.dealg_frqcy { if !dealg_frqcy_value.validate() { return false; } }
		if let Some(ref dealg_frqcy_desc_value) = self.dealg_frqcy_desc { if !dealg_frqcy_desc_value.validate() { return false; } }
		if let Some(ref dealg_cut_off_tm_frame_value) = self.dealg_cut_off_tm_frame { if !dealg_cut_off_tm_frame_value.validate() { return false; } }
		if let Some(ref deal_conf_tm_frame_value) = self.deal_conf_tm_frame { if !deal_conf_tm_frame_value.validate() { return false; } }
		if let Some(ref ltd_prd_value) = self.ltd_prd { if !ltd_prd_value.validate() { return false; } }
		if let Some(ref sttlm_cycl_value) = self.sttlm_cycl { if !sttlm_cycl_value.validate() { return false; } }
		if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if !item.validate() { return false; } } }
		return true
	}
}


// ProcessingCharacteristics12 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProcessingCharacteristics12 {
	#[serde(rename = "DealgCcyAccptd", skip_serializing_if = "Option::is_none")]
	pub dealg_ccy_accptd: Option<Vec<ActiveCurrencyCode>>,
	#[serde(rename = "RedAuthstn", skip_serializing_if = "Option::is_none")]
	pub red_authstn: Option<Forms1>,
	#[serde(rename = "AmtInd", skip_serializing_if = "Option::is_none")]
	pub amt_ind: Option<bool>,
	#[serde(rename = "UnitsInd", skip_serializing_if = "Option::is_none")]
	pub units_ind: Option<bool>,
	#[serde(rename = "Rndg", skip_serializing_if = "Option::is_none")]
	pub rndg: Option<RoundingDirection2Code>,
	#[serde(rename = "PctgInd", skip_serializing_if = "Option::is_none")]
	pub pctg_ind: Option<bool>,
	#[serde(rename = "MainFndOrdrDskLctn", skip_serializing_if = "Option::is_none")]
	pub main_fnd_ordr_dsk_lctn: Option<MainFundOrderDeskLocation1>,
	#[serde(rename = "DealgFrqcy", skip_serializing_if = "Option::is_none")]
	pub dealg_frqcy: Option<EventFrequency5Code>,
	#[serde(rename = "DealgFrqcyDesc", skip_serializing_if = "Option::is_none")]
	pub dealg_frqcy_desc: Option<Max350Text>,
	#[serde(rename = "DealgCutOffTm", skip_serializing_if = "Option::is_none")]
	pub dealg_cut_off_tm: Option<String>,
	#[serde(rename = "DealgCutOffTmFrame", skip_serializing_if = "Option::is_none")]
	pub dealg_cut_off_tm_frame: Option<TimeFrame9>,
	#[serde(rename = "DealConfTm", skip_serializing_if = "Option::is_none")]
	pub deal_conf_tm: Option<String>,
	#[serde(rename = "DealConfTmFrame", skip_serializing_if = "Option::is_none")]
	pub deal_conf_tm_frame: Option<TimeFrame10>,
	#[serde(rename = "LtdPrd", skip_serializing_if = "Option::is_none")]
	pub ltd_prd: Option<Max350Text>,
	#[serde(rename = "SttlmCycl", skip_serializing_if = "Option::is_none")]
	pub sttlm_cycl: Option<TimeFrame8Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}

impl ProcessingCharacteristics12 {
	pub fn validate(&self) -> bool {
		if let Some(ref dealg_ccy_accptd_vec) = self.dealg_ccy_accptd { for item in dealg_ccy_accptd_vec { if !item.validate() { return false; } } }
		if let Some(ref red_authstn_value) = self.red_authstn { if !red_authstn_value.validate() { return false; } }
		if let Some(ref rndg_value) = self.rndg { if !rndg_value.validate() { return false; } }
		if let Some(ref main_fnd_ordr_dsk_lctn_value) = self.main_fnd_ordr_dsk_lctn { if !main_fnd_ordr_dsk_lctn_value.validate() { return false; } }
		if let Some(ref dealg_frqcy_value) = self.dealg_frqcy { if !dealg_frqcy_value.validate() { return false; } }
		if let Some(ref dealg_frqcy_desc_value) = self.dealg_frqcy_desc { if !dealg_frqcy_desc_value.validate() { return false; } }
		if let Some(ref dealg_cut_off_tm_frame_value) = self.dealg_cut_off_tm_frame { if !dealg_cut_off_tm_frame_value.validate() { return false; } }
		if let Some(ref deal_conf_tm_frame_value) = self.deal_conf_tm_frame { if !deal_conf_tm_frame_value.validate() { return false; } }
		if let Some(ref ltd_prd_value) = self.ltd_prd { if !ltd_prd_value.validate() { return false; } }
		if let Some(ref sttlm_cycl_value) = self.sttlm_cycl { if !sttlm_cycl_value.validate() { return false; } }
		if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if !item.validate() { return false; } } }
		return true
	}
}


// ProcessingCharacteristics9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProcessingCharacteristics9 {
	#[serde(rename = "DealgCcyAccptd", skip_serializing_if = "Option::is_none")]
	pub dealg_ccy_accptd: Option<Vec<ActiveCurrencyCode>>,
	#[serde(rename = "SwtchAuthstn", skip_serializing_if = "Option::is_none")]
	pub swtch_authstn: Option<Forms1>,
	#[serde(rename = "AmtInd", skip_serializing_if = "Option::is_none")]
	pub amt_ind: Option<bool>,
	#[serde(rename = "UnitsInd", skip_serializing_if = "Option::is_none")]
	pub units_ind: Option<bool>,
	#[serde(rename = "Rndg", skip_serializing_if = "Option::is_none")]
	pub rndg: Option<RoundingDirection2Code>,
	#[serde(rename = "MainFndOrdrDskLctn", skip_serializing_if = "Option::is_none")]
	pub main_fnd_ordr_dsk_lctn: Option<MainFundOrderDeskLocation1>,
	#[serde(rename = "DealgFrqcy", skip_serializing_if = "Option::is_none")]
	pub dealg_frqcy: Option<EventFrequency5Code>,
	#[serde(rename = "DealgFrqcyDesc", skip_serializing_if = "Option::is_none")]
	pub dealg_frqcy_desc: Option<Max350Text>,
	#[serde(rename = "DealgCutOffTm", skip_serializing_if = "Option::is_none")]
	pub dealg_cut_off_tm: Option<String>,
	#[serde(rename = "DealgCutOffTmFrame", skip_serializing_if = "Option::is_none")]
	pub dealg_cut_off_tm_frame: Option<TimeFrame9>,
	#[serde(rename = "DealConfTm", skip_serializing_if = "Option::is_none")]
	pub deal_conf_tm: Option<String>,
	#[serde(rename = "DealConfTmFrame", skip_serializing_if = "Option::is_none")]
	pub deal_conf_tm_frame: Option<TimeFrame8>,
	#[serde(rename = "LtdPrd", skip_serializing_if = "Option::is_none")]
	pub ltd_prd: Option<Max350Text>,
	#[serde(rename = "SttlmCycl", skip_serializing_if = "Option::is_none")]
	pub sttlm_cycl: Option<TimeFrame8Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}

impl ProcessingCharacteristics9 {
	pub fn validate(&self) -> bool {
		if let Some(ref dealg_ccy_accptd_vec) = self.dealg_ccy_accptd { for item in dealg_ccy_accptd_vec { if !item.validate() { return false; } } }
		if let Some(ref swtch_authstn_value) = self.swtch_authstn { if !swtch_authstn_value.validate() { return false; } }
		if let Some(ref rndg_value) = self.rndg { if !rndg_value.validate() { return false; } }
		if let Some(ref main_fnd_ordr_dsk_lctn_value) = self.main_fnd_ordr_dsk_lctn { if !main_fnd_ordr_dsk_lctn_value.validate() { return false; } }
		if let Some(ref dealg_frqcy_value) = self.dealg_frqcy { if !dealg_frqcy_value.validate() { return false; } }
		if let Some(ref dealg_frqcy_desc_value) = self.dealg_frqcy_desc { if !dealg_frqcy_desc_value.validate() { return false; } }
		if let Some(ref dealg_cut_off_tm_frame_value) = self.dealg_cut_off_tm_frame { if !dealg_cut_off_tm_frame_value.validate() { return false; } }
		if let Some(ref deal_conf_tm_frame_value) = self.deal_conf_tm_frame { if !deal_conf_tm_frame_value.validate() { return false; } }
		if let Some(ref ltd_prd_value) = self.ltd_prd { if !ltd_prd_value.validate() { return false; } }
		if let Some(ref sttlm_cycl_value) = self.sttlm_cycl { if !sttlm_cycl_value.validate() { return false; } }
		if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if !item.validate() { return false; } } }
		return true
	}
}


// ProductStructure1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProductStructure1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ProductStructure1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl ProductStructure1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// ProductStructure1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ProductStructure1Code {
	#[default]
	#[serde(rename = "BOND")]
	CodeBOND,
	#[serde(rename = "NUMM")]
	CodeNUMM,
	#[serde(rename = "UCMM")]
	CodeUCMM,
	#[serde(rename = "EXTC")]
	CodeEXTC,
	#[serde(rename = "UCIT")]
	CodeUCIT,
	#[serde(rename = "SSEC")]
	CodeSSEC,
	#[serde(rename = "SFUN")]
	CodeSFUN,
	#[serde(rename = "NUCI")]
	CodeNUCI,
}

impl ProductStructure1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// QuotationType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct QuotationType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<QuotationType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl QuotationType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// QuotationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum QuotationType1Code {
	#[default]
	#[serde(rename = "ACTU")]
	CodeACTU,
	#[serde(rename = "PRCT")]
	CodePRCT,
}

impl QuotationType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ReferToFundOrderDesk1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ReferToFundOrderDesk1Code {
	#[default]
	#[serde(rename = "RFOD")]
	CodeRFOD,
}

impl ReferToFundOrderDesk1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// RiskLevel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RiskLevel1Code {
	#[default]
	#[serde(rename = "HIGH")]
	CodeHIGH,
	#[serde(rename = "LOWW")]
	CodeLOWW,
	#[serde(rename = "MEDM")]
	CodeMEDM,
}

impl RiskLevel1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// RiskTolerance1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RiskTolerance1 {
	#[serde(rename = "RskTlrncePRIIPSMthdlgy", skip_serializing_if = "Option::is_none")]
	pub rsk_tlrnce_priips_mthdlgy: Option<f64>,
	#[serde(rename = "RskTlrnceUCITSMthdlgy", skip_serializing_if = "Option::is_none")]
	pub rsk_tlrnce_ucits_mthdlgy: Option<f64>,
	#[serde(rename = "RskTlrnceIntl", skip_serializing_if = "Option::is_none")]
	pub rsk_tlrnce_intl: Option<RiskLevel1Code>,
	#[serde(rename = "RskTlrnceForNonPRIIPSAndNonUCITSES", skip_serializing_if = "Option::is_none")]
	pub rsk_tlrnce_for_non_priips_and_non_ucitses: Option<f64>,
	#[serde(rename = "NotForInvstrsWthTheLwstRskTlrnceDE", skip_serializing_if = "Option::is_none")]
	pub not_for_invstrs_wth_the_lwst_rsk_tlrnce_de: Option<TargetMarket2Code>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<OtherTargetMarketRiskTolerance1>>,
}

impl RiskTolerance1 {
	pub fn validate(&self) -> bool {
		if let Some(ref rsk_tlrnce_intl_value) = self.rsk_tlrnce_intl { if !rsk_tlrnce_intl_value.validate() { return false; } }
		if let Some(ref not_for_invstrs_wth_the_lwst_rsk_tlrnce_de_value) = self.not_for_invstrs_wth_the_lwst_rsk_tlrnce_de { if !not_for_invstrs_wth_the_lwst_rsk_tlrnce_de_value.validate() { return false; } }
		if let Some(ref othr_vec) = self.othr { for item in othr_vec { if !item.validate() { return false; } } }
		return true
	}
}


// RoundingDirection2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RoundingDirection2Code {
	#[default]
	#[serde(rename = "RDUP")]
	CodeRDUP,
	#[serde(rename = "RDWN")]
	CodeRDWN,
}

impl RoundingDirection2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// SecurityClassificationType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityClassificationType2Choice {
	#[serde(rename = "CFI", skip_serializing_if = "Option::is_none")]
	pub cfi: Option<CFIOct2015Identifier>,
	#[serde(rename = "AltrnClssfctn", skip_serializing_if = "Option::is_none")]
	pub altrn_clssfctn: Option<GenericIdentification3>,
}

impl SecurityClassificationType2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cfi_value) = self.cfi { if !cfi_value.validate() { return false; } }
		if let Some(ref altrn_clssfctn_value) = self.altrn_clssfctn { if !altrn_clssfctn_value.validate() { return false; } }
		return true
	}
}


// SecurityIdentification40 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification40 {
	#[serde(rename = "OthrId", skip_serializing_if = "Option::is_none")]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max140Text>,
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
}

impl SecurityIdentification40 {
	pub fn validate(&self) -> bool {
		if let Some(ref othr_id_vec) = self.othr_id { for item in othr_id_vec { if !item.validate() { return false; } } }
		if let Some(ref desc_value) = self.desc { if !desc_value.validate() { return false; } }
		if let Some(ref isin_value) = self.isin { if !isin_value.validate() { return false; } }
		return true
	}
}


// SecurityIdentification47 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification47 {
	#[serde(rename = "Id")]
	pub id: SecurityIdentification40,
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
	pub shrt_nm: Option<Max35Text>,
	#[serde(rename = "ClssTp", skip_serializing_if = "Option::is_none")]
	pub clss_tp: Option<Max35Text>,
	#[serde(rename = "UmbrllNm", skip_serializing_if = "Option::is_none")]
	pub umbrll_nm: Option<Max35Text>,
	#[serde(rename = "NewUmbrll", skip_serializing_if = "Option::is_none")]
	pub new_umbrll: Option<bool>,
	#[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
	pub clssfctn_tp: Option<SecurityClassificationType2Choice>,
	#[serde(rename = "BaseCcy", skip_serializing_if = "Option::is_none")]
	pub base_ccy: Option<ActiveCurrencyCode>,
	#[serde(rename = "CtryOfDmcl", skip_serializing_if = "Option::is_none")]
	pub ctry_of_dmcl: Option<CountryCode>,
	#[serde(rename = "RegdDstrbtnCtry", skip_serializing_if = "Option::is_none")]
	pub regd_dstrbtn_ctry: Option<Vec<CountryCode>>,
	#[serde(rename = "PdctTp", skip_serializing_if = "Option::is_none")]
	pub pdct_tp: Option<ProductStructure1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<ContactAttributes5>,
	#[serde(rename = "IssrPdctGovncPrc", skip_serializing_if = "Option::is_none")]
	pub issr_pdct_govnc_prc: Option<GovernanceProcess1Choice>,
	#[serde(rename = "PdctCtgy", skip_serializing_if = "Option::is_none")]
	pub pdct_ctgy: Option<Max140Text>,
	#[serde(rename = "PdctCtgyDE", skip_serializing_if = "Option::is_none")]
	pub pdct_ctgy_de: Option<Max140Text>,
	#[serde(rename = "NtnlOrUnitBased", skip_serializing_if = "Option::is_none")]
	pub ntnl_or_unit_based: Option<NotionalOrUnitBased1Choice>,
	#[serde(rename = "QtnTp", skip_serializing_if = "Option::is_none")]
	pub qtn_tp: Option<QuotationType1Choice>,
	#[serde(rename = "LvrgdOrCntngntLblty", skip_serializing_if = "Option::is_none")]
	pub lvrgd_or_cntngnt_lblty: Option<bool>,
	#[serde(rename = "NoRtrcssnInd", skip_serializing_if = "Option::is_none")]
	pub no_rtrcssn_ind: Option<bool>,
	#[serde(rename = "ExPstCostClctnBsis", skip_serializing_if = "Option::is_none")]
	pub ex_pst_cost_clctn_bsis: Option<ExPostCostCalculationBasis1Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}

impl SecurityIdentification47 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.nm.validate() { return false }
		if let Some(ref shrt_nm_value) = self.shrt_nm { if !shrt_nm_value.validate() { return false; } }
		if let Some(ref clss_tp_value) = self.clss_tp { if !clss_tp_value.validate() { return false; } }
		if let Some(ref umbrll_nm_value) = self.umbrll_nm { if !umbrll_nm_value.validate() { return false; } }
		if let Some(ref clssfctn_tp_value) = self.clssfctn_tp { if !clssfctn_tp_value.validate() { return false; } }
		if let Some(ref base_ccy_value) = self.base_ccy { if !base_ccy_value.validate() { return false; } }
		if let Some(ref ctry_of_dmcl_value) = self.ctry_of_dmcl { if !ctry_of_dmcl_value.validate() { return false; } }
		if let Some(ref regd_dstrbtn_ctry_vec) = self.regd_dstrbtn_ctry { for item in regd_dstrbtn_ctry_vec { if !item.validate() { return false; } } }
		if let Some(ref pdct_tp_value) = self.pdct_tp { if !pdct_tp_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		if let Some(ref issr_pdct_govnc_prc_value) = self.issr_pdct_govnc_prc { if !issr_pdct_govnc_prc_value.validate() { return false; } }
		if let Some(ref pdct_ctgy_value) = self.pdct_ctgy { if !pdct_ctgy_value.validate() { return false; } }
		if let Some(ref pdct_ctgy_de_value) = self.pdct_ctgy_de { if !pdct_ctgy_de_value.validate() { return false; } }
		if let Some(ref ntnl_or_unit_based_value) = self.ntnl_or_unit_based { if !ntnl_or_unit_based_value.validate() { return false; } }
		if let Some(ref qtn_tp_value) = self.qtn_tp { if !qtn_tp_value.validate() { return false; } }
		if let Some(ref ex_pst_cost_clctn_bsis_value) = self.ex_pst_cost_clctn_bsis { if !ex_pst_cost_clctn_bsis_value.validate() { return false; } }
		if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if !item.validate() { return false; } } }
		return true
	}
}


// SignatureType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SignatureType1Code {
	#[default]
	#[serde(rename = "ORIG")]
	CodeORIG,
	#[serde(rename = "DIGI")]
	CodeDIGI,
	#[serde(rename = "ELEC")]
	CodeELEC,
	#[serde(rename = "NONE")]
	CodeNONE,
}

impl SignatureType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// SustainabilityPreferences2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SustainabilityPreferences2Code {
	#[default]
	#[serde(rename = "NEUT")]
	CodeNEUT,
	#[serde(rename = "YSCO")]
	CodeYSCO,
}

impl SustainabilityPreferences2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// TargetMarket1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TargetMarket1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<TargetMarket1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl TargetMarket1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// TargetMarket1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TargetMarket1Code {
	#[default]
	#[serde(rename = "YSCO")]
	CodeYSCO,
	#[serde(rename = "NEUT")]
	CodeNEUT,
	#[serde(rename = "NSCO")]
	CodeNSCO,
}

impl TargetMarket1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// TargetMarket2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TargetMarket2Code {
	#[default]
	#[serde(rename = "NEUT")]
	CodeNEUT,
	#[serde(rename = "YSCO")]
	CodeYSCO,
}

impl TargetMarket2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// TargetMarket3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TargetMarket3Choice {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<InvestorType2Code>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<TargetMarket1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl TargetMarket3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// TargetMarket3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TargetMarket3Code {
	#[default]
	#[serde(rename = "YSCO")]
	CodeYSCO,
	#[serde(rename = "NSCO")]
	CodeNSCO,
}

impl TargetMarket3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// TargetMarket4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TargetMarket4 {
	#[serde(rename = "RefDt", skip_serializing_if = "Option::is_none")]
	pub ref_dt: Option<String>,
	#[serde(rename = "InvstrTp", skip_serializing_if = "Option::is_none")]
	pub invstr_tp: Option<InvestorType2>,
	#[serde(rename = "KnwldgAndOrExprnc", skip_serializing_if = "Option::is_none")]
	pub knwldg_and_or_exprnc: Option<InvestorKnowledge1>,
	#[serde(rename = "AbltyToBearLosses", skip_serializing_if = "Option::is_none")]
	pub ablty_to_bear_losses: Option<LossBearing2>,
	#[serde(rename = "RskTlrnce", skip_serializing_if = "Option::is_none")]
	pub rsk_tlrnce: Option<RiskTolerance1>,
	#[serde(rename = "ClntObjctvsAndNeeds", skip_serializing_if = "Option::is_none")]
	pub clnt_objctvs_and_needs: Option<InvestorRequirements4>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<OtherTargetMarket1>>,
}

impl TargetMarket4 {
	pub fn validate(&self) -> bool {
		if let Some(ref invstr_tp_value) = self.invstr_tp { if !invstr_tp_value.validate() { return false; } }
		if let Some(ref knwldg_and_or_exprnc_value) = self.knwldg_and_or_exprnc { if !knwldg_and_or_exprnc_value.validate() { return false; } }
		if let Some(ref ablty_to_bear_losses_value) = self.ablty_to_bear_losses { if !ablty_to_bear_losses_value.validate() { return false; } }
		if let Some(ref rsk_tlrnce_value) = self.rsk_tlrnce { if !rsk_tlrnce_value.validate() { return false; } }
		if let Some(ref clnt_objctvs_and_needs_value) = self.clnt_objctvs_and_needs { if !clnt_objctvs_and_needs_value.validate() { return false; } }
		if let Some(ref othr_vec) = self.othr { for item in othr_vec { if !item.validate() { return false; } } }
		return true
	}
}


// TargetMarket5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TargetMarket5Choice {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<InvestorType4Code>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<TargetMarket1Code>,
}

impl TargetMarket5Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
	}
}


// TimeFrame10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeFrame10 {
	#[serde(rename = "OthrTmFrameDesc", skip_serializing_if = "Option::is_none")]
	pub othr_tm_frame_desc: Option<Max350Text>,
	#[serde(rename = "TPlus", skip_serializing_if = "Option::is_none")]
	pub t_plus: Option<f64>,
	#[serde(rename = "NonWorkgDayAdjstmnt", skip_serializing_if = "Option::is_none")]
	pub non_workg_day_adjstmnt: Option<BusinessDayConvention1Code>,
	#[serde(rename = "RefrToOrdrDsk", skip_serializing_if = "Option::is_none")]
	pub refr_to_ordr_dsk: Option<ReferToFundOrderDesk1Code>,
}

impl TimeFrame10 {
	pub fn validate(&self) -> bool {
		if let Some(ref othr_tm_frame_desc_value) = self.othr_tm_frame_desc { if !othr_tm_frame_desc_value.validate() { return false; } }
		if let Some(ref non_workg_day_adjstmnt_value) = self.non_workg_day_adjstmnt { if !non_workg_day_adjstmnt_value.validate() { return false; } }
		if let Some(ref refr_to_ordr_dsk_value) = self.refr_to_ordr_dsk { if !refr_to_ordr_dsk_value.validate() { return false; } }
		return true
	}
}


// TimeFrame11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeFrame11 {
	#[serde(rename = "OthrTmFrameDesc", skip_serializing_if = "Option::is_none")]
	pub othr_tm_frame_desc: Option<Max350Text>,
	#[serde(rename = "TPlus", skip_serializing_if = "Option::is_none")]
	pub t_plus: Option<f64>,
	#[serde(rename = "NonWorkgDayAdjstmnt", skip_serializing_if = "Option::is_none")]
	pub non_workg_day_adjstmnt: Option<BusinessDayConvention1Code>,
	#[serde(rename = "RefrToOrdrDsk", skip_serializing_if = "Option::is_none")]
	pub refr_to_ordr_dsk: Option<ReferToFundOrderDesk1Code>,
}

impl TimeFrame11 {
	pub fn validate(&self) -> bool {
		if let Some(ref othr_tm_frame_desc_value) = self.othr_tm_frame_desc { if !othr_tm_frame_desc_value.validate() { return false; } }
		if let Some(ref non_workg_day_adjstmnt_value) = self.non_workg_day_adjstmnt { if !non_workg_day_adjstmnt_value.validate() { return false; } }
		if let Some(ref refr_to_ordr_dsk_value) = self.refr_to_ordr_dsk { if !refr_to_ordr_dsk_value.validate() { return false; } }
		return true
	}
}


// TimeFrame2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TimeFrame2Code {
	#[default]
	#[serde(rename = "HOLD")]
	CodeHOLD,
	#[serde(rename = "LONG")]
	CodeLONG,
	#[serde(rename = "MEDM")]
	CodeMEDM,
	#[serde(rename = "SHOR")]
	CodeSHOR,
	#[serde(rename = "VSHT")]
	CodeVSHT,
}

impl TimeFrame2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// TimeFrame7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeFrame7Choice {
	#[serde(rename = "TPlus", skip_serializing_if = "Option::is_none")]
	pub t_plus: Option<f64>,
	#[serde(rename = "Prepmt", skip_serializing_if = "Option::is_none")]
	pub prepmt: Option<bool>,
}

impl TimeFrame7Choice {
	pub fn validate(&self) -> bool {
		return true
	}
}


// TimeFrame8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeFrame8 {
	#[serde(rename = "OthrTmFrameDesc", skip_serializing_if = "Option::is_none")]
	pub othr_tm_frame_desc: Option<Max350Text>,
	#[serde(rename = "TPlus", skip_serializing_if = "Option::is_none")]
	pub t_plus: Option<f64>,
	#[serde(rename = "NonWorkgDayAdjstmnt", skip_serializing_if = "Option::is_none")]
	pub non_workg_day_adjstmnt: Option<BusinessDayConvention1Code>,
	#[serde(rename = "RefrToOrdrDsk", skip_serializing_if = "Option::is_none")]
	pub refr_to_ordr_dsk: Option<ReferToFundOrderDesk1Code>,
}

impl TimeFrame8 {
	pub fn validate(&self) -> bool {
		if let Some(ref othr_tm_frame_desc_value) = self.othr_tm_frame_desc { if !othr_tm_frame_desc_value.validate() { return false; } }
		if let Some(ref non_workg_day_adjstmnt_value) = self.non_workg_day_adjstmnt { if !non_workg_day_adjstmnt_value.validate() { return false; } }
		if let Some(ref refr_to_ordr_dsk_value) = self.refr_to_ordr_dsk { if !refr_to_ordr_dsk_value.validate() { return false; } }
		return true
	}
}


// TimeFrame8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeFrame8Choice {
	#[serde(rename = "TPlus", skip_serializing_if = "Option::is_none")]
	pub t_plus: Option<f64>,
	#[serde(rename = "RPlus", skip_serializing_if = "Option::is_none")]
	pub r_plus: Option<f64>,
}

impl TimeFrame8Choice {
	pub fn validate(&self) -> bool {
		return true
	}
}


// TimeFrame9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeFrame9 {
	#[serde(rename = "OthrTmFrameDesc", skip_serializing_if = "Option::is_none")]
	pub othr_tm_frame_desc: Option<Max350Text>,
	#[serde(rename = "TMns", skip_serializing_if = "Option::is_none")]
	pub t_mns: Option<f64>,
	#[serde(rename = "NonWorkgDayAdjstmnt", skip_serializing_if = "Option::is_none")]
	pub non_workg_day_adjstmnt: Option<BusinessDayConvention1Code>,
	#[serde(rename = "RefrToOrdrDsk", skip_serializing_if = "Option::is_none")]
	pub refr_to_ordr_dsk: Option<ReferToFundOrderDesk1Code>,
}

impl TimeFrame9 {
	pub fn validate(&self) -> bool {
		if let Some(ref othr_tm_frame_desc_value) = self.othr_tm_frame_desc { if !othr_tm_frame_desc_value.validate() { return false; } }
		if let Some(ref non_workg_day_adjstmnt_value) = self.non_workg_day_adjstmnt { if !non_workg_day_adjstmnt_value.validate() { return false; } }
		if let Some(ref refr_to_ordr_dsk_value) = self.refr_to_ordr_dsk { if !refr_to_ordr_dsk_value.validate() { return false; } }
		return true
	}
}


// TimeFrame9Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeFrame9Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<TimeFrame2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl TimeFrame9Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// TimeHorizon2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeHorizon2Choice {
	#[serde(rename = "NbOfYrs", skip_serializing_if = "Option::is_none")]
	pub nb_of_yrs: Option<f64>,
	#[serde(rename = "TmFrame", skip_serializing_if = "Option::is_none")]
	pub tm_frame: Option<TimeFrame9Choice>,
}

impl TimeHorizon2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref tm_frame_value) = self.tm_frame { if !tm_frame_value.validate() { return false; } }
		return true
	}
}


// UTCOffset1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UTCOffset1 {
	#[serde(rename = "Sgn")]
	pub sgn: bool,
	#[serde(rename = "NbOfHrs")]
	pub nb_of_hrs: String,
}

impl UTCOffset1 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// UnitsOrAmount1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitsOrAmount1Choice {
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
	pub unit: Option<f64>,
}

impl UnitsOrAmount1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref amt_value) = self.amt { if !amt_value.validate() { return false; } }
		return true
	}
}


// ValuationDealingProcessingCharacteristics3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ValuationDealingProcessingCharacteristics3 {
	#[serde(rename = "ValtnFrqcy", skip_serializing_if = "Option::is_none")]
	pub valtn_frqcy: Option<EventFrequency5Code>,
	#[serde(rename = "ValtnFrqcyDesc", skip_serializing_if = "Option::is_none")]
	pub valtn_frqcy_desc: Option<Max350Text>,
	#[serde(rename = "ValtnTm", skip_serializing_if = "Option::is_none")]
	pub valtn_tm: Option<String>,
	#[serde(rename = "DcmlstnUnits", skip_serializing_if = "Option::is_none")]
	pub dcmlstn_units: Option<f64>,
	#[serde(rename = "DcmlstnPric", skip_serializing_if = "Option::is_none")]
	pub dcmlstn_pric: Option<f64>,
	#[serde(rename = "DualFndInd", skip_serializing_if = "Option::is_none")]
	pub dual_fnd_ind: Option<bool>,
	#[serde(rename = "PricMtd", skip_serializing_if = "Option::is_none")]
	pub pric_mtd: Option<PriceMethod1Code>,
	#[serde(rename = "PricCcy", skip_serializing_if = "Option::is_none")]
	pub pric_ccy: Option<Vec<ActiveCurrencyCode>>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}

impl ValuationDealingProcessingCharacteristics3 {
	pub fn validate(&self) -> bool {
		if let Some(ref valtn_frqcy_value) = self.valtn_frqcy { if !valtn_frqcy_value.validate() { return false; } }
		if let Some(ref valtn_frqcy_desc_value) = self.valtn_frqcy_desc { if !valtn_frqcy_desc_value.validate() { return false; } }
		if let Some(ref pric_mtd_value) = self.pric_mtd { if !pric_mtd_value.validate() { return false; } }
		if let Some(ref pric_ccy_vec) = self.pric_ccy { for item in pric_ccy_vec { if !item.validate() { return false; } } }
		if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if !item.validate() { return false; } } }
		return true
	}
}


// ValueForMoney1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ValueForMoney1 {
	#[serde(rename = "EMTDataRptgVFMUK", skip_serializing_if = "Option::is_none")]
	pub emt_data_rptg_vfmuk: Option<EMTDataReportingVFMUKType1Code>,
	#[serde(rename = "AssmntOfValReqrdUdrCOLLUK", skip_serializing_if = "Option::is_none")]
	pub assmnt_of_val_reqrd_udr_colluk: Option<AssessmentOfValueRequiredUnderCOLLUKType1Code>,
	#[serde(rename = "OutcmOfCOLLAssmntOfValUK", skip_serializing_if = "Option::is_none")]
	pub outcm_of_coll_assmnt_of_val_uk: Option<OutcomeOfCOLLAssessmentOfValueUKType1Code>,
	#[serde(rename = "OutcmOfPRINValAssmntOrRvwUK", skip_serializing_if = "Option::is_none")]
	pub outcm_of_prin_val_assmnt_or_rvw_uk: Option<OutcomeOfPRINValueAssessmentOrReviewUKType1Code>,
	#[serde(rename = "OthrRvwRltdToValAndOrChrgsUK", skip_serializing_if = "Option::is_none")]
	pub othr_rvw_rltd_to_val_and_or_chrgs_uk: Option<OtherReviewRelatedToValueAndOrChargesUKType1Code>,
	#[serde(rename = "FrthrInfUK", skip_serializing_if = "Option::is_none")]
	pub frthr_inf_uk: Option<Max350Text>,
	#[serde(rename = "RvwDtUK", skip_serializing_if = "Option::is_none")]
	pub rvw_dt_uk: Option<String>,
	#[serde(rename = "RvwNxtDueUK", skip_serializing_if = "Option::is_none")]
	pub rvw_nxt_due_uk: Option<String>,
}

impl ValueForMoney1 {
	pub fn validate(&self) -> bool {
		if let Some(ref emt_data_rptg_vfmuk_value) = self.emt_data_rptg_vfmuk { if !emt_data_rptg_vfmuk_value.validate() { return false; } }
		if let Some(ref assmnt_of_val_reqrd_udr_colluk_value) = self.assmnt_of_val_reqrd_udr_colluk { if !assmnt_of_val_reqrd_udr_colluk_value.validate() { return false; } }
		if let Some(ref outcm_of_coll_assmnt_of_val_uk_value) = self.outcm_of_coll_assmnt_of_val_uk { if !outcm_of_coll_assmnt_of_val_uk_value.validate() { return false; } }
		if let Some(ref outcm_of_prin_val_assmnt_or_rvw_uk_value) = self.outcm_of_prin_val_assmnt_or_rvw_uk { if !outcm_of_prin_val_assmnt_or_rvw_uk_value.validate() { return false; } }
		if let Some(ref othr_rvw_rltd_to_val_and_or_chrgs_uk_value) = self.othr_rvw_rltd_to_val_and_or_chrgs_uk { if !othr_rvw_rltd_to_val_and_or_chrgs_uk_value.validate() { return false; } }
		if let Some(ref frthr_inf_uk_value) = self.frthr_inf_uk { if !frthr_inf_uk_value.validate() { return false; } }
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
