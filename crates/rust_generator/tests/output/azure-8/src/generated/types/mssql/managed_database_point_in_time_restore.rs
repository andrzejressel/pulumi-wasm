#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ManagedDatabasePointInTimeRestore {
    /// The point in time for the restore from `source_database_id`. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "restorePointInTime")]
    pub r#restore_point_in_time: Box<String>,
    /// The source database id that will be used to restore from. Changing this forces a new resource to be created.
    #[builder(into)]
    #[serde(rename = "sourceDatabaseId")]
    pub r#source_database_id: Box<String>,
}
