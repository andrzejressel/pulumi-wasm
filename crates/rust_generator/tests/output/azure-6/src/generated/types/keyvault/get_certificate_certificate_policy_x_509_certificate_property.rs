#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCertificateCertificatePolicyX509CertificateProperty {
    /// A list of Extended/Enhanced Key Usages.
    #[builder(into)]
    #[serde(rename = "extendedKeyUsages")]
    pub r#extended_key_usages: Box<Vec<String>>,
    /// A list of uses associated with this Key.
    #[builder(into)]
    #[serde(rename = "keyUsages")]
    pub r#key_usages: Box<Vec<String>>,
    /// The Certificate's Subject.
    #[builder(into)]
    #[serde(rename = "subject")]
    pub r#subject: Box<String>,
    /// A `subject_alternative_names` block as defined below.
    #[builder(into)]
    #[serde(rename = "subjectAlternativeNames")]
    pub r#subject_alternative_names: Box<Vec<super::super::types::keyvault::GetCertificateCertificatePolicyX509CertificatePropertySubjectAlternativeName>>,
    /// The Certificates Validity Period in Months.
    #[builder(into)]
    #[serde(rename = "validityInMonths")]
    pub r#validity_in_months: Box<i32>,
}
