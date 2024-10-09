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


// AccountContract2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountContract2 {
	#[serde(rename = "TrgtGoLiveDt")]
	pub trgt_go_live_dt: Option<String>,
	#[serde(rename = "TrgtClsgDt")]
	pub trgt_clsg_dt: Option<String>,
	#[serde(rename = "UrgcyFlg")]
	pub urgcy_flg: Option<bool>,
}


// AccountForAction1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountForAction1 {
	#[serde(rename = "Id")]
	pub id: AccountIdentification4Choice,
	#[serde(rename = "Ccy")]
	pub ccy: String,
}


// AccountIdentification4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountIdentification4Choice {
	#[serde(rename = "IBAN")]
	pub iban: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<GenericAccountIdentification1>,
}


// AccountMandateMaintenanceAmendmentRequestV04 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountMandateMaintenanceAmendmentRequestV04 {
	#[serde(rename = "Refs")]
	pub refs: References4,
	#[serde(rename = "Fr")]
	pub fr: Option<OrganisationIdentification39>,
	#[serde(rename = "CtrctDts")]
	pub ctrct_dts: Option<AccountContract2>,
	#[serde(rename = "UndrlygMstrAgrmt")]
	pub undrlyg_mstr_agrmt: Option<ContractDocument1>,
	#[serde(rename = "AcctId")]
	pub acct_id: Vec<AccountForAction1>,
	#[serde(rename = "AcctSvcrId")]
	pub acct_svcr_id: BranchAndFinancialInstitutionIdentification8,
	#[serde(rename = "OrgId")]
	pub org_id: OrganisationIdentification39,
	#[serde(rename = "Mndt")]
	pub mndt: Option<Vec<OperationMandate6>>,
	#[serde(rename = "Grp")]
	pub grp: Option<Vec<Group5>>,
	#[serde(rename = "DgtlSgntr")]
	pub dgtl_sgntr: Option<Vec<PartyAndSignature4>>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
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


// Authorisation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Authorisation2 {
	#[serde(rename = "MaxAmtByTx")]
	pub max_amt_by_tx: Option<FixedAmountOrUnlimited1Choice>,
	#[serde(rename = "MaxAmtByPrd")]
	pub max_amt_by_prd: Option<Vec<MaximumAmountByPeriod1>>,
	#[serde(rename = "MaxAmtByBlkSubmissn")]
	pub max_amt_by_blk_submissn: Option<FixedAmountOrUnlimited1Choice>,
}


// BICFIDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BICFIDec2014Identifier {
	#[serde(rename = "BICFIDec2014Identifier")]
	pub bicfi_dec2014_identifier: String,
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


// Channel2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Channel2Choice {
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


// CommunicationMethod3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommunicationMethod3Code {
	#[serde(rename = "CommunicationMethod3Code")]
	pub communication_method3_code: String,
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


// ContractDocument1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractDocument1 {
	#[serde(rename = "Ref")]
	pub ref_attr: String,
	#[serde(rename = "SgnOffDt")]
	pub sgn_off_dt: Option<String>,
	#[serde(rename = "Vrsn")]
	pub vrsn: Option<String>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
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


// FixedAmountOrUnlimited1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FixedAmountOrUnlimited1Choice {
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "NotLtd")]
	pub not_ltd: Option<String>,
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


// Group5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Group5 {
	#[serde(rename = "ModCd")]
	pub mod_cd: Option<String>,
	#[serde(rename = "GrpId")]
	pub grp_id: String,
	#[serde(rename = "Pty")]
	pub pty: Vec<PartyAndCertificate7>,
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


// Max10KBinary ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max10KBinary {
	#[serde(rename = "Max10KBinary")]
	pub max10_k_binary: String,
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


// Max3NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max3NumericText {
	#[serde(rename = "Max3NumericText")]
	pub max3_numeric_text: String,
}


// Max4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "Max4AlphaNumericText")]
	pub max4_alpha_numeric_text: String,
}


// Max4Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4Text {
	#[serde(rename = "Max4Text")]
	pub max4_text: String,
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


// MaximumAmountByPeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MaximumAmountByPeriod1 {
	#[serde(rename = "MaxAmt")]
	pub max_amt: ActiveCurrencyAndAmount,
	#[serde(rename = "NbOfDays")]
	pub nb_of_days: String,
}


// MessageIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
}


// Modification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Modification1Code {
	#[serde(rename = "Modification1Code")]
	pub modification1_code: String,
}


// NamePrefix2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NamePrefix2Code {
	#[serde(rename = "NamePrefix2Code")]
	pub name_prefix2_code: String,
}


// OperationMandate6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OperationMandate6 {
	#[serde(rename = "ModCd")]
	pub mod_cd: Option<String>,
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "AplblChanl")]
	pub aplbl_chanl: Vec<Channel2Choice>,
	#[serde(rename = "ReqrdSgntrNb")]
	pub reqrd_sgntr_nb: String,
	#[serde(rename = "SgntrOrdrInd")]
	pub sgntr_ordr_ind: bool,
	#[serde(rename = "MndtHldr")]
	pub mndt_hldr: Option<Vec<PartyAndAuthorisation6>>,
	#[serde(rename = "BkOpr")]
	pub bk_opr: Vec<BankTransactionCodeStructure4>,
	#[serde(rename = "StartDt")]
	pub start_dt: Option<String>,
	#[serde(rename = "EndDt")]
	pub end_dt: Option<String>,
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


// PartyAndAuthorisation6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyAndAuthorisation6 {
	#[serde(rename = "ModCd")]
	pub mod_cd: Option<String>,
	#[serde(rename = "PtyOrGrp")]
	pub pty_or_grp: PartyOrGroup3Choice,
	#[serde(rename = "SgntrOrdr")]
	pub sgntr_ordr: Option<String>,
	#[serde(rename = "Authstn")]
	pub authstn: Authorisation2,
}


// PartyAndCertificate6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyAndCertificate6 {
	#[serde(rename = "Pty")]
	pub pty: PartyIdentification272,
	#[serde(rename = "Cert")]
	pub cert: Option<String>,
}


// PartyAndCertificate7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyAndCertificate7 {
	#[serde(rename = "ModCd")]
	pub mod_cd: Option<String>,
	#[serde(rename = "Pty")]
	pub pty: PartyIdentification272,
	#[serde(rename = "Cert")]
	pub cert: Option<String>,
}


// PartyAndSignature4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyAndSignature4 {
	#[serde(rename = "Pty")]
	pub pty: PartyIdentification272,
	#[serde(rename = "Sgntr")]
	pub sgntr: SkipPayload,
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


// PartyOrGroup3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyOrGroup3Choice {
	#[serde(rename = "GrpId")]
	pub grp_id: Option<String>,
	#[serde(rename = "Pty")]
	pub pty: Option<PartyAndCertificate6>,
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


// ProprietaryBankTransactionCodeStructure1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProprietaryBankTransactionCodeStructure1 {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// References4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct References4 {
	#[serde(rename = "MsgId")]
	pub msg_id: MessageIdentification1,
	#[serde(rename = "PrcId")]
	pub prc_id: MessageIdentification1,
	#[serde(rename = "AttchdDocNm")]
	pub attchd_doc_nm: Option<Vec<String>>,
}


// SkipPayload ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SkipPayload {
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


// Unlimited9Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Unlimited9Text {
	#[serde(rename = "Unlimited9Text")]
	pub unlimited9_text: String,
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "YesNoIndicator")]
	pub yes_no_indicator: bool,
}
