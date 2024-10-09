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
	#[serde(rename = "IBAN")]
	pub iban: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<GenericAccountIdentification1>,
}


// AccountLevel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountLevel1Code {
	#[serde(rename = "AccountLevel1Code")]
	pub account_level1_code: String,
}


// AccountLevel2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountLevel2Code {
	#[serde(rename = "AccountLevel2Code")]
	pub account_level2_code: String,
}


// AccountSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// AccountTax1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountTax1 {
	#[serde(rename = "ClctnMtd")]
	pub clctn_mtd: String,
	#[serde(rename = "Rgn")]
	pub rgn: Option<String>,
	#[serde(rename = "NonResCtry")]
	pub non_res_ctry: Option<ResidenceLocation1Choice>,
}


// ActiveOrHistoricCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
	#[serde(rename = "ActiveOrHistoricCurrencyAndAmount_SimpleType")]
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
	#[serde(rename = "ActiveOrHistoricCurrencyCode")]
	pub active_or_historic_currency_code: String,
}


// AddressType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType2Code {
	#[serde(rename = "AddressType2Code")]
	pub address_type2_code: String,
}


// AddressType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
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
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// BICFIDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BICFIDec2014Identifier {
	#[serde(rename = "BICFIDec2014Identifier")]
	pub bicfi_dec2014_identifier: String,
}


// BalanceAdjustment1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BalanceAdjustment1 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "Desc")]
	pub desc: String,
	#[serde(rename = "BalAmt")]
	pub bal_amt: AmountAndDirection34,
	#[serde(rename = "AvrgAmt")]
	pub avrg_amt: Option<AmountAndDirection34>,
	#[serde(rename = "ErrDt")]
	pub err_dt: Option<String>,
	#[serde(rename = "PstngDt")]
	pub pstng_dt: String,
	#[serde(rename = "Days")]
	pub days: Option<f64>,
	#[serde(rename = "EarngsAdjstmntAmt")]
	pub earngs_adjstmnt_amt: Option<AmountAndDirection34>,
}


// BalanceAdjustmentType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BalanceAdjustmentType1Code {
	#[serde(rename = "BalanceAdjustmentType1Code")]
	pub balance_adjustment_type1_code: String,
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
	#[serde(rename = "Domn")]
	pub domn: Option<BankTransactionCodeStructure5>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<ProprietaryBankTransactionCodeStructure1>,
}


// BankTransactionCodeStructure5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BankTransactionCodeStructure5 {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Fmly")]
	pub fmly: BankTransactionCodeStructure6,
}


// BankTransactionCodeStructure6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BankTransactionCodeStructure6 {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "SubFmlyCd")]
	pub sub_fmly_cd: String,
}


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BaseOneRate {
	#[serde(rename = "BaseOneRate")]
	pub base_one_rate: f64,
}


// BillingBalance1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingBalance1 {
	#[serde(rename = "Tp")]
	pub tp: BillingBalanceType1Choice,
	#[serde(rename = "Val")]
	pub val: AmountAndDirection34,
	#[serde(rename = "CcyTp")]
	pub ccy_tp: Option<String>,
}


// BillingBalanceType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingBalanceType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// BillingChargeMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingChargeMethod1Code {
	#[serde(rename = "BillingChargeMethod1Code")]
	pub billing_charge_method1_code: String,
}


// BillingCompensation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingCompensation1 {
	#[serde(rename = "Tp")]
	pub tp: BillingCompensationType1Choice,
	#[serde(rename = "Val")]
	pub val: AmountAndDirection34,
	#[serde(rename = "CcyTp")]
	pub ccy_tp: Option<String>,
}


// BillingCompensationType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingCompensationType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// BillingCurrencyType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingCurrencyType1Code {
	#[serde(rename = "BillingCurrencyType1Code")]
	pub billing_currency_type1_code: String,
}


// BillingCurrencyType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingCurrencyType2Code {
	#[serde(rename = "BillingCurrencyType2Code")]
	pub billing_currency_type2_code: String,
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
	#[serde(rename = "MtdA")]
	pub mtd_a: Option<BillingMethod1>,
	#[serde(rename = "MtdB")]
	pub mtd_b: Option<BillingMethod2>,
	#[serde(rename = "MtdD")]
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
	#[serde(rename = "Ccy")]
	pub ccy: Option<String>,
	#[serde(rename = "UnitPric")]
	pub unit_pric: Option<AmountAndDirection34>,
	#[serde(rename = "Mtd")]
	pub mtd: Option<String>,
	#[serde(rename = "Rule")]
	pub rule: Option<String>,
}


// BillingRate1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingRate1 {
	#[serde(rename = "Id")]
	pub id: BillingRateIdentification1Choice,
	#[serde(rename = "Val")]
	pub val: f64,
	#[serde(rename = "DaysInPrd")]
	pub days_in_prd: Option<f64>,
	#[serde(rename = "DaysInYr")]
	pub days_in_yr: Option<f64>,
}


// BillingRateIdentification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingRateIdentification1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// BillingService2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingService2 {
	#[serde(rename = "SvcDtl")]
	pub svc_dtl: BillingServiceParameters3,
	#[serde(rename = "Pric")]
	pub pric: Option<BillingPrice1>,
	#[serde(rename = "PmtMtd")]
	pub pmt_mtd: String,
	#[serde(rename = "OrgnlChrgPric")]
	pub orgnl_chrg_pric: AmountAndDirection34,
	#[serde(rename = "OrgnlChrgSttlmAmt")]
	pub orgnl_chrg_sttlm_amt: Option<AmountAndDirection34>,
	#[serde(rename = "BalReqrdAcctAmt")]
	pub bal_reqrd_acct_amt: Option<AmountAndDirection34>,
	#[serde(rename = "TaxDsgnt")]
	pub tax_dsgnt: ServiceTaxDesignation1,
	#[serde(rename = "TaxClctn")]
	pub tax_clctn: Option<BillingMethod1Choice>,
}


// BillingServiceAdjustment1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingServiceAdjustment1 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "Desc")]
	pub desc: String,
	#[serde(rename = "Amt")]
	pub amt: AmountAndDirection34,
	#[serde(rename = "BalReqrdAmt")]
	pub bal_reqrd_amt: Option<AmountAndDirection34>,
	#[serde(rename = "ErrDt")]
	pub err_dt: Option<String>,
	#[serde(rename = "AdjstmntId")]
	pub adjstmnt_id: Option<String>,
	#[serde(rename = "SubSvc")]
	pub sub_svc: Option<BillingSubServiceIdentification1>,
	#[serde(rename = "PricChng")]
	pub pric_chng: Option<AmountAndDirection34>,
	#[serde(rename = "OrgnlPric")]
	pub orgnl_pric: Option<AmountAndDirection34>,
	#[serde(rename = "NewPric")]
	pub new_pric: Option<AmountAndDirection34>,
	#[serde(rename = "VolChng")]
	pub vol_chng: Option<f64>,
	#[serde(rename = "OrgnlVol")]
	pub orgnl_vol: Option<f64>,
	#[serde(rename = "NewVol")]
	pub new_vol: Option<f64>,
	#[serde(rename = "OrgnlChrgAmt")]
	pub orgnl_chrg_amt: Option<AmountAndDirection34>,
	#[serde(rename = "NewChrgAmt")]
	pub new_chrg_amt: Option<AmountAndDirection34>,
}


// BillingServiceCommonIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingServiceCommonIdentification1 {
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "Id")]
	pub id: String,
}


// BillingServiceIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingServiceIdentification2 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SubSvc")]
	pub sub_svc: Option<BillingSubServiceIdentification1>,
	#[serde(rename = "Desc")]
	pub desc: String,
}


// BillingServiceIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingServiceIdentification3 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SubSvc")]
	pub sub_svc: Option<BillingSubServiceIdentification1>,
	#[serde(rename = "Desc")]
	pub desc: String,
	#[serde(rename = "CmonCd")]
	pub cmon_cd: Option<BillingServiceCommonIdentification1>,
	#[serde(rename = "BkTxCd")]
	pub bk_tx_cd: Option<BankTransactionCodeStructure4>,
	#[serde(rename = "SvcTp")]
	pub svc_tp: Option<String>,
}


// BillingServiceParameters2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingServiceParameters2 {
	#[serde(rename = "BkSvc")]
	pub bk_svc: BillingServiceIdentification2,
	#[serde(rename = "Vol")]
	pub vol: Option<f64>,
	#[serde(rename = "UnitPric")]
	pub unit_pric: Option<AmountAndDirection34>,
	#[serde(rename = "SvcChrgAmt")]
	pub svc_chrg_amt: AmountAndDirection34,
}


// BillingServiceParameters3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingServiceParameters3 {
	#[serde(rename = "BkSvc")]
	pub bk_svc: BillingServiceIdentification3,
	#[serde(rename = "Vol")]
	pub vol: Option<f64>,
}


// BillingServicesAmount1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingServicesAmount1 {
	#[serde(rename = "HstAmt")]
	pub hst_amt: AmountAndDirection34,
	#[serde(rename = "PricgAmt")]
	pub pricg_amt: Option<AmountAndDirection34>,
}


// BillingServicesAmount2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingServicesAmount2 {
	#[serde(rename = "HstAmt")]
	pub hst_amt: AmountAndDirection34,
	#[serde(rename = "SttlmAmt")]
	pub sttlm_amt: Option<AmountAndDirection34>,
	#[serde(rename = "PricgAmt")]
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
	pub nb: String,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
	#[serde(rename = "Rate")]
	pub rate: f64,
	#[serde(rename = "HstAmt")]
	pub hst_amt: AmountAndDirection34,
	#[serde(rename = "PricgAmt")]
	pub pricg_amt: Option<AmountAndDirection34>,
}


// BillingServicesTax2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingServicesTax2 {
	#[serde(rename = "Nb")]
	pub nb: String,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
	#[serde(rename = "Rate")]
	pub rate: f64,
	#[serde(rename = "PricgAmt")]
	pub pricg_amt: AmountAndDirection34,
}


// BillingServicesTax3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingServicesTax3 {
	#[serde(rename = "Nb")]
	pub nb: String,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
	#[serde(rename = "Rate")]
	pub rate: f64,
	#[serde(rename = "TtlTaxAmt")]
	pub ttl_tax_amt: AmountAndDirection34,
}


// BillingStatement5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingStatement5 {
	#[serde(rename = "StmtId")]
	pub stmt_id: String,
	#[serde(rename = "FrToDt")]
	pub fr_to_dt: DatePeriod1,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
	#[serde(rename = "Sts")]
	pub sts: String,
	#[serde(rename = "AcctChrtcs")]
	pub acct_chrtcs: CashAccountCharacteristics5,
	#[serde(rename = "RateData")]
	pub rate_data: Option<Vec<BillingRate1>>,
	#[serde(rename = "CcyXchg")]
	pub ccy_xchg: Option<Vec<CurrencyExchange6>>,
	#[serde(rename = "Bal")]
	pub bal: Option<Vec<BillingBalance1>>,
	#[serde(rename = "Compstn")]
	pub compstn: Option<Vec<BillingCompensation1>>,
	#[serde(rename = "Svc")]
	pub svc: Option<Vec<BillingService2>>,
	#[serde(rename = "TaxRgn")]
	pub tax_rgn: Option<Vec<BillingTaxRegion3>>,
	#[serde(rename = "BalAdjstmnt")]
	pub bal_adjstmnt: Option<Vec<BalanceAdjustment1>>,
	#[serde(rename = "SvcAdjstmnt")]
	pub svc_adjstmnt: Option<Vec<BillingServiceAdjustment1>>,
}


// BillingStatementStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingStatementStatus1Code {
	#[serde(rename = "BillingStatementStatus1Code")]
	pub billing_statement_status1_code: String,
}


// BillingSubServiceIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingSubServiceIdentification1 {
	#[serde(rename = "Issr")]
	pub issr: BillingSubServiceQualifier1Choice,
	#[serde(rename = "Id")]
	pub id: String,
}


// BillingSubServiceQualifier1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingSubServiceQualifier1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// BillingSubServiceQualifier1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingSubServiceQualifier1Code {
	#[serde(rename = "BillingSubServiceQualifier1Code")]
	pub billing_sub_service_qualifier1_code: String,
}


// BillingTaxCalculationMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingTaxCalculationMethod1Code {
	#[serde(rename = "BillingTaxCalculationMethod1Code")]
	pub billing_tax_calculation_method1_code: String,
}


// BillingTaxIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingTaxIdentification3 {
	#[serde(rename = "VATRegnNb")]
	pub vat_regn_nb: Option<String>,
	#[serde(rename = "TaxRegnNb")]
	pub tax_regn_nb: Option<String>,
	#[serde(rename = "TaxCtct")]
	pub tax_ctct: Option<Contact13>,
}


// BillingTaxRegion3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BillingTaxRegion3 {
	#[serde(rename = "RgnNb")]
	pub rgn_nb: String,
	#[serde(rename = "RgnNm")]
	pub rgn_nm: String,
	#[serde(rename = "CstmrTaxId")]
	pub cstmr_tax_id: String,
	#[serde(rename = "PtDt")]
	pub pt_dt: Option<String>,
	#[serde(rename = "SndgFI")]
	pub sndg_fi: Option<BillingTaxIdentification3>,
	#[serde(rename = "InvcNb")]
	pub invc_nb: Option<String>,
	#[serde(rename = "MtdC")]
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
	#[serde(rename = "BrnchId")]
	pub brnch_id: Option<BranchData5>,
}


// BranchData5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BranchData5 {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress27>,
}


// CashAccount40 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccount40 {
	#[serde(rename = "Id")]
	pub id: Option<AccountIdentification4Choice>,
	#[serde(rename = "Tp")]
	pub tp: Option<CashAccountType2Choice>,
	#[serde(rename = "Ccy")]
	pub ccy: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Prxy")]
	pub prxy: Option<ProxyAccountIdentification1>,
}


// CashAccountCharacteristics5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountCharacteristics5 {
	#[serde(rename = "AcctLvl")]
	pub acct_lvl: String,
	#[serde(rename = "CshAcct")]
	pub csh_acct: CashAccount40,
	#[serde(rename = "AcctSvcr")]
	pub acct_svcr: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "PrntAcct")]
	pub prnt_acct: Option<ParentCashAccount5>,
	#[serde(rename = "CompstnMtd")]
	pub compstn_mtd: String,
	#[serde(rename = "DbtAcct")]
	pub dbt_acct: Option<AccountIdentification4Choice>,
	#[serde(rename = "DelydDbtDt")]
	pub delyd_dbt_dt: Option<String>,
	#[serde(rename = "SttlmAdvc")]
	pub sttlm_advc: Option<String>,
	#[serde(rename = "AcctBalCcyCd")]
	pub acct_bal_ccy_cd: String,
	#[serde(rename = "SttlmCcyCd")]
	pub sttlm_ccy_cd: Option<String>,
	#[serde(rename = "HstCcyCd")]
	pub hst_ccy_cd: Option<String>,
	#[serde(rename = "Tax")]
	pub tax: Option<AccountTax1>,
	#[serde(rename = "AcctSvcrCtct")]
	pub acct_svcr_ctct: Contact13,
}


// CashAccountType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountType2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ClearingSystemIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemIdentification2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ClearingSystemMemberIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemMemberIdentification2 {
	#[serde(rename = "ClrSysId")]
	pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
	#[serde(rename = "MmbId")]
	pub mmb_id: String,
}


// CompensationMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompensationMethod1Code {
	#[serde(rename = "CompensationMethod1Code")]
	pub compensation_method1_code: String,
}


// Contact13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Contact13 {
	#[serde(rename = "NmPrfx")]
	pub nm_prfx: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "PhneNb")]
	pub phne_nb: Option<String>,
	#[serde(rename = "MobNb")]
	pub mob_nb: Option<String>,
	#[serde(rename = "FaxNb")]
	pub fax_nb: Option<String>,
	#[serde(rename = "URLAdr")]
	pub url_adr: Option<String>,
	#[serde(rename = "EmailAdr")]
	pub email_adr: Option<String>,
	#[serde(rename = "EmailPurp")]
	pub email_purp: Option<String>,
	#[serde(rename = "JobTitl")]
	pub job_titl: Option<String>,
	#[serde(rename = "Rspnsblty")]
	pub rspnsblty: Option<String>,
	#[serde(rename = "Dept")]
	pub dept: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<OtherContact1>>,
	#[serde(rename = "PrefrdMtd")]
	pub prefrd_mtd: Option<String>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// CurrencyExchange6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CurrencyExchange6 {
	#[serde(rename = "SrcCcy")]
	pub src_ccy: String,
	#[serde(rename = "TrgtCcy")]
	pub trgt_ccy: String,
	#[serde(rename = "XchgRate")]
	pub xchg_rate: f64,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
	#[serde(rename = "UnitCcy")]
	pub unit_ccy: Option<String>,
	#[serde(rename = "Cmnts")]
	pub cmnts: Option<String>,
	#[serde(rename = "QtnDt")]
	pub qtn_dt: Option<String>,
}


// DatePeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriod1 {
	#[serde(rename = "FrDt")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "DecimalNumber")]
	pub decimal_number: f64,
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "Exact4AlphaNumericText")]
	pub exact4_alpha_numeric_text: String,
}


// ExternalAccountIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAccountIdentification1Code {
	#[serde(rename = "ExternalAccountIdentification1Code")]
	pub external_account_identification1_code: String,
}


// ExternalBankTransactionDomain1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalBankTransactionDomain1Code {
	#[serde(rename = "ExternalBankTransactionDomain1Code")]
	pub external_bank_transaction_domain1_code: String,
}


// ExternalBankTransactionFamily1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalBankTransactionFamily1Code {
	#[serde(rename = "ExternalBankTransactionFamily1Code")]
	pub external_bank_transaction_family1_code: String,
}


// ExternalBankTransactionSubFamily1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalBankTransactionSubFamily1Code {
	#[serde(rename = "ExternalBankTransactionSubFamily1Code")]
	pub external_bank_transaction_sub_family1_code: String,
}


// ExternalBillingBalanceType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalBillingBalanceType1Code {
	#[serde(rename = "ExternalBillingBalanceType1Code")]
	pub external_billing_balance_type1_code: String,
}


// ExternalBillingCompensationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalBillingCompensationType1Code {
	#[serde(rename = "ExternalBillingCompensationType1Code")]
	pub external_billing_compensation_type1_code: String,
}


// ExternalBillingRateIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalBillingRateIdentification1Code {
	#[serde(rename = "ExternalBillingRateIdentification1Code")]
	pub external_billing_rate_identification1_code: String,
}


// ExternalCashAccountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalCashAccountType1Code {
	#[serde(rename = "ExternalCashAccountType1Code")]
	pub external_cash_account_type1_code: String,
}


// ExternalClearingSystemIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalClearingSystemIdentification1Code {
	#[serde(rename = "ExternalClearingSystemIdentification1Code")]
	pub external_clearing_system_identification1_code: String,
}


// ExternalFinancialInstitutionIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalFinancialInstitutionIdentification1Code {
	#[serde(rename = "ExternalFinancialInstitutionIdentification1Code")]
	pub external_financial_institution_identification1_code: String,
}


// ExternalOrganisationIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalOrganisationIdentification1Code {
	#[serde(rename = "ExternalOrganisationIdentification1Code")]
	pub external_organisation_identification1_code: String,
}


// ExternalProxyAccountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalProxyAccountType1Code {
	#[serde(rename = "ExternalProxyAccountType1Code")]
	pub external_proxy_account_type1_code: String,
}


// FinancialIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// FinancialInstitutionIdentification19 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstitutionIdentification19 {
	#[serde(rename = "BICFI")]
	pub bicfi: Option<String>,
	#[serde(rename = "ClrSysMmbId")]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<GenericFinancialIdentification1>,
}


// FinancialInstitutionIdentification23 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstitutionIdentification23 {
	#[serde(rename = "BICFI")]
	pub bicfi: Option<String>,
	#[serde(rename = "ClrSysMmbId")]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress27>,
	#[serde(rename = "Othr")]
	pub othr: Option<GenericFinancialIdentification1>,
}


// GenericAccountIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericAccountIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<AccountSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericFinancialIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericFinancialIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<FinancialIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericIdentification30 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification30 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// GenericOrganisationIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericOrganisationIdentification3 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// IBAN2007Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IBAN2007Identifier {
	#[serde(rename = "IBAN2007Identifier")]
	pub iban2007_identifier: String,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// Max105Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max105Text {
	#[serde(rename = "Max105Text")]
	pub max105_text: String,
}


// Max10Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max10Text {
	#[serde(rename = "Max10Text")]
	pub max10_text: String,
}


// Max128Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max128Text {
	#[serde(rename = "Max128Text")]
	pub max128_text: String,
}


// Max12Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max12Text {
	#[serde(rename = "Max12Text")]
	pub max12_text: String,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max16Text {
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
}


// Max2048Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max2048Text {
	#[serde(rename = "Max2048Text")]
	pub max2048_text: String,
}


// Max20Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max20Text {
	#[serde(rename = "Max20Text")]
	pub max20_text: String,
}


// Max256Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max256Text {
	#[serde(rename = "Max256Text")]
	pub max256_text: String,
}


// Max34Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max34Text {
	#[serde(rename = "Max34Text")]
	pub max34_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Max40Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max40Text {
	#[serde(rename = "Max40Text")]
	pub max40_text: String,
}


// Max4Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4Text {
	#[serde(rename = "Max4Text")]
	pub max4_text: String,
}


// Max5NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max5NumericText {
	#[serde(rename = "Max5NumericText")]
	pub max5_numeric_text: String,
}


// Max6Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max6Text {
	#[serde(rename = "Max6Text")]
	pub max6_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// Max8Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max8Text {
	#[serde(rename = "Max8Text")]
	pub max8_text: String,
}


// NamePrefix2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NamePrefix2Code {
	#[serde(rename = "NamePrefix2Code")]
	pub name_prefix2_code: String,
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
}


// OrganisationIdentification39 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification39 {
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<GenericOrganisationIdentification3>>,
}


// OrganisationIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// OtherContact1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherContact1 {
	#[serde(rename = "ChanlTp")]
	pub chanl_tp: String,
	#[serde(rename = "Id")]
	pub id: Option<String>,
}


// Pagination1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Pagination1 {
	#[serde(rename = "PgNb")]
	pub pg_nb: String,
	#[serde(rename = "LastPgInd")]
	pub last_pg_ind: bool,
}


// ParentCashAccount5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ParentCashAccount5 {
	#[serde(rename = "Lvl")]
	pub lvl: Option<String>,
	#[serde(rename = "Id")]
	pub id: CashAccount40,
	#[serde(rename = "Svcr")]
	pub svcr: Option<BranchAndFinancialInstitutionIdentification8>,
}


// Party56Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party56Choice {
	#[serde(rename = "OrgId")]
	pub org_id: Option<OrganisationIdentification39>,
	#[serde(rename = "FIId")]
	pub fi_id: Option<FinancialInstitutionIdentification19>,
}


// PartyIdentification273 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification273 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "LglNm")]
	pub lgl_nm: Option<String>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress27>,
	#[serde(rename = "Id")]
	pub id: Party56Choice,
	#[serde(rename = "CtryOfRes")]
	pub ctry_of_res: Option<String>,
	#[serde(rename = "CtctDtls")]
	pub ctct_dtls: Option<Contact13>,
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// PhoneNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PhoneNumber {
	#[serde(rename = "PhoneNumber")]
	pub phone_number: String,
}


// PlusOrMinusIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "PlusOrMinusIndicator")]
	pub plus_or_minus_indicator: bool,
}


// PostalAddress27 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress27 {
	#[serde(rename = "AdrTp")]
	pub adr_tp: Option<AddressType3Choice>,
	#[serde(rename = "CareOf")]
	pub care_of: Option<String>,
	#[serde(rename = "Dept")]
	pub dept: Option<String>,
	#[serde(rename = "SubDept")]
	pub sub_dept: Option<String>,
	#[serde(rename = "StrtNm")]
	pub strt_nm: Option<String>,
	#[serde(rename = "BldgNb")]
	pub bldg_nb: Option<String>,
	#[serde(rename = "BldgNm")]
	pub bldg_nm: Option<String>,
	#[serde(rename = "Flr")]
	pub flr: Option<String>,
	#[serde(rename = "UnitNb")]
	pub unit_nb: Option<String>,
	#[serde(rename = "PstBx")]
	pub pst_bx: Option<String>,
	#[serde(rename = "Room")]
	pub room: Option<String>,
	#[serde(rename = "PstCd")]
	pub pst_cd: Option<String>,
	#[serde(rename = "TwnNm")]
	pub twn_nm: Option<String>,
	#[serde(rename = "TwnLctnNm")]
	pub twn_lctn_nm: Option<String>,
	#[serde(rename = "DstrctNm")]
	pub dstrct_nm: Option<String>,
	#[serde(rename = "CtrySubDvsn")]
	pub ctry_sub_dvsn: Option<String>,
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
	#[serde(rename = "AdrLine")]
	pub adr_line: Option<Vec<String>>,
}


// PreferredContactMethod2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PreferredContactMethod2Code {
	#[serde(rename = "PreferredContactMethod2Code")]
	pub preferred_contact_method2_code: String,
}


// ProprietaryBankTransactionCodeStructure1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryBankTransactionCodeStructure1 {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// ProxyAccountIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProxyAccountIdentification1 {
	#[serde(rename = "Tp")]
	pub tp: Option<ProxyAccountType1Choice>,
	#[serde(rename = "Id")]
	pub id: String,
}


// ProxyAccountType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProxyAccountType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ReportHeader6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportHeader6 {
	#[serde(rename = "RptId")]
	pub rpt_id: String,
	#[serde(rename = "MsgPgntn")]
	pub msg_pgntn: Option<Pagination1>,
}


// ResidenceLocation1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ResidenceLocation1Choice {
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
	#[serde(rename = "Area")]
	pub area: Option<String>,
}


// ServiceAdjustmentType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ServiceAdjustmentType1Code {
	#[serde(rename = "ServiceAdjustmentType1Code")]
	pub service_adjustment_type1_code: String,
}


// ServicePaymentMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ServicePaymentMethod1Code {
	#[serde(rename = "ServicePaymentMethod1Code")]
	pub service_payment_method1_code: String,
}


// ServiceTaxDesignation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ServiceTaxDesignation1 {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Rgn")]
	pub rgn: Option<String>,
	#[serde(rename = "TaxRsn")]
	pub tax_rsn: Option<Vec<TaxReason1>>,
}


// ServiceTaxDesignation1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ServiceTaxDesignation1Code {
	#[serde(rename = "ServiceTaxDesignation1Code")]
	pub service_tax_designation1_code: String,
}


// StatementGroup5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatementGroup5 {
	#[serde(rename = "GrpId")]
	pub grp_id: String,
	#[serde(rename = "Sndr")]
	pub sndr: PartyIdentification273,
	#[serde(rename = "SndrIndvCtct")]
	pub sndr_indv_ctct: Option<Vec<Contact13>>,
	#[serde(rename = "Rcvr")]
	pub rcvr: PartyIdentification273,
	#[serde(rename = "RcvrIndvCtct")]
	pub rcvr_indv_ctct: Option<Vec<Contact13>>,
	#[serde(rename = "BllgStmt")]
	pub bllg_stmt: Vec<BillingStatement5>,
}


// TaxCalculation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxCalculation1 {
	#[serde(rename = "HstCcy")]
	pub hst_ccy: String,
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
	pub cd: String,
	#[serde(rename = "Expltn")]
	pub expltn: String,
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "YesNoIndicator")]
	pub yes_no_indicator: bool,
}
