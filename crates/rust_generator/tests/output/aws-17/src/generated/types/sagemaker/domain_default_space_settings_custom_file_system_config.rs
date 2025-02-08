#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct DomainDefaultSpaceSettingsCustomFileSystemConfig {
    /// The default EBS storage settings for a private space. See `efs_file_system_config` Block below.
    #[builder(into, default)]
    #[serde(rename = "efsFileSystemConfig")]
    pub r#efs_file_system_config: Box<Option<super::super::types::sagemaker::DomainDefaultSpaceSettingsCustomFileSystemConfigEfsFileSystemConfig>>,
}
