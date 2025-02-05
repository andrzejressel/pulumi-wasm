#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetVirtualGatewaySpecLoggingAccessLogFile {
    #[builder(into)]
    #[serde(rename = "formats")]
    pub r#formats: Box<Vec<super::super::types::appmesh::GetVirtualGatewaySpecLoggingAccessLogFileFormat>>,
    #[builder(into)]
    #[serde(rename = "path")]
    pub r#path: Box<String>,
}
