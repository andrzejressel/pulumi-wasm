#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualGatewaySpecLoggingAccessLogFileFormat {
    /// The logging format for JSON.
    #[builder(into, default)]
    #[serde(rename = "jsons")]
    pub r#jsons: Box<Option<Vec<super::super::types::appmesh::VirtualGatewaySpecLoggingAccessLogFileFormatJson>>>,
    /// The logging format for text. Must be between 1 and 1000 characters in length.
    #[builder(into, default)]
    #[serde(rename = "text")]
    pub r#text: Box<Option<String>>,
}