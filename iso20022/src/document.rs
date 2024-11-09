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

#[cfg(feature = "derive_serde")]
use serde::{Deserialize, Serialize};
use open_payments_common::ValidationError;

#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_001_001_08::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_002_001_08::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_003_001_08::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_005_001_06::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_006_001_07::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_007_001_05::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_008_001_05::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_009_001_04::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_010_001_04::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_011_001_04::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_012_001_04::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_013_001_04::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_014_001_05::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_015_001_04::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_016_001_04::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_017_001_04::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_018_001_04::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_019_001_04::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_020_001_04::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_021_001_04::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_022_001_04::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_023_001_04::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_024_001_04::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_027_001_05::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_028_001_05::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_029_001_05::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_030_001_04::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_031_001_05::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_032_001_05::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_033_001_02::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_034_001_05::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_035_001_02::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_036_001_01::*;
#[cfg(feature = "acmt")] use open_payments_iso20022_acmt::acmt_037_001_02::*;

#[cfg(feature = "admi")] use open_payments_iso20022_admi::admi_002_001_01::*;
#[cfg(feature = "admi")] use open_payments_iso20022_admi::admi_004_001_02::*;
#[cfg(feature = "admi")] use open_payments_iso20022_admi::admi_005_001_02::*;
#[cfg(feature = "admi")] use open_payments_iso20022_admi::admi_006_001_01::*;
#[cfg(feature = "admi")] use open_payments_iso20022_admi::admi_007_001_01::*;
#[cfg(feature = "admi")] use open_payments_iso20022_admi::admi_009_001_02::*;
#[cfg(feature = "admi")] use open_payments_iso20022_admi::admi_010_001_02::*;
#[cfg(feature = "admi")] use open_payments_iso20022_admi::admi_011_001_01::*;
#[cfg(feature = "admi")] use open_payments_iso20022_admi::admi_017_001_02::*;
#[cfg(feature = "admi")] use open_payments_iso20022_admi::admi_024_001_01::*;

#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_001_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_002_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_003_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_012_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_013_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_014_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_015_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_016_001_03::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_017_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_018_001_04::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_019_001_04::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_020_001_04::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_021_001_04::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_022_001_04::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_023_001_04::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_024_001_04::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_025_001_04::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_026_001_04::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_027_001_04::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_028_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_029_001_05::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_030_001_04::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_031_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_032_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_033_001_03::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_034_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_035_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_036_001_03::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_038_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_039_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_040_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_041_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_042_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_043_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_044_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_045_001_03::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_047_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_048_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_049_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_050_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_052_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_053_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_054_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_055_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_056_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_057_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_058_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_059_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_060_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_061_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_062_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_063_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_064_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_065_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_066_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_067_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_068_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_069_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_070_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_071_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_072_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_076_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_077_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_078_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_079_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_080_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_083_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_084_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_085_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_086_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_090_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_091_001_03::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_092_001_04::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_094_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_100_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_101_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_102_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_105_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_106_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_107_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_108_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_109_001_02::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_112_001_01::*;
#[cfg(feature = "auth")] use open_payments_iso20022_auth::auth_113_001_01::*;

#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_003_001_08::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_004_001_10::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_005_001_11::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_006_001_11::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_007_001_10::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_008_001_11::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_009_001_08::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_010_001_09::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_011_001_08::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_012_001_08::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_013_001_04::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_014_001_05::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_015_001_04::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_016_001_04::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_017_001_05::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_018_001_05::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_019_001_07::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_020_001_04::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_021_001_06::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_023_001_07::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_024_001_08::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_025_001_08::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_026_001_10::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_027_001_10::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_028_001_12::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_029_001_13::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_030_001_06::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_031_001_07::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_032_001_05::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_033_001_07::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_034_001_07::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_035_001_06::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_036_001_06::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_037_001_10::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_038_001_05::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_039_001_06::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_040_001_04::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_041_001_04::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_042_001_04::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_043_001_04::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_044_001_03::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_045_001_03::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_046_001_08::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_047_001_08::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_048_001_07::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_049_001_07::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_050_001_07::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_051_001_07::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_052_001_12::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_053_001_12::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_054_001_12::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_055_001_12::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_056_001_11::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_057_001_08::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_058_001_09::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_059_001_08::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_060_001_07::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_061_001_02::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_062_001_03::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_063_001_02::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_066_001_02::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_067_001_02::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_068_001_02::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_069_001_05::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_070_001_06::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_071_001_05::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_072_001_02::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_073_001_02::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_074_001_02::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_075_001_02::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_078_001_02::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_079_001_02::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_080_001_02::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_081_001_02::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_082_001_02::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_083_001_02::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_084_001_02::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_085_001_02::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_086_001_05::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_087_001_09::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_088_001_02::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_101_001_02::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_102_001_03::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_103_001_03::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_104_001_01::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_105_001_02::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_106_001_02::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_107_001_02::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_108_001_02::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_109_001_02::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_110_001_01::*;
#[cfg(feature = "camt")] use open_payments_iso20022_camt::camt_111_001_01::*;

#[cfg(feature = "pacs")] use open_payments_iso20022_pacs::pacs_002_001_12::*;
#[cfg(feature = "pacs")] use open_payments_iso20022_pacs::pacs_002_001_14::*;
#[cfg(feature = "pacs")] use open_payments_iso20022_pacs::pacs_003_001_11::*;
#[cfg(feature = "pacs")] use open_payments_iso20022_pacs::pacs_004_001_13::*;
#[cfg(feature = "pacs")] use open_payments_iso20022_pacs::pacs_007_001_13::*;
#[cfg(feature = "pacs")] use open_payments_iso20022_pacs::pacs_008_001_12::*;
#[cfg(feature = "pacs")] use open_payments_iso20022_pacs::pacs_009_001_11::*;
#[cfg(feature = "pacs")] use open_payments_iso20022_pacs::pacs_010_001_06::*;
#[cfg(feature = "pacs")] use open_payments_iso20022_pacs::pacs_028_001_06::*;
#[cfg(feature = "pacs")] use open_payments_iso20022_pacs::pacs_029_001_02::*;

#[cfg(feature = "pain")] use open_payments_iso20022_pain::pain_001_001_12::*;
#[cfg(feature = "pain")] use open_payments_iso20022_pain::pain_002_001_14::*;
#[cfg(feature = "pain")] use open_payments_iso20022_pain::pain_007_001_12::*;
#[cfg(feature = "pain")] use open_payments_iso20022_pain::pain_008_001_11::*;
#[cfg(feature = "pain")] use open_payments_iso20022_pain::pain_009_001_08::*;
#[cfg(feature = "pain")] use open_payments_iso20022_pain::pain_010_001_08::*;
#[cfg(feature = "pain")] use open_payments_iso20022_pain::pain_011_001_08::*;
#[cfg(feature = "pain")] use open_payments_iso20022_pain::pain_012_001_08::*;
#[cfg(feature = "pain")] use open_payments_iso20022_pain::pain_013_001_11::*;
#[cfg(feature = "pain")] use open_payments_iso20022_pain::pain_014_001_11::*;
#[cfg(feature = "pain")] use open_payments_iso20022_pain::pain_017_001_04::*;
#[cfg(feature = "pain")] use open_payments_iso20022_pain::pain_018_001_04::*;

#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_001_001_04::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_002_001_04::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_004_001_07::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_005_001_03::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_006_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_007_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_008_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_009_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_010_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_012_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_013_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_014_001_02::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_015_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_016_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_017_001_02::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_018_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_019_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_020_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_021_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_022_001_02::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_023_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_029_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_030_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_031_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_032_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_033_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_034_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_035_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_036_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_037_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_041_001_02::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_042_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_043_001_02::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_056_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_057_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_058_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_059_001_01::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_060_001_02::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_061_001_02::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_064_001_02::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_065_001_02::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_066_001_02::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_067_001_02::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_068_001_02::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_069_001_02::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_070_001_02::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_071_001_02::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_072_001_02::*;
#[cfg(feature = "reda")] use open_payments_iso20022_reda::reda_073_001_02::*;

#[cfg(feature = "remt")] use open_payments_iso20022_remt::remt_001_001_06::*;
#[cfg(feature = "remt")] use open_payments_iso20022_remt::remt_002_001_03::*;

#[cfg(feature = "derive_samplify")]
use samplify_rs::Sampleable;

#[cfg_attr(feature = "derive_debug", derive(Debug))]
#[cfg_attr(feature = "derive_clone", derive(Clone))]
#[cfg_attr(feature = "derive_partial_eq", derive(PartialEq))]
#[cfg_attr(feature = "derive_default", derive(Default))]
#[cfg_attr(feature = "derive_serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_samplify", derive(Sampleable))]
pub enum Document {
	#[cfg_attr( feature = "derive_serde", serde(rename = "admi.002.001.01"))]
	#[cfg(feature = "admi")]
	Admi00200101(Box<Admi00200101>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SysEvtNtfctn"))]
	#[cfg(feature = "admi")]
	SystemEventNotificationV02(Box<SystemEventNotificationV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "RptQryReq"))]
	#[cfg(feature = "admi")]
	ReportQueryRequestV02(Box<ReportQueryRequestV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "RsndReq"))]
	#[cfg(feature = "admi")]
	ResendRequestV01(Box<ResendRequestV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "RctAck"))]
	#[cfg(feature = "admi")]
	ReceiptAcknowledgementV01(Box<ReceiptAcknowledgementV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "StatcDataReq"))]
	#[cfg(feature = "admi")]
	StaticDataRequestV02(Box<StaticDataRequestV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "StatcDataRpt"))]
	#[cfg(feature = "admi")]
	StaticDataReportV02(Box<StaticDataReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SysEvtAck"))]
	#[cfg(feature = "admi")]
	SystemEventAcknowledgementV01(Box<SystemEventAcknowledgementV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "PrcgReq"))]
	#[cfg(feature = "admi")]
	ProcessingRequestV02(Box<ProcessingRequestV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "NtfctnOfCrspdc"))]
	#[cfg(feature = "admi")]
	NotificationOfCorrespondenceV01(Box<NotificationOfCorrespondenceV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FIToFIPmtStsRpt"))]
	#[cfg(feature = "pacs")]
	FIToFIPaymentStatusReportV12(Box<FIToFIPaymentStatusReportV12>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FIToFIPmtStsRpt"))]
	#[cfg(feature = "pacs")]
	FIToFIPaymentStatusReportV14(Box<FIToFIPaymentStatusReportV14>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FIToFICstmrDrctDbt"))]
	#[cfg(feature = "pacs")]
	FIToFICustomerDirectDebitV11(Box<FIToFICustomerDirectDebitV11>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtRtr"))]
	#[cfg(feature = "pacs")]
	PaymentReturnV13(Box<PaymentReturnV13>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FIToFIPmtRvsl"))]
	#[cfg(feature = "pacs")]
	FIToFIPaymentReversalV13(Box<FIToFIPaymentReversalV13>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FIToFICstmrCdtTrf"))]
	#[cfg(feature = "pacs")]
	FIToFICustomerCreditTransferV12(Box<FIToFICustomerCreditTransferV12>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FICdtTrf"))]
	#[cfg(feature = "pacs")]
	FinancialInstitutionCreditTransferV11(Box<FinancialInstitutionCreditTransferV11>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FIDrctDbt"))]
	#[cfg(feature = "pacs")]
	FinancialInstitutionDirectDebitV06(Box<FinancialInstitutionDirectDebitV06>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FIToFIPmtStsReq"))]
	#[cfg(feature = "pacs")]
	FIToFIPaymentStatusRequestV06(Box<FIToFIPaymentStatusRequestV06>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "MulSttlmReq"))]
	#[cfg(feature = "pacs")]
	MultilateralSettlementRequestV02(Box<MultilateralSettlementRequestV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CstmrCdtTrfInitn"))]
	#[cfg(feature = "pain")]
	CustomerCreditTransferInitiationV12(Box<CustomerCreditTransferInitiationV12>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CstmrPmtStsRpt"))]
	#[cfg(feature = "pain")]
	CustomerPaymentStatusReportV14(Box<CustomerPaymentStatusReportV14>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CstmrPmtRvsl"))]
	#[cfg(feature = "pain")]
	CustomerPaymentReversalV12(Box<CustomerPaymentReversalV12>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CstmrDrctDbtInitn"))]
	#[cfg(feature = "pain")]
	CustomerDirectDebitInitiationV11(Box<CustomerDirectDebitInitiationV11>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtInitnReq"))]
	#[cfg(feature = "pain")]
	MandateInitiationRequestV08(Box<MandateInitiationRequestV08>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtAmdmntReq"))]
	#[cfg(feature = "pain")]
	MandateAmendmentRequestV08(Box<MandateAmendmentRequestV08>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtCxlReq"))]
	#[cfg(feature = "pain")]
	MandateCancellationRequestV08(Box<MandateCancellationRequestV08>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtAccptncRpt"))]
	#[cfg(feature = "pain")]
	MandateAcceptanceReportV08(Box<MandateAcceptanceReportV08>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrPmtActvtnReq"))]
	#[cfg(feature = "pain")]
	CreditorPaymentActivationRequestV11(Box<CreditorPaymentActivationRequestV11>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CdtrPmtActvtnReqStsRpt"))]
	#[cfg(feature = "pain")]
	CreditorPaymentActivationRequestStatusReportV11(Box<CreditorPaymentActivationRequestStatusReportV11>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtCpyReq"))]
	#[cfg(feature = "pain")]
	MandateCopyRequestV04(Box<MandateCopyRequestV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "MndtSspnsnReq"))]
	#[cfg(feature = "pain")]
	MandateSuspensionRequestV04(Box<MandateSuspensionRequestV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctOpngInstr"))]
	#[cfg(feature = "acmt")]
	AccountOpeningInstructionV08(Box<AccountOpeningInstructionV08>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctDtlsConf"))]
	#[cfg(feature = "acmt")]
	AccountDetailsConfirmationV08(Box<AccountDetailsConfirmationV08>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctModInstr"))]
	#[cfg(feature = "acmt")]
	AccountModificationInstructionV08(Box<AccountModificationInstructionV08>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqForAcctMgmtStsRpt"))]
	#[cfg(feature = "acmt")]
	RequestForAccountManagementStatusReportV06(Box<RequestForAccountManagementStatusReportV06>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctMgmtStsRpt"))]
	#[cfg(feature = "acmt")]
	AccountManagementStatusReportV07(Box<AccountManagementStatusReportV07>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctOpngReq"))]
	#[cfg(feature = "acmt")]
	AccountOpeningRequestV05(Box<AccountOpeningRequestV05>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctOpngAmdmntReq"))]
	#[cfg(feature = "acmt")]
	AccountOpeningAmendmentRequestV05(Box<AccountOpeningAmendmentRequestV05>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctOpngAddtlInfReq"))]
	#[cfg(feature = "acmt")]
	AccountOpeningAdditionalInformationRequestV04(Box<AccountOpeningAdditionalInformationRequestV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctReqAck"))]
	#[cfg(feature = "acmt")]
	AccountRequestAcknowledgementV04(Box<AccountRequestAcknowledgementV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctReqRjctn"))]
	#[cfg(feature = "acmt")]
	AccountRequestRejectionV04(Box<AccountRequestRejectionV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctAddtlInfReq"))]
	#[cfg(feature = "acmt")]
	AccountAdditionalInformationRequestV04(Box<AccountAdditionalInformationRequestV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctRptReq"))]
	#[cfg(feature = "acmt")]
	AccountReportRequestV04(Box<AccountReportRequestV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctRpt"))]
	#[cfg(feature = "acmt")]
	AccountReportV05(Box<AccountReportV05>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctExcldMndtMntncReq"))]
	#[cfg(feature = "acmt")]
	AccountExcludedMandateMaintenanceRequestV04(Box<AccountExcludedMandateMaintenanceRequestV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctExcldMndtMntncAmdmntReq"))]
	#[cfg(feature = "acmt")]
	AccountExcludedMandateMaintenanceAmendmentRequestV04(Box<AccountExcludedMandateMaintenanceAmendmentRequestV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctMndtMntncReq"))]
	#[cfg(feature = "acmt")]
	AccountMandateMaintenanceRequestV04(Box<AccountMandateMaintenanceRequestV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctMndtMntncAmdmntReq"))]
	#[cfg(feature = "acmt")]
	AccountMandateMaintenanceAmendmentRequestV04(Box<AccountMandateMaintenanceAmendmentRequestV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctClsgReq"))]
	#[cfg(feature = "acmt")]
	AccountClosingRequestV04(Box<AccountClosingRequestV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctClsgAmdmntReq"))]
	#[cfg(feature = "acmt")]
	AccountClosingAmendmentRequestV04(Box<AccountClosingAmendmentRequestV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctClsgAddtlInfReq"))]
	#[cfg(feature = "acmt")]
	AccountClosingAdditionalInformationRequestV04(Box<AccountClosingAdditionalInformationRequestV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "IdModAdvc"))]
	#[cfg(feature = "acmt")]
	IdentificationModificationAdviceV04(Box<IdentificationModificationAdviceV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "IdVrfctnReq"))]
	#[cfg(feature = "acmt")]
	IdentificationVerificationRequestV04(Box<IdentificationVerificationRequestV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "IdVrfctnRpt"))]
	#[cfg(feature = "acmt")]
	IdentificationVerificationReportV04(Box<IdentificationVerificationReportV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSwtchInfReq"))]
	#[cfg(feature = "acmt")]
	AccountSwitchInformationRequestV05(Box<AccountSwitchInformationRequestV05>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSwtchInfRspn"))]
	#[cfg(feature = "acmt")]
	AccountSwitchInformationResponseV05(Box<AccountSwitchInformationResponseV05>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSwtchCclExstgPmt"))]
	#[cfg(feature = "acmt")]
	AccountSwitchCancelExistingPaymentV05(Box<AccountSwitchCancelExistingPaymentV05>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSwtchReqRdrctn"))]
	#[cfg(feature = "acmt")]
	AccountSwitchRequestRedirectionV04(Box<AccountSwitchRequestRedirectionV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSwtchReqBalTrf"))]
	#[cfg(feature = "acmt")]
	AccountSwitchRequestBalanceTransferV05(Box<AccountSwitchRequestBalanceTransferV05>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSwtchBalTrfAck"))]
	#[cfg(feature = "acmt")]
	AccountSwitchBalanceTransferAcknowledgementV05(Box<AccountSwitchBalanceTransferAcknowledgementV05>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSwtchNtfyAcctSwtchCmplt"))]
	#[cfg(feature = "acmt")]
	AccountSwitchNotifyAccountSwitchCompleteV02(Box<AccountSwitchNotifyAccountSwitchCompleteV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSwtchReqPmt"))]
	#[cfg(feature = "acmt")]
	AccountSwitchRequestPaymentV05(Box<AccountSwitchRequestPaymentV05>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSwtchPmtRspn"))]
	#[cfg(feature = "acmt")]
	AccountSwitchPaymentResponseV02(Box<AccountSwitchPaymentResponseV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSwtchTermntnSwtch"))]
	#[cfg(feature = "acmt")]
	AccountSwitchTerminationSwitchV01(Box<AccountSwitchTerminationSwitchV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctSwtchTechRjctn"))]
	#[cfg(feature = "acmt")]
	AccountSwitchTechnicalRejectionV02(Box<AccountSwitchTechnicalRejectionV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "PricRpt"))]
	#[cfg(feature = "reda")]
	PriceReportV04(Box<PriceReportV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "PricRptCxl"))]
	#[cfg(feature = "reda")]
	PriceReportCancellationV04(Box<PriceReportCancellationV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FndRefDataRpt"))]
	#[cfg(feature = "reda")]
	FundReferenceDataReportV07(Box<FundReferenceDataReportV07>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstmtFndRptReq"))]
	#[cfg(feature = "reda")]
	InvestmentFundReportRequestV03(Box<InvestmentFundReportRequestV03>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctyCreReq"))]
	#[cfg(feature = "reda")]
	SecurityCreationRequestV01(Box<SecurityCreationRequestV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctyMntncReq"))]
	#[cfg(feature = "reda")]
	SecurityMaintenanceRequestV01(Box<SecurityMaintenanceRequestV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctyCreStsAdvc"))]
	#[cfg(feature = "reda")]
	SecurityCreationStatusAdviceV01(Box<SecurityCreationStatusAdviceV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctyActvtyAdvc"))]
	#[cfg(feature = "reda")]
	SecurityActivityAdviceV01(Box<SecurityActivityAdviceV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctyQry"))]
	#[cfg(feature = "reda")]
	SecurityQueryV01(Box<SecurityQueryV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctyRpt"))]
	#[cfg(feature = "reda")]
	SecurityReportV01(Box<SecurityReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctyDeltnReq"))]
	#[cfg(feature = "reda")]
	SecurityDeletionRequestV01(Box<SecurityDeletionRequestV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyCreReq"))]
	#[cfg(feature = "reda")]
	PartyCreationRequestV02(Box<PartyCreationRequestV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyQry"))]
	#[cfg(feature = "reda")]
	PartyQueryV01(Box<PartyQueryV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyStsAdvc"))]
	#[cfg(feature = "reda")]
	PartyStatusAdviceV01(Box<PartyStatusAdviceV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyRpt"))]
	#[cfg(feature = "reda")]
	PartyReportV02(Box<PartyReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesAcctCreReq"))]
	#[cfg(feature = "reda")]
	SecuritiesAccountCreationRequestV01(Box<SecuritiesAccountCreationRequestV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesAcctQry"))]
	#[cfg(feature = "reda")]
	SecuritiesAccountQueryV01(Box<SecuritiesAccountQueryV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesAcctStsAdvc"))]
	#[cfg(feature = "reda")]
	SecuritiesAccountStatusAdviceV01(Box<SecuritiesAccountStatusAdviceV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesAcctRpt"))]
	#[cfg(feature = "reda")]
	SecuritiesAccountReportV01(Box<SecuritiesAccountReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyModReq"))]
	#[cfg(feature = "reda")]
	PartyModificationRequestV02(Box<PartyModificationRequestV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesAcctModReq"))]
	#[cfg(feature = "reda")]
	SecuritiesAccountModificationRequestV01(Box<SecuritiesAccountModificationRequestV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctyMntncStsAdvc"))]
	#[cfg(feature = "reda")]
	SecurityMaintenanceStatusAdviceV01(Box<SecurityMaintenanceStatusAdviceV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctyDeltnStsAdvc"))]
	#[cfg(feature = "reda")]
	SecurityDeletionStatusAdviceV01(Box<SecurityDeletionStatusAdviceV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyDeltnReq"))]
	#[cfg(feature = "reda")]
	PartyDeletionRequestV01(Box<PartyDeletionRequestV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesAcctDeltnReq"))]
	#[cfg(feature = "reda")]
	SecuritiesAccountDeletionRequestV01(Box<SecuritiesAccountDeletionRequestV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesAudtTrlQry"))]
	#[cfg(feature = "reda")]
	SecuritiesAuditTrailQueryV01(Box<SecuritiesAuditTrailQueryV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesAudtTrlRpt"))]
	#[cfg(feature = "reda")]
	SecuritiesAuditTrailReportV01(Box<SecuritiesAuditTrailReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesAcctActvtyAdvc"))]
	#[cfg(feature = "reda")]
	SecuritiesAccountActivityAdviceV01(Box<SecuritiesAccountActivityAdviceV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesAcctAudtTrlQry"))]
	#[cfg(feature = "reda")]
	SecuritiesAccountAuditTrailQueryV01(Box<SecuritiesAccountAuditTrailQueryV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesAcctAudtTrlRpt"))]
	#[cfg(feature = "reda")]
	SecuritiesAccountAuditTrailReportV01(Box<SecuritiesAccountAuditTrailReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyActvtyAdvc"))]
	#[cfg(feature = "reda")]
	PartyActivityAdviceV02(Box<PartyActivityAdviceV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyAudtTrlQry"))]
	#[cfg(feature = "reda")]
	PartyAuditTrailQueryV01(Box<PartyAuditTrailQueryV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "PtyAudtTrlRpt"))]
	#[cfg(feature = "reda")]
	PartyAuditTrailReportV02(Box<PartyAuditTrailReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "StgSttlmInstr"))]
	#[cfg(feature = "reda")]
	StandingSettlementInstructionV01(Box<StandingSettlementInstructionV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "StgSttlmInstrDeltn"))]
	#[cfg(feature = "reda")]
	StandingSettlementInstructionDeletionV01(Box<StandingSettlementInstructionDeletionV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "StgSttlmInstrStsAdvc"))]
	#[cfg(feature = "reda")]
	StandingSettlementInstructionStatusAdviceV01(Box<StandingSettlementInstructionStatusAdviceV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "StgSttlmInstrCxl"))]
	#[cfg(feature = "reda")]
	StandingSettlementInstructionCancellationV01(Box<StandingSettlementInstructionCancellationV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "NetgCutOffRefDataUpdReq"))]
	#[cfg(feature = "reda")]
	NettingCutOffReferenceDataUpdateRequestV02(Box<NettingCutOffReferenceDataUpdateRequestV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "NetgCutOffRefDataRpt"))]
	#[cfg(feature = "reda")]
	NettingCutOffReferenceDataReportV02(Box<NettingCutOffReferenceDataReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CalQry"))]
	#[cfg(feature = "reda")]
	CalendarQueryV02(Box<CalendarQueryV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CalRpt"))]
	#[cfg(feature = "reda")]
	CalendarReportV02(Box<CalendarReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqToPayCdtrEnrlmntReq"))]
	#[cfg(feature = "reda")]
	RequestToPayCreditorEnrolmentRequestV02(Box<RequestToPayCreditorEnrolmentRequestV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqToPayCdtrEnrlmntAmdmntReq"))]
	#[cfg(feature = "reda")]
	RequestToPayCreditorEnrolmentAmendmentRequestV02(Box<RequestToPayCreditorEnrolmentAmendmentRequestV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqToPayCdtrEnrlmntCxlReq"))]
	#[cfg(feature = "reda")]
	RequestToPayCreditorEnrolmentCancellationRequestV02(Box<RequestToPayCreditorEnrolmentCancellationRequestV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqToPayCdtrEnrlmntStsRpt"))]
	#[cfg(feature = "reda")]
	RequestToPayCreditorEnrolmentStatusReportV02(Box<RequestToPayCreditorEnrolmentStatusReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqToPayDbtrActvtnReq"))]
	#[cfg(feature = "reda")]
	RequestToPayDebtorActivationRequestV02(Box<RequestToPayDebtorActivationRequestV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqToPayDbtrActvtnAmdmntReq"))]
	#[cfg(feature = "reda")]
	RequestToPayDebtorActivationAmendmentRequestV02(Box<RequestToPayDebtorActivationAmendmentRequestV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqToPayDbtrActvtnCxlReq"))]
	#[cfg(feature = "reda")]
	RequestToPayDebtorActivationCancellationRequestV02(Box<RequestToPayDebtorActivationCancellationRequestV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqToPayDbtrActvtnStsRpt"))]
	#[cfg(feature = "reda")]
	RequestToPayDebtorActivationStatusReportV02(Box<RequestToPayDebtorActivationStatusReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtAdvc"))]
	#[cfg(feature = "remt")]
	RemittanceAdviceV06(Box<RemittanceAdviceV06>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "RmtLctnAdvc"))]
	#[cfg(feature = "remt")]
	RemittanceLocationAdviceV03(Box<RemittanceLocationAdviceV03>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "GetAcct"))]
	#[cfg(feature = "camt")]
	GetAccountV08(Box<GetAccountV08>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "RtrAcct"))]
	#[cfg(feature = "camt")]
	ReturnAccountV10(Box<ReturnAccountV10>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "GetTx"))]
	#[cfg(feature = "camt")]
	GetTransactionV11(Box<GetTransactionV11>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "RtrTx"))]
	#[cfg(feature = "camt")]
	ReturnTransactionV11(Box<ReturnTransactionV11>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "ModfyTx"))]
	#[cfg(feature = "camt")]
	ModifyTransactionV10(Box<ModifyTransactionV10>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CclTx"))]
	#[cfg(feature = "camt")]
	CancelTransactionV11(Box<CancelTransactionV11>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "GetLmt"))]
	#[cfg(feature = "camt")]
	GetLimitV08(Box<GetLimitV08>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "RtrLmt"))]
	#[cfg(feature = "camt")]
	ReturnLimitV09(Box<ReturnLimitV09>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "ModfyLmt"))]
	#[cfg(feature = "camt")]
	ModifyLimitV08(Box<ModifyLimitV08>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "DelLmt"))]
	#[cfg(feature = "camt")]
	DeleteLimitV08(Box<DeleteLimitV08>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "GetMmb"))]
	#[cfg(feature = "camt")]
	GetMemberV04(Box<GetMemberV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "RtrMmb"))]
	#[cfg(feature = "camt")]
	ReturnMemberV05(Box<ReturnMemberV05>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "ModfyMmb"))]
	#[cfg(feature = "camt")]
	ModifyMemberV04(Box<ModifyMemberV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "GetCcyXchgRate"))]
	#[cfg(feature = "camt")]
	GetCurrencyExchangeRateV04(Box<GetCurrencyExchangeRateV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "RtrCcyXchgRate"))]
	#[cfg(feature = "camt")]
	ReturnCurrencyExchangeRateV05(Box<ReturnCurrencyExchangeRateV05>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "GetBizDayInf"))]
	#[cfg(feature = "camt")]
	GetBusinessDayInformationV05(Box<GetBusinessDayInformationV05>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "RtrBizDayInf"))]
	#[cfg(feature = "camt")]
	ReturnBusinessDayInformationV07(Box<ReturnBusinessDayInformationV07>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "GetGnlBizInf"))]
	#[cfg(feature = "camt")]
	GetGeneralBusinessInformationV04(Box<GetGeneralBusinessInformationV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "RtrGnlBizInf"))]
	#[cfg(feature = "camt")]
	ReturnGeneralBusinessInformationV06(Box<ReturnGeneralBusinessInformationV06>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "BckpPmt"))]
	#[cfg(feature = "camt")]
	BackupPaymentV07(Box<BackupPaymentV07>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "ModfyStgOrdr"))]
	#[cfg(feature = "camt")]
	ModifyStandingOrderV08(Box<ModifyStandingOrderV08>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "Rct"))]
	#[cfg(feature = "camt")]
	ReceiptV08(Box<ReceiptV08>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "UblToApply"))]
	#[cfg(feature = "camt")]
	UnableToApplyV10(Box<UnableToApplyV10>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "ClmNonRct"))]
	#[cfg(feature = "camt")]
	ClaimNonReceiptV10(Box<ClaimNonReceiptV10>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AddtlPmtInf"))]
	#[cfg(feature = "camt")]
	AdditionalPaymentInformationV12(Box<AdditionalPaymentInformationV12>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "RsltnOfInvstgtn"))]
	#[cfg(feature = "camt")]
	ResolutionOfInvestigationV13(Box<ResolutionOfInvestigationV13>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "NtfctnOfCaseAssgnmt"))]
	#[cfg(feature = "camt")]
	NotificationOfCaseAssignmentV06(Box<NotificationOfCaseAssignmentV06>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "RjctInvstgtn"))]
	#[cfg(feature = "camt")]
	RejectInvestigationV07(Box<RejectInvestigationV07>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CclCaseAssgnmt"))]
	#[cfg(feature = "camt")]
	CancelCaseAssignmentV05(Box<CancelCaseAssignmentV05>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqForDplct"))]
	#[cfg(feature = "camt")]
	RequestForDuplicateV07(Box<RequestForDuplicateV07>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "Dplct"))]
	#[cfg(feature = "camt")]
	DuplicateV07(Box<DuplicateV07>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "PrtryFrmtInvstgtn"))]
	#[cfg(feature = "camt")]
	ProprietaryFormatInvestigationV06(Box<ProprietaryFormatInvestigationV06>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtAuthstnRspn"))]
	#[cfg(feature = "camt")]
	DebitAuthorisationResponseV06(Box<DebitAuthorisationResponseV06>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "DbtAuthstnReq"))]
	#[cfg(feature = "camt")]
	DebitAuthorisationRequestV10(Box<DebitAuthorisationRequestV10>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CaseStsRptReq"))]
	#[cfg(feature = "camt")]
	CaseStatusReportRequestV05(Box<CaseStatusReportRequestV05>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CaseStsRpt"))]
	#[cfg(feature = "camt")]
	CaseStatusReportV06(Box<CaseStatusReportV06>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FndEstmtdCshFcstRpt"))]
	#[cfg(feature = "camt")]
	FundEstimatedCashForecastReportV04(Box<FundEstimatedCashForecastReportV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FndConfdCshFcstRpt"))]
	#[cfg(feature = "camt")]
	FundConfirmedCashForecastReportV04(Box<FundConfirmedCashForecastReportV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FndDtldEstmtdCshFcstRpt"))]
	#[cfg(feature = "camt")]
	FundDetailedEstimatedCashForecastReportV04(Box<FundDetailedEstimatedCashForecastReportV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FndDtldConfdCshFcstRpt"))]
	#[cfg(feature = "camt")]
	FundDetailedConfirmedCashForecastReportV04(Box<FundDetailedConfirmedCashForecastReportV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FndConfdCshFcstRptCxl"))]
	#[cfg(feature = "camt")]
	FundConfirmedCashForecastReportCancellationV03(Box<FundConfirmedCashForecastReportCancellationV03>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FndDtldConfdCshFcstRptCxl"))]
	#[cfg(feature = "camt")]
	FundDetailedConfirmedCashForecastReportCancellationV03(Box<FundDetailedConfirmedCashForecastReportCancellationV03>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "GetRsvatn"))]
	#[cfg(feature = "camt")]
	GetReservationV08(Box<GetReservationV08>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "RtrRsvatn"))]
	#[cfg(feature = "camt")]
	ReturnReservationV08(Box<ReturnReservationV08>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "ModfyRsvatn"))]
	#[cfg(feature = "camt")]
	ModifyReservationV07(Box<ModifyReservationV07>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "DelRsvatn"))]
	#[cfg(feature = "camt")]
	DeleteReservationV07(Box<DeleteReservationV07>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "LqdtyCdtTrf"))]
	#[cfg(feature = "camt")]
	LiquidityCreditTransferV07(Box<LiquidityCreditTransferV07>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "LqdtyDbtTrf"))]
	#[cfg(feature = "camt")]
	LiquidityDebitTransferV07(Box<LiquidityDebitTransferV07>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "BkToCstmrAcctRpt"))]
	#[cfg(feature = "camt")]
	BankToCustomerAccountReportV12(Box<BankToCustomerAccountReportV12>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "BkToCstmrStmt"))]
	#[cfg(feature = "camt")]
	BankToCustomerStatementV12(Box<BankToCustomerStatementV12>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "BkToCstmrDbtCdtNtfctn"))]
	#[cfg(feature = "camt")]
	BankToCustomerDebitCreditNotificationV12(Box<BankToCustomerDebitCreditNotificationV12>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CstmrPmtCxlReq"))]
	#[cfg(feature = "camt")]
	CustomerPaymentCancellationRequestV12(Box<CustomerPaymentCancellationRequestV12>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FIToFIPmtCxlReq"))]
	#[cfg(feature = "camt")]
	FIToFIPaymentCancellationRequestV11(Box<FIToFIPaymentCancellationRequestV11>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "NtfctnToRcv"))]
	#[cfg(feature = "camt")]
	NotificationToReceiveV08(Box<NotificationToReceiveV08>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "NtfctnToRcvCxlAdvc"))]
	#[cfg(feature = "camt")]
	NotificationToReceiveCancellationAdviceV09(Box<NotificationToReceiveCancellationAdviceV09>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "NtfctnToRcvStsRpt"))]
	#[cfg(feature = "camt")]
	NotificationToReceiveStatusReportV08(Box<NotificationToReceiveStatusReportV08>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "AcctRptgReq"))]
	#[cfg(feature = "camt")]
	AccountReportingRequestV07(Box<AccountReportingRequestV07>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "PayInCall"))]
	#[cfg(feature = "camt")]
	PayInCallV02(Box<PayInCallV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "PayInSchdl"))]
	#[cfg(feature = "camt")]
	PayInScheduleV03(Box<PayInScheduleV03>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "PayInEvtAck"))]
	#[cfg(feature = "camt")]
	PayInEventAcknowledgementV02(Box<PayInEventAcknowledgementV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "IntraBalMvmntInstr"))]
	#[cfg(feature = "camt")]
	IntraBalanceMovementInstructionV02(Box<IntraBalanceMovementInstructionV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "IntraBalMvmntStsAdvc"))]
	#[cfg(feature = "camt")]
	IntraBalanceMovementStatusAdviceV02(Box<IntraBalanceMovementStatusAdviceV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "IntraBalMvmntConf"))]
	#[cfg(feature = "camt")]
	IntraBalanceMovementConfirmationV02(Box<IntraBalanceMovementConfirmationV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "GetStgOrdr"))]
	#[cfg(feature = "camt")]
	GetStandingOrderV05(Box<GetStandingOrderV05>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "RtrStgOrdr"))]
	#[cfg(feature = "camt")]
	ReturnStandingOrderV06(Box<ReturnStandingOrderV06>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "DelStgOrdr"))]
	#[cfg(feature = "camt")]
	DeleteStandingOrderV05(Box<DeleteStandingOrderV05>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "IntraBalMvmntModReq"))]
	#[cfg(feature = "camt")]
	IntraBalanceMovementModificationRequestV02(Box<IntraBalanceMovementModificationRequestV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "IntraBalMvmntModReqStsAdvc"))]
	#[cfg(feature = "camt")]
	IntraBalanceMovementModificationRequestStatusAdviceV02(Box<IntraBalanceMovementModificationRequestStatusAdviceV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "IntraBalMvmntCxlReq"))]
	#[cfg(feature = "camt")]
	IntraBalanceMovementCancellationRequestV02(Box<IntraBalanceMovementCancellationRequestV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "IntraBalMvmntCxlReqStsAdvc"))]
	#[cfg(feature = "camt")]
	IntraBalanceMovementCancellationRequestStatusAdviceV02(Box<IntraBalanceMovementCancellationRequestStatusAdviceV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "IntraBalMvmntQry"))]
	#[cfg(feature = "camt")]
	IntraBalanceMovementQueryV02(Box<IntraBalanceMovementQueryV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "IntraBalMvmntQryRspn"))]
	#[cfg(feature = "camt")]
	IntraBalanceMovementQueryResponseV02(Box<IntraBalanceMovementQueryResponseV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "IntraBalMvmntModQry"))]
	#[cfg(feature = "camt")]
	IntraBalanceMovementModificationQueryV02(Box<IntraBalanceMovementModificationQueryV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "IntraBalMvmntModRpt"))]
	#[cfg(feature = "camt")]
	IntraBalanceMovementModificationReportV02(Box<IntraBalanceMovementModificationReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "IntraBalMvmntCxlQry"))]
	#[cfg(feature = "camt")]
	IntraBalanceMovementCancellationQueryV02(Box<IntraBalanceMovementCancellationQueryV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "IntraBalMvmntCxlRpt"))]
	#[cfg(feature = "camt")]
	IntraBalanceMovementCancellationReportV02(Box<IntraBalanceMovementCancellationReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "IntraBalMvmntPstngRpt"))]
	#[cfg(feature = "camt")]
	IntraBalanceMovementPostingReportV02(Box<IntraBalanceMovementPostingReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "IntraBalMvmntPdgRpt"))]
	#[cfg(feature = "camt")]
	IntraBalanceMovementPendingReportV02(Box<IntraBalanceMovementPendingReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "BkSvcsBllgStmt"))]
	#[cfg(feature = "camt")]
	BankServicesBillingStatementV05(Box<BankServicesBillingStatementV05>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "ReqToModfyPmt"))]
	#[cfg(feature = "camt")]
	RequestToModifyPaymentV09(Box<RequestToModifyPaymentV09>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "NetRpt"))]
	#[cfg(feature = "camt")]
	NetReportV02(Box<NetReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CretLmt"))]
	#[cfg(feature = "camt")]
	CreateLimitV02(Box<CreateLimitV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CretStgOrdr"))]
	#[cfg(feature = "camt")]
	CreateStandingOrderV03(Box<CreateStandingOrderV03>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CretRsvatn"))]
	#[cfg(feature = "camt")]
	CreateReservationV03(Box<CreateReservationV03>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CretMmb"))]
	#[cfg(feature = "camt")]
	CreateMemberV01(Box<CreateMemberV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgsPmtNtfctn"))]
	#[cfg(feature = "camt")]
	ChargesPaymentNotificationV02(Box<ChargesPaymentNotificationV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "ChrgsPmtReq"))]
	#[cfg(feature = "camt")]
	ChargesPaymentRequestV02(Box<ChargesPaymentRequestV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "ChqPresntmntNtfctn"))]
	#[cfg(feature = "camt")]
	ChequePresentmentNotificationV02(Box<ChequePresentmentNotificationV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "ChqCxlOrStopReq"))]
	#[cfg(feature = "camt")]
	ChequeCancellationOrStopRequestV02(Box<ChequeCancellationOrStopRequestV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "ChqCxlOrStopRpt"))]
	#[cfg(feature = "camt")]
	ChequeCancellationOrStopReportV02(Box<ChequeCancellationOrStopReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstgtnReq"))]
	#[cfg(feature = "camt")]
	InvestigationRequestV01(Box<InvestigationRequestV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "InvstgtnRspn"))]
	#[cfg(feature = "camt")]
	InvestigationResponseV01(Box<InvestigationResponseV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "InfReqOpng"))]
	#[cfg(feature = "auth")]
	InformationRequestOpeningV02(Box<InformationRequestOpeningV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "InfReqRspn"))]
	#[cfg(feature = "auth")]
	InformationRequestResponseV02(Box<InformationRequestResponseV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "InfReqStsChngNtfctn"))]
	#[cfg(feature = "auth")]
	InformationRequestStatusChangeNotificationV01(Box<InformationRequestStatusChangeNotificationV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "MnyMktScrdMktSttstclRpt"))]
	#[cfg(feature = "auth")]
	MoneyMarketSecuredMarketStatisticalReportV02(Box<MoneyMarketSecuredMarketStatisticalReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "MnyMktUscrdMktSttstclRpt"))]
	#[cfg(feature = "auth")]
	MoneyMarketUnsecuredMarketStatisticalReportV02(Box<MoneyMarketUnsecuredMarketStatisticalReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "MnyMktFXSwpsSttstclRpt"))]
	#[cfg(feature = "auth")]
	MoneyMarketForeignExchangeSwapsStatisticalReportV02(Box<MoneyMarketForeignExchangeSwapsStatisticalReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "MnyMktOvrnghtIndxSwpsSttstclRpt"))]
	#[cfg(feature = "auth")]
	MoneyMarketOvernightIndexSwapsStatisticalReportV02(Box<MoneyMarketOvernightIndexSwapsStatisticalReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmRptgTxRpt"))]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingTransactionReportV03(Box<FinancialInstrumentReportingTransactionReportV03>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmRptgRefDataRpt"))]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingReferenceDataReportV02(Box<FinancialInstrumentReportingReferenceDataReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctRegnReq"))]
	#[cfg(feature = "auth")]
	ContractRegistrationRequestV04(Box<ContractRegistrationRequestV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctRegnConf"))]
	#[cfg(feature = "auth")]
	ContractRegistrationConfirmationV04(Box<ContractRegistrationConfirmationV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctRegnClsrReq"))]
	#[cfg(feature = "auth")]
	ContractRegistrationClosureRequestV04(Box<ContractRegistrationClosureRequestV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctRegnAmdmntReq"))]
	#[cfg(feature = "auth")]
	ContractRegistrationAmendmentRequestV04(Box<ContractRegistrationAmendmentRequestV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctRegnStmt"))]
	#[cfg(feature = "auth")]
	ContractRegistrationStatementV04(Box<ContractRegistrationStatementV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CtrctRegnStmtReq"))]
	#[cfg(feature = "auth")]
	ContractRegistrationStatementRequestV04(Box<ContractRegistrationStatementRequestV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "PmtRgltryInfNtfctn"))]
	#[cfg(feature = "auth")]
	PaymentRegulatoryInformationNotificationV04(Box<PaymentRegulatoryInformationNotificationV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CcyCtrlSpprtgDocDlvry"))]
	#[cfg(feature = "auth")]
	CurrencyControlSupportingDocumentDeliveryV04(Box<CurrencyControlSupportingDocumentDeliveryV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CcyCtrlReqOrLttr"))]
	#[cfg(feature = "auth")]
	CurrencyControlRequestOrLetterV04(Box<CurrencyControlRequestOrLetterV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CcyCtrlStsAdvc"))]
	#[cfg(feature = "auth")]
	CurrencyControlStatusAdviceV04(Box<CurrencyControlStatusAdviceV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "MnyMktSttstclRptStsAdvc"))]
	#[cfg(feature = "auth")]
	MoneyMarketStatisticalReportStatusAdviceV01(Box<MoneyMarketStatisticalReportStatusAdviceV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "DerivsTradRptQry"))]
	#[cfg(feature = "auth")]
	DerivativesTradeReportQueryV05(Box<DerivativesTradeReportQueryV05>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "DerivsTradRpt"))]
	#[cfg(feature = "auth")]
	DerivativesTradeReportV04(Box<DerivativesTradeReportV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmRptgStsAdvc"))]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingStatusAdviceV01(Box<FinancialInstrumentReportingStatusAdviceV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmRptgEqtyTrnsprncyDataRpt"))]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingEquityTransparencyDataReportV01(Box<FinancialInstrumentReportingEquityTransparencyDataReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmRptgNonEqtyTrnsprncyDataRpt"))]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingNonEquityTransparencyDataReportV03(Box<FinancialInstrumentReportingNonEquityTransparencyDataReportV03>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "InvcTaxRpt"))]
	#[cfg(feature = "auth")]
	InvoiceTaxReportV01(Box<InvoiceTaxReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmRptgTradgVolCapDataRpt"))]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingTradingVolumeCapDataReportV01(Box<FinancialInstrumentReportingTradingVolumeCapDataReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmRptgRefDataDltaRpt"))]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingReferenceDataDeltaReportV03(Box<FinancialInstrumentReportingReferenceDataDeltaReportV03>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "InvcTaxRptStsAdvc"))]
	#[cfg(feature = "auth")]
	InvoiceTaxReportStatusAdviceV01(Box<InvoiceTaxReportStatusAdviceV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmRptgNonWorkgDayRpt"))]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingNonWorkingDayReportV01(Box<FinancialInstrumentReportingNonWorkingDayReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmRptgEqtyTradgActvtyRpt"))]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingEquityTradingActivityReportV01(Box<FinancialInstrumentReportingEquityTradingActivityReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmRptgNonEqtyTradgActvtyRpt"))]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingNonEquityTradingActivityReportV01(Box<FinancialInstrumentReportingNonEquityTradingActivityReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmRptgInvldRefDataRpt"))]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingInvalidReferenceDataReportV02(Box<FinancialInstrumentReportingInvalidReferenceDataReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmRptgRefDataIndxRpt"))]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingReferenceDataIndexReportV01(Box<FinancialInstrumentReportingReferenceDataIndexReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmRptgEqtyTradgActvtyRslt"))]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingEquityTradingActivityResultV02(Box<FinancialInstrumentReportingEquityTradingActivityResultV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmRptgNonEqtyTradgActvtyRslt"))]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingNonEquityTradingActivityResultV03(Box<FinancialInstrumentReportingNonEquityTradingActivityResultV03>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmRptgCtryCdRpt"))]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingCountryCodeReportV01(Box<FinancialInstrumentReportingCountryCodeReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmRptgCcyCdRpt"))]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingCurrencyCodeReportV01(Box<FinancialInstrumentReportingCurrencyCodeReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmRptgMktIdCdRpt"))]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingMarketIdentificationCodeReportV02(Box<FinancialInstrumentReportingMarketIdentificationCodeReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmRptgInstrmClssfctnRpt"))]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingInstrumentClassificationReportV01(Box<FinancialInstrumentReportingInstrumentClassificationReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesFincgRptgTxRpt"))]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingTransactionReportV02(Box<SecuritiesFinancingReportingTransactionReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmRptgTradgVolCapRsltRpt"))]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingTradingVolumeCapResultReportV01(Box<FinancialInstrumentReportingTradingVolumeCapResultReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CCPClrMmbRpt"))]
	#[cfg(feature = "auth")]
	CCPClearingMemberReportV01(Box<CCPClearingMemberReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CCPMmbRqrmntsRpt"))]
	#[cfg(feature = "auth")]
	CCPMemberRequirementsReportV01(Box<CCPMemberRequirementsReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CCPMmbOblgtnsRpt"))]
	#[cfg(feature = "auth")]
	CCPMemberObligationsReportV01(Box<CCPMemberObligationsReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CCPPrtflStrssTstgDefRpt"))]
	#[cfg(feature = "auth")]
	CCPPortfolioStressTestingDefinitionReportV02(Box<CCPPortfolioStressTestingDefinitionReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CCPPrtflStrssTstgRsltRpt"))]
	#[cfg(feature = "auth")]
	CCPPortfolioStressTestingResultReportV01(Box<CCPPortfolioStressTestingResultReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CCPIncmStmtAndCptlAdqcyRpt"))]
	#[cfg(feature = "auth")]
	CCPIncomeStatementAndCapitalAdequacyReportV01(Box<CCPIncomeStatementAndCapitalAdequacyReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CCPDalyCshFlowsRpt"))]
	#[cfg(feature = "auth")]
	CCPDailyCashFlowsReportV02(Box<CCPDailyCashFlowsReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CCPInvstmtsRpt"))]
	#[cfg(feature = "auth")]
	CCPInvestmentsReportV01(Box<CCPInvestmentsReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CCPLqdtyStrssTstgDefRpt"))]
	#[cfg(feature = "auth")]
	CCPLiquidityStressTestingDefinitionReportV01(Box<CCPLiquidityStressTestingDefinitionReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CCPLqdtyStrssTstgRsltRpt"))]
	#[cfg(feature = "auth")]
	CCPLiquidityStressTestingResultReportV01(Box<CCPLiquidityStressTestingResultReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CCPAvlblFinRsrcsRpt"))]
	#[cfg(feature = "auth")]
	CCPAvailableFinancialResourcesReportV01(Box<CCPAvailableFinancialResourcesReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CCPBckTstgDefRpt"))]
	#[cfg(feature = "auth")]
	CCPBackTestingDefinitionReportV01(Box<CCPBackTestingDefinitionReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CCPBckTstgRsltRpt"))]
	#[cfg(feature = "auth")]
	CCPBackTestingResultReportV01(Box<CCPBackTestingResultReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CCPCollRpt"))]
	#[cfg(feature = "auth")]
	CCPCollateralReportV01(Box<CCPCollateralReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CCPAcctPosRpt"))]
	#[cfg(feature = "auth")]
	CCPAccountPositionReportV01(Box<CCPAccountPositionReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CCPClrdPdctRpt"))]
	#[cfg(feature = "auth")]
	CCPClearedProductReportV01(Box<CCPClearedProductReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesFincgRptgTxMrgnDataRpt"))]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingTransactionMarginDataReportV02(Box<SecuritiesFinancingReportingTransactionMarginDataReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesFincgRptgTxReusdCollDataRpt"))]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingTransactionReusedCollateralDataReportV02(Box<SecuritiesFinancingReportingTransactionReusedCollateralDataReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmIntlrRpt"))]
	#[cfg(feature = "auth")]
	SettlementInternaliserReportV01(Box<SettlementInternaliserReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FinSprvsdPtyIdntyRpt"))]
	#[cfg(feature = "auth")]
	FinancialSupervisedPartyIdentityReportV01(Box<FinancialSupervisedPartyIdentityReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FinBchmkRpt"))]
	#[cfg(feature = "auth")]
	FinancialBenchmarkReportV01(Box<FinancialBenchmarkReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesFincgRptgPairgReq"))]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingPairingRequestV02(Box<SecuritiesFinancingReportingPairingRequestV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesFincgRptgTxStatRpt"))]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingTransactionStateReportV02(Box<SecuritiesFinancingReportingTransactionStateReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesFincgRptgRcncltnStsAdvc"))]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingReconciliationStatusAdviceV02(Box<SecuritiesFinancingReportingReconciliationStatusAdviceV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesFincgRptgMssngCollReq"))]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingMissingCollateralRequestV02(Box<SecuritiesFinancingReportingMissingCollateralRequestV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesFincgRptgTxStsAdvc"))]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingTransactionStatusAdviceV02(Box<SecuritiesFinancingReportingTransactionStatusAdviceV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesFincgRptgMrgnDataTxStatRpt"))]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingMarginDataTransactionStateReportV02(Box<SecuritiesFinancingReportingMarginDataTransactionStateReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesFincgRptgReusdCollDataTxStatRpt"))]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingReusedCollateralDataTransactionStateReportV02(Box<SecuritiesFinancingReportingReusedCollateralDataTransactionStateReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "DerivsTradPosSetRpt"))]
	#[cfg(feature = "auth")]
	DerivativesTradePositionSetReportV02(Box<DerivativesTradePositionSetReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "DerivsTradRcncltnSttstclRpt"))]
	#[cfg(feature = "auth")]
	DerivativesTradeReconciliationStatisticalReportV03(Box<DerivativesTradeReconciliationStatisticalReportV03>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "DerivsTradRjctnSttstclRpt"))]
	#[cfg(feature = "auth")]
	DerivativesTradeRejectionStatisticalReportV04(Box<DerivativesTradeRejectionStatisticalReportV04>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesFincgRptgTxQry"))]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingTransactionQueryV02(Box<SecuritiesFinancingReportingTransactionQueryV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmFlsMnthlyRpt"))]
	#[cfg(feature = "auth")]
	SettlementFailsMonthlyReportV01(Box<SettlementFailsMonthlyReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SttlmFlsAnlRpt"))]
	#[cfg(feature = "auth")]
	SettlementFailsAnnualReportV01(Box<SettlementFailsAnnualReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "FinInstrmRptgCxlRpt"))]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingCancellationReportV01(Box<FinancialInstrumentReportingCancellationReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "SctiesFincgRptgPosSetRpt"))]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingPositionSetReportV01(Box<SecuritiesFinancingReportingPositionSetReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "DerivsTradWrnngsRpt"))]
	#[cfg(feature = "auth")]
	DerivativesTradeWarningsReportV01(Box<DerivativesTradeWarningsReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "DerivsTradStatRpt"))]
	#[cfg(feature = "auth")]
	DerivativesTradeStateReportV02(Box<DerivativesTradeStateReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "DerivsTradMrgnDataRpt"))]
	#[cfg(feature = "auth")]
	DerivativesTradeMarginDataReportV02(Box<DerivativesTradeMarginDataReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "DerivsTradMrgnDataTxStatRpt"))]
	#[cfg(feature = "auth")]
	DerivativesTradeMarginDataTransactionStateReportV02(Box<DerivativesTradeMarginDataTransactionStateReportV02>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "CCPIntrprbltyRpt"))]
	#[cfg(feature = "auth")]
	CCPInteroperabilityReportV01(Box<CCPInteroperabilityReportV01>),

	#[cfg_attr( feature = "derive_serde", serde(rename = "OrdrBookRpt"))]
	#[cfg(feature = "auth")]
	OrderBookReportV01(Box<OrderBookReportV01>),

	#[cfg_attr(feature = "derive_default", default)]
	UNKNOWN,
}

impl Document {
    pub fn validate(&self) -> Result<(), ValidationError> {
        match self {
			#[cfg(feature = "admi")]
			Document::Admi00200101(ref value) => value.validate(),
			#[cfg(feature = "admi")]
			Document::SystemEventNotificationV02(ref value) => value.validate(),
			#[cfg(feature = "admi")]
			Document::ReportQueryRequestV02(ref value) => value.validate(),
			#[cfg(feature = "admi")]
			Document::ResendRequestV01(ref value) => value.validate(),
			#[cfg(feature = "admi")]
			Document::ReceiptAcknowledgementV01(ref value) => value.validate(),
			#[cfg(feature = "admi")]
			Document::StaticDataRequestV02(ref value) => value.validate(),
			#[cfg(feature = "admi")]
			Document::StaticDataReportV02(ref value) => value.validate(),
			#[cfg(feature = "admi")]
			Document::SystemEventAcknowledgementV01(ref value) => value.validate(),
			#[cfg(feature = "admi")]
			Document::ProcessingRequestV02(ref value) => value.validate(),
			#[cfg(feature = "admi")]
			Document::NotificationOfCorrespondenceV01(ref value) => value.validate(),
			#[cfg(feature = "pacs")]
			Document::FIToFIPaymentStatusReportV12(ref value) => value.validate(),
			#[cfg(feature = "pacs")]
			Document::FIToFIPaymentStatusReportV14(ref value) => value.validate(),
			#[cfg(feature = "pacs")]
			Document::FIToFICustomerDirectDebitV11(ref value) => value.validate(),
			#[cfg(feature = "pacs")]
			Document::PaymentReturnV13(ref value) => value.validate(),
			#[cfg(feature = "pacs")]
			Document::FIToFIPaymentReversalV13(ref value) => value.validate(),
			#[cfg(feature = "pacs")]
			Document::FIToFICustomerCreditTransferV12(ref value) => value.validate(),
			#[cfg(feature = "pacs")]
			Document::FinancialInstitutionCreditTransferV11(ref value) => value.validate(),
			#[cfg(feature = "pacs")]
			Document::FinancialInstitutionDirectDebitV06(ref value) => value.validate(),
			#[cfg(feature = "pacs")]
			Document::FIToFIPaymentStatusRequestV06(ref value) => value.validate(),
			#[cfg(feature = "pacs")]
			Document::MultilateralSettlementRequestV02(ref value) => value.validate(),
			#[cfg(feature = "pain")]
			Document::CustomerCreditTransferInitiationV12(ref value) => value.validate(),
			#[cfg(feature = "pain")]
			Document::CustomerPaymentStatusReportV14(ref value) => value.validate(),
			#[cfg(feature = "pain")]
			Document::CustomerPaymentReversalV12(ref value) => value.validate(),
			#[cfg(feature = "pain")]
			Document::CustomerDirectDebitInitiationV11(ref value) => value.validate(),
			#[cfg(feature = "pain")]
			Document::MandateInitiationRequestV08(ref value) => value.validate(),
			#[cfg(feature = "pain")]
			Document::MandateAmendmentRequestV08(ref value) => value.validate(),
			#[cfg(feature = "pain")]
			Document::MandateCancellationRequestV08(ref value) => value.validate(),
			#[cfg(feature = "pain")]
			Document::MandateAcceptanceReportV08(ref value) => value.validate(),
			#[cfg(feature = "pain")]
			Document::CreditorPaymentActivationRequestV11(ref value) => value.validate(),
			#[cfg(feature = "pain")]
			Document::CreditorPaymentActivationRequestStatusReportV11(ref value) => value.validate(),
			#[cfg(feature = "pain")]
			Document::MandateCopyRequestV04(ref value) => value.validate(),
			#[cfg(feature = "pain")]
			Document::MandateSuspensionRequestV04(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountOpeningInstructionV08(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountDetailsConfirmationV08(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountModificationInstructionV08(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::RequestForAccountManagementStatusReportV06(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountManagementStatusReportV07(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountOpeningRequestV05(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountOpeningAmendmentRequestV05(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountOpeningAdditionalInformationRequestV04(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountRequestAcknowledgementV04(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountRequestRejectionV04(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountAdditionalInformationRequestV04(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountReportRequestV04(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountReportV05(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountExcludedMandateMaintenanceRequestV04(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountExcludedMandateMaintenanceAmendmentRequestV04(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountMandateMaintenanceRequestV04(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountMandateMaintenanceAmendmentRequestV04(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountClosingRequestV04(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountClosingAmendmentRequestV04(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountClosingAdditionalInformationRequestV04(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::IdentificationModificationAdviceV04(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::IdentificationVerificationRequestV04(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::IdentificationVerificationReportV04(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountSwitchInformationRequestV05(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountSwitchInformationResponseV05(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountSwitchCancelExistingPaymentV05(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountSwitchRequestRedirectionV04(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountSwitchRequestBalanceTransferV05(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountSwitchBalanceTransferAcknowledgementV05(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountSwitchNotifyAccountSwitchCompleteV02(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountSwitchRequestPaymentV05(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountSwitchPaymentResponseV02(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountSwitchTerminationSwitchV01(ref value) => value.validate(),
			#[cfg(feature = "acmt")]
			Document::AccountSwitchTechnicalRejectionV02(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::PriceReportV04(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::PriceReportCancellationV04(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::FundReferenceDataReportV07(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::InvestmentFundReportRequestV03(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::SecurityCreationRequestV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::SecurityMaintenanceRequestV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::SecurityCreationStatusAdviceV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::SecurityActivityAdviceV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::SecurityQueryV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::SecurityReportV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::SecurityDeletionRequestV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::PartyCreationRequestV02(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::PartyQueryV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::PartyStatusAdviceV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::PartyReportV02(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::SecuritiesAccountCreationRequestV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::SecuritiesAccountQueryV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::SecuritiesAccountStatusAdviceV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::SecuritiesAccountReportV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::PartyModificationRequestV02(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::SecuritiesAccountModificationRequestV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::SecurityMaintenanceStatusAdviceV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::SecurityDeletionStatusAdviceV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::PartyDeletionRequestV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::SecuritiesAccountDeletionRequestV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::SecuritiesAuditTrailQueryV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::SecuritiesAuditTrailReportV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::SecuritiesAccountActivityAdviceV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::SecuritiesAccountAuditTrailQueryV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::SecuritiesAccountAuditTrailReportV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::PartyActivityAdviceV02(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::PartyAuditTrailQueryV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::PartyAuditTrailReportV02(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::StandingSettlementInstructionV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::StandingSettlementInstructionDeletionV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::StandingSettlementInstructionStatusAdviceV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::StandingSettlementInstructionCancellationV01(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::NettingCutOffReferenceDataUpdateRequestV02(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::NettingCutOffReferenceDataReportV02(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::CalendarQueryV02(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::CalendarReportV02(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::RequestToPayCreditorEnrolmentRequestV02(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::RequestToPayCreditorEnrolmentAmendmentRequestV02(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::RequestToPayCreditorEnrolmentCancellationRequestV02(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::RequestToPayCreditorEnrolmentStatusReportV02(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::RequestToPayDebtorActivationRequestV02(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::RequestToPayDebtorActivationAmendmentRequestV02(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::RequestToPayDebtorActivationCancellationRequestV02(ref value) => value.validate(),
			#[cfg(feature = "reda")]
			Document::RequestToPayDebtorActivationStatusReportV02(ref value) => value.validate(),
			#[cfg(feature = "remt")]
			Document::RemittanceAdviceV06(ref value) => value.validate(),
			#[cfg(feature = "remt")]
			Document::RemittanceLocationAdviceV03(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::GetAccountV08(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::ReturnAccountV10(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::GetTransactionV11(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::ReturnTransactionV11(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::ModifyTransactionV10(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::CancelTransactionV11(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::GetLimitV08(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::ReturnLimitV09(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::ModifyLimitV08(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::DeleteLimitV08(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::GetMemberV04(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::ReturnMemberV05(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::ModifyMemberV04(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::GetCurrencyExchangeRateV04(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::ReturnCurrencyExchangeRateV05(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::GetBusinessDayInformationV05(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::ReturnBusinessDayInformationV07(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::GetGeneralBusinessInformationV04(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::ReturnGeneralBusinessInformationV06(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::BackupPaymentV07(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::ModifyStandingOrderV08(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::ReceiptV08(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::UnableToApplyV10(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::ClaimNonReceiptV10(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::AdditionalPaymentInformationV12(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::ResolutionOfInvestigationV13(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::NotificationOfCaseAssignmentV06(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::RejectInvestigationV07(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::CancelCaseAssignmentV05(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::RequestForDuplicateV07(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::DuplicateV07(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::ProprietaryFormatInvestigationV06(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::DebitAuthorisationResponseV06(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::DebitAuthorisationRequestV10(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::CaseStatusReportRequestV05(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::CaseStatusReportV06(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::FundEstimatedCashForecastReportV04(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::FundConfirmedCashForecastReportV04(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::FundDetailedEstimatedCashForecastReportV04(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::FundDetailedConfirmedCashForecastReportV04(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::FundConfirmedCashForecastReportCancellationV03(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::FundDetailedConfirmedCashForecastReportCancellationV03(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::GetReservationV08(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::ReturnReservationV08(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::ModifyReservationV07(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::DeleteReservationV07(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::LiquidityCreditTransferV07(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::LiquidityDebitTransferV07(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::BankToCustomerAccountReportV12(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::BankToCustomerStatementV12(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::BankToCustomerDebitCreditNotificationV12(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::CustomerPaymentCancellationRequestV12(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::FIToFIPaymentCancellationRequestV11(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::NotificationToReceiveV08(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::NotificationToReceiveCancellationAdviceV09(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::NotificationToReceiveStatusReportV08(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::AccountReportingRequestV07(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::PayInCallV02(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::PayInScheduleV03(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::PayInEventAcknowledgementV02(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::IntraBalanceMovementInstructionV02(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::IntraBalanceMovementStatusAdviceV02(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::IntraBalanceMovementConfirmationV02(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::GetStandingOrderV05(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::ReturnStandingOrderV06(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::DeleteStandingOrderV05(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::IntraBalanceMovementModificationRequestV02(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::IntraBalanceMovementModificationRequestStatusAdviceV02(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::IntraBalanceMovementCancellationRequestV02(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::IntraBalanceMovementCancellationRequestStatusAdviceV02(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::IntraBalanceMovementQueryV02(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::IntraBalanceMovementQueryResponseV02(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::IntraBalanceMovementModificationQueryV02(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::IntraBalanceMovementModificationReportV02(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::IntraBalanceMovementCancellationQueryV02(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::IntraBalanceMovementCancellationReportV02(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::IntraBalanceMovementPostingReportV02(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::IntraBalanceMovementPendingReportV02(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::BankServicesBillingStatementV05(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::RequestToModifyPaymentV09(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::NetReportV02(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::CreateLimitV02(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::CreateStandingOrderV03(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::CreateReservationV03(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::CreateMemberV01(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::ChargesPaymentNotificationV02(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::ChargesPaymentRequestV02(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::ChequePresentmentNotificationV02(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::ChequeCancellationOrStopRequestV02(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::ChequeCancellationOrStopReportV02(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::InvestigationRequestV01(ref value) => value.validate(),
			#[cfg(feature = "camt")]
			Document::InvestigationResponseV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::InformationRequestOpeningV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::InformationRequestResponseV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::InformationRequestStatusChangeNotificationV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::MoneyMarketSecuredMarketStatisticalReportV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::MoneyMarketUnsecuredMarketStatisticalReportV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::MoneyMarketForeignExchangeSwapsStatisticalReportV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::MoneyMarketOvernightIndexSwapsStatisticalReportV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::FinancialInstrumentReportingTransactionReportV03(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::FinancialInstrumentReportingReferenceDataReportV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::ContractRegistrationRequestV04(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::ContractRegistrationConfirmationV04(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::ContractRegistrationClosureRequestV04(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::ContractRegistrationAmendmentRequestV04(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::ContractRegistrationStatementV04(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::ContractRegistrationStatementRequestV04(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::PaymentRegulatoryInformationNotificationV04(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::CurrencyControlSupportingDocumentDeliveryV04(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::CurrencyControlRequestOrLetterV04(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::CurrencyControlStatusAdviceV04(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::MoneyMarketStatisticalReportStatusAdviceV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::DerivativesTradeReportQueryV05(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::DerivativesTradeReportV04(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::FinancialInstrumentReportingStatusAdviceV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::FinancialInstrumentReportingEquityTransparencyDataReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::FinancialInstrumentReportingNonEquityTransparencyDataReportV03(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::InvoiceTaxReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::FinancialInstrumentReportingTradingVolumeCapDataReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::FinancialInstrumentReportingReferenceDataDeltaReportV03(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::InvoiceTaxReportStatusAdviceV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::FinancialInstrumentReportingNonWorkingDayReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::FinancialInstrumentReportingEquityTradingActivityReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::FinancialInstrumentReportingNonEquityTradingActivityReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::FinancialInstrumentReportingInvalidReferenceDataReportV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::FinancialInstrumentReportingReferenceDataIndexReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::FinancialInstrumentReportingEquityTradingActivityResultV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::FinancialInstrumentReportingNonEquityTradingActivityResultV03(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::FinancialInstrumentReportingCountryCodeReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::FinancialInstrumentReportingCurrencyCodeReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::FinancialInstrumentReportingMarketIdentificationCodeReportV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::FinancialInstrumentReportingInstrumentClassificationReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::SecuritiesFinancingReportingTransactionReportV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::FinancialInstrumentReportingTradingVolumeCapResultReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::CCPClearingMemberReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::CCPMemberRequirementsReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::CCPMemberObligationsReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::CCPPortfolioStressTestingDefinitionReportV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::CCPPortfolioStressTestingResultReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::CCPIncomeStatementAndCapitalAdequacyReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::CCPDailyCashFlowsReportV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::CCPInvestmentsReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::CCPLiquidityStressTestingDefinitionReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::CCPLiquidityStressTestingResultReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::CCPAvailableFinancialResourcesReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::CCPBackTestingDefinitionReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::CCPBackTestingResultReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::CCPCollateralReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::CCPAccountPositionReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::CCPClearedProductReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::SecuritiesFinancingReportingTransactionMarginDataReportV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::SecuritiesFinancingReportingTransactionReusedCollateralDataReportV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::SettlementInternaliserReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::FinancialSupervisedPartyIdentityReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::FinancialBenchmarkReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::SecuritiesFinancingReportingPairingRequestV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::SecuritiesFinancingReportingTransactionStateReportV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::SecuritiesFinancingReportingReconciliationStatusAdviceV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::SecuritiesFinancingReportingMissingCollateralRequestV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::SecuritiesFinancingReportingTransactionStatusAdviceV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::SecuritiesFinancingReportingMarginDataTransactionStateReportV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::SecuritiesFinancingReportingReusedCollateralDataTransactionStateReportV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::DerivativesTradePositionSetReportV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::DerivativesTradeReconciliationStatisticalReportV03(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::DerivativesTradeRejectionStatisticalReportV04(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::SecuritiesFinancingReportingTransactionQueryV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::SettlementFailsMonthlyReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::SettlementFailsAnnualReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::FinancialInstrumentReportingCancellationReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::SecuritiesFinancingReportingPositionSetReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::DerivativesTradeWarningsReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::DerivativesTradeStateReportV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::DerivativesTradeMarginDataReportV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::DerivativesTradeMarginDataTransactionStateReportV02(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::CCPInteroperabilityReportV01(ref value) => value.validate(),
			#[cfg(feature = "auth")]
			Document::OrderBookReportV01(ref value) => value.validate(),
			Document::UNKNOWN => {
                // Return an error for the UNKNOWN case
                Err(ValidationError::new(9999, "Unknown document type".to_string()))
            }
        }
    }
}