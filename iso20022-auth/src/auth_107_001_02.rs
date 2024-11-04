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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
		if let Some(ref val) = self.addtl_sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
		if let Some(ref val) = self.addtl_sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
		if let Some(ref val) = self.addtl_sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
		if let Some(ref val) = self.addtl_sub_pdct { val.validate()? }
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
		self.amt.validate()?;
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
		if let Some(ref val) = self.amt { val.validate()? }
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
		if let Some(ref val) = self.agrcltrl { val.validate()? }
		if let Some(ref val) = self.nrgy { val.validate()? }
		if let Some(ref val) = self.envttl { val.validate()? }
		if let Some(ref val) = self.frtlzr { val.validate()? }
		if let Some(ref val) = self.frght { val.validate()? }
		if let Some(ref val) = self.indx { val.validate()? }
		if let Some(ref val) = self.indstrl_pdct { val.validate()? }
		if let Some(ref val) = self.infltn { val.validate()? }
		if let Some(ref val) = self.metl { val.validate()? }
		if let Some(ref val) = self.multi_cmmdty_extc { val.validate()? }
		if let Some(ref val) = self.offcl_ecnmc_sttstcs { val.validate()? }
		if let Some(ref val) = self.othr { val.validate()? }
		if let Some(ref val) = self.othr_c10 { val.validate()? }
		if let Some(ref val) = self.ppr { val.validate()? }
		if let Some(ref val) = self.plprpln { val.validate()? }
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
		if let Some(ref val) = self.grn_oil_seed { val.validate()? }
		if let Some(ref val) = self.soft { val.validate()? }
		if let Some(ref val) = self.ptt { val.validate()? }
		if let Some(ref val) = self.olv_oil { val.validate()? }
		if let Some(ref val) = self.dairy { val.validate()? }
		if let Some(ref val) = self.frstry { val.validate()? }
		if let Some(ref val) = self.sfd { val.validate()? }
		if let Some(ref val) = self.live_stock { val.validate()? }
		if let Some(ref val) = self.grn { val.validate()? }
		if let Some(ref val) = self.othr { val.validate()? }
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
		self.base_pdct.validate()?;
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
		if let Some(ref val) = self.elctrcty { val.validate()? }
		if let Some(ref val) = self.ntrl_gas { val.validate()? }
		if let Some(ref val) = self.oil { val.validate()? }
		if let Some(ref val) = self.coal { val.validate()? }
		if let Some(ref val) = self.intr_nrgy { val.validate()? }
		if let Some(ref val) = self.rnwbl_nrgy { val.validate()? }
		if let Some(ref val) = self.lght_end { val.validate()? }
		if let Some(ref val) = self.dstllts { val.validate()? }
		if let Some(ref val) = self.othr { val.validate()? }
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
		if let Some(ref val) = self.emssns { val.validate()? }
		if let Some(ref val) = self.wthr { val.validate()? }
		if let Some(ref val) = self.crbn_rltd { val.validate()? }
		if let Some(ref val) = self.othr { val.validate()? }
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
		if let Some(ref val) = self.ammn { val.validate()? }
		if let Some(ref val) = self.dmmnm_phspht { val.validate()? }
		if let Some(ref val) = self.ptsh { val.validate()? }
		if let Some(ref val) = self.slphr { val.validate()? }
		if let Some(ref val) = self.urea { val.validate()? }
		if let Some(ref val) = self.urea_and_ammnm_ntrt { val.validate()? }
		if let Some(ref val) = self.othr { val.validate()? }
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
		if let Some(ref val) = self.dry { val.validate()? }
		if let Some(ref val) = self.wet { val.validate()? }
		if let Some(ref val) = self.cntnr_ship { val.validate()? }
		if let Some(ref val) = self.othr { val.validate()? }
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
		self.base_pdct.validate()?;
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
		if let Some(ref val) = self.cnstrctn { val.validate()? }
		if let Some(ref val) = self.manfctg { val.validate()? }
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
		self.base_pdct.validate()?;
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
		if let Some(ref val) = self.non_prcs { val.validate()? }
		if let Some(ref val) = self.prcs { val.validate()? }
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
		self.base_pdct.validate()?;
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
		self.base_pdct.validate()?;
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
		self.base_pdct.validate()?;
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
		if let Some(ref val) = self.cntnr_brd { val.validate()? }
		if let Some(ref val) = self.nwsprnt { val.validate()? }
		if let Some(ref val) = self.pulp { val.validate()? }
		if let Some(ref val) = self.rcvrd_ppr { val.validate()? }
		if let Some(ref val) = self.othr { val.validate()? }
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
		if let Some(ref val) = self.plstc { val.validate()? }
		if let Some(ref val) = self.othr { val.validate()? }
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
		self.instrm_id.validate()?;
		if let Some(ref val) = self.unit_of_measr { val.validate()? }
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
		if let Some(ref val) = self.clrd { val.validate()? }
		if let Some(ref val) = self.intnd_to_clear { val.validate()? }
		if let Some(ref val) = self.non_clrd { val.validate()? }
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
		self.rptg_ctr_pty.validate()?;
		if let Some(ref val) = self.othr_ctr_pty { val.validate()? }
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
		if let Some(ref val) = self.rsn { val.validate()? }
		if let Some(ref val) = self.ctr_pties { val.validate()? }
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
		if let Some(ref val) = self.rsn { val.validate()? }
		if let Some(ref val) = self.dtls { val.validate()? }
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
		if let Some(ref val) = self.ccp { val.validate()? }
		if let Some(ref val) = self.clr_idr { val.validate()? }
		if let Some(ref val) = self.orgnl_idr { val.validate()? }
		if let Some(ref val) = self.orgnl_trad_rpstry_idr { val.validate()? }
		if let Some(ref val) = self.clr_acct_orgn { val.validate()? }
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
		if let Some(ref val) = self.rsn { val.validate()? }
		if let Some(ref val) = self.dtls { val.validate()? }
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
		if let Some(ref val) = self.ccp { val.validate()? }
		if let Some(ref val) = self.clr_idr { val.validate()? }
		if let Some(ref val) = self.orgnl_idr { val.validate()? }
		if let Some(ref val) = self.orgnl_trad_rpstry_idr { val.validate()? }
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
		if let Some(ref val) = self.prtfl { val.validate()? }
		if let Some(ref val) = self.mrgn_prtfl_cd { val.validate()? }
		Ok(())
	}
}


// CommonTradeDataReport72 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CommonTradeDataReport72 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctData", skip_serializing_if = "Option::is_none") )]
	pub ctrct_data: Option<ContractType15>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxData") )]
	pub tx_data: TradeTransaction50,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctMod", skip_serializing_if = "Option::is_none") )]
	pub ctrct_mod: Option<ContractModification9>,
}

impl CommonTradeDataReport72 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ctrct_data { val.validate()? }
		self.tx_data.validate()?;
		if let Some(ref val) = self.ctrct_mod { val.validate()? }
		Ok(())
	}
}


// ContractModification9 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ContractModification9 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ActnTp", skip_serializing_if = "Option::is_none") )]
	pub actn_tp: Option<TransactionOperationType10Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Lvl", skip_serializing_if = "Option::is_none") )]
	pub lvl: Option<ModificationLevel1Code>,
}

impl ContractModification9 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.actn_tp { val.validate()? }
		if let Some(ref val) = self.lvl { val.validate()? }
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
	pub pdct_clssfctn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PdctId", skip_serializing_if = "Option::is_none") )]
	pub pdct_id: Option<SecurityIdentification46>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygInstrm", skip_serializing_if = "Option::is_none") )]
	pub undrlyg_instrm: Option<SecurityIdentification41Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygAsstTradgPltfmIdr", skip_serializing_if = "Option::is_none") )]
	pub undrlyg_asst_tradg_pltfm_idr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygAsstPricSrc", skip_serializing_if = "Option::is_none") )]
	pub undrlyg_asst_pric_src: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none") )]
	pub sttlm_ccy: Option<CurrencyExchange23>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmCcyScndLeg", skip_serializing_if = "Option::is_none") )]
	pub sttlm_ccy_scnd_leg: Option<CurrencyExchange23>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PlcOfSttlm", skip_serializing_if = "Option::is_none") )]
	pub plc_of_sttlm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DerivBasedOnCrptAsst", skip_serializing_if = "Option::is_none") )]
	pub deriv_based_on_crpt_asst: Option<bool>,
}

impl ContractType15 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ctrct_tp { val.validate()? }
		if let Some(ref val) = self.asst_clss { val.validate()? }
		if let Some(ref val) = self.pdct_clssfctn {
			let pattern = Regex::new("[A-Z]{6,6}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "pdct_clssfctn does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pdct_id { val.validate()? }
		if let Some(ref val) = self.undrlyg_instrm { val.validate()? }
		if let Some(ref val) = self.undrlyg_asst_tradg_pltfm_idr {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "undrlyg_asst_tradg_pltfm_idr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.undrlyg_asst_pric_src {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "undrlyg_asst_pric_src is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 50 {
				return Err(ValidationError::new(1002, "undrlyg_asst_pric_src exceeds the maximum length of 50".to_string()));
			}
		}
		if let Some(ref val) = self.sttlm_ccy { val.validate()? }
		if let Some(ref val) = self.sttlm_ccy_scnd_leg { val.validate()? }
		if let Some(ref val) = self.plc_of_sttlm {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "plc_of_sttlm does not match the required pattern".to_string()));
			}
		}
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
		if let Some(ref val) = self.ctrct_val { val.validate()? }
		if let Some(ref val) = self.tp { val.validate()? }
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
	pub tradr_lctn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BookgLctn", skip_serializing_if = "Option::is_none") )]
	pub bookg_lctn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgXmptn", skip_serializing_if = "Option::is_none") )]
	pub rptg_xmptn: Option<ReportingExemption1>,
}

impl Counterparty45 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		if let Some(ref val) = self.ntr { val.validate()? }
		if let Some(ref val) = self.tradg_cpcty { val.validate()? }
		if let Some(ref val) = self.drctn_or_sd { val.validate()? }
		if let Some(ref val) = self.tradr_lctn {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "tradr_lctn does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.bookg_lctn {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "bookg_lctn does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.rptg_xmptn { val.validate()? }
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
		if let Some(ref val) = self.id_tp { val.validate()? }
		if let Some(ref val) = self.ntr { val.validate()? }
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
		self.ctr_pty.validate()?;
		if let Some(ref val) = self.valtn { val.validate()? }
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
		if let Some(ref val) = self.fi { val.validate()? }
		if let Some(ref val) = self.nfi { val.validate()? }
		if let Some(ref val) = self.cntrl_cntr_pty { val.validate()? }
		if let Some(ref val) = self.othr { val.validate()? }
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
	pub clctn_bsis: Option<String>,
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
		if let Some(ref val) = self.snrty { val.validate()? }
		if let Some(ref val) = self.ref_pty { val.validate()? }
		if let Some(ref val) = self.pmt_frqcy { val.validate()? }
		if let Some(ref val) = self.clctn_bsis {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "clctn_bsis is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "clctn_bsis exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.trch { val.validate()? }
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
	pub dlvrbl_cross_ccy: Option<String>,
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
		if let Some(ref val) = self.dlvrbl_cross_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "dlvrbl_cross_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.xchg_rate_bsis { val.validate()? }
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
	pub ccy: String,
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
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.ccy) {
			return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.xchg_rate_bsis { val.validate()? }
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
	pub strr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cnsttnts", skip_serializing_if = "Option::is_none") )]
	pub cnsttnts: Option<Vec<BasketConstituents3>>,
}

impl CustomBasket4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.strr {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "strr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 52 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 52".to_string()));
			}
		}
		if let Some(ref vec) = self.cnsttnts { for item in vec { item.validate()? } }
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
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl DeliveryInterconnectionPoint1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			let pattern = Regex::new("[A-Z0-9\\-]{16}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "cd does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 52 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 52".to_string()));
			}
		}
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
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.id { val.validate()? }
		if let Some(ref val) = self.tm_stmp { val.validate()? }
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
	pub ctry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none") )]
	pub ctry_sub_dvsn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
}

impl DerivativePartyIdentification1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ctry_sub_dvsn {
			let pattern = Regex::new("[A-Z]{2,2}\\-[0-9A-Z]{1,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry_sub_dvsn does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// DerivativesTradeStateReportV02 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DerivativesTradeStateReportV02 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptHdr") )]
	pub rpt_hdr: TradeReportHeader4,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradData") )]
	pub trad_data: TradeData60Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl DerivativesTradeStateReportV02 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rpt_hdr.validate()?;
		self.trad_data.validate()?;
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
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
		self.drctn_of_the_frst_leg.validate()?;
		if let Some(ref val) = self.drctn_of_the_scnd_leg { val.validate()? }
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
		if let Some(ref val) = self.drctn { val.validate()? }
		if let Some(ref val) = self.ctr_pty_sd { val.validate()? }
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
	pub dssmntn_idr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlDssmntnIdr", skip_serializing_if = "Option::is_none") )]
	pub orgnl_dssmntn_idr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TmStmp") )]
	pub tm_stmp: String,
}

impl DisseminationData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.dssmntn_idr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "dssmntn_idr is shorter than the minimum length of 1".to_string()));
		}
		if self.dssmntn_idr.chars().count() > 52 {
			return Err(ValidationError::new(1002, "dssmntn_idr exceeds the maximum length of 52".to_string()));
		}
		if let Some(ref val) = self.orgnl_dssmntn_idr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_dssmntn_idr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 52 {
				return Err(ValidationError::new(1002, "orgnl_dssmntn_idr exceeds the maximum length of 52".to_string()));
			}
		}
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
		if let Some(ref val) = self.addtl_sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
		if let Some(ref val) = self.addtl_sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
		if let Some(ref val) = self.addtl_sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		if let Some(ref vec) = self.dlvry_intrvl { for item in vec { item.validate()? } }
		if let Some(ref val) = self.dlvry_dt { val.validate()? }
		if let Some(ref val) = self.drtn { val.validate()? }
		if let Some(ref vec) = self.wk_day { for item in vec { item.validate()? } }
		if let Some(ref val) = self.dlvry_cpcty { val.validate()? }
		if let Some(ref val) = self.qty_unit { val.validate()? }
		if let Some(ref val) = self.pric_tm_intrvl_qty { val.validate()? }
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
	pub prtry: Option<String>,
}

impl EnergyQuantityUnit2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 52 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 52".to_string()));
			}
		}
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
		if let Some(ref vec) = self.dlvry_pt_or_zone { for item in vec { item.validate()? } }
		if let Some(ref val) = self.intr_cnnctn_pt { val.validate()? }
		if let Some(ref val) = self.ld_tp { val.validate()? }
		if let Some(ref vec) = self.dlvry_attr { for item in vec { item.validate()? } }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
		if let Some(ref val) = self.addtl_sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
	pub evt_idr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstTradRskRdctnIdr", skip_serializing_if = "Option::is_none") )]
	pub pst_trad_rsk_rdctn_idr: Option<PostTradeRiskReductionIdentifier1>,
}

impl EventIdentifier1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.evt_idr {
			let pattern = Regex::new("[A-Z0-9]{18}[0-9]{2}[A-Z0-9]{0,32}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "evt_idr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pst_trad_rsk_rdctn_idr { val.validate()? }
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
	pub base_ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QtdCcy") )]
	pub qtd_ccy: String,
}

impl ExchangeRateBasis1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.base_ccy) {
			return Err(ValidationError::new(1005, "base_ccy does not match the required pattern".to_string()));
		}
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.qtd_ccy) {
			return Err(ValidationError::new(1005, "qtd_ccy does not match the required pattern".to_string()));
		}
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
	pub prtry: Option<String>,
}

impl ExchangeRateBasis1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ccy_pair { val.validate()? }
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 52 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 52".to_string()));
			}
		}
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
		if let Some(ref val) = self.pdg_dt_aplbl { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		for item in &self.sctr { item.validate()? }
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
		if let Some(ref val) = self.nmnl_val { val.validate()? }
		if let Some(ref val) = self.mntry_val { val.validate()? }
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
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
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
		if let Some(ref val) = self.rate { val.validate()? }
		if let Some(ref val) = self.day_cnt { val.validate()? }
		if let Some(ref val) = self.pmt_frqcy { val.validate()? }
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
	pub id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
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
		if let Some(ref val) = self.id {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.rate { val.validate()? }
		if let Some(ref val) = self.ref_prd { val.validate()? }
		if let Some(ref val) = self.sprd { val.validate()? }
		if let Some(ref val) = self.day_cnt { val.validate()? }
		if let Some(ref val) = self.pmt_frqcy { val.validate()? }
		if let Some(ref val) = self.rst_frqcy { val.validate()? }
		if let Some(ref val) = self.nxt_fltg_rst { val.validate()? }
		if let Some(ref val) = self.last_fltg_rst { val.validate()? }
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
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl FloatingRateIdentification8Choice {
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
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 350".to_string()));
			}
		}
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
		if let Some(ref val) = self.addtl_sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
		if let Some(ref val) = self.addtl_sub_pdct { val.validate()? }
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


// GenericIdentification179 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericIdentification179 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericIdentification179 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 52 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 52".to_string()));
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


// GenericIdentification184 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericIdentification184 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Src") )]
	pub src: String,
}

impl GenericIdentification184 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 210 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 210".to_string()));
		}
		if self.src.chars().count() < 1 {
			return Err(ValidationError::new(1001, "src is shorter than the minimum length of 1".to_string()));
		}
		if self.src.chars().count() > 100 {
			return Err(ValidationError::new(1002, "src exceeds the maximum length of 100".to_string()));
		}
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
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericIdentification185 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 100 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 100".to_string()));
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


// IndexIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct IndexIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
	pub isin: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Indx", skip_serializing_if = "Option::is_none") )]
	pub indx: Option<String>,
}

impl IndexIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.isin {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.indx {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "indx is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "indx exceeds the maximum length of 4".to_string()));
			}
		}
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
	pub isin: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AltrntvInstrmId", skip_serializing_if = "Option::is_none") )]
	pub altrntv_instrm_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none") )]
	pub unq_pdct_idr: Option<UniqueProductIdentifier1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrId", skip_serializing_if = "Option::is_none") )]
	pub othr_id: Option<GenericIdentification184>,
}

impl InstrumentIdentification6Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.isin {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.altrntv_instrm_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "altrntv_instrm_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 52 {
				return Err(ValidationError::new(1002, "altrntv_instrm_id exceeds the maximum length of 52".to_string()));
			}
		}
		if let Some(ref val) = self.unq_pdct_idr { val.validate()? }
		if let Some(ref val) = self.othr_id { val.validate()? }
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
	pub nrrtv: Option<String>,
}

impl InterestComputationMethodFormat7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd.validate()?;
		if let Some(ref val) = self.nrrtv {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nrrtv is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 1000 {
				return Err(ValidationError::new(1002, "nrrtv exceeds the maximum length of 1000".to_string()));
			}
		}
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
		if let Some(ref val) = self.fxd { val.validate()? }
		if let Some(ref val) = self.fltg { val.validate()? }
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
		if let Some(ref val) = self.unit { val.validate()? }
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
	pub prtry: Option<String>,
}

impl InterestRateFrequency3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.term { val.validate()? }
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 52 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 52".to_string()));
			}
		}
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
		if let Some(ref val) = self.frst_leg { val.validate()? }
		if let Some(ref val) = self.scnd_leg { val.validate()? }
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
	pub ctry: Option<String>,
}

impl LegalPersonIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
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
		if let Some(ref val) = self.initl_mrgn_prtfl_cd { val.validate()? }
		if let Some(ref val) = self.vartn_mrgn_prtfl_cd { val.validate()? }
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
	pub vrsn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrMstrAgrmtDtls", skip_serializing_if = "Option::is_none") )]
	pub othr_mstr_agrmt_dtls: Option<String>,
}

impl MasterAgreement8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
		if let Some(ref val) = self.addtl_sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
		if let Some(ref val) = self.addtl_sub_pdct { val.validate()? }
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
		self.id.validate()?;
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
	pub ctry: Option<String>,
}

impl NaturalPersonIdentification3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
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
	pub non_clr_rsn_inf: Option<String>,
}

impl NonClearingReason2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.clr_xmptn_xcptn { item.validate()? }
		if let Some(ref val) = self.non_clr_rsn_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "non_clr_rsn_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "non_clr_rsn_inf exceeds the maximum length of 350".to_string()));
			}
		}
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
		for item in &self.sctr { item.validate()? }
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
		if let Some(ref val) = self.amt { val.validate()? }
		if let Some(ref vec) = self.schdl_prd { for item in vec { item.validate()? } }
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
	pub ccy: Option<String>,
}

impl NotionalAmount6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.amt { val.validate()? }
		if let Some(ref vec) = self.schdl_prd { for item in vec { item.validate()? } }
		if let Some(ref val) = self.ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
			}
		}
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
		if let Some(ref val) = self.frst_leg { val.validate()? }
		if let Some(ref val) = self.scnd_leg { val.validate()? }
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
		if let Some(ref val) = self.unit_of_measr { val.validate()? }
		if let Some(ref val) = self.dtls { val.validate()? }
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
		if let Some(ref val) = self.frst_leg { val.validate()? }
		if let Some(ref val) = self.scnd_leg { val.validate()? }
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
		if let Some(ref val) = self.sngl { val.validate()? }
		if let Some(ref val) = self.mltpl { val.validate()? }
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
		self.lwr_lvl.validate()?;
		self.upper_lvl.validate()?;
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
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.mbdd_tp { val.validate()? }
		if let Some(ref vec) = self.exrc_style { for item in vec { item.validate()? } }
		if let Some(ref val) = self.exrc_dt { val.validate()? }
		if let Some(ref val) = self.strk_pric { val.validate()? }
		if let Some(ref vec) = self.strk_pric_schdl { for item in vec { item.validate()? } }
		if let Some(ref val) = self.call_amt { val.validate()? }
		if let Some(ref val) = self.put_amt { val.validate()? }
		if let Some(ref val) = self.prm_amt { val.validate()? }
		if let Some(ref val) = self.brrr_lvls { val.validate()? }
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
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.othr { val.validate()? }
		if let Some(ref val) = self.any_bic {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
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
		self.id.validate()?;
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
		if let Some(ref val) = self.pmt_amt { val.validate()? }
		if let Some(ref val) = self.pmt_tp { val.validate()? }
		if let Some(ref val) = self.pmt_pyer { val.validate()? }
		if let Some(ref val) = self.pmt_rcvr { val.validate()? }
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
		self.tchnq.validate()?;
		if let Some(ref val) = self.svc_prvdr { val.validate()? }
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
	pub cmplx_trad_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FxSwpLkId", skip_serializing_if = "Option::is_none") )]
	pub fx_swp_lk_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pric", skip_serializing_if = "Option::is_none") )]
	pub pric: Option<SecuritiesTransactionPrice17Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sprd", skip_serializing_if = "Option::is_none") )]
	pub sprd: Option<SecuritiesTransactionPrice20Choice>,
}

impl Package4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cmplx_trad_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cmplx_trad_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 100 {
				return Err(ValidationError::new(1002, "cmplx_trad_id exceeds the maximum length of 100".to_string()));
			}
		}
		if let Some(ref val) = self.fx_swp_lk_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "fx_swp_lk_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 100 {
				return Err(ValidationError::new(1002, "fx_swp_lk_id exceeds the maximum length of 100".to_string()));
			}
		}
		if let Some(ref val) = self.pric { val.validate()? }
		if let Some(ref val) = self.sprd { val.validate()? }
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
	pub pg_nb: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LastPgInd") )]
	pub last_pg_ind: bool,
}

impl Pagination1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{1,5}").unwrap();
		if !pattern.is_match(&self.pg_nb) {
			return Err(ValidationError::new(1005, "pg_nb does not match the required pattern".to_string()));
		}
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		if let Some(ref val) = self.lgl { val.validate()? }
		if let Some(ref val) = self.ntrl { val.validate()? }
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
		if let Some(ref val) = self.lgl { val.validate()? }
		if let Some(ref val) = self.ntrl { val.validate()? }
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
	pub prtry_tp: Option<String>,
}

impl PaymentType5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.prtry_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "prtry_tp exceeds the maximum length of 4".to_string()));
			}
			let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "prtry_tp does not match the required pattern".to_string()));
			}
		}
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
		self.base_pdct.validate()?;
		if let Some(ref val) = self.sub_pdct { val.validate()? }
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
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NoPrtfl", skip_serializing_if = "Option::is_none") )]
	pub no_prtfl: Option<NotApplicable1Code>,
}

impl PortfolioCode3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 52 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 52".to_string()));
			}
		}
		if let Some(ref val) = self.no_prtfl { val.validate()? }
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
		if let Some(ref val) = self.prtfl { val.validate()? }
		if let Some(ref val) = self.no_prtfl { val.validate()? }
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
	pub cd: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtflTxXmptn", skip_serializing_if = "Option::is_none") )]
	pub prtfl_tx_xmptn: Option<bool>,
}

impl PortfolioIdentification3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.cd.chars().count() < 1 {
			return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
		}
		if self.cd.chars().count() > 52 {
			return Err(ValidationError::new(1002, "cd exceeds the maximum length of 52".to_string()));
		}
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
	pub strr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
}

impl PostTradeRiskReductionIdentifier1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.strr) {
			return Err(ValidationError::new(1005, "strr does not match the required pattern".to_string()));
		}
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 52 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 52".to_string()));
		}
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
		if let Some(ref val) = self.pric { val.validate()? }
		if let Some(ref vec) = self.schdl_prd { for item in vec { item.validate()? } }
		if let Some(ref val) = self.unit_of_measr { val.validate()? }
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
	pub desc: Option<String>,
}

impl Quantity47Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 52 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 52".to_string()));
			}
		}
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
		if let Some(ref vec) = self.schdl_prd { for item in vec { item.validate()? } }
		if let Some(ref val) = self.term { val.validate()? }
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
		if let Some(ref val) = self.unit_of_measr { val.validate()? }
		if let Some(ref val) = self.tm_unit { val.validate()? }
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
	pub rsn: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
}

impl ReportingExemption1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.rsn.chars().count() < 1 {
			return Err(ValidationError::new(1001, "rsn is shorter than the minimum length of 1".to_string()));
		}
		if self.rsn.chars().count() > 4 {
			return Err(ValidationError::new(1002, "rsn exceeds the maximum length of 4".to_string()));
		}
		if let Some(ref val) = self.desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 1000 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 1000".to_string()));
			}
		}
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
		self.pric.validate()?;
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
		if let Some(ref val) = self.unit_of_measr { val.validate()? }
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
		self.amt.validate()?;
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
		self.pric.validate()?;
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
		if let Some(ref val) = self.mntry_val { val.validate()? }
		if let Some(ref val) = self.pdg_pric { val.validate()? }
		if let Some(ref val) = self.othr { val.validate()? }
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
		if let Some(ref val) = self.mntry_val { val.validate()? }
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
		if let Some(ref val) = self.mntry_val { val.validate()? }
		if let Some(ref val) = self.othr { val.validate()? }
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


// SecurityIdentification41Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecurityIdentification41Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
	pub isin: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AltrntvInstrmId", skip_serializing_if = "Option::is_none") )]
	pub altrntv_instrm_id: Option<String>,
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
		if let Some(ref val) = self.isin {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.altrntv_instrm_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "altrntv_instrm_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 52 {
				return Err(ValidationError::new(1002, "altrntv_instrm_id exceeds the maximum length of 52".to_string()));
			}
		}
		if let Some(ref val) = self.unq_pdct_idr { val.validate()? }
		if let Some(ref val) = self.bskt { val.validate()? }
		if let Some(ref val) = self.indx { val.validate()? }
		if let Some(ref val) = self.othr { val.validate()? }
		if let Some(ref val) = self.id_not_avlbl { val.validate()? }
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
	pub isin: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none") )]
	pub unq_pdct_idr: Option<UniqueProductIdentifier2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AltrntvInstrmId", skip_serializing_if = "Option::is_none") )]
	pub altrntv_instrm_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PdctDesc", skip_serializing_if = "Option::is_none") )]
	pub pdct_desc: Option<String>,
}

impl SecurityIdentification46 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.isin {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.unq_pdct_idr { val.validate()? }
		if let Some(ref val) = self.altrntv_instrm_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "altrntv_instrm_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 105 {
				return Err(ValidationError::new(1002, "altrntv_instrm_id exceeds the maximum length of 105".to_string()));
			}
		}
		if let Some(ref val) = self.pdct_desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pdct_desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 1000 {
				return Err(ValidationError::new(1002, "pdct_desc exceeds the maximum length of 1000".to_string()));
			}
		}
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
		self.envlp.validate()?;
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
	pub tech_rcrd_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RcncltnFlg", skip_serializing_if = "Option::is_none") )]
	pub rcncltn_flg: Option<Reconciliation3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptRctTmStmp", skip_serializing_if = "Option::is_none") )]
	pub rpt_rct_tm_stmp: Option<String>,
}

impl TechnicalAttributes5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tech_rcrd_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tech_rcrd_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "tech_rcrd_id exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.rcncltn_flg { val.validate()? }
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
		if let Some(ref val) = self.clr_oblgtn { val.validate()? }
		if let Some(ref val) = self.clr_sts { val.validate()? }
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
		if let Some(ref val) = self.confd { val.validate()? }
		if let Some(ref val) = self.non_confd { val.validate()? }
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
		self.tp.validate()?;
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
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl TradeCounterpartyRelationship1Choice {
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
			if val.chars().count() > 100 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 100".to_string()));
			}
		}
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
	pub desc: Option<String>,
}

impl TradeCounterpartyRelationshipRecord1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.start_rltsh_pty.validate()?;
		self.end_rltsh_pty.validate()?;
		self.rltsh_tp.validate()?;
		if let Some(ref val) = self.desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 1000 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 1000".to_string()));
			}
		}
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
		self.rptg_ctr_pty.validate()?;
		self.othr_ctr_pty.validate()?;
		if let Some(ref val) = self.brkr { val.validate()? }
		if let Some(ref val) = self.submitg_agt { val.validate()? }
		if let Some(ref val) = self.clr_mmb { val.validate()? }
		if let Some(ref vec) = self.bnfcry { for item in vec { item.validate()? } }
		if let Some(ref val) = self.ntty_rspnsbl_for_rpt { val.validate()? }
		if let Some(ref vec) = self.exctn_agt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.rltsh_rcrd { for item in vec { item.validate()? } }
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


// TradeData60Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradeData60Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none") )]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Stat", skip_serializing_if = "Option::is_none") )]
	pub stat: Option<Vec<TradeStateReport23>>,
}

impl TradeData60Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.data_set_actn { val.validate()? }
		if let Some(ref vec) = self.stat { for item in vec { item.validate()? } }
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
		self.tp.validate()?;
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
	pub cmptnt_authrty: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NewTradRpstryIdr", skip_serializing_if = "Option::is_none") )]
	pub new_trad_rpstry_idr: Option<OrganisationIdentification15Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgPurp", skip_serializing_if = "Option::is_none") )]
	pub rptg_purp: Option<Vec<String>>,
}

impl TradeReportHeader4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.msg_pgntn { val.validate()? }
		if let Some(ref vec) = self.cmptnt_authrty {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "cmptnt_authrty is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 100 {
					return Err(ValidationError::new(1002, "cmptnt_authrty exceeds the maximum length of 100".to_string()));
				}
			}
		}
		if let Some(ref val) = self.new_trad_rpstry_idr { val.validate()? }
		if let Some(ref vec) = self.rptg_purp {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "rptg_purp is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 100 {
					return Err(ValidationError::new(1002, "rptg_purp exceeds the maximum length of 100".to_string()));
				}
			}
		}
		Ok(())
	}
}


// TradeStateReport23 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TradeStateReport23 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrPtySpcfcData") )]
	pub ctr_pty_spcfc_data: Vec<CounterpartySpecificData36>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CmonTradData") )]
	pub cmon_trad_data: CommonTradeDataReport72,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechAttrbts", skip_serializing_if = "Option::is_none") )]
	pub tech_attrbts: Option<TechnicalAttributes5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PblcDssmntnData", skip_serializing_if = "Option::is_none") )]
	pub pblc_dssmntn_data: Option<DisseminationData1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl TradeStateReport23 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.ctr_pty_spcfc_data { item.validate()? }
		self.cmon_trad_data.validate()?;
		if let Some(ref val) = self.tech_attrbts { val.validate()? }
		if let Some(ref val) = self.pblc_dssmntn_data { val.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
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
	pub scndry_tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrrTxId", skip_serializing_if = "Option::is_none") )]
	pub prr_tx_id: Option<UniqueTransactionIdentifier3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SbsqntTxId", skip_serializing_if = "Option::is_none") )]
	pub sbsqnt_tx_id: Option<UniqueTransactionIdentifier3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CollPrtflCd", skip_serializing_if = "Option::is_none") )]
	pub coll_prtfl_cd: Option<CollateralPortfolioCode6Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptTrckgNb", skip_serializing_if = "Option::is_none") )]
	pub rpt_trckg_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PltfmIdr", skip_serializing_if = "Option::is_none") )]
	pub pltfm_idr: Option<String>,
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
		if let Some(ref val) = self.tx_id { val.validate()? }
		if let Some(ref val) = self.scndry_tx_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "scndry_tx_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 72 {
				return Err(ValidationError::new(1002, "scndry_tx_id exceeds the maximum length of 72".to_string()));
			}
		}
		if let Some(ref val) = self.prr_tx_id { val.validate()? }
		if let Some(ref val) = self.sbsqnt_tx_id { val.validate()? }
		if let Some(ref val) = self.coll_prtfl_cd { val.validate()? }
		if let Some(ref val) = self.rpt_trckg_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rpt_trckg_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 52 {
				return Err(ValidationError::new(1002, "rpt_trckg_nb exceeds the maximum length of 52".to_string()));
			}
		}
		if let Some(ref val) = self.pltfm_idr {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "pltfm_idr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.tx_pric { val.validate()? }
		if let Some(ref val) = self.ntnl_amt { val.validate()? }
		if let Some(ref val) = self.ntnl_qty { val.validate()? }
		if let Some(ref val) = self.qty { val.validate()? }
		if let Some(ref val) = self.dlvry_tp { val.validate()? }
		if let Some(ref val) = self.mstr_agrmt { val.validate()? }
		if let Some(ref val) = self.pst_trad_rsk_rdctn_evt { val.validate()? }
		if let Some(ref val) = self.deriv_evt { val.validate()? }
		if let Some(ref val) = self.trad_conf { val.validate()? }
		if let Some(ref val) = self.trad_clr { val.validate()? }
		if let Some(ref val) = self.intrst_rate { val.validate()? }
		if let Some(ref val) = self.ccy { val.validate()? }
		if let Some(ref val) = self.cmmdty { val.validate()? }
		if let Some(ref val) = self.optn { val.validate()? }
		if let Some(ref val) = self.nrgy_spcfc_attrbts { val.validate()? }
		if let Some(ref val) = self.cdt { val.validate()? }
		if let Some(ref vec) = self.othr_pmt { for item in vec { item.validate()? } }
		if let Some(ref val) = self.packg { val.validate()? }
		if let Some(ref val) = self.trad_allcn_sts { val.validate()? }
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
		if let Some(ref val) = self.trnchd { val.validate()? }
		if let Some(ref val) = self.utrnchd { val.validate()? }
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
	pub id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification175>,
}

impl UniqueProductIdentifier1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 52 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 52".to_string()));
			}
		}
		if let Some(ref val) = self.prtry { val.validate()? }
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
	pub id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification185>,
}

impl UniqueProductIdentifier2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 52 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 52".to_string()));
			}
		}
		if let Some(ref val) = self.prtry { val.validate()? }
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
	pub unq_tx_idr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification179>,
}

impl UniqueTransactionIdentifier1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.unq_tx_idr {
			let pattern = Regex::new("[A-Z0-9]{18}[0-9]{2}[A-Z0-9]{0,32}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "unq_tx_idr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prtry { val.validate()? }
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
	pub unq_tx_idr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification175>,
}

impl UniqueTransactionIdentifier2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.unq_tx_idr {
			let pattern = Regex::new("[A-Z0-9]{18}[0-9]{2}[A-Z0-9]{0,32}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "unq_tx_idr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prtry { val.validate()? }
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
	pub unq_tx_idr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification175>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NotAvlbl", skip_serializing_if = "Option::is_none") )]
	pub not_avlbl: Option<NoReasonCode>,
}

impl UniqueTransactionIdentifier3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.unq_tx_idr {
			let pattern = Regex::new("[A-Z0-9]{18}[0-9]{2}[A-Z0-9]{0,32}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "unq_tx_idr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prtry { val.validate()? }
		if let Some(ref val) = self.not_avlbl { val.validate()? }
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
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification175>,
}

impl UnitOfMeasure8Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry { val.validate()? }
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
