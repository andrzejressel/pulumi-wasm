#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetCertificateCertificatePolicyX509CertificatePropertySubjectAlternativeName {
    /// A list of alternative DNS names (FQDNs) identified by the Certificate.
    #[builder(into)]
    #[serde(rename = "dnsNames")]
    pub r#dns_names: Box<Vec<String>>,
    /// A list of email addresses identified by this Certificate.
    #[builder(into)]
    #[serde(rename = "emails")]
    pub r#emails: Box<Vec<String>>,
    /// A list of User Principal Names identified by the Certificate.
    #[builder(into)]
    #[serde(rename = "upns")]
    pub r#upns: Box<Vec<String>>,
}