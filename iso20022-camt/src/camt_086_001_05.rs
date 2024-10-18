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


// AccountIdentification4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountIdentification4Choice {
	#[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
	pub iban: Option<IBAN2007Identifier>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<GenericAccountIdentification1>,
}

impl AccountIdentification4Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref iban_value) = self.iban { if !iban_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
	}
}


// AccountLevel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AccountLevel1Code {
	#[default]
	#[serde(rename = "INTM")]
	CodeINTM,
	#[serde(rename = "SMRY")]
	CodeSMRY,
}

impl AccountLevel1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AccountLevel2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AccountLevel2Code {
	#[default]
	#[serde(rename = "INTM")]
	CodeINTM,
	#[serde(rename = "SMRY")]
	CodeSMRY,
	#[serde(rename = "DETL")]
	CodeDETL,
}

impl AccountLevel2Code {
	pub fn validate(&self) -> bool {
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


// AccountTax1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountTax1 {
	#[serde(rename = "ClctnMtd")]
	pub clctn_mtd: BillingTaxCalculationMethod1Code,
	#[serde(rename = "Rgn", skip_serializing_if = "Option::is_none")]
	pub rgn: Option<Max40Text>,
	#[serde(rename = "NonResCtry", skip_serializing_if = "Option::is_none")]
	pub non_res_ctry: Option<ResidenceLocation1Choice>,
}

impl AccountTax1 {
	pub fn validate(&self) -> bool {
		if !self.clctn_mtd.validate() { return false }
		if let Some(ref rgn_value) = self.rgn { if !rgn_value.validate() { return false; } }
		if let Some(ref non_res_ctry_value) = self.non_res_ctry { if !non_res_ctry_value.validate() { return false; } }
		return true
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
	pub fn validate(&self) -> bool {
		if self.active_or_historic_currency_and_amount_simple_type < 0.000000 {
			return false
		}
		return true
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


// AddressType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<AddressType2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}

impl AddressType3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// AmountAndDirection34 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection34 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "Sgn")]
	pub sgn: bool,
}

impl AmountAndDirection34 {
	pub fn validate(&self) -> bool {
		if !self.amt.validate() { return false }
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


// BICFIDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BICFIDec2014Identifier {
	#[serde(rename = "$value")]
	pub bicfi_dec2014_identifier: String,
}

impl BICFIDec2014Identifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
		if !pattern.is_match(&self.bicfi_dec2014_identifier) {
			return false
		}
		return true
	}
}


// BalanceAdjustment1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BalanceAdjustment1 {
	#[serde(rename = "Tp")]
	pub tp: BalanceAdjustmentType1Code,
	#[serde(rename = "Desc")]
	pub desc: Max105Text,
	#[serde(rename = "BalAmt")]
	pub bal_amt: AmountAndDirection34,
	#[serde(rename = "AvrgAmt", skip_serializing_if = "Option::is_none")]
	pub avrg_amt: Option<AmountAndDirection34>,
	#[serde(rename = "ErrDt", skip_serializing_if = "Option::is_none")]
	pub err_dt: Option<String>,
	#[serde(rename = "PstngDt")]
	pub pstng_dt: String,
	#[serde(rename = "Days", skip_serializing_if = "Option::is_none")]
	pub days: Option<f64>,
	#[serde(rename = "EarngsAdjstmntAmt", skip_serializing_if = "Option::is_none")]
	pub earngs_adjstmnt_amt: Option<AmountAndDirection34>,
}

impl BalanceAdjustment1 {
	pub fn validate(&self) -> bool {
		if !self.tp.validate() { return false }
		if !self.desc.validate() { return false }
		if !self.bal_amt.validate() { return false }
		if let Some(ref avrg_amt_value) = self.avrg_amt { if !avrg_amt_value.validate() { return false; } }
		if let Some(ref earngs_adjstmnt_amt_value) = self.earngs_adjstmnt_amt { if !earngs_adjstmnt_amt_value.validate() { return false; } }
		return true
	}
}


// BalanceAdjustmentType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BalanceAdjustmentType1Code {
	#[default]
	#[serde(rename = "LDGR")]
	CodeLDGR,
	#[serde(rename = "FLOT")]
	CodeFLOT,
	#[serde(rename = "CLLD")]
	CodeCLLD,
}

impl BalanceAdjustmentType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// BankServicesBillingStatementV05 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BankServicesBillingStatementV05 {
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: ReportHeader6,
	#[serde(rename = "BllgStmtGrp")]
	pub bllg_stmt_grp: Vec<StatementGroup5>,
}

impl BankServicesBillingStatementV05 {
	pub fn validate(&self) -> bool {
		if !self.rpt_hdr.validate() { return false }
		for item in &self.bllg_stmt_grp { if !item.validate() { return false; } }
		return true
	}
}


// BankTransactionCodeStructure4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BankTransactionCodeStructure4 {
	#[serde(rename = "Domn", skip_serializing_if = "Option::is_none")]
	pub domn: Option<BankTransactionCodeStructure5>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<ProprietaryBankTransactionCodeStructure1>,
}

impl BankTransactionCodeStructure4 {
	pub fn validate(&self) -> bool {
		if let Some(ref domn_value) = self.domn { if !domn_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// BankTransactionCodeStructure5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BankTransactionCodeStructure5 {
	#[serde(rename = "Cd")]
	pub cd: ExternalBankTransactionDomain1Code,
	#[serde(rename = "Fmly")]
	pub fmly: BankTransactionCodeStructure6,
}

impl BankTransactionCodeStructure5 {
	pub fn validate(&self) -> bool {
		if !self.cd.validate() { return false }
		if !self.fmly.validate() { return false }
		return true
	}
}


// BankTransactionCodeStructure6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BankTransactionCodeStructure6 {
	#[serde(rename = "Cd")]
	pub cd: ExternalBankTransactionFamily1Code,
	#[serde(rename = "SubFmlyCd")]
	pub sub_fmly_cd: ExternalBankTransactionSubFamily1Code,
}

impl BankTransactionCodeStructure6 {
	pub fn validate(&self) -> bool {
		if !self.cd.validate() { return false }
		if !self.sub_fmly_cd.validate() { return false }
		return true
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
	pub fn validate(&self) -> bool {
		return true
	}
}


// BillingBalance1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingBalance1 {
	#[serde(rename = "Tp")]
	pub tp: BillingBalanceType1Choice,
	#[serde(rename = "Val")]
	pub val: AmountAndDirection34,
	#[serde(rename = "CcyTp", skip_serializing_if = "Option::is_none")]
	pub ccy_tp: Option<BillingCurrencyType1Code>,
}

impl BillingBalance1 {
	pub fn validate(&self) -> bool {
		if !self.tp.validate() { return false }
		if !self.val.validate() { return false }
		if let Some(ref ccy_tp_value) = self.ccy_tp { if !ccy_tp_value.validate() { return false; } }
		return true
	}
}


// BillingBalanceType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingBalanceType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalBillingBalanceType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl BillingBalanceType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// BillingChargeMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BillingChargeMethod1Code {
	#[default]
	#[serde(rename = "UPRC")]
	CodeUPRC,
	#[serde(rename = "STAM")]
	CodeSTAM,
	#[serde(rename = "BCHG")]
	CodeBCHG,
	#[serde(rename = "DPRC")]
	CodeDPRC,
	#[serde(rename = "FCHG")]
	CodeFCHG,
	#[serde(rename = "LPRC")]
	CodeLPRC,
	#[serde(rename = "MCHG")]
	CodeMCHG,
	#[serde(rename = "MXRD")]
	CodeMXRD,
	#[serde(rename = "TIR1")]
	CodeTIR1,
	#[serde(rename = "TIR2")]
	CodeTIR2,
	#[serde(rename = "TIR3")]
	CodeTIR3,
	#[serde(rename = "TIR4")]
	CodeTIR4,
	#[serde(rename = "TIR5")]
	CodeTIR5,
	#[serde(rename = "TIR6")]
	CodeTIR6,
	#[serde(rename = "TIR7")]
	CodeTIR7,
	#[serde(rename = "TIR8")]
	CodeTIR8,
	#[serde(rename = "TIR9")]
	CodeTIR9,
	#[serde(rename = "TPRC")]
	CodeTPRC,
	#[serde(rename = "ZPRC")]
	CodeZPRC,
	#[serde(rename = "BBSE")]
	CodeBBSE,
}

impl BillingChargeMethod1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// BillingCompensation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingCompensation1 {
	#[serde(rename = "Tp")]
	pub tp: BillingCompensationType1Choice,
	#[serde(rename = "Val")]
	pub val: AmountAndDirection34,
	#[serde(rename = "CcyTp", skip_serializing_if = "Option::is_none")]
	pub ccy_tp: Option<BillingCurrencyType2Code>,
}

impl BillingCompensation1 {
	pub fn validate(&self) -> bool {
		if !self.tp.validate() { return false }
		if !self.val.validate() { return false }
		if let Some(ref ccy_tp_value) = self.ccy_tp { if !ccy_tp_value.validate() { return false; } }
		return true
	}
}


// BillingCompensationType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingCompensationType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalBillingCompensationType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl BillingCompensationType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// BillingCurrencyType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BillingCurrencyType1Code {
	#[default]
	#[serde(rename = "ACCT")]
	CodeACCT,
	#[serde(rename = "STLM")]
	CodeSTLM,
	#[serde(rename = "PRCG")]
	CodePRCG,
}

impl BillingCurrencyType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// BillingCurrencyType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BillingCurrencyType2Code {
	#[default]
	#[serde(rename = "ACCT")]
	CodeACCT,
	#[serde(rename = "STLM")]
	CodeSTLM,
	#[serde(rename = "PRCG")]
	CodePRCG,
	#[serde(rename = "HOST")]
	CodeHOST,
}

impl BillingCurrencyType2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// BillingMethod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingMethod1 {
	#[serde(rename = "SvcChrgHstAmt")]
	pub svc_chrg_hst_amt: AmountAndDirection34,
	#[serde(rename = "SvcTax")]
	pub svc_tax: BillingServicesAmount1,
	#[serde(rename = "TtlChrg")]
	pub ttl_chrg: BillingServicesAmount2,
	#[serde(rename = "TaxId")]
	pub tax_id: Vec<BillingServicesTax1>,
}

impl BillingMethod1 {
	pub fn validate(&self) -> bool {
		if !self.svc_chrg_hst_amt.validate() { return false }
		if !self.svc_tax.validate() { return false }
		if !self.ttl_chrg.validate() { return false }
		for item in &self.tax_id { if !item.validate() { return false; } }
		return true
	}
}


// BillingMethod1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingMethod1Choice {
	#[serde(rename = "MtdA", skip_serializing_if = "Option::is_none")]
	pub mtd_a: Option<BillingMethod1>,
	#[serde(rename = "MtdB", skip_serializing_if = "Option::is_none")]
	pub mtd_b: Option<BillingMethod2>,
	#[serde(rename = "MtdD", skip_serializing_if = "Option::is_none")]
	pub mtd_d: Option<BillingMethod3>,
}

impl BillingMethod1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref mtd_a_value) = self.mtd_a { if !mtd_a_value.validate() { return false; } }
		if let Some(ref mtd_b_value) = self.mtd_b { if !mtd_b_value.validate() { return false; } }
		if let Some(ref mtd_d_value) = self.mtd_d { if !mtd_d_value.validate() { return false; } }
		return true
	}
}


// BillingMethod2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingMethod2 {
	#[serde(rename = "SvcChrgHstAmt")]
	pub svc_chrg_hst_amt: AmountAndDirection34,
	#[serde(rename = "SvcTax")]
	pub svc_tax: BillingServicesAmount1,
	#[serde(rename = "TaxId")]
	pub tax_id: Vec<BillingServicesTax1>,
}

impl BillingMethod2 {
	pub fn validate(&self) -> bool {
		if !self.svc_chrg_hst_amt.validate() { return false }
		if !self.svc_tax.validate() { return false }
		for item in &self.tax_id { if !item.validate() { return false; } }
		return true
	}
}


// BillingMethod3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingMethod3 {
	#[serde(rename = "SvcTaxPricAmt")]
	pub svc_tax_pric_amt: AmountAndDirection34,
	#[serde(rename = "TaxId")]
	pub tax_id: Vec<BillingServicesTax2>,
}

impl BillingMethod3 {
	pub fn validate(&self) -> bool {
		if !self.svc_tax_pric_amt.validate() { return false }
		for item in &self.tax_id { if !item.validate() { return false; } }
		return true
	}
}


// BillingMethod4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingMethod4 {
	#[serde(rename = "SvcDtl")]
	pub svc_dtl: Vec<BillingServiceParameters2>,
	#[serde(rename = "TaxClctn")]
	pub tax_clctn: TaxCalculation1,
}

impl BillingMethod4 {
	pub fn validate(&self) -> bool {
		for item in &self.svc_dtl { if !item.validate() { return false; } }
		if !self.tax_clctn.validate() { return false }
		return true
	}
}


// BillingPrice1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingPrice1 {
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
	pub unit_pric: Option<AmountAndDirection34>,
	#[serde(rename = "Mtd", skip_serializing_if = "Option::is_none")]
	pub mtd: Option<BillingChargeMethod1Code>,
	#[serde(rename = "Rule", skip_serializing_if = "Option::is_none")]
	pub rule: Option<Max20Text>,
}

impl BillingPrice1 {
	pub fn validate(&self) -> bool {
		if let Some(ref ccy_value) = self.ccy { if !ccy_value.validate() { return false; } }
		if let Some(ref unit_pric_value) = self.unit_pric { if !unit_pric_value.validate() { return false; } }
		if let Some(ref mtd_value) = self.mtd { if !mtd_value.validate() { return false; } }
		if let Some(ref rule_value) = self.rule { if !rule_value.validate() { return false; } }
		return true
	}
}


// BillingRate1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingRate1 {
	#[serde(rename = "Id")]
	pub id: BillingRateIdentification1Choice,
	#[serde(rename = "Val")]
	pub val: f64,
	#[serde(rename = "DaysInPrd", skip_serializing_if = "Option::is_none")]
	pub days_in_prd: Option<f64>,
	#[serde(rename = "DaysInYr", skip_serializing_if = "Option::is_none")]
	pub days_in_yr: Option<f64>,
}

impl BillingRate1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		return true
	}
}


// BillingRateIdentification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingRateIdentification1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalBillingRateIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl BillingRateIdentification1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// BillingService2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingService2 {
	#[serde(rename = "SvcDtl")]
	pub svc_dtl: BillingServiceParameters3,
	#[serde(rename = "Pric", skip_serializing_if = "Option::is_none")]
	pub pric: Option<BillingPrice1>,
	#[serde(rename = "PmtMtd")]
	pub pmt_mtd: ServicePaymentMethod1Code,
	#[serde(rename = "OrgnlChrgPric")]
	pub orgnl_chrg_pric: AmountAndDirection34,
	#[serde(rename = "OrgnlChrgSttlmAmt", skip_serializing_if = "Option::is_none")]
	pub orgnl_chrg_sttlm_amt: Option<AmountAndDirection34>,
	#[serde(rename = "BalReqrdAcctAmt", skip_serializing_if = "Option::is_none")]
	pub bal_reqrd_acct_amt: Option<AmountAndDirection34>,
	#[serde(rename = "TaxDsgnt")]
	pub tax_dsgnt: ServiceTaxDesignation1,
	#[serde(rename = "TaxClctn", skip_serializing_if = "Option::is_none")]
	pub tax_clctn: Option<BillingMethod1Choice>,
}

impl BillingService2 {
	pub fn validate(&self) -> bool {
		if !self.svc_dtl.validate() { return false }
		if let Some(ref pric_value) = self.pric { if !pric_value.validate() { return false; } }
		if !self.pmt_mtd.validate() { return false }
		if !self.orgnl_chrg_pric.validate() { return false }
		if let Some(ref orgnl_chrg_sttlm_amt_value) = self.orgnl_chrg_sttlm_amt { if !orgnl_chrg_sttlm_amt_value.validate() { return false; } }
		if let Some(ref bal_reqrd_acct_amt_value) = self.bal_reqrd_acct_amt { if !bal_reqrd_acct_amt_value.validate() { return false; } }
		if !self.tax_dsgnt.validate() { return false }
		if let Some(ref tax_clctn_value) = self.tax_clctn { if !tax_clctn_value.validate() { return false; } }
		return true
	}
}


// BillingServiceAdjustment1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingServiceAdjustment1 {
	#[serde(rename = "Tp")]
	pub tp: ServiceAdjustmentType1Code,
	#[serde(rename = "Desc")]
	pub desc: Max140Text,
	#[serde(rename = "Amt")]
	pub amt: AmountAndDirection34,
	#[serde(rename = "BalReqrdAmt", skip_serializing_if = "Option::is_none")]
	pub bal_reqrd_amt: Option<AmountAndDirection34>,
	#[serde(rename = "ErrDt", skip_serializing_if = "Option::is_none")]
	pub err_dt: Option<String>,
	#[serde(rename = "AdjstmntId", skip_serializing_if = "Option::is_none")]
	pub adjstmnt_id: Option<Max35Text>,
	#[serde(rename = "SubSvc", skip_serializing_if = "Option::is_none")]
	pub sub_svc: Option<BillingSubServiceIdentification1>,
	#[serde(rename = "PricChng", skip_serializing_if = "Option::is_none")]
	pub pric_chng: Option<AmountAndDirection34>,
	#[serde(rename = "OrgnlPric", skip_serializing_if = "Option::is_none")]
	pub orgnl_pric: Option<AmountAndDirection34>,
	#[serde(rename = "NewPric", skip_serializing_if = "Option::is_none")]
	pub new_pric: Option<AmountAndDirection34>,
	#[serde(rename = "VolChng", skip_serializing_if = "Option::is_none")]
	pub vol_chng: Option<f64>,
	#[serde(rename = "OrgnlVol", skip_serializing_if = "Option::is_none")]
	pub orgnl_vol: Option<f64>,
	#[serde(rename = "NewVol", skip_serializing_if = "Option::is_none")]
	pub new_vol: Option<f64>,
	#[serde(rename = "OrgnlChrgAmt", skip_serializing_if = "Option::is_none")]
	pub orgnl_chrg_amt: Option<AmountAndDirection34>,
	#[serde(rename = "NewChrgAmt", skip_serializing_if = "Option::is_none")]
	pub new_chrg_amt: Option<AmountAndDirection34>,
}

impl BillingServiceAdjustment1 {
	pub fn validate(&self) -> bool {
		if !self.tp.validate() { return false }
		if !self.desc.validate() { return false }
		if !self.amt.validate() { return false }
		if let Some(ref bal_reqrd_amt_value) = self.bal_reqrd_amt { if !bal_reqrd_amt_value.validate() { return false; } }
		if let Some(ref adjstmnt_id_value) = self.adjstmnt_id { if !adjstmnt_id_value.validate() { return false; } }
		if let Some(ref sub_svc_value) = self.sub_svc { if !sub_svc_value.validate() { return false; } }
		if let Some(ref pric_chng_value) = self.pric_chng { if !pric_chng_value.validate() { return false; } }
		if let Some(ref orgnl_pric_value) = self.orgnl_pric { if !orgnl_pric_value.validate() { return false; } }
		if let Some(ref new_pric_value) = self.new_pric { if !new_pric_value.validate() { return false; } }
		if let Some(ref orgnl_chrg_amt_value) = self.orgnl_chrg_amt { if !orgnl_chrg_amt_value.validate() { return false; } }
		if let Some(ref new_chrg_amt_value) = self.new_chrg_amt { if !new_chrg_amt_value.validate() { return false; } }
		return true
	}
}


// BillingServiceCommonIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingServiceCommonIdentification1 {
	#[serde(rename = "Issr")]
	pub issr: Max6Text,
	#[serde(rename = "Id")]
	pub id: Max8Text,
}

impl BillingServiceCommonIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.issr.validate() { return false }
		if !self.id.validate() { return false }
		return true
	}
}


// BillingServiceIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingServiceIdentification2 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "SubSvc", skip_serializing_if = "Option::is_none")]
	pub sub_svc: Option<BillingSubServiceIdentification1>,
	#[serde(rename = "Desc")]
	pub desc: Max70Text,
}

impl BillingServiceIdentification2 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref sub_svc_value) = self.sub_svc { if !sub_svc_value.validate() { return false; } }
		if !self.desc.validate() { return false }
		return true
	}
}


// BillingServiceIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingServiceIdentification3 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "SubSvc", skip_serializing_if = "Option::is_none")]
	pub sub_svc: Option<BillingSubServiceIdentification1>,
	#[serde(rename = "Desc")]
	pub desc: Max70Text,
	#[serde(rename = "CmonCd", skip_serializing_if = "Option::is_none")]
	pub cmon_cd: Option<BillingServiceCommonIdentification1>,
	#[serde(rename = "BkTxCd", skip_serializing_if = "Option::is_none")]
	pub bk_tx_cd: Option<BankTransactionCodeStructure4>,
	#[serde(rename = "SvcTp", skip_serializing_if = "Option::is_none")]
	pub svc_tp: Option<Max12Text>,
}

impl BillingServiceIdentification3 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref sub_svc_value) = self.sub_svc { if !sub_svc_value.validate() { return false; } }
		if !self.desc.validate() { return false }
		if let Some(ref cmon_cd_value) = self.cmon_cd { if !cmon_cd_value.validate() { return false; } }
		if let Some(ref bk_tx_cd_value) = self.bk_tx_cd { if !bk_tx_cd_value.validate() { return false; } }
		if let Some(ref svc_tp_value) = self.svc_tp { if !svc_tp_value.validate() { return false; } }
		return true
	}
}


// BillingServiceParameters2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingServiceParameters2 {
	#[serde(rename = "BkSvc")]
	pub bk_svc: BillingServiceIdentification2,
	#[serde(rename = "Vol", skip_serializing_if = "Option::is_none")]
	pub vol: Option<f64>,
	#[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
	pub unit_pric: Option<AmountAndDirection34>,
	#[serde(rename = "SvcChrgAmt")]
	pub svc_chrg_amt: AmountAndDirection34,
}

impl BillingServiceParameters2 {
	pub fn validate(&self) -> bool {
		if !self.bk_svc.validate() { return false }
		if let Some(ref unit_pric_value) = self.unit_pric { if !unit_pric_value.validate() { return false; } }
		if !self.svc_chrg_amt.validate() { return false }
		return true
	}
}


// BillingServiceParameters3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingServiceParameters3 {
	#[serde(rename = "BkSvc")]
	pub bk_svc: BillingServiceIdentification3,
	#[serde(rename = "Vol", skip_serializing_if = "Option::is_none")]
	pub vol: Option<f64>,
}

impl BillingServiceParameters3 {
	pub fn validate(&self) -> bool {
		if !self.bk_svc.validate() { return false }
		return true
	}
}


// BillingServicesAmount1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingServicesAmount1 {
	#[serde(rename = "HstAmt")]
	pub hst_amt: AmountAndDirection34,
	#[serde(rename = "PricgAmt", skip_serializing_if = "Option::is_none")]
	pub pricg_amt: Option<AmountAndDirection34>,
}

impl BillingServicesAmount1 {
	pub fn validate(&self) -> bool {
		if !self.hst_amt.validate() { return false }
		if let Some(ref pricg_amt_value) = self.pricg_amt { if !pricg_amt_value.validate() { return false; } }
		return true
	}
}


// BillingServicesAmount2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingServicesAmount2 {
	#[serde(rename = "HstAmt")]
	pub hst_amt: AmountAndDirection34,
	#[serde(rename = "SttlmAmt", skip_serializing_if = "Option::is_none")]
	pub sttlm_amt: Option<AmountAndDirection34>,
	#[serde(rename = "PricgAmt", skip_serializing_if = "Option::is_none")]
	pub pricg_amt: Option<AmountAndDirection34>,
}

impl BillingServicesAmount2 {
	pub fn validate(&self) -> bool {
		if !self.hst_amt.validate() { return false }
		if let Some(ref sttlm_amt_value) = self.sttlm_amt { if !sttlm_amt_value.validate() { return false; } }
		if let Some(ref pricg_amt_value) = self.pricg_amt { if !pricg_amt_value.validate() { return false; } }
		return true
	}
}


// BillingServicesAmount3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingServicesAmount3 {
	#[serde(rename = "SrcAmt")]
	pub src_amt: AmountAndDirection34,
	#[serde(rename = "HstAmt")]
	pub hst_amt: AmountAndDirection34,
}

impl BillingServicesAmount3 {
	pub fn validate(&self) -> bool {
		if !self.src_amt.validate() { return false }
		if !self.hst_amt.validate() { return false }
		return true
	}
}


// BillingServicesTax1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingServicesTax1 {
	#[serde(rename = "Nb")]
	pub nb: Max35Text,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max40Text>,
	#[serde(rename = "Rate")]
	pub rate: f64,
	#[serde(rename = "HstAmt")]
	pub hst_amt: AmountAndDirection34,
	#[serde(rename = "PricgAmt", skip_serializing_if = "Option::is_none")]
	pub pricg_amt: Option<AmountAndDirection34>,
}

impl BillingServicesTax1 {
	pub fn validate(&self) -> bool {
		if !self.nb.validate() { return false }
		if let Some(ref desc_value) = self.desc { if !desc_value.validate() { return false; } }
		if !self.hst_amt.validate() { return false }
		if let Some(ref pricg_amt_value) = self.pricg_amt { if !pricg_amt_value.validate() { return false; } }
		return true
	}
}


// BillingServicesTax2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingServicesTax2 {
	#[serde(rename = "Nb")]
	pub nb: Max35Text,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max40Text>,
	#[serde(rename = "Rate")]
	pub rate: f64,
	#[serde(rename = "PricgAmt")]
	pub pricg_amt: AmountAndDirection34,
}

impl BillingServicesTax2 {
	pub fn validate(&self) -> bool {
		if !self.nb.validate() { return false }
		if let Some(ref desc_value) = self.desc { if !desc_value.validate() { return false; } }
		if !self.pricg_amt.validate() { return false }
		return true
	}
}


// BillingServicesTax3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingServicesTax3 {
	#[serde(rename = "Nb")]
	pub nb: Max35Text,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max40Text>,
	#[serde(rename = "Rate")]
	pub rate: f64,
	#[serde(rename = "TtlTaxAmt")]
	pub ttl_tax_amt: AmountAndDirection34,
}

impl BillingServicesTax3 {
	pub fn validate(&self) -> bool {
		if !self.nb.validate() { return false }
		if let Some(ref desc_value) = self.desc { if !desc_value.validate() { return false; } }
		if !self.ttl_tax_amt.validate() { return false }
		return true
	}
}


// BillingStatement5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingStatement5 {
	#[serde(rename = "StmtId")]
	pub stmt_id: Max35Text,
	#[serde(rename = "FrToDt")]
	pub fr_to_dt: DatePeriod1,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
	#[serde(rename = "Sts")]
	pub sts: BillingStatementStatus1Code,
	#[serde(rename = "AcctChrtcs")]
	pub acct_chrtcs: CashAccountCharacteristics5,
	#[serde(rename = "RateData", skip_serializing_if = "Option::is_none")]
	pub rate_data: Option<Vec<BillingRate1>>,
	#[serde(rename = "CcyXchg", skip_serializing_if = "Option::is_none")]
	pub ccy_xchg: Option<Vec<CurrencyExchange6>>,
	#[serde(rename = "Bal", skip_serializing_if = "Option::is_none")]
	pub bal: Option<Vec<BillingBalance1>>,
	#[serde(rename = "Compstn", skip_serializing_if = "Option::is_none")]
	pub compstn: Option<Vec<BillingCompensation1>>,
	#[serde(rename = "Svc", skip_serializing_if = "Option::is_none")]
	pub svc: Option<Vec<BillingService2>>,
	#[serde(rename = "TaxRgn", skip_serializing_if = "Option::is_none")]
	pub tax_rgn: Option<Vec<BillingTaxRegion3>>,
	#[serde(rename = "BalAdjstmnt", skip_serializing_if = "Option::is_none")]
	pub bal_adjstmnt: Option<Vec<BalanceAdjustment1>>,
	#[serde(rename = "SvcAdjstmnt", skip_serializing_if = "Option::is_none")]
	pub svc_adjstmnt: Option<Vec<BillingServiceAdjustment1>>,
}

impl BillingStatement5 {
	pub fn validate(&self) -> bool {
		if !self.stmt_id.validate() { return false }
		if !self.fr_to_dt.validate() { return false }
		if !self.sts.validate() { return false }
		if !self.acct_chrtcs.validate() { return false }
		if let Some(ref rate_data_vec) = self.rate_data { for item in rate_data_vec { if !item.validate() { return false; } } }
		if let Some(ref ccy_xchg_vec) = self.ccy_xchg { for item in ccy_xchg_vec { if !item.validate() { return false; } } }
		if let Some(ref bal_vec) = self.bal { for item in bal_vec { if !item.validate() { return false; } } }
		if let Some(ref compstn_vec) = self.compstn { for item in compstn_vec { if !item.validate() { return false; } } }
		if let Some(ref svc_vec) = self.svc { for item in svc_vec { if !item.validate() { return false; } } }
		if let Some(ref tax_rgn_vec) = self.tax_rgn { for item in tax_rgn_vec { if !item.validate() { return false; } } }
		if let Some(ref bal_adjstmnt_vec) = self.bal_adjstmnt { for item in bal_adjstmnt_vec { if !item.validate() { return false; } } }
		if let Some(ref svc_adjstmnt_vec) = self.svc_adjstmnt { for item in svc_adjstmnt_vec { if !item.validate() { return false; } } }
		return true
	}
}


// BillingStatementStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BillingStatementStatus1Code {
	#[default]
	#[serde(rename = "ORGN")]
	CodeORGN,
	#[serde(rename = "RPLC")]
	CodeRPLC,
	#[serde(rename = "TEST")]
	CodeTEST,
}

impl BillingStatementStatus1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// BillingSubServiceIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingSubServiceIdentification1 {
	#[serde(rename = "Issr")]
	pub issr: BillingSubServiceQualifier1Choice,
	#[serde(rename = "Id")]
	pub id: Max35Text,
}

impl BillingSubServiceIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.issr.validate() { return false }
		if !self.id.validate() { return false }
		return true
	}
}


// BillingSubServiceQualifier1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingSubServiceQualifier1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<BillingSubServiceQualifier1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl BillingSubServiceQualifier1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// BillingSubServiceQualifier1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BillingSubServiceQualifier1Code {
	#[default]
	#[serde(rename = "LBOX")]
	CodeLBOX,
	#[serde(rename = "STOR")]
	CodeSTOR,
	#[serde(rename = "BILA")]
	CodeBILA,
	#[serde(rename = "SEQN")]
	CodeSEQN,
	#[serde(rename = "MACT")]
	CodeMACT,
}

impl BillingSubServiceQualifier1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// BillingTaxCalculationMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BillingTaxCalculationMethod1Code {
	#[default]
	#[serde(rename = "NTAX")]
	CodeNTAX,
	#[serde(rename = "MTDA")]
	CodeMTDA,
	#[serde(rename = "MTDB")]
	CodeMTDB,
	#[serde(rename = "MTDC")]
	CodeMTDC,
	#[serde(rename = "MTDD")]
	CodeMTDD,
	#[serde(rename = "UDFD")]
	CodeUDFD,
}

impl BillingTaxCalculationMethod1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// BillingTaxIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingTaxIdentification3 {
	#[serde(rename = "VATRegnNb", skip_serializing_if = "Option::is_none")]
	pub vat_regn_nb: Option<Max35Text>,
	#[serde(rename = "TaxRegnNb", skip_serializing_if = "Option::is_none")]
	pub tax_regn_nb: Option<Max35Text>,
	#[serde(rename = "TaxCtct", skip_serializing_if = "Option::is_none")]
	pub tax_ctct: Option<Contact13>,
}

impl BillingTaxIdentification3 {
	pub fn validate(&self) -> bool {
		if let Some(ref vat_regn_nb_value) = self.vat_regn_nb { if !vat_regn_nb_value.validate() { return false; } }
		if let Some(ref tax_regn_nb_value) = self.tax_regn_nb { if !tax_regn_nb_value.validate() { return false; } }
		if let Some(ref tax_ctct_value) = self.tax_ctct { if !tax_ctct_value.validate() { return false; } }
		return true
	}
}


// BillingTaxRegion3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingTaxRegion3 {
	#[serde(rename = "RgnNb")]
	pub rgn_nb: Max40Text,
	#[serde(rename = "RgnNm")]
	pub rgn_nm: Max40Text,
	#[serde(rename = "CstmrTaxId")]
	pub cstmr_tax_id: Max40Text,
	#[serde(rename = "PtDt", skip_serializing_if = "Option::is_none")]
	pub pt_dt: Option<String>,
	#[serde(rename = "SndgFI", skip_serializing_if = "Option::is_none")]
	pub sndg_fi: Option<BillingTaxIdentification3>,
	#[serde(rename = "InvcNb", skip_serializing_if = "Option::is_none")]
	pub invc_nb: Option<Max40Text>,
	#[serde(rename = "MtdC", skip_serializing_if = "Option::is_none")]
	pub mtd_c: Option<BillingMethod4>,
	#[serde(rename = "SttlmAmt")]
	pub sttlm_amt: AmountAndDirection34,
	#[serde(rename = "TaxDueToRgn")]
	pub tax_due_to_rgn: AmountAndDirection34,
}

impl BillingTaxRegion3 {
	pub fn validate(&self) -> bool {
		if !self.rgn_nb.validate() { return false }
		if !self.rgn_nm.validate() { return false }
		if !self.cstmr_tax_id.validate() { return false }
		if let Some(ref sndg_fi_value) = self.sndg_fi { if !sndg_fi_value.validate() { return false; } }
		if let Some(ref invc_nb_value) = self.invc_nb { if !invc_nb_value.validate() { return false; } }
		if let Some(ref mtd_c_value) = self.mtd_c { if !mtd_c_value.validate() { return false; } }
		if !self.sttlm_amt.validate() { return false }
		if !self.tax_due_to_rgn.validate() { return false }
		return true
	}
}


// BranchAndFinancialInstitutionIdentification8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BranchAndFinancialInstitutionIdentification8 {
	#[serde(rename = "FinInstnId")]
	pub fin_instn_id: FinancialInstitutionIdentification23,
	#[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
	pub brnch_id: Option<BranchData5>,
}

impl BranchAndFinancialInstitutionIdentification8 {
	pub fn validate(&self) -> bool {
		if !self.fin_instn_id.validate() { return false }
		if let Some(ref brnch_id_value) = self.brnch_id { if !brnch_id_value.validate() { return false; } }
		return true
	}
}


// BranchData5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BranchData5 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max35Text>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress27>,
}

impl BranchData5 {
	pub fn validate(&self) -> bool {
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
		if let Some(ref lei_value) = self.lei { if !lei_value.validate() { return false; } }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref pstl_adr_value) = self.pstl_adr { if !pstl_adr_value.validate() { return false; } }
		return true
	}
}


// CashAccount40 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccount40 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<AccountIdentification4Choice>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<CashAccountType2Choice>,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max70Text>,
	#[serde(rename = "Prxy", skip_serializing_if = "Option::is_none")]
	pub prxy: Option<ProxyAccountIdentification1>,
}

impl CashAccount40 {
	pub fn validate(&self) -> bool {
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		if let Some(ref ccy_value) = self.ccy { if !ccy_value.validate() { return false; } }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref prxy_value) = self.prxy { if !prxy_value.validate() { return false; } }
		return true
	}
}


// CashAccountCharacteristics5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountCharacteristics5 {
	#[serde(rename = "AcctLvl")]
	pub acct_lvl: AccountLevel2Code,
	#[serde(rename = "CshAcct")]
	pub csh_acct: CashAccount40,
	#[serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none")]
	pub acct_svcr: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "PrntAcct", skip_serializing_if = "Option::is_none")]
	pub prnt_acct: Option<ParentCashAccount5>,
	#[serde(rename = "CompstnMtd")]
	pub compstn_mtd: CompensationMethod1Code,
	#[serde(rename = "DbtAcct", skip_serializing_if = "Option::is_none")]
	pub dbt_acct: Option<AccountIdentification4Choice>,
	#[serde(rename = "DelydDbtDt", skip_serializing_if = "Option::is_none")]
	pub delyd_dbt_dt: Option<String>,
	#[serde(rename = "SttlmAdvc", skip_serializing_if = "Option::is_none")]
	pub sttlm_advc: Option<Max105Text>,
	#[serde(rename = "AcctBalCcyCd")]
	pub acct_bal_ccy_cd: ActiveOrHistoricCurrencyCode,
	#[serde(rename = "SttlmCcyCd", skip_serializing_if = "Option::is_none")]
	pub sttlm_ccy_cd: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "HstCcyCd", skip_serializing_if = "Option::is_none")]
	pub hst_ccy_cd: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
	pub tax: Option<AccountTax1>,
	#[serde(rename = "AcctSvcrCtct")]
	pub acct_svcr_ctct: Contact13,
}

impl CashAccountCharacteristics5 {
	pub fn validate(&self) -> bool {
		if !self.acct_lvl.validate() { return false }
		if !self.csh_acct.validate() { return false }
		if let Some(ref acct_svcr_value) = self.acct_svcr { if !acct_svcr_value.validate() { return false; } }
		if let Some(ref prnt_acct_value) = self.prnt_acct { if !prnt_acct_value.validate() { return false; } }
		if !self.compstn_mtd.validate() { return false }
		if let Some(ref dbt_acct_value) = self.dbt_acct { if !dbt_acct_value.validate() { return false; } }
		if let Some(ref sttlm_advc_value) = self.sttlm_advc { if !sttlm_advc_value.validate() { return false; } }
		if !self.acct_bal_ccy_cd.validate() { return false }
		if let Some(ref sttlm_ccy_cd_value) = self.sttlm_ccy_cd { if !sttlm_ccy_cd_value.validate() { return false; } }
		if let Some(ref hst_ccy_cd_value) = self.hst_ccy_cd { if !hst_ccy_cd_value.validate() { return false; } }
		if let Some(ref tax_value) = self.tax { if !tax_value.validate() { return false; } }
		if !self.acct_svcr_ctct.validate() { return false }
		return true
	}
}


// CashAccountType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalCashAccountType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl CashAccountType2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// ClearingSystemIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemIdentification2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalClearingSystemIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl ClearingSystemIdentification2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// ClearingSystemMemberIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemMemberIdentification2 {
	#[serde(rename = "ClrSysId", skip_serializing_if = "Option::is_none")]
	pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
	#[serde(rename = "MmbId")]
	pub mmb_id: Max35Text,
}

impl ClearingSystemMemberIdentification2 {
	pub fn validate(&self) -> bool {
		if let Some(ref clr_sys_id_value) = self.clr_sys_id { if !clr_sys_id_value.validate() { return false; } }
		if !self.mmb_id.validate() { return false }
		return true
	}
}


// CompensationMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CompensationMethod1Code {
	#[default]
	#[serde(rename = "NOCP")]
	CodeNOCP,
	#[serde(rename = "DBTD")]
	CodeDBTD,
	#[serde(rename = "INVD")]
	CodeINVD,
	#[serde(rename = "DDBT")]
	CodeDDBT,
}

impl CompensationMethod1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Contact13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Contact13 {
	#[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
	pub nm_prfx: Option<NamePrefix2Code>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
	pub phne_nb: Option<PhoneNumber>,
	#[serde(rename = "MobNb", skip_serializing_if = "Option::is_none")]
	pub mob_nb: Option<PhoneNumber>,
	#[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
	pub fax_nb: Option<PhoneNumber>,
	#[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
	pub url_adr: Option<Max2048Text>,
	#[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
	pub email_adr: Option<Max256Text>,
	#[serde(rename = "EmailPurp", skip_serializing_if = "Option::is_none")]
	pub email_purp: Option<Max35Text>,
	#[serde(rename = "JobTitl", skip_serializing_if = "Option::is_none")]
	pub job_titl: Option<Max35Text>,
	#[serde(rename = "Rspnsblty", skip_serializing_if = "Option::is_none")]
	pub rspnsblty: Option<Max35Text>,
	#[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
	pub dept: Option<Max70Text>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<OtherContact1>>,
	#[serde(rename = "PrefrdMtd", skip_serializing_if = "Option::is_none")]
	pub prefrd_mtd: Option<PreferredContactMethod2Code>,
}

impl Contact13 {
	pub fn validate(&self) -> bool {
		if let Some(ref nm_prfx_value) = self.nm_prfx { if !nm_prfx_value.validate() { return false; } }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref phne_nb_value) = self.phne_nb { if !phne_nb_value.validate() { return false; } }
		if let Some(ref mob_nb_value) = self.mob_nb { if !mob_nb_value.validate() { return false; } }
		if let Some(ref fax_nb_value) = self.fax_nb { if !fax_nb_value.validate() { return false; } }
		if let Some(ref url_adr_value) = self.url_adr { if !url_adr_value.validate() { return false; } }
		if let Some(ref email_adr_value) = self.email_adr { if !email_adr_value.validate() { return false; } }
		if let Some(ref email_purp_value) = self.email_purp { if !email_purp_value.validate() { return false; } }
		if let Some(ref job_titl_value) = self.job_titl { if !job_titl_value.validate() { return false; } }
		if let Some(ref rspnsblty_value) = self.rspnsblty { if !rspnsblty_value.validate() { return false; } }
		if let Some(ref dept_value) = self.dept { if !dept_value.validate() { return false; } }
		if let Some(ref othr_vec) = self.othr { for item in othr_vec { if !item.validate() { return false; } } }
		if let Some(ref prefrd_mtd_value) = self.prefrd_mtd { if !prefrd_mtd_value.validate() { return false; } }
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


// CurrencyExchange6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CurrencyExchange6 {
	#[serde(rename = "SrcCcy")]
	pub src_ccy: ActiveOrHistoricCurrencyCode,
	#[serde(rename = "TrgtCcy")]
	pub trgt_ccy: ActiveOrHistoricCurrencyCode,
	#[serde(rename = "XchgRate")]
	pub xchg_rate: f64,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max40Text>,
	#[serde(rename = "UnitCcy", skip_serializing_if = "Option::is_none")]
	pub unit_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "Cmnts", skip_serializing_if = "Option::is_none")]
	pub cmnts: Option<Max70Text>,
	#[serde(rename = "QtnDt", skip_serializing_if = "Option::is_none")]
	pub qtn_dt: Option<String>,
}

impl CurrencyExchange6 {
	pub fn validate(&self) -> bool {
		if !self.src_ccy.validate() { return false }
		if !self.trgt_ccy.validate() { return false }
		if let Some(ref desc_value) = self.desc { if !desc_value.validate() { return false; } }
		if let Some(ref unit_ccy_value) = self.unit_ccy { if !unit_ccy_value.validate() { return false; } }
		if let Some(ref cmnts_value) = self.cmnts { if !cmnts_value.validate() { return false; } }
		return true
	}
}


// DatePeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriod1 {
	#[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}

impl DatePeriod1 {
	pub fn validate(&self) -> bool {
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


// ExternalBankTransactionDomain1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalBankTransactionDomain1Code {
	#[serde(rename = "$value")]
	pub external_bank_transaction_domain1_code: String,
}

impl ExternalBankTransactionDomain1Code {
	pub fn validate(&self) -> bool {
		if self.external_bank_transaction_domain1_code.chars().count() < 1 {
			return false
		}
		if self.external_bank_transaction_domain1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// ExternalBankTransactionFamily1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalBankTransactionFamily1Code {
	#[serde(rename = "$value")]
	pub external_bank_transaction_family1_code: String,
}

impl ExternalBankTransactionFamily1Code {
	pub fn validate(&self) -> bool {
		if self.external_bank_transaction_family1_code.chars().count() < 1 {
			return false
		}
		if self.external_bank_transaction_family1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// ExternalBankTransactionSubFamily1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalBankTransactionSubFamily1Code {
	#[serde(rename = "$value")]
	pub external_bank_transaction_sub_family1_code: String,
}

impl ExternalBankTransactionSubFamily1Code {
	pub fn validate(&self) -> bool {
		if self.external_bank_transaction_sub_family1_code.chars().count() < 1 {
			return false
		}
		if self.external_bank_transaction_sub_family1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// ExternalBillingBalanceType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalBillingBalanceType1Code {
	#[serde(rename = "$value")]
	pub external_billing_balance_type1_code: String,
}

impl ExternalBillingBalanceType1Code {
	pub fn validate(&self) -> bool {
		if self.external_billing_balance_type1_code.chars().count() < 1 {
			return false
		}
		if self.external_billing_balance_type1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// ExternalBillingCompensationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalBillingCompensationType1Code {
	#[serde(rename = "$value")]
	pub external_billing_compensation_type1_code: String,
}

impl ExternalBillingCompensationType1Code {
	pub fn validate(&self) -> bool {
		if self.external_billing_compensation_type1_code.chars().count() < 1 {
			return false
		}
		if self.external_billing_compensation_type1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// ExternalBillingRateIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalBillingRateIdentification1Code {
	#[serde(rename = "$value")]
	pub external_billing_rate_identification1_code: String,
}

impl ExternalBillingRateIdentification1Code {
	pub fn validate(&self) -> bool {
		if self.external_billing_rate_identification1_code.chars().count() < 1 {
			return false
		}
		if self.external_billing_rate_identification1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// ExternalCashAccountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalCashAccountType1Code {
	#[serde(rename = "$value")]
	pub external_cash_account_type1_code: String,
}

impl ExternalCashAccountType1Code {
	pub fn validate(&self) -> bool {
		if self.external_cash_account_type1_code.chars().count() < 1 {
			return false
		}
		if self.external_cash_account_type1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// ExternalClearingSystemIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalClearingSystemIdentification1Code {
	#[serde(rename = "$value")]
	pub external_clearing_system_identification1_code: String,
}

impl ExternalClearingSystemIdentification1Code {
	pub fn validate(&self) -> bool {
		if self.external_clearing_system_identification1_code.chars().count() < 1 {
			return false
		}
		if self.external_clearing_system_identification1_code.chars().count() > 5 {
			return false
		}
		return true
	}
}


// ExternalFinancialInstitutionIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalFinancialInstitutionIdentification1Code {
	#[serde(rename = "$value")]
	pub external_financial_institution_identification1_code: String,
}

impl ExternalFinancialInstitutionIdentification1Code {
	pub fn validate(&self) -> bool {
		if self.external_financial_institution_identification1_code.chars().count() < 1 {
			return false
		}
		if self.external_financial_institution_identification1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// ExternalOrganisationIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalOrganisationIdentification1Code {
	#[serde(rename = "$value")]
	pub external_organisation_identification1_code: String,
}

impl ExternalOrganisationIdentification1Code {
	pub fn validate(&self) -> bool {
		if self.external_organisation_identification1_code.chars().count() < 1 {
			return false
		}
		if self.external_organisation_identification1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// ExternalProxyAccountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalProxyAccountType1Code {
	#[serde(rename = "$value")]
	pub external_proxy_account_type1_code: String,
}

impl ExternalProxyAccountType1Code {
	pub fn validate(&self) -> bool {
		if self.external_proxy_account_type1_code.chars().count() < 1 {
			return false
		}
		if self.external_proxy_account_type1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// FinancialIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalFinancialInstitutionIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl FinancialIdentificationSchemeName1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// FinancialInstitutionIdentification19 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstitutionIdentification19 {
	#[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
	pub bicfi: Option<BICFIDec2014Identifier>,
	#[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<GenericFinancialIdentification1>,
}

impl FinancialInstitutionIdentification19 {
	pub fn validate(&self) -> bool {
		if let Some(ref bicfi_value) = self.bicfi { if !bicfi_value.validate() { return false; } }
		if let Some(ref clr_sys_mmb_id_value) = self.clr_sys_mmb_id { if !clr_sys_mmb_id_value.validate() { return false; } }
		if let Some(ref lei_value) = self.lei { if !lei_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
	}
}


// FinancialInstitutionIdentification23 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstitutionIdentification23 {
	#[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
	pub bicfi: Option<BICFIDec2014Identifier>,
	#[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress27>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<GenericFinancialIdentification1>,
}

impl FinancialInstitutionIdentification23 {
	pub fn validate(&self) -> bool {
		if let Some(ref bicfi_value) = self.bicfi { if !bicfi_value.validate() { return false; } }
		if let Some(ref clr_sys_mmb_id_value) = self.clr_sys_mmb_id { if !clr_sys_mmb_id_value.validate() { return false; } }
		if let Some(ref lei_value) = self.lei { if !lei_value.validate() { return false; } }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref pstl_adr_value) = self.pstl_adr { if !pstl_adr_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
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


// GenericFinancialIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericFinancialIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<FinancialIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericFinancialIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		return true
	}
}


// GenericIdentification30 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification30 {
	#[serde(rename = "Id")]
	pub id: Exact4AlphaNumericText,
	#[serde(rename = "Issr")]
	pub issr: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
}

impl GenericIdentification30 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.issr.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		return true
	}
}


// GenericOrganisationIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericOrganisationIdentification3 {
	#[serde(rename = "Id")]
	pub id: Max256Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericOrganisationIdentification3 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
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


// Max10Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max10Text {
	#[serde(rename = "$value")]
	pub max10_text: String,
}

impl Max10Text {
	pub fn validate(&self) -> bool {
		if self.max10_text.chars().count() < 1 {
			return false
		}
		if self.max10_text.chars().count() > 10 {
			return false
		}
		return true
	}
}


// Max128Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max128Text {
	#[serde(rename = "$value")]
	pub max128_text: String,
}

impl Max128Text {
	pub fn validate(&self) -> bool {
		if self.max128_text.chars().count() < 1 {
			return false
		}
		if self.max128_text.chars().count() > 128 {
			return false
		}
		return true
	}
}


// Max12Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max12Text {
	#[serde(rename = "$value")]
	pub max12_text: String,
}

impl Max12Text {
	pub fn validate(&self) -> bool {
		if self.max12_text.chars().count() < 1 {
			return false
		}
		if self.max12_text.chars().count() > 12 {
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


// Max20Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max20Text {
	#[serde(rename = "$value")]
	pub max20_text: String,
}

impl Max20Text {
	pub fn validate(&self) -> bool {
		if self.max20_text.chars().count() < 1 {
			return false
		}
		if self.max20_text.chars().count() > 20 {
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


// Max40Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max40Text {
	#[serde(rename = "$value")]
	pub max40_text: String,
}

impl Max40Text {
	pub fn validate(&self) -> bool {
		if self.max40_text.chars().count() < 1 {
			return false
		}
		if self.max40_text.chars().count() > 40 {
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


// Max6Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max6Text {
	#[serde(rename = "$value")]
	pub max6_text: String,
}

impl Max6Text {
	pub fn validate(&self) -> bool {
		if self.max6_text.chars().count() < 1 {
			return false
		}
		if self.max6_text.chars().count() > 6 {
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


// Max8Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max8Text {
	#[serde(rename = "$value")]
	pub max8_text: String,
}

impl Max8Text {
	pub fn validate(&self) -> bool {
		if self.max8_text.chars().count() < 1 {
			return false
		}
		if self.max8_text.chars().count() > 8 {
			return false
		}
		return true
	}
}


// NamePrefix2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NamePrefix2Code {
	#[default]
	#[serde(rename = "DOCT")]
	CodeDOCT,
	#[serde(rename = "MADM")]
	CodeMADM,
	#[serde(rename = "MISS")]
	CodeMISS,
	#[serde(rename = "MIST")]
	CodeMIST,
	#[serde(rename = "MIKS")]
	CodeMIKS,
}

impl NamePrefix2Code {
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


// OrganisationIdentification39 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification39 {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<GenericOrganisationIdentification3>>,
}

impl OrganisationIdentification39 {
	pub fn validate(&self) -> bool {
		if let Some(ref any_bic_value) = self.any_bic { if !any_bic_value.validate() { return false; } }
		if let Some(ref lei_value) = self.lei { if !lei_value.validate() { return false; } }
		if let Some(ref othr_vec) = self.othr { for item in othr_vec { if !item.validate() { return false; } } }
		return true
	}
}


// OrganisationIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalOrganisationIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl OrganisationIdentificationSchemeName1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// OtherContact1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherContact1 {
	#[serde(rename = "ChanlTp")]
	pub chanl_tp: Max4Text,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max128Text>,
}

impl OtherContact1 {
	pub fn validate(&self) -> bool {
		if !self.chanl_tp.validate() { return false }
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
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


// ParentCashAccount5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ParentCashAccount5 {
	#[serde(rename = "Lvl", skip_serializing_if = "Option::is_none")]
	pub lvl: Option<AccountLevel1Code>,
	#[serde(rename = "Id")]
	pub id: CashAccount40,
	#[serde(rename = "Svcr", skip_serializing_if = "Option::is_none")]
	pub svcr: Option<BranchAndFinancialInstitutionIdentification8>,
}

impl ParentCashAccount5 {
	pub fn validate(&self) -> bool {
		if let Some(ref lvl_value) = self.lvl { if !lvl_value.validate() { return false; } }
		if !self.id.validate() { return false }
		if let Some(ref svcr_value) = self.svcr { if !svcr_value.validate() { return false; } }
		return true
	}
}


// Party56Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party56Choice {
	#[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
	pub org_id: Option<OrganisationIdentification39>,
	#[serde(rename = "FIId", skip_serializing_if = "Option::is_none")]
	pub fi_id: Option<FinancialInstitutionIdentification19>,
}

impl Party56Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref org_id_value) = self.org_id { if !org_id_value.validate() { return false; } }
		if let Some(ref fi_id_value) = self.fi_id { if !fi_id_value.validate() { return false; } }
		return true
	}
}


// PartyIdentification273 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification273 {
	#[serde(rename = "Nm")]
	pub nm: Max140Text,
	#[serde(rename = "LglNm", skip_serializing_if = "Option::is_none")]
	pub lgl_nm: Option<Max140Text>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress27>,
	#[serde(rename = "Id")]
	pub id: Party56Choice,
	#[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
	pub ctry_of_res: Option<CountryCode>,
	#[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
	pub ctct_dtls: Option<Contact13>,
}

impl PartyIdentification273 {
	pub fn validate(&self) -> bool {
		if !self.nm.validate() { return false }
		if let Some(ref lgl_nm_value) = self.lgl_nm { if !lgl_nm_value.validate() { return false; } }
		if let Some(ref pstl_adr_value) = self.pstl_adr { if !pstl_adr_value.validate() { return false; } }
		if !self.id.validate() { return false }
		if let Some(ref ctry_of_res_value) = self.ctry_of_res { if !ctry_of_res_value.validate() { return false; } }
		if let Some(ref ctct_dtls_value) = self.ctct_dtls { if !ctct_dtls_value.validate() { return false; } }
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


// PostalAddress27 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress27 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<AddressType3Choice>,
	#[serde(rename = "CareOf", skip_serializing_if = "Option::is_none")]
	pub care_of: Option<Max140Text>,
	#[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
	pub dept: Option<Max70Text>,
	#[serde(rename = "SubDept", skip_serializing_if = "Option::is_none")]
	pub sub_dept: Option<Max70Text>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<Max140Text>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<Max16Text>,
	#[serde(rename = "BldgNm", skip_serializing_if = "Option::is_none")]
	pub bldg_nm: Option<Max140Text>,
	#[serde(rename = "Flr", skip_serializing_if = "Option::is_none")]
	pub flr: Option<Max70Text>,
	#[serde(rename = "UnitNb", skip_serializing_if = "Option::is_none")]
	pub unit_nb: Option<Max16Text>,
	#[serde(rename = "PstBx", skip_serializing_if = "Option::is_none")]
	pub pst_bx: Option<Max16Text>,
	#[serde(rename = "Room", skip_serializing_if = "Option::is_none")]
	pub room: Option<Max70Text>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<Max16Text>,
	#[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
	pub twn_nm: Option<Max140Text>,
	#[serde(rename = "TwnLctnNm", skip_serializing_if = "Option::is_none")]
	pub twn_lctn_nm: Option<Max140Text>,
	#[serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none")]
	pub dstrct_nm: Option<Max140Text>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<Max35Text>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
	pub adr_line: Option<Vec<Max70Text>>,
}

impl PostalAddress27 {
	pub fn validate(&self) -> bool {
		if let Some(ref adr_tp_value) = self.adr_tp { if !adr_tp_value.validate() { return false; } }
		if let Some(ref care_of_value) = self.care_of { if !care_of_value.validate() { return false; } }
		if let Some(ref dept_value) = self.dept { if !dept_value.validate() { return false; } }
		if let Some(ref sub_dept_value) = self.sub_dept { if !sub_dept_value.validate() { return false; } }
		if let Some(ref strt_nm_value) = self.strt_nm { if !strt_nm_value.validate() { return false; } }
		if let Some(ref bldg_nb_value) = self.bldg_nb { if !bldg_nb_value.validate() { return false; } }
		if let Some(ref bldg_nm_value) = self.bldg_nm { if !bldg_nm_value.validate() { return false; } }
		if let Some(ref flr_value) = self.flr { if !flr_value.validate() { return false; } }
		if let Some(ref unit_nb_value) = self.unit_nb { if !unit_nb_value.validate() { return false; } }
		if let Some(ref pst_bx_value) = self.pst_bx { if !pst_bx_value.validate() { return false; } }
		if let Some(ref room_value) = self.room { if !room_value.validate() { return false; } }
		if let Some(ref pst_cd_value) = self.pst_cd { if !pst_cd_value.validate() { return false; } }
		if let Some(ref twn_nm_value) = self.twn_nm { if !twn_nm_value.validate() { return false; } }
		if let Some(ref twn_lctn_nm_value) = self.twn_lctn_nm { if !twn_lctn_nm_value.validate() { return false; } }
		if let Some(ref dstrct_nm_value) = self.dstrct_nm { if !dstrct_nm_value.validate() { return false; } }
		if let Some(ref ctry_sub_dvsn_value) = self.ctry_sub_dvsn { if !ctry_sub_dvsn_value.validate() { return false; } }
		if let Some(ref ctry_value) = self.ctry { if !ctry_value.validate() { return false; } }
		if let Some(ref adr_line_vec) = self.adr_line { for item in adr_line_vec { if !item.validate() { return false; } } }
		return true
	}
}


// PreferredContactMethod2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PreferredContactMethod2Code {
	#[default]
	#[serde(rename = "MAIL")]
	CodeMAIL,
	#[serde(rename = "FAXX")]
	CodeFAXX,
	#[serde(rename = "LETT")]
	CodeLETT,
	#[serde(rename = "CELL")]
	CodeCELL,
	#[serde(rename = "ONLI")]
	CodeONLI,
	#[serde(rename = "PHON")]
	CodePHON,
}

impl PreferredContactMethod2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ProprietaryBankTransactionCodeStructure1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryBankTransactionCodeStructure1 {
	#[serde(rename = "Cd")]
	pub cd: Max35Text,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl ProprietaryBankTransactionCodeStructure1 {
	pub fn validate(&self) -> bool {
		if !self.cd.validate() { return false }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		return true
	}
}


// ProxyAccountIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProxyAccountIdentification1 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<ProxyAccountType1Choice>,
	#[serde(rename = "Id")]
	pub id: Max2048Text,
}

impl ProxyAccountIdentification1 {
	pub fn validate(&self) -> bool {
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		if !self.id.validate() { return false }
		return true
	}
}


// ProxyAccountType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProxyAccountType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalProxyAccountType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl ProxyAccountType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// ReportHeader6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportHeader6 {
	#[serde(rename = "RptId")]
	pub rpt_id: Max35Text,
	#[serde(rename = "MsgPgntn", skip_serializing_if = "Option::is_none")]
	pub msg_pgntn: Option<Pagination1>,
}

impl ReportHeader6 {
	pub fn validate(&self) -> bool {
		if !self.rpt_id.validate() { return false }
		if let Some(ref msg_pgntn_value) = self.msg_pgntn { if !msg_pgntn_value.validate() { return false; } }
		return true
	}
}


// ResidenceLocation1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ResidenceLocation1Choice {
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
	#[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
	pub area: Option<Max35Text>,
}

impl ResidenceLocation1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref ctry_value) = self.ctry { if !ctry_value.validate() { return false; } }
		if let Some(ref area_value) = self.area { if !area_value.validate() { return false; } }
		return true
	}
}


// ServiceAdjustmentType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ServiceAdjustmentType1Code {
	#[default]
	#[serde(rename = "COMP")]
	CodeCOMP,
	#[serde(rename = "NCMP")]
	CodeNCMP,
}

impl ServiceAdjustmentType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ServicePaymentMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ServicePaymentMethod1Code {
	#[default]
	#[serde(rename = "BCMP")]
	CodeBCMP,
	#[serde(rename = "FLAT")]
	CodeFLAT,
	#[serde(rename = "PVCH")]
	CodePVCH,
	#[serde(rename = "INVS")]
	CodeINVS,
	#[serde(rename = "WVED")]
	CodeWVED,
	#[serde(rename = "FREE")]
	CodeFREE,
}

impl ServicePaymentMethod1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ServiceTaxDesignation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ServiceTaxDesignation1 {
	#[serde(rename = "Cd")]
	pub cd: ServiceTaxDesignation1Code,
	#[serde(rename = "Rgn", skip_serializing_if = "Option::is_none")]
	pub rgn: Option<Max35Text>,
	#[serde(rename = "TaxRsn", skip_serializing_if = "Option::is_none")]
	pub tax_rsn: Option<Vec<TaxReason1>>,
}

impl ServiceTaxDesignation1 {
	pub fn validate(&self) -> bool {
		if !self.cd.validate() { return false }
		if let Some(ref rgn_value) = self.rgn { if !rgn_value.validate() { return false; } }
		if let Some(ref tax_rsn_vec) = self.tax_rsn { for item in tax_rsn_vec { if !item.validate() { return false; } } }
		return true
	}
}


// ServiceTaxDesignation1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ServiceTaxDesignation1Code {
	#[default]
	#[serde(rename = "XMPT")]
	CodeXMPT,
	#[serde(rename = "ZERO")]
	CodeZERO,
	#[serde(rename = "TAXE")]
	CodeTAXE,
}

impl ServiceTaxDesignation1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// StatementGroup5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatementGroup5 {
	#[serde(rename = "GrpId")]
	pub grp_id: Max35Text,
	#[serde(rename = "Sndr")]
	pub sndr: PartyIdentification273,
	#[serde(rename = "SndrIndvCtct", skip_serializing_if = "Option::is_none")]
	pub sndr_indv_ctct: Option<Vec<Contact13>>,
	#[serde(rename = "Rcvr")]
	pub rcvr: PartyIdentification273,
	#[serde(rename = "RcvrIndvCtct", skip_serializing_if = "Option::is_none")]
	pub rcvr_indv_ctct: Option<Vec<Contact13>>,
	#[serde(rename = "BllgStmt")]
	pub bllg_stmt: Vec<BillingStatement5>,
}

impl StatementGroup5 {
	pub fn validate(&self) -> bool {
		if !self.grp_id.validate() { return false }
		if !self.sndr.validate() { return false }
		if let Some(ref sndr_indv_ctct_vec) = self.sndr_indv_ctct { for item in sndr_indv_ctct_vec { if !item.validate() { return false; } } }
		if !self.rcvr.validate() { return false }
		if let Some(ref rcvr_indv_ctct_vec) = self.rcvr_indv_ctct { for item in rcvr_indv_ctct_vec { if !item.validate() { return false; } } }
		for item in &self.bllg_stmt { if !item.validate() { return false; } }
		return true
	}
}


// TaxCalculation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxCalculation1 {
	#[serde(rename = "HstCcy")]
	pub hst_ccy: ActiveOrHistoricCurrencyCode,
	#[serde(rename = "TaxblSvcChrgConvs")]
	pub taxbl_svc_chrg_convs: Vec<BillingServicesAmount3>,
	#[serde(rename = "TtlTaxblSvcChrgHstAmt")]
	pub ttl_taxbl_svc_chrg_hst_amt: AmountAndDirection34,
	#[serde(rename = "TaxId")]
	pub tax_id: Vec<BillingServicesTax3>,
	#[serde(rename = "TtlTax")]
	pub ttl_tax: AmountAndDirection34,
}

impl TaxCalculation1 {
	pub fn validate(&self) -> bool {
		if !self.hst_ccy.validate() { return false }
		for item in &self.taxbl_svc_chrg_convs { if !item.validate() { return false; } }
		if !self.ttl_taxbl_svc_chrg_hst_amt.validate() { return false }
		for item in &self.tax_id { if !item.validate() { return false; } }
		if !self.ttl_tax.validate() { return false }
		return true
	}
}


// TaxReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxReason1 {
	#[serde(rename = "Cd")]
	pub cd: Max10Text,
	#[serde(rename = "Expltn")]
	pub expltn: Max105Text,
}

impl TaxReason1 {
	pub fn validate(&self) -> bool {
		if !self.cd.validate() { return false }
		if !self.expltn.validate() { return false }
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
