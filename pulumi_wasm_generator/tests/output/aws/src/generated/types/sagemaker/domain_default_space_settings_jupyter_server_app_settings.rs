#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainDefaultSpaceSettingsJupyterServerAppSettings {
    /// A list of Git repositories that SageMaker automatically displays to users for cloning in the JupyterServer application. see `code_repository` Block below.
    #[builder(into, default)]
    #[serde(rename = "codeRepositories")]
    pub r#code_repositories: Box<Option<Vec<super::super::types::sagemaker::DomainDefaultSpaceSettingsJupyterServerAppSettingsCodeRepository>>>,
    /// The default instance type and the Amazon Resource Name (ARN) of the SageMaker image created on the instance. see `default_resource_spec` Block below.
    #[builder(into, default)]
    #[serde(rename = "defaultResourceSpec")]
    pub r#default_resource_spec: Box<Option<super::super::types::sagemaker::DomainDefaultSpaceSettingsJupyterServerAppSettingsDefaultResourceSpec>>,
    /// The Amazon Resource Name (ARN) of the Lifecycle Configurations.
    #[builder(into, default)]
    #[serde(rename = "lifecycleConfigArns")]
    pub r#lifecycle_config_arns: Box<Option<Vec<String>>>,
}