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


// ActiveCurrencyAnd13DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd13DecimalAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_currency_and13_decimal_amount_simple_type: f64,
}


// ActiveCurrencyAnd13DecimalAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAnd13DecimalAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "$value")]
	pub active_currency_code: String,
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


// AgriculturalCommodityGrain1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityGrain1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType5Code,
	#[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
	pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType15Code>,
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


// AgriculturalCommodityOliveOil1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgriculturalCommodityOliveOil1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType1Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType3Code,
	#[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
	pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType4Code>,
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


// AmountAndDirection61 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountAndDirection61 {
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAnd13DecimalAmount,
	#[serde(rename = "Sgn", skip_serializing_if = "Option::is_none")]
	pub sgn: Option<bool>,
}


// AssetClass2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClass2 {
	#[serde(rename = "Cmmdty", skip_serializing_if = "Option::is_none")]
	pub cmmdty: Option<DerivativeCommodity2>,
	#[serde(rename = "Intrst", skip_serializing_if = "Option::is_none")]
	pub intrst: Option<DerivativeInterest3>,
	#[serde(rename = "FX", skip_serializing_if = "Option::is_none")]
	pub fx: Option<DerivativeForeignExchange3>,
}


// AssetClassCommodity3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodity3Choice {
	#[serde(rename = "Agrcltrl", skip_serializing_if = "Option::is_none")]
	pub agrcltrl: Option<AssetClassCommodityAgricultural1Choice>,
	#[serde(rename = "Nrgy", skip_serializing_if = "Option::is_none")]
	pub nrgy: Option<AssetClassCommodityEnergy1Choice>,
	#[serde(rename = "Envttl", skip_serializing_if = "Option::is_none")]
	pub envttl: Option<AssetClassCommodityEnvironmental1Choice>,
	#[serde(rename = "Frtlzr", skip_serializing_if = "Option::is_none")]
	pub frtlzr: Option<AssetClassCommodityFertilizer1Choice>,
	#[serde(rename = "Frght", skip_serializing_if = "Option::is_none")]
	pub frght: Option<AssetClassCommodityFreight1Choice>,
	#[serde(rename = "IndstrlPdct", skip_serializing_if = "Option::is_none")]
	pub indstrl_pdct: Option<AssetClassCommodityIndustrialProduct1Choice>,
	#[serde(rename = "Metl", skip_serializing_if = "Option::is_none")]
	pub metl: Option<AssetClassCommodityMetal1Choice>,
	#[serde(rename = "OthrC10", skip_serializing_if = "Option::is_none")]
	pub othr_c10: Option<AssetClassCommodityOtherC102Choice>,
	#[serde(rename = "Ppr", skip_serializing_if = "Option::is_none")]
	pub ppr: Option<AssetClassCommodityPaper1Choice>,
	#[serde(rename = "Plprpln", skip_serializing_if = "Option::is_none")]
	pub plprpln: Option<AssetClassCommodityPolypropylene1Choice>,
	#[serde(rename = "Infltn", skip_serializing_if = "Option::is_none")]
	pub infltn: Option<AssetClassCommodityInflation1>,
	#[serde(rename = "MultiCmmdtyExtc", skip_serializing_if = "Option::is_none")]
	pub multi_cmmdty_extc: Option<AssetClassCommodityMultiCommodityExotic1>,
	#[serde(rename = "OffclEcnmcSttstcs", skip_serializing_if = "Option::is_none")]
	pub offcl_ecnmc_sttstcs: Option<AssetClassCommodityOfficialEconomicStatistics1>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<AssetClassCommodityOther1>,
}


// AssetClassCommodityAgricultural1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityAgricultural1Choice {
	#[serde(rename = "GrnOilSeed", skip_serializing_if = "Option::is_none")]
	pub grn_oil_seed: Option<AgriculturalCommodityOilSeed1>,
	#[serde(rename = "Soft", skip_serializing_if = "Option::is_none")]
	pub soft: Option<AgriculturalCommoditySoft1>,
	#[serde(rename = "Ptt", skip_serializing_if = "Option::is_none")]
	pub ptt: Option<AgriculturalCommodityPotato1>,
	#[serde(rename = "OlvOil", skip_serializing_if = "Option::is_none")]
	pub olv_oil: Option<AgriculturalCommodityOliveOil1>,
	#[serde(rename = "Dairy", skip_serializing_if = "Option::is_none")]
	pub dairy: Option<AgriculturalCommodityDairy1>,
	#[serde(rename = "Frstry", skip_serializing_if = "Option::is_none")]
	pub frstry: Option<AgriculturalCommodityForestry1>,
	#[serde(rename = "Sfd", skip_serializing_if = "Option::is_none")]
	pub sfd: Option<AgriculturalCommoditySeafood1>,
	#[serde(rename = "LiveStock", skip_serializing_if = "Option::is_none")]
	pub live_stock: Option<AgriculturalCommodityLiveStock1>,
	#[serde(rename = "Grn", skip_serializing_if = "Option::is_none")]
	pub grn: Option<AgriculturalCommodityGrain1>,
}


// AssetClassCommodityEnergy1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityEnergy1Choice {
	#[serde(rename = "Elctrcty", skip_serializing_if = "Option::is_none")]
	pub elctrcty: Option<EnergyCommodityElectricity1>,
	#[serde(rename = "NtrlGas", skip_serializing_if = "Option::is_none")]
	pub ntrl_gas: Option<EnergyCommodityNaturalGas1>,
	#[serde(rename = "Oil", skip_serializing_if = "Option::is_none")]
	pub oil: Option<EnergyCommodityOil1>,
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
}


// AssetClassCommodityEnvironmental1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityEnvironmental1Choice {
	#[serde(rename = "Emssns", skip_serializing_if = "Option::is_none")]
	pub emssns: Option<EnvironmentalCommodityEmission1>,
	#[serde(rename = "Wthr", skip_serializing_if = "Option::is_none")]
	pub wthr: Option<EnvironmentalCommodityWeather1>,
	#[serde(rename = "CrbnRltd", skip_serializing_if = "Option::is_none")]
	pub crbn_rltd: Option<EnvironmentalCommodityCarbonRelated1>,
}


// AssetClassCommodityFertilizer1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityFertilizer1Choice {
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
}


// AssetClassCommodityFreight1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityFreight1Choice {
	#[serde(rename = "Dry", skip_serializing_if = "Option::is_none")]
	pub dry: Option<FreightCommodityDry1>,
	#[serde(rename = "Wet", skip_serializing_if = "Option::is_none")]
	pub wet: Option<FreightCommodityWet1>,
	#[serde(rename = "CntnrShip", skip_serializing_if = "Option::is_none")]
	pub cntnr_ship: Option<FreightCommodityContainerShip1>,
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


// AssetClassCommodityPaper1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityPaper1Choice {
	#[serde(rename = "CntnrBrd", skip_serializing_if = "Option::is_none")]
	pub cntnr_brd: Option<PaperCommodityContainerBoard1>,
	#[serde(rename = "Nwsprnt", skip_serializing_if = "Option::is_none")]
	pub nwsprnt: Option<PaperCommodityNewsprint1>,
	#[serde(rename = "Pulp", skip_serializing_if = "Option::is_none")]
	pub pulp: Option<PaperCommodityPulp1>,
	#[serde(rename = "RcvrdPpr", skip_serializing_if = "Option::is_none")]
	pub rcvrd_ppr: Option<PaperCommodityRecoveredPaper1>,
}


// AssetClassCommodityPolypropylene1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssetClassCommodityPolypropylene1Choice {
	#[serde(rename = "Plstc", skip_serializing_if = "Option::is_none")]
	pub plstc: Option<PolypropyleneCommodityPlastic1>,
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


// AssetClassDetailedSubProductType12Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType12Code {
	#[default]
	#[serde(rename = "TNKR")]
	CodeTNKR,
}


// AssetClassDetailedSubProductType14Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType14Code {
	#[default]
	#[serde(rename = "DBCR")]
	CodeDBCR,
}


// AssetClassDetailedSubProductType15Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType15Code {
	#[default]
	#[serde(rename = "MWHT")]
	CodeMWHT,
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


// AssetClassDetailedSubProductType4Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType4Code {
	#[default]
	#[serde(rename = "LAMP")]
	CodeLAMP,
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


// AssetClassDetailedSubProductType6Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType6Code {
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
}


// AssetClassDetailedSubProductType7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassDetailedSubProductType7Code {
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


// AssetClassTransactionType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetClassTransactionType1Code {
	#[default]
	#[serde(rename = "CRCK")]
	CodeCRCK,
	#[serde(rename = "DIFF")]
	CodeDIFF,
	#[serde(rename = "FUTR")]
	CodeFUTR,
	#[serde(rename = "MINI")]
	CodeMINI,
	#[serde(rename = "OPTN")]
	CodeOPTN,
	#[serde(rename = "OTCT")]
	CodeOTCT,
	#[serde(rename = "ORIT")]
	CodeORIT,
	#[serde(rename = "SWAP")]
	CodeSWAP,
	#[serde(rename = "TAPO")]
	CodeTAPO,
	#[serde(rename = "OTHR")]
	CodeOTHR,
}


// AssetFXSubProductType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetFXSubProductType1Code {
	#[default]
	#[serde(rename = "FXCR")]
	CodeFXCR,
	#[serde(rename = "FXEM")]
	CodeFXEM,
	#[serde(rename = "FXMJ")]
	CodeFXMJ,
}


// AssetPriceType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssetPriceType1Code {
	#[default]
	#[serde(rename = "ARGM")]
	CodeARGM,
	#[serde(rename = "BLTC")]
	CodeBLTC,
	#[serde(rename = "EXOF")]
	CodeEXOF,
	#[serde(rename = "GBCL")]
	CodeGBCL,
	#[serde(rename = "IHSM")]
	CodeIHSM,
	#[serde(rename = "OTHR")]
	CodeOTHR,
	#[serde(rename = "PLAT")]
	CodePLAT,
}


// BenchmarkCurveName2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BenchmarkCurveName2Code {
	#[default]
	#[serde(rename = "WIBO")]
	CodeWIBO,
	#[serde(rename = "TREA")]
	CodeTREA,
	#[serde(rename = "TIBO")]
	CodeTIBO,
	#[serde(rename = "TLBO")]
	CodeTLBO,
	#[serde(rename = "SWAP")]
	CodeSWAP,
	#[serde(rename = "STBO")]
	CodeSTBO,
	#[serde(rename = "PRBO")]
	CodePRBO,
	#[serde(rename = "PFAN")]
	CodePFAN,
	#[serde(rename = "NIBO")]
	CodeNIBO,
	#[serde(rename = "MAAA")]
	CodeMAAA,
	#[serde(rename = "MOSP")]
	CodeMOSP,
	#[serde(rename = "LIBO")]
	CodeLIBO,
	#[serde(rename = "LIBI")]
	CodeLIBI,
	#[serde(rename = "JIBA")]
	CodeJIBA,
	#[serde(rename = "ISDA")]
	CodeISDA,
	#[serde(rename = "GCFR")]
	CodeGCFR,
	#[serde(rename = "FUSW")]
	CodeFUSW,
	#[serde(rename = "EUCH")]
	CodeEUCH,
	#[serde(rename = "EUUS")]
	CodeEUUS,
	#[serde(rename = "EURI")]
	CodeEURI,
	#[serde(rename = "EONS")]
	CodeEONS,
	#[serde(rename = "EONA")]
	CodeEONA,
	#[serde(rename = "CIBO")]
	CodeCIBO,
	#[serde(rename = "CDOR")]
	CodeCDOR,
	#[serde(rename = "BUBO")]
	CodeBUBO,
	#[serde(rename = "BBSW")]
	CodeBBSW,
}


// BenchmarkCurveName5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkCurveName5Choice {
	#[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
	pub indx: Option<BenchmarkCurveName2Code>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max25Text>,
}


// BenchmarkCurveName6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkCurveName6Choice {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
	#[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
	pub indx: Option<BenchmarkCurveName2Code>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max25Text>,
}


// CFIOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[serde(rename = "$value")]
	pub cfi_oct2015_identifier: String,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// DebtInstrument2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DebtInstrument2 {
	#[serde(rename = "TtlIssdNmnlAmt")]
	pub ttl_issd_nmnl_amt: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "NmnlValPerUnit")]
	pub nmnl_val_per_unit: ActiveOrHistoricCurrencyAndAmount,
	#[serde(rename = "IntrstRate")]
	pub intrst_rate: InterestRate6Choice,
	#[serde(rename = "DebtSnrty", skip_serializing_if = "Option::is_none")]
	pub debt_snrty: Option<DebtInstrumentSeniorityType1Code>,
}


// DebtInstrumentSeniorityType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum DebtInstrumentSeniorityType1Code {
	#[default]
	#[serde(rename = "SBOD")]
	CodeSBOD,
	#[serde(rename = "SNDB")]
	CodeSNDB,
	#[serde(rename = "MZZD")]
	CodeMZZD,
	#[serde(rename = "JUND")]
	CodeJUND,
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "$value")]
	pub decimal_number: f64,
}


// DerivativeCommodity2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativeCommodity2 {
	#[serde(rename = "Pdct")]
	pub pdct: AssetClassCommodity3Choice,
	#[serde(rename = "TxTp", skip_serializing_if = "Option::is_none")]
	pub tx_tp: Option<AssetClassTransactionType1Code>,
	#[serde(rename = "FnlPricTp", skip_serializing_if = "Option::is_none")]
	pub fnl_pric_tp: Option<AssetPriceType1Code>,
}


// DerivativeForeignExchange3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativeForeignExchange3 {
	#[serde(rename = "FxTp", skip_serializing_if = "Option::is_none")]
	pub fx_tp: Option<AssetFXSubProductType1Code>,
	#[serde(rename = "OthrNtnlCcy", skip_serializing_if = "Option::is_none")]
	pub othr_ntnl_ccy: Option<ActiveOrHistoricCurrencyCode>,
}


// DerivativeInstrument5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativeInstrument5 {
	#[serde(rename = "XpryDt", skip_serializing_if = "Option::is_none")]
	pub xpry_dt: Option<String>,
	#[serde(rename = "PricMltplr", skip_serializing_if = "Option::is_none")]
	pub pric_mltplr: Option<f64>,
	#[serde(rename = "UndrlygInstrm", skip_serializing_if = "Option::is_none")]
	pub undrlyg_instrm: Option<FinancialInstrumentIdentification5Choice>,
	#[serde(rename = "OptnTp", skip_serializing_if = "Option::is_none")]
	pub optn_tp: Option<OptionType2Code>,
	#[serde(rename = "StrkPric", skip_serializing_if = "Option::is_none")]
	pub strk_pric: Option<SecuritiesTransactionPrice4Choice>,
	#[serde(rename = "OptnExrcStyle", skip_serializing_if = "Option::is_none")]
	pub optn_exrc_style: Option<OptionStyle7Code>,
	#[serde(rename = "DlvryTp", skip_serializing_if = "Option::is_none")]
	pub dlvry_tp: Option<PhysicalTransferType4Code>,
	#[serde(rename = "AsstClssSpcfcAttrbts", skip_serializing_if = "Option::is_none")]
	pub asst_clss_spcfc_attrbts: Option<AssetClass2>,
}


// DerivativeInterest3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativeInterest3 {
	#[serde(rename = "IntrstRate")]
	pub intrst_rate: FloatingInterestRate8,
	#[serde(rename = "FrstLegIntrstRate", skip_serializing_if = "Option::is_none")]
	pub frst_leg_intrst_rate: Option<InterestRate8Choice>,
	#[serde(rename = "OthrNtnlCcy", skip_serializing_if = "Option::is_none")]
	pub othr_ntnl_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "OthrLegIntrstRate", skip_serializing_if = "Option::is_none")]
	pub othr_leg_intrst_rate: Option<InterestRate8Choice>,
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


// EnergyCommodityNaturalGas1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityNaturalGas1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType7Code,
	#[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
	pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType6Code>,
}


// EnergyCommodityOil1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityOil1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType8Code,
	#[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
	pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType7Code>,
}


// EnergyCommodityRenewableEnergy1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnergyCommodityRenewableEnergy1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType2Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType28Code,
}


// EnvironmentalCommodityCarbonRelated1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCommodityCarbonRelated1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType3Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType29Code,
}


// EnvironmentalCommodityEmission1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCommodityEmission1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType3Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType10Code,
	#[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
	pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType8Code>,
}


// EnvironmentalCommodityWeather1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EnvironmentalCommodityWeather1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType3Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType30Code,
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


// FinancialInstrument48Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument48Choice {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
	#[serde(rename = "Indx", skip_serializing_if = "Option::is_none")]
	pub indx: Option<FinancialInstrument58>,
}


// FinancialInstrument53 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument53 {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<Vec<ISINOct2015Identifier>>,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<Vec<LEIIdentifier>>,
}


// FinancialInstrument58 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument58 {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISINOct2015Identifier>,
	#[serde(rename = "Nm")]
	pub nm: FloatingInterestRate8,
}


// FinancialInstrumentIdentification5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentIdentification5Choice {
	#[serde(rename = "Sngl", skip_serializing_if = "Option::is_none")]
	pub sngl: Option<FinancialInstrument48Choice>,
	#[serde(rename = "Bskt", skip_serializing_if = "Option::is_none")]
	pub bskt: Option<FinancialInstrument53>,
}


// FinancialInstrumentReportingReferenceDataReportV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentReportingReferenceDataReportV02 {
	#[serde(rename = "RptHdr")]
	pub rpt_hdr: SecuritiesMarketReportHeader1,
	#[serde(rename = "RefData")]
	pub ref_data: Vec<SecuritiesReferenceDataReport6>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// FloatingInterestRate6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FloatingInterestRate6 {
	#[serde(rename = "RefRate")]
	pub ref_rate: BenchmarkCurveName6Choice,
	#[serde(rename = "Term")]
	pub term: InterestRateContractTerm2,
	#[serde(rename = "BsisPtSprd")]
	pub bsis_pt_sprd: f64,
}


// FloatingInterestRate8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FloatingInterestRate8 {
	#[serde(rename = "RefRate")]
	pub ref_rate: BenchmarkCurveName5Choice,
	#[serde(rename = "Term", skip_serializing_if = "Option::is_none")]
	pub term: Option<InterestRateContractTerm2>,
}


// FreightCommodityContainerShip1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityContainerShip1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType4Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType46Code,
}


// FreightCommodityDry1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityDry1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType4Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType31Code,
	#[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
	pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType14Code>,
}


// FreightCommodityWet1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FreightCommodityWet1 {
	#[serde(rename = "BasePdct")]
	pub base_pdct: AssetClassProductType4Code,
	#[serde(rename = "SubPdct")]
	pub sub_pdct: AssetClassSubProductType32Code,
	#[serde(rename = "AddtlSubPdct", skip_serializing_if = "Option::is_none")]
	pub addtl_sub_pdct: Option<AssetClassDetailedSubProductType12Code>,
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


// InterestRate6Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRate6Choice {
	#[serde(rename = "Fxd", skip_serializing_if = "Option::is_none")]
	pub fxd: Option<f64>,
	#[serde(rename = "Fltg", skip_serializing_if = "Option::is_none")]
	pub fltg: Option<FloatingInterestRate6>,
}


// InterestRate8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InterestRate8Choice {
	#[serde(rename = "Fxd", skip_serializing_if = "Option::is_none")]
	pub fxd: Option<f64>,
	#[serde(rename = "Fltg", skip_serializing_if = "Option::is_none")]
	pub fltg: Option<FloatingInterestRate8>,
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


// MICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MICIdentifier {
	#[serde(rename = "$value")]
	pub mic_identifier: String,
}


// Max25Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max25Text {
	#[serde(rename = "$value")]
	pub max25_text: String,
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


// Max50Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max50Text {
	#[serde(rename = "$value")]
	pub max50_text: String,
}


// Max5Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max5Number {
	#[serde(rename = "$value")]
	pub max5_number: f64,
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


// NonNegativeDecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NonNegativeDecimalNumber {
	#[serde(rename = "$value")]
	pub non_negative_decimal_number: f64,
}


// OptionStyle7Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OptionStyle7Code {
	#[default]
	#[serde(rename = "AMER")]
	CodeAMER,
	#[serde(rename = "ASIA")]
	CodeASIA,
	#[serde(rename = "BERM")]
	CodeBERM,
	#[serde(rename = "EURO")]
	CodeEURO,
	#[serde(rename = "OTHR")]
	CodeOTHR,
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


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "$value")]
	pub percentage_rate: f64,
}


// Period2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// Period4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period4Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "FrDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt: Option<String>,
	#[serde(rename = "ToDt", skip_serializing_if = "Option::is_none")]
	pub to_dt: Option<String>,
	#[serde(rename = "FrDtToDt", skip_serializing_if = "Option::is_none")]
	pub fr_dt_to_dt: Option<Period2>,
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


// RecordTechnicalData4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RecordTechnicalData4 {
	#[serde(rename = "IncnsstncyInd", skip_serializing_if = "Option::is_none")]
	pub incnsstncy_ind: Option<bool>,
	#[serde(rename = "LastUpd", skip_serializing_if = "Option::is_none")]
	pub last_upd: Option<String>,
	#[serde(rename = "SubmissnDtTm", skip_serializing_if = "Option::is_none")]
	pub submissn_dt_tm: Option<String>,
	#[serde(rename = "RlvntCmptntAuthrty", skip_serializing_if = "Option::is_none")]
	pub rlvnt_cmptnt_authrty: Option<CountryCode>,
	#[serde(rename = "PblctnPrd", skip_serializing_if = "Option::is_none")]
	pub pblctn_prd: Option<Period4Choice>,
	#[serde(rename = "NvrPblshd", skip_serializing_if = "Option::is_none")]
	pub nvr_pblshd: Option<bool>,
	#[serde(rename = "RlvntTradgVn", skip_serializing_if = "Option::is_none")]
	pub rlvnt_tradg_vn: Option<MICIdentifier>,
}


// SecuritiesMarketReportHeader1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesMarketReportHeader1 {
	#[serde(rename = "RptgNtty")]
	pub rptg_ntty: TradingVenueIdentification1Choice,
	#[serde(rename = "RptgPrd")]
	pub rptg_prd: Period4Choice,
	#[serde(rename = "SubmissnDtTm", skip_serializing_if = "Option::is_none")]
	pub submissn_dt_tm: Option<String>,
}


// SecuritiesReferenceDataReport6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesReferenceDataReport6 {
	#[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
	pub tech_rcrd_id: Option<Max35Text>,
	#[serde(rename = "FinInstrmGnlAttrbts")]
	pub fin_instrm_gnl_attrbts: SecurityInstrumentDescription9,
	#[serde(rename = "Issr")]
	pub issr: LEIIdentifier,
	#[serde(rename = "TradgVnRltdAttrbts")]
	pub tradg_vn_rltd_attrbts: Vec<TradingVenueAttributes1>,
	#[serde(rename = "DebtInstrmAttrbts", skip_serializing_if = "Option::is_none")]
	pub debt_instrm_attrbts: Option<DebtInstrument2>,
	#[serde(rename = "DerivInstrmAttrbts", skip_serializing_if = "Option::is_none")]
	pub deriv_instrm_attrbts: Option<DerivativeInstrument5>,
	#[serde(rename = "TechAttrbts", skip_serializing_if = "Option::is_none")]
	pub tech_attrbts: Option<RecordTechnicalData4>,
}


// SecuritiesTransactionPrice1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice1 {
	#[serde(rename = "Pdg")]
	pub pdg: PriceStatus1Code,
	#[serde(rename = "Ccy", skip_serializing_if = "Option::is_none")]
	pub ccy: Option<ActiveOrHistoricCurrencyCode>,
}


// SecuritiesTransactionPrice2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice2Choice {
	#[serde(rename = "MntryVal", skip_serializing_if = "Option::is_none")]
	pub mntry_val: Option<AmountAndDirection61>,
	#[serde(rename = "Pctg", skip_serializing_if = "Option::is_none")]
	pub pctg: Option<f64>,
	#[serde(rename = "Yld", skip_serializing_if = "Option::is_none")]
	pub yld: Option<f64>,
	#[serde(rename = "BsisPts", skip_serializing_if = "Option::is_none")]
	pub bsis_pts: Option<f64>,
}


// SecuritiesTransactionPrice4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionPrice4Choice {
	#[serde(rename = "Pric", skip_serializing_if = "Option::is_none")]
	pub pric: Option<SecuritiesTransactionPrice2Choice>,
	#[serde(rename = "NoPric", skip_serializing_if = "Option::is_none")]
	pub no_pric: Option<SecuritiesTransactionPrice1>,
}


// SecurityInstrumentDescription9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityInstrumentDescription9 {
	#[serde(rename = "Id")]
	pub id: ISINOct2015Identifier,
	#[serde(rename = "FullNm")]
	pub full_nm: Max350Text,
	#[serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none")]
	pub shrt_nm: Option<Max35Text>,
	#[serde(rename = "ClssfctnTp")]
	pub clssfctn_tp: CFIOct2015Identifier,
	#[serde(rename = "NtnlCcy")]
	pub ntnl_ccy: ActiveOrHistoricCurrencyCode,
	#[serde(rename = "CmmdtyDerivInd")]
	pub cmmdty_deriv_ind: bool,
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


// TradingVenue2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TradingVenue2Code {
	#[default]
	#[serde(rename = "APPA")]
	CodeAPPA,
	#[serde(rename = "CTPS")]
	CodeCTPS,
}


// TradingVenueAttributes1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingVenueAttributes1 {
	#[serde(rename = "Id")]
	pub id: MICIdentifier,
	#[serde(rename = "IssrReq")]
	pub issr_req: bool,
	#[serde(rename = "AdmssnApprvlDtByIssr", skip_serializing_if = "Option::is_none")]
	pub admssn_apprvl_dt_by_issr: Option<String>,
	#[serde(rename = "ReqForAdmssnDt", skip_serializing_if = "Option::is_none")]
	pub req_for_admssn_dt: Option<String>,
	#[serde(rename = "FrstTradDt", skip_serializing_if = "Option::is_none")]
	pub frst_trad_dt: Option<String>,
	#[serde(rename = "TermntnDt", skip_serializing_if = "Option::is_none")]
	pub termntn_dt: Option<String>,
}


// TradingVenueIdentification1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingVenueIdentification1Choice {
	#[serde(rename = "MktIdCd", skip_serializing_if = "Option::is_none")]
	pub mkt_id_cd: Option<MICIdentifier>,
	#[serde(rename = "NtlCmptntAuthrty", skip_serializing_if = "Option::is_none")]
	pub ntl_cmptnt_authrty: Option<CountryCode>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<TradingVenueIdentification2>,
}


// TradingVenueIdentification2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingVenueIdentification2 {
	#[serde(rename = "Id")]
	pub id: Max50Text,
	#[serde(rename = "Tp")]
	pub tp: TradingVenue2Code,
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "$value")]
	pub true_false_indicator: bool,
}
