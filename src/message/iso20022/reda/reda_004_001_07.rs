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
use serde_valid::Validate;


// AccountIdentificationAndName7 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountIdentificationAndName7 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: CashAccountIdentification8Choice,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
}


// AccountSchemeName1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ActiveCurrencyAnd13DecimalAmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd13DecimalAmountSimpleType {
	#[serde(rename = "ActiveCurrencyAnd13DecimalAmount_SimpleType")]
	pub active_currency_and13_decimal_amount_simple_type: f64,
}


// ActiveCurrencyAnd13DecimalAmount ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd13DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "ActiveCurrencyAndAmount_SimpleType")]
	pub active_currency_and_amount_simple_type: f64,
}


// ActiveCurrencyAndAmount ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveCurrencyCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[validate(pattern = "[A-Z]{3,3}")]
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// AdditionalInformation15 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AdditionalInformation15 {
	#[validate]
	#[serde(rename = "InfTp")]
	pub inf_tp: GenericIdentification36,
	#[serde(rename = "InfVal")]
	pub inf_val: String,
}


// AdditionalProductInformation3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AdditionalProductInformation3 {
	#[serde(rename = "FinInstrmTxCostsExAnteUK")]
	pub fin_instrm_tx_costs_ex_ante_uk: Option<f64>,
	#[serde(rename = "FinInstrmTxCostsExPstUK")]
	pub fin_instrm_tx_costs_ex_pst_uk: Option<f64>,
}


// AdditionalReference10 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AdditionalReference10 {
	#[serde(rename = "Ref")]
	pub ref_attr: String,
	#[validate]
	#[serde(rename = "RefIssr")]
	pub ref_issr: Option<PartyIdentification139>,
	#[serde(rename = "MsgNm")]
	pub msg_nm: Option<String>,
}


// AddressType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AddressType2Code {
	#[validate(enumerate = ["ADDR", "PBOX", "HOME", "BIZZ", "MLTO", "DLVY"])]
	#[serde(rename = "AddressType2Code")]
	pub address_type2_code: String,
}


// AnnualChargePaymentType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AnnualChargePaymentType1Code {
	#[validate(enumerate = ["CAPL", "INCO"])]
	#[serde(rename = "AnnualChargePaymentType1Code")]
	pub annual_charge_payment_type1_code: String,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[validate(pattern = "[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}")]
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// AssessmentOfValueRequiredUnderCOLLUKType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssessmentOfValueRequiredUnderCOLLUKType1Code {
	#[validate(enumerate = ["YSCO", "NSCO"])]
	#[serde(rename = "AssessmentOfValueRequiredUnderCOLLUKType1Code")]
	pub assessment_of_value_required_under_colluk_type1_code: String,
}


// BusinessDayConvention1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BusinessDayConvention1Code {
	#[validate(enumerate = ["FWNG", "PREC"])]
	#[serde(rename = "BusinessDayConvention1Code")]
	pub business_day_convention1_code: String,
}


// CFIOct2015Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[validate(pattern = "[A-Z]{6,6}")]
	#[serde(rename = "CFIOct2015Identifier")]
	pub cfi_oct2015_identifier: String,
}


// CashAccount205 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CashAccount205 {
	#[serde(rename = "Ccy")]
	pub ccy: Option<String>,
	#[validate]
	#[serde(rename = "PmryAcct")]
	pub pmry_acct: Option<CashAccount206>,
	#[validate]
	#[serde(rename = "ScndryAcct")]
	pub scndry_acct: Option<CashAccount206>,
}


// CashAccount206 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CashAccount206 {
	#[validate]
	#[serde(rename = "AcctId")]
	pub acct_id: AccountIdentificationAndName7,
	#[serde(rename = "Svcr")]
	pub svcr: Option<String>,
	#[serde(rename = "AcctTpDesc")]
	pub acct_tp_desc: Option<String>,
}


// CashAccountIdentification8Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CashAccountIdentification8Choice {
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<GenericAccountIdentification1>,
	#[serde(rename = "IBAN")]
	pub iban: Option<String>,
}


// ChargeType8Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ChargeType8Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// ContactAttributes5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ContactAttributes5 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[validate]
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress1>,
	#[serde(rename = "PhneNb")]
	pub phne_nb: Option<String>,
	#[serde(rename = "FaxNb")]
	pub fax_nb: Option<String>,
	#[serde(rename = "EmailAdr")]
	pub email_adr: Option<String>,
	#[serde(rename = "URLAdr")]
	pub url_adr: Option<String>,
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
}


// ContactAttributes6 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ContactAttributes6 {
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[validate]
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress1>,
	#[serde(rename = "PhneNb")]
	pub phne_nb: Option<String>,
	#[serde(rename = "FaxNb")]
	pub fax_nb: Option<String>,
	#[serde(rename = "EmailAdr")]
	pub email_adr: Option<String>,
	#[serde(rename = "URLAdr")]
	pub url_adr: Option<String>,
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
}


// CostsAndCharges2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CostsAndCharges2 {
	#[serde(rename = "ExAnteRefDt")]
	pub ex_ante_ref_dt: Option<String>,
	#[validate]
	#[serde(rename = "IndvCostOrChrg")]
	pub indv_cost_or_chrg: Vec<IndividualCostOrCharge2>,
	#[validate]
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<AdditionalInformation15>,
}


// CountryCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CountryCode {
	#[validate(pattern = "[A-Z]{2,2}")]
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// DecimalNumber ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "DecimalNumber")]
	pub decimal_number: f64,
}


// DistributionPolicy1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DistributionPolicy1Code {
	#[validate(enumerate = ["DIST", "ACCU"])]
	#[serde(rename = "DistributionPolicy1Code")]
	pub distribution_policy1_code: String,
}


// DistributionStrategy1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DistributionStrategy1 {
	#[validate]
	#[serde(rename = "ExctnOnly")]
	pub exctn_only: Option<DistributionStrategy1Choice>,
	#[validate]
	#[serde(rename = "ExctnWthApprprtnssTstOrNonAdvsdSvcs")]
	pub exctn_wth_apprprtnss_tst_or_non_advsd_svcs: Option<DistributionStrategy1Choice>,
	#[validate]
	#[serde(rename = "InvstmtAdvc")]
	pub invstmt_advc: Option<DistributionStrategy1Choice>,
	#[validate]
	#[serde(rename = "PrtflMgmt")]
	pub prtfl_mgmt: Option<DistributionStrategy1Choice>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<OtherDistributionStrategy1>,
}


// DistributionStrategy1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DistributionStrategy1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// DividendPolicy1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DividendPolicy1Code {
	#[validate(enumerate = ["CASH", "UNIT", "BOTH"])]
	#[serde(rename = "DividendPolicy1Code")]
	pub dividend_policy1_code: String,
}


// EMTDataReportingVFMUKType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EMTDataReportingVFMUKType1Code {
	#[validate(enumerate = ["YSCO"])]
	#[serde(rename = "EMTDataReportingVFMUKType1Code")]
	pub emt_data_reporting_vfmuk_type1_code: String,
}


// EUSavingsDirective1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EUSavingsDirective1Code {
	#[validate(enumerate = ["EUSI", "EUSO", "VARI"])]
	#[serde(rename = "EUSavingsDirective1Code")]
	pub eu_savings_directive1_code: String,
}


// EventFrequency5Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EventFrequency5Code {
	#[validate(enumerate = ["YEAR", "SEMI", "QUTR", "MNTH", "WEEK", "DAIL", "CLOS", "TOMN", "TOWK", "TWMN"])]
	#[serde(rename = "EventFrequency5Code")]
	pub event_frequency5_code: String,
}


// EventFrequency8Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EventFrequency8Code {
	#[validate(enumerate = ["ADHO", "YEAR", "DAIL", "FOMN", "TOMN", "TOWK", "TYEA", "INDA", "MNTH", "ONDE", "OVNG", "QUTR", "SEMI", "TWMN", "WEEK"])]
	#[serde(rename = "EventFrequency8Code")]
	pub event_frequency8_code: String,
}


// ExPostCostCalculationBasis1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExPostCostCalculationBasis1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// ExPostCostCalculationBasis1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExPostCostCalculationBasis1Code {
	#[validate(enumerate = ["FIXB", "ROLL"])]
	#[serde(rename = "ExPostCostCalculationBasis1Code")]
	pub ex_post_cost_calculation_basis1_code: String,
}


// Exact4AlphaNumericText ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[validate(pattern = "[a-zA-Z0-9]{4}")]
	#[serde(rename = "Exact4AlphaNumericText")]
	pub exact4_alpha_numeric_text: String,
}


// ExtendedParty13 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExtendedParty13 {
	#[validate]
	#[serde(rename = "PtyRole")]
	pub pty_role: GenericIdentification36,
	#[validate]
	#[serde(rename = "OthrPtyDtls")]
	pub othr_pty_dtls: ContactAttributes5,
}


// Extension1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Extension1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: String,
	#[serde(rename = "Txt")]
	pub txt: String,
}


// ExternalAccountIdentification1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalAccountIdentification1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalAccountIdentification1Code")]
	pub external_account_identification1_code: String,
}


// ExternalFinancialInstrumentIdentificationType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalFinancialInstrumentIdentificationType1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalFinancialInstrumentIdentificationType1Code")]
	pub external_financial_instrument_identification_type1_code: String,
}


// FinancialInstrument96 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstrument96 {
	#[serde(rename = "PhysBrScties")]
	pub phys_br_scties: Option<bool>,
	#[serde(rename = "DmtrlsdBrScties")]
	pub dmtrlsd_br_scties: Option<bool>,
	#[serde(rename = "PhysRegdScties")]
	pub phys_regd_scties: Option<bool>,
	#[serde(rename = "DmtrlsdRegdScties")]
	pub dmtrlsd_regd_scties: Option<bool>,
	#[serde(rename = "DstrbtnPlcy")]
	pub dstrbtn_plcy: Option<String>,
	#[serde(rename = "DvddPlcy")]
	pub dvdd_plcy: Option<String>,
	#[serde(rename = "DvddFrqcy")]
	pub dvdd_frqcy: Option<String>,
	#[serde(rename = "RinvstmtFrqcy")]
	pub rinvstmt_frqcy: Option<String>,
	#[serde(rename = "FrntEndLd")]
	pub frnt_end_ld: Option<bool>,
	#[serde(rename = "BckEndLd")]
	pub bck_end_ld: Option<bool>,
	#[serde(rename = "SwtchFee")]
	pub swtch_fee: Option<bool>,
	#[serde(rename = "EUSvgsDrctv")]
	pub eu_svgs_drctv: Option<String>,
	#[serde(rename = "LnchDt")]
	pub lnch_dt: Option<String>,
	#[serde(rename = "FndEndDt")]
	pub fnd_end_dt: Option<String>,
	#[serde(rename = "TermntnDt")]
	pub termntn_dt: Option<String>,
	#[serde(rename = "InitlOfferEndDt")]
	pub initl_offer_end_dt: Option<String>,
	#[serde(rename = "SspnsnStartDt")]
	pub sspnsn_start_dt: Option<String>,
	#[serde(rename = "SspnsnEndDt")]
	pub sspnsn_end_dt: Option<String>,
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "MayBeTermntdEarly")]
	pub may_be_termntd_early: Option<String>,
	#[serde(rename = "ClsdEndFnd")]
	pub clsd_end_fnd: Option<bool>,
	#[serde(rename = "Equlstn")]
	pub equlstn: Option<bool>,
	#[serde(rename = "TaxEffcntPdctElgbl")]
	pub tax_effcnt_pdct_elgbl: Option<bool>,
	#[serde(rename = "Authrsd")]
	pub authrsd: Option<bool>,
	#[serde(rename = "RDRCmplnt")]
	pub rdr_cmplnt: Option<bool>,
	#[serde(rename = "MgmtFeeSrc")]
	pub mgmt_fee_src: Option<String>,
	#[serde(rename = "PrfrmncFee")]
	pub prfrmnc_fee: Option<bool>,
	#[validate]
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}


// Forms1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Forms1 {
	#[serde(rename = "ApplForm")]
	pub appl_form: bool,
	#[serde(rename = "SgntrTp")]
	pub sgntr_tp: String,
}


// Frequency20Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Frequency20Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// FundOrderType10Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FundOrderType10Code {
	#[validate(enumerate = ["SUBS", "RDIV", "REDM", "RGSV", "WIDP"])]
	#[serde(rename = "FundOrderType10Code")]
	pub fund_order_type10_code: String,
}


// FundOrderType5Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FundOrderType5Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification36>,
}


// FundParties1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FundParties1 {
	#[validate]
	#[serde(rename = "Guarntr")]
	pub guarntr: Option<ContactAttributes5>,
	#[validate]
	#[serde(rename = "Audtr")]
	pub audtr: Option<ContactAttributes5>,
	#[validate]
	#[serde(rename = "Trstee")]
	pub trstee: Option<ContactAttributes5>,
	#[validate]
	#[serde(rename = "OthrPty")]
	pub othr_pty: Option<Vec<ExtendedParty13>>,
}


// FundPaymentType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FundPaymentType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification36>,
}


// FundPaymentType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FundPaymentType1Code {
	#[validate(enumerate = ["DRAF", "CACC", "CHEQ", "CRDT", "DDEB", "CARD"])]
	#[serde(rename = "FundPaymentType1Code")]
	pub fund_payment_type1_code: String,
}


// FundReferenceDataReport5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FundReferenceDataReport5 {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[validate]
	#[serde(rename = "Vrsn")]
	pub vrsn: Option<MarketPracticeVersion1>,
	#[validate]
	#[serde(rename = "AuthrsdPrxy")]
	pub authrsd_prxy: Option<ContactAttributes6>,
	#[serde(rename = "GnlRefDt")]
	pub gnl_ref_dt: String,
	#[serde(rename = "TrgtMktInd")]
	pub trgt_mkt_ind: Option<bool>,
	#[serde(rename = "ExAnteInd")]
	pub ex_ante_ind: Option<bool>,
	#[serde(rename = "ExPstInd")]
	pub ex_pst_ind: Option<bool>,
	#[validate]
	#[serde(rename = "SctyId")]
	pub scty_id: SecurityIdentification47,
	#[validate]
	#[serde(rename = "FndPties")]
	pub fnd_pties: Option<FundParties1>,
	#[validate]
	#[serde(rename = "MainFndOrdrDsk")]
	pub main_fnd_ordr_dsk: Option<OrderDesk1>,
	#[validate]
	#[serde(rename = "FndMgmtCpny")]
	pub fnd_mgmt_cpny: Option<ContactAttributes5>,
	#[validate]
	#[serde(rename = "FndDtls")]
	pub fnd_dtls: Option<FinancialInstrument96>,
	#[validate]
	#[serde(rename = "ValtnDealgChrtcs")]
	pub valtn_dealg_chrtcs: Option<ValuationDealingProcessingCharacteristics3>,
	#[validate]
	#[serde(rename = "InvstmtRstrctns")]
	pub invstmt_rstrctns: Option<InvestmentRestrictions3>,
	#[validate]
	#[serde(rename = "SbcptPrcgChrtcs")]
	pub sbcpt_prcg_chrtcs: Option<ProcessingCharacteristics11>,
	#[validate]
	#[serde(rename = "RedPrcgChrtcs")]
	pub red_prcg_chrtcs: Option<ProcessingCharacteristics12>,
	#[validate]
	#[serde(rename = "SwtchPrcgChrtcs")]
	pub swtch_prcg_chrtcs: Option<ProcessingCharacteristics9>,
	#[validate]
	#[serde(rename = "PlanChrtcs")]
	pub plan_chrtcs: Option<Vec<InvestmentPlanCharacteristics1>>,
	#[validate]
	#[serde(rename = "PmtInstrm")]
	pub pmt_instrm: Option<Vec<PaymentInstrument16>>,
	#[validate]
	#[serde(rename = "CshSttlmDtls")]
	pub csh_sttlm_dtls: Option<Vec<CashAccount205>>,
	#[validate]
	#[serde(rename = "LclMktAnx")]
	pub lcl_mkt_anx: Option<Vec<LocalMarketAnnex6>>,
	#[validate]
	#[serde(rename = "TrgtMkt")]
	pub trgt_mkt: Option<TargetMarket4>,
	#[validate]
	#[serde(rename = "DstrbtnStrtgy")]
	pub dstrbtn_strtgy: Option<DistributionStrategy1>,
	#[validate]
	#[serde(rename = "CostsAndChrgs")]
	pub costs_and_chrgs: Option<Vec<CostsAndCharges2>>,
	#[validate]
	#[serde(rename = "AddtlInfUKMkt")]
	pub addtl_inf_uk_mkt: Option<AdditionalProductInformation3>,
	#[validate]
	#[serde(rename = "ValForMny")]
	pub val_for_mny: Option<ValueForMoney1>,
	#[validate]
	#[serde(rename = "Xtnsn")]
	pub xtnsn: Option<Vec<Extension1>>,
}


// FundReferenceDataReportV07 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FundReferenceDataReportV07 {
	#[validate]
	#[serde(rename = "MsgId")]
	pub msg_id: MessageIdentification1,
	#[validate]
	#[serde(rename = "PrvsRef")]
	pub prvs_ref: Option<Vec<AdditionalReference10>>,
	#[validate]
	#[serde(rename = "RltdRef")]
	pub rltd_ref: Option<AdditionalReference10>,
	#[serde(rename = "FndRefDataRptId")]
	pub fnd_ref_data_rpt_id: Option<String>,
	#[validate]
	#[serde(rename = "Rpt")]
	pub rpt: Vec<FundReferenceDataReport5>,
}


// GenericAccountIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericAccountIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[validate]
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<AccountSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericIdentification3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification3 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericIdentification36 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification36 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// GenericIdentification47 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification47 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// GovernanceProcess1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GovernanceProcess1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// GovernanceProcessType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GovernanceProcessType1Code {
	#[validate(enumerate = ["BMIF", "NINF", "CMIF", "AMIF"])]
	#[serde(rename = "GovernanceProcessType1Code")]
	pub governance_process_type1_code: String,
}


// HoldingTransferable1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct HoldingTransferable1Code {
	#[validate(enumerate = ["TRAL", "TRNA", "RFOD"])]
	#[serde(rename = "HoldingTransferable1Code")]
	pub holding_transferable1_code: String,
}


// IBAN2007Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct IBAN2007Identifier {
	#[validate(pattern = "[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}")]
	#[serde(rename = "IBAN2007Identifier")]
	pub iban2007_identifier: String,
}


// ISINOct2015Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[validate(pattern = "[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}")]
	#[serde(rename = "ISINOct2015Identifier")]
	pub isin_oct2015_identifier: String,
}


// ISODate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// ISOTime ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISOTime {
	#[serde(rename = "ISOTime")]
	pub iso_time: String,
}


// ISOYearMonth ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISOYearMonth {
	#[serde(rename = "ISOYearMonth")]
	pub iso_year_month: String,
}


// IdentificationSource3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct IdentificationSource3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// IndividualCostOrCharge2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct IndividualCostOrCharge2 {
	#[validate]
	#[serde(rename = "CostTp")]
	pub cost_tp: ChargeType8Choice,
	#[serde(rename = "ExAnteOrExPst")]
	pub ex_ante_or_ex_pst: String,
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
	#[serde(rename = "Sgn")]
	pub sgn: Option<bool>,
	#[serde(rename = "Rate")]
	pub rate: Option<f64>,
	#[validate]
	#[serde(rename = "RefPrd")]
	pub ref_prd: Option<Period15>,
	#[validate]
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<AdditionalInformation15>,
}


// IntendedOrActual2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct IntendedOrActual2Code {
	#[validate(enumerate = ["ANTE", "POST"])]
	#[serde(rename = "IntendedOrActual2Code")]
	pub intended_or_actual2_code: String,
}


// InvestmentFundMiFIDFee2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestmentFundMiFIDFee2Code {
	#[validate(enumerate = ["BORF", "DIS2", "FES3", "FEND", "FES2", "GOC1", "GOCS", "INCF", "INCS", "MNF1", "MANS", "NET2", "NESF", "NETO", "NRAM", "OOEA", "OOSF", "OOSS", "BENS", "ENAC", "ENFX", "EXAC", "ENBX", "BEND", "PENO", "OTES", "OCAS", "RPSS", "TRS1"])]
	#[serde(rename = "InvestmentFundMiFIDFee2Code")]
	pub investment_fund_mi_fid_fee2_code: String,
}


// InvestmentFundPlanType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestmentFundPlanType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification36>,
}


// InvestmentFundPlanType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestmentFundPlanType1Code {
	#[validate(enumerate = ["INVP", "SWIP", "WTHP"])]
	#[serde(rename = "InvestmentFundPlanType1Code")]
	pub investment_fund_plan_type1_code: String,
}


// InvestmentNeed2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestmentNeed2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// InvestmentNeed2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestmentNeed2Code {
	#[validate(enumerate = ["NSPE", "OTHR", "ISLB"])]
	#[serde(rename = "InvestmentNeed2Code")]
	pub investment_need2_code: String,
}


// InvestmentPlanCharacteristics1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestmentPlanCharacteristics1 {
	#[validate]
	#[serde(rename = "PlanTp")]
	pub plan_tp: InvestmentFundPlanType1Choice,
	#[validate]
	#[serde(rename = "Frqcy")]
	pub frqcy: Option<Frequency20Choice>,
	#[serde(rename = "TtlNbOfInstlmts")]
	pub ttl_nb_of_instlmts: Option<f64>,
	#[validate]
	#[serde(rename = "Qty")]
	pub qty: Option<UnitsOrAmount1Choice>,
	#[serde(rename = "PlanConttn")]
	pub plan_conttn: Option<bool>,
	#[serde(rename = "AddtlSbcpt")]
	pub addtl_sbcpt: Option<bool>,
	#[serde(rename = "AddtlSbcptFctn")]
	pub addtl_sbcpt_fctn: Option<bool>,
	#[validate]
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}


// InvestmentRestrictions3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestmentRestrictions3 {
	#[validate]
	#[serde(rename = "MinInitlSbcptAmt")]
	pub min_initl_sbcpt_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "MinInitlSbcptUnits")]
	pub min_initl_sbcpt_units: Option<f64>,
	#[validate]
	#[serde(rename = "MinSbsqntSbcptAmt")]
	pub min_sbsqnt_sbcpt_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "MinSbsqntSbcptUnits")]
	pub min_sbsqnt_sbcpt_units: Option<f64>,
	#[validate]
	#[serde(rename = "MaxRedAmt")]
	pub max_red_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "MaxRedUnits")]
	pub max_red_units: Option<f64>,
	#[serde(rename = "MinRedPctg")]
	pub min_red_pctg: Option<f64>,
	#[serde(rename = "OthrRedRstrctns")]
	pub othr_red_rstrctns: Option<String>,
	#[validate]
	#[serde(rename = "MinSwtchSbcptAmt")]
	pub min_swtch_sbcpt_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "MinSwtchSbcptUnits")]
	pub min_swtch_sbcpt_units: Option<f64>,
	#[validate]
	#[serde(rename = "MaxSwtchRedAmt")]
	pub max_swtch_red_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "MaxSwtchRedUnits")]
	pub max_swtch_red_units: Option<f64>,
	#[serde(rename = "OthrSwtchRstrctns")]
	pub othr_swtch_rstrctns: Option<String>,
	#[validate]
	#[serde(rename = "MinHldgAmt")]
	pub min_hldg_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "MinHldgUnits")]
	pub min_hldg_units: Option<f64>,
	#[serde(rename = "MinHldgPrd")]
	pub min_hldg_prd: Option<String>,
	#[serde(rename = "HldgTrfbl")]
	pub hldg_trfbl: Option<String>,
	#[validate]
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}


// InvestorKnowledge1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestorKnowledge1 {
	#[serde(rename = "BsicInvstr")]
	pub bsic_invstr: Option<String>,
	#[serde(rename = "InfrmdInvstr")]
	pub infrmd_invstr: Option<String>,
	#[serde(rename = "AdvncdInvstr")]
	pub advncd_invstr: Option<String>,
	#[serde(rename = "ExprtInvstrDE")]
	pub exprt_invstr_de: Option<String>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<OtherTargetMarketInvestorKnowledge1>>,
}


// InvestorRequirements4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestorRequirements4 {
	#[serde(rename = "RtrPrflPrsrvtn")]
	pub rtr_prfl_prsrvtn: Option<String>,
	#[serde(rename = "RtrPrflGrwth")]
	pub rtr_prfl_grwth: Option<String>,
	#[serde(rename = "RtrPrflIncm")]
	pub rtr_prfl_incm: Option<String>,
	#[serde(rename = "RtrPrflHdgg")]
	pub rtr_prfl_hdgg: Option<String>,
	#[serde(rename = "OptnOrLvrgdRtrPrfl")]
	pub optn_or_lvrgd_rtr_prfl: Option<String>,
	#[serde(rename = "RtrPrflPnsnSchmeDE")]
	pub rtr_prfl_pnsn_schme_de: Option<String>,
	#[validate]
	#[serde(rename = "MinHldgPrd")]
	pub min_hldg_prd: Option<TimeHorizon2Choice>,
	#[serde(rename = "SstnbltyPrefs")]
	pub sstnblty_prefs: Option<String>,
	#[validate]
	#[serde(rename = "OthrSpcfcInvstmtNeed")]
	pub othr_spcfc_invstmt_need: Option<InvestmentNeed2Choice>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<OtherInvestmentNeed1>>,
}


// InvestorType2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestorType2 {
	#[serde(rename = "InvstrTpRtl")]
	pub invstr_tp_rtl: Option<String>,
	#[validate]
	#[serde(rename = "InvstrTpPrfssnl")]
	pub invstr_tp_prfssnl: Option<TargetMarket5Choice>,
	#[serde(rename = "InvstrTpElgblCtrPty")]
	pub invstr_tp_elgbl_ctr_pty: Option<String>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<OtherTargetMarketInvestor1>>,
}


// InvestorType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestorType2Code {
	#[validate(enumerate = ["BOT3", "EPRO", "PRF2"])]
	#[serde(rename = "InvestorType2Code")]
	pub investor_type2_code: String,
}


// InvestorType3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestorType3Code {
	#[validate(enumerate = ["RETL", "PRF2", "NEI1", "BOT2"])]
	#[serde(rename = "InvestorType3Code")]
	pub investor_type3_code: String,
}


// InvestorType4Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestorType4Code {
	#[validate(enumerate = ["BOT3", "NPRF", "PRF3", "PRF4"])]
	#[serde(rename = "InvestorType4Code")]
	pub investor_type4_code: String,
}


// LEIIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[validate(pattern = "[A-Z0-9]{18,18}[0-9]{2,2}")]
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LocalMarketAnnex6 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LocalMarketAnnex6 {
	#[serde(rename = "Ctry")]
	pub ctry: Vec<String>,
	#[validate]
	#[serde(rename = "LclOrdrDsk")]
	pub lcl_ordr_dsk: OrderDesk1,
	#[validate]
	#[serde(rename = "SbcptPrcgChrtcs")]
	pub sbcpt_prcg_chrtcs: Option<ProcessingCharacteristics11>,
	#[validate]
	#[serde(rename = "RedPrcgChrtcs")]
	pub red_prcg_chrtcs: Option<ProcessingCharacteristics10>,
	#[validate]
	#[serde(rename = "SwtchPrcgChrtcs")]
	pub swtch_prcg_chrtcs: Option<ProcessingCharacteristics9>,
	#[validate]
	#[serde(rename = "CshSttlmDtls")]
	pub csh_sttlm_dtls: Option<Vec<CashAccount205>>,
	#[validate]
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}


// LossBearing2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LossBearing2 {
	#[serde(rename = "NoCptlLoss")]
	pub no_cptl_loss: Option<String>,
	#[serde(rename = "LtdCptlLoss")]
	pub ltd_cptl_loss: Option<String>,
	#[serde(rename = "LtdCptlLossLvl")]
	pub ltd_cptl_loss_lvl: Option<f64>,
	#[serde(rename = "NoCptlGrnt")]
	pub no_cptl_grnt: Option<String>,
	#[serde(rename = "LossByndCptl")]
	pub loss_bynd_cptl: Option<String>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<OtherTargetMarketLossBearing1>>,
}


// MainFundOrderDeskLocation1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MainFundOrderDeskLocation1 {
	#[serde(rename = "Ctry")]
	pub ctry: String,
	#[validate]
	#[serde(rename = "TmZoneOffSet")]
	pub tm_zone_off_set: UTCOffset1,
}


// MarketPracticeVersion1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MarketPracticeVersion1 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "Nb")]
	pub nb: Option<String>,
}


// Max140Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max140Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 140)]
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max16Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max16Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 16)]
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
}


// Max1Number ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max1Number {
	#[serde(rename = "Max1Number")]
	pub max1_number: f64,
}


// Max2048Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max2048Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 2048)]
	#[serde(rename = "Max2048Text")]
	pub max2048_text: String,
}


// Max256Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max256Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 256)]
	#[serde(rename = "Max256Text")]
	pub max256_text: String,
}


// Max34Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max34Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 34)]
	#[serde(rename = "Max34Text")]
	pub max34_text: String,
}


// Max350Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max350Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 350)]
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max35Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 35)]
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Max4AlphaNumericText ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max4AlphaNumericText {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[validate(pattern = "[a-zA-Z0-9]{1,4}")]
	#[serde(rename = "Max4AlphaNumericText")]
	pub max4_alpha_numeric_text: String,
}


// Max70Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max70Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 70)]
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// MessageIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MessageIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
}


// NameAndAddress5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[validate]
	#[serde(rename = "Adr")]
	pub adr: Option<PostalAddress1>,
}


// NotionalOrUnitBased1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NotionalOrUnitBased1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// NotionalOrUnitBased1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NotionalOrUnitBased1Code {
	#[validate(enumerate = ["UNIT", "NOTI"])]
	#[serde(rename = "NotionalOrUnitBased1Code")]
	pub notional_or_unit_based1_code: String,
}


// Number ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
}


// OrderDesk1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OrderDesk1 {
	#[validate]
	#[serde(rename = "OrdrDsk")]
	pub ordr_dsk: Option<ContactAttributes5>,
	#[serde(rename = "ClsrDts")]
	pub clsr_dts: Option<Vec<String>>,
	#[validate]
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}


// OtherDistributionStrategy1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OtherDistributionStrategy1 {
	#[serde(rename = "DstrbtnStrtgyTp")]
	pub dstrbtn_strtgy_tp: Option<String>,
	#[validate]
	#[serde(rename = "Trgt")]
	pub trgt: Option<DistributionStrategy1Choice>,
	#[validate]
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<AdditionalInformation15>,
}


// OtherIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OtherIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Sfx")]
	pub sfx: Option<String>,
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: IdentificationSource3Choice,
}


// OtherInvestmentNeed1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OtherInvestmentNeed1 {
	#[serde(rename = "ClntObjctvsAndNeedsTp")]
	pub clnt_objctvs_and_needs_tp: Option<String>,
	#[validate]
	#[serde(rename = "Trgt")]
	pub trgt: Option<TargetMarket1Choice>,
	#[validate]
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<AdditionalInformation15>,
}


// OtherReviewRelatedToValueAndOrChargesUKType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OtherReviewRelatedToValueAndOrChargesUKType1Code {
	#[validate(enumerate = ["REVA", "REVO"])]
	#[serde(rename = "OtherReviewRelatedToValueAndOrChargesUKType1Code")]
	pub other_review_related_to_value_and_or_charges_uk_type1_code: String,
}


// OtherTargetMarket1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OtherTargetMarket1 {
	#[serde(rename = "TrgtMktTp")]
	pub trgt_mkt_tp: String,
	#[validate]
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<AdditionalInformation15>,
}


// OtherTargetMarketInvestor1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OtherTargetMarketInvestor1 {
	#[serde(rename = "InvstrTp")]
	pub invstr_tp: Option<String>,
	#[validate]
	#[serde(rename = "Trgt")]
	pub trgt: Option<TargetMarket3Choice>,
	#[validate]
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<AdditionalInformation15>,
}


// OtherTargetMarketInvestorKnowledge1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OtherTargetMarketInvestorKnowledge1 {
	#[serde(rename = "InvstrKnwldgTp")]
	pub invstr_knwldg_tp: Option<String>,
	#[validate]
	#[serde(rename = "Trgt")]
	pub trgt: Option<TargetMarket1Choice>,
	#[validate]
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<AdditionalInformation15>,
}


// OtherTargetMarketLossBearing1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OtherTargetMarketLossBearing1 {
	#[serde(rename = "AbltyToBearLossesTp")]
	pub ablty_to_bear_losses_tp: Option<String>,
	#[validate]
	#[serde(rename = "Trgt")]
	pub trgt: Option<TargetMarket1Choice>,
	#[validate]
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<AdditionalInformation15>,
}


// OtherTargetMarketRiskTolerance1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OtherTargetMarketRiskTolerance1 {
	#[serde(rename = "RskTlrnceTp")]
	pub rsk_tlrnce_tp: Option<String>,
	#[validate]
	#[serde(rename = "Trgt")]
	pub trgt: Option<TargetMarket1Choice>,
	#[validate]
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<AdditionalInformation15>,
}


// OutcomeOfCOLLAssessmentOfValueUKType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OutcomeOfCOLLAssessmentOfValueUKType1Code {
	#[validate(enumerate = ["COL1", "COL2"])]
	#[serde(rename = "OutcomeOfCOLLAssessmentOfValueUKType1Code")]
	pub outcome_of_coll_assessment_of_value_uk_type1_code: String,
}


// OutcomeOfPRINValueAssessmentOrReviewUKType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OutcomeOfPRINValueAssessmentOrReviewUKType1Code {
	#[validate(enumerate = ["PRI2", "PRI1"])]
	#[serde(rename = "OutcomeOfPRINValueAssessmentOrReviewUKType1Code")]
	pub outcome_of_prin_value_assessment_or_review_uk_type1_code: String,
}


// PartyIdentification125Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification125Choice {
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
	#[validate]
	#[serde(rename = "PrtryId")]
	pub prtry_id: Option<GenericIdentification1>,
	#[validate]
	#[serde(rename = "NmAndAdr")]
	pub nm_and_adr: Option<NameAndAddress5>,
}


// PartyIdentification139 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification139 {
	#[validate]
	#[serde(rename = "Pty")]
	pub pty: PartyIdentification125Choice,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
}


// PaymentInstrument16 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PaymentInstrument16 {
	#[validate]
	#[serde(rename = "OrdrTp")]
	pub ordr_tp: FundOrderType5Choice,
	#[validate]
	#[serde(rename = "InstrmTp")]
	pub instrm_tp: FundPaymentType1Choice,
	#[validate]
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}


// PercentageRate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// Period15 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Period15 {
	#[serde(rename = "StartDt")]
	pub start_dt: String,
	#[serde(rename = "EndDt")]
	pub end_dt: String,
}


// PhoneNumber ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PhoneNumber {
	#[validate(pattern = "\\+[0-9]{1,3}-[0-9()+\\-]{1,30}")]
	#[serde(rename = "PhoneNumber")]
	pub phone_number: String,
}


// PlusOrMinusIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "PlusOrMinusIndicator")]
	pub plus_or_minus_indicator: bool,
}


// PostalAddress1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PostalAddress1 {
	#[serde(rename = "AdrTp")]
	pub adr_tp: Option<String>,
	#[serde(rename = "AdrLine")]
	pub adr_line: Option<Vec<String>>,
	#[serde(rename = "StrtNm")]
	pub strt_nm: Option<String>,
	#[serde(rename = "BldgNb")]
	pub bldg_nb: Option<String>,
	#[serde(rename = "PstCd")]
	pub pst_cd: Option<String>,
	#[serde(rename = "TwnNm")]
	pub twn_nm: Option<String>,
	#[serde(rename = "CtrySubDvsn")]
	pub ctry_sub_dvsn: Option<String>,
	#[serde(rename = "Ctry")]
	pub ctry: String,
}


// PriceMethod1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PriceMethod1Code {
	#[validate(enumerate = ["FORW", "HIST"])]
	#[serde(rename = "PriceMethod1Code")]
	pub price_method1_code: String,
}


// ProcessingCharacteristics10 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ProcessingCharacteristics10 {
	#[serde(rename = "DealgCcyAccptd")]
	pub dealg_ccy_accptd: Option<Vec<String>>,
	#[validate]
	#[serde(rename = "RedAuthstn")]
	pub red_authstn: Option<Forms1>,
	#[serde(rename = "AmtInd")]
	pub amt_ind: Option<bool>,
	#[serde(rename = "UnitsInd")]
	pub units_ind: Option<bool>,
	#[serde(rename = "Rndg")]
	pub rndg: Option<String>,
	#[serde(rename = "PctgInd")]
	pub pctg_ind: Option<bool>,
	#[validate]
	#[serde(rename = "MainFndOrdrDskLctn")]
	pub main_fnd_ordr_dsk_lctn: Option<MainFundOrderDeskLocation1>,
	#[serde(rename = "DealgFrqcy")]
	pub dealg_frqcy: Option<String>,
	#[serde(rename = "DealgFrqcyDesc")]
	pub dealg_frqcy_desc: Option<String>,
	#[serde(rename = "DealgCutOffTm")]
	pub dealg_cut_off_tm: Option<String>,
	#[validate]
	#[serde(rename = "DealgCutOffTmFrame")]
	pub dealg_cut_off_tm_frame: Option<TimeFrame9>,
	#[serde(rename = "DealConfTm")]
	pub deal_conf_tm: Option<String>,
	#[validate]
	#[serde(rename = "DealConfTmFrame")]
	pub deal_conf_tm_frame: Option<TimeFrame8>,
	#[serde(rename = "LtdPrd")]
	pub ltd_prd: Option<String>,
	#[validate]
	#[serde(rename = "SttlmCycl")]
	pub sttlm_cycl: Option<TimeFrame8Choice>,
	#[validate]
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}


// ProcessingCharacteristics11 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ProcessingCharacteristics11 {
	#[serde(rename = "DealgCcyAccptd")]
	pub dealg_ccy_accptd: Option<Vec<String>>,
	#[validate]
	#[serde(rename = "InitlInvstmtAppl")]
	pub initl_invstmt_appl: Option<Forms1>,
	#[validate]
	#[serde(rename = "SbsqntInvstmtAppl")]
	pub sbsqnt_invstmt_appl: Option<Forms1>,
	#[serde(rename = "AmtInd")]
	pub amt_ind: Option<bool>,
	#[serde(rename = "UnitsInd")]
	pub units_ind: Option<bool>,
	#[serde(rename = "Rndg")]
	pub rndg: Option<String>,
	#[validate]
	#[serde(rename = "MainFndOrdrDskLctn")]
	pub main_fnd_ordr_dsk_lctn: Option<MainFundOrderDeskLocation1>,
	#[serde(rename = "DealgFrqcy")]
	pub dealg_frqcy: Option<String>,
	#[serde(rename = "DealgFrqcyDesc")]
	pub dealg_frqcy_desc: Option<String>,
	#[serde(rename = "DealgCutOffTm")]
	pub dealg_cut_off_tm: Option<String>,
	#[validate]
	#[serde(rename = "DealgCutOffTmFrame")]
	pub dealg_cut_off_tm_frame: Option<TimeFrame9>,
	#[serde(rename = "DealConfTm")]
	pub deal_conf_tm: Option<String>,
	#[validate]
	#[serde(rename = "DealConfTmFrame")]
	pub deal_conf_tm_frame: Option<TimeFrame11>,
	#[serde(rename = "LtdPrd")]
	pub ltd_prd: Option<String>,
	#[validate]
	#[serde(rename = "SttlmCycl")]
	pub sttlm_cycl: Option<TimeFrame7Choice>,
	#[validate]
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}


// ProcessingCharacteristics12 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ProcessingCharacteristics12 {
	#[serde(rename = "DealgCcyAccptd")]
	pub dealg_ccy_accptd: Option<Vec<String>>,
	#[validate]
	#[serde(rename = "RedAuthstn")]
	pub red_authstn: Option<Forms1>,
	#[serde(rename = "AmtInd")]
	pub amt_ind: Option<bool>,
	#[serde(rename = "UnitsInd")]
	pub units_ind: Option<bool>,
	#[serde(rename = "Rndg")]
	pub rndg: Option<String>,
	#[serde(rename = "PctgInd")]
	pub pctg_ind: Option<bool>,
	#[validate]
	#[serde(rename = "MainFndOrdrDskLctn")]
	pub main_fnd_ordr_dsk_lctn: Option<MainFundOrderDeskLocation1>,
	#[serde(rename = "DealgFrqcy")]
	pub dealg_frqcy: Option<String>,
	#[serde(rename = "DealgFrqcyDesc")]
	pub dealg_frqcy_desc: Option<String>,
	#[serde(rename = "DealgCutOffTm")]
	pub dealg_cut_off_tm: Option<String>,
	#[validate]
	#[serde(rename = "DealgCutOffTmFrame")]
	pub dealg_cut_off_tm_frame: Option<TimeFrame9>,
	#[serde(rename = "DealConfTm")]
	pub deal_conf_tm: Option<String>,
	#[validate]
	#[serde(rename = "DealConfTmFrame")]
	pub deal_conf_tm_frame: Option<TimeFrame10>,
	#[serde(rename = "LtdPrd")]
	pub ltd_prd: Option<String>,
	#[validate]
	#[serde(rename = "SttlmCycl")]
	pub sttlm_cycl: Option<TimeFrame8Choice>,
	#[validate]
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}


// ProcessingCharacteristics9 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ProcessingCharacteristics9 {
	#[serde(rename = "DealgCcyAccptd")]
	pub dealg_ccy_accptd: Option<Vec<String>>,
	#[validate]
	#[serde(rename = "SwtchAuthstn")]
	pub swtch_authstn: Option<Forms1>,
	#[serde(rename = "AmtInd")]
	pub amt_ind: Option<bool>,
	#[serde(rename = "UnitsInd")]
	pub units_ind: Option<bool>,
	#[serde(rename = "Rndg")]
	pub rndg: Option<String>,
	#[validate]
	#[serde(rename = "MainFndOrdrDskLctn")]
	pub main_fnd_ordr_dsk_lctn: Option<MainFundOrderDeskLocation1>,
	#[serde(rename = "DealgFrqcy")]
	pub dealg_frqcy: Option<String>,
	#[serde(rename = "DealgFrqcyDesc")]
	pub dealg_frqcy_desc: Option<String>,
	#[serde(rename = "DealgCutOffTm")]
	pub dealg_cut_off_tm: Option<String>,
	#[validate]
	#[serde(rename = "DealgCutOffTmFrame")]
	pub dealg_cut_off_tm_frame: Option<TimeFrame9>,
	#[serde(rename = "DealConfTm")]
	pub deal_conf_tm: Option<String>,
	#[validate]
	#[serde(rename = "DealConfTmFrame")]
	pub deal_conf_tm_frame: Option<TimeFrame8>,
	#[serde(rename = "LtdPrd")]
	pub ltd_prd: Option<String>,
	#[validate]
	#[serde(rename = "SttlmCycl")]
	pub sttlm_cycl: Option<TimeFrame8Choice>,
	#[validate]
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}


// ProductStructure1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ProductStructure1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// ProductStructure1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ProductStructure1Code {
	#[validate(enumerate = ["BOND", "NUMM", "UCMM", "EXTC", "UCIT", "SSEC", "SFUN", "NUCI"])]
	#[serde(rename = "ProductStructure1Code")]
	pub product_structure1_code: String,
}


// QuotationType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct QuotationType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// QuotationType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct QuotationType1Code {
	#[validate(enumerate = ["ACTU", "PRCT"])]
	#[serde(rename = "QuotationType1Code")]
	pub quotation_type1_code: String,
}


// ReferToFundOrderDesk1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReferToFundOrderDesk1Code {
	#[validate(enumerate = ["RFOD"])]
	#[serde(rename = "ReferToFundOrderDesk1Code")]
	pub refer_to_fund_order_desk1_code: String,
}


// RiskLevel1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RiskLevel1Code {
	#[validate(enumerate = ["HIGH", "LOWW", "MEDM"])]
	#[serde(rename = "RiskLevel1Code")]
	pub risk_level1_code: String,
}


// RiskTolerance1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RiskTolerance1 {
	#[serde(rename = "RskTlrncePRIIPSMthdlgy")]
	pub rsk_tlrnce_priips_mthdlgy: Option<f64>,
	#[serde(rename = "RskTlrnceUCITSMthdlgy")]
	pub rsk_tlrnce_ucits_mthdlgy: Option<f64>,
	#[serde(rename = "RskTlrnceIntl")]
	pub rsk_tlrnce_intl: Option<String>,
	#[serde(rename = "RskTlrnceForNonPRIIPSAndNonUCITSES")]
	pub rsk_tlrnce_for_non_priips_and_non_ucitses: Option<f64>,
	#[serde(rename = "NotForInvstrsWthTheLwstRskTlrnceDE")]
	pub not_for_invstrs_wth_the_lwst_rsk_tlrnce_de: Option<String>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<OtherTargetMarketRiskTolerance1>>,
}


// RoundingDirection2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RoundingDirection2Code {
	#[validate(enumerate = ["RDUP", "RDWN"])]
	#[serde(rename = "RoundingDirection2Code")]
	pub rounding_direction2_code: String,
}


// SecurityClassificationType2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityClassificationType2Choice {
	#[serde(rename = "CFI")]
	pub cfi: Option<String>,
	#[validate]
	#[serde(rename = "AltrnClssfctn")]
	pub altrn_clssfctn: Option<GenericIdentification3>,
}


// SecurityIdentification40 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityIdentification40 {
	#[validate]
	#[serde(rename = "OthrId")]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
}


// SecurityIdentification47 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityIdentification47 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: SecurityIdentification40,
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "ShrtNm")]
	pub shrt_nm: Option<String>,
	#[serde(rename = "ClssTp")]
	pub clss_tp: Option<String>,
	#[serde(rename = "UmbrllNm")]
	pub umbrll_nm: Option<String>,
	#[serde(rename = "NewUmbrll")]
	pub new_umbrll: Option<bool>,
	#[validate]
	#[serde(rename = "ClssfctnTp")]
	pub clssfctn_tp: Option<SecurityClassificationType2Choice>,
	#[serde(rename = "BaseCcy")]
	pub base_ccy: Option<String>,
	#[serde(rename = "CtryOfDmcl")]
	pub ctry_of_dmcl: Option<String>,
	#[serde(rename = "RegdDstrbtnCtry")]
	pub regd_dstrbtn_ctry: Option<Vec<String>>,
	#[validate]
	#[serde(rename = "PdctTp")]
	pub pdct_tp: Option<ProductStructure1Choice>,
	#[validate]
	#[serde(rename = "Issr")]
	pub issr: Option<ContactAttributes5>,
	#[validate]
	#[serde(rename = "IssrPdctGovncPrc")]
	pub issr_pdct_govnc_prc: Option<GovernanceProcess1Choice>,
	#[serde(rename = "PdctCtgy")]
	pub pdct_ctgy: Option<String>,
	#[serde(rename = "PdctCtgyDE")]
	pub pdct_ctgy_de: Option<String>,
	#[validate]
	#[serde(rename = "NtnlOrUnitBased")]
	pub ntnl_or_unit_based: Option<NotionalOrUnitBased1Choice>,
	#[validate]
	#[serde(rename = "QtnTp")]
	pub qtn_tp: Option<QuotationType1Choice>,
	#[serde(rename = "LvrgdOrCntngntLblty")]
	pub lvrgd_or_cntngnt_lblty: Option<bool>,
	#[serde(rename = "NoRtrcssnInd")]
	pub no_rtrcssn_ind: Option<bool>,
	#[validate]
	#[serde(rename = "ExPstCostClctnBsis")]
	pub ex_pst_cost_clctn_bsis: Option<ExPostCostCalculationBasis1Choice>,
	#[validate]
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}


// SignatureType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SignatureType1Code {
	#[validate(enumerate = ["ORIG", "DIGI", "ELEC", "NONE"])]
	#[serde(rename = "SignatureType1Code")]
	pub signature_type1_code: String,
}


// SustainabilityPreferences2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SustainabilityPreferences2Code {
	#[validate(enumerate = ["NEUT", "YSCO"])]
	#[serde(rename = "SustainabilityPreferences2Code")]
	pub sustainability_preferences2_code: String,
}


// TargetMarket1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TargetMarket1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// TargetMarket1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TargetMarket1Code {
	#[validate(enumerate = ["YSCO", "NEUT", "NSCO"])]
	#[serde(rename = "TargetMarket1Code")]
	pub target_market1_code: String,
}


// TargetMarket2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TargetMarket2Code {
	#[validate(enumerate = ["NEUT", "YSCO"])]
	#[serde(rename = "TargetMarket2Code")]
	pub target_market2_code: String,
}


// TargetMarket3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TargetMarket3Choice {
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// TargetMarket3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TargetMarket3Code {
	#[validate(enumerate = ["YSCO", "NSCO"])]
	#[serde(rename = "TargetMarket3Code")]
	pub target_market3_code: String,
}


// TargetMarket4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TargetMarket4 {
	#[serde(rename = "RefDt")]
	pub ref_dt: Option<String>,
	#[validate]
	#[serde(rename = "InvstrTp")]
	pub invstr_tp: Option<InvestorType2>,
	#[validate]
	#[serde(rename = "KnwldgAndOrExprnc")]
	pub knwldg_and_or_exprnc: Option<InvestorKnowledge1>,
	#[validate]
	#[serde(rename = "AbltyToBearLosses")]
	pub ablty_to_bear_losses: Option<LossBearing2>,
	#[validate]
	#[serde(rename = "RskTlrnce")]
	pub rsk_tlrnce: Option<RiskTolerance1>,
	#[validate]
	#[serde(rename = "ClntObjctvsAndNeeds")]
	pub clnt_objctvs_and_needs: Option<InvestorRequirements4>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<OtherTargetMarket1>>,
}


// TargetMarket5Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TargetMarket5Choice {
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<String>,
}


// TimeFrame10 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TimeFrame10 {
	#[serde(rename = "OthrTmFrameDesc")]
	pub othr_tm_frame_desc: Option<String>,
	#[serde(rename = "TPlus")]
	pub t_plus: Option<f64>,
	#[serde(rename = "NonWorkgDayAdjstmnt")]
	pub non_workg_day_adjstmnt: Option<String>,
	#[serde(rename = "RefrToOrdrDsk")]
	pub refr_to_ordr_dsk: Option<String>,
}


// TimeFrame11 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TimeFrame11 {
	#[serde(rename = "OthrTmFrameDesc")]
	pub othr_tm_frame_desc: Option<String>,
	#[serde(rename = "TPlus")]
	pub t_plus: Option<f64>,
	#[serde(rename = "NonWorkgDayAdjstmnt")]
	pub non_workg_day_adjstmnt: Option<String>,
	#[serde(rename = "RefrToOrdrDsk")]
	pub refr_to_ordr_dsk: Option<String>,
}


// TimeFrame2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TimeFrame2Code {
	#[validate(enumerate = ["HOLD", "LONG", "MEDM", "SHOR", "VSHT"])]
	#[serde(rename = "TimeFrame2Code")]
	pub time_frame2_code: String,
}


// TimeFrame7Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TimeFrame7Choice {
	#[serde(rename = "TPlus")]
	pub t_plus: Option<f64>,
	#[serde(rename = "Prepmt")]
	pub prepmt: Option<bool>,
}


// TimeFrame8 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TimeFrame8 {
	#[serde(rename = "OthrTmFrameDesc")]
	pub othr_tm_frame_desc: Option<String>,
	#[serde(rename = "TPlus")]
	pub t_plus: Option<f64>,
	#[serde(rename = "NonWorkgDayAdjstmnt")]
	pub non_workg_day_adjstmnt: Option<String>,
	#[serde(rename = "RefrToOrdrDsk")]
	pub refr_to_ordr_dsk: Option<String>,
}


// TimeFrame8Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TimeFrame8Choice {
	#[serde(rename = "TPlus")]
	pub t_plus: Option<f64>,
	#[serde(rename = "RPlus")]
	pub r_plus: Option<f64>,
}


// TimeFrame9 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TimeFrame9 {
	#[serde(rename = "OthrTmFrameDesc")]
	pub othr_tm_frame_desc: Option<String>,
	#[serde(rename = "TMns")]
	pub t_mns: Option<f64>,
	#[serde(rename = "NonWorkgDayAdjstmnt")]
	pub non_workg_day_adjstmnt: Option<String>,
	#[serde(rename = "RefrToOrdrDsk")]
	pub refr_to_ordr_dsk: Option<String>,
}


// TimeFrame9Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TimeFrame9Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// TimeHorizon2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TimeHorizon2Choice {
	#[serde(rename = "NbOfYrs")]
	pub nb_of_yrs: Option<f64>,
	#[validate]
	#[serde(rename = "TmFrame")]
	pub tm_frame: Option<TimeFrame9Choice>,
}


// UTCOffset1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct UTCOffset1 {
	#[serde(rename = "Sgn")]
	pub sgn: bool,
	#[serde(rename = "NbOfHrs")]
	pub nb_of_hrs: String,
}


// UnitsOrAmount1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct UnitsOrAmount1Choice {
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Unit")]
	pub unit: Option<f64>,
}


// ValuationDealingProcessingCharacteristics3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ValuationDealingProcessingCharacteristics3 {
	#[serde(rename = "ValtnFrqcy")]
	pub valtn_frqcy: Option<String>,
	#[serde(rename = "ValtnFrqcyDesc")]
	pub valtn_frqcy_desc: Option<String>,
	#[serde(rename = "ValtnTm")]
	pub valtn_tm: Option<String>,
	#[serde(rename = "DcmlstnUnits")]
	pub dcmlstn_units: Option<f64>,
	#[serde(rename = "DcmlstnPric")]
	pub dcmlstn_pric: Option<f64>,
	#[serde(rename = "DualFndInd")]
	pub dual_fnd_ind: Option<bool>,
	#[serde(rename = "PricMtd")]
	pub pric_mtd: Option<String>,
	#[serde(rename = "PricCcy")]
	pub pric_ccy: Option<Vec<String>>,
	#[validate]
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}


// ValueForMoney1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ValueForMoney1 {
	#[serde(rename = "EMTDataRptgVFMUK")]
	pub emt_data_rptg_vfmuk: Option<String>,
	#[serde(rename = "AssmntOfValReqrdUdrCOLLUK")]
	pub assmnt_of_val_reqrd_udr_colluk: Option<String>,
	#[serde(rename = "OutcmOfCOLLAssmntOfValUK")]
	pub outcm_of_coll_assmnt_of_val_uk: Option<String>,
	#[serde(rename = "OutcmOfPRINValAssmntOrRvwUK")]
	pub outcm_of_prin_val_assmnt_or_rvw_uk: Option<String>,
	#[serde(rename = "OthrRvwRltdToValAndOrChrgsUK")]
	pub othr_rvw_rltd_to_val_and_or_chrgs_uk: Option<String>,
	#[serde(rename = "FrthrInfUK")]
	pub frthr_inf_uk: Option<String>,
	#[serde(rename = "RvwDtUK")]
	pub rvw_dt_uk: Option<String>,
	#[serde(rename = "RvwNxtDueUK")]
	pub rvw_nxt_due_uk: Option<String>,
}


// YesNoIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "YesNoIndicator")]
	pub yes_no_indicator: bool,
}
