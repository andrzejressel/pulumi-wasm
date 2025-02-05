#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VpnServerConfigurationRadiusServerRootCertificate {
    /// A name used to uniquely identify this certificate.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The Public Key Data associated with the Certificate.
    #[builder(into)]
    #[serde(rename = "publicCertData")]
    pub r#public_cert_data: Box<String>,
}
