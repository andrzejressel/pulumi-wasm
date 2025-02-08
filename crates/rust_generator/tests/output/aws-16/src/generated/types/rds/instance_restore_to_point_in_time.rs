#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct InstanceRestoreToPointInTime {
    /// The date and time to restore from. Value must be a time in Universal Coordinated Time (UTC) format and must be before the latest restorable time for the DB instance. Cannot be specified with `use_latest_restorable_time`.
    #[builder(into, default)]
    #[serde(rename = "restoreTime")]
    pub r#restore_time: Box<Option<String>>,
    /// The ARN of the automated backup from which to restore. Required if `source_db_instance_identifier` or `source_dbi_resource_id` is not specified.
    #[builder(into, default)]
    #[serde(rename = "sourceDbInstanceAutomatedBackupsArn")]
    pub r#source_db_instance_automated_backups_arn: Box<Option<String>>,
    /// The identifier of the source DB instance from which to restore. Must match the identifier of an existing DB instance. Required if `source_db_instance_automated_backups_arn` or `source_dbi_resource_id` is not specified.
    #[builder(into, default)]
    #[serde(rename = "sourceDbInstanceIdentifier")]
    pub r#source_db_instance_identifier: Box<Option<String>>,
    /// The resource ID of the source DB instance from which to restore. Required if `source_db_instance_identifier` or `source_db_instance_automated_backups_arn` is not specified.
    #[builder(into, default)]
    #[serde(rename = "sourceDbiResourceId")]
    pub r#source_dbi_resource_id: Box<Option<String>>,
    /// A boolean value that indicates whether the DB instance is restored from the latest backup time. Defaults to `false`. Cannot be specified with `restore_time`.
    #[builder(into, default)]
    #[serde(rename = "useLatestRestorableTime")]
    pub r#use_latest_restorable_time: Box<Option<bool>>,
}
