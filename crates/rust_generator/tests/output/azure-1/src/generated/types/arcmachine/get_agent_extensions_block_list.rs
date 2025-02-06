#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAgentExtensionsBlockList {
    /// Publisher of the extension.
    #[builder(into)]
    #[serde(rename = "publisher")]
    pub r#publisher: Box<String>,
    /// The identity type.
    #[builder(into)]
    #[serde(rename = "type")]
    pub r#type_: Box<String>,
}
