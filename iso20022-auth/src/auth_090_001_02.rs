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
	
	
	// ActiveOrHistoricCurrencyAnd19DecimalAmountSimpleType ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ActiveOrHistoricCurrencyCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// AgriculturalCommodityDairy2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AgriculturalCommodityDairy2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType20Code>,
	}
	
	impl AgriculturalCommodityDairy2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AgriculturalCommodityForestry2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AgriculturalCommodityForestry2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType21Code>,
	}
	
	impl AgriculturalCommodityForestry2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AgriculturalCommodityGrain3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AgriculturalCommodityGrain3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType5Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none") )]
		pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType30Code>,
	}
	
	impl AgriculturalCommodityGrain3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if let Err(e) = addtl_sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AgriculturalCommodityLiveStock2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AgriculturalCommodityLiveStock2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType22Code>,
	}
	
	impl AgriculturalCommodityLiveStock2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AgriculturalCommodityOilSeed2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AgriculturalCommodityOilSeed2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none") )]
		pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType1Code>,
	}
	
	impl AgriculturalCommodityOilSeed2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if let Err(e) = addtl_sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AgriculturalCommodityOliveOil3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AgriculturalCommodityOliveOil3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType3Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none") )]
		pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType29Code>,
	}
	
	impl AgriculturalCommodityOliveOil3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if let Err(e) = addtl_sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AgriculturalCommodityOther2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AgriculturalCommodityOther2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType49Code>,
	}
	
	impl AgriculturalCommodityOther2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AgriculturalCommodityPotato2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AgriculturalCommodityPotato2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType45Code>,
	}
	
	impl AgriculturalCommodityPotato2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AgriculturalCommoditySeafood2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AgriculturalCommoditySeafood2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType23Code>,
	}
	
	impl AgriculturalCommoditySeafood2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AgriculturalCommoditySoft2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AgriculturalCommoditySoft2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none") )]
		pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType2Code>,
	}
	
	impl AgriculturalCommoditySoft2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if let Err(e) = addtl_sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AnyBICDec2014Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// AssetClassCommodity6Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AssetClassCommodity6Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Agrcltrl", skip_serializing_if = "Option::is_none") )]
		pub agrcltrl: Option<AssetClassCommodityAgricultural6Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nrgy", skip_serializing_if = "Option::is_none") )]
		pub nrgy: Option<AssetClassCommodityEnergy3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Envttl", skip_serializing_if = "Option::is_none") )]
		pub envttl: Option<AssetClassCommodityEnvironmental3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Frtlzr", skip_serializing_if = "Option::is_none") )]
		pub frtlzr: Option<AssetClassCommodityFertilizer4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Frght", skip_serializing_if = "Option::is_none") )]
		pub frght: Option<AssetClassCommodityFreight4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Indx", skip_serializing_if = "Option::is_none") )]
		pub indx: Option<AssetClassCommodityIndex1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IndstrlPdct", skip_serializing_if = "Option::is_none") )]
		pub indstrl_pdct: Option<AssetClassCommodityIndustrialProduct2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Infltn", skip_serializing_if = "Option::is_none") )]
		pub infltn: Option<AssetClassCommodityInflation1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Metl", skip_serializing_if = "Option::is_none") )]
		pub metl: Option<AssetClassCommodityMetal2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MultiCmmdtyExtc", skip_serializing_if = "Option::is_none") )]
		pub multi_cmmdty_extc: Option<AssetClassCommodityMultiCommodityExotic1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OffclEcnmcSttstcs", skip_serializing_if = "Option::is_none") )]
		pub offcl_ecnmc_sttstcs: Option<AssetClassCommodityOfficialEconomicStatistics1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<AssetClassCommodityOther1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrC10", skip_serializing_if = "Option::is_none") )]
		pub othr_c10: Option<AssetClassCommodityC10Other1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ppr", skip_serializing_if = "Option::is_none") )]
		pub ppr: Option<AssetClassCommodityPaper4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Plprpln", skip_serializing_if = "Option::is_none") )]
		pub plprpln: Option<AssetClassCommodityPolypropylene4Choice>,
	}
	
	impl AssetClassCommodity6Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref agrcltrl_value) = self.agrcltrl { if let Err(e) = agrcltrl_value.validate() { return Err(e); } }
			if let Some(ref nrgy_value) = self.nrgy { if let Err(e) = nrgy_value.validate() { return Err(e); } }
			if let Some(ref envttl_value) = self.envttl { if let Err(e) = envttl_value.validate() { return Err(e); } }
			if let Some(ref frtlzr_value) = self.frtlzr { if let Err(e) = frtlzr_value.validate() { return Err(e); } }
			if let Some(ref frght_value) = self.frght { if let Err(e) = frght_value.validate() { return Err(e); } }
			if let Some(ref indx_value) = self.indx { if let Err(e) = indx_value.validate() { return Err(e); } }
			if let Some(ref indstrl_pdct_value) = self.indstrl_pdct { if let Err(e) = indstrl_pdct_value.validate() { return Err(e); } }
			if let Some(ref infltn_value) = self.infltn { if let Err(e) = infltn_value.validate() { return Err(e); } }
			if let Some(ref metl_value) = self.metl { if let Err(e) = metl_value.validate() { return Err(e); } }
			if let Some(ref multi_cmmdty_extc_value) = self.multi_cmmdty_extc { if let Err(e) = multi_cmmdty_extc_value.validate() { return Err(e); } }
			if let Some(ref offcl_ecnmc_sttstcs_value) = self.offcl_ecnmc_sttstcs { if let Err(e) = offcl_ecnmc_sttstcs_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			if let Some(ref othr_c10_value) = self.othr_c10 { if let Err(e) = othr_c10_value.validate() { return Err(e); } }
			if let Some(ref ppr_value) = self.ppr { if let Err(e) = ppr_value.validate() { return Err(e); } }
			if let Some(ref plprpln_value) = self.plprpln { if let Err(e) = plprpln_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityAgricultural6Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AssetClassCommodityAgricultural6Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "GrnOilSeed", skip_serializing_if = "Option::is_none") )]
		pub grn_oil_seed: Option<AgriculturalCommodityOilSeed2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Soft", skip_serializing_if = "Option::is_none") )]
		pub soft: Option<AgriculturalCommoditySoft2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ptt", skip_serializing_if = "Option::is_none") )]
		pub ptt: Option<AgriculturalCommodityPotato2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OlvOil", skip_serializing_if = "Option::is_none") )]
		pub olv_oil: Option<AgriculturalCommodityOliveOil3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dairy", skip_serializing_if = "Option::is_none") )]
		pub dairy: Option<AgriculturalCommodityDairy2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Frstry", skip_serializing_if = "Option::is_none") )]
		pub frstry: Option<AgriculturalCommodityForestry2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sfd", skip_serializing_if = "Option::is_none") )]
		pub sfd: Option<AgriculturalCommoditySeafood2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LiveStock", skip_serializing_if = "Option::is_none") )]
		pub live_stock: Option<AgriculturalCommodityLiveStock2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Grn", skip_serializing_if = "Option::is_none") )]
		pub grn: Option<AgriculturalCommodityGrain3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<AgriculturalCommodityOther2>,
	}
	
	impl AssetClassCommodityAgricultural6Choice {
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
	
	
	// AssetClassCommodityC10Other1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AssetClassCommodityC10Other1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType11Code,
	}
	
	impl AssetClassCommodityC10Other1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityEnergy3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AssetClassCommodityEnergy3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Elctrcty", skip_serializing_if = "Option::is_none") )]
		pub elctrcty: Option<EnergyCommodityElectricity2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtrlGas", skip_serializing_if = "Option::is_none") )]
		pub ntrl_gas: Option<EnergyCommodityNaturalGas3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Oil", skip_serializing_if = "Option::is_none") )]
		pub oil: Option<EnergyCommodityOil3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Coal", skip_serializing_if = "Option::is_none") )]
		pub coal: Option<EnergyCommodityCoal2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrNrgy", skip_serializing_if = "Option::is_none") )]
		pub intr_nrgy: Option<EnergyCommodityInterEnergy2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RnwblNrgy", skip_serializing_if = "Option::is_none") )]
		pub rnwbl_nrgy: Option<EnergyCommodityRenewableEnergy2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LghtEnd", skip_serializing_if = "Option::is_none") )]
		pub lght_end: Option<EnergyCommodityLightEnd2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dstllts", skip_serializing_if = "Option::is_none") )]
		pub dstllts: Option<EnergyCommodityDistillates2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<EnergyCommodityOther2>,
	}
	
	impl AssetClassCommodityEnergy3Choice {
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
	
	
	// AssetClassCommodityEnvironmental3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AssetClassCommodityEnvironmental3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Emssns", skip_serializing_if = "Option::is_none") )]
		pub emssns: Option<EnvironmentalCommodityEmission3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Wthr", skip_serializing_if = "Option::is_none") )]
		pub wthr: Option<EnvironmentalCommodityWeather2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CrbnRltd", skip_serializing_if = "Option::is_none") )]
		pub crbn_rltd: Option<EnvironmentalCommodityCarbonRelated2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<EnvironmentCommodityOther2>,
	}
	
	impl AssetClassCommodityEnvironmental3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref emssns_value) = self.emssns { if let Err(e) = emssns_value.validate() { return Err(e); } }
			if let Some(ref wthr_value) = self.wthr { if let Err(e) = wthr_value.validate() { return Err(e); } }
			if let Some(ref crbn_rltd_value) = self.crbn_rltd { if let Err(e) = crbn_rltd_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityFertilizer4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AssetClassCommodityFertilizer4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ammn", skip_serializing_if = "Option::is_none") )]
		pub ammn: Option<FertilizerCommodityAmmonia2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DmmnmPhspht", skip_serializing_if = "Option::is_none") )]
		pub dmmnm_phspht: Option<FertilizerCommodityDiammoniumPhosphate2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ptsh", skip_serializing_if = "Option::is_none") )]
		pub ptsh: Option<FertilizerCommodityPotash2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Slphr", skip_serializing_if = "Option::is_none") )]
		pub slphr: Option<FertilizerCommoditySulphur2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Urea", skip_serializing_if = "Option::is_none") )]
		pub urea: Option<FertilizerCommodityUrea2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UreaAndAmmnmNtrt", skip_serializing_if = "Option::is_none") )]
		pub urea_and_ammnm_ntrt: Option<FertilizerCommodityUreaAndAmmoniumNitrate2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<FertilizerCommodityOther2>,
	}
	
	impl AssetClassCommodityFertilizer4Choice {
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
	
	
	// AssetClassCommodityFreight4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AssetClassCommodityFreight4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dry", skip_serializing_if = "Option::is_none") )]
		pub dry: Option<FreightCommodityDry3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Wet", skip_serializing_if = "Option::is_none") )]
		pub wet: Option<FreightCommodityWet3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CntnrShip", skip_serializing_if = "Option::is_none") )]
		pub cntnr_ship: Option<FreightCommodityContainerShip2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<FreightCommodityOther2>,
	}
	
	impl AssetClassCommodityFreight4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref dry_value) = self.dry { if let Err(e) = dry_value.validate() { return Err(e); } }
			if let Some(ref wet_value) = self.wet { if let Err(e) = wet_value.validate() { return Err(e); } }
			if let Some(ref cntnr_ship_value) = self.cntnr_ship { if let Err(e) = cntnr_ship_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityIndex1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AssetClassCommodityIndex1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType16Code,
	}
	
	impl AssetClassCommodityIndex1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityIndustrialProduct2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AssetClassCommodityIndustrialProduct2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cnstrctn", skip_serializing_if = "Option::is_none") )]
		pub cnstrctn: Option<IndustrialProductCommodityConstruction2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Manfctg", skip_serializing_if = "Option::is_none") )]
		pub manfctg: Option<IndustrialProductCommodityManufacturing2>,
	}
	
	impl AssetClassCommodityIndustrialProduct2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cnstrctn_value) = self.cnstrctn { if let Err(e) = cnstrctn_value.validate() { return Err(e); } }
			if let Some(ref manfctg_value) = self.manfctg { if let Err(e) = manfctg_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityInflation1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// AssetClassCommodityMetal2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AssetClassCommodityMetal2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NonPrcs", skip_serializing_if = "Option::is_none") )]
		pub non_prcs: Option<MetalCommodityNonPrecious2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prcs", skip_serializing_if = "Option::is_none") )]
		pub prcs: Option<MetalCommodityPrecious2>,
	}
	
	impl AssetClassCommodityMetal2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref non_prcs_value) = self.non_prcs { if let Err(e) = non_prcs_value.validate() { return Err(e); } }
			if let Some(ref prcs_value) = self.prcs { if let Err(e) = prcs_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityMultiCommodityExotic1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// AssetClassCommodityPaper4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AssetClassCommodityPaper4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CntnrBrd", skip_serializing_if = "Option::is_none") )]
		pub cntnr_brd: Option<PaperCommodityContainerBoard2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nwsprnt", skip_serializing_if = "Option::is_none") )]
		pub nwsprnt: Option<PaperCommodityNewsprint2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pulp", skip_serializing_if = "Option::is_none") )]
		pub pulp: Option<PaperCommodityPulp2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RcvrdPpr", skip_serializing_if = "Option::is_none") )]
		pub rcvrd_ppr: Option<PaperCommodityOther1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<PaperCommodityOther1>,
	}
	
	impl AssetClassCommodityPaper4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cntnr_brd_value) = self.cntnr_brd { if let Err(e) = cntnr_brd_value.validate() { return Err(e); } }
			if let Some(ref nwsprnt_value) = self.nwsprnt { if let Err(e) = nwsprnt_value.validate() { return Err(e); } }
			if let Some(ref pulp_value) = self.pulp { if let Err(e) = pulp_value.validate() { return Err(e); } }
			if let Some(ref rcvrd_ppr_value) = self.rcvrd_ppr { if let Err(e) = rcvrd_ppr_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityPolypropylene4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AssetClassCommodityPolypropylene4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Plstc", skip_serializing_if = "Option::is_none") )]
		pub plstc: Option<PolypropyleneCommodityPlastic2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<PolypropyleneCommodityOther2>,
	}
	
	impl AssetClassCommodityPolypropylene4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref plstc_value) = self.plstc { if let Err(e) = plstc_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetClassDetailedSubProductType10Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// AssetClassProductType16Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum AssetClassProductType16Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "INDX") )]
		CodeINDX,
	}
	
	impl AssetClassProductType16Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AssetClassProductType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// AssetClassSubProductType39Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// AssetClassSubProductType49Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// BasketConstituents3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BasketConstituents3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "InstrmId") )]
		pub instrm_id: InstrumentIdentification6Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Qty", skip_serializing_if = "Option::is_none") )]
		pub qty: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none") )]
		pub unit_of_measr: Option<UnitOfMeasure8Choice>,
	}
	
	impl BasketConstituents3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.instrm_id.validate() { return Err(e); }
			if let Some(ref unit_of_measr_value) = self.unit_of_measr { if let Err(e) = unit_of_measr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CollateralPortfolioCode5Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// CollateralisationType3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum CollateralisationType3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "FLCL") )]
		CodeFLCL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OWCL") )]
		CodeOWCL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OWC1") )]
		CodeOWC1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OWC2") )]
		CodeOWC2,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OWP1") )]
		CodeOWP1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OWP2") )]
		CodeOWP2,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRCL") )]
		CodePRCL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRC1") )]
		CodePRC1,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRC2") )]
		CodePRC2,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UNCL") )]
		CodeUNCL,
	}
	
	impl CollateralisationType3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Counterparty45 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Counterparty45 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: PartyIdentification248Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ntr", skip_serializing_if = "Option::is_none") )]
		pub ntr: Option<CounterpartyTradeNature15Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradgCpcty", skip_serializing_if = "Option::is_none") )]
		pub tradg_cpcty: Option<TradingCapacity7Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DrctnOrSd", skip_serializing_if = "Option::is_none") )]
		pub drctn_or_sd: Option<Direction4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradrLctn", skip_serializing_if = "Option::is_none") )]
		pub tradr_lctn: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BookgLctn", skip_serializing_if = "Option::is_none") )]
		pub bookg_lctn: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgXmptn", skip_serializing_if = "Option::is_none") )]
		pub rptg_xmptn: Option<ReportingExemption1>,
	}
	
	impl Counterparty45 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref ntr_value) = self.ntr { if let Err(e) = ntr_value.validate() { return Err(e); } }
			if let Some(ref tradg_cpcty_value) = self.tradg_cpcty { if let Err(e) = tradg_cpcty_value.validate() { return Err(e); } }
			if let Some(ref drctn_or_sd_value) = self.drctn_or_sd { if let Err(e) = drctn_or_sd_value.validate() { return Err(e); } }
			if let Some(ref tradr_lctn_value) = self.tradr_lctn { if let Err(e) = tradr_lctn_value.validate() { return Err(e); } }
			if let Some(ref bookg_lctn_value) = self.bookg_lctn { if let Err(e) = bookg_lctn_value.validate() { return Err(e); } }
			if let Some(ref rptg_xmptn_value) = self.rptg_xmptn { if let Err(e) = rptg_xmptn_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Counterparty46 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Counterparty46 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "IdTp", skip_serializing_if = "Option::is_none") )]
		pub id_tp: Option<PartyIdentification248Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ntr", skip_serializing_if = "Option::is_none") )]
		pub ntr: Option<CounterpartyTradeNature15Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgOblgtn", skip_serializing_if = "Option::is_none") )]
		pub rptg_oblgtn: Option<bool>,
	}
	
	impl Counterparty46 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref id_tp_value) = self.id_tp { if let Err(e) = id_tp_value.validate() { return Err(e); } }
			if let Some(ref ntr_value) = self.ntr { if let Err(e) = ntr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CounterpartyTradeNature15Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CounterpartyTradeNature15Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FI", skip_serializing_if = "Option::is_none") )]
		pub fi: Option<FinancialInstitutionSector1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NFI", skip_serializing_if = "Option::is_none") )]
		pub nfi: Option<NonFinancialInstitutionSector10>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CntrlCntrPty", skip_serializing_if = "Option::is_none") )]
		pub cntrl_cntr_pty: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<NoReasonCode>,
	}
	
	impl CounterpartyTradeNature15Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref fi_value) = self.fi { if let Err(e) = fi_value.validate() { return Err(e); } }
			if let Some(ref nfi_value) = self.nfi { if let Err(e) = nfi_value.validate() { return Err(e); } }
			if let Some(ref cntrl_cntr_pty_value) = self.cntrl_cntr_pty { if let Err(e) = cntrl_cntr_pty_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CountryCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// CountrySubDivisionCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct CountrySubDivisionCode {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub country_sub_division_code: String,
	}
	
	impl CountrySubDivisionCode {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z]{2,2}\\-[0-9A-Z]{1,3}").unwrap();
			if !pattern.is_match(&self.country_sub_division_code) {
				return Err(ValidationError::new(1005, "country_sub_division_code does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// CreditDerivative7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CreditDerivative7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Snrty", skip_serializing_if = "Option::is_none") )]
		pub snrty: Option<DebtInstrumentSeniorityType2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RefPty", skip_serializing_if = "Option::is_none") )]
		pub ref_pty: Option<DerivativePartyIdentification1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtFrqcy", skip_serializing_if = "Option::is_none") )]
		pub pmt_frqcy: Option<Frequency13Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClctnBsis", skip_serializing_if = "Option::is_none") )]
		pub clctn_bsis: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Srs", skip_serializing_if = "Option::is_none") )]
		pub srs: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Vrsn", skip_serializing_if = "Option::is_none") )]
		pub vrsn: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IndxFctr", skip_serializing_if = "Option::is_none") )]
		pub indx_fctr: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TrchInd", skip_serializing_if = "Option::is_none") )]
		pub trch_ind: Option<bool>,
	}
	
	impl CreditDerivative7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref snrty_value) = self.snrty { if let Err(e) = snrty_value.validate() { return Err(e); } }
			if let Some(ref ref_pty_value) = self.ref_pty { if let Err(e) = ref_pty_value.validate() { return Err(e); } }
			if let Some(ref pmt_frqcy_value) = self.pmt_frqcy { if let Err(e) = pmt_frqcy_value.validate() { return Err(e); } }
			if let Some(ref clctn_bsis_value) = self.clctn_bsis { if let Err(e) = clctn_bsis_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CustomBasket4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CustomBasket4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Strr", skip_serializing_if = "Option::is_none") )]
		pub strr: Option<LEIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<Max52Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cnsttnts", skip_serializing_if = "Option::is_none") )]
		pub cnsttnts: Option<Vec<BasketConstituents3>>,
	}
	
	impl CustomBasket4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref strr_value) = self.strr { if let Err(e) = strr_value.validate() { return Err(e); } }
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			if let Some(ref cnsttnts_vec) = self.cnsttnts { for item in cnsttnts_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// DebtInstrumentSeniorityType2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum DebtInstrumentSeniorityType2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "SBOD") )]
		CodeSBOD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SNDB") )]
		CodeSNDB,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
	}
	
	impl DebtInstrumentSeniorityType2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// DerivativePartyIdentification1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DerivativePartyIdentification1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
		pub ctry: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none") )]
		pub ctry_sub_dvsn: Option<CountrySubDivisionCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
		pub lei: Option<LEIIdentifier>,
	}
	
	impl DerivativePartyIdentification1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ctry_value) = self.ctry { if let Err(e) = ctry_value.validate() { return Err(e); } }
			if let Some(ref ctry_sub_dvsn_value) = self.ctry_sub_dvsn { if let Err(e) = ctry_sub_dvsn_value.validate() { return Err(e); } }
			if let Some(ref lei_value) = self.lei { if let Err(e) = lei_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DerivativesTradePositionSetReportV02 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DerivativesTradePositionSetReportV02 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AggtdPos") )]
		pub aggtd_pos: PositionSetAggregated2Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl DerivativesTradePositionSetReportV02 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.aggtd_pos.validate() { return Err(e); }
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// Direction2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Direction2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DrctnOfTheFrstLeg") )]
		pub drctn_of_the_frst_leg: OptionParty3Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DrctnOfTheScndLeg", skip_serializing_if = "Option::is_none") )]
		pub drctn_of_the_scnd_leg: Option<OptionParty3Code>,
	}
	
	impl Direction2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.drctn_of_the_frst_leg.validate() { return Err(e); }
			if let Some(ref drctn_of_the_scnd_leg_value) = self.drctn_of_the_scnd_leg { if let Err(e) = drctn_of_the_scnd_leg_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Direction4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Direction4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Drctn", skip_serializing_if = "Option::is_none") )]
		pub drctn: Option<Direction2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtySd", skip_serializing_if = "Option::is_none") )]
		pub ctr_pty_sd: Option<OptionParty1Code>,
	}
	
	impl Direction4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref drctn_value) = self.drctn { if let Err(e) = drctn_value.validate() { return Err(e); } }
			if let Some(ref ctr_pty_sd_value) = self.ctr_pty_sd { if let Err(e) = ctr_pty_sd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// EnergyCommodityCoal2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct EnergyCommodityCoal2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType2Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType24Code>,
	}
	
	impl EnergyCommodityCoal2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// EnergyCommodityDistillates2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct EnergyCommodityDistillates2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType2Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType25Code>,
	}
	
	impl EnergyCommodityDistillates2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// EnergyCommodityElectricity2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct EnergyCommodityElectricity2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType2Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType6Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none") )]
		pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType5Code>,
	}
	
	impl EnergyCommodityElectricity2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if let Err(e) = addtl_sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// EnergyCommodityInterEnergy2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct EnergyCommodityInterEnergy2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType2Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType26Code>,
	}
	
	impl EnergyCommodityInterEnergy2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// EnergyCommodityLightEnd2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct EnergyCommodityLightEnd2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType2Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType27Code>,
	}
	
	impl EnergyCommodityLightEnd2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// EnergyCommodityNaturalGas3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct EnergyCommodityNaturalGas3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType2Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType7Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none") )]
		pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType31Code>,
	}
	
	impl EnergyCommodityNaturalGas3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if let Err(e) = addtl_sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// EnergyCommodityOil3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct EnergyCommodityOil3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType2Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType8Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none") )]
		pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType32Code>,
	}
	
	impl EnergyCommodityOil3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if let Err(e) = addtl_sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// EnergyCommodityOther2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct EnergyCommodityOther2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType2Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType49Code>,
	}
	
	impl EnergyCommodityOther2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// EnergyCommodityRenewableEnergy2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct EnergyCommodityRenewableEnergy2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType2Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType28Code>,
	}
	
	impl EnergyCommodityRenewableEnergy2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// EnvironmentCommodityOther2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct EnvironmentCommodityOther2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType3Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType49Code>,
	}
	
	impl EnvironmentCommodityOther2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// EnvironmentalCommodityCarbonRelated2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct EnvironmentalCommodityCarbonRelated2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType3Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType29Code>,
	}
	
	impl EnvironmentalCommodityCarbonRelated2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// EnvironmentalCommodityEmission3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct EnvironmentalCommodityEmission3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType3Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType10Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none") )]
		pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType8Code>,
	}
	
	impl EnvironmentalCommodityEmission3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if let Err(e) = addtl_sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// EnvironmentalCommodityWeather2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct EnvironmentalCommodityWeather2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType3Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType30Code>,
	}
	
	impl EnvironmentalCommodityWeather2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ExchangeRateBasis1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ExchangeRateBasis1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BaseCcy") )]
		pub base_ccy: ActiveCurrencyCode,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QtdCcy") )]
		pub qtd_ccy: ActiveCurrencyCode,
	}
	
	impl ExchangeRateBasis1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_ccy.validate() { return Err(e); }
			if let Err(e) = self.qtd_ccy.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// ExchangeRateBasis1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ExchangeRateBasis1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CcyPair", skip_serializing_if = "Option::is_none") )]
		pub ccy_pair: Option<ExchangeRateBasis1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max52Text>,
	}
	
	impl ExchangeRateBasis1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ccy_pair_value) = self.ccy_pair { if let Err(e) = ccy_pair_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ExternalAgreementType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ExternalBenchmarkCurveName1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalBenchmarkCurveName1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_benchmark_curve_name1_code: String,
	}
	
	impl ExternalBenchmarkCurveName1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_benchmark_curve_name1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_benchmark_curve_name1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_benchmark_curve_name1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_benchmark_curve_name1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalPartyRelationshipType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ExternalPartyRelationshipType1Code {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub external_party_relationship_type1_code: String,
	}
	
	impl ExternalPartyRelationshipType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.external_party_relationship_type1_code.chars().count() < 1 {
			return Err(ValidationError::new(1001, "external_party_relationship_type1_code is shorter than the minimum length of 1".to_string()));
			}
			if self.external_party_relationship_type1_code.chars().count() > 4 {
				return Err(ValidationError::new(1002, "external_party_relationship_type1_code exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// ExternalUnitOfMeasure1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// FertilizerCommodityAmmonia2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FertilizerCommodityAmmonia2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType5Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType39Code>,
	}
	
	impl FertilizerCommodityAmmonia2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FertilizerCommodityDiammoniumPhosphate2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FertilizerCommodityDiammoniumPhosphate2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType5Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType40Code>,
	}
	
	impl FertilizerCommodityDiammoniumPhosphate2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FertilizerCommodityOther2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FertilizerCommodityOther2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType5Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType49Code>,
	}
	
	impl FertilizerCommodityOther2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FertilizerCommodityPotash2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FertilizerCommodityPotash2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType5Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType41Code>,
	}
	
	impl FertilizerCommodityPotash2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FertilizerCommoditySulphur2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FertilizerCommoditySulphur2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType5Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType42Code>,
	}
	
	impl FertilizerCommoditySulphur2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FertilizerCommodityUrea2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FertilizerCommodityUrea2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType5Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType43Code>,
	}
	
	impl FertilizerCommodityUrea2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FertilizerCommodityUreaAndAmmoniumNitrate2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FertilizerCommodityUreaAndAmmoniumNitrate2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType5Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType44Code>,
	}
	
	impl FertilizerCommodityUreaAndAmmoniumNitrate2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FinancialInstitutionSector1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FinancialInstitutionSector1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sctr") )]
		pub sctr: Vec<FinancialPartyClassification2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrThrshld", skip_serializing_if = "Option::is_none") )]
		pub clr_thrshld: Option<bool>,
	}
	
	impl FinancialInstitutionSector1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			for item in &self.sctr { if let Err(e) = item.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FinancialInstrumentContractType2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum FinancialInstrumentContractType2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CFDS") )]
		CodeCFDS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FRAS") )]
		CodeFRAS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FUTR") )]
		CodeFUTR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FORW") )]
		CodeFORW,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OPTN") )]
		CodeOPTN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SPDB") )]
		CodeSPDB,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SWAP") )]
		CodeSWAP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SWPT") )]
		CodeSWPT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
	}
	
	impl FinancialInstrumentContractType2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// FinancialPartyClassification2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FinancialPartyClassification2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<FinancialPartySectorType3Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification175>,
	}
	
	impl FinancialPartyClassification2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FinancialPartySectorType3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum FinancialPartySectorType3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "AIFD") )]
		CodeAIFD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CSDS") )]
		CodeCSDS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CCPS") )]
		CodeCCPS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CDTI") )]
		CodeCDTI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INUN") )]
		CodeINUN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ORPI") )]
		CodeORPI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INVF") )]
		CodeINVF,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REIN") )]
		CodeREIN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UCIT") )]
		CodeUCIT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ASSU") )]
		CodeASSU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
	}
	
	impl FinancialPartySectorType3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// FreightCommodityContainerShip2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FreightCommodityContainerShip2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType4Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType46Code>,
	}
	
	impl FreightCommodityContainerShip2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FreightCommodityDry3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FreightCommodityDry3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType4Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType31Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none") )]
		pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType33Code>,
	}
	
	impl FreightCommodityDry3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if let Err(e) = addtl_sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FreightCommodityOther2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FreightCommodityOther2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType4Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType49Code>,
	}
	
	impl FreightCommodityOther2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FreightCommodityWet3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FreightCommodityWet3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType4Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType32Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none") )]
		pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType34Code>,
	}
	
	impl FreightCommodityWet3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if let Err(e) = addtl_sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Frequency13Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum Frequency13Code {
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
	}
	
	impl Frequency13Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// GenericIdentification175 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// GenericIdentification184 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericIdentification184 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max210Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Src") )]
		pub src: Max100Text,
	}
	
	impl GenericIdentification184 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Err(e) = self.src.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// GenericIdentification185 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericIdentification185 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max100Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<Max35Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl GenericIdentification185 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref schme_nm_value) = self.schme_nm { if let Err(e) = schme_nm_value.validate() { return Err(e); } }
			if let Some(ref issr_value) = self.issr { if let Err(e) = issr_value.validate() { return Err(e); } }
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
	
	
	// ISODate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// IndexIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct IndexIdentification1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
		pub isin: Option<ISINOct2015Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Indx", skip_serializing_if = "Option::is_none") )]
		pub indx: Option<ExternalBenchmarkCurveName1Code>,
	}
	
	impl IndexIdentification1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref isin_value) = self.isin { if let Err(e) = isin_value.validate() { return Err(e); } }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref indx_value) = self.indx { if let Err(e) = indx_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// IndustrialProductCommodityConstruction2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct IndustrialProductCommodityConstruction2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType6Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType33Code>,
	}
	
	impl IndustrialProductCommodityConstruction2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// IndustrialProductCommodityManufacturing2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct IndustrialProductCommodityManufacturing2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType6Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType34Code>,
	}
	
	impl IndustrialProductCommodityManufacturing2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// InstrumentIdentification6Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct InstrumentIdentification6Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
		pub isin: Option<ISINOct2015Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AltrntvInstrmId", skip_serializing_if = "Option::is_none") )]
		pub altrntv_instrm_id: Option<Max52Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none") )]
		pub unq_pdct_idr: Option<UniqueProductIdentifier1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrId", skip_serializing_if = "Option::is_none") )]
		pub othr_id: Option<GenericIdentification184>,
	}
	
	impl InstrumentIdentification6Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref isin_value) = self.isin { if let Err(e) = isin_value.validate() { return Err(e); } }
			if let Some(ref altrntv_instrm_id_value) = self.altrntv_instrm_id { if let Err(e) = altrntv_instrm_id_value.validate() { return Err(e); } }
			if let Some(ref unq_pdct_idr_value) = self.unq_pdct_idr { if let Err(e) = unq_pdct_idr_value.validate() { return Err(e); } }
			if let Some(ref othr_id_value) = self.othr_id { if let Err(e) = othr_id_value.validate() { return Err(e); } }
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
	
	
	// LegalPersonIdentification1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// MarginCollateralReport4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MarginCollateralReport4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CollPrtflCd") )]
		pub coll_prtfl_cd: CollateralPortfolioCode5Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CollstnCtgy") )]
		pub collstn_ctgy: CollateralisationType3Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TmStmp", skip_serializing_if = "Option::is_none") )]
		pub tm_stmp: Option<String>,
	}
	
	impl MarginCollateralReport4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.coll_prtfl_cd.validate() { return Err(e); }
			if let Err(e) = self.collstn_ctgy.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// MarginPortfolio3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// MaturityTerm2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MaturityTerm2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Unit") )]
		pub unit: RateBasis1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
		pub val: f64,
	}
	
	impl MaturityTerm2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.unit.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// Max1000Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max1000Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max1000_text: String,
	}
	
	impl Max1000Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max1000_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max1000_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max1000_text.chars().count() > 1000 {
				return Err(ValidationError::new(1002, "max1000_text exceeds the maximum length of 1000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max100Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max100Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max100_text: String,
	}
	
	impl Max100Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max100_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max100_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max100_text.chars().count() > 100 {
				return Err(ValidationError::new(1002, "max100_text exceeds the maximum length of 100".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max105Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max20PositiveNumber ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max20PositiveNumber {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max20_positive_number: f64,
	}
	
	impl Max20PositiveNumber {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max20_positive_number < 0.000000 {
				return Err(ValidationError::new(1003, "max20_positive_number is less than the minimum value of 0.000000".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max210Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max210Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max210_text: String,
	}
	
	impl Max210Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max210_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max210_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max210_text.chars().count() > 210 {
				return Err(ValidationError::new(1002, "max210_text exceeds the maximum length of 210".to_string()));
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
	
	
	// Max3Number ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Max4AlphaNumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max4AlphaNumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max4_alpha_numeric_text: String,
	}
	
	impl Max4AlphaNumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max4_alpha_numeric_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max4_alpha_numeric_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max4_alpha_numeric_text.chars().count() > 4 {
				return Err(ValidationError::new(1002, "max4_alpha_numeric_text exceeds the maximum length of 4".to_string()));
			}
			let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
			if !pattern.is_match(&self.max4_alpha_numeric_text) {
				return Err(ValidationError::new(1005, "max4_alpha_numeric_text does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max4Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max4Text {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max4_text: String,
	}
	
	impl Max4Text {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.max4_text.chars().count() < 1 {
			return Err(ValidationError::new(1001, "max4_text is shorter than the minimum length of 1".to_string()));
			}
			if self.max4_text.chars().count() > 4 {
				return Err(ValidationError::new(1002, "max4_text exceeds the maximum length of 4".to_string()));
			}
			Ok(())
		}
	}
	
	
	// Max500Text ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// MetalCommodityNonPrecious2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MetalCommodityNonPrecious2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType7Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType15Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none") )]
		pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType10Code>,
	}
	
	impl MetalCommodityNonPrecious2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if let Err(e) = addtl_sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// MetalCommodityPrecious2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MetalCommodityPrecious2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType7Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType16Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none") )]
		pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType11Code>,
	}
	
	impl MetalCommodityPrecious2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if let Err(e) = addtl_sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// NaturalPersonIdentification2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// NoReasonCode ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// NonFinancialInstitutionSector10 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct NonFinancialInstitutionSector10 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sctr") )]
		pub sctr: Vec<GenericIdentification175>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrThrshld", skip_serializing_if = "Option::is_none") )]
		pub clr_thrshld: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DrctlyLkdActvty", skip_serializing_if = "Option::is_none") )]
		pub drctly_lkd_actvty: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FdrlInstn", skip_serializing_if = "Option::is_none") )]
		pub fdrl_instn: Option<bool>,
	}
	
	impl NonFinancialInstitutionSector10 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			for item in &self.sctr { if let Err(e) = item.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// NotApplicable1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// NotionalAmount7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct NotionalAmount7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
		pub amt: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AmtInFct", skip_serializing_if = "Option::is_none") )]
		pub amt_in_fct: Option<Vec<ActiveOrHistoricCurrencyAnd19DecimalAmount>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WghtdAvrgDlta", skip_serializing_if = "Option::is_none") )]
		pub wghtd_avrg_dlta: Option<f64>,
	}
	
	impl NotionalAmount7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
			if let Some(ref amt_in_fct_vec) = self.amt_in_fct { for item in amt_in_fct_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// NotionalAmountLegs6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct NotionalAmountLegs6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrstLeg", skip_serializing_if = "Option::is_none") )]
		pub frst_leg: Option<NotionalAmount7>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ScndLeg", skip_serializing_if = "Option::is_none") )]
		pub scnd_leg: Option<NotionalAmount7>,
	}
	
	impl NotionalAmountLegs6 {
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
	
	
	// OptionParty1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum OptionParty1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "SLLR") )]
		CodeSLLR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BYER") )]
		CodeBYER,
	}
	
	impl OptionParty1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// OptionParty3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum OptionParty3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "MAKE") )]
		CodeMAKE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TAKE") )]
		CodeTAKE,
	}
	
	impl OptionParty3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// OptionType2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum OptionType2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CALL") )]
		CodeCALL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PUTO") )]
		CodePUTO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
	}
	
	impl OptionType2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// OrganisationIdentification15Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// OtherPayment6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OtherPayment6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtCcy", skip_serializing_if = "Option::is_none") )]
		pub pmt_ccy: Option<ActiveOrHistoricCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTp", skip_serializing_if = "Option::is_none") )]
		pub pmt_tp: Option<PaymentType5Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtDt", skip_serializing_if = "Option::is_none") )]
		pub pmt_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtPyer", skip_serializing_if = "Option::is_none") )]
		pub pmt_pyer: Option<PartyIdentification236Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtRcvr", skip_serializing_if = "Option::is_none") )]
		pub pmt_rcvr: Option<PartyIdentification236Choice>,
	}
	
	impl OtherPayment6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref pmt_ccy_value) = self.pmt_ccy { if let Err(e) = pmt_ccy_value.validate() { return Err(e); } }
			if let Some(ref pmt_tp_value) = self.pmt_tp { if let Err(e) = pmt_tp_value.validate() { return Err(e); } }
			if let Some(ref pmt_pyer_value) = self.pmt_pyer { if let Err(e) = pmt_pyer_value.validate() { return Err(e); } }
			if let Some(ref pmt_rcvr_value) = self.pmt_rcvr { if let Err(e) = pmt_rcvr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaperCommodityContainerBoard2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaperCommodityContainerBoard2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType8Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType35Code>,
	}
	
	impl PaperCommodityContainerBoard2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaperCommodityNewsprint2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaperCommodityNewsprint2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType8Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType36Code>,
	}
	
	impl PaperCommodityNewsprint2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaperCommodityOther1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaperCommodityOther1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType8Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType49Code>,
	}
	
	impl PaperCommodityOther1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaperCommodityPulp2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaperCommodityPulp2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType8Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType37Code>,
	}
	
	impl PaperCommodityPulp2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PartyIdentification236Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// PartyIdentification248Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// PaymentType4Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum PaymentType4Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "UFRO") )]
		CodeUFRO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UWIN") )]
		CodeUWIN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PEXH") )]
		CodePEXH,
	}
	
	impl PaymentType4Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// PaymentType5Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaymentType5Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<PaymentType4Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryTp", skip_serializing_if = "Option::is_none") )]
		pub prtry_tp: Option<Max4AlphaNumericText>,
	}
	
	impl PaymentType5Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref prtry_tp_value) = self.prtry_tp { if let Err(e) = prtry_tp_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PercentageRate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// PolypropyleneCommodityOther2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PolypropyleneCommodityOther2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType9Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType49Code>,
	}
	
	impl PolypropyleneCommodityOther2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PolypropyleneCommodityPlastic2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PolypropyleneCommodityPlastic2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType9Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType18Code>,
	}
	
	impl PolypropyleneCommodityPlastic2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref sub_pdct_value) = self.sub_pdct { if let Err(e) = sub_pdct_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PortfolioCode3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// PositionSet21 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PositionSet21 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dmnsns") )]
		pub dmnsns: PositionSetDimensions16,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Mtrcs") )]
		pub mtrcs: PositionSetMetrics14,
	}
	
	impl PositionSet21 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.dmnsns.validate() { return Err(e); }
			if let Err(e) = self.mtrcs.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// PositionSet22 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PositionSet22 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dmnsns") )]
		pub dmnsns: PositionSetCollateralDimensions3,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Mtrcs") )]
		pub mtrcs: PositionSetCollateralMetrics2,
	}
	
	impl PositionSet22 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.dmnsns.validate() { return Err(e); }
			if let Err(e) = self.mtrcs.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// PositionSetAggregated2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PositionSetAggregated2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
		pub data_set_actn: Option<ReportPeriodActivity1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rpt", skip_serializing_if = "Option::is_none") )]
		pub rpt: Option<PositionSetAggregated4>,
	}
	
	impl PositionSetAggregated2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref data_set_actn_value) = self.data_set_actn { if let Err(e) = data_set_actn_value.validate() { return Err(e); } }
			if let Some(ref rpt_value) = self.rpt { if let Err(e) = rpt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PositionSetAggregated4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PositionSetAggregated4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RefDt") )]
		pub ref_dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PosSet", skip_serializing_if = "Option::is_none") )]
		pub pos_set: Option<Vec<PositionSet21>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CcyPosSet", skip_serializing_if = "Option::is_none") )]
		pub ccy_pos_set: Option<Vec<PositionSet21>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CollPosSet", skip_serializing_if = "Option::is_none") )]
		pub coll_pos_set: Option<Vec<PositionSet22>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CcyCollPosSet", skip_serializing_if = "Option::is_none") )]
		pub ccy_coll_pos_set: Option<Vec<PositionSet22>>,
	}
	
	impl PositionSetAggregated4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref pos_set_vec) = self.pos_set { for item in pos_set_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref ccy_pos_set_vec) = self.ccy_pos_set { for item in ccy_pos_set_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref coll_pos_set_vec) = self.coll_pos_set { for item in coll_pos_set_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref ccy_coll_pos_set_vec) = self.ccy_coll_pos_set { for item in ccy_coll_pos_set_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// PositionSetBuyerAndSeller2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PositionSetBuyerAndSeller2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Buyr", skip_serializing_if = "Option::is_none") )]
		pub buyr: Option<PositionSetTotal2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sellr", skip_serializing_if = "Option::is_none") )]
		pub sellr: Option<PositionSetTotal2>,
	}
	
	impl PositionSetBuyerAndSeller2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref buyr_value) = self.buyr { if let Err(e) = buyr_value.validate() { return Err(e); } }
			if let Some(ref sellr_value) = self.sellr { if let Err(e) = sellr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PositionSetCollateralDimensions3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PositionSetCollateralDimensions3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtyId", skip_serializing_if = "Option::is_none") )]
		pub ctr_pty_id: Option<TradeCounterpartyReport20>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Coll", skip_serializing_if = "Option::is_none") )]
		pub coll: Option<MarginCollateralReport4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InitlMrgnPstdCcy", skip_serializing_if = "Option::is_none") )]
		pub initl_mrgn_pstd_ccy: Option<ActiveOrHistoricCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VartnMrgnPstdCcy", skip_serializing_if = "Option::is_none") )]
		pub vartn_mrgn_pstd_ccy: Option<ActiveOrHistoricCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InitlMrgnRcvdCcy", skip_serializing_if = "Option::is_none") )]
		pub initl_mrgn_rcvd_ccy: Option<ActiveOrHistoricCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VartnMrgnRcvdCcy", skip_serializing_if = "Option::is_none") )]
		pub vartn_mrgn_rcvd_ccy: Option<ActiveOrHistoricCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XcssCollPstdCcy", skip_serializing_if = "Option::is_none") )]
		pub xcss_coll_pstd_ccy: Option<ActiveOrHistoricCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XcssCollRcvdCcy", skip_serializing_if = "Option::is_none") )]
		pub xcss_coll_rcvd_ccy: Option<ActiveOrHistoricCurrencyCode>,
	}
	
	impl PositionSetCollateralDimensions3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ctr_pty_id_value) = self.ctr_pty_id { if let Err(e) = ctr_pty_id_value.validate() { return Err(e); } }
			if let Some(ref coll_value) = self.coll { if let Err(e) = coll_value.validate() { return Err(e); } }
			if let Some(ref initl_mrgn_pstd_ccy_value) = self.initl_mrgn_pstd_ccy { if let Err(e) = initl_mrgn_pstd_ccy_value.validate() { return Err(e); } }
			if let Some(ref vartn_mrgn_pstd_ccy_value) = self.vartn_mrgn_pstd_ccy { if let Err(e) = vartn_mrgn_pstd_ccy_value.validate() { return Err(e); } }
			if let Some(ref initl_mrgn_rcvd_ccy_value) = self.initl_mrgn_rcvd_ccy { if let Err(e) = initl_mrgn_rcvd_ccy_value.validate() { return Err(e); } }
			if let Some(ref vartn_mrgn_rcvd_ccy_value) = self.vartn_mrgn_rcvd_ccy { if let Err(e) = vartn_mrgn_rcvd_ccy_value.validate() { return Err(e); } }
			if let Some(ref xcss_coll_pstd_ccy_value) = self.xcss_coll_pstd_ccy { if let Err(e) = xcss_coll_pstd_ccy_value.validate() { return Err(e); } }
			if let Some(ref xcss_coll_rcvd_ccy_value) = self.xcss_coll_rcvd_ccy { if let Err(e) = xcss_coll_rcvd_ccy_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PositionSetCollateralMetrics2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PositionSetCollateralMetrics2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ttl", skip_serializing_if = "Option::is_none") )]
		pub ttl: Option<PositionSetCollateralTotal2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Clean", skip_serializing_if = "Option::is_none") )]
		pub clean: Option<PositionSetCollateralTotal2>,
	}
	
	impl PositionSetCollateralMetrics2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ttl_value) = self.ttl { if let Err(e) = ttl_value.validate() { return Err(e); } }
			if let Some(ref clean_value) = self.clean { if let Err(e) = clean_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PositionSetCollateralTotal2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PositionSetCollateralTotal2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfRpts", skip_serializing_if = "Option::is_none") )]
		pub nb_of_rpts: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstdMrgnOrColl", skip_serializing_if = "Option::is_none") )]
		pub pstd_mrgn_or_coll: Option<PostedMarginOrCollateral6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RcvdMrgnOrColl", skip_serializing_if = "Option::is_none") )]
		pub rcvd_mrgn_or_coll: Option<ReceivedMarginOrCollateral6>,
	}
	
	impl PositionSetCollateralTotal2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref pstd_mrgn_or_coll_value) = self.pstd_mrgn_or_coll { if let Err(e) = pstd_mrgn_or_coll_value.validate() { return Err(e); } }
			if let Some(ref rcvd_mrgn_or_coll_value) = self.rcvd_mrgn_or_coll { if let Err(e) = rcvd_mrgn_or_coll_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PositionSetDimensions16 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PositionSetDimensions16 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtyId", skip_serializing_if = "Option::is_none") )]
		pub ctr_pty_id: Option<TradeCounterpartyReport20>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ValCcy", skip_serializing_if = "Option::is_none") )]
		pub val_ccy: Option<ActiveOrHistoricCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Coll", skip_serializing_if = "Option::is_none") )]
		pub coll: Option<MarginCollateralReport4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctTp", skip_serializing_if = "Option::is_none") )]
		pub ctrct_tp: Option<FinancialInstrumentContractType2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AsstClss", skip_serializing_if = "Option::is_none") )]
		pub asst_clss: Option<ProductType4Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygInstrm", skip_serializing_if = "Option::is_none") )]
		pub undrlyg_instrm: Option<SecurityIdentification41Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlCcy", skip_serializing_if = "Option::is_none") )]
		pub ntnl_ccy: Option<ActiveOrHistoricCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlCcyScndLeg", skip_serializing_if = "Option::is_none") )]
		pub ntnl_ccy_scnd_leg: Option<ActiveOrHistoricCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none") )]
		pub sttlm_ccy: Option<ActiveOrHistoricCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmCcyScndLeg", skip_serializing_if = "Option::is_none") )]
		pub sttlm_ccy_scnd_leg: Option<ActiveOrHistoricCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MstrAgrmt", skip_serializing_if = "Option::is_none") )]
		pub mstr_agrmt: Option<MasterAgreement8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Clrd", skip_serializing_if = "Option::is_none") )]
		pub clrd: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntraGrp", skip_serializing_if = "Option::is_none") )]
		pub intra_grp: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XchgRateBsis", skip_serializing_if = "Option::is_none") )]
		pub xchg_rate_bsis: Option<ExchangeRateBasis1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OptnTp", skip_serializing_if = "Option::is_none") )]
		pub optn_tp: Option<OptionType2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TmToMtrty", skip_serializing_if = "Option::is_none") )]
		pub tm_to_mtrty: Option<TimeToMaturity1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IRSTp", skip_serializing_if = "Option::is_none") )]
		pub irs_tp: Option<Max52Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cdt", skip_serializing_if = "Option::is_none") )]
		pub cdt: Option<CreditDerivative7>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none") )]
		pub cmmdty: Option<AssetClassCommodity6Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPmt", skip_serializing_if = "Option::is_none") )]
		pub othr_pmt: Option<OtherPayment6>,
	}
	
	impl PositionSetDimensions16 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ctr_pty_id_value) = self.ctr_pty_id { if let Err(e) = ctr_pty_id_value.validate() { return Err(e); } }
			if let Some(ref val_ccy_value) = self.val_ccy { if let Err(e) = val_ccy_value.validate() { return Err(e); } }
			if let Some(ref coll_value) = self.coll { if let Err(e) = coll_value.validate() { return Err(e); } }
			if let Some(ref ctrct_tp_value) = self.ctrct_tp { if let Err(e) = ctrct_tp_value.validate() { return Err(e); } }
			if let Some(ref asst_clss_value) = self.asst_clss { if let Err(e) = asst_clss_value.validate() { return Err(e); } }
			if let Some(ref undrlyg_instrm_value) = self.undrlyg_instrm { if let Err(e) = undrlyg_instrm_value.validate() { return Err(e); } }
			if let Some(ref ntnl_ccy_value) = self.ntnl_ccy { if let Err(e) = ntnl_ccy_value.validate() { return Err(e); } }
			if let Some(ref ntnl_ccy_scnd_leg_value) = self.ntnl_ccy_scnd_leg { if let Err(e) = ntnl_ccy_scnd_leg_value.validate() { return Err(e); } }
			if let Some(ref sttlm_ccy_value) = self.sttlm_ccy { if let Err(e) = sttlm_ccy_value.validate() { return Err(e); } }
			if let Some(ref sttlm_ccy_scnd_leg_value) = self.sttlm_ccy_scnd_leg { if let Err(e) = sttlm_ccy_scnd_leg_value.validate() { return Err(e); } }
			if let Some(ref mstr_agrmt_value) = self.mstr_agrmt { if let Err(e) = mstr_agrmt_value.validate() { return Err(e); } }
			if let Some(ref xchg_rate_bsis_value) = self.xchg_rate_bsis { if let Err(e) = xchg_rate_bsis_value.validate() { return Err(e); } }
			if let Some(ref optn_tp_value) = self.optn_tp { if let Err(e) = optn_tp_value.validate() { return Err(e); } }
			if let Some(ref tm_to_mtrty_value) = self.tm_to_mtrty { if let Err(e) = tm_to_mtrty_value.validate() { return Err(e); } }
			if let Some(ref irs_tp_value) = self.irs_tp { if let Err(e) = irs_tp_value.validate() { return Err(e); } }
			if let Some(ref cdt_value) = self.cdt { if let Err(e) = cdt_value.validate() { return Err(e); } }
			if let Some(ref cmmdty_value) = self.cmmdty { if let Err(e) = cmmdty_value.validate() { return Err(e); } }
			if let Some(ref othr_pmt_value) = self.othr_pmt { if let Err(e) = othr_pmt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PositionSetMetrics14 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PositionSetMetrics14 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ttl", skip_serializing_if = "Option::is_none") )]
		pub ttl: Option<PositionSetBuyerAndSeller2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Clean", skip_serializing_if = "Option::is_none") )]
		pub clean: Option<PositionSetBuyerAndSeller2>,
	}
	
	impl PositionSetMetrics14 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ttl_value) = self.ttl { if let Err(e) = ttl_value.validate() { return Err(e); } }
			if let Some(ref clean_value) = self.clean { if let Err(e) = clean_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PositionSetTotal2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PositionSetTotal2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTrds", skip_serializing_if = "Option::is_none") )]
		pub nb_of_trds: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PostvVal", skip_serializing_if = "Option::is_none") )]
		pub postv_val: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NegVal", skip_serializing_if = "Option::is_none") )]
		pub neg_val: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ntnl", skip_serializing_if = "Option::is_none") )]
		pub ntnl: Option<NotionalAmountLegs6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPmtAmt", skip_serializing_if = "Option::is_none") )]
		pub othr_pmt_amt: Option<Vec<ActiveOrHistoricCurrencyAnd19DecimalAmount>>,
	}
	
	impl PositionSetTotal2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref postv_val_value) = self.postv_val { if let Err(e) = postv_val_value.validate() { return Err(e); } }
			if let Some(ref neg_val_value) = self.neg_val { if let Err(e) = neg_val_value.validate() { return Err(e); } }
			if let Some(ref ntnl_value) = self.ntnl { if let Err(e) = ntnl_value.validate() { return Err(e); } }
			if let Some(ref othr_pmt_amt_vec) = self.othr_pmt_amt { for item in othr_pmt_amt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// PostedMarginOrCollateral6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PostedMarginOrCollateral6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "InitlMrgnPstdPreHrcut", skip_serializing_if = "Option::is_none") )]
		pub initl_mrgn_pstd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InitlMrgnPstdPstHrcut", skip_serializing_if = "Option::is_none") )]
		pub initl_mrgn_pstd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VartnMrgnPstdPreHrcut", skip_serializing_if = "Option::is_none") )]
		pub vartn_mrgn_pstd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VartnMrgnPstdPstHrcut", skip_serializing_if = "Option::is_none") )]
		pub vartn_mrgn_pstd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XcssCollPstd", skip_serializing_if = "Option::is_none") )]
		pub xcss_coll_pstd: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	}
	
	impl PostedMarginOrCollateral6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref initl_mrgn_pstd_pre_hrcut_value) = self.initl_mrgn_pstd_pre_hrcut { if let Err(e) = initl_mrgn_pstd_pre_hrcut_value.validate() { return Err(e); } }
			if let Some(ref initl_mrgn_pstd_pst_hrcut_value) = self.initl_mrgn_pstd_pst_hrcut { if let Err(e) = initl_mrgn_pstd_pst_hrcut_value.validate() { return Err(e); } }
			if let Some(ref vartn_mrgn_pstd_pre_hrcut_value) = self.vartn_mrgn_pstd_pre_hrcut { if let Err(e) = vartn_mrgn_pstd_pre_hrcut_value.validate() { return Err(e); } }
			if let Some(ref vartn_mrgn_pstd_pst_hrcut_value) = self.vartn_mrgn_pstd_pst_hrcut { if let Err(e) = vartn_mrgn_pstd_pst_hrcut_value.validate() { return Err(e); } }
			if let Some(ref xcss_coll_pstd_value) = self.xcss_coll_pstd { if let Err(e) = xcss_coll_pstd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ProductType4Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum ProductType4Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CRDT") )]
		CodeCRDT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CURR") )]
		CodeCURR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EQUI") )]
		CodeEQUI,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INTR") )]
		CodeINTR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "COMM") )]
		CodeCOMM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
	}
	
	impl ProductType4Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	
	
	// ReceivedMarginOrCollateral6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ReceivedMarginOrCollateral6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "InitlMrgnRcvdPreHrcut", skip_serializing_if = "Option::is_none") )]
		pub initl_mrgn_rcvd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "InitlMrgnRcvdPstHrcut", skip_serializing_if = "Option::is_none") )]
		pub initl_mrgn_rcvd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VartnMrgnRcvdPreHrcut", skip_serializing_if = "Option::is_none") )]
		pub vartn_mrgn_rcvd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VartnMrgnRcvdPstHrcut", skip_serializing_if = "Option::is_none") )]
		pub vartn_mrgn_rcvd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XcssCollRcvd", skip_serializing_if = "Option::is_none") )]
		pub xcss_coll_rcvd: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	}
	
	impl ReceivedMarginOrCollateral6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref initl_mrgn_rcvd_pre_hrcut_value) = self.initl_mrgn_rcvd_pre_hrcut { if let Err(e) = initl_mrgn_rcvd_pre_hrcut_value.validate() { return Err(e); } }
			if let Some(ref initl_mrgn_rcvd_pst_hrcut_value) = self.initl_mrgn_rcvd_pst_hrcut { if let Err(e) = initl_mrgn_rcvd_pst_hrcut_value.validate() { return Err(e); } }
			if let Some(ref vartn_mrgn_rcvd_pre_hrcut_value) = self.vartn_mrgn_rcvd_pre_hrcut { if let Err(e) = vartn_mrgn_rcvd_pre_hrcut_value.validate() { return Err(e); } }
			if let Some(ref vartn_mrgn_rcvd_pst_hrcut_value) = self.vartn_mrgn_rcvd_pst_hrcut { if let Err(e) = vartn_mrgn_rcvd_pst_hrcut_value.validate() { return Err(e); } }
			if let Some(ref xcss_coll_rcvd_value) = self.xcss_coll_rcvd { if let Err(e) = xcss_coll_rcvd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ReportPeriodActivity1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ReportingExemption1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ReportingExemption1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn") )]
		pub rsn: Max4Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<Max1000Text>,
	}
	
	impl ReportingExemption1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rsn.validate() { return Err(e); }
			if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SecurityIdentification41Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SecurityIdentification41Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
		pub isin: Option<ISINOct2015Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AltrntvInstrmId", skip_serializing_if = "Option::is_none") )]
		pub altrntv_instrm_id: Option<Max52Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none") )]
		pub unq_pdct_idr: Option<UniqueProductIdentifier2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Bskt", skip_serializing_if = "Option::is_none") )]
		pub bskt: Option<CustomBasket4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Indx", skip_serializing_if = "Option::is_none") )]
		pub indx: Option<IndexIdentification1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<GenericIdentification184>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IdNotAvlbl", skip_serializing_if = "Option::is_none") )]
		pub id_not_avlbl: Option<UnderlyingIdentification1Code>,
	}
	
	impl SecurityIdentification41Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref isin_value) = self.isin { if let Err(e) = isin_value.validate() { return Err(e); } }
			if let Some(ref altrntv_instrm_id_value) = self.altrntv_instrm_id { if let Err(e) = altrntv_instrm_id_value.validate() { return Err(e); } }
			if let Some(ref unq_pdct_idr_value) = self.unq_pdct_idr { if let Err(e) = unq_pdct_idr_value.validate() { return Err(e); } }
			if let Some(ref bskt_value) = self.bskt { if let Err(e) = bskt_value.validate() { return Err(e); } }
			if let Some(ref indx_value) = self.indx { if let Err(e) = indx_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			if let Some(ref id_not_avlbl_value) = self.id_not_avlbl { if let Err(e) = id_not_avlbl_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SpecialPurpose2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum SpecialPurpose2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "BLNK") )]
		CodeBLNK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NTAV") )]
		CodeNTAV,
	}
	
	impl SpecialPurpose2Code {
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
	
	
	// TimeToMaturity1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TimeToMaturity1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prd", skip_serializing_if = "Option::is_none") )]
		pub prd: Option<TimeToMaturityPeriod1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Spcl", skip_serializing_if = "Option::is_none") )]
		pub spcl: Option<SpecialPurpose2Code>,
	}
	
	impl TimeToMaturity1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref prd_value) = self.prd { if let Err(e) = prd_value.validate() { return Err(e); } }
			if let Some(ref spcl_value) = self.spcl { if let Err(e) = spcl_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TimeToMaturityPeriod1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TimeToMaturityPeriod1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Start", skip_serializing_if = "Option::is_none") )]
		pub start: Option<MaturityTerm2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "End", skip_serializing_if = "Option::is_none") )]
		pub end: Option<MaturityTerm2>,
	}
	
	impl TimeToMaturityPeriod1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref start_value) = self.start { if let Err(e) = start_value.validate() { return Err(e); } }
			if let Some(ref end_value) = self.end { if let Err(e) = end_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TradeCounterpartyRelationship1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeCounterpartyRelationship1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalPartyRelationshipType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max100Text>,
	}
	
	impl TradeCounterpartyRelationship1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TradeCounterpartyRelationshipRecord1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeCounterpartyRelationshipRecord1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "StartRltshPty") )]
		pub start_rltsh_pty: TradeCounterpartyType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EndRltshPty") )]
		pub end_rltsh_pty: TradeCounterpartyType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RltshTp") )]
		pub rltsh_tp: TradeCounterpartyRelationship1Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<Max1000Text>,
	}
	
	impl TradeCounterpartyRelationshipRecord1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.start_rltsh_pty.validate() { return Err(e); }
			if let Err(e) = self.end_rltsh_pty.validate() { return Err(e); }
			if let Err(e) = self.rltsh_tp.validate() { return Err(e); }
			if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TradeCounterpartyReport20 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeCounterpartyReport20 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgCtrPty") )]
		pub rptg_ctr_pty: Counterparty45,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrCtrPty") )]
		pub othr_ctr_pty: Counterparty46,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Brkr", skip_serializing_if = "Option::is_none") )]
		pub brkr: Option<OrganisationIdentification15Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubmitgAgt", skip_serializing_if = "Option::is_none") )]
		pub submitg_agt: Option<OrganisationIdentification15Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrMmb", skip_serializing_if = "Option::is_none") )]
		pub clr_mmb: Option<PartyIdentification248Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Bnfcry", skip_serializing_if = "Option::is_none") )]
		pub bnfcry: Option<Vec<PartyIdentification248Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none") )]
		pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ExctnAgt", skip_serializing_if = "Option::is_none") )]
		pub exctn_agt: Option<Vec<OrganisationIdentification15Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RltshRcrd", skip_serializing_if = "Option::is_none") )]
		pub rltsh_rcrd: Option<Vec<TradeCounterpartyRelationshipRecord1>>,
	}
	
	impl TradeCounterpartyReport20 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rptg_ctr_pty.validate() { return Err(e); }
			if let Err(e) = self.othr_ctr_pty.validate() { return Err(e); }
			if let Some(ref brkr_value) = self.brkr { if let Err(e) = brkr_value.validate() { return Err(e); } }
			if let Some(ref submitg_agt_value) = self.submitg_agt { if let Err(e) = submitg_agt_value.validate() { return Err(e); } }
			if let Some(ref clr_mmb_value) = self.clr_mmb { if let Err(e) = clr_mmb_value.validate() { return Err(e); } }
			if let Some(ref bnfcry_vec) = self.bnfcry { for item in bnfcry_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref ntty_rspnsbl_for_rpt_value) = self.ntty_rspnsbl_for_rpt { if let Err(e) = ntty_rspnsbl_for_rpt_value.validate() { return Err(e); } }
			if let Some(ref exctn_agt_vec) = self.exctn_agt { for item in exctn_agt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref rltsh_rcrd_vec) = self.rltsh_rcrd { for item in rltsh_rcrd_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TradeCounterpartyType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum TradeCounterpartyType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "BENE") )]
		CodeBENE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BROK") )]
		CodeBROK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLEM") )]
		CodeCLEM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EXEA") )]
		CodeEXEA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHC") )]
		CodeOTHC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "REPC") )]
		CodeREPC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SBMA") )]
		CodeSBMA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ERFR") )]
		CodeERFR,
	}
	
	impl TradeCounterpartyType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// TradingCapacity7Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum TradingCapacity7Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "AGEN") )]
		CodeAGEN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRIN") )]
		CodePRIN,
	}
	
	impl TradingCapacity7Code {
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
	
	
	// UnderlyingIdentification1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum UnderlyingIdentification1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "UKWN") )]
		CodeUKWN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BSKT") )]
		CodeBSKT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "INDX") )]
		CodeINDX,
	}
	
	impl UnderlyingIdentification1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// UniqueProductIdentifier1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct UniqueProductIdentifier1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<Max52Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification175>,
	}
	
	impl UniqueProductIdentifier1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// UniqueProductIdentifier2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct UniqueProductIdentifier2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<Max52Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification185>,
	}
	
	impl UniqueProductIdentifier2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// UnitOfMeasure8Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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