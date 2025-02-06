#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct BareMetalClusterClusterOperations {
    /// Whether collection of application logs/metrics should be enabled (in addition to system logs/metrics).
    #[builder(into, default)]
    #[serde(rename = "enableApplicationLogs")]
    pub r#enable_application_logs: Box<Option<bool>>,
}
