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


use crate::message::iso20022::acmt::acmt_001_001_08::*;
use crate::message::iso20022::acmt::acmt_002_001_08::*;
use crate::message::iso20022::acmt::acmt_003_001_08::*;
use crate::message::iso20022::acmt::acmt_005_001_06::*;
use crate::message::iso20022::acmt::acmt_006_001_07::*;
use crate::message::iso20022::acmt::acmt_007_001_05::*;
use crate::message::iso20022::acmt::acmt_008_001_05::*;
use crate::message::iso20022::acmt::acmt_009_001_04::*;
use crate::message::iso20022::acmt::acmt_010_001_04::*;
use crate::message::iso20022::acmt::acmt_011_001_04::*;
use crate::message::iso20022::acmt::acmt_012_001_04::*;
use crate::message::iso20022::acmt::acmt_013_001_04::*;
use crate::message::iso20022::acmt::acmt_014_001_05::*;
use crate::message::iso20022::acmt::acmt_015_001_04::*;
use crate::message::iso20022::acmt::acmt_016_001_04::*;
use crate::message::iso20022::acmt::acmt_017_001_04::*;
use crate::message::iso20022::acmt::acmt_018_001_04::*;
use crate::message::iso20022::acmt::acmt_019_001_04::*;
use crate::message::iso20022::acmt::acmt_020_001_04::*;
use crate::message::iso20022::acmt::acmt_021_001_04::*;
use crate::message::iso20022::acmt::acmt_022_001_04::*;
use crate::message::iso20022::acmt::acmt_023_001_04::*;
use crate::message::iso20022::acmt::acmt_024_001_04::*;
use crate::message::iso20022::acmt::acmt_027_001_05::*;
use crate::message::iso20022::acmt::acmt_028_001_05::*;
use crate::message::iso20022::acmt::acmt_029_001_05::*;
use crate::message::iso20022::acmt::acmt_030_001_04::*;
use crate::message::iso20022::acmt::acmt_031_001_05::*;
use crate::message::iso20022::acmt::acmt_032_001_05::*;
use crate::message::iso20022::acmt::acmt_033_001_02::*;
use crate::message::iso20022::acmt::acmt_034_001_05::*;
use crate::message::iso20022::acmt::acmt_035_001_02::*;
use crate::message::iso20022::acmt::acmt_036_001_01::*;
use crate::message::iso20022::acmt::acmt_037_001_02::*;

use crate::message::iso20022::admi::admi_002_001_01::*;
use crate::message::iso20022::admi::admi_004_001_02::*;
use crate::message::iso20022::admi::admi_005_001_02::*;
use crate::message::iso20022::admi::admi_006_001_01::*;
use crate::message::iso20022::admi::admi_007_001_01::*;
use crate::message::iso20022::admi::admi_009_001_02::*;
use crate::message::iso20022::admi::admi_010_001_02::*;
use crate::message::iso20022::admi::admi_011_001_01::*;
use crate::message::iso20022::admi::admi_017_001_02::*;
use crate::message::iso20022::admi::admi_024_001_01::*;

use crate::message::iso20022::auth::auth_001_001_02::*;
use crate::message::iso20022::auth::auth_002_001_02::*;
use crate::message::iso20022::auth::auth_003_001_01::*;
use crate::message::iso20022::auth::auth_012_001_02::*;
use crate::message::iso20022::auth::auth_013_001_02::*;
use crate::message::iso20022::auth::auth_014_001_02::*;
use crate::message::iso20022::auth::auth_015_001_02::*;
use crate::message::iso20022::auth::auth_016_001_03::*;
use crate::message::iso20022::auth::auth_017_001_02::*;
use crate::message::iso20022::auth::auth_018_001_04::*;
use crate::message::iso20022::auth::auth_019_001_04::*;
use crate::message::iso20022::auth::auth_020_001_04::*;
use crate::message::iso20022::auth::auth_021_001_04::*;
use crate::message::iso20022::auth::auth_022_001_04::*;
use crate::message::iso20022::auth::auth_023_001_04::*;
use crate::message::iso20022::auth::auth_024_001_04::*;
use crate::message::iso20022::auth::auth_025_001_04::*;
use crate::message::iso20022::auth::auth_026_001_04::*;
use crate::message::iso20022::auth::auth_027_001_04::*;
use crate::message::iso20022::auth::auth_028_001_01::*;
use crate::message::iso20022::auth::auth_029_001_05::*;
use crate::message::iso20022::auth::auth_030_001_04::*;
use crate::message::iso20022::auth::auth_031_001_01::*;
use crate::message::iso20022::auth::auth_032_001_01::*;
use crate::message::iso20022::auth::auth_033_001_03::*;
use crate::message::iso20022::auth::auth_034_001_01::*;
use crate::message::iso20022::auth::auth_035_001_01::*;
use crate::message::iso20022::auth::auth_036_001_03::*;
use crate::message::iso20022::auth::auth_038_001_01::*;
use crate::message::iso20022::auth::auth_039_001_01::*;
use crate::message::iso20022::auth::auth_040_001_01::*;
use crate::message::iso20022::auth::auth_041_001_01::*;
use crate::message::iso20022::auth::auth_042_001_02::*;
use crate::message::iso20022::auth::auth_043_001_01::*;
use crate::message::iso20022::auth::auth_044_001_02::*;
use crate::message::iso20022::auth::auth_045_001_03::*;
use crate::message::iso20022::auth::auth_047_001_01::*;
use crate::message::iso20022::auth::auth_048_001_01::*;
use crate::message::iso20022::auth::auth_049_001_02::*;
use crate::message::iso20022::auth::auth_050_001_01::*;
use crate::message::iso20022::auth::auth_052_001_02::*;
use crate::message::iso20022::auth::auth_053_001_01::*;
use crate::message::iso20022::auth::auth_054_001_01::*;
use crate::message::iso20022::auth::auth_055_001_01::*;
use crate::message::iso20022::auth::auth_056_001_01::*;
use crate::message::iso20022::auth::auth_057_001_02::*;
use crate::message::iso20022::auth::auth_058_001_01::*;
use crate::message::iso20022::auth::auth_059_001_01::*;
use crate::message::iso20022::auth::auth_060_001_02::*;
use crate::message::iso20022::auth::auth_061_001_01::*;
use crate::message::iso20022::auth::auth_062_001_01::*;
use crate::message::iso20022::auth::auth_063_001_01::*;
use crate::message::iso20022::auth::auth_064_001_01::*;
use crate::message::iso20022::auth::auth_065_001_01::*;
use crate::message::iso20022::auth::auth_066_001_01::*;
use crate::message::iso20022::auth::auth_067_001_01::*;
use crate::message::iso20022::auth::auth_068_001_01::*;
use crate::message::iso20022::auth::auth_069_001_01::*;
use crate::message::iso20022::auth::auth_070_001_02::*;
use crate::message::iso20022::auth::auth_071_001_02::*;
use crate::message::iso20022::auth::auth_072_001_01::*;
use crate::message::iso20022::auth::auth_076_001_01::*;
use crate::message::iso20022::auth::auth_077_001_01::*;
use crate::message::iso20022::auth::auth_078_001_02::*;
use crate::message::iso20022::auth::auth_079_001_02::*;
use crate::message::iso20022::auth::auth_080_001_02::*;
use crate::message::iso20022::auth::auth_083_001_02::*;
use crate::message::iso20022::auth::auth_084_001_02::*;
use crate::message::iso20022::auth::auth_085_001_02::*;
use crate::message::iso20022::auth::auth_086_001_02::*;
use crate::message::iso20022::auth::auth_090_001_02::*;
use crate::message::iso20022::auth::auth_091_001_03::*;
use crate::message::iso20022::auth::auth_092_001_04::*;
use crate::message::iso20022::auth::auth_094_001_02::*;
use crate::message::iso20022::auth::auth_100_001_01::*;
use crate::message::iso20022::auth::auth_101_001_01::*;
use crate::message::iso20022::auth::auth_102_001_01::*;
use crate::message::iso20022::auth::auth_105_001_01::*;
use crate::message::iso20022::auth::auth_106_001_01::*;
use crate::message::iso20022::auth::auth_107_001_02::*;
use crate::message::iso20022::auth::auth_108_001_02::*;
use crate::message::iso20022::auth::auth_109_001_02::*;
use crate::message::iso20022::auth::auth_112_001_01::*;
use crate::message::iso20022::auth::auth_113_001_01::*;

use crate::message::iso20022::camt::camt_003_001_08::*;
use crate::message::iso20022::camt::camt_004_001_10::*;
use crate::message::iso20022::camt::camt_005_001_11::*;
use crate::message::iso20022::camt::camt_006_001_11::*;
use crate::message::iso20022::camt::camt_007_001_10::*;
use crate::message::iso20022::camt::camt_008_001_11::*;
use crate::message::iso20022::camt::camt_009_001_08::*;
use crate::message::iso20022::camt::camt_010_001_09::*;
use crate::message::iso20022::camt::camt_011_001_08::*;
use crate::message::iso20022::camt::camt_012_001_08::*;
use crate::message::iso20022::camt::camt_013_001_04::*;
use crate::message::iso20022::camt::camt_014_001_05::*;
use crate::message::iso20022::camt::camt_015_001_04::*;
use crate::message::iso20022::camt::camt_016_001_04::*;
use crate::message::iso20022::camt::camt_017_001_05::*;
use crate::message::iso20022::camt::camt_018_001_05::*;
use crate::message::iso20022::camt::camt_019_001_07::*;
use crate::message::iso20022::camt::camt_020_001_04::*;
use crate::message::iso20022::camt::camt_021_001_06::*;
use crate::message::iso20022::camt::camt_023_001_07::*;
use crate::message::iso20022::camt::camt_024_001_08::*;
use crate::message::iso20022::camt::camt_025_001_08::*;
use crate::message::iso20022::camt::camt_026_001_10::*;
use crate::message::iso20022::camt::camt_027_001_10::*;
use crate::message::iso20022::camt::camt_028_001_12::*;
use crate::message::iso20022::camt::camt_029_001_13::*;
use crate::message::iso20022::camt::camt_030_001_06::*;
use crate::message::iso20022::camt::camt_031_001_07::*;
use crate::message::iso20022::camt::camt_032_001_05::*;
use crate::message::iso20022::camt::camt_033_001_07::*;
use crate::message::iso20022::camt::camt_034_001_07::*;
use crate::message::iso20022::camt::camt_035_001_06::*;
use crate::message::iso20022::camt::camt_036_001_06::*;
use crate::message::iso20022::camt::camt_037_001_10::*;
use crate::message::iso20022::camt::camt_038_001_05::*;
use crate::message::iso20022::camt::camt_039_001_06::*;
use crate::message::iso20022::camt::camt_040_001_04::*;
use crate::message::iso20022::camt::camt_041_001_04::*;
use crate::message::iso20022::camt::camt_042_001_04::*;
use crate::message::iso20022::camt::camt_043_001_04::*;
use crate::message::iso20022::camt::camt_044_001_03::*;
use crate::message::iso20022::camt::camt_045_001_03::*;
use crate::message::iso20022::camt::camt_046_001_08::*;
use crate::message::iso20022::camt::camt_047_001_08::*;
use crate::message::iso20022::camt::camt_048_001_07::*;
use crate::message::iso20022::camt::camt_049_001_07::*;
use crate::message::iso20022::camt::camt_050_001_07::*;
use crate::message::iso20022::camt::camt_051_001_07::*;
use crate::message::iso20022::camt::camt_052_001_12::*;
use crate::message::iso20022::camt::camt_053_001_12::*;
use crate::message::iso20022::camt::camt_054_001_12::*;
use crate::message::iso20022::camt::camt_055_001_12::*;
use crate::message::iso20022::camt::camt_056_001_11::*;
use crate::message::iso20022::camt::camt_057_001_08::*;
use crate::message::iso20022::camt::camt_058_001_09::*;
use crate::message::iso20022::camt::camt_059_001_08::*;
use crate::message::iso20022::camt::camt_060_001_07::*;
use crate::message::iso20022::camt::camt_061_001_02::*;
use crate::message::iso20022::camt::camt_062_001_03::*;
use crate::message::iso20022::camt::camt_063_001_02::*;
use crate::message::iso20022::camt::camt_066_001_02::*;
use crate::message::iso20022::camt::camt_067_001_02::*;
use crate::message::iso20022::camt::camt_068_001_02::*;
use crate::message::iso20022::camt::camt_069_001_05::*;
use crate::message::iso20022::camt::camt_070_001_06::*;
use crate::message::iso20022::camt::camt_071_001_05::*;
use crate::message::iso20022::camt::camt_072_001_02::*;
use crate::message::iso20022::camt::camt_073_001_02::*;
use crate::message::iso20022::camt::camt_074_001_02::*;
use crate::message::iso20022::camt::camt_075_001_02::*;
use crate::message::iso20022::camt::camt_078_001_02::*;
use crate::message::iso20022::camt::camt_079_001_02::*;
use crate::message::iso20022::camt::camt_080_001_02::*;
use crate::message::iso20022::camt::camt_081_001_02::*;
use crate::message::iso20022::camt::camt_082_001_02::*;
use crate::message::iso20022::camt::camt_083_001_02::*;
use crate::message::iso20022::camt::camt_084_001_02::*;
use crate::message::iso20022::camt::camt_085_001_02::*;
use crate::message::iso20022::camt::camt_086_001_05::*;
use crate::message::iso20022::camt::camt_087_001_09::*;
use crate::message::iso20022::camt::camt_088_001_02::*;
use crate::message::iso20022::camt::camt_101_001_02::*;
use crate::message::iso20022::camt::camt_102_001_03::*;
use crate::message::iso20022::camt::camt_103_001_03::*;
use crate::message::iso20022::camt::camt_104_001_01::*;
use crate::message::iso20022::camt::camt_105_001_02::*;
use crate::message::iso20022::camt::camt_106_001_02::*;
use crate::message::iso20022::camt::camt_107_001_02::*;
use crate::message::iso20022::camt::camt_108_001_02::*;
use crate::message::iso20022::camt::camt_109_001_02::*;
use crate::message::iso20022::camt::camt_110_001_01::*;
use crate::message::iso20022::camt::camt_111_001_01::*;

use crate::message::iso20022::pacs::pacs_002_001_12::*;
use crate::message::iso20022::pacs::pacs_002_001_14::*;
use crate::message::iso20022::pacs::pacs_003_001_11::*;
use crate::message::iso20022::pacs::pacs_004_001_13::*;
use crate::message::iso20022::pacs::pacs_007_001_13::*;
use crate::message::iso20022::pacs::pacs_008_001_12::*;
use crate::message::iso20022::pacs::pacs_009_001_11::*;
use crate::message::iso20022::pacs::pacs_010_001_06::*;
use crate::message::iso20022::pacs::pacs_028_001_06::*;
use crate::message::iso20022::pacs::pacs_029_001_02::*;

use crate::message::iso20022::pain::pain_001_001_12::*;
use crate::message::iso20022::pain::pain_002_001_14::*;
use crate::message::iso20022::pain::pain_007_001_12::*;
use crate::message::iso20022::pain::pain_008_001_11::*;
use crate::message::iso20022::pain::pain_009_001_08::*;
use crate::message::iso20022::pain::pain_010_001_08::*;
use crate::message::iso20022::pain::pain_011_001_08::*;
use crate::message::iso20022::pain::pain_012_001_08::*;
use crate::message::iso20022::pain::pain_013_001_11::*;
use crate::message::iso20022::pain::pain_014_001_11::*;
use crate::message::iso20022::pain::pain_017_001_04::*;
use crate::message::iso20022::pain::pain_018_001_04::*;

use crate::message::iso20022::reda::reda_001_001_04::*;
use crate::message::iso20022::reda::reda_002_001_04::*;
use crate::message::iso20022::reda::reda_004_001_07::*;
use crate::message::iso20022::reda::reda_005_001_03::*;
use crate::message::iso20022::reda::reda_006_001_01::*;
use crate::message::iso20022::reda::reda_007_001_01::*;
use crate::message::iso20022::reda::reda_008_001_01::*;
use crate::message::iso20022::reda::reda_009_001_01::*;
use crate::message::iso20022::reda::reda_010_001_01::*;
use crate::message::iso20022::reda::reda_012_001_01::*;
use crate::message::iso20022::reda::reda_013_001_01::*;
use crate::message::iso20022::reda::reda_014_001_02::*;
use crate::message::iso20022::reda::reda_015_001_01::*;
use crate::message::iso20022::reda::reda_016_001_01::*;
use crate::message::iso20022::reda::reda_017_001_02::*;
use crate::message::iso20022::reda::reda_018_001_01::*;
use crate::message::iso20022::reda::reda_019_001_01::*;
use crate::message::iso20022::reda::reda_020_001_01::*;
use crate::message::iso20022::reda::reda_021_001_01::*;
use crate::message::iso20022::reda::reda_022_001_02::*;
use crate::message::iso20022::reda::reda_023_001_01::*;
use crate::message::iso20022::reda::reda_029_001_01::*;
use crate::message::iso20022::reda::reda_030_001_01::*;
use crate::message::iso20022::reda::reda_031_001_01::*;
use crate::message::iso20022::reda::reda_032_001_01::*;
use crate::message::iso20022::reda::reda_033_001_01::*;
use crate::message::iso20022::reda::reda_034_001_01::*;
use crate::message::iso20022::reda::reda_035_001_01::*;
use crate::message::iso20022::reda::reda_036_001_01::*;
use crate::message::iso20022::reda::reda_037_001_01::*;
use crate::message::iso20022::reda::reda_041_001_02::*;
use crate::message::iso20022::reda::reda_042_001_01::*;
use crate::message::iso20022::reda::reda_043_001_02::*;
use crate::message::iso20022::reda::reda_056_001_01::*;
use crate::message::iso20022::reda::reda_057_001_01::*;
use crate::message::iso20022::reda::reda_058_001_01::*;
use crate::message::iso20022::reda::reda_059_001_01::*;
use crate::message::iso20022::reda::reda_060_001_02::*;
use crate::message::iso20022::reda::reda_061_001_02::*;
use crate::message::iso20022::reda::reda_064_001_02::*;
use crate::message::iso20022::reda::reda_065_001_02::*;
use crate::message::iso20022::reda::reda_066_001_02::*;
use crate::message::iso20022::reda::reda_067_001_02::*;
use crate::message::iso20022::reda::reda_068_001_02::*;
use crate::message::iso20022::reda::reda_069_001_02::*;
use crate::message::iso20022::reda::reda_070_001_02::*;
use crate::message::iso20022::reda::reda_071_001_02::*;
use crate::message::iso20022::reda::reda_072_001_02::*;
use crate::message::iso20022::reda::reda_073_001_02::*;

use crate::message::iso20022::remt::remt_001_001_06::*;
use crate::message::iso20022::remt::remt_002_001_03::*;


#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub enum Document {
	#[validate]
	#[serde(rename = "admi.002.001.01")]
	Admi00200101(Box<Admi00200101>),

	#[validate]
	#[serde(rename = "SysEvtNtfctn")]
	SystemEventNotificationV02(Box<SystemEventNotificationV02>),

	#[validate]
	#[serde(rename = "RptQryReq")]
	ReportQueryRequestV02(Box<ReportQueryRequestV02>),

	#[validate]
	#[serde(rename = "RsndReq")]
	ResendRequestV01(Box<ResendRequestV01>),

	#[validate]
	#[serde(rename = "RctAck")]
	ReceiptAcknowledgementV01(Box<ReceiptAcknowledgementV01>),

	#[validate]
	#[serde(rename = "StatcDataReq")]
	StaticDataRequestV02(Box<StaticDataRequestV02>),

	#[validate]
	#[serde(rename = "StatcDataRpt")]
	StaticDataReportV02(Box<StaticDataReportV02>),

	#[validate]
	#[serde(rename = "SysEvtAck")]
	SystemEventAcknowledgementV01(Box<SystemEventAcknowledgementV01>),

	#[validate]
	#[serde(rename = "PrcgReq")]
	ProcessingRequestV02(Box<ProcessingRequestV02>),

	#[validate]
	#[serde(rename = "NtfctnOfCrspdc")]
	NotificationOfCorrespondenceV01(Box<NotificationOfCorrespondenceV01>),

	#[validate]
	#[serde(rename = "FIToFIPmtStsRpt")]
	FIToFIPaymentStatusReportV12(Box<FIToFIPaymentStatusReportV12>),

	#[validate]
	#[serde(rename = "FIToFIPmtStsRpt")]
	FIToFIPaymentStatusReportV14(Box<FIToFIPaymentStatusReportV14>),

	#[validate]
	#[serde(rename = "FIToFICstmrDrctDbt")]
	FIToFICustomerDirectDebitV11(Box<FIToFICustomerDirectDebitV11>),

	#[validate]
	#[serde(rename = "PmtRtr")]
	PaymentReturnV13(Box<PaymentReturnV13>),

	#[validate]
	#[serde(rename = "FIToFIPmtRvsl")]
	FIToFIPaymentReversalV13(Box<FIToFIPaymentReversalV13>),

	#[validate]
	#[serde(rename = "FIToFICstmrCdtTrf")]
	FIToFICustomerCreditTransferV12(Box<FIToFICustomerCreditTransferV12>),

	#[validate]
	#[serde(rename = "FICdtTrf")]
	FinancialInstitutionCreditTransferV11(Box<FinancialInstitutionCreditTransferV11>),

	#[validate]
	#[serde(rename = "FIDrctDbt")]
	FinancialInstitutionDirectDebitV06(Box<FinancialInstitutionDirectDebitV06>),

	#[validate]
	#[serde(rename = "FIToFIPmtStsReq")]
	FIToFIPaymentStatusRequestV06(Box<FIToFIPaymentStatusRequestV06>),

	#[validate]
	#[serde(rename = "MulSttlmReq")]
	MultilateralSettlementRequestV02(Box<MultilateralSettlementRequestV02>),

	#[validate]
	#[serde(rename = "CstmrCdtTrfInitn")]
	CustomerCreditTransferInitiationV12(Box<CustomerCreditTransferInitiationV12>),

	#[validate]
	#[serde(rename = "CstmrPmtStsRpt")]
	CustomerPaymentStatusReportV14(Box<CustomerPaymentStatusReportV14>),

	#[validate]
	#[serde(rename = "CstmrPmtRvsl")]
	CustomerPaymentReversalV12(Box<CustomerPaymentReversalV12>),

	#[validate]
	#[serde(rename = "CstmrDrctDbtInitn")]
	CustomerDirectDebitInitiationV11(Box<CustomerDirectDebitInitiationV11>),

	#[validate]
	#[serde(rename = "MndtInitnReq")]
	MandateInitiationRequestV08(Box<MandateInitiationRequestV08>),

	#[validate]
	#[serde(rename = "MndtAmdmntReq")]
	MandateAmendmentRequestV08(Box<MandateAmendmentRequestV08>),

	#[validate]
	#[serde(rename = "MndtCxlReq")]
	MandateCancellationRequestV08(Box<MandateCancellationRequestV08>),

	#[validate]
	#[serde(rename = "MndtAccptncRpt")]
	MandateAcceptanceReportV08(Box<MandateAcceptanceReportV08>),

	#[validate]
	#[serde(rename = "CdtrPmtActvtnReq")]
	CreditorPaymentActivationRequestV11(Box<CreditorPaymentActivationRequestV11>),

	#[validate]
	#[serde(rename = "CdtrPmtActvtnReqStsRpt")]
	CreditorPaymentActivationRequestStatusReportV11(Box<CreditorPaymentActivationRequestStatusReportV11>),

	#[validate]
	#[serde(rename = "MndtCpyReq")]
	MandateCopyRequestV04(Box<MandateCopyRequestV04>),

	#[validate]
	#[serde(rename = "MndtSspnsnReq")]
	MandateSuspensionRequestV04(Box<MandateSuspensionRequestV04>),

	#[validate]
	#[serde(rename = "AcctOpngInstr")]
	AccountOpeningInstructionV08(Box<AccountOpeningInstructionV08>),

	#[validate]
	#[serde(rename = "AcctDtlsConf")]
	AccountDetailsConfirmationV08(Box<AccountDetailsConfirmationV08>),

	#[validate]
	#[serde(rename = "AcctModInstr")]
	AccountModificationInstructionV08(Box<AccountModificationInstructionV08>),

	#[validate]
	#[serde(rename = "ReqForAcctMgmtStsRpt")]
	RequestForAccountManagementStatusReportV06(Box<RequestForAccountManagementStatusReportV06>),

	#[validate]
	#[serde(rename = "AcctMgmtStsRpt")]
	AccountManagementStatusReportV07(Box<AccountManagementStatusReportV07>),

	#[validate]
	#[serde(rename = "AcctOpngReq")]
	AccountOpeningRequestV05(Box<AccountOpeningRequestV05>),

	#[validate]
	#[serde(rename = "AcctOpngAmdmntReq")]
	AccountOpeningAmendmentRequestV05(Box<AccountOpeningAmendmentRequestV05>),

	#[validate]
	#[serde(rename = "AcctOpngAddtlInfReq")]
	AccountOpeningAdditionalInformationRequestV04(Box<AccountOpeningAdditionalInformationRequestV04>),

	#[validate]
	#[serde(rename = "AcctReqAck")]
	AccountRequestAcknowledgementV04(Box<AccountRequestAcknowledgementV04>),

	#[validate]
	#[serde(rename = "AcctReqRjctn")]
	AccountRequestRejectionV04(Box<AccountRequestRejectionV04>),

	#[validate]
	#[serde(rename = "AcctAddtlInfReq")]
	AccountAdditionalInformationRequestV04(Box<AccountAdditionalInformationRequestV04>),

	#[validate]
	#[serde(rename = "AcctRptReq")]
	AccountReportRequestV04(Box<AccountReportRequestV04>),

	#[validate]
	#[serde(rename = "AcctRpt")]
	AccountReportV05(Box<AccountReportV05>),

	#[validate]
	#[serde(rename = "AcctExcldMndtMntncReq")]
	AccountExcludedMandateMaintenanceRequestV04(Box<AccountExcludedMandateMaintenanceRequestV04>),

	#[validate]
	#[serde(rename = "AcctExcldMndtMntncAmdmntReq")]
	AccountExcludedMandateMaintenanceAmendmentRequestV04(Box<AccountExcludedMandateMaintenanceAmendmentRequestV04>),

	#[validate]
	#[serde(rename = "AcctMndtMntncReq")]
	AccountMandateMaintenanceRequestV04(Box<AccountMandateMaintenanceRequestV04>),

	#[validate]
	#[serde(rename = "AcctMndtMntncAmdmntReq")]
	AccountMandateMaintenanceAmendmentRequestV04(Box<AccountMandateMaintenanceAmendmentRequestV04>),

	#[validate]
	#[serde(rename = "AcctClsgReq")]
	AccountClosingRequestV04(Box<AccountClosingRequestV04>),

	#[validate]
	#[serde(rename = "AcctClsgAmdmntReq")]
	AccountClosingAmendmentRequestV04(Box<AccountClosingAmendmentRequestV04>),

	#[validate]
	#[serde(rename = "AcctClsgAddtlInfReq")]
	AccountClosingAdditionalInformationRequestV04(Box<AccountClosingAdditionalInformationRequestV04>),

	#[validate]
	#[serde(rename = "IdModAdvc")]
	IdentificationModificationAdviceV04(Box<IdentificationModificationAdviceV04>),

	#[validate]
	#[serde(rename = "IdVrfctnReq")]
	IdentificationVerificationRequestV04(Box<IdentificationVerificationRequestV04>),

	#[validate]
	#[serde(rename = "IdVrfctnRpt")]
	IdentificationVerificationReportV04(Box<IdentificationVerificationReportV04>),

	#[validate]
	#[serde(rename = "AcctSwtchInfReq")]
	AccountSwitchInformationRequestV05(Box<AccountSwitchInformationRequestV05>),

	#[validate]
	#[serde(rename = "AcctSwtchInfRspn")]
	AccountSwitchInformationResponseV05(Box<AccountSwitchInformationResponseV05>),

	#[validate]
	#[serde(rename = "AcctSwtchCclExstgPmt")]
	AccountSwitchCancelExistingPaymentV05(Box<AccountSwitchCancelExistingPaymentV05>),

	#[validate]
	#[serde(rename = "AcctSwtchReqRdrctn")]
	AccountSwitchRequestRedirectionV04(Box<AccountSwitchRequestRedirectionV04>),

	#[validate]
	#[serde(rename = "AcctSwtchReqBalTrf")]
	AccountSwitchRequestBalanceTransferV05(Box<AccountSwitchRequestBalanceTransferV05>),

	#[validate]
	#[serde(rename = "AcctSwtchBalTrfAck")]
	AccountSwitchBalanceTransferAcknowledgementV05(Box<AccountSwitchBalanceTransferAcknowledgementV05>),

	#[validate]
	#[serde(rename = "AcctSwtchNtfyAcctSwtchCmplt")]
	AccountSwitchNotifyAccountSwitchCompleteV02(Box<AccountSwitchNotifyAccountSwitchCompleteV02>),

	#[validate]
	#[serde(rename = "AcctSwtchReqPmt")]
	AccountSwitchRequestPaymentV05(Box<AccountSwitchRequestPaymentV05>),

	#[validate]
	#[serde(rename = "AcctSwtchPmtRspn")]
	AccountSwitchPaymentResponseV02(Box<AccountSwitchPaymentResponseV02>),

	#[validate]
	#[serde(rename = "AcctSwtchTermntnSwtch")]
	AccountSwitchTerminationSwitchV01(Box<AccountSwitchTerminationSwitchV01>),

	#[validate]
	#[serde(rename = "AcctSwtchTechRjctn")]
	AccountSwitchTechnicalRejectionV02(Box<AccountSwitchTechnicalRejectionV02>),

	#[validate]
	#[serde(rename = "PricRpt")]
	PriceReportV04(Box<PriceReportV04>),

	#[validate]
	#[serde(rename = "PricRptCxl")]
	PriceReportCancellationV04(Box<PriceReportCancellationV04>),

	#[validate]
	#[serde(rename = "FndRefDataRpt")]
	FundReferenceDataReportV07(Box<FundReferenceDataReportV07>),

	#[validate]
	#[serde(rename = "InvstmtFndRptReq")]
	InvestmentFundReportRequestV03(Box<InvestmentFundReportRequestV03>),

	#[validate]
	#[serde(rename = "SctyCreReq")]
	SecurityCreationRequestV01(Box<SecurityCreationRequestV01>),

	#[validate]
	#[serde(rename = "SctyMntncReq")]
	SecurityMaintenanceRequestV01(Box<SecurityMaintenanceRequestV01>),

	#[validate]
	#[serde(rename = "SctyCreStsAdvc")]
	SecurityCreationStatusAdviceV01(Box<SecurityCreationStatusAdviceV01>),

	#[validate]
	#[serde(rename = "SctyActvtyAdvc")]
	SecurityActivityAdviceV01(Box<SecurityActivityAdviceV01>),

	#[validate]
	#[serde(rename = "SctyQry")]
	SecurityQueryV01(Box<SecurityQueryV01>),

	#[validate]
	#[serde(rename = "SctyRpt")]
	SecurityReportV01(Box<SecurityReportV01>),

	#[validate]
	#[serde(rename = "SctyDeltnReq")]
	SecurityDeletionRequestV01(Box<SecurityDeletionRequestV01>),

	#[validate]
	#[serde(rename = "PtyCreReq")]
	PartyCreationRequestV02(Box<PartyCreationRequestV02>),

	#[validate]
	#[serde(rename = "PtyQry")]
	PartyQueryV01(Box<PartyQueryV01>),

	#[validate]
	#[serde(rename = "PtyStsAdvc")]
	PartyStatusAdviceV01(Box<PartyStatusAdviceV01>),

	#[validate]
	#[serde(rename = "PtyRpt")]
	PartyReportV02(Box<PartyReportV02>),

	#[validate]
	#[serde(rename = "SctiesAcctCreReq")]
	SecuritiesAccountCreationRequestV01(Box<SecuritiesAccountCreationRequestV01>),

	#[validate]
	#[serde(rename = "SctiesAcctQry")]
	SecuritiesAccountQueryV01(Box<SecuritiesAccountQueryV01>),

	#[validate]
	#[serde(rename = "SctiesAcctStsAdvc")]
	SecuritiesAccountStatusAdviceV01(Box<SecuritiesAccountStatusAdviceV01>),

	#[validate]
	#[serde(rename = "SctiesAcctRpt")]
	SecuritiesAccountReportV01(Box<SecuritiesAccountReportV01>),

	#[validate]
	#[serde(rename = "PtyModReq")]
	PartyModificationRequestV02(Box<PartyModificationRequestV02>),

	#[validate]
	#[serde(rename = "SctiesAcctModReq")]
	SecuritiesAccountModificationRequestV01(Box<SecuritiesAccountModificationRequestV01>),

	#[validate]
	#[serde(rename = "SctyMntncStsAdvc")]
	SecurityMaintenanceStatusAdviceV01(Box<SecurityMaintenanceStatusAdviceV01>),

	#[validate]
	#[serde(rename = "SctyDeltnStsAdvc")]
	SecurityDeletionStatusAdviceV01(Box<SecurityDeletionStatusAdviceV01>),

	#[validate]
	#[serde(rename = "PtyDeltnReq")]
	PartyDeletionRequestV01(Box<PartyDeletionRequestV01>),

	#[validate]
	#[serde(rename = "SctiesAcctDeltnReq")]
	SecuritiesAccountDeletionRequestV01(Box<SecuritiesAccountDeletionRequestV01>),

	#[validate]
	#[serde(rename = "SctiesAudtTrlQry")]
	SecuritiesAuditTrailQueryV01(Box<SecuritiesAuditTrailQueryV01>),

	#[validate]
	#[serde(rename = "SctiesAudtTrlRpt")]
	SecuritiesAuditTrailReportV01(Box<SecuritiesAuditTrailReportV01>),

	#[validate]
	#[serde(rename = "SctiesAcctActvtyAdvc")]
	SecuritiesAccountActivityAdviceV01(Box<SecuritiesAccountActivityAdviceV01>),

	#[validate]
	#[serde(rename = "SctiesAcctAudtTrlQry")]
	SecuritiesAccountAuditTrailQueryV01(Box<SecuritiesAccountAuditTrailQueryV01>),

	#[validate]
	#[serde(rename = "SctiesAcctAudtTrlRpt")]
	SecuritiesAccountAuditTrailReportV01(Box<SecuritiesAccountAuditTrailReportV01>),

	#[validate]
	#[serde(rename = "PtyActvtyAdvc")]
	PartyActivityAdviceV02(Box<PartyActivityAdviceV02>),

	#[validate]
	#[serde(rename = "PtyAudtTrlQry")]
	PartyAuditTrailQueryV01(Box<PartyAuditTrailQueryV01>),

	#[validate]
	#[serde(rename = "PtyAudtTrlRpt")]
	PartyAuditTrailReportV02(Box<PartyAuditTrailReportV02>),

	#[validate]
	#[serde(rename = "StgSttlmInstr")]
	StandingSettlementInstructionV01(Box<StandingSettlementInstructionV01>),

	#[validate]
	#[serde(rename = "StgSttlmInstrDeltn")]
	StandingSettlementInstructionDeletionV01(Box<StandingSettlementInstructionDeletionV01>),

	#[validate]
	#[serde(rename = "StgSttlmInstrStsAdvc")]
	StandingSettlementInstructionStatusAdviceV01(Box<StandingSettlementInstructionStatusAdviceV01>),

	#[validate]
	#[serde(rename = "StgSttlmInstrCxl")]
	StandingSettlementInstructionCancellationV01(Box<StandingSettlementInstructionCancellationV01>),

	#[validate]
	#[serde(rename = "NetgCutOffRefDataUpdReq")]
	NettingCutOffReferenceDataUpdateRequestV02(Box<NettingCutOffReferenceDataUpdateRequestV02>),

	#[validate]
	#[serde(rename = "NetgCutOffRefDataRpt")]
	NettingCutOffReferenceDataReportV02(Box<NettingCutOffReferenceDataReportV02>),

	#[validate]
	#[serde(rename = "CalQry")]
	CalendarQueryV02(Box<CalendarQueryV02>),

	#[validate]
	#[serde(rename = "CalRpt")]
	CalendarReportV02(Box<CalendarReportV02>),

	#[validate]
	#[serde(rename = "ReqToPayCdtrEnrlmntReq")]
	RequestToPayCreditorEnrolmentRequestV02(Box<RequestToPayCreditorEnrolmentRequestV02>),

	#[validate]
	#[serde(rename = "ReqToPayCdtrEnrlmntAmdmntReq")]
	RequestToPayCreditorEnrolmentAmendmentRequestV02(Box<RequestToPayCreditorEnrolmentAmendmentRequestV02>),

	#[validate]
	#[serde(rename = "ReqToPayCdtrEnrlmntCxlReq")]
	RequestToPayCreditorEnrolmentCancellationRequestV02(Box<RequestToPayCreditorEnrolmentCancellationRequestV02>),

	#[validate]
	#[serde(rename = "ReqToPayCdtrEnrlmntStsRpt")]
	RequestToPayCreditorEnrolmentStatusReportV02(Box<RequestToPayCreditorEnrolmentStatusReportV02>),

	#[validate]
	#[serde(rename = "ReqToPayDbtrActvtnReq")]
	RequestToPayDebtorActivationRequestV02(Box<RequestToPayDebtorActivationRequestV02>),

	#[validate]
	#[serde(rename = "ReqToPayDbtrActvtnAmdmntReq")]
	RequestToPayDebtorActivationAmendmentRequestV02(Box<RequestToPayDebtorActivationAmendmentRequestV02>),

	#[validate]
	#[serde(rename = "ReqToPayDbtrActvtnCxlReq")]
	RequestToPayDebtorActivationCancellationRequestV02(Box<RequestToPayDebtorActivationCancellationRequestV02>),

	#[validate]
	#[serde(rename = "ReqToPayDbtrActvtnStsRpt")]
	RequestToPayDebtorActivationStatusReportV02(Box<RequestToPayDebtorActivationStatusReportV02>),

	#[validate]
	#[serde(rename = "RmtAdvc")]
	RemittanceAdviceV06(Box<RemittanceAdviceV06>),

	#[validate]
	#[serde(rename = "RmtLctnAdvc")]
	RemittanceLocationAdviceV03(Box<RemittanceLocationAdviceV03>),

	#[validate]
	#[serde(rename = "GetAcct")]
	GetAccountV08(Box<GetAccountV08>),

	#[validate]
	#[serde(rename = "RtrAcct")]
	ReturnAccountV10(Box<ReturnAccountV10>),

	#[validate]
	#[serde(rename = "GetTx")]
	GetTransactionV11(Box<GetTransactionV11>),

	#[validate]
	#[serde(rename = "RtrTx")]
	ReturnTransactionV11(Box<ReturnTransactionV11>),

	#[validate]
	#[serde(rename = "ModfyTx")]
	ModifyTransactionV10(Box<ModifyTransactionV10>),

	#[validate]
	#[serde(rename = "CclTx")]
	CancelTransactionV11(Box<CancelTransactionV11>),

	#[validate]
	#[serde(rename = "GetLmt")]
	GetLimitV08(Box<GetLimitV08>),

	#[validate]
	#[serde(rename = "RtrLmt")]
	ReturnLimitV09(Box<ReturnLimitV09>),

	#[validate]
	#[serde(rename = "ModfyLmt")]
	ModifyLimitV08(Box<ModifyLimitV08>),

	#[validate]
	#[serde(rename = "DelLmt")]
	DeleteLimitV08(Box<DeleteLimitV08>),

	#[validate]
	#[serde(rename = "GetMmb")]
	GetMemberV04(Box<GetMemberV04>),

	#[validate]
	#[serde(rename = "RtrMmb")]
	ReturnMemberV05(Box<ReturnMemberV05>),

	#[validate]
	#[serde(rename = "ModfyMmb")]
	ModifyMemberV04(Box<ModifyMemberV04>),

	#[validate]
	#[serde(rename = "GetCcyXchgRate")]
	GetCurrencyExchangeRateV04(Box<GetCurrencyExchangeRateV04>),

	#[validate]
	#[serde(rename = "RtrCcyXchgRate")]
	ReturnCurrencyExchangeRateV05(Box<ReturnCurrencyExchangeRateV05>),

	#[validate]
	#[serde(rename = "GetBizDayInf")]
	GetBusinessDayInformationV05(Box<GetBusinessDayInformationV05>),

	#[validate]
	#[serde(rename = "RtrBizDayInf")]
	ReturnBusinessDayInformationV07(Box<ReturnBusinessDayInformationV07>),

	#[validate]
	#[serde(rename = "GetGnlBizInf")]
	GetGeneralBusinessInformationV04(Box<GetGeneralBusinessInformationV04>),

	#[validate]
	#[serde(rename = "RtrGnlBizInf")]
	ReturnGeneralBusinessInformationV06(Box<ReturnGeneralBusinessInformationV06>),

	#[validate]
	#[serde(rename = "BckpPmt")]
	BackupPaymentV07(Box<BackupPaymentV07>),

	#[validate]
	#[serde(rename = "ModfyStgOrdr")]
	ModifyStandingOrderV08(Box<ModifyStandingOrderV08>),

	#[validate]
	#[serde(rename = "Rct")]
	ReceiptV08(Box<ReceiptV08>),

	#[validate]
	#[serde(rename = "UblToApply")]
	UnableToApplyV10(Box<UnableToApplyV10>),

	#[validate]
	#[serde(rename = "ClmNonRct")]
	ClaimNonReceiptV10(Box<ClaimNonReceiptV10>),

	#[validate]
	#[serde(rename = "AddtlPmtInf")]
	AdditionalPaymentInformationV12(Box<AdditionalPaymentInformationV12>),

	#[validate]
	#[serde(rename = "RsltnOfInvstgtn")]
	ResolutionOfInvestigationV13(Box<ResolutionOfInvestigationV13>),

	#[validate]
	#[serde(rename = "NtfctnOfCaseAssgnmt")]
	NotificationOfCaseAssignmentV06(Box<NotificationOfCaseAssignmentV06>),

	#[validate]
	#[serde(rename = "RjctInvstgtn")]
	RejectInvestigationV07(Box<RejectInvestigationV07>),

	#[validate]
	#[serde(rename = "CclCaseAssgnmt")]
	CancelCaseAssignmentV05(Box<CancelCaseAssignmentV05>),

	#[validate]
	#[serde(rename = "ReqForDplct")]
	RequestForDuplicateV07(Box<RequestForDuplicateV07>),

	#[validate]
	#[serde(rename = "Dplct")]
	DuplicateV07(Box<DuplicateV07>),

	#[validate]
	#[serde(rename = "PrtryFrmtInvstgtn")]
	ProprietaryFormatInvestigationV06(Box<ProprietaryFormatInvestigationV06>),

	#[validate]
	#[serde(rename = "DbtAuthstnRspn")]
	DebitAuthorisationResponseV06(Box<DebitAuthorisationResponseV06>),

	#[validate]
	#[serde(rename = "DbtAuthstnReq")]
	DebitAuthorisationRequestV10(Box<DebitAuthorisationRequestV10>),

	#[validate]
	#[serde(rename = "CaseStsRptReq")]
	CaseStatusReportRequestV05(Box<CaseStatusReportRequestV05>),

	#[validate]
	#[serde(rename = "CaseStsRpt")]
	CaseStatusReportV06(Box<CaseStatusReportV06>),

	#[validate]
	#[serde(rename = "FndEstmtdCshFcstRpt")]
	FundEstimatedCashForecastReportV04(Box<FundEstimatedCashForecastReportV04>),

	#[validate]
	#[serde(rename = "FndConfdCshFcstRpt")]
	FundConfirmedCashForecastReportV04(Box<FundConfirmedCashForecastReportV04>),

	#[validate]
	#[serde(rename = "FndDtldEstmtdCshFcstRpt")]
	FundDetailedEstimatedCashForecastReportV04(Box<FundDetailedEstimatedCashForecastReportV04>),

	#[validate]
	#[serde(rename = "FndDtldConfdCshFcstRpt")]
	FundDetailedConfirmedCashForecastReportV04(Box<FundDetailedConfirmedCashForecastReportV04>),

	#[validate]
	#[serde(rename = "FndConfdCshFcstRptCxl")]
	FundConfirmedCashForecastReportCancellationV03(Box<FundConfirmedCashForecastReportCancellationV03>),

	#[validate]
	#[serde(rename = "FndDtldConfdCshFcstRptCxl")]
	FundDetailedConfirmedCashForecastReportCancellationV03(Box<FundDetailedConfirmedCashForecastReportCancellationV03>),

	#[validate]
	#[serde(rename = "GetRsvatn")]
	GetReservationV08(Box<GetReservationV08>),

	#[validate]
	#[serde(rename = "RtrRsvatn")]
	ReturnReservationV08(Box<ReturnReservationV08>),

	#[validate]
	#[serde(rename = "ModfyRsvatn")]
	ModifyReservationV07(Box<ModifyReservationV07>),

	#[validate]
	#[serde(rename = "DelRsvatn")]
	DeleteReservationV07(Box<DeleteReservationV07>),

	#[validate]
	#[serde(rename = "LqdtyCdtTrf")]
	LiquidityCreditTransferV07(Box<LiquidityCreditTransferV07>),

	#[validate]
	#[serde(rename = "LqdtyDbtTrf")]
	LiquidityDebitTransferV07(Box<LiquidityDebitTransferV07>),

	#[validate]
	#[serde(rename = "BkToCstmrAcctRpt")]
	BankToCustomerAccountReportV12(Box<BankToCustomerAccountReportV12>),

	#[validate]
	#[serde(rename = "BkToCstmrStmt")]
	BankToCustomerStatementV12(Box<BankToCustomerStatementV12>),

	#[validate]
	#[serde(rename = "BkToCstmrDbtCdtNtfctn")]
	BankToCustomerDebitCreditNotificationV12(Box<BankToCustomerDebitCreditNotificationV12>),

	#[validate]
	#[serde(rename = "CstmrPmtCxlReq")]
	CustomerPaymentCancellationRequestV12(Box<CustomerPaymentCancellationRequestV12>),

	#[validate]
	#[serde(rename = "FIToFIPmtCxlReq")]
	FIToFIPaymentCancellationRequestV11(Box<FIToFIPaymentCancellationRequestV11>),

	#[validate]
	#[serde(rename = "NtfctnToRcv")]
	NotificationToReceiveV08(Box<NotificationToReceiveV08>),

	#[validate]
	#[serde(rename = "NtfctnToRcvCxlAdvc")]
	NotificationToReceiveCancellationAdviceV09(Box<NotificationToReceiveCancellationAdviceV09>),

	#[validate]
	#[serde(rename = "NtfctnToRcvStsRpt")]
	NotificationToReceiveStatusReportV08(Box<NotificationToReceiveStatusReportV08>),

	#[validate]
	#[serde(rename = "AcctRptgReq")]
	AccountReportingRequestV07(Box<AccountReportingRequestV07>),

	#[validate]
	#[serde(rename = "PayInCall")]
	PayInCallV02(Box<PayInCallV02>),

	#[validate]
	#[serde(rename = "PayInSchdl")]
	PayInScheduleV03(Box<PayInScheduleV03>),

	#[validate]
	#[serde(rename = "PayInEvtAck")]
	PayInEventAcknowledgementV02(Box<PayInEventAcknowledgementV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntInstr")]
	IntraBalanceMovementInstructionV02(Box<IntraBalanceMovementInstructionV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntStsAdvc")]
	IntraBalanceMovementStatusAdviceV02(Box<IntraBalanceMovementStatusAdviceV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntConf")]
	IntraBalanceMovementConfirmationV02(Box<IntraBalanceMovementConfirmationV02>),

	#[validate]
	#[serde(rename = "GetStgOrdr")]
	GetStandingOrderV05(Box<GetStandingOrderV05>),

	#[validate]
	#[serde(rename = "RtrStgOrdr")]
	ReturnStandingOrderV06(Box<ReturnStandingOrderV06>),

	#[validate]
	#[serde(rename = "DelStgOrdr")]
	DeleteStandingOrderV05(Box<DeleteStandingOrderV05>),

	#[validate]
	#[serde(rename = "IntraBalMvmntModReq")]
	IntraBalanceMovementModificationRequestV02(Box<IntraBalanceMovementModificationRequestV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntModReqStsAdvc")]
	IntraBalanceMovementModificationRequestStatusAdviceV02(Box<IntraBalanceMovementModificationRequestStatusAdviceV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntCxlReq")]
	IntraBalanceMovementCancellationRequestV02(Box<IntraBalanceMovementCancellationRequestV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntCxlReqStsAdvc")]
	IntraBalanceMovementCancellationRequestStatusAdviceV02(Box<IntraBalanceMovementCancellationRequestStatusAdviceV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntQry")]
	IntraBalanceMovementQueryV02(Box<IntraBalanceMovementQueryV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntQryRspn")]
	IntraBalanceMovementQueryResponseV02(Box<IntraBalanceMovementQueryResponseV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntModQry")]
	IntraBalanceMovementModificationQueryV02(Box<IntraBalanceMovementModificationQueryV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntModRpt")]
	IntraBalanceMovementModificationReportV02(Box<IntraBalanceMovementModificationReportV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntCxlQry")]
	IntraBalanceMovementCancellationQueryV02(Box<IntraBalanceMovementCancellationQueryV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntCxlRpt")]
	IntraBalanceMovementCancellationReportV02(Box<IntraBalanceMovementCancellationReportV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntPstngRpt")]
	IntraBalanceMovementPostingReportV02(Box<IntraBalanceMovementPostingReportV02>),

	#[validate]
	#[serde(rename = "IntraBalMvmntPdgRpt")]
	IntraBalanceMovementPendingReportV02(Box<IntraBalanceMovementPendingReportV02>),

	#[validate]
	#[serde(rename = "BkSvcsBllgStmt")]
	BankServicesBillingStatementV05(Box<BankServicesBillingStatementV05>),

	#[validate]
	#[serde(rename = "ReqToModfyPmt")]
	RequestToModifyPaymentV09(Box<RequestToModifyPaymentV09>),

	#[validate]
	#[serde(rename = "NetRpt")]
	NetReportV02(Box<NetReportV02>),

	#[validate]
	#[serde(rename = "CretLmt")]
	CreateLimitV02(Box<CreateLimitV02>),

	#[validate]
	#[serde(rename = "CretStgOrdr")]
	CreateStandingOrderV03(Box<CreateStandingOrderV03>),

	#[validate]
	#[serde(rename = "CretRsvatn")]
	CreateReservationV03(Box<CreateReservationV03>),

	#[validate]
	#[serde(rename = "CretMmb")]
	CreateMemberV01(Box<CreateMemberV01>),

	#[validate]
	#[serde(rename = "ChrgsPmtNtfctn")]
	ChargesPaymentNotificationV02(Box<ChargesPaymentNotificationV02>),

	#[validate]
	#[serde(rename = "ChrgsPmtReq")]
	ChargesPaymentRequestV02(Box<ChargesPaymentRequestV02>),

	#[validate]
	#[serde(rename = "ChqPresntmntNtfctn")]
	ChequePresentmentNotificationV02(Box<ChequePresentmentNotificationV02>),

	#[validate]
	#[serde(rename = "ChqCxlOrStopReq")]
	ChequeCancellationOrStopRequestV02(Box<ChequeCancellationOrStopRequestV02>),

	#[validate]
	#[serde(rename = "ChqCxlOrStopRpt")]
	ChequeCancellationOrStopReportV02(Box<ChequeCancellationOrStopReportV02>),

	#[validate]
	#[serde(rename = "InvstgtnReq")]
	InvestigationRequestV01(Box<InvestigationRequestV01>),

	#[validate]
	#[serde(rename = "InvstgtnRspn")]
	InvestigationResponseV01(Box<InvestigationResponseV01>),

	#[validate]
	#[serde(rename = "InfReqOpng")]
	InformationRequestOpeningV02(Box<InformationRequestOpeningV02>),

	#[validate]
	#[serde(rename = "InfReqRspn")]
	InformationRequestResponseV02(Box<InformationRequestResponseV02>),

	#[validate]
	#[serde(rename = "InfReqStsChngNtfctn")]
	InformationRequestStatusChangeNotificationV01(Box<InformationRequestStatusChangeNotificationV01>),

	#[validate]
	#[serde(rename = "MnyMktScrdMktSttstclRpt")]
	MoneyMarketSecuredMarketStatisticalReportV02(Box<MoneyMarketSecuredMarketStatisticalReportV02>),

	#[validate]
	#[serde(rename = "MnyMktUscrdMktSttstclRpt")]
	MoneyMarketUnsecuredMarketStatisticalReportV02(Box<MoneyMarketUnsecuredMarketStatisticalReportV02>),

	#[validate]
	#[serde(rename = "MnyMktFXSwpsSttstclRpt")]
	MoneyMarketForeignExchangeSwapsStatisticalReportV02(Box<MoneyMarketForeignExchangeSwapsStatisticalReportV02>),

	#[validate]
	#[serde(rename = "MnyMktOvrnghtIndxSwpsSttstclRpt")]
	MoneyMarketOvernightIndexSwapsStatisticalReportV02(Box<MoneyMarketOvernightIndexSwapsStatisticalReportV02>),

	#[validate]
	#[serde(rename = "FinInstrmRptgTxRpt")]
	FinancialInstrumentReportingTransactionReportV03(Box<FinancialInstrumentReportingTransactionReportV03>),

	#[validate]
	#[serde(rename = "FinInstrmRptgRefDataRpt")]
	FinancialInstrumentReportingReferenceDataReportV02(Box<FinancialInstrumentReportingReferenceDataReportV02>),

	#[validate]
	#[serde(rename = "CtrctRegnReq")]
	ContractRegistrationRequestV04(Box<ContractRegistrationRequestV04>),

	#[validate]
	#[serde(rename = "CtrctRegnConf")]
	ContractRegistrationConfirmationV04(Box<ContractRegistrationConfirmationV04>),

	#[validate]
	#[serde(rename = "CtrctRegnClsrReq")]
	ContractRegistrationClosureRequestV04(Box<ContractRegistrationClosureRequestV04>),

	#[validate]
	#[serde(rename = "CtrctRegnAmdmntReq")]
	ContractRegistrationAmendmentRequestV04(Box<ContractRegistrationAmendmentRequestV04>),

	#[validate]
	#[serde(rename = "CtrctRegnStmt")]
	ContractRegistrationStatementV04(Box<ContractRegistrationStatementV04>),

	#[validate]
	#[serde(rename = "CtrctRegnStmtReq")]
	ContractRegistrationStatementRequestV04(Box<ContractRegistrationStatementRequestV04>),

	#[validate]
	#[serde(rename = "PmtRgltryInfNtfctn")]
	PaymentRegulatoryInformationNotificationV04(Box<PaymentRegulatoryInformationNotificationV04>),

	#[validate]
	#[serde(rename = "CcyCtrlSpprtgDocDlvry")]
	CurrencyControlSupportingDocumentDeliveryV04(Box<CurrencyControlSupportingDocumentDeliveryV04>),

	#[validate]
	#[serde(rename = "CcyCtrlReqOrLttr")]
	CurrencyControlRequestOrLetterV04(Box<CurrencyControlRequestOrLetterV04>),

	#[validate]
	#[serde(rename = "CcyCtrlStsAdvc")]
	CurrencyControlStatusAdviceV04(Box<CurrencyControlStatusAdviceV04>),

	#[validate]
	#[serde(rename = "MnyMktSttstclRptStsAdvc")]
	MoneyMarketStatisticalReportStatusAdviceV01(Box<MoneyMarketStatisticalReportStatusAdviceV01>),

	#[validate]
	#[serde(rename = "DerivsTradRptQry")]
	DerivativesTradeReportQueryV05(Box<DerivativesTradeReportQueryV05>),

	#[validate]
	#[serde(rename = "DerivsTradRpt")]
	DerivativesTradeReportV04(Box<DerivativesTradeReportV04>),

	#[validate]
	#[serde(rename = "FinInstrmRptgStsAdvc")]
	FinancialInstrumentReportingStatusAdviceV01(Box<FinancialInstrumentReportingStatusAdviceV01>),

	#[validate]
	#[serde(rename = "FinInstrmRptgEqtyTrnsprncyDataRpt")]
	FinancialInstrumentReportingEquityTransparencyDataReportV01(Box<FinancialInstrumentReportingEquityTransparencyDataReportV01>),

	#[validate]
	#[serde(rename = "FinInstrmRptgNonEqtyTrnsprncyDataRpt")]
	FinancialInstrumentReportingNonEquityTransparencyDataReportV03(Box<FinancialInstrumentReportingNonEquityTransparencyDataReportV03>),

	#[validate]
	#[serde(rename = "InvcTaxRpt")]
	InvoiceTaxReportV01(Box<InvoiceTaxReportV01>),

	#[validate]
	#[serde(rename = "FinInstrmRptgTradgVolCapDataRpt")]
	FinancialInstrumentReportingTradingVolumeCapDataReportV01(Box<FinancialInstrumentReportingTradingVolumeCapDataReportV01>),

	#[validate]
	#[serde(rename = "FinInstrmRptgRefDataDltaRpt")]
	FinancialInstrumentReportingReferenceDataDeltaReportV03(Box<FinancialInstrumentReportingReferenceDataDeltaReportV03>),

	#[validate]
	#[serde(rename = "InvcTaxRptStsAdvc")]
	InvoiceTaxReportStatusAdviceV01(Box<InvoiceTaxReportStatusAdviceV01>),

	#[validate]
	#[serde(rename = "FinInstrmRptgNonWorkgDayRpt")]
	FinancialInstrumentReportingNonWorkingDayReportV01(Box<FinancialInstrumentReportingNonWorkingDayReportV01>),

	#[validate]
	#[serde(rename = "FinInstrmRptgEqtyTradgActvtyRpt")]
	FinancialInstrumentReportingEquityTradingActivityReportV01(Box<FinancialInstrumentReportingEquityTradingActivityReportV01>),

	#[validate]
	#[serde(rename = "FinInstrmRptgNonEqtyTradgActvtyRpt")]
	FinancialInstrumentReportingNonEquityTradingActivityReportV01(Box<FinancialInstrumentReportingNonEquityTradingActivityReportV01>),

	#[validate]
	#[serde(rename = "FinInstrmRptgInvldRefDataRpt")]
	FinancialInstrumentReportingInvalidReferenceDataReportV02(Box<FinancialInstrumentReportingInvalidReferenceDataReportV02>),

	#[validate]
	#[serde(rename = "FinInstrmRptgRefDataIndxRpt")]
	FinancialInstrumentReportingReferenceDataIndexReportV01(Box<FinancialInstrumentReportingReferenceDataIndexReportV01>),

	#[validate]
	#[serde(rename = "FinInstrmRptgEqtyTradgActvtyRslt")]
	FinancialInstrumentReportingEquityTradingActivityResultV02(Box<FinancialInstrumentReportingEquityTradingActivityResultV02>),

	#[validate]
	#[serde(rename = "FinInstrmRptgNonEqtyTradgActvtyRslt")]
	FinancialInstrumentReportingNonEquityTradingActivityResultV03(Box<FinancialInstrumentReportingNonEquityTradingActivityResultV03>),

	#[validate]
	#[serde(rename = "FinInstrmRptgCtryCdRpt")]
	FinancialInstrumentReportingCountryCodeReportV01(Box<FinancialInstrumentReportingCountryCodeReportV01>),

	#[validate]
	#[serde(rename = "FinInstrmRptgCcyCdRpt")]
	FinancialInstrumentReportingCurrencyCodeReportV01(Box<FinancialInstrumentReportingCurrencyCodeReportV01>),

	#[validate]
	#[serde(rename = "FinInstrmRptgMktIdCdRpt")]
	FinancialInstrumentReportingMarketIdentificationCodeReportV02(Box<FinancialInstrumentReportingMarketIdentificationCodeReportV02>),

	#[validate]
	#[serde(rename = "FinInstrmRptgInstrmClssfctnRpt")]
	FinancialInstrumentReportingInstrumentClassificationReportV01(Box<FinancialInstrumentReportingInstrumentClassificationReportV01>),

	#[validate]
	#[serde(rename = "SctiesFincgRptgTxRpt")]
	SecuritiesFinancingReportingTransactionReportV02(Box<SecuritiesFinancingReportingTransactionReportV02>),

	#[validate]
	#[serde(rename = "FinInstrmRptgTradgVolCapRsltRpt")]
	FinancialInstrumentReportingTradingVolumeCapResultReportV01(Box<FinancialInstrumentReportingTradingVolumeCapResultReportV01>),

	#[validate]
	#[serde(rename = "CCPClrMmbRpt")]
	CCPClearingMemberReportV01(Box<CCPClearingMemberReportV01>),

	#[validate]
	#[serde(rename = "CCPMmbRqrmntsRpt")]
	CCPMemberRequirementsReportV01(Box<CCPMemberRequirementsReportV01>),

	#[validate]
	#[serde(rename = "CCPMmbOblgtnsRpt")]
	CCPMemberObligationsReportV01(Box<CCPMemberObligationsReportV01>),

	#[validate]
	#[serde(rename = "CCPPrtflStrssTstgDefRpt")]
	CCPPortfolioStressTestingDefinitionReportV02(Box<CCPPortfolioStressTestingDefinitionReportV02>),

	#[validate]
	#[serde(rename = "CCPPrtflStrssTstgRsltRpt")]
	CCPPortfolioStressTestingResultReportV01(Box<CCPPortfolioStressTestingResultReportV01>),

	#[validate]
	#[serde(rename = "CCPIncmStmtAndCptlAdqcyRpt")]
	CCPIncomeStatementAndCapitalAdequacyReportV01(Box<CCPIncomeStatementAndCapitalAdequacyReportV01>),

	#[validate]
	#[serde(rename = "CCPDalyCshFlowsRpt")]
	CCPDailyCashFlowsReportV02(Box<CCPDailyCashFlowsReportV02>),

	#[validate]
	#[serde(rename = "CCPInvstmtsRpt")]
	CCPInvestmentsReportV01(Box<CCPInvestmentsReportV01>),

	#[validate]
	#[serde(rename = "CCPLqdtyStrssTstgDefRpt")]
	CCPLiquidityStressTestingDefinitionReportV01(Box<CCPLiquidityStressTestingDefinitionReportV01>),

	#[validate]
	#[serde(rename = "CCPLqdtyStrssTstgRsltRpt")]
	CCPLiquidityStressTestingResultReportV01(Box<CCPLiquidityStressTestingResultReportV01>),

	#[validate]
	#[serde(rename = "CCPAvlblFinRsrcsRpt")]
	CCPAvailableFinancialResourcesReportV01(Box<CCPAvailableFinancialResourcesReportV01>),

	#[validate]
	#[serde(rename = "CCPBckTstgDefRpt")]
	CCPBackTestingDefinitionReportV01(Box<CCPBackTestingDefinitionReportV01>),

	#[validate]
	#[serde(rename = "CCPBckTstgRsltRpt")]
	CCPBackTestingResultReportV01(Box<CCPBackTestingResultReportV01>),

	#[validate]
	#[serde(rename = "CCPCollRpt")]
	CCPCollateralReportV01(Box<CCPCollateralReportV01>),

	#[validate]
	#[serde(rename = "CCPAcctPosRpt")]
	CCPAccountPositionReportV01(Box<CCPAccountPositionReportV01>),

	#[validate]
	#[serde(rename = "CCPClrdPdctRpt")]
	CCPClearedProductReportV01(Box<CCPClearedProductReportV01>),

	#[validate]
	#[serde(rename = "SctiesFincgRptgTxMrgnDataRpt")]
	SecuritiesFinancingReportingTransactionMarginDataReportV02(Box<SecuritiesFinancingReportingTransactionMarginDataReportV02>),

	#[validate]
	#[serde(rename = "SctiesFincgRptgTxReusdCollDataRpt")]
	SecuritiesFinancingReportingTransactionReusedCollateralDataReportV02(Box<SecuritiesFinancingReportingTransactionReusedCollateralDataReportV02>),

	#[validate]
	#[serde(rename = "SttlmIntlrRpt")]
	SettlementInternaliserReportV01(Box<SettlementInternaliserReportV01>),

	#[validate]
	#[serde(rename = "FinSprvsdPtyIdntyRpt")]
	FinancialSupervisedPartyIdentityReportV01(Box<FinancialSupervisedPartyIdentityReportV01>),

	#[validate]
	#[serde(rename = "FinBchmkRpt")]
	FinancialBenchmarkReportV01(Box<FinancialBenchmarkReportV01>),

	#[validate]
	#[serde(rename = "SctiesFincgRptgPairgReq")]
	SecuritiesFinancingReportingPairingRequestV02(Box<SecuritiesFinancingReportingPairingRequestV02>),

	#[validate]
	#[serde(rename = "SctiesFincgRptgTxStatRpt")]
	SecuritiesFinancingReportingTransactionStateReportV02(Box<SecuritiesFinancingReportingTransactionStateReportV02>),

	#[validate]
	#[serde(rename = "SctiesFincgRptgRcncltnStsAdvc")]
	SecuritiesFinancingReportingReconciliationStatusAdviceV02(Box<SecuritiesFinancingReportingReconciliationStatusAdviceV02>),

	#[validate]
	#[serde(rename = "SctiesFincgRptgMssngCollReq")]
	SecuritiesFinancingReportingMissingCollateralRequestV02(Box<SecuritiesFinancingReportingMissingCollateralRequestV02>),

	#[validate]
	#[serde(rename = "SctiesFincgRptgTxStsAdvc")]
	SecuritiesFinancingReportingTransactionStatusAdviceV02(Box<SecuritiesFinancingReportingTransactionStatusAdviceV02>),

	#[validate]
	#[serde(rename = "SctiesFincgRptgMrgnDataTxStatRpt")]
	SecuritiesFinancingReportingMarginDataTransactionStateReportV02(Box<SecuritiesFinancingReportingMarginDataTransactionStateReportV02>),

	#[validate]
	#[serde(rename = "SctiesFincgRptgReusdCollDataTxStatRpt")]
	SecuritiesFinancingReportingReusedCollateralDataTransactionStateReportV02(Box<SecuritiesFinancingReportingReusedCollateralDataTransactionStateReportV02>),

	#[validate]
	#[serde(rename = "DerivsTradPosSetRpt")]
	DerivativesTradePositionSetReportV02(Box<DerivativesTradePositionSetReportV02>),

	#[validate]
	#[serde(rename = "DerivsTradRcncltnSttstclRpt")]
	DerivativesTradeReconciliationStatisticalReportV03(Box<DerivativesTradeReconciliationStatisticalReportV03>),

	#[validate]
	#[serde(rename = "DerivsTradRjctnSttstclRpt")]
	DerivativesTradeRejectionStatisticalReportV04(Box<DerivativesTradeRejectionStatisticalReportV04>),

	#[validate]
	#[serde(rename = "SctiesFincgRptgTxQry")]
	SecuritiesFinancingReportingTransactionQueryV02(Box<SecuritiesFinancingReportingTransactionQueryV02>),

	#[validate]
	#[serde(rename = "SttlmFlsMnthlyRpt")]
	SettlementFailsMonthlyReportV01(Box<SettlementFailsMonthlyReportV01>),

	#[validate]
	#[serde(rename = "SttlmFlsAnlRpt")]
	SettlementFailsAnnualReportV01(Box<SettlementFailsAnnualReportV01>),

	#[validate]
	#[serde(rename = "FinInstrmRptgCxlRpt")]
	FinancialInstrumentReportingCancellationReportV01(Box<FinancialInstrumentReportingCancellationReportV01>),

	#[validate]
	#[serde(rename = "SctiesFincgRptgPosSetRpt")]
	SecuritiesFinancingReportingPositionSetReportV01(Box<SecuritiesFinancingReportingPositionSetReportV01>),

	#[validate]
	#[serde(rename = "DerivsTradWrnngsRpt")]
	DerivativesTradeWarningsReportV01(Box<DerivativesTradeWarningsReportV01>),

	#[validate]
	#[serde(rename = "DerivsTradStatRpt")]
	DerivativesTradeStateReportV02(Box<DerivativesTradeStateReportV02>),

	#[validate]
	#[serde(rename = "DerivsTradMrgnDataRpt")]
	DerivativesTradeMarginDataReportV02(Box<DerivativesTradeMarginDataReportV02>),

	#[validate]
	#[serde(rename = "DerivsTradMrgnDataTxStatRpt")]
	DerivativesTradeMarginDataTransactionStateReportV02(Box<DerivativesTradeMarginDataTransactionStateReportV02>),

	#[validate]
	#[serde(rename = "CCPIntrprbltyRpt")]
	CCPInteroperabilityReportV01(Box<CCPInteroperabilityReportV01>),

	#[validate]
	#[serde(rename = "OrdrBookRpt")]
	OrderBookReportV01(Box<OrderBookReportV01>),
}
