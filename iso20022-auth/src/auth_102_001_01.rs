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


// ActiveCurrencyAnd13DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyAnd13DecimalAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_currency_and13_decimal_amount_simple_type: f64,
}

impl ActiveCurrencyAnd13DecimalAmountSimpleType {
	pub fn validate(&self) -> bool {
		if self.active_currency_and13_decimal_amount_simple_type < 0.000000 {
			return false
		}
		return true
	}
}


// ActiveCurrencyAnd13DecimalAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd13DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveCurrencyAnd13DecimalAmount {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "$value")]
	pub active_currency_code: String,
}

impl ActiveCurrencyCode {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.active_currency_code) {
			return false
		}
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


// AgriculturalCommodityGrain1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityGrain1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType5Code,
	#[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
	pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType15Code>,
}

impl AgriculturalCommodityGrain1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if !addtl_sub_pdct_value.validate() { return false; } }
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


// AgriculturalCommodityOliveOil1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityOliveOil1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType3Code,
	#[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
	pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType4Code>,
}

impl AgriculturalCommodityOliveOil1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if !addtl_sub_pdct_value.validate() { return false; } }
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


// AmountAndDirection61 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection61 {
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAnd13DecimalAmount,
	#[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
	pub sgn: Option<bool>,
}

impl AmountAndDirection61 {
	pub fn validate(&self) -> bool {
		if !self.amt.validate() { return false }
		return true
	}
}


// AssetClass2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClass2 {
	#[serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none")]
	pub cmmdty: Option<DerivativeCommodity2>,
	#[serde(rename = "Intrst", skip_serializing_if = "Option::is_none")]
	pub intrst: Option<DerivativeInterest3>,
	#[serde(rename = "FX", skip_serializing_if = "Option::is_none")]
	pub fx: Option<DerivativeForeignExchange3>,
}

impl AssetClass2 {
	pub fn validate(&self) -> bool {
		if let Some(ref cmmdty_value) = self.cmmdty { if !cmmdty_value.validate() { return false; } }
		if let Some(ref intrst_value) = self.intrst { if !intrst_value.validate() { return false; } }
		if let Some(ref fx_value) = self.fx { if !fx_value.validate() { return false; } }
		return true
	}
}


// AssetClassCommodity3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodity3Choice {
	#[serde(rename = "Agrcltrl", skip_serializing_if = "Option::is_none")]
	pub agrcltrl: Option<AssetClassCommodityAgricultural1Choice>,
	#[serde(rename = "Nrgy", skip_serializing_if = "Option::is_none")]
	pub nrgy: Option<AssetClassCommodityEnergy1Choice>,
	#[serde(rename = "Envttl", skip_serializing_if = "Option::is_none")]
	pub envttl: Option<AssetClassCommodityEnvironmental1Choice>,
	#[serde(rename = "Frtlzr", skip_serializing_if = "Option::is_none")]
	pub frtlzr: Option<AssetClassCommodityFertilizer1Choice>,
	#[serde(rename = "Frght", skip_serializing_if = "Option::is_none")]
	pub frght: Option<AssetClassCommodityFreight1Choice>,
	#[serde(rename = "IndstrlPdct", skip_serializing_if = "Option::is_none")]
	pub indstrl_pdct: Option<AssetClassCommodityIndustrialProduct1Choice>,
	#[serde(rename = "Metl", skip_serializing_if = "Option::is_none")]
	pub metl: Option<AssetClassCommodityMetal1Choice>,
	#[serde(rename = "OthrC10", skip_serializing_if = "Option::is_none")]
	pub othr_c10: Option<AssetClassCommodityOtherC102Choice>,
	#[serde(rename = "Ppr", skip_serializing_if = "Option::is_none")]
	pub ppr: Option<AssetClassCommodityPaper1Choice>,
	#[serde(rename = "Plprpln", skip_serializing_if = "Option::is_none")]
	pub plprpln: Option<AssetClassCommodityPolypropylene1Choice>,
	#[serde(rename = "Infltn", skip_serializing_if = "Option::is_none")]
	pub infltn: Option<AssetClassCommodityInflation1>,
	#[serde(rename = "MultiCmmdtyExtc", skip_serializing_if = "Option::is_none")]
	pub multi_cmmdty_extc: Option<AssetClassCommodityMultiCommodityExotic1>,
	#[serde(rename = "OffclEcnmcSttstcs", skip_serializing_if = "Option::is_none")]
	pub offcl_ecnmc_sttstcs: Option<AssetClassCommodityOfficialEconomicStatistics1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<AssetClassCommodityOther1>,
}

impl AssetClassCommodity3Choice {
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


// AssetClassCommodityAgricultural1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityAgricultural1Choice {
	#[serde(rename = "GrnOilSeed", skip_serializing_if = "Option::is_none")]
	pub grn_oil_seed: Option<AgriculturalCommodityOilSeed1>,
	#[serde(rename = "Soft", skip_serializing_if = "Option::is_none")]
	pub soft: Option<AgriculturalCommoditySoft1>,
	#[serde(rename = "Ptt", skip_serializing_if = "Option::is_none")]
	pub ptt: Option<AgriculturalCommodityPotato1>,
	#[serde(rename = "OlvOil", skip_serializing_if = "Option::is_none")]
	pub olv_oil: Option<AgriculturalCommodityOliveOil1>,
	#[serde(rename = "Dairy", skip_serializing_if = "Option::is_none")]
	pub dairy: Option<AgriculturalCommodityDairy1>,
	#[serde(rename = "Frstry", skip_serializing_if = "Option::is_none")]
	pub frstry: Option<AgriculturalCommodityForestry1>,
	#[serde(rename = "Sfd", skip_serializing_if = "Option::is_none")]
	pub sfd: Option<AgriculturalCommoditySeafood1>,
	#[serde(rename = "LiveStock", skip_serializing_if = "Option::is_none")]
	pub live_stock: Option<AgriculturalCommodityLiveStock1>,
	#[serde(rename = "Grn", skip_serializing_if = "Option::is_none")]
	pub grn: Option<AgriculturalCommodityGrain1>,
}

impl AssetClassCommodityAgricultural1Choice {
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
		return true
	}
}


// AssetClassCommodityEnergy1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityEnergy1Choice {
	#[serde(rename = "Elctrcty", skip_serializing_if = "Option::is_none")]
	pub elctrcty: Option<EnergyCommodityElectricity1>,
	#[serde(rename = "NtrlGas", skip_serializing_if = "Option::is_none")]
	pub ntrl_gas: Option<EnergyCommodityNaturalGas1>,
	#[serde(rename = "Oil", skip_serializing_if = "Option::is_none")]
	pub oil: Option<EnergyCommodityOil1>,
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
}

impl AssetClassCommodityEnergy1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref elctrcty_value) = self.elctrcty { if !elctrcty_value.validate() { return false; } }
		if let Some(ref ntrl_gas_value) = self.ntrl_gas { if !ntrl_gas_value.validate() { return false; } }
		if let Some(ref oil_value) = self.oil { if !oil_value.validate() { return false; } }
		if let Some(ref coal_value) = self.coal { if !coal_value.validate() { return false; } }
		if let Some(ref intr_nrgy_value) = self.intr_nrgy { if !intr_nrgy_value.validate() { return false; } }
		if let Some(ref rnwbl_nrgy_value) = self.rnwbl_nrgy { if !rnwbl_nrgy_value.validate() { return false; } }
		if let Some(ref lght_end_value) = self.lght_end { if !lght_end_value.validate() { return false; } }
		if let Some(ref dstllts_value) = self.dstllts { if !dstllts_value.validate() { return false; } }
		return true
	}
}


// AssetClassCommodityEnvironmental1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityEnvironmental1Choice {
	#[serde(rename = "Emssns", skip_serializing_if = "Option::is_none")]
	pub emssns: Option<EnvironmentalCommodityEmission1>,
	#[serde(rename = "Wthr", skip_serializing_if = "Option::is_none")]
	pub wthr: Option<EnvironmentalCommodityWeather1>,
	#[serde(rename = "CrbnRltd", skip_serializing_if = "Option::is_none")]
	pub crbn_rltd: Option<EnvironmentalCommodityCarbonRelated1>,
}

impl AssetClassCommodityEnvironmental1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref emssns_value) = self.emssns { if !emssns_value.validate() { return false; } }
		if let Some(ref wthr_value) = self.wthr { if !wthr_value.validate() { return false; } }
		if let Some(ref crbn_rltd_value) = self.crbn_rltd { if !crbn_rltd_value.validate() { return false; } }
		return true
	}
}


// AssetClassCommodityFertilizer1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityFertilizer1Choice {
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
}

impl AssetClassCommodityFertilizer1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref ammn_value) = self.ammn { if !ammn_value.validate() { return false; } }
		if let Some(ref dmmnm_phspht_value) = self.dmmnm_phspht { if !dmmnm_phspht_value.validate() { return false; } }
		if let Some(ref ptsh_value) = self.ptsh { if !ptsh_value.validate() { return false; } }
		if let Some(ref slphr_value) = self.slphr { if !slphr_value.validate() { return false; } }
		if let Some(ref urea_value) = self.urea { if !urea_value.validate() { return false; } }
		if let Some(ref urea_and_ammnm_ntrt_value) = self.urea_and_ammnm_ntrt { if !urea_and_ammnm_ntrt_value.validate() { return false; } }
		return true
	}
}


// AssetClassCommodityFreight1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityFreight1Choice {
	#[serde(rename = "Dry", skip_serializing_if = "Option::is_none")]
	pub dry: Option<FreightCommodityDry1>,
	#[serde(rename = "Wet", skip_serializing_if = "Option::is_none")]
	pub wet: Option<FreightCommodityWet1>,
	#[serde(rename = "CntnrShip", skip_serializing_if = "Option::is_none")]
	pub cntnr_ship: Option<FreightCommodityContainerShip1>,
}

impl AssetClassCommodityFreight1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref dry_value) = self.dry { if !dry_value.validate() { return false; } }
		if let Some(ref wet_value) = self.wet { if !wet_value.validate() { return false; } }
		if let Some(ref cntnr_ship_value) = self.cntnr_ship { if !cntnr_ship_value.validate() { return false; } }
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


// AssetClassCommodityPaper1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityPaper1Choice {
	#[serde(rename = "CntnrBrd", skip_serializing_if = "Option::is_none")]
	pub cntnr_brd: Option<PaperCommodityContainerBoard1>,
	#[serde(rename = "Nwsprnt", skip_serializing_if = "Option::is_none")]
	pub nwsprnt: Option<PaperCommodityNewsprint1>,
	#[serde(rename = "Pulp", skip_serializing_if = "Option::is_none")]
	pub pulp: Option<PaperCommodityPulp1>,
	#[serde(rename = "RcvrdPpr", skip_serializing_if = "Option::is_none")]
	pub rcvrd_ppr: Option<PaperCommodityRecoveredPaper1>,
}

impl AssetClassCommodityPaper1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cntnr_brd_value) = self.cntnr_brd { if !cntnr_brd_value.validate() { return false; } }
		if let Some(ref nwsprnt_value) = self.nwsprnt { if !nwsprnt_value.validate() { return false; } }
		if let Some(ref pulp_value) = self.pulp { if !pulp_value.validate() { return false; } }
		if let Some(ref rcvrd_ppr_value) = self.rcvrd_ppr { if !rcvrd_ppr_value.validate() { return false; } }
		return true
	}
}


// AssetClassCommodityPolypropylene1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityPolypropylene1Choice {
	#[serde(rename = "Plstc", skip_serializing_if = "Option::is_none")]
	pub plstc: Option<PolypropyleneCommodityPlastic1>,
}

impl AssetClassCommodityPolypropylene1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref plstc_value) = self.plstc { if !plstc_value.validate() { return false; } }
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


// AssetClassDetailedSubProductType12Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType12Code {
	#[default]
	#[serde(rename = "TNKR")]
	CodeTNKR,
}

impl AssetClassDetailedSubProductType12Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassDetailedSubProductType14Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType14Code {
	#[default]
	#[serde(rename = "DBCR")]
	CodeDBCR,
}

impl AssetClassDetailedSubProductType14Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassDetailedSubProductType15Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType15Code {
	#[default]
	#[serde(rename = "MWHT")]
	CodeMWHT,
}

impl AssetClassDetailedSubProductType15Code {
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


// AssetClassDetailedSubProductType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType4Code {
	#[default]
	#[serde(rename = "LAMP")]
	CodeLAMP,
}

impl AssetClassDetailedSubProductType4Code {
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


// AssetClassDetailedSubProductType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType6Code {
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
}

impl AssetClassDetailedSubProductType6Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetClassDetailedSubProductType7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType7Code {
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
}

impl AssetClassDetailedSubProductType7Code {
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


// AssetClassTransactionType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassTransactionType1Code {
	#[default]
	#[serde(rename = "CRCK")]
	CodeCRCK,
	#[serde(rename = "DIFF")]
	CodeDIFF,
	#[serde(rename = "FUTR")]
	CodeFUTR,
	#[serde(rename = "MINI")]
	CodeMINI,
	#[serde(rename = "OPTN")]
	CodeOPTN,
	#[serde(rename = "OTCT")]
	CodeOTCT,
	#[serde(rename = "ORIT")]
	CodeORIT,
	#[serde(rename = "SWAP")]
	CodeSWAP,
	#[serde(rename = "TAPO")]
	CodeTAPO,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}

impl AssetClassTransactionType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetFXSubProductType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetFXSubProductType1Code {
	#[default]
	#[serde(rename = "FXCR")]
	CodeFXCR,
	#[serde(rename = "FXEM")]
	CodeFXEM,
	#[serde(rename = "FXMJ")]
	CodeFXMJ,
}

impl AssetFXSubProductType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AssetPriceType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetPriceType1Code {
	#[default]
	#[serde(rename = "ARGM")]
	CodeARGM,
	#[serde(rename = "BLTC")]
	CodeBLTC,
	#[serde(rename = "EXOF")]
	CodeEXOF,
	#[serde(rename = "GBCL")]
	CodeGBCL,
	#[serde(rename = "IHSM")]
	CodeIHSM,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "PLAT")]
	CodePLAT,
}

impl AssetPriceType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// BenchmarkCurveName2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BenchmarkCurveName2Code {
	#[default]
	#[serde(rename = "WIBO")]
	CodeWIBO,
	#[serde(rename = "TREA")]
	CodeTREA,
	#[serde(rename = "TIBO")]
	CodeTIBO,
	#[serde(rename = "TLBO")]
	CodeTLBO,
	#[serde(rename = "SWAP")]
	CodeSWAP,
	#[serde(rename = "STBO")]
	CodeSTBO,
	#[serde(rename = "PRBO")]
	CodePRBO,
	#[serde(rename = "PFAN")]
	CodePFAN,
	#[serde(rename = "NIBO")]
	CodeNIBO,
	#[serde(rename = "MAAA")]
	CodeMAAA,
	#[serde(rename = "MOSP")]
	CodeMOSP,
	#[serde(rename = "LIBO")]
	CodeLIBO,
	#[serde(rename = "LIBI")]
	CodeLIBI,
	#[serde(rename = "JIBA")]
	CodeJIBA,
	#[serde(rename = "ISDA")]
	CodeISDA,
	#[serde(rename = "GCFR")]
	CodeGCFR,
	#[serde(rename = "FUSW")]
	CodeFUSW,
	#[serde(rename = "EUCH")]
	CodeEUCH,
	#[serde(rename = "EUUS")]
	CodeEUUS,
	#[serde(rename = "EURI")]
	CodeEURI,
	#[serde(rename = "EONS")]
	CodeEONS,
	#[serde(rename = "EONA")]
	CodeEONA,
	#[serde(rename = "CIBO")]
	CodeCIBO,
	#[serde(rename = "CDOR")]
	CodeCDOR,
	#[serde(rename = "BUBO")]
	CodeBUBO,
	#[serde(rename = "BBSW")]
	CodeBBSW,
}

impl BenchmarkCurveName2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// BenchmarkCurveName5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkCurveName5Choice {
	#[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
	pub indx: Option<BenchmarkCurveName2Code>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max25Text>,
}

impl BenchmarkCurveName5Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref indx_value) = self.indx { if !indx_value.validate() { return false; } }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		return true
	}
}


// BenchmarkCurveName6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkCurveName6Choice {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
	#[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
	pub indx: Option<BenchmarkCurveName2Code>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max25Text>,
}

impl BenchmarkCurveName6Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref isin_value) = self.isin { if !isin_value.validate() { return false; } }
		if let Some(ref indx_value) = self.indx { if !indx_value.validate() { return false; } }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
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


// DebtInstrument2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DebtInstrument2 {
	#[serde(rename = "TtlIssdNmnlAmt")]
	pub ttl_issd_nmnl_amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "NmnlValPerUnit")]
	pub nmnl_val_per_unit: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "IntrstRate")]
	pub intrst_rate: InterestRate6Choice,
	#[serde(rename = "DebtSnrty", skip_serializing_if = "Option::is_none")]
	pub debt_snrty: Option<DebtInstrumentSeniorityType1Code>,
}

impl DebtInstrument2 {
	pub fn validate(&self) -> bool {
		if !self.ttl_issd_nmnl_amt.validate() { return false }
		if !self.nmnl_val_per_unit.validate() { return false }
		if !self.intrst_rate.validate() { return false }
		if let Some(ref debt_snrty_value) = self.debt_snrty { if !debt_snrty_value.validate() { return false; } }
		return true
	}
}


// DebtInstrumentSeniorityType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum DebtInstrumentSeniorityType1Code {
	#[default]
	#[serde(rename = "SBOD")]
	CodeSBOD,
	#[serde(rename = "SNDB")]
	CodeSNDB,
	#[serde(rename = "MZZD")]
	CodeMZZD,
	#[serde(rename = "JUND")]
	CodeJUND,
}

impl DebtInstrumentSeniorityType1Code {
	pub fn validate(&self) -> bool {
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


// DerivativeCommodity2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativeCommodity2 {
	#[serde(rename = "Pdct")]
	pub pdct: AssetClassCommodity3Choice,
	#[serde(rename = "TxTp", skip_serializing_if = "Option::is_none")]
	pub tx_tp: Option<AssetClassTransactionType1Code>,
	#[serde(rename = "FnlPricTp", skip_serializing_if = "Option::is_none")]
	pub fnl_pric_tp: Option<AssetPriceType1Code>,
}

impl DerivativeCommodity2 {
	pub fn validate(&self) -> bool {
		if !self.pdct.validate() { return false }
		if let Some(ref tx_tp_value) = self.tx_tp { if !tx_tp_value.validate() { return false; } }
		if let Some(ref fnl_pric_tp_value) = self.fnl_pric_tp { if !fnl_pric_tp_value.validate() { return false; } }
		return true
	}
}


// DerivativeForeignExchange3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativeForeignExchange3 {
	#[serde(rename = "FxTp", skip_serializing_if = "Option::is_none")]
	pub fx_tp: Option<AssetFXSubProductType1Code>,
	#[serde(rename = "OthrNtnlCcy", skip_serializing_if = "Option::is_none")]
	pub othr_ntnl_ccy: Option<ActiveOrHistoricCurrencyCode>,
}

impl DerivativeForeignExchange3 {
	pub fn validate(&self) -> bool {
		if let Some(ref fx_tp_value) = self.fx_tp { if !fx_tp_value.validate() { return false; } }
		if let Some(ref othr_ntnl_ccy_value) = self.othr_ntnl_ccy { if !othr_ntnl_ccy_value.validate() { return false; } }
		return true
	}
}


// DerivativeInstrument5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativeInstrument5 {
	#[serde(rename = "XpryDt", skip_serializing_if = "Option::is_none")]
	pub xpry_dt: Option<String>,
	#[serde(rename = "PricMltplr", skip_serializing_if = "Option::is_none")]
	pub pric_mltplr: Option<f64>,
	#[serde(rename = "UndrlygInstrm", skip_serializing_if = "Option::is_none")]
	pub undrlyg_instrm: Option<FinancialInstrumentIdentification5Choice>,
	#[serde(rename = "OptnTp", skip_serializing_if = "Option::is_none")]
	pub optn_tp: Option<OptionType2Code>,
	#[serde(rename = "StrkPric", skip_serializing_if = "Option::is_none")]
	pub strk_pric: Option<SecuritiesTransactionPrice4Choice>,
	#[serde(rename = "OptnExrcStyle", skip_serializing_if = "Option::is_none")]
	pub optn_exrc_style: Option<OptionStyle7Code>,
	#[serde(rename = "DlvryTp", skip_serializing_if = "Option::is_none")]
	pub dlvry_tp: Option<PhysicalTransferType4Code>,
	#[serde(rename = "AsstClssSpcfcAttrbts", skip_serializing_if = "Option::is_none")]
	pub asst_clss_spcfc_attrbts: Option<AssetClass2>,
}

impl DerivativeInstrument5 {
	pub fn validate(&self) -> bool {
		if let Some(ref undrlyg_instrm_value) = self.undrlyg_instrm { if !undrlyg_instrm_value.validate() { return false; } }
		if let Some(ref optn_tp_value) = self.optn_tp { if !optn_tp_value.validate() { return false; } }
		if let Some(ref strk_pric_value) = self.strk_pric { if !strk_pric_value.validate() { return false; } }
		if let Some(ref optn_exrc_style_value) = self.optn_exrc_style { if !optn_exrc_style_value.validate() { return false; } }
		if let Some(ref dlvry_tp_value) = self.dlvry_tp { if !dlvry_tp_value.validate() { return false; } }
		if let Some(ref asst_clss_spcfc_attrbts_value) = self.asst_clss_spcfc_attrbts { if !asst_clss_spcfc_attrbts_value.validate() { return false; } }
		return true
	}
}


// DerivativeInterest3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativeInterest3 {
	#[serde(rename = "IntrstRate")]
	pub intrst_rate: FloatingInterestRate8,
	#[serde(rename = "FrstLegIntrstRate", skip_serializing_if = "Option::is_none")]
	pub frst_leg_intrst_rate: Option<InterestRate8Choice>,
	#[serde(rename = "OthrNtnlCcy", skip_serializing_if = "Option::is_none")]
	pub othr_ntnl_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "OthrLegIntrstRate", skip_serializing_if = "Option::is_none")]
	pub othr_leg_intrst_rate: Option<InterestRate8Choice>,
}

impl DerivativeInterest3 {
	pub fn validate(&self) -> bool {
		if !self.intrst_rate.validate() { return false }
		if let Some(ref frst_leg_intrst_rate_value) = self.frst_leg_intrst_rate { if !frst_leg_intrst_rate_value.validate() { return false; } }
		if let Some(ref othr_ntnl_ccy_value) = self.othr_ntnl_ccy { if !othr_ntnl_ccy_value.validate() { return false; } }
		if let Some(ref othr_leg_intrst_rate_value) = self.othr_leg_intrst_rate { if !othr_leg_intrst_rate_value.validate() { return false; } }
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


// EnergyCommodityNaturalGas1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityNaturalGas1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType7Code,
	#[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
	pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType6Code>,
}

impl EnergyCommodityNaturalGas1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if !addtl_sub_pdct_value.validate() { return false; } }
		return true
	}
}


// EnergyCommodityOil1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityOil1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType8Code,
	#[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
	pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType7Code>,
}

impl EnergyCommodityOil1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if !addtl_sub_pdct_value.validate() { return false; } }
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


// EnvironmentalCommodityEmission1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCommodityEmission1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType3Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType10Code,
	#[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
	pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType8Code>,
}

impl EnvironmentalCommodityEmission1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if !addtl_sub_pdct_value.validate() { return false; } }
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


// FinancialInstrument48Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument48Choice {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
	pub indx: Option<FinancialInstrument58>,
}

impl FinancialInstrument48Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref isin_value) = self.isin { if !isin_value.validate() { return false; } }
		if let Some(ref lei_value) = self.lei { if !lei_value.validate() { return false; } }
		if let Some(ref indx_value) = self.indx { if !indx_value.validate() { return false; } }
		return true
	}
}


// FinancialInstrument53 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument53 {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<Vec<ISINOct2015Identifier>>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<Vec<LEIIdentifier>>,
}

impl FinancialInstrument53 {
	pub fn validate(&self) -> bool {
		if let Some(ref isin_vec) = self.isin { for item in isin_vec { if !item.validate() { return false; } } }
		if let Some(ref lei_vec) = self.lei { for item in lei_vec { if !item.validate() { return false; } } }
		return true
	}
}


// FinancialInstrument58 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument58 {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
	#[serde(rename = "Nm")]
	pub nm: FloatingInterestRate8,
}

impl FinancialInstrument58 {
	pub fn validate(&self) -> bool {
		if let Some(ref isin_value) = self.isin { if !isin_value.validate() { return false; } }
		if !self.nm.validate() { return false }
		return true
	}
}


// FinancialInstrumentIdentification5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentIdentification5Choice {
	#[serde(rename = "Sngl", skip_serializing_if = "Option::is_none")]
	pub sngl: Option<FinancialInstrument48Choice>,
	#[serde(rename = "Bskt", skip_serializing_if = "Option::is_none")]
	pub bskt: Option<FinancialInstrument53>,
}

impl FinancialInstrumentIdentification5Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref sngl_value) = self.sngl { if !sngl_value.validate() { return false; } }
		if let Some(ref bskt_value) = self.bskt { if !bskt_value.validate() { return false; } }
		return true
	}
}


// FinancialInstrumentReportingCancellationReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentReportingCancellationReportV01 {
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: SecuritiesMarketReportHeader1,
	#[serde(rename = "CxlData")]
	pub cxl_data: Vec<SecuritiesReferenceDataReport7>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl FinancialInstrumentReportingCancellationReportV01 {
	pub fn validate(&self) -> bool {
		if !self.rpt_hdr.validate() { return false }
		for item in &self.cxl_data { if !item.validate() { return false; } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
	}
}


// FloatingInterestRate6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FloatingInterestRate6 {
	#[serde(rename = "RefRate")]
	pub ref_rate: BenchmarkCurveName6Choice,
	#[serde(rename = "Term")]
	pub term: InterestRateContractTerm2,
	#[serde(rename = "BsisPtSprd")]
	pub bsis_pt_sprd: f64,
}

impl FloatingInterestRate6 {
	pub fn validate(&self) -> bool {
		if !self.ref_rate.validate() { return false }
		if !self.term.validate() { return false }
		return true
	}
}


// FloatingInterestRate8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FloatingInterestRate8 {
	#[serde(rename = "RefRate")]
	pub ref_rate: BenchmarkCurveName5Choice,
	#[serde(rename = "Term", skip_serializing_if = "Option::is_none")]
	pub term: Option<InterestRateContractTerm2>,
}

impl FloatingInterestRate8 {
	pub fn validate(&self) -> bool {
		if !self.ref_rate.validate() { return false }
		if let Some(ref term_value) = self.term { if !term_value.validate() { return false; } }
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


// FreightCommodityDry1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityDry1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType4Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType31Code,
	#[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
	pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType14Code>,
}

impl FreightCommodityDry1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if !addtl_sub_pdct_value.validate() { return false; } }
		return true
	}
}


// FreightCommodityWet1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityWet1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType4Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType32Code,
	#[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
	pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType12Code>,
}

impl FreightCommodityWet1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if !self.sub_pdct.validate() { return false }
		if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if !addtl_sub_pdct_value.validate() { return false; } }
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


// InterestRate6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRate6Choice {
	#[serde(rename = "Fxd", skip_serializing_if = "Option::is_none")]
	pub fxd: Option<f64>,
	#[serde(rename = "Fltg", skip_serializing_if = "Option::is_none")]
	pub fltg: Option<FloatingInterestRate6>,
}

impl InterestRate6Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref fltg_value) = self.fltg { if !fltg_value.validate() { return false; } }
		return true
	}
}


// InterestRate8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRate8Choice {
	#[serde(rename = "Fxd", skip_serializing_if = "Option::is_none")]
	pub fxd: Option<f64>,
	#[serde(rename = "Fltg", skip_serializing_if = "Option::is_none")]
	pub fltg: Option<FloatingInterestRate8>,
}

impl InterestRate8Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref fltg_value) = self.fltg { if !fltg_value.validate() { return false; } }
		return true
	}
}


// InterestRateContractTerm2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRateContractTerm2 {
	#[serde(rename = "Unit")]
	pub unit: RateBasis1Code,
	#[serde(rename = "Val")]
	pub val: f64,
}

impl InterestRateContractTerm2 {
	pub fn validate(&self) -> bool {
		if !self.unit.validate() { return false }
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


// Max25Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max25Text {
	#[serde(rename = "$value")]
	pub max25_text: String,
}

impl Max25Text {
	pub fn validate(&self) -> bool {
		if self.max25_text.chars().count() < 1 {
			return false
		}
		if self.max25_text.chars().count() > 25 {
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


// NonNegativeDecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NonNegativeDecimalNumber {
	#[serde(rename = "$value")]
	pub non_negative_decimal_number: f64,
}

impl NonNegativeDecimalNumber {
	pub fn validate(&self) -> bool {
		if self.non_negative_decimal_number < 0.000000 {
			return false
		}
		return true
	}
}


// OptionStyle7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OptionStyle7Code {
	#[default]
	#[serde(rename = "AMER")]
	CodeAMER,
	#[serde(rename = "ASIA")]
	CodeASIA,
	#[serde(rename = "BERM")]
	CodeBERM,
	#[serde(rename = "EURO")]
	CodeEURO,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}

impl OptionStyle7Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// OptionType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OptionType2Code {
	#[default]
	#[serde(rename = "CALL")]
	CodeCALL,
	#[serde(rename = "PUTO")]
	CodePUTO,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}

impl OptionType2Code {
	pub fn validate(&self) -> bool {
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


// Period2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}

impl Period2 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Period4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period4Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt", skip_serializing_if = "Option::is_none")]
	pub to_dt: Option<String>,
	#[serde(rename = "FrDtToDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt_to_dt: Option<Period2>,
}

impl Period4Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref fr_dt_to_dt_value) = self.fr_dt_to_dt { if !fr_dt_to_dt_value.validate() { return false; } }
		return true
	}
}


// PhysicalTransferType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PhysicalTransferType4Code {
	#[default]
	#[serde(rename = "PHYS")]
	CodePHYS,
	#[serde(rename = "OPTL")]
	CodeOPTL,
	#[serde(rename = "CASH")]
	CodeCASH,
}

impl PhysicalTransferType4Code {
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


// RecordTechnicalData4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RecordTechnicalData4 {
	#[serde(rename = "IncnsstncyInd", skip_serializing_if = "Option::is_none")]
	pub incnsstncy_ind: Option<bool>,
	#[serde(rename = "LastUpd", skip_serializing_if = "Option::is_none")]
	pub last_upd: Option<String>,
	#[serde(rename = "SubmissnDtTm", skip_serializing_if = "Option::is_none")]
	pub submissn_dt_tm: Option<String>,
	#[serde(rename = "RlvntCmptntAuthrty", skip_serializing_if = "Option::is_none")]
	pub rlvnt_cmptnt_authrty: Option<CountryCode>,
	#[serde(rename = "PblctnPrd", skip_serializing_if = "Option::is_none")]
	pub pblctn_prd: Option<Period4Choice>,
	#[serde(rename = "NvrPblshd", skip_serializing_if = "Option::is_none")]
	pub nvr_pblshd: Option<bool>,
	#[serde(rename = "RlvntTradgVn", skip_serializing_if = "Option::is_none")]
	pub rlvnt_tradg_vn: Option<MICIdentifier>,
}

impl RecordTechnicalData4 {
	pub fn validate(&self) -> bool {
		if let Some(ref rlvnt_cmptnt_authrty_value) = self.rlvnt_cmptnt_authrty { if !rlvnt_cmptnt_authrty_value.validate() { return false; } }
		if let Some(ref pblctn_prd_value) = self.pblctn_prd { if !pblctn_prd_value.validate() { return false; } }
		if let Some(ref rlvnt_tradg_vn_value) = self.rlvnt_tradg_vn { if !rlvnt_tradg_vn_value.validate() { return false; } }
		return true
	}
}


// SecuritiesMarketReportHeader1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesMarketReportHeader1 {
	#[serde(rename = "RptgNtty")]
	pub rptg_ntty: TradingVenueIdentification1Choice,
	#[serde(rename = "RptgPrd")]
	pub rptg_prd: Period4Choice,
	#[serde(rename = "SubmissnDtTm", skip_serializing_if = "Option::is_none")]
	pub submissn_dt_tm: Option<String>,
}

impl SecuritiesMarketReportHeader1 {
	pub fn validate(&self) -> bool {
		if !self.rptg_ntty.validate() { return false }
		if !self.rptg_prd.validate() { return false }
		return true
	}
}


// SecuritiesReferenceDataReport7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesReferenceDataReport7 {
	#[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
	pub tech_rcrd_id: Option<Max35Text>,
	#[serde(rename = "FinInstrmGnlAttrbts")]
	pub fin_instrm_gnl_attrbts: SecurityInstrumentDescription17,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<LEIIdentifier>,
	#[serde(rename = "TradgVnRltdAttrbts")]
	pub tradg_vn_rltd_attrbts: Vec<TradingVenueAttributes2>,
	#[serde(rename = "DebtInstrmAttrbts", skip_serializing_if = "Option::is_none")]
	pub debt_instrm_attrbts: Option<DebtInstrument2>,
	#[serde(rename = "DerivInstrmAttrbts", skip_serializing_if = "Option::is_none")]
	pub deriv_instrm_attrbts: Option<DerivativeInstrument5>,
	#[serde(rename = "TechAttrbts", skip_serializing_if = "Option::is_none")]
	pub tech_attrbts: Option<RecordTechnicalData4>,
}

impl SecuritiesReferenceDataReport7 {
	pub fn validate(&self) -> bool {
		if let Some(ref tech_rcrd_id_value) = self.tech_rcrd_id { if !tech_rcrd_id_value.validate() { return false; } }
		if !self.fin_instrm_gnl_attrbts.validate() { return false }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		for item in &self.tradg_vn_rltd_attrbts { if !item.validate() { return false; } }
		if let Some(ref debt_instrm_attrbts_value) = self.debt_instrm_attrbts { if !debt_instrm_attrbts_value.validate() { return false; } }
		if let Some(ref deriv_instrm_attrbts_value) = self.deriv_instrm_attrbts { if !deriv_instrm_attrbts_value.validate() { return false; } }
		if let Some(ref tech_attrbts_value) = self.tech_attrbts { if !tech_attrbts_value.validate() { return false; } }
		return true
	}
}


// SecuritiesTransactionPrice1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice1 {
	#[serde(rename = "Pdg")]
	pub pdg: PriceStatus1Code,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveOrHistoricCurrencyCode>,
}

impl SecuritiesTransactionPrice1 {
	pub fn validate(&self) -> bool {
		if !self.pdg.validate() { return false }
		if let Some(ref ccy_value) = self.ccy { if !ccy_value.validate() { return false; } }
		return true
	}
}


// SecuritiesTransactionPrice2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice2Choice {
	#[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
	pub mntry_val: Option<AmountAndDirection61>,
	#[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
	pub pctg: Option<f64>,
	#[serde(rename = "Yld", skip_serializing_if = "Option::is_none")]
	pub yld: Option<f64>,
	#[serde(rename = "BsisPts", skip_serializing_if = "Option::is_none")]
	pub bsis_pts: Option<f64>,
}

impl SecuritiesTransactionPrice2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref mntry_val_value) = self.mntry_val { if !mntry_val_value.validate() { return false; } }
		return true
	}
}


// SecuritiesTransactionPrice4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice4Choice {
	#[serde(rename = "Pric", skip_serializing_if = "Option::is_none")]
	pub pric: Option<SecuritiesTransactionPrice2Choice>,
	#[serde(rename = "NoPric", skip_serializing_if = "Option::is_none")]
	pub no_pric: Option<SecuritiesTransactionPrice1>,
}

impl SecuritiesTransactionPrice4Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref pric_value) = self.pric { if !pric_value.validate() { return false; } }
		if let Some(ref no_pric_value) = self.no_pric { if !no_pric_value.validate() { return false; } }
		return true
	}
}


// SecurityInstrumentDescription17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityInstrumentDescription17 {
	#[serde(rename = "Id")]
	pub id: ISINOct2015Identifier,
	#[serde(rename = "FullNm", skip_serializing_if = "Option::is_none")]
	pub full_nm: Option<Max350Text>,
	#[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
	pub shrt_nm: Option<Max35Text>,
	#[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
	pub clssfctn_tp: Option<CFIOct2015Identifier>,
	#[serde(rename = "NtnlCcy", skip_serializing_if = "Option::is_none")]
	pub ntnl_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "CmmdtyDerivInd", skip_serializing_if = "Option::is_none")]
	pub cmmdty_deriv_ind: Option<bool>,
}

impl SecurityInstrumentDescription17 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref full_nm_value) = self.full_nm { if !full_nm_value.validate() { return false; } }
		if let Some(ref shrt_nm_value) = self.shrt_nm { if !shrt_nm_value.validate() { return false; } }
		if let Some(ref clssfctn_tp_value) = self.clssfctn_tp { if !clssfctn_tp_value.validate() { return false; } }
		if let Some(ref ntnl_ccy_value) = self.ntnl_ccy { if !ntnl_ccy_value.validate() { return false; } }
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


// TradingVenue2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TradingVenue2Code {
	#[default]
	#[serde(rename = "APPA")]
	CodeAPPA,
	#[serde(rename = "CTPS")]
	CodeCTPS,
}

impl TradingVenue2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// TradingVenueAttributes2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingVenueAttributes2 {
	#[serde(rename = "Id")]
	pub id: MICIdentifier,
	#[serde(rename = "IssrReq", skip_serializing_if = "Option::is_none")]
	pub issr_req: Option<bool>,
	#[serde(rename = "AdmssnApprvlDtByIssr", skip_serializing_if = "Option::is_none")]
	pub admssn_apprvl_dt_by_issr: Option<String>,
	#[serde(rename = "ReqForAdmssnDt", skip_serializing_if = "Option::is_none")]
	pub req_for_admssn_dt: Option<String>,
	#[serde(rename = "FrstTradDt", skip_serializing_if = "Option::is_none")]
	pub frst_trad_dt: Option<String>,
	#[serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none")]
	pub termntn_dt: Option<String>,
}

impl TradingVenueAttributes2 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		return true
	}
}


// TradingVenueIdentification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingVenueIdentification1Choice {
	#[serde(rename = "MktIdCd", skip_serializing_if = "Option::is_none")]
	pub mkt_id_cd: Option<MICIdentifier>,
	#[serde(rename = "NtlCmptntAuthrty", skip_serializing_if = "Option::is_none")]
	pub ntl_cmptnt_authrty: Option<CountryCode>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<TradingVenueIdentification2>,
}

impl TradingVenueIdentification1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref mkt_id_cd_value) = self.mkt_id_cd { if !mkt_id_cd_value.validate() { return false; } }
		if let Some(ref ntl_cmptnt_authrty_value) = self.ntl_cmptnt_authrty { if !ntl_cmptnt_authrty_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
	}
}


// TradingVenueIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingVenueIdentification2 {
	#[serde(rename = "Id")]
	pub id: Max50Text,
	#[serde(rename = "Tp")]
	pub tp: TradingVenue2Code,
}

impl TradingVenueIdentification2 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.tp.validate() { return false }
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
