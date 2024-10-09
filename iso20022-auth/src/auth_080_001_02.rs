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


// ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType {
	#[serde(rename = "ActiveOrHistoricCurrencyAnd20DecimalAmount_SimpleType")]
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


// ActiveOrHistoricCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
	#[serde(rename = "ActiveOrHistoricCurrencyAndAmount_SimpleType")]
	pub active_or_historic_currency_and_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyCode {
	#[serde(rename = "ActiveOrHistoricCurrencyCode")]
	pub active_or_historic_currency_code: String,
}


// AgreementType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgreementType1Choice {
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// AgreementType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgreementType2Choice {
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// AgriculturalCommodityDairy1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityDairy1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// AgriculturalCommodityForestry1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityForestry1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// AgriculturalCommodityGrain2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityGrain2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: String,
}


// AgriculturalCommodityLiveStock1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityLiveStock1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// AgriculturalCommodityOilSeed1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityOilSeed1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: String,
}


// AgriculturalCommodityOliveOil2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityOliveOil2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: String,
}


// AgriculturalCommodityOther1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// AgriculturalCommodityPotato1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityPotato1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// AgriculturalCommoditySeafood1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommoditySeafood1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// AgriculturalCommoditySoft1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommoditySoft1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: String,
}


// AmountAndDirection107 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection107 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAnd20DecimalAmount,
	#[serde(rename = "Sgn")]
	pub sgn: Option<bool>,
}


// AmountAndDirection53 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection53 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "Sgn")]
	pub sgn: Option<bool>,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// AssetClassCommodity5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodity5Choice {
	#[serde(rename = "Agrcltrl")]
	pub agrcltrl: Option<AssetClassCommodityAgricultural5Choice>,
	#[serde(rename = "Nrgy")]
	pub nrgy: Option<AssetClassCommodityEnergy2Choice>,
	#[serde(rename = "Envttl")]
	pub envttl: Option<AssetClassCommodityEnvironmental2Choice>,
	#[serde(rename = "Frtlzr")]
	pub frtlzr: Option<AssetClassCommodityFertilizer3Choice>,
	#[serde(rename = "Frght")]
	pub frght: Option<AssetClassCommodityFreight3Choice>,
	#[serde(rename = "IndstrlPdct")]
	pub indstrl_pdct: Option<AssetClassCommodityIndustrialProduct1Choice>,
	#[serde(rename = "Metl")]
	pub metl: Option<AssetClassCommodityMetal1Choice>,
	#[serde(rename = "OthrC10")]
	pub othr_c10: Option<AssetClassCommodityOtherC102Choice>,
	#[serde(rename = "Ppr")]
	pub ppr: Option<AssetClassCommodityPaper3Choice>,
	#[serde(rename = "Plprpln")]
	pub plprpln: Option<AssetClassCommodityPolypropylene3Choice>,
	#[serde(rename = "Infltn")]
	pub infltn: Option<AssetClassCommodityInflation1>,
	#[serde(rename = "MultiCmmdtyExtc")]
	pub multi_cmmdty_extc: Option<AssetClassCommodityMultiCommodityExotic1>,
	#[serde(rename = "OffclEcnmcSttstcs")]
	pub offcl_ecnmc_sttstcs: Option<AssetClassCommodityOfficialEconomicStatistics1>,
	#[serde(rename = "Othr")]
	pub othr: Option<AssetClassCommodityOther1>,
}


// AssetClassCommodityAgricultural5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityAgricultural5Choice {
	#[serde(rename = "GrnOilSeed")]
	pub grn_oil_seed: Option<AgriculturalCommodityOilSeed1>,
	#[serde(rename = "Soft")]
	pub soft: Option<AgriculturalCommoditySoft1>,
	#[serde(rename = "Ptt")]
	pub ptt: Option<AgriculturalCommodityPotato1>,
	#[serde(rename = "OlvOil")]
	pub olv_oil: Option<AgriculturalCommodityOliveOil2>,
	#[serde(rename = "Dairy")]
	pub dairy: Option<AgriculturalCommodityDairy1>,
	#[serde(rename = "Frstry")]
	pub frstry: Option<AgriculturalCommodityForestry1>,
	#[serde(rename = "Sfd")]
	pub sfd: Option<AgriculturalCommoditySeafood1>,
	#[serde(rename = "LiveStock")]
	pub live_stock: Option<AgriculturalCommodityLiveStock1>,
	#[serde(rename = "Grn")]
	pub grn: Option<AgriculturalCommodityGrain2>,
	#[serde(rename = "Othr")]
	pub othr: Option<AgriculturalCommodityOther1>,
}


// AssetClassCommodityEnergy2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityEnergy2Choice {
	#[serde(rename = "Elctrcty")]
	pub elctrcty: Option<EnergyCommodityElectricity1>,
	#[serde(rename = "NtrlGas")]
	pub ntrl_gas: Option<EnergyCommodityNaturalGas2>,
	#[serde(rename = "Oil")]
	pub oil: Option<EnergyCommodityOil2>,
	#[serde(rename = "Coal")]
	pub coal: Option<EnergyCommodityCoal1>,
	#[serde(rename = "IntrNrgy")]
	pub intr_nrgy: Option<EnergyCommodityInterEnergy1>,
	#[serde(rename = "RnwblNrgy")]
	pub rnwbl_nrgy: Option<EnergyCommodityRenewableEnergy1>,
	#[serde(rename = "LghtEnd")]
	pub lght_end: Option<EnergyCommodityLightEnd1>,
	#[serde(rename = "Dstllts")]
	pub dstllts: Option<EnergyCommodityDistillates1>,
	#[serde(rename = "Othr")]
	pub othr: Option<EnergyCommodityOther1>,
}


// AssetClassCommodityEnvironmental2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityEnvironmental2Choice {
	#[serde(rename = "Emssns")]
	pub emssns: Option<EnvironmentalCommodityEmission2>,
	#[serde(rename = "Wthr")]
	pub wthr: Option<EnvironmentalCommodityWeather1>,
	#[serde(rename = "CrbnRltd")]
	pub crbn_rltd: Option<EnvironmentalCommodityCarbonRelated1>,
	#[serde(rename = "Othr")]
	pub othr: Option<EnvironmentCommodityOther1>,
}


// AssetClassCommodityFertilizer3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityFertilizer3Choice {
	#[serde(rename = "Ammn")]
	pub ammn: Option<FertilizerCommodityAmmonia1>,
	#[serde(rename = "DmmnmPhspht")]
	pub dmmnm_phspht: Option<FertilizerCommodityDiammoniumPhosphate1>,
	#[serde(rename = "Ptsh")]
	pub ptsh: Option<FertilizerCommodityPotash1>,
	#[serde(rename = "Slphr")]
	pub slphr: Option<FertilizerCommoditySulphur1>,
	#[serde(rename = "Urea")]
	pub urea: Option<FertilizerCommodityUrea1>,
	#[serde(rename = "UreaAndAmmnmNtrt")]
	pub urea_and_ammnm_ntrt: Option<FertilizerCommodityUreaAndAmmoniumNitrate1>,
	#[serde(rename = "Othr")]
	pub othr: Option<FertilizerCommodityOther1>,
}


// AssetClassCommodityFreight3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityFreight3Choice {
	#[serde(rename = "Dry")]
	pub dry: Option<FreightCommodityDry2>,
	#[serde(rename = "Wet")]
	pub wet: Option<FreightCommodityWet2>,
	#[serde(rename = "CntnrShip")]
	pub cntnr_ship: Option<FreightCommodityContainerShip1>,
	#[serde(rename = "Othr")]
	pub othr: Option<FreightCommodityOther1>,
}


// AssetClassCommodityIndustrialProduct1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityIndustrialProduct1Choice {
	#[serde(rename = "Cnstrctn")]
	pub cnstrctn: Option<IndustrialProductCommodityConstruction1>,
	#[serde(rename = "Manfctg")]
	pub manfctg: Option<IndustrialProductCommodityManufacturing1>,
}


// AssetClassCommodityInflation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityInflation1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
}


// AssetClassCommodityMetal1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityMetal1Choice {
	#[serde(rename = "NonPrcs")]
	pub non_prcs: Option<MetalCommodityNonPrecious1>,
	#[serde(rename = "Prcs")]
	pub prcs: Option<MetalCommodityPrecious1>,
}


// AssetClassCommodityMultiCommodityExotic1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityMultiCommodityExotic1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
}


// AssetClassCommodityOfficialEconomicStatistics1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityOfficialEconomicStatistics1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
}


// AssetClassCommodityOther1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
}


// AssetClassCommodityOtherC102Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityOtherC102Choice {
	#[serde(rename = "Dlvrbl")]
	pub dlvrbl: Option<OtherC10CommodityDeliverable2>,
	#[serde(rename = "NonDlvrbl")]
	pub non_dlvrbl: Option<OtherC10CommodityNonDeliverable2>,
}


// AssetClassCommodityPaper3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityPaper3Choice {
	#[serde(rename = "CntnrBrd")]
	pub cntnr_brd: Option<PaperCommodityContainerBoard1>,
	#[serde(rename = "Nwsprnt")]
	pub nwsprnt: Option<PaperCommodityNewsprint1>,
	#[serde(rename = "Pulp")]
	pub pulp: Option<PaperCommodityPulp1>,
	#[serde(rename = "RcvrdPpr")]
	pub rcvrd_ppr: Option<PaperCommodityRecoveredPaper1>,
	#[serde(rename = "Othr")]
	pub othr: Option<PaperCommodityRecoveredPaper2>,
}


// AssetClassCommodityPolypropylene3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityPolypropylene3Choice {
	#[serde(rename = "Plstc")]
	pub plstc: Option<PolypropyleneCommodityPlastic1>,
	#[serde(rename = "Othr")]
	pub othr: Option<PolypropyleneCommodityOther1>,
}


// AssetClassDetailedSubProductType10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType10Code {
	#[serde(rename = "AssetClassDetailedSubProductType10Code")]
	pub asset_class_detailed_sub_product_type10_code: String,
}


// AssetClassDetailedSubProductType11Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType11Code {
	#[serde(rename = "AssetClassDetailedSubProductType11Code")]
	pub asset_class_detailed_sub_product_type11_code: String,
}


// AssetClassDetailedSubProductType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType1Code {
	#[serde(rename = "AssetClassDetailedSubProductType1Code")]
	pub asset_class_detailed_sub_product_type1_code: String,
}


// AssetClassDetailedSubProductType29Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType29Code {
	#[serde(rename = "AssetClassDetailedSubProductType29Code")]
	pub asset_class_detailed_sub_product_type29_code: String,
}


// AssetClassDetailedSubProductType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType2Code {
	#[serde(rename = "AssetClassDetailedSubProductType2Code")]
	pub asset_class_detailed_sub_product_type2_code: String,
}


// AssetClassDetailedSubProductType30Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType30Code {
	#[serde(rename = "AssetClassDetailedSubProductType30Code")]
	pub asset_class_detailed_sub_product_type30_code: String,
}


// AssetClassDetailedSubProductType31Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType31Code {
	#[serde(rename = "AssetClassDetailedSubProductType31Code")]
	pub asset_class_detailed_sub_product_type31_code: String,
}


// AssetClassDetailedSubProductType32Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType32Code {
	#[serde(rename = "AssetClassDetailedSubProductType32Code")]
	pub asset_class_detailed_sub_product_type32_code: String,
}


// AssetClassDetailedSubProductType33Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType33Code {
	#[serde(rename = "AssetClassDetailedSubProductType33Code")]
	pub asset_class_detailed_sub_product_type33_code: String,
}


// AssetClassDetailedSubProductType34Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType34Code {
	#[serde(rename = "AssetClassDetailedSubProductType34Code")]
	pub asset_class_detailed_sub_product_type34_code: String,
}


// AssetClassDetailedSubProductType5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType5Code {
	#[serde(rename = "AssetClassDetailedSubProductType5Code")]
	pub asset_class_detailed_sub_product_type5_code: String,
}


// AssetClassDetailedSubProductType8Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType8Code {
	#[serde(rename = "AssetClassDetailedSubProductType8Code")]
	pub asset_class_detailed_sub_product_type8_code: String,
}


// AssetClassProductType11Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType11Code {
	#[serde(rename = "AssetClassProductType11Code")]
	pub asset_class_product_type11_code: String,
}


// AssetClassProductType12Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType12Code {
	#[serde(rename = "AssetClassProductType12Code")]
	pub asset_class_product_type12_code: String,
}


// AssetClassProductType13Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType13Code {
	#[serde(rename = "AssetClassProductType13Code")]
	pub asset_class_product_type13_code: String,
}


// AssetClassProductType14Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType14Code {
	#[serde(rename = "AssetClassProductType14Code")]
	pub asset_class_product_type14_code: String,
}


// AssetClassProductType15Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType15Code {
	#[serde(rename = "AssetClassProductType15Code")]
	pub asset_class_product_type15_code: String,
}


// AssetClassProductType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType1Code {
	#[serde(rename = "AssetClassProductType1Code")]
	pub asset_class_product_type1_code: String,
}


// AssetClassProductType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType2Code {
	#[serde(rename = "AssetClassProductType2Code")]
	pub asset_class_product_type2_code: String,
}


// AssetClassProductType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType3Code {
	#[serde(rename = "AssetClassProductType3Code")]
	pub asset_class_product_type3_code: String,
}


// AssetClassProductType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType4Code {
	#[serde(rename = "AssetClassProductType4Code")]
	pub asset_class_product_type4_code: String,
}


// AssetClassProductType5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType5Code {
	#[serde(rename = "AssetClassProductType5Code")]
	pub asset_class_product_type5_code: String,
}


// AssetClassProductType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType6Code {
	#[serde(rename = "AssetClassProductType6Code")]
	pub asset_class_product_type6_code: String,
}


// AssetClassProductType7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType7Code {
	#[serde(rename = "AssetClassProductType7Code")]
	pub asset_class_product_type7_code: String,
}


// AssetClassProductType8Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType8Code {
	#[serde(rename = "AssetClassProductType8Code")]
	pub asset_class_product_type8_code: String,
}


// AssetClassProductType9Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassProductType9Code {
	#[serde(rename = "AssetClassProductType9Code")]
	pub asset_class_product_type9_code: String,
}


// AssetClassSubProductType10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType10Code {
	#[serde(rename = "AssetClassSubProductType10Code")]
	pub asset_class_sub_product_type10_code: String,
}


// AssetClassSubProductType15Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType15Code {
	#[serde(rename = "AssetClassSubProductType15Code")]
	pub asset_class_sub_product_type15_code: String,
}


// AssetClassSubProductType16Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType16Code {
	#[serde(rename = "AssetClassSubProductType16Code")]
	pub asset_class_sub_product_type16_code: String,
}


// AssetClassSubProductType18Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType18Code {
	#[serde(rename = "AssetClassSubProductType18Code")]
	pub asset_class_sub_product_type18_code: String,
}


// AssetClassSubProductType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType1Code {
	#[serde(rename = "AssetClassSubProductType1Code")]
	pub asset_class_sub_product_type1_code: String,
}


// AssetClassSubProductType20Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType20Code {
	#[serde(rename = "AssetClassSubProductType20Code")]
	pub asset_class_sub_product_type20_code: String,
}


// AssetClassSubProductType21Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType21Code {
	#[serde(rename = "AssetClassSubProductType21Code")]
	pub asset_class_sub_product_type21_code: String,
}


// AssetClassSubProductType22Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType22Code {
	#[serde(rename = "AssetClassSubProductType22Code")]
	pub asset_class_sub_product_type22_code: String,
}


// AssetClassSubProductType23Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType23Code {
	#[serde(rename = "AssetClassSubProductType23Code")]
	pub asset_class_sub_product_type23_code: String,
}


// AssetClassSubProductType24Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType24Code {
	#[serde(rename = "AssetClassSubProductType24Code")]
	pub asset_class_sub_product_type24_code: String,
}


// AssetClassSubProductType25Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType25Code {
	#[serde(rename = "AssetClassSubProductType25Code")]
	pub asset_class_sub_product_type25_code: String,
}


// AssetClassSubProductType26Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType26Code {
	#[serde(rename = "AssetClassSubProductType26Code")]
	pub asset_class_sub_product_type26_code: String,
}


// AssetClassSubProductType27Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType27Code {
	#[serde(rename = "AssetClassSubProductType27Code")]
	pub asset_class_sub_product_type27_code: String,
}


// AssetClassSubProductType28Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType28Code {
	#[serde(rename = "AssetClassSubProductType28Code")]
	pub asset_class_sub_product_type28_code: String,
}


// AssetClassSubProductType29Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType29Code {
	#[serde(rename = "AssetClassSubProductType29Code")]
	pub asset_class_sub_product_type29_code: String,
}


// AssetClassSubProductType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType2Code {
	#[serde(rename = "AssetClassSubProductType2Code")]
	pub asset_class_sub_product_type2_code: String,
}


// AssetClassSubProductType30Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType30Code {
	#[serde(rename = "AssetClassSubProductType30Code")]
	pub asset_class_sub_product_type30_code: String,
}


// AssetClassSubProductType31Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType31Code {
	#[serde(rename = "AssetClassSubProductType31Code")]
	pub asset_class_sub_product_type31_code: String,
}


// AssetClassSubProductType32Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType32Code {
	#[serde(rename = "AssetClassSubProductType32Code")]
	pub asset_class_sub_product_type32_code: String,
}


// AssetClassSubProductType33Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType33Code {
	#[serde(rename = "AssetClassSubProductType33Code")]
	pub asset_class_sub_product_type33_code: String,
}


// AssetClassSubProductType34Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType34Code {
	#[serde(rename = "AssetClassSubProductType34Code")]
	pub asset_class_sub_product_type34_code: String,
}


// AssetClassSubProductType35Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType35Code {
	#[serde(rename = "AssetClassSubProductType35Code")]
	pub asset_class_sub_product_type35_code: String,
}


// AssetClassSubProductType36Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType36Code {
	#[serde(rename = "AssetClassSubProductType36Code")]
	pub asset_class_sub_product_type36_code: String,
}


// AssetClassSubProductType37Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType37Code {
	#[serde(rename = "AssetClassSubProductType37Code")]
	pub asset_class_sub_product_type37_code: String,
}


// AssetClassSubProductType38Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType38Code {
	#[serde(rename = "AssetClassSubProductType38Code")]
	pub asset_class_sub_product_type38_code: String,
}


// AssetClassSubProductType39Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType39Code {
	#[serde(rename = "AssetClassSubProductType39Code")]
	pub asset_class_sub_product_type39_code: String,
}


// AssetClassSubProductType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType3Code {
	#[serde(rename = "AssetClassSubProductType3Code")]
	pub asset_class_sub_product_type3_code: String,
}


// AssetClassSubProductType40Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType40Code {
	#[serde(rename = "AssetClassSubProductType40Code")]
	pub asset_class_sub_product_type40_code: String,
}


// AssetClassSubProductType41Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType41Code {
	#[serde(rename = "AssetClassSubProductType41Code")]
	pub asset_class_sub_product_type41_code: String,
}


// AssetClassSubProductType42Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType42Code {
	#[serde(rename = "AssetClassSubProductType42Code")]
	pub asset_class_sub_product_type42_code: String,
}


// AssetClassSubProductType43Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType43Code {
	#[serde(rename = "AssetClassSubProductType43Code")]
	pub asset_class_sub_product_type43_code: String,
}


// AssetClassSubProductType44Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType44Code {
	#[serde(rename = "AssetClassSubProductType44Code")]
	pub asset_class_sub_product_type44_code: String,
}


// AssetClassSubProductType45Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType45Code {
	#[serde(rename = "AssetClassSubProductType45Code")]
	pub asset_class_sub_product_type45_code: String,
}


// AssetClassSubProductType46Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType46Code {
	#[serde(rename = "AssetClassSubProductType46Code")]
	pub asset_class_sub_product_type46_code: String,
}


// AssetClassSubProductType47Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType47Code {
	#[serde(rename = "AssetClassSubProductType47Code")]
	pub asset_class_sub_product_type47_code: String,
}


// AssetClassSubProductType48Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType48Code {
	#[serde(rename = "AssetClassSubProductType48Code")]
	pub asset_class_sub_product_type48_code: String,
}


// AssetClassSubProductType49Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType49Code {
	#[serde(rename = "AssetClassSubProductType49Code")]
	pub asset_class_sub_product_type49_code: String,
}


// AssetClassSubProductType5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType5Code {
	#[serde(rename = "AssetClassSubProductType5Code")]
	pub asset_class_sub_product_type5_code: String,
}


// AssetClassSubProductType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType6Code {
	#[serde(rename = "AssetClassSubProductType6Code")]
	pub asset_class_sub_product_type6_code: String,
}


// AssetClassSubProductType7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType7Code {
	#[serde(rename = "AssetClassSubProductType7Code")]
	pub asset_class_sub_product_type7_code: String,
}


// AssetClassSubProductType8Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassSubProductType8Code {
	#[serde(rename = "AssetClassSubProductType8Code")]
	pub asset_class_sub_product_type8_code: String,
}


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BaseOneRate {
	#[serde(rename = "BaseOneRate")]
	pub base_one_rate: f64,
}


// BenchmarkCurveName10Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkCurveName10Choice {
	#[serde(rename = "Indx")]
	pub indx: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
}


// BenchmarkCurveName3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkCurveName3Code {
	#[serde(rename = "BenchmarkCurveName3Code")]
	pub benchmark_curve_name3_code: String,
}


// CFIOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[serde(rename = "CFIOct2015Identifier")]
	pub cfi_oct2015_identifier: String,
}


// CashCompare3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashCompare3 {
	#[serde(rename = "Val")]
	pub val: Option<CompareAmountAndDirection2>,
	#[serde(rename = "HrcutOrMrgn")]
	pub hrcut_or_mrgn: Option<ComparePercentageRate3>,
}


// Cleared4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Cleared4Choice {
	#[serde(rename = "Clrd")]
	pub clrd: Option<String>,
	#[serde(rename = "NonClrd")]
	pub non_clrd: Option<String>,
}


// CollateralDeliveryMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralDeliveryMethod1Code {
	#[serde(rename = "CollateralDeliveryMethod1Code")]
	pub collateral_delivery_method1_code: String,
}


// CollateralMatchingCriteria6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralMatchingCriteria6 {
	#[serde(rename = "UncollsdFlg")]
	pub uncollsd_flg: Option<CompareTrueFalseIndicator3>,
	#[serde(rename = "NetXpsrCollstnInd")]
	pub net_xpsr_collstn_ind: Option<CompareTrueFalseIndicator3>,
	#[serde(rename = "CollValDt")]
	pub coll_val_dt: Option<CompareDate3>,
	#[serde(rename = "AsstTp")]
	pub asst_tp: Option<SecurityCommodityCash4>,
	#[serde(rename = "BsktIdr")]
	pub bskt_idr: Option<CompareSecurityIdentification4>,
}


// CollateralQualityType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralQualityType1Code {
	#[serde(rename = "CollateralQualityType1Code")]
	pub collateral_quality_type1_code: String,
}


// CollateralRole1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralRole1Code {
	#[serde(rename = "CollateralRole1Code")]
	pub collateral_role1_code: String,
}


// Commodity42 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Commodity42 {
	#[serde(rename = "Clssfctn")]
	pub clssfctn: Option<CompareCommodityAssetClass3>,
	#[serde(rename = "Qty")]
	pub qty: Option<CompareDecimalNumber3>,
	#[serde(rename = "UnitPric")]
	pub unit_pric: Option<CompareUnitPrice6>,
	#[serde(rename = "MktVal")]
	pub mkt_val: Option<CompareAmountAndDirection2>,
	#[serde(rename = "UnitOfMeasr")]
	pub unit_of_measr: Option<CompareUnitOfMeasure3>,
}


// CompareActiveOrHistoricCurrencyAndAmount3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareActiveOrHistoricCurrencyAndAmount3 {
	#[serde(rename = "Val1")]
	pub val1: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Val2")]
	pub val2: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// CompareAgreementType2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareAgreementType2 {
	#[serde(rename = "Val1")]
	pub val1: Option<AgreementType1Choice>,
	#[serde(rename = "Val2")]
	pub val2: Option<AgreementType1Choice>,
}


// CompareAmountAndDirection1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareAmountAndDirection1 {
	#[serde(rename = "Val1")]
	pub val1: Option<AmountAndDirection53>,
	#[serde(rename = "Val2")]
	pub val2: Option<AmountAndDirection53>,
}


// CompareAmountAndDirection2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareAmountAndDirection2 {
	#[serde(rename = "Val1")]
	pub val1: Option<AmountAndDirection53>,
	#[serde(rename = "Val2")]
	pub val2: Option<AmountAndDirection53>,
}


// CompareBenchmarkCurveName3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareBenchmarkCurveName3 {
	#[serde(rename = "Val1")]
	pub val1: Option<BenchmarkCurveName10Choice>,
	#[serde(rename = "Val2")]
	pub val2: Option<BenchmarkCurveName10Choice>,
}


// CompareCFIIdentifier3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareCFIIdentifier3 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareClearingStatus3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareClearingStatus3 {
	#[serde(rename = "Val1")]
	pub val1: Option<Cleared4Choice>,
	#[serde(rename = "Val2")]
	pub val2: Option<Cleared4Choice>,
}


// CompareCollateralQualityType3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareCollateralQualityType3 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareCommodityAssetClass3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareCommodityAssetClass3 {
	#[serde(rename = "Val1")]
	pub val1: Option<AssetClassCommodity5Choice>,
	#[serde(rename = "Val2")]
	pub val2: Option<AssetClassCommodity5Choice>,
}


// CompareCounterpartySide2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareCounterpartySide2 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareCountryCode3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareCountryCode3 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareDate3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareDate3 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareDateTime3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareDateTime3 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareDecimalNumber3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareDecimalNumber3 {
	#[serde(rename = "Val1")]
	pub val1: Option<f64>,
	#[serde(rename = "Val2")]
	pub val2: Option<f64>,
}


// CompareDeliveryMethod3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareDeliveryMethod3 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareExposureType3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareExposureType3 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareISINIdentifier4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareISINIdentifier4 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareInterestComputationMethod3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareInterestComputationMethod3 {
	#[serde(rename = "Val1")]
	pub val1: Option<InterestComputationMethodFormat6Choice>,
	#[serde(rename = "Val2")]
	pub val2: Option<InterestComputationMethodFormat6Choice>,
}


// CompareInterestRate1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareInterestRate1 {
	#[serde(rename = "MrgnLnAmt")]
	pub mrgn_ln_amt: Option<CompareAmountAndDirection1>,
	#[serde(rename = "FxdIntrstRate")]
	pub fxd_intrst_rate: Option<ComparePercentageRate3>,
	#[serde(rename = "DayCntBsis")]
	pub day_cnt_bsis: Option<CompareInterestComputationMethod3>,
	#[serde(rename = "FltgIntrstRefRate")]
	pub fltg_intrst_ref_rate: Option<CompareBenchmarkCurveName3>,
	#[serde(rename = "FltgIntrstRateTermUnit")]
	pub fltg_intrst_rate_term_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgIntrstRateTermVal")]
	pub fltg_intrst_rate_term_val: Option<CompareNumber5>,
	#[serde(rename = "FltgIntrstRatePmtFrqcyUnit")]
	pub fltg_intrst_rate_pmt_frqcy_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgIntrstRatePmtFrqcyVal")]
	pub fltg_intrst_rate_pmt_frqcy_val: Option<CompareNumber5>,
	#[serde(rename = "FltgIntrstRateRstFrqcyUnit")]
	pub fltg_intrst_rate_rst_frqcy_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgIntrstRateRstFrqcyVal")]
	pub fltg_intrst_rate_rst_frqcy_val: Option<CompareNumber6>,
	#[serde(rename = "BsisPtSprd")]
	pub bsis_pt_sprd: Option<CompareDecimalNumber3>,
}


// CompareMICIdentifier3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareMICIdentifier3 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareNumber5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareNumber5 {
	#[serde(rename = "Val1")]
	pub val1: Option<f64>,
	#[serde(rename = "Val2")]
	pub val2: Option<f64>,
}


// CompareNumber6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareNumber6 {
	#[serde(rename = "Val1")]
	pub val1: Option<f64>,
	#[serde(rename = "Val2")]
	pub val2: Option<f64>,
}


// CompareOrganisationIdentification6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareOrganisationIdentification6 {
	#[serde(rename = "Val1")]
	pub val1: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "Val2")]
	pub val2: Option<OrganisationIdentification15Choice>,
}


// CompareOrganisationIdentification7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareOrganisationIdentification7 {
	#[serde(rename = "Val1")]
	pub val1: Option<PartyIdentification236Choice>,
	#[serde(rename = "Val2")]
	pub val2: Option<PartyIdentification236Choice>,
}


// ComparePercentageRate3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ComparePercentageRate3 {
	#[serde(rename = "Val1")]
	pub val1: Option<f64>,
	#[serde(rename = "Val2")]
	pub val2: Option<f64>,
}


// CompareRateBasis3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareRateBasis3 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareReportingLevelType3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareReportingLevelType3 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareSecuritiesLendingType3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareSecuritiesLendingType3 {
	#[serde(rename = "Val1")]
	pub val1: Option<SecuritiesLendingType3Choice>,
	#[serde(rename = "Val2")]
	pub val2: Option<SecuritiesLendingType3Choice>,
}


// CompareSecurityIdentification4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareSecurityIdentification4 {
	#[serde(rename = "Val1")]
	pub val1: Option<SecurityIdentification26Choice>,
	#[serde(rename = "Val2")]
	pub val2: Option<SecurityIdentification26Choice>,
}


// CompareSpecialCollateral3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareSpecialCollateral3 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareTerminationOption3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareTerminationOption3 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareText2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareText2 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareTrueFalseIndicator3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareTrueFalseIndicator3 {
	#[serde(rename = "Val1")]
	pub val1: Option<bool>,
	#[serde(rename = "Val2")]
	pub val2: Option<bool>,
}


// CompareUnitOfMeasure3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareUnitOfMeasure3 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareUnitPrice6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareUnitPrice6 {
	#[serde(rename = "Val1")]
	pub val1: Option<SecuritiesTransactionPrice19Choice>,
	#[serde(rename = "Val2")]
	pub val2: Option<SecuritiesTransactionPrice19Choice>,
}


// CounterpartyMatchingCriteria4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyMatchingCriteria4 {
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: Option<CompareOrganisationIdentification6>,
	#[serde(rename = "OthrCtrPty")]
	pub othr_ctr_pty: Option<CompareOrganisationIdentification7>,
	#[serde(rename = "CtrPtySd")]
	pub ctr_pty_sd: Option<CompareCounterpartySide2>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "DecimalNumber")]
	pub decimal_number: f64,
}


// EnergyCommodityCoal1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityCoal1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// EnergyCommodityDistillates1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityDistillates1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// EnergyCommodityElectricity1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityElectricity1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: String,
}


// EnergyCommodityInterEnergy1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityInterEnergy1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// EnergyCommodityLightEnd1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityLightEnd1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// EnergyCommodityNaturalGas2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityNaturalGas2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: String,
}


// EnergyCommodityOil2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityOil2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: String,
}


// EnergyCommodityOther1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// EnergyCommodityRenewableEnergy1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityRenewableEnergy1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// EnvironmentCommodityOther1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// EnvironmentalCommodityCarbonRelated1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCommodityCarbonRelated1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// EnvironmentalCommodityEmission2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCommodityEmission2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: String,
}


// EnvironmentalCommodityWeather1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCommodityWeather1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// ExposureType10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExposureType10Code {
	#[serde(rename = "ExposureType10Code")]
	pub exposure_type10_code: String,
}


// ExternalAgreementType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAgreementType1Code {
	#[serde(rename = "ExternalAgreementType1Code")]
	pub external_agreement_type1_code: String,
}


// ExternalSecuritiesLendingType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalSecuritiesLendingType1Code {
	#[serde(rename = "ExternalSecuritiesLendingType1Code")]
	pub external_securities_lending_type1_code: String,
}


// FertilizerCommodityAmmonia1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityAmmonia1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// FertilizerCommodityDiammoniumPhosphate1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityDiammoniumPhosphate1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// FertilizerCommodityOther1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// FertilizerCommodityPotash1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityPotash1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// FertilizerCommoditySulphur1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommoditySulphur1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// FertilizerCommodityUrea1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityUrea1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// FertilizerCommodityUreaAndAmmoniumNitrate1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityUreaAndAmmoniumNitrate1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// FreightCommodityContainerShip1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityContainerShip1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// FreightCommodityDry2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityDry2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: String,
}


// FreightCommodityOther1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// FreightCommodityWet2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityWet2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: String,
}


// GenericIdentification175 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification175 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// ISINOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[serde(rename = "ISINOct2015Identifier")]
	pub isin_oct2015_identifier: String,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// IndustrialProductCommodityConstruction1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndustrialProductCommodityConstruction1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// IndustrialProductCommodityManufacturing1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndustrialProductCommodityManufacturing1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// InterestComputationMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestComputationMethod1Code {
	#[serde(rename = "InterestComputationMethod1Code")]
	pub interest_computation_method1_code: String,
}


// InterestComputationMethodFormat6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestComputationMethodFormat6Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LoanMatchingCriteria9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LoanMatchingCriteria9 {
	#[serde(rename = "UnqTradIdr")]
	pub unq_trad_idr: Option<CompareText2>,
	#[serde(rename = "TermntnDt")]
	pub termntn_dt: Option<CompareDate3>,
	#[serde(rename = "CtrctTp")]
	pub ctrct_tp: Option<CompareExposureType3>,
	#[serde(rename = "ClrSts")]
	pub clr_sts: Option<CompareClearingStatus3>,
	#[serde(rename = "ClrDtTm")]
	pub clr_dt_tm: Option<CompareDateTime3>,
	#[serde(rename = "CCP")]
	pub ccp: Option<CompareOrganisationIdentification6>,
	#[serde(rename = "TradgVn")]
	pub tradg_vn: Option<CompareMICIdentifier3>,
	#[serde(rename = "MstrAgrmtTp")]
	pub mstr_agrmt_tp: Option<CompareAgreementType2>,
	#[serde(rename = "ExctnDtTm")]
	pub exctn_dt_tm: Option<CompareDateTime3>,
	#[serde(rename = "ValDt")]
	pub val_dt: Option<CompareDate3>,
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: Option<CompareDate3>,
	#[serde(rename = "MinNtcePrd")]
	pub min_ntce_prd: Option<CompareNumber5>,
	#[serde(rename = "EarlstCallBckDt")]
	pub earlst_call_bck_dt: Option<CompareDate3>,
	#[serde(rename = "GnlColl")]
	pub gnl_coll: Option<CompareSpecialCollateral3>,
	#[serde(rename = "DlvryByVal")]
	pub dlvry_by_val: Option<CompareTrueFalseIndicator3>,
	#[serde(rename = "CollDlvryMtd")]
	pub coll_dlvry_mtd: Option<CompareDeliveryMethod3>,
	#[serde(rename = "OpnTerm")]
	pub opn_term: Option<CompareTrueFalseIndicator3>,
	#[serde(rename = "TermntnOptn")]
	pub termntn_optn: Option<CompareTerminationOption3>,
	#[serde(rename = "FxdIntrstRate")]
	pub fxd_intrst_rate: Option<ComparePercentageRate3>,
	#[serde(rename = "DayCntBsis")]
	pub day_cnt_bsis: Option<CompareInterestComputationMethod3>,
	#[serde(rename = "FltgIntrstRefRate")]
	pub fltg_intrst_ref_rate: Option<CompareBenchmarkCurveName3>,
	#[serde(rename = "FltgIntrstRateTermUnit")]
	pub fltg_intrst_rate_term_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgIntrstRateTermVal")]
	pub fltg_intrst_rate_term_val: Option<CompareNumber5>,
	#[serde(rename = "FltgIntrstRatePmtFrqcyUnit")]
	pub fltg_intrst_rate_pmt_frqcy_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgIntrstRatePmtFrqcyVal")]
	pub fltg_intrst_rate_pmt_frqcy_val: Option<CompareNumber5>,
	#[serde(rename = "FltgIntrstRateRstFrqcyUnit")]
	pub fltg_intrst_rate_rst_frqcy_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgIntrstRateRstFrqcyVal")]
	pub fltg_intrst_rate_rst_frqcy_val: Option<CompareNumber6>,
	#[serde(rename = "BsisPtSprd")]
	pub bsis_pt_sprd: Option<CompareDecimalNumber3>,
	#[serde(rename = "MrgnLnAttr")]
	pub mrgn_ln_attr: Option<Vec<CompareInterestRate1>>,
	#[serde(rename = "PrncplAmtValDtAmt")]
	pub prncpl_amt_val_dt_amt: Option<CompareActiveOrHistoricCurrencyAndAmount3>,
	#[serde(rename = "PrncplAmtMtrtyDtAmt")]
	pub prncpl_amt_mtrty_dt_amt: Option<CompareActiveOrHistoricCurrencyAndAmount3>,
	#[serde(rename = "AsstTp")]
	pub asst_tp: Option<SecurityCommodity7Choice>,
	#[serde(rename = "LnVal")]
	pub ln_val: Option<CompareActiveOrHistoricCurrencyAndAmount3>,
	#[serde(rename = "FxdRbtRefRate")]
	pub fxd_rbt_ref_rate: Option<ComparePercentageRate3>,
	#[serde(rename = "FltgRbtRefRate")]
	pub fltg_rbt_ref_rate: Option<CompareBenchmarkCurveName3>,
	#[serde(rename = "FltgRbtRateTermUnit")]
	pub fltg_rbt_rate_term_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgRbtRateTermVal")]
	pub fltg_rbt_rate_term_val: Option<CompareNumber6>,
	#[serde(rename = "FltgRbtRatePmtFrqcyUnit")]
	pub fltg_rbt_rate_pmt_frqcy_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgRbtRatePmtFrqcyVal")]
	pub fltg_rbt_rate_pmt_frqcy_val: Option<CompareNumber6>,
	#[serde(rename = "FltgRbtRateRstFrqcyUnit")]
	pub fltg_rbt_rate_rst_frqcy_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgRbtRateRstFrqcyVal")]
	pub fltg_rbt_rate_rst_frqcy_val: Option<CompareNumber6>,
	#[serde(rename = "RbtRateBsisPtSprd")]
	pub rbt_rate_bsis_pt_sprd: Option<CompareDecimalNumber3>,
	#[serde(rename = "FltgRateAdjstmnt")]
	pub fltg_rate_adjstmnt: Option<Vec<ComparePercentageRate3>>,
	#[serde(rename = "FltgRateAdjstmntDt")]
	pub fltg_rate_adjstmnt_dt: Option<Vec<CompareDate3>>,
	#[serde(rename = "LndgFee")]
	pub lndg_fee: Option<ComparePercentageRate3>,
	#[serde(rename = "OutsdngMrgnLnAmt")]
	pub outsdng_mrgn_ln_amt: Option<CompareActiveOrHistoricCurrencyAndAmount3>,
	#[serde(rename = "ShrtMktValAmt")]
	pub shrt_mkt_val_amt: Option<CompareActiveOrHistoricCurrencyAndAmount3>,
	#[serde(rename = "LvlTp")]
	pub lvl_tp: Option<CompareReportingLevelType3>,
	#[serde(rename = "UnitOfMeasr")]
	pub unit_of_measr: Option<CompareUnitOfMeasure3>,
}


// LongFraction19DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LongFraction19DecimalNumber {
	#[serde(rename = "LongFraction19DecimalNumber")]
	pub long_fraction19_decimal_number: f64,
}


// MICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MICIdentifier {
	#[serde(rename = "MICIdentifier")]
	pub mic_identifier: String,
}


// MasterAgreement7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MasterAgreement7 {
	#[serde(rename = "Tp")]
	pub tp: AgreementType2Choice,
	#[serde(rename = "Vrsn")]
	pub vrsn: Option<String>,
	#[serde(rename = "OthrMstrAgrmtDtls")]
	pub othr_mstr_agrmt_dtls: Option<String>,
}


// MatchingCriteria10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MatchingCriteria10 {
	#[serde(rename = "CtrPtyMtchgCrit")]
	pub ctr_pty_mtchg_crit: Option<CounterpartyMatchingCriteria4>,
	#[serde(rename = "LnMtchgCrit")]
	pub ln_mtchg_crit: Option<LoanMatchingCriteria9>,
	#[serde(rename = "CollMtchgCrit")]
	pub coll_mtchg_crit: Option<CollateralMatchingCriteria6>,
}


// Max105Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max105Text {
	#[serde(rename = "Max105Text")]
	pub max105_text: String,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max15NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max15NumericText {
	#[serde(rename = "Max15NumericText")]
	pub max15_numeric_text: String,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Max3Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max3Number {
	#[serde(rename = "Max3Number")]
	pub max3_number: f64,
}


// Max500Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max500Text {
	#[serde(rename = "Max500Text")]
	pub max500_text: String,
}


// Max50Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max50Text {
	#[serde(rename = "Max50Text")]
	pub max50_text: String,
}


// Max52Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max52Text {
	#[serde(rename = "Max52Text")]
	pub max52_text: String,
}


// Max5Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max5Number {
	#[serde(rename = "Max5Number")]
	pub max5_number: f64,
}


// Max72Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max72Text {
	#[serde(rename = "Max72Text")]
	pub max72_text: String,
}


// MetalCommodityNonPrecious1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MetalCommodityNonPrecious1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: String,
}


// MetalCommodityPrecious1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MetalCommodityPrecious1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: String,
}


// ModificationLevel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ModificationLevel1Code {
	#[serde(rename = "ModificationLevel1Code")]
	pub modification_level1_code: String,
}


// NaturalPersonIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NaturalPersonIdentification2 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Dmcl")]
	pub dmcl: Option<String>,
}


// NoReasonCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NoReasonCode {
	#[serde(rename = "NoReasonCode")]
	pub no_reason_code: String,
}


// NotAvailable1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotAvailable1Code {
	#[serde(rename = "NotAvailable1Code")]
	pub not_available1_code: String,
}


// NumberOfReportsPerStatus4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NumberOfReportsPerStatus4 {
	#[serde(rename = "DtldNbOfRpts")]
	pub dtld_nb_of_rpts: String,
	#[serde(rename = "DtldSts")]
	pub dtld_sts: String,
}


// OrganisationIdentification15Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification15Choice {
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<OrganisationIdentification38>,
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
}


// OrganisationIdentification38 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification38 {
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Dmcl")]
	pub dmcl: Option<String>,
}


// OtherC10CommodityDeliverable2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherC10CommodityDeliverable2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// OtherC10CommodityNonDeliverable2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherC10CommodityNonDeliverable2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PairedReconciled3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PairedReconciled3Code {
	#[serde(rename = "PairedReconciled3Code")]
	pub paired_reconciled3_code: String,
}


// PaperCommodityContainerBoard1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityContainerBoard1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PaperCommodityNewsprint1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityNewsprint1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PaperCommodityPulp1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityPulp1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PaperCommodityRecoveredPaper1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityRecoveredPaper1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PaperCommodityRecoveredPaper2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityRecoveredPaper2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PartyIdentification236Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification236Choice {
	#[serde(rename = "Lgl")]
	pub lgl: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "Ntrl")]
	pub ntrl: Option<NaturalPersonIdentification2>,
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// PlusOrMinusIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "PlusOrMinusIndicator")]
	pub plus_or_minus_indicator: bool,
}


// PolypropyleneCommodityOther1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PolypropyleneCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// PolypropyleneCommodityPlastic1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PolypropyleneCommodityPlastic1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PriceStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceStatus1Code {
	#[serde(rename = "PriceStatus1Code")]
	pub price_status1_code: String,
}


// RateBasis1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RateBasis1Code {
	#[serde(rename = "RateBasis1Code")]
	pub rate_basis1_code: String,
}


// ReconciliationMatchedStatus9Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReconciliationMatchedStatus9Choice {
	#[serde(rename = "Mtchd")]
	pub mtchd: Option<String>,
	#[serde(rename = "NotMtchd")]
	pub not_mtchd: Option<ReconciliationResult10>,
}


// ReconciliationReport8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReconciliationReport8 {
	#[serde(rename = "TechRcrdId")]
	pub tech_rcrd_id: Option<String>,
	#[serde(rename = "TxId")]
	pub tx_id: TradeTransactionIdentification19,
	#[serde(rename = "Modfd")]
	pub modfd: bool,
	#[serde(rename = "RcncltnSts")]
	pub rcncltn_sts: ReconciliationStatus8Choice,
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


// ReconciliationStatus8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReconciliationStatus8Choice {
	#[serde(rename = "NoRcncltnReqrd")]
	pub no_rcncltn_reqrd: Option<String>,
	#[serde(rename = "RptgData")]
	pub rptg_data: Option<ReconciliationMatchedStatus9Choice>,
}


// RepoTerminationOption2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RepoTerminationOption2Code {
	#[serde(rename = "RepoTerminationOption2Code")]
	pub repo_termination_option2_code: String,
}


// ReportPeriodActivity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportPeriodActivity1Code {
	#[serde(rename = "ReportPeriodActivity1Code")]
	pub report_period_activity1_code: String,
}


// SecuritiesFinancingReportingReconciliationStatusAdviceV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesFinancingReportingReconciliationStatusAdviceV02 {
	#[serde(rename = "RcncltnData")]
	pub rcncltn_data: TradeData34Choice,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// SecuritiesLendingType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesLendingType3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// SecuritiesTransactionPrice19Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice19Choice {
	#[serde(rename = "MntryVal")]
	pub mntry_val: Option<AmountAndDirection107>,
	#[serde(rename = "Unit")]
	pub unit: Option<f64>,
	#[serde(rename = "Pctg")]
	pub pctg: Option<f64>,
	#[serde(rename = "Yld")]
	pub yld: Option<f64>,
	#[serde(rename = "Dcml")]
	pub dcml: Option<f64>,
	#[serde(rename = "PdgPric")]
	pub pdg_pric: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<SecuritiesTransactionPrice5>,
}


// SecuritiesTransactionPrice5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice5 {
	#[serde(rename = "Val")]
	pub val: Option<f64>,
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
}


// Security48 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Security48 {
	#[serde(rename = "Id")]
	pub id: Option<CompareISINIdentifier4>,
	#[serde(rename = "ClssfctnTp")]
	pub clssfctn_tp: Option<CompareCFIIdentifier3>,
	#[serde(rename = "Qty")]
	pub qty: Option<CompareDecimalNumber3>,
	#[serde(rename = "NmnlVal")]
	pub nmnl_val: Option<CompareAmountAndDirection2>,
	#[serde(rename = "Qlty")]
	pub qlty: Option<CompareCollateralQualityType3>,
	#[serde(rename = "Mtrty")]
	pub mtrty: Option<CompareDate3>,
	#[serde(rename = "IssrId")]
	pub issr_id: Option<CompareOrganisationIdentification6>,
	#[serde(rename = "IssrCtry")]
	pub issr_ctry: Option<CompareCountryCode3>,
	#[serde(rename = "Tp")]
	pub tp: Option<Vec<CompareSecuritiesLendingType3>>,
	#[serde(rename = "UnitPric")]
	pub unit_pric: Option<CompareUnitPrice6>,
	#[serde(rename = "ExclsvArrgmnt")]
	pub exclsv_arrgmnt: Option<CompareTrueFalseIndicator3>,
	#[serde(rename = "MktVal")]
	pub mkt_val: Option<CompareAmountAndDirection2>,
	#[serde(rename = "AvlblForCollReuse")]
	pub avlbl_for_coll_reuse: Option<CompareTrueFalseIndicator3>,
	#[serde(rename = "HrcutOrMrgn")]
	pub hrcut_or_mrgn: Option<ComparePercentageRate3>,
}


// SecurityCommodity7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityCommodity7Choice {
	#[serde(rename = "Scty")]
	pub scty: Option<Vec<Security48>>,
	#[serde(rename = "Cmmdty")]
	pub cmmdty: Option<Vec<Commodity42>>,
}


// SecurityCommodityCash4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityCommodityCash4 {
	#[serde(rename = "Scty")]
	pub scty: Option<Vec<Security48>>,
	#[serde(rename = "Cmmdty")]
	pub cmmdty: Option<Vec<Commodity42>>,
	#[serde(rename = "Csh")]
	pub csh: Option<Vec<CashCompare3>>,
}


// SecurityIdentification26Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification26Choice {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "NotAvlbl")]
	pub not_avlbl: Option<String>,
}


// SpecialCollateral1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SpecialCollateral1Code {
	#[serde(rename = "SpecialCollateral1Code")]
	pub special_collateral1_code: String,
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Option<String>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}


// TradeData28 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeData28 {
	#[serde(rename = "PairgRcncltnSts")]
	pub pairg_rcncltn_sts: Option<Vec<NumberOfReportsPerStatus4>>,
	#[serde(rename = "RcncltnRpt")]
	pub rcncltn_rpt: Vec<ReconciliationReport8>,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// TradeData34Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeData34Choice {
	#[serde(rename = "DataSetActn")]
	pub data_set_actn: Option<String>,
	#[serde(rename = "Rpt")]
	pub rpt: Option<Vec<TradeData28>>,
}


// TradeTransactionIdentification19 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeTransactionIdentification19 {
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: OrganisationIdentification15Choice,
	#[serde(rename = "OthrCtrPty")]
	pub othr_ctr_pty: PartyIdentification236Choice,
	#[serde(rename = "NttyRspnsblForRpt")]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "UnqTradIdr")]
	pub unq_trad_idr: Option<String>,
	#[serde(rename = "MstrAgrmt")]
	pub mstr_agrmt: Option<MasterAgreement7>,
	#[serde(rename = "AgtLndr")]
	pub agt_lndr: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "TrptyAgt")]
	pub trpty_agt: Option<OrganisationIdentification15Choice>,
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}


// UnitOfMeasure11Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitOfMeasure11Code {
	#[serde(rename = "UnitOfMeasure11Code")]
	pub unit_of_measure11_code: String,
}
