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


// AgreementType2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AgreementType2Choice {
	#[serde(rename = "Tp")]
	pub tp: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "AnyBICDec2014Identifier")]
	pub any_bic_dec2014_identifier: String,
}


// CollateralPortfolioCode5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CollateralPortfolioCode5Choice {
	#[serde(rename = "Prtfl")]
	pub prtfl: Option<PortfolioCode3Choice>,
	#[serde(rename = "MrgnPrtflCd")]
	pub mrgn_prtfl_cd: Option<MarginPortfolio3>,
}


// CounterpartyData92 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CounterpartyData92 {
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "RptSubmitgNtty")]
	pub rpt_submitg_ntty: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "NttyRspnsblForRpt")]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// DateAndDateTime2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndDateTime2Choice {
	#[serde(rename = "Dt")]
	pub dt: Option<String>,
	#[serde(rename = "DtTm")]
	pub dt_tm: Option<String>,
}


// DerivativeEventType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativeEventType3Code {
	#[serde(rename = "DerivativeEventType3Code")]
	pub derivative_event_type3_code: String,
}


// DerivativesTradeRejectionStatisticalReportV04 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DerivativesTradeRejectionStatisticalReportV04 {
	#[serde(rename = "RjctnSttstcs")]
	pub rjctn_sttstcs: StatisticsPerCounterparty18Choice,
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// DetailedReportStatistics7 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DetailedReportStatistics7 {
	#[serde(rename = "TtlNbOfRpts")]
	pub ttl_nb_of_rpts: f64,
	#[serde(rename = "TtlNbOfRptsAccptd")]
	pub ttl_nb_of_rpts_accptd: f64,
	#[serde(rename = "TtlNbOfRptsRjctd")]
	pub ttl_nb_of_rpts_rjctd: f64,
	#[serde(rename = "NbOfRptsRjctdPerErr")]
	pub nb_of_rpts_rjctd_per_err: Option<Vec<NumberOfTransactionsPerValidationRule6>>,
}


// DetailedStatisticsPerCounterparty19 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DetailedStatisticsPerCounterparty19 {
	#[serde(rename = "RefDt")]
	pub ref_dt: String,
	#[serde(rename = "TtlNbOfRpts")]
	pub ttl_nb_of_rpts: f64,
	#[serde(rename = "TtlNbOfRptsAccptd")]
	pub ttl_nb_of_rpts_accptd: f64,
	#[serde(rename = "TtlNbOfRptsRjctd")]
	pub ttl_nb_of_rpts_rjctd: f64,
	#[serde(rename = "TtlNbOfTxs")]
	pub ttl_nb_of_txs: f64,
	#[serde(rename = "TtlNbOfTxsAccptd")]
	pub ttl_nb_of_txs_accptd: f64,
	#[serde(rename = "TtlNbOfTxsRjctd")]
	pub ttl_nb_of_txs_rjctd: f64,
	#[serde(rename = "TtlCrrctdRjctns")]
	pub ttl_crrctd_rjctns: Option<f64>,
	#[serde(rename = "RjctnSttstcs")]
	pub rjctn_sttstcs: Vec<RejectionStatistics9>,
}


// DetailedTransactionStatistics30 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DetailedTransactionStatistics30 {
	#[serde(rename = "TtlNbOfTxs")]
	pub ttl_nb_of_txs: f64,
	#[serde(rename = "TtlNbOfTxsAccptd")]
	pub ttl_nb_of_txs_accptd: f64,
	#[serde(rename = "TtlNbOfTxsRjctd")]
	pub ttl_nb_of_txs_rjctd: f64,
	#[serde(rename = "TtlCrrctdRjctns")]
	pub ttl_crrctd_rjctns: Option<f64>,
	#[serde(rename = "TxsRjctnsRsn")]
	pub txs_rjctns_rsn: Option<Vec<RejectionReason71>>,
}


// DetailedTransactionStatistics7Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DetailedTransactionStatistics7Choice {
	#[serde(rename = "DataSetActn")]
	pub data_set_actn: Option<String>,
	#[serde(rename = "DtldSttstcs")]
	pub dtld_sttstcs: Option<DetailedTransactionStatistics30>,
}


// ExternalAgreementType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAgreementType1Code {
	#[serde(rename = "ExternalAgreementType1Code")]
	pub external_agreement_type1_code: String,
}


// ExternalValidationRuleIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalValidationRuleIdentification1Code {
	#[serde(rename = "ExternalValidationRuleIdentification1Code")]
	pub external_validation_rule_identification1_code: String,
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


// GenericValidationRuleIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericValidationRuleIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "Desc")]
	pub desc: Option<String>,
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<ValidationRuleSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
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


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "LEIIdentifier")]
	pub lei_identifier: String,
}


// LegalPersonIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LegalPersonIdentification1 {
	#[serde(rename = "Id")]
	pub id: OrganisationIdentification15Choice,
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
}


// MarginPortfolio3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MarginPortfolio3 {
	#[serde(rename = "InitlMrgnPrtflCd")]
	pub initl_mrgn_prtfl_cd: PortfolioCode5Choice,
	#[serde(rename = "VartnMrgnPrtflCd")]
	pub vartn_mrgn_prtfl_cd: Option<PortfolioCode5Choice>,
}


// MasterAgreement8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MasterAgreement8 {
	#[serde(rename = "Tp")]
	pub tp: Option<AgreementType2Choice>,
	#[serde(rename = "Vrsn")]
	pub vrsn: Option<String>,
	#[serde(rename = "OthrMstrAgrmtDtls")]
	pub othr_mstr_agrmt_dtls: Option<String>,
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


// Max20PositiveNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max20PositiveNumber {
	#[serde(rename = "Max20PositiveNumber")]
	pub max20_positive_number: f64,
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


// Max72Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max72Text {
	#[serde(rename = "Max72Text")]
	pub max72_text: String,
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


// NaturalPersonIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NaturalPersonIdentification3 {
	#[serde(rename = "Id")]
	pub id: NaturalPersonIdentification2,
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
}


// NotApplicable1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NotApplicable1Code {
	#[serde(rename = "NotApplicable1Code")]
	pub not_applicable1_code: String,
}


// NumberOfTransactionsPerValidationRule6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NumberOfTransactionsPerValidationRule6 {
	#[serde(rename = "DtldNb")]
	pub dtld_nb: String,
	#[serde(rename = "RptSts")]
	pub rpt_sts: Vec<RejectionReason70>,
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


// PartyIdentification248Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification248Choice {
	#[serde(rename = "Lgl")]
	pub lgl: Option<LegalPersonIdentification1>,
	#[serde(rename = "Ntrl")]
	pub ntrl: Option<NaturalPersonIdentification3>,
}


// PortfolioCode3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioCode3Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "NoPrtfl")]
	pub no_prtfl: Option<String>,
}


// PortfolioCode5Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioCode5Choice {
	#[serde(rename = "Prtfl")]
	pub prtfl: Option<PortfolioIdentification3>,
	#[serde(rename = "NoPrtfl")]
	pub no_prtfl: Option<String>,
}


// PortfolioIdentification3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PortfolioIdentification3 {
	#[serde(rename = "Cd")]
	pub cd: String,
	#[serde(rename = "PrtflTxXmptn")]
	pub prtfl_tx_xmptn: Option<bool>,
}


// RejectionReason70 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RejectionReason70 {
	#[serde(rename = "MsgRptId")]
	pub msg_rpt_id: String,
	#[serde(rename = "Sts")]
	pub sts: String,
	#[serde(rename = "DtldVldtnRule")]
	pub dtld_vldtn_rule: Option<GenericValidationRuleIdentification1>,
}


// RejectionReason71 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RejectionReason71 {
	#[serde(rename = "TxId")]
	pub tx_id: TradeTransactionIdentification24,
	#[serde(rename = "Sts")]
	pub sts: String,
	#[serde(rename = "DtldVldtnRule")]
	pub dtld_vldtn_rule: Option<Vec<GenericValidationRuleIdentification1>>,
}


// RejectionStatistics9 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RejectionStatistics9 {
	#[serde(rename = "CtrPtyId")]
	pub ctr_pty_id: CounterpartyData92,
	#[serde(rename = "RptSttstcs")]
	pub rpt_sttstcs: DetailedReportStatistics7,
	#[serde(rename = "DerivSttstcs")]
	pub deriv_sttstcs: DetailedTransactionStatistics7Choice,
}


// ReportPeriodActivity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportPeriodActivity1Code {
	#[serde(rename = "ReportPeriodActivity1Code")]
	pub report_period_activity1_code: String,
}


// ReportingMessageStatus2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ReportingMessageStatus2Code {
	#[serde(rename = "ReportingMessageStatus2Code")]
	pub reporting_message_status2_code: String,
}


// StatisticsPerCounterparty18Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct StatisticsPerCounterparty18Choice {
	#[serde(rename = "DataSetActn")]
	pub data_set_actn: Option<String>,
	#[serde(rename = "Rpt")]
	pub rpt: Option<DetailedStatisticsPerCounterparty19>,
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


// TradeTransactionIdentification24 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeTransactionIdentification24 {
	#[serde(rename = "TechRcrdId")]
	pub tech_rcrd_id: Option<String>,
	#[serde(rename = "ActnTp")]
	pub actn_tp: Option<String>,
	#[serde(rename = "RptgTmStmp")]
	pub rptg_tm_stmp: Option<String>,
	#[serde(rename = "DerivEvtTp")]
	pub deriv_evt_tp: Option<String>,
	#[serde(rename = "DerivEvtTmStmp")]
	pub deriv_evt_tm_stmp: Option<DateAndDateTime2Choice>,
	#[serde(rename = "OthrCtrPty")]
	pub othr_ctr_pty: Option<PartyIdentification248Choice>,
	#[serde(rename = "UnqIdr")]
	pub unq_idr: Option<UniqueTransactionIdentifier2Choice>,
	#[serde(rename = "MstrAgrmt")]
	pub mstr_agrmt: Option<MasterAgreement8>,
	#[serde(rename = "CollPrtflCd")]
	pub coll_prtfl_cd: Option<CollateralPortfolioCode5Choice>,
}


// TransactionOperationType10Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionOperationType10Code {
	#[serde(rename = "TransactionOperationType10Code")]
	pub transaction_operation_type10_code: String,
}


// TrueFalseIndicator ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TrueFalseIndicator {
	#[serde(rename = "TrueFalseIndicator")]
	pub true_false_indicator: bool,
}


// UTIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UTIIdentifier {
	#[serde(rename = "UTIIdentifier")]
	pub uti_identifier: String,
}


// UniqueTransactionIdentifier2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct UniqueTransactionIdentifier2Choice {
	#[serde(rename = "UnqTxIdr")]
	pub unq_tx_idr: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<GenericIdentification175>,
}


// ValidationRuleSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ValidationRuleSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}
