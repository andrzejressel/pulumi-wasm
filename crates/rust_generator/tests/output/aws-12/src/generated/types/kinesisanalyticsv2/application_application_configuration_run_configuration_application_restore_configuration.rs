#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ApplicationApplicationConfigurationRunConfigurationApplicationRestoreConfiguration {
    /// Specifies how the application should be restored. Valid values: `RESTORE_FROM_CUSTOM_SNAPSHOT`, `RESTORE_FROM_LATEST_SNAPSHOT`, `SKIP_RESTORE_FROM_SNAPSHOT`.
    #[builder(into, default)]
    #[serde(rename = "applicationRestoreType")]
    pub r#application_restore_type: Box<Option<String>>,
    /// The identifier of an existing snapshot of application state to use to restart an application. The application uses this value if `RESTORE_FROM_CUSTOM_SNAPSHOT` is specified for `application_restore_type`.
    #[builder(into, default)]
    #[serde(rename = "snapshotName")]
    pub r#snapshot_name: Box<Option<String>>,
}
