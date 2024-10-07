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


#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub enum Document {
	#[validate]
	#[serde(rename = "admi.002.001.01")]
	Admi00200101(Box<Admi00200101>),

	#[validate]
	#[serde(rename = "SysEvtNtfctn")]
	SystemEventNotificationV02(Box<SystemEventNotificationV02>),

	#[validate]
	#[serde(rename = "RsndReq")]
	ResendRequestV01(Box<ResendRequestV01>),

	#[validate]
	#[serde(rename = "RctAck")]
	ReceiptAcknowledgementV01(Box<ReceiptAcknowledgementV01>),

	#[validate]
	#[serde(rename = "FIToFIPmtStsRpt")]
	FIToFIPaymentStatusReportV10(Box<FIToFIPaymentStatusReportV10>),

	#[validate]
	#[serde(rename = "PmtRtr")]
	PaymentReturnV10(Box<PaymentReturnV10>),

	#[validate]
	#[serde(rename = "FIToFICstmrCdtTrf")]
	FIToFICustomerCreditTransferV08(Box<FIToFICustomerCreditTransferV08>),

	#[validate]
	#[serde(rename = "FICdtTrf")]
	FinancialInstitutionCreditTransferV08(Box<FinancialInstitutionCreditTransferV08>),

	#[validate]
	#[serde(rename = "FIToFIPmtStsReq")]
	FIToFIPaymentStatusRequestV03(Box<FIToFIPaymentStatusRequestV03>),

	#[validate]
	#[serde(rename = "CdtrPmtActvtnReq")]
	CreditorPaymentActivationRequestV07(Box<CreditorPaymentActivationRequestV07>),

	#[validate]
	#[serde(rename = "CdtrPmtActvtnReqStsRpt")]
	CreditorPaymentActivationRequestStatusReportV07(Box<CreditorPaymentActivationRequestStatusReportV07>),

	#[validate]
	#[serde(rename = "UblToApply")]
	UnableToApplyV07(Box<UnableToApplyV07>),

	#[validate]
	#[serde(rename = "AddtlPmtInf")]
	AdditionalPaymentInformationV09(Box<AdditionalPaymentInformationV09>),

	#[validate]
	#[serde(rename = "RsltnOfInvstgtn")]
	ResolutionOfInvestigationV09(Box<ResolutionOfInvestigationV09>),

	#[validate]
	#[serde(rename = "CstmrPmtCxlReq")]
	CustomerPaymentCancellationRequestV09(Box<CustomerPaymentCancellationRequestV09>),

	#[validate]
	#[serde(rename = "FIToFIPmtCxlReq")]
	FIToFIPaymentCancellationRequestV08(Box<FIToFIPaymentCancellationRequestV08>),

	#[validate]
	#[serde(rename = "AcctRptgReq")]
	AccountReportingRequestV05(Box<AccountReportingRequestV05>),

	#[validate]
	#[serde(rename = "SysEvtAck")]
	SystemEventAcknowledgementV01(Box<SystemEventAcknowledgementV01>),

	#[validate]
	#[serde(rename = "AdmstnPrtryMsg")]
	AdministrationProprietaryMessageV02(Box<AdministrationProprietaryMessageV02>),

	#[validate]
	#[serde(rename = "BkToCstmrAcctRpt")]
	BankToCustomerAccountReportV08(Box<BankToCustomerAccountReportV08>),

	#[validate]
	#[serde(rename = "BkToCstmrDbtCdtNtfctn")]
	BankToCustomerDebitCreditNotificationV08(Box<BankToCustomerDebitCreditNotificationV08>),
}
