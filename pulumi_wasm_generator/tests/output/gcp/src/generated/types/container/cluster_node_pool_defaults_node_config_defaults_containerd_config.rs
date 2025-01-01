#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterNodePoolDefaultsNodeConfigDefaultsContainerdConfig {
    /// Configuration for private container registries. There are two fields in this config:
    #[builder(into, default)]
    #[serde(rename = "privateRegistryAccessConfig")]
    pub r#private_registry_access_config: Box<Option<super::super::types::container::ClusterNodePoolDefaultsNodeConfigDefaultsContainerdConfigPrivateRegistryAccessConfig>>,
}
