#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct SpaceSpaceSettingsJupyterLabAppSettings {
    /// Settings that are used to configure and manage the lifecycle of JupyterLab applications in a space. See `app_lifecycle_management` Block below.
    #[builder(into, default)]
    #[serde(rename = "appLifecycleManagement")]
    pub r#app_lifecycle_management: Box<Option<super::super::types::sagemaker::SpaceSpaceSettingsJupyterLabAppSettingsAppLifecycleManagement>>,
    /// A list of Git repositories that SageMaker automatically displays to users for cloning in the JupyterLab application. See `code_repository` Block below.
    #[builder(into, default)]
    #[serde(rename = "codeRepositories")]
    pub r#code_repositories: Box<Option<Vec<super::super::types::sagemaker::SpaceSpaceSettingsJupyterLabAppSettingsCodeRepository>>>,
    /// The default instance type and the Amazon Resource Name (ARN) of the SageMaker image created on the instance. See `default_resource_spec` Block below.
    #[builder(into)]
    #[serde(rename = "defaultResourceSpec")]
    pub r#default_resource_spec: Box<super::super::types::sagemaker::SpaceSpaceSettingsJupyterLabAppSettingsDefaultResourceSpec>,
}
