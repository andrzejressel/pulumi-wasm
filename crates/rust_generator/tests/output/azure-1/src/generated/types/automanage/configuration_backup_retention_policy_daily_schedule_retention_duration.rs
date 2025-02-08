#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConfigurationBackupRetentionPolicyDailyScheduleRetentionDuration {
    /// The count of the retention duration of the backup policy. Valid value inside `daily_schedule` is `7` to `9999` and inside `weekly_schedule` is `1` to `5163`.
    #[builder(into, default)]
    #[serde(rename = "count")]
    pub r#count: Box<Option<i32>>,
    /// The duration type of the retention duration of the backup policy. Valid value inside `daily_schedule` is `Days` and inside `weekly_schedule` is `Weeks`. Defaults to `Days`.
    #[builder(into, default)]
    #[serde(rename = "durationType")]
    pub r#duration_type: Box<Option<String>>,
}
