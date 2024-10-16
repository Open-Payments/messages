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
	#[serde(rename = "TrgtGoLiveDt", skip_serializing_if = "Option::is_none")]
	pub trgt_go_live_dt: Option<String>,
	#[serde(rename = "TrgtClsgDt", skip_serializing_if = "Option::is_none")]
	pub trgt_clsg_dt: Option<String>,
	#[serde(rename = "UrgcyFlg", skip_serializing_if = "Option::is_none")]
	pub urgcy_flg: Option<bool>,
}


// AccountExcludedMandateMaintenanceAmendmentRequestV04 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountExcludedMandateMaintenanceAmendmentRequestV04 {
	#[serde(rename = "Refs")]
	pub refs: References4,
	#[serde(rename = "Fr", skip_serializing_if = "Option::is_none")]
	pub fr: Option<OrganisationIdentification39>,
	#[serde(rename = "CtrctDts", skip_serializing_if = "Option::is_none")]
	pub ctrct_dts: Option<AccountContract2>,
	#[serde(rename = "UndrlygMstrAgrmt", skip_serializing_if = "Option::is_none")]
	pub undrlyg_mstr_agrmt: Option<ContractDocument1>,
	#[serde(rename = "Acct")]
	pub acct: CustomerAccountModification1,
	#[serde(rename = "AcctSvcrId")]
	pub acct_svcr_id: BranchAndFinancialInstitutionIdentification8,
	#[serde(rename = "Org")]
	pub org: OrganisationModification3,
	#[serde(rename = "DgtlSgntr", skip_serializing_if = "Option::is_none")]
	pub dgtl_sgntr: Option<Vec<PartyAndSignature4>>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


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


// AccountStatus3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AccountStatus3Code {
	#[default]
	#[serde(rename = "ENAB")]
	CodeENAB,
	#[serde(rename = "DISA")]
	CodeDISA,
	#[serde(rename = "DELE")]
	CodeDELE,
	#[serde(rename = "FORM")]
	CodeFORM,

}


// AccountStatusModification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountStatusModification1 {
	#[serde(rename = "ModCd", skip_serializing_if = "Option::is_none")]
	pub mod_cd: Option<Modification1Code>,
	#[serde(rename = "Sts")]
	pub sts: AccountStatus3Code,
}


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "$value")]
	pub active_currency_code: String,
}


// AddressModification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressModification3 {
	#[serde(rename = "ModCd", skip_serializing_if = "Option::is_none")]
	pub mod_cd: Option<Modification1Code>,
	#[serde(rename = "Adr")]
	pub adr: PostalAddress27,
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


// AmountModification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountModification1 {
	#[serde(rename = "ModCd", skip_serializing_if = "Option::is_none")]
	pub mod_cd: Option<Modification1Code>,
	#[serde(rename = "Amt")]
	pub amt: f64,
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


// CashAccountType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashAccountType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalCashAccountType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
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


// CodeOrProprietary1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CodeOrProprietary1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Max4Text>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification13>,
}


// CommunicationFormat1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommunicationFormat1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalCommunicationFormat1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// CommunicationMethod2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommunicationMethod2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<CommunicationMethod2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// CommunicationMethod2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CommunicationMethod2Code {
	#[default]
	#[serde(rename = "EMAL")]
	CodeEMAL,
	#[serde(rename = "FAXI")]
	CodeFAXI,
	#[serde(rename = "FILE")]
	CodeFILE,
	#[serde(rename = "ONLI")]
	CodeONLI,
	#[serde(rename = "POST")]
	CodePOST,

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


// ContractDocument1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractDocument1 {
	#[serde(rename = "Ref")]
	pub ref_attr: Max35Text,
	#[serde(rename = "SgnOffDt", skip_serializing_if = "Option::is_none")]
	pub sgn_off_dt: Option<String>,
	#[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
	pub vrsn: Option<Max6Text>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// CustomerAccountModification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CustomerAccountModification1 {
	#[serde(rename = "Id")]
	pub id: Vec<AccountIdentification4Choice>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<NameModification1>,
	#[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
	pub sts: Option<AccountStatusModification1>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<TypeModification1>,
	#[serde(rename = "Ccy")]
	pub ccy: ActiveCurrencyCode,
	#[serde(rename = "MnthlyPmtVal", skip_serializing_if = "Option::is_none")]
	pub mnthly_pmt_val: Option<AmountModification1>,
	#[serde(rename = "MnthlyRcvdVal", skip_serializing_if = "Option::is_none")]
	pub mnthly_rcvd_val: Option<AmountModification1>,
	#[serde(rename = "MnthlyTxNb", skip_serializing_if = "Option::is_none")]
	pub mnthly_tx_nb: Option<NumberModification1>,
	#[serde(rename = "AvrgBal", skip_serializing_if = "Option::is_none")]
	pub avrg_bal: Option<AmountModification1>,
	#[serde(rename = "AcctPurp", skip_serializing_if = "Option::is_none")]
	pub acct_purp: Option<PurposeModification1>,
	#[serde(rename = "FlrNtfctnAmt", skip_serializing_if = "Option::is_none")]
	pub flr_ntfctn_amt: Option<AmountModification1>,
	#[serde(rename = "ClngNtfctnAmt", skip_serializing_if = "Option::is_none")]
	pub clng_ntfctn_amt: Option<AmountModification1>,
	#[serde(rename = "StmtFrqcyAndFrmt", skip_serializing_if = "Option::is_none")]
	pub stmt_frqcy_and_frmt: Option<Vec<StatementFrequencyAndFormModification1>>,
	#[serde(rename = "ClsgDt", skip_serializing_if = "Option::is_none")]
	pub clsg_dt: Option<DateModification1>,
	#[serde(rename = "Rstrctn", skip_serializing_if = "Option::is_none")]
	pub rstrctn: Option<Vec<RestrictionModification1>>,
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


// DateModification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateModification1 {
	#[serde(rename = "ModCd", skip_serializing_if = "Option::is_none")]
	pub mod_cd: Option<Modification1Code>,
	#[serde(rename = "Dt")]
	pub dt: String,
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


// ExternalCashAccountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalCashAccountType1Code {
	#[serde(rename = "$value")]
	pub external_cash_account_type1_code: String,
}


// ExternalClearingSystemIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalClearingSystemIdentification1Code {
	#[serde(rename = "$value")]
	pub external_clearing_system_identification1_code: String,
}


// ExternalCommunicationFormat1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalCommunicationFormat1Code {
	#[serde(rename = "$value")]
	pub external_communication_format1_code: String,
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


// Frequency7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Frequency7Code {
	#[default]
	#[serde(rename = "YEAR")]
	CodeYEAR,
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "QURT")]
	CodeQURT,
	#[serde(rename = "MIAN")]
	CodeMIAN,
	#[serde(rename = "TEND")]
	CodeTEND,
	#[serde(rename = "MOVE")]
	CodeMOVE,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "INDA")]
	CodeINDA,

}


// FullLegalNameModification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FullLegalNameModification1 {
	#[serde(rename = "ModCd", skip_serializing_if = "Option::is_none")]
	pub mod_cd: Option<Modification1Code>,
	#[serde(rename = "FullLglNm")]
	pub full_lgl_nm: Max350Text,
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


// GenericIdentification13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification13 {
	#[serde(rename = "Id")]
	pub id: Max4AlphaNumericText,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
	#[serde(rename = "Issr")]
	pub issr: Max35Text,
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


// ImpliedCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImpliedCurrencyAndAmount {
	#[serde(rename = "$value")]
	pub implied_currency_and_amount: f64,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
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


// Max4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "$value")]
	pub max4_alpha_numeric_text: String,
}


// Max4Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4Text {
	#[serde(rename = "$value")]
	pub max4_text: String,
}


// Max5NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max5NumericText {
	#[serde(rename = "$value")]
	pub max5_numeric_text: String,
}


// Max6Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max6Text {
	#[serde(rename = "$value")]
	pub max6_text: String,
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


// Modification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Modification1Code {
	#[default]
	#[serde(rename = "NOCH")]
	CodeNOCH,
	#[serde(rename = "MODI")]
	CodeMODI,
	#[serde(rename = "DELE")]
	CodeDELE,
	#[serde(rename = "ADDD")]
	CodeADDD,

}


// NameModification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameModification1 {
	#[serde(rename = "ModCd", skip_serializing_if = "Option::is_none")]
	pub mod_cd: Option<Modification1Code>,
	#[serde(rename = "Nm")]
	pub nm: Max70Text,
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


// NumberModification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NumberModification1 {
	#[serde(rename = "ModCd", skip_serializing_if = "Option::is_none")]
	pub mod_cd: Option<Modification1Code>,
	#[serde(rename = "Nb")]
	pub nb: Max5NumericText,
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


// OrganisationModification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationModification3 {
	#[serde(rename = "FullLglNm")]
	pub full_lgl_nm: FullLegalNameModification1,
	#[serde(rename = "TradgNm", skip_serializing_if = "Option::is_none")]
	pub tradg_nm: Option<TradingNameModification1>,
	#[serde(rename = "CtryOfOpr")]
	pub ctry_of_opr: CountryCode,
	#[serde(rename = "RegnDt", skip_serializing_if = "Option::is_none")]
	pub regn_dt: Option<String>,
	#[serde(rename = "OprlAdr", skip_serializing_if = "Option::is_none")]
	pub oprl_adr: Option<AddressModification3>,
	#[serde(rename = "BizAdr", skip_serializing_if = "Option::is_none")]
	pub biz_adr: Option<AddressModification3>,
	#[serde(rename = "LglAdr")]
	pub lgl_adr: AddressModification3,
	#[serde(rename = "BllgAdr", skip_serializing_if = "Option::is_none")]
	pub bllg_adr: Option<AddressModification3>,
	#[serde(rename = "OrgId")]
	pub org_id: OrganisationIdentification39,
	#[serde(rename = "RprtvOffcr", skip_serializing_if = "Option::is_none")]
	pub rprtv_offcr: Option<Vec<PartyModification3>>,
	#[serde(rename = "TrsrMgr", skip_serializing_if = "Option::is_none")]
	pub trsr_mgr: Option<PartyModification3>,
	#[serde(rename = "MainMndtHldr", skip_serializing_if = "Option::is_none")]
	pub main_mndt_hldr: Option<Vec<PartyModification3>>,
	#[serde(rename = "Sndr", skip_serializing_if = "Option::is_none")]
	pub sndr: Option<Vec<PartyModification3>>,
	#[serde(rename = "LglRprtv", skip_serializing_if = "Option::is_none")]
	pub lgl_rprtv: Option<Vec<PartyModification3>>,
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


// PartyIdentification274 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification274 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress27>,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<PersonIdentification18>,
	#[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
	pub ctry_of_res: Option<CountryCode>,
	#[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
	pub ctct_dtls: Option<Contact13>,
}


// PartyModification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyModification3 {
	#[serde(rename = "ModCd", skip_serializing_if = "Option::is_none")]
	pub mod_cd: Option<Modification1Code>,
	#[serde(rename = "PtyId")]
	pub pty_id: PartyIdentification274,
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


// PurposeModification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PurposeModification1 {
	#[serde(rename = "ModCd", skip_serializing_if = "Option::is_none")]
	pub mod_cd: Option<Modification1Code>,
	#[serde(rename = "Purp")]
	pub purp: Max140Text,
}


// References4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct References4 {
	#[serde(rename = "MsgId")]
	pub msg_id: MessageIdentification1,
	#[serde(rename = "PrcId")]
	pub prc_id: MessageIdentification1,
	#[serde(rename = "AttchdDocNm", skip_serializing_if = "Option::is_none")]
	pub attchd_doc_nm: Option<Vec<Max70Text>>,
}


// Restriction1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Restriction1 {
	#[serde(rename = "RstrctnTp")]
	pub rstrctn_tp: CodeOrProprietary1Choice,
	#[serde(rename = "VldFr")]
	pub vld_fr: String,
	#[serde(rename = "VldUntil", skip_serializing_if = "Option::is_none")]
	pub vld_until: Option<String>,
}


// RestrictionModification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RestrictionModification1 {
	#[serde(rename = "ModCd", skip_serializing_if = "Option::is_none")]
	pub mod_cd: Option<Modification1Code>,
	#[serde(rename = "Rstrctn")]
	pub rstrctn: Restriction1,
}


// SkipPayload ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SkipPayload {
}


// StatementFrequencyAndForm1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatementFrequencyAndForm1 {
	#[serde(rename = "Frqcy")]
	pub frqcy: Frequency7Code,
	#[serde(rename = "ComMtd")]
	pub com_mtd: CommunicationMethod2Choice,
	#[serde(rename = "DlvryAdr")]
	pub dlvry_adr: Max350Text,
	#[serde(rename = "Frmt")]
	pub frmt: CommunicationFormat1Choice,
}


// StatementFrequencyAndFormModification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatementFrequencyAndFormModification1 {
	#[serde(rename = "ModCd", skip_serializing_if = "Option::is_none")]
	pub mod_cd: Option<Modification1Code>,
	#[serde(rename = "StmtFrqcyAndForm")]
	pub stmt_frqcy_and_form: StatementFrequencyAndForm1,
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


// TradingNameModification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingNameModification1 {
	#[serde(rename = "ModCd", skip_serializing_if = "Option::is_none")]
	pub mod_cd: Option<Modification1Code>,
	#[serde(rename = "TradgNm")]
	pub tradg_nm: Max350Text,
}


// TypeModification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TypeModification1 {
	#[serde(rename = "ModCd", skip_serializing_if = "Option::is_none")]
	pub mod_cd: Option<Modification1Code>,
	#[serde(rename = "Tp")]
	pub tp: CashAccountType2Choice,
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "$value")]
	pub yes_no_indicator: bool,
}
