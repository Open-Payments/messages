// FedNow Message Parsing Library
// https://github.com/Open-Payments/messages
//
// This library is designed to parse FedNow message formats based on ISO 20022 standards.
// It handles various message types, including administrative notifications, payment status reports, 
// customer credit transfers, and more, using Serde for efficient serialization and deserialization.
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
use utoipa::ToSchema;
use crate::message::fednow::document::*;
use crate::message::fednow::fednow::key_exchange::*;
use crate::message::fednow::iso::head_001_001_02::BusinessApplicationHeaderV02;


// FedNowIncoming ...
#[derive(Debug, Validate, ToSchema, Deserialize, Serialize, PartialEq)]
pub struct FedNowIncoming {
	#[validate]
	#[serde(rename = "FedNowTechnicalHeader")]
	pub fed_now_technical_header: Option<FedNowTechnicalHeader>,
	#[validate]
	#[serde(rename = "FedNowIncomingMessage")]
	pub fed_now_incoming_message: FedNowIncomingMessage,
}


// FedNowTechnicalHeader ...
#[derive(Debug, Validate, ToSchema, Deserialize, Serialize, PartialEq)]
pub struct FedNowTechnicalHeader {
}


// FedNowIncomingMessage ...
#[derive(Debug, Validate, ToSchema, Deserialize, Serialize, PartialEq)]
pub struct FedNowIncomingMessage {
	#[validate]
	#[serde(rename = "FedNowMessageReject")]
	pub fed_now_message_reject: Option<FedNowMessageReject>,
	#[validate]
	#[serde(rename = "FedNowParticipantBroadcast")]
	pub fed_now_participant_broadcast: Option<FedNowParticipantBroadcast>,
	#[validate]
	#[serde(rename = "FedNowRetrievalRequest")]
	pub fed_now_retrieval_request: Option<FedNowRetrievalRequest>,
	#[validate]
	#[serde(rename = "FedNowReceiptAcknowledgement")]
	pub fed_now_receipt_acknowledgement: Option<FedNowReceiptAcknowledgement>,
	#[validate]
	#[serde(rename = "FedNowPaymentStatus")]
	pub fed_now_payment_status: Option<FedNowPaymentStatus>,
	#[validate]
	#[serde(rename = "FedNowPaymentReturn")]
	pub fed_now_payment_return: Option<FedNowPaymentReturn>,
	#[validate]
	#[serde(rename = "FedNowCustomerCreditTransfer")]
	pub fed_now_customer_credit_transfer: Option<FedNowCustomerCreditTransfer>,
	#[validate]
	#[serde(rename = "FedNowInstitutionCreditTransfer")]
	pub fed_now_institution_credit_transfer: Option<FedNowInstitutionCreditTransfer>,
	#[validate]
	#[serde(rename = "FedNowPaymentStatusRequest")]
	pub fed_now_payment_status_request: Option<FedNowPaymentStatusRequest>,
	#[validate]
	#[serde(rename = "FedNowRequestForPayment")]
	pub fed_now_request_for_payment: Option<FedNowRequestForPayment>,
	#[validate]
	#[serde(rename = "FedNowRequestForPaymentResponse")]
	pub fed_now_request_for_payment_response: Option<FedNowRequestForPaymentResponse>,
	#[validate]
	#[serde(rename = "FedNowInformationRequest")]
	pub fed_now_information_request: Option<FedNowInformationRequest>,
	#[validate]
	#[serde(rename = "FedNowAdditionalPaymentInformation")]
	pub fed_now_additional_payment_information: Option<FedNowAdditionalPaymentInformation>,
	#[validate]
	#[serde(rename = "FedNowInformationRequestResponse")]
	pub fed_now_information_request_response: Option<FedNowInformationRequestResponse>,
	#[validate]
	#[serde(rename = "FedNowRequestForPaymentCancellationRequestResponse")]
	pub fed_now_request_for_payment_cancellation_request_response: Option<FedNowRequestForPaymentCancellationRequestResponse>,
	#[validate]
	#[serde(rename = "FedNowReturnRequestResponse")]
	pub fed_now_return_request_response: Option<FedNowReturnRequestResponse>,
	#[validate]
	#[serde(rename = "FedNowRequestForPaymentCancellationRequest")]
	pub fed_now_request_for_payment_cancellation_request: Option<FedNowRequestForPaymentCancellationRequest>,
	#[validate]
	#[serde(rename = "FedNowReturnRequest")]
	pub fed_now_return_request: Option<FedNowReturnRequest>,
	#[validate]
	#[serde(rename = "FedNowAccountReportingRequest")]
	pub fed_now_account_reporting_request: Option<FedNowAccountReportingRequest>,
	#[validate]
	#[serde(rename = "FedNowIncomingMessageSignatureManagement")]
	pub fed_now_incoming_message_signature_management: Option<FedNowIncomingMessageSignatureManagement>,
}


// FedNowMessageReject ...
#[derive(Debug, Validate, ToSchema, Deserialize, Serialize, PartialEq)]
pub struct FedNowMessageReject {
	#[validate]
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[validate]
	#[serde(rename = "Document")]
	pub a2_document: Document,
}


// FedNowParticipantBroadcast ...
#[derive(Debug, Validate, ToSchema, Deserialize, Serialize, PartialEq)]
pub struct FedNowParticipantBroadcast {
	#[validate]
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[validate]
	#[serde(rename = "Document")]
	pub a4_document: Document,
}


// FedNowRetrievalRequest ...
#[derive(Debug, Validate, ToSchema, Deserialize, Serialize, PartialEq)]
pub struct FedNowRetrievalRequest {
	#[validate]
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[validate]
	#[serde(rename = "Document")]
	pub a6_document: Document,
}


// FedNowReceiptAcknowledgement ...
#[derive(Debug, Validate, ToSchema, Deserialize, Serialize, PartialEq)]
pub struct FedNowReceiptAcknowledgement {
	#[validate]
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[validate]
	#[serde(rename = "Document")]
	pub a7_document: Document,
}


// FedNowPaymentStatus ...
#[derive(Debug, Validate, ToSchema, Deserialize, Serialize, PartialEq)]
pub struct FedNowPaymentStatus {
	#[validate]
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[validate]
	#[serde(rename = "Document")]
	pub p2_document: Document,
}


// FedNowPaymentReturn ...
#[derive(Debug, Validate, ToSchema, Deserialize, Serialize, PartialEq)]
pub struct FedNowPaymentReturn {
	#[validate]
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[validate]
	#[serde(rename = "Document")]
	pub p4_document: Document,
}


// FedNowCustomerCreditTransfer ...
#[derive(Debug, Validate, ToSchema, Deserialize, Serialize, PartialEq)]
pub struct FedNowCustomerCreditTransfer {
	#[validate]
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[validate]
	#[serde(rename = "Document")]
	pub p8_document: Document,
}


// FedNowInstitutionCreditTransfer ...
#[derive(Debug, Validate, ToSchema, Deserialize, Serialize, PartialEq)]
pub struct FedNowInstitutionCreditTransfer {
	#[validate]
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[validate]
	#[serde(rename = "Document")]
	pub p9_document: Document,
}


// FedNowPaymentStatusRequest ...
#[derive(Debug, Validate, ToSchema, Deserialize, Serialize, PartialEq)]
pub struct FedNowPaymentStatusRequest {
	#[validate]
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[validate]
	#[serde(rename = "Document")]
	pub p28_document: Document,
}


// FedNowRequestForPayment ...
#[derive(Debug, Validate, ToSchema, Deserialize, Serialize, PartialEq)]
pub struct FedNowRequestForPayment {
	#[validate]
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[validate]
	#[serde(rename = "Document")]
	pub pain13_document: Document,
}


// FedNowRequestForPaymentResponse ...
#[derive(Debug, Validate, ToSchema, Deserialize, Serialize, PartialEq)]
pub struct FedNowRequestForPaymentResponse {
	#[validate]
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[validate]
	#[serde(rename = "Document")]
	pub pain14_document: Document,
}


// FedNowInformationRequest ...
#[derive(Debug, Validate, ToSchema, Deserialize, Serialize, PartialEq)]
pub struct FedNowInformationRequest {
	#[validate]
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[validate]
	#[serde(rename = "Document")]
	pub c26_document: Document,
}


// FedNowAdditionalPaymentInformation ...
#[derive(Debug, Validate, ToSchema, Deserialize, Serialize, PartialEq)]
pub struct FedNowAdditionalPaymentInformation {
	#[validate]
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[validate]
	#[serde(rename = "Document")]
	pub c28_document: Document,
}


// FedNowInformationRequestResponse ...
#[derive(Debug, Validate, ToSchema, Deserialize, Serialize, PartialEq)]
pub struct FedNowInformationRequestResponse {
	#[validate]
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[validate]
	#[serde(rename = "Document")]
	pub c29_document: Document,
}


// FedNowRequestForPaymentCancellationRequestResponse ...
#[derive(Debug, Validate, ToSchema, Deserialize, Serialize, PartialEq)]
pub struct FedNowRequestForPaymentCancellationRequestResponse {
	#[validate]
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[validate]
	#[serde(rename = "Document")]
	pub c29_document: Document,
}


// FedNowReturnRequestResponse ...
#[derive(Debug, Validate, ToSchema, Deserialize, Serialize, PartialEq)]
pub struct FedNowReturnRequestResponse {
	#[validate]
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[validate]
	#[serde(rename = "Document")]
	pub c29_document: Document,
}


// FedNowRequestForPaymentCancellationRequest ...
#[derive(Debug, Validate, ToSchema, Deserialize, Serialize, PartialEq)]
pub struct FedNowRequestForPaymentCancellationRequest {
	#[validate]
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[validate]
	#[serde(rename = "Document")]
	pub c55_document: Document,
}


// FedNowReturnRequest ...
#[derive(Debug, Validate, ToSchema, Deserialize, Serialize, PartialEq)]
pub struct FedNowReturnRequest {
	#[validate]
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[validate]
	#[serde(rename = "Document")]
	pub c56_document: Document,
}


// FedNowAccountReportingRequest ...
#[derive(Debug, Validate, ToSchema, Deserialize, Serialize, PartialEq)]
pub struct FedNowAccountReportingRequest {
	#[validate]
	#[serde(rename = "AppHdr")]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[validate]
	#[serde(rename = "Document")]
	pub c60_document: Document,
}


// FedNowIncomingMessageSignatureManagement ...
#[derive(Debug, Validate, ToSchema, Deserialize, Serialize, PartialEq)]
pub struct FedNowIncomingMessageSignatureManagement {
	#[serde(rename = "SenderId")]
	pub sender_id: String,
	#[validate]
	#[serde(rename = "GetAllFedNowActivePublicKeys")]
	pub ke_get_all_fed_now_active_public_keys: Option<GetAllFedNowActivePublicKeys>,
	#[validate]
	#[serde(rename = "GetAllCustomerPublicKeys")]
	pub ke_get_all_customer_public_keys: Option<GetAllCustomerPublicKeys>,
	#[validate]
	#[serde(rename = "FedNowMessageSignatureKeyExchange")]
	pub ke_fed_now_message_signature_key_exchange: Option<FedNowMessageSignatureKeyExchange>,
}
