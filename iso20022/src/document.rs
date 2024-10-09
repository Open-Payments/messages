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


#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum Document {
	#[serde(rename = "admi.002.001.01")]
	#[cfg(feature = "admi")]
	Admi00200101(Box<Admi00200101>),

	#[serde(rename = "SysEvtNtfctn")]
	#[cfg(feature = "admi")]
	SystemEventNotificationV02(Box<SystemEventNotificationV02>),

	#[serde(rename = "RptQryReq")]
	#[cfg(feature = "admi")]
	ReportQueryRequestV02(Box<ReportQueryRequestV02>),

	#[serde(rename = "RsndReq")]
	#[cfg(feature = "admi")]
	ResendRequestV01(Box<ResendRequestV01>),

	#[serde(rename = "RctAck")]
	#[cfg(feature = "admi")]
	ReceiptAcknowledgementV01(Box<ReceiptAcknowledgementV01>),

	#[serde(rename = "StatcDataReq")]
	#[cfg(feature = "admi")]
	StaticDataRequestV02(Box<StaticDataRequestV02>),

	#[serde(rename = "StatcDataRpt")]
	#[cfg(feature = "admi")]
	StaticDataReportV02(Box<StaticDataReportV02>),

	#[serde(rename = "SysEvtAck")]
	#[cfg(feature = "admi")]
	SystemEventAcknowledgementV01(Box<SystemEventAcknowledgementV01>),

	#[serde(rename = "PrcgReq")]
	#[cfg(feature = "admi")]
	ProcessingRequestV02(Box<ProcessingRequestV02>),

	#[serde(rename = "NtfctnOfCrspdc")]
	#[cfg(feature = "admi")]
	NotificationOfCorrespondenceV01(Box<NotificationOfCorrespondenceV01>),

	#[serde(rename = "FIToFIPmtStsRpt")]
	#[cfg(feature = "pacs")]
	FIToFIPaymentStatusReportV12(Box<FIToFIPaymentStatusReportV12>),

	#[serde(rename = "FIToFIPmtStsRpt")]
	#[cfg(feature = "pacs")]
	FIToFIPaymentStatusReportV14(Box<FIToFIPaymentStatusReportV14>),

	#[serde(rename = "FIToFICstmrDrctDbt")]
	#[cfg(feature = "pacs")]
	FIToFICustomerDirectDebitV11(Box<FIToFICustomerDirectDebitV11>),

	#[serde(rename = "PmtRtr")]
	#[cfg(feature = "pacs")]
	PaymentReturnV13(Box<PaymentReturnV13>),

	#[serde(rename = "FIToFIPmtRvsl")]
	#[cfg(feature = "pacs")]
	FIToFIPaymentReversalV13(Box<FIToFIPaymentReversalV13>),

	#[serde(rename = "FIToFICstmrCdtTrf")]
	#[cfg(feature = "pacs")]
	FIToFICustomerCreditTransferV12(Box<FIToFICustomerCreditTransferV12>),

	#[serde(rename = "FICdtTrf")]
	#[cfg(feature = "pacs")]
	FinancialInstitutionCreditTransferV11(Box<FinancialInstitutionCreditTransferV11>),

	#[serde(rename = "FIDrctDbt")]
	#[cfg(feature = "pacs")]
	FinancialInstitutionDirectDebitV06(Box<FinancialInstitutionDirectDebitV06>),

	#[serde(rename = "FIToFIPmtStsReq")]
	#[cfg(feature = "pacs")]
	FIToFIPaymentStatusRequestV06(Box<FIToFIPaymentStatusRequestV06>),

	#[serde(rename = "MulSttlmReq")]
	#[cfg(feature = "pacs")]
	MultilateralSettlementRequestV02(Box<MultilateralSettlementRequestV02>),

	#[serde(rename = "CstmrCdtTrfInitn")]
	#[cfg(feature = "pain")]
	CustomerCreditTransferInitiationV12(Box<CustomerCreditTransferInitiationV12>),

	#[serde(rename = "CstmrPmtStsRpt")]
	#[cfg(feature = "pain")]
	CustomerPaymentStatusReportV14(Box<CustomerPaymentStatusReportV14>),

	#[serde(rename = "CstmrPmtRvsl")]
	#[cfg(feature = "pain")]
	CustomerPaymentReversalV12(Box<CustomerPaymentReversalV12>),

	#[serde(rename = "CstmrDrctDbtInitn")]
	#[cfg(feature = "pain")]
	CustomerDirectDebitInitiationV11(Box<CustomerDirectDebitInitiationV11>),

	#[serde(rename = "MndtInitnReq")]
	#[cfg(feature = "pain")]
	MandateInitiationRequestV08(Box<MandateInitiationRequestV08>),

	#[serde(rename = "MndtAmdmntReq")]
	#[cfg(feature = "pain")]
	MandateAmendmentRequestV08(Box<MandateAmendmentRequestV08>),

	#[serde(rename = "MndtCxlReq")]
	#[cfg(feature = "pain")]
	MandateCancellationRequestV08(Box<MandateCancellationRequestV08>),

	#[serde(rename = "MndtAccptncRpt")]
	#[cfg(feature = "pain")]
	MandateAcceptanceReportV08(Box<MandateAcceptanceReportV08>),

	#[serde(rename = "CdtrPmtActvtnReq")]
	#[cfg(feature = "pain")]
	CreditorPaymentActivationRequestV11(Box<CreditorPaymentActivationRequestV11>),

	#[serde(rename = "CdtrPmtActvtnReqStsRpt")]
	#[cfg(feature = "pain")]
	CreditorPaymentActivationRequestStatusReportV11(Box<CreditorPaymentActivationRequestStatusReportV11>),

	#[serde(rename = "MndtCpyReq")]
	#[cfg(feature = "pain")]
	MandateCopyRequestV04(Box<MandateCopyRequestV04>),

	#[serde(rename = "MndtSspnsnReq")]
	#[cfg(feature = "pain")]
	MandateSuspensionRequestV04(Box<MandateSuspensionRequestV04>),

	#[serde(rename = "AcctOpngInstr")]
	#[cfg(feature = "acmt")]
	AccountOpeningInstructionV08(Box<AccountOpeningInstructionV08>),

	#[serde(rename = "AcctDtlsConf")]
	#[cfg(feature = "acmt")]
	AccountDetailsConfirmationV08(Box<AccountDetailsConfirmationV08>),

	#[serde(rename = "AcctModInstr")]
	#[cfg(feature = "acmt")]
	AccountModificationInstructionV08(Box<AccountModificationInstructionV08>),

	#[serde(rename = "ReqForAcctMgmtStsRpt")]
	#[cfg(feature = "acmt")]
	RequestForAccountManagementStatusReportV06(Box<RequestForAccountManagementStatusReportV06>),

	#[serde(rename = "AcctMgmtStsRpt")]
	#[cfg(feature = "acmt")]
	AccountManagementStatusReportV07(Box<AccountManagementStatusReportV07>),

	#[serde(rename = "AcctOpngReq")]
	#[cfg(feature = "acmt")]
	AccountOpeningRequestV05(Box<AccountOpeningRequestV05>),

	#[serde(rename = "AcctOpngAmdmntReq")]
	#[cfg(feature = "acmt")]
	AccountOpeningAmendmentRequestV05(Box<AccountOpeningAmendmentRequestV05>),

	#[serde(rename = "AcctOpngAddtlInfReq")]
	#[cfg(feature = "acmt")]
	AccountOpeningAdditionalInformationRequestV04(Box<AccountOpeningAdditionalInformationRequestV04>),

	#[serde(rename = "AcctReqAck")]
	#[cfg(feature = "acmt")]
	AccountRequestAcknowledgementV04(Box<AccountRequestAcknowledgementV04>),

	#[serde(rename = "AcctReqRjctn")]
	#[cfg(feature = "acmt")]
	AccountRequestRejectionV04(Box<AccountRequestRejectionV04>),

	#[serde(rename = "AcctAddtlInfReq")]
	#[cfg(feature = "acmt")]
	AccountAdditionalInformationRequestV04(Box<AccountAdditionalInformationRequestV04>),

	#[serde(rename = "AcctRptReq")]
	#[cfg(feature = "acmt")]
	AccountReportRequestV04(Box<AccountReportRequestV04>),

	#[serde(rename = "AcctRpt")]
	#[cfg(feature = "acmt")]
	AccountReportV05(Box<AccountReportV05>),

	#[serde(rename = "AcctExcldMndtMntncReq")]
	#[cfg(feature = "acmt")]
	AccountExcludedMandateMaintenanceRequestV04(Box<AccountExcludedMandateMaintenanceRequestV04>),

	#[serde(rename = "AcctExcldMndtMntncAmdmntReq")]
	#[cfg(feature = "acmt")]
	AccountExcludedMandateMaintenanceAmendmentRequestV04(Box<AccountExcludedMandateMaintenanceAmendmentRequestV04>),

	#[serde(rename = "AcctMndtMntncReq")]
	#[cfg(feature = "acmt")]
	AccountMandateMaintenanceRequestV04(Box<AccountMandateMaintenanceRequestV04>),

	#[serde(rename = "AcctMndtMntncAmdmntReq")]
	#[cfg(feature = "acmt")]
	AccountMandateMaintenanceAmendmentRequestV04(Box<AccountMandateMaintenanceAmendmentRequestV04>),

	#[serde(rename = "AcctClsgReq")]
	#[cfg(feature = "acmt")]
	AccountClosingRequestV04(Box<AccountClosingRequestV04>),

	#[serde(rename = "AcctClsgAmdmntReq")]
	#[cfg(feature = "acmt")]
	AccountClosingAmendmentRequestV04(Box<AccountClosingAmendmentRequestV04>),

	#[serde(rename = "AcctClsgAddtlInfReq")]
	#[cfg(feature = "acmt")]
	AccountClosingAdditionalInformationRequestV04(Box<AccountClosingAdditionalInformationRequestV04>),

	#[serde(rename = "IdModAdvc")]
	#[cfg(feature = "acmt")]
	IdentificationModificationAdviceV04(Box<IdentificationModificationAdviceV04>),

	#[serde(rename = "IdVrfctnReq")]
	#[cfg(feature = "acmt")]
	IdentificationVerificationRequestV04(Box<IdentificationVerificationRequestV04>),

	#[serde(rename = "IdVrfctnRpt")]
	#[cfg(feature = "acmt")]
	IdentificationVerificationReportV04(Box<IdentificationVerificationReportV04>),

	#[serde(rename = "AcctSwtchInfReq")]
	#[cfg(feature = "acmt")]
	AccountSwitchInformationRequestV05(Box<AccountSwitchInformationRequestV05>),

	#[serde(rename = "AcctSwtchInfRspn")]
	#[cfg(feature = "acmt")]
	AccountSwitchInformationResponseV05(Box<AccountSwitchInformationResponseV05>),

	#[serde(rename = "AcctSwtchCclExstgPmt")]
	#[cfg(feature = "acmt")]
	AccountSwitchCancelExistingPaymentV05(Box<AccountSwitchCancelExistingPaymentV05>),

	#[serde(rename = "AcctSwtchReqRdrctn")]
	#[cfg(feature = "acmt")]
	AccountSwitchRequestRedirectionV04(Box<AccountSwitchRequestRedirectionV04>),

	#[serde(rename = "AcctSwtchReqBalTrf")]
	#[cfg(feature = "acmt")]
	AccountSwitchRequestBalanceTransferV05(Box<AccountSwitchRequestBalanceTransferV05>),

	#[serde(rename = "AcctSwtchBalTrfAck")]
	#[cfg(feature = "acmt")]
	AccountSwitchBalanceTransferAcknowledgementV05(Box<AccountSwitchBalanceTransferAcknowledgementV05>),

	#[serde(rename = "AcctSwtchNtfyAcctSwtchCmplt")]
	#[cfg(feature = "acmt")]
	AccountSwitchNotifyAccountSwitchCompleteV02(Box<AccountSwitchNotifyAccountSwitchCompleteV02>),

	#[serde(rename = "AcctSwtchReqPmt")]
	#[cfg(feature = "acmt")]
	AccountSwitchRequestPaymentV05(Box<AccountSwitchRequestPaymentV05>),

	#[serde(rename = "AcctSwtchPmtRspn")]
	#[cfg(feature = "acmt")]
	AccountSwitchPaymentResponseV02(Box<AccountSwitchPaymentResponseV02>),

	#[serde(rename = "AcctSwtchTermntnSwtch")]
	#[cfg(feature = "acmt")]
	AccountSwitchTerminationSwitchV01(Box<AccountSwitchTerminationSwitchV01>),

	#[serde(rename = "AcctSwtchTechRjctn")]
	#[cfg(feature = "acmt")]
	AccountSwitchTechnicalRejectionV02(Box<AccountSwitchTechnicalRejectionV02>),

	#[serde(rename = "PricRpt")]
	#[cfg(feature = "reda")]
	PriceReportV04(Box<PriceReportV04>),

	#[serde(rename = "PricRptCxl")]
	#[cfg(feature = "reda")]
	PriceReportCancellationV04(Box<PriceReportCancellationV04>),

	#[serde(rename = "FndRefDataRpt")]
	#[cfg(feature = "reda")]
	FundReferenceDataReportV07(Box<FundReferenceDataReportV07>),

	#[serde(rename = "InvstmtFndRptReq")]
	#[cfg(feature = "reda")]
	InvestmentFundReportRequestV03(Box<InvestmentFundReportRequestV03>),

	#[serde(rename = "SctyCreReq")]
	#[cfg(feature = "reda")]
	SecurityCreationRequestV01(Box<SecurityCreationRequestV01>),

	#[serde(rename = "SctyMntncReq")]
	#[cfg(feature = "reda")]
	SecurityMaintenanceRequestV01(Box<SecurityMaintenanceRequestV01>),

	#[serde(rename = "SctyCreStsAdvc")]
	#[cfg(feature = "reda")]
	SecurityCreationStatusAdviceV01(Box<SecurityCreationStatusAdviceV01>),

	#[serde(rename = "SctyActvtyAdvc")]
	#[cfg(feature = "reda")]
	SecurityActivityAdviceV01(Box<SecurityActivityAdviceV01>),

	#[serde(rename = "SctyQry")]
	#[cfg(feature = "reda")]
	SecurityQueryV01(Box<SecurityQueryV01>),

	#[serde(rename = "SctyRpt")]
	#[cfg(feature = "reda")]
	SecurityReportV01(Box<SecurityReportV01>),

	#[serde(rename = "SctyDeltnReq")]
	#[cfg(feature = "reda")]
	SecurityDeletionRequestV01(Box<SecurityDeletionRequestV01>),

	#[serde(rename = "PtyCreReq")]
	#[cfg(feature = "reda")]
	PartyCreationRequestV02(Box<PartyCreationRequestV02>),

	#[serde(rename = "PtyQry")]
	#[cfg(feature = "reda")]
	PartyQueryV01(Box<PartyQueryV01>),

	#[serde(rename = "PtyStsAdvc")]
	#[cfg(feature = "reda")]
	PartyStatusAdviceV01(Box<PartyStatusAdviceV01>),

	#[serde(rename = "PtyRpt")]
	#[cfg(feature = "reda")]
	PartyReportV02(Box<PartyReportV02>),

	#[serde(rename = "SctiesAcctCreReq")]
	#[cfg(feature = "reda")]
	SecuritiesAccountCreationRequestV01(Box<SecuritiesAccountCreationRequestV01>),

	#[serde(rename = "SctiesAcctQry")]
	#[cfg(feature = "reda")]
	SecuritiesAccountQueryV01(Box<SecuritiesAccountQueryV01>),

	#[serde(rename = "SctiesAcctStsAdvc")]
	#[cfg(feature = "reda")]
	SecuritiesAccountStatusAdviceV01(Box<SecuritiesAccountStatusAdviceV01>),

	#[serde(rename = "SctiesAcctRpt")]
	#[cfg(feature = "reda")]
	SecuritiesAccountReportV01(Box<SecuritiesAccountReportV01>),

	#[serde(rename = "PtyModReq")]
	#[cfg(feature = "reda")]
	PartyModificationRequestV02(Box<PartyModificationRequestV02>),

	#[serde(rename = "SctiesAcctModReq")]
	#[cfg(feature = "reda")]
	SecuritiesAccountModificationRequestV01(Box<SecuritiesAccountModificationRequestV01>),

	#[serde(rename = "SctyMntncStsAdvc")]
	#[cfg(feature = "reda")]
	SecurityMaintenanceStatusAdviceV01(Box<SecurityMaintenanceStatusAdviceV01>),

	#[serde(rename = "SctyDeltnStsAdvc")]
	#[cfg(feature = "reda")]
	SecurityDeletionStatusAdviceV01(Box<SecurityDeletionStatusAdviceV01>),

	#[serde(rename = "PtyDeltnReq")]
	#[cfg(feature = "reda")]
	PartyDeletionRequestV01(Box<PartyDeletionRequestV01>),

	#[serde(rename = "SctiesAcctDeltnReq")]
	#[cfg(feature = "reda")]
	SecuritiesAccountDeletionRequestV01(Box<SecuritiesAccountDeletionRequestV01>),

	#[serde(rename = "SctiesAudtTrlQry")]
	#[cfg(feature = "reda")]
	SecuritiesAuditTrailQueryV01(Box<SecuritiesAuditTrailQueryV01>),

	#[serde(rename = "SctiesAudtTrlRpt")]
	#[cfg(feature = "reda")]
	SecuritiesAuditTrailReportV01(Box<SecuritiesAuditTrailReportV01>),

	#[serde(rename = "SctiesAcctActvtyAdvc")]
	#[cfg(feature = "reda")]
	SecuritiesAccountActivityAdviceV01(Box<SecuritiesAccountActivityAdviceV01>),

	#[serde(rename = "SctiesAcctAudtTrlQry")]
	#[cfg(feature = "reda")]
	SecuritiesAccountAuditTrailQueryV01(Box<SecuritiesAccountAuditTrailQueryV01>),

	#[serde(rename = "SctiesAcctAudtTrlRpt")]
	#[cfg(feature = "reda")]
	SecuritiesAccountAuditTrailReportV01(Box<SecuritiesAccountAuditTrailReportV01>),

	#[serde(rename = "PtyActvtyAdvc")]
	#[cfg(feature = "reda")]
	PartyActivityAdviceV02(Box<PartyActivityAdviceV02>),

	#[serde(rename = "PtyAudtTrlQry")]
	#[cfg(feature = "reda")]
	PartyAuditTrailQueryV01(Box<PartyAuditTrailQueryV01>),

	#[serde(rename = "PtyAudtTrlRpt")]
	#[cfg(feature = "reda")]
	PartyAuditTrailReportV02(Box<PartyAuditTrailReportV02>),

	#[serde(rename = "StgSttlmInstr")]
	#[cfg(feature = "reda")]
	StandingSettlementInstructionV01(Box<StandingSettlementInstructionV01>),

	#[serde(rename = "StgSttlmInstrDeltn")]
	#[cfg(feature = "reda")]
	StandingSettlementInstructionDeletionV01(Box<StandingSettlementInstructionDeletionV01>),

	#[serde(rename = "StgSttlmInstrStsAdvc")]
	#[cfg(feature = "reda")]
	StandingSettlementInstructionStatusAdviceV01(Box<StandingSettlementInstructionStatusAdviceV01>),

	#[serde(rename = "StgSttlmInstrCxl")]
	#[cfg(feature = "reda")]
	StandingSettlementInstructionCancellationV01(Box<StandingSettlementInstructionCancellationV01>),

	#[serde(rename = "NetgCutOffRefDataUpdReq")]
	#[cfg(feature = "reda")]
	NettingCutOffReferenceDataUpdateRequestV02(Box<NettingCutOffReferenceDataUpdateRequestV02>),

	#[serde(rename = "NetgCutOffRefDataRpt")]
	#[cfg(feature = "reda")]
	NettingCutOffReferenceDataReportV02(Box<NettingCutOffReferenceDataReportV02>),

	#[serde(rename = "CalQry")]
	#[cfg(feature = "reda")]
	CalendarQueryV02(Box<CalendarQueryV02>),

	#[serde(rename = "CalRpt")]
	#[cfg(feature = "reda")]
	CalendarReportV02(Box<CalendarReportV02>),

	#[serde(rename = "ReqToPayCdtrEnrlmntReq")]
	#[cfg(feature = "reda")]
	RequestToPayCreditorEnrolmentRequestV02(Box<RequestToPayCreditorEnrolmentRequestV02>),

	#[serde(rename = "ReqToPayCdtrEnrlmntAmdmntReq")]
	#[cfg(feature = "reda")]
	RequestToPayCreditorEnrolmentAmendmentRequestV02(Box<RequestToPayCreditorEnrolmentAmendmentRequestV02>),

	#[serde(rename = "ReqToPayCdtrEnrlmntCxlReq")]
	#[cfg(feature = "reda")]
	RequestToPayCreditorEnrolmentCancellationRequestV02(Box<RequestToPayCreditorEnrolmentCancellationRequestV02>),

	#[serde(rename = "ReqToPayCdtrEnrlmntStsRpt")]
	#[cfg(feature = "reda")]
	RequestToPayCreditorEnrolmentStatusReportV02(Box<RequestToPayCreditorEnrolmentStatusReportV02>),

	#[serde(rename = "ReqToPayDbtrActvtnReq")]
	#[cfg(feature = "reda")]
	RequestToPayDebtorActivationRequestV02(Box<RequestToPayDebtorActivationRequestV02>),

	#[serde(rename = "ReqToPayDbtrActvtnAmdmntReq")]
	#[cfg(feature = "reda")]
	RequestToPayDebtorActivationAmendmentRequestV02(Box<RequestToPayDebtorActivationAmendmentRequestV02>),

	#[serde(rename = "ReqToPayDbtrActvtnCxlReq")]
	#[cfg(feature = "reda")]
	RequestToPayDebtorActivationCancellationRequestV02(Box<RequestToPayDebtorActivationCancellationRequestV02>),

	#[serde(rename = "ReqToPayDbtrActvtnStsRpt")]
	#[cfg(feature = "reda")]
	RequestToPayDebtorActivationStatusReportV02(Box<RequestToPayDebtorActivationStatusReportV02>),

	#[serde(rename = "RmtAdvc")]
	#[cfg(feature = "remt")]
	RemittanceAdviceV06(Box<RemittanceAdviceV06>),

	#[serde(rename = "RmtLctnAdvc")]
	#[cfg(feature = "remt")]
	RemittanceLocationAdviceV03(Box<RemittanceLocationAdviceV03>),

	#[serde(rename = "GetAcct")]
	#[cfg(feature = "camt")]
	GetAccountV08(Box<GetAccountV08>),

	#[serde(rename = "RtrAcct")]
	#[cfg(feature = "camt")]
	ReturnAccountV10(Box<ReturnAccountV10>),

	#[serde(rename = "GetTx")]
	#[cfg(feature = "camt")]
	GetTransactionV11(Box<GetTransactionV11>),

	#[serde(rename = "RtrTx")]
	#[cfg(feature = "camt")]
	ReturnTransactionV11(Box<ReturnTransactionV11>),

	#[serde(rename = "ModfyTx")]
	#[cfg(feature = "camt")]
	ModifyTransactionV10(Box<ModifyTransactionV10>),

	#[serde(rename = "CclTx")]
	#[cfg(feature = "camt")]
	CancelTransactionV11(Box<CancelTransactionV11>),

	#[serde(rename = "GetLmt")]
	#[cfg(feature = "camt")]
	GetLimitV08(Box<GetLimitV08>),

	#[serde(rename = "RtrLmt")]
	#[cfg(feature = "camt")]
	ReturnLimitV09(Box<ReturnLimitV09>),

	#[serde(rename = "ModfyLmt")]
	#[cfg(feature = "camt")]
	ModifyLimitV08(Box<ModifyLimitV08>),

	#[serde(rename = "DelLmt")]
	#[cfg(feature = "camt")]
	DeleteLimitV08(Box<DeleteLimitV08>),

	#[serde(rename = "GetMmb")]
	#[cfg(feature = "camt")]
	GetMemberV04(Box<GetMemberV04>),

	#[serde(rename = "RtrMmb")]
	#[cfg(feature = "camt")]
	ReturnMemberV05(Box<ReturnMemberV05>),

	#[serde(rename = "ModfyMmb")]
	#[cfg(feature = "camt")]
	ModifyMemberV04(Box<ModifyMemberV04>),

	#[serde(rename = "GetCcyXchgRate")]
	#[cfg(feature = "camt")]
	GetCurrencyExchangeRateV04(Box<GetCurrencyExchangeRateV04>),

	#[serde(rename = "RtrCcyXchgRate")]
	#[cfg(feature = "camt")]
	ReturnCurrencyExchangeRateV05(Box<ReturnCurrencyExchangeRateV05>),

	#[serde(rename = "GetBizDayInf")]
	#[cfg(feature = "camt")]
	GetBusinessDayInformationV05(Box<GetBusinessDayInformationV05>),

	#[serde(rename = "RtrBizDayInf")]
	#[cfg(feature = "camt")]
	ReturnBusinessDayInformationV07(Box<ReturnBusinessDayInformationV07>),

	#[serde(rename = "GetGnlBizInf")]
	#[cfg(feature = "camt")]
	GetGeneralBusinessInformationV04(Box<GetGeneralBusinessInformationV04>),

	#[serde(rename = "RtrGnlBizInf")]
	#[cfg(feature = "camt")]
	ReturnGeneralBusinessInformationV06(Box<ReturnGeneralBusinessInformationV06>),

	#[serde(rename = "BckpPmt")]
	#[cfg(feature = "camt")]
	BackupPaymentV07(Box<BackupPaymentV07>),

	#[serde(rename = "ModfyStgOrdr")]
	#[cfg(feature = "camt")]
	ModifyStandingOrderV08(Box<ModifyStandingOrderV08>),

	#[serde(rename = "Rct")]
	#[cfg(feature = "camt")]
	ReceiptV08(Box<ReceiptV08>),

	#[serde(rename = "UblToApply")]
	#[cfg(feature = "camt")]
	UnableToApplyV10(Box<UnableToApplyV10>),

	#[serde(rename = "ClmNonRct")]
	#[cfg(feature = "camt")]
	ClaimNonReceiptV10(Box<ClaimNonReceiptV10>),

	#[serde(rename = "AddtlPmtInf")]
	#[cfg(feature = "camt")]
	AdditionalPaymentInformationV12(Box<AdditionalPaymentInformationV12>),

	#[serde(rename = "RsltnOfInvstgtn")]
	#[cfg(feature = "camt")]
	ResolutionOfInvestigationV13(Box<ResolutionOfInvestigationV13>),

	#[serde(rename = "NtfctnOfCaseAssgnmt")]
	#[cfg(feature = "camt")]
	NotificationOfCaseAssignmentV06(Box<NotificationOfCaseAssignmentV06>),

	#[serde(rename = "RjctInvstgtn")]
	#[cfg(feature = "camt")]
	RejectInvestigationV07(Box<RejectInvestigationV07>),

	#[serde(rename = "CclCaseAssgnmt")]
	#[cfg(feature = "camt")]
	CancelCaseAssignmentV05(Box<CancelCaseAssignmentV05>),

	#[serde(rename = "ReqForDplct")]
	#[cfg(feature = "camt")]
	RequestForDuplicateV07(Box<RequestForDuplicateV07>),

	#[serde(rename = "Dplct")]
	#[cfg(feature = "camt")]
	DuplicateV07(Box<DuplicateV07>),

	#[serde(rename = "PrtryFrmtInvstgtn")]
	#[cfg(feature = "camt")]
	ProprietaryFormatInvestigationV06(Box<ProprietaryFormatInvestigationV06>),

	#[serde(rename = "DbtAuthstnRspn")]
	#[cfg(feature = "camt")]
	DebitAuthorisationResponseV06(Box<DebitAuthorisationResponseV06>),

	#[serde(rename = "DbtAuthstnReq")]
	#[cfg(feature = "camt")]
	DebitAuthorisationRequestV10(Box<DebitAuthorisationRequestV10>),

	#[serde(rename = "CaseStsRptReq")]
	#[cfg(feature = "camt")]
	CaseStatusReportRequestV05(Box<CaseStatusReportRequestV05>),

	#[serde(rename = "CaseStsRpt")]
	#[cfg(feature = "camt")]
	CaseStatusReportV06(Box<CaseStatusReportV06>),

	#[serde(rename = "FndEstmtdCshFcstRpt")]
	#[cfg(feature = "camt")]
	FundEstimatedCashForecastReportV04(Box<FundEstimatedCashForecastReportV04>),

	#[serde(rename = "FndConfdCshFcstRpt")]
	#[cfg(feature = "camt")]
	FundConfirmedCashForecastReportV04(Box<FundConfirmedCashForecastReportV04>),

	#[serde(rename = "FndDtldEstmtdCshFcstRpt")]
	#[cfg(feature = "camt")]
	FundDetailedEstimatedCashForecastReportV04(Box<FundDetailedEstimatedCashForecastReportV04>),

	#[serde(rename = "FndDtldConfdCshFcstRpt")]
	#[cfg(feature = "camt")]
	FundDetailedConfirmedCashForecastReportV04(Box<FundDetailedConfirmedCashForecastReportV04>),

	#[serde(rename = "FndConfdCshFcstRptCxl")]
	#[cfg(feature = "camt")]
	FundConfirmedCashForecastReportCancellationV03(Box<FundConfirmedCashForecastReportCancellationV03>),

	#[serde(rename = "FndDtldConfdCshFcstRptCxl")]
	#[cfg(feature = "camt")]
	FundDetailedConfirmedCashForecastReportCancellationV03(Box<FundDetailedConfirmedCashForecastReportCancellationV03>),

	#[serde(rename = "GetRsvatn")]
	#[cfg(feature = "camt")]
	GetReservationV08(Box<GetReservationV08>),

	#[serde(rename = "RtrRsvatn")]
	#[cfg(feature = "camt")]
	ReturnReservationV08(Box<ReturnReservationV08>),

	#[serde(rename = "ModfyRsvatn")]
	#[cfg(feature = "camt")]
	ModifyReservationV07(Box<ModifyReservationV07>),

	#[serde(rename = "DelRsvatn")]
	#[cfg(feature = "camt")]
	DeleteReservationV07(Box<DeleteReservationV07>),

	#[serde(rename = "LqdtyCdtTrf")]
	#[cfg(feature = "camt")]
	LiquidityCreditTransferV07(Box<LiquidityCreditTransferV07>),

	#[serde(rename = "LqdtyDbtTrf")]
	#[cfg(feature = "camt")]
	LiquidityDebitTransferV07(Box<LiquidityDebitTransferV07>),

	#[serde(rename = "BkToCstmrAcctRpt")]
	#[cfg(feature = "camt")]
	BankToCustomerAccountReportV12(Box<BankToCustomerAccountReportV12>),

	#[serde(rename = "BkToCstmrStmt")]
	#[cfg(feature = "camt")]
	BankToCustomerStatementV12(Box<BankToCustomerStatementV12>),

	#[serde(rename = "BkToCstmrDbtCdtNtfctn")]
	#[cfg(feature = "camt")]
	BankToCustomerDebitCreditNotificationV12(Box<BankToCustomerDebitCreditNotificationV12>),

	#[serde(rename = "CstmrPmtCxlReq")]
	#[cfg(feature = "camt")]
	CustomerPaymentCancellationRequestV12(Box<CustomerPaymentCancellationRequestV12>),

	#[serde(rename = "FIToFIPmtCxlReq")]
	#[cfg(feature = "camt")]
	FIToFIPaymentCancellationRequestV11(Box<FIToFIPaymentCancellationRequestV11>),

	#[serde(rename = "NtfctnToRcv")]
	#[cfg(feature = "camt")]
	NotificationToReceiveV08(Box<NotificationToReceiveV08>),

	#[serde(rename = "NtfctnToRcvCxlAdvc")]
	#[cfg(feature = "camt")]
	NotificationToReceiveCancellationAdviceV09(Box<NotificationToReceiveCancellationAdviceV09>),

	#[serde(rename = "NtfctnToRcvStsRpt")]
	#[cfg(feature = "camt")]
	NotificationToReceiveStatusReportV08(Box<NotificationToReceiveStatusReportV08>),

	#[serde(rename = "AcctRptgReq")]
	#[cfg(feature = "camt")]
	AccountReportingRequestV07(Box<AccountReportingRequestV07>),

	#[serde(rename = "PayInCall")]
	#[cfg(feature = "camt")]
	PayInCallV02(Box<PayInCallV02>),

	#[serde(rename = "PayInSchdl")]
	#[cfg(feature = "camt")]
	PayInScheduleV03(Box<PayInScheduleV03>),

	#[serde(rename = "PayInEvtAck")]
	#[cfg(feature = "camt")]
	PayInEventAcknowledgementV02(Box<PayInEventAcknowledgementV02>),

	#[serde(rename = "IntraBalMvmntInstr")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementInstructionV02(Box<IntraBalanceMovementInstructionV02>),

	#[serde(rename = "IntraBalMvmntStsAdvc")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementStatusAdviceV02(Box<IntraBalanceMovementStatusAdviceV02>),

	#[serde(rename = "IntraBalMvmntConf")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementConfirmationV02(Box<IntraBalanceMovementConfirmationV02>),

	#[serde(rename = "GetStgOrdr")]
	#[cfg(feature = "camt")]
	GetStandingOrderV05(Box<GetStandingOrderV05>),

	#[serde(rename = "RtrStgOrdr")]
	#[cfg(feature = "camt")]
	ReturnStandingOrderV06(Box<ReturnStandingOrderV06>),

	#[serde(rename = "DelStgOrdr")]
	#[cfg(feature = "camt")]
	DeleteStandingOrderV05(Box<DeleteStandingOrderV05>),

	#[serde(rename = "IntraBalMvmntModReq")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementModificationRequestV02(Box<IntraBalanceMovementModificationRequestV02>),

	#[serde(rename = "IntraBalMvmntModReqStsAdvc")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementModificationRequestStatusAdviceV02(Box<IntraBalanceMovementModificationRequestStatusAdviceV02>),

	#[serde(rename = "IntraBalMvmntCxlReq")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementCancellationRequestV02(Box<IntraBalanceMovementCancellationRequestV02>),

	#[serde(rename = "IntraBalMvmntCxlReqStsAdvc")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementCancellationRequestStatusAdviceV02(Box<IntraBalanceMovementCancellationRequestStatusAdviceV02>),

	#[serde(rename = "IntraBalMvmntQry")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementQueryV02(Box<IntraBalanceMovementQueryV02>),

	#[serde(rename = "IntraBalMvmntQryRspn")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementQueryResponseV02(Box<IntraBalanceMovementQueryResponseV02>),

	#[serde(rename = "IntraBalMvmntModQry")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementModificationQueryV02(Box<IntraBalanceMovementModificationQueryV02>),

	#[serde(rename = "IntraBalMvmntModRpt")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementModificationReportV02(Box<IntraBalanceMovementModificationReportV02>),

	#[serde(rename = "IntraBalMvmntCxlQry")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementCancellationQueryV02(Box<IntraBalanceMovementCancellationQueryV02>),

	#[serde(rename = "IntraBalMvmntCxlRpt")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementCancellationReportV02(Box<IntraBalanceMovementCancellationReportV02>),

	#[serde(rename = "IntraBalMvmntPstngRpt")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementPostingReportV02(Box<IntraBalanceMovementPostingReportV02>),

	#[serde(rename = "IntraBalMvmntPdgRpt")]
	#[cfg(feature = "camt")]
	IntraBalanceMovementPendingReportV02(Box<IntraBalanceMovementPendingReportV02>),

	#[serde(rename = "BkSvcsBllgStmt")]
	#[cfg(feature = "camt")]
	BankServicesBillingStatementV05(Box<BankServicesBillingStatementV05>),

	#[serde(rename = "ReqToModfyPmt")]
	#[cfg(feature = "camt")]
	RequestToModifyPaymentV09(Box<RequestToModifyPaymentV09>),

	#[serde(rename = "NetRpt")]
	#[cfg(feature = "camt")]
	NetReportV02(Box<NetReportV02>),

	#[serde(rename = "CretLmt")]
	#[cfg(feature = "camt")]
	CreateLimitV02(Box<CreateLimitV02>),

	#[serde(rename = "CretStgOrdr")]
	#[cfg(feature = "camt")]
	CreateStandingOrderV03(Box<CreateStandingOrderV03>),

	#[serde(rename = "CretRsvatn")]
	#[cfg(feature = "camt")]
	CreateReservationV03(Box<CreateReservationV03>),

	#[serde(rename = "CretMmb")]
	#[cfg(feature = "camt")]
	CreateMemberV01(Box<CreateMemberV01>),

	#[serde(rename = "ChrgsPmtNtfctn")]
	#[cfg(feature = "camt")]
	ChargesPaymentNotificationV02(Box<ChargesPaymentNotificationV02>),

	#[serde(rename = "ChrgsPmtReq")]
	#[cfg(feature = "camt")]
	ChargesPaymentRequestV02(Box<ChargesPaymentRequestV02>),

	#[serde(rename = "ChqPresntmntNtfctn")]
	#[cfg(feature = "camt")]
	ChequePresentmentNotificationV02(Box<ChequePresentmentNotificationV02>),

	#[serde(rename = "ChqCxlOrStopReq")]
	#[cfg(feature = "camt")]
	ChequeCancellationOrStopRequestV02(Box<ChequeCancellationOrStopRequestV02>),

	#[serde(rename = "ChqCxlOrStopRpt")]
	#[cfg(feature = "camt")]
	ChequeCancellationOrStopReportV02(Box<ChequeCancellationOrStopReportV02>),

	#[serde(rename = "InvstgtnReq")]
	#[cfg(feature = "camt")]
	InvestigationRequestV01(Box<InvestigationRequestV01>),

	#[serde(rename = "InvstgtnRspn")]
	#[cfg(feature = "camt")]
	InvestigationResponseV01(Box<InvestigationResponseV01>),

	#[serde(rename = "InfReqOpng")]
	#[cfg(feature = "auth")]
	InformationRequestOpeningV02(Box<InformationRequestOpeningV02>),

	#[serde(rename = "InfReqRspn")]
	#[cfg(feature = "auth")]
	InformationRequestResponseV02(Box<InformationRequestResponseV02>),

	#[serde(rename = "InfReqStsChngNtfctn")]
	#[cfg(feature = "auth")]
	InformationRequestStatusChangeNotificationV01(Box<InformationRequestStatusChangeNotificationV01>),

	#[serde(rename = "MnyMktScrdMktSttstclRpt")]
	#[cfg(feature = "auth")]
	MoneyMarketSecuredMarketStatisticalReportV02(Box<MoneyMarketSecuredMarketStatisticalReportV02>),

	#[serde(rename = "MnyMktUscrdMktSttstclRpt")]
	#[cfg(feature = "auth")]
	MoneyMarketUnsecuredMarketStatisticalReportV02(Box<MoneyMarketUnsecuredMarketStatisticalReportV02>),

	#[serde(rename = "MnyMktFXSwpsSttstclRpt")]
	#[cfg(feature = "auth")]
	MoneyMarketForeignExchangeSwapsStatisticalReportV02(Box<MoneyMarketForeignExchangeSwapsStatisticalReportV02>),

	#[serde(rename = "MnyMktOvrnghtIndxSwpsSttstclRpt")]
	#[cfg(feature = "auth")]
	MoneyMarketOvernightIndexSwapsStatisticalReportV02(Box<MoneyMarketOvernightIndexSwapsStatisticalReportV02>),

	#[serde(rename = "FinInstrmRptgTxRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingTransactionReportV03(Box<FinancialInstrumentReportingTransactionReportV03>),

	#[serde(rename = "FinInstrmRptgRefDataRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingReferenceDataReportV02(Box<FinancialInstrumentReportingReferenceDataReportV02>),

	#[serde(rename = "CtrctRegnReq")]
	#[cfg(feature = "auth")]
	ContractRegistrationRequestV04(Box<ContractRegistrationRequestV04>),

	#[serde(rename = "CtrctRegnConf")]
	#[cfg(feature = "auth")]
	ContractRegistrationConfirmationV04(Box<ContractRegistrationConfirmationV04>),

	#[serde(rename = "CtrctRegnClsrReq")]
	#[cfg(feature = "auth")]
	ContractRegistrationClosureRequestV04(Box<ContractRegistrationClosureRequestV04>),

	#[serde(rename = "CtrctRegnAmdmntReq")]
	#[cfg(feature = "auth")]
	ContractRegistrationAmendmentRequestV04(Box<ContractRegistrationAmendmentRequestV04>),

	#[serde(rename = "CtrctRegnStmt")]
	#[cfg(feature = "auth")]
	ContractRegistrationStatementV04(Box<ContractRegistrationStatementV04>),

	#[serde(rename = "CtrctRegnStmtReq")]
	#[cfg(feature = "auth")]
	ContractRegistrationStatementRequestV04(Box<ContractRegistrationStatementRequestV04>),

	#[serde(rename = "PmtRgltryInfNtfctn")]
	#[cfg(feature = "auth")]
	PaymentRegulatoryInformationNotificationV04(Box<PaymentRegulatoryInformationNotificationV04>),

	#[serde(rename = "CcyCtrlSpprtgDocDlvry")]
	#[cfg(feature = "auth")]
	CurrencyControlSupportingDocumentDeliveryV04(Box<CurrencyControlSupportingDocumentDeliveryV04>),

	#[serde(rename = "CcyCtrlReqOrLttr")]
	#[cfg(feature = "auth")]
	CurrencyControlRequestOrLetterV04(Box<CurrencyControlRequestOrLetterV04>),

	#[serde(rename = "CcyCtrlStsAdvc")]
	#[cfg(feature = "auth")]
	CurrencyControlStatusAdviceV04(Box<CurrencyControlStatusAdviceV04>),

	#[serde(rename = "MnyMktSttstclRptStsAdvc")]
	#[cfg(feature = "auth")]
	MoneyMarketStatisticalReportStatusAdviceV01(Box<MoneyMarketStatisticalReportStatusAdviceV01>),

	#[serde(rename = "DerivsTradRptQry")]
	#[cfg(feature = "auth")]
	DerivativesTradeReportQueryV05(Box<DerivativesTradeReportQueryV05>),

	#[serde(rename = "DerivsTradRpt")]
	#[cfg(feature = "auth")]
	DerivativesTradeReportV04(Box<DerivativesTradeReportV04>),

	#[serde(rename = "FinInstrmRptgStsAdvc")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingStatusAdviceV01(Box<FinancialInstrumentReportingStatusAdviceV01>),

	#[serde(rename = "FinInstrmRptgEqtyTrnsprncyDataRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingEquityTransparencyDataReportV01(Box<FinancialInstrumentReportingEquityTransparencyDataReportV01>),

	#[serde(rename = "FinInstrmRptgNonEqtyTrnsprncyDataRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingNonEquityTransparencyDataReportV03(Box<FinancialInstrumentReportingNonEquityTransparencyDataReportV03>),

	#[serde(rename = "InvcTaxRpt")]
	#[cfg(feature = "auth")]
	InvoiceTaxReportV01(Box<InvoiceTaxReportV01>),

	#[serde(rename = "FinInstrmRptgTradgVolCapDataRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingTradingVolumeCapDataReportV01(Box<FinancialInstrumentReportingTradingVolumeCapDataReportV01>),

	#[serde(rename = "FinInstrmRptgRefDataDltaRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingReferenceDataDeltaReportV03(Box<FinancialInstrumentReportingReferenceDataDeltaReportV03>),

	#[serde(rename = "InvcTaxRptStsAdvc")]
	#[cfg(feature = "auth")]
	InvoiceTaxReportStatusAdviceV01(Box<InvoiceTaxReportStatusAdviceV01>),

	#[serde(rename = "FinInstrmRptgNonWorkgDayRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingNonWorkingDayReportV01(Box<FinancialInstrumentReportingNonWorkingDayReportV01>),

	#[serde(rename = "FinInstrmRptgEqtyTradgActvtyRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingEquityTradingActivityReportV01(Box<FinancialInstrumentReportingEquityTradingActivityReportV01>),

	#[serde(rename = "FinInstrmRptgNonEqtyTradgActvtyRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingNonEquityTradingActivityReportV01(Box<FinancialInstrumentReportingNonEquityTradingActivityReportV01>),

	#[serde(rename = "FinInstrmRptgInvldRefDataRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingInvalidReferenceDataReportV02(Box<FinancialInstrumentReportingInvalidReferenceDataReportV02>),

	#[serde(rename = "FinInstrmRptgRefDataIndxRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingReferenceDataIndexReportV01(Box<FinancialInstrumentReportingReferenceDataIndexReportV01>),

	#[serde(rename = "FinInstrmRptgEqtyTradgActvtyRslt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingEquityTradingActivityResultV02(Box<FinancialInstrumentReportingEquityTradingActivityResultV02>),

	#[serde(rename = "FinInstrmRptgNonEqtyTradgActvtyRslt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingNonEquityTradingActivityResultV03(Box<FinancialInstrumentReportingNonEquityTradingActivityResultV03>),

	#[serde(rename = "FinInstrmRptgCtryCdRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingCountryCodeReportV01(Box<FinancialInstrumentReportingCountryCodeReportV01>),

	#[serde(rename = "FinInstrmRptgCcyCdRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingCurrencyCodeReportV01(Box<FinancialInstrumentReportingCurrencyCodeReportV01>),

	#[serde(rename = "FinInstrmRptgMktIdCdRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingMarketIdentificationCodeReportV02(Box<FinancialInstrumentReportingMarketIdentificationCodeReportV02>),

	#[serde(rename = "FinInstrmRptgInstrmClssfctnRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingInstrumentClassificationReportV01(Box<FinancialInstrumentReportingInstrumentClassificationReportV01>),

	#[serde(rename = "SctiesFincgRptgTxRpt")]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingTransactionReportV02(Box<SecuritiesFinancingReportingTransactionReportV02>),

	#[serde(rename = "FinInstrmRptgTradgVolCapRsltRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingTradingVolumeCapResultReportV01(Box<FinancialInstrumentReportingTradingVolumeCapResultReportV01>),

	#[serde(rename = "CCPClrMmbRpt")]
	#[cfg(feature = "auth")]
	CCPClearingMemberReportV01(Box<CCPClearingMemberReportV01>),

	#[serde(rename = "CCPMmbRqrmntsRpt")]
	#[cfg(feature = "auth")]
	CCPMemberRequirementsReportV01(Box<CCPMemberRequirementsReportV01>),

	#[serde(rename = "CCPMmbOblgtnsRpt")]
	#[cfg(feature = "auth")]
	CCPMemberObligationsReportV01(Box<CCPMemberObligationsReportV01>),

	#[serde(rename = "CCPPrtflStrssTstgDefRpt")]
	#[cfg(feature = "auth")]
	CCPPortfolioStressTestingDefinitionReportV02(Box<CCPPortfolioStressTestingDefinitionReportV02>),

	#[serde(rename = "CCPPrtflStrssTstgRsltRpt")]
	#[cfg(feature = "auth")]
	CCPPortfolioStressTestingResultReportV01(Box<CCPPortfolioStressTestingResultReportV01>),

	#[serde(rename = "CCPIncmStmtAndCptlAdqcyRpt")]
	#[cfg(feature = "auth")]
	CCPIncomeStatementAndCapitalAdequacyReportV01(Box<CCPIncomeStatementAndCapitalAdequacyReportV01>),

	#[serde(rename = "CCPDalyCshFlowsRpt")]
	#[cfg(feature = "auth")]
	CCPDailyCashFlowsReportV02(Box<CCPDailyCashFlowsReportV02>),

	#[serde(rename = "CCPInvstmtsRpt")]
	#[cfg(feature = "auth")]
	CCPInvestmentsReportV01(Box<CCPInvestmentsReportV01>),

	#[serde(rename = "CCPLqdtyStrssTstgDefRpt")]
	#[cfg(feature = "auth")]
	CCPLiquidityStressTestingDefinitionReportV01(Box<CCPLiquidityStressTestingDefinitionReportV01>),

	#[serde(rename = "CCPLqdtyStrssTstgRsltRpt")]
	#[cfg(feature = "auth")]
	CCPLiquidityStressTestingResultReportV01(Box<CCPLiquidityStressTestingResultReportV01>),

	#[serde(rename = "CCPAvlblFinRsrcsRpt")]
	#[cfg(feature = "auth")]
	CCPAvailableFinancialResourcesReportV01(Box<CCPAvailableFinancialResourcesReportV01>),

	#[serde(rename = "CCPBckTstgDefRpt")]
	#[cfg(feature = "auth")]
	CCPBackTestingDefinitionReportV01(Box<CCPBackTestingDefinitionReportV01>),

	#[serde(rename = "CCPBckTstgRsltRpt")]
	#[cfg(feature = "auth")]
	CCPBackTestingResultReportV01(Box<CCPBackTestingResultReportV01>),

	#[serde(rename = "CCPCollRpt")]
	#[cfg(feature = "auth")]
	CCPCollateralReportV01(Box<CCPCollateralReportV01>),

	#[serde(rename = "CCPAcctPosRpt")]
	#[cfg(feature = "auth")]
	CCPAccountPositionReportV01(Box<CCPAccountPositionReportV01>),

	#[serde(rename = "CCPClrdPdctRpt")]
	#[cfg(feature = "auth")]
	CCPClearedProductReportV01(Box<CCPClearedProductReportV01>),

	#[serde(rename = "SctiesFincgRptgTxMrgnDataRpt")]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingTransactionMarginDataReportV02(Box<SecuritiesFinancingReportingTransactionMarginDataReportV02>),

	#[serde(rename = "SctiesFincgRptgTxReusdCollDataRpt")]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingTransactionReusedCollateralDataReportV02(Box<SecuritiesFinancingReportingTransactionReusedCollateralDataReportV02>),

	#[serde(rename = "SttlmIntlrRpt")]
	#[cfg(feature = "auth")]
	SettlementInternaliserReportV01(Box<SettlementInternaliserReportV01>),

	#[serde(rename = "FinSprvsdPtyIdntyRpt")]
	#[cfg(feature = "auth")]
	FinancialSupervisedPartyIdentityReportV01(Box<FinancialSupervisedPartyIdentityReportV01>),

	#[serde(rename = "FinBchmkRpt")]
	#[cfg(feature = "auth")]
	FinancialBenchmarkReportV01(Box<FinancialBenchmarkReportV01>),

	#[serde(rename = "SctiesFincgRptgPairgReq")]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingPairingRequestV02(Box<SecuritiesFinancingReportingPairingRequestV02>),

	#[serde(rename = "SctiesFincgRptgTxStatRpt")]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingTransactionStateReportV02(Box<SecuritiesFinancingReportingTransactionStateReportV02>),

	#[serde(rename = "SctiesFincgRptgRcncltnStsAdvc")]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingReconciliationStatusAdviceV02(Box<SecuritiesFinancingReportingReconciliationStatusAdviceV02>),

	#[serde(rename = "SctiesFincgRptgMssngCollReq")]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingMissingCollateralRequestV02(Box<SecuritiesFinancingReportingMissingCollateralRequestV02>),

	#[serde(rename = "SctiesFincgRptgTxStsAdvc")]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingTransactionStatusAdviceV02(Box<SecuritiesFinancingReportingTransactionStatusAdviceV02>),

	#[serde(rename = "SctiesFincgRptgMrgnDataTxStatRpt")]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingMarginDataTransactionStateReportV02(Box<SecuritiesFinancingReportingMarginDataTransactionStateReportV02>),

	#[serde(rename = "SctiesFincgRptgReusdCollDataTxStatRpt")]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingReusedCollateralDataTransactionStateReportV02(Box<SecuritiesFinancingReportingReusedCollateralDataTransactionStateReportV02>),

	#[serde(rename = "DerivsTradPosSetRpt")]
	#[cfg(feature = "auth")]
	DerivativesTradePositionSetReportV02(Box<DerivativesTradePositionSetReportV02>),

	#[serde(rename = "DerivsTradRcncltnSttstclRpt")]
	#[cfg(feature = "auth")]
	DerivativesTradeReconciliationStatisticalReportV03(Box<DerivativesTradeReconciliationStatisticalReportV03>),

	#[serde(rename = "DerivsTradRjctnSttstclRpt")]
	#[cfg(feature = "auth")]
	DerivativesTradeRejectionStatisticalReportV04(Box<DerivativesTradeRejectionStatisticalReportV04>),

	#[serde(rename = "SctiesFincgRptgTxQry")]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingTransactionQueryV02(Box<SecuritiesFinancingReportingTransactionQueryV02>),

	#[serde(rename = "SttlmFlsMnthlyRpt")]
	#[cfg(feature = "auth")]
	SettlementFailsMonthlyReportV01(Box<SettlementFailsMonthlyReportV01>),

	#[serde(rename = "SttlmFlsAnlRpt")]
	#[cfg(feature = "auth")]
	SettlementFailsAnnualReportV01(Box<SettlementFailsAnnualReportV01>),

	#[serde(rename = "FinInstrmRptgCxlRpt")]
	#[cfg(feature = "auth")]
	FinancialInstrumentReportingCancellationReportV01(Box<FinancialInstrumentReportingCancellationReportV01>),

	#[serde(rename = "SctiesFincgRptgPosSetRpt")]
	#[cfg(feature = "auth")]
	SecuritiesFinancingReportingPositionSetReportV01(Box<SecuritiesFinancingReportingPositionSetReportV01>),

	#[serde(rename = "DerivsTradWrnngsRpt")]
	#[cfg(feature = "auth")]
	DerivativesTradeWarningsReportV01(Box<DerivativesTradeWarningsReportV01>),

	#[serde(rename = "DerivsTradStatRpt")]
	#[cfg(feature = "auth")]
	DerivativesTradeStateReportV02(Box<DerivativesTradeStateReportV02>),

	#[serde(rename = "DerivsTradMrgnDataRpt")]
	#[cfg(feature = "auth")]
	DerivativesTradeMarginDataReportV02(Box<DerivativesTradeMarginDataReportV02>),

	#[serde(rename = "DerivsTradMrgnDataTxStatRpt")]
	#[cfg(feature = "auth")]
	DerivativesTradeMarginDataTransactionStateReportV02(Box<DerivativesTradeMarginDataTransactionStateReportV02>),

	#[serde(rename = "CCPIntrprbltyRpt")]
	#[cfg(feature = "auth")]
	CCPInteroperabilityReportV01(Box<CCPInteroperabilityReportV01>),

	#[serde(rename = "OrdrBookRpt")]
	#[cfg(feature = "auth")]
	OrderBookReportV01(Box<OrderBookReportV01>),

	#[default]
	UNKNOWN,
}
