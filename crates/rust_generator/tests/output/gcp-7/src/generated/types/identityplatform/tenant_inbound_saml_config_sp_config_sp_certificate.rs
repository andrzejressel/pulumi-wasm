#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TenantInboundSamlConfigSpConfigSpCertificate {
    /// The x509 certificate
    #[builder(into, default)]
    #[serde(rename = "x509Certificate")]
    pub r#x_509_certificate: Box<Option<String>>,
}
