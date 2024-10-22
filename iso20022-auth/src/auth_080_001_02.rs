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
	
	
	// ActiveOrHistoricCurrencyAndAmount ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// AgreementType1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AgreementType1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
		pub tp: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl AgreementType1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tp {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 4 {
					return Err(ValidationError::new(1002, "tp exceeds the maximum length of 4".to_string()));
				}
			}
			if let Some(ref val) = self.prtry {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
				}
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
		pub tp: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl AgreementType2Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tp {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 4 {
					return Err(ValidationError::new(1002, "tp exceeds the maximum length of 4".to_string()));
				}
			}
			if let Some(ref val) = self.prtry {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 50 {
					return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 50".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// AgriculturalCommodityDairy1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// AssetClassCommodity5Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if let Some(ref val) = self.agrcltrl { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.nrgy { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.envttl { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.frtlzr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.frght { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.indstrl_pdct { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.metl { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.othr_c10 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ppr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.plprpln { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.infltn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.multi_cmmdty_extc { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.offcl_ecnmc_sttstcs { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.othr { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityAgricultural5Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if let Some(ref val) = self.grn_oil_seed { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.soft { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ptt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.olv_oil { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.dairy { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.frstry { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.sfd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.live_stock { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.grn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.othr { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityEnergy2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if let Some(ref val) = self.elctrcty { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ntrl_gas { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.oil { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.coal { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.intr_nrgy { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.rnwbl_nrgy { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.lght_end { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.dstllts { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.othr { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityEnvironmental2Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if let Some(ref val) = self.emssns { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.wthr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.crbn_rltd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.othr { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityFertilizer3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if let Some(ref val) = self.ammn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.dmmnm_phspht { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ptsh { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.slphr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.urea { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.urea_and_ammnm_ntrt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.othr { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityFreight3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if let Some(ref val) = self.dry { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.wet { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.cntnr_ship { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.othr { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityIndustrialProduct1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AssetClassCommodityIndustrialProduct1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cnstrctn", skip_serializing_if = "Option::is_none") )]
		pub cnstrctn: Option<IndustrialProductCommodityConstruction1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Manfctg", skip_serializing_if = "Option::is_none") )]
		pub manfctg: Option<IndustrialProductCommodityManufacturing1>,
	}
	
	impl AssetClassCommodityIndustrialProduct1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cnstrctn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.manfctg { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// AssetClassCommodityMetal1Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AssetClassCommodityMetal1Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NonPrcs", skip_serializing_if = "Option::is_none") )]
		pub non_prcs: Option<MetalCommodityNonPrecious1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prcs", skip_serializing_if = "Option::is_none") )]
		pub prcs: Option<MetalCommodityPrecious1>,
	}
	
	impl AssetClassCommodityMetal1Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.non_prcs { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prcs { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// AssetClassCommodityOtherC102Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AssetClassCommodityOtherC102Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dlvrbl", skip_serializing_if = "Option::is_none") )]
		pub dlvrbl: Option<OtherC10CommodityDeliverable2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NonDlvrbl", skip_serializing_if = "Option::is_none") )]
		pub non_dlvrbl: Option<OtherC10CommodityNonDeliverable2>,
	}
	
	impl AssetClassCommodityOtherC102Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.dlvrbl { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.non_dlvrbl { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityPaper3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if let Some(ref val) = self.cntnr_brd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.nwsprnt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.pulp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.rcvrd_ppr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.othr { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// AssetClassCommodityPolypropylene3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AssetClassCommodityPolypropylene3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Plstc", skip_serializing_if = "Option::is_none") )]
		pub plstc: Option<PolypropyleneCommodityPlastic1>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<PolypropyleneCommodityOther1>,
	}
	
	impl AssetClassCommodityPolypropylene3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.plstc { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.othr { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// AssetClassSubProductType38Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// AssetClassSubProductType47Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// BenchmarkCurveName10Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct BenchmarkCurveName10Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Indx", skip_serializing_if = "Option::is_none") )]
		pub indx: Option<BenchmarkCurveName3Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
		pub nm: Option<String>,
	}
	
	impl BenchmarkCurveName10Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.indx { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 350 {
					return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// BenchmarkCurveName3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// CashCompare3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CashCompare3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val", skip_serializing_if = "Option::is_none") )]
		pub val: Option<CompareAmountAndDirection2>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "HrcutOrMrgn", skip_serializing_if = "Option::is_none") )]
		pub hrcut_or_mrgn: Option<ComparePercentageRate3>,
	}
	
	impl CashCompare3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.val { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.hrcut_or_mrgn { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// Cleared4Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct Cleared4Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Clrd", skip_serializing_if = "Option::is_none") )]
		pub clrd: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NonClrd", skip_serializing_if = "Option::is_none") )]
		pub non_clrd: Option<NoReasonCode>,
	}
	
	impl Cleared4Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.clrd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.non_clrd { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CollateralDeliveryMethod1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if let Some(ref val) = self.uncollsd_flg { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.net_xpsr_collstn_ind { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.coll_val_dt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.asst_tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.bskt_idr { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CollateralQualityType1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if let Some(ref val) = self.clssfctn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.qty { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.unit_pric { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.mkt_val { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.unit_of_measr { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareActiveOrHistoricCurrencyAndAmount3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareActiveOrHistoricCurrencyAndAmount3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<ActiveOrHistoricCurrencyAndAmount>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<ActiveOrHistoricCurrencyAndAmount>,
	}
	
	impl CompareActiveOrHistoricCurrencyAndAmount3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.val1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.val2 { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareAgreementType2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareAgreementType2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<AgreementType1Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<AgreementType1Choice>,
	}
	
	impl CompareAgreementType2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.val1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.val2 { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareAmountAndDirection1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareAmountAndDirection1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<AmountAndDirection53>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<AmountAndDirection53>,
	}
	
	impl CompareAmountAndDirection1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.val1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.val2 { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareAmountAndDirection2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareAmountAndDirection2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<AmountAndDirection53>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<AmountAndDirection53>,
	}
	
	impl CompareAmountAndDirection2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.val1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.val2 { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareBenchmarkCurveName3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareBenchmarkCurveName3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<BenchmarkCurveName10Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<BenchmarkCurveName10Choice>,
	}
	
	impl CompareBenchmarkCurveName3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.val1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.val2 { if let Err(e) = val.validate() { return Err(e); } }
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
		pub val1: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<String>,
	}
	
	impl CompareCFIIdentifier3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.val1 {
				let pattern = Regex::new("[A-Z]{6,6}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "val1 does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.val2 {
				let pattern = Regex::new("[A-Z]{6,6}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "val2 does not match the required pattern".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// CompareClearingStatus3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareClearingStatus3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<Cleared4Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<Cleared4Choice>,
	}
	
	impl CompareClearingStatus3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.val1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.val2 { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareCollateralQualityType3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareCollateralQualityType3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<CollateralQualityType1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<CollateralQualityType1Code>,
	}
	
	impl CompareCollateralQualityType3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.val1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.val2 { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareCommodityAssetClass3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareCommodityAssetClass3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<AssetClassCommodity5Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<AssetClassCommodity5Choice>,
	}
	
	impl CompareCommodityAssetClass3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.val1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.val2 { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareCounterpartySide2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareCounterpartySide2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<CollateralRole1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<CollateralRole1Code>,
	}
	
	impl CompareCounterpartySide2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.val1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.val2 { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareCountryCode3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareCountryCode3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<String>,
	}
	
	impl CompareCountryCode3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.val1 {
				let pattern = Regex::new("[A-Z]{2,2}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "val1 does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.val2 {
				let pattern = Regex::new("[A-Z]{2,2}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "val2 does not match the required pattern".to_string()));
				}
			}
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
	
	
	// CompareDecimalNumber3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareDeliveryMethod3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<CollateralDeliveryMethod1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<CollateralDeliveryMethod1Code>,
	}
	
	impl CompareDeliveryMethod3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.val1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.val2 { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareExposureType3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareExposureType3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<ExposureType10Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<ExposureType10Code>,
	}
	
	impl CompareExposureType3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.val1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.val2 { if let Err(e) = val.validate() { return Err(e); } }
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
		pub val1: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<String>,
	}
	
	impl CompareISINIdentifier4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.val1 {
				let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "val1 does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.val2 {
				let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "val2 does not match the required pattern".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// CompareInterestComputationMethod3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareInterestComputationMethod3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<InterestComputationMethodFormat6Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<InterestComputationMethodFormat6Choice>,
	}
	
	impl CompareInterestComputationMethod3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.val1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.val2 { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareInterestRate1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if let Some(ref val) = self.mrgn_ln_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fxd_intrst_rate { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.day_cnt_bsis { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fltg_intrst_ref_rate { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fltg_intrst_rate_term_unit { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fltg_intrst_rate_term_val { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fltg_intrst_rate_pmt_frqcy_unit { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fltg_intrst_rate_pmt_frqcy_val { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fltg_intrst_rate_rst_frqcy_unit { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fltg_intrst_rate_rst_frqcy_val { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.bsis_pt_sprd { if let Err(e) = val.validate() { return Err(e); } }
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
		pub val1: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<String>,
	}
	
	impl CompareMICIdentifier3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.val1 {
				let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "val1 does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.val2 {
				let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "val2 does not match the required pattern".to_string()));
				}
			}
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
	
	
	// CompareNumber6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if let Some(ref val) = self.val1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.val2 { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.val1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.val2 { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// CompareRateBasis3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareRateBasis3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<RateBasis1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<RateBasis1Code>,
	}
	
	impl CompareRateBasis3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.val1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.val2 { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareReportingLevelType3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareReportingLevelType3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<ModificationLevel1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<ModificationLevel1Code>,
	}
	
	impl CompareReportingLevelType3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.val1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.val2 { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareSecuritiesLendingType3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareSecuritiesLendingType3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<SecuritiesLendingType3Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<SecuritiesLendingType3Choice>,
	}
	
	impl CompareSecuritiesLendingType3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.val1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.val2 { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareSecurityIdentification4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareSecurityIdentification4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<SecurityIdentification26Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<SecurityIdentification26Choice>,
	}
	
	impl CompareSecurityIdentification4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.val1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.val2 { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareSpecialCollateral3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareSpecialCollateral3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<SpecialCollateral1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<SpecialCollateral1Code>,
	}
	
	impl CompareSpecialCollateral3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.val1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.val2 { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareTerminationOption3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareTerminationOption3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<RepoTerminationOption2Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<RepoTerminationOption2Code>,
	}
	
	impl CompareTerminationOption3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.val1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.val2 { if let Err(e) = val.validate() { return Err(e); } }
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
		pub val1: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<String>,
	}
	
	impl CompareText2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.val1 {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "val1 is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 52 {
					return Err(ValidationError::new(1002, "val1 exceeds the maximum length of 52".to_string()));
				}
			}
			if let Some(ref val) = self.val2 {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "val2 is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 52 {
					return Err(ValidationError::new(1002, "val2 exceeds the maximum length of 52".to_string()));
				}
			}
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
	
	
	// CompareUnitOfMeasure3 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareUnitOfMeasure3 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<UnitOfMeasure11Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<UnitOfMeasure11Code>,
	}
	
	impl CompareUnitOfMeasure3 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.val1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.val2 { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CompareUnitPrice6 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct CompareUnitPrice6 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val1", skip_serializing_if = "Option::is_none") )]
		pub val1: Option<SecuritiesTransactionPrice19Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
		pub val2: Option<SecuritiesTransactionPrice19Choice>,
	}
	
	impl CompareUnitPrice6 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.val1 { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.val2 { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// CounterpartyMatchingCriteria4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if let Some(ref val) = self.rptg_ctr_pty { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.othr_ctr_pty { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ctr_pty_sd { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// EnergyCommodityCoal1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// FertilizerCommodityAmmonia1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct GenericIdentification175 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
		pub id: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
		pub schme_nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
		pub issr: Option<String>,
	}
	
	impl GenericIdentification175 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if self.id.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if self.id.chars().count() > 72 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 72".to_string()));
			}
			if let Some(ref val) = self.schme_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "schme_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "schme_nm exceeds the maximum length of 35".to_string()));
				}
			}
			if let Some(ref val) = self.issr {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// IndustrialProductCommodityConstruction1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct IndustrialProductCommodityConstruction1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType6Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType33Code>,
	}
	
	impl IndustrialProductCommodityConstruction1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref val) = self.sub_pdct { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// IndustrialProductCommodityManufacturing1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct IndustrialProductCommodityManufacturing1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType6Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType34Code>,
	}
	
	impl IndustrialProductCommodityManufacturing1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref val) = self.sub_pdct { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// InterestComputationMethod1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct InterestComputationMethodFormat6Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<InterestComputationMethod1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl InterestComputationMethodFormat6Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prtry {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// LoanMatchingCriteria9 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if let Some(ref val) = self.unq_trad_idr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.termntn_dt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ctrct_tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.clr_sts { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.clr_dt_tm { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ccp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.tradg_vn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.mstr_agrmt_tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.exctn_dt_tm { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.val_dt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.mtrty_dt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.min_ntce_prd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.earlst_call_bck_dt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.gnl_coll { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.dlvry_by_val { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.coll_dlvry_mtd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.opn_term { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.termntn_optn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fxd_intrst_rate { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.day_cnt_bsis { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fltg_intrst_ref_rate { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fltg_intrst_rate_term_unit { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fltg_intrst_rate_term_val { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fltg_intrst_rate_pmt_frqcy_unit { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fltg_intrst_rate_pmt_frqcy_val { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fltg_intrst_rate_rst_frqcy_unit { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fltg_intrst_rate_rst_frqcy_val { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.bsis_pt_sprd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.mrgn_ln_attr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.prncpl_amt_val_dt_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.prncpl_amt_mtrty_dt_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.asst_tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ln_val { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fxd_rbt_ref_rate { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fltg_rbt_ref_rate { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fltg_rbt_rate_term_unit { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fltg_rbt_rate_term_val { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fltg_rbt_rate_pmt_frqcy_unit { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fltg_rbt_rate_pmt_frqcy_val { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fltg_rbt_rate_rst_frqcy_unit { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.fltg_rbt_rate_rst_frqcy_val { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.rbt_rate_bsis_pt_sprd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.fltg_rate_adjstmnt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.fltg_rate_adjstmnt_dt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.lndg_fee { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.outsdng_mrgn_ln_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.shrt_mkt_val_amt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.lvl_tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.unit_of_measr { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// MasterAgreement7 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct MasterAgreement7 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
		pub tp: AgreementType2Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Vrsn", skip_serializing_if = "Option::is_none") )]
		pub vrsn: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrMstrAgrmtDtls", skip_serializing_if = "Option::is_none") )]
		pub othr_mstr_agrmt_dtls: Option<String>,
	}
	
	impl MasterAgreement7 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.tp.validate() { return Err(e); }
			if let Some(ref val) = self.vrsn {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "vrsn is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 50 {
					return Err(ValidationError::new(1002, "vrsn exceeds the maximum length of 50".to_string()));
				}
			}
			if let Some(ref val) = self.othr_mstr_agrmt_dtls {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "othr_mstr_agrmt_dtls is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 350 {
					return Err(ValidationError::new(1002, "othr_mstr_agrmt_dtls exceeds the maximum length of 350".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// MatchingCriteria10 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if let Some(ref val) = self.ctr_pty_mtchg_crit { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ln_mtchg_crit { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.coll_mtchg_crit { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// MetalCommodityNonPrecious1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
		pub nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dmcl", skip_serializing_if = "Option::is_none") )]
		pub dmcl: Option<String>,
	}
	
	impl NaturalPersonIdentification2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref val) = self.nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 105 {
					return Err(ValidationError::new(1002, "nm exceeds the maximum length of 105".to_string()));
				}
			}
			if let Some(ref val) = self.dmcl {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "dmcl is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 500 {
					return Err(ValidationError::new(1002, "dmcl exceeds the maximum length of 500".to_string()));
				}
			}
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
	
	
	// NotAvailable1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct NumberOfReportsPerStatus4 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtldNbOfRpts") )]
		pub dtld_nb_of_rpts: String,
		#[cfg_attr( feature = "derive_serde", serde(rename = "DtldSts") )]
		pub dtld_sts: PairedReconciled3Code,
	}
	
	impl NumberOfReportsPerStatus4 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(&self.dtld_nb_of_rpts) {
				return Err(ValidationError::new(1005, "dtld_nb_of_rpts does not match the required pattern".to_string()));
			}
			if let Err(e) = self.dtld_sts.validate() { return Err(e); }
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
		pub lei: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
		pub othr: Option<OrganisationIdentification38>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
		pub any_bic: Option<String>,
	}
	
	impl OrganisationIdentification15Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.lei {
				let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.othr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.any_bic {
				let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
				}
			}
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
		pub nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Dmcl", skip_serializing_if = "Option::is_none") )]
		pub dmcl: Option<String>,
	}
	
	impl OrganisationIdentification38 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.id.validate() { return Err(e); }
			if let Some(ref val) = self.nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 105 {
					return Err(ValidationError::new(1002, "nm exceeds the maximum length of 105".to_string()));
				}
			}
			if let Some(ref val) = self.dmcl {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "dmcl is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 500 {
					return Err(ValidationError::new(1002, "dmcl exceeds the maximum length of 500".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// OtherC10CommodityDeliverable2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OtherC10CommodityDeliverable2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType11Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType47Code>,
	}
	
	impl OtherC10CommodityDeliverable2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref val) = self.sub_pdct { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// OtherC10CommodityNonDeliverable2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct OtherC10CommodityNonDeliverable2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType11Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType48Code>,
	}
	
	impl OtherC10CommodityNonDeliverable2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref val) = self.sub_pdct { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PairedReconciled3Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaperCommodityContainerBoard1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType8Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType35Code>,
	}
	
	impl PaperCommodityContainerBoard1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref val) = self.sub_pdct { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaperCommodityNewsprint1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaperCommodityNewsprint1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType8Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType36Code>,
	}
	
	impl PaperCommodityNewsprint1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref val) = self.sub_pdct { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaperCommodityPulp1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaperCommodityPulp1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType8Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType37Code>,
	}
	
	impl PaperCommodityPulp1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref val) = self.sub_pdct { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaperCommodityRecoveredPaper1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaperCommodityRecoveredPaper1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType8Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType38Code>,
	}
	
	impl PaperCommodityRecoveredPaper1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref val) = self.sub_pdct { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PaperCommodityRecoveredPaper2 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PaperCommodityRecoveredPaper2 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType8Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType49Code>,
	}
	
	impl PaperCommodityRecoveredPaper2 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref val) = self.sub_pdct { if let Err(e) = val.validate() { return Err(e); } }
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
			if let Some(ref val) = self.lgl { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.ntrl { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// PolypropyleneCommodityOther1 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct PolypropyleneCommodityPlastic1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "BasePdct") )]
		pub base_pdct: AssetClassProductType9Code,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SubPdct", skip_serializing_if = "Option::is_none") )]
		pub sub_pdct: Option<AssetClassSubProductType18Code>,
	}
	
	impl PolypropyleneCommodityPlastic1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.base_pdct.validate() { return Err(e); }
			if let Some(ref val) = self.sub_pdct { if let Err(e) = val.validate() { return Err(e); } }
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
	
	
	// ReconciliationMatchedStatus9Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ReconciliationMatchedStatus9Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Mtchd", skip_serializing_if = "Option::is_none") )]
		pub mtchd: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NotMtchd", skip_serializing_if = "Option::is_none") )]
		pub not_mtchd: Option<ReconciliationResult10>,
	}
	
	impl ReconciliationMatchedStatus9Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.mtchd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.not_mtchd { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// ReconciliationReport8 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ReconciliationReport8 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none") )]
		pub tech_rcrd_id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "TxId") )]
		pub tx_id: TradeTransactionIdentification19,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Modfd") )]
		pub modfd: bool,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RcncltnSts") )]
		pub rcncltn_sts: ReconciliationStatus8Choice,
	}
	
	impl ReconciliationReport8 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tech_rcrd_id {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "tech_rcrd_id is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 140 {
					return Err(ValidationError::new(1002, "tech_rcrd_id exceeds the maximum length of 140".to_string()));
				}
			}
			if let Err(e) = self.tx_id.validate() { return Err(e); }
			if let Err(e) = self.rcncltn_sts.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// ReconciliationResult10 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct ReconciliationStatus8Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "NoRcncltnReqrd", skip_serializing_if = "Option::is_none") )]
		pub no_rcncltn_reqrd: Option<NoReasonCode>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgData", skip_serializing_if = "Option::is_none") )]
		pub rptg_data: Option<ReconciliationMatchedStatus9Choice>,
	}
	
	impl ReconciliationStatus8Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.no_rcncltn_reqrd { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.rptg_data { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// RepoTerminationOption2Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	
	
	// SecuritiesFinancingReportingReconciliationStatusAdviceV02 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SecuritiesFinancingReportingReconciliationStatusAdviceV02 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RcncltnData") )]
		pub rcncltn_data: TradeData34Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
		pub splmtry_data: Option<Vec<SupplementaryData1>>,
	}
	
	impl SecuritiesFinancingReportingReconciliationStatusAdviceV02 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.rcncltn_data.validate() { return Err(e); }
			if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// SecuritiesLendingType3Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SecuritiesLendingType3Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
		pub cd: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
		pub prtry: Option<String>,
	}
	
	impl SecuritiesLendingType3Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.cd {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 4 {
					return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
				}
			}
			if let Some(ref val) = self.prtry {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// SecuritiesTransactionPrice19Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if let Some(ref val) = self.mntry_val { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.pdg_pric { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.othr { if let Err(e) = val.validate() { return Err(e); } }
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
		pub tp: Option<String>,
	}
	
	impl SecuritiesTransactionPrice5 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.tp {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 35 {
					return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
				}
			}
			Ok(())
		}
	}
	
	
	// Security48 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if let Some(ref val) = self.id { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.clssfctn_tp { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.qty { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.nmnl_val { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.qlty { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.mtrty { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.issr_id { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.issr_ctry { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.tp { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref val) = self.unit_pric { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.exclsv_arrgmnt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.mkt_val { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.avlbl_for_coll_reuse { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.hrcut_or_mrgn { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SecurityCommodity7Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SecurityCommodity7Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Scty", skip_serializing_if = "Option::is_none") )]
		pub scty: Option<Vec<Security48>>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none") )]
		pub cmmdty: Option<Vec<Commodity42>>,
	}
	
	impl SecurityCommodity7Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref vec) = self.scty { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.cmmdty { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// SecurityCommodityCash4 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if let Some(ref vec) = self.scty { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.cmmdty { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			if let Some(ref vec) = self.csh { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// SecurityIdentification26Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SecurityIdentification26Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
		pub id: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NotAvlbl", skip_serializing_if = "Option::is_none") )]
		pub not_avlbl: Option<NotAvailable1Code>,
	}
	
	impl SecurityIdentification26Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.id {
				let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
				if !pattern.is_match(&val) {
					return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
				}
			}
			if let Some(ref val) = self.not_avlbl { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// SpecialCollateral1Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct SupplementaryData1 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none") )]
		pub plc_and_nm: Option<String>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Envlp") )]
		pub envlp: SupplementaryDataEnvelope1,
	}
	
	impl SupplementaryData1 {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.plc_and_nm {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "plc_and_nm is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 350 {
					return Err(ValidationError::new(1002, "plc_and_nm exceeds the maximum length of 350".to_string()));
				}
			}
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
	
	
	// TradeData28 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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
			if let Some(ref vec) = self.pairg_rcncltn_sts { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			for item in &self.rcncltn_rpt { if let Err(e) = item.validate() { return Err(e); } }
			if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TradeData34Choice ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeData34Choice {
		#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
		pub data_set_actn: Option<ReportPeriodActivity1Code>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Rpt", skip_serializing_if = "Option::is_none") )]
		pub rpt: Option<Vec<TradeData28>>,
	}
	
	impl TradeData34Choice {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref val) = self.data_set_actn { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref vec) = self.rpt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
			Ok(())
		}
	}
	
	
	// TradeTransactionIdentification19 ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct TradeTransactionIdentification19 {
		#[cfg_attr( feature = "derive_serde", serde(rename = "RptgCtrPty") )]
		pub rptg_ctr_pty: OrganisationIdentification15Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "OthrCtrPty") )]
		pub othr_ctr_pty: PartyIdentification236Choice,
		#[cfg_attr( feature = "derive_serde", serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none") )]
		pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "UnqTradIdr", skip_serializing_if = "Option::is_none") )]
		pub unq_trad_idr: Option<String>,
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
			if let Some(ref val) = self.ntty_rspnsbl_for_rpt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.unq_trad_idr {
				if val.chars().count() < 1 {
					return Err(ValidationError::new(1001, "unq_trad_idr is shorter than the minimum length of 1".to_string()));
				}
				if val.chars().count() > 52 {
					return Err(ValidationError::new(1002, "unq_trad_idr exceeds the maximum length of 52".to_string()));
				}
			}
			if let Some(ref val) = self.mstr_agrmt { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.agt_lndr { if let Err(e) = val.validate() { return Err(e); } }
			if let Some(ref val) = self.trpty_agt { if let Err(e) = val.validate() { return Err(e); } }
			Ok(())
		}
	}
	
	
	// UnitOfMeasure11Code ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
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