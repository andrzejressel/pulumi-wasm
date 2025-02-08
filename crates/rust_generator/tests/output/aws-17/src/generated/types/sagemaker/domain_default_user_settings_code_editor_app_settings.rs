#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DomainDefaultUserSettingsCodeEditorAppSettings {
    /// Indicates whether idle shutdown is activated for JupyterLab applications. see `app_lifecycle_management` Block below.
    #[builder(into, default)]
    #[serde(rename = "appLifecycleManagement")]
    pub r#app_lifecycle_management: Box<Option<super::super::types::sagemaker::DomainDefaultUserSettingsCodeEditorAppSettingsAppLifecycleManagement>>,
    /// The lifecycle configuration that runs before the default lifecycle configuration. It can override changes made in the default lifecycle configuration.
    #[builder(into, default)]
    #[serde(rename = "builtInLifecycleConfigArn")]
    pub r#built_in_lifecycle_config_arn: Box<Option<String>>,
    /// A list of custom SageMaker images that are configured to run as a CodeEditor app. see `custom_image` Block below.
    #[builder(into, default)]
    #[serde(rename = "customImages")]
    pub r#custom_images: Box<Option<Vec<super::super::types::sagemaker::DomainDefaultUserSettingsCodeEditorAppSettingsCustomImage>>>,
    /// The default instance type and the Amazon Resource Name (ARN) of the SageMaker image created on the instance. see `default_resource_spec` Block below.
    #[builder(into, default)]
    #[serde(rename = "defaultResourceSpec")]
    pub r#default_resource_spec: Box<Option<super::super::types::sagemaker::DomainDefaultUserSettingsCodeEditorAppSettingsDefaultResourceSpec>>,
    /// The Amazon Resource Name (ARN) of the Lifecycle Configurations.
    #[builder(into, default)]
    #[serde(rename = "lifecycleConfigArns")]
    pub r#lifecycle_config_arns: Box<Option<Vec<String>>>,
}
