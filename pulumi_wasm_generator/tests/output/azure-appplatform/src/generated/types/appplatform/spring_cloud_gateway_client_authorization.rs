#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SpringCloudGatewayClientAuthorization {
    /// Specifies the Spring Cloud Certificate IDs of the Spring Cloud Gateway.
    #[builder(into, default)]
    #[serde(rename = "certificateIds")]
    pub r#certificate_ids: Box<Option<Vec<String>>>,
    /// Specifies whether the client certificate verification is enabled.
    #[builder(into, default)]
    #[serde(rename = "verificationEnabled")]
    pub r#verification_enabled: Box<Option<bool>>,
}