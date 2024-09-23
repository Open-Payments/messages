// Auto-generated lib.rs
pub mod message;

use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

use crate::message::fednow::fednow_incoming_external::FedNowIncoming;
use crate::message::fednow::fednow_outgoing_external::FedNowOutgoing;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum FednowMessage {
    #[serde(rename = "FedNowIncoming")]
    FedNowIncoming(Box<FedNowIncoming>),

    #[serde(rename = "FedNowOutgoing")]
    FedNowOutgoing(Box<FedNowOutgoing>),
}