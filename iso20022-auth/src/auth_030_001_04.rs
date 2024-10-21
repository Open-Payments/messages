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
	
	
	// AllocationIndicator1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum AllocationIndicator1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "POST") )]
		CodePOST,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PREA") )]
		CodePREA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UNAL") )]
		CodeUNAL,
	}
	
	impl AllocationIndicator1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// AmountAndDirection106 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// AmountAndDirection109 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AmountAndDirection109 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
		pub amt: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sgn", skip_serializing_if = "Option::is_none") )]
		pub sgn: Option<bool>,
	}
	
	impl AmountAndDirection109 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref amt_value) = self.amt { if let Err(e) = amt_value.validate() { return Err(e); } }
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
	
	
	// AssetClassCommodity7Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AssetClassCommodity7Choice {
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
		pub ppr: Option<AssetClassCommodityPaper5Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Plprpln", skip_serializing_if = "Option::is_none") )]
		pub plprpln: Option<AssetClassCommodityPolypropylene4Choice>,
	}
	
	impl AssetClassCommodity7Choice {
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
	
	
	// AssetClassCommodityPaper5Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AssetClassCommodityPaper5Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CntnrBrd", skip_serializing_if = "Option::is_none") )]
		pub cntnr_brd: Option<PaperCommodityContainerBoard2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nwsprnt", skip_serializing_if = "Option::is_none") )]
		pub nwsprnt: Option<PaperCommodityNewsprint2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pulp", skip_serializing_if = "Option::is_none") )]
		pub pulp: Option<PaperCommodityPulp2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RcvrdPpr", skip_serializing_if = "Option::is_none") )]
		pub rcvrd_ppr: Option<PaperCommodityRecoveredPaper3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<PaperCommodityOther1>,
	}
	
	impl AssetClassCommodityPaper5Choice {
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
	
	
	// AssetClassSubProductType50Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum AssetClassSubProductType50Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RCVP") )]
		CodeRCVP,
	}
	
	impl AssetClassSubProductType50Code {
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
	
	
	// BaseOne18Rate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct BaseOne18Rate {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub base_one18_rate: f64,
	}
	
	impl BaseOne18Rate {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// BaseOneRate ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// CFIOct2015Identifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Cleared23Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Cleared23Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Clrd", skip_serializing_if = "Option::is_none") )]
		pub clrd: Option<ClearingPartyAndTime21Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntndToClear", skip_serializing_if = "Option::is_none") )]
		pub intnd_to_clear: Option<ClearingPartyAndTime22Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NonClrd", skip_serializing_if = "Option::is_none") )]
		pub non_clrd: Option<ClearingExceptionOrExemption3Choice>,
	}
	
	impl Cleared23Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref clrd_value) = self.clrd { if let Err(e) = clrd_value.validate() { return Err(e); } }
			if let Some(ref intnd_to_clear_value) = self.intnd_to_clear { if let Err(e) = intnd_to_clear_value.validate() { return Err(e); } }
			if let Some(ref non_clrd_value) = self.non_clrd { if let Err(e) = non_clrd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ClearingAccountType4Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum ClearingAccountType4Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CLIE") )]
		CodeCLIE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HOUS") )]
		CodeHOUS,
	}
	
	impl ClearingAccountType4Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ClearingExceptionOrExemption2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ClearingExceptionOrExemption2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgCtrPty") )]
		pub rptg_ctr_pty: NonClearingReason2,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none") )]
		pub othr_ctr_pty: Option<NonClearingReason2>,
	}
	
	impl ClearingExceptionOrExemption2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rptg_ctr_pty.validate() { return Err(e); }
			if let Some(ref othr_ctr_pty_value) = self.othr_ctr_pty { if let Err(e) = othr_ctr_pty_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ClearingExceptionOrExemption3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ClearingExceptionOrExemption3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPties", skip_serializing_if = "Option::is_none") )]
		pub ctr_pties: Option<ClearingExceptionOrExemption2>,
	}
	
	impl ClearingExceptionOrExemption3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref rsn_value) = self.rsn { if let Err(e) = rsn_value.validate() { return Err(e); } }
			if let Some(ref ctr_pties_value) = self.ctr_pties { if let Err(e) = ctr_pties_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ClearingExemptionException1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum ClearingExemptionException1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "COOP") )]
		CodeCOOP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ENDU") )]
		CodeENDU,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AFFL") )]
		CodeAFFL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NOAL") )]
		CodeNOAL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NORE") )]
		CodeNORE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SMBK") )]
		CodeSMBK,
	}
	
	impl ClearingExemptionException1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ClearingObligationType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum ClearingObligationType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "FLSE") )]
		CodeFLSE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UKWN") )]
		CodeUKWN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TRUE") )]
		CodeTRUE,
	}
	
	impl ClearingObligationType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ClearingPartyAndTime21Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ClearingPartyAndTime21Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dtls", skip_serializing_if = "Option::is_none") )]
		pub dtls: Option<ClearingPartyAndTime22>,
	}
	
	impl ClearingPartyAndTime21Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref rsn_value) = self.rsn { if let Err(e) = rsn_value.validate() { return Err(e); } }
			if let Some(ref dtls_value) = self.dtls { if let Err(e) = dtls_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ClearingPartyAndTime22 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ClearingPartyAndTime22 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CCP", skip_serializing_if = "Option::is_none") )]
		pub ccp: Option<OrganisationIdentification15Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrRctDtTm", skip_serializing_if = "Option::is_none") )]
		pub clr_rct_dt_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrDtTm", skip_serializing_if = "Option::is_none") )]
		pub clr_dt_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrIdr", skip_serializing_if = "Option::is_none") )]
		pub clr_idr: Option<UniqueTransactionIdentifier2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlIdr", skip_serializing_if = "Option::is_none") )]
		pub orgnl_idr: Option<UniqueTransactionIdentifier2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTradRpstryIdr", skip_serializing_if = "Option::is_none") )]
		pub orgnl_trad_rpstry_idr: Option<OrganisationIdentification15Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrAcctOrgn", skip_serializing_if = "Option::is_none") )]
		pub clr_acct_orgn: Option<ClearingAccountType4Code>,
	}
	
	impl ClearingPartyAndTime22 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ccp_value) = self.ccp { if let Err(e) = ccp_value.validate() { return Err(e); } }
			if let Some(ref clr_idr_value) = self.clr_idr { if let Err(e) = clr_idr_value.validate() { return Err(e); } }
			if let Some(ref orgnl_idr_value) = self.orgnl_idr { if let Err(e) = orgnl_idr_value.validate() { return Err(e); } }
			if let Some(ref orgnl_trad_rpstry_idr_value) = self.orgnl_trad_rpstry_idr { if let Err(e) = orgnl_trad_rpstry_idr_value.validate() { return Err(e); } }
			if let Some(ref clr_acct_orgn_value) = self.clr_acct_orgn { if let Err(e) = clr_acct_orgn_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ClearingPartyAndTime22Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ClearingPartyAndTime22Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
		pub rsn: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dtls", skip_serializing_if = "Option::is_none") )]
		pub dtls: Option<ClearingPartyAndTime23>,
	}
	
	impl ClearingPartyAndTime22Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref rsn_value) = self.rsn { if let Err(e) = rsn_value.validate() { return Err(e); } }
			if let Some(ref dtls_value) = self.dtls { if let Err(e) = dtls_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ClearingPartyAndTime23 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ClearingPartyAndTime23 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CCP", skip_serializing_if = "Option::is_none") )]
		pub ccp: Option<OrganisationIdentification15Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrRctDtTm", skip_serializing_if = "Option::is_none") )]
		pub clr_rct_dt_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrDtTm", skip_serializing_if = "Option::is_none") )]
		pub clr_dt_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrIdr", skip_serializing_if = "Option::is_none") )]
		pub clr_idr: Option<UniqueTransactionIdentifier1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlIdr", skip_serializing_if = "Option::is_none") )]
		pub orgnl_idr: Option<UniqueTransactionIdentifier1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTradRpstryIdr", skip_serializing_if = "Option::is_none") )]
		pub orgnl_trad_rpstry_idr: Option<OrganisationIdentification15Choice>,
	}
	
	impl ClearingPartyAndTime23 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ccp_value) = self.ccp { if let Err(e) = ccp_value.validate() { return Err(e); } }
			if let Some(ref clr_idr_value) = self.clr_idr { if let Err(e) = clr_idr_value.validate() { return Err(e); } }
			if let Some(ref orgnl_idr_value) = self.orgnl_idr { if let Err(e) = orgnl_idr_value.validate() { return Err(e); } }
			if let Some(ref orgnl_trad_rpstry_idr_value) = self.orgnl_trad_rpstry_idr { if let Err(e) = orgnl_trad_rpstry_idr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CollateralPortfolioCode6Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CollateralPortfolioCode6Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtfl", skip_serializing_if = "Option::is_none") )]
		pub prtfl: Option<PortfolioCode3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MrgnPrtflCd", skip_serializing_if = "Option::is_none") )]
		pub mrgn_prtfl_cd: Option<MarginPortfolio4>,
	}
	
	impl CollateralPortfolioCode6Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref prtfl_value) = self.prtfl { if let Err(e) = prtfl_value.validate() { return Err(e); } }
			if let Some(ref mrgn_prtfl_cd_value) = self.mrgn_prtfl_cd { if let Err(e) = mrgn_prtfl_cd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CommonTradeDataReport71 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CommonTradeDataReport71 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctData", skip_serializing_if = "Option::is_none") )]
		pub ctrct_data: Option<ContractType15>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxData") )]
		pub tx_data: TradeTransaction50,
	}
	
	impl CommonTradeDataReport71 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ctrct_data_value) = self.ctrct_data { if let Err(e) = ctrct_data_value.validate() { return Err(e); } }
			if let Err(e) = self.tx_data.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// ContractType15 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ContractType15 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctTp", skip_serializing_if = "Option::is_none") )]
		pub ctrct_tp: Option<FinancialInstrumentContractType2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AsstClss", skip_serializing_if = "Option::is_none") )]
		pub asst_clss: Option<ProductType4Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PdctClssfctn", skip_serializing_if = "Option::is_none") )]
		pub pdct_clssfctn: Option<CFIOct2015Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PdctId", skip_serializing_if = "Option::is_none") )]
		pub pdct_id: Option<SecurityIdentification46>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygInstrm", skip_serializing_if = "Option::is_none") )]
		pub undrlyg_instrm: Option<SecurityIdentification41Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygAsstTradgPltfmIdr", skip_serializing_if = "Option::is_none") )]
		pub undrlyg_asst_tradg_pltfm_idr: Option<MICIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygAsstPricSrc", skip_serializing_if = "Option::is_none") )]
		pub undrlyg_asst_pric_src: Option<Max50Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none") )]
		pub sttlm_ccy: Option<CurrencyExchange23>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmCcyScndLeg", skip_serializing_if = "Option::is_none") )]
		pub sttlm_ccy_scnd_leg: Option<CurrencyExchange23>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PlcOfSttlm", skip_serializing_if = "Option::is_none") )]
		pub plc_of_sttlm: Option<CountryCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DerivBasedOnCrptAsst", skip_serializing_if = "Option::is_none") )]
		pub deriv_based_on_crpt_asst: Option<bool>,
	}
	
	impl ContractType15 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ctrct_tp_value) = self.ctrct_tp { if let Err(e) = ctrct_tp_value.validate() { return Err(e); } }
			if let Some(ref asst_clss_value) = self.asst_clss { if let Err(e) = asst_clss_value.validate() { return Err(e); } }
			if let Some(ref pdct_clssfctn_value) = self.pdct_clssfctn { if let Err(e) = pdct_clssfctn_value.validate() { return Err(e); } }
			if let Some(ref pdct_id_value) = self.pdct_id { if let Err(e) = pdct_id_value.validate() { return Err(e); } }
			if let Some(ref undrlyg_instrm_value) = self.undrlyg_instrm { if let Err(e) = undrlyg_instrm_value.validate() { return Err(e); } }
			if let Some(ref undrlyg_asst_tradg_pltfm_idr_value) = self.undrlyg_asst_tradg_pltfm_idr { if let Err(e) = undrlyg_asst_tradg_pltfm_idr_value.validate() { return Err(e); } }
			if let Some(ref undrlyg_asst_pric_src_value) = self.undrlyg_asst_pric_src { if let Err(e) = undrlyg_asst_pric_src_value.validate() { return Err(e); } }
			if let Some(ref sttlm_ccy_value) = self.sttlm_ccy { if let Err(e) = sttlm_ccy_value.validate() { return Err(e); } }
			if let Some(ref sttlm_ccy_scnd_leg_value) = self.sttlm_ccy_scnd_leg { if let Err(e) = sttlm_ccy_scnd_leg_value.validate() { return Err(e); } }
			if let Some(ref plc_of_sttlm_value) = self.plc_of_sttlm { if let Err(e) = plc_of_sttlm_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ContractValuationData8 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ContractValuationData8 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctVal", skip_serializing_if = "Option::is_none") )]
		pub ctrct_val: Option<AmountAndDirection109>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TmStmp", skip_serializing_if = "Option::is_none") )]
		pub tm_stmp: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<ValuationType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dlta", skip_serializing_if = "Option::is_none") )]
		pub dlta: Option<f64>,
	}
	
	impl ContractValuationData8 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ctrct_val_value) = self.ctrct_val { if let Err(e) = ctrct_val_value.validate() { return Err(e); } }
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
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
	
	
	// CounterpartySpecificData36 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CounterpartySpecificData36 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPty") )]
		pub ctr_pty: TradeCounterpartyReport20,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Valtn", skip_serializing_if = "Option::is_none") )]
		pub valtn: Option<ContractValuationData8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgTmStmp", skip_serializing_if = "Option::is_none") )]
		pub rptg_tm_stmp: Option<String>,
	}
	
	impl CounterpartySpecificData36 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.ctr_pty.validate() { return Err(e); }
			if let Some(ref valtn_value) = self.valtn { if let Err(e) = valtn_value.validate() { return Err(e); } }
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
	
	
	// CreditDerivative4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CreditDerivative4 {
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
		#[cfg_attr( feature = "derive_serde", serde(rename = "Trch", skip_serializing_if = "Option::is_none") )]
		pub trch: Option<TrancheIndicator3Choice>,
	}
	
	impl CreditDerivative4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref snrty_value) = self.snrty { if let Err(e) = snrty_value.validate() { return Err(e); } }
			if let Some(ref ref_pty_value) = self.ref_pty { if let Err(e) = ref_pty_value.validate() { return Err(e); } }
			if let Some(ref pmt_frqcy_value) = self.pmt_frqcy { if let Err(e) = pmt_frqcy_value.validate() { return Err(e); } }
			if let Some(ref clctn_bsis_value) = self.clctn_bsis { if let Err(e) = clctn_bsis_value.validate() { return Err(e); } }
			if let Some(ref trch_value) = self.trch { if let Err(e) = trch_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CurrencyExchange22 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CurrencyExchange22 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DlvrblCrossCcy", skip_serializing_if = "Option::is_none") )]
		pub dlvrbl_cross_ccy: Option<ActiveOrHistoricCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XchgRate", skip_serializing_if = "Option::is_none") )]
		pub xchg_rate: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FwdXchgRate", skip_serializing_if = "Option::is_none") )]
		pub fwd_xchg_rate: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XchgRateBsis", skip_serializing_if = "Option::is_none") )]
		pub xchg_rate_bsis: Option<ExchangeRateBasis1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FxgDt", skip_serializing_if = "Option::is_none") )]
		pub fxg_dt: Option<String>,
	}
	
	impl CurrencyExchange22 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref dlvrbl_cross_ccy_value) = self.dlvrbl_cross_ccy { if let Err(e) = dlvrbl_cross_ccy_value.validate() { return Err(e); } }
			if let Some(ref xchg_rate_bsis_value) = self.xchg_rate_bsis { if let Err(e) = xchg_rate_bsis_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CurrencyExchange23 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CurrencyExchange23 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
		pub ccy: ActiveOrHistoricCurrencyCode,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XchgRate", skip_serializing_if = "Option::is_none") )]
		pub xchg_rate: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FwdXchgRate", skip_serializing_if = "Option::is_none") )]
		pub fwd_xchg_rate: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XchgRateBsis", skip_serializing_if = "Option::is_none") )]
		pub xchg_rate_bsis: Option<ExchangeRateBasis1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FxgDt", skip_serializing_if = "Option::is_none") )]
		pub fxg_dt: Option<String>,
	}
	
	impl CurrencyExchange23 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.ccy.validate() { return Err(e); }
			if let Some(ref xchg_rate_bsis_value) = self.xchg_rate_bsis { if let Err(e) = xchg_rate_bsis_value.validate() { return Err(e); } }
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
	
	
	// DateAndDateTime2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// DatePeriod1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DatePeriod1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt", skip_serializing_if = "Option::is_none") )]
		pub fr_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt") )]
		pub to_dt: String,
	}
	
	impl DatePeriod1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	
	
	// DeliveryInterconnectionPoint1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DeliveryInterconnectionPoint1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<EICIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max52Text>,
	}
	
	impl DeliveryInterconnectionPoint1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DerivativeEvent6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DerivativeEvent6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<DerivativeEventType3Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<EventIdentifier1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TmStmp", skip_serializing_if = "Option::is_none") )]
		pub tm_stmp: Option<DateAndDateTime2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AmdmntInd", skip_serializing_if = "Option::is_none") )]
		pub amdmnt_ind: Option<bool>,
	}
	
	impl DerivativeEvent6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			if let Some(ref tm_stmp_value) = self.tm_stmp { if let Err(e) = tm_stmp_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DerivativeEventType3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// DerivativesTradeReportV04 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DerivativesTradeReportV04 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptHdr") )]
		pub rpt_hdr: TradeReportHeader4,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradData") )]
		pub trad_data: TradeData59Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl DerivativesTradeReportV04 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rpt_hdr.validate() { return Err(e); }
			if let Err(e) = self.trad_data.validate() { return Err(e); }
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
	
	
	// DisseminationData1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DisseminationData1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DssmntnIdr") )]
		pub dssmntn_idr: Max52Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlDssmntnIdr", skip_serializing_if = "Option::is_none") )]
		pub orgnl_dssmntn_idr: Option<Max52Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TmStmp") )]
		pub tm_stmp: String,
	}
	
	impl DisseminationData1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.dssmntn_idr.validate() { return Err(e); }
			if let Some(ref orgnl_dssmntn_idr_value) = self.orgnl_dssmntn_idr { if let Err(e) = orgnl_dssmntn_idr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// DurationType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum DurationType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
		CodeYEAR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WEEK") )]
		CodeWEEK,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SEAS") )]
		CodeSEAS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QURT") )]
		CodeQURT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
		CodeMNTH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MNUT") )]
		CodeMNUT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HOUR") )]
		CodeHOUR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DASD") )]
		CodeDASD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
	}
	
	impl DurationType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// EICIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct EICIdentifier {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub eic_identifier: String,
	}
	
	impl EICIdentifier {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[A-Z0-9\\-]{16}").unwrap();
			if !pattern.is_match(&self.eic_identifier) {
				return Err(ValidationError::new(1005, "eic_identifier does not match the required pattern".to_string()));
			}
			Ok(())
		}
	}
	
	
	// EmbeddedType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum EmbeddedType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CANC") )]
		CodeCANC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EXTD") )]
		CodeEXTD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OPET") )]
		CodeOPET,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MDET") )]
		CodeMDET,
	}
	
	impl EmbeddedType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	
	
	// EnergyDeliveryAttribute10 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct EnergyDeliveryAttribute10 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DlvryIntrvl", skip_serializing_if = "Option::is_none") )]
		pub dlvry_intrvl: Option<Vec<TimePeriodDetails1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DlvryDt", skip_serializing_if = "Option::is_none") )]
		pub dlvry_dt: Option<DatePeriod1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Drtn", skip_serializing_if = "Option::is_none") )]
		pub drtn: Option<DurationType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WkDay", skip_serializing_if = "Option::is_none") )]
		pub wk_day: Option<Vec<WeekDay3Code>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DlvryCpcty", skip_serializing_if = "Option::is_none") )]
		pub dlvry_cpcty: Option<Quantity47Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "QtyUnit", skip_serializing_if = "Option::is_none") )]
		pub qty_unit: Option<EnergyQuantityUnit2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PricTmIntrvlQty", skip_serializing_if = "Option::is_none") )]
		pub pric_tm_intrvl_qty: Option<AmountAndDirection106>,
	}
	
	impl EnergyDeliveryAttribute10 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref dlvry_intrvl_vec) = self.dlvry_intrvl { for item in dlvry_intrvl_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref dlvry_dt_value) = self.dlvry_dt { if let Err(e) = dlvry_dt_value.validate() { return Err(e); } }
			if let Some(ref drtn_value) = self.drtn { if let Err(e) = drtn_value.validate() { return Err(e); } }
			if let Some(ref wk_day_vec) = self.wk_day { for item in wk_day_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref dlvry_cpcty_value) = self.dlvry_cpcty { if let Err(e) = dlvry_cpcty_value.validate() { return Err(e); } }
			if let Some(ref qty_unit_value) = self.qty_unit { if let Err(e) = qty_unit_value.validate() { return Err(e); } }
			if let Some(ref pric_tm_intrvl_qty_value) = self.pric_tm_intrvl_qty { if let Err(e) = pric_tm_intrvl_qty_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// EnergyLoadType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum EnergyLoadType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "BSLD") )]
		CodeBSLD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GASD") )]
		CodeGASD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HABH") )]
		CodeHABH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OFFP") )]
		CodeOFFP,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PKLD") )]
		CodePKLD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SHPD") )]
		CodeSHPD,
	}
	
	impl EnergyLoadType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// EnergyQuantityUnit2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct EnergyQuantityUnit2Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<EnergyQuantityUnit2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max52Text>,
	}
	
	impl EnergyQuantityUnit2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// EnergyQuantityUnit2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum EnergyQuantityUnit2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "BTUD") )]
		CodeBTUD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CMPD") )]
		CodeCMPD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GJDD") )]
		CodeGJDD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GWAT") )]
		CodeGWAT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GWHD") )]
		CodeGWHD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "GWHH") )]
		CodeGWHH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HMJD") )]
		CodeHMJD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "KTMD") )]
		CodeKTMD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "KWAT") )]
		CodeKWAT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "KWHD") )]
		CodeKWHD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "KWHH") )]
		CodeKWHH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MCMD") )]
		CodeMCMD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MJDD") )]
		CodeMJDD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MBTD") )]
		CodeMBTD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MMJD") )]
		CodeMMJD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MTMD") )]
		CodeMTMD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MWAT") )]
		CodeMWAT,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MWHD") )]
		CodeMWHD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MWHH") )]
		CodeMWHH,
		#[cfg_attr( feature = "derive_serde", serde(rename = "THMD") )]
		CodeTHMD,
	}
	
	impl EnergyQuantityUnit2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// EnergySpecificAttribute9 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct EnergySpecificAttribute9 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DlvryPtOrZone", skip_serializing_if = "Option::is_none") )]
		pub dlvry_pt_or_zone: Option<Vec<DeliveryInterconnectionPoint1Choice>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrCnnctnPt", skip_serializing_if = "Option::is_none") )]
		pub intr_cnnctn_pt: Option<DeliveryInterconnectionPoint1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LdTp", skip_serializing_if = "Option::is_none") )]
		pub ld_tp: Option<EnergyLoadType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DlvryAttr", skip_serializing_if = "Option::is_none") )]
		pub dlvry_attr: Option<Vec<EnergyDeliveryAttribute10>>,
	}
	
	impl EnergySpecificAttribute9 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref dlvry_pt_or_zone_vec) = self.dlvry_pt_or_zone { for item in dlvry_pt_or_zone_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref intr_cnnctn_pt_value) = self.intr_cnnctn_pt { if let Err(e) = intr_cnnctn_pt_value.validate() { return Err(e); } }
			if let Some(ref ld_tp_value) = self.ld_tp { if let Err(e) = ld_tp_value.validate() { return Err(e); } }
			if let Some(ref dlvry_attr_vec) = self.dlvry_attr { for item in dlvry_attr_vec { if let Err(e) = item.validate() { return Err(e); } } }
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
	
	
	// EventIdentifier1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct EventIdentifier1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "EvtIdr", skip_serializing_if = "Option::is_none") )]
		pub evt_idr: Option<UTIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstTradRskRdctnIdr", skip_serializing_if = "Option::is_none") )]
		pub pst_trad_rsk_rdctn_idr: Option<PostTradeRiskReductionIdentifier1>,
	}
	
	impl EventIdentifier1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref evt_idr_value) = self.evt_idr { if let Err(e) = evt_idr_value.validate() { return Err(e); } }
			if let Some(ref pst_trad_rsk_rdctn_idr_value) = self.pst_trad_rsk_rdctn_idr { if let Err(e) = pst_trad_rsk_rdctn_idr_value.validate() { return Err(e); } }
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
	
	
	// ExerciseDate1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ExerciseDate1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrstExrcDt", skip_serializing_if = "Option::is_none") )]
		pub frst_exrc_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PdgDtAplbl", skip_serializing_if = "Option::is_none") )]
		pub pdg_dt_aplbl: Option<PriceStatus2Code>,
	}
	
	impl ExerciseDate1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref pdg_dt_aplbl_value) = self.pdg_dt_aplbl { if let Err(e) = pdg_dt_aplbl_value.validate() { return Err(e); } }
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
	
	
	// FinancialInstrumentQuantity32Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FinancialInstrumentQuantity32Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Unit", skip_serializing_if = "Option::is_none") )]
		pub unit: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NmnlVal", skip_serializing_if = "Option::is_none") )]
		pub nmnl_val: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MntryVal", skip_serializing_if = "Option::is_none") )]
		pub mntry_val: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
	}
	
	impl FinancialInstrumentQuantity32Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref nmnl_val_value) = self.nmnl_val { if let Err(e) = nmnl_val_value.validate() { return Err(e); } }
			if let Some(ref mntry_val_value) = self.mntry_val { if let Err(e) = mntry_val_value.validate() { return Err(e); } }
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
	
	
	// FixedRate10 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FixedRate10 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
		pub rate: Option<SecuritiesTransactionPrice14Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DayCnt", skip_serializing_if = "Option::is_none") )]
		pub day_cnt: Option<InterestComputationMethodFormat7>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtFrqcy", skip_serializing_if = "Option::is_none") )]
		pub pmt_frqcy: Option<InterestRateFrequency3Choice>,
	}
	
	impl FixedRate10 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref rate_value) = self.rate { if let Err(e) = rate_value.validate() { return Err(e); } }
			if let Some(ref day_cnt_value) = self.day_cnt { if let Err(e) = day_cnt_value.validate() { return Err(e); } }
			if let Some(ref pmt_frqcy_value) = self.pmt_frqcy { if let Err(e) = pmt_frqcy_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FloatingRate13 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FloatingRate13 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<ISINOct2015Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
		pub rate: Option<FloatingRateIdentification8Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RefPrd", skip_serializing_if = "Option::is_none") )]
		pub ref_prd: Option<InterestRateContractTerm4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sprd", skip_serializing_if = "Option::is_none") )]
		pub sprd: Option<SecuritiesTransactionPrice20Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DayCnt", skip_serializing_if = "Option::is_none") )]
		pub day_cnt: Option<InterestComputationMethodFormat7>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtFrqcy", skip_serializing_if = "Option::is_none") )]
		pub pmt_frqcy: Option<InterestRateFrequency3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RstFrqcy", skip_serializing_if = "Option::is_none") )]
		pub rst_frqcy: Option<InterestRateFrequency3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NxtFltgRst", skip_serializing_if = "Option::is_none") )]
		pub nxt_fltg_rst: Option<ResetDateAndValue1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LastFltgRst", skip_serializing_if = "Option::is_none") )]
		pub last_fltg_rst: Option<ResetDateAndValue1>,
	}
	
	impl FloatingRate13 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref id_value) = self.id { if let Err(e) = id_value.validate() { return Err(e); } }
			if let Some(ref nm_value) = self.nm { if let Err(e) = nm_value.validate() { return Err(e); } }
			if let Some(ref rate_value) = self.rate { if let Err(e) = rate_value.validate() { return Err(e); } }
			if let Some(ref ref_prd_value) = self.ref_prd { if let Err(e) = ref_prd_value.validate() { return Err(e); } }
			if let Some(ref sprd_value) = self.sprd { if let Err(e) = sprd_value.validate() { return Err(e); } }
			if let Some(ref day_cnt_value) = self.day_cnt { if let Err(e) = day_cnt_value.validate() { return Err(e); } }
			if let Some(ref pmt_frqcy_value) = self.pmt_frqcy { if let Err(e) = pmt_frqcy_value.validate() { return Err(e); } }
			if let Some(ref rst_frqcy_value) = self.rst_frqcy { if let Err(e) = rst_frqcy_value.validate() { return Err(e); } }
			if let Some(ref nxt_fltg_rst_value) = self.nxt_fltg_rst { if let Err(e) = nxt_fltg_rst_value.validate() { return Err(e); } }
			if let Some(ref last_fltg_rst_value) = self.last_fltg_rst { if let Err(e) = last_fltg_rst_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// FloatingRateIdentification8Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FloatingRateIdentification8Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<ExternalBenchmarkCurveName1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max350Text>,
	}
	
	impl FloatingRateIdentification8Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cd_value) = self.cd { if let Err(e) = cd_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
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
	
	
	// Frequency19Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// GenericIdentification179 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericIdentification179 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max52Text,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<Max35Text>,
	}
	
	impl GenericIdentification179 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
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
	
	
	// ISOTime ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct ISOTime {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub iso_time: String,
	}
	
	impl ISOTime {
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
	
	
	// InterestComputationMethod4Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum InterestComputationMethod4Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "A004") )]
		CodeA004,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A019") )]
		CodeA019,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A017") )]
		CodeA017,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A005") )]
		CodeA005,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A009") )]
		CodeA009,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A014") )]
		CodeA014,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A010") )]
		CodeA010,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A006") )]
		CodeA006,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A008") )]
		CodeA008,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A015") )]
		CodeA015,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A018") )]
		CodeA018,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A011") )]
		CodeA011,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A001") )]
		CodeA001,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A002") )]
		CodeA002,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A003") )]
		CodeA003,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A012") )]
		CodeA012,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A013") )]
		CodeA013,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A007") )]
		CodeA007,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A016") )]
		CodeA016,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NARR") )]
		CodeNARR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "A020") )]
		CodeA020,
	}
	
	impl InterestComputationMethod4Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// InterestComputationMethodFormat7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct InterestComputationMethodFormat7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
		pub cd: InterestComputationMethod4Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nrrtv", skip_serializing_if = "Option::is_none") )]
		pub nrrtv: Option<Max1000Text>,
	}
	
	impl InterestComputationMethodFormat7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.cd.validate() { return Err(e); }
			if let Some(ref nrrtv_value) = self.nrrtv { if let Err(e) = nrrtv_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// InterestRate33Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct InterestRate33Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Fxd", skip_serializing_if = "Option::is_none") )]
		pub fxd: Option<FixedRate10>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Fltg", skip_serializing_if = "Option::is_none") )]
		pub fltg: Option<FloatingRate13>,
	}
	
	impl InterestRate33Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref fxd_value) = self.fxd { if let Err(e) = fxd_value.validate() { return Err(e); } }
			if let Some(ref fltg_value) = self.fltg { if let Err(e) = fltg_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// InterestRateContractTerm4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct InterestRateContractTerm4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Unit", skip_serializing_if = "Option::is_none") )]
		pub unit: Option<Frequency13Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val", skip_serializing_if = "Option::is_none") )]
		pub val: Option<f64>,
	}
	
	impl InterestRateContractTerm4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref unit_value) = self.unit { if let Err(e) = unit_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// InterestRateFrequency3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct InterestRateFrequency3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Term", skip_serializing_if = "Option::is_none") )]
		pub term: Option<InterestRateContractTerm4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<Max52Text>,
	}
	
	impl InterestRateFrequency3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref term_value) = self.term { if let Err(e) = term_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// InterestRateLegs14 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct InterestRateLegs14 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrstLeg", skip_serializing_if = "Option::is_none") )]
		pub frst_leg: Option<InterestRate33Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ScndLeg", skip_serializing_if = "Option::is_none") )]
		pub scnd_leg: Option<InterestRate33Choice>,
	}
	
	impl InterestRateLegs14 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref frst_leg_value) = self.frst_leg { if let Err(e) = frst_leg_value.validate() { return Err(e); } }
			if let Some(ref scnd_leg_value) = self.scnd_leg { if let Err(e) = scnd_leg_value.validate() { return Err(e); } }
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
	
	
	// MarginPortfolio4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MarginPortfolio4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "InitlMrgnPrtflCd", skip_serializing_if = "Option::is_none") )]
		pub initl_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "VartnMrgnPrtflCd", skip_serializing_if = "Option::is_none") )]
		pub vartn_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
	}
	
	impl MarginPortfolio4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref initl_mrgn_prtfl_cd_value) = self.initl_mrgn_prtfl_cd { if let Err(e) = initl_mrgn_prtfl_cd_value.validate() { return Err(e); } }
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
	
	
	// Max5NumericText ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct Max5NumericText {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub max5_numeric_text: String,
	}
	
	impl Max5NumericText {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[0-9]{1,5}").unwrap();
			if !pattern.is_match(&self.max5_numeric_text) {
				return Err(ValidationError::new(1005, "max5_numeric_text does not match the required pattern".to_string()));
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
	
	
	// ModificationLevel1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// NonClearingReason2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct NonClearingReason2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrXmptnXcptn") )]
		pub clr_xmptn_xcptn: Vec<ClearingExemptionException1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NonClrRsnInf", skip_serializing_if = "Option::is_none") )]
		pub non_clr_rsn_inf: Option<Max350Text>,
	}
	
	impl NonClearingReason2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			for item in &self.clr_xmptn_xcptn { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref non_clr_rsn_inf_value) = self.non_clr_rsn_inf { if let Err(e) = non_clr_rsn_inf_value.validate() { return Err(e); } }
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
	
	
	// NotionalAmount5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// OptionBarrierLevel1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OptionBarrierLevel1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sngl", skip_serializing_if = "Option::is_none") )]
		pub sngl: Option<SecuritiesTransactionPrice23Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Mltpl", skip_serializing_if = "Option::is_none") )]
		pub mltpl: Option<OptionMultipleBarrierLevels1>,
	}
	
	impl OptionBarrierLevel1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref sngl_value) = self.sngl { if let Err(e) = sngl_value.validate() { return Err(e); } }
			if let Some(ref mltpl_value) = self.mltpl { if let Err(e) = mltpl_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// OptionMultipleBarrierLevels1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OptionMultipleBarrierLevels1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "LwrLvl") )]
		pub lwr_lvl: SecuritiesTransactionPrice23Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UpperLvl") )]
		pub upper_lvl: SecuritiesTransactionPrice23Choice,
	}
	
	impl OptionMultipleBarrierLevels1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.lwr_lvl.validate() { return Err(e); }
			if let Err(e) = self.upper_lvl.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// OptionOrSwaption11 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OptionOrSwaption11 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<OptionType2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MbddTp", skip_serializing_if = "Option::is_none") )]
		pub mbdd_tp: Option<EmbeddedType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ExrcStyle", skip_serializing_if = "Option::is_none") )]
		pub exrc_style: Option<Vec<OptionStyle6Code>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ExrcDt", skip_serializing_if = "Option::is_none") )]
		pub exrc_dt: Option<ExerciseDate1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StrkPric", skip_serializing_if = "Option::is_none") )]
		pub strk_pric: Option<SecuritiesTransactionPrice17Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "StrkPricSchdl", skip_serializing_if = "Option::is_none") )]
		pub strk_pric_schdl: Option<Vec<Schedule4>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CallAmt", skip_serializing_if = "Option::is_none") )]
		pub call_amt: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PutAmt", skip_serializing_if = "Option::is_none") )]
		pub put_amt: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrmAmt", skip_serializing_if = "Option::is_none") )]
		pub prm_amt: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrmPmtDt", skip_serializing_if = "Option::is_none") )]
		pub prm_pmt_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MtrtyDtOfUndrlyg", skip_serializing_if = "Option::is_none") )]
		pub mtrty_dt_of_undrlyg: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BrrrLvls", skip_serializing_if = "Option::is_none") )]
		pub brrr_lvls: Option<OptionBarrierLevel1Choice>,
	}
	
	impl OptionOrSwaption11 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
			if let Some(ref mbdd_tp_value) = self.mbdd_tp { if let Err(e) = mbdd_tp_value.validate() { return Err(e); } }
			if let Some(ref exrc_style_vec) = self.exrc_style { for item in exrc_style_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref exrc_dt_value) = self.exrc_dt { if let Err(e) = exrc_dt_value.validate() { return Err(e); } }
			if let Some(ref strk_pric_value) = self.strk_pric { if let Err(e) = strk_pric_value.validate() { return Err(e); } }
			if let Some(ref strk_pric_schdl_vec) = self.strk_pric_schdl { for item in strk_pric_schdl_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref call_amt_value) = self.call_amt { if let Err(e) = call_amt_value.validate() { return Err(e); } }
			if let Some(ref put_amt_value) = self.put_amt { if let Err(e) = put_amt_value.validate() { return Err(e); } }
			if let Some(ref prm_amt_value) = self.prm_amt { if let Err(e) = prm_amt_value.validate() { return Err(e); } }
			if let Some(ref brrr_lvls_value) = self.brrr_lvls { if let Err(e) = brrr_lvls_value.validate() { return Err(e); } }
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
	
	
	// OptionStyle6Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum OptionStyle6Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "EURO") )]
		CodeEURO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BERM") )]
		CodeBERM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ASIA") )]
		CodeASIA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AMER") )]
		CodeAMER,
	}
	
	impl OptionStyle6Code {
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
	
	
	// OtherPayment5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OtherPayment5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtAmt", skip_serializing_if = "Option::is_none") )]
		pub pmt_amt: Option<AmountAndDirection106>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTp", skip_serializing_if = "Option::is_none") )]
		pub pmt_tp: Option<PaymentType5Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtDt", skip_serializing_if = "Option::is_none") )]
		pub pmt_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtPyer", skip_serializing_if = "Option::is_none") )]
		pub pmt_pyer: Option<PartyIdentification236Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PmtRcvr", skip_serializing_if = "Option::is_none") )]
		pub pmt_rcvr: Option<PartyIdentification236Choice>,
	}
	
	impl OtherPayment5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref pmt_amt_value) = self.pmt_amt { if let Err(e) = pmt_amt_value.validate() { return Err(e); } }
			if let Some(ref pmt_tp_value) = self.pmt_tp { if let Err(e) = pmt_tp_value.validate() { return Err(e); } }
			if let Some(ref pmt_pyer_value) = self.pmt_pyer { if let Err(e) = pmt_pyer_value.validate() { return Err(e); } }
			if let Some(ref pmt_rcvr_value) = self.pmt_rcvr { if let Err(e) = pmt_rcvr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PTRREvent2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PTRREvent2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tchnq") )]
		pub tchnq: RiskReductionService1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SvcPrvdr", skip_serializing_if = "Option::is_none") )]
		pub svc_prvdr: Option<OrganisationIdentification15Choice>,
	}
	
	impl PTRREvent2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tchnq.validate() { return Err(e); }
			if let Some(ref svc_prvdr_value) = self.svc_prvdr { if let Err(e) = svc_prvdr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Package4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Package4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CmplxTradId", skip_serializing_if = "Option::is_none") )]
		pub cmplx_trad_id: Option<Max100Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FxSwpLkId", skip_serializing_if = "Option::is_none") )]
		pub fx_swp_lk_id: Option<Max100Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pric", skip_serializing_if = "Option::is_none") )]
		pub pric: Option<SecuritiesTransactionPrice17Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Sprd", skip_serializing_if = "Option::is_none") )]
		pub sprd: Option<SecuritiesTransactionPrice20Choice>,
	}
	
	impl Package4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref cmplx_trad_id_value) = self.cmplx_trad_id { if let Err(e) = cmplx_trad_id_value.validate() { return Err(e); } }
			if let Some(ref fx_swp_lk_id_value) = self.fx_swp_lk_id { if let Err(e) = fx_swp_lk_id_value.validate() { return Err(e); } }
			if let Some(ref pric_value) = self.pric { if let Err(e) = pric_value.validate() { return Err(e); } }
			if let Some(ref sprd_value) = self.sprd { if let Err(e) = sprd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Pagination1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Pagination1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PgNb") )]
		pub pg_nb: Max5NumericText,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LastPgInd") )]
		pub last_pg_ind: bool,
	}
	
	impl Pagination1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.pg_nb.validate() { return Err(e); }
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
	
	
	// PaperCommodityRecoveredPaper3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaperCommodityRecoveredPaper3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType8Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType50Code>,
	}
	
	impl PaperCommodityRecoveredPaper3 {
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
	
	
	// PlusOrMinusIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// PostTradeRiskReductionIdentifier1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PostTradeRiskReductionIdentifier1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Strr") )]
		pub strr: LEIIdentifier,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: Max52Text,
	}
	
	impl PostTradeRiskReductionIdentifier1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.strr.validate() { return Err(e); }
			if let Err(e) = self.id.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// PriceData2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PriceData2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pric", skip_serializing_if = "Option::is_none") )]
		pub pric: Option<SecuritiesTransactionPrice17Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchdlPrd", skip_serializing_if = "Option::is_none") )]
		pub schdl_prd: Option<Vec<Schedule1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none") )]
		pub unit_of_measr: Option<UnitOfMeasure8Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PricMltplr", skip_serializing_if = "Option::is_none") )]
		pub pric_mltplr: Option<f64>,
	}
	
	impl PriceData2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref pric_value) = self.pric { if let Err(e) = pric_value.validate() { return Err(e); } }
			if let Some(ref schdl_prd_vec) = self.schdl_prd { for item in schdl_prd_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref unit_of_measr_value) = self.unit_of_measr { if let Err(e) = unit_of_measr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PriceStatus1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// PriceStatus2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum PriceStatus2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "PNDG") )]
		CodePNDG,
	}
	
	impl PriceStatus2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	
	
	// Quantity47Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Quantity47Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Qty", skip_serializing_if = "Option::is_none") )]
		pub qty: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
		pub desc: Option<Max52Text>,
	}
	
	impl Quantity47Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref desc_value) = self.desc { if let Err(e) = desc_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// QuantityOrTerm1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Reconciliation3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum Reconciliation3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "DPRW") )]
		CodeDPRW,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DPRV") )]
		CodeDPRV,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DSMA") )]
		CodeDSMA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DSNM") )]
		CodeDSNM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NORE") )]
		CodeNORE,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SSMA") )]
		CodeSSMA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SSPA") )]
		CodeSSPA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SPRW") )]
		CodeSPRW,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SPRV") )]
		CodeSPRV,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SSUN") )]
		CodeSSUN,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SSNE") )]
		CodeSSNE,
	}
	
	impl Reconciliation3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	
	
	// ResetDateAndValue1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ResetDateAndValue1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dt") )]
		pub dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val", skip_serializing_if = "Option::is_none") )]
		pub val: Option<f64>,
	}
	
	impl ResetDateAndValue1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// RiskReductionService1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum RiskReductionService1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "NORR") )]
		CodeNORR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PWOS") )]
		CodePWOS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
		CodeOTHR,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PRBM") )]
		CodePRBM,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PWAS") )]
		CodePWAS,
	}
	
	impl RiskReductionService1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// Schedule1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Schedule1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "UadjstdFctvDt") )]
		pub uadjstd_fctv_dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UadjstdEndDt", skip_serializing_if = "Option::is_none") )]
		pub uadjstd_end_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pric") )]
		pub pric: SecuritiesTransactionPrice17Choice,
	}
	
	impl Schedule1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.pric.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// Schedule10 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// Schedule4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Schedule4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "UadjstdFctvDt") )]
		pub uadjstd_fctv_dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UadjstdEndDt", skip_serializing_if = "Option::is_none") )]
		pub uadjstd_end_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pric") )]
		pub pric: SecuritiesTransactionPrice17Choice,
	}
	
	impl Schedule4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.pric.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// SecuritiesTransactionPrice14Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SecuritiesTransactionPrice14Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
		pub rate: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dcml", skip_serializing_if = "Option::is_none") )]
		pub dcml: Option<f64>,
	}
	
	impl SecuritiesTransactionPrice14Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// SecuritiesTransactionPrice17Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SecuritiesTransactionPrice17Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MntryVal", skip_serializing_if = "Option::is_none") )]
		pub mntry_val: Option<AmountAndDirection106>,
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
	
	impl SecuritiesTransactionPrice17Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref mntry_val_value) = self.mntry_val { if let Err(e) = mntry_val_value.validate() { return Err(e); } }
			if let Some(ref pdg_pric_value) = self.pdg_pric { if let Err(e) = pdg_pric_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SecuritiesTransactionPrice20Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SecuritiesTransactionPrice20Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MntryVal", skip_serializing_if = "Option::is_none") )]
		pub mntry_val: Option<AmountAndDirection106>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pctg", skip_serializing_if = "Option::is_none") )]
		pub pctg: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dcml", skip_serializing_if = "Option::is_none") )]
		pub dcml: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BsisPtSprd", skip_serializing_if = "Option::is_none") )]
		pub bsis_pt_sprd: Option<f64>,
	}
	
	impl SecuritiesTransactionPrice20Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref mntry_val_value) = self.mntry_val { if let Err(e) = mntry_val_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SecuritiesTransactionPrice23Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SecuritiesTransactionPrice23Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MntryVal", skip_serializing_if = "Option::is_none") )]
		pub mntry_val: Option<AmountAndDirection106>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Unit", skip_serializing_if = "Option::is_none") )]
		pub unit: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pctg", skip_serializing_if = "Option::is_none") )]
		pub pctg: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Yld", skip_serializing_if = "Option::is_none") )]
		pub yld: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dcml", skip_serializing_if = "Option::is_none") )]
		pub dcml: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<SecuritiesTransactionPrice5>,
	}
	
	impl SecuritiesTransactionPrice23Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref mntry_val_value) = self.mntry_val { if let Err(e) = mntry_val_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SecuritiesTransactionPrice5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// SecurityIdentification46 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SecurityIdentification46 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
		pub isin: Option<ISINOct2015Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none") )]
		pub unq_pdct_idr: Option<UniqueProductIdentifier2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AltrntvInstrmId", skip_serializing_if = "Option::is_none") )]
		pub altrntv_instrm_id: Option<Max105Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PdctDesc", skip_serializing_if = "Option::is_none") )]
		pub pdct_desc: Option<Max1000Text>,
	}
	
	impl SecurityIdentification46 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref isin_value) = self.isin { if let Err(e) = isin_value.validate() { return Err(e); } }
			if let Some(ref unq_pdct_idr_value) = self.unq_pdct_idr { if let Err(e) = unq_pdct_idr_value.validate() { return Err(e); } }
			if let Some(ref altrntv_instrm_id_value) = self.altrntv_instrm_id { if let Err(e) = altrntv_instrm_id_value.validate() { return Err(e); } }
			if let Some(ref pdct_desc_value) = self.pdct_desc { if let Err(e) = pdct_desc_value.validate() { return Err(e); } }
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
	
	
	// TechnicalAttributes5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TechnicalAttributes5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none") )]
		pub tech_rcrd_id: Option<Max140Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RcncltnFlg", skip_serializing_if = "Option::is_none") )]
		pub rcncltn_flg: Option<Reconciliation3Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptRctTmStmp", skip_serializing_if = "Option::is_none") )]
		pub rpt_rct_tm_stmp: Option<String>,
	}
	
	impl TechnicalAttributes5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tech_rcrd_id_value) = self.tech_rcrd_id { if let Err(e) = tech_rcrd_id_value.validate() { return Err(e); } }
			if let Some(ref rcncltn_flg_value) = self.rcncltn_flg { if let Err(e) = rcncltn_flg_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TimePeriodDetails1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TimePeriodDetails1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrTm") )]
		pub fr_tm: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ToTm", skip_serializing_if = "Option::is_none") )]
		pub to_tm: Option<String>,
	}
	
	impl TimePeriodDetails1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// TradeClearing11 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeClearing11 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrOblgtn", skip_serializing_if = "Option::is_none") )]
		pub clr_oblgtn: Option<ClearingObligationType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSts", skip_serializing_if = "Option::is_none") )]
		pub clr_sts: Option<Cleared23Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntraGrp", skip_serializing_if = "Option::is_none") )]
		pub intra_grp: Option<bool>,
	}
	
	impl TradeClearing11 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref clr_oblgtn_value) = self.clr_oblgtn { if let Err(e) = clr_oblgtn_value.validate() { return Err(e); } }
			if let Some(ref clr_sts_value) = self.clr_sts { if let Err(e) = clr_sts_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TradeConfirmation4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeConfirmation4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Confd", skip_serializing_if = "Option::is_none") )]
		pub confd: Option<TradeConfirmation5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NonConfd", skip_serializing_if = "Option::is_none") )]
		pub non_confd: Option<TradeNonConfirmation1>,
	}
	
	impl TradeConfirmation4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref confd_value) = self.confd { if let Err(e) = confd_value.validate() { return Err(e); } }
			if let Some(ref non_confd_value) = self.non_confd { if let Err(e) = non_confd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TradeConfirmation5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeConfirmation5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: TradeConfirmationType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TmStmp", skip_serializing_if = "Option::is_none") )]
		pub tm_stmp: Option<String>,
	}
	
	impl TradeConfirmation5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// TradeConfirmationType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum TradeConfirmationType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ECNF") )]
		CodeECNF,
		#[cfg_attr( feature = "derive_serde", serde(rename = "YCNF") )]
		CodeYCNF,
	}
	
	impl TradeConfirmationType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// TradeConfirmationType2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum TradeConfirmationType2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "NCNF") )]
		CodeNCNF,
	}
	
	impl TradeConfirmationType2Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	
	
	// TradeData43 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeData43 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtySpcfcData") )]
		pub ctr_pty_spcfc_data: Vec<CounterpartySpecificData36>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CmonTradData") )]
		pub cmon_trad_data: CommonTradeDataReport71,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Lvl", skip_serializing_if = "Option::is_none") )]
		pub lvl: Option<ModificationLevel1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TechAttrbts", skip_serializing_if = "Option::is_none") )]
		pub tech_attrbts: Option<TechnicalAttributes5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PblcDssmntnData", skip_serializing_if = "Option::is_none") )]
		pub pblc_dssmntn_data: Option<DisseminationData1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl TradeData43 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			for item in &self.ctr_pty_spcfc_data { if let Err(e) = item.validate() { return Err(e); } }
			if let Err(e) = self.cmon_trad_data.validate() { return Err(e); }
			if let Some(ref lvl_value) = self.lvl { if let Err(e) = lvl_value.validate() { return Err(e); } }
			if let Some(ref tech_attrbts_value) = self.tech_attrbts { if let Err(e) = tech_attrbts_value.validate() { return Err(e); } }
			if let Some(ref pblc_dssmntn_data_value) = self.pblc_dssmntn_data { if let Err(e) = pblc_dssmntn_data_value.validate() { return Err(e); } }
			if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TradeData59Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeData59Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
		pub data_set_actn: Option<ReportPeriodActivity1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rpt", skip_serializing_if = "Option::is_none") )]
		pub rpt: Option<Vec<TradeReport33Choice>>,
	}
	
	impl TradeData59Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref data_set_actn_value) = self.data_set_actn { if let Err(e) = data_set_actn_value.validate() { return Err(e); } }
			if let Some(ref rpt_vec) = self.rpt { for item in rpt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TradeNonConfirmation1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeNonConfirmation1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: TradeConfirmationType2Code,
	}
	
	impl TradeNonConfirmation1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// TradeReport33Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeReport33Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "New", skip_serializing_if = "Option::is_none") )]
		pub new: Option<TradeData43>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Mod", skip_serializing_if = "Option::is_none") )]
		pub mod_attr: Option<TradeData43>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Crrctn", skip_serializing_if = "Option::is_none") )]
		pub crrctn: Option<TradeData43>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Termntn", skip_serializing_if = "Option::is_none") )]
		pub termntn: Option<TradeData43>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PosCmpnt", skip_serializing_if = "Option::is_none") )]
		pub pos_cmpnt: Option<TradeData43>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ValtnUpd", skip_serializing_if = "Option::is_none") )]
		pub valtn_upd: Option<TradeData43>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cmprssn", skip_serializing_if = "Option::is_none") )]
		pub cmprssn: Option<TradeData43>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Err", skip_serializing_if = "Option::is_none") )]
		pub err: Option<TradeData43>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PortOut", skip_serializing_if = "Option::is_none") )]
		pub port_out: Option<TradeData43>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rvv", skip_serializing_if = "Option::is_none") )]
		pub rvv: Option<TradeData43>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<TradeData43>,
	}
	
	impl TradeReport33Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref new_value) = self.new { if let Err(e) = new_value.validate() { return Err(e); } }
			if let Some(ref mod_attr_value) = self.mod_attr { if let Err(e) = mod_attr_value.validate() { return Err(e); } }
			if let Some(ref crrctn_value) = self.crrctn { if let Err(e) = crrctn_value.validate() { return Err(e); } }
			if let Some(ref termntn_value) = self.termntn { if let Err(e) = termntn_value.validate() { return Err(e); } }
			if let Some(ref pos_cmpnt_value) = self.pos_cmpnt { if let Err(e) = pos_cmpnt_value.validate() { return Err(e); } }
			if let Some(ref valtn_upd_value) = self.valtn_upd { if let Err(e) = valtn_upd_value.validate() { return Err(e); } }
			if let Some(ref cmprssn_value) = self.cmprssn { if let Err(e) = cmprssn_value.validate() { return Err(e); } }
			if let Some(ref err_value) = self.err { if let Err(e) = err_value.validate() { return Err(e); } }
			if let Some(ref port_out_value) = self.port_out { if let Err(e) = port_out_value.validate() { return Err(e); } }
			if let Some(ref rvv_value) = self.rvv { if let Err(e) = rvv_value.validate() { return Err(e); } }
			if let Some(ref othr_value) = self.othr { if let Err(e) = othr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TradeReportHeader4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeReportHeader4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptExctnDt", skip_serializing_if = "Option::is_none") )]
		pub rpt_exctn_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MsgPgntn", skip_serializing_if = "Option::is_none") )]
		pub msg_pgntn: Option<Pagination1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NbRcrds") )]
		pub nb_rcrds: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CmptntAuthrty", skip_serializing_if = "Option::is_none") )]
		pub cmptnt_authrty: Option<Vec<Max100Text>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NewTradRpstryIdr", skip_serializing_if = "Option::is_none") )]
		pub new_trad_rpstry_idr: Option<OrganisationIdentification15Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgPurp", skip_serializing_if = "Option::is_none") )]
		pub rptg_purp: Option<Vec<Max100Text>>,
	}
	
	impl TradeReportHeader4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref msg_pgntn_value) = self.msg_pgntn { if let Err(e) = msg_pgntn_value.validate() { return Err(e); } }
			if let Some(ref cmptnt_authrty_vec) = self.cmptnt_authrty { for item in cmptnt_authrty_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref new_trad_rpstry_idr_value) = self.new_trad_rpstry_idr { if let Err(e) = new_trad_rpstry_idr_value.validate() { return Err(e); } }
			if let Some(ref rptg_purp_vec) = self.rptg_purp { for item in rptg_purp_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TradeTransaction50 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeTransaction50 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxId", skip_serializing_if = "Option::is_none") )]
		pub tx_id: Option<UniqueTransactionIdentifier2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ScndryTxId", skip_serializing_if = "Option::is_none") )]
		pub scndry_tx_id: Option<Max72Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrrTxId", skip_serializing_if = "Option::is_none") )]
		pub prr_tx_id: Option<UniqueTransactionIdentifier3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SbsqntTxId", skip_serializing_if = "Option::is_none") )]
		pub sbsqnt_tx_id: Option<UniqueTransactionIdentifier3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CollPrtflCd", skip_serializing_if = "Option::is_none") )]
		pub coll_prtfl_cd: Option<CollateralPortfolioCode6Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptTrckgNb", skip_serializing_if = "Option::is_none") )]
		pub rpt_trckg_nb: Option<Max52Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PltfmIdr", skip_serializing_if = "Option::is_none") )]
		pub pltfm_idr: Option<MICIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MrrrOrTrggrTx", skip_serializing_if = "Option::is_none") )]
		pub mrrr_or_trggr_tx: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxPric", skip_serializing_if = "Option::is_none") )]
		pub tx_pric: Option<PriceData2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlAmt", skip_serializing_if = "Option::is_none") )]
		pub ntnl_amt: Option<NotionalAmountLegs5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlQty", skip_serializing_if = "Option::is_none") )]
		pub ntnl_qty: Option<NotionalQuantityLegs5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Qty", skip_serializing_if = "Option::is_none") )]
		pub qty: Option<FinancialInstrumentQuantity32Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DlvryTp", skip_serializing_if = "Option::is_none") )]
		pub dlvry_tp: Option<PhysicalTransferType4Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ExctnTmStmp", skip_serializing_if = "Option::is_none") )]
		pub exctn_tm_stmp: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FctvDt", skip_serializing_if = "Option::is_none") )]
		pub fctv_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XprtnDt", skip_serializing_if = "Option::is_none") )]
		pub xprtn_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EarlyTermntnDt", skip_serializing_if = "Option::is_none") )]
		pub early_termntn_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmDt", skip_serializing_if = "Option::is_none") )]
		pub sttlm_dt: Option<Vec<String>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MstrAgrmt", skip_serializing_if = "Option::is_none") )]
		pub mstr_agrmt: Option<MasterAgreement8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cmprssn", skip_serializing_if = "Option::is_none") )]
		pub cmprssn: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstTradRskRdctnFlg", skip_serializing_if = "Option::is_none") )]
		pub pst_trad_rsk_rdctn_flg: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstTradRskRdctnEvt", skip_serializing_if = "Option::is_none") )]
		pub pst_trad_rsk_rdctn_evt: Option<PTRREvent2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DerivEvt", skip_serializing_if = "Option::is_none") )]
		pub deriv_evt: Option<DerivativeEvent6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradConf", skip_serializing_if = "Option::is_none") )]
		pub trad_conf: Option<TradeConfirmation4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NonStdsdTerm", skip_serializing_if = "Option::is_none") )]
		pub non_stdsd_term: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradClr", skip_serializing_if = "Option::is_none") )]
		pub trad_clr: Option<TradeClearing11>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BlckTradElctn", skip_serializing_if = "Option::is_none") )]
		pub blck_trad_elctn: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "LrgNtnlOffFcltyElctn", skip_serializing_if = "Option::is_none") )]
		pub lrg_ntnl_off_fclty_elctn: Option<bool>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none") )]
		pub intrst_rate: Option<InterestRateLegs14>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy", skip_serializing_if = "Option::is_none") )]
		pub ccy: Option<CurrencyExchange22>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none") )]
		pub cmmdty: Option<AssetClassCommodity7Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Optn", skip_serializing_if = "Option::is_none") )]
		pub optn: Option<OptionOrSwaption11>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NrgySpcfcAttrbts", skip_serializing_if = "Option::is_none") )]
		pub nrgy_spcfc_attrbts: Option<EnergySpecificAttribute9>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cdt", skip_serializing_if = "Option::is_none") )]
		pub cdt: Option<CreditDerivative4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPmt", skip_serializing_if = "Option::is_none") )]
		pub othr_pmt: Option<Vec<OtherPayment5>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Packg", skip_serializing_if = "Option::is_none") )]
		pub packg: Option<Package4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradAllcnSts", skip_serializing_if = "Option::is_none") )]
		pub trad_allcn_sts: Option<AllocationIndicator1Code>,
	}
	
	impl TradeTransaction50 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tx_id_value) = self.tx_id { if let Err(e) = tx_id_value.validate() { return Err(e); } }
			if let Some(ref scndry_tx_id_value) = self.scndry_tx_id { if let Err(e) = scndry_tx_id_value.validate() { return Err(e); } }
			if let Some(ref prr_tx_id_value) = self.prr_tx_id { if let Err(e) = prr_tx_id_value.validate() { return Err(e); } }
			if let Some(ref sbsqnt_tx_id_value) = self.sbsqnt_tx_id { if let Err(e) = sbsqnt_tx_id_value.validate() { return Err(e); } }
			if let Some(ref coll_prtfl_cd_value) = self.coll_prtfl_cd { if let Err(e) = coll_prtfl_cd_value.validate() { return Err(e); } }
			if let Some(ref rpt_trckg_nb_value) = self.rpt_trckg_nb { if let Err(e) = rpt_trckg_nb_value.validate() { return Err(e); } }
			if let Some(ref pltfm_idr_value) = self.pltfm_idr { if let Err(e) = pltfm_idr_value.validate() { return Err(e); } }
			if let Some(ref tx_pric_value) = self.tx_pric { if let Err(e) = tx_pric_value.validate() { return Err(e); } }
			if let Some(ref ntnl_amt_value) = self.ntnl_amt { if let Err(e) = ntnl_amt_value.validate() { return Err(e); } }
			if let Some(ref ntnl_qty_value) = self.ntnl_qty { if let Err(e) = ntnl_qty_value.validate() { return Err(e); } }
			if let Some(ref qty_value) = self.qty { if let Err(e) = qty_value.validate() { return Err(e); } }
			if let Some(ref dlvry_tp_value) = self.dlvry_tp { if let Err(e) = dlvry_tp_value.validate() { return Err(e); } }
			if let Some(ref mstr_agrmt_value) = self.mstr_agrmt { if let Err(e) = mstr_agrmt_value.validate() { return Err(e); } }
			if let Some(ref pst_trad_rsk_rdctn_evt_value) = self.pst_trad_rsk_rdctn_evt { if let Err(e) = pst_trad_rsk_rdctn_evt_value.validate() { return Err(e); } }
			if let Some(ref deriv_evt_value) = self.deriv_evt { if let Err(e) = deriv_evt_value.validate() { return Err(e); } }
			if let Some(ref trad_conf_value) = self.trad_conf { if let Err(e) = trad_conf_value.validate() { return Err(e); } }
			if let Some(ref trad_clr_value) = self.trad_clr { if let Err(e) = trad_clr_value.validate() { return Err(e); } }
			if let Some(ref intrst_rate_value) = self.intrst_rate { if let Err(e) = intrst_rate_value.validate() { return Err(e); } }
			if let Some(ref ccy_value) = self.ccy { if let Err(e) = ccy_value.validate() { return Err(e); } }
			if let Some(ref cmmdty_value) = self.cmmdty { if let Err(e) = cmmdty_value.validate() { return Err(e); } }
			if let Some(ref optn_value) = self.optn { if let Err(e) = optn_value.validate() { return Err(e); } }
			if let Some(ref nrgy_spcfc_attrbts_value) = self.nrgy_spcfc_attrbts { if let Err(e) = nrgy_spcfc_attrbts_value.validate() { return Err(e); } }
			if let Some(ref cdt_value) = self.cdt { if let Err(e) = cdt_value.validate() { return Err(e); } }
			if let Some(ref othr_pmt_vec) = self.othr_pmt { for item in othr_pmt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref packg_value) = self.packg { if let Err(e) = packg_value.validate() { return Err(e); } }
			if let Some(ref trad_allcn_sts_value) = self.trad_allcn_sts { if let Err(e) = trad_allcn_sts_value.validate() { return Err(e); } }
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
	
	
	// Tranche3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Tranche3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AttchmntPt", skip_serializing_if = "Option::is_none") )]
		pub attchmnt_pt: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtchmntPt", skip_serializing_if = "Option::is_none") )]
		pub dtchmnt_pt: Option<f64>,
	}
	
	impl Tranche3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// TrancheIndicator3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TrancheIndicator3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Trnchd", skip_serializing_if = "Option::is_none") )]
		pub trnchd: Option<Tranche3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Utrnchd", skip_serializing_if = "Option::is_none") )]
		pub utrnchd: Option<NoReasonCode>,
	}
	
	impl TrancheIndicator3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref trnchd_value) = self.trnchd { if let Err(e) = trnchd_value.validate() { return Err(e); } }
			if let Some(ref utrnchd_value) = self.utrnchd { if let Err(e) = utrnchd_value.validate() { return Err(e); } }
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
	
	
	// UTIIdentifier ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// UniqueTransactionIdentifier1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct UniqueTransactionIdentifier1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnqTxIdr", skip_serializing_if = "Option::is_none") )]
		pub unq_tx_idr: Option<UTIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification179>,
	}
	
	impl UniqueTransactionIdentifier1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref unq_tx_idr_value) = self.unq_tx_idr { if let Err(e) = unq_tx_idr_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// UniqueTransactionIdentifier2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// UniqueTransactionIdentifier3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct UniqueTransactionIdentifier3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnqTxIdr", skip_serializing_if = "Option::is_none") )]
		pub unq_tx_idr: Option<UTIIdentifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<GenericIdentification175>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NotAvlbl", skip_serializing_if = "Option::is_none") )]
		pub not_avlbl: Option<NoReasonCode>,
	}
	
	impl UniqueTransactionIdentifier3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref unq_tx_idr_value) = self.unq_tx_idr { if let Err(e) = unq_tx_idr_value.validate() { return Err(e); } }
			if let Some(ref prtry_value) = self.prtry { if let Err(e) = prtry_value.validate() { return Err(e); } }
			if let Some(ref not_avlbl_value) = self.not_avlbl { if let Err(e) = not_avlbl_value.validate() { return Err(e); } }
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
	
	
	// ValuationType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum ValuationType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "CCPV") )]
		CodeCCPV,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MTMA") )]
		CodeMTMA,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MTMO") )]
		CodeMTMO,
	}
	
	impl ValuationType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// WeekDay3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum WeekDay3Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "ALLD") )]
		CodeALLD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XBHL") )]
		CodeXBHL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IBHL") )]
		CodeIBHL,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FRID") )]
		CodeFRID,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MOND") )]
		CodeMOND,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SATD") )]
		CodeSATD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SUND") )]
		CodeSUND,
		#[cfg_attr( feature = "derive_serde", serde(rename = "THUD") )]
		CodeTHUD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TUED") )]
		CodeTUED,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WEDD") )]
		CodeWEDD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WDAY") )]
		CodeWDAY,
		#[cfg_attr( feature = "derive_serde", serde(rename = "WEND") )]
		CodeWEND,
	}
	
	impl WeekDay3Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// YesNoIndicator ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	#[cfg_attr( feature = "derive_serde", serde(transparent) )]
	pub struct YesNoIndicator {
		#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
		pub yes_no_indicator: bool,
	}
	
	impl YesNoIndicator {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
}