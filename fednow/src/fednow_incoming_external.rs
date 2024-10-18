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
use crate::validationerror::*;
use crate::document::*;
use crate::fednow::key_exchange::*;
use crate::iso::head_001_001_02::BusinessApplicationHeaderV02;


// FedNowIncoming ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowIncoming {
	#[serde(rename = "FedNowTechnicalHeader", skip_serializing_if = "Option::is_none")]
	pub fed_now_technical_header: Option<FedNowTechnicalHeader>,
	#[serde(rename = "FedNowIncomingMessage")]
	pub fed_now_incoming_message: FedNowIncomingMessage,
}

impl FedNowIncoming {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref fed_now_technical_header_value) = self.fed_now_technical_header { if let Err(e) = fed_now_technical_header_value.validate() { return Err(e); } }
		if let Err(e) = self.fed_now_incoming_message.validate() { return Err(e); }
		Ok(())
	}
}


// FedNowTechnicalHeader ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowTechnicalHeader {
}

impl FedNowTechnicalHeader {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FedNowIncomingMessage ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowIncomingMessage {
	#[serde(rename = "FedNowMessageReject", skip_serializing_if = "Option::is_none")]
	pub fed_now_message_reject: Option<FedNowMessageReject>,
	#[serde(rename = "FedNowParticipantBroadcast", skip_serializing_if = "Option::is_none")]
	pub fed_now_participant_broadcast: Option<FedNowParticipantBroadcast>,
	#[serde(rename = "FedNowRetrievalRequest", skip_serializing_if = "Option::is_none")]
	pub fed_now_retrieval_request: Option<FedNowRetrievalRequest>,
	#[serde(rename = "FedNowReceiptAcknowledgement", skip_serializing_if = "Option::is_none")]
	pub fed_now_receipt_acknowledgement: Option<FedNowReceiptAcknowledgement>,
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
	#[serde(rename = "FedNowInformationRequestResponse", skip_serializing_if = "Option::is_none")]
	pub fed_now_information_request_response: Option<FedNowInformationRequestResponse>,
	#[serde(rename = "FedNowRequestForPaymentCancellationRequestResponse", skip_serializing_if = "Option::is_none")]
	pub fed_now_request_for_payment_cancellation_request_response: Option<FedNowRequestForPaymentCancellationRequestResponse>,
	#[serde(rename = "FedNowReturnRequestResponse", skip_serializing_if = "Option::is_none")]
	pub fed_now_return_request_response: Option<FedNowReturnRequestResponse>,
	#[serde(rename = "FedNowRequestForPaymentCancellationRequest", skip_serializing_if = "Option::is_none")]
	pub fed_now_request_for_payment_cancellation_request: Option<FedNowRequestForPaymentCancellationRequest>,
	#[serde(rename = "FedNowReturnRequest", skip_serializing_if = "Option::is_none")]
	pub fed_now_return_request: Option<FedNowReturnRequest>,
	#[serde(rename = "FedNowAccountReportingRequest", skip_serializing_if = "Option::is_none")]
	pub fed_now_account_reporting_request: Option<FedNowAccountReportingRequest>,
	#[serde(rename = "FedNowIncomingMessageSignatureManagement", skip_serializing_if = "Option::is_none")]
	pub fed_now_incoming_message_signature_management: Option<FedNowIncomingMessageSignatureManagement>,
}

impl FedNowIncomingMessage {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref fed_now_message_reject_value) = self.fed_now_message_reject { if let Err(e) = fed_now_message_reject_value.validate() { return Err(e); } }
		if let Some(ref fed_now_participant_broadcast_value) = self.fed_now_participant_broadcast { if let Err(e) = fed_now_participant_broadcast_value.validate() { return Err(e); } }
		if let Some(ref fed_now_retrieval_request_value) = self.fed_now_retrieval_request { if let Err(e) = fed_now_retrieval_request_value.validate() { return Err(e); } }
		if let Some(ref fed_now_receipt_acknowledgement_value) = self.fed_now_receipt_acknowledgement { if let Err(e) = fed_now_receipt_acknowledgement_value.validate() { return Err(e); } }
		if let Some(ref fed_now_payment_status_value) = self.fed_now_payment_status { if let Err(e) = fed_now_payment_status_value.validate() { return Err(e); } }
		if let Some(ref fed_now_payment_return_value) = self.fed_now_payment_return { if let Err(e) = fed_now_payment_return_value.validate() { return Err(e); } }
		if let Some(ref fed_now_customer_credit_transfer_value) = self.fed_now_customer_credit_transfer { if let Err(e) = fed_now_customer_credit_transfer_value.validate() { return Err(e); } }
		if let Some(ref fed_now_institution_credit_transfer_value) = self.fed_now_institution_credit_transfer { if let Err(e) = fed_now_institution_credit_transfer_value.validate() { return Err(e); } }
		if let Some(ref fed_now_payment_status_request_value) = self.fed_now_payment_status_request { if let Err(e) = fed_now_payment_status_request_value.validate() { return Err(e); } }
		if let Some(ref fed_now_request_for_payment_value) = self.fed_now_request_for_payment { if let Err(e) = fed_now_request_for_payment_value.validate() { return Err(e); } }
		if let Some(ref fed_now_request_for_payment_response_value) = self.fed_now_request_for_payment_response { if let Err(e) = fed_now_request_for_payment_response_value.validate() { return Err(e); } }
		if let Some(ref fed_now_information_request_value) = self.fed_now_information_request { if let Err(e) = fed_now_information_request_value.validate() { return Err(e); } }
		if let Some(ref fed_now_additional_payment_information_value) = self.fed_now_additional_payment_information { if let Err(e) = fed_now_additional_payment_information_value.validate() { return Err(e); } }
		if let Some(ref fed_now_information_request_response_value) = self.fed_now_information_request_response { if let Err(e) = fed_now_information_request_response_value.validate() { return Err(e); } }
		if let Some(ref fed_now_request_for_payment_cancellation_request_response_value) = self.fed_now_request_for_payment_cancellation_request_response { if let Err(e) = fed_now_request_for_payment_cancellation_request_response_value.validate() { return Err(e); } }
		if let Some(ref fed_now_return_request_response_value) = self.fed_now_return_request_response { if let Err(e) = fed_now_return_request_response_value.validate() { return Err(e); } }
		if let Some(ref fed_now_request_for_payment_cancellation_request_value) = self.fed_now_request_for_payment_cancellation_request { if let Err(e) = fed_now_request_for_payment_cancellation_request_value.validate() { return Err(e); } }
		if let Some(ref fed_now_return_request_value) = self.fed_now_return_request { if let Err(e) = fed_now_return_request_value.validate() { return Err(e); } }
		if let Some(ref fed_now_account_reporting_request_value) = self.fed_now_account_reporting_request { if let Err(e) = fed_now_account_reporting_request_value.validate() { return Err(e); } }
		if let Some(ref fed_now_incoming_message_signature_management_value) = self.fed_now_incoming_message_signature_management { if let Err(e) = fed_now_incoming_message_signature_management_value.validate() { return Err(e); } }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
		if let Err(e) = self.a2_document.validate() { return Err(e); }
		Ok(())
	}
}


// FedNowParticipantBroadcast ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowParticipantBroadcast {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub a4_document: Document,
}

impl FedNowParticipantBroadcast {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
		if let Err(e) = self.a4_document.validate() { return Err(e); }
		Ok(())
	}
}


// FedNowRetrievalRequest ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowRetrievalRequest {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub a6_document: Document,
}

impl FedNowRetrievalRequest {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
		if let Err(e) = self.a6_document.validate() { return Err(e); }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
		if let Err(e) = self.a7_document.validate() { return Err(e); }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
		if let Err(e) = self.p2_document.validate() { return Err(e); }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
		if let Err(e) = self.p4_document.validate() { return Err(e); }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
		if let Err(e) = self.p8_document.validate() { return Err(e); }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
		if let Err(e) = self.p9_document.validate() { return Err(e); }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
		if let Err(e) = self.p28_document.validate() { return Err(e); }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
		if let Err(e) = self.pain13_document.validate() { return Err(e); }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
		if let Err(e) = self.pain14_document.validate() { return Err(e); }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
		if let Err(e) = self.c26_document.validate() { return Err(e); }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
		if let Err(e) = self.c28_document.validate() { return Err(e); }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
		if let Err(e) = self.c29_document.validate() { return Err(e); }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
		if let Err(e) = self.c29_document.validate() { return Err(e); }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
		if let Err(e) = self.c29_document.validate() { return Err(e); }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
		if let Err(e) = self.c55_document.validate() { return Err(e); }
		Ok(())
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
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
		if let Err(e) = self.c56_document.validate() { return Err(e); }
		Ok(())
	}
}


// FedNowAccountReportingRequest ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowAccountReportingRequest {
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[serde(rename = "Document")]
	pub c60_document: Document,
}

impl FedNowAccountReportingRequest {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
		if let Err(e) = self.c60_document.validate() { return Err(e); }
		Ok(())
	}
}


// FedNowIncomingMessageSignatureManagement ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct FedNowIncomingMessageSignatureManagement {
	#[serde(rename = "SenderId")]
	pub sender_id: String,
	#[serde(rename = "GetAllFedNowActivePublicKeys", skip_serializing_if = "Option::is_none")]
	pub ke_get_all_fed_now_active_public_keys: Option<GetAllFedNowActivePublicKeys>,
	#[serde(rename = "GetAllCustomerPublicKeys", skip_serializing_if = "Option::is_none")]
	pub ke_get_all_customer_public_keys: Option<GetAllCustomerPublicKeys>,
	#[serde(rename = "FedNowMessageSignatureKeyExchange", skip_serializing_if = "Option::is_none")]
	pub ke_fed_now_message_signature_key_exchange: Option<FedNowMessageSignatureKeyExchange>,
}

impl FedNowIncomingMessageSignatureManagement {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref ke_get_all_fed_now_active_public_keys_value) = self.ke_get_all_fed_now_active_public_keys { if let Err(e) = ke_get_all_fed_now_active_public_keys_value.validate() { return Err(e); } }
		if let Some(ref ke_get_all_customer_public_keys_value) = self.ke_get_all_customer_public_keys { if let Err(e) = ke_get_all_customer_public_keys_value.validate() { return Err(e); } }
		if let Some(ref ke_fed_now_message_signature_key_exchange_value) = self.ke_fed_now_message_signature_key_exchange { if let Err(e) = ke_fed_now_message_signature_key_exchange_value.validate() { return Err(e); } }
		Ok(())
	}
}
