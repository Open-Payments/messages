// Code generated by xgen. DO NOT EDIT.

use serde::{Deserialize, Serialize};
use crate::Document;


// document ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct document {
	#[serde(rename = "Document")]
	pub document: Document,
}


// Event2 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Event2 {
	#[serde(rename = "EvtCd")]
	pub evt_cd: String,
	#[serde(rename = "EvtParam")]
	pub evt_param: Vec<String>,
	#[serde(rename = "EvtDesc")]
	pub evt_desc: Option<String>,
	#[serde(rename = "EvtTm")]
	pub evt_tm: Option<String>,
}


// ISODateTime ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// Max1000Text ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max1000Text {
	#[serde(rename = "Max1000Text")]
	pub max1000_text: String,
}


// Max35Text ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max35Text {
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Max4AlphaNumericText ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Max4AlphaNumericText {
	#[serde(rename = "Max4AlphaNumericText")]
	pub max4_alpha_numeric_text: String,
}


// SystemEventNotificationV02 ...
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct SystemEventNotificationV02 {
	#[serde(rename = "EvtInf")]
	pub evt_inf: Event2,
}