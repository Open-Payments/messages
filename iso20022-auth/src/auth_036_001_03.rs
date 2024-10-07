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


// ActiveCurrencyAnd13DecimalAmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd13DecimalAmountSimpleType {
	#[serde(rename = "ActiveCurrencyAnd13DecimalAmount_SimpleType")]
	pub active_currency_and13_decimal_amount_simple_type: f64,
}


// ActiveCurrencyAnd13DecimalAmount ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd13DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveCurrencyCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[validate(pattern = "[A-Z]{3,3}")]
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
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


// AgriculturalCommodityGrain1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AgriculturalCommodityGrain1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
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


// AgriculturalCommodityOliveOil1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AgriculturalCommodityOliveOil1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
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


// AmountAndDirection61 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AmountAndDirection61 {
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAnd13DecimalAmount,
	#[serde(rename = "Sgn")]
	pub sgn: Option<bool>,
}


// AssetClass2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClass2 {
	#[validate]
	#[serde(rename = "Cmmdty")]
	pub cmmdty: Option<DerivativeCommodity2>,
	#[validate]
	#[serde(rename = "Intrst")]
	pub intrst: Option<DerivativeInterest3>,
	#[validate]
	#[serde(rename = "FX")]
	pub fx: Option<DerivativeForeignExchange3>,
}


// AssetClassCommodity3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodity3Choice {
	#[validate]
	#[serde(rename = "Agrcltrl")]
	pub agrcltrl: Option<AssetClassCommodityAgricultural1Choice>,
	#[validate]
	#[serde(rename = "Nrgy")]
	pub nrgy: Option<AssetClassCommodityEnergy1Choice>,
	#[validate]
	#[serde(rename = "Envttl")]
	pub envttl: Option<AssetClassCommodityEnvironmental1Choice>,
	#[validate]
	#[serde(rename = "Frtlzr")]
	pub frtlzr: Option<AssetClassCommodityFertilizer1Choice>,
	#[validate]
	#[serde(rename = "Frght")]
	pub frght: Option<AssetClassCommodityFreight1Choice>,
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
	pub ppr: Option<AssetClassCommodityPaper1Choice>,
	#[validate]
	#[serde(rename = "Plprpln")]
	pub plprpln: Option<AssetClassCommodityPolypropylene1Choice>,
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


// AssetClassCommodityAgricultural1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityAgricultural1Choice {
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
	pub olv_oil: Option<AgriculturalCommodityOliveOil1>,
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
	pub grn: Option<AgriculturalCommodityGrain1>,
}


// AssetClassCommodityEnergy1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityEnergy1Choice {
	#[validate]
	#[serde(rename = "Elctrcty")]
	pub elctrcty: Option<EnergyCommodityElectricity1>,
	#[validate]
	#[serde(rename = "NtrlGas")]
	pub ntrl_gas: Option<EnergyCommodityNaturalGas1>,
	#[validate]
	#[serde(rename = "Oil")]
	pub oil: Option<EnergyCommodityOil1>,
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
}


// AssetClassCommodityEnvironmental1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityEnvironmental1Choice {
	#[validate]
	#[serde(rename = "Emssns")]
	pub emssns: Option<EnvironmentalCommodityEmission1>,
	#[validate]
	#[serde(rename = "Wthr")]
	pub wthr: Option<EnvironmentalCommodityWeather1>,
	#[validate]
	#[serde(rename = "CrbnRltd")]
	pub crbn_rltd: Option<EnvironmentalCommodityCarbonRelated1>,
}


// AssetClassCommodityFertilizer1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityFertilizer1Choice {
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
}


// AssetClassCommodityFreight1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityFreight1Choice {
	#[validate]
	#[serde(rename = "Dry")]
	pub dry: Option<FreightCommodityDry1>,
	#[validate]
	#[serde(rename = "Wet")]
	pub wet: Option<FreightCommodityWet1>,
	#[validate]
	#[serde(rename = "CntnrShip")]
	pub cntnr_ship: Option<FreightCommodityContainerShip1>,
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


// AssetClassCommodityPaper1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityPaper1Choice {
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
}


// AssetClassCommodityPolypropylene1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassCommodityPolypropylene1Choice {
	#[validate]
	#[serde(rename = "Plstc")]
	pub plstc: Option<PolypropyleneCommodityPlastic1>,
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


// AssetClassDetailedSubProductType12Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType12Code {
	#[validate(enumerate = ["TNKR"])]
	#[serde(rename = "AssetClassDetailedSubProductType12Code")]
	pub asset_class_detailed_sub_product_type12_code: String,
}


// AssetClassDetailedSubProductType14Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType14Code {
	#[validate(enumerate = ["DBCR"])]
	#[serde(rename = "AssetClassDetailedSubProductType14Code")]
	pub asset_class_detailed_sub_product_type14_code: String,
}


// AssetClassDetailedSubProductType15Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType15Code {
	#[validate(enumerate = ["MWHT"])]
	#[serde(rename = "AssetClassDetailedSubProductType15Code")]
	pub asset_class_detailed_sub_product_type15_code: String,
}


// AssetClassDetailedSubProductType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType1Code {
	#[validate(enumerate = ["FWHT", "SOYB", "RPSD", "OTHR", "CORN", "RICE"])]
	#[serde(rename = "AssetClassDetailedSubProductType1Code")]
	pub asset_class_detailed_sub_product_type1_code: String,
}


// AssetClassDetailedSubProductType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType2Code {
	#[validate(enumerate = ["ROBU", "CCOA", "BRWN", "WHSG", "OTHR"])]
	#[serde(rename = "AssetClassDetailedSubProductType2Code")]
	pub asset_class_detailed_sub_product_type2_code: String,
}


// AssetClassDetailedSubProductType4Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType4Code {
	#[validate(enumerate = ["LAMP"])]
	#[serde(rename = "AssetClassDetailedSubProductType4Code")]
	pub asset_class_detailed_sub_product_type4_code: String,
}


// AssetClassDetailedSubProductType5Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType5Code {
	#[validate(enumerate = ["BSLD", "FITR", "PKLD", "OFFP", "OTHR"])]
	#[serde(rename = "AssetClassDetailedSubProductType5Code")]
	pub asset_class_detailed_sub_product_type5_code: String,
}


// AssetClassDetailedSubProductType6Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType6Code {
	#[validate(enumerate = ["GASP", "LNGG", "NCGG", "TTFG", "NBPG"])]
	#[serde(rename = "AssetClassDetailedSubProductType6Code")]
	pub asset_class_detailed_sub_product_type6_code: String,
}


// AssetClassDetailedSubProductType7Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassDetailedSubProductType7Code {
	#[validate(enumerate = ["BAKK", "BDSL", "BRNT", "BRNX", "CNDA", "COND", "DSEL", "DUBA", "ESPO", "ETHA", "FUEL", "FOIL", "GOIL", "GSLN", "HEAT", "JTFL", "KERO", "LLSO", "MARS", "NAPH", "NGLO", "TAPI", "WTIO", "URAL"])]
	#[serde(rename = "AssetClassDetailedSubProductType7Code")]
	pub asset_class_detailed_sub_product_type7_code: String,
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


// AssetClassTransactionType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetClassTransactionType1Code {
	#[validate(enumerate = ["CRCK", "DIFF", "FUTR", "MINI", "OPTN", "OTCT", "ORIT", "SWAP", "TAPO", "OTHR"])]
	#[serde(rename = "AssetClassTransactionType1Code")]
	pub asset_class_transaction_type1_code: String,
}


// AssetFXSubProductType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetFXSubProductType1Code {
	#[validate(enumerate = ["FXCR", "FXEM", "FXMJ"])]
	#[serde(rename = "AssetFXSubProductType1Code")]
	pub asset_fx_sub_product_type1_code: String,
}


// AssetPriceType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssetPriceType1Code {
	#[validate(enumerate = ["ARGM", "BLTC", "EXOF", "GBCL", "IHSM", "OTHR", "PLAT"])]
	#[serde(rename = "AssetPriceType1Code")]
	pub asset_price_type1_code: String,
}


// BenchmarkCurveName2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BenchmarkCurveName2Code {
	#[validate(enumerate = ["WIBO", "TREA", "TIBO", "TLBO", "SWAP", "STBO", "PRBO", "PFAN", "NIBO", "MAAA", "MOSP", "LIBO", "LIBI", "JIBA", "ISDA", "GCFR", "FUSW", "EUCH", "EUUS", "EURI", "EONS", "EONA", "CIBO", "CDOR", "BUBO", "BBSW"])]
	#[serde(rename = "BenchmarkCurveName2Code")]
	pub benchmark_curve_name2_code: String,
}


// BenchmarkCurveName5Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BenchmarkCurveName5Choice {
	#[serde(rename = "Indx")]
	pub indx: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
}


// BenchmarkCurveName6Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BenchmarkCurveName6Choice {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[serde(rename = "Indx")]
	pub indx: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
}


// CFIOct2015Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[validate(pattern = "[A-Z]{6,6}")]
	#[serde(rename = "CFIOct2015Identifier")]
	pub cfi_oct2015_identifier: String,
}


// CountryCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CountryCode {
	#[validate(pattern = "[A-Z]{2,2}")]
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// DebtInstrument2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DebtInstrument2 {
	#[validate]
	#[serde(rename = "TtlIssdNmnlAmt")]
	pub ttl_issd_nmnl_amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: Option<String>,
	#[validate]
	#[serde(rename = "NmnlValPerUnit")]
	pub nmnl_val_per_unit: ActiveOrHistoricCurrencyAndAmount,
	#[validate]
	#[serde(rename = "IntrstRate")]
	pub intrst_rate: InterestRate6Choice,
	#[serde(rename = "DebtSnrty")]
	pub debt_snrty: Option<String>,
}


// DebtInstrumentSeniorityType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DebtInstrumentSeniorityType1Code {
	#[validate(enumerate = ["SBOD", "SNDB", "MZZD", "JUND"])]
	#[serde(rename = "DebtInstrumentSeniorityType1Code")]
	pub debt_instrument_seniority_type1_code: String,
}


// DecimalNumber ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "DecimalNumber")]
	pub decimal_number: f64,
}


// DerivativeCommodity2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DerivativeCommodity2 {
	#[validate]
	#[serde(rename = "Pdct")]
	pub pdct: AssetClassCommodity3Choice,
	#[serde(rename = "TxTp")]
	pub tx_tp: Option<String>,
	#[serde(rename = "FnlPricTp")]
	pub fnl_pric_tp: Option<String>,
}


// DerivativeForeignExchange3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DerivativeForeignExchange3 {
	#[serde(rename = "FxTp")]
	pub fx_tp: Option<String>,
	#[serde(rename = "OthrNtnlCcy")]
	pub othr_ntnl_ccy: Option<String>,
}


// DerivativeInstrument5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DerivativeInstrument5 {
	#[serde(rename = "XpryDt")]
	pub xpry_dt: Option<String>,
	#[serde(rename = "PricMltplr")]
	pub pric_mltplr: Option<f64>,
	#[validate]
	#[serde(rename = "UndrlygInstrm")]
	pub undrlyg_instrm: Option<FinancialInstrumentIdentification5Choice>,
	#[serde(rename = "OptnTp")]
	pub optn_tp: Option<String>,
	#[validate]
	#[serde(rename = "StrkPric")]
	pub strk_pric: Option<SecuritiesTransactionPrice4Choice>,
	#[serde(rename = "OptnExrcStyle")]
	pub optn_exrc_style: Option<String>,
	#[serde(rename = "DlvryTp")]
	pub dlvry_tp: Option<String>,
	#[validate]
	#[serde(rename = "AsstClssSpcfcAttrbts")]
	pub asst_clss_spcfc_attrbts: Option<AssetClass2>,
}


// DerivativeInterest3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DerivativeInterest3 {
	#[validate]
	#[serde(rename = "IntrstRate")]
	pub intrst_rate: FloatingInterestRate8,
	#[validate]
	#[serde(rename = "FrstLegIntrstRate")]
	pub frst_leg_intrst_rate: Option<InterestRate8Choice>,
	#[serde(rename = "OthrNtnlCcy")]
	pub othr_ntnl_ccy: Option<String>,
	#[validate]
	#[serde(rename = "OthrLegIntrstRate")]
	pub othr_leg_intrst_rate: Option<InterestRate8Choice>,
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


// EnergyCommodityNaturalGas1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnergyCommodityNaturalGas1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// EnergyCommodityOil1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnergyCommodityOil1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// EnergyCommodityRenewableEnergy1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnergyCommodityRenewableEnergy1 {
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


// EnvironmentalCommodityEmission1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnvironmentalCommodityEmission1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// EnvironmentalCommodityWeather1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EnvironmentalCommodityWeather1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
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


// FinancialInstrument48Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstrument48Choice {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
	#[validate]
	#[serde(rename = "Indx")]
	pub indx: Option<FinancialInstrument58>,
}


// FinancialInstrument53 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstrument53 {
	#[serde(rename = "ISIN")]
	pub isin: Option<Vec<String>>,
	#[serde(rename = "LEI")]
	pub lei: Option<Vec<String>>,
}


// FinancialInstrument58 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstrument58 {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[validate]
	#[serde(rename = "Nm")]
	pub nm: FloatingInterestRate8,
}


// FinancialInstrumentIdentification5Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstrumentIdentification5Choice {
	#[validate]
	#[serde(rename = "Sngl")]
	pub sngl: Option<FinancialInstrument48Choice>,
	#[validate]
	#[serde(rename = "Bskt")]
	pub bskt: Option<FinancialInstrument53>,
}


// FinancialInstrumentReportingReferenceDataDeltaReportV03 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstrumentReportingReferenceDataDeltaReportV03 {
	#[validate]
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: SecuritiesMarketReportHeader1,
	#[validate]
	#[serde(rename = "FinInstrm")]
	pub fin_instrm: Option<Vec<SecuritiesReferenceDeltaStatusReport5Choice>>,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// FloatingInterestRate6 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FloatingInterestRate6 {
	#[validate]
	#[serde(rename = "RefRate")]
	pub ref_rate: BenchmarkCurveName6Choice,
	#[validate]
	#[serde(rename = "Term")]
	pub term: InterestRateContractTerm2,
	#[serde(rename = "BsisPtSprd")]
	pub bsis_pt_sprd: f64,
}


// FloatingInterestRate8 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FloatingInterestRate8 {
	#[validate]
	#[serde(rename = "RefRate")]
	pub ref_rate: BenchmarkCurveName5Choice,
	#[validate]
	#[serde(rename = "Term")]
	pub term: Option<InterestRateContractTerm2>,
}


// FreightCommodityContainerShip1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FreightCommodityContainerShip1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
}


// FreightCommodityDry1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FreightCommodityDry1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
}


// FreightCommodityWet1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FreightCommodityWet1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: String,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: String,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: Option<String>,
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


// InterestRate6Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InterestRate6Choice {
	#[serde(rename = "Fxd")]
	pub fxd: Option<f64>,
	#[validate]
	#[serde(rename = "Fltg")]
	pub fltg: Option<FloatingInterestRate6>,
}


// InterestRate8Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InterestRate8Choice {
	#[serde(rename = "Fxd")]
	pub fxd: Option<f64>,
	#[validate]
	#[serde(rename = "Fltg")]
	pub fltg: Option<FloatingInterestRate8>,
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


// MICIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MICIdentifier {
	#[validate(pattern = "[A-Z0-9]{4,4}")]
	#[serde(rename = "MICIdentifier")]
	pub mic_identifier: String,
}


// Max25Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max25Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 25)]
	#[serde(rename = "Max25Text")]
	pub max25_text: String,
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


// Max50Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max50Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 50)]
	#[serde(rename = "Max50Text")]
	pub max50_text: String,
}


// Max5Number ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max5Number {
	#[serde(rename = "Max5Number")]
	pub max5_number: f64,
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


// NonNegativeDecimalNumber ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NonNegativeDecimalNumber {
	#[serde(rename = "NonNegativeDecimalNumber")]
	pub non_negative_decimal_number: f64,
}


// OptionStyle7Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OptionStyle7Code {
	#[validate(enumerate = ["AMER", "ASIA", "BERM", "EURO", "OTHR"])]
	#[serde(rename = "OptionStyle7Code")]
	pub option_style7_code: String,
}


// OptionType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OptionType2Code {
	#[validate(enumerate = ["CALL", "PUTO", "OTHR"])]
	#[serde(rename = "OptionType2Code")]
	pub option_type2_code: String,
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


// PercentageRate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// Period2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Period2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// Period4Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Period4Choice {
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "FrDt")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt")]
	pub to_dt: Option<String>,
	#[validate]
	#[serde(rename = "FrDtToDt")]
	pub fr_dt_to_dt: Option<Period2>,
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


// RateBasis1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RateBasis1Code {
	#[validate(enumerate = ["DAYS", "MNTH", "WEEK", "YEAR"])]
	#[serde(rename = "RateBasis1Code")]
	pub rate_basis1_code: String,
}


// RecordTechnicalData4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RecordTechnicalData4 {
	#[serde(rename = "IncnsstncyInd")]
	pub incnsstncy_ind: Option<bool>,
	#[serde(rename = "LastUpd")]
	pub last_upd: Option<String>,
	#[serde(rename = "SubmissnDtTm")]
	pub submissn_dt_tm: Option<String>,
	#[serde(rename = "RlvntCmptntAuthrty")]
	pub rlvnt_cmptnt_authrty: Option<String>,
	#[validate]
	#[serde(rename = "PblctnPrd")]
	pub pblctn_prd: Option<Period4Choice>,
	#[serde(rename = "NvrPblshd")]
	pub nvr_pblshd: Option<bool>,
	#[serde(rename = "RlvntTradgVn")]
	pub rlvnt_tradg_vn: Option<String>,
}


// SecuritiesMarketReportHeader1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesMarketReportHeader1 {
	#[validate]
	#[serde(rename = "RptgNtty")]
	pub rptg_ntty: TradingVenueIdentification1Choice,
	#[validate]
	#[serde(rename = "RptgPrd")]
	pub rptg_prd: Period4Choice,
	#[serde(rename = "SubmissnDtTm")]
	pub submissn_dt_tm: Option<String>,
}


// SecuritiesReferenceDataReport6 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesReferenceDataReport6 {
	#[serde(rename = "TechRcrdId")]
	pub tech_rcrd_id: Option<String>,
	#[validate]
	#[serde(rename = "FinInstrmGnlAttrbts")]
	pub fin_instrm_gnl_attrbts: SecurityInstrumentDescription9,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[validate]
	#[serde(rename = "TradgVnRltdAttrbts")]
	pub tradg_vn_rltd_attrbts: Vec<TradingVenueAttributes1>,
	#[validate]
	#[serde(rename = "DebtInstrmAttrbts")]
	pub debt_instrm_attrbts: Option<DebtInstrument2>,
	#[validate]
	#[serde(rename = "DerivInstrmAttrbts")]
	pub deriv_instrm_attrbts: Option<DerivativeInstrument5>,
	#[validate]
	#[serde(rename = "TechAttrbts")]
	pub tech_attrbts: Option<RecordTechnicalData4>,
}


// SecuritiesReferenceDataReport7 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesReferenceDataReport7 {
	#[serde(rename = "TechRcrdId")]
	pub tech_rcrd_id: Option<String>,
	#[validate]
	#[serde(rename = "FinInstrmGnlAttrbts")]
	pub fin_instrm_gnl_attrbts: SecurityInstrumentDescription17,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
	#[validate]
	#[serde(rename = "TradgVnRltdAttrbts")]
	pub tradg_vn_rltd_attrbts: Vec<TradingVenueAttributes2>,
	#[validate]
	#[serde(rename = "DebtInstrmAttrbts")]
	pub debt_instrm_attrbts: Option<DebtInstrument2>,
	#[validate]
	#[serde(rename = "DerivInstrmAttrbts")]
	pub deriv_instrm_attrbts: Option<DerivativeInstrument5>,
	#[validate]
	#[serde(rename = "TechAttrbts")]
	pub tech_attrbts: Option<RecordTechnicalData4>,
}


// SecuritiesReferenceDeltaStatusReport5Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesReferenceDeltaStatusReport5Choice {
	#[validate]
	#[serde(rename = "ModfdRcrd")]
	pub modfd_rcrd: Option<SecuritiesReferenceDataReport6>,
	#[validate]
	#[serde(rename = "NewRcrd")]
	pub new_rcrd: Option<SecuritiesReferenceDataReport6>,
	#[validate]
	#[serde(rename = "TermntdRcrd")]
	pub termntd_rcrd: Option<SecuritiesReferenceDataReport6>,
	#[validate]
	#[serde(rename = "CancRcrd")]
	pub canc_rcrd: Option<SecuritiesReferenceDataReport7>,
}


// SecuritiesTransactionPrice1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice1 {
	#[serde(rename = "Pdg")]
	pub pdg: String,
	#[serde(rename = "Ccy")]
	pub ccy: Option<String>,
}


// SecuritiesTransactionPrice2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice2Choice {
	#[validate]
	#[serde(rename = "MntryVal")]
	pub mntry_val: Option<AmountAndDirection61>,
	#[serde(rename = "Pctg")]
	pub pctg: Option<f64>,
	#[serde(rename = "Yld")]
	pub yld: Option<f64>,
	#[serde(rename = "BsisPts")]
	pub bsis_pts: Option<f64>,
}


// SecuritiesTransactionPrice4Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice4Choice {
	#[validate]
	#[serde(rename = "Pric")]
	pub pric: Option<SecuritiesTransactionPrice2Choice>,
	#[validate]
	#[serde(rename = "NoPric")]
	pub no_pric: Option<SecuritiesTransactionPrice1>,
}


// SecurityInstrumentDescription17 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityInstrumentDescription17 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "FullNm")]
	pub full_nm: Option<String>,
	#[serde(rename = "ShrtNm")]
	pub shrt_nm: Option<String>,
	#[serde(rename = "ClssfctnTp")]
	pub clssfctn_tp: Option<String>,
	#[serde(rename = "NtnlCcy")]
	pub ntnl_ccy: Option<String>,
	#[serde(rename = "CmmdtyDerivInd")]
	pub cmmdty_deriv_ind: Option<bool>,
}


// SecurityInstrumentDescription9 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityInstrumentDescription9 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "FullNm")]
	pub full_nm: String,
	#[serde(rename = "ShrtNm")]
	pub shrt_nm: Option<String>,
	#[serde(rename = "ClssfctnTp")]
	pub clssfctn_tp: String,
	#[serde(rename = "NtnlCcy")]
	pub ntnl_ccy: String,
	#[serde(rename = "CmmdtyDerivInd")]
	pub cmmdty_deriv_ind: bool,
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


// TradingVenue2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradingVenue2Code {
	#[validate(enumerate = ["APPA", "CTPS"])]
	#[serde(rename = "TradingVenue2Code")]
	pub trading_venue2_code: String,
}


// TradingVenueAttributes1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradingVenueAttributes1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "IssrReq")]
	pub issr_req: bool,
	#[serde(rename = "AdmssnApprvlDtByIssr")]
	pub admssn_apprvl_dt_by_issr: Option<String>,
	#[serde(rename = "ReqForAdmssnDt")]
	pub req_for_admssn_dt: Option<String>,
	#[serde(rename = "FrstTradDt")]
	pub frst_trad_dt: Option<String>,
	#[serde(rename = "TermntnDt")]
	pub termntn_dt: Option<String>,
}


// TradingVenueAttributes2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradingVenueAttributes2 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "IssrReq")]
	pub issr_req: Option<bool>,
	#[serde(rename = "AdmssnApprvlDtByIssr")]
	pub admssn_apprvl_dt_by_issr: Option<String>,
	#[serde(rename = "ReqForAdmssnDt")]
	pub req_for_admssn_dt: Option<String>,
	#[serde(rename = "FrstTradDt")]
	pub frst_trad_dt: Option<String>,
	#[serde(rename = "TermntnDt")]
	pub termntn_dt: Option<String>,
}


// TradingVenueIdentification1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradingVenueIdentification1Choice {
	#[serde(rename = "MktIdCd")]
	pub mkt_id_cd: Option<String>,
	#[serde(rename = "NtlCmptntAuthrty")]
	pub ntl_cmptnt_authrty: Option<String>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<TradingVenueIdentification2>,
}


// TradingVenueIdentification2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradingVenueIdentification2 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Tp")]
	pub tp: String,
}


// TrueFalseIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}
