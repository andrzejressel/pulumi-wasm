#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicyTargetTrackingConfiguration {
    /// Customized metric. Conflicts with `predefined_metric_specification`.
    #[builder(into, default)]
    #[serde(rename = "customizedMetricSpecification")]
    pub r#customized_metric_specification: Box<Option<super::super::types::autoscaling::PolicyTargetTrackingConfigurationCustomizedMetricSpecification>>,
    /// Whether scale in by the target tracking policy is disabled.
    #[builder(into, default)]
    #[serde(rename = "disableScaleIn")]
    pub r#disable_scale_in: Box<Option<bool>>,
    /// Predefined metric. Conflicts with `customized_metric_specification`.
    #[builder(into, default)]
    #[serde(rename = "predefinedMetricSpecification")]
    pub r#predefined_metric_specification: Box<Option<super::super::types::autoscaling::PolicyTargetTrackingConfigurationPredefinedMetricSpecification>>,
    /// Target value for the metric.
    #[builder(into)]
    #[serde(rename = "targetValue")]
    pub r#target_value: Box<f64>,
}
