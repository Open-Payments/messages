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

pub mod fednow_extra;
pub mod iso;
pub mod fednow_incoming_external;
pub mod fednow_outgoing_external;
pub mod document;
pub mod common;

use crate::fednow_incoming_external::*;
use crate::fednow_outgoing_external::*;

#[cfg(feature = "derive_serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
pub enum FednowMessage {
    #[cfg_attr( feature = "derive_serde", serde(rename = "FedNowIncoming") )]
    FedNowIncoming(Box<FedNowIncoming>),

    #[cfg_attr( feature = "derive_serde", serde(rename = "FedNowOutgoing") )]
    FedNowOutgoing(Box<FedNowOutgoing>),

    #[cfg_attr(feature = "derive_default", default)]
    UNKNOWN
}
