// Suppress warnings about unused imports when features are not enabled
#![allow(unused_imports)]
use regex::Regex;

// Conditionally import necessary traits and modules
#[cfg(feature = "derive_serde")]
use serde::{Deserialize, Serialize};

use crate::document::Document;
use crate::fednow_extra::key_exchange::*;


#[derive(Debug)]
pub struct ValidationError {
    pub code: u32,
    pub message: String,
}

impl ValidationError {
    pub fn new(code: u32, message: String) -> Self {
        ValidationError { code, message }
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
		self.bah_app_hdr.validate()?;
		self.c54_document.validate()?;
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
		self.bah_app_hdr.validate()?;
		self.c52_document.validate()?;
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
		self.bah_app_hdr.validate()?;
		self.c52_document.validate()?;
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
		self.bah_app_hdr.validate()?;
		self.c52_document.validate()?;
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
		self.bah_app_hdr.validate()?;
		self.a4_document.validate()?;
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
		if let Some(ref val) = self.fed_now_message_reject { val.validate()? }
		if let Some(ref val) = self.fed_now_broadcast { val.validate()? }
		if let Some(ref val) = self.fed_now_receipt_acknowledgement { val.validate()? }
		if let Some(ref val) = self.fed_now_system_response { val.validate()? }
		if let Some(ref val) = self.fed_now_participant_file { val.validate()? }
		if let Some(ref val) = self.fed_now_payment_status { val.validate()? }
		if let Some(ref val) = self.fed_now_payment_return { val.validate()? }
		if let Some(ref val) = self.fed_now_customer_credit_transfer { val.validate()? }
		if let Some(ref val) = self.fed_now_institution_credit_transfer { val.validate()? }
		if let Some(ref val) = self.fed_now_payment_status_request { val.validate()? }
		if let Some(ref val) = self.fed_now_request_for_payment { val.validate()? }
		if let Some(ref val) = self.fed_now_request_for_payment_response { val.validate()? }
		if let Some(ref val) = self.fed_now_information_request { val.validate()? }
		if let Some(ref val) = self.fed_now_additional_payment_information { val.validate()? }
		if let Some(ref val) = self.fed_now_return_request_response { val.validate()? }
		if let Some(ref val) = self.fed_now_information_request_response { val.validate()? }
		if let Some(ref val) = self.fed_now_account_activity_details_report { val.validate()? }
		if let Some(ref val) = self.fed_now_account_activity_totals_report { val.validate()? }
		if let Some(ref val) = self.fed_now_account_balance_report { val.validate()? }
		if let Some(ref val) = self.account_debit_credit_notification { val.validate()? }
		if let Some(ref val) = self.fed_now_request_for_payment_cancellation_request { val.validate()? }
		if let Some(ref val) = self.fed_now_request_for_payment_cancellation_request_response { val.validate()? }
		if let Some(ref val) = self.fed_now_return_request { val.validate()? }
		if let Some(ref val) = self.fed_now_outgoing_message_signature_management { val.validate()? }
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
		if let Some(ref val) = self.ke_fed_now_public_key_responses { val.validate()? }
		if let Some(ref val) = self.ke_fed_now_customer_message_signature_key_operation_response { val.validate()? }
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
		self.bah_app_hdr.validate()?;
		self.a998_document.validate()?;
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
		self.bah_app_hdr.validate()?;
		self.a11_document.validate()?;
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


// AccountIdentification4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AccountIdentification4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "IBAN", skip_serializing_if = "Option::is_none") )]
	pub iban: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<GenericAccountIdentification1>,
}

impl AccountIdentification4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.iban {
			let pattern = Regex::new("[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "iban does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.othr { val.validate()? }
		Ok(())
	}
}


// AccountInterest4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AccountInterest4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<InterestType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
	pub rate: Option<Vec<Rate4>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrToDt", skip_serializing_if = "Option::is_none") )]
	pub fr_to_dt: Option<DateTimePeriod1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tax", skip_serializing_if = "Option::is_none") )]
	pub tax: Option<TaxCharges2>,
}

impl AccountInterest4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref vec) = self.rate { for item in vec { item.validate()? } }
		if let Some(ref val) = self.fr_to_dt { val.validate()? }
		if let Some(ref val) = self.rsn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rsn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "rsn exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.tax { val.validate()? }
		Ok(())
	}
}


// AccountNotification17 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AccountNotification17 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NtfctnPgntn", skip_serializing_if = "Option::is_none") )]
	pub ntfctn_pgntn: Option<Pagination1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ElctrncSeqNb", skip_serializing_if = "Option::is_none") )]
	pub elctrnc_seq_nb: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgSeq", skip_serializing_if = "Option::is_none") )]
	pub rptg_seq: Option<SequenceRange1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglSeqNb", skip_serializing_if = "Option::is_none") )]
	pub lgl_seq_nb: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
	pub cre_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrToDt", skip_serializing_if = "Option::is_none") )]
	pub fr_to_dt: Option<DateTimePeriod1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CpyDplctInd", skip_serializing_if = "Option::is_none") )]
	pub cpy_dplct_ind: Option<CopyDuplicate1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgSrc", skip_serializing_if = "Option::is_none") )]
	pub rptg_src: Option<ReportingSource1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Acct") )]
	pub acct: CashAccount39,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdAcct", skip_serializing_if = "Option::is_none") )]
	pub rltd_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Intrst", skip_serializing_if = "Option::is_none") )]
	pub intrst: Option<Vec<AccountInterest4>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxsSummry", skip_serializing_if = "Option::is_none") )]
	pub txs_summry: Option<TotalTransactions6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ntry", skip_serializing_if = "Option::is_none") )]
	pub ntry: Option<Vec<ReportEntry10>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlNtfctnInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_ntfctn_inf: Option<String>,
}

impl AccountNotification17 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.ntfctn_pgntn { val.validate()? }
		if let Some(ref val) = self.rptg_seq { val.validate()? }
		if let Some(ref val) = self.fr_to_dt { val.validate()? }
		if let Some(ref val) = self.cpy_dplct_ind { val.validate()? }
		if let Some(ref val) = self.rptg_src { val.validate()? }
		self.acct.validate()?;
		if let Some(ref val) = self.rltd_acct { val.validate()? }
		if let Some(ref vec) = self.intrst { for item in vec { item.validate()? } }
		if let Some(ref val) = self.txs_summry { val.validate()? }
		if let Some(ref vec) = self.ntry { for item in vec { item.validate()? } }
		if let Some(ref val) = self.addtl_ntfctn_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_ntfctn_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 500 {
				return Err(ValidationError::new(1002, "addtl_ntfctn_inf exceeds the maximum length of 500".to_string()));
			}
		}
		Ok(())
	}
}


// AccountReport25 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AccountReport25 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptPgntn", skip_serializing_if = "Option::is_none") )]
	pub rpt_pgntn: Option<Pagination1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ElctrncSeqNb", skip_serializing_if = "Option::is_none") )]
	pub elctrnc_seq_nb: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgSeq", skip_serializing_if = "Option::is_none") )]
	pub rptg_seq: Option<SequenceRange1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LglSeqNb", skip_serializing_if = "Option::is_none") )]
	pub lgl_seq_nb: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
	pub cre_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrToDt", skip_serializing_if = "Option::is_none") )]
	pub fr_to_dt: Option<DateTimePeriod1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CpyDplctInd", skip_serializing_if = "Option::is_none") )]
	pub cpy_dplct_ind: Option<CopyDuplicate1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgSrc", skip_serializing_if = "Option::is_none") )]
	pub rptg_src: Option<ReportingSource1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Acct") )]
	pub acct: CashAccount39,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdAcct", skip_serializing_if = "Option::is_none") )]
	pub rltd_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Intrst", skip_serializing_if = "Option::is_none") )]
	pub intrst: Option<Vec<AccountInterest4>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Bal", skip_serializing_if = "Option::is_none") )]
	pub bal: Option<Vec<CashBalance8>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxsSummry", skip_serializing_if = "Option::is_none") )]
	pub txs_summry: Option<TotalTransactions6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ntry", skip_serializing_if = "Option::is_none") )]
	pub ntry: Option<Vec<ReportEntry10>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRptInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_rpt_inf: Option<String>,
}

impl AccountReport25 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.rpt_pgntn { val.validate()? }
		if let Some(ref val) = self.rptg_seq { val.validate()? }
		if let Some(ref val) = self.fr_to_dt { val.validate()? }
		if let Some(ref val) = self.cpy_dplct_ind { val.validate()? }
		if let Some(ref val) = self.rptg_src { val.validate()? }
		self.acct.validate()?;
		if let Some(ref val) = self.rltd_acct { val.validate()? }
		if let Some(ref vec) = self.intrst { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.bal { for item in vec { item.validate()? } }
		if let Some(ref val) = self.txs_summry { val.validate()? }
		if let Some(ref vec) = self.ntry { for item in vec { item.validate()? } }
		if let Some(ref val) = self.addtl_rpt_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_rpt_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 500 {
				return Err(ValidationError::new(1002, "addtl_rpt_inf exceeds the maximum length of 500".to_string()));
			}
		}
		Ok(())
	}
}


// AccountSchemeName1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AccountSchemeName1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl AccountSchemeName1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// ActiveCurrencyAndAmount ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ActiveCurrencyAndAmount {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
	pub value: f64,
}

impl ActiveCurrencyAndAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ActiveOrHistoricCurrencyAnd13DecimalAmount ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmount {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
	pub value: f64,
}

impl ActiveOrHistoricCurrencyAnd13DecimalAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ActiveOrHistoricCurrencyAndAmount ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ActiveOrHistoricCurrencyAndAmount {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "$value") )]
	pub value: f64,
}

impl ActiveOrHistoricCurrencyAndAmount {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ActiveOrHistoricCurrencyAndAmountRange2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ActiveOrHistoricCurrencyAndAmountRange2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ImpliedCurrencyAmountRange1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none") )]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy") )]
	pub ccy: String,
}

impl ActiveOrHistoricCurrencyAndAmountRange2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		if let Some(ref val) = self.cdt_dbt_ind { val.validate()? }
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.ccy) {
			return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// AddressType2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum AddressType2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ADDR") )]
	CodeADDR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PBOX") )]
	CodePBOX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HOME") )]
	CodeHOME,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BIZZ") )]
	CodeBIZZ,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MLTO") )]
	CodeMLTO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DLVY") )]
	CodeDLVY,
}

impl AddressType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AddressType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AddressType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<AddressType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification30>,
}

impl AddressType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// AmendmentInformationDetails13 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AmendmentInformationDetails13 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMndtId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_mndt_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCdtrSchmeId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_cdtr_schme_id: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCdtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub orgnl_cdtr_agt_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlDbtr", skip_serializing_if = "Option::is_none") )]
	pub orgnl_dbtr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlDbtrAcct", skip_serializing_if = "Option::is_none") )]
	pub orgnl_dbtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlDbtrAgt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlDbtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub orgnl_dbtr_agt_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlFnlColltnDt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_fnl_colltn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlFrqcy", skip_serializing_if = "Option::is_none") )]
	pub orgnl_frqcy: Option<Frequency36Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlRsn", skip_serializing_if = "Option::is_none") )]
	pub orgnl_rsn: Option<MandateSetupReason1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTrckgDays", skip_serializing_if = "Option::is_none") )]
	pub orgnl_trckg_days: Option<String>,
}

impl AmendmentInformationDetails13 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_mndt_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_mndt_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_mndt_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_cdtr_schme_id { val.validate()? }
		if let Some(ref val) = self.orgnl_cdtr_agt { val.validate()? }
		if let Some(ref val) = self.orgnl_cdtr_agt_acct { val.validate()? }
		if let Some(ref val) = self.orgnl_dbtr { val.validate()? }
		if let Some(ref val) = self.orgnl_dbtr_acct { val.validate()? }
		if let Some(ref val) = self.orgnl_dbtr_agt { val.validate()? }
		if let Some(ref val) = self.orgnl_dbtr_agt_acct { val.validate()? }
		if let Some(ref val) = self.orgnl_frqcy { val.validate()? }
		if let Some(ref val) = self.orgnl_rsn { val.validate()? }
		if let Some(ref val) = self.orgnl_trckg_days {
			let pattern = Regex::new("[0-9]{2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_trckg_days does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// AmountAndCurrencyExchange3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AmountAndCurrencyExchange3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none") )]
	pub instd_amt: Option<AmountAndCurrencyExchangeDetails3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxAmt", skip_serializing_if = "Option::is_none") )]
	pub tx_amt: Option<AmountAndCurrencyExchangeDetails3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CntrValAmt", skip_serializing_if = "Option::is_none") )]
	pub cntr_val_amt: Option<AmountAndCurrencyExchangeDetails3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnncdPstngAmt", skip_serializing_if = "Option::is_none") )]
	pub anncd_pstng_amt: Option<AmountAndCurrencyExchangeDetails3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryAmt", skip_serializing_if = "Option::is_none") )]
	pub prtry_amt: Option<Vec<AmountAndCurrencyExchangeDetails4>>,
}

impl AmountAndCurrencyExchange3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.instd_amt { val.validate()? }
		if let Some(ref val) = self.tx_amt { val.validate()? }
		if let Some(ref val) = self.cntr_val_amt { val.validate()? }
		if let Some(ref val) = self.anncd_pstng_amt { val.validate()? }
		if let Some(ref vec) = self.prtry_amt { for item in vec { item.validate()? } }
		Ok(())
	}
}


// AmountAndCurrencyExchangeDetails3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AmountAndCurrencyExchangeDetails3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CcyXchg", skip_serializing_if = "Option::is_none") )]
	pub ccy_xchg: Option<CurrencyExchange5>,
}

impl AmountAndCurrencyExchangeDetails3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		if let Some(ref val) = self.ccy_xchg { val.validate()? }
		Ok(())
	}
}


// AmountAndCurrencyExchangeDetails4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AmountAndCurrencyExchangeDetails4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CcyXchg", skip_serializing_if = "Option::is_none") )]
	pub ccy_xchg: Option<CurrencyExchange5>,
}

impl AmountAndCurrencyExchangeDetails4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
		}
		if self.tp.chars().count() > 35 {
			return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
		}
		self.amt.validate()?;
		if let Some(ref val) = self.ccy_xchg { val.validate()? }
		Ok(())
	}
}


// AmountAndDirection35 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AmountAndDirection35 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: f64,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd") )]
	pub cdt_dbt_ind: CreditDebitCode,
}

impl AmountAndDirection35 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.amt < 0.000000 {
			return Err(ValidationError::new(1003, "amt is less than the minimum value of 0.000000".to_string()));
		}
		self.cdt_dbt_ind.validate()?;
		Ok(())
	}
}


// AmountOrRate1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AmountOrRate1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
	pub rate: Option<f64>,
}

impl AmountOrRate1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.amt { val.validate()? }
		Ok(())
	}
}


// AmountRangeBoundary1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AmountRangeBoundary1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "BdryAmt") )]
	pub bdry_amt: f64,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Incl") )]
	pub incl: bool,
}

impl AmountRangeBoundary1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.bdry_amt < 0.000000 {
			return Err(ValidationError::new(1003, "bdry_amt is less than the minimum value of 0.000000".to_string()));
		}
		Ok(())
	}
}


// AmountType4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct AmountType4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none") )]
	pub instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EqvtAmt", skip_serializing_if = "Option::is_none") )]
	pub eqvt_amt: Option<EquivalentAmount2>,
}

impl AmountType4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.instd_amt { val.validate()? }
		if let Some(ref val) = self.eqvt_amt { val.validate()? }
		Ok(())
	}
}


// AttendanceContext1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum AttendanceContext1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ATTD") )]
	CodeATTD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SATT") )]
	CodeSATT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UATT") )]
	CodeUATT,
}

impl AttendanceContext1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AuthenticationEntity1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum AuthenticationEntity1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ICCD") )]
	CodeICCD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AGNT") )]
	CodeAGNT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MERC") )]
	CodeMERC,
}

impl AuthenticationEntity1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// AuthenticationMethod1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum AuthenticationMethod1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "UKNW") )]
	CodeUKNW,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BYPS") )]
	CodeBYPS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NPIN") )]
	CodeNPIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FPIN") )]
	CodeFPIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CPSG") )]
	CodeCPSG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PPSG") )]
	CodePPSG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MANU") )]
	CodeMANU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MERC") )]
	CodeMERC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SCRT") )]
	CodeSCRT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SNCT") )]
	CodeSNCT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SCNL") )]
	CodeSCNL,
}

impl AuthenticationMethod1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Authorisation1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Authorisation1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<Authorisation1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl Authorisation1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 128 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 128".to_string()));
			}
		}
		Ok(())
	}
}


// Authorisation1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum Authorisation1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "AUTH") )]
	CodeAUTH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FDET") )]
	CodeFDET,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FSUM") )]
	CodeFSUM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ILEV") )]
	CodeILEV,
}

impl Authorisation1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// BalanceSubType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BalanceSubType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl BalanceSubType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// BalanceType10Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BalanceType10Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl BalanceType10Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// BalanceType13 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BalanceType13 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdOrPrtry") )]
	pub cd_or_prtry: BalanceType10Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubTp", skip_serializing_if = "Option::is_none") )]
	pub sub_tp: Option<BalanceSubType1Choice>,
}

impl BalanceType13 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd_or_prtry.validate()?;
		if let Some(ref val) = self.sub_tp { val.validate()? }
		Ok(())
	}
}


// BankTransactionCodeStructure4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BankTransactionCodeStructure4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Domn", skip_serializing_if = "Option::is_none") )]
	pub domn: Option<BankTransactionCodeStructure5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<ProprietaryBankTransactionCodeStructure1>,
}

impl BankTransactionCodeStructure4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.domn { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// BankTransactionCodeStructure5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BankTransactionCodeStructure5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
	pub cd: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Fmly") )]
	pub fmly: BankTransactionCodeStructure6,
}

impl BankTransactionCodeStructure5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.cd.chars().count() < 1 {
			return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
		}
		if self.cd.chars().count() > 4 {
			return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
		}
		self.fmly.validate()?;
		Ok(())
	}
}


// BankTransactionCodeStructure6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BankTransactionCodeStructure6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
	pub cd: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubFmlyCd") )]
	pub sub_fmly_cd: String,
}

impl BankTransactionCodeStructure6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.cd.chars().count() < 1 {
			return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
		}
		if self.cd.chars().count() > 4 {
			return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
		}
		if self.sub_fmly_cd.chars().count() < 1 {
			return Err(ValidationError::new(1001, "sub_fmly_cd is shorter than the minimum length of 1".to_string()));
		}
		if self.sub_fmly_cd.chars().count() > 4 {
			return Err(ValidationError::new(1002, "sub_fmly_cd exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// BatchInformation2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BatchInformation2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId", skip_serializing_if = "Option::is_none") )]
	pub msg_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtInfId", skip_serializing_if = "Option::is_none") )]
	pub pmt_inf_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxs", skip_serializing_if = "Option::is_none") )]
	pub nb_of_txs: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none") )]
	pub ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none") )]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
}

impl BatchInformation2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.msg_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.pmt_inf_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pmt_inf_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "pmt_inf_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.nb_of_txs {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "nb_of_txs does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ttl_amt { val.validate()? }
		if let Some(ref val) = self.cdt_dbt_ind { val.validate()? }
		Ok(())
	}
}


// BranchAndFinancialInstitutionIdentification6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BranchAndFinancialInstitutionIdentification6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstnId") )]
	pub fin_instn_id: FinancialInstitutionIdentification18,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BrnchId", skip_serializing_if = "Option::is_none") )]
	pub brnch_id: Option<BranchData3>,
}

impl BranchAndFinancialInstitutionIdentification6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.fin_instn_id.validate()?;
		if let Some(ref val) = self.brnch_id { val.validate()? }
		Ok(())
	}
}


// BranchData3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BranchData3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
	pub pstl_adr: Option<PostalAddress24>,
}

impl BranchData3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.pstl_adr { val.validate()? }
		Ok(())
	}
}


// BusinessApplicationHeader5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BusinessApplicationHeader5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CharSet", skip_serializing_if = "Option::is_none") )]
	pub char_set: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Fr") )]
	pub fr: Party44Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "To") )]
	pub to: Party44Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BizMsgIdr") )]
	pub biz_msg_idr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgDefIdr") )]
	pub msg_def_idr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BizSvc", skip_serializing_if = "Option::is_none") )]
	pub biz_svc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDt") )]
	pub cre_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CpyDplct", skip_serializing_if = "Option::is_none") )]
	pub cpy_dplct: Option<CopyDuplicate1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PssblDplct", skip_serializing_if = "Option::is_none") )]
	pub pssbl_dplct: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prty", skip_serializing_if = "Option::is_none") )]
	pub prty: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sgntr", skip_serializing_if = "Option::is_none") )]
	pub sgntr: Option<SignatureEnvelope>,
}

impl BusinessApplicationHeader5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.fr.validate()?;
		self.to.validate()?;
		if self.biz_msg_idr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "biz_msg_idr is shorter than the minimum length of 1".to_string()));
		}
		if self.biz_msg_idr.chars().count() > 35 {
			return Err(ValidationError::new(1002, "biz_msg_idr exceeds the maximum length of 35".to_string()));
		}
		if self.msg_def_idr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_def_idr is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_def_idr.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_def_idr exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.biz_svc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "biz_svc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "biz_svc exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.cpy_dplct { val.validate()? }
		if let Some(ref val) = self.sgntr { val.validate()? }
		Ok(())
	}
}


// BusinessApplicationHeaderV02 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct BusinessApplicationHeaderV02 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CharSet", skip_serializing_if = "Option::is_none") )]
	pub char_set: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Fr") )]
	pub fr: Party44Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "To") )]
	pub to: Party44Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BizMsgIdr") )]
	pub biz_msg_idr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgDefIdr") )]
	pub msg_def_idr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BizSvc", skip_serializing_if = "Option::is_none") )]
	pub biz_svc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktPrctc", skip_serializing_if = "Option::is_none") )]
	pub mkt_prctc: Option<ImplementationSpecification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDt") )]
	pub cre_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BizPrcgDt", skip_serializing_if = "Option::is_none") )]
	pub biz_prcg_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CpyDplct", skip_serializing_if = "Option::is_none") )]
	pub cpy_dplct: Option<CopyDuplicate1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PssblDplct", skip_serializing_if = "Option::is_none") )]
	pub pssbl_dplct: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prty", skip_serializing_if = "Option::is_none") )]
	pub prty: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sgntr", skip_serializing_if = "Option::is_none") )]
	pub sgntr: Option<SignatureEnvelope>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rltd", skip_serializing_if = "Option::is_none") )]
	pub rltd: Option<Vec<BusinessApplicationHeader5>>,
}

impl BusinessApplicationHeaderV02 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.fr.validate()?;
		self.to.validate()?;
		if self.biz_msg_idr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "biz_msg_idr is shorter than the minimum length of 1".to_string()));
		}
		if self.biz_msg_idr.chars().count() > 35 {
			return Err(ValidationError::new(1002, "biz_msg_idr exceeds the maximum length of 35".to_string()));
		}
		if self.msg_def_idr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_def_idr is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_def_idr.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_def_idr exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.biz_svc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "biz_svc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "biz_svc exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.mkt_prctc { val.validate()? }
		if let Some(ref val) = self.cpy_dplct { val.validate()? }
		if let Some(ref val) = self.sgntr { val.validate()? }
		if let Some(ref vec) = self.rltd { for item in vec { item.validate()? } }
		Ok(())
	}
}


// CSCManagement1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum CSCManagement1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRST") )]
	CodePRST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BYPS") )]
	CodeBYPS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UNRD") )]
	CodeUNRD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NCSC") )]
	CodeNCSC,
}

impl CSCManagement1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CancellationIndividualStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum CancellationIndividualStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "RJCR") )]
	CodeRJCR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACCR") )]
	CodeACCR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PDCR") )]
	CodePDCR,
}

impl CancellationIndividualStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CancellationReason33Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CancellationReason33Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl CancellationReason33Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// CancellationStatusReason3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CancellationStatusReason3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl CancellationStatusReason3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// CancellationStatusReason4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CancellationStatusReason4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Orgtr", skip_serializing_if = "Option::is_none") )]
	pub orgtr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<CancellationStatusReason3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<String>>,
}

impl CancellationStatusReason4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgtr { val.validate()? }
		if let Some(ref val) = self.rsn { val.validate()? }
		if let Some(ref vec) = self.addtl_inf {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 105 {
					return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 105".to_string()));
				}
			}
		}
		Ok(())
	}
}


// CardAggregated2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CardAggregated2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSvc", skip_serializing_if = "Option::is_none") )]
	pub addtl_svc: Option<CardPaymentServiceType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxCtgy", skip_serializing_if = "Option::is_none") )]
	pub tx_ctgy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SaleRcncltnId", skip_serializing_if = "Option::is_none") )]
	pub sale_rcncltn_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SeqNbRg", skip_serializing_if = "Option::is_none") )]
	pub seq_nb_rg: Option<CardSequenceNumberRange1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxDtRg", skip_serializing_if = "Option::is_none") )]
	pub tx_dt_rg: Option<DateOrDateTimePeriod1Choice>,
}

impl CardAggregated2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.addtl_svc { val.validate()? }
		if let Some(ref val) = self.tx_ctgy {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tx_ctgy is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "tx_ctgy exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.sale_rcncltn_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sale_rcncltn_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "sale_rcncltn_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.seq_nb_rg { val.validate()? }
		if let Some(ref val) = self.tx_dt_rg { val.validate()? }
		Ok(())
	}
}


// CardDataReading1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum CardDataReading1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "TAGC") )]
	CodeTAGC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PHYS") )]
	CodePHYS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BRCD") )]
	CodeBRCD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MGST") )]
	CodeMGST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CICC") )]
	CodeCICC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DFLE") )]
	CodeDFLE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CTLS") )]
	CodeCTLS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ECTL") )]
	CodeECTL,
}

impl CardDataReading1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CardEntry4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CardEntry4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Card", skip_serializing_if = "Option::is_none") )]
	pub card: Option<PaymentCard4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "POI", skip_serializing_if = "Option::is_none") )]
	pub poi: Option<PointOfInteraction1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AggtdNtry", skip_serializing_if = "Option::is_none") )]
	pub aggtd_ntry: Option<CardAggregated2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrePdAcct", skip_serializing_if = "Option::is_none") )]
	pub pre_pd_acct: Option<CashAccount38>,
}

impl CardEntry4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.card { val.validate()? }
		if let Some(ref val) = self.poi { val.validate()? }
		if let Some(ref val) = self.aggtd_ntry { val.validate()? }
		if let Some(ref val) = self.pre_pd_acct { val.validate()? }
		Ok(())
	}
}


// CardIndividualTransaction2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CardIndividualTransaction2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ICCRltdData", skip_serializing_if = "Option::is_none") )]
	pub icc_rltd_data: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtCntxt", skip_serializing_if = "Option::is_none") )]
	pub pmt_cntxt: Option<PaymentContext3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlSvc", skip_serializing_if = "Option::is_none") )]
	pub addtl_svc: Option<CardPaymentServiceType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxCtgy", skip_serializing_if = "Option::is_none") )]
	pub tx_ctgy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SaleRcncltnId", skip_serializing_if = "Option::is_none") )]
	pub sale_rcncltn_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SaleRefNb", skip_serializing_if = "Option::is_none") )]
	pub sale_ref_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RePresntmntRsn", skip_serializing_if = "Option::is_none") )]
	pub re_presntmnt_rsn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SeqNb", skip_serializing_if = "Option::is_none") )]
	pub seq_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxId", skip_serializing_if = "Option::is_none") )]
	pub tx_id: Option<TransactionIdentifier1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pdct", skip_serializing_if = "Option::is_none") )]
	pub pdct: Option<Product2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldtnDt", skip_serializing_if = "Option::is_none") )]
	pub vldtn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldtnSeqNb", skip_serializing_if = "Option::is_none") )]
	pub vldtn_seq_nb: Option<String>,
}

impl CardIndividualTransaction2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.icc_rltd_data {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "icc_rltd_data is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 1025 {
				return Err(ValidationError::new(1002, "icc_rltd_data exceeds the maximum length of 1025".to_string()));
			}
		}
		if let Some(ref val) = self.pmt_cntxt { val.validate()? }
		if let Some(ref val) = self.addtl_svc { val.validate()? }
		if let Some(ref val) = self.tx_ctgy {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tx_ctgy is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "tx_ctgy exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.sale_rcncltn_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sale_rcncltn_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "sale_rcncltn_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.sale_ref_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sale_ref_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "sale_ref_nb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.re_presntmnt_rsn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "re_presntmnt_rsn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "re_presntmnt_rsn exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.seq_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "seq_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "seq_nb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.tx_id { val.validate()? }
		if let Some(ref val) = self.pdct { val.validate()? }
		if let Some(ref val) = self.vldtn_seq_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "vldtn_seq_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "vldtn_seq_nb exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// CardPaymentServiceType2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum CardPaymentServiceType2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "AGGR") )]
	CodeAGGR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DCCV") )]
	CodeDCCV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GRTT") )]
	CodeGRTT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INSP") )]
	CodeINSP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LOYT") )]
	CodeLOYT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NRES") )]
	CodeNRES,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUCO") )]
	CodePUCO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RECP") )]
	CodeRECP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SOAF") )]
	CodeSOAF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UNAF") )]
	CodeUNAF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VCAU") )]
	CodeVCAU,
}

impl CardPaymentServiceType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CardSecurityInformation1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CardSecurityInformation1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CSCMgmt") )]
	pub csc_mgmt: CSCManagement1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CSCVal", skip_serializing_if = "Option::is_none") )]
	pub csc_val: Option<String>,
}

impl CardSecurityInformation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.csc_mgmt.validate()?;
		if let Some(ref val) = self.csc_val {
			let pattern = Regex::new("[0-9]{3,4}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "csc_val does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// CardSequenceNumberRange1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CardSequenceNumberRange1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrstTx", skip_serializing_if = "Option::is_none") )]
	pub frst_tx: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LastTx", skip_serializing_if = "Option::is_none") )]
	pub last_tx: Option<String>,
}

impl CardSequenceNumberRange1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.frst_tx {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "frst_tx is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "frst_tx exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.last_tx {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "last_tx is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "last_tx exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// CardTransaction17 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CardTransaction17 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Card", skip_serializing_if = "Option::is_none") )]
	pub card: Option<PaymentCard4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "POI", skip_serializing_if = "Option::is_none") )]
	pub poi: Option<PointOfInteraction1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tx", skip_serializing_if = "Option::is_none") )]
	pub tx: Option<CardTransaction3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrePdAcct", skip_serializing_if = "Option::is_none") )]
	pub pre_pd_acct: Option<CashAccount38>,
}

impl CardTransaction17 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.card { val.validate()? }
		if let Some(ref val) = self.poi { val.validate()? }
		if let Some(ref val) = self.tx { val.validate()? }
		if let Some(ref val) = self.pre_pd_acct { val.validate()? }
		Ok(())
	}
}


// CardTransaction3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CardTransaction3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Aggtd", skip_serializing_if = "Option::is_none") )]
	pub aggtd: Option<CardAggregated2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Indv", skip_serializing_if = "Option::is_none") )]
	pub indv: Option<CardIndividualTransaction2>,
}

impl CardTransaction3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.aggtd { val.validate()? }
		if let Some(ref val) = self.indv { val.validate()? }
		Ok(())
	}
}


// CardholderAuthentication2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CardholderAuthentication2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AuthntcnMtd") )]
	pub authntcn_mtd: AuthenticationMethod1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AuthntcnNtty") )]
	pub authntcn_ntty: AuthenticationEntity1Code,
}

impl CardholderAuthentication2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.authntcn_mtd.validate()?;
		self.authntcn_ntty.validate()?;
		Ok(())
	}
}


// CardholderVerificationCapability1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum CardholderVerificationCapability1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MNSG") )]
	CodeMNSG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NPIN") )]
	CodeNPIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FCPN") )]
	CodeFCPN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FEPN") )]
	CodeFEPN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FDSG") )]
	CodeFDSG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FBIO") )]
	CodeFBIO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MNVR") )]
	CodeMNVR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FBIG") )]
	CodeFBIG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "APKI") )]
	CodeAPKI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PKIS") )]
	CodePKIS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CHDT") )]
	CodeCHDT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SCEC") )]
	CodeSCEC,
}

impl CardholderVerificationCapability1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Case5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Case5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cretr") )]
	pub cretr: Party40Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReopCaseIndctn", skip_serializing_if = "Option::is_none") )]
	pub reop_case_indctn: Option<bool>,
}

impl Case5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		self.cretr.validate()?;
		Ok(())
	}
}


// CaseAssignment5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CaseAssignment5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Assgnr") )]
	pub assgnr: Party40Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Assgne") )]
	pub assgne: Party40Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
	pub cre_dt_tm: String,
}

impl CaseAssignment5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		self.assgnr.validate()?;
		self.assgne.validate()?;
		Ok(())
	}
}


// CashAccount38 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CashAccount38 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: AccountIdentification4Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<CashAccountType2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy", skip_serializing_if = "Option::is_none") )]
	pub ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prxy", skip_serializing_if = "Option::is_none") )]
	pub prxy: Option<ProxyAccountIdentification1>,
}

impl CashAccount38 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.prxy { val.validate()? }
		Ok(())
	}
}


// CashAccount39 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CashAccount39 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: AccountIdentification4Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<CashAccountType2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ccy", skip_serializing_if = "Option::is_none") )]
	pub ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prxy", skip_serializing_if = "Option::is_none") )]
	pub prxy: Option<ProxyAccountIdentification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ownr", skip_serializing_if = "Option::is_none") )]
	pub ownr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Svcr", skip_serializing_if = "Option::is_none") )]
	pub svcr: Option<BranchAndFinancialInstitutionIdentification6>,
}

impl CashAccount39 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.prxy { val.validate()? }
		if let Some(ref val) = self.ownr { val.validate()? }
		if let Some(ref val) = self.svcr { val.validate()? }
		Ok(())
	}
}


// CashAccountType2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CashAccountType2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl CashAccountType2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// CashAvailability1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CashAvailability1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt") )]
	pub dt: CashAvailabilityDate1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd") )]
	pub cdt_dbt_ind: CreditDebitCode,
}

impl CashAvailability1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.dt.validate()?;
		self.amt.validate()?;
		self.cdt_dbt_ind.validate()?;
		Ok(())
	}
}


// CashAvailabilityDate1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CashAvailabilityDate1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfDays", skip_serializing_if = "Option::is_none") )]
	pub nb_of_days: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ActlDt", skip_serializing_if = "Option::is_none") )]
	pub actl_dt: Option<String>,
}

impl CashAvailabilityDate1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nb_of_days {
			let pattern = Regex::new("[\\+]{0,1}[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "nb_of_days does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// CashBalance8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CashBalance8 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: BalanceType13,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtLine", skip_serializing_if = "Option::is_none") )]
	pub cdt_line: Option<Vec<CreditLine3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd") )]
	pub cdt_dbt_ind: CreditDebitCode,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt") )]
	pub dt: DateAndDateTime2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Avlbty", skip_serializing_if = "Option::is_none") )]
	pub avlbty: Option<Vec<CashAvailability1>>,
}

impl CashBalance8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tp.validate()?;
		if let Some(ref vec) = self.cdt_line { for item in vec { item.validate()? } }
		self.amt.validate()?;
		self.cdt_dbt_ind.validate()?;
		self.dt.validate()?;
		if let Some(ref vec) = self.avlbty { for item in vec { item.validate()? } }
		Ok(())
	}
}


// CashDeposit1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CashDeposit1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NoteDnmtn") )]
	pub note_dnmtn: ActiveCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfNotes") )]
	pub nb_of_notes: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveCurrencyAndAmount,
}

impl CashDeposit1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.note_dnmtn.validate()?;
		let pattern = Regex::new("[0-9]{1,15}").unwrap();
		if !pattern.is_match(&self.nb_of_notes) {
			return Err(ValidationError::new(1005, "nb_of_notes does not match the required pattern".to_string()));
		}
		self.amt.validate()?;
		Ok(())
	}
}


// CategoryPurpose1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CategoryPurpose1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl CategoryPurpose1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// ChargeBearerType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ChargeBearerType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DEBT") )]
	CodeDEBT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRED") )]
	CodeCRED,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SHAR") )]
	CodeSHAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SLEV") )]
	CodeSLEV,
}

impl ChargeBearerType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ChargeType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ChargeType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification3>,
}

impl ChargeType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// Charges6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Charges6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlChrgsAndTaxAmt", skip_serializing_if = "Option::is_none") )]
	pub ttl_chrgs_and_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rcrd", skip_serializing_if = "Option::is_none") )]
	pub rcrd: Option<Vec<ChargesRecord3>>,
}

impl Charges6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ttl_chrgs_and_tax_amt { val.validate()? }
		if let Some(ref vec) = self.rcrd { for item in vec { item.validate()? } }
		Ok(())
	}
}


// Charges7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Charges7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Agt") )]
	pub agt: BranchAndFinancialInstitutionIdentification6,
}

impl Charges7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		self.agt.validate()?;
		Ok(())
	}
}


// ChargesRecord3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ChargesRecord3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none") )]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgInclInd", skip_serializing_if = "Option::is_none") )]
	pub chrg_incl_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<ChargeType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
	pub rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Br", skip_serializing_if = "Option::is_none") )]
	pub br: Option<ChargeBearerType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Agt", skip_serializing_if = "Option::is_none") )]
	pub agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tax", skip_serializing_if = "Option::is_none") )]
	pub tax: Option<TaxCharges2>,
}

impl ChargesRecord3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		if let Some(ref val) = self.cdt_dbt_ind { val.validate()? }
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.br { val.validate()? }
		if let Some(ref val) = self.agt { val.validate()? }
		if let Some(ref val) = self.tax { val.validate()? }
		Ok(())
	}
}


// Cheque11 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Cheque11 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChqTp", skip_serializing_if = "Option::is_none") )]
	pub chq_tp: Option<ChequeType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChqNb", skip_serializing_if = "Option::is_none") )]
	pub chq_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChqFr", skip_serializing_if = "Option::is_none") )]
	pub chq_fr: Option<NameAndAddress16>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DlvryMtd", skip_serializing_if = "Option::is_none") )]
	pub dlvry_mtd: Option<ChequeDeliveryMethod1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DlvrTo", skip_serializing_if = "Option::is_none") )]
	pub dlvr_to: Option<NameAndAddress16>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none") )]
	pub instr_prty: Option<Priority2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChqMtrtyDt", skip_serializing_if = "Option::is_none") )]
	pub chq_mtrty_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrmsCd", skip_serializing_if = "Option::is_none") )]
	pub frms_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MemoFld", skip_serializing_if = "Option::is_none") )]
	pub memo_fld: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RgnlClrZone", skip_serializing_if = "Option::is_none") )]
	pub rgnl_clr_zone: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtLctn", skip_serializing_if = "Option::is_none") )]
	pub prt_lctn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sgntr", skip_serializing_if = "Option::is_none") )]
	pub sgntr: Option<Vec<String>>,
}

impl Cheque11 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.chq_tp { val.validate()? }
		if let Some(ref val) = self.chq_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "chq_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "chq_nb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.chq_fr { val.validate()? }
		if let Some(ref val) = self.dlvry_mtd { val.validate()? }
		if let Some(ref val) = self.dlvr_to { val.validate()? }
		if let Some(ref val) = self.instr_prty { val.validate()? }
		if let Some(ref val) = self.frms_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "frms_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "frms_cd exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.memo_fld {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "memo_fld is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 35 {
					return Err(ValidationError::new(1002, "memo_fld exceeds the maximum length of 35".to_string()));
				}
			}
		}
		if let Some(ref val) = self.rgnl_clr_zone {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rgnl_clr_zone is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "rgnl_clr_zone exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.prt_lctn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prt_lctn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prt_lctn exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.sgntr {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "sgntr is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 70 {
					return Err(ValidationError::new(1002, "sgntr exceeds the maximum length of 70".to_string()));
				}
			}
		}
		Ok(())
	}
}


// ChequeDelivery1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ChequeDelivery1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MLDB") )]
	CodeMLDB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MLCD") )]
	CodeMLCD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MLFA") )]
	CodeMLFA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRDB") )]
	CodeCRDB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRCD") )]
	CodeCRCD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRFA") )]
	CodeCRFA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUDB") )]
	CodePUDB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUCD") )]
	CodePUCD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUFA") )]
	CodePUFA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RGDB") )]
	CodeRGDB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RGCD") )]
	CodeRGCD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RGFA") )]
	CodeRGFA,
}

impl ChequeDelivery1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ChequeDeliveryMethod1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ChequeDeliveryMethod1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<ChequeDelivery1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ChequeDeliveryMethod1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// ChequeType2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ChequeType2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CCHQ") )]
	CodeCCHQ,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CCCH") )]
	CodeCCCH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BCHQ") )]
	CodeBCHQ,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DRFT") )]
	CodeDRFT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ELDR") )]
	CodeELDR,
}

impl ChequeType2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ClaimNonReceipt2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ClaimNonReceipt2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtPrcd") )]
	pub dt_prcd: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlNxtAgt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_nxt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
}

impl ClaimNonReceipt2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_nxt_agt { val.validate()? }
		Ok(())
	}
}


// ClaimNonReceipt2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ClaimNonReceipt2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Accptd", skip_serializing_if = "Option::is_none") )]
	pub accptd: Option<ClaimNonReceipt2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rjctd", skip_serializing_if = "Option::is_none") )]
	pub rjctd: Option<ClaimNonReceiptRejectReason1Choice>,
}

impl ClaimNonReceipt2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.accptd { val.validate()? }
		if let Some(ref val) = self.rjctd { val.validate()? }
		Ok(())
	}
}


// ClaimNonReceiptRejectReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ClaimNonReceiptRejectReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ClaimNonReceiptRejectReason1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// ClearingChannel2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum ClearingChannel2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "RTGS") )]
	CodeRTGS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RTNS") )]
	CodeRTNS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MPNS") )]
	CodeMPNS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BOOK") )]
	CodeBOOK,
}

impl ClearingChannel2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ClearingSystemIdentification2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ClearingSystemIdentification2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ClearingSystemIdentification2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 5 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 5".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// ClearingSystemIdentification3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ClearingSystemIdentification3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ClearingSystemIdentification3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 3 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 3".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// ClearingSystemMemberIdentification2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ClearingSystemMemberIdentification2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysId", skip_serializing_if = "Option::is_none") )]
	pub clr_sys_id: Option<ClearingSystemIdentification2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MmbId") )]
	pub mmb_id: String,
}

impl ClearingSystemMemberIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.clr_sys_id { val.validate()? }
		if self.mmb_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "mmb_id is shorter than the minimum length of 1".to_string()));
		}
		if self.mmb_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "mmb_id exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// Compensation2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Compensation2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt") )]
	pub dbtr_agt: BranchAndFinancialInstitutionIdentification6,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt") )]
	pub cdtr_agt: BranchAndFinancialInstitutionIdentification6,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn") )]
	pub rsn: CompensationReason1Choice,
}

impl Compensation2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		self.dbtr_agt.validate()?;
		self.cdtr_agt.validate()?;
		self.rsn.validate()?;
		Ok(())
	}
}


// CompensationReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CompensationReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl CompensationReason1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// Contact4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Contact4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none") )]
	pub nm_prfx: Option<NamePrefix2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PhneNb", skip_serializing_if = "Option::is_none") )]
	pub phne_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MobNb", skip_serializing_if = "Option::is_none") )]
	pub mob_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FaxNb", skip_serializing_if = "Option::is_none") )]
	pub fax_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none") )]
	pub email_adr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EmailPurp", skip_serializing_if = "Option::is_none") )]
	pub email_purp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "JobTitl", skip_serializing_if = "Option::is_none") )]
	pub job_titl: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rspnsblty", skip_serializing_if = "Option::is_none") )]
	pub rspnsblty: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dept", skip_serializing_if = "Option::is_none") )]
	pub dept: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<OtherContact1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrefrdMtd", skip_serializing_if = "Option::is_none") )]
	pub prefrd_mtd: Option<PreferredContactMethod1Code>,
}

impl Contact4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm_prfx { val.validate()? }
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.phne_nb {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "phne_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.mob_nb {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "mob_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.fax_nb {
			let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "fax_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.email_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "email_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 2048 {
				return Err(ValidationError::new(1002, "email_adr exceeds the maximum length of 2048".to_string()));
			}
		}
		if let Some(ref val) = self.email_purp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "email_purp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "email_purp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.job_titl {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "job_titl is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "job_titl exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.rspnsblty {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rspnsblty is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "rspnsblty exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.dept {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dept is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "dept exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref vec) = self.othr { for item in vec { item.validate()? } }
		if let Some(ref val) = self.prefrd_mtd { val.validate()? }
		Ok(())
	}
}


// ControlData1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ControlData1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxs") )]
	pub nb_of_txs: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrlSum", skip_serializing_if = "Option::is_none") )]
	pub ctrl_sum: Option<f64>,
}

impl ControlData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{1,15}").unwrap();
		if !pattern.is_match(&self.nb_of_txs) {
			return Err(ValidationError::new(1005, "nb_of_txs does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// CopyDuplicate1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum CopyDuplicate1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CODU") )]
	CodeCODU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "COPY") )]
	CodeCOPY,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DUPL") )]
	CodeDUPL,
}

impl CopyDuplicate1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CorporateAction9 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CorporateAction9 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "EvtTp") )]
	pub evt_tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EvtId") )]
	pub evt_id: String,
}

impl CorporateAction9 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.evt_tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "evt_tp is shorter than the minimum length of 1".to_string()));
		}
		if self.evt_tp.chars().count() > 35 {
			return Err(ValidationError::new(1002, "evt_tp exceeds the maximum length of 35".to_string()));
		}
		if self.evt_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "evt_id is shorter than the minimum length of 1".to_string()));
		}
		if self.evt_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "evt_id exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// CorrectiveGroupInformation1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CorrectiveGroupInformation1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgNmId") )]
	pub msg_nm_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
	pub cre_dt_tm: Option<String>,
}

impl CorrectiveGroupInformation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if self.msg_nm_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_nm_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_nm_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_nm_id exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// CorrectiveInterbankTransaction2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CorrectiveInterbankTransaction2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrpHdr", skip_serializing_if = "Option::is_none") )]
	pub grp_hdr: Option<CorrectiveGroupInformation1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrId", skip_serializing_if = "Option::is_none") )]
	pub instr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none") )]
	pub end_to_end_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxId", skip_serializing_if = "Option::is_none") )]
	pub tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UETR", skip_serializing_if = "Option::is_none") )]
	pub uetr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmAmt") )]
	pub intr_bk_sttlm_amt: ActiveOrHistoricCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmDt") )]
	pub intr_bk_sttlm_dt: String,
}

impl CorrectiveInterbankTransaction2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.grp_hdr { val.validate()? }
		if let Some(ref val) = self.instr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "instr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "instr_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.end_to_end_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "end_to_end_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "end_to_end_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.tx_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tx_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tx_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.uetr {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "uetr does not match the required pattern".to_string()));
			}
		}
		self.intr_bk_sttlm_amt.validate()?;
		Ok(())
	}
}


// CorrectivePaymentInitiation4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CorrectivePaymentInitiation4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrpHdr", skip_serializing_if = "Option::is_none") )]
	pub grp_hdr: Option<CorrectiveGroupInformation1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtInfId", skip_serializing_if = "Option::is_none") )]
	pub pmt_inf_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrId", skip_serializing_if = "Option::is_none") )]
	pub instr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none") )]
	pub end_to_end_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UETR", skip_serializing_if = "Option::is_none") )]
	pub uetr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAmt") )]
	pub instd_amt: ActiveOrHistoricCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none") )]
	pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdColltnDt", skip_serializing_if = "Option::is_none") )]
	pub reqd_colltn_dt: Option<String>,
}

impl CorrectivePaymentInitiation4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.grp_hdr { val.validate()? }
		if let Some(ref val) = self.pmt_inf_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pmt_inf_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "pmt_inf_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.instr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "instr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "instr_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.end_to_end_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "end_to_end_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "end_to_end_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.uetr {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "uetr does not match the required pattern".to_string()));
			}
		}
		self.instd_amt.validate()?;
		if let Some(ref val) = self.reqd_exctn_dt { val.validate()? }
		Ok(())
	}
}


// CorrectiveTransaction4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CorrectiveTransaction4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Initn", skip_serializing_if = "Option::is_none") )]
	pub initn: Option<CorrectivePaymentInitiation4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBk", skip_serializing_if = "Option::is_none") )]
	pub intr_bk: Option<CorrectiveInterbankTransaction2>,
}

impl CorrectiveTransaction4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.initn { val.validate()? }
		if let Some(ref val) = self.intr_bk { val.validate()? }
		Ok(())
	}
}


// CreditDebitCode ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum CreditDebitCode {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRDT") )]
	CodeCRDT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DBIT") )]
	CodeDBIT,
}

impl CreditDebitCode {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// CreditLine3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CreditLine3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Incl") )]
	pub incl: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<CreditLineType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<DateAndDateTime2Choice>,
}

impl CreditLine3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.amt { val.validate()? }
		if let Some(ref val) = self.dt { val.validate()? }
		Ok(())
	}
}


// CreditLineType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CreditLineType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl CreditLineType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// CreditTransferMandateData1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CreditTransferMandateData1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtId", skip_serializing_if = "Option::is_none") )]
	pub mndt_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<MandateTypeInformation2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtOfSgntr", skip_serializing_if = "Option::is_none") )]
	pub dt_of_sgntr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtOfVrfctn", skip_serializing_if = "Option::is_none") )]
	pub dt_of_vrfctn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ElctrncSgntr", skip_serializing_if = "Option::is_none") )]
	pub elctrnc_sgntr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrstPmtDt", skip_serializing_if = "Option::is_none") )]
	pub frst_pmt_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FnlPmtDt", skip_serializing_if = "Option::is_none") )]
	pub fnl_pmt_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Frqcy", skip_serializing_if = "Option::is_none") )]
	pub frqcy: Option<Frequency36Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<MandateSetupReason1Choice>,
}

impl CreditTransferMandateData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mndt_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mndt_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mndt_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.elctrnc_sgntr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "elctrnc_sgntr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 10240 {
				return Err(ValidationError::new(1002, "elctrnc_sgntr exceeds the maximum length of 10240".to_string()));
			}
		}
		if let Some(ref val) = self.frqcy { val.validate()? }
		if let Some(ref val) = self.rsn { val.validate()? }
		Ok(())
	}
}


// CreditTransferTransaction35 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CreditTransferTransaction35 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtId") )]
	pub pmt_id: PaymentIdentification6,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none") )]
	pub pmt_tp_inf: Option<PaymentTypeInformation26>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtCond", skip_serializing_if = "Option::is_none") )]
	pub pmt_cond: Option<PaymentCondition1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: AmountType4Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgBr") )]
	pub chrg_br: ChargeBearerType1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChqInstr", skip_serializing_if = "Option::is_none") )]
	pub chq_instr: Option<Cheque11>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt") )]
	pub cdtr_agt: BranchAndFinancialInstitutionIdentification6,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr") )]
	pub cdtr: PartyIdentification135,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrForCdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub instr_for_cdtr_agt: Option<Vec<InstructionForCreditorAgent1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
	pub purp: Option<Purpose2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RgltryRptg", skip_serializing_if = "Option::is_none") )]
	pub rgltry_rptg: Option<Vec<RegulatoryReporting3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tax", skip_serializing_if = "Option::is_none") )]
	pub tax: Option<TaxInformation8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdRmtInf", skip_serializing_if = "Option::is_none") )]
	pub rltd_rmt_inf: Option<Vec<RemittanceLocation7>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtInf", skip_serializing_if = "Option::is_none") )]
	pub rmt_inf: Option<RemittanceInformation16>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NclsdFile", skip_serializing_if = "Option::is_none") )]
	pub nclsd_file: Option<Vec<Document12>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CreditTransferTransaction35 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pmt_id.validate()?;
		if let Some(ref val) = self.pmt_tp_inf { val.validate()? }
		if let Some(ref val) = self.pmt_cond { val.validate()? }
		self.amt.validate()?;
		self.chrg_br.validate()?;
		if let Some(ref val) = self.chq_instr { val.validate()? }
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.intrmy_agt1 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt2 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt3 { val.validate()? }
		self.cdtr_agt.validate()?;
		self.cdtr.validate()?;
		if let Some(ref val) = self.cdtr_acct { val.validate()? }
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		if let Some(ref vec) = self.instr_for_cdtr_agt { for item in vec { item.validate()? } }
		if let Some(ref val) = self.purp { val.validate()? }
		if let Some(ref vec) = self.rgltry_rptg { for item in vec { item.validate()? } }
		if let Some(ref val) = self.tax { val.validate()? }
		if let Some(ref vec) = self.rltd_rmt_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.rmt_inf { val.validate()? }
		if let Some(ref vec) = self.nclsd_file { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// CreditTransferTransaction36 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CreditTransferTransaction36 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtId") )]
	pub pmt_id: PaymentIdentification7,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none") )]
	pub pmt_tp_inf: Option<PaymentTypeInformation28>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmAmt") )]
	pub intr_bk_sttlm_amt: ActiveCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none") )]
	pub intr_bk_sttlm_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmPrty", skip_serializing_if = "Option::is_none") )]
	pub sttlm_prty: Option<Priority3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmTmIndctn", skip_serializing_if = "Option::is_none") )]
	pub sttlm_tm_indctn: Option<SettlementDateTimeIndication1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmTmReq", skip_serializing_if = "Option::is_none") )]
	pub sttlm_tm_req: Option<SettlementTimeRequest2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt1", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt1Acct", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt1_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt2", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt2Acct", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt2_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt3", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt3Acct", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt3_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none") )]
	pub instg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none") )]
	pub instd_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt1Acct", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt1_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt2Acct", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt2_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt3Acct", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt3_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr") )]
	pub dbtr: BranchAndFinancialInstitutionIdentification6,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr") )]
	pub cdtr: BranchAndFinancialInstitutionIdentification6,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrForCdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub instr_for_cdtr_agt: Option<Vec<InstructionForCreditorAgent2>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrForNxtAgt", skip_serializing_if = "Option::is_none") )]
	pub instr_for_nxt_agt: Option<Vec<InstructionForNextAgent1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
	pub purp: Option<Purpose2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtInf", skip_serializing_if = "Option::is_none") )]
	pub rmt_inf: Option<RemittanceInformation2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygCstmrCdtTrf", skip_serializing_if = "Option::is_none") )]
	pub undrlyg_cstmr_cdt_trf: Option<CreditTransferTransaction37>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CreditTransferTransaction36 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pmt_id.validate()?;
		if let Some(ref val) = self.pmt_tp_inf { val.validate()? }
		self.intr_bk_sttlm_amt.validate()?;
		if let Some(ref val) = self.sttlm_prty { val.validate()? }
		if let Some(ref val) = self.sttlm_tm_indctn { val.validate()? }
		if let Some(ref val) = self.sttlm_tm_req { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt1 { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt1_acct { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt2 { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt2_acct { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt3 { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt3_acct { val.validate()? }
		if let Some(ref val) = self.instg_agt { val.validate()? }
		if let Some(ref val) = self.instd_agt { val.validate()? }
		if let Some(ref val) = self.intrmy_agt1 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt1_acct { val.validate()? }
		if let Some(ref val) = self.intrmy_agt2 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt2_acct { val.validate()? }
		if let Some(ref val) = self.intrmy_agt3 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt3_acct { val.validate()? }
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		self.dbtr.validate()?;
		if let Some(ref val) = self.dbtr_acct { val.validate()? }
		if let Some(ref val) = self.dbtr_agt { val.validate()? }
		if let Some(ref val) = self.dbtr_agt_acct { val.validate()? }
		if let Some(ref val) = self.cdtr_agt { val.validate()? }
		if let Some(ref val) = self.cdtr_agt_acct { val.validate()? }
		self.cdtr.validate()?;
		if let Some(ref val) = self.cdtr_acct { val.validate()? }
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		if let Some(ref vec) = self.instr_for_cdtr_agt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.instr_for_nxt_agt { for item in vec { item.validate()? } }
		if let Some(ref val) = self.purp { val.validate()? }
		if let Some(ref val) = self.rmt_inf { val.validate()? }
		if let Some(ref val) = self.undrlyg_cstmr_cdt_trf { val.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// CreditTransferTransaction37 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CreditTransferTransaction37 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitgPty", skip_serializing_if = "Option::is_none") )]
	pub initg_pty: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr") )]
	pub dbtr: PartyIdentification135,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt") )]
	pub dbtr_agt: BranchAndFinancialInstitutionIdentification6,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt1", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt1Acct", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt1_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt2", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt2Acct", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt2_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt3", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt3Acct", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt3_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt1Acct", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt1_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt2Acct", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt2_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt3Acct", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt3_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt") )]
	pub cdtr_agt: BranchAndFinancialInstitutionIdentification6,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr") )]
	pub cdtr: PartyIdentification135,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrForCdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub instr_for_cdtr_agt: Option<Vec<InstructionForCreditorAgent1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrForNxtAgt", skip_serializing_if = "Option::is_none") )]
	pub instr_for_nxt_agt: Option<Vec<InstructionForNextAgent1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tax", skip_serializing_if = "Option::is_none") )]
	pub tax: Option<TaxInformation8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtInf", skip_serializing_if = "Option::is_none") )]
	pub rmt_inf: Option<RemittanceInformation16>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none") )]
	pub instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl CreditTransferTransaction37 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.initg_pty { val.validate()? }
		self.dbtr.validate()?;
		if let Some(ref val) = self.dbtr_acct { val.validate()? }
		self.dbtr_agt.validate()?;
		if let Some(ref val) = self.dbtr_agt_acct { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt1 { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt1_acct { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt2 { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt2_acct { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt3 { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt3_acct { val.validate()? }
		if let Some(ref val) = self.intrmy_agt1 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt1_acct { val.validate()? }
		if let Some(ref val) = self.intrmy_agt2 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt2_acct { val.validate()? }
		if let Some(ref val) = self.intrmy_agt3 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt3_acct { val.validate()? }
		self.cdtr_agt.validate()?;
		if let Some(ref val) = self.cdtr_agt_acct { val.validate()? }
		self.cdtr.validate()?;
		if let Some(ref val) = self.cdtr_acct { val.validate()? }
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		if let Some(ref vec) = self.instr_for_cdtr_agt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.instr_for_nxt_agt { for item in vec { item.validate()? } }
		if let Some(ref val) = self.tax { val.validate()? }
		if let Some(ref val) = self.rmt_inf { val.validate()? }
		if let Some(ref val) = self.instd_amt { val.validate()? }
		Ok(())
	}
}


// CreditTransferTransaction39 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CreditTransferTransaction39 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtId") )]
	pub pmt_id: PaymentIdentification7,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none") )]
	pub pmt_tp_inf: Option<PaymentTypeInformation28>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmAmt") )]
	pub intr_bk_sttlm_amt: ActiveCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none") )]
	pub intr_bk_sttlm_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmPrty", skip_serializing_if = "Option::is_none") )]
	pub sttlm_prty: Option<Priority3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmTmIndctn", skip_serializing_if = "Option::is_none") )]
	pub sttlm_tm_indctn: Option<SettlementDateTimeIndication1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmTmReq", skip_serializing_if = "Option::is_none") )]
	pub sttlm_tm_req: Option<SettlementTimeRequest2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AccptncDtTm", skip_serializing_if = "Option::is_none") )]
	pub accptnc_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PoolgAdjstmntDt", skip_serializing_if = "Option::is_none") )]
	pub poolg_adjstmnt_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none") )]
	pub instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XchgRate", skip_serializing_if = "Option::is_none") )]
	pub xchg_rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgBr") )]
	pub chrg_br: ChargeBearerType1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgsInf", skip_serializing_if = "Option::is_none") )]
	pub chrgs_inf: Option<Vec<Charges7>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt1", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt1Acct", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt1_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt2", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt2Acct", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt2_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt3", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt3Acct", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt3_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none") )]
	pub instg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none") )]
	pub instd_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt1Acct", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt1_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt2Acct", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt2_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt3Acct", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt3_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitgPty", skip_serializing_if = "Option::is_none") )]
	pub initg_pty: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr") )]
	pub dbtr: PartyIdentification135,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt") )]
	pub dbtr_agt: BranchAndFinancialInstitutionIdentification6,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt") )]
	pub cdtr_agt: BranchAndFinancialInstitutionIdentification6,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr") )]
	pub cdtr: PartyIdentification135,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrForCdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub instr_for_cdtr_agt: Option<Vec<InstructionForCreditorAgent1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrForNxtAgt", skip_serializing_if = "Option::is_none") )]
	pub instr_for_nxt_agt: Option<Vec<InstructionForNextAgent1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
	pub purp: Option<Purpose2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RgltryRptg", skip_serializing_if = "Option::is_none") )]
	pub rgltry_rptg: Option<Vec<RegulatoryReporting3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tax", skip_serializing_if = "Option::is_none") )]
	pub tax: Option<TaxInformation8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdRmtInf", skip_serializing_if = "Option::is_none") )]
	pub rltd_rmt_inf: Option<Vec<RemittanceLocation7>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtInf", skip_serializing_if = "Option::is_none") )]
	pub rmt_inf: Option<RemittanceInformation16>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl CreditTransferTransaction39 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pmt_id.validate()?;
		if let Some(ref val) = self.pmt_tp_inf { val.validate()? }
		self.intr_bk_sttlm_amt.validate()?;
		if let Some(ref val) = self.sttlm_prty { val.validate()? }
		if let Some(ref val) = self.sttlm_tm_indctn { val.validate()? }
		if let Some(ref val) = self.sttlm_tm_req { val.validate()? }
		if let Some(ref val) = self.instd_amt { val.validate()? }
		self.chrg_br.validate()?;
		if let Some(ref vec) = self.chrgs_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.prvs_instg_agt1 { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt1_acct { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt2 { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt2_acct { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt3 { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt3_acct { val.validate()? }
		if let Some(ref val) = self.instg_agt { val.validate()? }
		if let Some(ref val) = self.instd_agt { val.validate()? }
		if let Some(ref val) = self.intrmy_agt1 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt1_acct { val.validate()? }
		if let Some(ref val) = self.intrmy_agt2 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt2_acct { val.validate()? }
		if let Some(ref val) = self.intrmy_agt3 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt3_acct { val.validate()? }
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.initg_pty { val.validate()? }
		self.dbtr.validate()?;
		if let Some(ref val) = self.dbtr_acct { val.validate()? }
		self.dbtr_agt.validate()?;
		if let Some(ref val) = self.dbtr_agt_acct { val.validate()? }
		self.cdtr_agt.validate()?;
		if let Some(ref val) = self.cdtr_agt_acct { val.validate()? }
		self.cdtr.validate()?;
		if let Some(ref val) = self.cdtr_acct { val.validate()? }
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		if let Some(ref vec) = self.instr_for_cdtr_agt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.instr_for_nxt_agt { for item in vec { item.validate()? } }
		if let Some(ref val) = self.purp { val.validate()? }
		if let Some(ref vec) = self.rgltry_rptg { for item in vec { item.validate()? } }
		if let Some(ref val) = self.tax { val.validate()? }
		if let Some(ref vec) = self.rltd_rmt_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.rmt_inf { val.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// CreditTransferTransaction45 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CreditTransferTransaction45 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitgPty", skip_serializing_if = "Option::is_none") )]
	pub initg_pty: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr") )]
	pub dbtr: PartyIdentification135,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt") )]
	pub dbtr_agt: BranchAndFinancialInstitutionIdentification6,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt1", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt1Acct", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt1_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt2", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt2Acct", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt2_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt3", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt3Acct", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt3_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt1Acct", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt1_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt2Acct", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt2_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt3Acct", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt3_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt") )]
	pub cdtr_agt: BranchAndFinancialInstitutionIdentification6,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr") )]
	pub cdtr: PartyIdentification135,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrForCdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub instr_for_cdtr_agt: Option<Vec<InstructionForCreditorAgent3>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrForNxtAgt", skip_serializing_if = "Option::is_none") )]
	pub instr_for_nxt_agt: Option<Vec<InstructionForNextAgent1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tax", skip_serializing_if = "Option::is_none") )]
	pub tax: Option<TaxInformation8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtInf", skip_serializing_if = "Option::is_none") )]
	pub rmt_inf: Option<RemittanceInformation16>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAmt", skip_serializing_if = "Option::is_none") )]
	pub instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl CreditTransferTransaction45 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.initg_pty { val.validate()? }
		self.dbtr.validate()?;
		if let Some(ref val) = self.dbtr_acct { val.validate()? }
		self.dbtr_agt.validate()?;
		if let Some(ref val) = self.dbtr_agt_acct { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt1 { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt1_acct { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt2 { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt2_acct { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt3 { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt3_acct { val.validate()? }
		if let Some(ref val) = self.intrmy_agt1 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt1_acct { val.validate()? }
		if let Some(ref val) = self.intrmy_agt2 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt2_acct { val.validate()? }
		if let Some(ref val) = self.intrmy_agt3 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt3_acct { val.validate()? }
		self.cdtr_agt.validate()?;
		if let Some(ref val) = self.cdtr_agt_acct { val.validate()? }
		self.cdtr.validate()?;
		if let Some(ref val) = self.cdtr_acct { val.validate()? }
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		if let Some(ref vec) = self.instr_for_cdtr_agt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.instr_for_nxt_agt { for item in vec { item.validate()? } }
		if let Some(ref val) = self.tax { val.validate()? }
		if let Some(ref val) = self.rmt_inf { val.validate()? }
		if let Some(ref val) = self.instd_amt { val.validate()? }
		Ok(())
	}
}


// CreditorReferenceInformation2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CreditorReferenceInformation2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<CreditorReferenceType2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ref", skip_serializing_if = "Option::is_none") )]
	pub ref_attr: Option<String>,
}

impl CreditorReferenceInformation2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.ref_attr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ref_attr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ref_attr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// CreditorReferenceType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CreditorReferenceType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<DocumentType3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl CreditorReferenceType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// CreditorReferenceType2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CreditorReferenceType2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdOrPrtry") )]
	pub cd_or_prtry: CreditorReferenceType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl CreditorReferenceType2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd_or_prtry.validate()?;
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// CurrencyExchange5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct CurrencyExchange5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SrcCcy") )]
	pub src_ccy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrgtCcy", skip_serializing_if = "Option::is_none") )]
	pub trgt_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitCcy", skip_serializing_if = "Option::is_none") )]
	pub unit_ccy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XchgRate") )]
	pub xchg_rate: f64,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctId", skip_serializing_if = "Option::is_none") )]
	pub ctrct_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QtnDt", skip_serializing_if = "Option::is_none") )]
	pub qtn_dt: Option<String>,
}

impl CurrencyExchange5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.src_ccy) {
			return Err(ValidationError::new(1005, "src_ccy does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.trgt_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "trgt_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.unit_ccy {
			let pattern = Regex::new("[A-Z]{3,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "unit_ccy does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ctrct_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ctrct_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ctrct_id exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// DateAndDateTime2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DateAndDateTime2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtTm", skip_serializing_if = "Option::is_none") )]
	pub dt_tm: Option<String>,
}

impl DateAndDateTime2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DateAndPlaceOfBirth1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DateAndPlaceOfBirth1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "BirthDt") )]
	pub birth_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none") )]
	pub prvc_of_birth: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CityOfBirth") )]
	pub city_of_birth: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfBirth") )]
	pub ctry_of_birth: String,
}

impl DateAndPlaceOfBirth1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.prvc_of_birth {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prvc_of_birth is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prvc_of_birth exceeds the maximum length of 35".to_string()));
			}
		}
		if self.city_of_birth.chars().count() < 1 {
			return Err(ValidationError::new(1001, "city_of_birth is shorter than the minimum length of 1".to_string()));
		}
		if self.city_of_birth.chars().count() > 35 {
			return Err(ValidationError::new(1002, "city_of_birth exceeds the maximum length of 35".to_string()));
		}
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.ctry_of_birth) {
			return Err(ValidationError::new(1005, "ctry_of_birth does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// DateOrDateTimePeriod1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DateOrDateTimePeriod1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<DatePeriod2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtTm", skip_serializing_if = "Option::is_none") )]
	pub dt_tm: Option<DateTimePeriod1>,
}

impl DateOrDateTimePeriod1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dt { val.validate()? }
		if let Some(ref val) = self.dt_tm { val.validate()? }
		Ok(())
	}
}


// DatePeriod2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DatePeriod2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt") )]
	pub fr_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt") )]
	pub to_dt: String,
}

impl DatePeriod2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DatePeriodDetails1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DatePeriodDetails1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDt") )]
	pub fr_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDt", skip_serializing_if = "Option::is_none") )]
	pub to_dt: Option<String>,
}

impl DatePeriodDetails1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DateTimePeriod1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DateTimePeriod1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrDtTm") )]
	pub fr_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToDtTm") )]
	pub to_dt_tm: String,
}

impl DateTimePeriod1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DiscountAmountAndType1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DiscountAmountAndType1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<DiscountAmountType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}

impl DiscountAmountAndType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		self.amt.validate()?;
		Ok(())
	}
}


// DiscountAmountType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DiscountAmountType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl DiscountAmountType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// DisplayCapabilities1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DisplayCapabilities1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DispTp") )]
	pub disp_tp: UserInterface2Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfLines") )]
	pub nb_of_lines: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LineWidth") )]
	pub line_width: String,
}

impl DisplayCapabilities1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.disp_tp.validate()?;
		let pattern = Regex::new("[0-9]{1,3}").unwrap();
		if !pattern.is_match(&self.nb_of_lines) {
			return Err(ValidationError::new(1005, "nb_of_lines does not match the required pattern".to_string()));
		}
		let pattern = Regex::new("[0-9]{1,3}").unwrap();
		if !pattern.is_match(&self.line_width) {
			return Err(ValidationError::new(1005, "line_width does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// Document12 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Document12 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: DocumentType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IsseDt") )]
	pub isse_dt: DateAndDateTime2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LangCd", skip_serializing_if = "Option::is_none") )]
	pub lang_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Frmt") )]
	pub frmt: DocumentFormat1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FileNm", skip_serializing_if = "Option::is_none") )]
	pub file_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DgtlSgntr", skip_serializing_if = "Option::is_none") )]
	pub dgtl_sgntr: Option<PartyAndSignature3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nclsr") )]
	pub nclsr: String,
}

impl Document12 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tp.validate()?;
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		self.isse_dt.validate()?;
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
		}
		self.frmt.validate()?;
		if let Some(ref val) = self.file_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "file_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "file_nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.dgtl_sgntr { val.validate()? }
		if self.nclsr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nclsr is shorter than the minimum length of 1".to_string()));
		}
		if self.nclsr.chars().count() > 10485760 {
			return Err(ValidationError::new(1002, "nclsr exceeds the maximum length of 10485760".to_string()));
		}
		Ok(())
	}
}


// DocumentAdjustment1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DocumentAdjustment1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none") )]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<String>,
}

impl DocumentAdjustment1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		if let Some(ref val) = self.cdt_dbt_ind { val.validate()? }
		if let Some(ref val) = self.rsn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rsn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "rsn exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.addtl_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// DocumentFormat1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DocumentFormat1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification1>,
}

impl DocumentFormat1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// DocumentLineIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DocumentLineIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<DocumentLineType1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nb", skip_serializing_if = "Option::is_none") )]
	pub nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdDt", skip_serializing_if = "Option::is_none") )]
	pub rltd_dt: Option<String>,
}

impl DocumentLineIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nb exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// DocumentLineInformation1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DocumentLineInformation1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: Vec<DocumentLineIdentification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<RemittanceAmount3>,
}

impl DocumentLineInformation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		for item in &self.id { item.validate()? }
		if let Some(ref val) = self.desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 2048 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 2048".to_string()));
			}
		}
		if let Some(ref val) = self.amt { val.validate()? }
		Ok(())
	}
}


// DocumentLineType1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DocumentLineType1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdOrPrtry") )]
	pub cd_or_prtry: DocumentLineType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl DocumentLineType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd_or_prtry.validate()?;
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// DocumentLineType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DocumentLineType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl DocumentLineType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// DocumentType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct DocumentType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification1>,
}

impl DocumentType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// DocumentType3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum DocumentType3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "RADM") )]
	CodeRADM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RPIN") )]
	CodeRPIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FXDR") )]
	CodeFXDR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DISP") )]
	CodeDISP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUOR") )]
	CodePUOR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SCOR") )]
	CodeSCOR,
}

impl DocumentType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// DocumentType6Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum DocumentType6Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MSIN") )]
	CodeMSIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CNFA") )]
	CodeCNFA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DNFA") )]
	CodeDNFA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CINV") )]
	CodeCINV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CREN") )]
	CodeCREN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DEBN") )]
	CodeDEBN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HIRI") )]
	CodeHIRI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SBIN") )]
	CodeSBIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CMCN") )]
	CodeCMCN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SOAC") )]
	CodeSOAC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DISP") )]
	CodeDISP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BOLD") )]
	CodeBOLD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VCHR") )]
	CodeVCHR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AROI") )]
	CodeAROI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TSUT") )]
	CodeTSUT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUOR") )]
	CodePUOR,
}

impl DocumentType6Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// EntryDetails9 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct EntryDetails9 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Btch", skip_serializing_if = "Option::is_none") )]
	pub btch: Option<BatchInformation2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxDtls", skip_serializing_if = "Option::is_none") )]
	pub tx_dtls: Option<Vec<EntryTransaction10>>,
}

impl EntryDetails9 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.btch { val.validate()? }
		if let Some(ref vec) = self.tx_dtls { for item in vec { item.validate()? } }
		Ok(())
	}
}


// EntryStatus1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct EntryStatus1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl EntryStatus1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// EntryTransaction10 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct EntryTransaction10 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Refs", skip_serializing_if = "Option::is_none") )]
	pub refs: Option<TransactionReferences6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd", skip_serializing_if = "Option::is_none") )]
	pub cdt_dbt_ind: Option<CreditDebitCode>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmtDtls", skip_serializing_if = "Option::is_none") )]
	pub amt_dtls: Option<AmountAndCurrencyExchange3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Avlbty", skip_serializing_if = "Option::is_none") )]
	pub avlbty: Option<Vec<CashAvailability1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BkTxCd", skip_serializing_if = "Option::is_none") )]
	pub bk_tx_cd: Option<BankTransactionCodeStructure4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Chrgs", skip_serializing_if = "Option::is_none") )]
	pub chrgs: Option<Charges6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Intrst", skip_serializing_if = "Option::is_none") )]
	pub intrst: Option<TransactionInterest4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdPties", skip_serializing_if = "Option::is_none") )]
	pub rltd_pties: Option<TransactionParties6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdAgts", skip_serializing_if = "Option::is_none") )]
	pub rltd_agts: Option<TransactionAgents5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none") )]
	pub lcl_instrm: Option<LocalInstrument2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
	pub purp: Option<Purpose2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdRmtInf", skip_serializing_if = "Option::is_none") )]
	pub rltd_rmt_inf: Option<Vec<RemittanceLocation7>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtInf", skip_serializing_if = "Option::is_none") )]
	pub rmt_inf: Option<RemittanceInformation16>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdDts", skip_serializing_if = "Option::is_none") )]
	pub rltd_dts: Option<TransactionDates3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdPric", skip_serializing_if = "Option::is_none") )]
	pub rltd_pric: Option<TransactionPrice4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdQties", skip_serializing_if = "Option::is_none") )]
	pub rltd_qties: Option<Vec<TransactionQuantities3Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmId", skip_serializing_if = "Option::is_none") )]
	pub fin_instrm_id: Option<SecurityIdentification19>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tax", skip_serializing_if = "Option::is_none") )]
	pub tax: Option<TaxInformation8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RtrInf", skip_serializing_if = "Option::is_none") )]
	pub rtr_inf: Option<PaymentReturnReason5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CorpActn", skip_serializing_if = "Option::is_none") )]
	pub corp_actn: Option<CorporateAction9>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SfkpgAcct", skip_serializing_if = "Option::is_none") )]
	pub sfkpg_acct: Option<SecuritiesAccount19>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CshDpst", skip_serializing_if = "Option::is_none") )]
	pub csh_dpst: Option<Vec<CashDeposit1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CardTx", skip_serializing_if = "Option::is_none") )]
	pub card_tx: Option<CardTransaction17>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlTxInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_tx_inf: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl EntryTransaction10 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.refs { val.validate()? }
		if let Some(ref val) = self.amt { val.validate()? }
		if let Some(ref val) = self.cdt_dbt_ind { val.validate()? }
		if let Some(ref val) = self.amt_dtls { val.validate()? }
		if let Some(ref vec) = self.avlbty { for item in vec { item.validate()? } }
		if let Some(ref val) = self.bk_tx_cd { val.validate()? }
		if let Some(ref val) = self.chrgs { val.validate()? }
		if let Some(ref val) = self.intrst { val.validate()? }
		if let Some(ref val) = self.rltd_pties { val.validate()? }
		if let Some(ref val) = self.rltd_agts { val.validate()? }
		if let Some(ref val) = self.lcl_instrm { val.validate()? }
		if let Some(ref val) = self.purp { val.validate()? }
		if let Some(ref vec) = self.rltd_rmt_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.rmt_inf { val.validate()? }
		if let Some(ref val) = self.rltd_dts { val.validate()? }
		if let Some(ref val) = self.rltd_pric { val.validate()? }
		if let Some(ref vec) = self.rltd_qties { for item in vec { item.validate()? } }
		if let Some(ref val) = self.fin_instrm_id { val.validate()? }
		if let Some(ref val) = self.tax { val.validate()? }
		if let Some(ref val) = self.rtr_inf { val.validate()? }
		if let Some(ref val) = self.corp_actn { val.validate()? }
		if let Some(ref val) = self.sfkpg_acct { val.validate()? }
		if let Some(ref vec) = self.csh_dpst { for item in vec { item.validate()? } }
		if let Some(ref val) = self.card_tx { val.validate()? }
		if let Some(ref val) = self.addtl_tx_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_tx_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 500 {
				return Err(ValidationError::new(1002, "addtl_tx_inf exceeds the maximum length of 500".to_string()));
			}
		}
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// EquivalentAmount2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct EquivalentAmount2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CcyOfTrf") )]
	pub ccy_of_trf: String,
}

impl EquivalentAmount2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.ccy_of_trf) {
			return Err(ValidationError::new(1005, "ccy_of_trf does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// Event1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Event1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "EvtCd") )]
	pub evt_cd: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EvtParam", skip_serializing_if = "Option::is_none") )]
	pub evt_param: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EvtDesc", skip_serializing_if = "Option::is_none") )]
	pub evt_desc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EvtTm", skip_serializing_if = "Option::is_none") )]
	pub evt_tm: Option<String>,
}

impl Event1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.evt_cd.chars().count() < 1 {
			return Err(ValidationError::new(1001, "evt_cd is shorter than the minimum length of 1".to_string()));
		}
		if self.evt_cd.chars().count() > 4 {
			return Err(ValidationError::new(1002, "evt_cd exceeds the maximum length of 4".to_string()));
		}
		let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
		if !pattern.is_match(&self.evt_cd) {
			return Err(ValidationError::new(1005, "evt_cd does not match the required pattern".to_string()));
		}
		if let Some(ref vec) = self.evt_param {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "evt_param is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 35 {
					return Err(ValidationError::new(1002, "evt_param exceeds the maximum length of 35".to_string()));
				}
			}
		}
		if let Some(ref val) = self.evt_desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "evt_desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "evt_desc exceeds the maximum length of 350".to_string()));
			}
		}
		Ok(())
	}
}


// Event2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Event2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "EvtCd") )]
	pub evt_cd: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EvtParam", skip_serializing_if = "Option::is_none") )]
	pub evt_param: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EvtDesc", skip_serializing_if = "Option::is_none") )]
	pub evt_desc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EvtTm", skip_serializing_if = "Option::is_none") )]
	pub evt_tm: Option<String>,
}

impl Event2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.evt_cd.chars().count() < 1 {
			return Err(ValidationError::new(1001, "evt_cd is shorter than the minimum length of 1".to_string()));
		}
		if self.evt_cd.chars().count() > 4 {
			return Err(ValidationError::new(1002, "evt_cd exceeds the maximum length of 4".to_string()));
		}
		let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
		if !pattern.is_match(&self.evt_cd) {
			return Err(ValidationError::new(1005, "evt_cd does not match the required pattern".to_string()));
		}
		if let Some(ref vec) = self.evt_param {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "evt_param is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 35 {
					return Err(ValidationError::new(1002, "evt_param exceeds the maximum length of 35".to_string()));
				}
			}
		}
		if let Some(ref val) = self.evt_desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "evt_desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 1000 {
				return Err(ValidationError::new(1002, "evt_desc exceeds the maximum length of 1000".to_string()));
			}
		}
		Ok(())
	}
}


// FinancialIdentificationSchemeName1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialIdentificationSchemeName1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl FinancialIdentificationSchemeName1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// FinancialInstitutionIdentification18 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstitutionIdentification18 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "BICFI", skip_serializing_if = "Option::is_none") )]
	pub bicfi: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysMmbId", skip_serializing_if = "Option::is_none") )]
	pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
	pub pstl_adr: Option<PostalAddress24>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<GenericFinancialIdentification1>,
}

impl FinancialInstitutionIdentification18 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.bicfi {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "bicfi does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.clr_sys_mmb_id { val.validate()? }
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.pstl_adr { val.validate()? }
		if let Some(ref val) = self.othr { val.validate()? }
		Ok(())
	}
}


// FinancialInstrumentQuantity1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FinancialInstrumentQuantity1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Unit", skip_serializing_if = "Option::is_none") )]
	pub unit: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FaceAmt", skip_serializing_if = "Option::is_none") )]
	pub face_amt: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmtsdVal", skip_serializing_if = "Option::is_none") )]
	pub amtsd_val: Option<f64>,
}

impl FinancialInstrumentQuantity1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.face_amt {
			if *val < 0.000000 {
				return Err(ValidationError::new(1003, "face_amt is less than the minimum value of 0.000000".to_string()));
			}
		}
		if let Some(ref val) = self.amtsd_val {
			if *val < 0.000000 {
				return Err(ValidationError::new(1003, "amtsd_val is less than the minimum value of 0.000000".to_string()));
			}
		}
		Ok(())
	}
}


// FloorLimitType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum FloorLimitType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRED") )]
	CodeCRED,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DEBT") )]
	CodeDEBT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BOTH") )]
	CodeBOTH,
}

impl FloorLimitType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Frequency36Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Frequency36Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<Frequency6Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prd", skip_serializing_if = "Option::is_none") )]
	pub prd: Option<FrequencyPeriod1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtInTm", skip_serializing_if = "Option::is_none") )]
	pub pt_in_tm: Option<FrequencyAndMoment1>,
}

impl Frequency36Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.prd { val.validate()? }
		if let Some(ref val) = self.pt_in_tm { val.validate()? }
		Ok(())
	}
}


// Frequency6Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum Frequency6Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "YEAR") )]
	CodeYEAR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MNTH") )]
	CodeMNTH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QURT") )]
	CodeQURT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MIAN") )]
	CodeMIAN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "WEEK") )]
	CodeWEEK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DAIL") )]
	CodeDAIL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ADHO") )]
	CodeADHO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INDA") )]
	CodeINDA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FRTN") )]
	CodeFRTN,
}

impl Frequency6Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FrequencyAndMoment1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FrequencyAndMoment1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: Frequency6Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PtInTm") )]
	pub pt_in_tm: String,
}

impl FrequencyAndMoment1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tp.validate()?;
		let pattern = Regex::new("[0-9]{2}").unwrap();
		if !pattern.is_match(&self.pt_in_tm) {
			return Err(ValidationError::new(1005, "pt_in_tm does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// FrequencyPeriod1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FrequencyPeriod1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: Frequency6Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CntPerPrd") )]
	pub cnt_per_prd: f64,
}

impl FrequencyPeriod1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tp.validate()?;
		Ok(())
	}
}


// FromToAmountRange1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FromToAmountRange1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrAmt") )]
	pub fr_amt: AmountRangeBoundary1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToAmt") )]
	pub to_amt: AmountRangeBoundary1,
}

impl FromToAmountRange1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.fr_amt.validate()?;
		self.to_amt.validate()?;
		Ok(())
	}
}


// Garnishment3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Garnishment3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: GarnishmentType1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Grnshee", skip_serializing_if = "Option::is_none") )]
	pub grnshee: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrnshmtAdmstr", skip_serializing_if = "Option::is_none") )]
	pub grnshmt_admstr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefNb", skip_serializing_if = "Option::is_none") )]
	pub ref_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none") )]
	pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FmlyMdclInsrncInd", skip_serializing_if = "Option::is_none") )]
	pub fmly_mdcl_insrnc_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MplyeeTermntnInd", skip_serializing_if = "Option::is_none") )]
	pub mplyee_termntn_ind: Option<bool>,
}

impl Garnishment3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tp.validate()?;
		if let Some(ref val) = self.grnshee { val.validate()? }
		if let Some(ref val) = self.grnshmt_admstr { val.validate()? }
		if let Some(ref val) = self.ref_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ref_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "ref_nb exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.rmtd_amt { val.validate()? }
		Ok(())
	}
}


// GarnishmentType1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GarnishmentType1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdOrPrtry") )]
	pub cd_or_prtry: GarnishmentType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GarnishmentType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd_or_prtry.validate()?;
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// GarnishmentType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GarnishmentType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl GarnishmentType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// GenericAccountIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericAccountIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<AccountSchemeName1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericAccountIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 34 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 34".to_string()));
		}
		if let Some(ref val) = self.schme_nm { val.validate()? }
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// GenericFinancialIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericFinancialIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<FinancialIdentificationSchemeName1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericFinancialIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.schme_nm { val.validate()? }
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// GenericIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.schme_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "schme_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "schme_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// GenericIdentification3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericIdentification3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericIdentification3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// GenericIdentification30 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericIdentification30 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
	pub issr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<String>,
}

impl GenericIdentification30 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[a-zA-Z0-9]{4}").unwrap();
		if !pattern.is_match(&self.id) {
			return Err(ValidationError::new(1005, "id does not match the required pattern".to_string()));
		}
		if self.issr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
		}
		if self.issr.chars().count() > 35 {
			return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.schme_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "schme_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "schme_nm exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// GenericIdentification32 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericIdentification32 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<PartyType3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<PartyType4Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ShrtNm", skip_serializing_if = "Option::is_none") )]
	pub shrt_nm: Option<String>,
}

impl GenericIdentification32 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.issr { val.validate()? }
		if let Some(ref val) = self.shrt_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "shrt_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "shrt_nm exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// GenericIdentification36 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericIdentification36 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr") )]
	pub issr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<String>,
}

impl GenericIdentification36 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if self.issr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
		}
		if self.issr.chars().count() > 35 {
			return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.schme_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "schme_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "schme_nm exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// GenericOrganisationIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericOrganisationIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericOrganisationIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.schme_nm { val.validate()? }
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// GenericPersonIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GenericPersonIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none") )]
	pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl GenericPersonIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.schme_nm { val.validate()? }
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// GroupCancellationStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum GroupCancellationStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "PACR") )]
	CodePACR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RJCR") )]
	CodeRJCR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACCR") )]
	CodeACCR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PDCR") )]
	CodePDCR,
}

impl GroupCancellationStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// GroupHeader77 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GroupHeader77 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
	pub cre_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgSndr", skip_serializing_if = "Option::is_none") )]
	pub msg_sndr: Option<Party40Choice>,
}

impl GroupHeader77 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.msg_sndr { val.validate()? }
		Ok(())
	}
}


// GroupHeader78 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GroupHeader78 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
	pub cre_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxs") )]
	pub nb_of_txs: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrlSum", skip_serializing_if = "Option::is_none") )]
	pub ctrl_sum: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitgPty") )]
	pub initg_pty: PartyIdentification135,
}

impl GroupHeader78 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		let pattern = Regex::new("[0-9]{1,15}").unwrap();
		if !pattern.is_match(&self.nb_of_txs) {
			return Err(ValidationError::new(1005, "nb_of_txs does not match the required pattern".to_string()));
		}
		self.initg_pty.validate()?;
		Ok(())
	}
}


// GroupHeader81 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GroupHeader81 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
	pub cre_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgRcpt", skip_serializing_if = "Option::is_none") )]
	pub msg_rcpt: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgPgntn", skip_serializing_if = "Option::is_none") )]
	pub msg_pgntn: Option<Pagination1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlBizQry", skip_serializing_if = "Option::is_none") )]
	pub orgnl_biz_qry: Option<OriginalBusinessQuery1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<String>,
}

impl GroupHeader81 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.msg_rcpt { val.validate()? }
		if let Some(ref val) = self.msg_pgntn { val.validate()? }
		if let Some(ref val) = self.orgnl_biz_qry { val.validate()? }
		if let Some(ref val) = self.addtl_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 500 {
				return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 500".to_string()));
			}
		}
		Ok(())
	}
}


// GroupHeader87 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GroupHeader87 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
	pub cre_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitgPty") )]
	pub initg_pty: PartyIdentification135,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
}

impl GroupHeader87 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		self.initg_pty.validate()?;
		if let Some(ref val) = self.dbtr_agt { val.validate()? }
		if let Some(ref val) = self.cdtr_agt { val.validate()? }
		Ok(())
	}
}


// GroupHeader90 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GroupHeader90 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
	pub cre_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Authstn", skip_serializing_if = "Option::is_none") )]
	pub authstn: Option<Vec<Authorisation1Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BtchBookg", skip_serializing_if = "Option::is_none") )]
	pub btch_bookg: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxs") )]
	pub nb_of_txs: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrlSum", skip_serializing_if = "Option::is_none") )]
	pub ctrl_sum: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrpRtr", skip_serializing_if = "Option::is_none") )]
	pub grp_rtr: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlRtrdIntrBkSttlmAmt", skip_serializing_if = "Option::is_none") )]
	pub ttl_rtrd_intr_bk_sttlm_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none") )]
	pub intr_bk_sttlm_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmInf") )]
	pub sttlm_inf: SettlementInstruction7,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none") )]
	pub instg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none") )]
	pub instd_agt: Option<BranchAndFinancialInstitutionIdentification6>,
}

impl GroupHeader90 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref vec) = self.authstn { for item in vec { item.validate()? } }
		let pattern = Regex::new("[0-9]{1,15}").unwrap();
		if !pattern.is_match(&self.nb_of_txs) {
			return Err(ValidationError::new(1005, "nb_of_txs does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.ttl_rtrd_intr_bk_sttlm_amt { val.validate()? }
		self.sttlm_inf.validate()?;
		if let Some(ref val) = self.instg_agt { val.validate()? }
		if let Some(ref val) = self.instd_agt { val.validate()? }
		Ok(())
	}
}


// GroupHeader91 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GroupHeader91 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
	pub cre_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none") )]
	pub instg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none") )]
	pub instd_agt: Option<BranchAndFinancialInstitutionIdentification6>,
}

impl GroupHeader91 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.instg_agt { val.validate()? }
		if let Some(ref val) = self.instd_agt { val.validate()? }
		Ok(())
	}
}


// GroupHeader93 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct GroupHeader93 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm") )]
	pub cre_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BtchBookg", skip_serializing_if = "Option::is_none") )]
	pub btch_bookg: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxs") )]
	pub nb_of_txs: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrlSum", skip_serializing_if = "Option::is_none") )]
	pub ctrl_sum: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlIntrBkSttlmAmt", skip_serializing_if = "Option::is_none") )]
	pub ttl_intr_bk_sttlm_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none") )]
	pub intr_bk_sttlm_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmInf") )]
	pub sttlm_inf: SettlementInstruction7,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none") )]
	pub pmt_tp_inf: Option<PaymentTypeInformation28>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none") )]
	pub instg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none") )]
	pub instd_agt: Option<BranchAndFinancialInstitutionIdentification6>,
}

impl GroupHeader93 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		let pattern = Regex::new("[0-9]{1,15}").unwrap();
		if !pattern.is_match(&self.nb_of_txs) {
			return Err(ValidationError::new(1005, "nb_of_txs does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.ttl_intr_bk_sttlm_amt { val.validate()? }
		self.sttlm_inf.validate()?;
		if let Some(ref val) = self.pmt_tp_inf { val.validate()? }
		if let Some(ref val) = self.instg_agt { val.validate()? }
		if let Some(ref val) = self.instd_agt { val.validate()? }
		Ok(())
	}
}


// IdentificationSource3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct IdentificationSource3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl IdentificationSource3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// ImplementationSpecification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ImplementationSpecification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Regy") )]
	pub regy: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
}

impl ImplementationSpecification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.regy.chars().count() < 1 {
			return Err(ValidationError::new(1001, "regy is shorter than the minimum length of 1".to_string()));
		}
		if self.regy.chars().count() > 350 {
			return Err(ValidationError::new(1002, "regy exceeds the maximum length of 350".to_string()));
		}
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 2048 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 2048".to_string()));
		}
		Ok(())
	}
}


// ImpliedCurrencyAmountRange1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ImpliedCurrencyAmountRange1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrAmt", skip_serializing_if = "Option::is_none") )]
	pub fr_amt: Option<AmountRangeBoundary1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToAmt", skip_serializing_if = "Option::is_none") )]
	pub to_amt: Option<AmountRangeBoundary1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrToAmt", skip_serializing_if = "Option::is_none") )]
	pub fr_to_amt: Option<FromToAmountRange1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EQAmt", skip_serializing_if = "Option::is_none") )]
	pub eq_amt: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NEQAmt", skip_serializing_if = "Option::is_none") )]
	pub neq_amt: Option<f64>,
}

impl ImpliedCurrencyAmountRange1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.fr_amt { val.validate()? }
		if let Some(ref val) = self.to_amt { val.validate()? }
		if let Some(ref val) = self.fr_to_amt { val.validate()? }
		if let Some(ref val) = self.eq_amt {
			if *val < 0.000000 {
				return Err(ValidationError::new(1003, "eq_amt is less than the minimum value of 0.000000".to_string()));
			}
		}
		if let Some(ref val) = self.neq_amt {
			if *val < 0.000000 {
				return Err(ValidationError::new(1003, "neq_amt is less than the minimum value of 0.000000".to_string()));
			}
		}
		Ok(())
	}
}


// Instruction3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum Instruction3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CHQB") )]
	CodeCHQB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HOLD") )]
	CodeHOLD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PHOB") )]
	CodePHOB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TELB") )]
	CodeTELB,
}

impl Instruction3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Instruction4Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum Instruction4Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "PHOA") )]
	CodePHOA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TELA") )]
	CodeTELA,
}

impl Instruction4Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Instruction5Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum Instruction5Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "PHOB") )]
	CodePHOB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TELB") )]
	CodeTELB,
}

impl Instruction5Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InstructionForCreditorAgent1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InstructionForCreditorAgent1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<Instruction3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrInf", skip_serializing_if = "Option::is_none") )]
	pub instr_inf: Option<String>,
}

impl InstructionForCreditorAgent1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.instr_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "instr_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "instr_inf exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// InstructionForCreditorAgent2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InstructionForCreditorAgent2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<Instruction5Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrInf", skip_serializing_if = "Option::is_none") )]
	pub instr_inf: Option<String>,
}

impl InstructionForCreditorAgent2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.instr_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "instr_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "instr_inf exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// InstructionForCreditorAgent3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InstructionForCreditorAgent3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrInf", skip_serializing_if = "Option::is_none") )]
	pub instr_inf: Option<String>,
}

impl InstructionForCreditorAgent3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.instr_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "instr_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "instr_inf exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// InstructionForNextAgent1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InstructionForNextAgent1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<Instruction4Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrInf", skip_serializing_if = "Option::is_none") )]
	pub instr_inf: Option<String>,
}

impl InstructionForNextAgent1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.instr_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "instr_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "instr_inf exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// InterestRecord2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InterestRecord2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd") )]
	pub cdt_dbt_ind: CreditDebitCode,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<InterestType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
	pub rate: Option<Rate4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrToDt", skip_serializing_if = "Option::is_none") )]
	pub fr_to_dt: Option<DateTimePeriod1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tax", skip_serializing_if = "Option::is_none") )]
	pub tax: Option<TaxCharges2>,
}

impl InterestRecord2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		self.cdt_dbt_ind.validate()?;
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.rate { val.validate()? }
		if let Some(ref val) = self.fr_to_dt { val.validate()? }
		if let Some(ref val) = self.rsn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rsn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "rsn exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.tax { val.validate()? }
		Ok(())
	}
}


// InterestType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InterestType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<InterestType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl InterestType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// InterestType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum InterestType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "INDY") )]
	CodeINDY,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OVRN") )]
	CodeOVRN,
}

impl InterestType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// InvestigationStatus5Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct InvestigationStatus5Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Conf", skip_serializing_if = "Option::is_none") )]
	pub conf: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RjctdMod", skip_serializing_if = "Option::is_none") )]
	pub rjctd_mod: Option<Vec<ModificationStatusReason1Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DplctOf", skip_serializing_if = "Option::is_none") )]
	pub dplct_of: Option<Case5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AssgnmtCxlConf", skip_serializing_if = "Option::is_none") )]
	pub assgnmt_cxl_conf: Option<bool>,
}

impl InvestigationStatus5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.conf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "conf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "conf exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref vec) = self.rjctd_mod { for item in vec { item.validate()? } }
		if let Some(ref val) = self.dplct_of { val.validate()? }
		Ok(())
	}
}


// Limit2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Limit2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd") )]
	pub cdt_dbt_ind: FloorLimitType1Code,
}

impl Limit2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.amt.validate()?;
		self.cdt_dbt_ind.validate()?;
		Ok(())
	}
}


// LocalInstrument2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct LocalInstrument2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl LocalInstrument2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// MandateClassification1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MandateClassification1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<MandateClassification1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl MandateClassification1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// MandateClassification1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum MandateClassification1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FIXE") )]
	CodeFIXE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USGB") )]
	CodeUSGB,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VARI") )]
	CodeVARI,
}

impl MandateClassification1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// MandateRelatedData1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MandateRelatedData1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DrctDbtMndt", skip_serializing_if = "Option::is_none") )]
	pub drct_dbt_mndt: Option<MandateRelatedInformation14>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtTrfMndt", skip_serializing_if = "Option::is_none") )]
	pub cdt_trf_mndt: Option<CreditTransferMandateData1>,
}

impl MandateRelatedData1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.drct_dbt_mndt { val.validate()? }
		if let Some(ref val) = self.cdt_trf_mndt { val.validate()? }
		Ok(())
	}
}


// MandateRelatedInformation14 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MandateRelatedInformation14 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtId", skip_serializing_if = "Option::is_none") )]
	pub mndt_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtOfSgntr", skip_serializing_if = "Option::is_none") )]
	pub dt_of_sgntr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmdmntInd", skip_serializing_if = "Option::is_none") )]
	pub amdmnt_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmdmntInfDtls", skip_serializing_if = "Option::is_none") )]
	pub amdmnt_inf_dtls: Option<AmendmentInformationDetails13>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ElctrncSgntr", skip_serializing_if = "Option::is_none") )]
	pub elctrnc_sgntr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrstColltnDt", skip_serializing_if = "Option::is_none") )]
	pub frst_colltn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FnlColltnDt", skip_serializing_if = "Option::is_none") )]
	pub fnl_colltn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Frqcy", skip_serializing_if = "Option::is_none") )]
	pub frqcy: Option<Frequency36Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<MandateSetupReason1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrckgDays", skip_serializing_if = "Option::is_none") )]
	pub trckg_days: Option<String>,
}

impl MandateRelatedInformation14 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mndt_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mndt_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mndt_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.amdmnt_inf_dtls { val.validate()? }
		if let Some(ref val) = self.elctrnc_sgntr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "elctrnc_sgntr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 1025 {
				return Err(ValidationError::new(1002, "elctrnc_sgntr exceeds the maximum length of 1025".to_string()));
			}
		}
		if let Some(ref val) = self.frqcy { val.validate()? }
		if let Some(ref val) = self.rsn { val.validate()? }
		if let Some(ref val) = self.trckg_days {
			let pattern = Regex::new("[0-9]{2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "trckg_days does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// MandateSetupReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MandateSetupReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl MandateSetupReason1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 70".to_string()));
			}
		}
		Ok(())
	}
}


// MandateTypeInformation2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MandateTypeInformation2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none") )]
	pub svc_lvl: Option<ServiceLevel8Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none") )]
	pub lcl_instrm: Option<LocalInstrument2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none") )]
	pub ctgy_purp: Option<CategoryPurpose1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Clssfctn", skip_serializing_if = "Option::is_none") )]
	pub clssfctn: Option<MandateClassification1Choice>,
}

impl MandateTypeInformation2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.svc_lvl { val.validate()? }
		if let Some(ref val) = self.lcl_instrm { val.validate()? }
		if let Some(ref val) = self.ctgy_purp { val.validate()? }
		if let Some(ref val) = self.clssfctn { val.validate()? }
		Ok(())
	}
}


// MessageHeader10 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MessageHeader10 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
	pub cre_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QryNm", skip_serializing_if = "Option::is_none") )]
	pub qry_nm: Option<String>,
}

impl MessageHeader10 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.qry_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "qry_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "qry_nm exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// MessageHeader7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MessageHeader7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
	pub cre_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqTp", skip_serializing_if = "Option::is_none") )]
	pub req_tp: Option<RequestType4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlBizQry", skip_serializing_if = "Option::is_none") )]
	pub orgnl_biz_qry: Option<OriginalBusinessQuery1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QryNm", skip_serializing_if = "Option::is_none") )]
	pub qry_nm: Option<String>,
}

impl MessageHeader7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.req_tp { val.validate()? }
		if let Some(ref val) = self.orgnl_biz_qry { val.validate()? }
		if let Some(ref val) = self.qry_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "qry_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "qry_nm exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// MessageIdentification2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MessageIdentification2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none") )]
	pub msg_nm_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId", skip_serializing_if = "Option::is_none") )]
	pub msg_id: Option<String>,
}

impl MessageIdentification2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.msg_nm_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "msg_nm_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "msg_nm_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.msg_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// MessageReference ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MessageReference {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ref") )]
	pub ref_attr: String,
}

impl MessageReference {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.ref_attr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "ref_attr is shorter than the minimum length of 1".to_string()));
		}
		if self.ref_attr.chars().count() > 35 {
			return Err(ValidationError::new(1002, "ref_attr exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// MessageReference1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MessageReference1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ref") )]
	pub ref_attr: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgNm", skip_serializing_if = "Option::is_none") )]
	pub msg_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefIssr", skip_serializing_if = "Option::is_none") )]
	pub ref_issr: Option<PartyIdentification136>,
}

impl MessageReference1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.ref_attr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "ref_attr is shorter than the minimum length of 1".to_string()));
		}
		if self.ref_attr.chars().count() > 35 {
			return Err(ValidationError::new(1002, "ref_attr exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.msg_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "msg_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "msg_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ref_issr { val.validate()? }
		Ok(())
	}
}


// MissingOrIncorrectInformation3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct MissingOrIncorrectInformation3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AMLReq", skip_serializing_if = "Option::is_none") )]
	pub aml_req: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MssngInf", skip_serializing_if = "Option::is_none") )]
	pub mssng_inf: Option<Vec<UnableToApplyMissing1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IncrrctInf", skip_serializing_if = "Option::is_none") )]
	pub incrrct_inf: Option<Vec<UnableToApplyIncorrect1>>,
}

impl MissingOrIncorrectInformation3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.mssng_inf { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.incrrct_inf { for item in vec { item.validate()? } }
		Ok(())
	}
}


// ModificationStatusReason1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ModificationStatusReason1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ModificationStatusReason1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// ModificationStatusReason2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ModificationStatusReason2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Orgtr", skip_serializing_if = "Option::is_none") )]
	pub orgtr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<ModificationStatusReason1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<String>>,
}

impl ModificationStatusReason2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgtr { val.validate()? }
		if let Some(ref val) = self.rsn { val.validate()? }
		if let Some(ref vec) = self.addtl_inf {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 105 {
					return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 105".to_string()));
				}
			}
		}
		Ok(())
	}
}


// NameAndAddress16 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NameAndAddress16 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Adr") )]
	pub adr: PostalAddress24,
}

impl NameAndAddress16 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 140 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
		}
		self.adr.validate()?;
		Ok(())
	}
}


// NameAndAddress5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NameAndAddress5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm") )]
	pub nm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Adr", skip_serializing_if = "Option::is_none") )]
	pub adr: Option<PostalAddress1>,
}

impl NameAndAddress5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.nm.chars().count() < 1 {
			return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
		}
		if self.nm.chars().count() > 350 {
			return Err(ValidationError::new(1002, "nm exceeds the maximum length of 350".to_string()));
		}
		if let Some(ref val) = self.adr { val.validate()? }
		Ok(())
	}
}


// NamePrefix2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum NamePrefix2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DOCT") )]
	CodeDOCT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MADM") )]
	CodeMADM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MISS") )]
	CodeMISS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MIST") )]
	CodeMIST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MIKS") )]
	CodeMIKS,
}

impl NamePrefix2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// NumberAndSumOfTransactions1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NumberAndSumOfTransactions1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfNtries", skip_serializing_if = "Option::is_none") )]
	pub nb_of_ntries: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sum", skip_serializing_if = "Option::is_none") )]
	pub sum: Option<f64>,
}

impl NumberAndSumOfTransactions1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nb_of_ntries {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "nb_of_ntries does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// NumberAndSumOfTransactions4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NumberAndSumOfTransactions4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfNtries", skip_serializing_if = "Option::is_none") )]
	pub nb_of_ntries: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sum", skip_serializing_if = "Option::is_none") )]
	pub sum: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNetNtry", skip_serializing_if = "Option::is_none") )]
	pub ttl_net_ntry: Option<AmountAndDirection35>,
}

impl NumberAndSumOfTransactions4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nb_of_ntries {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "nb_of_ntries does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ttl_net_ntry { val.validate()? }
		Ok(())
	}
}


// NumberOfCancellationsPerStatus1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NumberOfCancellationsPerStatus1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtldNbOfTxs") )]
	pub dtld_nb_of_txs: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtldSts") )]
	pub dtld_sts: CancellationIndividualStatus1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtldCtrlSum", skip_serializing_if = "Option::is_none") )]
	pub dtld_ctrl_sum: Option<f64>,
}

impl NumberOfCancellationsPerStatus1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{1,15}").unwrap();
		if !pattern.is_match(&self.dtld_nb_of_txs) {
			return Err(ValidationError::new(1005, "dtld_nb_of_txs does not match the required pattern".to_string()));
		}
		self.dtld_sts.validate()?;
		Ok(())
	}
}


// NumberOfTransactionsPerStatus1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NumberOfTransactionsPerStatus1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtldNbOfTxs") )]
	pub dtld_nb_of_txs: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtldSts") )]
	pub dtld_sts: TransactionIndividualStatus1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtldCtrlSum", skip_serializing_if = "Option::is_none") )]
	pub dtld_ctrl_sum: Option<f64>,
}

impl NumberOfTransactionsPerStatus1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{1,15}").unwrap();
		if !pattern.is_match(&self.dtld_nb_of_txs) {
			return Err(ValidationError::new(1005, "dtld_nb_of_txs does not match the required pattern".to_string()));
		}
		self.dtld_sts.validate()?;
		Ok(())
	}
}


// NumberOfTransactionsPerStatus5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct NumberOfTransactionsPerStatus5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtldNbOfTxs") )]
	pub dtld_nb_of_txs: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtldSts") )]
	pub dtld_sts: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtldCtrlSum", skip_serializing_if = "Option::is_none") )]
	pub dtld_ctrl_sum: Option<f64>,
}

impl NumberOfTransactionsPerStatus5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{1,15}").unwrap();
		if !pattern.is_match(&self.dtld_nb_of_txs) {
			return Err(ValidationError::new(1005, "dtld_nb_of_txs does not match the required pattern".to_string()));
		}
		if self.dtld_sts.chars().count() < 1 {
			return Err(ValidationError::new(1001, "dtld_sts is shorter than the minimum length of 1".to_string()));
		}
		if self.dtld_sts.chars().count() > 4 {
			return Err(ValidationError::new(1002, "dtld_sts exceeds the maximum length of 4".to_string()));
		}
		Ok(())
	}
}


// OnLineCapability1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum OnLineCapability1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "OFLN") )]
	CodeOFLN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ONLN") )]
	CodeONLN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SMON") )]
	CodeSMON,
}

impl OnLineCapability1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OrganisationIdentification29 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OrganisationIdentification29 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
	pub any_bic: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<GenericOrganisationIdentification1>>,
}

impl OrganisationIdentification29 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.any_bic {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.othr { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OrganisationIdentificationSchemeName1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OrganisationIdentificationSchemeName1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl OrganisationIdentificationSchemeName1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// OriginalAndCurrentQuantities1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OriginalAndCurrentQuantities1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FaceAmt") )]
	pub face_amt: f64,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmtsdVal") )]
	pub amtsd_val: f64,
}

impl OriginalAndCurrentQuantities1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// OriginalBusinessQuery1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OriginalBusinessQuery1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId") )]
	pub msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgNmId", skip_serializing_if = "Option::is_none") )]
	pub msg_nm_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CreDtTm", skip_serializing_if = "Option::is_none") )]
	pub cre_dt_tm: Option<String>,
}

impl OriginalBusinessQuery1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.msg_nm_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "msg_nm_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "msg_nm_id exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// OriginalGroupHeader14 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OriginalGroupHeader14 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlGrpCxlId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_grp_cxl_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RslvdCase", skip_serializing_if = "Option::is_none") )]
	pub rslvd_case: Option<Case5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgId") )]
	pub orgnl_msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgNmId") )]
	pub orgnl_msg_nm_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none") )]
	pub orgnl_cre_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlNbOfTxs", skip_serializing_if = "Option::is_none") )]
	pub orgnl_nb_of_txs: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCtrlSum", skip_serializing_if = "Option::is_none") )]
	pub orgnl_ctrl_sum: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrpCxlSts", skip_serializing_if = "Option::is_none") )]
	pub grp_cxl_sts: Option<GroupCancellationStatus1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CxlStsRsnInf", skip_serializing_if = "Option::is_none") )]
	pub cxl_sts_rsn_inf: Option<Vec<CancellationStatusReason4>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxsPerCxlSts", skip_serializing_if = "Option::is_none") )]
	pub nb_of_txs_per_cxl_sts: Option<Vec<NumberOfTransactionsPerStatus1>>,
}

impl OriginalGroupHeader14 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_grp_cxl_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_grp_cxl_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_grp_cxl_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.rslvd_case { val.validate()? }
		if self.orgnl_msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_msg_id exceeds the maximum length of 35".to_string()));
		}
		if self.orgnl_msg_nm_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_msg_nm_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_msg_nm_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_msg_nm_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.orgnl_nb_of_txs {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_nb_of_txs does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.grp_cxl_sts { val.validate()? }
		if let Some(ref vec) = self.cxl_sts_rsn_inf { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.nb_of_txs_per_cxl_sts { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OriginalGroupHeader15 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OriginalGroupHeader15 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrpCxlId", skip_serializing_if = "Option::is_none") )]
	pub grp_cxl_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Case", skip_serializing_if = "Option::is_none") )]
	pub case: Option<Case5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgId") )]
	pub orgnl_msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgNmId") )]
	pub orgnl_msg_nm_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none") )]
	pub orgnl_cre_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxs", skip_serializing_if = "Option::is_none") )]
	pub nb_of_txs: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrlSum", skip_serializing_if = "Option::is_none") )]
	pub ctrl_sum: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrpCxl", skip_serializing_if = "Option::is_none") )]
	pub grp_cxl: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CxlRsnInf", skip_serializing_if = "Option::is_none") )]
	pub cxl_rsn_inf: Option<Vec<PaymentCancellationReason5>>,
}

impl OriginalGroupHeader15 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.grp_cxl_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "grp_cxl_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "grp_cxl_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.case { val.validate()? }
		if self.orgnl_msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_msg_id exceeds the maximum length of 35".to_string()));
		}
		if self.orgnl_msg_nm_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_msg_nm_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_msg_nm_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_msg_nm_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.nb_of_txs {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "nb_of_txs does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.cxl_rsn_inf { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OriginalGroupHeader17 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OriginalGroupHeader17 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgId") )]
	pub orgnl_msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgNmId") )]
	pub orgnl_msg_nm_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none") )]
	pub orgnl_cre_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlNbOfTxs", skip_serializing_if = "Option::is_none") )]
	pub orgnl_nb_of_txs: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCtrlSum", skip_serializing_if = "Option::is_none") )]
	pub orgnl_ctrl_sum: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrpSts", skip_serializing_if = "Option::is_none") )]
	pub grp_sts: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsRsnInf", skip_serializing_if = "Option::is_none") )]
	pub sts_rsn_inf: Option<Vec<StatusReasonInformation12>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxsPerSts", skip_serializing_if = "Option::is_none") )]
	pub nb_of_txs_per_sts: Option<Vec<NumberOfTransactionsPerStatus5>>,
}

impl OriginalGroupHeader17 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.orgnl_msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_msg_id exceeds the maximum length of 35".to_string()));
		}
		if self.orgnl_msg_nm_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_msg_nm_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_msg_nm_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_msg_nm_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.orgnl_nb_of_txs {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_nb_of_txs does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.grp_sts {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "grp_sts is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "grp_sts exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref vec) = self.sts_rsn_inf { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.nb_of_txs_per_sts { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OriginalGroupHeader18 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OriginalGroupHeader18 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgId") )]
	pub orgnl_msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgNmId") )]
	pub orgnl_msg_nm_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none") )]
	pub orgnl_cre_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RtrRsnInf", skip_serializing_if = "Option::is_none") )]
	pub rtr_rsn_inf: Option<Vec<PaymentReturnReason6>>,
}

impl OriginalGroupHeader18 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.orgnl_msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_msg_id exceeds the maximum length of 35".to_string()));
		}
		if self.orgnl_msg_nm_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_msg_nm_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_msg_nm_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_msg_nm_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref vec) = self.rtr_rsn_inf { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OriginalGroupInformation27 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OriginalGroupInformation27 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgId") )]
	pub orgnl_msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgNmId") )]
	pub orgnl_msg_nm_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none") )]
	pub orgnl_cre_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlNbOfTxs", skip_serializing_if = "Option::is_none") )]
	pub orgnl_nb_of_txs: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCtrlSum", skip_serializing_if = "Option::is_none") )]
	pub orgnl_ctrl_sum: Option<f64>,
}

impl OriginalGroupInformation27 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.orgnl_msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_msg_id exceeds the maximum length of 35".to_string()));
		}
		if self.orgnl_msg_nm_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_msg_nm_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_msg_nm_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_msg_nm_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.orgnl_nb_of_txs {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_nb_of_txs does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// OriginalGroupInformation29 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OriginalGroupInformation29 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgId") )]
	pub orgnl_msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgNmId") )]
	pub orgnl_msg_nm_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none") )]
	pub orgnl_cre_dt_tm: Option<String>,
}

impl OriginalGroupInformation29 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.orgnl_msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_msg_id exceeds the maximum length of 35".to_string()));
		}
		if self.orgnl_msg_nm_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_msg_nm_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_msg_nm_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_msg_nm_id exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// OriginalGroupInformation30 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OriginalGroupInformation30 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgId") )]
	pub orgnl_msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgNmId") )]
	pub orgnl_msg_nm_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none") )]
	pub orgnl_cre_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlNbOfTxs", skip_serializing_if = "Option::is_none") )]
	pub orgnl_nb_of_txs: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCtrlSum", skip_serializing_if = "Option::is_none") )]
	pub orgnl_ctrl_sum: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrpSts", skip_serializing_if = "Option::is_none") )]
	pub grp_sts: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsRsnInf", skip_serializing_if = "Option::is_none") )]
	pub sts_rsn_inf: Option<Vec<StatusReasonInformation12>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxsPerSts", skip_serializing_if = "Option::is_none") )]
	pub nb_of_txs_per_sts: Option<Vec<NumberOfTransactionsPerStatus5>>,
}

impl OriginalGroupInformation30 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.orgnl_msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_msg_id exceeds the maximum length of 35".to_string()));
		}
		if self.orgnl_msg_nm_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_msg_nm_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_msg_nm_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_msg_nm_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.orgnl_nb_of_txs {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_nb_of_txs does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.grp_sts {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "grp_sts is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "grp_sts exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref vec) = self.sts_rsn_inf { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.nb_of_txs_per_sts { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OriginalPaymentInstruction30 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OriginalPaymentInstruction30 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlPmtInfCxlId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_pmt_inf_cxl_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RslvdCase", skip_serializing_if = "Option::is_none") )]
	pub rslvd_case: Option<Case5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlPmtInfId") )]
	pub orgnl_pmt_inf_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none") )]
	pub orgnl_grp_inf: Option<OriginalGroupInformation29>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlNbOfTxs", skip_serializing_if = "Option::is_none") )]
	pub orgnl_nb_of_txs: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCtrlSum", skip_serializing_if = "Option::is_none") )]
	pub orgnl_ctrl_sum: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtInfCxlSts", skip_serializing_if = "Option::is_none") )]
	pub pmt_inf_cxl_sts: Option<GroupCancellationStatus1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CxlStsRsnInf", skip_serializing_if = "Option::is_none") )]
	pub cxl_sts_rsn_inf: Option<Vec<CancellationStatusReason4>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxsPerCxlSts", skip_serializing_if = "Option::is_none") )]
	pub nb_of_txs_per_cxl_sts: Option<Vec<NumberOfCancellationsPerStatus1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxInfAndSts", skip_serializing_if = "Option::is_none") )]
	pub tx_inf_and_sts: Option<Vec<PaymentTransaction103>>,
}

impl OriginalPaymentInstruction30 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_pmt_inf_cxl_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_pmt_inf_cxl_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_pmt_inf_cxl_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.rslvd_case { val.validate()? }
		if self.orgnl_pmt_inf_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_pmt_inf_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_pmt_inf_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_pmt_inf_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.orgnl_grp_inf { val.validate()? }
		if let Some(ref val) = self.orgnl_nb_of_txs {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_nb_of_txs does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pmt_inf_cxl_sts { val.validate()? }
		if let Some(ref vec) = self.cxl_sts_rsn_inf { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.nb_of_txs_per_cxl_sts { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.tx_inf_and_sts { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OriginalPaymentInstruction31 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OriginalPaymentInstruction31 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlPmtInfId") )]
	pub orgnl_pmt_inf_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlNbOfTxs", skip_serializing_if = "Option::is_none") )]
	pub orgnl_nb_of_txs: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCtrlSum", skip_serializing_if = "Option::is_none") )]
	pub orgnl_ctrl_sum: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtInfSts", skip_serializing_if = "Option::is_none") )]
	pub pmt_inf_sts: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsRsnInf", skip_serializing_if = "Option::is_none") )]
	pub sts_rsn_inf: Option<Vec<StatusReasonInformation12>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxsPerSts", skip_serializing_if = "Option::is_none") )]
	pub nb_of_txs_per_sts: Option<Vec<NumberOfTransactionsPerStatus5>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxInfAndSts", skip_serializing_if = "Option::is_none") )]
	pub tx_inf_and_sts: Option<Vec<PaymentTransaction104>>,
}

impl OriginalPaymentInstruction31 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.orgnl_pmt_inf_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_pmt_inf_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_pmt_inf_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_pmt_inf_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.orgnl_nb_of_txs {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_nb_of_txs does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.pmt_inf_sts {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pmt_inf_sts is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "pmt_inf_sts exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref vec) = self.sts_rsn_inf { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.nb_of_txs_per_sts { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.tx_inf_and_sts { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OriginalPaymentInstruction36 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OriginalPaymentInstruction36 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtCxlId", skip_serializing_if = "Option::is_none") )]
	pub pmt_cxl_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Case", skip_serializing_if = "Option::is_none") )]
	pub case: Option<Case5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlPmtInfId") )]
	pub orgnl_pmt_inf_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none") )]
	pub orgnl_grp_inf: Option<OriginalGroupInformation29>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfTxs", skip_serializing_if = "Option::is_none") )]
	pub nb_of_txs: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrlSum", skip_serializing_if = "Option::is_none") )]
	pub ctrl_sum: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtInfCxl", skip_serializing_if = "Option::is_none") )]
	pub pmt_inf_cxl: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CxlRsnInf", skip_serializing_if = "Option::is_none") )]
	pub cxl_rsn_inf: Option<Vec<PaymentCancellationReason5>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxInf", skip_serializing_if = "Option::is_none") )]
	pub tx_inf: Option<Vec<PaymentTransaction124>>,
}

impl OriginalPaymentInstruction36 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pmt_cxl_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pmt_cxl_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "pmt_cxl_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.case { val.validate()? }
		if self.orgnl_pmt_inf_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_pmt_inf_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_pmt_inf_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_pmt_inf_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.orgnl_grp_inf { val.validate()? }
		if let Some(ref val) = self.nb_of_txs {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "nb_of_txs does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.cxl_rsn_inf { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.tx_inf { for item in vec { item.validate()? } }
		Ok(())
	}
}


// OriginalTransactionReference28 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OriginalTransactionReference28 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmAmt", skip_serializing_if = "Option::is_none") )]
	pub intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<AmountType4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none") )]
	pub intr_bk_sttlm_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdColltnDt", skip_serializing_if = "Option::is_none") )]
	pub reqd_colltn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none") )]
	pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrSchmeId", skip_serializing_if = "Option::is_none") )]
	pub cdtr_schme_id: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmInf", skip_serializing_if = "Option::is_none") )]
	pub sttlm_inf: Option<SettlementInstruction7>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none") )]
	pub pmt_tp_inf: Option<PaymentTypeInformation27>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtMtd", skip_serializing_if = "Option::is_none") )]
	pub pmt_mtd: Option<PaymentMethod4Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtRltdInf", skip_serializing_if = "Option::is_none") )]
	pub mndt_rltd_inf: Option<MandateRelatedInformation14>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtInf", skip_serializing_if = "Option::is_none") )]
	pub rmt_inf: Option<RemittanceInformation16>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr", skip_serializing_if = "Option::is_none") )]
	pub dbtr: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr", skip_serializing_if = "Option::is_none") )]
	pub cdtr: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
	pub purp: Option<Purpose2Choice>,
}

impl OriginalTransactionReference28 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.intr_bk_sttlm_amt { val.validate()? }
		if let Some(ref val) = self.amt { val.validate()? }
		if let Some(ref val) = self.reqd_exctn_dt { val.validate()? }
		if let Some(ref val) = self.cdtr_schme_id { val.validate()? }
		if let Some(ref val) = self.sttlm_inf { val.validate()? }
		if let Some(ref val) = self.pmt_tp_inf { val.validate()? }
		if let Some(ref val) = self.pmt_mtd { val.validate()? }
		if let Some(ref val) = self.mndt_rltd_inf { val.validate()? }
		if let Some(ref val) = self.rmt_inf { val.validate()? }
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.dbtr { val.validate()? }
		if let Some(ref val) = self.dbtr_acct { val.validate()? }
		if let Some(ref val) = self.dbtr_agt { val.validate()? }
		if let Some(ref val) = self.dbtr_agt_acct { val.validate()? }
		if let Some(ref val) = self.cdtr_agt { val.validate()? }
		if let Some(ref val) = self.cdtr_agt_acct { val.validate()? }
		if let Some(ref val) = self.cdtr { val.validate()? }
		if let Some(ref val) = self.cdtr_acct { val.validate()? }
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		if let Some(ref val) = self.purp { val.validate()? }
		Ok(())
	}
}


// OriginalTransactionReference29 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OriginalTransactionReference29 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<AmountType4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none") )]
	pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XpryDt", skip_serializing_if = "Option::is_none") )]
	pub xpry_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtCond", skip_serializing_if = "Option::is_none") )]
	pub pmt_cond: Option<PaymentCondition1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none") )]
	pub pmt_tp_inf: Option<PaymentTypeInformation26>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtMtd", skip_serializing_if = "Option::is_none") )]
	pub pmt_mtd: Option<PaymentMethod4Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtInf", skip_serializing_if = "Option::is_none") )]
	pub rmt_inf: Option<RemittanceInformation16>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NclsdFile", skip_serializing_if = "Option::is_none") )]
	pub nclsd_file: Option<Vec<Document12>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr", skip_serializing_if = "Option::is_none") )]
	pub dbtr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt") )]
	pub cdtr_agt: BranchAndFinancialInstitutionIdentification6,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr") )]
	pub cdtr: PartyIdentification135,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<PartyIdentification135>,
}

impl OriginalTransactionReference29 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.amt { val.validate()? }
		if let Some(ref val) = self.reqd_exctn_dt { val.validate()? }
		if let Some(ref val) = self.xpry_dt { val.validate()? }
		if let Some(ref val) = self.pmt_cond { val.validate()? }
		if let Some(ref val) = self.pmt_tp_inf { val.validate()? }
		if let Some(ref val) = self.pmt_mtd { val.validate()? }
		if let Some(ref val) = self.rmt_inf { val.validate()? }
		if let Some(ref vec) = self.nclsd_file { for item in vec { item.validate()? } }
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.dbtr { val.validate()? }
		if let Some(ref val) = self.dbtr_acct { val.validate()? }
		if let Some(ref val) = self.dbtr_agt { val.validate()? }
		self.cdtr_agt.validate()?;
		self.cdtr.validate()?;
		if let Some(ref val) = self.cdtr_acct { val.validate()? }
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		Ok(())
	}
}


// OriginalTransactionReference31 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OriginalTransactionReference31 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmAmt", skip_serializing_if = "Option::is_none") )]
	pub intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<AmountType4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none") )]
	pub intr_bk_sttlm_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdColltnDt", skip_serializing_if = "Option::is_none") )]
	pub reqd_colltn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none") )]
	pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrSchmeId", skip_serializing_if = "Option::is_none") )]
	pub cdtr_schme_id: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmInf", skip_serializing_if = "Option::is_none") )]
	pub sttlm_inf: Option<SettlementInstruction7>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none") )]
	pub pmt_tp_inf: Option<PaymentTypeInformation27>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtMtd", skip_serializing_if = "Option::is_none") )]
	pub pmt_mtd: Option<PaymentMethod4Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtRltdInf", skip_serializing_if = "Option::is_none") )]
	pub mndt_rltd_inf: Option<MandateRelatedData1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtInf", skip_serializing_if = "Option::is_none") )]
	pub rmt_inf: Option<RemittanceInformation16>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr", skip_serializing_if = "Option::is_none") )]
	pub dbtr: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr", skip_serializing_if = "Option::is_none") )]
	pub cdtr: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
	pub purp: Option<Purpose2Choice>,
}

impl OriginalTransactionReference31 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.intr_bk_sttlm_amt { val.validate()? }
		if let Some(ref val) = self.amt { val.validate()? }
		if let Some(ref val) = self.reqd_exctn_dt { val.validate()? }
		if let Some(ref val) = self.cdtr_schme_id { val.validate()? }
		if let Some(ref val) = self.sttlm_inf { val.validate()? }
		if let Some(ref val) = self.pmt_tp_inf { val.validate()? }
		if let Some(ref val) = self.pmt_mtd { val.validate()? }
		if let Some(ref val) = self.mndt_rltd_inf { val.validate()? }
		if let Some(ref val) = self.rmt_inf { val.validate()? }
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.dbtr { val.validate()? }
		if let Some(ref val) = self.dbtr_acct { val.validate()? }
		if let Some(ref val) = self.dbtr_agt { val.validate()? }
		if let Some(ref val) = self.dbtr_agt_acct { val.validate()? }
		if let Some(ref val) = self.cdtr_agt { val.validate()? }
		if let Some(ref val) = self.cdtr_agt_acct { val.validate()? }
		if let Some(ref val) = self.cdtr { val.validate()? }
		if let Some(ref val) = self.cdtr_acct { val.validate()? }
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		if let Some(ref val) = self.purp { val.validate()? }
		Ok(())
	}
}


// OriginalTransactionReference32 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OriginalTransactionReference32 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmAmt", skip_serializing_if = "Option::is_none") )]
	pub intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<AmountType4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none") )]
	pub intr_bk_sttlm_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdColltnDt", skip_serializing_if = "Option::is_none") )]
	pub reqd_colltn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none") )]
	pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrSchmeId", skip_serializing_if = "Option::is_none") )]
	pub cdtr_schme_id: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmInf", skip_serializing_if = "Option::is_none") )]
	pub sttlm_inf: Option<SettlementInstruction7>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none") )]
	pub pmt_tp_inf: Option<PaymentTypeInformation27>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtMtd", skip_serializing_if = "Option::is_none") )]
	pub pmt_mtd: Option<PaymentMethod4Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtRltdInf", skip_serializing_if = "Option::is_none") )]
	pub mndt_rltd_inf: Option<MandateRelatedData1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtInf", skip_serializing_if = "Option::is_none") )]
	pub rmt_inf: Option<RemittanceInformation16>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr", skip_serializing_if = "Option::is_none") )]
	pub dbtr: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr", skip_serializing_if = "Option::is_none") )]
	pub cdtr: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
	pub purp: Option<Purpose2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UndrlygCstmrCdtTrf", skip_serializing_if = "Option::is_none") )]
	pub undrlyg_cstmr_cdt_trf: Option<CreditTransferTransaction45>,
}

impl OriginalTransactionReference32 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.intr_bk_sttlm_amt { val.validate()? }
		if let Some(ref val) = self.amt { val.validate()? }
		if let Some(ref val) = self.reqd_exctn_dt { val.validate()? }
		if let Some(ref val) = self.cdtr_schme_id { val.validate()? }
		if let Some(ref val) = self.sttlm_inf { val.validate()? }
		if let Some(ref val) = self.pmt_tp_inf { val.validate()? }
		if let Some(ref val) = self.pmt_mtd { val.validate()? }
		if let Some(ref val) = self.mndt_rltd_inf { val.validate()? }
		if let Some(ref val) = self.rmt_inf { val.validate()? }
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.dbtr { val.validate()? }
		if let Some(ref val) = self.dbtr_acct { val.validate()? }
		if let Some(ref val) = self.dbtr_agt { val.validate()? }
		if let Some(ref val) = self.dbtr_agt_acct { val.validate()? }
		if let Some(ref val) = self.cdtr_agt { val.validate()? }
		if let Some(ref val) = self.cdtr_agt_acct { val.validate()? }
		if let Some(ref val) = self.cdtr { val.validate()? }
		if let Some(ref val) = self.cdtr_acct { val.validate()? }
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		if let Some(ref val) = self.purp { val.validate()? }
		if let Some(ref val) = self.undrlyg_cstmr_cdt_trf { val.validate()? }
		Ok(())
	}
}


// OtherContact1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OtherContact1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChanlTp") )]
	pub chanl_tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
}

impl OtherContact1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.chanl_tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "chanl_tp is shorter than the minimum length of 1".to_string()));
		}
		if self.chanl_tp.chars().count() > 4 {
			return Err(ValidationError::new(1002, "chanl_tp exceeds the maximum length of 4".to_string()));
		}
		if let Some(ref val) = self.id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 128 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 128".to_string()));
			}
		}
		Ok(())
	}
}


// OtherIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct OtherIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sfx", skip_serializing_if = "Option::is_none") )]
	pub sfx: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: IdentificationSource3Choice,
}

impl OtherIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.sfx {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sfx is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "sfx exceeds the maximum length of 16".to_string()));
			}
		}
		self.tp.validate()?;
		Ok(())
	}
}


// POIComponentType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum POIComponentType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "SOFT") )]
	CodeSOFT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EMVK") )]
	CodeEMVK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EMVO") )]
	CodeEMVO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MRIT") )]
	CodeMRIT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CHIT") )]
	CodeCHIT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SECM") )]
	CodeSECM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PEDV") )]
	CodePEDV,
}

impl POIComponentType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Pagination1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Pagination1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PgNb") )]
	pub pg_nb: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LastPgInd") )]
	pub last_pg_ind: bool,
}

impl Pagination1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{1,5}").unwrap();
		if !pattern.is_match(&self.pg_nb) {
			return Err(ValidationError::new(1005, "pg_nb does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// Party38Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Party38Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgId", skip_serializing_if = "Option::is_none") )]
	pub org_id: Option<OrganisationIdentification29>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvtId", skip_serializing_if = "Option::is_none") )]
	pub prvt_id: Option<PersonIdentification13>,
}

impl Party38Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.org_id { val.validate()? }
		if let Some(ref val) = self.prvt_id { val.validate()? }
		Ok(())
	}
}


// Party40Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Party40Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pty", skip_serializing_if = "Option::is_none") )]
	pub pty: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Agt", skip_serializing_if = "Option::is_none") )]
	pub agt: Option<BranchAndFinancialInstitutionIdentification6>,
}

impl Party40Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pty { val.validate()? }
		if let Some(ref val) = self.agt { val.validate()? }
		Ok(())
	}
}


// Party44Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Party44Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgId", skip_serializing_if = "Option::is_none") )]
	pub org_id: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FIId", skip_serializing_if = "Option::is_none") )]
	pub fi_id: Option<BranchAndFinancialInstitutionIdentification6>,
}

impl Party44Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.org_id { val.validate()? }
		if let Some(ref val) = self.fi_id { val.validate()? }
		Ok(())
	}
}


// PartyAndSignature3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyAndSignature3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pty") )]
	pub pty: PartyIdentification135,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sgntr") )]
	pub sgntr: SkipPayload,
}

impl PartyAndSignature3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.pty.validate()?;
		self.sgntr.validate()?;
		Ok(())
	}
}


// PartyIdentification120Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyIdentification120Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none") )]
	pub any_bic: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryId", skip_serializing_if = "Option::is_none") )]
	pub prtry_id: Option<GenericIdentification36>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NmAndAdr", skip_serializing_if = "Option::is_none") )]
	pub nm_and_adr: Option<NameAndAddress5>,
}

impl PartyIdentification120Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.any_bic {
			let pattern = Regex::new("[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "any_bic does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.prtry_id { val.validate()? }
		if let Some(ref val) = self.nm_and_adr { val.validate()? }
		Ok(())
	}
}


// PartyIdentification135 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyIdentification135 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
	pub pstl_adr: Option<PostalAddress24>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<Party38Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none") )]
	pub ctry_of_res: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none") )]
	pub ctct_dtls: Option<Contact4>,
}

impl PartyIdentification135 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.pstl_adr { val.validate()? }
		if let Some(ref val) = self.id { val.validate()? }
		if let Some(ref val) = self.ctry_of_res {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry_of_res does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ctct_dtls { val.validate()? }
		Ok(())
	}
}


// PartyIdentification136 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PartyIdentification136 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: PartyIdentification120Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LEI", skip_serializing_if = "Option::is_none") )]
	pub lei: Option<String>,
}

impl PartyIdentification136 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		if let Some(ref val) = self.lei {
			let pattern = Regex::new("[A-Z0-9]{18,18}[0-9]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "lei does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// PartyType3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum PartyType3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "OPOI") )]
	CodeOPOI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MERC") )]
	CodeMERC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACCP") )]
	CodeACCP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ITAG") )]
	CodeITAG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACQR") )]
	CodeACQR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CISS") )]
	CodeCISS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DLIS") )]
	CodeDLIS,
}

impl PartyType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PartyType4Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum PartyType4Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MERC") )]
	CodeMERC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACCP") )]
	CodeACCP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ITAG") )]
	CodeITAG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACQR") )]
	CodeACQR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CISS") )]
	CodeCISS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TAXH") )]
	CodeTAXH,
}

impl PartyType4Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PaymentCancellationReason5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentCancellationReason5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Orgtr", skip_serializing_if = "Option::is_none") )]
	pub orgtr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<CancellationReason33Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<String>>,
}

impl PaymentCancellationReason5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgtr { val.validate()? }
		if let Some(ref val) = self.rsn { val.validate()? }
		if let Some(ref vec) = self.addtl_inf {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 105 {
					return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 105".to_string()));
				}
			}
		}
		Ok(())
	}
}


// PaymentCard4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentCard4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PlainCardData", skip_serializing_if = "Option::is_none") )]
	pub plain_card_data: Option<PlainCardData1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CardCtryCd", skip_serializing_if = "Option::is_none") )]
	pub card_ctry_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CardBrnd", skip_serializing_if = "Option::is_none") )]
	pub card_brnd: Option<GenericIdentification1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlCardData", skip_serializing_if = "Option::is_none") )]
	pub addtl_card_data: Option<String>,
}

impl PaymentCard4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.plain_card_data { val.validate()? }
		if let Some(ref val) = self.card_ctry_cd {
			let pattern = Regex::new("[0-9]{3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "card_ctry_cd does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.card_brnd { val.validate()? }
		if let Some(ref val) = self.addtl_card_data {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_card_data is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "addtl_card_data exceeds the maximum length of 70".to_string()));
			}
		}
		Ok(())
	}
}


// PaymentComplementaryInformation8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentComplementaryInformation8 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrId", skip_serializing_if = "Option::is_none") )]
	pub instr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none") )]
	pub end_to_end_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxId", skip_serializing_if = "Option::is_none") )]
	pub tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none") )]
	pub pmt_tp_inf: Option<PaymentTypeInformation27>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none") )]
	pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdColltnDt", skip_serializing_if = "Option::is_none") )]
	pub reqd_colltn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none") )]
	pub intr_bk_sttlm_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<AmountType4Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmAmt", skip_serializing_if = "Option::is_none") )]
	pub intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgBr", skip_serializing_if = "Option::is_none") )]
	pub chrg_br: Option<ChargeBearerType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr", skip_serializing_if = "Option::is_none") )]
	pub dbtr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmInf", skip_serializing_if = "Option::is_none") )]
	pub sttlm_inf: Option<SettlementInstruction7>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt1Acct", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt1_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt2Acct", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt2_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt3Acct", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt3_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr", skip_serializing_if = "Option::is_none") )]
	pub cdtr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
	pub purp: Option<Purpose2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrForDbtrAgt", skip_serializing_if = "Option::is_none") )]
	pub instr_for_dbtr_agt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt1", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt1Acct", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt1_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt2", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt2Acct", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt2_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt3", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt3Acct", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt3_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrForNxtAgt", skip_serializing_if = "Option::is_none") )]
	pub instr_for_nxt_agt: Option<Vec<InstructionForNextAgent1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrForCdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub instr_for_cdtr_agt: Option<Vec<InstructionForCreditorAgent1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtInf", skip_serializing_if = "Option::is_none") )]
	pub rmt_inf: Option<RemittanceInformation16>,
}

impl PaymentComplementaryInformation8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.instr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "instr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "instr_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.end_to_end_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "end_to_end_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "end_to_end_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.tx_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tx_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tx_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.pmt_tp_inf { val.validate()? }
		if let Some(ref val) = self.reqd_exctn_dt { val.validate()? }
		if let Some(ref val) = self.amt { val.validate()? }
		if let Some(ref val) = self.intr_bk_sttlm_amt { val.validate()? }
		if let Some(ref val) = self.chrg_br { val.validate()? }
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.dbtr { val.validate()? }
		if let Some(ref val) = self.dbtr_acct { val.validate()? }
		if let Some(ref val) = self.dbtr_agt { val.validate()? }
		if let Some(ref val) = self.dbtr_agt_acct { val.validate()? }
		if let Some(ref val) = self.sttlm_inf { val.validate()? }
		if let Some(ref val) = self.intrmy_agt1 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt1_acct { val.validate()? }
		if let Some(ref val) = self.intrmy_agt2 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt2_acct { val.validate()? }
		if let Some(ref val) = self.intrmy_agt3 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt3_acct { val.validate()? }
		if let Some(ref val) = self.cdtr_agt { val.validate()? }
		if let Some(ref val) = self.cdtr_agt_acct { val.validate()? }
		if let Some(ref val) = self.cdtr { val.validate()? }
		if let Some(ref val) = self.cdtr_acct { val.validate()? }
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		if let Some(ref val) = self.purp { val.validate()? }
		if let Some(ref val) = self.instr_for_dbtr_agt {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "instr_for_dbtr_agt is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "instr_for_dbtr_agt exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.prvs_instg_agt1 { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt1_acct { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt2 { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt2_acct { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt3 { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt3_acct { val.validate()? }
		if let Some(ref vec) = self.instr_for_nxt_agt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.instr_for_cdtr_agt { for item in vec { item.validate()? } }
		if let Some(ref val) = self.rmt_inf { val.validate()? }
		Ok(())
	}
}


// PaymentCondition1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentCondition1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmtModAllwd") )]
	pub amt_mod_allwd: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EarlyPmtAllwd") )]
	pub early_pmt_allwd: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DelyPnlty", skip_serializing_if = "Option::is_none") )]
	pub dely_pnlty: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ImdtPmtRbt", skip_serializing_if = "Option::is_none") )]
	pub imdt_pmt_rbt: Option<AmountOrRate1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrntedPmtReqd") )]
	pub grnted_pmt_reqd: bool,
}

impl PaymentCondition1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dely_pnlty {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dely_pnlty is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "dely_pnlty exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.imdt_pmt_rbt { val.validate()? }
		Ok(())
	}
}


// PaymentConditionStatus1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentConditionStatus1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AccptdAmt", skip_serializing_if = "Option::is_none") )]
	pub accptd_amt: Option<ActiveCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrntedPmt") )]
	pub grnted_pmt: bool,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EarlyPmt") )]
	pub early_pmt: bool,
}

impl PaymentConditionStatus1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.accptd_amt { val.validate()? }
		Ok(())
	}
}


// PaymentContext3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentContext3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CardPres", skip_serializing_if = "Option::is_none") )]
	pub card_pres: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CrdhldrPres", skip_serializing_if = "Option::is_none") )]
	pub crdhldr_pres: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OnLineCntxt", skip_serializing_if = "Option::is_none") )]
	pub on_line_cntxt: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AttndncCntxt", skip_serializing_if = "Option::is_none") )]
	pub attndnc_cntxt: Option<AttendanceContext1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxEnvt", skip_serializing_if = "Option::is_none") )]
	pub tx_envt: Option<TransactionEnvironment1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxChanl", skip_serializing_if = "Option::is_none") )]
	pub tx_chanl: Option<TransactionChannel1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AttndntMsgCpbl", skip_serializing_if = "Option::is_none") )]
	pub attndnt_msg_cpbl: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AttndntLang", skip_serializing_if = "Option::is_none") )]
	pub attndnt_lang: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CardDataNtryMd") )]
	pub card_data_ntry_md: CardDataReading1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FllbckInd", skip_serializing_if = "Option::is_none") )]
	pub fllbck_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AuthntcnMtd", skip_serializing_if = "Option::is_none") )]
	pub authntcn_mtd: Option<CardholderAuthentication2>,
}

impl PaymentContext3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.attndnc_cntxt { val.validate()? }
		if let Some(ref val) = self.tx_envt { val.validate()? }
		if let Some(ref val) = self.tx_chanl { val.validate()? }
		if let Some(ref val) = self.attndnt_lang {
			let pattern = Regex::new("[a-z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "attndnt_lang does not match the required pattern".to_string()));
			}
		}
		self.card_data_ntry_md.validate()?;
		if let Some(ref val) = self.authntcn_mtd { val.validate()? }
		Ok(())
	}
}


// PaymentIdentification6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentIdentification6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrId", skip_serializing_if = "Option::is_none") )]
	pub instr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndToEndId") )]
	pub end_to_end_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UETR", skip_serializing_if = "Option::is_none") )]
	pub uetr: Option<String>,
}

impl PaymentIdentification6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.instr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "instr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "instr_id exceeds the maximum length of 35".to_string()));
			}
		}
		if self.end_to_end_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "end_to_end_id is shorter than the minimum length of 1".to_string()));
		}
		if self.end_to_end_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "end_to_end_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.uetr {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "uetr does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// PaymentIdentification7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentIdentification7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrId", skip_serializing_if = "Option::is_none") )]
	pub instr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndToEndId") )]
	pub end_to_end_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxId", skip_serializing_if = "Option::is_none") )]
	pub tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UETR", skip_serializing_if = "Option::is_none") )]
	pub uetr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysRef", skip_serializing_if = "Option::is_none") )]
	pub clr_sys_ref: Option<String>,
}

impl PaymentIdentification7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.instr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "instr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "instr_id exceeds the maximum length of 35".to_string()));
			}
		}
		if self.end_to_end_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "end_to_end_id is shorter than the minimum length of 1".to_string()));
		}
		if self.end_to_end_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "end_to_end_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.tx_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tx_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tx_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.uetr {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "uetr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.clr_sys_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "clr_sys_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "clr_sys_ref exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// PaymentInstruction31 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentInstruction31 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtInfId", skip_serializing_if = "Option::is_none") )]
	pub pmt_inf_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtMtd") )]
	pub pmt_mtd: PaymentMethod7Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtTpInf", skip_serializing_if = "Option::is_none") )]
	pub pmt_tp_inf: Option<PaymentTypeInformation26>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdExctnDt") )]
	pub reqd_exctn_dt: DateAndDateTime2Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XpryDt", skip_serializing_if = "Option::is_none") )]
	pub xpry_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtCond", skip_serializing_if = "Option::is_none") )]
	pub pmt_cond: Option<PaymentCondition1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr") )]
	pub dbtr: PartyIdentification135,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt") )]
	pub dbtr_agt: BranchAndFinancialInstitutionIdentification6,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgBr", skip_serializing_if = "Option::is_none") )]
	pub chrg_br: Option<ChargeBearerType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtTrfTx") )]
	pub cdt_trf_tx: Vec<CreditTransferTransaction35>,
}

impl PaymentInstruction31 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pmt_inf_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pmt_inf_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "pmt_inf_id exceeds the maximum length of 35".to_string()));
			}
		}
		self.pmt_mtd.validate()?;
		if let Some(ref val) = self.pmt_tp_inf { val.validate()? }
		self.reqd_exctn_dt.validate()?;
		if let Some(ref val) = self.xpry_dt { val.validate()? }
		if let Some(ref val) = self.pmt_cond { val.validate()? }
		self.dbtr.validate()?;
		if let Some(ref val) = self.dbtr_acct { val.validate()? }
		self.dbtr_agt.validate()?;
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.chrg_br { val.validate()? }
		for item in &self.cdt_trf_tx { item.validate()? }
		Ok(())
	}
}


// PaymentMethod4Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum PaymentMethod4Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CHK") )]
	CodeCHK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRF") )]
	CodeTRF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DD") )]
	CodeDD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRA") )]
	CodeTRA,
}

impl PaymentMethod4Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PaymentMethod7Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum PaymentMethod7Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CHK") )]
	CodeCHK,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TRF") )]
	CodeTRF,
}

impl PaymentMethod7Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// PaymentReturnReason5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentReturnReason5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlBkTxCd", skip_serializing_if = "Option::is_none") )]
	pub orgnl_bk_tx_cd: Option<BankTransactionCodeStructure4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Orgtr", skip_serializing_if = "Option::is_none") )]
	pub orgtr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<ReturnReason5Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<String>>,
}

impl PaymentReturnReason5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_bk_tx_cd { val.validate()? }
		if let Some(ref val) = self.orgtr { val.validate()? }
		if let Some(ref val) = self.rsn { val.validate()? }
		if let Some(ref vec) = self.addtl_inf {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 105 {
					return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 105".to_string()));
				}
			}
		}
		Ok(())
	}
}


// PaymentReturnReason6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentReturnReason6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Orgtr", skip_serializing_if = "Option::is_none") )]
	pub orgtr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<ReturnReason5Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<String>>,
}

impl PaymentReturnReason6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgtr { val.validate()? }
		if let Some(ref val) = self.rsn { val.validate()? }
		if let Some(ref vec) = self.addtl_inf {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 105 {
					return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 105".to_string()));
				}
			}
		}
		Ok(())
	}
}


// PaymentTransaction102 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentTransaction102 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CxlStsId", skip_serializing_if = "Option::is_none") )]
	pub cxl_sts_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RslvdCase", skip_serializing_if = "Option::is_none") )]
	pub rslvd_case: Option<Case5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none") )]
	pub orgnl_grp_inf: Option<OriginalGroupInformation29>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_instr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_end_to_end_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTxId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlClrSysRef", skip_serializing_if = "Option::is_none") )]
	pub orgnl_clr_sys_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none") )]
	pub orgnl_uetr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxCxlSts", skip_serializing_if = "Option::is_none") )]
	pub tx_cxl_sts: Option<CancellationIndividualStatus1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CxlStsRsnInf", skip_serializing_if = "Option::is_none") )]
	pub cxl_sts_rsn_inf: Option<Vec<CancellationStatusReason4>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RsltnRltdInf", skip_serializing_if = "Option::is_none") )]
	pub rsltn_rltd_inf: Option<ResolutionData1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlIntrBkSttlmAmt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlIntrBkSttlmDt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_intr_bk_sttlm_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Assgnr", skip_serializing_if = "Option::is_none") )]
	pub assgnr: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Assgne", skip_serializing_if = "Option::is_none") )]
	pub assgne: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none") )]
	pub orgnl_tx_ref: Option<OriginalTransactionReference28>,
}

impl PaymentTransaction102 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cxl_sts_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cxl_sts_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "cxl_sts_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.rslvd_case { val.validate()? }
		if let Some(ref val) = self.orgnl_grp_inf { val.validate()? }
		if let Some(ref val) = self.orgnl_instr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_instr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_instr_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_end_to_end_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_end_to_end_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_end_to_end_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_tx_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_tx_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_tx_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_clr_sys_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_clr_sys_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_clr_sys_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_uetr {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_uetr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.tx_cxl_sts { val.validate()? }
		if let Some(ref vec) = self.cxl_sts_rsn_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.rsltn_rltd_inf { val.validate()? }
		if let Some(ref val) = self.orgnl_intr_bk_sttlm_amt { val.validate()? }
		if let Some(ref val) = self.assgnr { val.validate()? }
		if let Some(ref val) = self.assgne { val.validate()? }
		if let Some(ref val) = self.orgnl_tx_ref { val.validate()? }
		Ok(())
	}
}


// PaymentTransaction103 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentTransaction103 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CxlStsId", skip_serializing_if = "Option::is_none") )]
	pub cxl_sts_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RslvdCase", skip_serializing_if = "Option::is_none") )]
	pub rslvd_case: Option<Case5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_instr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_end_to_end_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UETR", skip_serializing_if = "Option::is_none") )]
	pub uetr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxCxlSts", skip_serializing_if = "Option::is_none") )]
	pub tx_cxl_sts: Option<CancellationIndividualStatus1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CxlStsRsnInf", skip_serializing_if = "Option::is_none") )]
	pub cxl_sts_rsn_inf: Option<Vec<CancellationStatusReason4>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlInstdAmt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlReqdExctnDt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlReqdColltnDt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_reqd_colltn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none") )]
	pub orgnl_tx_ref: Option<OriginalTransactionReference28>,
}

impl PaymentTransaction103 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cxl_sts_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cxl_sts_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "cxl_sts_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.rslvd_case { val.validate()? }
		if let Some(ref val) = self.orgnl_instr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_instr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_instr_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_end_to_end_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_end_to_end_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_end_to_end_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.uetr {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "uetr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.tx_cxl_sts { val.validate()? }
		if let Some(ref vec) = self.cxl_sts_rsn_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.orgnl_instd_amt { val.validate()? }
		if let Some(ref val) = self.orgnl_reqd_exctn_dt { val.validate()? }
		if let Some(ref val) = self.orgnl_tx_ref { val.validate()? }
		Ok(())
	}
}


// PaymentTransaction104 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentTransaction104 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsId", skip_serializing_if = "Option::is_none") )]
	pub sts_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_instr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_end_to_end_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none") )]
	pub orgnl_uetr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxSts", skip_serializing_if = "Option::is_none") )]
	pub tx_sts: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsRsnInf", skip_serializing_if = "Option::is_none") )]
	pub sts_rsn_inf: Option<Vec<StatusReasonInformation12>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtCondSts", skip_serializing_if = "Option::is_none") )]
	pub pmt_cond_sts: Option<PaymentConditionStatus1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgsInf", skip_serializing_if = "Option::is_none") )]
	pub chrgs_inf: Option<Vec<Charges7>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrDcsnDtTm", skip_serializing_if = "Option::is_none") )]
	pub dbtr_dcsn_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AccptncDtTm", skip_serializing_if = "Option::is_none") )]
	pub accptnc_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrRef", skip_serializing_if = "Option::is_none") )]
	pub acct_svcr_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysRef", skip_serializing_if = "Option::is_none") )]
	pub clr_sys_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none") )]
	pub orgnl_tx_ref: Option<OriginalTransactionReference29>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NclsdFile", skip_serializing_if = "Option::is_none") )]
	pub nclsd_file: Option<Vec<Document12>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl PaymentTransaction104 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.sts_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sts_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "sts_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_instr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_instr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_instr_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_end_to_end_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_end_to_end_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_end_to_end_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_uetr {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_uetr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.tx_sts {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tx_sts is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "tx_sts exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref vec) = self.sts_rsn_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.pmt_cond_sts { val.validate()? }
		if let Some(ref vec) = self.chrgs_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.acct_svcr_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_svcr_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acct_svcr_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.clr_sys_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "clr_sys_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "clr_sys_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_tx_ref { val.validate()? }
		if let Some(ref vec) = self.nclsd_file { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PaymentTransaction106 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentTransaction106 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CxlId", skip_serializing_if = "Option::is_none") )]
	pub cxl_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Case", skip_serializing_if = "Option::is_none") )]
	pub case: Option<Case5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none") )]
	pub orgnl_grp_inf: Option<OriginalGroupInformation29>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_instr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_end_to_end_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTxId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none") )]
	pub orgnl_uetr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlClrSysRef", skip_serializing_if = "Option::is_none") )]
	pub orgnl_clr_sys_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlIntrBkSttlmAmt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlIntrBkSttlmDt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_intr_bk_sttlm_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Assgnr", skip_serializing_if = "Option::is_none") )]
	pub assgnr: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Assgne", skip_serializing_if = "Option::is_none") )]
	pub assgne: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CxlRsnInf", skip_serializing_if = "Option::is_none") )]
	pub cxl_rsn_inf: Option<Vec<PaymentCancellationReason5>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none") )]
	pub orgnl_tx_ref: Option<OriginalTransactionReference28>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl PaymentTransaction106 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cxl_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cxl_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "cxl_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.case { val.validate()? }
		if let Some(ref val) = self.orgnl_grp_inf { val.validate()? }
		if let Some(ref val) = self.orgnl_instr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_instr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_instr_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_end_to_end_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_end_to_end_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_end_to_end_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_tx_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_tx_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_tx_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_uetr {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_uetr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_clr_sys_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_clr_sys_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_clr_sys_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_intr_bk_sttlm_amt { val.validate()? }
		if let Some(ref val) = self.assgnr { val.validate()? }
		if let Some(ref val) = self.assgne { val.validate()? }
		if let Some(ref vec) = self.cxl_rsn_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.orgnl_tx_ref { val.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PaymentTransaction107 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentTransaction107 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModStsId", skip_serializing_if = "Option::is_none") )]
	pub mod_sts_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RslvdCase", skip_serializing_if = "Option::is_none") )]
	pub rslvd_case: Option<Case5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlGrpInf") )]
	pub orgnl_grp_inf: OriginalGroupInformation29,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlPmtInfId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_pmt_inf_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_instr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_end_to_end_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTxId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlClrSysRef", skip_serializing_if = "Option::is_none") )]
	pub orgnl_clr_sys_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none") )]
	pub orgnl_uetr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ModStsRsnInf", skip_serializing_if = "Option::is_none") )]
	pub mod_sts_rsn_inf: Option<Vec<ModificationStatusReason2>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RsltnRltdInf", skip_serializing_if = "Option::is_none") )]
	pub rsltn_rltd_inf: Option<ResolutionData1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlIntrBkSttlmAmt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlIntrBkSttlmDt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_intr_bk_sttlm_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Assgnr", skip_serializing_if = "Option::is_none") )]
	pub assgnr: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Assgne", skip_serializing_if = "Option::is_none") )]
	pub assgne: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none") )]
	pub orgnl_tx_ref: Option<OriginalTransactionReference28>,
}

impl PaymentTransaction107 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mod_sts_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mod_sts_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mod_sts_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.rslvd_case { val.validate()? }
		self.orgnl_grp_inf.validate()?;
		if let Some(ref val) = self.orgnl_pmt_inf_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_pmt_inf_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_pmt_inf_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_instr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_instr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_instr_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_end_to_end_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_end_to_end_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_end_to_end_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_tx_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_tx_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_tx_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_clr_sys_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_clr_sys_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_clr_sys_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_uetr {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_uetr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.mod_sts_rsn_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.rsltn_rltd_inf { val.validate()? }
		if let Some(ref val) = self.orgnl_intr_bk_sttlm_amt { val.validate()? }
		if let Some(ref val) = self.assgnr { val.validate()? }
		if let Some(ref val) = self.assgne { val.validate()? }
		if let Some(ref val) = self.orgnl_tx_ref { val.validate()? }
		Ok(())
	}
}


// PaymentTransaction110 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentTransaction110 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsId", skip_serializing_if = "Option::is_none") )]
	pub sts_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none") )]
	pub orgnl_grp_inf: Option<OriginalGroupInformation29>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_instr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_end_to_end_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTxId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none") )]
	pub orgnl_uetr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxSts", skip_serializing_if = "Option::is_none") )]
	pub tx_sts: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsRsnInf", skip_serializing_if = "Option::is_none") )]
	pub sts_rsn_inf: Option<Vec<StatusReasonInformation12>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgsInf", skip_serializing_if = "Option::is_none") )]
	pub chrgs_inf: Option<Vec<Charges7>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AccptncDtTm", skip_serializing_if = "Option::is_none") )]
	pub accptnc_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FctvIntrBkSttlmDt", skip_serializing_if = "Option::is_none") )]
	pub fctv_intr_bk_sttlm_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrRef", skip_serializing_if = "Option::is_none") )]
	pub acct_svcr_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysRef", skip_serializing_if = "Option::is_none") )]
	pub clr_sys_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none") )]
	pub instg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none") )]
	pub instd_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none") )]
	pub orgnl_tx_ref: Option<OriginalTransactionReference28>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl PaymentTransaction110 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.sts_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sts_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "sts_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_grp_inf { val.validate()? }
		if let Some(ref val) = self.orgnl_instr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_instr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_instr_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_end_to_end_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_end_to_end_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_end_to_end_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_tx_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_tx_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_tx_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_uetr {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_uetr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.tx_sts {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tx_sts is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "tx_sts exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref vec) = self.sts_rsn_inf { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.chrgs_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.fctv_intr_bk_sttlm_dt { val.validate()? }
		if let Some(ref val) = self.acct_svcr_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_svcr_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acct_svcr_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.clr_sys_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "clr_sys_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "clr_sys_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.instg_agt { val.validate()? }
		if let Some(ref val) = self.instd_agt { val.validate()? }
		if let Some(ref val) = self.orgnl_tx_ref { val.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PaymentTransaction113 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentTransaction113 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsReqId", skip_serializing_if = "Option::is_none") )]
	pub sts_req_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none") )]
	pub orgnl_grp_inf: Option<OriginalGroupInformation29>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_instr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_end_to_end_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTxId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none") )]
	pub orgnl_uetr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AccptncDtTm", skip_serializing_if = "Option::is_none") )]
	pub accptnc_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysRef", skip_serializing_if = "Option::is_none") )]
	pub clr_sys_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none") )]
	pub instg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none") )]
	pub instd_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none") )]
	pub orgnl_tx_ref: Option<OriginalTransactionReference28>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl PaymentTransaction113 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.sts_req_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sts_req_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "sts_req_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_grp_inf { val.validate()? }
		if let Some(ref val) = self.orgnl_instr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_instr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_instr_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_end_to_end_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_end_to_end_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_end_to_end_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_tx_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_tx_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_tx_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_uetr {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_uetr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.clr_sys_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "clr_sys_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "clr_sys_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.instg_agt { val.validate()? }
		if let Some(ref val) = self.instd_agt { val.validate()? }
		if let Some(ref val) = self.orgnl_tx_ref { val.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PaymentTransaction118 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentTransaction118 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RtrId", skip_serializing_if = "Option::is_none") )]
	pub rtr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none") )]
	pub orgnl_grp_inf: Option<OriginalGroupInformation29>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_instr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_end_to_end_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTxId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none") )]
	pub orgnl_uetr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlClrSysRef", skip_serializing_if = "Option::is_none") )]
	pub orgnl_clr_sys_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlIntrBkSttlmAmt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlIntrBkSttlmDt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_intr_bk_sttlm_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RtrdIntrBkSttlmAmt") )]
	pub rtrd_intr_bk_sttlm_amt: ActiveCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none") )]
	pub intr_bk_sttlm_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmPrty", skip_serializing_if = "Option::is_none") )]
	pub sttlm_prty: Option<Priority3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmTmIndctn", skip_serializing_if = "Option::is_none") )]
	pub sttlm_tm_indctn: Option<SettlementDateTimeIndication1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RtrdInstdAmt", skip_serializing_if = "Option::is_none") )]
	pub rtrd_instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XchgRate", skip_serializing_if = "Option::is_none") )]
	pub xchg_rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CompstnAmt", skip_serializing_if = "Option::is_none") )]
	pub compstn_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgBr", skip_serializing_if = "Option::is_none") )]
	pub chrg_br: Option<ChargeBearerType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgsInf", skip_serializing_if = "Option::is_none") )]
	pub chrgs_inf: Option<Vec<Charges7>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysRef", skip_serializing_if = "Option::is_none") )]
	pub clr_sys_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none") )]
	pub instg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none") )]
	pub instd_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RtrChain", skip_serializing_if = "Option::is_none") )]
	pub rtr_chain: Option<TransactionParties8>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RtrRsnInf", skip_serializing_if = "Option::is_none") )]
	pub rtr_rsn_inf: Option<Vec<PaymentReturnReason6>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none") )]
	pub orgnl_tx_ref: Option<OriginalTransactionReference32>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl PaymentTransaction118 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rtr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rtr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "rtr_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_grp_inf { val.validate()? }
		if let Some(ref val) = self.orgnl_instr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_instr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_instr_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_end_to_end_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_end_to_end_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_end_to_end_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_tx_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_tx_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_tx_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_uetr {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_uetr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_clr_sys_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_clr_sys_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_clr_sys_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_intr_bk_sttlm_amt { val.validate()? }
		self.rtrd_intr_bk_sttlm_amt.validate()?;
		if let Some(ref val) = self.sttlm_prty { val.validate()? }
		if let Some(ref val) = self.sttlm_tm_indctn { val.validate()? }
		if let Some(ref val) = self.rtrd_instd_amt { val.validate()? }
		if let Some(ref val) = self.compstn_amt { val.validate()? }
		if let Some(ref val) = self.chrg_br { val.validate()? }
		if let Some(ref vec) = self.chrgs_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.clr_sys_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "clr_sys_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "clr_sys_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.instg_agt { val.validate()? }
		if let Some(ref val) = self.instd_agt { val.validate()? }
		if let Some(ref val) = self.rtr_chain { val.validate()? }
		if let Some(ref vec) = self.rtr_rsn_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.orgnl_tx_ref { val.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PaymentTransaction124 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentTransaction124 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CxlId", skip_serializing_if = "Option::is_none") )]
	pub cxl_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Case", skip_serializing_if = "Option::is_none") )]
	pub case: Option<Case5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_instr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_end_to_end_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none") )]
	pub orgnl_uetr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlInstdAmt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlReqdExctnDt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlReqdColltnDt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_reqd_colltn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CxlRsnInf", skip_serializing_if = "Option::is_none") )]
	pub cxl_rsn_inf: Option<Vec<PaymentCancellationReason5>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none") )]
	pub orgnl_tx_ref: Option<OriginalTransactionReference31>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none") )]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl PaymentTransaction124 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cxl_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cxl_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "cxl_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.case { val.validate()? }
		if let Some(ref val) = self.orgnl_instr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_instr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_instr_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_end_to_end_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_end_to_end_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_end_to_end_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_uetr {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_uetr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_instd_amt { val.validate()? }
		if let Some(ref val) = self.orgnl_reqd_exctn_dt { val.validate()? }
		if let Some(ref vec) = self.cxl_rsn_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.orgnl_tx_ref { val.validate()? }
		if let Some(ref vec) = self.splmtry_data { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PaymentTypeInformation26 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentTypeInformation26 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none") )]
	pub instr_prty: Option<Priority2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none") )]
	pub svc_lvl: Option<Vec<ServiceLevel8Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none") )]
	pub lcl_instrm: Option<LocalInstrument2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none") )]
	pub ctgy_purp: Option<CategoryPurpose1Choice>,
}

impl PaymentTypeInformation26 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.instr_prty { val.validate()? }
		if let Some(ref vec) = self.svc_lvl { for item in vec { item.validate()? } }
		if let Some(ref val) = self.lcl_instrm { val.validate()? }
		if let Some(ref val) = self.ctgy_purp { val.validate()? }
		Ok(())
	}
}


// PaymentTypeInformation27 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentTypeInformation27 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none") )]
	pub instr_prty: Option<Priority2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrChanl", skip_serializing_if = "Option::is_none") )]
	pub clr_chanl: Option<ClearingChannel2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none") )]
	pub svc_lvl: Option<Vec<ServiceLevel8Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none") )]
	pub lcl_instrm: Option<LocalInstrument2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SeqTp", skip_serializing_if = "Option::is_none") )]
	pub seq_tp: Option<SequenceType3Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none") )]
	pub ctgy_purp: Option<CategoryPurpose1Choice>,
}

impl PaymentTypeInformation27 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.instr_prty { val.validate()? }
		if let Some(ref val) = self.clr_chanl { val.validate()? }
		if let Some(ref vec) = self.svc_lvl { for item in vec { item.validate()? } }
		if let Some(ref val) = self.lcl_instrm { val.validate()? }
		if let Some(ref val) = self.seq_tp { val.validate()? }
		if let Some(ref val) = self.ctgy_purp { val.validate()? }
		Ok(())
	}
}


// PaymentTypeInformation28 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PaymentTypeInformation28 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrPrty", skip_serializing_if = "Option::is_none") )]
	pub instr_prty: Option<Priority2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrChanl", skip_serializing_if = "Option::is_none") )]
	pub clr_chanl: Option<ClearingChannel2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SvcLvl", skip_serializing_if = "Option::is_none") )]
	pub svc_lvl: Option<Vec<ServiceLevel8Choice>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LclInstrm", skip_serializing_if = "Option::is_none") )]
	pub lcl_instrm: Option<LocalInstrument2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtgyPurp", skip_serializing_if = "Option::is_none") )]
	pub ctgy_purp: Option<CategoryPurpose1Choice>,
}

impl PaymentTypeInformation28 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.instr_prty { val.validate()? }
		if let Some(ref val) = self.clr_chanl { val.validate()? }
		if let Some(ref vec) = self.svc_lvl { for item in vec { item.validate()? } }
		if let Some(ref val) = self.lcl_instrm { val.validate()? }
		if let Some(ref val) = self.ctgy_purp { val.validate()? }
		Ok(())
	}
}


// PersonIdentification13 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PersonIdentification13 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none") )]
	pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<Vec<GenericPersonIdentification1>>,
}

impl PersonIdentification13 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dt_and_plc_of_birth { val.validate()? }
		if let Some(ref vec) = self.othr { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PersonIdentificationSchemeName1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PersonIdentificationSchemeName1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl PersonIdentificationSchemeName1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// PlainCardData1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PlainCardData1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PAN") )]
	pub pan: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CardSeqNb", skip_serializing_if = "Option::is_none") )]
	pub card_seq_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FctvDt", skip_serializing_if = "Option::is_none") )]
	pub fctv_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "XpryDt") )]
	pub xpry_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SvcCd", skip_serializing_if = "Option::is_none") )]
	pub svc_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrckData", skip_serializing_if = "Option::is_none") )]
	pub trck_data: Option<Vec<TrackData1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CardSctyCd", skip_serializing_if = "Option::is_none") )]
	pub card_scty_cd: Option<CardSecurityInformation1>,
}

impl PlainCardData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[0-9]{8,28}").unwrap();
		if !pattern.is_match(&self.pan) {
			return Err(ValidationError::new(1005, "pan does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.card_seq_nb {
			let pattern = Regex::new("[0-9]{2,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "card_seq_nb does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.svc_cd {
			let pattern = Regex::new("[0-9]{3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "svc_cd does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.trck_data { for item in vec { item.validate()? } }
		if let Some(ref val) = self.card_scty_cd { val.validate()? }
		Ok(())
	}
}


// PointOfInteraction1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PointOfInteraction1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: GenericIdentification32,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SysNm", skip_serializing_if = "Option::is_none") )]
	pub sys_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrpId", skip_serializing_if = "Option::is_none") )]
	pub grp_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cpblties", skip_serializing_if = "Option::is_none") )]
	pub cpblties: Option<PointOfInteractionCapabilities1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cmpnt", skip_serializing_if = "Option::is_none") )]
	pub cmpnt: Option<Vec<PointOfInteractionComponent1>>,
}

impl PointOfInteraction1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.id.validate()?;
		if let Some(ref val) = self.sys_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sys_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "sys_nm exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.grp_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "grp_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "grp_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.cpblties { val.validate()? }
		if let Some(ref vec) = self.cmpnt { for item in vec { item.validate()? } }
		Ok(())
	}
}


// PointOfInteractionCapabilities1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PointOfInteractionCapabilities1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CardRdngCpblties", skip_serializing_if = "Option::is_none") )]
	pub card_rdng_cpblties: Option<Vec<CardDataReading1Code>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CrdhldrVrfctnCpblties", skip_serializing_if = "Option::is_none") )]
	pub crdhldr_vrfctn_cpblties: Option<Vec<CardholderVerificationCapability1Code>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OnLineCpblties", skip_serializing_if = "Option::is_none") )]
	pub on_line_cpblties: Option<OnLineCapability1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DispCpblties", skip_serializing_if = "Option::is_none") )]
	pub disp_cpblties: Option<Vec<DisplayCapabilities1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtLineWidth", skip_serializing_if = "Option::is_none") )]
	pub prt_line_width: Option<String>,
}

impl PointOfInteractionCapabilities1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.card_rdng_cpblties { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.crdhldr_vrfctn_cpblties { for item in vec { item.validate()? } }
		if let Some(ref val) = self.on_line_cpblties { val.validate()? }
		if let Some(ref vec) = self.disp_cpblties { for item in vec { item.validate()? } }
		if let Some(ref val) = self.prt_line_width {
			let pattern = Regex::new("[0-9]{1,3}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "prt_line_width does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// PointOfInteractionComponent1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PointOfInteractionComponent1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "POICmpntTp") )]
	pub poi_cmpnt_tp: POIComponentType1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ManfctrId", skip_serializing_if = "Option::is_none") )]
	pub manfctr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mdl", skip_serializing_if = "Option::is_none") )]
	pub mdl: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VrsnNb", skip_serializing_if = "Option::is_none") )]
	pub vrsn_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SrlNb", skip_serializing_if = "Option::is_none") )]
	pub srl_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ApprvlNb", skip_serializing_if = "Option::is_none") )]
	pub apprvl_nb: Option<Vec<String>>,
}

impl PointOfInteractionComponent1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.poi_cmpnt_tp.validate()?;
		if let Some(ref val) = self.manfctr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "manfctr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "manfctr_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.mdl {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mdl is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mdl exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.vrsn_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "vrsn_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "vrsn_nb exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.srl_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "srl_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "srl_nb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.apprvl_nb {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "apprvl_nb is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 70 {
					return Err(ValidationError::new(1002, "apprvl_nb exceeds the maximum length of 70".to_string()));
				}
			}
		}
		Ok(())
	}
}


// PostalAddress1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PostalAddress1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdrTp", skip_serializing_if = "Option::is_none") )]
	pub adr_tp: Option<AddressType2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdrLine", skip_serializing_if = "Option::is_none") )]
	pub adr_line: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StrtNm", skip_serializing_if = "Option::is_none") )]
	pub strt_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNb", skip_serializing_if = "Option::is_none") )]
	pub bldg_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstCd", skip_serializing_if = "Option::is_none") )]
	pub pst_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TwnNm", skip_serializing_if = "Option::is_none") )]
	pub twn_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none") )]
	pub ctry_sub_dvsn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry") )]
	pub ctry: String,
}

impl PostalAddress1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.adr_tp { val.validate()? }
		if let Some(ref vec) = self.adr_line {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "adr_line is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 70 {
					return Err(ValidationError::new(1002, "adr_line exceeds the maximum length of 70".to_string()));
				}
			}
		}
		if let Some(ref val) = self.strt_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "strt_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "strt_nm exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.bldg_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "bldg_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "bldg_nb exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.pst_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pst_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "pst_cd exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.twn_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "twn_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "twn_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ctry_sub_dvsn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ctry_sub_dvsn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ctry_sub_dvsn exceeds the maximum length of 35".to_string()));
			}
		}
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.ctry) {
			return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
		}
		Ok(())
	}
}


// PostalAddress24 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PostalAddress24 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdrTp", skip_serializing_if = "Option::is_none") )]
	pub adr_tp: Option<AddressType3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dept", skip_serializing_if = "Option::is_none") )]
	pub dept: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SubDept", skip_serializing_if = "Option::is_none") )]
	pub sub_dept: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StrtNm", skip_serializing_if = "Option::is_none") )]
	pub strt_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNb", skip_serializing_if = "Option::is_none") )]
	pub bldg_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BldgNm", skip_serializing_if = "Option::is_none") )]
	pub bldg_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Flr", skip_serializing_if = "Option::is_none") )]
	pub flr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstBx", skip_serializing_if = "Option::is_none") )]
	pub pst_bx: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Room", skip_serializing_if = "Option::is_none") )]
	pub room: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstCd", skip_serializing_if = "Option::is_none") )]
	pub pst_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TwnNm", skip_serializing_if = "Option::is_none") )]
	pub twn_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TwnLctnNm", skip_serializing_if = "Option::is_none") )]
	pub twn_lctn_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DstrctNm", skip_serializing_if = "Option::is_none") )]
	pub dstrct_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none") )]
	pub ctry_sub_dvsn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
	pub ctry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdrLine", skip_serializing_if = "Option::is_none") )]
	pub adr_line: Option<Vec<String>>,
}

impl PostalAddress24 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.adr_tp { val.validate()? }
		if let Some(ref val) = self.dept {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dept is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "dept exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.sub_dept {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "sub_dept is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "sub_dept exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.strt_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "strt_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "strt_nm exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.bldg_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "bldg_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "bldg_nb exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.bldg_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "bldg_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "bldg_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.flr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "flr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "flr exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.pst_bx {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pst_bx is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "pst_bx exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.room {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "room is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "room exceeds the maximum length of 70".to_string()));
			}
		}
		if let Some(ref val) = self.pst_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pst_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 16 {
				return Err(ValidationError::new(1002, "pst_cd exceeds the maximum length of 16".to_string()));
			}
		}
		if let Some(ref val) = self.twn_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "twn_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "twn_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.twn_lctn_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "twn_lctn_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "twn_lctn_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.dstrct_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dstrct_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "dstrct_nm exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ctry_sub_dvsn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ctry_sub_dvsn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ctry_sub_dvsn exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.adr_line {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "adr_line is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 70 {
					return Err(ValidationError::new(1002, "adr_line exceeds the maximum length of 70".to_string()));
				}
			}
		}
		Ok(())
	}
}


// PreferredContactMethod1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum PreferredContactMethod1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "LETT") )]
	CodeLETT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MAIL") )]
	CodeMAIL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PHON") )]
	CodePHON,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FAXX") )]
	CodeFAXX,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CELL") )]
	CodeCELL,
}

impl PreferredContactMethod1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Price7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Price7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: YieldedOrValueType1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Val") )]
	pub val: PriceRateOrAmount3Choice,
}

impl Price7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tp.validate()?;
		self.val.validate()?;
		Ok(())
	}
}


// PriceRateOrAmount3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct PriceRateOrAmount3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
	pub rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveOrHistoricCurrencyAnd13DecimalAmount>,
}

impl PriceRateOrAmount3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.amt { val.validate()? }
		Ok(())
	}
}


// PriceValueType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum PriceValueType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "DISC") )]
	CodeDISC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PREM") )]
	CodePREM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PARV") )]
	CodePARV,
}

impl PriceValueType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Priority2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum Priority2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "HIGH") )]
	CodeHIGH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NORM") )]
	CodeNORM,
}

impl Priority2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Priority3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum Priority3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "URGT") )]
	CodeURGT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HIGH") )]
	CodeHIGH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NORM") )]
	CodeNORM,
}

impl Priority3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Product2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Product2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PdctCd") )]
	pub pdct_cd: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitOfMeasr", skip_serializing_if = "Option::is_none") )]
	pub unit_of_measr: Option<UnitOfMeasure1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PdctQty", skip_serializing_if = "Option::is_none") )]
	pub pdct_qty: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UnitPric", skip_serializing_if = "Option::is_none") )]
	pub unit_pric: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PdctAmt", skip_serializing_if = "Option::is_none") )]
	pub pdct_amt: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxTp", skip_serializing_if = "Option::is_none") )]
	pub tax_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlPdctInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_pdct_inf: Option<String>,
}

impl Product2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.pdct_cd.chars().count() < 1 {
			return Err(ValidationError::new(1001, "pdct_cd is shorter than the minimum length of 1".to_string()));
		}
		if self.pdct_cd.chars().count() > 70 {
			return Err(ValidationError::new(1002, "pdct_cd exceeds the maximum length of 70".to_string()));
		}
		if let Some(ref val) = self.unit_of_measr { val.validate()? }
		if let Some(ref val) = self.tax_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tax_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tax_tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.addtl_pdct_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_pdct_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "addtl_pdct_inf exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// ProprietaryAgent4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ProprietaryAgent4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Agt") )]
	pub agt: BranchAndFinancialInstitutionIdentification6,
}

impl ProprietaryAgent4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
		}
		if self.tp.chars().count() > 35 {
			return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
		}
		self.agt.validate()?;
		Ok(())
	}
}


// ProprietaryBankTransactionCodeStructure1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ProprietaryBankTransactionCodeStructure1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
	pub cd: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl ProprietaryBankTransactionCodeStructure1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.cd.chars().count() < 1 {
			return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
		}
		if self.cd.chars().count() > 35 {
			return Err(ValidationError::new(1002, "cd exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// ProprietaryData5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ProprietaryData5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Data") )]
	pub data: SupplementaryDataEnvelope1,
}

impl ProprietaryData5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
		}
		if self.tp.chars().count() > 35 {
			return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
		}
		self.data.validate()?;
		Ok(())
	}
}


// ProprietaryDate3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ProprietaryDate3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt") )]
	pub dt: DateAndDateTime2Choice,
}

impl ProprietaryDate3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
		}
		if self.tp.chars().count() > 35 {
			return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
		}
		self.dt.validate()?;
		Ok(())
	}
}


// ProprietaryParty5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ProprietaryParty5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pty") )]
	pub pty: Party40Choice,
}

impl ProprietaryParty5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
		}
		if self.tp.chars().count() > 35 {
			return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
		}
		self.pty.validate()?;
		Ok(())
	}
}


// ProprietaryPrice2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ProprietaryPrice2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pric") )]
	pub pric: ActiveOrHistoricCurrencyAndAmount,
}

impl ProprietaryPrice2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
		}
		if self.tp.chars().count() > 35 {
			return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
		}
		self.pric.validate()?;
		Ok(())
	}
}


// ProprietaryQuantity1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ProprietaryQuantity1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Qty") )]
	pub qty: String,
}

impl ProprietaryQuantity1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
		}
		if self.tp.chars().count() > 35 {
			return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
		}
		if self.qty.chars().count() < 1 {
			return Err(ValidationError::new(1001, "qty is shorter than the minimum length of 1".to_string()));
		}
		if self.qty.chars().count() > 35 {
			return Err(ValidationError::new(1002, "qty exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// ProprietaryReference1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ProprietaryReference1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ref") )]
	pub ref_attr: String,
}

impl ProprietaryReference1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.tp.chars().count() < 1 {
			return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
		}
		if self.tp.chars().count() > 35 {
			return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
		}
		if self.ref_attr.chars().count() < 1 {
			return Err(ValidationError::new(1001, "ref_attr is shorter than the minimum length of 1".to_string()));
		}
		if self.ref_attr.chars().count() > 35 {
			return Err(ValidationError::new(1002, "ref_attr exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// ProxyAccountIdentification1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ProxyAccountIdentification1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<ProxyAccountType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
}

impl ProxyAccountIdentification1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 2048 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 2048".to_string()));
		}
		Ok(())
	}
}


// ProxyAccountType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ProxyAccountType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ProxyAccountType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// Purpose2Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Purpose2Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl Purpose2Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// QueryType3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum QueryType3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ALLL") )]
	CodeALLL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CHNG") )]
	CodeCHNG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MODF") )]
	CodeMODF,
}

impl QueryType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// Rate4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct Rate4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: RateType4Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "VldtyRg", skip_serializing_if = "Option::is_none") )]
	pub vldty_rg: Option<ActiveOrHistoricCurrencyAndAmountRange2>,
}

impl Rate4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.tp.validate()?;
		if let Some(ref val) = self.vldty_rg { val.validate()? }
		Ok(())
	}
}


// RateType4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RateType4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Pctg", skip_serializing_if = "Option::is_none") )]
	pub pctg: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Othr", skip_serializing_if = "Option::is_none") )]
	pub othr: Option<String>,
}

impl RateType4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.othr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "othr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "othr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// ReceiptAcknowledgementReport2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ReceiptAcknowledgementReport2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdRef") )]
	pub rltd_ref: MessageReference1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqHdlg") )]
	pub req_hdlg: RequestHandling2,
}

impl ReceiptAcknowledgementReport2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.rltd_ref.validate()?;
		self.req_hdlg.validate()?;
		Ok(())
	}
}


// ReferredDocumentInformation7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ReferredDocumentInformation7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<ReferredDocumentType4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nb", skip_serializing_if = "Option::is_none") )]
	pub nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RltdDt", skip_serializing_if = "Option::is_none") )]
	pub rltd_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LineDtls", skip_serializing_if = "Option::is_none") )]
	pub line_dtls: Option<Vec<DocumentLineInformation1>>,
}

impl ReferredDocumentInformation7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "nb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.line_dtls { for item in vec { item.validate()? } }
		Ok(())
	}
}


// ReferredDocumentType3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ReferredDocumentType3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<DocumentType6Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ReferredDocumentType3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd { val.validate()? }
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// ReferredDocumentType4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ReferredDocumentType4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdOrPrtry") )]
	pub cd_or_prtry: ReferredDocumentType3Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Issr", skip_serializing_if = "Option::is_none") )]
	pub issr: Option<String>,
}

impl ReferredDocumentType4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd_or_prtry.validate()?;
		if let Some(ref val) = self.issr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "issr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "issr exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// RegulatoryAuthority2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RegulatoryAuthority2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
	pub ctry: Option<String>,
}

impl RegulatoryAuthority2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// RegulatoryReporting3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RegulatoryReporting3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtCdtRptgInd", skip_serializing_if = "Option::is_none") )]
	pub dbt_cdt_rptg_ind: Option<RegulatoryReportingType1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Authrty", skip_serializing_if = "Option::is_none") )]
	pub authrty: Option<RegulatoryAuthority2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dtls", skip_serializing_if = "Option::is_none") )]
	pub dtls: Option<Vec<StructuredRegulatoryReporting3>>,
}

impl RegulatoryReporting3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.dbt_cdt_rptg_ind { val.validate()? }
		if let Some(ref val) = self.authrty { val.validate()? }
		if let Some(ref vec) = self.dtls { for item in vec { item.validate()? } }
		Ok(())
	}
}


// RegulatoryReportingType1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum RegulatoryReportingType1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "CRED") )]
	CodeCRED,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DEBT") )]
	CodeDEBT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BOTH") )]
	CodeBOTH,
}

impl RegulatoryReportingType1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// RejectionReason2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RejectionReason2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RjctgPtyRsn") )]
	pub rjctg_pty_rsn: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RjctnDtTm", skip_serializing_if = "Option::is_none") )]
	pub rjctn_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ErrLctn", skip_serializing_if = "Option::is_none") )]
	pub err_lctn: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RsnDesc", skip_serializing_if = "Option::is_none") )]
	pub rsn_desc: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlData", skip_serializing_if = "Option::is_none") )]
	pub addtl_data: Option<String>,
}

impl RejectionReason2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.rjctg_pty_rsn.chars().count() < 1 {
			return Err(ValidationError::new(1001, "rjctg_pty_rsn is shorter than the minimum length of 1".to_string()));
		}
		if self.rjctg_pty_rsn.chars().count() > 35 {
			return Err(ValidationError::new(1002, "rjctg_pty_rsn exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.err_lctn {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "err_lctn is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "err_lctn exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.rsn_desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rsn_desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "rsn_desc exceeds the maximum length of 350".to_string()));
			}
		}
		if let Some(ref val) = self.addtl_data {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_data is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 20000 {
				return Err(ValidationError::new(1002, "addtl_data exceeds the maximum length of 20000".to_string()));
			}
		}
		Ok(())
	}
}


// RemittanceAmount2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RemittanceAmount2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DuePyblAmt", skip_serializing_if = "Option::is_none") )]
	pub due_pybl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DscntApldAmt", skip_serializing_if = "Option::is_none") )]
	pub dscnt_apld_amt: Option<Vec<DiscountAmountAndType1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtNoteAmt", skip_serializing_if = "Option::is_none") )]
	pub cdt_note_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none") )]
	pub tax_amt: Option<Vec<TaxAmountAndType1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdjstmntAmtAndRsn", skip_serializing_if = "Option::is_none") )]
	pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none") )]
	pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl RemittanceAmount2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.due_pybl_amt { val.validate()? }
		if let Some(ref vec) = self.dscnt_apld_amt { for item in vec { item.validate()? } }
		if let Some(ref val) = self.cdt_note_amt { val.validate()? }
		if let Some(ref vec) = self.tax_amt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.adjstmnt_amt_and_rsn { for item in vec { item.validate()? } }
		if let Some(ref val) = self.rmtd_amt { val.validate()? }
		Ok(())
	}
}


// RemittanceAmount3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RemittanceAmount3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DuePyblAmt", skip_serializing_if = "Option::is_none") )]
	pub due_pybl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DscntApldAmt", skip_serializing_if = "Option::is_none") )]
	pub dscnt_apld_amt: Option<Vec<DiscountAmountAndType1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtNoteAmt", skip_serializing_if = "Option::is_none") )]
	pub cdt_note_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none") )]
	pub tax_amt: Option<Vec<TaxAmountAndType1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdjstmntAmtAndRsn", skip_serializing_if = "Option::is_none") )]
	pub adjstmnt_amt_and_rsn: Option<Vec<DocumentAdjustment1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtdAmt", skip_serializing_if = "Option::is_none") )]
	pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl RemittanceAmount3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.due_pybl_amt { val.validate()? }
		if let Some(ref vec) = self.dscnt_apld_amt { for item in vec { item.validate()? } }
		if let Some(ref val) = self.cdt_note_amt { val.validate()? }
		if let Some(ref vec) = self.tax_amt { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.adjstmnt_amt_and_rsn { for item in vec { item.validate()? } }
		if let Some(ref val) = self.rmtd_amt { val.validate()? }
		Ok(())
	}
}


// RemittanceInformation16 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RemittanceInformation16 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ustrd", skip_serializing_if = "Option::is_none") )]
	pub ustrd: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Strd", skip_serializing_if = "Option::is_none") )]
	pub strd: Option<Vec<StructuredRemittanceInformation16>>,
}

impl RemittanceInformation16 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.ustrd {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "ustrd is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 140 {
					return Err(ValidationError::new(1002, "ustrd exceeds the maximum length of 140".to_string()));
				}
			}
		}
		if let Some(ref vec) = self.strd { for item in vec { item.validate()? } }
		Ok(())
	}
}


// RemittanceInformation2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RemittanceInformation2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ustrd", skip_serializing_if = "Option::is_none") )]
	pub ustrd: Option<Vec<String>>,
}

impl RemittanceInformation2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.ustrd {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "ustrd is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 140 {
					return Err(ValidationError::new(1002, "ustrd exceeds the maximum length of 140".to_string()));
				}
			}
		}
		Ok(())
	}
}


// RemittanceLocation7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RemittanceLocation7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtId", skip_serializing_if = "Option::is_none") )]
	pub rmt_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtLctnDtls", skip_serializing_if = "Option::is_none") )]
	pub rmt_lctn_dtls: Option<Vec<RemittanceLocationData1>>,
}

impl RemittanceLocation7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.rmt_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "rmt_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "rmt_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.rmt_lctn_dtls { for item in vec { item.validate()? } }
		Ok(())
	}
}


// RemittanceLocationData1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RemittanceLocationData1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mtd") )]
	pub mtd: RemittanceLocationMethod2Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ElctrncAdr", skip_serializing_if = "Option::is_none") )]
	pub elctrnc_adr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none") )]
	pub pstl_adr: Option<NameAndAddress16>,
}

impl RemittanceLocationData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.mtd.validate()?;
		if let Some(ref val) = self.elctrnc_adr {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "elctrnc_adr is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 2048 {
				return Err(ValidationError::new(1002, "elctrnc_adr exceeds the maximum length of 2048".to_string()));
			}
		}
		if let Some(ref val) = self.pstl_adr { val.validate()? }
		Ok(())
	}
}


// RemittanceLocationMethod2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum RemittanceLocationMethod2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FAXI") )]
	CodeFAXI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EDIC") )]
	CodeEDIC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "URID") )]
	CodeURID,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EMAL") )]
	CodeEMAL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "POST") )]
	CodePOST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SMSM") )]
	CodeSMSM,
}

impl RemittanceLocationMethod2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ReportEntry10 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ReportEntry10 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NtryRef", skip_serializing_if = "Option::is_none") )]
	pub ntry_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd") )]
	pub cdt_dbt_ind: CreditDebitCode,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RvslInd", skip_serializing_if = "Option::is_none") )]
	pub rvsl_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts") )]
	pub sts: EntryStatus1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BookgDt", skip_serializing_if = "Option::is_none") )]
	pub bookg_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValDt", skip_serializing_if = "Option::is_none") )]
	pub val_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrRef", skip_serializing_if = "Option::is_none") )]
	pub acct_svcr_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Avlbty", skip_serializing_if = "Option::is_none") )]
	pub avlbty: Option<Vec<CashAvailability1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BkTxCd") )]
	pub bk_tx_cd: BankTransactionCodeStructure4,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ComssnWvrInd", skip_serializing_if = "Option::is_none") )]
	pub comssn_wvr_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInfInd", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf_ind: Option<MessageIdentification2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AmtDtls", skip_serializing_if = "Option::is_none") )]
	pub amt_dtls: Option<AmountAndCurrencyExchange3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Chrgs", skip_serializing_if = "Option::is_none") )]
	pub chrgs: Option<Charges6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TechInptChanl", skip_serializing_if = "Option::is_none") )]
	pub tech_inpt_chanl: Option<TechnicalInputChannel1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Intrst", skip_serializing_if = "Option::is_none") )]
	pub intrst: Option<TransactionInterest4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CardTx", skip_serializing_if = "Option::is_none") )]
	pub card_tx: Option<CardEntry4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NtryDtls", skip_serializing_if = "Option::is_none") )]
	pub ntry_dtls: Option<Vec<EntryDetails9>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlNtryInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_ntry_inf: Option<String>,
}

impl ReportEntry10 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ntry_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ntry_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ntry_ref exceeds the maximum length of 35".to_string()));
			}
		}
		self.amt.validate()?;
		self.cdt_dbt_ind.validate()?;
		self.sts.validate()?;
		if let Some(ref val) = self.bookg_dt { val.validate()? }
		if let Some(ref val) = self.val_dt { val.validate()? }
		if let Some(ref val) = self.acct_svcr_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_svcr_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acct_svcr_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.avlbty { for item in vec { item.validate()? } }
		self.bk_tx_cd.validate()?;
		if let Some(ref val) = self.addtl_inf_ind { val.validate()? }
		if let Some(ref val) = self.amt_dtls { val.validate()? }
		if let Some(ref val) = self.chrgs { val.validate()? }
		if let Some(ref val) = self.tech_inpt_chanl { val.validate()? }
		if let Some(ref val) = self.intrst { val.validate()? }
		if let Some(ref val) = self.card_tx { val.validate()? }
		if let Some(ref vec) = self.ntry_dtls { for item in vec { item.validate()? } }
		if let Some(ref val) = self.addtl_ntry_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_ntry_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 500 {
				return Err(ValidationError::new(1002, "addtl_ntry_inf exceeds the maximum length of 500".to_string()));
			}
		}
		Ok(())
	}
}


// ReportingPeriod2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ReportingPeriod2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrToDt") )]
	pub fr_to_dt: DatePeriodDetails1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrToTm", skip_serializing_if = "Option::is_none") )]
	pub fr_to_tm: Option<TimePeriodDetails1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp") )]
	pub tp: QueryType3Code,
}

impl ReportingPeriod2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.fr_to_dt.validate()?;
		if let Some(ref val) = self.fr_to_tm { val.validate()? }
		self.tp.validate()?;
		Ok(())
	}
}


// ReportingRequest5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ReportingRequest5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdMsgNmId") )]
	pub reqd_msg_nm_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Acct", skip_serializing_if = "Option::is_none") )]
	pub acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctOwnr") )]
	pub acct_ownr: Party40Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcr", skip_serializing_if = "Option::is_none") )]
	pub acct_svcr: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgPrd", skip_serializing_if = "Option::is_none") )]
	pub rptg_prd: Option<ReportingPeriod2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RptgSeq", skip_serializing_if = "Option::is_none") )]
	pub rptg_seq: Option<SequenceRange1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdTxTp", skip_serializing_if = "Option::is_none") )]
	pub reqd_tx_tp: Option<TransactionType2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdBalTp", skip_serializing_if = "Option::is_none") )]
	pub reqd_bal_tp: Option<Vec<BalanceType13>>,
}

impl ReportingRequest5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
		}
		if self.reqd_msg_nm_id.chars().count() < 15 {
			return Err(ValidationError::new(1001, "reqd_msg_nm_id is shorter than the minimum length of 15".to_string()));
		}
		if self.reqd_msg_nm_id.chars().count() > 15 {
			return Err(ValidationError::new(1002, "reqd_msg_nm_id exceeds the maximum length of 15".to_string()));
		}
		let pattern = Regex::new("[a-z]{4,4}[.]{1,1}[0-9]{3,3}[.]{1,1}001[.]{1,1}[0-9]{2,2}").unwrap();
		if !pattern.is_match(&self.reqd_msg_nm_id) {
			return Err(ValidationError::new(1005, "reqd_msg_nm_id does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.acct { val.validate()? }
		self.acct_ownr.validate()?;
		if let Some(ref val) = self.acct_svcr { val.validate()? }
		if let Some(ref val) = self.rptg_prd { val.validate()? }
		if let Some(ref val) = self.rptg_seq { val.validate()? }
		if let Some(ref val) = self.reqd_tx_tp { val.validate()? }
		if let Some(ref vec) = self.reqd_bal_tp { for item in vec { item.validate()? } }
		Ok(())
	}
}


// ReportingSource1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ReportingSource1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ReportingSource1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// RequestHandling2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RequestHandling2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsCd") )]
	pub sts_cd: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StsDtTm", skip_serializing_if = "Option::is_none") )]
	pub sts_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
}

impl RequestHandling2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.sts_cd.chars().count() < 1 {
			return Err(ValidationError::new(1001, "sts_cd is shorter than the minimum length of 1".to_string()));
		}
		if self.sts_cd.chars().count() > 4 {
			return Err(ValidationError::new(1002, "sts_cd exceeds the maximum length of 4".to_string()));
		}
		let pattern = Regex::new("[a-zA-Z0-9]{1,4}").unwrap();
		if !pattern.is_match(&self.sts_cd) {
			return Err(ValidationError::new(1005, "sts_cd does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// RequestType4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct RequestType4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtCtrl", skip_serializing_if = "Option::is_none") )]
	pub pmt_ctrl: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Enqry", skip_serializing_if = "Option::is_none") )]
	pub enqry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<GenericIdentification1>,
}

impl RequestType4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.pmt_ctrl {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pmt_ctrl is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "pmt_ctrl exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.enqry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "enqry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "enqry exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// ResendSearchCriteria2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ResendSearchCriteria2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "BizDt", skip_serializing_if = "Option::is_none") )]
	pub biz_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SeqNb", skip_serializing_if = "Option::is_none") )]
	pub seq_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SeqRg", skip_serializing_if = "Option::is_none") )]
	pub seq_rg: Option<SequenceRange1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgNmId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_msg_nm_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FileRef", skip_serializing_if = "Option::is_none") )]
	pub file_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rcpt") )]
	pub rcpt: PartyIdentification136,
}

impl ResendSearchCriteria2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.seq_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "seq_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "seq_nb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.seq_rg { val.validate()? }
		if let Some(ref val) = self.orgnl_msg_nm_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_msg_nm_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_msg_nm_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.file_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "file_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "file_ref exceeds the maximum length of 35".to_string()));
			}
		}
		self.rcpt.validate()?;
		Ok(())
	}
}


// ResolutionData1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ResolutionData1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none") )]
	pub end_to_end_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxId", skip_serializing_if = "Option::is_none") )]
	pub tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UETR", skip_serializing_if = "Option::is_none") )]
	pub uetr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmAmt", skip_serializing_if = "Option::is_none") )]
	pub intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none") )]
	pub intr_bk_sttlm_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrChanl", skip_serializing_if = "Option::is_none") )]
	pub clr_chanl: Option<ClearingChannel2Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Compstn", skip_serializing_if = "Option::is_none") )]
	pub compstn: Option<Compensation2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Chrgs", skip_serializing_if = "Option::is_none") )]
	pub chrgs: Option<Vec<Charges7>>,
}

impl ResolutionData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.end_to_end_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "end_to_end_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "end_to_end_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.tx_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tx_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tx_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.uetr {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "uetr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.intr_bk_sttlm_amt { val.validate()? }
		if let Some(ref val) = self.clr_chanl { val.validate()? }
		if let Some(ref val) = self.compstn { val.validate()? }
		if let Some(ref vec) = self.chrgs { for item in vec { item.validate()? } }
		Ok(())
	}
}


// ReturnReason5Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ReturnReason5Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ReturnReason5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// SecuritiesAccount19 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecuritiesAccount19 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id") )]
	pub id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<GenericIdentification30>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
}

impl SecuritiesAccount19 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
		}
		if self.id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 70 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 70".to_string()));
			}
		}
		Ok(())
	}
}


// SecurityIdentification19 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SecurityIdentification19 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "ISIN", skip_serializing_if = "Option::is_none") )]
	pub isin: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OthrId", skip_serializing_if = "Option::is_none") )]
	pub othr_id: Option<Vec<OtherIdentification1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Desc", skip_serializing_if = "Option::is_none") )]
	pub desc: Option<String>,
}

impl SecurityIdentification19 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.isin {
			let pattern = Regex::new("[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "isin does not match the required pattern".to_string()));
			}
		}
		if let Some(ref vec) = self.othr_id { for item in vec { item.validate()? } }
		if let Some(ref val) = self.desc {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "desc is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "desc exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// SequenceRange1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SequenceRange1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrSeq") )]
	pub fr_seq: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToSeq") )]
	pub to_seq: String,
}

impl SequenceRange1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.fr_seq.chars().count() < 1 {
			return Err(ValidationError::new(1001, "fr_seq is shorter than the minimum length of 1".to_string()));
		}
		if self.fr_seq.chars().count() > 35 {
			return Err(ValidationError::new(1002, "fr_seq exceeds the maximum length of 35".to_string()));
		}
		if self.to_seq.chars().count() < 1 {
			return Err(ValidationError::new(1001, "to_seq is shorter than the minimum length of 1".to_string()));
		}
		if self.to_seq.chars().count() > 35 {
			return Err(ValidationError::new(1002, "to_seq exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// SequenceRange1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SequenceRange1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrSeq", skip_serializing_if = "Option::is_none") )]
	pub fr_seq: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToSeq", skip_serializing_if = "Option::is_none") )]
	pub to_seq: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrToSeq", skip_serializing_if = "Option::is_none") )]
	pub fr_to_seq: Option<Vec<SequenceRange1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EQSeq", skip_serializing_if = "Option::is_none") )]
	pub eq_seq: Option<Vec<String>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NEQSeq", skip_serializing_if = "Option::is_none") )]
	pub neq_seq: Option<Vec<String>>,
}

impl SequenceRange1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.fr_seq {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "fr_seq is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "fr_seq exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.to_seq {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "to_seq is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "to_seq exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.fr_to_seq { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.eq_seq {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "eq_seq is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 35 {
					return Err(ValidationError::new(1002, "eq_seq exceeds the maximum length of 35".to_string()));
				}
			}
		}
		if let Some(ref vec) = self.neq_seq {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "neq_seq is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 35 {
					return Err(ValidationError::new(1002, "neq_seq exceeds the maximum length of 35".to_string()));
				}
			}
		}
		Ok(())
	}
}


// SequenceType3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum SequenceType3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "FRST") )]
	CodeFRST,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RCUR") )]
	CodeRCUR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FNAL") )]
	CodeFNAL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OOFF") )]
	CodeOOFF,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RPRE") )]
	CodeRPRE,
}

impl SequenceType3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// ServiceLevel8Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct ServiceLevel8Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl ServiceLevel8Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// SettlementDateTimeIndication1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementDateTimeIndication1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtDtTm", skip_serializing_if = "Option::is_none") )]
	pub dbt_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDtTm", skip_serializing_if = "Option::is_none") )]
	pub cdt_dt_tm: Option<String>,
}

impl SettlementDateTimeIndication1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SettlementInstruction7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementInstruction7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmMtd") )]
	pub sttlm_mtd: SettlementMethod1Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmAcct", skip_serializing_if = "Option::is_none") )]
	pub sttlm_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSys", skip_serializing_if = "Option::is_none") )]
	pub clr_sys: Option<ClearingSystemIdentification3Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstgRmbrsmntAgt", skip_serializing_if = "Option::is_none") )]
	pub instg_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstgRmbrsmntAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub instg_rmbrsmnt_agt_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdRmbrsmntAgt", skip_serializing_if = "Option::is_none") )]
	pub instd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdRmbrsmntAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub instd_rmbrsmnt_agt_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ThrdRmbrsmntAgt", skip_serializing_if = "Option::is_none") )]
	pub thrd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ThrdRmbrsmntAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub thrd_rmbrsmnt_agt_acct: Option<CashAccount38>,
}

impl SettlementInstruction7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.sttlm_mtd.validate()?;
		if let Some(ref val) = self.sttlm_acct { val.validate()? }
		if let Some(ref val) = self.clr_sys { val.validate()? }
		if let Some(ref val) = self.instg_rmbrsmnt_agt { val.validate()? }
		if let Some(ref val) = self.instg_rmbrsmnt_agt_acct { val.validate()? }
		if let Some(ref val) = self.instd_rmbrsmnt_agt { val.validate()? }
		if let Some(ref val) = self.instd_rmbrsmnt_agt_acct { val.validate()? }
		if let Some(ref val) = self.thrd_rmbrsmnt_agt { val.validate()? }
		if let Some(ref val) = self.thrd_rmbrsmnt_agt_acct { val.validate()? }
		Ok(())
	}
}


// SettlementMethod1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum SettlementMethod1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "INDA") )]
	CodeINDA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INGA") )]
	CodeINGA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "COVE") )]
	CodeCOVE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLRG") )]
	CodeCLRG,
}

impl SettlementMethod1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SettlementTimeRequest2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SettlementTimeRequest2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "CLSTm", skip_serializing_if = "Option::is_none") )]
	pub cls_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TillTm", skip_serializing_if = "Option::is_none") )]
	pub till_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrTm", skip_serializing_if = "Option::is_none") )]
	pub fr_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RjctTm", skip_serializing_if = "Option::is_none") )]
	pub rjct_tm: Option<String>,
}

impl SettlementTimeRequest2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SignatureEnvelope ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SignatureEnvelope {
}

impl SignatureEnvelope {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// SkipPayload ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SkipPayload {
}

impl SkipPayload {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// StatementResolutionEntry4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct StatementResolutionEntry4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none") )]
	pub orgnl_grp_inf: Option<OriginalGroupInformation29>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlStmtId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_stmt_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UETR", skip_serializing_if = "Option::is_none") )]
	pub uetr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrRef", skip_serializing_if = "Option::is_none") )]
	pub acct_svcr_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CrrctdAmt", skip_serializing_if = "Option::is_none") )]
	pub crrctd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Chrgs", skip_serializing_if = "Option::is_none") )]
	pub chrgs: Option<Vec<Charges6>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Purp", skip_serializing_if = "Option::is_none") )]
	pub purp: Option<Purpose2Choice>,
}

impl StatementResolutionEntry4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_grp_inf { val.validate()? }
		if let Some(ref val) = self.orgnl_stmt_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_stmt_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_stmt_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.uetr {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "uetr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.acct_svcr_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_svcr_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acct_svcr_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.crrctd_amt { val.validate()? }
		if let Some(ref vec) = self.chrgs { for item in vec { item.validate()? } }
		if let Some(ref val) = self.purp { val.validate()? }
		Ok(())
	}
}


// StatusReason6Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct StatusReason6Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl StatusReason6Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// StatusReasonInformation12 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct StatusReasonInformation12 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Orgtr", skip_serializing_if = "Option::is_none") )]
	pub orgtr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rsn", skip_serializing_if = "Option::is_none") )]
	pub rsn: Option<StatusReason6Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<Vec<String>>,
}

impl StatusReasonInformation12 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgtr { val.validate()? }
		if let Some(ref val) = self.rsn { val.validate()? }
		if let Some(ref vec) = self.addtl_inf {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 105 {
					return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 105".to_string()));
				}
			}
		}
		Ok(())
	}
}


// StructuredRegulatoryReporting3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct StructuredRegulatoryReporting3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctry", skip_serializing_if = "Option::is_none") )]
	pub ctry: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Inf", skip_serializing_if = "Option::is_none") )]
	pub inf: Option<Vec<String>>,
}

impl StructuredRegulatoryReporting3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ctry {
			let pattern = Regex::new("[A-Z]{2,2}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "ctry does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 10 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 10".to_string()));
			}
		}
		if let Some(ref val) = self.amt { val.validate()? }
		if let Some(ref vec) = self.inf {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "inf is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 35 {
					return Err(ValidationError::new(1002, "inf exceeds the maximum length of 35".to_string()));
				}
			}
		}
		Ok(())
	}
}


// StructuredRemittanceInformation16 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct StructuredRemittanceInformation16 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "RfrdDocInf", skip_serializing_if = "Option::is_none") )]
	pub rfrd_doc_inf: Option<Vec<ReferredDocumentInformation7>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RfrdDocAmt", skip_serializing_if = "Option::is_none") )]
	pub rfrd_doc_amt: Option<RemittanceAmount2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrRefInf", skip_serializing_if = "Option::is_none") )]
	pub cdtr_ref_inf: Option<CreditorReferenceInformation2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Invcr", skip_serializing_if = "Option::is_none") )]
	pub invcr: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Invcee", skip_serializing_if = "Option::is_none") )]
	pub invcee: Option<PartyIdentification135>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxRmt", skip_serializing_if = "Option::is_none") )]
	pub tax_rmt: Option<TaxInformation7>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GrnshmtRmt", skip_serializing_if = "Option::is_none") )]
	pub grnshmt_rmt: Option<Garnishment3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlRmtInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_rmt_inf: Option<Vec<String>>,
}

impl StructuredRemittanceInformation16 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.rfrd_doc_inf { for item in vec { item.validate()? } }
		if let Some(ref val) = self.rfrd_doc_amt { val.validate()? }
		if let Some(ref val) = self.cdtr_ref_inf { val.validate()? }
		if let Some(ref val) = self.invcr { val.validate()? }
		if let Some(ref val) = self.invcee { val.validate()? }
		if let Some(ref val) = self.tax_rmt { val.validate()? }
		if let Some(ref val) = self.grnshmt_rmt { val.validate()? }
		if let Some(ref vec) = self.addtl_rmt_inf {
			for item in vec {
				if item.chars().count() < 1 {
					return Err(ValidationError::new(1001, "addtl_rmt_inf is shorter than the minimum length of 1".to_string()));
				}
				if item.chars().count() > 140 {
					return Err(ValidationError::new(1002, "addtl_rmt_inf exceeds the maximum length of 140".to_string()));
				}
			}
		}
		Ok(())
	}
}


// SupplementaryData1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SupplementaryData1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none") )]
	pub plc_and_nm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Envlp") )]
	pub envlp: SupplementaryDataEnvelope1,
}

impl SupplementaryData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.plc_and_nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "plc_and_nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 350 {
				return Err(ValidationError::new(1002, "plc_and_nm exceeds the maximum length of 350".to_string()));
			}
		}
		self.envlp.validate()?;
		Ok(())
	}
}


// SupplementaryDataEnvelope1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TaxAmount2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TaxAmount2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
	pub rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxblBaseAmt", skip_serializing_if = "Option::is_none") )]
	pub taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlAmt", skip_serializing_if = "Option::is_none") )]
	pub ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dtls", skip_serializing_if = "Option::is_none") )]
	pub dtls: Option<Vec<TaxRecordDetails2>>,
}

impl TaxAmount2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.taxbl_base_amt { val.validate()? }
		if let Some(ref val) = self.ttl_amt { val.validate()? }
		if let Some(ref vec) = self.dtls { for item in vec { item.validate()? } }
		Ok(())
	}
}


// TaxAmountAndType1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TaxAmountAndType1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<TaxAmountType1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}

impl TaxAmountAndType1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		self.amt.validate()?;
		Ok(())
	}
}


// TaxAmountType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TaxAmountType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl TaxAmountType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// TaxAuthorisation1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TaxAuthorisation1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Titl", skip_serializing_if = "Option::is_none") )]
	pub titl: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Nm", skip_serializing_if = "Option::is_none") )]
	pub nm: Option<String>,
}

impl TaxAuthorisation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.titl {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "titl is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "titl exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.nm {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "nm is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "nm exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// TaxCharges2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TaxCharges2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Id", skip_serializing_if = "Option::is_none") )]
	pub id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rate", skip_serializing_if = "Option::is_none") )]
	pub rate: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt", skip_serializing_if = "Option::is_none") )]
	pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}

impl TaxCharges2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.amt { val.validate()? }
		Ok(())
	}
}


// TaxInformation7 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TaxInformation7 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr", skip_serializing_if = "Option::is_none") )]
	pub cdtr: Option<TaxParty1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr", skip_serializing_if = "Option::is_none") )]
	pub dbtr: Option<TaxParty2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<TaxParty2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdmstnZone", skip_serializing_if = "Option::is_none") )]
	pub admstn_zone: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefNb", skip_serializing_if = "Option::is_none") )]
	pub ref_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mtd", skip_serializing_if = "Option::is_none") )]
	pub mtd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlTaxblBaseAmt", skip_serializing_if = "Option::is_none") )]
	pub ttl_taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlTaxAmt", skip_serializing_if = "Option::is_none") )]
	pub ttl_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SeqNb", skip_serializing_if = "Option::is_none") )]
	pub seq_nb: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rcrd", skip_serializing_if = "Option::is_none") )]
	pub rcrd: Option<Vec<TaxRecord2>>,
}

impl TaxInformation7 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cdtr { val.validate()? }
		if let Some(ref val) = self.dbtr { val.validate()? }
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.admstn_zone {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "admstn_zone is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "admstn_zone exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ref_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ref_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "ref_nb exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.mtd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mtd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mtd exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ttl_taxbl_base_amt { val.validate()? }
		if let Some(ref val) = self.ttl_tax_amt { val.validate()? }
		if let Some(ref vec) = self.rcrd { for item in vec { item.validate()? } }
		Ok(())
	}
}


// TaxInformation8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TaxInformation8 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr", skip_serializing_if = "Option::is_none") )]
	pub cdtr: Option<TaxParty1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr", skip_serializing_if = "Option::is_none") )]
	pub dbtr: Option<TaxParty2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AdmstnZone", skip_serializing_if = "Option::is_none") )]
	pub admstn_zone: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RefNb", skip_serializing_if = "Option::is_none") )]
	pub ref_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Mtd", skip_serializing_if = "Option::is_none") )]
	pub mtd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlTaxblBaseAmt", skip_serializing_if = "Option::is_none") )]
	pub ttl_taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlTaxAmt", skip_serializing_if = "Option::is_none") )]
	pub ttl_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SeqNb", skip_serializing_if = "Option::is_none") )]
	pub seq_nb: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rcrd", skip_serializing_if = "Option::is_none") )]
	pub rcrd: Option<Vec<TaxRecord2>>,
}

impl TaxInformation8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cdtr { val.validate()? }
		if let Some(ref val) = self.dbtr { val.validate()? }
		if let Some(ref val) = self.admstn_zone {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "admstn_zone is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "admstn_zone exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ref_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ref_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "ref_nb exceeds the maximum length of 140".to_string()));
			}
		}
		if let Some(ref val) = self.mtd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mtd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mtd exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ttl_taxbl_base_amt { val.validate()? }
		if let Some(ref val) = self.ttl_tax_amt { val.validate()? }
		if let Some(ref vec) = self.rcrd { for item in vec { item.validate()? } }
		Ok(())
	}
}


// TaxParty1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TaxParty1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxId", skip_serializing_if = "Option::is_none") )]
	pub tax_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegnId", skip_serializing_if = "Option::is_none") )]
	pub regn_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxTp", skip_serializing_if = "Option::is_none") )]
	pub tax_tp: Option<String>,
}

impl TaxParty1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tax_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tax_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tax_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.regn_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "regn_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "regn_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.tax_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tax_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tax_tp exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// TaxParty2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TaxParty2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxId", skip_serializing_if = "Option::is_none") )]
	pub tax_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RegnId", skip_serializing_if = "Option::is_none") )]
	pub regn_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxTp", skip_serializing_if = "Option::is_none") )]
	pub tax_tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Authstn", skip_serializing_if = "Option::is_none") )]
	pub authstn: Option<TaxAuthorisation1>,
}

impl TaxParty2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tax_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tax_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tax_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.regn_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "regn_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "regn_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.tax_tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tax_tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tax_tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.authstn { val.validate()? }
		Ok(())
	}
}


// TaxPeriod2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TaxPeriod2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Yr", skip_serializing_if = "Option::is_none") )]
	pub yr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<TaxRecordPeriod1Code>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrToDt", skip_serializing_if = "Option::is_none") )]
	pub fr_to_dt: Option<DatePeriod2>,
}

impl TaxPeriod2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp { val.validate()? }
		if let Some(ref val) = self.fr_to_dt { val.validate()? }
		Ok(())
	}
}


// TaxRecord2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TaxRecord2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Tp", skip_serializing_if = "Option::is_none") )]
	pub tp: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Ctgy", skip_serializing_if = "Option::is_none") )]
	pub ctgy: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CtgyDtls", skip_serializing_if = "Option::is_none") )]
	pub ctgy_dtls: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrSts", skip_serializing_if = "Option::is_none") )]
	pub dbtr_sts: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CertId", skip_serializing_if = "Option::is_none") )]
	pub cert_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrmsCd", skip_serializing_if = "Option::is_none") )]
	pub frms_cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prd", skip_serializing_if = "Option::is_none") )]
	pub prd: Option<TaxPeriod2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TaxAmt", skip_serializing_if = "Option::is_none") )]
	pub tax_amt: Option<TaxAmount2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_inf: Option<String>,
}

impl TaxRecord2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.tp {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tp is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tp exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ctgy {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ctgy is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ctgy exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.ctgy_dtls {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "ctgy_dtls is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "ctgy_dtls exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.dbtr_sts {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "dbtr_sts is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "dbtr_sts exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.cert_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cert_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "cert_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.frms_cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "frms_cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "frms_cd exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.prd { val.validate()? }
		if let Some(ref val) = self.tax_amt { val.validate()? }
		if let Some(ref val) = self.addtl_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "addtl_inf exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// TaxRecordDetails2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TaxRecordDetails2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prd", skip_serializing_if = "Option::is_none") )]
	pub prd: Option<TaxPeriod2>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Amt") )]
	pub amt: ActiveOrHistoricCurrencyAndAmount,
}

impl TaxRecordDetails2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.prd { val.validate()? }
		self.amt.validate()?;
		Ok(())
	}
}


// TaxRecordPeriod1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TaxRecordPeriod1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM01") )]
	CodeMM01,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM02") )]
	CodeMM02,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM03") )]
	CodeMM03,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM04") )]
	CodeMM04,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM05") )]
	CodeMM05,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM06") )]
	CodeMM06,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM07") )]
	CodeMM07,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM08") )]
	CodeMM08,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM09") )]
	CodeMM09,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM10") )]
	CodeMM10,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM11") )]
	CodeMM11,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM12") )]
	CodeMM12,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QTR1") )]
	CodeQTR1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QTR2") )]
	CodeQTR2,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QTR3") )]
	CodeQTR3,
	#[cfg_attr( feature = "derive_serde", serde(rename = "QTR4") )]
	CodeQTR4,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HLF1") )]
	CodeHLF1,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HLF2") )]
	CodeHLF2,
}

impl TaxRecordPeriod1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TechnicalInputChannel1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TechnicalInputChannel1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd", skip_serializing_if = "Option::is_none") )]
	pub cd: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<String>,
}

impl TechnicalInputChannel1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.cd {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "cd is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 4 {
				return Err(ValidationError::new(1002, "cd exceeds the maximum length of 4".to_string()));
			}
		}
		if let Some(ref val) = self.prtry {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prtry is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prtry exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// TimePeriodDetails1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TimePeriodDetails1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FrTm") )]
	pub fr_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ToTm", skip_serializing_if = "Option::is_none") )]
	pub to_tm: Option<String>,
}

impl TimePeriodDetails1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TotalTransactions6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TotalTransactions6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNtries", skip_serializing_if = "Option::is_none") )]
	pub ttl_ntries: Option<NumberAndSumOfTransactions4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlCdtNtries", skip_serializing_if = "Option::is_none") )]
	pub ttl_cdt_ntries: Option<NumberAndSumOfTransactions1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlDbtNtries", skip_serializing_if = "Option::is_none") )]
	pub ttl_dbt_ntries: Option<NumberAndSumOfTransactions1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNtriesPerBkTxCd", skip_serializing_if = "Option::is_none") )]
	pub ttl_ntries_per_bk_tx_cd: Option<Vec<TotalsPerBankTransactionCode5>>,
}

impl TotalTransactions6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ttl_ntries { val.validate()? }
		if let Some(ref val) = self.ttl_cdt_ntries { val.validate()? }
		if let Some(ref val) = self.ttl_dbt_ntries { val.validate()? }
		if let Some(ref vec) = self.ttl_ntries_per_bk_tx_cd { for item in vec { item.validate()? } }
		Ok(())
	}
}


// TotalsPerBankTransactionCode5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TotalsPerBankTransactionCode5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "NbOfNtries", skip_serializing_if = "Option::is_none") )]
	pub nb_of_ntries: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sum", skip_serializing_if = "Option::is_none") )]
	pub sum: Option<f64>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlNetNtry", skip_serializing_if = "Option::is_none") )]
	pub ttl_net_ntry: Option<AmountAndDirection35>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtNtries", skip_serializing_if = "Option::is_none") )]
	pub cdt_ntries: Option<NumberAndSumOfTransactions1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtNtries", skip_serializing_if = "Option::is_none") )]
	pub dbt_ntries: Option<NumberAndSumOfTransactions1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FcstInd", skip_serializing_if = "Option::is_none") )]
	pub fcst_ind: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "BkTxCd") )]
	pub bk_tx_cd: BankTransactionCodeStructure4,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Avlbty", skip_serializing_if = "Option::is_none") )]
	pub avlbty: Option<Vec<CashAvailability1>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dt", skip_serializing_if = "Option::is_none") )]
	pub dt: Option<DateAndDateTime2Choice>,
}

impl TotalsPerBankTransactionCode5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.nb_of_ntries {
			let pattern = Regex::new("[0-9]{1,15}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "nb_of_ntries does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.ttl_net_ntry { val.validate()? }
		if let Some(ref val) = self.cdt_ntries { val.validate()? }
		if let Some(ref val) = self.dbt_ntries { val.validate()? }
		self.bk_tx_cd.validate()?;
		if let Some(ref vec) = self.avlbty { for item in vec { item.validate()? } }
		if let Some(ref val) = self.dt { val.validate()? }
		Ok(())
	}
}


// TrackData1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TrackData1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrckNb", skip_serializing_if = "Option::is_none") )]
	pub trck_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TrckVal") )]
	pub trck_val: String,
}

impl TrackData1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.trck_nb {
			let pattern = Regex::new("[0-9]").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "trck_nb does not match the required pattern".to_string()));
			}
		}
		if self.trck_val.chars().count() < 1 {
			return Err(ValidationError::new(1001, "trck_val is shorter than the minimum length of 1".to_string()));
		}
		if self.trck_val.chars().count() > 140 {
			return Err(ValidationError::new(1002, "trck_val exceeds the maximum length of 140".to_string()));
		}
		Ok(())
	}
}


// TransactionAgents5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TransactionAgents5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstgAgt", skip_serializing_if = "Option::is_none") )]
	pub instg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstdAgt", skip_serializing_if = "Option::is_none") )]
	pub instd_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RcvgAgt", skip_serializing_if = "Option::is_none") )]
	pub rcvg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DlvrgAgt", skip_serializing_if = "Option::is_none") )]
	pub dlvrg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IssgAgt", skip_serializing_if = "Option::is_none") )]
	pub issg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmPlc", skip_serializing_if = "Option::is_none") )]
	pub sttlm_plc: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<Vec<ProprietaryAgent4>>,
}

impl TransactionAgents5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.instg_agt { val.validate()? }
		if let Some(ref val) = self.instd_agt { val.validate()? }
		if let Some(ref val) = self.dbtr_agt { val.validate()? }
		if let Some(ref val) = self.cdtr_agt { val.validate()? }
		if let Some(ref val) = self.intrmy_agt1 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt2 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt3 { val.validate()? }
		if let Some(ref val) = self.rcvg_agt { val.validate()? }
		if let Some(ref val) = self.dlvrg_agt { val.validate()? }
		if let Some(ref val) = self.issg_agt { val.validate()? }
		if let Some(ref val) = self.sttlm_plc { val.validate()? }
		if let Some(ref vec) = self.prtry { for item in vec { item.validate()? } }
		Ok(())
	}
}


// TransactionChannel1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TransactionChannel1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MAIL") )]
	CodeMAIL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TLPH") )]
	CodeTLPH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ECOM") )]
	CodeECOM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TVPY") )]
	CodeTVPY,
}

impl TransactionChannel1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TransactionDates3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TransactionDates3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AccptncDtTm", skip_serializing_if = "Option::is_none") )]
	pub accptnc_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradActvtyCtrctlSttlmDt", skip_serializing_if = "Option::is_none") )]
	pub trad_actvty_ctrctl_sttlm_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradDt", skip_serializing_if = "Option::is_none") )]
	pub trad_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBkSttlmDt", skip_serializing_if = "Option::is_none") )]
	pub intr_bk_sttlm_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StartDt", skip_serializing_if = "Option::is_none") )]
	pub start_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndDt", skip_serializing_if = "Option::is_none") )]
	pub end_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxDtTm", skip_serializing_if = "Option::is_none") )]
	pub tx_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<Vec<ProprietaryDate3>>,
}

impl TransactionDates3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref vec) = self.prtry { for item in vec { item.validate()? } }
		Ok(())
	}
}


// TransactionEnvironment1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TransactionEnvironment1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MERC") )]
	CodeMERC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PRIV") )]
	CodePRIV,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUBL") )]
	CodePUBL,
}

impl TransactionEnvironment1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TransactionIdentifier1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TransactionIdentifier1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxDtTm") )]
	pub tx_dt_tm: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxRef") )]
	pub tx_ref: String,
}

impl TransactionIdentifier1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.tx_ref.chars().count() < 1 {
			return Err(ValidationError::new(1001, "tx_ref is shorter than the minimum length of 1".to_string()));
		}
		if self.tx_ref.chars().count() > 35 {
			return Err(ValidationError::new(1002, "tx_ref exceeds the maximum length of 35".to_string()));
		}
		Ok(())
	}
}


// TransactionIndividualStatus1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum TransactionIndividualStatus1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACTC") )]
	CodeACTC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "RJCT") )]
	CodeRJCT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PDNG") )]
	CodePDNG,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACCP") )]
	CodeACCP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACSP") )]
	CodeACSP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACSC") )]
	CodeACSC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACCR") )]
	CodeACCR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACWC") )]
	CodeACWC,
}

impl TransactionIndividualStatus1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// TransactionInterest4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TransactionInterest4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "TtlIntrstAndTaxAmt", skip_serializing_if = "Option::is_none") )]
	pub ttl_intrst_and_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Rcrd", skip_serializing_if = "Option::is_none") )]
	pub rcrd: Option<Vec<InterestRecord2>>,
}

impl TransactionInterest4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ttl_intrst_and_tax_amt { val.validate()? }
		if let Some(ref vec) = self.rcrd { for item in vec { item.validate()? } }
		Ok(())
	}
}


// TransactionParties6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TransactionParties6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitgPty", skip_serializing_if = "Option::is_none") )]
	pub initg_pty: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr", skip_serializing_if = "Option::is_none") )]
	pub dbtr: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr", skip_serializing_if = "Option::is_none") )]
	pub cdtr: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TradgPty", skip_serializing_if = "Option::is_none") )]
	pub tradg_pty: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<Vec<ProprietaryParty5>>,
}

impl TransactionParties6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.initg_pty { val.validate()? }
		if let Some(ref val) = self.dbtr { val.validate()? }
		if let Some(ref val) = self.dbtr_acct { val.validate()? }
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		if let Some(ref val) = self.cdtr { val.validate()? }
		if let Some(ref val) = self.cdtr_acct { val.validate()? }
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		if let Some(ref val) = self.tradg_pty { val.validate()? }
		if let Some(ref vec) = self.prtry { for item in vec { item.validate()? } }
		Ok(())
	}
}


// TransactionParties8 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TransactionParties8 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtDbtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_dbtr: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Dbtr") )]
	pub dbtr: Party40Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InitgPty", skip_serializing_if = "Option::is_none") )]
	pub initg_pty: Option<Party40Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgt", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub dbtr_agt_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt1", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt1Acct", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt1_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt2", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt2Acct", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt2_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt3", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrvsInstgAgt3Acct", skip_serializing_if = "Option::is_none") )]
	pub prvs_instg_agt3_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt1", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt1Acct", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt1_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt2", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt2Acct", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt2_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt3", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrmyAgt3Acct", skip_serializing_if = "Option::is_none") )]
	pub intrmy_agt3_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgt", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAgtAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_agt_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cdtr") )]
	pub cdtr: Party40Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrAcct", skip_serializing_if = "Option::is_none") )]
	pub cdtr_acct: Option<CashAccount38>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UltmtCdtr", skip_serializing_if = "Option::is_none") )]
	pub ultmt_cdtr: Option<Party40Choice>,
}

impl TransactionParties8 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.ultmt_dbtr { val.validate()? }
		self.dbtr.validate()?;
		if let Some(ref val) = self.dbtr_acct { val.validate()? }
		if let Some(ref val) = self.initg_pty { val.validate()? }
		if let Some(ref val) = self.dbtr_agt { val.validate()? }
		if let Some(ref val) = self.dbtr_agt_acct { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt1 { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt1_acct { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt2 { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt2_acct { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt3 { val.validate()? }
		if let Some(ref val) = self.prvs_instg_agt3_acct { val.validate()? }
		if let Some(ref val) = self.intrmy_agt1 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt1_acct { val.validate()? }
		if let Some(ref val) = self.intrmy_agt2 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt2_acct { val.validate()? }
		if let Some(ref val) = self.intrmy_agt3 { val.validate()? }
		if let Some(ref val) = self.intrmy_agt3_acct { val.validate()? }
		if let Some(ref val) = self.cdtr_agt { val.validate()? }
		if let Some(ref val) = self.cdtr_agt_acct { val.validate()? }
		self.cdtr.validate()?;
		if let Some(ref val) = self.cdtr_acct { val.validate()? }
		if let Some(ref val) = self.ultmt_cdtr { val.validate()? }
		Ok(())
	}
}


// TransactionPrice4Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TransactionPrice4Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "DealPric", skip_serializing_if = "Option::is_none") )]
	pub deal_pric: Option<Price7>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<Vec<ProprietaryPrice2>>,
}

impl TransactionPrice4Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.deal_pric { val.validate()? }
		if let Some(ref vec) = self.prtry { for item in vec { item.validate()? } }
		Ok(())
	}
}


// TransactionQuantities3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TransactionQuantities3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Qty", skip_serializing_if = "Option::is_none") )]
	pub qty: Option<FinancialInstrumentQuantity1Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlAndCurFaceAmt", skip_serializing_if = "Option::is_none") )]
	pub orgnl_and_cur_face_amt: Option<OriginalAndCurrentQuantities1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<ProprietaryQuantity1>,
}

impl TransactionQuantities3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.qty { val.validate()? }
		if let Some(ref val) = self.orgnl_and_cur_face_amt { val.validate()? }
		if let Some(ref val) = self.prtry { val.validate()? }
		Ok(())
	}
}


// TransactionReferences6 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TransactionReferences6 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "MsgId", skip_serializing_if = "Option::is_none") )]
	pub msg_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrRef", skip_serializing_if = "Option::is_none") )]
	pub acct_svcr_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtInfId", skip_serializing_if = "Option::is_none") )]
	pub pmt_inf_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "InstrId", skip_serializing_if = "Option::is_none") )]
	pub instr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EndToEndId", skip_serializing_if = "Option::is_none") )]
	pub end_to_end_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "UETR", skip_serializing_if = "Option::is_none") )]
	pub uetr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxId", skip_serializing_if = "Option::is_none") )]
	pub tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtId", skip_serializing_if = "Option::is_none") )]
	pub mndt_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ChqNb", skip_serializing_if = "Option::is_none") )]
	pub chq_nb: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ClrSysRef", skip_serializing_if = "Option::is_none") )]
	pub clr_sys_ref: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctOwnrTxId", skip_serializing_if = "Option::is_none") )]
	pub acct_ownr_tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSvcrTxId", skip_serializing_if = "Option::is_none") )]
	pub acct_svcr_tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MktInfrstrctrTxId", skip_serializing_if = "Option::is_none") )]
	pub mkt_infrstrctr_tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PrcgId", skip_serializing_if = "Option::is_none") )]
	pub prcg_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Prtry", skip_serializing_if = "Option::is_none") )]
	pub prtry: Option<Vec<ProprietaryReference1>>,
}

impl TransactionReferences6 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.msg_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "msg_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "msg_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.acct_svcr_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_svcr_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acct_svcr_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.pmt_inf_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "pmt_inf_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "pmt_inf_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.instr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "instr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "instr_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.end_to_end_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "end_to_end_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "end_to_end_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.uetr {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "uetr does not match the required pattern".to_string()));
			}
		}
		if let Some(ref val) = self.tx_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "tx_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "tx_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.mndt_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mndt_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mndt_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.chq_nb {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "chq_nb is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "chq_nb exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.clr_sys_ref {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "clr_sys_ref is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "clr_sys_ref exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.acct_ownr_tx_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_ownr_tx_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acct_ownr_tx_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.acct_svcr_tx_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "acct_svcr_tx_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "acct_svcr_tx_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.mkt_infrstrctr_tx_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "mkt_infrstrctr_tx_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "mkt_infrstrctr_tx_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.prcg_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "prcg_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "prcg_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref vec) = self.prtry { for item in vec { item.validate()? } }
		Ok(())
	}
}


// TransactionType2 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct TransactionType2 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Sts") )]
	pub sts: EntryStatus1Choice,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtDbtInd") )]
	pub cdt_dbt_ind: CreditDebitCode,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FlrLmt", skip_serializing_if = "Option::is_none") )]
	pub flr_lmt: Option<Vec<Limit2>>,
}

impl TransactionType2 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.sts.validate()?;
		self.cdt_dbt_ind.validate()?;
		if let Some(ref vec) = self.flr_lmt { for item in vec { item.validate()? } }
		Ok(())
	}
}


// UnableToApplyIncorrect1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UnableToApplyIncorrect1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
	pub cd: UnableToApplyIncorrectInformation4Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlIncrrctInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_incrrct_inf: Option<String>,
}

impl UnableToApplyIncorrect1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd.validate()?;
		if let Some(ref val) = self.addtl_incrrct_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_incrrct_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "addtl_incrrct_inf exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// UnableToApplyIncorrectInformation4Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum UnableToApplyIncorrectInformation4Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "IN01") )]
	CodeIN01,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IN02") )]
	CodeIN02,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IN03") )]
	CodeIN03,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IN04") )]
	CodeIN04,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IN05") )]
	CodeIN05,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IN06") )]
	CodeIN06,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IN07") )]
	CodeIN07,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IN08") )]
	CodeIN08,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IN09") )]
	CodeIN09,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IN10") )]
	CodeIN10,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IN11") )]
	CodeIN11,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IN12") )]
	CodeIN12,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IN13") )]
	CodeIN13,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IN14") )]
	CodeIN14,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IN15") )]
	CodeIN15,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IN16") )]
	CodeIN16,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IN17") )]
	CodeIN17,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IN18") )]
	CodeIN18,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IN19") )]
	CodeIN19,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM20") )]
	CodeMM20,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM21") )]
	CodeMM21,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM22") )]
	CodeMM22,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM25") )]
	CodeMM25,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM26") )]
	CodeMM26,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM27") )]
	CodeMM27,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM28") )]
	CodeMM28,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM29") )]
	CodeMM29,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM30") )]
	CodeMM30,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM31") )]
	CodeMM31,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM32") )]
	CodeMM32,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IN33") )]
	CodeIN33,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM34") )]
	CodeMM34,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MM35") )]
	CodeMM35,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IN36") )]
	CodeIN36,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IN37") )]
	CodeIN37,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IN38") )]
	CodeIN38,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IN39") )]
	CodeIN39,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NARR") )]
	CodeNARR,
}

impl UnableToApplyIncorrectInformation4Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// UnableToApplyJustification3Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UnableToApplyJustification3Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "AnyInf", skip_serializing_if = "Option::is_none") )]
	pub any_inf: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MssngOrIncrrctInf", skip_serializing_if = "Option::is_none") )]
	pub mssng_or_incrrct_inf: Option<MissingOrIncorrectInformation3>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PssblDplctInstr", skip_serializing_if = "Option::is_none") )]
	pub pssbl_dplct_instr: Option<bool>,
}

impl UnableToApplyJustification3Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.mssng_or_incrrct_inf { val.validate()? }
		Ok(())
	}
}


// UnableToApplyMissing1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UnableToApplyMissing1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Cd") )]
	pub cd: UnableToApplyMissingInformation3Code,
	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlMssngInf", skip_serializing_if = "Option::is_none") )]
	pub addtl_mssng_inf: Option<String>,
}

impl UnableToApplyMissing1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.cd.validate()?;
		if let Some(ref val) = self.addtl_mssng_inf {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "addtl_mssng_inf is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 140 {
				return Err(ValidationError::new(1002, "addtl_mssng_inf exceeds the maximum length of 140".to_string()));
			}
		}
		Ok(())
	}
}


// UnableToApplyMissingInformation3Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum UnableToApplyMissingInformation3Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MS01") )]
	CodeMS01,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MS02") )]
	CodeMS02,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MS03") )]
	CodeMS03,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MS04") )]
	CodeMS04,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MS05") )]
	CodeMS05,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MS06") )]
	CodeMS06,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MS07") )]
	CodeMS07,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MS08") )]
	CodeMS08,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MS09") )]
	CodeMS09,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MS10") )]
	CodeMS10,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MS11") )]
	CodeMS11,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MS12") )]
	CodeMS12,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MS13") )]
	CodeMS13,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MS14") )]
	CodeMS14,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MS15") )]
	CodeMS15,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MS16") )]
	CodeMS16,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MS17") )]
	CodeMS17,
	#[cfg_attr( feature = "derive_serde", serde(rename = "NARR") )]
	CodeNARR,
}

impl UnableToApplyMissingInformation3Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// UnderlyingGroupInformation1 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UnderlyingGroupInformation1 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgId") )]
	pub orgnl_msg_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgNmId") )]
	pub orgnl_msg_nm_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlCreDtTm", skip_serializing_if = "Option::is_none") )]
	pub orgnl_cre_dt_tm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlMsgDlvryChanl", skip_serializing_if = "Option::is_none") )]
	pub orgnl_msg_dlvry_chanl: Option<String>,
}

impl UnderlyingGroupInformation1 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if self.orgnl_msg_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_msg_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_msg_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_msg_id exceeds the maximum length of 35".to_string()));
		}
		if self.orgnl_msg_nm_id.chars().count() < 1 {
			return Err(ValidationError::new(1001, "orgnl_msg_nm_id is shorter than the minimum length of 1".to_string()));
		}
		if self.orgnl_msg_nm_id.chars().count() > 35 {
			return Err(ValidationError::new(1002, "orgnl_msg_nm_id exceeds the maximum length of 35".to_string()));
		}
		if let Some(ref val) = self.orgnl_msg_dlvry_chanl {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_msg_dlvry_chanl is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_msg_dlvry_chanl exceeds the maximum length of 35".to_string()));
			}
		}
		Ok(())
	}
}


// UnderlyingPaymentInstruction5 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UnderlyingPaymentInstruction5 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none") )]
	pub orgnl_grp_inf: Option<UnderlyingGroupInformation1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlPmtInfId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_pmt_inf_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_instr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_end_to_end_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none") )]
	pub orgnl_uetr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlInstdAmt") )]
	pub orgnl_instd_amt: ActiveOrHistoricCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdExctnDt", skip_serializing_if = "Option::is_none") )]
	pub reqd_exctn_dt: Option<DateAndDateTime2Choice>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqdColltnDt", skip_serializing_if = "Option::is_none") )]
	pub reqd_colltn_dt: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none") )]
	pub orgnl_tx_ref: Option<OriginalTransactionReference28>,
}

impl UnderlyingPaymentInstruction5 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_grp_inf { val.validate()? }
		if let Some(ref val) = self.orgnl_pmt_inf_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_pmt_inf_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_pmt_inf_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_instr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_instr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_instr_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_end_to_end_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_end_to_end_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_end_to_end_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_uetr {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_uetr does not match the required pattern".to_string()));
			}
		}
		self.orgnl_instd_amt.validate()?;
		if let Some(ref val) = self.reqd_exctn_dt { val.validate()? }
		if let Some(ref val) = self.orgnl_tx_ref { val.validate()? }
		Ok(())
	}
}


// UnderlyingPaymentTransaction4 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UnderlyingPaymentTransaction4 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none") )]
	pub orgnl_grp_inf: Option<UnderlyingGroupInformation1>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlInstrId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_instr_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlEndToEndId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_end_to_end_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTxId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_tx_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none") )]
	pub orgnl_uetr: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlIntrBkSttlmAmt") )]
	pub orgnl_intr_bk_sttlm_amt: ActiveOrHistoricCurrencyAndAmount,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlIntrBkSttlmDt") )]
	pub orgnl_intr_bk_sttlm_dt: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlTxRef", skip_serializing_if = "Option::is_none") )]
	pub orgnl_tx_ref: Option<OriginalTransactionReference28>,
}

impl UnderlyingPaymentTransaction4 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_grp_inf { val.validate()? }
		if let Some(ref val) = self.orgnl_instr_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_instr_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_instr_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_end_to_end_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_end_to_end_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_end_to_end_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_tx_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_tx_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_tx_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_uetr {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_uetr does not match the required pattern".to_string()));
			}
		}
		self.orgnl_intr_bk_sttlm_amt.validate()?;
		if let Some(ref val) = self.orgnl_tx_ref { val.validate()? }
		Ok(())
	}
}


// UnderlyingStatementEntry3 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UnderlyingStatementEntry3 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlGrpInf", skip_serializing_if = "Option::is_none") )]
	pub orgnl_grp_inf: Option<OriginalGroupInformation29>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlStmtId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_stmt_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlNtryId", skip_serializing_if = "Option::is_none") )]
	pub orgnl_ntry_id: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlUETR", skip_serializing_if = "Option::is_none") )]
	pub orgnl_uetr: Option<String>,
}

impl UnderlyingStatementEntry3 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_grp_inf { val.validate()? }
		if let Some(ref val) = self.orgnl_stmt_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_stmt_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_stmt_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_ntry_id {
			if val.chars().count() < 1 {
				return Err(ValidationError::new(1001, "orgnl_ntry_id is shorter than the minimum length of 1".to_string()));
			}
			if val.chars().count() > 35 {
				return Err(ValidationError::new(1002, "orgnl_ntry_id exceeds the maximum length of 35".to_string()));
			}
		}
		if let Some(ref val) = self.orgnl_uetr {
			let pattern = Regex::new("[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "orgnl_uetr does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// UnderlyingTransaction22 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UnderlyingTransaction22 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlGrpInfAndSts", skip_serializing_if = "Option::is_none") )]
	pub orgnl_grp_inf_and_sts: Option<OriginalGroupHeader14>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlPmtInfAndSts", skip_serializing_if = "Option::is_none") )]
	pub orgnl_pmt_inf_and_sts: Option<Vec<OriginalPaymentInstruction30>>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxInfAndSts", skip_serializing_if = "Option::is_none") )]
	pub tx_inf_and_sts: Option<Vec<PaymentTransaction102>>,
}

impl UnderlyingTransaction22 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_grp_inf_and_sts { val.validate()? }
		if let Some(ref vec) = self.orgnl_pmt_inf_and_sts { for item in vec { item.validate()? } }
		if let Some(ref vec) = self.tx_inf_and_sts { for item in vec { item.validate()? } }
		Ok(())
	}
}


// UnderlyingTransaction23 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UnderlyingTransaction23 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlGrpInfAndCxl", skip_serializing_if = "Option::is_none") )]
	pub orgnl_grp_inf_and_cxl: Option<OriginalGroupHeader15>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TxInf", skip_serializing_if = "Option::is_none") )]
	pub tx_inf: Option<Vec<PaymentTransaction106>>,
}

impl UnderlyingTransaction23 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_grp_inf_and_cxl { val.validate()? }
		if let Some(ref vec) = self.tx_inf { for item in vec { item.validate()? } }
		Ok(())
	}
}


// UnderlyingTransaction27 ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UnderlyingTransaction27 {
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlGrpInfAndCxl", skip_serializing_if = "Option::is_none") )]
	pub orgnl_grp_inf_and_cxl: Option<OriginalGroupHeader15>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "OrgnlPmtInfAndCxl", skip_serializing_if = "Option::is_none") )]
	pub orgnl_pmt_inf_and_cxl: Option<Vec<OriginalPaymentInstruction36>>,
}

impl UnderlyingTransaction27 {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.orgnl_grp_inf_and_cxl { val.validate()? }
		if let Some(ref vec) = self.orgnl_pmt_inf_and_cxl { for item in vec { item.validate()? } }
		Ok(())
	}
}


// UnderlyingTransaction5Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct UnderlyingTransaction5Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Initn", skip_serializing_if = "Option::is_none") )]
	pub initn: Option<UnderlyingPaymentInstruction5>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "IntrBk", skip_serializing_if = "Option::is_none") )]
	pub intr_bk: Option<UnderlyingPaymentTransaction4>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StmtNtry", skip_serializing_if = "Option::is_none") )]
	pub stmt_ntry: Option<UnderlyingStatementEntry3>,
}

impl UnderlyingTransaction5Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.initn { val.validate()? }
		if let Some(ref val) = self.intr_bk { val.validate()? }
		if let Some(ref val) = self.stmt_ntry { val.validate()? }
		Ok(())
	}
}


// UnitOfMeasure1Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum UnitOfMeasure1Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "PIEC") )]
	CodePIEC,
	#[cfg_attr( feature = "derive_serde", serde(rename = "TONS") )]
	CodeTONS,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FOOT") )]
	CodeFOOT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GBGA") )]
	CodeGBGA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USGA") )]
	CodeUSGA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GRAM") )]
	CodeGRAM,
	#[cfg_attr( feature = "derive_serde", serde(rename = "INCH") )]
	CodeINCH,
	#[cfg_attr( feature = "derive_serde", serde(rename = "KILO") )]
	CodeKILO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "PUND") )]
	CodePUND,
	#[cfg_attr( feature = "derive_serde", serde(rename = "METR") )]
	CodeMETR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CMET") )]
	CodeCMET,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MMET") )]
	CodeMMET,
	#[cfg_attr( feature = "derive_serde", serde(rename = "LITR") )]
	CodeLITR,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CELI") )]
	CodeCELI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MILI") )]
	CodeMILI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GBOU") )]
	CodeGBOU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USOU") )]
	CodeUSOU,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GBQA") )]
	CodeGBQA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USQA") )]
	CodeUSQA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "GBPI") )]
	CodeGBPI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "USPI") )]
	CodeUSPI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "MILE") )]
	CodeMILE,
	#[cfg_attr( feature = "derive_serde", serde(rename = "KMET") )]
	CodeKMET,
	#[cfg_attr( feature = "derive_serde", serde(rename = "YARD") )]
	CodeYARD,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SQKI") )]
	CodeSQKI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "HECT") )]
	CodeHECT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ARES") )]
	CodeARES,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SMET") )]
	CodeSMET,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SCMT") )]
	CodeSCMT,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SMIL") )]
	CodeSMIL,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SQMI") )]
	CodeSQMI,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SQYA") )]
	CodeSQYA,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SQFO") )]
	CodeSQFO,
	#[cfg_attr( feature = "derive_serde", serde(rename = "SQIN") )]
	CodeSQIN,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ACRE") )]
	CodeACRE,
}

impl UnitOfMeasure1Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// UserInterface2Code ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub enum UserInterface2Code {
	#[cfg_attr(feature = "derive_default", default)]
	#[cfg_attr( feature = "derive_serde", serde(rename = "MDSP") )]
	CodeMDSP,
	#[cfg_attr( feature = "derive_serde", serde(rename = "CDSP") )]
	CodeCDSP,
}

impl UserInterface2Code {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// YieldedOrValueType1Choice ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct YieldedOrValueType1Choice {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Yldd", skip_serializing_if = "Option::is_none") )]
	pub yldd: Option<bool>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "ValTp", skip_serializing_if = "Option::is_none") )]
	pub val_tp: Option<PriceValueType1Code>,
}

impl YieldedOrValueType1Choice {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.val_tp { val.validate()? }
		Ok(())
	}
}



// FedNowMessageSignatureKey ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowMessageSignatureKey {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowKeyID") )]
	pub fed_now_key_id: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Name") )]
	pub name: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "EncodedPublicKey") )]
	pub encoded_public_key: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Encoding") )]
	pub encoding: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "Algorithm", skip_serializing_if = "Option::is_none") )]
	pub algorithm: Option<String>,
	#[cfg_attr( feature = "derive_serde", serde(rename = "KeyCreationDateTime", skip_serializing_if = "Option::is_none") )]
	pub key_creation_date_time: Option<String>,
}

impl FedNowMessageSignatureKey {
	pub fn validate(&self) -> Result<(), ValidationError> {
		let pattern = Regex::new("[A-Za-z0-9\\-_]{1,300}").unwrap();
		if !pattern.is_match(&self.fed_now_key_id) {
			return Err(ValidationError::new(1005, "fed_now_key_id does not match the required pattern".to_string()));
		}
		let pattern = Regex::new("[A-Za-z0-9\\-_]{1,300}").unwrap();
		if !pattern.is_match(&self.name) {
			return Err(ValidationError::new(1005, "name does not match the required pattern".to_string()));
		}
		let pattern = Regex::new("[A-Za-z0-9\\-_]{1,50}").unwrap();
		if !pattern.is_match(&self.encoding) {
			return Err(ValidationError::new(1005, "encoding does not match the required pattern".to_string()));
		}
		if let Some(ref val) = self.algorithm {
			let pattern = Regex::new("[A-Za-z0-9\\-_]{1,50}").unwrap();
			if !pattern.is_match(val) {
				return Err(ValidationError::new(1005, "algorithm does not match the required pattern".to_string()));
			}
		}
		Ok(())
	}
}


// FedNowMessageSignatureKeyStatus ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowMessageSignatureKeyStatus {
	#[cfg_attr( feature = "derive_serde", serde(rename = "KeyStatus") )]
	pub key_status: String,
	#[cfg_attr( feature = "derive_serde", serde(rename = "StatusDateTime") )]
	pub status_date_time: String,
}

impl FedNowMessageSignatureKeyStatus {
	pub fn validate(&self) -> Result<(), ValidationError> {
		Ok(())
	}
}


// FedNowPublicKeyResponse ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct FedNowPublicKeyResponse {
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowMessageSignatureKeyStatus") )]
	pub fed_now_message_signature_key_status: FedNowMessageSignatureKeyStatus,
	#[cfg_attr( feature = "derive_serde", serde(rename = "FedNowMessageSignatureKey") )]
	pub fed_now_message_signature_key: FedNowMessageSignatureKey,
}

impl FedNowPublicKeyResponse {
	pub fn validate(&self) -> Result<(), ValidationError> {
		self.fed_now_message_signature_key_status.validate()?;
		self.fed_now_message_signature_key.validate()?;
		Ok(())
	}
}


// KeyAddition ...
#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
pub struct KeyAddition {
	#[cfg_attr( feature = "derive_serde", serde(rename = "Key", skip_serializing_if = "Option::is_none") )]
	pub key: Option<FedNowMessageSignatureKey>,
}

impl KeyAddition {
	pub fn validate(&self) -> Result<(), ValidationError> {
		if let Some(ref val) = self.key { val.validate()? }
		Ok(())
	}
}