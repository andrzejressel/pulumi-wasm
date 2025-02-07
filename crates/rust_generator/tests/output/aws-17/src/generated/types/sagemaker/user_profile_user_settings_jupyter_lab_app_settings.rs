#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UserProfileUserSettingsJupyterLabAppSettings {
    /// Indicates whether idle shutdown is activated for JupyterLab applications. see `app_lifecycle_management` Block below.
    #[builder(into, default)]
    #[serde(rename = "appLifecycleManagement")]
    pub r#app_lifecycle_management: Box<Option<super::super::types::sagemaker::UserProfileUserSettingsJupyterLabAppSettingsAppLifecycleManagement>>,
    /// The lifecycle configuration that runs before the default lifecycle configuration. It can override changes made in the default lifecycle configuration.
    #[builder(into, default)]
    #[serde(rename = "builtInLifecycleConfigArn")]
    pub r#built_in_lifecycle_config_arn: Box<Option<String>>,
    /// A list of Git repositories that SageMaker automatically displays to users for cloning in the JupyterServer application. see Code Repository below.
    #[builder(into, default)]
    #[serde(rename = "codeRepositories")]
    pub r#code_repositories: Box<Option<Vec<super::super::types::sagemaker::UserProfileUserSettingsJupyterLabAppSettingsCodeRepository>>>,
    #[builder(into, default)]
    #[serde(rename = "customImages")]
    pub r#custom_images: Box<Option<Vec<super::super::types::sagemaker::UserProfileUserSettingsJupyterLabAppSettingsCustomImage>>>,
    /// The default instance type and the Amazon Resource Name (ARN) of the SageMaker image created on the instance. see Default Resource Spec below.
    #[builder(into, default)]
    #[serde(rename = "defaultResourceSpec")]
    pub r#default_resource_spec: Box<Option<super::super::types::sagemaker::UserProfileUserSettingsJupyterLabAppSettingsDefaultResourceSpec>>,
    /// The configuration parameters that specify the IAM roles assumed by the execution role of SageMaker (assumable roles) and the cluster instances or job execution environments (execution roles or runtime roles) to manage and access resources required for running Amazon EMR clusters or Amazon EMR Serverless applications. see `emr_settings` Block below.
    #[builder(into, default)]
    #[serde(rename = "emrSettings")]
    pub r#emr_settings: Box<Option<super::super::types::sagemaker::UserProfileUserSettingsJupyterLabAppSettingsEmrSettings>>,
    /// The Amazon Resource Name (ARN) of the Lifecycle Configurations.
    #[builder(into, default)]
    #[serde(rename = "lifecycleConfigArns")]
    pub r#lifecycle_config_arns: Box<Option<Vec<String>>>,
}
