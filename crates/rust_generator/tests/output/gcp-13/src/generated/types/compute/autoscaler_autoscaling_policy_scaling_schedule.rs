#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AutoscalerAutoscalingPolicyScalingSchedule {
    /// An optional description of this resource.
    #[builder(into, default)]
    #[serde(rename = "description")]
    pub r#description: Box<Option<String>>,
    /// A boolean value that specifies if a scaling schedule can influence autoscaler recommendations. If set to true, then a scaling schedule has no effect.
    #[builder(into, default)]
    #[serde(rename = "disabled")]
    pub r#disabled: Box<Option<bool>>,
    /// The duration of time intervals (in seconds) for which this scaling schedule will be running. The minimum allowed value is 300.
    #[builder(into)]
    #[serde(rename = "durationSec")]
    pub r#duration_sec: Box<i32>,
    /// Minimum number of VM instances that autoscaler will recommend in time intervals starting according to schedule.
    #[builder(into)]
    #[serde(rename = "minRequiredReplicas")]
    pub r#min_required_replicas: Box<i32>,
    /// The identifier for this object. Format specified above.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// The start timestamps of time intervals when this scaling schedule should provide a scaling signal. This field uses the extended cron format (with an optional year field).
    #[builder(into)]
    #[serde(rename = "schedule")]
    pub r#schedule: Box<String>,
    /// The time zone to be used when interpreting the schedule. The value of this field must be a time zone name from the tz database: http://en.wikipedia.org/wiki/Tz_database.
    #[builder(into, default)]
    #[serde(rename = "timeZone")]
    pub r#time_zone: Box<Option<String>>,
}
