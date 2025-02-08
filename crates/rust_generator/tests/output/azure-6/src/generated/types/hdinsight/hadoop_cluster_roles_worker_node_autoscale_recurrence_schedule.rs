#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct HadoopClusterRolesWorkerNodeAutoscaleRecurrenceSchedule {
    /// The days of the week to perform autoscale. Possible values are `Monday`, `Tuesday`, `Wednesday`, `Thursday`, `Friday`, `Saturday` and `Sunday`.
    #[builder(into)]
    #[serde(rename = "days")]
    pub r#days: Box<Vec<String>>,
    /// The number of worker nodes to autoscale at the specified time.
    #[builder(into)]
    #[serde(rename = "targetInstanceCount")]
    pub r#target_instance_count: Box<i32>,
    /// The time of day to perform the autoscale in 24hour format.
    #[builder(into)]
    #[serde(rename = "time")]
    pub r#time: Box<String>,
}
