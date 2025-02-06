#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConfigurationSetDeliveryOptions {
    /// Whether messages that use the configuration set are required to use Transport Layer Security (TLS). If the value is `Require`, messages are only delivered if a TLS connection can be established. If the value is `Optional`, messages can be delivered in plain text if a TLS connection can't be established. Valid values: `Require` or `Optional`. Defaults to `Optional`.
    #[builder(into, default)]
    #[serde(rename = "tlsPolicy")]
    pub r#tls_policy: Box<Option<String>>,
}
