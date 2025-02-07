#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetLinuxWebAppLogHttpLogFileSystem {
    /// The retention period in days.
    #[builder(into)]
    #[serde(rename = "retentionInDays")]
    pub r#retention_in_days: Box<i32>,
    /// The maximum size in megabytes that log files can use.
    #[builder(into)]
    #[serde(rename = "retentionInMb")]
    pub r#retention_in_mb: Box<i32>,
}
