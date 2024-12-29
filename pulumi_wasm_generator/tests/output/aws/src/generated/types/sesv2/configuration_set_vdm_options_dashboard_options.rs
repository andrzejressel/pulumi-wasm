#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConfigurationSetVdmOptionsDashboardOptions {
    /// Specifies the status of your VDM engagement metrics collection. Valid values: `ENABLED`, `DISABLED`.
    #[builder(into, default)]
    #[serde(rename = "engagementMetrics")]
    pub r#engagement_metrics: Box<Option<String>>,
}
