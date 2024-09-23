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
