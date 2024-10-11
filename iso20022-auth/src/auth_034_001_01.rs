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


// ActiveCurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// AdditionalInformation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AdditionalInformation1 {
	#[serde(rename = "InfTp")]
	pub inf_tp: InformationType1Choice,
	#[serde(rename = "InfVal")]
	pub inf_val: String,
}


// AddressType2Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AddressType2Code {
	#[serde(rename = "AddressType2Code")]
	pub address_type2_code: String,
}


// AnyBICIdentifier ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AnyBICIdentifier {
	#[serde(rename = "AnyBICIdentifier")]
	pub any_bic_identifier: String,
}


// BaseOneRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BaseOneRate {
	#[serde(rename = "BaseOneRate")]
	pub base_one_rate: f64,
}


// BinaryFile1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct BinaryFile1 {
	#[serde(rename = "MIMETp", skip_serializing_if = "Option::is_none")]
	pub mime_tp: Option<String>,
	#[serde(rename = "NcodgTp", skip_serializing_if = "Option::is_none")]
	pub ncodg_tp: Option<String>,
	#[serde(rename = "CharSet", skip_serializing_if = "Option::is_none")]
	pub char_set: Option<String>,
	#[serde(rename = "InclBinryObjct", skip_serializing_if = "Option::is_none")]
	pub incl_binry_objct: Option<String>,
}


// ContactDetails2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ContactDetails2 {
	#[serde(rename = "NmPrfx", skip_serializing_if = "Option::is_none")]
	pub nm_prfx: Option<String>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
	#[serde(rename = "PhneNb", skip_serializing_if = "Option::is_none")]
	pub phne_nb: Option<String>,
	#[serde(rename = "MobNb", skip_serializing_if = "Option::is_none")]
	pub mob_nb: Option<String>,
	#[serde(rename = "FaxNb", skip_serializing_if = "Option::is_none")]
	pub fax_nb: Option<String>,
	#[serde(rename = "EmailAdr", skip_serializing_if = "Option::is_none")]
	pub email_adr: Option<String>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<String>,
}


// CountryCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CountryCode {
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// CreditorReferenceInformation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceInformation2 {
	#[serde(rename = "Tp", skip_serializing_if = "Option::is_none")]
	pub tp: Option<CreditorReferenceType2>,
	#[serde(rename = "Ref", skip_serializing_if = "Option::is_none")]
	pub ref_attr: Option<String>,
}


// CreditorReferenceType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// CreditorReferenceType2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CreditorReferenceType2 {
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: CreditorReferenceType1Choice,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}


// CurrencyAndAmountSimpleType ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CurrencyAndAmountSimpleType {
	#[serde(rename = "CurrencyAndAmount_SimpleType")]
	pub currency_and_amount_simple_type: f64,
}


// CurrencyAndAmount ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// CurrencyCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CurrencyCode {
	#[serde(rename = "CurrencyCode")]
	pub currency_code: String,
}


// CurrencyReference3 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct CurrencyReference3 {
	#[serde(rename = "TrgtCcy")]
	pub trgt_ccy: String,
	#[serde(rename = "SrcCcy")]
	pub src_ccy: String,
	#[serde(rename = "XchgRateInf", skip_serializing_if = "Option::is_none")]
	pub xchg_rate_inf: Option<Vec<ExchangeRateInformation1>>,
}


// DateAndPlaceOfBirth ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DateAndPlaceOfBirth {
	#[serde(rename = "BirthDt")]
	pub birth_dt: String,
	#[serde(rename = "PrvcOfBirth", skip_serializing_if = "Option::is_none")]
	pub prvc_of_birth: Option<String>,
	#[serde(rename = "CityOfBirth")]
	pub city_of_birth: String,
	#[serde(rename = "CtryOfBirth")]
	pub ctry_of_birth: String,
}


// DocumentGeneralInformation2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentGeneralInformation2 {
	#[serde(rename = "DocTp")]
	pub doc_tp: String,
	#[serde(rename = "DocNb")]
	pub doc_nb: String,
	#[serde(rename = "SndrRcvrSeqId", skip_serializing_if = "Option::is_none")]
	pub sndr_rcvr_seq_id: Option<String>,
	#[serde(rename = "IsseDt", skip_serializing_if = "Option::is_none")]
	pub isse_dt: Option<String>,
	#[serde(rename = "URL", skip_serializing_if = "Option::is_none")]
	pub url: Option<String>,
	#[serde(rename = "AttchdBinryFile", skip_serializing_if = "Option::is_none")]
	pub attchd_binry_file: Option<Vec<BinaryFile1>>,
}


// DocumentType3Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct DocumentType3Code {
	#[serde(rename = "DocumentType3Code")]
	pub document_type3_code: String,
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


// EarlyPaymentsVAT1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EarlyPaymentsVAT1 {
	#[serde(rename = "TaxRate")]
	pub tax_rate: f64,
	#[serde(rename = "DscntTaxTp")]
	pub dscnt_tax_tp: String,
	#[serde(rename = "DscntTaxAmt")]
	pub dscnt_tax_amt: CurrencyAndAmount,
}


// ExchangeRateInformation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExchangeRateInformation1 {
	#[serde(rename = "XchgRate", skip_serializing_if = "Option::is_none")]
	pub xchg_rate: Option<f64>,
	#[serde(rename = "RateTp", skip_serializing_if = "Option::is_none")]
	pub rate_tp: Option<String>,
	#[serde(rename = "CtrctId", skip_serializing_if = "Option::is_none")]
	pub ctrct_id: Option<String>,
}


// ExchangeRateType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExchangeRateType1Code {
	#[serde(rename = "ExchangeRateType1Code")]
	pub exchange_rate_type1_code: String,
}


// ExternalDocumentType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalDocumentType1Code {
	#[serde(rename = "ExternalDocumentType1Code")]
	pub external_document_type1_code: String,
}


// ExternalOrganisationIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalOrganisationIdentification1Code {
	#[serde(rename = "ExternalOrganisationIdentification1Code")]
	pub external_organisation_identification1_code: String,
}


// ExternalPersonIdentification1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ExternalPersonIdentification1Code {
	#[serde(rename = "ExternalPersonIdentification1Code")]
	pub external_person_identification1_code: String,
}


// GenericOrganisationIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericOrganisationIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}


// GenericPersonIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GenericPersonIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "SchmeNm", skip_serializing_if = "Option::is_none")]
	pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr", skip_serializing_if = "Option::is_none")]
	pub issr: Option<String>,
}


// GroupHeader69 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct GroupHeader69 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "IssdDt")]
	pub issd_dt: String,
	#[serde(rename = "RptCtgy")]
	pub rpt_ctgy: String,
	#[serde(rename = "TaxRptPurp")]
	pub tax_rpt_purp: String,
	#[serde(rename = "OrgnlId", skip_serializing_if = "Option::is_none")]
	pub orgnl_id: Option<String>,
	#[serde(rename = "SellrTaxRprtv", skip_serializing_if = "Option::is_none")]
	pub sellr_tax_rprtv: Option<PartyIdentification116>,
	#[serde(rename = "BuyrTaxRprtv", skip_serializing_if = "Option::is_none")]
	pub buyr_tax_rprtv: Option<PartyIdentification116>,
	#[serde(rename = "LangCd", skip_serializing_if = "Option::is_none")]
	pub lang_cd: Option<String>,
}


// ISODate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// InformationType1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InformationType1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// InformationType1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct InformationType1Code {
	#[serde(rename = "InformationType1Code")]
	pub information_type1_code: String,
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


// LanguageCode ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LanguageCode {
	#[serde(rename = "LanguageCode")]
	pub language_code: String,
}


// LegalOrganisation1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct LegalOrganisation1 {
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
}


// Max100KBinary ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max100KBinary {
	#[serde(rename = "Max100KBinary")]
	pub max100_k_binary: String,
}


// Max140Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max140Text {
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max16Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max16Text {
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
}


// Max2048Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max2048Text {
	#[serde(rename = "Max2048Text")]
	pub max2048_text: String,
}


// Max256Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max256Text {
	#[serde(rename = "Max256Text")]
	pub max256_text: String,
}


// Max350Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max350Text {
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max35Text {
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Max4Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max4Text {
	#[serde(rename = "Max4Text")]
	pub max4_text: String,
}


// Max500Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max500Text {
	#[serde(rename = "Max500Text")]
	pub max500_text: String,
}


// Max70Text ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Max70Text {
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// MessageIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct MessageIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
}


// NamePrefix1Code ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct NamePrefix1Code {
	#[serde(rename = "NamePrefix1Code")]
	pub name_prefix1_code: String,
}


// Number ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
}


// OrganisationIdentification28 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification28 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress6>,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<OrganisationIdentification8>,
	#[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
	pub ctry_of_res: Option<String>,
	#[serde(rename = "CtctDtls")]
	pub ctct_dtls: ContactDetails2,
}


// OrganisationIdentification8 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentification8 {
	#[serde(rename = "AnyBIC", skip_serializing_if = "Option::is_none")]
	pub any_bic: Option<String>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<GenericOrganisationIdentification1>>,
}


// OrganisationIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct OrganisationIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// Party11Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Party11Choice {
	#[serde(rename = "OrgId", skip_serializing_if = "Option::is_none")]
	pub org_id: Option<OrganisationIdentification8>,
	#[serde(rename = "PrvtId", skip_serializing_if = "Option::is_none")]
	pub prvt_id: Option<PersonIdentification5>,
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


// PartyIdentification43 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PartyIdentification43 {
	#[serde(rename = "Nm", skip_serializing_if = "Option::is_none")]
	pub nm: Option<String>,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress6>,
	#[serde(rename = "Id", skip_serializing_if = "Option::is_none")]
	pub id: Option<Party11Choice>,
	#[serde(rename = "CtryOfRes", skip_serializing_if = "Option::is_none")]
	pub ctry_of_res: Option<String>,
	#[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
	pub ctct_dtls: Option<ContactDetails2>,
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


// PercentageRate ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// Period2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct Period2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// PersonIdentification5 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentification5 {
	#[serde(rename = "DtAndPlcOfBirth", skip_serializing_if = "Option::is_none")]
	pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth>,
	#[serde(rename = "Othr", skip_serializing_if = "Option::is_none")]
	pub othr: Option<Vec<GenericPersonIdentification1>>,
}


// PersonIdentificationSchemeName1Choice ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PersonIdentificationSchemeName1Choice {
	#[serde(rename = "Cd", skip_serializing_if = "Option::is_none")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry", skip_serializing_if = "Option::is_none")]
	pub prtry: Option<String>,
}


// PhoneNumber ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PhoneNumber {
	#[serde(rename = "PhoneNumber")]
	pub phone_number: String,
}


// PostalAddress6 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct PostalAddress6 {
	#[serde(rename = "AdrTp", skip_serializing_if = "Option::is_none")]
	pub adr_tp: Option<String>,
	#[serde(rename = "Dept", skip_serializing_if = "Option::is_none")]
	pub dept: Option<String>,
	#[serde(rename = "SubDept", skip_serializing_if = "Option::is_none")]
	pub sub_dept: Option<String>,
	#[serde(rename = "StrtNm", skip_serializing_if = "Option::is_none")]
	pub strt_nm: Option<String>,
	#[serde(rename = "BldgNb", skip_serializing_if = "Option::is_none")]
	pub bldg_nb: Option<String>,
	#[serde(rename = "PstCd", skip_serializing_if = "Option::is_none")]
	pub pst_cd: Option<String>,
	#[serde(rename = "TwnNm", skip_serializing_if = "Option::is_none")]
	pub twn_nm: Option<String>,
	#[serde(rename = "CtrySubDvsn", skip_serializing_if = "Option::is_none")]
	pub ctry_sub_dvsn: Option<String>,
	#[serde(rename = "Ctry", skip_serializing_if = "Option::is_none")]
	pub ctry: Option<String>,
	#[serde(rename = "AdrLine", skip_serializing_if = "Option::is_none")]
	pub adr_line: Option<Vec<String>>,
}


// SettlementSubTotalCalculatedTax2 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SettlementSubTotalCalculatedTax2 {
	#[serde(rename = "TpCd", skip_serializing_if = "Option::is_none")]
	pub tp_cd: Option<String>,
	#[serde(rename = "ClctdRate", skip_serializing_if = "Option::is_none")]
	pub clctd_rate: Option<f64>,
	#[serde(rename = "BsisAmt", skip_serializing_if = "Option::is_none")]
	pub bsis_amt: Option<Vec<CurrencyAndAmount>>,
	#[serde(rename = "ClctdAmt", skip_serializing_if = "Option::is_none")]
	pub clctd_amt: Option<Vec<CurrencyAndAmount>>,
	#[serde(rename = "XmptnRsnCd", skip_serializing_if = "Option::is_none")]
	pub xmptn_rsn_cd: Option<String>,
	#[serde(rename = "XmptnRsnTxt", skip_serializing_if = "Option::is_none")]
	pub xmptn_rsn_txt: Option<String>,
	#[serde(rename = "TaxCcyXchg", skip_serializing_if = "Option::is_none")]
	pub tax_ccy_xchg: Option<CurrencyReference3>,
}


// SupplementaryData1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm", skip_serializing_if = "Option::is_none")]
	pub plc_and_nm: Option<String>,
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}


// TaxOrganisationIdentification1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxOrganisationIdentification1 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[serde(rename = "PstlAdr", skip_serializing_if = "Option::is_none")]
	pub pstl_adr: Option<PostalAddress6>,
	#[serde(rename = "CtctDtls", skip_serializing_if = "Option::is_none")]
	pub ctct_dtls: Option<ContactDetails2>,
}


// TaxParty1 ...
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct TaxParty1 {
	#[serde(rename = "TaxId", skip_serializing_if = "Option::is_none")]
	pub tax_id: Option<String>,
	#[serde(rename = "RegnId", skip_serializing_if = "Option::is_none")]
	pub regn_id: Option<String>,
	#[serde(rename = "TaxTp", skip_serializing_if = "Option::is_none")]
	pub tax_tp: Option<String>,
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
	pub xmptn_rsn_cd: Option<String>,
	#[serde(rename = "XmptnRsn", skip_serializing_if = "Option::is_none")]
	pub xmptn_rsn: Option<String>,
	#[serde(rename = "SubTtlClctdTax", skip_serializing_if = "Option::is_none")]
	pub sub_ttl_clctd_tax: Option<Vec<SettlementSubTotalCalculatedTax2>>,
	#[serde(rename = "EarlyPmts", skip_serializing_if = "Option::is_none")]
	pub early_pmts: Option<Vec<EarlyPayment1>>,
}
