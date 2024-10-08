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


// AccountInterest4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountInterest4 {
	#[serde(rename = "Tp")]
	pub tp: Option<InterestType1Choice>,
	#[serde(rename = "Rate")]
	pub rate: Option<Vec<Rate4>>,
	#[serde(rename = "FrToDt")]
	pub fr_to_dt: Option<DateTimePeriod1>,
	#[serde(rename = "Rsn")]
	pub rsn: Option<String>,
	#[serde(rename = "Tax")]
	pub tax: Option<TaxCharges2>,
}


// AccountReport25 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountReport25 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "RptPgntn")]
	pub rpt_pgntn: Option<Pagination1>,
	#[serde(rename = "ElctrncSeqNb")]
	pub elctrnc_seq_nb: Option<f64>,
	#[serde(rename = "RptgSeq")]
	pub rptg_seq: Option<SequenceRange1Choice>,
	#[serde(rename = "LglSeqNb")]
	pub lgl_seq_nb: Option<f64>,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: Option<String>,
	#[serde(rename = "FrToDt")]
	pub fr_to_dt: Option<DateTimePeriod1>,
	#[serde(rename = "CpyDplctInd")]
	pub cpy_dplct_ind: Option<String>,
	#[serde(rename = "RptgSrc")]
	pub rptg_src: Option<ReportingSource1Choice>,
	#[serde(rename = "Acct")]
	pub acct: CashAccount39,
	#[serde(rename = "RltdAcct")]
	pub rltd_acct: Option<CashAccount38>,
	#[serde(rename = "Intrst")]
	pub intrst: Option<Vec<AccountInterest4>>,
	#[serde(rename = "Bal")]
	pub bal: Option<Vec<CashBalance8>>,
	#[serde(rename = "TxsSummry")]
	pub txs_summry: Option<TotalTransactions6>,
	#[serde(rename = "Ntry")]
	pub ntry: Option<Vec<ReportEntry10>>,
	#[serde(rename = "AddtlRptInf")]
	pub addtl_rpt_inf: Option<String>,
}


// AccountSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
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


// ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType {
	#[serde(rename = "ActiveOrHistoricCurrencyAnd13DecimalAmount_SimpleType")]
	pub active_or_historic_currency_and13_decimal_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAnd13DecimalAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
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


// ActiveOrHistoricCurrencyAndAmountRange2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmountRange2 {
	#[serde(rename = "Amt")]
	pub amt: ImpliedCurrencyAmountRange1Choice,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: Option<String>,
	#[serde(rename = "Ccy")]
	pub ccy: String,
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


// AmountAndCurrencyExchange3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndCurrencyExchange3 {
	#[serde(rename = "InstdAmt")]
	pub instd_amt: Option<AmountAndCurrencyExchangeDetails3>,
	#[serde(rename = "TxAmt")]
	pub tx_amt: Option<AmountAndCurrencyExchangeDetails3>,
	#[serde(rename = "CntrValAmt")]
	pub cntr_val_amt: Option<AmountAndCurrencyExchangeDetails3>,
	#[serde(rename = "AnncdPstngAmt")]
	pub anncd_pstng_amt: Option<AmountAndCurrencyExchangeDetails3>,
	#[serde(rename = "PrtryAmt")]
	pub prtry_amt: Option<Vec<AmountAndCurrencyExchangeDetails4>>,
}


// AmountAndCurrencyExchangeDetails3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndCurrencyExchangeDetails3 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CcyXchg")]
	pub ccy_xchg: Option<CurrencyExchange5>,
}


// AmountAndCurrencyExchangeDetails4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndCurrencyExchangeDetails4 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CcyXchg")]
	pub ccy_xchg: Option<CurrencyExchange5>,
}


// AmountAndDirection35 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection35 {
	#[serde(rename = "Amt")]
	pub amt: f64,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: String,
}


// AmountRangeBoundary1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountRangeBoundary1 {
	#[serde(rename = "BdryAmt")]
	pub bdry_amt: f64,
	#[serde(rename = "Incl")]
	pub incl: bool,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// AttendanceContext1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AttendanceContext1Code {
	#[serde(rename = "AttendanceContext1Code")]
	pub attendance_context1_code: String,
}


// AuthenticationEntity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AuthenticationEntity1Code {
	#[serde(rename = "AuthenticationEntity1Code")]
	pub authentication_entity1_code: String,
}


// AuthenticationMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AuthenticationMethod1Code {
	#[serde(rename = "AuthenticationMethod1Code")]
	pub authentication_method1_code: String,
}


// BICFIDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BICFIDec2014Identifier {
	#[serde(rename = "BICFIDec2014Identifier")]
	pub bicfi_dec2014_identifier: String,
}


// BalanceSubType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BalanceSubType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// BalanceType10Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BalanceType10Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// BalanceType13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BalanceType13 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: BalanceType10Choice,
	#[serde(rename = "SubTp")]
	pub sub_tp: Option<BalanceSubType1Choice>,
}


// BankToCustomerAccountReportV08 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BankToCustomerAccountReportV08 {
	#[serde(rename = "GrpHdr")]
	pub grp_hdr: GroupHeader81,
	#[serde(rename = "Rpt")]
	pub rpt: Vec<AccountReport25>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
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


// BatchInformation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BatchInformation2 {
	#[serde(rename = "MsgId")]
	pub msg_id: Option<String>,
	#[serde(rename = "PmtInfId")]
	pub pmt_inf_id: Option<String>,
	#[serde(rename = "NbOfTxs")]
	pub nb_of_txs: Option<String>,
	#[serde(rename = "TtlAmt")]
	pub ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: Option<String>,
}


// BranchAndFinancialInstitutionIdentification6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BranchAndFinancialInstitutionIdentification6 {
	#[serde(rename = "FinInstnId")]
	pub fin_instn_id: FinancialInstitutionIdentification18,
	#[serde(rename = "BrnchId")]
	pub brnch_id: Option<BranchData3>,
}


// BranchData3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BranchData3 {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress24>,
}


// CSCManagement1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CSCManagement1Code {
	#[serde(rename = "CSCManagement1Code")]
	pub csc_management1_code: String,
}


// CardAggregated2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardAggregated2 {
	#[serde(rename = "AddtlSvc")]
	pub addtl_svc: Option<String>,
	#[serde(rename = "TxCtgy")]
	pub tx_ctgy: Option<String>,
	#[serde(rename = "SaleRcncltnId")]
	pub sale_rcncltn_id: Option<String>,
	#[serde(rename = "SeqNbRg")]
	pub seq_nb_rg: Option<CardSequenceNumberRange1>,
	#[serde(rename = "TxDtRg")]
	pub tx_dt_rg: Option<DateOrDateTimePeriod1Choice>,
}


// CardDataReading1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardDataReading1Code {
	#[serde(rename = "CardDataReading1Code")]
	pub card_data_reading1_code: String,
}


// CardEntry4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardEntry4 {
	#[serde(rename = "Card")]
	pub card: Option<PaymentCard4>,
	#[serde(rename = "POI")]
	pub poi: Option<PointOfInteraction1>,
	#[serde(rename = "AggtdNtry")]
	pub aggtd_ntry: Option<CardAggregated2>,
	#[serde(rename = "PrePdAcct")]
	pub pre_pd_acct: Option<CashAccount38>,
}


// CardIndividualTransaction2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardIndividualTransaction2 {
	#[serde(rename = "ICCRltdData")]
	pub icc_rltd_data: Option<String>,
	#[serde(rename = "PmtCntxt")]
	pub pmt_cntxt: Option<PaymentContext3>,
	#[serde(rename = "AddtlSvc")]
	pub addtl_svc: Option<String>,
	#[serde(rename = "TxCtgy")]
	pub tx_ctgy: Option<String>,
	#[serde(rename = "SaleRcncltnId")]
	pub sale_rcncltn_id: Option<String>,
	#[serde(rename = "SaleRefNb")]
	pub sale_ref_nb: Option<String>,
	#[serde(rename = "RePresntmntRsn")]
	pub re_presntmnt_rsn: Option<String>,
	#[serde(rename = "SeqNb")]
	pub seq_nb: Option<String>,
	#[serde(rename = "TxId")]
	pub tx_id: Option<TransactionIdentifier1>,
	#[serde(rename = "Pdct")]
	pub pdct: Option<Product2>,
	#[serde(rename = "VldtnDt")]
	pub vldtn_dt: Option<String>,
	#[serde(rename = "VldtnSeqNb")]
	pub vldtn_seq_nb: Option<String>,
}


// CardPaymentServiceType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardPaymentServiceType2Code {
	#[serde(rename = "CardPaymentServiceType2Code")]
	pub card_payment_service_type2_code: String,
}


// CardSecurityInformation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardSecurityInformation1 {
	#[serde(rename = "CSCMgmt")]
	pub csc_mgmt: String,
	#[serde(rename = "CSCVal")]
	pub csc_val: Option<String>,
}


// CardSequenceNumberRange1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardSequenceNumberRange1 {
	#[serde(rename = "FrstTx")]
	pub frst_tx: Option<String>,
	#[serde(rename = "LastTx")]
	pub last_tx: Option<String>,
}


// CardTransaction17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardTransaction17 {
	#[serde(rename = "Card")]
	pub card: Option<PaymentCard4>,
	#[serde(rename = "POI")]
	pub poi: Option<PointOfInteraction1>,
	#[serde(rename = "Tx")]
	pub tx: Option<CardTransaction3Choice>,
	#[serde(rename = "PrePdAcct")]
	pub pre_pd_acct: Option<CashAccount38>,
}


// CardTransaction3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardTransaction3Choice {
	#[serde(rename = "Aggtd")]
	pub aggtd: Option<CardAggregated2>,
	#[serde(rename = "Indv")]
	pub indv: Option<CardIndividualTransaction2>,
}


// CardholderAuthentication2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardholderAuthentication2 {
	#[serde(rename = "AuthntcnMtd")]
	pub authntcn_mtd: String,
	#[serde(rename = "AuthntcnNtty")]
	pub authntcn_ntty: String,
}


// CardholderVerificationCapability1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardholderVerificationCapability1Code {
	#[serde(rename = "CardholderVerificationCapability1Code")]
	pub cardholder_verification_capability1_code: String,
}


// CashAccount38 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccount38 {
	#[serde(rename = "Id")]
	pub id: AccountIdentification4Choice,
	#[serde(rename = "Tp")]
	pub tp: Option<CashAccountType2Choice>,
	#[serde(rename = "Ccy")]
	pub ccy: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Prxy")]
	pub prxy: Option<ProxyAccountIdentification1>,
}


// CashAccount39 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccount39 {
	#[serde(rename = "Id")]
	pub id: AccountIdentification4Choice,
	#[serde(rename = "Tp")]
	pub tp: Option<CashAccountType2Choice>,
	#[serde(rename = "Ccy")]
	pub ccy: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Prxy")]
	pub prxy: Option<ProxyAccountIdentification1>,
	#[serde(rename = "Ownr")]
	pub ownr: Option<PartyIdentification135>,
	#[serde(rename = "Svcr")]
	pub svcr: Option<BranchAndFinancialInstitutionIdentification6>,
}


// CashAccountType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountType2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// CashAvailability1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAvailability1 {
	#[serde(rename = "Dt")]
	pub dt: CashAvailabilityDate1Choice,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: String,
}


// CashAvailabilityDate1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAvailabilityDate1Choice {
	#[serde(rename = "NbOfDays")]
	pub nb_of_days: Option<String>,
	#[serde(rename = "ActlDt")]
	pub actl_dt: Option<String>,
}


// CashBalance8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashBalance8 {
	#[serde(rename = "Tp")]
	pub tp: BalanceType13,
	#[serde(rename = "CdtLine")]
	pub cdt_line: Option<Vec<CreditLine3>>,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: String,
	#[serde(rename = "Dt")]
	pub dt: DateAndDateTime2Choice,
	#[serde(rename = "Avlbty")]
	pub avlbty: Option<Vec<CashAvailability1>>,
}


// CashDeposit1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashDeposit1 {
	#[serde(rename = "NoteDnmtn")]
	pub note_dnmtn: ActiveCurrencyAndAmount,
	#[serde(rename = "NbOfNotes")]
	pub nb_of_notes: String,
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
}


// ChargeBearerType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ChargeBearerType1Code {
	#[serde(rename = "ChargeBearerType1Code")]
	pub charge_bearer_type1_code: String,
}


// ChargeIncludedIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ChargeIncludedIndicator {
	#[serde(rename = "ChargeIncludedIndicator")]
	pub charge_included_indicator: bool,
}


// ChargeType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ChargeType3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification3>,
}


// Charges6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Charges6 {
	#[serde(rename = "TtlChrgsAndTaxAmt")]
	pub ttl_chrgs_and_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Rcrd")]
	pub rcrd: Option<Vec<ChargesRecord3>>,
}


// ChargesRecord3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ChargesRecord3 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: Option<String>,
	#[serde(rename = "ChrgInclInd")]
	pub chrg_incl_ind: Option<bool>,
	#[serde(rename = "Tp")]
	pub tp: Option<ChargeType3Choice>,
	#[serde(rename = "Rate")]
	pub rate: Option<f64>,
	#[serde(rename = "Br")]
	pub br: Option<String>,
	#[serde(rename = "Agt")]
	pub agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "Tax")]
	pub tax: Option<TaxCharges2>,
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


// Contact4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Contact4 {
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


// CopyDuplicate1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CopyDuplicate1Code {
	#[serde(rename = "CopyDuplicate1Code")]
	pub copy_duplicate1_code: String,
}


// CorporateAction9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CorporateAction9 {
	#[serde(rename = "EvtTp")]
	pub evt_tp: String,
	#[serde(rename = "EvtId")]
	pub evt_id: String,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// CreditDebitCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditDebitCode {
	#[serde(rename = "CreditDebitCode")]
	pub credit_debit_code: String,
}


// CreditLine3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditLine3 {
	#[serde(rename = "Incl")]
	pub incl: bool,
	#[serde(rename = "Tp")]
	pub tp: Option<CreditLineType1Choice>,
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Dt")]
	pub dt: Option<DateAndDateTime2Choice>,
}


// CreditLineType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditLineType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// CreditorReferenceInformation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceInformation2 {
	#[serde(rename = "Tp")]
	pub tp: Option<CreditorReferenceType2>,
	#[serde(rename = "Ref")]
	pub ref_attr: Option<String>,
}


// CreditorReferenceType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// CreditorReferenceType2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceType2 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: CreditorReferenceType1Choice,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// CurrencyExchange5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CurrencyExchange5 {
	#[serde(rename = "SrcCcy")]
	pub src_ccy: String,
	#[serde(rename = "TrgtCcy")]
	pub trgt_ccy: Option<String>,
	#[serde(rename = "UnitCcy")]
	pub unit_ccy: Option<String>,
	#[serde(rename = "XchgRate")]
	pub xchg_rate: f64,
	#[serde(rename = "CtrctId")]
	pub ctrct_id: Option<String>,
	#[serde(rename = "QtnDt")]
	pub qtn_dt: Option<String>,
}


// DateAndDateTime2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTime2Choice {
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm")]
	pub dt_tm: Option<String>,
}


// DateAndPlaceOfBirth1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndPlaceOfBirth1 {
	#[serde(rename = "BirthDt")]
	pub birth_dt: String,
	#[serde(rename = "PrvcOfBirth")]
	pub prvc_of_birth: Option<String>,
	#[serde(rename = "CityOfBirth")]
	pub city_of_birth: String,
	#[serde(rename = "CtryOfBirth")]
	pub ctry_of_birth: String,
}


// DateOrDateTimePeriod1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateOrDateTimePeriod1Choice {
	#[serde(rename = "Dt")]
	pub dt: Option<DatePeriod2>,
	#[serde(rename = "DtTm")]
	pub dt_tm: Option<DateTimePeriod1>,
}


// DatePeriod2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriod2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// DateTimePeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod1 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: String,
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "DecimalNumber")]
	pub decimal_number: f64,
}


// DiscountAmountAndType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DiscountAmountAndType1 {
	#[serde(rename = "Tp")]
	pub tp: Option<DiscountAmountType1Choice>,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}


// DiscountAmountType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DiscountAmountType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// DisplayCapabilities1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DisplayCapabilities1 {
	#[serde(rename = "DispTp")]
	pub disp_tp: String,
	#[serde(rename = "NbOfLines")]
	pub nb_of_lines: String,
	#[serde(rename = "LineWidth")]
	pub line_width: String,
}


// DocumentAdjustment1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentAdjustment1 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: Option<String>,
	#[serde(rename = "Rsn")]
	pub rsn: Option<String>,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<String>,
}


// DocumentLineIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentLineIdentification1 {
	#[serde(rename = "Tp")]
	pub tp: Option<DocumentLineType1>,
	#[serde(rename = "Nb")]
	pub nb: Option<String>,
	#[serde(rename = "RltdDt")]
	pub rltd_dt: Option<String>,
}


// DocumentLineInformation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentLineInformation1 {
	#[serde(rename = "Id")]
	pub id: Vec<DocumentLineIdentification1>,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
	#[serde(rename = "Amt")]
	pub amt: Option<RemittanceAmount3>,
}


// DocumentLineType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentLineType1 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: DocumentLineType1Choice,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// DocumentLineType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentLineType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// DocumentType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentType3Code {
	#[serde(rename = "DocumentType3Code")]
	pub document_type3_code: String,
}


// DocumentType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentType6Code {
	#[serde(rename = "DocumentType6Code")]
	pub document_type6_code: String,
}


// EntryDetails9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EntryDetails9 {
	#[serde(rename = "Btch")]
	pub btch: Option<BatchInformation2>,
	#[serde(rename = "TxDtls")]
	pub tx_dtls: Option<Vec<EntryTransaction10>>,
}


// EntryStatus1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EntryStatus1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// EntryTransaction10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EntryTransaction10 {
	#[serde(rename = "Refs")]
	pub refs: Option<TransactionReferences6>,
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: Option<String>,
	#[serde(rename = "AmtDtls")]
	pub amt_dtls: Option<AmountAndCurrencyExchange3>,
	#[serde(rename = "Avlbty")]
	pub avlbty: Option<Vec<CashAvailability1>>,
	#[serde(rename = "BkTxCd")]
	pub bk_tx_cd: Option<BankTransactionCodeStructure4>,
	#[serde(rename = "Chrgs")]
	pub chrgs: Option<Charges6>,
	#[serde(rename = "Intrst")]
	pub intrst: Option<TransactionInterest4>,
	#[serde(rename = "RltdPties")]
	pub rltd_pties: Option<TransactionParties6>,
	#[serde(rename = "RltdAgts")]
	pub rltd_agts: Option<TransactionAgents5>,
	#[serde(rename = "LclInstrm")]
	pub lcl_instrm: Option<LocalInstrument2Choice>,
	#[serde(rename = "Purp")]
	pub purp: Option<Purpose2Choice>,
	#[serde(rename = "RltdRmtInf")]
	pub rltd_rmt_inf: Option<Vec<RemittanceLocation7>>,
	#[serde(rename = "RmtInf")]
	pub rmt_inf: Option<RemittanceInformation16>,
	#[serde(rename = "RltdDts")]
	pub rltd_dts: Option<TransactionDates3>,
	#[serde(rename = "RltdPric")]
	pub rltd_pric: Option<TransactionPrice4Choice>,
	#[serde(rename = "RltdQties")]
	pub rltd_qties: Option<Vec<TransactionQuantities3Choice>>,
	#[serde(rename = "FinInstrmId")]
	pub fin_instrm_id: Option<SecurityIdentification19>,
	#[serde(rename = "Tax")]
	pub tax: Option<TaxInformation8>,
	#[serde(rename = "RtrInf")]
	pub rtr_inf: Option<PaymentReturnReason5>,
	#[serde(rename = "CorpActn")]
	pub corp_actn: Option<CorporateAction9>,
	#[serde(rename = "SfkpgAcct")]
	pub sfkpg_acct: Option<SecuritiesAccount19>,
	#[serde(rename = "CshDpst")]
	pub csh_dpst: Option<Vec<CashDeposit1>>,
	#[serde(rename = "CardTx")]
	pub card_tx: Option<CardTransaction17>,
	#[serde(rename = "AddtlTxInf")]
	pub addtl_tx_inf: Option<String>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// Exact1NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact1NumericText {
	#[serde(rename = "Exact1NumericText")]
	pub exact1_numeric_text: String,
}


// Exact3NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact3NumericText {
	#[serde(rename = "Exact3NumericText")]
	pub exact3_numeric_text: String,
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


// ExternalBalanceSubType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalBalanceSubType1Code {
	#[serde(rename = "ExternalBalanceSubType1Code")]
	pub external_balance_sub_type1_code: String,
}


// ExternalBalanceType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalBalanceType1Code {
	#[serde(rename = "ExternalBalanceType1Code")]
	pub external_balance_type1_code: String,
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


// ExternalCardTransactionCategory1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalCardTransactionCategory1Code {
	#[serde(rename = "ExternalCardTransactionCategory1Code")]
	pub external_card_transaction_category1_code: String,
}


// ExternalCashAccountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalCashAccountType1Code {
	#[serde(rename = "ExternalCashAccountType1Code")]
	pub external_cash_account_type1_code: String,
}


// ExternalChargeType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalChargeType1Code {
	#[serde(rename = "ExternalChargeType1Code")]
	pub external_charge_type1_code: String,
}


// ExternalClearingSystemIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalClearingSystemIdentification1Code {
	#[serde(rename = "ExternalClearingSystemIdentification1Code")]
	pub external_clearing_system_identification1_code: String,
}


// ExternalCreditLineType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalCreditLineType1Code {
	#[serde(rename = "ExternalCreditLineType1Code")]
	pub external_credit_line_type1_code: String,
}


// ExternalDiscountAmountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalDiscountAmountType1Code {
	#[serde(rename = "ExternalDiscountAmountType1Code")]
	pub external_discount_amount_type1_code: String,
}


// ExternalDocumentLineType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalDocumentLineType1Code {
	#[serde(rename = "ExternalDocumentLineType1Code")]
	pub external_document_line_type1_code: String,
}


// ExternalEntryStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalEntryStatus1Code {
	#[serde(rename = "ExternalEntryStatus1Code")]
	pub external_entry_status1_code: String,
}


// ExternalFinancialInstitutionIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalFinancialInstitutionIdentification1Code {
	#[serde(rename = "ExternalFinancialInstitutionIdentification1Code")]
	pub external_financial_institution_identification1_code: String,
}


// ExternalFinancialInstrumentIdentificationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalFinancialInstrumentIdentificationType1Code {
	#[serde(rename = "ExternalFinancialInstrumentIdentificationType1Code")]
	pub external_financial_instrument_identification_type1_code: String,
}


// ExternalGarnishmentType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalGarnishmentType1Code {
	#[serde(rename = "ExternalGarnishmentType1Code")]
	pub external_garnishment_type1_code: String,
}


// ExternalLocalInstrument1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalLocalInstrument1Code {
	#[serde(rename = "ExternalLocalInstrument1Code")]
	pub external_local_instrument1_code: String,
}


// ExternalOrganisationIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalOrganisationIdentification1Code {
	#[serde(rename = "ExternalOrganisationIdentification1Code")]
	pub external_organisation_identification1_code: String,
}


// ExternalPersonIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPersonIdentification1Code {
	#[serde(rename = "ExternalPersonIdentification1Code")]
	pub external_person_identification1_code: String,
}


// ExternalProxyAccountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalProxyAccountType1Code {
	#[serde(rename = "ExternalProxyAccountType1Code")]
	pub external_proxy_account_type1_code: String,
}


// ExternalPurpose1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPurpose1Code {
	#[serde(rename = "ExternalPurpose1Code")]
	pub external_purpose1_code: String,
}


// ExternalRePresentmentReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalRePresentmentReason1Code {
	#[serde(rename = "ExternalRePresentmentReason1Code")]
	pub external_re_presentment_reason1_code: String,
}


// ExternalReportingSource1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalReportingSource1Code {
	#[serde(rename = "ExternalReportingSource1Code")]
	pub external_reporting_source1_code: String,
}


// ExternalReturnReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalReturnReason1Code {
	#[serde(rename = "ExternalReturnReason1Code")]
	pub external_return_reason1_code: String,
}


// ExternalTaxAmountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalTaxAmountType1Code {
	#[serde(rename = "ExternalTaxAmountType1Code")]
	pub external_tax_amount_type1_code: String,
}


// ExternalTechnicalInputChannel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalTechnicalInputChannel1Code {
	#[serde(rename = "ExternalTechnicalInputChannel1Code")]
	pub external_technical_input_channel1_code: String,
}


// FinancialIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// FinancialInstitutionIdentification18 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstitutionIdentification18 {
	#[serde(rename = "BICFI")]
	pub bicfi: Option<String>,
	#[serde(rename = "ClrSysMmbId")]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress24>,
	#[serde(rename = "Othr")]
	pub othr: Option<GenericFinancialIdentification1>,
}


// FinancialInstrumentQuantity1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentQuantity1Choice {
	#[serde(rename = "Unit")]
	pub unit: Option<f64>,
	#[serde(rename = "FaceAmt")]
	pub face_amt: Option<f64>,
	#[serde(rename = "AmtsdVal")]
	pub amtsd_val: Option<f64>,
}


// FromToAmountRange1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FromToAmountRange1 {
	#[serde(rename = "FrAmt")]
	pub fr_amt: AmountRangeBoundary1,
	#[serde(rename = "ToAmt")]
	pub to_amt: AmountRangeBoundary1,
}


// Garnishment3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Garnishment3 {
	#[serde(rename = "Tp")]
	pub tp: GarnishmentType1,
	#[serde(rename = "Grnshee")]
	pub grnshee: Option<PartyIdentification135>,
	#[serde(rename = "GrnshmtAdmstr")]
	pub grnshmt_admstr: Option<PartyIdentification135>,
	#[serde(rename = "RefNb")]
	pub ref_nb: Option<String>,
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "RmtdAmt")]
	pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "FmlyMdclInsrncInd")]
	pub fmly_mdcl_insrnc_ind: Option<bool>,
	#[serde(rename = "MplyeeTermntnInd")]
	pub mplyee_termntn_ind: Option<bool>,
}


// GarnishmentType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GarnishmentType1 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: GarnishmentType1Choice,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GarnishmentType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GarnishmentType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
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


// GenericIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification3 {
	#[serde(rename = "Id")]
	pub id: String,
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


// GenericIdentification32 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification32 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
	#[serde(rename = "ShrtNm")]
	pub shrt_nm: Option<String>,
}


// GenericOrganisationIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericOrganisationIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericPersonIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericPersonIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GroupHeader81 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GroupHeader81 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
	#[serde(rename = "MsgRcpt")]
	pub msg_rcpt: Option<PartyIdentification135>,
	#[serde(rename = "MsgPgntn")]
	pub msg_pgntn: Option<Pagination1>,
	#[serde(rename = "OrgnlBizQry")]
	pub orgnl_biz_qry: Option<OriginalBusinessQuery1>,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<String>,
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


// ISO2ALanguageCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISO2ALanguageCode {
	#[serde(rename = "ISO2ALanguageCode")]
	pub iso2_a_language_code: String,
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


// ISOYearMonth ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISOYearMonth {
	#[serde(rename = "ISOYearMonth")]
	pub iso_year_month: String,
}


// IdentificationSource3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IdentificationSource3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ImpliedCurrencyAmountRange1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImpliedCurrencyAmountRange1Choice {
	#[serde(rename = "FrAmt")]
	pub fr_amt: Option<AmountRangeBoundary1>,
	#[serde(rename = "ToAmt")]
	pub to_amt: Option<AmountRangeBoundary1>,
	#[serde(rename = "FrToAmt")]
	pub fr_to_amt: Option<FromToAmountRange1>,
	#[serde(rename = "EQAmt")]
	pub eq_amt: Option<f64>,
	#[serde(rename = "NEQAmt")]
	pub neq_amt: Option<f64>,
}


// ImpliedCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImpliedCurrencyAndAmount {
	#[serde(rename = "ImpliedCurrencyAndAmount")]
	pub implied_currency_and_amount: f64,
}


// InterestRecord2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRecord2 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: String,
	#[serde(rename = "Tp")]
	pub tp: Option<InterestType1Choice>,
	#[serde(rename = "Rate")]
	pub rate: Option<Rate4>,
	#[serde(rename = "FrToDt")]
	pub fr_to_dt: Option<DateTimePeriod1>,
	#[serde(rename = "Rsn")]
	pub rsn: Option<String>,
	#[serde(rename = "Tax")]
	pub tax: Option<TaxCharges2>,
}


// InterestType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// InterestType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestType1Code {
	#[serde(rename = "InterestType1Code")]
	pub interest_type1_code: String,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LocalInstrument2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LocalInstrument2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// Max1025Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max1025Text {
	#[serde(rename = "Max1025Text")]
	pub max1025_text: String,
}


// Max105Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max105Text {
	#[serde(rename = "Max105Text")]
	pub max105_text: String,
}


// Max128Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max128Text {
	#[serde(rename = "Max128Text")]
	pub max128_text: String,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max15NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max15NumericText {
	#[serde(rename = "Max15NumericText")]
	pub max15_numeric_text: String,
}


// Max15PlusSignedNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max15PlusSignedNumericText {
	#[serde(rename = "Max15PlusSignedNumericText")]
	pub max15_plus_signed_numeric_text: String,
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


// Max3NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max3NumericText {
	#[serde(rename = "Max3NumericText")]
	pub max3_numeric_text: String,
}


// Max4Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4Text {
	#[serde(rename = "Max4Text")]
	pub max4_text: String,
}


// Max500Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max500Text {
	#[serde(rename = "Max500Text")]
	pub max500_text: String,
}


// Max5NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max5NumericText {
	#[serde(rename = "Max5NumericText")]
	pub max5_numeric_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// MessageIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageIdentification2 {
	#[serde(rename = "MsgNmId")]
	pub msg_nm_id: Option<String>,
	#[serde(rename = "MsgId")]
	pub msg_id: Option<String>,
}


// Min2Max3NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Min2Max3NumericText {
	#[serde(rename = "Min2Max3NumericText")]
	pub min2_max3_numeric_text: String,
}


// Min3Max4NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Min3Max4NumericText {
	#[serde(rename = "Min3Max4NumericText")]
	pub min3_max4_numeric_text: String,
}


// Min8Max28NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Min8Max28NumericText {
	#[serde(rename = "Min8Max28NumericText")]
	pub min8_max28_numeric_text: String,
}


// NameAndAddress16 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress16 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Adr")]
	pub adr: PostalAddress24,
}


// NamePrefix2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NamePrefix2Code {
	#[serde(rename = "NamePrefix2Code")]
	pub name_prefix2_code: String,
}


// NonNegativeDecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NonNegativeDecimalNumber {
	#[serde(rename = "NonNegativeDecimalNumber")]
	pub non_negative_decimal_number: f64,
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
}


// NumberAndSumOfTransactions1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NumberAndSumOfTransactions1 {
	#[serde(rename = "NbOfNtries")]
	pub nb_of_ntries: Option<String>,
	#[serde(rename = "Sum")]
	pub sum: Option<f64>,
}


// NumberAndSumOfTransactions4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NumberAndSumOfTransactions4 {
	#[serde(rename = "NbOfNtries")]
	pub nb_of_ntries: Option<String>,
	#[serde(rename = "Sum")]
	pub sum: Option<f64>,
	#[serde(rename = "TtlNetNtry")]
	pub ttl_net_ntry: Option<AmountAndDirection35>,
}


// OnLineCapability1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OnLineCapability1Code {
	#[serde(rename = "OnLineCapability1Code")]
	pub on_line_capability1_code: String,
}


// OrganisationIdentification29 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification29 {
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<GenericOrganisationIdentification1>>,
}


// OrganisationIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// OriginalAndCurrentQuantities1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OriginalAndCurrentQuantities1 {
	#[serde(rename = "FaceAmt")]
	pub face_amt: f64,
	#[serde(rename = "AmtsdVal")]
	pub amtsd_val: f64,
}


// OriginalBusinessQuery1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OriginalBusinessQuery1 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "MsgNmId")]
	pub msg_nm_id: Option<String>,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: Option<String>,
}


// OtherContact1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherContact1 {
	#[serde(rename = "ChanlTp")]
	pub chanl_tp: String,
	#[serde(rename = "Id")]
	pub id: Option<String>,
}


// OtherIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Sfx")]
	pub sfx: Option<String>,
	#[serde(rename = "Tp")]
	pub tp: IdentificationSource3Choice,
}


// POIComponentType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct POIComponentType1Code {
	#[serde(rename = "POIComponentType1Code")]
	pub poi_component_type1_code: String,
}


// Pagination1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Pagination1 {
	#[serde(rename = "PgNb")]
	pub pg_nb: String,
	#[serde(rename = "LastPgInd")]
	pub last_pg_ind: bool,
}


// Party38Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party38Choice {
	#[serde(rename = "OrgId")]
	pub org_id: Option<OrganisationIdentification29>,
	#[serde(rename = "PrvtId")]
	pub prvt_id: Option<PersonIdentification13>,
}


// Party40Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party40Choice {
	#[serde(rename = "Pty")]
	pub pty: Option<PartyIdentification135>,
	#[serde(rename = "Agt")]
	pub agt: Option<BranchAndFinancialInstitutionIdentification6>,
}


// PartyIdentification135 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification135 {
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress24>,
	#[serde(rename = "Id")]
	pub id: Option<Party38Choice>,
	#[serde(rename = "CtryOfRes")]
	pub ctry_of_res: Option<String>,
	#[serde(rename = "CtctDtls")]
	pub ctct_dtls: Option<Contact4>,
}


// PartyType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyType3Code {
	#[serde(rename = "PartyType3Code")]
	pub party_type3_code: String,
}


// PartyType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyType4Code {
	#[serde(rename = "PartyType4Code")]
	pub party_type4_code: String,
}


// PaymentCard4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentCard4 {
	#[serde(rename = "PlainCardData")]
	pub plain_card_data: Option<PlainCardData1>,
	#[serde(rename = "CardCtryCd")]
	pub card_ctry_cd: Option<String>,
	#[serde(rename = "CardBrnd")]
	pub card_brnd: Option<GenericIdentification1>,
	#[serde(rename = "AddtlCardData")]
	pub addtl_card_data: Option<String>,
}


// PaymentContext3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentContext3 {
	#[serde(rename = "CardPres")]
	pub card_pres: Option<bool>,
	#[serde(rename = "CrdhldrPres")]
	pub crdhldr_pres: Option<bool>,
	#[serde(rename = "OnLineCntxt")]
	pub on_line_cntxt: Option<bool>,
	#[serde(rename = "AttndncCntxt")]
	pub attndnc_cntxt: Option<String>,
	#[serde(rename = "TxEnvt")]
	pub tx_envt: Option<String>,
	#[serde(rename = "TxChanl")]
	pub tx_chanl: Option<String>,
	#[serde(rename = "AttndntMsgCpbl")]
	pub attndnt_msg_cpbl: Option<bool>,
	#[serde(rename = "AttndntLang")]
	pub attndnt_lang: Option<String>,
	#[serde(rename = "CardDataNtryMd")]
	pub card_data_ntry_md: String,
	#[serde(rename = "FllbckInd")]
	pub fllbck_ind: Option<bool>,
	#[serde(rename = "AuthntcnMtd")]
	pub authntcn_mtd: Option<CardholderAuthentication2>,
}


// PaymentReturnReason5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentReturnReason5 {
	#[serde(rename = "OrgnlBkTxCd")]
	pub orgnl_bk_tx_cd: Option<BankTransactionCodeStructure4>,
	#[serde(rename = "Orgtr")]
	pub orgtr: Option<PartyIdentification135>,
	#[serde(rename = "Rsn")]
	pub rsn: Option<ReturnReason5Choice>,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<Vec<String>>,
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// PersonIdentification13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentification13 {
	#[serde(rename = "DtAndPlcOfBirth")]
	pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<GenericPersonIdentification1>>,
}


// PersonIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// PhoneNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PhoneNumber {
	#[serde(rename = "PhoneNumber")]
	pub phone_number: String,
}


// PlainCardData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlainCardData1 {
	#[serde(rename = "PAN")]
	pub pan: String,
	#[serde(rename = "CardSeqNb")]
	pub card_seq_nb: Option<String>,
	#[serde(rename = "FctvDt")]
	pub fctv_dt: Option<String>,
	#[serde(rename = "XpryDt")]
	pub xpry_dt: String,
	#[serde(rename = "SvcCd")]
	pub svc_cd: Option<String>,
	#[serde(rename = "TrckData")]
	pub trck_data: Option<Vec<TrackData1>>,
	#[serde(rename = "CardSctyCd")]
	pub card_scty_cd: Option<CardSecurityInformation1>,
}


// PointOfInteraction1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PointOfInteraction1 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification32,
	#[serde(rename = "SysNm")]
	pub sys_nm: Option<String>,
	#[serde(rename = "GrpId")]
	pub grp_id: Option<String>,
	#[serde(rename = "Cpblties")]
	pub cpblties: Option<PointOfInteractionCapabilities1>,
	#[serde(rename = "Cmpnt")]
	pub cmpnt: Option<Vec<PointOfInteractionComponent1>>,
}


// PointOfInteractionCapabilities1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PointOfInteractionCapabilities1 {
	#[serde(rename = "CardRdngCpblties")]
	pub card_rdng_cpblties: Option<Vec<String>>,
	#[serde(rename = "CrdhldrVrfctnCpblties")]
	pub crdhldr_vrfctn_cpblties: Option<Vec<String>>,
	#[serde(rename = "OnLineCpblties")]
	pub on_line_cpblties: Option<String>,
	#[serde(rename = "DispCpblties")]
	pub disp_cpblties: Option<Vec<DisplayCapabilities1>>,
	#[serde(rename = "PrtLineWidth")]
	pub prt_line_width: Option<String>,
}


// PointOfInteractionComponent1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PointOfInteractionComponent1 {
	#[serde(rename = "POICmpntTp")]
	pub poi_cmpnt_tp: String,
	#[serde(rename = "ManfctrId")]
	pub manfctr_id: Option<String>,
	#[serde(rename = "Mdl")]
	pub mdl: Option<String>,
	#[serde(rename = "VrsnNb")]
	pub vrsn_nb: Option<String>,
	#[serde(rename = "SrlNb")]
	pub srl_nb: Option<String>,
	#[serde(rename = "ApprvlNb")]
	pub apprvl_nb: Option<Vec<String>>,
}


// PostalAddress24 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress24 {
	#[serde(rename = "AdrTp")]
	pub adr_tp: Option<AddressType3Choice>,
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


// PreferredContactMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PreferredContactMethod1Code {
	#[serde(rename = "PreferredContactMethod1Code")]
	pub preferred_contact_method1_code: String,
}


// Price7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Price7 {
	#[serde(rename = "Tp")]
	pub tp: YieldedOrValueType1Choice,
	#[serde(rename = "Val")]
	pub val: PriceRateOrAmount3Choice,
}


// PriceRateOrAmount3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceRateOrAmount3Choice {
	#[serde(rename = "Rate")]
	pub rate: Option<f64>,
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
}


// PriceValueType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceValueType1Code {
	#[serde(rename = "PriceValueType1Code")]
	pub price_value_type1_code: String,
}


// Product2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Product2 {
	#[serde(rename = "PdctCd")]
	pub pdct_cd: String,
	#[serde(rename = "UnitOfMeasr")]
	pub unit_of_measr: Option<String>,
	#[serde(rename = "PdctQty")]
	pub pdct_qty: Option<f64>,
	#[serde(rename = "UnitPric")]
	pub unit_pric: Option<f64>,
	#[serde(rename = "PdctAmt")]
	pub pdct_amt: Option<f64>,
	#[serde(rename = "TaxTp")]
	pub tax_tp: Option<String>,
	#[serde(rename = "AddtlPdctInf")]
	pub addtl_pdct_inf: Option<String>,
}


// ProprietaryAgent4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryAgent4 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "Agt")]
	pub agt: BranchAndFinancialInstitutionIdentification6,
}


// ProprietaryBankTransactionCodeStructure1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryBankTransactionCodeStructure1 {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// ProprietaryDate3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryDate3 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "Dt")]
	pub dt: DateAndDateTime2Choice,
}


// ProprietaryParty5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryParty5 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "Pty")]
	pub pty: Party40Choice,
}


// ProprietaryPrice2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryPrice2 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "Pric")]
	pub pric: ActiveOrHistoricCurrencyAndAmount,
}


// ProprietaryQuantity1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryQuantity1 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "Qty")]
	pub qty: String,
}


// ProprietaryReference1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryReference1 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "Ref")]
	pub ref_attr: String,
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


// Purpose2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Purpose2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// Rate4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Rate4 {
	#[serde(rename = "Tp")]
	pub tp: RateType4Choice,
	#[serde(rename = "VldtyRg")]
	pub vldty_rg: Option<ActiveOrHistoricCurrencyAndAmountRange2>,
}


// RateType4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RateType4Choice {
	#[serde(rename = "Pctg")]
	pub pctg: Option<f64>,
	#[serde(rename = "Othr")]
	pub othr: Option<String>,
}


// ReferredDocumentInformation7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReferredDocumentInformation7 {
	#[serde(rename = "Tp")]
	pub tp: Option<ReferredDocumentType4>,
	#[serde(rename = "Nb")]
	pub nb: Option<String>,
	#[serde(rename = "RltdDt")]
	pub rltd_dt: Option<String>,
	#[serde(rename = "LineDtls")]
	pub line_dtls: Option<Vec<DocumentLineInformation1>>,
}


// ReferredDocumentType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReferredDocumentType3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ReferredDocumentType4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReferredDocumentType4 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: ReferredDocumentType3Choice,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// RemittanceAmount2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceAmount2 {
	#[serde(rename = "DuePyblAmt")]
	pub due_pybl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "DscntApldAmt")]
	pub dscnt_apld_amt: Option<Vec<DiscountAmountAndType1>>,
	#[serde(rename = "CdtNoteAmt")]
	pub cdt_note_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TaxAmt")]
	pub tax_amt: Option<Vec<TaxAmountAndType1>>,
	#[serde(rename = "AdjstmntAmtAndRsn")]
	pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment1>>,
	#[serde(rename = "RmtdAmt")]
	pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// RemittanceAmount3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceAmount3 {
	#[serde(rename = "DuePyblAmt")]
	pub due_pybl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "DscntApldAmt")]
	pub dscnt_apld_amt: Option<Vec<DiscountAmountAndType1>>,
	#[serde(rename = "CdtNoteAmt")]
	pub cdt_note_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TaxAmt")]
	pub tax_amt: Option<Vec<TaxAmountAndType1>>,
	#[serde(rename = "AdjstmntAmtAndRsn")]
	pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment1>>,
	#[serde(rename = "RmtdAmt")]
	pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// RemittanceInformation16 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceInformation16 {
	#[serde(rename = "Ustrd")]
	pub ustrd: Option<Vec<String>>,
	#[serde(rename = "Strd")]
	pub strd: Option<Vec<StructuredRemittanceInformation16>>,
}


// RemittanceLocation7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceLocation7 {
	#[serde(rename = "RmtId")]
	pub rmt_id: Option<String>,
	#[serde(rename = "RmtLctnDtls")]
	pub rmt_lctn_dtls: Option<Vec<RemittanceLocationData1>>,
}


// RemittanceLocationData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceLocationData1 {
	#[serde(rename = "Mtd")]
	pub mtd: String,
	#[serde(rename = "ElctrncAdr")]
	pub elctrnc_adr: Option<String>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<NameAndAddress16>,
}


// RemittanceLocationMethod2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceLocationMethod2Code {
	#[serde(rename = "RemittanceLocationMethod2Code")]
	pub remittance_location_method2_code: String,
}


// ReportEntry10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportEntry10 {
	#[serde(rename = "NtryRef")]
	pub ntry_ref: Option<String>,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: String,
	#[serde(rename = "RvslInd")]
	pub rvsl_ind: Option<bool>,
	#[serde(rename = "Sts")]
	pub sts: EntryStatus1Choice,
	#[serde(rename = "BookgDt")]
	pub bookg_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "ValDt")]
	pub val_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "AcctSvcrRef")]
	pub acct_svcr_ref: Option<String>,
	#[serde(rename = "Avlbty")]
	pub avlbty: Option<Vec<CashAvailability1>>,
	#[serde(rename = "BkTxCd")]
	pub bk_tx_cd: BankTransactionCodeStructure4,
	#[serde(rename = "ComssnWvrInd")]
	pub comssn_wvr_ind: Option<bool>,
	#[serde(rename = "AddtlInfInd")]
	pub addtl_inf_ind: Option<MessageIdentification2>,
	#[serde(rename = "AmtDtls")]
	pub amt_dtls: Option<AmountAndCurrencyExchange3>,
	#[serde(rename = "Chrgs")]
	pub chrgs: Option<Charges6>,
	#[serde(rename = "TechInptChanl")]
	pub tech_inpt_chanl: Option<TechnicalInputChannel1Choice>,
	#[serde(rename = "Intrst")]
	pub intrst: Option<TransactionInterest4>,
	#[serde(rename = "CardTx")]
	pub card_tx: Option<CardEntry4>,
	#[serde(rename = "NtryDtls")]
	pub ntry_dtls: Option<Vec<EntryDetails9>>,
	#[serde(rename = "AddtlNtryInf")]
	pub addtl_ntry_inf: Option<String>,
}


// ReportingSource1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportingSource1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ReturnReason5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReturnReason5Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// SecuritiesAccount19 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesAccount19 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Tp")]
	pub tp: Option<GenericIdentification30>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
}


// SecurityIdentification19 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification19 {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[serde(rename = "OthrId")]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// SequenceRange1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SequenceRange1 {
	#[serde(rename = "FrSeq")]
	pub fr_seq: String,
	#[serde(rename = "ToSeq")]
	pub to_seq: String,
}


// SequenceRange1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SequenceRange1Choice {
	#[serde(rename = "FrSeq")]
	pub fr_seq: Option<String>,
	#[serde(rename = "ToSeq")]
	pub to_seq: Option<String>,
	#[serde(rename = "FrToSeq")]
	pub fr_to_seq: Option<Vec<SequenceRange1>>,
	#[serde(rename = "EQSeq")]
	pub eq_seq: Option<Vec<String>>,
	#[serde(rename = "NEQSeq")]
	pub neq_seq: Option<Vec<String>>,
}


// StructuredRemittanceInformation16 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StructuredRemittanceInformation16 {
	#[serde(rename = "RfrdDocInf")]
	pub rfrd_doc_inf: Option<Vec<ReferredDocumentInformation7>>,
	#[serde(rename = "RfrdDocAmt")]
	pub rfrd_doc_amt: Option<RemittanceAmount2>,
	#[serde(rename = "CdtrRefInf")]
	pub cdtr_ref_inf: Option<CreditorReferenceInformation2>,
	#[serde(rename = "Invcr")]
	pub invcr: Option<PartyIdentification135>,
	#[serde(rename = "Invcee")]
	pub invcee: Option<PartyIdentification135>,
	#[serde(rename = "TaxRmt")]
	pub tax_rmt: Option<TaxInformation7>,
	#[serde(rename = "GrnshmtRmt")]
	pub grnshmt_rmt: Option<Garnishment3>,
	#[serde(rename = "AddtlRmtInf")]
	pub addtl_rmt_inf: Option<Vec<String>>,
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Option<String>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}


// TaxAmount2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxAmount2 {
	#[serde(rename = "Rate")]
	pub rate: Option<f64>,
	#[serde(rename = "TaxblBaseAmt")]
	pub taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TtlAmt")]
	pub ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Dtls")]
	pub dtls: Option<Vec<TaxRecordDetails2>>,
}


// TaxAmountAndType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxAmountAndType1 {
	#[serde(rename = "Tp")]
	pub tp: Option<TaxAmountType1Choice>,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}


// TaxAmountType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxAmountType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// TaxAuthorisation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxAuthorisation1 {
	#[serde(rename = "Titl")]
	pub titl: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
}


// TaxCharges2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxCharges2 {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "Rate")]
	pub rate: Option<f64>,
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// TaxInformation7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxInformation7 {
	#[serde(rename = "Cdtr")]
	pub cdtr: Option<TaxParty1>,
	#[serde(rename = "Dbtr")]
	pub dbtr: Option<TaxParty2>,
	#[serde(rename = "UltmtDbtr")]
	pub ultmt_dbtr: Option<TaxParty2>,
	#[serde(rename = "AdmstnZone")]
	pub admstn_zone: Option<String>,
	#[serde(rename = "RefNb")]
	pub ref_nb: Option<String>,
	#[serde(rename = "Mtd")]
	pub mtd: Option<String>,
	#[serde(rename = "TtlTaxblBaseAmt")]
	pub ttl_taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TtlTaxAmt")]
	pub ttl_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "SeqNb")]
	pub seq_nb: Option<f64>,
	#[serde(rename = "Rcrd")]
	pub rcrd: Option<Vec<TaxRecord2>>,
}


// TaxInformation8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxInformation8 {
	#[serde(rename = "Cdtr")]
	pub cdtr: Option<TaxParty1>,
	#[serde(rename = "Dbtr")]
	pub dbtr: Option<TaxParty2>,
	#[serde(rename = "AdmstnZone")]
	pub admstn_zone: Option<String>,
	#[serde(rename = "RefNb")]
	pub ref_nb: Option<String>,
	#[serde(rename = "Mtd")]
	pub mtd: Option<String>,
	#[serde(rename = "TtlTaxblBaseAmt")]
	pub ttl_taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TtlTaxAmt")]
	pub ttl_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "SeqNb")]
	pub seq_nb: Option<f64>,
	#[serde(rename = "Rcrd")]
	pub rcrd: Option<Vec<TaxRecord2>>,
}


// TaxParty1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxParty1 {
	#[serde(rename = "TaxId")]
	pub tax_id: Option<String>,
	#[serde(rename = "RegnId")]
	pub regn_id: Option<String>,
	#[serde(rename = "TaxTp")]
	pub tax_tp: Option<String>,
}


// TaxParty2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxParty2 {
	#[serde(rename = "TaxId")]
	pub tax_id: Option<String>,
	#[serde(rename = "RegnId")]
	pub regn_id: Option<String>,
	#[serde(rename = "TaxTp")]
	pub tax_tp: Option<String>,
	#[serde(rename = "Authstn")]
	pub authstn: Option<TaxAuthorisation1>,
}


// TaxPeriod2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxPeriod2 {
	#[serde(rename = "Yr")]
	pub yr: Option<String>,
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "FrToDt")]
	pub fr_to_dt: Option<DatePeriod2>,
}


// TaxRecord2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxRecord2 {
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "Ctgy")]
	pub ctgy: Option<String>,
	#[serde(rename = "CtgyDtls")]
	pub ctgy_dtls: Option<String>,
	#[serde(rename = "DbtrSts")]
	pub dbtr_sts: Option<String>,
	#[serde(rename = "CertId")]
	pub cert_id: Option<String>,
	#[serde(rename = "FrmsCd")]
	pub frms_cd: Option<String>,
	#[serde(rename = "Prd")]
	pub prd: Option<TaxPeriod2>,
	#[serde(rename = "TaxAmt")]
	pub tax_amt: Option<TaxAmount2>,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<String>,
}


// TaxRecordDetails2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxRecordDetails2 {
	#[serde(rename = "Prd")]
	pub prd: Option<TaxPeriod2>,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}


// TaxRecordPeriod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxRecordPeriod1Code {
	#[serde(rename = "TaxRecordPeriod1Code")]
	pub tax_record_period1_code: String,
}


// TechnicalInputChannel1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TechnicalInputChannel1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// TotalTransactions6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TotalTransactions6 {
	#[serde(rename = "TtlNtries")]
	pub ttl_ntries: Option<NumberAndSumOfTransactions4>,
	#[serde(rename = "TtlCdtNtries")]
	pub ttl_cdt_ntries: Option<NumberAndSumOfTransactions1>,
	#[serde(rename = "TtlDbtNtries")]
	pub ttl_dbt_ntries: Option<NumberAndSumOfTransactions1>,
	#[serde(rename = "TtlNtriesPerBkTxCd")]
	pub ttl_ntries_per_bk_tx_cd: Option<Vec<TotalsPerBankTransactionCode5>>,
}


// TotalsPerBankTransactionCode5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TotalsPerBankTransactionCode5 {
	#[serde(rename = "NbOfNtries")]
	pub nb_of_ntries: Option<String>,
	#[serde(rename = "Sum")]
	pub sum: Option<f64>,
	#[serde(rename = "TtlNetNtry")]
	pub ttl_net_ntry: Option<AmountAndDirection35>,
	#[serde(rename = "CdtNtries")]
	pub cdt_ntries: Option<NumberAndSumOfTransactions1>,
	#[serde(rename = "DbtNtries")]
	pub dbt_ntries: Option<NumberAndSumOfTransactions1>,
	#[serde(rename = "FcstInd")]
	pub fcst_ind: Option<bool>,
	#[serde(rename = "BkTxCd")]
	pub bk_tx_cd: BankTransactionCodeStructure4,
	#[serde(rename = "Avlbty")]
	pub avlbty: Option<Vec<CashAvailability1>>,
	#[serde(rename = "Dt")]
	pub dt: Option<DateAndDateTime2Choice>,
}


// TrackData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrackData1 {
	#[serde(rename = "TrckNb")]
	pub trck_nb: Option<String>,
	#[serde(rename = "TrckVal")]
	pub trck_val: String,
}


// TransactionAgents5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionAgents5 {
	#[serde(rename = "InstgAgt")]
	pub instg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "InstdAgt")]
	pub instd_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "DbtrAgt")]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "CdtrAgt")]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "IntrmyAgt1")]
	pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "IntrmyAgt2")]
	pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "IntrmyAgt3")]
	pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "RcvgAgt")]
	pub rcvg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "DlvrgAgt")]
	pub dlvrg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "IssgAgt")]
	pub issg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "SttlmPlc")]
	pub sttlm_plc: Option<BranchAndFinancialInstitutionIdentification6>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<Vec<ProprietaryAgent4>>,
}


// TransactionChannel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionChannel1Code {
	#[serde(rename = "TransactionChannel1Code")]
	pub transaction_channel1_code: String,
}


// TransactionDates3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionDates3 {
	#[serde(rename = "AccptncDtTm")]
	pub accptnc_dt_tm: Option<String>,
	#[serde(rename = "TradActvtyCtrctlSttlmDt")]
	pub trad_actvty_ctrctl_sttlm_dt: Option<String>,
	#[serde(rename = "TradDt")]
	pub trad_dt: Option<String>,
	#[serde(rename = "IntrBkSttlmDt")]
	pub intr_bk_sttlm_dt: Option<String>,
	#[serde(rename = "StartDt")]
	pub start_dt: Option<String>,
	#[serde(rename = "EndDt")]
	pub end_dt: Option<String>,
	#[serde(rename = "TxDtTm")]
	pub tx_dt_tm: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<Vec<ProprietaryDate3>>,
}


// TransactionEnvironment1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionEnvironment1Code {
	#[serde(rename = "TransactionEnvironment1Code")]
	pub transaction_environment1_code: String,
}


// TransactionIdentifier1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionIdentifier1 {
	#[serde(rename = "TxDtTm")]
	pub tx_dt_tm: String,
	#[serde(rename = "TxRef")]
	pub tx_ref: String,
}


// TransactionInterest4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionInterest4 {
	#[serde(rename = "TtlIntrstAndTaxAmt")]
	pub ttl_intrst_and_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Rcrd")]
	pub rcrd: Option<Vec<InterestRecord2>>,
}


// TransactionParties6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionParties6 {
	#[serde(rename = "InitgPty")]
	pub initg_pty: Option<Party40Choice>,
	#[serde(rename = "Dbtr")]
	pub dbtr: Option<Party40Choice>,
	#[serde(rename = "DbtrAcct")]
	pub dbtr_acct: Option<CashAccount38>,
	#[serde(rename = "UltmtDbtr")]
	pub ultmt_dbtr: Option<Party40Choice>,
	#[serde(rename = "Cdtr")]
	pub cdtr: Option<Party40Choice>,
	#[serde(rename = "CdtrAcct")]
	pub cdtr_acct: Option<CashAccount38>,
	#[serde(rename = "UltmtCdtr")]
	pub ultmt_cdtr: Option<Party40Choice>,
	#[serde(rename = "TradgPty")]
	pub tradg_pty: Option<Party40Choice>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<Vec<ProprietaryParty5>>,
}


// TransactionPrice4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionPrice4Choice {
	#[serde(rename = "DealPric")]
	pub deal_pric: Option<Price7>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<Vec<ProprietaryPrice2>>,
}


// TransactionQuantities3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionQuantities3Choice {
	#[serde(rename = "Qty")]
	pub qty: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "OrgnlAndCurFaceAmt")]
	pub orgnl_and_cur_face_amt: Option<OriginalAndCurrentQuantities1>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<ProprietaryQuantity1>,
}


// TransactionReferences6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionReferences6 {
	#[serde(rename = "MsgId")]
	pub msg_id: Option<String>,
	#[serde(rename = "AcctSvcrRef")]
	pub acct_svcr_ref: Option<String>,
	#[serde(rename = "PmtInfId")]
	pub pmt_inf_id: Option<String>,
	#[serde(rename = "InstrId")]
	pub instr_id: Option<String>,
	#[serde(rename = "EndToEndId")]
	pub end_to_end_id: Option<String>,
	#[serde(rename = "UETR")]
	pub uetr: Option<String>,
	#[serde(rename = "TxId")]
	pub tx_id: Option<String>,
	#[serde(rename = "MndtId")]
	pub mndt_id: Option<String>,
	#[serde(rename = "ChqNb")]
	pub chq_nb: Option<String>,
	#[serde(rename = "ClrSysRef")]
	pub clr_sys_ref: Option<String>,
	#[serde(rename = "AcctOwnrTxId")]
	pub acct_ownr_tx_id: Option<String>,
	#[serde(rename = "AcctSvcrTxId")]
	pub acct_svcr_tx_id: Option<String>,
	#[serde(rename = "MktInfrstrctrTxId")]
	pub mkt_infrstrctr_tx_id: Option<String>,
	#[serde(rename = "PrcgId")]
	pub prcg_id: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<Vec<ProprietaryReference1>>,
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}


// UUIDv4Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UUIDv4Identifier {
	#[serde(rename = "UUIDv4Identifier")]
	pub uui_dv4_identifier: String,
}


// UnitOfMeasure1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitOfMeasure1Code {
	#[serde(rename = "UnitOfMeasure1Code")]
	pub unit_of_measure1_code: String,
}


// UserInterface2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UserInterface2Code {
	#[serde(rename = "UserInterface2Code")]
	pub user_interface2_code: String,
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "YesNoIndicator")]
	pub yes_no_indicator: bool,
}


// YieldedOrValueType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YieldedOrValueType1Choice {
	#[serde(rename = "Yldd")]
	pub yldd: Option<bool>,
	#[serde(rename = "ValTp")]
	pub val_tp: Option<String>,
}
