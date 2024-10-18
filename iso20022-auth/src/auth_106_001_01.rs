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


// AbnormalValuesData4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AbnormalValuesData4 {
	#[serde(rename = "CtrPtyId")]
	pub ctr_pty_id: CounterpartyData92,
	#[serde(rename = "NbOfDerivsRptd")]
	pub nb_of_derivs_rptd: f64,
	#[serde(rename = "NbOfDerivsRptdWthOtlrs")]
	pub nb_of_derivs_rptd_wth_otlrs: f64,
	#[serde(rename = "TxDtls", skip_serializing_if = "Option::is_none")]
	pub tx_dtls: Option<Vec<AbnormalValuesTransactionData2>>,
}

impl AbnormalValuesData4 {
	pub fn validate(&self) -> bool {
		if !self.ctr_pty_id.validate() { return false }
		if let Some(ref tx_dtls_vec) = self.tx_dtls { for item in tx_dtls_vec { if !item.validate() { return false; } } }
		return true
	}
}


// AbnormalValuesTransactionData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AbnormalValuesTransactionData2 {
	#[serde(rename = "TxId")]
	pub tx_id: TradeTransactionIdentification24,
	#[serde(rename = "NtnlAmt", skip_serializing_if = "Option::is_none")]
	pub ntnl_amt: Option<NotionalAmountLegs5>,
	#[serde(rename = "NtnlQty", skip_serializing_if = "Option::is_none")]
	pub ntnl_qty: Option<NotionalQuantityLegs5>,
}

impl AbnormalValuesTransactionData2 {
	pub fn validate(&self) -> bool {
		if !self.tx_id.validate() { return false }
		if let Some(ref ntnl_amt_value) = self.ntnl_amt { if !ntnl_amt_value.validate() { return false; } }
		if let Some(ref ntnl_qty_value) = self.ntnl_qty { if !ntnl_qty_value.validate() { return false; } }
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


// ActiveOrHistoricCurrencyAnd19DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyAnd19DecimalAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_and19_decimal_amount_simple_type: f64,
}

impl ActiveOrHistoricCurrencyAnd19DecimalAmountSimpleType {
	pub fn validate(&self) -> bool {
		if self.active_or_historic_currency_and19_decimal_amount_simple_type < 0.000000 {
			return false
		}
		return true
	}
}


// ActiveOrHistoricCurrencyAnd19DecimalAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd19DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveOrHistoricCurrencyAnd19DecimalAmount {
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


// AgreementType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgreementType2Choice {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<ExternalAgreementType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max50Text>,
}

impl AgreementType2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// AmountAndDirection106 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection106 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAnd19DecimalAmount,
	#[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
	pub sgn: Option<bool>,
}

impl AmountAndDirection106 {
	pub fn validate(&self) -> bool {
		if !self.amt.validate() { return false }
		return true
	}
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
}

impl AnyBICDec2014Identifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
		if !pattern.is_match(&self.any_bic_dec2014_identifier) {
			return false
		}
		return true
	}
}


// CollateralPortfolioCode5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralPortfolioCode5Choice {
	#[serde(rename = "Prtfl", skip_serializing_if = "Option::is_none")]
	pub prtfl: Option<PortfolioCode3Choice>,
	#[serde(rename = "MrgnPrtflCd", skip_serializing_if = "Option::is_none")]
	pub mrgn_prtfl_cd: Option<MarginPortfolio3>,
}

impl CollateralPortfolioCode5Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref prtfl_value) = self.prtfl { if !prtfl_value.validate() { return false; } }
		if let Some(ref mrgn_prtfl_cd_value) = self.mrgn_prtfl_cd { if !mrgn_prtfl_cd_value.validate() { return false; } }
		return true
	}
}


// CounterpartyData92 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyData92 {
	#[serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none")]
	pub rptg_ctr_pty: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "RptSubmitgNtty", skip_serializing_if = "Option::is_none")]
	pub rpt_submitg_ntty: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none")]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
}

impl CounterpartyData92 {
	pub fn validate(&self) -> bool {
		if let Some(ref rptg_ctr_pty_value) = self.rptg_ctr_pty { if !rptg_ctr_pty_value.validate() { return false; } }
		if let Some(ref rpt_submitg_ntty_value) = self.rpt_submitg_ntty { if !rpt_submitg_ntty_value.validate() { return false; } }
		if let Some(ref ntty_rspnsbl_for_rpt_value) = self.ntty_rspnsbl_for_rpt { if !ntty_rspnsbl_for_rpt_value.validate() { return false; } }
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


// DateAndDateTime2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTime2Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<String>,
}

impl DateAndDateTime2Choice {
	pub fn validate(&self) -> bool {
		return true
	}
}


// DerivativeEventType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum DerivativeEventType3Code {
	#[default]
	#[serde(rename = "ALOC")]
	CodeALOC,
	#[serde(rename = "CLRG")]
	CodeCLRG,
	#[serde(rename = "CLAL")]
	CodeCLAL,
	#[serde(rename = "COMP")]
	CodeCOMP,
	#[serde(rename = "CORP")]
	CodeCORP,
	#[serde(rename = "CREV")]
	CodeCREV,
	#[serde(rename = "ETRM")]
	CodeETRM,
	#[serde(rename = "EXER")]
	CodeEXER,
	#[serde(rename = "INCP")]
	CodeINCP,
	#[serde(rename = "NOVA")]
	CodeNOVA,
	#[serde(rename = "PTNG")]
	CodePTNG,
	#[serde(rename = "TRAD")]
	CodeTRAD,
	#[serde(rename = "UPDT")]
	CodeUPDT,
}

impl DerivativeEventType3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// DerivativesTradeWarningsReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativesTradeWarningsReportV01 {
	#[serde(rename = "WrnngsSttstcs")]
	pub wrnngs_sttstcs: StatisticsPerCounterparty16Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl DerivativesTradeWarningsReportV01 {
	pub fn validate(&self) -> bool {
		if !self.wrnngs_sttstcs.validate() { return false }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
	}
}


// DetailedAbnormalValuesStatistics4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DetailedAbnormalValuesStatistics4Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[serde(rename = "Rpt", skip_serializing_if = "Option::is_none")]
	pub rpt: Option<DetailedTransactionStatistics28>,
}

impl DetailedAbnormalValuesStatistics4Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref data_set_actn_value) = self.data_set_actn { if !data_set_actn_value.validate() { return false; } }
		if let Some(ref rpt_value) = self.rpt { if !rpt_value.validate() { return false; } }
		return true
	}
}


// DetailedMissingMarginInformationStatistics4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DetailedMissingMarginInformationStatistics4Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[serde(rename = "Rpt", skip_serializing_if = "Option::is_none")]
	pub rpt: Option<DetailedTransactionStatistics26>,
}

impl DetailedMissingMarginInformationStatistics4Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref data_set_actn_value) = self.data_set_actn { if !data_set_actn_value.validate() { return false; } }
		if let Some(ref rpt_value) = self.rpt { if !rpt_value.validate() { return false; } }
		return true
	}
}


// DetailedMissingValuationsStatistics4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DetailedMissingValuationsStatistics4Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[serde(rename = "Rpt", skip_serializing_if = "Option::is_none")]
	pub rpt: Option<DetailedTransactionStatistics27>,
}

impl DetailedMissingValuationsStatistics4Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref data_set_actn_value) = self.data_set_actn { if !data_set_actn_value.validate() { return false; } }
		if let Some(ref rpt_value) = self.rpt { if !rpt_value.validate() { return false; } }
		return true
	}
}


// DetailedStatisticsPerCounterparty17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DetailedStatisticsPerCounterparty17 {
	#[serde(rename = "RefDt")]
	pub ref_dt: String,
	#[serde(rename = "MssngValtn")]
	pub mssng_valtn: DetailedMissingValuationsStatistics4Choice,
	#[serde(rename = "MssngMrgnInf")]
	pub mssng_mrgn_inf: DetailedMissingMarginInformationStatistics4Choice,
	#[serde(rename = "AbnrmlVals")]
	pub abnrml_vals: DetailedAbnormalValuesStatistics4Choice,
}

impl DetailedStatisticsPerCounterparty17 {
	pub fn validate(&self) -> bool {
		if !self.mssng_valtn.validate() { return false }
		if !self.mssng_mrgn_inf.validate() { return false }
		if !self.abnrml_vals.validate() { return false }
		return true
	}
}


// DetailedTransactionStatistics26 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DetailedTransactionStatistics26 {
	#[serde(rename = "NbOfOutsdngDerivs")]
	pub nb_of_outsdng_derivs: f64,
	#[serde(rename = "NbOfOutsdngDerivsWthNoMrgnInf")]
	pub nb_of_outsdng_derivs_wth_no_mrgn_inf: f64,
	#[serde(rename = "NbOfOutsdngDerivsWthOutdtdMrgnInf")]
	pub nb_of_outsdng_derivs_wth_outdtd_mrgn_inf: f64,
	#[serde(rename = "Wrnngs")]
	pub wrnngs: Vec<MissingMarginData2>,
}

impl DetailedTransactionStatistics26 {
	pub fn validate(&self) -> bool {
		for item in &self.wrnngs { if !item.validate() { return false; } }
		return true
	}
}


// DetailedTransactionStatistics27 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DetailedTransactionStatistics27 {
	#[serde(rename = "NbOfOutsdngDerivs")]
	pub nb_of_outsdng_derivs: f64,
	#[serde(rename = "NbOfOutsdngDerivsWthNoValtn")]
	pub nb_of_outsdng_derivs_wth_no_valtn: f64,
	#[serde(rename = "NbOfOutsdngDerivsWthOutdtdValtn")]
	pub nb_of_outsdng_derivs_wth_outdtd_valtn: f64,
	#[serde(rename = "Wrnngs")]
	pub wrnngs: Vec<MissingValuationsData2>,
}

impl DetailedTransactionStatistics27 {
	pub fn validate(&self) -> bool {
		for item in &self.wrnngs { if !item.validate() { return false; } }
		return true
	}
}


// DetailedTransactionStatistics28 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DetailedTransactionStatistics28 {
	#[serde(rename = "NbOfDerivsRptd")]
	pub nb_of_derivs_rptd: f64,
	#[serde(rename = "NbOfDerivsRptdWthOtlrs")]
	pub nb_of_derivs_rptd_wth_otlrs: f64,
	#[serde(rename = "Wrnngs")]
	pub wrnngs: Vec<AbnormalValuesData4>,
}

impl DetailedTransactionStatistics28 {
	pub fn validate(&self) -> bool {
		for item in &self.wrnngs { if !item.validate() { return false; } }
		return true
	}
}


// ExternalAgreementType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalAgreementType1Code {
	#[serde(rename = "$value")]
	pub external_agreement_type1_code: String,
}

impl ExternalAgreementType1Code {
	pub fn validate(&self) -> bool {
		if self.external_agreement_type1_code.chars().count() < 1 {
			return false
		}
		if self.external_agreement_type1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// ExternalUnitOfMeasure1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalUnitOfMeasure1Code {
	#[serde(rename = "$value")]
	pub external_unit_of_measure1_code: String,
}

impl ExternalUnitOfMeasure1Code {
	pub fn validate(&self) -> bool {
		if self.external_unit_of_measure1_code.chars().count() < 1 {
			return false
		}
		if self.external_unit_of_measure1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// Frequency19Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Frequency19Code {
	#[default]
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "YEAR")]
	CodeYEAR,
	#[serde(rename = "ADHO")]
	CodeADHO,
	#[serde(rename = "EXPI")]
	CodeEXPI,
	#[serde(rename = "MIAN")]
	CodeMIAN,
	#[serde(rename = "QURT")]
	CodeQURT,
	#[serde(rename = "HOUL")]
	CodeHOUL,
	#[serde(rename = "ODMD")]
	CodeODMD,
}

impl Frequency19Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// GenericIdentification175 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification175 {
	#[serde(rename = "Id")]
	pub id: Max72Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericIdentification175 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
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


// LegalPersonIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LegalPersonIdentification1 {
	#[serde(rename = "Id")]
	pub id: OrganisationIdentification15Choice,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
}

impl LegalPersonIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref ctry_value) = self.ctry { if !ctry_value.validate() { return false; } }
		return true
	}
}


// LongFraction19DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LongFraction19DecimalNumber {
	#[serde(rename = "$value")]
	pub long_fraction19_decimal_number: f64,
}

impl LongFraction19DecimalNumber {
	pub fn validate(&self) -> bool {
		return true
	}
}


// MarginPortfolio3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarginPortfolio3 {
	#[serde(rename = "InitlMrgnPrtflCd")]
	pub initl_mrgn_prtfl_cd: PortfolioCode5Choice,
	#[serde(rename = "VartnMrgnPrtflCd", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
}

impl MarginPortfolio3 {
	pub fn validate(&self) -> bool {
		if !self.initl_mrgn_prtfl_cd.validate() { return false }
		if let Some(ref vartn_mrgn_prtfl_cd_value) = self.vartn_mrgn_prtfl_cd { if !vartn_mrgn_prtfl_cd_value.validate() { return false; } }
		return true
	}
}


// MasterAgreement8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MasterAgreement8 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<AgreementType2Choice>,
	#[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
	pub vrsn: Option<Max50Text>,
	#[serde(rename = "OthrMstrAgrmtDtls", skip_serializing_if = "Option::is_none")]
	pub othr_mstr_agrmt_dtls: Option<Max350Text>,
}

impl MasterAgreement8 {
	pub fn validate(&self) -> bool {
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		if let Some(ref vrsn_value) = self.vrsn { if !vrsn_value.validate() { return false; } }
		if let Some(ref othr_mstr_agrmt_dtls_value) = self.othr_mstr_agrmt_dtls { if !othr_mstr_agrmt_dtls_value.validate() { return false; } }
		return true
	}
}


// Max105Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max105Text {
	#[serde(rename = "$value")]
	pub max105_text: String,
}

impl Max105Text {
	pub fn validate(&self) -> bool {
		if self.max105_text.chars().count() < 1 {
			return false
		}
		if self.max105_text.chars().count() > 105 {
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


// Max3Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max3Number {
	#[serde(rename = "$value")]
	pub max3_number: f64,
}

impl Max3Number {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Max500Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max500Text {
	#[serde(rename = "$value")]
	pub max500_text: String,
}

impl Max500Text {
	pub fn validate(&self) -> bool {
		if self.max500_text.chars().count() < 1 {
			return false
		}
		if self.max500_text.chars().count() > 500 {
			return false
		}
		return true
	}
}


// Max50Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max50Text {
	#[serde(rename = "$value")]
	pub max50_text: String,
}

impl Max50Text {
	pub fn validate(&self) -> bool {
		if self.max50_text.chars().count() < 1 {
			return false
		}
		if self.max50_text.chars().count() > 50 {
			return false
		}
		return true
	}
}


// Max52Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max52Text {
	#[serde(rename = "$value")]
	pub max52_text: String,
}

impl Max52Text {
	pub fn validate(&self) -> bool {
		if self.max52_text.chars().count() < 1 {
			return false
		}
		if self.max52_text.chars().count() > 52 {
			return false
		}
		return true
	}
}


// Max72Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max72Text {
	#[serde(rename = "$value")]
	pub max72_text: String,
}

impl Max72Text {
	pub fn validate(&self) -> bool {
		if self.max72_text.chars().count() < 1 {
			return false
		}
		if self.max72_text.chars().count() > 72 {
			return false
		}
		return true
	}
}


// MissingMarginData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MissingMarginData2 {
	#[serde(rename = "CtrPtyId")]
	pub ctr_pty_id: CounterpartyData92,
	#[serde(rename = "NbOfOutsdngDerivs")]
	pub nb_of_outsdng_derivs: f64,
	#[serde(rename = "NbOfOutsdngDerivsWthNoMrgnInf")]
	pub nb_of_outsdng_derivs_wth_no_mrgn_inf: f64,
	#[serde(rename = "NbOfOutsdngDerivsWthOutdtdMrgnInf")]
	pub nb_of_outsdng_derivs_wth_outdtd_mrgn_inf: f64,
	#[serde(rename = "TxDtls", skip_serializing_if = "Option::is_none")]
	pub tx_dtls: Option<Vec<MissingMarginTransactionData2>>,
}

impl MissingMarginData2 {
	pub fn validate(&self) -> bool {
		if !self.ctr_pty_id.validate() { return false }
		if let Some(ref tx_dtls_vec) = self.tx_dtls { for item in tx_dtls_vec { if !item.validate() { return false; } } }
		return true
	}
}


// MissingMarginTransactionData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MissingMarginTransactionData2 {
	#[serde(rename = "TxId")]
	pub tx_id: TradeTransactionIdentification24,
	#[serde(rename = "CollTmStmp", skip_serializing_if = "Option::is_none")]
	pub coll_tm_stmp: Option<String>,
}

impl MissingMarginTransactionData2 {
	pub fn validate(&self) -> bool {
		if !self.tx_id.validate() { return false }
		return true
	}
}


// MissingValuationsData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MissingValuationsData2 {
	#[serde(rename = "CtrPtyId")]
	pub ctr_pty_id: CounterpartyData92,
	#[serde(rename = "NbOfOutsdngDerivs")]
	pub nb_of_outsdng_derivs: f64,
	#[serde(rename = "NbOfOutsdngDerivsWthNoValtn")]
	pub nb_of_outsdng_derivs_wth_no_valtn: f64,
	#[serde(rename = "NbOfOutsdngDerivsWthOutdtdValtn")]
	pub nb_of_outsdng_derivs_wth_outdtd_valtn: f64,
	#[serde(rename = "TxDtls", skip_serializing_if = "Option::is_none")]
	pub tx_dtls: Option<Vec<MissingValuationsTransactionData2>>,
}

impl MissingValuationsData2 {
	pub fn validate(&self) -> bool {
		if !self.ctr_pty_id.validate() { return false }
		if let Some(ref tx_dtls_vec) = self.tx_dtls { for item in tx_dtls_vec { if !item.validate() { return false; } } }
		return true
	}
}


// MissingValuationsTransactionData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MissingValuationsTransactionData2 {
	#[serde(rename = "TxId")]
	pub tx_id: TradeTransactionIdentification24,
	#[serde(rename = "ValtnAmt", skip_serializing_if = "Option::is_none")]
	pub valtn_amt: Option<AmountAndDirection106>,
	#[serde(rename = "ValtnTmStmp", skip_serializing_if = "Option::is_none")]
	pub valtn_tm_stmp: Option<DateAndDateTime2Choice>,
}

impl MissingValuationsTransactionData2 {
	pub fn validate(&self) -> bool {
		if !self.tx_id.validate() { return false }
		if let Some(ref valtn_amt_value) = self.valtn_amt { if !valtn_amt_value.validate() { return false; } }
		if let Some(ref valtn_tm_stmp_value) = self.valtn_tm_stmp { if !valtn_tm_stmp_value.validate() { return false; } }
		return true
	}
}


// NaturalPersonIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NaturalPersonIdentification2 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max105Text>,
	#[serde(rename = "Dmcl", skip_serializing_if = "Option::is_none")]
	pub dmcl: Option<Max500Text>,
}

impl NaturalPersonIdentification2 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref dmcl_value) = self.dmcl { if !dmcl_value.validate() { return false; } }
		return true
	}
}


// NaturalPersonIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NaturalPersonIdentification3 {
	#[serde(rename = "Id")]
	pub id: NaturalPersonIdentification2,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
}

impl NaturalPersonIdentification3 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref ctry_value) = self.ctry { if !ctry_value.validate() { return false; } }
		return true
	}
}


// NotApplicable1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NotApplicable1Code {
	#[default]
	#[serde(rename = "NOAP")]
	CodeNOAP,
}

impl NotApplicable1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// NotionalAmount5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalAmount5 {
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<AmountAndDirection106>,
	#[serde(rename = "SchdlPrd", skip_serializing_if = "Option::is_none")]
	pub schdl_prd: Option<Vec<Schedule11>>,
}

impl NotionalAmount5 {
	pub fn validate(&self) -> bool {
		if let Some(ref amt_value) = self.amt { if !amt_value.validate() { return false; } }
		if let Some(ref schdl_prd_vec) = self.schdl_prd { for item in schdl_prd_vec { if !item.validate() { return false; } } }
		return true
	}
}


// NotionalAmount6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalAmount6 {
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<AmountAndDirection106>,
	#[serde(rename = "SchdlPrd", skip_serializing_if = "Option::is_none")]
	pub schdl_prd: Option<Vec<Schedule11>>,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveOrHistoricCurrencyCode>,
}

impl NotionalAmount6 {
	pub fn validate(&self) -> bool {
		if let Some(ref amt_value) = self.amt { if !amt_value.validate() { return false; } }
		if let Some(ref schdl_prd_vec) = self.schdl_prd { for item in schdl_prd_vec { if !item.validate() { return false; } } }
		if let Some(ref ccy_value) = self.ccy { if !ccy_value.validate() { return false; } }
		return true
	}
}


// NotionalAmountLegs5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalAmountLegs5 {
	#[serde(rename = "FrstLeg", skip_serializing_if = "Option::is_none")]
	pub frst_leg: Option<NotionalAmount5>,
	#[serde(rename = "ScndLeg", skip_serializing_if = "Option::is_none")]
	pub scnd_leg: Option<NotionalAmount6>,
}

impl NotionalAmountLegs5 {
	pub fn validate(&self) -> bool {
		if let Some(ref frst_leg_value) = self.frst_leg { if !frst_leg_value.validate() { return false; } }
		if let Some(ref scnd_leg_value) = self.scnd_leg { if !scnd_leg_value.validate() { return false; } }
		return true
	}
}


// NotionalQuantity9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalQuantity9 {
	#[serde(rename = "TtlQty", skip_serializing_if = "Option::is_none")]
	pub ttl_qty: Option<f64>,
	#[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
	pub unit_of_measr: Option<UnitOfMeasure8Choice>,
	#[serde(rename = "Dtls", skip_serializing_if = "Option::is_none")]
	pub dtls: Option<QuantityOrTerm1Choice>,
}

impl NotionalQuantity9 {
	pub fn validate(&self) -> bool {
		if let Some(ref unit_of_measr_value) = self.unit_of_measr { if !unit_of_measr_value.validate() { return false; } }
		if let Some(ref dtls_value) = self.dtls { if !dtls_value.validate() { return false; } }
		return true
	}
}


// NotionalQuantityLegs5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalQuantityLegs5 {
	#[serde(rename = "FrstLeg", skip_serializing_if = "Option::is_none")]
	pub frst_leg: Option<NotionalQuantity9>,
	#[serde(rename = "ScndLeg", skip_serializing_if = "Option::is_none")]
	pub scnd_leg: Option<NotionalQuantity9>,
}

impl NotionalQuantityLegs5 {
	pub fn validate(&self) -> bool {
		if let Some(ref frst_leg_value) = self.frst_leg { if !frst_leg_value.validate() { return false; } }
		if let Some(ref scnd_leg_value) = self.scnd_leg { if !scnd_leg_value.validate() { return false; } }
		return true
	}
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Number {
	#[serde(rename = "$value")]
	pub number: f64,
}

impl Number {
	pub fn validate(&self) -> bool {
		return true
	}
}


// OrganisationIdentification15Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification15Choice {
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<OrganisationIdentification38>,
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
}

impl OrganisationIdentification15Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref lei_value) = self.lei { if !lei_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		if let Some(ref any_bic_value) = self.any_bic { if !any_bic_value.validate() { return false; } }
		return true
	}
}


// OrganisationIdentification38 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification38 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max105Text>,
	#[serde(rename = "Dmcl", skip_serializing_if = "Option::is_none")]
	pub dmcl: Option<Max500Text>,
}

impl OrganisationIdentification38 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref dmcl_value) = self.dmcl { if !dmcl_value.validate() { return false; } }
		return true
	}
}


// PartyIdentification248Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification248Choice {
	#[serde(rename = "Lgl", skip_serializing_if = "Option::is_none")]
	pub lgl: Option<LegalPersonIdentification1>,
	#[serde(rename = "Ntrl", skip_serializing_if = "Option::is_none")]
	pub ntrl: Option<NaturalPersonIdentification3>,
}

impl PartyIdentification248Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref lgl_value) = self.lgl { if !lgl_value.validate() { return false; } }
		if let Some(ref ntrl_value) = self.ntrl { if !ntrl_value.validate() { return false; } }
		return true
	}
}


// PlusOrMinusIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "$value")]
	pub plus_or_minus_indicator: bool,
}

impl PlusOrMinusIndicator {
	pub fn validate(&self) -> bool {
		return true
	}
}


// PortfolioCode3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioCode3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Max52Text>,
	#[serde(rename = "NoPrtfl", skip_serializing_if = "Option::is_none")]
	pub no_prtfl: Option<NotApplicable1Code>,
}

impl PortfolioCode3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref no_prtfl_value) = self.no_prtfl { if !no_prtfl_value.validate() { return false; } }
		return true
	}
}


// PortfolioCode5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioCode5Choice {
	#[serde(rename = "Prtfl", skip_serializing_if = "Option::is_none")]
	pub prtfl: Option<PortfolioIdentification3>,
	#[serde(rename = "NoPrtfl", skip_serializing_if = "Option::is_none")]
	pub no_prtfl: Option<NotApplicable1Code>,
}

impl PortfolioCode5Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref prtfl_value) = self.prtfl { if !prtfl_value.validate() { return false; } }
		if let Some(ref no_prtfl_value) = self.no_prtfl { if !no_prtfl_value.validate() { return false; } }
		return true
	}
}


// PortfolioIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioIdentification3 {
	#[serde(rename = "Cd")]
	pub cd: Max52Text,
	#[serde(rename = "PrtflTxXmptn", skip_serializing_if = "Option::is_none")]
	pub prtfl_tx_xmptn: Option<bool>,
}

impl PortfolioIdentification3 {
	pub fn validate(&self) -> bool {
		if !self.cd.validate() { return false }
		return true
	}
}


// QuantityOrTerm1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct QuantityOrTerm1Choice {
	#[serde(rename = "SchdlPrd", skip_serializing_if = "Option::is_none")]
	pub schdl_prd: Option<Vec<Schedule10>>,
	#[serde(rename = "Term", skip_serializing_if = "Option::is_none")]
	pub term: Option<QuantityTerm1>,
}

impl QuantityOrTerm1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref schdl_prd_vec) = self.schdl_prd { for item in schdl_prd_vec { if !item.validate() { return false; } } }
		if let Some(ref term_value) = self.term { if !term_value.validate() { return false; } }
		return true
	}
}


// QuantityTerm1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct QuantityTerm1 {
	#[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
	pub qty: Option<f64>,
	#[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
	pub unit_of_measr: Option<UnitOfMeasure8Choice>,
	#[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
	pub val: Option<f64>,
	#[serde(rename = "TmUnit", skip_serializing_if = "Option::is_none")]
	pub tm_unit: Option<Frequency19Code>,
}

impl QuantityTerm1 {
	pub fn validate(&self) -> bool {
		if let Some(ref unit_of_measr_value) = self.unit_of_measr { if !unit_of_measr_value.validate() { return false; } }
		if let Some(ref tm_unit_value) = self.tm_unit { if !tm_unit_value.validate() { return false; } }
		return true
	}
}


// ReportPeriodActivity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ReportPeriodActivity1Code {
	#[default]
	#[serde(rename = "NOTX")]
	CodeNOTX,
}

impl ReportPeriodActivity1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Schedule10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Schedule10 {
	#[serde(rename = "Qty")]
	pub qty: f64,
	#[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
	pub unit_of_measr: Option<UnitOfMeasure8Choice>,
	#[serde(rename = "UadjstdFctvDt")]
	pub uadjstd_fctv_dt: String,
	#[serde(rename = "UadjstdEndDt", skip_serializing_if = "Option::is_none")]
	pub uadjstd_end_dt: Option<String>,
}

impl Schedule10 {
	pub fn validate(&self) -> bool {
		if let Some(ref unit_of_measr_value) = self.unit_of_measr { if !unit_of_measr_value.validate() { return false; } }
		return true
	}
}


// Schedule11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Schedule11 {
	#[serde(rename = "UadjstdFctvDt")]
	pub uadjstd_fctv_dt: String,
	#[serde(rename = "UadjstdEndDt", skip_serializing_if = "Option::is_none")]
	pub uadjstd_end_dt: Option<String>,
	#[serde(rename = "Amt")]
	pub amt: AmountAndDirection106,
}

impl Schedule11 {
	pub fn validate(&self) -> bool {
		if !self.amt.validate() { return false }
		return true
	}
}


// StatisticsPerCounterparty16Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatisticsPerCounterparty16Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[serde(rename = "Rpt", skip_serializing_if = "Option::is_none")]
	pub rpt: Option<DetailedStatisticsPerCounterparty17>,
}

impl StatisticsPerCounterparty16Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref data_set_actn_value) = self.data_set_actn { if !data_set_actn_value.validate() { return false; } }
		if let Some(ref rpt_value) = self.rpt { if !rpt_value.validate() { return false; } }
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


// TradeTransactionIdentification24 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeTransactionIdentification24 {
	#[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
	pub tech_rcrd_id: Option<Max140Text>,
	#[serde(rename = "ActnTp", skip_serializing_if = "Option::is_none")]
	pub actn_tp: Option<TransactionOperationType10Code>,
	#[serde(rename = "RptgTmStmp", skip_serializing_if = "Option::is_none")]
	pub rptg_tm_stmp: Option<String>,
	#[serde(rename = "DerivEvtTp", skip_serializing_if = "Option::is_none")]
	pub deriv_evt_tp: Option<DerivativeEventType3Code>,
	#[serde(rename = "DerivEvtTmStmp", skip_serializing_if = "Option::is_none")]
	pub deriv_evt_tm_stmp: Option<DateAndDateTime2Choice>,
	#[serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none")]
	pub othr_ctr_pty: Option<PartyIdentification248Choice>,
	#[serde(rename = "UnqIdr", skip_serializing_if = "Option::is_none")]
	pub unq_idr: Option<UniqueTransactionIdentifier2Choice>,
	#[serde(rename = "MstrAgrmt", skip_serializing_if = "Option::is_none")]
	pub mstr_agrmt: Option<MasterAgreement8>,
	#[serde(rename = "CollPrtflCd", skip_serializing_if = "Option::is_none")]
	pub coll_prtfl_cd: Option<CollateralPortfolioCode5Choice>,
}

impl TradeTransactionIdentification24 {
	pub fn validate(&self) -> bool {
		if let Some(ref tech_rcrd_id_value) = self.tech_rcrd_id { if !tech_rcrd_id_value.validate() { return false; } }
		if let Some(ref actn_tp_value) = self.actn_tp { if !actn_tp_value.validate() { return false; } }
		if let Some(ref deriv_evt_tp_value) = self.deriv_evt_tp { if !deriv_evt_tp_value.validate() { return false; } }
		if let Some(ref deriv_evt_tm_stmp_value) = self.deriv_evt_tm_stmp { if !deriv_evt_tm_stmp_value.validate() { return false; } }
		if let Some(ref othr_ctr_pty_value) = self.othr_ctr_pty { if !othr_ctr_pty_value.validate() { return false; } }
		if let Some(ref unq_idr_value) = self.unq_idr { if !unq_idr_value.validate() { return false; } }
		if let Some(ref mstr_agrmt_value) = self.mstr_agrmt { if !mstr_agrmt_value.validate() { return false; } }
		if let Some(ref coll_prtfl_cd_value) = self.coll_prtfl_cd { if !coll_prtfl_cd_value.validate() { return false; } }
		return true
	}
}


// TransactionOperationType10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TransactionOperationType10Code {
	#[default]
	#[serde(rename = "COMP")]
	CodeCOMP,
	#[serde(rename = "CORR")]
	CodeCORR,
	#[serde(rename = "EROR")]
	CodeEROR,
	#[serde(rename = "MODI")]
	CodeMODI,
	#[serde(rename = "NEWT")]
	CodeNEWT,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "POSC")]
	CodePOSC,
	#[serde(rename = "REVI")]
	CodeREVI,
	#[serde(rename = "TERM")]
	CodeTERM,
	#[serde(rename = "VALU")]
	CodeVALU,
	#[serde(rename = "MARU")]
	CodeMARU,
	#[serde(rename = "PRTO")]
	CodePRTO,
}

impl TransactionOperationType10Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}

impl TrueFalseIndicator {
	pub fn validate(&self) -> bool {
		return true
	}
}


// UTIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UTIIdentifier {
	#[serde(rename = "$value")]
	pub uti_identifier: String,
}

impl UTIIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{18}[0-9]{2}[A-Z0-9]{0,32}").unwrap();
		if !pattern.is_match(&self.uti_identifier) {
			return false
		}
		return true
	}
}


// UniqueTransactionIdentifier2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UniqueTransactionIdentifier2Choice {
	#[serde(rename = "UnqTxIdr", skip_serializing_if = "Option::is_none")]
	pub unq_tx_idr: Option<UTIIdentifier>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification175>,
}

impl UniqueTransactionIdentifier2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref unq_tx_idr_value) = self.unq_tx_idr { if !unq_tx_idr_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// UnitOfMeasure8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitOfMeasure8Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalUnitOfMeasure1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification175>,
}

impl UnitOfMeasure8Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}
