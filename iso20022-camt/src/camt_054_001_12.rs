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


// AccountInterest4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountInterest4 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<InterestType1Choice>,
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<Vec<Rate4>>,
	#[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
	pub fr_to_dt: Option<DateTimePeriod1>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Max35Text>,
	#[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
	pub tax: Option<TaxCharges2>,
}


// AccountNotification22 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountNotification22 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "NtfctnPgntn", skip_serializing_if = "Option::is_none")]
	pub ntfctn_pgntn: Option<Pagination1>,
	#[serde(rename = "ElctrncSeqNb", skip_serializing_if = "Option::is_none")]
	pub elctrnc_seq_nb: Option<f64>,
	#[serde(rename = "RptgSeq", skip_serializing_if = "Option::is_none")]
	pub rptg_seq: Option<SequenceRange1Choice>,
	#[serde(rename = "LglSeqNb", skip_serializing_if = "Option::is_none")]
	pub lgl_seq_nb: Option<f64>,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
	#[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
	pub fr_to_dt: Option<DateTimePeriod1>,
	#[serde(rename = "CpyDplctInd", skip_serializing_if = "Option::is_none")]
	pub cpy_dplct_ind: Option<CopyDuplicate1Code>,
	#[serde(rename = "RptgSrc", skip_serializing_if = "Option::is_none")]
	pub rptg_src: Option<ReportingSource1Choice>,
	#[serde(rename = "Acct")]
	pub acct: CashAccount43,
	#[serde(rename = "RltdAcct", skip_serializing_if = "Option::is_none")]
	pub rltd_acct: Option<CashAccount40>,
	#[serde(rename = "Intrst", skip_serializing_if = "Option::is_none")]
	pub intrst: Option<Vec<AccountInterest4>>,
	#[serde(rename = "TxsSummry", skip_serializing_if = "Option::is_none")]
	pub txs_summry: Option<TotalTransactions6>,
	#[serde(rename = "Ntry", skip_serializing_if = "Option::is_none")]
	pub ntry: Option<Vec<ReportEntry14>>,
	#[serde(rename = "AddtlNtfctnInf", skip_serializing_if = "Option::is_none")]
	pub addtl_ntfctn_inf: Option<Max500Text>,
}


// AccountSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalAccountIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
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
	#[serde(rename = "$value")]
	pub active_currency_code: String,
}


// ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType {
	#[serde(rename = "$value")]
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


// ActiveOrHistoricCurrencyAndAmountRange2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmountRange2 {
	#[serde(rename = "Amt")]
	pub amt: ImpliedCurrencyAmountRange1Choice,
	#[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
	#[serde(rename = "Ccy")]
	pub ccy: ActiveOrHistoricCurrencyCode,
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


// AmountAndCurrencyExchange4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndCurrencyExchange4 {
	#[serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none")]
	pub instd_amt: Option<AmountAndCurrencyExchangeDetails5>,
	#[serde(rename = "TxAmt", skip_serializing_if = "Option::is_none")]
	pub tx_amt: Option<AmountAndCurrencyExchangeDetails5>,
	#[serde(rename = "CntrValAmt", skip_serializing_if = "Option::is_none")]
	pub cntr_val_amt: Option<AmountAndCurrencyExchangeDetails5>,
	#[serde(rename = "AnncdPstngAmt", skip_serializing_if = "Option::is_none")]
	pub anncd_pstng_amt: Option<AmountAndCurrencyExchangeDetails5>,
	#[serde(rename = "PrtryAmt", skip_serializing_if = "Option::is_none")]
	pub prtry_amt: Option<Vec<AmountAndCurrencyExchangeDetails6>>,
}


// AmountAndCurrencyExchangeDetails5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndCurrencyExchangeDetails5 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CcyXchg", skip_serializing_if = "Option::is_none")]
	pub ccy_xchg: Option<CurrencyExchange24>,
}


// AmountAndCurrencyExchangeDetails6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndCurrencyExchangeDetails6 {
	#[serde(rename = "Tp")]
	pub tp: Max35Text,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CcyXchg", skip_serializing_if = "Option::is_none")]
	pub ccy_xchg: Option<CurrencyExchange24>,
}


// AmountAndDirection35 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection35 {
	#[serde(rename = "Amt")]
	pub amt: f64,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: CreditDebitCode,
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
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
}


// AttendanceContext1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AttendanceContext1Code {
	#[default]
	#[serde(rename = "ATTD")]
	CodeATTD,
	#[serde(rename = "SATT")]
	CodeSATT,
	#[serde(rename = "UATT")]
	CodeUATT,

}


// AuthenticationEntity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AuthenticationEntity1Code {
	#[default]
	#[serde(rename = "ICCD")]
	CodeICCD,
	#[serde(rename = "AGNT")]
	CodeAGNT,
	#[serde(rename = "MERC")]
	CodeMERC,

}


// AuthenticationMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AuthenticationMethod1Code {
	#[default]
	#[serde(rename = "UKNW")]
	CodeUKNW,
	#[serde(rename = "BYPS")]
	CodeBYPS,
	#[serde(rename = "NPIN")]
	CodeNPIN,
	#[serde(rename = "FPIN")]
	CodeFPIN,
	#[serde(rename = "CPSG")]
	CodeCPSG,
	#[serde(rename = "PPSG")]
	CodePPSG,
	#[serde(rename = "MANU")]
	CodeMANU,
	#[serde(rename = "MERC")]
	CodeMERC,
	#[serde(rename = "SCRT")]
	CodeSCRT,
	#[serde(rename = "SNCT")]
	CodeSNCT,
	#[serde(rename = "SCNL")]
	CodeSCNL,

}


// BICFIDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BICFIDec2014Identifier {
	#[serde(rename = "$value")]
	pub bicfi_dec2014_identifier: String,
}


// BankToCustomerDebitCreditNotificationV12 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BankToCustomerDebitCreditNotificationV12 {
	#[serde(rename = "GrpHdr")]
	pub grp_hdr: GroupHeader116,
	#[serde(rename = "Ntfctn")]
	pub ntfctn: Vec<AccountNotification22>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
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


// BatchInformation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BatchInformation2 {
	#[serde(rename = "MsgId", skip_serializing_if = "Option::is_none")]
	pub msg_id: Option<Max35Text>,
	#[serde(rename = "PmtInfId", skip_serializing_if = "Option::is_none")]
	pub pmt_inf_id: Option<Max35Text>,
	#[serde(rename = "NbOfTxs", skip_serializing_if = "Option::is_none")]
	pub nb_of_txs: Option<Max15NumericText>,
	#[serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none")]
	pub ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
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


// CSCManagement1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CSCManagement1Code {
	#[default]
	#[serde(rename = "PRST")]
	CodePRST,
	#[serde(rename = "BYPS")]
	CodeBYPS,
	#[serde(rename = "UNRD")]
	CodeUNRD,
	#[serde(rename = "NCSC")]
	CodeNCSC,

}


// CardAggregated2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardAggregated2 {
	#[serde(rename = "AddtlSvc", skip_serializing_if = "Option::is_none")]
	pub addtl_svc: Option<CardPaymentServiceType2Code>,
	#[serde(rename = "TxCtgy", skip_serializing_if = "Option::is_none")]
	pub tx_ctgy: Option<ExternalCardTransactionCategory1Code>,
	#[serde(rename = "SaleRcncltnId", skip_serializing_if = "Option::is_none")]
	pub sale_rcncltn_id: Option<Max35Text>,
	#[serde(rename = "SeqNbRg", skip_serializing_if = "Option::is_none")]
	pub seq_nb_rg: Option<CardSequenceNumberRange1>,
	#[serde(rename = "TxDtRg", skip_serializing_if = "Option::is_none")]
	pub tx_dt_rg: Option<DateOrDateTimePeriod1Choice>,
}


// CardDataReading1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CardDataReading1Code {
	#[default]
	#[serde(rename = "TAGC")]
	CodeTAGC,
	#[serde(rename = "PHYS")]
	CodePHYS,
	#[serde(rename = "BRCD")]
	CodeBRCD,
	#[serde(rename = "MGST")]
	CodeMGST,
	#[serde(rename = "CICC")]
	CodeCICC,
	#[serde(rename = "DFLE")]
	CodeDFLE,
	#[serde(rename = "CTLS")]
	CodeCTLS,
	#[serde(rename = "ECTL")]
	CodeECTL,

}


// CardEntry5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardEntry5 {
	#[serde(rename = "Card", skip_serializing_if = "Option::is_none")]
	pub card: Option<PaymentCard4>,
	#[serde(rename = "POI", skip_serializing_if = "Option::is_none")]
	pub poi: Option<PointOfInteraction1>,
	#[serde(rename = "AggtdNtry", skip_serializing_if = "Option::is_none")]
	pub aggtd_ntry: Option<CardAggregated2>,
	#[serde(rename = "PrePdAcct", skip_serializing_if = "Option::is_none")]
	pub pre_pd_acct: Option<CashAccount40>,
}


// CardIndividualTransaction2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardIndividualTransaction2 {
	#[serde(rename = "ICCRltdData", skip_serializing_if = "Option::is_none")]
	pub icc_rltd_data: Option<Max1025Text>,
	#[serde(rename = "PmtCntxt", skip_serializing_if = "Option::is_none")]
	pub pmt_cntxt: Option<PaymentContext3>,
	#[serde(rename = "AddtlSvc", skip_serializing_if = "Option::is_none")]
	pub addtl_svc: Option<CardPaymentServiceType2Code>,
	#[serde(rename = "TxCtgy", skip_serializing_if = "Option::is_none")]
	pub tx_ctgy: Option<ExternalCardTransactionCategory1Code>,
	#[serde(rename = "SaleRcncltnId", skip_serializing_if = "Option::is_none")]
	pub sale_rcncltn_id: Option<Max35Text>,
	#[serde(rename = "SaleRefNb", skip_serializing_if = "Option::is_none")]
	pub sale_ref_nb: Option<Max35Text>,
	#[serde(rename = "RePresntmntRsn", skip_serializing_if = "Option::is_none")]
	pub re_presntmnt_rsn: Option<ExternalRePresentmentReason1Code>,
	#[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
	pub seq_nb: Option<Max35Text>,
	#[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
	pub tx_id: Option<TransactionIdentifier1>,
	#[serde(rename = "Pdct", skip_serializing_if = "Option::is_none")]
	pub pdct: Option<Product2>,
	#[serde(rename = "VldtnDt", skip_serializing_if = "Option::is_none")]
	pub vldtn_dt: Option<String>,
	#[serde(rename = "VldtnSeqNb", skip_serializing_if = "Option::is_none")]
	pub vldtn_seq_nb: Option<Max35Text>,
}


// CardPaymentServiceType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CardPaymentServiceType2Code {
	#[default]
	#[serde(rename = "AGGR")]
	CodeAGGR,
	#[serde(rename = "DCCV")]
	CodeDCCV,
	#[serde(rename = "GRTT")]
	CodeGRTT,
	#[serde(rename = "INSP")]
	CodeINSP,
	#[serde(rename = "LOYT")]
	CodeLOYT,
	#[serde(rename = "NRES")]
	CodeNRES,
	#[serde(rename = "PUCO")]
	CodePUCO,
	#[serde(rename = "RECP")]
	CodeRECP,
	#[serde(rename = "SOAF")]
	CodeSOAF,
	#[serde(rename = "UNAF")]
	CodeUNAF,
	#[serde(rename = "VCAU")]
	CodeVCAU,

}


// CardSecurityInformation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardSecurityInformation1 {
	#[serde(rename = "CSCMgmt")]
	pub csc_mgmt: CSCManagement1Code,
	#[serde(rename = "CSCVal", skip_serializing_if = "Option::is_none")]
	pub csc_val: Option<Min3Max4NumericText>,
}


// CardSequenceNumberRange1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardSequenceNumberRange1 {
	#[serde(rename = "FrstTx", skip_serializing_if = "Option::is_none")]
	pub frst_tx: Option<Max35Text>,
	#[serde(rename = "LastTx", skip_serializing_if = "Option::is_none")]
	pub last_tx: Option<Max35Text>,
}


// CardTransaction18 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardTransaction18 {
	#[serde(rename = "Card", skip_serializing_if = "Option::is_none")]
	pub card: Option<PaymentCard4>,
	#[serde(rename = "POI", skip_serializing_if = "Option::is_none")]
	pub poi: Option<PointOfInteraction1>,
	#[serde(rename = "Tx", skip_serializing_if = "Option::is_none")]
	pub tx: Option<CardTransaction3Choice>,
	#[serde(rename = "PrePdAcct", skip_serializing_if = "Option::is_none")]
	pub pre_pd_acct: Option<CashAccount40>,
}


// CardTransaction3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardTransaction3Choice {
	#[serde(rename = "Aggtd", skip_serializing_if = "Option::is_none")]
	pub aggtd: Option<CardAggregated2>,
	#[serde(rename = "Indv", skip_serializing_if = "Option::is_none")]
	pub indv: Option<CardIndividualTransaction2>,
}


// CardholderAuthentication2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CardholderAuthentication2 {
	#[serde(rename = "AuthntcnMtd")]
	pub authntcn_mtd: AuthenticationMethod1Code,
	#[serde(rename = "AuthntcnNtty")]
	pub authntcn_ntty: AuthenticationEntity1Code,
}


// CardholderVerificationCapability1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CardholderVerificationCapability1Code {
	#[default]
	#[serde(rename = "MNSG")]
	CodeMNSG,
	#[serde(rename = "NPIN")]
	CodeNPIN,
	#[serde(rename = "FCPN")]
	CodeFCPN,
	#[serde(rename = "FEPN")]
	CodeFEPN,
	#[serde(rename = "FDSG")]
	CodeFDSG,
	#[serde(rename = "FBIO")]
	CodeFBIO,
	#[serde(rename = "MNVR")]
	CodeMNVR,
	#[serde(rename = "FBIG")]
	CodeFBIG,
	#[serde(rename = "APKI")]
	CodeAPKI,
	#[serde(rename = "PKIS")]
	CodePKIS,
	#[serde(rename = "CHDT")]
	CodeCHDT,
	#[serde(rename = "SCEC")]
	CodeSCEC,

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


// CashAccount43 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccount43 {
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
	#[serde(rename = "Ownr", skip_serializing_if = "Option::is_none")]
	pub ownr: Option<PartyIdentification272>,
	#[serde(rename = "Svcr", skip_serializing_if = "Option::is_none")]
	pub svcr: Option<BranchAndFinancialInstitutionIdentification8>,
}


// CashAccountType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalCashAccountType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// CashAvailability1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAvailability1 {
	#[serde(rename = "Dt")]
	pub dt: CashAvailabilityDate1Choice,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: CreditDebitCode,
}


// CashAvailabilityDate1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAvailabilityDate1Choice {
	#[serde(rename = "NbOfDays", skip_serializing_if = "Option::is_none")]
	pub nb_of_days: Option<Max15PlusSignedNumericText>,
	#[serde(rename = "ActlDt", skip_serializing_if = "Option::is_none")]
	pub actl_dt: Option<String>,
}


// CashDeposit1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashDeposit1 {
	#[serde(rename = "NoteDnmtn")]
	pub note_dnmtn: ActiveCurrencyAndAmount,
	#[serde(rename = "NbOfNotes")]
	pub nb_of_notes: Max15NumericText,
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
}


// CategoryPurpose1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CategoryPurpose1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalCategoryPurpose1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// ChargeBearerType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ChargeBearerType1Code {
	#[default]
	#[serde(rename = "DEBT")]
	CodeDEBT,
	#[serde(rename = "CRED")]
	CodeCRED,
	#[serde(rename = "SHAR")]
	CodeSHAR,
	#[serde(rename = "SLEV")]
	CodeSLEV,

}


// ChargeIncludedIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ChargeIncludedIndicator {
	#[serde(rename = "$value")]
	pub charge_included_indicator: bool,
}


// ChargeType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ChargeType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalChargeType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification3>,
}


// Charges15 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Charges15 {
	#[serde(rename = "TtlChrgsAndTaxAmt", skip_serializing_if = "Option::is_none")]
	pub ttl_chrgs_and_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Rcrd", skip_serializing_if = "Option::is_none")]
	pub rcrd: Option<Vec<ChargesRecord8>>,
}


// ChargesRecord8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ChargesRecord8 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
	#[serde(rename = "ChrgInclInd", skip_serializing_if = "Option::is_none")]
	pub chrg_incl_ind: Option<bool>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<ChargeType3Choice>,
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "Br", skip_serializing_if = "Option::is_none")]
	pub br: Option<ChargeBearerType1Code>,
	#[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
	pub agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
	pub tax: Option<TaxCharges2>,
}


// ClearingChannel2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ClearingChannel2Code {
	#[default]
	#[serde(rename = "RTGS")]
	CodeRTGS,
	#[serde(rename = "RTNS")]
	CodeRTNS,
	#[serde(rename = "MPNS")]
	CodeMPNS,
	#[serde(rename = "BOOK")]
	CodeBOOK,

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


// CopyDuplicate1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CopyDuplicate1Code {
	#[default]
	#[serde(rename = "CODU")]
	CodeCODU,
	#[serde(rename = "COPY")]
	CodeCOPY,
	#[serde(rename = "DUPL")]
	CodeDUPL,

}


// CorporateAction9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CorporateAction9 {
	#[serde(rename = "EvtTp")]
	pub evt_tp: Max35Text,
	#[serde(rename = "EvtId")]
	pub evt_id: Max35Text,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// CreditDebitCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CreditDebitCode {
	#[default]
	#[serde(rename = "CRDT")]
	CodeCRDT,
	#[serde(rename = "DBIT")]
	CodeDBIT,

}


// CreditorReferenceInformation3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceInformation3 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<CreditorReferenceType3>,
	#[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
	pub ref_attr: Option<Max35Text>,
}


// CreditorReferenceType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalCreditorReferenceType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// CreditorReferenceType3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceType3 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: CreditorReferenceType2Choice,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}


// CurrencyExchange24 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CurrencyExchange24 {
	#[serde(rename = "SrcCcy")]
	pub src_ccy: ActiveOrHistoricCurrencyCode,
	#[serde(rename = "TrgtCcy", skip_serializing_if = "Option::is_none")]
	pub trgt_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "UnitCcy", skip_serializing_if = "Option::is_none")]
	pub unit_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "XchgRate")]
	pub xchg_rate: f64,
	#[serde(rename = "CtrctId", skip_serializing_if = "Option::is_none")]
	pub ctrct_id: Option<Max35Text>,
	#[serde(rename = "QtnDt", skip_serializing_if = "Option::is_none")]
	pub qtn_dt: Option<String>,
	#[serde(rename = "XchgRateBase", skip_serializing_if = "Option::is_none")]
	pub xchg_rate_base: Option<PositiveNumber>,
}


// DateAndDateTime2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTime2Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<String>,
}


// DateAndPlaceOfBirth1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndPlaceOfBirth1 {
	#[serde(rename = "BirthDt")]
	pub birth_dt: String,
	#[serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none")]
	pub prvc_of_birth: Option<Max35Text>,
	#[serde(rename = "CityOfBirth")]
	pub city_of_birth: Max35Text,
	#[serde(rename = "CtryOfBirth")]
	pub ctry_of_birth: CountryCode,
}


// DateAndType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndType1 {
	#[serde(rename = "Tp")]
	pub tp: DateType2Choice,
	#[serde(rename = "Dt")]
	pub dt: String,
}


// DateOrDateTimePeriod1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateOrDateTimePeriod1Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<DatePeriod2>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
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


// DateType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalDateType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "$value")]
	pub decimal_number: f64,
}


// DisplayCapabilities1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DisplayCapabilities1 {
	#[serde(rename = "DispTp")]
	pub disp_tp: UserInterface2Code,
	#[serde(rename = "NbOfLines")]
	pub nb_of_lines: Max3NumericText,
	#[serde(rename = "LineWidth")]
	pub line_width: Max3NumericText,
}


// DocumentAdjustment1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentAdjustment1 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Max4Text>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max140Text>,
}


// DocumentAmount1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentAmount1 {
	#[serde(rename = "Tp")]
	pub tp: DocumentAmountType1Choice,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}


// DocumentAmountType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentAmountType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalDocumentAmountType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// DocumentLineIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentLineIdentification1 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<DocumentLineType1>,
	#[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
	pub nb: Option<Max35Text>,
	#[serde(rename = "RltdDt", skip_serializing_if = "Option::is_none")]
	pub rltd_dt: Option<String>,
}


// DocumentLineInformation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentLineInformation2 {
	#[serde(rename = "Id")]
	pub id: Vec<DocumentLineIdentification1>,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max2048Text>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<RemittanceAmount4>,
}


// DocumentLineType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentLineType1 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: DocumentLineType1Choice,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}


// DocumentLineType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentLineType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalDocumentLineType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// DocumentType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentType1 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: DocumentType2Choice,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}


// DocumentType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalDocumentType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// EntryDetails13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EntryDetails13 {
	#[serde(rename = "Btch", skip_serializing_if = "Option::is_none")]
	pub btch: Option<BatchInformation2>,
	#[serde(rename = "TxDtls", skip_serializing_if = "Option::is_none")]
	pub tx_dtls: Option<Vec<EntryTransaction14>>,
}


// EntryStatus1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EntryStatus1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalEntryStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// EntryTransaction14 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EntryTransaction14 {
	#[serde(rename = "Refs", skip_serializing_if = "Option::is_none")]
	pub refs: Option<TransactionReferences6>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
	#[serde(rename = "AmtDtls", skip_serializing_if = "Option::is_none")]
	pub amt_dtls: Option<AmountAndCurrencyExchange4>,
	#[serde(rename = "Avlbty", skip_serializing_if = "Option::is_none")]
	pub avlbty: Option<Vec<CashAvailability1>>,
	#[serde(rename = "BkTxCd", skip_serializing_if = "Option::is_none")]
	pub bk_tx_cd: Option<BankTransactionCodeStructure4>,
	#[serde(rename = "Chrgs", skip_serializing_if = "Option::is_none")]
	pub chrgs: Option<Charges15>,
	#[serde(rename = "Intrst", skip_serializing_if = "Option::is_none")]
	pub intrst: Option<TransactionInterest4>,
	#[serde(rename = "RltdPties", skip_serializing_if = "Option::is_none")]
	pub rltd_pties: Option<TransactionParties12>,
	#[serde(rename = "RltdAgts", skip_serializing_if = "Option::is_none")]
	pub rltd_agts: Option<TransactionAgents6>,
	#[serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none")]
	pub lcl_instrm: Option<LocalInstrument2Choice>,
	#[serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none")]
	pub pmt_tp_inf: Option<PaymentTypeInformation27>,
	#[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
	pub purp: Option<Purpose2Choice>,
	#[serde(rename = "RltdRmtInf", skip_serializing_if = "Option::is_none")]
	pub rltd_rmt_inf: Option<Vec<RemittanceLocation8>>,
	#[serde(rename = "RmtInf", skip_serializing_if = "Option::is_none")]
	pub rmt_inf: Option<RemittanceInformation22>,
	#[serde(rename = "RltdDts", skip_serializing_if = "Option::is_none")]
	pub rltd_dts: Option<TransactionDates3>,
	#[serde(rename = "RltdPric", skip_serializing_if = "Option::is_none")]
	pub rltd_pric: Option<TransactionPrice4Choice>,
	#[serde(rename = "RltdQties", skip_serializing_if = "Option::is_none")]
	pub rltd_qties: Option<Vec<TransactionQuantities3Choice>>,
	#[serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_id: Option<SecurityIdentification19>,
	#[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
	pub tax: Option<TaxData1>,
	#[serde(rename = "RtrInf", skip_serializing_if = "Option::is_none")]
	pub rtr_inf: Option<PaymentReturnReason8>,
	#[serde(rename = "CorpActn", skip_serializing_if = "Option::is_none")]
	pub corp_actn: Option<CorporateAction9>,
	#[serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none")]
	pub sfkpg_acct: Option<SecuritiesAccount19>,
	#[serde(rename = "CshDpst", skip_serializing_if = "Option::is_none")]
	pub csh_dpst: Option<Vec<CashDeposit1>>,
	#[serde(rename = "CardTx", skip_serializing_if = "Option::is_none")]
	pub card_tx: Option<CardTransaction18>,
	#[serde(rename = "AddtlTxInf", skip_serializing_if = "Option::is_none")]
	pub addtl_tx_inf: Option<Max500Text>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// Exact1NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact1NumericText {
	#[serde(rename = "$value")]
	pub exact1_numeric_text: String,
}


// Exact3NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact3NumericText {
	#[serde(rename = "$value")]
	pub exact3_numeric_text: String,
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


// ExternalCardTransactionCategory1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalCardTransactionCategory1Code {
	#[serde(rename = "$value")]
	pub external_card_transaction_category1_code: String,
}


// ExternalCashAccountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalCashAccountType1Code {
	#[serde(rename = "$value")]
	pub external_cash_account_type1_code: String,
}


// ExternalCategoryPurpose1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalCategoryPurpose1Code {
	#[serde(rename = "$value")]
	pub external_category_purpose1_code: String,
}


// ExternalChargeType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalChargeType1Code {
	#[serde(rename = "$value")]
	pub external_charge_type1_code: String,
}


// ExternalClearingSystemIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalClearingSystemIdentification1Code {
	#[serde(rename = "$value")]
	pub external_clearing_system_identification1_code: String,
}


// ExternalCreditorReferenceType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalCreditorReferenceType1Code {
	#[serde(rename = "$value")]
	pub external_creditor_reference_type1_code: String,
}


// ExternalDateType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalDateType1Code {
	#[serde(rename = "$value")]
	pub external_date_type1_code: String,
}


// ExternalDocumentAmountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalDocumentAmountType1Code {
	#[serde(rename = "$value")]
	pub external_document_amount_type1_code: String,
}


// ExternalDocumentLineType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalDocumentLineType1Code {
	#[serde(rename = "$value")]
	pub external_document_line_type1_code: String,
}


// ExternalDocumentType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalDocumentType1Code {
	#[serde(rename = "$value")]
	pub external_document_type1_code: String,
}


// ExternalEntryStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalEntryStatus1Code {
	#[serde(rename = "$value")]
	pub external_entry_status1_code: String,
}


// ExternalFinancialInstitutionIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalFinancialInstitutionIdentification1Code {
	#[serde(rename = "$value")]
	pub external_financial_institution_identification1_code: String,
}


// ExternalFinancialInstrumentIdentificationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalFinancialInstrumentIdentificationType1Code {
	#[serde(rename = "$value")]
	pub external_financial_instrument_identification_type1_code: String,
}


// ExternalGarnishmentType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalGarnishmentType1Code {
	#[serde(rename = "$value")]
	pub external_garnishment_type1_code: String,
}


// ExternalLocalInstrument1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalLocalInstrument1Code {
	#[serde(rename = "$value")]
	pub external_local_instrument1_code: String,
}


// ExternalOrganisationIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalOrganisationIdentification1Code {
	#[serde(rename = "$value")]
	pub external_organisation_identification1_code: String,
}


// ExternalPersonIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPersonIdentification1Code {
	#[serde(rename = "$value")]
	pub external_person_identification1_code: String,
}


// ExternalProxyAccountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalProxyAccountType1Code {
	#[serde(rename = "$value")]
	pub external_proxy_account_type1_code: String,
}


// ExternalPurpose1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPurpose1Code {
	#[serde(rename = "$value")]
	pub external_purpose1_code: String,
}


// ExternalRePresentmentReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalRePresentmentReason1Code {
	#[serde(rename = "$value")]
	pub external_re_presentment_reason1_code: String,
}


// ExternalReportingSource1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalReportingSource1Code {
	#[serde(rename = "$value")]
	pub external_reporting_source1_code: String,
}


// ExternalReturnReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalReturnReason1Code {
	#[serde(rename = "$value")]
	pub external_return_reason1_code: String,
}


// ExternalServiceLevel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalServiceLevel1Code {
	#[serde(rename = "$value")]
	pub external_service_level1_code: String,
}


// ExternalTechnicalInputChannel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalTechnicalInputChannel1Code {
	#[serde(rename = "$value")]
	pub external_technical_input_channel1_code: String,
}


// FinancialIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalFinancialInstitutionIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
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


// FinancialInstrumentQuantity1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentQuantity1Choice {
	#[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
	pub unit: Option<f64>,
	#[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
	pub face_amt: Option<f64>,
	#[serde(rename = "AmtsdVal", skip_serializing_if = "Option::is_none")]
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


// Garnishment4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Garnishment4 {
	#[serde(rename = "Tp")]
	pub tp: GarnishmentType1,
	#[serde(rename = "Grnshee", skip_serializing_if = "Option::is_none")]
	pub grnshee: Option<PartyIdentification272>,
	#[serde(rename = "GrnshmtAdmstr", skip_serializing_if = "Option::is_none")]
	pub grnshmt_admstr: Option<PartyIdentification272>,
	#[serde(rename = "RefNb", skip_serializing_if = "Option::is_none")]
	pub ref_nb: Option<Max140Text>,
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none")]
	pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "FmlyMdclInsrncInd", skip_serializing_if = "Option::is_none")]
	pub fmly_mdcl_insrnc_ind: Option<bool>,
	#[serde(rename = "MplyeeTermntnInd", skip_serializing_if = "Option::is_none")]
	pub mplyee_termntn_ind: Option<bool>,
}


// GarnishmentType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GarnishmentType1 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: GarnishmentType1Choice,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}


// GarnishmentType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GarnishmentType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalGarnishmentType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
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


// GenericIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification3 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
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


// GenericIdentification32 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification32 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<PartyType3Code>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<PartyType4Code>,
	#[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
	pub shrt_nm: Option<Max35Text>,
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


// GenericPersonIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericPersonIdentification2 {
	#[serde(rename = "Id")]
	pub id: Max256Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}


// GroupHeader116 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GroupHeader116 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
	#[serde(rename = "MsgRcpt", skip_serializing_if = "Option::is_none")]
	pub msg_rcpt: Option<PartyIdentification272>,
	#[serde(rename = "MsgPgntn", skip_serializing_if = "Option::is_none")]
	pub msg_pgntn: Option<Pagination1>,
	#[serde(rename = "OrgnlBizQry", skip_serializing_if = "Option::is_none")]
	pub orgnl_biz_qry: Option<OriginalBusinessQuery1>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max500Text>,
}


// IBAN2007Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IBAN2007Identifier {
	#[serde(rename = "$value")]
	pub iban2007_identifier: String,
}


// ISINOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "$value")]
	pub isin_oct2015_identifier: String,
}


// ISO2ALanguageCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISO2ALanguageCode {
	#[serde(rename = "$value")]
	pub iso2_a_language_code: String,
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


// ISOYear ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISOYear {
	#[serde(rename = "$value")]
	pub iso_year: String,
}


// ISOYearMonth ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISOYearMonth {
	#[serde(rename = "$value")]
	pub iso_year_month: String,
}


// IdentificationSource3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IdentificationSource3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalFinancialInstrumentIdentificationType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// ImpliedCurrencyAmountRange1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImpliedCurrencyAmountRange1Choice {
	#[serde(rename = "FrAmt", skip_serializing_if = "Option::is_none")]
	pub fr_amt: Option<AmountRangeBoundary1>,
	#[serde(rename = "ToAmt", skip_serializing_if = "Option::is_none")]
	pub to_amt: Option<AmountRangeBoundary1>,
	#[serde(rename = "FrToAmt", skip_serializing_if = "Option::is_none")]
	pub fr_to_amt: Option<FromToAmountRange1>,
	#[serde(rename = "EQAmt", skip_serializing_if = "Option::is_none")]
	pub eq_amt: Option<f64>,
	#[serde(rename = "NEQAmt", skip_serializing_if = "Option::is_none")]
	pub neq_amt: Option<f64>,
}


// ImpliedCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImpliedCurrencyAndAmount {
	#[serde(rename = "$value")]
	pub implied_currency_and_amount: f64,
}


// InterestRecord2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRecord2 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: CreditDebitCode,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<InterestType1Choice>,
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<Rate4>,
	#[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
	pub fr_to_dt: Option<DateTimePeriod1>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Max35Text>,
	#[serde(rename = "Tax", skip_serializing_if = "Option::is_none")]
	pub tax: Option<TaxCharges2>,
}


// InterestType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InterestType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// InterestType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InterestType1Code {
	#[default]
	#[serde(rename = "INDY")]
	CodeINDY,
	#[serde(rename = "OVRN")]
	CodeOVRN,

}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}


// LocalInstrument2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LocalInstrument2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalLocalInstrument1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// Max1025Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max1025Text {
	#[serde(rename = "$value")]
	pub max1025_text: String,
}


// Max105Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max105Text {
	#[serde(rename = "$value")]
	pub max105_text: String,
}


// Max128Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max128Text {
	#[serde(rename = "$value")]
	pub max128_text: String,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
}


// Max15NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max15NumericText {
	#[serde(rename = "$value")]
	pub max15_numeric_text: String,
}


// Max15PlusSignedNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max15PlusSignedNumericText {
	#[serde(rename = "$value")]
	pub max15_plus_signed_numeric_text: String,
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


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}


// Max3NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max3NumericText {
	#[serde(rename = "$value")]
	pub max3_numeric_text: String,
}


// Max4Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4Text {
	#[serde(rename = "$value")]
	pub max4_text: String,
}


// Max500Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max500Text {
	#[serde(rename = "$value")]
	pub max500_text: String,
}


// Max5NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max5NumericText {
	#[serde(rename = "$value")]
	pub max5_numeric_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
}


// MessageIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageIdentification2 {
	#[serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none")]
	pub msg_nm_id: Option<Max35Text>,
	#[serde(rename = "MsgId", skip_serializing_if = "Option::is_none")]
	pub msg_id: Option<Max35Text>,
}


// Min2Max3NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Min2Max3NumericText {
	#[serde(rename = "$value")]
	pub min2_max3_numeric_text: String,
}


// Min3Max4NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Min3Max4NumericText {
	#[serde(rename = "$value")]
	pub min3_max4_numeric_text: String,
}


// Min8Max28NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Min8Max28NumericText {
	#[serde(rename = "$value")]
	pub min8_max28_numeric_text: String,
}


// NameAndAddress18 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress18 {
	#[serde(rename = "Nm")]
	pub nm: Max140Text,
	#[serde(rename = "Adr")]
	pub adr: PostalAddress27,
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


// NonNegativeDecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NonNegativeDecimalNumber {
	#[serde(rename = "$value")]
	pub non_negative_decimal_number: f64,
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "$value")]
	pub number: f64,
}


// NumberAndSumOfTransactions1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NumberAndSumOfTransactions1 {
	#[serde(rename = "NbOfNtries", skip_serializing_if = "Option::is_none")]
	pub nb_of_ntries: Option<Max15NumericText>,
	#[serde(rename = "Sum", skip_serializing_if = "Option::is_none")]
	pub sum: Option<f64>,
}


// NumberAndSumOfTransactions4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NumberAndSumOfTransactions4 {
	#[serde(rename = "NbOfNtries", skip_serializing_if = "Option::is_none")]
	pub nb_of_ntries: Option<Max15NumericText>,
	#[serde(rename = "Sum", skip_serializing_if = "Option::is_none")]
	pub sum: Option<f64>,
	#[serde(rename = "TtlNetNtry", skip_serializing_if = "Option::is_none")]
	pub ttl_net_ntry: Option<AmountAndDirection35>,
}


// OnLineCapability1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OnLineCapability1Code {
	#[default]
	#[serde(rename = "OFLN")]
	CodeOFLN,
	#[serde(rename = "ONLN")]
	CodeONLN,
	#[serde(rename = "SMON")]
	CodeSMON,

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
	pub msg_id: Max35Text,
	#[serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none")]
	pub msg_nm_id: Option<Max35Text>,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
}


// OtherContact1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherContact1 {
	#[serde(rename = "ChanlTp")]
	pub chanl_tp: Max4Text,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max128Text>,
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


// POIComponentType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum POIComponentType1Code {
	#[default]
	#[serde(rename = "SOFT")]
	CodeSOFT,
	#[serde(rename = "EMVK")]
	CodeEMVK,
	#[serde(rename = "EMVO")]
	CodeEMVO,
	#[serde(rename = "MRIT")]
	CodeMRIT,
	#[serde(rename = "CHIT")]
	CodeCHIT,
	#[serde(rename = "SECM")]
	CodeSECM,
	#[serde(rename = "PEDV")]
	CodePEDV,

}


// Pagination1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Pagination1 {
	#[serde(rename = "PgNb")]
	pub pg_nb: Max5NumericText,
	#[serde(rename = "LastPgInd")]
	pub last_pg_ind: bool,
}


// Party50Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party50Choice {
	#[serde(rename = "Pty", skip_serializing_if = "Option::is_none")]
	pub pty: Option<PartyIdentification272>,
	#[serde(rename = "Agt", skip_serializing_if = "Option::is_none")]
	pub agt: Option<BranchAndFinancialInstitutionIdentification8>,
}


// Party52Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party52Choice {
	#[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
	pub org_id: Option<OrganisationIdentification39>,
	#[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
	pub prvt_id: Option<PersonIdentification18>,
}


// PartyIdentification272 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification272 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress27>,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Party52Choice>,
	#[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
	pub ctry_of_res: Option<CountryCode>,
	#[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
	pub ctct_dtls: Option<Contact13>,
}


// PartyType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PartyType3Code {
	#[default]
	#[serde(rename = "OPOI")]
	CodeOPOI,
	#[serde(rename = "MERC")]
	CodeMERC,
	#[serde(rename = "ACCP")]
	CodeACCP,
	#[serde(rename = "ITAG")]
	CodeITAG,
	#[serde(rename = "ACQR")]
	CodeACQR,
	#[serde(rename = "CISS")]
	CodeCISS,
	#[serde(rename = "DLIS")]
	CodeDLIS,

}


// PartyType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PartyType4Code {
	#[default]
	#[serde(rename = "MERC")]
	CodeMERC,
	#[serde(rename = "ACCP")]
	CodeACCP,
	#[serde(rename = "ITAG")]
	CodeITAG,
	#[serde(rename = "ACQR")]
	CodeACQR,
	#[serde(rename = "CISS")]
	CodeCISS,
	#[serde(rename = "TAXH")]
	CodeTAXH,

}


// PaymentCard4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentCard4 {
	#[serde(rename = "PlainCardData", skip_serializing_if = "Option::is_none")]
	pub plain_card_data: Option<PlainCardData1>,
	#[serde(rename = "CardCtryCd", skip_serializing_if = "Option::is_none")]
	pub card_ctry_cd: Option<Exact3NumericText>,
	#[serde(rename = "CardBrnd", skip_serializing_if = "Option::is_none")]
	pub card_brnd: Option<GenericIdentification1>,
	#[serde(rename = "AddtlCardData", skip_serializing_if = "Option::is_none")]
	pub addtl_card_data: Option<Max70Text>,
}


// PaymentContext3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentContext3 {
	#[serde(rename = "CardPres", skip_serializing_if = "Option::is_none")]
	pub card_pres: Option<bool>,
	#[serde(rename = "CrdhldrPres", skip_serializing_if = "Option::is_none")]
	pub crdhldr_pres: Option<bool>,
	#[serde(rename = "OnLineCntxt", skip_serializing_if = "Option::is_none")]
	pub on_line_cntxt: Option<bool>,
	#[serde(rename = "AttndncCntxt", skip_serializing_if = "Option::is_none")]
	pub attndnc_cntxt: Option<AttendanceContext1Code>,
	#[serde(rename = "TxEnvt", skip_serializing_if = "Option::is_none")]
	pub tx_envt: Option<TransactionEnvironment1Code>,
	#[serde(rename = "TxChanl", skip_serializing_if = "Option::is_none")]
	pub tx_chanl: Option<TransactionChannel1Code>,
	#[serde(rename = "AttndntMsgCpbl", skip_serializing_if = "Option::is_none")]
	pub attndnt_msg_cpbl: Option<bool>,
	#[serde(rename = "AttndntLang", skip_serializing_if = "Option::is_none")]
	pub attndnt_lang: Option<ISO2ALanguageCode>,
	#[serde(rename = "CardDataNtryMd")]
	pub card_data_ntry_md: CardDataReading1Code,
	#[serde(rename = "FllbckInd", skip_serializing_if = "Option::is_none")]
	pub fllbck_ind: Option<bool>,
	#[serde(rename = "AuthntcnMtd", skip_serializing_if = "Option::is_none")]
	pub authntcn_mtd: Option<CardholderAuthentication2>,
}


// PaymentReturnReason8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentReturnReason8 {
	#[serde(rename = "OrgnlBkTxCd", skip_serializing_if = "Option::is_none")]
	pub orgnl_bk_tx_cd: Option<BankTransactionCodeStructure4>,
	#[serde(rename = "Orgtr", skip_serializing_if = "Option::is_none")]
	pub orgtr: Option<PartyIdentification272>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<ReturnReason5Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<Max105Text>>,
}


// PaymentTypeInformation27 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentTypeInformation27 {
	#[serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none")]
	pub instr_prty: Option<Priority2Code>,
	#[serde(rename = "ClrChanl", skip_serializing_if = "Option::is_none")]
	pub clr_chanl: Option<ClearingChannel2Code>,
	#[serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none")]
	pub svc_lvl: Option<Vec<ServiceLevel8Choice>>,
	#[serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none")]
	pub lcl_instrm: Option<LocalInstrument2Choice>,
	#[serde(rename = "SeqTp", skip_serializing_if = "Option::is_none")]
	pub seq_tp: Option<SequenceType3Code>,
	#[serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none")]
	pub ctgy_purp: Option<CategoryPurpose1Choice>,
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "$value")]
	pub percentage_rate: f64,
}


// PersonIdentification18 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentification18 {
	#[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
	pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<GenericPersonIdentification2>>,
}


// PersonIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalPersonIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// PhoneNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PhoneNumber {
	#[serde(rename = "$value")]
	pub phone_number: String,
}


// PlainCardData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlainCardData1 {
	#[serde(rename = "PAN")]
	pub pan: Min8Max28NumericText,
	#[serde(rename = "CardSeqNb", skip_serializing_if = "Option::is_none")]
	pub card_seq_nb: Option<Min2Max3NumericText>,
	#[serde(rename = "FctvDt", skip_serializing_if = "Option::is_none")]
	pub fctv_dt: Option<String>,
	#[serde(rename = "XpryDt")]
	pub xpry_dt: String,
	#[serde(rename = "SvcCd", skip_serializing_if = "Option::is_none")]
	pub svc_cd: Option<Exact3NumericText>,
	#[serde(rename = "TrckData", skip_serializing_if = "Option::is_none")]
	pub trck_data: Option<Vec<TrackData1>>,
	#[serde(rename = "CardSctyCd", skip_serializing_if = "Option::is_none")]
	pub card_scty_cd: Option<CardSecurityInformation1>,
}


// PointOfInteraction1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PointOfInteraction1 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification32,
	#[serde(rename = "SysNm", skip_serializing_if = "Option::is_none")]
	pub sys_nm: Option<Max70Text>,
	#[serde(rename = "GrpId", skip_serializing_if = "Option::is_none")]
	pub grp_id: Option<Max35Text>,
	#[serde(rename = "Cpblties", skip_serializing_if = "Option::is_none")]
	pub cpblties: Option<PointOfInteractionCapabilities1>,
	#[serde(rename = "Cmpnt", skip_serializing_if = "Option::is_none")]
	pub cmpnt: Option<Vec<PointOfInteractionComponent1>>,
}


// PointOfInteractionCapabilities1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PointOfInteractionCapabilities1 {
	#[serde(rename = "CardRdngCpblties", skip_serializing_if = "Option::is_none")]
	pub card_rdng_cpblties: Option<Vec<CardDataReading1Code>>,
	#[serde(rename = "CrdhldrVrfctnCpblties", skip_serializing_if = "Option::is_none")]
	pub crdhldr_vrfctn_cpblties: Option<Vec<CardholderVerificationCapability1Code>>,
	#[serde(rename = "OnLineCpblties", skip_serializing_if = "Option::is_none")]
	pub on_line_cpblties: Option<OnLineCapability1Code>,
	#[serde(rename = "DispCpblties", skip_serializing_if = "Option::is_none")]
	pub disp_cpblties: Option<Vec<DisplayCapabilities1>>,
	#[serde(rename = "PrtLineWidth", skip_serializing_if = "Option::is_none")]
	pub prt_line_width: Option<Max3NumericText>,
}


// PointOfInteractionComponent1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PointOfInteractionComponent1 {
	#[serde(rename = "POICmpntTp")]
	pub poi_cmpnt_tp: POIComponentType1Code,
	#[serde(rename = "ManfctrId", skip_serializing_if = "Option::is_none")]
	pub manfctr_id: Option<Max35Text>,
	#[serde(rename = "Mdl", skip_serializing_if = "Option::is_none")]
	pub mdl: Option<Max35Text>,
	#[serde(rename = "VrsnNb", skip_serializing_if = "Option::is_none")]
	pub vrsn_nb: Option<Max16Text>,
	#[serde(rename = "SrlNb", skip_serializing_if = "Option::is_none")]
	pub srl_nb: Option<Max35Text>,
	#[serde(rename = "ApprvlNb", skip_serializing_if = "Option::is_none")]
	pub apprvl_nb: Option<Vec<Max70Text>>,
}


// PositiveNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositiveNumber {
	#[serde(rename = "$value")]
	pub positive_number: f64,
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
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
}


// PriceValueType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PriceValueType1Code {
	#[default]
	#[serde(rename = "DISC")]
	CodeDISC,
	#[serde(rename = "PREM")]
	CodePREM,
	#[serde(rename = "PARV")]
	CodePARV,

}


// Priority2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Priority2Code {
	#[default]
	#[serde(rename = "HIGH")]
	CodeHIGH,
	#[serde(rename = "NORM")]
	CodeNORM,

}


// Product2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Product2 {
	#[serde(rename = "PdctCd")]
	pub pdct_cd: Max70Text,
	#[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
	pub unit_of_measr: Option<UnitOfMeasure1Code>,
	#[serde(rename = "PdctQty", skip_serializing_if = "Option::is_none")]
	pub pdct_qty: Option<f64>,
	#[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
	pub unit_pric: Option<f64>,
	#[serde(rename = "PdctAmt", skip_serializing_if = "Option::is_none")]
	pub pdct_amt: Option<f64>,
	#[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
	pub tax_tp: Option<Max35Text>,
	#[serde(rename = "AddtlPdctInf", skip_serializing_if = "Option::is_none")]
	pub addtl_pdct_inf: Option<Max35Text>,
}


// ProprietaryAgent5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryAgent5 {
	#[serde(rename = "Tp")]
	pub tp: Max35Text,
	#[serde(rename = "Agt")]
	pub agt: BranchAndFinancialInstitutionIdentification8,
}


// ProprietaryBankTransactionCodeStructure1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryBankTransactionCodeStructure1 {
	#[serde(rename = "Cd")]
	pub cd: Max35Text,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}


// ProprietaryDate3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryDate3 {
	#[serde(rename = "Tp")]
	pub tp: Max35Text,
	#[serde(rename = "Dt")]
	pub dt: DateAndDateTime2Choice,
}


// ProprietaryParty6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryParty6 {
	#[serde(rename = "Tp")]
	pub tp: Max35Text,
	#[serde(rename = "Pty")]
	pub pty: Party50Choice,
}


// ProprietaryPrice2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryPrice2 {
	#[serde(rename = "Tp")]
	pub tp: Max35Text,
	#[serde(rename = "Pric")]
	pub pric: ActiveOrHistoricCurrencyAndAmount,
}


// ProprietaryQuantity1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryQuantity1 {
	#[serde(rename = "Tp")]
	pub tp: Max35Text,
	#[serde(rename = "Qty")]
	pub qty: Max35Text,
}


// ProprietaryReference1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryReference1 {
	#[serde(rename = "Tp")]
	pub tp: Max35Text,
	#[serde(rename = "Ref")]
	pub ref_attr: Max35Text,
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


// Purpose2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Purpose2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalPurpose1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// Rate4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Rate4 {
	#[serde(rename = "Tp")]
	pub tp: RateType4Choice,
	#[serde(rename = "VldtyRg", skip_serializing_if = "Option::is_none")]
	pub vldty_rg: Option<ActiveOrHistoricCurrencyAndAmountRange2>,
}


// RateType4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RateType4Choice {
	#[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
	pub pctg: Option<f64>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Max35Text>,
}


// ReferredDocumentInformation8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReferredDocumentInformation8 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<DocumentType1>,
	#[serde(rename = "Nb", skip_serializing_if = "Option::is_none")]
	pub nb: Option<Max35Text>,
	#[serde(rename = "RltdDt", skip_serializing_if = "Option::is_none")]
	pub rltd_dt: Option<DateAndType1>,
	#[serde(rename = "LineDtls", skip_serializing_if = "Option::is_none")]
	pub line_dtls: Option<Vec<DocumentLineInformation2>>,
}


// RemittanceAmount4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceAmount4 {
	#[serde(rename = "RmtAmtAndTp", skip_serializing_if = "Option::is_none")]
	pub rmt_amt_and_tp: Option<Vec<DocumentAmount1>>,
	#[serde(rename = "AdjstmntAmtAndRsn", skip_serializing_if = "Option::is_none")]
	pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment1>>,
}


// RemittanceInformation22 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceInformation22 {
	#[serde(rename = "Ustrd", skip_serializing_if = "Option::is_none")]
	pub ustrd: Option<Vec<Max140Text>>,
	#[serde(rename = "Strd", skip_serializing_if = "Option::is_none")]
	pub strd: Option<Vec<StructuredRemittanceInformation18>>,
}


// RemittanceLocation8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceLocation8 {
	#[serde(rename = "RmtId", skip_serializing_if = "Option::is_none")]
	pub rmt_id: Option<Max35Text>,
	#[serde(rename = "RmtLctnDtls", skip_serializing_if = "Option::is_none")]
	pub rmt_lctn_dtls: Option<Vec<RemittanceLocationData2>>,
}


// RemittanceLocationData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RemittanceLocationData2 {
	#[serde(rename = "Mtd")]
	pub mtd: RemittanceLocationMethod2Code,
	#[serde(rename = "ElctrncAdr", skip_serializing_if = "Option::is_none")]
	pub elctrnc_adr: Option<Max2048Text>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<NameAndAddress18>,
}


// RemittanceLocationMethod2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RemittanceLocationMethod2Code {
	#[default]
	#[serde(rename = "FAXI")]
	CodeFAXI,
	#[serde(rename = "EDIC")]
	CodeEDIC,
	#[serde(rename = "URID")]
	CodeURID,
	#[serde(rename = "EMAL")]
	CodeEMAL,
	#[serde(rename = "POST")]
	CodePOST,
	#[serde(rename = "SMSM")]
	CodeSMSM,

}


// ReportEntry14 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportEntry14 {
	#[serde(rename = "NtryRef", skip_serializing_if = "Option::is_none")]
	pub ntry_ref: Option<Max35Text>,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: CreditDebitCode,
	#[serde(rename = "RvslInd", skip_serializing_if = "Option::is_none")]
	pub rvsl_ind: Option<bool>,
	#[serde(rename = "Sts")]
	pub sts: EntryStatus1Choice,
	#[serde(rename = "BookgDt", skip_serializing_if = "Option::is_none")]
	pub bookg_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
	pub val_dt: Option<DateAndDateTime2Choice>,
	#[serde(rename = "AcctSvcrRef", skip_serializing_if = "Option::is_none")]
	pub acct_svcr_ref: Option<Max35Text>,
	#[serde(rename = "Avlbty", skip_serializing_if = "Option::is_none")]
	pub avlbty: Option<Vec<CashAvailability1>>,
	#[serde(rename = "BkTxCd")]
	pub bk_tx_cd: BankTransactionCodeStructure4,
	#[serde(rename = "ComssnWvrInd", skip_serializing_if = "Option::is_none")]
	pub comssn_wvr_ind: Option<bool>,
	#[serde(rename = "AddtlInfInd", skip_serializing_if = "Option::is_none")]
	pub addtl_inf_ind: Option<MessageIdentification2>,
	#[serde(rename = "AmtDtls", skip_serializing_if = "Option::is_none")]
	pub amt_dtls: Option<AmountAndCurrencyExchange4>,
	#[serde(rename = "Chrgs", skip_serializing_if = "Option::is_none")]
	pub chrgs: Option<Charges15>,
	#[serde(rename = "TechInptChanl", skip_serializing_if = "Option::is_none")]
	pub tech_inpt_chanl: Option<TechnicalInputChannel1Choice>,
	#[serde(rename = "Intrst", skip_serializing_if = "Option::is_none")]
	pub intrst: Option<TransactionInterest4>,
	#[serde(rename = "CardTx", skip_serializing_if = "Option::is_none")]
	pub card_tx: Option<CardEntry5>,
	#[serde(rename = "NtryDtls", skip_serializing_if = "Option::is_none")]
	pub ntry_dtls: Option<Vec<EntryDetails13>>,
	#[serde(rename = "AddtlNtryInf", skip_serializing_if = "Option::is_none")]
	pub addtl_ntry_inf: Option<Max500Text>,
}


// ReportingSource1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportingSource1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalReportingSource1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// ReturnReason5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReturnReason5Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalReturnReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// SecuritiesAccount19 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesAccount19 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<GenericIdentification30>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max70Text>,
}


// SecurityIdentification19 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification19 {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
	#[serde(rename = "OthrId", skip_serializing_if = "Option::is_none")]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max140Text>,
}


// SequenceRange1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SequenceRange1 {
	#[serde(rename = "FrSeq")]
	pub fr_seq: Max35Text,
	#[serde(rename = "ToSeq")]
	pub to_seq: Max35Text,
}


// SequenceRange1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SequenceRange1Choice {
	#[serde(rename = "FrSeq", skip_serializing_if = "Option::is_none")]
	pub fr_seq: Option<Max35Text>,
	#[serde(rename = "ToSeq", skip_serializing_if = "Option::is_none")]
	pub to_seq: Option<Max35Text>,
	#[serde(rename = "FrToSeq", skip_serializing_if = "Option::is_none")]
	pub fr_to_seq: Option<Vec<SequenceRange1>>,
	#[serde(rename = "EQSeq", skip_serializing_if = "Option::is_none")]
	pub eq_seq: Option<Vec<Max35Text>>,
	#[serde(rename = "NEQSeq", skip_serializing_if = "Option::is_none")]
	pub neq_seq: Option<Vec<Max35Text>>,
}


// SequenceType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SequenceType3Code {
	#[default]
	#[serde(rename = "FRST")]
	CodeFRST,
	#[serde(rename = "RCUR")]
	CodeRCUR,
	#[serde(rename = "FNAL")]
	CodeFNAL,
	#[serde(rename = "OOFF")]
	CodeOOFF,
	#[serde(rename = "RPRE")]
	CodeRPRE,

}


// ServiceLevel8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ServiceLevel8Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalServiceLevel1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// StructuredRemittanceInformation18 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StructuredRemittanceInformation18 {
	#[serde(rename = "RfrdDocInf", skip_serializing_if = "Option::is_none")]
	pub rfrd_doc_inf: Option<Vec<ReferredDocumentInformation8>>,
	#[serde(rename = "RfrdDocAmt", skip_serializing_if = "Option::is_none")]
	pub rfrd_doc_amt: Option<RemittanceAmount4>,
	#[serde(rename = "CdtrRefInf", skip_serializing_if = "Option::is_none")]
	pub cdtr_ref_inf: Option<CreditorReferenceInformation3>,
	#[serde(rename = "Invcr", skip_serializing_if = "Option::is_none")]
	pub invcr: Option<PartyIdentification272>,
	#[serde(rename = "Invcee", skip_serializing_if = "Option::is_none")]
	pub invcee: Option<PartyIdentification272>,
	#[serde(rename = "TaxRmt", skip_serializing_if = "Option::is_none")]
	pub tax_rmt: Option<TaxData1>,
	#[serde(rename = "GrnshmtRmt", skip_serializing_if = "Option::is_none")]
	pub grnshmt_rmt: Option<Garnishment4>,
	#[serde(rename = "AddtlRmtInf", skip_serializing_if = "Option::is_none")]
	pub addtl_rmt_inf: Option<Vec<Max140Text>>,
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
	pub plc_and_nm: Option<Max350Text>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}


// TaxAmount3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxAmount3 {
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "TaxblBaseAmt", skip_serializing_if = "Option::is_none")]
	pub taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none")]
	pub ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Dtls", skip_serializing_if = "Option::is_none")]
	pub dtls: Option<Vec<TaxRecordDetails3>>,
}


// TaxAuthorisation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxAuthorisation1 {
	#[serde(rename = "Titl", skip_serializing_if = "Option::is_none")]
	pub titl: Option<Max35Text>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
}


// TaxCharges2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxCharges2 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max35Text>,
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// TaxData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxData1 {
	#[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
	pub cdtr: Option<TaxParty1>,
	#[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
	pub dbtr: Option<TaxParty2>,
	#[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
	pub ultmt_dbtr: Option<TaxParty2>,
	#[serde(rename = "AdmstnZone", skip_serializing_if = "Option::is_none")]
	pub admstn_zone: Option<Max35Text>,
	#[serde(rename = "RefNb", skip_serializing_if = "Option::is_none")]
	pub ref_nb: Option<Max140Text>,
	#[serde(rename = "Mtd", skip_serializing_if = "Option::is_none")]
	pub mtd: Option<Max35Text>,
	#[serde(rename = "TtlTaxblBaseAmt", skip_serializing_if = "Option::is_none")]
	pub ttl_taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TtlTaxAmt", skip_serializing_if = "Option::is_none")]
	pub ttl_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
	pub seq_nb: Option<f64>,
	#[serde(rename = "Rcrd", skip_serializing_if = "Option::is_none")]
	pub rcrd: Option<Vec<TaxRecord3>>,
}


// TaxParty1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxParty1 {
	#[serde(rename = "TaxId", skip_serializing_if = "Option::is_none")]
	pub tax_id: Option<Max35Text>,
	#[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
	pub regn_id: Option<Max35Text>,
	#[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
	pub tax_tp: Option<Max35Text>,
}


// TaxParty2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxParty2 {
	#[serde(rename = "TaxId", skip_serializing_if = "Option::is_none")]
	pub tax_id: Option<Max35Text>,
	#[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
	pub regn_id: Option<Max35Text>,
	#[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
	pub tax_tp: Option<Max35Text>,
	#[serde(rename = "Authstn", skip_serializing_if = "Option::is_none")]
	pub authstn: Option<TaxAuthorisation1>,
}


// TaxPeriod3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxPeriod3 {
	#[serde(rename = "Yr", skip_serializing_if = "Option::is_none")]
	pub yr: Option<String>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<TaxRecordPeriod1Code>,
	#[serde(rename = "FrToDt", skip_serializing_if = "Option::is_none")]
	pub fr_to_dt: Option<DatePeriod2>,
}


// TaxRecord3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxRecord3 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<Max35Text>,
	#[serde(rename = "Ctgy", skip_serializing_if = "Option::is_none")]
	pub ctgy: Option<Max35Text>,
	#[serde(rename = "CtgyDtls", skip_serializing_if = "Option::is_none")]
	pub ctgy_dtls: Option<Max35Text>,
	#[serde(rename = "DbtrSts", skip_serializing_if = "Option::is_none")]
	pub dbtr_sts: Option<Max35Text>,
	#[serde(rename = "CertId", skip_serializing_if = "Option::is_none")]
	pub cert_id: Option<Max35Text>,
	#[serde(rename = "FrmsCd", skip_serializing_if = "Option::is_none")]
	pub frms_cd: Option<Max35Text>,
	#[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
	pub prd: Option<TaxPeriod3>,
	#[serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none")]
	pub tax_amt: Option<TaxAmount3>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max140Text>,
}


// TaxRecordDetails3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxRecordDetails3 {
	#[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
	pub prd: Option<TaxPeriod3>,
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}


// TaxRecordPeriod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TaxRecordPeriod1Code {
	#[default]
	#[serde(rename = "MM01")]
	CodeMM01,
	#[serde(rename = "MM02")]
	CodeMM02,
	#[serde(rename = "MM03")]
	CodeMM03,
	#[serde(rename = "MM04")]
	CodeMM04,
	#[serde(rename = "MM05")]
	CodeMM05,
	#[serde(rename = "MM06")]
	CodeMM06,
	#[serde(rename = "MM07")]
	CodeMM07,
	#[serde(rename = "MM08")]
	CodeMM08,
	#[serde(rename = "MM09")]
	CodeMM09,
	#[serde(rename = "MM10")]
	CodeMM10,
	#[serde(rename = "MM11")]
	CodeMM11,
	#[serde(rename = "MM12")]
	CodeMM12,
	#[serde(rename = "QTR1")]
	CodeQTR1,
	#[serde(rename = "QTR2")]
	CodeQTR2,
	#[serde(rename = "QTR3")]
	CodeQTR3,
	#[serde(rename = "QTR4")]
	CodeQTR4,
	#[serde(rename = "HLF1")]
	CodeHLF1,
	#[serde(rename = "HLF2")]
	CodeHLF2,

}


// TechnicalInputChannel1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TechnicalInputChannel1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalTechnicalInputChannel1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// TotalTransactions6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TotalTransactions6 {
	#[serde(rename = "TtlNtries", skip_serializing_if = "Option::is_none")]
	pub ttl_ntries: Option<NumberAndSumOfTransactions4>,
	#[serde(rename = "TtlCdtNtries", skip_serializing_if = "Option::is_none")]
	pub ttl_cdt_ntries: Option<NumberAndSumOfTransactions1>,
	#[serde(rename = "TtlDbtNtries", skip_serializing_if = "Option::is_none")]
	pub ttl_dbt_ntries: Option<NumberAndSumOfTransactions1>,
	#[serde(rename = "TtlNtriesPerBkTxCd", skip_serializing_if = "Option::is_none")]
	pub ttl_ntries_per_bk_tx_cd: Option<Vec<TotalsPerBankTransactionCode5>>,
}


// TotalsPerBankTransactionCode5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TotalsPerBankTransactionCode5 {
	#[serde(rename = "NbOfNtries", skip_serializing_if = "Option::is_none")]
	pub nb_of_ntries: Option<Max15NumericText>,
	#[serde(rename = "Sum", skip_serializing_if = "Option::is_none")]
	pub sum: Option<f64>,
	#[serde(rename = "TtlNetNtry", skip_serializing_if = "Option::is_none")]
	pub ttl_net_ntry: Option<AmountAndDirection35>,
	#[serde(rename = "CdtNtries", skip_serializing_if = "Option::is_none")]
	pub cdt_ntries: Option<NumberAndSumOfTransactions1>,
	#[serde(rename = "DbtNtries", skip_serializing_if = "Option::is_none")]
	pub dbt_ntries: Option<NumberAndSumOfTransactions1>,
	#[serde(rename = "FcstInd", skip_serializing_if = "Option::is_none")]
	pub fcst_ind: Option<bool>,
	#[serde(rename = "BkTxCd")]
	pub bk_tx_cd: BankTransactionCodeStructure4,
	#[serde(rename = "Avlbty", skip_serializing_if = "Option::is_none")]
	pub avlbty: Option<Vec<CashAvailability1>>,
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<DateAndDateTime2Choice>,
}


// TrackData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrackData1 {
	#[serde(rename = "TrckNb", skip_serializing_if = "Option::is_none")]
	pub trck_nb: Option<Exact1NumericText>,
	#[serde(rename = "TrckVal")]
	pub trck_val: Max140Text,
}


// TransactionAgents6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionAgents6 {
	#[serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none")]
	pub instg_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none")]
	pub instd_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none")]
	pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "RcvgAgt", skip_serializing_if = "Option::is_none")]
	pub rcvg_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "DlvrgAgt", skip_serializing_if = "Option::is_none")]
	pub dlvrg_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "IssgAgt", skip_serializing_if = "Option::is_none")]
	pub issg_agt: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "SttlmPlc", skip_serializing_if = "Option::is_none")]
	pub sttlm_plc: Option<BranchAndFinancialInstitutionIdentification8>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Vec<ProprietaryAgent5>>,
}


// TransactionChannel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TransactionChannel1Code {
	#[default]
	#[serde(rename = "MAIL")]
	CodeMAIL,
	#[serde(rename = "TLPH")]
	CodeTLPH,
	#[serde(rename = "ECOM")]
	CodeECOM,
	#[serde(rename = "TVPY")]
	CodeTVPY,

}


// TransactionDates3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionDates3 {
	#[serde(rename = "AccptncDtTm", skip_serializing_if = "Option::is_none")]
	pub accptnc_dt_tm: Option<String>,
	#[serde(rename = "TradActvtyCtrctlSttlmDt", skip_serializing_if = "Option::is_none")]
	pub trad_actvty_ctrctl_sttlm_dt: Option<String>,
	#[serde(rename = "TradDt", skip_serializing_if = "Option::is_none")]
	pub trad_dt: Option<String>,
	#[serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none")]
	pub intr_bk_sttlm_dt: Option<String>,
	#[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
	pub start_dt: Option<String>,
	#[serde(rename = "EndDt", skip_serializing_if = "Option::is_none")]
	pub end_dt: Option<String>,
	#[serde(rename = "TxDtTm", skip_serializing_if = "Option::is_none")]
	pub tx_dt_tm: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Vec<ProprietaryDate3>>,
}


// TransactionEnvironment1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TransactionEnvironment1Code {
	#[default]
	#[serde(rename = "MERC")]
	CodeMERC,
	#[serde(rename = "PRIV")]
	CodePRIV,
	#[serde(rename = "PUBL")]
	CodePUBL,

}


// TransactionIdentifier1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionIdentifier1 {
	#[serde(rename = "TxDtTm")]
	pub tx_dt_tm: String,
	#[serde(rename = "TxRef")]
	pub tx_ref: Max35Text,
}


// TransactionInterest4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionInterest4 {
	#[serde(rename = "TtlIntrstAndTaxAmt", skip_serializing_if = "Option::is_none")]
	pub ttl_intrst_and_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Rcrd", skip_serializing_if = "Option::is_none")]
	pub rcrd: Option<Vec<InterestRecord2>>,
}


// TransactionParties12 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionParties12 {
	#[serde(rename = "InitgPty", skip_serializing_if = "Option::is_none")]
	pub initg_pty: Option<Party50Choice>,
	#[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
	pub dbtr: Option<Party50Choice>,
	#[serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none")]
	pub dbtr_acct: Option<CashAccount40>,
	#[serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none")]
	pub ultmt_dbtr: Option<Party50Choice>,
	#[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
	pub cdtr: Option<Party50Choice>,
	#[serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none")]
	pub cdtr_acct: Option<CashAccount40>,
	#[serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none")]
	pub ultmt_cdtr: Option<Party50Choice>,
	#[serde(rename = "TradgPty", skip_serializing_if = "Option::is_none")]
	pub tradg_pty: Option<Party50Choice>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Vec<ProprietaryParty6>>,
}


// TransactionPrice4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionPrice4Choice {
	#[serde(rename = "DealPric", skip_serializing_if = "Option::is_none")]
	pub deal_pric: Option<Price7>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Vec<ProprietaryPrice2>>,
}


// TransactionQuantities3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionQuantities3Choice {
	#[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
	pub qty: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "OrgnlAndCurFaceAmt", skip_serializing_if = "Option::is_none")]
	pub orgnl_and_cur_face_amt: Option<OriginalAndCurrentQuantities1>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<ProprietaryQuantity1>,
}


// TransactionReferences6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionReferences6 {
	#[serde(rename = "MsgId", skip_serializing_if = "Option::is_none")]
	pub msg_id: Option<Max35Text>,
	#[serde(rename = "AcctSvcrRef", skip_serializing_if = "Option::is_none")]
	pub acct_svcr_ref: Option<Max35Text>,
	#[serde(rename = "PmtInfId", skip_serializing_if = "Option::is_none")]
	pub pmt_inf_id: Option<Max35Text>,
	#[serde(rename = "InstrId", skip_serializing_if = "Option::is_none")]
	pub instr_id: Option<Max35Text>,
	#[serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none")]
	pub end_to_end_id: Option<Max35Text>,
	#[serde(rename = "UETR", skip_serializing_if = "Option::is_none")]
	pub uetr: Option<UUIDv4Identifier>,
	#[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
	pub tx_id: Option<Max35Text>,
	#[serde(rename = "MndtId", skip_serializing_if = "Option::is_none")]
	pub mndt_id: Option<Max35Text>,
	#[serde(rename = "ChqNb", skip_serializing_if = "Option::is_none")]
	pub chq_nb: Option<Max35Text>,
	#[serde(rename = "ClrSysRef", skip_serializing_if = "Option::is_none")]
	pub clr_sys_ref: Option<Max35Text>,
	#[serde(rename = "AcctOwnrTxId", skip_serializing_if = "Option::is_none")]
	pub acct_ownr_tx_id: Option<Max35Text>,
	#[serde(rename = "AcctSvcrTxId", skip_serializing_if = "Option::is_none")]
	pub acct_svcr_tx_id: Option<Max35Text>,
	#[serde(rename = "MktInfrstrctrTxId", skip_serializing_if = "Option::is_none")]
	pub mkt_infrstrctr_tx_id: Option<Max35Text>,
	#[serde(rename = "PrcgId", skip_serializing_if = "Option::is_none")]
	pub prcg_id: Option<Max35Text>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Vec<ProprietaryReference1>>,
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}


// UUIDv4Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UUIDv4Identifier {
	#[serde(rename = "$value")]
	pub uui_dv4_identifier: String,
}


// UnitOfMeasure1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum UnitOfMeasure1Code {
	#[default]
	#[serde(rename = "PIEC")]
	CodePIEC,
	#[serde(rename = "TONS")]
	CodeTONS,
	#[serde(rename = "FOOT")]
	CodeFOOT,
	#[serde(rename = "GBGA")]
	CodeGBGA,
	#[serde(rename = "USGA")]
	CodeUSGA,
	#[serde(rename = "GRAM")]
	CodeGRAM,
	#[serde(rename = "INCH")]
	CodeINCH,
	#[serde(rename = "KILO")]
	CodeKILO,
	#[serde(rename = "PUND")]
	CodePUND,
	#[serde(rename = "METR")]
	CodeMETR,
	#[serde(rename = "CMET")]
	CodeCMET,
	#[serde(rename = "MMET")]
	CodeMMET,
	#[serde(rename = "LITR")]
	CodeLITR,
	#[serde(rename = "CELI")]
	CodeCELI,
	#[serde(rename = "MILI")]
	CodeMILI,
	#[serde(rename = "GBOU")]
	CodeGBOU,
	#[serde(rename = "USOU")]
	CodeUSOU,
	#[serde(rename = "GBQA")]
	CodeGBQA,
	#[serde(rename = "USQA")]
	CodeUSQA,
	#[serde(rename = "GBPI")]
	CodeGBPI,
	#[serde(rename = "USPI")]
	CodeUSPI,
	#[serde(rename = "MILE")]
	CodeMILE,
	#[serde(rename = "KMET")]
	CodeKMET,
	#[serde(rename = "YARD")]
	CodeYARD,
	#[serde(rename = "SQKI")]
	CodeSQKI,
	#[serde(rename = "HECT")]
	CodeHECT,
	#[serde(rename = "ARES")]
	CodeARES,
	#[serde(rename = "SMET")]
	CodeSMET,
	#[serde(rename = "SCMT")]
	CodeSCMT,
	#[serde(rename = "SMIL")]
	CodeSMIL,
	#[serde(rename = "SQMI")]
	CodeSQMI,
	#[serde(rename = "SQYA")]
	CodeSQYA,
	#[serde(rename = "SQFO")]
	CodeSQFO,
	#[serde(rename = "SQIN")]
	CodeSQIN,
	#[serde(rename = "ACRE")]
	CodeACRE,

}


// UserInterface2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum UserInterface2Code {
	#[default]
	#[serde(rename = "MDSP")]
	CodeMDSP,
	#[serde(rename = "CDSP")]
	CodeCDSP,

}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "$value")]
	pub yes_no_indicator: bool,
}


// YieldedOrValueType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YieldedOrValueType1Choice {
	#[serde(rename = "Yldd", skip_serializing_if = "Option::is_none")]
	pub yldd: Option<bool>,
	#[serde(rename = "ValTp", skip_serializing_if = "Option::is_none")]
	pub val_tp: Option<PriceValueType1Code>,
}
