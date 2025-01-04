#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainDefaultUserSettingsCustomFileSystemConfig {
    /// The default EBS storage settings for a private space. See `efs_file_system_config` Block below.
    #[builder(into, default)]
    #[serde(rename = "efsFileSystemConfig")]
    pub r#efs_file_system_config: Box<Option<super::super::types::sagemaker::DomainDefaultUserSettingsCustomFileSystemConfigEfsFileSystemConfig>>,
}
