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
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<ExternalAgreementType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max50Text>,
}


// AnyBICDec2014Identifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICDec2014Identifier {
	#[serde(rename = "$value")]
	pub any_bic_dec2014_identifier: String,
}


// DetailedReportStatistics5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DetailedReportStatistics5 {
	#[serde(rename = "TtlNbOfRpts")]
	pub ttl_nb_of_rpts: Max15NumericText,
	#[serde(rename = "TtlNbOfRptsAccptd")]
	pub ttl_nb_of_rpts_accptd: Max15NumericText,
	#[serde(rename = "TtlNbOfRptsRjctd")]
	pub ttl_nb_of_rpts_rjctd: Max15NumericText,
	#[serde(rename = "NbOfRptsRjctdPerErr", skip_serializing_if = "Option::is_none")]
	pub nb_of_rpts_rjctd_per_err: Option<Vec<NumberOfTransactionsPerValidationRule5>>,
}


// DetailedTransactionStatistics13 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DetailedTransactionStatistics13 {
	#[serde(rename = "TtlNbOfTxs")]
	pub ttl_nb_of_txs: Max15NumericText,
	#[serde(rename = "TtlNbOfTxsAccptd")]
	pub ttl_nb_of_txs_accptd: Max15NumericText,
	#[serde(rename = "TtlNbOfTxsRjctd")]
	pub ttl_nb_of_txs_rjctd: Max15NumericText,
	#[serde(rename = "TxsRjctnsRsn", skip_serializing_if = "Option::is_none")]
	pub txs_rjctns_rsn: Option<Vec<RejectionReason53>>,
}


// DetailedTransactionStatistics2Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DetailedTransactionStatistics2Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[serde(rename = "DtldSttstcs", skip_serializing_if = "Option::is_none")]
	pub dtld_sttstcs: Option<DetailedTransactionStatistics13>,
}


// ExternalAgreementType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalAgreementType1Code {
	#[serde(rename = "$value")]
	pub external_agreement_type1_code: String,
}


// ExternalValidationRuleIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalValidationRuleIdentification1Code {
	#[serde(rename = "$value")]
	pub external_validation_rule_identification1_code: String,
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


// GenericValidationRuleIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericValidationRuleIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "Desc", skip_serializing_if = "Option::is_none")]
	pub desc: Option<Max350Text>,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<ValidationRuleSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}


// LEIIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LEIIdentifier {
	#[serde(rename = "$value")]
	pub lei_identifier: String,
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


// NumberOfTransactionsPerValidationRule5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NumberOfTransactionsPerValidationRule5 {
	#[serde(rename = "DtldNb")]
	pub dtld_nb: Max15NumericText,
	#[serde(rename = "RptSts")]
	pub rpt_sts: Vec<RejectionReason45>,
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


// PartyIdentification236Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification236Choice {
	#[serde(rename = "Lgl", skip_serializing_if = "Option::is_none")]
	pub lgl: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "Ntrl", skip_serializing_if = "Option::is_none")]
	pub ntrl: Option<NaturalPersonIdentification2>,
}


// RejectionReason45 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RejectionReason45 {
	#[serde(rename = "MsgRptId")]
	pub msg_rpt_id: Max140Text,
	#[serde(rename = "Sts")]
	pub sts: ReportingMessageStatus1Code,
	#[serde(rename = "DtldVldtnRule", skip_serializing_if = "Option::is_none")]
	pub dtld_vldtn_rule: Option<GenericValidationRuleIdentification1>,
}


// RejectionReason53 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RejectionReason53 {
	#[serde(rename = "TxId")]
	pub tx_id: TransactionIdentification3Choice,
	#[serde(rename = "Sts")]
	pub sts: ReportingMessageStatus1Code,
	#[serde(rename = "DtldVldtnRule", skip_serializing_if = "Option::is_none")]
	pub dtld_vldtn_rule: Option<Vec<GenericValidationRuleIdentification1>>,
}


// ReportPeriodActivity1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ReportPeriodActivity1Code {
	#[default]
	#[serde(rename = "NOTX")]
	CodeNOTX,

}


// ReportingMessageStatus1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ReportingMessageStatus1Code {
	#[default]
	#[serde(rename = "ACPT")]
	CodeACPT,
	#[serde(rename = "ACTC")]
	CodeACTC,
	#[serde(rename = "PART")]
	CodePART,
	#[serde(rename = "RCVD")]
	CodeRCVD,
	#[serde(rename = "RJCT")]
	CodeRJCT,
	#[serde(rename = "RMDR")]
	CodeRMDR,
	#[serde(rename = "WARN")]
	CodeWARN,
	#[serde(rename = "INCF")]
	CodeINCF,
	#[serde(rename = "CRPT")]
	CodeCRPT,

}


// SecuritiesFinancingReportingTransactionStatusAdviceV02 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SecuritiesFinancingReportingTransactionStatusAdviceV02 {
	#[serde(rename = "TxRptStsAndRsn")]
	pub tx_rpt_sts_and_rsn: Vec<TradeData35Choice>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
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


// TradeData29 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeData29 {
	#[serde(rename = "RptSttstcs")]
	pub rpt_sttstcs: Vec<DetailedReportStatistics5>,
	#[serde(rename = "TxSttstcs")]
	pub tx_sttstcs: Vec<DetailedTransactionStatistics2Choice>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// TradeData35Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeData35Choice {
	#[serde(rename = "DataSetActn", skip_serializing_if = "Option::is_none")]
	pub data_set_actn: Option<ReportPeriodActivity1Code>,
	#[serde(rename = "Rpt", skip_serializing_if = "Option::is_none")]
	pub rpt: Option<Vec<TradeData29>>,
}


// TradeTransactionIdentification16 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeTransactionIdentification16 {
	#[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
	pub tech_rcrd_id: Option<Max140Text>,
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: OrganisationIdentification15Choice,
	#[serde(rename = "OthrCtrPty")]
	pub othr_ctr_pty: PartyIdentification236Choice,
	#[serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none")]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
	#[serde(rename = "CollPrtflId", skip_serializing_if = "Option::is_none")]
	pub coll_prtfl_id: Option<Max52Text>,
}


// TradeTransactionIdentification17 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeTransactionIdentification17 {
	#[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
	pub tech_rcrd_id: Option<Max140Text>,
	#[serde(rename = "RptgCtrPty")]
	pub rptg_ctr_pty: OrganisationIdentification15Choice,
	#[serde(rename = "RptSubmitgNtty")]
	pub rpt_submitg_ntty: OrganisationIdentification15Choice,
	#[serde(rename = "NttyRspnsblForRpt", skip_serializing_if = "Option::is_none")]
	pub ntty_rspnsbl_for_rpt: Option<OrganisationIdentification15Choice>,
}


// TradeTransactionIdentification20 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeTransactionIdentification20 {
	#[serde(rename = "TechRcrdId", skip_serializing_if = "Option::is_none")]
	pub tech_rcrd_id: Option<Max140Text>,
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


// TransactionIdentification3Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TransactionIdentification3Choice {
	#[serde(rename = "Tx", skip_serializing_if = "Option::is_none")]
	pub tx: Option<TradeTransactionIdentification20>,
	#[serde(rename = "MrgnRptg", skip_serializing_if = "Option::is_none")]
	pub mrgn_rptg: Option<TradeTransactionIdentification16>,
	#[serde(rename = "CollReuse", skip_serializing_if = "Option::is_none")]
	pub coll_reuse: Option<TradeTransactionIdentification17>,
}


// ValidationRuleSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ValidationRuleSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalValidationRuleIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}
