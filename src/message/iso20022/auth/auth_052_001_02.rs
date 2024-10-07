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


// ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd20DecimalAmountSimpleType {
	#[serde(rename = "ActiveOrHistoricCurrencyAnd20DecimalAmount_SimpleType")]
	pub active_or_historic_currency_and20_decimal_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAnd20DecimalAmount ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd20DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveOrHistoricCurrencyAndAmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
	#[serde(rename = "ActiveOrHistoricCurrencyAndAmount_SimpleType")]
	pub active_or_historic_currency_and_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAndAmount ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmount {
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


// AgriculturalCommodityDairy1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AgriculturalCommodityDairy1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// AgriculturalCommodityForestry1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AgriculturalCommodityForestry1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// AgriculturalCommodityGrain2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AgriculturalCommodityGrain2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: String,
}


// AgriculturalCommodityLiveStock1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AgriculturalCommodityLiveStock1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// AgriculturalCommodityOilSeed1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AgriculturalCommodityOilSeed1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: String,
}


// AgriculturalCommodityOliveOil2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AgriculturalCommodityOliveOil2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: String,
}


// AgriculturalCommodityOther1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AgriculturalCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// AgriculturalCommodityPotato1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AgriculturalCommodityPotato1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// AgriculturalCommoditySeafood1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AgriculturalCommoditySeafood1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// AgriculturalCommoditySoft1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AgriculturalCommoditySoft1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: String,
}


// AmountAndDirection107 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AmountAndDirection107 {
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAnd20DecimalAmount,
	#[serde(rename = "Sgn")]
	pub sgn: Option<bool>,
}


// AmountAndDirection53 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AmountAndDirection53 {
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "Sgn")]
	pub sgn: Option<bool>,
}


// AmountHaircutMargin1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AmountHaircutMargin1 {
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: AmountAndDirection53,
	#[serde(rename = "HrcutOrMrgn")]
	pub hrcut_or_mrgn: Option<f64>,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[validate(pattern = "[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}")]
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// AssetClassCommodity5Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodity5Choice {
	#[validate]
	#[serde(rename = "Agrcltrl")]
	pub agrcltrl: Option<AssetClassCommodityAgricultural5Choice>,
	#[validate]
	#[serde(rename = "Nrgy")]
	pub nrgy: Option<AssetClassCommodityEnergy2Choice>,
	#[validate]
	#[serde(rename = "Envttl")]
	pub envttl: Option<AssetClassCommodityEnvironmental2Choice>,
	#[validate]
	#[serde(rename = "Frtlzr")]
	pub frtlzr: Option<AssetClassCommodityFertilizer3Choice>,
	#[validate]
	#[serde(rename = "Frght")]
	pub frght: Option<AssetClassCommodityFreight3Choice>,
	#[validate]
	#[serde(rename = "IndstrlPdct")]
	pub indstrl_pdct: Option<AssetClassCommodityIndustrialProduct1Choice>,
	#[validate]
	#[serde(rename = "Metl")]
	pub metl: Option<AssetClassCommodityMetal1Choice>,
	#[validate]
	#[serde(rename = "OthrC10")]
	pub othr_c10: Option<AssetClassCommodityOtherC102Choice>,
	#[validate]
	#[serde(rename = "Ppr")]
	pub ppr: Option<AssetClassCommodityPaper3Choice>,
	#[validate]
	#[serde(rename = "Plprpln")]
	pub plprpln: Option<AssetClassCommodityPolypropylene3Choice>,
	#[validate]
	#[serde(rename = "Infltn")]
	pub infltn: Option<AssetClassCommodityInflation1>,
	#[validate]
	#[serde(rename = "MultiCmmdtyExtc")]
	pub multi_cmmdty_extc: Option<AssetClassCommodityMultiCommodityExotic1>,
	#[validate]
	#[serde(rename = "OffclEcnmcSttstcs")]
	pub offcl_ecnmc_sttstcs: Option<AssetClassCommodityOfficialEconomicStatistics1>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<AssetClassCommodityOther1>,
}


// AssetClassCommodityAgricultural5Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityAgricultural5Choice {
	#[validate]
	#[serde(rename = "GrnOilSeed")]
	pub grn_oil_seed: Option<AgriculturalCommodityOilSeed1>,
	#[validate]
	#[serde(rename = "Soft")]
	pub soft: Option<AgriculturalCommoditySoft1>,
	#[validate]
	#[serde(rename = "Ptt")]
	pub ptt: Option<AgriculturalCommodityPotato1>,
	#[validate]
	#[serde(rename = "OlvOil")]
	pub olv_oil: Option<AgriculturalCommodityOliveOil2>,
	#[validate]
	#[serde(rename = "Dairy")]
	pub dairy: Option<AgriculturalCommodityDairy1>,
	#[validate]
	#[serde(rename = "Frstry")]
	pub frstry: Option<AgriculturalCommodityForestry1>,
	#[validate]
	#[serde(rename = "Sfd")]
	pub sfd: Option<AgriculturalCommoditySeafood1>,
	#[validate]
	#[serde(rename = "LiveStock")]
	pub live_stock: Option<AgriculturalCommodityLiveStock1>,
	#[validate]
	#[serde(rename = "Grn")]
	pub grn: Option<AgriculturalCommodityGrain2>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<AgriculturalCommodityOther1>,
}


// AssetClassCommodityEnergy2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityEnergy2Choice {
	#[validate]
	#[serde(rename = "Elctrcty")]
	pub elctrcty: Option<EnergyCommodityElectricity1>,
	#[validate]
	#[serde(rename = "NtrlGas")]
	pub ntrl_gas: Option<EnergyCommodityNaturalGas2>,
	#[validate]
	#[serde(rename = "Oil")]
	pub oil: Option<EnergyCommodityOil2>,
	#[validate]
	#[serde(rename = "Coal")]
	pub coal: Option<EnergyCommodityCoal1>,
	#[validate]
	#[serde(rename = "IntrNrgy")]
	pub intr_nrgy: Option<EnergyCommodityInterEnergy1>,
	#[validate]
	#[serde(rename = "RnwblNrgy")]
	pub rnwbl_nrgy: Option<EnergyCommodityRenewableEnergy1>,
	#[validate]
	#[serde(rename = "LghtEnd")]
	pub lght_end: Option<EnergyCommodityLightEnd1>,
	#[validate]
	#[serde(rename = "Dstllts")]
	pub dstllts: Option<EnergyCommodityDistillates1>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<EnergyCommodityOther1>,
}


// AssetClassCommodityEnvironmental2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityEnvironmental2Choice {
	#[validate]
	#[serde(rename = "Emssns")]
	pub emssns: Option<EnvironmentalCommodityEmission2>,
	#[validate]
	#[serde(rename = "Wthr")]
	pub wthr: Option<EnvironmentalCommodityWeather1>,
	#[validate]
	#[serde(rename = "CrbnRltd")]
	pub crbn_rltd: Option<EnvironmentalCommodityCarbonRelated1>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<EnvironmentCommodityOther1>,
}


// AssetClassCommodityFertilizer3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityFertilizer3Choice {
	#[validate]
	#[serde(rename = "Ammn")]
	pub ammn: Option<FertilizerCommodityAmmonia1>,
	#[validate]
	#[serde(rename = "DmmnmPhspht")]
	pub dmmnm_phspht: Option<FertilizerCommodityDiammoniumPhosphate1>,
	#[validate]
	#[serde(rename = "Ptsh")]
	pub ptsh: Option<FertilizerCommodityPotash1>,
	#[validate]
	#[serde(rename = "Slphr")]
	pub slphr: Option<FertilizerCommoditySulphur1>,
	#[validate]
	#[serde(rename = "Urea")]
	pub urea: Option<FertilizerCommodityUrea1>,
	#[validate]
	#[serde(rename = "UreaAndAmmnmNtrt")]
	pub urea_and_ammnm_ntrt: Option<FertilizerCommodityUreaAndAmmoniumNitrate1>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<FertilizerCommodityOther1>,
}


// AssetClassCommodityFreight3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityFreight3Choice {
	#[validate]
	#[serde(rename = "Dry")]
	pub dry: Option<FreightCommodityDry2>,
	#[validate]
	#[serde(rename = "Wet")]
	pub wet: Option<FreightCommodityWet2>,
	#[validate]
	#[serde(rename = "CntnrShip")]
	pub cntnr_ship: Option<FreightCommodityContainerShip1>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<FreightCommodityOther1>,
}


// AssetClassCommodityIndustrialProduct1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityIndustrialProduct1Choice {
	#[validate]
	#[serde(rename = "Cnstrctn")]
	pub cnstrctn: Option<IndustrialProductCommodityConstruction1>,
	#[validate]
	#[serde(rename = "Manfctg")]
	pub manfctg: Option<IndustrialProductCommodityManufacturing1>,
}


// AssetClassCommodityInflation1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityInflation1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
}


// AssetClassCommodityMetal1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityMetal1Choice {
	#[validate]
	#[serde(rename = "NonPrcs")]
	pub non_prcs: Option<MetalCommodityNonPrecious1>,
	#[validate]
	#[serde(rename = "Prcs")]
	pub prcs: Option<MetalCommodityPrecious1>,
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


// AssetClassCommodityOtherC102Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityOtherC102Choice {
	#[validate]
	#[serde(rename = "Dlvrbl")]
	pub dlvrbl: Option<OtherC10CommodityDeliverable2>,
	#[validate]
	#[serde(rename = "NonDlvrbl")]
	pub non_dlvrbl: Option<OtherC10CommodityNonDeliverable2>,
}


// AssetClassCommodityPaper3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityPaper3Choice {
	#[validate]
	#[serde(rename = "CntnrBrd")]
	pub cntnr_brd: Option<PaperCommodityContainerBoard1>,
	#[validate]
	#[serde(rename = "Nwsprnt")]
	pub nwsprnt: Option<PaperCommodityNewsprint1>,
	#[validate]
	#[serde(rename = "Pulp")]
	pub pulp: Option<PaperCommodityPulp1>,
	#[validate]
	#[serde(rename = "RcvrdPpr")]
	pub rcvrd_ppr: Option<PaperCommodityRecoveredPaper1>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<PaperCommodityRecoveredPaper2>,
}


// AssetClassCommodityPolypropylene3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityPolypropylene3Choice {
	#[validate]
	#[serde(rename = "Plstc")]
	pub plstc: Option<PolypropyleneCommodityPlastic1>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<PolypropyleneCommodityOther1>,
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


// AssetClassSubProductType38Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType38Code {
	#[validate(enumerate = ["RCVP"])]
	#[serde(rename = "AssetClassSubProductType38Code")]
	pub asset_class_sub_product_type38_code: String,
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


// AssetClassSubProductType47Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType47Code {
	#[validate(enumerate = ["DLVR"])]
	#[serde(rename = "AssetClassSubProductType47Code")]
	pub asset_class_sub_product_type47_code: String,
}


// AssetClassSubProductType48Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassSubProductType48Code {
	#[validate(enumerate = ["NDLV"])]
	#[serde(rename = "AssetClassSubProductType48Code")]
	pub asset_class_sub_product_type48_code: String,
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


// BaseOneRate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BaseOneRate {
	#[serde(rename = "BaseOneRate")]
	pub base_one_rate: f64,
}


// BenchmarkCurveName10Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BenchmarkCurveName10Choice {
	#[serde(rename = "Indx")]
	pub indx: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
}


// BenchmarkCurveName3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BenchmarkCurveName3Code {
	#[validate(enumerate = ["ESTR", "BBSW", "BUBO", "CDOR", "CIBO", "EONA", "EONS", "EURI", "EUUS", "EUCH", "FUSW", "GCFR", "ISDA", "JIBA", "LIBI", "LIBO", "MOSP", "MAAA", "NIBO", "PFAN", "PRBO", "STBO", "SWAP", "TLBO", "TIBO", "TREA", "WIBO", "SOFR", "SONA"])]
	#[serde(rename = "BenchmarkCurveName3Code")]
	pub benchmark_curve_name3_code: String,
}


// Branch5Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Branch5Choice {
	#[validate]
	#[serde(rename = "Id")]
	pub id: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
}


// Branch6Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Branch6Choice {
	#[validate]
	#[serde(rename = "Id")]
	pub id: Option<PartyIdentification236Choice>,
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
}


// CFIOct2015Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[validate(pattern = "[A-Z]{6,6}")]
	#[serde(rename = "CFIOct2015Identifier")]
	pub cfi_oct2015_identifier: String,
}


// Cleared16Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Cleared16Choice {
	#[validate]
	#[serde(rename = "Clrd")]
	pub clrd: Option<ClearingPartyAndTime14>,
	#[serde(rename = "NonClrd")]
	pub non_clrd: Option<String>,
}


// ClearingPartyAndTime14 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ClearingPartyAndTime14 {
	#[validate]
	#[serde(rename = "CCP")]
	pub ccp: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "ClrDtTm")]
	pub clr_dt_tm: Option<String>,
	#[serde(rename = "RptTrckgNb")]
	pub rpt_trckg_nb: Option<String>,
	#[serde(rename = "PrtflCd")]
	pub prtfl_cd: Option<String>,
}


// Collateral52 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Collateral52 {
	#[serde(rename = "CollValDt")]
	pub coll_val_dt: Option<String>,
	#[validate]
	#[serde(rename = "AsstTp")]
	pub asst_tp: Option<CollateralType21>,
	#[serde(rename = "NetXpsrCollstnInd")]
	pub net_xpsr_collstn_ind: Option<bool>,
	#[validate]
	#[serde(rename = "BsktIdr")]
	pub bskt_idr: Option<SecurityIdentification26Choice>,
}


// CollateralData35 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CollateralData35 {
	#[validate]
	#[serde(rename = "AsstTp")]
	pub asst_tp: Option<CollateralType21>,
	#[serde(rename = "NetXpsrCollstnInd")]
	pub net_xpsr_collstn_ind: Option<bool>,
	#[validate]
	#[serde(rename = "BsktIdr")]
	pub bskt_idr: Option<SecurityIdentification26Choice>,
}


// CollateralDeliveryMethod1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CollateralDeliveryMethod1Code {
	#[validate(enumerate = ["SICA", "SIUR", "TTCA"])]
	#[serde(rename = "CollateralDeliveryMethod1Code")]
	pub collateral_delivery_method1_code: String,
}


// CollateralFlag13Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CollateralFlag13Choice {
	#[validate]
	#[serde(rename = "Collsd")]
	pub collsd: Option<CollaterisedData12>,
	#[serde(rename = "Uncollsd")]
	pub uncollsd: Option<String>,
}


// CollateralQualityType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CollateralQualityType1Code {
	#[validate(enumerate = ["INVG", "NIVG", "NOTR", "NOAP"])]
	#[serde(rename = "CollateralQualityType1Code")]
	pub collateral_quality_type1_code: String,
}


// CollateralRole1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CollateralRole1Code {
	#[validate(enumerate = ["GIVE", "TAKE"])]
	#[serde(rename = "CollateralRole1Code")]
	pub collateral_role1_code: String,
}


// CollateralType21 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CollateralType21 {
	#[validate]
	#[serde(rename = "Scty")]
	pub scty: Option<Vec<Security52>>,
	#[validate]
	#[serde(rename = "Csh")]
	pub csh: Option<Vec<AmountHaircutMargin1>>,
	#[validate]
	#[serde(rename = "Cmmdty")]
	pub cmmdty: Option<Vec<Commodity43>>,
}


// CollaterisedData12 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CollaterisedData12 {
	#[serde(rename = "CollValDt")]
	pub coll_val_dt: Option<String>,
	#[validate]
	#[serde(rename = "AsstTp")]
	pub asst_tp: Option<CollateralType21>,
	#[serde(rename = "NetXpsrCollstnInd")]
	pub net_xpsr_collstn_ind: Option<bool>,
	#[validate]
	#[serde(rename = "BsktIdr")]
	pub bskt_idr: Option<SecurityIdentification26Choice>,
}


// Commodity43 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Commodity43 {
	#[validate]
	#[serde(rename = "Clssfctn")]
	pub clssfctn: Option<AssetClassCommodity5Choice>,
	#[validate]
	#[serde(rename = "Qty")]
	pub qty: Option<Quantity17>,
	#[validate]
	#[serde(rename = "UnitPric")]
	pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
	#[validate]
	#[serde(rename = "MktVal")]
	pub mkt_val: Option<AmountAndDirection53>,
}


// ContractTerm7Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ContractTerm7Choice {
	#[validate]
	#[serde(rename = "Opn")]
	pub opn: Option<FixedOpenTermContract2>,
	#[validate]
	#[serde(rename = "Fxd")]
	pub fxd: Option<FixedOpenTermContract2>,
}


// CounterpartyData88 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CounterpartyData88 {
	#[serde(rename = "RptgDtTm")]
	pub rptg_dt_tm: String,
	#[validate]
	#[serde(rename = "RptSubmitgNtty")]
	pub rpt_submitg_ntty: OrganisationIdentification15Choice,
	#[validate]
	#[serde(rename = "CtrPty")]
	pub ctr_pty: Vec<CounterpartyData89>,
}


// CounterpartyData89 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CounterpartyData89 {
	#[validate]
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: CounterpartyIdentification11,
	#[validate]
	#[serde(rename = "OthrCtrPty")]
	pub othr_ctr_pty: CounterpartyIdentification12,
	#[validate]
	#[serde(rename = "NttyRspnsblForRpt")]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
	#[validate]
	#[serde(rename = "OthrPtyData")]
	pub othr_pty_data: Option<TransactionCounterpartyData11>,
}


// CounterpartyIdentification11 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CounterpartyIdentification11 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: OrganisationIdentification15Choice,
	#[validate]
	#[serde(rename = "Ntr")]
	pub ntr: Option<CounterpartyTradeNature7Choice>,
	#[validate]
	#[serde(rename = "Brnch")]
	pub brnch: Option<Branch5Choice>,
	#[serde(rename = "Sd")]
	pub sd: Option<String>,
}


// CounterpartyIdentification12 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CounterpartyIdentification12 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: PartyIdentification236Choice,
	#[validate]
	#[serde(rename = "Brnch")]
	pub brnch: Option<Branch6Choice>,
	#[serde(rename = "CtryCd")]
	pub ctry_cd: Option<String>,
}


// CounterpartyTradeNature7Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CounterpartyTradeNature7Choice {
	#[validate]
	#[serde(rename = "FI")]
	pub fi: Option<FinancialPartyClassification1>,
	#[validate]
	#[serde(rename = "NFI")]
	pub nfi: Option<Vec<FinancialPartyClassification2>>,
}


// CountryCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CountryCode {
	#[validate(pattern = "[A-Z]{2,2}")]
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// DecimalNumber ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "DecimalNumber")]
	pub decimal_number: f64,
}


// EnergyCommodityCoal1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnergyCommodityCoal1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// EnergyCommodityDistillates1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnergyCommodityDistillates1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// EnergyCommodityElectricity1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnergyCommodityElectricity1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: String,
}


// EnergyCommodityInterEnergy1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnergyCommodityInterEnergy1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// EnergyCommodityLightEnd1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnergyCommodityLightEnd1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// EnergyCommodityNaturalGas2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnergyCommodityNaturalGas2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: String,
}


// EnergyCommodityOil2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnergyCommodityOil2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: String,
}


// EnergyCommodityOther1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnergyCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// EnergyCommodityRenewableEnergy1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnergyCommodityRenewableEnergy1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// EnvironmentCommodityOther1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnvironmentCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// EnvironmentalCommodityCarbonRelated1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnvironmentalCommodityCarbonRelated1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// EnvironmentalCommodityEmission2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnvironmentalCommodityEmission2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: String,
}


// EnvironmentalCommodityWeather1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnvironmentalCommodityWeather1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// ExternalAgreementType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalAgreementType1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalAgreementType1Code")]
	pub external_agreement_type1_code: String,
}


// ExternalSecuritiesLendingType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalSecuritiesLendingType1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalSecuritiesLendingType1Code")]
	pub external_securities_lending_type1_code: String,
}


// FertilizerCommodityAmmonia1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FertilizerCommodityAmmonia1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// FertilizerCommodityDiammoniumPhosphate1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FertilizerCommodityDiammoniumPhosphate1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// FertilizerCommodityOther1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FertilizerCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// FertilizerCommodityPotash1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FertilizerCommodityPotash1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// FertilizerCommoditySulphur1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FertilizerCommoditySulphur1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// FertilizerCommodityUrea1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FertilizerCommodityUrea1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// FertilizerCommodityUreaAndAmmoniumNitrate1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FertilizerCommodityUreaAndAmmoniumNitrate1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// FinancialPartyClassification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialPartyClassification1 {
	#[serde(rename = "Clssfctn")]
	pub clssfctn: Vec<String>,
	#[serde(rename = "InvstmtFndClssfctn")]
	pub invstmt_fnd_clssfctn: Option<String>,
}


// FinancialPartyClassification2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialPartyClassification2 {
	#[serde(rename = "Clssfctn")]
	pub clssfctn: Vec<String>,
	#[serde(rename = "InvstmtFndClssfctn")]
	pub invstmt_fnd_clssfctn: Option<String>,
}


// FinancialPartySectorType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialPartySectorType2Code {
	#[validate(enumerate = ["AIFD", "CSDS", "CCPS", "CDTI", "INUN", "ORPI", "INVF", "REIN", "UCIT"])]
	#[serde(rename = "FinancialPartySectorType2Code")]
	pub financial_party_sector_type2_code: String,
}


// FixedOpenTermContract2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FixedOpenTermContract2 {
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "TermntnOptn")]
	pub termntn_optn: Option<String>,
}


// FixedRate11 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FixedRate11 {
	#[serde(rename = "Rate")]
	pub rate: Option<f64>,
	#[validate]
	#[serde(rename = "DayCntBsis")]
	pub day_cnt_bsis: Option<InterestComputationMethodFormat6Choice>,
}


// FloatingInterestRate22 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FloatingInterestRate22 {
	#[validate]
	#[serde(rename = "RefRate")]
	pub ref_rate: Option<BenchmarkCurveName10Choice>,
	#[validate]
	#[serde(rename = "Term")]
	pub term: Option<InterestRateContractTerm2>,
	#[validate]
	#[serde(rename = "PmtFrqcy")]
	pub pmt_frqcy: Option<InterestRateContractTerm2>,
	#[validate]
	#[serde(rename = "RstFrqcy")]
	pub rst_frqcy: Option<InterestRateContractTerm2>,
	#[validate]
	#[serde(rename = "Sprd")]
	pub sprd: Option<SecuritiesTransactionPrice18Choice>,
	#[validate]
	#[serde(rename = "RateAdjstmnt")]
	pub rate_adjstmnt: Option<Vec<RateAdjustment1>>,
	#[validate]
	#[serde(rename = "DayCntBsis")]
	pub day_cnt_bsis: Option<InterestComputationMethodFormat6Choice>,
}


// FreightCommodityContainerShip1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FreightCommodityContainerShip1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// FreightCommodityDry2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FreightCommodityDry2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: String,
}


// FreightCommodityOther1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FreightCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// FreightCommodityWet2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FreightCommodityWet2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: String,
}


// FundType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FundType2Code {
	#[validate(enumerate = ["ETFT", "MMFT", "OTHR", "REIT"])]
	#[serde(rename = "FundType2Code")]
	pub fund_type2_code: String,
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


// IndustrialProductCommodityConstruction1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct IndustrialProductCommodityConstruction1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// IndustrialProductCommodityManufacturing1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct IndustrialProductCommodityManufacturing1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// InterestComputationMethod1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InterestComputationMethod1Code {
	#[validate(enumerate = ["A001", "A002", "A003", "A004", "A005", "A006", "A007", "A008", "A009", "A010", "A011", "A012", "A013", "A014"])]
	#[serde(rename = "InterestComputationMethod1Code")]
	pub interest_computation_method1_code: String,
}


// InterestComputationMethodFormat6Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InterestComputationMethodFormat6Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// InterestRate27Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InterestRate27Choice {
	#[validate]
	#[serde(rename = "Fxd")]
	pub fxd: Option<FixedRate11>,
	#[validate]
	#[serde(rename = "Fltg")]
	pub fltg: Option<FloatingInterestRate22>,
}


// InterestRate6 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InterestRate6 {
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: AmountAndDirection53,
	#[validate]
	#[serde(rename = "IntrstRate")]
	pub intrst_rate: InterestRate27Choice,
}


// InterestRateContractTerm2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InterestRateContractTerm2 {
	#[serde(rename = "Unit")]
	pub unit: String,
	#[serde(rename = "Val")]
	pub val: f64,
}


// LEIIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[validate(pattern = "[A-Z0-9]{18,18}[0-9]{2,2}")]
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LoanData113 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LoanData113 {
	#[serde(rename = "UnqTradIdr")]
	pub unq_trad_idr: String,
	#[serde(rename = "EvtDt")]
	pub evt_dt: String,
	#[validate]
	#[serde(rename = "MktVal")]
	pub mkt_val: AmountAndDirection53,
}


// LoanData120 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LoanData120 {
	#[serde(rename = "EvtDt")]
	pub evt_dt: String,
	#[serde(rename = "UnqTradIdr")]
	pub unq_trad_idr: Option<String>,
	#[validate]
	#[serde(rename = "MstrAgrmt")]
	pub mstr_agrmt: Option<MasterAgreement7>,
}


// LoanData135 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LoanData135 {
	#[serde(rename = "UnqTradIdr")]
	pub unq_trad_idr: String,
	#[serde(rename = "EvtDt")]
	pub evt_dt: String,
	#[serde(rename = "ExctnDtTm")]
	pub exctn_dt_tm: String,
	#[validate]
	#[serde(rename = "ClrSts")]
	pub clr_sts: Option<Cleared16Choice>,
	#[serde(rename = "TradgVn")]
	pub tradg_vn: Option<String>,
	#[validate]
	#[serde(rename = "MstrAgrmt")]
	pub mstr_agrmt: Option<MasterAgreement7>,
	#[serde(rename = "ValDt")]
	pub val_dt: Option<String>,
	#[serde(rename = "MinNtcePrd")]
	pub min_ntce_prd: Option<f64>,
	#[serde(rename = "EarlstCallBckDt")]
	pub earlst_call_bck_dt: Option<String>,
	#[serde(rename = "GnlColl")]
	pub gnl_coll: Option<String>,
	#[serde(rename = "DlvryByVal")]
	pub dlvry_by_val: Option<bool>,
	#[serde(rename = "CollDlvryMtd")]
	pub coll_dlvry_mtd: Option<String>,
	#[validate]
	#[serde(rename = "Term")]
	pub term: Option<Vec<ContractTerm7Choice>>,
	#[validate]
	#[serde(rename = "IntrstRate")]
	pub intrst_rate: Option<InterestRate27Choice>,
	#[validate]
	#[serde(rename = "PrncplAmt")]
	pub prncpl_amt: Option<PrincipalAmount3>,
	#[serde(rename = "TermntnDt")]
	pub termntn_dt: Option<String>,
}


// LoanData136 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LoanData136 {
	#[serde(rename = "UnqTradIdr")]
	pub unq_trad_idr: String,
	#[serde(rename = "EvtDt")]
	pub evt_dt: String,
	#[serde(rename = "ExctnDtTm")]
	pub exctn_dt_tm: String,
	#[validate]
	#[serde(rename = "ClrSts")]
	pub clr_sts: Option<Cleared16Choice>,
	#[serde(rename = "TradgVn")]
	pub tradg_vn: Option<String>,
	#[validate]
	#[serde(rename = "MstrAgrmt")]
	pub mstr_agrmt: Option<MasterAgreement7>,
	#[serde(rename = "ValDt")]
	pub val_dt: Option<String>,
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "GnlColl")]
	pub gnl_coll: Option<String>,
	#[validate]
	#[serde(rename = "PrncplAmt")]
	pub prncpl_amt: Option<PrincipalAmount3>,
	#[validate]
	#[serde(rename = "UnitPric")]
	pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
	#[serde(rename = "TermntnDt")]
	pub termntn_dt: Option<String>,
}


// LoanData137 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LoanData137 {
	#[serde(rename = "UnqTradIdr")]
	pub unq_trad_idr: String,
	#[serde(rename = "EvtDt")]
	pub evt_dt: String,
	#[serde(rename = "ExctnDtTm")]
	pub exctn_dt_tm: String,
	#[validate]
	#[serde(rename = "ClrSts")]
	pub clr_sts: Option<Cleared16Choice>,
	#[serde(rename = "TradgVn")]
	pub tradg_vn: Option<String>,
	#[validate]
	#[serde(rename = "MstrAgrmt")]
	pub mstr_agrmt: Option<MasterAgreement7>,
	#[serde(rename = "ValDt")]
	pub val_dt: Option<String>,
	#[serde(rename = "GnlColl")]
	pub gnl_coll: Option<String>,
	#[serde(rename = "DlvryByVal")]
	pub dlvry_by_val: Option<bool>,
	#[serde(rename = "CollDlvryMtd")]
	pub coll_dlvry_mtd: Option<String>,
	#[validate]
	#[serde(rename = "Term")]
	pub term: Option<Vec<ContractTerm7Choice>>,
	#[validate]
	#[serde(rename = "AsstTp")]
	pub asst_tp: Option<SecurityCommodity9>,
	#[validate]
	#[serde(rename = "LnVal")]
	pub ln_val: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "RbtRate")]
	pub rbt_rate: Option<InterestRate27Choice>,
	#[serde(rename = "LndgFee")]
	pub lndg_fee: Option<f64>,
	#[serde(rename = "TermntnDt")]
	pub termntn_dt: Option<String>,
}


// LoanData138 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LoanData138 {
	#[serde(rename = "UnqTradIdr")]
	pub unq_trad_idr: String,
	#[serde(rename = "EvtDt")]
	pub evt_dt: String,
	#[serde(rename = "ExctnDtTm")]
	pub exctn_dt_tm: String,
	#[serde(rename = "TradgVn")]
	pub tradg_vn: Option<String>,
	#[serde(rename = "CollDlvryMtd")]
	pub coll_dlvry_mtd: Option<String>,
	#[validate]
	#[serde(rename = "OutsdngMrgnLnAmt")]
	pub outsdng_mrgn_ln_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "ShrtMktValAmt")]
	pub shrt_mkt_val_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "MrgnLnAttr")]
	pub mrgn_ln_attr: Option<Vec<InterestRate6>>,
	#[serde(rename = "TermntnDt")]
	pub termntn_dt: Option<String>,
}


// LoanData139 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LoanData139 {
	#[serde(rename = "UnqTradIdr")]
	pub unq_trad_idr: Option<String>,
	#[serde(rename = "EvtDt")]
	pub evt_dt: Option<String>,
	#[serde(rename = "ExctnDtTm")]
	pub exctn_dt_tm: Option<String>,
	#[validate]
	#[serde(rename = "ClrSts")]
	pub clr_sts: Option<Cleared16Choice>,
	#[serde(rename = "TradgVn")]
	pub tradg_vn: Option<String>,
	#[validate]
	#[serde(rename = "MstrAgrmt")]
	pub mstr_agrmt: Option<MasterAgreement7>,
	#[serde(rename = "ValDt")]
	pub val_dt: Option<String>,
	#[serde(rename = "MinNtcePrd")]
	pub min_ntce_prd: Option<f64>,
	#[serde(rename = "EarlstCallBckDt")]
	pub earlst_call_bck_dt: Option<String>,
	#[serde(rename = "GnlColl")]
	pub gnl_coll: Option<String>,
	#[serde(rename = "DlvryByVal")]
	pub dlvry_by_val: Option<bool>,
	#[serde(rename = "CollDlvryMtd")]
	pub coll_dlvry_mtd: Option<String>,
	#[validate]
	#[serde(rename = "Term")]
	pub term: Option<Vec<ContractTerm7Choice>>,
	#[validate]
	#[serde(rename = "IntrstRate")]
	pub intrst_rate: Option<InterestRate27Choice>,
	#[validate]
	#[serde(rename = "PrncplAmt")]
	pub prncpl_amt: Option<PrincipalAmount3>,
	#[serde(rename = "TermntnDt")]
	pub termntn_dt: Option<String>,
}


// LoanData140 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LoanData140 {
	#[serde(rename = "UnqTradIdr")]
	pub unq_trad_idr: Option<String>,
	#[serde(rename = "EvtDt")]
	pub evt_dt: String,
	#[serde(rename = "ExctnDtTm")]
	pub exctn_dt_tm: Option<String>,
	#[validate]
	#[serde(rename = "ClrSts")]
	pub clr_sts: Option<Cleared16Choice>,
	#[serde(rename = "TradgVn")]
	pub tradg_vn: Option<String>,
	#[validate]
	#[serde(rename = "MstrAgrmt")]
	pub mstr_agrmt: Option<MasterAgreement7>,
	#[serde(rename = "ValDt")]
	pub val_dt: Option<String>,
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "GnlColl")]
	pub gnl_coll: Option<String>,
	#[validate]
	#[serde(rename = "PrncplAmt")]
	pub prncpl_amt: Option<PrincipalAmount3>,
	#[validate]
	#[serde(rename = "UnitPric")]
	pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
	#[serde(rename = "TermntnDt")]
	pub termntn_dt: Option<String>,
}


// LoanData141 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LoanData141 {
	#[serde(rename = "UnqTradIdr")]
	pub unq_trad_idr: Option<String>,
	#[serde(rename = "EvtDt")]
	pub evt_dt: String,
	#[serde(rename = "ExctnDtTm")]
	pub exctn_dt_tm: Option<String>,
	#[validate]
	#[serde(rename = "ClrSts")]
	pub clr_sts: Option<Cleared16Choice>,
	#[serde(rename = "TradgVn")]
	pub tradg_vn: Option<String>,
	#[validate]
	#[serde(rename = "MstrAgrmt")]
	pub mstr_agrmt: Option<MasterAgreement7>,
	#[serde(rename = "ValDt")]
	pub val_dt: Option<String>,
	#[serde(rename = "GnlColl")]
	pub gnl_coll: Option<String>,
	#[serde(rename = "DlvryByVal")]
	pub dlvry_by_val: Option<bool>,
	#[serde(rename = "CollDlvryMtd")]
	pub coll_dlvry_mtd: Option<String>,
	#[validate]
	#[serde(rename = "Term")]
	pub term: Option<Vec<ContractTerm7Choice>>,
	#[validate]
	#[serde(rename = "AsstTp")]
	pub asst_tp: Option<SecurityCommodity9>,
	#[validate]
	#[serde(rename = "LnVal")]
	pub ln_val: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "RbtRate")]
	pub rbt_rate: Option<InterestRate27Choice>,
	#[serde(rename = "LndgFee")]
	pub lndg_fee: Option<f64>,
	#[serde(rename = "TermntnDt")]
	pub termntn_dt: Option<String>,
}


// LoanData142 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LoanData142 {
	#[serde(rename = "UnqTradIdr")]
	pub unq_trad_idr: Option<String>,
	#[serde(rename = "EvtDt")]
	pub evt_dt: String,
	#[serde(rename = "ExctnDtTm")]
	pub exctn_dt_tm: Option<String>,
	#[serde(rename = "TradgVn")]
	pub tradg_vn: Option<String>,
	#[serde(rename = "CollDlvryMtd")]
	pub coll_dlvry_mtd: Option<String>,
	#[validate]
	#[serde(rename = "OutsdngMrgnLnAmt")]
	pub outsdng_mrgn_ln_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "ShrtMktValAmt")]
	pub shrt_mkt_val_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "MrgnLnAttr")]
	pub mrgn_ln_attr: Option<Vec<InterestRate6>>,
	#[serde(rename = "TermntnDt")]
	pub termntn_dt: Option<String>,
}


// LoanData143 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LoanData143 {
	#[serde(rename = "UnqTradIdr")]
	pub unq_trad_idr: String,
	#[serde(rename = "EvtDt")]
	pub evt_dt: String,
	#[serde(rename = "ExctnDtTm")]
	pub exctn_dt_tm: String,
	#[validate]
	#[serde(rename = "ClrSts")]
	pub clr_sts: Cleared16Choice,
	#[serde(rename = "TradgVn")]
	pub tradg_vn: String,
	#[validate]
	#[serde(rename = "MstrAgrmt")]
	pub mstr_agrmt: Option<MasterAgreement7>,
	#[serde(rename = "ValDt")]
	pub val_dt: String,
	#[serde(rename = "GnlColl")]
	pub gnl_coll: Option<String>,
	#[serde(rename = "DlvryByVal")]
	pub dlvry_by_val: bool,
	#[serde(rename = "CollDlvryMtd")]
	pub coll_dlvry_mtd: String,
	#[validate]
	#[serde(rename = "Term")]
	pub term: Option<Vec<ContractTerm7Choice>>,
	#[validate]
	#[serde(rename = "IntrstRate")]
	pub intrst_rate: Option<InterestRate27Choice>,
	#[validate]
	#[serde(rename = "PrncplAmt")]
	pub prncpl_amt: Option<PrincipalAmount3>,
	#[serde(rename = "TermntnDt")]
	pub termntn_dt: Option<String>,
	#[validate]
	#[serde(rename = "UnitPric")]
	pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
}


// LoanData144 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LoanData144 {
	#[validate]
	#[serde(rename = "PrncplAmt")]
	pub prncpl_amt: Option<PrincipalAmount3>,
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: String,
	#[serde(rename = "UnqTradIdr")]
	pub unq_trad_idr: String,
	#[serde(rename = "EvtDt")]
	pub evt_dt: String,
	#[validate]
	#[serde(rename = "ClrSts")]
	pub clr_sts: Cleared16Choice,
	#[serde(rename = "TradgVn")]
	pub tradg_vn: String,
	#[validate]
	#[serde(rename = "MstrAgrmt")]
	pub mstr_agrmt: Option<MasterAgreement7>,
	#[serde(rename = "ExctnDtTm")]
	pub exctn_dt_tm: String,
	#[serde(rename = "ValDt")]
	pub val_dt: String,
	#[serde(rename = "TermntnDt")]
	pub termntn_dt: Option<String>,
	#[serde(rename = "GnlColl")]
	pub gnl_coll: Option<String>,
	#[validate]
	#[serde(rename = "UnitPric")]
	pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
}


// LoanData145 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LoanData145 {
	#[serde(rename = "DlvryByVal")]
	pub dlvry_by_val: bool,
	#[serde(rename = "CollDlvryMtd")]
	pub coll_dlvry_mtd: Option<String>,
	#[validate]
	#[serde(rename = "Term")]
	pub term: Option<Vec<ContractTerm7Choice>>,
	#[validate]
	#[serde(rename = "AsstTp")]
	pub asst_tp: SecurityCommodity9,
	#[validate]
	#[serde(rename = "RbtRate")]
	pub rbt_rate: Option<InterestRate27Choice>,
	#[validate]
	#[serde(rename = "LnVal")]
	pub ln_val: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "LndgFee")]
	pub lndg_fee: Option<f64>,
	#[serde(rename = "UnqTradIdr")]
	pub unq_trad_idr: String,
	#[serde(rename = "EvtDt")]
	pub evt_dt: String,
	#[validate]
	#[serde(rename = "ClrSts")]
	pub clr_sts: Cleared16Choice,
	#[serde(rename = "TradgVn")]
	pub tradg_vn: String,
	#[validate]
	#[serde(rename = "MstrAgrmt")]
	pub mstr_agrmt: Option<MasterAgreement7>,
	#[serde(rename = "ExctnDtTm")]
	pub exctn_dt_tm: String,
	#[serde(rename = "ValDt")]
	pub val_dt: String,
	#[serde(rename = "TermntnDt")]
	pub termntn_dt: Option<String>,
	#[serde(rename = "GnlColl")]
	pub gnl_coll: Option<String>,
}


// LoanData86 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LoanData86 {
	#[serde(rename = "UnqTradIdr")]
	pub unq_trad_idr: String,
	#[serde(rename = "EvtDt")]
	pub evt_dt: Option<String>,
	#[serde(rename = "TermntnDt")]
	pub termntn_dt: Option<String>,
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


// MasterAgreement7 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MasterAgreement7 {
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: AgreementType2Choice,
	#[serde(rename = "Vrsn")]
	pub vrsn: Option<String>,
	#[serde(rename = "OthrMstrAgrmtDtls")]
	pub othr_mstr_agrmt_dtls: Option<String>,
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


// Max20PositiveNumber ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max20PositiveNumber {
	#[serde(rename = "Max20PositiveNumber")]
	pub max20_positive_number: f64,
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


// MetalCommodityNonPrecious1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MetalCommodityNonPrecious1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: String,
}


// MetalCommodityPrecious1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MetalCommodityPrecious1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: String,
}


// ModificationLevel1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ModificationLevel1Code {
	#[validate(enumerate = ["PSTN", "TCTN"])]
	#[serde(rename = "ModificationLevel1Code")]
	pub modification_level1_code: String,
}


// NACEDomainIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NACEDomainIdentifier {
	#[validate(pattern = "[A-U]{1,1}")]
	#[serde(rename = "NACEDomainIdentifier")]
	pub nace_domain_identifier: String,
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


// NoReasonCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NoReasonCode {
	#[validate(enumerate = ["NORE"])]
	#[serde(rename = "NoReasonCode")]
	pub no_reason_code: String,
}


// NotAvailable1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NotAvailable1Code {
	#[validate(enumerate = ["NTAV"])]
	#[serde(rename = "NotAvailable1Code")]
	pub not_available1_code: String,
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


// OtherC10CommodityDeliverable2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OtherC10CommodityDeliverable2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// OtherC10CommodityNonDeliverable2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OtherC10CommodityNonDeliverable2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PaperCommodityContainerBoard1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PaperCommodityContainerBoard1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PaperCommodityNewsprint1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PaperCommodityNewsprint1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PaperCommodityPulp1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PaperCommodityPulp1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PaperCommodityRecoveredPaper1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PaperCommodityRecoveredPaper1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PaperCommodityRecoveredPaper2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PaperCommodityRecoveredPaper2 {
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


// PercentageRate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// PlusOrMinusIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "PlusOrMinusIndicator")]
	pub plus_or_minus_indicator: bool,
}


// PolypropyleneCommodityOther1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PolypropyleneCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// PolypropyleneCommodityPlastic1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PolypropyleneCommodityPlastic1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: Option<String>,
}


// PriceStatus1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PriceStatus1Code {
	#[validate(enumerate = ["PNDG", "NOAP"])]
	#[serde(rename = "PriceStatus1Code")]
	pub price_status1_code: String,
}


// PrincipalAmount3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PrincipalAmount3 {
	#[validate]
	#[serde(rename = "ValDtAmt")]
	pub val_dt_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "MtrtyDtAmt")]
	pub mtrty_dt_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// Quantity17 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Quantity17 {
	#[serde(rename = "Val")]
	pub val: f64,
	#[serde(rename = "UnitOfMeasr")]
	pub unit_of_measr: String,
}


// QuantityNominalValue2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct QuantityNominalValue2Choice {
	#[serde(rename = "Qty")]
	pub qty: Option<f64>,
	#[validate]
	#[serde(rename = "NmnlVal")]
	pub nmnl_val: Option<AmountAndDirection53>,
}


// RateAdjustment1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RateAdjustment1 {
	#[serde(rename = "Rate")]
	pub rate: f64,
	#[serde(rename = "AdjstmntDt")]
	pub adjstmnt_dt: String,
}


// RateBasis1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RateBasis1Code {
	#[validate(enumerate = ["DAYS", "MNTH", "WEEK", "YEAR"])]
	#[serde(rename = "RateBasis1Code")]
	pub rate_basis1_code: String,
}


// RepoTerminationOption2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RepoTerminationOption2Code {
	#[validate(enumerate = ["EGRN", "EGAE", "ETSB", "NOAP"])]
	#[serde(rename = "RepoTerminationOption2Code")]
	pub repo_termination_option2_code: String,
}


// ReportPeriodActivity1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ReportPeriodActivity1Code {
	#[validate(enumerate = ["NOTX"])]
	#[serde(rename = "ReportPeriodActivity1Code")]
	pub report_period_activity1_code: String,
}


// SecuritiesFinancingReportingTransactionReportV02 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesFinancingReportingTransactionReportV02 {
	#[validate]
	#[serde(rename = "TradData")]
	pub trad_data: TradeData40Choice,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// SecuritiesLendingType3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesLendingType3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// SecuritiesTransactionPrice18Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice18Choice {
	#[validate]
	#[serde(rename = "MntryVal")]
	pub mntry_val: Option<AmountAndDirection107>,
	#[serde(rename = "Pctg")]
	pub pctg: Option<f64>,
	#[serde(rename = "Dcml")]
	pub dcml: Option<f64>,
	#[serde(rename = "BsisPts")]
	pub bsis_pts: Option<f64>,
}


// SecuritiesTransactionPrice19Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice19Choice {
	#[validate]
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


// Security51 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Security51 {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "ClssfctnTp")]
	pub clssfctn_tp: Option<String>,
	#[validate]
	#[serde(rename = "QtyOrNmnlVal")]
	pub qty_or_nmnl_val: Option<QuantityNominalValue2Choice>,
	#[validate]
	#[serde(rename = "UnitPric")]
	pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
	#[validate]
	#[serde(rename = "MktVal")]
	pub mkt_val: Option<AmountAndDirection53>,
	#[serde(rename = "Qlty")]
	pub qlty: Option<String>,
	#[serde(rename = "Mtrty")]
	pub mtrty: Option<String>,
	#[validate]
	#[serde(rename = "Issr")]
	pub issr: Option<SecurityIssuer4>,
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: Option<Vec<SecuritiesLendingType3Choice>>,
	#[serde(rename = "ExclsvArrgmnt")]
	pub exclsv_arrgmnt: Option<bool>,
	#[serde(rename = "AvlblForCollReuse")]
	pub avlbl_for_coll_reuse: Option<bool>,
}


// Security52 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Security52 {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "ClssfctnTp")]
	pub clssfctn_tp: Option<String>,
	#[validate]
	#[serde(rename = "QtyOrNmnlVal")]
	pub qty_or_nmnl_val: Option<QuantityNominalValue2Choice>,
	#[validate]
	#[serde(rename = "UnitPric")]
	pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
	#[validate]
	#[serde(rename = "MktVal")]
	pub mkt_val: Option<AmountAndDirection53>,
	#[serde(rename = "Qlty")]
	pub qlty: Option<String>,
	#[serde(rename = "Mtrty")]
	pub mtrty: Option<String>,
	#[validate]
	#[serde(rename = "Issr")]
	pub issr: Option<SecurityIssuer4>,
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: Option<Vec<SecuritiesLendingType3Choice>>,
	#[serde(rename = "ExclsvArrgmnt")]
	pub exclsv_arrgmnt: Option<bool>,
	#[serde(rename = "HrcutOrMrgn")]
	pub hrcut_or_mrgn: Option<f64>,
	#[serde(rename = "AvlblForCollReuse")]
	pub avlbl_for_coll_reuse: Option<bool>,
}


// Security55 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Security55 {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "ClssfctnTp")]
	pub clssfctn_tp: Option<String>,
	#[validate]
	#[serde(rename = "QtyOrNmnlVal")]
	pub qty_or_nmnl_val: Option<QuantityNominalValue2Choice>,
	#[validate]
	#[serde(rename = "UnitPric")]
	pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
	#[validate]
	#[serde(rename = "MktVal")]
	pub mkt_val: Option<AmountAndDirection53>,
	#[serde(rename = "Qlty")]
	pub qlty: Option<String>,
	#[serde(rename = "Mtrty")]
	pub mtrty: Option<String>,
	#[validate]
	#[serde(rename = "Issr")]
	pub issr: Option<SecurityIssuer4>,
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: Option<Vec<SecuritiesLendingType3Choice>>,
	#[serde(rename = "ExclsvArrgmnt")]
	pub exclsv_arrgmnt: Option<bool>,
	#[serde(rename = "AvlblForCollReuse")]
	pub avlbl_for_coll_reuse: Option<bool>,
	#[serde(rename = "HrcutOrMrgn")]
	pub hrcut_or_mrgn: Option<f64>,
}


// SecurityCommodity9 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityCommodity9 {
	#[validate]
	#[serde(rename = "Scty")]
	pub scty: Option<Vec<Security51>>,
	#[validate]
	#[serde(rename = "Cmmdty")]
	pub cmmdty: Option<Vec<Commodity43>>,
}


// SecurityIdentification26Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityIdentification26Choice {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "NotAvlbl")]
	pub not_avlbl: Option<String>,
}


// SecurityIssuer4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityIssuer4 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "JursdctnCtry")]
	pub jursdctn_ctry: String,
}


// SettlementParties34Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementParties34Choice {
	#[validate]
	#[serde(rename = "CntrlSctiesDpstryPtcpt")]
	pub cntrl_scties_dpstry_ptcpt: Option<OrganisationIdentification15Choice>,
	#[validate]
	#[serde(rename = "IndrctPtcpt")]
	pub indrct_ptcpt: Option<OrganisationIdentification15Choice>,
}


// SpecialCollateral1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SpecialCollateral1Code {
	#[validate(enumerate = ["GENE", "SPEC"])]
	#[serde(rename = "SpecialCollateral1Code")]
	pub special_collateral1_code: String,
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


// TradeData40Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeData40Choice {
	#[serde(rename = "DataSetActn")]
	pub data_set_actn: Option<String>,
	#[validate]
	#[serde(rename = "Rpt")]
	pub rpt: Option<Vec<TradeReport22Choice>>,
}


// TradeError9 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeError9 {
	#[serde(rename = "TechRcrdId")]
	pub tech_rcrd_id: Option<String>,
	#[validate]
	#[serde(rename = "CtrPtySpcfcData")]
	pub ctr_pty_spcfc_data: CounterpartyData88,
	#[validate]
	#[serde(rename = "LnData")]
	pub ln_data: LoanData86,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// TradeNewTransaction13 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeNewTransaction13 {
	#[serde(rename = "TechRcrdId")]
	pub tech_rcrd_id: Option<String>,
	#[validate]
	#[serde(rename = "CtrPtySpcfcData")]
	pub ctr_pty_spcfc_data: CounterpartyData88,
	#[validate]
	#[serde(rename = "LnData")]
	pub ln_data: TransactionLoanData30Choice,
	#[validate]
	#[serde(rename = "CollData")]
	pub coll_data: Option<TransactionCollateralData18Choice>,
	#[serde(rename = "LvlTp")]
	pub lvl_tp: String,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// TradeReport22Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeReport22Choice {
	#[validate]
	#[serde(rename = "New")]
	pub new: Option<TradeNewTransaction13>,
	#[validate]
	#[serde(rename = "Mod")]
	pub mod_attr: Option<TradeTransactionCorrection13>,
	#[validate]
	#[serde(rename = "Err")]
	pub err: Option<TradeError9>,
	#[validate]
	#[serde(rename = "EarlyTermntn")]
	pub early_termntn: Option<TradeError9>,
	#[validate]
	#[serde(rename = "PosCmpnt")]
	pub pos_cmpnt: Option<TradeTransactionPositionComponent8>,
	#[validate]
	#[serde(rename = "CollUpd")]
	pub coll_upd: Option<TradeTransactionCollateralUpdate8>,
	#[validate]
	#[serde(rename = "Crrctn")]
	pub crrctn: Option<TradeTransactionCorrection13>,
	#[validate]
	#[serde(rename = "ValtnUpd")]
	pub valtn_upd: Option<TradeValuationUpdate9>,
}


// TradeTransactionCollateralUpdate8 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeTransactionCollateralUpdate8 {
	#[serde(rename = "TechRcrdId")]
	pub tech_rcrd_id: Option<String>,
	#[validate]
	#[serde(rename = "CtrPtySpcfcData")]
	pub ctr_pty_spcfc_data: CounterpartyData88,
	#[validate]
	#[serde(rename = "LnData")]
	pub ln_data: Option<TransactionLoanData26Choice>,
	#[validate]
	#[serde(rename = "CollData")]
	pub coll_data: TransactionCollateralData18Choice,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// TradeTransactionCorrection13 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeTransactionCorrection13 {
	#[serde(rename = "TechRcrdId")]
	pub tech_rcrd_id: Option<String>,
	#[validate]
	#[serde(rename = "CtrPtySpcfcData")]
	pub ctr_pty_spcfc_data: CounterpartyData88,
	#[validate]
	#[serde(rename = "LnData")]
	pub ln_data: Option<TransactionLoanData31Choice>,
	#[validate]
	#[serde(rename = "CollData")]
	pub coll_data: Option<TransactionCollateralData18Choice>,
	#[serde(rename = "LvlTp")]
	pub lvl_tp: String,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// TradeTransactionPositionComponent8 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeTransactionPositionComponent8 {
	#[serde(rename = "TechRcrdId")]
	pub tech_rcrd_id: Option<String>,
	#[validate]
	#[serde(rename = "CtrPtySpcfcData")]
	pub ctr_pty_spcfc_data: CounterpartyData88,
	#[validate]
	#[serde(rename = "LnData")]
	pub ln_data: Option<TransactionLoanData32Choice>,
	#[validate]
	#[serde(rename = "CollData")]
	pub coll_data: Option<CollateralData35>,
	#[serde(rename = "LvlTp")]
	pub lvl_tp: String,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// TradeValuationUpdate9 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeValuationUpdate9 {
	#[serde(rename = "TechRcrdId")]
	pub tech_rcrd_id: Option<String>,
	#[validate]
	#[serde(rename = "CtrPtySpcfcData")]
	pub ctr_pty_spcfc_data: CounterpartyData88,
	#[validate]
	#[serde(rename = "LnData")]
	pub ln_data: LoanData113,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// TransactionCollateralData18Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TransactionCollateralData18Choice {
	#[validate]
	#[serde(rename = "RpTrad")]
	pub rp_trad: Option<Collateral52>,
	#[validate]
	#[serde(rename = "BuySellBck")]
	pub buy_sell_bck: Option<Collateral52>,
	#[validate]
	#[serde(rename = "SctiesLndg")]
	pub scties_lndg: Option<CollateralFlag13Choice>,
	#[validate]
	#[serde(rename = "MrgnLndg")]
	pub mrgn_lndg: Option<Vec<Security55>>,
}


// TransactionCounterpartyData11 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TransactionCounterpartyData11 {
	#[validate]
	#[serde(rename = "Bnfcry")]
	pub bnfcry: Option<PartyIdentification236Choice>,
	#[validate]
	#[serde(rename = "TrptyAgt")]
	pub trpty_agt: Option<OrganisationIdentification15Choice>,
	#[validate]
	#[serde(rename = "Brkr")]
	pub brkr: Option<OrganisationIdentification15Choice>,
	#[validate]
	#[serde(rename = "ClrMmb")]
	pub clr_mmb: Option<OrganisationIdentification15Choice>,
	#[validate]
	#[serde(rename = "SttlmPties")]
	pub sttlm_pties: Option<SettlementParties34Choice>,
	#[validate]
	#[serde(rename = "AgtLndr")]
	pub agt_lndr: Option<OrganisationIdentification15Choice>,
}


// TransactionLoanData26Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TransactionLoanData26Choice {
	#[validate]
	#[serde(rename = "RpTrad")]
	pub rp_trad: Option<LoanData120>,
	#[validate]
	#[serde(rename = "BuySellBck")]
	pub buy_sell_bck: Option<LoanData120>,
	#[validate]
	#[serde(rename = "SctiesLndg")]
	pub scties_lndg: Option<LoanData120>,
	#[validate]
	#[serde(rename = "MrgnLndg")]
	pub mrgn_lndg: Option<LoanData120>,
}


// TransactionLoanData30Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TransactionLoanData30Choice {
	#[validate]
	#[serde(rename = "RpTrad")]
	pub rp_trad: Option<LoanData135>,
	#[validate]
	#[serde(rename = "BuySellBck")]
	pub buy_sell_bck: Option<LoanData136>,
	#[validate]
	#[serde(rename = "SctiesLndg")]
	pub scties_lndg: Option<LoanData137>,
	#[validate]
	#[serde(rename = "MrgnLndg")]
	pub mrgn_lndg: Option<LoanData138>,
}


// TransactionLoanData31Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TransactionLoanData31Choice {
	#[validate]
	#[serde(rename = "RpTrad")]
	pub rp_trad: Option<LoanData139>,
	#[validate]
	#[serde(rename = "BuySellBck")]
	pub buy_sell_bck: Option<LoanData140>,
	#[validate]
	#[serde(rename = "SctiesLndg")]
	pub scties_lndg: Option<LoanData141>,
	#[validate]
	#[serde(rename = "MrgnLndg")]
	pub mrgn_lndg: Option<LoanData142>,
}


// TransactionLoanData32Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TransactionLoanData32Choice {
	#[validate]
	#[serde(rename = "RpTrad")]
	pub rp_trad: Option<LoanData143>,
	#[validate]
	#[serde(rename = "BuySellBck")]
	pub buy_sell_bck: Option<LoanData144>,
	#[validate]
	#[serde(rename = "SctiesLndg")]
	pub scties_lndg: Option<LoanData145>,
}


// TrueFalseIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}


// UnitOfMeasure11Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct UnitOfMeasure11Code {
	#[validate(enumerate = ["ALOW", "ACCY", "BARL", "BCUF", "BDFT", "BUSL", "CEER", "CLRT", "KILO", "PIEC", "TONS", "METR", "INCH", "YARD", "GBGA", "GRAM", "CMET", "SMET", "FOOT", "MILE", "SQIN", "SQFO", "SQMI", "GBOU", "USOU", "GBPI", "USPI", "GBQA", "USGA", "MMET", "KMET", "SQYA", "ACRE", "ARES", "SMIL", "SCMT", "HECT", "SQKI", "MILI", "CELI", "LITR", "PUND", "CBME", "DAYS", "DMET", "ENVC", "ENVO", "HUWG", "KWDC", "KWHO", "KWHC", "KMOC", "KWMC", "KWYC", "MWDC", "MWHO", "MWHC", "MWMC", "MMOC", "MWYC", "TONE", "MIBA", "MBTU", "OZTR", "UCWT", "IPNT", "PWRD", "DGEU", "TOCD", "GGEU", "USQA"])]
	#[serde(rename = "UnitOfMeasure11Code")]
	pub unit_of_measure11_code: String,
}
