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
use regex::Regex;


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "$value")]
	pub active_currency_code: String,
}

impl ActiveCurrencyCode {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.active_currency_code) {
			return false
		}
		return true
	}
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

impl ContactDetails4 {
	pub fn validate(&self) -> bool {
		if !self.nm.validate() { return false }
		if !self.phne_nb.validate() { return false }
		if !self.email_adr.validate() { return false }
		if !self.fctn.validate() { return false }
		return true
	}
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}

impl CountryCode {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.country_code) {
			return false
		}
		return true
	}
}


// Exact2UpperCaseAlphaText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Exact2UpperCaseAlphaText {
	#[serde(rename = "$value")]
	pub exact2_upper_case_alpha_text: String,
}

impl Exact2UpperCaseAlphaText {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{2}").unwrap();
		if !pattern.is_match(&self.exact2_upper_case_alpha_text) {
			return false
		}
		return true
	}
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}

impl ISODate {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}

impl ISODateTime {
	pub fn validate(&self) -> bool {
		return true
	}
}


// InternalisationData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InternalisationData1 {
	#[serde(rename = "Aggt")]
	pub aggt: InternalisationData2,
	#[serde(rename = "FaildRate")]
	pub faild_rate: InternalisationDataRate1,
}

impl InternalisationData1 {
	pub fn validate(&self) -> bool {
		if !self.aggt.validate() { return false }
		if !self.faild_rate.validate() { return false }
		return true
	}
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

impl InternalisationData2 {
	pub fn validate(&self) -> bool {
		if !self.sttld.validate() { return false }
		if !self.faild.validate() { return false }
		if !self.ttl.validate() { return false }
		return true
	}
}


// InternalisationDataRate1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InternalisationDataRate1 {
	#[serde(rename = "VolPctg")]
	pub vol_pctg: f64,
	#[serde(rename = "Val")]
	pub val: f64,
}

impl InternalisationDataRate1 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// InternalisationDataVolume1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InternalisationDataVolume1 {
	#[serde(rename = "Vol")]
	pub vol: f64,
	#[serde(rename = "Val")]
	pub val: f64,
}

impl InternalisationDataVolume1 {
	pub fn validate(&self) -> bool {
		return true
	}
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

impl IssuerCSDIdentification1 {
	pub fn validate(&self) -> bool {
		if let Some(ref lei_value) = self.lei { if !lei_value.validate() { return false; } }
		if !self.frst_two_chars_instrm_id.validate() { return false }
		if let Some(ref ctry_value) = self.ctry { if !ctry_value.validate() { return false; } }
		return true
	}
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

impl IssuerCSDReport1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.ovrll_ttl.validate() { return false }
		if !self.fin_instrm.validate() { return false }
		if !self.tx_tp.validate() { return false }
		if !self.clnt_tp.validate() { return false }
		if !self.ttl_csh_trf.validate() { return false }
		return true
	}
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}

impl LEIIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.lei_identifier) {
			return false
		}
		return true
	}
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
}

impl Max140Text {
	pub fn validate(&self) -> bool {
		if self.max140_text.chars().count() < 1 {
			return false
		}
		if self.max140_text.chars().count() > 140 {
			return false
		}
		return true
	}
}


// Max2048Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max2048Text {
	#[serde(rename = "$value")]
	pub max2048_text: String,
}

impl Max2048Text {
	pub fn validate(&self) -> bool {
		if self.max2048_text.chars().count() < 1 {
			return false
		}
		if self.max2048_text.chars().count() > 2048 {
			return false
		}
		return true
	}
}


// Max20PositiveDecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max20PositiveDecimalNumber {
	#[serde(rename = "$value")]
	pub max20_positive_decimal_number: f64,
}

impl Max20PositiveDecimalNumber {
	pub fn validate(&self) -> bool {
		if self.max20_positive_decimal_number < 0.000000 {
			return false
		}
		return true
	}
}


// Max20PositiveNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max20PositiveNumber {
	#[serde(rename = "$value")]
	pub max20_positive_number: f64,
}

impl Max20PositiveNumber {
	pub fn validate(&self) -> bool {
		if self.max20_positive_number < 0.000000 {
			return false
		}
		return true
	}
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}

impl Max350Text {
	pub fn validate(&self) -> bool {
		if self.max350_text.chars().count() < 1 {
			return false
		}
		if self.max350_text.chars().count() > 350 {
			return false
		}
		return true
	}
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PercentageRate {
	#[serde(rename = "$value")]
	pub percentage_rate: f64,
}

impl PercentageRate {
	pub fn validate(&self) -> bool {
		return true
	}
}


// PhoneNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PhoneNumber {
	#[serde(rename = "$value")]
	pub phone_number: String,
}

impl PhoneNumber {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
		if !pattern.is_match(&self.phone_number) {
			return false
		}
		return true
	}
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

impl SettlementInternaliser1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.ovrll_ttl.validate() { return false }
		if !self.fin_instrm.validate() { return false }
		if !self.tx_tp.validate() { return false }
		if !self.clnt_tp.validate() { return false }
		if !self.ttl_csh_trf.validate() { return false }
		return true
	}
}


// SettlementInternaliserClientType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementInternaliserClientType1 {
	#[serde(rename = "Prfssnl")]
	pub prfssnl: InternalisationData1,
	#[serde(rename = "Rtl")]
	pub rtl: InternalisationData1,
}

impl SettlementInternaliserClientType1 {
	pub fn validate(&self) -> bool {
		if !self.prfssnl.validate() { return false }
		if !self.rtl.validate() { return false }
		return true
	}
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

impl SettlementInternaliserFinancialInstrument1 {
	pub fn validate(&self) -> bool {
		if !self.eqty.validate() { return false }
		if !self.svrgn_debt.validate() { return false }
		if !self.bd.validate() { return false }
		if !self.othr_trfbl_scties.validate() { return false }
		if !self.xchg_tradg_fnds.validate() { return false }
		if !self.cllctv_invstmt_udrtkgs.validate() { return false }
		if !self.mny_mkt_instrm.validate() { return false }
		if !self.emssn_allwnc.validate() { return false }
		if !self.othr_fin_instrms.validate() { return false }
		return true
	}
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

impl SettlementInternaliserIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.lei.validate() { return false }
		if !self.rspnsbl_prsn.validate() { return false }
		if !self.ctry.validate() { return false }
		if let Some(ref brnch_id_value) = self.brnch_id { if !brnch_id_value.validate() { return false; } }
		return true
	}
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

impl SettlementInternaliserReportHeader1 {
	pub fn validate(&self) -> bool {
		if !self.ccy.validate() { return false }
		if !self.rpt_sts.validate() { return false }
		return true
	}
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

impl SettlementInternaliserReportV01 {
	pub fn validate(&self) -> bool {
		if !self.rpt_hdr.validate() { return false }
		if !self.sttlm_intlr.validate() { return false }
		for item in &self.issr_csd { if !item.validate() { return false; } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
	}
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

impl SettlementInternaliserTransactionType1 {
	pub fn validate(&self) -> bool {
		if !self.scties_buy_or_sell.validate() { return false }
		if !self.coll_mgmt_opr.validate() { return false }
		if !self.scties_lndg_or_brrwg.validate() { return false }
		if !self.rp_agrmt.validate() { return false }
		if !self.othr_txs.validate() { return false }
		return true
	}
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
	pub plc_and_nm: Option<Max350Text>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}

impl SupplementaryData1 {
	pub fn validate(&self) -> bool {
		if let Some(ref plc_and_nm_value) = self.plc_and_nm { if !plc_and_nm_value.validate() { return false; } }
		if !self.envlp.validate() { return false }
		return true
	}
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> bool {
		return true
	}
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

impl TransactionOperationType4Code {
	pub fn validate(&self) -> bool {
		return true
	}
}
