#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct FileSystemProtection {
    /// Indicates whether replication overwrite protection is enabled. Valid values: `ENABLED` or `DISABLED`.
    #[builder(into, default)]
    #[serde(rename = "replicationOverwrite")]
    pub r#replication_overwrite: Box<Option<String>>,
}
