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


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "$value")]
	pub active_currency_code: String,
}


// ContactDetails4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContactDetails4 {
	#[serde(rename = "Nm")]
	pub nm: Max140Text,
	#[serde(rename = "PhneNb")]
	pub phne_nb: PhoneNumber,
	#[serde(rename = "EmailAdr")]
	pub email_adr: Max2048Text,
	#[serde(rename = "Fctn")]
	pub fctn: Max140Text,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// Exact2UpperCaseAlphaText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Exact2UpperCaseAlphaText {
	#[serde(rename = "$value")]
	pub exact2_upper_case_alpha_text: String,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}


// InternalisationData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InternalisationData1 {
	#[serde(rename = "Aggt")]
	pub aggt: InternalisationData2,
	#[serde(rename = "FaildRate")]
	pub faild_rate: InternalisationDataRate1,
}


// InternalisationData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InternalisationData2 {
	#[serde(rename = "Sttld")]
	pub sttld: InternalisationDataVolume1,
	#[serde(rename = "Faild")]
	pub faild: InternalisationDataVolume1,
	#[serde(rename = "Ttl")]
	pub ttl: InternalisationDataVolume1,
}


// InternalisationDataRate1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InternalisationDataRate1 {
	#[serde(rename = "VolPctg")]
	pub vol_pctg: f64,
	#[serde(rename = "Val")]
	pub val: f64,
}


// InternalisationDataVolume1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InternalisationDataVolume1 {
	#[serde(rename = "Vol")]
	pub vol: f64,
	#[serde(rename = "Val")]
	pub val: f64,
}


// IssuerCSDIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IssuerCSDIdentification1 {
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "FrstTwoCharsInstrmId")]
	pub frst_two_chars_instrm_id: Exact2UpperCaseAlphaText,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
}


// IssuerCSDReport1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IssuerCSDReport1 {
	#[serde(rename = "Id")]
	pub id: IssuerCSDIdentification1,
	#[serde(rename = "OvrllTtl")]
	pub ovrll_ttl: InternalisationData1,
	#[serde(rename = "FinInstrm")]
	pub fin_instrm: SettlementInternaliserFinancialInstrument1,
	#[serde(rename = "TxTp")]
	pub tx_tp: SettlementInternaliserTransactionType1,
	#[serde(rename = "ClntTp")]
	pub clnt_tp: SettlementInternaliserClientType1,
	#[serde(rename = "TtlCshTrf")]
	pub ttl_csh_trf: InternalisationData1,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
}


// Max2048Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max2048Text {
	#[serde(rename = "$value")]
	pub max2048_text: String,
}


// Max20PositiveDecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max20PositiveDecimalNumber {
	#[serde(rename = "$value")]
	pub max20_positive_decimal_number: f64,
}


// Max20PositiveNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max20PositiveNumber {
	#[serde(rename = "$value")]
	pub max20_positive_number: f64,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PercentageRate {
	#[serde(rename = "$value")]
	pub percentage_rate: f64,
}


// PhoneNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PhoneNumber {
	#[serde(rename = "$value")]
	pub phone_number: String,
}


// SettlementInternaliser1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementInternaliser1 {
	#[serde(rename = "Id")]
	pub id: SettlementInternaliserIdentification1,
	#[serde(rename = "OvrllTtl")]
	pub ovrll_ttl: InternalisationData1,
	#[serde(rename = "FinInstrm")]
	pub fin_instrm: SettlementInternaliserFinancialInstrument1,
	#[serde(rename = "TxTp")]
	pub tx_tp: SettlementInternaliserTransactionType1,
	#[serde(rename = "ClntTp")]
	pub clnt_tp: SettlementInternaliserClientType1,
	#[serde(rename = "TtlCshTrf")]
	pub ttl_csh_trf: InternalisationData1,
}


// SettlementInternaliserClientType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementInternaliserClientType1 {
	#[serde(rename = "Prfssnl")]
	pub prfssnl: InternalisationData1,
	#[serde(rename = "Rtl")]
	pub rtl: InternalisationData1,
}


// SettlementInternaliserFinancialInstrument1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementInternaliserFinancialInstrument1 {
	#[serde(rename = "Eqty")]
	pub eqty: InternalisationData1,
	#[serde(rename = "SvrgnDebt")]
	pub svrgn_debt: InternalisationData1,
	#[serde(rename = "Bd")]
	pub bd: InternalisationData1,
	#[serde(rename = "OthrTrfblScties")]
	pub othr_trfbl_scties: InternalisationData1,
	#[serde(rename = "XchgTradgFnds")]
	pub xchg_tradg_fnds: InternalisationData1,
	#[serde(rename = "CllctvInvstmtUdrtkgs")]
	pub cllctv_invstmt_udrtkgs: InternalisationData1,
	#[serde(rename = "MnyMktInstrm")]
	pub mny_mkt_instrm: InternalisationData1,
	#[serde(rename = "EmssnAllwnc")]
	pub emssn_allwnc: InternalisationData1,
	#[serde(rename = "OthrFinInstrms")]
	pub othr_fin_instrms: InternalisationData1,
}


// SettlementInternaliserIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementInternaliserIdentification1 {
	#[serde(rename = "LEI")]
	pub lei: LEIIdentifier,
	#[serde(rename = "RspnsblPrsn")]
	pub rspnsbl_prsn: ContactDetails4,
	#[serde(rename = "Ctry")]
	pub ctry: CountryCode,
	#[serde(rename = "BrnchId", skip_serializing_if = "Option::is_none")]
	pub brnch_id: Option<Exact2UpperCaseAlphaText>,
}


// SettlementInternaliserReportHeader1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementInternaliserReportHeader1 {
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
	#[serde(rename = "RptgDt")]
	pub rptg_dt: String,
	#[serde(rename = "Ccy")]
	pub ccy: ActiveCurrencyCode,
	#[serde(rename = "RptSts")]
	pub rpt_sts: TransactionOperationType4Code,
}


// SettlementInternaliserReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementInternaliserReportV01 {
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: SettlementInternaliserReportHeader1,
	#[serde(rename = "SttlmIntlr")]
	pub sttlm_intlr: SettlementInternaliser1,
	#[serde(rename = "IssrCSD")]
	pub issr_csd: Vec<IssuerCSDReport1>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// SettlementInternaliserTransactionType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementInternaliserTransactionType1 {
	#[serde(rename = "SctiesBuyOrSell")]
	pub scties_buy_or_sell: InternalisationData1,
	#[serde(rename = "CollMgmtOpr")]
	pub coll_mgmt_opr: InternalisationData1,
	#[serde(rename = "SctiesLndgOrBrrwg")]
	pub scties_lndg_or_brrwg: InternalisationData1,
	#[serde(rename = "RpAgrmt")]
	pub rp_agrmt: InternalisationData1,
	#[serde(rename = "OthrTxs")]
	pub othr_txs: InternalisationData1,
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


// TransactionOperationType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TransactionOperationType4Code {
	#[default]
	#[serde(rename = "NEWT")]
	CodeNEWT,
	#[serde(rename = "AMND")]
	CodeAMND,
	#[serde(rename = "CANC")]
	CodeCANC,
}
