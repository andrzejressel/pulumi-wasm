#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetMetastoreServiceScalingConfig {
    /// Represents the autoscaling configuration of a metastore service.
    #[builder(into)]
    #[serde(rename = "autoscalingConfigs")]
    pub r#autoscaling_configs: Box<Vec<super::super::types::dataproc::GetMetastoreServiceScalingConfigAutoscalingConfig>>,
    /// Metastore instance sizes. Possible values: ["EXTRA_SMALL", "SMALL", "MEDIUM", "LARGE", "EXTRA_LARGE"]
    #[builder(into)]
    #[serde(rename = "instanceSize")]
    pub r#instance_size: Box<String>,
    /// Scaling factor, in increments of 0.1 for values less than 1.0, and increments of 1.0 for values greater than 1.0.
    #[builder(into)]
    #[serde(rename = "scalingFactor")]
    pub r#scaling_factor: Box<f64>,
}
