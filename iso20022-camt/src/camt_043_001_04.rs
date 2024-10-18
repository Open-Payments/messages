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


// AccountIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountIdentification1 {
	#[serde(rename = "Prtry")]
	pub prtry: SimpleIdentificationInformation,
}

impl AccountIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.prtry.validate() { return false }
		return true
	}
}


// ActiveCurrencyAnd13DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyAnd13DecimalAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_currency_and13_decimal_amount_simple_type: f64,
}

impl ActiveCurrencyAnd13DecimalAmountSimpleType {
	pub fn validate(&self) -> bool {
		if self.active_currency_and13_decimal_amount_simple_type < 0.000000 {
			return false
		}
		return true
	}
}


// ActiveCurrencyAnd13DecimalAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd13DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveCurrencyAnd13DecimalAmount {
	pub fn validate(&self) -> bool {
		return true
	}
}


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


// ActiveOrHistoricCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_and_amount_simple_type: f64,
}

impl ActiveOrHistoricCurrencyAndAmountSimpleType {
	pub fn validate(&self) -> bool {
		if self.active_or_historic_currency_and_amount_simple_type < 0.000000 {
			return false
		}
		return true
	}
}


// ActiveOrHistoricCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveOrHistoricCurrencyAndAmount {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_code: String,
}

impl ActiveOrHistoricCurrencyCode {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.active_or_historic_currency_code) {
			return false
		}
		return true
	}
}


// AdditionalParameters1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdditionalParameters1 {
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "GeoArea", skip_serializing_if = "Option::is_none")]
	pub geo_area: Option<Max35Text>,
}

impl AdditionalParameters1 {
	pub fn validate(&self) -> bool {
		if let Some(ref ctry_value) = self.ctry { if !ctry_value.validate() { return false; } }
		if let Some(ref ccy_value) = self.ccy { if !ccy_value.validate() { return false; } }
		if let Some(ref geo_area_value) = self.geo_area { if !geo_area_value.validate() { return false; } }
		return true
	}
}


// AdditionalReference3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdditionalReference3 {
	#[serde(rename = "Ref")]
	pub ref_attr: Max35Text,
	#[serde(rename = "RefIssr", skip_serializing_if = "Option::is_none")]
	pub ref_issr: Option<PartyIdentification2Choice>,
	#[serde(rename = "MsgNm", skip_serializing_if = "Option::is_none")]
	pub msg_nm: Option<Max35Text>,
}

impl AdditionalReference3 {
	pub fn validate(&self) -> bool {
		if !self.ref_attr.validate() { return false }
		if let Some(ref ref_issr_value) = self.ref_issr { if !ref_issr_value.validate() { return false; } }
		if let Some(ref msg_nm_value) = self.msg_nm { if !msg_nm_value.validate() { return false; } }
		return true
	}
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

impl AddressType2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AlternateSecurityIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AlternateSecurityIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "DmstIdSrc", skip_serializing_if = "Option::is_none")]
	pub dmst_id_src: Option<CountryCode>,
	#[serde(rename = "PrtryIdSrc", skip_serializing_if = "Option::is_none")]
	pub prtry_id_src: Option<Max35Text>,
}

impl AlternateSecurityIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref dmst_id_src_value) = self.dmst_id_src { if !dmst_id_src_value.validate() { return false; } }
		if let Some(ref prtry_id_src_value) = self.prtry_id_src { if !prtry_id_src_value.validate() { return false; } }
		return true
	}
}


// AmountOrRate3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountOrRate3Choice {
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAnd13DecimalAmount>,
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
}

impl AmountOrRate3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref amt_value) = self.amt { if !amt_value.validate() { return false; } }
		return true
	}
}


// AnyBICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnyBICIdentifier {
	#[serde(rename = "$value")]
	pub any_bic_identifier: String,
}

impl AnyBICIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}").unwrap();
		if !pattern.is_match(&self.any_bic_identifier) {
			return false
		}
		return true
	}
}


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BaseOneRate {
	#[serde(rename = "$value")]
	pub base_one_rate: f64,
}

impl BaseOneRate {
	pub fn validate(&self) -> bool {
		return true
	}
}


// BelgianIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BelgianIdentifier {
	#[serde(rename = "$value")]
	pub belgian_identifier: String,
}

impl BelgianIdentifier {
	pub fn validate(&self) -> bool {
		return true
	}
}


// BloombergIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BloombergIdentifier {
	#[serde(rename = "$value")]
	pub bloomberg_identifier: String,
}

impl BloombergIdentifier {
	pub fn validate(&self) -> bool {
		if self.bloomberg_identifier.chars().count() < 1 {
			return false
		}
		if self.bloomberg_identifier.chars().count() > 35 {
			return false
		}
		return true
	}
}


// BreakdownByCountry2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BreakdownByCountry2 {
	#[serde(rename = "Ctry")]
	pub ctry: CountryCode,
	#[serde(rename = "CshInFcst", skip_serializing_if = "Option::is_none")]
	pub csh_in_fcst: Option<Vec<CashInForecast5>>,
	#[serde(rename = "CshOutFcst", skip_serializing_if = "Option::is_none")]
	pub csh_out_fcst: Option<Vec<CashOutForecast5>>,
	#[serde(rename = "NetCshFcst", skip_serializing_if = "Option::is_none")]
	pub net_csh_fcst: Option<Vec<NetCashForecast4>>,
}

impl BreakdownByCountry2 {
	pub fn validate(&self) -> bool {
		if !self.ctry.validate() { return false }
		if let Some(ref csh_in_fcst_vec) = self.csh_in_fcst { for item in csh_in_fcst_vec { if !item.validate() { return false; } } }
		if let Some(ref csh_out_fcst_vec) = self.csh_out_fcst { for item in csh_out_fcst_vec { if !item.validate() { return false; } } }
		if let Some(ref net_csh_fcst_vec) = self.net_csh_fcst { for item in net_csh_fcst_vec { if !item.validate() { return false; } } }
		return true
	}
}


// BreakdownByCurrency2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BreakdownByCurrency2 {
	#[serde(rename = "Ccy")]
	pub ccy: ActiveOrHistoricCurrencyCode,
	#[serde(rename = "CshOutFcst", skip_serializing_if = "Option::is_none")]
	pub csh_out_fcst: Option<Vec<CashOutForecast5>>,
	#[serde(rename = "CshInFcst", skip_serializing_if = "Option::is_none")]
	pub csh_in_fcst: Option<Vec<CashInForecast5>>,
	#[serde(rename = "NetCshFcst", skip_serializing_if = "Option::is_none")]
	pub net_csh_fcst: Option<Vec<NetCashForecast4>>,
}

impl BreakdownByCurrency2 {
	pub fn validate(&self) -> bool {
		if !self.ccy.validate() { return false }
		if let Some(ref csh_out_fcst_vec) = self.csh_out_fcst { for item in csh_out_fcst_vec { if !item.validate() { return false; } } }
		if let Some(ref csh_in_fcst_vec) = self.csh_in_fcst { for item in csh_in_fcst_vec { if !item.validate() { return false; } } }
		if let Some(ref net_csh_fcst_vec) = self.net_csh_fcst { for item in net_csh_fcst_vec { if !item.validate() { return false; } } }
		return true
	}
}


// BreakdownByParty3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BreakdownByParty3 {
	#[serde(rename = "Pty")]
	pub pty: InvestmentAccount42,
	#[serde(rename = "AddtlParams", skip_serializing_if = "Option::is_none")]
	pub addtl_params: Option<AdditionalParameters1>,
	#[serde(rename = "CshInFcst", skip_serializing_if = "Option::is_none")]
	pub csh_in_fcst: Option<Vec<CashInForecast5>>,
	#[serde(rename = "CshOutFcst", skip_serializing_if = "Option::is_none")]
	pub csh_out_fcst: Option<Vec<CashOutForecast5>>,
	#[serde(rename = "NetCshFcst", skip_serializing_if = "Option::is_none")]
	pub net_csh_fcst: Option<Vec<NetCashForecast4>>,
}

impl BreakdownByParty3 {
	pub fn validate(&self) -> bool {
		if !self.pty.validate() { return false }
		if let Some(ref addtl_params_value) = self.addtl_params { if !addtl_params_value.validate() { return false; } }
		if let Some(ref csh_in_fcst_vec) = self.csh_in_fcst { for item in csh_in_fcst_vec { if !item.validate() { return false; } } }
		if let Some(ref csh_out_fcst_vec) = self.csh_out_fcst { for item in csh_out_fcst_vec { if !item.validate() { return false; } } }
		if let Some(ref net_csh_fcst_vec) = self.net_csh_fcst { for item in net_csh_fcst_vec { if !item.validate() { return false; } } }
		return true
	}
}


// BreakdownByUserDefinedParameter3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BreakdownByUserDefinedParameter3 {
	#[serde(rename = "Pty", skip_serializing_if = "Option::is_none")]
	pub pty: Option<InvestmentAccount42>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "UsrDfnd", skip_serializing_if = "Option::is_none")]
	pub usr_dfnd: Option<DataFormat2Choice>,
	#[serde(rename = "CshInFcst", skip_serializing_if = "Option::is_none")]
	pub csh_in_fcst: Option<Vec<CashInForecast5>>,
	#[serde(rename = "CshOutFcst", skip_serializing_if = "Option::is_none")]
	pub csh_out_fcst: Option<Vec<CashOutForecast5>>,
	#[serde(rename = "NetCshFcst", skip_serializing_if = "Option::is_none")]
	pub net_csh_fcst: Option<Vec<NetCashForecast4>>,
}

impl BreakdownByUserDefinedParameter3 {
	pub fn validate(&self) -> bool {
		if let Some(ref pty_value) = self.pty { if !pty_value.validate() { return false; } }
		if let Some(ref ctry_value) = self.ctry { if !ctry_value.validate() { return false; } }
		if let Some(ref ccy_value) = self.ccy { if !ccy_value.validate() { return false; } }
		if let Some(ref usr_dfnd_value) = self.usr_dfnd { if !usr_dfnd_value.validate() { return false; } }
		if let Some(ref csh_in_fcst_vec) = self.csh_in_fcst { for item in csh_in_fcst_vec { if !item.validate() { return false; } } }
		if let Some(ref csh_out_fcst_vec) = self.csh_out_fcst { for item in csh_out_fcst_vec { if !item.validate() { return false; } } }
		if let Some(ref net_csh_fcst_vec) = self.net_csh_fcst { for item in net_csh_fcst_vec { if !item.validate() { return false; } } }
		return true
	}
}


// CUSIPIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CUSIPIdentifier {
	#[serde(rename = "$value")]
	pub cusip_identifier: String,
}

impl CUSIPIdentifier {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CashInForecast5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashInForecast5 {
	#[serde(rename = "CshSttlmDt")]
	pub csh_sttlm_dt: String,
	#[serde(rename = "SubTtlAmt", skip_serializing_if = "Option::is_none")]
	pub sub_ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "SubTtlUnitsNb", skip_serializing_if = "Option::is_none")]
	pub sub_ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "XcptnlCshFlowInd", skip_serializing_if = "Option::is_none")]
	pub xcptnl_csh_flow_ind: Option<bool>,
	#[serde(rename = "CshInBrkdwnDtls", skip_serializing_if = "Option::is_none")]
	pub csh_in_brkdwn_dtls: Option<Vec<FundCashInBreakdown3>>,
	#[serde(rename = "AddtlBal", skip_serializing_if = "Option::is_none")]
	pub addtl_bal: Option<FundBalance1>,
}

impl CashInForecast5 {
	pub fn validate(&self) -> bool {
		if let Some(ref sub_ttl_amt_value) = self.sub_ttl_amt { if !sub_ttl_amt_value.validate() { return false; } }
		if let Some(ref sub_ttl_units_nb_value) = self.sub_ttl_units_nb { if !sub_ttl_units_nb_value.validate() { return false; } }
		if let Some(ref csh_in_brkdwn_dtls_vec) = self.csh_in_brkdwn_dtls { for item in csh_in_brkdwn_dtls_vec { if !item.validate() { return false; } } }
		if let Some(ref addtl_bal_value) = self.addtl_bal { if !addtl_bal_value.validate() { return false; } }
		return true
	}
}


// CashOutForecast5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashOutForecast5 {
	#[serde(rename = "CshSttlmDt")]
	pub csh_sttlm_dt: String,
	#[serde(rename = "SubTtlAmt", skip_serializing_if = "Option::is_none")]
	pub sub_ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "SubTtlUnitsNb", skip_serializing_if = "Option::is_none")]
	pub sub_ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "XcptnlCshFlowInd", skip_serializing_if = "Option::is_none")]
	pub xcptnl_csh_flow_ind: Option<bool>,
	#[serde(rename = "CshOutBrkdwnDtls", skip_serializing_if = "Option::is_none")]
	pub csh_out_brkdwn_dtls: Option<Vec<FundCashOutBreakdown3>>,
	#[serde(rename = "AddtlBal", skip_serializing_if = "Option::is_none")]
	pub addtl_bal: Option<FundBalance1>,
}

impl CashOutForecast5 {
	pub fn validate(&self) -> bool {
		if let Some(ref sub_ttl_amt_value) = self.sub_ttl_amt { if !sub_ttl_amt_value.validate() { return false; } }
		if let Some(ref sub_ttl_units_nb_value) = self.sub_ttl_units_nb { if !sub_ttl_units_nb_value.validate() { return false; } }
		if let Some(ref csh_out_brkdwn_dtls_vec) = self.csh_out_brkdwn_dtls { for item in csh_out_brkdwn_dtls_vec { if !item.validate() { return false; } } }
		if let Some(ref addtl_bal_value) = self.addtl_bal { if !addtl_bal_value.validate() { return false; } }
		return true
	}
}


// Charge26 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Charge26 {
	#[serde(rename = "Tp")]
	pub tp: ChargeType4Choice,
	#[serde(rename = "ChrgApld")]
	pub chrg_apld: AmountOrRate3Choice,
}

impl Charge26 {
	pub fn validate(&self) -> bool {
		if !self.tp.validate() { return false }
		if !self.chrg_apld.validate() { return false }
		return true
	}
}


// ChargeType12Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ChargeType12Code {
	#[default]
	#[serde(rename = "BEND")]
	CodeBEND,
	#[serde(rename = "DISC")]
	CodeDISC,
	#[serde(rename = "FEND")]
	CodeFEND,
	#[serde(rename = "POST")]
	CodePOST,
	#[serde(rename = "REGF")]
	CodeREGF,
	#[serde(rename = "SHIP")]
	CodeSHIP,
	#[serde(rename = "SPCN")]
	CodeSPCN,
	#[serde(rename = "TRAN")]
	CodeTRAN,
}

impl ChargeType12Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ChargeType4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ChargeType4Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ChargeType12Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl ChargeType4Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// Commission21 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Commission21 {
	#[serde(rename = "ComssnTp")]
	pub comssn_tp: CommissionType5Choice,
	#[serde(rename = "ComssnApld")]
	pub comssn_apld: AmountOrRate3Choice,
}

impl Commission21 {
	pub fn validate(&self) -> bool {
		if !self.comssn_tp.validate() { return false }
		if !self.comssn_apld.validate() { return false }
		return true
	}
}


// CommissionType5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommissionType5Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<CommissionType6Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl CommissionType5Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// CommissionType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CommissionType6Code {
	#[default]
	#[serde(rename = "FEND")]
	CodeFEND,
	#[serde(rename = "BEND")]
	CodeBEND,
	#[serde(rename = "CDPL")]
	CodeCDPL,
}

impl CommissionType6Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ConsolidatedTapeAssociationIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ConsolidatedTapeAssociationIdentifier {
	#[serde(rename = "$value")]
	pub consolidated_tape_association_identifier: String,
}

impl ConsolidatedTapeAssociationIdentifier {
	pub fn validate(&self) -> bool {
		if self.consolidated_tape_association_identifier.chars().count() < 1 {
			return false
		}
		if self.consolidated_tape_association_identifier.chars().count() > 35 {
			return false
		}
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


// CurrencyDesignation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CurrencyDesignation1 {
	#[serde(rename = "CcyDsgnt", skip_serializing_if = "Option::is_none")]
	pub ccy_dsgnt: Option<CurrencyDesignation1Code>,
	#[serde(rename = "Lctn", skip_serializing_if = "Option::is_none")]
	pub lctn: Option<CountryCode>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Max350Text>,
}

impl CurrencyDesignation1 {
	pub fn validate(&self) -> bool {
		if let Some(ref ccy_dsgnt_value) = self.ccy_dsgnt { if !ccy_dsgnt_value.validate() { return false; } }
		if let Some(ref lctn_value) = self.lctn { if !lctn_value.validate() { return false; } }
		if let Some(ref addtl_inf_value) = self.addtl_inf { if !addtl_inf_value.validate() { return false; } }
		return true
	}
}


// CurrencyDesignation1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CurrencyDesignation1Code {
	#[default]
	#[serde(rename = "ONSH")]
	CodeONSH,
	#[serde(rename = "OFFS")]
	CodeOFFS,
}

impl CurrencyDesignation1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// DataFormat2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DataFormat2Choice {
	#[serde(rename = "Strd", skip_serializing_if = "Option::is_none")]
	pub strd: Option<GenericIdentification1>,
	#[serde(rename = "Ustrd", skip_serializing_if = "Option::is_none")]
	pub ustrd: Option<Max140Text>,
}

impl DataFormat2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref strd_value) = self.strd { if !strd_value.validate() { return false; } }
		if let Some(ref ustrd_value) = self.ustrd { if !ustrd_value.validate() { return false; } }
		return true
	}
}


// DateAndDateTimeChoice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTimeChoice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<String>,
}

impl DateAndDateTimeChoice {
	pub fn validate(&self) -> bool {
		return true
	}
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DecimalNumber {
	#[serde(rename = "$value")]
	pub decimal_number: f64,
}

impl DecimalNumber {
	pub fn validate(&self) -> bool {
		return true
	}
}


// DistributionPolicy1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum DistributionPolicy1Code {
	#[default]
	#[serde(rename = "DIST")]
	CodeDIST,
	#[serde(rename = "ACCU")]
	CodeACCU,
}

impl DistributionPolicy1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// DutchIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DutchIdentifier {
	#[serde(rename = "$value")]
	pub dutch_identifier: String,
}

impl DutchIdentifier {
	pub fn validate(&self) -> bool {
		return true
	}
}


// EuroclearClearstreamIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EuroclearClearstreamIdentifier {
	#[serde(rename = "$value")]
	pub euroclear_clearstream_identifier: String,
}

impl EuroclearClearstreamIdentifier {
	pub fn validate(&self) -> bool {
		if self.euroclear_clearstream_identifier.chars().count() < 1 {
			return false
		}
		if self.euroclear_clearstream_identifier.chars().count() > 12 {
			return false
		}
		return true
	}
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "$value")]
	pub exact4_alpha_numeric_text: String,
}

impl Exact4AlphaNumericText {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
		if !pattern.is_match(&self.exact4_alpha_numeric_text) {
			return false
		}
		return true
	}
}


// Extension1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Extension1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Max350Text,
	#[serde(rename = "Txt")]
	pub txt: Max350Text,
}

impl Extension1 {
	pub fn validate(&self) -> bool {
		if !self.plc_and_nm.validate() { return false }
		if !self.txt.validate() { return false }
		return true
	}
}


// FinancialInstrument9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument9 {
	#[serde(rename = "Id")]
	pub id: SecurityIdentification3Choice,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max350Text>,
	#[serde(rename = "SplmtryId", skip_serializing_if = "Option::is_none")]
	pub splmtry_id: Option<Max35Text>,
	#[serde(rename = "ReqdNAVCcy", skip_serializing_if = "Option::is_none")]
	pub reqd_nav_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "ClssTp", skip_serializing_if = "Option::is_none")]
	pub clss_tp: Option<Max35Text>,
	#[serde(rename = "SctiesForm", skip_serializing_if = "Option::is_none")]
	pub scties_form: Option<FormOfSecurity1Code>,
	#[serde(rename = "DstrbtnPlcy", skip_serializing_if = "Option::is_none")]
	pub dstrbtn_plcy: Option<DistributionPolicy1Code>,
	#[serde(rename = "DualFndInd")]
	pub dual_fnd_ind: bool,
}

impl FinancialInstrument9 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref splmtry_id_value) = self.splmtry_id { if !splmtry_id_value.validate() { return false; } }
		if let Some(ref reqd_nav_ccy_value) = self.reqd_nav_ccy { if !reqd_nav_ccy_value.validate() { return false; } }
		if let Some(ref clss_tp_value) = self.clss_tp { if !clss_tp_value.validate() { return false; } }
		if let Some(ref scties_form_value) = self.scties_form { if !scties_form_value.validate() { return false; } }
		if let Some(ref dstrbtn_plcy_value) = self.dstrbtn_plcy { if !dstrbtn_plcy_value.validate() { return false; } }
		return true
	}
}


// FinancialInstrumentQuantity1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentQuantity1 {
	#[serde(rename = "Unit")]
	pub unit: f64,
}

impl FinancialInstrumentQuantity1 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// FlowDirectionType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FlowDirectionType1Code {
	#[default]
	#[serde(rename = "INCG")]
	CodeINCG,
	#[serde(rename = "OUTG")]
	CodeOUTG,
}

impl FlowDirectionType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ForeignExchangeTerms19 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ForeignExchangeTerms19 {
	#[serde(rename = "UnitCcy")]
	pub unit_ccy: ActiveCurrencyCode,
	#[serde(rename = "QtdCcy")]
	pub qtd_ccy: ActiveCurrencyCode,
	#[serde(rename = "XchgRate")]
	pub xchg_rate: f64,
}

impl ForeignExchangeTerms19 {
	pub fn validate(&self) -> bool {
		if !self.unit_ccy.validate() { return false }
		if !self.qtd_ccy.validate() { return false }
		return true
	}
}


// FormOfSecurity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FormOfSecurity1Code {
	#[default]
	#[serde(rename = "BEAR")]
	CodeBEAR,
	#[serde(rename = "REGD")]
	CodeREGD,
}

impl FormOfSecurity1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Fund4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Fund4 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max350Text>,
	#[serde(rename = "LglNttyIdr", skip_serializing_if = "Option::is_none")]
	pub lgl_ntty_idr: Option<LEIIdentifier>,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<OtherIdentification4>,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "TtlNAV", skip_serializing_if = "Option::is_none")]
	pub ttl_nav: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "PrvsTtlNAV", skip_serializing_if = "Option::is_none")]
	pub prvs_ttl_nav: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TtlUnitsNb", skip_serializing_if = "Option::is_none")]
	pub ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "PrvsTtlUnitsNb", skip_serializing_if = "Option::is_none")]
	pub prvs_ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "PctgOfFndTtlNAV", skip_serializing_if = "Option::is_none")]
	pub pctg_of_fnd_ttl_nav: Option<f64>,
}

impl Fund4 {
	pub fn validate(&self) -> bool {
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref lgl_ntty_idr_value) = self.lgl_ntty_idr { if !lgl_ntty_idr_value.validate() { return false; } }
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
		if let Some(ref ccy_value) = self.ccy { if !ccy_value.validate() { return false; } }
		if let Some(ref ttl_nav_value) = self.ttl_nav { if !ttl_nav_value.validate() { return false; } }
		if let Some(ref prvs_ttl_nav_value) = self.prvs_ttl_nav { if !prvs_ttl_nav_value.validate() { return false; } }
		if let Some(ref ttl_units_nb_value) = self.ttl_units_nb { if !ttl_units_nb_value.validate() { return false; } }
		if let Some(ref prvs_ttl_units_nb_value) = self.prvs_ttl_units_nb { if !prvs_ttl_units_nb_value.validate() { return false; } }
		return true
	}
}


// FundBalance1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundBalance1 {
	#[serde(rename = "TtlUnitsFrUnitOrdrs", skip_serializing_if = "Option::is_none")]
	pub ttl_units_fr_unit_ordrs: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "TtlUnitsFrCshOrdrs", skip_serializing_if = "Option::is_none")]
	pub ttl_units_fr_csh_ordrs: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "TtlCshFrUnitOrdrs", skip_serializing_if = "Option::is_none")]
	pub ttl_csh_fr_unit_ordrs: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "TtlCshFrCshOrdrs", skip_serializing_if = "Option::is_none")]
	pub ttl_csh_fr_csh_ordrs: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl FundBalance1 {
	pub fn validate(&self) -> bool {
		if let Some(ref ttl_units_fr_unit_ordrs_value) = self.ttl_units_fr_unit_ordrs { if !ttl_units_fr_unit_ordrs_value.validate() { return false; } }
		if let Some(ref ttl_units_fr_csh_ordrs_value) = self.ttl_units_fr_csh_ordrs { if !ttl_units_fr_csh_ordrs_value.validate() { return false; } }
		if let Some(ref ttl_csh_fr_unit_ordrs_value) = self.ttl_csh_fr_unit_ordrs { if !ttl_csh_fr_unit_ordrs_value.validate() { return false; } }
		if let Some(ref ttl_csh_fr_csh_ordrs_value) = self.ttl_csh_fr_csh_ordrs { if !ttl_csh_fr_csh_ordrs_value.validate() { return false; } }
		return true
	}
}


// FundCashForecast6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundCashForecast6 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "TradDtTm")]
	pub trad_dt_tm: DateAndDateTimeChoice,
	#[serde(rename = "PrvsTradDtTm", skip_serializing_if = "Option::is_none")]
	pub prvs_trad_dt_tm: Option<DateAndDateTimeChoice>,
	#[serde(rename = "FinInstrmDtls")]
	pub fin_instrm_dtls: FinancialInstrument9,
	#[serde(rename = "TtlNAV", skip_serializing_if = "Option::is_none")]
	pub ttl_nav: Option<Vec<ActiveOrHistoricCurrencyAndAmount>>,
	#[serde(rename = "PrvsTtlNAV", skip_serializing_if = "Option::is_none")]
	pub prvs_ttl_nav: Option<Vec<ActiveOrHistoricCurrencyAndAmount>>,
	#[serde(rename = "TtlUnitsNb", skip_serializing_if = "Option::is_none")]
	pub ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "PrvsTtlUnitsNb", skip_serializing_if = "Option::is_none")]
	pub prvs_ttl_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "TtlNAVChngRate", skip_serializing_if = "Option::is_none")]
	pub ttl_nav_chng_rate: Option<f64>,
	#[serde(rename = "InvstmtCcy", skip_serializing_if = "Option::is_none")]
	pub invstmt_ccy: Option<Vec<ActiveOrHistoricCurrencyCode>>,
	#[serde(rename = "CcySts", skip_serializing_if = "Option::is_none")]
	pub ccy_sts: Option<CurrencyDesignation1>,
	#[serde(rename = "XcptnlNetCshFlowInd")]
	pub xcptnl_net_csh_flow_ind: bool,
	#[serde(rename = "Pric", skip_serializing_if = "Option::is_none")]
	pub pric: Option<UnitPrice19>,
	#[serde(rename = "FXRate", skip_serializing_if = "Option::is_none")]
	pub fx_rate: Option<ForeignExchangeTerms19>,
	#[serde(rename = "PctgOfShrClssTtlNAV", skip_serializing_if = "Option::is_none")]
	pub pctg_of_shr_clss_ttl_nav: Option<f64>,
	#[serde(rename = "BrkdwnByPty", skip_serializing_if = "Option::is_none")]
	pub brkdwn_by_pty: Option<Vec<BreakdownByParty3>>,
	#[serde(rename = "BrkdwnByCtry", skip_serializing_if = "Option::is_none")]
	pub brkdwn_by_ctry: Option<Vec<BreakdownByCountry2>>,
	#[serde(rename = "BrkdwnByCcy", skip_serializing_if = "Option::is_none")]
	pub brkdwn_by_ccy: Option<Vec<BreakdownByCurrency2>>,
	#[serde(rename = "BrkdwnByUsrDfndParam", skip_serializing_if = "Option::is_none")]
	pub brkdwn_by_usr_dfnd_param: Option<Vec<BreakdownByUserDefinedParameter3>>,
	#[serde(rename = "NetCshFcstDtls", skip_serializing_if = "Option::is_none")]
	pub net_csh_fcst_dtls: Option<Vec<NetCashForecast4>>,
}

impl FundCashForecast6 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.trad_dt_tm.validate() { return false }
		if let Some(ref prvs_trad_dt_tm_value) = self.prvs_trad_dt_tm { if !prvs_trad_dt_tm_value.validate() { return false; } }
		if !self.fin_instrm_dtls.validate() { return false }
		if let Some(ref ttl_nav_vec) = self.ttl_nav { for item in ttl_nav_vec { if !item.validate() { return false; } } }
		if let Some(ref prvs_ttl_nav_vec) = self.prvs_ttl_nav { for item in prvs_ttl_nav_vec { if !item.validate() { return false; } } }
		if let Some(ref ttl_units_nb_value) = self.ttl_units_nb { if !ttl_units_nb_value.validate() { return false; } }
		if let Some(ref prvs_ttl_units_nb_value) = self.prvs_ttl_units_nb { if !prvs_ttl_units_nb_value.validate() { return false; } }
		if let Some(ref invstmt_ccy_vec) = self.invstmt_ccy { for item in invstmt_ccy_vec { if !item.validate() { return false; } } }
		if let Some(ref ccy_sts_value) = self.ccy_sts { if !ccy_sts_value.validate() { return false; } }
		if let Some(ref pric_value) = self.pric { if !pric_value.validate() { return false; } }
		if let Some(ref fx_rate_value) = self.fx_rate { if !fx_rate_value.validate() { return false; } }
		if let Some(ref brkdwn_by_pty_vec) = self.brkdwn_by_pty { for item in brkdwn_by_pty_vec { if !item.validate() { return false; } } }
		if let Some(ref brkdwn_by_ctry_vec) = self.brkdwn_by_ctry { for item in brkdwn_by_ctry_vec { if !item.validate() { return false; } } }
		if let Some(ref brkdwn_by_ccy_vec) = self.brkdwn_by_ccy { for item in brkdwn_by_ccy_vec { if !item.validate() { return false; } } }
		if let Some(ref brkdwn_by_usr_dfnd_param_vec) = self.brkdwn_by_usr_dfnd_param { for item in brkdwn_by_usr_dfnd_param_vec { if !item.validate() { return false; } } }
		if let Some(ref net_csh_fcst_dtls_vec) = self.net_csh_fcst_dtls { for item in net_csh_fcst_dtls_vec { if !item.validate() { return false; } } }
		return true
	}
}


// FundCashInBreakdown3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundCashInBreakdown3 {
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "UnitsNb", skip_serializing_if = "Option::is_none")]
	pub units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "NewAmtInd", skip_serializing_if = "Option::is_none")]
	pub new_amt_ind: Option<bool>,
	#[serde(rename = "InvstmtFndTxInTp")]
	pub invstmt_fnd_tx_in_tp: InvestmentFundTransactionInType1Choice,
	#[serde(rename = "OrgnlOrdrQtyTp")]
	pub orgnl_ordr_qty_tp: QuantityType1Choice,
	#[serde(rename = "ChrgDtls", skip_serializing_if = "Option::is_none")]
	pub chrg_dtls: Option<Vec<Charge26>>,
	#[serde(rename = "ComssnDtls", skip_serializing_if = "Option::is_none")]
	pub comssn_dtls: Option<Vec<Commission21>>,
	#[serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none")]
	pub sttlm_ccy: Option<ActiveCurrencyCode>,
}

impl FundCashInBreakdown3 {
	pub fn validate(&self) -> bool {
		if let Some(ref amt_value) = self.amt { if !amt_value.validate() { return false; } }
		if let Some(ref units_nb_value) = self.units_nb { if !units_nb_value.validate() { return false; } }
		if !self.invstmt_fnd_tx_in_tp.validate() { return false }
		if !self.orgnl_ordr_qty_tp.validate() { return false }
		if let Some(ref chrg_dtls_vec) = self.chrg_dtls { for item in chrg_dtls_vec { if !item.validate() { return false; } } }
		if let Some(ref comssn_dtls_vec) = self.comssn_dtls { for item in comssn_dtls_vec { if !item.validate() { return false; } } }
		if let Some(ref sttlm_ccy_value) = self.sttlm_ccy { if !sttlm_ccy_value.validate() { return false; } }
		return true
	}
}


// FundCashOutBreakdown3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundCashOutBreakdown3 {
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "UnitsNb", skip_serializing_if = "Option::is_none")]
	pub units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "NewAmtInd", skip_serializing_if = "Option::is_none")]
	pub new_amt_ind: Option<bool>,
	#[serde(rename = "InvstmtFndTxOutTp")]
	pub invstmt_fnd_tx_out_tp: InvestmentFundTransactionOutType1Choice,
	#[serde(rename = "OrgnlOrdrQtyTp")]
	pub orgnl_ordr_qty_tp: QuantityType1Choice,
	#[serde(rename = "ChrgDtls", skip_serializing_if = "Option::is_none")]
	pub chrg_dtls: Option<Vec<Charge26>>,
	#[serde(rename = "ComssnDtls", skip_serializing_if = "Option::is_none")]
	pub comssn_dtls: Option<Vec<Commission21>>,
	#[serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none")]
	pub sttlm_ccy: Option<ActiveCurrencyCode>,
}

impl FundCashOutBreakdown3 {
	pub fn validate(&self) -> bool {
		if let Some(ref amt_value) = self.amt { if !amt_value.validate() { return false; } }
		if let Some(ref units_nb_value) = self.units_nb { if !units_nb_value.validate() { return false; } }
		if !self.invstmt_fnd_tx_out_tp.validate() { return false }
		if !self.orgnl_ordr_qty_tp.validate() { return false }
		if let Some(ref chrg_dtls_vec) = self.chrg_dtls { for item in chrg_dtls_vec { if !item.validate() { return false; } } }
		if let Some(ref comssn_dtls_vec) = self.comssn_dtls { for item in comssn_dtls_vec { if !item.validate() { return false; } } }
		if let Some(ref sttlm_ccy_value) = self.sttlm_ccy { if !sttlm_ccy_value.validate() { return false; } }
		return true
	}
}


// FundDetailedConfirmedCashForecastReportV04 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FundDetailedConfirmedCashForecastReportV04 {
	#[serde(rename = "MsgId")]
	pub msg_id: MessageIdentification1,
	#[serde(rename = "PoolRef", skip_serializing_if = "Option::is_none")]
	pub pool_ref: Option<AdditionalReference3>,
	#[serde(rename = "PrvsRef", skip_serializing_if = "Option::is_none")]
	pub prvs_ref: Option<Vec<AdditionalReference3>>,
	#[serde(rename = "RltdRef", skip_serializing_if = "Option::is_none")]
	pub rltd_ref: Option<Vec<AdditionalReference3>>,
	#[serde(rename = "MsgPgntn")]
	pub msg_pgntn: Pagination,
	#[serde(rename = "FndOrSubFndDtls", skip_serializing_if = "Option::is_none")]
	pub fnd_or_sub_fnd_dtls: Option<Fund4>,
	#[serde(rename = "FndCshFcstDtls")]
	pub fnd_csh_fcst_dtls: Vec<FundCashForecast6>,
	#[serde(rename = "CnsltdNetCshFcst", skip_serializing_if = "Option::is_none")]
	pub cnsltd_net_csh_fcst: Option<NetCashForecast3>,
	#[serde(rename = "Xtnsn", skip_serializing_if = "Option::is_none")]
	pub xtnsn: Option<Vec<Extension1>>,
}

impl FundDetailedConfirmedCashForecastReportV04 {
	pub fn validate(&self) -> bool {
		if !self.msg_id.validate() { return false }
		if let Some(ref pool_ref_value) = self.pool_ref { if !pool_ref_value.validate() { return false; } }
		if let Some(ref prvs_ref_vec) = self.prvs_ref { for item in prvs_ref_vec { if !item.validate() { return false; } } }
		if let Some(ref rltd_ref_vec) = self.rltd_ref { for item in rltd_ref_vec { if !item.validate() { return false; } } }
		if !self.msg_pgntn.validate() { return false }
		if let Some(ref fnd_or_sub_fnd_dtls_value) = self.fnd_or_sub_fnd_dtls { if !fnd_or_sub_fnd_dtls_value.validate() { return false; } }
		for item in &self.fnd_csh_fcst_dtls { if !item.validate() { return false; } }
		if let Some(ref cnsltd_net_csh_fcst_value) = self.cnsltd_net_csh_fcst { if !cnsltd_net_csh_fcst_value.validate() { return false; } }
		if let Some(ref xtnsn_vec) = self.xtnsn { for item in xtnsn_vec { if !item.validate() { return false; } } }
		return true
	}
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

impl GenericIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		return true
	}
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

impl GenericIdentification47 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.issr.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		return true
	}
}


// ISINIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISINIdentifier {
	#[serde(rename = "$value")]
	pub isin_identifier: String,
}

impl ISINIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{12,12}").unwrap();
		if !pattern.is_match(&self.isin_identifier) {
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


// IdentificationSource5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IdentificationSource5Choice {
	#[serde(rename = "DmstIdSrc", skip_serializing_if = "Option::is_none")]
	pub dmst_id_src: Option<CountryCode>,
	#[serde(rename = "PrtryIdSrc", skip_serializing_if = "Option::is_none")]
	pub prtry_id_src: Option<Max35Text>,
}

impl IdentificationSource5Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref dmst_id_src_value) = self.dmst_id_src { if !dmst_id_src_value.validate() { return false; } }
		if let Some(ref prtry_id_src_value) = self.prtry_id_src { if !prtry_id_src_value.validate() { return false; } }
		return true
	}
}


// InvestmentAccount42 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentAccount42 {
	#[serde(rename = "AcctId", skip_serializing_if = "Option::is_none")]
	pub acct_id: Option<AccountIdentification1>,
	#[serde(rename = "OwnrId", skip_serializing_if = "Option::is_none")]
	pub ownr_id: Option<PartyIdentification2Choice>,
	#[serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none")]
	pub acct_svcr: Option<PartyIdentification2Choice>,
}

impl InvestmentAccount42 {
	pub fn validate(&self) -> bool {
		if let Some(ref acct_id_value) = self.acct_id { if !acct_id_value.validate() { return false; } }
		if let Some(ref ownr_id_value) = self.ownr_id { if !ownr_id_value.validate() { return false; } }
		if let Some(ref acct_svcr_value) = self.acct_svcr { if !acct_svcr_value.validate() { return false; } }
		return true
	}
}


// InvestmentFundTransactionInType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentFundTransactionInType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InvestmentFundTransactionInType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl InvestmentFundTransactionInType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// InvestmentFundTransactionInType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InvestmentFundTransactionInType1Code {
	#[default]
	#[serde(rename = "SUBS")]
	CodeSUBS,
	#[serde(rename = "SWII")]
	CodeSWII,
	#[serde(rename = "INSP")]
	CodeINSP,
	#[serde(rename = "CROI")]
	CodeCROI,
	#[serde(rename = "RDIV")]
	CodeRDIV,
}

impl InvestmentFundTransactionInType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// InvestmentFundTransactionOutType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestmentFundTransactionOutType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InvestmentFundTransactionOutType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl InvestmentFundTransactionOutType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// InvestmentFundTransactionOutType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InvestmentFundTransactionOutType1Code {
	#[default]
	#[serde(rename = "REDM")]
	CodeREDM,
	#[serde(rename = "SWIO")]
	CodeSWIO,
	#[serde(rename = "INSP")]
	CodeINSP,
	#[serde(rename = "CROO")]
	CodeCROO,
}

impl InvestmentFundTransactionOutType1Code {
	pub fn validate(&self) -> bool {
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


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max16Text {
	#[serde(rename = "$value")]
	pub max16_text: String,
}

impl Max16Text {
	pub fn validate(&self) -> bool {
		if self.max16_text.chars().count() < 1 {
			return false
		}
		if self.max16_text.chars().count() > 16 {
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


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}

impl Max35Text {
	pub fn validate(&self) -> bool {
		if self.max35_text.chars().count() < 1 {
			return false
		}
		if self.max35_text.chars().count() > 35 {
			return false
		}
		return true
	}
}


// Max4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "$value")]
	pub max4_alpha_numeric_text: String,
}

impl Max4AlphaNumericText {
	pub fn validate(&self) -> bool {
		if self.max4_alpha_numeric_text.chars().count() < 1 {
			return false
		}
		if self.max4_alpha_numeric_text.chars().count() > 4 {
			return false
		}
		let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
		if !pattern.is_match(&self.max4_alpha_numeric_text) {
			return false
		}
		return true
	}
}


// Max5NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max5NumericText {
	#[serde(rename = "$value")]
	pub max5_numeric_text: String,
}

impl Max5NumericText {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[0-9]{1,5}").unwrap();
		if !pattern.is_match(&self.max5_numeric_text) {
			return false
		}
		return true
	}
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
}

impl Max70Text {
	pub fn validate(&self) -> bool {
		if self.max70_text.chars().count() < 1 {
			return false
		}
		if self.max70_text.chars().count() > 70 {
			return false
		}
		return true
	}
}


// MessageIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
}

impl MessageIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		return true
	}
}


// NameAndAddress5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<PostalAddress1>,
}

impl NameAndAddress5 {
	pub fn validate(&self) -> bool {
		if !self.nm.validate() { return false }
		if let Some(ref adr_value) = self.adr { if !adr_value.validate() { return false; } }
		return true
	}
}


// NetCashForecast3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NetCashForecast3 {
	#[serde(rename = "NetAmt", skip_serializing_if = "Option::is_none")]
	pub net_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "NetUnitsNb", skip_serializing_if = "Option::is_none")]
	pub net_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "FlowDrctn")]
	pub flow_drctn: FlowDirectionType1Code,
}

impl NetCashForecast3 {
	pub fn validate(&self) -> bool {
		if let Some(ref net_amt_value) = self.net_amt { if !net_amt_value.validate() { return false; } }
		if let Some(ref net_units_nb_value) = self.net_units_nb { if !net_units_nb_value.validate() { return false; } }
		if !self.flow_drctn.validate() { return false }
		return true
	}
}


// NetCashForecast4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NetCashForecast4 {
	#[serde(rename = "CshSttlmDt")]
	pub csh_sttlm_dt: String,
	#[serde(rename = "NetAmt", skip_serializing_if = "Option::is_none")]
	pub net_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "NetUnitsNb", skip_serializing_if = "Option::is_none")]
	pub net_units_nb: Option<FinancialInstrumentQuantity1>,
	#[serde(rename = "FlowDrctn")]
	pub flow_drctn: FlowDirectionType1Code,
	#[serde(rename = "AddtlBal", skip_serializing_if = "Option::is_none")]
	pub addtl_bal: Option<FundBalance1>,
}

impl NetCashForecast4 {
	pub fn validate(&self) -> bool {
		if let Some(ref net_amt_value) = self.net_amt { if !net_amt_value.validate() { return false; } }
		if let Some(ref net_units_nb_value) = self.net_units_nb { if !net_units_nb_value.validate() { return false; } }
		if !self.flow_drctn.validate() { return false }
		if let Some(ref addtl_bal_value) = self.addtl_bal { if !addtl_bal_value.validate() { return false; } }
		return true
	}
}


// OrderQuantityType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OrderQuantityType2Code {
	#[default]
	#[serde(rename = "UNIT")]
	CodeUNIT,
	#[serde(rename = "CASH")]
	CodeCASH,
}

impl OrderQuantityType2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// OtherIdentification4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherIdentification4 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Tp")]
	pub tp: IdentificationSource5Choice,
}

impl OtherIdentification4 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.tp.validate() { return false }
		return true
	}
}


// Pagination ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Pagination {
	#[serde(rename = "PgNb")]
	pub pg_nb: Max5NumericText,
	#[serde(rename = "LastPgInd")]
	pub last_pg_ind: bool,
}

impl Pagination {
	pub fn validate(&self) -> bool {
		if !self.pg_nb.validate() { return false }
		return true
	}
}


// PartyIdentification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification2Choice {
	#[serde(rename = "BICOrBEI", skip_serializing_if = "Option::is_none")]
	pub bic_or_bei: Option<AnyBICIdentifier>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification1>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
}

impl PartyIdentification2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref bic_or_bei_value) = self.bic_or_bei { if !bic_or_bei_value.validate() { return false; } }
		if let Some(ref prtry_id_value) = self.prtry_id { if !prtry_id_value.validate() { return false; } }
		if let Some(ref nm_and_adr_value) = self.nm_and_adr { if !nm_and_adr_value.validate() { return false; } }
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

impl PostalAddress1 {
	pub fn validate(&self) -> bool {
		if let Some(ref adr_tp_value) = self.adr_tp { if !adr_tp_value.validate() { return false; } }
		if let Some(ref adr_line_vec) = self.adr_line { for item in adr_line_vec { if !item.validate() { return false; } } }
		if let Some(ref strt_nm_value) = self.strt_nm { if !strt_nm_value.validate() { return false; } }
		if let Some(ref bldg_nb_value) = self.bldg_nb { if !bldg_nb_value.validate() { return false; } }
		if let Some(ref pst_cd_value) = self.pst_cd { if !pst_cd_value.validate() { return false; } }
		if let Some(ref twn_nm_value) = self.twn_nm { if !twn_nm_value.validate() { return false; } }
		if let Some(ref ctry_sub_dvsn_value) = self.ctry_sub_dvsn { if !ctry_sub_dvsn_value.validate() { return false; } }
		if !self.ctry.validate() { return false }
		return true
	}
}


// PriceValue1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceValue1 {
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAnd13DecimalAmount,
}

impl PriceValue1 {
	pub fn validate(&self) -> bool {
		if !self.amt.validate() { return false }
		return true
	}
}


// QUICKIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct QUICKIdentifier {
	#[serde(rename = "$value")]
	pub quick_identifier: String,
}

impl QUICKIdentifier {
	pub fn validate(&self) -> bool {
		return true
	}
}


// QuantityType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct QuantityType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<OrderQuantityType2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl QuantityType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// RICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RICIdentifier {
	#[serde(rename = "$value")]
	pub ric_identifier: String,
}

impl RICIdentifier {
	pub fn validate(&self) -> bool {
		if self.ric_identifier.chars().count() < 1 {
			return false
		}
		if self.ric_identifier.chars().count() > 35 {
			return false
		}
		return true
	}
}


// SEDOLIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SEDOLIdentifier {
	#[serde(rename = "$value")]
	pub sedol_identifier: String,
}

impl SEDOLIdentifier {
	pub fn validate(&self) -> bool {
		return true
	}
}


// SecurityIdentification3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification3Choice {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINIdentifier>,
	#[serde(rename = "SEDOL", skip_serializing_if = "Option::is_none")]
	pub sedol: Option<String>,
	#[serde(rename = "CUSIP", skip_serializing_if = "Option::is_none")]
	pub cusip: Option<String>,
	#[serde(rename = "RIC", skip_serializing_if = "Option::is_none")]
	pub ric: Option<RICIdentifier>,
	#[serde(rename = "TckrSymb", skip_serializing_if = "Option::is_none")]
	pub tckr_symb: Option<TickerIdentifier>,
	#[serde(rename = "Blmbrg", skip_serializing_if = "Option::is_none")]
	pub blmbrg: Option<BloombergIdentifier>,
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
	pub othr_prtry_id: Option<AlternateSecurityIdentification1>,
}

impl SecurityIdentification3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref isin_value) = self.isin { if !isin_value.validate() { return false; } }
		if let Some(ref ric_value) = self.ric { if !ric_value.validate() { return false; } }
		if let Some(ref tckr_symb_value) = self.tckr_symb { if !tckr_symb_value.validate() { return false; } }
		if let Some(ref blmbrg_value) = self.blmbrg { if !blmbrg_value.validate() { return false; } }
		if let Some(ref cta_value) = self.cta { if !cta_value.validate() { return false; } }
		if let Some(ref cmon_value) = self.cmon { if !cmon_value.validate() { return false; } }
		if let Some(ref othr_prtry_id_value) = self.othr_prtry_id { if !othr_prtry_id_value.validate() { return false; } }
		return true
	}
}


// SicovamIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SicovamIdentifier {
	#[serde(rename = "$value")]
	pub sicovam_identifier: String,
}

impl SicovamIdentifier {
	pub fn validate(&self) -> bool {
		return true
	}
}


// SimpleIdentificationInformation ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SimpleIdentificationInformation {
	#[serde(rename = "Id")]
	pub id: Max35Text,
}

impl SimpleIdentificationInformation {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		return true
	}
}


// TickerIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TickerIdentifier {
	#[serde(rename = "$value")]
	pub ticker_identifier: String,
}

impl TickerIdentifier {
	pub fn validate(&self) -> bool {
		if self.ticker_identifier.chars().count() < 1 {
			return false
		}
		if self.ticker_identifier.chars().count() > 35 {
			return false
		}
		return true
	}
}


// TypeOfPrice10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypeOfPrice10Code {
	#[default]
	#[serde(rename = "BIDE")]
	CodeBIDE,
	#[serde(rename = "OFFR")]
	CodeOFFR,
	#[serde(rename = "NAVL")]
	CodeNAVL,
	#[serde(rename = "CREA")]
	CodeCREA,
	#[serde(rename = "CANC")]
	CodeCANC,
	#[serde(rename = "INTE")]
	CodeINTE,
	#[serde(rename = "SWNG")]
	CodeSWNG,
	#[serde(rename = "MIDD")]
	CodeMIDD,
	#[serde(rename = "RINV")]
	CodeRINV,
	#[serde(rename = "SWIC")]
	CodeSWIC,
	#[serde(rename = "DDVR")]
	CodeDDVR,
	#[serde(rename = "ACTU")]
	CodeACTU,
}

impl TypeOfPrice10Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// UnitPrice19 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitPrice19 {
	#[serde(rename = "PricTp")]
	pub pric_tp: UnitPriceType2Choice,
	#[serde(rename = "Val")]
	pub val: PriceValue1,
}

impl UnitPrice19 {
	pub fn validate(&self) -> bool {
		if !self.pric_tp.validate() { return false }
		if !self.val.validate() { return false }
		return true
	}
}


// UnitPriceType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitPriceType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<TypeOfPrice10Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification47>,
}

impl UnitPriceType2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// ValorenIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ValorenIdentifier {
	#[serde(rename = "$value")]
	pub valoren_identifier: String,
}

impl ValorenIdentifier {
	pub fn validate(&self) -> bool {
		return true
	}
}


// WertpapierIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WertpapierIdentifier {
	#[serde(rename = "$value")]
	pub wertpapier_identifier: String,
}

impl WertpapierIdentifier {
	pub fn validate(&self) -> bool {
		return true
	}
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct YesNoIndicator {
	#[serde(rename = "$value")]
	pub yes_no_indicator: bool,
}

impl YesNoIndicator {
	pub fn validate(&self) -> bool {
		return true
	}
}
