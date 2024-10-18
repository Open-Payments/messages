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
use regex::Regex;
use crate::document::*;
use crate::fednow::key_exchange::*;
use crate::iso::head_001_001_02::BusinessApplicationHeaderV02;


// FedNowOutgoing ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowOutgoing {
	#[serde(rename = "FedNowTechnicalHeader", skip_serializing_if = "Option::is_none")]
	pub fed_now_technical_header: Option<FedNowTechnicalHeader>,
	#[serde(rename = "FedNowOutgoingMessage")]
	pub fed_now_outgoing_message: FedNowOutgoingMessage,
}

impl FedNowOutgoing {
	pub fn validate(&self) -> bool {
		if let Some(ref fed_now_technical_header_value) = self.fed_now_technical_header { if !fed_now_technical_header_value.validate() { return false; } }
		if !self.fed_now_outgoing_message.validate() { return false }
		return true
	}
}


// FedNowTechnicalHeader ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowTechnicalHeader {
}

impl FedNowTechnicalHeader {
	pub fn validate(&self) -> bool {
		return true
	}
}


// FedNowOutgoingMessage ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowOutgoingMessage {
	#[serde(rename = "FedNowMessageReject", skip_serializing_if = "Option::is_none")]
	pub fed_now_message_reject: Option<FedNowMessageReject>,
	#[serde(rename = "FedNowBroadcast", skip_serializing_if = "Option::is_none")]
	pub fed_now_broadcast: Option<FedNowBroadcast>,
	#[serde(rename = "FedNowReceiptAcknowledgement", skip_serializing_if = "Option::is_none")]
	pub fed_now_receipt_acknowledgement: Option<FedNowReceiptAcknowledgement>,
	#[serde(rename = "FedNowSystemResponse", skip_serializing_if = "Option::is_none")]
	pub fed_now_system_response: Option<FedNowSystemResponse>,
	#[serde(rename = "FedNowParticipantFile", skip_serializing_if = "Option::is_none")]
	pub fed_now_participant_file: Option<FedNowParticipantFile>,
	#[serde(rename = "FedNowPaymentStatus", skip_serializing_if = "Option::is_none")]
	pub fed_now_payment_status: Option<FedNowPaymentStatus>,
	#[serde(rename = "FedNowPaymentReturn", skip_serializing_if = "Option::is_none")]
	pub fed_now_payment_return: Option<FedNowPaymentReturn>,
	#[serde(rename = "FedNowCustomerCreditTransfer", skip_serializing_if = "Option::is_none")]
	pub fed_now_customer_credit_transfer: Option<FedNowCustomerCreditTransfer>,
	#[serde(rename = "FedNowInstitutionCreditTransfer", skip_serializing_if = "Option::is_none")]
	pub fed_now_institution_credit_transfer: Option<FedNowInstitutionCreditTransfer>,
	#[serde(rename = "FedNowPaymentStatusRequest", skip_serializing_if = "Option::is_none")]
	pub fed_now_payment_status_request: Option<FedNowPaymentStatusRequest>,
	#[serde(rename = "FedNowRequestForPayment", skip_serializing_if = "Option::is_none")]
	pub fed_now_request_for_payment: Option<FedNowRequestForPayment>,
	#[serde(rename = "FedNowRequestForPaymentResponse", skip_serializing_if = "Option::is_none")]
	pub fed_now_request_for_payment_response: Option<FedNowRequestForPaymentResponse>,
	#[serde(rename = "FedNowInformationRequest", skip_serializing_if = "Option::is_none")]
	pub fed_now_information_request: Option<FedNowInformationRequest>,
	#[serde(rename = "FedNowAdditionalPaymentInformation", skip_serializing_if = "Option::is_none")]
	pub fed_now_additional_payment_information: Option<FedNowAdditionalPaymentInformation>,
	#[serde(rename = "FedNowReturnRequestResponse", skip_serializing_if = "Option::is_none")]
	pub fed_now_return_request_response: Option<FedNowReturnRequestResponse>,
	#[serde(rename = "FedNowInformationRequestResponse", skip_serializing_if = "Option::is_none")]
	pub fed_now_information_request_response: Option<FedNowInformationRequestResponse>,
	#[serde(rename = "FedNowAccountActivityDetailsReport", skip_serializing_if = "Option::is_none")]
	pub fed_now_account_activity_details_report: Option<FedNowAccountActivityDetailsReport>,
	#[serde(rename = "FedNowAccountActivityTotalsReport", skip_serializing_if = "Option::is_none")]
	pub fed_now_account_activity_totals_report: Option<FedNowAccountActivityTotalsReport>,
	#[serde(rename = "FedNowAccountBalanceReport", skip_serializing_if = "Option::is_none")]
	pub fed_now_account_balance_report: Option<FedNowAccountBalanceReport>,
	#[serde(rename = "AccountDebitCreditNotification", skip_serializing_if = "Option::is_none")]
	pub account_debit_credit_notification: Option<AccountDebitCreditNotification>,
	#[serde(rename = "FedNowRequestForPaymentCancellationRequest", skip_serializing_if = "Option::is_none")]
	pub fed_now_request_for_payment_cancellation_request: Option<FedNowRequestForPaymentCancellationRequest>,
	#[serde(rename = "FedNowRequestForPaymentCancellationRequestResponse", skip_serializing_if = "Option::is_none")]
	pub fed_now_request_for_payment_cancellation_request_response: Option<FedNowRequestForPaymentCancellationRequestResponse>,
	#[serde(rename = "FedNowReturnRequest", skip_serializing_if = "Option::is_none")]
	pub fed_now_return_request: Option<FedNowReturnRequest>,
	#[serde(rename = "FedNowOutgoingMessageSignatureManagement", skip_serializing_if = "Option::is_none")]
	pub fed_now_outgoing_message_signature_management: Option<FedNowOutgoingMessageSignatureManagement>,
}

impl FedNowOutgoingMessage {
	pub fn validate(&self) -> bool {
		if let Some(ref fed_now_message_reject_value) = self.fed_now_message_reject { if !fed_now_message_reject_value.validate() { return false; } }
		if let Some(ref fed_now_broadcast_value) = self.fed_now_broadcast { if !fed_now_broadcast_value.validate() { return false; } }
		if let Some(ref fed_now_receipt_acknowledgement_value) = self.fed_now_receipt_acknowledgement { if !fed_now_receipt_acknowledgement_value.validate() { return false; } }
		if let Some(ref fed_now_system_response_value) = self.fed_now_system_response { if !fed_now_system_response_value.validate() { return false; } }
		if let Some(ref fed_now_participant_file_value) = self.fed_now_participant_file { if !fed_now_participant_file_value.validate() { return false; } }
		if let Some(ref fed_now_payment_status_value) = self.fed_now_payment_status { if !fed_now_payment_status_value.validate() { return false; } }
		if let Some(ref fed_now_payment_return_value) = self.fed_now_payment_return { if !fed_now_payment_return_value.validate() { return false; } }
		if let Some(ref fed_now_customer_credit_transfer_value) = self.fed_now_customer_credit_transfer { if !fed_now_customer_credit_transfer_value.validate() { return false; } }
		if let Some(ref fed_now_institution_credit_transfer_value) = self.fed_now_institution_credit_transfer { if !fed_now_institution_credit_transfer_value.validate() { return false; } }
		if let Some(ref fed_now_payment_status_request_value) = self.fed_now_payment_status_request { if !fed_now_payment_status_request_value.validate() { return false; } }
		if let Some(ref fed_now_request_for_payment_value) = self.fed_now_request_for_payment { if !fed_now_request_for_payment_value.validate() { return false; } }
		if let Some(ref fed_now_request_for_payment_response_value) = self.fed_now_request_for_payment_response { if !fed_now_request_for_payment_response_value.validate() { return false; } }
		if let Some(ref fed_now_information_request_value) = self.fed_now_information_request { if !fed_now_information_request_value.validate() { return false; } }
		if let Some(ref fed_now_additional_payment_information_value) = self.fed_now_additional_payment_information { if !fed_now_additional_payment_information_value.validate() { return false; } }
		if let Some(ref fed_now_return_request_response_value) = self.fed_now_return_request_response { if !fed_now_return_request_response_value.validate() { return false; } }
		if let Some(ref fed_now_information_request_response_value) = self.fed_now_information_request_response { if !fed_now_information_request_response_value.validate() { return false; } }
		if let Some(ref fed_now_account_activity_details_report_value) = self.fed_now_account_activity_details_report { if !fed_now_account_activity_details_report_value.validate() { return false; } }
		if let Some(ref fed_now_account_activity_totals_report_value) = self.fed_now_account_activity_totals_report { if !fed_now_account_activity_totals_report_value.validate() { return false; } }
		if let Some(ref fed_now_account_balance_report_value) = self.fed_now_account_balance_report { if !fed_now_account_balance_report_value.validate() { return false; } }
		if let Some(ref account_debit_credit_notification_value) = self.account_debit_credit_notification { if !account_debit_credit_notification_value.validate() { return false; } }
		if let Some(ref fed_now_request_for_payment_cancellation_request_value) = self.fed_now_request_for_payment_cancellation_request { if !fed_now_request_for_payment_cancellation_request_value.validate() { return false; } }
		if let Some(ref fed_now_request_for_payment_cancellation_request_response_value) = self.fed_now_request_for_payment_cancellation_request_response { if !fed_now_request_for_payment_cancellation_request_response_value.validate() { return false; } }
		if let Some(ref fed_now_return_request_value) = self.fed_now_return_request { if !fed_now_return_request_value.validate() { return false; } }
		if let Some(ref fed_now_outgoing_message_signature_management_value) = self.fed_now_outgoing_message_signature_management { if !fed_now_outgoing_message_signature_management_value.validate() { return false; } }
		return true
	}
}


// FedNowMessageReject ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowMessageReject {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub a2_document: Document,
}

impl FedNowMessageReject {
	pub fn validate(&self) -> bool {
		if !self.bah_app_hdr.validate() { return false }
		if !self.a2_document.validate() { return false }
		return true
	}
}


// FedNowBroadcast ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowBroadcast {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub a4_document: Document,
}

impl FedNowBroadcast {
	pub fn validate(&self) -> bool {
		if !self.bah_app_hdr.validate() { return false }
		if !self.a4_document.validate() { return false }
		return true
	}
}


// FedNowReceiptAcknowledgement ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowReceiptAcknowledgement {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub a7_document: Document,
}

impl FedNowReceiptAcknowledgement {
	pub fn validate(&self) -> bool {
		if !self.bah_app_hdr.validate() { return false }
		if !self.a7_document.validate() { return false }
		return true
	}
}


// FedNowSystemResponse ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowSystemResponse {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub a11_document: Document,
}

impl FedNowSystemResponse {
	pub fn validate(&self) -> bool {
		if !self.bah_app_hdr.validate() { return false }
		if !self.a11_document.validate() { return false }
		return true
	}
}


// FedNowParticipantFile ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowParticipantFile {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub a998_document: Document,
}

impl FedNowParticipantFile {
	pub fn validate(&self) -> bool {
		if !self.bah_app_hdr.validate() { return false }
		if !self.a998_document.validate() { return false }
		return true
	}
}


// FedNowPaymentStatus ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowPaymentStatus {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub p2_document: Document,
}

impl FedNowPaymentStatus {
	pub fn validate(&self) -> bool {
		if !self.bah_app_hdr.validate() { return false }
		if !self.p2_document.validate() { return false }
		return true
	}
}


// FedNowPaymentReturn ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowPaymentReturn {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub p4_document: Document,
}

impl FedNowPaymentReturn {
	pub fn validate(&self) -> bool {
		if !self.bah_app_hdr.validate() { return false }
		if !self.p4_document.validate() { return false }
		return true
	}
}


// FedNowCustomerCreditTransfer ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowCustomerCreditTransfer {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub p8_document: Document,
}

impl FedNowCustomerCreditTransfer {
	pub fn validate(&self) -> bool {
		if !self.bah_app_hdr.validate() { return false }
		if !self.p8_document.validate() { return false }
		return true
	}
}


// FedNowInstitutionCreditTransfer ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowInstitutionCreditTransfer {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub p9_document: Document,
}

impl FedNowInstitutionCreditTransfer {
	pub fn validate(&self) -> bool {
		if !self.bah_app_hdr.validate() { return false }
		if !self.p9_document.validate() { return false }
		return true
	}
}


// FedNowPaymentStatusRequest ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowPaymentStatusRequest {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub p28_document: Document,
}

impl FedNowPaymentStatusRequest {
	pub fn validate(&self) -> bool {
		if !self.bah_app_hdr.validate() { return false }
		if !self.p28_document.validate() { return false }
		return true
	}
}


// FedNowRequestForPayment ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowRequestForPayment {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub pain13_document: Document,
}

impl FedNowRequestForPayment {
	pub fn validate(&self) -> bool {
		if !self.bah_app_hdr.validate() { return false }
		if !self.pain13_document.validate() { return false }
		return true
	}
}


// FedNowRequestForPaymentResponse ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowRequestForPaymentResponse {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub pain14_document: Document,
}

impl FedNowRequestForPaymentResponse {
	pub fn validate(&self) -> bool {
		if !self.bah_app_hdr.validate() { return false }
		if !self.pain14_document.validate() { return false }
		return true
	}
}


// FedNowInformationRequest ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowInformationRequest {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub c26_document: Document,
}

impl FedNowInformationRequest {
	pub fn validate(&self) -> bool {
		if !self.bah_app_hdr.validate() { return false }
		if !self.c26_document.validate() { return false }
		return true
	}
}


// FedNowAdditionalPaymentInformation ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowAdditionalPaymentInformation {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub c28_document: Document,
}

impl FedNowAdditionalPaymentInformation {
	pub fn validate(&self) -> bool {
		if !self.bah_app_hdr.validate() { return false }
		if !self.c28_document.validate() { return false }
		return true
	}
}


// FedNowRequestForPaymentCancellationRequestResponse ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowRequestForPaymentCancellationRequestResponse {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub c29_document: Document,
}

impl FedNowRequestForPaymentCancellationRequestResponse {
	pub fn validate(&self) -> bool {
		if !self.bah_app_hdr.validate() { return false }
		if !self.c29_document.validate() { return false }
		return true
	}
}


// FedNowReturnRequestResponse ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowReturnRequestResponse {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub c29_document: Document,
}

impl FedNowReturnRequestResponse {
	pub fn validate(&self) -> bool {
		if !self.bah_app_hdr.validate() { return false }
		if !self.c29_document.validate() { return false }
		return true
	}
}


// FedNowInformationRequestResponse ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowInformationRequestResponse {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub c29_document: Document,
}

impl FedNowInformationRequestResponse {
	pub fn validate(&self) -> bool {
		if !self.bah_app_hdr.validate() { return false }
		if !self.c29_document.validate() { return false }
		return true
	}
}


// FedNowAccountActivityDetailsReport ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowAccountActivityDetailsReport {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub c52_document: Document,
}

impl FedNowAccountActivityDetailsReport {
	pub fn validate(&self) -> bool {
		if !self.bah_app_hdr.validate() { return false }
		if !self.c52_document.validate() { return false }
		return true
	}
}


// FedNowAccountActivityTotalsReport ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowAccountActivityTotalsReport {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub c52_document: Document,
}

impl FedNowAccountActivityTotalsReport {
	pub fn validate(&self) -> bool {
		if !self.bah_app_hdr.validate() { return false }
		if !self.c52_document.validate() { return false }
		return true
	}
}


// FedNowAccountBalanceReport ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowAccountBalanceReport {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub c52_document: Document,
}

impl FedNowAccountBalanceReport {
	pub fn validate(&self) -> bool {
		if !self.bah_app_hdr.validate() { return false }
		if !self.c52_document.validate() { return false }
		return true
	}
}


// AccountDebitCreditNotification ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AccountDebitCreditNotification {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub c54_document: Document,
}

impl AccountDebitCreditNotification {
	pub fn validate(&self) -> bool {
		if !self.bah_app_hdr.validate() { return false }
		if !self.c54_document.validate() { return false }
		return true
	}
}


// FedNowRequestForPaymentCancellationRequest ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowRequestForPaymentCancellationRequest {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub c55_document: Document,
}

impl FedNowRequestForPaymentCancellationRequest {
	pub fn validate(&self) -> bool {
		if !self.bah_app_hdr.validate() { return false }
		if !self.c55_document.validate() { return false }
		return true
	}
}


// FedNowReturnRequest ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowReturnRequest {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub c56_document: Document,
}

impl FedNowReturnRequest {
	pub fn validate(&self) -> bool {
		if !self.bah_app_hdr.validate() { return false }
		if !self.c56_document.validate() { return false }
		return true
	}
}


// FedNowOutgoingMessageSignatureManagement ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowOutgoingMessageSignatureManagement {
	#[serde(rename = "FedNowPublicKeyResponses", skip_serializing_if = "Option::is_none")]
	pub ke_fed_now_public_key_responses: Option<FedNowPublicKeyResponses>,
	#[serde(rename = "FedNowCustomerMessageSignatureKeyOperationResponse", skip_serializing_if = "Option::is_none")]
	pub ke_fed_now_customer_message_signature_key_operation_response: Option<FedNowCustomerMessageSignatureKeyOperationResponse>,
}

impl FedNowOutgoingMessageSignatureManagement {
	pub fn validate(&self) -> bool {
		if let Some(ref ke_fed_now_public_key_responses_value) = self.ke_fed_now_public_key_responses { if !ke_fed_now_public_key_responses_value.validate() { return false; } }
		if let Some(ref ke_fed_now_customer_message_signature_key_operation_response_value) = self.ke_fed_now_customer_message_signature_key_operation_response { if !ke_fed_now_customer_message_signature_key_operation_response_value.validate() { return false; } }
		return true
	}
}
