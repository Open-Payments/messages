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


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "$value")]
	pub active_currency_code: String,
}


// ActiveOrHistoricCurrencyAnd19DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd19DecimalAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_and19_decimal_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAnd19DecimalAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd19DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_code: String,
}


// AgreementType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgreementType2Choice {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<ExternalAgreementType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max50Text>,
}


// AgriculturalCommodityDairy2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityDairy2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType20Code>,
}


// AgriculturalCommodityForestry2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityForestry2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType21Code>,
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


// AgriculturalCommodityLiveStock2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityLiveStock2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType22Code>,
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


// AgriculturalCommodityOther2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType49Code>,
}


// AgriculturalCommodityPotato2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityPotato2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType45Code>,
}


// AgriculturalCommoditySeafood2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommoditySeafood2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType23Code>,
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


// AllocationIndicator1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AllocationIndicator1Code {
	#[default]
	#[serde(rename = "POST")]
	CodePOST,
	#[serde(rename = "PREA")]
	CodePREA,
	#[serde(rename = "UNAL")]
	CodeUNAL,

}


// AmountAndDirection106 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection106 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAnd19DecimalAmount,
	#[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
	pub sgn: Option<bool>,
}


// AmountAndDirection109 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection109 {
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
	#[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
	pub sgn: Option<bool>,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
}


// AssetClassCommodity7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodity7Choice {
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
	pub ppr: Option<AssetClassCommodityPaper5Choice>,
	#[serde(rename = "Plprpln", skip_serializing_if = "Option::is_none")]
	pub plprpln: Option<AssetClassCommodityPolypropylene4Choice>,
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


// AssetClassCommodityC10Other1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityC10Other1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType11Code,
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


// AssetClassCommodityIndex1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityIndex1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType16Code,
}


// AssetClassCommodityIndustrialProduct2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityIndustrialProduct2Choice {
	#[serde(rename = "Cnstrctn", skip_serializing_if = "Option::is_none")]
	pub cnstrctn: Option<IndustrialProductCommodityConstruction2>,
	#[serde(rename = "Manfctg", skip_serializing_if = "Option::is_none")]
	pub manfctg: Option<IndustrialProductCommodityManufacturing2>,
}


// AssetClassCommodityInflation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityInflation1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType12Code,
}


// AssetClassCommodityMetal2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityMetal2Choice {
	#[serde(rename = "NonPrcs", skip_serializing_if = "Option::is_none")]
	pub non_prcs: Option<MetalCommodityNonPrecious2>,
	#[serde(rename = "Prcs", skip_serializing_if = "Option::is_none")]
	pub prcs: Option<MetalCommodityPrecious2>,
}


// AssetClassCommodityMultiCommodityExotic1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityMultiCommodityExotic1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType13Code,
}


// AssetClassCommodityOfficialEconomicStatistics1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityOfficialEconomicStatistics1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType14Code,
}


// AssetClassCommodityOther1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType15Code,
}


// AssetClassCommodityPaper5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityPaper5Choice {
	#[serde(rename = "CntnrBrd", skip_serializing_if = "Option::is_none")]
	pub cntnr_brd: Option<PaperCommodityContainerBoard2>,
	#[serde(rename = "Nwsprnt", skip_serializing_if = "Option::is_none")]
	pub nwsprnt: Option<PaperCommodityNewsprint2>,
	#[serde(rename = "Pulp", skip_serializing_if = "Option::is_none")]
	pub pulp: Option<PaperCommodityPulp2>,
	#[serde(rename = "RcvrdPpr", skip_serializing_if = "Option::is_none")]
	pub rcvrd_ppr: Option<PaperCommodityRecoveredPaper3>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<PaperCommodityOther1>,
}


// AssetClassCommodityPolypropylene4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityPolypropylene4Choice {
	#[serde(rename = "Plstc", skip_serializing_if = "Option::is_none")]
	pub plstc: Option<PolypropyleneCommodityPlastic2>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<PolypropyleneCommodityOther2>,
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


// AssetClassDetailedSubProductType29Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType29Code {
	#[default]
	#[serde(rename = "LAMP")]
	CodeLAMP,
	#[serde(rename = "OTHR")]
	CodeOTHR,

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


// AssetClassDetailedSubProductType30Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType30Code {
	#[default]
	#[serde(rename = "MWHT")]
	CodeMWHT,
	#[serde(rename = "OTHR")]
	CodeOTHR,

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


// AssetClassDetailedSubProductType33Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType33Code {
	#[default]
	#[serde(rename = "DBCR")]
	CodeDBCR,
	#[serde(rename = "OTHR")]
	CodeOTHR,

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


// AssetClassProductType11Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType11Code {
	#[default]
	#[serde(rename = "OTHC")]
	CodeOTHC,

}


// AssetClassProductType12Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType12Code {
	#[default]
	#[serde(rename = "INFL")]
	CodeINFL,

}


// AssetClassProductType13Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType13Code {
	#[default]
	#[serde(rename = "MCEX")]
	CodeMCEX,

}


// AssetClassProductType14Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType14Code {
	#[default]
	#[serde(rename = "OEST")]
	CodeOEST,

}


// AssetClassProductType15Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType15Code {
	#[default]
	#[serde(rename = "OTHR")]
	CodeOTHR,

}


// AssetClassProductType16Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType16Code {
	#[default]
	#[serde(rename = "INDX")]
	CodeINDX,

}


// AssetClassProductType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType1Code {
	#[default]
	#[serde(rename = "AGRI")]
	CodeAGRI,

}


// AssetClassProductType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType2Code {
	#[default]
	#[serde(rename = "NRGY")]
	CodeNRGY,

}


// AssetClassProductType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType3Code {
	#[default]
	#[serde(rename = "ENVR")]
	CodeENVR,

}


// AssetClassProductType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType4Code {
	#[default]
	#[serde(rename = "FRGT")]
	CodeFRGT,

}


// AssetClassProductType5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType5Code {
	#[default]
	#[serde(rename = "FRTL")]
	CodeFRTL,

}


// AssetClassProductType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType6Code {
	#[default]
	#[serde(rename = "INDP")]
	CodeINDP,

}


// AssetClassProductType7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType7Code {
	#[default]
	#[serde(rename = "METL")]
	CodeMETL,

}


// AssetClassProductType8Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType8Code {
	#[default]
	#[serde(rename = "PAPR")]
	CodePAPR,

}


// AssetClassProductType9Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassProductType9Code {
	#[default]
	#[serde(rename = "POLY")]
	CodePOLY,

}


// AssetClassSubProductType10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType10Code {
	#[default]
	#[serde(rename = "EMIS")]
	CodeEMIS,

}


// AssetClassSubProductType15Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType15Code {
	#[default]
	#[serde(rename = "NPRM")]
	CodeNPRM,

}


// AssetClassSubProductType16Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType16Code {
	#[default]
	#[serde(rename = "PRME")]
	CodePRME,

}


// AssetClassSubProductType18Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType18Code {
	#[default]
	#[serde(rename = "PLST")]
	CodePLST,

}


// AssetClassSubProductType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType1Code {
	#[default]
	#[serde(rename = "GROS")]
	CodeGROS,

}


// AssetClassSubProductType20Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType20Code {
	#[default]
	#[serde(rename = "DIRY")]
	CodeDIRY,

}


// AssetClassSubProductType21Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType21Code {
	#[default]
	#[serde(rename = "FRST")]
	CodeFRST,

}


// AssetClassSubProductType22Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType22Code {
	#[default]
	#[serde(rename = "LSTK")]
	CodeLSTK,

}


// AssetClassSubProductType23Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType23Code {
	#[default]
	#[serde(rename = "SEAF")]
	CodeSEAF,

}


// AssetClassSubProductType24Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType24Code {
	#[default]
	#[serde(rename = "COAL")]
	CodeCOAL,

}


// AssetClassSubProductType25Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType25Code {
	#[default]
	#[serde(rename = "DIST")]
	CodeDIST,

}


// AssetClassSubProductType26Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType26Code {
	#[default]
	#[serde(rename = "INRG")]
	CodeINRG,

}


// AssetClassSubProductType27Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType27Code {
	#[default]
	#[serde(rename = "LGHT")]
	CodeLGHT,

}


// AssetClassSubProductType28Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType28Code {
	#[default]
	#[serde(rename = "RNNG")]
	CodeRNNG,

}


// AssetClassSubProductType29Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType29Code {
	#[default]
	#[serde(rename = "CRBR")]
	CodeCRBR,

}


// AssetClassSubProductType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType2Code {
	#[default]
	#[serde(rename = "SOFT")]
	CodeSOFT,

}


// AssetClassSubProductType30Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType30Code {
	#[default]
	#[serde(rename = "WTHR")]
	CodeWTHR,

}


// AssetClassSubProductType31Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType31Code {
	#[default]
	#[serde(rename = "DRYF")]
	CodeDRYF,

}


// AssetClassSubProductType32Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType32Code {
	#[default]
	#[serde(rename = "WETF")]
	CodeWETF,

}


// AssetClassSubProductType33Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType33Code {
	#[default]
	#[serde(rename = "CSTR")]
	CodeCSTR,

}


// AssetClassSubProductType34Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType34Code {
	#[default]
	#[serde(rename = "MFTG")]
	CodeMFTG,

}


// AssetClassSubProductType35Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType35Code {
	#[default]
	#[serde(rename = "CBRD")]
	CodeCBRD,

}


// AssetClassSubProductType36Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType36Code {
	#[default]
	#[serde(rename = "NSPT")]
	CodeNSPT,

}


// AssetClassSubProductType37Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType37Code {
	#[default]
	#[serde(rename = "PULP")]
	CodePULP,

}


// AssetClassSubProductType39Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType39Code {
	#[default]
	#[serde(rename = "AMMO")]
	CodeAMMO,

}


// AssetClassSubProductType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType3Code {
	#[default]
	#[serde(rename = "OOLI")]
	CodeOOLI,

}


// AssetClassSubProductType40Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType40Code {
	#[default]
	#[serde(rename = "DAPH")]
	CodeDAPH,

}


// AssetClassSubProductType41Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType41Code {
	#[default]
	#[serde(rename = "PTSH")]
	CodePTSH,

}


// AssetClassSubProductType42Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType42Code {
	#[default]
	#[serde(rename = "SLPH")]
	CodeSLPH,

}


// AssetClassSubProductType43Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType43Code {
	#[default]
	#[serde(rename = "UREA")]
	CodeUREA,

}


// AssetClassSubProductType44Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType44Code {
	#[default]
	#[serde(rename = "UAAN")]
	CodeUAAN,

}


// AssetClassSubProductType45Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType45Code {
	#[default]
	#[serde(rename = "POTA")]
	CodePOTA,

}


// AssetClassSubProductType46Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType46Code {
	#[default]
	#[serde(rename = "CSHP")]
	CodeCSHP,

}


// AssetClassSubProductType49Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType49Code {
	#[default]
	#[serde(rename = "OTHR")]
	CodeOTHR,

}


// AssetClassSubProductType50Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType50Code {
	#[default]
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "RCVP")]
	CodeRCVP,

}


// AssetClassSubProductType5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType5Code {
	#[default]
	#[serde(rename = "GRIN")]
	CodeGRIN,

}


// AssetClassSubProductType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType6Code {
	#[default]
	#[serde(rename = "ELEC")]
	CodeELEC,

}


// AssetClassSubProductType7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType7Code {
	#[default]
	#[serde(rename = "NGAS")]
	CodeNGAS,

}


// AssetClassSubProductType8Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType8Code {
	#[default]
	#[serde(rename = "OILP")]
	CodeOILP,

}


// BaseOne18Rate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BaseOne18Rate {
	#[serde(rename = "$value")]
	pub base_one18_rate: f64,
}


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BaseOneRate {
	#[serde(rename = "$value")]
	pub base_one_rate: f64,
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


// CFIOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[serde(rename = "$value")]
	pub cfi_oct2015_identifier: String,
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


// ClearingAccountType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ClearingAccountType4Code {
	#[default]
	#[serde(rename = "CLIE")]
	CodeCLIE,
	#[serde(rename = "HOUS")]
	CodeHOUS,

}


// ClearingExceptionOrExemption2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingExceptionOrExemption2 {
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: NonClearingReason2,
	#[serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none")]
	pub othr_ctr_pty: Option<NonClearingReason2>,
}


// ClearingExceptionOrExemption3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingExceptionOrExemption3Choice {
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<NoReasonCode>,
	#[serde(rename = "CtrPties", skip_serializing_if = "Option::is_none")]
	pub ctr_pties: Option<ClearingExceptionOrExemption2>,
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


// ClearingPartyAndTime21Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingPartyAndTime21Choice {
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<NoReasonCode>,
	#[serde(rename = "Dtls", skip_serializing_if = "Option::is_none")]
	pub dtls: Option<ClearingPartyAndTime22>,
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


// ClearingPartyAndTime22Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingPartyAndTime22Choice {
	#[serde(rename = "Rsn", skip_serializing_if = "Option::is_none")]
	pub rsn: Option<NoReasonCode>,
	#[serde(rename = "Dtls", skip_serializing_if = "Option::is_none")]
	pub dtls: Option<ClearingPartyAndTime23>,
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


// CollateralPortfolioCode6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralPortfolioCode6Choice {
	#[serde(rename = "Prtfl", skip_serializing_if = "Option::is_none")]
	pub prtfl: Option<PortfolioCode3Choice>,
	#[serde(rename = "MrgnPrtflCd", skip_serializing_if = "Option::is_none")]
	pub mrgn_prtfl_cd: Option<MarginPortfolio4>,
}


// CommonTradeDataReport71 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommonTradeDataReport71 {
	#[serde(rename = "CtrctData", skip_serializing_if = "Option::is_none")]
	pub ctrct_data: Option<ContractType15>,
	#[serde(rename = "TxData")]
	pub tx_data: TradeTransaction50,
}


// ContractType15 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractType15 {
	#[serde(rename = "CtrctTp", skip_serializing_if = "Option::is_none")]
	pub ctrct_tp: Option<FinancialInstrumentContractType2Code>,
	#[serde(rename = "AsstClss", skip_serializing_if = "Option::is_none")]
	pub asst_clss: Option<ProductType4Code>,
	#[serde(rename = "PdctClssfctn", skip_serializing_if = "Option::is_none")]
	pub pdct_clssfctn: Option<CFIOct2015Identifier>,
	#[serde(rename = "PdctId", skip_serializing_if = "Option::is_none")]
	pub pdct_id: Option<SecurityIdentification46>,
	#[serde(rename = "UndrlygInstrm", skip_serializing_if = "Option::is_none")]
	pub undrlyg_instrm: Option<SecurityIdentification41Choice>,
	#[serde(rename = "UndrlygAsstTradgPltfmIdr", skip_serializing_if = "Option::is_none")]
	pub undrlyg_asst_tradg_pltfm_idr: Option<MICIdentifier>,
	#[serde(rename = "UndrlygAsstPricSrc", skip_serializing_if = "Option::is_none")]
	pub undrlyg_asst_pric_src: Option<Max50Text>,
	#[serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none")]
	pub sttlm_ccy: Option<CurrencyExchange23>,
	#[serde(rename = "SttlmCcyScndLeg", skip_serializing_if = "Option::is_none")]
	pub sttlm_ccy_scnd_leg: Option<CurrencyExchange23>,
	#[serde(rename = "PlcOfSttlm", skip_serializing_if = "Option::is_none")]
	pub plc_of_sttlm: Option<CountryCode>,
	#[serde(rename = "DerivBasedOnCrptAsst", skip_serializing_if = "Option::is_none")]
	pub deriv_based_on_crpt_asst: Option<bool>,
}


// ContractValuationData8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractValuationData8 {
	#[serde(rename = "CtrctVal", skip_serializing_if = "Option::is_none")]
	pub ctrct_val: Option<AmountAndDirection109>,
	#[serde(rename = "TmStmp", skip_serializing_if = "Option::is_none")]
	pub tm_stmp: Option<String>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<ValuationType1Code>,
	#[serde(rename = "Dlta", skip_serializing_if = "Option::is_none")]
	pub dlta: Option<f64>,
}


// Counterparty45 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Counterparty45 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification248Choice,
	#[serde(rename = "Ntr", skip_serializing_if = "Option::is_none")]
	pub ntr: Option<CounterpartyTradeNature15Choice>,
	#[serde(rename = "TradgCpcty", skip_serializing_if = "Option::is_none")]
	pub tradg_cpcty: Option<TradingCapacity7Code>,
	#[serde(rename = "DrctnOrSd", skip_serializing_if = "Option::is_none")]
	pub drctn_or_sd: Option<Direction4Choice>,
	#[serde(rename = "TradrLctn", skip_serializing_if = "Option::is_none")]
	pub tradr_lctn: Option<CountryCode>,
	#[serde(rename = "BookgLctn", skip_serializing_if = "Option::is_none")]
	pub bookg_lctn: Option<CountryCode>,
	#[serde(rename = "RptgXmptn", skip_serializing_if = "Option::is_none")]
	pub rptg_xmptn: Option<ReportingExemption1>,
}


// Counterparty46 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Counterparty46 {
	#[serde(rename = "IdTp", skip_serializing_if = "Option::is_none")]
	pub id_tp: Option<PartyIdentification248Choice>,
	#[serde(rename = "Ntr", skip_serializing_if = "Option::is_none")]
	pub ntr: Option<CounterpartyTradeNature15Choice>,
	#[serde(rename = "RptgOblgtn", skip_serializing_if = "Option::is_none")]
	pub rptg_oblgtn: Option<bool>,
}


// CounterpartySpecificData36 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartySpecificData36 {
	#[serde(rename = "CtrPty")]
	pub ctr_pty: TradeCounterpartyReport20,
	#[serde(rename = "Valtn", skip_serializing_if = "Option::is_none")]
	pub valtn: Option<ContractValuationData8>,
	#[serde(rename = "RptgTmStmp", skip_serializing_if = "Option::is_none")]
	pub rptg_tm_stmp: Option<String>,
}


// CounterpartyTradeNature15Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyTradeNature15Choice {
	#[serde(rename = "FI", skip_serializing_if = "Option::is_none")]
	pub fi: Option<FinancialInstitutionSector1>,
	#[serde(rename = "NFI", skip_serializing_if = "Option::is_none")]
	pub nfi: Option<NonFinancialInstitutionSector10>,
	#[serde(rename = "CntrlCntrPty", skip_serializing_if = "Option::is_none")]
	pub cntrl_cntr_pty: Option<NoReasonCode>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<NoReasonCode>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// CountrySubDivisionCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountrySubDivisionCode {
	#[serde(rename = "$value")]
	pub country_sub_division_code: String,
}


// CreditDerivative4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditDerivative4 {
	#[serde(rename = "Snrty", skip_serializing_if = "Option::is_none")]
	pub snrty: Option<DebtInstrumentSeniorityType2Code>,
	#[serde(rename = "RefPty", skip_serializing_if = "Option::is_none")]
	pub ref_pty: Option<DerivativePartyIdentification1Choice>,
	#[serde(rename = "PmtFrqcy", skip_serializing_if = "Option::is_none")]
	pub pmt_frqcy: Option<Frequency13Code>,
	#[serde(rename = "ClctnBsis", skip_serializing_if = "Option::is_none")]
	pub clctn_bsis: Option<Max35Text>,
	#[serde(rename = "Srs", skip_serializing_if = "Option::is_none")]
	pub srs: Option<f64>,
	#[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
	pub vrsn: Option<f64>,
	#[serde(rename = "IndxFctr", skip_serializing_if = "Option::is_none")]
	pub indx_fctr: Option<f64>,
	#[serde(rename = "Trch", skip_serializing_if = "Option::is_none")]
	pub trch: Option<TrancheIndicator3Choice>,
}


// CurrencyExchange22 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CurrencyExchange22 {
	#[serde(rename = "DlvrblCrossCcy", skip_serializing_if = "Option::is_none")]
	pub dlvrbl_cross_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "XchgRate", skip_serializing_if = "Option::is_none")]
	pub xchg_rate: Option<f64>,
	#[serde(rename = "FwdXchgRate", skip_serializing_if = "Option::is_none")]
	pub fwd_xchg_rate: Option<f64>,
	#[serde(rename = "XchgRateBsis", skip_serializing_if = "Option::is_none")]
	pub xchg_rate_bsis: Option<ExchangeRateBasis1Choice>,
	#[serde(rename = "FxgDt", skip_serializing_if = "Option::is_none")]
	pub fxg_dt: Option<String>,
}


// CurrencyExchange23 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CurrencyExchange23 {
	#[serde(rename = "Ccy")]
	pub ccy: ActiveOrHistoricCurrencyCode,
	#[serde(rename = "XchgRate", skip_serializing_if = "Option::is_none")]
	pub xchg_rate: Option<f64>,
	#[serde(rename = "FwdXchgRate", skip_serializing_if = "Option::is_none")]
	pub fwd_xchg_rate: Option<f64>,
	#[serde(rename = "XchgRateBsis", skip_serializing_if = "Option::is_none")]
	pub xchg_rate_bsis: Option<ExchangeRateBasis1Choice>,
	#[serde(rename = "FxgDt", skip_serializing_if = "Option::is_none")]
	pub fxg_dt: Option<String>,
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


// DateAndDateTime2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTime2Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<String>,
}


// DatePeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DatePeriod1 {
	#[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
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


// DeliveryInterconnectionPoint1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DeliveryInterconnectionPoint1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<EICIdentifier>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max52Text>,
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


// DerivativesTradeReportV04 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativesTradeReportV04 {
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: TradeReportHeader4,
	#[serde(rename = "TradData")]
	pub trad_data: TradeData59Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// Direction2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Direction2 {
	#[serde(rename = "DrctnOfTheFrstLeg")]
	pub drctn_of_the_frst_leg: OptionParty3Code,
	#[serde(rename = "DrctnOfTheScndLeg", skip_serializing_if = "Option::is_none")]
	pub drctn_of_the_scnd_leg: Option<OptionParty3Code>,
}


// Direction4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Direction4Choice {
	#[serde(rename = "Drctn", skip_serializing_if = "Option::is_none")]
	pub drctn: Option<Direction2>,
	#[serde(rename = "CtrPtySd", skip_serializing_if = "Option::is_none")]
	pub ctr_pty_sd: Option<OptionParty1Code>,
}


// DisseminationData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DisseminationData1 {
	#[serde(rename = "DssmntnIdr")]
	pub dssmntn_idr: Max52Text,
	#[serde(rename = "OrgnlDssmntnIdr", skip_serializing_if = "Option::is_none")]
	pub orgnl_dssmntn_idr: Option<Max52Text>,
	#[serde(rename = "TmStmp")]
	pub tm_stmp: String,
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


// EICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EICIdentifier {
	#[serde(rename = "$value")]
	pub eic_identifier: String,
}


// EmbeddedType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum EmbeddedType1Code {
	#[default]
	#[serde(rename = "CANC")]
	CodeCANC,
	#[serde(rename = "EXTD")]
	CodeEXTD,
	#[serde(rename = "OPET")]
	CodeOPET,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "MDET")]
	CodeMDET,

}


// EnergyCommodityCoal2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityCoal2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType24Code>,
}


// EnergyCommodityDistillates2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityDistillates2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType25Code>,
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


// EnergyCommodityInterEnergy2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityInterEnergy2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType26Code>,
}


// EnergyCommodityLightEnd2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityLightEnd2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType27Code>,
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


// EnergyCommodityOther2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType49Code>,
}


// EnergyCommodityRenewableEnergy2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityRenewableEnergy2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType28Code>,
}


// EnergyDeliveryAttribute10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyDeliveryAttribute10 {
	#[serde(rename = "DlvryIntrvl", skip_serializing_if = "Option::is_none")]
	pub dlvry_intrvl: Option<Vec<TimePeriodDetails1>>,
	#[serde(rename = "DlvryDt", skip_serializing_if = "Option::is_none")]
	pub dlvry_dt: Option<DatePeriod1>,
	#[serde(rename = "Drtn", skip_serializing_if = "Option::is_none")]
	pub drtn: Option<DurationType1Code>,
	#[serde(rename = "WkDay", skip_serializing_if = "Option::is_none")]
	pub wk_day: Option<Vec<WeekDay3Code>>,
	#[serde(rename = "DlvryCpcty", skip_serializing_if = "Option::is_none")]
	pub dlvry_cpcty: Option<Quantity47Choice>,
	#[serde(rename = "QtyUnit", skip_serializing_if = "Option::is_none")]
	pub qty_unit: Option<EnergyQuantityUnit2Choice>,
	#[serde(rename = "PricTmIntrvlQty", skip_serializing_if = "Option::is_none")]
	pub pric_tm_intrvl_qty: Option<AmountAndDirection106>,
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


// EnergyQuantityUnit2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyQuantityUnit2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<EnergyQuantityUnit2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max52Text>,
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


// EnergySpecificAttribute9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergySpecificAttribute9 {
	#[serde(rename = "DlvryPtOrZone", skip_serializing_if = "Option::is_none")]
	pub dlvry_pt_or_zone: Option<Vec<DeliveryInterconnectionPoint1Choice>>,
	#[serde(rename = "IntrCnnctnPt", skip_serializing_if = "Option::is_none")]
	pub intr_cnnctn_pt: Option<DeliveryInterconnectionPoint1Choice>,
	#[serde(rename = "LdTp", skip_serializing_if = "Option::is_none")]
	pub ld_tp: Option<EnergyLoadType1Code>,
	#[serde(rename = "DlvryAttr", skip_serializing_if = "Option::is_none")]
	pub dlvry_attr: Option<Vec<EnergyDeliveryAttribute10>>,
}


// EnvironmentCommodityOther2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType3Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType49Code>,
}


// EnvironmentalCommodityCarbonRelated2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCommodityCarbonRelated2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType3Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType29Code>,
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


// EnvironmentalCommodityWeather2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCommodityWeather2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType3Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType30Code>,
}


// EventIdentifier1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EventIdentifier1Choice {
	#[serde(rename = "EvtIdr", skip_serializing_if = "Option::is_none")]
	pub evt_idr: Option<UTIIdentifier>,
	#[serde(rename = "PstTradRskRdctnIdr", skip_serializing_if = "Option::is_none")]
	pub pst_trad_rsk_rdctn_idr: Option<PostTradeRiskReductionIdentifier1>,
}


// ExchangeRateBasis1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExchangeRateBasis1 {
	#[serde(rename = "BaseCcy")]
	pub base_ccy: ActiveCurrencyCode,
	#[serde(rename = "QtdCcy")]
	pub qtd_ccy: ActiveCurrencyCode,
}


// ExchangeRateBasis1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExchangeRateBasis1Choice {
	#[serde(rename = "CcyPair", skip_serializing_if = "Option::is_none")]
	pub ccy_pair: Option<ExchangeRateBasis1>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max52Text>,
}


// ExerciseDate1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExerciseDate1Choice {
	#[serde(rename = "FrstExrcDt", skip_serializing_if = "Option::is_none")]
	pub frst_exrc_dt: Option<String>,
	#[serde(rename = "PdgDtAplbl", skip_serializing_if = "Option::is_none")]
	pub pdg_dt_aplbl: Option<PriceStatus2Code>,
}


// ExternalAgreementType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAgreementType1Code {
	#[serde(rename = "$value")]
	pub external_agreement_type1_code: String,
}


// ExternalBenchmarkCurveName1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalBenchmarkCurveName1Code {
	#[serde(rename = "$value")]
	pub external_benchmark_curve_name1_code: String,
}


// ExternalPartyRelationshipType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPartyRelationshipType1Code {
	#[serde(rename = "$value")]
	pub external_party_relationship_type1_code: String,
}


// ExternalUnitOfMeasure1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalUnitOfMeasure1Code {
	#[serde(rename = "$value")]
	pub external_unit_of_measure1_code: String,
}


// FertilizerCommodityAmmonia2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityAmmonia2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType39Code>,
}


// FertilizerCommodityDiammoniumPhosphate2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityDiammoniumPhosphate2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType40Code>,
}


// FertilizerCommodityOther2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType49Code>,
}


// FertilizerCommodityPotash2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityPotash2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType41Code>,
}


// FertilizerCommoditySulphur2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommoditySulphur2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType42Code>,
}


// FertilizerCommodityUrea2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityUrea2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType43Code>,
}


// FertilizerCommodityUreaAndAmmoniumNitrate2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityUreaAndAmmoniumNitrate2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType44Code>,
}


// FinancialInstitutionSector1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstitutionSector1 {
	#[serde(rename = "Sctr")]
	pub sctr: Vec<FinancialPartyClassification2Choice>,
	#[serde(rename = "ClrThrshld", skip_serializing_if = "Option::is_none")]
	pub clr_thrshld: Option<bool>,
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


// FinancialInstrumentQuantity32Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentQuantity32Choice {
	#[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
	pub unit: Option<f64>,
	#[serde(rename = "NmnlVal", skip_serializing_if = "Option::is_none")]
	pub nmnl_val: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
	#[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
	pub mntry_val: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
}


// FinancialPartyClassification2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialPartyClassification2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<FinancialPartySectorType3Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification175>,
}


// FinancialPartySectorType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FinancialPartySectorType3Code {
	#[default]
	#[serde(rename = "AIFD")]
	CodeAIFD,
	#[serde(rename = "CSDS")]
	CodeCSDS,
	#[serde(rename = "CCPS")]
	CodeCCPS,
	#[serde(rename = "CDTI")]
	CodeCDTI,
	#[serde(rename = "INUN")]
	CodeINUN,
	#[serde(rename = "ORPI")]
	CodeORPI,
	#[serde(rename = "INVF")]
	CodeINVF,
	#[serde(rename = "REIN")]
	CodeREIN,
	#[serde(rename = "UCIT")]
	CodeUCIT,
	#[serde(rename = "ASSU")]
	CodeASSU,
	#[serde(rename = "OTHR")]
	CodeOTHR,

}


// FixedRate10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FixedRate10 {
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<SecuritiesTransactionPrice14Choice>,
	#[serde(rename = "DayCnt", skip_serializing_if = "Option::is_none")]
	pub day_cnt: Option<InterestComputationMethodFormat7>,
	#[serde(rename = "PmtFrqcy", skip_serializing_if = "Option::is_none")]
	pub pmt_frqcy: Option<InterestRateFrequency3Choice>,
}


// FloatingRate13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FloatingRate13 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ISINOct2015Identifier>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max350Text>,
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<FloatingRateIdentification8Choice>,
	#[serde(rename = "RefPrd", skip_serializing_if = "Option::is_none")]
	pub ref_prd: Option<InterestRateContractTerm4>,
	#[serde(rename = "Sprd", skip_serializing_if = "Option::is_none")]
	pub sprd: Option<SecuritiesTransactionPrice20Choice>,
	#[serde(rename = "DayCnt", skip_serializing_if = "Option::is_none")]
	pub day_cnt: Option<InterestComputationMethodFormat7>,
	#[serde(rename = "PmtFrqcy", skip_serializing_if = "Option::is_none")]
	pub pmt_frqcy: Option<InterestRateFrequency3Choice>,
	#[serde(rename = "RstFrqcy", skip_serializing_if = "Option::is_none")]
	pub rst_frqcy: Option<InterestRateFrequency3Choice>,
	#[serde(rename = "NxtFltgRst", skip_serializing_if = "Option::is_none")]
	pub nxt_fltg_rst: Option<ResetDateAndValue1>,
	#[serde(rename = "LastFltgRst", skip_serializing_if = "Option::is_none")]
	pub last_fltg_rst: Option<ResetDateAndValue1>,
}


// FloatingRateIdentification8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FloatingRateIdentification8Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalBenchmarkCurveName1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max350Text>,
}


// FreightCommodityContainerShip2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityContainerShip2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType4Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType46Code>,
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


// FreightCommodityOther2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType4Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType49Code>,
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


// Frequency19Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Frequency19Code {
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
	#[serde(rename = "HOUL")]
	CodeHOUL,
	#[serde(rename = "ODMD")]
	CodeODMD,

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


// GenericIdentification179 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification179 {
	#[serde(rename = "Id")]
	pub id: Max52Text,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}


// GenericIdentification184 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification184 {
	#[serde(rename = "Id")]
	pub id: Max210Text,
	#[serde(rename = "Src")]
	pub src: Max100Text,
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


// ISINOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "$value")]
	pub isin_oct2015_identifier: String,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}


// ISOTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISOTime {
	#[serde(rename = "$value")]
	pub iso_time: String,
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


// IndustrialProductCommodityConstruction2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndustrialProductCommodityConstruction2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType6Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType33Code>,
}


// IndustrialProductCommodityManufacturing2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndustrialProductCommodityManufacturing2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType6Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType34Code>,
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


// InterestComputationMethodFormat7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestComputationMethodFormat7 {
	#[serde(rename = "Cd")]
	pub cd: InterestComputationMethod4Code,
	#[serde(rename = "Nrrtv", skip_serializing_if = "Option::is_none")]
	pub nrrtv: Option<Max1000Text>,
}


// InterestRate33Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRate33Choice {
	#[serde(rename = "Fxd", skip_serializing_if = "Option::is_none")]
	pub fxd: Option<FixedRate10>,
	#[serde(rename = "Fltg", skip_serializing_if = "Option::is_none")]
	pub fltg: Option<FloatingRate13>,
}


// InterestRateContractTerm4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRateContractTerm4 {
	#[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
	pub unit: Option<Frequency13Code>,
	#[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
	pub val: Option<f64>,
}


// InterestRateFrequency3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRateFrequency3Choice {
	#[serde(rename = "Term", skip_serializing_if = "Option::is_none")]
	pub term: Option<InterestRateContractTerm4>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max52Text>,
}


// InterestRateLegs14 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRateLegs14 {
	#[serde(rename = "FrstLeg", skip_serializing_if = "Option::is_none")]
	pub frst_leg: Option<InterestRate33Choice>,
	#[serde(rename = "ScndLeg", skip_serializing_if = "Option::is_none")]
	pub scnd_leg: Option<InterestRate33Choice>,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}


// LegalPersonIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LegalPersonIdentification1 {
	#[serde(rename = "Id")]
	pub id: OrganisationIdentification15Choice,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
}


// LongFraction19DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LongFraction19DecimalNumber {
	#[serde(rename = "$value")]
	pub long_fraction19_decimal_number: f64,
}


// MICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MICIdentifier {
	#[serde(rename = "$value")]
	pub mic_identifier: String,
}


// MarginPortfolio4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarginPortfolio4 {
	#[serde(rename = "InitlMrgnPrtflCd", skip_serializing_if = "Option::is_none")]
	pub initl_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
	#[serde(rename = "VartnMrgnPrtflCd", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
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


// Max1000Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max1000Text {
	#[serde(rename = "$value")]
	pub max1000_text: String,
}


// Max100Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max100Text {
	#[serde(rename = "$value")]
	pub max100_text: String,
}


// Max105Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max105Text {
	#[serde(rename = "$value")]
	pub max105_text: String,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
}


// Max210Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max210Text {
	#[serde(rename = "$value")]
	pub max210_text: String,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}


// Max3Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max3Number {
	#[serde(rename = "$value")]
	pub max3_number: f64,
}


// Max4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "$value")]
	pub max4_alpha_numeric_text: String,
}


// Max4Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4Text {
	#[serde(rename = "$value")]
	pub max4_text: String,
}


// Max500Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max500Text {
	#[serde(rename = "$value")]
	pub max500_text: String,
}


// Max50Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max50Text {
	#[serde(rename = "$value")]
	pub max50_text: String,
}


// Max52Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max52Text {
	#[serde(rename = "$value")]
	pub max52_text: String,
}


// Max5NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max5NumericText {
	#[serde(rename = "$value")]
	pub max5_numeric_text: String,
}


// Max72Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max72Text {
	#[serde(rename = "$value")]
	pub max72_text: String,
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


// ModificationLevel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ModificationLevel1Code {
	#[default]
	#[serde(rename = "PSTN")]
	CodePSTN,
	#[serde(rename = "TCTN")]
	CodeTCTN,

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


// NaturalPersonIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NaturalPersonIdentification3 {
	#[serde(rename = "Id")]
	pub id: NaturalPersonIdentification2,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
}


// NoReasonCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NoReasonCode {
	#[default]
	#[serde(rename = "NORE")]
	CodeNORE,

}


// NonClearingReason2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NonClearingReason2 {
	#[serde(rename = "ClrXmptnXcptn")]
	pub clr_xmptn_xcptn: Vec<ClearingExemptionException1Code>,
	#[serde(rename = "NonClrRsnInf", skip_serializing_if = "Option::is_none")]
	pub non_clr_rsn_inf: Option<Max350Text>,
}


// NonFinancialInstitutionSector10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NonFinancialInstitutionSector10 {
	#[serde(rename = "Sctr")]
	pub sctr: Vec<GenericIdentification175>,
	#[serde(rename = "ClrThrshld", skip_serializing_if = "Option::is_none")]
	pub clr_thrshld: Option<bool>,
	#[serde(rename = "DrctlyLkdActvty", skip_serializing_if = "Option::is_none")]
	pub drctly_lkd_actvty: Option<bool>,
	#[serde(rename = "FdrlInstn", skip_serializing_if = "Option::is_none")]
	pub fdrl_instn: Option<bool>,
}


// NotApplicable1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NotApplicable1Code {
	#[default]
	#[serde(rename = "NOAP")]
	CodeNOAP,

}


// NotionalAmount5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalAmount5 {
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<AmountAndDirection106>,
	#[serde(rename = "SchdlPrd", skip_serializing_if = "Option::is_none")]
	pub schdl_prd: Option<Vec<Schedule11>>,
}


// NotionalAmount6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalAmount6 {
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<AmountAndDirection106>,
	#[serde(rename = "SchdlPrd", skip_serializing_if = "Option::is_none")]
	pub schdl_prd: Option<Vec<Schedule11>>,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveOrHistoricCurrencyCode>,
}


// NotionalAmountLegs5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalAmountLegs5 {
	#[serde(rename = "FrstLeg", skip_serializing_if = "Option::is_none")]
	pub frst_leg: Option<NotionalAmount5>,
	#[serde(rename = "ScndLeg", skip_serializing_if = "Option::is_none")]
	pub scnd_leg: Option<NotionalAmount6>,
}


// NotionalQuantity9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalQuantity9 {
	#[serde(rename = "TtlQty", skip_serializing_if = "Option::is_none")]
	pub ttl_qty: Option<f64>,
	#[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
	pub unit_of_measr: Option<UnitOfMeasure8Choice>,
	#[serde(rename = "Dtls", skip_serializing_if = "Option::is_none")]
	pub dtls: Option<QuantityOrTerm1Choice>,
}


// NotionalQuantityLegs5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalQuantityLegs5 {
	#[serde(rename = "FrstLeg", skip_serializing_if = "Option::is_none")]
	pub frst_leg: Option<NotionalQuantity9>,
	#[serde(rename = "ScndLeg", skip_serializing_if = "Option::is_none")]
	pub scnd_leg: Option<NotionalQuantity9>,
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "$value")]
	pub number: f64,
}


// OptionBarrierLevel1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OptionBarrierLevel1Choice {
	#[serde(rename = "Sngl", skip_serializing_if = "Option::is_none")]
	pub sngl: Option<SecuritiesTransactionPrice23Choice>,
	#[serde(rename = "Mltpl", skip_serializing_if = "Option::is_none")]
	pub mltpl: Option<OptionMultipleBarrierLevels1>,
}


// OptionMultipleBarrierLevels1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OptionMultipleBarrierLevels1 {
	#[serde(rename = "LwrLvl")]
	pub lwr_lvl: SecuritiesTransactionPrice23Choice,
	#[serde(rename = "UpperLvl")]
	pub upper_lvl: SecuritiesTransactionPrice23Choice,
}


// OptionOrSwaption11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OptionOrSwaption11 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<OptionType2Code>,
	#[serde(rename = "MbddTp", skip_serializing_if = "Option::is_none")]
	pub mbdd_tp: Option<EmbeddedType1Code>,
	#[serde(rename = "ExrcStyle", skip_serializing_if = "Option::is_none")]
	pub exrc_style: Option<Vec<OptionStyle6Code>>,
	#[serde(rename = "ExrcDt", skip_serializing_if = "Option::is_none")]
	pub exrc_dt: Option<ExerciseDate1Choice>,
	#[serde(rename = "StrkPric", skip_serializing_if = "Option::is_none")]
	pub strk_pric: Option<SecuritiesTransactionPrice17Choice>,
	#[serde(rename = "StrkPricSchdl", skip_serializing_if = "Option::is_none")]
	pub strk_pric_schdl: Option<Vec<Schedule4>>,
	#[serde(rename = "CallAmt", skip_serializing_if = "Option::is_none")]
	pub call_amt: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
	#[serde(rename = "PutAmt", skip_serializing_if = "Option::is_none")]
	pub put_amt: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
	#[serde(rename = "PrmAmt", skip_serializing_if = "Option::is_none")]
	pub prm_amt: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
	#[serde(rename = "PrmPmtDt", skip_serializing_if = "Option::is_none")]
	pub prm_pmt_dt: Option<String>,
	#[serde(rename = "MtrtyDtOfUndrlyg", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt_of_undrlyg: Option<String>,
	#[serde(rename = "BrrrLvls", skip_serializing_if = "Option::is_none")]
	pub brrr_lvls: Option<OptionBarrierLevel1Choice>,
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


// OptionParty3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OptionParty3Code {
	#[default]
	#[serde(rename = "MAKE")]
	CodeMAKE,
	#[serde(rename = "TAKE")]
	CodeTAKE,

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


// OtherPayment5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherPayment5 {
	#[serde(rename = "PmtAmt", skip_serializing_if = "Option::is_none")]
	pub pmt_amt: Option<AmountAndDirection106>,
	#[serde(rename = "PmtTp", skip_serializing_if = "Option::is_none")]
	pub pmt_tp: Option<PaymentType5Choice>,
	#[serde(rename = "PmtDt", skip_serializing_if = "Option::is_none")]
	pub pmt_dt: Option<String>,
	#[serde(rename = "PmtPyer", skip_serializing_if = "Option::is_none")]
	pub pmt_pyer: Option<PartyIdentification236Choice>,
	#[serde(rename = "PmtRcvr", skip_serializing_if = "Option::is_none")]
	pub pmt_rcvr: Option<PartyIdentification236Choice>,
}


// PTRREvent2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PTRREvent2 {
	#[serde(rename = "Tchnq")]
	pub tchnq: RiskReductionService1Code,
	#[serde(rename = "SvcPrvdr", skip_serializing_if = "Option::is_none")]
	pub svc_prvdr: Option<OrganisationIdentification15Choice>,
}


// Package4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Package4 {
	#[serde(rename = "CmplxTradId", skip_serializing_if = "Option::is_none")]
	pub cmplx_trad_id: Option<Max100Text>,
	#[serde(rename = "FxSwpLkId", skip_serializing_if = "Option::is_none")]
	pub fx_swp_lk_id: Option<Max100Text>,
	#[serde(rename = "Pric", skip_serializing_if = "Option::is_none")]
	pub pric: Option<SecuritiesTransactionPrice17Choice>,
	#[serde(rename = "Sprd", skip_serializing_if = "Option::is_none")]
	pub sprd: Option<SecuritiesTransactionPrice20Choice>,
}


// Pagination1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Pagination1 {
	#[serde(rename = "PgNb")]
	pub pg_nb: Max5NumericText,
	#[serde(rename = "LastPgInd")]
	pub last_pg_ind: bool,
}


// PaperCommodityContainerBoard2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityContainerBoard2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType8Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType35Code>,
}


// PaperCommodityNewsprint2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityNewsprint2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType8Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType36Code>,
}


// PaperCommodityOther1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType8Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType49Code>,
}


// PaperCommodityPulp2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityPulp2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType8Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType37Code>,
}


// PaperCommodityRecoveredPaper3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityRecoveredPaper3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType8Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType50Code>,
}


// PartyIdentification236Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification236Choice {
	#[serde(rename = "Lgl", skip_serializing_if = "Option::is_none")]
	pub lgl: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "Ntrl", skip_serializing_if = "Option::is_none")]
	pub ntrl: Option<NaturalPersonIdentification2>,
}


// PartyIdentification248Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification248Choice {
	#[serde(rename = "Lgl", skip_serializing_if = "Option::is_none")]
	pub lgl: Option<LegalPersonIdentification1>,
	#[serde(rename = "Ntrl", skip_serializing_if = "Option::is_none")]
	pub ntrl: Option<NaturalPersonIdentification3>,
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


// PaymentType5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentType5Choice {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<PaymentType4Code>,
	#[serde(rename = "PrtryTp", skip_serializing_if = "Option::is_none")]
	pub prtry_tp: Option<Max4AlphaNumericText>,
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "$value")]
	pub percentage_rate: f64,
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


// PlusOrMinusIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "$value")]
	pub plus_or_minus_indicator: bool,
}


// PolypropyleneCommodityOther2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PolypropyleneCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType9Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType49Code>,
}


// PolypropyleneCommodityPlastic2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PolypropyleneCommodityPlastic2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType9Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType18Code>,
}


// PortfolioCode3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioCode3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Max52Text>,
	#[serde(rename = "NoPrtfl", skip_serializing_if = "Option::is_none")]
	pub no_prtfl: Option<NotApplicable1Code>,
}


// PortfolioCode5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioCode5Choice {
	#[serde(rename = "Prtfl", skip_serializing_if = "Option::is_none")]
	pub prtfl: Option<PortfolioIdentification3>,
	#[serde(rename = "NoPrtfl", skip_serializing_if = "Option::is_none")]
	pub no_prtfl: Option<NotApplicable1Code>,
}


// PortfolioIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioIdentification3 {
	#[serde(rename = "Cd")]
	pub cd: Max52Text,
	#[serde(rename = "PrtflTxXmptn", skip_serializing_if = "Option::is_none")]
	pub prtfl_tx_xmptn: Option<bool>,
}


// PostTradeRiskReductionIdentifier1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostTradeRiskReductionIdentifier1 {
	#[serde(rename = "Strr")]
	pub strr: LEIIdentifier,
	#[serde(rename = "Id")]
	pub id: Max52Text,
}


// PriceData2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceData2 {
	#[serde(rename = "Pric", skip_serializing_if = "Option::is_none")]
	pub pric: Option<SecuritiesTransactionPrice17Choice>,
	#[serde(rename = "SchdlPrd", skip_serializing_if = "Option::is_none")]
	pub schdl_prd: Option<Vec<Schedule1>>,
	#[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
	pub unit_of_measr: Option<UnitOfMeasure8Choice>,
	#[serde(rename = "PricMltplr", skip_serializing_if = "Option::is_none")]
	pub pric_mltplr: Option<f64>,
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


// PriceStatus2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PriceStatus2Code {
	#[default]
	#[serde(rename = "PNDG")]
	CodePNDG,

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


// Quantity47Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Quantity47Choice {
	#[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
	pub qty: Option<f64>,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max52Text>,
}


// QuantityOrTerm1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct QuantityOrTerm1Choice {
	#[serde(rename = "SchdlPrd", skip_serializing_if = "Option::is_none")]
	pub schdl_prd: Option<Vec<Schedule10>>,
	#[serde(rename = "Term", skip_serializing_if = "Option::is_none")]
	pub term: Option<QuantityTerm1>,
}


// QuantityTerm1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct QuantityTerm1 {
	#[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
	pub qty: Option<f64>,
	#[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
	pub unit_of_measr: Option<UnitOfMeasure8Choice>,
	#[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
	pub val: Option<f64>,
	#[serde(rename = "TmUnit", skip_serializing_if = "Option::is_none")]
	pub tm_unit: Option<Frequency19Code>,
}


// Reconciliation3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Reconciliation3Code {
	#[default]
	#[serde(rename = "DPRW")]
	CodeDPRW,
	#[serde(rename = "DPRV")]
	CodeDPRV,
	#[serde(rename = "DSMA")]
	CodeDSMA,
	#[serde(rename = "DSNM")]
	CodeDSNM,
	#[serde(rename = "NORE")]
	CodeNORE,
	#[serde(rename = "SSMA")]
	CodeSSMA,
	#[serde(rename = "SSPA")]
	CodeSSPA,
	#[serde(rename = "SPRW")]
	CodeSPRW,
	#[serde(rename = "SPRV")]
	CodeSPRV,
	#[serde(rename = "SSUN")]
	CodeSSUN,
	#[serde(rename = "SSNE")]
	CodeSSNE,

}


// ReportPeriodActivity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ReportPeriodActivity1Code {
	#[default]
	#[serde(rename = "NOTX")]
	CodeNOTX,

}


// ReportingExemption1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportingExemption1 {
	#[serde(rename = "Rsn")]
	pub rsn: Max4Text,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max1000Text>,
}


// ResetDateAndValue1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ResetDateAndValue1 {
	#[serde(rename = "Dt")]
	pub dt: String,
	#[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
	pub val: Option<f64>,
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


// Schedule1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Schedule1 {
	#[serde(rename = "UadjstdFctvDt")]
	pub uadjstd_fctv_dt: String,
	#[serde(rename = "UadjstdEndDt", skip_serializing_if = "Option::is_none")]
	pub uadjstd_end_dt: Option<String>,
	#[serde(rename = "Pric")]
	pub pric: SecuritiesTransactionPrice17Choice,
}


// Schedule10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Schedule10 {
	#[serde(rename = "Qty")]
	pub qty: f64,
	#[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
	pub unit_of_measr: Option<UnitOfMeasure8Choice>,
	#[serde(rename = "UadjstdFctvDt")]
	pub uadjstd_fctv_dt: String,
	#[serde(rename = "UadjstdEndDt", skip_serializing_if = "Option::is_none")]
	pub uadjstd_end_dt: Option<String>,
}


// Schedule11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Schedule11 {
	#[serde(rename = "UadjstdFctvDt")]
	pub uadjstd_fctv_dt: String,
	#[serde(rename = "UadjstdEndDt", skip_serializing_if = "Option::is_none")]
	pub uadjstd_end_dt: Option<String>,
	#[serde(rename = "Amt")]
	pub amt: AmountAndDirection106,
}


// Schedule4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Schedule4 {
	#[serde(rename = "UadjstdFctvDt")]
	pub uadjstd_fctv_dt: String,
	#[serde(rename = "UadjstdEndDt", skip_serializing_if = "Option::is_none")]
	pub uadjstd_end_dt: Option<String>,
	#[serde(rename = "Pric")]
	pub pric: SecuritiesTransactionPrice17Choice,
}


// SecuritiesTransactionPrice14Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice14Choice {
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "Dcml", skip_serializing_if = "Option::is_none")]
	pub dcml: Option<f64>,
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


// SecuritiesTransactionPrice20Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice20Choice {
	#[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
	pub mntry_val: Option<AmountAndDirection106>,
	#[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
	pub pctg: Option<f64>,
	#[serde(rename = "Dcml", skip_serializing_if = "Option::is_none")]
	pub dcml: Option<f64>,
	#[serde(rename = "BsisPtSprd", skip_serializing_if = "Option::is_none")]
	pub bsis_pt_sprd: Option<f64>,
}


// SecuritiesTransactionPrice23Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice23Choice {
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
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<SecuritiesTransactionPrice5>,
}


// SecuritiesTransactionPrice5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice5 {
	#[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
	pub val: Option<f64>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<Max35Text>,
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


// SecurityIdentification46 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification46 {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
	#[serde(rename = "UnqPdctIdr", skip_serializing_if = "Option::is_none")]
	pub unq_pdct_idr: Option<UniqueProductIdentifier2Choice>,
	#[serde(rename = "AltrntvInstrmId", skip_serializing_if = "Option::is_none")]
	pub altrntv_instrm_id: Option<Max105Text>,
	#[serde(rename = "PdctDesc", skip_serializing_if = "Option::is_none")]
	pub pdct_desc: Option<Max1000Text>,
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
	pub plc_and_nm: Option<Max350Text>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}


// TechnicalAttributes5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TechnicalAttributes5 {
	#[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
	pub tech_rcrd_id: Option<Max140Text>,
	#[serde(rename = "RcncltnFlg", skip_serializing_if = "Option::is_none")]
	pub rcncltn_flg: Option<Reconciliation3Code>,
	#[serde(rename = "RptRctTmStmp", skip_serializing_if = "Option::is_none")]
	pub rpt_rct_tm_stmp: Option<String>,
}


// TimePeriodDetails1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimePeriodDetails1 {
	#[serde(rename = "FrTm")]
	pub fr_tm: String,
	#[serde(rename = "ToTm", skip_serializing_if = "Option::is_none")]
	pub to_tm: Option<String>,
}


// TradeClearing11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeClearing11 {
	#[serde(rename = "ClrOblgtn", skip_serializing_if = "Option::is_none")]
	pub clr_oblgtn: Option<ClearingObligationType1Code>,
	#[serde(rename = "ClrSts", skip_serializing_if = "Option::is_none")]
	pub clr_sts: Option<Cleared23Choice>,
	#[serde(rename = "IntraGrp", skip_serializing_if = "Option::is_none")]
	pub intra_grp: Option<bool>,
}


// TradeConfirmation4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeConfirmation4Choice {
	#[serde(rename = "Confd", skip_serializing_if = "Option::is_none")]
	pub confd: Option<TradeConfirmation5>,
	#[serde(rename = "NonConfd", skip_serializing_if = "Option::is_none")]
	pub non_confd: Option<TradeNonConfirmation1>,
}


// TradeConfirmation5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeConfirmation5 {
	#[serde(rename = "Tp")]
	pub tp: TradeConfirmationType1Code,
	#[serde(rename = "TmStmp", skip_serializing_if = "Option::is_none")]
	pub tm_stmp: Option<String>,
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


// TradeConfirmationType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TradeConfirmationType2Code {
	#[default]
	#[serde(rename = "NCNF")]
	CodeNCNF,

}


// TradeCounterpartyRelationship1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeCounterpartyRelationship1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalPartyRelationshipType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max100Text>,
}


// TradeCounterpartyRelationshipRecord1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeCounterpartyRelationshipRecord1 {
	#[serde(rename = "StartRltshPty")]
	pub start_rltsh_pty: TradeCounterpartyType1Code,
	#[serde(rename = "EndRltshPty")]
	pub end_rltsh_pty: TradeCounterpartyType1Code,
	#[serde(rename = "RltshTp")]
	pub rltsh_tp: TradeCounterpartyRelationship1Choice,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max1000Text>,
}


// TradeCounterpartyReport20 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeCounterpartyReport20 {
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: Counterparty45,
	#[serde(rename = "OthrCtrPty")]
	pub othr_ctr_pty: Counterparty46,
	#[serde(rename = "Brkr", skip_serializing_if = "Option::is_none")]
	pub brkr: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "SubmitgAgt", skip_serializing_if = "Option::is_none")]
	pub submitg_agt: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "ClrMmb", skip_serializing_if = "Option::is_none")]
	pub clr_mmb: Option<PartyIdentification248Choice>,
	#[serde(rename = "Bnfcry", skip_serializing_if = "Option::is_none")]
	pub bnfcry: Option<Vec<PartyIdentification248Choice>>,
	#[serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none")]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "ExctnAgt", skip_serializing_if = "Option::is_none")]
	pub exctn_agt: Option<Vec<OrganisationIdentification15Choice>>,
	#[serde(rename = "RltshRcrd", skip_serializing_if = "Option::is_none")]
	pub rltsh_rcrd: Option<Vec<TradeCounterpartyRelationshipRecord1>>,
}


// TradeCounterpartyType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TradeCounterpartyType1Code {
	#[default]
	#[serde(rename = "BENE")]
	CodeBENE,
	#[serde(rename = "BROK")]
	CodeBROK,
	#[serde(rename = "CLEM")]
	CodeCLEM,
	#[serde(rename = "EXEA")]
	CodeEXEA,
	#[serde(rename = "OTHC")]
	CodeOTHC,
	#[serde(rename = "REPC")]
	CodeREPC,
	#[serde(rename = "SBMA")]
	CodeSBMA,
	#[serde(rename = "ERFR")]
	CodeERFR,

}


// TradeData43 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeData43 {
	#[serde(rename = "CtrPtySpcfcData")]
	pub ctr_pty_spcfc_data: Vec<CounterpartySpecificData36>,
	#[serde(rename = "CmonTradData")]
	pub cmon_trad_data: CommonTradeDataReport71,
	#[serde(rename = "Lvl", skip_serializing_if = "Option::is_none")]
	pub lvl: Option<ModificationLevel1Code>,
	#[serde(rename = "TechAttrbts", skip_serializing_if = "Option::is_none")]
	pub tech_attrbts: Option<TechnicalAttributes5>,
	#[serde(rename = "PblcDssmntnData", skip_serializing_if = "Option::is_none")]
	pub pblc_dssmntn_data: Option<DisseminationData1>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// TradeData59Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeData59Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[serde(rename = "Rpt", skip_serializing_if = "Option::is_none")]
	pub rpt: Option<Vec<TradeReport33Choice>>,
}


// TradeNonConfirmation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeNonConfirmation1 {
	#[serde(rename = "Tp")]
	pub tp: TradeConfirmationType2Code,
}


// TradeReport33Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeReport33Choice {
	#[serde(rename = "New", skip_serializing_if = "Option::is_none")]
	pub new: Option<TradeData43>,
	#[serde(rename = "Mod", skip_serializing_if = "Option::is_none")]
	pub mod_attr: Option<TradeData43>,
	#[serde(rename = "Crrctn", skip_serializing_if = "Option::is_none")]
	pub crrctn: Option<TradeData43>,
	#[serde(rename = "Termntn", skip_serializing_if = "Option::is_none")]
	pub termntn: Option<TradeData43>,
	#[serde(rename = "PosCmpnt", skip_serializing_if = "Option::is_none")]
	pub pos_cmpnt: Option<TradeData43>,
	#[serde(rename = "ValtnUpd", skip_serializing_if = "Option::is_none")]
	pub valtn_upd: Option<TradeData43>,
	#[serde(rename = "Cmprssn", skip_serializing_if = "Option::is_none")]
	pub cmprssn: Option<TradeData43>,
	#[serde(rename = "Err", skip_serializing_if = "Option::is_none")]
	pub err: Option<TradeData43>,
	#[serde(rename = "PortOut", skip_serializing_if = "Option::is_none")]
	pub port_out: Option<TradeData43>,
	#[serde(rename = "Rvv", skip_serializing_if = "Option::is_none")]
	pub rvv: Option<TradeData43>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<TradeData43>,
}


// TradeReportHeader4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeReportHeader4 {
	#[serde(rename = "RptExctnDt", skip_serializing_if = "Option::is_none")]
	pub rpt_exctn_dt: Option<String>,
	#[serde(rename = "MsgPgntn", skip_serializing_if = "Option::is_none")]
	pub msg_pgntn: Option<Pagination1>,
	#[serde(rename = "NbRcrds")]
	pub nb_rcrds: f64,
	#[serde(rename = "CmptntAuthrty", skip_serializing_if = "Option::is_none")]
	pub cmptnt_authrty: Option<Vec<Max100Text>>,
	#[serde(rename = "NewTradRpstryIdr", skip_serializing_if = "Option::is_none")]
	pub new_trad_rpstry_idr: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "RptgPurp", skip_serializing_if = "Option::is_none")]
	pub rptg_purp: Option<Vec<Max100Text>>,
}


// TradeTransaction50 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeTransaction50 {
	#[serde(rename = "TxId", skip_serializing_if = "Option::is_none")]
	pub tx_id: Option<UniqueTransactionIdentifier2Choice>,
	#[serde(rename = "ScndryTxId", skip_serializing_if = "Option::is_none")]
	pub scndry_tx_id: Option<Max72Text>,
	#[serde(rename = "PrrTxId", skip_serializing_if = "Option::is_none")]
	pub prr_tx_id: Option<UniqueTransactionIdentifier3Choice>,
	#[serde(rename = "SbsqntTxId", skip_serializing_if = "Option::is_none")]
	pub sbsqnt_tx_id: Option<UniqueTransactionIdentifier3Choice>,
	#[serde(rename = "CollPrtflCd", skip_serializing_if = "Option::is_none")]
	pub coll_prtfl_cd: Option<CollateralPortfolioCode6Choice>,
	#[serde(rename = "RptTrckgNb", skip_serializing_if = "Option::is_none")]
	pub rpt_trckg_nb: Option<Max52Text>,
	#[serde(rename = "PltfmIdr", skip_serializing_if = "Option::is_none")]
	pub pltfm_idr: Option<MICIdentifier>,
	#[serde(rename = "MrrrOrTrggrTx", skip_serializing_if = "Option::is_none")]
	pub mrrr_or_trggr_tx: Option<bool>,
	#[serde(rename = "TxPric", skip_serializing_if = "Option::is_none")]
	pub tx_pric: Option<PriceData2>,
	#[serde(rename = "NtnlAmt", skip_serializing_if = "Option::is_none")]
	pub ntnl_amt: Option<NotionalAmountLegs5>,
	#[serde(rename = "NtnlQty", skip_serializing_if = "Option::is_none")]
	pub ntnl_qty: Option<NotionalQuantityLegs5>,
	#[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
	pub qty: Option<FinancialInstrumentQuantity32Choice>,
	#[serde(rename = "DlvryTp", skip_serializing_if = "Option::is_none")]
	pub dlvry_tp: Option<PhysicalTransferType4Code>,
	#[serde(rename = "ExctnTmStmp", skip_serializing_if = "Option::is_none")]
	pub exctn_tm_stmp: Option<String>,
	#[serde(rename = "FctvDt", skip_serializing_if = "Option::is_none")]
	pub fctv_dt: Option<String>,
	#[serde(rename = "XprtnDt", skip_serializing_if = "Option::is_none")]
	pub xprtn_dt: Option<String>,
	#[serde(rename = "EarlyTermntnDt", skip_serializing_if = "Option::is_none")]
	pub early_termntn_dt: Option<String>,
	#[serde(rename = "SttlmDt", skip_serializing_if = "Option::is_none")]
	pub sttlm_dt: Option<Vec<String>>,
	#[serde(rename = "MstrAgrmt", skip_serializing_if = "Option::is_none")]
	pub mstr_agrmt: Option<MasterAgreement8>,
	#[serde(rename = "Cmprssn", skip_serializing_if = "Option::is_none")]
	pub cmprssn: Option<bool>,
	#[serde(rename = "PstTradRskRdctnFlg", skip_serializing_if = "Option::is_none")]
	pub pst_trad_rsk_rdctn_flg: Option<bool>,
	#[serde(rename = "PstTradRskRdctnEvt", skip_serializing_if = "Option::is_none")]
	pub pst_trad_rsk_rdctn_evt: Option<PTRREvent2>,
	#[serde(rename = "DerivEvt", skip_serializing_if = "Option::is_none")]
	pub deriv_evt: Option<DerivativeEvent6>,
	#[serde(rename = "TradConf", skip_serializing_if = "Option::is_none")]
	pub trad_conf: Option<TradeConfirmation4Choice>,
	#[serde(rename = "NonStdsdTerm", skip_serializing_if = "Option::is_none")]
	pub non_stdsd_term: Option<bool>,
	#[serde(rename = "TradClr", skip_serializing_if = "Option::is_none")]
	pub trad_clr: Option<TradeClearing11>,
	#[serde(rename = "BlckTradElctn", skip_serializing_if = "Option::is_none")]
	pub blck_trad_elctn: Option<bool>,
	#[serde(rename = "LrgNtnlOffFcltyElctn", skip_serializing_if = "Option::is_none")]
	pub lrg_ntnl_off_fclty_elctn: Option<bool>,
	#[serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none")]
	pub intrst_rate: Option<InterestRateLegs14>,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<CurrencyExchange22>,
	#[serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none")]
	pub cmmdty: Option<AssetClassCommodity7Choice>,
	#[serde(rename = "Optn", skip_serializing_if = "Option::is_none")]
	pub optn: Option<OptionOrSwaption11>,
	#[serde(rename = "NrgySpcfcAttrbts", skip_serializing_if = "Option::is_none")]
	pub nrgy_spcfc_attrbts: Option<EnergySpecificAttribute9>,
	#[serde(rename = "Cdt", skip_serializing_if = "Option::is_none")]
	pub cdt: Option<CreditDerivative4>,
	#[serde(rename = "OthrPmt", skip_serializing_if = "Option::is_none")]
	pub othr_pmt: Option<Vec<OtherPayment5>>,
	#[serde(rename = "Packg", skip_serializing_if = "Option::is_none")]
	pub packg: Option<Package4>,
	#[serde(rename = "TradAllcnSts", skip_serializing_if = "Option::is_none")]
	pub trad_allcn_sts: Option<AllocationIndicator1Code>,
}


// TradingCapacity7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TradingCapacity7Code {
	#[default]
	#[serde(rename = "AGEN")]
	CodeAGEN,
	#[serde(rename = "PRIN")]
	CodePRIN,

}


// Tranche3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Tranche3 {
	#[serde(rename = "AttchmntPt", skip_serializing_if = "Option::is_none")]
	pub attchmnt_pt: Option<f64>,
	#[serde(rename = "DtchmntPt", skip_serializing_if = "Option::is_none")]
	pub dtchmnt_pt: Option<f64>,
}


// TrancheIndicator3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrancheIndicator3Choice {
	#[serde(rename = "Trnchd", skip_serializing_if = "Option::is_none")]
	pub trnchd: Option<Tranche3>,
	#[serde(rename = "Utrnchd", skip_serializing_if = "Option::is_none")]
	pub utrnchd: Option<NoReasonCode>,
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}


// UTIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UTIIdentifier {
	#[serde(rename = "$value")]
	pub uti_identifier: String,
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


// UniqueProductIdentifier1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UniqueProductIdentifier1Choice {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max52Text>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification175>,
}


// UniqueProductIdentifier2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UniqueProductIdentifier2Choice {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max52Text>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification185>,
}


// UniqueTransactionIdentifier1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UniqueTransactionIdentifier1Choice {
	#[serde(rename = "UnqTxIdr", skip_serializing_if = "Option::is_none")]
	pub unq_tx_idr: Option<UTIIdentifier>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification179>,
}


// UniqueTransactionIdentifier2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UniqueTransactionIdentifier2Choice {
	#[serde(rename = "UnqTxIdr", skip_serializing_if = "Option::is_none")]
	pub unq_tx_idr: Option<UTIIdentifier>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification175>,
}


// UniqueTransactionIdentifier3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UniqueTransactionIdentifier3Choice {
	#[serde(rename = "UnqTxIdr", skip_serializing_if = "Option::is_none")]
	pub unq_tx_idr: Option<UTIIdentifier>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification175>,
	#[serde(rename = "NotAvlbl", skip_serializing_if = "Option::is_none")]
	pub not_avlbl: Option<NoReasonCode>,
}


// UnitOfMeasure8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitOfMeasure8Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalUnitOfMeasure1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification175>,
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


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "$value")]
	pub yes_no_indicator: bool,
}
