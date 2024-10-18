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

use serde::{Deserialize, Serialize};
use regex::Regex;


// ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_and20_decimal_amount_simple_type: f64,
}

impl ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType {
	pub fn validate(&self) -> bool {
		if self.active_or_historic_currency_and20_decimal_amount_simple_type < 0.000000 {
			return false
		}
		return true
	}
}


// ActiveOrHistoricCurrencyAnd20DecimalAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd20DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveOrHistoricCurrencyAnd20DecimalAmount {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ActiveOrHistoricCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_and_amount_simple_type: f64,
}

impl ActiveOrHistoricCurrencyAndAmountSimpleType {
	pub fn validate(&self) -> bool {
		if self.active_or_historic_currency_and_amount_simple_type < 0.000000 {
			return false
		}
		return true
	}
}


// ActiveOrHistoricCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveOrHistoricCurrencyAndAmount {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_code: String,
}

impl ActiveOrHistoricCurrencyCode {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.active_or_historic_currency_code) {
			return false
		}
		return true
	}
}


// AgreementType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgreementType1Choice {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<ExternalAgreementType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl AgreementType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// AgreementType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgreementType2Choice {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<ExternalAgreementType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max50Text>,
}

impl AgreementType2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// AgriculturalCommodityDairy1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityDairy1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType20Code,
}

impl AgriculturalCommodityDairy1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		return true
	}
}


// AgriculturalCommodityForestry1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityForestry1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType21Code,
}

impl AgriculturalCommodityForestry1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		return true
	}
}


// AgriculturalCommodityGrain2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityGrain2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType5Code,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: AssetClassDetailedSubProductType30Code,
}

impl AgriculturalCommodityGrain2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		if !self.addtl_sub_pdct.validate() { return false }
		return true
	}
}


// AgriculturalCommodityLiveStock1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityLiveStock1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType22Code,
}

impl AgriculturalCommodityLiveStock1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		return true
	}
}


// AgriculturalCommodityOilSeed1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityOilSeed1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType1Code,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: AssetClassDetailedSubProductType1Code,
}

impl AgriculturalCommodityOilSeed1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		if !self.addtl_sub_pdct.validate() { return false }
		return true
	}
}


// AgriculturalCommodityOliveOil2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityOliveOil2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType3Code,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: AssetClassDetailedSubProductType29Code,
}

impl AgriculturalCommodityOliveOil2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		if !self.addtl_sub_pdct.validate() { return false }
		return true
	}
}


// AgriculturalCommodityOther1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType49Code,
}

impl AgriculturalCommodityOther1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		return true
	}
}


// AgriculturalCommodityPotato1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityPotato1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType45Code,
}

impl AgriculturalCommodityPotato1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		return true
	}
}


// AgriculturalCommoditySeafood1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommoditySeafood1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType23Code,
}

impl AgriculturalCommoditySeafood1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		return true
	}
}


// AgriculturalCommoditySoft1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommoditySoft1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType2Code,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: AssetClassDetailedSubProductType2Code,
}

impl AgriculturalCommoditySoft1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		if !self.addtl_sub_pdct.validate() { return false }
		return true
	}
}


// AmountAndDirection107 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection107 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAnd20DecimalAmount,
	#[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
	pub sgn: Option<bool>,
}

impl AmountAndDirection107 {
	pub fn validate(&self) -> bool {
		if !self.amt.validate() { return false }
		return true
	}
}


// AmountAndDirection53 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection53 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
	pub sgn: Option<bool>,
}

impl AmountAndDirection53 {
	pub fn validate(&self) -> bool {
		if !self.amt.validate() { return false }
		return true
	}
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
}

impl AnyBICDec2014Identifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
		if !pattern.is_match(&self.any_bic_dec2014_identifier) {
			return false
		}
		return true
	}
}


// AssetClassCommodity5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodity5Choice {
	#[serde(rename = "Agrcltrl", skip_serializing_if = "Option::is_none")]
	pub agrcltrl: Option<AssetClassCommodityAgricultural5Choice>,
	#[serde(rename = "Nrgy", skip_serializing_if = "Option::is_none")]
	pub nrgy: Option<AssetClassCommodityEnergy2Choice>,
	#[serde(rename = "Envttl", skip_serializing_if = "Option::is_none")]
	pub envttl: Option<AssetClassCommodityEnvironmental2Choice>,
	#[serde(rename = "Frtlzr", skip_serializing_if = "Option::is_none")]
	pub frtlzr: Option<AssetClassCommodityFertilizer3Choice>,
	#[serde(rename = "Frght", skip_serializing_if = "Option::is_none")]
	pub frght: Option<AssetClassCommodityFreight3Choice>,
	#[serde(rename = "IndstrlPdct", skip_serializing_if = "Option::is_none")]
	pub indstrl_pdct: Option<AssetClassCommodityIndustrialProduct1Choice>,
	#[serde(rename = "Metl", skip_serializing_if = "Option::is_none")]
	pub metl: Option<AssetClassCommodityMetal1Choice>,
	#[serde(rename = "OthrC10", skip_serializing_if = "Option::is_none")]
	pub othr_c10: Option<AssetClassCommodityOtherC102Choice>,
	#[serde(rename = "Ppr", skip_serializing_if = "Option::is_none")]
	pub ppr: Option<AssetClassCommodityPaper3Choice>,
	#[serde(rename = "Plprpln", skip_serializing_if = "Option::is_none")]
	pub plprpln: Option<AssetClassCommodityPolypropylene3Choice>,
	#[serde(rename = "Infltn", skip_serializing_if = "Option::is_none")]
	pub infltn: Option<AssetClassCommodityInflation1>,
	#[serde(rename = "MultiCmmdtyExtc", skip_serializing_if = "Option::is_none")]
	pub multi_cmmdty_extc: Option<AssetClassCommodityMultiCommodityExotic1>,
	#[serde(rename = "OffclEcnmcSttstcs", skip_serializing_if = "Option::is_none")]
	pub offcl_ecnmc_sttstcs: Option<AssetClassCommodityOfficialEconomicStatistics1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<AssetClassCommodityOther1>,
}

impl AssetClassCommodity5Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref agrcltrl_value) = self.agrcltrl { if !agrcltrl_value.validate() { return false; } }
		if let Some(ref nrgy_value) = self.nrgy { if !nrgy_value.validate() { return false; } }
		if let Some(ref envttl_value) = self.envttl { if !envttl_value.validate() { return false; } }
		if let Some(ref frtlzr_value) = self.frtlzr { if !frtlzr_value.validate() { return false; } }
		if let Some(ref frght_value) = self.frght { if !frght_value.validate() { return false; } }
		if let Some(ref indstrl_pdct_value) = self.indstrl_pdct { if !indstrl_pdct_value.validate() { return false; } }
		if let Some(ref metl_value) = self.metl { if !metl_value.validate() { return false; } }
		if let Some(ref othr_c10_value) = self.othr_c10 { if !othr_c10_value.validate() { return false; } }
		if let Some(ref ppr_value) = self.ppr { if !ppr_value.validate() { return false; } }
		if let Some(ref plprpln_value) = self.plprpln { if !plprpln_value.validate() { return false; } }
		if let Some(ref infltn_value) = self.infltn { if !infltn_value.validate() { return false; } }
		if let Some(ref multi_cmmdty_extc_value) = self.multi_cmmdty_extc { if !multi_cmmdty_extc_value.validate() { return false; } }
		if let Some(ref offcl_ecnmc_sttstcs_value) = self.offcl_ecnmc_sttstcs { if !offcl_ecnmc_sttstcs_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
	}
}


// AssetClassCommodityAgricultural5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityAgricultural5Choice {
	#[serde(rename = "GrnOilSeed", skip_serializing_if = "Option::is_none")]
	pub grn_oil_seed: Option<AgriculturalCommodityOilSeed1>,
	#[serde(rename = "Soft", skip_serializing_if = "Option::is_none")]
	pub soft: Option<AgriculturalCommoditySoft1>,
	#[serde(rename = "Ptt", skip_serializing_if = "Option::is_none")]
	pub ptt: Option<AgriculturalCommodityPotato1>,
	#[serde(rename = "OlvOil", skip_serializing_if = "Option::is_none")]
	pub olv_oil: Option<AgriculturalCommodityOliveOil2>,
	#[serde(rename = "Dairy", skip_serializing_if = "Option::is_none")]
	pub dairy: Option<AgriculturalCommodityDairy1>,
	#[serde(rename = "Frstry", skip_serializing_if = "Option::is_none")]
	pub frstry: Option<AgriculturalCommodityForestry1>,
	#[serde(rename = "Sfd", skip_serializing_if = "Option::is_none")]
	pub sfd: Option<AgriculturalCommoditySeafood1>,
	#[serde(rename = "LiveStock", skip_serializing_if = "Option::is_none")]
	pub live_stock: Option<AgriculturalCommodityLiveStock1>,
	#[serde(rename = "Grn", skip_serializing_if = "Option::is_none")]
	pub grn: Option<AgriculturalCommodityGrain2>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<AgriculturalCommodityOther1>,
}

impl AssetClassCommodityAgricultural5Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref grn_oil_seed_value) = self.grn_oil_seed { if !grn_oil_seed_value.validate() { return false; } }
		if let Some(ref soft_value) = self.soft { if !soft_value.validate() { return false; } }
		if let Some(ref ptt_value) = self.ptt { if !ptt_value.validate() { return false; } }
		if let Some(ref olv_oil_value) = self.olv_oil { if !olv_oil_value.validate() { return false; } }
		if let Some(ref dairy_value) = self.dairy { if !dairy_value.validate() { return false; } }
		if let Some(ref frstry_value) = self.frstry { if !frstry_value.validate() { return false; } }
		if let Some(ref sfd_value) = self.sfd { if !sfd_value.validate() { return false; } }
		if let Some(ref live_stock_value) = self.live_stock { if !live_stock_value.validate() { return false; } }
		if let Some(ref grn_value) = self.grn { if !grn_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
	}
}


// AssetClassCommodityEnergy2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityEnergy2Choice {
	#[serde(rename = "Elctrcty", skip_serializing_if = "Option::is_none")]
	pub elctrcty: Option<EnergyCommodityElectricity1>,
	#[serde(rename = "NtrlGas", skip_serializing_if = "Option::is_none")]
	pub ntrl_gas: Option<EnergyCommodityNaturalGas2>,
	#[serde(rename = "Oil", skip_serializing_if = "Option::is_none")]
	pub oil: Option<EnergyCommodityOil2>,
	#[serde(rename = "Coal", skip_serializing_if = "Option::is_none")]
	pub coal: Option<EnergyCommodityCoal1>,
	#[serde(rename = "IntrNrgy", skip_serializing_if = "Option::is_none")]
	pub intr_nrgy: Option<EnergyCommodityInterEnergy1>,
	#[serde(rename = "RnwblNrgy", skip_serializing_if = "Option::is_none")]
	pub rnwbl_nrgy: Option<EnergyCommodityRenewableEnergy1>,
	#[serde(rename = "LghtEnd", skip_serializing_if = "Option::is_none")]
	pub lght_end: Option<EnergyCommodityLightEnd1>,
	#[serde(rename = "Dstllts", skip_serializing_if = "Option::is_none")]
	pub dstllts: Option<EnergyCommodityDistillates1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<EnergyCommodityOther1>,
}

impl AssetClassCommodityEnergy2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref elctrcty_value) = self.elctrcty { if !elctrcty_value.validate() { return false; } }
		if let Some(ref ntrl_gas_value) = self.ntrl_gas { if !ntrl_gas_value.validate() { return false; } }
		if let Some(ref oil_value) = self.oil { if !oil_value.validate() { return false; } }
		if let Some(ref coal_value) = self.coal { if !coal_value.validate() { return false; } }
		if let Some(ref intr_nrgy_value) = self.intr_nrgy { if !intr_nrgy_value.validate() { return false; } }
		if let Some(ref rnwbl_nrgy_value) = self.rnwbl_nrgy { if !rnwbl_nrgy_value.validate() { return false; } }
		if let Some(ref lght_end_value) = self.lght_end { if !lght_end_value.validate() { return false; } }
		if let Some(ref dstllts_value) = self.dstllts { if !dstllts_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
	}
}


// AssetClassCommodityEnvironmental2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityEnvironmental2Choice {
	#[serde(rename = "Emssns", skip_serializing_if = "Option::is_none")]
	pub emssns: Option<EnvironmentalCommodityEmission2>,
	#[serde(rename = "Wthr", skip_serializing_if = "Option::is_none")]
	pub wthr: Option<EnvironmentalCommodityWeather1>,
	#[serde(rename = "CrbnRltd", skip_serializing_if = "Option::is_none")]
	pub crbn_rltd: Option<EnvironmentalCommodityCarbonRelated1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<EnvironmentCommodityOther1>,
}

impl AssetClassCommodityEnvironmental2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref emssns_value) = self.emssns { if !emssns_value.validate() { return false; } }
		if let Some(ref wthr_value) = self.wthr { if !wthr_value.validate() { return false; } }
		if let Some(ref crbn_rltd_value) = self.crbn_rltd { if !crbn_rltd_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
	}
}


// AssetClassCommodityFertilizer3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityFertilizer3Choice {
	#[serde(rename = "Ammn", skip_serializing_if = "Option::is_none")]
	pub ammn: Option<FertilizerCommodityAmmonia1>,
	#[serde(rename = "DmmnmPhspht", skip_serializing_if = "Option::is_none")]
	pub dmmnm_phspht: Option<FertilizerCommodityDiammoniumPhosphate1>,
	#[serde(rename = "Ptsh", skip_serializing_if = "Option::is_none")]
	pub ptsh: Option<FertilizerCommodityPotash1>,
	#[serde(rename = "Slphr", skip_serializing_if = "Option::is_none")]
	pub slphr: Option<FertilizerCommoditySulphur1>,
	#[serde(rename = "Urea", skip_serializing_if = "Option::is_none")]
	pub urea: Option<FertilizerCommodityUrea1>,
	#[serde(rename = "UreaAndAmmnmNtrt", skip_serializing_if = "Option::is_none")]
	pub urea_and_ammnm_ntrt: Option<FertilizerCommodityUreaAndAmmoniumNitrate1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<FertilizerCommodityOther1>,
}

impl AssetClassCommodityFertilizer3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref ammn_value) = self.ammn { if !ammn_value.validate() { return false; } }
		if let Some(ref dmmnm_phspht_value) = self.dmmnm_phspht { if !dmmnm_phspht_value.validate() { return false; } }
		if let Some(ref ptsh_value) = self.ptsh { if !ptsh_value.validate() { return false; } }
		if let Some(ref slphr_value) = self.slphr { if !slphr_value.validate() { return false; } }
		if let Some(ref urea_value) = self.urea { if !urea_value.validate() { return false; } }
		if let Some(ref urea_and_ammnm_ntrt_value) = self.urea_and_ammnm_ntrt { if !urea_and_ammnm_ntrt_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
	}
}


// AssetClassCommodityFreight3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityFreight3Choice {
	#[serde(rename = "Dry", skip_serializing_if = "Option::is_none")]
	pub dry: Option<FreightCommodityDry2>,
	#[serde(rename = "Wet", skip_serializing_if = "Option::is_none")]
	pub wet: Option<FreightCommodityWet2>,
	#[serde(rename = "CntnrShip", skip_serializing_if = "Option::is_none")]
	pub cntnr_ship: Option<FreightCommodityContainerShip1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<FreightCommodityOther1>,
}

impl AssetClassCommodityFreight3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref dry_value) = self.dry { if !dry_value.validate() { return false; } }
		if let Some(ref wet_value) = self.wet { if !wet_value.validate() { return false; } }
		if let Some(ref cntnr_ship_value) = self.cntnr_ship { if !cntnr_ship_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
	}
}


// AssetClassCommodityIndustrialProduct1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityIndustrialProduct1Choice {
	#[serde(rename = "Cnstrctn", skip_serializing_if = "Option::is_none")]
	pub cnstrctn: Option<IndustrialProductCommodityConstruction1>,
	#[serde(rename = "Manfctg", skip_serializing_if = "Option::is_none")]
	pub manfctg: Option<IndustrialProductCommodityManufacturing1>,
}

impl AssetClassCommodityIndustrialProduct1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cnstrctn_value) = self.cnstrctn { if !cnstrctn_value.validate() { return false; } }
		if let Some(ref manfctg_value) = self.manfctg { if !manfctg_value.validate() { return false; } }
		return true
	}
}


// AssetClassCommodityInflation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityInflation1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType12Code,
}

impl AssetClassCommodityInflation1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		return true
	}
}


// AssetClassCommodityMetal1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityMetal1Choice {
	#[serde(rename = "NonPrcs", skip_serializing_if = "Option::is_none")]
	pub non_prcs: Option<MetalCommodityNonPrecious1>,
	#[serde(rename = "Prcs", skip_serializing_if = "Option::is_none")]
	pub prcs: Option<MetalCommodityPrecious1>,
}

impl AssetClassCommodityMetal1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref non_prcs_value) = self.non_prcs { if !non_prcs_value.validate() { return false; } }
		if let Some(ref prcs_value) = self.prcs { if !prcs_value.validate() { return false; } }
		return true
	}
}


// AssetClassCommodityMultiCommodityExotic1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityMultiCommodityExotic1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType13Code,
}

impl AssetClassCommodityMultiCommodityExotic1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		return true
	}
}


// AssetClassCommodityOfficialEconomicStatistics1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityOfficialEconomicStatistics1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType14Code,
}

impl AssetClassCommodityOfficialEconomicStatistics1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		return true
	}
}


// AssetClassCommodityOther1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType15Code,
}

impl AssetClassCommodityOther1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		return true
	}
}


// AssetClassCommodityOtherC102Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityOtherC102Choice {
	#[serde(rename = "Dlvrbl", skip_serializing_if = "Option::is_none")]
	pub dlvrbl: Option<OtherC10CommodityDeliverable2>,
	#[serde(rename = "NonDlvrbl", skip_serializing_if = "Option::is_none")]
	pub non_dlvrbl: Option<OtherC10CommodityNonDeliverable2>,
}

impl AssetClassCommodityOtherC102Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref dlvrbl_value) = self.dlvrbl { if !dlvrbl_value.validate() { return false; } }
		if let Some(ref non_dlvrbl_value) = self.non_dlvrbl { if !non_dlvrbl_value.validate() { return false; } }
		return true
	}
}


// AssetClassCommodityPaper3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityPaper3Choice {
	#[serde(rename = "CntnrBrd", skip_serializing_if = "Option::is_none")]
	pub cntnr_brd: Option<PaperCommodityContainerBoard1>,
	#[serde(rename = "Nwsprnt", skip_serializing_if = "Option::is_none")]
	pub nwsprnt: Option<PaperCommodityNewsprint1>,
	#[serde(rename = "Pulp", skip_serializing_if = "Option::is_none")]
	pub pulp: Option<PaperCommodityPulp1>,
	#[serde(rename = "RcvrdPpr", skip_serializing_if = "Option::is_none")]
	pub rcvrd_ppr: Option<PaperCommodityRecoveredPaper1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<PaperCommodityRecoveredPaper2>,
}

impl AssetClassCommodityPaper3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cntnr_brd_value) = self.cntnr_brd { if !cntnr_brd_value.validate() { return false; } }
		if let Some(ref nwsprnt_value) = self.nwsprnt { if !nwsprnt_value.validate() { return false; } }
		if let Some(ref pulp_value) = self.pulp { if !pulp_value.validate() { return false; } }
		if let Some(ref rcvrd_ppr_value) = self.rcvrd_ppr { if !rcvrd_ppr_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
	}
}


// AssetClassCommodityPolypropylene3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityPolypropylene3Choice {
	#[serde(rename = "Plstc", skip_serializing_if = "Option::is_none")]
	pub plstc: Option<PolypropyleneCommodityPlastic1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<PolypropyleneCommodityOther1>,
}

impl AssetClassCommodityPolypropylene3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref plstc_value) = self.plstc { if !plstc_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
	}
}


// AssetClassDetailedSubProductType10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType10Code {
	#[default]
	#[serde(rename = "ALUM")]
	CodeALUM,
	#[serde(rename = "ALUA")]
	CodeALUA,
	#[serde(rename = "CBLT")]
	CodeCBLT,
	#[serde(rename = "COPR")]
	CodeCOPR,
	#[serde(rename = "IRON")]
	CodeIRON,
	#[serde(rename = "MOLY")]
	CodeMOLY,
	#[serde(rename = "NASC")]
	CodeNASC,
	#[serde(rename = "NICK")]
	CodeNICK,
	#[serde(rename = "STEL")]
	CodeSTEL,
	#[serde(rename = "TINN")]
	CodeTINN,
	#[serde(rename = "ZINC")]
	CodeZINC,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "LEAD")]
	CodeLEAD,
}

impl AssetClassDetailedSubProductType10Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassDetailedSubProductType11Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType11Code {
	#[default]
	#[serde(rename = "GOLD")]
	CodeGOLD,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "PLDM")]
	CodePLDM,
	#[serde(rename = "PTNM")]
	CodePTNM,
	#[serde(rename = "SLVR")]
	CodeSLVR,
}

impl AssetClassDetailedSubProductType11Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassDetailedSubProductType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType1Code {
	#[default]
	#[serde(rename = "FWHT")]
	CodeFWHT,
	#[serde(rename = "SOYB")]
	CodeSOYB,
	#[serde(rename = "RPSD")]
	CodeRPSD,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "CORN")]
	CodeCORN,
	#[serde(rename = "RICE")]
	CodeRICE,
}

impl AssetClassDetailedSubProductType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassDetailedSubProductType29Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType29Code {
	#[default]
	#[serde(rename = "LAMP")]
	CodeLAMP,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}

impl AssetClassDetailedSubProductType29Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassDetailedSubProductType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType2Code {
	#[default]
	#[serde(rename = "ROBU")]
	CodeROBU,
	#[serde(rename = "CCOA")]
	CodeCCOA,
	#[serde(rename = "BRWN")]
	CodeBRWN,
	#[serde(rename = "WHSG")]
	CodeWHSG,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}

impl AssetClassDetailedSubProductType2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassDetailedSubProductType30Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType30Code {
	#[default]
	#[serde(rename = "MWHT")]
	CodeMWHT,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}

impl AssetClassDetailedSubProductType30Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassDetailedSubProductType31Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType31Code {
	#[default]
	#[serde(rename = "GASP")]
	CodeGASP,
	#[serde(rename = "LNGG")]
	CodeLNGG,
	#[serde(rename = "NCGG")]
	CodeNCGG,
	#[serde(rename = "TTFG")]
	CodeTTFG,
	#[serde(rename = "NBPG")]
	CodeNBPG,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}

impl AssetClassDetailedSubProductType31Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassDetailedSubProductType32Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType32Code {
	#[default]
	#[serde(rename = "BAKK")]
	CodeBAKK,
	#[serde(rename = "BDSL")]
	CodeBDSL,
	#[serde(rename = "BRNT")]
	CodeBRNT,
	#[serde(rename = "BRNX")]
	CodeBRNX,
	#[serde(rename = "CNDA")]
	CodeCNDA,
	#[serde(rename = "COND")]
	CodeCOND,
	#[serde(rename = "DSEL")]
	CodeDSEL,
	#[serde(rename = "DUBA")]
	CodeDUBA,
	#[serde(rename = "ESPO")]
	CodeESPO,
	#[serde(rename = "ETHA")]
	CodeETHA,
	#[serde(rename = "FUEL")]
	CodeFUEL,
	#[serde(rename = "FOIL")]
	CodeFOIL,
	#[serde(rename = "GOIL")]
	CodeGOIL,
	#[serde(rename = "GSLN")]
	CodeGSLN,
	#[serde(rename = "HEAT")]
	CodeHEAT,
	#[serde(rename = "JTFL")]
	CodeJTFL,
	#[serde(rename = "KERO")]
	CodeKERO,
	#[serde(rename = "LLSO")]
	CodeLLSO,
	#[serde(rename = "MARS")]
	CodeMARS,
	#[serde(rename = "NAPH")]
	CodeNAPH,
	#[serde(rename = "NGLO")]
	CodeNGLO,
	#[serde(rename = "TAPI")]
	CodeTAPI,
	#[serde(rename = "WTIO")]
	CodeWTIO,
	#[serde(rename = "URAL")]
	CodeURAL,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}

impl AssetClassDetailedSubProductType32Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassDetailedSubProductType33Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType33Code {
	#[default]
	#[serde(rename = "DBCR")]
	CodeDBCR,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}

impl AssetClassDetailedSubProductType33Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassDetailedSubProductType34Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType34Code {
	#[default]
	#[serde(rename = "TNKR")]
	CodeTNKR,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}

impl AssetClassDetailedSubProductType34Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassDetailedSubProductType5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType5Code {
	#[default]
	#[serde(rename = "BSLD")]
	CodeBSLD,
	#[serde(rename = "FITR")]
	CodeFITR,
	#[serde(rename = "PKLD")]
	CodePKLD,
	#[serde(rename = "OFFP")]
	CodeOFFP,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}

impl AssetClassDetailedSubProductType5Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassDetailedSubProductType8Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType8Code {
	#[default]
	#[serde(rename = "CERE")]
	CodeCERE,
	#[serde(rename = "ERUE")]
	CodeERUE,
	#[serde(rename = "EUAE")]
	CodeEUAE,
	#[serde(rename = "EUAA")]
	CodeEUAA,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}

impl AssetClassDetailedSubProductType8Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassProductType11Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType11Code {
	#[default]
	#[serde(rename = "OTHC")]
	CodeOTHC,
}

impl AssetClassProductType11Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassProductType12Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType12Code {
	#[default]
	#[serde(rename = "INFL")]
	CodeINFL,
}

impl AssetClassProductType12Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassProductType13Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType13Code {
	#[default]
	#[serde(rename = "MCEX")]
	CodeMCEX,
}

impl AssetClassProductType13Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassProductType14Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType14Code {
	#[default]
	#[serde(rename = "OEST")]
	CodeOEST,
}

impl AssetClassProductType14Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassProductType15Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType15Code {
	#[default]
	#[serde(rename = "OTHR")]
	CodeOTHR,
}

impl AssetClassProductType15Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassProductType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType1Code {
	#[default]
	#[serde(rename = "AGRI")]
	CodeAGRI,
}

impl AssetClassProductType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassProductType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType2Code {
	#[default]
	#[serde(rename = "NRGY")]
	CodeNRGY,
}

impl AssetClassProductType2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassProductType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType3Code {
	#[default]
	#[serde(rename = "ENVR")]
	CodeENVR,
}

impl AssetClassProductType3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassProductType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType4Code {
	#[default]
	#[serde(rename = "FRGT")]
	CodeFRGT,
}

impl AssetClassProductType4Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassProductType5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType5Code {
	#[default]
	#[serde(rename = "FRTL")]
	CodeFRTL,
}

impl AssetClassProductType5Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassProductType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType6Code {
	#[default]
	#[serde(rename = "INDP")]
	CodeINDP,
}

impl AssetClassProductType6Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassProductType7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType7Code {
	#[default]
	#[serde(rename = "METL")]
	CodeMETL,
}

impl AssetClassProductType7Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassProductType8Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType8Code {
	#[default]
	#[serde(rename = "PAPR")]
	CodePAPR,
}

impl AssetClassProductType8Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassProductType9Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType9Code {
	#[default]
	#[serde(rename = "POLY")]
	CodePOLY,
}

impl AssetClassProductType9Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType10Code {
	#[default]
	#[serde(rename = "EMIS")]
	CodeEMIS,
}

impl AssetClassSubProductType10Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType15Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType15Code {
	#[default]
	#[serde(rename = "NPRM")]
	CodeNPRM,
}

impl AssetClassSubProductType15Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType16Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType16Code {
	#[default]
	#[serde(rename = "PRME")]
	CodePRME,
}

impl AssetClassSubProductType16Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType18Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType18Code {
	#[default]
	#[serde(rename = "PLST")]
	CodePLST,
}

impl AssetClassSubProductType18Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType1Code {
	#[default]
	#[serde(rename = "GROS")]
	CodeGROS,
}

impl AssetClassSubProductType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType20Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType20Code {
	#[default]
	#[serde(rename = "DIRY")]
	CodeDIRY,
}

impl AssetClassSubProductType20Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType21Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType21Code {
	#[default]
	#[serde(rename = "FRST")]
	CodeFRST,
}

impl AssetClassSubProductType21Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType22Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType22Code {
	#[default]
	#[serde(rename = "LSTK")]
	CodeLSTK,
}

impl AssetClassSubProductType22Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType23Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType23Code {
	#[default]
	#[serde(rename = "SEAF")]
	CodeSEAF,
}

impl AssetClassSubProductType23Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType24Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType24Code {
	#[default]
	#[serde(rename = "COAL")]
	CodeCOAL,
}

impl AssetClassSubProductType24Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType25Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType25Code {
	#[default]
	#[serde(rename = "DIST")]
	CodeDIST,
}

impl AssetClassSubProductType25Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType26Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType26Code {
	#[default]
	#[serde(rename = "INRG")]
	CodeINRG,
}

impl AssetClassSubProductType26Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType27Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType27Code {
	#[default]
	#[serde(rename = "LGHT")]
	CodeLGHT,
}

impl AssetClassSubProductType27Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType28Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType28Code {
	#[default]
	#[serde(rename = "RNNG")]
	CodeRNNG,
}

impl AssetClassSubProductType28Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType29Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType29Code {
	#[default]
	#[serde(rename = "CRBR")]
	CodeCRBR,
}

impl AssetClassSubProductType29Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType2Code {
	#[default]
	#[serde(rename = "SOFT")]
	CodeSOFT,
}

impl AssetClassSubProductType2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType30Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType30Code {
	#[default]
	#[serde(rename = "WTHR")]
	CodeWTHR,
}

impl AssetClassSubProductType30Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType31Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType31Code {
	#[default]
	#[serde(rename = "DRYF")]
	CodeDRYF,
}

impl AssetClassSubProductType31Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType32Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType32Code {
	#[default]
	#[serde(rename = "WETF")]
	CodeWETF,
}

impl AssetClassSubProductType32Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType33Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType33Code {
	#[default]
	#[serde(rename = "CSTR")]
	CodeCSTR,
}

impl AssetClassSubProductType33Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType34Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType34Code {
	#[default]
	#[serde(rename = "MFTG")]
	CodeMFTG,
}

impl AssetClassSubProductType34Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType35Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType35Code {
	#[default]
	#[serde(rename = "CBRD")]
	CodeCBRD,
}

impl AssetClassSubProductType35Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType36Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType36Code {
	#[default]
	#[serde(rename = "NSPT")]
	CodeNSPT,
}

impl AssetClassSubProductType36Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType37Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType37Code {
	#[default]
	#[serde(rename = "PULP")]
	CodePULP,
}

impl AssetClassSubProductType37Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType38Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType38Code {
	#[default]
	#[serde(rename = "RCVP")]
	CodeRCVP,
}

impl AssetClassSubProductType38Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType39Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType39Code {
	#[default]
	#[serde(rename = "AMMO")]
	CodeAMMO,
}

impl AssetClassSubProductType39Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType3Code {
	#[default]
	#[serde(rename = "OOLI")]
	CodeOOLI,
}

impl AssetClassSubProductType3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType40Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType40Code {
	#[default]
	#[serde(rename = "DAPH")]
	CodeDAPH,
}

impl AssetClassSubProductType40Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType41Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType41Code {
	#[default]
	#[serde(rename = "PTSH")]
	CodePTSH,
}

impl AssetClassSubProductType41Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType42Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType42Code {
	#[default]
	#[serde(rename = "SLPH")]
	CodeSLPH,
}

impl AssetClassSubProductType42Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType43Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType43Code {
	#[default]
	#[serde(rename = "UREA")]
	CodeUREA,
}

impl AssetClassSubProductType43Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType44Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType44Code {
	#[default]
	#[serde(rename = "UAAN")]
	CodeUAAN,
}

impl AssetClassSubProductType44Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType45Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType45Code {
	#[default]
	#[serde(rename = "POTA")]
	CodePOTA,
}

impl AssetClassSubProductType45Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType46Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType46Code {
	#[default]
	#[serde(rename = "CSHP")]
	CodeCSHP,
}

impl AssetClassSubProductType46Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType47Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType47Code {
	#[default]
	#[serde(rename = "DLVR")]
	CodeDLVR,
}

impl AssetClassSubProductType47Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType48Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType48Code {
	#[default]
	#[serde(rename = "NDLV")]
	CodeNDLV,
}

impl AssetClassSubProductType48Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType49Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType49Code {
	#[default]
	#[serde(rename = "OTHR")]
	CodeOTHR,
}

impl AssetClassSubProductType49Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType5Code {
	#[default]
	#[serde(rename = "GRIN")]
	CodeGRIN,
}

impl AssetClassSubProductType5Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType6Code {
	#[default]
	#[serde(rename = "ELEC")]
	CodeELEC,
}

impl AssetClassSubProductType6Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType7Code {
	#[default]
	#[serde(rename = "NGAS")]
	CodeNGAS,
}

impl AssetClassSubProductType7Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassSubProductType8Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType8Code {
	#[default]
	#[serde(rename = "OILP")]
	CodeOILP,
}

impl AssetClassSubProductType8Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BaseOneRate {
	#[serde(rename = "$value")]
	pub base_one_rate: f64,
}

impl BaseOneRate {
	pub fn validate(&self) -> bool {
		return true
	}
}


// BenchmarkCurveName10Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkCurveName10Choice {
	#[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
	pub indx: Option<BenchmarkCurveName3Code>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max350Text>,
}

impl BenchmarkCurveName10Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref indx_value) = self.indx { if !indx_value.validate() { return false; } }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		return true
	}
}


// BenchmarkCurveName3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BenchmarkCurveName3Code {
	#[default]
	#[serde(rename = "ESTR")]
	CodeESTR,
	#[serde(rename = "BBSW")]
	CodeBBSW,
	#[serde(rename = "BUBO")]
	CodeBUBO,
	#[serde(rename = "CDOR")]
	CodeCDOR,
	#[serde(rename = "CIBO")]
	CodeCIBO,
	#[serde(rename = "EONA")]
	CodeEONA,
	#[serde(rename = "EONS")]
	CodeEONS,
	#[serde(rename = "EURI")]
	CodeEURI,
	#[serde(rename = "EUUS")]
	CodeEUUS,
	#[serde(rename = "EUCH")]
	CodeEUCH,
	#[serde(rename = "FUSW")]
	CodeFUSW,
	#[serde(rename = "GCFR")]
	CodeGCFR,
	#[serde(rename = "ISDA")]
	CodeISDA,
	#[serde(rename = "JIBA")]
	CodeJIBA,
	#[serde(rename = "LIBI")]
	CodeLIBI,
	#[serde(rename = "LIBO")]
	CodeLIBO,
	#[serde(rename = "MOSP")]
	CodeMOSP,
	#[serde(rename = "MAAA")]
	CodeMAAA,
	#[serde(rename = "NIBO")]
	CodeNIBO,
	#[serde(rename = "PFAN")]
	CodePFAN,
	#[serde(rename = "PRBO")]
	CodePRBO,
	#[serde(rename = "STBO")]
	CodeSTBO,
	#[serde(rename = "SWAP")]
	CodeSWAP,
	#[serde(rename = "TLBO")]
	CodeTLBO,
	#[serde(rename = "TIBO")]
	CodeTIBO,
	#[serde(rename = "TREA")]
	CodeTREA,
	#[serde(rename = "WIBO")]
	CodeWIBO,
	#[serde(rename = "SOFR")]
	CodeSOFR,
	#[serde(rename = "SONA")]
	CodeSONA,
}

impl BenchmarkCurveName3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CFIOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CFIOct2015Identifier {
	#[serde(rename = "$value")]
	pub cfi_oct2015_identifier: String,
}

impl CFIOct2015Identifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{6,6}").unwrap();
		if !pattern.is_match(&self.cfi_oct2015_identifier) {
			return false
		}
		return true
	}
}


// CashCompare3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashCompare3 {
	#[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
	pub val: Option<CompareAmountAndDirection2>,
	#[serde(rename = "HrcutOrMrgn", skip_serializing_if = "Option::is_none")]
	pub hrcut_or_mrgn: Option<ComparePercentageRate3>,
}

impl CashCompare3 {
	pub fn validate(&self) -> bool {
		if let Some(ref val_value) = self.val { if !val_value.validate() { return false; } }
		if let Some(ref hrcut_or_mrgn_value) = self.hrcut_or_mrgn { if !hrcut_or_mrgn_value.validate() { return false; } }
		return true
	}
}


// Cleared4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Cleared4Choice {
	#[serde(rename = "Clrd", skip_serializing_if = "Option::is_none")]
	pub clrd: Option<NoReasonCode>,
	#[serde(rename = "NonClrd", skip_serializing_if = "Option::is_none")]
	pub non_clrd: Option<NoReasonCode>,
}

impl Cleared4Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref clrd_value) = self.clrd { if !clrd_value.validate() { return false; } }
		if let Some(ref non_clrd_value) = self.non_clrd { if !non_clrd_value.validate() { return false; } }
		return true
	}
}


// CollateralDeliveryMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CollateralDeliveryMethod1Code {
	#[default]
	#[serde(rename = "SICA")]
	CodeSICA,
	#[serde(rename = "SIUR")]
	CodeSIUR,
	#[serde(rename = "TTCA")]
	CodeTTCA,
}

impl CollateralDeliveryMethod1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CollateralMatchingCriteria6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralMatchingCriteria6 {
	#[serde(rename = "UncollsdFlg", skip_serializing_if = "Option::is_none")]
	pub uncollsd_flg: Option<CompareTrueFalseIndicator3>,
	#[serde(rename = "NetXpsrCollstnInd", skip_serializing_if = "Option::is_none")]
	pub net_xpsr_collstn_ind: Option<CompareTrueFalseIndicator3>,
	#[serde(rename = "CollValDt", skip_serializing_if = "Option::is_none")]
	pub coll_val_dt: Option<CompareDate3>,
	#[serde(rename = "AsstTp", skip_serializing_if = "Option::is_none")]
	pub asst_tp: Option<SecurityCommodityCash4>,
	#[serde(rename = "BsktIdr", skip_serializing_if = "Option::is_none")]
	pub bskt_idr: Option<CompareSecurityIdentification4>,
}

impl CollateralMatchingCriteria6 {
	pub fn validate(&self) -> bool {
		if let Some(ref uncollsd_flg_value) = self.uncollsd_flg { if !uncollsd_flg_value.validate() { return false; } }
		if let Some(ref net_xpsr_collstn_ind_value) = self.net_xpsr_collstn_ind { if !net_xpsr_collstn_ind_value.validate() { return false; } }
		if let Some(ref coll_val_dt_value) = self.coll_val_dt { if !coll_val_dt_value.validate() { return false; } }
		if let Some(ref asst_tp_value) = self.asst_tp { if !asst_tp_value.validate() { return false; } }
		if let Some(ref bskt_idr_value) = self.bskt_idr { if !bskt_idr_value.validate() { return false; } }
		return true
	}
}


// CollateralQualityType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CollateralQualityType1Code {
	#[default]
	#[serde(rename = "INVG")]
	CodeINVG,
	#[serde(rename = "NIVG")]
	CodeNIVG,
	#[serde(rename = "NOTR")]
	CodeNOTR,
	#[serde(rename = "NOAP")]
	CodeNOAP,
}

impl CollateralQualityType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CollateralRole1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CollateralRole1Code {
	#[default]
	#[serde(rename = "GIVE")]
	CodeGIVE,
	#[serde(rename = "TAKE")]
	CodeTAKE,
}

impl CollateralRole1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Commodity42 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Commodity42 {
	#[serde(rename = "Clssfctn", skip_serializing_if = "Option::is_none")]
	pub clssfctn: Option<CompareCommodityAssetClass3>,
	#[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
	pub qty: Option<CompareDecimalNumber3>,
	#[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
	pub unit_pric: Option<CompareUnitPrice6>,
	#[serde(rename = "MktVal", skip_serializing_if = "Option::is_none")]
	pub mkt_val: Option<CompareAmountAndDirection2>,
	#[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
	pub unit_of_measr: Option<CompareUnitOfMeasure3>,
}

impl Commodity42 {
	pub fn validate(&self) -> bool {
		if let Some(ref clssfctn_value) = self.clssfctn { if !clssfctn_value.validate() { return false; } }
		if let Some(ref qty_value) = self.qty { if !qty_value.validate() { return false; } }
		if let Some(ref unit_pric_value) = self.unit_pric { if !unit_pric_value.validate() { return false; } }
		if let Some(ref mkt_val_value) = self.mkt_val { if !mkt_val_value.validate() { return false; } }
		if let Some(ref unit_of_measr_value) = self.unit_of_measr { if !unit_of_measr_value.validate() { return false; } }
		return true
	}
}


// CompareActiveOrHistoricCurrencyAndAmount3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareActiveOrHistoricCurrencyAndAmount3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl CompareActiveOrHistoricCurrencyAndAmount3 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareAgreementType2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareAgreementType2 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<AgreementType1Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<AgreementType1Choice>,
}

impl CompareAgreementType2 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareAmountAndDirection1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareAmountAndDirection1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<AmountAndDirection53>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<AmountAndDirection53>,
}

impl CompareAmountAndDirection1 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareAmountAndDirection2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareAmountAndDirection2 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<AmountAndDirection53>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<AmountAndDirection53>,
}

impl CompareAmountAndDirection2 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareBenchmarkCurveName3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareBenchmarkCurveName3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<BenchmarkCurveName10Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<BenchmarkCurveName10Choice>,
}

impl CompareBenchmarkCurveName3 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareCFIIdentifier3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareCFIIdentifier3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<CFIOct2015Identifier>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<CFIOct2015Identifier>,
}

impl CompareCFIIdentifier3 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareClearingStatus3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareClearingStatus3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<Cleared4Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<Cleared4Choice>,
}

impl CompareClearingStatus3 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareCollateralQualityType3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareCollateralQualityType3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<CollateralQualityType1Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<CollateralQualityType1Code>,
}

impl CompareCollateralQualityType3 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareCommodityAssetClass3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareCommodityAssetClass3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<AssetClassCommodity5Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<AssetClassCommodity5Choice>,
}

impl CompareCommodityAssetClass3 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareCounterpartySide2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareCounterpartySide2 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<CollateralRole1Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<CollateralRole1Code>,
}

impl CompareCounterpartySide2 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareCountryCode3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareCountryCode3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<CountryCode>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<CountryCode>,
}

impl CompareCountryCode3 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareDate3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareDate3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<String>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<String>,
}

impl CompareDate3 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CompareDateTime3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareDateTime3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<String>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<String>,
}

impl CompareDateTime3 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CompareDecimalNumber3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareDecimalNumber3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<f64>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<f64>,
}

impl CompareDecimalNumber3 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CompareDeliveryMethod3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareDeliveryMethod3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<CollateralDeliveryMethod1Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<CollateralDeliveryMethod1Code>,
}

impl CompareDeliveryMethod3 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareExposureType3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareExposureType3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<ExposureType10Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<ExposureType10Code>,
}

impl CompareExposureType3 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareISINIdentifier4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareISINIdentifier4 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<ISINOct2015Identifier>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<ISINOct2015Identifier>,
}

impl CompareISINIdentifier4 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareInterestComputationMethod3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareInterestComputationMethod3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<InterestComputationMethodFormat6Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<InterestComputationMethodFormat6Choice>,
}

impl CompareInterestComputationMethod3 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareInterestRate1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareInterestRate1 {
	#[serde(rename = "MrgnLnAmt", skip_serializing_if = "Option::is_none")]
	pub mrgn_ln_amt: Option<CompareAmountAndDirection1>,
	#[serde(rename = "FxdIntrstRate", skip_serializing_if = "Option::is_none")]
	pub fxd_intrst_rate: Option<ComparePercentageRate3>,
	#[serde(rename = "DayCntBsis", skip_serializing_if = "Option::is_none")]
	pub day_cnt_bsis: Option<CompareInterestComputationMethod3>,
	#[serde(rename = "FltgIntrstRefRate", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_ref_rate: Option<CompareBenchmarkCurveName3>,
	#[serde(rename = "FltgIntrstRateTermUnit", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_rate_term_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgIntrstRateTermVal", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_rate_term_val: Option<CompareNumber5>,
	#[serde(rename = "FltgIntrstRatePmtFrqcyUnit", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_rate_pmt_frqcy_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgIntrstRatePmtFrqcyVal", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_rate_pmt_frqcy_val: Option<CompareNumber5>,
	#[serde(rename = "FltgIntrstRateRstFrqcyUnit", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_rate_rst_frqcy_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgIntrstRateRstFrqcyVal", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_rate_rst_frqcy_val: Option<CompareNumber6>,
	#[serde(rename = "BsisPtSprd", skip_serializing_if = "Option::is_none")]
	pub bsis_pt_sprd: Option<CompareDecimalNumber3>,
}

impl CompareInterestRate1 {
	pub fn validate(&self) -> bool {
		if let Some(ref mrgn_ln_amt_value) = self.mrgn_ln_amt { if !mrgn_ln_amt_value.validate() { return false; } }
		if let Some(ref fxd_intrst_rate_value) = self.fxd_intrst_rate { if !fxd_intrst_rate_value.validate() { return false; } }
		if let Some(ref day_cnt_bsis_value) = self.day_cnt_bsis { if !day_cnt_bsis_value.validate() { return false; } }
		if let Some(ref fltg_intrst_ref_rate_value) = self.fltg_intrst_ref_rate { if !fltg_intrst_ref_rate_value.validate() { return false; } }
		if let Some(ref fltg_intrst_rate_term_unit_value) = self.fltg_intrst_rate_term_unit { if !fltg_intrst_rate_term_unit_value.validate() { return false; } }
		if let Some(ref fltg_intrst_rate_term_val_value) = self.fltg_intrst_rate_term_val { if !fltg_intrst_rate_term_val_value.validate() { return false; } }
		if let Some(ref fltg_intrst_rate_pmt_frqcy_unit_value) = self.fltg_intrst_rate_pmt_frqcy_unit { if !fltg_intrst_rate_pmt_frqcy_unit_value.validate() { return false; } }
		if let Some(ref fltg_intrst_rate_pmt_frqcy_val_value) = self.fltg_intrst_rate_pmt_frqcy_val { if !fltg_intrst_rate_pmt_frqcy_val_value.validate() { return false; } }
		if let Some(ref fltg_intrst_rate_rst_frqcy_unit_value) = self.fltg_intrst_rate_rst_frqcy_unit { if !fltg_intrst_rate_rst_frqcy_unit_value.validate() { return false; } }
		if let Some(ref fltg_intrst_rate_rst_frqcy_val_value) = self.fltg_intrst_rate_rst_frqcy_val { if !fltg_intrst_rate_rst_frqcy_val_value.validate() { return false; } }
		if let Some(ref bsis_pt_sprd_value) = self.bsis_pt_sprd { if !bsis_pt_sprd_value.validate() { return false; } }
		return true
	}
}


// CompareMICIdentifier3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareMICIdentifier3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<MICIdentifier>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<MICIdentifier>,
}

impl CompareMICIdentifier3 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareNumber5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareNumber5 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<f64>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<f64>,
}

impl CompareNumber5 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CompareNumber6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareNumber6 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<f64>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<f64>,
}

impl CompareNumber6 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CompareOrganisationIdentification6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareOrganisationIdentification6 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<OrganisationIdentification15Choice>,
}

impl CompareOrganisationIdentification6 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareOrganisationIdentification7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareOrganisationIdentification7 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<PartyIdentification236Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<PartyIdentification236Choice>,
}

impl CompareOrganisationIdentification7 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// ComparePercentageRate3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ComparePercentageRate3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<f64>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<f64>,
}

impl ComparePercentageRate3 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CompareRateBasis3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareRateBasis3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<RateBasis1Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<RateBasis1Code>,
}

impl CompareRateBasis3 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareReportingLevelType3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareReportingLevelType3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<ModificationLevel1Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<ModificationLevel1Code>,
}

impl CompareReportingLevelType3 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareSecuritiesLendingType3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareSecuritiesLendingType3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<SecuritiesLendingType3Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<SecuritiesLendingType3Choice>,
}

impl CompareSecuritiesLendingType3 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareSecurityIdentification4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareSecurityIdentification4 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<SecurityIdentification26Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<SecurityIdentification26Choice>,
}

impl CompareSecurityIdentification4 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareSpecialCollateral3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareSpecialCollateral3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<SpecialCollateral1Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<SpecialCollateral1Code>,
}

impl CompareSpecialCollateral3 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareTerminationOption3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareTerminationOption3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<RepoTerminationOption2Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<RepoTerminationOption2Code>,
}

impl CompareTerminationOption3 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareText2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareText2 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<Max52Text>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<Max52Text>,
}

impl CompareText2 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareTrueFalseIndicator3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareTrueFalseIndicator3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<bool>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<bool>,
}

impl CompareTrueFalseIndicator3 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CompareUnitOfMeasure3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareUnitOfMeasure3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<UnitOfMeasure11Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<UnitOfMeasure11Code>,
}

impl CompareUnitOfMeasure3 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareUnitPrice6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareUnitPrice6 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<SecuritiesTransactionPrice19Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<SecuritiesTransactionPrice19Choice>,
}

impl CompareUnitPrice6 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CounterpartyMatchingCriteria4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyMatchingCriteria4 {
	#[serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none")]
	pub rptg_ctr_pty: Option<CompareOrganisationIdentification6>,
	#[serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none")]
	pub othr_ctr_pty: Option<CompareOrganisationIdentification7>,
	#[serde(rename = "CtrPtySd", skip_serializing_if = "Option::is_none")]
	pub ctr_pty_sd: Option<CompareCounterpartySide2>,
}

impl CounterpartyMatchingCriteria4 {
	pub fn validate(&self) -> bool {
		if let Some(ref rptg_ctr_pty_value) = self.rptg_ctr_pty { if !rptg_ctr_pty_value.validate() { return false; } }
		if let Some(ref othr_ctr_pty_value) = self.othr_ctr_pty { if !othr_ctr_pty_value.validate() { return false; } }
		if let Some(ref ctr_pty_sd_value) = self.ctr_pty_sd { if !ctr_pty_sd_value.validate() { return false; } }
		return true
	}
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}

impl CountryCode {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.country_code) {
			return false
		}
		return true
	}
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DecimalNumber {
	#[serde(rename = "$value")]
	pub decimal_number: f64,
}

impl DecimalNumber {
	pub fn validate(&self) -> bool {
		return true
	}
}


// EnergyCommodityCoal1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityCoal1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType24Code,
}

impl EnergyCommodityCoal1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		return true
	}
}


// EnergyCommodityDistillates1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityDistillates1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType25Code,
}

impl EnergyCommodityDistillates1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		return true
	}
}


// EnergyCommodityElectricity1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityElectricity1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType6Code,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: AssetClassDetailedSubProductType5Code,
}

impl EnergyCommodityElectricity1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		if !self.addtl_sub_pdct.validate() { return false }
		return true
	}
}


// EnergyCommodityInterEnergy1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityInterEnergy1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType26Code,
}

impl EnergyCommodityInterEnergy1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		return true
	}
}


// EnergyCommodityLightEnd1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityLightEnd1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType27Code,
}

impl EnergyCommodityLightEnd1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		return true
	}
}


// EnergyCommodityNaturalGas2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityNaturalGas2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType7Code,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: AssetClassDetailedSubProductType31Code,
}

impl EnergyCommodityNaturalGas2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		if !self.addtl_sub_pdct.validate() { return false }
		return true
	}
}


// EnergyCommodityOil2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityOil2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType8Code,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: AssetClassDetailedSubProductType32Code,
}

impl EnergyCommodityOil2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		if !self.addtl_sub_pdct.validate() { return false }
		return true
	}
}


// EnergyCommodityOther1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType49Code,
}

impl EnergyCommodityOther1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		return true
	}
}


// EnergyCommodityRenewableEnergy1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityRenewableEnergy1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType28Code,
}

impl EnergyCommodityRenewableEnergy1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		return true
	}
}


// EnvironmentCommodityOther1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType3Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType49Code,
}

impl EnvironmentCommodityOther1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		return true
	}
}


// EnvironmentalCommodityCarbonRelated1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCommodityCarbonRelated1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType3Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType29Code,
}

impl EnvironmentalCommodityCarbonRelated1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		return true
	}
}


// EnvironmentalCommodityEmission2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCommodityEmission2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType3Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType10Code,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: AssetClassDetailedSubProductType8Code,
}

impl EnvironmentalCommodityEmission2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		if !self.addtl_sub_pdct.validate() { return false }
		return true
	}
}


// EnvironmentalCommodityWeather1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCommodityWeather1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType3Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType30Code,
}

impl EnvironmentalCommodityWeather1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		return true
	}
}


// ExposureType10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ExposureType10Code {
	#[default]
	#[serde(rename = "SBSC")]
	CodeSBSC,
	#[serde(rename = "MGLD")]
	CodeMGLD,
	#[serde(rename = "SLEB")]
	CodeSLEB,
	#[serde(rename = "REPO")]
	CodeREPO,
}

impl ExposureType10Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ExternalAgreementType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalAgreementType1Code {
	#[serde(rename = "$value")]
	pub external_agreement_type1_code: String,
}

impl ExternalAgreementType1Code {
	pub fn validate(&self) -> bool {
		if self.external_agreement_type1_code.chars().count() < 1 {
			return false
		}
		if self.external_agreement_type1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// ExternalSecuritiesLendingType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalSecuritiesLendingType1Code {
	#[serde(rename = "$value")]
	pub external_securities_lending_type1_code: String,
}

impl ExternalSecuritiesLendingType1Code {
	pub fn validate(&self) -> bool {
		if self.external_securities_lending_type1_code.chars().count() < 1 {
			return false
		}
		if self.external_securities_lending_type1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// FertilizerCommodityAmmonia1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityAmmonia1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType39Code,
}

impl FertilizerCommodityAmmonia1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		return true
	}
}


// FertilizerCommodityDiammoniumPhosphate1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityDiammoniumPhosphate1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType40Code,
}

impl FertilizerCommodityDiammoniumPhosphate1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		return true
	}
}


// FertilizerCommodityOther1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType49Code,
}

impl FertilizerCommodityOther1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		return true
	}
}


// FertilizerCommodityPotash1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityPotash1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType41Code,
}

impl FertilizerCommodityPotash1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		return true
	}
}


// FertilizerCommoditySulphur1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommoditySulphur1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType42Code,
}

impl FertilizerCommoditySulphur1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		return true
	}
}


// FertilizerCommodityUrea1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityUrea1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType43Code,
}

impl FertilizerCommodityUrea1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		return true
	}
}


// FertilizerCommodityUreaAndAmmoniumNitrate1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityUreaAndAmmoniumNitrate1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType44Code,
}

impl FertilizerCommodityUreaAndAmmoniumNitrate1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		return true
	}
}


// FreightCommodityContainerShip1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityContainerShip1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType4Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType46Code,
}

impl FreightCommodityContainerShip1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		return true
	}
}


// FreightCommodityDry2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityDry2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType4Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType31Code,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: AssetClassDetailedSubProductType33Code,
}

impl FreightCommodityDry2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		if !self.addtl_sub_pdct.validate() { return false }
		return true
	}
}


// FreightCommodityOther1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType4Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType49Code,
}

impl FreightCommodityOther1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		return true
	}
}


// FreightCommodityWet2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityWet2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType4Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType32Code,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: AssetClassDetailedSubProductType34Code,
}

impl FreightCommodityWet2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		if !self.addtl_sub_pdct.validate() { return false }
		return true
	}
}


// GenericIdentification175 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification175 {
	#[serde(rename = "Id")]
	pub id: Max72Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericIdentification175 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		return true
	}
}


// ISINOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "$value")]
	pub isin_oct2015_identifier: String,
}

impl ISINOct2015Identifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
		if !pattern.is_match(&self.isin_oct2015_identifier) {
			return false
		}
		return true
	}
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}

impl ISODate {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}

impl ISODateTime {
	pub fn validate(&self) -> bool {
		return true
	}
}


// IndustrialProductCommodityConstruction1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndustrialProductCommodityConstruction1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType6Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType33Code>,
}

impl IndustrialProductCommodityConstruction1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// IndustrialProductCommodityManufacturing1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndustrialProductCommodityManufacturing1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType6Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType34Code>,
}

impl IndustrialProductCommodityManufacturing1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// InterestComputationMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InterestComputationMethod1Code {
	#[default]
	#[serde(rename = "A001")]
	CodeA001,
	#[serde(rename = "A002")]
	CodeA002,
	#[serde(rename = "A003")]
	CodeA003,
	#[serde(rename = "A004")]
	CodeA004,
	#[serde(rename = "A005")]
	CodeA005,
	#[serde(rename = "A006")]
	CodeA006,
	#[serde(rename = "A007")]
	CodeA007,
	#[serde(rename = "A008")]
	CodeA008,
	#[serde(rename = "A009")]
	CodeA009,
	#[serde(rename = "A010")]
	CodeA010,
	#[serde(rename = "A011")]
	CodeA011,
	#[serde(rename = "A012")]
	CodeA012,
	#[serde(rename = "A013")]
	CodeA013,
	#[serde(rename = "A014")]
	CodeA014,
}

impl InterestComputationMethod1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// InterestComputationMethodFormat6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestComputationMethodFormat6Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InterestComputationMethod1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl InterestComputationMethodFormat6Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}

impl LEIIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.lei_identifier) {
			return false
		}
		return true
	}
}


// LoanMatchingCriteria9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LoanMatchingCriteria9 {
	#[serde(rename = "UnqTradIdr", skip_serializing_if = "Option::is_none")]
	pub unq_trad_idr: Option<CompareText2>,
	#[serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none")]
	pub termntn_dt: Option<CompareDate3>,
	#[serde(rename = "CtrctTp", skip_serializing_if = "Option::is_none")]
	pub ctrct_tp: Option<CompareExposureType3>,
	#[serde(rename = "ClrSts", skip_serializing_if = "Option::is_none")]
	pub clr_sts: Option<CompareClearingStatus3>,
	#[serde(rename = "ClrDtTm", skip_serializing_if = "Option::is_none")]
	pub clr_dt_tm: Option<CompareDateTime3>,
	#[serde(rename = "CCP", skip_serializing_if = "Option::is_none")]
	pub ccp: Option<CompareOrganisationIdentification6>,
	#[serde(rename = "TradgVn", skip_serializing_if = "Option::is_none")]
	pub tradg_vn: Option<CompareMICIdentifier3>,
	#[serde(rename = "MstrAgrmtTp", skip_serializing_if = "Option::is_none")]
	pub mstr_agrmt_tp: Option<CompareAgreementType2>,
	#[serde(rename = "ExctnDtTm", skip_serializing_if = "Option::is_none")]
	pub exctn_dt_tm: Option<CompareDateTime3>,
	#[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
	pub val_dt: Option<CompareDate3>,
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<CompareDate3>,
	#[serde(rename = "MinNtcePrd", skip_serializing_if = "Option::is_none")]
	pub min_ntce_prd: Option<CompareNumber5>,
	#[serde(rename = "EarlstCallBckDt", skip_serializing_if = "Option::is_none")]
	pub earlst_call_bck_dt: Option<CompareDate3>,
	#[serde(rename = "GnlColl", skip_serializing_if = "Option::is_none")]
	pub gnl_coll: Option<CompareSpecialCollateral3>,
	#[serde(rename = "DlvryByVal", skip_serializing_if = "Option::is_none")]
	pub dlvry_by_val: Option<CompareTrueFalseIndicator3>,
	#[serde(rename = "CollDlvryMtd", skip_serializing_if = "Option::is_none")]
	pub coll_dlvry_mtd: Option<CompareDeliveryMethod3>,
	#[serde(rename = "OpnTerm", skip_serializing_if = "Option::is_none")]
	pub opn_term: Option<CompareTrueFalseIndicator3>,
	#[serde(rename = "TermntnOptn", skip_serializing_if = "Option::is_none")]
	pub termntn_optn: Option<CompareTerminationOption3>,
	#[serde(rename = "FxdIntrstRate", skip_serializing_if = "Option::is_none")]
	pub fxd_intrst_rate: Option<ComparePercentageRate3>,
	#[serde(rename = "DayCntBsis", skip_serializing_if = "Option::is_none")]
	pub day_cnt_bsis: Option<CompareInterestComputationMethod3>,
	#[serde(rename = "FltgIntrstRefRate", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_ref_rate: Option<CompareBenchmarkCurveName3>,
	#[serde(rename = "FltgIntrstRateTermUnit", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_rate_term_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgIntrstRateTermVal", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_rate_term_val: Option<CompareNumber5>,
	#[serde(rename = "FltgIntrstRatePmtFrqcyUnit", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_rate_pmt_frqcy_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgIntrstRatePmtFrqcyVal", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_rate_pmt_frqcy_val: Option<CompareNumber5>,
	#[serde(rename = "FltgIntrstRateRstFrqcyUnit", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_rate_rst_frqcy_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgIntrstRateRstFrqcyVal", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_rate_rst_frqcy_val: Option<CompareNumber6>,
	#[serde(rename = "BsisPtSprd", skip_serializing_if = "Option::is_none")]
	pub bsis_pt_sprd: Option<CompareDecimalNumber3>,
	#[serde(rename = "MrgnLnAttr", skip_serializing_if = "Option::is_none")]
	pub mrgn_ln_attr: Option<Vec<CompareInterestRate1>>,
	#[serde(rename = "PrncplAmtValDtAmt", skip_serializing_if = "Option::is_none")]
	pub prncpl_amt_val_dt_amt: Option<CompareActiveOrHistoricCurrencyAndAmount3>,
	#[serde(rename = "PrncplAmtMtrtyDtAmt", skip_serializing_if = "Option::is_none")]
	pub prncpl_amt_mtrty_dt_amt: Option<CompareActiveOrHistoricCurrencyAndAmount3>,
	#[serde(rename = "AsstTp", skip_serializing_if = "Option::is_none")]
	pub asst_tp: Option<SecurityCommodity7Choice>,
	#[serde(rename = "LnVal", skip_serializing_if = "Option::is_none")]
	pub ln_val: Option<CompareActiveOrHistoricCurrencyAndAmount3>,
	#[serde(rename = "FxdRbtRefRate", skip_serializing_if = "Option::is_none")]
	pub fxd_rbt_ref_rate: Option<ComparePercentageRate3>,
	#[serde(rename = "FltgRbtRefRate", skip_serializing_if = "Option::is_none")]
	pub fltg_rbt_ref_rate: Option<CompareBenchmarkCurveName3>,
	#[serde(rename = "FltgRbtRateTermUnit", skip_serializing_if = "Option::is_none")]
	pub fltg_rbt_rate_term_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgRbtRateTermVal", skip_serializing_if = "Option::is_none")]
	pub fltg_rbt_rate_term_val: Option<CompareNumber6>,
	#[serde(rename = "FltgRbtRatePmtFrqcyUnit", skip_serializing_if = "Option::is_none")]
	pub fltg_rbt_rate_pmt_frqcy_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgRbtRatePmtFrqcyVal", skip_serializing_if = "Option::is_none")]
	pub fltg_rbt_rate_pmt_frqcy_val: Option<CompareNumber6>,
	#[serde(rename = "FltgRbtRateRstFrqcyUnit", skip_serializing_if = "Option::is_none")]
	pub fltg_rbt_rate_rst_frqcy_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgRbtRateRstFrqcyVal", skip_serializing_if = "Option::is_none")]
	pub fltg_rbt_rate_rst_frqcy_val: Option<CompareNumber6>,
	#[serde(rename = "RbtRateBsisPtSprd", skip_serializing_if = "Option::is_none")]
	pub rbt_rate_bsis_pt_sprd: Option<CompareDecimalNumber3>,
	#[serde(rename = "FltgRateAdjstmnt", skip_serializing_if = "Option::is_none")]
	pub fltg_rate_adjstmnt: Option<Vec<ComparePercentageRate3>>,
	#[serde(rename = "FltgRateAdjstmntDt", skip_serializing_if = "Option::is_none")]
	pub fltg_rate_adjstmnt_dt: Option<Vec<CompareDate3>>,
	#[serde(rename = "LndgFee", skip_serializing_if = "Option::is_none")]
	pub lndg_fee: Option<ComparePercentageRate3>,
	#[serde(rename = "OutsdngMrgnLnAmt", skip_serializing_if = "Option::is_none")]
	pub outsdng_mrgn_ln_amt: Option<CompareActiveOrHistoricCurrencyAndAmount3>,
	#[serde(rename = "ShrtMktValAmt", skip_serializing_if = "Option::is_none")]
	pub shrt_mkt_val_amt: Option<CompareActiveOrHistoricCurrencyAndAmount3>,
	#[serde(rename = "LvlTp", skip_serializing_if = "Option::is_none")]
	pub lvl_tp: Option<CompareReportingLevelType3>,
	#[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
	pub unit_of_measr: Option<CompareUnitOfMeasure3>,
}

impl LoanMatchingCriteria9 {
	pub fn validate(&self) -> bool {
		if let Some(ref unq_trad_idr_value) = self.unq_trad_idr { if !unq_trad_idr_value.validate() { return false; } }
		if let Some(ref termntn_dt_value) = self.termntn_dt { if !termntn_dt_value.validate() { return false; } }
		if let Some(ref ctrct_tp_value) = self.ctrct_tp { if !ctrct_tp_value.validate() { return false; } }
		if let Some(ref clr_sts_value) = self.clr_sts { if !clr_sts_value.validate() { return false; } }
		if let Some(ref clr_dt_tm_value) = self.clr_dt_tm { if !clr_dt_tm_value.validate() { return false; } }
		if let Some(ref ccp_value) = self.ccp { if !ccp_value.validate() { return false; } }
		if let Some(ref tradg_vn_value) = self.tradg_vn { if !tradg_vn_value.validate() { return false; } }
		if let Some(ref mstr_agrmt_tp_value) = self.mstr_agrmt_tp { if !mstr_agrmt_tp_value.validate() { return false; } }
		if let Some(ref exctn_dt_tm_value) = self.exctn_dt_tm { if !exctn_dt_tm_value.validate() { return false; } }
		if let Some(ref val_dt_value) = self.val_dt { if !val_dt_value.validate() { return false; } }
		if let Some(ref mtrty_dt_value) = self.mtrty_dt { if !mtrty_dt_value.validate() { return false; } }
		if let Some(ref min_ntce_prd_value) = self.min_ntce_prd { if !min_ntce_prd_value.validate() { return false; } }
		if let Some(ref earlst_call_bck_dt_value) = self.earlst_call_bck_dt { if !earlst_call_bck_dt_value.validate() { return false; } }
		if let Some(ref gnl_coll_value) = self.gnl_coll { if !gnl_coll_value.validate() { return false; } }
		if let Some(ref dlvry_by_val_value) = self.dlvry_by_val { if !dlvry_by_val_value.validate() { return false; } }
		if let Some(ref coll_dlvry_mtd_value) = self.coll_dlvry_mtd { if !coll_dlvry_mtd_value.validate() { return false; } }
		if let Some(ref opn_term_value) = self.opn_term { if !opn_term_value.validate() { return false; } }
		if let Some(ref termntn_optn_value) = self.termntn_optn { if !termntn_optn_value.validate() { return false; } }
		if let Some(ref fxd_intrst_rate_value) = self.fxd_intrst_rate { if !fxd_intrst_rate_value.validate() { return false; } }
		if let Some(ref day_cnt_bsis_value) = self.day_cnt_bsis { if !day_cnt_bsis_value.validate() { return false; } }
		if let Some(ref fltg_intrst_ref_rate_value) = self.fltg_intrst_ref_rate { if !fltg_intrst_ref_rate_value.validate() { return false; } }
		if let Some(ref fltg_intrst_rate_term_unit_value) = self.fltg_intrst_rate_term_unit { if !fltg_intrst_rate_term_unit_value.validate() { return false; } }
		if let Some(ref fltg_intrst_rate_term_val_value) = self.fltg_intrst_rate_term_val { if !fltg_intrst_rate_term_val_value.validate() { return false; } }
		if let Some(ref fltg_intrst_rate_pmt_frqcy_unit_value) = self.fltg_intrst_rate_pmt_frqcy_unit { if !fltg_intrst_rate_pmt_frqcy_unit_value.validate() { return false; } }
		if let Some(ref fltg_intrst_rate_pmt_frqcy_val_value) = self.fltg_intrst_rate_pmt_frqcy_val { if !fltg_intrst_rate_pmt_frqcy_val_value.validate() { return false; } }
		if let Some(ref fltg_intrst_rate_rst_frqcy_unit_value) = self.fltg_intrst_rate_rst_frqcy_unit { if !fltg_intrst_rate_rst_frqcy_unit_value.validate() { return false; } }
		if let Some(ref fltg_intrst_rate_rst_frqcy_val_value) = self.fltg_intrst_rate_rst_frqcy_val { if !fltg_intrst_rate_rst_frqcy_val_value.validate() { return false; } }
		if let Some(ref bsis_pt_sprd_value) = self.bsis_pt_sprd { if !bsis_pt_sprd_value.validate() { return false; } }
		if let Some(ref mrgn_ln_attr_vec) = self.mrgn_ln_attr { for item in mrgn_ln_attr_vec { if !item.validate() { return false; } } }
		if let Some(ref prncpl_amt_val_dt_amt_value) = self.prncpl_amt_val_dt_amt { if !prncpl_amt_val_dt_amt_value.validate() { return false; } }
		if let Some(ref prncpl_amt_mtrty_dt_amt_value) = self.prncpl_amt_mtrty_dt_amt { if !prncpl_amt_mtrty_dt_amt_value.validate() { return false; } }
		if let Some(ref asst_tp_value) = self.asst_tp { if !asst_tp_value.validate() { return false; } }
		if let Some(ref ln_val_value) = self.ln_val { if !ln_val_value.validate() { return false; } }
		if let Some(ref fxd_rbt_ref_rate_value) = self.fxd_rbt_ref_rate { if !fxd_rbt_ref_rate_value.validate() { return false; } }
		if let Some(ref fltg_rbt_ref_rate_value) = self.fltg_rbt_ref_rate { if !fltg_rbt_ref_rate_value.validate() { return false; } }
		if let Some(ref fltg_rbt_rate_term_unit_value) = self.fltg_rbt_rate_term_unit { if !fltg_rbt_rate_term_unit_value.validate() { return false; } }
		if let Some(ref fltg_rbt_rate_term_val_value) = self.fltg_rbt_rate_term_val { if !fltg_rbt_rate_term_val_value.validate() { return false; } }
		if let Some(ref fltg_rbt_rate_pmt_frqcy_unit_value) = self.fltg_rbt_rate_pmt_frqcy_unit { if !fltg_rbt_rate_pmt_frqcy_unit_value.validate() { return false; } }
		if let Some(ref fltg_rbt_rate_pmt_frqcy_val_value) = self.fltg_rbt_rate_pmt_frqcy_val { if !fltg_rbt_rate_pmt_frqcy_val_value.validate() { return false; } }
		if let Some(ref fltg_rbt_rate_rst_frqcy_unit_value) = self.fltg_rbt_rate_rst_frqcy_unit { if !fltg_rbt_rate_rst_frqcy_unit_value.validate() { return false; } }
		if let Some(ref fltg_rbt_rate_rst_frqcy_val_value) = self.fltg_rbt_rate_rst_frqcy_val { if !fltg_rbt_rate_rst_frqcy_val_value.validate() { return false; } }
		if let Some(ref rbt_rate_bsis_pt_sprd_value) = self.rbt_rate_bsis_pt_sprd { if !rbt_rate_bsis_pt_sprd_value.validate() { return false; } }
		if let Some(ref fltg_rate_adjstmnt_vec) = self.fltg_rate_adjstmnt { for item in fltg_rate_adjstmnt_vec { if !item.validate() { return false; } } }
		if let Some(ref fltg_rate_adjstmnt_dt_vec) = self.fltg_rate_adjstmnt_dt { for item in fltg_rate_adjstmnt_dt_vec { if !item.validate() { return false; } } }
		if let Some(ref lndg_fee_value) = self.lndg_fee { if !lndg_fee_value.validate() { return false; } }
		if let Some(ref outsdng_mrgn_ln_amt_value) = self.outsdng_mrgn_ln_amt { if !outsdng_mrgn_ln_amt_value.validate() { return false; } }
		if let Some(ref shrt_mkt_val_amt_value) = self.shrt_mkt_val_amt { if !shrt_mkt_val_amt_value.validate() { return false; } }
		if let Some(ref lvl_tp_value) = self.lvl_tp { if !lvl_tp_value.validate() { return false; } }
		if let Some(ref unit_of_measr_value) = self.unit_of_measr { if !unit_of_measr_value.validate() { return false; } }
		return true
	}
}


// LongFraction19DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LongFraction19DecimalNumber {
	#[serde(rename = "$value")]
	pub long_fraction19_decimal_number: f64,
}

impl LongFraction19DecimalNumber {
	pub fn validate(&self) -> bool {
		return true
	}
}


// MICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MICIdentifier {
	#[serde(rename = "$value")]
	pub mic_identifier: String,
}

impl MICIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{4,4}").unwrap();
		if !pattern.is_match(&self.mic_identifier) {
			return false
		}
		return true
	}
}


// MasterAgreement7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MasterAgreement7 {
	#[serde(rename = "Tp")]
	pub tp: AgreementType2Choice,
	#[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
	pub vrsn: Option<Max50Text>,
	#[serde(rename = "OthrMstrAgrmtDtls", skip_serializing_if = "Option::is_none")]
	pub othr_mstr_agrmt_dtls: Option<Max350Text>,
}

impl MasterAgreement7 {
	pub fn validate(&self) -> bool {
		if !self.tp.validate() { return false }
		if let Some(ref vrsn_value) = self.vrsn { if !vrsn_value.validate() { return false; } }
		if let Some(ref othr_mstr_agrmt_dtls_value) = self.othr_mstr_agrmt_dtls { if !othr_mstr_agrmt_dtls_value.validate() { return false; } }
		return true
	}
}


// MatchingCriteria10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MatchingCriteria10 {
	#[serde(rename = "CtrPtyMtchgCrit", skip_serializing_if = "Option::is_none")]
	pub ctr_pty_mtchg_crit: Option<CounterpartyMatchingCriteria4>,
	#[serde(rename = "LnMtchgCrit", skip_serializing_if = "Option::is_none")]
	pub ln_mtchg_crit: Option<LoanMatchingCriteria9>,
	#[serde(rename = "CollMtchgCrit", skip_serializing_if = "Option::is_none")]
	pub coll_mtchg_crit: Option<CollateralMatchingCriteria6>,
}

impl MatchingCriteria10 {
	pub fn validate(&self) -> bool {
		if let Some(ref ctr_pty_mtchg_crit_value) = self.ctr_pty_mtchg_crit { if !ctr_pty_mtchg_crit_value.validate() { return false; } }
		if let Some(ref ln_mtchg_crit_value) = self.ln_mtchg_crit { if !ln_mtchg_crit_value.validate() { return false; } }
		if let Some(ref coll_mtchg_crit_value) = self.coll_mtchg_crit { if !coll_mtchg_crit_value.validate() { return false; } }
		return true
	}
}


// Max105Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max105Text {
	#[serde(rename = "$value")]
	pub max105_text: String,
}

impl Max105Text {
	pub fn validate(&self) -> bool {
		if self.max105_text.chars().count() < 1 {
			return false
		}
		if self.max105_text.chars().count() > 105 {
			return false
		}
		return true
	}
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
}

impl Max140Text {
	pub fn validate(&self) -> bool {
		if self.max140_text.chars().count() < 1 {
			return false
		}
		if self.max140_text.chars().count() > 140 {
			return false
		}
		return true
	}
}


// Max15NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max15NumericText {
	#[serde(rename = "$value")]
	pub max15_numeric_text: String,
}

impl Max15NumericText {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[0-9]{1,15}").unwrap();
		if !pattern.is_match(&self.max15_numeric_text) {
			return false
		}
		return true
	}
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}

impl Max350Text {
	pub fn validate(&self) -> bool {
		if self.max350_text.chars().count() < 1 {
			return false
		}
		if self.max350_text.chars().count() > 350 {
			return false
		}
		return true
	}
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}

impl Max35Text {
	pub fn validate(&self) -> bool {
		if self.max35_text.chars().count() < 1 {
			return false
		}
		if self.max35_text.chars().count() > 35 {
			return false
		}
		return true
	}
}


// Max3Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max3Number {
	#[serde(rename = "$value")]
	pub max3_number: f64,
}

impl Max3Number {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Max500Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max500Text {
	#[serde(rename = "$value")]
	pub max500_text: String,
}

impl Max500Text {
	pub fn validate(&self) -> bool {
		if self.max500_text.chars().count() < 1 {
			return false
		}
		if self.max500_text.chars().count() > 500 {
			return false
		}
		return true
	}
}


// Max50Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max50Text {
	#[serde(rename = "$value")]
	pub max50_text: String,
}

impl Max50Text {
	pub fn validate(&self) -> bool {
		if self.max50_text.chars().count() < 1 {
			return false
		}
		if self.max50_text.chars().count() > 50 {
			return false
		}
		return true
	}
}


// Max52Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max52Text {
	#[serde(rename = "$value")]
	pub max52_text: String,
}

impl Max52Text {
	pub fn validate(&self) -> bool {
		if self.max52_text.chars().count() < 1 {
			return false
		}
		if self.max52_text.chars().count() > 52 {
			return false
		}
		return true
	}
}


// Max5Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max5Number {
	#[serde(rename = "$value")]
	pub max5_number: f64,
}

impl Max5Number {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Max72Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max72Text {
	#[serde(rename = "$value")]
	pub max72_text: String,
}

impl Max72Text {
	pub fn validate(&self) -> bool {
		if self.max72_text.chars().count() < 1 {
			return false
		}
		if self.max72_text.chars().count() > 72 {
			return false
		}
		return true
	}
}


// MetalCommodityNonPrecious1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MetalCommodityNonPrecious1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType7Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType15Code,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: AssetClassDetailedSubProductType10Code,
}

impl MetalCommodityNonPrecious1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		if !self.addtl_sub_pdct.validate() { return false }
		return true
	}
}


// MetalCommodityPrecious1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MetalCommodityPrecious1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType7Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType16Code,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: AssetClassDetailedSubProductType11Code,
}

impl MetalCommodityPrecious1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		if !self.addtl_sub_pdct.validate() { return false }
		return true
	}
}


// ModificationLevel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ModificationLevel1Code {
	#[default]
	#[serde(rename = "PSTN")]
	CodePSTN,
	#[serde(rename = "TCTN")]
	CodeTCTN,
}

impl ModificationLevel1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// NaturalPersonIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NaturalPersonIdentification2 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max105Text>,
	#[serde(rename = "Dmcl", skip_serializing_if = "Option::is_none")]
	pub dmcl: Option<Max500Text>,
}

impl NaturalPersonIdentification2 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref dmcl_value) = self.dmcl { if !dmcl_value.validate() { return false; } }
		return true
	}
}


// NoReasonCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NoReasonCode {
	#[default]
	#[serde(rename = "NORE")]
	CodeNORE,
}

impl NoReasonCode {
	pub fn validate(&self) -> bool {
		return true
	}
}


// NotAvailable1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NotAvailable1Code {
	#[default]
	#[serde(rename = "NTAV")]
	CodeNTAV,
}

impl NotAvailable1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// NumberOfReportsPerStatus4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NumberOfReportsPerStatus4 {
	#[serde(rename = "DtldNbOfRpts")]
	pub dtld_nb_of_rpts: Max15NumericText,
	#[serde(rename = "DtldSts")]
	pub dtld_sts: PairedReconciled3Code,
}

impl NumberOfReportsPerStatus4 {
	pub fn validate(&self) -> bool {
		if !self.dtld_nb_of_rpts.validate() { return false }
		if !self.dtld_sts.validate() { return false }
		return true
	}
}


// OrganisationIdentification15Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification15Choice {
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<OrganisationIdentification38>,
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
}

impl OrganisationIdentification15Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref lei_value) = self.lei { if !lei_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		if let Some(ref any_bic_value) = self.any_bic { if !any_bic_value.validate() { return false; } }
		return true
	}
}


// OrganisationIdentification38 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification38 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max105Text>,
	#[serde(rename = "Dmcl", skip_serializing_if = "Option::is_none")]
	pub dmcl: Option<Max500Text>,
}

impl OrganisationIdentification38 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref dmcl_value) = self.dmcl { if !dmcl_value.validate() { return false; } }
		return true
	}
}


// OtherC10CommodityDeliverable2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherC10CommodityDeliverable2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType11Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType47Code>,
}

impl OtherC10CommodityDeliverable2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// OtherC10CommodityNonDeliverable2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherC10CommodityNonDeliverable2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType11Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType48Code>,
}

impl OtherC10CommodityNonDeliverable2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// PairedReconciled3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PairedReconciled3Code {
	#[default]
	#[serde(rename = "CLRC")]
	CodeCLRC,
	#[serde(rename = "LNRC")]
	CodeLNRC,
	#[serde(rename = "PARD")]
	CodePARD,
	#[serde(rename = "RECO")]
	CodeRECO,
	#[serde(rename = "UNPR")]
	CodeUNPR,
}

impl PairedReconciled3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// PaperCommodityContainerBoard1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityContainerBoard1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType8Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType35Code>,
}

impl PaperCommodityContainerBoard1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// PaperCommodityNewsprint1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityNewsprint1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType8Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType36Code>,
}

impl PaperCommodityNewsprint1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// PaperCommodityPulp1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityPulp1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType8Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType37Code>,
}

impl PaperCommodityPulp1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// PaperCommodityRecoveredPaper1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityRecoveredPaper1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType8Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType38Code>,
}

impl PaperCommodityRecoveredPaper1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// PaperCommodityRecoveredPaper2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityRecoveredPaper2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType8Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType49Code>,
}

impl PaperCommodityRecoveredPaper2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// PartyIdentification236Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification236Choice {
	#[serde(rename = "Lgl", skip_serializing_if = "Option::is_none")]
	pub lgl: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "Ntrl", skip_serializing_if = "Option::is_none")]
	pub ntrl: Option<NaturalPersonIdentification2>,
}

impl PartyIdentification236Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref lgl_value) = self.lgl { if !lgl_value.validate() { return false; } }
		if let Some(ref ntrl_value) = self.ntrl { if !ntrl_value.validate() { return false; } }
		return true
	}
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PercentageRate {
	#[serde(rename = "$value")]
	pub percentage_rate: f64,
}

impl PercentageRate {
	pub fn validate(&self) -> bool {
		return true
	}
}


// PlusOrMinusIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "$value")]
	pub plus_or_minus_indicator: bool,
}

impl PlusOrMinusIndicator {
	pub fn validate(&self) -> bool {
		return true
	}
}


// PolypropyleneCommodityOther1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PolypropyleneCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType9Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType49Code,
}

impl PolypropyleneCommodityOther1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		return true
	}
}


// PolypropyleneCommodityPlastic1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PolypropyleneCommodityPlastic1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType9Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType18Code>,
}

impl PolypropyleneCommodityPlastic1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// PriceStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PriceStatus1Code {
	#[default]
	#[serde(rename = "PNDG")]
	CodePNDG,
	#[serde(rename = "NOAP")]
	CodeNOAP,
}

impl PriceStatus1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// RateBasis1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RateBasis1Code {
	#[default]
	#[serde(rename = "DAYS")]
	CodeDAYS,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "YEAR")]
	CodeYEAR,
}

impl RateBasis1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ReconciliationMatchedStatus9Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReconciliationMatchedStatus9Choice {
	#[serde(rename = "Mtchd", skip_serializing_if = "Option::is_none")]
	pub mtchd: Option<NoReasonCode>,
	#[serde(rename = "NotMtchd", skip_serializing_if = "Option::is_none")]
	pub not_mtchd: Option<ReconciliationResult10>,
}

impl ReconciliationMatchedStatus9Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref mtchd_value) = self.mtchd { if !mtchd_value.validate() { return false; } }
		if let Some(ref not_mtchd_value) = self.not_mtchd { if !not_mtchd_value.validate() { return false; } }
		return true
	}
}


// ReconciliationReport8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReconciliationReport8 {
	#[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
	pub tech_rcrd_id: Option<Max140Text>,
	#[serde(rename = "TxId")]
	pub tx_id: TradeTransactionIdentification19,
	#[serde(rename = "Modfd")]
	pub modfd: bool,
	#[serde(rename = "RcncltnSts")]
	pub rcncltn_sts: ReconciliationStatus8Choice,
}

impl ReconciliationReport8 {
	pub fn validate(&self) -> bool {
		if let Some(ref tech_rcrd_id_value) = self.tech_rcrd_id { if !tech_rcrd_id_value.validate() { return false; } }
		if !self.tx_id.validate() { return false }
		if !self.rcncltn_sts.validate() { return false }
		return true
	}
}


// ReconciliationResult10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReconciliationResult10 {
	#[serde(rename = "CtrPty1")]
	pub ctr_pty1: OrganisationIdentification15Choice,
	#[serde(rename = "CtrPty2")]
	pub ctr_pty2: OrganisationIdentification15Choice,
	#[serde(rename = "MtchgCrit")]
	pub mtchg_crit: MatchingCriteria10,
}

impl ReconciliationResult10 {
	pub fn validate(&self) -> bool {
		if !self.ctr_pty1.validate() { return false }
		if !self.ctr_pty2.validate() { return false }
		if !self.mtchg_crit.validate() { return false }
		return true
	}
}


// ReconciliationStatus8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReconciliationStatus8Choice {
	#[serde(rename = "NoRcncltnReqrd", skip_serializing_if = "Option::is_none")]
	pub no_rcncltn_reqrd: Option<NoReasonCode>,
	#[serde(rename = "RptgData", skip_serializing_if = "Option::is_none")]
	pub rptg_data: Option<ReconciliationMatchedStatus9Choice>,
}

impl ReconciliationStatus8Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref no_rcncltn_reqrd_value) = self.no_rcncltn_reqrd { if !no_rcncltn_reqrd_value.validate() { return false; } }
		if let Some(ref rptg_data_value) = self.rptg_data { if !rptg_data_value.validate() { return false; } }
		return true
	}
}


// RepoTerminationOption2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RepoTerminationOption2Code {
	#[default]
	#[serde(rename = "EGRN")]
	CodeEGRN,
	#[serde(rename = "EGAE")]
	CodeEGAE,
	#[serde(rename = "ETSB")]
	CodeETSB,
	#[serde(rename = "NOAP")]
	CodeNOAP,
}

impl RepoTerminationOption2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ReportPeriodActivity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ReportPeriodActivity1Code {
	#[default]
	#[serde(rename = "NOTX")]
	CodeNOTX,
}

impl ReportPeriodActivity1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// SecuritiesFinancingReportingReconciliationStatusAdviceV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesFinancingReportingReconciliationStatusAdviceV02 {
	#[serde(rename = "RcncltnData")]
	pub rcncltn_data: TradeData34Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl SecuritiesFinancingReportingReconciliationStatusAdviceV02 {
	pub fn validate(&self) -> bool {
		if !self.rcncltn_data.validate() { return false }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
	}
}


// SecuritiesLendingType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesLendingType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalSecuritiesLendingType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl SecuritiesLendingType3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// SecuritiesTransactionPrice19Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice19Choice {
	#[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
	pub mntry_val: Option<AmountAndDirection107>,
	#[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
	pub unit: Option<f64>,
	#[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
	pub pctg: Option<f64>,
	#[serde(rename = "Yld", skip_serializing_if = "Option::is_none")]
	pub yld: Option<f64>,
	#[serde(rename = "Dcml", skip_serializing_if = "Option::is_none")]
	pub dcml: Option<f64>,
	#[serde(rename = "PdgPric", skip_serializing_if = "Option::is_none")]
	pub pdg_pric: Option<PriceStatus1Code>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<SecuritiesTransactionPrice5>,
}

impl SecuritiesTransactionPrice19Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref mntry_val_value) = self.mntry_val { if !mntry_val_value.validate() { return false; } }
		if let Some(ref pdg_pric_value) = self.pdg_pric { if !pdg_pric_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
	}
}


// SecuritiesTransactionPrice5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice5 {
	#[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
	pub val: Option<f64>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<Max35Text>,
}

impl SecuritiesTransactionPrice5 {
	pub fn validate(&self) -> bool {
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		return true
	}
}


// Security48 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Security48 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<CompareISINIdentifier4>,
	#[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
	pub clssfctn_tp: Option<CompareCFIIdentifier3>,
	#[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
	pub qty: Option<CompareDecimalNumber3>,
	#[serde(rename = "NmnlVal", skip_serializing_if = "Option::is_none")]
	pub nmnl_val: Option<CompareAmountAndDirection2>,
	#[serde(rename = "Qlty", skip_serializing_if = "Option::is_none")]
	pub qlty: Option<CompareCollateralQualityType3>,
	#[serde(rename = "Mtrty", skip_serializing_if = "Option::is_none")]
	pub mtrty: Option<CompareDate3>,
	#[serde(rename = "IssrId", skip_serializing_if = "Option::is_none")]
	pub issr_id: Option<CompareOrganisationIdentification6>,
	#[serde(rename = "IssrCtry", skip_serializing_if = "Option::is_none")]
	pub issr_ctry: Option<CompareCountryCode3>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<Vec<CompareSecuritiesLendingType3>>,
	#[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
	pub unit_pric: Option<CompareUnitPrice6>,
	#[serde(rename = "ExclsvArrgmnt", skip_serializing_if = "Option::is_none")]
	pub exclsv_arrgmnt: Option<CompareTrueFalseIndicator3>,
	#[serde(rename = "MktVal", skip_serializing_if = "Option::is_none")]
	pub mkt_val: Option<CompareAmountAndDirection2>,
	#[serde(rename = "AvlblForCollReuse", skip_serializing_if = "Option::is_none")]
	pub avlbl_for_coll_reuse: Option<CompareTrueFalseIndicator3>,
	#[serde(rename = "HrcutOrMrgn", skip_serializing_if = "Option::is_none")]
	pub hrcut_or_mrgn: Option<ComparePercentageRate3>,
}

impl Security48 {
	pub fn validate(&self) -> bool {
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
		if let Some(ref clssfctn_tp_value) = self.clssfctn_tp { if !clssfctn_tp_value.validate() { return false; } }
		if let Some(ref qty_value) = self.qty { if !qty_value.validate() { return false; } }
		if let Some(ref nmnl_val_value) = self.nmnl_val { if !nmnl_val_value.validate() { return false; } }
		if let Some(ref qlty_value) = self.qlty { if !qlty_value.validate() { return false; } }
		if let Some(ref mtrty_value) = self.mtrty { if !mtrty_value.validate() { return false; } }
		if let Some(ref issr_id_value) = self.issr_id { if !issr_id_value.validate() { return false; } }
		if let Some(ref issr_ctry_value) = self.issr_ctry { if !issr_ctry_value.validate() { return false; } }
		if let Some(ref tp_vec) = self.tp { for item in tp_vec { if !item.validate() { return false; } } }
		if let Some(ref unit_pric_value) = self.unit_pric { if !unit_pric_value.validate() { return false; } }
		if let Some(ref exclsv_arrgmnt_value) = self.exclsv_arrgmnt { if !exclsv_arrgmnt_value.validate() { return false; } }
		if let Some(ref mkt_val_value) = self.mkt_val { if !mkt_val_value.validate() { return false; } }
		if let Some(ref avlbl_for_coll_reuse_value) = self.avlbl_for_coll_reuse { if !avlbl_for_coll_reuse_value.validate() { return false; } }
		if let Some(ref hrcut_or_mrgn_value) = self.hrcut_or_mrgn { if !hrcut_or_mrgn_value.validate() { return false; } }
		return true
	}
}


// SecurityCommodity7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityCommodity7Choice {
	#[serde(rename = "Scty", skip_serializing_if = "Option::is_none")]
	pub scty: Option<Vec<Security48>>,
	#[serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none")]
	pub cmmdty: Option<Vec<Commodity42>>,
}

impl SecurityCommodity7Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref scty_vec) = self.scty { for item in scty_vec { if !item.validate() { return false; } } }
		if let Some(ref cmmdty_vec) = self.cmmdty { for item in cmmdty_vec { if !item.validate() { return false; } } }
		return true
	}
}


// SecurityCommodityCash4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityCommodityCash4 {
	#[serde(rename = "Scty", skip_serializing_if = "Option::is_none")]
	pub scty: Option<Vec<Security48>>,
	#[serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none")]
	pub cmmdty: Option<Vec<Commodity42>>,
	#[serde(rename = "Csh", skip_serializing_if = "Option::is_none")]
	pub csh: Option<Vec<CashCompare3>>,
}

impl SecurityCommodityCash4 {
	pub fn validate(&self) -> bool {
		if let Some(ref scty_vec) = self.scty { for item in scty_vec { if !item.validate() { return false; } } }
		if let Some(ref cmmdty_vec) = self.cmmdty { for item in cmmdty_vec { if !item.validate() { return false; } } }
		if let Some(ref csh_vec) = self.csh { for item in csh_vec { if !item.validate() { return false; } } }
		return true
	}
}


// SecurityIdentification26Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification26Choice {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ISINOct2015Identifier>,
	#[serde(rename = "NotAvlbl", skip_serializing_if = "Option::is_none")]
	pub not_avlbl: Option<NotAvailable1Code>,
}

impl SecurityIdentification26Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
		if let Some(ref not_avlbl_value) = self.not_avlbl { if !not_avlbl_value.validate() { return false; } }
		return true
	}
}


// SpecialCollateral1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SpecialCollateral1Code {
	#[default]
	#[serde(rename = "GENE")]
	CodeGENE,
	#[serde(rename = "SPEC")]
	CodeSPEC,
}

impl SpecialCollateral1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
	pub plc_and_nm: Option<Max350Text>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}

impl SupplementaryData1 {
	pub fn validate(&self) -> bool {
		if let Some(ref plc_and_nm_value) = self.plc_and_nm { if !plc_and_nm_value.validate() { return false; } }
		if !self.envlp.validate() { return false }
		return true
	}
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// TradeData28 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeData28 {
	#[serde(rename = "PairgRcncltnSts", skip_serializing_if = "Option::is_none")]
	pub pairg_rcncltn_sts: Option<Vec<NumberOfReportsPerStatus4>>,
	#[serde(rename = "RcncltnRpt")]
	pub rcncltn_rpt: Vec<ReconciliationReport8>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl TradeData28 {
	pub fn validate(&self) -> bool {
		if let Some(ref pairg_rcncltn_sts_vec) = self.pairg_rcncltn_sts { for item in pairg_rcncltn_sts_vec { if !item.validate() { return false; } } }
		for item in &self.rcncltn_rpt { if !item.validate() { return false; } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
	}
}


// TradeData34Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeData34Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[serde(rename = "Rpt", skip_serializing_if = "Option::is_none")]
	pub rpt: Option<Vec<TradeData28>>,
}

impl TradeData34Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref data_set_actn_value) = self.data_set_actn { if !data_set_actn_value.validate() { return false; } }
		if let Some(ref rpt_vec) = self.rpt { for item in rpt_vec { if !item.validate() { return false; } } }
		return true
	}
}


// TradeTransactionIdentification19 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeTransactionIdentification19 {
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: OrganisationIdentification15Choice,
	#[serde(rename = "OthrCtrPty")]
	pub othr_ctr_pty: PartyIdentification236Choice,
	#[serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none")]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "UnqTradIdr", skip_serializing_if = "Option::is_none")]
	pub unq_trad_idr: Option<Max52Text>,
	#[serde(rename = "MstrAgrmt", skip_serializing_if = "Option::is_none")]
	pub mstr_agrmt: Option<MasterAgreement7>,
	#[serde(rename = "AgtLndr", skip_serializing_if = "Option::is_none")]
	pub agt_lndr: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "TrptyAgt", skip_serializing_if = "Option::is_none")]
	pub trpty_agt: Option<OrganisationIdentification15Choice>,
}

impl TradeTransactionIdentification19 {
	pub fn validate(&self) -> bool {
		if !self.rptg_ctr_pty.validate() { return false }
		if !self.othr_ctr_pty.validate() { return false }
		if let Some(ref ntty_rspnsbl_for_rpt_value) = self.ntty_rspnsbl_for_rpt { if !ntty_rspnsbl_for_rpt_value.validate() { return false; } }
		if let Some(ref unq_trad_idr_value) = self.unq_trad_idr { if !unq_trad_idr_value.validate() { return false; } }
		if let Some(ref mstr_agrmt_value) = self.mstr_agrmt { if !mstr_agrmt_value.validate() { return false; } }
		if let Some(ref agt_lndr_value) = self.agt_lndr { if !agt_lndr_value.validate() { return false; } }
		if let Some(ref trpty_agt_value) = self.trpty_agt { if !trpty_agt_value.validate() { return false; } }
		return true
	}
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}

impl TrueFalseIndicator {
	pub fn validate(&self) -> bool {
		return true
	}
}


// UnitOfMeasure11Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum UnitOfMeasure11Code {
	#[default]
	#[serde(rename = "ALOW")]
	CodeALOW,
	#[serde(rename = "ACCY")]
	CodeACCY,
	#[serde(rename = "BARL")]
	CodeBARL,
	#[serde(rename = "BCUF")]
	CodeBCUF,
	#[serde(rename = "BDFT")]
	CodeBDFT,
	#[serde(rename = "BUSL")]
	CodeBUSL,
	#[serde(rename = "CEER")]
	CodeCEER,
	#[serde(rename = "CLRT")]
	CodeCLRT,
	#[serde(rename = "KILO")]
	CodeKILO,
	#[serde(rename = "PIEC")]
	CodePIEC,
	#[serde(rename = "TONS")]
	CodeTONS,
	#[serde(rename = "METR")]
	CodeMETR,
	#[serde(rename = "INCH")]
	CodeINCH,
	#[serde(rename = "YARD")]
	CodeYARD,
	#[serde(rename = "GBGA")]
	CodeGBGA,
	#[serde(rename = "GRAM")]
	CodeGRAM,
	#[serde(rename = "CMET")]
	CodeCMET,
	#[serde(rename = "SMET")]
	CodeSMET,
	#[serde(rename = "FOOT")]
	CodeFOOT,
	#[serde(rename = "MILE")]
	CodeMILE,
	#[serde(rename = "SQIN")]
	CodeSQIN,
	#[serde(rename = "SQFO")]
	CodeSQFO,
	#[serde(rename = "SQMI")]
	CodeSQMI,
	#[serde(rename = "GBOU")]
	CodeGBOU,
	#[serde(rename = "USOU")]
	CodeUSOU,
	#[serde(rename = "GBPI")]
	CodeGBPI,
	#[serde(rename = "USPI")]
	CodeUSPI,
	#[serde(rename = "GBQA")]
	CodeGBQA,
	#[serde(rename = "USGA")]
	CodeUSGA,
	#[serde(rename = "MMET")]
	CodeMMET,
	#[serde(rename = "KMET")]
	CodeKMET,
	#[serde(rename = "SQYA")]
	CodeSQYA,
	#[serde(rename = "ACRE")]
	CodeACRE,
	#[serde(rename = "ARES")]
	CodeARES,
	#[serde(rename = "SMIL")]
	CodeSMIL,
	#[serde(rename = "SCMT")]
	CodeSCMT,
	#[serde(rename = "HECT")]
	CodeHECT,
	#[serde(rename = "SQKI")]
	CodeSQKI,
	#[serde(rename = "MILI")]
	CodeMILI,
	#[serde(rename = "CELI")]
	CodeCELI,
	#[serde(rename = "LITR")]
	CodeLITR,
	#[serde(rename = "PUND")]
	CodePUND,
	#[serde(rename = "CBME")]
	CodeCBME,
	#[serde(rename = "DAYS")]
	CodeDAYS,
	#[serde(rename = "DMET")]
	CodeDMET,
	#[serde(rename = "ENVC")]
	CodeENVC,
	#[serde(rename = "ENVO")]
	CodeENVO,
	#[serde(rename = "HUWG")]
	CodeHUWG,
	#[serde(rename = "KWDC")]
	CodeKWDC,
	#[serde(rename = "KWHO")]
	CodeKWHO,
	#[serde(rename = "KWHC")]
	CodeKWHC,
	#[serde(rename = "KMOC")]
	CodeKMOC,
	#[serde(rename = "KWMC")]
	CodeKWMC,
	#[serde(rename = "KWYC")]
	CodeKWYC,
	#[serde(rename = "MWDC")]
	CodeMWDC,
	#[serde(rename = "MWHO")]
	CodeMWHO,
	#[serde(rename = "MWHC")]
	CodeMWHC,
	#[serde(rename = "MWMC")]
	CodeMWMC,
	#[serde(rename = "MMOC")]
	CodeMMOC,
	#[serde(rename = "MWYC")]
	CodeMWYC,
	#[serde(rename = "TONE")]
	CodeTONE,
	#[serde(rename = "MIBA")]
	CodeMIBA,
	#[serde(rename = "MBTU")]
	CodeMBTU,
	#[serde(rename = "OZTR")]
	CodeOZTR,
	#[serde(rename = "UCWT")]
	CodeUCWT,
	#[serde(rename = "IPNT")]
	CodeIPNT,
	#[serde(rename = "PWRD")]
	CodePWRD,
	#[serde(rename = "DGEU")]
	CodeDGEU,
	#[serde(rename = "TOCD")]
	CodeTOCD,
	#[serde(rename = "GGEU")]
	CodeGGEU,
	#[serde(rename = "USQA")]
	CodeUSQA,
}

impl UnitOfMeasure11Code {
	pub fn validate(&self) -> bool {
		return true
	}
}
