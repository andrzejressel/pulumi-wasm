#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WindowsVirtualMachineGalleryApplication {
    /// Specifies whether the version will be automatically updated for the VM when a new Gallery Application version is available in PIR/SIG. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "automaticUpgradeEnabled")]
    pub r#automatic_upgrade_enabled: Box<Option<bool>>,
    /// Specifies the URI to an Azure Blob that will replace the default configuration for the package if provided.
    #[builder(into, default)]
    #[serde(rename = "configurationBlobUri")]
    pub r#configuration_blob_uri: Box<Option<String>>,
    /// Specifies the order in which the packages have to be installed. Possible values are between `0` and `2147483647`. Defaults to `0`.
    #[builder(into, default)]
    #[serde(rename = "order")]
    pub r#order: Box<Option<i32>>,
    /// Specifies a passthrough value for more generic context. This field can be any valid `string` value.
    #[builder(into, default)]
    #[serde(rename = "tag")]
    pub r#tag: Box<Option<String>>,
    /// Specifies whether any failure for any operation in the VmApplication will fail the deployment of the VM. Defaults to `false`.
    #[builder(into, default)]
    #[serde(rename = "treatFailureAsDeploymentFailureEnabled")]
    pub r#treat_failure_as_deployment_failure_enabled: Box<Option<bool>>,
    /// Specifies the Gallery Application Version resource ID.
    #[builder(into)]
    #[serde(rename = "versionId")]
    pub r#version_id: Box<String>,
}
