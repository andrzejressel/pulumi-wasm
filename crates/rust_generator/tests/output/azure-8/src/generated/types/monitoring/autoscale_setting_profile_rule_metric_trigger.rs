#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct AutoscaleSettingProfileRuleMetricTrigger {
    /// One or more `dimensions` block as defined below.
    #[builder(into, default)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Box<Option<Vec<super::super::types::monitoring::AutoscaleSettingProfileRuleMetricTriggerDimension>>>,
    /// Whether to enable metric divide by instance count.
    #[builder(into, default)]
    #[serde(rename = "divideByInstanceCount")]
    pub r#divide_by_instance_count: Box<Option<bool>>,
    /// The name of the metric that defines what the rule monitors, such as `Percentage CPU` for `Virtual Machine Scale Sets` and `CpuPercentage` for `App Service Plan`.
    /// 
    /// > **NOTE:** The allowed value of `metric_name` highly depends on the targeting resource type, please visit [Supported metrics with Azure Monitor](https://docs.microsoft.com/azure/azure-monitor/platform/metrics-supported) for more details.
    #[builder(into)]
    #[serde(rename = "metricName")]
    pub r#metric_name: Box<String>,
    /// The namespace of the metric that defines what the rule monitors, such as `microsoft.compute/virtualmachinescalesets` for `Virtual Machine Scale Sets`.
    #[builder(into, default)]
    #[serde(rename = "metricNamespace")]
    pub r#metric_namespace: Box<Option<String>>,
    /// The ID of the Resource which the Rule monitors.
    #[builder(into)]
    #[serde(rename = "metricResourceId")]
    pub r#metric_resource_id: Box<String>,
    /// Specifies the operator used to compare the metric data and threshold. Possible values are: `Equals`, `NotEquals`, `GreaterThan`, `GreaterThanOrEqual`, `LessThan`, `LessThanOrEqual`.
    #[builder(into)]
    #[serde(rename = "operator")]
    pub r#operator: Box<String>,
    /// Specifies how the metrics from multiple instances are combined. Possible values are `Average`, `Max`, `Min` and `Sum`.
    #[builder(into)]
    #[serde(rename = "statistic")]
    pub r#statistic: Box<String>,
    /// Specifies the threshold of the metric that triggers the scale action.
    #[builder(into)]
    #[serde(rename = "threshold")]
    pub r#threshold: Box<f64>,
    /// Specifies how the data that's collected should be combined over time. Possible values include `Average`, `Count`, `Maximum`, `Minimum`, `Last` and `Total`.
    #[builder(into)]
    #[serde(rename = "timeAggregation")]
    pub r#time_aggregation: Box<String>,
    /// Specifies the granularity of metrics that the rule monitors, which must be one of the pre-defined values returned from the metric definitions for the metric. This value must be between 1 minute and 12 hours an be formatted as an ISO 8601 string.
    #[builder(into)]
    #[serde(rename = "timeGrain")]
    pub r#time_grain: Box<String>,
    /// Specifies the time range for which data is collected, which must be greater than the delay in metric collection (which varies from resource to resource). This value must be between 5 minutes and 12 hours and be formatted as an ISO 8601 string.
    #[builder(into)]
    #[serde(rename = "timeWindow")]
    pub r#time_window: Box<String>,
}
