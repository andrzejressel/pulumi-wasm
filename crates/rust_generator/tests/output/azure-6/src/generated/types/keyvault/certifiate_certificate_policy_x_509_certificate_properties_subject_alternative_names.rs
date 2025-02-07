#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CertifiateCertificatePolicyX509CertificatePropertiesSubjectAlternativeNames {
    /// A list of alternative DNS names (FQDNs) identified by the Certificate.
    #[builder(into, default)]
    #[serde(rename = "dnsNames")]
    pub r#dns_names: Box<Option<Vec<String>>>,
    /// A list of email addresses identified by this Certificate.
    #[builder(into, default)]
    #[serde(rename = "emails")]
    pub r#emails: Box<Option<Vec<String>>>,
    /// A list of User Principal Names identified by the Certificate.
    #[builder(into, default)]
    #[serde(rename = "upns")]
    pub r#upns: Box<Option<Vec<String>>>,
}
