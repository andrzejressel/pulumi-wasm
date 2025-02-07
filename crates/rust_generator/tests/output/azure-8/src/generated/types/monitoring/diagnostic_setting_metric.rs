#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DiagnosticSettingMetric {
    /// The name of a Diagnostic Metric Category for this Resource.
    /// 
    /// > **NOTE:** The Metric Categories available vary depending on the Resource being used. You may wish to use the `azure.monitoring.getDiagnosticCategories` Data Source to identify which categories are available for a given Resource.
    #[builder(into)]
    #[serde(rename = "category")]
    pub r#category: Box<String>,
    /// Is this Diagnostic Metric enabled? Defaults to `true`.
    #[builder(into, default)]
    #[serde(rename = "enabled")]
    pub r#enabled: Box<Option<bool>>,
    #[builder(into, default)]
    #[serde(rename = "retentionPolicy")]
    pub r#retention_policy: Box<Option<super::super::types::monitoring::DiagnosticSettingMetricRetentionPolicy>>,
}
