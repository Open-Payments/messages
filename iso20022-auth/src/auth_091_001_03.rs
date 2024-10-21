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
	
	
	// CompareActiveOrHistoricCurrencyAndAmount4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareActiveOrHistoricCurrencyAndAmount4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
	}
	
	impl CompareActiveOrHistoricCurrencyAndAmount4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareActiveOrHistoricCurrencyCode1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareActiveOrHistoricCurrencyCode1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<ActiveOrHistoricCurrencyCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<ActiveOrHistoricCurrencyCode>,
	}
	
	impl CompareActiveOrHistoricCurrencyCode1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareAmountAndDirection3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareAmountAndDirection3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<AmountAndDirection106>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<AmountAndDirection106>,
	}
	
	impl CompareAmountAndDirection3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareAssetClass1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareAssetClass1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<ProductType4Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<ProductType4Code>,
	}
	
	impl CompareAssetClass1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareBenchmarkCode1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareBenchmarkCode1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<ExternalBenchmarkCurveName1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<ExternalBenchmarkCurveName1Code>,
	}
	
	impl CompareBenchmarkCode1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareCFIIdentifier3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// CompareCommodityAssetClass4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareCommodityAssetClass4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<AssetClassCommodity6Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<AssetClassCommodity6Choice>,
	}
	
	impl CompareCommodityAssetClass4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareDate3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// CompareDatePeriod2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareDatePeriod2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<DatePeriod4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<DatePeriod4>,
	}
	
	impl CompareDatePeriod2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareDateTime3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// CompareDayCount1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareDayCount1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<InterestComputationMethodFormat7>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<InterestComputationMethodFormat7>,
	}
	
	impl CompareDayCount1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareDeliveryInterconnectionPoint1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareDeliveryInterconnectionPoint1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<DeliveryInterconnectionPoint1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<DeliveryInterconnectionPoint1Choice>,
	}
	
	impl CompareDeliveryInterconnectionPoint1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareDeliveryType1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareDeliveryType1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<PhysicalTransferType4Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<PhysicalTransferType4Code>,
	}
	
	impl CompareDeliveryType1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareDerivativeEvent1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareDerivativeEvent1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<DerivativeEvent6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<DerivativeEvent6>,
	}
	
	impl CompareDerivativeEvent1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareDurationType1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareDurationType1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<DurationType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<DurationType1Code>,
	}
	
	impl CompareDurationType1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareEnergyDeliveryAttribute1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareEnergyDeliveryAttribute1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NrgyDlvryIntrvl", skip_serializing_if = "Option::is_none") )]
		pub nrgy_dlvry_intrvl: Option<Vec<CompareTimePeriod2>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NrgyDt", skip_serializing_if = "Option::is_none") )]
		pub nrgy_dt: Option<CompareDatePeriod2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NrgyDrtn", skip_serializing_if = "Option::is_none") )]
		pub nrgy_drtn: Option<CompareDurationType1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NrgyWkDay", skip_serializing_if = "Option::is_none") )]
		pub nrgy_wk_day: Option<Vec<CompareWeekDay1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NrgyDlvryCpcty", skip_serializing_if = "Option::is_none") )]
		pub nrgy_dlvry_cpcty: Option<CompareLongFraction19DecimalNumber1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NrgyQtyUnit", skip_serializing_if = "Option::is_none") )]
		pub nrgy_qty_unit: Option<CompareEnergyQuantityUnit1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NrgyPricTmIntrvlQty", skip_serializing_if = "Option::is_none") )]
		pub nrgy_pric_tm_intrvl_qty: Option<CompareAmountAndDirection3>,
	}
	
	impl CompareEnergyDeliveryAttribute1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref nrgy_dlvry_intrvl_vec) = self.nrgy_dlvry_intrvl { for item in nrgy_dlvry_intrvl_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref nrgy_dt_value) = self.nrgy_dt { if let Err(e) = nrgy_dt_value.validate() { return Err(e); } }
			if let Some(ref nrgy_drtn_value) = self.nrgy_drtn { if let Err(e) = nrgy_drtn_value.validate() { return Err(e); } }
			if let Some(ref nrgy_wk_day_vec) = self.nrgy_wk_day { for item in nrgy_wk_day_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref nrgy_dlvry_cpcty_value) = self.nrgy_dlvry_cpcty { if let Err(e) = nrgy_dlvry_cpcty_value.validate() { return Err(e); } }
			if let Some(ref nrgy_qty_unit_value) = self.nrgy_qty_unit { if let Err(e) = nrgy_qty_unit_value.validate() { return Err(e); } }
			if let Some(ref nrgy_pric_tm_intrvl_qty_value) = self.nrgy_pric_tm_intrvl_qty { if let Err(e) = nrgy_pric_tm_intrvl_qty_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareEnergyLoadType1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareEnergyLoadType1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<EnergyLoadType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<EnergyLoadType1Code>,
	}
	
	impl CompareEnergyLoadType1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareEnergyQuantityUnit1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareEnergyQuantityUnit1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<EnergyQuantityUnit2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<EnergyQuantityUnit2Choice>,
	}
	
	impl CompareEnergyQuantityUnit1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareExchangeRate1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareExchangeRate1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<f64>,
	}
	
	impl CompareExchangeRate1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CompareExchangeRateBasis1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareExchangeRateBasis1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<ExchangeRateBasis1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<ExchangeRateBasis1Choice>,
	}
	
	impl CompareExchangeRateBasis1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareFinancialInstrumentContractType1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareFinancialInstrumentContractType1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<FinancialInstrumentContractType2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<FinancialInstrumentContractType2Code>,
	}
	
	impl CompareFinancialInstrumentContractType1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareFrequencyUnit1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareFrequencyUnit1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<Frequency13Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<Frequency13Code>,
	}
	
	impl CompareFrequencyUnit1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareISINIdentifier2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareISINIdentifier2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<ISINOct2015Identifier>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<ISINOct2015Identifier>,
	}
	
	impl CompareISINIdentifier2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareISINIdentifier4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// CompareLegDirection2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareLegDirection2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<Direction4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<Direction4Choice>,
	}
	
	impl CompareLegDirection2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareLongFraction19DecimalNumber1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareLongFraction19DecimalNumber1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<f64>,
	}
	
	impl CompareLongFraction19DecimalNumber1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CompareMICIdentifier3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// CompareMasterAgreementType1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareMasterAgreementType1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<AgreementType2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<AgreementType2Choice>,
	}
	
	impl CompareMasterAgreementType1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareMax350Text1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareMax350Text1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<Max350Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<Max350Text>,
	}
	
	impl CompareMax350Text1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareMax50Text1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareMax50Text1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<Max50Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<Max50Text>,
	}
	
	impl CompareMax50Text1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareNumber5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// CompareNumber7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareNumber7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<f64>,
	}
	
	impl CompareNumber7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// CompareOptionStyle1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareOptionStyle1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<OptionStyle6Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<OptionStyle6Code>,
	}
	
	impl CompareOptionStyle1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareOptionType1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareOptionType1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<OptionType2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<OptionType2Code>,
	}
	
	impl CompareOptionType1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareOrganisationIdentification6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// CompareOtherPayment1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareOtherPayment1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPmtTp", skip_serializing_if = "Option::is_none") )]
		pub othr_pmt_tp: Option<CompareOtherPaymentType1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPmtAmt", skip_serializing_if = "Option::is_none") )]
		pub othr_pmt_amt: Option<CompareAmountAndDirection3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPmtDt", skip_serializing_if = "Option::is_none") )]
		pub othr_pmt_dt: Option<CompareDate3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPmtPyer", skip_serializing_if = "Option::is_none") )]
		pub othr_pmt_pyer: Option<CompareOrganisationIdentification7>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPmtRcvr", skip_serializing_if = "Option::is_none") )]
		pub othr_pmt_rcvr: Option<CompareOrganisationIdentification7>,
	}
	
	impl CompareOtherPayment1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref othr_pmt_tp_value) = self.othr_pmt_tp { if let Err(e) = othr_pmt_tp_value.validate() { return Err(e); } }
			if let Some(ref othr_pmt_amt_value) = self.othr_pmt_amt { if let Err(e) = othr_pmt_amt_value.validate() { return Err(e); } }
			if let Some(ref othr_pmt_dt_value) = self.othr_pmt_dt { if let Err(e) = othr_pmt_dt_value.validate() { return Err(e); } }
			if let Some(ref othr_pmt_pyer_value) = self.othr_pmt_pyer { if let Err(e) = othr_pmt_pyer_value.validate() { return Err(e); } }
			if let Some(ref othr_pmt_rcvr_value) = self.othr_pmt_rcvr { if let Err(e) = othr_pmt_rcvr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareOtherPaymentType1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareOtherPaymentType1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<PaymentType5Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<PaymentType5Choice>,
	}
	
	impl CompareOtherPaymentType1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ComparePercentageRate3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ComparePostTradeRiskReduction2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ComparePostTradeRiskReduction2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<PTRREvent3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<PTRREvent3>,
	}
	
	impl ComparePostTradeRiskReduction2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareReferenceParty1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareReferenceParty1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<DerivativePartyIdentification1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<DerivativePartyIdentification1Choice>,
	}
	
	impl CompareReferenceParty1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareReportingLevelType2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareReportingLevelType2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<ModificationLevel1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<ModificationLevel1Code>,
	}
	
	impl CompareReportingLevelType2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareSeniorityType1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareSeniorityType1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<DebtInstrumentSeniorityType2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<DebtInstrumentSeniorityType2Code>,
	}
	
	impl CompareSeniorityType1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareText1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareText1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<Max52Text>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<Max52Text>,
	}
	
	impl CompareText1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareText2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// CompareTimePeriod2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareTimePeriod2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<TimePeriod3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<TimePeriod3>,
	}
	
	impl CompareTimePeriod2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareTradeClearingObligation1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareTradeClearingObligation1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<ClearingObligationType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<ClearingObligationType1Code>,
	}
	
	impl CompareTradeClearingObligation1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareTradeClearingStatus3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareTradeClearingStatus3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<Cleared23Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<Cleared23Choice>,
	}
	
	impl CompareTradeClearingStatus3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareTradeConfirmation2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareTradeConfirmation2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<TradeConfirmation3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<TradeConfirmation3Choice>,
	}
	
	impl CompareTradeConfirmation2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareTrancheIndicator1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareTrancheIndicator1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<TrancheIndicator3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<TrancheIndicator3Choice>,
	}
	
	impl CompareTrancheIndicator1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareTrueFalseIndicator3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// CompareUnderlyingInstrument3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareUnderlyingInstrument3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<SecurityIdentification41Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<SecurityIdentification41Choice>,
	}
	
	impl CompareUnderlyingInstrument3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareUniqueProductIdentifier2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareUniqueProductIdentifier2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<UniqueProductIdentifier2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<UniqueProductIdentifier2Choice>,
	}
	
	impl CompareUniqueProductIdentifier2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareUniqueTransactionIdentifier2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareUniqueTransactionIdentifier2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<UniqueTransactionIdentifier2Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<UniqueTransactionIdentifier2Choice>,
	}
	
	impl CompareUniqueTransactionIdentifier2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareUnitPrice4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareUnitPrice4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<SecuritiesTransactionPrice17Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<SecuritiesTransactionPrice17Choice>,
	}
	
	impl CompareUnitPrice4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareUnitPrice5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareUnitPrice5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<SecuritiesTransactionPrice17Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<SecuritiesTransactionPrice17Choice>,
	}
	
	impl CompareUnitPrice5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareUnitPrice7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareUnitPrice7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<SecuritiesTransactionPrice14Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<SecuritiesTransactionPrice14Choice>,
	}
	
	impl CompareUnitPrice7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareUnitPrice8 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareUnitPrice8 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<SecuritiesTransactionPrice13Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<SecuritiesTransactionPrice13Choice>,
	}
	
	impl CompareUnitPrice8 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareValuationType1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareValuationType1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<ValuationType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<ValuationType1Code>,
	}
	
	impl CompareValuationType1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareWeekDay1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareWeekDay1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<WeekDay3Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<WeekDay3Code>,
	}
	
	impl CompareWeekDay1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val1_value) = self.val1 { if let Err(e) = val1_value.validate() { return Err(e); } }
			if let Some(ref val2_value) = self.val2 { if let Err(e) = val2_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ContractMatchingCriteria3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ContractMatchingCriteria3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
		pub isin: Option<CompareISINIdentifier2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none") )]
		pub unq_pdct_idr: Option<CompareUniqueProductIdentifier2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AltrntvInstrmId", skip_serializing_if = "Option::is_none") )]
		pub altrntv_instrm_id: Option<CompareText1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PdctClssfctn", skip_serializing_if = "Option::is_none") )]
		pub pdct_clssfctn: Option<CompareCFIIdentifier3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctTp", skip_serializing_if = "Option::is_none") )]
		pub ctrct_tp: Option<CompareFinancialInstrumentContractType1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AsstClss", skip_serializing_if = "Option::is_none") )]
		pub asst_clss: Option<CompareAssetClass1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DerivBasedOnCrptAsst", skip_serializing_if = "Option::is_none") )]
		pub deriv_based_on_crpt_asst: Option<CompareTrueFalseIndicator3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygInstrm", skip_serializing_if = "Option::is_none") )]
		pub undrlyg_instrm: Option<CompareUnderlyingInstrument3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none") )]
		pub sttlm_ccy: Option<CompareActiveOrHistoricCurrencyCode1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmCcyScndLeg", skip_serializing_if = "Option::is_none") )]
		pub sttlm_ccy_scnd_leg: Option<CompareActiveOrHistoricCurrencyCode1>,
	}
	
	impl ContractMatchingCriteria3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref isin_value) = self.isin { if let Err(e) = isin_value.validate() { return Err(e); } }
			if let Some(ref unq_pdct_idr_value) = self.unq_pdct_idr { if let Err(e) = unq_pdct_idr_value.validate() { return Err(e); } }
			if let Some(ref altrntv_instrm_id_value) = self.altrntv_instrm_id { if let Err(e) = altrntv_instrm_id_value.validate() { return Err(e); } }
			if let Some(ref pdct_clssfctn_value) = self.pdct_clssfctn { if let Err(e) = pdct_clssfctn_value.validate() { return Err(e); } }
			if let Some(ref ctrct_tp_value) = self.ctrct_tp { if let Err(e) = ctrct_tp_value.validate() { return Err(e); } }
			if let Some(ref asst_clss_value) = self.asst_clss { if let Err(e) = asst_clss_value.validate() { return Err(e); } }
			if let Some(ref deriv_based_on_crpt_asst_value) = self.deriv_based_on_crpt_asst { if let Err(e) = deriv_based_on_crpt_asst_value.validate() { return Err(e); } }
			if let Some(ref undrlyg_instrm_value) = self.undrlyg_instrm { if let Err(e) = undrlyg_instrm_value.validate() { return Err(e); } }
			if let Some(ref sttlm_ccy_value) = self.sttlm_ccy { if let Err(e) = sttlm_ccy_value.validate() { return Err(e); } }
			if let Some(ref sttlm_ccy_scnd_leg_value) = self.sttlm_ccy_scnd_leg { if let Err(e) = sttlm_ccy_scnd_leg_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CounterpartyData91 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CounterpartyData91 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none") )]
		pub rptg_ctr_pty: Option<OrganisationIdentification15Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none") )]
		pub othr_ctr_pty: Option<PartyIdentification236Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptSubmitgNtty", skip_serializing_if = "Option::is_none") )]
		pub rpt_submitg_ntty: Option<OrganisationIdentification15Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none") )]
		pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
	}
	
	impl CounterpartyData91 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref rptg_ctr_pty_value) = self.rptg_ctr_pty { if let Err(e) = rptg_ctr_pty_value.validate() { return Err(e); } }
			if let Some(ref othr_ctr_pty_value) = self.othr_ctr_pty { if let Err(e) = othr_ctr_pty_value.validate() { return Err(e); } }
			if let Some(ref rpt_submitg_ntty_value) = self.rpt_submitg_ntty { if let Err(e) = rpt_submitg_ntty_value.validate() { return Err(e); } }
			if let Some(ref ntty_rspnsbl_for_rpt_value) = self.ntty_rspnsbl_for_rpt { if let Err(e) = ntty_rspnsbl_for_rpt_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CounterpartyMatchingCriteria6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CounterpartyMatchingCriteria6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none") )]
		pub rptg_ctr_pty: Option<CompareOrganisationIdentification6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none") )]
		pub othr_ctr_pty: Option<CompareOrganisationIdentification7>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DrctnOrSd", skip_serializing_if = "Option::is_none") )]
		pub drctn_or_sd: Option<CompareLegDirection2>,
	}
	
	impl CounterpartyMatchingCriteria6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref rptg_ctr_pty_value) = self.rptg_ctr_pty { if let Err(e) = rptg_ctr_pty_value.validate() { return Err(e); } }
			if let Some(ref othr_ctr_pty_value) = self.othr_ctr_pty { if let Err(e) = othr_ctr_pty_value.validate() { return Err(e); } }
			if let Some(ref drctn_or_sd_value) = self.drctn_or_sd { if let Err(e) = drctn_or_sd_value.validate() { return Err(e); } }
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
	
	
	// DatePeriod4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DatePeriod4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt", skip_serializing_if = "Option::is_none") )]
		pub fr_dt: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt", skip_serializing_if = "Option::is_none") )]
		pub to_dt: Option<String>,
	}
	
	impl DatePeriod4 {
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
	
	
	// DerivativesTradeReconciliationStatisticalReportV03 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct DerivativesTradeReconciliationStatisticalReportV03 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RcncltnSttstcs") )]
		pub rcncltn_sttstcs: StatisticsPerCounterparty19Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl DerivativesTradeReconciliationStatisticalReportV03 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rcncltn_sttstcs.validate() { return Err(e); }
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
	
	
	// MatchingCriteria17 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MatchingCriteria17 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtyMtchgCrit", skip_serializing_if = "Option::is_none") )]
		pub ctr_pty_mtchg_crit: Option<CounterpartyMatchingCriteria6>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ValtnMtchgCrit", skip_serializing_if = "Option::is_none") )]
		pub valtn_mtchg_crit: Option<ValuationMatchingCriteria1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctMtchgCrit", skip_serializing_if = "Option::is_none") )]
		pub ctrct_mtchg_crit: Option<ContractMatchingCriteria3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxMtchgCrit", skip_serializing_if = "Option::is_none") )]
		pub tx_mtchg_crit: Option<TransactionMatchingCriteria7>,
	}
	
	impl MatchingCriteria17 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ctr_pty_mtchg_crit_value) = self.ctr_pty_mtchg_crit { if let Err(e) = ctr_pty_mtchg_crit_value.validate() { return Err(e); } }
			if let Some(ref valtn_mtchg_crit_value) = self.valtn_mtchg_crit { if let Err(e) = valtn_mtchg_crit_value.validate() { return Err(e); } }
			if let Some(ref ctrct_mtchg_crit_value) = self.ctrct_mtchg_crit { if let Err(e) = ctrct_mtchg_crit_value.validate() { return Err(e); } }
			if let Some(ref tx_mtchg_crit_value) = self.tx_mtchg_crit { if let Err(e) = tx_mtchg_crit_value.validate() { return Err(e); } }
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
	
	
	// PTRREvent3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PTRREvent3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tchnq", skip_serializing_if = "Option::is_none") )]
		pub tchnq: Option<RiskReductionService1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SvcPrvdr", skip_serializing_if = "Option::is_none") )]
		pub svc_prvdr: Option<OrganisationIdentification15Choice>,
	}
	
	impl PTRREvent3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tchnq_value) = self.tchnq { if let Err(e) = tchnq_value.validate() { return Err(e); } }
			if let Some(ref svc_prvdr_value) = self.svc_prvdr { if let Err(e) = svc_prvdr_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PairingStatus1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum PairingStatus1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "PARD") )]
		CodePARD,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UNPR") )]
		CodeUNPR,
	}
	
	impl PairingStatus1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
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
	
	
	// ReconciliationCategory4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ReconciliationCategory4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rvvd") )]
		pub rvvd: bool,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrthrMod") )]
		pub frthr_mod: bool,
	}
	
	impl ReconciliationCategory4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ReconciliationCategory5 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ReconciliationCategory5 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgTp") )]
		pub rptg_tp: TradeRepositoryReportingType1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pairg") )]
		pub pairg: PairingStatus1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rcncltn") )]
		pub rcncltn: ReconciliationStatus1Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ValtnRcncltn") )]
		pub valtn_rcncltn: ReconciliationStatus2Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rvvd") )]
		pub rvvd: bool,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrthrMod") )]
		pub frthr_mod: bool,
	}
	
	impl ReconciliationCategory5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rptg_tp.validate() { return Err(e); }
			if let Err(e) = self.pairg.validate() { return Err(e); }
			if let Err(e) = self.rcncltn.validate() { return Err(e); }
			if let Err(e) = self.valtn_rcncltn.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// ReconciliationCounterpartyPairStatistics7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ReconciliationCounterpartyPairStatistics7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtyId") )]
		pub ctr_pty_id: CounterpartyData91,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfTxs") )]
		pub ttl_nb_of_txs: f64,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RcncltnRpt") )]
		pub rcncltn_rpt: Vec<ReconciliationReport15>,
	}
	
	impl ReconciliationCounterpartyPairStatistics7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.ctr_pty_id.validate() { return Err(e); }
			for item in &self.rcncltn_rpt { if let Err(e) = item.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ReconciliationReport15 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ReconciliationReport15 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxId") )]
		pub tx_id: TradeTransactionIdentification24,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MtchgCrit") )]
		pub mtchg_crit: MatchingCriteria17,
	}
	
	impl ReconciliationReport15 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tx_id.validate() { return Err(e); }
			if let Err(e) = self.mtchg_crit.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// ReconciliationStatisticsPerCounterparty4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ReconciliationStatisticsPerCounterparty4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RefDt") )]
		pub ref_dt: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RcncltnCtgrs") )]
		pub rcncltn_ctgrs: ReportingRequirement3Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNbOfTxs", skip_serializing_if = "Option::is_none") )]
		pub ttl_nb_of_txs: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxDtls", skip_serializing_if = "Option::is_none") )]
		pub tx_dtls: Option<Vec<ReconciliationCounterpartyPairStatistics7>>,
	}
	
	impl ReconciliationStatisticsPerCounterparty4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rcncltn_ctgrs.validate() { return Err(e); }
			if let Some(ref tx_dtls_vec) = self.tx_dtls { for item in tx_dtls_vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// ReconciliationStatus1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum ReconciliationStatus1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "NREC") )]
		CodeNREC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RECO") )]
		CodeRECO,
	}
	
	impl ReconciliationStatus1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// ReconciliationStatus2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum ReconciliationStatus2Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "NREC") )]
		CodeNREC,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RECO") )]
		CodeRECO,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NOAP") )]
		CodeNOAP,
	}
	
	impl ReconciliationStatus2Code {
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
	
	
	// ReportingRequirement3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ReportingRequirement3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgRqrmnt", skip_serializing_if = "Option::is_none") )]
		pub rptg_rqrmnt: Option<ReconciliationCategory5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoRptgRqrmnt", skip_serializing_if = "Option::is_none") )]
		pub no_rptg_rqrmnt: Option<ReconciliationCategory4>,
	}
	
	impl ReportingRequirement3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref rptg_rqrmnt_value) = self.rptg_rqrmnt { if let Err(e) = rptg_rqrmnt_value.validate() { return Err(e); } }
			if let Some(ref no_rptg_rqrmnt_value) = self.no_rptg_rqrmnt { if let Err(e) = no_rptg_rqrmnt_value.validate() { return Err(e); } }
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
	
	
	// SecuritiesTransactionPrice13Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SecuritiesTransactionPrice13Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "MntryVal", skip_serializing_if = "Option::is_none") )]
		pub mntry_val: Option<AmountAndDirection106>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Pctg", skip_serializing_if = "Option::is_none") )]
		pub pctg: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dcml", skip_serializing_if = "Option::is_none") )]
		pub dcml: Option<f64>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "BsisPtSprd", skip_serializing_if = "Option::is_none") )]
		pub bsis_pt_sprd: Option<f64>,
	}
	
	impl SecuritiesTransactionPrice13Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref mntry_val_value) = self.mntry_val { if let Err(e) = mntry_val_value.validate() { return Err(e); } }
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
	
	
	// StatisticsPerCounterparty19Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct StatisticsPerCounterparty19Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
		pub data_set_actn: Option<ReportPeriodActivity1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rpt", skip_serializing_if = "Option::is_none") )]
		pub rpt: Option<Vec<ReconciliationStatisticsPerCounterparty4>>,
	}
	
	impl StatisticsPerCounterparty19Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref data_set_actn_value) = self.data_set_actn { if let Err(e) = data_set_actn_value.validate() { return Err(e); } }
			if let Some(ref rpt_vec) = self.rpt { for item in rpt_vec { if let Err(e) = item.validate() { return Err(e); } } }
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
	
	
	// TimePeriod3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TimePeriod3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FrTm", skip_serializing_if = "Option::is_none") )]
		pub fr_tm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ToTm", skip_serializing_if = "Option::is_none") )]
		pub to_tm: Option<String>,
	}
	
	impl TimePeriod3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// TradeConfirmation3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeConfirmation3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Confd", skip_serializing_if = "Option::is_none") )]
		pub confd: Option<TradeConfirmation4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NonConfd", skip_serializing_if = "Option::is_none") )]
		pub non_confd: Option<TradeNonConfirmation1>,
	}
	
	impl TradeConfirmation3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref confd_value) = self.confd { if let Err(e) = confd_value.validate() { return Err(e); } }
			if let Some(ref non_confd_value) = self.non_confd { if let Err(e) = non_confd_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TradeConfirmation4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeConfirmation4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<TradeConfirmationType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TmStmp", skip_serializing_if = "Option::is_none") )]
		pub tm_stmp: Option<String>,
	}
	
	impl TradeConfirmation4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
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
	
	
	// TradeRepositoryReportingType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub enum TradeRepositoryReportingType1Code {
		#[cfg_attr(feature = "derive_default", default)]
		#[cfg_attr( feature = "derive_serde", serde(rename = "SWOS") )]
		CodeSWOS,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TWOS") )]
		CodeTWOS,
	}
	
	impl TradeRepositoryReportingType1Code {
		pub fn validate(&self) -> Result<(), ValidationError> {
			Ok(())
		}
	}
	
	
	// TradeTransactionIdentification24 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// TransactionMatchingCriteria7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TransactionMatchingCriteria7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptTrckgNb", skip_serializing_if = "Option::is_none") )]
		pub rpt_trckg_nb: Option<CompareText2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnqTxIdr", skip_serializing_if = "Option::is_none") )]
		pub unq_tx_idr: Option<CompareUniqueTransactionIdentifier2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PrrUnqTxIdr", skip_serializing_if = "Option::is_none") )]
		pub prr_unq_tx_idr: Option<CompareUniqueTransactionIdentifier2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SbsqntPosUnqTxIdr", skip_serializing_if = "Option::is_none") )]
		pub sbsqnt_pos_unq_tx_idr: Option<CompareUniqueTransactionIdentifier2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dlta", skip_serializing_if = "Option::is_none") )]
		pub dlta: Option<CompareLongFraction19DecimalNumber1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradConf", skip_serializing_if = "Option::is_none") )]
		pub trad_conf: Option<CompareTradeConfirmation2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradClrOblgtn", skip_serializing_if = "Option::is_none") )]
		pub trad_clr_oblgtn: Option<CompareTradeClearingObligation1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TradClrSts", skip_serializing_if = "Option::is_none") )]
		pub trad_clr_sts: Option<CompareTradeClearingStatus3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MstrAgrmtTp", skip_serializing_if = "Option::is_none") )]
		pub mstr_agrmt_tp: Option<CompareMasterAgreementType1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "MstrAgrmtVrsn", skip_serializing_if = "Option::is_none") )]
		pub mstr_agrmt_vrsn: Option<CompareMax50Text1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntraGrp", skip_serializing_if = "Option::is_none") )]
		pub intra_grp: Option<CompareTrueFalseIndicator3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PstTradRskRdctn", skip_serializing_if = "Option::is_none") )]
		pub pst_trad_rsk_rdctn: Option<ComparePostTradeRiskReduction2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DerivEvt", skip_serializing_if = "Option::is_none") )]
		pub deriv_evt: Option<CompareDerivativeEvent1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PltfmIdr", skip_serializing_if = "Option::is_none") )]
		pub pltfm_idr: Option<CompareMICIdentifier3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "ExctnTmStmp", skip_serializing_if = "Option::is_none") )]
		pub exctn_tm_stmp: Option<CompareDateTime3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FctvDt", skip_serializing_if = "Option::is_none") )]
		pub fctv_dt: Option<CompareDate3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "XprtnDt", skip_serializing_if = "Option::is_none") )]
		pub xprtn_dt: Option<CompareDate3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "EarlyTermntnDt", skip_serializing_if = "Option::is_none") )]
		pub early_termntn_dt: Option<CompareDate3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmDt", skip_serializing_if = "Option::is_none") )]
		pub sttlm_dt: Option<Vec<CompareDate3>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DlvryTp", skip_serializing_if = "Option::is_none") )]
		pub dlvry_tp: Option<CompareDeliveryType1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxPric", skip_serializing_if = "Option::is_none") )]
		pub tx_pric: Option<CompareUnitPrice5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PricSchdlUadjstdFctvDt", skip_serializing_if = "Option::is_none") )]
		pub pric_schdl_uadjstd_fctv_dt: Option<Vec<CompareDate3>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PricSchdlUadjstdEndDt", skip_serializing_if = "Option::is_none") )]
		pub pric_schdl_uadjstd_end_dt: Option<Vec<CompareDate3>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxSchdlPric", skip_serializing_if = "Option::is_none") )]
		pub tx_schdl_pric: Option<Vec<CompareUnitPrice5>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PackgPric", skip_serializing_if = "Option::is_none") )]
		pub packg_pric: Option<CompareUnitPrice5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlAmtFrstLeg", skip_serializing_if = "Option::is_none") )]
		pub ntnl_amt_frst_leg: Option<CompareAmountAndDirection3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlAmtFrstLegUadjstdFctvDt", skip_serializing_if = "Option::is_none") )]
		pub ntnl_amt_frst_leg_uadjstd_fctv_dt: Option<Vec<CompareDate3>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlAmtFrstLegUadjstdEndDt", skip_serializing_if = "Option::is_none") )]
		pub ntnl_amt_frst_leg_uadjstd_end_dt: Option<Vec<CompareDate3>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlAmtFrstLegSchdlAmt", skip_serializing_if = "Option::is_none") )]
		pub ntnl_amt_frst_leg_schdl_amt: Option<Vec<CompareAmountAndDirection3>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlQtyFrstLeg", skip_serializing_if = "Option::is_none") )]
		pub ntnl_qty_frst_leg: Option<CompareLongFraction19DecimalNumber1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlQtyFrstLegUadjstdFctvDt", skip_serializing_if = "Option::is_none") )]
		pub ntnl_qty_frst_leg_uadjstd_fctv_dt: Option<Vec<CompareDate3>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlQtyFrstLegUadjstdEndDt", skip_serializing_if = "Option::is_none") )]
		pub ntnl_qty_frst_leg_uadjstd_end_dt: Option<Vec<CompareDate3>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlQtyFrstLegSchdlQty", skip_serializing_if = "Option::is_none") )]
		pub ntnl_qty_frst_leg_schdl_qty: Option<Vec<CompareLongFraction19DecimalNumber1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlAmtScndLeg", skip_serializing_if = "Option::is_none") )]
		pub ntnl_amt_scnd_leg: Option<CompareAmountAndDirection3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlAmtScndLegUadjstdFctvDt", skip_serializing_if = "Option::is_none") )]
		pub ntnl_amt_scnd_leg_uadjstd_fctv_dt: Option<Vec<CompareDate3>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlAmtScndLegUadjstdEndDt", skip_serializing_if = "Option::is_none") )]
		pub ntnl_amt_scnd_leg_uadjstd_end_dt: Option<Vec<CompareDate3>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlAmtScndLegSchdlAmt", skip_serializing_if = "Option::is_none") )]
		pub ntnl_amt_scnd_leg_schdl_amt: Option<Vec<CompareAmountAndDirection3>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlQtyScndLeg", skip_serializing_if = "Option::is_none") )]
		pub ntnl_qty_scnd_leg: Option<CompareLongFraction19DecimalNumber1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlQtyScndLegUadjstdFctvDt", skip_serializing_if = "Option::is_none") )]
		pub ntnl_qty_scnd_leg_uadjstd_fctv_dt: Option<Vec<CompareDate3>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlQtyScndLegUadjstdEndDt", skip_serializing_if = "Option::is_none") )]
		pub ntnl_qty_scnd_leg_uadjstd_end_dt: Option<Vec<CompareDate3>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlQtyScndLegSchdlQty", skip_serializing_if = "Option::is_none") )]
		pub ntnl_qty_scnd_leg_schdl_qty: Option<Vec<CompareLongFraction19DecimalNumber1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPmt", skip_serializing_if = "Option::is_none") )]
		pub othr_pmt: Option<Vec<CompareOtherPayment1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFxdRateFrstLeg", skip_serializing_if = "Option::is_none") )]
		pub intrst_fxd_rate_frst_leg: Option<CompareUnitPrice7>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFxdRateFrstLegDayCnt", skip_serializing_if = "Option::is_none") )]
		pub intrst_fxd_rate_frst_leg_day_cnt: Option<CompareDayCount1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFxdRateFrstLegPmtFrqcyUnit", skip_serializing_if = "Option::is_none") )]
		pub intrst_fxd_rate_frst_leg_pmt_frqcy_unit: Option<CompareFrequencyUnit1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFxdRateFrstLegPmtFrqcyVal", skip_serializing_if = "Option::is_none") )]
		pub intrst_fxd_rate_frst_leg_pmt_frqcy_val: Option<CompareNumber5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFltgRateFrstLegId", skip_serializing_if = "Option::is_none") )]
		pub intrst_fltg_rate_frst_leg_id: Option<CompareISINIdentifier4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFltgRateFrstLegCd", skip_serializing_if = "Option::is_none") )]
		pub intrst_fltg_rate_frst_leg_cd: Option<CompareBenchmarkCode1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFltgRateFrstLegNm", skip_serializing_if = "Option::is_none") )]
		pub intrst_fltg_rate_frst_leg_nm: Option<CompareMax350Text1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFltgRateFrstLegDayCnt", skip_serializing_if = "Option::is_none") )]
		pub intrst_fltg_rate_frst_leg_day_cnt: Option<CompareDayCount1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFltgRateFrstLegPmtFrqcyUnit", skip_serializing_if = "Option::is_none") )]
		pub intrst_fltg_rate_frst_leg_pmt_frqcy_unit: Option<CompareFrequencyUnit1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFltgRateFrstLegPmtFrqcyVal", skip_serializing_if = "Option::is_none") )]
		pub intrst_fltg_rate_frst_leg_pmt_frqcy_val: Option<CompareNumber5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFltgRateFrstLegRefPrdUnit", skip_serializing_if = "Option::is_none") )]
		pub intrst_fltg_rate_frst_leg_ref_prd_unit: Option<CompareFrequencyUnit1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFltgRateFrstLegRefPrdVal", skip_serializing_if = "Option::is_none") )]
		pub intrst_fltg_rate_frst_leg_ref_prd_val: Option<CompareNumber5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFltgRateFrstLegRstFrqcyUnit", skip_serializing_if = "Option::is_none") )]
		pub intrst_fltg_rate_frst_leg_rst_frqcy_unit: Option<CompareFrequencyUnit1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFltgRateFrstLegRstFrqcyVal", skip_serializing_if = "Option::is_none") )]
		pub intrst_fltg_rate_frst_leg_rst_frqcy_val: Option<CompareNumber5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFltgRateFrstLegSprd", skip_serializing_if = "Option::is_none") )]
		pub intrst_fltg_rate_frst_leg_sprd: Option<CompareUnitPrice8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstRateFxdScndLeg", skip_serializing_if = "Option::is_none") )]
		pub intrst_rate_fxd_scnd_leg: Option<CompareUnitPrice7>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFxdRateScndLegDayCnt", skip_serializing_if = "Option::is_none") )]
		pub intrst_fxd_rate_scnd_leg_day_cnt: Option<CompareDayCount1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFxdRateScndLegPmtFrqcyUnit", skip_serializing_if = "Option::is_none") )]
		pub intrst_fxd_rate_scnd_leg_pmt_frqcy_unit: Option<CompareFrequencyUnit1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFxdRateScndLegPmtFrqcyVal", skip_serializing_if = "Option::is_none") )]
		pub intrst_fxd_rate_scnd_leg_pmt_frqcy_val: Option<CompareNumber5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFltgRateScndLegId", skip_serializing_if = "Option::is_none") )]
		pub intrst_fltg_rate_scnd_leg_id: Option<CompareISINIdentifier4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFltgRateScndLegCd", skip_serializing_if = "Option::is_none") )]
		pub intrst_fltg_rate_scnd_leg_cd: Option<CompareBenchmarkCode1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFltgRateScndLegNm", skip_serializing_if = "Option::is_none") )]
		pub intrst_fltg_rate_scnd_leg_nm: Option<CompareMax350Text1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFltgRateScndLegDayCnt", skip_serializing_if = "Option::is_none") )]
		pub intrst_fltg_rate_scnd_leg_day_cnt: Option<CompareDayCount1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFltgRateScndLegPmtFrqcyUnit", skip_serializing_if = "Option::is_none") )]
		pub intrst_fltg_rate_scnd_leg_pmt_frqcy_unit: Option<CompareFrequencyUnit1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFltgRateScndLegPmtFrqcyVal", skip_serializing_if = "Option::is_none") )]
		pub intrst_fltg_rate_scnd_leg_pmt_frqcy_val: Option<CompareNumber5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFltgRateScndLegRefPrdUnit", skip_serializing_if = "Option::is_none") )]
		pub intrst_fltg_rate_scnd_leg_ref_prd_unit: Option<CompareFrequencyUnit1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFltgRateScndLegRefPrdVal", skip_serializing_if = "Option::is_none") )]
		pub intrst_fltg_rate_scnd_leg_ref_prd_val: Option<CompareNumber5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFltgRateScndLegRstFrqcyUnit", skip_serializing_if = "Option::is_none") )]
		pub intrst_fltg_rate_scnd_leg_rst_frqcy_unit: Option<CompareFrequencyUnit1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFltgRateScndLegRstFrqcyVal", skip_serializing_if = "Option::is_none") )]
		pub intrst_fltg_rate_scnd_leg_rst_frqcy_val: Option<CompareNumber5>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstFltgRateScndLegSprd", skip_serializing_if = "Option::is_none") )]
		pub intrst_fltg_rate_scnd_leg_sprd: Option<CompareUnitPrice8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "PackgSprd", skip_serializing_if = "Option::is_none") )]
		pub packg_sprd: Option<CompareUnitPrice8>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CcyXchgRate", skip_serializing_if = "Option::is_none") )]
		pub ccy_xchg_rate: Option<CompareExchangeRate1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CcyFwdXchgRate", skip_serializing_if = "Option::is_none") )]
		pub ccy_fwd_xchg_rate: Option<CompareExchangeRate1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CcyXchgRateBsis", skip_serializing_if = "Option::is_none") )]
		pub ccy_xchg_rate_bsis: Option<CompareExchangeRateBasis1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none") )]
		pub cmmdty: Option<CompareCommodityAssetClass4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NrgyDlvryPtOrZone", skip_serializing_if = "Option::is_none") )]
		pub nrgy_dlvry_pt_or_zone: Option<Vec<CompareDeliveryInterconnectionPoint1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NrgyIntrCnnctnPt", skip_serializing_if = "Option::is_none") )]
		pub nrgy_intr_cnnctn_pt: Option<CompareDeliveryInterconnectionPoint1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NrgyLdTp", skip_serializing_if = "Option::is_none") )]
		pub nrgy_ld_tp: Option<CompareEnergyLoadType1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DlvryAttr", skip_serializing_if = "Option::is_none") )]
		pub dlvry_attr: Option<Vec<CompareEnergyDeliveryAttribute1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OptnTp", skip_serializing_if = "Option::is_none") )]
		pub optn_tp: Option<CompareOptionType1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OptnExrcStyle", skip_serializing_if = "Option::is_none") )]
		pub optn_exrc_style: Option<Vec<CompareOptionStyle1>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OptnStrkPric", skip_serializing_if = "Option::is_none") )]
		pub optn_strk_pric: Option<CompareUnitPrice4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OptnStrkPricSchdlUadjstdFctvDt", skip_serializing_if = "Option::is_none") )]
		pub optn_strk_pric_schdl_uadjstd_fctv_dt: Option<Vec<CompareDate3>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OptnStrkPricSchdlUadjstdEndDt", skip_serializing_if = "Option::is_none") )]
		pub optn_strk_pric_schdl_uadjstd_end_dt: Option<Vec<CompareDate3>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OptnStrkPricSchdlAmt", skip_serializing_if = "Option::is_none") )]
		pub optn_strk_pric_schdl_amt: Option<Vec<CompareUnitPrice4>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OptnPrmAmt", skip_serializing_if = "Option::is_none") )]
		pub optn_prm_amt: Option<CompareActiveOrHistoricCurrencyAndAmount4>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OptnPrmPmtDt", skip_serializing_if = "Option::is_none") )]
		pub optn_prm_pmt_dt: Option<CompareDate3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OptnMtrtyDtOfUndrlyg", skip_serializing_if = "Option::is_none") )]
		pub optn_mtrty_dt_of_undrlyg: Option<CompareDate3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtSnrty", skip_serializing_if = "Option::is_none") )]
		pub cdt_snrty: Option<CompareSeniorityType1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtRefPty", skip_serializing_if = "Option::is_none") )]
		pub cdt_ref_pty: Option<CompareReferenceParty1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtSrs", skip_serializing_if = "Option::is_none") )]
		pub cdt_srs: Option<CompareNumber7>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtVrsn", skip_serializing_if = "Option::is_none") )]
		pub cdt_vrsn: Option<CompareNumber7>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtIndxFctr", skip_serializing_if = "Option::is_none") )]
		pub cdt_indx_fctr: Option<ComparePercentageRate3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "CdtTrch", skip_serializing_if = "Option::is_none") )]
		pub cdt_trch: Option<CompareTrancheIndicator1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Lvl", skip_serializing_if = "Option::is_none") )]
		pub lvl: Option<CompareReportingLevelType2>,
	}
	
	impl TransactionMatchingCriteria7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref rpt_trckg_nb_value) = self.rpt_trckg_nb { if let Err(e) = rpt_trckg_nb_value.validate() { return Err(e); } }
			if let Some(ref unq_tx_idr_value) = self.unq_tx_idr { if let Err(e) = unq_tx_idr_value.validate() { return Err(e); } }
			if let Some(ref prr_unq_tx_idr_value) = self.prr_unq_tx_idr { if let Err(e) = prr_unq_tx_idr_value.validate() { return Err(e); } }
			if let Some(ref sbsqnt_pos_unq_tx_idr_value) = self.sbsqnt_pos_unq_tx_idr { if let Err(e) = sbsqnt_pos_unq_tx_idr_value.validate() { return Err(e); } }
			if let Some(ref dlta_value) = self.dlta { if let Err(e) = dlta_value.validate() { return Err(e); } }
			if let Some(ref trad_conf_value) = self.trad_conf { if let Err(e) = trad_conf_value.validate() { return Err(e); } }
			if let Some(ref trad_clr_oblgtn_value) = self.trad_clr_oblgtn { if let Err(e) = trad_clr_oblgtn_value.validate() { return Err(e); } }
			if let Some(ref trad_clr_sts_value) = self.trad_clr_sts { if let Err(e) = trad_clr_sts_value.validate() { return Err(e); } }
			if let Some(ref mstr_agrmt_tp_value) = self.mstr_agrmt_tp { if let Err(e) = mstr_agrmt_tp_value.validate() { return Err(e); } }
			if let Some(ref mstr_agrmt_vrsn_value) = self.mstr_agrmt_vrsn { if let Err(e) = mstr_agrmt_vrsn_value.validate() { return Err(e); } }
			if let Some(ref intra_grp_value) = self.intra_grp { if let Err(e) = intra_grp_value.validate() { return Err(e); } }
			if let Some(ref pst_trad_rsk_rdctn_value) = self.pst_trad_rsk_rdctn { if let Err(e) = pst_trad_rsk_rdctn_value.validate() { return Err(e); } }
			if let Some(ref deriv_evt_value) = self.deriv_evt { if let Err(e) = deriv_evt_value.validate() { return Err(e); } }
			if let Some(ref pltfm_idr_value) = self.pltfm_idr { if let Err(e) = pltfm_idr_value.validate() { return Err(e); } }
			if let Some(ref exctn_tm_stmp_value) = self.exctn_tm_stmp { if let Err(e) = exctn_tm_stmp_value.validate() { return Err(e); } }
			if let Some(ref fctv_dt_value) = self.fctv_dt { if let Err(e) = fctv_dt_value.validate() { return Err(e); } }
			if let Some(ref xprtn_dt_value) = self.xprtn_dt { if let Err(e) = xprtn_dt_value.validate() { return Err(e); } }
			if let Some(ref early_termntn_dt_value) = self.early_termntn_dt { if let Err(e) = early_termntn_dt_value.validate() { return Err(e); } }
			if let Some(ref sttlm_dt_vec) = self.sttlm_dt { for item in sttlm_dt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref dlvry_tp_value) = self.dlvry_tp { if let Err(e) = dlvry_tp_value.validate() { return Err(e); } }
			if let Some(ref tx_pric_value) = self.tx_pric { if let Err(e) = tx_pric_value.validate() { return Err(e); } }
			if let Some(ref pric_schdl_uadjstd_fctv_dt_vec) = self.pric_schdl_uadjstd_fctv_dt { for item in pric_schdl_uadjstd_fctv_dt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref pric_schdl_uadjstd_end_dt_vec) = self.pric_schdl_uadjstd_end_dt { for item in pric_schdl_uadjstd_end_dt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref tx_schdl_pric_vec) = self.tx_schdl_pric { for item in tx_schdl_pric_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref packg_pric_value) = self.packg_pric { if let Err(e) = packg_pric_value.validate() { return Err(e); } }
			if let Some(ref ntnl_amt_frst_leg_value) = self.ntnl_amt_frst_leg { if let Err(e) = ntnl_amt_frst_leg_value.validate() { return Err(e); } }
			if let Some(ref ntnl_amt_frst_leg_uadjstd_fctv_dt_vec) = self.ntnl_amt_frst_leg_uadjstd_fctv_dt { for item in ntnl_amt_frst_leg_uadjstd_fctv_dt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref ntnl_amt_frst_leg_uadjstd_end_dt_vec) = self.ntnl_amt_frst_leg_uadjstd_end_dt { for item in ntnl_amt_frst_leg_uadjstd_end_dt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref ntnl_amt_frst_leg_schdl_amt_vec) = self.ntnl_amt_frst_leg_schdl_amt { for item in ntnl_amt_frst_leg_schdl_amt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref ntnl_qty_frst_leg_value) = self.ntnl_qty_frst_leg { if let Err(e) = ntnl_qty_frst_leg_value.validate() { return Err(e); } }
			if let Some(ref ntnl_qty_frst_leg_uadjstd_fctv_dt_vec) = self.ntnl_qty_frst_leg_uadjstd_fctv_dt { for item in ntnl_qty_frst_leg_uadjstd_fctv_dt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref ntnl_qty_frst_leg_uadjstd_end_dt_vec) = self.ntnl_qty_frst_leg_uadjstd_end_dt { for item in ntnl_qty_frst_leg_uadjstd_end_dt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref ntnl_qty_frst_leg_schdl_qty_vec) = self.ntnl_qty_frst_leg_schdl_qty { for item in ntnl_qty_frst_leg_schdl_qty_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref ntnl_amt_scnd_leg_value) = self.ntnl_amt_scnd_leg { if let Err(e) = ntnl_amt_scnd_leg_value.validate() { return Err(e); } }
			if let Some(ref ntnl_amt_scnd_leg_uadjstd_fctv_dt_vec) = self.ntnl_amt_scnd_leg_uadjstd_fctv_dt { for item in ntnl_amt_scnd_leg_uadjstd_fctv_dt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref ntnl_amt_scnd_leg_uadjstd_end_dt_vec) = self.ntnl_amt_scnd_leg_uadjstd_end_dt { for item in ntnl_amt_scnd_leg_uadjstd_end_dt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref ntnl_amt_scnd_leg_schdl_amt_vec) = self.ntnl_amt_scnd_leg_schdl_amt { for item in ntnl_amt_scnd_leg_schdl_amt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref ntnl_qty_scnd_leg_value) = self.ntnl_qty_scnd_leg { if let Err(e) = ntnl_qty_scnd_leg_value.validate() { return Err(e); } }
			if let Some(ref ntnl_qty_scnd_leg_uadjstd_fctv_dt_vec) = self.ntnl_qty_scnd_leg_uadjstd_fctv_dt { for item in ntnl_qty_scnd_leg_uadjstd_fctv_dt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref ntnl_qty_scnd_leg_uadjstd_end_dt_vec) = self.ntnl_qty_scnd_leg_uadjstd_end_dt { for item in ntnl_qty_scnd_leg_uadjstd_end_dt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref ntnl_qty_scnd_leg_schdl_qty_vec) = self.ntnl_qty_scnd_leg_schdl_qty { for item in ntnl_qty_scnd_leg_schdl_qty_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref othr_pmt_vec) = self.othr_pmt { for item in othr_pmt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref intrst_fxd_rate_frst_leg_value) = self.intrst_fxd_rate_frst_leg { if let Err(e) = intrst_fxd_rate_frst_leg_value.validate() { return Err(e); } }
			if let Some(ref intrst_fxd_rate_frst_leg_day_cnt_value) = self.intrst_fxd_rate_frst_leg_day_cnt { if let Err(e) = intrst_fxd_rate_frst_leg_day_cnt_value.validate() { return Err(e); } }
			if let Some(ref intrst_fxd_rate_frst_leg_pmt_frqcy_unit_value) = self.intrst_fxd_rate_frst_leg_pmt_frqcy_unit { if let Err(e) = intrst_fxd_rate_frst_leg_pmt_frqcy_unit_value.validate() { return Err(e); } }
			if let Some(ref intrst_fxd_rate_frst_leg_pmt_frqcy_val_value) = self.intrst_fxd_rate_frst_leg_pmt_frqcy_val { if let Err(e) = intrst_fxd_rate_frst_leg_pmt_frqcy_val_value.validate() { return Err(e); } }
			if let Some(ref intrst_fltg_rate_frst_leg_id_value) = self.intrst_fltg_rate_frst_leg_id { if let Err(e) = intrst_fltg_rate_frst_leg_id_value.validate() { return Err(e); } }
			if let Some(ref intrst_fltg_rate_frst_leg_cd_value) = self.intrst_fltg_rate_frst_leg_cd { if let Err(e) = intrst_fltg_rate_frst_leg_cd_value.validate() { return Err(e); } }
			if let Some(ref intrst_fltg_rate_frst_leg_nm_value) = self.intrst_fltg_rate_frst_leg_nm { if let Err(e) = intrst_fltg_rate_frst_leg_nm_value.validate() { return Err(e); } }
			if let Some(ref intrst_fltg_rate_frst_leg_day_cnt_value) = self.intrst_fltg_rate_frst_leg_day_cnt { if let Err(e) = intrst_fltg_rate_frst_leg_day_cnt_value.validate() { return Err(e); } }
			if let Some(ref intrst_fltg_rate_frst_leg_pmt_frqcy_unit_value) = self.intrst_fltg_rate_frst_leg_pmt_frqcy_unit { if let Err(e) = intrst_fltg_rate_frst_leg_pmt_frqcy_unit_value.validate() { return Err(e); } }
			if let Some(ref intrst_fltg_rate_frst_leg_pmt_frqcy_val_value) = self.intrst_fltg_rate_frst_leg_pmt_frqcy_val { if let Err(e) = intrst_fltg_rate_frst_leg_pmt_frqcy_val_value.validate() { return Err(e); } }
			if let Some(ref intrst_fltg_rate_frst_leg_ref_prd_unit_value) = self.intrst_fltg_rate_frst_leg_ref_prd_unit { if let Err(e) = intrst_fltg_rate_frst_leg_ref_prd_unit_value.validate() { return Err(e); } }
			if let Some(ref intrst_fltg_rate_frst_leg_ref_prd_val_value) = self.intrst_fltg_rate_frst_leg_ref_prd_val { if let Err(e) = intrst_fltg_rate_frst_leg_ref_prd_val_value.validate() { return Err(e); } }
			if let Some(ref intrst_fltg_rate_frst_leg_rst_frqcy_unit_value) = self.intrst_fltg_rate_frst_leg_rst_frqcy_unit { if let Err(e) = intrst_fltg_rate_frst_leg_rst_frqcy_unit_value.validate() { return Err(e); } }
			if let Some(ref intrst_fltg_rate_frst_leg_rst_frqcy_val_value) = self.intrst_fltg_rate_frst_leg_rst_frqcy_val { if let Err(e) = intrst_fltg_rate_frst_leg_rst_frqcy_val_value.validate() { return Err(e); } }
			if let Some(ref intrst_fltg_rate_frst_leg_sprd_value) = self.intrst_fltg_rate_frst_leg_sprd { if let Err(e) = intrst_fltg_rate_frst_leg_sprd_value.validate() { return Err(e); } }
			if let Some(ref intrst_rate_fxd_scnd_leg_value) = self.intrst_rate_fxd_scnd_leg { if let Err(e) = intrst_rate_fxd_scnd_leg_value.validate() { return Err(e); } }
			if let Some(ref intrst_fxd_rate_scnd_leg_day_cnt_value) = self.intrst_fxd_rate_scnd_leg_day_cnt { if let Err(e) = intrst_fxd_rate_scnd_leg_day_cnt_value.validate() { return Err(e); } }
			if let Some(ref intrst_fxd_rate_scnd_leg_pmt_frqcy_unit_value) = self.intrst_fxd_rate_scnd_leg_pmt_frqcy_unit { if let Err(e) = intrst_fxd_rate_scnd_leg_pmt_frqcy_unit_value.validate() { return Err(e); } }
			if let Some(ref intrst_fxd_rate_scnd_leg_pmt_frqcy_val_value) = self.intrst_fxd_rate_scnd_leg_pmt_frqcy_val { if let Err(e) = intrst_fxd_rate_scnd_leg_pmt_frqcy_val_value.validate() { return Err(e); } }
			if let Some(ref intrst_fltg_rate_scnd_leg_id_value) = self.intrst_fltg_rate_scnd_leg_id { if let Err(e) = intrst_fltg_rate_scnd_leg_id_value.validate() { return Err(e); } }
			if let Some(ref intrst_fltg_rate_scnd_leg_cd_value) = self.intrst_fltg_rate_scnd_leg_cd { if let Err(e) = intrst_fltg_rate_scnd_leg_cd_value.validate() { return Err(e); } }
			if let Some(ref intrst_fltg_rate_scnd_leg_nm_value) = self.intrst_fltg_rate_scnd_leg_nm { if let Err(e) = intrst_fltg_rate_scnd_leg_nm_value.validate() { return Err(e); } }
			if let Some(ref intrst_fltg_rate_scnd_leg_day_cnt_value) = self.intrst_fltg_rate_scnd_leg_day_cnt { if let Err(e) = intrst_fltg_rate_scnd_leg_day_cnt_value.validate() { return Err(e); } }
			if let Some(ref intrst_fltg_rate_scnd_leg_pmt_frqcy_unit_value) = self.intrst_fltg_rate_scnd_leg_pmt_frqcy_unit { if let Err(e) = intrst_fltg_rate_scnd_leg_pmt_frqcy_unit_value.validate() { return Err(e); } }
			if let Some(ref intrst_fltg_rate_scnd_leg_pmt_frqcy_val_value) = self.intrst_fltg_rate_scnd_leg_pmt_frqcy_val { if let Err(e) = intrst_fltg_rate_scnd_leg_pmt_frqcy_val_value.validate() { return Err(e); } }
			if let Some(ref intrst_fltg_rate_scnd_leg_ref_prd_unit_value) = self.intrst_fltg_rate_scnd_leg_ref_prd_unit { if let Err(e) = intrst_fltg_rate_scnd_leg_ref_prd_unit_value.validate() { return Err(e); } }
			if let Some(ref intrst_fltg_rate_scnd_leg_ref_prd_val_value) = self.intrst_fltg_rate_scnd_leg_ref_prd_val { if let Err(e) = intrst_fltg_rate_scnd_leg_ref_prd_val_value.validate() { return Err(e); } }
			if let Some(ref intrst_fltg_rate_scnd_leg_rst_frqcy_unit_value) = self.intrst_fltg_rate_scnd_leg_rst_frqcy_unit { if let Err(e) = intrst_fltg_rate_scnd_leg_rst_frqcy_unit_value.validate() { return Err(e); } }
			if let Some(ref intrst_fltg_rate_scnd_leg_rst_frqcy_val_value) = self.intrst_fltg_rate_scnd_leg_rst_frqcy_val { if let Err(e) = intrst_fltg_rate_scnd_leg_rst_frqcy_val_value.validate() { return Err(e); } }
			if let Some(ref intrst_fltg_rate_scnd_leg_sprd_value) = self.intrst_fltg_rate_scnd_leg_sprd { if let Err(e) = intrst_fltg_rate_scnd_leg_sprd_value.validate() { return Err(e); } }
			if let Some(ref packg_sprd_value) = self.packg_sprd { if let Err(e) = packg_sprd_value.validate() { return Err(e); } }
			if let Some(ref ccy_xchg_rate_value) = self.ccy_xchg_rate { if let Err(e) = ccy_xchg_rate_value.validate() { return Err(e); } }
			if let Some(ref ccy_fwd_xchg_rate_value) = self.ccy_fwd_xchg_rate { if let Err(e) = ccy_fwd_xchg_rate_value.validate() { return Err(e); } }
			if let Some(ref ccy_xchg_rate_bsis_value) = self.ccy_xchg_rate_bsis { if let Err(e) = ccy_xchg_rate_bsis_value.validate() { return Err(e); } }
			if let Some(ref cmmdty_value) = self.cmmdty { if let Err(e) = cmmdty_value.validate() { return Err(e); } }
			if let Some(ref nrgy_dlvry_pt_or_zone_vec) = self.nrgy_dlvry_pt_or_zone { for item in nrgy_dlvry_pt_or_zone_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref nrgy_intr_cnnctn_pt_value) = self.nrgy_intr_cnnctn_pt { if let Err(e) = nrgy_intr_cnnctn_pt_value.validate() { return Err(e); } }
			if let Some(ref nrgy_ld_tp_value) = self.nrgy_ld_tp { if let Err(e) = nrgy_ld_tp_value.validate() { return Err(e); } }
			if let Some(ref dlvry_attr_vec) = self.dlvry_attr { for item in dlvry_attr_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref optn_tp_value) = self.optn_tp { if let Err(e) = optn_tp_value.validate() { return Err(e); } }
			if let Some(ref optn_exrc_style_vec) = self.optn_exrc_style { for item in optn_exrc_style_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref optn_strk_pric_value) = self.optn_strk_pric { if let Err(e) = optn_strk_pric_value.validate() { return Err(e); } }
			if let Some(ref optn_strk_pric_schdl_uadjstd_fctv_dt_vec) = self.optn_strk_pric_schdl_uadjstd_fctv_dt { for item in optn_strk_pric_schdl_uadjstd_fctv_dt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref optn_strk_pric_schdl_uadjstd_end_dt_vec) = self.optn_strk_pric_schdl_uadjstd_end_dt { for item in optn_strk_pric_schdl_uadjstd_end_dt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref optn_strk_pric_schdl_amt_vec) = self.optn_strk_pric_schdl_amt { for item in optn_strk_pric_schdl_amt_vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref optn_prm_amt_value) = self.optn_prm_amt { if let Err(e) = optn_prm_amt_value.validate() { return Err(e); } }
			if let Some(ref optn_prm_pmt_dt_value) = self.optn_prm_pmt_dt { if let Err(e) = optn_prm_pmt_dt_value.validate() { return Err(e); } }
			if let Some(ref optn_mtrty_dt_of_undrlyg_value) = self.optn_mtrty_dt_of_undrlyg { if let Err(e) = optn_mtrty_dt_of_undrlyg_value.validate() { return Err(e); } }
			if let Some(ref cdt_snrty_value) = self.cdt_snrty { if let Err(e) = cdt_snrty_value.validate() { return Err(e); } }
			if let Some(ref cdt_ref_pty_value) = self.cdt_ref_pty { if let Err(e) = cdt_ref_pty_value.validate() { return Err(e); } }
			if let Some(ref cdt_srs_value) = self.cdt_srs { if let Err(e) = cdt_srs_value.validate() { return Err(e); } }
			if let Some(ref cdt_vrsn_value) = self.cdt_vrsn { if let Err(e) = cdt_vrsn_value.validate() { return Err(e); } }
			if let Some(ref cdt_indx_fctr_value) = self.cdt_indx_fctr { if let Err(e) = cdt_indx_fctr_value.validate() { return Err(e); } }
			if let Some(ref cdt_trch_value) = self.cdt_trch { if let Err(e) = cdt_trch_value.validate() { return Err(e); } }
			if let Some(ref lvl_value) = self.lvl { if let Err(e) = lvl_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// TransactionOperationType10Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// ValuationMatchingCriteria1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ValuationMatchingCriteria1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctVal", skip_serializing_if = "Option::is_none") )]
		pub ctrct_val: Option<CompareAmountAndDirection3>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<CompareValuationType1>,
	}
	
	impl ValuationMatchingCriteria1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ctrct_val_value) = self.ctrct_val { if let Err(e) = ctrct_val_value.validate() { return Err(e); } }
			if let Some(ref tp_value) = self.tp { if let Err(e) = tp_value.validate() { return Err(e); } }
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