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
use serde_valid::Validate;


// ActiveCurrencyCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[validate(pattern = "[A-Z]{3,3}")]
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// ActiveOrHistoricCurrencyAnd19DecimalAmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd19DecimalAmountSimpleType {
	#[serde(rename = "ActiveOrHistoricCurrencyAnd19DecimalAmount_SimpleType")]
	pub active_or_historic_currency_and19_decimal_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAnd19DecimalAmount ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd19DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveOrHistoricCurrencyCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyCode {
	#[validate(pattern = "[A-Z]{3,3}")]
	#[serde(rename = "ActiveOrHistoricCurrencyCode")]
	pub active_or_historic_currency_code: String,
}


// AgreementType2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AgreementType2Choice {
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// AgriculturalCommodityDairy2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AgriculturalCommodityDairy2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// AgriculturalCommodityForestry2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AgriculturalCommodityForestry2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// AgriculturalCommodityGrain3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AgriculturalCommodityGrain3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// AgriculturalCommodityLiveStock2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AgriculturalCommodityLiveStock2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// AgriculturalCommodityOilSeed2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AgriculturalCommodityOilSeed2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// AgriculturalCommodityOliveOil3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AgriculturalCommodityOliveOil3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// AgriculturalCommodityOther2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AgriculturalCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// AgriculturalCommodityPotato2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AgriculturalCommodityPotato2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// AgriculturalCommoditySeafood2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AgriculturalCommoditySeafood2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// AgriculturalCommoditySoft2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AgriculturalCommoditySoft2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// AmountAndDirection106 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AmountAndDirection106 {
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAnd19DecimalAmount,
	#[serde(rename = "Sgn")]
	pub sgn: Option<bool>,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[validate(pattern = "[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}")]
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// AssetClassCommodity6Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodity6Choice {
	#[validate]
	#[serde(rename = "Agrcltrl")]
	pub agrcltrl: Option<AssetClassCommodityAgricultural6Choice>,
	#[validate]
	#[serde(rename = "Nrgy")]
	pub nrgy: Option<AssetClassCommodityEnergy3Choice>,
	#[validate]
	#[serde(rename = "Envttl")]
	pub envttl: Option<AssetClassCommodityEnvironmental3Choice>,
	#[validate]
	#[serde(rename = "Frtlzr")]
	pub frtlzr: Option<AssetClassCommodityFertilizer4Choice>,
	#[validate]
	#[serde(rename = "Frght")]
	pub frght: Option<AssetClassCommodityFreight4Choice>,
	#[validate]
	#[serde(rename = "Indx")]
	pub indx: Option<AssetClassCommodityIndex1>,
	#[validate]
	#[serde(rename = "IndstrlPdct")]
	pub indstrl_pdct: Option<AssetClassCommodityIndustrialProduct2Choice>,
	#[validate]
	#[serde(rename = "Infltn")]
	pub infltn: Option<AssetClassCommodityInflation1>,
	#[validate]
	#[serde(rename = "Metl")]
	pub metl: Option<AssetClassCommodityMetal2Choice>,
	#[validate]
	#[serde(rename = "MultiCmmdtyExtc")]
	pub multi_cmmdty_extc: Option<AssetClassCommodityMultiCommodityExotic1>,
	#[validate]
	#[serde(rename = "OffclEcnmcSttstcs")]
	pub offcl_ecnmc_sttstcs: Option<AssetClassCommodityOfficialEconomicStatistics1>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<AssetClassCommodityOther1>,
	#[validate]
	#[serde(rename = "OthrC10")]
	pub othr_c10: Option<AssetClassCommodityC10Other1>,
	#[validate]
	#[serde(rename = "Ppr")]
	pub ppr: Option<AssetClassCommodityPaper4Choice>,
	#[validate]
	#[serde(rename = "Plprpln")]
	pub plprpln: Option<AssetClassCommodityPolypropylene4Choice>,
}


// AssetClassCommodityAgricultural6Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityAgricultural6Choice {
	#[validate]
	#[serde(rename = "GrnOilSeed")]
	pub grn_oil_seed: Option<AgriculturalCommodityOilSeed2>,
	#[validate]
	#[serde(rename = "Soft")]
	pub soft: Option<AgriculturalCommoditySoft2>,
	#[validate]
	#[serde(rename = "Ptt")]
	pub ptt: Option<AgriculturalCommodityPotato2>,
	#[validate]
	#[serde(rename = "OlvOil")]
	pub olv_oil: Option<AgriculturalCommodityOliveOil3>,
	#[validate]
	#[serde(rename = "Dairy")]
	pub dairy: Option<AgriculturalCommodityDairy2>,
	#[validate]
	#[serde(rename = "Frstry")]
	pub frstry: Option<AgriculturalCommodityForestry2>,
	#[validate]
	#[serde(rename = "Sfd")]
	pub sfd: Option<AgriculturalCommoditySeafood2>,
	#[validate]
	#[serde(rename = "LiveStock")]
	pub live_stock: Option<AgriculturalCommodityLiveStock2>,
	#[validate]
	#[serde(rename = "Grn")]
	pub grn: Option<AgriculturalCommodityGrain3>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<AgriculturalCommodityOther2>,
}


// AssetClassCommodityC10Other1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityC10Other1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
}


// AssetClassCommodityEnergy3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityEnergy3Choice {
	#[validate]
	#[serde(rename = "Elctrcty")]
	pub elctrcty: Option<EnergyCommodityElectricity2>,
	#[validate]
	#[serde(rename = "NtrlGas")]
	pub ntrl_gas: Option<EnergyCommodityNaturalGas3>,
	#[validate]
	#[serde(rename = "Oil")]
	pub oil: Option<EnergyCommodityOil3>,
	#[validate]
	#[serde(rename = "Coal")]
	pub coal: Option<EnergyCommodityCoal2>,
	#[validate]
	#[serde(rename = "IntrNrgy")]
	pub intr_nrgy: Option<EnergyCommodityInterEnergy2>,
	#[validate]
	#[serde(rename = "RnwblNrgy")]
	pub rnwbl_nrgy: Option<EnergyCommodityRenewableEnergy2>,
	#[validate]
	#[serde(rename = "LghtEnd")]
	pub lght_end: Option<EnergyCommodityLightEnd2>,
	#[validate]
	#[serde(rename = "Dstllts")]
	pub dstllts: Option<EnergyCommodityDistillates2>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<EnergyCommodityOther2>,
}


// AssetClassCommodityEnvironmental3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityEnvironmental3Choice {
	#[validate]
	#[serde(rename = "Emssns")]
	pub emssns: Option<EnvironmentalCommodityEmission3>,
	#[validate]
	#[serde(rename = "Wthr")]
	pub wthr: Option<EnvironmentalCommodityWeather2>,
	#[validate]
	#[serde(rename = "CrbnRltd")]
	pub crbn_rltd: Option<EnvironmentalCommodityCarbonRelated2>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<EnvironmentCommodityOther2>,
}


// AssetClassCommodityFertilizer4Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityFertilizer4Choice {
	#[validate]
	#[serde(rename = "Ammn")]
	pub ammn: Option<FertilizerCommodityAmmonia2>,
	#[validate]
	#[serde(rename = "DmmnmPhspht")]
	pub dmmnm_phspht: Option<FertilizerCommodityDiammoniumPhosphate2>,
	#[validate]
	#[serde(rename = "Ptsh")]
	pub ptsh: Option<FertilizerCommodityPotash2>,
	#[validate]
	#[serde(rename = "Slphr")]
	pub slphr: Option<FertilizerCommoditySulphur2>,
	#[validate]
	#[serde(rename = "Urea")]
	pub urea: Option<FertilizerCommodityUrea2>,
	#[validate]
	#[serde(rename = "UreaAndAmmnmNtrt")]
	pub urea_and_ammnm_ntrt: Option<FertilizerCommodityUreaAndAmmoniumNitrate2>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<FertilizerCommodityOther2>,
}


// AssetClassCommodityFreight4Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityFreight4Choice {
	#[validate]
	#[serde(rename = "Dry")]
	pub dry: Option<FreightCommodityDry3>,
	#[validate]
	#[serde(rename = "Wet")]
	pub wet: Option<FreightCommodityWet3>,
	#[validate]
	#[serde(rename = "CntnrShip")]
	pub cntnr_ship: Option<FreightCommodityContainerShip2>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<FreightCommodityOther2>,
}


// AssetClassCommodityIndex1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityIndex1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
}


// AssetClassCommodityIndustrialProduct2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityIndustrialProduct2Choice {
	#[validate]
	#[serde(rename = "Cnstrctn")]
	pub cnstrctn: Option<IndustrialProductCommodityConstruction2>,
	#[validate]
	#[serde(rename = "Manfctg")]
	pub manfctg: Option<IndustrialProductCommodityManufacturing2>,
}


// AssetClassCommodityInflation1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityInflation1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
}


// AssetClassCommodityMetal2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityMetal2Choice {
	#[validate]
	#[serde(rename = "NonPrcs")]
	pub non_prcs: Option<MetalCommodityNonPrecious2>,
	#[validate]
	#[serde(rename = "Prcs")]
	pub prcs: Option<MetalCommodityPrecious2>,
}


// AssetClassCommodityMultiCommodityExotic1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityMultiCommodityExotic1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
}


// AssetClassCommodityOfficialEconomicStatistics1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityOfficialEconomicStatistics1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
}


// AssetClassCommodityOther1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
}


// AssetClassCommodityPaper4Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityPaper4Choice {
	#[validate]
	#[serde(rename = "CntnrBrd")]
	pub cntnr_brd: Option<PaperCommodityContainerBoard2>,
	#[validate]
	#[serde(rename = "Nwsprnt")]
	pub nwsprnt: Option<PaperCommodityNewsprint2>,
	#[validate]
	#[serde(rename = "Pulp")]
	pub pulp: Option<PaperCommodityPulp2>,
	#[validate]
	#[serde(rename = "RcvrdPpr")]
	pub rcvrd_ppr: Option<PaperCommodityOther1>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<PaperCommodityOther1>,
}


// AssetClassCommodityPolypropylene4Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityPolypropylene4Choice {
	#[validate]
	#[serde(rename = "Plstc")]
	pub plstc: Option<PolypropyleneCommodityPlastic2>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<PolypropyleneCommodityOther2>,
}


// AssetClassDetailedSubProductType10Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType10Code {
	#[validate(enumerate = ["ALUM", "ALUA", "CBLT", "COPR", "IRON", "MOLY", "NASC", "NICK", "STEL", "TINN", "ZINC", "OTHR", "LEAD"])]
	#[serde(rename = "AssetClassDetailedSubProductType10Code")]
	pub asset_class_detailed_sub_product_type10_code: String,
}


// AssetClassDetailedSubProductType11Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType11Code {
	#[validate(enumerate = ["GOLD", "OTHR", "PLDM", "PTNM", "SLVR"])]
	#[serde(rename = "AssetClassDetailedSubProductType11Code")]
	pub asset_class_detailed_sub_product_type11_code: String,
}


// AssetClassDetailedSubProductType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType1Code {
	#[validate(enumerate = ["FWHT", "SOYB", "RPSD", "OTHR", "CORN", "RICE"])]
	#[serde(rename = "AssetClassDetailedSubProductType1Code")]
	pub asset_class_detailed_sub_product_type1_code: String,
}


// AssetClassDetailedSubProductType29Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType29Code {
	#[validate(enumerate = ["LAMP", "OTHR"])]
	#[serde(rename = "AssetClassDetailedSubProductType29Code")]
	pub asset_class_detailed_sub_product_type29_code: String,
}


// AssetClassDetailedSubProductType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType2Code {
	#[validate(enumerate = ["ROBU", "CCOA", "BRWN", "WHSG", "OTHR"])]
	#[serde(rename = "AssetClassDetailedSubProductType2Code")]
	pub asset_class_detailed_sub_product_type2_code: String,
}


// AssetClassDetailedSubProductType30Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType30Code {
	#[validate(enumerate = ["MWHT", "OTHR"])]
	#[serde(rename = "AssetClassDetailedSubProductType30Code")]
	pub asset_class_detailed_sub_product_type30_code: String,
}


// AssetClassDetailedSubProductType31Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType31Code {
	#[validate(enumerate = ["GASP", "LNGG", "NCGG", "TTFG", "NBPG", "OTHR"])]
	#[serde(rename = "AssetClassDetailedSubProductType31Code")]
	pub asset_class_detailed_sub_product_type31_code: String,
}


// AssetClassDetailedSubProductType32Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType32Code {
	#[validate(enumerate = ["BAKK", "BDSL", "BRNT", "BRNX", "CNDA", "COND", "DSEL", "DUBA", "ESPO", "ETHA", "FUEL", "FOIL", "GOIL", "GSLN", "HEAT", "JTFL", "KERO", "LLSO", "MARS", "NAPH", "NGLO", "TAPI", "WTIO", "URAL", "OTHR"])]
	#[serde(rename = "AssetClassDetailedSubProductType32Code")]
	pub asset_class_detailed_sub_product_type32_code: String,
}


// AssetClassDetailedSubProductType33Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType33Code {
	#[validate(enumerate = ["DBCR", "OTHR"])]
	#[serde(rename = "AssetClassDetailedSubProductType33Code")]
	pub asset_class_detailed_sub_product_type33_code: String,
}


// AssetClassDetailedSubProductType34Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType34Code {
	#[validate(enumerate = ["TNKR", "OTHR"])]
	#[serde(rename = "AssetClassDetailedSubProductType34Code")]
	pub asset_class_detailed_sub_product_type34_code: String,
}


// AssetClassDetailedSubProductType5Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType5Code {
	#[validate(enumerate = ["BSLD", "FITR", "PKLD", "OFFP", "OTHR"])]
	#[serde(rename = "AssetClassDetailedSubProductType5Code")]
	pub asset_class_detailed_sub_product_type5_code: String,
}


// AssetClassDetailedSubProductType8Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType8Code {
	#[validate(enumerate = ["CERE", "ERUE", "EUAE", "EUAA", "OTHR"])]
	#[serde(rename = "AssetClassDetailedSubProductType8Code")]
	pub asset_class_detailed_sub_product_type8_code: String,
}


// AssetClassProductType11Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassProductType11Code {
	#[validate(enumerate = ["OTHC"])]
	#[serde(rename = "AssetClassProductType11Code")]
	pub asset_class_product_type11_code: String,
}


// AssetClassProductType12Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassProductType12Code {
	#[validate(enumerate = ["INFL"])]
	#[serde(rename = "AssetClassProductType12Code")]
	pub asset_class_product_type12_code: String,
}


// AssetClassProductType13Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassProductType13Code {
	#[validate(enumerate = ["MCEX"])]
	#[serde(rename = "AssetClassProductType13Code")]
	pub asset_class_product_type13_code: String,
}


// AssetClassProductType14Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassProductType14Code {
	#[validate(enumerate = ["OEST"])]
	#[serde(rename = "AssetClassProductType14Code")]
	pub asset_class_product_type14_code: String,
}


// AssetClassProductType15Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassProductType15Code {
	#[validate(enumerate = ["OTHR"])]
	#[serde(rename = "AssetClassProductType15Code")]
	pub asset_class_product_type15_code: String,
}


// AssetClassProductType16Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassProductType16Code {
	#[validate(enumerate = ["INDX"])]
	#[serde(rename = "AssetClassProductType16Code")]
	pub asset_class_product_type16_code: String,
}


// AssetClassProductType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassProductType1Code {
	#[validate(enumerate = ["AGRI"])]
	#[serde(rename = "AssetClassProductType1Code")]
	pub asset_class_product_type1_code: String,
}


// AssetClassProductType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassProductType2Code {
	#[validate(enumerate = ["NRGY"])]
	#[serde(rename = "AssetClassProductType2Code")]
	pub asset_class_product_type2_code: String,
}


// AssetClassProductType3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassProductType3Code {
	#[validate(enumerate = ["ENVR"])]
	#[serde(rename = "AssetClassProductType3Code")]
	pub asset_class_product_type3_code: String,
}


// AssetClassProductType4Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassProductType4Code {
	#[validate(enumerate = ["FRGT"])]
	#[serde(rename = "AssetClassProductType4Code")]
	pub asset_class_product_type4_code: String,
}


// AssetClassProductType5Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassProductType5Code {
	#[validate(enumerate = ["FRTL"])]
	#[serde(rename = "AssetClassProductType5Code")]
	pub asset_class_product_type5_code: String,
}


// AssetClassProductType6Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassProductType6Code {
	#[validate(enumerate = ["INDP"])]
	#[serde(rename = "AssetClassProductType6Code")]
	pub asset_class_product_type6_code: String,
}


// AssetClassProductType7Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassProductType7Code {
	#[validate(enumerate = ["METL"])]
	#[serde(rename = "AssetClassProductType7Code")]
	pub asset_class_product_type7_code: String,
}


// AssetClassProductType8Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassProductType8Code {
	#[validate(enumerate = ["PAPR"])]
	#[serde(rename = "AssetClassProductType8Code")]
	pub asset_class_product_type8_code: String,
}


// AssetClassProductType9Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassProductType9Code {
	#[validate(enumerate = ["POLY"])]
	#[serde(rename = "AssetClassProductType9Code")]
	pub asset_class_product_type9_code: String,
}


// AssetClassSubProductType10Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType10Code {
	#[validate(enumerate = ["EMIS"])]
	#[serde(rename = "AssetClassSubProductType10Code")]
	pub asset_class_sub_product_type10_code: String,
}


// AssetClassSubProductType15Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType15Code {
	#[validate(enumerate = ["NPRM"])]
	#[serde(rename = "AssetClassSubProductType15Code")]
	pub asset_class_sub_product_type15_code: String,
}


// AssetClassSubProductType16Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType16Code {
	#[validate(enumerate = ["PRME"])]
	#[serde(rename = "AssetClassSubProductType16Code")]
	pub asset_class_sub_product_type16_code: String,
}


// AssetClassSubProductType18Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType18Code {
	#[validate(enumerate = ["PLST"])]
	#[serde(rename = "AssetClassSubProductType18Code")]
	pub asset_class_sub_product_type18_code: String,
}


// AssetClassSubProductType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType1Code {
	#[validate(enumerate = ["GROS"])]
	#[serde(rename = "AssetClassSubProductType1Code")]
	pub asset_class_sub_product_type1_code: String,
}


// AssetClassSubProductType20Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType20Code {
	#[validate(enumerate = ["DIRY"])]
	#[serde(rename = "AssetClassSubProductType20Code")]
	pub asset_class_sub_product_type20_code: String,
}


// AssetClassSubProductType21Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType21Code {
	#[validate(enumerate = ["FRST"])]
	#[serde(rename = "AssetClassSubProductType21Code")]
	pub asset_class_sub_product_type21_code: String,
}


// AssetClassSubProductType22Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType22Code {
	#[validate(enumerate = ["LSTK"])]
	#[serde(rename = "AssetClassSubProductType22Code")]
	pub asset_class_sub_product_type22_code: String,
}


// AssetClassSubProductType23Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType23Code {
	#[validate(enumerate = ["SEAF"])]
	#[serde(rename = "AssetClassSubProductType23Code")]
	pub asset_class_sub_product_type23_code: String,
}


// AssetClassSubProductType24Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType24Code {
	#[validate(enumerate = ["COAL"])]
	#[serde(rename = "AssetClassSubProductType24Code")]
	pub asset_class_sub_product_type24_code: String,
}


// AssetClassSubProductType25Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType25Code {
	#[validate(enumerate = ["DIST"])]
	#[serde(rename = "AssetClassSubProductType25Code")]
	pub asset_class_sub_product_type25_code: String,
}


// AssetClassSubProductType26Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType26Code {
	#[validate(enumerate = ["INRG"])]
	#[serde(rename = "AssetClassSubProductType26Code")]
	pub asset_class_sub_product_type26_code: String,
}


// AssetClassSubProductType27Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType27Code {
	#[validate(enumerate = ["LGHT"])]
	#[serde(rename = "AssetClassSubProductType27Code")]
	pub asset_class_sub_product_type27_code: String,
}


// AssetClassSubProductType28Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType28Code {
	#[validate(enumerate = ["RNNG"])]
	#[serde(rename = "AssetClassSubProductType28Code")]
	pub asset_class_sub_product_type28_code: String,
}


// AssetClassSubProductType29Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType29Code {
	#[validate(enumerate = ["CRBR"])]
	#[serde(rename = "AssetClassSubProductType29Code")]
	pub asset_class_sub_product_type29_code: String,
}


// AssetClassSubProductType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType2Code {
	#[validate(enumerate = ["SOFT"])]
	#[serde(rename = "AssetClassSubProductType2Code")]
	pub asset_class_sub_product_type2_code: String,
}


// AssetClassSubProductType30Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType30Code {
	#[validate(enumerate = ["WTHR"])]
	#[serde(rename = "AssetClassSubProductType30Code")]
	pub asset_class_sub_product_type30_code: String,
}


// AssetClassSubProductType31Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType31Code {
	#[validate(enumerate = ["DRYF"])]
	#[serde(rename = "AssetClassSubProductType31Code")]
	pub asset_class_sub_product_type31_code: String,
}


// AssetClassSubProductType32Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType32Code {
	#[validate(enumerate = ["WETF"])]
	#[serde(rename = "AssetClassSubProductType32Code")]
	pub asset_class_sub_product_type32_code: String,
}


// AssetClassSubProductType33Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType33Code {
	#[validate(enumerate = ["CSTR"])]
	#[serde(rename = "AssetClassSubProductType33Code")]
	pub asset_class_sub_product_type33_code: String,
}


// AssetClassSubProductType34Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType34Code {
	#[validate(enumerate = ["MFTG"])]
	#[serde(rename = "AssetClassSubProductType34Code")]
	pub asset_class_sub_product_type34_code: String,
}


// AssetClassSubProductType35Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType35Code {
	#[validate(enumerate = ["CBRD"])]
	#[serde(rename = "AssetClassSubProductType35Code")]
	pub asset_class_sub_product_type35_code: String,
}


// AssetClassSubProductType36Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType36Code {
	#[validate(enumerate = ["NSPT"])]
	#[serde(rename = "AssetClassSubProductType36Code")]
	pub asset_class_sub_product_type36_code: String,
}


// AssetClassSubProductType37Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType37Code {
	#[validate(enumerate = ["PULP"])]
	#[serde(rename = "AssetClassSubProductType37Code")]
	pub asset_class_sub_product_type37_code: String,
}


// AssetClassSubProductType39Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType39Code {
	#[validate(enumerate = ["AMMO"])]
	#[serde(rename = "AssetClassSubProductType39Code")]
	pub asset_class_sub_product_type39_code: String,
}


// AssetClassSubProductType3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType3Code {
	#[validate(enumerate = ["OOLI"])]
	#[serde(rename = "AssetClassSubProductType3Code")]
	pub asset_class_sub_product_type3_code: String,
}


// AssetClassSubProductType40Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType40Code {
	#[validate(enumerate = ["DAPH"])]
	#[serde(rename = "AssetClassSubProductType40Code")]
	pub asset_class_sub_product_type40_code: String,
}


// AssetClassSubProductType41Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType41Code {
	#[validate(enumerate = ["PTSH"])]
	#[serde(rename = "AssetClassSubProductType41Code")]
	pub asset_class_sub_product_type41_code: String,
}


// AssetClassSubProductType42Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType42Code {
	#[validate(enumerate = ["SLPH"])]
	#[serde(rename = "AssetClassSubProductType42Code")]
	pub asset_class_sub_product_type42_code: String,
}


// AssetClassSubProductType43Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType43Code {
	#[validate(enumerate = ["UREA"])]
	#[serde(rename = "AssetClassSubProductType43Code")]
	pub asset_class_sub_product_type43_code: String,
}


// AssetClassSubProductType44Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType44Code {
	#[validate(enumerate = ["UAAN"])]
	#[serde(rename = "AssetClassSubProductType44Code")]
	pub asset_class_sub_product_type44_code: String,
}


// AssetClassSubProductType45Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType45Code {
	#[validate(enumerate = ["POTA"])]
	#[serde(rename = "AssetClassSubProductType45Code")]
	pub asset_class_sub_product_type45_code: String,
}


// AssetClassSubProductType46Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType46Code {
	#[validate(enumerate = ["CSHP"])]
	#[serde(rename = "AssetClassSubProductType46Code")]
	pub asset_class_sub_product_type46_code: String,
}


// AssetClassSubProductType49Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType49Code {
	#[validate(enumerate = ["OTHR"])]
	#[serde(rename = "AssetClassSubProductType49Code")]
	pub asset_class_sub_product_type49_code: String,
}


// AssetClassSubProductType5Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType5Code {
	#[validate(enumerate = ["GRIN"])]
	#[serde(rename = "AssetClassSubProductType5Code")]
	pub asset_class_sub_product_type5_code: String,
}


// AssetClassSubProductType6Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType6Code {
	#[validate(enumerate = ["ELEC"])]
	#[serde(rename = "AssetClassSubProductType6Code")]
	pub asset_class_sub_product_type6_code: String,
}


// AssetClassSubProductType7Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType7Code {
	#[validate(enumerate = ["NGAS"])]
	#[serde(rename = "AssetClassSubProductType7Code")]
	pub asset_class_sub_product_type7_code: String,
}


// AssetClassSubProductType8Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType8Code {
	#[validate(enumerate = ["OILP"])]
	#[serde(rename = "AssetClassSubProductType8Code")]
	pub asset_class_sub_product_type8_code: String,
}


// BaseOne18Rate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BaseOne18Rate {
	#[serde(rename = "BaseOne18Rate")]
	pub base_one18_rate: f64,
}


// BaseOneRate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BaseOneRate {
	#[serde(rename = "BaseOneRate")]
	pub base_one_rate: f64,
}


// BasketConstituents3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BasketConstituents3 {
	#[validate]
	#[serde(rename = "InstrmId")]
	pub instrm_id: InstrumentIdentification6Choice,
	#[serde(rename = "Qty")]
	pub qty: Option<f64>,
	#[validate]
	#[serde(rename = "UnitOfMeasr")]
	pub unit_of_measr: Option<UnitOfMeasure8Choice>,
}


// CFIOct2015Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[validate(pattern = "[A-Z]{6,6}")]
	#[serde(rename = "CFIOct2015Identifier")]
	pub cfi_oct2015_identifier: String,
}


// Cleared23Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Cleared23Choice {
	#[validate]
	#[serde(rename = "Clrd")]
	pub clrd: Option<ClearingPartyAndTime21Choice>,
	#[validate]
	#[serde(rename = "IntndToClear")]
	pub intnd_to_clear: Option<ClearingPartyAndTime22Choice>,
	#[validate]
	#[serde(rename = "NonClrd")]
	pub non_clrd: Option<ClearingExceptionOrExemption3Choice>,
}


// ClearingAccountType4Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ClearingAccountType4Code {
	#[validate(enumerate = ["CLIE", "HOUS"])]
	#[serde(rename = "ClearingAccountType4Code")]
	pub clearing_account_type4_code: String,
}


// ClearingExceptionOrExemption2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ClearingExceptionOrExemption2 {
	#[validate]
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: NonClearingReason2,
	#[validate]
	#[serde(rename = "OthrCtrPty")]
	pub othr_ctr_pty: Option<NonClearingReason2>,
}


// ClearingExceptionOrExemption3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ClearingExceptionOrExemption3Choice {
	#[serde(rename = "Rsn")]
	pub rsn: Option<String>,
	#[validate]
	#[serde(rename = "CtrPties")]
	pub ctr_pties: Option<ClearingExceptionOrExemption2>,
}


// ClearingExemptionException1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ClearingExemptionException1Code {
	#[validate(enumerate = ["COOP", "ENDU", "AFFL", "NOAL", "NORE", "OTHR", "SMBK"])]
	#[serde(rename = "ClearingExemptionException1Code")]
	pub clearing_exemption_exception1_code: String,
}


// ClearingObligationType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ClearingObligationType1Code {
	#[validate(enumerate = ["FLSE", "UKWN", "TRUE"])]
	#[serde(rename = "ClearingObligationType1Code")]
	pub clearing_obligation_type1_code: String,
}


// ClearingPartyAndTime21Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ClearingPartyAndTime21Choice {
	#[serde(rename = "Rsn")]
	pub rsn: Option<String>,
	#[validate]
	#[serde(rename = "Dtls")]
	pub dtls: Option<ClearingPartyAndTime22>,
}


// ClearingPartyAndTime22 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ClearingPartyAndTime22 {
	#[validate]
	#[serde(rename = "CCP")]
	pub ccp: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "ClrRctDtTm")]
	pub clr_rct_dt_tm: Option<String>,
	#[serde(rename = "ClrDtTm")]
	pub clr_dt_tm: Option<String>,
	#[validate]
	#[serde(rename = "ClrIdr")]
	pub clr_idr: Option<UniqueTransactionIdentifier2Choice>,
	#[validate]
	#[serde(rename = "OrgnlIdr")]
	pub orgnl_idr: Option<UniqueTransactionIdentifier2Choice>,
	#[validate]
	#[serde(rename = "OrgnlTradRpstryIdr")]
	pub orgnl_trad_rpstry_idr: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "ClrAcctOrgn")]
	pub clr_acct_orgn: Option<String>,
}


// ClearingPartyAndTime22Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ClearingPartyAndTime22Choice {
	#[serde(rename = "Rsn")]
	pub rsn: Option<String>,
	#[validate]
	#[serde(rename = "Dtls")]
	pub dtls: Option<ClearingPartyAndTime23>,
}


// ClearingPartyAndTime23 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ClearingPartyAndTime23 {
	#[validate]
	#[serde(rename = "CCP")]
	pub ccp: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "ClrRctDtTm")]
	pub clr_rct_dt_tm: Option<String>,
	#[serde(rename = "ClrDtTm")]
	pub clr_dt_tm: Option<String>,
	#[validate]
	#[serde(rename = "ClrIdr")]
	pub clr_idr: Option<UniqueTransactionIdentifier1Choice>,
	#[validate]
	#[serde(rename = "OrgnlIdr")]
	pub orgnl_idr: Option<UniqueTransactionIdentifier1Choice>,
	#[validate]
	#[serde(rename = "OrgnlTradRpstryIdr")]
	pub orgnl_trad_rpstry_idr: Option<OrganisationIdentification15Choice>,
}


// CollateralPortfolioCode5Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CollateralPortfolioCode5Choice {
	#[validate]
	#[serde(rename = "Prtfl")]
	pub prtfl: Option<PortfolioCode3Choice>,
	#[validate]
	#[serde(rename = "MrgnPrtflCd")]
	pub mrgn_prtfl_cd: Option<MarginPortfolio3>,
}


// CompareActiveOrHistoricCurrencyAndAmount4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareActiveOrHistoricCurrencyAndAmount4 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
}


// CompareActiveOrHistoricCurrencyCode1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareActiveOrHistoricCurrencyCode1 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareAmountAndDirection3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareAmountAndDirection3 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<AmountAndDirection106>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<AmountAndDirection106>,
}


// CompareAssetClass1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareAssetClass1 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareBenchmarkCode1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareBenchmarkCode1 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareCFIIdentifier3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareCFIIdentifier3 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareCommodityAssetClass4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareCommodityAssetClass4 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<AssetClassCommodity6Choice>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<AssetClassCommodity6Choice>,
}


// CompareDate3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareDate3 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareDatePeriod2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareDatePeriod2 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<DatePeriod4>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<DatePeriod4>,
}


// CompareDateTime3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareDateTime3 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareDayCount1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareDayCount1 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<InterestComputationMethodFormat7>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<InterestComputationMethodFormat7>,
}


// CompareDeliveryInterconnectionPoint1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareDeliveryInterconnectionPoint1 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<DeliveryInterconnectionPoint1Choice>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<DeliveryInterconnectionPoint1Choice>,
}


// CompareDeliveryType1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareDeliveryType1 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareDerivativeEvent1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareDerivativeEvent1 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<DerivativeEvent6>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<DerivativeEvent6>,
}


// CompareDurationType1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareDurationType1 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareEnergyDeliveryAttribute1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareEnergyDeliveryAttribute1 {
	#[validate]
	#[serde(rename = "NrgyDlvryIntrvl")]
	pub nrgy_dlvry_intrvl: Option<Vec<CompareTimePeriod2>>,
	#[validate]
	#[serde(rename = "NrgyDt")]
	pub nrgy_dt: Option<CompareDatePeriod2>,
	#[validate]
	#[serde(rename = "NrgyDrtn")]
	pub nrgy_drtn: Option<CompareDurationType1>,
	#[validate]
	#[serde(rename = "NrgyWkDay")]
	pub nrgy_wk_day: Option<Vec<CompareWeekDay1>>,
	#[validate]
	#[serde(rename = "NrgyDlvryCpcty")]
	pub nrgy_dlvry_cpcty: Option<CompareLongFraction19DecimalNumber1>,
	#[validate]
	#[serde(rename = "NrgyQtyUnit")]
	pub nrgy_qty_unit: Option<CompareEnergyQuantityUnit1>,
	#[validate]
	#[serde(rename = "NrgyPricTmIntrvlQty")]
	pub nrgy_pric_tm_intrvl_qty: Option<CompareAmountAndDirection3>,
}


// CompareEnergyLoadType1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareEnergyLoadType1 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareEnergyQuantityUnit1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareEnergyQuantityUnit1 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<EnergyQuantityUnit2Choice>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<EnergyQuantityUnit2Choice>,
}


// CompareExchangeRate1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareExchangeRate1 {
	#[serde(rename = "Val1")]
	pub val1: Option<f64>,
	#[serde(rename = "Val2")]
	pub val2: Option<f64>,
}


// CompareExchangeRateBasis1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareExchangeRateBasis1 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<ExchangeRateBasis1Choice>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<ExchangeRateBasis1Choice>,
}


// CompareFinancialInstrumentContractType1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareFinancialInstrumentContractType1 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareFrequencyUnit1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareFrequencyUnit1 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareISINIdentifier2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareISINIdentifier2 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareISINIdentifier4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareISINIdentifier4 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareLegDirection2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareLegDirection2 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<Direction4Choice>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<Direction4Choice>,
}


// CompareLongFraction19DecimalNumber1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareLongFraction19DecimalNumber1 {
	#[serde(rename = "Val1")]
	pub val1: Option<f64>,
	#[serde(rename = "Val2")]
	pub val2: Option<f64>,
}


// CompareMICIdentifier3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareMICIdentifier3 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareMasterAgreementType1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareMasterAgreementType1 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<AgreementType2Choice>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<AgreementType2Choice>,
}


// CompareMax350Text1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareMax350Text1 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareMax50Text1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareMax50Text1 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareNumber5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareNumber5 {
	#[serde(rename = "Val1")]
	pub val1: Option<f64>,
	#[serde(rename = "Val2")]
	pub val2: Option<f64>,
}


// CompareNumber7 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareNumber7 {
	#[serde(rename = "Val1")]
	pub val1: Option<f64>,
	#[serde(rename = "Val2")]
	pub val2: Option<f64>,
}


// CompareOptionStyle1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareOptionStyle1 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareOptionType1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareOptionType1 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareOrganisationIdentification6 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareOrganisationIdentification6 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<OrganisationIdentification15Choice>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<OrganisationIdentification15Choice>,
}


// CompareOrganisationIdentification7 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareOrganisationIdentification7 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<PartyIdentification236Choice>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<PartyIdentification236Choice>,
}


// CompareOtherPayment1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareOtherPayment1 {
	#[validate]
	#[serde(rename = "OthrPmtTp")]
	pub othr_pmt_tp: Option<CompareOtherPaymentType1>,
	#[validate]
	#[serde(rename = "OthrPmtAmt")]
	pub othr_pmt_amt: Option<CompareAmountAndDirection3>,
	#[validate]
	#[serde(rename = "OthrPmtDt")]
	pub othr_pmt_dt: Option<CompareDate3>,
	#[validate]
	#[serde(rename = "OthrPmtPyer")]
	pub othr_pmt_pyer: Option<CompareOrganisationIdentification7>,
	#[validate]
	#[serde(rename = "OthrPmtRcvr")]
	pub othr_pmt_rcvr: Option<CompareOrganisationIdentification7>,
}


// CompareOtherPaymentType1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareOtherPaymentType1 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<PaymentType5Choice>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<PaymentType5Choice>,
}


// ComparePercentageRate3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ComparePercentageRate3 {
	#[serde(rename = "Val1")]
	pub val1: Option<f64>,
	#[serde(rename = "Val2")]
	pub val2: Option<f64>,
}


// ComparePostTradeRiskReduction2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ComparePostTradeRiskReduction2 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<PTRREvent3>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<PTRREvent3>,
}


// CompareReferenceParty1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareReferenceParty1 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<DerivativePartyIdentification1Choice>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<DerivativePartyIdentification1Choice>,
}


// CompareReportingLevelType2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareReportingLevelType2 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareSeniorityType1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareSeniorityType1 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareText1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareText1 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareText2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareText2 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareTimePeriod2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareTimePeriod2 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<TimePeriod3>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<TimePeriod3>,
}


// CompareTradeClearingObligation1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareTradeClearingObligation1 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareTradeClearingStatus3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareTradeClearingStatus3 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<Cleared23Choice>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<Cleared23Choice>,
}


// CompareTradeConfirmation2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareTradeConfirmation2 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<TradeConfirmation3Choice>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<TradeConfirmation3Choice>,
}


// CompareTrancheIndicator1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareTrancheIndicator1 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<TrancheIndicator3Choice>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<TrancheIndicator3Choice>,
}


// CompareTrueFalseIndicator3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareTrueFalseIndicator3 {
	#[serde(rename = "Val1")]
	pub val1: Option<bool>,
	#[serde(rename = "Val2")]
	pub val2: Option<bool>,
}


// CompareUnderlyingInstrument3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareUnderlyingInstrument3 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<SecurityIdentification41Choice>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<SecurityIdentification41Choice>,
}


// CompareUniqueProductIdentifier2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareUniqueProductIdentifier2 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<UniqueProductIdentifier2Choice>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<UniqueProductIdentifier2Choice>,
}


// CompareUniqueTransactionIdentifier2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareUniqueTransactionIdentifier2 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<UniqueTransactionIdentifier2Choice>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<UniqueTransactionIdentifier2Choice>,
}


// CompareUnitPrice4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareUnitPrice4 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<SecuritiesTransactionPrice17Choice>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<SecuritiesTransactionPrice17Choice>,
}


// CompareUnitPrice5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareUnitPrice5 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<SecuritiesTransactionPrice17Choice>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<SecuritiesTransactionPrice17Choice>,
}


// CompareUnitPrice7 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareUnitPrice7 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<SecuritiesTransactionPrice14Choice>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<SecuritiesTransactionPrice14Choice>,
}


// CompareUnitPrice8 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareUnitPrice8 {
	#[validate]
	#[serde(rename = "Val1")]
	pub val1: Option<SecuritiesTransactionPrice13Choice>,
	#[validate]
	#[serde(rename = "Val2")]
	pub val2: Option<SecuritiesTransactionPrice13Choice>,
}


// CompareValuationType1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareValuationType1 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// CompareWeekDay1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CompareWeekDay1 {
	#[serde(rename = "Val1")]
	pub val1: Option<String>,
	#[serde(rename = "Val2")]
	pub val2: Option<String>,
}


// ContractMatchingCriteria3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ContractMatchingCriteria3 {
	#[validate]
	#[serde(rename = "ISIN")]
	pub isin: Option<CompareISINIdentifier2>,
	#[validate]
	#[serde(rename = "UnqPdctIdr")]
	pub unq_pdct_idr: Option<CompareUniqueProductIdentifier2>,
	#[validate]
	#[serde(rename = "AltrntvInstrmId")]
	pub altrntv_instrm_id: Option<CompareText1>,
	#[validate]
	#[serde(rename = "PdctClssfctn")]
	pub pdct_clssfctn: Option<CompareCFIIdentifier3>,
	#[validate]
	#[serde(rename = "CtrctTp")]
	pub ctrct_tp: Option<CompareFinancialInstrumentContractType1>,
	#[validate]
	#[serde(rename = "AsstClss")]
	pub asst_clss: Option<CompareAssetClass1>,
	#[validate]
	#[serde(rename = "DerivBasedOnCrptAsst")]
	pub deriv_based_on_crpt_asst: Option<CompareTrueFalseIndicator3>,
	#[validate]
	#[serde(rename = "UndrlygInstrm")]
	pub undrlyg_instrm: Option<CompareUnderlyingInstrument3>,
	#[validate]
	#[serde(rename = "SttlmCcy")]
	pub sttlm_ccy: Option<CompareActiveOrHistoricCurrencyCode1>,
	#[validate]
	#[serde(rename = "SttlmCcyScndLeg")]
	pub sttlm_ccy_scnd_leg: Option<CompareActiveOrHistoricCurrencyCode1>,
}


// CounterpartyData91 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CounterpartyData91 {
	#[validate]
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: Option<OrganisationIdentification15Choice>,
	#[validate]
	#[serde(rename = "OthrCtrPty")]
	pub othr_ctr_pty: Option<PartyIdentification236Choice>,
	#[validate]
	#[serde(rename = "RptSubmitgNtty")]
	pub rpt_submitg_ntty: Option<OrganisationIdentification15Choice>,
	#[validate]
	#[serde(rename = "NttyRspnsblForRpt")]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
}


// CounterpartyMatchingCriteria6 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CounterpartyMatchingCriteria6 {
	#[validate]
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: Option<CompareOrganisationIdentification6>,
	#[validate]
	#[serde(rename = "OthrCtrPty")]
	pub othr_ctr_pty: Option<CompareOrganisationIdentification7>,
	#[validate]
	#[serde(rename = "DrctnOrSd")]
	pub drctn_or_sd: Option<CompareLegDirection2>,
}


// CountryCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CountryCode {
	#[validate(pattern = "[A-Z]{2,2}")]
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// CountrySubDivisionCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CountrySubDivisionCode {
	#[validate(pattern = "[A-Z]{2,2}\\-[0-9A-Z]{1,3}")]
	#[serde(rename = "CountrySubDivisionCode")]
	pub country_sub_division_code: String,
}


// CustomBasket4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CustomBasket4 {
	#[serde(rename = "Strr")]
	pub strr: Option<String>,
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[validate]
	#[serde(rename = "Cnsttnts")]
	pub cnsttnts: Option<Vec<BasketConstituents3>>,
}


// DateAndDateTime2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DateAndDateTime2Choice {
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm")]
	pub dt_tm: Option<String>,
}


// DatePeriod4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DatePeriod4 {
	#[serde(rename = "FrDt")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt")]
	pub to_dt: Option<String>,
}


// DebtInstrumentSeniorityType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DebtInstrumentSeniorityType2Code {
	#[validate(enumerate = ["SBOD", "SNDB", "OTHR"])]
	#[serde(rename = "DebtInstrumentSeniorityType2Code")]
	pub debt_instrument_seniority_type2_code: String,
}


// DeliveryInterconnectionPoint1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DeliveryInterconnectionPoint1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// DerivativeEvent6 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DerivativeEvent6 {
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[validate]
	#[serde(rename = "Id")]
	pub id: Option<EventIdentifier1Choice>,
	#[validate]
	#[serde(rename = "TmStmp")]
	pub tm_stmp: Option<DateAndDateTime2Choice>,
	#[serde(rename = "AmdmntInd")]
	pub amdmnt_ind: Option<bool>,
}


// DerivativeEventType3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DerivativeEventType3Code {
	#[validate(enumerate = ["ALOC", "CLRG", "CLAL", "COMP", "CORP", "CREV", "ETRM", "EXER", "INCP", "NOVA", "PTNG", "TRAD", "UPDT"])]
	#[serde(rename = "DerivativeEventType3Code")]
	pub derivative_event_type3_code: String,
}


// DerivativePartyIdentification1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DerivativePartyIdentification1Choice {
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
	#[serde(rename = "CtrySubDvsn")]
	pub ctry_sub_dvsn: Option<String>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
}


// DerivativesTradeReconciliationStatisticalReportV03 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DerivativesTradeReconciliationStatisticalReportV03 {
	#[validate]
	#[serde(rename = "RcncltnSttstcs")]
	pub rcncltn_sttstcs: StatisticsPerCounterparty19Choice,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// Direction2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Direction2 {
	#[serde(rename = "DrctnOfTheFrstLeg")]
	pub drctn_of_the_frst_leg: String,
	#[serde(rename = "DrctnOfTheScndLeg")]
	pub drctn_of_the_scnd_leg: Option<String>,
}


// Direction4Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Direction4Choice {
	#[validate]
	#[serde(rename = "Drctn")]
	pub drctn: Option<Direction2>,
	#[serde(rename = "CtrPtySd")]
	pub ctr_pty_sd: Option<String>,
}


// DurationType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DurationType1Code {
	#[validate(enumerate = ["YEAR", "WEEK", "SEAS", "QURT", "MNTH", "MNUT", "HOUR", "DASD", "OTHR"])]
	#[serde(rename = "DurationType1Code")]
	pub duration_type1_code: String,
}


// EICIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EICIdentifier {
	#[validate(pattern = "[A-Z0-9\\-]{16}")]
	#[serde(rename = "EICIdentifier")]
	pub eic_identifier: String,
}


// EnergyCommodityCoal2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnergyCommodityCoal2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// EnergyCommodityDistillates2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnergyCommodityDistillates2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// EnergyCommodityElectricity2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnergyCommodityElectricity2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// EnergyCommodityInterEnergy2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnergyCommodityInterEnergy2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// EnergyCommodityLightEnd2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnergyCommodityLightEnd2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// EnergyCommodityNaturalGas3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnergyCommodityNaturalGas3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// EnergyCommodityOil3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnergyCommodityOil3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// EnergyCommodityOther2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnergyCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// EnergyCommodityRenewableEnergy2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnergyCommodityRenewableEnergy2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// EnergyLoadType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnergyLoadType1Code {
	#[validate(enumerate = ["BSLD", "GASD", "HABH", "OFFP", "OTHR", "PKLD", "SHPD"])]
	#[serde(rename = "EnergyLoadType1Code")]
	pub energy_load_type1_code: String,
}


// EnergyQuantityUnit2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnergyQuantityUnit2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// EnergyQuantityUnit2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnergyQuantityUnit2Code {
	#[validate(enumerate = ["BTUD", "CMPD", "GJDD", "GWAT", "GWHD", "GWHH", "HMJD", "KTMD", "KWAT", "KWHD", "KWHH", "MCMD", "MJDD", "MBTD", "MMJD", "MTMD", "MWAT", "MWHD", "MWHH", "THMD"])]
	#[serde(rename = "EnergyQuantityUnit2Code")]
	pub energy_quantity_unit2_code: String,
}


// EnvironmentCommodityOther2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnvironmentCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// EnvironmentalCommodityCarbonRelated2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnvironmentalCommodityCarbonRelated2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// EnvironmentalCommodityEmission3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnvironmentalCommodityEmission3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// EnvironmentalCommodityWeather2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnvironmentalCommodityWeather2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// EventIdentifier1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EventIdentifier1Choice {
	#[serde(rename = "EvtIdr")]
	pub evt_idr: Option<String>,
	#[validate]
	#[serde(rename = "PstTradRskRdctnIdr")]
	pub pst_trad_rsk_rdctn_idr: Option<PostTradeRiskReductionIdentifier1>,
}


// ExchangeRateBasis1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExchangeRateBasis1 {
	#[serde(rename = "BaseCcy")]
	pub base_ccy: String,
	#[serde(rename = "QtdCcy")]
	pub qtd_ccy: String,
}


// ExchangeRateBasis1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExchangeRateBasis1Choice {
	#[validate]
	#[serde(rename = "CcyPair")]
	pub ccy_pair: Option<ExchangeRateBasis1>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ExternalAgreementType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalAgreementType1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalAgreementType1Code")]
	pub external_agreement_type1_code: String,
}


// ExternalBenchmarkCurveName1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalBenchmarkCurveName1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalBenchmarkCurveName1Code")]
	pub external_benchmark_curve_name1_code: String,
}


// ExternalUnitOfMeasure1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalUnitOfMeasure1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalUnitOfMeasure1Code")]
	pub external_unit_of_measure1_code: String,
}


// FertilizerCommodityAmmonia2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FertilizerCommodityAmmonia2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FertilizerCommodityDiammoniumPhosphate2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FertilizerCommodityDiammoniumPhosphate2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FertilizerCommodityOther2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FertilizerCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FertilizerCommodityPotash2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FertilizerCommodityPotash2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FertilizerCommoditySulphur2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FertilizerCommoditySulphur2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FertilizerCommodityUrea2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FertilizerCommodityUrea2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FertilizerCommodityUreaAndAmmoniumNitrate2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FertilizerCommodityUreaAndAmmoniumNitrate2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FinancialInstrumentContractType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstrumentContractType2Code {
	#[validate(enumerate = ["CFDS", "FRAS", "FUTR", "FORW", "OPTN", "SPDB", "SWAP", "SWPT", "OTHR"])]
	#[serde(rename = "FinancialInstrumentContractType2Code")]
	pub financial_instrument_contract_type2_code: String,
}


// FreightCommodityContainerShip2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FreightCommodityContainerShip2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FreightCommodityDry3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FreightCommodityDry3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// FreightCommodityOther2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FreightCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// FreightCommodityWet3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FreightCommodityWet3 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// Frequency13Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Frequency13Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[validate(enumerate = ["DAIL", "WEEK", "MNTH", "YEAR", "ADHO", "EXPI", "MIAN", "QURT"])]
	#[serde(rename = "Frequency13Code")]
	pub frequency13_code: String,
}


// GenericIdentification175 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification175 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericIdentification179 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification179 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericIdentification184 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification184 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Src")]
	pub src: String,
}


// GenericIdentification185 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification185 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// ISINOct2015Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISINOct2015Identifier {
	#[validate(pattern = "[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}")]
	#[serde(rename = "ISINOct2015Identifier")]
	pub isin_oct2015_identifier: String,
}


// ISODate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// ISOTime ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISOTime {
	#[serde(rename = "ISOTime")]
	pub iso_time: String,
}


// IndexIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct IndexIdentification1 {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Indx")]
	pub indx: Option<String>,
}


// IndustrialProductCommodityConstruction2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct IndustrialProductCommodityConstruction2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// IndustrialProductCommodityManufacturing2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct IndustrialProductCommodityManufacturing2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// InstrumentIdentification6Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InstrumentIdentification6Choice {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[serde(rename = "AltrntvInstrmId")]
	pub altrntv_instrm_id: Option<String>,
	#[validate]
	#[serde(rename = "UnqPdctIdr")]
	pub unq_pdct_idr: Option<UniqueProductIdentifier1Choice>,
	#[validate]
	#[serde(rename = "OthrId")]
	pub othr_id: Option<GenericIdentification184>,
}


// InterestComputationMethod4Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InterestComputationMethod4Code {
	#[validate(enumerate = ["A004", "A019", "A017", "A005", "A009", "A014", "A010", "A006", "A008", "A015", "A018", "A011", "A001", "A002", "A003", "A012", "A013", "A007", "A016", "NARR", "A020"])]
	#[serde(rename = "InterestComputationMethod4Code")]
	pub interest_computation_method4_code: String,
}


// InterestComputationMethodFormat7 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InterestComputationMethodFormat7 {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "Nrrtv")]
	pub nrrtv: Option<String>,
}


// LEIIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[validate(pattern = "[A-Z0-9]{18,18}[0-9]{2,2}")]
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LegalPersonIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LegalPersonIdentification1 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: OrganisationIdentification15Choice,
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
}


// LongFraction19DecimalNumber ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LongFraction19DecimalNumber {
	#[serde(rename = "LongFraction19DecimalNumber")]
	pub long_fraction19_decimal_number: f64,
}


// MICIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MICIdentifier {
	#[validate(pattern = "[A-Z0-9]{4,4}")]
	#[serde(rename = "MICIdentifier")]
	pub mic_identifier: String,
}


// MarginPortfolio3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MarginPortfolio3 {
	#[validate]
	#[serde(rename = "InitlMrgnPrtflCd")]
	pub initl_mrgn_prtfl_cd: PortfolioCode5Choice,
	#[validate]
	#[serde(rename = "VartnMrgnPrtflCd")]
	pub vartn_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
}


// MasterAgreement8 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MasterAgreement8 {
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: Option<AgreementType2Choice>,
	#[serde(rename = "Vrsn")]
	pub vrsn: Option<String>,
	#[serde(rename = "OthrMstrAgrmtDtls")]
	pub othr_mstr_agrmt_dtls: Option<String>,
}


// MatchingCriteria17 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MatchingCriteria17 {
	#[validate]
	#[serde(rename = "CtrPtyMtchgCrit")]
	pub ctr_pty_mtchg_crit: Option<CounterpartyMatchingCriteria6>,
	#[validate]
	#[serde(rename = "ValtnMtchgCrit")]
	pub valtn_mtchg_crit: Option<ValuationMatchingCriteria1>,
	#[validate]
	#[serde(rename = "CtrctMtchgCrit")]
	pub ctrct_mtchg_crit: Option<ContractMatchingCriteria3>,
	#[validate]
	#[serde(rename = "TxMtchgCrit")]
	pub tx_mtchg_crit: Option<TransactionMatchingCriteria7>,
}


// Max1000Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max1000Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 1000)]
	#[serde(rename = "Max1000Text")]
	pub max1000_text: String,
}


// Max100Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max100Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 100)]
	#[serde(rename = "Max100Text")]
	pub max100_text: String,
}


// Max105Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max105Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 105)]
	#[serde(rename = "Max105Text")]
	pub max105_text: String,
}


// Max140Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max140Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 140)]
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max210Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max210Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 210)]
	#[serde(rename = "Max210Text")]
	pub max210_text: String,
}


// Max350Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max350Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 350)]
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max35Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 35)]
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Max3Number ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max3Number {
	#[serde(rename = "Max3Number")]
	pub max3_number: f64,
}


// Max4AlphaNumericText ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max4AlphaNumericText {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[validate(pattern = "[a-zA-Z0-9]{1,4}")]
	#[serde(rename = "Max4AlphaNumericText")]
	pub max4_alpha_numeric_text: String,
}


// Max500Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max500Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 500)]
	#[serde(rename = "Max500Text")]
	pub max500_text: String,
}


// Max50Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max50Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 50)]
	#[serde(rename = "Max50Text")]
	pub max50_text: String,
}


// Max52Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max52Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 52)]
	#[serde(rename = "Max52Text")]
	pub max52_text: String,
}


// Max72Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max72Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 72)]
	#[serde(rename = "Max72Text")]
	pub max72_text: String,
}


// MetalCommodityNonPrecious2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MetalCommodityNonPrecious2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// MetalCommodityPrecious2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MetalCommodityPrecious2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// ModificationLevel1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ModificationLevel1Code {
	#[validate(enumerate = ["PSTN", "TCTN"])]
	#[serde(rename = "ModificationLevel1Code")]
	pub modification_level1_code: String,
}


// NaturalPersonIdentification2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NaturalPersonIdentification2 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Dmcl")]
	pub dmcl: Option<String>,
}


// NaturalPersonIdentification3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NaturalPersonIdentification3 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: NaturalPersonIdentification2,
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
}


// NoReasonCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NoReasonCode {
	#[validate(enumerate = ["NORE"])]
	#[serde(rename = "NoReasonCode")]
	pub no_reason_code: String,
}


// NonClearingReason2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NonClearingReason2 {
	#[serde(rename = "ClrXmptnXcptn")]
	pub clr_xmptn_xcptn: Vec<String>,
	#[serde(rename = "NonClrRsnInf")]
	pub non_clr_rsn_inf: Option<String>,
}


// NotApplicable1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NotApplicable1Code {
	#[validate(max_length = 4)]
	#[validate(enumerate = ["NOAP"])]
	#[serde(rename = "NotApplicable1Code")]
	pub not_applicable1_code: String,
}


// Number ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
}


// OptionParty1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OptionParty1Code {
	#[validate(enumerate = ["SLLR", "BYER"])]
	#[serde(rename = "OptionParty1Code")]
	pub option_party1_code: String,
}


// OptionParty3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OptionParty3Code {
	#[validate(enumerate = ["MAKE", "TAKE"])]
	#[serde(rename = "OptionParty3Code")]
	pub option_party3_code: String,
}


// OptionStyle6Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OptionStyle6Code {
	#[validate(enumerate = ["EURO", "BERM", "ASIA", "AMER"])]
	#[serde(rename = "OptionStyle6Code")]
	pub option_style6_code: String,
}


// OptionType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OptionType2Code {
	#[validate(enumerate = ["CALL", "PUTO", "OTHR"])]
	#[serde(rename = "OptionType2Code")]
	pub option_type2_code: String,
}


// OrganisationIdentification15Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OrganisationIdentification15Choice {
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<OrganisationIdentification38>,
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
}


// OrganisationIdentification38 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OrganisationIdentification38 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: GenericIdentification175,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "Dmcl")]
	pub dmcl: Option<String>,
}


// PTRREvent3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PTRREvent3 {
	#[serde(rename = "Tchnq")]
	pub tchnq: Option<String>,
	#[validate]
	#[serde(rename = "SvcPrvdr")]
	pub svc_prvdr: Option<OrganisationIdentification15Choice>,
}


// PairingStatus1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PairingStatus1Code {
	#[validate(enumerate = ["PARD", "UNPR"])]
	#[serde(rename = "PairingStatus1Code")]
	pub pairing_status1_code: String,
}


// PaperCommodityContainerBoard2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PaperCommodityContainerBoard2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PaperCommodityNewsprint2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PaperCommodityNewsprint2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PaperCommodityOther1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PaperCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PaperCommodityPulp2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PaperCommodityPulp2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PartyIdentification236Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification236Choice {
	#[validate]
	#[serde(rename = "Lgl")]
	pub lgl: Option<OrganisationIdentification15Choice>,
	#[validate]
	#[serde(rename = "Ntrl")]
	pub ntrl: Option<NaturalPersonIdentification2>,
}


// PartyIdentification248Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification248Choice {
	#[validate]
	#[serde(rename = "Lgl")]
	pub lgl: Option<LegalPersonIdentification1>,
	#[validate]
	#[serde(rename = "Ntrl")]
	pub ntrl: Option<NaturalPersonIdentification3>,
}


// PaymentType4Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PaymentType4Code {
	#[validate(enumerate = ["UFRO", "UWIN", "PEXH"])]
	#[serde(rename = "PaymentType4Code")]
	pub payment_type4_code: String,
}


// PaymentType5Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PaymentType5Choice {
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "PrtryTp")]
	pub prtry_tp: Option<String>,
}


// PercentageRate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// PhysicalTransferType4Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PhysicalTransferType4Code {
	#[validate(enumerate = ["PHYS", "OPTL", "CASH"])]
	#[serde(rename = "PhysicalTransferType4Code")]
	pub physical_transfer_type4_code: String,
}


// PlusOrMinusIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "PlusOrMinusIndicator")]
	pub plus_or_minus_indicator: bool,
}


// PolypropyleneCommodityOther2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PolypropyleneCommodityOther2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PolypropyleneCommodityPlastic2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PolypropyleneCommodityPlastic2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PortfolioCode3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PortfolioCode3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "NoPrtfl")]
	pub no_prtfl: Option<String>,
}


// PortfolioCode5Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PortfolioCode5Choice {
	#[validate]
	#[serde(rename = "Prtfl")]
	pub prtfl: Option<PortfolioIdentification3>,
	#[serde(rename = "NoPrtfl")]
	pub no_prtfl: Option<String>,
}


// PortfolioIdentification3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PortfolioIdentification3 {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "PrtflTxXmptn")]
	pub prtfl_tx_xmptn: Option<bool>,
}


// PostTradeRiskReductionIdentifier1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PostTradeRiskReductionIdentifier1 {
	#[serde(rename = "Strr")]
	pub strr: String,
	#[serde(rename = "Id")]
	pub id: String,
}


// PriceStatus1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PriceStatus1Code {
	#[validate(enumerate = ["PNDG", "NOAP"])]
	#[serde(rename = "PriceStatus1Code")]
	pub price_status1_code: String,
}


// ProductType4Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ProductType4Code {
	#[validate(enumerate = ["CRDT", "CURR", "EQUI", "INTR", "COMM", "OTHR"])]
	#[serde(rename = "ProductType4Code")]
	pub product_type4_code: String,
}


// ReconciliationCategory4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReconciliationCategory4 {
	#[serde(rename = "Rvvd")]
	pub rvvd: bool,
	#[serde(rename = "FrthrMod")]
	pub frthr_mod: bool,
}


// ReconciliationCategory5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReconciliationCategory5 {
	#[serde(rename = "RptgTp")]
	pub rptg_tp: String,
	#[serde(rename = "Pairg")]
	pub pairg: String,
	#[serde(rename = "Rcncltn")]
	pub rcncltn: String,
	#[serde(rename = "ValtnRcncltn")]
	pub valtn_rcncltn: String,
	#[serde(rename = "Rvvd")]
	pub rvvd: bool,
	#[serde(rename = "FrthrMod")]
	pub frthr_mod: bool,
}


// ReconciliationCounterpartyPairStatistics7 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReconciliationCounterpartyPairStatistics7 {
	#[validate]
	#[serde(rename = "CtrPtyId")]
	pub ctr_pty_id: CounterpartyData91,
	#[serde(rename = "TtlNbOfTxs")]
	pub ttl_nb_of_txs: f64,
	#[validate]
	#[serde(rename = "RcncltnRpt")]
	pub rcncltn_rpt: Vec<ReconciliationReport15>,
}


// ReconciliationReport15 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReconciliationReport15 {
	#[validate]
	#[serde(rename = "TxId")]
	pub tx_id: TradeTransactionIdentification24,
	#[validate]
	#[serde(rename = "MtchgCrit")]
	pub mtchg_crit: MatchingCriteria17,
}


// ReconciliationStatisticsPerCounterparty4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReconciliationStatisticsPerCounterparty4 {
	#[serde(rename = "RefDt")]
	pub ref_dt: String,
	#[validate]
	#[serde(rename = "RcncltnCtgrs")]
	pub rcncltn_ctgrs: ReportingRequirement3Choice,
	#[serde(rename = "TtlNbOfTxs")]
	pub ttl_nb_of_txs: Option<f64>,
	#[validate]
	#[serde(rename = "TxDtls")]
	pub tx_dtls: Option<Vec<ReconciliationCounterpartyPairStatistics7>>,
}


// ReconciliationStatus1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReconciliationStatus1Code {
	#[validate(enumerate = ["NREC", "RECO"])]
	#[serde(rename = "ReconciliationStatus1Code")]
	pub reconciliation_status1_code: String,
}


// ReconciliationStatus2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReconciliationStatus2Code {
	#[validate(enumerate = ["NREC", "RECO", "NOAP"])]
	#[serde(rename = "ReconciliationStatus2Code")]
	pub reconciliation_status2_code: String,
}


// ReportPeriodActivity1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReportPeriodActivity1Code {
	#[validate(enumerate = ["NOTX"])]
	#[serde(rename = "ReportPeriodActivity1Code")]
	pub report_period_activity1_code: String,
}


// ReportingRequirement3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReportingRequirement3Choice {
	#[validate]
	#[serde(rename = "RptgRqrmnt")]
	pub rptg_rqrmnt: Option<ReconciliationCategory5>,
	#[validate]
	#[serde(rename = "NoRptgRqrmnt")]
	pub no_rptg_rqrmnt: Option<ReconciliationCategory4>,
}


// RiskReductionService1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RiskReductionService1Code {
	#[validate(enumerate = ["NORR", "PWOS", "OTHR", "PRBM", "PWAS"])]
	#[serde(rename = "RiskReductionService1Code")]
	pub risk_reduction_service1_code: String,
}


// SecuritiesTransactionPrice13Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice13Choice {
	#[validate]
	#[serde(rename = "MntryVal")]
	pub mntry_val: Option<AmountAndDirection106>,
	#[serde(rename = "Pctg")]
	pub pctg: Option<f64>,
	#[serde(rename = "Dcml")]
	pub dcml: Option<f64>,
	#[serde(rename = "BsisPtSprd")]
	pub bsis_pt_sprd: Option<f64>,
}


// SecuritiesTransactionPrice14Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice14Choice {
	#[serde(rename = "Rate")]
	pub rate: Option<f64>,
	#[serde(rename = "Dcml")]
	pub dcml: Option<f64>,
}


// SecuritiesTransactionPrice17Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice17Choice {
	#[validate]
	#[serde(rename = "MntryVal")]
	pub mntry_val: Option<AmountAndDirection106>,
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
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<SecuritiesTransactionPrice5>,
}


// SecuritiesTransactionPrice5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice5 {
	#[serde(rename = "Val")]
	pub val: Option<f64>,
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
}


// SecurityIdentification41Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityIdentification41Choice {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[serde(rename = "AltrntvInstrmId")]
	pub altrntv_instrm_id: Option<String>,
	#[validate]
	#[serde(rename = "UnqPdctIdr")]
	pub unq_pdct_idr: Option<UniqueProductIdentifier2Choice>,
	#[validate]
	#[serde(rename = "Bskt")]
	pub bskt: Option<CustomBasket4>,
	#[validate]
	#[serde(rename = "Indx")]
	pub indx: Option<IndexIdentification1>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<GenericIdentification184>,
	#[serde(rename = "IdNotAvlbl")]
	pub id_not_avlbl: Option<String>,
}


// StatisticsPerCounterparty19Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct StatisticsPerCounterparty19Choice {
	#[serde(rename = "DataSetActn")]
	pub data_set_actn: Option<String>,
	#[validate]
	#[serde(rename = "Rpt")]
	pub rpt: Option<Vec<ReconciliationStatisticsPerCounterparty4>>,
}


// SupplementaryData1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Option<String>,
	#[validate]
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}


// TimePeriod3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TimePeriod3 {
	#[serde(rename = "FrTm")]
	pub fr_tm: Option<String>,
	#[serde(rename = "ToTm")]
	pub to_tm: Option<String>,
}


// TradeConfirmation3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeConfirmation3Choice {
	#[validate]
	#[serde(rename = "Confd")]
	pub confd: Option<TradeConfirmation4>,
	#[validate]
	#[serde(rename = "NonConfd")]
	pub non_confd: Option<TradeNonConfirmation1>,
}


// TradeConfirmation4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeConfirmation4 {
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "TmStmp")]
	pub tm_stmp: Option<String>,
}


// TradeConfirmationType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeConfirmationType1Code {
	#[validate(enumerate = ["ECNF", "YCNF"])]
	#[serde(rename = "TradeConfirmationType1Code")]
	pub trade_confirmation_type1_code: String,
}


// TradeConfirmationType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeConfirmationType2Code {
	#[validate(enumerate = ["NCNF"])]
	#[serde(rename = "TradeConfirmationType2Code")]
	pub trade_confirmation_type2_code: String,
}


// TradeNonConfirmation1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeNonConfirmation1 {
	#[serde(rename = "Tp")]
	pub tp: String,
}


// TradeRepositoryReportingType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeRepositoryReportingType1Code {
	#[validate(enumerate = ["SWOS", "TWOS"])]
	#[serde(rename = "TradeRepositoryReportingType1Code")]
	pub trade_repository_reporting_type1_code: String,
}


// TradeTransactionIdentification24 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeTransactionIdentification24 {
	#[serde(rename = "TechRcrdId")]
	pub tech_rcrd_id: Option<String>,
	#[serde(rename = "ActnTp")]
	pub actn_tp: Option<String>,
	#[serde(rename = "RptgTmStmp")]
	pub rptg_tm_stmp: Option<String>,
	#[serde(rename = "DerivEvtTp")]
	pub deriv_evt_tp: Option<String>,
	#[validate]
	#[serde(rename = "DerivEvtTmStmp")]
	pub deriv_evt_tm_stmp: Option<DateAndDateTime2Choice>,
	#[validate]
	#[serde(rename = "OthrCtrPty")]
	pub othr_ctr_pty: Option<PartyIdentification248Choice>,
	#[validate]
	#[serde(rename = "UnqIdr")]
	pub unq_idr: Option<UniqueTransactionIdentifier2Choice>,
	#[validate]
	#[serde(rename = "MstrAgrmt")]
	pub mstr_agrmt: Option<MasterAgreement8>,
	#[validate]
	#[serde(rename = "CollPrtflCd")]
	pub coll_prtfl_cd: Option<CollateralPortfolioCode5Choice>,
}


// Tranche3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Tranche3 {
	#[serde(rename = "AttchmntPt")]
	pub attchmnt_pt: Option<f64>,
	#[serde(rename = "DtchmntPt")]
	pub dtchmnt_pt: Option<f64>,
}


// TrancheIndicator3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TrancheIndicator3Choice {
	#[validate]
	#[serde(rename = "Trnchd")]
	pub trnchd: Option<Tranche3>,
	#[serde(rename = "Utrnchd")]
	pub utrnchd: Option<String>,
}


// TransactionMatchingCriteria7 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TransactionMatchingCriteria7 {
	#[validate]
	#[serde(rename = "RptTrckgNb")]
	pub rpt_trckg_nb: Option<CompareText2>,
	#[validate]
	#[serde(rename = "UnqTxIdr")]
	pub unq_tx_idr: Option<CompareUniqueTransactionIdentifier2>,
	#[validate]
	#[serde(rename = "PrrUnqTxIdr")]
	pub prr_unq_tx_idr: Option<CompareUniqueTransactionIdentifier2>,
	#[validate]
	#[serde(rename = "SbsqntPosUnqTxIdr")]
	pub sbsqnt_pos_unq_tx_idr: Option<CompareUniqueTransactionIdentifier2>,
	#[validate]
	#[serde(rename = "Dlta")]
	pub dlta: Option<CompareLongFraction19DecimalNumber1>,
	#[validate]
	#[serde(rename = "TradConf")]
	pub trad_conf: Option<CompareTradeConfirmation2>,
	#[validate]
	#[serde(rename = "TradClrOblgtn")]
	pub trad_clr_oblgtn: Option<CompareTradeClearingObligation1>,
	#[validate]
	#[serde(rename = "TradClrSts")]
	pub trad_clr_sts: Option<CompareTradeClearingStatus3>,
	#[validate]
	#[serde(rename = "MstrAgrmtTp")]
	pub mstr_agrmt_tp: Option<CompareMasterAgreementType1>,
	#[validate]
	#[serde(rename = "MstrAgrmtVrsn")]
	pub mstr_agrmt_vrsn: Option<CompareMax50Text1>,
	#[validate]
	#[serde(rename = "IntraGrp")]
	pub intra_grp: Option<CompareTrueFalseIndicator3>,
	#[validate]
	#[serde(rename = "PstTradRskRdctn")]
	pub pst_trad_rsk_rdctn: Option<ComparePostTradeRiskReduction2>,
	#[validate]
	#[serde(rename = "DerivEvt")]
	pub deriv_evt: Option<CompareDerivativeEvent1>,
	#[validate]
	#[serde(rename = "PltfmIdr")]
	pub pltfm_idr: Option<CompareMICIdentifier3>,
	#[validate]
	#[serde(rename = "ExctnTmStmp")]
	pub exctn_tm_stmp: Option<CompareDateTime3>,
	#[validate]
	#[serde(rename = "FctvDt")]
	pub fctv_dt: Option<CompareDate3>,
	#[validate]
	#[serde(rename = "XprtnDt")]
	pub xprtn_dt: Option<CompareDate3>,
	#[validate]
	#[serde(rename = "EarlyTermntnDt")]
	pub early_termntn_dt: Option<CompareDate3>,
	#[validate]
	#[serde(rename = "SttlmDt")]
	pub sttlm_dt: Option<Vec<CompareDate3>>,
	#[validate]
	#[serde(rename = "DlvryTp")]
	pub dlvry_tp: Option<CompareDeliveryType1>,
	#[validate]
	#[serde(rename = "TxPric")]
	pub tx_pric: Option<CompareUnitPrice5>,
	#[validate]
	#[serde(rename = "PricSchdlUadjstdFctvDt")]
	pub pric_schdl_uadjstd_fctv_dt: Option<Vec<CompareDate3>>,
	#[validate]
	#[serde(rename = "PricSchdlUadjstdEndDt")]
	pub pric_schdl_uadjstd_end_dt: Option<Vec<CompareDate3>>,
	#[validate]
	#[serde(rename = "TxSchdlPric")]
	pub tx_schdl_pric: Option<Vec<CompareUnitPrice5>>,
	#[validate]
	#[serde(rename = "PackgPric")]
	pub packg_pric: Option<CompareUnitPrice5>,
	#[validate]
	#[serde(rename = "NtnlAmtFrstLeg")]
	pub ntnl_amt_frst_leg: Option<CompareAmountAndDirection3>,
	#[validate]
	#[serde(rename = "NtnlAmtFrstLegUadjstdFctvDt")]
	pub ntnl_amt_frst_leg_uadjstd_fctv_dt: Option<Vec<CompareDate3>>,
	#[validate]
	#[serde(rename = "NtnlAmtFrstLegUadjstdEndDt")]
	pub ntnl_amt_frst_leg_uadjstd_end_dt: Option<Vec<CompareDate3>>,
	#[validate]
	#[serde(rename = "NtnlAmtFrstLegSchdlAmt")]
	pub ntnl_amt_frst_leg_schdl_amt: Option<Vec<CompareAmountAndDirection3>>,
	#[validate]
	#[serde(rename = "NtnlQtyFrstLeg")]
	pub ntnl_qty_frst_leg: Option<CompareLongFraction19DecimalNumber1>,
	#[validate]
	#[serde(rename = "NtnlQtyFrstLegUadjstdFctvDt")]
	pub ntnl_qty_frst_leg_uadjstd_fctv_dt: Option<Vec<CompareDate3>>,
	#[validate]
	#[serde(rename = "NtnlQtyFrstLegUadjstdEndDt")]
	pub ntnl_qty_frst_leg_uadjstd_end_dt: Option<Vec<CompareDate3>>,
	#[validate]
	#[serde(rename = "NtnlQtyFrstLegSchdlQty")]
	pub ntnl_qty_frst_leg_schdl_qty: Option<Vec<CompareLongFraction19DecimalNumber1>>,
	#[validate]
	#[serde(rename = "NtnlAmtScndLeg")]
	pub ntnl_amt_scnd_leg: Option<CompareAmountAndDirection3>,
	#[validate]
	#[serde(rename = "NtnlAmtScndLegUadjstdFctvDt")]
	pub ntnl_amt_scnd_leg_uadjstd_fctv_dt: Option<Vec<CompareDate3>>,
	#[validate]
	#[serde(rename = "NtnlAmtScndLegUadjstdEndDt")]
	pub ntnl_amt_scnd_leg_uadjstd_end_dt: Option<Vec<CompareDate3>>,
	#[validate]
	#[serde(rename = "NtnlAmtScndLegSchdlAmt")]
	pub ntnl_amt_scnd_leg_schdl_amt: Option<Vec<CompareAmountAndDirection3>>,
	#[validate]
	#[serde(rename = "NtnlQtyScndLeg")]
	pub ntnl_qty_scnd_leg: Option<CompareLongFraction19DecimalNumber1>,
	#[validate]
	#[serde(rename = "NtnlQtyScndLegUadjstdFctvDt")]
	pub ntnl_qty_scnd_leg_uadjstd_fctv_dt: Option<Vec<CompareDate3>>,
	#[validate]
	#[serde(rename = "NtnlQtyScndLegUadjstdEndDt")]
	pub ntnl_qty_scnd_leg_uadjstd_end_dt: Option<Vec<CompareDate3>>,
	#[validate]
	#[serde(rename = "NtnlQtyScndLegSchdlQty")]
	pub ntnl_qty_scnd_leg_schdl_qty: Option<Vec<CompareLongFraction19DecimalNumber1>>,
	#[validate]
	#[serde(rename = "OthrPmt")]
	pub othr_pmt: Option<Vec<CompareOtherPayment1>>,
	#[validate]
	#[serde(rename = "IntrstFxdRateFrstLeg")]
	pub intrst_fxd_rate_frst_leg: Option<CompareUnitPrice7>,
	#[validate]
	#[serde(rename = "IntrstFxdRateFrstLegDayCnt")]
	pub intrst_fxd_rate_frst_leg_day_cnt: Option<CompareDayCount1>,
	#[validate]
	#[serde(rename = "IntrstFxdRateFrstLegPmtFrqcyUnit")]
	pub intrst_fxd_rate_frst_leg_pmt_frqcy_unit: Option<CompareFrequencyUnit1>,
	#[validate]
	#[serde(rename = "IntrstFxdRateFrstLegPmtFrqcyVal")]
	pub intrst_fxd_rate_frst_leg_pmt_frqcy_val: Option<CompareNumber5>,
	#[validate]
	#[serde(rename = "IntrstFltgRateFrstLegId")]
	pub intrst_fltg_rate_frst_leg_id: Option<CompareISINIdentifier4>,
	#[validate]
	#[serde(rename = "IntrstFltgRateFrstLegCd")]
	pub intrst_fltg_rate_frst_leg_cd: Option<CompareBenchmarkCode1>,
	#[validate]
	#[serde(rename = "IntrstFltgRateFrstLegNm")]
	pub intrst_fltg_rate_frst_leg_nm: Option<CompareMax350Text1>,
	#[validate]
	#[serde(rename = "IntrstFltgRateFrstLegDayCnt")]
	pub intrst_fltg_rate_frst_leg_day_cnt: Option<CompareDayCount1>,
	#[validate]
	#[serde(rename = "IntrstFltgRateFrstLegPmtFrqcyUnit")]
	pub intrst_fltg_rate_frst_leg_pmt_frqcy_unit: Option<CompareFrequencyUnit1>,
	#[validate]
	#[serde(rename = "IntrstFltgRateFrstLegPmtFrqcyVal")]
	pub intrst_fltg_rate_frst_leg_pmt_frqcy_val: Option<CompareNumber5>,
	#[validate]
	#[serde(rename = "IntrstFltgRateFrstLegRefPrdUnit")]
	pub intrst_fltg_rate_frst_leg_ref_prd_unit: Option<CompareFrequencyUnit1>,
	#[validate]
	#[serde(rename = "IntrstFltgRateFrstLegRefPrdVal")]
	pub intrst_fltg_rate_frst_leg_ref_prd_val: Option<CompareNumber5>,
	#[validate]
	#[serde(rename = "IntrstFltgRateFrstLegRstFrqcyUnit")]
	pub intrst_fltg_rate_frst_leg_rst_frqcy_unit: Option<CompareFrequencyUnit1>,
	#[validate]
	#[serde(rename = "IntrstFltgRateFrstLegRstFrqcyVal")]
	pub intrst_fltg_rate_frst_leg_rst_frqcy_val: Option<CompareNumber5>,
	#[validate]
	#[serde(rename = "IntrstFltgRateFrstLegSprd")]
	pub intrst_fltg_rate_frst_leg_sprd: Option<CompareUnitPrice8>,
	#[validate]
	#[serde(rename = "IntrstRateFxdScndLeg")]
	pub intrst_rate_fxd_scnd_leg: Option<CompareUnitPrice7>,
	#[validate]
	#[serde(rename = "IntrstFxdRateScndLegDayCnt")]
	pub intrst_fxd_rate_scnd_leg_day_cnt: Option<CompareDayCount1>,
	#[validate]
	#[serde(rename = "IntrstFxdRateScndLegPmtFrqcyUnit")]
	pub intrst_fxd_rate_scnd_leg_pmt_frqcy_unit: Option<CompareFrequencyUnit1>,
	#[validate]
	#[serde(rename = "IntrstFxdRateScndLegPmtFrqcyVal")]
	pub intrst_fxd_rate_scnd_leg_pmt_frqcy_val: Option<CompareNumber5>,
	#[validate]
	#[serde(rename = "IntrstFltgRateScndLegId")]
	pub intrst_fltg_rate_scnd_leg_id: Option<CompareISINIdentifier4>,
	#[validate]
	#[serde(rename = "IntrstFltgRateScndLegCd")]
	pub intrst_fltg_rate_scnd_leg_cd: Option<CompareBenchmarkCode1>,
	#[validate]
	#[serde(rename = "IntrstFltgRateScndLegNm")]
	pub intrst_fltg_rate_scnd_leg_nm: Option<CompareMax350Text1>,
	#[validate]
	#[serde(rename = "IntrstFltgRateScndLegDayCnt")]
	pub intrst_fltg_rate_scnd_leg_day_cnt: Option<CompareDayCount1>,
	#[validate]
	#[serde(rename = "IntrstFltgRateScndLegPmtFrqcyUnit")]
	pub intrst_fltg_rate_scnd_leg_pmt_frqcy_unit: Option<CompareFrequencyUnit1>,
	#[validate]
	#[serde(rename = "IntrstFltgRateScndLegPmtFrqcyVal")]
	pub intrst_fltg_rate_scnd_leg_pmt_frqcy_val: Option<CompareNumber5>,
	#[validate]
	#[serde(rename = "IntrstFltgRateScndLegRefPrdUnit")]
	pub intrst_fltg_rate_scnd_leg_ref_prd_unit: Option<CompareFrequencyUnit1>,
	#[validate]
	#[serde(rename = "IntrstFltgRateScndLegRefPrdVal")]
	pub intrst_fltg_rate_scnd_leg_ref_prd_val: Option<CompareNumber5>,
	#[validate]
	#[serde(rename = "IntrstFltgRateScndLegRstFrqcyUnit")]
	pub intrst_fltg_rate_scnd_leg_rst_frqcy_unit: Option<CompareFrequencyUnit1>,
	#[validate]
	#[serde(rename = "IntrstFltgRateScndLegRstFrqcyVal")]
	pub intrst_fltg_rate_scnd_leg_rst_frqcy_val: Option<CompareNumber5>,
	#[validate]
	#[serde(rename = "IntrstFltgRateScndLegSprd")]
	pub intrst_fltg_rate_scnd_leg_sprd: Option<CompareUnitPrice8>,
	#[validate]
	#[serde(rename = "PackgSprd")]
	pub packg_sprd: Option<CompareUnitPrice8>,
	#[validate]
	#[serde(rename = "CcyXchgRate")]
	pub ccy_xchg_rate: Option<CompareExchangeRate1>,
	#[validate]
	#[serde(rename = "CcyFwdXchgRate")]
	pub ccy_fwd_xchg_rate: Option<CompareExchangeRate1>,
	#[validate]
	#[serde(rename = "CcyXchgRateBsis")]
	pub ccy_xchg_rate_bsis: Option<CompareExchangeRateBasis1>,
	#[validate]
	#[serde(rename = "Cmmdty")]
	pub cmmdty: Option<CompareCommodityAssetClass4>,
	#[validate]
	#[serde(rename = "NrgyDlvryPtOrZone")]
	pub nrgy_dlvry_pt_or_zone: Option<Vec<CompareDeliveryInterconnectionPoint1>>,
	#[validate]
	#[serde(rename = "NrgyIntrCnnctnPt")]
	pub nrgy_intr_cnnctn_pt: Option<CompareDeliveryInterconnectionPoint1>,
	#[validate]
	#[serde(rename = "NrgyLdTp")]
	pub nrgy_ld_tp: Option<CompareEnergyLoadType1>,
	#[validate]
	#[serde(rename = "DlvryAttr")]
	pub dlvry_attr: Option<Vec<CompareEnergyDeliveryAttribute1>>,
	#[validate]
	#[serde(rename = "OptnTp")]
	pub optn_tp: Option<CompareOptionType1>,
	#[validate]
	#[serde(rename = "OptnExrcStyle")]
	pub optn_exrc_style: Option<Vec<CompareOptionStyle1>>,
	#[validate]
	#[serde(rename = "OptnStrkPric")]
	pub optn_strk_pric: Option<CompareUnitPrice4>,
	#[validate]
	#[serde(rename = "OptnStrkPricSchdlUadjstdFctvDt")]
	pub optn_strk_pric_schdl_uadjstd_fctv_dt: Option<Vec<CompareDate3>>,
	#[validate]
	#[serde(rename = "OptnStrkPricSchdlUadjstdEndDt")]
	pub optn_strk_pric_schdl_uadjstd_end_dt: Option<Vec<CompareDate3>>,
	#[validate]
	#[serde(rename = "OptnStrkPricSchdlAmt")]
	pub optn_strk_pric_schdl_amt: Option<Vec<CompareUnitPrice4>>,
	#[validate]
	#[serde(rename = "OptnPrmAmt")]
	pub optn_prm_amt: Option<CompareActiveOrHistoricCurrencyAndAmount4>,
	#[validate]
	#[serde(rename = "OptnPrmPmtDt")]
	pub optn_prm_pmt_dt: Option<CompareDate3>,
	#[validate]
	#[serde(rename = "OptnMtrtyDtOfUndrlyg")]
	pub optn_mtrty_dt_of_undrlyg: Option<CompareDate3>,
	#[validate]
	#[serde(rename = "CdtSnrty")]
	pub cdt_snrty: Option<CompareSeniorityType1>,
	#[validate]
	#[serde(rename = "CdtRefPty")]
	pub cdt_ref_pty: Option<CompareReferenceParty1>,
	#[validate]
	#[serde(rename = "CdtSrs")]
	pub cdt_srs: Option<CompareNumber7>,
	#[validate]
	#[serde(rename = "CdtVrsn")]
	pub cdt_vrsn: Option<CompareNumber7>,
	#[validate]
	#[serde(rename = "CdtIndxFctr")]
	pub cdt_indx_fctr: Option<ComparePercentageRate3>,
	#[validate]
	#[serde(rename = "CdtTrch")]
	pub cdt_trch: Option<CompareTrancheIndicator1>,
	#[validate]
	#[serde(rename = "Lvl")]
	pub lvl: Option<CompareReportingLevelType2>,
}


// TransactionOperationType10Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TransactionOperationType10Code {
	#[validate(enumerate = ["COMP", "CORR", "EROR", "MODI", "NEWT", "OTHR", "POSC", "REVI", "TERM", "VALU", "MARU", "PRTO"])]
	#[serde(rename = "TransactionOperationType10Code")]
	pub transaction_operation_type10_code: String,
}


// TrueFalseIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}


// UTIIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct UTIIdentifier {
	#[validate(pattern = "[A-Z0-9]{18}[0-9]{2}[A-Z0-9]{0,32}")]
	#[serde(rename = "UTIIdentifier")]
	pub uti_identifier: String,
}


// UnderlyingIdentification1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct UnderlyingIdentification1Code {
	#[validate(enumerate = ["UKWN", "BSKT", "INDX"])]
	#[serde(rename = "UnderlyingIdentification1Code")]
	pub underlying_identification1_code: String,
}


// UniqueProductIdentifier1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct UniqueProductIdentifier1Choice {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification175>,
}


// UniqueProductIdentifier2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct UniqueProductIdentifier2Choice {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification185>,
}


// UniqueTransactionIdentifier1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct UniqueTransactionIdentifier1Choice {
	#[serde(rename = "UnqTxIdr")]
	pub unq_tx_idr: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification179>,
}


// UniqueTransactionIdentifier2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct UniqueTransactionIdentifier2Choice {
	#[serde(rename = "UnqTxIdr")]
	pub unq_tx_idr: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification175>,
}


// UnitOfMeasure8Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct UnitOfMeasure8Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification175>,
}


// ValuationMatchingCriteria1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ValuationMatchingCriteria1 {
	#[validate]
	#[serde(rename = "CtrctVal")]
	pub ctrct_val: Option<CompareAmountAndDirection3>,
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: Option<CompareValuationType1>,
}


// ValuationType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ValuationType1Code {
	#[validate(enumerate = ["CCPV", "MTMA", "MTMO"])]
	#[serde(rename = "ValuationType1Code")]
	pub valuation_type1_code: String,
}


// WeekDay3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct WeekDay3Code {
	#[validate(enumerate = ["ALLD", "XBHL", "IBHL", "FRID", "MOND", "SATD", "SUND", "THUD", "TUED", "WEDD", "WDAY", "WEND"])]
	#[serde(rename = "WeekDay3Code")]
	pub week_day3_code: String,
}


// YesNoIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "YesNoIndicator")]
	pub yes_no_indicator: bool,
}
