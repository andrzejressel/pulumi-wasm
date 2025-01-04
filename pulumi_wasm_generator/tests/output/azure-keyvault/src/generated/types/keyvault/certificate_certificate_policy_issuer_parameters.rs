#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CertificateCertificatePolicyIssuerParameters {
    /// The name of the Certificate Issuer. Possible values include `Self` (for self-signed certificate), or `Unknown` (for a certificate issuing authority like `Let's Encrypt` and Azure direct supported ones).
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}
