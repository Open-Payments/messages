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


// AmountHaircutMargin1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AmountHaircutMargin1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: AmountAndDirection53,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HrcutOrMrgn", skip_serializing_if = "Option::is_none") )]
	pub hrcut_or_mrgn: Option<f64>,
}

impl AmountHaircutMargin1 {
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


// Branch5Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Branch5Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<OrganisationIdentification15Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
	pub ctry: Option<String>,
}

impl Branch5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// Branch6Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Branch6Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<PartyIdentification236Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
	pub ctry: Option<String>,
}

impl Branch6Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// Cleared16Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Cleared16Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Clrd", skip_serializing_if = "Option::is_none") )]
	pub clrd: Option<ClearingPartyAndTime14>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NonClrd", skip_serializing_if = "Option::is_none") )]
	pub non_clrd: Option<NoReasonCode>,
}

impl Cleared16Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.clrd { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.non_clrd { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// ClearingPartyAndTime14 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ClearingPartyAndTime14 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CCP", skip_serializing_if = "Option::is_none") )]
	pub ccp: Option<OrganisationIdentification15Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrDtTm", skip_serializing_if = "Option::is_none") )]
	pub clr_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptTrckgNb", skip_serializing_if = "Option::is_none") )]
	pub rpt_trckg_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtflCd", skip_serializing_if = "Option::is_none") )]
	pub prtfl_cd: Option<String>,
}

impl ClearingPartyAndTime14 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ccp { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.rpt_trckg_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rpt_trckg_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 52 {
				return Err(ValidationError::new(1002, "rpt_trckg_nb exceeds the maximum length of 52".to_string()));
			}
		}
		if let Some(ref val) = self.prtfl_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtfl_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 52 {
				return Err(ValidationError::new(1002, "prtfl_cd exceeds the maximum length of 52".to_string()));
			}
		}
		Ok(())
	}
}


// Collateral52 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Collateral52 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollValDt", skip_serializing_if = "Option::is_none") )]
	pub coll_val_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AsstTp", skip_serializing_if = "Option::is_none") )]
	pub asst_tp: Option<CollateralType21>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetXpsrCollstnInd", skip_serializing_if = "Option::is_none") )]
	pub net_xpsr_collstn_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BsktIdr", skip_serializing_if = "Option::is_none") )]
	pub bskt_idr: Option<SecurityIdentification26Choice>,
}

impl Collateral52 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.asst_tp { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.bskt_idr { if let Err(e) = val.validate() { return Err(e); } }
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


// CollateralFlag13Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CollateralFlag13Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Collsd", skip_serializing_if = "Option::is_none") )]
	pub collsd: Option<CollaterisedData12>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Uncollsd", skip_serializing_if = "Option::is_none") )]
	pub uncollsd: Option<NoReasonCode>,
}

impl CollateralFlag13Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.collsd { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.uncollsd { if let Err(e) = val.validate() { return Err(e); } }
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


// CollateralType21 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CollateralType21 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Scty", skip_serializing_if = "Option::is_none") )]
	pub scty: Option<Vec<Security52>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Csh", skip_serializing_if = "Option::is_none") )]
	pub csh: Option<Vec<AmountHaircutMargin1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none") )]
	pub cmmdty: Option<Vec<Commodity43>>,
}

impl CollateralType21 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.scty { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref vec) = self.csh { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref vec) = self.cmmdty { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// CollaterisedData12 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CollaterisedData12 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollValDt", skip_serializing_if = "Option::is_none") )]
	pub coll_val_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AsstTp", skip_serializing_if = "Option::is_none") )]
	pub asst_tp: Option<CollateralType21>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NetXpsrCollstnInd", skip_serializing_if = "Option::is_none") )]
	pub net_xpsr_collstn_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BsktIdr", skip_serializing_if = "Option::is_none") )]
	pub bskt_idr: Option<SecurityIdentification26Choice>,
}

impl CollaterisedData12 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.asst_tp { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.bskt_idr { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// Commodity43 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Commodity43 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Clssfctn", skip_serializing_if = "Option::is_none") )]
	pub clssfctn: Option<AssetClassCommodity5Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Qty", skip_serializing_if = "Option::is_none") )]
	pub qty: Option<Quantity17>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitPric", skip_serializing_if = "Option::is_none") )]
	pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktVal", skip_serializing_if = "Option::is_none") )]
	pub mkt_val: Option<AmountAndDirection53>,
}

impl Commodity43 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.clssfctn { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.qty { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.unit_pric { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.mkt_val { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// ContractModification3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ContractModification3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ActnTp") )]
	pub actn_tp: TransactionOperationType6Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Lvl", skip_serializing_if = "Option::is_none") )]
	pub lvl: Option<ModificationLevel1Code>,
}

impl ContractModification3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.actn_tp.validate() { return Err(e); }
		if let Some(ref val) = self.lvl { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// ContractTerm7Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ContractTerm7Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Opn", skip_serializing_if = "Option::is_none") )]
	pub opn: Option<FixedOpenTermContract2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Fxd", skip_serializing_if = "Option::is_none") )]
	pub fxd: Option<FixedOpenTermContract2>,
}

impl ContractTerm7Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.opn { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.fxd { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// CounterpartyData88 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CounterpartyData88 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgDtTm") )]
	pub rptg_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptSubmitgNtty") )]
	pub rpt_submitg_ntty: OrganisationIdentification15Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPty") )]
	pub ctr_pty: Vec<CounterpartyData89>,
}

impl CounterpartyData88 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.rpt_submitg_ntty.validate() { return Err(e); }
		for item in &self.ctr_pty { if let Err(e) = item.validate() { return Err(e); } }
		Ok(())
	}
}


// CounterpartyData89 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CounterpartyData89 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgCtrPty") )]
	pub rptg_ctr_pty: CounterpartyIdentification11,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrCtrPty") )]
	pub othr_ctr_pty: CounterpartyIdentification12,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none") )]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPtyData", skip_serializing_if = "Option::is_none") )]
	pub othr_pty_data: Option<TransactionCounterpartyData11>,
}

impl CounterpartyData89 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.rptg_ctr_pty.validate() { return Err(e); }
		if let Err(e) = self.othr_ctr_pty.validate() { return Err(e); }
		if let Some(ref val) = self.ntty_rspnsbl_for_rpt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.othr_pty_data { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// CounterpartyIdentification11 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CounterpartyIdentification11 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: OrganisationIdentification15Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ntr", skip_serializing_if = "Option::is_none") )]
	pub ntr: Option<CounterpartyTradeNature7Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Brnch", skip_serializing_if = "Option::is_none") )]
	pub brnch: Option<Branch5Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sd", skip_serializing_if = "Option::is_none") )]
	pub sd: Option<CollateralRole1Code>,
}

impl CounterpartyIdentification11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref val) = self.ntr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.brnch { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.sd { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// CounterpartyIdentification12 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CounterpartyIdentification12 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: PartyIdentification236Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Brnch", skip_serializing_if = "Option::is_none") )]
	pub brnch: Option<Branch6Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryCd", skip_serializing_if = "Option::is_none") )]
	pub ctry_cd: Option<String>,
}

impl CounterpartyIdentification12 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.id.validate() { return Err(e); }
		if let Some(ref val) = self.brnch { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.ctry_cd {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "ctry_cd does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// CounterpartyTradeNature7Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CounterpartyTradeNature7Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FI", skip_serializing_if = "Option::is_none") )]
	pub fi: Option<FinancialPartyClassification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NFI", skip_serializing_if = "Option::is_none") )]
	pub nfi: Option<Vec<FinancialPartyClassification2>>,
}

impl CounterpartyTradeNature7Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.fi { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.nfi { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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


// FinancialPartyClassification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialPartyClassification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Clssfctn") )]
	pub clssfctn: Vec<FinancialPartySectorType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstmtFndClssfctn", skip_serializing_if = "Option::is_none") )]
	pub invstmt_fnd_clssfctn: Option<FundType2Code>,
}

impl FinancialPartyClassification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.clssfctn { if let Err(e) = item.validate() { return Err(e); } }
		if let Some(ref val) = self.invstmt_fnd_clssfctn { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// FinancialPartyClassification2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialPartyClassification2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Clssfctn") )]
	pub clssfctn: Vec<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstmtFndClssfctn", skip_serializing_if = "Option::is_none") )]
	pub invstmt_fnd_clssfctn: Option<FundType2Code>,
}

impl FinancialPartyClassification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.clssfctn {
			let pattern = Regex::new("[A-U]{1,1}").unwrap();
			if !pattern.is_match(&item) {
				return Err(ValidationError::new(1005, "clssfctn does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.invstmt_fnd_clssfctn { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// FinancialPartySectorType2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum FinancialPartySectorType2Code {
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
}

impl FinancialPartySectorType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FixedOpenTermContract2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FixedOpenTermContract2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none") )]
	pub mtrty_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TermntnOptn", skip_serializing_if = "Option::is_none") )]
	pub termntn_optn: Option<RepoTerminationOption2Code>,
}

impl FixedOpenTermContract2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.termntn_optn { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// FixedRate11 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FixedRate11 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
	pub rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DayCntBsis", skip_serializing_if = "Option::is_none") )]
	pub day_cnt_bsis: Option<InterestComputationMethodFormat6Choice>,
}

impl FixedRate11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.day_cnt_bsis { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// FloatingInterestRate22 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FloatingInterestRate22 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefRate", skip_serializing_if = "Option::is_none") )]
	pub ref_rate: Option<BenchmarkCurveName10Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Term", skip_serializing_if = "Option::is_none") )]
	pub term: Option<InterestRateContractTerm2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtFrqcy", skip_serializing_if = "Option::is_none") )]
	pub pmt_frqcy: Option<InterestRateContractTerm2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RstFrqcy", skip_serializing_if = "Option::is_none") )]
	pub rst_frqcy: Option<InterestRateContractTerm2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sprd", skip_serializing_if = "Option::is_none") )]
	pub sprd: Option<SecuritiesTransactionPrice18Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RateAdjstmnt", skip_serializing_if = "Option::is_none") )]
	pub rate_adjstmnt: Option<Vec<RateAdjustment1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DayCntBsis", skip_serializing_if = "Option::is_none") )]
	pub day_cnt_bsis: Option<InterestComputationMethodFormat6Choice>,
}

impl FloatingInterestRate22 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ref_rate { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.term { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.pmt_frqcy { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.rst_frqcy { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.sprd { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.rate_adjstmnt { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref val) = self.day_cnt_bsis { if let Err(e) = val.validate() { return Err(e); } }
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


// FundType2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum FundType2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ETFT") )]
	CodeETFT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MMFT") )]
	CodeMMFT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OTHR") )]
	CodeOTHR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "REIT") )]
	CodeREIT,
}

impl FundType2Code {
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


// InterestRate27Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InterestRate27Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Fxd", skip_serializing_if = "Option::is_none") )]
	pub fxd: Option<FixedRate11>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Fltg", skip_serializing_if = "Option::is_none") )]
	pub fltg: Option<FloatingInterestRate22>,
}

impl InterestRate27Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.fxd { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.fltg { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// InterestRate6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InterestRate6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: AmountAndDirection53,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstRate") )]
	pub intrst_rate: InterestRate27Choice,
}

impl InterestRate6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.amt.validate() { return Err(e); }
		if let Err(e) = self.intrst_rate.validate() { return Err(e); }
		Ok(())
	}
}


// InterestRateContractTerm2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InterestRateContractTerm2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Unit") )]
	pub unit: RateBasis1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
	pub val: f64,
}

impl InterestRateContractTerm2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.unit.validate() { return Err(e); }
		Ok(())
	}
}


// LoanData139 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct LoanData139 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnqTradIdr", skip_serializing_if = "Option::is_none") )]
	pub unq_trad_idr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EvtDt", skip_serializing_if = "Option::is_none") )]
	pub evt_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExctnDtTm", skip_serializing_if = "Option::is_none") )]
	pub exctn_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSts", skip_serializing_if = "Option::is_none") )]
	pub clr_sts: Option<Cleared16Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgVn", skip_serializing_if = "Option::is_none") )]
	pub tradg_vn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MstrAgrmt", skip_serializing_if = "Option::is_none") )]
	pub mstr_agrmt: Option<MasterAgreement7>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValDt", skip_serializing_if = "Option::is_none") )]
	pub val_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MinNtcePrd", skip_serializing_if = "Option::is_none") )]
	pub min_ntce_prd: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EarlstCallBckDt", skip_serializing_if = "Option::is_none") )]
	pub earlst_call_bck_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GnlColl", skip_serializing_if = "Option::is_none") )]
	pub gnl_coll: Option<SpecialCollateral1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DlvryByVal", skip_serializing_if = "Option::is_none") )]
	pub dlvry_by_val: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollDlvryMtd", skip_serializing_if = "Option::is_none") )]
	pub coll_dlvry_mtd: Option<CollateralDeliveryMethod1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Term", skip_serializing_if = "Option::is_none") )]
	pub term: Option<Vec<ContractTerm7Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none") )]
	pub intrst_rate: Option<InterestRate27Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrncplAmt", skip_serializing_if = "Option::is_none") )]
	pub prncpl_amt: Option<PrincipalAmount3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none") )]
	pub termntn_dt: Option<String>,
}

impl LoanData139 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.unq_trad_idr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "unq_trad_idr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 52 {
				return Err(ValidationError::new(1002, "unq_trad_idr exceeds the maximum length of 52".to_string()));
			}
		}
		if let Some(ref val) = self.clr_sts { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.tradg_vn {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "tradg_vn does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.mstr_agrmt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.min_ntce_prd {
			if *val < 0.000000 {
				return Err(ValidationError::new(1003, "min_ntce_prd is less than the minimum value of 0.000000".to_string()));
			}
		}
		if let Some(ref val) = self.gnl_coll { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.coll_dlvry_mtd { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.term { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref val) = self.intrst_rate { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.prncpl_amt { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// LoanData140 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct LoanData140 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnqTradIdr", skip_serializing_if = "Option::is_none") )]
	pub unq_trad_idr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EvtDt") )]
	pub evt_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExctnDtTm", skip_serializing_if = "Option::is_none") )]
	pub exctn_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSts", skip_serializing_if = "Option::is_none") )]
	pub clr_sts: Option<Cleared16Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgVn", skip_serializing_if = "Option::is_none") )]
	pub tradg_vn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MstrAgrmt", skip_serializing_if = "Option::is_none") )]
	pub mstr_agrmt: Option<MasterAgreement7>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValDt", skip_serializing_if = "Option::is_none") )]
	pub val_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none") )]
	pub mtrty_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GnlColl", skip_serializing_if = "Option::is_none") )]
	pub gnl_coll: Option<SpecialCollateral1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrncplAmt", skip_serializing_if = "Option::is_none") )]
	pub prncpl_amt: Option<PrincipalAmount3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitPric", skip_serializing_if = "Option::is_none") )]
	pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none") )]
	pub termntn_dt: Option<String>,
}

impl LoanData140 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.unq_trad_idr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "unq_trad_idr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 52 {
				return Err(ValidationError::new(1002, "unq_trad_idr exceeds the maximum length of 52".to_string()));
			}
		}
		if let Some(ref val) = self.clr_sts { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.tradg_vn {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "tradg_vn does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.mstr_agrmt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.gnl_coll { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.prncpl_amt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.unit_pric { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// LoanData141 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct LoanData141 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnqTradIdr", skip_serializing_if = "Option::is_none") )]
	pub unq_trad_idr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EvtDt") )]
	pub evt_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExctnDtTm", skip_serializing_if = "Option::is_none") )]
	pub exctn_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSts", skip_serializing_if = "Option::is_none") )]
	pub clr_sts: Option<Cleared16Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgVn", skip_serializing_if = "Option::is_none") )]
	pub tradg_vn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MstrAgrmt", skip_serializing_if = "Option::is_none") )]
	pub mstr_agrmt: Option<MasterAgreement7>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValDt", skip_serializing_if = "Option::is_none") )]
	pub val_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GnlColl", skip_serializing_if = "Option::is_none") )]
	pub gnl_coll: Option<SpecialCollateral1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DlvryByVal", skip_serializing_if = "Option::is_none") )]
	pub dlvry_by_val: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollDlvryMtd", skip_serializing_if = "Option::is_none") )]
	pub coll_dlvry_mtd: Option<CollateralDeliveryMethod1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Term", skip_serializing_if = "Option::is_none") )]
	pub term: Option<Vec<ContractTerm7Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AsstTp", skip_serializing_if = "Option::is_none") )]
	pub asst_tp: Option<SecurityCommodity9>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LnVal", skip_serializing_if = "Option::is_none") )]
	pub ln_val: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RbtRate", skip_serializing_if = "Option::is_none") )]
	pub rbt_rate: Option<InterestRate27Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LndgFee", skip_serializing_if = "Option::is_none") )]
	pub lndg_fee: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none") )]
	pub termntn_dt: Option<String>,
}

impl LoanData141 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.unq_trad_idr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "unq_trad_idr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 52 {
				return Err(ValidationError::new(1002, "unq_trad_idr exceeds the maximum length of 52".to_string()));
			}
		}
		if let Some(ref val) = self.clr_sts { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.tradg_vn {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "tradg_vn does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.mstr_agrmt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.gnl_coll { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.coll_dlvry_mtd { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.term { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref val) = self.asst_tp { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.ln_val { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.rbt_rate { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// LoanData142 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct LoanData142 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnqTradIdr", skip_serializing_if = "Option::is_none") )]
	pub unq_trad_idr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EvtDt") )]
	pub evt_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExctnDtTm", skip_serializing_if = "Option::is_none") )]
	pub exctn_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgVn", skip_serializing_if = "Option::is_none") )]
	pub tradg_vn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollDlvryMtd", skip_serializing_if = "Option::is_none") )]
	pub coll_dlvry_mtd: Option<CollateralDeliveryMethod1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OutsdngMrgnLnAmt", skip_serializing_if = "Option::is_none") )]
	pub outsdng_mrgn_ln_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtMktValAmt", skip_serializing_if = "Option::is_none") )]
	pub shrt_mkt_val_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MrgnLnAttr", skip_serializing_if = "Option::is_none") )]
	pub mrgn_ln_attr: Option<Vec<InterestRate6>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none") )]
	pub termntn_dt: Option<String>,
}

impl LoanData142 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.unq_trad_idr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "unq_trad_idr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 52 {
				return Err(ValidationError::new(1002, "unq_trad_idr exceeds the maximum length of 52".to_string()));
			}
		}
		if let Some(ref val) = self.tradg_vn {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "tradg_vn does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.coll_dlvry_mtd { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.outsdng_mrgn_ln_amt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.shrt_mkt_val_amt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.mrgn_ln_attr { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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


// PrincipalAmount3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PrincipalAmount3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValDtAmt", skip_serializing_if = "Option::is_none") )]
	pub val_dt_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MtrtyDtAmt", skip_serializing_if = "Option::is_none") )]
	pub mtrty_dt_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl PrincipalAmount3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.val_dt_amt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.mtrty_dt_amt { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// Quantity17 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Quantity17 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
	pub val: f64,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitOfMeasr") )]
	pub unit_of_measr: UnitOfMeasure11Code,
}

impl Quantity17 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.unit_of_measr.validate() { return Err(e); }
		Ok(())
	}
}


// QuantityNominalValue2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct QuantityNominalValue2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Qty", skip_serializing_if = "Option::is_none") )]
	pub qty: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmnlVal", skip_serializing_if = "Option::is_none") )]
	pub nmnl_val: Option<AmountAndDirection53>,
}

impl QuantityNominalValue2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nmnl_val { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// RateAdjustment1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RateAdjustment1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rate") )]
	pub rate: f64,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdjstmntDt") )]
	pub adjstmnt_dt: String,
}

impl RateAdjustment1 {
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


// ReconciliationFlag2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ReconciliationFlag2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptTp", skip_serializing_if = "Option::is_none") )]
	pub rpt_tp: Option<TradeRepositoryReportingType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BothCtrPtiesRptg", skip_serializing_if = "Option::is_none") )]
	pub both_ctr_pties_rptg: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PairdSts", skip_serializing_if = "Option::is_none") )]
	pub paird_sts: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LnRcncltnSts", skip_serializing_if = "Option::is_none") )]
	pub ln_rcncltn_sts: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollRcncltnSts", skip_serializing_if = "Option::is_none") )]
	pub coll_rcncltn_sts: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModSts", skip_serializing_if = "Option::is_none") )]
	pub mod_sts: Option<bool>,
}

impl ReconciliationFlag2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rpt_tp { if let Err(e) = val.validate() { return Err(e); } }
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


// SecuritiesFinancingReportingTransactionStateReportV02 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesFinancingReportingTransactionStateReportV02 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradData") )]
	pub trad_data: TradeStateReport5Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl SecuritiesFinancingReportingTransactionStateReportV02 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.trad_data.validate() { return Err(e); }
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


// SecuritiesTransactionPrice18Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesTransactionPrice18Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MntryVal", skip_serializing_if = "Option::is_none") )]
	pub mntry_val: Option<AmountAndDirection107>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pctg", skip_serializing_if = "Option::is_none") )]
	pub pctg: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dcml", skip_serializing_if = "Option::is_none") )]
	pub dcml: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BsisPts", skip_serializing_if = "Option::is_none") )]
	pub bsis_pts: Option<f64>,
}

impl SecuritiesTransactionPrice18Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mntry_val { if let Err(e) = val.validate() { return Err(e); } }
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


// Security51 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Security51 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none") )]
	pub clssfctn_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QtyOrNmnlVal", skip_serializing_if = "Option::is_none") )]
	pub qty_or_nmnl_val: Option<QuantityNominalValue2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitPric", skip_serializing_if = "Option::is_none") )]
	pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktVal", skip_serializing_if = "Option::is_none") )]
	pub mkt_val: Option<AmountAndDirection53>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Qlty", skip_serializing_if = "Option::is_none") )]
	pub qlty: Option<CollateralQualityType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mtrty", skip_serializing_if = "Option::is_none") )]
	pub mtrty: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<SecurityIssuer4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<Vec<SecuritiesLendingType3Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExclsvArrgmnt", skip_serializing_if = "Option::is_none") )]
	pub exclsv_arrgmnt: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AvlblForCollReuse", skip_serializing_if = "Option::is_none") )]
	pub avlbl_for_coll_reuse: Option<bool>,
}

impl Security51 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.clssfctn_tp {
			let pattern = Regex::new("[A-Z]{6,6}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "clssfctn_tp does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.qty_or_nmnl_val { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.unit_pric { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.mkt_val { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.qlty { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.issr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.tp { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// Security52 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Security52 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none") )]
	pub clssfctn_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QtyOrNmnlVal", skip_serializing_if = "Option::is_none") )]
	pub qty_or_nmnl_val: Option<QuantityNominalValue2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitPric", skip_serializing_if = "Option::is_none") )]
	pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktVal", skip_serializing_if = "Option::is_none") )]
	pub mkt_val: Option<AmountAndDirection53>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Qlty", skip_serializing_if = "Option::is_none") )]
	pub qlty: Option<CollateralQualityType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mtrty", skip_serializing_if = "Option::is_none") )]
	pub mtrty: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<SecurityIssuer4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<Vec<SecuritiesLendingType3Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExclsvArrgmnt", skip_serializing_if = "Option::is_none") )]
	pub exclsv_arrgmnt: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HrcutOrMrgn", skip_serializing_if = "Option::is_none") )]
	pub hrcut_or_mrgn: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AvlblForCollReuse", skip_serializing_if = "Option::is_none") )]
	pub avlbl_for_coll_reuse: Option<bool>,
}

impl Security52 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.clssfctn_tp {
			let pattern = Regex::new("[A-Z]{6,6}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "clssfctn_tp does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.qty_or_nmnl_val { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.unit_pric { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.mkt_val { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.qlty { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.issr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.tp { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// Security55 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Security55 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none") )]
	pub clssfctn_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QtyOrNmnlVal", skip_serializing_if = "Option::is_none") )]
	pub qty_or_nmnl_val: Option<QuantityNominalValue2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitPric", skip_serializing_if = "Option::is_none") )]
	pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktVal", skip_serializing_if = "Option::is_none") )]
	pub mkt_val: Option<AmountAndDirection53>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Qlty", skip_serializing_if = "Option::is_none") )]
	pub qlty: Option<CollateralQualityType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mtrty", skip_serializing_if = "Option::is_none") )]
	pub mtrty: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<SecurityIssuer4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<Vec<SecuritiesLendingType3Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ExclsvArrgmnt", skip_serializing_if = "Option::is_none") )]
	pub exclsv_arrgmnt: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AvlblForCollReuse", skip_serializing_if = "Option::is_none") )]
	pub avlbl_for_coll_reuse: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HrcutOrMrgn", skip_serializing_if = "Option::is_none") )]
	pub hrcut_or_mrgn: Option<f64>,
}

impl Security55 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.clssfctn_tp {
			let pattern = Regex::new("[A-Z]{6,6}").unwrap();
			if !pattern.is_match(&val) {
				return Err(ValidationError::new(1005, "clssfctn_tp does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.qty_or_nmnl_val { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.unit_pric { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.mkt_val { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.qlty { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.issr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.tp { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// SecurityCommodity9 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecurityCommodity9 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Scty", skip_serializing_if = "Option::is_none") )]
	pub scty: Option<Vec<Security51>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none") )]
	pub cmmdty: Option<Vec<Commodity43>>,
}

impl SecurityCommodity9 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.scty { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		if let Some(ref vec) = self.cmmdty { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
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


// SecurityIssuer4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecurityIssuer4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<OrganisationIdentification15Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "JursdctnCtry") )]
	pub jursdctn_ctry: String,
}

impl SecurityIssuer4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id { if let Err(e) = val.validate() { return Err(e); } }
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.jursdctn_ctry) {
			return Err(ValidationError::new(1005, "jursdctn_ctry does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// SettlementParties34Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementParties34Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CntrlSctiesDpstryPtcpt", skip_serializing_if = "Option::is_none") )]
	pub cntrl_scties_dpstry_ptcpt: Option<OrganisationIdentification15Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IndrctPtcpt", skip_serializing_if = "Option::is_none") )]
	pub indrct_ptcpt: Option<OrganisationIdentification15Choice>,
}

impl SettlementParties34Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cntrl_scties_dpstry_ptcpt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.indrct_ptcpt { if let Err(e) = val.validate() { return Err(e); } }
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


// TradeStateReport16 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradeStateReport16 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none") )]
	pub tech_rcrd_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtySpcfcData") )]
	pub ctr_pty_spcfc_data: CounterpartyData88,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LnData", skip_serializing_if = "Option::is_none") )]
	pub ln_data: Option<TransactionLoanData31Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollData", skip_serializing_if = "Option::is_none") )]
	pub coll_data: Option<TransactionCollateralData18Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RcncltnFlg", skip_serializing_if = "Option::is_none") )]
	pub rcncltn_flg: Option<ReconciliationFlag2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctMod") )]
	pub ctrct_mod: ContractModification3,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl TradeStateReport16 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tech_rcrd_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tech_rcrd_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "tech_rcrd_id exceeds the maximum length of 140".to_string()));
			}
		}
		if let Err(e) = self.ctr_pty_spcfc_data.validate() { return Err(e); }
		if let Some(ref val) = self.ln_data { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.coll_data { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.rcncltn_flg { if let Err(e) = val.validate() { return Err(e); } }
		if let Err(e) = self.ctrct_mod.validate() { return Err(e); }
		if let Some(ref vec) = self.splmtry_data { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// TradeStateReport5Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradeStateReport5Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Stat", skip_serializing_if = "Option::is_none") )]
	pub stat: Option<Vec<TradeStateReport16>>,
}

impl TradeStateReport5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.data_set_actn { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.stat { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// TransactionCollateralData18Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TransactionCollateralData18Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RpTrad", skip_serializing_if = "Option::is_none") )]
	pub rp_trad: Option<Collateral52>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BuySellBck", skip_serializing_if = "Option::is_none") )]
	pub buy_sell_bck: Option<Collateral52>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesLndg", skip_serializing_if = "Option::is_none") )]
	pub scties_lndg: Option<CollateralFlag13Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MrgnLndg", skip_serializing_if = "Option::is_none") )]
	pub mrgn_lndg: Option<Vec<Security55>>,
}

impl TransactionCollateralData18Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rp_trad { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.buy_sell_bck { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.scties_lndg { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref vec) = self.mrgn_lndg { for item in vec { if let Err(e) = item.validate() { return Err(e); } } }
		Ok(())
	}
}


// TransactionCounterpartyData11 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TransactionCounterpartyData11 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Bnfcry", skip_serializing_if = "Option::is_none") )]
	pub bnfcry: Option<PartyIdentification236Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrptyAgt", skip_serializing_if = "Option::is_none") )]
	pub trpty_agt: Option<OrganisationIdentification15Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Brkr", skip_serializing_if = "Option::is_none") )]
	pub brkr: Option<OrganisationIdentification15Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrMmb", skip_serializing_if = "Option::is_none") )]
	pub clr_mmb: Option<OrganisationIdentification15Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmPties", skip_serializing_if = "Option::is_none") )]
	pub sttlm_pties: Option<SettlementParties34Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AgtLndr", skip_serializing_if = "Option::is_none") )]
	pub agt_lndr: Option<OrganisationIdentification15Choice>,
}

impl TransactionCounterpartyData11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.bnfcry { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.trpty_agt { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.brkr { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.clr_mmb { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.sttlm_pties { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.agt_lndr { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// TransactionLoanData31Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TransactionLoanData31Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RpTrad", skip_serializing_if = "Option::is_none") )]
	pub rp_trad: Option<LoanData139>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BuySellBck", skip_serializing_if = "Option::is_none") )]
	pub buy_sell_bck: Option<LoanData140>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesLndg", skip_serializing_if = "Option::is_none") )]
	pub scties_lndg: Option<LoanData141>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MrgnLndg", skip_serializing_if = "Option::is_none") )]
	pub mrgn_lndg: Option<LoanData142>,
}

impl TransactionLoanData31Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rp_trad { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.buy_sell_bck { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.scties_lndg { if let Err(e) = val.validate() { return Err(e); } }
		if let Some(ref val) = self.mrgn_lndg { if let Err(e) = val.validate() { return Err(e); } }
		Ok(())
	}
}


// TransactionOperationType6Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TransactionOperationType6Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "REUU") )]
	CodeREUU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "COLU") )]
	CodeCOLU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CORR") )]
	CodeCORR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ETRM") )]
	CodeETRM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VALU") )]
	CodeVALU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "POSC") )]
	CodePOSC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NEWT") )]
	CodeNEWT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MODI") )]
	CodeMODI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MARU") )]
	CodeMARU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EROR") )]
	CodeEROR,
}

impl TransactionOperationType6Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
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
