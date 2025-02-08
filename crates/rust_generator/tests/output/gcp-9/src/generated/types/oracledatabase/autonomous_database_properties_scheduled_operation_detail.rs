#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct AutonomousDatabasePropertiesScheduledOperationDetail {
    /// Possible values:
    ///  DAY_OF_WEEK_UNSPECIFIED
    /// MONDAY
    /// TUESDAY
    /// WEDNESDAY
    /// THURSDAY
    /// FRIDAY
    /// SATURDAY
    /// SUNDAY
    #[builder(into, default)]
    #[serde(rename = "dayOfWeek")]
    pub r#day_of_week: Box<Option<String>>,
    /// Represents a time of day. The date and time zone are either not significant
    /// or are specified elsewhere. An API may choose to allow leap seconds. Related
    /// types are google.type.Date and 'google.protobuf.Timestamp'.
    #[builder(into, default)]
    #[serde(rename = "startTimes")]
    pub r#start_times: Box<Option<Vec<super::super::types::oracledatabase::AutonomousDatabasePropertiesScheduledOperationDetailStartTime>>>,
    /// Represents a time of day. The date and time zone are either not significant
    /// or are specified elsewhere. An API may choose to allow leap seconds. Related
    /// types are google.type.Date and 'google.protobuf.Timestamp'.
    #[builder(into, default)]
    #[serde(rename = "stopTimes")]
    pub r#stop_times: Box<Option<Vec<super::super::types::oracledatabase::AutonomousDatabasePropertiesScheduledOperationDetailStopTime>>>,
}
