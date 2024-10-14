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


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_code: String,
}


// BICFIDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BICFIDec2014Identifier {
	#[serde(rename = "$value")]
	pub bicfi_dec2014_identifier: String,
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


// CommunicationAddress10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommunicationAddress10 {
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: LongPostalAddress1Choice,
	#[serde(rename = "PhneNb")]
	pub phne_nb: PhoneNumber,
	#[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
	pub fax_nb: Option<PhoneNumber>,
	#[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
	pub email_adr: Option<Max2048Text>,
}


// ContactIdentificationAndAddress2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContactIdentificationAndAddress2 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max35Text>,
	#[serde(rename = "Role")]
	pub role: PaymentRole1Choice,
	#[serde(rename = "ComAdr")]
	pub com_adr: CommunicationAddress10,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// ErrorHandling1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ErrorHandling1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max4AlphaNumericText>,
}


// ErrorHandling1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ErrorHandling1Code {
	#[serde(rename = "X020")]
	CodeX020,
	#[serde(rename = "X030")]
	CodeX030,
	#[serde(rename = "X050")]
	CodeX050,

	#[default]
	UNKOWN
}


// ErrorHandling3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ErrorHandling3 {
	#[serde(rename = "Err")]
	pub err: ErrorHandling1Choice,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max140Text>,
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


// ExternalEnquiryRequestType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalEnquiryRequestType1Code {
	#[serde(rename = "$value")]
	pub external_enquiry_request_type1_code: String,
}


// ExternalFinancialInstitutionIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalFinancialInstitutionIdentification1Code {
	#[serde(rename = "$value")]
	pub external_financial_institution_identification1_code: String,
}


// ExternalPaymentControlRequestType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPaymentControlRequestType1Code {
	#[serde(rename = "$value")]
	pub external_payment_control_request_type1_code: String,
}


// ExternalPaymentRole1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPaymentRole1Code {
	#[serde(rename = "$value")]
	pub external_payment_role1_code: String,
}


// ExternalProxyAccountType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalProxyAccountType1Code {
	#[serde(rename = "$value")]
	pub external_proxy_account_type1_code: String,
}


// ExternalSystemMemberType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalSystemMemberType1Code {
	#[serde(rename = "$value")]
	pub external_system_member_type1_code: String,
}


// FinancialIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalFinancialInstitutionIdentification1Code>,
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


// IBAN2007Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IBAN2007Identifier {
	#[serde(rename = "$value")]
	pub iban2007_identifier: String,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}


// LongPostalAddress1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LongPostalAddress1Choice {
	#[serde(rename = "Ustrd", skip_serializing_if = "Option::is_none")]
	pub ustrd: Option<Max140Text>,
	#[serde(rename = "Strd", skip_serializing_if = "Option::is_none")]
	pub strd: Option<StructuredLongPostalAddress1>,
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


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
}


// Member7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Member7 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max35Text>,
	#[serde(rename = "RtrAdr", skip_serializing_if = "Option::is_none")]
	pub rtr_adr: Option<Vec<MemberIdentification3Choice>>,
	#[serde(rename = "Acct", skip_serializing_if = "Option::is_none")]
	pub acct: Option<Vec<CashAccount40>>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<SystemMemberType1Choice>,
	#[serde(rename = "Sts", skip_serializing_if = "Option::is_none")]
	pub sts: Option<SystemMemberStatus1Choice>,
	#[serde(rename = "CtctRef", skip_serializing_if = "Option::is_none")]
	pub ctct_ref: Option<Vec<ContactIdentificationAndAddress2>>,
	#[serde(rename = "ComAdr", skip_serializing_if = "Option::is_none")]
	pub com_adr: Option<CommunicationAddress10>,
}


// MemberIdentification3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MemberIdentification3Choice {
	#[serde(rename = "BICFI", skip_serializing_if = "Option::is_none")]
	pub bicfi: Option<BICFIDec2014Identifier>,
	#[serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none")]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<GenericFinancialIdentification1>,
}


// MemberReport6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MemberReport6 {
	#[serde(rename = "MmbId")]
	pub mmb_id: MemberIdentification3Choice,
	#[serde(rename = "MmbOrErr")]
	pub mmb_or_err: MemberReportOrError8Choice,
}


// MemberReportOrError7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MemberReportOrError7Choice {
	#[serde(rename = "Rpt", skip_serializing_if = "Option::is_none")]
	pub rpt: Option<Vec<MemberReport6>>,
	#[serde(rename = "OprlErr", skip_serializing_if = "Option::is_none")]
	pub oprl_err: Option<Vec<ErrorHandling3>>,
}


// MemberReportOrError8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MemberReportOrError8Choice {
	#[serde(rename = "Mmb", skip_serializing_if = "Option::is_none")]
	pub mmb: Option<Member7>,
	#[serde(rename = "BizErr", skip_serializing_if = "Option::is_none")]
	pub biz_err: Option<ErrorHandling3>,
}


// MemberStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum MemberStatus1Code {
	#[serde(rename = "ENBL")]
	CodeENBL,
	#[serde(rename = "DSBL")]
	CodeDSBL,
	#[serde(rename = "DLTD")]
	CodeDLTD,
	#[serde(rename = "JOIN")]
	CodeJOIN,

	#[default]
	UNKOWN
}


// MessageHeader7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader7 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
	#[serde(rename = "ReqTp", skip_serializing_if = "Option::is_none")]
	pub req_tp: Option<RequestType4Choice>,
	#[serde(rename = "OrgnlBizQry", skip_serializing_if = "Option::is_none")]
	pub orgnl_biz_qry: Option<OriginalBusinessQuery1>,
	#[serde(rename = "QryNm", skip_serializing_if = "Option::is_none")]
	pub qry_nm: Option<Max35Text>,
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


// PaymentRole1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentRole1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalPaymentRole1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// PhoneNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PhoneNumber {
	#[serde(rename = "$value")]
	pub phone_number: String,
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


// RequestType4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RequestType4Choice {
	#[serde(rename = "PmtCtrl", skip_serializing_if = "Option::is_none")]
	pub pmt_ctrl: Option<ExternalPaymentControlRequestType1Code>,
	#[serde(rename = "Enqry", skip_serializing_if = "Option::is_none")]
	pub enqry: Option<ExternalEnquiryRequestType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification1>,
}


// ReturnMemberV05 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReturnMemberV05 {
	#[serde(rename = "MsgHdr")]
	pub msg_hdr: MessageHeader7,
	#[serde(rename = "RptOrErr")]
	pub rpt_or_err: MemberReportOrError7Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// StructuredLongPostalAddress1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StructuredLongPostalAddress1 {
	#[serde(rename = "BldgNm", skip_serializing_if = "Option::is_none")]
	pub bldg_nm: Option<Max35Text>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<Max35Text>,
	#[serde(rename = "StrtBldgId", skip_serializing_if = "Option::is_none")]
	pub strt_bldg_id: Option<Max35Text>,
	#[serde(rename = "Flr", skip_serializing_if = "Option::is_none")]
	pub flr: Option<Max16Text>,
	#[serde(rename = "TwnNm")]
	pub twn_nm: Max35Text,
	#[serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none")]
	pub dstrct_nm: Option<Max35Text>,
	#[serde(rename = "RgnId", skip_serializing_if = "Option::is_none")]
	pub rgn_id: Option<Max35Text>,
	#[serde(rename = "Stat", skip_serializing_if = "Option::is_none")]
	pub stat: Option<Max35Text>,
	#[serde(rename = "CtyId", skip_serializing_if = "Option::is_none")]
	pub cty_id: Option<Max35Text>,
	#[serde(rename = "Ctry")]
	pub ctry: CountryCode,
	#[serde(rename = "PstCdId")]
	pub pst_cd_id: Max16Text,
	#[serde(rename = "POB", skip_serializing_if = "Option::is_none")]
	pub pob: Option<Max16Text>,
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


// SystemMemberStatus1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemMemberStatus1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<MemberStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// SystemMemberType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemMemberType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalSystemMemberType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}
