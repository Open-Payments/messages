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


// ActiveOrHistoricCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
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


// AgriculturalCommodityDairy1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityDairy1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType20Code,
}


// AgriculturalCommodityForestry1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityForestry1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType21Code,
}


// AgriculturalCommodityGrain2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityGrain2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType5Code,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: AssetClassDetailedSubProductType30Code,
}


// AgriculturalCommodityLiveStock1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityLiveStock1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType22Code,
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


// AgriculturalCommodityOliveOil2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityOliveOil2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType3Code,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: AssetClassDetailedSubProductType29Code,
}


// AgriculturalCommodityOther1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType49Code,
}


// AgriculturalCommodityPotato1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityPotato1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType45Code,
}


// AgriculturalCommoditySeafood1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommoditySeafood1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType23Code,
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


// AmountAndDirection107 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection107 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAnd20DecimalAmount,
	#[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
	pub sgn: Option<bool>,
}


// AmountAndDirection53 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection53 {
	#[serde(rename = "Amt")]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
	pub sgn: Option<bool>,
}


// AmountHaircutMargin1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountHaircutMargin1 {
	#[serde(rename = "Amt")]
	pub amt: AmountAndDirection53,
	#[serde(rename = "HrcutOrMrgn", skip_serializing_if = "Option::is_none")]
	pub hrcut_or_mrgn: Option<f64>,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
}


// AssetClassCommodity5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodity5Choice {
	#[serde(rename = "Agrcltrl", skip_serializing_if = "Option::is_none")]
	pub agrcltrl: Option<AssetClassCommodityAgricultural5Choice>,
	#[serde(rename = "Nrgy", skip_serializing_if = "Option::is_none")]
	pub nrgy: Option<AssetClassCommodityEnergy2Choice>,
	#[serde(rename = "Envttl", skip_serializing_if = "Option::is_none")]
	pub envttl: Option<AssetClassCommodityEnvironmental2Choice>,
	#[serde(rename = "Frtlzr", skip_serializing_if = "Option::is_none")]
	pub frtlzr: Option<AssetClassCommodityFertilizer3Choice>,
	#[serde(rename = "Frght", skip_serializing_if = "Option::is_none")]
	pub frght: Option<AssetClassCommodityFreight3Choice>,
	#[serde(rename = "IndstrlPdct", skip_serializing_if = "Option::is_none")]
	pub indstrl_pdct: Option<AssetClassCommodityIndustrialProduct1Choice>,
	#[serde(rename = "Metl", skip_serializing_if = "Option::is_none")]
	pub metl: Option<AssetClassCommodityMetal1Choice>,
	#[serde(rename = "OthrC10", skip_serializing_if = "Option::is_none")]
	pub othr_c10: Option<AssetClassCommodityOtherC102Choice>,
	#[serde(rename = "Ppr", skip_serializing_if = "Option::is_none")]
	pub ppr: Option<AssetClassCommodityPaper3Choice>,
	#[serde(rename = "Plprpln", skip_serializing_if = "Option::is_none")]
	pub plprpln: Option<AssetClassCommodityPolypropylene3Choice>,
	#[serde(rename = "Infltn", skip_serializing_if = "Option::is_none")]
	pub infltn: Option<AssetClassCommodityInflation1>,
	#[serde(rename = "MultiCmmdtyExtc", skip_serializing_if = "Option::is_none")]
	pub multi_cmmdty_extc: Option<AssetClassCommodityMultiCommodityExotic1>,
	#[serde(rename = "OffclEcnmcSttstcs", skip_serializing_if = "Option::is_none")]
	pub offcl_ecnmc_sttstcs: Option<AssetClassCommodityOfficialEconomicStatistics1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<AssetClassCommodityOther1>,
}


// AssetClassCommodityAgricultural5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityAgricultural5Choice {
	#[serde(rename = "GrnOilSeed", skip_serializing_if = "Option::is_none")]
	pub grn_oil_seed: Option<AgriculturalCommodityOilSeed1>,
	#[serde(rename = "Soft", skip_serializing_if = "Option::is_none")]
	pub soft: Option<AgriculturalCommoditySoft1>,
	#[serde(rename = "Ptt", skip_serializing_if = "Option::is_none")]
	pub ptt: Option<AgriculturalCommodityPotato1>,
	#[serde(rename = "OlvOil", skip_serializing_if = "Option::is_none")]
	pub olv_oil: Option<AgriculturalCommodityOliveOil2>,
	#[serde(rename = "Dairy", skip_serializing_if = "Option::is_none")]
	pub dairy: Option<AgriculturalCommodityDairy1>,
	#[serde(rename = "Frstry", skip_serializing_if = "Option::is_none")]
	pub frstry: Option<AgriculturalCommodityForestry1>,
	#[serde(rename = "Sfd", skip_serializing_if = "Option::is_none")]
	pub sfd: Option<AgriculturalCommoditySeafood1>,
	#[serde(rename = "LiveStock", skip_serializing_if = "Option::is_none")]
	pub live_stock: Option<AgriculturalCommodityLiveStock1>,
	#[serde(rename = "Grn", skip_serializing_if = "Option::is_none")]
	pub grn: Option<AgriculturalCommodityGrain2>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<AgriculturalCommodityOther1>,
}


// AssetClassCommodityEnergy2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityEnergy2Choice {
	#[serde(rename = "Elctrcty", skip_serializing_if = "Option::is_none")]
	pub elctrcty: Option<EnergyCommodityElectricity1>,
	#[serde(rename = "NtrlGas", skip_serializing_if = "Option::is_none")]
	pub ntrl_gas: Option<EnergyCommodityNaturalGas2>,
	#[serde(rename = "Oil", skip_serializing_if = "Option::is_none")]
	pub oil: Option<EnergyCommodityOil2>,
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
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<EnergyCommodityOther1>,
}


// AssetClassCommodityEnvironmental2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityEnvironmental2Choice {
	#[serde(rename = "Emssns", skip_serializing_if = "Option::is_none")]
	pub emssns: Option<EnvironmentalCommodityEmission2>,
	#[serde(rename = "Wthr", skip_serializing_if = "Option::is_none")]
	pub wthr: Option<EnvironmentalCommodityWeather1>,
	#[serde(rename = "CrbnRltd", skip_serializing_if = "Option::is_none")]
	pub crbn_rltd: Option<EnvironmentalCommodityCarbonRelated1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<EnvironmentCommodityOther1>,
}


// AssetClassCommodityFertilizer3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityFertilizer3Choice {
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
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<FertilizerCommodityOther1>,
}


// AssetClassCommodityFreight3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityFreight3Choice {
	#[serde(rename = "Dry", skip_serializing_if = "Option::is_none")]
	pub dry: Option<FreightCommodityDry2>,
	#[serde(rename = "Wet", skip_serializing_if = "Option::is_none")]
	pub wet: Option<FreightCommodityWet2>,
	#[serde(rename = "CntnrShip", skip_serializing_if = "Option::is_none")]
	pub cntnr_ship: Option<FreightCommodityContainerShip1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<FreightCommodityOther1>,
}


// AssetClassCommodityIndustrialProduct1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityIndustrialProduct1Choice {
	#[serde(rename = "Cnstrctn", skip_serializing_if = "Option::is_none")]
	pub cnstrctn: Option<IndustrialProductCommodityConstruction1>,
	#[serde(rename = "Manfctg", skip_serializing_if = "Option::is_none")]
	pub manfctg: Option<IndustrialProductCommodityManufacturing1>,
}


// AssetClassCommodityInflation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityInflation1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType12Code,
}


// AssetClassCommodityMetal1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityMetal1Choice {
	#[serde(rename = "NonPrcs", skip_serializing_if = "Option::is_none")]
	pub non_prcs: Option<MetalCommodityNonPrecious1>,
	#[serde(rename = "Prcs", skip_serializing_if = "Option::is_none")]
	pub prcs: Option<MetalCommodityPrecious1>,
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


// AssetClassCommodityOtherC102Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityOtherC102Choice {
	#[serde(rename = "Dlvrbl", skip_serializing_if = "Option::is_none")]
	pub dlvrbl: Option<OtherC10CommodityDeliverable2>,
	#[serde(rename = "NonDlvrbl", skip_serializing_if = "Option::is_none")]
	pub non_dlvrbl: Option<OtherC10CommodityNonDeliverable2>,
}


// AssetClassCommodityPaper3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityPaper3Choice {
	#[serde(rename = "CntnrBrd", skip_serializing_if = "Option::is_none")]
	pub cntnr_brd: Option<PaperCommodityContainerBoard1>,
	#[serde(rename = "Nwsprnt", skip_serializing_if = "Option::is_none")]
	pub nwsprnt: Option<PaperCommodityNewsprint1>,
	#[serde(rename = "Pulp", skip_serializing_if = "Option::is_none")]
	pub pulp: Option<PaperCommodityPulp1>,
	#[serde(rename = "RcvrdPpr", skip_serializing_if = "Option::is_none")]
	pub rcvrd_ppr: Option<PaperCommodityRecoveredPaper1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<PaperCommodityRecoveredPaper2>,
}


// AssetClassCommodityPolypropylene3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityPolypropylene3Choice {
	#[serde(rename = "Plstc", skip_serializing_if = "Option::is_none")]
	pub plstc: Option<PolypropyleneCommodityPlastic1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<PolypropyleneCommodityOther1>,
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


// AssetClassSubProductType38Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType38Code {
	#[default]
	#[serde(rename = "RCVP")]
	CodeRCVP,
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


// AssetClassSubProductType47Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType47Code {
	#[default]
	#[serde(rename = "DLVR")]
	CodeDLVR,
}


// AssetClassSubProductType48Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassSubProductType48Code {
	#[default]
	#[serde(rename = "NDLV")]
	CodeNDLV,
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


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BaseOneRate {
	#[serde(rename = "$value")]
	pub base_one_rate: f64,
}


// BenchmarkCurveName10Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkCurveName10Choice {
	#[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
	pub indx: Option<BenchmarkCurveName3Code>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max350Text>,
}


// BenchmarkCurveName3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BenchmarkCurveName3Code {
	#[default]
	#[serde(rename = "ESTR")]
	CodeESTR,
	#[serde(rename = "BBSW")]
	CodeBBSW,
	#[serde(rename = "BUBO")]
	CodeBUBO,
	#[serde(rename = "CDOR")]
	CodeCDOR,
	#[serde(rename = "CIBO")]
	CodeCIBO,
	#[serde(rename = "EONA")]
	CodeEONA,
	#[serde(rename = "EONS")]
	CodeEONS,
	#[serde(rename = "EURI")]
	CodeEURI,
	#[serde(rename = "EUUS")]
	CodeEUUS,
	#[serde(rename = "EUCH")]
	CodeEUCH,
	#[serde(rename = "FUSW")]
	CodeFUSW,
	#[serde(rename = "GCFR")]
	CodeGCFR,
	#[serde(rename = "ISDA")]
	CodeISDA,
	#[serde(rename = "JIBA")]
	CodeJIBA,
	#[serde(rename = "LIBI")]
	CodeLIBI,
	#[serde(rename = "LIBO")]
	CodeLIBO,
	#[serde(rename = "MOSP")]
	CodeMOSP,
	#[serde(rename = "MAAA")]
	CodeMAAA,
	#[serde(rename = "NIBO")]
	CodeNIBO,
	#[serde(rename = "PFAN")]
	CodePFAN,
	#[serde(rename = "PRBO")]
	CodePRBO,
	#[serde(rename = "STBO")]
	CodeSTBO,
	#[serde(rename = "SWAP")]
	CodeSWAP,
	#[serde(rename = "TLBO")]
	CodeTLBO,
	#[serde(rename = "TIBO")]
	CodeTIBO,
	#[serde(rename = "TREA")]
	CodeTREA,
	#[serde(rename = "WIBO")]
	CodeWIBO,
	#[serde(rename = "SOFR")]
	CodeSOFR,
	#[serde(rename = "SONA")]
	CodeSONA,
}


// Branch5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Branch5Choice {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
}


// Branch6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Branch6Choice {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<PartyIdentification236Choice>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
}


// CFIOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[serde(rename = "$value")]
	pub cfi_oct2015_identifier: String,
}


// Cleared16Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Cleared16Choice {
	#[serde(rename = "Clrd", skip_serializing_if = "Option::is_none")]
	pub clrd: Option<ClearingPartyAndTime14>,
	#[serde(rename = "NonClrd", skip_serializing_if = "Option::is_none")]
	pub non_clrd: Option<NoReasonCode>,
}


// ClearingPartyAndTime14 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClearingPartyAndTime14 {
	#[serde(rename = "CCP", skip_serializing_if = "Option::is_none")]
	pub ccp: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "ClrDtTm", skip_serializing_if = "Option::is_none")]
	pub clr_dt_tm: Option<String>,
	#[serde(rename = "RptTrckgNb", skip_serializing_if = "Option::is_none")]
	pub rpt_trckg_nb: Option<Max52Text>,
	#[serde(rename = "PrtflCd", skip_serializing_if = "Option::is_none")]
	pub prtfl_cd: Option<Max52Text>,
}


// Collateral52 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Collateral52 {
	#[serde(rename = "CollValDt", skip_serializing_if = "Option::is_none")]
	pub coll_val_dt: Option<String>,
	#[serde(rename = "AsstTp", skip_serializing_if = "Option::is_none")]
	pub asst_tp: Option<CollateralType21>,
	#[serde(rename = "NetXpsrCollstnInd", skip_serializing_if = "Option::is_none")]
	pub net_xpsr_collstn_ind: Option<bool>,
	#[serde(rename = "BsktIdr", skip_serializing_if = "Option::is_none")]
	pub bskt_idr: Option<SecurityIdentification26Choice>,
}


// CollateralDeliveryMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CollateralDeliveryMethod1Code {
	#[default]
	#[serde(rename = "SICA")]
	CodeSICA,
	#[serde(rename = "SIUR")]
	CodeSIUR,
	#[serde(rename = "TTCA")]
	CodeTTCA,
}


// CollateralFlag13Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralFlag13Choice {
	#[serde(rename = "Collsd", skip_serializing_if = "Option::is_none")]
	pub collsd: Option<CollaterisedData12>,
	#[serde(rename = "Uncollsd", skip_serializing_if = "Option::is_none")]
	pub uncollsd: Option<NoReasonCode>,
}


// CollateralQualityType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CollateralQualityType1Code {
	#[default]
	#[serde(rename = "INVG")]
	CodeINVG,
	#[serde(rename = "NIVG")]
	CodeNIVG,
	#[serde(rename = "NOTR")]
	CodeNOTR,
	#[serde(rename = "NOAP")]
	CodeNOAP,
}


// CollateralRole1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CollateralRole1Code {
	#[default]
	#[serde(rename = "GIVE")]
	CodeGIVE,
	#[serde(rename = "TAKE")]
	CodeTAKE,
}


// CollateralType21 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralType21 {
	#[serde(rename = "Scty", skip_serializing_if = "Option::is_none")]
	pub scty: Option<Vec<Security52>>,
	#[serde(rename = "Csh", skip_serializing_if = "Option::is_none")]
	pub csh: Option<Vec<AmountHaircutMargin1>>,
	#[serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none")]
	pub cmmdty: Option<Vec<Commodity43>>,
}


// CollaterisedData12 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollaterisedData12 {
	#[serde(rename = "CollValDt", skip_serializing_if = "Option::is_none")]
	pub coll_val_dt: Option<String>,
	#[serde(rename = "AsstTp", skip_serializing_if = "Option::is_none")]
	pub asst_tp: Option<CollateralType21>,
	#[serde(rename = "NetXpsrCollstnInd", skip_serializing_if = "Option::is_none")]
	pub net_xpsr_collstn_ind: Option<bool>,
	#[serde(rename = "BsktIdr", skip_serializing_if = "Option::is_none")]
	pub bskt_idr: Option<SecurityIdentification26Choice>,
}


// Commodity43 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Commodity43 {
	#[serde(rename = "Clssfctn", skip_serializing_if = "Option::is_none")]
	pub clssfctn: Option<AssetClassCommodity5Choice>,
	#[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
	pub qty: Option<Quantity17>,
	#[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
	pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
	#[serde(rename = "MktVal", skip_serializing_if = "Option::is_none")]
	pub mkt_val: Option<AmountAndDirection53>,
}


// ContractModification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractModification3 {
	#[serde(rename = "ActnTp")]
	pub actn_tp: TransactionOperationType6Code,
	#[serde(rename = "Lvl", skip_serializing_if = "Option::is_none")]
	pub lvl: Option<ModificationLevel1Code>,
}


// ContractTerm7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContractTerm7Choice {
	#[serde(rename = "Opn", skip_serializing_if = "Option::is_none")]
	pub opn: Option<FixedOpenTermContract2>,
	#[serde(rename = "Fxd", skip_serializing_if = "Option::is_none")]
	pub fxd: Option<FixedOpenTermContract2>,
}


// CounterpartyData88 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyData88 {
	#[serde(rename = "RptgDtTm")]
	pub rptg_dt_tm: String,
	#[serde(rename = "RptSubmitgNtty")]
	pub rpt_submitg_ntty: OrganisationIdentification15Choice,
	#[serde(rename = "CtrPty")]
	pub ctr_pty: Vec<CounterpartyData89>,
}


// CounterpartyData89 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyData89 {
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: CounterpartyIdentification11,
	#[serde(rename = "OthrCtrPty")]
	pub othr_ctr_pty: CounterpartyIdentification12,
	#[serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none")]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "OthrPtyData", skip_serializing_if = "Option::is_none")]
	pub othr_pty_data: Option<TransactionCounterpartyData11>,
}


// CounterpartyIdentification11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyIdentification11 {
	#[serde(rename = "Id")]
	pub id: OrganisationIdentification15Choice,
	#[serde(rename = "Ntr", skip_serializing_if = "Option::is_none")]
	pub ntr: Option<CounterpartyTradeNature7Choice>,
	#[serde(rename = "Brnch", skip_serializing_if = "Option::is_none")]
	pub brnch: Option<Branch5Choice>,
	#[serde(rename = "Sd", skip_serializing_if = "Option::is_none")]
	pub sd: Option<CollateralRole1Code>,
}


// CounterpartyIdentification12 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyIdentification12 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification236Choice,
	#[serde(rename = "Brnch", skip_serializing_if = "Option::is_none")]
	pub brnch: Option<Branch6Choice>,
	#[serde(rename = "CtryCd", skip_serializing_if = "Option::is_none")]
	pub ctry_cd: Option<CountryCode>,
}


// CounterpartyTradeNature7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyTradeNature7Choice {
	#[serde(rename = "FI", skip_serializing_if = "Option::is_none")]
	pub fi: Option<FinancialPartyClassification1>,
	#[serde(rename = "NFI", skip_serializing_if = "Option::is_none")]
	pub nfi: Option<Vec<FinancialPartyClassification2>>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "$value")]
	pub decimal_number: f64,
}


// EnergyCommodityCoal1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityCoal1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType24Code,
}


// EnergyCommodityDistillates1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityDistillates1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType25Code,
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


// EnergyCommodityInterEnergy1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityInterEnergy1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType26Code,
}


// EnergyCommodityLightEnd1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityLightEnd1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType27Code,
}


// EnergyCommodityNaturalGas2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityNaturalGas2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType7Code,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: AssetClassDetailedSubProductType31Code,
}


// EnergyCommodityOil2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityOil2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType8Code,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: AssetClassDetailedSubProductType32Code,
}


// EnergyCommodityOther1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType49Code,
}


// EnergyCommodityRenewableEnergy1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityRenewableEnergy1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType28Code,
}


// EnvironmentCommodityOther1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType3Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType49Code,
}


// EnvironmentalCommodityCarbonRelated1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCommodityCarbonRelated1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType3Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType29Code,
}


// EnvironmentalCommodityEmission2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCommodityEmission2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType3Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType10Code,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: AssetClassDetailedSubProductType8Code,
}


// EnvironmentalCommodityWeather1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCommodityWeather1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType3Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType30Code,
}


// ExternalAgreementType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAgreementType1Code {
	#[serde(rename = "$value")]
	pub external_agreement_type1_code: String,
}


// ExternalSecuritiesLendingType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalSecuritiesLendingType1Code {
	#[serde(rename = "$value")]
	pub external_securities_lending_type1_code: String,
}


// FertilizerCommodityAmmonia1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityAmmonia1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType39Code,
}


// FertilizerCommodityDiammoniumPhosphate1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityDiammoniumPhosphate1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType40Code,
}


// FertilizerCommodityOther1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType49Code,
}


// FertilizerCommodityPotash1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityPotash1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType41Code,
}


// FertilizerCommoditySulphur1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommoditySulphur1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType42Code,
}


// FertilizerCommodityUrea1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityUrea1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType43Code,
}


// FertilizerCommodityUreaAndAmmoniumNitrate1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FertilizerCommodityUreaAndAmmoniumNitrate1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType5Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType44Code,
}


// FinancialPartyClassification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialPartyClassification1 {
	#[serde(rename = "Clssfctn")]
	pub clssfctn: Vec<FinancialPartySectorType2Code>,
	#[serde(rename = "InvstmtFndClssfctn", skip_serializing_if = "Option::is_none")]
	pub invstmt_fnd_clssfctn: Option<FundType2Code>,
}


// FinancialPartyClassification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialPartyClassification2 {
	#[serde(rename = "Clssfctn")]
	pub clssfctn: Vec<NACEDomainIdentifier>,
	#[serde(rename = "InvstmtFndClssfctn", skip_serializing_if = "Option::is_none")]
	pub invstmt_fnd_clssfctn: Option<FundType2Code>,
}


// FinancialPartySectorType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FinancialPartySectorType2Code {
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
}


// FixedOpenTermContract2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FixedOpenTermContract2 {
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "TermntnOptn", skip_serializing_if = "Option::is_none")]
	pub termntn_optn: Option<RepoTerminationOption2Code>,
}


// FixedRate11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FixedRate11 {
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "DayCntBsis", skip_serializing_if = "Option::is_none")]
	pub day_cnt_bsis: Option<InterestComputationMethodFormat6Choice>,
}


// FloatingInterestRate22 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FloatingInterestRate22 {
	#[serde(rename = "RefRate", skip_serializing_if = "Option::is_none")]
	pub ref_rate: Option<BenchmarkCurveName10Choice>,
	#[serde(rename = "Term", skip_serializing_if = "Option::is_none")]
	pub term: Option<InterestRateContractTerm2>,
	#[serde(rename = "PmtFrqcy", skip_serializing_if = "Option::is_none")]
	pub pmt_frqcy: Option<InterestRateContractTerm2>,
	#[serde(rename = "RstFrqcy", skip_serializing_if = "Option::is_none")]
	pub rst_frqcy: Option<InterestRateContractTerm2>,
	#[serde(rename = "Sprd", skip_serializing_if = "Option::is_none")]
	pub sprd: Option<SecuritiesTransactionPrice18Choice>,
	#[serde(rename = "RateAdjstmnt", skip_serializing_if = "Option::is_none")]
	pub rate_adjstmnt: Option<Vec<RateAdjustment1>>,
	#[serde(rename = "DayCntBsis", skip_serializing_if = "Option::is_none")]
	pub day_cnt_bsis: Option<InterestComputationMethodFormat6Choice>,
}


// FreightCommodityContainerShip1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityContainerShip1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType4Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType46Code,
}


// FreightCommodityDry2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityDry2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType4Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType31Code,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: AssetClassDetailedSubProductType33Code,
}


// FreightCommodityOther1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType4Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType49Code,
}


// FreightCommodityWet2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityWet2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType4Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType32Code,
	#[serde(rename = "AddtlSubPdct")]
	pub addtl_sub_pdct: AssetClassDetailedSubProductType34Code,
}


// FundType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FundType2Code {
	#[default]
	#[serde(rename = "ETFT")]
	CodeETFT,
	#[serde(rename = "MMFT")]
	CodeMMFT,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "REIT")]
	CodeREIT,
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


// IndustrialProductCommodityConstruction1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndustrialProductCommodityConstruction1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType6Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType33Code>,
}


// IndustrialProductCommodityManufacturing1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IndustrialProductCommodityManufacturing1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType6Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType34Code>,
}


// InterestComputationMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InterestComputationMethod1Code {
	#[default]
	#[serde(rename = "A001")]
	CodeA001,
	#[serde(rename = "A002")]
	CodeA002,
	#[serde(rename = "A003")]
	CodeA003,
	#[serde(rename = "A004")]
	CodeA004,
	#[serde(rename = "A005")]
	CodeA005,
	#[serde(rename = "A006")]
	CodeA006,
	#[serde(rename = "A007")]
	CodeA007,
	#[serde(rename = "A008")]
	CodeA008,
	#[serde(rename = "A009")]
	CodeA009,
	#[serde(rename = "A010")]
	CodeA010,
	#[serde(rename = "A011")]
	CodeA011,
	#[serde(rename = "A012")]
	CodeA012,
	#[serde(rename = "A013")]
	CodeA013,
	#[serde(rename = "A014")]
	CodeA014,
}


// InterestComputationMethodFormat6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestComputationMethodFormat6Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InterestComputationMethod1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// InterestRate27Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRate27Choice {
	#[serde(rename = "Fxd", skip_serializing_if = "Option::is_none")]
	pub fxd: Option<FixedRate11>,
	#[serde(rename = "Fltg", skip_serializing_if = "Option::is_none")]
	pub fltg: Option<FloatingInterestRate22>,
}


// InterestRate6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRate6 {
	#[serde(rename = "Amt")]
	pub amt: AmountAndDirection53,
	#[serde(rename = "IntrstRate")]
	pub intrst_rate: InterestRate27Choice,
}


// InterestRateContractTerm2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRateContractTerm2 {
	#[serde(rename = "Unit")]
	pub unit: RateBasis1Code,
	#[serde(rename = "Val")]
	pub val: f64,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}


// LoanData139 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LoanData139 {
	#[serde(rename = "UnqTradIdr", skip_serializing_if = "Option::is_none")]
	pub unq_trad_idr: Option<Max52Text>,
	#[serde(rename = "EvtDt", skip_serializing_if = "Option::is_none")]
	pub evt_dt: Option<String>,
	#[serde(rename = "ExctnDtTm", skip_serializing_if = "Option::is_none")]
	pub exctn_dt_tm: Option<String>,
	#[serde(rename = "ClrSts", skip_serializing_if = "Option::is_none")]
	pub clr_sts: Option<Cleared16Choice>,
	#[serde(rename = "TradgVn", skip_serializing_if = "Option::is_none")]
	pub tradg_vn: Option<MICIdentifier>,
	#[serde(rename = "MstrAgrmt", skip_serializing_if = "Option::is_none")]
	pub mstr_agrmt: Option<MasterAgreement7>,
	#[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
	pub val_dt: Option<String>,
	#[serde(rename = "MinNtcePrd", skip_serializing_if = "Option::is_none")]
	pub min_ntce_prd: Option<f64>,
	#[serde(rename = "EarlstCallBckDt", skip_serializing_if = "Option::is_none")]
	pub earlst_call_bck_dt: Option<String>,
	#[serde(rename = "GnlColl", skip_serializing_if = "Option::is_none")]
	pub gnl_coll: Option<SpecialCollateral1Code>,
	#[serde(rename = "DlvryByVal", skip_serializing_if = "Option::is_none")]
	pub dlvry_by_val: Option<bool>,
	#[serde(rename = "CollDlvryMtd", skip_serializing_if = "Option::is_none")]
	pub coll_dlvry_mtd: Option<CollateralDeliveryMethod1Code>,
	#[serde(rename = "Term", skip_serializing_if = "Option::is_none")]
	pub term: Option<Vec<ContractTerm7Choice>>,
	#[serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none")]
	pub intrst_rate: Option<InterestRate27Choice>,
	#[serde(rename = "PrncplAmt", skip_serializing_if = "Option::is_none")]
	pub prncpl_amt: Option<PrincipalAmount3>,
	#[serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none")]
	pub termntn_dt: Option<String>,
}


// LoanData140 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LoanData140 {
	#[serde(rename = "UnqTradIdr", skip_serializing_if = "Option::is_none")]
	pub unq_trad_idr: Option<Max52Text>,
	#[serde(rename = "EvtDt")]
	pub evt_dt: String,
	#[serde(rename = "ExctnDtTm", skip_serializing_if = "Option::is_none")]
	pub exctn_dt_tm: Option<String>,
	#[serde(rename = "ClrSts", skip_serializing_if = "Option::is_none")]
	pub clr_sts: Option<Cleared16Choice>,
	#[serde(rename = "TradgVn", skip_serializing_if = "Option::is_none")]
	pub tradg_vn: Option<MICIdentifier>,
	#[serde(rename = "MstrAgrmt", skip_serializing_if = "Option::is_none")]
	pub mstr_agrmt: Option<MasterAgreement7>,
	#[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
	pub val_dt: Option<String>,
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "GnlColl", skip_serializing_if = "Option::is_none")]
	pub gnl_coll: Option<SpecialCollateral1Code>,
	#[serde(rename = "PrncplAmt", skip_serializing_if = "Option::is_none")]
	pub prncpl_amt: Option<PrincipalAmount3>,
	#[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
	pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
	#[serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none")]
	pub termntn_dt: Option<String>,
}


// LoanData141 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LoanData141 {
	#[serde(rename = "UnqTradIdr", skip_serializing_if = "Option::is_none")]
	pub unq_trad_idr: Option<Max52Text>,
	#[serde(rename = "EvtDt")]
	pub evt_dt: String,
	#[serde(rename = "ExctnDtTm", skip_serializing_if = "Option::is_none")]
	pub exctn_dt_tm: Option<String>,
	#[serde(rename = "ClrSts", skip_serializing_if = "Option::is_none")]
	pub clr_sts: Option<Cleared16Choice>,
	#[serde(rename = "TradgVn", skip_serializing_if = "Option::is_none")]
	pub tradg_vn: Option<MICIdentifier>,
	#[serde(rename = "MstrAgrmt", skip_serializing_if = "Option::is_none")]
	pub mstr_agrmt: Option<MasterAgreement7>,
	#[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
	pub val_dt: Option<String>,
	#[serde(rename = "GnlColl", skip_serializing_if = "Option::is_none")]
	pub gnl_coll: Option<SpecialCollateral1Code>,
	#[serde(rename = "DlvryByVal", skip_serializing_if = "Option::is_none")]
	pub dlvry_by_val: Option<bool>,
	#[serde(rename = "CollDlvryMtd", skip_serializing_if = "Option::is_none")]
	pub coll_dlvry_mtd: Option<CollateralDeliveryMethod1Code>,
	#[serde(rename = "Term", skip_serializing_if = "Option::is_none")]
	pub term: Option<Vec<ContractTerm7Choice>>,
	#[serde(rename = "AsstTp", skip_serializing_if = "Option::is_none")]
	pub asst_tp: Option<SecurityCommodity9>,
	#[serde(rename = "LnVal", skip_serializing_if = "Option::is_none")]
	pub ln_val: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "RbtRate", skip_serializing_if = "Option::is_none")]
	pub rbt_rate: Option<InterestRate27Choice>,
	#[serde(rename = "LndgFee", skip_serializing_if = "Option::is_none")]
	pub lndg_fee: Option<f64>,
	#[serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none")]
	pub termntn_dt: Option<String>,
}


// LoanData142 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LoanData142 {
	#[serde(rename = "UnqTradIdr", skip_serializing_if = "Option::is_none")]
	pub unq_trad_idr: Option<Max52Text>,
	#[serde(rename = "EvtDt")]
	pub evt_dt: String,
	#[serde(rename = "ExctnDtTm", skip_serializing_if = "Option::is_none")]
	pub exctn_dt_tm: Option<String>,
	#[serde(rename = "TradgVn", skip_serializing_if = "Option::is_none")]
	pub tradg_vn: Option<MICIdentifier>,
	#[serde(rename = "CollDlvryMtd", skip_serializing_if = "Option::is_none")]
	pub coll_dlvry_mtd: Option<CollateralDeliveryMethod1Code>,
	#[serde(rename = "OutsdngMrgnLnAmt", skip_serializing_if = "Option::is_none")]
	pub outsdng_mrgn_ln_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "ShrtMktValAmt", skip_serializing_if = "Option::is_none")]
	pub shrt_mkt_val_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "MrgnLnAttr", skip_serializing_if = "Option::is_none")]
	pub mrgn_ln_attr: Option<Vec<InterestRate6>>,
	#[serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none")]
	pub termntn_dt: Option<String>,
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


// MasterAgreement7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MasterAgreement7 {
	#[serde(rename = "Tp")]
	pub tp: AgreementType2Choice,
	#[serde(rename = "Vrsn", skip_serializing_if = "Option::is_none")]
	pub vrsn: Option<Max50Text>,
	#[serde(rename = "OthrMstrAgrmtDtls", skip_serializing_if = "Option::is_none")]
	pub othr_mstr_agrmt_dtls: Option<Max350Text>,
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


// Max20PositiveNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max20PositiveNumber {
	#[serde(rename = "$value")]
	pub max20_positive_number: f64,
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


// Max72Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max72Text {
	#[serde(rename = "$value")]
	pub max72_text: String,
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


// ModificationLevel1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ModificationLevel1Code {
	#[default]
	#[serde(rename = "PSTN")]
	CodePSTN,
	#[serde(rename = "TCTN")]
	CodeTCTN,
}


// NACEDomainIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NACEDomainIdentifier {
	#[serde(rename = "$value")]
	pub nace_domain_identifier: String,
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


// NoReasonCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NoReasonCode {
	#[default]
	#[serde(rename = "NORE")]
	CodeNORE,
}


// NotAvailable1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NotAvailable1Code {
	#[default]
	#[serde(rename = "NTAV")]
	CodeNTAV,
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


// OtherC10CommodityDeliverable2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherC10CommodityDeliverable2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType11Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType47Code>,
}


// OtherC10CommodityNonDeliverable2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherC10CommodityNonDeliverable2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType11Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType48Code>,
}


// PaperCommodityContainerBoard1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityContainerBoard1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType8Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType35Code>,
}


// PaperCommodityNewsprint1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityNewsprint1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType8Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType36Code>,
}


// PaperCommodityPulp1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityPulp1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType8Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType37Code>,
}


// PaperCommodityRecoveredPaper1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityRecoveredPaper1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType8Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType38Code>,
}


// PaperCommodityRecoveredPaper2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaperCommodityRecoveredPaper2 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType8Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType49Code>,
}


// PartyIdentification236Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification236Choice {
	#[serde(rename = "Lgl", skip_serializing_if = "Option::is_none")]
	pub lgl: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "Ntrl", skip_serializing_if = "Option::is_none")]
	pub ntrl: Option<NaturalPersonIdentification2>,
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "$value")]
	pub percentage_rate: f64,
}


// PlusOrMinusIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PlusOrMinusIndicator {
	#[serde(rename = "$value")]
	pub plus_or_minus_indicator: bool,
}


// PolypropyleneCommodityOther1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PolypropyleneCommodityOther1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType9Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType49Code,
}


// PolypropyleneCommodityPlastic1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PolypropyleneCommodityPlastic1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType9Code,
	#[serde(rename = "SubPdct", skip_serializing_if = "Option::is_none")]
	pub sub_pdct: Option<AssetClassSubProductType18Code>,
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


// PrincipalAmount3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PrincipalAmount3 {
	#[serde(rename = "ValDtAmt", skip_serializing_if = "Option::is_none")]
	pub val_dt_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "MtrtyDtAmt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// Quantity17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Quantity17 {
	#[serde(rename = "Val")]
	pub val: f64,
	#[serde(rename = "UnitOfMeasr")]
	pub unit_of_measr: UnitOfMeasure11Code,
}


// QuantityNominalValue2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct QuantityNominalValue2Choice {
	#[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
	pub qty: Option<f64>,
	#[serde(rename = "NmnlVal", skip_serializing_if = "Option::is_none")]
	pub nmnl_val: Option<AmountAndDirection53>,
}


// RateAdjustment1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RateAdjustment1 {
	#[serde(rename = "Rate")]
	pub rate: f64,
	#[serde(rename = "AdjstmntDt")]
	pub adjstmnt_dt: String,
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


// ReconciliationFlag2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReconciliationFlag2 {
	#[serde(rename = "RptTp", skip_serializing_if = "Option::is_none")]
	pub rpt_tp: Option<TradeRepositoryReportingType1Code>,
	#[serde(rename = "BothCtrPtiesRptg", skip_serializing_if = "Option::is_none")]
	pub both_ctr_pties_rptg: Option<bool>,
	#[serde(rename = "PairdSts", skip_serializing_if = "Option::is_none")]
	pub paird_sts: Option<bool>,
	#[serde(rename = "LnRcncltnSts", skip_serializing_if = "Option::is_none")]
	pub ln_rcncltn_sts: Option<bool>,
	#[serde(rename = "CollRcncltnSts", skip_serializing_if = "Option::is_none")]
	pub coll_rcncltn_sts: Option<bool>,
	#[serde(rename = "ModSts", skip_serializing_if = "Option::is_none")]
	pub mod_sts: Option<bool>,
}


// RepoTerminationOption2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RepoTerminationOption2Code {
	#[default]
	#[serde(rename = "EGRN")]
	CodeEGRN,
	#[serde(rename = "EGAE")]
	CodeEGAE,
	#[serde(rename = "ETSB")]
	CodeETSB,
	#[serde(rename = "NOAP")]
	CodeNOAP,
}


// ReportPeriodActivity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ReportPeriodActivity1Code {
	#[default]
	#[serde(rename = "NOTX")]
	CodeNOTX,
}


// SecuritiesFinancingReportingTransactionStateReportV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesFinancingReportingTransactionStateReportV02 {
	#[serde(rename = "TradData")]
	pub trad_data: TradeStateReport5Choice,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// SecuritiesLendingType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesLendingType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalSecuritiesLendingType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// SecuritiesTransactionPrice18Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice18Choice {
	#[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
	pub mntry_val: Option<AmountAndDirection107>,
	#[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
	pub pctg: Option<f64>,
	#[serde(rename = "Dcml", skip_serializing_if = "Option::is_none")]
	pub dcml: Option<f64>,
	#[serde(rename = "BsisPts", skip_serializing_if = "Option::is_none")]
	pub bsis_pts: Option<f64>,
}


// SecuritiesTransactionPrice19Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice19Choice {
	#[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
	pub mntry_val: Option<AmountAndDirection107>,
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


// SecuritiesTransactionPrice5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice5 {
	#[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
	pub val: Option<f64>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<Max35Text>,
}


// Security51 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Security51 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ISINOct2015Identifier>,
	#[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
	pub clssfctn_tp: Option<CFIOct2015Identifier>,
	#[serde(rename = "QtyOrNmnlVal", skip_serializing_if = "Option::is_none")]
	pub qty_or_nmnl_val: Option<QuantityNominalValue2Choice>,
	#[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
	pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
	#[serde(rename = "MktVal", skip_serializing_if = "Option::is_none")]
	pub mkt_val: Option<AmountAndDirection53>,
	#[serde(rename = "Qlty", skip_serializing_if = "Option::is_none")]
	pub qlty: Option<CollateralQualityType1Code>,
	#[serde(rename = "Mtrty", skip_serializing_if = "Option::is_none")]
	pub mtrty: Option<String>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<SecurityIssuer4>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<Vec<SecuritiesLendingType3Choice>>,
	#[serde(rename = "ExclsvArrgmnt", skip_serializing_if = "Option::is_none")]
	pub exclsv_arrgmnt: Option<bool>,
	#[serde(rename = "AvlblForCollReuse", skip_serializing_if = "Option::is_none")]
	pub avlbl_for_coll_reuse: Option<bool>,
}


// Security52 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Security52 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ISINOct2015Identifier>,
	#[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
	pub clssfctn_tp: Option<CFIOct2015Identifier>,
	#[serde(rename = "QtyOrNmnlVal", skip_serializing_if = "Option::is_none")]
	pub qty_or_nmnl_val: Option<QuantityNominalValue2Choice>,
	#[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
	pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
	#[serde(rename = "MktVal", skip_serializing_if = "Option::is_none")]
	pub mkt_val: Option<AmountAndDirection53>,
	#[serde(rename = "Qlty", skip_serializing_if = "Option::is_none")]
	pub qlty: Option<CollateralQualityType1Code>,
	#[serde(rename = "Mtrty", skip_serializing_if = "Option::is_none")]
	pub mtrty: Option<String>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<SecurityIssuer4>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<Vec<SecuritiesLendingType3Choice>>,
	#[serde(rename = "ExclsvArrgmnt", skip_serializing_if = "Option::is_none")]
	pub exclsv_arrgmnt: Option<bool>,
	#[serde(rename = "HrcutOrMrgn", skip_serializing_if = "Option::is_none")]
	pub hrcut_or_mrgn: Option<f64>,
	#[serde(rename = "AvlblForCollReuse", skip_serializing_if = "Option::is_none")]
	pub avlbl_for_coll_reuse: Option<bool>,
}


// Security55 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Security55 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ISINOct2015Identifier>,
	#[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
	pub clssfctn_tp: Option<CFIOct2015Identifier>,
	#[serde(rename = "QtyOrNmnlVal", skip_serializing_if = "Option::is_none")]
	pub qty_or_nmnl_val: Option<QuantityNominalValue2Choice>,
	#[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
	pub unit_pric: Option<SecuritiesTransactionPrice19Choice>,
	#[serde(rename = "MktVal", skip_serializing_if = "Option::is_none")]
	pub mkt_val: Option<AmountAndDirection53>,
	#[serde(rename = "Qlty", skip_serializing_if = "Option::is_none")]
	pub qlty: Option<CollateralQualityType1Code>,
	#[serde(rename = "Mtrty", skip_serializing_if = "Option::is_none")]
	pub mtrty: Option<String>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<SecurityIssuer4>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<Vec<SecuritiesLendingType3Choice>>,
	#[serde(rename = "ExclsvArrgmnt", skip_serializing_if = "Option::is_none")]
	pub exclsv_arrgmnt: Option<bool>,
	#[serde(rename = "AvlblForCollReuse", skip_serializing_if = "Option::is_none")]
	pub avlbl_for_coll_reuse: Option<bool>,
	#[serde(rename = "HrcutOrMrgn", skip_serializing_if = "Option::is_none")]
	pub hrcut_or_mrgn: Option<f64>,
}


// SecurityCommodity9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityCommodity9 {
	#[serde(rename = "Scty", skip_serializing_if = "Option::is_none")]
	pub scty: Option<Vec<Security51>>,
	#[serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none")]
	pub cmmdty: Option<Vec<Commodity43>>,
}


// SecurityIdentification26Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification26Choice {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ISINOct2015Identifier>,
	#[serde(rename = "NotAvlbl", skip_serializing_if = "Option::is_none")]
	pub not_avlbl: Option<NotAvailable1Code>,
}


// SecurityIssuer4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIssuer4 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "JursdctnCtry")]
	pub jursdctn_ctry: CountryCode,
}


// SettlementParties34Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementParties34Choice {
	#[serde(rename = "CntrlSctiesDpstryPtcpt", skip_serializing_if = "Option::is_none")]
	pub cntrl_scties_dpstry_ptcpt: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "IndrctPtcpt", skip_serializing_if = "Option::is_none")]
	pub indrct_ptcpt: Option<OrganisationIdentification15Choice>,
}


// SpecialCollateral1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SpecialCollateral1Code {
	#[default]
	#[serde(rename = "GENE")]
	CodeGENE,
	#[serde(rename = "SPEC")]
	CodeSPEC,
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


// TradeRepositoryReportingType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TradeRepositoryReportingType1Code {
	#[default]
	#[serde(rename = "SWOS")]
	CodeSWOS,
	#[serde(rename = "TWOS")]
	CodeTWOS,
}


// TradeStateReport16 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeStateReport16 {
	#[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
	pub tech_rcrd_id: Option<Max140Text>,
	#[serde(rename = "CtrPtySpcfcData")]
	pub ctr_pty_spcfc_data: CounterpartyData88,
	#[serde(rename = "LnData", skip_serializing_if = "Option::is_none")]
	pub ln_data: Option<TransactionLoanData31Choice>,
	#[serde(rename = "CollData", skip_serializing_if = "Option::is_none")]
	pub coll_data: Option<TransactionCollateralData18Choice>,
	#[serde(rename = "RcncltnFlg", skip_serializing_if = "Option::is_none")]
	pub rcncltn_flg: Option<ReconciliationFlag2>,
	#[serde(rename = "CtrctMod")]
	pub ctrct_mod: ContractModification3,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// TradeStateReport5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeStateReport5Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[serde(rename = "Stat", skip_serializing_if = "Option::is_none")]
	pub stat: Option<Vec<TradeStateReport16>>,
}


// TransactionCollateralData18Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionCollateralData18Choice {
	#[serde(rename = "RpTrad", skip_serializing_if = "Option::is_none")]
	pub rp_trad: Option<Collateral52>,
	#[serde(rename = "BuySellBck", skip_serializing_if = "Option::is_none")]
	pub buy_sell_bck: Option<Collateral52>,
	#[serde(rename = "SctiesLndg", skip_serializing_if = "Option::is_none")]
	pub scties_lndg: Option<CollateralFlag13Choice>,
	#[serde(rename = "MrgnLndg", skip_serializing_if = "Option::is_none")]
	pub mrgn_lndg: Option<Vec<Security55>>,
}


// TransactionCounterpartyData11 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionCounterpartyData11 {
	#[serde(rename = "Bnfcry", skip_serializing_if = "Option::is_none")]
	pub bnfcry: Option<PartyIdentification236Choice>,
	#[serde(rename = "TrptyAgt", skip_serializing_if = "Option::is_none")]
	pub trpty_agt: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "Brkr", skip_serializing_if = "Option::is_none")]
	pub brkr: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "ClrMmb", skip_serializing_if = "Option::is_none")]
	pub clr_mmb: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "SttlmPties", skip_serializing_if = "Option::is_none")]
	pub sttlm_pties: Option<SettlementParties34Choice>,
	#[serde(rename = "AgtLndr", skip_serializing_if = "Option::is_none")]
	pub agt_lndr: Option<OrganisationIdentification15Choice>,
}


// TransactionLoanData31Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionLoanData31Choice {
	#[serde(rename = "RpTrad", skip_serializing_if = "Option::is_none")]
	pub rp_trad: Option<LoanData139>,
	#[serde(rename = "BuySellBck", skip_serializing_if = "Option::is_none")]
	pub buy_sell_bck: Option<LoanData140>,
	#[serde(rename = "SctiesLndg", skip_serializing_if = "Option::is_none")]
	pub scties_lndg: Option<LoanData141>,
	#[serde(rename = "MrgnLndg", skip_serializing_if = "Option::is_none")]
	pub mrgn_lndg: Option<LoanData142>,
}


// TransactionOperationType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TransactionOperationType6Code {
	#[default]
	#[serde(rename = "REUU")]
	CodeREUU,
	#[serde(rename = "COLU")]
	CodeCOLU,
	#[serde(rename = "CORR")]
	CodeCORR,
	#[serde(rename = "ETRM")]
	CodeETRM,
	#[serde(rename = "VALU")]
	CodeVALU,
	#[serde(rename = "POSC")]
	CodePOSC,
	#[serde(rename = "NEWT")]
	CodeNEWT,
	#[serde(rename = "MODI")]
	CodeMODI,
	#[serde(rename = "MARU")]
	CodeMARU,
	#[serde(rename = "EROR")]
	CodeEROR,
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}


// UnitOfMeasure11Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum UnitOfMeasure11Code {
	#[default]
	#[serde(rename = "ALOW")]
	CodeALOW,
	#[serde(rename = "ACCY")]
	CodeACCY,
	#[serde(rename = "BARL")]
	CodeBARL,
	#[serde(rename = "BCUF")]
	CodeBCUF,
	#[serde(rename = "BDFT")]
	CodeBDFT,
	#[serde(rename = "BUSL")]
	CodeBUSL,
	#[serde(rename = "CEER")]
	CodeCEER,
	#[serde(rename = "CLRT")]
	CodeCLRT,
	#[serde(rename = "KILO")]
	CodeKILO,
	#[serde(rename = "PIEC")]
	CodePIEC,
	#[serde(rename = "TONS")]
	CodeTONS,
	#[serde(rename = "METR")]
	CodeMETR,
	#[serde(rename = "INCH")]
	CodeINCH,
	#[serde(rename = "YARD")]
	CodeYARD,
	#[serde(rename = "GBGA")]
	CodeGBGA,
	#[serde(rename = "GRAM")]
	CodeGRAM,
	#[serde(rename = "CMET")]
	CodeCMET,
	#[serde(rename = "SMET")]
	CodeSMET,
	#[serde(rename = "FOOT")]
	CodeFOOT,
	#[serde(rename = "MILE")]
	CodeMILE,
	#[serde(rename = "SQIN")]
	CodeSQIN,
	#[serde(rename = "SQFO")]
	CodeSQFO,
	#[serde(rename = "SQMI")]
	CodeSQMI,
	#[serde(rename = "GBOU")]
	CodeGBOU,
	#[serde(rename = "USOU")]
	CodeUSOU,
	#[serde(rename = "GBPI")]
	CodeGBPI,
	#[serde(rename = "USPI")]
	CodeUSPI,
	#[serde(rename = "GBQA")]
	CodeGBQA,
	#[serde(rename = "USGA")]
	CodeUSGA,
	#[serde(rename = "MMET")]
	CodeMMET,
	#[serde(rename = "KMET")]
	CodeKMET,
	#[serde(rename = "SQYA")]
	CodeSQYA,
	#[serde(rename = "ACRE")]
	CodeACRE,
	#[serde(rename = "ARES")]
	CodeARES,
	#[serde(rename = "SMIL")]
	CodeSMIL,
	#[serde(rename = "SCMT")]
	CodeSCMT,
	#[serde(rename = "HECT")]
	CodeHECT,
	#[serde(rename = "SQKI")]
	CodeSQKI,
	#[serde(rename = "MILI")]
	CodeMILI,
	#[serde(rename = "CELI")]
	CodeCELI,
	#[serde(rename = "LITR")]
	CodeLITR,
	#[serde(rename = "PUND")]
	CodePUND,
	#[serde(rename = "CBME")]
	CodeCBME,
	#[serde(rename = "DAYS")]
	CodeDAYS,
	#[serde(rename = "DMET")]
	CodeDMET,
	#[serde(rename = "ENVC")]
	CodeENVC,
	#[serde(rename = "ENVO")]
	CodeENVO,
	#[serde(rename = "HUWG")]
	CodeHUWG,
	#[serde(rename = "KWDC")]
	CodeKWDC,
	#[serde(rename = "KWHO")]
	CodeKWHO,
	#[serde(rename = "KWHC")]
	CodeKWHC,
	#[serde(rename = "KMOC")]
	CodeKMOC,
	#[serde(rename = "KWMC")]
	CodeKWMC,
	#[serde(rename = "KWYC")]
	CodeKWYC,
	#[serde(rename = "MWDC")]
	CodeMWDC,
	#[serde(rename = "MWHO")]
	CodeMWHO,
	#[serde(rename = "MWHC")]
	CodeMWHC,
	#[serde(rename = "MWMC")]
	CodeMWMC,
	#[serde(rename = "MMOC")]
	CodeMMOC,
	#[serde(rename = "MWYC")]
	CodeMWYC,
	#[serde(rename = "TONE")]
	CodeTONE,
	#[serde(rename = "MIBA")]
	CodeMIBA,
	#[serde(rename = "MBTU")]
	CodeMBTU,
	#[serde(rename = "OZTR")]
	CodeOZTR,
	#[serde(rename = "UCWT")]
	CodeUCWT,
	#[serde(rename = "IPNT")]
	CodeIPNT,
	#[serde(rename = "PWRD")]
	CodePWRD,
	#[serde(rename = "DGEU")]
	CodeDGEU,
	#[serde(rename = "TOCD")]
	CodeTOCD,
	#[serde(rename = "GGEU")]
	CodeGGEU,
	#[serde(rename = "USQA")]
	CodeUSQA,
}
