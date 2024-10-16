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


// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_currency_and_amount_simple_type: f64,
}


// ActiveCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmount {
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


// ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType {
	#[serde(rename = "$value")]
	pub active_or_historic_currency_and13_decimal_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAnd13DecimalAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmount {
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


// AddressType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AddressType1Code {
	#[default]
	#[serde(rename = "HOME")]
	CodeHOME,
	#[serde(rename = "BIZZ")]
	CodeBIZZ,

}


// AddressType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AddressType2Code {
	#[default]
	#[serde(rename = "ADDR")]
	CodeADDR,
	#[serde(rename = "PBOX")]
	CodePBOX,
	#[serde(rename = "HOME")]
	CodeHOME,
	#[serde(rename = "BIZZ")]
	CodeBIZZ,
	#[serde(rename = "MLTO")]
	CodeMLTO,
	#[serde(rename = "DLVY")]
	CodeDLVY,

}


// AmountOrPercentageRange1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AmountOrPercentageRange1 {
	#[serde(rename = "Opr", skip_serializing_if = "Option::is_none")]
	pub opr: Option<Operation1Code>,
	#[serde(rename = "Term", skip_serializing_if = "Option::is_none")]
	pub term: Option<Vec<Term1>>,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
}


// Appearance1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Appearance1Code {
	#[default]
	#[serde(rename = "DELI")]
	CodeDELI,
	#[serde(rename = "NDEL")]
	CodeNDEL,
	#[serde(rename = "LIMI")]
	CodeLIMI,
	#[serde(rename = "BENT")]
	CodeBENT,
	#[serde(rename = "DFBE")]
	CodeDFBE,
	#[serde(rename = "DLBE")]
	CodeDLBE,
	#[serde(rename = "TMPG")]
	CodeTMPG,
	#[serde(rename = "GLOB")]
	CodeGLOB,

}


// Appearance3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Appearance3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Appearance1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// AssignmentMethod1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AssignmentMethod1Code {
	#[default]
	#[serde(rename = "RAND")]
	CodeRAND,
	#[serde(rename = "PROR")]
	CodePROR,

}


// AssignmentMethod2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AssignmentMethod2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<AssignmentMethod1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BaseOneRate {
	#[serde(rename = "$value")]
	pub base_one_rate: f64,
}


// BenchmarkCurve6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkCurve6 {
	#[serde(rename = "Sprd", skip_serializing_if = "Option::is_none")]
	pub sprd: Option<f64>,
	#[serde(rename = "BchmkId", skip_serializing_if = "Option::is_none")]
	pub bchmk_id: Option<SecurityIdentification39>,
	#[serde(rename = "BchmkPric", skip_serializing_if = "Option::is_none")]
	pub bchmk_pric: Option<Price8>,
	#[serde(rename = "BchmkCrvCcy", skip_serializing_if = "Option::is_none")]
	pub bchmk_crv_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "BchmkCrvNm", skip_serializing_if = "Option::is_none")]
	pub bchmk_crv_nm: Option<BenchmarkCurveName7Choice>,
	#[serde(rename = "BchmkCrvPt", skip_serializing_if = "Option::is_none")]
	pub bchmk_crv_pt: Option<Max256Text>,
}


// BenchmarkCurveName1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum BenchmarkCurveName1Code {
	#[default]
	#[serde(rename = "MAAA")]
	CodeMAAA,
	#[serde(rename = "FUSW")]
	CodeFUSW,
	#[serde(rename = "LIBI")]
	CodeLIBI,
	#[serde(rename = "LIBO")]
	CodeLIBO,
	#[serde(rename = "SWAP")]
	CodeSWAP,
	#[serde(rename = "TREA")]
	CodeTREA,
	#[serde(rename = "EURI")]
	CodeEURI,
	#[serde(rename = "PFAN")]
	CodePFAN,

}


// BenchmarkCurveName7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BenchmarkCurveName7Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<BenchmarkCurveName1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// CFIOct2015Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[serde(rename = "$value")]
	pub cfi_oct2015_identifier: String,
}


// CalculationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CalculationType1Code {
	#[default]
	#[serde(rename = "AFTX")]
	CodeAFTX,
	#[serde(rename = "ANNU")]
	CodeANNU,
	#[serde(rename = "ISSU")]
	CodeISSU,
	#[serde(rename = "AVMA")]
	CodeAVMA,
	#[serde(rename = "BOOK")]
	CodeBOOK,
	#[serde(rename = "YTNC")]
	CodeYTNC,
	#[serde(rename = "CHCL")]
	CodeCHCL,
	#[serde(rename = "CLOS")]
	CodeCLOS,
	#[serde(rename = "CMPD")]
	CodeCMPD,
	#[serde(rename = "CUYI")]
	CodeCUYI,
	#[serde(rename = "TRGR")]
	CodeTRGR,
	#[serde(rename = "GVEQ")]
	CodeGVEQ,
	#[serde(rename = "FLAS")]
	CodeFLAS,
	#[serde(rename = "NVFL")]
	CodeNVFL,
	#[serde(rename = "LSCL")]
	CodeLSCL,
	#[serde(rename = "LSMT")]
	CodeLSMT,
	#[serde(rename = "LSQR")]
	CodeLSQR,
	#[serde(rename = "LSYR")]
	CodeLSYR,
	#[serde(rename = "LGAL")]
	CodeLGAL,
	#[serde(rename = "MARK")]
	CodeMARK,
	#[serde(rename = "YTMA")]
	CodeYTMA,
	#[serde(rename = "NXRF")]
	CodeNXRF,
	#[serde(rename = "PNAV")]
	CodePNAV,
	#[serde(rename = "NXPT")]
	CodeNXPT,
	#[serde(rename = "PRCL")]
	CodePRCL,
	#[serde(rename = "PRYL")]
	CodePRYL,
	#[serde(rename = "SEMI")]
	CodeSEMI,
	#[serde(rename = "SHLF")]
	CodeSHLF,
	#[serde(rename = "SPLL")]
	CodeSPLL,
	#[serde(rename = "TXQV")]
	CodeTXQV,
	#[serde(rename = "TTDT")]
	CodeTTDT,
	#[serde(rename = "TRYL")]
	CodeTRYL,
	#[serde(rename = "WRST")]
	CodeWRST,

}


// CalculationType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CalculationType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<CalculationType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// CallType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum CallType1Code {
	#[default]
	#[serde(rename = "LOTT")]
	CodeLOTT,
	#[serde(rename = "PRTA")]
	CodePRTA,

}


// CallType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CallType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<CallType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// ClassificationType2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ClassificationType2 {
	#[serde(rename = "ClssfctnFinInstrm", skip_serializing_if = "Option::is_none")]
	pub clssfctn_fin_instrm: Option<CFIOct2015Identifier>,
	#[serde(rename = "FinInstrmPdctTpCd", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_pdct_tp_cd: Option<ExternalFinancialInstrumentProductType1Code>,
	#[serde(rename = "AltrnClssfctn", skip_serializing_if = "Option::is_none")]
	pub altrn_clssfctn: Option<Vec<GenericIdentification36>>,
}


// CommonFinancialInstrumentAttributes12 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommonFinancialInstrumentAttributes12 {
	#[serde(rename = "SctySts", skip_serializing_if = "Option::is_none")]
	pub scty_sts: Option<SecurityStatus3Choice>,
	#[serde(rename = "ISOSctyLngNm", skip_serializing_if = "Option::is_none")]
	pub iso_scty_lng_nm: Option<Max350Text>,
	#[serde(rename = "ISOSctyShrtNm", skip_serializing_if = "Option::is_none")]
	pub iso_scty_shrt_nm: Option<Max35Text>,
	#[serde(rename = "NmVldFr", skip_serializing_if = "Option::is_none")]
	pub nm_vld_fr: Option<DateAndDateTime2Choice>,
	#[serde(rename = "DnmtnCcy", skip_serializing_if = "Option::is_none")]
	pub dnmtn_ccy: Option<ActiveOrHistoricCurrencyCode>,
	#[serde(rename = "CertNb", skip_serializing_if = "Option::is_none")]
	pub cert_nb: Option<Max35Text>,
	#[serde(rename = "CtrctVrsnNb", skip_serializing_if = "Option::is_none")]
	pub ctrct_vrsn_nb: Option<f64>,
	#[serde(rename = "CpnAttchdNb", skip_serializing_if = "Option::is_none")]
	pub cpn_attchd_nb: Option<Max3NumericText>,
	#[serde(rename = "TaxLotNb", skip_serializing_if = "Option::is_none")]
	pub tax_lot_nb: Option<Max15NumericText>,
	#[serde(rename = "PoolNb", skip_serializing_if = "Option::is_none")]
	pub pool_nb: Option<Max15NumericText>,
	#[serde(rename = "CvrdInd", skip_serializing_if = "Option::is_none")]
	pub cvrd_ind: Option<bool>,
	#[serde(rename = "LglRstrctns", skip_serializing_if = "Option::is_none")]
	pub lgl_rstrctns: Option<LegalRestrictions4Choice>,
	#[serde(rename = "PosLmt", skip_serializing_if = "Option::is_none")]
	pub pos_lmt: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "NearTermPosLmt", skip_serializing_if = "Option::is_none")]
	pub near_term_pos_lmt: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "ListgDt", skip_serializing_if = "Option::is_none")]
	pub listg_dt: Option<String>,
	#[serde(rename = "RcrdDt", skip_serializing_if = "Option::is_none")]
	pub rcrd_dt: Option<String>,
	#[serde(rename = "XpryDt", skip_serializing_if = "Option::is_none")]
	pub xpry_dt: Option<String>,
	#[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
	pub purp: Option<Max256Text>,
	#[serde(rename = "ClssfctnTp", skip_serializing_if = "Option::is_none")]
	pub clssfctn_tp: Option<ClassificationType2>,
	#[serde(rename = "Issnc", skip_serializing_if = "Option::is_none")]
	pub issnc: Option<Issuance5>,
	#[serde(rename = "TradgMkt", skip_serializing_if = "Option::is_none")]
	pub tradg_mkt: Option<Vec<TradingParameters2>>,
	#[serde(rename = "SprdAndBchmkCrv", skip_serializing_if = "Option::is_none")]
	pub sprd_and_bchmk_crv: Option<Vec<BenchmarkCurve6>>,
	#[serde(rename = "PutTp", skip_serializing_if = "Option::is_none")]
	pub put_tp: Option<PutType3Choice>,
	#[serde(rename = "CallTp", skip_serializing_if = "Option::is_none")]
	pub call_tp: Option<CallType3Choice>,
	#[serde(rename = "FngbInd", skip_serializing_if = "Option::is_none")]
	pub fngb_ind: Option<bool>,
	#[serde(rename = "Cnfdtl", skip_serializing_if = "Option::is_none")]
	pub cnfdtl: Option<bool>,
	#[serde(rename = "PrvtPlcmnt", skip_serializing_if = "Option::is_none")]
	pub prvt_plcmnt: Option<bool>,
	#[serde(rename = "ConvtblInd", skip_serializing_if = "Option::is_none")]
	pub convtbl_ind: Option<bool>,
	#[serde(rename = "ConvsPrd", skip_serializing_if = "Option::is_none")]
	pub convs_prd: Option<DateTimePeriod1>,
	#[serde(rename = "ConvsRatioNmrtr", skip_serializing_if = "Option::is_none")]
	pub convs_ratio_nmrtr: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "ConvsRatioDnmtr", skip_serializing_if = "Option::is_none")]
	pub convs_ratio_dnmtr: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "PmryPlcOfDpst", skip_serializing_if = "Option::is_none")]
	pub pmry_plc_of_dpst: Option<PartyIdentification136>,
	#[serde(rename = "TradgMtd", skip_serializing_if = "Option::is_none")]
	pub tradg_mtd: Option<UnitOrFaceAmount1Choice>,
	#[serde(rename = "TEFRARule", skip_serializing_if = "Option::is_none")]
	pub tefra_rule: Option<TEFRARules3Choice>,
	#[serde(rename = "SrNb", skip_serializing_if = "Option::is_none")]
	pub sr_nb: Option<Max16Text>,
	#[serde(rename = "Clss", skip_serializing_if = "Option::is_none")]
	pub clss: Option<Max16Text>,
	#[serde(rename = "WhldgTaxRgm", skip_serializing_if = "Option::is_none")]
	pub whldg_tax_rgm: Option<Vec<SecurityWithHoldingTax1>>,
	#[serde(rename = "PmtSts", skip_serializing_if = "Option::is_none")]
	pub pmt_sts: Option<SecuritiesPaymentStatus5Choice>,
	#[serde(rename = "InitlPhysForm", skip_serializing_if = "Option::is_none")]
	pub initl_phys_form: Option<InitialPhysicalForm4Choice>,
	#[serde(rename = "AftrXchgPhysForm", skip_serializing_if = "Option::is_none")]
	pub aftr_xchg_phys_form: Option<InitialPhysicalForm3Choice>,
	#[serde(rename = "CmonSfkpr", skip_serializing_if = "Option::is_none")]
	pub cmon_sfkpr: Option<PartyIdentification177Choice>,
	#[serde(rename = "RedTp", skip_serializing_if = "Option::is_none")]
	pub red_tp: Option<MaturityRedemptionType3Choice>,
	#[serde(rename = "RedPmtCcy", skip_serializing_if = "Option::is_none")]
	pub red_pmt_ccy: Option<ActiveCurrencyCode>,
	#[serde(rename = "Rstrctn", skip_serializing_if = "Option::is_none")]
	pub rstrctn: Option<Vec<SecurityRestriction3>>,
	#[serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_id: Option<SecurityIdentification39>,
	#[serde(rename = "SttlmInf", skip_serializing_if = "Option::is_none")]
	pub sttlm_inf: Option<Vec<SettlementInformation17>>,
	#[serde(rename = "FinInstrmForm", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_form: Option<FinancialInstrumentForm2>,
	#[serde(rename = "CtctNm", skip_serializing_if = "Option::is_none")]
	pub ctct_nm: Option<Organisation38>,
	#[serde(rename = "LeadMgr", skip_serializing_if = "Option::is_none")]
	pub lead_mgr: Option<Organisation38>,
	#[serde(rename = "PrncplPngAgt", skip_serializing_if = "Option::is_none")]
	pub prncpl_png_agt: Option<Organisation38>,
	#[serde(rename = "PngAgt", skip_serializing_if = "Option::is_none")]
	pub png_agt: Option<Organisation38>,
	#[serde(rename = "Dpstry", skip_serializing_if = "Option::is_none")]
	pub dpstry: Option<Organisation38>,
	#[serde(rename = "UndrlygRsk", skip_serializing_if = "Option::is_none")]
	pub undrlyg_rsk: Option<Organisation38>,
}


// CommunicationAddress3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CommunicationAddress3 {
	#[serde(rename = "Email", skip_serializing_if = "Option::is_none")]
	pub email: Option<Max256Text>,
	#[serde(rename = "Phne", skip_serializing_if = "Option::is_none")]
	pub phne: Option<PhoneNumber>,
	#[serde(rename = "Mob", skip_serializing_if = "Option::is_none")]
	pub mob: Option<PhoneNumber>,
	#[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
	pub fax_nb: Option<PhoneNumber>,
	#[serde(rename = "TlxAdr", skip_serializing_if = "Option::is_none")]
	pub tlx_adr: Option<Max35Text>,
	#[serde(rename = "URLAdr", skip_serializing_if = "Option::is_none")]
	pub url_adr: Option<Max256Text>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}


// DateAndDateTime2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTime2Choice {
	#[serde(rename = "Dt", skip_serializing_if = "Option::is_none")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm", skip_serializing_if = "Option::is_none")]
	pub dt_tm: Option<String>,
}


// DateTimePeriod1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod1 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: String,
}


// DateTimePeriod1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod1Choice {
	#[serde(rename = "FrDtTm", skip_serializing_if = "Option::is_none")]
	pub fr_dt_tm: Option<String>,
	#[serde(rename = "ToDtTm", skip_serializing_if = "Option::is_none")]
	pub to_dt_tm: Option<String>,
	#[serde(rename = "DtTmRg", skip_serializing_if = "Option::is_none")]
	pub dt_tm_rg: Option<DateTimePeriod1>,
}


// DateTimePeriod2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateTimePeriod2 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm", skip_serializing_if = "Option::is_none")]
	pub to_dt_tm: Option<String>,
}


// Debt5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Debt5 {
	#[serde(rename = "PmtCcy", skip_serializing_if = "Option::is_none")]
	pub pmt_ccy: Option<ActiveCurrencyCode>,
	#[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
	pub face_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "PmtFrqcy", skip_serializing_if = "Option::is_none")]
	pub pmt_frqcy: Option<Frequency35Choice>,
	#[serde(rename = "IntrstFxgDt", skip_serializing_if = "Option::is_none")]
	pub intrst_fxg_dt: Option<String>,
	#[serde(rename = "DtdDt", skip_serializing_if = "Option::is_none")]
	pub dtd_dt: Option<String>,
	#[serde(rename = "FrstPmtDt", skip_serializing_if = "Option::is_none")]
	pub frst_pmt_dt: Option<String>,
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "NxtCpnDt", skip_serializing_if = "Option::is_none")]
	pub nxt_cpn_dt: Option<String>,
	#[serde(rename = "PutblDt", skip_serializing_if = "Option::is_none")]
	pub putbl_dt: Option<String>,
	#[serde(rename = "NxtCllblDt", skip_serializing_if = "Option::is_none")]
	pub nxt_cllbl_dt: Option<String>,
	#[serde(rename = "NxtFctrDt", skip_serializing_if = "Option::is_none")]
	pub nxt_fctr_dt: Option<String>,
	#[serde(rename = "XprtnDt", skip_serializing_if = "Option::is_none")]
	pub xprtn_dt: Option<String>,
	#[serde(rename = "PmtDrctnInd", skip_serializing_if = "Option::is_none")]
	pub pmt_drctn_ind: Option<bool>,
	#[serde(rename = "IntrstRate", skip_serializing_if = "Option::is_none")]
	pub intrst_rate: Option<f64>,
	#[serde(rename = "NxtIntrstRate", skip_serializing_if = "Option::is_none")]
	pub nxt_intrst_rate: Option<f64>,
	#[serde(rename = "OddCpnInd", skip_serializing_if = "Option::is_none")]
	pub odd_cpn_ind: Option<bool>,
	#[serde(rename = "CllblInd", skip_serializing_if = "Option::is_none")]
	pub cllbl_ind: Option<bool>,
	#[serde(rename = "CPPrgm", skip_serializing_if = "Option::is_none")]
	pub cp_prgm: Option<f64>,
	#[serde(rename = "CPRegnTp", skip_serializing_if = "Option::is_none")]
	pub cp_regn_tp: Option<Max350Text>,
	#[serde(rename = "IntrstAcrlDt", skip_serializing_if = "Option::is_none")]
	pub intrst_acrl_dt: Option<String>,
	#[serde(rename = "PutblInd", skip_serializing_if = "Option::is_none")]
	pub putbl_ind: Option<bool>,
	#[serde(rename = "PreFnddInd", skip_serializing_if = "Option::is_none")]
	pub pre_fndd_ind: Option<bool>,
	#[serde(rename = "EscrwdInd", skip_serializing_if = "Option::is_none")]
	pub escrwd_ind: Option<bool>,
	#[serde(rename = "PerptlInd", skip_serializing_if = "Option::is_none")]
	pub perptl_ind: Option<bool>,
	#[serde(rename = "SubrdntdInd", skip_serializing_if = "Option::is_none")]
	pub subrdntd_ind: Option<bool>,
	#[serde(rename = "XtndblInd", skip_serializing_if = "Option::is_none")]
	pub xtndbl_ind: Option<bool>,
	#[serde(rename = "XtndblPrd", skip_serializing_if = "Option::is_none")]
	pub xtndbl_prd: Option<DateTimePeriod1Choice>,
	#[serde(rename = "VarblRateInd", skip_serializing_if = "Option::is_none")]
	pub varbl_rate_ind: Option<bool>,
	#[serde(rename = "OverAlltmtAmt", skip_serializing_if = "Option::is_none")]
	pub over_alltmt_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "OverAlltmtRate", skip_serializing_if = "Option::is_none")]
	pub over_alltmt_rate: Option<f64>,
	#[serde(rename = "AmtsblInd", skip_serializing_if = "Option::is_none")]
	pub amtsbl_ind: Option<bool>,
	#[serde(rename = "IntrstClctnMtd", skip_serializing_if = "Option::is_none")]
	pub intrst_clctn_mtd: Option<Max70Text>,
	#[serde(rename = "CptlsdIntrst", skip_serializing_if = "Option::is_none")]
	pub cptlsd_intrst: Option<DistributionPolicy2Choice>,
	#[serde(rename = "ActlDnmtnAmt", skip_serializing_if = "Option::is_none")]
	pub actl_dnmtn_amt: Option<Vec<ActiveCurrencyAndAmount>>,
	#[serde(rename = "CurFctr", skip_serializing_if = "Option::is_none")]
	pub cur_fctr: Option<f64>,
	#[serde(rename = "NxtFctr", skip_serializing_if = "Option::is_none")]
	pub nxt_fctr: Option<f64>,
	#[serde(rename = "PrvsFctr", skip_serializing_if = "Option::is_none")]
	pub prvs_fctr: Option<f64>,
	#[serde(rename = "Pcs", skip_serializing_if = "Option::is_none")]
	pub pcs: Option<f64>,
	#[serde(rename = "PlsMax", skip_serializing_if = "Option::is_none")]
	pub pls_max: Option<f64>,
	#[serde(rename = "PlsPerMln", skip_serializing_if = "Option::is_none")]
	pub pls_per_mln: Option<f64>,
	#[serde(rename = "PlsPerLot", skip_serializing_if = "Option::is_none")]
	pub pls_per_lot: Option<f64>,
	#[serde(rename = "PlsPerTrad", skip_serializing_if = "Option::is_none")]
	pub pls_per_trad: Option<f64>,
	#[serde(rename = "CstPrePmtPnltyInd", skip_serializing_if = "Option::is_none")]
	pub cst_pre_pmt_pnlty_ind: Option<bool>,
	#[serde(rename = "LotId", skip_serializing_if = "Option::is_none")]
	pub lot_id: Option<Max35Text>,
	#[serde(rename = "CstPrePmtYld", skip_serializing_if = "Option::is_none")]
	pub cst_pre_pmt_yld: Option<f64>,
	#[serde(rename = "WghtdAvrgCpn", skip_serializing_if = "Option::is_none")]
	pub wghtd_avrg_cpn: Option<f64>,
	#[serde(rename = "WghtdAvrgLife", skip_serializing_if = "Option::is_none")]
	pub wghtd_avrg_life: Option<f64>,
	#[serde(rename = "WghtdAvrgLn", skip_serializing_if = "Option::is_none")]
	pub wghtd_avrg_ln: Option<f64>,
	#[serde(rename = "WghtdAvrgMtrty", skip_serializing_if = "Option::is_none")]
	pub wghtd_avrg_mtrty: Option<f64>,
	#[serde(rename = "InsrdInd", skip_serializing_if = "Option::is_none")]
	pub insrd_ind: Option<bool>,
	#[serde(rename = "BkQlfdInd", skip_serializing_if = "Option::is_none")]
	pub bk_qlfd_ind: Option<bool>,
	#[serde(rename = "YldClctn", skip_serializing_if = "Option::is_none")]
	pub yld_clctn: Option<Vec<YieldCalculation6>>,
	#[serde(rename = "IntrstTp", skip_serializing_if = "Option::is_none")]
	pub intrst_tp: Option<InterestType3Code>,
	#[serde(rename = "InstrmStrTp", skip_serializing_if = "Option::is_none")]
	pub instrm_str_tp: Option<InstrumentSubStructureType2Choice>,
	#[serde(rename = "GblTp", skip_serializing_if = "Option::is_none")]
	pub gbl_tp: Option<GlobalNote2Choice>,
	#[serde(rename = "PotntlEuroSysElgblty", skip_serializing_if = "Option::is_none")]
	pub potntl_euro_sys_elgblty: Option<bool>,
	#[serde(rename = "Geogcs", skip_serializing_if = "Option::is_none")]
	pub geogcs: Option<Max35Text>,
	#[serde(rename = "YldRg", skip_serializing_if = "Option::is_none")]
	pub yld_rg: Option<AmountOrPercentageRange1>,
	#[serde(rename = "CpnRg", skip_serializing_if = "Option::is_none")]
	pub cpn_rg: Option<AmountOrPercentageRange1>,
	#[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
	pub purp: Option<Max256Text>,
	#[serde(rename = "AltrntvMinTaxInd", skip_serializing_if = "Option::is_none")]
	pub altrntv_min_tax_ind: Option<bool>,
	#[serde(rename = "AutoRinvstmt", skip_serializing_if = "Option::is_none")]
	pub auto_rinvstmt: Option<f64>,
	#[serde(rename = "Hrcut", skip_serializing_if = "Option::is_none")]
	pub hrcut: Option<f64>,
	#[serde(rename = "TxConds", skip_serializing_if = "Option::is_none")]
	pub tx_conds: Option<TradeTransactionCondition7Choice>,
	#[serde(rename = "LookBck", skip_serializing_if = "Option::is_none")]
	pub look_bck: Option<f64>,
	#[serde(rename = "MaxSbstitn", skip_serializing_if = "Option::is_none")]
	pub max_sbstitn: Option<f64>,
	#[serde(rename = "MinIncrmt", skip_serializing_if = "Option::is_none")]
	pub min_incrmt: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "MinQty", skip_serializing_if = "Option::is_none")]
	pub min_qty: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "Pdctn", skip_serializing_if = "Option::is_none")]
	pub pdctn: Option<Max35Text>,
	#[serde(rename = "RstrctdInd", skip_serializing_if = "Option::is_none")]
	pub rstrctd_ind: Option<bool>,
	#[serde(rename = "PricFrqcy", skip_serializing_if = "Option::is_none")]
	pub pric_frqcy: Option<Frequency35Choice>,
	#[serde(rename = "Sctr", skip_serializing_if = "Option::is_none")]
	pub sctr: Option<Max35Text>,
	#[serde(rename = "SbstitnFrqcy", skip_serializing_if = "Option::is_none")]
	pub sbstitn_frqcy: Option<Frequency35Choice>,
	#[serde(rename = "SbstitnLft", skip_serializing_if = "Option::is_none")]
	pub sbstitn_lft: Option<f64>,
	#[serde(rename = "WhlPoolInd", skip_serializing_if = "Option::is_none")]
	pub whl_pool_ind: Option<bool>,
	#[serde(rename = "PricSrc", skip_serializing_if = "Option::is_none")]
	pub pric_src: Option<Max35Text>,
	#[serde(rename = "PricRg", skip_serializing_if = "Option::is_none")]
	pub pric_rg: Option<AmountOrPercentageRange1>,
}


// DecimalNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "$value")]
	pub decimal_number: f64,
}


// Derivative4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Derivative4 {
	#[serde(rename = "Futr", skip_serializing_if = "Option::is_none")]
	pub futr: Option<Future4>,
	#[serde(rename = "Optn", skip_serializing_if = "Option::is_none")]
	pub optn: Option<Option15>,
}


// DistributionPolicy1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum DistributionPolicy1Code {
	#[default]
	#[serde(rename = "DIST")]
	CodeDIST,
	#[serde(rename = "ACCU")]
	CodeACCU,

}


// DistributionPolicy2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DistributionPolicy2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<DistributionPolicy1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// Equity3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Equity3 {
	#[serde(rename = "PrefToIncm")]
	pub pref_to_incm: PreferenceToIncome5Choice,
	#[serde(rename = "MtrtyDt", skip_serializing_if = "Option::is_none")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "NonPdAmt", skip_serializing_if = "Option::is_none")]
	pub non_pd_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "ParVal", skip_serializing_if = "Option::is_none")]
	pub par_val: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "VtngRghtsPerShr", skip_serializing_if = "Option::is_none")]
	pub vtng_rghts_per_shr: Option<f64>,
}


// Exact4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[serde(rename = "$value")]
	pub exact4_alpha_numeric_text: String,
}


// ExternalFinancialInstrumentIdentificationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalFinancialInstrumentIdentificationType1Code {
	#[serde(rename = "$value")]
	pub external_financial_instrument_identification_type1_code: String,
}


// ExternalFinancialInstrumentProductType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalFinancialInstrumentProductType1Code {
	#[serde(rename = "$value")]
	pub external_financial_instrument_product_type1_code: String,
}


// ExternalSecuritiesUpdateReason1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalSecuritiesUpdateReason1Code {
	#[serde(rename = "$value")]
	pub external_securities_update_reason1_code: String,
}


// FinancialInstrument97 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrument97 {
	#[serde(rename = "Eqty", skip_serializing_if = "Option::is_none")]
	pub eqty: Option<Equity3>,
	#[serde(rename = "Warrt", skip_serializing_if = "Option::is_none")]
	pub warrt: Option<Warrant4>,
	#[serde(rename = "Debt", skip_serializing_if = "Option::is_none")]
	pub debt: Option<Debt5>,
	#[serde(rename = "Deriv", skip_serializing_if = "Option::is_none")]
	pub deriv: Option<Derivative4>,
}


// FinancialInstrumentForm2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentForm2 {
	#[serde(rename = "BookgApprnc", skip_serializing_if = "Option::is_none")]
	pub bookg_apprnc: Option<Appearance3Choice>,
	#[serde(rename = "LglForm", skip_serializing_if = "Option::is_none")]
	pub lgl_form: Option<FormOfSecurity8Choice>,
}


// FinancialInstrumentQuantity1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FinancialInstrumentQuantity1Choice {
	#[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
	pub unit: Option<f64>,
	#[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
	pub face_amt: Option<f64>,
	#[serde(rename = "AmtsdVal", skip_serializing_if = "Option::is_none")]
	pub amtsd_val: Option<f64>,
}


// FormOfSecurity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum FormOfSecurity1Code {
	#[default]
	#[serde(rename = "BEAR")]
	CodeBEAR,
	#[serde(rename = "REGD")]
	CodeREGD,

}


// FormOfSecurity8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FormOfSecurity8Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<FormOfSecurity1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// Frequency35Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Frequency35Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Frequency5Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// Frequency5Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Frequency5Code {
	#[default]
	#[serde(rename = "YEAR")]
	CodeYEAR,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "QURT")]
	CodeQURT,
	#[serde(rename = "MIAN")]
	CodeMIAN,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "DAIL")]
	CodeDAIL,
	#[serde(rename = "ADHO")]
	CodeADHO,
	#[serde(rename = "INDA")]
	CodeINDA,
	#[serde(rename = "OVNG")]
	CodeOVNG,
	#[serde(rename = "TEND")]
	CodeTEND,

}


// Future4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Future4 {
	#[serde(rename = "CtrctSz", skip_serializing_if = "Option::is_none")]
	pub ctrct_sz: Option<f64>,
	#[serde(rename = "ExrcPric", skip_serializing_if = "Option::is_none")]
	pub exrc_pric: Option<Price8>,
	#[serde(rename = "FutrDt", skip_serializing_if = "Option::is_none")]
	pub futr_dt: Option<String>,
	#[serde(rename = "MinSz", skip_serializing_if = "Option::is_none")]
	pub min_sz: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none")]
	pub unit_of_measr: Option<UnitOfMeasure7Choice>,
	#[serde(rename = "TmUnit", skip_serializing_if = "Option::is_none")]
	pub tm_unit: Option<TimeUnit3Choice>,
	#[serde(rename = "AddtlUndrlygAttrbts", skip_serializing_if = "Option::is_none")]
	pub addtl_undrlyg_attrbts: Option<Vec<UnderlyingAttributes4>>,
}


// GenericIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}


// GenericIdentification13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification13 {
	#[serde(rename = "Id")]
	pub id: Max4AlphaNumericText,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
	#[serde(rename = "Issr")]
	pub issr: Max35Text,
}


// GenericIdentification30 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification30 {
	#[serde(rename = "Id")]
	pub id: Exact4AlphaNumericText,
	#[serde(rename = "Issr")]
	pub issr: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
}


// GenericIdentification36 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericIdentification36 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Issr")]
	pub issr: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<Max35Text>,
}


// GlobalNote1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum GlobalNote1Code {
	#[default]
	#[serde(rename = "NGNO")]
	CodeNGNO,
	#[serde(rename = "CGNO")]
	CodeCGNO,

}


// GlobalNote2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GlobalNote2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<GlobalNote1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// ISIN2021Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISIN2021Identifier {
	#[serde(rename = "$value")]
	pub isin2021_identifier: String,
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


// ISOYearMonth ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISOYearMonth {
	#[serde(rename = "$value")]
	pub iso_year_month: String,
}


// IdentificationSource3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct IdentificationSource3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalFinancialInstrumentIdentificationType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}


// ImpliedCurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ImpliedCurrencyAndAmount {
	#[serde(rename = "$value")]
	pub implied_currency_and_amount: f64,
}


// InitialPhysicalForm1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InitialPhysicalForm1Code {
	#[default]
	#[serde(rename = "GTGT")]
	CodeGTGT,
	#[serde(rename = "GPGP")]
	CodeGPGP,
	#[serde(rename = "DERN")]
	CodeDERN,

}


// InitialPhysicalForm2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InitialPhysicalForm2Code {
	#[default]
	#[serde(rename = "GPGP")]
	CodeGPGP,
	#[serde(rename = "DERN")]
	CodeDERN,

}


// InitialPhysicalForm3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InitialPhysicalForm3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InitialPhysicalForm2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// InitialPhysicalForm4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InitialPhysicalForm4Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InitialPhysicalForm1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// InstrumentSubStructureType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InstrumentSubStructureType1Code {
	#[default]
	#[serde(rename = "ABSE")]
	CodeABSE,
	#[serde(rename = "AIRT")]
	CodeAIRT,
	#[serde(rename = "AUTT")]
	CodeAUTT,
	#[serde(rename = "CBOB")]
	CodeCBOB,
	#[serde(rename = "CDOB")]
	CodeCDOB,
	#[serde(rename = "CLNO")]
	CodeCLNO,
	#[serde(rename = "CLOB")]
	CodeCLOB,
	#[serde(rename = "CMBS")]
	CodeCMBS,
	#[serde(rename = "CSMR")]
	CodeCSMR,
	#[serde(rename = "CRCT")]
	CodeCRCT,
	#[serde(rename = "HELO")]
	CodeHELO,
	#[serde(rename = "LPNO")]
	CodeLPNO,
	#[serde(rename = "PFAB")]
	CodePFAB,
	#[serde(rename = "PYRT")]
	CodePYRT,
	#[serde(rename = "REPK")]
	CodeREPK,
	#[serde(rename = "RMBS")]
	CodeRMBS,
	#[serde(rename = "SCBO")]
	CodeSCBO,
	#[serde(rename = "STRB")]
	CodeSTRB,
	#[serde(rename = "STUT")]
	CodeSTUT,
	#[serde(rename = "WBSE")]
	CodeWBSE,

}


// InstrumentSubStructureType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InstrumentSubStructureType2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InstrumentSubStructureType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// InterestType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InterestType3Code {
	#[default]
	#[serde(rename = "ZCPN")]
	CodeZCPN,
	#[serde(rename = "FIXD")]
	CodeFIXD,
	#[serde(rename = "FLRN")]
	CodeFLRN,
	#[serde(rename = "DUAL")]
	CodeDUAL,
	#[serde(rename = "INDE")]
	CodeINDE,
	#[serde(rename = "DSCO")]
	CodeDSCO,

}


// InvestorRestrictionType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InvestorRestrictionType1Code {
	#[default]
	#[serde(rename = "LERE")]
	CodeLERE,
	#[serde(rename = "CITI")]
	CodeCITI,
	#[serde(rename = "INDV")]
	CodeINDV,

}


// InvestorRestrictionType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestorRestrictionType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InvestorRestrictionType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// InvestorType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InvestorType1Code {
	#[default]
	#[serde(rename = "RETL")]
	CodeRETL,
	#[serde(rename = "PROF")]
	CodePROF,
	#[serde(rename = "STAF")]
	CodeSTAF,
	#[serde(rename = "PPER")]
	CodePPER,

}


// InvestorType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvestorType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InvestorType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// Issuance5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Issuance5 {
	#[serde(rename = "IssePlc", skip_serializing_if = "Option::is_none")]
	pub isse_plc: Option<MICIdentifier>,
	#[serde(rename = "CtryOfIsse", skip_serializing_if = "Option::is_none")]
	pub ctry_of_isse: Option<CountryCode>,
	#[serde(rename = "IsseDt", skip_serializing_if = "Option::is_none")]
	pub isse_dt: Option<String>,
	#[serde(rename = "AnncmntDt", skip_serializing_if = "Option::is_none")]
	pub anncmnt_dt: Option<String>,
	#[serde(rename = "ISINVldFr", skip_serializing_if = "Option::is_none")]
	pub isin_vld_fr: Option<String>,
	#[serde(rename = "IssrOrg", skip_serializing_if = "Option::is_none")]
	pub issr_org: Option<Organisation38>,
	#[serde(rename = "IsseNmnlAmt", skip_serializing_if = "Option::is_none")]
	pub isse_nmnl_amt: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "FullIssdAmt", skip_serializing_if = "Option::is_none")]
	pub full_issd_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "IsseSz", skip_serializing_if = "Option::is_none")]
	pub isse_sz: Option<f64>,
	#[serde(rename = "IssePric", skip_serializing_if = "Option::is_none")]
	pub isse_pric: Option<PriceValue1>,
	#[serde(rename = "IssncDstrbtn", skip_serializing_if = "Option::is_none")]
	pub issnc_dstrbtn: Option<SecuritiesTransactionType31Choice>,
	#[serde(rename = "GovngLaw", skip_serializing_if = "Option::is_none")]
	pub govng_law: Option<Vec<Jurisdiction1>>,
}


// Jurisdiction1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Jurisdiction1 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max70Text>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
}


// LegalRestrictions1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum LegalRestrictions1Code {
	#[default]
	#[serde(rename = "USLE")]
	CodeUSLE,
	#[serde(rename = "NORE")]
	CodeNORE,
	#[serde(rename = "REST")]
	CodeREST,

}


// LegalRestrictions2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum LegalRestrictions2Code {
	#[default]
	#[serde(rename = "JURO")]
	CodeJURO,
	#[serde(rename = "PPLA")]
	CodePPLA,
	#[serde(rename = "ACRI")]
	CodeACRI,
	#[serde(rename = "MARG")]
	CodeMARG,
	#[serde(rename = "PRIV")]
	CodePRIV,

}


// LegalRestrictions4Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LegalRestrictions4Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<LegalRestrictions1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// LegalRestrictions5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LegalRestrictions5Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<LegalRestrictions2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// MICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MICIdentifier {
	#[serde(rename = "$value")]
	pub mic_identifier: String,
}


// MaturityRedemptionType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum MaturityRedemptionType1Code {
	#[default]
	#[serde(rename = "FRED")]
	CodeFRED,
	#[serde(rename = "PRNR")]
	CodePRNR,
	#[serde(rename = "PRWR")]
	CodePRWR,
	#[serde(rename = "RNDM")]
	CodeRNDM,
	#[serde(rename = "PRRA")]
	CodePRRA,
	#[serde(rename = "CALL")]
	CodeCALL,
	#[serde(rename = "PUUT")]
	CodePUUT,

}


// MaturityRedemptionType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MaturityRedemptionType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<MaturityRedemptionType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
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


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max16Text {
	#[serde(rename = "$value")]
	pub max16_text: String,
}


// Max256Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max256Text {
	#[serde(rename = "$value")]
	pub max256_text: String,
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


// Max3NumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max3NumericText {
	#[serde(rename = "$value")]
	pub max3_numeric_text: String,
}


// Max4AlphaNumericText ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "$value")]
	pub max4_alpha_numeric_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
}


// MessageHeader1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageHeader1 {
	#[serde(rename = "MsgId")]
	pub msg_id: Max35Text,
	#[serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none")]
	pub cre_dt_tm: Option<String>,
}


// NameAndAddress4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress4 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max350Text>,
	#[serde(rename = "Adr")]
	pub adr: PostalAddress1,
}


// NameAndAddress5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: Max350Text,
	#[serde(rename = "Adr", skip_serializing_if = "Option::is_none")]
	pub adr: Option<PostalAddress1>,
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "$value")]
	pub number: f64,
}


// Operation1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Operation1Code {
	#[default]
	#[serde(rename = "TILL")]
	CodeTILL,
	#[serde(rename = "ORRR")]
	CodeORRR,
	#[serde(rename = "ANDD")]
	CodeANDD,

}


// Operator1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Operator1Code {
	#[default]
	#[serde(rename = "SMAL")]
	CodeSMAL,
	#[serde(rename = "SMEQ")]
	CodeSMEQ,
	#[serde(rename = "GREA")]
	CodeGREA,
	#[serde(rename = "GREQ")]
	CodeGREQ,
	#[serde(rename = "EQAL")]
	CodeEQAL,

}


// Option15 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Option15 {
	#[serde(rename = "OptnSttlmStyle", skip_serializing_if = "Option::is_none")]
	pub optn_sttlm_style: Option<SettleStyle2Choice>,
	#[serde(rename = "ConvsDt", skip_serializing_if = "Option::is_none")]
	pub convs_dt: Option<String>,
	#[serde(rename = "StrkPric", skip_serializing_if = "Option::is_none")]
	pub strk_pric: Option<Price8>,
	#[serde(rename = "MinExrcblQty", skip_serializing_if = "Option::is_none")]
	pub min_exrcbl_qty: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "ConvsPrd", skip_serializing_if = "Option::is_none")]
	pub convs_prd: Option<DateTimePeriod1Choice>,
	#[serde(rename = "OptnStyle", skip_serializing_if = "Option::is_none")]
	pub optn_style: Option<OptionStyle1Choice>,
	#[serde(rename = "OptnTp", skip_serializing_if = "Option::is_none")]
	pub optn_tp: Option<OptionType8Choice>,
	#[serde(rename = "StrkVal", skip_serializing_if = "Option::is_none")]
	pub strk_val: Option<f64>,
	#[serde(rename = "StrkMltplr", skip_serializing_if = "Option::is_none")]
	pub strk_mltplr: Option<f64>,
	#[serde(rename = "InstrmAssgnmtMtd", skip_serializing_if = "Option::is_none")]
	pub instrm_assgnmt_mtd: Option<AssignmentMethod2Choice>,
	#[serde(rename = "VrsnNb", skip_serializing_if = "Option::is_none")]
	pub vrsn_nb: Option<f64>,
	#[serde(rename = "XpryLctn", skip_serializing_if = "Option::is_none")]
	pub xpry_lctn: Option<Max4AlphaNumericText>,
	#[serde(rename = "Stdstn", skip_serializing_if = "Option::is_none")]
	pub stdstn: Option<Standardisation3Choice>,
	#[serde(rename = "TradgPtyRole", skip_serializing_if = "Option::is_none")]
	pub tradg_pty_role: Option<OptionParty3Choice>,
	#[serde(rename = "CtrctSz", skip_serializing_if = "Option::is_none")]
	pub ctrct_sz: Option<f64>,
	#[serde(rename = "AddtlUndrlygAttrbts", skip_serializing_if = "Option::is_none")]
	pub addtl_undrlyg_attrbts: Option<Vec<UnderlyingAttributes4>>,
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


// OptionParty3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OptionParty3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Vec<OptionParty1Code>>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// OptionStyle1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OptionStyle1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<OptionStyle1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification13>,
}


// OptionStyle1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OptionStyle1Code {
	#[default]
	#[serde(rename = "AMER")]
	CodeAMER,
	#[serde(rename = "EURO")]
	CodeEURO,
	#[serde(rename = "BERM")]
	CodeBERM,
	#[serde(rename = "ASIA")]
	CodeASIA,
	#[serde(rename = "CANA")]
	CodeCANA,

}


// OptionType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum OptionType1Code {
	#[default]
	#[serde(rename = "CALL")]
	CodeCALL,
	#[serde(rename = "PUTO")]
	CodePUTO,

}


// OptionType8Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OptionType8Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Vec<OptionType1Code>>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// Organisation38 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Organisation38 {
	#[serde(rename = "Nm")]
	pub nm: Max140Text,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<PartyIdentification177Choice>,
	#[serde(rename = "Purp", skip_serializing_if = "Option::is_none")]
	pub purp: Option<Max35Text>,
	#[serde(rename = "TaxtnCtry", skip_serializing_if = "Option::is_none")]
	pub taxtn_ctry: Option<CountryCode>,
	#[serde(rename = "RegnCtry", skip_serializing_if = "Option::is_none")]
	pub regn_ctry: Option<CountryCode>,
	#[serde(rename = "RegnDt", skip_serializing_if = "Option::is_none")]
	pub regn_dt: Option<String>,
	#[serde(rename = "TaxIdNb", skip_serializing_if = "Option::is_none")]
	pub tax_id_nb: Option<Max35Text>,
	#[serde(rename = "NtlRegnNb", skip_serializing_if = "Option::is_none")]
	pub ntl_regn_nb: Option<Max35Text>,
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Vec<PostalAddress3>,
	#[serde(rename = "PmryComAdr", skip_serializing_if = "Option::is_none")]
	pub pmry_com_adr: Option<CommunicationAddress3>,
	#[serde(rename = "ScndryComAdr", skip_serializing_if = "Option::is_none")]
	pub scndry_com_adr: Option<CommunicationAddress3>,
}


// OtherIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OtherIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Sfx", skip_serializing_if = "Option::is_none")]
	pub sfx: Option<Max16Text>,
	#[serde(rename = "Tp")]
	pub tp: IdentificationSource3Choice,
}


// PartyIdentification120Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification120Choice {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification36>,
	#[serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none")]
	pub nm_and_adr: Option<NameAndAddress5>,
}


// PartyIdentification136 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification136 {
	#[serde(rename = "Id")]
	pub id: PartyIdentification120Choice,
	#[serde(rename = "LEI", skip_serializing_if = "Option::is_none")]
	pub lei: Option<LEIIdentifier>,
}


// PartyIdentification177Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification177Choice {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICDec2014Identifier>,
	#[serde(rename = "PrtryId", skip_serializing_if = "Option::is_none")]
	pub prtry_id: Option<GenericIdentification1>,
}


// PaymentDirectionIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PaymentDirectionIndicator {
	#[serde(rename = "$value")]
	pub payment_direction_indicator: bool,
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "$value")]
	pub percentage_rate: f64,
}


// PhoneNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PhoneNumber {
	#[serde(rename = "$value")]
	pub phone_number: String,
}


// PostalAddress1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress1 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<AddressType2Code>,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
	pub adr_line: Option<Vec<Max70Text>>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<Max70Text>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<Max16Text>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<Max16Text>,
	#[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
	pub twn_nm: Option<Max35Text>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<Max35Text>,
	#[serde(rename = "Ctry")]
	pub ctry: CountryCode,
}


// PostalAddress3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress3 {
	#[serde(rename = "AdrTp")]
	pub adr_tp: AddressType1Code,
	#[serde(rename = "MlngInd")]
	pub mlng_ind: bool,
	#[serde(rename = "RegnAdrInd")]
	pub regn_adr_ind: bool,
	#[serde(rename = "NmAndAdr")]
	pub nm_and_adr: NameAndAddress4,
}


// PreferenceToIncome1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PreferenceToIncome1Code {
	#[default]
	#[serde(rename = "ORDN")]
	CodeORDN,
	#[serde(rename = "PFRD")]
	CodePFRD,

}


// PreferenceToIncome5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PreferenceToIncome5Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PreferenceToIncome1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// Price8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Price8 {
	#[serde(rename = "ValTp", skip_serializing_if = "Option::is_none")]
	pub val_tp: Option<PriceValueType3Code>,
	#[serde(rename = "Val")]
	pub val: PriceRateOrAmount3Choice,
	#[serde(rename = "PricTp", skip_serializing_if = "Option::is_none")]
	pub pric_tp: Option<TypeOfPrice1Code>,
}


// PriceRateOrAmount3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceRateOrAmount3Choice {
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
}


// PriceValue1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PriceValue1 {
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAnd13DecimalAmount,
}


// PriceValueType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PriceValueType3Code {
	#[default]
	#[serde(rename = "DISC")]
	CodeDISC,
	#[serde(rename = "PREM")]
	CodePREM,
	#[serde(rename = "PARV")]
	CodePARV,
	#[serde(rename = "YIEL")]
	CodeYIEL,
	#[serde(rename = "SPRE")]
	CodeSPRE,
	#[serde(rename = "PEUN")]
	CodePEUN,
	#[serde(rename = "ABSO")]
	CodeABSO,
	#[serde(rename = "TEDP")]
	CodeTEDP,
	#[serde(rename = "TEDY")]
	CodeTEDY,
	#[serde(rename = "FICT")]
	CodeFICT,
	#[serde(rename = "VACT")]
	CodeVACT,

}


// PutType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum PutType1Code {
	#[default]
	#[serde(rename = "MAND")]
	CodeMAND,
	#[serde(rename = "OPTI")]
	CodeOPTI,
	#[serde(rename = "TWOS")]
	CodeTWOS,

}


// PutType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PutType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<PutType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// RateAndAmountFormat1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RateAndAmountFormat1Choice {
	#[serde(rename = "Rate", skip_serializing_if = "Option::is_none")]
	pub rate: Option<f64>,
	#[serde(rename = "Amt", skip_serializing_if = "Option::is_none")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "NotSpcfdRate", skip_serializing_if = "Option::is_none")]
	pub not_spcfd_rate: Option<RateType12FormatChoice>,
}


// RateOrAbsoluteValue1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RateOrAbsoluteValue1Choice {
	#[serde(rename = "RateVal", skip_serializing_if = "Option::is_none")]
	pub rate_val: Option<f64>,
	#[serde(rename = "AbsVal", skip_serializing_if = "Option::is_none")]
	pub abs_val: Option<f64>,
}


// RateType12Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RateType12Code {
	#[default]
	#[serde(rename = "OPEN")]
	CodeOPEN,
	#[serde(rename = "UKWN")]
	CodeUKWN,
	#[serde(rename = "NILP")]
	CodeNILP,

}


// RateType12FormatChoice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RateType12FormatChoice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<RateType12Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification13>,
}


// RestrictionType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum RestrictionType1Code {
	#[default]
	#[serde(rename = "SELR")]
	CodeSELR,
	#[serde(rename = "BUYR")]
	CodeBUYR,
	#[serde(rename = "PLAR")]
	CodePLAR,
	#[serde(rename = "HOLR")]
	CodeHOLR,
	#[serde(rename = "VOTR")]
	CodeVOTR,

}


// SecuritiesPaymentStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SecuritiesPaymentStatus1Code {
	#[default]
	#[serde(rename = "FULL")]
	CodeFULL,
	#[serde(rename = "NILL")]
	CodeNILL,
	#[serde(rename = "PART")]
	CodePART,

}


// SecuritiesPaymentStatus5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesPaymentStatus5Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<SecuritiesPaymentStatus1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// SecuritiesTransactionType11Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SecuritiesTransactionType11Code {
	#[default]
	#[serde(rename = "NSYN")]
	CodeNSYN,
	#[serde(rename = "SYND")]
	CodeSYND,

}


// SecuritiesTransactionType31Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesTransactionType31Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<SecuritiesTransactionType11Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// SecuritiesUpdateReason1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesUpdateReason1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalSecuritiesUpdateReason1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// SecurityAttributes12 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityAttributes12 {
	#[serde(rename = "FinInstrmTp", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_tp: Option<Vec<FinancialInstrument97>>,
	#[serde(rename = "FinInstrmAttrbts", skip_serializing_if = "Option::is_none")]
	pub fin_instrm_attrbts: Option<Vec<CommonFinancialInstrumentAttributes12>>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// SecurityIdentification39 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityIdentification39 {
	#[serde(rename = "ISIN", skip_serializing_if = "Option::is_none")]
	pub isin: Option<ISIN2021Identifier>,
	#[serde(rename = "OthrId", skip_serializing_if = "Option::is_none")]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max140Text>,
}


// SecurityMaintenanceRequestV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityMaintenanceRequestV01 {
	#[serde(rename = "MsgHdr", skip_serializing_if = "Option::is_none")]
	pub msg_hdr: Option<MessageHeader1>,
	#[serde(rename = "UpdTp")]
	pub upd_tp: UpdateType36Choice,
	#[serde(rename = "UpdRsn", skip_serializing_if = "Option::is_none")]
	pub upd_rsn: Option<SecuritiesUpdateReason1Choice>,
	#[serde(rename = "FinInstrmId")]
	pub fin_instrm_id: SecurityIdentification39,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// SecurityRestriction3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityRestriction3 {
	#[serde(rename = "FctvPrd", skip_serializing_if = "Option::is_none")]
	pub fctv_prd: Option<DateTimePeriod2>,
	#[serde(rename = "RstrctnTp", skip_serializing_if = "Option::is_none")]
	pub rstrctn_tp: Option<SecurityRestrictionType2Choice>,
	#[serde(rename = "LglRstrctnTp", skip_serializing_if = "Option::is_none")]
	pub lgl_rstrctn_tp: Option<LegalRestrictions5Choice>,
	#[serde(rename = "InvstrRstrctnTp", skip_serializing_if = "Option::is_none")]
	pub invstr_rstrctn_tp: Option<Vec<InvestorRestrictionType3Choice>>,
	#[serde(rename = "InvstrTp", skip_serializing_if = "Option::is_none")]
	pub invstr_tp: Option<Vec<InvestorType3Choice>>,
}


// SecurityRestrictionType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityRestrictionType2Choice {
	#[serde(rename = "RstrctnTp", skip_serializing_if = "Option::is_none")]
	pub rstrctn_tp: Option<RestrictionType1Code>,
	#[serde(rename = "PrtryRstrctn", skip_serializing_if = "Option::is_none")]
	pub prtry_rstrctn: Option<GenericIdentification30>,
}


// SecurityStatus2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SecurityStatus2Code {
	#[default]
	#[serde(rename = "ACTV")]
	CodeACTV,
	#[serde(rename = "INAC")]
	CodeINAC,
	#[serde(rename = "SUSP")]
	CodeSUSP,

}


// SecurityStatus3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityStatus3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<SecurityStatus2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// SecurityWithHoldingTax1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecurityWithHoldingTax1 {
	#[serde(rename = "WhldgTaxVal")]
	pub whldg_tax_val: RateAndAmountFormat1Choice,
	#[serde(rename = "Ctry")]
	pub ctry: CountryCode,
}


// SettleStyle1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SettleStyle1Code {
	#[default]
	#[serde(rename = "SETC")]
	CodeSETC,
	#[serde(rename = "SETO")]
	CodeSETO,

}


// SettleStyle2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettleStyle2Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Vec<SettleStyle1Code>>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// SettlementInformation17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementInformation17 {
	#[serde(rename = "SctiesQtyTp", skip_serializing_if = "Option::is_none")]
	pub scties_qty_tp: Option<SettlementUnitType3Choice>,
	#[serde(rename = "CtrctSttlmMnth", skip_serializing_if = "Option::is_none")]
	pub ctrct_sttlm_mnth: Option<String>,
	#[serde(rename = "MinDnmtn", skip_serializing_if = "Option::is_none")]
	pub min_dnmtn: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "MinMltplQty", skip_serializing_if = "Option::is_none")]
	pub min_mltpl_qty: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "DevtgSttlmUnit", skip_serializing_if = "Option::is_none")]
	pub devtg_sttlm_unit: Option<Vec<FinancialInstrumentQuantity1Choice>>,
}


// SettlementType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SettlementType1Code {
	#[default]
	#[serde(rename = "PRIN")]
	CodePRIN,
	#[serde(rename = "NETO")]
	CodeNETO,

}


// SettlementType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<SettlementType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// SettlementUnitType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum SettlementUnitType1Code {
	#[default]
	#[serde(rename = "FAMT")]
	CodeFAMT,
	#[serde(rename = "UNIT")]
	CodeUNIT,

}


// SettlementUnitType3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementUnitType3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<SettlementUnitType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// Standardisation1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Standardisation1Code {
	#[default]
	#[serde(rename = "FLEX")]
	CodeFLEX,
	#[serde(rename = "NSTA")]
	CodeNSTA,
	#[serde(rename = "STAN")]
	CodeSTAN,

}


// Standardisation3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Standardisation3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<Vec<Standardisation1Code>>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
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


// TEFRARules1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TEFRARules1Code {
	#[default]
	#[serde(rename = "RULC")]
	CodeRULC,
	#[serde(rename = "RULD")]
	CodeRULD,

}


// TEFRARules3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TEFRARules3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<TEFRARules1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// Term1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Term1 {
	#[serde(rename = "Oprtr")]
	pub oprtr: Operator1Code,
	#[serde(rename = "Val")]
	pub val: RateOrAbsoluteValue1Choice,
}


// TimeUnit1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TimeUnit1Code {
	#[default]
	#[serde(rename = "DAYC")]
	CodeDAYC,
	#[serde(rename = "HOUR")]
	CodeHOUR,
	#[serde(rename = "MINU")]
	CodeMINU,
	#[serde(rename = "MNTH")]
	CodeMNTH,
	#[serde(rename = "SECO")]
	CodeSECO,
	#[serde(rename = "WEEK")]
	CodeWEEK,
	#[serde(rename = "YEAR")]
	CodeYEAR,

}


// TimeUnit3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TimeUnit3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<TimeUnit1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// TradeTransactionCondition2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TradeTransactionCondition2Code {
	#[default]
	#[serde(rename = "SPCC")]
	CodeSPCC,
	#[serde(rename = "SECN")]
	CodeSECN,
	#[serde(rename = "SEBN")]
	CodeSEBN,
	#[serde(rename = "SCBN")]
	CodeSCBN,
	#[serde(rename = "SCRT")]
	CodeSCRT,
	#[serde(rename = "SERT")]
	CodeSERT,
	#[serde(rename = "SCCR")]
	CodeSCCR,
	#[serde(rename = "SECR")]
	CodeSECR,
	#[serde(rename = "CAST")]
	CodeCAST,
	#[serde(rename = "SPPR")]
	CodeSPPR,
	#[serde(rename = "SPCU")]
	CodeSPCU,
	#[serde(rename = "SPEX")]
	CodeSPEX,
	#[serde(rename = "GTDL")]
	CodeGTDL,

}


// TradeTransactionCondition7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeTransactionCondition7Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<TradeTransactionCondition2Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// TradingParameters2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradingParameters2 {
	#[serde(rename = "MktId", skip_serializing_if = "Option::is_none")]
	pub mkt_id: Option<MICIdentifier>,
	#[serde(rename = "RndLot", skip_serializing_if = "Option::is_none")]
	pub rnd_lot: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "TradLotSz", skip_serializing_if = "Option::is_none")]
	pub trad_lot_sz: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "ScndryPlcOfListg", skip_serializing_if = "Option::is_none")]
	pub scndry_plc_of_listg: Option<Vec<MICIdentifier>>,
	#[serde(rename = "MinTraddNmnlQty", skip_serializing_if = "Option::is_none")]
	pub min_tradd_nmnl_qty: Option<UnitOrFaceAmount1Choice>,
	#[serde(rename = "MaxTraddNmnlQty", skip_serializing_if = "Option::is_none")]
	pub max_tradd_nmnl_qty: Option<UnitOrFaceAmount1Choice>,
	#[serde(rename = "MinTradgPricgIncrmt", skip_serializing_if = "Option::is_none")]
	pub min_tradg_pricg_incrmt: Option<f64>,
	#[serde(rename = "PmryPlcOfListgId", skip_serializing_if = "Option::is_none")]
	pub pmry_plc_of_listg_id: Option<MICIdentifier>,
}


// TypeOfPrice1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum TypeOfPrice1Code {
	#[default]
	#[serde(rename = "AVER")]
	CodeAVER,
	#[serde(rename = "AVOV")]
	CodeAVOV,
	#[serde(rename = "COMB")]
	CodeCOMB,
	#[serde(rename = "GREX")]
	CodeGREX,
	#[serde(rename = "LIMI")]
	CodeLIMI,
	#[serde(rename = "NET2")]
	CodeNET2,
	#[serde(rename = "NDIS")]
	CodeNDIS,
	#[serde(rename = "NET1")]
	CodeNET1,
	#[serde(rename = "NUND")]
	CodeNUND,
	#[serde(rename = "NOGR")]
	CodeNOGR,
	#[serde(rename = "PARV")]
	CodePARV,
	#[serde(rename = "RDAV")]
	CodeRDAV,
	#[serde(rename = "STOP")]
	CodeSTOP,

}


// UnderlyingAttributes4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnderlyingAttributes4 {
	#[serde(rename = "AllcnPctg", skip_serializing_if = "Option::is_none")]
	pub allcn_pctg: Option<f64>,
	#[serde(rename = "Qty", skip_serializing_if = "Option::is_none")]
	pub qty: Option<UnitOrFaceAmount1Choice>,
	#[serde(rename = "SttlmTp", skip_serializing_if = "Option::is_none")]
	pub sttlm_tp: Option<SettlementType3Choice>,
	#[serde(rename = "CshAmt", skip_serializing_if = "Option::is_none")]
	pub csh_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "CshTp", skip_serializing_if = "Option::is_none")]
	pub csh_tp: Option<Max35Text>,
	#[serde(rename = "Pric", skip_serializing_if = "Option::is_none")]
	pub pric: Option<Price8>,
	#[serde(rename = "DrtyPric", skip_serializing_if = "Option::is_none")]
	pub drty_pric: Option<Price8>,
	#[serde(rename = "EndPric", skip_serializing_if = "Option::is_none")]
	pub end_pric: Option<Price8>,
	#[serde(rename = "StartVal", skip_serializing_if = "Option::is_none")]
	pub start_val: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "CurVal", skip_serializing_if = "Option::is_none")]
	pub cur_val: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "EndVal", skip_serializing_if = "Option::is_none")]
	pub end_val: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "AdjstdQty", skip_serializing_if = "Option::is_none")]
	pub adjstd_qty: Option<UnitOrFaceAmount1Choice>,
	#[serde(rename = "XchgRate", skip_serializing_if = "Option::is_none")]
	pub xchg_rate: Option<f64>,
	#[serde(rename = "CapVal", skip_serializing_if = "Option::is_none")]
	pub cap_val: Option<ActiveCurrencyAndAmount>,
}


// UnitOfMeasure7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitOfMeasure7Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<UnitOfMeasure9Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// UnitOfMeasure9Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum UnitOfMeasure9Code {
	#[default]
	#[serde(rename = "BAGG")]
	CodeBAGG,
	#[serde(rename = "BALE")]
	CodeBALE,
	#[serde(rename = "BOTL")]
	CodeBOTL,
	#[serde(rename = "BOXX")]
	CodeBOXX,
	#[serde(rename = "CRTN")]
	CodeCRTN,
	#[serde(rename = "CELI")]
	CodeCELI,
	#[serde(rename = "CMET")]
	CodeCMET,
	#[serde(rename = "CNTR")]
	CodeCNTR,
	#[serde(rename = "CRAT")]
	CodeCRAT,
	#[serde(rename = "CBIN")]
	CodeCBIN,
	#[serde(rename = "CBME")]
	CodeCBME,
	#[serde(rename = "CBML")]
	CodeCBML,
	#[serde(rename = "PIEC")]
	CodePIEC,
	#[serde(rename = "FOOT")]
	CodeFOOT,
	#[serde(rename = "GBFO")]
	CodeGBFO,
	#[serde(rename = "GBGA")]
	CodeGBGA,
	#[serde(rename = "GBPI")]
	CodeGBPI,
	#[serde(rename = "GBQA")]
	CodeGBQA,
	#[serde(rename = "GBTN")]
	CodeGBTN,
	#[serde(rename = "GRAM")]
	CodeGRAM,
	#[serde(rename = "INCH")]
	CodeINCH,
	#[serde(rename = "KILO")]
	CodeKILO,
	#[serde(rename = "KMET")]
	CodeKMET,
	#[serde(rename = "LITR")]
	CodeLITR,
	#[serde(rename = "METR")]
	CodeMETR,
	#[serde(rename = "TONE")]
	CodeTONE,
	#[serde(rename = "MILE")]
	CodeMILE,
	#[serde(rename = "MMET")]
	CodeMMET,
	#[serde(rename = "MILI")]
	CodeMILI,
	#[serde(rename = "PUND")]
	CodePUND,
	#[serde(rename = "USOU")]
	CodeUSOU,
	#[serde(rename = "SCMT")]
	CodeSCMT,
	#[serde(rename = "SQFO")]
	CodeSQFO,
	#[serde(rename = "SQIN")]
	CodeSQIN,
	#[serde(rename = "SQKI")]
	CodeSQKI,
	#[serde(rename = "SMET")]
	CodeSMET,
	#[serde(rename = "SQMI")]
	CodeSQMI,
	#[serde(rename = "SMIL")]
	CodeSMIL,
	#[serde(rename = "SQYA")]
	CodeSQYA,
	#[serde(rename = "USBA")]
	CodeUSBA,
	#[serde(rename = "USFO")]
	CodeUSFO,
	#[serde(rename = "USGA")]
	CodeUSGA,
	#[serde(rename = "USPI")]
	CodeUSPI,
	#[serde(rename = "USQA")]
	CodeUSQA,
	#[serde(rename = "USTN")]
	CodeUSTN,
	#[serde(rename = "YARD")]
	CodeYARD,
	#[serde(rename = "GBOU")]
	CodeGBOU,
	#[serde(rename = "ACRE")]
	CodeACRE,
	#[serde(rename = "ARES")]
	CodeARES,
	#[serde(rename = "HECT")]
	CodeHECT,

}


// UnitOrFaceAmount1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UnitOrFaceAmount1Choice {
	#[serde(rename = "Unit", skip_serializing_if = "Option::is_none")]
	pub unit: Option<f64>,
	#[serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none")]
	pub face_amt: Option<ActiveCurrencyAndAmount>,
}


// UpdateType35Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateType35Choice {
	#[serde(rename = "Add", skip_serializing_if = "Option::is_none")]
	pub add: Option<SecurityAttributes12>,
	#[serde(rename = "Del", skip_serializing_if = "Option::is_none")]
	pub del: Option<SecurityAttributes12>,
	#[serde(rename = "Modfy", skip_serializing_if = "Option::is_none")]
	pub modfy: Option<SecurityAttributes12>,
}


// UpdateType36Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UpdateType36Choice {
	#[serde(rename = "UpdTp", skip_serializing_if = "Option::is_none")]
	pub upd_tp: Option<Vec<UpdateType35Choice>>,
	#[serde(rename = "Rplc", skip_serializing_if = "Option::is_none")]
	pub rplc: Option<SecurityAttributes12>,
}


// Warrant4 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Warrant4 {
	#[serde(rename = "Mltplr", skip_serializing_if = "Option::is_none")]
	pub mltplr: Option<f64>,
	#[serde(rename = "SbcptPric", skip_serializing_if = "Option::is_none")]
	pub sbcpt_pric: Option<Price8>,
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<WarrantStyle3Choice>,
	#[serde(rename = "WarrtAgt", skip_serializing_if = "Option::is_none")]
	pub warrt_agt: Option<Vec<Organisation38>>,
}


// WarrantStyle1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum WarrantStyle1Code {
	#[default]
	#[serde(rename = "AMER")]
	CodeAMER,
	#[serde(rename = "EURO")]
	CodeEURO,
	#[serde(rename = "BERM")]
	CodeBERM,

}


// WarrantStyle3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct WarrantStyle3Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<WarrantStyle1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<GenericIdentification30>,
}


// YesNoIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "$value")]
	pub yes_no_indicator: bool,
}


// YieldCalculation6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct YieldCalculation6 {
	#[serde(rename = "Val")]
	pub val: f64,
	#[serde(rename = "ClctnTp", skip_serializing_if = "Option::is_none")]
	pub clctn_tp: Option<CalculationType3Choice>,
	#[serde(rename = "RedPric", skip_serializing_if = "Option::is_none")]
	pub red_pric: Option<Price8>,
	#[serde(rename = "ValDt")]
	pub val_dt: String,
	#[serde(rename = "ValPrd")]
	pub val_prd: DateTimePeriod1Choice,
	#[serde(rename = "ClctnDt")]
	pub clctn_dt: String,
}
