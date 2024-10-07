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


// ActiveCurrencyAndAmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmountSimpleType {
	#[serde(rename = "ActiveCurrencyAndAmount_SimpleType")]
	pub active_currency_and_amount_simple_type: f64,
}


// ActiveCurrencyAndAmount ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyAndAmount {
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


// ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType {
	#[serde(rename = "ActiveOrHistoricCurrencyAnd13DecimalAmount_SimpleType")]
	pub active_or_historic_currency_and13_decimal_amount_simple_type: f64,
}


// ActiveOrHistoricCurrencyAnd13DecimalAmount ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmount {
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


// AddressType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AddressType1Code {
	#[validate(enumerate = ["HOME", "BIZZ"])]
	#[serde(rename = "AddressType1Code")]
	pub address_type1_code: String,
}


// AddressType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AddressType2Code {
	#[validate(enumerate = ["ADDR", "PBOX", "HOME", "BIZZ", "MLTO", "DLVY"])]
	#[serde(rename = "AddressType2Code")]
	pub address_type2_code: String,
}


// AmountOrPercentageRange1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AmountOrPercentageRange1 {
	#[serde(rename = "Opr")]
	pub opr: Option<String>,
	#[validate]
	#[serde(rename = "Term")]
	pub term: Option<Vec<Term1>>,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[validate(pattern = "[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}")]
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// Appearance1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Appearance1Code {
	#[validate(enumerate = ["DELI", "NDEL", "LIMI", "BENT", "DFBE", "DLBE", "TMPG", "GLOB"])]
	#[serde(rename = "Appearance1Code")]
	pub appearance1_code: String,
}


// Appearance3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Appearance3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// AssignmentMethod1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssignmentMethod1Code {
	#[validate(enumerate = ["RAND", "PROR"])]
	#[serde(rename = "AssignmentMethod1Code")]
	pub assignment_method1_code: String,
}


// AssignmentMethod2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AssignmentMethod2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// BaseOneRate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BaseOneRate {
	#[serde(rename = "BaseOneRate")]
	pub base_one_rate: f64,
}


// BenchmarkCurve6 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BenchmarkCurve6 {
	#[serde(rename = "Sprd")]
	pub sprd: Option<f64>,
	#[validate]
	#[serde(rename = "BchmkId")]
	pub bchmk_id: Option<SecurityIdentification39>,
	#[validate]
	#[serde(rename = "BchmkPric")]
	pub bchmk_pric: Option<Price8>,
	#[serde(rename = "BchmkCrvCcy")]
	pub bchmk_crv_ccy: Option<String>,
	#[validate]
	#[serde(rename = "BchmkCrvNm")]
	pub bchmk_crv_nm: Option<BenchmarkCurveName7Choice>,
	#[serde(rename = "BchmkCrvPt")]
	pub bchmk_crv_pt: Option<String>,
}


// BenchmarkCurveName1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BenchmarkCurveName1Code {
	#[validate(enumerate = ["MAAA", "FUSW", "LIBI", "LIBO", "SWAP", "TREA", "EURI", "PFAN"])]
	#[serde(rename = "BenchmarkCurveName1Code")]
	pub benchmark_curve_name1_code: String,
}


// BenchmarkCurveName7Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BenchmarkCurveName7Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// BusinessError4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BusinessError4 {
	#[validate]
	#[serde(rename = "FinInstrmId")]
	pub fin_instrm_id: SecurityIdentification39,
	#[validate]
	#[serde(rename = "BizErr")]
	pub biz_err: Vec<ErrorHandling5>,
}


// CFIOct2015Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CFIOct2015Identifier {
	#[validate(pattern = "[A-Z]{6,6}")]
	#[serde(rename = "CFIOct2015Identifier")]
	pub cfi_oct2015_identifier: String,
}


// CalculationType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CalculationType1Code {
	#[validate(enumerate = ["AFTX", "ANNU", "ISSU", "AVMA", "BOOK", "YTNC", "CHCL", "CLOS", "CMPD", "CUYI", "TRGR", "GVEQ", "FLAS", "NVFL", "LSCL", "LSMT", "LSQR", "LSYR", "LGAL", "MARK", "YTMA", "NXRF", "PNAV", "NXPT", "PRCL", "PRYL", "SEMI", "SHLF", "SPLL", "TXQV", "TTDT", "TRYL", "WRST"])]
	#[serde(rename = "CalculationType1Code")]
	pub calculation_type1_code: String,
}


// CalculationType3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CalculationType3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// CallType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CallType1Code {
	#[validate(enumerate = ["LOTT", "PRTA"])]
	#[serde(rename = "CallType1Code")]
	pub call_type1_code: String,
}


// CallType3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CallType3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// ClassificationType2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ClassificationType2 {
	#[serde(rename = "ClssfctnFinInstrm")]
	pub clssfctn_fin_instrm: Option<String>,
	#[serde(rename = "FinInstrmPdctTpCd")]
	pub fin_instrm_pdct_tp_cd: Option<String>,
	#[validate]
	#[serde(rename = "AltrnClssfctn")]
	pub altrn_clssfctn: Option<Vec<GenericIdentification36>>,
}


// CommonFinancialInstrumentAttributes11 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CommonFinancialInstrumentAttributes11 {
	#[validate]
	#[serde(rename = "SctySts")]
	pub scty_sts: Option<SecurityStatus3Choice>,
	#[validate]
	#[serde(rename = "FinInstrmNm")]
	pub fin_instrm_nm: Option<Vec<FinancialInstrumentName2>>,
	#[serde(rename = "DnmtnCcy")]
	pub dnmtn_ccy: Option<String>,
	#[serde(rename = "CertNb")]
	pub cert_nb: Option<String>,
	#[serde(rename = "CtrctVrsnNb")]
	pub ctrct_vrsn_nb: Option<f64>,
	#[serde(rename = "CpnAttchdNb")]
	pub cpn_attchd_nb: Option<String>,
	#[serde(rename = "TaxLotNb")]
	pub tax_lot_nb: Option<String>,
	#[serde(rename = "PoolNb")]
	pub pool_nb: Option<String>,
	#[serde(rename = "CvrdInd")]
	pub cvrd_ind: Option<bool>,
	#[validate]
	#[serde(rename = "LglRstrctns")]
	pub lgl_rstrctns: Option<LegalRestrictions4Choice>,
	#[validate]
	#[serde(rename = "PosLmt")]
	pub pos_lmt: Option<FinancialInstrumentQuantity1Choice>,
	#[validate]
	#[serde(rename = "NearTermPosLmt")]
	pub near_term_pos_lmt: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "ListgDt")]
	pub listg_dt: Option<String>,
	#[serde(rename = "RcrdDt")]
	pub rcrd_dt: Option<String>,
	#[serde(rename = "XpryDt")]
	pub xpry_dt: Option<String>,
	#[serde(rename = "Purp")]
	pub purp: Option<String>,
	#[validate]
	#[serde(rename = "ClssfctnTp")]
	pub clssfctn_tp: Option<ClassificationType2>,
	#[validate]
	#[serde(rename = "Issnc")]
	pub issnc: Option<Issuance6>,
	#[validate]
	#[serde(rename = "TradgMkt")]
	pub tradg_mkt: Option<Vec<TradingParameters2>>,
	#[validate]
	#[serde(rename = "SprdAndBchmkCrv")]
	pub sprd_and_bchmk_crv: Option<Vec<BenchmarkCurve6>>,
	#[validate]
	#[serde(rename = "PutTp")]
	pub put_tp: Option<PutType3Choice>,
	#[validate]
	#[serde(rename = "CallTp")]
	pub call_tp: Option<CallType3Choice>,
	#[serde(rename = "FngbInd")]
	pub fngb_ind: Option<bool>,
	#[serde(rename = "Cnfdtl")]
	pub cnfdtl: Option<bool>,
	#[serde(rename = "PrvtPlcmnt")]
	pub prvt_plcmnt: Option<bool>,
	#[serde(rename = "ConvtblInd")]
	pub convtbl_ind: Option<bool>,
	#[validate]
	#[serde(rename = "ConvsPrd")]
	pub convs_prd: Option<DateTimePeriod1>,
	#[validate]
	#[serde(rename = "ConvsRatioNmrtr")]
	pub convs_ratio_nmrtr: Option<FinancialInstrumentQuantity1Choice>,
	#[validate]
	#[serde(rename = "ConvsRatioDnmtr")]
	pub convs_ratio_dnmtr: Option<FinancialInstrumentQuantity1Choice>,
	#[validate]
	#[serde(rename = "PmryPlcOfDpst")]
	pub pmry_plc_of_dpst: Option<PartyIdentification136>,
	#[validate]
	#[serde(rename = "TradgMtd")]
	pub tradg_mtd: Option<UnitOrFaceAmount1Choice>,
	#[validate]
	#[serde(rename = "TEFRARule")]
	pub tefra_rule: Option<TEFRARules3Choice>,
	#[serde(rename = "SrNb")]
	pub sr_nb: Option<String>,
	#[serde(rename = "Clss")]
	pub clss: Option<String>,
	#[validate]
	#[serde(rename = "WhldgTaxRgm")]
	pub whldg_tax_rgm: Option<Vec<SecurityWithHoldingTax1>>,
	#[validate]
	#[serde(rename = "PmtSts")]
	pub pmt_sts: Option<SecuritiesPaymentStatus5Choice>,
	#[validate]
	#[serde(rename = "InitlPhysForm")]
	pub initl_phys_form: Option<InitialPhysicalForm4Choice>,
	#[validate]
	#[serde(rename = "AftrXchgPhysForm")]
	pub aftr_xchg_phys_form: Option<InitialPhysicalForm3Choice>,
	#[validate]
	#[serde(rename = "CmonSfkpr")]
	pub cmon_sfkpr: Option<PartyIdentification177Choice>,
	#[validate]
	#[serde(rename = "RedTp")]
	pub red_tp: Option<MaturityRedemptionType3Choice>,
	#[serde(rename = "RedPmtCcy")]
	pub red_pmt_ccy: Option<String>,
	#[validate]
	#[serde(rename = "Rstrctn")]
	pub rstrctn: Option<Vec<SecurityRestriction3>>,
	#[validate]
	#[serde(rename = "FinInstrmIdVldty")]
	pub fin_instrm_id_vldty: Option<Vec<FinancialInstrumentIdentificationValidity3>>,
	#[validate]
	#[serde(rename = "SttlmInf")]
	pub sttlm_inf: Option<Vec<SettlementInformation17>>,
	#[validate]
	#[serde(rename = "FinInstrmForm")]
	pub fin_instrm_form: Option<FinancialInstrumentForm2>,
	#[validate]
	#[serde(rename = "CtctNm")]
	pub ctct_nm: Option<Organisation38>,
	#[validate]
	#[serde(rename = "LeadMgr")]
	pub lead_mgr: Option<Organisation38>,
	#[validate]
	#[serde(rename = "PrncplPngAgt")]
	pub prncpl_png_agt: Option<Organisation38>,
	#[validate]
	#[serde(rename = "PngAgt")]
	pub png_agt: Option<Organisation38>,
	#[validate]
	#[serde(rename = "Dpstry")]
	pub dpstry: Option<Organisation38>,
	#[validate]
	#[serde(rename = "UndrlygRsk")]
	pub undrlyg_rsk: Option<Organisation38>,
	#[validate]
	#[serde(rename = "SctyCSDLk")]
	pub scty_csd_lk: Option<Vec<SecurityCSDLink7>>,
}


// CommunicationAddress3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CommunicationAddress3 {
	#[serde(rename = "Email")]
	pub email: Option<String>,
	#[serde(rename = "Phne")]
	pub phne: Option<String>,
	#[serde(rename = "Mob")]
	pub mob: Option<String>,
	#[serde(rename = "FaxNb")]
	pub fax_nb: Option<String>,
	#[serde(rename = "TlxAdr")]
	pub tlx_adr: Option<String>,
	#[serde(rename = "URLAdr")]
	pub url_adr: Option<String>,
}


// CountryCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CountryCode {
	#[validate(pattern = "[A-Z]{2,2}")]
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// DateAndDateTime2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DateAndDateTime2Choice {
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm")]
	pub dt_tm: Option<String>,
}


// DateTimePeriod1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DateTimePeriod1 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: String,
}


// DateTimePeriod1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DateTimePeriod1Choice {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: Option<String>,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: Option<String>,
	#[validate]
	#[serde(rename = "DtTmRg")]
	pub dt_tm_rg: Option<DateTimePeriod1>,
}


// DateTimePeriod2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DateTimePeriod2 {
	#[serde(rename = "FrDtTm")]
	pub fr_dt_tm: String,
	#[serde(rename = "ToDtTm")]
	pub to_dt_tm: Option<String>,
}


// Debt5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Debt5 {
	#[serde(rename = "PmtCcy")]
	pub pmt_ccy: Option<String>,
	#[validate]
	#[serde(rename = "FaceAmt")]
	pub face_amt: Option<ActiveCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "PmtFrqcy")]
	pub pmt_frqcy: Option<Frequency35Choice>,
	#[serde(rename = "IntrstFxgDt")]
	pub intrst_fxg_dt: Option<String>,
	#[serde(rename = "DtdDt")]
	pub dtd_dt: Option<String>,
	#[serde(rename = "FrstPmtDt")]
	pub frst_pmt_dt: Option<String>,
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: Option<String>,
	#[serde(rename = "NxtCpnDt")]
	pub nxt_cpn_dt: Option<String>,
	#[serde(rename = "PutblDt")]
	pub putbl_dt: Option<String>,
	#[serde(rename = "NxtCllblDt")]
	pub nxt_cllbl_dt: Option<String>,
	#[serde(rename = "NxtFctrDt")]
	pub nxt_fctr_dt: Option<String>,
	#[serde(rename = "XprtnDt")]
	pub xprtn_dt: Option<String>,
	#[serde(rename = "PmtDrctnInd")]
	pub pmt_drctn_ind: Option<bool>,
	#[serde(rename = "IntrstRate")]
	pub intrst_rate: Option<f64>,
	#[serde(rename = "NxtIntrstRate")]
	pub nxt_intrst_rate: Option<f64>,
	#[serde(rename = "OddCpnInd")]
	pub odd_cpn_ind: Option<bool>,
	#[serde(rename = "CllblInd")]
	pub cllbl_ind: Option<bool>,
	#[serde(rename = "CPPrgm")]
	pub cp_prgm: Option<f64>,
	#[serde(rename = "CPRegnTp")]
	pub cp_regn_tp: Option<String>,
	#[serde(rename = "IntrstAcrlDt")]
	pub intrst_acrl_dt: Option<String>,
	#[serde(rename = "PutblInd")]
	pub putbl_ind: Option<bool>,
	#[serde(rename = "PreFnddInd")]
	pub pre_fndd_ind: Option<bool>,
	#[serde(rename = "EscrwdInd")]
	pub escrwd_ind: Option<bool>,
	#[serde(rename = "PerptlInd")]
	pub perptl_ind: Option<bool>,
	#[serde(rename = "SubrdntdInd")]
	pub subrdntd_ind: Option<bool>,
	#[serde(rename = "XtndblInd")]
	pub xtndbl_ind: Option<bool>,
	#[validate]
	#[serde(rename = "XtndblPrd")]
	pub xtndbl_prd: Option<DateTimePeriod1Choice>,
	#[serde(rename = "VarblRateInd")]
	pub varbl_rate_ind: Option<bool>,
	#[validate]
	#[serde(rename = "OverAlltmtAmt")]
	pub over_alltmt_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "OverAlltmtRate")]
	pub over_alltmt_rate: Option<f64>,
	#[serde(rename = "AmtsblInd")]
	pub amtsbl_ind: Option<bool>,
	#[serde(rename = "IntrstClctnMtd")]
	pub intrst_clctn_mtd: Option<String>,
	#[validate]
	#[serde(rename = "CptlsdIntrst")]
	pub cptlsd_intrst: Option<DistributionPolicy2Choice>,
	#[validate]
	#[serde(rename = "ActlDnmtnAmt")]
	pub actl_dnmtn_amt: Option<Vec<ActiveCurrencyAndAmount>>,
	#[serde(rename = "CurFctr")]
	pub cur_fctr: Option<f64>,
	#[serde(rename = "NxtFctr")]
	pub nxt_fctr: Option<f64>,
	#[serde(rename = "PrvsFctr")]
	pub prvs_fctr: Option<f64>,
	#[serde(rename = "Pcs")]
	pub pcs: Option<f64>,
	#[serde(rename = "PlsMax")]
	pub pls_max: Option<f64>,
	#[serde(rename = "PlsPerMln")]
	pub pls_per_mln: Option<f64>,
	#[serde(rename = "PlsPerLot")]
	pub pls_per_lot: Option<f64>,
	#[serde(rename = "PlsPerTrad")]
	pub pls_per_trad: Option<f64>,
	#[serde(rename = "CstPrePmtPnltyInd")]
	pub cst_pre_pmt_pnlty_ind: Option<bool>,
	#[serde(rename = "LotId")]
	pub lot_id: Option<String>,
	#[serde(rename = "CstPrePmtYld")]
	pub cst_pre_pmt_yld: Option<f64>,
	#[serde(rename = "WghtdAvrgCpn")]
	pub wghtd_avrg_cpn: Option<f64>,
	#[serde(rename = "WghtdAvrgLife")]
	pub wghtd_avrg_life: Option<f64>,
	#[serde(rename = "WghtdAvrgLn")]
	pub wghtd_avrg_ln: Option<f64>,
	#[serde(rename = "WghtdAvrgMtrty")]
	pub wghtd_avrg_mtrty: Option<f64>,
	#[serde(rename = "InsrdInd")]
	pub insrd_ind: Option<bool>,
	#[serde(rename = "BkQlfdInd")]
	pub bk_qlfd_ind: Option<bool>,
	#[validate]
	#[serde(rename = "YldClctn")]
	pub yld_clctn: Option<Vec<YieldCalculation6>>,
	#[serde(rename = "IntrstTp")]
	pub intrst_tp: Option<String>,
	#[validate]
	#[serde(rename = "InstrmStrTp")]
	pub instrm_str_tp: Option<InstrumentSubStructureType2Choice>,
	#[validate]
	#[serde(rename = "GblTp")]
	pub gbl_tp: Option<GlobalNote2Choice>,
	#[serde(rename = "PotntlEuroSysElgblty")]
	pub potntl_euro_sys_elgblty: Option<bool>,
	#[serde(rename = "Geogcs")]
	pub geogcs: Option<String>,
	#[validate]
	#[serde(rename = "YldRg")]
	pub yld_rg: Option<AmountOrPercentageRange1>,
	#[validate]
	#[serde(rename = "CpnRg")]
	pub cpn_rg: Option<AmountOrPercentageRange1>,
	#[serde(rename = "Purp")]
	pub purp: Option<String>,
	#[serde(rename = "AltrntvMinTaxInd")]
	pub altrntv_min_tax_ind: Option<bool>,
	#[serde(rename = "AutoRinvstmt")]
	pub auto_rinvstmt: Option<f64>,
	#[serde(rename = "Hrcut")]
	pub hrcut: Option<f64>,
	#[validate]
	#[serde(rename = "TxConds")]
	pub tx_conds: Option<TradeTransactionCondition7Choice>,
	#[serde(rename = "LookBck")]
	pub look_bck: Option<f64>,
	#[serde(rename = "MaxSbstitn")]
	pub max_sbstitn: Option<f64>,
	#[validate]
	#[serde(rename = "MinIncrmt")]
	pub min_incrmt: Option<FinancialInstrumentQuantity1Choice>,
	#[validate]
	#[serde(rename = "MinQty")]
	pub min_qty: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "Pdctn")]
	pub pdctn: Option<String>,
	#[serde(rename = "RstrctdInd")]
	pub rstrctd_ind: Option<bool>,
	#[validate]
	#[serde(rename = "PricFrqcy")]
	pub pric_frqcy: Option<Frequency35Choice>,
	#[serde(rename = "Sctr")]
	pub sctr: Option<String>,
	#[validate]
	#[serde(rename = "SbstitnFrqcy")]
	pub sbstitn_frqcy: Option<Frequency35Choice>,
	#[serde(rename = "SbstitnLft")]
	pub sbstitn_lft: Option<f64>,
	#[serde(rename = "WhlPoolInd")]
	pub whl_pool_ind: Option<bool>,
	#[serde(rename = "PricSrc")]
	pub pric_src: Option<String>,
	#[validate]
	#[serde(rename = "PricRg")]
	pub pric_rg: Option<AmountOrPercentageRange1>,
}


// DecimalNumber ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DecimalNumber {
	#[serde(rename = "DecimalNumber")]
	pub decimal_number: f64,
}


// Derivative4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Derivative4 {
	#[validate]
	#[serde(rename = "Futr")]
	pub futr: Option<Future4>,
	#[validate]
	#[serde(rename = "Optn")]
	pub optn: Option<Option15>,
}


// DistributionPolicy1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DistributionPolicy1Code {
	#[validate(enumerate = ["DIST", "ACCU"])]
	#[serde(rename = "DistributionPolicy1Code")]
	pub distribution_policy1_code: String,
}


// DistributionPolicy2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DistributionPolicy2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// Equity3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Equity3 {
	#[validate]
	#[serde(rename = "PrefToIncm")]
	pub pref_to_incm: PreferenceToIncome5Choice,
	#[serde(rename = "MtrtyDt")]
	pub mtrty_dt: Option<String>,
	#[validate]
	#[serde(rename = "NonPdAmt")]
	pub non_pd_amt: Option<ActiveCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "ParVal")]
	pub par_val: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "VtngRghtsPerShr")]
	pub vtng_rghts_per_shr: Option<f64>,
}


// ErrorHandling3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ErrorHandling3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ErrorHandling5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ErrorHandling5 {
	#[validate]
	#[serde(rename = "Err")]
	pub err: ErrorHandling3Choice,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// Exact4AlphaNumericText ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Exact4AlphaNumericText {
	#[validate(pattern = "[a-zA-Z0-9]{4}")]
	#[serde(rename = "Exact4AlphaNumericText")]
	pub exact4_alpha_numeric_text: String,
}


// ExternalFinancialInstrumentIdentificationType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalFinancialInstrumentIdentificationType1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalFinancialInstrumentIdentificationType1Code")]
	pub external_financial_instrument_identification_type1_code: String,
}


// ExternalFinancialInstrumentProductType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalFinancialInstrumentProductType1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalFinancialInstrumentProductType1Code")]
	pub external_financial_instrument_product_type1_code: String,
}


// ExternalSystemErrorHandling1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalSystemErrorHandling1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalSystemErrorHandling1Code")]
	pub external_system_error_handling1_code: String,
}


// FinancialInstrument97 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstrument97 {
	#[validate]
	#[serde(rename = "Eqty")]
	pub eqty: Option<Equity3>,
	#[validate]
	#[serde(rename = "Warrt")]
	pub warrt: Option<Warrant4>,
	#[validate]
	#[serde(rename = "Debt")]
	pub debt: Option<Debt5>,
	#[validate]
	#[serde(rename = "Deriv")]
	pub deriv: Option<Derivative4>,
}


// FinancialInstrumentForm2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstrumentForm2 {
	#[validate]
	#[serde(rename = "BookgApprnc")]
	pub bookg_apprnc: Option<Appearance3Choice>,
	#[validate]
	#[serde(rename = "LglForm")]
	pub lgl_form: Option<FormOfSecurity8Choice>,
}


// FinancialInstrumentIdentificationValidity3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstrumentIdentificationValidity3 {
	#[validate]
	#[serde(rename = "FinInstrmId")]
	pub fin_instrm_id: Option<SecurityIdentification39>,
	#[serde(rename = "ISINVldFr")]
	pub isin_vld_fr: Option<String>,
}


// FinancialInstrumentName2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstrumentName2 {
	#[serde(rename = "ISOShrtNm")]
	pub iso_shrt_nm: Option<String>,
	#[serde(rename = "ISOLngNm")]
	pub iso_lng_nm: Option<String>,
	#[validate]
	#[serde(rename = "VldFr")]
	pub vld_fr: Option<DateAndDateTime2Choice>,
}


// FinancialInstrumentQuantity1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FinancialInstrumentQuantity1Choice {
	#[serde(rename = "Unit")]
	pub unit: Option<f64>,
	#[serde(rename = "FaceAmt")]
	pub face_amt: Option<f64>,
	#[serde(rename = "AmtsdVal")]
	pub amtsd_val: Option<f64>,
}


// FormOfSecurity1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FormOfSecurity1Code {
	#[validate(enumerate = ["BEAR", "REGD"])]
	#[serde(rename = "FormOfSecurity1Code")]
	pub form_of_security1_code: String,
}


// FormOfSecurity8Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct FormOfSecurity8Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// Frequency35Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Frequency35Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// Frequency5Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Frequency5Code {
	#[validate(enumerate = ["YEAR", "MNTH", "QURT", "MIAN", "WEEK", "DAIL", "ADHO", "INDA", "OVNG", "TEND"])]
	#[serde(rename = "Frequency5Code")]
	pub frequency5_code: String,
}


// Future4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Future4 {
	#[serde(rename = "CtrctSz")]
	pub ctrct_sz: Option<f64>,
	#[validate]
	#[serde(rename = "ExrcPric")]
	pub exrc_pric: Option<Price8>,
	#[serde(rename = "FutrDt")]
	pub futr_dt: Option<String>,
	#[validate]
	#[serde(rename = "MinSz")]
	pub min_sz: Option<ActiveCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "UnitOfMeasr")]
	pub unit_of_measr: Option<UnitOfMeasure7Choice>,
	#[validate]
	#[serde(rename = "TmUnit")]
	pub tm_unit: Option<TimeUnit3Choice>,
	#[validate]
	#[serde(rename = "AddtlUndrlygAttrbts")]
	pub addtl_undrlyg_attrbts: Option<Vec<UnderlyingAttributes4>>,
}


// GenericIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericIdentification13 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification13 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
	#[serde(rename = "Issr")]
	pub issr: String,
}


// GenericIdentification30 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification30 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// GenericIdentification36 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericIdentification36 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Issr")]
	pub issr: String,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<String>,
}


// GlobalNote1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GlobalNote1Code {
	#[validate(enumerate = ["NGNO", "CGNO"])]
	#[serde(rename = "GlobalNote1Code")]
	pub global_note1_code: String,
}


// GlobalNote2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GlobalNote2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// ISIN2021Identifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISIN2021Identifier {
	#[validate(pattern = "[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}")]
	#[serde(rename = "ISIN2021Identifier")]
	pub isin2021_identifier: String,
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


// ISOYearMonth ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISOYearMonth {
	#[serde(rename = "ISOYearMonth")]
	pub iso_year_month: String,
}


// IdentificationSource3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct IdentificationSource3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// ImpliedCurrencyAndAmount ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ImpliedCurrencyAndAmount {
	#[serde(rename = "ImpliedCurrencyAndAmount")]
	pub implied_currency_and_amount: f64,
}


// InitialPhysicalForm1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InitialPhysicalForm1Code {
	#[validate(enumerate = ["GTGT", "GPGP", "DERN"])]
	#[serde(rename = "InitialPhysicalForm1Code")]
	pub initial_physical_form1_code: String,
}


// InitialPhysicalForm2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InitialPhysicalForm2Code {
	#[validate(enumerate = ["GPGP", "DERN"])]
	#[serde(rename = "InitialPhysicalForm2Code")]
	pub initial_physical_form2_code: String,
}


// InitialPhysicalForm3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InitialPhysicalForm3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// InitialPhysicalForm4Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InitialPhysicalForm4Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// InstrumentSubStructureType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InstrumentSubStructureType1Code {
	#[validate(enumerate = ["ABSE", "AIRT", "AUTT", "CBOB", "CDOB", "CLNO", "CLOB", "CMBS", "CSMR", "CRCT", "HELO", "LPNO", "PFAB", "PYRT", "REPK", "RMBS", "SCBO", "STRB", "STUT", "WBSE"])]
	#[serde(rename = "InstrumentSubStructureType1Code")]
	pub instrument_sub_structure_type1_code: String,
}


// InstrumentSubStructureType2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InstrumentSubStructureType2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// InterestType3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InterestType3Code {
	#[validate(enumerate = ["ZCPN", "FIXD", "FLRN", "DUAL", "INDE", "DSCO"])]
	#[serde(rename = "InterestType3Code")]
	pub interest_type3_code: String,
}


// InvestorRestrictionType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestorRestrictionType1Code {
	#[validate(enumerate = ["LERE", "CITI", "INDV"])]
	#[serde(rename = "InvestorRestrictionType1Code")]
	pub investor_restriction_type1_code: String,
}


// InvestorRestrictionType3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestorRestrictionType3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// InvestorType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestorType1Code {
	#[validate(enumerate = ["RETL", "PROF", "STAF", "PPER"])]
	#[serde(rename = "InvestorType1Code")]
	pub investor_type1_code: String,
}


// InvestorType3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvestorType3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// Issuance6 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Issuance6 {
	#[serde(rename = "IssePlc")]
	pub isse_plc: Option<String>,
	#[serde(rename = "CtryOfIsse")]
	pub ctry_of_isse: Option<String>,
	#[serde(rename = "IsseDt")]
	pub isse_dt: Option<String>,
	#[serde(rename = "AnncmntDt")]
	pub anncmnt_dt: Option<String>,
	#[validate]
	#[serde(rename = "IssrOrg")]
	pub issr_org: Option<Organisation38>,
	#[validate]
	#[serde(rename = "IsseNmnlAmt")]
	pub isse_nmnl_amt: Option<FinancialInstrumentQuantity1Choice>,
	#[validate]
	#[serde(rename = "FullIssdAmt")]
	pub full_issd_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "IsseSz")]
	pub isse_sz: Option<f64>,
	#[validate]
	#[serde(rename = "IssePric")]
	pub isse_pric: Option<PriceValue1>,
	#[validate]
	#[serde(rename = "IssncDstrbtn")]
	pub issnc_dstrbtn: Option<SecuritiesTransactionType31Choice>,
	#[validate]
	#[serde(rename = "GovngLaw")]
	pub govng_law: Option<Vec<Jurisdiction1>>,
}


// IssuanceAccount2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct IssuanceAccount2 {
	#[validate]
	#[serde(rename = "IssncAcct")]
	pub issnc_acct: SecuritiesAccount19,
	#[serde(rename = "PmryAcctInd")]
	pub pmry_acct_ind: bool,
}


// Jurisdiction1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Jurisdiction1 {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
}


// LEIIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[validate(pattern = "[A-Z0-9]{18,18}[0-9]{2,2}")]
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LegalRestrictions1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LegalRestrictions1Code {
	#[validate(enumerate = ["USLE", "NORE", "REST"])]
	#[serde(rename = "LegalRestrictions1Code")]
	pub legal_restrictions1_code: String,
}


// LegalRestrictions2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LegalRestrictions2Code {
	#[validate(enumerate = ["JURO", "PPLA", "ACRI", "MARG", "PRIV"])]
	#[serde(rename = "LegalRestrictions2Code")]
	pub legal_restrictions2_code: String,
}


// LegalRestrictions4Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LegalRestrictions4Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// LegalRestrictions5Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LegalRestrictions5Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// MICIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MICIdentifier {
	#[validate(pattern = "[A-Z0-9]{4,4}")]
	#[serde(rename = "MICIdentifier")]
	pub mic_identifier: String,
}


// MaturityRedemptionType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MaturityRedemptionType1Code {
	#[validate(enumerate = ["FRED", "PRNR", "PRWR", "RNDM", "PRRA", "CALL", "PUUT"])]
	#[serde(rename = "MaturityRedemptionType1Code")]
	pub maturity_redemption_type1_code: String,
}


// MaturityRedemptionType3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MaturityRedemptionType3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// Max140Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max140Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 140)]
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max15NumericText ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max15NumericText {
	#[validate(pattern = "[0-9]{1,15}")]
	#[serde(rename = "Max15NumericText")]
	pub max15_numeric_text: String,
}


// Max16Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max16Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 16)]
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
}


// Max256Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max256Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 256)]
	#[serde(rename = "Max256Text")]
	pub max256_text: String,
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


// Max3NumericText ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max3NumericText {
	#[validate(pattern = "[0-9]{1,3}")]
	#[serde(rename = "Max3NumericText")]
	pub max3_numeric_text: String,
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


// Max5NumericText ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max5NumericText {
	#[validate(pattern = "[0-9]{1,5}")]
	#[serde(rename = "Max5NumericText")]
	pub max5_numeric_text: String,
}


// Max70Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max70Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 70)]
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// MessageHeader12 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MessageHeader12 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: Option<String>,
	#[validate]
	#[serde(rename = "OrgnlBizInstr")]
	pub orgnl_biz_instr: Option<OriginalBusinessInstruction1>,
}


// NameAndAddress4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NameAndAddress4 {
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[validate]
	#[serde(rename = "Adr")]
	pub adr: PostalAddress1,
}


// NameAndAddress5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NameAndAddress5 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[validate]
	#[serde(rename = "Adr")]
	pub adr: Option<PostalAddress1>,
}


// Number ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
}


// Operation1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Operation1Code {
	#[validate(enumerate = ["TILL", "ORRR", "ANDD"])]
	#[serde(rename = "Operation1Code")]
	pub operation1_code: String,
}


// Operator1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Operator1Code {
	#[validate(enumerate = ["SMAL", "SMEQ", "GREA", "GREQ", "EQAL"])]
	#[serde(rename = "Operator1Code")]
	pub operator1_code: String,
}


// Option15 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Option15 {
	#[validate]
	#[serde(rename = "OptnSttlmStyle")]
	pub optn_sttlm_style: Option<SettleStyle2Choice>,
	#[serde(rename = "ConvsDt")]
	pub convs_dt: Option<String>,
	#[validate]
	#[serde(rename = "StrkPric")]
	pub strk_pric: Option<Price8>,
	#[validate]
	#[serde(rename = "MinExrcblQty")]
	pub min_exrcbl_qty: Option<FinancialInstrumentQuantity1Choice>,
	#[validate]
	#[serde(rename = "ConvsPrd")]
	pub convs_prd: Option<DateTimePeriod1Choice>,
	#[validate]
	#[serde(rename = "OptnStyle")]
	pub optn_style: Option<OptionStyle1Choice>,
	#[validate]
	#[serde(rename = "OptnTp")]
	pub optn_tp: Option<OptionType8Choice>,
	#[serde(rename = "StrkVal")]
	pub strk_val: Option<f64>,
	#[serde(rename = "StrkMltplr")]
	pub strk_mltplr: Option<f64>,
	#[validate]
	#[serde(rename = "InstrmAssgnmtMtd")]
	pub instrm_assgnmt_mtd: Option<AssignmentMethod2Choice>,
	#[serde(rename = "VrsnNb")]
	pub vrsn_nb: Option<f64>,
	#[serde(rename = "XpryLctn")]
	pub xpry_lctn: Option<String>,
	#[validate]
	#[serde(rename = "Stdstn")]
	pub stdstn: Option<Standardisation3Choice>,
	#[validate]
	#[serde(rename = "TradgPtyRole")]
	pub tradg_pty_role: Option<OptionParty3Choice>,
	#[serde(rename = "CtrctSz")]
	pub ctrct_sz: Option<f64>,
	#[validate]
	#[serde(rename = "AddtlUndrlygAttrbts")]
	pub addtl_undrlyg_attrbts: Option<Vec<UnderlyingAttributes4>>,
}


// OptionParty1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OptionParty1Code {
	#[validate(enumerate = ["SLLR", "BYER"])]
	#[serde(rename = "OptionParty1Code")]
	pub option_party1_code: String,
}


// OptionParty3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OptionParty3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<Vec<String>>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// OptionStyle1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OptionStyle1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification13>,
}


// OptionStyle1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OptionStyle1Code {
	#[validate(enumerate = ["AMER", "EURO", "BERM", "ASIA", "CANA"])]
	#[serde(rename = "OptionStyle1Code")]
	pub option_style1_code: String,
}


// OptionType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OptionType1Code {
	#[validate(enumerate = ["CALL", "PUTO"])]
	#[serde(rename = "OptionType1Code")]
	pub option_type1_code: String,
}


// OptionType8Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OptionType8Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<Vec<String>>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// Organisation38 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Organisation38 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[validate]
	#[serde(rename = "Id")]
	pub id: Option<PartyIdentification177Choice>,
	#[serde(rename = "Purp")]
	pub purp: Option<String>,
	#[serde(rename = "TaxtnCtry")]
	pub taxtn_ctry: Option<String>,
	#[serde(rename = "RegnCtry")]
	pub regn_ctry: Option<String>,
	#[serde(rename = "RegnDt")]
	pub regn_dt: Option<String>,
	#[serde(rename = "TaxIdNb")]
	pub tax_id_nb: Option<String>,
	#[serde(rename = "NtlRegnNb")]
	pub ntl_regn_nb: Option<String>,
	#[validate]
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Vec<PostalAddress3>,
	#[validate]
	#[serde(rename = "PmryComAdr")]
	pub pmry_com_adr: Option<CommunicationAddress3>,
	#[validate]
	#[serde(rename = "ScndryComAdr")]
	pub scndry_com_adr: Option<CommunicationAddress3>,
}


// OriginalBusinessInstruction1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OriginalBusinessInstruction1 {
	#[serde(rename = "MsgId")]
	pub msg_id: String,
	#[serde(rename = "MsgNmId")]
	pub msg_nm_id: Option<String>,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: Option<String>,
}


// OtherIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OtherIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Sfx")]
	pub sfx: Option<String>,
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: IdentificationSource3Choice,
}


// Pagination1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Pagination1 {
	#[serde(rename = "PgNb")]
	pub pg_nb: String,
	#[serde(rename = "LastPgInd")]
	pub last_pg_ind: bool,
}


// PartyIdentification120Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification120Choice {
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
	#[validate]
	#[serde(rename = "PrtryId")]
	pub prtry_id: Option<GenericIdentification36>,
	#[validate]
	#[serde(rename = "NmAndAdr")]
	pub nm_and_adr: Option<NameAndAddress5>,
}


// PartyIdentification136 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification136 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: PartyIdentification120Choice,
	#[serde(rename = "LEI")]
	pub lei: Option<String>,
}


// PartyIdentification177Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification177Choice {
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
	#[validate]
	#[serde(rename = "PrtryId")]
	pub prtry_id: Option<GenericIdentification1>,
}


// PaymentDirectionIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PaymentDirectionIndicator {
	#[serde(rename = "PaymentDirectionIndicator")]
	pub payment_direction_indicator: bool,
}


// PercentageRate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// PhoneNumber ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PhoneNumber {
	#[validate(pattern = "\\+[0-9]{1,3}-[0-9()+\\-]{1,30}")]
	#[serde(rename = "PhoneNumber")]
	pub phone_number: String,
}


// PostalAddress1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PostalAddress1 {
	#[serde(rename = "AdrTp")]
	pub adr_tp: Option<String>,
	#[serde(rename = "AdrLine")]
	pub adr_line: Option<Vec<String>>,
	#[serde(rename = "StrtNm")]
	pub strt_nm: Option<String>,
	#[serde(rename = "BldgNb")]
	pub bldg_nb: Option<String>,
	#[serde(rename = "PstCd")]
	pub pst_cd: Option<String>,
	#[serde(rename = "TwnNm")]
	pub twn_nm: Option<String>,
	#[serde(rename = "CtrySubDvsn")]
	pub ctry_sub_dvsn: Option<String>,
	#[serde(rename = "Ctry")]
	pub ctry: String,
}


// PostalAddress3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PostalAddress3 {
	#[serde(rename = "AdrTp")]
	pub adr_tp: String,
	#[serde(rename = "MlngInd")]
	pub mlng_ind: bool,
	#[serde(rename = "RegnAdrInd")]
	pub regn_adr_ind: bool,
	#[validate]
	#[serde(rename = "NmAndAdr")]
	pub nm_and_adr: NameAndAddress4,
}


// PreferenceToIncome1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PreferenceToIncome1Code {
	#[validate(enumerate = ["ORDN", "PFRD"])]
	#[serde(rename = "PreferenceToIncome1Code")]
	pub preference_to_income1_code: String,
}


// PreferenceToIncome5Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PreferenceToIncome5Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// Price8 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Price8 {
	#[serde(rename = "ValTp")]
	pub val_tp: Option<String>,
	#[validate]
	#[serde(rename = "Val")]
	pub val: PriceRateOrAmount3Choice,
	#[serde(rename = "PricTp")]
	pub pric_tp: Option<String>,
}


// PriceRateOrAmount3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PriceRateOrAmount3Choice {
	#[serde(rename = "Rate")]
	pub rate: Option<f64>,
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
}


// PriceValue1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PriceValue1 {
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: ActiveCurrencyAnd13DecimalAmount,
}


// PriceValueType3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PriceValueType3Code {
	#[validate(enumerate = ["DISC", "PREM", "PARV", "YIEL", "SPRE", "PEUN", "ABSO", "TEDP", "TEDY", "FICT", "VACT"])]
	#[serde(rename = "PriceValueType3Code")]
	pub price_value_type3_code: String,
}


// PutType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PutType1Code {
	#[validate(enumerate = ["MAND", "OPTI", "TWOS"])]
	#[serde(rename = "PutType1Code")]
	pub put_type1_code: String,
}


// PutType3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PutType3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// RateAndAmountFormat1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RateAndAmountFormat1Choice {
	#[serde(rename = "Rate")]
	pub rate: Option<f64>,
	#[validate]
	#[serde(rename = "Amt")]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "NotSpcfdRate")]
	pub not_spcfd_rate: Option<RateType12FormatChoice>,
}


// RateOrAbsoluteValue1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RateOrAbsoluteValue1Choice {
	#[serde(rename = "RateVal")]
	pub rate_val: Option<f64>,
	#[serde(rename = "AbsVal")]
	pub abs_val: Option<f64>,
}


// RateType12Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RateType12Code {
	#[validate(enumerate = ["OPEN", "UKWN", "NILP"])]
	#[serde(rename = "RateType12Code")]
	pub rate_type12_code: String,
}


// RateType12FormatChoice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RateType12FormatChoice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification13>,
}


// RestrictionType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct RestrictionType1Code {
	#[validate(enumerate = ["SELR", "BUYR", "PLAR", "HOLR", "VOTR"])]
	#[serde(rename = "RestrictionType1Code")]
	pub restriction_type1_code: String,
}


// SecuritiesAccount19 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesAccount19 {
	#[serde(rename = "Id")]
	pub id: String,
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: Option<GenericIdentification30>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
}


// SecuritiesPaymentStatus1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesPaymentStatus1Code {
	#[validate(enumerate = ["FULL", "NILL", "PART"])]
	#[serde(rename = "SecuritiesPaymentStatus1Code")]
	pub securities_payment_status1_code: String,
}


// SecuritiesPaymentStatus5Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesPaymentStatus5Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// SecuritiesTransactionType11Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesTransactionType11Code {
	#[validate(enumerate = ["NSYN", "SYND"])]
	#[serde(rename = "SecuritiesTransactionType11Code")]
	pub securities_transaction_type11_code: String,
}


// SecuritiesTransactionType31Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecuritiesTransactionType31Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// SecurityAttributes11 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityAttributes11 {
	#[validate]
	#[serde(rename = "FinInstrmId")]
	pub fin_instrm_id: Option<Vec<SecurityIdentification39>>,
	#[validate]
	#[serde(rename = "FinInstrmTp")]
	pub fin_instrm_tp: Option<FinancialInstrument97>,
	#[validate]
	#[serde(rename = "FinInstrmAttrbts")]
	pub fin_instrm_attrbts: Option<Vec<CommonFinancialInstrumentAttributes11>>,
}


// SecurityCSDLink7 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityCSDLink7 {
	#[validate]
	#[serde(rename = "VldFr")]
	pub vld_fr: DateAndDateTime2Choice,
	#[validate]
	#[serde(rename = "VldTo")]
	pub vld_to: Option<DateAndDateTime2Choice>,
	#[serde(rename = "SctyMntnc")]
	pub scty_mntnc: Option<bool>,
	#[validate]
	#[serde(rename = "IssrCSD")]
	pub issr_csd: Option<SystemPartyIdentification2Choice>,
	#[validate]
	#[serde(rename = "InvstrCSD")]
	pub invstr_csd: Option<SystemPartyIdentification2Choice>,
	#[validate]
	#[serde(rename = "TechIssrCSD")]
	pub tech_issr_csd: Option<SystemPartyIdentification2Choice>,
	#[validate]
	#[serde(rename = "IssncAcct")]
	pub issnc_acct: Option<Vec<IssuanceAccount2>>,
}


// SecurityIdentification39 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityIdentification39 {
	#[serde(rename = "ISIN")]
	pub isin: Option<String>,
	#[validate]
	#[serde(rename = "OthrId")]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
}


// SecurityOrBusinessError4Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityOrBusinessError4Choice {
	#[validate]
	#[serde(rename = "SctyRpt")]
	pub scty_rpt: Option<Vec<SecurityAttributes11>>,
	#[validate]
	#[serde(rename = "BizErr")]
	pub biz_err: Option<Vec<BusinessError4>>,
}


// SecurityOrOperationalError4Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityOrOperationalError4Choice {
	#[validate]
	#[serde(rename = "SctyRptOrBizErr")]
	pub scty_rpt_or_biz_err: Option<SecurityOrBusinessError4Choice>,
	#[validate]
	#[serde(rename = "OprlErr")]
	pub oprl_err: Option<Vec<ErrorHandling5>>,
}


// SecurityReportV01 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityReportV01 {
	#[validate]
	#[serde(rename = "MsgHdr")]
	pub msg_hdr: Option<MessageHeader12>,
	#[validate]
	#[serde(rename = "Pgntn")]
	pub pgntn: Pagination1,
	#[validate]
	#[serde(rename = "SctyRptOrErr")]
	pub scty_rpt_or_err: SecurityOrOperationalError4Choice,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// SecurityRestriction3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityRestriction3 {
	#[validate]
	#[serde(rename = "FctvPrd")]
	pub fctv_prd: Option<DateTimePeriod2>,
	#[validate]
	#[serde(rename = "RstrctnTp")]
	pub rstrctn_tp: Option<SecurityRestrictionType2Choice>,
	#[validate]
	#[serde(rename = "LglRstrctnTp")]
	pub lgl_rstrctn_tp: Option<LegalRestrictions5Choice>,
	#[validate]
	#[serde(rename = "InvstrRstrctnTp")]
	pub invstr_rstrctn_tp: Option<Vec<InvestorRestrictionType3Choice>>,
	#[validate]
	#[serde(rename = "InvstrTp")]
	pub invstr_tp: Option<Vec<InvestorType3Choice>>,
}


// SecurityRestrictionType2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityRestrictionType2Choice {
	#[serde(rename = "RstrctnTp")]
	pub rstrctn_tp: Option<String>,
	#[validate]
	#[serde(rename = "PrtryRstrctn")]
	pub prtry_rstrctn: Option<GenericIdentification30>,
}


// SecurityStatus2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityStatus2Code {
	#[validate(enumerate = ["ACTV", "INAC", "SUSP"])]
	#[serde(rename = "SecurityStatus2Code")]
	pub security_status2_code: String,
}


// SecurityStatus3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityStatus3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// SecurityWithHoldingTax1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SecurityWithHoldingTax1 {
	#[validate]
	#[serde(rename = "WhldgTaxVal")]
	pub whldg_tax_val: RateAndAmountFormat1Choice,
	#[serde(rename = "Ctry")]
	pub ctry: String,
}


// SettleStyle1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettleStyle1Code {
	#[validate(enumerate = ["SETC", "SETO"])]
	#[serde(rename = "SettleStyle1Code")]
	pub settle_style1_code: String,
}


// SettleStyle2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettleStyle2Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<Vec<String>>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// SettlementInformation17 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementInformation17 {
	#[validate]
	#[serde(rename = "SctiesQtyTp")]
	pub scties_qty_tp: Option<SettlementUnitType3Choice>,
	#[serde(rename = "CtrctSttlmMnth")]
	pub ctrct_sttlm_mnth: Option<String>,
	#[validate]
	#[serde(rename = "MinDnmtn")]
	pub min_dnmtn: Option<FinancialInstrumentQuantity1Choice>,
	#[validate]
	#[serde(rename = "MinMltplQty")]
	pub min_mltpl_qty: Option<FinancialInstrumentQuantity1Choice>,
	#[validate]
	#[serde(rename = "DevtgSttlmUnit")]
	pub devtg_sttlm_unit: Option<Vec<FinancialInstrumentQuantity1Choice>>,
}


// SettlementType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementType1Code {
	#[validate(enumerate = ["PRIN", "NETO"])]
	#[serde(rename = "SettlementType1Code")]
	pub settlement_type1_code: String,
}


// SettlementType3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementType3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// SettlementUnitType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementUnitType1Code {
	#[validate(enumerate = ["FAMT", "UNIT"])]
	#[serde(rename = "SettlementUnitType1Code")]
	pub settlement_unit_type1_code: String,
}


// SettlementUnitType3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementUnitType3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// Standardisation1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Standardisation1Code {
	#[validate(enumerate = ["FLEX", "NSTA", "STAN"])]
	#[serde(rename = "Standardisation1Code")]
	pub standardisation1_code: String,
}


// Standardisation3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Standardisation3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<Vec<String>>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
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


// SystemPartyIdentification2Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SystemPartyIdentification2Choice {
	#[validate]
	#[serde(rename = "OrgId")]
	pub org_id: Option<PartyIdentification136>,
	#[validate]
	#[serde(rename = "CmbndId")]
	pub cmbnd_id: Option<SystemPartyIdentification8>,
}


// SystemPartyIdentification8 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SystemPartyIdentification8 {
	#[validate]
	#[serde(rename = "Id")]
	pub id: PartyIdentification136,
	#[validate]
	#[serde(rename = "RspnsblPtyId")]
	pub rspnsbl_pty_id: Option<PartyIdentification136>,
}


// TEFRARules1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TEFRARules1Code {
	#[validate(enumerate = ["RULC", "RULD"])]
	#[serde(rename = "TEFRARules1Code")]
	pub tefra_rules1_code: String,
}


// TEFRARules3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TEFRARules3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// Term1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Term1 {
	#[serde(rename = "Oprtr")]
	pub oprtr: String,
	#[validate]
	#[serde(rename = "Val")]
	pub val: RateOrAbsoluteValue1Choice,
}


// TimeUnit1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TimeUnit1Code {
	#[validate(enumerate = ["DAYC", "HOUR", "MINU", "MNTH", "SECO", "WEEK", "YEAR"])]
	#[serde(rename = "TimeUnit1Code")]
	pub time_unit1_code: String,
}


// TimeUnit3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TimeUnit3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// TradeTransactionCondition2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeTransactionCondition2Code {
	#[validate(enumerate = ["SPCC", "SECN", "SEBN", "SCBN", "SCRT", "SERT", "SCCR", "SECR", "CAST", "SPPR", "SPCU", "SPEX", "GTDL"])]
	#[serde(rename = "TradeTransactionCondition2Code")]
	pub trade_transaction_condition2_code: String,
}


// TradeTransactionCondition7Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeTransactionCondition7Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// TradingParameters2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradingParameters2 {
	#[serde(rename = "MktId")]
	pub mkt_id: Option<String>,
	#[validate]
	#[serde(rename = "RndLot")]
	pub rnd_lot: Option<FinancialInstrumentQuantity1Choice>,
	#[validate]
	#[serde(rename = "TradLotSz")]
	pub trad_lot_sz: Option<FinancialInstrumentQuantity1Choice>,
	#[serde(rename = "ScndryPlcOfListg")]
	pub scndry_plc_of_listg: Option<Vec<String>>,
	#[validate]
	#[serde(rename = "MinTraddNmnlQty")]
	pub min_tradd_nmnl_qty: Option<UnitOrFaceAmount1Choice>,
	#[validate]
	#[serde(rename = "MaxTraddNmnlQty")]
	pub max_tradd_nmnl_qty: Option<UnitOrFaceAmount1Choice>,
	#[serde(rename = "MinTradgPricgIncrmt")]
	pub min_tradg_pricg_incrmt: Option<f64>,
	#[serde(rename = "PmryPlcOfListgId")]
	pub pmry_plc_of_listg_id: Option<String>,
}


// TypeOfPrice1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TypeOfPrice1Code {
	#[validate(enumerate = ["AVER", "AVOV", "COMB", "GREX", "LIMI", "NET2", "NDIS", "NET1", "NUND", "NOGR", "PARV", "RDAV", "STOP"])]
	#[serde(rename = "TypeOfPrice1Code")]
	pub type_of_price1_code: String,
}


// UnderlyingAttributes4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct UnderlyingAttributes4 {
	#[serde(rename = "AllcnPctg")]
	pub allcn_pctg: Option<f64>,
	#[validate]
	#[serde(rename = "Qty")]
	pub qty: Option<UnitOrFaceAmount1Choice>,
	#[validate]
	#[serde(rename = "SttlmTp")]
	pub sttlm_tp: Option<SettlementType3Choice>,
	#[validate]
	#[serde(rename = "CshAmt")]
	pub csh_amt: Option<ActiveCurrencyAndAmount>,
	#[serde(rename = "CshTp")]
	pub csh_tp: Option<String>,
	#[validate]
	#[serde(rename = "Pric")]
	pub pric: Option<Price8>,
	#[validate]
	#[serde(rename = "DrtyPric")]
	pub drty_pric: Option<Price8>,
	#[validate]
	#[serde(rename = "EndPric")]
	pub end_pric: Option<Price8>,
	#[validate]
	#[serde(rename = "StartVal")]
	pub start_val: Option<ActiveCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "CurVal")]
	pub cur_val: Option<ActiveCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "EndVal")]
	pub end_val: Option<ActiveCurrencyAndAmount>,
	#[validate]
	#[serde(rename = "AdjstdQty")]
	pub adjstd_qty: Option<UnitOrFaceAmount1Choice>,
	#[serde(rename = "XchgRate")]
	pub xchg_rate: Option<f64>,
	#[validate]
	#[serde(rename = "CapVal")]
	pub cap_val: Option<ActiveCurrencyAndAmount>,
}


// UnitOfMeasure7Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct UnitOfMeasure7Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// UnitOfMeasure9Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct UnitOfMeasure9Code {
	#[validate(enumerate = ["BAGG", "BALE", "BOTL", "BOXX", "CRTN", "CELI", "CMET", "CNTR", "CRAT", "CBIN", "CBME", "CBML", "PIEC", "FOOT", "GBFO", "GBGA", "GBPI", "GBQA", "GBTN", "GRAM", "INCH", "KILO", "KMET", "LITR", "METR", "TONE", "MILE", "MMET", "MILI", "PUND", "USOU", "SCMT", "SQFO", "SQIN", "SQKI", "SMET", "SQMI", "SMIL", "SQYA", "USBA", "USFO", "USGA", "USPI", "USQA", "USTN", "YARD", "GBOU", "ACRE", "ARES", "HECT"])]
	#[serde(rename = "UnitOfMeasure9Code")]
	pub unit_of_measure9_code: String,
}


// UnitOrFaceAmount1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct UnitOrFaceAmount1Choice {
	#[serde(rename = "Unit")]
	pub unit: Option<f64>,
	#[validate]
	#[serde(rename = "FaceAmt")]
	pub face_amt: Option<ActiveCurrencyAndAmount>,
}


// Warrant4 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Warrant4 {
	#[serde(rename = "Mltplr")]
	pub mltplr: Option<f64>,
	#[validate]
	#[serde(rename = "SbcptPric")]
	pub sbcpt_pric: Option<Price8>,
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: Option<WarrantStyle3Choice>,
	#[validate]
	#[serde(rename = "WarrtAgt")]
	pub warrt_agt: Option<Vec<Organisation38>>,
}


// WarrantStyle1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct WarrantStyle1Code {
	#[validate(enumerate = ["AMER", "EURO", "BERM"])]
	#[serde(rename = "WarrantStyle1Code")]
	pub warrant_style1_code: String,
}


// WarrantStyle3Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct WarrantStyle3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[validate]
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification30>,
}


// YesNoIndicator ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct YesNoIndicator {
	#[serde(rename = "YesNoIndicator")]
	pub yes_no_indicator: bool,
}


// YieldCalculation6 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct YieldCalculation6 {
	#[serde(rename = "Val")]
	pub val: f64,
	#[validate]
	#[serde(rename = "ClctnTp")]
	pub clctn_tp: Option<CalculationType3Choice>,
	#[validate]
	#[serde(rename = "RedPric")]
	pub red_pric: Option<Price8>,
	#[serde(rename = "ValDt")]
	pub val_dt: String,
	#[validate]
	#[serde(rename = "ValPrd")]
	pub val_prd: DateTimePeriod1Choice,
	#[serde(rename = "ClctnDt")]
	pub clctn_dt: String,
}
