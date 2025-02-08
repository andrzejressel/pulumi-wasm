#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct WorkflowTemplateJobScheduling {
    /// Maximum number of times per hour a driver may be restarted as a result of driver exiting with non-zero code before job is reported failed. A job may be reported as thrashing if driver exits with non-zero code 4 times within 10 minute window. Maximum value is 10.
    #[builder(into, default)]
    #[serde(rename = "maxFailuresPerHour")]
    pub r#max_failures_per_hour: Box<Option<i32>>,
    /// Maximum number of times in total a driver may be restarted as a result of driver exiting with non-zero code before job is reported failed. Maximum value is 240
    #[builder(into, default)]
    #[serde(rename = "maxFailuresTotal")]
    pub r#max_failures_total: Box<Option<i32>>,
}
