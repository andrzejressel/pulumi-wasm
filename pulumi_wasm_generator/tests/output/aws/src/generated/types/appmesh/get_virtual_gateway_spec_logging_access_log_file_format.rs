#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualGatewaySpecLoggingAccessLogFileFormat {
    #[builder(into)]
    #[serde(rename = "jsons")]
    pub r#jsons: Box<Vec<super::super::types::appmesh::GetVirtualGatewaySpecLoggingAccessLogFileFormatJson>>,
    #[builder(into)]
    #[serde(rename = "text")]
    pub r#text: Box<String>,
}