#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetAutonomousDatabasesAutonomousDatabasePropertyScheduledOperationDetail {
    /// Possible values:
    ///  DAY_OF_WEEK_UNSPECIFIED
    /// MONDAY
    /// TUESDAY
    /// WEDNESDAY
    /// THURSDAY
    /// FRIDAY
    /// SATURDAY
    /// SUNDAY
    #[builder(into)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: Box<String>,
    /// Represents a time of day. The date and time zone are either not significant
    /// or are specified elsewhere. An API may choose to allow leap seconds. Related
    /// types are google.type.Date and 'google.protobuf.Timestamp'.
    #[builder(into)]
    #[serde(rename = "startTimes")]
    pub r#start_times: Box<Vec<super::super::types::oracledatabase::GetAutonomousDatabasesAutonomousDatabasePropertyScheduledOperationDetailStartTime>>,
    /// Represents a time of day. The date and time zone are either not significant
    /// or are specified elsewhere. An API may choose to allow leap seconds. Related
    /// types are google.type.Date and 'google.protobuf.Timestamp'.
    #[builder(into)]
    #[serde(rename = "stopTimes")]
    pub r#stop_times: Box<Vec<super::super::types::oracledatabase::GetAutonomousDatabasesAutonomousDatabasePropertyScheduledOperationDetailStopTime>>,
}
