#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpaceSpaceSettingsJupyterServerAppSettings {
    /// A list of Git repositories that SageMaker automatically displays to users for cloning in the JupyterServer application. See `code_repository` Block below.
    #[builder(into, default)]
    #[serde(rename = "codeRepositories")]
    pub r#code_repositories: Box<Option<Vec<super::super::types::sagemaker::SpaceSpaceSettingsJupyterServerAppSettingsCodeRepository>>>,
    /// The default instance type and the Amazon Resource Name (ARN) of the SageMaker image created on the instance. See `default_resource_spec` Block below.
    #[builder(into)]
    #[serde(rename = "defaultResourceSpec")]
    pub r#default_resource_spec: Box<super::super::types::sagemaker::SpaceSpaceSettingsJupyterServerAppSettingsDefaultResourceSpec>,
    /// The Amazon Resource Name (ARN) of the Lifecycle Configurations.
    #[builder(into, default)]
    #[serde(rename = "lifecycleConfigArns")]
    pub r#lifecycle_config_arns: Box<Option<Vec<String>>>,
}
