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


// Amount2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Amount2Choice {
	#[serde(rename = "AmtWthtCcy", skip_serializing_if = "Option::is_none")]
	pub amt_wtht_ccy: Option<f64>,
	#[serde(rename = "AmtWthCcy", skip_serializing_if = "Option::is_none")]
	pub amt_wth_ccy: Option<ActiveCurrencyAndAmount>,
}


// BICFIDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BICFIDec2014Identifier {
	#[serde(rename = "$value")]
	pub bicfi_dec2014_identifier: String,
}


// BackupPaymentV07 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BackupPaymentV07 {
	#[serde(rename = "MsgHdr")]
	pub msg_hdr: MessageHeader1,
	#[serde(rename = "OrgnlMsgId", skip_serializing_if = "Option::is_none")]
	pub orgnl_msg_id: Option<MessageHeader1>,
	#[serde(rename = "InstrInf", skip_serializing_if = "Option::is_none")]
	pub instr_inf: Option<PaymentInstruction13>,
	#[serde(rename = "TrfdAmt")]
	pub trfd_amt: Amount2Choice,
	#[serde(rename = "Cdtr")]
	pub cdtr: SystemMember3,
	#[serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none")]
	pub cdtr_agt: Option<SystemMember3>,
	#[serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none")]
	pub dbtr_agt: Option<SystemMember3>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
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


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// ExternalClearingSystemIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalClearingSystemIdentification1Code {
	#[serde(rename = "$value")]
	pub external_clearing_system_identification1_code: String,
}


// ExternalFinancialInstitutionIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalFinancialInstitutionIdentification1Code {
	#[serde(rename = "$value")]
	pub external_financial_institution_identification1_code: String,
}


// ExternalMarketInfrastructure1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalMarketInfrastructure1Code {
	#[serde(rename = "$value")]
	pub external_market_infrastructure1_code: String,
}


// FinancialIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalFinancialInstitutionIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
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


// MarketInfrastructureIdentification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarketInfrastructureIdentification1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalMarketInfrastructure1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
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


// MessageHeader1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader1 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
}


// PaymentInstruction13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentInstruction13 {
	#[serde(rename = "ReqdExctnDtTm", skip_serializing_if = "Option::is_none")]
	pub reqd_exctn_dt_tm: Option<String>,
	#[serde(rename = "PmtTp", skip_serializing_if = "Option::is_none")]
	pub pmt_tp: Option<PaymentType4Choice>,
}


// PaymentType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PaymentType3Code {
	#[serde(rename = "CBS")]
	CodeCBS,
	#[serde(rename = "BCK")]
	CodeBCK,
	#[serde(rename = "BAL")]
	CodeBAL,
	#[serde(rename = "CLS")]
	CodeCLS,
	#[serde(rename = "CTR")]
	CodeCTR,
	#[serde(rename = "CBH")]
	CodeCBH,
	#[serde(rename = "CBP")]
	CodeCBP,
	#[serde(rename = "DPG")]
	CodeDPG,
	#[serde(rename = "DPN")]
	CodeDPN,
	#[serde(rename = "EXP")]
	CodeEXP,
	#[serde(rename = "TCH")]
	CodeTCH,
	#[serde(rename = "LMT")]
	CodeLMT,
	#[serde(rename = "LIQ")]
	CodeLIQ,
	#[serde(rename = "DPP")]
	CodeDPP,
	#[serde(rename = "DPH")]
	CodeDPH,
	#[serde(rename = "DPS")]
	CodeDPS,
	#[serde(rename = "STF")]
	CodeSTF,
	#[serde(rename = "TRP")]
	CodeTRP,
	#[serde(rename = "TCS")]
	CodeTCS,
	#[serde(rename = "LOA")]
	CodeLOA,
	#[serde(rename = "LOR")]
	CodeLOR,
	#[serde(rename = "TCP")]
	CodeTCP,
	#[serde(rename = "OND")]
	CodeOND,
	#[serde(rename = "MGL")]
	CodeMGL,

	#[default]
	UNKOWN
}


// PaymentType4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentType4Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PaymentType3Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
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


// SystemIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemIdentification2Choice {
	#[serde(rename = "MktInfrstrctrId", skip_serializing_if = "Option::is_none")]
	pub mkt_infrstrctr_id: Option<MarketInfrastructureIdentification1Choice>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
}


// SystemMember3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SystemMember3 {
	#[serde(rename = "SysId", skip_serializing_if = "Option::is_none")]
	pub sys_id: Option<SystemIdentification2Choice>,
	#[serde(rename = "MmbId")]
	pub mmb_id: MemberIdentification3Choice,
}
