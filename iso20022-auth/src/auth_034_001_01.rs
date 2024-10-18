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
use regex::Regex;


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "$value")]
	pub active_currency_code: String,
}

impl ActiveCurrencyCode {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.active_currency_code) {
			return false
		}
		return true
	}
}


// AdditionalInformation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdditionalInformation1 {
	#[serde(rename = "InfTp")]
	pub inf_tp: InformationType1Choice,
	#[serde(rename = "InfVal")]
	pub inf_val: Max350Text,
}

impl AdditionalInformation1 {
	pub fn validate(&self) -> bool {
		if !self.inf_tp.validate() { return false }
		if !self.inf_val.validate() { return false }
		return true
	}
}


// AddressType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum AddressType2Code {
	#[default]
	#[serde(rename = "ADDR")]
	CodeADDR,
	#[serde(rename = "PBOX")]
	CodePBOX,
	#[serde(rename = "HOME")]
	CodeHOME,
	#[serde(rename = "BIZZ")]
	CodeBIZZ,
	#[serde(rename = "MLTO")]
	CodeMLTO,
	#[serde(rename = "DLVY")]
	CodeDLVY,
}

impl AddressType2Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// AnyBICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnyBICIdentifier {
	#[serde(rename = "$value")]
	pub any_bic_identifier: String,
}

impl AnyBICIdentifier {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}").unwrap();
		if !pattern.is_match(&self.any_bic_identifier) {
			return false
		}
		return true
	}
}


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BaseOneRate {
	#[serde(rename = "$value")]
	pub base_one_rate: f64,
}

impl BaseOneRate {
	pub fn validate(&self) -> bool {
		return true
	}
}


// BinaryFile1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BinaryFile1 {
	#[serde(rename = "MIMETp", skip_serializing_if = "Option::is_none")]
	pub mime_tp: Option<Max35Text>,
	#[serde(rename = "NcodgTp", skip_serializing_if = "Option::is_none")]
	pub ncodg_tp: Option<Max35Text>,
	#[serde(rename = "CharSet", skip_serializing_if = "Option::is_none")]
	pub char_set: Option<Max35Text>,
	#[serde(rename = "InclBinryObjct", skip_serializing_if = "Option::is_none")]
	pub incl_binry_objct: Option<Max100KBinary>,
}

impl BinaryFile1 {
	pub fn validate(&self) -> bool {
		if let Some(ref mime_tp_value) = self.mime_tp { if !mime_tp_value.validate() { return false; } }
		if let Some(ref ncodg_tp_value) = self.ncodg_tp { if !ncodg_tp_value.validate() { return false; } }
		if let Some(ref char_set_value) = self.char_set { if !char_set_value.validate() { return false; } }
		if let Some(ref incl_binry_objct_value) = self.incl_binry_objct { if !incl_binry_objct_value.validate() { return false; } }
		return true
	}
}


// ContactDetails2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContactDetails2 {
	#[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
	pub nm_prfx: Option<NamePrefix1Code>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
	pub phne_nb: Option<PhoneNumber>,
	#[serde(rename = "MobNb", skip_serializing_if = "Option::is_none")]
	pub mob_nb: Option<PhoneNumber>,
	#[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
	pub fax_nb: Option<PhoneNumber>,
	#[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
	pub email_adr: Option<Max2048Text>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Max35Text>,
}

impl ContactDetails2 {
	pub fn validate(&self) -> bool {
		if let Some(ref nm_prfx_value) = self.nm_prfx { if !nm_prfx_value.validate() { return false; } }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref phne_nb_value) = self.phne_nb { if !phne_nb_value.validate() { return false; } }
		if let Some(ref mob_nb_value) = self.mob_nb { if !mob_nb_value.validate() { return false; } }
		if let Some(ref fax_nb_value) = self.fax_nb { if !fax_nb_value.validate() { return false; } }
		if let Some(ref email_adr_value) = self.email_adr { if !email_adr_value.validate() { return false; } }
		if let Some(ref othr_value) = self.othr { if !othr_value.validate() { return false; } }
		return true
	}
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CountryCode {
	#[serde(rename = "$value")]
	pub country_code: String,
}

impl CountryCode {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{2,2}").unwrap();
		if !pattern.is_match(&self.country_code) {
			return false
		}
		return true
	}
}


// CreditorReferenceInformation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceInformation2 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<CreditorReferenceType2>,
	#[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
	pub ref_attr: Option<Max35Text>,
}

impl CreditorReferenceInformation2 {
	pub fn validate(&self) -> bool {
		if let Some(ref tp_value) = self.tp { if !tp_value.validate() { return false; } }
		if let Some(ref ref_attr_value) = self.ref_attr { if !ref_attr_value.validate() { return false; } }
		return true
	}
}


// CreditorReferenceType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<DocumentType3Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl CreditorReferenceType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// CreditorReferenceType2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceType2 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: CreditorReferenceType1Choice,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl CreditorReferenceType2 {
	pub fn validate(&self) -> bool {
		if !self.cd_or_prtry.validate() { return false }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		return true
	}
}


// CurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CurrencyAndAmountSimpleType {
	#[serde(rename = "$value")]
	pub currency_and_amount_simple_type: f64,
}

impl CurrencyAndAmountSimpleType {
	pub fn validate(&self) -> bool {
		if self.currency_and_amount_simple_type < 0.000000 {
			return false
		}
		return true
	}
}


// CurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}

impl CurrencyAndAmount {
	pub fn validate(&self) -> bool {
		return true
	}
}


// CurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CurrencyCode {
	#[serde(rename = "$value")]
	pub currency_code: String,
}

impl CurrencyCode {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("[A-Z]{3,3}").unwrap();
		if !pattern.is_match(&self.currency_code) {
			return false
		}
		return true
	}
}


// CurrencyReference3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CurrencyReference3 {
	#[serde(rename = "TrgtCcy")]
	pub trgt_ccy: ActiveCurrencyCode,
	#[serde(rename = "SrcCcy")]
	pub src_ccy: ActiveCurrencyCode,
	#[serde(rename = "XchgRateInf", skip_serializing_if = "Option::is_none")]
	pub xchg_rate_inf: Option<Vec<ExchangeRateInformation1>>,
}

impl CurrencyReference3 {
	pub fn validate(&self) -> bool {
		if !self.trgt_ccy.validate() { return false }
		if !self.src_ccy.validate() { return false }
		if let Some(ref xchg_rate_inf_vec) = self.xchg_rate_inf { for item in xchg_rate_inf_vec { if !item.validate() { return false; } } }
		return true
	}
}


// DateAndPlaceOfBirth ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndPlaceOfBirth {
	#[serde(rename = "BirthDt")]
	pub birth_dt: String,
	#[serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none")]
	pub prvc_of_birth: Option<Max35Text>,
	#[serde(rename = "CityOfBirth")]
	pub city_of_birth: Max35Text,
	#[serde(rename = "CtryOfBirth")]
	pub ctry_of_birth: CountryCode,
}

impl DateAndPlaceOfBirth {
	pub fn validate(&self) -> bool {
		if let Some(ref prvc_of_birth_value) = self.prvc_of_birth { if !prvc_of_birth_value.validate() { return false; } }
		if !self.city_of_birth.validate() { return false }
		if !self.ctry_of_birth.validate() { return false }
		return true
	}
}


// DocumentGeneralInformation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentGeneralInformation2 {
	#[serde(rename = "DocTp")]
	pub doc_tp: ExternalDocumentType1Code,
	#[serde(rename = "DocNb")]
	pub doc_nb: Max35Text,
	#[serde(rename = "SndrRcvrSeqId", skip_serializing_if = "Option::is_none")]
	pub sndr_rcvr_seq_id: Option<Max140Text>,
	#[serde(rename = "IsseDt", skip_serializing_if = "Option::is_none")]
	pub isse_dt: Option<String>,
	#[serde(rename = "URL", skip_serializing_if = "Option::is_none")]
	pub url: Option<Max256Text>,
	#[serde(rename = "AttchdBinryFile", skip_serializing_if = "Option::is_none")]
	pub attchd_binry_file: Option<Vec<BinaryFile1>>,
}

impl DocumentGeneralInformation2 {
	pub fn validate(&self) -> bool {
		if !self.doc_tp.validate() { return false }
		if !self.doc_nb.validate() { return false }
		if let Some(ref sndr_rcvr_seq_id_value) = self.sndr_rcvr_seq_id { if !sndr_rcvr_seq_id_value.validate() { return false; } }
		if let Some(ref url_value) = self.url { if !url_value.validate() { return false; } }
		if let Some(ref attchd_binry_file_vec) = self.attchd_binry_file { for item in attchd_binry_file_vec { if !item.validate() { return false; } } }
		return true
	}
}


// DocumentType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum DocumentType3Code {
	#[default]
	#[serde(rename = "RADM")]
	CodeRADM,
	#[serde(rename = "RPIN")]
	CodeRPIN,
	#[serde(rename = "FXDR")]
	CodeFXDR,
	#[serde(rename = "DISP")]
	CodeDISP,
	#[serde(rename = "PUOR")]
	CodePUOR,
	#[serde(rename = "SCOR")]
	CodeSCOR,
}

impl DocumentType3Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// EarlyPayment1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EarlyPayment1 {
	#[serde(rename = "EarlyPmtDt")]
	pub early_pmt_dt: String,
	#[serde(rename = "DscntPct")]
	pub dscnt_pct: f64,
	#[serde(rename = "DscntAmt")]
	pub dscnt_amt: CurrencyAndAmount,
	#[serde(rename = "EarlyPmtTaxSpcfctn", skip_serializing_if = "Option::is_none")]
	pub early_pmt_tax_spcfctn: Option<Vec<EarlyPaymentsVAT1>>,
	#[serde(rename = "EarlyPmtTaxTtl", skip_serializing_if = "Option::is_none")]
	pub early_pmt_tax_ttl: Option<CurrencyAndAmount>,
	#[serde(rename = "DuePyblAmtWthEarlyPmt", skip_serializing_if = "Option::is_none")]
	pub due_pybl_amt_wth_early_pmt: Option<CurrencyAndAmount>,
}

impl EarlyPayment1 {
	pub fn validate(&self) -> bool {
		if !self.dscnt_amt.validate() { return false }
		if let Some(ref early_pmt_tax_spcfctn_vec) = self.early_pmt_tax_spcfctn { for item in early_pmt_tax_spcfctn_vec { if !item.validate() { return false; } } }
		if let Some(ref early_pmt_tax_ttl_value) = self.early_pmt_tax_ttl { if !early_pmt_tax_ttl_value.validate() { return false; } }
		if let Some(ref due_pybl_amt_wth_early_pmt_value) = self.due_pybl_amt_wth_early_pmt { if !due_pybl_amt_wth_early_pmt_value.validate() { return false; } }
		return true
	}
}


// EarlyPaymentsVAT1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EarlyPaymentsVAT1 {
	#[serde(rename = "TaxRate")]
	pub tax_rate: f64,
	#[serde(rename = "DscntTaxTp")]
	pub dscnt_tax_tp: Max4Text,
	#[serde(rename = "DscntTaxAmt")]
	pub dscnt_tax_amt: CurrencyAndAmount,
}

impl EarlyPaymentsVAT1 {
	pub fn validate(&self) -> bool {
		if !self.dscnt_tax_tp.validate() { return false }
		if !self.dscnt_tax_amt.validate() { return false }
		return true
	}
}


// ExchangeRateInformation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExchangeRateInformation1 {
	#[serde(rename = "XchgRate", skip_serializing_if = "Option::is_none")]
	pub xchg_rate: Option<f64>,
	#[serde(rename = "RateTp", skip_serializing_if = "Option::is_none")]
	pub rate_tp: Option<ExchangeRateType1Code>,
	#[serde(rename = "CtrctId", skip_serializing_if = "Option::is_none")]
	pub ctrct_id: Option<Max35Text>,
}

impl ExchangeRateInformation1 {
	pub fn validate(&self) -> bool {
		if let Some(ref rate_tp_value) = self.rate_tp { if !rate_tp_value.validate() { return false; } }
		if let Some(ref ctrct_id_value) = self.ctrct_id { if !ctrct_id_value.validate() { return false; } }
		return true
	}
}


// ExchangeRateType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum ExchangeRateType1Code {
	#[default]
	#[serde(rename = "SPOT")]
	CodeSPOT,
	#[serde(rename = "SALE")]
	CodeSALE,
	#[serde(rename = "AGRD")]
	CodeAGRD,
}

impl ExchangeRateType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ExternalDocumentType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalDocumentType1Code {
	#[serde(rename = "$value")]
	pub external_document_type1_code: String,
}

impl ExternalDocumentType1Code {
	pub fn validate(&self) -> bool {
		if self.external_document_type1_code.chars().count() < 1 {
			return false
		}
		if self.external_document_type1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// ExternalOrganisationIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalOrganisationIdentification1Code {
	#[serde(rename = "$value")]
	pub external_organisation_identification1_code: String,
}

impl ExternalOrganisationIdentification1Code {
	pub fn validate(&self) -> bool {
		if self.external_organisation_identification1_code.chars().count() < 1 {
			return false
		}
		if self.external_organisation_identification1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// ExternalPersonIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExternalPersonIdentification1Code {
	#[serde(rename = "$value")]
	pub external_person_identification1_code: String,
}

impl ExternalPersonIdentification1Code {
	pub fn validate(&self) -> bool {
		if self.external_person_identification1_code.chars().count() < 1 {
			return false
		}
		if self.external_person_identification1_code.chars().count() > 4 {
			return false
		}
		return true
	}
}


// GenericOrganisationIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericOrganisationIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericOrganisationIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		return true
	}
}


// GenericPersonIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericPersonIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<Max35Text>,
}

impl GenericPersonIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if let Some(ref schme_nm_value) = self.schme_nm { if !schme_nm_value.validate() { return false; } }
		if let Some(ref issr_value) = self.issr { if !issr_value.validate() { return false; } }
		return true
	}
}


// GroupHeader69 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GroupHeader69 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "IssdDt")]
	pub issd_dt: String,
	#[serde(rename = "RptCtgy")]
	pub rpt_ctgy: ExternalDocumentType1Code,
	#[serde(rename = "TaxRptPurp")]
	pub tax_rpt_purp: ExternalDocumentType1Code,
	#[serde(rename = "OrgnlId", skip_serializing_if = "Option::is_none")]
	pub orgnl_id: Option<Max35Text>,
	#[serde(rename = "SellrTaxRprtv", skip_serializing_if = "Option::is_none")]
	pub sellr_tax_rprtv: Option<PartyIdentification116>,
	#[serde(rename = "BuyrTaxRprtv", skip_serializing_if = "Option::is_none")]
	pub buyr_tax_rprtv: Option<PartyIdentification116>,
	#[serde(rename = "LangCd", skip_serializing_if = "Option::is_none")]
	pub lang_cd: Option<String>,
}

impl GroupHeader69 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		if !self.rpt_ctgy.validate() { return false }
		if !self.tax_rpt_purp.validate() { return false }
		if let Some(ref orgnl_id_value) = self.orgnl_id { if !orgnl_id_value.validate() { return false; } }
		if let Some(ref sellr_tax_rprtv_value) = self.sellr_tax_rprtv { if !sellr_tax_rprtv_value.validate() { return false; } }
		if let Some(ref buyr_tax_rprtv_value) = self.buyr_tax_rprtv { if !buyr_tax_rprtv_value.validate() { return false; } }
		return true
	}
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODate {
	#[serde(rename = "$value")]
	pub iso_date: String,
}

impl ISODate {
	pub fn validate(&self) -> bool {
		return true
	}
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ISODateTime {
	#[serde(rename = "$value")]
	pub iso_date_time: String,
}

impl ISODateTime {
	pub fn validate(&self) -> bool {
		return true
	}
}


// InformationType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InformationType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<InformationType1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max140Text>,
}

impl InformationType1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// InformationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum InformationType1Code {
	#[default]
	#[serde(rename = "INST")]
	CodeINST,
	#[serde(rename = "RELY")]
	CodeRELY,
}

impl InformationType1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// InvoiceTaxReportV01 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InvoiceTaxReportV01 {
	#[serde(rename = "InvcTaxRptHdr")]
	pub invc_tax_rpt_hdr: TaxReportHeader1,
	#[serde(rename = "TaxRpt")]
	pub tax_rpt: Vec<TaxReport1>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl InvoiceTaxReportV01 {
	pub fn validate(&self) -> bool {
		if !self.invc_tax_rpt_hdr.validate() { return false }
		for item in &self.tax_rpt { if !item.validate() { return false; } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
	}
}


// LanguageCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LanguageCode {
	#[serde(rename = "$value")]
	pub language_code: String,
}

impl LanguageCode {
	pub fn validate(&self) -> bool {
		return true
	}
}


// LegalOrganisation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LegalOrganisation1 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Max35Text>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
}

impl LegalOrganisation1 {
	pub fn validate(&self) -> bool {
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		return true
	}
}


// Max100KBinary ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max100KBinary {
	#[serde(rename = "$value")]
	pub max100_k_binary: String,
}

impl Max100KBinary {
	pub fn validate(&self) -> bool {
		if self.max100_k_binary.chars().count() < 1 {
			return false
		}
		if self.max100_k_binary.chars().count() > 102400 {
			return false
		}
		return true
	}
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max140Text {
	#[serde(rename = "$value")]
	pub max140_text: String,
}

impl Max140Text {
	pub fn validate(&self) -> bool {
		if self.max140_text.chars().count() < 1 {
			return false
		}
		if self.max140_text.chars().count() > 140 {
			return false
		}
		return true
	}
}


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max16Text {
	#[serde(rename = "$value")]
	pub max16_text: String,
}

impl Max16Text {
	pub fn validate(&self) -> bool {
		if self.max16_text.chars().count() < 1 {
			return false
		}
		if self.max16_text.chars().count() > 16 {
			return false
		}
		return true
	}
}


// Max2048Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max2048Text {
	#[serde(rename = "$value")]
	pub max2048_text: String,
}

impl Max2048Text {
	pub fn validate(&self) -> bool {
		if self.max2048_text.chars().count() < 1 {
			return false
		}
		if self.max2048_text.chars().count() > 2048 {
			return false
		}
		return true
	}
}


// Max256Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max256Text {
	#[serde(rename = "$value")]
	pub max256_text: String,
}

impl Max256Text {
	pub fn validate(&self) -> bool {
		if self.max256_text.chars().count() < 1 {
			return false
		}
		if self.max256_text.chars().count() > 256 {
			return false
		}
		return true
	}
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max350Text {
	#[serde(rename = "$value")]
	pub max350_text: String,
}

impl Max350Text {
	pub fn validate(&self) -> bool {
		if self.max350_text.chars().count() < 1 {
			return false
		}
		if self.max350_text.chars().count() > 350 {
			return false
		}
		return true
	}
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max35Text {
	#[serde(rename = "$value")]
	pub max35_text: String,
}

impl Max35Text {
	pub fn validate(&self) -> bool {
		if self.max35_text.chars().count() < 1 {
			return false
		}
		if self.max35_text.chars().count() > 35 {
			return false
		}
		return true
	}
}


// Max4Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max4Text {
	#[serde(rename = "$value")]
	pub max4_text: String,
}

impl Max4Text {
	pub fn validate(&self) -> bool {
		if self.max4_text.chars().count() < 1 {
			return false
		}
		if self.max4_text.chars().count() > 4 {
			return false
		}
		return true
	}
}


// Max500Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max500Text {
	#[serde(rename = "$value")]
	pub max500_text: String,
}

impl Max500Text {
	pub fn validate(&self) -> bool {
		if self.max500_text.chars().count() < 1 {
			return false
		}
		if self.max500_text.chars().count() > 500 {
			return false
		}
		return true
	}
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Max70Text {
	#[serde(rename = "$value")]
	pub max70_text: String,
}

impl Max70Text {
	pub fn validate(&self) -> bool {
		if self.max70_text.chars().count() < 1 {
			return false
		}
		if self.max70_text.chars().count() > 70 {
			return false
		}
		return true
	}
}


// MessageIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageIdentification1 {
	#[serde(rename = "Id")]
	pub id: Max35Text,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
}

impl MessageIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.id.validate() { return false }
		return true
	}
}


// NamePrefix1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub enum NamePrefix1Code {
	#[default]
	#[serde(rename = "DOCT")]
	CodeDOCT,
	#[serde(rename = "MIST")]
	CodeMIST,
	#[serde(rename = "MISS")]
	CodeMISS,
	#[serde(rename = "MADM")]
	CodeMADM,
}

impl NamePrefix1Code {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Number {
	#[serde(rename = "$value")]
	pub number: f64,
}

impl Number {
	pub fn validate(&self) -> bool {
		return true
	}
}


// OrganisationIdentification28 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification28 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress6>,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<OrganisationIdentification8>,
	#[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
	pub ctry_of_res: Option<CountryCode>,
	#[serde(rename = "CtctDtls")]
	pub ctct_dtls: ContactDetails2,
}

impl OrganisationIdentification28 {
	pub fn validate(&self) -> bool {
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref pstl_adr_value) = self.pstl_adr { if !pstl_adr_value.validate() { return false; } }
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
		if let Some(ref ctry_of_res_value) = self.ctry_of_res { if !ctry_of_res_value.validate() { return false; } }
		if !self.ctct_dtls.validate() { return false }
		return true
	}
}


// OrganisationIdentification8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification8 {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<AnyBICIdentifier>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<GenericOrganisationIdentification1>>,
}

impl OrganisationIdentification8 {
	pub fn validate(&self) -> bool {
		if let Some(ref any_bic_value) = self.any_bic { if !any_bic_value.validate() { return false; } }
		if let Some(ref othr_vec) = self.othr { for item in othr_vec { if !item.validate() { return false; } } }
		return true
	}
}


// OrganisationIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalOrganisationIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl OrganisationIdentificationSchemeName1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// Party11Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party11Choice {
	#[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
	pub org_id: Option<OrganisationIdentification8>,
	#[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
	pub prvt_id: Option<PersonIdentification5>,
}

impl Party11Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref org_id_value) = self.org_id { if !org_id_value.validate() { return false; } }
		if let Some(ref prvt_id_value) = self.prvt_id { if !prvt_id_value.validate() { return false; } }
		return true
	}
}


// PartyIdentification116 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification116 {
	#[serde(rename = "PtyId")]
	pub pty_id: OrganisationIdentification28,
	#[serde(rename = "LglOrg", skip_serializing_if = "Option::is_none")]
	pub lgl_org: Option<LegalOrganisation1>,
	#[serde(rename = "TaxPty", skip_serializing_if = "Option::is_none")]
	pub tax_pty: Option<TaxParty1>,
}

impl PartyIdentification116 {
	pub fn validate(&self) -> bool {
		if !self.pty_id.validate() { return false }
		if let Some(ref lgl_org_value) = self.lgl_org { if !lgl_org_value.validate() { return false; } }
		if let Some(ref tax_pty_value) = self.tax_pty { if !tax_pty_value.validate() { return false; } }
		return true
	}
}


// PartyIdentification43 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification43 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<Max140Text>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress6>,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Party11Choice>,
	#[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
	pub ctry_of_res: Option<CountryCode>,
	#[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
	pub ctct_dtls: Option<ContactDetails2>,
}

impl PartyIdentification43 {
	pub fn validate(&self) -> bool {
		if let Some(ref nm_value) = self.nm { if !nm_value.validate() { return false; } }
		if let Some(ref pstl_adr_value) = self.pstl_adr { if !pstl_adr_value.validate() { return false; } }
		if let Some(ref id_value) = self.id { if !id_value.validate() { return false; } }
		if let Some(ref ctry_of_res_value) = self.ctry_of_res { if !ctry_of_res_value.validate() { return false; } }
		if let Some(ref ctct_dtls_value) = self.ctct_dtls { if !ctct_dtls_value.validate() { return false; } }
		return true
	}
}


// PartyIdentification72 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification72 {
	#[serde(rename = "PtyId")]
	pub pty_id: PartyIdentification43,
	#[serde(rename = "LglOrg", skip_serializing_if = "Option::is_none")]
	pub lgl_org: Option<LegalOrganisation1>,
	#[serde(rename = "TaxPty", skip_serializing_if = "Option::is_none")]
	pub tax_pty: Option<TaxParty1>,
}

impl PartyIdentification72 {
	pub fn validate(&self) -> bool {
		if !self.pty_id.validate() { return false }
		if let Some(ref lgl_org_value) = self.lgl_org { if !lgl_org_value.validate() { return false; } }
		if let Some(ref tax_pty_value) = self.tax_pty { if !tax_pty_value.validate() { return false; } }
		return true
	}
}


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PercentageRate {
	#[serde(rename = "$value")]
	pub percentage_rate: f64,
}

impl PercentageRate {
	pub fn validate(&self) -> bool {
		return true
	}
}


// Period2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}

impl Period2 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// PersonIdentification5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentification5 {
	#[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
	pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<GenericPersonIdentification1>>,
}

impl PersonIdentification5 {
	pub fn validate(&self) -> bool {
		if let Some(ref dt_and_plc_of_birth_value) = self.dt_and_plc_of_birth { if !dt_and_plc_of_birth_value.validate() { return false; } }
		if let Some(ref othr_vec) = self.othr { for item in othr_vec { if !item.validate() { return false; } } }
		return true
	}
}


// PersonIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<ExternalPersonIdentification1Code>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<Max35Text>,
}

impl PersonIdentificationSchemeName1Choice {
	pub fn validate(&self) -> bool {
		if let Some(ref cd_value) = self.cd { if !cd_value.validate() { return false; } }
		if let Some(ref prtry_value) = self.prtry { if !prtry_value.validate() { return false; } }
		return true
	}
}


// PhoneNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PhoneNumber {
	#[serde(rename = "$value")]
	pub phone_number: String,
}

impl PhoneNumber {
	pub fn validate(&self) -> bool {
		let pattern = Regex::new("\\+[0-9]{1,3}-[0-9()+\\-]{1,30}").unwrap();
		if !pattern.is_match(&self.phone_number) {
			return false
		}
		return true
	}
}


// PostalAddress6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress6 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<AddressType2Code>,
	#[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
	pub dept: Option<Max70Text>,
	#[serde(rename = "SubDept", skip_serializing_if = "Option::is_none")]
	pub sub_dept: Option<Max70Text>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<Max70Text>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<Max16Text>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<Max16Text>,
	#[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
	pub twn_nm: Option<Max35Text>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<Max35Text>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<CountryCode>,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
	pub adr_line: Option<Vec<Max70Text>>,
}

impl PostalAddress6 {
	pub fn validate(&self) -> bool {
		if let Some(ref adr_tp_value) = self.adr_tp { if !adr_tp_value.validate() { return false; } }
		if let Some(ref dept_value) = self.dept { if !dept_value.validate() { return false; } }
		if let Some(ref sub_dept_value) = self.sub_dept { if !sub_dept_value.validate() { return false; } }
		if let Some(ref strt_nm_value) = self.strt_nm { if !strt_nm_value.validate() { return false; } }
		if let Some(ref bldg_nb_value) = self.bldg_nb { if !bldg_nb_value.validate() { return false; } }
		if let Some(ref pst_cd_value) = self.pst_cd { if !pst_cd_value.validate() { return false; } }
		if let Some(ref twn_nm_value) = self.twn_nm { if !twn_nm_value.validate() { return false; } }
		if let Some(ref ctry_sub_dvsn_value) = self.ctry_sub_dvsn { if !ctry_sub_dvsn_value.validate() { return false; } }
		if let Some(ref ctry_value) = self.ctry { if !ctry_value.validate() { return false; } }
		if let Some(ref adr_line_vec) = self.adr_line { for item in adr_line_vec { if !item.validate() { return false; } } }
		return true
	}
}


// SettlementSubTotalCalculatedTax2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementSubTotalCalculatedTax2 {
	#[serde(rename = "TpCd", skip_serializing_if = "Option::is_none")]
	pub tp_cd: Option<Max4Text>,
	#[serde(rename = "ClctdRate", skip_serializing_if = "Option::is_none")]
	pub clctd_rate: Option<f64>,
	#[serde(rename = "BsisAmt", skip_serializing_if = "Option::is_none")]
	pub bsis_amt: Option<Vec<CurrencyAndAmount>>,
	#[serde(rename = "ClctdAmt", skip_serializing_if = "Option::is_none")]
	pub clctd_amt: Option<Vec<CurrencyAndAmount>>,
	#[serde(rename = "XmptnRsnCd", skip_serializing_if = "Option::is_none")]
	pub xmptn_rsn_cd: Option<Max4Text>,
	#[serde(rename = "XmptnRsnTxt", skip_serializing_if = "Option::is_none")]
	pub xmptn_rsn_txt: Option<Max500Text>,
	#[serde(rename = "TaxCcyXchg", skip_serializing_if = "Option::is_none")]
	pub tax_ccy_xchg: Option<CurrencyReference3>,
}

impl SettlementSubTotalCalculatedTax2 {
	pub fn validate(&self) -> bool {
		if let Some(ref tp_cd_value) = self.tp_cd { if !tp_cd_value.validate() { return false; } }
		if let Some(ref bsis_amt_vec) = self.bsis_amt { for item in bsis_amt_vec { if !item.validate() { return false; } } }
		if let Some(ref clctd_amt_vec) = self.clctd_amt { for item in clctd_amt_vec { if !item.validate() { return false; } } }
		if let Some(ref xmptn_rsn_cd_value) = self.xmptn_rsn_cd { if !xmptn_rsn_cd_value.validate() { return false; } }
		if let Some(ref xmptn_rsn_txt_value) = self.xmptn_rsn_txt { if !xmptn_rsn_txt_value.validate() { return false; } }
		if let Some(ref tax_ccy_xchg_value) = self.tax_ccy_xchg { if !tax_ccy_xchg_value.validate() { return false; } }
		return true
	}
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
	pub plc_and_nm: Option<Max350Text>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}

impl SupplementaryData1 {
	pub fn validate(&self) -> bool {
		if let Some(ref plc_and_nm_value) = self.plc_and_nm { if !plc_and_nm_value.validate() { return false; } }
		if !self.envlp.validate() { return false }
		return true
	}
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}

impl SupplementaryDataEnvelope1 {
	pub fn validate(&self) -> bool {
		return true
	}
}


// TaxOrganisationIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxOrganisationIdentification1 {
	#[serde(rename = "Nm")]
	pub nm: Max140Text,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress6>,
	#[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
	pub ctct_dtls: Option<ContactDetails2>,
}

impl TaxOrganisationIdentification1 {
	pub fn validate(&self) -> bool {
		if !self.nm.validate() { return false }
		if let Some(ref pstl_adr_value) = self.pstl_adr { if !pstl_adr_value.validate() { return false; } }
		if let Some(ref ctct_dtls_value) = self.ctct_dtls { if !ctct_dtls_value.validate() { return false; } }
		return true
	}
}


// TaxParty1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxParty1 {
	#[serde(rename = "TaxId", skip_serializing_if = "Option::is_none")]
	pub tax_id: Option<Max35Text>,
	#[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
	pub regn_id: Option<Max35Text>,
	#[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
	pub tax_tp: Option<Max35Text>,
}

impl TaxParty1 {
	pub fn validate(&self) -> bool {
		if let Some(ref tax_id_value) = self.tax_id { if !tax_id_value.validate() { return false; } }
		if let Some(ref regn_id_value) = self.regn_id { if !regn_id_value.validate() { return false; } }
		if let Some(ref tax_tp_value) = self.tax_tp { if !tax_tp_value.validate() { return false; } }
		return true
	}
}


// TaxReport1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxReport1 {
	#[serde(rename = "TaxRptHdr")]
	pub tax_rpt_hdr: GroupHeader69,
	#[serde(rename = "Sellr")]
	pub sellr: PartyIdentification72,
	#[serde(rename = "Buyr", skip_serializing_if = "Option::is_none")]
	pub buyr: Option<PartyIdentification72>,
	#[serde(rename = "TradSttlm")]
	pub trad_sttlm: TradeSettlement2,
	#[serde(rename = "OthrPty", skip_serializing_if = "Option::is_none")]
	pub othr_pty: Option<Vec<PartyIdentification72>>,
	#[serde(rename = "AddtlInf", skip_serializing_if = "Option::is_none")]
	pub addtl_inf: Option<Vec<AdditionalInformation1>>,
	#[serde(rename = "AddtlRef", skip_serializing_if = "Option::is_none")]
	pub addtl_ref: Option<Vec<DocumentGeneralInformation2>>,
	#[serde(rename = "SplmtryData", skip_serializing_if = "Option::is_none")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}

impl TaxReport1 {
	pub fn validate(&self) -> bool {
		if !self.tax_rpt_hdr.validate() { return false }
		if !self.sellr.validate() { return false }
		if let Some(ref buyr_value) = self.buyr { if !buyr_value.validate() { return false; } }
		if !self.trad_sttlm.validate() { return false }
		if let Some(ref othr_pty_vec) = self.othr_pty { for item in othr_pty_vec { if !item.validate() { return false; } } }
		if let Some(ref addtl_inf_vec) = self.addtl_inf { for item in addtl_inf_vec { if !item.validate() { return false; } } }
		if let Some(ref addtl_ref_vec) = self.addtl_ref { for item in addtl_ref_vec { if !item.validate() { return false; } } }
		if let Some(ref splmtry_data_vec) = self.splmtry_data { for item in splmtry_data_vec { if !item.validate() { return false; } } }
		return true
	}
}


// TaxReportHeader1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxReportHeader1 {
	#[serde(rename = "MsgId")]
	pub msg_id: MessageIdentification1,
	#[serde(rename = "NbOfTaxRpts", skip_serializing_if = "Option::is_none")]
	pub nb_of_tax_rpts: Option<f64>,
	#[serde(rename = "TaxAuthrty", skip_serializing_if = "Option::is_none")]
	pub tax_authrty: Option<Vec<TaxOrganisationIdentification1>>,
}

impl TaxReportHeader1 {
	pub fn validate(&self) -> bool {
		if !self.msg_id.validate() { return false }
		if let Some(ref tax_authrty_vec) = self.tax_authrty { for item in tax_authrty_vec { if !item.validate() { return false; } } }
		return true
	}
}


// TradeSettlement2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TradeSettlement2 {
	#[serde(rename = "PmtRef", skip_serializing_if = "Option::is_none")]
	pub pmt_ref: Option<CreditorReferenceInformation2>,
	#[serde(rename = "DueDt", skip_serializing_if = "Option::is_none")]
	pub due_dt: Option<String>,
	#[serde(rename = "DuePyblAmt")]
	pub due_pybl_amt: CurrencyAndAmount,
	#[serde(rename = "InvcCcyXchg", skip_serializing_if = "Option::is_none")]
	pub invc_ccy_xchg: Option<CurrencyReference3>,
	#[serde(rename = "DlvryDt", skip_serializing_if = "Option::is_none")]
	pub dlvry_dt: Option<String>,
	#[serde(rename = "BllgPrd", skip_serializing_if = "Option::is_none")]
	pub bllg_prd: Option<Period2>,
	#[serde(rename = "TaxTtlAmt")]
	pub tax_ttl_amt: CurrencyAndAmount,
	#[serde(rename = "XmptnRsnCd", skip_serializing_if = "Option::is_none")]
	pub xmptn_rsn_cd: Option<Max4Text>,
	#[serde(rename = "XmptnRsn", skip_serializing_if = "Option::is_none")]
	pub xmptn_rsn: Option<Max500Text>,
	#[serde(rename = "SubTtlClctdTax", skip_serializing_if = "Option::is_none")]
	pub sub_ttl_clctd_tax: Option<Vec<SettlementSubTotalCalculatedTax2>>,
	#[serde(rename = "EarlyPmts", skip_serializing_if = "Option::is_none")]
	pub early_pmts: Option<Vec<EarlyPayment1>>,
}

impl TradeSettlement2 {
	pub fn validate(&self) -> bool {
		if let Some(ref pmt_ref_value) = self.pmt_ref { if !pmt_ref_value.validate() { return false; } }
		if !self.due_pybl_amt.validate() { return false }
		if let Some(ref invc_ccy_xchg_value) = self.invc_ccy_xchg { if !invc_ccy_xchg_value.validate() { return false; } }
		if let Some(ref bllg_prd_value) = self.bllg_prd { if !bllg_prd_value.validate() { return false; } }
		if !self.tax_ttl_amt.validate() { return false }
		if let Some(ref xmptn_rsn_cd_value) = self.xmptn_rsn_cd { if !xmptn_rsn_cd_value.validate() { return false; } }
		if let Some(ref xmptn_rsn_value) = self.xmptn_rsn { if !xmptn_rsn_value.validate() { return false; } }
		if let Some(ref sub_ttl_clctd_tax_vec) = self.sub_ttl_clctd_tax { for item in sub_ttl_clctd_tax_vec { if !item.validate() { return false; } } }
		if let Some(ref early_pmts_vec) = self.early_pmts { for item in early_pmts_vec { if !item.validate() { return false; } } }
		return true
	}
}
