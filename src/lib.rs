// Auto-generated lib.rs
pub mod message;

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

use crate::message::fednow::fednow_incoming_external::FedNowIncomingMessage;
use crate::message::fednow::fednow_outgoing_external::FedNowOutgoingMessage;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FednowMessage {
    #[serde(rename = "FedNowIncomingMessage")]
    pub fed_now_incoming_message: Box<Option<FedNowIncomingMessage>>,

    #[serde(rename = "FedNowOutgoingMessage")]
    pub fed_now_outgoing_message: Box<Option<FedNowOutgoingMessage>>,
}