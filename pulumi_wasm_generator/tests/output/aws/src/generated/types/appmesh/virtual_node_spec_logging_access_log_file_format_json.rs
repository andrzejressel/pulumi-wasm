#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualNodeSpecLoggingAccessLogFileFormatJson {
    /// The specified key for the JSON. Must be between 1 and 100 characters in length.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// The specified value for the JSON. Must be between 1 and 100 characters in length.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}