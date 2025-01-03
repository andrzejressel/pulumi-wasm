#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct TableTtl {
    /// Name of the table attribute to store the TTL timestamp in.
    /// Required if `enabled` is `true`, must not be set otherwise.
    #[builder(into, default)]
    #[serde(rename = "attributeName")]
    pub r#attribute_name: Box<Option<String>>,
    /// Whether TTL is enabled.
    /// Default value is `false`.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
}
