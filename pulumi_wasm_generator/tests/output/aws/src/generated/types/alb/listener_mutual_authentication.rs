#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ListenerMutualAuthentication {
    /// Valid values are `off` and `on`.
    #[builder(into, default)]
    #[serde(rename = "advertiseTrustStoreCaNames")]
    pub r#advertise_trust_store_ca_names: Box<Option<String>>,
    /// Whether client certificate expiry is ignored. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "ignoreClientCertificateExpiry")]
    pub r#ignore_client_certificate_expiry: Box<Option<bool>>,
    /// Valid values are `off`, `verify` and `passthrough`.
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    /// ARN of the elbv2 Trust Store.
    #[builder(into, default)]
    #[serde(rename = "trustStoreArn")]
    pub r#trust_store_arn: Box<Option<String>>,
}
