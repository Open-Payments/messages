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


// AccountIdentification4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountIdentification4Choice {
	#[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
	pub iban: Option<IBAN2007Identifier>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<GenericAccountIdentification1>,
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


// AccountSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalAccountIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
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


// ActiveOrHistoricCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_and_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_code: String,
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


// AddressType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<AddressType2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// AmountAndDirection34 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection34 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "Sgn")]
	pub sgn: bool,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
}


// BICFIDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BICFIDec2014Identifier {
	#[serde(rename = "$value")]
	pub bicfi_dec2014_identifier: String,
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


// BankServicesBillingStatementV05 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BankServicesBillingStatementV05 {
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: ReportHeader6,
	#[serde(rename = "BllgStmtGrp")]
	pub bllg_stmt_grp: Vec<StatementGroup5>,
}


// BankTransactionCodeStructure4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BankTransactionCodeStructure4 {
	#[serde(rename = "Domn", skip_serializing_if = "Option::is_none")]
	pub domn: Option<BankTransactionCodeStructure5>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<ProprietaryBankTransactionCodeStructure1>,
}


// BankTransactionCodeStructure5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BankTransactionCodeStructure5 {
	#[serde(rename = "Cd")]
	pub cd: ExternalBankTransactionDomain1Code,
	#[serde(rename = "Fmly")]
	pub fmly: BankTransactionCodeStructure6,
}


// BankTransactionCodeStructure6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BankTransactionCodeStructure6 {
	#[serde(rename = "Cd")]
	pub cd: ExternalBankTransactionFamily1Code,
	#[serde(rename = "SubFmlyCd")]
	pub sub_fmly_cd: ExternalBankTransactionSubFamily1Code,
}


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BaseOneRate {
	#[serde(rename = "$value")]
	pub base_one_rate: f64,
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


// BillingBalanceType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingBalanceType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalBillingBalanceType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
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


// BillingCompensationType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingCompensationType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalBillingCompensationType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
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


// BillingMethod3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingMethod3 {
	#[serde(rename = "SvcTaxPricAmt")]
	pub svc_tax_pric_amt: AmountAndDirection34,
	#[serde(rename = "TaxId")]
	pub tax_id: Vec<BillingServicesTax2>,
}


// BillingMethod4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingMethod4 {
	#[serde(rename = "SvcDtl")]
	pub svc_dtl: Vec<BillingServiceParameters2>,
	#[serde(rename = "TaxClctn")]
	pub tax_clctn: TaxCalculation1,
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


// BillingRateIdentification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingRateIdentification1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalBillingRateIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
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


// BillingServiceCommonIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingServiceCommonIdentification1 {
	#[serde(rename = "Issr")]
	pub issr: Max6Text,
	#[serde(rename = "Id")]
	pub id: Max8Text,
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


// BillingServiceParameters3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingServiceParameters3 {
	#[serde(rename = "BkSvc")]
	pub bk_svc: BillingServiceIdentification3,
	#[serde(rename = "Vol", skip_serializing_if = "Option::is_none")]
	pub vol: Option<f64>,
}


// BillingServicesAmount1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingServicesAmount1 {
	#[serde(rename = "HstAmt")]
	pub hst_amt: AmountAndDirection34,
	#[serde(rename = "PricgAmt", skip_serializing_if = "Option::is_none")]
	pub pricg_amt: Option<AmountAndDirection34>,
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


// BillingServicesAmount3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingServicesAmount3 {
	#[serde(rename = "SrcAmt")]
	pub src_amt: AmountAndDirection34,
	#[serde(rename = "HstAmt")]
	pub hst_amt: AmountAndDirection34,
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


// BillingSubServiceIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingSubServiceIdentification1 {
	#[serde(rename = "Issr")]
	pub issr: BillingSubServiceQualifier1Choice,
	#[serde(rename = "Id")]
	pub id: Max35Text,
}


// BillingSubServiceQualifier1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingSubServiceQualifier1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<BillingSubServiceQualifier1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
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


// BranchAndFinancialInstitutionIdentification8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BranchAndFinancialInstitutionIdentification8 {
	#[serde(rename = "FinInstnId")]
	pub fin_instn_id: FinancialInstitutionIdentification23,
	#[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
	pub brnch_id: Option<BranchData5>,
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


// CashAccountType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalCashAccountType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// ClearingSystemIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemIdentification2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalClearingSystemIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// ClearingSystemMemberIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemMemberIdentification2 {
	#[serde(rename = "ClrSysId", skip_serializing_if = "Option::is_none")]
	pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
	#[serde(rename = "MmbId")]
	pub mmb_id: Max35Text,
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


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
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


// DatePeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriod1 {
	#[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "$value")]
	pub decimal_number: f64,
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "$value")]
	pub exact4_alpha_numeric_text: String,
}


// ExternalAccountIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAccountIdentification1Code {
	#[serde(rename = "$value")]
	pub external_account_identification1_code: String,
}


// ExternalBankTransactionDomain1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalBankTransactionDomain1Code {
	#[serde(rename = "$value")]
	pub external_bank_transaction_domain1_code: String,
}


// ExternalBankTransactionFamily1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalBankTransactionFamily1Code {
	#[serde(rename = "$value")]
	pub external_bank_transaction_family1_code: String,
}


// ExternalBankTransactionSubFamily1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalBankTransactionSubFamily1Code {
	#[serde(rename = "$value")]
	pub external_bank_transaction_sub_family1_code: String,
}


// ExternalBillingBalanceType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalBillingBalanceType1Code {
	#[serde(rename = "$value")]
	pub external_billing_balance_type1_code: String,
}


// ExternalBillingCompensationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalBillingCompensationType1Code {
	#[serde(rename = "$value")]
	pub external_billing_compensation_type1_code: String,
}


// ExternalBillingRateIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalBillingRateIdentification1Code {
	#[serde(rename = "$value")]
	pub external_billing_rate_identification1_code: String,
}


// ExternalCashAccountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalCashAccountType1Code {
	#[serde(rename = "$value")]
	pub external_cash_account_type1_code: String,
}


// ExternalClearingSystemIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalClearingSystemIdentification1Code {
	#[serde(rename = "$value")]
	pub external_clearing_system_identification1_code: String,
}


// ExternalFinancialInstitutionIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalFinancialInstitutionIdentification1Code {
	#[serde(rename = "$value")]
	pub external_financial_institution_identification1_code: String,
}


// ExternalOrganisationIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalOrganisationIdentification1Code {
	#[serde(rename = "$value")]
	pub external_organisation_identification1_code: String,
}


// ExternalProxyAccountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalProxyAccountType1Code {
	#[serde(rename = "$value")]
	pub external_proxy_account_type1_code: String,
}


// FinancialIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalFinancialInstitutionIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
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


// IBAN2007Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IBAN2007Identifier {
	#[serde(rename = "$value")]
	pub iban2007_identifier: String,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}


// Max105Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max105Text {
	#[serde(rename = "$value")]
	pub max105_text: String,
}


// Max10Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max10Text {
	#[serde(rename = "$value")]
	pub max10_text: String,
}


// Max128Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max128Text {
	#[serde(rename = "$value")]
	pub max128_text: String,
}


// Max12Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max12Text {
	#[serde(rename = "$value")]
	pub max12_text: String,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
}


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max16Text {
	#[serde(rename = "$value")]
	pub max16_text: String,
}


// Max2048Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max2048Text {
	#[serde(rename = "$value")]
	pub max2048_text: String,
}


// Max20Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max20Text {
	#[serde(rename = "$value")]
	pub max20_text: String,
}


// Max256Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max256Text {
	#[serde(rename = "$value")]
	pub max256_text: String,
}


// Max34Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max34Text {
	#[serde(rename = "$value")]
	pub max34_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}


// Max40Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max40Text {
	#[serde(rename = "$value")]
	pub max40_text: String,
}


// Max4Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4Text {
	#[serde(rename = "$value")]
	pub max4_text: String,
}


// Max5NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max5NumericText {
	#[serde(rename = "$value")]
	pub max5_numeric_text: String,
}


// Max6Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max6Text {
	#[serde(rename = "$value")]
	pub max6_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
}


// Max8Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max8Text {
	#[serde(rename = "$value")]
	pub max8_text: String,
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


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "$value")]
	pub number: f64,
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


// OrganisationIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalOrganisationIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// OtherContact1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherContact1 {
	#[serde(rename = "ChanlTp")]
	pub chanl_tp: Max4Text,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max128Text>,
}


// Pagination1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Pagination1 {
	#[serde(rename = "PgNb")]
	pub pg_nb: Max5NumericText,
	#[serde(rename = "LastPgInd")]
	pub last_pg_ind: bool,
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


// Party56Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party56Choice {
	#[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
	pub org_id: Option<OrganisationIdentification39>,
	#[serde(rename = "FIId", skip_serializing_if = "Option::is_none")]
	pub fi_id: Option<FinancialInstitutionIdentification19>,
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


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "$value")]
	pub percentage_rate: f64,
}


// PhoneNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PhoneNumber {
	#[serde(rename = "$value")]
	pub phone_number: String,
}


// PlusOrMinusIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "$value")]
	pub plus_or_minus_indicator: bool,
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


// ProprietaryBankTransactionCodeStructure1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryBankTransactionCodeStructure1 {
	#[serde(rename = "Cd")]
	pub cd: Max35Text,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}


// ProxyAccountIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProxyAccountIdentification1 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<ProxyAccountType1Choice>,
	#[serde(rename = "Id")]
	pub id: Max2048Text,
}


// ProxyAccountType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProxyAccountType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalProxyAccountType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// ReportHeader6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportHeader6 {
	#[serde(rename = "RptId")]
	pub rpt_id: Max35Text,
	#[serde(rename = "MsgPgntn", skip_serializing_if = "Option::is_none")]
	pub msg_pgntn: Option<Pagination1>,
}


// ResidenceLocation1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ResidenceLocation1Choice {
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
	#[serde(rename = "Area", skip_serializing_if = "Option::is_none")]
	pub area: Option<Max35Text>,
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


// TaxReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxReason1 {
	#[serde(rename = "Cd")]
	pub cd: Max10Text,
	#[serde(rename = "Expltn")]
	pub expltn: Max105Text,
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "$value")]
	pub yes_no_indicator: bool,
}
