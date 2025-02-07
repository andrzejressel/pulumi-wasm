#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetContainerRecipeInstanceConfiguration {
    /// Set of objects with block device mappings for the instance configuration.
    #[builder(into)]
    #[serde(rename = "blockDeviceMappings")]
    pub r#block_device_mappings: Box<Vec<super::super::types::imagebuilder::GetContainerRecipeInstanceConfigurationBlockDeviceMapping>>,
    /// AMI ID of the base image for container build and test instance.
    #[builder(into)]
    #[serde(rename = "image")]
    pub r#image: Box<String>,
}
