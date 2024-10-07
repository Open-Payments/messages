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


use serde::{Deserialize, Serialize};
use serde_valid::Validate;

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


#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub enum Document {
	#[validate]
	#[serde(rename = "admi.002.001.01")]
	#[cfg(feature = "admi")]
	Admi00200101(Box<Admi00200101>),

	#[validate]
	#[serde(rename = "SysEvtNtfctn")]
	#[cfg(feature = "admi")]
	SystemEventNotificationV02(Box<SystemEventNotificationV02>),

	#[validate]
	#[serde(rename = "RptQryReq")]
	#[cfg(feature = "admi")]
	ReportQueryRequestV02(Box<ReportQueryRequestV02>),

	#[validate]
	#[serde(rename = "RsndReq")]
	#[cfg(feature = "admi")]
	ResendRequestV01(Box<ResendRequestV01>),

	#[validate]
	#[serde(rename = "RctAck")]
	#[cfg(feature = "admi")]
	ReceiptAcknowledgementV01(Box<ReceiptAcknowledgementV01>),

	#[validate]
	#[serde(rename = "StatcDataReq")]
	#[cfg(feature = "admi")]
	StaticDataRequestV02(Box<StaticDataRequestV02>),

	#[validate]
	#[serde(rename = "StatcDataRpt")]
	#[cfg(feature = "admi")]
	StaticDataReportV02(Box<StaticDataReportV02>),

	#[validate]
	#[serde(rename = "SysEvtAck")]
	#[cfg(feature = "admi")]
	SystemEventAcknowledgementV01(Box<SystemEventAcknowledgementV01>),

	#[validate]
	#[serde(rename = "PrcgReq")]
	#[cfg(feature = "admi")]
	ProcessingRequestV02(Box<ProcessingRequestV02>),

	#[validate]
	#[serde(rename = "NtfctnOfCrspdc")]
	#[cfg(feature = "admi")]
	NotificationOfCorrespondenceV01(Box<NotificationOfCorrespondenceV01>),

	#[validate]
	#[serde(rename = "FIToFIPmtStsRpt")]
	#[cfg(feature = "pacs")]
	FIToFIPaymentStatusReportV12(Box<FIToFIPaymentStatusReportV12>),

	#[validate]
	#[serde(rename = "FIToFIPmtStsRpt")]
	#[cfg(feature = "pacs")]
	FIToFIPaymentStatusReportV14(Box<FIToFIPaymentStatusReportV14>),

	#[validate]
	#[serde(rename = "FIToFICstmrDrctDbt")]
	#[cfg(feature = "pacs")]
	FIToFICustomerDirectDebitV11(Box<FIToFICustomerDirectDebitV11>),

	#[validate]
	#[serde(rename = "PmtRtr")]
	#[cfg(feature = "pacs")]
	PaymentReturnV13(Box<PaymentReturnV13>),

	#[validate]
	#[serde(rename = "FIToFIPmtRvsl")]
	#[cfg(feature = "pacs")]
	FIToFIPaymentReversalV13(Box<FIToFIPaymentReversalV13>),

	#[validate]
	#[serde(rename = "FIToFICstmrCdtTrf")]
	#[cfg(feature = "pacs")]
	FIToFICustomerCreditTransferV12(Box<FIToFICustomerCreditTransferV12>),

	#[validate]
	#[serde(rename = "FICdtTrf")]
	#[cfg(feature = "pacs")]
	FinancialInstitutionCreditTransferV11(Box<FinancialInstitutionCreditTransferV11>),

	#[validate]
	#[serde(rename = "FIDrctDbt")]
	#[cfg(feature = "pacs")]
	FinancialInstitutionDirectDebitV06(Box<FinancialInstitutionDirectDebitV06>),

	#[validate]
	#[serde(rename = "FIToFIPmtStsReq")]
	#[cfg(feature = "pacs")]
	FIToFIPaymentStatusRequestV06(Box<FIToFIPaymentStatusRequestV06>),

	#[validate]
	#[serde(rename = "MulSttlmReq")]
	#[cfg(feature = "pacs")]
	MultilateralSettlementRequestV02(Box<MultilateralSettlementRequestV02>),

	#[validate]
	#[serde(rename = "CstmrCdtTrfInitn")]
	#[cfg(feature = "pain")]
	CustomerCreditTransferInitiationV12(Box<CustomerCreditTransferInitiationV12>),

	#[validate]
	#[serde(rename = "CstmrPmtStsRpt")]
	#[cfg(feature = "pain")]
	CustomerPaymentStatusReportV14(Box<CustomerPaymentStatusReportV14>),

	#[validate]
	#[serde(rename = "CstmrPmtRvsl")]
	#[cfg(feature = "pain")]
	CustomerPaymentReversalV12(Box<CustomerPaymentReversalV12>),

	#[validate]
	#[serde(rename = "CstmrDrctDbtInitn")]
	#[cfg(feature = "pain")]
	CustomerDirectDebitInitiationV11(Box<CustomerDirectDebitInitiationV11>),

	#[validate]
	#[serde(rename = "MndtInitnReq")]
	#[cfg(feature = "pain")]
	MandateInitiationRequestV08(Box<MandateInitiationRequestV08>),

	#[validate]
	#[serde(rename = "MndtAmdmntReq")]
	#[cfg(feature = "pain")]
	MandateAmendmentRequestV08(Box<MandateAmendmentRequestV08>),

	#[validate]
	#[serde(rename = "MndtCxlReq")]
	#[cfg(feature = "pain")]
	MandateCancellationRequestV08(Box<MandateCancellationRequestV08>),

	#[validate]
	#[serde(rename = "MndtAccptncRpt")]
	#[cfg(feature = "pain")]
	MandateAcceptanceReportV08(Box<MandateAcceptanceReportV08>),

	#[validate]
	#[serde(rename = "CdtrPmtActvtnReq")]
	#[cfg(feature = "pain")]
	CreditorPaymentActivationRequestV11(Box<CreditorPaymentActivationRequestV11>),

	#[validate]
	#[serde(rename = "CdtrPmtActvtnReqStsRpt")]
	#[cfg(feature = "pain")]
	CreditorPaymentActivationRequestStatusReportV11(Box<CreditorPaymentActivationRequestStatusReportV11>),

	#[validate]
	#[serde(rename = "MndtCpyReq")]
	#[cfg(feature = "pain")]
	MandateCopyRequestV04(Box<MandateCopyRequestV04>),

	#[validate]
	#[serde(rename = "MndtSspnsnReq")]
	#[cfg(feature = "pain")]
	MandateSuspensionRequestV04(Box<MandateSuspensionRequestV04>),

	#[validate]
	#[serde(rename = "AcctOpngInstr")]
	#[cfg(feature = "acmt")]
	AccountOpeningInstructionV08(Box<AccountOpeningInstructionV08>),

	#[validate]
	#[serde(rename = "AcctDtlsConf")]
	#[cfg(feature = "acmt")]
	AccountDetailsConfirmationV08(Box<AccountDetailsConfirmationV08>),

	#[validate]
	#[serde(rename = "AcctModInstr")]
	#[cfg(feature = "acmt")]
	AccountModificationInstructionV08(Box<AccountModificationInstructionV08>),

	#[validate]
	#[serde(rename = "ReqForAcctMgmtStsRpt")]
	#[cfg(feature = "acmt")]
	RequestForAccountManagementStatusReportV06(Box<RequestForAccountManagementStatusReportV06>),

	#[validate]
	#[serde(rename = "AcctMgmtStsRpt")]
	#[cfg(feature = "acmt")]
	AccountManagementStatusReportV07(Box<AccountManagementStatusReportV07>),

	#[validate]
	#[serde(rename = "AcctOpngReq")]
	#[cfg(feature = "acmt")]
	AccountOpeningRequestV05(Box<AccountOpeningRequestV05>),

	#[validate]
	#[serde(rename = "AcctOpngAmdmntReq")]
	#[cfg(feature = "acmt")]
	AccountOpeningAmendmentRequestV05(Box<AccountOpeningAmendmentRequestV05>),

	#[validate]
	#[serde(rename = "AcctOpngAddtlInfReq")]
	#[cfg(feature = "acmt")]
	AccountOpeningAdditionalInformationRequestV04(Box<AccountOpeningAdditionalInformationRequestV04>),

	#[validate]
	#[serde(rename = "AcctReqAck")]
	#[cfg(feature = "acmt")]
	AccountRequestAcknowledgementV04(Box<AccountRequestAcknowledgementV04>),

	#[validate]
	#[serde(rename = "AcctReqRjctn")]
	#[cfg(feature = "acmt")]
	AccountRequestRejectionV04(Box<AccountRequestRejectionV04>),

	#[validate]
	#[serde(rename = "AcctAddtlInfReq")]
	#[cfg(feature = "acmt")]
	AccountAdditionalInformationRequestV04(Box<AccountAdditionalInformationRequestV04>),

	#[validate]
	#[serde(rename = "AcctRptReq")]
	#[cfg(feature = "acmt")]
	AccountReportRequestV04(Box<AccountReportRequestV04>),

	#[validate]
	#[serde(rename = "AcctRpt")]
	#[cfg(feature = "acmt")]
	AccountReportV05(Box<AccountReportV05>),

	#[validate]
	#[serde(rename = "AcctExcldMndtMntncReq")]
	#[cfg(feature = "acmt")]
	AccountExcludedMandateMaintenanceRequestV04(Box<AccountExcludedMandateMaintenanceRequestV04>),

	#[validate]
	#[serde(rename = "AcctExcldMndtMntncAmdmntReq")]
	#[cfg(feature = "acmt")]
	AccountExcludedMandateMaintenanceAmendmentRequestV04(Box<AccountExcludedMandateMaintenanceAmendmentRequestV04>),

	#[validate]
	#[serde(rename = "AcctMndtMntncReq")]
	#[cfg(feature = "acmt")]
	AccountMandateMaintenanceRequestV04(Box<AccountMandateMaintenanceRequestV04>),

	#[validate]
	#[serde(rename = "AcctMndtMntncAmdmntReq")]
	#[cfg(feature = "acmt")]
	AccountMandateMaintenanceAmendmentRequestV04(Box<AccountMandateMaintenanceAmendmentRequestV04>),

	#[validate]
	#[serde(rename = "AcctClsgReq")]
	#[cfg(feature = "acmt")]
	AccountClosingRequestV04(Box<AccountClosingRequestV04>),

	#[validate]
	#[serde(rename = "AcctClsgAmdmntReq")]
	#[cfg(feature = "acmt")]
	AccountClosingAmendmentRequestV04(Box<AccountClosingAmendmentRequestV04>),

	#[validate]
	#[serde(rename = "AcctClsgAddtlInfReq")]
	#[cfg(feature = "acmt")]
	AccountClosingAdditionalInformationRequestV04(Box<AccountClosingAdditionalInformationRequestV04>),

	#[validate]
	#[serde(rename = "IdModAdvc")]
	#[cfg(feature = "acmt")]
	IdentificationModificationAdviceV04(Box<IdentificationModificationAdviceV04>),

	#[validate]
	#[serde(rename = "IdVrfctnReq")]
	#[cfg(feature = "acmt")]
	IdentificationVerificationRequestV04(Box<IdentificationVerificationRequestV04>),

	#[validate]
	#[serde(rename = "IdVrfctnRpt")]
	#[cfg(feature = "acmt")]
	IdentificationVerificationReportV04(Box<IdentificationVerificationReportV04>),

	#[validate]
	#[serde(rename = "AcctSwtchInfReq")]
	#[cfg(feature = "acmt")]
	AccountSwitchInformationRequestV05(Box<AccountSwitchInformationRequestV05>),

	#[validate]
	#[serde(rename = "AcctSwtchInfRspn")]
	#[cfg(feature = "acmt")]
	AccountSwitchInformationResponseV05(Box<AccountSwitchInformationResponseV05>),

	#[validate]
	#[serde(rename = "AcctSwtchCclExstgPmt")]
	#[cfg(feature = "acmt")]
	AccountSwitchCancelExistingPaymentV05(Box<AccountSwitchCancelExistingPaymentV05>),

	#[validate]
	#[serde(rename = "AcctSwtchReqRdrctn")]
	#[cfg(feature = "acmt")]
	AccountSwitchRequestRedirectionV04(Box<AccountSwitchRequestRedirectionV04>),

	#[validate]
	#[serde(rename = "AcctSwtchReqBalTrf")]
	#[cfg(feature = "acmt")]
	AccountSwitchRequestBalanceTransferV05(Box<AccountSwitchRequestBalanceTransferV05>),

	#[validate]
	#[serde(rename = "AcctSwtchBalTrfAck")]
	#[cfg(feature = "acmt")]
	AccountSwitchBalanceTransferAcknowledgementV05(Box<AccountSwitchBalanceTransferAcknowledgementV05>),

	#[validate]
	#[serde(rename = "AcctSwtchNtfyAcctSwtchCmplt")]
	#[cfg(feature = "acmt")]
	AccountSwitchNotifyAccountSwitchCompleteV02(Box<AccountSwitchNotifyAccountSwitchCompleteV02>),

	#[validate]
	#[serde(rename = "AcctSwtchReqPmt")]
	#[cfg(feature = "acmt")]
	AccountSwitchRequestPaymentV05(Box<AccountSwitchRequestPaymentV05>),

	#[validate]
	#[serde(rename = "AcctSwtchPmtRspn")]
	#[cfg(feature = "acmt")]
	AccountSwitchPaymentResponseV02(Box<AccountSwitchPaymentResponseV02>),

	#[validate]
	#[serde(rename = "AcctSwtchTermntnSwtch")]
	#[cfg(feature = "acmt")]
	AccountSwitchTerminationSwitchV01(Box<AccountSwitchTerminationSwitchV01>),

	#[validate]
	#[serde(rename = "AcctSwtchTechRjctn")]
	#[cfg(feature = "acmt")]
	AccountSwitchTechnicalRejectionV02(Box<AccountSwitchTechnicalRejectionV02>),

	#[validate]
	#[serde(rename = "PricRpt")]
	#[cfg(feature = "reda")]
	PriceReportV04(Box<PriceReportV04>),

	#[validate]
	#[serde(rename = "PricRptCxl")]
	#[cfg(feature = "reda")]
	PriceReportCancellationV04(Box<PriceReportCancellationV04>),

	#[validate]
	#[serde(rename = "FndRefDataRpt")]
	#[cfg(feature = "reda")]
	FundReferenceDataReportV07(Box<FundReferenceDataReportV07>),

	#[validate]
	#[serde(rename = "InvstmtFndRptReq")]
	#[cfg(feature = "reda")]
	InvestmentFundReportRequestV03(Box<InvestmentFundReportRequestV03>),

	#[validate]
	#[serde(rename = "SctyCreReq")]
	#[cfg(feature = "reda")]
	SecurityCreationRequestV01(Box<SecurityCreationRequestV01>),

	#[validate]
	#[serde(rename = "SctyMntncReq")]
	#[cfg(feature = "reda")]
	SecurityMaintenanceRequestV01(Box<SecurityMaintenanceRequestV01>),

	#[validate]
	#[serde(rename = "SctyCreStsAdvc")]
	#[cfg(feature = "reda")]
	SecurityCreationStatusAdviceV01(Box<SecurityCreationStatusAdviceV01>),

	#[validate]
	#[serde(rename = "SctyActvtyAdvc")]
	#[cfg(feature = "reda")]
	SecurityActivityAdviceV01(Box<SecurityActivityAdviceV01>),

	#[validate]
	#[serde(rename = "SctyQry")]
	#[cfg(feature = "reda")]
	SecurityQueryV01(Box<SecurityQueryV01>),

	#[validate]
	#[serde(rename = "SctyRpt")]
	#[cfg(feature = "reda")]
	SecurityReportV01(Box<SecurityReportV01>),

	#[validate]
	#[serde(rename = "SctyDeltnReq")]
	#[cfg(feature = "reda")]
	SecurityDeletionRequestV01(Box<SecurityDeletionRequestV01>),

	#[validate]
	#[serde(rename = "PtyCreReq")]
	#[cfg(feature = "reda")]
	PartyCreationRequestV02(Box<PartyCreationRequestV02>),

	#[validate]
	#[serde(rename = "PtyQry")]
	#[cfg(feature = "reda")]
	PartyQueryV01(Box<PartyQueryV01>),

	#[validate]
	#[serde(rename = "PtyStsAdvc")]
	#[cfg(feature = "reda")]
	PartyStatusAdviceV01(Box<PartyStatusAdviceV01>),

	#[validate]
	#[serde(rename = "PtyRpt")]
	#[cfg(feature = "reda")]
	PartyReportV02(Box<PartyReportV02>),

	#[validate]
	#[serde(rename = "SctiesAcctCreReq")]
	#[cfg(feature = "reda")]
	SecuritiesAccountCreationRequestV01(Box<SecuritiesAccountCreationRequestV01>),

	#[validate]
	#[serde(rename = "SctiesAcctQry")]
	#[cfg(feature = "reda")]
	SecuritiesAccountQueryV01(Box<SecuritiesAccountQueryV01>),

	#[validate]
	#[serde(rename = "SctiesAcctStsAdvc")]
	#[cfg(feature = "reda")]
	SecuritiesAccountStatusAdviceV01(Box<SecuritiesAccountStatusAdviceV01>),

	#[validate]
	#[serde(rename = "SctiesAcctRpt")]
	#[cfg(feature = "reda")]
	SecuritiesAccountReportV01(Box<SecuritiesAccountReportV01>),

	#[validate]
	#[serde(rename = "PtyModReq")]
	#[cfg(feature = "reda")]
	PartyModificationRequestV02(Box<PartyModificationRequestV02>),

	#[validate]
	#[serde(rename = "SctiesAcctModReq")]
	#[cfg(feature = "reda")]
	SecuritiesAccountModificationRequestV01(Box<SecuritiesAccountModificationRequestV01>),

	#[validate]
	#[serde(rename = "SctyMntncStsAdvc")]
	#[cfg(feature = "reda")]
	SecurityMaintenanceStatusAdviceV01(Box<SecurityMaintenanceStatusAdviceV01>),

	#[validate]
	#[serde(rename = "SctyDeltnStsAdvc")]
	#[cfg(feature = "reda")]
	SecurityDeletionStatusAdviceV01(Box<SecurityDeletionStatusAdviceV01>),

	#[validate]
	#[serde(rename = "PtyDeltnReq")]
	#[cfg(feature = "reda")]
	PartyDeletionRequestV01(Box<PartyDeletionRequestV01>),

	#[validate]
	#[serde(rename = "SctiesAcctDeltnReq")]
	#[cfg(feature = "reda")]
	SecuritiesAccountDeletionRequestV01(Box<SecuritiesAccountDeletionRequestV01>),

	#[validate]
	#[serde(rename = "SctiesAudtTrlQry")]
	#[cfg(feature = "reda")]
	SecuritiesAuditTrailQueryV01(Box<SecuritiesAuditTrailQueryV01>),

	#[validate]
	#[serde(rename = "SctiesAudtTrlRpt")]
	#[cfg(feature = "reda")]
	SecuritiesAuditTrailReportV01(Box<SecuritiesAuditTrailReportV01>),

	#[validate]
	#[serde(rename = "SctiesAcctActvtyAdvc")]
	#[cfg(feature = "reda")]
	SecuritiesAccountActivityAdviceV01(Box<SecuritiesAccountActivityAdviceV01>),

	#[validate]
	#[serde(rename = "SctiesAcctAudtTrlQry")]
	#[cfg(feature = "reda")]
	SecuritiesAccountAuditTrailQueryV01(Box<SecuritiesAccountAuditTrailQueryV01>),

	#[validate]
	#[serde(rename = "SctiesAcctAudtTrlRpt")]
	#[cfg(feature = "reda")]
	SecuritiesAccountAuditTrailReportV01(Box<SecuritiesAccountAuditTrailReportV01>),

	#[validate]
	#[serde(rename = "PtyActvtyAdvc")]
	#[cfg(feature = "reda")]
	PartyActivityAdviceV02(Box<PartyActivityAdviceV02>),

	#[validate]
	#[serde(rename = "PtyAudtTrlQry")]
	#[cfg(feature = "reda")]
	PartyAuditTrailQueryV01(Box<PartyAuditTrailQueryV01>),

	#[validate]
	#[serde(rename = "PtyAudtTrlRpt")]
	#[cfg(feature = "reda")]
	PartyAuditTrailReportV02(Box<PartyAuditTrailReportV02>),

	#[validate]
	#[serde(rename = "StgSttlmInstr")]
	#[cfg(feature = "reda")]
	StandingSettlementInstructionV01(Box<StandingSettlementInstructionV01>),

	#[validate]
	#[serde(rename = "StgSttlmInstrDeltn")]
	#[cfg(feature = "reda")]
	StandingSettlementInstructionDeletionV01(Box<StandingSettlementInstructionDeletionV01>),

	#[validate]
	#[serde(rename = "StgSttlmInstrStsAdvc")]
	#[cfg(feature = "reda")]
	StandingSettlementInstructionStatusAdviceV01(Box<StandingSettlementInstructionStatusAdviceV01>),

	#[validate]
	#[serde(rename = "StgSttlmInstrCxl")]
	#[cfg(feature = "reda")]
	StandingSettlementInstructionCancellationV01(Box<StandingSettlementInstructionCancellationV01>),

	#[validate]
	#[serde(rename = "NetgCutOffRefDataUpdReq")]
	#[cfg(feature = "reda")]
	NettingCutOffReferenceDataUpdateRequestV02(Box<NettingCutOffReferenceDataUpdateRequestV02>),

	#[validate]
	#[serde(rename = "NetgCutOffRefDataRpt")]
	#[cfg(feature = "reda")]
	NettingCutOffReferenceDataReportV02(Box<NettingCutOffReferenceDataReportV02>),

	#[validate]
	#[serde(rename = "CalQry")]
	#[cfg(feature = "reda")]
	CalendarQueryV02(Box<CalendarQueryV02>),

	#[validate]
	#[serde(rename = "CalRpt")]
	#[cfg(feature = "reda")]
	CalendarReportV02(Box<CalendarReportV02>),

	#[validate]
	#[serde(rename = "ReqToPayCdtrEnrlmntReq")]
	#[cfg(feature = "reda")]
	RequestToPayCreditorEnrolmentRequestV02(Box<RequestToPayCreditorEnrolmentRequestV02>),

	#[validate]
	#[serde(rename = "ReqToPayCdtrEnrlmntAmdmntReq")]
	#[cfg(feature = "reda")]
	RequestToPayCreditorEnrolmentAmendmentRequestV02(Box<RequestToPayCreditorEnrolmentAmendmentRequestV02>),

	#[validate]
	#[serde(rename = "ReqToPayCdtrEnrlmntCxlReq")]
	#[cfg(feature = "reda")]
	RequestToPayCreditorEnrolmentCancellationRequestV02(Box<RequestToPayCreditorEnrolmentCancellationRequestV02>),

	#[validate]
	#[serde(rename = "ReqToPayCdtrEnrlmntStsRpt")]
	#[cfg(feature = "reda")]
	RequestToPayCreditorEnrolmentStatusReportV02(Box<RequestToPayCreditorEnrolmentStatusReportV02>),

	#[validate]
	#[serde(rename = "ReqToPayDbtrActvtnReq")]
	#[cfg(feature = "reda")]
	RequestToPayDebtorActivationRequestV02(Box<RequestToPayDebtorActivationRequestV02>),

	#[validate]
	#[serde(rename = "ReqToPayDbtrActvtnAmdmntReq")]
	#[cfg(feature = "reda")]
	RequestToPayDebtorActivationAmendmentRequestV02(Box<RequestToPayDebtorActivationAmendmentRequestV02>),

	#[validate]
	#[serde(rename = "ReqToPayDbtrActvtnCxlReq")]
	#[cfg(feature = "reda")]
	RequestToPayDebtorActivationCancellationRequestV02(Box<RequestToPayDebtorActivationCancellationRequestV02>),

	#[validate]
	#[serde(rename = "ReqToPayDbtrActvtnStsRpt")]
	#[cfg(feature = "reda")]
	RequestToPayDebtorActivationStatusReportV02(Box<RequestToPayDebtorActivationStatusReportV02>),

	#[validate]
	#[serde(rename = "RmtAdvc")]
	#[cfg(feature = "remt")]
	RemittanceAdviceV06(Box<RemittanceAdviceV06>),

	#[validate]
	#[serde(rename = "RmtLctnAdvc")]
	#[cfg(feature = "remt")]
	RemittanceLocationAdviceV03(Box<RemittanceLocationAdviceV03>),

	#[validate]
	#[serde(rename = "GetAcct")]
	#[cfg(feature = "camt")]
	GetAccountV08(Box<GetAccountV08>),

	#[validate]
	#[serde(rename = "RtrAcct")]
	#[cfg(feature = "camt")]
	ReturnAccountV10(Box<ReturnAccountV10>),

	#[validate]
	#[serde(rename = "GetTx")]
	#[cfg(feature = "camt")]
	GetTransactionV11(Box<GetTransactionV11>),

	#[validate]
	#[serde(rename = "RtrTx")]
	#[cfg(feature = "camt")]
	ReturnTransactionV11(Box<ReturnTransactionV11>),

	#[validate]
	#[serde(rename = "ModfyTx")]
	#[cfg(feature = "camt")]
	ModifyTransactionV10(Box<ModifyTransactionV10>),

	#[validate]
	#[serde(rename = "CclTx")]
	#[cfg(feature = "camt")]
	CancelTransactionV11(Box<CancelTransactionV11>),

	#[validate]
	#[serde(rename = "GetLmt")]
	#[cfg(feature = "camt")]
	GetLimitV08(Box<GetLimitV08>),

	#[validate]
	#[serde(rename = "RtrLmt")]
	#[cfg(feature = "camt")]
	ReturnLimitV09(Box<ReturnLimitV09>),

	#[validate]
	#[serde(rename = "ModfyLmt")]
	#[cfg(feature = "camt")]
	ModifyLimitV08(Box<ModifyLimitV08>),

	#[validate]
	#[serde(rename = "DelLmt")]
	#[cfg(feature = "camt")]
	DeleteLimitV08(Box<DeleteLimitV08>),

	#[validate]
	#[serde(rename = "GetMmb")]
	#[cfg(feature = "camt")]
	GetMemberV04(Box<GetMemberV04>),

	#[validate]
	#[serde(rename = "RtrMmb")]
	#[cfg(feature = "camt")]
	ReturnMemberV05(Box<ReturnMemberV05>),

	#[validate]
	#[serde(rename = "ModfyMmb")]
	#[cfg(feature = "camt")]
	ModifyMemberV04(Box<ModifyMemberV04>),

	#[validate]
	#[serde(rename = "GetCcyXchgRate")]
	#[cfg(feature = "camt")]
	GetCurrencyExchangeRateV04(Box<GetCurrencyExchangeRateV04>),

	#[validate]
	#[serde(rename = "RtrCcyXchgRate")]
	#[cfg(feature = "camt")]
	ReturnCurrencyExchangeRateV05(Box<ReturnCurrencyExchangeRateV05>),

	#[validate]
	#[serde(rename = "GetBizDayInf")]
	#[cfg(feature = "camt")]
	GetBusinessDayInformationV05(Box<GetBusinessDayInformationV05>),

	#[validate]
	#[serde(rename = "RtrBizDayInf")]
	#[cfg(feature = "camt")]
	ReturnBusinessDayInformationV07(Box<ReturnBusinessDayInformationV07>),

	#[validate]
	#[serde(rename = "GetGnlBizInf")]
	#[cfg(feature = "camt")]
	GetGeneralBusinessInformationV04(Box<GetGeneralBusinessInformationV04>),

	#[validate]
	#[serde(rename = "RtrGnlBizInf")]
	#[cfg(feature = "camt")]
	ReturnGeneralBusinessInformationV06(Box<ReturnGeneralBusinessInformationV06>),

	#[validate]
	#[serde(rename = "BckpPmt")]
	#[cfg(feature = "camt")]
	BackupPaymentV07(Box<BackupPaymentV07>),

	#[validate]
	#[serde(rename = "ModfyStgOrdr")]
	#[cfg(feature = "camt")]
	ModifyStandingOrderV08(Box<ModifyStandingOrderV08>),

	#[validate]
	#[serde(rename = "Rct")]
	#[cfg(feature = "camt")]
	ReceiptV08(Box<ReceiptV08>),

	#[validate]
	#[serde(rename = "UblToApply")]
	#[cfg(feature = "camt")]
	UnableToApplyV10(Box<UnableToApplyV10>),

	#[validate]
	#[serde(rename = "ClmNonRct")]
	#[cfg(feature = "camt")]
	ClaimNonReceiptV10(Box<ClaimNonReceiptV10>),

	#[validate]
	#[serde(rename = "AddtlPmtInf")]
	#[cfg(feature = "camt")]
	AdditionalPaymentInformationV12(Box<AdditionalPaymentInformationV12>),

	#[validate]
	#[serde(rename = "RsltnOfInvstgtn")]
	#[cfg(feature = "camt")]
	ResolutionOfInvestigationV13(Box<ResolutionOfInvestigationV13>),

	#[validate]
	#[serde(rename = "NtfctnOfCaseAssgnmt")]
	#[cfg(feature = "camt")]
	NotificationOfCaseAssignmentV06(Box<NotificationOfCaseAssignmentV06>),

	#[validate]
	#[serde(rename = "RjctInvstgtn")]
	#[cfg(feature = "camt")]
	RejectInvestigationV07(Box<RejectInvestigationV07>),

	#[validate]
	#[serde(rename = "CclCaseAssgnmt")]
	#[cfg(feature = "camt")]
	CancelCaseAssignmentV05(Box<CancelCaseAssignmentV05>),

	#[validate]
	#[serde(rename = "ReqForDplct")]
	#[cfg(feature = "camt")]
	RequestForDuplicateV07(Box<RequestForDuplicateV07>),

	#[validate]
	#[serde(rename = "Dplct")]
	#[cfg(feature = "camt")]
	DuplicateV07(Box<DuplicateV07>),

	#[validate]
	#[serde(rename = "PrtryFrmtInvstgtn")]
	#[cfg(feature = "camt")]
	ProprietaryFormatInvestigationV06(Box<ProprietaryFormatInvestigationV06>),

	#[validate]
	#[serde(rename = "DbtAuthstnRspn")]
	#[cfg(feature = "camt")]
	DebitAuthorisationResponseV06(Box<DebitAuthorisationResponseV06>),

	#[validate]
	#[serde(rename = "DbtAuthstnReq")]
	#[cfg(feature = "camt")]
	DebitAuthorisationRequestV10(Box<DebitAuthorisationRequestV10>),

	#[validate]
	#[serde(rename = "CaseStsRptReq")]
	#[cfg(feature = "camt")]
	CaseStatusReportRequestV05(Box<CaseStatusReportRequestV05>),

	#[validate]
	#[serde(rename = "CaseStsRpt")]
	#[cfg(feature = "camt")]
	CaseStatusReportV06(Box<CaseStatusReportV06>),

	#[validate]
	#[serde(rename = "FndEstmtdCshFcstRpt")]
	#[cfg(feature = "camt")]
	FundEstimatedCashForecastReportV04(Box<FundEstimatedCashForecastReportV04>),

	#[validate]
	#[serde(rename = "FndConfdCshFcstRpt")]
	#[cfg(feature = "camt")]
	FundConfirmedCashForecastReportV04(Box<FundConfirmedCashForecastReportV04>),

	#[validate]
	#[serde(rename = "FndDtldEstmtdCshFcstRpt")]
	#[cfg(feature = "camt")]
	FundDetailedEstimatedCashForecastReportV04(Box<FundDetailedEstimatedCashForecastReportV04>),

	#[validate]
	#[serde(rename = "FndDtldConfdCshFcstRpt")]
	#[cfg(feature = "camt")]
	FundDetailedConfirmedCashForecastReportV04(Box<FundDetailedConfirmedCashForecastReportV04>),

	#[validate]
	#[serde(rename = "FndConfdCshFcstRptCxl")]
	#[cfg(feature = "camt")]
	FundConfirmedCashForecastReportCancellationV03(Box<FundConfirmedCashForecastReportCancellationV03>),

	#[validate]
	#[serde(rename = "FndDtldConfdCshFcstRptCxl")]
	#[cfg(feature = "camt")]
	FundDetailedConfirmedCashForecastReportCancellationV03(Box<FundDetailedConfirmedCashForecastReportCancellationV03>),

	#[validate]
	#[serde(rename = "GetRsvatn")]
	#[cfg(feature = "camt")]
	GetReservationV08(Box<GetReservationV08>),

	#[validate]
	#[serde(rename = "RtrRsvatn")]
	#[cfg(feature = "camt")]
	ReturnReservationV08(Box<ReturnReservationV08>),

	#[validate]
	#[serde(rename = "ModfyRsvatn")]
	#[cfg(feature = "camt")]
	ModifyReservationV07(Box<ModifyReservationV07>),

	#[validate]
	#[serde(rename = "DelRsvatn")]
	#[cfg(feature = "camt")]
	DeleteReservationV07(Box<DeleteReservationV07>),

	#[validate]
	#[serde(rename = "LqdtyCdtTrf")]
	#[cfg(feature = "camt")]
	LiquidityCreditTransferV07(Box<LiquidityCreditTransferV07>),

	#[validate]
	#[serde(rename = "LqdtyDbtTrf")]
	#[cfg(feature = "camt")]
	LiquidityDebitTransferV07(Box<LiquidityDebitTransferV07>),

	#[validate]
	#[serde(rename = "BkToCstmrAcctRpt")]
	#[cfg(feature = "camt")]
	BankToCustomerAccountReportV12(Box<BankToCustomerAccountReportV12>),

	#[validate]
	#[serde(rename = "BkToCstmrStmt")]
	#[cfg(feature = "camt")]
	BankToCustomerStatementV12(Box<BankToCustomerStatementV12>),

	#[validate]
	#[serde(rename = "BkToCstmrDbtCdtNtfctn")]
	#[cfg(feature = "camt")]
	BankToCustomerDebitCreditNotificationV12(Box<BankToCustomerDebitCreditNotificationV12>),

	#[validate]
	#[serde(rename = "CstmrPmtCxlReq")]
	#[cfg(feature = "camt")]
	CustomerPaymentCancellationRequestV12(Box<CustomerPaymentCancellationRequestV12>),

	#[validate]
	#[serde(rename = "FIToFIPmtCxlReq")]
	#[cfg(feature = "camt")]
	FIToFIPaymentCancellationRequestV11(Box<FIToFIPaymentCancellationRequestV11>),

	#[validate]
	#[serde(rename = "NtfctnToRcv")]
	#[cfg(feature = "camt")]
	NotificationToReceiveV08(Box<NotificationToReceiveV08>),

	#[validate]
	#[serde(rename = "NtfctnToRcvCxlAdvc")]
	#[cfg(feature = "camt")]
	NotificationToReceiveCancellationAdviceV09(Box<NotificationToReceiveCancellationAdviceV09>),

	#[validate]
	#[serde(rename = "NtfctnToRcvStsRpt")]
	#[cfg(feature = "camt")]
	NotificationToReceiveStatusReportV08(Box<NotificationToReceiveStatusReportV08>),

	#[validate]
	#[serde(rename = "AcctRptgReq")]
	#[cfg(feature = "camt")]
	AccountReportingRequestV07(Box<AccountReportingRequestV07>),

	#[validate]
	#[serde(rename = "PayInCall")]
	#[cfg(feature = "camt")]
	PayInCallV02(Box<PayInCallV02>),

	#[validate]
	#[serde(rename = "PayInSchdl")]
	#[cfg(feature = "camt")]
	PayInScheduleV03(Box<PayInScheduleV03>),

	#[validate]
	#[serde(rename = "PayInEvtAck")]
	#[cfg(feature = "camt")]
	PayInEventAcknowledgementV02(Box<PayInEventAcknowledgementV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntInstr")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementInstructionV02(Box<IntraBalanceMovementInstructionV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntStsAdvc")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementStatusAdviceV02(Box<IntraBalanceMovementStatusAdviceV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntConf")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementConfirmationV02(Box<IntraBalanceMovementConfirmationV02>),

	#[validate]
	#[serde(rename = "GetStgOrdr")]
	#[cfg(feature = "camt")]
	GetStandingOrderV05(Box<GetStandingOrderV05>),

	#[validate]
	#[serde(rename = "RtrStgOrdr")]
	#[cfg(feature = "camt")]
	ReturnStandingOrderV06(Box<ReturnStandingOrderV06>),

	#[validate]
	#[serde(rename = "DelStgOrdr")]
	#[cfg(feature = "camt")]
	DeleteStandingOrderV05(Box<DeleteStandingOrderV05>),

	#[validate]
	#[serde(rename = "IntraBalMvmntModReq")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementModificationRequestV02(Box<IntraBalanceMovementModificationRequestV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntModReqStsAdvc")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementModificationRequestStatusAdviceV02(Box<IntraBalanceMovementModificationRequestStatusAdviceV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntCxlReq")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementCancellationRequestV02(Box<IntraBalanceMovementCancellationRequestV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntCxlReqStsAdvc")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementCancellationRequestStatusAdviceV02(Box<IntraBalanceMovementCancellationRequestStatusAdviceV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntQry")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementQueryV02(Box<IntraBalanceMovementQueryV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntQryRspn")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementQueryResponseV02(Box<IntraBalanceMovementQueryResponseV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntModQry")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementModificationQueryV02(Box<IntraBalanceMovementModificationQueryV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntModRpt")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementModificationReportV02(Box<IntraBalanceMovementModificationReportV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntCxlQry")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementCancellationQueryV02(Box<IntraBalanceMovementCancellationQueryV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntCxlRpt")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementCancellationReportV02(Box<IntraBalanceMovementCancellationReportV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntPstngRpt")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementPostingReportV02(Box<IntraBalanceMovementPostingReportV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntPdgRpt")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementPendingReportV02(Box<IntraBalanceMovementPendingReportV02>),

	#[validate]
	#[serde(rename = "BkSvcsBllgStmt")]
	#[cfg(feature = "camt")]
	BankServicesBillingStatementV05(Box<BankServicesBillingStatementV05>),

	#[validate]
	#[serde(rename = "ReqToModfyPmt")]
	#[cfg(feature = "camt")]
	RequestToModifyPaymentV09(Box<RequestToModifyPaymentV09>),

	#[validate]
	#[serde(rename = "NetRpt")]
	#[cfg(feature = "camt")]
	NetReportV02(Box<NetReportV02>),

	#[validate]
	#[serde(rename = "CretLmt")]
	#[cfg(feature = "camt")]
	CreateLimitV02(Box<CreateLimitV02>),

	#[validate]
	#[serde(rename = "CretStgOrdr")]
	#[cfg(feature = "camt")]
	CreateStandingOrderV03(Box<CreateStandingOrderV03>),

	#[validate]
	#[serde(rename = "CretRsvatn")]
	#[cfg(feature = "camt")]
	CreateReservationV03(Box<CreateReservationV03>),

	#[validate]
	#[serde(rename = "CretMmb")]
	#[cfg(feature = "camt")]
	CreateMemberV01(Box<CreateMemberV01>),

	#[validate]
	#[serde(rename = "ChrgsPmtNtfctn")]
	#[cfg(feature = "camt")]
	ChargesPaymentNotificationV02(Box<ChargesPaymentNotificationV02>),

	#[validate]
	#[serde(rename = "ChrgsPmtReq")]
	#[cfg(feature = "camt")]
	ChargesPaymentRequestV02(Box<ChargesPaymentRequestV02>),

	#[validate]
	#[serde(rename = "ChqPresntmntNtfctn")]
	#[cfg(feature = "camt")]
	ChequePresentmentNotificationV02(Box<ChequePresentmentNotificationV02>),

	#[validate]
	#[serde(rename = "ChqCxlOrStopReq")]
	#[cfg(feature = "camt")]
	ChequeCancellationOrStopRequestV02(Box<ChequeCancellationOrStopRequestV02>),

	#[validate]
	#[serde(rename = "ChqCxlOrStopRpt")]
	#[cfg(feature = "camt")]
	ChequeCancellationOrStopReportV02(Box<ChequeCancellationOrStopReportV02>),

	#[validate]
	#[serde(rename = "InvstgtnReq")]
	#[cfg(feature = "camt")]
	InvestigationRequestV01(Box<InvestigationRequestV01>),

	#[validate]
	#[serde(rename = "InvstgtnRspn")]
	#[cfg(feature = "camt")]
	InvestigationResponseV01(Box<InvestigationResponseV01>),

	#[validate]
	#[serde(rename = "InfReqOpng")]
	#[cfg(feature = "auth")]
	InformationRequestOpeningV02(Box<InformationRequestOpeningV02>),

	#[validate]
	#[serde(rename = "InfReqRspn")]
	#[cfg(feature = "auth")]
	InformationRequestResponseV02(Box<InformationRequestResponseV02>),

	#[validate]
	#[serde(rename = "InfReqStsChngNtfctn")]
	#[cfg(feature = "auth")]
	InformationRequestStatusChangeNotificationV01(Box<InformationRequestStatusChangeNotificationV01>),

	#[validate]
	#[serde(rename = "MnyMktScrdMktSttstclRpt")]
	#[cfg(feature = "auth")]
	MoneyMarketSecuredMarketStatisticalReportV02(Box<MoneyMarketSecuredMarketStatisticalReportV02>),

	#[validate]
	#[serde(rename = "MnyMktUscrdMktSttstclRpt")]
	#[cfg(feature = "auth")]
	MoneyMarketUnsecuredMarketStatisticalReportV02(Box<MoneyMarketUnsecuredMarketStatisticalReportV02>),

	#[validate]
	#[serde(rename = "MnyMktFXSwpsSttstclRpt")]
	#[cfg(feature = "auth")]
	MoneyMarketForeignExchangeSwapsStatisticalReportV02(Box<MoneyMarketForeignExchangeSwapsStatisticalReportV02>),

	#[validate]
	#[serde(rename = "MnyMktOvrnghtIndxSwpsSttstclRpt")]
	#[cfg(feature = "auth")]
	MoneyMarketOvernightIndexSwapsStatisticalReportV02(Box<MoneyMarketOvernightIndexSwapsStatisticalReportV02>),

	#[validate]
	#[serde(rename = "FinInstrmRptgTxRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingTransactionReportV03(Box<FinancialInstrumentReportingTransactionReportV03>),

	#[validate]
	#[serde(rename = "FinInstrmRptgRefDataRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingReferenceDataReportV02(Box<FinancialInstrumentReportingReferenceDataReportV02>),

	#[validate]
	#[serde(rename = "CtrctRegnReq")]
	#[cfg(feature = "auth")]
	ContractRegistrationRequestV04(Box<ContractRegistrationRequestV04>),

	#[validate]
	#[serde(rename = "CtrctRegnConf")]
	#[cfg(feature = "auth")]
	ContractRegistrationConfirmationV04(Box<ContractRegistrationConfirmationV04>),

	#[validate]
	#[serde(rename = "CtrctRegnClsrReq")]
	#[cfg(feature = "auth")]
	ContractRegistrationClosureRequestV04(Box<ContractRegistrationClosureRequestV04>),

	#[validate]
	#[serde(rename = "CtrctRegnAmdmntReq")]
	#[cfg(feature = "auth")]
	ContractRegistrationAmendmentRequestV04(Box<ContractRegistrationAmendmentRequestV04>),

	#[validate]
	#[serde(rename = "CtrctRegnStmt")]
	#[cfg(feature = "auth")]
	ContractRegistrationStatementV04(Box<ContractRegistrationStatementV04>),

	#[validate]
	#[serde(rename = "CtrctRegnStmtReq")]
	#[cfg(feature = "auth")]
	ContractRegistrationStatementRequestV04(Box<ContractRegistrationStatementRequestV04>),

	#[validate]
	#[serde(rename = "PmtRgltryInfNtfctn")]
	#[cfg(feature = "auth")]
	PaymentRegulatoryInformationNotificationV04(Box<PaymentRegulatoryInformationNotificationV04>),

	#[validate]
	#[serde(rename = "CcyCtrlSpprtgDocDlvry")]
	#[cfg(feature = "auth")]
	CurrencyControlSupportingDocumentDeliveryV04(Box<CurrencyControlSupportingDocumentDeliveryV04>),

	#[validate]
	#[serde(rename = "CcyCtrlReqOrLttr")]
	#[cfg(feature = "auth")]
	CurrencyControlRequestOrLetterV04(Box<CurrencyControlRequestOrLetterV04>),

	#[validate]
	#[serde(rename = "CcyCtrlStsAdvc")]
	#[cfg(feature = "auth")]
	CurrencyControlStatusAdviceV04(Box<CurrencyControlStatusAdviceV04>),

	#[validate]
	#[serde(rename = "MnyMktSttstclRptStsAdvc")]
	#[cfg(feature = "auth")]
	MoneyMarketStatisticalReportStatusAdviceV01(Box<MoneyMarketStatisticalReportStatusAdviceV01>),

	#[validate]
	#[serde(rename = "DerivsTradRptQry")]
	#[cfg(feature = "auth")]
	DerivativesTradeReportQueryV05(Box<DerivativesTradeReportQueryV05>),

	#[validate]
	#[serde(rename = "DerivsTradRpt")]
	#[cfg(feature = "auth")]
	DerivativesTradeReportV04(Box<DerivativesTradeReportV04>),

	#[validate]
	#[serde(rename = "FinInstrmRptgStsAdvc")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingStatusAdviceV01(Box<FinancialInstrumentReportingStatusAdviceV01>),

	#[validate]
	#[serde(rename = "FinInstrmRptgEqtyTrnsprncyDataRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingEquityTransparencyDataReportV01(Box<FinancialInstrumentReportingEquityTransparencyDataReportV01>),

	#[validate]
	#[serde(rename = "FinInstrmRptgNonEqtyTrnsprncyDataRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingNonEquityTransparencyDataReportV03(Box<FinancialInstrumentReportingNonEquityTransparencyDataReportV03>),

	#[validate]
	#[serde(rename = "InvcTaxRpt")]
	#[cfg(feature = "auth")]
	InvoiceTaxReportV01(Box<InvoiceTaxReportV01>),

	#[validate]
	#[serde(rename = "FinInstrmRptgTradgVolCapDataRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingTradingVolumeCapDataReportV01(Box<FinancialInstrumentReportingTradingVolumeCapDataReportV01>),

	#[validate]
	#[serde(rename = "FinInstrmRptgRefDataDltaRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingReferenceDataDeltaReportV03(Box<FinancialInstrumentReportingReferenceDataDeltaReportV03>),

	#[validate]
	#[serde(rename = "InvcTaxRptStsAdvc")]
	#[cfg(feature = "auth")]
	InvoiceTaxReportStatusAdviceV01(Box<InvoiceTaxReportStatusAdviceV01>),

	#[validate]
	#[serde(rename = "FinInstrmRptgNonWorkgDayRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingNonWorkingDayReportV01(Box<FinancialInstrumentReportingNonWorkingDayReportV01>),

	#[validate]
	#[serde(rename = "FinInstrmRptgEqtyTradgActvtyRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingEquityTradingActivityReportV01(Box<FinancialInstrumentReportingEquityTradingActivityReportV01>),

	#[validate]
	#[serde(rename = "FinInstrmRptgNonEqtyTradgActvtyRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingNonEquityTradingActivityReportV01(Box<FinancialInstrumentReportingNonEquityTradingActivityReportV01>),

	#[validate]
	#[serde(rename = "FinInstrmRptgInvldRefDataRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingInvalidReferenceDataReportV02(Box<FinancialInstrumentReportingInvalidReferenceDataReportV02>),

	#[validate]
	#[serde(rename = "FinInstrmRptgRefDataIndxRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingReferenceDataIndexReportV01(Box<FinancialInstrumentReportingReferenceDataIndexReportV01>),

	#[validate]
	#[serde(rename = "FinInstrmRptgEqtyTradgActvtyRslt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingEquityTradingActivityResultV02(Box<FinancialInstrumentReportingEquityTradingActivityResultV02>),

	#[validate]
	#[serde(rename = "FinInstrmRptgNonEqtyTradgActvtyRslt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingNonEquityTradingActivityResultV03(Box<FinancialInstrumentReportingNonEquityTradingActivityResultV03>),

	#[validate]
	#[serde(rename = "FinInstrmRptgCtryCdRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingCountryCodeReportV01(Box<FinancialInstrumentReportingCountryCodeReportV01>),

	#[validate]
	#[serde(rename = "FinInstrmRptgCcyCdRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingCurrencyCodeReportV01(Box<FinancialInstrumentReportingCurrencyCodeReportV01>),

	#[validate]
	#[serde(rename = "FinInstrmRptgMktIdCdRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingMarketIdentificationCodeReportV02(Box<FinancialInstrumentReportingMarketIdentificationCodeReportV02>),

	#[validate]
	#[serde(rename = "FinInstrmRptgInstrmClssfctnRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingInstrumentClassificationReportV01(Box<FinancialInstrumentReportingInstrumentClassificationReportV01>),

	#[validate]
	#[serde(rename = "SctiesFincgRptgTxRpt")]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingTransactionReportV02(Box<SecuritiesFinancingReportingTransactionReportV02>),

	#[validate]
	#[serde(rename = "FinInstrmRptgTradgVolCapRsltRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingTradingVolumeCapResultReportV01(Box<FinancialInstrumentReportingTradingVolumeCapResultReportV01>),

	#[validate]
	#[serde(rename = "CCPClrMmbRpt")]
	#[cfg(feature = "auth")]
	CCPClearingMemberReportV01(Box<CCPClearingMemberReportV01>),

	#[validate]
	#[serde(rename = "CCPMmbRqrmntsRpt")]
	#[cfg(feature = "auth")]
	CCPMemberRequirementsReportV01(Box<CCPMemberRequirementsReportV01>),

	#[validate]
	#[serde(rename = "CCPMmbOblgtnsRpt")]
	#[cfg(feature = "auth")]
	CCPMemberObligationsReportV01(Box<CCPMemberObligationsReportV01>),

	#[validate]
	#[serde(rename = "CCPPrtflStrssTstgDefRpt")]
	#[cfg(feature = "auth")]
	CCPPortfolioStressTestingDefinitionReportV02(Box<CCPPortfolioStressTestingDefinitionReportV02>),

	#[validate]
	#[serde(rename = "CCPPrtflStrssTstgRsltRpt")]
	#[cfg(feature = "auth")]
	CCPPortfolioStressTestingResultReportV01(Box<CCPPortfolioStressTestingResultReportV01>),

	#[validate]
	#[serde(rename = "CCPIncmStmtAndCptlAdqcyRpt")]
	#[cfg(feature = "auth")]
	CCPIncomeStatementAndCapitalAdequacyReportV01(Box<CCPIncomeStatementAndCapitalAdequacyReportV01>),

	#[validate]
	#[serde(rename = "CCPDalyCshFlowsRpt")]
	#[cfg(feature = "auth")]
	CCPDailyCashFlowsReportV02(Box<CCPDailyCashFlowsReportV02>),

	#[validate]
	#[serde(rename = "CCPInvstmtsRpt")]
	#[cfg(feature = "auth")]
	CCPInvestmentsReportV01(Box<CCPInvestmentsReportV01>),

	#[validate]
	#[serde(rename = "CCPLqdtyStrssTstgDefRpt")]
	#[cfg(feature = "auth")]
	CCPLiquidityStressTestingDefinitionReportV01(Box<CCPLiquidityStressTestingDefinitionReportV01>),

	#[validate]
	#[serde(rename = "CCPLqdtyStrssTstgRsltRpt")]
	#[cfg(feature = "auth")]
	CCPLiquidityStressTestingResultReportV01(Box<CCPLiquidityStressTestingResultReportV01>),

	#[validate]
	#[serde(rename = "CCPAvlblFinRsrcsRpt")]
	#[cfg(feature = "auth")]
	CCPAvailableFinancialResourcesReportV01(Box<CCPAvailableFinancialResourcesReportV01>),

	#[validate]
	#[serde(rename = "CCPBckTstgDefRpt")]
	#[cfg(feature = "auth")]
	CCPBackTestingDefinitionReportV01(Box<CCPBackTestingDefinitionReportV01>),

	#[validate]
	#[serde(rename = "CCPBckTstgRsltRpt")]
	#[cfg(feature = "auth")]
	CCPBackTestingResultReportV01(Box<CCPBackTestingResultReportV01>),

	#[validate]
	#[serde(rename = "CCPCollRpt")]
	#[cfg(feature = "auth")]
	CCPCollateralReportV01(Box<CCPCollateralReportV01>),

	#[validate]
	#[serde(rename = "CCPAcctPosRpt")]
	#[cfg(feature = "auth")]
	CCPAccountPositionReportV01(Box<CCPAccountPositionReportV01>),

	#[validate]
	#[serde(rename = "CCPClrdPdctRpt")]
	#[cfg(feature = "auth")]
	CCPClearedProductReportV01(Box<CCPClearedProductReportV01>),

	#[validate]
	#[serde(rename = "SctiesFincgRptgTxMrgnDataRpt")]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingTransactionMarginDataReportV02(Box<SecuritiesFinancingReportingTransactionMarginDataReportV02>),

	#[validate]
	#[serde(rename = "SctiesFincgRptgTxReusdCollDataRpt")]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingTransactionReusedCollateralDataReportV02(Box<SecuritiesFinancingReportingTransactionReusedCollateralDataReportV02>),

	#[validate]
	#[serde(rename = "SttlmIntlrRpt")]
	#[cfg(feature = "auth")]
	SettlementInternaliserReportV01(Box<SettlementInternaliserReportV01>),

	#[validate]
	#[serde(rename = "FinSprvsdPtyIdntyRpt")]
	#[cfg(feature = "auth")]
	FinancialSupervisedPartyIdentityReportV01(Box<FinancialSupervisedPartyIdentityReportV01>),

	#[validate]
	#[serde(rename = "FinBchmkRpt")]
	#[cfg(feature = "auth")]
	FinancialBenchmarkReportV01(Box<FinancialBenchmarkReportV01>),

	#[validate]
	#[serde(rename = "SctiesFincgRptgPairgReq")]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingPairingRequestV02(Box<SecuritiesFinancingReportingPairingRequestV02>),

	#[validate]
	#[serde(rename = "SctiesFincgRptgTxStatRpt")]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingTransactionStateReportV02(Box<SecuritiesFinancingReportingTransactionStateReportV02>),

	#[validate]
	#[serde(rename = "SctiesFincgRptgRcncltnStsAdvc")]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingReconciliationStatusAdviceV02(Box<SecuritiesFinancingReportingReconciliationStatusAdviceV02>),

	#[validate]
	#[serde(rename = "SctiesFincgRptgMssngCollReq")]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingMissingCollateralRequestV02(Box<SecuritiesFinancingReportingMissingCollateralRequestV02>),

	#[validate]
	#[serde(rename = "SctiesFincgRptgTxStsAdvc")]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingTransactionStatusAdviceV02(Box<SecuritiesFinancingReportingTransactionStatusAdviceV02>),

	#[validate]
	#[serde(rename = "SctiesFincgRptgMrgnDataTxStatRpt")]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingMarginDataTransactionStateReportV02(Box<SecuritiesFinancingReportingMarginDataTransactionStateReportV02>),

	#[validate]
	#[serde(rename = "SctiesFincgRptgReusdCollDataTxStatRpt")]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingReusedCollateralDataTransactionStateReportV02(Box<SecuritiesFinancingReportingReusedCollateralDataTransactionStateReportV02>),

	#[validate]
	#[serde(rename = "DerivsTradPosSetRpt")]
	#[cfg(feature = "auth")]
	DerivativesTradePositionSetReportV02(Box<DerivativesTradePositionSetReportV02>),

	#[validate]
	#[serde(rename = "DerivsTradRcncltnSttstclRpt")]
	#[cfg(feature = "auth")]
	DerivativesTradeReconciliationStatisticalReportV03(Box<DerivativesTradeReconciliationStatisticalReportV03>),

	#[validate]
	#[serde(rename = "DerivsTradRjctnSttstclRpt")]
	#[cfg(feature = "auth")]
	DerivativesTradeRejectionStatisticalReportV04(Box<DerivativesTradeRejectionStatisticalReportV04>),

	#[validate]
	#[serde(rename = "SctiesFincgRptgTxQry")]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingTransactionQueryV02(Box<SecuritiesFinancingReportingTransactionQueryV02>),

	#[validate]
	#[serde(rename = "SttlmFlsMnthlyRpt")]
	#[cfg(feature = "auth")]
	SettlementFailsMonthlyReportV01(Box<SettlementFailsMonthlyReportV01>),

	#[validate]
	#[serde(rename = "SttlmFlsAnlRpt")]
	#[cfg(feature = "auth")]
	SettlementFailsAnnualReportV01(Box<SettlementFailsAnnualReportV01>),

	#[validate]
	#[serde(rename = "FinInstrmRptgCxlRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingCancellationReportV01(Box<FinancialInstrumentReportingCancellationReportV01>),

	#[validate]
	#[serde(rename = "SctiesFincgRptgPosSetRpt")]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingPositionSetReportV01(Box<SecuritiesFinancingReportingPositionSetReportV01>),

	#[validate]
	#[serde(rename = "DerivsTradWrnngsRpt")]
	#[cfg(feature = "auth")]
	DerivativesTradeWarningsReportV01(Box<DerivativesTradeWarningsReportV01>),

	#[validate]
	#[serde(rename = "DerivsTradStatRpt")]
	#[cfg(feature = "auth")]
	DerivativesTradeStateReportV02(Box<DerivativesTradeStateReportV02>),

	#[validate]
	#[serde(rename = "DerivsTradMrgnDataRpt")]
	#[cfg(feature = "auth")]
	DerivativesTradeMarginDataReportV02(Box<DerivativesTradeMarginDataReportV02>),

	#[validate]
	#[serde(rename = "DerivsTradMrgnDataTxStatRpt")]
	#[cfg(feature = "auth")]
	DerivativesTradeMarginDataTransactionStateReportV02(Box<DerivativesTradeMarginDataTransactionStateReportV02>),

	#[validate]
	#[serde(rename = "CCPIntrprbltyRpt")]
	#[cfg(feature = "auth")]
	CCPInteroperabilityReportV01(Box<CCPInteroperabilityReportV01>),

	#[validate]
	#[serde(rename = "OrdrBookRpt")]
	#[cfg(feature = "auth")]
	OrderBookReportV01(Box<OrderBookReportV01>),
}
