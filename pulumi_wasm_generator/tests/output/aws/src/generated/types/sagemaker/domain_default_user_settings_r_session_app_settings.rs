#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainDefaultUserSettingsRSessionAppSettings {
    /// A list of custom SageMaker images that are configured to run as a RSession app. see `custom_image` Block below.
    #[builder(into, default)]
    #[serde(rename = "customImages")]
    pub r#custom_images: Box<Option<Vec<super::super::types::sagemaker::DomainDefaultUserSettingsRSessionAppSettingsCustomImage>>>,
    /// The default instance type and the Amazon Resource Name (ARN) of the SageMaker image created on the instance. see `default_resource_spec` Block above.
    #[builder(into, default)]
    #[serde(rename = "defaultResourceSpec")]
    pub r#default_resource_spec: Box<Option<super::super::types::sagemaker::DomainDefaultUserSettingsRSessionAppSettingsDefaultResourceSpec>>,
}
