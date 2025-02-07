#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DistributionConfigurationDistributionContainerDistributionConfiguration {
    /// Set of tags that are attached to the container distribution configuration.
    #[builder(into, default)]
    #[serde(rename = "containerTags")]
    pub r#container_tags: Box<Option<Vec<String>>>,
    /// Description of the container distribution configuration.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// Configuration block with the destination repository for the container distribution configuration.
    #[builder(into)]
    #[serde(rename = "targetRepository")]
    pub r#target_repository: Box<super::super::types::imagebuilder::DistributionConfigurationDistributionContainerDistributionConfigurationTargetRepository>,
}
