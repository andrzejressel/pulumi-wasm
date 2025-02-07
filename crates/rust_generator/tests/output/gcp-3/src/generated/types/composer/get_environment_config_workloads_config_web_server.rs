#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetEnvironmentConfigWorkloadsConfigWebServer {
    /// CPU request and limit for Airflow web server.
    #[builder(into)]
    #[serde(rename = "cpu")]
    pub r#cpu: Box<f64>,
    /// Memory (GB) request and limit for Airflow web server.
    #[builder(into)]
    #[serde(rename = "memoryGb")]
    pub r#memory_gb: Box<f64>,
    /// Storage (GB) request and limit for Airflow web server.
    #[builder(into)]
    #[serde(rename = "storageGb")]
    pub r#storage_gb: Box<f64>,
}
