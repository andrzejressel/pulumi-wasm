#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EnvironmentConfigWorkloadsConfigTriggerer {
    /// The number of triggerers.
    #[builder(into)]
    #[serde(rename = "count")]
    pub r#count: Box<i32>,
    /// CPU request and limit for a single Airflow triggerer replica.
    #[builder(into)]
    #[serde(rename = "cpu")]
    pub r#cpu: Box<f64>,
    /// Memory (GB) request and limit for a single Airflow triggerer replica.
    #[builder(into)]
    #[serde(rename = "memoryGb")]
    pub r#memory_gb: Box<f64>,
}
