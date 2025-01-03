#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainDefaultUserSettingsTensorBoardAppSettings {
    /// The default instance type and the Amazon Resource Name (ARN) of the SageMaker image created on the instance. see `default_resource_spec` Block below.
    #[builder(into, default)]
    #[serde(rename = "defaultResourceSpec")]
    pub r#default_resource_spec: Box<Option<super::super::types::sagemaker::DomainDefaultUserSettingsTensorBoardAppSettingsDefaultResourceSpec>>,
}
