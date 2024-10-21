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

pub mod iso20022 {
	use regex::Regex;
	use crate::common::*;
	#[cfg(feature = "derive_serde")]
	use serde::{Deserialize, Serialize};
	
	
	// ActiveCurrencyAnd24AmountSimpleType ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveCurrencyAnd24AmountSimpleType {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_currency_and24_amount_simple_type: f64,
	}
	
	impl ActiveCurrencyAnd24AmountSimpleType {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.active_currency_and24_amount_simple_type < 0.000000 {
				return Err(ValidationError::new(1003, "active_currency_and24_amount_simple_type is less than the minimum value of 0.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ActiveCurrencyAnd24Amount ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ActiveCurrencyAnd24Amount {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
		pub ccy: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub value: f64,
	}
	
	impl ActiveCurrencyAnd24Amount {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ActiveCurrencyAndAmountSimpleType ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveCurrencyAndAmountSimpleType {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_currency_and_amount_simple_type: f64,
	}
	
	impl ActiveCurrencyAndAmountSimpleType {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.active_currency_and_amount_simple_type < 0.000000 {
				return Err(ValidationError::new(1003, "active_currency_and_amount_simple_type is less than the minimum value of 0.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ActiveCurrencyAndAmount ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ActiveCurrencyAndAmount {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
		pub ccy: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub value: f64,
	}
	
	impl ActiveCurrencyAndAmount {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ActiveCurrencyCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// CCPClearedProductReportV01 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CCPClearedProductReportV01 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrdPdct") )]
		pub clrd_pdct: Vec<ClearedProduct1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl CCPClearedProductReportV01 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			for item in &self.clrd_pdct { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// ClearedProduct1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ClearedProduct1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradgVn") )]
		pub tradg_vn: Vec<MICIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CCPPdctId") )]
		pub ccp_pdct_id: GenericIdentification168,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UvrslPdctId", skip_serializing_if = "Option::is_none") )]
		pub uvrsl_pdct_id: Option<GenericIdentification168>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pdct") )]
		pub pdct: Product1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OpnIntrst") )]
		pub opn_intrst: OpenInterest1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TrdsClrd", skip_serializing_if = "Option::is_none") )]
		pub trds_clrd: Option<f64>,
	}
	
	impl ClearedProduct1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			for item in &self.tradg_vn { if let Err(e) = item.validate() { return Err(e); } }
			if let Err(e) = self.ccp_pdct_id.validate() { return Err(e); }
			if let Some(ref uvrsl_pdct_id_value) = self.uvrsl_pdct_id { if let Err(e) = uvrsl_pdct_id_value.validate() { return Err(e); } }
			if let Err(e) = self.pdct.validate() { return Err(e); }
			if let Err(e) = self.opn_intrst.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// ContractSize1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ContractSize1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "LotSz") )]
		pub lot_sz: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Unit", skip_serializing_if = "Option::is_none") )]
		pub unit: Option<UnitOfMeasure5Choice>,
	}
	
	impl ContractSize1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref unit_value) = self.unit { if let Err(e) = unit_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DefinedAttributes1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DefinedAttributes1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "QtyDfndAttrbts", skip_serializing_if = "Option::is_none") )]
		pub qty_dfnd_attrbts: Option<FinancialInstrumentAttributes89>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ValDfndAttrbts", skip_serializing_if = "Option::is_none") )]
		pub val_dfnd_attrbts: Option<FinancialInstrumentAttributes90>,
	}
	
	impl DefinedAttributes1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref qty_dfnd_attrbts_value) = self.qty_dfnd_attrbts { if let Err(e) = qty_dfnd_attrbts_value.validate() { return Err(e); } }
			if let Some(ref val_dfnd_attrbts_value) = self.val_dfnd_attrbts { if let Err(e) = val_dfnd_attrbts_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Derivative3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Derivative3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DerivClssfctn") )]
		pub deriv_clssfctn: DerivativeClassification1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DerivUndrlygLeg") )]
		pub deriv_undrlyg_leg: Vec<DerivativeUnderlyingLeg1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OptnAttrbts", skip_serializing_if = "Option::is_none") )]
		pub optn_attrbts: Option<Option14>,
	}
	
	impl Derivative3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.deriv_clssfctn.validate() { return Err(e); }
			for item in &self.deriv_undrlyg_leg { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref optn_attrbts_value) = self.optn_attrbts { if let Err(e) = optn_attrbts_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DerivativeClassification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DerivativeClassification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AsstClss") )]
		pub asst_clss: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct", skip_serializing_if = "Option::is_none") )]
		pub base_pdct: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubCmmdty", skip_serializing_if = "Option::is_none") )]
		pub sub_cmmdty: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxTp", skip_serializing_if = "Option::is_none") )]
		pub tx_tp: Option<Max35Text>,
	}
	
	impl DerivativeClassification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.asst_clss.validate() { return Err(e); }
			if let Some(ref base_pdct_value) = self.base_pdct { if let Err(e) = base_pdct_value.validate() { return Err(e); } }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			if let Some(ref sub_cmmdty_value) = self.sub_cmmdty { if let Err(e) = sub_cmmdty_value.validate() { return Err(e); } }
			if let Some(ref tx_tp_value) = self.tx_tp { if let Err(e) = tx_tp_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DerivativeUnderlyingLeg1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DerivativeUnderlyingLeg1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctAttrbts") )]
		pub ctrct_attrbts: FinancialInstrumentAttributes88,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DfndAttrbts", skip_serializing_if = "Option::is_none") )]
		pub dfnd_attrbts: Option<DefinedAttributes1Choice>,
	}
	
	impl DerivativeUnderlyingLeg1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.ctrct_attrbts.validate() { return Err(e); }
			if let Some(ref dfnd_attrbts_value) = self.dfnd_attrbts { if let Err(e) = dfnd_attrbts_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ExoticOptionStyle1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum ExoticOptionStyle1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "BINA") )]
		CodeBINA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DIGI") )]
		CodeDIGI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NOTO") )]
		CodeNOTO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VANI") )]
		CodeVANI,
	}
	
	impl ExoticOptionStyle1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// FinancialInstrument59 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FinancialInstrument59 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: ISINOct2015Identifier,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
		pub issr: LEIIdentifier,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sctr", skip_serializing_if = "Option::is_none") )]
		pub sctr: Option<String>,
	}
	
	impl FinancialInstrument59 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Err(e) = self.issr.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// FinancialInstrumentAttributes88 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FinancialInstrumentAttributes88 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctTerm", skip_serializing_if = "Option::is_none") )]
		pub ctrct_term: Option<InterestRateContractTerm1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Stdstn", skip_serializing_if = "Option::is_none") )]
		pub stdstn: Option<Vec<Standardisation1Code>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtFrqcy") )]
		pub pmt_frqcy: Frequency11Code,
	}
	
	impl FinancialInstrumentAttributes88 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ctrct_term_value) = self.ctrct_term { if let Err(e) = ctrct_term_value.validate() { return Err(e); } }
			if let Some(ref stdstn_vec) = self.stdstn { for item in stdstn_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Err(e) = self.pmt_frqcy.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// FinancialInstrumentAttributes89 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FinancialInstrumentAttributes89 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctSz") )]
		pub ctrct_sz: ContractSize1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DlvryTp") )]
		pub dlvry_tp: PhysicalTransferType4Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygId") )]
		pub undrlyg_id: GenericIdentification165,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PricCcy") )]
		pub pric_ccy: ActiveCurrencyCode,
	}
	
	impl FinancialInstrumentAttributes89 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.ctrct_sz.validate() { return Err(e); }
			if let Err(e) = self.dlvry_tp.validate() { return Err(e); }
			if let Err(e) = self.undrlyg_id.validate() { return Err(e); }
			if let Err(e) = self.pric_ccy.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// FinancialInstrumentAttributes90 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FinancialInstrumentAttributes90 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ntnl", skip_serializing_if = "Option::is_none") )]
		pub ntnl: Option<ActiveCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnitVal") )]
		pub unit_val: ActiveCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IndxId") )]
		pub indx_id: GenericIdentification168,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IndxUnit") )]
		pub indx_unit: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstRateTerms", skip_serializing_if = "Option::is_none") )]
		pub intrst_rate_terms: Option<InterestComputationMethod2Code>,
	}
	
	impl FinancialInstrumentAttributes90 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ntnl_value) = self.ntnl { if let Err(e) = ntnl_value.validate() { return Err(e); } }
			if let Err(e) = self.unit_val.validate() { return Err(e); }
			if let Err(e) = self.indx_id.validate() { return Err(e); }
			if let Err(e) = self.indx_unit.validate() { return Err(e); }
			if let Some(ref intrst_rate_terms_value) = self.intrst_rate_terms { if let Err(e) = intrst_rate_terms_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Frequency11Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum Frequency11Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
		CodeYEAR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DAIL") )]
		CodeDAIL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
		CodeMNTH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EXPI") )]
		CodeEXPI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OVNG") )]
		CodeOVNG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QURT") )]
		CodeQURT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MIAN") )]
		CodeMIAN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UPFR") )]
		CodeUPFR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WEEK") )]
		CodeWEEK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CRED") )]
		CodeCRED,
	}
	
	impl Frequency11Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// GeneralCollateral2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GeneralCollateral2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ElgblFinInstrmId") )]
		pub elgbl_fin_instrm_id: Vec<Max35Text>,
	}
	
	impl GeneralCollateral2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			for item in &self.elgbl_fin_instrm_id { if let Err(e) = item.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GenericIdentification165 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericIdentification165 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max256Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<SchemeIdentificationType1Code>,
	}
	
	impl GenericIdentification165 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GenericIdentification168 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericIdentification168 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max256Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<Max35Text>,
	}
	
	impl GenericIdentification168 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// GenericIdentification36 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericIdentification36 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
		pub issr: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<Max35Text>,
	}
	
	impl GenericIdentification36 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Err(e) = self.issr.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ISINOct2015Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISINOct2015Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub isin_oct2015_identifier: String,
	}
	
	impl ISINOct2015Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(&self.isin_oct2015_identifier) {
				return Err(ValidationError::new(1005, "isin_oct2015_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// InterestComputationMethod2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum InterestComputationMethod2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "A001") )]
		CodeA001,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A002") )]
		CodeA002,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A003") )]
		CodeA003,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A004") )]
		CodeA004,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A005") )]
		CodeA005,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A006") )]
		CodeA006,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A007") )]
		CodeA007,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A008") )]
		CodeA008,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A009") )]
		CodeA009,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A010") )]
		CodeA010,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A011") )]
		CodeA011,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A012") )]
		CodeA012,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A013") )]
		CodeA013,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A014") )]
		CodeA014,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NARR") )]
		CodeNARR,
	}
	
	impl InterestComputationMethod2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// InterestRateContractTerm1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct InterestRateContractTerm1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Unit") )]
		pub unit: RateBasis1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
		pub val: f64,
	}
	
	impl InterestRateContractTerm1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.unit.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// LEIIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// MICIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct MICIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub mic_identifier: String,
	}
	
	impl MICIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(&self.mic_identifier) {
				return Err(ValidationError::new(1005, "mic_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max140Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max256Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max256Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max256_text: String,
	}
	
	impl Max256Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max256_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max256_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max256_text.chars().count() > 256 {
				return Err(ValidationError::new(1002, "max256_text exceeds the maximum length of 256".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max350Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// NonNegativeNumber ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct NonNegativeNumber {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub non_negative_number: f64,
	}
	
	impl NonNegativeNumber {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.non_negative_number < 0.000000 {
				return Err(ValidationError::new(1003, "non_negative_number is less than the minimum value of 0.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Number ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// OpenInterest1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OpenInterest1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "GrssNtnlAmt") )]
		pub grss_ntnl_amt: ActiveCurrencyAnd24Amount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfLots", skip_serializing_if = "Option::is_none") )]
		pub nb_of_lots: Option<f64>,
	}
	
	impl OpenInterest1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.grss_ntnl_amt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// Option14 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Option14 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "XprtnStyle") )]
		pub xprtn_style: Vec<OptionStyle5Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OptnStyle", skip_serializing_if = "Option::is_none") )]
		pub optn_style: Option<ExoticOptionStyle1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OptnTp", skip_serializing_if = "Option::is_none") )]
		pub optn_tp: Option<OptionType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BrrrInd", skip_serializing_if = "Option::is_none") )]
		pub brrr_ind: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EvtTp", skip_serializing_if = "Option::is_none") )]
		pub evt_tp: Option<OptionEvent2>,
	}
	
	impl Option14 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			for item in &self.xprtn_style { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref optn_style_value) = self.optn_style { if let Err(e) = optn_style_value.validate() { return Err(e); } }
			if let Some(ref optn_tp_value) = self.optn_tp { if let Err(e) = optn_tp_value.validate() { return Err(e); } }
			if let Some(ref evt_tp_value) = self.evt_tp { if let Err(e) = evt_tp_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// OptionEvent2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OptionEvent2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: OptionEventType1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc") )]
		pub desc: Max35Text,
	}
	
	impl OptionEvent2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Err(e) = self.desc.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// OptionEventType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OptionEventType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<OptionEventType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification36>,
	}
	
	impl OptionEventType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// OptionEventType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum OptionEventType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLST") )]
		CodeCLST,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CONF") )]
		CodeCONF,
		#[cfg_attr( feature = "derive_serde", serde(rename = "KNIN") )]
		CodeKNIN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "KNOC") )]
		CodeKNOC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TRIG") )]
		CodeTRIG,
	}
	
	impl OptionEventType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// OptionStyle5Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum OptionStyle5Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "AMER") )]
		CodeAMER,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ASIA") )]
		CodeASIA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BERM") )]
		CodeBERM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EURO") )]
		CodeEURO,
	}
	
	impl OptionStyle5Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// OptionType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum OptionType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CALL") )]
		CodeCALL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PUTO") )]
		CodePUTO,
	}
	
	impl OptionType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PhysicalTransferType4Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum PhysicalTransferType4Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "PHYS") )]
		CodePHYS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OPTL") )]
		CodeOPTL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CASH") )]
		CodeCASH,
	}
	
	impl PhysicalTransferType4Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PositiveNumber ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct PositiveNumber {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub positive_number: f64,
	}
	
	impl PositiveNumber {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.positive_number < 1.000000 {
				return Err(ValidationError::new(1003, "positive_number is less than the minimum value of 1.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Product1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Product1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Deriv", skip_serializing_if = "Option::is_none") )]
		pub deriv: Option<Derivative3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesFincgTx", skip_serializing_if = "Option::is_none") )]
		pub scties_fincg_tx: Option<RepurchaseAgreement3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Scty", skip_serializing_if = "Option::is_none") )]
		pub scty: Option<FinancialInstrument59>,
	}
	
	impl Product1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref deriv_value) = self.deriv { if let Err(e) = deriv_value.validate() { return Err(e); } }
			if let Some(ref scties_fincg_tx_value) = self.scties_fincg_tx { if let Err(e) = scties_fincg_tx_value.validate() { return Err(e); } }
			if let Some(ref scty_value) = self.scty { if let Err(e) = scty_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ProductClassification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ProductClassification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AsstClss") )]
		pub asst_clss: Max35Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct", skip_serializing_if = "Option::is_none") )]
		pub base_pdct: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubCmmdty", skip_serializing_if = "Option::is_none") )]
		pub sub_cmmdty: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxTp", skip_serializing_if = "Option::is_none") )]
		pub tx_tp: Option<Max35Text>,
	}
	
	impl ProductClassification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.asst_clss.validate() { return Err(e); }
			if let Some(ref base_pdct_value) = self.base_pdct { if let Err(e) = base_pdct_value.validate() { return Err(e); } }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			if let Some(ref sub_cmmdty_value) = self.sub_cmmdty { if let Err(e) = sub_cmmdty_value.validate() { return Err(e); } }
			if let Some(ref tx_tp_value) = self.tx_tp { if let Err(e) = tx_tp_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// RateBasis1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum RateBasis1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "DAYS") )]
		CodeDAYS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
		CodeMNTH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WEEK") )]
		CodeWEEK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
		CodeYEAR,
	}
	
	impl RateBasis1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// RepurchaseAgreement3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RepurchaseAgreement3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PdctClssfctn") )]
		pub pdct_clssfctn: ProductClassification1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RpAgrmtTp") )]
		pub rp_agrmt_tp: RepurchaseAgreementType1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TrptyAgt", skip_serializing_if = "Option::is_none") )]
		pub trpty_agt: Option<LEIIdentifier>,
	}
	
	impl RepurchaseAgreement3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.pdct_clssfctn.validate() { return Err(e); }
			if let Err(e) = self.rp_agrmt_tp.validate() { return Err(e); }
			if let Some(ref trpty_agt_value) = self.trpty_agt { if let Err(e) = trpty_agt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// RepurchaseAgreementType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct RepurchaseAgreementType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "SpcfcColl", skip_serializing_if = "Option::is_none") )]
		pub spcfc_coll: Option<SpecificCollateral2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GnlColl", skip_serializing_if = "Option::is_none") )]
		pub gnl_coll: Option<GeneralCollateral2>,
	}
	
	impl RepurchaseAgreementType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref spcfc_coll_value) = self.spcfc_coll { if let Err(e) = spcfc_coll_value.validate() { return Err(e); } }
			if let Some(ref gnl_coll_value) = self.gnl_coll { if let Err(e) = gnl_coll_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SNA2008SectorIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct SNA2008SectorIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub sna2008_sector_identifier: String,
	}
	
	impl SNA2008SectorIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// SchemeIdentificationType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum SchemeIdentificationType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "MARG") )]
		CodeMARG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "COLL") )]
		CodeCOLL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "POSI") )]
		CodePOSI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLIM") )]
		CodeCLIM,
	}
	
	impl SchemeIdentificationType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// SpecificCollateral2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SpecificCollateral2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmId") )]
		pub fin_instrm_id: FinancialInstrument59,
	}
	
	impl SpecificCollateral2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.fin_instrm_id.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// Standardisation1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum Standardisation1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "FLEX") )]
		CodeFLEX,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NSTA") )]
		CodeNSTA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "STAN") )]
		CodeSTAN,
	}
	
	impl Standardisation1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// SupplementaryData1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SupplementaryDataEnvelope1 {
	}
	
	impl SupplementaryDataEnvelope1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// TrueFalseIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// UnitOfMeasure5Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct UnitOfMeasure5Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<UnitOfMeasure8Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification36>,
	}
	
	impl UnitOfMeasure5Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// UnitOfMeasure8Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum UnitOfMeasure8Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "KILO") )]
		CodeKILO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "KMET") )]
		CodeKMET,
		#[cfg_attr( feature = "derive_serde", serde(rename = "KWDC") )]
		CodeKWDC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "KWHO") )]
		CodeKWHO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "KWHC") )]
		CodeKWHC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "KMOC") )]
		CodeKMOC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "KWMC") )]
		CodeKWMC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "KWYC") )]
		CodeKWYC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LITR") )]
		CodeLITR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MWDC") )]
		CodeMWDC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MWHO") )]
		CodeMWHO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MWHC") )]
		CodeMWHC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MWMC") )]
		CodeMWMC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MMOC") )]
		CodeMMOC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MWYC") )]
		CodeMWYC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "METR") )]
		CodeMETR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TONE") )]
		CodeTONE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MILE") )]
		CodeMILE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MILI") )]
		CodeMILI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MMET") )]
		CodeMMET,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MIBA") )]
		CodeMIBA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MBTU") )]
		CodeMBTU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PIEC") )]
		CodePIEC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PUND") )]
		CodePUND,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PWRD") )]
		CodePWRD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SHAS") )]
		CodeSHAS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SCMT") )]
		CodeSCMT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SQFO") )]
		CodeSQFO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SQIN") )]
		CodeSQIN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SQKI") )]
		CodeSQKI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SMET") )]
		CodeSMET,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SQMI") )]
		CodeSQMI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SMIL") )]
		CodeSMIL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SQYA") )]
		CodeSQYA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "THMS") )]
		CodeTHMS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TONS") )]
		CodeTONS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TOCD") )]
		CodeTOCD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OZTR") )]
		CodeOZTR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "USGA") )]
		CodeUSGA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UCWT") )]
		CodeUCWT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "USOU") )]
		CodeUSOU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "USPI") )]
		CodeUSPI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "USQA") )]
		CodeUSQA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "YARD") )]
		CodeYARD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ACRE") )]
		CodeACRE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ALOW") )]
		CodeALOW,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ACCY") )]
		CodeACCY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ARES") )]
		CodeARES,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BARL") )]
		CodeBARL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BCUF") )]
		CodeBCUF,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BDFT") )]
		CodeBDFT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BUSL") )]
		CodeBUSL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CELI") )]
		CodeCELI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CMET") )]
		CodeCMET,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CEER") )]
		CodeCEER,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLRT") )]
		CodeCLRT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CBME") )]
		CodeCBME,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DAYS") )]
		CodeDAYS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DGEU") )]
		CodeDGEU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DMET") )]
		CodeDMET,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ENVC") )]
		CodeENVC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ENVO") )]
		CodeENVO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FOOT") )]
		CodeFOOT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GGEU") )]
		CodeGGEU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GBGA") )]
		CodeGBGA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GBOU") )]
		CodeGBOU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GBPI") )]
		CodeGBPI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GBQA") )]
		CodeGBQA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GRAM") )]
		CodeGRAM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HECT") )]
		CodeHECT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HUWG") )]
		CodeHUWG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INCH") )]
		CodeINCH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IPNT") )]
		CodeIPNT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FUTU") )]
		CodeFUTU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "USTN") )]
		CodeUSTN,
	}
	
	impl UnitOfMeasure8Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
}