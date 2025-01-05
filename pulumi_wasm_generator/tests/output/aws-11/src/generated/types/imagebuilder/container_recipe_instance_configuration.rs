#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ContainerRecipeInstanceConfiguration {
    /// Configuration block(s) with block device mappings for the container recipe. Detailed below.
    #[builder(into, default)]
    #[serde(rename = "blockDeviceMappings")]
    pub r#block_device_mappings: Box<Option<Vec<super::super::types::imagebuilder::ContainerRecipeInstanceConfigurationBlockDeviceMapping>>>,
    /// The AMI ID to use as the base image for a container build and test instance. If not specified, Image Builder will use the appropriate ECS-optimized AMI as a base image.
    #[builder(into, default)]
    #[serde(rename = "image")]
    pub r#image: Box<Option<String>>,
}
