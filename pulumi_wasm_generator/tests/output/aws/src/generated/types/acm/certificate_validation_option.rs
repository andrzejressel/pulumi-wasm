#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CertificateValidationOption {
    /// Fully qualified domain name (FQDN) in the certificate.
    #[builder(into)]
    #[serde(rename = "domainName")]
    pub r#domain_name: Box<String>,
    /// Domain name that you want ACM to use to send you validation emails. This domain name is the suffix of the email addresses that you want ACM to use. This must be the same as the `domain_name` value or a superdomain of the `domain_name` value. For example, if you request a certificate for `"testing.example.com"`, you can specify `"example.com"` for this value.
    #[builder(into)]
    #[serde(rename = "validationDomain")]
    pub r#validation_domain: Box<String>,
}