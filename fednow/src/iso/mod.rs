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


pub mod admi_002_001_01;
pub mod admi_004_001_02;
pub mod admi_006_001_01;
pub mod admi_007_001_01;
pub mod admi_011_001_01;
pub mod admi_998_001_02;
pub mod camt_026_001_07;
pub mod camt_028_001_09;
pub mod camt_029_001_09;
pub mod camt_052_001_08;
pub mod camt_054_001_08;
pub mod camt_055_001_09;
pub mod camt_056_001_08;
pub mod camt_060_001_05;
pub mod head_001_001_02;
pub mod pacs_002_001_10;
pub mod pacs_004_001_10;
pub mod pacs_008_001_08;
pub mod pacs_009_001_08;
pub mod pacs_028_001_03;
pub mod pain_013_001_07;
pub mod pain_014_001_07;
