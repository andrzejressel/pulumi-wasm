#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InstanceSettingsMetadata {
    /// A metadata key/value items map. The total size of all keys and values must be less than 512KB
    #[builder(into, default)]
    #[serde(rename = "items")]
    pub r#items: Box<Option<std::collections::HashMap<String, String>>>,
}
