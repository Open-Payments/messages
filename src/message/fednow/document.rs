use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

use crate::message::fednow::iso::admi_002_001_01::*;
use crate::message::fednow::iso::admi_004_001_02::*;
use crate::message::fednow::iso::admi_006_001_01::*;
use crate::message::fednow::iso::admi_007_001_01::*;
use crate::message::fednow::iso::admi_011_001_01::*;
use crate::message::fednow::iso::admi_998_001_02::*;

use crate::message::fednow::iso::camt_026_001_07::*;
use crate::message::fednow::iso::camt_028_001_09::*;
use crate::message::fednow::iso::camt_029_001_09::*;
use crate::message::fednow::iso::camt_052_001_08::*;
use crate::message::fednow::iso::camt_054_001_08::*;
use crate::message::fednow::iso::camt_055_001_09::*;
use crate::message::fednow::iso::camt_056_001_08::*;
use crate::message::fednow::iso::camt_060_001_05::*;

use crate::message::fednow::iso::pacs_002_001_10::*;
use crate::message::fednow::iso::pacs_004_001_10::*;
use crate::message::fednow::iso::pacs_008_001_08::*;
use crate::message::fednow::iso::pacs_009_001_08::*;
use crate::message::fednow::iso::pacs_028_001_03::*;
use crate::message::fednow::iso::pain_013_001_07::*;
use crate::message::fednow::iso::pain_014_001_07::*;


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Document {
	#[serde(rename = "admi.002.001.01")]
	pub admi00200101: Box<Option<Admi00200101>>,

	#[serde(rename = "SysEvtNtfctn")]
	pub sys_evt_ntfctn: Box<Option<SystemEventNotificationV02>>,

	#[serde(rename = "RsndReq")]
	pub rsnd_req: Box<Option<ResendRequestV01>>,

	#[serde(rename = "RctAck")]
	pub rct_ack: Box<Option<ReceiptAcknowledgementV01>>,

	#[serde(rename = "FIToFIPmtStsRpt")]
	pub fi_to_fi_pmt_sts_rpt: Box<Option<FIToFIPaymentStatusReportV10>>,

	#[serde(rename = "PmtRtr")]
	pub pmt_rtr: Box<Option<PaymentReturnV10>>,

	#[serde(rename = "FIToFICstmrCdtTrf")]
	pub fi_to_fi_cstmr_cdt_trf: Box<Option<FIToFICustomerCreditTransferV08>>,

	#[serde(rename = "FICdtTrf")]
	pub fi_cdt_trf: Box<Option<FinancialInstitutionCreditTransferV08>>,

	#[serde(rename = "FIToFIPmtStsReq")]
	pub fi_to_fi_pmt_sts_req: Box<Option<FIToFIPaymentStatusRequestV03>>,

	#[serde(rename = "CdtrPmtActvtnReq")]
	pub cdtr_pmt_actvtn_req: Box<Option<CreditorPaymentActivationRequestV07>>,

	#[serde(rename = "CdtrPmtActvtnReqStsRpt")]
	pub cdtr_pmt_actvtn_req_sts_rpt: Box<Option<CreditorPaymentActivationRequestStatusReportV07>>,

	#[serde(rename = "UblToApply")]
	pub ubl_to_apply: Box<Option<UnableToApplyV07>>,

	#[serde(rename = "AddtlPmtInf")]
	pub addtl_pmt_inf: Box<Option<AdditionalPaymentInformationV09>>,

	#[serde(rename = "RsltnOfInvstgtn")]
	pub rsltn_of_invstgtn: Box<Option<ResolutionOfInvestigationV09>>,

	#[serde(rename = "CstmrPmtCxlReq")]
	pub cstmr_pmt_cxl_req: Box<Option<CustomerPaymentCancellationRequestV09>>,

	#[serde(rename = "FIToFIPmtCxlReq")]
	pub fi_to_fi_pmt_cxl_req: Box<Option<FIToFIPaymentCancellationRequestV08>>,

	#[serde(rename = "AcctRptgReq")]
	pub acct_rptg_req: Box<Option<AccountReportingRequestV05>>,

	#[serde(rename = "SysEvtAck")]
	pub sys_evt_ack: Box<Option<SystemEventAcknowledgementV01>>,

	#[serde(rename = "AdmstnPrtryMsg")]
	pub admstn_prtry_msg: Box<Option<AdministrationProprietaryMessageV02>>,

	#[serde(rename = "BkToCstmrAcctRpt")]
	pub bk_to_cstmr_acct_rpt: Box<Option<BankToCustomerAccountReportV08>>,

	#[serde(rename = "BkToCstmrDbtCdtNtfctn")]
	pub bk_to_cstmr_dbt_cdt_ntfctn: Box<Option<BankToCustomerDebitCreditNotificationV08>>,
}

