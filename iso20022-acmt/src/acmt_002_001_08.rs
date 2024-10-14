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


// Account23 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Account23 {
	#[serde(rename = "AcctId")]
	pub acct_id: Max35Text,
	#[serde(rename = "RltdAcctDtls", skip_serializing_if = "Option::is_none")]
	pub rltd_acct_dtls: Option<GenericIdentification1>,
}


// Account32 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Account32 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max35Text>,
	#[serde(rename = "AcctSvcr")]
	pub acct_svcr: PartyIdentification125Choice,
}


// AccountDesignation1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountDesignation1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Rank1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// AccountDetailsConfirmationV08 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountDetailsConfirmationV08 {
	#[serde(rename = "MsgId")]
	pub msg_id: MessageIdentification1,
	#[serde(rename = "OrdrRef", skip_serializing_if = "Option::is_none")]
	pub ordr_ref: Option<InvestmentFundOrder4>,
	#[serde(rename = "RltdRef", skip_serializing_if = "Option::is_none")]
	pub rltd_ref: Option<AdditionalReference13>,
	#[serde(rename = "ConfDtls")]
	pub conf_dtls: AccountManagementConfirmation5,
	#[serde(rename = "InvstmtAcct", skip_serializing_if = "Option::is_none")]
	pub invstmt_acct: Option<InvestmentAccount74>,
	#[serde(rename = "AcctPties", skip_serializing_if = "Option::is_none")]
	pub acct_pties: Option<AccountParties17>,
	#[serde(rename = "Intrmies", skip_serializing_if = "Option::is_none")]
	pub intrmies: Option<Vec<Intermediary46>>,
	#[serde(rename = "Plcmnt", skip_serializing_if = "Option::is_none")]
	pub plcmnt: Option<ReferredAgent3>,
	#[serde(rename = "NewIsseAllcn", skip_serializing_if = "Option::is_none")]
	pub new_isse_allcn: Option<NewIssueAllocation2>,
	#[serde(rename = "SvgsInvstmtPlan", skip_serializing_if = "Option::is_none")]
	pub svgs_invstmt_plan: Option<Vec<InvestmentPlan17>>,
	#[serde(rename = "WdrwlInvstmtPlan", skip_serializing_if = "Option::is_none")]
	pub wdrwl_invstmt_plan: Option<Vec<InvestmentPlan17>>,
	#[serde(rename = "CshSttlm", skip_serializing_if = "Option::is_none")]
	pub csh_sttlm: Option<Vec<CashSettlement3>>,
	#[serde(rename = "SvcLvlAgrmt", skip_serializing_if = "Option::is_none")]
	pub svc_lvl_agrmt: Option<Vec<DocumentToSend4>>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<AdditiononalInformation13>>,
	#[serde(rename = "MktPrctcVrsn", skip_serializing_if = "Option::is_none")]
	pub mkt_prctc_vrsn: Option<MarketPracticeVersion1>,
	#[serde(rename = "Xtnsn", skip_serializing_if = "Option::is_none")]
	pub xtnsn: Option<Vec<Extension1>>,
}


// AccountIdentification4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountIdentification4Choice {
	#[serde(rename = "IBAN", skip_serializing_if = "Option::is_none")]
	pub iban: Option<IBAN2007Identifier>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<GenericAccountIdentification1>,
}


// AccountIdentificationAndName5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountIdentificationAndName5 {
	#[serde(rename = "Id")]
	pub id: AccountIdentification4Choice,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max35Text>,
}


// AccountManagementConfirmation5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountManagementConfirmation5 {
	#[serde(rename = "ConfTp")]
	pub conf_tp: ConfirmationType1Choice,
	#[serde(rename = "AcctApplId", skip_serializing_if = "Option::is_none")]
	pub acct_appl_id: Option<Max35Text>,
	#[serde(rename = "ClntRef", skip_serializing_if = "Option::is_none")]
	pub clnt_ref: Option<Max35Text>,
	#[serde(rename = "CtrPtyRef", skip_serializing_if = "Option::is_none")]
	pub ctr_pty_ref: Option<AdditionalReference13>,
	#[serde(rename = "ExstgAcctId", skip_serializing_if = "Option::is_none")]
	pub exstg_acct_id: Option<Vec<Account23>>,
}


// AccountManagementType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AccountManagementType2Code {
	#[serde(rename = "ACCO")]
	CodeACCO,
	#[serde(rename = "ACCM")]
	CodeACCM,
	#[serde(rename = "GACC")]
	CodeGACC,

	#[default]
	UNKOWN
}


// AccountOwnershipType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AccountOwnershipType4Code {
	#[serde(rename = "UNCO")]
	CodeUNCO,
	#[serde(rename = "LIPA")]
	CodeLIPA,
	#[serde(rename = "ENTR")]
	CodeENTR,
	#[serde(rename = "CORP")]
	CodeCORP,
	#[serde(rename = "CUST")]
	CodeCUST,
	#[serde(rename = "EURE")]
	CodeEURE,
	#[serde(rename = "PART")]
	CodePART,
	#[serde(rename = "TRUS")]
	CodeTRUS,
	#[serde(rename = "GOVO")]
	CodeGOVO,
	#[serde(rename = "JOIT")]
	CodeJOIT,
	#[serde(rename = "COMO")]
	CodeCOMO,
	#[serde(rename = "JOIN")]
	CodeJOIN,
	#[serde(rename = "LLCO")]
	CodeLLCO,
	#[serde(rename = "NOMI")]
	CodeNOMI,
	#[serde(rename = "NFPO")]
	CodeNFPO,
	#[serde(rename = "ONIS")]
	CodeONIS,
	#[serde(rename = "RGIC")]
	CodeRGIC,
	#[serde(rename = "SIGL")]
	CodeSIGL,

	#[default]
	UNKOWN
}


// AccountParties12Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountParties12Choice {
	#[serde(rename = "PmryOwnr", skip_serializing_if = "Option::is_none")]
	pub pmry_ownr: Option<InvestmentAccountOwnershipInformation16>,
	#[serde(rename = "Trstee", skip_serializing_if = "Option::is_none")]
	pub trstee: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "Nmnee", skip_serializing_if = "Option::is_none")]
	pub nmnee: Option<InvestmentAccountOwnershipInformation16>,
	#[serde(rename = "JntOwnr", skip_serializing_if = "Option::is_none")]
	pub jnt_ownr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
}


// AccountParties17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountParties17 {
	#[serde(rename = "PrncplAcctPty")]
	pub prncpl_acct_pty: AccountParties12Choice,
	#[serde(rename = "ScndryOwnr", skip_serializing_if = "Option::is_none")]
	pub scndry_ownr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "Bnfcry", skip_serializing_if = "Option::is_none")]
	pub bnfcry: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "PwrOfAttny", skip_serializing_if = "Option::is_none")]
	pub pwr_of_attny: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "LglGuardn", skip_serializing_if = "Option::is_none")]
	pub lgl_guardn: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "CtdnForMnr", skip_serializing_if = "Option::is_none")]
	pub ctdn_for_mnr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "SucssrOnDth", skip_serializing_if = "Option::is_none")]
	pub sucssr_on_dth: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "Admstr", skip_serializing_if = "Option::is_none")]
	pub admstr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "OthrPty", skip_serializing_if = "Option::is_none")]
	pub othr_pty: Option<Vec<ExtendedParty14>>,
	#[serde(rename = "Grntr", skip_serializing_if = "Option::is_none")]
	pub grntr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "Sttlr", skip_serializing_if = "Option::is_none")]
	pub sttlr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "SnrMggOffcl", skip_serializing_if = "Option::is_none")]
	pub snr_mgg_offcl: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "Prtctr", skip_serializing_if = "Option::is_none")]
	pub prtctr: Option<Vec<InvestmentAccountOwnershipInformation16>>,
	#[serde(rename = "RegdShrhldrNm", skip_serializing_if = "Option::is_none")]
	pub regd_shrhldr_nm: Option<RegisteredShareholderName1Choice>,
}


// AccountSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalAccountIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// AccountStatus2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountStatus2 {
	#[serde(rename = "Nbld", skip_serializing_if = "Option::is_none")]
	pub nbld: Option<EnabledStatusReason1Choice>,
	#[serde(rename = "Dsbld", skip_serializing_if = "Option::is_none")]
	pub dsbld: Option<DisabledStatusReason1Choice>,
	#[serde(rename = "Pdg", skip_serializing_if = "Option::is_none")]
	pub pdg: Option<PendingStatusReason1Choice>,
	#[serde(rename = "PdgOpng", skip_serializing_if = "Option::is_none")]
	pub pdg_opng: Option<PendingOpeningStatusReason1Choice>,
	#[serde(rename = "Profrm", skip_serializing_if = "Option::is_none")]
	pub profrm: Option<ProformaStatusReason1Choice>,
	#[serde(rename = "Clsd", skip_serializing_if = "Option::is_none")]
	pub clsd: Option<ClosedStatusReason1Choice>,
	#[serde(rename = "ClsrPdg", skip_serializing_if = "Option::is_none")]
	pub clsr_pdg: Option<ClosurePendingStatusReason1Choice>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<OtherAccountStatus1>>,
}


// AccountType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<FundCashAccount4Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// AccountUsageType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountUsageType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<AccountUsageType2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// AccountUsageType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AccountUsageType2Code {
	#[serde(rename = "INVE")]
	CodeINVE,
	#[serde(rename = "ISSP")]
	CodeISSP,
	#[serde(rename = "SETP")]
	CodeSETP,
	#[serde(rename = "TRDP")]
	CodeTRDP,

	#[default]
	UNKOWN
}


// AccountingStatus1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountingStatus1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<AccountingStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// AccountingStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AccountingStatus1Code {
	#[serde(rename = "YDOM")]
	CodeYDOM,
	#[serde(rename = "NDOM")]
	CodeNDOM,

	#[default]
	UNKOWN
}


// ActiveCurrencyAnd13DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd13DecimalAmountSimpleType {
	#[serde(rename = "$value")]
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


// AdditionalReference13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdditionalReference13 {
	#[serde(rename = "Ref")]
	pub ref_attr: Max35Text,
	#[serde(rename = "RefIssr", skip_serializing_if = "Option::is_none")]
	pub ref_issr: Option<PartyIdentification125Choice>,
	#[serde(rename = "MsgNm", skip_serializing_if = "Option::is_none")]
	pub msg_nm: Option<Max35Text>,
}


// AdditiononalInformation13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdditiononalInformation13 {
	#[serde(rename = "Lmttn", skip_serializing_if = "Option::is_none")]
	pub lmttn: Option<Max350Text>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
	#[serde(rename = "AcctVldtn", skip_serializing_if = "Option::is_none")]
	pub acct_vldtn: Option<Max350Text>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<Max35Text>,
	#[serde(rename = "Rgltr", skip_serializing_if = "Option::is_none")]
	pub rgltr: Option<PartyIdentification125Choice>,
	#[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
	pub sts: Option<RestrictionStatus1Choice>,
	#[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
	pub prd: Option<DateTimePeriod2>,
}


// AddressType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<AddressType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// AddressType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AddressType1Code {
	#[serde(rename = "HOME")]
	CodeHOME,
	#[serde(rename = "BIZZ")]
	CodeBIZZ,

	#[default]
	UNKOWN
}


// AddressType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<AddressType2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// AddressType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AddressType2Code {
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

	#[default]
	UNKOWN
}


// AlternateSecurityIdentification7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AlternateSecurityIdentification7 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "IdSrc")]
	pub id_src: IdentificationSource1Choice,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
}


// AustrianBankleitzahlIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AustrianBankleitzahlIdentifier {
	#[serde(rename = "$value")]
	pub austrian_bankleitzahl_identifier: String,
}


// BICFIDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BICFIDec2014Identifier {
	#[serde(rename = "$value")]
	pub bicfi_dec2014_identifier: String,
}


// BelgianIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BelgianIdentifier {
	#[serde(rename = "$value")]
	pub belgian_identifier: String,
}


// BlockedHoldingDetails2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BlockedHoldingDetails2 {
	#[serde(rename = "BlckdHldg")]
	pub blckd_hldg: Holding1Code,
	#[serde(rename = "PrtlHldgUnits", skip_serializing_if = "Option::is_none")]
	pub prtl_hldg_units: Option<f64>,
	#[serde(rename = "HldgCertNb", skip_serializing_if = "Option::is_none")]
	pub hldg_cert_nb: Option<Max35Text>,
}


// BlockedReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BlockedReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<BlockedReason2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// BlockedReason2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BlockedReason2Code {
	#[serde(rename = "BKRP")]
	CodeBKRP,
	#[serde(rename = "CMMT")]
	CodeCMMT,
	#[serde(rename = "CNFS")]
	CodeCNFS,
	#[serde(rename = "MORT")]
	CodeMORT,
	#[serde(rename = "PCOM")]
	CodePCOM,
	#[serde(rename = "PLDG")]
	CodePLDG,
	#[serde(rename = "TRPE")]
	CodeTRPE,
	#[serde(rename = "SANC")]
	CodeSANC,
	#[serde(rename = "TRAN")]
	CodeTRAN,

	#[default]
	UNKOWN
}


// BlockedStatusReason2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BlockedStatusReason2 {
	#[serde(rename = "TxTp")]
	pub tx_tp: TransactionType5Choice,
	#[serde(rename = "Blckd")]
	pub blckd: bool,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<BlockedReason2Choice>>,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Max350Text,
}


// BlockedStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BlockedStatusReason2Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<BlockedStatusReason2>>,
}


// Bloomberg2Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Bloomberg2Identifier {
	#[serde(rename = "$value")]
	pub bloomberg2_identifier: String,
}


// BranchData4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BranchData4 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max35Text>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max35Text>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress1>,
}


// CHIPSParticipantIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CHIPSParticipantIdentifier {
	#[serde(rename = "$value")]
	pub chips_participant_identifier: String,
}


// CHIPSUniversalIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CHIPSUniversalIdentifier {
	#[serde(rename = "$value")]
	pub chips_universal_identifier: String,
}


// CRSForm1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CRSForm1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<CRSFormType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// CRSFormType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CRSFormType1Code {
	#[serde(rename = "CER4")]
	CodeCER4,
	#[serde(rename = "CER3")]
	CodeCER3,
	#[serde(rename = "CER5")]
	CodeCER5,
	#[serde(rename = "CER6")]
	CodeCER6,
	#[serde(rename = "CER8")]
	CodeCER8,
	#[serde(rename = "CER1")]
	CodeCER1,
	#[serde(rename = "CER2")]
	CodeCER2,
	#[serde(rename = "CER7")]
	CodeCER7,

	#[default]
	UNKOWN
}


// CRSSource1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CRSSource1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<CRSSourceStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// CRSSourceStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CRSSourceStatus1Code {
	#[serde(rename = "CALC")]
	CodeCALC,
	#[serde(rename = "DECL")]
	CodeDECL,

	#[default]
	UNKOWN
}


// CRSStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CRSStatus1Code {
	#[serde(rename = "C101")]
	CodeC101,
	#[serde(rename = "C102")]
	CodeC102,
	#[serde(rename = "C103")]
	CodeC103,
	#[serde(rename = "C104")]
	CodeC104,
	#[serde(rename = "C105")]
	CodeC105,
	#[serde(rename = "C106")]
	CodeC106,
	#[serde(rename = "C107")]
	CodeC107,
	#[serde(rename = "C108")]
	CodeC108,
	#[serde(rename = "C109")]
	CodeC109,
	#[serde(rename = "C110")]
	CodeC110,
	#[serde(rename = "C111")]
	CodeC111,
	#[serde(rename = "C112")]
	CodeC112,
	#[serde(rename = "C113")]
	CodeC113,
	#[serde(rename = "C114")]
	CodeC114,

	#[default]
	UNKOWN
}


// CRSStatus3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CRSStatus3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<CRSStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// CRSStatus4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CRSStatus4 {
	#[serde(rename = "Tp")]
	pub tp: CRSStatus3Choice,
	#[serde(rename = "Src", skip_serializing_if = "Option::is_none")]
	pub src: Option<CRSSource1Choice>,
	#[serde(rename = "XcptnlRptgCtry", skip_serializing_if = "Option::is_none")]
	pub xcptnl_rptg_ctry: Option<CountryCode>,
}


// CUSIPIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CUSIPIdentifier {
	#[serde(rename = "$value")]
	pub cusip_identifier: String,
}


// CanadianPaymentsARNIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CanadianPaymentsARNIdentifier {
	#[serde(rename = "$value")]
	pub canadian_payments_arn_identifier: String,
}


// CardType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CardType1Code {
	#[serde(rename = "CRDT")]
	CodeCRDT,
	#[serde(rename = "DBIT")]
	CodeDBIT,

	#[default]
	UNKOWN
}


// CashAccount204 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccount204 {
	#[serde(rename = "SttlmCcy")]
	pub sttlm_ccy: ActiveCurrencyCode,
	#[serde(rename = "Id")]
	pub id: AccountIdentificationAndName5,
	#[serde(rename = "AcctOwnr", skip_serializing_if = "Option::is_none")]
	pub acct_ownr: Option<PartyIdentification125Choice>,
	#[serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none")]
	pub acct_svcr: Option<FinancialInstitutionIdentification11Choice>,
	#[serde(rename = "AcctSvcrBrnch", skip_serializing_if = "Option::is_none")]
	pub acct_svcr_brnch: Option<BranchData4>,
	#[serde(rename = "AcctOwnrOthrId", skip_serializing_if = "Option::is_none")]
	pub acct_ownr_othr_id: Option<Vec<GenericIdentification82>>,
	#[serde(rename = "InvstmtAcctTp", skip_serializing_if = "Option::is_none")]
	pub invstmt_acct_tp: Option<AccountType2Choice>,
	#[serde(rename = "CdtDbt", skip_serializing_if = "Option::is_none")]
	pub cdt_dbt: Option<CreditDebit3Code>,
	#[serde(rename = "SttlmInstrRsn", skip_serializing_if = "Option::is_none")]
	pub sttlm_instr_rsn: Option<SettlementInstructionReason1Choice>,
	#[serde(rename = "CshAcctPurp", skip_serializing_if = "Option::is_none")]
	pub csh_acct_purp: Option<CashAccountType3Choice>,
	#[serde(rename = "CshAcctDsgnt", skip_serializing_if = "Option::is_none")]
	pub csh_acct_dsgnt: Option<AccountDesignation1Choice>,
	#[serde(rename = "DvddPctg", skip_serializing_if = "Option::is_none")]
	pub dvdd_pctg: Option<PercentageBoundedRate>,
}


// CashAccountType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<CashAccountType5Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// CashAccountType5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CashAccountType5Code {
	#[serde(rename = "LEND")]
	CodeLEND,
	#[serde(rename = "COLL")]
	CodeCOLL,
	#[serde(rename = "SETT")]
	CodeSETT,
	#[serde(rename = "MARR")]
	CodeMARR,
	#[serde(rename = "SEGT")]
	CodeSEGT,

	#[default]
	UNKOWN
}


// CashSettlement3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashSettlement3 {
	#[serde(rename = "CshAcctDtls", skip_serializing_if = "Option::is_none")]
	pub csh_acct_dtls: Option<Vec<CashAccount204>>,
	#[serde(rename = "OthrCshSttlmDtls", skip_serializing_if = "Option::is_none")]
	pub othr_csh_sttlm_dtls: Option<Vec<PaymentInstrument17>>,
}


// CertificateType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CertificateType2Code {
	#[serde(rename = "AMLC")]
	CodeAMLC,
	#[serde(rename = "DVLC")]
	CodeDVLC,
	#[serde(rename = "DFOR")]
	CodeDFOR,
	#[serde(rename = "GOST")]
	CodeGOST,
	#[serde(rename = "IDEN")]
	CodeIDEN,
	#[serde(rename = "INCU")]
	CodeINCU,
	#[serde(rename = "LREF")]
	CodeLREF,
	#[serde(rename = "PASS")]
	CodePASS,
	#[serde(rename = "PRAD")]
	CodePRAD,
	#[serde(rename = "PKIC")]
	CodePKIC,

	#[default]
	UNKOWN
}


// CertificationType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CertificationType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<CertificateType2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// Cheque4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Cheque4 {
	#[serde(rename = "PyeeId")]
	pub pyee_id: NameAndAddress5,
}


// CitizenshipInformation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CitizenshipInformation2 {
	#[serde(rename = "Ntlty")]
	pub ntlty: String,
	#[serde(rename = "MnrInd")]
	pub mnr_ind: bool,
}


// CivilStatus1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CivilStatus1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<CivilStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// CivilStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CivilStatus1Code {
	#[serde(rename = "DIVO")]
	CodeDIVO,
	#[serde(rename = "LDIV")]
	CodeLDIV,
	#[serde(rename = "MARR")]
	CodeMARR,
	#[serde(rename = "SEPA")]
	CodeSEPA,
	#[serde(rename = "SING")]
	CodeSING,
	#[serde(rename = "UNIO")]
	CodeUNIO,
	#[serde(rename = "WIDO")]
	CodeWIDO,

	#[default]
	UNKOWN
}


// ClearingSystemMemberIdentification4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingSystemMemberIdentification4Choice {
	#[serde(rename = "USCHU", skip_serializing_if = "Option::is_none")]
	pub uschu: Option<CHIPSUniversalIdentifier>,
	#[serde(rename = "NZNCC", skip_serializing_if = "Option::is_none")]
	pub nzncc: Option<NewZealandNCCIdentifier>,
	#[serde(rename = "IENSC", skip_serializing_if = "Option::is_none")]
	pub iensc: Option<IrishNSCIdentifier>,
	#[serde(rename = "GBSC", skip_serializing_if = "Option::is_none")]
	pub gbsc: Option<UKDomesticSortCodeIdentifier>,
	#[serde(rename = "USCH", skip_serializing_if = "Option::is_none")]
	pub usch: Option<CHIPSParticipantIdentifier>,
	#[serde(rename = "CHBC", skip_serializing_if = "Option::is_none")]
	pub chbc: Option<SwissBCIdentifier>,
	#[serde(rename = "USFW", skip_serializing_if = "Option::is_none")]
	pub usfw: Option<FedwireRoutingNumberIdentifier>,
	#[serde(rename = "PTNCC", skip_serializing_if = "Option::is_none")]
	pub ptncc: Option<PortugueseNCCIdentifier>,
	#[serde(rename = "RUCB", skip_serializing_if = "Option::is_none")]
	pub rucb: Option<RussianCentralBankIdentificationCodeIdentifier>,
	#[serde(rename = "ITNCC", skip_serializing_if = "Option::is_none")]
	pub itncc: Option<ItalianDomesticIdentifier>,
	#[serde(rename = "ATBLZ", skip_serializing_if = "Option::is_none")]
	pub atblz: Option<AustrianBankleitzahlIdentifier>,
	#[serde(rename = "CACPA", skip_serializing_if = "Option::is_none")]
	pub cacpa: Option<CanadianPaymentsARNIdentifier>,
	#[serde(rename = "CHSIC", skip_serializing_if = "Option::is_none")]
	pub chsic: Option<SwissSICIdentifier>,
	#[serde(rename = "DEBLZ", skip_serializing_if = "Option::is_none")]
	pub deblz: Option<GermanBankleitzahlIdentifier>,
	#[serde(rename = "ESNCC", skip_serializing_if = "Option::is_none")]
	pub esncc: Option<SpanishDomesticInterbankingIdentifier>,
	#[serde(rename = "ZANCC", skip_serializing_if = "Option::is_none")]
	pub zancc: Option<SouthAfricanNCCIdentifier>,
	#[serde(rename = "HKNCC", skip_serializing_if = "Option::is_none")]
	pub hkncc: Option<HongKongBankIdentifier>,
	#[serde(rename = "AUBSBx", skip_serializing_if = "Option::is_none")]
	pub aubs_bx: Option<ExtensiveBranchNetworkIdentifier>,
	#[serde(rename = "AUBSBs", skip_serializing_if = "Option::is_none")]
	pub aubs_bs: Option<SmallNetworkIdentifier>,
}


// ClosedStatusReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClosedStatusReason1 {
	#[serde(rename = "Cd")]
	pub cd: ClosedStatusReason2Choice,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
}


// ClosedStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClosedStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<ClosedStatusReason1>>,
}


// ClosedStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ClosedStatusReason1Code {
	#[serde(rename = "ASIN")]
	CodeASIN,
	#[serde(rename = "CLIN")]
	CodeCLIN,

	#[default]
	UNKOWN
}


// ClosedStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClosedStatusReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ClosedStatusReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}


// ClosurePendingStatusReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClosurePendingStatusReason1 {
	#[serde(rename = "Cd")]
	pub cd: ClosurePendingStatusReason2Choice,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
}


// ClosurePendingStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClosurePendingStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<ClosurePendingStatusReason1>>,
}


// ClosurePendingStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ClosurePendingStatusReason1Code {
	#[serde(rename = "CLOS")]
	CodeCLOS,
	#[serde(rename = "PEND")]
	CodePEND,

	#[default]
	UNKOWN
}


// ClosurePendingStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClosurePendingStatusReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ClosurePendingStatusReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}


// Collateral1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Collateral1Code {
	#[serde(rename = "COLL")]
	CodeCOLL,
	#[serde(rename = "NCOL")]
	CodeNCOL,

	#[default]
	UNKOWN
}


// CommunicationAddress6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommunicationAddress6 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<AddressType1Choice>,
	#[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
	pub email: Option<Max256Text>,
	#[serde(rename = "Phne", skip_serializing_if = "Option::is_none")]
	pub phne: Option<PhoneNumber>,
	#[serde(rename = "Mob", skip_serializing_if = "Option::is_none")]
	pub mob: Option<PhoneNumber>,
	#[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
	pub fax_nb: Option<PhoneNumber>,
	#[serde(rename = "TlxAdr", skip_serializing_if = "Option::is_none")]
	pub tlx_adr: Option<Max35Text>,
	#[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
	pub url_adr: Option<Max256Text>,
}


// CommunicationMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CommunicationMethod1Code {
	#[serde(rename = "SWMT")]
	CodeSWMT,
	#[serde(rename = "SWMX")]
	CodeSWMX,
	#[serde(rename = "FAXI")]
	CodeFAXI,
	#[serde(rename = "EMAL")]
	CodeEMAL,
	#[serde(rename = "PROP")]
	CodePROP,

	#[default]
	UNKOWN
}


// CommunicationMethod3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommunicationMethod3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<CommunicationMethod1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// CompanyLink1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompanyLink1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<CompanyLink1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// CompanyLink1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CompanyLink1Code {
	#[serde(rename = "AGEN")]
	CodeAGEN,
	#[serde(rename = "BROK")]
	CodeBROK,
	#[serde(rename = "PART")]
	CodePART,
	#[serde(rename = "MEMB")]
	CodeMEMB,
	#[serde(rename = "PCOM")]
	CodePCOM,
	#[serde(rename = "RELA")]
	CodeRELA,

	#[default]
	UNKOWN
}


// ConductClassification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ConductClassification1Code {
	#[serde(rename = "NSTA")]
	CodeNSTA,
	#[serde(rename = "RCLT")]
	CodeRCLT,
	#[serde(rename = "STAN")]
	CodeSTAN,

	#[default]
	UNKOWN
}


// ConfirmationType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConfirmationType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<AccountManagementType2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// ConsolidatedTapeAssociationIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConsolidatedTapeAssociationIdentifier {
	#[serde(rename = "$value")]
	pub consolidated_tape_association_identifier: String,
}


// ConsolidationType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ConsolidationType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ConsolidationType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// ConsolidationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ConsolidationType1Code {
	#[serde(rename = "GENL")]
	CodeGENL,
	#[serde(rename = "PART")]
	CodePART,

	#[default]
	UNKOWN
}


// CountryAndResidentialStatusType2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryAndResidentialStatusType2 {
	#[serde(rename = "Ctry")]
	pub ctry: CountryCode,
	#[serde(rename = "ResdtlSts")]
	pub resdtl_sts: ResidentialStatus1Code,
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
	#[serde(rename = "CRDT")]
	CodeCRDT,
	#[serde(rename = "DBIT")]
	CodeDBIT,

	#[default]
	UNKOWN
}


// CustomerConductClassification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CustomerConductClassification1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ConductClassification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// DataBaseCheck1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DataBaseCheck1 {
	#[serde(rename = "DBChck")]
	pub db_chck: bool,
	#[serde(rename = "Id")]
	pub id: Max35Text,
}


// DateAndAmount1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndAmount1 {
	#[serde(rename = "Dt")]
	pub dt: String,
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
}


// DateAndDateTime1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTime1Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<String>,
}


// DateTimePeriod2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod2 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm", skip_serializing_if = "Option::is_none")]
	pub to_dt_tm: Option<String>,
}


// DeMinimus1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DeMinimus1Choice {
	#[serde(rename = "DeMnmsAplbl", skip_serializing_if = "Option::is_none")]
	pub de_mnms_aplbl: Option<DeMinimusApplicable1>,
	#[serde(rename = "DeMnmsNotAplbl", skip_serializing_if = "Option::is_none")]
	pub de_mnms_not_aplbl: Option<DeMinimusNotApplicable1>,
}


// DeMinimusApplicable1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DeMinimusApplicable1 {
	#[serde(rename = "NewIssePrmssn")]
	pub new_isse_prmssn: bool,
	#[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
	pub pctg: Option<f64>,
}


// DeMinimusNotApplicable1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DeMinimusNotApplicable1 {
	#[serde(rename = "RstrctdPrsnRsn")]
	pub rstrctd_prsn_rsn: Max350Text,
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "$value")]
	pub decimal_number: f64,
}


// DirectDebitMandate7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DirectDebitMandate7 {
	#[serde(rename = "DbtrAcct")]
	pub dbtr_acct: AccountIdentificationAndName5,
	#[serde(rename = "Dbtr", skip_serializing_if = "Option::is_none")]
	pub dbtr: Option<PartyIdentification125Choice>,
	#[serde(rename = "DbtrTaxIdNb", skip_serializing_if = "Option::is_none")]
	pub dbtr_tax_id_nb: Option<Max35Text>,
	#[serde(rename = "DbtrNtlRegnNb", skip_serializing_if = "Option::is_none")]
	pub dbtr_ntl_regn_nb: Option<Max35Text>,
	#[serde(rename = "Cdtr", skip_serializing_if = "Option::is_none")]
	pub cdtr: Option<PartyIdentification125Choice>,
	#[serde(rename = "DbtrAgt")]
	pub dbtr_agt: FinancialInstitutionIdentification11Choice,
	#[serde(rename = "DbtrAgtBrnch", skip_serializing_if = "Option::is_none")]
	pub dbtr_agt_brnch: Option<BranchData4>,
	#[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
	pub cdtr_agt: Option<FinancialInstitutionIdentification11Choice>,
	#[serde(rename = "CdtrAgtBrnch", skip_serializing_if = "Option::is_none")]
	pub cdtr_agt_brnch: Option<BranchData4>,
	#[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
	pub regn_id: Option<Max35Text>,
	#[serde(rename = "MndtId", skip_serializing_if = "Option::is_none")]
	pub mndt_id: Option<Max35Text>,
}


// DisabledReason2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum DisabledReason2Code {
	#[serde(rename = "CLOS")]
	CodeCLOS,
	#[serde(rename = "BKRP")]
	CodeBKRP,
	#[serde(rename = "CMMT")]
	CodeCMMT,
	#[serde(rename = "CNFS")]
	CodeCNFS,
	#[serde(rename = "MORT")]
	CodeMORT,
	#[serde(rename = "PCOM")]
	CodePCOM,
	#[serde(rename = "PLDG")]
	CodePLDG,
	#[serde(rename = "TRPE")]
	CodeTRPE,
	#[serde(rename = "SANC")]
	CodeSANC,
	#[serde(rename = "TRAN")]
	CodeTRAN,
	#[serde(rename = "REJT")]
	CodeREJT,

	#[default]
	UNKOWN
}


// DisabledStatusReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DisabledStatusReason1 {
	#[serde(rename = "Cd")]
	pub cd: DisabledStatusReason2Choice,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
}


// DisabledStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DisabledStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<DisabledStatusReason1>>,
}


// DisabledStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DisabledStatusReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<DisabledReason2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}


// DistributionPolicy1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum DistributionPolicy1Code {
	#[serde(rename = "DIST")]
	CodeDIST,
	#[serde(rename = "ACCU")]
	CodeACCU,

	#[default]
	UNKOWN
}


// DocumentToSend4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentToSend4 {
	#[serde(rename = "Tp")]
	pub tp: Max140Text,
	#[serde(rename = "Rcpt")]
	pub rcpt: PartyIdentification125Choice,
	#[serde(rename = "MtdOfTrnsmssn")]
	pub mtd_of_trnsmssn: CommunicationMethod3Choice,
}


// DutchIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DutchIdentifier {
	#[serde(rename = "$value")]
	pub dutch_identifier: String,
}


// Eligible1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Eligible1Code {
	#[serde(rename = "ELIG")]
	CodeELIG,
	#[serde(rename = "NELI")]
	CodeNELI,

	#[default]
	UNKOWN
}


// EnabledStatusReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnabledStatusReason1 {
	#[serde(rename = "Cd")]
	pub cd: EnabledStatusReason2Choice,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
}


// EnabledStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnabledStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<EnabledStatusReason1>>,
}


// EnabledStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EnabledStatusReason1Code {
	#[serde(rename = "MODI")]
	CodeMODI,

	#[default]
	UNKOWN
}


// EnabledStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnabledStatusReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<EnabledStatusReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}


// EuroclearClearstreamIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EuroclearClearstreamIdentifier {
	#[serde(rename = "$value")]
	pub euroclear_clearstream_identifier: String,
}


// EventFrequency10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EventFrequency10Code {
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "ADHO")]
	CodeADHO,

	#[default]
	UNKOWN
}


// EventFrequency1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EventFrequency1Code {
	#[serde(rename = "YEAR")]
	CodeYEAR,
	#[serde(rename = "SEMI")]
	CodeSEMI,
	#[serde(rename = "QUTR")]
	CodeQUTR,
	#[serde(rename = "TOMN")]
	CodeTOMN,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "TWMN")]
	CodeTWMN,
	#[serde(rename = "TOWK")]
	CodeTOWK,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "ADHO")]
	CodeADHO,
	#[serde(rename = "INDA")]
	CodeINDA,
	#[serde(rename = "OVNG")]
	CodeOVNG,
	#[serde(rename = "ONDE")]
	CodeONDE,

	#[default]
	UNKOWN
}


// EventFrequency8Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EventFrequency8Code {
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

	#[default]
	UNKOWN
}


// EventFrequency9Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EventFrequency9Code {
	#[serde(rename = "YEAR")]
	CodeYEAR,
	#[serde(rename = "SEMI")]
	CodeSEMI,
	#[serde(rename = "QUTR")]
	CodeQUTR,
	#[serde(rename = "TOMN")]
	CodeTOMN,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "TWMN")]
	CodeTWMN,
	#[serde(rename = "TOWK")]
	CodeTOWK,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "ADHO")]
	CodeADHO,
	#[serde(rename = "INDA")]
	CodeINDA,
	#[serde(rename = "OVNG")]
	CodeOVNG,
	#[serde(rename = "ONDE")]
	CodeONDE,
	#[serde(rename = "NONE")]
	CodeNONE,

	#[default]
	UNKOWN
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "$value")]
	pub exact4_alpha_numeric_text: String,
}


// Extended350Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Extended350Code {
	#[serde(rename = "$value")]
	pub extended350_code: String,
}


// ExtendedParty14 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExtendedParty14 {
	#[serde(rename = "XtndedPtyRole")]
	pub xtnded_pty_role: Extended350Code,
	#[serde(rename = "OthrPtyDtls")]
	pub othr_pty_dtls: InvestmentAccountOwnershipInformation16,
}


// Extension1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Extension1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Max350Text,
	#[serde(rename = "Txt")]
	pub txt: Max350Text,
}


// ExtensiveBranchNetworkIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExtensiveBranchNetworkIdentifier {
	#[serde(rename = "$value")]
	pub extensive_branch_network_identifier: String,
}


// ExternalAccountIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAccountIdentification1Code {
	#[serde(rename = "$value")]
	pub external_account_identification1_code: String,
}


// FATCAForm1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FATCAForm1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<FATCAFormType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// FATCAFormType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FATCAFormType1Code {
	#[serde(rename = "CER5")]
	CodeCER5,
	#[serde(rename = "CER7")]
	CodeCER7,
	#[serde(rename = "CER1")]
	CodeCER1,
	#[serde(rename = "CER2")]
	CodeCER2,
	#[serde(rename = "CER3")]
	CodeCER3,
	#[serde(rename = "CER4")]
	CodeCER4,
	#[serde(rename = "CER6")]
	CodeCER6,

	#[default]
	UNKOWN
}


// FATCASource1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FATCASource1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<FATCASourceStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// FATCASourceStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FATCASourceStatus1Code {
	#[serde(rename = "CALC")]
	CodeCALC,
	#[serde(rename = "DECL")]
	CodeDECL,

	#[default]
	UNKOWN
}


// FATCAStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FATCAStatus1Code {
	#[serde(rename = "F101")]
	CodeF101,
	#[serde(rename = "F102")]
	CodeF102,
	#[serde(rename = "F103")]
	CodeF103,
	#[serde(rename = "F104")]
	CodeF104,
	#[serde(rename = "F105")]
	CodeF105,
	#[serde(rename = "F201")]
	CodeF201,
	#[serde(rename = "F202")]
	CodeF202,
	#[serde(rename = "F203")]
	CodeF203,
	#[serde(rename = "F204")]
	CodeF204,
	#[serde(rename = "F205")]
	CodeF205,
	#[serde(rename = "F206")]
	CodeF206,

	#[default]
	UNKOWN
}


// FATCAStatus2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FATCAStatus2 {
	#[serde(rename = "Tp")]
	pub tp: FATCAStatus2Choice,
	#[serde(rename = "Src", skip_serializing_if = "Option::is_none")]
	pub src: Option<FATCASource1Choice>,
}


// FATCAStatus2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FATCAStatus2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<FATCAStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// FedwireRoutingNumberIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedwireRoutingNumberIdentifier {
	#[serde(rename = "$value")]
	pub fedwire_routing_number_identifier: String,
}


// FinancialInstitutionIdentification11Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstitutionIdentification11Choice {
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
	#[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
	pub bicfi: Option<BICFIDec2014Identifier>,
	#[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification4Choice>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<SimpleIdentificationInformation4>,
}


// FinancialInstrument87 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument87 {
	#[serde(rename = "Id")]
	pub id: SecurityIdentification25Choice,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max350Text>,
	#[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
	pub shrt_nm: Option<Max35Text>,
	#[serde(rename = "SplmtryId", skip_serializing_if = "Option::is_none")]
	pub splmtry_id: Option<Max35Text>,
	#[serde(rename = "ClssTp", skip_serializing_if = "Option::is_none")]
	pub clss_tp: Option<Max35Text>,
	#[serde(rename = "SctiesForm", skip_serializing_if = "Option::is_none")]
	pub scties_form: Option<FormOfSecurity1Code>,
	#[serde(rename = "DstrbtnPlcy", skip_serializing_if = "Option::is_none")]
	pub dstrbtn_plcy: Option<DistributionPolicy1Code>,
	#[serde(rename = "PdctGrp", skip_serializing_if = "Option::is_none")]
	pub pdct_grp: Option<Max140Text>,
	#[serde(rename = "BlckdHldgDtls", skip_serializing_if = "Option::is_none")]
	pub blckd_hldg_dtls: Option<BlockedHoldingDetails2>,
	#[serde(rename = "Pldgg", skip_serializing_if = "Option::is_none")]
	pub pldgg: Option<Eligible1Code>,
	#[serde(rename = "Coll", skip_serializing_if = "Option::is_none")]
	pub coll: Option<Collateral1Code>,
	#[serde(rename = "ThrdPtyRghts", skip_serializing_if = "Option::is_none")]
	pub thrd_pty_rghts: Option<ThirdPartyRights2>,
	#[serde(rename = "FndOwnrsh", skip_serializing_if = "Option::is_none")]
	pub fnd_ownrsh: Option<FundOwnership1Code>,
	#[serde(rename = "FndIntntn", skip_serializing_if = "Option::is_none")]
	pub fnd_intntn: Option<FundIntention1Code>,
	#[serde(rename = "OprlSts", skip_serializing_if = "Option::is_none")]
	pub oprl_sts: Option<OperationalStatus1Code>,
}


// FiscalYear1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FiscalYear1Choice {
	#[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
	pub start_dt: Option<String>,
	#[serde(rename = "EndDt", skip_serializing_if = "Option::is_none")]
	pub end_dt: Option<String>,
}


// FormOfSecurity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FormOfSecurity1Code {
	#[serde(rename = "BEAR")]
	CodeBEAR,
	#[serde(rename = "REGD")]
	CodeREGD,

	#[default]
	UNKOWN
}


// Frequency20Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Frequency20Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<EventFrequency8Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// FundCashAccount4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FundCashAccount4Code {
	#[serde(rename = "HEDG")]
	CodeHEDG,
	#[serde(rename = "CPFO")]
	CodeCPFO,
	#[serde(rename = "CPFS")]
	CodeCPFS,
	#[serde(rename = "SRSA")]
	CodeSRSA,
	#[serde(rename = "CSDO")]
	CodeCSDO,
	#[serde(rename = "TOFF")]
	CodeTOFF,
	#[serde(rename = "ICSA")]
	CodeICSA,
	#[serde(rename = "CSDM")]
	CodeCSDM,
	#[serde(rename = "CSDP")]
	CodeCSDP,
	#[serde(rename = "PPEN")]
	CodePPEN,
	#[serde(rename = "CPEN")]
	CodeCPEN,

	#[default]
	UNKOWN
}


// FundIntention1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FundIntention1Code {
	#[serde(rename = "YQUA")]
	CodeYQUA,
	#[serde(rename = "NQUA")]
	CodeNQUA,

	#[default]
	UNKOWN
}


// FundOwnership1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FundOwnership1Code {
	#[serde(rename = "YALL")]
	CodeYALL,
	#[serde(rename = "NALL")]
	CodeNALL,

	#[default]
	UNKOWN
}


// GDPRData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GDPRData1 {
	#[serde(rename = "CnsntTp")]
	pub cnsnt_tp: GDPRDataConsent1Choice,
	#[serde(rename = "CnsntInd")]
	pub cnsnt_ind: bool,
	#[serde(rename = "CnsntDt")]
	pub cnsnt_dt: String,
}


// GDPRDataConsent1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GDPRDataConsent1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<GDPRDataConsent1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// GDPRDataConsent1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum GDPRDataConsent1Code {
	#[serde(rename = "DP00")]
	CodeDP00,
	#[serde(rename = "DP03")]
	CodeDP03,
	#[serde(rename = "DP01")]
	CodeDP01,
	#[serde(rename = "DP02")]
	CodeDP02,

	#[default]
	UNKOWN
}


// Gender1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Gender1Code {
	#[serde(rename = "FEMA")]
	CodeFEMA,
	#[serde(rename = "MALE")]
	CodeMALE,

	#[default]
	UNKOWN
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


// GenericIdentification82 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification82 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Tp")]
	pub tp: OtherIdentification3Choice,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
	#[serde(rename = "IsseDt", skip_serializing_if = "Option::is_none")]
	pub isse_dt: Option<String>,
	#[serde(rename = "XpryDt", skip_serializing_if = "Option::is_none")]
	pub xpry_dt: Option<String>,
	#[serde(rename = "Stat", skip_serializing_if = "Option::is_none")]
	pub stat: Option<Max70Text>,
	#[serde(rename = "IssrCtry", skip_serializing_if = "Option::is_none")]
	pub issr_ctry: Option<CountryCode>,
}


// GermanBankleitzahlIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GermanBankleitzahlIdentifier {
	#[serde(rename = "$value")]
	pub german_bankleitzahl_identifier: String,
}


// HighFrequencyTradingProfile1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct HighFrequencyTradingProfile1 {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "SttlmFrqcy", skip_serializing_if = "Option::is_none")]
	pub sttlm_frqcy: Option<SettlementFrequency1Choice>,
	#[serde(rename = "CnsldtnTp", skip_serializing_if = "Option::is_none")]
	pub cnsldtn_tp: Option<ConsolidationType1Choice>,
}


// Holding1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Holding1Code {
	#[serde(rename = "CERT")]
	CodeCERT,
	#[serde(rename = "NPRH")]
	CodeNPRH,
	#[serde(rename = "PRTH")]
	CodePRTH,

	#[default]
	UNKOWN
}


// HongKongBankIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct HongKongBankIdentifier {
	#[serde(rename = "$value")]
	pub hong_kong_bank_identifier: String,
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


// ISOYearMonth ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISOYearMonth {
	#[serde(rename = "$value")]
	pub iso_year_month: String,
}


// IdentificationSource1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IdentificationSource1Choice {
	#[serde(rename = "Dmst", skip_serializing_if = "Option::is_none")]
	pub dmst: Option<CountryCode>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// IncomePreference2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum IncomePreference2Code {
	#[serde(rename = "CASH")]
	CodeCASH,
	#[serde(rename = "SECU")]
	CodeSECU,

	#[default]
	UNKOWN
}


// IndividualPerson29 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndividualPerson29 {
	#[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
	pub nm_prfx: Option<NamePrefix1Choice>,
	#[serde(rename = "GvnNm", skip_serializing_if = "Option::is_none")]
	pub gvn_nm: Option<Max35Text>,
	#[serde(rename = "MddlNm", skip_serializing_if = "Option::is_none")]
	pub mddl_nm: Option<Max35Text>,
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Vec<PostalAddress21>,
}


// IndividualPerson37 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndividualPerson37 {
	#[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
	pub nm_prfx: Option<NamePrefix1Choice>,
	#[serde(rename = "GvnNm", skip_serializing_if = "Option::is_none")]
	pub gvn_nm: Option<Max35Text>,
	#[serde(rename = "MddlNm", skip_serializing_if = "Option::is_none")]
	pub mddl_nm: Option<Max35Text>,
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "NmSfx", skip_serializing_if = "Option::is_none")]
	pub nm_sfx: Option<Max35Text>,
	#[serde(rename = "Gndr", skip_serializing_if = "Option::is_none")]
	pub gndr: Option<Gender1Code>,
	#[serde(rename = "BirthDt", skip_serializing_if = "Option::is_none")]
	pub birth_dt: Option<String>,
	#[serde(rename = "CtryOfBirth", skip_serializing_if = "Option::is_none")]
	pub ctry_of_birth: Option<CountryCode>,
	#[serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none")]
	pub prvc_of_birth: Option<Max35Text>,
	#[serde(rename = "CityOfBirth", skip_serializing_if = "Option::is_none")]
	pub city_of_birth: Option<Max35Text>,
	#[serde(rename = "Prfssn", skip_serializing_if = "Option::is_none")]
	pub prfssn: Option<Max35Text>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Vec<PostalAddress21>,
	#[serde(rename = "Ctznsh", skip_serializing_if = "Option::is_none")]
	pub ctznsh: Option<Vec<CitizenshipInformation2>>,
	#[serde(rename = "EmplngCpny", skip_serializing_if = "Option::is_none")]
	pub emplng_cpny: Option<Max140Text>,
	#[serde(rename = "BizFctn", skip_serializing_if = "Option::is_none")]
	pub biz_fctn: Option<Max35Text>,
	#[serde(rename = "PltclyXpsdPrsn", skip_serializing_if = "Option::is_none")]
	pub pltcly_xpsd_prsn: Option<PoliticallyExposedPerson1>,
	#[serde(rename = "DthDt", skip_serializing_if = "Option::is_none")]
	pub dth_dt: Option<String>,
	#[serde(rename = "CvlSts", skip_serializing_if = "Option::is_none")]
	pub cvl_sts: Option<CivilStatus1Choice>,
	#[serde(rename = "EdctnLvl", skip_serializing_if = "Option::is_none")]
	pub edctn_lvl: Option<Max35Text>,
	#[serde(rename = "FmlyInf", skip_serializing_if = "Option::is_none")]
	pub fmly_inf: Option<PersonalInformation1>,
	#[serde(rename = "GDPRData", skip_serializing_if = "Option::is_none")]
	pub gdpr_data: Option<Vec<GDPRData1>>,
}


// InformationDistribution1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InformationDistribution1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InformationDistribution2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// InformationDistribution2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InformationDistribution2Code {
	#[serde(rename = "ELEC")]
	CodeELEC,
	#[serde(rename = "NONE")]
	CodeNONE,
	#[serde(rename = "PAPR")]
	CodePAPR,

	#[default]
	UNKOWN
}


// InitialAmount1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InitialAmount1Choice {
	#[serde(rename = "InitlNbOfInstlmts", skip_serializing_if = "Option::is_none")]
	pub initl_nb_of_instlmts: Option<f64>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAndAmount>,
}


// Insurance1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Insurance1Code {
	#[serde(rename = "LIFE")]
	CodeLIFE,
	#[serde(rename = "PDIS")]
	CodePDIS,

	#[default]
	UNKOWN
}


// InsuranceType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InsuranceType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Insurance1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// Intermediary46 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Intermediary46 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification177Choice,
	#[serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none")]
	pub lgl_ntty_idr: Option<LEIIdentifier>,
	#[serde(rename = "Acct", skip_serializing_if = "Option::is_none")]
	pub acct: Option<Account32>,
	#[serde(rename = "WvdTrlrComssnInd", skip_serializing_if = "Option::is_none")]
	pub wvd_trlr_comssn_ind: Option<bool>,
	#[serde(rename = "Role", skip_serializing_if = "Option::is_none")]
	pub role: Option<PartyRole2Choice>,
	#[serde(rename = "PmryComAdr", skip_serializing_if = "Option::is_none")]
	pub pmry_com_adr: Option<Vec<CommunicationAddress6>>,
	#[serde(rename = "ScndryComAdr", skip_serializing_if = "Option::is_none")]
	pub scndry_com_adr: Option<Vec<CommunicationAddress6>>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress4>,
}


// InvestmentAccount74 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentAccount74 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "AcctSts", skip_serializing_if = "Option::is_none")]
	pub acct_sts: Option<AccountStatus2>,
	#[serde(rename = "BlckdSts", skip_serializing_if = "Option::is_none")]
	pub blckd_sts: Option<BlockedStatusReason2Choice>,
	#[serde(rename = "StsDt", skip_serializing_if = "Option::is_none")]
	pub sts_dt: Option<DateAndDateTime1Choice>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max35Text>,
	#[serde(rename = "Dsgnt", skip_serializing_if = "Option::is_none")]
	pub dsgnt: Option<Max35Text>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<AccountType2Choice>,
	#[serde(rename = "OwnrshTp", skip_serializing_if = "Option::is_none")]
	pub ownrsh_tp: Option<OwnershipType2Choice>,
	#[serde(rename = "TaxXmptn", skip_serializing_if = "Option::is_none")]
	pub tax_xmptn: Option<TaxExemptionReason2Choice>,
	#[serde(rename = "StmtFrqcy", skip_serializing_if = "Option::is_none")]
	pub stmt_frqcy: Option<StatementFrequencyReason2Choice>,
	#[serde(rename = "RefCcy", skip_serializing_if = "Option::is_none")]
	pub ref_ccy: Option<ActiveCurrencyCode>,
	#[serde(rename = "Lang", skip_serializing_if = "Option::is_none")]
	pub lang: Option<String>,
	#[serde(rename = "IncmPref", skip_serializing_if = "Option::is_none")]
	pub incm_pref: Option<IncomePreference2Code>,
	#[serde(rename = "RinvstmtDtls", skip_serializing_if = "Option::is_none")]
	pub rinvstmt_dtls: Option<Vec<Reinvestment4>>,
	#[serde(rename = "TaxWhldgMtd", skip_serializing_if = "Option::is_none")]
	pub tax_whldg_mtd: Option<TaxWithholdingMethod3Code>,
	#[serde(rename = "TaxRptg", skip_serializing_if = "Option::is_none")]
	pub tax_rptg: Option<Vec<TaxReporting3>>,
	#[serde(rename = "LttrInttDtls", skip_serializing_if = "Option::is_none")]
	pub lttr_intt_dtls: Option<LetterIntent1>,
	#[serde(rename = "AcmltnRghtRef", skip_serializing_if = "Option::is_none")]
	pub acmltn_rght_ref: Option<Max35Text>,
	#[serde(rename = "ReqrdSgntriesNb", skip_serializing_if = "Option::is_none")]
	pub reqrd_sgntries_nb: Option<f64>,
	#[serde(rename = "FndFmlyNm", skip_serializing_if = "Option::is_none")]
	pub fnd_fmly_nm: Option<Max350Text>,
	#[serde(rename = "FinInstrmDtls", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_dtls: Option<Vec<FinancialInstrument87>>,
	#[serde(rename = "RndgDtls", skip_serializing_if = "Option::is_none")]
	pub rndg_dtls: Option<RoundingParameters1>,
	#[serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none")]
	pub acct_svcr: Option<PartyIdentification125Choice>,
	#[serde(rename = "AcctUsgTp", skip_serializing_if = "Option::is_none")]
	pub acct_usg_tp: Option<AccountUsageType2Choice>,
	#[serde(rename = "FrgnStsCertfctn", skip_serializing_if = "Option::is_none")]
	pub frgn_sts_certfctn: Option<Provided1Code>,
	#[serde(rename = "AcctSgntrDtTm", skip_serializing_if = "Option::is_none")]
	pub acct_sgntr_dt_tm: Option<DateAndDateTime1Choice>,
	#[serde(rename = "TxChanlTp", skip_serializing_if = "Option::is_none")]
	pub tx_chanl_tp: Option<TransactionChannelType1Choice>,
	#[serde(rename = "InvstmtAcctCtgy", skip_serializing_if = "Option::is_none")]
	pub invstmt_acct_ctgy: Option<InvestmentAccountCategory1Choice>,
	#[serde(rename = "Pldgg", skip_serializing_if = "Option::is_none")]
	pub pldgg: Option<Eligible1Code>,
	#[serde(rename = "Coll", skip_serializing_if = "Option::is_none")]
	pub coll: Option<Collateral1Code>,
	#[serde(rename = "ThrdPtyRghts", skip_serializing_if = "Option::is_none")]
	pub thrd_pty_rghts: Option<ThirdPartyRights2>,
	#[serde(rename = "PwrOfAttnyLvlOfCtrl", skip_serializing_if = "Option::is_none")]
	pub pwr_of_attny_lvl_of_ctrl: Option<LevelOfControl1Choice>,
	#[serde(rename = "AcctgSts", skip_serializing_if = "Option::is_none")]
	pub acctg_sts: Option<AccountingStatus1Choice>,
	#[serde(rename = "OpngDt", skip_serializing_if = "Option::is_none")]
	pub opng_dt: Option<DateAndDateTime1Choice>,
	#[serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none")]
	pub clsg_dt: Option<DateAndDateTime1Choice>,
	#[serde(rename = "NegInd", skip_serializing_if = "Option::is_none")]
	pub neg_ind: Option<bool>,
	#[serde(rename = "PrcgOrdr", skip_serializing_if = "Option::is_none")]
	pub prcg_ordr: Option<PositionEffect3Code>,
	#[serde(rename = "Lblty", skip_serializing_if = "Option::is_none")]
	pub lblty: Option<Liability1Choice>,
	#[serde(rename = "InvstrPrfl", skip_serializing_if = "Option::is_none")]
	pub invstr_prfl: Option<Vec<InvestorProfile2>>,
	#[serde(rename = "FsclYr", skip_serializing_if = "Option::is_none")]
	pub fscl_yr: Option<FiscalYear1Choice>,
}


// InvestmentAccountCategory1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentAccountCategory1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InvestmentAccountCategory1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// InvestmentAccountCategory1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InvestmentAccountCategory1Code {
	#[serde(rename = "MAND")]
	CodeMAND,
	#[serde(rename = "RETA")]
	CodeRETA,

	#[default]
	UNKOWN
}


// InvestmentAccountOwnershipInformation16 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentAccountOwnershipInformation16 {
	#[serde(rename = "Pty")]
	pub pty: Party47Choice,
	#[serde(rename = "MnyLndrgChck", skip_serializing_if = "Option::is_none")]
	pub mny_lndrg_chck: Option<MoneyLaunderingCheck1Choice>,
	#[serde(rename = "InvstrPrflVldtn", skip_serializing_if = "Option::is_none")]
	pub invstr_prfl_vldtn: Option<Vec<PartyProfileInformation5>>,
	#[serde(rename = "OwnrshBnfcryRate", skip_serializing_if = "Option::is_none")]
	pub ownrsh_bnfcry_rate: Option<OwnershipBeneficiaryRate1>,
	#[serde(rename = "ClntId", skip_serializing_if = "Option::is_none")]
	pub clnt_id: Option<Max35Text>,
	#[serde(rename = "FsclXmptn", skip_serializing_if = "Option::is_none")]
	pub fscl_xmptn: Option<bool>,
	#[serde(rename = "SgntryRghtInd", skip_serializing_if = "Option::is_none")]
	pub sgntry_rght_ind: Option<bool>,
	#[serde(rename = "MiFIDClssfctn", skip_serializing_if = "Option::is_none")]
	pub mi_fid_clssfctn: Option<MiFIDClassification1>,
	#[serde(rename = "Ntfctn", skip_serializing_if = "Option::is_none")]
	pub ntfctn: Option<Vec<Notification2>>,
	#[serde(rename = "FATCAFormTp", skip_serializing_if = "Option::is_none")]
	pub fatca_form_tp: Option<Vec<FATCAForm1Choice>>,
	#[serde(rename = "FATCASts", skip_serializing_if = "Option::is_none")]
	pub fatca_sts: Option<Vec<FATCAStatus2>>,
	#[serde(rename = "FATCARptgDt", skip_serializing_if = "Option::is_none")]
	pub fatca_rptg_dt: Option<String>,
	#[serde(rename = "CRSFormTp", skip_serializing_if = "Option::is_none")]
	pub crs_form_tp: Option<Vec<CRSForm1Choice>>,
	#[serde(rename = "CRSSts", skip_serializing_if = "Option::is_none")]
	pub crs_sts: Option<Vec<CRSStatus4>>,
	#[serde(rename = "CRSRptgDt", skip_serializing_if = "Option::is_none")]
	pub crs_rptg_dt: Option<String>,
	#[serde(rename = "OthrId", skip_serializing_if = "Option::is_none")]
	pub othr_id: Option<Vec<GenericIdentification82>>,
	#[serde(rename = "TaxXmptn", skip_serializing_if = "Option::is_none")]
	pub tax_xmptn: Option<TaxExemptionReason2Choice>,
	#[serde(rename = "TaxRptg", skip_serializing_if = "Option::is_none")]
	pub tax_rptg: Option<Vec<TaxReporting3>>,
	#[serde(rename = "Lang", skip_serializing_if = "Option::is_none")]
	pub lang: Option<String>,
	#[serde(rename = "MailTp", skip_serializing_if = "Option::is_none")]
	pub mail_tp: Option<MailType1Choice>,
	#[serde(rename = "CtryAndResdtlSts", skip_serializing_if = "Option::is_none")]
	pub ctry_and_resdtl_sts: Option<CountryAndResidentialStatusType2>,
	#[serde(rename = "MntryWlth", skip_serializing_if = "Option::is_none")]
	pub mntry_wlth: Option<DateAndAmount1>,
	#[serde(rename = "EqtyVal", skip_serializing_if = "Option::is_none")]
	pub eqty_val: Option<DateAndAmount1>,
	#[serde(rename = "WorkgCptl", skip_serializing_if = "Option::is_none")]
	pub workg_cptl: Option<DateAndAmount1>,
	#[serde(rename = "CpnyLk", skip_serializing_if = "Option::is_none")]
	pub cpny_lk: Option<CompanyLink1Choice>,
	#[serde(rename = "ElctrncMlngSvcRef", skip_serializing_if = "Option::is_none")]
	pub elctrnc_mlng_svc_ref: Option<Max350Text>,
	#[serde(rename = "PmryComAdr", skip_serializing_if = "Option::is_none")]
	pub pmry_com_adr: Option<Vec<CommunicationAddress6>>,
	#[serde(rename = "ScndryComAdr", skip_serializing_if = "Option::is_none")]
	pub scndry_com_adr: Option<Vec<CommunicationAddress6>>,
	#[serde(rename = "AddtlRgltryInf", skip_serializing_if = "Option::is_none")]
	pub addtl_rgltry_inf: Option<RegulatoryInformation1>,
	#[serde(rename = "AcctgSts", skip_serializing_if = "Option::is_none")]
	pub acctg_sts: Option<AccountingStatus1Choice>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<AdditiononalInformation13>>,
	#[serde(rename = "CtrlgPty", skip_serializing_if = "Option::is_none")]
	pub ctrlg_pty: Option<bool>,
}


// InvestmentFundOrder4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentFundOrder4 {
	#[serde(rename = "OrdrRef", skip_serializing_if = "Option::is_none")]
	pub ordr_ref: Option<Max35Text>,
	#[serde(rename = "MstrRef", skip_serializing_if = "Option::is_none")]
	pub mstr_ref: Option<Max35Text>,
}


// InvestmentFundRole6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InvestmentFundRole6Code {
	#[serde(rename = "CACO")]
	CodeCACO,
	#[serde(rename = "CONC")]
	CodeCONC,
	#[serde(rename = "CUST")]
	CodeCUST,
	#[serde(rename = "DATP")]
	CodeDATP,
	#[serde(rename = "DIST")]
	CodeDIST,
	#[serde(rename = "FACT")]
	CodeFACT,
	#[serde(rename = "FIAD")]
	CodeFIAD,
	#[serde(rename = "FIAG")]
	CodeFIAG,
	#[serde(rename = "FMCO")]
	CodeFMCO,
	#[serde(rename = "FNBR")]
	CodeFNBR,
	#[serde(rename = "FTAG")]
	CodeFTAG,
	#[serde(rename = "INTR")]
	CodeINTR,
	#[serde(rename = "INVE")]
	CodeINVE,
	#[serde(rename = "INVS")]
	CodeINVS,
	#[serde(rename = "PAYI")]
	CodePAYI,
	#[serde(rename = "REGI")]
	CodeREGI,
	#[serde(rename = "TRAG")]
	CodeTRAG,
	#[serde(rename = "TRAN")]
	CodeTRAN,

	#[default]
	UNKOWN
}


// InvestmentFundRole7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InvestmentFundRole7Code {
	#[serde(rename = "CONC")]
	CodeCONC,
	#[serde(rename = "DIST")]
	CodeDIST,
	#[serde(rename = "FMCO")]
	CodeFMCO,
	#[serde(rename = "INTR")]
	CodeINTR,
	#[serde(rename = "PAYI")]
	CodePAYI,
	#[serde(rename = "TRAG")]
	CodeTRAG,
	#[serde(rename = "CUST")]
	CodeCUST,
	#[serde(rename = "CACO")]
	CodeCACO,
	#[serde(rename = "FACT")]
	CodeFACT,
	#[serde(rename = "INVE")]
	CodeINVE,
	#[serde(rename = "INVS")]
	CodeINVS,

	#[default]
	UNKOWN
}


// InvestmentFundTransactionType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InvestmentFundTransactionType1Code {
	#[serde(rename = "ALLL")]
	CodeALLL,
	#[serde(rename = "SELL")]
	CodeSELL,
	#[serde(rename = "BUYI")]
	CodeBUYI,
	#[serde(rename = "SWIO")]
	CodeSWIO,
	#[serde(rename = "TRIN")]
	CodeTRIN,
	#[serde(rename = "TOUT")]
	CodeTOUT,
	#[serde(rename = "SUBS")]
	CodeSUBS,
	#[serde(rename = "REDM")]
	CodeREDM,
	#[serde(rename = "CDEP")]
	CodeCDEP,
	#[serde(rename = "CWIT")]
	CodeCWIT,
	#[serde(rename = "DIVP")]
	CodeDIVP,
	#[serde(rename = "CAEV")]
	CodeCAEV,
	#[serde(rename = "CROI")]
	CodeCROI,
	#[serde(rename = "CROO")]
	CodeCROO,
	#[serde(rename = "DIVI")]
	CodeDIVI,
	#[serde(rename = "INSP")]
	CodeINSP,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "REAA")]
	CodeREAA,
	#[serde(rename = "RWPL")]
	CodeRWPL,
	#[serde(rename = "RDIV")]
	CodeRDIV,
	#[serde(rename = "SSPL")]
	CodeSSPL,
	#[serde(rename = "SUAA")]
	CodeSUAA,

	#[default]
	UNKOWN
}


// InvestmentPlan17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentPlan17 {
	#[serde(rename = "Frqcy")]
	pub frqcy: Frequency20Choice,
	#[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
	pub start_dt: Option<String>,
	#[serde(rename = "EndDt", skip_serializing_if = "Option::is_none")]
	pub end_dt: Option<String>,
	#[serde(rename = "Qty")]
	pub qty: UnitsOrAmount1Choice,
	#[serde(rename = "GrssAmtInd", skip_serializing_if = "Option::is_none")]
	pub grss_amt_ind: Option<bool>,
	#[serde(rename = "IncmPref", skip_serializing_if = "Option::is_none")]
	pub incm_pref: Option<IncomePreference2Code>,
	#[serde(rename = "InitlAmt", skip_serializing_if = "Option::is_none")]
	pub initl_amt: Option<InitialAmount1Choice>,
	#[serde(rename = "TtlNbOfInstlmts", skip_serializing_if = "Option::is_none")]
	pub ttl_nb_of_instlmts: Option<f64>,
	#[serde(rename = "RndgDrctn", skip_serializing_if = "Option::is_none")]
	pub rndg_drctn: Option<RoundingDirection1Code>,
	#[serde(rename = "SctyDtls")]
	pub scty_dtls: Vec<Repartition6>,
	#[serde(rename = "CshSttlm", skip_serializing_if = "Option::is_none")]
	pub csh_sttlm: Option<Vec<CashSettlement3>>,
	#[serde(rename = "CtrctRef", skip_serializing_if = "Option::is_none")]
	pub ctrct_ref: Option<Max35Text>,
	#[serde(rename = "RltdCtrctRef", skip_serializing_if = "Option::is_none")]
	pub rltd_ctrct_ref: Option<Max35Text>,
	#[serde(rename = "PdctId", skip_serializing_if = "Option::is_none")]
	pub pdct_id: Option<Max35Text>,
	#[serde(rename = "SLAChrgAndComssnRef", skip_serializing_if = "Option::is_none")]
	pub sla_chrg_and_comssn_ref: Option<Max35Text>,
	#[serde(rename = "InsrncCover", skip_serializing_if = "Option::is_none")]
	pub insrnc_cover: Option<InsuranceType2Choice>,
	#[serde(rename = "PlanSts", skip_serializing_if = "Option::is_none")]
	pub plan_sts: Option<PlanStatus2Choice>,
	#[serde(rename = "InstlmtMgrRole", skip_serializing_if = "Option::is_none")]
	pub instlmt_mgr_role: Option<PartyRole4Choice>,
}


// InvestorProfile2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestorProfile2 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<ProfileType1Choice>,
	#[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
	pub sts: Option<InvestorProfileStatus1Choice>,
	#[serde(rename = "Trsr", skip_serializing_if = "Option::is_none")]
	pub trsr: Option<TreasuryProfile1>,
	#[serde(rename = "HghFrqcyTradg", skip_serializing_if = "Option::is_none")]
	pub hgh_frqcy_tradg: Option<HighFrequencyTradingProfile1>,
	#[serde(rename = "MktMakr", skip_serializing_if = "Option::is_none")]
	pub mkt_makr: Option<MarketMakerProfile2>,
}


// InvestorProfileStatus1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestorProfileStatus1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InvestorProfileStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// InvestorProfileStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InvestorProfileStatus1Code {
	#[serde(rename = "DISA")]
	CodeDISA,
	#[serde(rename = "DISG")]
	CodeDISG,
	#[serde(rename = "ENAB")]
	CodeENAB,
	#[serde(rename = "ENBG")]
	CodeENBG,
	#[serde(rename = "ADMI")]
	CodeADMI,
	#[serde(rename = "ANLY")]
	CodeANLY,
	#[serde(rename = "NAPP")]
	CodeNAPP,
	#[serde(rename = "PSUS")]
	CodePSUS,
	#[serde(rename = "PEND")]
	CodePEND,
	#[serde(rename = "SUPS")]
	CodeSUPS,

	#[default]
	UNKOWN
}


// IrishNSCIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IrishNSCIdentifier {
	#[serde(rename = "$value")]
	pub irish_nsc_identifier: String,
}


// ItalianDomesticIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ItalianDomesticIdentifier {
	#[serde(rename = "$value")]
	pub italian_domestic_identifier: String,
}


// KYCCheckType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct KYCCheckType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<KnowYourCustomerCheckType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// KnowYourCustomerCheckType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum KnowYourCustomerCheckType1Code {
	#[serde(rename = "ENHA")]
	CodeENHA,
	#[serde(rename = "ORDN")]
	CodeORDN,
	#[serde(rename = "SIMP")]
	CodeSIMP,

	#[default]
	UNKOWN
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}


// LanguageCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LanguageCode {
	#[serde(rename = "$value")]
	pub language_code: String,
}


// LetterIntent1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LetterIntent1 {
	#[serde(rename = "LttrInttRef")]
	pub lttr_intt_ref: Max35Text,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
	#[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
	pub start_dt: Option<String>,
	#[serde(rename = "EndDt", skip_serializing_if = "Option::is_none")]
	pub end_dt: Option<String>,
}


// LevelOfControl1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LevelOfControl1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<LevelOfControl1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// LevelOfControl1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum LevelOfControl1Code {
	#[serde(rename = "TRAN")]
	CodeTRAN,
	#[serde(rename = "VIEW")]
	CodeVIEW,

	#[default]
	UNKOWN
}


// Liability1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Liability1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Liability1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// Liability1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Liability1Code {
	#[serde(rename = "INVE")]
	CodeINVE,
	#[serde(rename = "BROK")]
	CodeBROK,

	#[default]
	UNKOWN
}


// MICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MICIdentifier {
	#[serde(rename = "$value")]
	pub mic_identifier: String,
}


// MailType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MailType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<MailType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// MailType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum MailType1Code {
	#[serde(rename = "AIRM")]
	CodeAIRM,
	#[serde(rename = "ORDM")]
	CodeORDM,
	#[serde(rename = "REGM")]
	CodeREGM,

	#[default]
	UNKOWN
}


// MarketMakerProfile2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketMakerProfile2 {
	#[serde(rename = "CtrctPrd", skip_serializing_if = "Option::is_none")]
	pub ctrct_prd: Option<DateTimePeriod2>,
	#[serde(rename = "Cmplc", skip_serializing_if = "Option::is_none")]
	pub cmplc: Option<bool>,
	#[serde(rename = "MaxSprd", skip_serializing_if = "Option::is_none")]
	pub max_sprd: Option<f64>,
	#[serde(rename = "Dscnt", skip_serializing_if = "Option::is_none")]
	pub dscnt: Option<f64>,
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


// Max10Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max10Text {
	#[serde(rename = "$value")]
	pub max10_text: String,
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


// Max3Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max3Text {
	#[serde(rename = "$value")]
	pub max3_text: String,
}


// Max4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "$value")]
	pub max4_alpha_numeric_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
}


// MessageIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
}


// MiFIDClassification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MiFIDClassification1 {
	#[serde(rename = "Clssfctn")]
	pub clssfctn: OrderOriginatorEligibility1Code,
	#[serde(rename = "Nrrtv", skip_serializing_if = "Option::is_none")]
	pub nrrtv: Option<Max350Text>,
}


// MoneyLaunderingCheck1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MoneyLaunderingCheck1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<MoneyLaunderingCheck1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// MoneyLaunderingCheck1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum MoneyLaunderingCheck1Code {
	#[serde(rename = "PASS")]
	CodePASS,
	#[serde(rename = "NOTC")]
	CodeNOTC,
	#[serde(rename = "EXEM")]
	CodeEXEM,
	#[serde(rename = "CLMO")]
	CodeCLMO,
	#[serde(rename = "AUTH")]
	CodeAUTH,
	#[serde(rename = "POEP")]
	CodePOEP,

	#[default]
	UNKOWN
}


// NameAndAddress4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress4 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max350Text>,
	#[serde(rename = "Adr")]
	pub adr: PostalAddress1,
}


// NameAndAddress5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<PostalAddress1>,
}


// NamePrefix1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NamePrefix1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<NamePrefix1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// NamePrefix1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NamePrefix1Code {
	#[serde(rename = "DOCT")]
	CodeDOCT,
	#[serde(rename = "MIST")]
	CodeMIST,
	#[serde(rename = "MISS")]
	CodeMISS,
	#[serde(rename = "MADM")]
	CodeMADM,

	#[default]
	UNKOWN
}


// NationalityCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NationalityCode {
	#[serde(rename = "$value")]
	pub nationality_code: String,
}


// NewIssueAllocation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NewIssueAllocation2 {
	#[serde(rename = "Rstrctd")]
	pub rstrctd: bool,
	#[serde(rename = "XmptPrsnRsn", skip_serializing_if = "Option::is_none")]
	pub xmpt_prsn_rsn: Option<Max350Text>,
	#[serde(rename = "DeMnms", skip_serializing_if = "Option::is_none")]
	pub de_mnms: Option<DeMinimus1Choice>,
}


// NewZealandNCCIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NewZealandNCCIdentifier {
	#[serde(rename = "$value")]
	pub new_zealand_ncc_identifier: String,
}


// NoReasonCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NoReasonCode {
	#[serde(rename = "NORE")]
	CodeNORE,

	#[default]
	UNKOWN
}


// Notification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Notification2 {
	#[serde(rename = "NtfctnTp")]
	pub ntfctn_tp: Max35Text,
	#[serde(rename = "Reqrd")]
	pub reqrd: bool,
	#[serde(rename = "DstrbtnTp", skip_serializing_if = "Option::is_none")]
	pub dstrbtn_tp: Option<InformationDistribution1Choice>,
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "$value")]
	pub number: f64,
}


// OperationalStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OperationalStatus1Code {
	#[serde(rename = "ENAB")]
	CodeENAB,
	#[serde(rename = "SPEC")]
	CodeSPEC,

	#[default]
	UNKOWN
}


// OrderOriginatorEligibility1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OrderOriginatorEligibility1Code {
	#[serde(rename = "ELIG")]
	CodeELIG,
	#[serde(rename = "RETL")]
	CodeRETL,
	#[serde(rename = "PROF")]
	CodePROF,

	#[default]
	UNKOWN
}


// Organisation23 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Organisation23 {
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
	pub shrt_nm: Option<Max35Text>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Vec<PostalAddress21>,
}


// Organisation39 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Organisation39 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max350Text>,
	#[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
	pub shrt_nm: Option<Max35Text>,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<PartyIdentification177Choice>,
	#[serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none")]
	pub lgl_ntty_idr: Option<LEIIdentifier>,
	#[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
	pub purp: Option<Max35Text>,
	#[serde(rename = "RegnCtry", skip_serializing_if = "Option::is_none")]
	pub regn_ctry: Option<CountryCode>,
	#[serde(rename = "RegnDt", skip_serializing_if = "Option::is_none")]
	pub regn_dt: Option<String>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<Vec<PostalAddress21>>,
	#[serde(rename = "TpOfOrg", skip_serializing_if = "Option::is_none")]
	pub tp_of_org: Option<OrganisationType1Choice>,
	#[serde(rename = "PlcOfListg", skip_serializing_if = "Option::is_none")]
	pub plc_of_listg: Option<Vec<MICIdentifier>>,
}


// OrganisationType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<OrganisationType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// OrganisationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OrganisationType1Code {
	#[serde(rename = "IFUN")]
	CodeIFUN,
	#[serde(rename = "PRIV")]
	CodePRIV,
	#[serde(rename = "PUBL")]
	CodePUBL,
	#[serde(rename = "PFUN")]
	CodePFUN,

	#[default]
	UNKOWN
}


// OtherAccountStatus1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherAccountStatus1 {
	#[serde(rename = "Sts")]
	pub sts: GenericIdentification36,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<GenericIdentification36>,
}


// OtherIdentification3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherIdentification3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PartyIdentificationType7Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// OwnershipBeneficiaryRate1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OwnershipBeneficiaryRate1 {
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "Frctn", skip_serializing_if = "Option::is_none")]
	pub frctn: Option<Max35Text>,
}


// OwnershipType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OwnershipType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<AccountOwnershipType4Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// Party47Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party47Choice {
	#[serde(rename = "Org", skip_serializing_if = "Option::is_none")]
	pub org: Option<Organisation39>,
	#[serde(rename = "IndvPrsn", skip_serializing_if = "Option::is_none")]
	pub indv_prsn: Option<IndividualPerson37>,
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


// PartyIdentification177Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification177Choice {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification1>,
}


// PartyIdentificationType7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PartyIdentificationType7Code {
	#[serde(rename = "ATIN")]
	CodeATIN,
	#[serde(rename = "IDCD")]
	CodeIDCD,
	#[serde(rename = "NRIN")]
	CodeNRIN,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "PASS")]
	CodePASS,
	#[serde(rename = "POCD")]
	CodePOCD,
	#[serde(rename = "SOCS")]
	CodeSOCS,
	#[serde(rename = "SRSA")]
	CodeSRSA,
	#[serde(rename = "GUNL")]
	CodeGUNL,
	#[serde(rename = "GTIN")]
	CodeGTIN,
	#[serde(rename = "ITIN")]
	CodeITIN,
	#[serde(rename = "CPFA")]
	CodeCPFA,
	#[serde(rename = "AREG")]
	CodeAREG,
	#[serde(rename = "DRLC")]
	CodeDRLC,
	#[serde(rename = "EMID")]
	CodeEMID,
	#[serde(rename = "NINV")]
	CodeNINV,
	#[serde(rename = "INCL")]
	CodeINCL,
	#[serde(rename = "GIIN")]
	CodeGIIN,

	#[default]
	UNKOWN
}


// PartyProfileInformation5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyProfileInformation5 {
	#[serde(rename = "CertfctnInd", skip_serializing_if = "Option::is_none")]
	pub certfctn_ind: Option<bool>,
	#[serde(rename = "VldtngPty", skip_serializing_if = "Option::is_none")]
	pub vldtng_pty: Option<Max140Text>,
	#[serde(rename = "ChckngPty", skip_serializing_if = "Option::is_none")]
	pub chckng_pty: Option<Max140Text>,
	#[serde(rename = "RspnsblPty", skip_serializing_if = "Option::is_none")]
	pub rspnsbl_pty: Option<Max140Text>,
	#[serde(rename = "CertTp", skip_serializing_if = "Option::is_none")]
	pub cert_tp: Option<CertificationType1Choice>,
	#[serde(rename = "ChckngDt", skip_serializing_if = "Option::is_none")]
	pub chckng_dt: Option<String>,
	#[serde(rename = "ChckngFrqcy", skip_serializing_if = "Option::is_none")]
	pub chckng_frqcy: Option<EventFrequency1Code>,
	#[serde(rename = "NxtRvsnDt", skip_serializing_if = "Option::is_none")]
	pub nxt_rvsn_dt: Option<String>,
	#[serde(rename = "SlryRg", skip_serializing_if = "Option::is_none")]
	pub slry_rg: Option<Max35Text>,
	#[serde(rename = "SrcOfWlth", skip_serializing_if = "Option::is_none")]
	pub src_of_wlth: Option<Max140Text>,
	#[serde(rename = "CstmrCndctClssfctn", skip_serializing_if = "Option::is_none")]
	pub cstmr_cndct_clssfctn: Option<CustomerConductClassification1Choice>,
	#[serde(rename = "RskLvl", skip_serializing_if = "Option::is_none")]
	pub rsk_lvl: Option<RiskLevel2Choice>,
	#[serde(rename = "KnowYourCstmrChckTp", skip_serializing_if = "Option::is_none")]
	pub know_your_cstmr_chck_tp: Option<KYCCheckType1Choice>,
	#[serde(rename = "KnowYourCstmrDBChck", skip_serializing_if = "Option::is_none")]
	pub know_your_cstmr_db_chck: Option<DataBaseCheck1>,
}


// PartyRole1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PartyRole1Code {
	#[serde(rename = "CUST")]
	CodeCUST,
	#[serde(rename = "INVS")]
	CodeINVS,

	#[default]
	UNKOWN
}


// PartyRole2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyRole2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InvestmentFundRole6Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// PartyRole4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyRole4Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InvestmentFundRole7Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// PartyRole5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyRole5Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PartyRole1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// PaymentCard29 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentCard29 {
	#[serde(rename = "Tp")]
	pub tp: CardType1Code,
	#[serde(rename = "Nb")]
	pub nb: Max35Text,
	#[serde(rename = "HldrNm")]
	pub hldr_nm: Max35Text,
	#[serde(rename = "StartDt", skip_serializing_if = "Option::is_none")]
	pub start_dt: Option<String>,
	#[serde(rename = "XpryDt")]
	pub xpry_dt: String,
	#[serde(rename = "CardIssrNm", skip_serializing_if = "Option::is_none")]
	pub card_issr_nm: Option<Max35Text>,
	#[serde(rename = "CardIssrId", skip_serializing_if = "Option::is_none")]
	pub card_issr_id: Option<PartyIdentification125Choice>,
	#[serde(rename = "SctyCd", skip_serializing_if = "Option::is_none")]
	pub scty_cd: Option<Max35Text>,
	#[serde(rename = "SeqNb", skip_serializing_if = "Option::is_none")]
	pub seq_nb: Option<Max3Text>,
}


// PaymentInstrument17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentInstrument17 {
	#[serde(rename = "SttlmCcy")]
	pub sttlm_ccy: ActiveCurrencyCode,
	#[serde(rename = "DvddPctg", skip_serializing_if = "Option::is_none")]
	pub dvdd_pctg: Option<PercentageBoundedRate>,
	#[serde(rename = "SbcptPmtInstrm", skip_serializing_if = "Option::is_none")]
	pub sbcpt_pmt_instrm: Option<PaymentInstrument24Choice>,
	#[serde(rename = "RedPmtInstrm", skip_serializing_if = "Option::is_none")]
	pub red_pmt_instrm: Option<PaymentInstrument19Choice>,
	#[serde(rename = "DvddPmtInstrm", skip_serializing_if = "Option::is_none")]
	pub dvdd_pmt_instrm: Option<PaymentInstrument19Choice>,
	#[serde(rename = "SvgsPlanPmtInstrm", skip_serializing_if = "Option::is_none")]
	pub svgs_plan_pmt_instrm: Option<PaymentInstrument24Choice>,
	#[serde(rename = "IntrstPmtInstrm", skip_serializing_if = "Option::is_none")]
	pub intrst_pmt_instrm: Option<PaymentInstrument19Choice>,
}


// PaymentInstrument19Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentInstrument19Choice {
	#[serde(rename = "ChqDtls", skip_serializing_if = "Option::is_none")]
	pub chq_dtls: Option<Cheque4>,
	#[serde(rename = "BkrsDrftDtls", skip_serializing_if = "Option::is_none")]
	pub bkrs_drft_dtls: Option<Cheque4>,
}


// PaymentInstrument24Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentInstrument24Choice {
	#[serde(rename = "PmtCardDtls", skip_serializing_if = "Option::is_none")]
	pub pmt_card_dtls: Option<PaymentCard29>,
	#[serde(rename = "DrctDbtDtls", skip_serializing_if = "Option::is_none")]
	pub drct_dbt_dtls: Option<DirectDebitMandate7>,
	#[serde(rename = "Chq", skip_serializing_if = "Option::is_none")]
	pub chq: Option<bool>,
	#[serde(rename = "BkrsDrft", skip_serializing_if = "Option::is_none")]
	pub bkrs_drft: Option<bool>,
}


// PendingOpeningStatusReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingOpeningStatusReason1 {
	#[serde(rename = "Cd")]
	pub cd: PendingOpeningStatusReason2Choice,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
}


// PendingOpeningStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingOpeningStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<PendingOpeningStatusReason1>>,
}


// PendingOpeningStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PendingOpeningStatusReason1Code {
	#[serde(rename = "ATHR")]
	CodeATHR,
	#[serde(rename = "ATHP")]
	CodeATHP,
	#[serde(rename = "FRDM")]
	CodeFRDM,
	#[serde(rename = "KYCM")]
	CodeKYCM,
	#[serde(rename = "NOTO")]
	CodeNOTO,
	#[serde(rename = "REST")]
	CodeREST,
	#[serde(rename = "RIGH")]
	CodeRIGH,

	#[default]
	UNKOWN
}


// PendingOpeningStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingOpeningStatusReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PendingOpeningStatusReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}


// PendingStatusReason14 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingStatusReason14 {
	#[serde(rename = "Cd")]
	pub cd: PendingStatusReason2Choice,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
}


// PendingStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<PendingStatusReason14>>,
}


// PendingStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PendingStatusReason1Code {
	#[serde(rename = "KYCM")]
	CodeKYCM,
	#[serde(rename = "FRDM")]
	CodeFRDM,
	#[serde(rename = "RIGH")]
	CodeRIGH,
	#[serde(rename = "ATHR")]
	CodeATHR,
	#[serde(rename = "ATHP")]
	CodeATHP,
	#[serde(rename = "MODI")]
	CodeMODI,

	#[default]
	UNKOWN
}


// PendingStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PendingStatusReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PendingStatusReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}


// PercentageBoundedRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageBoundedRate {
	#[serde(rename = "$value")]
	pub percentage_bounded_rate: f64,
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "$value")]
	pub percentage_rate: f64,
}


// PersonalInformation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonalInformation1 {
	#[serde(rename = "NmOfFthr", skip_serializing_if = "Option::is_none")]
	pub nm_of_fthr: Option<Max35Text>,
	#[serde(rename = "MdnNmOfMthr", skip_serializing_if = "Option::is_none")]
	pub mdn_nm_of_mthr: Option<Max35Text>,
	#[serde(rename = "NmOfPrtnr", skip_serializing_if = "Option::is_none")]
	pub nm_of_prtnr: Option<Max35Text>,
}


// PhoneNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PhoneNumber {
	#[serde(rename = "$value")]
	pub phone_number: String,
}


// PlanStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PlanStatus1Code {
	#[serde(rename = "ACTV")]
	CodeACTV,
	#[serde(rename = "CLOS")]
	CodeCLOS,
	#[serde(rename = "SUSP")]
	CodeSUSP,

	#[default]
	UNKOWN
}


// PlanStatus2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlanStatus2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PlanStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// PoliticalExposureType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PoliticalExposureType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PoliticalExposureType2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// PoliticalExposureType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PoliticalExposureType2Code {
	#[serde(rename = "NPEX")]
	CodeNPEX,
	#[serde(rename = "YPEX")]
	CodeYPEX,
	#[serde(rename = "PEXD")]
	CodePEXD,
	#[serde(rename = "PEXF")]
	CodePEXF,

	#[default]
	UNKOWN
}


// PoliticallyExposedPerson1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PoliticallyExposedPerson1 {
	#[serde(rename = "PltclyXpsdPrsnTp")]
	pub pltcly_xpsd_prsn_tp: PoliticalExposureType2Choice,
	#[serde(rename = "PltclyXpsdPrsnSts", skip_serializing_if = "Option::is_none")]
	pub pltcly_xpsd_prsn_sts: Option<PoliticallyExposedPersonStatus1Choice>,
}


// PoliticallyExposedPersonStatus1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PoliticallyExposedPersonStatus1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PoliticallyExposedPersonStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// PoliticallyExposedPersonStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PoliticallyExposedPersonStatus1Code {
	#[serde(rename = "PE03")]
	CodePE03,
	#[serde(rename = "PE01")]
	CodePE01,
	#[serde(rename = "PE02")]
	CodePE02,

	#[default]
	UNKOWN
}


// PortugueseNCCIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortugueseNCCIdentifier {
	#[serde(rename = "$value")]
	pub portuguese_ncc_identifier: String,
}


// PositionEffect3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PositionEffect3Code {
	#[serde(rename = "FIFO")]
	CodeFIFO,
	#[serde(rename = "LIFO")]
	CodeLIFO,

	#[default]
	UNKOWN
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


// PostalAddress21 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress21 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<AddressType2Choice>,
	#[serde(rename = "MlngInd", skip_serializing_if = "Option::is_none")]
	pub mlng_ind: Option<bool>,
	#[serde(rename = "RegnAdrInd", skip_serializing_if = "Option::is_none")]
	pub regn_adr_ind: Option<bool>,
	#[serde(rename = "CareOf", skip_serializing_if = "Option::is_none")]
	pub care_of: Option<Max70Text>,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
	pub adr_line: Option<Vec<Max70Text>>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<Max70Text>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<Max16Text>,
	#[serde(rename = "BldgNm", skip_serializing_if = "Option::is_none")]
	pub bldg_nm: Option<Max35Text>,
	#[serde(rename = "PstBx", skip_serializing_if = "Option::is_none")]
	pub pst_bx: Option<Max10Text>,
	#[serde(rename = "SdInBldg", skip_serializing_if = "Option::is_none")]
	pub sd_in_bldg: Option<Max35Text>,
	#[serde(rename = "Flr", skip_serializing_if = "Option::is_none")]
	pub flr: Option<Max70Text>,
	#[serde(rename = "SuiteId", skip_serializing_if = "Option::is_none")]
	pub suite_id: Option<Max10Text>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<Max16Text>,
	#[serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none")]
	pub dstrct_nm: Option<Max35Text>,
	#[serde(rename = "Vllg", skip_serializing_if = "Option::is_none")]
	pub vllg: Option<Max70Text>,
	#[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
	pub twn_nm: Option<Max35Text>,
	#[serde(rename = "Stat", skip_serializing_if = "Option::is_none")]
	pub stat: Option<Max70Text>,
	#[serde(rename = "Ctry")]
	pub ctry: CountryCode,
}


// ProfileType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProfileType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ProfileType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// ProfileType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ProfileType1Code {
	#[serde(rename = "HEDG")]
	CodeHEDG,
	#[serde(rename = "HFTR")]
	CodeHFTR,
	#[serde(rename = "MAKE")]
	CodeMAKE,
	#[serde(rename = "TREA")]
	CodeTREA,

	#[default]
	UNKOWN
}


// ProformaStatusReason1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProformaStatusReason1 {
	#[serde(rename = "Cd")]
	pub cd: ProformaStatusReason2Choice,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
}


// ProformaStatusReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProformaStatusReason1Choice {
	#[serde(rename = "NoSpcfdRsn", skip_serializing_if = "Option::is_none")]
	pub no_spcfd_rsn: Option<NoReasonCode>,
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<Vec<ProformaStatusReason1>>,
}


// ProformaStatusReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ProformaStatusReason1Code {
	#[serde(rename = "MODI")]
	CodeMODI,
	#[serde(rename = "RIGH")]
	CodeRIGH,

	#[default]
	UNKOWN
}


// ProformaStatusReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProformaStatusReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ProformaStatusReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification36>,
}


// Provided1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Provided1Code {
	#[serde(rename = "NPRO")]
	CodeNPRO,
	#[serde(rename = "PROV")]
	CodePROV,

	#[default]
	UNKOWN
}


// QUICKIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct QUICKIdentifier {
	#[serde(rename = "$value")]
	pub quick_identifier: String,
}


// RICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RICIdentifier {
	#[serde(rename = "$value")]
	pub ric_identifier: String,
}


// Rank1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Rank1Code {
	#[serde(rename = "PRIM")]
	CodePRIM,
	#[serde(rename = "SECO")]
	CodeSECO,

	#[default]
	UNKOWN
}


// Referred1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Referred1Code {
	#[serde(rename = "REFR")]
	CodeREFR,
	#[serde(rename = "NRFR")]
	CodeNRFR,
	#[serde(rename = "UKNW")]
	CodeUKNW,

	#[default]
	UNKOWN
}


// ReferredAgent3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReferredAgent3 {
	#[serde(rename = "Rfrd")]
	pub rfrd: Referred1Code,
	#[serde(rename = "RfrdPlcmntAgt", skip_serializing_if = "Option::is_none")]
	pub rfrd_plcmnt_agt: Option<PartyIdentification125Choice>,
}


// RegisteredShareholderName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegisteredShareholderName1Choice {
	#[serde(rename = "IndvPrsn", skip_serializing_if = "Option::is_none")]
	pub indv_prsn: Option<IndividualPerson29>,
	#[serde(rename = "Org", skip_serializing_if = "Option::is_none")]
	pub org: Option<Organisation23>,
}


// RegulatoryInformation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RegulatoryInformation1 {
	#[serde(rename = "Sctr", skip_serializing_if = "Option::is_none")]
	pub sctr: Option<Max35Text>,
	#[serde(rename = "Brnch", skip_serializing_if = "Option::is_none")]
	pub brnch: Option<Max35Text>,
	#[serde(rename = "Grp", skip_serializing_if = "Option::is_none")]
	pub grp: Option<Max35Text>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Max35Text>,
}


// Reinvestment4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Reinvestment4 {
	#[serde(rename = "FinInstrmDtls")]
	pub fin_instrm_dtls: FinancialInstrument87,
	#[serde(rename = "ReqdNAVCcy", skip_serializing_if = "Option::is_none")]
	pub reqd_nav_ccy: Option<ActiveCurrencyCode>,
	#[serde(rename = "RinvstmtPctg")]
	pub rinvstmt_pctg: f64,
}


// Repartition6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Repartition6 {
	#[serde(rename = "Qty")]
	pub qty: UnitsOrAmountOrPercentage1Choice,
	#[serde(rename = "FinInstrm")]
	pub fin_instrm: FinancialInstrument87,
	#[serde(rename = "CcyOfPlan", skip_serializing_if = "Option::is_none")]
	pub ccy_of_plan: Option<ActiveOrHistoricCurrencyCode>,
}


// ResidentialStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ResidentialStatus1Code {
	#[serde(rename = "RESI")]
	CodeRESI,
	#[serde(rename = "PRES")]
	CodePRES,
	#[serde(rename = "NRES")]
	CodeNRES,

	#[default]
	UNKOWN
}


// RestrictionStatus1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RestrictionStatus1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<RestrictionStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// RestrictionStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RestrictionStatus1Code {
	#[serde(rename = "ACTV")]
	CodeACTV,
	#[serde(rename = "INAC")]
	CodeINAC,

	#[default]
	UNKOWN
}


// RiskLevel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RiskLevel1Code {
	#[serde(rename = "HIGH")]
	CodeHIGH,
	#[serde(rename = "LOWW")]
	CodeLOWW,
	#[serde(rename = "MEDM")]
	CodeMEDM,

	#[default]
	UNKOWN
}


// RiskLevel2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RiskLevel2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<RiskLevel1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// RoundingDirection1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RoundingDirection1Code {
	#[serde(rename = "RDUP")]
	CodeRDUP,
	#[serde(rename = "RDWN")]
	CodeRDWN,
	#[serde(rename = "STAN")]
	CodeSTAN,
	#[serde(rename = "DIST")]
	CodeDIST,

	#[default]
	UNKOWN
}


// RoundingParameters1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RoundingParameters1 {
	#[serde(rename = "RndgMdlus", skip_serializing_if = "Option::is_none")]
	pub rndg_mdlus: Option<f64>,
	#[serde(rename = "RndgDrctn")]
	pub rndg_drctn: RoundingDirection1Code,
}


// RussianCentralBankIdentificationCodeIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RussianCentralBankIdentificationCodeIdentifier {
	#[serde(rename = "$value")]
	pub russian_central_bank_identification_code_identifier: String,
}


// SEDOLIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SEDOLIdentifier {
	#[serde(rename = "$value")]
	pub sedol_identifier: String,
}


// SecurityIdentification25Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification25Choice {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
	#[serde(rename = "SEDOL", skip_serializing_if = "Option::is_none")]
	pub sedol: Option<String>,
	#[serde(rename = "CUSIP", skip_serializing_if = "Option::is_none")]
	pub cusip: Option<String>,
	#[serde(rename = "RIC", skip_serializing_if = "Option::is_none")]
	pub ric: Option<RICIdentifier>,
	#[serde(rename = "TckrSymb", skip_serializing_if = "Option::is_none")]
	pub tckr_symb: Option<TickerIdentifier>,
	#[serde(rename = "Blmbrg", skip_serializing_if = "Option::is_none")]
	pub blmbrg: Option<Bloomberg2Identifier>,
	#[serde(rename = "CTA", skip_serializing_if = "Option::is_none")]
	pub cta: Option<ConsolidatedTapeAssociationIdentifier>,
	#[serde(rename = "QUICK", skip_serializing_if = "Option::is_none")]
	pub quick: Option<String>,
	#[serde(rename = "Wrtppr", skip_serializing_if = "Option::is_none")]
	pub wrtppr: Option<String>,
	#[serde(rename = "Dtch", skip_serializing_if = "Option::is_none")]
	pub dtch: Option<String>,
	#[serde(rename = "Vlrn", skip_serializing_if = "Option::is_none")]
	pub vlrn: Option<String>,
	#[serde(rename = "SCVM", skip_serializing_if = "Option::is_none")]
	pub scvm: Option<String>,
	#[serde(rename = "Belgn", skip_serializing_if = "Option::is_none")]
	pub belgn: Option<String>,
	#[serde(rename = "Cmon", skip_serializing_if = "Option::is_none")]
	pub cmon: Option<EuroclearClearstreamIdentifier>,
	#[serde(rename = "OthrPrtryId", skip_serializing_if = "Option::is_none")]
	pub othr_prtry_id: Option<AlternateSecurityIdentification7>,
}


// SettlementFrequency1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementFrequency1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<EventFrequency10Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// SettlementInstructionReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementInstructionReason1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<SettlementInstructionReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// SettlementInstructionReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SettlementInstructionReason1Code {
	#[serde(rename = "CSHI")]
	CodeCSHI,
	#[serde(rename = "ALLL")]
	CodeALLL,
	#[serde(rename = "CSHO")]
	CodeCSHO,
	#[serde(rename = "CHAR")]
	CodeCHAR,
	#[serde(rename = "DIVI")]
	CodeDIVI,
	#[serde(rename = "INTE")]
	CodeINTE,
	#[serde(rename = "SAVP")]
	CodeSAVP,
	#[serde(rename = "REDM")]
	CodeREDM,
	#[serde(rename = "SAVE")]
	CodeSAVE,
	#[serde(rename = "BUYI")]
	CodeBUYI,
	#[serde(rename = "SELL")]
	CodeSELL,
	#[serde(rename = "SUBS")]
	CodeSUBS,
	#[serde(rename = "WTHP")]
	CodeWTHP,
	#[serde(rename = "CORP")]
	CodeCORP,

	#[default]
	UNKOWN
}


// SicovamIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SicovamIdentifier {
	#[serde(rename = "$value")]
	pub sicovam_identifier: String,
}


// SimpleIdentificationInformation4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SimpleIdentificationInformation4 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
}


// SmallNetworkIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SmallNetworkIdentifier {
	#[serde(rename = "$value")]
	pub small_network_identifier: String,
}


// SouthAfricanNCCIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SouthAfricanNCCIdentifier {
	#[serde(rename = "$value")]
	pub south_african_ncc_identifier: String,
}


// SpanishDomesticInterbankingIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SpanishDomesticInterbankingIdentifier {
	#[serde(rename = "$value")]
	pub spanish_domestic_interbanking_identifier: String,
}


// StatementFrequencyReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatementFrequencyReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<EventFrequency9Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// SwissBCIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SwissBCIdentifier {
	#[serde(rename = "$value")]
	pub swiss_bc_identifier: String,
}


// SwissSICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SwissSICIdentifier {
	#[serde(rename = "$value")]
	pub swiss_sic_identifier: String,
}


// TaxExemptReason3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TaxExemptReason3Code {
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
	#[serde(rename = "FORE")]
	CodeFORE,
	#[serde(rename = "INCA")]
	CodeINCA,
	#[serde(rename = "MINO")]
	CodeMINO,
	#[serde(rename = "ASSO")]
	CodeASSO,
	#[serde(rename = "DIPL")]
	CodeDIPL,
	#[serde(rename = "DOME")]
	CodeDOME,
	#[serde(rename = "FORP")]
	CodeFORP,
	#[serde(rename = "ORDR")]
	CodeORDR,
	#[serde(rename = "PENF")]
	CodePENF,
	#[serde(rename = "REFU")]
	CodeREFU,
	#[serde(rename = "RIHO")]
	CodeRIHO,
	#[serde(rename = "ADMI")]
	CodeADMI,
	#[serde(rename = "TANR")]
	CodeTANR,
	#[serde(rename = "OANR")]
	CodeOANR,

	#[default]
	UNKOWN
}


// TaxExemptionReason2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxExemptionReason2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<TaxExemptReason3Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// TaxReporting3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxReporting3 {
	#[serde(rename = "TaxtnCtry")]
	pub taxtn_ctry: CountryCode,
	#[serde(rename = "TaxRate", skip_serializing_if = "Option::is_none")]
	pub tax_rate: Option<f64>,
	#[serde(rename = "TaxPyer", skip_serializing_if = "Option::is_none")]
	pub tax_pyer: Option<PartyIdentification125Choice>,
	#[serde(rename = "TaxRcpt", skip_serializing_if = "Option::is_none")]
	pub tax_rcpt: Option<PartyIdentification125Choice>,
	#[serde(rename = "CshAcctDtls", skip_serializing_if = "Option::is_none")]
	pub csh_acct_dtls: Option<CashAccount204>,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max350Text>,
}


// TaxWithholdingMethod3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TaxWithholdingMethod3Code {
	#[serde(rename = "MITX")]
	CodeMITX,
	#[serde(rename = "INVE")]
	CodeINVE,
	#[serde(rename = "ACCT")]
	CodeACCT,
	#[serde(rename = "EXMT")]
	CodeEXMT,
	#[serde(rename = "REPT")]
	CodeREPT,
	#[serde(rename = "CRTF")]
	CodeCRTF,
	#[serde(rename = "WHCO")]
	CodeWHCO,
	#[serde(rename = "WTHD")]
	CodeWTHD,
	#[serde(rename = "WTRE")]
	CodeWTRE,

	#[default]
	UNKOWN
}


// ThirdPartyRights2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ThirdPartyRights2 {
	#[serde(rename = "Tp")]
	pub tp: Max35Text,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<String>,
	#[serde(rename = "Hldr", skip_serializing_if = "Option::is_none")]
	pub hldr: Option<PartyIdentification125Choice>,
	#[serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none")]
	pub lgl_ntty_idr: Option<LEIIdentifier>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max350Text>,
}


// TickerIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TickerIdentifier {
	#[serde(rename = "$value")]
	pub ticker_identifier: String,
}


// TransactionChannel2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TransactionChannel2Code {
	#[serde(rename = "FIAD")]
	CodeFIAD,
	#[serde(rename = "HOBA")]
	CodeHOBA,
	#[serde(rename = "BRAN")]
	CodeBRAN,

	#[default]
	UNKOWN
}


// TransactionChannelType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionChannelType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<TransactionChannel2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// TransactionType5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionType5Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InvestmentFundTransactionType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}


// TreasuryProfile1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TreasuryProfile1 {
	#[serde(rename = "Dt")]
	pub dt: String,
	#[serde(rename = "TradrTp")]
	pub tradr_tp: PartyRole5Choice,
	#[serde(rename = "Rate")]
	pub rate: f64,
}


// UKDomesticSortCodeIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UKDomesticSortCodeIdentifier {
	#[serde(rename = "$value")]
	pub uk_domestic_sort_code_identifier: String,
}


// UnitsOrAmount1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitsOrAmount1Choice {
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
	pub unit: Option<f64>,
}


// UnitsOrAmountOrPercentage1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitsOrAmountOrPercentage1Choice {
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
	pub unit: Option<f64>,
	#[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
	pub pctg: Option<f64>,
}


// ValorenIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ValorenIdentifier {
	#[serde(rename = "$value")]
	pub valoren_identifier: String,
}


// WertpapierIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct WertpapierIdentifier {
	#[serde(rename = "$value")]
	pub wertpapier_identifier: String,
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "$value")]
	pub yes_no_indicator: bool,
}
