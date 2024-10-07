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


// AllocationIndicator1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AllocationIndicator1Code {
	#[validate(enumerate = ["POST", "PREA", "UNAL"])]
	#[serde(rename = "AllocationIndicator1Code")]
	pub allocation_indicator1_code: String,
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


// AmountAndDirection109 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AmountAndDirection109 {
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
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


// AssetClassCommodity7Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodity7Choice {
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
	pub ppr: Option<AssetClassCommodityPaper5Choice>,
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


// AssetClassCommodityPaper5Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityPaper5Choice {
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
	pub rcvrd_ppr: Option<PaperCommodityRecoveredPaper3>,
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


// AssetClassSubProductType50Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType50Code {
	#[validate(enumerate = ["OTHR", "RCVP"])]
	#[serde(rename = "AssetClassSubProductType50Code")]
	pub asset_class_sub_product_type50_code: String,
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


// CollateralPortfolioCode6Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CollateralPortfolioCode6Choice {
	#[validate]
	#[serde(rename = "Prtfl")]
	pub prtfl: Option<PortfolioCode3Choice>,
	#[validate]
	#[serde(rename = "MrgnPrtflCd")]
	pub mrgn_prtfl_cd: Option<MarginPortfolio4>,
}


// CommonTradeDataReport71 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CommonTradeDataReport71 {
	#[validate]
	#[serde(rename = "CtrctData")]
	pub ctrct_data: Option<ContractType15>,
	#[validate]
	#[serde(rename = "TxData")]
	pub tx_data: TradeTransaction50,
}


// ContractType15 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ContractType15 {
	#[serde(rename = "CtrctTp")]
	pub ctrct_tp: Option<String>,
	#[serde(rename = "AsstClss")]
	pub asst_clss: Option<String>,
	#[serde(rename = "PdctClssfctn")]
	pub pdct_clssfctn: Option<String>,
	#[validate]
	#[serde(rename = "PdctId")]
	pub pdct_id: Option<SecurityIdentification46>,
	#[validate]
	#[serde(rename = "UndrlygInstrm")]
	pub undrlyg_instrm: Option<SecurityIdentification41Choice>,
	#[serde(rename = "UndrlygAsstTradgPltfmIdr")]
	pub undrlyg_asst_tradg_pltfm_idr: Option<String>,
	#[serde(rename = "UndrlygAsstPricSrc")]
	pub undrlyg_asst_pric_src: Option<String>,
	#[validate]
	#[serde(rename = "SttlmCcy")]
	pub sttlm_ccy: Option<CurrencyExchange23>,
	#[validate]
	#[serde(rename = "SttlmCcyScndLeg")]
	pub sttlm_ccy_scnd_leg: Option<CurrencyExchange23>,
	#[serde(rename = "PlcOfSttlm")]
	pub plc_of_sttlm: Option<String>,
	#[serde(rename = "DerivBasedOnCrptAsst")]
	pub deriv_based_on_crpt_asst: Option<bool>,
}


// ContractValuationData8 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ContractValuationData8 {
	#[validate]
	#[serde(rename = "CtrctVal")]
	pub ctrct_val: Option<AmountAndDirection109>,
	#[serde(rename = "TmStmp")]
	pub tm_stmp: Option<String>,
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "Dlta")]
	pub dlta: Option<f64>,
}


// Counterparty45 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Counterparty45 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: PartyIdentification248Choice,
	#[validate]
	#[serde(rename = "Ntr")]
	pub ntr: Option<CounterpartyTradeNature15Choice>,
	#[serde(rename = "TradgCpcty")]
	pub tradg_cpcty: Option<String>,
	#[validate]
	#[serde(rename = "DrctnOrSd")]
	pub drctn_or_sd: Option<Direction4Choice>,
	#[serde(rename = "TradrLctn")]
	pub tradr_lctn: Option<String>,
	#[serde(rename = "BookgLctn")]
	pub bookg_lctn: Option<String>,
	#[validate]
	#[serde(rename = "RptgXmptn")]
	pub rptg_xmptn: Option<ReportingExemption1>,
}


// Counterparty46 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Counterparty46 {
	#[validate]
	#[serde(rename = "IdTp")]
	pub id_tp: Option<PartyIdentification248Choice>,
	#[validate]
	#[serde(rename = "Ntr")]
	pub ntr: Option<CounterpartyTradeNature15Choice>,
	#[serde(rename = "RptgOblgtn")]
	pub rptg_oblgtn: Option<bool>,
}


// CounterpartySpecificData36 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CounterpartySpecificData36 {
	#[validate]
	#[serde(rename = "CtrPty")]
	pub ctr_pty: TradeCounterpartyReport20,
	#[validate]
	#[serde(rename = "Valtn")]
	pub valtn: Option<ContractValuationData8>,
	#[serde(rename = "RptgTmStmp")]
	pub rptg_tm_stmp: Option<String>,
}


// CounterpartyTradeNature15Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CounterpartyTradeNature15Choice {
	#[validate]
	#[serde(rename = "FI")]
	pub fi: Option<FinancialInstitutionSector1>,
	#[validate]
	#[serde(rename = "NFI")]
	pub nfi: Option<NonFinancialInstitutionSector10>,
	#[serde(rename = "CntrlCntrPty")]
	pub cntrl_cntr_pty: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<String>,
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


// CreditDerivative4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CreditDerivative4 {
	#[serde(rename = "Snrty")]
	pub snrty: Option<String>,
	#[validate]
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
	#[validate]
	#[serde(rename = "Trch")]
	pub trch: Option<TrancheIndicator3Choice>,
}


// CurrencyExchange22 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CurrencyExchange22 {
	#[serde(rename = "DlvrblCrossCcy")]
	pub dlvrbl_cross_ccy: Option<String>,
	#[serde(rename = "XchgRate")]
	pub xchg_rate: Option<f64>,
	#[serde(rename = "FwdXchgRate")]
	pub fwd_xchg_rate: Option<f64>,
	#[validate]
	#[serde(rename = "XchgRateBsis")]
	pub xchg_rate_bsis: Option<ExchangeRateBasis1Choice>,
	#[serde(rename = "FxgDt")]
	pub fxg_dt: Option<String>,
}


// CurrencyExchange23 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CurrencyExchange23 {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "XchgRate")]
	pub xchg_rate: Option<f64>,
	#[serde(rename = "FwdXchgRate")]
	pub fwd_xchg_rate: Option<f64>,
	#[validate]
	#[serde(rename = "XchgRateBsis")]
	pub xchg_rate_bsis: Option<ExchangeRateBasis1Choice>,
	#[serde(rename = "FxgDt")]
	pub fxg_dt: Option<String>,
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


// DatePeriod1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DatePeriod1 {
	#[serde(rename = "FrDt")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
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


// DerivativesTradeReportV04 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DerivativesTradeReportV04 {
	#[validate]
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: TradeReportHeader4,
	#[validate]
	#[serde(rename = "TradData")]
	pub trad_data: TradeData59Choice,
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


// DisseminationData1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DisseminationData1 {
	#[serde(rename = "DssmntnIdr")]
	pub dssmntn_idr: String,
	#[serde(rename = "OrgnlDssmntnIdr")]
	pub orgnl_dssmntn_idr: Option<String>,
	#[serde(rename = "TmStmp")]
	pub tm_stmp: String,
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


// EmbeddedType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EmbeddedType1Code {
	#[validate(enumerate = ["CANC", "EXTD", "OPET", "OTHR", "MDET"])]
	#[serde(rename = "EmbeddedType1Code")]
	pub embedded_type1_code: String,
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


// EnergyDeliveryAttribute10 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnergyDeliveryAttribute10 {
	#[validate]
	#[serde(rename = "DlvryIntrvl")]
	pub dlvry_intrvl: Option<Vec<TimePeriodDetails1>>,
	#[validate]
	#[serde(rename = "DlvryDt")]
	pub dlvry_dt: Option<DatePeriod1>,
	#[serde(rename = "Drtn")]
	pub drtn: Option<String>,
	#[serde(rename = "WkDay")]
	pub wk_day: Option<Vec<String>>,
	#[validate]
	#[serde(rename = "DlvryCpcty")]
	pub dlvry_cpcty: Option<Quantity47Choice>,
	#[validate]
	#[serde(rename = "QtyUnit")]
	pub qty_unit: Option<EnergyQuantityUnit2Choice>,
	#[validate]
	#[serde(rename = "PricTmIntrvlQty")]
	pub pric_tm_intrvl_qty: Option<AmountAndDirection106>,
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


// EnergySpecificAttribute9 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnergySpecificAttribute9 {
	#[validate]
	#[serde(rename = "DlvryPtOrZone")]
	pub dlvry_pt_or_zone: Option<Vec<DeliveryInterconnectionPoint1Choice>>,
	#[validate]
	#[serde(rename = "IntrCnnctnPt")]
	pub intr_cnnctn_pt: Option<DeliveryInterconnectionPoint1Choice>,
	#[serde(rename = "LdTp")]
	pub ld_tp: Option<String>,
	#[validate]
	#[serde(rename = "DlvryAttr")]
	pub dlvry_attr: Option<Vec<EnergyDeliveryAttribute10>>,
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


// ExerciseDate1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExerciseDate1Choice {
	#[serde(rename = "FrstExrcDt")]
	pub frst_exrc_dt: Option<String>,
	#[serde(rename = "PdgDtAplbl")]
	pub pdg_dt_aplbl: Option<String>,
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


// ExternalPartyRelationshipType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalPartyRelationshipType1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalPartyRelationshipType1Code")]
	pub external_party_relationship_type1_code: String,
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


// FinancialInstitutionSector1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstitutionSector1 {
	#[validate]
	#[serde(rename = "Sctr")]
	pub sctr: Vec<FinancialPartyClassification2Choice>,
	#[serde(rename = "ClrThrshld")]
	pub clr_thrshld: Option<bool>,
}


// FinancialInstrumentContractType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstrumentContractType2Code {
	#[validate(enumerate = ["CFDS", "FRAS", "FUTR", "FORW", "OPTN", "SPDB", "SWAP", "SWPT", "OTHR"])]
	#[serde(rename = "FinancialInstrumentContractType2Code")]
	pub financial_instrument_contract_type2_code: String,
}


// FinancialInstrumentQuantity32Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstrumentQuantity32Choice {
	#[serde(rename = "Unit")]
	pub unit: Option<f64>,
	#[validate]
	#[serde(rename = "NmnlVal")]
	pub nmnl_val: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
	#[validate]
	#[serde(rename = "MntryVal")]
	pub mntry_val: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
}


// FinancialPartyClassification2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialPartyClassification2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification175>,
}


// FinancialPartySectorType3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialPartySectorType3Code {
	#[validate(enumerate = ["AIFD", "CSDS", "CCPS", "CDTI", "INUN", "ORPI", "INVF", "REIN", "UCIT", "ASSU", "OTHR"])]
	#[serde(rename = "FinancialPartySectorType3Code")]
	pub financial_party_sector_type3_code: String,
}


// FixedRate10 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FixedRate10 {
	#[validate]
	#[serde(rename = "Rate")]
	pub rate: Option<SecuritiesTransactionPrice14Choice>,
	#[validate]
	#[serde(rename = "DayCnt")]
	pub day_cnt: Option<InterestComputationMethodFormat7>,
	#[validate]
	#[serde(rename = "PmtFrqcy")]
	pub pmt_frqcy: Option<InterestRateFrequency3Choice>,
}


// FloatingRate13 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FloatingRate13 {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[validate]
	#[serde(rename = "Rate")]
	pub rate: Option<FloatingRateIdentification8Choice>,
	#[validate]
	#[serde(rename = "RefPrd")]
	pub ref_prd: Option<InterestRateContractTerm4>,
	#[validate]
	#[serde(rename = "Sprd")]
	pub sprd: Option<SecuritiesTransactionPrice20Choice>,
	#[validate]
	#[serde(rename = "DayCnt")]
	pub day_cnt: Option<InterestComputationMethodFormat7>,
	#[validate]
	#[serde(rename = "PmtFrqcy")]
	pub pmt_frqcy: Option<InterestRateFrequency3Choice>,
	#[validate]
	#[serde(rename = "RstFrqcy")]
	pub rst_frqcy: Option<InterestRateFrequency3Choice>,
	#[validate]
	#[serde(rename = "NxtFltgRst")]
	pub nxt_fltg_rst: Option<ResetDateAndValue1>,
	#[validate]
	#[serde(rename = "LastFltgRst")]
	pub last_fltg_rst: Option<ResetDateAndValue1>,
}


// FloatingRateIdentification8Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FloatingRateIdentification8Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
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


// Frequency19Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Frequency19Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[validate(enumerate = ["DAIL", "WEEK", "MNTH", "YEAR", "ADHO", "EXPI", "MIAN", "QURT", "HOUL", "ODMD"])]
	#[serde(rename = "Frequency19Code")]
	pub frequency19_code: String,
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


// InterestRate33Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InterestRate33Choice {
	#[validate]
	#[serde(rename = "Fxd")]
	pub fxd: Option<FixedRate10>,
	#[validate]
	#[serde(rename = "Fltg")]
	pub fltg: Option<FloatingRate13>,
}


// InterestRateContractTerm4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InterestRateContractTerm4 {
	#[serde(rename = "Unit")]
	pub unit: Option<String>,
	#[serde(rename = "Val")]
	pub val: Option<f64>,
}


// InterestRateFrequency3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InterestRateFrequency3Choice {
	#[validate]
	#[serde(rename = "Term")]
	pub term: Option<InterestRateContractTerm4>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// InterestRateLegs14 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InterestRateLegs14 {
	#[validate]
	#[serde(rename = "FrstLeg")]
	pub frst_leg: Option<InterestRate33Choice>,
	#[validate]
	#[serde(rename = "ScndLeg")]
	pub scnd_leg: Option<InterestRate33Choice>,
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


// MarginPortfolio4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MarginPortfolio4 {
	#[validate]
	#[serde(rename = "InitlMrgnPrtflCd")]
	pub initl_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
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


// Max4Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max4Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "Max4Text")]
	pub max4_text: String,
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


// Max5NumericText ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max5NumericText {
	#[validate(pattern = "[0-9]{1,5}")]
	#[serde(rename = "Max5NumericText")]
	pub max5_numeric_text: String,
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


// NonFinancialInstitutionSector10 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NonFinancialInstitutionSector10 {
	#[validate]
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
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NotApplicable1Code {
	#[validate(max_length = 4)]
	#[validate(enumerate = ["NOAP"])]
	#[serde(rename = "NotApplicable1Code")]
	pub not_applicable1_code: String,
}


// NotionalAmount5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NotionalAmount5 {
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: Option<AmountAndDirection106>,
	#[validate]
	#[serde(rename = "SchdlPrd")]
	pub schdl_prd: Option<Vec<Schedule11>>,
}


// NotionalAmount6 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NotionalAmount6 {
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: Option<AmountAndDirection106>,
	#[validate]
	#[serde(rename = "SchdlPrd")]
	pub schdl_prd: Option<Vec<Schedule11>>,
	#[serde(rename = "Ccy")]
	pub ccy: Option<String>,
}


// NotionalAmountLegs5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NotionalAmountLegs5 {
	#[validate]
	#[serde(rename = "FrstLeg")]
	pub frst_leg: Option<NotionalAmount5>,
	#[validate]
	#[serde(rename = "ScndLeg")]
	pub scnd_leg: Option<NotionalAmount6>,
}


// NotionalQuantity9 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NotionalQuantity9 {
	#[serde(rename = "TtlQty")]
	pub ttl_qty: Option<f64>,
	#[validate]
	#[serde(rename = "UnitOfMeasr")]
	pub unit_of_measr: Option<UnitOfMeasure8Choice>,
	#[validate]
	#[serde(rename = "Dtls")]
	pub dtls: Option<QuantityOrTerm1Choice>,
}


// NotionalQuantityLegs5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NotionalQuantityLegs5 {
	#[validate]
	#[serde(rename = "FrstLeg")]
	pub frst_leg: Option<NotionalQuantity9>,
	#[validate]
	#[serde(rename = "ScndLeg")]
	pub scnd_leg: Option<NotionalQuantity9>,
}


// Number ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
}


// OptionBarrierLevel1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OptionBarrierLevel1Choice {
	#[validate]
	#[serde(rename = "Sngl")]
	pub sngl: Option<SecuritiesTransactionPrice23Choice>,
	#[validate]
	#[serde(rename = "Mltpl")]
	pub mltpl: Option<OptionMultipleBarrierLevels1>,
}


// OptionMultipleBarrierLevels1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OptionMultipleBarrierLevels1 {
	#[validate]
	#[serde(rename = "LwrLvl")]
	pub lwr_lvl: SecuritiesTransactionPrice23Choice,
	#[validate]
	#[serde(rename = "UpperLvl")]
	pub upper_lvl: SecuritiesTransactionPrice23Choice,
}


// OptionOrSwaption11 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OptionOrSwaption11 {
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "MbddTp")]
	pub mbdd_tp: Option<String>,
	#[serde(rename = "ExrcStyle")]
	pub exrc_style: Option<Vec<String>>,
	#[validate]
	#[serde(rename = "ExrcDt")]
	pub exrc_dt: Option<ExerciseDate1Choice>,
	#[validate]
	#[serde(rename = "StrkPric")]
	pub strk_pric: Option<SecuritiesTransactionPrice17Choice>,
	#[validate]
	#[serde(rename = "StrkPricSchdl")]
	pub strk_pric_schdl: Option<Vec<Schedule4>>,
	#[validate]
	#[serde(rename = "CallAmt")]
	pub call_amt: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
	#[validate]
	#[serde(rename = "PutAmt")]
	pub put_amt: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
	#[validate]
	#[serde(rename = "PrmAmt")]
	pub prm_amt: Option<ActiveOrHistoricCurrencyAnd19DecimalAmount>,
	#[serde(rename = "PrmPmtDt")]
	pub prm_pmt_dt: Option<String>,
	#[serde(rename = "MtrtyDtOfUndrlyg")]
	pub mtrty_dt_of_undrlyg: Option<String>,
	#[validate]
	#[serde(rename = "BrrrLvls")]
	pub brrr_lvls: Option<OptionBarrierLevel1Choice>,
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


// OtherPayment5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OtherPayment5 {
	#[validate]
	#[serde(rename = "PmtAmt")]
	pub pmt_amt: Option<AmountAndDirection106>,
	#[validate]
	#[serde(rename = "PmtTp")]
	pub pmt_tp: Option<PaymentType5Choice>,
	#[serde(rename = "PmtDt")]
	pub pmt_dt: Option<String>,
	#[validate]
	#[serde(rename = "PmtPyer")]
	pub pmt_pyer: Option<PartyIdentification236Choice>,
	#[validate]
	#[serde(rename = "PmtRcvr")]
	pub pmt_rcvr: Option<PartyIdentification236Choice>,
}


// PTRREvent2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PTRREvent2 {
	#[serde(rename = "Tchnq")]
	pub tchnq: String,
	#[validate]
	#[serde(rename = "SvcPrvdr")]
	pub svc_prvdr: Option<OrganisationIdentification15Choice>,
}


// Package4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Package4 {
	#[serde(rename = "CmplxTradId")]
	pub cmplx_trad_id: Option<String>,
	#[serde(rename = "FxSwpLkId")]
	pub fx_swp_lk_id: Option<String>,
	#[validate]
	#[serde(rename = "Pric")]
	pub pric: Option<SecuritiesTransactionPrice17Choice>,
	#[validate]
	#[serde(rename = "Sprd")]
	pub sprd: Option<SecuritiesTransactionPrice20Choice>,
}


// Pagination1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Pagination1 {
	#[serde(rename = "PgNb")]
	pub pg_nb: String,
	#[serde(rename = "LastPgInd")]
	pub last_pg_ind: bool,
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


// PaperCommodityRecoveredPaper3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PaperCommodityRecoveredPaper3 {
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


// PriceData2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PriceData2 {
	#[validate]
	#[serde(rename = "Pric")]
	pub pric: Option<SecuritiesTransactionPrice17Choice>,
	#[validate]
	#[serde(rename = "SchdlPrd")]
	pub schdl_prd: Option<Vec<Schedule1>>,
	#[validate]
	#[serde(rename = "UnitOfMeasr")]
	pub unit_of_measr: Option<UnitOfMeasure8Choice>,
	#[serde(rename = "PricMltplr")]
	pub pric_mltplr: Option<f64>,
}


// PriceStatus1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PriceStatus1Code {
	#[validate(enumerate = ["PNDG", "NOAP"])]
	#[serde(rename = "PriceStatus1Code")]
	pub price_status1_code: String,
}


// PriceStatus2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PriceStatus2Code {
	#[validate(enumerate = ["PNDG"])]
	#[serde(rename = "PriceStatus2Code")]
	pub price_status2_code: String,
}


// ProductType4Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ProductType4Code {
	#[validate(enumerate = ["CRDT", "CURR", "EQUI", "INTR", "COMM", "OTHR"])]
	#[serde(rename = "ProductType4Code")]
	pub product_type4_code: String,
}


// Quantity47Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Quantity47Choice {
	#[serde(rename = "Qty")]
	pub qty: Option<f64>,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// QuantityOrTerm1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct QuantityOrTerm1Choice {
	#[validate]
	#[serde(rename = "SchdlPrd")]
	pub schdl_prd: Option<Vec<Schedule10>>,
	#[validate]
	#[serde(rename = "Term")]
	pub term: Option<QuantityTerm1>,
}


// QuantityTerm1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct QuantityTerm1 {
	#[serde(rename = "Qty")]
	pub qty: Option<f64>,
	#[validate]
	#[serde(rename = "UnitOfMeasr")]
	pub unit_of_measr: Option<UnitOfMeasure8Choice>,
	#[serde(rename = "Val")]
	pub val: Option<f64>,
	#[serde(rename = "TmUnit")]
	pub tm_unit: Option<String>,
}


// Reconciliation3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Reconciliation3Code {
	#[validate(enumerate = ["DPRW", "DPRV", "DSMA", "DSNM", "NORE", "SSMA", "SSPA", "SPRW", "SPRV", "SSUN", "SSNE"])]
	#[serde(rename = "Reconciliation3Code")]
	pub reconciliation3_code: String,
}


// ReportPeriodActivity1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReportPeriodActivity1Code {
	#[validate(enumerate = ["NOTX"])]
	#[serde(rename = "ReportPeriodActivity1Code")]
	pub report_period_activity1_code: String,
}


// ReportingExemption1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReportingExemption1 {
	#[serde(rename = "Rsn")]
	pub rsn: String,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// ResetDateAndValue1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ResetDateAndValue1 {
	#[serde(rename = "Dt")]
	pub dt: String,
	#[serde(rename = "Val")]
	pub val: Option<f64>,
}


// RiskReductionService1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RiskReductionService1Code {
	#[validate(enumerate = ["NORR", "PWOS", "OTHR", "PRBM", "PWAS"])]
	#[serde(rename = "RiskReductionService1Code")]
	pub risk_reduction_service1_code: String,
}


// Schedule1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Schedule1 {
	#[serde(rename = "UadjstdFctvDt")]
	pub uadjstd_fctv_dt: String,
	#[serde(rename = "UadjstdEndDt")]
	pub uadjstd_end_dt: Option<String>,
	#[validate]
	#[serde(rename = "Pric")]
	pub pric: SecuritiesTransactionPrice17Choice,
}


// Schedule10 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Schedule10 {
	#[serde(rename = "Qty")]
	pub qty: f64,
	#[validate]
	#[serde(rename = "UnitOfMeasr")]
	pub unit_of_measr: Option<UnitOfMeasure8Choice>,
	#[serde(rename = "UadjstdFctvDt")]
	pub uadjstd_fctv_dt: String,
	#[serde(rename = "UadjstdEndDt")]
	pub uadjstd_end_dt: Option<String>,
}


// Schedule11 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Schedule11 {
	#[serde(rename = "UadjstdFctvDt")]
	pub uadjstd_fctv_dt: String,
	#[serde(rename = "UadjstdEndDt")]
	pub uadjstd_end_dt: Option<String>,
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: AmountAndDirection106,
}


// Schedule4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Schedule4 {
	#[serde(rename = "UadjstdFctvDt")]
	pub uadjstd_fctv_dt: String,
	#[serde(rename = "UadjstdEndDt")]
	pub uadjstd_end_dt: Option<String>,
	#[validate]
	#[serde(rename = "Pric")]
	pub pric: SecuritiesTransactionPrice17Choice,
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


// SecuritiesTransactionPrice20Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice20Choice {
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


// SecuritiesTransactionPrice23Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice23Choice {
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


// SecurityIdentification46 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityIdentification46 {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[validate]
	#[serde(rename = "UnqPdctIdr")]
	pub unq_pdct_idr: Option<UniqueProductIdentifier2Choice>,
	#[serde(rename = "AltrntvInstrmId")]
	pub altrntv_instrm_id: Option<String>,
	#[serde(rename = "PdctDesc")]
	pub pdct_desc: Option<String>,
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


// TechnicalAttributes5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TechnicalAttributes5 {
	#[serde(rename = "TechRcrdId")]
	pub tech_rcrd_id: Option<String>,
	#[serde(rename = "RcncltnFlg")]
	pub rcncltn_flg: Option<String>,
	#[serde(rename = "RptRctTmStmp")]
	pub rpt_rct_tm_stmp: Option<String>,
}


// TimePeriodDetails1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TimePeriodDetails1 {
	#[serde(rename = "FrTm")]
	pub fr_tm: String,
	#[serde(rename = "ToTm")]
	pub to_tm: Option<String>,
}


// TradeClearing11 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeClearing11 {
	#[serde(rename = "ClrOblgtn")]
	pub clr_oblgtn: Option<String>,
	#[validate]
	#[serde(rename = "ClrSts")]
	pub clr_sts: Option<Cleared23Choice>,
	#[serde(rename = "IntraGrp")]
	pub intra_grp: Option<bool>,
}


// TradeConfirmation4Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeConfirmation4Choice {
	#[validate]
	#[serde(rename = "Confd")]
	pub confd: Option<TradeConfirmation5>,
	#[validate]
	#[serde(rename = "NonConfd")]
	pub non_confd: Option<TradeNonConfirmation1>,
}


// TradeConfirmation5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeConfirmation5 {
	#[serde(rename = "Tp")]
	pub tp: String,
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


// TradeCounterpartyRelationship1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeCounterpartyRelationship1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// TradeCounterpartyRelationshipRecord1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeCounterpartyRelationshipRecord1 {
	#[serde(rename = "StartRltshPty")]
	pub start_rltsh_pty: String,
	#[serde(rename = "EndRltshPty")]
	pub end_rltsh_pty: String,
	#[validate]
	#[serde(rename = "RltshTp")]
	pub rltsh_tp: TradeCounterpartyRelationship1Choice,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// TradeCounterpartyReport20 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeCounterpartyReport20 {
	#[validate]
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: Counterparty45,
	#[validate]
	#[serde(rename = "OthrCtrPty")]
	pub othr_ctr_pty: Counterparty46,
	#[validate]
	#[serde(rename = "Brkr")]
	pub brkr: Option<OrganisationIdentification15Choice>,
	#[validate]
	#[serde(rename = "SubmitgAgt")]
	pub submitg_agt: Option<OrganisationIdentification15Choice>,
	#[validate]
	#[serde(rename = "ClrMmb")]
	pub clr_mmb: Option<PartyIdentification248Choice>,
	#[validate]
	#[serde(rename = "Bnfcry")]
	pub bnfcry: Option<Vec<PartyIdentification248Choice>>,
	#[validate]
	#[serde(rename = "NttyRspnsblForRpt")]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
	#[validate]
	#[serde(rename = "ExctnAgt")]
	pub exctn_agt: Option<Vec<OrganisationIdentification15Choice>>,
	#[validate]
	#[serde(rename = "RltshRcrd")]
	pub rltsh_rcrd: Option<Vec<TradeCounterpartyRelationshipRecord1>>,
}


// TradeCounterpartyType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeCounterpartyType1Code {
	#[validate(enumerate = ["BENE", "BROK", "CLEM", "EXEA", "OTHC", "REPC", "SBMA", "ERFR"])]
	#[serde(rename = "TradeCounterpartyType1Code")]
	pub trade_counterparty_type1_code: String,
}


// TradeData43 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeData43 {
	#[validate]
	#[serde(rename = "CtrPtySpcfcData")]
	pub ctr_pty_spcfc_data: Vec<CounterpartySpecificData36>,
	#[validate]
	#[serde(rename = "CmonTradData")]
	pub cmon_trad_data: CommonTradeDataReport71,
	#[serde(rename = "Lvl")]
	pub lvl: Option<String>,
	#[validate]
	#[serde(rename = "TechAttrbts")]
	pub tech_attrbts: Option<TechnicalAttributes5>,
	#[validate]
	#[serde(rename = "PblcDssmntnData")]
	pub pblc_dssmntn_data: Option<DisseminationData1>,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// TradeData59Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeData59Choice {
	#[serde(rename = "DataSetActn")]
	pub data_set_actn: Option<String>,
	#[validate]
	#[serde(rename = "Rpt")]
	pub rpt: Option<Vec<TradeReport33Choice>>,
}


// TradeNonConfirmation1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeNonConfirmation1 {
	#[serde(rename = "Tp")]
	pub tp: String,
}


// TradeReport33Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeReport33Choice {
	#[validate]
	#[serde(rename = "New")]
	pub new: Option<TradeData43>,
	#[validate]
	#[serde(rename = "Mod")]
	pub mod_attr: Option<TradeData43>,
	#[validate]
	#[serde(rename = "Crrctn")]
	pub crrctn: Option<TradeData43>,
	#[validate]
	#[serde(rename = "Termntn")]
	pub termntn: Option<TradeData43>,
	#[validate]
	#[serde(rename = "PosCmpnt")]
	pub pos_cmpnt: Option<TradeData43>,
	#[validate]
	#[serde(rename = "ValtnUpd")]
	pub valtn_upd: Option<TradeData43>,
	#[validate]
	#[serde(rename = "Cmprssn")]
	pub cmprssn: Option<TradeData43>,
	#[validate]
	#[serde(rename = "Err")]
	pub err: Option<TradeData43>,
	#[validate]
	#[serde(rename = "PortOut")]
	pub port_out: Option<TradeData43>,
	#[validate]
	#[serde(rename = "Rvv")]
	pub rvv: Option<TradeData43>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<TradeData43>,
}


// TradeReportHeader4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeReportHeader4 {
	#[serde(rename = "RptExctnDt")]
	pub rpt_exctn_dt: Option<String>,
	#[validate]
	#[serde(rename = "MsgPgntn")]
	pub msg_pgntn: Option<Pagination1>,
	#[serde(rename = "NbRcrds")]
	pub nb_rcrds: f64,
	#[serde(rename = "CmptntAuthrty")]
	pub cmptnt_authrty: Option<Vec<String>>,
	#[validate]
	#[serde(rename = "NewTradRpstryIdr")]
	pub new_trad_rpstry_idr: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "RptgPurp")]
	pub rptg_purp: Option<Vec<String>>,
}


// TradeTransaction50 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeTransaction50 {
	#[validate]
	#[serde(rename = "TxId")]
	pub tx_id: Option<UniqueTransactionIdentifier2Choice>,
	#[serde(rename = "ScndryTxId")]
	pub scndry_tx_id: Option<String>,
	#[validate]
	#[serde(rename = "PrrTxId")]
	pub prr_tx_id: Option<UniqueTransactionIdentifier3Choice>,
	#[validate]
	#[serde(rename = "SbsqntTxId")]
	pub sbsqnt_tx_id: Option<UniqueTransactionIdentifier3Choice>,
	#[validate]
	#[serde(rename = "CollPrtflCd")]
	pub coll_prtfl_cd: Option<CollateralPortfolioCode6Choice>,
	#[serde(rename = "RptTrckgNb")]
	pub rpt_trckg_nb: Option<String>,
	#[serde(rename = "PltfmIdr")]
	pub pltfm_idr: Option<String>,
	#[serde(rename = "MrrrOrTrggrTx")]
	pub mrrr_or_trggr_tx: Option<bool>,
	#[validate]
	#[serde(rename = "TxPric")]
	pub tx_pric: Option<PriceData2>,
	#[validate]
	#[serde(rename = "NtnlAmt")]
	pub ntnl_amt: Option<NotionalAmountLegs5>,
	#[validate]
	#[serde(rename = "NtnlQty")]
	pub ntnl_qty: Option<NotionalQuantityLegs5>,
	#[validate]
	#[serde(rename = "Qty")]
	pub qty: Option<FinancialInstrumentQuantity32Choice>,
	#[serde(rename = "DlvryTp")]
	pub dlvry_tp: Option<String>,
	#[serde(rename = "ExctnTmStmp")]
	pub exctn_tm_stmp: Option<String>,
	#[serde(rename = "FctvDt")]
	pub fctv_dt: Option<String>,
	#[serde(rename = "XprtnDt")]
	pub xprtn_dt: Option<String>,
	#[serde(rename = "EarlyTermntnDt")]
	pub early_termntn_dt: Option<String>,
	#[serde(rename = "SttlmDt")]
	pub sttlm_dt: Option<Vec<String>>,
	#[validate]
	#[serde(rename = "MstrAgrmt")]
	pub mstr_agrmt: Option<MasterAgreement8>,
	#[serde(rename = "Cmprssn")]
	pub cmprssn: Option<bool>,
	#[serde(rename = "PstTradRskRdctnFlg")]
	pub pst_trad_rsk_rdctn_flg: Option<bool>,
	#[validate]
	#[serde(rename = "PstTradRskRdctnEvt")]
	pub pst_trad_rsk_rdctn_evt: Option<PTRREvent2>,
	#[validate]
	#[serde(rename = "DerivEvt")]
	pub deriv_evt: Option<DerivativeEvent6>,
	#[validate]
	#[serde(rename = "TradConf")]
	pub trad_conf: Option<TradeConfirmation4Choice>,
	#[serde(rename = "NonStdsdTerm")]
	pub non_stdsd_term: Option<bool>,
	#[validate]
	#[serde(rename = "TradClr")]
	pub trad_clr: Option<TradeClearing11>,
	#[serde(rename = "BlckTradElctn")]
	pub blck_trad_elctn: Option<bool>,
	#[serde(rename = "LrgNtnlOffFcltyElctn")]
	pub lrg_ntnl_off_fclty_elctn: Option<bool>,
	#[validate]
	#[serde(rename = "IntrstRate")]
	pub intrst_rate: Option<InterestRateLegs14>,
	#[validate]
	#[serde(rename = "Ccy")]
	pub ccy: Option<CurrencyExchange22>,
	#[validate]
	#[serde(rename = "Cmmdty")]
	pub cmmdty: Option<AssetClassCommodity7Choice>,
	#[validate]
	#[serde(rename = "Optn")]
	pub optn: Option<OptionOrSwaption11>,
	#[validate]
	#[serde(rename = "NrgySpcfcAttrbts")]
	pub nrgy_spcfc_attrbts: Option<EnergySpecificAttribute9>,
	#[validate]
	#[serde(rename = "Cdt")]
	pub cdt: Option<CreditDerivative4>,
	#[validate]
	#[serde(rename = "OthrPmt")]
	pub othr_pmt: Option<Vec<OtherPayment5>>,
	#[validate]
	#[serde(rename = "Packg")]
	pub packg: Option<Package4>,
	#[serde(rename = "TradAllcnSts")]
	pub trad_allcn_sts: Option<String>,
}


// TradingCapacity7Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradingCapacity7Code {
	#[validate(enumerate = ["AGEN", "PRIN"])]
	#[serde(rename = "TradingCapacity7Code")]
	pub trading_capacity7_code: String,
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


// UniqueTransactionIdentifier3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct UniqueTransactionIdentifier3Choice {
	#[serde(rename = "UnqTxIdr")]
	pub unq_tx_idr: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification175>,
	#[serde(rename = "NotAvlbl")]
	pub not_avlbl: Option<String>,
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
