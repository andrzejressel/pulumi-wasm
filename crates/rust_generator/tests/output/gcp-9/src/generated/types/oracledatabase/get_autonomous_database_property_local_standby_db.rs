#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetAutonomousDatabasePropertyLocalStandbyDb {
    /// The date and time the Autonomous Data Guard role was switched for the
    /// standby Autonomous Database.
    #[builder(into)]
    #[serde(rename = "dataGuardRoleChangedTime")]
    pub r#data_guard_role_changed_time: Box<String>,
    /// The date and time the Disaster Recovery role was switched for the standby
    /// Autonomous Database.
    #[builder(into)]
    #[serde(rename = "disasterRecoveryRoleChangedTime")]
    pub r#disaster_recovery_role_changed_time: Box<String>,
    /// The amount of time, in seconds, that the data of the standby database lags
    /// in comparison to the data of the primary database.
    #[builder(into)]
    #[serde(rename = "lagTimeDuration")]
    pub r#lag_time_duration: Box<String>,
    /// The additional details about the current lifecycle state of the
    /// Autonomous Database.
    #[builder(into)]
    #[serde(rename = "lifecycleDetails")]
    pub r#lifecycle_details: Box<String>,
    /// Possible values:
    ///  STATE_UNSPECIFIED
    /// PROVISIONING
    /// AVAILABLE
    /// STOPPING
    /// STOPPED
    /// STARTING
    /// TERMINATING
    /// TERMINATED
    /// UNAVAILABLE
    /// RESTORE_IN_PROGRESS
    /// RESTORE_FAILED
    /// BACKUP_IN_PROGRESS
    /// SCALE_IN_PROGRESS
    /// AVAILABLE_NEEDS_ATTENTION
    /// UPDATING
    /// MAINTENANCE_IN_PROGRESS
    /// RESTARTING
    /// RECREATING
    /// ROLE_CHANGE_IN_PROGRESS
    /// UPGRADING
    /// INACCESSIBLE
    /// STANDBY
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
}
