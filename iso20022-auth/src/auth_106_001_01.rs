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

#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub mod iso20022 {
	use regex::Regex;
	use crate::common::*;
	#[cfg(feature = "derive_serde")]
	use serde::{Deserialize, Serialize};
	
	
	// AbnormalValuesData4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AbnormalValuesData4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtyId") )]
		pub ctr_pty_id: CounterpartyData92,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfDerivsRptd") )]
		pub nb_of_derivs_rptd: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfDerivsRptdWthOtlrs") )]
		pub nb_of_derivs_rptd_wth_otlrs: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxDtls", skip_serializing_if = "Option::is_none") )]
		pub tx_dtls: Option<Vec<AbnormalValuesTransactionData2>>,
	}
	
	impl AbnormalValuesData4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.ctr_pty_id.validate() { return Err(e); }
			if let Some(ref tx_dtls_vec) = self.tx_dtls { for item in tx_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// AbnormalValuesTransactionData2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AbnormalValuesTransactionData2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxId") )]
		pub tx_id: TradeTransactionIdentification24,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlAmt", skip_serializing_if = "Option::is_none") )]
		pub ntnl_amt: Option<NotionalAmountLegs5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlQty", skip_serializing_if = "Option::is_none") )]
		pub ntnl_qty: Option<NotionalQuantityLegs5>,
	}
	
	impl AbnormalValuesTransactionData2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tx_id.validate() { return Err(e); }
			if let Some(ref ntnl_amt_value) = self.ntnl_amt { if let Err(e) = ntnl_amt_value.validate() { return Err(e); } }
			if let Some(ref ntnl_qty_value) = self.ntnl_qty { if let Err(e) = ntnl_qty_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ActiveCurrencyCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveCurrencyCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_currency_code: String,
	}
	
	impl ActiveCurrencyCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(&self.active_currency_code) {
				return Err(ValidationError::new(1005, "active_currency_code does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ActiveOrHistoricCurrencyAnd19DecimalAmountSimpleType ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveOrHistoricCurrencyAnd19DecimalAmountSimpleType {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_or_historic_currency_and19_decimal_amount_simple_type: f64,
	}
	
	impl ActiveOrHistoricCurrencyAnd19DecimalAmountSimpleType {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.active_or_historic_currency_and19_decimal_amount_simple_type < 0.000000 {
				return Err(ValidationError::new(1003, "active_or_historic_currency_and19_decimal_amount_simple_type is less than the minimum value of 0.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ActiveOrHistoricCurrencyAnd19DecimalAmount ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ActiveOrHistoricCurrencyAnd19DecimalAmount {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
		pub ccy: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub value: f64,
	}
	
	impl ActiveOrHistoricCurrencyAnd19DecimalAmount {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ActiveOrHistoricCurrencyCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveOrHistoricCurrencyCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_or_historic_currency_code: String,
	}
	
	impl ActiveOrHistoricCurrencyCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(&self.active_or_historic_currency_code) {
				return Err(ValidationError::new(1005, "active_or_historic_currency_code does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// AgreementType2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AgreementType2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<ExternalAgreementType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max50Text>,
	}
	
	impl AgreementType2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AmountAndDirection106 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AmountAndDirection106 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveOrHistoricCurrencyAnd19DecimalAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sgn", skip_serializing_if = "Option::is_none") )]
		pub sgn: Option<bool>,
	}
	
	impl AmountAndDirection106 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.amt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// AnyBICDec2014Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct AnyBICDec2014Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub any_bic_dec2014_identifier: String,
	}
	
	impl AnyBICDec2014Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(&self.any_bic_dec2014_identifier) {
				return Err(ValidationError::new(1005, "any_bic_dec2014_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// CollateralPortfolioCode5Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CollateralPortfolioCode5Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtfl", skip_serializing_if = "Option::is_none") )]
		pub prtfl: Option<PortfolioCode3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MrgnPrtflCd", skip_serializing_if = "Option::is_none") )]
		pub mrgn_prtfl_cd: Option<MarginPortfolio3>,
	}
	
	impl CollateralPortfolioCode5Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref prtfl_value) = self.prtfl { if let Err(e) = prtfl_value.validate() { return Err(e); } }
			if let Some(ref mrgn_prtfl_cd_value) = self.mrgn_prtfl_cd { if let Err(e) = mrgn_prtfl_cd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CounterpartyData92 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CounterpartyData92 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none") )]
		pub rptg_ctr_pty: Option<OrganisationIdentification15Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptSubmitgNtty", skip_serializing_if = "Option::is_none") )]
		pub rpt_submitg_ntty: Option<OrganisationIdentification15Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none") )]
		pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
	}
	
	impl CounterpartyData92 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref rptg_ctr_pty_value) = self.rptg_ctr_pty { if let Err(e) = rptg_ctr_pty_value.validate() { return Err(e); } }
			if let Some(ref rpt_submitg_ntty_value) = self.rpt_submitg_ntty { if let Err(e) = rpt_submitg_ntty_value.validate() { return Err(e); } }
			if let Some(ref ntty_rspnsbl_for_rpt_value) = self.ntty_rspnsbl_for_rpt { if let Err(e) = ntty_rspnsbl_for_rpt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CountryCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct CountryCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub country_code: String,
	}
	
	impl CountryCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(&self.country_code) {
				return Err(ValidationError::new(1005, "country_code does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// DateAndDateTime2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct DateAndDateTime2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
		pub dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtTm", skip_serializing_if = "Option::is_none") )]
		pub dt_tm: Option<String>,
	}
	
	impl DateAndDateTime2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// DerivativeEventType3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum DerivativeEventType3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ALOC") )]
		CodeALOC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLRG") )]
		CodeCLRG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLAL") )]
		CodeCLAL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "COMP") )]
		CodeCOMP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CORP") )]
		CodeCORP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CREV") )]
		CodeCREV,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ETRM") )]
		CodeETRM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EXER") )]
		CodeEXER,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INCP") )]
		CodeINCP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NOVA") )]
		CodeNOVA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PTNG") )]
		CodePTNG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TRAD") )]
		CodeTRAD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UPDT") )]
		CodeUPDT,
	}
	
	impl DerivativeEventType3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// DerivativesTradeWarningsReportV01 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct DerivativesTradeWarningsReportV01 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "WrnngsSttstcs") )]
		pub wrnngs_sttstcs: StatisticsPerCounterparty16Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl DerivativesTradeWarningsReportV01 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.wrnngs_sttstcs.validate() { return Err(e); }
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// DetailedAbnormalValuesStatistics4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct DetailedAbnormalValuesStatistics4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
		pub data_set_actn: Option<ReportPeriodActivity1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rpt", skip_serializing_if = "Option::is_none") )]
		pub rpt: Option<DetailedTransactionStatistics28>,
	}
	
	impl DetailedAbnormalValuesStatistics4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref data_set_actn_value) = self.data_set_actn { if let Err(e) = data_set_actn_value.validate() { return Err(e); } }
			if let Some(ref rpt_value) = self.rpt { if let Err(e) = rpt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DetailedMissingMarginInformationStatistics4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct DetailedMissingMarginInformationStatistics4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
		pub data_set_actn: Option<ReportPeriodActivity1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rpt", skip_serializing_if = "Option::is_none") )]
		pub rpt: Option<DetailedTransactionStatistics26>,
	}
	
	impl DetailedMissingMarginInformationStatistics4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref data_set_actn_value) = self.data_set_actn { if let Err(e) = data_set_actn_value.validate() { return Err(e); } }
			if let Some(ref rpt_value) = self.rpt { if let Err(e) = rpt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DetailedMissingValuationsStatistics4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct DetailedMissingValuationsStatistics4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
		pub data_set_actn: Option<ReportPeriodActivity1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rpt", skip_serializing_if = "Option::is_none") )]
		pub rpt: Option<DetailedTransactionStatistics27>,
	}
	
	impl DetailedMissingValuationsStatistics4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref data_set_actn_value) = self.data_set_actn { if let Err(e) = data_set_actn_value.validate() { return Err(e); } }
			if let Some(ref rpt_value) = self.rpt { if let Err(e) = rpt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DetailedStatisticsPerCounterparty17 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct DetailedStatisticsPerCounterparty17 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RefDt") )]
		pub ref_dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MssngValtn") )]
		pub mssng_valtn: DetailedMissingValuationsStatistics4Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MssngMrgnInf") )]
		pub mssng_mrgn_inf: DetailedMissingMarginInformationStatistics4Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AbnrmlVals") )]
		pub abnrml_vals: DetailedAbnormalValuesStatistics4Choice,
	}
	
	impl DetailedStatisticsPerCounterparty17 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.mssng_valtn.validate() { return Err(e); }
			if let Err(e) = self.mssng_mrgn_inf.validate() { return Err(e); }
			if let Err(e) = self.abnrml_vals.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// DetailedTransactionStatistics26 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct DetailedTransactionStatistics26 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfOutsdngDerivs") )]
		pub nb_of_outsdng_derivs: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfOutsdngDerivsWthNoMrgnInf") )]
		pub nb_of_outsdng_derivs_wth_no_mrgn_inf: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfOutsdngDerivsWthOutdtdMrgnInf") )]
		pub nb_of_outsdng_derivs_wth_outdtd_mrgn_inf: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Wrnngs") )]
		pub wrnngs: Vec<MissingMarginData2>,
	}
	
	impl DetailedTransactionStatistics26 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			for item in &self.wrnngs { if let Err(e) = item.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DetailedTransactionStatistics27 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct DetailedTransactionStatistics27 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfOutsdngDerivs") )]
		pub nb_of_outsdng_derivs: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfOutsdngDerivsWthNoValtn") )]
		pub nb_of_outsdng_derivs_wth_no_valtn: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfOutsdngDerivsWthOutdtdValtn") )]
		pub nb_of_outsdng_derivs_wth_outdtd_valtn: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Wrnngs") )]
		pub wrnngs: Vec<MissingValuationsData2>,
	}
	
	impl DetailedTransactionStatistics27 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			for item in &self.wrnngs { if let Err(e) = item.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DetailedTransactionStatistics28 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct DetailedTransactionStatistics28 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfDerivsRptd") )]
		pub nb_of_derivs_rptd: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfDerivsRptdWthOtlrs") )]
		pub nb_of_derivs_rptd_wth_otlrs: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Wrnngs") )]
		pub wrnngs: Vec<AbnormalValuesData4>,
	}
	
	impl DetailedTransactionStatistics28 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			for item in &self.wrnngs { if let Err(e) = item.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ExternalAgreementType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalAgreementType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_agreement_type1_code: String,
	}
	
	impl ExternalAgreementType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_agreement_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_agreement_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_agreement_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_agreement_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalUnitOfMeasure1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalUnitOfMeasure1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_unit_of_measure1_code: String,
	}
	
	impl ExternalUnitOfMeasure1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_unit_of_measure1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_unit_of_measure1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_unit_of_measure1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_unit_of_measure1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Frequency19Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum Frequency19Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "DAIL") )]
		CodeDAIL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WEEK") )]
		CodeWEEK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
		CodeMNTH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
		CodeYEAR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ADHO") )]
		CodeADHO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EXPI") )]
		CodeEXPI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MIAN") )]
		CodeMIAN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QURT") )]
		CodeQURT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HOUL") )]
		CodeHOUL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ODMD") )]
		CodeODMD,
	}
	
	impl Frequency19Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// GenericIdentification175 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct GenericIdentification175 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max72Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl GenericIdentification175 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ISODate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISODate {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iso_date: String,
	}
	
	impl ISODate {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ISODateTime ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISODateTime {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iso_date_time: String,
	}
	
	impl ISODateTime {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// LEIIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct LEIIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub lei_identifier: String,
	}
	
	impl LEIIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(&self.lei_identifier) {
				return Err(ValidationError::new(1005, "lei_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// LegalPersonIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct LegalPersonIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: OrganisationIdentification15Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
		pub ctry: Option<CountryCode>,
	}
	
	impl LegalPersonIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref ctry_value) = self.ctry { if let Err(e) = ctry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// LongFraction19DecimalNumber ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct LongFraction19DecimalNumber {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub long_fraction19_decimal_number: f64,
	}
	
	impl LongFraction19DecimalNumber {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// MarginPortfolio3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct MarginPortfolio3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "InitlMrgnPrtflCd") )]
		pub initl_mrgn_prtfl_cd: PortfolioCode5Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VartnMrgnPrtflCd", skip_serializing_if = "Option::is_none") )]
		pub vartn_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
	}
	
	impl MarginPortfolio3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.initl_mrgn_prtfl_cd.validate() { return Err(e); }
			if let Some(ref vartn_mrgn_prtfl_cd_value) = self.vartn_mrgn_prtfl_cd { if let Err(e) = vartn_mrgn_prtfl_cd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// MasterAgreement8 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct MasterAgreement8 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<AgreementType2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Vrsn", skip_serializing_if = "Option::is_none") )]
		pub vrsn: Option<Max50Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrMstrAgrmtDtls", skip_serializing_if = "Option::is_none") )]
		pub othr_mstr_agrmt_dtls: Option<Max350Text>,
	}
	
	impl MasterAgreement8 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref vrsn_value) = self.vrsn { if let Err(e) = vrsn_value.validate() { return Err(e); } }
			if let Some(ref othr_mstr_agrmt_dtls_value) = self.othr_mstr_agrmt_dtls { if let Err(e) = othr_mstr_agrmt_dtls_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Max105Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max105Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max105_text: String,
	}
	
	impl Max105Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max105_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max105_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max105_text.chars().count() > 105 {
				return Err(ValidationError::new(1002, "max105_text exceeds the maximum length of 105".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max140Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max140Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max140_text: String,
	}
	
	impl Max140Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max140_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max140_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max140_text.chars().count() > 140 {
				return Err(ValidationError::new(1002, "max140_text exceeds the maximum length of 140".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max350Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max350Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max350_text: String,
	}
	
	impl Max350Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max350_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max350_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max350_text.chars().count() > 350 {
				return Err(ValidationError::new(1002, "max350_text exceeds the maximum length of 350".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max35Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max35Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max35_text: String,
	}
	
	impl Max35Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max35_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max35_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max35_text.chars().count() > 35 {
				return Err(ValidationError::new(1002, "max35_text exceeds the maximum length of 35".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max3Number ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max3Number {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max3_number: f64,
	}
	
	impl Max3Number {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Max500Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max500Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max500_text: String,
	}
	
	impl Max500Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max500_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max500_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max500_text.chars().count() > 500 {
				return Err(ValidationError::new(1002, "max500_text exceeds the maximum length of 500".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max50Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max50Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max50_text: String,
	}
	
	impl Max50Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max50_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max50_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max50_text.chars().count() > 50 {
				return Err(ValidationError::new(1002, "max50_text exceeds the maximum length of 50".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max52Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max52Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max52_text: String,
	}
	
	impl Max52Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max52_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max52_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max52_text.chars().count() > 52 {
				return Err(ValidationError::new(1002, "max52_text exceeds the maximum length of 52".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max72Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max72Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max72_text: String,
	}
	
	impl Max72Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max72_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max72_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max72_text.chars().count() > 72 {
				return Err(ValidationError::new(1002, "max72_text exceeds the maximum length of 72".to_string()));
			}
			Ok(())
		}
	}
	
	
	// MissingMarginData2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct MissingMarginData2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtyId") )]
		pub ctr_pty_id: CounterpartyData92,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfOutsdngDerivs") )]
		pub nb_of_outsdng_derivs: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfOutsdngDerivsWthNoMrgnInf") )]
		pub nb_of_outsdng_derivs_wth_no_mrgn_inf: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfOutsdngDerivsWthOutdtdMrgnInf") )]
		pub nb_of_outsdng_derivs_wth_outdtd_mrgn_inf: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxDtls", skip_serializing_if = "Option::is_none") )]
		pub tx_dtls: Option<Vec<MissingMarginTransactionData2>>,
	}
	
	impl MissingMarginData2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.ctr_pty_id.validate() { return Err(e); }
			if let Some(ref tx_dtls_vec) = self.tx_dtls { for item in tx_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// MissingMarginTransactionData2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct MissingMarginTransactionData2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxId") )]
		pub tx_id: TradeTransactionIdentification24,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CollTmStmp", skip_serializing_if = "Option::is_none") )]
		pub coll_tm_stmp: Option<String>,
	}
	
	impl MissingMarginTransactionData2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tx_id.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// MissingValuationsData2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct MissingValuationsData2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtyId") )]
		pub ctr_pty_id: CounterpartyData92,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfOutsdngDerivs") )]
		pub nb_of_outsdng_derivs: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfOutsdngDerivsWthNoValtn") )]
		pub nb_of_outsdng_derivs_wth_no_valtn: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfOutsdngDerivsWthOutdtdValtn") )]
		pub nb_of_outsdng_derivs_wth_outdtd_valtn: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxDtls", skip_serializing_if = "Option::is_none") )]
		pub tx_dtls: Option<Vec<MissingValuationsTransactionData2>>,
	}
	
	impl MissingValuationsData2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.ctr_pty_id.validate() { return Err(e); }
			if let Some(ref tx_dtls_vec) = self.tx_dtls { for item in tx_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// MissingValuationsTransactionData2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct MissingValuationsTransactionData2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxId") )]
		pub tx_id: TradeTransactionIdentification24,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ValtnAmt", skip_serializing_if = "Option::is_none") )]
		pub valtn_amt: Option<AmountAndDirection106>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ValtnTmStmp", skip_serializing_if = "Option::is_none") )]
		pub valtn_tm_stmp: Option<DateAndDateTime2Choice>,
	}
	
	impl MissingValuationsTransactionData2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tx_id.validate() { return Err(e); }
			if let Some(ref valtn_amt_value) = self.valtn_amt { if let Err(e) = valtn_amt_value.validate() { return Err(e); } }
			if let Some(ref valtn_tm_stmp_value) = self.valtn_tm_stmp { if let Err(e) = valtn_tm_stmp_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// NaturalPersonIdentification2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct NaturalPersonIdentification2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: GenericIdentification175,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max105Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dmcl", skip_serializing_if = "Option::is_none") )]
		pub dmcl: Option<Max500Text>,
	}
	
	impl NaturalPersonIdentification2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref dmcl_value) = self.dmcl { if let Err(e) = dmcl_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// NaturalPersonIdentification3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct NaturalPersonIdentification3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: NaturalPersonIdentification2,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
		pub ctry: Option<CountryCode>,
	}
	
	impl NaturalPersonIdentification3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref ctry_value) = self.ctry { if let Err(e) = ctry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// NotApplicable1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum NotApplicable1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "NOAP") )]
		CodeNOAP,
	}
	
	impl NotApplicable1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// NotionalAmount5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct NotionalAmount5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
		pub amt: Option<AmountAndDirection106>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchdlPrd", skip_serializing_if = "Option::is_none") )]
		pub schdl_prd: Option<Vec<Schedule11>>,
	}
	
	impl NotionalAmount5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
			if let Some(ref schdl_prd_vec) = self.schdl_prd { for item in schdl_prd_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// NotionalAmount6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct NotionalAmount6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
		pub amt: Option<AmountAndDirection106>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchdlPrd", skip_serializing_if = "Option::is_none") )]
		pub schdl_prd: Option<Vec<Schedule11>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy", skip_serializing_if = "Option::is_none") )]
		pub ccy: Option<ActiveOrHistoricCurrencyCode>,
	}
	
	impl NotionalAmount6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
			if let Some(ref schdl_prd_vec) = self.schdl_prd { for item in schdl_prd_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref ccy_value) = self.ccy { if let Err(e) = ccy_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// NotionalAmountLegs5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct NotionalAmountLegs5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrstLeg", skip_serializing_if = "Option::is_none") )]
		pub frst_leg: Option<NotionalAmount5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ScndLeg", skip_serializing_if = "Option::is_none") )]
		pub scnd_leg: Option<NotionalAmount6>,
	}
	
	impl NotionalAmountLegs5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref frst_leg_value) = self.frst_leg { if let Err(e) = frst_leg_value.validate() { return Err(e); } }
			if let Some(ref scnd_leg_value) = self.scnd_leg { if let Err(e) = scnd_leg_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// NotionalQuantity9 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct NotionalQuantity9 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlQty", skip_serializing_if = "Option::is_none") )]
		pub ttl_qty: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none") )]
		pub unit_of_measr: Option<UnitOfMeasure8Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dtls", skip_serializing_if = "Option::is_none") )]
		pub dtls: Option<QuantityOrTerm1Choice>,
	}
	
	impl NotionalQuantity9 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref unit_of_measr_value) = self.unit_of_measr { if let Err(e) = unit_of_measr_value.validate() { return Err(e); } }
			if let Some(ref dtls_value) = self.dtls { if let Err(e) = dtls_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// NotionalQuantityLegs5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct NotionalQuantityLegs5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrstLeg", skip_serializing_if = "Option::is_none") )]
		pub frst_leg: Option<NotionalQuantity9>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ScndLeg", skip_serializing_if = "Option::is_none") )]
		pub scnd_leg: Option<NotionalQuantity9>,
	}
	
	impl NotionalQuantityLegs5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref frst_leg_value) = self.frst_leg { if let Err(e) = frst_leg_value.validate() { return Err(e); } }
			if let Some(ref scnd_leg_value) = self.scnd_leg { if let Err(e) = scnd_leg_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Number ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Number {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub number: f64,
	}
	
	impl Number {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// OrganisationIdentification15Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct OrganisationIdentification15Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<LEIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<OrganisationIdentification38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
		pub any_bic: Option<AnyBICDec2014Identifier>,
	}
	
	impl OrganisationIdentification15Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			if let Some(ref any_bic_value) = self.any_bic { if let Err(e) = any_bic_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// OrganisationIdentification38 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct OrganisationIdentification38 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: GenericIdentification175,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max105Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dmcl", skip_serializing_if = "Option::is_none") )]
		pub dmcl: Option<Max500Text>,
	}
	
	impl OrganisationIdentification38 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref dmcl_value) = self.dmcl { if let Err(e) = dmcl_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PartyIdentification248Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PartyIdentification248Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Lgl", skip_serializing_if = "Option::is_none") )]
		pub lgl: Option<LegalPersonIdentification1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ntrl", skip_serializing_if = "Option::is_none") )]
		pub ntrl: Option<NaturalPersonIdentification3>,
	}
	
	impl PartyIdentification248Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref lgl_value) = self.lgl { if let Err(e) = lgl_value.validate() { return Err(e); } }
			if let Some(ref ntrl_value) = self.ntrl { if let Err(e) = ntrl_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PlusOrMinusIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct PlusOrMinusIndicator {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub plus_or_minus_indicator: bool,
	}
	
	impl PlusOrMinusIndicator {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PortfolioCode3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PortfolioCode3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<Max52Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoPrtfl", skip_serializing_if = "Option::is_none") )]
		pub no_prtfl: Option<NotApplicable1Code>,
	}
	
	impl PortfolioCode3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref no_prtfl_value) = self.no_prtfl { if let Err(e) = no_prtfl_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PortfolioCode5Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PortfolioCode5Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtfl", skip_serializing_if = "Option::is_none") )]
		pub prtfl: Option<PortfolioIdentification3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoPrtfl", skip_serializing_if = "Option::is_none") )]
		pub no_prtfl: Option<NotApplicable1Code>,
	}
	
	impl PortfolioCode5Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref prtfl_value) = self.prtfl { if let Err(e) = prtfl_value.validate() { return Err(e); } }
			if let Some(ref no_prtfl_value) = self.no_prtfl { if let Err(e) = no_prtfl_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PortfolioIdentification3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PortfolioIdentification3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: Max52Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtflTxXmptn", skip_serializing_if = "Option::is_none") )]
		pub prtfl_tx_xmptn: Option<bool>,
	}
	
	impl PortfolioIdentification3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// QuantityOrTerm1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct QuantityOrTerm1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchdlPrd", skip_serializing_if = "Option::is_none") )]
		pub schdl_prd: Option<Vec<Schedule10>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Term", skip_serializing_if = "Option::is_none") )]
		pub term: Option<QuantityTerm1>,
	}
	
	impl QuantityOrTerm1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref schdl_prd_vec) = self.schdl_prd { for item in schdl_prd_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref term_value) = self.term { if let Err(e) = term_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// QuantityTerm1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct QuantityTerm1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Qty", skip_serializing_if = "Option::is_none") )]
		pub qty: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none") )]
		pub unit_of_measr: Option<UnitOfMeasure8Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val", skip_serializing_if = "Option::is_none") )]
		pub val: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TmUnit", skip_serializing_if = "Option::is_none") )]
		pub tm_unit: Option<Frequency19Code>,
	}
	
	impl QuantityTerm1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref unit_of_measr_value) = self.unit_of_measr { if let Err(e) = unit_of_measr_value.validate() { return Err(e); } }
			if let Some(ref tm_unit_value) = self.tm_unit { if let Err(e) = tm_unit_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ReportPeriodActivity1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum ReportPeriodActivity1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "NOTX") )]
		CodeNOTX,
	}
	
	impl ReportPeriodActivity1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Schedule10 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Schedule10 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Qty") )]
		pub qty: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none") )]
		pub unit_of_measr: Option<UnitOfMeasure8Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UadjstdFctvDt") )]
		pub uadjstd_fctv_dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UadjstdEndDt", skip_serializing_if = "Option::is_none") )]
		pub uadjstd_end_dt: Option<String>,
	}
	
	impl Schedule10 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref unit_of_measr_value) = self.unit_of_measr { if let Err(e) = unit_of_measr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Schedule11 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Schedule11 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "UadjstdFctvDt") )]
		pub uadjstd_fctv_dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UadjstdEndDt", skip_serializing_if = "Option::is_none") )]
		pub uadjstd_end_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: AmountAndDirection106,
	}
	
	impl Schedule11 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.amt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// StatisticsPerCounterparty16Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct StatisticsPerCounterparty16Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
		pub data_set_actn: Option<ReportPeriodActivity1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rpt", skip_serializing_if = "Option::is_none") )]
		pub rpt: Option<DetailedStatisticsPerCounterparty17>,
	}
	
	impl StatisticsPerCounterparty16Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref data_set_actn_value) = self.data_set_actn { if let Err(e) = data_set_actn_value.validate() { return Err(e); } }
			if let Some(ref rpt_value) = self.rpt { if let Err(e) = rpt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SupplementaryData1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SupplementaryData1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none") )]
		pub plc_and_nm: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Envlp") )]
		pub envlp: SupplementaryDataEnvelope1,
	}
	
	impl SupplementaryData1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref plc_and_nm_value) = self.plc_and_nm { if let Err(e) = plc_and_nm_value.validate() { return Err(e); } }
			if let Err(e) = self.envlp.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// SupplementaryDataEnvelope1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SupplementaryDataEnvelope1 {
	}
	
	impl SupplementaryDataEnvelope1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// TradeTransactionIdentification24 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct TradeTransactionIdentification24 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none") )]
		pub tech_rcrd_id: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ActnTp", skip_serializing_if = "Option::is_none") )]
		pub actn_tp: Option<TransactionOperationType10Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgTmStmp", skip_serializing_if = "Option::is_none") )]
		pub rptg_tm_stmp: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DerivEvtTp", skip_serializing_if = "Option::is_none") )]
		pub deriv_evt_tp: Option<DerivativeEventType3Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DerivEvtTmStmp", skip_serializing_if = "Option::is_none") )]
		pub deriv_evt_tm_stmp: Option<DateAndDateTime2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none") )]
		pub othr_ctr_pty: Option<PartyIdentification248Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnqIdr", skip_serializing_if = "Option::is_none") )]
		pub unq_idr: Option<UniqueTransactionIdentifier2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MstrAgrmt", skip_serializing_if = "Option::is_none") )]
		pub mstr_agrmt: Option<MasterAgreement8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CollPrtflCd", skip_serializing_if = "Option::is_none") )]
		pub coll_prtfl_cd: Option<CollateralPortfolioCode5Choice>,
	}
	
	impl TradeTransactionIdentification24 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tech_rcrd_id_value) = self.tech_rcrd_id { if let Err(e) = tech_rcrd_id_value.validate() { return Err(e); } }
			if let Some(ref actn_tp_value) = self.actn_tp { if let Err(e) = actn_tp_value.validate() { return Err(e); } }
			if let Some(ref deriv_evt_tp_value) = self.deriv_evt_tp { if let Err(e) = deriv_evt_tp_value.validate() { return Err(e); } }
			if let Some(ref deriv_evt_tm_stmp_value) = self.deriv_evt_tm_stmp { if let Err(e) = deriv_evt_tm_stmp_value.validate() { return Err(e); } }
			if let Some(ref othr_ctr_pty_value) = self.othr_ctr_pty { if let Err(e) = othr_ctr_pty_value.validate() { return Err(e); } }
			if let Some(ref unq_idr_value) = self.unq_idr { if let Err(e) = unq_idr_value.validate() { return Err(e); } }
			if let Some(ref mstr_agrmt_value) = self.mstr_agrmt { if let Err(e) = mstr_agrmt_value.validate() { return Err(e); } }
			if let Some(ref coll_prtfl_cd_value) = self.coll_prtfl_cd { if let Err(e) = coll_prtfl_cd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TransactionOperationType10Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum TransactionOperationType10Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "COMP") )]
		CodeCOMP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CORR") )]
		CodeCORR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EROR") )]
		CodeEROR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MODI") )]
		CodeMODI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NEWT") )]
		CodeNEWT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "POSC") )]
		CodePOSC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REVI") )]
		CodeREVI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TERM") )]
		CodeTERM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VALU") )]
		CodeVALU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MARU") )]
		CodeMARU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRTO") )]
		CodePRTO,
	}
	
	impl TransactionOperationType10Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// TrueFalseIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct TrueFalseIndicator {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub true_false_indicator: bool,
	}
	
	impl TrueFalseIndicator {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// UTIIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct UTIIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub uti_identifier: String,
	}
	
	impl UTIIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9]{18}[0-9]{2}[A-Z0-9]{0,32}").unwrap();
			if !pattern.is_match(&self.uti_identifier) {
				return Err(ValidationError::new(1005, "uti_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// UniqueTransactionIdentifier2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct UniqueTransactionIdentifier2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnqTxIdr", skip_serializing_if = "Option::is_none") )]
		pub unq_tx_idr: Option<UTIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification175>,
	}
	
	impl UniqueTransactionIdentifier2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref unq_tx_idr_value) = self.unq_tx_idr { if let Err(e) = unq_tx_idr_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// UnitOfMeasure8Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct UnitOfMeasure8Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalUnitOfMeasure1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification175>,
	}
	
	impl UnitOfMeasure8Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
}