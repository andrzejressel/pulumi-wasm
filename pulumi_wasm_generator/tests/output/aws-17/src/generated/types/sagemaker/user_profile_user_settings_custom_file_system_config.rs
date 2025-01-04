#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct UserProfileUserSettingsCustomFileSystemConfig {
    /// The default EBS storage settings for a private space. See EFS File System Config below.
    #[builder(into, default)]
    #[serde(rename = "efsFileSystemConfigs")]
    pub r#efs_file_system_configs: Box<Option<Vec<super::super::types::sagemaker::UserProfileUserSettingsCustomFileSystemConfigEfsFileSystemConfig>>>,
}
