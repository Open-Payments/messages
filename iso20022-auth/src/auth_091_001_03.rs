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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
	pub val1: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
	pub val2: Option<String>,
}

impl CompareActiveOrHistoricCurrencyCode1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.val1 {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "val1 does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.val2 {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "val2 does not match the required pattern".to_string()));
			}
		}
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
	pub val1: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
	pub val2: Option<String>,
}

impl CompareBenchmarkCode1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.val1 {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "val1 is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "val1 exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.val2 {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "val2 is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "val2 exceeds the maximum length of 4".to_string()));
			}
		}
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
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "val1 does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.val2 {
			let pattern = Regex::new("[A-Z]{6,6}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "val2 does not match the required pattern".to_string()));
			}
		}
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref vec) = self.nrgy_dlvry_intrvl { for item in vec { item.validate()? } }
		if let Some(ref val) = self.nrgy_dt { val.validate()? }
		if let Some(ref val) = self.nrgy_drtn { val.validate()? }
		if let Some(ref vec) = self.nrgy_wk_day { for item in vec { item.validate()? } }
		if let Some(ref val) = self.nrgy_dlvry_cpcty { val.validate()? }
		if let Some(ref val) = self.nrgy_qty_unit { val.validate()? }
		if let Some(ref val) = self.nrgy_pric_tm_intrvl_qty { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
	pub val1: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
	pub val2: Option<String>,
}

impl CompareISINIdentifier2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.val1 {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "val1 does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.val2 {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "val2 does not match the required pattern".to_string()));
			}
		}
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
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "val1 does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.val2 {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "val2 does not match the required pattern".to_string()));
			}
		}
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
	pub val1: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
	pub val2: Option<String>,
}

impl CompareMICIdentifier3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.val1 {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "val1 does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.val2 {
			let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "val2 does not match the required pattern".to_string()));
			}
		}
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
	pub val1: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
	pub val2: Option<String>,
}

impl CompareMax350Text1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.val1 {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "val1 is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "val1 exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.val2 {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "val2 is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "val2 exceeds the maximum length of 350".to_string()));
			}
		}
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
	pub val1: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
	pub val2: Option<String>,
}

impl CompareMax50Text1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.val1 {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "val1 is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 50 {
				return Err(ValidationError::new(1002, "val1 exceeds the maximum length of 50".to_string()));
			}
		}
		if let Some(ref val) = self.val2 {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "val2 is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 50 {
				return Err(ValidationError::new(1002, "val2 exceeds the maximum length of 50".to_string()));
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.othr_pmt_tp { val.validate()? }
		if let Some(ref val) = self.othr_pmt_amt { val.validate()? }
		if let Some(ref val) = self.othr_pmt_dt { val.validate()? }
		if let Some(ref val) = self.othr_pmt_pyer { val.validate()? }
		if let Some(ref val) = self.othr_pmt_rcvr { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
	pub val1: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val2", skip_serializing_if = "Option::is_none") )]
	pub val2: Option<String>,
}

impl CompareText1 {
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.val1 { val.validate()? }
		if let Some(ref val) = self.val2 { val.validate()? }
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
		if let Some(ref val) = self.isin { val.validate()? }
		if let Some(ref val) = self.unq_pdct_idr { val.validate()? }
		if let Some(ref val) = self.altrntv_instrm_id { val.validate()? }
		if let Some(ref val) = self.pdct_clssfctn { val.validate()? }
		if let Some(ref val) = self.ctrct_tp { val.validate()? }
		if let Some(ref val) = self.asst_clss { val.validate()? }
		if let Some(ref val) = self.deriv_based_on_crpt_asst { val.validate()? }
		if let Some(ref val) = self.undrlyg_instrm { val.validate()? }
		if let Some(ref val) = self.sttlm_ccy { val.validate()? }
		if let Some(ref val) = self.sttlm_ccy_scnd_leg { val.validate()? }
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
		if let Some(ref val) = self.rptg_ctr_pty { val.validate()? }
		if let Some(ref val) = self.othr_ctr_pty { val.validate()? }
		if let Some(ref val) = self.rpt_submitg_ntty { val.validate()? }
		if let Some(ref val) = self.ntty_rspnsbl_for_rpt { val.validate()? }
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
		if let Some(ref val) = self.rptg_ctr_pty { val.validate()? }
		if let Some(ref val) = self.othr_ctr_pty { val.validate()? }
		if let Some(ref val) = self.drctn_or_sd { val.validate()? }
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
		self.rcncltn_sttstcs.validate()?;
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
		if let Some(ref val) = self.ctr_pty_mtchg_crit { val.validate()? }
		if let Some(ref val) = self.valtn_mtchg_crit { val.validate()? }
		if let Some(ref val) = self.ctrct_mtchg_crit { val.validate()? }
		if let Some(ref val) = self.tx_mtchg_crit { val.validate()? }
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
		if let Some(ref val) = self.tchnq { val.validate()? }
		if let Some(ref val) = self.svc_prvdr { val.validate()? }
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
		self.rptg_tp.validate()?;
		self.pairg.validate()?;
		self.rcncltn.validate()?;
		self.valtn_rcncltn.validate()?;
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
		self.ctr_pty_id.validate()?;
		for item in &self.rcncltn_rpt { item.validate()? }
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
		self.tx_id.validate()?;
		self.mtchg_crit.validate()?;
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
		self.rcncltn_ctgrs.validate()?;
		if let Some(ref vec) = self.tx_dtls { for item in vec { item.validate()? } }
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
		if let Some(ref val) = self.rptg_rqrmnt { val.validate()? }
		if let Some(ref val) = self.no_rptg_rqrmnt { val.validate()? }
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
		if let Some(ref val) = self.mntry_val { val.validate()? }
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
		if let Some(ref val) = self.data_set_actn { val.validate()? }
		if let Some(ref vec) = self.rpt { for item in vec { item.validate()? } }
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
		if let Some(ref val) = self.confd { val.validate()? }
		if let Some(ref val) = self.non_confd { val.validate()? }
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
		if let Some(ref val) = self.tp { val.validate()? }
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
		self.tp.validate()?;
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
	pub tech_rcrd_id: Option<String>,
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
		if let Some(ref val) = self.tech_rcrd_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tech_rcrd_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "tech_rcrd_id exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.actn_tp { val.validate()? }
		if let Some(ref val) = self.deriv_evt_tp { val.validate()? }
		if let Some(ref val) = self.deriv_evt_tm_stmp { val.validate()? }
		if let Some(ref val) = self.othr_ctr_pty { val.validate()? }
		if let Some(ref val) = self.unq_idr { val.validate()? }
		if let Some(ref val) = self.mstr_agrmt { val.validate()? }
		if let Some(ref val) = self.coll_prtfl_cd { val.validate()? }
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
		if let Some(ref val) = self.rpt_trckg_nb { val.validate()? }
		if let Some(ref val) = self.unq_tx_idr { val.validate()? }
		if let Some(ref val) = self.prr_unq_tx_idr { val.validate()? }
		if let Some(ref val) = self.sbsqnt_pos_unq_tx_idr { val.validate()? }
		if let Some(ref val) = self.dlta { val.validate()? }
		if let Some(ref val) = self.trad_conf { val.validate()? }
		if let Some(ref val) = self.trad_clr_oblgtn { val.validate()? }
		if let Some(ref val) = self.trad_clr_sts { val.validate()? }
		if let Some(ref val) = self.mstr_agrmt_tp { val.validate()? }
		if let Some(ref val) = self.mstr_agrmt_vrsn { val.validate()? }
		if let Some(ref val) = self.intra_grp { val.validate()? }
		if let Some(ref val) = self.pst_trad_rsk_rdctn { val.validate()? }
		if let Some(ref val) = self.deriv_evt { val.validate()? }
		if let Some(ref val) = self.pltfm_idr { val.validate()? }
		if let Some(ref val) = self.exctn_tm_stmp { val.validate()? }
		if let Some(ref val) = self.fctv_dt { val.validate()? }
		if let Some(ref val) = self.xprtn_dt { val.validate()? }
		if let Some(ref val) = self.early_termntn_dt { val.validate()? }
		if let Some(ref vec) = self.sttlm_dt { for item in vec { item.validate()? } }
		if let Some(ref val) = self.dlvry_tp { val.validate()? }
		if let Some(ref val) = self.tx_pric { val.validate()? }
		if let Some(ref vec) = self.pric_schdl_uadjstd_fctv_dt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.pric_schdl_uadjstd_end_dt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.tx_schdl_pric { for item in vec { item.validate()? } }
		if let Some(ref val) = self.packg_pric { val.validate()? }
		if let Some(ref val) = self.ntnl_amt_frst_leg { val.validate()? }
		if let Some(ref vec) = self.ntnl_amt_frst_leg_uadjstd_fctv_dt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.ntnl_amt_frst_leg_uadjstd_end_dt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.ntnl_amt_frst_leg_schdl_amt { for item in vec { item.validate()? } }
		if let Some(ref val) = self.ntnl_qty_frst_leg { val.validate()? }
		if let Some(ref vec) = self.ntnl_qty_frst_leg_uadjstd_fctv_dt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.ntnl_qty_frst_leg_uadjstd_end_dt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.ntnl_qty_frst_leg_schdl_qty { for item in vec { item.validate()? } }
		if let Some(ref val) = self.ntnl_amt_scnd_leg { val.validate()? }
		if let Some(ref vec) = self.ntnl_amt_scnd_leg_uadjstd_fctv_dt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.ntnl_amt_scnd_leg_uadjstd_end_dt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.ntnl_amt_scnd_leg_schdl_amt { for item in vec { item.validate()? } }
		if let Some(ref val) = self.ntnl_qty_scnd_leg { val.validate()? }
		if let Some(ref vec) = self.ntnl_qty_scnd_leg_uadjstd_fctv_dt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.ntnl_qty_scnd_leg_uadjstd_end_dt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.ntnl_qty_scnd_leg_schdl_qty { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.othr_pmt { for item in vec { item.validate()? } }
		if let Some(ref val) = self.intrst_fxd_rate_frst_leg { val.validate()? }
		if let Some(ref val) = self.intrst_fxd_rate_frst_leg_day_cnt { val.validate()? }
		if let Some(ref val) = self.intrst_fxd_rate_frst_leg_pmt_frqcy_unit { val.validate()? }
		if let Some(ref val) = self.intrst_fxd_rate_frst_leg_pmt_frqcy_val { val.validate()? }
		if let Some(ref val) = self.intrst_fltg_rate_frst_leg_id { val.validate()? }
		if let Some(ref val) = self.intrst_fltg_rate_frst_leg_cd { val.validate()? }
		if let Some(ref val) = self.intrst_fltg_rate_frst_leg_nm { val.validate()? }
		if let Some(ref val) = self.intrst_fltg_rate_frst_leg_day_cnt { val.validate()? }
		if let Some(ref val) = self.intrst_fltg_rate_frst_leg_pmt_frqcy_unit { val.validate()? }
		if let Some(ref val) = self.intrst_fltg_rate_frst_leg_pmt_frqcy_val { val.validate()? }
		if let Some(ref val) = self.intrst_fltg_rate_frst_leg_ref_prd_unit { val.validate()? }
		if let Some(ref val) = self.intrst_fltg_rate_frst_leg_ref_prd_val { val.validate()? }
		if let Some(ref val) = self.intrst_fltg_rate_frst_leg_rst_frqcy_unit { val.validate()? }
		if let Some(ref val) = self.intrst_fltg_rate_frst_leg_rst_frqcy_val { val.validate()? }
		if let Some(ref val) = self.intrst_fltg_rate_frst_leg_sprd { val.validate()? }
		if let Some(ref val) = self.intrst_rate_fxd_scnd_leg { val.validate()? }
		if let Some(ref val) = self.intrst_fxd_rate_scnd_leg_day_cnt { val.validate()? }
		if let Some(ref val) = self.intrst_fxd_rate_scnd_leg_pmt_frqcy_unit { val.validate()? }
		if let Some(ref val) = self.intrst_fxd_rate_scnd_leg_pmt_frqcy_val { val.validate()? }
		if let Some(ref val) = self.intrst_fltg_rate_scnd_leg_id { val.validate()? }
		if let Some(ref val) = self.intrst_fltg_rate_scnd_leg_cd { val.validate()? }
		if let Some(ref val) = self.intrst_fltg_rate_scnd_leg_nm { val.validate()? }
		if let Some(ref val) = self.intrst_fltg_rate_scnd_leg_day_cnt { val.validate()? }
		if let Some(ref val) = self.intrst_fltg_rate_scnd_leg_pmt_frqcy_unit { val.validate()? }
		if let Some(ref val) = self.intrst_fltg_rate_scnd_leg_pmt_frqcy_val { val.validate()? }
		if let Some(ref val) = self.intrst_fltg_rate_scnd_leg_ref_prd_unit { val.validate()? }
		if let Some(ref val) = self.intrst_fltg_rate_scnd_leg_ref_prd_val { val.validate()? }
		if let Some(ref val) = self.intrst_fltg_rate_scnd_leg_rst_frqcy_unit { val.validate()? }
		if let Some(ref val) = self.intrst_fltg_rate_scnd_leg_rst_frqcy_val { val.validate()? }
		if let Some(ref val) = self.intrst_fltg_rate_scnd_leg_sprd { val.validate()? }
		if let Some(ref val) = self.packg_sprd { val.validate()? }
		if let Some(ref val) = self.ccy_xchg_rate { val.validate()? }
		if let Some(ref val) = self.ccy_fwd_xchg_rate { val.validate()? }
		if let Some(ref val) = self.ccy_xchg_rate_bsis { val.validate()? }
		if let Some(ref val) = self.cmmdty { val.validate()? }
		if let Some(ref vec) = self.nrgy_dlvry_pt_or_zone { for item in vec { item.validate()? } }
		if let Some(ref val) = self.nrgy_intr_cnnctn_pt { val.validate()? }
		if let Some(ref val) = self.nrgy_ld_tp { val.validate()? }
		if let Some(ref vec) = self.dlvry_attr { for item in vec { item.validate()? } }
		if let Some(ref val) = self.optn_tp { val.validate()? }
		if let Some(ref vec) = self.optn_exrc_style { for item in vec { item.validate()? } }
		if let Some(ref val) = self.optn_strk_pric { val.validate()? }
		if let Some(ref vec) = self.optn_strk_pric_schdl_uadjstd_fctv_dt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.optn_strk_pric_schdl_uadjstd_end_dt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.optn_strk_pric_schdl_amt { for item in vec { item.validate()? } }
		if let Some(ref val) = self.optn_prm_amt { val.validate()? }
		if let Some(ref val) = self.optn_prm_pmt_dt { val.validate()? }
		if let Some(ref val) = self.optn_mtrty_dt_of_undrlyg { val.validate()? }
		if let Some(ref val) = self.cdt_snrty { val.validate()? }
		if let Some(ref val) = self.cdt_ref_pty { val.validate()? }
		if let Some(ref val) = self.cdt_srs { val.validate()? }
		if let Some(ref val) = self.cdt_vrsn { val.validate()? }
		if let Some(ref val) = self.cdt_indx_fctr { val.validate()? }
		if let Some(ref val) = self.cdt_trch { val.validate()? }
		if let Some(ref val) = self.lvl { val.validate()? }
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
		if let Some(ref val) = self.ctrct_val { val.validate()? }
		if let Some(ref val) = self.tp { val.validate()? }
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
