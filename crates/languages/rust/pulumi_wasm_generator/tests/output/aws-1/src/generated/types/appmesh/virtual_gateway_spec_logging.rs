#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualGatewaySpecLogging {
    /// Access log configuration for a virtual gateway.
    #[builder(into, default)]
    #[serde(rename = "accessLog")]
    pub r#access_log: Box<Option<super::super::types::appmesh::VirtualGatewaySpecLoggingAccessLog>>,
}
