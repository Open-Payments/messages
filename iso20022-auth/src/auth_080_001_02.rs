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


// AgreementType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgreementType1Choice {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<ExternalAgreementType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
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


// CFIOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[serde(rename = "$value")]
	pub cfi_oct2015_identifier: String,
}


// CashCompare3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CashCompare3 {
	#[serde(rename = "Val", skip_serializing_if = "Option::is_none")]
	pub val: Option<CompareAmountAndDirection2>,
	#[serde(rename = "HrcutOrMrgn", skip_serializing_if = "Option::is_none")]
	pub hrcut_or_mrgn: Option<ComparePercentageRate3>,
}


// Cleared4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Cleared4Choice {
	#[serde(rename = "Clrd", skip_serializing_if = "Option::is_none")]
	pub clrd: Option<NoReasonCode>,
	#[serde(rename = "NonClrd", skip_serializing_if = "Option::is_none")]
	pub non_clrd: Option<NoReasonCode>,
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


// CollateralMatchingCriteria6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralMatchingCriteria6 {
	#[serde(rename = "UncollsdFlg", skip_serializing_if = "Option::is_none")]
	pub uncollsd_flg: Option<CompareTrueFalseIndicator3>,
	#[serde(rename = "NetXpsrCollstnInd", skip_serializing_if = "Option::is_none")]
	pub net_xpsr_collstn_ind: Option<CompareTrueFalseIndicator3>,
	#[serde(rename = "CollValDt", skip_serializing_if = "Option::is_none")]
	pub coll_val_dt: Option<CompareDate3>,
	#[serde(rename = "AsstTp", skip_serializing_if = "Option::is_none")]
	pub asst_tp: Option<SecurityCommodityCash4>,
	#[serde(rename = "BsktIdr", skip_serializing_if = "Option::is_none")]
	pub bskt_idr: Option<CompareSecurityIdentification4>,
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


// Commodity42 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Commodity42 {
	#[serde(rename = "Clssfctn", skip_serializing_if = "Option::is_none")]
	pub clssfctn: Option<CompareCommodityAssetClass3>,
	#[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
	pub qty: Option<CompareDecimalNumber3>,
	#[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
	pub unit_pric: Option<CompareUnitPrice6>,
	#[serde(rename = "MktVal", skip_serializing_if = "Option::is_none")]
	pub mkt_val: Option<CompareAmountAndDirection2>,
	#[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
	pub unit_of_measr: Option<CompareUnitOfMeasure3>,
}


// CompareActiveOrHistoricCurrencyAndAmount3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareActiveOrHistoricCurrencyAndAmount3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<ActiveOrHistoricCurrencyAndAmount>,
}


// CompareAgreementType2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareAgreementType2 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<AgreementType1Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<AgreementType1Choice>,
}


// CompareAmountAndDirection1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareAmountAndDirection1 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<AmountAndDirection53>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<AmountAndDirection53>,
}


// CompareAmountAndDirection2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareAmountAndDirection2 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<AmountAndDirection53>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<AmountAndDirection53>,
}


// CompareBenchmarkCurveName3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareBenchmarkCurveName3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<BenchmarkCurveName10Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<BenchmarkCurveName10Choice>,
}


// CompareCFIIdentifier3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareCFIIdentifier3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<CFIOct2015Identifier>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<CFIOct2015Identifier>,
}


// CompareClearingStatus3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareClearingStatus3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<Cleared4Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<Cleared4Choice>,
}


// CompareCollateralQualityType3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareCollateralQualityType3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<CollateralQualityType1Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<CollateralQualityType1Code>,
}


// CompareCommodityAssetClass3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareCommodityAssetClass3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<AssetClassCommodity5Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<AssetClassCommodity5Choice>,
}


// CompareCounterpartySide2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareCounterpartySide2 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<CollateralRole1Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<CollateralRole1Code>,
}


// CompareCountryCode3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareCountryCode3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<CountryCode>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<CountryCode>,
}


// CompareDate3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareDate3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<String>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<String>,
}


// CompareDateTime3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareDateTime3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<String>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<String>,
}


// CompareDecimalNumber3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareDecimalNumber3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<f64>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<f64>,
}


// CompareDeliveryMethod3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareDeliveryMethod3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<CollateralDeliveryMethod1Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<CollateralDeliveryMethod1Code>,
}


// CompareExposureType3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareExposureType3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<ExposureType10Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<ExposureType10Code>,
}


// CompareISINIdentifier4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareISINIdentifier4 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<ISINOct2015Identifier>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<ISINOct2015Identifier>,
}


// CompareInterestComputationMethod3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareInterestComputationMethod3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<InterestComputationMethodFormat6Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<InterestComputationMethodFormat6Choice>,
}


// CompareInterestRate1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareInterestRate1 {
	#[serde(rename = "MrgnLnAmt", skip_serializing_if = "Option::is_none")]
	pub mrgn_ln_amt: Option<CompareAmountAndDirection1>,
	#[serde(rename = "FxdIntrstRate", skip_serializing_if = "Option::is_none")]
	pub fxd_intrst_rate: Option<ComparePercentageRate3>,
	#[serde(rename = "DayCntBsis", skip_serializing_if = "Option::is_none")]
	pub day_cnt_bsis: Option<CompareInterestComputationMethod3>,
	#[serde(rename = "FltgIntrstRefRate", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_ref_rate: Option<CompareBenchmarkCurveName3>,
	#[serde(rename = "FltgIntrstRateTermUnit", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_rate_term_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgIntrstRateTermVal", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_rate_term_val: Option<CompareNumber5>,
	#[serde(rename = "FltgIntrstRatePmtFrqcyUnit", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_rate_pmt_frqcy_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgIntrstRatePmtFrqcyVal", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_rate_pmt_frqcy_val: Option<CompareNumber5>,
	#[serde(rename = "FltgIntrstRateRstFrqcyUnit", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_rate_rst_frqcy_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgIntrstRateRstFrqcyVal", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_rate_rst_frqcy_val: Option<CompareNumber6>,
	#[serde(rename = "BsisPtSprd", skip_serializing_if = "Option::is_none")]
	pub bsis_pt_sprd: Option<CompareDecimalNumber3>,
}


// CompareMICIdentifier3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareMICIdentifier3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<MICIdentifier>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<MICIdentifier>,
}


// CompareNumber5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareNumber5 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<f64>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<f64>,
}


// CompareNumber6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareNumber6 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<f64>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<f64>,
}


// CompareOrganisationIdentification6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareOrganisationIdentification6 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<OrganisationIdentification15Choice>,
}


// CompareOrganisationIdentification7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareOrganisationIdentification7 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<PartyIdentification236Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<PartyIdentification236Choice>,
}


// ComparePercentageRate3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ComparePercentageRate3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<f64>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<f64>,
}


// CompareRateBasis3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareRateBasis3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<RateBasis1Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<RateBasis1Code>,
}


// CompareReportingLevelType3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareReportingLevelType3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<ModificationLevel1Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<ModificationLevel1Code>,
}


// CompareSecuritiesLendingType3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareSecuritiesLendingType3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<SecuritiesLendingType3Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<SecuritiesLendingType3Choice>,
}


// CompareSecurityIdentification4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareSecurityIdentification4 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<SecurityIdentification26Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<SecurityIdentification26Choice>,
}


// CompareSpecialCollateral3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareSpecialCollateral3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<SpecialCollateral1Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<SpecialCollateral1Code>,
}


// CompareTerminationOption3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareTerminationOption3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<RepoTerminationOption2Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<RepoTerminationOption2Code>,
}


// CompareText2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareText2 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<Max52Text>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<Max52Text>,
}


// CompareTrueFalseIndicator3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareTrueFalseIndicator3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<bool>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<bool>,
}


// CompareUnitOfMeasure3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareUnitOfMeasure3 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<UnitOfMeasure11Code>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<UnitOfMeasure11Code>,
}


// CompareUnitPrice6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CompareUnitPrice6 {
	#[serde(rename = "Val1", skip_serializing_if = "Option::is_none")]
	pub val1: Option<SecuritiesTransactionPrice19Choice>,
	#[serde(rename = "Val2", skip_serializing_if = "Option::is_none")]
	pub val2: Option<SecuritiesTransactionPrice19Choice>,
}


// CounterpartyMatchingCriteria4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyMatchingCriteria4 {
	#[serde(rename = "RptgCtrPty", skip_serializing_if = "Option::is_none")]
	pub rptg_ctr_pty: Option<CompareOrganisationIdentification6>,
	#[serde(rename = "OthrCtrPty", skip_serializing_if = "Option::is_none")]
	pub othr_ctr_pty: Option<CompareOrganisationIdentification7>,
	#[serde(rename = "CtrPtySd", skip_serializing_if = "Option::is_none")]
	pub ctr_pty_sd: Option<CompareCounterpartySide2>,
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


// ExposureType10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ExposureType10Code {
	#[default]
	#[serde(rename = "SBSC")]
	CodeSBSC,
	#[serde(rename = "MGLD")]
	CodeMGLD,
	#[serde(rename = "SLEB")]
	CodeSLEB,
	#[serde(rename = "REPO")]
	CodeREPO,
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


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}


// LoanMatchingCriteria9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LoanMatchingCriteria9 {
	#[serde(rename = "UnqTradIdr", skip_serializing_if = "Option::is_none")]
	pub unq_trad_idr: Option<CompareText2>,
	#[serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none")]
	pub termntn_dt: Option<CompareDate3>,
	#[serde(rename = "CtrctTp", skip_serializing_if = "Option::is_none")]
	pub ctrct_tp: Option<CompareExposureType3>,
	#[serde(rename = "ClrSts", skip_serializing_if = "Option::is_none")]
	pub clr_sts: Option<CompareClearingStatus3>,
	#[serde(rename = "ClrDtTm", skip_serializing_if = "Option::is_none")]
	pub clr_dt_tm: Option<CompareDateTime3>,
	#[serde(rename = "CCP", skip_serializing_if = "Option::is_none")]
	pub ccp: Option<CompareOrganisationIdentification6>,
	#[serde(rename = "TradgVn", skip_serializing_if = "Option::is_none")]
	pub tradg_vn: Option<CompareMICIdentifier3>,
	#[serde(rename = "MstrAgrmtTp", skip_serializing_if = "Option::is_none")]
	pub mstr_agrmt_tp: Option<CompareAgreementType2>,
	#[serde(rename = "ExctnDtTm", skip_serializing_if = "Option::is_none")]
	pub exctn_dt_tm: Option<CompareDateTime3>,
	#[serde(rename = "ValDt", skip_serializing_if = "Option::is_none")]
	pub val_dt: Option<CompareDate3>,
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<CompareDate3>,
	#[serde(rename = "MinNtcePrd", skip_serializing_if = "Option::is_none")]
	pub min_ntce_prd: Option<CompareNumber5>,
	#[serde(rename = "EarlstCallBckDt", skip_serializing_if = "Option::is_none")]
	pub earlst_call_bck_dt: Option<CompareDate3>,
	#[serde(rename = "GnlColl", skip_serializing_if = "Option::is_none")]
	pub gnl_coll: Option<CompareSpecialCollateral3>,
	#[serde(rename = "DlvryByVal", skip_serializing_if = "Option::is_none")]
	pub dlvry_by_val: Option<CompareTrueFalseIndicator3>,
	#[serde(rename = "CollDlvryMtd", skip_serializing_if = "Option::is_none")]
	pub coll_dlvry_mtd: Option<CompareDeliveryMethod3>,
	#[serde(rename = "OpnTerm", skip_serializing_if = "Option::is_none")]
	pub opn_term: Option<CompareTrueFalseIndicator3>,
	#[serde(rename = "TermntnOptn", skip_serializing_if = "Option::is_none")]
	pub termntn_optn: Option<CompareTerminationOption3>,
	#[serde(rename = "FxdIntrstRate", skip_serializing_if = "Option::is_none")]
	pub fxd_intrst_rate: Option<ComparePercentageRate3>,
	#[serde(rename = "DayCntBsis", skip_serializing_if = "Option::is_none")]
	pub day_cnt_bsis: Option<CompareInterestComputationMethod3>,
	#[serde(rename = "FltgIntrstRefRate", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_ref_rate: Option<CompareBenchmarkCurveName3>,
	#[serde(rename = "FltgIntrstRateTermUnit", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_rate_term_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgIntrstRateTermVal", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_rate_term_val: Option<CompareNumber5>,
	#[serde(rename = "FltgIntrstRatePmtFrqcyUnit", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_rate_pmt_frqcy_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgIntrstRatePmtFrqcyVal", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_rate_pmt_frqcy_val: Option<CompareNumber5>,
	#[serde(rename = "FltgIntrstRateRstFrqcyUnit", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_rate_rst_frqcy_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgIntrstRateRstFrqcyVal", skip_serializing_if = "Option::is_none")]
	pub fltg_intrst_rate_rst_frqcy_val: Option<CompareNumber6>,
	#[serde(rename = "BsisPtSprd", skip_serializing_if = "Option::is_none")]
	pub bsis_pt_sprd: Option<CompareDecimalNumber3>,
	#[serde(rename = "MrgnLnAttr", skip_serializing_if = "Option::is_none")]
	pub mrgn_ln_attr: Option<Vec<CompareInterestRate1>>,
	#[serde(rename = "PrncplAmtValDtAmt", skip_serializing_if = "Option::is_none")]
	pub prncpl_amt_val_dt_amt: Option<CompareActiveOrHistoricCurrencyAndAmount3>,
	#[serde(rename = "PrncplAmtMtrtyDtAmt", skip_serializing_if = "Option::is_none")]
	pub prncpl_amt_mtrty_dt_amt: Option<CompareActiveOrHistoricCurrencyAndAmount3>,
	#[serde(rename = "AsstTp", skip_serializing_if = "Option::is_none")]
	pub asst_tp: Option<SecurityCommodity7Choice>,
	#[serde(rename = "LnVal", skip_serializing_if = "Option::is_none")]
	pub ln_val: Option<CompareActiveOrHistoricCurrencyAndAmount3>,
	#[serde(rename = "FxdRbtRefRate", skip_serializing_if = "Option::is_none")]
	pub fxd_rbt_ref_rate: Option<ComparePercentageRate3>,
	#[serde(rename = "FltgRbtRefRate", skip_serializing_if = "Option::is_none")]
	pub fltg_rbt_ref_rate: Option<CompareBenchmarkCurveName3>,
	#[serde(rename = "FltgRbtRateTermUnit", skip_serializing_if = "Option::is_none")]
	pub fltg_rbt_rate_term_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgRbtRateTermVal", skip_serializing_if = "Option::is_none")]
	pub fltg_rbt_rate_term_val: Option<CompareNumber6>,
	#[serde(rename = "FltgRbtRatePmtFrqcyUnit", skip_serializing_if = "Option::is_none")]
	pub fltg_rbt_rate_pmt_frqcy_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgRbtRatePmtFrqcyVal", skip_serializing_if = "Option::is_none")]
	pub fltg_rbt_rate_pmt_frqcy_val: Option<CompareNumber6>,
	#[serde(rename = "FltgRbtRateRstFrqcyUnit", skip_serializing_if = "Option::is_none")]
	pub fltg_rbt_rate_rst_frqcy_unit: Option<CompareRateBasis3>,
	#[serde(rename = "FltgRbtRateRstFrqcyVal", skip_serializing_if = "Option::is_none")]
	pub fltg_rbt_rate_rst_frqcy_val: Option<CompareNumber6>,
	#[serde(rename = "RbtRateBsisPtSprd", skip_serializing_if = "Option::is_none")]
	pub rbt_rate_bsis_pt_sprd: Option<CompareDecimalNumber3>,
	#[serde(rename = "FltgRateAdjstmnt", skip_serializing_if = "Option::is_none")]
	pub fltg_rate_adjstmnt: Option<Vec<ComparePercentageRate3>>,
	#[serde(rename = "FltgRateAdjstmntDt", skip_serializing_if = "Option::is_none")]
	pub fltg_rate_adjstmnt_dt: Option<Vec<CompareDate3>>,
	#[serde(rename = "LndgFee", skip_serializing_if = "Option::is_none")]
	pub lndg_fee: Option<ComparePercentageRate3>,
	#[serde(rename = "OutsdngMrgnLnAmt", skip_serializing_if = "Option::is_none")]
	pub outsdng_mrgn_ln_amt: Option<CompareActiveOrHistoricCurrencyAndAmount3>,
	#[serde(rename = "ShrtMktValAmt", skip_serializing_if = "Option::is_none")]
	pub shrt_mkt_val_amt: Option<CompareActiveOrHistoricCurrencyAndAmount3>,
	#[serde(rename = "LvlTp", skip_serializing_if = "Option::is_none")]
	pub lvl_tp: Option<CompareReportingLevelType3>,
	#[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
	pub unit_of_measr: Option<CompareUnitOfMeasure3>,
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


// MatchingCriteria10 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MatchingCriteria10 {
	#[serde(rename = "CtrPtyMtchgCrit", skip_serializing_if = "Option::is_none")]
	pub ctr_pty_mtchg_crit: Option<CounterpartyMatchingCriteria4>,
	#[serde(rename = "LnMtchgCrit", skip_serializing_if = "Option::is_none")]
	pub ln_mtchg_crit: Option<LoanMatchingCriteria9>,
	#[serde(rename = "CollMtchgCrit", skip_serializing_if = "Option::is_none")]
	pub coll_mtchg_crit: Option<CollateralMatchingCriteria6>,
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


// Max15NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max15NumericText {
	#[serde(rename = "$value")]
	pub max15_numeric_text: String,
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


// Max5Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max5Number {
	#[serde(rename = "$value")]
	pub max5_number: f64,
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


// NumberOfReportsPerStatus4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NumberOfReportsPerStatus4 {
	#[serde(rename = "DtldNbOfRpts")]
	pub dtld_nb_of_rpts: Max15NumericText,
	#[serde(rename = "DtldSts")]
	pub dtld_sts: PairedReconciled3Code,
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


// PairedReconciled3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PairedReconciled3Code {
	#[default]
	#[serde(rename = "CLRC")]
	CodeCLRC,
	#[serde(rename = "LNRC")]
	CodeLNRC,
	#[serde(rename = "PARD")]
	CodePARD,
	#[serde(rename = "RECO")]
	CodeRECO,
	#[serde(rename = "UNPR")]
	CodeUNPR,
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


// ReconciliationMatchedStatus9Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReconciliationMatchedStatus9Choice {
	#[serde(rename = "Mtchd", skip_serializing_if = "Option::is_none")]
	pub mtchd: Option<NoReasonCode>,
	#[serde(rename = "NotMtchd", skip_serializing_if = "Option::is_none")]
	pub not_mtchd: Option<ReconciliationResult10>,
}


// ReconciliationReport8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReconciliationReport8 {
	#[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
	pub tech_rcrd_id: Option<Max140Text>,
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
	#[serde(rename = "NoRcncltnReqrd", skip_serializing_if = "Option::is_none")]
	pub no_rcncltn_reqrd: Option<NoReasonCode>,
	#[serde(rename = "RptgData", skip_serializing_if = "Option::is_none")]
	pub rptg_data: Option<ReconciliationMatchedStatus9Choice>,
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


// SecuritiesFinancingReportingReconciliationStatusAdviceV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesFinancingReportingReconciliationStatusAdviceV02 {
	#[serde(rename = "RcncltnData")]
	pub rcncltn_data: TradeData34Choice,
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


// Security48 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Security48 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<CompareISINIdentifier4>,
	#[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
	pub clssfctn_tp: Option<CompareCFIIdentifier3>,
	#[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
	pub qty: Option<CompareDecimalNumber3>,
	#[serde(rename = "NmnlVal", skip_serializing_if = "Option::is_none")]
	pub nmnl_val: Option<CompareAmountAndDirection2>,
	#[serde(rename = "Qlty", skip_serializing_if = "Option::is_none")]
	pub qlty: Option<CompareCollateralQualityType3>,
	#[serde(rename = "Mtrty", skip_serializing_if = "Option::is_none")]
	pub mtrty: Option<CompareDate3>,
	#[serde(rename = "IssrId", skip_serializing_if = "Option::is_none")]
	pub issr_id: Option<CompareOrganisationIdentification6>,
	#[serde(rename = "IssrCtry", skip_serializing_if = "Option::is_none")]
	pub issr_ctry: Option<CompareCountryCode3>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<Vec<CompareSecuritiesLendingType3>>,
	#[serde(rename = "UnitPric", skip_serializing_if = "Option::is_none")]
	pub unit_pric: Option<CompareUnitPrice6>,
	#[serde(rename = "ExclsvArrgmnt", skip_serializing_if = "Option::is_none")]
	pub exclsv_arrgmnt: Option<CompareTrueFalseIndicator3>,
	#[serde(rename = "MktVal", skip_serializing_if = "Option::is_none")]
	pub mkt_val: Option<CompareAmountAndDirection2>,
	#[serde(rename = "AvlblForCollReuse", skip_serializing_if = "Option::is_none")]
	pub avlbl_for_coll_reuse: Option<CompareTrueFalseIndicator3>,
	#[serde(rename = "HrcutOrMrgn", skip_serializing_if = "Option::is_none")]
	pub hrcut_or_mrgn: Option<ComparePercentageRate3>,
}


// SecurityCommodity7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityCommodity7Choice {
	#[serde(rename = "Scty", skip_serializing_if = "Option::is_none")]
	pub scty: Option<Vec<Security48>>,
	#[serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none")]
	pub cmmdty: Option<Vec<Commodity42>>,
}


// SecurityCommodityCash4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityCommodityCash4 {
	#[serde(rename = "Scty", skip_serializing_if = "Option::is_none")]
	pub scty: Option<Vec<Security48>>,
	#[serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none")]
	pub cmmdty: Option<Vec<Commodity42>>,
	#[serde(rename = "Csh", skip_serializing_if = "Option::is_none")]
	pub csh: Option<Vec<CashCompare3>>,
}


// SecurityIdentification26Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification26Choice {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<ISINOct2015Identifier>,
	#[serde(rename = "NotAvlbl", skip_serializing_if = "Option::is_none")]
	pub not_avlbl: Option<NotAvailable1Code>,
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


// TradeData28 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeData28 {
	#[serde(rename = "PairgRcncltnSts", skip_serializing_if = "Option::is_none")]
	pub pairg_rcncltn_sts: Option<Vec<NumberOfReportsPerStatus4>>,
	#[serde(rename = "RcncltnRpt")]
	pub rcncltn_rpt: Vec<ReconciliationReport8>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// TradeData34Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeData34Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[serde(rename = "Rpt", skip_serializing_if = "Option::is_none")]
	pub rpt: Option<Vec<TradeData28>>,
}


// TradeTransactionIdentification19 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeTransactionIdentification19 {
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: OrganisationIdentification15Choice,
	#[serde(rename = "OthrCtrPty")]
	pub othr_ctr_pty: PartyIdentification236Choice,
	#[serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none")]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "UnqTradIdr", skip_serializing_if = "Option::is_none")]
	pub unq_trad_idr: Option<Max52Text>,
	#[serde(rename = "MstrAgrmt", skip_serializing_if = "Option::is_none")]
	pub mstr_agrmt: Option<MasterAgreement7>,
	#[serde(rename = "AgtLndr", skip_serializing_if = "Option::is_none")]
	pub agt_lndr: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "TrptyAgt", skip_serializing_if = "Option::is_none")]
	pub trpty_agt: Option<OrganisationIdentification15Choice>,
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
