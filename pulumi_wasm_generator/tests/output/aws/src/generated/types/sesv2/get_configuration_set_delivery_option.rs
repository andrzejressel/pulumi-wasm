#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetConfigurationSetDeliveryOption {
    /// The name of the dedicated IP pool to associate with the configuration set.
    #[builder(into)]
    #[serde(rename = "sendingPoolName")]
    pub r#sending_pool_name: Box<String>,
    /// Specifies whether messages that use the configuration set are required to use Transport Layer Security (TLS).
    #[builder(into)]
    #[serde(rename = "tlsPolicy")]
    pub r#tls_policy: Box<String>,
}
