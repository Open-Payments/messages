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


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BaseOneRate {
	#[serde(rename = "BaseOneRate")]
	pub base_one_rate: f64,
}


// BenchmarkCurveName2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkCurveName2Code {
	#[serde(rename = "BenchmarkCurveName2Code")]
	pub benchmark_curve_name2_code: String,
}


// BenchmarkCurveName4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkCurveName4Choice {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[serde(rename = "Indx")]
	pub indx: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
}


// BinaryFile1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BinaryFile1 {
	#[serde(rename = "MIMETp")]
	pub mime_tp: Option<String>,
	#[serde(rename = "NcodgTp")]
	pub ncodg_tp: Option<String>,
	#[serde(rename = "CharSet")]
	pub char_set: Option<String>,
	#[serde(rename = "InclBinryObjct")]
	pub incl_binry_objct: Option<String>,
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


// CashCollateral5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashCollateral5 {
	#[serde(rename = "CollId")]
	pub coll_id: Option<String>,
	#[serde(rename = "CshAcctId")]
	pub csh_acct_id: Option<AccountIdentification4Choice>,
	#[serde(rename = "AsstNb")]
	pub asst_nb: Option<String>,
	#[serde(rename = "DpstAmt")]
	pub dpst_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "DpstTp")]
	pub dpst_tp: Option<String>,
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "ValDt")]
	pub val_dt: Option<String>,
	#[serde(rename = "XchgRate")]
	pub xchg_rate: Option<f64>,
	#[serde(rename = "CollVal")]
	pub coll_val: ActiveCurrencyAndAmount,
	#[serde(rename = "Hrcut")]
	pub hrcut: Option<f64>,
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


// CommunicationMethod4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommunicationMethod4Code {
	#[serde(rename = "CommunicationMethod4Code")]
	pub communication_method4_code: String,
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


// ContractBalance1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractBalance1 {
	#[serde(rename = "Tp")]
	pub tp: ContractBalanceType1Choice,
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
	#[serde(rename = "CdtDbtInd")]
	pub cdt_dbt_ind: String,
}


// ContractBalanceType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractBalanceType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ContractClosureReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractClosureReason1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ContractCollateral1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractCollateral1 {
	#[serde(rename = "TtlAmt")]
	pub ttl_amt: ActiveCurrencyAndAmount,
	#[serde(rename = "CollDesc")]
	pub coll_desc: Option<Vec<CashCollateral5>>,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<String>,
}


// ContractRegistrationConfirmationV04 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractRegistrationConfirmationV04 {
	#[serde(rename = "GrpHdr")]
	pub grp_hdr: CurrencyControlHeader7,
	#[serde(rename = "RegdCtrct")]
	pub regd_ctrct: Vec<RegisteredContract20>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// CreditDebit3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditDebit3Code {
	#[serde(rename = "CreditDebit3Code")]
	pub credit_debit3_code: String,
}


// CurrencyControlHeader7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CurrencyControlHeader7 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
	#[serde(rename = "NbOfItms")]
	pub nb_of_itms: String,
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
	#[serde(rename = "PrvcOfBirth")]
	pub prvc_of_birth: Option<String>,
	#[serde(rename = "CityOfBirth")]
	pub city_of_birth: String,
	#[serde(rename = "CtryOfBirth")]
	pub ctry_of_birth: String,
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "DecimalNumber")]
	pub decimal_number: f64,
}


// DepositType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DepositType1Code {
	#[serde(rename = "DepositType1Code")]
	pub deposit_type1_code: String,
}


// DocumentGeneralInformation5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentGeneralInformation5 {
	#[serde(rename = "DocTp")]
	pub doc_tp: String,
	#[serde(rename = "DocNb")]
	pub doc_nb: String,
	#[serde(rename = "DocNm")]
	pub doc_nm: Option<String>,
	#[serde(rename = "SndrRcvrSeqId")]
	pub sndr_rcvr_seq_id: Option<String>,
	#[serde(rename = "IsseDt")]
	pub isse_dt: Option<String>,
	#[serde(rename = "URL")]
	pub url: Option<String>,
	#[serde(rename = "LkFileHash")]
	pub lk_file_hash: Option<SignatureEnvelopeReference>,
	#[serde(rename = "AttchdBinryFile")]
	pub attchd_binry_file: BinaryFile1,
}


// DocumentIdentification22 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentIdentification22 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "DtOfIsse")]
	pub dt_of_isse: Option<String>,
}


// DocumentIdentification28 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentIdentification28 {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "DtOfIsse")]
	pub dt_of_isse: String,
}


// DocumentIdentification29 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentIdentification29 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "DtOfIsse")]
	pub dt_of_isse: String,
}


// Exact1NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact1NumericText {
	#[serde(rename = "Exact1NumericText")]
	pub exact1_numeric_text: String,
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "Exact4AlphaNumericText")]
	pub exact4_alpha_numeric_text: String,
}


// ExchangeRate1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExchangeRate1 {
	#[serde(rename = "UnitCcy")]
	pub unit_ccy: Option<String>,
	#[serde(rename = "XchgRate")]
	pub xchg_rate: Option<f64>,
	#[serde(rename = "RateTp")]
	pub rate_tp: Option<String>,
	#[serde(rename = "CtrctId")]
	pub ctrct_id: Option<String>,
}


// ExchangeRateType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExchangeRateType1Code {
	#[serde(rename = "ExchangeRateType1Code")]
	pub exchange_rate_type1_code: String,
}


// ExternalAccountIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAccountIdentification1Code {
	#[serde(rename = "ExternalAccountIdentification1Code")]
	pub external_account_identification1_code: String,
}


// ExternalClearingSystemIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalClearingSystemIdentification1Code {
	#[serde(rename = "ExternalClearingSystemIdentification1Code")]
	pub external_clearing_system_identification1_code: String,
}


// ExternalContractBalanceType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalContractBalanceType1Code {
	#[serde(rename = "ExternalContractBalanceType1Code")]
	pub external_contract_balance_type1_code: String,
}


// ExternalContractClosureReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalContractClosureReason1Code {
	#[serde(rename = "ExternalContractClosureReason1Code")]
	pub external_contract_closure_reason1_code: String,
}


// ExternalDocumentType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalDocumentType1Code {
	#[serde(rename = "ExternalDocumentType1Code")]
	pub external_document_type1_code: String,
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


// ExternalPersonIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPersonIdentification1Code {
	#[serde(rename = "ExternalPersonIdentification1Code")]
	pub external_person_identification1_code: String,
}


// FinancialIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
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


// GenericPersonIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericPersonIdentification2 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
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


// InterestPaymentDateRange1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestPaymentDateRange1 {
	#[serde(rename = "IntrstSchdlId")]
	pub intrst_schdl_id: Option<String>,
	#[serde(rename = "XpctdDt")]
	pub xpctd_dt: Option<String>,
	#[serde(rename = "DueDt")]
	pub due_dt: Option<String>,
}


// InterestPaymentSchedule1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestPaymentSchedule1 {
	#[serde(rename = "IntrstSchdlId")]
	pub intrst_schdl_id: Option<String>,
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "XpctdDt")]
	pub xpctd_dt: Option<String>,
	#[serde(rename = "DueDt")]
	pub due_dt: Option<String>,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<String>,
}


// InterestRate2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRate2Choice {
	#[serde(rename = "Fxd")]
	pub fxd: Option<f64>,
	#[serde(rename = "Fltg")]
	pub fltg: Option<FloatingInterestRate4>,
}


// InterestRateContractTerm1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRateContractTerm1 {
	#[serde(rename = "Unit")]
	pub unit: String,
	#[serde(rename = "Val")]
	pub val: f64,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LegalOrganisation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LegalOrganisation2 {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "EstblishmtDt")]
	pub estblishmt_dt: Option<String>,
	#[serde(rename = "RegnDt")]
	pub regn_dt: Option<String>,
}


// LoanContract4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LoanContract4 {
	#[serde(rename = "CtrctDocId")]
	pub ctrct_doc_id: DocumentIdentification22,
	#[serde(rename = "LnTpId")]
	pub ln_tp_id: Option<String>,
	#[serde(rename = "Buyr")]
	pub buyr: Vec<TradeParty6>,
	#[serde(rename = "Sellr")]
	pub sellr: Vec<TradeParty6>,
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "PrlngtnFlg")]
	pub prlngtn_flg: Option<bool>,
	#[serde(rename = "StartDt")]
	pub start_dt: Option<String>,
	#[serde(rename = "SttlmCcy")]
	pub sttlm_ccy: Option<String>,
	#[serde(rename = "SpclConds")]
	pub spcl_conds: Option<SpecialCondition1>,
	#[serde(rename = "DrtnCd")]
	pub drtn_cd: Option<String>,
	#[serde(rename = "IntrstRate")]
	pub intrst_rate: Option<InterestRate2Choice>,
	#[serde(rename = "Trch")]
	pub trch: Option<Vec<LoanContractTranche1>>,
	#[serde(rename = "PmtSchdl")]
	pub pmt_schdl: Option<Vec<PaymentSchedule1>>,
	#[serde(rename = "IntrstSchdl")]
	pub intrst_schdl: Option<Vec<InterestPaymentSchedule1>>,
	#[serde(rename = "IntraCpnyLn")]
	pub intra_cpny_ln: bool,
	#[serde(rename = "Coll")]
	pub coll: Option<ContractCollateral1>,
	#[serde(rename = "SndctdLn")]
	pub sndctd_ln: Option<Vec<SyndicatedLoan3>>,
	#[serde(rename = "Attchmnt")]
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
	#[serde(rename = "DueDt")]
	pub due_dt: Option<String>,
	#[serde(rename = "DrtnCd")]
	pub drtn_cd: Option<String>,
	#[serde(rename = "LastTrchInd")]
	pub last_trch_ind: Option<bool>,
}


// Max100KBinary ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max100KBinary {
	#[serde(rename = "Max100KBinary")]
	pub max100_k_binary: String,
}


// Max1025Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max1025Text {
	#[serde(rename = "Max1025Text")]
	pub max1025_text: String,
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


// Max256Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max256Text {
	#[serde(rename = "Max256Text")]
	pub max256_text: String,
}


// Max25Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max25Text {
	#[serde(rename = "Max25Text")]
	pub max25_text: String,
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


// Max4Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4Text {
	#[serde(rename = "Max4Text")]
	pub max4_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
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


// Party52Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party52Choice {
	#[serde(rename = "OrgId")]
	pub org_id: Option<OrganisationIdentification39>,
	#[serde(rename = "PrvtId")]
	pub prvt_id: Option<PersonIdentification18>,
}


// PartyIdentification272 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification272 {
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress27>,
	#[serde(rename = "Id")]
	pub id: Option<Party52Choice>,
	#[serde(rename = "CtryOfRes")]
	pub ctry_of_res: Option<String>,
	#[serde(rename = "CtctDtls")]
	pub ctct_dtls: Option<Contact13>,
}


// PaymentSchedule1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentSchedule1 {
	#[serde(rename = "PmtSchdlId")]
	pub pmt_schdl_id: Option<String>,
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "XpctdDt")]
	pub xpctd_dt: Option<String>,
	#[serde(rename = "DueDt")]
	pub due_dt: Option<String>,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<String>,
}


// PaymentScheduleType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentScheduleType2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// PaymentScheduleType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentScheduleType2Code {
	#[serde(rename = "PaymentScheduleType2Code")]
	pub payment_schedule_type2_code: String,
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// PersonIdentification18 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentification18 {
	#[serde(rename = "DtAndPlcOfBirth")]
	pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<GenericPersonIdentification2>>,
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


// RateBasis1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RateBasis1Code {
	#[serde(rename = "RateBasis1Code")]
	pub rate_basis1_code: String,
}


// RegisteredContract20 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegisteredContract20 {
	#[serde(rename = "OrgnlCtrctRegnReq")]
	pub orgnl_ctrct_regn_req: Option<String>,
	#[serde(rename = "RptgPty")]
	pub rptg_pty: TradeParty6,
	#[serde(rename = "RegnAgt")]
	pub regn_agt: BranchAndFinancialInstitutionIdentification8,
	#[serde(rename = "IssrFI")]
	pub issr_fi: BranchAndFinancialInstitutionIdentification8,
	#[serde(rename = "Ctrct")]
	pub ctrct: UnderlyingContract4Choice,
	#[serde(rename = "CtrctBal")]
	pub ctrct_bal: Option<Vec<ContractBalance1>>,
	#[serde(rename = "PmtSchdlTp")]
	pub pmt_schdl_tp: Option<PaymentScheduleType2Choice>,
	#[serde(rename = "RegdCtrctId")]
	pub regd_ctrct_id: DocumentIdentification29,
	#[serde(rename = "PrvsRegdCtrctId")]
	pub prvs_regd_ctrct_id: Option<DocumentIdentification22>,
	#[serde(rename = "RegdCtrctJrnl")]
	pub regd_ctrct_jrnl: Option<Vec<RegisteredContractJournal3>>,
	#[serde(rename = "Amdmnt")]
	pub amdmnt: Option<Vec<RegisteredContractAmendment1>>,
	#[serde(rename = "Submissn")]
	pub submissn: RegisteredContractCommunication1,
	#[serde(rename = "Dlvry")]
	pub dlvry: RegisteredContractCommunication1,
	#[serde(rename = "LnPrncplAmt")]
	pub ln_prncpl_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "EstmtdDtInd")]
	pub estmtd_dt_ind: bool,
	#[serde(rename = "IntrCpnyLn")]
	pub intr_cpny_ln: bool,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<String>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// RegisteredContractAmendment1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegisteredContractAmendment1 {
	#[serde(rename = "AmdmntDt")]
	pub amdmnt_dt: String,
	#[serde(rename = "Doc")]
	pub doc: DocumentIdentification28,
	#[serde(rename = "StartDt")]
	pub start_dt: Option<String>,
	#[serde(rename = "AmdmntRsn")]
	pub amdmnt_rsn: Option<String>,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<String>,
}


// RegisteredContractCommunication1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegisteredContractCommunication1 {
	#[serde(rename = "Mtd")]
	pub mtd: String,
	#[serde(rename = "Dt")]
	pub dt: String,
}


// RegisteredContractJournal3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegisteredContractJournal3 {
	#[serde(rename = "RegnAgt")]
	pub regn_agt: BranchAndFinancialInstitutionIdentification8,
	#[serde(rename = "UnqId")]
	pub unq_id: Option<DocumentIdentification28>,
	#[serde(rename = "ClsrDt")]
	pub clsr_dt: String,
	#[serde(rename = "ClsrRsn")]
	pub clsr_rsn: ContractClosureReason1Choice,
}


// ShipmentDateRange1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ShipmentDateRange1 {
	#[serde(rename = "EarlstShipmntDt")]
	pub earlst_shipmnt_dt: Option<String>,
	#[serde(rename = "LatstShipmntDt")]
	pub latst_shipmnt_dt: Option<String>,
}


// ShipmentDateRange2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ShipmentDateRange2 {
	#[serde(rename = "SubQtyVal")]
	pub sub_qty_val: f64,
	#[serde(rename = "EarlstShipmntDt")]
	pub earlst_shipmnt_dt: Option<String>,
	#[serde(rename = "LatstShipmntDt")]
	pub latst_shipmnt_dt: Option<String>,
}


// ShipmentSchedule2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ShipmentSchedule2Choice {
	#[serde(rename = "ShipmntDtRg")]
	pub shipmnt_dt_rg: Option<ShipmentDateRange1>,
	#[serde(rename = "ShipmntSubSchdl")]
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
	#[serde(rename = "IncmgAmtToOthrAcct")]
	pub incmg_amt_to_othr_acct: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "PmtFrOthrAcct")]
	pub pmt_fr_othr_acct: Option<ActiveCurrencyAndAmount>,
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


// SyndicatedLoan3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SyndicatedLoan3 {
	#[serde(rename = "Brrwr")]
	pub brrwr: TradeParty6,
	#[serde(rename = "Lndr")]
	pub lndr: Option<TradeParty6>,
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Shr")]
	pub shr: Option<f64>,
	#[serde(rename = "XchgRateInf")]
	pub xchg_rate_inf: Option<ExchangeRate1>,
}


// TaxExemptReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxExemptReason1Code {
	#[serde(rename = "TaxExemptReason1Code")]
	pub tax_exempt_reason1_code: String,
}


// TaxExemptionReasonFormat1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxExemptionReasonFormat1Choice {
	#[serde(rename = "Ustrd")]
	pub ustrd: Option<String>,
	#[serde(rename = "Strd")]
	pub strd: Option<String>,
}


// TaxParty4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxParty4 {
	#[serde(rename = "TaxId")]
	pub tax_id: Option<String>,
	#[serde(rename = "TaxTp")]
	pub tax_tp: Option<String>,
	#[serde(rename = "RegnId")]
	pub regn_id: Option<String>,
	#[serde(rename = "TaxXmptnRsn")]
	pub tax_xmptn_rsn: Option<Vec<TaxExemptionReasonFormat1Choice>>,
}


// TradeContract4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeContract4 {
	#[serde(rename = "CtrctDocId")]
	pub ctrct_doc_id: Option<DocumentIdentification22>,
	#[serde(rename = "TradTpId")]
	pub trad_tp_id: Option<String>,
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Buyr")]
	pub buyr: Vec<TradeParty6>,
	#[serde(rename = "Sellr")]
	pub sellr: Vec<TradeParty6>,
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "PrlngtnFlg")]
	pub prlngtn_flg: Option<bool>,
	#[serde(rename = "StartDt")]
	pub start_dt: Option<String>,
	#[serde(rename = "SttlmCcy")]
	pub sttlm_ccy: Option<String>,
	#[serde(rename = "XchgRateInf")]
	pub xchg_rate_inf: Option<ExchangeRate1>,
	#[serde(rename = "PmtSchdl")]
	pub pmt_schdl: Option<InterestPaymentDateRange1>,
	#[serde(rename = "ShipmntSchdl")]
	pub shipmnt_schdl: Option<ShipmentSchedule2Choice>,
	#[serde(rename = "Attchmnt")]
	pub attchmnt: Option<Vec<DocumentGeneralInformation5>>,
}


// TradeParty6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeParty6 {
	#[serde(rename = "PtyId")]
	pub pty_id: PartyIdentification272,
	#[serde(rename = "LglOrg")]
	pub lgl_org: Option<LegalOrganisation2>,
	#[serde(rename = "TaxPty")]
	pub tax_pty: Option<Vec<TaxParty4>>,
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}


// UnderlyingContract4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingContract4Choice {
	#[serde(rename = "Ln")]
	pub ln: Option<LoanContract4>,
	#[serde(rename = "Trad")]
	pub trad: Option<TradeContract4>,
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "YesNoIndicator")]
	pub yes_no_indicator: bool,
}
