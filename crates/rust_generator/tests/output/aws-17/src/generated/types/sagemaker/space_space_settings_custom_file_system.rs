#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct SpaceSpaceSettingsCustomFileSystem {
    /// A custom file system in Amazon EFS. See `efs_file_system` Block below.
    #[builder(into)]
    #[serde(rename = "efsFileSystem")]
    pub r#efs_file_system: Box<super::super::types::sagemaker::SpaceSpaceSettingsCustomFileSystemEfsFileSystem>,
}
