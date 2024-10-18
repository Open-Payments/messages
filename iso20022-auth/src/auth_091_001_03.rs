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


// ActiveOrHistoricCurrencyAnd19DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyAnd19DecimalAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_and19_decimal_amount_simple_type: f64,
}

impl ActiveOrHistoricCurrencyAnd19DecimalAmountSimpleType {
	pub fn validate(&self) -> bool {
		if self.active_or_historic_currency_and19_decimal_amount_simple_type < 0.000000 {
			return false
		}
		return true
	}
}


// ActiveOrHistoricCurrencyAnd19DecimalAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd19DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl ActiveOrHistoricCurrencyAnd19DecimalAmount {
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


// AgriculturalCommodityDairy2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityDairy2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType20Code>,
}

impl AgriculturalCommodityDairy2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// AgriculturalCommodityForestry2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityForestry2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType21Code>,
}

impl AgriculturalCommodityForestry2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// AgriculturalCommodityGrain3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityGrain3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType5Code>,
	#[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
	pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType30Code>,
}

impl AgriculturalCommodityGrain3 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if !addtl_sub_pdct_value.validate() { return false; } }
		return true
	}
}


// AgriculturalCommodityLiveStock2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityLiveStock2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType22Code>,
}

impl AgriculturalCommodityLiveStock2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// AgriculturalCommodityOilSeed2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityOilSeed2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType1Code>,
	#[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
	pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType1Code>,
}

impl AgriculturalCommodityOilSeed2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if !addtl_sub_pdct_value.validate() { return false; } }
		return true
	}
}


// AgriculturalCommodityOliveOil3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityOliveOil3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType3Code>,
	#[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
	pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType29Code>,
}

impl AgriculturalCommodityOliveOil3 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if !addtl_sub_pdct_value.validate() { return false; } }
		return true
	}
}


// AgriculturalCommodityOther2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType49Code>,
}

impl AgriculturalCommodityOther2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// AgriculturalCommodityPotato2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityPotato2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType45Code>,
}

impl AgriculturalCommodityPotato2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// AgriculturalCommoditySeafood2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommoditySeafood2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType23Code>,
}

impl AgriculturalCommoditySeafood2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// AgriculturalCommoditySoft2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommoditySoft2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType2Code>,
	#[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
	pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType2Code>,
}

impl AgriculturalCommoditySoft2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if !addtl_sub_pdct_value.validate() { return false; } }
		return true
	}
}


// AmountAndDirection106 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection106 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAnd19DecimalAmount,
	#[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
	pub sgn: Option<bool>,
}

impl AmountAndDirection106 {
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


// AssetClassCommodity6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodity6Choice {
	#[serde(rename = "Agrcltrl", skip_serializing_if = "Option::is_none")]
	pub agrcltrl: Option<AssetClassCommodityAgricultural6Choice>,
	#[serde(rename = "Nrgy", skip_serializing_if = "Option::is_none")]
	pub nrgy: Option<AssetClassCommodityEnergy3Choice>,
	#[serde(rename = "Envttl", skip_serializing_if = "Option::is_none")]
	pub envttl: Option<AssetClassCommodityEnvironmental3Choice>,
	#[serde(rename = "Frtlzr", skip_serializing_if = "Option::is_none")]
	pub frtlzr: Option<AssetClassCommodityFertilizer4Choice>,
	#[serde(rename = "Frght", skip_serializing_if = "Option::is_none")]
	pub frght: Option<AssetClassCommodityFreight4Choice>,
	#[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
	pub indx: Option<AssetClassCommodityIndex1>,
	#[serde(rename = "IndstrlPdct", skip_serializing_if = "Option::is_none")]
	pub indstrl_pdct: Option<AssetClassCommodityIndustrialProduct2Choice>,
	#[serde(rename = "Infltn", skip_serializing_if = "Option::is_none")]
	pub infltn: Option<AssetClassCommodityInflation1>,
	#[serde(rename = "Metl", skip_serializing_if = "Option::is_none")]
	pub metl: Option<AssetClassCommodityMetal2Choice>,
	#[serde(rename = "MultiCmmdtyExtc", skip_serializing_if = "Option::is_none")]
	pub multi_cmmdty_extc: Option<AssetClassCommodityMultiCommodityExotic1>,
	#[serde(rename = "OffclEcnmcSttstcs", skip_serializing_if = "Option::is_none")]
	pub offcl_ecnmc_sttstcs: Option<AssetClassCommodityOfficialEconomicStatistics1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<AssetClassCommodityOther1>,
	#[serde(rename = "OthrC10", skip_serializing_if = "Option::is_none")]
	pub othr_c10: Option<AssetClassCommodityC10Other1>,
	#[serde(rename = "Ppr", skip_serializing_if = "Option::is_none")]
	pub ppr: Option<AssetClassCommodityPaper4Choice>,
	#[serde(rename = "Plprpln", skip_serializing_if = "Option::is_none")]
	pub plprpln: Option<AssetClassCommodityPolypropylene4Choice>,
}

impl AssetClassCommodity6Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref agrcltrl_value) = self.agrcltrl { if !agrcltrl_value.validate() { return false; } }
		if let Some(ref nrgy_value) = self.nrgy { if !nrgy_value.validate() { return false; } }
		if let Some(ref envttl_value) = self.envttl { if !envttl_value.validate() { return false; } }
		if let Some(ref frtlzr_value) = self.frtlzr { if !frtlzr_value.validate() { return false; } }
		if let Some(ref frght_value) = self.frght { if !frght_value.validate() { return false; } }
		if let Some(ref indx_value) = self.indx { if !indx_value.validate() { return false; } }
		if let Some(ref indstrl_pdct_value) = self.indstrl_pdct { if !indstrl_pdct_value.validate() { return false; } }
		if let Some(ref infltn_value) = self.infltn { if !infltn_value.validate() { return false; } }
		if let Some(ref metl_value) = self.metl { if !metl_value.validate() { return false; } }
		if let Some(ref multi_cmmdty_extc_value) = self.multi_cmmdty_extc { if !multi_cmmdty_extc_value.validate() { return false; } }
		if let Some(ref offcl_ecnmc_sttstcs_value) = self.offcl_ecnmc_sttstcs { if !offcl_ecnmc_sttstcs_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		if let Some(ref othr_c10_value) = self.othr_c10 { if !othr_c10_value.validate() { return false; } }
		if let Some(ref ppr_value) = self.ppr { if !ppr_value.validate() { return false; } }
		if let Some(ref plprpln_value) = self.plprpln { if !plprpln_value.validate() { return false; } }
		return true
	}
}


// AssetClassCommodityAgricultural6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityAgricultural6Choice {
	#[serde(rename = "GrnOilSeed", skip_serializing_if = "Option::is_none")]
	pub grn_oil_seed: Option<AgriculturalCommodityOilSeed2>,
	#[serde(rename = "Soft", skip_serializing_if = "Option::is_none")]
	pub soft: Option<AgriculturalCommoditySoft2>,
	#[serde(rename = "Ptt", skip_serializing_if = "Option::is_none")]
	pub ptt: Option<AgriculturalCommodityPotato2>,
	#[serde(rename = "OlvOil", skip_serializing_if = "Option::is_none")]
	pub olv_oil: Option<AgriculturalCommodityOliveOil3>,
	#[serde(rename = "Dairy", skip_serializing_if = "Option::is_none")]
	pub dairy: Option<AgriculturalCommodityDairy2>,
	#[serde(rename = "Frstry", skip_serializing_if = "Option::is_none")]
	pub frstry: Option<AgriculturalCommodityForestry2>,
	#[serde(rename = "Sfd", skip_serializing_if = "Option::is_none")]
	pub sfd: Option<AgriculturalCommoditySeafood2>,
	#[serde(rename = "LiveStock", skip_serializing_if = "Option::is_none")]
	pub live_stock: Option<AgriculturalCommodityLiveStock2>,
	#[serde(rename = "Grn", skip_serializing_if = "Option::is_none")]
	pub grn: Option<AgriculturalCommodityGrain3>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<AgriculturalCommodityOther2>,
}

impl AssetClassCommodityAgricultural6Choice {
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


// AssetClassCommodityC10Other1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityC10Other1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType11Code,
}

impl AssetClassCommodityC10Other1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		return true
	}
}


// AssetClassCommodityEnergy3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityEnergy3Choice {
	#[serde(rename = "Elctrcty", skip_serializing_if = "Option::is_none")]
	pub elctrcty: Option<EnergyCommodityElectricity2>,
	#[serde(rename = "NtrlGas", skip_serializing_if = "Option::is_none")]
	pub ntrl_gas: Option<EnergyCommodityNaturalGas3>,
	#[serde(rename = "Oil", skip_serializing_if = "Option::is_none")]
	pub oil: Option<EnergyCommodityOil3>,
	#[serde(rename = "Coal", skip_serializing_if = "Option::is_none")]
	pub coal: Option<EnergyCommodityCoal2>,
	#[serde(rename = "IntrNrgy", skip_serializing_if = "Option::is_none")]
	pub intr_nrgy: Option<EnergyCommodityInterEnergy2>,
	#[serde(rename = "RnwblNrgy", skip_serializing_if = "Option::is_none")]
	pub rnwbl_nrgy: Option<EnergyCommodityRenewableEnergy2>,
	#[serde(rename = "LghtEnd", skip_serializing_if = "Option::is_none")]
	pub lght_end: Option<EnergyCommodityLightEnd2>,
	#[serde(rename = "Dstllts", skip_serializing_if = "Option::is_none")]
	pub dstllts: Option<EnergyCommodityDistillates2>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<EnergyCommodityOther2>,
}

impl AssetClassCommodityEnergy3Choice {
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


// AssetClassCommodityEnvironmental3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityEnvironmental3Choice {
	#[serde(rename = "Emssns", skip_serializing_if = "Option::is_none")]
	pub emssns: Option<EnvironmentalCommodityEmission3>,
	#[serde(rename = "Wthr", skip_serializing_if = "Option::is_none")]
	pub wthr: Option<EnvironmentalCommodityWeather2>,
	#[serde(rename = "CrbnRltd", skip_serializing_if = "Option::is_none")]
	pub crbn_rltd: Option<EnvironmentalCommodityCarbonRelated2>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<EnvironmentCommodityOther2>,
}

impl AssetClassCommodityEnvironmental3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref emssns_value) = self.emssns { if !emssns_value.validate() { return false; } }
		if let Some(ref wthr_value) = self.wthr { if !wthr_value.validate() { return false; } }
		if let Some(ref crbn_rltd_value) = self.crbn_rltd { if !crbn_rltd_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
	}
}


// AssetClassCommodityFertilizer4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityFertilizer4Choice {
	#[serde(rename = "Ammn", skip_serializing_if = "Option::is_none")]
	pub ammn: Option<FertilizerCommodityAmmonia2>,
	#[serde(rename = "DmmnmPhspht", skip_serializing_if = "Option::is_none")]
	pub dmmnm_phspht: Option<FertilizerCommodityDiammoniumPhosphate2>,
	#[serde(rename = "Ptsh", skip_serializing_if = "Option::is_none")]
	pub ptsh: Option<FertilizerCommodityPotash2>,
	#[serde(rename = "Slphr", skip_serializing_if = "Option::is_none")]
	pub slphr: Option<FertilizerCommoditySulphur2>,
	#[serde(rename = "Urea", skip_serializing_if = "Option::is_none")]
	pub urea: Option<FertilizerCommodityUrea2>,
	#[serde(rename = "UreaAndAmmnmNtrt", skip_serializing_if = "Option::is_none")]
	pub urea_and_ammnm_ntrt: Option<FertilizerCommodityUreaAndAmmoniumNitrate2>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<FertilizerCommodityOther2>,
}

impl AssetClassCommodityFertilizer4Choice {
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


// AssetClassCommodityFreight4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityFreight4Choice {
	#[serde(rename = "Dry", skip_serializing_if = "Option::is_none")]
	pub dry: Option<FreightCommodityDry3>,
	#[serde(rename = "Wet", skip_serializing_if = "Option::is_none")]
	pub wet: Option<FreightCommodityWet3>,
	#[serde(rename = "CntnrShip", skip_serializing_if = "Option::is_none")]
	pub cntnr_ship: Option<FreightCommodityContainerShip2>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<FreightCommodityOther2>,
}

impl AssetClassCommodityFreight4Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref dry_value) = self.dry { if !dry_value.validate() { return false; } }
		if let Some(ref wet_value) = self.wet { if !wet_value.validate() { return false; } }
		if let Some(ref cntnr_ship_value) = self.cntnr_ship { if !cntnr_ship_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
	}
}


// AssetClassCommodityIndex1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityIndex1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType16Code,
}

impl AssetClassCommodityIndex1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		return true
	}
}


// AssetClassCommodityIndustrialProduct2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityIndustrialProduct2Choice {
	#[serde(rename = "Cnstrctn", skip_serializing_if = "Option::is_none")]
	pub cnstrctn: Option<IndustrialProductCommodityConstruction2>,
	#[serde(rename = "Manfctg", skip_serializing_if = "Option::is_none")]
	pub manfctg: Option<IndustrialProductCommodityManufacturing2>,
}

impl AssetClassCommodityIndustrialProduct2Choice {
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


// AssetClassCommodityMetal2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityMetal2Choice {
	#[serde(rename = "NonPrcs", skip_serializing_if = "Option::is_none")]
	pub non_prcs: Option<MetalCommodityNonPrecious2>,
	#[serde(rename = "Prcs", skip_serializing_if = "Option::is_none")]
	pub prcs: Option<MetalCommodityPrecious2>,
}

impl AssetClassCommodityMetal2Choice {
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


// AssetClassCommodityPaper4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityPaper4Choice {
	#[serde(rename = "CntnrBrd", skip_serializing_if = "Option::is_none")]
	pub cntnr_brd: Option<PaperCommodityContainerBoard2>,
	#[serde(rename = "Nwsprnt", skip_serializing_if = "Option::is_none")]
	pub nwsprnt: Option<PaperCommodityNewsprint2>,
	#[serde(rename = "Pulp", skip_serializing_if = "Option::is_none")]
	pub pulp: Option<PaperCommodityPulp2>,
	#[serde(rename = "RcvrdPpr", skip_serializing_if = "Option::is_none")]
	pub rcvrd_ppr: Option<PaperCommodityOther1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<PaperCommodityOther1>,
}

impl AssetClassCommodityPaper4Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cntnr_brd_value) = self.cntnr_brd { if !cntnr_brd_value.validate() { return false; } }
		if let Some(ref nwsprnt_value) = self.nwsprnt { if !nwsprnt_value.validate() { return false; } }
		if let Some(ref pulp_value) = self.pulp { if !pulp_value.validate() { return false; } }
		if let Some(ref rcvrd_ppr_value) = self.rcvrd_ppr { if !rcvrd_ppr_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
	}
}


// AssetClassCommodityPolypropylene4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityPolypropylene4Choice {
	#[serde(rename = "Plstc", skip_serializing_if = "Option::is_none")]
	pub plstc: Option<PolypropyleneCommodityPlastic2>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<PolypropyleneCommodityOther2>,
}

impl AssetClassCommodityPolypropylene4Choice {
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


// AssetClassProductType16Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType16Code {
	#[default]
	#[serde(rename = "INDX")]
	CodeINDX,
}

impl AssetClassProductType16Code {
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


// BaseOne18Rate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BaseOne18Rate {
	#[serde(rename = "$value")]
	pub base_one18_rate: f64,
}

impl BaseOne18Rate {
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


// BasketConstituents3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BasketConstituents3 {
	#[serde(rename = "InstrmId")]
	pub instrm_id: InstrumentIdentification6Choice,
	#[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
	pub qty: Option<f64>,
	#[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
	pub unit_of_measr: Option<UnitOfMeasure8Choice>,
}

impl BasketConstituents3 {
	pub fn validate(&self) -> bool {
		if !self.instrm_id.validate() { return false }
		if let Some(ref unit_of_measr_value) = self.unit_of_measr { if !unit_of_measr_value.validate() { return false; } }
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


// Cleared23Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Cleared23Choice {
	#[serde(rename = "Clrd", skip_serializing_if = "Option::is_none")]
	pub clrd: Option<ClearingPartyAndTime21Choice>,
	#[serde(rename = "IntndToClear", skip_serializing_if = "Option::is_none")]
	pub intnd_to_clear: Option<ClearingPartyAndTime22Choice>,
	#[serde(rename = "NonClrd", skip_serializing_if = "Option::is_none")]
	pub non_clrd: Option<ClearingExceptionOrExemption3Choice>,
}

impl Cleared23Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref clrd_value) = self.clrd { if !clrd_value.validate() { return false; } }
		if let Some(ref intnd_to_clear_value) = self.intnd_to_clear { if !intnd_to_clear_value.validate() { return false; } }
		if let Some(ref non_clrd_value) = self.non_clrd { if !non_clrd_value.validate() { return false; } }
		return true
	}
}


// ClearingAccountType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ClearingAccountType4Code {
	#[default]
	#[serde(rename = "CLIE")]
	CodeCLIE,
	#[serde(rename = "HOUS")]
	CodeHOUS,
}

impl ClearingAccountType4Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ClearingExceptionOrExemption2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingExceptionOrExemption2 {
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: NonClearingReason2,
	#[serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none")]
	pub othr_ctr_pty: Option<NonClearingReason2>,
}

impl ClearingExceptionOrExemption2 {
	pub fn validate(&self) -> bool {
		if !self.rptg_ctr_pty.validate() { return false }
		if let Some(ref othr_ctr_pty_value) = self.othr_ctr_pty { if !othr_ctr_pty_value.validate() { return false; } }
		return true
	}
}


// ClearingExceptionOrExemption3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingExceptionOrExemption3Choice {
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<NoReasonCode>,
	#[serde(rename = "CtrPties", skip_serializing_if = "Option::is_none")]
	pub ctr_pties: Option<ClearingExceptionOrExemption2>,
}

impl ClearingExceptionOrExemption3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref rsn_value) = self.rsn { if !rsn_value.validate() { return false; } }
		if let Some(ref ctr_pties_value) = self.ctr_pties { if !ctr_pties_value.validate() { return false; } }
		return true
	}
}


// ClearingExemptionException1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ClearingExemptionException1Code {
	#[default]
	#[serde(rename = "COOP")]
	CodeCOOP,
	#[serde(rename = "ENDU")]
	CodeENDU,
	#[serde(rename = "AFFL")]
	CodeAFFL,
	#[serde(rename = "NOAL")]
	CodeNOAL,
	#[serde(rename = "NORE")]
	CodeNORE,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "SMBK")]
	CodeSMBK,
}

impl ClearingExemptionException1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ClearingObligationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ClearingObligationType1Code {
	#[default]
	#[serde(rename = "FLSE")]
	CodeFLSE,
	#[serde(rename = "UKWN")]
	CodeUKWN,
	#[serde(rename = "TRUE")]
	CodeTRUE,
}

impl ClearingObligationType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ClearingPartyAndTime21Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingPartyAndTime21Choice {
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<NoReasonCode>,
	#[serde(rename = "Dtls", skip_serializing_if = "Option::is_none")]
	pub dtls: Option<ClearingPartyAndTime22>,
}

impl ClearingPartyAndTime21Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref rsn_value) = self.rsn { if !rsn_value.validate() { return false; } }
		if let Some(ref dtls_value) = self.dtls { if !dtls_value.validate() { return false; } }
		return true
	}
}


// ClearingPartyAndTime22 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingPartyAndTime22 {
	#[serde(rename = "CCP", skip_serializing_if = "Option::is_none")]
	pub ccp: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "ClrRctDtTm", skip_serializing_if = "Option::is_none")]
	pub clr_rct_dt_tm: Option<String>,
	#[serde(rename = "ClrDtTm", skip_serializing_if = "Option::is_none")]
	pub clr_dt_tm: Option<String>,
	#[serde(rename = "ClrIdr", skip_serializing_if = "Option::is_none")]
	pub clr_idr: Option<UniqueTransactionIdentifier2Choice>,
	#[serde(rename = "OrgnlIdr", skip_serializing_if = "Option::is_none")]
	pub orgnl_idr: Option<UniqueTransactionIdentifier2Choice>,
	#[serde(rename = "OrgnlTradRpstryIdr", skip_serializing_if = "Option::is_none")]
	pub orgnl_trad_rpstry_idr: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "ClrAcctOrgn", skip_serializing_if = "Option::is_none")]
	pub clr_acct_orgn: Option<ClearingAccountType4Code>,
}

impl ClearingPartyAndTime22 {
	pub fn validate(&self) -> bool {
		if let Some(ref ccp_value) = self.ccp { if !ccp_value.validate() { return false; } }
		if let Some(ref clr_idr_value) = self.clr_idr { if !clr_idr_value.validate() { return false; } }
		if let Some(ref orgnl_idr_value) = self.orgnl_idr { if !orgnl_idr_value.validate() { return false; } }
		if let Some(ref orgnl_trad_rpstry_idr_value) = self.orgnl_trad_rpstry_idr { if !orgnl_trad_rpstry_idr_value.validate() { return false; } }
		if let Some(ref clr_acct_orgn_value) = self.clr_acct_orgn { if !clr_acct_orgn_value.validate() { return false; } }
		return true
	}
}


// ClearingPartyAndTime22Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingPartyAndTime22Choice {
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<NoReasonCode>,
	#[serde(rename = "Dtls", skip_serializing_if = "Option::is_none")]
	pub dtls: Option<ClearingPartyAndTime23>,
}

impl ClearingPartyAndTime22Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref rsn_value) = self.rsn { if !rsn_value.validate() { return false; } }
		if let Some(ref dtls_value) = self.dtls { if !dtls_value.validate() { return false; } }
		return true
	}
}


// ClearingPartyAndTime23 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingPartyAndTime23 {
	#[serde(rename = "CCP", skip_serializing_if = "Option::is_none")]
	pub ccp: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "ClrRctDtTm", skip_serializing_if = "Option::is_none")]
	pub clr_rct_dt_tm: Option<String>,
	#[serde(rename = "ClrDtTm", skip_serializing_if = "Option::is_none")]
	pub clr_dt_tm: Option<String>,
	#[serde(rename = "ClrIdr", skip_serializing_if = "Option::is_none")]
	pub clr_idr: Option<UniqueTransactionIdentifier1Choice>,
	#[serde(rename = "OrgnlIdr", skip_serializing_if = "Option::is_none")]
	pub orgnl_idr: Option<UniqueTransactionIdentifier1Choice>,
	#[serde(rename = "OrgnlTradRpstryIdr", skip_serializing_if = "Option::is_none")]
	pub orgnl_trad_rpstry_idr: Option<OrganisationIdentification15Choice>,
}

impl ClearingPartyAndTime23 {
	pub fn validate(&self) -> bool {
		if let Some(ref ccp_value) = self.ccp { if !ccp_value.validate() { return false; } }
		if let Some(ref clr_idr_value) = self.clr_idr { if !clr_idr_value.validate() { return false; } }
		if let Some(ref orgnl_idr_value) = self.orgnl_idr { if !orgnl_idr_value.validate() { return false; } }
		if let Some(ref orgnl_trad_rpstry_idr_value) = self.orgnl_trad_rpstry_idr { if !orgnl_trad_rpstry_idr_value.validate() { return false; } }
		return true
	}
}


// CollateralPortfolioCode5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralPortfolioCode5Choice {
	#[serde(rename = "Prtfl", skip_serializing_if = "Option::is_none")]
	pub prtfl: Option<PortfolioCode3Choice>,
	#[serde(rename = "MrgnPrtflCd", skip_serializing_if = "Option::is_none")]
	pub mrgn_prtfl_cd: Option<MarginPortfolio3>,
}

impl CollateralPortfolioCode5Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref prtfl_value) = self.prtfl { if !prtfl_value.validate() { return false; } }
		if let Some(ref mrgn_prtfl_cd_value) = self.mrgn_prtfl_cd { if !mrgn_prtfl_cd_value.validate() { return false; } }
		return true
	}
}


// CompareActiveOrHistoricCurrencyAndAmount4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareActiveOrHistoricCurrencyAndAmount4 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
}

impl CompareActiveOrHistoricCurrencyAndAmount4 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareActiveOrHistoricCurrencyCode1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareActiveOrHistoricCurrencyCode1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<ActiveOrHistoricCurrencyCode>,
}

impl CompareActiveOrHistoricCurrencyCode1 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareAmountAndDirection3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareAmountAndDirection3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<AmountAndDirection106>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<AmountAndDirection106>,
}

impl CompareAmountAndDirection3 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareAssetClass1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareAssetClass1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<ProductType4Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<ProductType4Code>,
}

impl CompareAssetClass1 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareBenchmarkCode1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareBenchmarkCode1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<ExternalBenchmarkCurveName1Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<ExternalBenchmarkCurveName1Code>,
}

impl CompareBenchmarkCode1 {
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


// CompareCommodityAssetClass4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareCommodityAssetClass4 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<AssetClassCommodity6Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<AssetClassCommodity6Choice>,
}

impl CompareCommodityAssetClass4 {
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


// CompareDatePeriod2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareDatePeriod2 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<DatePeriod4>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<DatePeriod4>,
}

impl CompareDatePeriod2 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
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


// CompareDayCount1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareDayCount1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<InterestComputationMethodFormat7>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<InterestComputationMethodFormat7>,
}

impl CompareDayCount1 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareDeliveryInterconnectionPoint1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareDeliveryInterconnectionPoint1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<DeliveryInterconnectionPoint1Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<DeliveryInterconnectionPoint1Choice>,
}

impl CompareDeliveryInterconnectionPoint1 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareDeliveryType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareDeliveryType1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<PhysicalTransferType4Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<PhysicalTransferType4Code>,
}

impl CompareDeliveryType1 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareDerivativeEvent1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareDerivativeEvent1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<DerivativeEvent6>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<DerivativeEvent6>,
}

impl CompareDerivativeEvent1 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareDurationType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareDurationType1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<DurationType1Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<DurationType1Code>,
}

impl CompareDurationType1 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareEnergyDeliveryAttribute1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareEnergyDeliveryAttribute1 {
	#[serde(rename = "NrgyDlvryIntrvl", skip_serializing_if = "Option::is_none")]
	pub nrgy_dlvry_intrvl: Option<Vec<CompareTimePeriod2>>,
	#[serde(rename = "NrgyDt", skip_serializing_if = "Option::is_none")]
	pub nrgy_dt: Option<CompareDatePeriod2>,
	#[serde(rename = "NrgyDrtn", skip_serializing_if = "Option::is_none")]
	pub nrgy_drtn: Option<CompareDurationType1>,
	#[serde(rename = "NrgyWkDay", skip_serializing_if = "Option::is_none")]
	pub nrgy_wk_day: Option<Vec<CompareWeekDay1>>,
	#[serde(rename = "NrgyDlvryCpcty", skip_serializing_if = "Option::is_none")]
	pub nrgy_dlvry_cpcty: Option<CompareLongFraction19DecimalNumber1>,
	#[serde(rename = "NrgyQtyUnit", skip_serializing_if = "Option::is_none")]
	pub nrgy_qty_unit: Option<CompareEnergyQuantityUnit1>,
	#[serde(rename = "NrgyPricTmIntrvlQty", skip_serializing_if = "Option::is_none")]
	pub nrgy_pric_tm_intrvl_qty: Option<CompareAmountAndDirection3>,
}

impl CompareEnergyDeliveryAttribute1 {
	pub fn validate(&self) -> bool {
		if let Some(ref nrgy_dlvry_intrvl_vec) = self.nrgy_dlvry_intrvl { for item in nrgy_dlvry_intrvl_vec { if !item.validate() { return false; } } }
		if let Some(ref nrgy_dt_value) = self.nrgy_dt { if !nrgy_dt_value.validate() { return false; } }
		if let Some(ref nrgy_drtn_value) = self.nrgy_drtn { if !nrgy_drtn_value.validate() { return false; } }
		if let Some(ref nrgy_wk_day_vec) = self.nrgy_wk_day { for item in nrgy_wk_day_vec { if !item.validate() { return false; } } }
		if let Some(ref nrgy_dlvry_cpcty_value) = self.nrgy_dlvry_cpcty { if !nrgy_dlvry_cpcty_value.validate() { return false; } }
		if let Some(ref nrgy_qty_unit_value) = self.nrgy_qty_unit { if !nrgy_qty_unit_value.validate() { return false; } }
		if let Some(ref nrgy_pric_tm_intrvl_qty_value) = self.nrgy_pric_tm_intrvl_qty { if !nrgy_pric_tm_intrvl_qty_value.validate() { return false; } }
		return true
	}
}


// CompareEnergyLoadType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareEnergyLoadType1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<EnergyLoadType1Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<EnergyLoadType1Code>,
}

impl CompareEnergyLoadType1 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareEnergyQuantityUnit1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareEnergyQuantityUnit1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<EnergyQuantityUnit2Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<EnergyQuantityUnit2Choice>,
}

impl CompareEnergyQuantityUnit1 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareExchangeRate1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareExchangeRate1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<f64>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<f64>,
}

impl CompareExchangeRate1 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CompareExchangeRateBasis1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareExchangeRateBasis1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<ExchangeRateBasis1Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<ExchangeRateBasis1Choice>,
}

impl CompareExchangeRateBasis1 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareFinancialInstrumentContractType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareFinancialInstrumentContractType1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<FinancialInstrumentContractType2Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<FinancialInstrumentContractType2Code>,
}

impl CompareFinancialInstrumentContractType1 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareFrequencyUnit1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareFrequencyUnit1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<Frequency13Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<Frequency13Code>,
}

impl CompareFrequencyUnit1 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareISINIdentifier2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareISINIdentifier2 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<ISINOct2015Identifier>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<ISINOct2015Identifier>,
}

impl CompareISINIdentifier2 {
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


// CompareLegDirection2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareLegDirection2 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<Direction4Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<Direction4Choice>,
}

impl CompareLegDirection2 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareLongFraction19DecimalNumber1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareLongFraction19DecimalNumber1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<f64>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<f64>,
}

impl CompareLongFraction19DecimalNumber1 {
	pub fn validate(&self) -> bool {
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


// CompareMasterAgreementType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareMasterAgreementType1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<AgreementType2Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<AgreementType2Choice>,
}

impl CompareMasterAgreementType1 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareMax350Text1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareMax350Text1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<Max350Text>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<Max350Text>,
}

impl CompareMax350Text1 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareMax50Text1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareMax50Text1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<Max50Text>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<Max50Text>,
}

impl CompareMax50Text1 {
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


// CompareNumber7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareNumber7 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<f64>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<f64>,
}

impl CompareNumber7 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CompareOptionStyle1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareOptionStyle1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<OptionStyle6Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<OptionStyle6Code>,
}

impl CompareOptionStyle1 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareOptionType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareOptionType1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<OptionType2Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<OptionType2Code>,
}

impl CompareOptionType1 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
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


// CompareOtherPayment1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareOtherPayment1 {
	#[serde(rename = "OthrPmtTp", skip_serializing_if = "Option::is_none")]
	pub othr_pmt_tp: Option<CompareOtherPaymentType1>,
	#[serde(rename = "OthrPmtAmt", skip_serializing_if = "Option::is_none")]
	pub othr_pmt_amt: Option<CompareAmountAndDirection3>,
	#[serde(rename = "OthrPmtDt", skip_serializing_if = "Option::is_none")]
	pub othr_pmt_dt: Option<CompareDate3>,
	#[serde(rename = "OthrPmtPyer", skip_serializing_if = "Option::is_none")]
	pub othr_pmt_pyer: Option<CompareOrganisationIdentification7>,
	#[serde(rename = "OthrPmtRcvr", skip_serializing_if = "Option::is_none")]
	pub othr_pmt_rcvr: Option<CompareOrganisationIdentification7>,
}

impl CompareOtherPayment1 {
	pub fn validate(&self) -> bool {
		if let Some(ref othr_pmt_tp_value) = self.othr_pmt_tp { if !othr_pmt_tp_value.validate() { return false; } }
		if let Some(ref othr_pmt_amt_value) = self.othr_pmt_amt { if !othr_pmt_amt_value.validate() { return false; } }
		if let Some(ref othr_pmt_dt_value) = self.othr_pmt_dt { if !othr_pmt_dt_value.validate() { return false; } }
		if let Some(ref othr_pmt_pyer_value) = self.othr_pmt_pyer { if !othr_pmt_pyer_value.validate() { return false; } }
		if let Some(ref othr_pmt_rcvr_value) = self.othr_pmt_rcvr { if !othr_pmt_rcvr_value.validate() { return false; } }
		return true
	}
}


// CompareOtherPaymentType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareOtherPaymentType1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<PaymentType5Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<PaymentType5Choice>,
}

impl CompareOtherPaymentType1 {
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


// ComparePostTradeRiskReduction2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ComparePostTradeRiskReduction2 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<PTRREvent3>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<PTRREvent3>,
}

impl ComparePostTradeRiskReduction2 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareReferenceParty1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareReferenceParty1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<DerivativePartyIdentification1Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<DerivativePartyIdentification1Choice>,
}

impl CompareReferenceParty1 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareReportingLevelType2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareReportingLevelType2 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<ModificationLevel1Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<ModificationLevel1Code>,
}

impl CompareReportingLevelType2 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareSeniorityType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareSeniorityType1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<DebtInstrumentSeniorityType2Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<DebtInstrumentSeniorityType2Code>,
}

impl CompareSeniorityType1 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareText1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareText1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<Max52Text>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<Max52Text>,
}

impl CompareText1 {
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


// CompareTimePeriod2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareTimePeriod2 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<TimePeriod3>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<TimePeriod3>,
}

impl CompareTimePeriod2 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareTradeClearingObligation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareTradeClearingObligation1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<ClearingObligationType1Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<ClearingObligationType1Code>,
}

impl CompareTradeClearingObligation1 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareTradeClearingStatus3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareTradeClearingStatus3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<Cleared23Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<Cleared23Choice>,
}

impl CompareTradeClearingStatus3 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareTradeConfirmation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareTradeConfirmation2 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<TradeConfirmation3Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<TradeConfirmation3Choice>,
}

impl CompareTradeConfirmation2 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareTrancheIndicator1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareTrancheIndicator1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<TrancheIndicator3Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<TrancheIndicator3Choice>,
}

impl CompareTrancheIndicator1 {
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


// CompareUnderlyingInstrument3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareUnderlyingInstrument3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<SecurityIdentification41Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<SecurityIdentification41Choice>,
}

impl CompareUnderlyingInstrument3 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareUniqueProductIdentifier2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareUniqueProductIdentifier2 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<UniqueProductIdentifier2Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<UniqueProductIdentifier2Choice>,
}

impl CompareUniqueProductIdentifier2 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareUniqueTransactionIdentifier2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareUniqueTransactionIdentifier2 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<UniqueTransactionIdentifier2Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<UniqueTransactionIdentifier2Choice>,
}

impl CompareUniqueTransactionIdentifier2 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareUnitPrice4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareUnitPrice4 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<SecuritiesTransactionPrice17Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<SecuritiesTransactionPrice17Choice>,
}

impl CompareUnitPrice4 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareUnitPrice5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareUnitPrice5 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<SecuritiesTransactionPrice17Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<SecuritiesTransactionPrice17Choice>,
}

impl CompareUnitPrice5 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareUnitPrice7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareUnitPrice7 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<SecuritiesTransactionPrice14Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<SecuritiesTransactionPrice14Choice>,
}

impl CompareUnitPrice7 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareUnitPrice8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareUnitPrice8 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<SecuritiesTransactionPrice13Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<SecuritiesTransactionPrice13Choice>,
}

impl CompareUnitPrice8 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareValuationType1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareValuationType1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<ValuationType1Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<ValuationType1Code>,
}

impl CompareValuationType1 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// CompareWeekDay1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareWeekDay1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<WeekDay3Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<WeekDay3Code>,
}

impl CompareWeekDay1 {
	pub fn validate(&self) -> bool {
		if let Some(ref val1_value) = self.val1 { if !val1_value.validate() { return false; } }
		if let Some(ref val2_value) = self.val2 { if !val2_value.validate() { return false; } }
		return true
	}
}


// ContractMatchingCriteria3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractMatchingCriteria3 {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<CompareISINIdentifier2>,
	#[serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none")]
	pub unq_pdct_idr: Option<CompareUniqueProductIdentifier2>,
	#[serde(rename = "AltrntvInstrmId", skip_serializing_if = "Option::is_none")]
	pub altrntv_instrm_id: Option<CompareText1>,
	#[serde(rename = "PdctClssfctn", skip_serializing_if = "Option::is_none")]
	pub pdct_clssfctn: Option<CompareCFIIdentifier3>,
	#[serde(rename = "CtrctTp", skip_serializing_if = "Option::is_none")]
	pub ctrct_tp: Option<CompareFinancialInstrumentContractType1>,
	#[serde(rename = "AsstClss", skip_serializing_if = "Option::is_none")]
	pub asst_clss: Option<CompareAssetClass1>,
	#[serde(rename = "DerivBasedOnCrptAsst", skip_serializing_if = "Option::is_none")]
	pub deriv_based_on_crpt_asst: Option<CompareTrueFalseIndicator3>,
	#[serde(rename = "UndrlygInstrm", skip_serializing_if = "Option::is_none")]
	pub undrlyg_instrm: Option<CompareUnderlyingInstrument3>,
	#[serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none")]
	pub sttlm_ccy: Option<CompareActiveOrHistoricCurrencyCode1>,
	#[serde(rename = "SttlmCcyScndLeg", skip_serializing_if = "Option::is_none")]
	pub sttlm_ccy_scnd_leg: Option<CompareActiveOrHistoricCurrencyCode1>,
}

impl ContractMatchingCriteria3 {
	pub fn validate(&self) -> bool {
		if let Some(ref isin_value) = self.isin { if !isin_value.validate() { return false; } }
		if let Some(ref unq_pdct_idr_value) = self.unq_pdct_idr { if !unq_pdct_idr_value.validate() { return false; } }
		if let Some(ref altrntv_instrm_id_value) = self.altrntv_instrm_id { if !altrntv_instrm_id_value.validate() { return false; } }
		if let Some(ref pdct_clssfctn_value) = self.pdct_clssfctn { if !pdct_clssfctn_value.validate() { return false; } }
		if let Some(ref ctrct_tp_value) = self.ctrct_tp { if !ctrct_tp_value.validate() { return false; } }
		if let Some(ref asst_clss_value) = self.asst_clss { if !asst_clss_value.validate() { return false; } }
		if let Some(ref deriv_based_on_crpt_asst_value) = self.deriv_based_on_crpt_asst { if !deriv_based_on_crpt_asst_value.validate() { return false; } }
		if let Some(ref undrlyg_instrm_value) = self.undrlyg_instrm { if !undrlyg_instrm_value.validate() { return false; } }
		if let Some(ref sttlm_ccy_value) = self.sttlm_ccy { if !sttlm_ccy_value.validate() { return false; } }
		if let Some(ref sttlm_ccy_scnd_leg_value) = self.sttlm_ccy_scnd_leg { if !sttlm_ccy_scnd_leg_value.validate() { return false; } }
		return true
	}
}


// CounterpartyData91 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyData91 {
	#[serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none")]
	pub rptg_ctr_pty: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none")]
	pub othr_ctr_pty: Option<PartyIdentification236Choice>,
	#[serde(rename = "RptSubmitgNtty", skip_serializing_if = "Option::is_none")]
	pub rpt_submitg_ntty: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none")]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
}

impl CounterpartyData91 {
	pub fn validate(&self) -> bool {
		if let Some(ref rptg_ctr_pty_value) = self.rptg_ctr_pty { if !rptg_ctr_pty_value.validate() { return false; } }
		if let Some(ref othr_ctr_pty_value) = self.othr_ctr_pty { if !othr_ctr_pty_value.validate() { return false; } }
		if let Some(ref rpt_submitg_ntty_value) = self.rpt_submitg_ntty { if !rpt_submitg_ntty_value.validate() { return false; } }
		if let Some(ref ntty_rspnsbl_for_rpt_value) = self.ntty_rspnsbl_for_rpt { if !ntty_rspnsbl_for_rpt_value.validate() { return false; } }
		return true
	}
}


// CounterpartyMatchingCriteria6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyMatchingCriteria6 {
	#[serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none")]
	pub rptg_ctr_pty: Option<CompareOrganisationIdentification6>,
	#[serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none")]
	pub othr_ctr_pty: Option<CompareOrganisationIdentification7>,
	#[serde(rename = "DrctnOrSd", skip_serializing_if = "Option::is_none")]
	pub drctn_or_sd: Option<CompareLegDirection2>,
}

impl CounterpartyMatchingCriteria6 {
	pub fn validate(&self) -> bool {
		if let Some(ref rptg_ctr_pty_value) = self.rptg_ctr_pty { if !rptg_ctr_pty_value.validate() { return false; } }
		if let Some(ref othr_ctr_pty_value) = self.othr_ctr_pty { if !othr_ctr_pty_value.validate() { return false; } }
		if let Some(ref drctn_or_sd_value) = self.drctn_or_sd { if !drctn_or_sd_value.validate() { return false; } }
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


// CountrySubDivisionCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CountrySubDivisionCode {
	#[serde(rename = "$value")]
	pub country_sub_division_code: String,
}

impl CountrySubDivisionCode {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{2,2}\\-[0-9A-Z]{1,3}").unwrap();
		if !pattern.is_match(&self.country_sub_division_code) {
			return false
		}
		return true
	}
}


// CustomBasket4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CustomBasket4 {
	#[serde(rename = "Strr", skip_serializing_if = "Option::is_none")]
	pub strr: Option<LEIIdentifier>,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max52Text>,
	#[serde(rename = "Cnsttnts", skip_serializing_if = "Option::is_none")]
	pub cnsttnts: Option<Vec<BasketConstituents3>>,
}

impl CustomBasket4 {
	pub fn validate(&self) -> bool {
		if let Some(ref strr_value) = self.strr { if !strr_value.validate() { return false; } }
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
		if let Some(ref cnsttnts_vec) = self.cnsttnts { for item in cnsttnts_vec { if !item.validate() { return false; } } }
		return true
	}
}


// DateAndDateTime2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTime2Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<String>,
}

impl DateAndDateTime2Choice {
	pub fn validate(&self) -> bool {
		return true
	}
}


// DatePeriod4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriod4 {
	#[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt", skip_serializing_if = "Option::is_none")]
	pub to_dt: Option<String>,
}

impl DatePeriod4 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// DebtInstrumentSeniorityType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum DebtInstrumentSeniorityType2Code {
	#[default]
	#[serde(rename = "SBOD")]
	CodeSBOD,
	#[serde(rename = "SNDB")]
	CodeSNDB,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}

impl DebtInstrumentSeniorityType2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// DeliveryInterconnectionPoint1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DeliveryInterconnectionPoint1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<EICIdentifier>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max52Text>,
}

impl DeliveryInterconnectionPoint1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// DerivativeEvent6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativeEvent6 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<DerivativeEventType3Code>,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<EventIdentifier1Choice>,
	#[serde(rename = "TmStmp", skip_serializing_if = "Option::is_none")]
	pub tm_stmp: Option<DateAndDateTime2Choice>,
	#[serde(rename = "AmdmntInd", skip_serializing_if = "Option::is_none")]
	pub amdmnt_ind: Option<bool>,
}

impl DerivativeEvent6 {
	pub fn validate(&self) -> bool {
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
		if let Some(ref tm_stmp_value) = self.tm_stmp { if !tm_stmp_value.validate() { return false; } }
		return true
	}
}


// DerivativeEventType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum DerivativeEventType3Code {
	#[default]
	#[serde(rename = "ALOC")]
	CodeALOC,
	#[serde(rename = "CLRG")]
	CodeCLRG,
	#[serde(rename = "CLAL")]
	CodeCLAL,
	#[serde(rename = "COMP")]
	CodeCOMP,
	#[serde(rename = "CORP")]
	CodeCORP,
	#[serde(rename = "CREV")]
	CodeCREV,
	#[serde(rename = "ETRM")]
	CodeETRM,
	#[serde(rename = "EXER")]
	CodeEXER,
	#[serde(rename = "INCP")]
	CodeINCP,
	#[serde(rename = "NOVA")]
	CodeNOVA,
	#[serde(rename = "PTNG")]
	CodePTNG,
	#[serde(rename = "TRAD")]
	CodeTRAD,
	#[serde(rename = "UPDT")]
	CodeUPDT,
}

impl DerivativeEventType3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// DerivativePartyIdentification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativePartyIdentification1Choice {
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<CountrySubDivisionCode>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
}

impl DerivativePartyIdentification1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref ctry_value) = self.ctry { if !ctry_value.validate() { return false; } }
		if let Some(ref ctry_sub_dvsn_value) = self.ctry_sub_dvsn { if !ctry_sub_dvsn_value.validate() { return false; } }
		if let Some(ref lei_value) = self.lei { if !lei_value.validate() { return false; } }
		return true
	}
}


// DerivativesTradeReconciliationStatisticalReportV03 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativesTradeReconciliationStatisticalReportV03 {
	#[serde(rename = "RcncltnSttstcs")]
	pub rcncltn_sttstcs: StatisticsPerCounterparty19Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl DerivativesTradeReconciliationStatisticalReportV03 {
	pub fn validate(&self) -> bool {
		if !self.rcncltn_sttstcs.validate() { return false }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
	}
}


// Direction2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Direction2 {
	#[serde(rename = "DrctnOfTheFrstLeg")]
	pub drctn_of_the_frst_leg: OptionParty3Code,
	#[serde(rename = "DrctnOfTheScndLeg", skip_serializing_if = "Option::is_none")]
	pub drctn_of_the_scnd_leg: Option<OptionParty3Code>,
}

impl Direction2 {
	pub fn validate(&self) -> bool {
		if !self.drctn_of_the_frst_leg.validate() { return false }
		if let Some(ref drctn_of_the_scnd_leg_value) = self.drctn_of_the_scnd_leg { if !drctn_of_the_scnd_leg_value.validate() { return false; } }
		return true
	}
}


// Direction4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Direction4Choice {
	#[serde(rename = "Drctn", skip_serializing_if = "Option::is_none")]
	pub drctn: Option<Direction2>,
	#[serde(rename = "CtrPtySd", skip_serializing_if = "Option::is_none")]
	pub ctr_pty_sd: Option<OptionParty1Code>,
}

impl Direction4Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref drctn_value) = self.drctn { if !drctn_value.validate() { return false; } }
		if let Some(ref ctr_pty_sd_value) = self.ctr_pty_sd { if !ctr_pty_sd_value.validate() { return false; } }
		return true
	}
}


// DurationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum DurationType1Code {
	#[default]
	#[serde(rename = "YEAR")]
	CodeYEAR,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "SEAS")]
	CodeSEAS,
	#[serde(rename = "QURT")]
	CodeQURT,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "MNUT")]
	CodeMNUT,
	#[serde(rename = "HOUR")]
	CodeHOUR,
	#[serde(rename = "DASD")]
	CodeDASD,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}

impl DurationType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// EICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EICIdentifier {
	#[serde(rename = "$value")]
	pub eic_identifier: String,
}

impl EICIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9\\-]{16}").unwrap();
		if !pattern.is_match(&self.eic_identifier) {
			return false
		}
		return true
	}
}


// EnergyCommodityCoal2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityCoal2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType24Code>,
}

impl EnergyCommodityCoal2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// EnergyCommodityDistillates2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityDistillates2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType25Code>,
}

impl EnergyCommodityDistillates2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// EnergyCommodityElectricity2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityElectricity2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType6Code>,
	#[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
	pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType5Code>,
}

impl EnergyCommodityElectricity2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if !addtl_sub_pdct_value.validate() { return false; } }
		return true
	}
}


// EnergyCommodityInterEnergy2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityInterEnergy2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType26Code>,
}

impl EnergyCommodityInterEnergy2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// EnergyCommodityLightEnd2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityLightEnd2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType27Code>,
}

impl EnergyCommodityLightEnd2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// EnergyCommodityNaturalGas3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityNaturalGas3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType7Code>,
	#[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
	pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType31Code>,
}

impl EnergyCommodityNaturalGas3 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if !addtl_sub_pdct_value.validate() { return false; } }
		return true
	}
}


// EnergyCommodityOil3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityOil3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType8Code>,
	#[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
	pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType32Code>,
}

impl EnergyCommodityOil3 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if !addtl_sub_pdct_value.validate() { return false; } }
		return true
	}
}


// EnergyCommodityOther2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType49Code>,
}

impl EnergyCommodityOther2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// EnergyCommodityRenewableEnergy2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityRenewableEnergy2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType28Code>,
}

impl EnergyCommodityRenewableEnergy2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// EnergyLoadType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EnergyLoadType1Code {
	#[default]
	#[serde(rename = "BSLD")]
	CodeBSLD,
	#[serde(rename = "GASD")]
	CodeGASD,
	#[serde(rename = "HABH")]
	CodeHABH,
	#[serde(rename = "OFFP")]
	CodeOFFP,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "PKLD")]
	CodePKLD,
	#[serde(rename = "SHPD")]
	CodeSHPD,
}

impl EnergyLoadType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// EnergyQuantityUnit2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyQuantityUnit2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<EnergyQuantityUnit2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max52Text>,
}

impl EnergyQuantityUnit2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// EnergyQuantityUnit2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EnergyQuantityUnit2Code {
	#[default]
	#[serde(rename = "BTUD")]
	CodeBTUD,
	#[serde(rename = "CMPD")]
	CodeCMPD,
	#[serde(rename = "GJDD")]
	CodeGJDD,
	#[serde(rename = "GWAT")]
	CodeGWAT,
	#[serde(rename = "GWHD")]
	CodeGWHD,
	#[serde(rename = "GWHH")]
	CodeGWHH,
	#[serde(rename = "HMJD")]
	CodeHMJD,
	#[serde(rename = "KTMD")]
	CodeKTMD,
	#[serde(rename = "KWAT")]
	CodeKWAT,
	#[serde(rename = "KWHD")]
	CodeKWHD,
	#[serde(rename = "KWHH")]
	CodeKWHH,
	#[serde(rename = "MCMD")]
	CodeMCMD,
	#[serde(rename = "MJDD")]
	CodeMJDD,
	#[serde(rename = "MBTD")]
	CodeMBTD,
	#[serde(rename = "MMJD")]
	CodeMMJD,
	#[serde(rename = "MTMD")]
	CodeMTMD,
	#[serde(rename = "MWAT")]
	CodeMWAT,
	#[serde(rename = "MWHD")]
	CodeMWHD,
	#[serde(rename = "MWHH")]
	CodeMWHH,
	#[serde(rename = "THMD")]
	CodeTHMD,
}

impl EnergyQuantityUnit2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// EnvironmentCommodityOther2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType3Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType49Code>,
}

impl EnvironmentCommodityOther2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// EnvironmentalCommodityCarbonRelated2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCommodityCarbonRelated2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType3Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType29Code>,
}

impl EnvironmentalCommodityCarbonRelated2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// EnvironmentalCommodityEmission3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCommodityEmission3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType3Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType10Code>,
	#[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
	pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType8Code>,
}

impl EnvironmentalCommodityEmission3 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if !addtl_sub_pdct_value.validate() { return false; } }
		return true
	}
}


// EnvironmentalCommodityWeather2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCommodityWeather2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType3Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType30Code>,
}

impl EnvironmentalCommodityWeather2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// EventIdentifier1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EventIdentifier1Choice {
	#[serde(rename = "EvtIdr", skip_serializing_if = "Option::is_none")]
	pub evt_idr: Option<UTIIdentifier>,
	#[serde(rename = "PstTradRskRdctnIdr", skip_serializing_if = "Option::is_none")]
	pub pst_trad_rsk_rdctn_idr: Option<PostTradeRiskReductionIdentifier1>,
}

impl EventIdentifier1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref evt_idr_value) = self.evt_idr { if !evt_idr_value.validate() { return false; } }
		if let Some(ref pst_trad_rsk_rdctn_idr_value) = self.pst_trad_rsk_rdctn_idr { if !pst_trad_rsk_rdctn_idr_value.validate() { return false; } }
		return true
	}
}


// ExchangeRateBasis1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExchangeRateBasis1 {
	#[serde(rename = "BaseCcy")]
	pub base_ccy: ActiveCurrencyCode,
	#[serde(rename = "QtdCcy")]
	pub qtd_ccy: ActiveCurrencyCode,
}

impl ExchangeRateBasis1 {
	pub fn validate(&self) -> bool {
		if !self.base_ccy.validate() { return false }
		if !self.qtd_ccy.validate() { return false }
		return true
	}
}


// ExchangeRateBasis1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExchangeRateBasis1Choice {
	#[serde(rename = "CcyPair", skip_serializing_if = "Option::is_none")]
	pub ccy_pair: Option<ExchangeRateBasis1>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max52Text>,
}

impl ExchangeRateBasis1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref ccy_pair_value) = self.ccy_pair { if !ccy_pair_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
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


// ExternalBenchmarkCurveName1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalBenchmarkCurveName1Code {
	#[serde(rename = "$value")]
	pub external_benchmark_curve_name1_code: String,
}

impl ExternalBenchmarkCurveName1Code {
	pub fn validate(&self) -> bool {
		if self.external_benchmark_curve_name1_code.chars().count() < 1 {
			return false
		}
		if self.external_benchmark_curve_name1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// ExternalUnitOfMeasure1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalUnitOfMeasure1Code {
	#[serde(rename = "$value")]
	pub external_unit_of_measure1_code: String,
}

impl ExternalUnitOfMeasure1Code {
	pub fn validate(&self) -> bool {
		if self.external_unit_of_measure1_code.chars().count() < 1 {
			return false
		}
		if self.external_unit_of_measure1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// FertilizerCommodityAmmonia2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityAmmonia2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType39Code>,
}

impl FertilizerCommodityAmmonia2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// FertilizerCommodityDiammoniumPhosphate2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityDiammoniumPhosphate2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType40Code>,
}

impl FertilizerCommodityDiammoniumPhosphate2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// FertilizerCommodityOther2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType49Code>,
}

impl FertilizerCommodityOther2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// FertilizerCommodityPotash2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityPotash2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType41Code>,
}

impl FertilizerCommodityPotash2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// FertilizerCommoditySulphur2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommoditySulphur2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType42Code>,
}

impl FertilizerCommoditySulphur2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// FertilizerCommodityUrea2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityUrea2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType43Code>,
}

impl FertilizerCommodityUrea2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// FertilizerCommodityUreaAndAmmoniumNitrate2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityUreaAndAmmoniumNitrate2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType44Code>,
}

impl FertilizerCommodityUreaAndAmmoniumNitrate2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// FinancialInstrumentContractType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FinancialInstrumentContractType2Code {
	#[default]
	#[serde(rename = "CFDS")]
	CodeCFDS,
	#[serde(rename = "FRAS")]
	CodeFRAS,
	#[serde(rename = "FUTR")]
	CodeFUTR,
	#[serde(rename = "FORW")]
	CodeFORW,
	#[serde(rename = "OPTN")]
	CodeOPTN,
	#[serde(rename = "SPDB")]
	CodeSPDB,
	#[serde(rename = "SWAP")]
	CodeSWAP,
	#[serde(rename = "SWPT")]
	CodeSWPT,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}

impl FinancialInstrumentContractType2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// FreightCommodityContainerShip2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityContainerShip2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType4Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType46Code>,
}

impl FreightCommodityContainerShip2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// FreightCommodityDry3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityDry3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType4Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType31Code>,
	#[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
	pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType33Code>,
}

impl FreightCommodityDry3 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if !addtl_sub_pdct_value.validate() { return false; } }
		return true
	}
}


// FreightCommodityOther2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType4Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType49Code>,
}

impl FreightCommodityOther2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// FreightCommodityWet3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityWet3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType4Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType32Code>,
	#[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
	pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType34Code>,
}

impl FreightCommodityWet3 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if !addtl_sub_pdct_value.validate() { return false; } }
		return true
	}
}


// Frequency13Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Frequency13Code {
	#[default]
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "YEAR")]
	CodeYEAR,
	#[serde(rename = "ADHO")]
	CodeADHO,
	#[serde(rename = "EXPI")]
	CodeEXPI,
	#[serde(rename = "MIAN")]
	CodeMIAN,
	#[serde(rename = "QURT")]
	CodeQURT,
}

impl Frequency13Code {
	pub fn validate(&self) -> bool {
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


// GenericIdentification179 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification179 {
	#[serde(rename = "Id")]
	pub id: Max52Text,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericIdentification179 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		return true
	}
}


// GenericIdentification184 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification184 {
	#[serde(rename = "Id")]
	pub id: Max210Text,
	#[serde(rename = "Src")]
	pub src: Max100Text,
}

impl GenericIdentification184 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.src.validate() { return false }
		return true
	}
}


// GenericIdentification185 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification185 {
	#[serde(rename = "Id")]
	pub id: Max100Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericIdentification185 {
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


// ISOTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISOTime {
	#[serde(rename = "$value")]
	pub iso_time: String,
}

impl ISOTime {
	pub fn validate(&self) -> bool {
		return true
	}
}


// IndexIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndexIdentification1 {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max350Text>,
	#[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
	pub indx: Option<ExternalBenchmarkCurveName1Code>,
}

impl IndexIdentification1 {
	pub fn validate(&self) -> bool {
		if let Some(ref isin_value) = self.isin { if !isin_value.validate() { return false; } }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref indx_value) = self.indx { if !indx_value.validate() { return false; } }
		return true
	}
}


// IndustrialProductCommodityConstruction2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndustrialProductCommodityConstruction2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType6Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType33Code>,
}

impl IndustrialProductCommodityConstruction2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// IndustrialProductCommodityManufacturing2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndustrialProductCommodityManufacturing2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType6Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType34Code>,
}

impl IndustrialProductCommodityManufacturing2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// InstrumentIdentification6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InstrumentIdentification6Choice {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
	#[serde(rename = "AltrntvInstrmId", skip_serializing_if = "Option::is_none")]
	pub altrntv_instrm_id: Option<Max52Text>,
	#[serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none")]
	pub unq_pdct_idr: Option<UniqueProductIdentifier1Choice>,
	#[serde(rename = "OthrId", skip_serializing_if = "Option::is_none")]
	pub othr_id: Option<GenericIdentification184>,
}

impl InstrumentIdentification6Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref isin_value) = self.isin { if !isin_value.validate() { return false; } }
		if let Some(ref altrntv_instrm_id_value) = self.altrntv_instrm_id { if !altrntv_instrm_id_value.validate() { return false; } }
		if let Some(ref unq_pdct_idr_value) = self.unq_pdct_idr { if !unq_pdct_idr_value.validate() { return false; } }
		if let Some(ref othr_id_value) = self.othr_id { if !othr_id_value.validate() { return false; } }
		return true
	}
}


// InterestComputationMethod4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InterestComputationMethod4Code {
	#[default]
	#[serde(rename = "A004")]
	CodeA004,
	#[serde(rename = "A019")]
	CodeA019,
	#[serde(rename = "A017")]
	CodeA017,
	#[serde(rename = "A005")]
	CodeA005,
	#[serde(rename = "A009")]
	CodeA009,
	#[serde(rename = "A014")]
	CodeA014,
	#[serde(rename = "A010")]
	CodeA010,
	#[serde(rename = "A006")]
	CodeA006,
	#[serde(rename = "A008")]
	CodeA008,
	#[serde(rename = "A015")]
	CodeA015,
	#[serde(rename = "A018")]
	CodeA018,
	#[serde(rename = "A011")]
	CodeA011,
	#[serde(rename = "A001")]
	CodeA001,
	#[serde(rename = "A002")]
	CodeA002,
	#[serde(rename = "A003")]
	CodeA003,
	#[serde(rename = "A012")]
	CodeA012,
	#[serde(rename = "A013")]
	CodeA013,
	#[serde(rename = "A007")]
	CodeA007,
	#[serde(rename = "A016")]
	CodeA016,
	#[serde(rename = "NARR")]
	CodeNARR,
	#[serde(rename = "A020")]
	CodeA020,
}

impl InterestComputationMethod4Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// InterestComputationMethodFormat7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestComputationMethodFormat7 {
	#[serde(rename = "Cd")]
	pub cd: InterestComputationMethod4Code,
	#[serde(rename = "Nrrtv", skip_serializing_if = "Option::is_none")]
	pub nrrtv: Option<Max1000Text>,
}

impl InterestComputationMethodFormat7 {
	pub fn validate(&self) -> bool {
		if !self.cd.validate() { return false }
		if let Some(ref nrrtv_value) = self.nrrtv { if !nrrtv_value.validate() { return false; } }
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


// LegalPersonIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LegalPersonIdentification1 {
	#[serde(rename = "Id")]
	pub id: OrganisationIdentification15Choice,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
}

impl LegalPersonIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref ctry_value) = self.ctry { if !ctry_value.validate() { return false; } }
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


// MarginPortfolio3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarginPortfolio3 {
	#[serde(rename = "InitlMrgnPrtflCd")]
	pub initl_mrgn_prtfl_cd: PortfolioCode5Choice,
	#[serde(rename = "VartnMrgnPrtflCd", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
}

impl MarginPortfolio3 {
	pub fn validate(&self) -> bool {
		if !self.initl_mrgn_prtfl_cd.validate() { return false }
		if let Some(ref vartn_mrgn_prtfl_cd_value) = self.vartn_mrgn_prtfl_cd { if !vartn_mrgn_prtfl_cd_value.validate() { return false; } }
		return true
	}
}


// MasterAgreement8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MasterAgreement8 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<AgreementType2Choice>,
	#[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
	pub vrsn: Option<Max50Text>,
	#[serde(rename = "OthrMstrAgrmtDtls", skip_serializing_if = "Option::is_none")]
	pub othr_mstr_agrmt_dtls: Option<Max350Text>,
}

impl MasterAgreement8 {
	pub fn validate(&self) -> bool {
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		if let Some(ref vrsn_value) = self.vrsn { if !vrsn_value.validate() { return false; } }
		if let Some(ref othr_mstr_agrmt_dtls_value) = self.othr_mstr_agrmt_dtls { if !othr_mstr_agrmt_dtls_value.validate() { return false; } }
		return true
	}
}


// MatchingCriteria17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MatchingCriteria17 {
	#[serde(rename = "CtrPtyMtchgCrit", skip_serializing_if = "Option::is_none")]
	pub ctr_pty_mtchg_crit: Option<CounterpartyMatchingCriteria6>,
	#[serde(rename = "ValtnMtchgCrit", skip_serializing_if = "Option::is_none")]
	pub valtn_mtchg_crit: Option<ValuationMatchingCriteria1>,
	#[serde(rename = "CtrctMtchgCrit", skip_serializing_if = "Option::is_none")]
	pub ctrct_mtchg_crit: Option<ContractMatchingCriteria3>,
	#[serde(rename = "TxMtchgCrit", skip_serializing_if = "Option::is_none")]
	pub tx_mtchg_crit: Option<TransactionMatchingCriteria7>,
}

impl MatchingCriteria17 {
	pub fn validate(&self) -> bool {
		if let Some(ref ctr_pty_mtchg_crit_value) = self.ctr_pty_mtchg_crit { if !ctr_pty_mtchg_crit_value.validate() { return false; } }
		if let Some(ref valtn_mtchg_crit_value) = self.valtn_mtchg_crit { if !valtn_mtchg_crit_value.validate() { return false; } }
		if let Some(ref ctrct_mtchg_crit_value) = self.ctrct_mtchg_crit { if !ctrct_mtchg_crit_value.validate() { return false; } }
		if let Some(ref tx_mtchg_crit_value) = self.tx_mtchg_crit { if !tx_mtchg_crit_value.validate() { return false; } }
		return true
	}
}


// Max1000Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max1000Text {
	#[serde(rename = "$value")]
	pub max1000_text: String,
}

impl Max1000Text {
	pub fn validate(&self) -> bool {
		if self.max1000_text.chars().count() < 1 {
			return false
		}
		if self.max1000_text.chars().count() > 1000 {
			return false
		}
		return true
	}
}


// Max100Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max100Text {
	#[serde(rename = "$value")]
	pub max100_text: String,
}

impl Max100Text {
	pub fn validate(&self) -> bool {
		if self.max100_text.chars().count() < 1 {
			return false
		}
		if self.max100_text.chars().count() > 100 {
			return false
		}
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


// Max210Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max210Text {
	#[serde(rename = "$value")]
	pub max210_text: String,
}

impl Max210Text {
	pub fn validate(&self) -> bool {
		if self.max210_text.chars().count() < 1 {
			return false
		}
		if self.max210_text.chars().count() > 210 {
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


// Max4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "$value")]
	pub max4_alpha_numeric_text: String,
}

impl Max4AlphaNumericText {
	pub fn validate(&self) -> bool {
		if self.max4_alpha_numeric_text.chars().count() < 1 {
			return false
		}
		if self.max4_alpha_numeric_text.chars().count() > 4 {
			return false
		}
		let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
		if !pattern.is_match(&self.max4_alpha_numeric_text) {
			return false
		}
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


// MetalCommodityNonPrecious2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MetalCommodityNonPrecious2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType7Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType15Code>,
	#[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
	pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType10Code>,
}

impl MetalCommodityNonPrecious2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if !addtl_sub_pdct_value.validate() { return false; } }
		return true
	}
}


// MetalCommodityPrecious2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MetalCommodityPrecious2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType7Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType16Code>,
	#[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
	pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType11Code>,
}

impl MetalCommodityPrecious2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		if let Some(ref addtl_sub_pdct_value) = self.addtl_sub_pdct { if !addtl_sub_pdct_value.validate() { return false; } }
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


// NaturalPersonIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NaturalPersonIdentification3 {
	#[serde(rename = "Id")]
	pub id: NaturalPersonIdentification2,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
}

impl NaturalPersonIdentification3 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref ctry_value) = self.ctry { if !ctry_value.validate() { return false; } }
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


// NonClearingReason2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NonClearingReason2 {
	#[serde(rename = "ClrXmptnXcptn")]
	pub clr_xmptn_xcptn: Vec<ClearingExemptionException1Code>,
	#[serde(rename = "NonClrRsnInf", skip_serializing_if = "Option::is_none")]
	pub non_clr_rsn_inf: Option<Max350Text>,
}

impl NonClearingReason2 {
	pub fn validate(&self) -> bool {
		for item in &self.clr_xmptn_xcptn { if !item.validate() { return false; } }
		if let Some(ref non_clr_rsn_inf_value) = self.non_clr_rsn_inf { if !non_clr_rsn_inf_value.validate() { return false; } }
		return true
	}
}


// NotApplicable1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NotApplicable1Code {
	#[default]
	#[serde(rename = "NOAP")]
	CodeNOAP,
}

impl NotApplicable1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Number {
	#[serde(rename = "$value")]
	pub number: f64,
}

impl Number {
	pub fn validate(&self) -> bool {
		return true
	}
}


// OptionParty1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OptionParty1Code {
	#[default]
	#[serde(rename = "SLLR")]
	CodeSLLR,
	#[serde(rename = "BYER")]
	CodeBYER,
}

impl OptionParty1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// OptionParty3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OptionParty3Code {
	#[default]
	#[serde(rename = "MAKE")]
	CodeMAKE,
	#[serde(rename = "TAKE")]
	CodeTAKE,
}

impl OptionParty3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// OptionStyle6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OptionStyle6Code {
	#[default]
	#[serde(rename = "EURO")]
	CodeEURO,
	#[serde(rename = "BERM")]
	CodeBERM,
	#[serde(rename = "ASIA")]
	CodeASIA,
	#[serde(rename = "AMER")]
	CodeAMER,
}

impl OptionStyle6Code {
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


// PTRREvent3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PTRREvent3 {
	#[serde(rename = "Tchnq", skip_serializing_if = "Option::is_none")]
	pub tchnq: Option<RiskReductionService1Code>,
	#[serde(rename = "SvcPrvdr", skip_serializing_if = "Option::is_none")]
	pub svc_prvdr: Option<OrganisationIdentification15Choice>,
}

impl PTRREvent3 {
	pub fn validate(&self) -> bool {
		if let Some(ref tchnq_value) = self.tchnq { if !tchnq_value.validate() { return false; } }
		if let Some(ref svc_prvdr_value) = self.svc_prvdr { if !svc_prvdr_value.validate() { return false; } }
		return true
	}
}


// PairingStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PairingStatus1Code {
	#[default]
	#[serde(rename = "PARD")]
	CodePARD,
	#[serde(rename = "UNPR")]
	CodeUNPR,
}

impl PairingStatus1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// PaperCommodityContainerBoard2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityContainerBoard2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType8Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType35Code>,
}

impl PaperCommodityContainerBoard2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// PaperCommodityNewsprint2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityNewsprint2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType8Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType36Code>,
}

impl PaperCommodityNewsprint2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// PaperCommodityOther1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType8Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType49Code>,
}

impl PaperCommodityOther1 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// PaperCommodityPulp2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityPulp2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType8Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType37Code>,
}

impl PaperCommodityPulp2 {
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


// PartyIdentification248Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification248Choice {
	#[serde(rename = "Lgl", skip_serializing_if = "Option::is_none")]
	pub lgl: Option<LegalPersonIdentification1>,
	#[serde(rename = "Ntrl", skip_serializing_if = "Option::is_none")]
	pub ntrl: Option<NaturalPersonIdentification3>,
}

impl PartyIdentification248Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref lgl_value) = self.lgl { if !lgl_value.validate() { return false; } }
		if let Some(ref ntrl_value) = self.ntrl { if !ntrl_value.validate() { return false; } }
		return true
	}
}


// PaymentType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PaymentType4Code {
	#[default]
	#[serde(rename = "UFRO")]
	CodeUFRO,
	#[serde(rename = "UWIN")]
	CodeUWIN,
	#[serde(rename = "PEXH")]
	CodePEXH,
}

impl PaymentType4Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// PaymentType5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentType5Choice {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<PaymentType4Code>,
	#[serde(rename = "PrtryTp", skip_serializing_if = "Option::is_none")]
	pub prtry_tp: Option<Max4AlphaNumericText>,
}

impl PaymentType5Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		if let Some(ref prtry_tp_value) = self.prtry_tp { if !prtry_tp_value.validate() { return false; } }
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


// PolypropyleneCommodityOther2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PolypropyleneCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType9Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType49Code>,
}

impl PolypropyleneCommodityOther2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// PolypropyleneCommodityPlastic2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PolypropyleneCommodityPlastic2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType9Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType18Code>,
}

impl PolypropyleneCommodityPlastic2 {
	pub fn validate(&self) -> bool {
		if !self.base_pdct.validate() { return false }
		if let Some(ref sub_pdct_value) = self.sub_pdct { if !sub_pdct_value.validate() { return false; } }
		return true
	}
}


// PortfolioCode3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioCode3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Max52Text>,
	#[serde(rename = "NoPrtfl", skip_serializing_if = "Option::is_none")]
	pub no_prtfl: Option<NotApplicable1Code>,
}

impl PortfolioCode3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref no_prtfl_value) = self.no_prtfl { if !no_prtfl_value.validate() { return false; } }
		return true
	}
}


// PortfolioCode5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioCode5Choice {
	#[serde(rename = "Prtfl", skip_serializing_if = "Option::is_none")]
	pub prtfl: Option<PortfolioIdentification3>,
	#[serde(rename = "NoPrtfl", skip_serializing_if = "Option::is_none")]
	pub no_prtfl: Option<NotApplicable1Code>,
}

impl PortfolioCode5Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref prtfl_value) = self.prtfl { if !prtfl_value.validate() { return false; } }
		if let Some(ref no_prtfl_value) = self.no_prtfl { if !no_prtfl_value.validate() { return false; } }
		return true
	}
}


// PortfolioIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioIdentification3 {
	#[serde(rename = "Cd")]
	pub cd: Max52Text,
	#[serde(rename = "PrtflTxXmptn", skip_serializing_if = "Option::is_none")]
	pub prtfl_tx_xmptn: Option<bool>,
}

impl PortfolioIdentification3 {
	pub fn validate(&self) -> bool {
		if !self.cd.validate() { return false }
		return true
	}
}


// PostTradeRiskReductionIdentifier1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostTradeRiskReductionIdentifier1 {
	#[serde(rename = "Strr")]
	pub strr: LEIIdentifier,
	#[serde(rename = "Id")]
	pub id: Max52Text,
}

impl PostTradeRiskReductionIdentifier1 {
	pub fn validate(&self) -> bool {
		if !self.strr.validate() { return false }
		if !self.id.validate() { return false }
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


// ProductType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ProductType4Code {
	#[default]
	#[serde(rename = "CRDT")]
	CodeCRDT,
	#[serde(rename = "CURR")]
	CodeCURR,
	#[serde(rename = "EQUI")]
	CodeEQUI,
	#[serde(rename = "INTR")]
	CodeINTR,
	#[serde(rename = "COMM")]
	CodeCOMM,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}

impl ProductType4Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ReconciliationCategory4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReconciliationCategory4 {
	#[serde(rename = "Rvvd")]
	pub rvvd: bool,
	#[serde(rename = "FrthrMod")]
	pub frthr_mod: bool,
}

impl ReconciliationCategory4 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ReconciliationCategory5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReconciliationCategory5 {
	#[serde(rename = "RptgTp")]
	pub rptg_tp: TradeRepositoryReportingType1Code,
	#[serde(rename = "Pairg")]
	pub pairg: PairingStatus1Code,
	#[serde(rename = "Rcncltn")]
	pub rcncltn: ReconciliationStatus1Code,
	#[serde(rename = "ValtnRcncltn")]
	pub valtn_rcncltn: ReconciliationStatus2Code,
	#[serde(rename = "Rvvd")]
	pub rvvd: bool,
	#[serde(rename = "FrthrMod")]
	pub frthr_mod: bool,
}

impl ReconciliationCategory5 {
	pub fn validate(&self) -> bool {
		if !self.rptg_tp.validate() { return false }
		if !self.pairg.validate() { return false }
		if !self.rcncltn.validate() { return false }
		if !self.valtn_rcncltn.validate() { return false }
		return true
	}
}


// ReconciliationCounterpartyPairStatistics7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReconciliationCounterpartyPairStatistics7 {
	#[serde(rename = "CtrPtyId")]
	pub ctr_pty_id: CounterpartyData91,
	#[serde(rename = "TtlNbOfTxs")]
	pub ttl_nb_of_txs: f64,
	#[serde(rename = "RcncltnRpt")]
	pub rcncltn_rpt: Vec<ReconciliationReport15>,
}

impl ReconciliationCounterpartyPairStatistics7 {
	pub fn validate(&self) -> bool {
		if !self.ctr_pty_id.validate() { return false }
		for item in &self.rcncltn_rpt { if !item.validate() { return false; } }
		return true
	}
}


// ReconciliationReport15 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReconciliationReport15 {
	#[serde(rename = "TxId")]
	pub tx_id: TradeTransactionIdentification24,
	#[serde(rename = "MtchgCrit")]
	pub mtchg_crit: MatchingCriteria17,
}

impl ReconciliationReport15 {
	pub fn validate(&self) -> bool {
		if !self.tx_id.validate() { return false }
		if !self.mtchg_crit.validate() { return false }
		return true
	}
}


// ReconciliationStatisticsPerCounterparty4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReconciliationStatisticsPerCounterparty4 {
	#[serde(rename = "RefDt")]
	pub ref_dt: String,
	#[serde(rename = "RcncltnCtgrs")]
	pub rcncltn_ctgrs: ReportingRequirement3Choice,
	#[serde(rename = "TtlNbOfTxs", skip_serializing_if = "Option::is_none")]
	pub ttl_nb_of_txs: Option<f64>,
	#[serde(rename = "TxDtls", skip_serializing_if = "Option::is_none")]
	pub tx_dtls: Option<Vec<ReconciliationCounterpartyPairStatistics7>>,
}

impl ReconciliationStatisticsPerCounterparty4 {
	pub fn validate(&self) -> bool {
		if !self.rcncltn_ctgrs.validate() { return false }
		if let Some(ref tx_dtls_vec) = self.tx_dtls { for item in tx_dtls_vec { if !item.validate() { return false; } } }
		return true
	}
}


// ReconciliationStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ReconciliationStatus1Code {
	#[default]
	#[serde(rename = "NREC")]
	CodeNREC,
	#[serde(rename = "RECO")]
	CodeRECO,
}

impl ReconciliationStatus1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ReconciliationStatus2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ReconciliationStatus2Code {
	#[default]
	#[serde(rename = "NREC")]
	CodeNREC,
	#[serde(rename = "RECO")]
	CodeRECO,
	#[serde(rename = "NOAP")]
	CodeNOAP,
}

impl ReconciliationStatus2Code {
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


// ReportingRequirement3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportingRequirement3Choice {
	#[serde(rename = "RptgRqrmnt", skip_serializing_if = "Option::is_none")]
	pub rptg_rqrmnt: Option<ReconciliationCategory5>,
	#[serde(rename = "NoRptgRqrmnt", skip_serializing_if = "Option::is_none")]
	pub no_rptg_rqrmnt: Option<ReconciliationCategory4>,
}

impl ReportingRequirement3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref rptg_rqrmnt_value) = self.rptg_rqrmnt { if !rptg_rqrmnt_value.validate() { return false; } }
		if let Some(ref no_rptg_rqrmnt_value) = self.no_rptg_rqrmnt { if !no_rptg_rqrmnt_value.validate() { return false; } }
		return true
	}
}


// RiskReductionService1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RiskReductionService1Code {
	#[default]
	#[serde(rename = "NORR")]
	CodeNORR,
	#[serde(rename = "PWOS")]
	CodePWOS,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "PRBM")]
	CodePRBM,
	#[serde(rename = "PWAS")]
	CodePWAS,
}

impl RiskReductionService1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// SecuritiesTransactionPrice13Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice13Choice {
	#[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
	pub mntry_val: Option<AmountAndDirection106>,
	#[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
	pub pctg: Option<f64>,
	#[serde(rename = "Dcml", skip_serializing_if = "Option::is_none")]
	pub dcml: Option<f64>,
	#[serde(rename = "BsisPtSprd", skip_serializing_if = "Option::is_none")]
	pub bsis_pt_sprd: Option<f64>,
}

impl SecuritiesTransactionPrice13Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref mntry_val_value) = self.mntry_val { if !mntry_val_value.validate() { return false; } }
		return true
	}
}


// SecuritiesTransactionPrice14Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice14Choice {
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "Dcml", skip_serializing_if = "Option::is_none")]
	pub dcml: Option<f64>,
}

impl SecuritiesTransactionPrice14Choice {
	pub fn validate(&self) -> bool {
		return true
	}
}


// SecuritiesTransactionPrice17Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice17Choice {
	#[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
	pub mntry_val: Option<AmountAndDirection106>,
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

impl SecuritiesTransactionPrice17Choice {
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


// SecurityIdentification41Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification41Choice {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
	#[serde(rename = "AltrntvInstrmId", skip_serializing_if = "Option::is_none")]
	pub altrntv_instrm_id: Option<Max52Text>,
	#[serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none")]
	pub unq_pdct_idr: Option<UniqueProductIdentifier2Choice>,
	#[serde(rename = "Bskt", skip_serializing_if = "Option::is_none")]
	pub bskt: Option<CustomBasket4>,
	#[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
	pub indx: Option<IndexIdentification1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<GenericIdentification184>,
	#[serde(rename = "IdNotAvlbl", skip_serializing_if = "Option::is_none")]
	pub id_not_avlbl: Option<UnderlyingIdentification1Code>,
}

impl SecurityIdentification41Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref isin_value) = self.isin { if !isin_value.validate() { return false; } }
		if let Some(ref altrntv_instrm_id_value) = self.altrntv_instrm_id { if !altrntv_instrm_id_value.validate() { return false; } }
		if let Some(ref unq_pdct_idr_value) = self.unq_pdct_idr { if !unq_pdct_idr_value.validate() { return false; } }
		if let Some(ref bskt_value) = self.bskt { if !bskt_value.validate() { return false; } }
		if let Some(ref indx_value) = self.indx { if !indx_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		if let Some(ref id_not_avlbl_value) = self.id_not_avlbl { if !id_not_avlbl_value.validate() { return false; } }
		return true
	}
}


// StatisticsPerCounterparty19Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatisticsPerCounterparty19Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[serde(rename = "Rpt", skip_serializing_if = "Option::is_none")]
	pub rpt: Option<Vec<ReconciliationStatisticsPerCounterparty4>>,
}

impl StatisticsPerCounterparty19Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref data_set_actn_value) = self.data_set_actn { if !data_set_actn_value.validate() { return false; } }
		if let Some(ref rpt_vec) = self.rpt { for item in rpt_vec { if !item.validate() { return false; } } }
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


// TimePeriod3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimePeriod3 {
	#[serde(rename = "FrTm", skip_serializing_if = "Option::is_none")]
	pub fr_tm: Option<String>,
	#[serde(rename = "ToTm", skip_serializing_if = "Option::is_none")]
	pub to_tm: Option<String>,
}

impl TimePeriod3 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// TradeConfirmation3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeConfirmation3Choice {
	#[serde(rename = "Confd", skip_serializing_if = "Option::is_none")]
	pub confd: Option<TradeConfirmation4>,
	#[serde(rename = "NonConfd", skip_serializing_if = "Option::is_none")]
	pub non_confd: Option<TradeNonConfirmation1>,
}

impl TradeConfirmation3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref confd_value) = self.confd { if !confd_value.validate() { return false; } }
		if let Some(ref non_confd_value) = self.non_confd { if !non_confd_value.validate() { return false; } }
		return true
	}
}


// TradeConfirmation4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeConfirmation4 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<TradeConfirmationType1Code>,
	#[serde(rename = "TmStmp", skip_serializing_if = "Option::is_none")]
	pub tm_stmp: Option<String>,
}

impl TradeConfirmation4 {
	pub fn validate(&self) -> bool {
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		return true
	}
}


// TradeConfirmationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TradeConfirmationType1Code {
	#[default]
	#[serde(rename = "ECNF")]
	CodeECNF,
	#[serde(rename = "YCNF")]
	CodeYCNF,
}

impl TradeConfirmationType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// TradeConfirmationType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TradeConfirmationType2Code {
	#[default]
	#[serde(rename = "NCNF")]
	CodeNCNF,
}

impl TradeConfirmationType2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// TradeNonConfirmation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeNonConfirmation1 {
	#[serde(rename = "Tp")]
	pub tp: TradeConfirmationType2Code,
}

impl TradeNonConfirmation1 {
	pub fn validate(&self) -> bool {
		if !self.tp.validate() { return false }
		return true
	}
}


// TradeRepositoryReportingType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TradeRepositoryReportingType1Code {
	#[default]
	#[serde(rename = "SWOS")]
	CodeSWOS,
	#[serde(rename = "TWOS")]
	CodeTWOS,
}

impl TradeRepositoryReportingType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// TradeTransactionIdentification24 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeTransactionIdentification24 {
	#[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
	pub tech_rcrd_id: Option<Max140Text>,
	#[serde(rename = "ActnTp", skip_serializing_if = "Option::is_none")]
	pub actn_tp: Option<TransactionOperationType10Code>,
	#[serde(rename = "RptgTmStmp", skip_serializing_if = "Option::is_none")]
	pub rptg_tm_stmp: Option<String>,
	#[serde(rename = "DerivEvtTp", skip_serializing_if = "Option::is_none")]
	pub deriv_evt_tp: Option<DerivativeEventType3Code>,
	#[serde(rename = "DerivEvtTmStmp", skip_serializing_if = "Option::is_none")]
	pub deriv_evt_tm_stmp: Option<DateAndDateTime2Choice>,
	#[serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none")]
	pub othr_ctr_pty: Option<PartyIdentification248Choice>,
	#[serde(rename = "UnqIdr", skip_serializing_if = "Option::is_none")]
	pub unq_idr: Option<UniqueTransactionIdentifier2Choice>,
	#[serde(rename = "MstrAgrmt", skip_serializing_if = "Option::is_none")]
	pub mstr_agrmt: Option<MasterAgreement8>,
	#[serde(rename = "CollPrtflCd", skip_serializing_if = "Option::is_none")]
	pub coll_prtfl_cd: Option<CollateralPortfolioCode5Choice>,
}

impl TradeTransactionIdentification24 {
	pub fn validate(&self) -> bool {
		if let Some(ref tech_rcrd_id_value) = self.tech_rcrd_id { if !tech_rcrd_id_value.validate() { return false; } }
		if let Some(ref actn_tp_value) = self.actn_tp { if !actn_tp_value.validate() { return false; } }
		if let Some(ref deriv_evt_tp_value) = self.deriv_evt_tp { if !deriv_evt_tp_value.validate() { return false; } }
		if let Some(ref deriv_evt_tm_stmp_value) = self.deriv_evt_tm_stmp { if !deriv_evt_tm_stmp_value.validate() { return false; } }
		if let Some(ref othr_ctr_pty_value) = self.othr_ctr_pty { if !othr_ctr_pty_value.validate() { return false; } }
		if let Some(ref unq_idr_value) = self.unq_idr { if !unq_idr_value.validate() { return false; } }
		if let Some(ref mstr_agrmt_value) = self.mstr_agrmt { if !mstr_agrmt_value.validate() { return false; } }
		if let Some(ref coll_prtfl_cd_value) = self.coll_prtfl_cd { if !coll_prtfl_cd_value.validate() { return false; } }
		return true
	}
}


// Tranche3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Tranche3 {
	#[serde(rename = "AttchmntPt", skip_serializing_if = "Option::is_none")]
	pub attchmnt_pt: Option<f64>,
	#[serde(rename = "DtchmntPt", skip_serializing_if = "Option::is_none")]
	pub dtchmnt_pt: Option<f64>,
}

impl Tranche3 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// TrancheIndicator3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrancheIndicator3Choice {
	#[serde(rename = "Trnchd", skip_serializing_if = "Option::is_none")]
	pub trnchd: Option<Tranche3>,
	#[serde(rename = "Utrnchd", skip_serializing_if = "Option::is_none")]
	pub utrnchd: Option<NoReasonCode>,
}

impl TrancheIndicator3Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref trnchd_value) = self.trnchd { if !trnchd_value.validate() { return false; } }
		if let Some(ref utrnchd_value) = self.utrnchd { if !utrnchd_value.validate() { return false; } }
		return true
	}
}


// TransactionMatchingCriteria7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionMatchingCriteria7 {
	#[serde(rename = "RptTrckgNb", skip_serializing_if = "Option::is_none")]
	pub rpt_trckg_nb: Option<CompareText2>,
	#[serde(rename = "UnqTxIdr", skip_serializing_if = "Option::is_none")]
	pub unq_tx_idr: Option<CompareUniqueTransactionIdentifier2>,
	#[serde(rename = "PrrUnqTxIdr", skip_serializing_if = "Option::is_none")]
	pub prr_unq_tx_idr: Option<CompareUniqueTransactionIdentifier2>,
	#[serde(rename = "SbsqntPosUnqTxIdr", skip_serializing_if = "Option::is_none")]
	pub sbsqnt_pos_unq_tx_idr: Option<CompareUniqueTransactionIdentifier2>,
	#[serde(rename = "Dlta", skip_serializing_if = "Option::is_none")]
	pub dlta: Option<CompareLongFraction19DecimalNumber1>,
	#[serde(rename = "TradConf", skip_serializing_if = "Option::is_none")]
	pub trad_conf: Option<CompareTradeConfirmation2>,
	#[serde(rename = "TradClrOblgtn", skip_serializing_if = "Option::is_none")]
	pub trad_clr_oblgtn: Option<CompareTradeClearingObligation1>,
	#[serde(rename = "TradClrSts", skip_serializing_if = "Option::is_none")]
	pub trad_clr_sts: Option<CompareTradeClearingStatus3>,
	#[serde(rename = "MstrAgrmtTp", skip_serializing_if = "Option::is_none")]
	pub mstr_agrmt_tp: Option<CompareMasterAgreementType1>,
	#[serde(rename = "MstrAgrmtVrsn", skip_serializing_if = "Option::is_none")]
	pub mstr_agrmt_vrsn: Option<CompareMax50Text1>,
	#[serde(rename = "IntraGrp", skip_serializing_if = "Option::is_none")]
	pub intra_grp: Option<CompareTrueFalseIndicator3>,
	#[serde(rename = "PstTradRskRdctn", skip_serializing_if = "Option::is_none")]
	pub pst_trad_rsk_rdctn: Option<ComparePostTradeRiskReduction2>,
	#[serde(rename = "DerivEvt", skip_serializing_if = "Option::is_none")]
	pub deriv_evt: Option<CompareDerivativeEvent1>,
	#[serde(rename = "PltfmIdr", skip_serializing_if = "Option::is_none")]
	pub pltfm_idr: Option<CompareMICIdentifier3>,
	#[serde(rename = "ExctnTmStmp", skip_serializing_if = "Option::is_none")]
	pub exctn_tm_stmp: Option<CompareDateTime3>,
	#[serde(rename = "FctvDt", skip_serializing_if = "Option::is_none")]
	pub fctv_dt: Option<CompareDate3>,
	#[serde(rename = "XprtnDt", skip_serializing_if = "Option::is_none")]
	pub xprtn_dt: Option<CompareDate3>,
	#[serde(rename = "EarlyTermntnDt", skip_serializing_if = "Option::is_none")]
	pub early_termntn_dt: Option<CompareDate3>,
	#[serde(rename = "SttlmDt", skip_serializing_if = "Option::is_none")]
	pub sttlm_dt: Option<Vec<CompareDate3>>,
	#[serde(rename = "DlvryTp", skip_serializing_if = "Option::is_none")]
	pub dlvry_tp: Option<CompareDeliveryType1>,
	#[serde(rename = "TxPric", skip_serializing_if = "Option::is_none")]
	pub tx_pric: Option<CompareUnitPrice5>,
	#[serde(rename = "PricSchdlUadjstdFctvDt", skip_serializing_if = "Option::is_none")]
	pub pric_schdl_uadjstd_fctv_dt: Option<Vec<CompareDate3>>,
	#[serde(rename = "PricSchdlUadjstdEndDt", skip_serializing_if = "Option::is_none")]
	pub pric_schdl_uadjstd_end_dt: Option<Vec<CompareDate3>>,
	#[serde(rename = "TxSchdlPric", skip_serializing_if = "Option::is_none")]
	pub tx_schdl_pric: Option<Vec<CompareUnitPrice5>>,
	#[serde(rename = "PackgPric", skip_serializing_if = "Option::is_none")]
	pub packg_pric: Option<CompareUnitPrice5>,
	#[serde(rename = "NtnlAmtFrstLeg", skip_serializing_if = "Option::is_none")]
	pub ntnl_amt_frst_leg: Option<CompareAmountAndDirection3>,
	#[serde(rename = "NtnlAmtFrstLegUadjstdFctvDt", skip_serializing_if = "Option::is_none")]
	pub ntnl_amt_frst_leg_uadjstd_fctv_dt: Option<Vec<CompareDate3>>,
	#[serde(rename = "NtnlAmtFrstLegUadjstdEndDt", skip_serializing_if = "Option::is_none")]
	pub ntnl_amt_frst_leg_uadjstd_end_dt: Option<Vec<CompareDate3>>,
	#[serde(rename = "NtnlAmtFrstLegSchdlAmt", skip_serializing_if = "Option::is_none")]
	pub ntnl_amt_frst_leg_schdl_amt: Option<Vec<CompareAmountAndDirection3>>,
	#[serde(rename = "NtnlQtyFrstLeg", skip_serializing_if = "Option::is_none")]
	pub ntnl_qty_frst_leg: Option<CompareLongFraction19DecimalNumber1>,
	#[serde(rename = "NtnlQtyFrstLegUadjstdFctvDt", skip_serializing_if = "Option::is_none")]
	pub ntnl_qty_frst_leg_uadjstd_fctv_dt: Option<Vec<CompareDate3>>,
	#[serde(rename = "NtnlQtyFrstLegUadjstdEndDt", skip_serializing_if = "Option::is_none")]
	pub ntnl_qty_frst_leg_uadjstd_end_dt: Option<Vec<CompareDate3>>,
	#[serde(rename = "NtnlQtyFrstLegSchdlQty", skip_serializing_if = "Option::is_none")]
	pub ntnl_qty_frst_leg_schdl_qty: Option<Vec<CompareLongFraction19DecimalNumber1>>,
	#[serde(rename = "NtnlAmtScndLeg", skip_serializing_if = "Option::is_none")]
	pub ntnl_amt_scnd_leg: Option<CompareAmountAndDirection3>,
	#[serde(rename = "NtnlAmtScndLegUadjstdFctvDt", skip_serializing_if = "Option::is_none")]
	pub ntnl_amt_scnd_leg_uadjstd_fctv_dt: Option<Vec<CompareDate3>>,
	#[serde(rename = "NtnlAmtScndLegUadjstdEndDt", skip_serializing_if = "Option::is_none")]
	pub ntnl_amt_scnd_leg_uadjstd_end_dt: Option<Vec<CompareDate3>>,
	#[serde(rename = "NtnlAmtScndLegSchdlAmt", skip_serializing_if = "Option::is_none")]
	pub ntnl_amt_scnd_leg_schdl_amt: Option<Vec<CompareAmountAndDirection3>>,
	#[serde(rename = "NtnlQtyScndLeg", skip_serializing_if = "Option::is_none")]
	pub ntnl_qty_scnd_leg: Option<CompareLongFraction19DecimalNumber1>,
	#[serde(rename = "NtnlQtyScndLegUadjstdFctvDt", skip_serializing_if = "Option::is_none")]
	pub ntnl_qty_scnd_leg_uadjstd_fctv_dt: Option<Vec<CompareDate3>>,
	#[serde(rename = "NtnlQtyScndLegUadjstdEndDt", skip_serializing_if = "Option::is_none")]
	pub ntnl_qty_scnd_leg_uadjstd_end_dt: Option<Vec<CompareDate3>>,
	#[serde(rename = "NtnlQtyScndLegSchdlQty", skip_serializing_if = "Option::is_none")]
	pub ntnl_qty_scnd_leg_schdl_qty: Option<Vec<CompareLongFraction19DecimalNumber1>>,
	#[serde(rename = "OthrPmt", skip_serializing_if = "Option::is_none")]
	pub othr_pmt: Option<Vec<CompareOtherPayment1>>,
	#[serde(rename = "IntrstFxdRateFrstLeg", skip_serializing_if = "Option::is_none")]
	pub intrst_fxd_rate_frst_leg: Option<CompareUnitPrice7>,
	#[serde(rename = "IntrstFxdRateFrstLegDayCnt", skip_serializing_if = "Option::is_none")]
	pub intrst_fxd_rate_frst_leg_day_cnt: Option<CompareDayCount1>,
	#[serde(rename = "IntrstFxdRateFrstLegPmtFrqcyUnit", skip_serializing_if = "Option::is_none")]
	pub intrst_fxd_rate_frst_leg_pmt_frqcy_unit: Option<CompareFrequencyUnit1>,
	#[serde(rename = "IntrstFxdRateFrstLegPmtFrqcyVal", skip_serializing_if = "Option::is_none")]
	pub intrst_fxd_rate_frst_leg_pmt_frqcy_val: Option<CompareNumber5>,
	#[serde(rename = "IntrstFltgRateFrstLegId", skip_serializing_if = "Option::is_none")]
	pub intrst_fltg_rate_frst_leg_id: Option<CompareISINIdentifier4>,
	#[serde(rename = "IntrstFltgRateFrstLegCd", skip_serializing_if = "Option::is_none")]
	pub intrst_fltg_rate_frst_leg_cd: Option<CompareBenchmarkCode1>,
	#[serde(rename = "IntrstFltgRateFrstLegNm", skip_serializing_if = "Option::is_none")]
	pub intrst_fltg_rate_frst_leg_nm: Option<CompareMax350Text1>,
	#[serde(rename = "IntrstFltgRateFrstLegDayCnt", skip_serializing_if = "Option::is_none")]
	pub intrst_fltg_rate_frst_leg_day_cnt: Option<CompareDayCount1>,
	#[serde(rename = "IntrstFltgRateFrstLegPmtFrqcyUnit", skip_serializing_if = "Option::is_none")]
	pub intrst_fltg_rate_frst_leg_pmt_frqcy_unit: Option<CompareFrequencyUnit1>,
	#[serde(rename = "IntrstFltgRateFrstLegPmtFrqcyVal", skip_serializing_if = "Option::is_none")]
	pub intrst_fltg_rate_frst_leg_pmt_frqcy_val: Option<CompareNumber5>,
	#[serde(rename = "IntrstFltgRateFrstLegRefPrdUnit", skip_serializing_if = "Option::is_none")]
	pub intrst_fltg_rate_frst_leg_ref_prd_unit: Option<CompareFrequencyUnit1>,
	#[serde(rename = "IntrstFltgRateFrstLegRefPrdVal", skip_serializing_if = "Option::is_none")]
	pub intrst_fltg_rate_frst_leg_ref_prd_val: Option<CompareNumber5>,
	#[serde(rename = "IntrstFltgRateFrstLegRstFrqcyUnit", skip_serializing_if = "Option::is_none")]
	pub intrst_fltg_rate_frst_leg_rst_frqcy_unit: Option<CompareFrequencyUnit1>,
	#[serde(rename = "IntrstFltgRateFrstLegRstFrqcyVal", skip_serializing_if = "Option::is_none")]
	pub intrst_fltg_rate_frst_leg_rst_frqcy_val: Option<CompareNumber5>,
	#[serde(rename = "IntrstFltgRateFrstLegSprd", skip_serializing_if = "Option::is_none")]
	pub intrst_fltg_rate_frst_leg_sprd: Option<CompareUnitPrice8>,
	#[serde(rename = "IntrstRateFxdScndLeg", skip_serializing_if = "Option::is_none")]
	pub intrst_rate_fxd_scnd_leg: Option<CompareUnitPrice7>,
	#[serde(rename = "IntrstFxdRateScndLegDayCnt", skip_serializing_if = "Option::is_none")]
	pub intrst_fxd_rate_scnd_leg_day_cnt: Option<CompareDayCount1>,
	#[serde(rename = "IntrstFxdRateScndLegPmtFrqcyUnit", skip_serializing_if = "Option::is_none")]
	pub intrst_fxd_rate_scnd_leg_pmt_frqcy_unit: Option<CompareFrequencyUnit1>,
	#[serde(rename = "IntrstFxdRateScndLegPmtFrqcyVal", skip_serializing_if = "Option::is_none")]
	pub intrst_fxd_rate_scnd_leg_pmt_frqcy_val: Option<CompareNumber5>,
	#[serde(rename = "IntrstFltgRateScndLegId", skip_serializing_if = "Option::is_none")]
	pub intrst_fltg_rate_scnd_leg_id: Option<CompareISINIdentifier4>,
	#[serde(rename = "IntrstFltgRateScndLegCd", skip_serializing_if = "Option::is_none")]
	pub intrst_fltg_rate_scnd_leg_cd: Option<CompareBenchmarkCode1>,
	#[serde(rename = "IntrstFltgRateScndLegNm", skip_serializing_if = "Option::is_none")]
	pub intrst_fltg_rate_scnd_leg_nm: Option<CompareMax350Text1>,
	#[serde(rename = "IntrstFltgRateScndLegDayCnt", skip_serializing_if = "Option::is_none")]
	pub intrst_fltg_rate_scnd_leg_day_cnt: Option<CompareDayCount1>,
	#[serde(rename = "IntrstFltgRateScndLegPmtFrqcyUnit", skip_serializing_if = "Option::is_none")]
	pub intrst_fltg_rate_scnd_leg_pmt_frqcy_unit: Option<CompareFrequencyUnit1>,
	#[serde(rename = "IntrstFltgRateScndLegPmtFrqcyVal", skip_serializing_if = "Option::is_none")]
	pub intrst_fltg_rate_scnd_leg_pmt_frqcy_val: Option<CompareNumber5>,
	#[serde(rename = "IntrstFltgRateScndLegRefPrdUnit", skip_serializing_if = "Option::is_none")]
	pub intrst_fltg_rate_scnd_leg_ref_prd_unit: Option<CompareFrequencyUnit1>,
	#[serde(rename = "IntrstFltgRateScndLegRefPrdVal", skip_serializing_if = "Option::is_none")]
	pub intrst_fltg_rate_scnd_leg_ref_prd_val: Option<CompareNumber5>,
	#[serde(rename = "IntrstFltgRateScndLegRstFrqcyUnit", skip_serializing_if = "Option::is_none")]
	pub intrst_fltg_rate_scnd_leg_rst_frqcy_unit: Option<CompareFrequencyUnit1>,
	#[serde(rename = "IntrstFltgRateScndLegRstFrqcyVal", skip_serializing_if = "Option::is_none")]
	pub intrst_fltg_rate_scnd_leg_rst_frqcy_val: Option<CompareNumber5>,
	#[serde(rename = "IntrstFltgRateScndLegSprd", skip_serializing_if = "Option::is_none")]
	pub intrst_fltg_rate_scnd_leg_sprd: Option<CompareUnitPrice8>,
	#[serde(rename = "PackgSprd", skip_serializing_if = "Option::is_none")]
	pub packg_sprd: Option<CompareUnitPrice8>,
	#[serde(rename = "CcyXchgRate", skip_serializing_if = "Option::is_none")]
	pub ccy_xchg_rate: Option<CompareExchangeRate1>,
	#[serde(rename = "CcyFwdXchgRate", skip_serializing_if = "Option::is_none")]
	pub ccy_fwd_xchg_rate: Option<CompareExchangeRate1>,
	#[serde(rename = "CcyXchgRateBsis", skip_serializing_if = "Option::is_none")]
	pub ccy_xchg_rate_bsis: Option<CompareExchangeRateBasis1>,
	#[serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none")]
	pub cmmdty: Option<CompareCommodityAssetClass4>,
	#[serde(rename = "NrgyDlvryPtOrZone", skip_serializing_if = "Option::is_none")]
	pub nrgy_dlvry_pt_or_zone: Option<Vec<CompareDeliveryInterconnectionPoint1>>,
	#[serde(rename = "NrgyIntrCnnctnPt", skip_serializing_if = "Option::is_none")]
	pub nrgy_intr_cnnctn_pt: Option<CompareDeliveryInterconnectionPoint1>,
	#[serde(rename = "NrgyLdTp", skip_serializing_if = "Option::is_none")]
	pub nrgy_ld_tp: Option<CompareEnergyLoadType1>,
	#[serde(rename = "DlvryAttr", skip_serializing_if = "Option::is_none")]
	pub dlvry_attr: Option<Vec<CompareEnergyDeliveryAttribute1>>,
	#[serde(rename = "OptnTp", skip_serializing_if = "Option::is_none")]
	pub optn_tp: Option<CompareOptionType1>,
	#[serde(rename = "OptnExrcStyle", skip_serializing_if = "Option::is_none")]
	pub optn_exrc_style: Option<Vec<CompareOptionStyle1>>,
	#[serde(rename = "OptnStrkPric", skip_serializing_if = "Option::is_none")]
	pub optn_strk_pric: Option<CompareUnitPrice4>,
	#[serde(rename = "OptnStrkPricSchdlUadjstdFctvDt", skip_serializing_if = "Option::is_none")]
	pub optn_strk_pric_schdl_uadjstd_fctv_dt: Option<Vec<CompareDate3>>,
	#[serde(rename = "OptnStrkPricSchdlUadjstdEndDt", skip_serializing_if = "Option::is_none")]
	pub optn_strk_pric_schdl_uadjstd_end_dt: Option<Vec<CompareDate3>>,
	#[serde(rename = "OptnStrkPricSchdlAmt", skip_serializing_if = "Option::is_none")]
	pub optn_strk_pric_schdl_amt: Option<Vec<CompareUnitPrice4>>,
	#[serde(rename = "OptnPrmAmt", skip_serializing_if = "Option::is_none")]
	pub optn_prm_amt: Option<CompareActiveOrHistoricCurrencyAndAmount4>,
	#[serde(rename = "OptnPrmPmtDt", skip_serializing_if = "Option::is_none")]
	pub optn_prm_pmt_dt: Option<CompareDate3>,
	#[serde(rename = "OptnMtrtyDtOfUndrlyg", skip_serializing_if = "Option::is_none")]
	pub optn_mtrty_dt_of_undrlyg: Option<CompareDate3>,
	#[serde(rename = "CdtSnrty", skip_serializing_if = "Option::is_none")]
	pub cdt_snrty: Option<CompareSeniorityType1>,
	#[serde(rename = "CdtRefPty", skip_serializing_if = "Option::is_none")]
	pub cdt_ref_pty: Option<CompareReferenceParty1>,
	#[serde(rename = "CdtSrs", skip_serializing_if = "Option::is_none")]
	pub cdt_srs: Option<CompareNumber7>,
	#[serde(rename = "CdtVrsn", skip_serializing_if = "Option::is_none")]
	pub cdt_vrsn: Option<CompareNumber7>,
	#[serde(rename = "CdtIndxFctr", skip_serializing_if = "Option::is_none")]
	pub cdt_indx_fctr: Option<ComparePercentageRate3>,
	#[serde(rename = "CdtTrch", skip_serializing_if = "Option::is_none")]
	pub cdt_trch: Option<CompareTrancheIndicator1>,
	#[serde(rename = "Lvl", skip_serializing_if = "Option::is_none")]
	pub lvl: Option<CompareReportingLevelType2>,
}

impl TransactionMatchingCriteria7 {
	pub fn validate(&self) -> bool {
		if let Some(ref rpt_trckg_nb_value) = self.rpt_trckg_nb { if !rpt_trckg_nb_value.validate() { return false; } }
		if let Some(ref unq_tx_idr_value) = self.unq_tx_idr { if !unq_tx_idr_value.validate() { return false; } }
		if let Some(ref prr_unq_tx_idr_value) = self.prr_unq_tx_idr { if !prr_unq_tx_idr_value.validate() { return false; } }
		if let Some(ref sbsqnt_pos_unq_tx_idr_value) = self.sbsqnt_pos_unq_tx_idr { if !sbsqnt_pos_unq_tx_idr_value.validate() { return false; } }
		if let Some(ref dlta_value) = self.dlta { if !dlta_value.validate() { return false; } }
		if let Some(ref trad_conf_value) = self.trad_conf { if !trad_conf_value.validate() { return false; } }
		if let Some(ref trad_clr_oblgtn_value) = self.trad_clr_oblgtn { if !trad_clr_oblgtn_value.validate() { return false; } }
		if let Some(ref trad_clr_sts_value) = self.trad_clr_sts { if !trad_clr_sts_value.validate() { return false; } }
		if let Some(ref mstr_agrmt_tp_value) = self.mstr_agrmt_tp { if !mstr_agrmt_tp_value.validate() { return false; } }
		if let Some(ref mstr_agrmt_vrsn_value) = self.mstr_agrmt_vrsn { if !mstr_agrmt_vrsn_value.validate() { return false; } }
		if let Some(ref intra_grp_value) = self.intra_grp { if !intra_grp_value.validate() { return false; } }
		if let Some(ref pst_trad_rsk_rdctn_value) = self.pst_trad_rsk_rdctn { if !pst_trad_rsk_rdctn_value.validate() { return false; } }
		if let Some(ref deriv_evt_value) = self.deriv_evt { if !deriv_evt_value.validate() { return false; } }
		if let Some(ref pltfm_idr_value) = self.pltfm_idr { if !pltfm_idr_value.validate() { return false; } }
		if let Some(ref exctn_tm_stmp_value) = self.exctn_tm_stmp { if !exctn_tm_stmp_value.validate() { return false; } }
		if let Some(ref fctv_dt_value) = self.fctv_dt { if !fctv_dt_value.validate() { return false; } }
		if let Some(ref xprtn_dt_value) = self.xprtn_dt { if !xprtn_dt_value.validate() { return false; } }
		if let Some(ref early_termntn_dt_value) = self.early_termntn_dt { if !early_termntn_dt_value.validate() { return false; } }
		if let Some(ref sttlm_dt_vec) = self.sttlm_dt { for item in sttlm_dt_vec { if !item.validate() { return false; } } }
		if let Some(ref dlvry_tp_value) = self.dlvry_tp { if !dlvry_tp_value.validate() { return false; } }
		if let Some(ref tx_pric_value) = self.tx_pric { if !tx_pric_value.validate() { return false; } }
		if let Some(ref pric_schdl_uadjstd_fctv_dt_vec) = self.pric_schdl_uadjstd_fctv_dt { for item in pric_schdl_uadjstd_fctv_dt_vec { if !item.validate() { return false; } } }
		if let Some(ref pric_schdl_uadjstd_end_dt_vec) = self.pric_schdl_uadjstd_end_dt { for item in pric_schdl_uadjstd_end_dt_vec { if !item.validate() { return false; } } }
		if let Some(ref tx_schdl_pric_vec) = self.tx_schdl_pric { for item in tx_schdl_pric_vec { if !item.validate() { return false; } } }
		if let Some(ref packg_pric_value) = self.packg_pric { if !packg_pric_value.validate() { return false; } }
		if let Some(ref ntnl_amt_frst_leg_value) = self.ntnl_amt_frst_leg { if !ntnl_amt_frst_leg_value.validate() { return false; } }
		if let Some(ref ntnl_amt_frst_leg_uadjstd_fctv_dt_vec) = self.ntnl_amt_frst_leg_uadjstd_fctv_dt { for item in ntnl_amt_frst_leg_uadjstd_fctv_dt_vec { if !item.validate() { return false; } } }
		if let Some(ref ntnl_amt_frst_leg_uadjstd_end_dt_vec) = self.ntnl_amt_frst_leg_uadjstd_end_dt { for item in ntnl_amt_frst_leg_uadjstd_end_dt_vec { if !item.validate() { return false; } } }
		if let Some(ref ntnl_amt_frst_leg_schdl_amt_vec) = self.ntnl_amt_frst_leg_schdl_amt { for item in ntnl_amt_frst_leg_schdl_amt_vec { if !item.validate() { return false; } } }
		if let Some(ref ntnl_qty_frst_leg_value) = self.ntnl_qty_frst_leg { if !ntnl_qty_frst_leg_value.validate() { return false; } }
		if let Some(ref ntnl_qty_frst_leg_uadjstd_fctv_dt_vec) = self.ntnl_qty_frst_leg_uadjstd_fctv_dt { for item in ntnl_qty_frst_leg_uadjstd_fctv_dt_vec { if !item.validate() { return false; } } }
		if let Some(ref ntnl_qty_frst_leg_uadjstd_end_dt_vec) = self.ntnl_qty_frst_leg_uadjstd_end_dt { for item in ntnl_qty_frst_leg_uadjstd_end_dt_vec { if !item.validate() { return false; } } }
		if let Some(ref ntnl_qty_frst_leg_schdl_qty_vec) = self.ntnl_qty_frst_leg_schdl_qty { for item in ntnl_qty_frst_leg_schdl_qty_vec { if !item.validate() { return false; } } }
		if let Some(ref ntnl_amt_scnd_leg_value) = self.ntnl_amt_scnd_leg { if !ntnl_amt_scnd_leg_value.validate() { return false; } }
		if let Some(ref ntnl_amt_scnd_leg_uadjstd_fctv_dt_vec) = self.ntnl_amt_scnd_leg_uadjstd_fctv_dt { for item in ntnl_amt_scnd_leg_uadjstd_fctv_dt_vec { if !item.validate() { return false; } } }
		if let Some(ref ntnl_amt_scnd_leg_uadjstd_end_dt_vec) = self.ntnl_amt_scnd_leg_uadjstd_end_dt { for item in ntnl_amt_scnd_leg_uadjstd_end_dt_vec { if !item.validate() { return false; } } }
		if let Some(ref ntnl_amt_scnd_leg_schdl_amt_vec) = self.ntnl_amt_scnd_leg_schdl_amt { for item in ntnl_amt_scnd_leg_schdl_amt_vec { if !item.validate() { return false; } } }
		if let Some(ref ntnl_qty_scnd_leg_value) = self.ntnl_qty_scnd_leg { if !ntnl_qty_scnd_leg_value.validate() { return false; } }
		if let Some(ref ntnl_qty_scnd_leg_uadjstd_fctv_dt_vec) = self.ntnl_qty_scnd_leg_uadjstd_fctv_dt { for item in ntnl_qty_scnd_leg_uadjstd_fctv_dt_vec { if !item.validate() { return false; } } }
		if let Some(ref ntnl_qty_scnd_leg_uadjstd_end_dt_vec) = self.ntnl_qty_scnd_leg_uadjstd_end_dt { for item in ntnl_qty_scnd_leg_uadjstd_end_dt_vec { if !item.validate() { return false; } } }
		if let Some(ref ntnl_qty_scnd_leg_schdl_qty_vec) = self.ntnl_qty_scnd_leg_schdl_qty { for item in ntnl_qty_scnd_leg_schdl_qty_vec { if !item.validate() { return false; } } }
		if let Some(ref othr_pmt_vec) = self.othr_pmt { for item in othr_pmt_vec { if !item.validate() { return false; } } }
		if let Some(ref intrst_fxd_rate_frst_leg_value) = self.intrst_fxd_rate_frst_leg { if !intrst_fxd_rate_frst_leg_value.validate() { return false; } }
		if let Some(ref intrst_fxd_rate_frst_leg_day_cnt_value) = self.intrst_fxd_rate_frst_leg_day_cnt { if !intrst_fxd_rate_frst_leg_day_cnt_value.validate() { return false; } }
		if let Some(ref intrst_fxd_rate_frst_leg_pmt_frqcy_unit_value) = self.intrst_fxd_rate_frst_leg_pmt_frqcy_unit { if !intrst_fxd_rate_frst_leg_pmt_frqcy_unit_value.validate() { return false; } }
		if let Some(ref intrst_fxd_rate_frst_leg_pmt_frqcy_val_value) = self.intrst_fxd_rate_frst_leg_pmt_frqcy_val { if !intrst_fxd_rate_frst_leg_pmt_frqcy_val_value.validate() { return false; } }
		if let Some(ref intrst_fltg_rate_frst_leg_id_value) = self.intrst_fltg_rate_frst_leg_id { if !intrst_fltg_rate_frst_leg_id_value.validate() { return false; } }
		if let Some(ref intrst_fltg_rate_frst_leg_cd_value) = self.intrst_fltg_rate_frst_leg_cd { if !intrst_fltg_rate_frst_leg_cd_value.validate() { return false; } }
		if let Some(ref intrst_fltg_rate_frst_leg_nm_value) = self.intrst_fltg_rate_frst_leg_nm { if !intrst_fltg_rate_frst_leg_nm_value.validate() { return false; } }
		if let Some(ref intrst_fltg_rate_frst_leg_day_cnt_value) = self.intrst_fltg_rate_frst_leg_day_cnt { if !intrst_fltg_rate_frst_leg_day_cnt_value.validate() { return false; } }
		if let Some(ref intrst_fltg_rate_frst_leg_pmt_frqcy_unit_value) = self.intrst_fltg_rate_frst_leg_pmt_frqcy_unit { if !intrst_fltg_rate_frst_leg_pmt_frqcy_unit_value.validate() { return false; } }
		if let Some(ref intrst_fltg_rate_frst_leg_pmt_frqcy_val_value) = self.intrst_fltg_rate_frst_leg_pmt_frqcy_val { if !intrst_fltg_rate_frst_leg_pmt_frqcy_val_value.validate() { return false; } }
		if let Some(ref intrst_fltg_rate_frst_leg_ref_prd_unit_value) = self.intrst_fltg_rate_frst_leg_ref_prd_unit { if !intrst_fltg_rate_frst_leg_ref_prd_unit_value.validate() { return false; } }
		if let Some(ref intrst_fltg_rate_frst_leg_ref_prd_val_value) = self.intrst_fltg_rate_frst_leg_ref_prd_val { if !intrst_fltg_rate_frst_leg_ref_prd_val_value.validate() { return false; } }
		if let Some(ref intrst_fltg_rate_frst_leg_rst_frqcy_unit_value) = self.intrst_fltg_rate_frst_leg_rst_frqcy_unit { if !intrst_fltg_rate_frst_leg_rst_frqcy_unit_value.validate() { return false; } }
		if let Some(ref intrst_fltg_rate_frst_leg_rst_frqcy_val_value) = self.intrst_fltg_rate_frst_leg_rst_frqcy_val { if !intrst_fltg_rate_frst_leg_rst_frqcy_val_value.validate() { return false; } }
		if let Some(ref intrst_fltg_rate_frst_leg_sprd_value) = self.intrst_fltg_rate_frst_leg_sprd { if !intrst_fltg_rate_frst_leg_sprd_value.validate() { return false; } }
		if let Some(ref intrst_rate_fxd_scnd_leg_value) = self.intrst_rate_fxd_scnd_leg { if !intrst_rate_fxd_scnd_leg_value.validate() { return false; } }
		if let Some(ref intrst_fxd_rate_scnd_leg_day_cnt_value) = self.intrst_fxd_rate_scnd_leg_day_cnt { if !intrst_fxd_rate_scnd_leg_day_cnt_value.validate() { return false; } }
		if let Some(ref intrst_fxd_rate_scnd_leg_pmt_frqcy_unit_value) = self.intrst_fxd_rate_scnd_leg_pmt_frqcy_unit { if !intrst_fxd_rate_scnd_leg_pmt_frqcy_unit_value.validate() { return false; } }
		if let Some(ref intrst_fxd_rate_scnd_leg_pmt_frqcy_val_value) = self.intrst_fxd_rate_scnd_leg_pmt_frqcy_val { if !intrst_fxd_rate_scnd_leg_pmt_frqcy_val_value.validate() { return false; } }
		if let Some(ref intrst_fltg_rate_scnd_leg_id_value) = self.intrst_fltg_rate_scnd_leg_id { if !intrst_fltg_rate_scnd_leg_id_value.validate() { return false; } }
		if let Some(ref intrst_fltg_rate_scnd_leg_cd_value) = self.intrst_fltg_rate_scnd_leg_cd { if !intrst_fltg_rate_scnd_leg_cd_value.validate() { return false; } }
		if let Some(ref intrst_fltg_rate_scnd_leg_nm_value) = self.intrst_fltg_rate_scnd_leg_nm { if !intrst_fltg_rate_scnd_leg_nm_value.validate() { return false; } }
		if let Some(ref intrst_fltg_rate_scnd_leg_day_cnt_value) = self.intrst_fltg_rate_scnd_leg_day_cnt { if !intrst_fltg_rate_scnd_leg_day_cnt_value.validate() { return false; } }
		if let Some(ref intrst_fltg_rate_scnd_leg_pmt_frqcy_unit_value) = self.intrst_fltg_rate_scnd_leg_pmt_frqcy_unit { if !intrst_fltg_rate_scnd_leg_pmt_frqcy_unit_value.validate() { return false; } }
		if let Some(ref intrst_fltg_rate_scnd_leg_pmt_frqcy_val_value) = self.intrst_fltg_rate_scnd_leg_pmt_frqcy_val { if !intrst_fltg_rate_scnd_leg_pmt_frqcy_val_value.validate() { return false; } }
		if let Some(ref intrst_fltg_rate_scnd_leg_ref_prd_unit_value) = self.intrst_fltg_rate_scnd_leg_ref_prd_unit { if !intrst_fltg_rate_scnd_leg_ref_prd_unit_value.validate() { return false; } }
		if let Some(ref intrst_fltg_rate_scnd_leg_ref_prd_val_value) = self.intrst_fltg_rate_scnd_leg_ref_prd_val { if !intrst_fltg_rate_scnd_leg_ref_prd_val_value.validate() { return false; } }
		if let Some(ref intrst_fltg_rate_scnd_leg_rst_frqcy_unit_value) = self.intrst_fltg_rate_scnd_leg_rst_frqcy_unit { if !intrst_fltg_rate_scnd_leg_rst_frqcy_unit_value.validate() { return false; } }
		if let Some(ref intrst_fltg_rate_scnd_leg_rst_frqcy_val_value) = self.intrst_fltg_rate_scnd_leg_rst_frqcy_val { if !intrst_fltg_rate_scnd_leg_rst_frqcy_val_value.validate() { return false; } }
		if let Some(ref intrst_fltg_rate_scnd_leg_sprd_value) = self.intrst_fltg_rate_scnd_leg_sprd { if !intrst_fltg_rate_scnd_leg_sprd_value.validate() { return false; } }
		if let Some(ref packg_sprd_value) = self.packg_sprd { if !packg_sprd_value.validate() { return false; } }
		if let Some(ref ccy_xchg_rate_value) = self.ccy_xchg_rate { if !ccy_xchg_rate_value.validate() { return false; } }
		if let Some(ref ccy_fwd_xchg_rate_value) = self.ccy_fwd_xchg_rate { if !ccy_fwd_xchg_rate_value.validate() { return false; } }
		if let Some(ref ccy_xchg_rate_bsis_value) = self.ccy_xchg_rate_bsis { if !ccy_xchg_rate_bsis_value.validate() { return false; } }
		if let Some(ref cmmdty_value) = self.cmmdty { if !cmmdty_value.validate() { return false; } }
		if let Some(ref nrgy_dlvry_pt_or_zone_vec) = self.nrgy_dlvry_pt_or_zone { for item in nrgy_dlvry_pt_or_zone_vec { if !item.validate() { return false; } } }
		if let Some(ref nrgy_intr_cnnctn_pt_value) = self.nrgy_intr_cnnctn_pt { if !nrgy_intr_cnnctn_pt_value.validate() { return false; } }
		if let Some(ref nrgy_ld_tp_value) = self.nrgy_ld_tp { if !nrgy_ld_tp_value.validate() { return false; } }
		if let Some(ref dlvry_attr_vec) = self.dlvry_attr { for item in dlvry_attr_vec { if !item.validate() { return false; } } }
		if let Some(ref optn_tp_value) = self.optn_tp { if !optn_tp_value.validate() { return false; } }
		if let Some(ref optn_exrc_style_vec) = self.optn_exrc_style { for item in optn_exrc_style_vec { if !item.validate() { return false; } } }
		if let Some(ref optn_strk_pric_value) = self.optn_strk_pric { if !optn_strk_pric_value.validate() { return false; } }
		if let Some(ref optn_strk_pric_schdl_uadjstd_fctv_dt_vec) = self.optn_strk_pric_schdl_uadjstd_fctv_dt { for item in optn_strk_pric_schdl_uadjstd_fctv_dt_vec { if !item.validate() { return false; } } }
		if let Some(ref optn_strk_pric_schdl_uadjstd_end_dt_vec) = self.optn_strk_pric_schdl_uadjstd_end_dt { for item in optn_strk_pric_schdl_uadjstd_end_dt_vec { if !item.validate() { return false; } } }
		if let Some(ref optn_strk_pric_schdl_amt_vec) = self.optn_strk_pric_schdl_amt { for item in optn_strk_pric_schdl_amt_vec { if !item.validate() { return false; } } }
		if let Some(ref optn_prm_amt_value) = self.optn_prm_amt { if !optn_prm_amt_value.validate() { return false; } }
		if let Some(ref optn_prm_pmt_dt_value) = self.optn_prm_pmt_dt { if !optn_prm_pmt_dt_value.validate() { return false; } }
		if let Some(ref optn_mtrty_dt_of_undrlyg_value) = self.optn_mtrty_dt_of_undrlyg { if !optn_mtrty_dt_of_undrlyg_value.validate() { return false; } }
		if let Some(ref cdt_snrty_value) = self.cdt_snrty { if !cdt_snrty_value.validate() { return false; } }
		if let Some(ref cdt_ref_pty_value) = self.cdt_ref_pty { if !cdt_ref_pty_value.validate() { return false; } }
		if let Some(ref cdt_srs_value) = self.cdt_srs { if !cdt_srs_value.validate() { return false; } }
		if let Some(ref cdt_vrsn_value) = self.cdt_vrsn { if !cdt_vrsn_value.validate() { return false; } }
		if let Some(ref cdt_indx_fctr_value) = self.cdt_indx_fctr { if !cdt_indx_fctr_value.validate() { return false; } }
		if let Some(ref cdt_trch_value) = self.cdt_trch { if !cdt_trch_value.validate() { return false; } }
		if let Some(ref lvl_value) = self.lvl { if !lvl_value.validate() { return false; } }
		return true
	}
}


// TransactionOperationType10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TransactionOperationType10Code {
	#[default]
	#[serde(rename = "COMP")]
	CodeCOMP,
	#[serde(rename = "CORR")]
	CodeCORR,
	#[serde(rename = "EROR")]
	CodeEROR,
	#[serde(rename = "MODI")]
	CodeMODI,
	#[serde(rename = "NEWT")]
	CodeNEWT,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "POSC")]
	CodePOSC,
	#[serde(rename = "REVI")]
	CodeREVI,
	#[serde(rename = "TERM")]
	CodeTERM,
	#[serde(rename = "VALU")]
	CodeVALU,
	#[serde(rename = "MARU")]
	CodeMARU,
	#[serde(rename = "PRTO")]
	CodePRTO,
}

impl TransactionOperationType10Code {
	pub fn validate(&self) -> bool {
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


// UTIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UTIIdentifier {
	#[serde(rename = "$value")]
	pub uti_identifier: String,
}

impl UTIIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z0-9]{18}[0-9]{2}[A-Z0-9]{0,32}").unwrap();
		if !pattern.is_match(&self.uti_identifier) {
			return false
		}
		return true
	}
}


// UnderlyingIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum UnderlyingIdentification1Code {
	#[default]
	#[serde(rename = "UKWN")]
	CodeUKWN,
	#[serde(rename = "BSKT")]
	CodeBSKT,
	#[serde(rename = "INDX")]
	CodeINDX,
}

impl UnderlyingIdentification1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// UniqueProductIdentifier1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UniqueProductIdentifier1Choice {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max52Text>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification175>,
}

impl UniqueProductIdentifier1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// UniqueProductIdentifier2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UniqueProductIdentifier2Choice {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max52Text>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification185>,
}

impl UniqueProductIdentifier2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// UniqueTransactionIdentifier1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UniqueTransactionIdentifier1Choice {
	#[serde(rename = "UnqTxIdr", skip_serializing_if = "Option::is_none")]
	pub unq_tx_idr: Option<UTIIdentifier>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification179>,
}

impl UniqueTransactionIdentifier1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref unq_tx_idr_value) = self.unq_tx_idr { if !unq_tx_idr_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// UniqueTransactionIdentifier2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UniqueTransactionIdentifier2Choice {
	#[serde(rename = "UnqTxIdr", skip_serializing_if = "Option::is_none")]
	pub unq_tx_idr: Option<UTIIdentifier>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification175>,
}

impl UniqueTransactionIdentifier2Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref unq_tx_idr_value) = self.unq_tx_idr { if !unq_tx_idr_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// UnitOfMeasure8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitOfMeasure8Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalUnitOfMeasure1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification175>,
}

impl UnitOfMeasure8Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// ValuationMatchingCriteria1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ValuationMatchingCriteria1 {
	#[serde(rename = "CtrctVal", skip_serializing_if = "Option::is_none")]
	pub ctrct_val: Option<CompareAmountAndDirection3>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<CompareValuationType1>,
}

impl ValuationMatchingCriteria1 {
	pub fn validate(&self) -> bool {
		if let Some(ref ctrct_val_value) = self.ctrct_val { if !ctrct_val_value.validate() { return false; } }
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		return true
	}
}


// ValuationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ValuationType1Code {
	#[default]
	#[serde(rename = "CCPV")]
	CodeCCPV,
	#[serde(rename = "MTMA")]
	CodeMTMA,
	#[serde(rename = "MTMO")]
	CodeMTMO,
}

impl ValuationType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// WeekDay3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum WeekDay3Code {
	#[default]
	#[serde(rename = "ALLD")]
	CodeALLD,
	#[serde(rename = "XBHL")]
	CodeXBHL,
	#[serde(rename = "IBHL")]
	CodeIBHL,
	#[serde(rename = "FRID")]
	CodeFRID,
	#[serde(rename = "MOND")]
	CodeMOND,
	#[serde(rename = "SATD")]
	CodeSATD,
	#[serde(rename = "SUND")]
	CodeSUND,
	#[serde(rename = "THUD")]
	CodeTHUD,
	#[serde(rename = "TUED")]
	CodeTUED,
	#[serde(rename = "WEDD")]
	CodeWEDD,
	#[serde(rename = "WDAY")]
	CodeWDAY,
	#[serde(rename = "WEND")]
	CodeWEND,
}

impl WeekDay3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct YesNoIndicator {
	#[serde(rename = "$value")]
	pub yes_no_indicator: bool,
}

impl YesNoIndicator {
	pub fn validate(&self) -> bool {
		return true
	}
}
