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


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BaseOneRate {
	#[serde(rename = "$value")]
	pub base_one_rate: f64,
}


// BenchmarkCurveName2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BenchmarkCurveName2Code {
	#[default]
	#[serde(rename = "WIBO")]
	CodeWIBO,
	#[serde(rename = "TREA")]
	CodeTREA,
	#[serde(rename = "TIBO")]
	CodeTIBO,
	#[serde(rename = "TLBO")]
	CodeTLBO,
	#[serde(rename = "SWAP")]
	CodeSWAP,
	#[serde(rename = "STBO")]
	CodeSTBO,
	#[serde(rename = "PRBO")]
	CodePRBO,
	#[serde(rename = "PFAN")]
	CodePFAN,
	#[serde(rename = "NIBO")]
	CodeNIBO,
	#[serde(rename = "MAAA")]
	CodeMAAA,
	#[serde(rename = "MOSP")]
	CodeMOSP,
	#[serde(rename = "LIBO")]
	CodeLIBO,
	#[serde(rename = "LIBI")]
	CodeLIBI,
	#[serde(rename = "JIBA")]
	CodeJIBA,
	#[serde(rename = "ISDA")]
	CodeISDA,
	#[serde(rename = "GCFR")]
	CodeGCFR,
	#[serde(rename = "FUSW")]
	CodeFUSW,
	#[serde(rename = "EUCH")]
	CodeEUCH,
	#[serde(rename = "EUUS")]
	CodeEUUS,
	#[serde(rename = "EURI")]
	CodeEURI,
	#[serde(rename = "EONS")]
	CodeEONS,
	#[serde(rename = "EONA")]
	CodeEONA,
	#[serde(rename = "CIBO")]
	CodeCIBO,
	#[serde(rename = "CDOR")]
	CodeCDOR,
	#[serde(rename = "BUBO")]
	CodeBUBO,
	#[serde(rename = "BBSW")]
	CodeBBSW,

}


// BenchmarkCurveName4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkCurveName4Choice {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
	#[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
	pub indx: Option<BenchmarkCurveName2Code>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max25Text>,
}


// BinaryFile1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BinaryFile1 {
	#[serde(rename = "MIMETp", skip_serializing_if = "Option::is_none")]
	pub mime_tp: Option<Max35Text>,
	#[serde(rename = "NcodgTp", skip_serializing_if = "Option::is_none")]
	pub ncodg_tp: Option<Max35Text>,
	#[serde(rename = "CharSet", skip_serializing_if = "Option::is_none")]
	pub char_set: Option<Max35Text>,
	#[serde(rename = "InclBinryObjct", skip_serializing_if = "Option::is_none")]
	pub incl_binry_objct: Option<Max100KBinary>,
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


// CashCollateral5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashCollateral5 {
	#[serde(rename = "CollId", skip_serializing_if = "Option::is_none")]
	pub coll_id: Option<Max35Text>,
	#[serde(rename = "CshAcctId", skip_serializing_if = "Option::is_none")]
	pub csh_acct_id: Option<AccountIdentification4Choice>,
	#[serde(rename = "AsstNb", skip_serializing_if = "Option::is_none")]
	pub asst_nb: Option<Max35Text>,
	#[serde(rename = "DpstAmt", skip_serializing_if = "Option::is_none")]
	pub dpst_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "DpstTp", skip_serializing_if = "Option::is_none")]
	pub dpst_tp: Option<DepositType1Code>,
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
	pub val_dt: Option<String>,
	#[serde(rename = "XchgRate", skip_serializing_if = "Option::is_none")]
	pub xchg_rate: Option<f64>,
	#[serde(rename = "CollVal")]
	pub coll_val: ActiveCurrencyAndAmount,
	#[serde(rename = "Hrcut", skip_serializing_if = "Option::is_none")]
	pub hrcut: Option<f64>,
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


// CommunicationMethod4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CommunicationMethod4Code {
	#[default]
	#[serde(rename = "EMAL")]
	CodeEMAL,
	#[serde(rename = "FAXI")]
	CodeFAXI,
	#[serde(rename = "FILE")]
	CodeFILE,
	#[serde(rename = "ONLI")]
	CodeONLI,
	#[serde(rename = "PHON")]
	CodePHON,
	#[serde(rename = "POST")]
	CodePOST,
	#[serde(rename = "PROP")]
	CodePROP,
	#[serde(rename = "SWMT")]
	CodeSWMT,
	#[serde(rename = "SWMX")]
	CodeSWMX,

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


// ContractBalance1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractBalance1 {
	#[serde(rename = "Tp")]
	pub tp: ContractBalanceType1Choice,
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: CreditDebit3Code,
}


// ContractBalanceType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractBalanceType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalContractBalanceType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// ContractClosureReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractClosureReason1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalContractClosureReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// ContractCollateral1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractCollateral1 {
	#[serde(rename = "TtlAmt")]
	pub ttl_amt: ActiveCurrencyAndAmount,
	#[serde(rename = "CollDesc", skip_serializing_if = "Option::is_none")]
	pub coll_desc: Option<Vec<CashCollateral5>>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max1025Text>,
}


// ContractRegistrationConfirmationV04 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractRegistrationConfirmationV04 {
	#[serde(rename = "GrpHdr")]
	pub grp_hdr: CurrencyControlHeader7,
	#[serde(rename = "RegdCtrct")]
	pub regd_ctrct: Vec<RegisteredContract20>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// CreditDebit3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CreditDebit3Code {
	#[default]
	#[serde(rename = "CRDT")]
	CodeCRDT,
	#[serde(rename = "DBIT")]
	CodeDBIT,

}


// CurrencyControlHeader7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CurrencyControlHeader7 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
	#[serde(rename = "NbOfItms")]
	pub nb_of_itms: Max15NumericText,
	#[serde(rename = "RcvgPty")]
	pub rcvg_pty: PartyIdentification272,
	#[serde(rename = "RegnAgt")]
	pub regn_agt: BranchAndFinancialInstitutionIdentification8,
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


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "$value")]
	pub decimal_number: f64,
}


// DepositType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum DepositType1Code {
	#[default]
	#[serde(rename = "FITE")]
	CodeFITE,
	#[serde(rename = "CALL")]
	CodeCALL,

}


// DocumentGeneralInformation5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentGeneralInformation5 {
	#[serde(rename = "DocTp")]
	pub doc_tp: ExternalDocumentType1Code,
	#[serde(rename = "DocNb")]
	pub doc_nb: Max35Text,
	#[serde(rename = "DocNm", skip_serializing_if = "Option::is_none")]
	pub doc_nm: Option<Max140Text>,
	#[serde(rename = "SndrRcvrSeqId", skip_serializing_if = "Option::is_none")]
	pub sndr_rcvr_seq_id: Option<Max140Text>,
	#[serde(rename = "IsseDt", skip_serializing_if = "Option::is_none")]
	pub isse_dt: Option<String>,
	#[serde(rename = "URL", skip_serializing_if = "Option::is_none")]
	pub url: Option<Max256Text>,
	#[serde(rename = "LkFileHash", skip_serializing_if = "Option::is_none")]
	pub lk_file_hash: Option<SignatureEnvelopeReference>,
	#[serde(rename = "AttchdBinryFile")]
	pub attchd_binry_file: BinaryFile1,
}


// DocumentIdentification22 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentIdentification22 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "DtOfIsse", skip_serializing_if = "Option::is_none")]
	pub dt_of_isse: Option<String>,
}


// DocumentIdentification28 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentIdentification28 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max35Text>,
	#[serde(rename = "DtOfIsse")]
	pub dt_of_isse: String,
}


// DocumentIdentification29 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentIdentification29 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "DtOfIsse")]
	pub dt_of_isse: String,
}


// Exact1NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact1NumericText {
	#[serde(rename = "$value")]
	pub exact1_numeric_text: String,
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "$value")]
	pub exact4_alpha_numeric_text: String,
}


// ExchangeRate1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExchangeRate1 {
	#[serde(rename = "UnitCcy", skip_serializing_if = "Option::is_none")]
	pub unit_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "XchgRate", skip_serializing_if = "Option::is_none")]
	pub xchg_rate: Option<f64>,
	#[serde(rename = "RateTp", skip_serializing_if = "Option::is_none")]
	pub rate_tp: Option<ExchangeRateType1Code>,
	#[serde(rename = "CtrctId", skip_serializing_if = "Option::is_none")]
	pub ctrct_id: Option<Max35Text>,
}


// ExchangeRateType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ExchangeRateType1Code {
	#[default]
	#[serde(rename = "SPOT")]
	CodeSPOT,
	#[serde(rename = "SALE")]
	CodeSALE,
	#[serde(rename = "AGRD")]
	CodeAGRD,

}


// ExternalAccountIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAccountIdentification1Code {
	#[serde(rename = "$value")]
	pub external_account_identification1_code: String,
}


// ExternalClearingSystemIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalClearingSystemIdentification1Code {
	#[serde(rename = "$value")]
	pub external_clearing_system_identification1_code: String,
}


// ExternalContractBalanceType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalContractBalanceType1Code {
	#[serde(rename = "$value")]
	pub external_contract_balance_type1_code: String,
}


// ExternalContractClosureReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalContractClosureReason1Code {
	#[serde(rename = "$value")]
	pub external_contract_closure_reason1_code: String,
}


// ExternalDocumentType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalDocumentType1Code {
	#[serde(rename = "$value")]
	pub external_document_type1_code: String,
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


// ExternalPersonIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPersonIdentification1Code {
	#[serde(rename = "$value")]
	pub external_person_identification1_code: String,
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


// FloatingInterestRate4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FloatingInterestRate4 {
	#[serde(rename = "RefRate")]
	pub ref_rate: BenchmarkCurveName4Choice,
	#[serde(rename = "Term")]
	pub term: InterestRateContractTerm1,
	#[serde(rename = "BsisPtSprd")]
	pub bsis_pt_sprd: f64,
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


// InterestPaymentDateRange1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestPaymentDateRange1 {
	#[serde(rename = "IntrstSchdlId", skip_serializing_if = "Option::is_none")]
	pub intrst_schdl_id: Option<Max35Text>,
	#[serde(rename = "XpctdDt", skip_serializing_if = "Option::is_none")]
	pub xpctd_dt: Option<String>,
	#[serde(rename = "DueDt", skip_serializing_if = "Option::is_none")]
	pub due_dt: Option<String>,
}


// InterestPaymentSchedule1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestPaymentSchedule1 {
	#[serde(rename = "IntrstSchdlId", skip_serializing_if = "Option::is_none")]
	pub intrst_schdl_id: Option<Max35Text>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "XpctdDt", skip_serializing_if = "Option::is_none")]
	pub xpctd_dt: Option<String>,
	#[serde(rename = "DueDt", skip_serializing_if = "Option::is_none")]
	pub due_dt: Option<String>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max1025Text>,
}


// InterestRate2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRate2Choice {
	#[serde(rename = "Fxd", skip_serializing_if = "Option::is_none")]
	pub fxd: Option<f64>,
	#[serde(rename = "Fltg", skip_serializing_if = "Option::is_none")]
	pub fltg: Option<FloatingInterestRate4>,
}


// InterestRateContractTerm1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRateContractTerm1 {
	#[serde(rename = "Unit")]
	pub unit: RateBasis1Code,
	#[serde(rename = "Val")]
	pub val: f64,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}


// LegalOrganisation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LegalOrganisation2 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max35Text>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "EstblishmtDt", skip_serializing_if = "Option::is_none")]
	pub estblishmt_dt: Option<String>,
	#[serde(rename = "RegnDt", skip_serializing_if = "Option::is_none")]
	pub regn_dt: Option<String>,
}


// LoanContract4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LoanContract4 {
	#[serde(rename = "CtrctDocId")]
	pub ctrct_doc_id: DocumentIdentification22,
	#[serde(rename = "LnTpId", skip_serializing_if = "Option::is_none")]
	pub ln_tp_id: Option<Max35Text>,
	#[serde(rename = "Buyr")]
	pub buyr: Vec<TradeParty6>,
	#[serde(rename = "Sellr")]
	pub sellr: Vec<TradeParty6>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "PrlngtnFlg", skip_serializing_if = "Option::is_none")]
	pub prlngtn_flg: Option<bool>,
	#[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
	pub start_dt: Option<String>,
	#[serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none")]
	pub sttlm_ccy: Option<ActiveCurrencyCode>,
	#[serde(rename = "SpclConds", skip_serializing_if = "Option::is_none")]
	pub spcl_conds: Option<SpecialCondition1>,
	#[serde(rename = "DrtnCd", skip_serializing_if = "Option::is_none")]
	pub drtn_cd: Option<Exact1NumericText>,
	#[serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none")]
	pub intrst_rate: Option<InterestRate2Choice>,
	#[serde(rename = "Trch", skip_serializing_if = "Option::is_none")]
	pub trch: Option<Vec<LoanContractTranche1>>,
	#[serde(rename = "PmtSchdl", skip_serializing_if = "Option::is_none")]
	pub pmt_schdl: Option<Vec<PaymentSchedule1>>,
	#[serde(rename = "IntrstSchdl", skip_serializing_if = "Option::is_none")]
	pub intrst_schdl: Option<Vec<InterestPaymentSchedule1>>,
	#[serde(rename = "IntraCpnyLn")]
	pub intra_cpny_ln: bool,
	#[serde(rename = "Coll", skip_serializing_if = "Option::is_none")]
	pub coll: Option<ContractCollateral1>,
	#[serde(rename = "SndctdLn", skip_serializing_if = "Option::is_none")]
	pub sndctd_ln: Option<Vec<SyndicatedLoan3>>,
	#[serde(rename = "Attchmnt", skip_serializing_if = "Option::is_none")]
	pub attchmnt: Option<Vec<DocumentGeneralInformation5>>,
}


// LoanContractTranche1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LoanContractTranche1 {
	#[serde(rename = "TrchNb")]
	pub trch_nb: f64,
	#[serde(rename = "XpctdDt")]
	pub xpctd_dt: String,
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
	#[serde(rename = "DueDt", skip_serializing_if = "Option::is_none")]
	pub due_dt: Option<String>,
	#[serde(rename = "DrtnCd", skip_serializing_if = "Option::is_none")]
	pub drtn_cd: Option<Exact1NumericText>,
	#[serde(rename = "LastTrchInd", skip_serializing_if = "Option::is_none")]
	pub last_trch_ind: Option<bool>,
}


// Max100KBinary ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max100KBinary {
	#[serde(rename = "$value")]
	pub max100_k_binary: String,
}


// Max1025Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max1025Text {
	#[serde(rename = "$value")]
	pub max1025_text: String,
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


// Max25Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max25Text {
	#[serde(rename = "$value")]
	pub max25_text: String,
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


// Max4Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4Text {
	#[serde(rename = "$value")]
	pub max4_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
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


// PaymentSchedule1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentSchedule1 {
	#[serde(rename = "PmtSchdlId", skip_serializing_if = "Option::is_none")]
	pub pmt_schdl_id: Option<Max35Text>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "XpctdDt", skip_serializing_if = "Option::is_none")]
	pub xpctd_dt: Option<String>,
	#[serde(rename = "DueDt", skip_serializing_if = "Option::is_none")]
	pub due_dt: Option<String>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max1025Text>,
}


// PaymentScheduleType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentScheduleType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PaymentScheduleType2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// PaymentScheduleType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PaymentScheduleType2Code {
	#[default]
	#[serde(rename = "CNTR")]
	CodeCNTR,
	#[serde(rename = "ESTM")]
	CodeESTM,
	#[serde(rename = "BOTH")]
	CodeBOTH,

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


// RegisteredContract20 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegisteredContract20 {
	#[serde(rename = "OrgnlCtrctRegnReq", skip_serializing_if = "Option::is_none")]
	pub orgnl_ctrct_regn_req: Option<Max35Text>,
	#[serde(rename = "RptgPty")]
	pub rptg_pty: TradeParty6,
	#[serde(rename = "RegnAgt")]
	pub regn_agt: BranchAndFinancialInstitutionIdentification8,
	#[serde(rename = "IssrFI")]
	pub issr_fi: BranchAndFinancialInstitutionIdentification8,
	#[serde(rename = "Ctrct")]
	pub ctrct: UnderlyingContract4Choice,
	#[serde(rename = "CtrctBal", skip_serializing_if = "Option::is_none")]
	pub ctrct_bal: Option<Vec<ContractBalance1>>,
	#[serde(rename = "PmtSchdlTp", skip_serializing_if = "Option::is_none")]
	pub pmt_schdl_tp: Option<PaymentScheduleType2Choice>,
	#[serde(rename = "RegdCtrctId")]
	pub regd_ctrct_id: DocumentIdentification29,
	#[serde(rename = "PrvsRegdCtrctId", skip_serializing_if = "Option::is_none")]
	pub prvs_regd_ctrct_id: Option<DocumentIdentification22>,
	#[serde(rename = "RegdCtrctJrnl", skip_serializing_if = "Option::is_none")]
	pub regd_ctrct_jrnl: Option<Vec<RegisteredContractJournal3>>,
	#[serde(rename = "Amdmnt", skip_serializing_if = "Option::is_none")]
	pub amdmnt: Option<Vec<RegisteredContractAmendment1>>,
	#[serde(rename = "Submissn")]
	pub submissn: RegisteredContractCommunication1,
	#[serde(rename = "Dlvry")]
	pub dlvry: RegisteredContractCommunication1,
	#[serde(rename = "LnPrncplAmt", skip_serializing_if = "Option::is_none")]
	pub ln_prncpl_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "EstmtdDtInd")]
	pub estmtd_dt_ind: bool,
	#[serde(rename = "IntrCpnyLn")]
	pub intr_cpny_ln: bool,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max1025Text>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// RegisteredContractAmendment1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegisteredContractAmendment1 {
	#[serde(rename = "AmdmntDt")]
	pub amdmnt_dt: String,
	#[serde(rename = "Doc")]
	pub doc: DocumentIdentification28,
	#[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
	pub start_dt: Option<String>,
	#[serde(rename = "AmdmntRsn", skip_serializing_if = "Option::is_none")]
	pub amdmnt_rsn: Option<Max35Text>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max1025Text>,
}


// RegisteredContractCommunication1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegisteredContractCommunication1 {
	#[serde(rename = "Mtd")]
	pub mtd: CommunicationMethod4Code,
	#[serde(rename = "Dt")]
	pub dt: String,
}


// RegisteredContractJournal3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegisteredContractJournal3 {
	#[serde(rename = "RegnAgt")]
	pub regn_agt: BranchAndFinancialInstitutionIdentification8,
	#[serde(rename = "UnqId", skip_serializing_if = "Option::is_none")]
	pub unq_id: Option<DocumentIdentification28>,
	#[serde(rename = "ClsrDt")]
	pub clsr_dt: String,
	#[serde(rename = "ClsrRsn")]
	pub clsr_rsn: ContractClosureReason1Choice,
}


// ShipmentDateRange1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ShipmentDateRange1 {
	#[serde(rename = "EarlstShipmntDt", skip_serializing_if = "Option::is_none")]
	pub earlst_shipmnt_dt: Option<String>,
	#[serde(rename = "LatstShipmntDt", skip_serializing_if = "Option::is_none")]
	pub latst_shipmnt_dt: Option<String>,
}


// ShipmentDateRange2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ShipmentDateRange2 {
	#[serde(rename = "SubQtyVal")]
	pub sub_qty_val: f64,
	#[serde(rename = "EarlstShipmntDt", skip_serializing_if = "Option::is_none")]
	pub earlst_shipmnt_dt: Option<String>,
	#[serde(rename = "LatstShipmntDt", skip_serializing_if = "Option::is_none")]
	pub latst_shipmnt_dt: Option<String>,
}


// ShipmentSchedule2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ShipmentSchedule2Choice {
	#[serde(rename = "ShipmntDtRg", skip_serializing_if = "Option::is_none")]
	pub shipmnt_dt_rg: Option<ShipmentDateRange1>,
	#[serde(rename = "ShipmntSubSchdl", skip_serializing_if = "Option::is_none")]
	pub shipmnt_sub_schdl: Option<Vec<ShipmentDateRange2>>,
}


// SignatureEnvelopeReference ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SignatureEnvelopeReference {
}


// SpecialCondition1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SpecialCondition1 {
	#[serde(rename = "IncmgAmt")]
	pub incmg_amt: ActiveCurrencyAndAmount,
	#[serde(rename = "OutgngAmt")]
	pub outgng_amt: ActiveCurrencyAndAmount,
	#[serde(rename = "IncmgAmtToOthrAcct", skip_serializing_if = "Option::is_none")]
	pub incmg_amt_to_othr_acct: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "PmtFrOthrAcct", skip_serializing_if = "Option::is_none")]
	pub pmt_fr_othr_acct: Option<ActiveCurrencyAndAmount>,
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


// SyndicatedLoan3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SyndicatedLoan3 {
	#[serde(rename = "Brrwr")]
	pub brrwr: TradeParty6,
	#[serde(rename = "Lndr", skip_serializing_if = "Option::is_none")]
	pub lndr: Option<TradeParty6>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Shr", skip_serializing_if = "Option::is_none")]
	pub shr: Option<f64>,
	#[serde(rename = "XchgRateInf", skip_serializing_if = "Option::is_none")]
	pub xchg_rate_inf: Option<ExchangeRate1>,
}


// TaxExemptReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TaxExemptReason1Code {
	#[default]
	#[serde(rename = "NONE")]
	CodeNONE,
	#[serde(rename = "MASA")]
	CodeMASA,
	#[serde(rename = "MISA")]
	CodeMISA,
	#[serde(rename = "SISA")]
	CodeSISA,
	#[serde(rename = "IISA")]
	CodeIISA,
	#[serde(rename = "CUYP")]
	CodeCUYP,
	#[serde(rename = "PRYP")]
	CodePRYP,
	#[serde(rename = "ASTR")]
	CodeASTR,
	#[serde(rename = "EMPY")]
	CodeEMPY,
	#[serde(rename = "EMCY")]
	CodeEMCY,
	#[serde(rename = "EPRY")]
	CodeEPRY,
	#[serde(rename = "ECYE")]
	CodeECYE,
	#[serde(rename = "NFPI")]
	CodeNFPI,
	#[serde(rename = "NFQP")]
	CodeNFQP,
	#[serde(rename = "DECP")]
	CodeDECP,
	#[serde(rename = "IRAC")]
	CodeIRAC,
	#[serde(rename = "IRAR")]
	CodeIRAR,
	#[serde(rename = "KEOG")]
	CodeKEOG,
	#[serde(rename = "PFSP")]
	CodePFSP,
	#[serde(rename = "401K")]
	Code401K,
	#[serde(rename = "SIRA")]
	CodeSIRA,
	#[serde(rename = "403B")]
	Code403B,
	#[serde(rename = "457X")]
	Code457X,
	#[serde(rename = "RIRA")]
	CodeRIRA,
	#[serde(rename = "RIAN")]
	CodeRIAN,
	#[serde(rename = "RCRF")]
	CodeRCRF,
	#[serde(rename = "RCIP")]
	CodeRCIP,
	#[serde(rename = "EIFP")]
	CodeEIFP,
	#[serde(rename = "EIOP")]
	CodeEIOP,

}


// TaxExemptionReasonFormat1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxExemptionReasonFormat1Choice {
	#[serde(rename = "Ustrd", skip_serializing_if = "Option::is_none")]
	pub ustrd: Option<Max140Text>,
	#[serde(rename = "Strd", skip_serializing_if = "Option::is_none")]
	pub strd: Option<TaxExemptReason1Code>,
}


// TaxParty4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxParty4 {
	#[serde(rename = "TaxId", skip_serializing_if = "Option::is_none")]
	pub tax_id: Option<Max35Text>,
	#[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
	pub tax_tp: Option<Max35Text>,
	#[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
	pub regn_id: Option<Max35Text>,
	#[serde(rename = "TaxXmptnRsn", skip_serializing_if = "Option::is_none")]
	pub tax_xmptn_rsn: Option<Vec<TaxExemptionReasonFormat1Choice>>,
}


// TradeContract4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeContract4 {
	#[serde(rename = "CtrctDocId", skip_serializing_if = "Option::is_none")]
	pub ctrct_doc_id: Option<DocumentIdentification22>,
	#[serde(rename = "TradTpId", skip_serializing_if = "Option::is_none")]
	pub trad_tp_id: Option<Max35Text>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Buyr")]
	pub buyr: Vec<TradeParty6>,
	#[serde(rename = "Sellr")]
	pub sellr: Vec<TradeParty6>,
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "PrlngtnFlg", skip_serializing_if = "Option::is_none")]
	pub prlngtn_flg: Option<bool>,
	#[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
	pub start_dt: Option<String>,
	#[serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none")]
	pub sttlm_ccy: Option<ActiveCurrencyCode>,
	#[serde(rename = "XchgRateInf", skip_serializing_if = "Option::is_none")]
	pub xchg_rate_inf: Option<ExchangeRate1>,
	#[serde(rename = "PmtSchdl", skip_serializing_if = "Option::is_none")]
	pub pmt_schdl: Option<InterestPaymentDateRange1>,
	#[serde(rename = "ShipmntSchdl", skip_serializing_if = "Option::is_none")]
	pub shipmnt_schdl: Option<ShipmentSchedule2Choice>,
	#[serde(rename = "Attchmnt", skip_serializing_if = "Option::is_none")]
	pub attchmnt: Option<Vec<DocumentGeneralInformation5>>,
}


// TradeParty6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeParty6 {
	#[serde(rename = "PtyId")]
	pub pty_id: PartyIdentification272,
	#[serde(rename = "LglOrg", skip_serializing_if = "Option::is_none")]
	pub lgl_org: Option<LegalOrganisation2>,
	#[serde(rename = "TaxPty", skip_serializing_if = "Option::is_none")]
	pub tax_pty: Option<Vec<TaxParty4>>,
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}


// UnderlyingContract4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingContract4Choice {
	#[serde(rename = "Ln", skip_serializing_if = "Option::is_none")]
	pub ln: Option<LoanContract4>,
	#[serde(rename = "Trad", skip_serializing_if = "Option::is_none")]
	pub trad: Option<TradeContract4>,
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "$value")]
	pub yes_no_indicator: bool,
}
