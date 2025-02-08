#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EnvironmentConfigWorkloadsConfigWebServer {
    /// CPU request and limit for Airflow web server.
    #[builder(into, default)]
    #[serde(rename = "cpu")]
    pub r#cpu: Box<Option<f64>>,
    /// Memory (GB) request and limit for Airflow web server.
    #[builder(into, default)]
    #[serde(rename = "memoryGb")]
    pub r#memory_gb: Box<Option<f64>>,
    /// Storage (GB) request and limit for Airflow web server.
    #[builder(into, default)]
    #[serde(rename = "storageGb")]
    pub r#storage_gb: Box<Option<f64>>,
}
