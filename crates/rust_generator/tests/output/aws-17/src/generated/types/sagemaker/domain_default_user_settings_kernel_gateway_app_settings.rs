#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainDefaultUserSettingsKernelGatewayAppSettings {
    /// A list of custom SageMaker images that are configured to run as a KernelGateway app. see `custom_image` Block below.
    #[builder(into, default)]
    #[serde(rename = "customImages")]
    pub r#custom_images: Box<Option<Vec<super::super::types::sagemaker::DomainDefaultUserSettingsKernelGatewayAppSettingsCustomImage>>>,
    /// The default instance type and the Amazon Resource Name (ARN) of the SageMaker image created on the instance. see `default_resource_spec` Block below.
    #[builder(into, default)]
    #[serde(rename = "defaultResourceSpec")]
    pub r#default_resource_spec: Box<Option<super::super::types::sagemaker::DomainDefaultUserSettingsKernelGatewayAppSettingsDefaultResourceSpec>>,
    /// The Amazon Resource Name (ARN) of the Lifecycle Configurations.
    #[builder(into, default)]
    #[serde(rename = "lifecycleConfigArns")]
    pub r#lifecycle_config_arns: Box<Option<Vec<String>>>,
}
