#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetListenerMutualAuthentication {
    #[builder(into)]
    #[serde(rename = "ignoreClientCertificateExpiry")]
    pub r#ignore_client_certificate_expiry: Box<bool>,
    #[builder(into)]
    #[serde(rename = "mode")]
    pub r#mode: Box<String>,
    #[builder(into)]
    #[serde(rename = "trustStoreArn")]
    pub r#trust_store_arn: Box<String>,
}