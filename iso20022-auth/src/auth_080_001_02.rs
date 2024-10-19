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

#![allow(unused_imports)]

pub mod iso20022 {
	use regex::Regex;
	use crate::common::*;
	#[cfg(feature = "derive_serde")]
	use serde::{Deserialize, Serialize};
	
	
	// ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_or_historic_currency_and20_decimal_amount_simple_type: f64,
	}
	
	impl ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.active_or_historic_currency_and20_decimal_amount_simple_type < 0.000000 {
				return Err(ValidationError::new(1003, "active_or_historic_currency_and20_decimal_amount_simple_type is less than the minimum value of 0.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ActiveOrHistoricCurrencyAnd20DecimalAmount ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ActiveOrHistoricCurrencyAnd20DecimalAmount {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
		pub ccy: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub value: f64,
	}
	
	impl ActiveOrHistoricCurrencyAnd20DecimalAmount {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ActiveOrHistoricCurrencyAndAmountSimpleType ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub active_or_historic_currency_and_amount_simple_type: f64,
	}
	
	impl ActiveOrHistoricCurrencyAndAmountSimpleType {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.active_or_historic_currency_and_amount_simple_type < 0.000000 {
				return Err(ValidationError::new(1003, "active_or_historic_currency_and_amount_simple_type is less than the minimum value of 0.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ActiveOrHistoricCurrencyAndAmount ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ActiveOrHistoricCurrencyAndAmount {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
		pub ccy: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub value: f64,
	}
	
	impl ActiveOrHistoricCurrencyAndAmount {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ActiveOrHistoricCurrencyCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// AgreementType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AgreementType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<ExternalAgreementType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl AgreementType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AgreementType2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// AgriculturalCommodityDairy1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AgriculturalCommodityDairy1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType20Code,
	}
	
	impl AgriculturalCommodityDairy1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// AgriculturalCommodityForestry1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AgriculturalCommodityForestry1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType21Code,
	}
	
	impl AgriculturalCommodityForestry1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// AgriculturalCommodityGrain2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AgriculturalCommodityGrain2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType5Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSubPdct") )]
		pub addtl_sub_pdct: AssetClassDetailedSubProductType30Code,
	}
	
	impl AgriculturalCommodityGrain2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			if let Err(e) = self.addtl_sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// AgriculturalCommodityLiveStock1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AgriculturalCommodityLiveStock1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType22Code,
	}
	
	impl AgriculturalCommodityLiveStock1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// AgriculturalCommodityOilSeed1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AgriculturalCommodityOilSeed1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSubPdct") )]
		pub addtl_sub_pdct: AssetClassDetailedSubProductType1Code,
	}
	
	impl AgriculturalCommodityOilSeed1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			if let Err(e) = self.addtl_sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// AgriculturalCommodityOliveOil2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AgriculturalCommodityOliveOil2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType3Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSubPdct") )]
		pub addtl_sub_pdct: AssetClassDetailedSubProductType29Code,
	}
	
	impl AgriculturalCommodityOliveOil2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			if let Err(e) = self.addtl_sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// AgriculturalCommodityOther1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AgriculturalCommodityOther1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType49Code,
	}
	
	impl AgriculturalCommodityOther1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// AgriculturalCommodityPotato1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AgriculturalCommodityPotato1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType45Code,
	}
	
	impl AgriculturalCommodityPotato1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// AgriculturalCommoditySeafood1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AgriculturalCommoditySeafood1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType23Code,
	}
	
	impl AgriculturalCommoditySeafood1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// AgriculturalCommoditySoft1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AgriculturalCommoditySoft1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType2Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSubPdct") )]
		pub addtl_sub_pdct: AssetClassDetailedSubProductType2Code,
	}
	
	impl AgriculturalCommoditySoft1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			if let Err(e) = self.addtl_sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// AmountAndDirection107 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AmountAndDirection107 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveOrHistoricCurrencyAnd20DecimalAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sgn", skip_serializing_if = "Option::is_none") )]
		pub sgn: Option<bool>,
	}
	
	impl AmountAndDirection107 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.amt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// AmountAndDirection53 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AmountAndDirection53 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
		pub amt: ActiveOrHistoricCurrencyAndAmount,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sgn", skip_serializing_if = "Option::is_none") )]
		pub sgn: Option<bool>,
	}
	
	impl AmountAndDirection53 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.amt.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// AnyBICDec2014Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// AssetClassCommodity5Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AssetClassCommodity5Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Agrcltrl", skip_serializing_if = "Option::is_none") )]
		pub agrcltrl: Option<AssetClassCommodityAgricultural5Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nrgy", skip_serializing_if = "Option::is_none") )]
		pub nrgy: Option<AssetClassCommodityEnergy2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Envttl", skip_serializing_if = "Option::is_none") )]
		pub envttl: Option<AssetClassCommodityEnvironmental2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Frtlzr", skip_serializing_if = "Option::is_none") )]
		pub frtlzr: Option<AssetClassCommodityFertilizer3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Frght", skip_serializing_if = "Option::is_none") )]
		pub frght: Option<AssetClassCommodityFreight3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IndstrlPdct", skip_serializing_if = "Option::is_none") )]
		pub indstrl_pdct: Option<AssetClassCommodityIndustrialProduct1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Metl", skip_serializing_if = "Option::is_none") )]
		pub metl: Option<AssetClassCommodityMetal1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrC10", skip_serializing_if = "Option::is_none") )]
		pub othr_c10: Option<AssetClassCommodityOtherC102Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ppr", skip_serializing_if = "Option::is_none") )]
		pub ppr: Option<AssetClassCommodityPaper3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Plprpln", skip_serializing_if = "Option::is_none") )]
		pub plprpln: Option<AssetClassCommodityPolypropylene3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Infltn", skip_serializing_if = "Option::is_none") )]
		pub infltn: Option<AssetClassCommodityInflation1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MultiCmmdtyExtc", skip_serializing_if = "Option::is_none") )]
		pub multi_cmmdty_extc: Option<AssetClassCommodityMultiCommodityExotic1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OffclEcnmcSttstcs", skip_serializing_if = "Option::is_none") )]
		pub offcl_ecnmc_sttstcs: Option<AssetClassCommodityOfficialEconomicStatistics1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<AssetClassCommodityOther1>,
	}
	
	impl AssetClassCommodity5Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref agrcltrl_value) = self.agrcltrl { if let Err(e) = agrcltrl_value.validate() { return Err(e); } }
			if let Some(ref nrgy_value) = self.nrgy { if let Err(e) = nrgy_value.validate() { return Err(e); } }
			if let Some(ref envttl_value) = self.envttl { if let Err(e) = envttl_value.validate() { return Err(e); } }
			if let Some(ref frtlzr_value) = self.frtlzr { if let Err(e) = frtlzr_value.validate() { return Err(e); } }
			if let Some(ref frght_value) = self.frght { if let Err(e) = frght_value.validate() { return Err(e); } }
			if let Some(ref indstrl_pdct_value) = self.indstrl_pdct { if let Err(e) = indstrl_pdct_value.validate() { return Err(e); } }
			if let Some(ref metl_value) = self.metl { if let Err(e) = metl_value.validate() { return Err(e); } }
			if let Some(ref othr_c10_value) = self.othr_c10 { if let Err(e) = othr_c10_value.validate() { return Err(e); } }
			if let Some(ref ppr_value) = self.ppr { if let Err(e) = ppr_value.validate() { return Err(e); } }
			if let Some(ref plprpln_value) = self.plprpln { if let Err(e) = plprpln_value.validate() { return Err(e); } }
			if let Some(ref infltn_value) = self.infltn { if let Err(e) = infltn_value.validate() { return Err(e); } }
			if let Some(ref multi_cmmdty_extc_value) = self.multi_cmmdty_extc { if let Err(e) = multi_cmmdty_extc_value.validate() { return Err(e); } }
			if let Some(ref offcl_ecnmc_sttstcs_value) = self.offcl_ecnmc_sttstcs { if let Err(e) = offcl_ecnmc_sttstcs_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityAgricultural5Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AssetClassCommodityAgricultural5Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "GrnOilSeed", skip_serializing_if = "Option::is_none") )]
		pub grn_oil_seed: Option<AgriculturalCommodityOilSeed1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Soft", skip_serializing_if = "Option::is_none") )]
		pub soft: Option<AgriculturalCommoditySoft1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ptt", skip_serializing_if = "Option::is_none") )]
		pub ptt: Option<AgriculturalCommodityPotato1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OlvOil", skip_serializing_if = "Option::is_none") )]
		pub olv_oil: Option<AgriculturalCommodityOliveOil2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dairy", skip_serializing_if = "Option::is_none") )]
		pub dairy: Option<AgriculturalCommodityDairy1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Frstry", skip_serializing_if = "Option::is_none") )]
		pub frstry: Option<AgriculturalCommodityForestry1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sfd", skip_serializing_if = "Option::is_none") )]
		pub sfd: Option<AgriculturalCommoditySeafood1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LiveStock", skip_serializing_if = "Option::is_none") )]
		pub live_stock: Option<AgriculturalCommodityLiveStock1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Grn", skip_serializing_if = "Option::is_none") )]
		pub grn: Option<AgriculturalCommodityGrain2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<AgriculturalCommodityOther1>,
	}
	
	impl AssetClassCommodityAgricultural5Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref grn_oil_seed_value) = self.grn_oil_seed { if let Err(e) = grn_oil_seed_value.validate() { return Err(e); } }
			if let Some(ref soft_value) = self.soft { if let Err(e) = soft_value.validate() { return Err(e); } }
			if let Some(ref ptt_value) = self.ptt { if let Err(e) = ptt_value.validate() { return Err(e); } }
			if let Some(ref olv_oil_value) = self.olv_oil { if let Err(e) = olv_oil_value.validate() { return Err(e); } }
			if let Some(ref dairy_value) = self.dairy { if let Err(e) = dairy_value.validate() { return Err(e); } }
			if let Some(ref frstry_value) = self.frstry { if let Err(e) = frstry_value.validate() { return Err(e); } }
			if let Some(ref sfd_value) = self.sfd { if let Err(e) = sfd_value.validate() { return Err(e); } }
			if let Some(ref live_stock_value) = self.live_stock { if let Err(e) = live_stock_value.validate() { return Err(e); } }
			if let Some(ref grn_value) = self.grn { if let Err(e) = grn_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityEnergy2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AssetClassCommodityEnergy2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Elctrcty", skip_serializing_if = "Option::is_none") )]
		pub elctrcty: Option<EnergyCommodityElectricity1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtrlGas", skip_serializing_if = "Option::is_none") )]
		pub ntrl_gas: Option<EnergyCommodityNaturalGas2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Oil", skip_serializing_if = "Option::is_none") )]
		pub oil: Option<EnergyCommodityOil2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Coal", skip_serializing_if = "Option::is_none") )]
		pub coal: Option<EnergyCommodityCoal1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrNrgy", skip_serializing_if = "Option::is_none") )]
		pub intr_nrgy: Option<EnergyCommodityInterEnergy1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RnwblNrgy", skip_serializing_if = "Option::is_none") )]
		pub rnwbl_nrgy: Option<EnergyCommodityRenewableEnergy1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LghtEnd", skip_serializing_if = "Option::is_none") )]
		pub lght_end: Option<EnergyCommodityLightEnd1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dstllts", skip_serializing_if = "Option::is_none") )]
		pub dstllts: Option<EnergyCommodityDistillates1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<EnergyCommodityOther1>,
	}
	
	impl AssetClassCommodityEnergy2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref elctrcty_value) = self.elctrcty { if let Err(e) = elctrcty_value.validate() { return Err(e); } }
			if let Some(ref ntrl_gas_value) = self.ntrl_gas { if let Err(e) = ntrl_gas_value.validate() { return Err(e); } }
			if let Some(ref oil_value) = self.oil { if let Err(e) = oil_value.validate() { return Err(e); } }
			if let Some(ref coal_value) = self.coal { if let Err(e) = coal_value.validate() { return Err(e); } }
			if let Some(ref intr_nrgy_value) = self.intr_nrgy { if let Err(e) = intr_nrgy_value.validate() { return Err(e); } }
			if let Some(ref rnwbl_nrgy_value) = self.rnwbl_nrgy { if let Err(e) = rnwbl_nrgy_value.validate() { return Err(e); } }
			if let Some(ref lght_end_value) = self.lght_end { if let Err(e) = lght_end_value.validate() { return Err(e); } }
			if let Some(ref dstllts_value) = self.dstllts { if let Err(e) = dstllts_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityEnvironmental2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AssetClassCommodityEnvironmental2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Emssns", skip_serializing_if = "Option::is_none") )]
		pub emssns: Option<EnvironmentalCommodityEmission2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Wthr", skip_serializing_if = "Option::is_none") )]
		pub wthr: Option<EnvironmentalCommodityWeather1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CrbnRltd", skip_serializing_if = "Option::is_none") )]
		pub crbn_rltd: Option<EnvironmentalCommodityCarbonRelated1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<EnvironmentCommodityOther1>,
	}
	
	impl AssetClassCommodityEnvironmental2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref emssns_value) = self.emssns { if let Err(e) = emssns_value.validate() { return Err(e); } }
			if let Some(ref wthr_value) = self.wthr { if let Err(e) = wthr_value.validate() { return Err(e); } }
			if let Some(ref crbn_rltd_value) = self.crbn_rltd { if let Err(e) = crbn_rltd_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityFertilizer3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AssetClassCommodityFertilizer3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ammn", skip_serializing_if = "Option::is_none") )]
		pub ammn: Option<FertilizerCommodityAmmonia1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DmmnmPhspht", skip_serializing_if = "Option::is_none") )]
		pub dmmnm_phspht: Option<FertilizerCommodityDiammoniumPhosphate1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ptsh", skip_serializing_if = "Option::is_none") )]
		pub ptsh: Option<FertilizerCommodityPotash1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Slphr", skip_serializing_if = "Option::is_none") )]
		pub slphr: Option<FertilizerCommoditySulphur1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Urea", skip_serializing_if = "Option::is_none") )]
		pub urea: Option<FertilizerCommodityUrea1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UreaAndAmmnmNtrt", skip_serializing_if = "Option::is_none") )]
		pub urea_and_ammnm_ntrt: Option<FertilizerCommodityUreaAndAmmoniumNitrate1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<FertilizerCommodityOther1>,
	}
	
	impl AssetClassCommodityFertilizer3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ammn_value) = self.ammn { if let Err(e) = ammn_value.validate() { return Err(e); } }
			if let Some(ref dmmnm_phspht_value) = self.dmmnm_phspht { if let Err(e) = dmmnm_phspht_value.validate() { return Err(e); } }
			if let Some(ref ptsh_value) = self.ptsh { if let Err(e) = ptsh_value.validate() { return Err(e); } }
			if let Some(ref slphr_value) = self.slphr { if let Err(e) = slphr_value.validate() { return Err(e); } }
			if let Some(ref urea_value) = self.urea { if let Err(e) = urea_value.validate() { return Err(e); } }
			if let Some(ref urea_and_ammnm_ntrt_value) = self.urea_and_ammnm_ntrt { if let Err(e) = urea_and_ammnm_ntrt_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityFreight3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AssetClassCommodityFreight3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dry", skip_serializing_if = "Option::is_none") )]
		pub dry: Option<FreightCommodityDry2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Wet", skip_serializing_if = "Option::is_none") )]
		pub wet: Option<FreightCommodityWet2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CntnrShip", skip_serializing_if = "Option::is_none") )]
		pub cntnr_ship: Option<FreightCommodityContainerShip1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<FreightCommodityOther1>,
	}
	
	impl AssetClassCommodityFreight3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref dry_value) = self.dry { if let Err(e) = dry_value.validate() { return Err(e); } }
			if let Some(ref wet_value) = self.wet { if let Err(e) = wet_value.validate() { return Err(e); } }
			if let Some(ref cntnr_ship_value) = self.cntnr_ship { if let Err(e) = cntnr_ship_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityIndustrialProduct1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AssetClassCommodityIndustrialProduct1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cnstrctn", skip_serializing_if = "Option::is_none") )]
		pub cnstrctn: Option<IndustrialProductCommodityConstruction1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Manfctg", skip_serializing_if = "Option::is_none") )]
		pub manfctg: Option<IndustrialProductCommodityManufacturing1>,
	}
	
	impl AssetClassCommodityIndustrialProduct1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cnstrctn_value) = self.cnstrctn { if let Err(e) = cnstrctn_value.validate() { return Err(e); } }
			if let Some(ref manfctg_value) = self.manfctg { if let Err(e) = manfctg_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityInflation1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AssetClassCommodityInflation1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType12Code,
	}
	
	impl AssetClassCommodityInflation1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityMetal1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AssetClassCommodityMetal1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NonPrcs", skip_serializing_if = "Option::is_none") )]
		pub non_prcs: Option<MetalCommodityNonPrecious1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prcs", skip_serializing_if = "Option::is_none") )]
		pub prcs: Option<MetalCommodityPrecious1>,
	}
	
	impl AssetClassCommodityMetal1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref non_prcs_value) = self.non_prcs { if let Err(e) = non_prcs_value.validate() { return Err(e); } }
			if let Some(ref prcs_value) = self.prcs { if let Err(e) = prcs_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityMultiCommodityExotic1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AssetClassCommodityMultiCommodityExotic1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType13Code,
	}
	
	impl AssetClassCommodityMultiCommodityExotic1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityOfficialEconomicStatistics1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AssetClassCommodityOfficialEconomicStatistics1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType14Code,
	}
	
	impl AssetClassCommodityOfficialEconomicStatistics1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityOther1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AssetClassCommodityOther1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType15Code,
	}
	
	impl AssetClassCommodityOther1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityOtherC102Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AssetClassCommodityOtherC102Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dlvrbl", skip_serializing_if = "Option::is_none") )]
		pub dlvrbl: Option<OtherC10CommodityDeliverable2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NonDlvrbl", skip_serializing_if = "Option::is_none") )]
		pub non_dlvrbl: Option<OtherC10CommodityNonDeliverable2>,
	}
	
	impl AssetClassCommodityOtherC102Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref dlvrbl_value) = self.dlvrbl { if let Err(e) = dlvrbl_value.validate() { return Err(e); } }
			if let Some(ref non_dlvrbl_value) = self.non_dlvrbl { if let Err(e) = non_dlvrbl_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityPaper3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AssetClassCommodityPaper3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CntnrBrd", skip_serializing_if = "Option::is_none") )]
		pub cntnr_brd: Option<PaperCommodityContainerBoard1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nwsprnt", skip_serializing_if = "Option::is_none") )]
		pub nwsprnt: Option<PaperCommodityNewsprint1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pulp", skip_serializing_if = "Option::is_none") )]
		pub pulp: Option<PaperCommodityPulp1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RcvrdPpr", skip_serializing_if = "Option::is_none") )]
		pub rcvrd_ppr: Option<PaperCommodityRecoveredPaper1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<PaperCommodityRecoveredPaper2>,
	}
	
	impl AssetClassCommodityPaper3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cntnr_brd_value) = self.cntnr_brd { if let Err(e) = cntnr_brd_value.validate() { return Err(e); } }
			if let Some(ref nwsprnt_value) = self.nwsprnt { if let Err(e) = nwsprnt_value.validate() { return Err(e); } }
			if let Some(ref pulp_value) = self.pulp { if let Err(e) = pulp_value.validate() { return Err(e); } }
			if let Some(ref rcvrd_ppr_value) = self.rcvrd_ppr { if let Err(e) = rcvrd_ppr_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityPolypropylene3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct AssetClassCommodityPolypropylene3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Plstc", skip_serializing_if = "Option::is_none") )]
		pub plstc: Option<PolypropyleneCommodityPlastic1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<PolypropyleneCommodityOther1>,
	}
	
	impl AssetClassCommodityPolypropylene3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref plstc_value) = self.plstc { if let Err(e) = plstc_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetClassDetailedSubProductType10Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassDetailedSubProductType10Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ALUM") )]
		CodeALUM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ALUA") )]
		CodeALUA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CBLT") )]
		CodeCBLT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "COPR") )]
		CodeCOPR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IRON") )]
		CodeIRON,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MOLY") )]
		CodeMOLY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NASC") )]
		CodeNASC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NICK") )]
		CodeNICK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "STEL") )]
		CodeSTEL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TINN") )]
		CodeTINN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ZINC") )]
		CodeZINC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEAD") )]
		CodeLEAD,
	}
	
	impl AssetClassDetailedSubProductType10Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassDetailedSubProductType11Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassDetailedSubProductType11Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "GOLD") )]
		CodeGOLD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PLDM") )]
		CodePLDM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PTNM") )]
		CodePTNM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SLVR") )]
		CodeSLVR,
	}
	
	impl AssetClassDetailedSubProductType11Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassDetailedSubProductType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassDetailedSubProductType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "FWHT") )]
		CodeFWHT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SOYB") )]
		CodeSOYB,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RPSD") )]
		CodeRPSD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CORN") )]
		CodeCORN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RICE") )]
		CodeRICE,
	}
	
	impl AssetClassDetailedSubProductType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassDetailedSubProductType29Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassDetailedSubProductType29Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "LAMP") )]
		CodeLAMP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
	}
	
	impl AssetClassDetailedSubProductType29Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassDetailedSubProductType2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassDetailedSubProductType2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ROBU") )]
		CodeROBU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CCOA") )]
		CodeCCOA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BRWN") )]
		CodeBRWN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WHSG") )]
		CodeWHSG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
	}
	
	impl AssetClassDetailedSubProductType2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassDetailedSubProductType30Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassDetailedSubProductType30Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "MWHT") )]
		CodeMWHT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
	}
	
	impl AssetClassDetailedSubProductType30Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassDetailedSubProductType31Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassDetailedSubProductType31Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "GASP") )]
		CodeGASP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LNGG") )]
		CodeLNGG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NCGG") )]
		CodeNCGG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TTFG") )]
		CodeTTFG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NBPG") )]
		CodeNBPG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
	}
	
	impl AssetClassDetailedSubProductType31Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassDetailedSubProductType32Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassDetailedSubProductType32Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "BAKK") )]
		CodeBAKK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BDSL") )]
		CodeBDSL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BRNT") )]
		CodeBRNT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BRNX") )]
		CodeBRNX,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CNDA") )]
		CodeCNDA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "COND") )]
		CodeCOND,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DSEL") )]
		CodeDSEL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DUBA") )]
		CodeDUBA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ESPO") )]
		CodeESPO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ETHA") )]
		CodeETHA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FUEL") )]
		CodeFUEL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FOIL") )]
		CodeFOIL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GOIL") )]
		CodeGOIL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GSLN") )]
		CodeGSLN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HEAT") )]
		CodeHEAT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "JTFL") )]
		CodeJTFL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "KERO") )]
		CodeKERO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LLSO") )]
		CodeLLSO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MARS") )]
		CodeMARS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NAPH") )]
		CodeNAPH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NGLO") )]
		CodeNGLO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TAPI") )]
		CodeTAPI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WTIO") )]
		CodeWTIO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "URAL") )]
		CodeURAL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
	}
	
	impl AssetClassDetailedSubProductType32Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassDetailedSubProductType33Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassDetailedSubProductType33Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "DBCR") )]
		CodeDBCR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
	}
	
	impl AssetClassDetailedSubProductType33Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassDetailedSubProductType34Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassDetailedSubProductType34Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "TNKR") )]
		CodeTNKR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
	}
	
	impl AssetClassDetailedSubProductType34Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassDetailedSubProductType5Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassDetailedSubProductType5Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "BSLD") )]
		CodeBSLD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FITR") )]
		CodeFITR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PKLD") )]
		CodePKLD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OFFP") )]
		CodeOFFP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
	}
	
	impl AssetClassDetailedSubProductType5Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassDetailedSubProductType8Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassDetailedSubProductType8Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CERE") )]
		CodeCERE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ERUE") )]
		CodeERUE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EUAE") )]
		CodeEUAE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EUAA") )]
		CodeEUAA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
	}
	
	impl AssetClassDetailedSubProductType8Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassProductType11Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassProductType11Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHC") )]
		CodeOTHC,
	}
	
	impl AssetClassProductType11Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassProductType12Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassProductType12Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "INFL") )]
		CodeINFL,
	}
	
	impl AssetClassProductType12Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassProductType13Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassProductType13Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "MCEX") )]
		CodeMCEX,
	}
	
	impl AssetClassProductType13Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassProductType14Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassProductType14Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "OEST") )]
		CodeOEST,
	}
	
	impl AssetClassProductType14Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassProductType15Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassProductType15Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
	}
	
	impl AssetClassProductType15Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassProductType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassProductType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "AGRI") )]
		CodeAGRI,
	}
	
	impl AssetClassProductType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassProductType2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassProductType2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "NRGY") )]
		CodeNRGY,
	}
	
	impl AssetClassProductType2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassProductType3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassProductType3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ENVR") )]
		CodeENVR,
	}
	
	impl AssetClassProductType3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassProductType4Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassProductType4Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "FRGT") )]
		CodeFRGT,
	}
	
	impl AssetClassProductType4Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassProductType5Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassProductType5Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "FRTL") )]
		CodeFRTL,
	}
	
	impl AssetClassProductType5Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassProductType6Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassProductType6Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "INDP") )]
		CodeINDP,
	}
	
	impl AssetClassProductType6Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassProductType7Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassProductType7Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "METL") )]
		CodeMETL,
	}
	
	impl AssetClassProductType7Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassProductType8Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassProductType8Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "PAPR") )]
		CodePAPR,
	}
	
	impl AssetClassProductType8Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassProductType9Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassProductType9Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "POLY") )]
		CodePOLY,
	}
	
	impl AssetClassProductType9Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType10Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType10Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "EMIS") )]
		CodeEMIS,
	}
	
	impl AssetClassSubProductType10Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType15Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType15Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "NPRM") )]
		CodeNPRM,
	}
	
	impl AssetClassSubProductType15Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType16Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType16Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRME") )]
		CodePRME,
	}
	
	impl AssetClassSubProductType16Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType18Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType18Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "PLST") )]
		CodePLST,
	}
	
	impl AssetClassSubProductType18Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "GROS") )]
		CodeGROS,
	}
	
	impl AssetClassSubProductType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType20Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType20Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "DIRY") )]
		CodeDIRY,
	}
	
	impl AssetClassSubProductType20Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType21Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType21Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "FRST") )]
		CodeFRST,
	}
	
	impl AssetClassSubProductType21Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType22Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType22Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "LSTK") )]
		CodeLSTK,
	}
	
	impl AssetClassSubProductType22Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType23Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType23Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "SEAF") )]
		CodeSEAF,
	}
	
	impl AssetClassSubProductType23Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType24Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType24Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "COAL") )]
		CodeCOAL,
	}
	
	impl AssetClassSubProductType24Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType25Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType25Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "DIST") )]
		CodeDIST,
	}
	
	impl AssetClassSubProductType25Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType26Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType26Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "INRG") )]
		CodeINRG,
	}
	
	impl AssetClassSubProductType26Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType27Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType27Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "LGHT") )]
		CodeLGHT,
	}
	
	impl AssetClassSubProductType27Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType28Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType28Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "RNNG") )]
		CodeRNNG,
	}
	
	impl AssetClassSubProductType28Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType29Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType29Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CRBR") )]
		CodeCRBR,
	}
	
	impl AssetClassSubProductType29Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "SOFT") )]
		CodeSOFT,
	}
	
	impl AssetClassSubProductType2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType30Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType30Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "WTHR") )]
		CodeWTHR,
	}
	
	impl AssetClassSubProductType30Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType31Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType31Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "DRYF") )]
		CodeDRYF,
	}
	
	impl AssetClassSubProductType31Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType32Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType32Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "WETF") )]
		CodeWETF,
	}
	
	impl AssetClassSubProductType32Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType33Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType33Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CSTR") )]
		CodeCSTR,
	}
	
	impl AssetClassSubProductType33Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType34Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType34Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "MFTG") )]
		CodeMFTG,
	}
	
	impl AssetClassSubProductType34Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType35Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType35Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CBRD") )]
		CodeCBRD,
	}
	
	impl AssetClassSubProductType35Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType36Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType36Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "NSPT") )]
		CodeNSPT,
	}
	
	impl AssetClassSubProductType36Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType37Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType37Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "PULP") )]
		CodePULP,
	}
	
	impl AssetClassSubProductType37Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType38Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType38Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "RCVP") )]
		CodeRCVP,
	}
	
	impl AssetClassSubProductType38Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType39Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType39Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "AMMO") )]
		CodeAMMO,
	}
	
	impl AssetClassSubProductType39Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "OOLI") )]
		CodeOOLI,
	}
	
	impl AssetClassSubProductType3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType40Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType40Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "DAPH") )]
		CodeDAPH,
	}
	
	impl AssetClassSubProductType40Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType41Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType41Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "PTSH") )]
		CodePTSH,
	}
	
	impl AssetClassSubProductType41Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType42Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType42Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "SLPH") )]
		CodeSLPH,
	}
	
	impl AssetClassSubProductType42Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType43Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType43Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "UREA") )]
		CodeUREA,
	}
	
	impl AssetClassSubProductType43Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType44Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType44Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "UAAN") )]
		CodeUAAN,
	}
	
	impl AssetClassSubProductType44Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType45Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType45Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "POTA") )]
		CodePOTA,
	}
	
	impl AssetClassSubProductType45Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType46Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType46Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CSHP") )]
		CodeCSHP,
	}
	
	impl AssetClassSubProductType46Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType47Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType47Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "DLVR") )]
		CodeDLVR,
	}
	
	impl AssetClassSubProductType47Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType48Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType48Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "NDLV") )]
		CodeNDLV,
	}
	
	impl AssetClassSubProductType48Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType49Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType49Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
	}
	
	impl AssetClassSubProductType49Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType5Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType5Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "GRIN") )]
		CodeGRIN,
	}
	
	impl AssetClassSubProductType5Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType6Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType6Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ELEC") )]
		CodeELEC,
	}
	
	impl AssetClassSubProductType6Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType7Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType7Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "NGAS") )]
		CodeNGAS,
	}
	
	impl AssetClassSubProductType7Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassSubProductType8Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum AssetClassSubProductType8Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "OILP") )]
		CodeOILP,
	}
	
	impl AssetClassSubProductType8Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// BaseOneRate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct BaseOneRate {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub base_one_rate: f64,
	}
	
	impl BaseOneRate {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// BenchmarkCurveName10Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct BenchmarkCurveName10Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Indx", skip_serializing_if = "Option::is_none") )]
		pub indx: Option<BenchmarkCurveName3Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max350Text>,
	}
	
	impl BenchmarkCurveName10Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref indx_value) = self.indx { if let Err(e) = indx_value.validate() { return Err(e); } }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// BenchmarkCurveName3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum BenchmarkCurveName3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ESTR") )]
		CodeESTR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BBSW") )]
		CodeBBSW,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BUBO") )]
		CodeBUBO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CDOR") )]
		CodeCDOR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CIBO") )]
		CodeCIBO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EONA") )]
		CodeEONA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EONS") )]
		CodeEONS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EURI") )]
		CodeEURI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EUUS") )]
		CodeEUUS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EUCH") )]
		CodeEUCH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FUSW") )]
		CodeFUSW,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GCFR") )]
		CodeGCFR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ISDA") )]
		CodeISDA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "JIBA") )]
		CodeJIBA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LIBI") )]
		CodeLIBI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LIBO") )]
		CodeLIBO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MOSP") )]
		CodeMOSP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MAAA") )]
		CodeMAAA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NIBO") )]
		CodeNIBO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PFAN") )]
		CodePFAN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRBO") )]
		CodePRBO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "STBO") )]
		CodeSTBO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SWAP") )]
		CodeSWAP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TLBO") )]
		CodeTLBO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TIBO") )]
		CodeTIBO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TREA") )]
		CodeTREA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WIBO") )]
		CodeWIBO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SOFR") )]
		CodeSOFR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SONA") )]
		CodeSONA,
	}
	
	impl BenchmarkCurveName3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CFIOct2015Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct CFIOct2015Identifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub cfi_oct2015_identifier: String,
	}
	
	impl CFIOct2015Identifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{6,6}").unwrap();
			if !pattern.is_match(&self.cfi_oct2015_identifier) {
				return Err(ValidationError::new(1005, "cfi_oct2015_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// CashCompare3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CashCompare3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val", skip_serializing_if = "Option::is_none") )]
		pub val: Option<CompareAmountAndDirection2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HrcutOrMrgn", skip_serializing_if = "Option::is_none") )]
		pub hrcut_or_mrgn: Option<ComparePercentageRate3>,
	}
	
	impl CashCompare3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val_value) = self.val { if let Err(e) = val_value.validate() { return Err(e); } }
			if let Some(ref hrcut_or_mrgn_value) = self.hrcut_or_mrgn { if let Err(e) = hrcut_or_mrgn_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Cleared4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Cleared4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Clrd", skip_serializing_if = "Option::is_none") )]
		pub clrd: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NonClrd", skip_serializing_if = "Option::is_none") )]
		pub non_clrd: Option<NoReasonCode>,
	}
	
	impl Cleared4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref clrd_value) = self.clrd { if let Err(e) = clrd_value.validate() { return Err(e); } }
			if let Some(ref non_clrd_value) = self.non_clrd { if let Err(e) = non_clrd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CollateralDeliveryMethod1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum CollateralDeliveryMethod1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "SICA") )]
		CodeSICA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SIUR") )]
		CodeSIUR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TTCA") )]
		CodeTTCA,
	}
	
	impl CollateralDeliveryMethod1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CollateralMatchingCriteria6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CollateralMatchingCriteria6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "UncollsdFlg", skip_serializing_if = "Option::is_none") )]
		pub uncollsd_flg: Option<CompareTrueFalseIndicator3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NetXpsrCollstnInd", skip_serializing_if = "Option::is_none") )]
		pub net_xpsr_collstn_ind: Option<CompareTrueFalseIndicator3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CollValDt", skip_serializing_if = "Option::is_none") )]
		pub coll_val_dt: Option<CompareDate3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AsstTp", skip_serializing_if = "Option::is_none") )]
		pub asst_tp: Option<SecurityCommodityCash4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BsktIdr", skip_serializing_if = "Option::is_none") )]
		pub bskt_idr: Option<CompareSecurityIdentification4>,
	}
	
	impl CollateralMatchingCriteria6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref uncollsd_flg_value) = self.uncollsd_flg { if let Err(e) = uncollsd_flg_value.validate() { return Err(e); } }
			if let Some(ref net_xpsr_collstn_ind_value) = self.net_xpsr_collstn_ind { if let Err(e) = net_xpsr_collstn_ind_value.validate() { return Err(e); } }
			if let Some(ref coll_val_dt_value) = self.coll_val_dt { if let Err(e) = coll_val_dt_value.validate() { return Err(e); } }
			if let Some(ref asst_tp_value) = self.asst_tp { if let Err(e) = asst_tp_value.validate() { return Err(e); } }
			if let Some(ref bskt_idr_value) = self.bskt_idr { if let Err(e) = bskt_idr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CollateralQualityType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum CollateralQualityType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "INVG") )]
		CodeINVG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NIVG") )]
		CodeNIVG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NOTR") )]
		CodeNOTR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NOAP") )]
		CodeNOAP,
	}
	
	impl CollateralQualityType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CollateralRole1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum CollateralRole1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "GIVE") )]
		CodeGIVE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TAKE") )]
		CodeTAKE,
	}
	
	impl CollateralRole1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Commodity42 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Commodity42 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Clssfctn", skip_serializing_if = "Option::is_none") )]
		pub clssfctn: Option<CompareCommodityAssetClass3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Qty", skip_serializing_if = "Option::is_none") )]
		pub qty: Option<CompareDecimalNumber3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnitPric", skip_serializing_if = "Option::is_none") )]
		pub unit_pric: Option<CompareUnitPrice6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MktVal", skip_serializing_if = "Option::is_none") )]
		pub mkt_val: Option<CompareAmountAndDirection2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none") )]
		pub unit_of_measr: Option<CompareUnitOfMeasure3>,
	}
	
	impl Commodity42 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref clssfctn_value) = self.clssfctn { if let Err(e) = clssfctn_value.validate() { return Err(e); } }
			if let Some(ref qty_value) = self.qty { if let Err(e) = qty_value.validate() { return Err(e); } }
			if let Some(ref unit_pric_value) = self.unit_pric { if let Err(e) = unit_pric_value.validate() { return Err(e); } }
			if let Some(ref mkt_val_value) = self.mkt_val { if let Err(e) = mkt_val_value.validate() { return Err(e); } }
			if let Some(ref unit_of_measr_value) = self.unit_of_measr { if let Err(e) = unit_of_measr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareActiveOrHistoricCurrencyAndAmount3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareActiveOrHistoricCurrencyAndAmount3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<ActiveOrHistoricCurrencyAndAmount>,
	}
	
	impl CompareActiveOrHistoricCurrencyAndAmount3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareAgreementType2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareAgreementType2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<AgreementType1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<AgreementType1Choice>,
	}
	
	impl CompareAgreementType2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareAmountAndDirection1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareAmountAndDirection1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<AmountAndDirection53>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<AmountAndDirection53>,
	}
	
	impl CompareAmountAndDirection1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareAmountAndDirection2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareAmountAndDirection2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<AmountAndDirection53>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<AmountAndDirection53>,
	}
	
	impl CompareAmountAndDirection2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareBenchmarkCurveName3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareBenchmarkCurveName3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<BenchmarkCurveName10Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<BenchmarkCurveName10Choice>,
	}
	
	impl CompareBenchmarkCurveName3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareCFIIdentifier3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareCFIIdentifier3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<CFIOct2015Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<CFIOct2015Identifier>,
	}
	
	impl CompareCFIIdentifier3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareClearingStatus3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareClearingStatus3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<Cleared4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<Cleared4Choice>,
	}
	
	impl CompareClearingStatus3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareCollateralQualityType3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareCollateralQualityType3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<CollateralQualityType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<CollateralQualityType1Code>,
	}
	
	impl CompareCollateralQualityType3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareCommodityAssetClass3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareCommodityAssetClass3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<AssetClassCommodity5Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<AssetClassCommodity5Choice>,
	}
	
	impl CompareCommodityAssetClass3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareCounterpartySide2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareCounterpartySide2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<CollateralRole1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<CollateralRole1Code>,
	}
	
	impl CompareCounterpartySide2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareCountryCode3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareCountryCode3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<CountryCode>,
	}
	
	impl CompareCountryCode3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareDate3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareDate3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<String>,
	}
	
	impl CompareDate3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CompareDateTime3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareDateTime3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<String>,
	}
	
	impl CompareDateTime3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CompareDecimalNumber3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareDecimalNumber3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<f64>,
	}
	
	impl CompareDecimalNumber3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CompareDeliveryMethod3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareDeliveryMethod3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<CollateralDeliveryMethod1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<CollateralDeliveryMethod1Code>,
	}
	
	impl CompareDeliveryMethod3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareExposureType3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareExposureType3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<ExposureType10Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<ExposureType10Code>,
	}
	
	impl CompareExposureType3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareISINIdentifier4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareISINIdentifier4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<ISINOct2015Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<ISINOct2015Identifier>,
	}
	
	impl CompareISINIdentifier4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareInterestComputationMethod3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareInterestComputationMethod3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<InterestComputationMethodFormat6Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<InterestComputationMethodFormat6Choice>,
	}
	
	impl CompareInterestComputationMethod3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareInterestRate1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareInterestRate1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MrgnLnAmt", skip_serializing_if = "Option::is_none") )]
		pub mrgn_ln_amt: Option<CompareAmountAndDirection1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FxdIntrstRate", skip_serializing_if = "Option::is_none") )]
		pub fxd_intrst_rate: Option<ComparePercentageRate3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DayCntBsis", skip_serializing_if = "Option::is_none") )]
		pub day_cnt_bsis: Option<CompareInterestComputationMethod3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FltgIntrstRefRate", skip_serializing_if = "Option::is_none") )]
		pub fltg_intrst_ref_rate: Option<CompareBenchmarkCurveName3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FltgIntrstRateTermUnit", skip_serializing_if = "Option::is_none") )]
		pub fltg_intrst_rate_term_unit: Option<CompareRateBasis3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FltgIntrstRateTermVal", skip_serializing_if = "Option::is_none") )]
		pub fltg_intrst_rate_term_val: Option<CompareNumber5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FltgIntrstRatePmtFrqcyUnit", skip_serializing_if = "Option::is_none") )]
		pub fltg_intrst_rate_pmt_frqcy_unit: Option<CompareRateBasis3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FltgIntrstRatePmtFrqcyVal", skip_serializing_if = "Option::is_none") )]
		pub fltg_intrst_rate_pmt_frqcy_val: Option<CompareNumber5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FltgIntrstRateRstFrqcyUnit", skip_serializing_if = "Option::is_none") )]
		pub fltg_intrst_rate_rst_frqcy_unit: Option<CompareRateBasis3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FltgIntrstRateRstFrqcyVal", skip_serializing_if = "Option::is_none") )]
		pub fltg_intrst_rate_rst_frqcy_val: Option<CompareNumber6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BsisPtSprd", skip_serializing_if = "Option::is_none") )]
		pub bsis_pt_sprd: Option<CompareDecimalNumber3>,
	}
	
	impl CompareInterestRate1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref mrgn_ln_amt_value) = self.mrgn_ln_amt { if let Err(e) = mrgn_ln_amt_value.validate() { return Err(e); } }
			if let Some(ref fxd_intrst_rate_value) = self.fxd_intrst_rate { if let Err(e) = fxd_intrst_rate_value.validate() { return Err(e); } }
			if let Some(ref day_cnt_bsis_value) = self.day_cnt_bsis { if let Err(e) = day_cnt_bsis_value.validate() { return Err(e); } }
			if let Some(ref fltg_intrst_ref_rate_value) = self.fltg_intrst_ref_rate { if let Err(e) = fltg_intrst_ref_rate_value.validate() { return Err(e); } }
			if let Some(ref fltg_intrst_rate_term_unit_value) = self.fltg_intrst_rate_term_unit { if let Err(e) = fltg_intrst_rate_term_unit_value.validate() { return Err(e); } }
			if let Some(ref fltg_intrst_rate_term_val_value) = self.fltg_intrst_rate_term_val { if let Err(e) = fltg_intrst_rate_term_val_value.validate() { return Err(e); } }
			if let Some(ref fltg_intrst_rate_pmt_frqcy_unit_value) = self.fltg_intrst_rate_pmt_frqcy_unit { if let Err(e) = fltg_intrst_rate_pmt_frqcy_unit_value.validate() { return Err(e); } }
			if let Some(ref fltg_intrst_rate_pmt_frqcy_val_value) = self.fltg_intrst_rate_pmt_frqcy_val { if let Err(e) = fltg_intrst_rate_pmt_frqcy_val_value.validate() { return Err(e); } }
			if let Some(ref fltg_intrst_rate_rst_frqcy_unit_value) = self.fltg_intrst_rate_rst_frqcy_unit { if let Err(e) = fltg_intrst_rate_rst_frqcy_unit_value.validate() { return Err(e); } }
			if let Some(ref fltg_intrst_rate_rst_frqcy_val_value) = self.fltg_intrst_rate_rst_frqcy_val { if let Err(e) = fltg_intrst_rate_rst_frqcy_val_value.validate() { return Err(e); } }
			if let Some(ref bsis_pt_sprd_value) = self.bsis_pt_sprd { if let Err(e) = bsis_pt_sprd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareMICIdentifier3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareMICIdentifier3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<MICIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<MICIdentifier>,
	}
	
	impl CompareMICIdentifier3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareNumber5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareNumber5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<f64>,
	}
	
	impl CompareNumber5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CompareNumber6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareNumber6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<f64>,
	}
	
	impl CompareNumber6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CompareOrganisationIdentification6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareOrganisationIdentification6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<OrganisationIdentification15Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<OrganisationIdentification15Choice>,
	}
	
	impl CompareOrganisationIdentification6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareOrganisationIdentification7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareOrganisationIdentification7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<PartyIdentification236Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<PartyIdentification236Choice>,
	}
	
	impl CompareOrganisationIdentification7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ComparePercentageRate3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ComparePercentageRate3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<f64>,
	}
	
	impl ComparePercentageRate3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CompareRateBasis3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareRateBasis3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<RateBasis1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<RateBasis1Code>,
	}
	
	impl CompareRateBasis3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareReportingLevelType3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareReportingLevelType3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<ModificationLevel1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<ModificationLevel1Code>,
	}
	
	impl CompareReportingLevelType3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareSecuritiesLendingType3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareSecuritiesLendingType3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<SecuritiesLendingType3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<SecuritiesLendingType3Choice>,
	}
	
	impl CompareSecuritiesLendingType3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareSecurityIdentification4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareSecurityIdentification4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<SecurityIdentification26Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<SecurityIdentification26Choice>,
	}
	
	impl CompareSecurityIdentification4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareSpecialCollateral3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareSpecialCollateral3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<SpecialCollateral1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<SpecialCollateral1Code>,
	}
	
	impl CompareSpecialCollateral3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareTerminationOption3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareTerminationOption3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<RepoTerminationOption2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<RepoTerminationOption2Code>,
	}
	
	impl CompareTerminationOption3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareText2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareText2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<Max52Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<Max52Text>,
	}
	
	impl CompareText2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareTrueFalseIndicator3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareTrueFalseIndicator3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<bool>,
	}
	
	impl CompareTrueFalseIndicator3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CompareUnitOfMeasure3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareUnitOfMeasure3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<UnitOfMeasure11Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<UnitOfMeasure11Code>,
	}
	
	impl CompareUnitOfMeasure3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareUnitPrice6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CompareUnitPrice6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<SecuritiesTransactionPrice19Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<SecuritiesTransactionPrice19Choice>,
	}
	
	impl CompareUnitPrice6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CounterpartyMatchingCriteria4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct CounterpartyMatchingCriteria4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none") )]
		pub rptg_ctr_pty: Option<CompareOrganisationIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none") )]
		pub othr_ctr_pty: Option<CompareOrganisationIdentification7>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtySd", skip_serializing_if = "Option::is_none") )]
		pub ctr_pty_sd: Option<CompareCounterpartySide2>,
	}
	
	impl CounterpartyMatchingCriteria4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref rptg_ctr_pty_value) = self.rptg_ctr_pty { if let Err(e) = rptg_ctr_pty_value.validate() { return Err(e); } }
			if let Some(ref othr_ctr_pty_value) = self.othr_ctr_pty { if let Err(e) = othr_ctr_pty_value.validate() { return Err(e); } }
			if let Some(ref ctr_pty_sd_value) = self.ctr_pty_sd { if let Err(e) = ctr_pty_sd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CountryCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// DecimalNumber ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct DecimalNumber {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub decimal_number: f64,
	}
	
	impl DecimalNumber {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// EnergyCommodityCoal1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct EnergyCommodityCoal1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType2Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType24Code,
	}
	
	impl EnergyCommodityCoal1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// EnergyCommodityDistillates1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct EnergyCommodityDistillates1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType2Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType25Code,
	}
	
	impl EnergyCommodityDistillates1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// EnergyCommodityElectricity1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct EnergyCommodityElectricity1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType2Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType6Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSubPdct") )]
		pub addtl_sub_pdct: AssetClassDetailedSubProductType5Code,
	}
	
	impl EnergyCommodityElectricity1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			if let Err(e) = self.addtl_sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// EnergyCommodityInterEnergy1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct EnergyCommodityInterEnergy1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType2Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType26Code,
	}
	
	impl EnergyCommodityInterEnergy1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// EnergyCommodityLightEnd1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct EnergyCommodityLightEnd1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType2Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType27Code,
	}
	
	impl EnergyCommodityLightEnd1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// EnergyCommodityNaturalGas2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct EnergyCommodityNaturalGas2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType2Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType7Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSubPdct") )]
		pub addtl_sub_pdct: AssetClassDetailedSubProductType31Code,
	}
	
	impl EnergyCommodityNaturalGas2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			if let Err(e) = self.addtl_sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// EnergyCommodityOil2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct EnergyCommodityOil2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType2Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType8Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSubPdct") )]
		pub addtl_sub_pdct: AssetClassDetailedSubProductType32Code,
	}
	
	impl EnergyCommodityOil2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			if let Err(e) = self.addtl_sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// EnergyCommodityOther1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct EnergyCommodityOther1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType2Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType49Code,
	}
	
	impl EnergyCommodityOther1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// EnergyCommodityRenewableEnergy1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct EnergyCommodityRenewableEnergy1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType2Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType28Code,
	}
	
	impl EnergyCommodityRenewableEnergy1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// EnvironmentCommodityOther1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct EnvironmentCommodityOther1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType3Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType49Code,
	}
	
	impl EnvironmentCommodityOther1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// EnvironmentalCommodityCarbonRelated1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct EnvironmentalCommodityCarbonRelated1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType3Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType29Code,
	}
	
	impl EnvironmentalCommodityCarbonRelated1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// EnvironmentalCommodityEmission2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct EnvironmentalCommodityEmission2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType3Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType10Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSubPdct") )]
		pub addtl_sub_pdct: AssetClassDetailedSubProductType8Code,
	}
	
	impl EnvironmentalCommodityEmission2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			if let Err(e) = self.addtl_sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// EnvironmentalCommodityWeather1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct EnvironmentalCommodityWeather1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType3Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType30Code,
	}
	
	impl EnvironmentalCommodityWeather1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// ExposureType10Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum ExposureType10Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "SBSC") )]
		CodeSBSC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MGLD") )]
		CodeMGLD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SLEB") )]
		CodeSLEB,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REPO") )]
		CodeREPO,
	}
	
	impl ExposureType10Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ExternalAgreementType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ExternalSecuritiesLendingType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalSecuritiesLendingType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_securities_lending_type1_code: String,
	}
	
	impl ExternalSecuritiesLendingType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_securities_lending_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_securities_lending_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_securities_lending_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_securities_lending_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// FertilizerCommodityAmmonia1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FertilizerCommodityAmmonia1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType5Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType39Code,
	}
	
	impl FertilizerCommodityAmmonia1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// FertilizerCommodityDiammoniumPhosphate1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FertilizerCommodityDiammoniumPhosphate1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType5Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType40Code,
	}
	
	impl FertilizerCommodityDiammoniumPhosphate1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// FertilizerCommodityOther1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FertilizerCommodityOther1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType5Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType49Code,
	}
	
	impl FertilizerCommodityOther1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// FertilizerCommodityPotash1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FertilizerCommodityPotash1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType5Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType41Code,
	}
	
	impl FertilizerCommodityPotash1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// FertilizerCommoditySulphur1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FertilizerCommoditySulphur1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType5Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType42Code,
	}
	
	impl FertilizerCommoditySulphur1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// FertilizerCommodityUrea1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FertilizerCommodityUrea1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType5Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType43Code,
	}
	
	impl FertilizerCommodityUrea1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// FertilizerCommodityUreaAndAmmoniumNitrate1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FertilizerCommodityUreaAndAmmoniumNitrate1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType5Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType44Code,
	}
	
	impl FertilizerCommodityUreaAndAmmoniumNitrate1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// FreightCommodityContainerShip1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FreightCommodityContainerShip1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType4Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType46Code,
	}
	
	impl FreightCommodityContainerShip1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// FreightCommodityDry2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FreightCommodityDry2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType4Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType31Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSubPdct") )]
		pub addtl_sub_pdct: AssetClassDetailedSubProductType33Code,
	}
	
	impl FreightCommodityDry2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			if let Err(e) = self.addtl_sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// FreightCommodityOther1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FreightCommodityOther1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType4Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType49Code,
	}
	
	impl FreightCommodityOther1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// FreightCommodityWet2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct FreightCommodityWet2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType4Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType32Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSubPdct") )]
		pub addtl_sub_pdct: AssetClassDetailedSubProductType34Code,
	}
	
	impl FreightCommodityWet2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			if let Err(e) = self.addtl_sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// GenericIdentification175 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ISINOct2015Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// ISODate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// IndustrialProductCommodityConstruction1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct IndustrialProductCommodityConstruction1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType6Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType33Code>,
	}
	
	impl IndustrialProductCommodityConstruction1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// IndustrialProductCommodityManufacturing1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct IndustrialProductCommodityManufacturing1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType6Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType34Code>,
	}
	
	impl IndustrialProductCommodityManufacturing1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// InterestComputationMethod1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum InterestComputationMethod1Code {
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
	}
	
	impl InterestComputationMethod1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// InterestComputationMethodFormat6Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct InterestComputationMethodFormat6Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<InterestComputationMethod1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl InterestComputationMethodFormat6Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// LEIIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// LoanMatchingCriteria9 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct LoanMatchingCriteria9 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnqTradIdr", skip_serializing_if = "Option::is_none") )]
		pub unq_trad_idr: Option<CompareText2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none") )]
		pub termntn_dt: Option<CompareDate3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctTp", skip_serializing_if = "Option::is_none") )]
		pub ctrct_tp: Option<CompareExposureType3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSts", skip_serializing_if = "Option::is_none") )]
		pub clr_sts: Option<CompareClearingStatus3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrDtTm", skip_serializing_if = "Option::is_none") )]
		pub clr_dt_tm: Option<CompareDateTime3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CCP", skip_serializing_if = "Option::is_none") )]
		pub ccp: Option<CompareOrganisationIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradgVn", skip_serializing_if = "Option::is_none") )]
		pub tradg_vn: Option<CompareMICIdentifier3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MstrAgrmtTp", skip_serializing_if = "Option::is_none") )]
		pub mstr_agrmt_tp: Option<CompareAgreementType2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ExctnDtTm", skip_serializing_if = "Option::is_none") )]
		pub exctn_dt_tm: Option<CompareDateTime3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ValDt", skip_serializing_if = "Option::is_none") )]
		pub val_dt: Option<CompareDate3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none") )]
		pub mtrty_dt: Option<CompareDate3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MinNtcePrd", skip_serializing_if = "Option::is_none") )]
		pub min_ntce_prd: Option<CompareNumber5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EarlstCallBckDt", skip_serializing_if = "Option::is_none") )]
		pub earlst_call_bck_dt: Option<CompareDate3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GnlColl", skip_serializing_if = "Option::is_none") )]
		pub gnl_coll: Option<CompareSpecialCollateral3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DlvryByVal", skip_serializing_if = "Option::is_none") )]
		pub dlvry_by_val: Option<CompareTrueFalseIndicator3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CollDlvryMtd", skip_serializing_if = "Option::is_none") )]
		pub coll_dlvry_mtd: Option<CompareDeliveryMethod3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OpnTerm", skip_serializing_if = "Option::is_none") )]
		pub opn_term: Option<CompareTrueFalseIndicator3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TermntnOptn", skip_serializing_if = "Option::is_none") )]
		pub termntn_optn: Option<CompareTerminationOption3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FxdIntrstRate", skip_serializing_if = "Option::is_none") )]
		pub fxd_intrst_rate: Option<ComparePercentageRate3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DayCntBsis", skip_serializing_if = "Option::is_none") )]
		pub day_cnt_bsis: Option<CompareInterestComputationMethod3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FltgIntrstRefRate", skip_serializing_if = "Option::is_none") )]
		pub fltg_intrst_ref_rate: Option<CompareBenchmarkCurveName3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FltgIntrstRateTermUnit", skip_serializing_if = "Option::is_none") )]
		pub fltg_intrst_rate_term_unit: Option<CompareRateBasis3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FltgIntrstRateTermVal", skip_serializing_if = "Option::is_none") )]
		pub fltg_intrst_rate_term_val: Option<CompareNumber5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FltgIntrstRatePmtFrqcyUnit", skip_serializing_if = "Option::is_none") )]
		pub fltg_intrst_rate_pmt_frqcy_unit: Option<CompareRateBasis3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FltgIntrstRatePmtFrqcyVal", skip_serializing_if = "Option::is_none") )]
		pub fltg_intrst_rate_pmt_frqcy_val: Option<CompareNumber5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FltgIntrstRateRstFrqcyUnit", skip_serializing_if = "Option::is_none") )]
		pub fltg_intrst_rate_rst_frqcy_unit: Option<CompareRateBasis3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FltgIntrstRateRstFrqcyVal", skip_serializing_if = "Option::is_none") )]
		pub fltg_intrst_rate_rst_frqcy_val: Option<CompareNumber6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BsisPtSprd", skip_serializing_if = "Option::is_none") )]
		pub bsis_pt_sprd: Option<CompareDecimalNumber3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MrgnLnAttr", skip_serializing_if = "Option::is_none") )]
		pub mrgn_ln_attr: Option<Vec<CompareInterestRate1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrncplAmtValDtAmt", skip_serializing_if = "Option::is_none") )]
		pub prncpl_amt_val_dt_amt: Option<CompareActiveOrHistoricCurrencyAndAmount3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrncplAmtMtrtyDtAmt", skip_serializing_if = "Option::is_none") )]
		pub prncpl_amt_mtrty_dt_amt: Option<CompareActiveOrHistoricCurrencyAndAmount3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AsstTp", skip_serializing_if = "Option::is_none") )]
		pub asst_tp: Option<SecurityCommodity7Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LnVal", skip_serializing_if = "Option::is_none") )]
		pub ln_val: Option<CompareActiveOrHistoricCurrencyAndAmount3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FxdRbtRefRate", skip_serializing_if = "Option::is_none") )]
		pub fxd_rbt_ref_rate: Option<ComparePercentageRate3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FltgRbtRefRate", skip_serializing_if = "Option::is_none") )]
		pub fltg_rbt_ref_rate: Option<CompareBenchmarkCurveName3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FltgRbtRateTermUnit", skip_serializing_if = "Option::is_none") )]
		pub fltg_rbt_rate_term_unit: Option<CompareRateBasis3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FltgRbtRateTermVal", skip_serializing_if = "Option::is_none") )]
		pub fltg_rbt_rate_term_val: Option<CompareNumber6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FltgRbtRatePmtFrqcyUnit", skip_serializing_if = "Option::is_none") )]
		pub fltg_rbt_rate_pmt_frqcy_unit: Option<CompareRateBasis3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FltgRbtRatePmtFrqcyVal", skip_serializing_if = "Option::is_none") )]
		pub fltg_rbt_rate_pmt_frqcy_val: Option<CompareNumber6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FltgRbtRateRstFrqcyUnit", skip_serializing_if = "Option::is_none") )]
		pub fltg_rbt_rate_rst_frqcy_unit: Option<CompareRateBasis3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FltgRbtRateRstFrqcyVal", skip_serializing_if = "Option::is_none") )]
		pub fltg_rbt_rate_rst_frqcy_val: Option<CompareNumber6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RbtRateBsisPtSprd", skip_serializing_if = "Option::is_none") )]
		pub rbt_rate_bsis_pt_sprd: Option<CompareDecimalNumber3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FltgRateAdjstmnt", skip_serializing_if = "Option::is_none") )]
		pub fltg_rate_adjstmnt: Option<Vec<ComparePercentageRate3>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FltgRateAdjstmntDt", skip_serializing_if = "Option::is_none") )]
		pub fltg_rate_adjstmnt_dt: Option<Vec<CompareDate3>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LndgFee", skip_serializing_if = "Option::is_none") )]
		pub lndg_fee: Option<ComparePercentageRate3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OutsdngMrgnLnAmt", skip_serializing_if = "Option::is_none") )]
		pub outsdng_mrgn_ln_amt: Option<CompareActiveOrHistoricCurrencyAndAmount3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtMktValAmt", skip_serializing_if = "Option::is_none") )]
		pub shrt_mkt_val_amt: Option<CompareActiveOrHistoricCurrencyAndAmount3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LvlTp", skip_serializing_if = "Option::is_none") )]
		pub lvl_tp: Option<CompareReportingLevelType3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none") )]
		pub unit_of_measr: Option<CompareUnitOfMeasure3>,
	}
	
	impl LoanMatchingCriteria9 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref unq_trad_idr_value) = self.unq_trad_idr { if let Err(e) = unq_trad_idr_value.validate() { return Err(e); } }
			if let Some(ref termntn_dt_value) = self.termntn_dt { if let Err(e) = termntn_dt_value.validate() { return Err(e); } }
			if let Some(ref ctrct_tp_value) = self.ctrct_tp { if let Err(e) = ctrct_tp_value.validate() { return Err(e); } }
			if let Some(ref clr_sts_value) = self.clr_sts { if let Err(e) = clr_sts_value.validate() { return Err(e); } }
			if let Some(ref clr_dt_tm_value) = self.clr_dt_tm { if let Err(e) = clr_dt_tm_value.validate() { return Err(e); } }
			if let Some(ref ccp_value) = self.ccp { if let Err(e) = ccp_value.validate() { return Err(e); } }
			if let Some(ref tradg_vn_value) = self.tradg_vn { if let Err(e) = tradg_vn_value.validate() { return Err(e); } }
			if let Some(ref mstr_agrmt_tp_value) = self.mstr_agrmt_tp { if let Err(e) = mstr_agrmt_tp_value.validate() { return Err(e); } }
			if let Some(ref exctn_dt_tm_value) = self.exctn_dt_tm { if let Err(e) = exctn_dt_tm_value.validate() { return Err(e); } }
			if let Some(ref val_dt_value) = self.val_dt { if let Err(e) = val_dt_value.validate() { return Err(e); } }
			if let Some(ref mtrty_dt_value) = self.mtrty_dt { if let Err(e) = mtrty_dt_value.validate() { return Err(e); } }
			if let Some(ref min_ntce_prd_value) = self.min_ntce_prd { if let Err(e) = min_ntce_prd_value.validate() { return Err(e); } }
			if let Some(ref earlst_call_bck_dt_value) = self.earlst_call_bck_dt { if let Err(e) = earlst_call_bck_dt_value.validate() { return Err(e); } }
			if let Some(ref gnl_coll_value) = self.gnl_coll { if let Err(e) = gnl_coll_value.validate() { return Err(e); } }
			if let Some(ref dlvry_by_val_value) = self.dlvry_by_val { if let Err(e) = dlvry_by_val_value.validate() { return Err(e); } }
			if let Some(ref coll_dlvry_mtd_value) = self.coll_dlvry_mtd { if let Err(e) = coll_dlvry_mtd_value.validate() { return Err(e); } }
			if let Some(ref opn_term_value) = self.opn_term { if let Err(e) = opn_term_value.validate() { return Err(e); } }
			if let Some(ref termntn_optn_value) = self.termntn_optn { if let Err(e) = termntn_optn_value.validate() { return Err(e); } }
			if let Some(ref fxd_intrst_rate_value) = self.fxd_intrst_rate { if let Err(e) = fxd_intrst_rate_value.validate() { return Err(e); } }
			if let Some(ref day_cnt_bsis_value) = self.day_cnt_bsis { if let Err(e) = day_cnt_bsis_value.validate() { return Err(e); } }
			if let Some(ref fltg_intrst_ref_rate_value) = self.fltg_intrst_ref_rate { if let Err(e) = fltg_intrst_ref_rate_value.validate() { return Err(e); } }
			if let Some(ref fltg_intrst_rate_term_unit_value) = self.fltg_intrst_rate_term_unit { if let Err(e) = fltg_intrst_rate_term_unit_value.validate() { return Err(e); } }
			if let Some(ref fltg_intrst_rate_term_val_value) = self.fltg_intrst_rate_term_val { if let Err(e) = fltg_intrst_rate_term_val_value.validate() { return Err(e); } }
			if let Some(ref fltg_intrst_rate_pmt_frqcy_unit_value) = self.fltg_intrst_rate_pmt_frqcy_unit { if let Err(e) = fltg_intrst_rate_pmt_frqcy_unit_value.validate() { return Err(e); } }
			if let Some(ref fltg_intrst_rate_pmt_frqcy_val_value) = self.fltg_intrst_rate_pmt_frqcy_val { if let Err(e) = fltg_intrst_rate_pmt_frqcy_val_value.validate() { return Err(e); } }
			if let Some(ref fltg_intrst_rate_rst_frqcy_unit_value) = self.fltg_intrst_rate_rst_frqcy_unit { if let Err(e) = fltg_intrst_rate_rst_frqcy_unit_value.validate() { return Err(e); } }
			if let Some(ref fltg_intrst_rate_rst_frqcy_val_value) = self.fltg_intrst_rate_rst_frqcy_val { if let Err(e) = fltg_intrst_rate_rst_frqcy_val_value.validate() { return Err(e); } }
			if let Some(ref bsis_pt_sprd_value) = self.bsis_pt_sprd { if let Err(e) = bsis_pt_sprd_value.validate() { return Err(e); } }
			if let Some(ref mrgn_ln_attr_vec) = self.mrgn_ln_attr { for item in mrgn_ln_attr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref prncpl_amt_val_dt_amt_value) = self.prncpl_amt_val_dt_amt { if let Err(e) = prncpl_amt_val_dt_amt_value.validate() { return Err(e); } }
			if let Some(ref prncpl_amt_mtrty_dt_amt_value) = self.prncpl_amt_mtrty_dt_amt { if let Err(e) = prncpl_amt_mtrty_dt_amt_value.validate() { return Err(e); } }
			if let Some(ref asst_tp_value) = self.asst_tp { if let Err(e) = asst_tp_value.validate() { return Err(e); } }
			if let Some(ref ln_val_value) = self.ln_val { if let Err(e) = ln_val_value.validate() { return Err(e); } }
			if let Some(ref fxd_rbt_ref_rate_value) = self.fxd_rbt_ref_rate { if let Err(e) = fxd_rbt_ref_rate_value.validate() { return Err(e); } }
			if let Some(ref fltg_rbt_ref_rate_value) = self.fltg_rbt_ref_rate { if let Err(e) = fltg_rbt_ref_rate_value.validate() { return Err(e); } }
			if let Some(ref fltg_rbt_rate_term_unit_value) = self.fltg_rbt_rate_term_unit { if let Err(e) = fltg_rbt_rate_term_unit_value.validate() { return Err(e); } }
			if let Some(ref fltg_rbt_rate_term_val_value) = self.fltg_rbt_rate_term_val { if let Err(e) = fltg_rbt_rate_term_val_value.validate() { return Err(e); } }
			if let Some(ref fltg_rbt_rate_pmt_frqcy_unit_value) = self.fltg_rbt_rate_pmt_frqcy_unit { if let Err(e) = fltg_rbt_rate_pmt_frqcy_unit_value.validate() { return Err(e); } }
			if let Some(ref fltg_rbt_rate_pmt_frqcy_val_value) = self.fltg_rbt_rate_pmt_frqcy_val { if let Err(e) = fltg_rbt_rate_pmt_frqcy_val_value.validate() { return Err(e); } }
			if let Some(ref fltg_rbt_rate_rst_frqcy_unit_value) = self.fltg_rbt_rate_rst_frqcy_unit { if let Err(e) = fltg_rbt_rate_rst_frqcy_unit_value.validate() { return Err(e); } }
			if let Some(ref fltg_rbt_rate_rst_frqcy_val_value) = self.fltg_rbt_rate_rst_frqcy_val { if let Err(e) = fltg_rbt_rate_rst_frqcy_val_value.validate() { return Err(e); } }
			if let Some(ref rbt_rate_bsis_pt_sprd_value) = self.rbt_rate_bsis_pt_sprd { if let Err(e) = rbt_rate_bsis_pt_sprd_value.validate() { return Err(e); } }
			if let Some(ref fltg_rate_adjstmnt_vec) = self.fltg_rate_adjstmnt { for item in fltg_rate_adjstmnt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref fltg_rate_adjstmnt_dt_vec) = self.fltg_rate_adjstmnt_dt { for item in fltg_rate_adjstmnt_dt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref lndg_fee_value) = self.lndg_fee { if let Err(e) = lndg_fee_value.validate() { return Err(e); } }
			if let Some(ref outsdng_mrgn_ln_amt_value) = self.outsdng_mrgn_ln_amt { if let Err(e) = outsdng_mrgn_ln_amt_value.validate() { return Err(e); } }
			if let Some(ref shrt_mkt_val_amt_value) = self.shrt_mkt_val_amt { if let Err(e) = shrt_mkt_val_amt_value.validate() { return Err(e); } }
			if let Some(ref lvl_tp_value) = self.lvl_tp { if let Err(e) = lvl_tp_value.validate() { return Err(e); } }
			if let Some(ref unit_of_measr_value) = self.unit_of_measr { if let Err(e) = unit_of_measr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// LongFraction19DecimalNumber ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// MICIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// MasterAgreement7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct MasterAgreement7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: AgreementType2Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Vrsn", skip_serializing_if = "Option::is_none") )]
		pub vrsn: Option<Max50Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrMstrAgrmtDtls", skip_serializing_if = "Option::is_none") )]
		pub othr_mstr_agrmt_dtls: Option<Max350Text>,
	}
	
	impl MasterAgreement7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Some(ref vrsn_value) = self.vrsn { if let Err(e) = vrsn_value.validate() { return Err(e); } }
			if let Some(ref othr_mstr_agrmt_dtls_value) = self.othr_mstr_agrmt_dtls { if let Err(e) = othr_mstr_agrmt_dtls_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// MatchingCriteria10 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct MatchingCriteria10 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtyMtchgCrit", skip_serializing_if = "Option::is_none") )]
		pub ctr_pty_mtchg_crit: Option<CounterpartyMatchingCriteria4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LnMtchgCrit", skip_serializing_if = "Option::is_none") )]
		pub ln_mtchg_crit: Option<LoanMatchingCriteria9>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CollMtchgCrit", skip_serializing_if = "Option::is_none") )]
		pub coll_mtchg_crit: Option<CollateralMatchingCriteria6>,
	}
	
	impl MatchingCriteria10 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ctr_pty_mtchg_crit_value) = self.ctr_pty_mtchg_crit { if let Err(e) = ctr_pty_mtchg_crit_value.validate() { return Err(e); } }
			if let Some(ref ln_mtchg_crit_value) = self.ln_mtchg_crit { if let Err(e) = ln_mtchg_crit_value.validate() { return Err(e); } }
			if let Some(ref coll_mtchg_crit_value) = self.coll_mtchg_crit { if let Err(e) = coll_mtchg_crit_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Max105Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max15NumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max15NumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max15_numeric_text: String,
	}
	
	impl Max15NumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(&self.max15_numeric_text) {
				return Err(ValidationError::new(1005, "max15_numeric_text does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max350Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max5Number ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max5Number {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max5_number: f64,
	}
	
	impl Max5Number {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Max72Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// MetalCommodityNonPrecious1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct MetalCommodityNonPrecious1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType7Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType15Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSubPdct") )]
		pub addtl_sub_pdct: AssetClassDetailedSubProductType10Code,
	}
	
	impl MetalCommodityNonPrecious1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			if let Err(e) = self.addtl_sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// MetalCommodityPrecious1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct MetalCommodityPrecious1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType7Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType16Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSubPdct") )]
		pub addtl_sub_pdct: AssetClassDetailedSubProductType11Code,
	}
	
	impl MetalCommodityPrecious1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			if let Err(e) = self.addtl_sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// ModificationLevel1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum ModificationLevel1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "PSTN") )]
		CodePSTN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TCTN") )]
		CodeTCTN,
	}
	
	impl ModificationLevel1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// NaturalPersonIdentification2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// NoReasonCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum NoReasonCode {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "NORE") )]
		CodeNORE,
	}
	
	impl NoReasonCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// NotAvailable1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum NotAvailable1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "NTAV") )]
		CodeNTAV,
	}
	
	impl NotAvailable1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// NumberOfReportsPerStatus4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct NumberOfReportsPerStatus4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtldNbOfRpts") )]
		pub dtld_nb_of_rpts: Max15NumericText,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtldSts") )]
		pub dtld_sts: PairedReconciled3Code,
	}
	
	impl NumberOfReportsPerStatus4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.dtld_nb_of_rpts.validate() { return Err(e); }
			if let Err(e) = self.dtld_sts.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// OrganisationIdentification15Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// OtherC10CommodityDeliverable2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct OtherC10CommodityDeliverable2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType11Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType47Code>,
	}
	
	impl OtherC10CommodityDeliverable2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// OtherC10CommodityNonDeliverable2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct OtherC10CommodityNonDeliverable2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType11Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType48Code>,
	}
	
	impl OtherC10CommodityNonDeliverable2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PairedReconciled3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum PairedReconciled3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLRC") )]
		CodeCLRC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LNRC") )]
		CodeLNRC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PARD") )]
		CodePARD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RECO") )]
		CodeRECO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UNPR") )]
		CodeUNPR,
	}
	
	impl PairedReconciled3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PaperCommodityContainerBoard1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PaperCommodityContainerBoard1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType8Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType35Code>,
	}
	
	impl PaperCommodityContainerBoard1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaperCommodityNewsprint1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PaperCommodityNewsprint1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType8Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType36Code>,
	}
	
	impl PaperCommodityNewsprint1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaperCommodityPulp1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PaperCommodityPulp1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType8Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType37Code>,
	}
	
	impl PaperCommodityPulp1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaperCommodityRecoveredPaper1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PaperCommodityRecoveredPaper1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType8Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType38Code>,
	}
	
	impl PaperCommodityRecoveredPaper1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaperCommodityRecoveredPaper2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PaperCommodityRecoveredPaper2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType8Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType49Code>,
	}
	
	impl PaperCommodityRecoveredPaper2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PartyIdentification236Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PartyIdentification236Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Lgl", skip_serializing_if = "Option::is_none") )]
		pub lgl: Option<OrganisationIdentification15Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ntrl", skip_serializing_if = "Option::is_none") )]
		pub ntrl: Option<NaturalPersonIdentification2>,
	}
	
	impl PartyIdentification236Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref lgl_value) = self.lgl { if let Err(e) = lgl_value.validate() { return Err(e); } }
			if let Some(ref ntrl_value) = self.ntrl { if let Err(e) = ntrl_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PercentageRate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct PercentageRate {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub percentage_rate: f64,
	}
	
	impl PercentageRate {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PlusOrMinusIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// PolypropyleneCommodityOther1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PolypropyleneCommodityOther1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType9Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct") )]
		pub sub_pdct: AssetClassSubProductType49Code,
	}
	
	impl PolypropyleneCommodityOther1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Err(e) = self.sub_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// PolypropyleneCommodityPlastic1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct PolypropyleneCommodityPlastic1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType9Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType18Code>,
	}
	
	impl PolypropyleneCommodityPlastic1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PriceStatus1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum PriceStatus1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "PNDG") )]
		CodePNDG,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NOAP") )]
		CodeNOAP,
	}
	
	impl PriceStatus1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// RateBasis1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
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
	
	
	// ReconciliationMatchedStatus9Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ReconciliationMatchedStatus9Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Mtchd", skip_serializing_if = "Option::is_none") )]
		pub mtchd: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NotMtchd", skip_serializing_if = "Option::is_none") )]
		pub not_mtchd: Option<ReconciliationResult10>,
	}
	
	impl ReconciliationMatchedStatus9Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref mtchd_value) = self.mtchd { if let Err(e) = mtchd_value.validate() { return Err(e); } }
			if let Some(ref not_mtchd_value) = self.not_mtchd { if let Err(e) = not_mtchd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ReconciliationReport8 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ReconciliationReport8 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none") )]
		pub tech_rcrd_id: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxId") )]
		pub tx_id: TradeTransactionIdentification19,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Modfd") )]
		pub modfd: bool,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RcncltnSts") )]
		pub rcncltn_sts: ReconciliationStatus8Choice,
	}
	
	impl ReconciliationReport8 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tech_rcrd_id_value) = self.tech_rcrd_id { if let Err(e) = tech_rcrd_id_value.validate() { return Err(e); } }
			if let Err(e) = self.tx_id.validate() { return Err(e); }
			if let Err(e) = self.rcncltn_sts.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// ReconciliationResult10 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ReconciliationResult10 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPty1") )]
		pub ctr_pty1: OrganisationIdentification15Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPty2") )]
		pub ctr_pty2: OrganisationIdentification15Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MtchgCrit") )]
		pub mtchg_crit: MatchingCriteria10,
	}
	
	impl ReconciliationResult10 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.ctr_pty1.validate() { return Err(e); }
			if let Err(e) = self.ctr_pty2.validate() { return Err(e); }
			if let Err(e) = self.mtchg_crit.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// ReconciliationStatus8Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct ReconciliationStatus8Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoRcncltnReqrd", skip_serializing_if = "Option::is_none") )]
		pub no_rcncltn_reqrd: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgData", skip_serializing_if = "Option::is_none") )]
		pub rptg_data: Option<ReconciliationMatchedStatus9Choice>,
	}
	
	impl ReconciliationStatus8Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref no_rcncltn_reqrd_value) = self.no_rcncltn_reqrd { if let Err(e) = no_rcncltn_reqrd_value.validate() { return Err(e); } }
			if let Some(ref rptg_data_value) = self.rptg_data { if let Err(e) = rptg_data_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// RepoTerminationOption2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum RepoTerminationOption2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "EGRN") )]
		CodeEGRN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EGAE") )]
		CodeEGAE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ETSB") )]
		CodeETSB,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NOAP") )]
		CodeNOAP,
	}
	
	impl RepoTerminationOption2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ReportPeriodActivity1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// SecuritiesFinancingReportingReconciliationStatusAdviceV02 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SecuritiesFinancingReportingReconciliationStatusAdviceV02 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RcncltnData") )]
		pub rcncltn_data: TradeData34Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl SecuritiesFinancingReportingReconciliationStatusAdviceV02 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rcncltn_data.validate() { return Err(e); }
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// SecuritiesLendingType3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SecuritiesLendingType3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalSecuritiesLendingType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max35Text>,
	}
	
	impl SecuritiesLendingType3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SecuritiesTransactionPrice19Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SecuritiesTransactionPrice19Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MntryVal", skip_serializing_if = "Option::is_none") )]
		pub mntry_val: Option<AmountAndDirection107>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Unit", skip_serializing_if = "Option::is_none") )]
		pub unit: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pctg", skip_serializing_if = "Option::is_none") )]
		pub pctg: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Yld", skip_serializing_if = "Option::is_none") )]
		pub yld: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dcml", skip_serializing_if = "Option::is_none") )]
		pub dcml: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PdgPric", skip_serializing_if = "Option::is_none") )]
		pub pdg_pric: Option<PriceStatus1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<SecuritiesTransactionPrice5>,
	}
	
	impl SecuritiesTransactionPrice19Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref mntry_val_value) = self.mntry_val { if let Err(e) = mntry_val_value.validate() { return Err(e); } }
			if let Some(ref pdg_pric_value) = self.pdg_pric { if let Err(e) = pdg_pric_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SecuritiesTransactionPrice5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SecuritiesTransactionPrice5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val", skip_serializing_if = "Option::is_none") )]
		pub val: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<Max35Text>,
	}
	
	impl SecuritiesTransactionPrice5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Security48 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct Security48 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<CompareISINIdentifier4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none") )]
		pub clssfctn_tp: Option<CompareCFIIdentifier3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Qty", skip_serializing_if = "Option::is_none") )]
		pub qty: Option<CompareDecimalNumber3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NmnlVal", skip_serializing_if = "Option::is_none") )]
		pub nmnl_val: Option<CompareAmountAndDirection2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Qlty", skip_serializing_if = "Option::is_none") )]
		pub qlty: Option<CompareCollateralQualityType3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Mtrty", skip_serializing_if = "Option::is_none") )]
		pub mtrty: Option<CompareDate3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IssrId", skip_serializing_if = "Option::is_none") )]
		pub issr_id: Option<CompareOrganisationIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IssrCtry", skip_serializing_if = "Option::is_none") )]
		pub issr_ctry: Option<CompareCountryCode3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<Vec<CompareSecuritiesLendingType3>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnitPric", skip_serializing_if = "Option::is_none") )]
		pub unit_pric: Option<CompareUnitPrice6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ExclsvArrgmnt", skip_serializing_if = "Option::is_none") )]
		pub exclsv_arrgmnt: Option<CompareTrueFalseIndicator3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MktVal", skip_serializing_if = "Option::is_none") )]
		pub mkt_val: Option<CompareAmountAndDirection2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AvlblForCollReuse", skip_serializing_if = "Option::is_none") )]
		pub avlbl_for_coll_reuse: Option<CompareTrueFalseIndicator3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HrcutOrMrgn", skip_serializing_if = "Option::is_none") )]
		pub hrcut_or_mrgn: Option<ComparePercentageRate3>,
	}
	
	impl Security48 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			if let Some(ref clssfctn_tp_value) = self.clssfctn_tp { if let Err(e) = clssfctn_tp_value.validate() { return Err(e); } }
			if let Some(ref qty_value) = self.qty { if let Err(e) = qty_value.validate() { return Err(e); } }
			if let Some(ref nmnl_val_value) = self.nmnl_val { if let Err(e) = nmnl_val_value.validate() { return Err(e); } }
			if let Some(ref qlty_value) = self.qlty { if let Err(e) = qlty_value.validate() { return Err(e); } }
			if let Some(ref mtrty_value) = self.mtrty { if let Err(e) = mtrty_value.validate() { return Err(e); } }
			if let Some(ref issr_id_value) = self.issr_id { if let Err(e) = issr_id_value.validate() { return Err(e); } }
			if let Some(ref issr_ctry_value) = self.issr_ctry { if let Err(e) = issr_ctry_value.validate() { return Err(e); } }
			if let Some(ref tp_vec) = self.tp { for item in tp_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref unit_pric_value) = self.unit_pric { if let Err(e) = unit_pric_value.validate() { return Err(e); } }
			if let Some(ref exclsv_arrgmnt_value) = self.exclsv_arrgmnt { if let Err(e) = exclsv_arrgmnt_value.validate() { return Err(e); } }
			if let Some(ref mkt_val_value) = self.mkt_val { if let Err(e) = mkt_val_value.validate() { return Err(e); } }
			if let Some(ref avlbl_for_coll_reuse_value) = self.avlbl_for_coll_reuse { if let Err(e) = avlbl_for_coll_reuse_value.validate() { return Err(e); } }
			if let Some(ref hrcut_or_mrgn_value) = self.hrcut_or_mrgn { if let Err(e) = hrcut_or_mrgn_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SecurityCommodity7Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SecurityCommodity7Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Scty", skip_serializing_if = "Option::is_none") )]
		pub scty: Option<Vec<Security48>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none") )]
		pub cmmdty: Option<Vec<Commodity42>>,
	}
	
	impl SecurityCommodity7Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref scty_vec) = self.scty { for item in scty_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref cmmdty_vec) = self.cmmdty { for item in cmmdty_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// SecurityCommodityCash4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SecurityCommodityCash4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Scty", skip_serializing_if = "Option::is_none") )]
		pub scty: Option<Vec<Security48>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none") )]
		pub cmmdty: Option<Vec<Commodity42>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Csh", skip_serializing_if = "Option::is_none") )]
		pub csh: Option<Vec<CashCompare3>>,
	}
	
	impl SecurityCommodityCash4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref scty_vec) = self.scty { for item in scty_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref cmmdty_vec) = self.cmmdty { for item in cmmdty_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref csh_vec) = self.csh { for item in csh_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// SecurityIdentification26Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SecurityIdentification26Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<ISINOct2015Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NotAvlbl", skip_serializing_if = "Option::is_none") )]
		pub not_avlbl: Option<NotAvailable1Code>,
	}
	
	impl SecurityIdentification26Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			if let Some(ref not_avlbl_value) = self.not_avlbl { if let Err(e) = not_avlbl_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SpecialCollateral1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum SpecialCollateral1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "GENE") )]
		CodeGENE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SPEC") )]
		CodeSPEC,
	}
	
	impl SpecialCollateral1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// SupplementaryData1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct SupplementaryDataEnvelope1 {
	}
	
	impl SupplementaryDataEnvelope1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// TradeData28 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct TradeData28 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PairgRcncltnSts", skip_serializing_if = "Option::is_none") )]
		pub pairg_rcncltn_sts: Option<Vec<NumberOfReportsPerStatus4>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RcncltnRpt") )]
		pub rcncltn_rpt: Vec<ReconciliationReport8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl TradeData28 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref pairg_rcncltn_sts_vec) = self.pairg_rcncltn_sts { for item in pairg_rcncltn_sts_vec { if let Err(e) = item.validate() { return Err(e); } } }
			for item in &self.rcncltn_rpt { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TradeData34Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct TradeData34Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
		pub data_set_actn: Option<ReportPeriodActivity1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rpt", skip_serializing_if = "Option::is_none") )]
		pub rpt: Option<Vec<TradeData28>>,
	}
	
	impl TradeData34Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref data_set_actn_value) = self.data_set_actn { if let Err(e) = data_set_actn_value.validate() { return Err(e); } }
			if let Some(ref rpt_vec) = self.rpt { for item in rpt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TradeTransactionIdentification19 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub struct TradeTransactionIdentification19 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgCtrPty") )]
		pub rptg_ctr_pty: OrganisationIdentification15Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrCtrPty") )]
		pub othr_ctr_pty: PartyIdentification236Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none") )]
		pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnqTradIdr", skip_serializing_if = "Option::is_none") )]
		pub unq_trad_idr: Option<Max52Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MstrAgrmt", skip_serializing_if = "Option::is_none") )]
		pub mstr_agrmt: Option<MasterAgreement7>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AgtLndr", skip_serializing_if = "Option::is_none") )]
		pub agt_lndr: Option<OrganisationIdentification15Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TrptyAgt", skip_serializing_if = "Option::is_none") )]
		pub trpty_agt: Option<OrganisationIdentification15Choice>,
	}
	
	impl TradeTransactionIdentification19 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rptg_ctr_pty.validate() { return Err(e); }
			if let Err(e) = self.othr_ctr_pty.validate() { return Err(e); }
			if let Some(ref ntty_rspnsbl_for_rpt_value) = self.ntty_rspnsbl_for_rpt { if let Err(e) = ntty_rspnsbl_for_rpt_value.validate() { return Err(e); } }
			if let Some(ref unq_trad_idr_value) = self.unq_trad_idr { if let Err(e) = unq_trad_idr_value.validate() { return Err(e); } }
			if let Some(ref mstr_agrmt_value) = self.mstr_agrmt { if let Err(e) = mstr_agrmt_value.validate() { return Err(e); } }
			if let Some(ref agt_lndr_value) = self.agt_lndr { if let Err(e) = agt_lndr_value.validate() { return Err(e); } }
			if let Some(ref trpty_agt_value) = self.trpty_agt { if let Err(e) = trpty_agt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TrueFalseIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// UnitOfMeasure11Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	pub enum UnitOfMeasure11Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ALOW") )]
		CodeALOW,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ACCY") )]
		CodeACCY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BARL") )]
		CodeBARL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BCUF") )]
		CodeBCUF,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BDFT") )]
		CodeBDFT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BUSL") )]
		CodeBUSL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CEER") )]
		CodeCEER,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLRT") )]
		CodeCLRT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "KILO") )]
		CodeKILO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PIEC") )]
		CodePIEC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TONS") )]
		CodeTONS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "METR") )]
		CodeMETR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INCH") )]
		CodeINCH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "YARD") )]
		CodeYARD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GBGA") )]
		CodeGBGA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GRAM") )]
		CodeGRAM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CMET") )]
		CodeCMET,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SMET") )]
		CodeSMET,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FOOT") )]
		CodeFOOT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MILE") )]
		CodeMILE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SQIN") )]
		CodeSQIN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SQFO") )]
		CodeSQFO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SQMI") )]
		CodeSQMI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GBOU") )]
		CodeGBOU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "USOU") )]
		CodeUSOU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GBPI") )]
		CodeGBPI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "USPI") )]
		CodeUSPI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GBQA") )]
		CodeGBQA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "USGA") )]
		CodeUSGA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MMET") )]
		CodeMMET,
		#[cfg_attr( feature = "derive_serde", serde(rename = "KMET") )]
		CodeKMET,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SQYA") )]
		CodeSQYA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ACRE") )]
		CodeACRE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ARES") )]
		CodeARES,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SMIL") )]
		CodeSMIL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SCMT") )]
		CodeSCMT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HECT") )]
		CodeHECT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SQKI") )]
		CodeSQKI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MILI") )]
		CodeMILI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CELI") )]
		CodeCELI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LITR") )]
		CodeLITR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PUND") )]
		CodePUND,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CBME") )]
		CodeCBME,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DAYS") )]
		CodeDAYS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DMET") )]
		CodeDMET,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ENVC") )]
		CodeENVC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ENVO") )]
		CodeENVO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HUWG") )]
		CodeHUWG,
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
		#[cfg_attr( feature = "derive_serde", serde(rename = "TONE") )]
		CodeTONE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MIBA") )]
		CodeMIBA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MBTU") )]
		CodeMBTU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OZTR") )]
		CodeOZTR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UCWT") )]
		CodeUCWT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IPNT") )]
		CodeIPNT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PWRD") )]
		CodePWRD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DGEU") )]
		CodeDGEU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TOCD") )]
		CodeTOCD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GGEU") )]
		CodeGGEU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "USQA") )]
		CodeUSQA,
	}
	
	impl UnitOfMeasure11Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
}