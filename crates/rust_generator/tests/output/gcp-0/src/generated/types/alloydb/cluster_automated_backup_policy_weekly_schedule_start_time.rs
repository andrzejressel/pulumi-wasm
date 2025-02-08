#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ClusterAutomatedBackupPolicyWeeklyScheduleStartTime {
    /// Hours of day in 24 hour format. Should be from 0 to 23. An API may choose to allow the value "24:00:00" for scenarios like business closing time.
    #[builder(into, default)]
    #[serde(rename = "hours")]
    pub r#hours: Box<Option<i32>>,
    /// Minutes of hour of day. Currently, only the value 0 is supported.
    #[builder(into, default)]
    #[serde(rename = "minutes")]
    pub r#minutes: Box<Option<i32>>,
    /// Fractions of seconds in nanoseconds. Currently, only the value 0 is supported.
    #[builder(into, default)]
    #[serde(rename = "nanos")]
    pub r#nanos: Box<Option<i32>>,
    /// Seconds of minutes of the time. Currently, only the value 0 is supported.
    #[builder(into, default)]
    #[serde(rename = "seconds")]
    pub r#seconds: Box<Option<i32>>,
}
