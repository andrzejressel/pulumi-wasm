#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TrustConfigAllowlistedCertificate {
    /// PEM certificate that is allowlisted. The certificate can be up to 5k bytes, and must be a parseable X.509 certificate.
    #[builder(into)]
    #[serde(rename = "pemCertificate")]
    pub r#pem_certificate: Box<String>,
}
