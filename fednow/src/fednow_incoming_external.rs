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


use regex::Regex;
use crate::common::*;
#[cfg(feature = "derive_serde")]
use serde::{Deserialize, Serialize};
use crate::document::Document;
use crate::fednow_extra::key_exchange::*;
use crate::iso::head_001_001_02::BusinessApplicationHeaderV02;

// FedNowIncoming ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowIncoming {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowTechnicalHeader", skip_serializing_if = "Option::is_none") )]
	pub fed_now_technical_header: Option<FedNowTechnicalHeader>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowIncomingMessage") )]
	pub fed_now_incoming_message: FedNowIncomingMessage,
}

impl FedNowIncoming {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.fed_now_technical_header { val.validate()? }
		self.fed_now_incoming_message.validate()?;
		Ok(())
	}
}


// FedNowTechnicalHeader ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowTechnicalHeader {
}

impl FedNowTechnicalHeader {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FedNowIncomingMessage ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowIncomingMessage {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowMessageReject", skip_serializing_if = "Option::is_none") )]
	pub fed_now_message_reject: Option<FedNowMessageReject>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowParticipantBroadcast", skip_serializing_if = "Option::is_none") )]
	pub fed_now_participant_broadcast: Option<FedNowParticipantBroadcast>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowRetrievalRequest", skip_serializing_if = "Option::is_none") )]
	pub fed_now_retrieval_request: Option<FedNowRetrievalRequest>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowReceiptAcknowledgement", skip_serializing_if = "Option::is_none") )]
	pub fed_now_receipt_acknowledgement: Option<FedNowReceiptAcknowledgement>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowPaymentStatus", skip_serializing_if = "Option::is_none") )]
	pub fed_now_payment_status: Option<FedNowPaymentStatus>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowPaymentReturn", skip_serializing_if = "Option::is_none") )]
	pub fed_now_payment_return: Option<FedNowPaymentReturn>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowCustomerCreditTransfer", skip_serializing_if = "Option::is_none") )]
	pub fed_now_customer_credit_transfer: Option<FedNowCustomerCreditTransfer>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowInstitutionCreditTransfer", skip_serializing_if = "Option::is_none") )]
	pub fed_now_institution_credit_transfer: Option<FedNowInstitutionCreditTransfer>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowPaymentStatusRequest", skip_serializing_if = "Option::is_none") )]
	pub fed_now_payment_status_request: Option<FedNowPaymentStatusRequest>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowRequestForPayment", skip_serializing_if = "Option::is_none") )]
	pub fed_now_request_for_payment: Option<FedNowRequestForPayment>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowRequestForPaymentResponse", skip_serializing_if = "Option::is_none") )]
	pub fed_now_request_for_payment_response: Option<FedNowRequestForPaymentResponse>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowInformationRequest", skip_serializing_if = "Option::is_none") )]
	pub fed_now_information_request: Option<FedNowInformationRequest>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowAdditionalPaymentInformation", skip_serializing_if = "Option::is_none") )]
	pub fed_now_additional_payment_information: Option<FedNowAdditionalPaymentInformation>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowInformationRequestResponse", skip_serializing_if = "Option::is_none") )]
	pub fed_now_information_request_response: Option<FedNowInformationRequestResponse>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowRequestForPaymentCancellationRequestResponse", skip_serializing_if = "Option::is_none") )]
	pub fed_now_request_for_payment_cancellation_request_response: Option<FedNowRequestForPaymentCancellationRequestResponse>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowReturnRequestResponse", skip_serializing_if = "Option::is_none") )]
	pub fed_now_return_request_response: Option<FedNowReturnRequestResponse>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowRequestForPaymentCancellationRequest", skip_serializing_if = "Option::is_none") )]
	pub fed_now_request_for_payment_cancellation_request: Option<FedNowRequestForPaymentCancellationRequest>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowReturnRequest", skip_serializing_if = "Option::is_none") )]
	pub fed_now_return_request: Option<FedNowReturnRequest>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowAccountReportingRequest", skip_serializing_if = "Option::is_none") )]
	pub fed_now_account_reporting_request: Option<FedNowAccountReportingRequest>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowIncomingMessageSignatureManagement", skip_serializing_if = "Option::is_none") )]
	pub fed_now_incoming_message_signature_management: Option<FedNowIncomingMessageSignatureManagement>,
}

impl FedNowIncomingMessage {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.fed_now_message_reject { val.validate()? }
		if let Some(ref val) = self.fed_now_participant_broadcast { val.validate()? }
		if let Some(ref val) = self.fed_now_retrieval_request { val.validate()? }
		if let Some(ref val) = self.fed_now_receipt_acknowledgement { val.validate()? }
		if let Some(ref val) = self.fed_now_payment_status { val.validate()? }
		if let Some(ref val) = self.fed_now_payment_return { val.validate()? }
		if let Some(ref val) = self.fed_now_customer_credit_transfer { val.validate()? }
		if let Some(ref val) = self.fed_now_institution_credit_transfer { val.validate()? }
		if let Some(ref val) = self.fed_now_payment_status_request { val.validate()? }
		if let Some(ref val) = self.fed_now_request_for_payment { val.validate()? }
		if let Some(ref val) = self.fed_now_request_for_payment_response { val.validate()? }
		if let Some(ref val) = self.fed_now_information_request { val.validate()? }
		if let Some(ref val) = self.fed_now_additional_payment_information { val.validate()? }
		if let Some(ref val) = self.fed_now_information_request_response { val.validate()? }
		if let Some(ref val) = self.fed_now_request_for_payment_cancellation_request_response { val.validate()? }
		if let Some(ref val) = self.fed_now_return_request_response { val.validate()? }
		if let Some(ref val) = self.fed_now_request_for_payment_cancellation_request { val.validate()? }
		if let Some(ref val) = self.fed_now_return_request { val.validate()? }
		if let Some(ref val) = self.fed_now_account_reporting_request { val.validate()? }
		if let Some(ref val) = self.fed_now_incoming_message_signature_management { val.validate()? }
		Ok(())
	}
}


// FedNowMessageReject ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowMessageReject {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AppHdr") )]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]
	pub a2_document: Document,
}

impl FedNowMessageReject {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.bah_app_hdr.validate()?;
		self.a2_document.validate()?;
		Ok(())
	}
}


// FedNowParticipantBroadcast ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowParticipantBroadcast {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AppHdr") )]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]
	pub a4_document: Document,
}

impl FedNowParticipantBroadcast {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.bah_app_hdr.validate()?;
		self.a4_document.validate()?;
		Ok(())
	}
}


// FedNowRetrievalRequest ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowRetrievalRequest {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AppHdr") )]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]
	pub a6_document: Document,
}

impl FedNowRetrievalRequest {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.bah_app_hdr.validate()?;
		self.a6_document.validate()?;
		Ok(())
	}
}


// FedNowReceiptAcknowledgement ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowReceiptAcknowledgement {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AppHdr") )]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]
	pub a7_document: Document,
}

impl FedNowReceiptAcknowledgement {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.bah_app_hdr.validate()?;
		self.a7_document.validate()?;
		Ok(())
	}
}


// FedNowPaymentStatus ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowPaymentStatus {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AppHdr") )]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]
	pub p2_document: Document,
}

impl FedNowPaymentStatus {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.bah_app_hdr.validate()?;
		self.p2_document.validate()?;
		Ok(())
	}
}


// FedNowPaymentReturn ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowPaymentReturn {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AppHdr") )]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]
	pub p4_document: Document,
}

impl FedNowPaymentReturn {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.bah_app_hdr.validate()?;
		self.p4_document.validate()?;
		Ok(())
	}
}


// FedNowCustomerCreditTransfer ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowCustomerCreditTransfer {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AppHdr") )]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]
	pub p8_document: Document,
}

impl FedNowCustomerCreditTransfer {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.bah_app_hdr.validate()?;
		self.p8_document.validate()?;
		Ok(())
	}
}


// FedNowInstitutionCreditTransfer ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowInstitutionCreditTransfer {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AppHdr") )]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]
	pub p9_document: Document,
}

impl FedNowInstitutionCreditTransfer {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.bah_app_hdr.validate()?;
		self.p9_document.validate()?;
		Ok(())
	}
}


// FedNowPaymentStatusRequest ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowPaymentStatusRequest {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AppHdr") )]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]
	pub p28_document: Document,
}

impl FedNowPaymentStatusRequest {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.bah_app_hdr.validate()?;
		self.p28_document.validate()?;
		Ok(())
	}
}


// FedNowRequestForPayment ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowRequestForPayment {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AppHdr") )]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]
	pub pain13_document: Document,
}

impl FedNowRequestForPayment {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.bah_app_hdr.validate()?;
		self.pain13_document.validate()?;
		Ok(())
	}
}


// FedNowRequestForPaymentResponse ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowRequestForPaymentResponse {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AppHdr") )]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]
	pub pain14_document: Document,
}

impl FedNowRequestForPaymentResponse {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.bah_app_hdr.validate()?;
		self.pain14_document.validate()?;
		Ok(())
	}
}


// FedNowInformationRequest ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowInformationRequest {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AppHdr") )]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]
	pub c26_document: Document,
}

impl FedNowInformationRequest {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.bah_app_hdr.validate()?;
		self.c26_document.validate()?;
		Ok(())
	}
}


// FedNowAdditionalPaymentInformation ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowAdditionalPaymentInformation {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AppHdr") )]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]
	pub c28_document: Document,
}

impl FedNowAdditionalPaymentInformation {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.bah_app_hdr.validate()?;
		self.c28_document.validate()?;
		Ok(())
	}
}


// FedNowInformationRequestResponse ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowInformationRequestResponse {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AppHdr") )]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]
	pub c29_document: Document,
}

impl FedNowInformationRequestResponse {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.bah_app_hdr.validate()?;
		self.c29_document.validate()?;
		Ok(())
	}
}


// FedNowRequestForPaymentCancellationRequestResponse ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowRequestForPaymentCancellationRequestResponse {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AppHdr") )]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]
	pub c29_document: Document,
}

impl FedNowRequestForPaymentCancellationRequestResponse {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.bah_app_hdr.validate()?;
		self.c29_document.validate()?;
		Ok(())
	}
}


// FedNowReturnRequestResponse ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowReturnRequestResponse {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AppHdr") )]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]
	pub c29_document: Document,
}

impl FedNowReturnRequestResponse {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.bah_app_hdr.validate()?;
		self.c29_document.validate()?;
		Ok(())
	}
}


// FedNowRequestForPaymentCancellationRequest ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowRequestForPaymentCancellationRequest {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AppHdr") )]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]
	pub c55_document: Document,
}

impl FedNowRequestForPaymentCancellationRequest {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.bah_app_hdr.validate()?;
		self.c55_document.validate()?;
		Ok(())
	}
}


// FedNowReturnRequest ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowReturnRequest {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AppHdr") )]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]
	pub c56_document: Document,
}

impl FedNowReturnRequest {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.bah_app_hdr.validate()?;
		self.c56_document.validate()?;
		Ok(())
	}
}


// FedNowAccountReportingRequest ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowAccountReportingRequest {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AppHdr") )]
	pub bah_app_hdr: BusinessApplicationHeaderV02,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]
	pub c60_document: Document,
}

impl FedNowAccountReportingRequest {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.bah_app_hdr.validate()?;
		self.c60_document.validate()?;
		Ok(())
	}
}


// FedNowIncomingMessageSignatureManagement ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowIncomingMessageSignatureManagement {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SenderId") )]
	pub sender_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GetAllFedNowActivePublicKeys", skip_serializing_if = "Option::is_none") )]
	pub ke_get_all_fed_now_active_public_keys: Option<GetAllFedNowActivePublicKeys>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GetAllCustomerPublicKeys", skip_serializing_if = "Option::is_none") )]
	pub ke_get_all_customer_public_keys: Option<GetAllCustomerPublicKeys>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowMessageSignatureKeyExchange", skip_serializing_if = "Option::is_none") )]
	pub ke_fed_now_message_signature_key_exchange: Option<FedNowMessageSignatureKeyExchange>,
}

impl FedNowIncomingMessageSignatureManagement {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ke_get_all_fed_now_active_public_keys { val.validate()? }
		if let Some(ref val) = self.ke_get_all_customer_public_keys { val.validate()? }
		if let Some(ref val) = self.ke_fed_now_message_signature_key_exchange { val.validate()? }
		Ok(())
	}
}
