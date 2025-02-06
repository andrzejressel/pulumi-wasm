#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionJobTriggerTriggerSchedule {
    /// With this option a job is started a regular periodic basis. For example: every day (86400 seconds).
    /// A scheduled start time will be skipped if the previous execution has not ended when its scheduled time occurs.
    /// This value must be set to a time duration greater than or equal to 1 day and can be no longer than 60 days.
    /// A duration in seconds with up to nine fractional digits, terminated by 's'. Example: "3.5s".
    /// 
    /// - - -
    #[builder(into, default)]
    #[serde(rename = "recurrencePeriodDuration")]
    pub r#recurrence_period_duration: Box<Option<String>>,
}
