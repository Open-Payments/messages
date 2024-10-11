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


// AccountIdentificationAndName7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountIdentificationAndName7 {
	#[serde(rename = "Id")]
	pub id: CashAccountIdentification8Choice,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
}


// AccountSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// ActiveCurrencyAnd13DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd13DecimalAmountSimpleType {
	#[serde(rename = "ActiveCurrencyAnd13DecimalAmount_SimpleType")]
	pub active_currency_and13_decimal_amount_simple_type: f64,
}


// ActiveCurrencyAnd13DecimalAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd13DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "ActiveCurrencyAndAmount_SimpleType")]
	pub active_currency_and_amount_simple_type: f64,
}


// ActiveCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// AdditionalInformation15 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdditionalInformation15 {
	#[serde(rename = "InfTp")]
	pub inf_tp: GenericIdentification36,
	#[serde(rename = "InfVal")]
	pub inf_val: String,
}


// AdditionalProductInformation3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdditionalProductInformation3 {
	#[serde(rename = "FinInstrmTxCostsExAnteUK", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_tx_costs_ex_ante_uk: Option<f64>,
	#[serde(rename = "FinInstrmTxCostsExPstUK", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_tx_costs_ex_pst_uk: Option<f64>,
}


// AdditionalReference10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdditionalReference10 {
	#[serde(rename = "Ref")]
	pub ref_attr: String,
	#[serde(rename = "RefIssr", skip_serializing_if = "Option::is_none")]
	pub ref_issr: Option<PartyIdentification139>,
	#[serde(rename = "MsgNm", skip_serializing_if = "Option::is_none")]
	pub msg_nm: Option<String>,
}


// AddressType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType2Code {
	#[serde(rename = "AddressType2Code")]
	pub address_type2_code: String,
}


// AnnualChargePaymentType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnnualChargePaymentType1Code {
	#[serde(rename = "AnnualChargePaymentType1Code")]
	pub annual_charge_payment_type1_code: String,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// AssessmentOfValueRequiredUnderCOLLUKType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssessmentOfValueRequiredUnderCOLLUKType1Code {
	#[serde(rename = "AssessmentOfValueRequiredUnderCOLLUKType1Code")]
	pub assessment_of_value_required_under_colluk_type1_code: String,
}


// BusinessDayConvention1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BusinessDayConvention1Code {
	#[serde(rename = "BusinessDayConvention1Code")]
	pub business_day_convention1_code: String,
}


// CFIOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[serde(rename = "CFIOct2015Identifier")]
	pub cfi_oct2015_identifier: String,
}


// CashAccount205 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccount205 {
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<String>,
	#[serde(rename = "PmryAcct", skip_serializing_if = "Option::is_none")]
	pub pmry_acct: Option<CashAccount206>,
	#[serde(rename = "ScndryAcct", skip_serializing_if = "Option::is_none")]
	pub scndry_acct: Option<CashAccount206>,
}


// CashAccount206 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccount206 {
	#[serde(rename = "AcctId")]
	pub acct_id: AccountIdentificationAndName7,
	#[serde(rename = "Svcr", skip_serializing_if = "Option::is_none")]
	pub svcr: Option<String>,
	#[serde(rename = "AcctTpDesc", skip_serializing_if = "Option::is_none")]
	pub acct_tp_desc: Option<String>,
}


// CashAccountIdentification8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountIdentification8Choice {
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<GenericAccountIdentification1>,
	#[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
	pub iban: Option<String>,
}


// ChargeType8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ChargeType8Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// ContactAttributes5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContactAttributes5 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress1>,
	#[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
	pub phne_nb: Option<String>,
	#[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
	pub fax_nb: Option<String>,
	#[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
	pub email_adr: Option<String>,
	#[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
	pub url_adr: Option<String>,
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<String>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<String>,
}


// ContactAttributes6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContactAttributes6 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress1>,
	#[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
	pub phne_nb: Option<String>,
	#[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
	pub fax_nb: Option<String>,
	#[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
	pub email_adr: Option<String>,
	#[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
	pub url_adr: Option<String>,
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<String>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<String>,
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


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "DecimalNumber")]
	pub decimal_number: f64,
}


// DistributionPolicy1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DistributionPolicy1Code {
	#[serde(rename = "DistributionPolicy1Code")]
	pub distribution_policy1_code: String,
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


// DistributionStrategy1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DistributionStrategy1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// DividendPolicy1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DividendPolicy1Code {
	#[serde(rename = "DividendPolicy1Code")]
	pub dividend_policy1_code: String,
}


// EMTDataReportingVFMUKType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EMTDataReportingVFMUKType1Code {
	#[serde(rename = "EMTDataReportingVFMUKType1Code")]
	pub emt_data_reporting_vfmuk_type1_code: String,
}


// EUSavingsDirective1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EUSavingsDirective1Code {
	#[serde(rename = "EUSavingsDirective1Code")]
	pub eu_savings_directive1_code: String,
}


// EventFrequency5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EventFrequency5Code {
	#[serde(rename = "EventFrequency5Code")]
	pub event_frequency5_code: String,
}


// EventFrequency8Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EventFrequency8Code {
	#[serde(rename = "EventFrequency8Code")]
	pub event_frequency8_code: String,
}


// ExPostCostCalculationBasis1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExPostCostCalculationBasis1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// ExPostCostCalculationBasis1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExPostCostCalculationBasis1Code {
	#[serde(rename = "ExPostCostCalculationBasis1Code")]
	pub ex_post_cost_calculation_basis1_code: String,
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "Exact4AlphaNumericText")]
	pub exact4_alpha_numeric_text: String,
}


// ExtendedParty13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExtendedParty13 {
	#[serde(rename = "PtyRole")]
	pub pty_role: GenericIdentification36,
	#[serde(rename = "OthrPtyDtls")]
	pub othr_pty_dtls: ContactAttributes5,
}


// Extension1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Extension1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: String,
	#[serde(rename = "Txt")]
	pub txt: String,
}


// ExternalAccountIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAccountIdentification1Code {
	#[serde(rename = "ExternalAccountIdentification1Code")]
	pub external_account_identification1_code: String,
}


// ExternalFinancialInstrumentIdentificationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalFinancialInstrumentIdentificationType1Code {
	#[serde(rename = "ExternalFinancialInstrumentIdentificationType1Code")]
	pub external_financial_instrument_identification_type1_code: String,
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
	pub dstrbtn_plcy: Option<String>,
	#[serde(rename = "DvddPlcy", skip_serializing_if = "Option::is_none")]
	pub dvdd_plcy: Option<String>,
	#[serde(rename = "DvddFrqcy", skip_serializing_if = "Option::is_none")]
	pub dvdd_frqcy: Option<String>,
	#[serde(rename = "RinvstmtFrqcy", skip_serializing_if = "Option::is_none")]
	pub rinvstmt_frqcy: Option<String>,
	#[serde(rename = "FrntEndLd", skip_serializing_if = "Option::is_none")]
	pub frnt_end_ld: Option<bool>,
	#[serde(rename = "BckEndLd", skip_serializing_if = "Option::is_none")]
	pub bck_end_ld: Option<bool>,
	#[serde(rename = "SwtchFee", skip_serializing_if = "Option::is_none")]
	pub swtch_fee: Option<bool>,
	#[serde(rename = "EUSvgsDrctv", skip_serializing_if = "Option::is_none")]
	pub eu_svgs_drctv: Option<String>,
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
	pub may_be_termntd_early: Option<String>,
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
	pub mgmt_fee_src: Option<String>,
	#[serde(rename = "PrfrmncFee", skip_serializing_if = "Option::is_none")]
	pub prfrmnc_fee: Option<bool>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}


// Forms1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Forms1 {
	#[serde(rename = "ApplForm")]
	pub appl_form: bool,
	#[serde(rename = "SgntrTp")]
	pub sgntr_tp: String,
}


// Frequency20Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Frequency20Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// FundOrderType10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundOrderType10Code {
	#[serde(rename = "FundOrderType10Code")]
	pub fund_order_type10_code: String,
}


// FundOrderType5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundOrderType5Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
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


// FundPaymentType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundPaymentType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}


// FundPaymentType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundPaymentType1Code {
	#[serde(rename = "FundPaymentType1Code")]
	pub fund_payment_type1_code: String,
}


// FundReferenceDataReport5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundReferenceDataReport5 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
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
	pub fnd_ref_data_rpt_id: Option<String>,
	#[serde(rename = "Rpt")]
	pub rpt: Vec<FundReferenceDataReport5>,
}


// GenericAccountIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericAccountIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<AccountSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}


// GenericIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}


// GenericIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification3 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}


// GenericIdentification36 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification36 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<String>,
}


// GenericIdentification47 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification47 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<String>,
}


// GovernanceProcess1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GovernanceProcess1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// GovernanceProcessType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GovernanceProcessType1Code {
	#[serde(rename = "GovernanceProcessType1Code")]
	pub governance_process_type1_code: String,
}


// HoldingTransferable1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct HoldingTransferable1Code {
	#[serde(rename = "HoldingTransferable1Code")]
	pub holding_transferable1_code: String,
}


// IBAN2007Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IBAN2007Identifier {
	#[serde(rename = "IBAN2007Identifier")]
	pub iban2007_identifier: String,
}


// ISINOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "ISINOct2015Identifier")]
	pub isin_oct2015_identifier: String,
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


// ISOTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISOTime {
	#[serde(rename = "ISOTime")]
	pub iso_time: String,
}


// ISOYearMonth ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISOYearMonth {
	#[serde(rename = "ISOYearMonth")]
	pub iso_year_month: String,
}


// IdentificationSource3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IdentificationSource3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// IndividualCostOrCharge2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndividualCostOrCharge2 {
	#[serde(rename = "CostTp")]
	pub cost_tp: ChargeType8Choice,
	#[serde(rename = "ExAnteOrExPst")]
	pub ex_ante_or_ex_pst: String,
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


// IntendedOrActual2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IntendedOrActual2Code {
	#[serde(rename = "IntendedOrActual2Code")]
	pub intended_or_actual2_code: String,
}


// InvestmentFundMiFIDFee2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentFundMiFIDFee2Code {
	#[serde(rename = "InvestmentFundMiFIDFee2Code")]
	pub investment_fund_mi_fid_fee2_code: String,
}


// InvestmentFundPlanType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentFundPlanType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}


// InvestmentFundPlanType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentFundPlanType1Code {
	#[serde(rename = "InvestmentFundPlanType1Code")]
	pub investment_fund_plan_type1_code: String,
}


// InvestmentNeed2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentNeed2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// InvestmentNeed2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentNeed2Code {
	#[serde(rename = "InvestmentNeed2Code")]
	pub investment_need2_code: String,
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
	pub othr_red_rstrctns: Option<String>,
	#[serde(rename = "MinSwtchSbcptAmt", skip_serializing_if = "Option::is_none")]
	pub min_swtch_sbcpt_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "MinSwtchSbcptUnits", skip_serializing_if = "Option::is_none")]
	pub min_swtch_sbcpt_units: Option<f64>,
	#[serde(rename = "MaxSwtchRedAmt", skip_serializing_if = "Option::is_none")]
	pub max_swtch_red_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "MaxSwtchRedUnits", skip_serializing_if = "Option::is_none")]
	pub max_swtch_red_units: Option<f64>,
	#[serde(rename = "OthrSwtchRstrctns", skip_serializing_if = "Option::is_none")]
	pub othr_swtch_rstrctns: Option<String>,
	#[serde(rename = "MinHldgAmt", skip_serializing_if = "Option::is_none")]
	pub min_hldg_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "MinHldgUnits", skip_serializing_if = "Option::is_none")]
	pub min_hldg_units: Option<f64>,
	#[serde(rename = "MinHldgPrd", skip_serializing_if = "Option::is_none")]
	pub min_hldg_prd: Option<String>,
	#[serde(rename = "HldgTrfbl", skip_serializing_if = "Option::is_none")]
	pub hldg_trfbl: Option<String>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}


// InvestorKnowledge1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestorKnowledge1 {
	#[serde(rename = "BsicInvstr", skip_serializing_if = "Option::is_none")]
	pub bsic_invstr: Option<String>,
	#[serde(rename = "InfrmdInvstr", skip_serializing_if = "Option::is_none")]
	pub infrmd_invstr: Option<String>,
	#[serde(rename = "AdvncdInvstr", skip_serializing_if = "Option::is_none")]
	pub advncd_invstr: Option<String>,
	#[serde(rename = "ExprtInvstrDE", skip_serializing_if = "Option::is_none")]
	pub exprt_invstr_de: Option<String>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<OtherTargetMarketInvestorKnowledge1>>,
}


// InvestorRequirements4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestorRequirements4 {
	#[serde(rename = "RtrPrflPrsrvtn", skip_serializing_if = "Option::is_none")]
	pub rtr_prfl_prsrvtn: Option<String>,
	#[serde(rename = "RtrPrflGrwth", skip_serializing_if = "Option::is_none")]
	pub rtr_prfl_grwth: Option<String>,
	#[serde(rename = "RtrPrflIncm", skip_serializing_if = "Option::is_none")]
	pub rtr_prfl_incm: Option<String>,
	#[serde(rename = "RtrPrflHdgg", skip_serializing_if = "Option::is_none")]
	pub rtr_prfl_hdgg: Option<String>,
	#[serde(rename = "OptnOrLvrgdRtrPrfl", skip_serializing_if = "Option::is_none")]
	pub optn_or_lvrgd_rtr_prfl: Option<String>,
	#[serde(rename = "RtrPrflPnsnSchmeDE", skip_serializing_if = "Option::is_none")]
	pub rtr_prfl_pnsn_schme_de: Option<String>,
	#[serde(rename = "MinHldgPrd", skip_serializing_if = "Option::is_none")]
	pub min_hldg_prd: Option<TimeHorizon2Choice>,
	#[serde(rename = "SstnbltyPrefs", skip_serializing_if = "Option::is_none")]
	pub sstnblty_prefs: Option<String>,
	#[serde(rename = "OthrSpcfcInvstmtNeed", skip_serializing_if = "Option::is_none")]
	pub othr_spcfc_invstmt_need: Option<InvestmentNeed2Choice>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<OtherInvestmentNeed1>>,
}


// InvestorType2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestorType2 {
	#[serde(rename = "InvstrTpRtl", skip_serializing_if = "Option::is_none")]
	pub invstr_tp_rtl: Option<String>,
	#[serde(rename = "InvstrTpPrfssnl", skip_serializing_if = "Option::is_none")]
	pub invstr_tp_prfssnl: Option<TargetMarket5Choice>,
	#[serde(rename = "InvstrTpElgblCtrPty", skip_serializing_if = "Option::is_none")]
	pub invstr_tp_elgbl_ctr_pty: Option<String>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<OtherTargetMarketInvestor1>>,
}


// InvestorType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestorType2Code {
	#[serde(rename = "InvestorType2Code")]
	pub investor_type2_code: String,
}


// InvestorType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestorType3Code {
	#[serde(rename = "InvestorType3Code")]
	pub investor_type3_code: String,
}


// InvestorType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestorType4Code {
	#[serde(rename = "InvestorType4Code")]
	pub investor_type4_code: String,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LocalMarketAnnex6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LocalMarketAnnex6 {
	#[serde(rename = "Ctry")]
	pub ctry: Vec<String>,
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


// LossBearing2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LossBearing2 {
	#[serde(rename = "NoCptlLoss", skip_serializing_if = "Option::is_none")]
	pub no_cptl_loss: Option<String>,
	#[serde(rename = "LtdCptlLoss", skip_serializing_if = "Option::is_none")]
	pub ltd_cptl_loss: Option<String>,
	#[serde(rename = "LtdCptlLossLvl", skip_serializing_if = "Option::is_none")]
	pub ltd_cptl_loss_lvl: Option<f64>,
	#[serde(rename = "NoCptlGrnt", skip_serializing_if = "Option::is_none")]
	pub no_cptl_grnt: Option<String>,
	#[serde(rename = "LossByndCptl", skip_serializing_if = "Option::is_none")]
	pub loss_bynd_cptl: Option<String>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<OtherTargetMarketLossBearing1>>,
}


// MainFundOrderDeskLocation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MainFundOrderDeskLocation1 {
	#[serde(rename = "Ctry")]
	pub ctry: String,
	#[serde(rename = "TmZoneOffSet")]
	pub tm_zone_off_set: UTCOffset1,
}


// MarketPracticeVersion1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketPracticeVersion1 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
	pub nb: Option<String>,
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


// Max1Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max1Number {
	#[serde(rename = "Max1Number")]
	pub max1_number: f64,
}


// Max2048Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max2048Text {
	#[serde(rename = "Max2048Text")]
	pub max2048_text: String,
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


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Max4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "Max4AlphaNumericText")]
	pub max4_alpha_numeric_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// MessageIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
}


// NameAndAddress5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<PostalAddress1>,
}


// NotionalOrUnitBased1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalOrUnitBased1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// NotionalOrUnitBased1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalOrUnitBased1Code {
	#[serde(rename = "NotionalOrUnitBased1Code")]
	pub notional_or_unit_based1_code: String,
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
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


// OtherDistributionStrategy1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherDistributionStrategy1 {
	#[serde(rename = "DstrbtnStrtgyTp", skip_serializing_if = "Option::is_none")]
	pub dstrbtn_strtgy_tp: Option<String>,
	#[serde(rename = "Trgt", skip_serializing_if = "Option::is_none")]
	pub trgt: Option<DistributionStrategy1Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<AdditionalInformation15>,
}


// OtherIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Sfx", skip_serializing_if = "Option::is_none")]
	pub sfx: Option<String>,
	#[serde(rename = "Tp")]
	pub tp: IdentificationSource3Choice,
}


// OtherInvestmentNeed1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherInvestmentNeed1 {
	#[serde(rename = "ClntObjctvsAndNeedsTp", skip_serializing_if = "Option::is_none")]
	pub clnt_objctvs_and_needs_tp: Option<String>,
	#[serde(rename = "Trgt", skip_serializing_if = "Option::is_none")]
	pub trgt: Option<TargetMarket1Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<AdditionalInformation15>,
}


// OtherReviewRelatedToValueAndOrChargesUKType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherReviewRelatedToValueAndOrChargesUKType1Code {
	#[serde(rename = "OtherReviewRelatedToValueAndOrChargesUKType1Code")]
	pub other_review_related_to_value_and_or_charges_uk_type1_code: String,
}


// OtherTargetMarket1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherTargetMarket1 {
	#[serde(rename = "TrgtMktTp")]
	pub trgt_mkt_tp: String,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<AdditionalInformation15>,
}


// OtherTargetMarketInvestor1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherTargetMarketInvestor1 {
	#[serde(rename = "InvstrTp", skip_serializing_if = "Option::is_none")]
	pub invstr_tp: Option<String>,
	#[serde(rename = "Trgt", skip_serializing_if = "Option::is_none")]
	pub trgt: Option<TargetMarket3Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<AdditionalInformation15>,
}


// OtherTargetMarketInvestorKnowledge1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherTargetMarketInvestorKnowledge1 {
	#[serde(rename = "InvstrKnwldgTp", skip_serializing_if = "Option::is_none")]
	pub invstr_knwldg_tp: Option<String>,
	#[serde(rename = "Trgt", skip_serializing_if = "Option::is_none")]
	pub trgt: Option<TargetMarket1Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<AdditionalInformation15>,
}


// OtherTargetMarketLossBearing1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherTargetMarketLossBearing1 {
	#[serde(rename = "AbltyToBearLossesTp", skip_serializing_if = "Option::is_none")]
	pub ablty_to_bear_losses_tp: Option<String>,
	#[serde(rename = "Trgt", skip_serializing_if = "Option::is_none")]
	pub trgt: Option<TargetMarket1Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<AdditionalInformation15>,
}


// OtherTargetMarketRiskTolerance1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherTargetMarketRiskTolerance1 {
	#[serde(rename = "RskTlrnceTp", skip_serializing_if = "Option::is_none")]
	pub rsk_tlrnce_tp: Option<String>,
	#[serde(rename = "Trgt", skip_serializing_if = "Option::is_none")]
	pub trgt: Option<TargetMarket1Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<AdditionalInformation15>,
}


// OutcomeOfCOLLAssessmentOfValueUKType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OutcomeOfCOLLAssessmentOfValueUKType1Code {
	#[serde(rename = "OutcomeOfCOLLAssessmentOfValueUKType1Code")]
	pub outcome_of_coll_assessment_of_value_uk_type1_code: String,
}


// OutcomeOfPRINValueAssessmentOrReviewUKType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OutcomeOfPRINValueAssessmentOrReviewUKType1Code {
	#[serde(rename = "OutcomeOfPRINValueAssessmentOrReviewUKType1Code")]
	pub outcome_of_prin_value_assessment_or_review_uk_type1_code: String,
}


// PartyIdentification125Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification125Choice {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<String>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification1>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
}


// PartyIdentification139 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification139 {
	#[serde(rename = "Pty")]
	pub pty: PartyIdentification125Choice,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<String>,
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


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// Period15 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period15 {
	#[serde(rename = "StartDt")]
	pub start_dt: String,
	#[serde(rename = "EndDt")]
	pub end_dt: String,
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


// PostalAddress1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress1 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<String>,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
	pub adr_line: Option<Vec<String>>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<String>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<String>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<String>,
	#[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
	pub twn_nm: Option<String>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<String>,
	#[serde(rename = "Ctry")]
	pub ctry: String,
}


// PriceMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceMethod1Code {
	#[serde(rename = "PriceMethod1Code")]
	pub price_method1_code: String,
}


// ProcessingCharacteristics10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProcessingCharacteristics10 {
	#[serde(rename = "DealgCcyAccptd", skip_serializing_if = "Option::is_none")]
	pub dealg_ccy_accptd: Option<Vec<String>>,
	#[serde(rename = "RedAuthstn", skip_serializing_if = "Option::is_none")]
	pub red_authstn: Option<Forms1>,
	#[serde(rename = "AmtInd", skip_serializing_if = "Option::is_none")]
	pub amt_ind: Option<bool>,
	#[serde(rename = "UnitsInd", skip_serializing_if = "Option::is_none")]
	pub units_ind: Option<bool>,
	#[serde(rename = "Rndg", skip_serializing_if = "Option::is_none")]
	pub rndg: Option<String>,
	#[serde(rename = "PctgInd", skip_serializing_if = "Option::is_none")]
	pub pctg_ind: Option<bool>,
	#[serde(rename = "MainFndOrdrDskLctn", skip_serializing_if = "Option::is_none")]
	pub main_fnd_ordr_dsk_lctn: Option<MainFundOrderDeskLocation1>,
	#[serde(rename = "DealgFrqcy", skip_serializing_if = "Option::is_none")]
	pub dealg_frqcy: Option<String>,
	#[serde(rename = "DealgFrqcyDesc", skip_serializing_if = "Option::is_none")]
	pub dealg_frqcy_desc: Option<String>,
	#[serde(rename = "DealgCutOffTm", skip_serializing_if = "Option::is_none")]
	pub dealg_cut_off_tm: Option<String>,
	#[serde(rename = "DealgCutOffTmFrame", skip_serializing_if = "Option::is_none")]
	pub dealg_cut_off_tm_frame: Option<TimeFrame9>,
	#[serde(rename = "DealConfTm", skip_serializing_if = "Option::is_none")]
	pub deal_conf_tm: Option<String>,
	#[serde(rename = "DealConfTmFrame", skip_serializing_if = "Option::is_none")]
	pub deal_conf_tm_frame: Option<TimeFrame8>,
	#[serde(rename = "LtdPrd", skip_serializing_if = "Option::is_none")]
	pub ltd_prd: Option<String>,
	#[serde(rename = "SttlmCycl", skip_serializing_if = "Option::is_none")]
	pub sttlm_cycl: Option<TimeFrame8Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}


// ProcessingCharacteristics11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProcessingCharacteristics11 {
	#[serde(rename = "DealgCcyAccptd", skip_serializing_if = "Option::is_none")]
	pub dealg_ccy_accptd: Option<Vec<String>>,
	#[serde(rename = "InitlInvstmtAppl", skip_serializing_if = "Option::is_none")]
	pub initl_invstmt_appl: Option<Forms1>,
	#[serde(rename = "SbsqntInvstmtAppl", skip_serializing_if = "Option::is_none")]
	pub sbsqnt_invstmt_appl: Option<Forms1>,
	#[serde(rename = "AmtInd", skip_serializing_if = "Option::is_none")]
	pub amt_ind: Option<bool>,
	#[serde(rename = "UnitsInd", skip_serializing_if = "Option::is_none")]
	pub units_ind: Option<bool>,
	#[serde(rename = "Rndg", skip_serializing_if = "Option::is_none")]
	pub rndg: Option<String>,
	#[serde(rename = "MainFndOrdrDskLctn", skip_serializing_if = "Option::is_none")]
	pub main_fnd_ordr_dsk_lctn: Option<MainFundOrderDeskLocation1>,
	#[serde(rename = "DealgFrqcy", skip_serializing_if = "Option::is_none")]
	pub dealg_frqcy: Option<String>,
	#[serde(rename = "DealgFrqcyDesc", skip_serializing_if = "Option::is_none")]
	pub dealg_frqcy_desc: Option<String>,
	#[serde(rename = "DealgCutOffTm", skip_serializing_if = "Option::is_none")]
	pub dealg_cut_off_tm: Option<String>,
	#[serde(rename = "DealgCutOffTmFrame", skip_serializing_if = "Option::is_none")]
	pub dealg_cut_off_tm_frame: Option<TimeFrame9>,
	#[serde(rename = "DealConfTm", skip_serializing_if = "Option::is_none")]
	pub deal_conf_tm: Option<String>,
	#[serde(rename = "DealConfTmFrame", skip_serializing_if = "Option::is_none")]
	pub deal_conf_tm_frame: Option<TimeFrame11>,
	#[serde(rename = "LtdPrd", skip_serializing_if = "Option::is_none")]
	pub ltd_prd: Option<String>,
	#[serde(rename = "SttlmCycl", skip_serializing_if = "Option::is_none")]
	pub sttlm_cycl: Option<TimeFrame7Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}


// ProcessingCharacteristics12 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProcessingCharacteristics12 {
	#[serde(rename = "DealgCcyAccptd", skip_serializing_if = "Option::is_none")]
	pub dealg_ccy_accptd: Option<Vec<String>>,
	#[serde(rename = "RedAuthstn", skip_serializing_if = "Option::is_none")]
	pub red_authstn: Option<Forms1>,
	#[serde(rename = "AmtInd", skip_serializing_if = "Option::is_none")]
	pub amt_ind: Option<bool>,
	#[serde(rename = "UnitsInd", skip_serializing_if = "Option::is_none")]
	pub units_ind: Option<bool>,
	#[serde(rename = "Rndg", skip_serializing_if = "Option::is_none")]
	pub rndg: Option<String>,
	#[serde(rename = "PctgInd", skip_serializing_if = "Option::is_none")]
	pub pctg_ind: Option<bool>,
	#[serde(rename = "MainFndOrdrDskLctn", skip_serializing_if = "Option::is_none")]
	pub main_fnd_ordr_dsk_lctn: Option<MainFundOrderDeskLocation1>,
	#[serde(rename = "DealgFrqcy", skip_serializing_if = "Option::is_none")]
	pub dealg_frqcy: Option<String>,
	#[serde(rename = "DealgFrqcyDesc", skip_serializing_if = "Option::is_none")]
	pub dealg_frqcy_desc: Option<String>,
	#[serde(rename = "DealgCutOffTm", skip_serializing_if = "Option::is_none")]
	pub dealg_cut_off_tm: Option<String>,
	#[serde(rename = "DealgCutOffTmFrame", skip_serializing_if = "Option::is_none")]
	pub dealg_cut_off_tm_frame: Option<TimeFrame9>,
	#[serde(rename = "DealConfTm", skip_serializing_if = "Option::is_none")]
	pub deal_conf_tm: Option<String>,
	#[serde(rename = "DealConfTmFrame", skip_serializing_if = "Option::is_none")]
	pub deal_conf_tm_frame: Option<TimeFrame10>,
	#[serde(rename = "LtdPrd", skip_serializing_if = "Option::is_none")]
	pub ltd_prd: Option<String>,
	#[serde(rename = "SttlmCycl", skip_serializing_if = "Option::is_none")]
	pub sttlm_cycl: Option<TimeFrame8Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}


// ProcessingCharacteristics9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProcessingCharacteristics9 {
	#[serde(rename = "DealgCcyAccptd", skip_serializing_if = "Option::is_none")]
	pub dealg_ccy_accptd: Option<Vec<String>>,
	#[serde(rename = "SwtchAuthstn", skip_serializing_if = "Option::is_none")]
	pub swtch_authstn: Option<Forms1>,
	#[serde(rename = "AmtInd", skip_serializing_if = "Option::is_none")]
	pub amt_ind: Option<bool>,
	#[serde(rename = "UnitsInd", skip_serializing_if = "Option::is_none")]
	pub units_ind: Option<bool>,
	#[serde(rename = "Rndg", skip_serializing_if = "Option::is_none")]
	pub rndg: Option<String>,
	#[serde(rename = "MainFndOrdrDskLctn", skip_serializing_if = "Option::is_none")]
	pub main_fnd_ordr_dsk_lctn: Option<MainFundOrderDeskLocation1>,
	#[serde(rename = "DealgFrqcy", skip_serializing_if = "Option::is_none")]
	pub dealg_frqcy: Option<String>,
	#[serde(rename = "DealgFrqcyDesc", skip_serializing_if = "Option::is_none")]
	pub dealg_frqcy_desc: Option<String>,
	#[serde(rename = "DealgCutOffTm", skip_serializing_if = "Option::is_none")]
	pub dealg_cut_off_tm: Option<String>,
	#[serde(rename = "DealgCutOffTmFrame", skip_serializing_if = "Option::is_none")]
	pub dealg_cut_off_tm_frame: Option<TimeFrame9>,
	#[serde(rename = "DealConfTm", skip_serializing_if = "Option::is_none")]
	pub deal_conf_tm: Option<String>,
	#[serde(rename = "DealConfTmFrame", skip_serializing_if = "Option::is_none")]
	pub deal_conf_tm_frame: Option<TimeFrame8>,
	#[serde(rename = "LtdPrd", skip_serializing_if = "Option::is_none")]
	pub ltd_prd: Option<String>,
	#[serde(rename = "SttlmCycl", skip_serializing_if = "Option::is_none")]
	pub sttlm_cycl: Option<TimeFrame8Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}


// ProductStructure1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProductStructure1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// ProductStructure1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProductStructure1Code {
	#[serde(rename = "ProductStructure1Code")]
	pub product_structure1_code: String,
}


// QuotationType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct QuotationType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// QuotationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct QuotationType1Code {
	#[serde(rename = "QuotationType1Code")]
	pub quotation_type1_code: String,
}


// ReferToFundOrderDesk1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReferToFundOrderDesk1Code {
	#[serde(rename = "ReferToFundOrderDesk1Code")]
	pub refer_to_fund_order_desk1_code: String,
}


// RiskLevel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RiskLevel1Code {
	#[serde(rename = "RiskLevel1Code")]
	pub risk_level1_code: String,
}


// RiskTolerance1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RiskTolerance1 {
	#[serde(rename = "RskTlrncePRIIPSMthdlgy", skip_serializing_if = "Option::is_none")]
	pub rsk_tlrnce_priips_mthdlgy: Option<f64>,
	#[serde(rename = "RskTlrnceUCITSMthdlgy", skip_serializing_if = "Option::is_none")]
	pub rsk_tlrnce_ucits_mthdlgy: Option<f64>,
	#[serde(rename = "RskTlrnceIntl", skip_serializing_if = "Option::is_none")]
	pub rsk_tlrnce_intl: Option<String>,
	#[serde(rename = "RskTlrnceForNonPRIIPSAndNonUCITSES", skip_serializing_if = "Option::is_none")]
	pub rsk_tlrnce_for_non_priips_and_non_ucitses: Option<f64>,
	#[serde(rename = "NotForInvstrsWthTheLwstRskTlrnceDE", skip_serializing_if = "Option::is_none")]
	pub not_for_invstrs_wth_the_lwst_rsk_tlrnce_de: Option<String>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<OtherTargetMarketRiskTolerance1>>,
}


// RoundingDirection2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RoundingDirection2Code {
	#[serde(rename = "RoundingDirection2Code")]
	pub rounding_direction2_code: String,
}


// SecurityClassificationType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityClassificationType2Choice {
	#[serde(rename = "CFI", skip_serializing_if = "Option::is_none")]
	pub cfi: Option<String>,
	#[serde(rename = "AltrnClssfctn", skip_serializing_if = "Option::is_none")]
	pub altrn_clssfctn: Option<GenericIdentification3>,
}


// SecurityIdentification40 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification40 {
	#[serde(rename = "OthrId", skip_serializing_if = "Option::is_none")]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<String>,
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<String>,
}


// SecurityIdentification47 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification47 {
	#[serde(rename = "Id")]
	pub id: SecurityIdentification40,
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
	pub shrt_nm: Option<String>,
	#[serde(rename = "ClssTp", skip_serializing_if = "Option::is_none")]
	pub clss_tp: Option<String>,
	#[serde(rename = "UmbrllNm", skip_serializing_if = "Option::is_none")]
	pub umbrll_nm: Option<String>,
	#[serde(rename = "NewUmbrll", skip_serializing_if = "Option::is_none")]
	pub new_umbrll: Option<bool>,
	#[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
	pub clssfctn_tp: Option<SecurityClassificationType2Choice>,
	#[serde(rename = "BaseCcy", skip_serializing_if = "Option::is_none")]
	pub base_ccy: Option<String>,
	#[serde(rename = "CtryOfDmcl", skip_serializing_if = "Option::is_none")]
	pub ctry_of_dmcl: Option<String>,
	#[serde(rename = "RegdDstrbtnCtry", skip_serializing_if = "Option::is_none")]
	pub regd_dstrbtn_ctry: Option<Vec<String>>,
	#[serde(rename = "PdctTp", skip_serializing_if = "Option::is_none")]
	pub pdct_tp: Option<ProductStructure1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<ContactAttributes5>,
	#[serde(rename = "IssrPdctGovncPrc", skip_serializing_if = "Option::is_none")]
	pub issr_pdct_govnc_prc: Option<GovernanceProcess1Choice>,
	#[serde(rename = "PdctCtgy", skip_serializing_if = "Option::is_none")]
	pub pdct_ctgy: Option<String>,
	#[serde(rename = "PdctCtgyDE", skip_serializing_if = "Option::is_none")]
	pub pdct_ctgy_de: Option<String>,
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


// SignatureType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SignatureType1Code {
	#[serde(rename = "SignatureType1Code")]
	pub signature_type1_code: String,
}


// SustainabilityPreferences2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SustainabilityPreferences2Code {
	#[serde(rename = "SustainabilityPreferences2Code")]
	pub sustainability_preferences2_code: String,
}


// TargetMarket1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TargetMarket1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// TargetMarket1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TargetMarket1Code {
	#[serde(rename = "TargetMarket1Code")]
	pub target_market1_code: String,
}


// TargetMarket2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TargetMarket2Code {
	#[serde(rename = "TargetMarket2Code")]
	pub target_market2_code: String,
}


// TargetMarket3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TargetMarket3Choice {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<String>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// TargetMarket3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TargetMarket3Code {
	#[serde(rename = "TargetMarket3Code")]
	pub target_market3_code: String,
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


// TargetMarket5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TargetMarket5Choice {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<String>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<String>,
}


// TimeFrame10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeFrame10 {
	#[serde(rename = "OthrTmFrameDesc", skip_serializing_if = "Option::is_none")]
	pub othr_tm_frame_desc: Option<String>,
	#[serde(rename = "TPlus", skip_serializing_if = "Option::is_none")]
	pub t_plus: Option<f64>,
	#[serde(rename = "NonWorkgDayAdjstmnt", skip_serializing_if = "Option::is_none")]
	pub non_workg_day_adjstmnt: Option<String>,
	#[serde(rename = "RefrToOrdrDsk", skip_serializing_if = "Option::is_none")]
	pub refr_to_ordr_dsk: Option<String>,
}


// TimeFrame11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeFrame11 {
	#[serde(rename = "OthrTmFrameDesc", skip_serializing_if = "Option::is_none")]
	pub othr_tm_frame_desc: Option<String>,
	#[serde(rename = "TPlus", skip_serializing_if = "Option::is_none")]
	pub t_plus: Option<f64>,
	#[serde(rename = "NonWorkgDayAdjstmnt", skip_serializing_if = "Option::is_none")]
	pub non_workg_day_adjstmnt: Option<String>,
	#[serde(rename = "RefrToOrdrDsk", skip_serializing_if = "Option::is_none")]
	pub refr_to_ordr_dsk: Option<String>,
}


// TimeFrame2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeFrame2Code {
	#[serde(rename = "TimeFrame2Code")]
	pub time_frame2_code: String,
}


// TimeFrame7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeFrame7Choice {
	#[serde(rename = "TPlus", skip_serializing_if = "Option::is_none")]
	pub t_plus: Option<f64>,
	#[serde(rename = "Prepmt", skip_serializing_if = "Option::is_none")]
	pub prepmt: Option<bool>,
}


// TimeFrame8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeFrame8 {
	#[serde(rename = "OthrTmFrameDesc", skip_serializing_if = "Option::is_none")]
	pub othr_tm_frame_desc: Option<String>,
	#[serde(rename = "TPlus", skip_serializing_if = "Option::is_none")]
	pub t_plus: Option<f64>,
	#[serde(rename = "NonWorkgDayAdjstmnt", skip_serializing_if = "Option::is_none")]
	pub non_workg_day_adjstmnt: Option<String>,
	#[serde(rename = "RefrToOrdrDsk", skip_serializing_if = "Option::is_none")]
	pub refr_to_ordr_dsk: Option<String>,
}


// TimeFrame8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeFrame8Choice {
	#[serde(rename = "TPlus", skip_serializing_if = "Option::is_none")]
	pub t_plus: Option<f64>,
	#[serde(rename = "RPlus", skip_serializing_if = "Option::is_none")]
	pub r_plus: Option<f64>,
}


// TimeFrame9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeFrame9 {
	#[serde(rename = "OthrTmFrameDesc", skip_serializing_if = "Option::is_none")]
	pub othr_tm_frame_desc: Option<String>,
	#[serde(rename = "TMns", skip_serializing_if = "Option::is_none")]
	pub t_mns: Option<f64>,
	#[serde(rename = "NonWorkgDayAdjstmnt", skip_serializing_if = "Option::is_none")]
	pub non_workg_day_adjstmnt: Option<String>,
	#[serde(rename = "RefrToOrdrDsk", skip_serializing_if = "Option::is_none")]
	pub refr_to_ordr_dsk: Option<String>,
}


// TimeFrame9Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeFrame9Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// TimeHorizon2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeHorizon2Choice {
	#[serde(rename = "NbOfYrs", skip_serializing_if = "Option::is_none")]
	pub nb_of_yrs: Option<f64>,
	#[serde(rename = "TmFrame", skip_serializing_if = "Option::is_none")]
	pub tm_frame: Option<TimeFrame9Choice>,
}


// UTCOffset1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UTCOffset1 {
	#[serde(rename = "Sgn")]
	pub sgn: bool,
	#[serde(rename = "NbOfHrs")]
	pub nb_of_hrs: String,
}


// UnitsOrAmount1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitsOrAmount1Choice {
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
	pub unit: Option<f64>,
}


// ValuationDealingProcessingCharacteristics3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ValuationDealingProcessingCharacteristics3 {
	#[serde(rename = "ValtnFrqcy", skip_serializing_if = "Option::is_none")]
	pub valtn_frqcy: Option<String>,
	#[serde(rename = "ValtnFrqcyDesc", skip_serializing_if = "Option::is_none")]
	pub valtn_frqcy_desc: Option<String>,
	#[serde(rename = "ValtnTm", skip_serializing_if = "Option::is_none")]
	pub valtn_tm: Option<String>,
	#[serde(rename = "DcmlstnUnits", skip_serializing_if = "Option::is_none")]
	pub dcmlstn_units: Option<f64>,
	#[serde(rename = "DcmlstnPric", skip_serializing_if = "Option::is_none")]
	pub dcmlstn_pric: Option<f64>,
	#[serde(rename = "DualFndInd", skip_serializing_if = "Option::is_none")]
	pub dual_fnd_ind: Option<bool>,
	#[serde(rename = "PricMtd", skip_serializing_if = "Option::is_none")]
	pub pric_mtd: Option<String>,
	#[serde(rename = "PricCcy", skip_serializing_if = "Option::is_none")]
	pub pric_ccy: Option<Vec<String>>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<AdditionalInformation15>>,
}


// ValueForMoney1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ValueForMoney1 {
	#[serde(rename = "EMTDataRptgVFMUK", skip_serializing_if = "Option::is_none")]
	pub emt_data_rptg_vfmuk: Option<String>,
	#[serde(rename = "AssmntOfValReqrdUdrCOLLUK", skip_serializing_if = "Option::is_none")]
	pub assmnt_of_val_reqrd_udr_colluk: Option<String>,
	#[serde(rename = "OutcmOfCOLLAssmntOfValUK", skip_serializing_if = "Option::is_none")]
	pub outcm_of_coll_assmnt_of_val_uk: Option<String>,
	#[serde(rename = "OutcmOfPRINValAssmntOrRvwUK", skip_serializing_if = "Option::is_none")]
	pub outcm_of_prin_val_assmnt_or_rvw_uk: Option<String>,
	#[serde(rename = "OthrRvwRltdToValAndOrChrgsUK", skip_serializing_if = "Option::is_none")]
	pub othr_rvw_rltd_to_val_and_or_chrgs_uk: Option<String>,
	#[serde(rename = "FrthrInfUK", skip_serializing_if = "Option::is_none")]
	pub frthr_inf_uk: Option<String>,
	#[serde(rename = "RvwDtUK", skip_serializing_if = "Option::is_none")]
	pub rvw_dt_uk: Option<String>,
	#[serde(rename = "RvwNxtDueUK", skip_serializing_if = "Option::is_none")]
	pub rvw_nxt_due_uk: Option<String>,
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "YesNoIndicator")]
	pub yes_no_indicator: bool,
}
