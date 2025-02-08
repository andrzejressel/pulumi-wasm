#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetClusterNodePoolNodeConfigContainerdConfig {
    /// Parameters for private container registries configuration.
    #[builder(into)]
    #[serde(rename = "privateRegistryAccessConfigs")]
    pub r#private_registry_access_configs: Box<Vec<super::super::types::container::GetClusterNodePoolNodeConfigContainerdConfigPrivateRegistryAccessConfig>>,
}
