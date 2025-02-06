#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct EnvironmentConfigWorkloadsConfigWorker {
    /// CPU request and limit for a single Airflow worker replica.
    #[builder(into, default)]
    #[serde(rename = "cpu")]
    pub r#cpu: Box<Option<f64>>,
    /// Maximum number of workers for autoscaling.
    #[builder(into, default)]
    #[serde(rename = "maxCount")]
    pub r#max_count: Box<Option<i32>>,
    /// Memory (GB) request and limit for a single Airflow worker replica.
    #[builder(into, default)]
    #[serde(rename = "memoryGb")]
    pub r#memory_gb: Box<Option<f64>>,
    /// Minimum number of workers for autoscaling.
    #[builder(into, default)]
    #[serde(rename = "minCount")]
    pub r#min_count: Box<Option<i32>>,
    /// Storage (GB) request and limit for a single Airflow worker replica.
    #[builder(into, default)]
    #[serde(rename = "storageGb")]
    pub r#storage_gb: Box<Option<f64>>,
}
