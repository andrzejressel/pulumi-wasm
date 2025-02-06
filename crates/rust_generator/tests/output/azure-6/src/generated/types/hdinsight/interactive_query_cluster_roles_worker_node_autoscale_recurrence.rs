#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InteractiveQueryClusterRolesWorkerNodeAutoscaleRecurrence {
    /// A list of `schedule` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "schedules")]
    pub r#schedules: Box<Vec<super::super::types::hdinsight::InteractiveQueryClusterRolesWorkerNodeAutoscaleRecurrenceSchedule>>,
    /// The time zone for the autoscale schedule times.
    #[builder(into)]
    #[serde(rename = "timezone")]
    pub r#timezone: Box<String>,
}
