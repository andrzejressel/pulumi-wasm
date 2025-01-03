#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct IgnoreTags {
    /// Resource tag key prefixes to ignore across all resources. Can also be configured with the TF_AWS_IGNORE_TAGS_KEY_PREFIXES environment variable.
    #[builder(into, default)]
    #[serde(rename = "keyPrefixes")]
    pub r#key_prefixes: Box<Option<Vec<String>>>,
    /// Resource tag keys to ignore across all resources. Can also be configured with the TF_AWS_IGNORE_TAGS_KEYS environment variable.
    #[builder(into, default)]
    #[serde(rename = "keys")]
    pub r#keys: Box<Option<Vec<String>>>,
}
