#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PolicyVmTieringPolicy {
    /// An `archived_restore_point` block as defined below.
    #[builder(into)]
    #[serde(rename = "archivedRestorePoint")]
    pub r#archived_restore_point: Box<super::super::types::backup::PolicyVmTieringPolicyArchivedRestorePoint>,
}
