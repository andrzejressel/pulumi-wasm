#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct NodePoolNodeConfigContainerdConfig {
    /// Parameters for private container registries configuration.
    #[builder(into, default)]
    #[serde(rename = "privateRegistryAccessConfig")]
    pub r#private_registry_access_config: Box<Option<super::super::types::container::NodePoolNodeConfigContainerdConfigPrivateRegistryAccessConfig>>,
}
