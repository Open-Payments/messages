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


pub mod camt_003_001_08;
pub mod camt_004_001_10;
pub mod camt_005_001_11;
pub mod camt_006_001_11;
pub mod camt_007_001_10;
pub mod camt_008_001_11;
pub mod camt_009_001_08;
pub mod camt_010_001_09;
pub mod camt_011_001_08;
pub mod camt_012_001_08;
pub mod camt_013_001_04;
pub mod camt_014_001_05;
pub mod camt_015_001_04;
pub mod camt_016_001_04;
pub mod camt_017_001_05;
pub mod camt_018_001_05;
pub mod camt_019_001_07;
pub mod camt_020_001_04;
pub mod camt_021_001_06;
pub mod camt_023_001_07;
pub mod camt_024_001_08;
pub mod camt_025_001_08;
pub mod camt_026_001_10;
pub mod camt_027_001_10;
pub mod camt_028_001_12;
pub mod camt_029_001_13;
pub mod camt_030_001_06;
pub mod camt_031_001_07;
pub mod camt_032_001_05;
pub mod camt_033_001_07;
pub mod camt_034_001_07;
pub mod camt_035_001_06;
pub mod camt_036_001_06;
pub mod camt_037_001_10;
pub mod camt_038_001_05;
pub mod camt_039_001_06;
pub mod camt_040_001_04;
pub mod camt_041_001_04;
pub mod camt_042_001_04;
pub mod camt_043_001_04;
pub mod camt_044_001_03;
pub mod camt_045_001_03;
pub mod camt_046_001_08;
pub mod camt_047_001_08;
pub mod camt_048_001_07;
pub mod camt_049_001_07;
pub mod camt_050_001_07;
pub mod camt_051_001_07;
pub mod camt_052_001_12;
pub mod camt_053_001_12;
pub mod camt_054_001_12;
pub mod camt_055_001_12;
pub mod camt_056_001_11;
pub mod camt_057_001_08;
pub mod camt_058_001_09;
pub mod camt_059_001_08;
pub mod camt_060_001_07;
pub mod camt_061_001_02;
pub mod camt_062_001_03;
pub mod camt_063_001_02;
pub mod camt_066_001_02;
pub mod camt_067_001_02;
pub mod camt_068_001_02;
pub mod camt_069_001_05;
pub mod camt_070_001_06;
pub mod camt_071_001_05;
pub mod camt_072_001_02;
pub mod camt_073_001_02;
pub mod camt_074_001_02;
pub mod camt_075_001_02;
pub mod camt_078_001_02;
pub mod camt_079_001_02;
pub mod camt_080_001_02;
pub mod camt_081_001_02;
pub mod camt_082_001_02;
pub mod camt_083_001_02;
pub mod camt_084_001_02;
pub mod camt_085_001_02;
pub mod camt_086_001_05;
pub mod camt_087_001_09;
pub mod camt_088_001_02;
pub mod camt_101_001_02;
pub mod camt_102_001_03;
pub mod camt_103_001_03;
pub mod camt_104_001_01;
pub mod camt_105_001_02;
pub mod camt_106_001_02;
pub mod camt_107_001_02;
pub mod camt_108_001_02;
pub mod camt_109_001_02;
pub mod camt_110_001_01;
pub mod camt_111_001_01;

pub mod validationerror;
