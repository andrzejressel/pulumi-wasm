#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ManagedClusterAuthenticationCertificate {
    /// The certificate's CN.
    #[builder(into, default)]
    #[serde(rename = "commonName")]
    pub r#common_name: Box<Option<String>>,
    /// The thumbprint of the certificate.
    #[builder(into)]
    #[serde(rename = "thumbprint")]
    pub r#thumbprint: Box<String>,
    /// The type of the certificate. Can be `AdminClient` or `ReadOnlyClient`.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type: Box<String>,
}