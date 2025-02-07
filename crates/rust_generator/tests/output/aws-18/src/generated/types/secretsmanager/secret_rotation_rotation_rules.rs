#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SecretRotationRotationRules {
    /// Specifies the number of days between automatic scheduled rotations of the secret. Either `automatically_after_days` or `schedule_expression` must be specified.
    #[builder(into, default)]
    #[serde(rename = "automaticallyAfterDays")]
    pub r#automatically_after_days: Box<Option<i32>>,
    /// The length of the rotation window in hours. For example, `3h` for a three hour window.
    #[builder(into, default)]
    #[serde(rename = "duration")]
    pub r#duration: Box<Option<String>>,
    /// A `cron()` or `rate()` expression that defines the schedule for rotating your secret. Either `automatically_after_days` or `schedule_expression` must be specified.
    #[builder(into, default)]
    #[serde(rename = "scheduleExpression")]
    pub r#schedule_expression: Box<Option<String>>,
}
