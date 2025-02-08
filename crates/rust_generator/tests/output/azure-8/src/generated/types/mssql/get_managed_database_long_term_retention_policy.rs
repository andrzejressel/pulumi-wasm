#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetManagedDatabaseLongTermRetentionPolicy {
    /// Specifies if the backups are immutable.
    #[builder(into)]
    #[serde(rename = "immutableBackupsEnabled")]
    pub r#immutable_backups_enabled: Box<bool>,
    /// The monthly retention policy for an LTR backup in an ISO 8601 format.
    #[builder(into)]
    #[serde(rename = "monthlyRetention")]
    pub r#monthly_retention: Box<String>,
    /// The week of year to take the yearly backup.
    #[builder(into)]
    #[serde(rename = "weekOfYear")]
    pub r#week_of_year: Box<i32>,
    /// The weekly retention policy for an LTR backup in an ISO 8601 format.
    #[builder(into)]
    #[serde(rename = "weeklyRetention")]
    pub r#weekly_retention: Box<String>,
    /// The yearly retention policy for an LTR backup in an ISO 8601 format.
    #[builder(into)]
    #[serde(rename = "yearlyRetention")]
    pub r#yearly_retention: Box<String>,
}
