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
#[serde(transparent)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "$value")]
	pub active_currency_code: String,
}


// ActiveOrHistoricCurrencyAnd19DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
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


// ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_and20_decimal_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAnd20DecimalAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd20DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
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


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
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


// CollateralPortfolioCode5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralPortfolioCode5Choice {
	#[serde(rename = "Prtfl", skip_serializing_if = "Option::is_none")]
	pub prtfl: Option<PortfolioCode3Choice>,
	#[serde(rename = "MrgnPrtflCd", skip_serializing_if = "Option::is_none")]
	pub mrgn_prtfl_cd: Option<MarginPortfolio3>,
}


// CollateralisationType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CollateralisationType3Code {
	#[default]
	#[serde(rename = "FLCL")]
	CodeFLCL,
	#[serde(rename = "OWCL")]
	CodeOWCL,
	#[serde(rename = "OWC1")]
	CodeOWC1,
	#[serde(rename = "OWC2")]
	CodeOWC2,
	#[serde(rename = "OWP1")]
	CodeOWP1,
	#[serde(rename = "OWP2")]
	CodeOWP2,
	#[serde(rename = "PRCL")]
	CodePRCL,
	#[serde(rename = "PRC1")]
	CodePRC1,
	#[serde(rename = "PRC2")]
	CodePRC2,
	#[serde(rename = "UNCL")]
	CodeUNCL,
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
#[serde(transparent)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// CountrySubDivisionCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CountrySubDivisionCode {
	#[serde(rename = "$value")]
	pub country_sub_division_code: String,
}


// CreditDerivative7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditDerivative7 {
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
	#[serde(rename = "TrchInd", skip_serializing_if = "Option::is_none")]
	pub trch_ind: Option<bool>,
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


// DerivativesTradePositionSetReportV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativesTradePositionSetReportV02 {
	#[serde(rename = "AggtdPos")]
	pub aggtd_pos: PositionSetAggregated2Choice,
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


// ExternalAgreementType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalAgreementType1Code {
	#[serde(rename = "$value")]
	pub external_agreement_type1_code: String,
}


// ExternalBenchmarkCurveName1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalBenchmarkCurveName1Code {
	#[serde(rename = "$value")]
	pub external_benchmark_curve_name1_code: String,
}


// ExternalPartyRelationshipType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalPartyRelationshipType1Code {
	#[serde(rename = "$value")]
	pub external_party_relationship_type1_code: String,
}


// ExternalUnitOfMeasure1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
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
#[serde(transparent)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "$value")]
	pub isin_oct2015_identifier: String,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
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


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
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
#[serde(transparent)]
pub struct LongFraction19DecimalNumber {
	#[serde(rename = "$value")]
	pub long_fraction19_decimal_number: f64,
}


// MarginCollateralReport4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarginCollateralReport4 {
	#[serde(rename = "CollPrtflCd")]
	pub coll_prtfl_cd: CollateralPortfolioCode5Choice,
	#[serde(rename = "CollstnCtgy")]
	pub collstn_ctgy: CollateralisationType3Code,
	#[serde(rename = "TmStmp", skip_serializing_if = "Option::is_none")]
	pub tm_stmp: Option<String>,
}


// MarginPortfolio3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarginPortfolio3 {
	#[serde(rename = "InitlMrgnPrtflCd")]
	pub initl_mrgn_prtfl_cd: PortfolioCode5Choice,
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


// MaturityTerm2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MaturityTerm2 {
	#[serde(rename = "Unit")]
	pub unit: RateBasis1Code,
	#[serde(rename = "Val")]
	pub val: f64,
}


// Max1000Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max1000Text {
	#[serde(rename = "$value")]
	pub max1000_text: String,
}


// Max100Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max100Text {
	#[serde(rename = "$value")]
	pub max100_text: String,
}


// Max105Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max105Text {
	#[serde(rename = "$value")]
	pub max105_text: String,
}


// Max20PositiveNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max20PositiveNumber {
	#[serde(rename = "$value")]
	pub max20_positive_number: f64,
}


// Max210Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max210Text {
	#[serde(rename = "$value")]
	pub max210_text: String,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}


// Max3Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max3Number {
	#[serde(rename = "$value")]
	pub max3_number: f64,
}


// Max4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "$value")]
	pub max4_alpha_numeric_text: String,
}


// Max4Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max4Text {
	#[serde(rename = "$value")]
	pub max4_text: String,
}


// Max500Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max500Text {
	#[serde(rename = "$value")]
	pub max500_text: String,
}


// Max50Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max50Text {
	#[serde(rename = "$value")]
	pub max50_text: String,
}


// Max52Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max52Text {
	#[serde(rename = "$value")]
	pub max52_text: String,
}


// Max72Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
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


// NotionalAmount7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalAmount7 {
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
	#[serde(rename = "AmtInFct", skip_serializing_if = "Option::is_none")]
	pub amt_in_fct: Option<Vec<ActiveOrHistoricCurrencyAnd19DecimalAmount>>,
	#[serde(rename = "WghtdAvrgDlta", skip_serializing_if = "Option::is_none")]
	pub wghtd_avrg_dlta: Option<f64>,
}


// NotionalAmountLegs6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalAmountLegs6 {
	#[serde(rename = "FrstLeg", skip_serializing_if = "Option::is_none")]
	pub frst_leg: Option<NotionalAmount7>,
	#[serde(rename = "ScndLeg", skip_serializing_if = "Option::is_none")]
	pub scnd_leg: Option<NotionalAmount7>,
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Number {
	#[serde(rename = "$value")]
	pub number: f64,
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


// OtherPayment6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherPayment6 {
	#[serde(rename = "PmtCcy", skip_serializing_if = "Option::is_none")]
	pub pmt_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "PmtTp", skip_serializing_if = "Option::is_none")]
	pub pmt_tp: Option<PaymentType5Choice>,
	#[serde(rename = "PmtDt", skip_serializing_if = "Option::is_none")]
	pub pmt_dt: Option<String>,
	#[serde(rename = "PmtPyer", skip_serializing_if = "Option::is_none")]
	pub pmt_pyer: Option<PartyIdentification236Choice>,
	#[serde(rename = "PmtRcvr", skip_serializing_if = "Option::is_none")]
	pub pmt_rcvr: Option<PartyIdentification236Choice>,
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
#[serde(transparent)]
pub struct PercentageRate {
	#[serde(rename = "$value")]
	pub percentage_rate: f64,
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


// PositionSet21 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSet21 {
	#[serde(rename = "Dmnsns")]
	pub dmnsns: PositionSetDimensions16,
	#[serde(rename = "Mtrcs")]
	pub mtrcs: PositionSetMetrics14,
}


// PositionSet22 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSet22 {
	#[serde(rename = "Dmnsns")]
	pub dmnsns: PositionSetCollateralDimensions3,
	#[serde(rename = "Mtrcs")]
	pub mtrcs: PositionSetCollateralMetrics2,
}


// PositionSetAggregated2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetAggregated2Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[serde(rename = "Rpt", skip_serializing_if = "Option::is_none")]
	pub rpt: Option<PositionSetAggregated4>,
}


// PositionSetAggregated4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetAggregated4 {
	#[serde(rename = "RefDt")]
	pub ref_dt: String,
	#[serde(rename = "PosSet", skip_serializing_if = "Option::is_none")]
	pub pos_set: Option<Vec<PositionSet21>>,
	#[serde(rename = "CcyPosSet", skip_serializing_if = "Option::is_none")]
	pub ccy_pos_set: Option<Vec<PositionSet21>>,
	#[serde(rename = "CollPosSet", skip_serializing_if = "Option::is_none")]
	pub coll_pos_set: Option<Vec<PositionSet22>>,
	#[serde(rename = "CcyCollPosSet", skip_serializing_if = "Option::is_none")]
	pub ccy_coll_pos_set: Option<Vec<PositionSet22>>,
}


// PositionSetBuyerAndSeller2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetBuyerAndSeller2 {
	#[serde(rename = "Buyr", skip_serializing_if = "Option::is_none")]
	pub buyr: Option<PositionSetTotal2>,
	#[serde(rename = "Sellr", skip_serializing_if = "Option::is_none")]
	pub sellr: Option<PositionSetTotal2>,
}


// PositionSetCollateralDimensions3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetCollateralDimensions3 {
	#[serde(rename = "CtrPtyId", skip_serializing_if = "Option::is_none")]
	pub ctr_pty_id: Option<TradeCounterpartyReport20>,
	#[serde(rename = "Coll", skip_serializing_if = "Option::is_none")]
	pub coll: Option<MarginCollateralReport4>,
	#[serde(rename = "InitlMrgnPstdCcy", skip_serializing_if = "Option::is_none")]
	pub initl_mrgn_pstd_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "VartnMrgnPstdCcy", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_pstd_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "InitlMrgnRcvdCcy", skip_serializing_if = "Option::is_none")]
	pub initl_mrgn_rcvd_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "VartnMrgnRcvdCcy", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_rcvd_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "XcssCollPstdCcy", skip_serializing_if = "Option::is_none")]
	pub xcss_coll_pstd_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "XcssCollRcvdCcy", skip_serializing_if = "Option::is_none")]
	pub xcss_coll_rcvd_ccy: Option<ActiveOrHistoricCurrencyCode>,
}


// PositionSetCollateralMetrics2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetCollateralMetrics2 {
	#[serde(rename = "Ttl", skip_serializing_if = "Option::is_none")]
	pub ttl: Option<PositionSetCollateralTotal2>,
	#[serde(rename = "Clean", skip_serializing_if = "Option::is_none")]
	pub clean: Option<PositionSetCollateralTotal2>,
}


// PositionSetCollateralTotal2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetCollateralTotal2 {
	#[serde(rename = "NbOfRpts", skip_serializing_if = "Option::is_none")]
	pub nb_of_rpts: Option<f64>,
	#[serde(rename = "PstdMrgnOrColl", skip_serializing_if = "Option::is_none")]
	pub pstd_mrgn_or_coll: Option<PostedMarginOrCollateral6>,
	#[serde(rename = "RcvdMrgnOrColl", skip_serializing_if = "Option::is_none")]
	pub rcvd_mrgn_or_coll: Option<ReceivedMarginOrCollateral6>,
}


// PositionSetDimensions16 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetDimensions16 {
	#[serde(rename = "CtrPtyId", skip_serializing_if = "Option::is_none")]
	pub ctr_pty_id: Option<TradeCounterpartyReport20>,
	#[serde(rename = "ValCcy", skip_serializing_if = "Option::is_none")]
	pub val_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "Coll", skip_serializing_if = "Option::is_none")]
	pub coll: Option<MarginCollateralReport4>,
	#[serde(rename = "CtrctTp", skip_serializing_if = "Option::is_none")]
	pub ctrct_tp: Option<FinancialInstrumentContractType2Code>,
	#[serde(rename = "AsstClss", skip_serializing_if = "Option::is_none")]
	pub asst_clss: Option<ProductType4Code>,
	#[serde(rename = "UndrlygInstrm", skip_serializing_if = "Option::is_none")]
	pub undrlyg_instrm: Option<SecurityIdentification41Choice>,
	#[serde(rename = "NtnlCcy", skip_serializing_if = "Option::is_none")]
	pub ntnl_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "NtnlCcyScndLeg", skip_serializing_if = "Option::is_none")]
	pub ntnl_ccy_scnd_leg: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "SttlmCcy", skip_serializing_if = "Option::is_none")]
	pub sttlm_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "SttlmCcyScndLeg", skip_serializing_if = "Option::is_none")]
	pub sttlm_ccy_scnd_leg: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "MstrAgrmt", skip_serializing_if = "Option::is_none")]
	pub mstr_agrmt: Option<MasterAgreement8>,
	#[serde(rename = "Clrd", skip_serializing_if = "Option::is_none")]
	pub clrd: Option<bool>,
	#[serde(rename = "IntraGrp", skip_serializing_if = "Option::is_none")]
	pub intra_grp: Option<bool>,
	#[serde(rename = "XchgRateBsis", skip_serializing_if = "Option::is_none")]
	pub xchg_rate_bsis: Option<ExchangeRateBasis1Choice>,
	#[serde(rename = "OptnTp", skip_serializing_if = "Option::is_none")]
	pub optn_tp: Option<OptionType2Code>,
	#[serde(rename = "TmToMtrty", skip_serializing_if = "Option::is_none")]
	pub tm_to_mtrty: Option<TimeToMaturity1Choice>,
	#[serde(rename = "IRSTp", skip_serializing_if = "Option::is_none")]
	pub irs_tp: Option<Max52Text>,
	#[serde(rename = "Cdt", skip_serializing_if = "Option::is_none")]
	pub cdt: Option<CreditDerivative7>,
	#[serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none")]
	pub cmmdty: Option<AssetClassCommodity6Choice>,
	#[serde(rename = "OthrPmt", skip_serializing_if = "Option::is_none")]
	pub othr_pmt: Option<OtherPayment6>,
}


// PositionSetMetrics14 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetMetrics14 {
	#[serde(rename = "Ttl", skip_serializing_if = "Option::is_none")]
	pub ttl: Option<PositionSetBuyerAndSeller2>,
	#[serde(rename = "Clean", skip_serializing_if = "Option::is_none")]
	pub clean: Option<PositionSetBuyerAndSeller2>,
}


// PositionSetTotal2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetTotal2 {
	#[serde(rename = "NbOfTrds", skip_serializing_if = "Option::is_none")]
	pub nb_of_trds: Option<f64>,
	#[serde(rename = "PostvVal", skip_serializing_if = "Option::is_none")]
	pub postv_val: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
	#[serde(rename = "NegVal", skip_serializing_if = "Option::is_none")]
	pub neg_val: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
	#[serde(rename = "Ntnl", skip_serializing_if = "Option::is_none")]
	pub ntnl: Option<NotionalAmountLegs6>,
	#[serde(rename = "OthrPmtAmt", skip_serializing_if = "Option::is_none")]
	pub othr_pmt_amt: Option<Vec<ActiveOrHistoricCurrencyAnd19DecimalAmount>>,
}


// PostedMarginOrCollateral6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostedMarginOrCollateral6 {
	#[serde(rename = "InitlMrgnPstdPreHrcut", skip_serializing_if = "Option::is_none")]
	pub initl_mrgn_pstd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "InitlMrgnPstdPstHrcut", skip_serializing_if = "Option::is_none")]
	pub initl_mrgn_pstd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "VartnMrgnPstdPreHrcut", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_pstd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "VartnMrgnPstdPstHrcut", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_pstd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "XcssCollPstd", skip_serializing_if = "Option::is_none")]
	pub xcss_coll_pstd: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
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


// ReceivedMarginOrCollateral6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReceivedMarginOrCollateral6 {
	#[serde(rename = "InitlMrgnRcvdPreHrcut", skip_serializing_if = "Option::is_none")]
	pub initl_mrgn_rcvd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "InitlMrgnRcvdPstHrcut", skip_serializing_if = "Option::is_none")]
	pub initl_mrgn_rcvd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "VartnMrgnRcvdPreHrcut", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_rcvd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "VartnMrgnRcvdPstHrcut", skip_serializing_if = "Option::is_none")]
	pub vartn_mrgn_rcvd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "XcssCollRcvd", skip_serializing_if = "Option::is_none")]
	pub xcss_coll_rcvd: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
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


// SpecialPurpose2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SpecialPurpose2Code {
	#[default]
	#[serde(rename = "BLNK")]
	CodeBLNK,
	#[serde(rename = "NTAV")]
	CodeNTAV,
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


// TimeToMaturity1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeToMaturity1Choice {
	#[serde(rename = "Prd", skip_serializing_if = "Option::is_none")]
	pub prd: Option<TimeToMaturityPeriod1>,
	#[serde(rename = "Spcl", skip_serializing_if = "Option::is_none")]
	pub spcl: Option<SpecialPurpose2Code>,
}


// TimeToMaturityPeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeToMaturityPeriod1 {
	#[serde(rename = "Start", skip_serializing_if = "Option::is_none")]
	pub start: Option<MaturityTerm2>,
	#[serde(rename = "End", skip_serializing_if = "Option::is_none")]
	pub end: Option<MaturityTerm2>,
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


// TradingCapacity7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TradingCapacity7Code {
	#[default]
	#[serde(rename = "AGEN")]
	CodeAGEN,
	#[serde(rename = "PRIN")]
	CodePRIN,
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
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


// UnitOfMeasure8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitOfMeasure8Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalUnitOfMeasure1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification175>,
}
