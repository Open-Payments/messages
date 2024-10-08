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
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// ActiveOrHistoricCurrencyAnd19DecimalAmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd19DecimalAmountSimpleType {
	#[serde(rename = "ActiveOrHistoricCurrencyAnd19DecimalAmount_SimpleType")]
	pub active_or_historic_currency_and19_decimal_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAnd19DecimalAmount ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd19DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType {
	#[serde(rename = "ActiveOrHistoricCurrencyAnd20DecimalAmount_SimpleType")]
	pub active_or_historic_currency_and20_decimal_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAnd20DecimalAmount ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd20DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "ActiveOrHistoricCurrencyCode")]
	pub active_or_historic_currency_code: String,
}


// AgreementType2Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgreementType2Choice {
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// AgriculturalCommodityDairy2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityDairy2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// AgriculturalCommodityForestry2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityForestry2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// AgriculturalCommodityGrain3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityGrain3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// AgriculturalCommodityLiveStock2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityLiveStock2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// AgriculturalCommodityOilSeed2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityOilSeed2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// AgriculturalCommodityOliveOil3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityOliveOil3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// AgriculturalCommodityOther2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// AgriculturalCommodityPotato2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityPotato2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// AgriculturalCommoditySeafood2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommoditySeafood2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// AgriculturalCommoditySoft2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommoditySoft2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// AssetClassCommodity6Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodity6Choice {
	#[serde(rename = "Agrcltrl")]
	pub agrcltrl: Option<AssetClassCommodityAgricultural6Choice>,
	#[serde(rename = "Nrgy")]
	pub nrgy: Option<AssetClassCommodityEnergy3Choice>,
	#[serde(rename = "Envttl")]
	pub envttl: Option<AssetClassCommodityEnvironmental3Choice>,
	#[serde(rename = "Frtlzr")]
	pub frtlzr: Option<AssetClassCommodityFertilizer4Choice>,
	#[serde(rename = "Frght")]
	pub frght: Option<AssetClassCommodityFreight4Choice>,
	#[serde(rename = "Indx")]
	pub indx: Option<AssetClassCommodityIndex1>,
	#[serde(rename = "IndstrlPdct")]
	pub indstrl_pdct: Option<AssetClassCommodityIndustrialProduct2Choice>,
	#[serde(rename = "Infltn")]
	pub infltn: Option<AssetClassCommodityInflation1>,
	#[serde(rename = "Metl")]
	pub metl: Option<AssetClassCommodityMetal2Choice>,
	#[serde(rename = "MultiCmmdtyExtc")]
	pub multi_cmmdty_extc: Option<AssetClassCommodityMultiCommodityExotic1>,
	#[serde(rename = "OffclEcnmcSttstcs")]
	pub offcl_ecnmc_sttstcs: Option<AssetClassCommodityOfficialEconomicStatistics1>,
	#[serde(rename = "Othr")]
	pub othr: Option<AssetClassCommodityOther1>,
	#[serde(rename = "OthrC10")]
	pub othr_c10: Option<AssetClassCommodityC10Other1>,
	#[serde(rename = "Ppr")]
	pub ppr: Option<AssetClassCommodityPaper4Choice>,
	#[serde(rename = "Plprpln")]
	pub plprpln: Option<AssetClassCommodityPolypropylene4Choice>,
}


// AssetClassCommodityAgricultural6Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityAgricultural6Choice {
	#[serde(rename = "GrnOilSeed")]
	pub grn_oil_seed: Option<AgriculturalCommodityOilSeed2>,
	#[serde(rename = "Soft")]
	pub soft: Option<AgriculturalCommoditySoft2>,
	#[serde(rename = "Ptt")]
	pub ptt: Option<AgriculturalCommodityPotato2>,
	#[serde(rename = "OlvOil")]
	pub olv_oil: Option<AgriculturalCommodityOliveOil3>,
	#[serde(rename = "Dairy")]
	pub dairy: Option<AgriculturalCommodityDairy2>,
	#[serde(rename = "Frstry")]
	pub frstry: Option<AgriculturalCommodityForestry2>,
	#[serde(rename = "Sfd")]
	pub sfd: Option<AgriculturalCommoditySeafood2>,
	#[serde(rename = "LiveStock")]
	pub live_stock: Option<AgriculturalCommodityLiveStock2>,
	#[serde(rename = "Grn")]
	pub grn: Option<AgriculturalCommodityGrain3>,
	#[serde(rename = "Othr")]
	pub othr: Option<AgriculturalCommodityOther2>,
}


// AssetClassCommodityC10Other1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityC10Other1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
}


// AssetClassCommodityEnergy3Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityEnergy3Choice {
	#[serde(rename = "Elctrcty")]
	pub elctrcty: Option<EnergyCommodityElectricity2>,
	#[serde(rename = "NtrlGas")]
	pub ntrl_gas: Option<EnergyCommodityNaturalGas3>,
	#[serde(rename = "Oil")]
	pub oil: Option<EnergyCommodityOil3>,
	#[serde(rename = "Coal")]
	pub coal: Option<EnergyCommodityCoal2>,
	#[serde(rename = "IntrNrgy")]
	pub intr_nrgy: Option<EnergyCommodityInterEnergy2>,
	#[serde(rename = "RnwblNrgy")]
	pub rnwbl_nrgy: Option<EnergyCommodityRenewableEnergy2>,
	#[serde(rename = "LghtEnd")]
	pub lght_end: Option<EnergyCommodityLightEnd2>,
	#[serde(rename = "Dstllts")]
	pub dstllts: Option<EnergyCommodityDistillates2>,
	#[serde(rename = "Othr")]
	pub othr: Option<EnergyCommodityOther2>,
}


// AssetClassCommodityEnvironmental3Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityEnvironmental3Choice {
	#[serde(rename = "Emssns")]
	pub emssns: Option<EnvironmentalCommodityEmission3>,
	#[serde(rename = "Wthr")]
	pub wthr: Option<EnvironmentalCommodityWeather2>,
	#[serde(rename = "CrbnRltd")]
	pub crbn_rltd: Option<EnvironmentalCommodityCarbonRelated2>,
	#[serde(rename = "Othr")]
	pub othr: Option<EnvironmentCommodityOther2>,
}


// AssetClassCommodityFertilizer4Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityFertilizer4Choice {
	#[serde(rename = "Ammn")]
	pub ammn: Option<FertilizerCommodityAmmonia2>,
	#[serde(rename = "DmmnmPhspht")]
	pub dmmnm_phspht: Option<FertilizerCommodityDiammoniumPhosphate2>,
	#[serde(rename = "Ptsh")]
	pub ptsh: Option<FertilizerCommodityPotash2>,
	#[serde(rename = "Slphr")]
	pub slphr: Option<FertilizerCommoditySulphur2>,
	#[serde(rename = "Urea")]
	pub urea: Option<FertilizerCommodityUrea2>,
	#[serde(rename = "UreaAndAmmnmNtrt")]
	pub urea_and_ammnm_ntrt: Option<FertilizerCommodityUreaAndAmmoniumNitrate2>,
	#[serde(rename = "Othr")]
	pub othr: Option<FertilizerCommodityOther2>,
}


// AssetClassCommodityFreight4Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityFreight4Choice {
	#[serde(rename = "Dry")]
	pub dry: Option<FreightCommodityDry3>,
	#[serde(rename = "Wet")]
	pub wet: Option<FreightCommodityWet3>,
	#[serde(rename = "CntnrShip")]
	pub cntnr_ship: Option<FreightCommodityContainerShip2>,
	#[serde(rename = "Othr")]
	pub othr: Option<FreightCommodityOther2>,
}


// AssetClassCommodityIndex1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityIndex1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
}


// AssetClassCommodityIndustrialProduct2Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityIndustrialProduct2Choice {
	#[serde(rename = "Cnstrctn")]
	pub cnstrctn: Option<IndustrialProductCommodityConstruction2>,
	#[serde(rename = "Manfctg")]
	pub manfctg: Option<IndustrialProductCommodityManufacturing2>,
}


// AssetClassCommodityInflation1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityInflation1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
}


// AssetClassCommodityMetal2Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityMetal2Choice {
	#[serde(rename = "NonPrcs")]
	pub non_prcs: Option<MetalCommodityNonPrecious2>,
	#[serde(rename = "Prcs")]
	pub prcs: Option<MetalCommodityPrecious2>,
}


// AssetClassCommodityMultiCommodityExotic1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityMultiCommodityExotic1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
}


// AssetClassCommodityOfficialEconomicStatistics1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityOfficialEconomicStatistics1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
}


// AssetClassCommodityOther1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
}


// AssetClassCommodityPaper4Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityPaper4Choice {
	#[serde(rename = "CntnrBrd")]
	pub cntnr_brd: Option<PaperCommodityContainerBoard2>,
	#[serde(rename = "Nwsprnt")]
	pub nwsprnt: Option<PaperCommodityNewsprint2>,
	#[serde(rename = "Pulp")]
	pub pulp: Option<PaperCommodityPulp2>,
	#[serde(rename = "RcvrdPpr")]
	pub rcvrd_ppr: Option<PaperCommodityOther1>,
	#[serde(rename = "Othr")]
	pub othr: Option<PaperCommodityOther1>,
}


// AssetClassCommodityPolypropylene4Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityPolypropylene4Choice {
	#[serde(rename = "Plstc")]
	pub plstc: Option<PolypropyleneCommodityPlastic2>,
	#[serde(rename = "Othr")]
	pub othr: Option<PolypropyleneCommodityOther2>,
}


// AssetClassDetailedSubProductType10Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType10Code {
	#[serde(rename = "AssetClassDetailedSubProductType10Code")]
	pub asset_class_detailed_sub_product_type10_code: String,
}


// AssetClassDetailedSubProductType11Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType11Code {
	#[serde(rename = "AssetClassDetailedSubProductType11Code")]
	pub asset_class_detailed_sub_product_type11_code: String,
}


// AssetClassDetailedSubProductType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType1Code {
	#[serde(rename = "AssetClassDetailedSubProductType1Code")]
	pub asset_class_detailed_sub_product_type1_code: String,
}


// AssetClassDetailedSubProductType29Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType29Code {
	#[serde(rename = "AssetClassDetailedSubProductType29Code")]
	pub asset_class_detailed_sub_product_type29_code: String,
}


// AssetClassDetailedSubProductType2Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType2Code {
	#[serde(rename = "AssetClassDetailedSubProductType2Code")]
	pub asset_class_detailed_sub_product_type2_code: String,
}


// AssetClassDetailedSubProductType30Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType30Code {
	#[serde(rename = "AssetClassDetailedSubProductType30Code")]
	pub asset_class_detailed_sub_product_type30_code: String,
}


// AssetClassDetailedSubProductType31Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType31Code {
	#[serde(rename = "AssetClassDetailedSubProductType31Code")]
	pub asset_class_detailed_sub_product_type31_code: String,
}


// AssetClassDetailedSubProductType32Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType32Code {
	#[serde(rename = "AssetClassDetailedSubProductType32Code")]
	pub asset_class_detailed_sub_product_type32_code: String,
}


// AssetClassDetailedSubProductType33Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType33Code {
	#[serde(rename = "AssetClassDetailedSubProductType33Code")]
	pub asset_class_detailed_sub_product_type33_code: String,
}


// AssetClassDetailedSubProductType34Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType34Code {
	#[serde(rename = "AssetClassDetailedSubProductType34Code")]
	pub asset_class_detailed_sub_product_type34_code: String,
}


// AssetClassDetailedSubProductType5Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType5Code {
	#[serde(rename = "AssetClassDetailedSubProductType5Code")]
	pub asset_class_detailed_sub_product_type5_code: String,
}


// AssetClassDetailedSubProductType8Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType8Code {
	#[serde(rename = "AssetClassDetailedSubProductType8Code")]
	pub asset_class_detailed_sub_product_type8_code: String,
}


// AssetClassProductType11Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType11Code {
	#[serde(rename = "AssetClassProductType11Code")]
	pub asset_class_product_type11_code: String,
}


// AssetClassProductType12Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType12Code {
	#[serde(rename = "AssetClassProductType12Code")]
	pub asset_class_product_type12_code: String,
}


// AssetClassProductType13Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType13Code {
	#[serde(rename = "AssetClassProductType13Code")]
	pub asset_class_product_type13_code: String,
}


// AssetClassProductType14Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType14Code {
	#[serde(rename = "AssetClassProductType14Code")]
	pub asset_class_product_type14_code: String,
}


// AssetClassProductType15Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType15Code {
	#[serde(rename = "AssetClassProductType15Code")]
	pub asset_class_product_type15_code: String,
}


// AssetClassProductType16Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType16Code {
	#[serde(rename = "AssetClassProductType16Code")]
	pub asset_class_product_type16_code: String,
}


// AssetClassProductType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType1Code {
	#[serde(rename = "AssetClassProductType1Code")]
	pub asset_class_product_type1_code: String,
}


// AssetClassProductType2Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType2Code {
	#[serde(rename = "AssetClassProductType2Code")]
	pub asset_class_product_type2_code: String,
}


// AssetClassProductType3Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType3Code {
	#[serde(rename = "AssetClassProductType3Code")]
	pub asset_class_product_type3_code: String,
}


// AssetClassProductType4Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType4Code {
	#[serde(rename = "AssetClassProductType4Code")]
	pub asset_class_product_type4_code: String,
}


// AssetClassProductType5Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType5Code {
	#[serde(rename = "AssetClassProductType5Code")]
	pub asset_class_product_type5_code: String,
}


// AssetClassProductType6Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType6Code {
	#[serde(rename = "AssetClassProductType6Code")]
	pub asset_class_product_type6_code: String,
}


// AssetClassProductType7Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType7Code {
	#[serde(rename = "AssetClassProductType7Code")]
	pub asset_class_product_type7_code: String,
}


// AssetClassProductType8Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType8Code {
	#[serde(rename = "AssetClassProductType8Code")]
	pub asset_class_product_type8_code: String,
}


// AssetClassProductType9Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType9Code {
	#[serde(rename = "AssetClassProductType9Code")]
	pub asset_class_product_type9_code: String,
}


// AssetClassSubProductType10Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType10Code {
	#[serde(rename = "AssetClassSubProductType10Code")]
	pub asset_class_sub_product_type10_code: String,
}


// AssetClassSubProductType15Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType15Code {
	#[serde(rename = "AssetClassSubProductType15Code")]
	pub asset_class_sub_product_type15_code: String,
}


// AssetClassSubProductType16Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType16Code {
	#[serde(rename = "AssetClassSubProductType16Code")]
	pub asset_class_sub_product_type16_code: String,
}


// AssetClassSubProductType18Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType18Code {
	#[serde(rename = "AssetClassSubProductType18Code")]
	pub asset_class_sub_product_type18_code: String,
}


// AssetClassSubProductType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType1Code {
	#[serde(rename = "AssetClassSubProductType1Code")]
	pub asset_class_sub_product_type1_code: String,
}


// AssetClassSubProductType20Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType20Code {
	#[serde(rename = "AssetClassSubProductType20Code")]
	pub asset_class_sub_product_type20_code: String,
}


// AssetClassSubProductType21Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType21Code {
	#[serde(rename = "AssetClassSubProductType21Code")]
	pub asset_class_sub_product_type21_code: String,
}


// AssetClassSubProductType22Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType22Code {
	#[serde(rename = "AssetClassSubProductType22Code")]
	pub asset_class_sub_product_type22_code: String,
}


// AssetClassSubProductType23Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType23Code {
	#[serde(rename = "AssetClassSubProductType23Code")]
	pub asset_class_sub_product_type23_code: String,
}


// AssetClassSubProductType24Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType24Code {
	#[serde(rename = "AssetClassSubProductType24Code")]
	pub asset_class_sub_product_type24_code: String,
}


// AssetClassSubProductType25Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType25Code {
	#[serde(rename = "AssetClassSubProductType25Code")]
	pub asset_class_sub_product_type25_code: String,
}


// AssetClassSubProductType26Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType26Code {
	#[serde(rename = "AssetClassSubProductType26Code")]
	pub asset_class_sub_product_type26_code: String,
}


// AssetClassSubProductType27Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType27Code {
	#[serde(rename = "AssetClassSubProductType27Code")]
	pub asset_class_sub_product_type27_code: String,
}


// AssetClassSubProductType28Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType28Code {
	#[serde(rename = "AssetClassSubProductType28Code")]
	pub asset_class_sub_product_type28_code: String,
}


// AssetClassSubProductType29Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType29Code {
	#[serde(rename = "AssetClassSubProductType29Code")]
	pub asset_class_sub_product_type29_code: String,
}


// AssetClassSubProductType2Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType2Code {
	#[serde(rename = "AssetClassSubProductType2Code")]
	pub asset_class_sub_product_type2_code: String,
}


// AssetClassSubProductType30Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType30Code {
	#[serde(rename = "AssetClassSubProductType30Code")]
	pub asset_class_sub_product_type30_code: String,
}


// AssetClassSubProductType31Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType31Code {
	#[serde(rename = "AssetClassSubProductType31Code")]
	pub asset_class_sub_product_type31_code: String,
}


// AssetClassSubProductType32Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType32Code {
	#[serde(rename = "AssetClassSubProductType32Code")]
	pub asset_class_sub_product_type32_code: String,
}


// AssetClassSubProductType33Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType33Code {
	#[serde(rename = "AssetClassSubProductType33Code")]
	pub asset_class_sub_product_type33_code: String,
}


// AssetClassSubProductType34Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType34Code {
	#[serde(rename = "AssetClassSubProductType34Code")]
	pub asset_class_sub_product_type34_code: String,
}


// AssetClassSubProductType35Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType35Code {
	#[serde(rename = "AssetClassSubProductType35Code")]
	pub asset_class_sub_product_type35_code: String,
}


// AssetClassSubProductType36Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType36Code {
	#[serde(rename = "AssetClassSubProductType36Code")]
	pub asset_class_sub_product_type36_code: String,
}


// AssetClassSubProductType37Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType37Code {
	#[serde(rename = "AssetClassSubProductType37Code")]
	pub asset_class_sub_product_type37_code: String,
}


// AssetClassSubProductType39Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType39Code {
	#[serde(rename = "AssetClassSubProductType39Code")]
	pub asset_class_sub_product_type39_code: String,
}


// AssetClassSubProductType3Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType3Code {
	#[serde(rename = "AssetClassSubProductType3Code")]
	pub asset_class_sub_product_type3_code: String,
}


// AssetClassSubProductType40Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType40Code {
	#[serde(rename = "AssetClassSubProductType40Code")]
	pub asset_class_sub_product_type40_code: String,
}


// AssetClassSubProductType41Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType41Code {
	#[serde(rename = "AssetClassSubProductType41Code")]
	pub asset_class_sub_product_type41_code: String,
}


// AssetClassSubProductType42Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType42Code {
	#[serde(rename = "AssetClassSubProductType42Code")]
	pub asset_class_sub_product_type42_code: String,
}


// AssetClassSubProductType43Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType43Code {
	#[serde(rename = "AssetClassSubProductType43Code")]
	pub asset_class_sub_product_type43_code: String,
}


// AssetClassSubProductType44Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType44Code {
	#[serde(rename = "AssetClassSubProductType44Code")]
	pub asset_class_sub_product_type44_code: String,
}


// AssetClassSubProductType45Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType45Code {
	#[serde(rename = "AssetClassSubProductType45Code")]
	pub asset_class_sub_product_type45_code: String,
}


// AssetClassSubProductType46Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType46Code {
	#[serde(rename = "AssetClassSubProductType46Code")]
	pub asset_class_sub_product_type46_code: String,
}


// AssetClassSubProductType49Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType49Code {
	#[serde(rename = "AssetClassSubProductType49Code")]
	pub asset_class_sub_product_type49_code: String,
}


// AssetClassSubProductType5Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType5Code {
	#[serde(rename = "AssetClassSubProductType5Code")]
	pub asset_class_sub_product_type5_code: String,
}


// AssetClassSubProductType6Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType6Code {
	#[serde(rename = "AssetClassSubProductType6Code")]
	pub asset_class_sub_product_type6_code: String,
}


// AssetClassSubProductType7Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType7Code {
	#[serde(rename = "AssetClassSubProductType7Code")]
	pub asset_class_sub_product_type7_code: String,
}


// AssetClassSubProductType8Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType8Code {
	#[serde(rename = "AssetClassSubProductType8Code")]
	pub asset_class_sub_product_type8_code: String,
}


// BasketConstituents3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct BasketConstituents3 {
	#[serde(rename = "InstrmId")]
	pub instrm_id: InstrumentIdentification6Choice,
	#[serde(rename = "Qty")]
	pub qty: Option<f64>,
	#[serde(rename = "UnitOfMeasr")]
	pub unit_of_measr: Option<UnitOfMeasure8Choice>,
}


// CollateralPortfolioCode5Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralPortfolioCode5Choice {
	#[serde(rename = "Prtfl")]
	pub prtfl: Option<PortfolioCode3Choice>,
	#[serde(rename = "MrgnPrtflCd")]
	pub mrgn_prtfl_cd: Option<MarginPortfolio3>,
}


// CollateralisationType3Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralisationType3Code {
	#[serde(rename = "CollateralisationType3Code")]
	pub collateralisation_type3_code: String,
}


// Counterparty45 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Counterparty45 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification248Choice,
	#[serde(rename = "Ntr")]
	pub ntr: Option<CounterpartyTradeNature15Choice>,
	#[serde(rename = "TradgCpcty")]
	pub tradg_cpcty: Option<String>,
	#[serde(rename = "DrctnOrSd")]
	pub drctn_or_sd: Option<Direction4Choice>,
	#[serde(rename = "TradrLctn")]
	pub tradr_lctn: Option<String>,
	#[serde(rename = "BookgLctn")]
	pub bookg_lctn: Option<String>,
	#[serde(rename = "RptgXmptn")]
	pub rptg_xmptn: Option<ReportingExemption1>,
}


// Counterparty46 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Counterparty46 {
	#[serde(rename = "IdTp")]
	pub id_tp: Option<PartyIdentification248Choice>,
	#[serde(rename = "Ntr")]
	pub ntr: Option<CounterpartyTradeNature15Choice>,
	#[serde(rename = "RptgOblgtn")]
	pub rptg_oblgtn: Option<bool>,
}


// CounterpartyTradeNature15Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyTradeNature15Choice {
	#[serde(rename = "FI")]
	pub fi: Option<FinancialInstitutionSector1>,
	#[serde(rename = "NFI")]
	pub nfi: Option<NonFinancialInstitutionSector10>,
	#[serde(rename = "CntrlCntrPty")]
	pub cntrl_cntr_pty: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<String>,
}


// CountryCode ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// CountrySubDivisionCode ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountrySubDivisionCode {
	#[serde(rename = "CountrySubDivisionCode")]
	pub country_sub_division_code: String,
}


// CreditDerivative7 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditDerivative7 {
	#[serde(rename = "Snrty")]
	pub snrty: Option<String>,
	#[serde(rename = "RefPty")]
	pub ref_pty: Option<DerivativePartyIdentification1Choice>,
	#[serde(rename = "PmtFrqcy")]
	pub pmt_frqcy: Option<String>,
	#[serde(rename = "ClctnBsis")]
	pub clctn_bsis: Option<String>,
	#[serde(rename = "Srs")]
	pub srs: Option<f64>,
	#[serde(rename = "Vrsn")]
	pub vrsn: Option<f64>,
	#[serde(rename = "IndxFctr")]
	pub indx_fctr: Option<f64>,
	#[serde(rename = "TrchInd")]
	pub trch_ind: Option<bool>,
}


// CustomBasket4 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CustomBasket4 {
	#[serde(rename = "Strr")]
	pub strr: Option<String>,
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "Cnsttnts")]
	pub cnsttnts: Option<Vec<BasketConstituents3>>,
}


// DebtInstrumentSeniorityType2Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DebtInstrumentSeniorityType2Code {
	#[serde(rename = "DebtInstrumentSeniorityType2Code")]
	pub debt_instrument_seniority_type2_code: String,
}


// DerivativePartyIdentification1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativePartyIdentification1Choice {
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
	#[serde(rename = "CtrySubDvsn")]
	pub ctry_sub_dvsn: Option<String>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
}


// DerivativesTradePositionSetReportV02 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativesTradePositionSetReportV02 {
	#[serde(rename = "AggtdPos")]
	pub aggtd_pos: PositionSetAggregated2Choice,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// Direction2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Direction2 {
	#[serde(rename = "DrctnOfTheFrstLeg")]
	pub drctn_of_the_frst_leg: String,
	#[serde(rename = "DrctnOfTheScndLeg")]
	pub drctn_of_the_scnd_leg: Option<String>,
}


// Direction4Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Direction4Choice {
	#[serde(rename = "Drctn")]
	pub drctn: Option<Direction2>,
	#[serde(rename = "CtrPtySd")]
	pub ctr_pty_sd: Option<String>,
}


// EnergyCommodityCoal2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityCoal2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// EnergyCommodityDistillates2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityDistillates2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// EnergyCommodityElectricity2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityElectricity2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// EnergyCommodityInterEnergy2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityInterEnergy2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// EnergyCommodityLightEnd2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityLightEnd2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// EnergyCommodityNaturalGas3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityNaturalGas3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// EnergyCommodityOil3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityOil3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// EnergyCommodityOther2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// EnergyCommodityRenewableEnergy2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityRenewableEnergy2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// EnvironmentCommodityOther2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// EnvironmentalCommodityCarbonRelated2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCommodityCarbonRelated2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// EnvironmentalCommodityEmission3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCommodityEmission3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// EnvironmentalCommodityWeather2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCommodityWeather2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// ExchangeRateBasis1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExchangeRateBasis1 {
	#[serde(rename = "BaseCcy")]
	pub base_ccy: String,
	#[serde(rename = "QtdCcy")]
	pub qtd_ccy: String,
}


// ExchangeRateBasis1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExchangeRateBasis1Choice {
	#[serde(rename = "CcyPair")]
	pub ccy_pair: Option<ExchangeRateBasis1>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ExternalAgreementType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAgreementType1Code {
	#[serde(rename = "ExternalAgreementType1Code")]
	pub external_agreement_type1_code: String,
}


// ExternalBenchmarkCurveName1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalBenchmarkCurveName1Code {
	#[serde(rename = "ExternalBenchmarkCurveName1Code")]
	pub external_benchmark_curve_name1_code: String,
}


// ExternalPartyRelationshipType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPartyRelationshipType1Code {
	#[serde(rename = "ExternalPartyRelationshipType1Code")]
	pub external_party_relationship_type1_code: String,
}


// ExternalUnitOfMeasure1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalUnitOfMeasure1Code {
	#[serde(rename = "ExternalUnitOfMeasure1Code")]
	pub external_unit_of_measure1_code: String,
}


// FertilizerCommodityAmmonia2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityAmmonia2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FertilizerCommodityDiammoniumPhosphate2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityDiammoniumPhosphate2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FertilizerCommodityOther2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FertilizerCommodityPotash2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityPotash2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FertilizerCommoditySulphur2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommoditySulphur2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FertilizerCommodityUrea2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityUrea2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FertilizerCommodityUreaAndAmmoniumNitrate2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityUreaAndAmmoniumNitrate2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FinancialInstitutionSector1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstitutionSector1 {
	#[serde(rename = "Sctr")]
	pub sctr: Vec<FinancialPartyClassification2Choice>,
	#[serde(rename = "ClrThrshld")]
	pub clr_thrshld: Option<bool>,
}


// FinancialInstrumentContractType2Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentContractType2Code {
	#[serde(rename = "FinancialInstrumentContractType2Code")]
	pub financial_instrument_contract_type2_code: String,
}


// FinancialPartyClassification2Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialPartyClassification2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification175>,
}


// FinancialPartySectorType3Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialPartySectorType3Code {
	#[serde(rename = "FinancialPartySectorType3Code")]
	pub financial_party_sector_type3_code: String,
}


// FreightCommodityContainerShip2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityContainerShip2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FreightCommodityDry3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityDry3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// FreightCommodityOther2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FreightCommodityWet3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityWet3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// Frequency13Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Frequency13Code {
	#[serde(rename = "Frequency13Code")]
	pub frequency13_code: String,
}


// GenericIdentification175 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification175 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericIdentification184 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification184 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Src")]
	pub src: String,
}


// GenericIdentification185 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification185 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// ISINOct2015Identifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "ISINOct2015Identifier")]
	pub isin_oct2015_identifier: String,
}


// ISODate ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// IndexIdentification1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndexIdentification1 {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Indx")]
	pub indx: Option<String>,
}


// IndustrialProductCommodityConstruction2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndustrialProductCommodityConstruction2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// IndustrialProductCommodityManufacturing2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndustrialProductCommodityManufacturing2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// InstrumentIdentification6Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct InstrumentIdentification6Choice {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[serde(rename = "AltrntvInstrmId")]
	pub altrntv_instrm_id: Option<String>,
	#[serde(rename = "UnqPdctIdr")]
	pub unq_pdct_idr: Option<UniqueProductIdentifier1Choice>,
	#[serde(rename = "OthrId")]
	pub othr_id: Option<GenericIdentification184>,
}


// LEIIdentifier ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LegalPersonIdentification1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LegalPersonIdentification1 {
	#[serde(rename = "Id")]
	pub id: OrganisationIdentification15Choice,
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
}


// LongFraction19DecimalNumber ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct LongFraction19DecimalNumber {
	#[serde(rename = "LongFraction19DecimalNumber")]
	pub long_fraction19_decimal_number: f64,
}


// MarginCollateralReport4 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarginCollateralReport4 {
	#[serde(rename = "CollPrtflCd")]
	pub coll_prtfl_cd: CollateralPortfolioCode5Choice,
	#[serde(rename = "CollstnCtgy")]
	pub collstn_ctgy: String,
	#[serde(rename = "TmStmp")]
	pub tm_stmp: Option<String>,
}


// MarginPortfolio3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarginPortfolio3 {
	#[serde(rename = "InitlMrgnPrtflCd")]
	pub initl_mrgn_prtfl_cd: PortfolioCode5Choice,
	#[serde(rename = "VartnMrgnPrtflCd")]
	pub vartn_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
}


// MasterAgreement8 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MasterAgreement8 {
	#[serde(rename = "Tp")]
	pub tp: Option<AgreementType2Choice>,
	#[serde(rename = "Vrsn")]
	pub vrsn: Option<String>,
	#[serde(rename = "OthrMstrAgrmtDtls")]
	pub othr_mstr_agrmt_dtls: Option<String>,
}


// MaturityTerm2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MaturityTerm2 {
	#[serde(rename = "Unit")]
	pub unit: String,
	#[serde(rename = "Val")]
	pub val: f64,
}


// Max1000Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max1000Text {
	#[serde(rename = "Max1000Text")]
	pub max1000_text: String,
}


// Max100Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max100Text {
	#[serde(rename = "Max100Text")]
	pub max100_text: String,
}


// Max105Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max105Text {
	#[serde(rename = "Max105Text")]
	pub max105_text: String,
}


// Max20PositiveNumber ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max20PositiveNumber {
	#[serde(rename = "Max20PositiveNumber")]
	pub max20_positive_number: f64,
}


// Max210Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max210Text {
	#[serde(rename = "Max210Text")]
	pub max210_text: String,
}


// Max350Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Max3Number ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max3Number {
	#[serde(rename = "Max3Number")]
	pub max3_number: f64,
}


// Max4AlphaNumericText ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "Max4AlphaNumericText")]
	pub max4_alpha_numeric_text: String,
}


// Max4Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4Text {
	#[serde(rename = "Max4Text")]
	pub max4_text: String,
}


// Max500Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max500Text {
	#[serde(rename = "Max500Text")]
	pub max500_text: String,
}


// Max50Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max50Text {
	#[serde(rename = "Max50Text")]
	pub max50_text: String,
}


// Max52Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max52Text {
	#[serde(rename = "Max52Text")]
	pub max52_text: String,
}


// Max72Text ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max72Text {
	#[serde(rename = "Max72Text")]
	pub max72_text: String,
}


// MetalCommodityNonPrecious2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MetalCommodityNonPrecious2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// MetalCommodityPrecious2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MetalCommodityPrecious2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// NaturalPersonIdentification2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NaturalPersonIdentification2 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Dmcl")]
	pub dmcl: Option<String>,
}


// NaturalPersonIdentification3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NaturalPersonIdentification3 {
	#[serde(rename = "Id")]
	pub id: NaturalPersonIdentification2,
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
}


// NoReasonCode ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NoReasonCode {
	#[serde(rename = "NoReasonCode")]
	pub no_reason_code: String,
}


// NonFinancialInstitutionSector10 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NonFinancialInstitutionSector10 {
	#[serde(rename = "Sctr")]
	pub sctr: Vec<GenericIdentification175>,
	#[serde(rename = "ClrThrshld")]
	pub clr_thrshld: Option<bool>,
	#[serde(rename = "DrctlyLkdActvty")]
	pub drctly_lkd_actvty: Option<bool>,
	#[serde(rename = "FdrlInstn")]
	pub fdrl_instn: Option<bool>,
}


// NotApplicable1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotApplicable1Code {
	#[serde(rename = "NotApplicable1Code")]
	pub not_applicable1_code: String,
}


// NotionalAmount7 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalAmount7 {
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
	#[serde(rename = "AmtInFct")]
	pub amt_in_fct: Option<Vec<ActiveOrHistoricCurrencyAnd19DecimalAmount>>,
	#[serde(rename = "WghtdAvrgDlta")]
	pub wghtd_avrg_dlta: Option<f64>,
}


// NotionalAmountLegs6 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotionalAmountLegs6 {
	#[serde(rename = "FrstLeg")]
	pub frst_leg: Option<NotionalAmount7>,
	#[serde(rename = "ScndLeg")]
	pub scnd_leg: Option<NotionalAmount7>,
}


// Number ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
}


// OptionParty1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OptionParty1Code {
	#[serde(rename = "OptionParty1Code")]
	pub option_party1_code: String,
}


// OptionParty3Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OptionParty3Code {
	#[serde(rename = "OptionParty3Code")]
	pub option_party3_code: String,
}


// OptionType2Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OptionType2Code {
	#[serde(rename = "OptionType2Code")]
	pub option_type2_code: String,
}


// OrganisationIdentification15Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification15Choice {
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<OrganisationIdentification38>,
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
}


// OrganisationIdentification38 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification38 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Dmcl")]
	pub dmcl: Option<String>,
}


// OtherPayment6 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherPayment6 {
	#[serde(rename = "PmtCcy")]
	pub pmt_ccy: Option<String>,
	#[serde(rename = "PmtTp")]
	pub pmt_tp: Option<PaymentType5Choice>,
	#[serde(rename = "PmtDt")]
	pub pmt_dt: Option<String>,
	#[serde(rename = "PmtPyer")]
	pub pmt_pyer: Option<PartyIdentification236Choice>,
	#[serde(rename = "PmtRcvr")]
	pub pmt_rcvr: Option<PartyIdentification236Choice>,
}


// PaperCommodityContainerBoard2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityContainerBoard2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PaperCommodityNewsprint2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityNewsprint2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PaperCommodityOther1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PaperCommodityPulp2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityPulp2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PartyIdentification236Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification236Choice {
	#[serde(rename = "Lgl")]
	pub lgl: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "Ntrl")]
	pub ntrl: Option<NaturalPersonIdentification2>,
}


// PartyIdentification248Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification248Choice {
	#[serde(rename = "Lgl")]
	pub lgl: Option<LegalPersonIdentification1>,
	#[serde(rename = "Ntrl")]
	pub ntrl: Option<NaturalPersonIdentification3>,
}


// PaymentType4Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentType4Code {
	#[serde(rename = "PaymentType4Code")]
	pub payment_type4_code: String,
}


// PaymentType5Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentType5Choice {
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "PrtryTp")]
	pub prtry_tp: Option<String>,
}


// PercentageRate ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// PolypropyleneCommodityOther2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PolypropyleneCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PolypropyleneCommodityPlastic2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PolypropyleneCommodityPlastic2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PortfolioCode3Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioCode3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "NoPrtfl")]
	pub no_prtfl: Option<String>,
}


// PortfolioCode5Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioCode5Choice {
	#[serde(rename = "Prtfl")]
	pub prtfl: Option<PortfolioIdentification3>,
	#[serde(rename = "NoPrtfl")]
	pub no_prtfl: Option<String>,
}


// PortfolioIdentification3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioIdentification3 {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "PrtflTxXmptn")]
	pub prtfl_tx_xmptn: Option<bool>,
}


// PositionSet21 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSet21 {
	#[serde(rename = "Dmnsns")]
	pub dmnsns: PositionSetDimensions16,
	#[serde(rename = "Mtrcs")]
	pub mtrcs: PositionSetMetrics14,
}


// PositionSet22 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSet22 {
	#[serde(rename = "Dmnsns")]
	pub dmnsns: PositionSetCollateralDimensions3,
	#[serde(rename = "Mtrcs")]
	pub mtrcs: PositionSetCollateralMetrics2,
}


// PositionSetAggregated2Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetAggregated2Choice {
	#[serde(rename = "DataSetActn")]
	pub data_set_actn: Option<String>,
	#[serde(rename = "Rpt")]
	pub rpt: Option<PositionSetAggregated4>,
}


// PositionSetAggregated4 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetAggregated4 {
	#[serde(rename = "RefDt")]
	pub ref_dt: String,
	#[serde(rename = "PosSet")]
	pub pos_set: Option<Vec<PositionSet21>>,
	#[serde(rename = "CcyPosSet")]
	pub ccy_pos_set: Option<Vec<PositionSet21>>,
	#[serde(rename = "CollPosSet")]
	pub coll_pos_set: Option<Vec<PositionSet22>>,
	#[serde(rename = "CcyCollPosSet")]
	pub ccy_coll_pos_set: Option<Vec<PositionSet22>>,
}


// PositionSetBuyerAndSeller2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetBuyerAndSeller2 {
	#[serde(rename = "Buyr")]
	pub buyr: Option<PositionSetTotal2>,
	#[serde(rename = "Sellr")]
	pub sellr: Option<PositionSetTotal2>,
}


// PositionSetCollateralDimensions3 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetCollateralDimensions3 {
	#[serde(rename = "CtrPtyId")]
	pub ctr_pty_id: Option<TradeCounterpartyReport20>,
	#[serde(rename = "Coll")]
	pub coll: Option<MarginCollateralReport4>,
	#[serde(rename = "InitlMrgnPstdCcy")]
	pub initl_mrgn_pstd_ccy: Option<String>,
	#[serde(rename = "VartnMrgnPstdCcy")]
	pub vartn_mrgn_pstd_ccy: Option<String>,
	#[serde(rename = "InitlMrgnRcvdCcy")]
	pub initl_mrgn_rcvd_ccy: Option<String>,
	#[serde(rename = "VartnMrgnRcvdCcy")]
	pub vartn_mrgn_rcvd_ccy: Option<String>,
	#[serde(rename = "XcssCollPstdCcy")]
	pub xcss_coll_pstd_ccy: Option<String>,
	#[serde(rename = "XcssCollRcvdCcy")]
	pub xcss_coll_rcvd_ccy: Option<String>,
}


// PositionSetCollateralMetrics2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetCollateralMetrics2 {
	#[serde(rename = "Ttl")]
	pub ttl: Option<PositionSetCollateralTotal2>,
	#[serde(rename = "Clean")]
	pub clean: Option<PositionSetCollateralTotal2>,
}


// PositionSetCollateralTotal2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetCollateralTotal2 {
	#[serde(rename = "NbOfRpts")]
	pub nb_of_rpts: Option<f64>,
	#[serde(rename = "PstdMrgnOrColl")]
	pub pstd_mrgn_or_coll: Option<PostedMarginOrCollateral6>,
	#[serde(rename = "RcvdMrgnOrColl")]
	pub rcvd_mrgn_or_coll: Option<ReceivedMarginOrCollateral6>,
}


// PositionSetDimensions16 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetDimensions16 {
	#[serde(rename = "CtrPtyId")]
	pub ctr_pty_id: Option<TradeCounterpartyReport20>,
	#[serde(rename = "ValCcy")]
	pub val_ccy: Option<String>,
	#[serde(rename = "Coll")]
	pub coll: Option<MarginCollateralReport4>,
	#[serde(rename = "CtrctTp")]
	pub ctrct_tp: Option<String>,
	#[serde(rename = "AsstClss")]
	pub asst_clss: Option<String>,
	#[serde(rename = "UndrlygInstrm")]
	pub undrlyg_instrm: Option<SecurityIdentification41Choice>,
	#[serde(rename = "NtnlCcy")]
	pub ntnl_ccy: Option<String>,
	#[serde(rename = "NtnlCcyScndLeg")]
	pub ntnl_ccy_scnd_leg: Option<String>,
	#[serde(rename = "SttlmCcy")]
	pub sttlm_ccy: Option<String>,
	#[serde(rename = "SttlmCcyScndLeg")]
	pub sttlm_ccy_scnd_leg: Option<String>,
	#[serde(rename = "MstrAgrmt")]
	pub mstr_agrmt: Option<MasterAgreement8>,
	#[serde(rename = "Clrd")]
	pub clrd: Option<bool>,
	#[serde(rename = "IntraGrp")]
	pub intra_grp: Option<bool>,
	#[serde(rename = "XchgRateBsis")]
	pub xchg_rate_bsis: Option<ExchangeRateBasis1Choice>,
	#[serde(rename = "OptnTp")]
	pub optn_tp: Option<String>,
	#[serde(rename = "TmToMtrty")]
	pub tm_to_mtrty: Option<TimeToMaturity1Choice>,
	#[serde(rename = "IRSTp")]
	pub irs_tp: Option<String>,
	#[serde(rename = "Cdt")]
	pub cdt: Option<CreditDerivative7>,
	#[serde(rename = "Cmmdty")]
	pub cmmdty: Option<AssetClassCommodity6Choice>,
	#[serde(rename = "OthrPmt")]
	pub othr_pmt: Option<OtherPayment6>,
}


// PositionSetMetrics14 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetMetrics14 {
	#[serde(rename = "Ttl")]
	pub ttl: Option<PositionSetBuyerAndSeller2>,
	#[serde(rename = "Clean")]
	pub clean: Option<PositionSetBuyerAndSeller2>,
}


// PositionSetTotal2 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PositionSetTotal2 {
	#[serde(rename = "NbOfTrds")]
	pub nb_of_trds: Option<f64>,
	#[serde(rename = "PostvVal")]
	pub postv_val: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
	#[serde(rename = "NegVal")]
	pub neg_val: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
	#[serde(rename = "Ntnl")]
	pub ntnl: Option<NotionalAmountLegs6>,
	#[serde(rename = "OthrPmtAmt")]
	pub othr_pmt_amt: Option<Vec<ActiveOrHistoricCurrencyAnd19DecimalAmount>>,
}


// PostedMarginOrCollateral6 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostedMarginOrCollateral6 {
	#[serde(rename = "InitlMrgnPstdPreHrcut")]
	pub initl_mrgn_pstd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "InitlMrgnPstdPstHrcut")]
	pub initl_mrgn_pstd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "VartnMrgnPstdPreHrcut")]
	pub vartn_mrgn_pstd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "VartnMrgnPstdPstHrcut")]
	pub vartn_mrgn_pstd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "XcssCollPstd")]
	pub xcss_coll_pstd: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
}


// ProductType4Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ProductType4Code {
	#[serde(rename = "ProductType4Code")]
	pub product_type4_code: String,
}


// RateBasis1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct RateBasis1Code {
	#[serde(rename = "RateBasis1Code")]
	pub rate_basis1_code: String,
}


// ReceivedMarginOrCollateral6 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReceivedMarginOrCollateral6 {
	#[serde(rename = "InitlMrgnRcvdPreHrcut")]
	pub initl_mrgn_rcvd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "InitlMrgnRcvdPstHrcut")]
	pub initl_mrgn_rcvd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "VartnMrgnRcvdPreHrcut")]
	pub vartn_mrgn_rcvd_pre_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "VartnMrgnRcvdPstHrcut")]
	pub vartn_mrgn_rcvd_pst_hrcut: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
	#[serde(rename = "XcssCollRcvd")]
	pub xcss_coll_rcvd: Option<ActiveOrHistoricCurrencyAnd20DecimalAmount>,
}


// ReportPeriodActivity1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportPeriodActivity1Code {
	#[serde(rename = "ReportPeriodActivity1Code")]
	pub report_period_activity1_code: String,
}


// ReportingExemption1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportingExemption1 {
	#[serde(rename = "Rsn")]
	pub rsn: String,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// SecurityIdentification41Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification41Choice {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[serde(rename = "AltrntvInstrmId")]
	pub altrntv_instrm_id: Option<String>,
	#[serde(rename = "UnqPdctIdr")]
	pub unq_pdct_idr: Option<UniqueProductIdentifier2Choice>,
	#[serde(rename = "Bskt")]
	pub bskt: Option<CustomBasket4>,
	#[serde(rename = "Indx")]
	pub indx: Option<IndexIdentification1>,
	#[serde(rename = "Othr")]
	pub othr: Option<GenericIdentification184>,
	#[serde(rename = "IdNotAvlbl")]
	pub id_not_avlbl: Option<String>,
}


// SpecialPurpose2Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SpecialPurpose2Code {
	#[serde(rename = "SpecialPurpose2Code")]
	pub special_purpose2_code: String,
}


// SupplementaryData1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Option<String>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}


// TimeToMaturity1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeToMaturity1Choice {
	#[serde(rename = "Prd")]
	pub prd: Option<TimeToMaturityPeriod1>,
	#[serde(rename = "Spcl")]
	pub spcl: Option<String>,
}


// TimeToMaturityPeriod1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeToMaturityPeriod1 {
	#[serde(rename = "Start")]
	pub start: Option<MaturityTerm2>,
	#[serde(rename = "End")]
	pub end: Option<MaturityTerm2>,
}


// TradeCounterpartyRelationship1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeCounterpartyRelationship1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// TradeCounterpartyRelationshipRecord1 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeCounterpartyRelationshipRecord1 {
	#[serde(rename = "StartRltshPty")]
	pub start_rltsh_pty: String,
	#[serde(rename = "EndRltshPty")]
	pub end_rltsh_pty: String,
	#[serde(rename = "RltshTp")]
	pub rltsh_tp: TradeCounterpartyRelationship1Choice,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// TradeCounterpartyReport20 ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeCounterpartyReport20 {
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: Counterparty45,
	#[serde(rename = "OthrCtrPty")]
	pub othr_ctr_pty: Counterparty46,
	#[serde(rename = "Brkr")]
	pub brkr: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "SubmitgAgt")]
	pub submitg_agt: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "ClrMmb")]
	pub clr_mmb: Option<PartyIdentification248Choice>,
	#[serde(rename = "Bnfcry")]
	pub bnfcry: Option<Vec<PartyIdentification248Choice>>,
	#[serde(rename = "NttyRspnsblForRpt")]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "ExctnAgt")]
	pub exctn_agt: Option<Vec<OrganisationIdentification15Choice>>,
	#[serde(rename = "RltshRcrd")]
	pub rltsh_rcrd: Option<Vec<TradeCounterpartyRelationshipRecord1>>,
}


// TradeCounterpartyType1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeCounterpartyType1Code {
	#[serde(rename = "TradeCounterpartyType1Code")]
	pub trade_counterparty_type1_code: String,
}


// TradingCapacity7Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingCapacity7Code {
	#[serde(rename = "TradingCapacity7Code")]
	pub trading_capacity7_code: String,
}


// TrueFalseIndicator ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}


// UnderlyingIdentification1Code ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingIdentification1Code {
	#[serde(rename = "UnderlyingIdentification1Code")]
	pub underlying_identification1_code: String,
}


// UniqueProductIdentifier1Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UniqueProductIdentifier1Choice {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification175>,
}


// UniqueProductIdentifier2Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UniqueProductIdentifier2Choice {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification185>,
}


// UnitOfMeasure8Choice ...
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitOfMeasure8Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification175>,
}
