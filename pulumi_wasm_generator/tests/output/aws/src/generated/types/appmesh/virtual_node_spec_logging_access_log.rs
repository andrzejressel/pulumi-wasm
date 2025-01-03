#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualNodeSpecLoggingAccessLog {
    /// File object to send virtual node access logs to.
    #[builder(into, default)]
    #[serde(rename = "file")]
    pub r#file: Box<Option<super::super::types::appmesh::VirtualNodeSpecLoggingAccessLogFile>>,
}
