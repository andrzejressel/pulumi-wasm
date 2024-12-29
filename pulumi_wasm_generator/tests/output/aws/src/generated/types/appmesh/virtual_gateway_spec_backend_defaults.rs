#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VirtualGatewaySpecBackendDefaults {
    /// Default client policy for virtual gateway backends.
    #[builder(into, default)]
    #[serde(rename = "clientPolicy")]
    pub r#client_policy: Box<Option<super::super::types::appmesh::VirtualGatewaySpecBackendDefaultsClientPolicy>>,
}
