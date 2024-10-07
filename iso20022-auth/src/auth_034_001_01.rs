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


// ActiveCurrencyCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ActiveCurrencyCode {
	#[validate(pattern = "[A-Z]{3,3}")]
	#[serde(rename = "ActiveCurrencyCode")]
	pub active_currency_code: String,
}


// AdditionalInformation1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AdditionalInformation1 {
	#[validate]
	#[serde(rename = "InfTp")]
	pub inf_tp: InformationType1Choice,
	#[serde(rename = "InfVal")]
	pub inf_val: String,
}


// AddressType2Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AddressType2Code {
	#[validate(enumerate = ["ADDR", "PBOX", "HOME", "BIZZ", "MLTO", "DLVY"])]
	#[serde(rename = "AddressType2Code")]
	pub address_type2_code: String,
}


// AnyBICIdentifier ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct AnyBICIdentifier {
	#[validate(pattern = "[A-Z]{6,6}[A-Z2-9][A-NP-Z0-9]([A-Z0-9]{3,3}){0,1}")]
	#[serde(rename = "AnyBICIdentifier")]
	pub any_bic_identifier: String,
}


// BaseOneRate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BaseOneRate {
	#[serde(rename = "BaseOneRate")]
	pub base_one_rate: f64,
}


// BinaryFile1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct BinaryFile1 {
	#[serde(rename = "MIMETp")]
	pub mime_tp: Option<String>,
	#[serde(rename = "NcodgTp")]
	pub ncodg_tp: Option<String>,
	#[serde(rename = "CharSet")]
	pub char_set: Option<String>,
	#[serde(rename = "InclBinryObjct")]
	pub incl_binry_objct: Option<String>,
}


// ContactDetails2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ContactDetails2 {
	#[serde(rename = "NmPrfx")]
	pub nm_prfx: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[serde(rename = "PhneNb")]
	pub phne_nb: Option<String>,
	#[serde(rename = "MobNb")]
	pub mob_nb: Option<String>,
	#[serde(rename = "FaxNb")]
	pub fax_nb: Option<String>,
	#[serde(rename = "EmailAdr")]
	pub email_adr: Option<String>,
	#[serde(rename = "Othr")]
	pub othr: Option<String>,
}


// CountryCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CountryCode {
	#[validate(pattern = "[A-Z]{2,2}")]
	#[serde(rename = "CountryCode")]
	pub country_code: String,
}


// CreditorReferenceInformation2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CreditorReferenceInformation2 {
	#[validate]
	#[serde(rename = "Tp")]
	pub tp: Option<CreditorReferenceType2>,
	#[serde(rename = "Ref")]
	pub ref_attr: Option<String>,
}


// CreditorReferenceType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CreditorReferenceType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// CreditorReferenceType2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CreditorReferenceType2 {
	#[validate]
	#[serde(rename = "CdOrPrtry")]
	pub cd_or_prtry: CreditorReferenceType1Choice,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// CurrencyAndAmountSimpleType ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CurrencyAndAmountSimpleType {
	#[serde(rename = "CurrencyAndAmount_SimpleType")]
	pub currency_and_amount_simple_type: f64,
}


// CurrencyAndAmount ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CurrencyAndAmount {
	#[serde(rename = "Ccy")]
	pub ccy: String,
	#[serde(rename = "$value")]
	pub value: f64,
}


// CurrencyCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CurrencyCode {
	#[validate(pattern = "[A-Z]{3,3}")]
	#[serde(rename = "CurrencyCode")]
	pub currency_code: String,
}


// CurrencyReference3 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct CurrencyReference3 {
	#[serde(rename = "TrgtCcy")]
	pub trgt_ccy: String,
	#[serde(rename = "SrcCcy")]
	pub src_ccy: String,
	#[validate]
	#[serde(rename = "XchgRateInf")]
	pub xchg_rate_inf: Option<Vec<ExchangeRateInformation1>>,
}


// DateAndPlaceOfBirth ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DateAndPlaceOfBirth {
	#[serde(rename = "BirthDt")]
	pub birth_dt: String,
	#[serde(rename = "PrvcOfBirth")]
	pub prvc_of_birth: Option<String>,
	#[serde(rename = "CityOfBirth")]
	pub city_of_birth: String,
	#[serde(rename = "CtryOfBirth")]
	pub ctry_of_birth: String,
}


// DocumentGeneralInformation2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DocumentGeneralInformation2 {
	#[serde(rename = "DocTp")]
	pub doc_tp: String,
	#[serde(rename = "DocNb")]
	pub doc_nb: String,
	#[serde(rename = "SndrRcvrSeqId")]
	pub sndr_rcvr_seq_id: Option<String>,
	#[serde(rename = "IsseDt")]
	pub isse_dt: Option<String>,
	#[serde(rename = "URL")]
	pub url: Option<String>,
	#[validate]
	#[serde(rename = "AttchdBinryFile")]
	pub attchd_binry_file: Option<Vec<BinaryFile1>>,
}


// DocumentType3Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct DocumentType3Code {
	#[validate(enumerate = ["RADM", "RPIN", "FXDR", "DISP", "PUOR", "SCOR"])]
	#[serde(rename = "DocumentType3Code")]
	pub document_type3_code: String,
}


// EarlyPayment1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EarlyPayment1 {
	#[serde(rename = "EarlyPmtDt")]
	pub early_pmt_dt: String,
	#[serde(rename = "DscntPct")]
	pub dscnt_pct: f64,
	#[validate]
	#[serde(rename = "DscntAmt")]
	pub dscnt_amt: CurrencyAndAmount,
	#[validate]
	#[serde(rename = "EarlyPmtTaxSpcfctn")]
	pub early_pmt_tax_spcfctn: Option<Vec<EarlyPaymentsVAT1>>,
	#[validate]
	#[serde(rename = "EarlyPmtTaxTtl")]
	pub early_pmt_tax_ttl: Option<CurrencyAndAmount>,
	#[validate]
	#[serde(rename = "DuePyblAmtWthEarlyPmt")]
	pub due_pybl_amt_wth_early_pmt: Option<CurrencyAndAmount>,
}


// EarlyPaymentsVAT1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct EarlyPaymentsVAT1 {
	#[serde(rename = "TaxRate")]
	pub tax_rate: f64,
	#[serde(rename = "DscntTaxTp")]
	pub dscnt_tax_tp: String,
	#[validate]
	#[serde(rename = "DscntTaxAmt")]
	pub dscnt_tax_amt: CurrencyAndAmount,
}


// ExchangeRateInformation1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExchangeRateInformation1 {
	#[serde(rename = "XchgRate")]
	pub xchg_rate: Option<f64>,
	#[serde(rename = "RateTp")]
	pub rate_tp: Option<String>,
	#[serde(rename = "CtrctId")]
	pub ctrct_id: Option<String>,
}


// ExchangeRateType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExchangeRateType1Code {
	#[validate(enumerate = ["SPOT", "SALE", "AGRD"])]
	#[serde(rename = "ExchangeRateType1Code")]
	pub exchange_rate_type1_code: String,
}


// ExternalDocumentType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalDocumentType1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalDocumentType1Code")]
	pub external_document_type1_code: String,
}


// ExternalOrganisationIdentification1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalOrganisationIdentification1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalOrganisationIdentification1Code")]
	pub external_organisation_identification1_code: String,
}


// ExternalPersonIdentification1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ExternalPersonIdentification1Code {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "ExternalPersonIdentification1Code")]
	pub external_person_identification1_code: String,
}


// GenericOrganisationIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericOrganisationIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[validate]
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<OrganisationIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GenericPersonIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GenericPersonIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[validate]
	#[serde(rename = "SchmeNm")]
	pub schme_nm: Option<PersonIdentificationSchemeName1Choice>,
	#[serde(rename = "Issr")]
	pub issr: Option<String>,
}


// GroupHeader69 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct GroupHeader69 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "IssdDt")]
	pub issd_dt: String,
	#[serde(rename = "RptCtgy")]
	pub rpt_ctgy: String,
	#[serde(rename = "TaxRptPurp")]
	pub tax_rpt_purp: String,
	#[serde(rename = "OrgnlId")]
	pub orgnl_id: Option<String>,
	#[validate]
	#[serde(rename = "SellrTaxRprtv")]
	pub sellr_tax_rprtv: Option<PartyIdentification116>,
	#[validate]
	#[serde(rename = "BuyrTaxRprtv")]
	pub buyr_tax_rprtv: Option<PartyIdentification116>,
	#[serde(rename = "LangCd")]
	pub lang_cd: Option<String>,
}


// ISODate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISODate {
	#[serde(rename = "ISODate")]
	pub iso_date: String,
}


// ISODateTime ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct ISODateTime {
	#[serde(rename = "ISODateTime")]
	pub iso_date_time: String,
}


// InformationType1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InformationType1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// InformationType1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InformationType1Code {
	#[validate(enumerate = ["INST", "RELY"])]
	#[serde(rename = "InformationType1Code")]
	pub information_type1_code: String,
}


// InvoiceTaxReportV01 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct InvoiceTaxReportV01 {
	#[validate]
	#[serde(rename = "InvcTaxRptHdr")]
	pub invc_tax_rpt_hdr: TaxReportHeader1,
	#[validate]
	#[serde(rename = "TaxRpt")]
	pub tax_rpt: Vec<TaxReport1>,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// LanguageCode ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LanguageCode {
	#[serde(rename = "LanguageCode")]
	pub language_code: String,
}


// LegalOrganisation1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct LegalOrganisation1 {
	#[serde(rename = "Id")]
	pub id: Option<String>,
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
}


// Max100KBinary ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max100KBinary {
	#[validate(min_length = 1)]
	#[validate(max_length = 102400)]
	#[serde(rename = "Max100KBinary")]
	pub max100_k_binary: String,
}


// Max140Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max140Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 140)]
	#[serde(rename = "Max140Text")]
	pub max140_text: String,
}


// Max16Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max16Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 16)]
	#[serde(rename = "Max16Text")]
	pub max16_text: String,
}


// Max2048Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max2048Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 2048)]
	#[serde(rename = "Max2048Text")]
	pub max2048_text: String,
}


// Max256Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max256Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 256)]
	#[serde(rename = "Max256Text")]
	pub max256_text: String,
}


// Max350Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max350Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 350)]
	#[serde(rename = "Max350Text")]
	pub max350_text: String,
}


// Max35Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max35Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 35)]
	#[serde(rename = "Max35Text")]
	pub max35_text: String,
}


// Max4Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max4Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 4)]
	#[serde(rename = "Max4Text")]
	pub max4_text: String,
}


// Max500Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max500Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 500)]
	#[serde(rename = "Max500Text")]
	pub max500_text: String,
}


// Max70Text ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Max70Text {
	#[validate(min_length = 1)]
	#[validate(max_length = 70)]
	#[serde(rename = "Max70Text")]
	pub max70_text: String,
}


// MessageIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct MessageIdentification1 {
	#[serde(rename = "Id")]
	pub id: String,
	#[serde(rename = "CreDtTm")]
	pub cre_dt_tm: String,
}


// NamePrefix1Code ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct NamePrefix1Code {
	#[validate(enumerate = ["DOCT", "MIST", "MISS", "MADM"])]
	#[serde(rename = "NamePrefix1Code")]
	pub name_prefix1_code: String,
}


// Number ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Number {
	#[serde(rename = "Number")]
	pub number: f64,
}


// OrganisationIdentification28 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OrganisationIdentification28 {
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[validate]
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress6>,
	#[validate]
	#[serde(rename = "Id")]
	pub id: Option<OrganisationIdentification8>,
	#[serde(rename = "CtryOfRes")]
	pub ctry_of_res: Option<String>,
	#[validate]
	#[serde(rename = "CtctDtls")]
	pub ctct_dtls: ContactDetails2,
}


// OrganisationIdentification8 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OrganisationIdentification8 {
	#[serde(rename = "AnyBIC")]
	pub any_bic: Option<String>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<GenericOrganisationIdentification1>>,
}


// OrganisationIdentificationSchemeName1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct OrganisationIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// Party11Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Party11Choice {
	#[validate]
	#[serde(rename = "OrgId")]
	pub org_id: Option<OrganisationIdentification8>,
	#[validate]
	#[serde(rename = "PrvtId")]
	pub prvt_id: Option<PersonIdentification5>,
}


// PartyIdentification116 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification116 {
	#[validate]
	#[serde(rename = "PtyId")]
	pub pty_id: OrganisationIdentification28,
	#[validate]
	#[serde(rename = "LglOrg")]
	pub lgl_org: Option<LegalOrganisation1>,
	#[validate]
	#[serde(rename = "TaxPty")]
	pub tax_pty: Option<TaxParty1>,
}


// PartyIdentification43 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification43 {
	#[serde(rename = "Nm")]
	pub nm: Option<String>,
	#[validate]
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress6>,
	#[validate]
	#[serde(rename = "Id")]
	pub id: Option<Party11Choice>,
	#[serde(rename = "CtryOfRes")]
	pub ctry_of_res: Option<String>,
	#[validate]
	#[serde(rename = "CtctDtls")]
	pub ctct_dtls: Option<ContactDetails2>,
}


// PartyIdentification72 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PartyIdentification72 {
	#[validate]
	#[serde(rename = "PtyId")]
	pub pty_id: PartyIdentification43,
	#[validate]
	#[serde(rename = "LglOrg")]
	pub lgl_org: Option<LegalOrganisation1>,
	#[validate]
	#[serde(rename = "TaxPty")]
	pub tax_pty: Option<TaxParty1>,
}


// PercentageRate ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PercentageRate {
	#[serde(rename = "PercentageRate")]
	pub percentage_rate: f64,
}


// Period2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct Period2 {
	#[serde(rename = "FrDt")]
	pub fr_dt: String,
	#[serde(rename = "ToDt")]
	pub to_dt: String,
}


// PersonIdentification5 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PersonIdentification5 {
	#[validate]
	#[serde(rename = "DtAndPlcOfBirth")]
	pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth>,
	#[validate]
	#[serde(rename = "Othr")]
	pub othr: Option<Vec<GenericPersonIdentification1>>,
}


// PersonIdentificationSchemeName1Choice ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PersonIdentificationSchemeName1Choice {
	#[serde(rename = "Cd")]
	pub cd: Option<String>,
	#[serde(rename = "Prtry")]
	pub prtry: Option<String>,
}


// PhoneNumber ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PhoneNumber {
	#[validate(pattern = "\\+[0-9]{1,3}-[0-9()+\\-]{1,30}")]
	#[serde(rename = "PhoneNumber")]
	pub phone_number: String,
}


// PostalAddress6 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct PostalAddress6 {
	#[serde(rename = "AdrTp")]
	pub adr_tp: Option<String>,
	#[serde(rename = "Dept")]
	pub dept: Option<String>,
	#[serde(rename = "SubDept")]
	pub sub_dept: Option<String>,
	#[serde(rename = "StrtNm")]
	pub strt_nm: Option<String>,
	#[serde(rename = "BldgNb")]
	pub bldg_nb: Option<String>,
	#[serde(rename = "PstCd")]
	pub pst_cd: Option<String>,
	#[serde(rename = "TwnNm")]
	pub twn_nm: Option<String>,
	#[serde(rename = "CtrySubDvsn")]
	pub ctry_sub_dvsn: Option<String>,
	#[serde(rename = "Ctry")]
	pub ctry: Option<String>,
	#[serde(rename = "AdrLine")]
	pub adr_line: Option<Vec<String>>,
}


// SettlementSubTotalCalculatedTax2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SettlementSubTotalCalculatedTax2 {
	#[serde(rename = "TpCd")]
	pub tp_cd: Option<String>,
	#[serde(rename = "ClctdRate")]
	pub clctd_rate: Option<f64>,
	#[validate]
	#[serde(rename = "BsisAmt")]
	pub bsis_amt: Option<Vec<CurrencyAndAmount>>,
	#[validate]
	#[serde(rename = "ClctdAmt")]
	pub clctd_amt: Option<Vec<CurrencyAndAmount>>,
	#[serde(rename = "XmptnRsnCd")]
	pub xmptn_rsn_cd: Option<String>,
	#[serde(rename = "XmptnRsnTxt")]
	pub xmptn_rsn_txt: Option<String>,
	#[validate]
	#[serde(rename = "TaxCcyXchg")]
	pub tax_ccy_xchg: Option<CurrencyReference3>,
}


// SupplementaryData1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SupplementaryData1 {
	#[serde(rename = "PlcAndNm")]
	pub plc_and_nm: Option<String>,
	#[validate]
	#[serde(rename = "Envlp")]
	pub envlp: SupplementaryDataEnvelope1,
}


// SupplementaryDataEnvelope1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct SupplementaryDataEnvelope1 {
}


// TaxOrganisationIdentification1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TaxOrganisationIdentification1 {
	#[serde(rename = "Nm")]
	pub nm: String,
	#[validate]
	#[serde(rename = "PstlAdr")]
	pub pstl_adr: Option<PostalAddress6>,
	#[validate]
	#[serde(rename = "CtctDtls")]
	pub ctct_dtls: Option<ContactDetails2>,
}


// TaxParty1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TaxParty1 {
	#[serde(rename = "TaxId")]
	pub tax_id: Option<String>,
	#[serde(rename = "RegnId")]
	pub regn_id: Option<String>,
	#[serde(rename = "TaxTp")]
	pub tax_tp: Option<String>,
}


// TaxReport1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TaxReport1 {
	#[validate]
	#[serde(rename = "TaxRptHdr")]
	pub tax_rpt_hdr: GroupHeader69,
	#[validate]
	#[serde(rename = "Sellr")]
	pub sellr: PartyIdentification72,
	#[validate]
	#[serde(rename = "Buyr")]
	pub buyr: Option<PartyIdentification72>,
	#[validate]
	#[serde(rename = "TradSttlm")]
	pub trad_sttlm: TradeSettlement2,
	#[validate]
	#[serde(rename = "OthrPty")]
	pub othr_pty: Option<Vec<PartyIdentification72>>,
	#[validate]
	#[serde(rename = "AddtlInf")]
	pub addtl_inf: Option<Vec<AdditionalInformation1>>,
	#[validate]
	#[serde(rename = "AddtlRef")]
	pub addtl_ref: Option<Vec<DocumentGeneralInformation2>>,
	#[validate]
	#[serde(rename = "SplmtryData")]
	pub splmtry_data: Option<Vec<SupplementaryData1>>,
}


// TaxReportHeader1 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TaxReportHeader1 {
	#[validate]
	#[serde(rename = "MsgId")]
	pub msg_id: MessageIdentification1,
	#[serde(rename = "NbOfTaxRpts")]
	pub nb_of_tax_rpts: Option<f64>,
	#[validate]
	#[serde(rename = "TaxAuthrty")]
	pub tax_authrty: Option<Vec<TaxOrganisationIdentification1>>,
}


// TradeSettlement2 ...
#[derive(Debug, PartialEq, Clone, Validate, Serialize, Deserialize)]
pub struct TradeSettlement2 {
	#[validate]
	#[serde(rename = "PmtRef")]
	pub pmt_ref: Option<CreditorReferenceInformation2>,
	#[serde(rename = "DueDt")]
	pub due_dt: Option<String>,
	#[validate]
	#[serde(rename = "DuePyblAmt")]
	pub due_pybl_amt: CurrencyAndAmount,
	#[validate]
	#[serde(rename = "InvcCcyXchg")]
	pub invc_ccy_xchg: Option<CurrencyReference3>,
	#[serde(rename = "DlvryDt")]
	pub dlvry_dt: Option<String>,
	#[validate]
	#[serde(rename = "BllgPrd")]
	pub bllg_prd: Option<Period2>,
	#[validate]
	#[serde(rename = "TaxTtlAmt")]
	pub tax_ttl_amt: CurrencyAndAmount,
	#[serde(rename = "XmptnRsnCd")]
	pub xmptn_rsn_cd: Option<String>,
	#[serde(rename = "XmptnRsn")]
	pub xmptn_rsn: Option<String>,
	#[validate]
	#[serde(rename = "SubTtlClctdTax")]
	pub sub_ttl_clctd_tax: Option<Vec<SettlementSubTotalCalculatedTax2>>,
	#[validate]
	#[serde(rename = "EarlyPmts")]
	pub early_pmts: Option<Vec<EarlyPayment1>>,
}
