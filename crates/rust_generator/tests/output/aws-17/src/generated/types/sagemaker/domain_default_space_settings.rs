#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DomainDefaultSpaceSettings {
    /// The settings for assigning a custom file system to a user profile. Permitted users can access this file system in Amazon SageMaker Studio. See `custom_file_system_config` Block below.
    #[builder(into, default)]
    #[serde(rename = "customFileSystemConfigs")]
    pub r#custom_file_system_configs: Box<Option<Vec<super::super::types::sagemaker::DomainDefaultSpaceSettingsCustomFileSystemConfig>>>,
    /// Details about the POSIX identity that is used for file system operations. See `custom_posix_user_config` Block below.
    #[builder(into, default)]
    #[serde(rename = "customPosixUserConfig")]
    pub r#custom_posix_user_config: Box<Option<super::super::types::sagemaker::DomainDefaultSpaceSettingsCustomPosixUserConfig>>,
    /// The execution role for the space.
    #[builder(into)]
    #[serde(rename = "executionRole")]
    pub r#execution_role: Box<String>,
    /// The settings for the JupyterLab application. See `jupyter_lab_app_settings` Block below.
    #[builder(into, default)]
    #[serde(rename = "jupyterLabAppSettings")]
    pub r#jupyter_lab_app_settings: Box<Option<super::super::types::sagemaker::DomainDefaultSpaceSettingsJupyterLabAppSettings>>,
    /// The Jupyter server's app settings. See `jupyter_server_app_settings` Block below.
    #[builder(into, default)]
    #[serde(rename = "jupyterServerAppSettings")]
    pub r#jupyter_server_app_settings: Box<Option<super::super::types::sagemaker::DomainDefaultSpaceSettingsJupyterServerAppSettings>>,
    /// The kernel gateway app settings. See `kernel_gateway_app_settings` Block below.
    #[builder(into, default)]
    #[serde(rename = "kernelGatewayAppSettings")]
    pub r#kernel_gateway_app_settings: Box<Option<super::super::types::sagemaker::DomainDefaultSpaceSettingsKernelGatewayAppSettings>>,
    /// The security groups for the Amazon Virtual Private Cloud that the space uses for communication.
    #[builder(into, default)]
    #[serde(rename = "securityGroups")]
    pub r#security_groups: Box<Option<Vec<String>>>,
    /// The storage settings for a private space. See `space_storage_settings` Block below.
    #[builder(into, default)]
    #[serde(rename = "spaceStorageSettings")]
    pub r#space_storage_settings: Box<Option<super::super::types::sagemaker::DomainDefaultSpaceSettingsSpaceStorageSettings>>,
}
