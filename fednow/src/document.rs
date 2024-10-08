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

use crate::iso::admi_002_001_01::*;
use crate::iso::admi_004_001_02::*;
use crate::iso::admi_006_001_01::*;
use crate::iso::admi_007_001_01::*;
use crate::iso::admi_011_001_01::*;
use crate::iso::admi_998_001_02::*;

use crate::iso::camt_026_001_07::*;
use crate::iso::camt_028_001_09::*;
use crate::iso::camt_029_001_09::*;
use crate::iso::camt_052_001_08::*;
use crate::iso::camt_054_001_08::*;
use crate::iso::camt_055_001_09::*;
use crate::iso::camt_056_001_08::*;
use crate::iso::camt_060_001_05::*;

use crate::iso::pacs_002_001_10::*;
use crate::iso::pacs_004_001_10::*;
use crate::iso::pacs_008_001_08::*;
use crate::iso::pacs_009_001_08::*;
use crate::iso::pacs_028_001_03::*;
use crate::iso::pain_013_001_07::*;
use crate::iso::pain_014_001_07::*;


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Document {
	#[serde(rename = "admi.002.001.01")]
	Admi00200101(Box<Admi00200101>),

	#[serde(rename = "SysEvtNtfctn")]
	SystemEventNotificationV02(Box<SystemEventNotificationV02>),

	#[serde(rename = "RsndReq")]
	ResendRequestV01(Box<ResendRequestV01>),

	#[serde(rename = "RctAck")]
	ReceiptAcknowledgementV01(Box<ReceiptAcknowledgementV01>),

	#[serde(rename = "FIToFIPmtStsRpt")]
	FIToFIPaymentStatusReportV10(Box<FIToFIPaymentStatusReportV10>),

	#[serde(rename = "PmtRtr")]
	PaymentReturnV10(Box<PaymentReturnV10>),

	#[serde(rename = "FIToFICstmrCdtTrf")]
	FIToFICustomerCreditTransferV08(Box<FIToFICustomerCreditTransferV08>),

	#[serde(rename = "FICdtTrf")]
	FinancialInstitutionCreditTransferV08(Box<FinancialInstitutionCreditTransferV08>),

	#[serde(rename = "FIToFIPmtStsReq")]
	FIToFIPaymentStatusRequestV03(Box<FIToFIPaymentStatusRequestV03>),

	#[serde(rename = "CdtrPmtActvtnReq")]
	CreditorPaymentActivationRequestV07(Box<CreditorPaymentActivationRequestV07>),

	#[serde(rename = "CdtrPmtActvtnReqStsRpt")]
	CreditorPaymentActivationRequestStatusReportV07(Box<CreditorPaymentActivationRequestStatusReportV07>),

	#[serde(rename = "UblToApply")]
	UnableToApplyV07(Box<UnableToApplyV07>),

	#[serde(rename = "AddtlPmtInf")]
	AdditionalPaymentInformationV09(Box<AdditionalPaymentInformationV09>),

	#[serde(rename = "RsltnOfInvstgtn")]
	ResolutionOfInvestigationV09(Box<ResolutionOfInvestigationV09>),

	#[serde(rename = "CstmrPmtCxlReq")]
	CustomerPaymentCancellationRequestV09(Box<CustomerPaymentCancellationRequestV09>),

	#[serde(rename = "FIToFIPmtCxlReq")]
	FIToFIPaymentCancellationRequestV08(Box<FIToFIPaymentCancellationRequestV08>),

	#[serde(rename = "AcctRptgReq")]
	AccountReportingRequestV05(Box<AccountReportingRequestV05>),

	#[serde(rename = "SysEvtAck")]
	SystemEventAcknowledgementV01(Box<SystemEventAcknowledgementV01>),

	#[serde(rename = "AdmstnPrtryMsg")]
	AdministrationProprietaryMessageV02(Box<AdministrationProprietaryMessageV02>),

	#[serde(rename = "BkToCstmrAcctRpt")]
	BankToCustomerAccountReportV08(Box<BankToCustomerAccountReportV08>),

	#[serde(rename = "BkToCstmrDbtCdtNtfctn")]
	BankToCustomerDebitCreditNotificationV08(Box<BankToCustomerDebitCreditNotificationV08>),
}
