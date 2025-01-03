#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualGatewaySpec {
    /// Defaults for backends.
    #[builder(into, default)]
    #[serde(rename = "backendDefaults")]
    pub r#backend_defaults: Box<Option<super::super::types::appmesh::VirtualGatewaySpecBackendDefaults>>,
    /// Listeners that the mesh endpoint is expected to receive inbound traffic from. You can specify one listener.
    #[builder(into)]
    #[serde(rename = "listeners")]
    pub r#listeners: Box<Vec<super::super::types::appmesh::VirtualGatewaySpecListener>>,
    /// Inbound and outbound access logging information for the virtual gateway.
    #[builder(into, default)]
    #[serde(rename = "logging")]
    pub r#logging: Box<Option<super::super::types::appmesh::VirtualGatewaySpecLogging>>,
}
