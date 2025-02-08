#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct UserProfileUserSettings {
    /// Indicates whether auto-mounting of an EFS volume is supported for the user profile. The `DefaultAsDomain` value is only supported for user profiles. Do not use the `DefaultAsDomain` value when setting this parameter for a domain. Valid values are: `Enabled`, `Disabled`, and `DefaultAsDomain`.
    #[builder(into, default)]
    #[serde(rename = "autoMountHomeEfs")]
    pub r#auto_mount_home_efs: Box<Option<String>>,
    /// The Canvas app settings. See Canvas App Settings below.
    #[builder(into, default)]
    #[serde(rename = "canvasAppSettings")]
    pub r#canvas_app_settings: Box<Option<super::super::types::sagemaker::UserProfileUserSettingsCanvasAppSettings>>,
    /// The Code Editor application settings. See Code Editor App Settings below.
    #[builder(into, default)]
    #[serde(rename = "codeEditorAppSettings")]
    pub r#code_editor_app_settings: Box<Option<super::super::types::sagemaker::UserProfileUserSettingsCodeEditorAppSettings>>,
    /// The settings for assigning a custom file system to a user profile. Permitted users can access this file system in Amazon SageMaker Studio. See Custom File System Config below.
    #[builder(into, default)]
    #[serde(rename = "customFileSystemConfigs")]
    pub r#custom_file_system_configs: Box<Option<Vec<super::super::types::sagemaker::UserProfileUserSettingsCustomFileSystemConfig>>>,
    /// Details about the POSIX identity that is used for file system operations. See Custom Posix User Config below.
    #[builder(into, default)]
    #[serde(rename = "customPosixUserConfig")]
    pub r#custom_posix_user_config: Box<Option<super::super::types::sagemaker::UserProfileUserSettingsCustomPosixUserConfig>>,
    /// The default experience that the user is directed to when accessing the domain. The supported values are: `studio::`: Indicates that Studio is the default experience. This value can only be passed if StudioWebPortal is set to ENABLED. `app:JupyterServer:`: Indicates that Studio Classic is the default experience.
    #[builder(into, default)]
    #[serde(rename = "defaultLandingUri")]
    pub r#default_landing_uri: Box<Option<String>>,
    /// The execution role ARN for the user.
    #[builder(into)]
    #[serde(rename = "executionRole")]
    pub r#execution_role: Box<String>,
    /// The settings for the JupyterLab application. See Jupyter Lab App Settings below.
    #[builder(into, default)]
    #[serde(rename = "jupyterLabAppSettings")]
    pub r#jupyter_lab_app_settings: Box<Option<super::super::types::sagemaker::UserProfileUserSettingsJupyterLabAppSettings>>,
    /// The Jupyter server's app settings. See Jupyter Server App Settings below.
    #[builder(into, default)]
    #[serde(rename = "jupyterServerAppSettings")]
    pub r#jupyter_server_app_settings: Box<Option<super::super::types::sagemaker::UserProfileUserSettingsJupyterServerAppSettings>>,
    /// The kernel gateway app settings. See Kernel Gateway App Settings below.
    #[builder(into, default)]
    #[serde(rename = "kernelGatewayAppSettings")]
    pub r#kernel_gateway_app_settings: Box<Option<super::super::types::sagemaker::UserProfileUserSettingsKernelGatewayAppSettings>>,
    /// The RSession app settings. See RSession App Settings below.
    #[builder(into, default)]
    #[serde(rename = "rSessionAppSettings")]
    pub r#r_session_app_settings: Box<Option<super::super::types::sagemaker::UserProfileUserSettingsRSessionAppSettings>>,
    /// A collection of settings that configure user interaction with the RStudioServerPro app. See RStudioServerProAppSettings below.
    #[builder(into, default)]
    #[serde(rename = "rStudioServerProAppSettings")]
    pub r#r_studio_server_pro_app_settings: Box<Option<super::super::types::sagemaker::UserProfileUserSettingsRStudioServerProAppSettings>>,
    /// A list of security group IDs that will be attached to the user.
    #[builder(into, default)]
    #[serde(rename = "securityGroups")]
    pub r#security_groups: Box<Option<Vec<String>>>,
    /// The sharing settings. See Sharing Settings below.
    #[builder(into, default)]
    #[serde(rename = "sharingSettings")]
    pub r#sharing_settings: Box<Option<super::super::types::sagemaker::UserProfileUserSettingsSharingSettings>>,
    /// The storage settings for a private space. See Space Storage Settings below.
    #[builder(into, default)]
    #[serde(rename = "spaceStorageSettings")]
    pub r#space_storage_settings: Box<Option<super::super::types::sagemaker::UserProfileUserSettingsSpaceStorageSettings>>,
    /// Whether the user can access Studio. If this value is set to `DISABLED`, the user cannot access Studio, even if that is the default experience for the domain. Valid values are `ENABLED` and `DISABLED`.
    #[builder(into, default)]
    #[serde(rename = "studioWebPortal")]
    pub r#studio_web_portal: Box<Option<String>>,
    /// The Studio Web Portal settings. See `studio_web_portal_settings` Block below.
    #[builder(into, default)]
    #[serde(rename = "studioWebPortalSettings")]
    pub r#studio_web_portal_settings: Box<Option<super::super::types::sagemaker::UserProfileUserSettingsStudioWebPortalSettings>>,
    /// The TensorBoard app settings. See TensorBoard App Settings below.
    #[builder(into, default)]
    #[serde(rename = "tensorBoardAppSettings")]
    pub r#tensor_board_app_settings: Box<Option<super::super::types::sagemaker::UserProfileUserSettingsTensorBoardAppSettings>>,
}
