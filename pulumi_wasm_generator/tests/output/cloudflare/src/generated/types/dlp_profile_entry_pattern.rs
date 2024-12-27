#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DlpProfileEntryPattern {
    /// The regex that defines the pattern.
    #[builder(into)]
    #[serde(rename = "regex")]
    pub r#regex: Box<String>,
    /// The validation algorithm to apply with this pattern.
    #[builder(into, default)]
    #[serde(rename = "validation")]
    pub r#validation: Box<Option<String>>,
}
