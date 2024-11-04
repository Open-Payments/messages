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
		if let Some(ref val) = self.prtfl { val.validate()? }
		if let Some(ref val) = self.mrgn_prtfl_cd { val.validate()? }
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
	pub clctn_bsis: Option<String>,
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
		self.aggtd_pos.validate()?;
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
		self.coll_prtfl_cd.validate()?;
		self.collstn_ctgy.validate()?;
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
		self.initl_mrgn_prtfl_cd.validate()?;
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
		self.unit.validate()?;
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
		if let Some(ref val) = self.amt { val.validate()? }
		if let Some(ref vec) = self.amt_in_fct { for item in vec { item.validate()? } }
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
		if let Some(ref val) = self.frst_leg { val.validate()? }
		if let Some(ref val) = self.scnd_leg { val.validate()? }
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


// OtherPayment6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OtherPayment6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtCcy", skip_serializing_if = "Option::is_none") )]
	pub pmt_ccy: Option<String>,
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
		if let Some(ref val) = self.pmt_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "pmt_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pmt_tp { val.validate()? }
		if let Some(ref val) = self.pmt_pyer { val.validate()? }
		if let Some(ref val) = self.pmt_rcvr { val.validate()? }
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
		self.dmnsns.validate()?;
		self.mtrcs.validate()?;
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
		self.dmnsns.validate()?;
		self.mtrcs.validate()?;
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
		if let Some(ref val) = self.data_set_actn { val.validate()? }
		if let Some(ref val) = self.rpt { val.validate()? }
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
		if let Some(ref vec) = self.pos_set { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.ccy_pos_set { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.coll_pos_set { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.ccy_coll_pos_set { for item in vec { item.validate()? } }
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
		if let Some(ref val) = self.buyr { val.validate()? }
		if let Some(ref val) = self.sellr { val.validate()? }
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
	pub initl_mrgn_pstd_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VartnMrgnPstdCcy", skip_serializing_if = "Option::is_none") )]
	pub vartn_mrgn_pstd_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitlMrgnRcvdCcy", skip_serializing_if = "Option::is_none") )]
	pub initl_mrgn_rcvd_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VartnMrgnRcvdCcy", skip_serializing_if = "Option::is_none") )]
	pub vartn_mrgn_rcvd_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XcssCollPstdCcy", skip_serializing_if = "Option::is_none") )]
	pub xcss_coll_pstd_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XcssCollRcvdCcy", skip_serializing_if = "Option::is_none") )]
	pub xcss_coll_rcvd_ccy: Option<String>,
}

impl PositionSetCollateralDimensions3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ctr_pty_id { val.validate()? }
		if let Some(ref val) = self.coll { val.validate()? }
		if let Some(ref val) = self.initl_mrgn_pstd_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "initl_mrgn_pstd_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.vartn_mrgn_pstd_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "vartn_mrgn_pstd_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.initl_mrgn_rcvd_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "initl_mrgn_rcvd_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.vartn_mrgn_rcvd_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "vartn_mrgn_rcvd_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.xcss_coll_pstd_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "xcss_coll_pstd_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.xcss_coll_rcvd_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "xcss_coll_rcvd_ccy does not match the required pattern".to_string()));
			}
		}
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
		if let Some(ref val) = self.ttl { val.validate()? }
		if let Some(ref val) = self.clean { val.validate()? }
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
		if let Some(ref val) = self.pstd_mrgn_or_coll { val.validate()? }
		if let Some(ref val) = self.rcvd_mrgn_or_coll { val.validate()? }
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
	pub val_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Coll", skip_serializing_if = "Option::is_none") )]
	pub coll: Option<MarginCollateralReport4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctTp", skip_serializing_if = "Option::is_none") )]
	pub ctrct_tp: Option<FinancialInstrumentContractType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AsstClss", skip_serializing_if = "Option::is_none") )]
	pub asst_clss: Option<ProductType4Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygInstrm", skip_serializing_if = "Option::is_none") )]
	pub undrlyg_instrm: Option<SecurityIdentification41Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlCcy", skip_serializing_if = "Option::is_none") )]
	pub ntnl_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NtnlCcyScndLeg", skip_serializing_if = "Option::is_none") )]
	pub ntnl_ccy_scnd_leg: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none") )]
	pub sttlm_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmCcyScndLeg", skip_serializing_if = "Option::is_none") )]
	pub sttlm_ccy_scnd_leg: Option<String>,
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
	pub irs_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdt", skip_serializing_if = "Option::is_none") )]
	pub cdt: Option<CreditDerivative7>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none") )]
	pub cmmdty: Option<AssetClassCommodity6Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrPmt", skip_serializing_if = "Option::is_none") )]
	pub othr_pmt: Option<OtherPayment6>,
}

impl PositionSetDimensions16 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ctr_pty_id { val.validate()? }
		if let Some(ref val) = self.val_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "val_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.coll { val.validate()? }
		if let Some(ref val) = self.ctrct_tp { val.validate()? }
		if let Some(ref val) = self.asst_clss { val.validate()? }
		if let Some(ref val) = self.undrlyg_instrm { val.validate()? }
		if let Some(ref val) = self.ntnl_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ntnl_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ntnl_ccy_scnd_leg {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ntnl_ccy_scnd_leg does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.sttlm_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "sttlm_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.sttlm_ccy_scnd_leg {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "sttlm_ccy_scnd_leg does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.mstr_agrmt { val.validate()? }
		if let Some(ref val) = self.xchg_rate_bsis { val.validate()? }
		if let Some(ref val) = self.optn_tp { val.validate()? }
		if let Some(ref val) = self.tm_to_mtrty { val.validate()? }
		if let Some(ref val) = self.irs_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "irs_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 52 {
				return Err(ValidationError::new(1002, "irs_tp exceeds the maximum length of 52".to_string()));
			}
		}
		if let Some(ref val) = self.cdt { val.validate()? }
		if let Some(ref val) = self.cmmdty { val.validate()? }
		if let Some(ref val) = self.othr_pmt { val.validate()? }
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
		if let Some(ref val) = self.ttl { val.validate()? }
		if let Some(ref val) = self.clean { val.validate()? }
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
		if let Some(ref val) = self.postv_val { val.validate()? }
		if let Some(ref val) = self.neg_val { val.validate()? }
		if let Some(ref val) = self.ntnl { val.validate()? }
		if let Some(ref vec) = self.othr_pmt_amt { for item in vec { item.validate()? } }
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
		if let Some(ref val) = self.initl_mrgn_pstd_pre_hrcut { val.validate()? }
		if let Some(ref val) = self.initl_mrgn_pstd_pst_hrcut { val.validate()? }
		if let Some(ref val) = self.vartn_mrgn_pstd_pre_hrcut { val.validate()? }
		if let Some(ref val) = self.vartn_mrgn_pstd_pst_hrcut { val.validate()? }
		if let Some(ref val) = self.xcss_coll_pstd { val.validate()? }
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
		if let Some(ref val) = self.initl_mrgn_rcvd_pre_hrcut { val.validate()? }
		if let Some(ref val) = self.initl_mrgn_rcvd_pst_hrcut { val.validate()? }
		if let Some(ref val) = self.vartn_mrgn_rcvd_pre_hrcut { val.validate()? }
		if let Some(ref val) = self.vartn_mrgn_rcvd_pst_hrcut { val.validate()? }
		if let Some(ref val) = self.xcss_coll_rcvd { val.validate()? }
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
		if let Some(ref val) = self.prd { val.validate()? }
		if let Some(ref val) = self.spcl { val.validate()? }
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
		if let Some(ref val) = self.start { val.validate()? }
		if let Some(ref val) = self.end { val.validate()? }
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
