#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetResourcePolicyInstanceSchedulePolicy {
    /// The expiration time of the schedule. The timestamp is an RFC3339 string.
    #[builder(into)]
    #[serde(rename = "expirationTime")]
    pub r#expiration_time: Box<String>,
    /// The start time of the schedule. The timestamp is an RFC3339 string.
    #[builder(into)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<String>,
    /// Specifies the time zone to be used in interpreting the schedule. The value of this field must be a time zone name
    /// from the tz database: http://en.wikipedia.org/wiki/Tz_database.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Box<String>,
    /// Specifies the schedule for starting instances.
    #[builder(into)]
    #[serde(rename = "vmStartSchedules")]
    pub r#vm_start_schedules: Box<Vec<super::super::types::compute::GetResourcePolicyInstanceSchedulePolicyVmStartSchedule>>,
    /// Specifies the schedule for stopping instances.
    #[builder(into)]
    #[serde(rename = "vmStopSchedules")]
    pub r#vm_stop_schedules: Box<Vec<super::super::types::compute::GetResourcePolicyInstanceSchedulePolicyVmStopSchedule>>,
}
