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

pub mod fednow {
	use regex::Regex;
	use crate::common::*;
	#[cfg(feature = "derive_serde")]
	use serde::{Deserialize, Serialize};
	use crate::document::Document;
	use crate::fednow_extra::key_exchange::fednow::*;
	use crate::iso::head_001_001_02::fednow::BusinessApplicationHeaderV02;

	// FedNowOutgoing ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FedNowOutgoing {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowTechnicalHeader", skip_serializing_if = "Option::is_none") )]
		pub fed_now_technical_header: Option<FedNowTechnicalHeader>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowOutgoingMessage") )]
		pub fed_now_outgoing_message: FedNowOutgoingMessage,
	}
	
	impl FedNowOutgoing {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref fed_now_technical_header_value) = self.fed_now_technical_header { if let Err(e) = fed_now_technical_header_value.validate() { return Err(e); } }
			if let Err(e) = self.fed_now_outgoing_message.validate() { return Err(e); }
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
	
	
	// FedNowOutgoingMessage ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FedNowOutgoingMessage {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowMessageReject", skip_serializing_if = "Option::is_none") )]
		pub fed_now_message_reject: Option<FedNowMessageReject>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowBroadcast", skip_serializing_if = "Option::is_none") )]
		pub fed_now_broadcast: Option<FedNowBroadcast>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowReceiptAcknowledgement", skip_serializing_if = "Option::is_none") )]
		pub fed_now_receipt_acknowledgement: Option<FedNowReceiptAcknowledgement>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowSystemResponse", skip_serializing_if = "Option::is_none") )]
		pub fed_now_system_response: Option<FedNowSystemResponse>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowParticipantFile", skip_serializing_if = "Option::is_none") )]
		pub fed_now_participant_file: Option<FedNowParticipantFile>,
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
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowReturnRequestResponse", skip_serializing_if = "Option::is_none") )]
		pub fed_now_return_request_response: Option<FedNowReturnRequestResponse>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowInformationRequestResponse", skip_serializing_if = "Option::is_none") )]
		pub fed_now_information_request_response: Option<FedNowInformationRequestResponse>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowAccountActivityDetailsReport", skip_serializing_if = "Option::is_none") )]
		pub fed_now_account_activity_details_report: Option<FedNowAccountActivityDetailsReport>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowAccountActivityTotalsReport", skip_serializing_if = "Option::is_none") )]
		pub fed_now_account_activity_totals_report: Option<FedNowAccountActivityTotalsReport>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowAccountBalanceReport", skip_serializing_if = "Option::is_none") )]
		pub fed_now_account_balance_report: Option<FedNowAccountBalanceReport>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "AccountDebitCreditNotification", skip_serializing_if = "Option::is_none") )]
		pub account_debit_credit_notification: Option<AccountDebitCreditNotification>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowRequestForPaymentCancellationRequest", skip_serializing_if = "Option::is_none") )]
		pub fed_now_request_for_payment_cancellation_request: Option<FedNowRequestForPaymentCancellationRequest>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowRequestForPaymentCancellationRequestResponse", skip_serializing_if = "Option::is_none") )]
		pub fed_now_request_for_payment_cancellation_request_response: Option<FedNowRequestForPaymentCancellationRequestResponse>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowReturnRequest", skip_serializing_if = "Option::is_none") )]
		pub fed_now_return_request: Option<FedNowReturnRequest>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowOutgoingMessageSignatureManagement", skip_serializing_if = "Option::is_none") )]
		pub fed_now_outgoing_message_signature_management: Option<FedNowOutgoingMessageSignatureManagement>,
	}
	
	impl FedNowOutgoingMessage {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref fed_now_message_reject_value) = self.fed_now_message_reject { if let Err(e) = fed_now_message_reject_value.validate() { return Err(e); } }
			if let Some(ref fed_now_broadcast_value) = self.fed_now_broadcast { if let Err(e) = fed_now_broadcast_value.validate() { return Err(e); } }
			if let Some(ref fed_now_receipt_acknowledgement_value) = self.fed_now_receipt_acknowledgement { if let Err(e) = fed_now_receipt_acknowledgement_value.validate() { return Err(e); } }
			if let Some(ref fed_now_system_response_value) = self.fed_now_system_response { if let Err(e) = fed_now_system_response_value.validate() { return Err(e); } }
			if let Some(ref fed_now_participant_file_value) = self.fed_now_participant_file { if let Err(e) = fed_now_participant_file_value.validate() { return Err(e); } }
			if let Some(ref fed_now_payment_status_value) = self.fed_now_payment_status { if let Err(e) = fed_now_payment_status_value.validate() { return Err(e); } }
			if let Some(ref fed_now_payment_return_value) = self.fed_now_payment_return { if let Err(e) = fed_now_payment_return_value.validate() { return Err(e); } }
			if let Some(ref fed_now_customer_credit_transfer_value) = self.fed_now_customer_credit_transfer { if let Err(e) = fed_now_customer_credit_transfer_value.validate() { return Err(e); } }
			if let Some(ref fed_now_institution_credit_transfer_value) = self.fed_now_institution_credit_transfer { if let Err(e) = fed_now_institution_credit_transfer_value.validate() { return Err(e); } }
			if let Some(ref fed_now_payment_status_request_value) = self.fed_now_payment_status_request { if let Err(e) = fed_now_payment_status_request_value.validate() { return Err(e); } }
			if let Some(ref fed_now_request_for_payment_value) = self.fed_now_request_for_payment { if let Err(e) = fed_now_request_for_payment_value.validate() { return Err(e); } }
			if let Some(ref fed_now_request_for_payment_response_value) = self.fed_now_request_for_payment_response { if let Err(e) = fed_now_request_for_payment_response_value.validate() { return Err(e); } }
			if let Some(ref fed_now_information_request_value) = self.fed_now_information_request { if let Err(e) = fed_now_information_request_value.validate() { return Err(e); } }
			if let Some(ref fed_now_additional_payment_information_value) = self.fed_now_additional_payment_information { if let Err(e) = fed_now_additional_payment_information_value.validate() { return Err(e); } }
			if let Some(ref fed_now_return_request_response_value) = self.fed_now_return_request_response { if let Err(e) = fed_now_return_request_response_value.validate() { return Err(e); } }
			if let Some(ref fed_now_information_request_response_value) = self.fed_now_information_request_response { if let Err(e) = fed_now_information_request_response_value.validate() { return Err(e); } }
			if let Some(ref fed_now_account_activity_details_report_value) = self.fed_now_account_activity_details_report { if let Err(e) = fed_now_account_activity_details_report_value.validate() { return Err(e); } }
			if let Some(ref fed_now_account_activity_totals_report_value) = self.fed_now_account_activity_totals_report { if let Err(e) = fed_now_account_activity_totals_report_value.validate() { return Err(e); } }
			if let Some(ref fed_now_account_balance_report_value) = self.fed_now_account_balance_report { if let Err(e) = fed_now_account_balance_report_value.validate() { return Err(e); } }
			if let Some(ref account_debit_credit_notification_value) = self.account_debit_credit_notification { if let Err(e) = account_debit_credit_notification_value.validate() { return Err(e); } }
			if let Some(ref fed_now_request_for_payment_cancellation_request_value) = self.fed_now_request_for_payment_cancellation_request { if let Err(e) = fed_now_request_for_payment_cancellation_request_value.validate() { return Err(e); } }
			if let Some(ref fed_now_request_for_payment_cancellation_request_response_value) = self.fed_now_request_for_payment_cancellation_request_response { if let Err(e) = fed_now_request_for_payment_cancellation_request_response_value.validate() { return Err(e); } }
			if let Some(ref fed_now_return_request_value) = self.fed_now_return_request { if let Err(e) = fed_now_return_request_value.validate() { return Err(e); } }
			if let Some(ref fed_now_outgoing_message_signature_management_value) = self.fed_now_outgoing_message_signature_management { if let Err(e) = fed_now_outgoing_message_signature_management_value.validate() { return Err(e); } }
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
			if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
			if let Err(e) = self.a2_document.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// FedNowBroadcast ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FedNowBroadcast {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AppHdr") )]
		pub bah_app_hdr: BusinessApplicationHeaderV02,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]
		pub a4_document: Document,
	}
	
	impl FedNowBroadcast {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
			if let Err(e) = self.a4_document.validate() { return Err(e); }
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
			if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
			if let Err(e) = self.a7_document.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// FedNowSystemResponse ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FedNowSystemResponse {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AppHdr") )]
		pub bah_app_hdr: BusinessApplicationHeaderV02,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]
		pub a11_document: Document,
	}
	
	impl FedNowSystemResponse {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
			if let Err(e) = self.a11_document.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// FedNowParticipantFile ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FedNowParticipantFile {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AppHdr") )]
		pub bah_app_hdr: BusinessApplicationHeaderV02,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]
		pub a998_document: Document,
	}
	
	impl FedNowParticipantFile {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
			if let Err(e) = self.a998_document.validate() { return Err(e); }
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
			if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
			if let Err(e) = self.p2_document.validate() { return Err(e); }
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
			if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
			if let Err(e) = self.p4_document.validate() { return Err(e); }
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
			if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
			if let Err(e) = self.p8_document.validate() { return Err(e); }
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
			if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
			if let Err(e) = self.p9_document.validate() { return Err(e); }
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
			if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
			if let Err(e) = self.p28_document.validate() { return Err(e); }
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
			if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
			if let Err(e) = self.pain13_document.validate() { return Err(e); }
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
			if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
			if let Err(e) = self.pain14_document.validate() { return Err(e); }
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
			if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
			if let Err(e) = self.c26_document.validate() { return Err(e); }
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
			if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
			if let Err(e) = self.c28_document.validate() { return Err(e); }
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
			if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
			if let Err(e) = self.c29_document.validate() { return Err(e); }
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
			if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
			if let Err(e) = self.c29_document.validate() { return Err(e); }
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
			if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
			if let Err(e) = self.c29_document.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// FedNowAccountActivityDetailsReport ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FedNowAccountActivityDetailsReport {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AppHdr") )]
		pub bah_app_hdr: BusinessApplicationHeaderV02,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]
		pub c52_document: Document,
	}
	
	impl FedNowAccountActivityDetailsReport {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
			if let Err(e) = self.c52_document.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// FedNowAccountActivityTotalsReport ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FedNowAccountActivityTotalsReport {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AppHdr") )]
		pub bah_app_hdr: BusinessApplicationHeaderV02,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]
		pub c52_document: Document,
	}
	
	impl FedNowAccountActivityTotalsReport {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
			if let Err(e) = self.c52_document.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// FedNowAccountBalanceReport ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FedNowAccountBalanceReport {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AppHdr") )]
		pub bah_app_hdr: BusinessApplicationHeaderV02,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]
		pub c52_document: Document,
	}
	
	impl FedNowAccountBalanceReport {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
			if let Err(e) = self.c52_document.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// AccountDebitCreditNotification ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct AccountDebitCreditNotification {
		#[cfg_attr( feature = "derive_serde", serde(rename = "AppHdr") )]
		pub bah_app_hdr: BusinessApplicationHeaderV02,
		#[cfg_attr( feature = "derive_serde", serde(rename = "Document") )]
		pub c54_document: Document,
	}
	
	impl AccountDebitCreditNotification {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
			if let Err(e) = self.c54_document.validate() { return Err(e); }
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
			if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
			if let Err(e) = self.c55_document.validate() { return Err(e); }
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
			if let Err(e) = self.bah_app_hdr.validate() { return Err(e); }
			if let Err(e) = self.c56_document.validate() { return Err(e); }
			Ok(())
		}
	}
	
	
	// FedNowOutgoingMessageSignatureManagement ...
	#[cfg_attr(feature = "derive_debug", derive(Debug))]
	#[cfg_attr(feature = "derive_default", derive(Default))]
	#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
	#[cfg_attr(feature = "derive_clone", derive(Clone))]
	#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
	pub struct FedNowOutgoingMessageSignatureManagement {
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowPublicKeyResponses", skip_serializing_if = "Option::is_none") )]
		pub ke_fed_now_public_key_responses: Option<FedNowPublicKeyResponses>,
		#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowCustomerMessageSignatureKeyOperationResponse", skip_serializing_if = "Option::is_none") )]
		pub ke_fed_now_customer_message_signature_key_operation_response: Option<FedNowCustomerMessageSignatureKeyOperationResponse>,
	}
	
	impl FedNowOutgoingMessageSignatureManagement {
		pub fn validate(&self) -> Result<(), ValidationError> {
			if let Some(ref ke_fed_now_public_key_responses_value) = self.ke_fed_now_public_key_responses { if let Err(e) = ke_fed_now_public_key_responses_value.validate() { return Err(e); } }
			if let Some(ref ke_fed_now_customer_message_signature_key_operation_response_value) = self.ke_fed_now_customer_message_signature_key_operation_response { if let Err(e) = ke_fed_now_customer_message_signature_key_operation_response_value.validate() { return Err(e); } }
			Ok(())
		}
	}
	
}