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

#[cfg(feature = "derive_serde")]
use serde::{Deserialize, Serialize};
use open_payments_common::ValidationError;

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


#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
pub enum Document {
	#[cfg_attr( feature = "derive_serde", serde(rename = "admi.002.001.01"))]
	Admi00200101(Box<Admi00200101>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SysEvtNtfctn"))]
	SystemEventNotificationV02(Box<SystemEventNotificationV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "RsndReq"))]
	ResendRequestV01(Box<ResendRequestV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "RctAck"))]
	ReceiptAcknowledgementV01(Box<ReceiptAcknowledgementV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FIToFIPmtStsRpt"))]
	FIToFIPaymentStatusReportV10(Box<FIToFIPaymentStatusReportV10>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtRtr"))]
	PaymentReturnV10(Box<PaymentReturnV10>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FIToFICstmrCdtTrf"))]
	FIToFICustomerCreditTransferV08(Box<FIToFICustomerCreditTransferV08>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FICdtTrf"))]
	FinancialInstitutionCreditTransferV08(Box<FinancialInstitutionCreditTransferV08>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FIToFIPmtStsReq"))]
	FIToFIPaymentStatusRequestV03(Box<FIToFIPaymentStatusRequestV03>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrPmtActvtnReq"))]
	CreditorPaymentActivationRequestV07(Box<CreditorPaymentActivationRequestV07>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrPmtActvtnReqStsRpt"))]
	CreditorPaymentActivationRequestStatusReportV07(Box<CreditorPaymentActivationRequestStatusReportV07>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "UblToApply"))]
	UnableToApplyV07(Box<UnableToApplyV07>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlPmtInf"))]
	AdditionalPaymentInformationV09(Box<AdditionalPaymentInformationV09>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "RsltnOfInvstgtn"))]
	ResolutionOfInvestigationV09(Box<ResolutionOfInvestigationV09>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CstmrPmtCxlReq"))]
	CustomerPaymentCancellationRequestV09(Box<CustomerPaymentCancellationRequestV09>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FIToFIPmtCxlReq"))]
	FIToFIPaymentCancellationRequestV08(Box<FIToFIPaymentCancellationRequestV08>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctRptgReq"))]
	AccountReportingRequestV05(Box<AccountReportingRequestV05>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SysEvtAck"))]
	SystemEventAcknowledgementV01(Box<SystemEventAcknowledgementV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AdmstnPrtryMsg"))]
	AdministrationProprietaryMessageV02(Box<AdministrationProprietaryMessageV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "BkToCstmrAcctRpt"))]
	BankToCustomerAccountReportV08(Box<BankToCustomerAccountReportV08>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "BkToCstmrDbtCdtNtfctn"))]
	BankToCustomerDebitCreditNotificationV08(Box<BankToCustomerDebitCreditNotificationV08>),

	#[cfg_attr(feature = "derive_default", default)]
	UNKNOWN
}

impl Document {
    pub fn validate(&self) -> Result<(), ValidationError> {
        match self {
            Document::Admi00200101(ref value) => value.validate(),
            Document::SystemEventNotificationV02(ref value) => value.validate(),
            Document::ResendRequestV01(ref value) => value.validate(),
            Document::ReceiptAcknowledgementV01(ref value) => value.validate(),
            Document::FIToFIPaymentStatusReportV10(ref value) => value.validate(),
            Document::PaymentReturnV10(ref value) => value.validate(),
            Document::FIToFICustomerCreditTransferV08(ref value) => value.validate(),
            Document::FinancialInstitutionCreditTransferV08(ref value) => value.validate(),
            Document::FIToFIPaymentStatusRequestV03(ref value) => value.validate(),
            Document::CreditorPaymentActivationRequestV07(ref value) => value.validate(),
            Document::CreditorPaymentActivationRequestStatusReportV07(ref value) => value.validate(),
            Document::UnableToApplyV07(ref value) => value.validate(),
            Document::AdditionalPaymentInformationV09(ref value) => value.validate(),
            Document::ResolutionOfInvestigationV09(ref value) => value.validate(),
            Document::CustomerPaymentCancellationRequestV09(ref value) => value.validate(),
            Document::FIToFIPaymentCancellationRequestV08(ref value) => value.validate(),
            Document::AccountReportingRequestV05(ref value) => value.validate(),
            Document::SystemEventAcknowledgementV01(ref value) => value.validate(),
            Document::AdministrationProprietaryMessageV02(ref value) => value.validate(),
            Document::BankToCustomerAccountReportV08(ref value) => value.validate(),
            Document::BankToCustomerDebitCreditNotificationV08(ref value) => value.validate(),
            Document::UNKNOWN => {
                // Return an error for the UNKNOWN case
                Err(ValidationError::new(9999, "Unknown document type".to_string()))
            }
        }
    }
}