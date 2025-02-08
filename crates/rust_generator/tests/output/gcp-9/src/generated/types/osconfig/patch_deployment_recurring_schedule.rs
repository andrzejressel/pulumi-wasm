#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct PatchDeploymentRecurringSchedule {
    /// The end time at which a recurring patch deployment schedule is no longer active.
    /// A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
    #[builder(into, default)]
    #[serde(rename = "endTime")]
    pub r#end_time: Box<Option<String>>,
    /// (Output)
    /// The time the last patch job ran successfully.
    /// A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
    #[builder(into, default)]
    #[serde(rename = "lastExecuteTime")]
    pub r#last_execute_time: Box<Option<String>>,
    /// Schedule with monthly executions.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "monthly")]
    pub r#monthly: Box<Option<super::super::types::osconfig::PatchDeploymentRecurringScheduleMonthly>>,
    /// (Output)
    /// The time the next patch job is scheduled to run.
    /// A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
    #[builder(into, default)]
    #[serde(rename = "nextExecuteTime")]
    pub r#next_execute_time: Box<Option<String>>,
    /// The time that the recurring schedule becomes effective. Defaults to createTime of the patch deployment.
    /// A timestamp in RFC3339 UTC "Zulu" format, accurate to nanoseconds. Example: "2014-10-02T15:01:23.045123456Z".
    #[builder(into, default)]
    #[serde(rename = "startTime")]
    pub r#start_time: Box<Option<String>>,
    /// Time of the day to run a recurring deployment.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "timeOfDay")]
    pub r#time_of_day: Box<super::super::types::osconfig::PatchDeploymentRecurringScheduleTimeOfDay>,
    /// Defines the time zone that timeOfDay is relative to. The rules for daylight saving time are
    /// determined by the chosen time zone.
    /// Structure is documented below.
    #[builder(into)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Box<super::super::types::osconfig::PatchDeploymentRecurringScheduleTimeZone>,
    /// Schedule with weekly executions.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "weekly")]
    pub r#weekly: Box<Option<super::super::types::osconfig::PatchDeploymentRecurringScheduleWeekly>>,
}
