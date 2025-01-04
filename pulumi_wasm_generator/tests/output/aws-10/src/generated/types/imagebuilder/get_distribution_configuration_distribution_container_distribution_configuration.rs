#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetDistributionConfigurationDistributionContainerDistributionConfiguration {
    /// Set of tags that are attached to the container distribution configuration.
    #[builder(into)]
    #[serde(rename = "containerTags")]
    pub r#container_tags: Box<Vec<String>>,
    /// Description of the container distribution configuration.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// Set of destination repositories for the container distribution configuration.
    #[builder(into)]
    #[serde(rename = "targetRepositories")]
    pub r#target_repositories: Box<Vec<super::super::types::imagebuilder::GetDistributionConfigurationDistributionContainerDistributionConfigurationTargetRepository>>,
}
