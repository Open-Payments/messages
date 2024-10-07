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


// Account23 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Account23 {
	#[serde(rename = "AcctId")]
	pub acct_id: String,
	#[validate]
	#[serde(rename = "RltdAcctDtls")]
	pub rltd_acct_dtls: Option<GenericIdentification1>,
}


// Account32 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Account32 {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[validate]
	#[serde(rename = "AcctSvcr")]
	pub acct_svcr: PartyIdentification125Choice,
}


// AccountDesignation1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountDesignation1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// AccountIdentification4Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountIdentification4Choice {
	#[serde(rename = "IBAN")]
	pub iban: Option<String>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<GenericAccountIdentification1>,
}


// AccountIdentificationAndName5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountIdentificationAndName5 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: AccountIdentification4Choice,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
}


// AccountModificationInstructionV08 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountModificationInstructionV08 {
	#[validate]
	#[serde(rename = "MsgId")]
	pub msg_id: MessageIdentification1,
	#[validate]
	#[serde(rename = "PrvsRef")]
	pub prvs_ref: Option<AdditionalReference13>,
	#[validate]
	#[serde(rename = "InstrDtls")]
	pub instr_dtls: Option<InvestmentAccountModification4>,
	#[validate]
	#[serde(rename = "InvstmtAcctSelctn")]
	pub invstmt_acct_selctn: AccountSelection3Choice,
	#[validate]
	#[serde(rename = "ModfdInvstmtAcct")]
	pub modfd_invstmt_acct: Option<InvestmentAccount75>,
	#[validate]
	#[serde(rename = "ModfdAcctPties")]
	pub modfd_acct_pties: Option<Vec<AccountParties18>>,
	#[validate]
	#[serde(rename = "ModfdIntrmies")]
	pub modfd_intrmies: Option<Vec<ModificationScope40>>,
	#[validate]
	#[serde(rename = "ModfdPlcmnt")]
	pub modfd_plcmnt: Option<ModificationScope43>,
	#[validate]
	#[serde(rename = "ModfdIsseAllcn")]
	pub modfd_isse_allcn: Option<ModificationScope21>,
	#[validate]
	#[serde(rename = "ModfdSvgsInvstmtPlan")]
	pub modfd_svgs_invstmt_plan: Option<Vec<ModificationScope41>>,
	#[validate]
	#[serde(rename = "ModfdWdrwlInvstmtPlan")]
	pub modfd_wdrwl_invstmt_plan: Option<Vec<ModificationScope41>>,
	#[validate]
	#[serde(rename = "ModfdCshSttlm")]
	pub modfd_csh_sttlm: Option<Vec<CashSettlement4>>,
	#[validate]
	#[serde(rename = "ModfdSvcLvlAgrmt")]
	pub modfd_svc_lvl_agrmt: Option<Vec<ModificationScope44>>,
	#[validate]
	#[serde(rename = "ModfdAddtlInf")]
	pub modfd_addtl_inf: Option<Vec<ModificationScope45>>,
	#[validate]
	#[serde(rename = "MktPrctcVrsn")]
	pub mkt_prctc_vrsn: Option<MarketPracticeVersion1>,
	#[validate]
	#[serde(rename = "Xtnsn")]
	pub xtnsn: Option<Vec<Extension1>>,
}


// AccountOwner3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountOwner3Choice {
	#[validate]
	#[serde(rename = "IndvOwnrId")]
	pub indv_ownr_id: Option<IndividualPersonIdentification3Choice>,
	#[validate]
	#[serde(rename = "OrgOwnrId")]
	pub org_ownr_id: Option<PartyIdentification220>,
}


// AccountOwnershipType4Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountOwnershipType4Code {
	#[validate(enumerate = ["UNCO", "LIPA", "ENTR", "CORP", "CUST", "EURE", "PART", "TRUS", "GOVO", "JOIT", "COMO", "JOIN", "LLCO", "NOMI", "NFPO", "ONIS", "RGIC", "SIGL"])]
	#[serde(rename = "AccountOwnershipType4Code")]
	pub account_ownership_type4_code: String,
}


// AccountParties13Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountParties13Choice {
	#[validate]
	#[serde(rename = "PmryOwnr")]
	pub pmry_ownr: Option<InvestmentAccountOwnershipInformation17>,
	#[validate]
	#[serde(rename = "Trstee")]
	pub trstee: Option<Vec<InvestmentAccountOwnershipInformation17>>,
	#[validate]
	#[serde(rename = "Nmnee")]
	pub nmnee: Option<InvestmentAccountOwnershipInformation17>,
	#[validate]
	#[serde(rename = "JntOwnr")]
	pub jnt_ownr: Option<Vec<InvestmentAccountOwnershipInformation17>>,
}


// AccountParties18 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountParties18 {
	#[serde(rename = "ModScpIndctn")]
	pub mod_scp_indctn: String,
	#[validate]
	#[serde(rename = "PrncplAcctPty")]
	pub prncpl_acct_pty: Option<AccountParties13Choice>,
	#[validate]
	#[serde(rename = "ScndryOwnr")]
	pub scndry_ownr: Option<Vec<InvestmentAccountOwnershipInformation17>>,
	#[validate]
	#[serde(rename = "Bnfcry")]
	pub bnfcry: Option<Vec<InvestmentAccountOwnershipInformation17>>,
	#[validate]
	#[serde(rename = "PwrOfAttny")]
	pub pwr_of_attny: Option<Vec<InvestmentAccountOwnershipInformation17>>,
	#[validate]
	#[serde(rename = "LglGuardn")]
	pub lgl_guardn: Option<Vec<InvestmentAccountOwnershipInformation17>>,
	#[validate]
	#[serde(rename = "CtdnForMnr")]
	pub ctdn_for_mnr: Option<Vec<InvestmentAccountOwnershipInformation17>>,
	#[validate]
	#[serde(rename = "SucssrOnDth")]
	pub sucssr_on_dth: Option<Vec<InvestmentAccountOwnershipInformation17>>,
	#[validate]
	#[serde(rename = "Admstr")]
	pub admstr: Option<Vec<InvestmentAccountOwnershipInformation17>>,
	#[validate]
	#[serde(rename = "OthrPty")]
	pub othr_pty: Option<Vec<ExtendedParty15>>,
	#[validate]
	#[serde(rename = "Grntr")]
	pub grntr: Option<Vec<InvestmentAccountOwnershipInformation17>>,
	#[validate]
	#[serde(rename = "Sttlr")]
	pub sttlr: Option<Vec<InvestmentAccountOwnershipInformation17>>,
	#[validate]
	#[serde(rename = "SnrMggOffcl")]
	pub snr_mgg_offcl: Option<Vec<InvestmentAccountOwnershipInformation17>>,
	#[validate]
	#[serde(rename = "Prtctr")]
	pub prtctr: Option<Vec<InvestmentAccountOwnershipInformation17>>,
	#[validate]
	#[serde(rename = "RegdShrhldrNm")]
	pub regd_shrhldr_nm: Option<RegisteredShareholderName1Choice>,
}


// AccountSchemeName1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// AccountSelection3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountSelection3Choice {
	#[serde(rename = "AcctId")]
	pub acct_id: Option<String>,
	#[validate]
	#[serde(rename = "OthrAcctSelctnData")]
	pub othr_acct_selctn_data: Option<InvestmentAccount76>,
}


// AccountStatusUpdateInstruction1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountStatusUpdateInstruction1 {
	#[validate]
	#[serde(rename = "UpdInstr")]
	pub upd_instr: AccountStatusUpdateInstruction1Choice,
	#[validate]
	#[serde(rename = "UpdInstrRsn")]
	pub upd_instr_rsn: Option<AccountStatusUpdateInstructionReason1Choice>,
}


// AccountStatusUpdateInstruction1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountStatusUpdateInstruction1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification36>,
}


// AccountStatusUpdateInstruction1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountStatusUpdateInstruction1Code {
	#[validate(enumerate = ["CLOS", "REAC"])]
	#[serde(rename = "AccountStatusUpdateInstruction1Code")]
	pub account_status_update_instruction1_code: String,
}


// AccountStatusUpdateInstructionReason1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountStatusUpdateInstructionReason1 {
	#[validate]
	#[serde(rename = "Cd")]
	pub cd: Option<AccountStatusUpdateInstructionReason2Choice>,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<String>,
}


// AccountStatusUpdateInstructionReason1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountStatusUpdateInstructionReason1Choice {
	#[serde(rename = "NoSpcfdRsn")]
	pub no_spcfd_rsn: Option<String>,
	#[validate]
	#[serde(rename = "Rsn")]
	pub rsn: Option<Vec<AccountStatusUpdateInstructionReason1>>,
}


// AccountStatusUpdateInstructionReason2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountStatusUpdateInstructionReason2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification36>,
}


// AccountStatusUpdateRequestReason1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountStatusUpdateRequestReason1Code {
	#[validate(enumerate = ["CLOE"])]
	#[serde(rename = "AccountStatusUpdateRequestReason1Code")]
	pub account_status_update_request_reason1_code: String,
}


// AccountType2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountType2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// AccountUsageType2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountUsageType2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// AccountUsageType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountUsageType2Code {
	#[validate(enumerate = ["INVE", "ISSP", "SETP", "TRDP"])]
	#[serde(rename = "AccountUsageType2Code")]
	pub account_usage_type2_code: String,
}


// AccountingStatus1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountingStatus1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// AccountingStatus1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AccountingStatus1Code {
	#[validate(enumerate = ["YDOM", "NDOM"])]
	#[serde(rename = "AccountingStatus1Code")]
	pub accounting_status1_code: String,
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


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyCode {
	#[validate(pattern = "[A-Z]{3,3}")]
	#[serde(rename = "ActiveOrHistoricCurrencyCode")]
	pub active_or_historic_currency_code: String,
}


// AdditionalReference13 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AdditionalReference13 {
	#[serde(rename = "Ref")]
	pub ref_attr: String,
	#[validate]
	#[serde(rename = "RefIssr")]
	pub ref_issr: Option<PartyIdentification125Choice>,
	#[serde(rename = "MsgNm")]
	pub msg_nm: Option<String>,
}


// AdditiononalInformation13 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AdditiononalInformation13 {
	#[serde(rename = "Lmttn")]
	pub lmttn: Option<String>,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<String>,
	#[serde(rename = "AcctVldtn")]
	pub acct_vldtn: Option<String>,
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[validate]
	#[serde(rename = "Rgltr")]
	pub rgltr: Option<PartyIdentification125Choice>,
	#[validate]
	#[serde(rename = "Sts")]
	pub sts: Option<RestrictionStatus1Choice>,
	#[validate]
	#[serde(rename = "Prd")]
	pub prd: Option<DateTimePeriod2>,
}


// AddressType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AddressType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// AddressType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AddressType1Code {
	#[validate(enumerate = ["HOME", "BIZZ"])]
	#[serde(rename = "AddressType1Code")]
	pub address_type1_code: String,
}


// AddressType2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AddressType2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// AddressType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AddressType2Code {
	#[validate(enumerate = ["ADDR", "PBOX", "HOME", "BIZZ", "MLTO", "DLVY"])]
	#[serde(rename = "AddressType2Code")]
	pub address_type2_code: String,
}


// AlternateSecurityIdentification7 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AlternateSecurityIdentification7 {
	#[serde(rename = "Id")]
	pub id: String,
	#[validate]
	#[serde(rename = "IdSrc")]
	pub id_src: IdentificationSource1Choice,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[validate(pattern = "[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}")]
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// AustrianBankleitzahlIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AustrianBankleitzahlIdentifier {
	#[validate(pattern = "AT[0-9]{5,5}")]
	#[serde(rename = "AustrianBankleitzahlIdentifier")]
	pub austrian_bankleitzahl_identifier: String,
}


// BICFIDec2014Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BICFIDec2014Identifier {
	#[validate(pattern = "[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}")]
	#[serde(rename = "BICFIDec2014Identifier")]
	pub bicfi_dec2014_identifier: String,
}


// BelgianIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BelgianIdentifier {
	#[serde(rename = "BelgianIdentifier")]
	pub belgian_identifier: String,
}


// BlockedHoldingDetails2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BlockedHoldingDetails2 {
	#[serde(rename = "BlckdHldg")]
	pub blckd_hldg: String,
	#[serde(rename = "PrtlHldgUnits")]
	pub prtl_hldg_units: Option<f64>,
	#[serde(rename = "HldgCertNb")]
	pub hldg_cert_nb: Option<String>,
}


// BlockedReason2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BlockedReason2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// BlockedReason2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BlockedReason2Code {
	#[validate(enumerate = ["BKRP", "CMMT", "CNFS", "MORT", "PCOM", "PLDG", "TRPE", "SANC", "TRAN"])]
	#[serde(rename = "BlockedReason2Code")]
	pub blocked_reason2_code: String,
}


// BlockedStatusReason2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BlockedStatusReason2 {
	#[validate]
	#[serde(rename = "TxTp")]
	pub tx_tp: TransactionType5Choice,
	#[serde(rename = "Blckd")]
	pub blckd: bool,
	#[validate]
	#[serde(rename = "Rsn")]
	pub rsn: Option<Vec<BlockedReason2Choice>>,
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: String,
}


// BlockedStatusReason2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BlockedStatusReason2Choice {
	#[serde(rename = "NoSpcfdRsn")]
	pub no_spcfd_rsn: Option<String>,
	#[validate]
	#[serde(rename = "Rsn")]
	pub rsn: Option<Vec<BlockedStatusReason2>>,
}


// Bloomberg2Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Bloomberg2Identifier {
	#[validate(pattern = "(BBG)[BCDFGHJKLMNPQRSTVWXYZ\\d]{8}\\d")]
	#[serde(rename = "Bloomberg2Identifier")]
	pub bloomberg2_identifier: String,
}


// BranchData4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BranchData4 {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[validate]
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress1>,
}


// CHIPSParticipantIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CHIPSParticipantIdentifier {
	#[validate(pattern = "CP[0-9]{4,4}")]
	#[serde(rename = "CHIPSParticipantIdentifier")]
	pub chips_participant_identifier: String,
}


// CHIPSUniversalIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CHIPSUniversalIdentifier {
	#[validate(pattern = "CH[0-9]{6,6}")]
	#[serde(rename = "CHIPSUniversalIdentifier")]
	pub chips_universal_identifier: String,
}


// CRSForm1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CRSForm1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// CRSFormType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CRSFormType1Code {
	#[validate(enumerate = ["CER4", "CER3", "CER5", "CER6", "CER8", "CER1", "CER2", "CER7"])]
	#[serde(rename = "CRSFormType1Code")]
	pub crs_form_type1_code: String,
}


// CRSSource1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CRSSource1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// CRSSourceStatus1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CRSSourceStatus1Code {
	#[validate(enumerate = ["CALC", "DECL"])]
	#[serde(rename = "CRSSourceStatus1Code")]
	pub crs_source_status1_code: String,
}


// CRSStatus1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CRSStatus1Code {
	#[validate(enumerate = ["C101", "C102", "C103", "C104", "C105", "C106", "C107", "C108", "C109", "C110", "C111", "C112", "C113", "C114"])]
	#[serde(rename = "CRSStatus1Code")]
	pub crs_status1_code: String,
}


// CRSStatus3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CRSStatus3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// CRSStatus4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CRSStatus4 {
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: CRSStatus3Choice,
	#[validate]
	#[serde(rename = "Src")]
	pub src: Option<CRSSource1Choice>,
	#[serde(rename = "XcptnlRptgCtry")]
	pub xcptnl_rptg_ctry: Option<String>,
}


// CUSIPIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CUSIPIdentifier {
	#[serde(rename = "CUSIPIdentifier")]
	pub cusip_identifier: String,
}


// CanadianPaymentsARNIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CanadianPaymentsARNIdentifier {
	#[validate(pattern = "CA[0-9]{9,9}")]
	#[serde(rename = "CanadianPaymentsARNIdentifier")]
	pub canadian_payments_arn_identifier: String,
}


// CardType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CardType1Code {
	#[validate(enumerate = ["CRDT", "DBIT"])]
	#[serde(rename = "CardType1Code")]
	pub card_type1_code: String,
}


// CashAccount204 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CashAccount204 {
	#[serde(rename = "SttlmCcy")]
	pub sttlm_ccy: String,
	#[validate]
	#[serde(rename = "Id")]
	pub id: AccountIdentificationAndName5,
	#[validate]
	#[serde(rename = "AcctOwnr")]
	pub acct_ownr: Option<PartyIdentification125Choice>,
	#[validate]
	#[serde(rename = "AcctSvcr")]
	pub acct_svcr: Option<FinancialInstitutionIdentification11Choice>,
	#[validate]
	#[serde(rename = "AcctSvcrBrnch")]
	pub acct_svcr_brnch: Option<BranchData4>,
	#[validate]
	#[serde(rename = "AcctOwnrOthrId")]
	pub acct_ownr_othr_id: Option<Vec<GenericIdentification82>>,
	#[validate]
	#[serde(rename = "InvstmtAcctTp")]
	pub invstmt_acct_tp: Option<AccountType2Choice>,
	#[serde(rename = "CdtDbt")]
	pub cdt_dbt: Option<String>,
	#[validate]
	#[serde(rename = "SttlmInstrRsn")]
	pub sttlm_instr_rsn: Option<SettlementInstructionReason1Choice>,
	#[validate]
	#[serde(rename = "CshAcctPurp")]
	pub csh_acct_purp: Option<CashAccountType3Choice>,
	#[validate]
	#[serde(rename = "CshAcctDsgnt")]
	pub csh_acct_dsgnt: Option<AccountDesignation1Choice>,
	#[serde(rename = "DvddPctg")]
	pub dvdd_pctg: Option<f64>,
}


// CashAccountType3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CashAccountType3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// CashAccountType5Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CashAccountType5Code {
	#[validate(enumerate = ["LEND", "COLL", "SETT", "MARR", "SEGT"])]
	#[serde(rename = "CashAccountType5Code")]
	pub cash_account_type5_code: String,
}


// CashSettlement4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CashSettlement4 {
	#[serde(rename = "ModScpIndctn")]
	pub mod_scp_indctn: String,
	#[validate]
	#[serde(rename = "CshAcctDtls")]
	pub csh_acct_dtls: Option<Vec<CashAccount204>>,
	#[validate]
	#[serde(rename = "OthrCshSttlmDtls")]
	pub othr_csh_sttlm_dtls: Option<Vec<PaymentInstrument17>>,
}


// CertificateType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CertificateType2Code {
	#[validate(enumerate = ["AMLC", "DVLC", "DFOR", "GOST", "IDEN", "INCU", "LREF", "PASS", "PRAD", "PKIC"])]
	#[serde(rename = "CertificateType2Code")]
	pub certificate_type2_code: String,
}


// CertificationType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CertificationType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// Cheque4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Cheque4 {
	#[validate]
	#[serde(rename = "PyeeId")]
	pub pyee_id: NameAndAddress5,
}


// CitizenshipInformation2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CitizenshipInformation2 {
	#[serde(rename = "Ntlty")]
	pub ntlty: String,
	#[serde(rename = "MnrInd")]
	pub mnr_ind: bool,
}


// CivilStatus1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CivilStatus1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// CivilStatus1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CivilStatus1Code {
	#[validate(enumerate = ["DIVO", "LDIV", "MARR", "SEPA", "SING", "UNIO", "WIDO"])]
	#[serde(rename = "CivilStatus1Code")]
	pub civil_status1_code: String,
}


// ClearingSystemMemberIdentification4Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ClearingSystemMemberIdentification4Choice {
	#[serde(rename = "USCHU")]
	pub uschu: Option<String>,
	#[serde(rename = "NZNCC")]
	pub nzncc: Option<String>,
	#[serde(rename = "IENSC")]
	pub iensc: Option<String>,
	#[serde(rename = "GBSC")]
	pub gbsc: Option<String>,
	#[serde(rename = "USCH")]
	pub usch: Option<String>,
	#[serde(rename = "CHBC")]
	pub chbc: Option<String>,
	#[serde(rename = "USFW")]
	pub usfw: Option<String>,
	#[serde(rename = "PTNCC")]
	pub ptncc: Option<String>,
	#[serde(rename = "RUCB")]
	pub rucb: Option<String>,
	#[serde(rename = "ITNCC")]
	pub itncc: Option<String>,
	#[serde(rename = "ATBLZ")]
	pub atblz: Option<String>,
	#[serde(rename = "CACPA")]
	pub cacpa: Option<String>,
	#[serde(rename = "CHSIC")]
	pub chsic: Option<String>,
	#[serde(rename = "DEBLZ")]
	pub deblz: Option<String>,
	#[serde(rename = "ESNCC")]
	pub esncc: Option<String>,
	#[serde(rename = "ZANCC")]
	pub zancc: Option<String>,
	#[serde(rename = "HKNCC")]
	pub hkncc: Option<String>,
	#[serde(rename = "AUBSBx")]
	pub aubs_bx: Option<String>,
	#[serde(rename = "AUBSBs")]
	pub aubs_bs: Option<String>,
}


// Collateral1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Collateral1Code {
	#[validate(enumerate = ["COLL", "NCOL"])]
	#[serde(rename = "Collateral1Code")]
	pub collateral1_code: String,
}


// CommunicationAddress6 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CommunicationAddress6 {
	#[validate]
	#[serde(rename = "AdrTp")]
	pub adr_tp: Option<AddressType1Choice>,
	#[serde(rename = "Email")]
	pub email: Option<String>,
	#[serde(rename = "Phne")]
	pub phne: Option<String>,
	#[serde(rename = "Mob")]
	pub mob: Option<String>,
	#[serde(rename = "FaxNb")]
	pub fax_nb: Option<String>,
	#[serde(rename = "TlxAdr")]
	pub tlx_adr: Option<String>,
	#[serde(rename = "URLAdr")]
	pub url_adr: Option<String>,
}


// CommunicationMethod1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CommunicationMethod1Code {
	#[validate(enumerate = ["SWMT", "SWMX", "FAXI", "EMAL", "PROP"])]
	#[serde(rename = "CommunicationMethod1Code")]
	pub communication_method1_code: String,
}


// CommunicationMethod3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CommunicationMethod3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// CompanyLink1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompanyLink1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// CompanyLink1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompanyLink1Code {
	#[validate(enumerate = ["AGEN", "BROK", "PART", "MEMB", "PCOM", "RELA"])]
	#[serde(rename = "CompanyLink1Code")]
	pub company_link1_code: String,
}


// ConductClassification1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ConductClassification1Code {
	#[validate(enumerate = ["NSTA", "RCLT", "STAN"])]
	#[serde(rename = "ConductClassification1Code")]
	pub conduct_classification1_code: String,
}


// ConsolidatedTapeAssociationIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ConsolidatedTapeAssociationIdentifier {
	#[validate(min_length = 1)]
	#[validate(max_length = 35)]
	#[serde(rename = "ConsolidatedTapeAssociationIdentifier")]
	pub consolidated_tape_association_identifier: String,
}


// ConsolidationType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ConsolidationType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// ConsolidationType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ConsolidationType1Code {
	#[validate(enumerate = ["GENL", "PART"])]
	#[serde(rename = "ConsolidationType1Code")]
	pub consolidation_type1_code: String,
}


// CountryAndResidentialStatusType2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CountryAndResidentialStatusType2 {
	#[serde(rename = "Ctry")]
	pub ctry: String,
	#[serde(rename = "ResdtlSts")]
	pub resdtl_sts: String,
}


// CountryCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CountryCode {
	#[validate(pattern = "[A-Z]{2,2}")]
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// CreditDebit3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CreditDebit3Code {
	#[validate(enumerate = ["CRDT", "DBIT"])]
	#[serde(rename = "CreditDebit3Code")]
	pub credit_debit3_code: String,
}


// CustomerConductClassification1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CustomerConductClassification1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// DataBaseCheck1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DataBaseCheck1 {
	#[serde(rename = "DBChck")]
	pub db_chck: bool,
	#[serde(rename = "Id")]
	pub id: String,
}


// DataModification1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DataModification1Code {
	#[validate(enumerate = ["INSE", "UPDT", "DELT"])]
	#[serde(rename = "DataModification1Code")]
	pub data_modification1_code: String,
}


// DataModification2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DataModification2Code {
	#[validate(enumerate = ["INSE", "DELT"])]
	#[serde(rename = "DataModification2Code")]
	pub data_modification2_code: String,
}


// DateAndAmount1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DateAndAmount1 {
	#[serde(rename = "Dt")]
	pub dt: String,
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAndAmount,
}


// DateAndDateTime1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DateAndDateTime1Choice {
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm")]
	pub dt_tm: Option<String>,
}


// DateTimePeriod2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DateTimePeriod2 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: Option<String>,
}


// DeMinimus1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DeMinimus1Choice {
	#[validate]
	#[serde(rename = "DeMnmsAplbl")]
	pub de_mnms_aplbl: Option<DeMinimusApplicable1>,
	#[validate]
	#[serde(rename = "DeMnmsNotAplbl")]
	pub de_mnms_not_aplbl: Option<DeMinimusNotApplicable1>,
}


// DeMinimusApplicable1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DeMinimusApplicable1 {
	#[serde(rename = "NewIssePrmssn")]
	pub new_isse_prmssn: bool,
	#[serde(rename = "Pctg")]
	pub pctg: Option<f64>,
}


// DeMinimusNotApplicable1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DeMinimusNotApplicable1 {
	#[serde(rename = "RstrctdPrsnRsn")]
	pub rstrctd_prsn_rsn: String,
}


// DecimalNumber ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "DecimalNumber")]
	pub decimal_number: f64,
}


// DirectDebitMandate7 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DirectDebitMandate7 {
	#[validate]
	#[serde(rename = "DbtrAcct")]
	pub dbtr_acct: AccountIdentificationAndName5,
	#[validate]
	#[serde(rename = "Dbtr")]
	pub dbtr: Option<PartyIdentification125Choice>,
	#[serde(rename = "DbtrTaxIdNb")]
	pub dbtr_tax_id_nb: Option<String>,
	#[serde(rename = "DbtrNtlRegnNb")]
	pub dbtr_ntl_regn_nb: Option<String>,
	#[validate]
	#[serde(rename = "Cdtr")]
	pub cdtr: Option<PartyIdentification125Choice>,
	#[validate]
	#[serde(rename = "DbtrAgt")]
	pub dbtr_agt: FinancialInstitutionIdentification11Choice,
	#[validate]
	#[serde(rename = "DbtrAgtBrnch")]
	pub dbtr_agt_brnch: Option<BranchData4>,
	#[validate]
	#[serde(rename = "CdtrAgt")]
	pub cdtr_agt: Option<FinancialInstitutionIdentification11Choice>,
	#[validate]
	#[serde(rename = "CdtrAgtBrnch")]
	pub cdtr_agt_brnch: Option<BranchData4>,
	#[serde(rename = "RegnId")]
	pub regn_id: Option<String>,
	#[serde(rename = "MndtId")]
	pub mndt_id: Option<String>,
}


// DistributionPolicy1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DistributionPolicy1Code {
	#[validate(enumerate = ["DIST", "ACCU"])]
	#[serde(rename = "DistributionPolicy1Code")]
	pub distribution_policy1_code: String,
}


// DocumentToSend4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DocumentToSend4 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[validate]
	#[serde(rename = "Rcpt")]
	pub rcpt: PartyIdentification125Choice,
	#[validate]
	#[serde(rename = "MtdOfTrnsmssn")]
	pub mtd_of_trnsmssn: CommunicationMethod3Choice,
}


// DutchIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DutchIdentifier {
	#[serde(rename = "DutchIdentifier")]
	pub dutch_identifier: String,
}


// Eligible1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Eligible1Code {
	#[validate(enumerate = ["ELIG", "NELI"])]
	#[serde(rename = "Eligible1Code")]
	pub eligible1_code: String,
}


// EuroclearClearstreamIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EuroclearClearstreamIdentifier {
	#[validate(min_length = 1)]
	#[validate(max_length = 12)]
	#[serde(rename = "EuroclearClearstreamIdentifier")]
	pub euroclear_clearstream_identifier: String,
}


// EventFrequency10Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EventFrequency10Code {
	#[validate(enumerate = ["DAIL", "ADHO"])]
	#[serde(rename = "EventFrequency10Code")]
	pub event_frequency10_code: String,
}


// EventFrequency1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EventFrequency1Code {
	#[validate(enumerate = ["YEAR", "SEMI", "QUTR", "TOMN", "MNTH", "TWMN", "TOWK", "WEEK", "DAIL", "ADHO", "INDA", "OVNG", "ONDE"])]
	#[serde(rename = "EventFrequency1Code")]
	pub event_frequency1_code: String,
}


// EventFrequency8Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EventFrequency8Code {
	#[validate(enumerate = ["ADHO", "YEAR", "DAIL", "FOMN", "TOMN", "TOWK", "TYEA", "INDA", "MNTH", "ONDE", "OVNG", "QUTR", "SEMI", "TWMN", "WEEK"])]
	#[serde(rename = "EventFrequency8Code")]
	pub event_frequency8_code: String,
}


// EventFrequency9Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EventFrequency9Code {
	#[validate(enumerate = ["YEAR", "SEMI", "QUTR", "TOMN", "MNTH", "TWMN", "TOWK", "WEEK", "DAIL", "ADHO", "INDA", "OVNG", "ONDE", "NONE"])]
	#[serde(rename = "EventFrequency9Code")]
	pub event_frequency9_code: String,
}


// Exact4AlphaNumericText ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[validate(pattern = "[a-zA-Z0-9]{4}")]
	#[serde(rename = "Exact4AlphaNumericText")]
	pub exact4_alpha_numeric_text: String,
}


// Extended350Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Extended350Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 350)]
	#[serde(rename = "Extended350Code")]
	pub extended350_code: String,
}


// ExtendedParty15 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExtendedParty15 {
	#[serde(rename = "XtndedPtyRole")]
	pub xtnded_pty_role: String,
	#[validate]
	#[serde(rename = "OthrPtyDtls")]
	pub othr_pty_dtls: InvestmentAccountOwnershipInformation17,
}


// Extension1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Extension1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: String,
	#[serde(rename = "Txt")]
	pub txt: String,
}


// ExtensiveBranchNetworkIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExtensiveBranchNetworkIdentifier {
	#[validate(pattern = "AU[0-9]{6,6}")]
	#[serde(rename = "ExtensiveBranchNetworkIdentifier")]
	pub extensive_branch_network_identifier: String,
}


// ExternalAccountIdentification1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalAccountIdentification1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalAccountIdentification1Code")]
	pub external_account_identification1_code: String,
}


// FATCAForm1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FATCAForm1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// FATCAFormType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FATCAFormType1Code {
	#[validate(enumerate = ["CER5", "CER7", "CER1", "CER2", "CER3", "CER4", "CER6"])]
	#[serde(rename = "FATCAFormType1Code")]
	pub fatca_form_type1_code: String,
}


// FATCASource1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FATCASource1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// FATCASourceStatus1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FATCASourceStatus1Code {
	#[validate(enumerate = ["CALC", "DECL"])]
	#[serde(rename = "FATCASourceStatus1Code")]
	pub fatca_source_status1_code: String,
}


// FATCAStatus1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FATCAStatus1Code {
	#[validate(enumerate = ["F101", "F102", "F103", "F104", "F105", "F201", "F202", "F203", "F204", "F205", "F206"])]
	#[serde(rename = "FATCAStatus1Code")]
	pub fatca_status1_code: String,
}


// FATCAStatus2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FATCAStatus2 {
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: FATCAStatus2Choice,
	#[validate]
	#[serde(rename = "Src")]
	pub src: Option<FATCASource1Choice>,
}


// FATCAStatus2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FATCAStatus2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// FedwireRoutingNumberIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FedwireRoutingNumberIdentifier {
	#[validate(pattern = "FW[0-9]{9,9}")]
	#[serde(rename = "FedwireRoutingNumberIdentifier")]
	pub fedwire_routing_number_identifier: String,
}


// FinancialInstitutionIdentification11Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstitutionIdentification11Choice {
	#[validate]
	#[serde(rename = "NmAndAdr")]
	pub nm_and_adr: Option<NameAndAddress5>,
	#[serde(rename = "BICFI")]
	pub bicfi: Option<String>,
	#[validate]
	#[serde(rename = "ClrSysMmbId")]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification4Choice>,
	#[validate]
	#[serde(rename = "PrtryId")]
	pub prtry_id: Option<SimpleIdentificationInformation4>,
}


// FinancialInstrument55 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstrument55 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: SecurityIdentification25Choice,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "ShrtNm")]
	pub shrt_nm: Option<String>,
	#[serde(rename = "SplmtryId")]
	pub splmtry_id: Option<String>,
	#[serde(rename = "ClssTp")]
	pub clss_tp: Option<String>,
	#[serde(rename = "SctiesForm")]
	pub scties_form: Option<String>,
	#[serde(rename = "DstrbtnPlcy")]
	pub dstrbtn_plcy: Option<String>,
	#[serde(rename = "PdctGrp")]
	pub pdct_grp: Option<String>,
}


// FinancialInstrument87 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstrument87 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: SecurityIdentification25Choice,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "ShrtNm")]
	pub shrt_nm: Option<String>,
	#[serde(rename = "SplmtryId")]
	pub splmtry_id: Option<String>,
	#[serde(rename = "ClssTp")]
	pub clss_tp: Option<String>,
	#[serde(rename = "SctiesForm")]
	pub scties_form: Option<String>,
	#[serde(rename = "DstrbtnPlcy")]
	pub dstrbtn_plcy: Option<String>,
	#[serde(rename = "PdctGrp")]
	pub pdct_grp: Option<String>,
	#[validate]
	#[serde(rename = "BlckdHldgDtls")]
	pub blckd_hldg_dtls: Option<BlockedHoldingDetails2>,
	#[serde(rename = "Pldgg")]
	pub pldgg: Option<String>,
	#[serde(rename = "Coll")]
	pub coll: Option<String>,
	#[validate]
	#[serde(rename = "ThrdPtyRghts")]
	pub thrd_pty_rghts: Option<ThirdPartyRights2>,
	#[serde(rename = "FndOwnrsh")]
	pub fnd_ownrsh: Option<String>,
	#[serde(rename = "FndIntntn")]
	pub fnd_intntn: Option<String>,
	#[serde(rename = "OprlSts")]
	pub oprl_sts: Option<String>,
}


// FiscalYear1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FiscalYear1Choice {
	#[serde(rename = "StartDt")]
	pub start_dt: Option<String>,
	#[serde(rename = "EndDt")]
	pub end_dt: Option<String>,
}


// FormOfSecurity1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FormOfSecurity1Code {
	#[validate(enumerate = ["BEAR", "REGD"])]
	#[serde(rename = "FormOfSecurity1Code")]
	pub form_of_security1_code: String,
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


// FundCashAccount4Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FundCashAccount4Code {
	#[validate(enumerate = ["HEDG", "CPFO", "CPFS", "SRSA", "CSDO", "TOFF", "ICSA", "CSDM", "CSDP", "PPEN", "CPEN"])]
	#[serde(rename = "FundCashAccount4Code")]
	pub fund_cash_account4_code: String,
}


// FundIntention1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FundIntention1Code {
	#[validate(enumerate = ["YQUA", "NQUA"])]
	#[serde(rename = "FundIntention1Code")]
	pub fund_intention1_code: String,
}


// FundOwnership1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FundOwnership1Code {
	#[validate(enumerate = ["YALL", "NALL"])]
	#[serde(rename = "FundOwnership1Code")]
	pub fund_ownership1_code: String,
}


// GDPRData1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GDPRData1 {
	#[validate]
	#[serde(rename = "CnsntTp")]
	pub cnsnt_tp: GDPRDataConsent1Choice,
	#[serde(rename = "CnsntInd")]
	pub cnsnt_ind: bool,
	#[serde(rename = "CnsntDt")]
	pub cnsnt_dt: String,
}


// GDPRDataConsent1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GDPRDataConsent1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// GDPRDataConsent1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GDPRDataConsent1Code {
	#[validate(enumerate = ["DP00", "DP03", "DP01", "DP02"])]
	#[serde(rename = "GDPRDataConsent1Code")]
	pub gdpr_data_consent1_code: String,
}


// Gender1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Gender1Code {
	#[validate(enumerate = ["FEMA", "MALE"])]
	#[serde(rename = "Gender1Code")]
	pub gender1_code: String,
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


// GenericIdentification81 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification81 {
	#[serde(rename = "Id")]
	pub id: String,
	#[validate]
	#[serde(rename = "IdTp")]
	pub id_tp: OtherIdentification3Choice,
}


// GenericIdentification82 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification82 {
	#[serde(rename = "Id")]
	pub id: String,
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: OtherIdentification3Choice,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
	#[serde(rename = "IsseDt")]
	pub isse_dt: Option<String>,
	#[serde(rename = "XpryDt")]
	pub xpry_dt: Option<String>,
	#[serde(rename = "Stat")]
	pub stat: Option<String>,
	#[serde(rename = "IssrCtry")]
	pub issr_ctry: Option<String>,
}


// GermanBankleitzahlIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GermanBankleitzahlIdentifier {
	#[validate(pattern = "BL[0-9]{8,8}")]
	#[serde(rename = "GermanBankleitzahlIdentifier")]
	pub german_bankleitzahl_identifier: String,
}


// HighFrequencyTradingProfile1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct HighFrequencyTradingProfile1 {
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[validate]
	#[serde(rename = "SttlmFrqcy")]
	pub sttlm_frqcy: Option<SettlementFrequency1Choice>,
	#[validate]
	#[serde(rename = "CnsldtnTp")]
	pub cnsldtn_tp: Option<ConsolidationType1Choice>,
}


// Holding1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Holding1Code {
	#[validate(enumerate = ["CERT", "NPRH", "PRTH"])]
	#[serde(rename = "Holding1Code")]
	pub holding1_code: String,
}


// HongKongBankIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct HongKongBankIdentifier {
	#[validate(pattern = "HK[0-9]{3,3}")]
	#[serde(rename = "HongKongBankIdentifier")]
	pub hong_kong_bank_identifier: String,
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


// ISOYearMonth ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISOYearMonth {
	#[serde(rename = "ISOYearMonth")]
	pub iso_year_month: String,
}


// IdentificationSource1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct IdentificationSource1Choice {
	#[serde(rename = "Dmst")]
	pub dmst: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// IncomePreference2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct IncomePreference2Code {
	#[validate(enumerate = ["CASH", "SECU"])]
	#[serde(rename = "IncomePreference2Code")]
	pub income_preference2_code: String,
}


// IndividualPerson29 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct IndividualPerson29 {
	#[validate]
	#[serde(rename = "NmPrfx")]
	pub nm_prfx: Option<NamePrefix1Choice>,
	#[serde(rename = "GvnNm")]
	pub gvn_nm: Option<String>,
	#[serde(rename = "MddlNm")]
	pub mddl_nm: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: String,
	#[validate]
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Vec<PostalAddress21>,
}


// IndividualPerson35 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct IndividualPerson35 {
	#[serde(rename = "GvnNm")]
	pub gvn_nm: Option<String>,
	#[serde(rename = "MddlNm")]
	pub mddl_nm: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "Gndr")]
	pub gndr: Option<String>,
	#[serde(rename = "BirthDt")]
	pub birth_dt: Option<String>,
}


// IndividualPerson38 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct IndividualPerson38 {
	#[validate]
	#[serde(rename = "NmPrfx")]
	pub nm_prfx: Option<NamePrefix1Choice>,
	#[serde(rename = "GvnNm")]
	pub gvn_nm: Option<String>,
	#[serde(rename = "MddlNm")]
	pub mddl_nm: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "NmSfx")]
	pub nm_sfx: Option<String>,
	#[serde(rename = "Gndr")]
	pub gndr: Option<String>,
	#[serde(rename = "BirthDt")]
	pub birth_dt: Option<String>,
	#[serde(rename = "CtryOfBirth")]
	pub ctry_of_birth: Option<String>,
	#[serde(rename = "PrvcOfBirth")]
	pub prvc_of_birth: Option<String>,
	#[serde(rename = "CityOfBirth")]
	pub city_of_birth: Option<String>,
	#[serde(rename = "Prfssn")]
	pub prfssn: Option<String>,
	#[validate]
	#[serde(rename = "ModfdPstlAdr")]
	pub modfd_pstl_adr: Option<Vec<ModificationScope34>>,
	#[validate]
	#[serde(rename = "ModfdCtznsh")]
	pub modfd_ctznsh: Option<Vec<ModificationScope39>>,
	#[serde(rename = "EmplngCpny")]
	pub emplng_cpny: Option<String>,
	#[serde(rename = "BizFctn")]
	pub biz_fctn: Option<String>,
	#[validate]
	#[serde(rename = "PltclyXpsdPrsn")]
	pub pltcly_xpsd_prsn: Option<PoliticallyExposedPerson1>,
	#[serde(rename = "DthDt")]
	pub dth_dt: Option<String>,
	#[validate]
	#[serde(rename = "CvlSts")]
	pub cvl_sts: Option<CivilStatus1Choice>,
	#[serde(rename = "EdctnLvl")]
	pub edctn_lvl: Option<String>,
	#[validate]
	#[serde(rename = "FmlyInf")]
	pub fmly_inf: Option<PersonalInformation1>,
	#[validate]
	#[serde(rename = "GDPRData")]
	pub gdpr_data: Option<Vec<GDPRData1>>,
}


// IndividualPersonIdentification3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct IndividualPersonIdentification3Choice {
	#[validate]
	#[serde(rename = "IdNb")]
	pub id_nb: Option<GenericIdentification81>,
	#[validate]
	#[serde(rename = "PrsnNm")]
	pub prsn_nm: Option<IndividualPerson35>,
}


// InformationDistribution1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InformationDistribution1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// InformationDistribution2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InformationDistribution2Code {
	#[validate(enumerate = ["ELEC", "NONE", "PAPR"])]
	#[serde(rename = "InformationDistribution2Code")]
	pub information_distribution2_code: String,
}


// InitialAmount1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InitialAmount1Choice {
	#[serde(rename = "InitlNbOfInstlmts")]
	pub initl_nb_of_instlmts: Option<f64>,
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveCurrencyAndAmount>,
}


// Insurance1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Insurance1Code {
	#[validate(enumerate = ["LIFE", "PDIS"])]
	#[serde(rename = "Insurance1Code")]
	pub insurance1_code: String,
}


// InsuranceType2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InsuranceType2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// Intermediary46 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Intermediary46 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: PartyIdentification177Choice,
	#[serde(rename = "LglNttyIdr")]
	pub lgl_ntty_idr: Option<String>,
	#[validate]
	#[serde(rename = "Acct")]
	pub acct: Option<Account32>,
	#[serde(rename = "WvdTrlrComssnInd")]
	pub wvd_trlr_comssn_ind: Option<bool>,
	#[validate]
	#[serde(rename = "Role")]
	pub role: Option<PartyRole2Choice>,
	#[validate]
	#[serde(rename = "PmryComAdr")]
	pub pmry_com_adr: Option<Vec<CommunicationAddress6>>,
	#[validate]
	#[serde(rename = "ScndryComAdr")]
	pub scndry_com_adr: Option<Vec<CommunicationAddress6>>,
	#[validate]
	#[serde(rename = "NmAndAdr")]
	pub nm_and_adr: Option<NameAndAddress4>,
}


// Intermediary47 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Intermediary47 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: PartyIdentification125Choice,
	#[serde(rename = "LglNttyIdr")]
	pub lgl_ntty_idr: Option<String>,
	#[validate]
	#[serde(rename = "Acct")]
	pub acct: Option<Account32>,
}


// InvestmentAccount75 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestmentAccount75 {
	#[validate]
	#[serde(rename = "AcctStsUpdInstr")]
	pub acct_sts_upd_instr: Option<AccountStatusUpdateInstruction1>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Dsgnt")]
	pub dsgnt: Option<String>,
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: Option<AccountType2Choice>,
	#[validate]
	#[serde(rename = "OwnrshTp")]
	pub ownrsh_tp: Option<OwnershipType2Choice>,
	#[validate]
	#[serde(rename = "TaxXmptn")]
	pub tax_xmptn: Option<TaxExemptionReason2Choice>,
	#[validate]
	#[serde(rename = "StmtFrqcy")]
	pub stmt_frqcy: Option<StatementFrequencyReason2Choice>,
	#[serde(rename = "RefCcy")]
	pub ref_ccy: Option<String>,
	#[serde(rename = "Lang")]
	pub lang: Option<String>,
	#[serde(rename = "IncmPref")]
	pub incm_pref: Option<String>,
	#[validate]
	#[serde(rename = "RinvstmtDtls")]
	pub rinvstmt_dtls: Option<Vec<Reinvestment4>>,
	#[serde(rename = "TaxWhldgMtd")]
	pub tax_whldg_mtd: Option<String>,
	#[validate]
	#[serde(rename = "TaxRptg")]
	pub tax_rptg: Option<Vec<TaxReporting3>>,
	#[validate]
	#[serde(rename = "LttrInttDtls")]
	pub lttr_intt_dtls: Option<LetterIntent1>,
	#[serde(rename = "AcmltnRghtRef")]
	pub acmltn_rght_ref: Option<String>,
	#[serde(rename = "ReqrdSgntriesNb")]
	pub reqrd_sgntries_nb: Option<f64>,
	#[serde(rename = "FndFmlyNm")]
	pub fnd_fmly_nm: Option<String>,
	#[validate]
	#[serde(rename = "ModfdFinInstrmDtls")]
	pub modfd_fin_instrm_dtls: Option<Vec<ModificationScope42>>,
	#[validate]
	#[serde(rename = "RndgDtls")]
	pub rndg_dtls: Option<RoundingParameters1>,
	#[validate]
	#[serde(rename = "AcctSvcr")]
	pub acct_svcr: Option<PartyIdentification125Choice>,
	#[validate]
	#[serde(rename = "BlckdSts")]
	pub blckd_sts: Option<BlockedStatusReason2Choice>,
	#[validate]
	#[serde(rename = "AcctUsgTp")]
	pub acct_usg_tp: Option<AccountUsageType2Choice>,
	#[serde(rename = "FrgnStsCertfctn")]
	pub frgn_sts_certfctn: Option<String>,
	#[validate]
	#[serde(rename = "AcctSgntrDtTm")]
	pub acct_sgntr_dt_tm: Option<DateAndDateTime1Choice>,
	#[validate]
	#[serde(rename = "TxChanlTp")]
	pub tx_chanl_tp: Option<TransactionChannelType1Choice>,
	#[validate]
	#[serde(rename = "InvstmtAcctCtgy")]
	pub invstmt_acct_ctgy: Option<InvestmentAccountCategory1Choice>,
	#[serde(rename = "Pldgg")]
	pub pldgg: Option<String>,
	#[serde(rename = "Coll")]
	pub coll: Option<String>,
	#[validate]
	#[serde(rename = "ThrdPtyRghts")]
	pub thrd_pty_rghts: Option<ThirdPartyRights2>,
	#[validate]
	#[serde(rename = "PwrOfAttnyLvlOfCtrl")]
	pub pwr_of_attny_lvl_of_ctrl: Option<LevelOfControl1Choice>,
	#[validate]
	#[serde(rename = "AcctgSts")]
	pub acctg_sts: Option<AccountingStatus1Choice>,
	#[validate]
	#[serde(rename = "OpngDt")]
	pub opng_dt: Option<DateAndDateTime1Choice>,
	#[validate]
	#[serde(rename = "ClsgDt")]
	pub clsg_dt: Option<DateAndDateTime1Choice>,
	#[serde(rename = "NegInd")]
	pub neg_ind: Option<bool>,
	#[serde(rename = "PrcgOrdr")]
	pub prcg_ordr: Option<String>,
	#[validate]
	#[serde(rename = "Lblty")]
	pub lblty: Option<Liability1Choice>,
	#[validate]
	#[serde(rename = "ModfdInvstrPrfl")]
	pub modfd_invstr_prfl: Option<Vec<ModificationScope46>>,
	#[validate]
	#[serde(rename = "FsclYr")]
	pub fscl_yr: Option<FiscalYear1Choice>,
}


// InvestmentAccount76 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestmentAccount76 {
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Dsgnt")]
	pub dsgnt: Option<String>,
	#[serde(rename = "FndTp")]
	pub fnd_tp: Option<String>,
	#[serde(rename = "FndFmlyNm")]
	pub fnd_fmly_nm: Option<String>,
	#[validate]
	#[serde(rename = "SctyDtls")]
	pub scty_dtls: Option<FinancialInstrument55>,
	#[validate]
	#[serde(rename = "AcctOwnr")]
	pub acct_ownr: Option<AccountOwner3Choice>,
	#[validate]
	#[serde(rename = "Intrmy")]
	pub intrmy: Option<Vec<Intermediary47>>,
	#[validate]
	#[serde(rename = "AcctSvcr")]
	pub acct_svcr: Option<PartyIdentification125Choice>,
}


// InvestmentAccountCategory1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestmentAccountCategory1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// InvestmentAccountCategory1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestmentAccountCategory1Code {
	#[validate(enumerate = ["MAND", "RETA"])]
	#[serde(rename = "InvestmentAccountCategory1Code")]
	pub investment_account_category1_code: String,
}


// InvestmentAccountModification4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestmentAccountModification4 {
	#[serde(rename = "ModRsn")]
	pub mod_rsn: Option<String>,
	#[serde(rename = "AcctApplId")]
	pub acct_appl_id: Option<String>,
	#[serde(rename = "ClntRef")]
	pub clnt_ref: Option<String>,
	#[validate]
	#[serde(rename = "CtrPtyRef")]
	pub ctr_pty_ref: Option<AdditionalReference13>,
	#[validate]
	#[serde(rename = "ExstgAcctId")]
	pub exstg_acct_id: Option<Vec<Account23>>,
}


// InvestmentAccountOwnershipInformation17 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestmentAccountOwnershipInformation17 {
	#[validate]
	#[serde(rename = "Pty")]
	pub pty: Party48Choice,
	#[validate]
	#[serde(rename = "MnyLndrgChck")]
	pub mny_lndrg_chck: Option<MoneyLaunderingCheck1Choice>,
	#[validate]
	#[serde(rename = "ModfdInvstrPrflVldtn")]
	pub modfd_invstr_prfl_vldtn: Option<Vec<ModificationScope27>>,
	#[validate]
	#[serde(rename = "OwnrshBnfcryRate")]
	pub ownrsh_bnfcry_rate: Option<OwnershipBeneficiaryRate1>,
	#[serde(rename = "ClntId")]
	pub clnt_id: Option<String>,
	#[serde(rename = "FsclXmptn")]
	pub fscl_xmptn: Option<bool>,
	#[serde(rename = "SgntryRghtInd")]
	pub sgntry_rght_ind: Option<bool>,
	#[validate]
	#[serde(rename = "MiFIDClssfctn")]
	pub mi_fid_clssfctn: Option<MiFIDClassification1>,
	#[validate]
	#[serde(rename = "Ntfctn")]
	pub ntfctn: Option<Vec<Notification2>>,
	#[validate]
	#[serde(rename = "FATCAFormTp")]
	pub fatca_form_tp: Option<Vec<FATCAForm1Choice>>,
	#[validate]
	#[serde(rename = "FATCASts")]
	pub fatca_sts: Option<Vec<FATCAStatus2>>,
	#[serde(rename = "FATCARptgDt")]
	pub fatca_rptg_dt: Option<String>,
	#[validate]
	#[serde(rename = "CRSFormTp")]
	pub crs_form_tp: Option<Vec<CRSForm1Choice>>,
	#[validate]
	#[serde(rename = "CRSSts")]
	pub crs_sts: Option<Vec<CRSStatus4>>,
	#[serde(rename = "CRSRptgDt")]
	pub crs_rptg_dt: Option<String>,
	#[validate]
	#[serde(rename = "OthrId")]
	pub othr_id: Option<Vec<GenericIdentification82>>,
	#[validate]
	#[serde(rename = "TaxXmptn")]
	pub tax_xmptn: Option<TaxExemptionReason2Choice>,
	#[validate]
	#[serde(rename = "TaxRptg")]
	pub tax_rptg: Option<Vec<TaxReporting3>>,
	#[serde(rename = "Lang")]
	pub lang: Option<String>,
	#[validate]
	#[serde(rename = "MailTp")]
	pub mail_tp: Option<MailType1Choice>,
	#[validate]
	#[serde(rename = "CtryAndResdtlSts")]
	pub ctry_and_resdtl_sts: Option<CountryAndResidentialStatusType2>,
	#[validate]
	#[serde(rename = "MntryWlth")]
	pub mntry_wlth: Option<DateAndAmount1>,
	#[validate]
	#[serde(rename = "EqtyVal")]
	pub eqty_val: Option<DateAndAmount1>,
	#[validate]
	#[serde(rename = "WorkgCptl")]
	pub workg_cptl: Option<DateAndAmount1>,
	#[validate]
	#[serde(rename = "CpnyLk")]
	pub cpny_lk: Option<CompanyLink1Choice>,
	#[serde(rename = "ElctrncMlngSvcRef")]
	pub elctrnc_mlng_svc_ref: Option<String>,
	#[validate]
	#[serde(rename = "PmryComAdr")]
	pub pmry_com_adr: Option<Vec<CommunicationAddress6>>,
	#[validate]
	#[serde(rename = "ScndryComAdr")]
	pub scndry_com_adr: Option<Vec<CommunicationAddress6>>,
	#[validate]
	#[serde(rename = "AddtlRgltryInf")]
	pub addtl_rgltry_inf: Option<RegulatoryInformation1>,
	#[validate]
	#[serde(rename = "AcctgSts")]
	pub acctg_sts: Option<AccountingStatus1Choice>,
	#[validate]
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<Vec<AdditiononalInformation13>>,
	#[serde(rename = "CtrlgPty")]
	pub ctrlg_pty: Option<bool>,
}


// InvestmentFundRole6Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestmentFundRole6Code {
	#[validate(enumerate = ["CACO", "CONC", "CUST", "DATP", "DIST", "FACT", "FIAD", "FIAG", "FMCO", "FNBR", "FTAG", "INTR", "INVE", "INVS", "PAYI", "REGI", "TRAG", "TRAN"])]
	#[serde(rename = "InvestmentFundRole6Code")]
	pub investment_fund_role6_code: String,
}


// InvestmentFundRole7Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestmentFundRole7Code {
	#[validate(enumerate = ["CONC", "DIST", "FMCO", "INTR", "PAYI", "TRAG", "CUST", "CACO", "FACT", "INVE", "INVS"])]
	#[serde(rename = "InvestmentFundRole7Code")]
	pub investment_fund_role7_code: String,
}


// InvestmentFundTransactionType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestmentFundTransactionType1Code {
	#[validate(enumerate = ["ALLL", "SELL", "BUYI", "SWIO", "TRIN", "TOUT", "SUBS", "REDM", "CDEP", "CWIT", "DIVP", "CAEV", "CROI", "CROO", "DIVI", "INSP", "OTHR", "REAA", "RWPL", "RDIV", "SSPL", "SUAA"])]
	#[serde(rename = "InvestmentFundTransactionType1Code")]
	pub investment_fund_transaction_type1_code: String,
}


// InvestmentPlan16 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestmentPlan16 {
	#[validate]
	#[serde(rename = "Frqcy")]
	pub frqcy: Frequency20Choice,
	#[serde(rename = "StartDt")]
	pub start_dt: Option<String>,
	#[serde(rename = "EndDt")]
	pub end_dt: Option<String>,
	#[validate]
	#[serde(rename = "Qty")]
	pub qty: UnitsOrAmount1Choice,
	#[serde(rename = "GrssAmtInd")]
	pub grss_amt_ind: Option<bool>,
	#[serde(rename = "IncmPref")]
	pub incm_pref: Option<String>,
	#[validate]
	#[serde(rename = "InitlAmt")]
	pub initl_amt: Option<InitialAmount1Choice>,
	#[serde(rename = "TtlNbOfInstlmts")]
	pub ttl_nb_of_instlmts: Option<f64>,
	#[serde(rename = "RndgDrctn")]
	pub rndg_drctn: Option<String>,
	#[validate]
	#[serde(rename = "SctyDtls")]
	pub scty_dtls: Vec<Repartition6>,
	#[validate]
	#[serde(rename = "ModfdCshSttlm")]
	pub modfd_csh_sttlm: Option<Vec<CashSettlement4>>,
	#[serde(rename = "CtrctRef")]
	pub ctrct_ref: Option<String>,
	#[serde(rename = "RltdCtrctRef")]
	pub rltd_ctrct_ref: Option<String>,
	#[serde(rename = "PdctId")]
	pub pdct_id: Option<String>,
	#[serde(rename = "SLAChrgAndComssnRef")]
	pub sla_chrg_and_comssn_ref: Option<String>,
	#[validate]
	#[serde(rename = "InsrncCover")]
	pub insrnc_cover: Option<InsuranceType2Choice>,
	#[validate]
	#[serde(rename = "PlanSts")]
	pub plan_sts: Option<PlanStatus2Choice>,
	#[validate]
	#[serde(rename = "InstlmtMgrRole")]
	pub instlmt_mgr_role: Option<PartyRole4Choice>,
}


// InvestorProfile2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestorProfile2 {
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: Option<ProfileType1Choice>,
	#[validate]
	#[serde(rename = "Sts")]
	pub sts: Option<InvestorProfileStatus1Choice>,
	#[validate]
	#[serde(rename = "Trsr")]
	pub trsr: Option<TreasuryProfile1>,
	#[validate]
	#[serde(rename = "HghFrqcyTradg")]
	pub hgh_frqcy_tradg: Option<HighFrequencyTradingProfile1>,
	#[validate]
	#[serde(rename = "MktMakr")]
	pub mkt_makr: Option<MarketMakerProfile2>,
}


// InvestorProfileStatus1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestorProfileStatus1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// InvestorProfileStatus1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestorProfileStatus1Code {
	#[validate(enumerate = ["DISA", "DISG", "ENAB", "ENBG", "ADMI", "ANLY", "NAPP", "PSUS", "PEND", "SUPS"])]
	#[serde(rename = "InvestorProfileStatus1Code")]
	pub investor_profile_status1_code: String,
}


// IrishNSCIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct IrishNSCIdentifier {
	#[validate(pattern = "IE[0-9]{6,6}")]
	#[serde(rename = "IrishNSCIdentifier")]
	pub irish_nsc_identifier: String,
}


// ItalianDomesticIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ItalianDomesticIdentifier {
	#[validate(pattern = "IT[0-9]{10,10}")]
	#[serde(rename = "ItalianDomesticIdentifier")]
	pub italian_domestic_identifier: String,
}


// KYCCheckType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct KYCCheckType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// KnowYourCustomerCheckType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct KnowYourCustomerCheckType1Code {
	#[validate(enumerate = ["ENHA", "ORDN", "SIMP"])]
	#[serde(rename = "KnowYourCustomerCheckType1Code")]
	pub know_your_customer_check_type1_code: String,
}


// LEIIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[validate(pattern = "[A-Z0-9]{18,18}[0-9]{2,2}")]
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LanguageCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LanguageCode {
	#[serde(rename = "LanguageCode")]
	pub language_code: String,
}


// LetterIntent1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LetterIntent1 {
	#[serde(rename = "LttrInttRef")]
	pub lttr_intt_ref: String,
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
	#[serde(rename = "StartDt")]
	pub start_dt: Option<String>,
	#[serde(rename = "EndDt")]
	pub end_dt: Option<String>,
}


// LevelOfControl1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LevelOfControl1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// LevelOfControl1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LevelOfControl1Code {
	#[validate(enumerate = ["TRAN", "VIEW"])]
	#[serde(rename = "LevelOfControl1Code")]
	pub level_of_control1_code: String,
}


// Liability1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Liability1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// Liability1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Liability1Code {
	#[validate(enumerate = ["INVE", "BROK"])]
	#[serde(rename = "Liability1Code")]
	pub liability1_code: String,
}


// MICIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MICIdentifier {
	#[validate(pattern = "[A-Z0-9]{4,4}")]
	#[serde(rename = "MICIdentifier")]
	pub mic_identifier: String,
}


// MailType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MailType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// MailType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MailType1Code {
	#[validate(enumerate = ["AIRM", "ORDM", "REGM"])]
	#[serde(rename = "MailType1Code")]
	pub mail_type1_code: String,
}


// MarketMakerProfile2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MarketMakerProfile2 {
	#[validate]
	#[serde(rename = "CtrctPrd")]
	pub ctrct_prd: Option<DateTimePeriod2>,
	#[serde(rename = "Cmplc")]
	pub cmplc: Option<bool>,
	#[serde(rename = "MaxSprd")]
	pub max_sprd: Option<f64>,
	#[serde(rename = "Dscnt")]
	pub dscnt: Option<f64>,
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


// Max10Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max10Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 10)]
	#[serde(rename = "Max10Text")]
	pub max10_text: String,
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


// Max3Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max3Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 3)]
	#[serde(rename = "Max3Text")]
	pub max3_text: String,
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


// MiFIDClassification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MiFIDClassification1 {
	#[serde(rename = "Clssfctn")]
	pub clssfctn: String,
	#[serde(rename = "Nrrtv")]
	pub nrrtv: Option<String>,
}


// ModificationScope21 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ModificationScope21 {
	#[serde(rename = "ModScpIndctn")]
	pub mod_scp_indctn: String,
	#[validate]
	#[serde(rename = "IsseAllcn")]
	pub isse_allcn: NewIssueAllocation2,
}


// ModificationScope27 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ModificationScope27 {
	#[serde(rename = "ModScpIndctn")]
	pub mod_scp_indctn: String,
	#[validate]
	#[serde(rename = "InvstrPrflVldtn")]
	pub invstr_prfl_vldtn: PartyProfileInformation5,
}


// ModificationScope34 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ModificationScope34 {
	#[serde(rename = "ModScpIndctn")]
	pub mod_scp_indctn: String,
	#[validate]
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: PostalAddress21,
}


// ModificationScope39 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ModificationScope39 {
	#[serde(rename = "ModScpIndctn")]
	pub mod_scp_indctn: String,
	#[validate]
	#[serde(rename = "Ctznsh")]
	pub ctznsh: CitizenshipInformation2,
}


// ModificationScope40 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ModificationScope40 {
	#[serde(rename = "ModScpIndctn")]
	pub mod_scp_indctn: String,
	#[validate]
	#[serde(rename = "Intrmy")]
	pub intrmy: Intermediary46,
}


// ModificationScope41 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ModificationScope41 {
	#[serde(rename = "ModScpIndctn")]
	pub mod_scp_indctn: String,
	#[validate]
	#[serde(rename = "InvstmtPlan")]
	pub invstmt_plan: InvestmentPlan16,
}


// ModificationScope42 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ModificationScope42 {
	#[serde(rename = "ModScpIndctn")]
	pub mod_scp_indctn: String,
	#[validate]
	#[serde(rename = "FinInstrmDtls")]
	pub fin_instrm_dtls: FinancialInstrument87,
}


// ModificationScope43 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ModificationScope43 {
	#[serde(rename = "ModScpIndctn")]
	pub mod_scp_indctn: String,
	#[validate]
	#[serde(rename = "Plcmnt")]
	pub plcmnt: ReferredAgent3,
}


// ModificationScope44 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ModificationScope44 {
	#[serde(rename = "ModScpIndctn")]
	pub mod_scp_indctn: String,
	#[validate]
	#[serde(rename = "SvcLvlAgrmt")]
	pub svc_lvl_agrmt: DocumentToSend4,
}


// ModificationScope45 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ModificationScope45 {
	#[serde(rename = "ModScpIndctn")]
	pub mod_scp_indctn: String,
	#[validate]
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Vec<AdditiononalInformation13>,
}


// ModificationScope46 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ModificationScope46 {
	#[serde(rename = "ModScpIndctn")]
	pub mod_scp_indctn: String,
	#[validate]
	#[serde(rename = "InvstrPrfl")]
	pub invstr_prfl: InvestorProfile2,
}


// MoneyLaunderingCheck1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MoneyLaunderingCheck1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// MoneyLaunderingCheck1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MoneyLaunderingCheck1Code {
	#[validate(enumerate = ["PASS", "NOTC", "EXEM", "CLMO", "AUTH", "POEP"])]
	#[serde(rename = "MoneyLaunderingCheck1Code")]
	pub money_laundering_check1_code: String,
}


// NameAndAddress15 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NameAndAddress15 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[validate]
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress21>,
}


// NameAndAddress4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NameAndAddress4 {
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[validate]
	#[serde(rename = "Adr")]
	pub adr: PostalAddress1,
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


// NamePrefix1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NamePrefix1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// NamePrefix1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NamePrefix1Code {
	#[validate(enumerate = ["DOCT", "MIST", "MISS", "MADM"])]
	#[serde(rename = "NamePrefix1Code")]
	pub name_prefix1_code: String,
}


// NationalityCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NationalityCode {
	#[serde(rename = "NationalityCode")]
	pub nationality_code: String,
}


// NewIssueAllocation2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NewIssueAllocation2 {
	#[serde(rename = "Rstrctd")]
	pub rstrctd: bool,
	#[serde(rename = "XmptPrsnRsn")]
	pub xmpt_prsn_rsn: Option<String>,
	#[validate]
	#[serde(rename = "DeMnms")]
	pub de_mnms: Option<DeMinimus1Choice>,
}


// NewZealandNCCIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NewZealandNCCIdentifier {
	#[validate(pattern = "NZ[0-9]{6,6}")]
	#[serde(rename = "NewZealandNCCIdentifier")]
	pub new_zealand_ncc_identifier: String,
}


// NoReasonCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NoReasonCode {
	#[validate(enumerate = ["NORE"])]
	#[serde(rename = "NoReasonCode")]
	pub no_reason_code: String,
}


// Notification2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Notification2 {
	#[serde(rename = "NtfctnTp")]
	pub ntfctn_tp: String,
	#[serde(rename = "Reqrd")]
	pub reqrd: bool,
	#[validate]
	#[serde(rename = "DstrbtnTp")]
	pub dstrbtn_tp: Option<InformationDistribution1Choice>,
}


// Number ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
}


// OperationalStatus1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OperationalStatus1Code {
	#[validate(enumerate = ["ENAB", "SPEC"])]
	#[serde(rename = "OperationalStatus1Code")]
	pub operational_status1_code: String,
}


// OrderOriginatorEligibility1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OrderOriginatorEligibility1Code {
	#[validate(enumerate = ["ELIG", "RETL", "PROF"])]
	#[serde(rename = "OrderOriginatorEligibility1Code")]
	pub order_originator_eligibility1_code: String,
}


// Organisation23 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Organisation23 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "ShrtNm")]
	pub shrt_nm: Option<String>,
	#[validate]
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Vec<PostalAddress21>,
}


// Organisation40 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Organisation40 {
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "ShrtNm")]
	pub shrt_nm: Option<String>,
	#[validate]
	#[serde(rename = "Id")]
	pub id: Option<PartyIdentification177Choice>,
	#[serde(rename = "LglNttyIdr")]
	pub lgl_ntty_idr: Option<String>,
	#[serde(rename = "Purp")]
	pub purp: Option<String>,
	#[serde(rename = "RegnCtry")]
	pub regn_ctry: Option<String>,
	#[serde(rename = "RegnDt")]
	pub regn_dt: Option<String>,
	#[validate]
	#[serde(rename = "ModfdPstlAdr")]
	pub modfd_pstl_adr: Option<Vec<ModificationScope34>>,
	#[validate]
	#[serde(rename = "TpOfOrg")]
	pub tp_of_org: Option<OrganisationType1Choice>,
	#[serde(rename = "PlcOfListg")]
	pub plc_of_listg: Option<Vec<String>>,
}


// OrganisationType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OrganisationType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// OrganisationType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OrganisationType1Code {
	#[validate(enumerate = ["IFUN", "PRIV", "PUBL", "PFUN"])]
	#[serde(rename = "OrganisationType1Code")]
	pub organisation_type1_code: String,
}


// OtherIdentification3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OtherIdentification3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// OwnershipBeneficiaryRate1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OwnershipBeneficiaryRate1 {
	#[serde(rename = "Rate")]
	pub rate: Option<f64>,
	#[serde(rename = "Frctn")]
	pub frctn: Option<String>,
}


// OwnershipType2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OwnershipType2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// Party48Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Party48Choice {
	#[validate]
	#[serde(rename = "Org")]
	pub org: Option<Organisation40>,
	#[validate]
	#[serde(rename = "IndvPrsn")]
	pub indv_prsn: Option<IndividualPerson38>,
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


// PartyIdentification177Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification177Choice {
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
	#[validate]
	#[serde(rename = "PrtryId")]
	pub prtry_id: Option<GenericIdentification1>,
}


// PartyIdentification182Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification182Choice {
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
	#[validate]
	#[serde(rename = "PrtryId")]
	pub prtry_id: Option<GenericIdentification1>,
	#[validate]
	#[serde(rename = "NmAndAdr")]
	pub nm_and_adr: Option<NameAndAddress15>,
	#[serde(rename = "TaxIdNb")]
	pub tax_id_nb: Option<String>,
	#[serde(rename = "NtlRegnNb")]
	pub ntl_regn_nb: Option<String>,
}


// PartyIdentification220 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification220 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: Option<PartyIdentification182Choice>,
	#[serde(rename = "LglNttyIdr")]
	pub lgl_ntty_idr: Option<String>,
}


// PartyIdentificationType7Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentificationType7Code {
	#[validate(enumerate = ["ATIN", "IDCD", "NRIN", "OTHR", "PASS", "POCD", "SOCS", "SRSA", "GUNL", "GTIN", "ITIN", "CPFA", "AREG", "DRLC", "EMID", "NINV", "INCL", "GIIN"])]
	#[serde(rename = "PartyIdentificationType7Code")]
	pub party_identification_type7_code: String,
}


// PartyProfileInformation5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyProfileInformation5 {
	#[serde(rename = "CertfctnInd")]
	pub certfctn_ind: Option<bool>,
	#[serde(rename = "VldtngPty")]
	pub vldtng_pty: Option<String>,
	#[serde(rename = "ChckngPty")]
	pub chckng_pty: Option<String>,
	#[serde(rename = "RspnsblPty")]
	pub rspnsbl_pty: Option<String>,
	#[validate]
	#[serde(rename = "CertTp")]
	pub cert_tp: Option<CertificationType1Choice>,
	#[serde(rename = "ChckngDt")]
	pub chckng_dt: Option<String>,
	#[serde(rename = "ChckngFrqcy")]
	pub chckng_frqcy: Option<String>,
	#[serde(rename = "NxtRvsnDt")]
	pub nxt_rvsn_dt: Option<String>,
	#[serde(rename = "SlryRg")]
	pub slry_rg: Option<String>,
	#[serde(rename = "SrcOfWlth")]
	pub src_of_wlth: Option<String>,
	#[validate]
	#[serde(rename = "CstmrCndctClssfctn")]
	pub cstmr_cndct_clssfctn: Option<CustomerConductClassification1Choice>,
	#[validate]
	#[serde(rename = "RskLvl")]
	pub rsk_lvl: Option<RiskLevel2Choice>,
	#[validate]
	#[serde(rename = "KnowYourCstmrChckTp")]
	pub know_your_cstmr_chck_tp: Option<KYCCheckType1Choice>,
	#[validate]
	#[serde(rename = "KnowYourCstmrDBChck")]
	pub know_your_cstmr_db_chck: Option<DataBaseCheck1>,
}


// PartyRole1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyRole1Code {
	#[validate(enumerate = ["CUST", "INVS"])]
	#[serde(rename = "PartyRole1Code")]
	pub party_role1_code: String,
}


// PartyRole2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyRole2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// PartyRole4Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyRole4Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// PartyRole5Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyRole5Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// PaymentCard29 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PaymentCard29 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "Nb")]
	pub nb: String,
	#[serde(rename = "HldrNm")]
	pub hldr_nm: String,
	#[serde(rename = "StartDt")]
	pub start_dt: Option<String>,
	#[serde(rename = "XpryDt")]
	pub xpry_dt: String,
	#[serde(rename = "CardIssrNm")]
	pub card_issr_nm: Option<String>,
	#[validate]
	#[serde(rename = "CardIssrId")]
	pub card_issr_id: Option<PartyIdentification125Choice>,
	#[serde(rename = "SctyCd")]
	pub scty_cd: Option<String>,
	#[serde(rename = "SeqNb")]
	pub seq_nb: Option<String>,
}


// PaymentInstrument17 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PaymentInstrument17 {
	#[serde(rename = "SttlmCcy")]
	pub sttlm_ccy: String,
	#[serde(rename = "DvddPctg")]
	pub dvdd_pctg: Option<f64>,
	#[validate]
	#[serde(rename = "SbcptPmtInstrm")]
	pub sbcpt_pmt_instrm: Option<PaymentInstrument24Choice>,
	#[validate]
	#[serde(rename = "RedPmtInstrm")]
	pub red_pmt_instrm: Option<PaymentInstrument19Choice>,
	#[validate]
	#[serde(rename = "DvddPmtInstrm")]
	pub dvdd_pmt_instrm: Option<PaymentInstrument19Choice>,
	#[validate]
	#[serde(rename = "SvgsPlanPmtInstrm")]
	pub svgs_plan_pmt_instrm: Option<PaymentInstrument24Choice>,
	#[validate]
	#[serde(rename = "IntrstPmtInstrm")]
	pub intrst_pmt_instrm: Option<PaymentInstrument19Choice>,
}


// PaymentInstrument19Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PaymentInstrument19Choice {
	#[validate]
	#[serde(rename = "ChqDtls")]
	pub chq_dtls: Option<Cheque4>,
	#[validate]
	#[serde(rename = "BkrsDrftDtls")]
	pub bkrs_drft_dtls: Option<Cheque4>,
}


// PaymentInstrument24Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PaymentInstrument24Choice {
	#[validate]
	#[serde(rename = "PmtCardDtls")]
	pub pmt_card_dtls: Option<PaymentCard29>,
	#[validate]
	#[serde(rename = "DrctDbtDtls")]
	pub drct_dbt_dtls: Option<DirectDebitMandate7>,
	#[serde(rename = "Chq")]
	pub chq: Option<bool>,
	#[serde(rename = "BkrsDrft")]
	pub bkrs_drft: Option<bool>,
}


// PercentageBoundedRate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PercentageBoundedRate {
	#[serde(rename = "PercentageBoundedRate")]
	pub percentage_bounded_rate: f64,
}


// PercentageRate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// PersonalInformation1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PersonalInformation1 {
	#[serde(rename = "NmOfFthr")]
	pub nm_of_fthr: Option<String>,
	#[serde(rename = "MdnNmOfMthr")]
	pub mdn_nm_of_mthr: Option<String>,
	#[serde(rename = "NmOfPrtnr")]
	pub nm_of_prtnr: Option<String>,
}


// PhoneNumber ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PhoneNumber {
	#[validate(pattern = "\\+[0-9]{1,3}-[0-9()+\\-]{1,30}")]
	#[serde(rename = "PhoneNumber")]
	pub phone_number: String,
}


// PlanStatus1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PlanStatus1Code {
	#[validate(enumerate = ["ACTV", "CLOS", "SUSP"])]
	#[serde(rename = "PlanStatus1Code")]
	pub plan_status1_code: String,
}


// PlanStatus2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PlanStatus2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// PoliticalExposureType2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PoliticalExposureType2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// PoliticalExposureType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PoliticalExposureType2Code {
	#[validate(enumerate = ["NPEX", "YPEX", "PEXD", "PEXF"])]
	#[serde(rename = "PoliticalExposureType2Code")]
	pub political_exposure_type2_code: String,
}


// PoliticallyExposedPerson1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PoliticallyExposedPerson1 {
	#[validate]
	#[serde(rename = "PltclyXpsdPrsnTp")]
	pub pltcly_xpsd_prsn_tp: PoliticalExposureType2Choice,
	#[validate]
	#[serde(rename = "PltclyXpsdPrsnSts")]
	pub pltcly_xpsd_prsn_sts: Option<PoliticallyExposedPersonStatus1Choice>,
}


// PoliticallyExposedPersonStatus1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PoliticallyExposedPersonStatus1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// PoliticallyExposedPersonStatus1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PoliticallyExposedPersonStatus1Code {
	#[validate(enumerate = ["PE03", "PE01", "PE02"])]
	#[serde(rename = "PoliticallyExposedPersonStatus1Code")]
	pub politically_exposed_person_status1_code: String,
}


// PortugueseNCCIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PortugueseNCCIdentifier {
	#[validate(pattern = "PT[0-9]{8,8}")]
	#[serde(rename = "PortugueseNCCIdentifier")]
	pub portuguese_ncc_identifier: String,
}


// PositionEffect3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PositionEffect3Code {
	#[validate(enumerate = ["FIFO", "LIFO"])]
	#[serde(rename = "PositionEffect3Code")]
	pub position_effect3_code: String,
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


// PostalAddress21 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PostalAddress21 {
	#[validate]
	#[serde(rename = "AdrTp")]
	pub adr_tp: Option<AddressType2Choice>,
	#[serde(rename = "MlngInd")]
	pub mlng_ind: Option<bool>,
	#[serde(rename = "RegnAdrInd")]
	pub regn_adr_ind: Option<bool>,
	#[serde(rename = "CareOf")]
	pub care_of: Option<String>,
	#[serde(rename = "AdrLine")]
	pub adr_line: Option<Vec<String>>,
	#[serde(rename = "StrtNm")]
	pub strt_nm: Option<String>,
	#[serde(rename = "BldgNb")]
	pub bldg_nb: Option<String>,
	#[serde(rename = "BldgNm")]
	pub bldg_nm: Option<String>,
	#[serde(rename = "PstBx")]
	pub pst_bx: Option<String>,
	#[serde(rename = "SdInBldg")]
	pub sd_in_bldg: Option<String>,
	#[serde(rename = "Flr")]
	pub flr: Option<String>,
	#[serde(rename = "SuiteId")]
	pub suite_id: Option<String>,
	#[serde(rename = "PstCd")]
	pub pst_cd: Option<String>,
	#[serde(rename = "DstrctNm")]
	pub dstrct_nm: Option<String>,
	#[serde(rename = "Vllg")]
	pub vllg: Option<String>,
	#[serde(rename = "TwnNm")]
	pub twn_nm: Option<String>,
	#[serde(rename = "Stat")]
	pub stat: Option<String>,
	#[serde(rename = "Ctry")]
	pub ctry: String,
}


// ProfileType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ProfileType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// ProfileType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ProfileType1Code {
	#[validate(enumerate = ["HEDG", "HFTR", "MAKE", "TREA"])]
	#[serde(rename = "ProfileType1Code")]
	pub profile_type1_code: String,
}


// Provided1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Provided1Code {
	#[validate(enumerate = ["NPRO", "PROV"])]
	#[serde(rename = "Provided1Code")]
	pub provided1_code: String,
}


// QUICKIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct QUICKIdentifier {
	#[serde(rename = "QUICKIdentifier")]
	pub quick_identifier: String,
}


// RICIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RICIdentifier {
	#[validate(min_length = 1)]
	#[validate(max_length = 35)]
	#[serde(rename = "RICIdentifier")]
	pub ric_identifier: String,
}


// Rank1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Rank1Code {
	#[validate(enumerate = ["PRIM", "SECO"])]
	#[serde(rename = "Rank1Code")]
	pub rank1_code: String,
}


// Referred1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Referred1Code {
	#[validate(enumerate = ["REFR", "NRFR", "UKNW"])]
	#[serde(rename = "Referred1Code")]
	pub referred1_code: String,
}


// ReferredAgent3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReferredAgent3 {
	#[serde(rename = "Rfrd")]
	pub rfrd: String,
	#[validate]
	#[serde(rename = "RfrdPlcmntAgt")]
	pub rfrd_plcmnt_agt: Option<PartyIdentification125Choice>,
}


// RegisteredShareholderName1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RegisteredShareholderName1Choice {
	#[validate]
	#[serde(rename = "IndvPrsn")]
	pub indv_prsn: Option<IndividualPerson29>,
	#[validate]
	#[serde(rename = "Org")]
	pub org: Option<Organisation23>,
}


// RegulatoryInformation1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RegulatoryInformation1 {
	#[serde(rename = "Sctr")]
	pub sctr: Option<String>,
	#[serde(rename = "Brnch")]
	pub brnch: Option<String>,
	#[serde(rename = "Grp")]
	pub grp: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<String>,
}


// Reinvestment4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Reinvestment4 {
	#[validate]
	#[serde(rename = "FinInstrmDtls")]
	pub fin_instrm_dtls: FinancialInstrument87,
	#[serde(rename = "ReqdNAVCcy")]
	pub reqd_nav_ccy: Option<String>,
	#[serde(rename = "RinvstmtPctg")]
	pub rinvstmt_pctg: f64,
}


// Repartition6 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Repartition6 {
	#[validate]
	#[serde(rename = "Qty")]
	pub qty: UnitsOrAmountOrPercentage1Choice,
	#[validate]
	#[serde(rename = "FinInstrm")]
	pub fin_instrm: FinancialInstrument87,
	#[serde(rename = "CcyOfPlan")]
	pub ccy_of_plan: Option<String>,
}


// ResidentialStatus1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ResidentialStatus1Code {
	#[validate(enumerate = ["RESI", "PRES", "NRES"])]
	#[serde(rename = "ResidentialStatus1Code")]
	pub residential_status1_code: String,
}


// RestrictionStatus1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RestrictionStatus1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// RestrictionStatus1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RestrictionStatus1Code {
	#[validate(enumerate = ["ACTV", "INAC"])]
	#[serde(rename = "RestrictionStatus1Code")]
	pub restriction_status1_code: String,
}


// RiskLevel1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RiskLevel1Code {
	#[validate(enumerate = ["HIGH", "LOWW", "MEDM"])]
	#[serde(rename = "RiskLevel1Code")]
	pub risk_level1_code: String,
}


// RiskLevel2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RiskLevel2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// RoundingDirection1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RoundingDirection1Code {
	#[validate(enumerate = ["RDUP", "RDWN", "STAN", "DIST"])]
	#[serde(rename = "RoundingDirection1Code")]
	pub rounding_direction1_code: String,
}


// RoundingParameters1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RoundingParameters1 {
	#[serde(rename = "RndgMdlus")]
	pub rndg_mdlus: Option<f64>,
	#[serde(rename = "RndgDrctn")]
	pub rndg_drctn: String,
}


// RussianCentralBankIdentificationCodeIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RussianCentralBankIdentificationCodeIdentifier {
	#[validate(pattern = "RU[0-9]{9,9}")]
	#[serde(rename = "RussianCentralBankIdentificationCodeIdentifier")]
	pub russian_central_bank_identification_code_identifier: String,
}


// SEDOLIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SEDOLIdentifier {
	#[serde(rename = "SEDOLIdentifier")]
	pub sedol_identifier: String,
}


// SecurityIdentification25Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityIdentification25Choice {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[serde(rename = "SEDOL")]
	pub sedol: Option<String>,
	#[serde(rename = "CUSIP")]
	pub cusip: Option<String>,
	#[serde(rename = "RIC")]
	pub ric: Option<String>,
	#[serde(rename = "TckrSymb")]
	pub tckr_symb: Option<String>,
	#[serde(rename = "Blmbrg")]
	pub blmbrg: Option<String>,
	#[serde(rename = "CTA")]
	pub cta: Option<String>,
	#[serde(rename = "QUICK")]
	pub quick: Option<String>,
	#[serde(rename = "Wrtppr")]
	pub wrtppr: Option<String>,
	#[serde(rename = "Dtch")]
	pub dtch: Option<String>,
	#[serde(rename = "Vlrn")]
	pub vlrn: Option<String>,
	#[serde(rename = "SCVM")]
	pub scvm: Option<String>,
	#[serde(rename = "Belgn")]
	pub belgn: Option<String>,
	#[serde(rename = "Cmon")]
	pub cmon: Option<String>,
	#[validate]
	#[serde(rename = "OthrPrtryId")]
	pub othr_prtry_id: Option<AlternateSecurityIdentification7>,
}


// SettlementFrequency1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementFrequency1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// SettlementInstructionReason1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementInstructionReason1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// SettlementInstructionReason1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementInstructionReason1Code {
	#[validate(enumerate = ["CSHI", "ALLL", "CSHO", "CHAR", "DIVI", "INTE", "SAVP", "REDM", "SAVE", "BUYI", "SELL", "SUBS", "WTHP", "CORP"])]
	#[serde(rename = "SettlementInstructionReason1Code")]
	pub settlement_instruction_reason1_code: String,
}


// SicovamIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SicovamIdentifier {
	#[serde(rename = "SicovamIdentifier")]
	pub sicovam_identifier: String,
}


// SimpleIdentificationInformation4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SimpleIdentificationInformation4 {
	#[serde(rename = "Id")]
	pub id: String,
}


// SmallNetworkIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SmallNetworkIdentifier {
	#[validate(pattern = "AU[0-9]{6,6}")]
	#[serde(rename = "SmallNetworkIdentifier")]
	pub small_network_identifier: String,
}


// SouthAfricanNCCIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SouthAfricanNCCIdentifier {
	#[validate(pattern = "ZA[0-9]{6,6}")]
	#[serde(rename = "SouthAfricanNCCIdentifier")]
	pub south_african_ncc_identifier: String,
}


// SpanishDomesticInterbankingIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SpanishDomesticInterbankingIdentifier {
	#[validate(pattern = "ES[0-9]{8,9}")]
	#[serde(rename = "SpanishDomesticInterbankingIdentifier")]
	pub spanish_domestic_interbanking_identifier: String,
}


// StatementFrequencyReason2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct StatementFrequencyReason2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// SwissBCIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SwissBCIdentifier {
	#[validate(pattern = "SW[0-9]{3,5}")]
	#[serde(rename = "SwissBCIdentifier")]
	pub swiss_bc_identifier: String,
}


// SwissSICIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SwissSICIdentifier {
	#[validate(pattern = "SW[0-9]{6,6}")]
	#[serde(rename = "SwissSICIdentifier")]
	pub swiss_sic_identifier: String,
}


// TaxExemptReason3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TaxExemptReason3Code {
	#[validate(enumerate = ["NONE", "MASA", "MISA", "SISA", "IISA", "CUYP", "PRYP", "ASTR", "EMPY", "EMCY", "EPRY", "ECYE", "NFPI", "NFQP", "DECP", "IRAC", "IRAR", "KEOG", "PFSP", "401K", "SIRA", "403B", "457X", "RIRA", "RIAN", "RCRF", "RCIP", "EIFP", "EIOP", "FORE", "INCA", "MINO", "ASSO", "DIPL", "DOME", "FORP", "ORDR", "PENF", "REFU", "RIHO", "ADMI", "TANR", "OANR"])]
	#[serde(rename = "TaxExemptReason3Code")]
	pub tax_exempt_reason3_code: String,
}


// TaxExemptionReason2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TaxExemptionReason2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// TaxReporting3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TaxReporting3 {
	#[serde(rename = "TaxtnCtry")]
	pub taxtn_ctry: String,
	#[serde(rename = "TaxRate")]
	pub tax_rate: Option<f64>,
	#[validate]
	#[serde(rename = "TaxPyer")]
	pub tax_pyer: Option<PartyIdentification125Choice>,
	#[validate]
	#[serde(rename = "TaxRcpt")]
	pub tax_rcpt: Option<PartyIdentification125Choice>,
	#[validate]
	#[serde(rename = "CshAcctDtls")]
	pub csh_acct_dtls: Option<CashAccount204>,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// TaxWithholdingMethod3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TaxWithholdingMethod3Code {
	#[validate(enumerate = ["MITX", "INVE", "ACCT", "EXMT", "REPT", "CRTF", "WHCO", "WTHD", "WTRE"])]
	#[serde(rename = "TaxWithholdingMethod3Code")]
	pub tax_withholding_method3_code: String,
}


// ThirdPartyRights2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ThirdPartyRights2 {
	#[serde(rename = "Tp")]
	pub tp: String,
	#[serde(rename = "DtTm")]
	pub dt_tm: Option<String>,
	#[validate]
	#[serde(rename = "Hldr")]
	pub hldr: Option<PartyIdentification125Choice>,
	#[serde(rename = "LglNttyIdr")]
	pub lgl_ntty_idr: Option<String>,
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// TickerIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TickerIdentifier {
	#[validate(min_length = 1)]
	#[validate(max_length = 35)]
	#[serde(rename = "TickerIdentifier")]
	pub ticker_identifier: String,
}


// TransactionChannel2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TransactionChannel2Code {
	#[validate(enumerate = ["FIAD", "HOBA", "BRAN"])]
	#[serde(rename = "TransactionChannel2Code")]
	pub transaction_channel2_code: String,
}


// TransactionChannelType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TransactionChannelType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// TransactionType5Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TransactionType5Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification47>,
}


// TreasuryProfile1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TreasuryProfile1 {
	#[serde(rename = "Dt")]
	pub dt: String,
	#[validate]
	#[serde(rename = "TradrTp")]
	pub tradr_tp: PartyRole5Choice,
	#[serde(rename = "Rate")]
	pub rate: f64,
}


// UKDomesticSortCodeIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct UKDomesticSortCodeIdentifier {
	#[validate(pattern = "SC[0-9]{6,6}")]
	#[serde(rename = "UKDomesticSortCodeIdentifier")]
	pub uk_domestic_sort_code_identifier: String,
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


// UnitsOrAmountOrPercentage1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct UnitsOrAmountOrPercentage1Choice {
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "Unit")]
	pub unit: Option<f64>,
	#[serde(rename = "Pctg")]
	pub pctg: Option<f64>,
}


// ValorenIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ValorenIdentifier {
	#[serde(rename = "ValorenIdentifier")]
	pub valoren_identifier: String,
}


// WertpapierIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct WertpapierIdentifier {
	#[serde(rename = "WertpapierIdentifier")]
	pub wertpapier_identifier: String,
}


// YesNoIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "YesNoIndicator")]
	pub yes_no_indicator: bool,
}
