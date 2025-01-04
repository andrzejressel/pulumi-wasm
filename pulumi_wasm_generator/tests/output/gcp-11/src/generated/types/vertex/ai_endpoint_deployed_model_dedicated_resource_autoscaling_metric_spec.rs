#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AiEndpointDeployedModelDedicatedResourceAutoscalingMetricSpec {
    /// (Output)
    /// The resource metric name. Supported metrics: * For Online Prediction: * `aiplatform.googleapis.com/prediction/online/accelerator/duty_cycle` * `aiplatform.googleapis.com/prediction/online/cpu/utilization`
    #[builder(into, default)]
    #[serde(rename = "metricName")]
    pub r#metric_name: Box<Option<String>>,
    /// (Output)
    /// The target resource utilization in percentage (1% - 100%) for the given metric; once the real usage deviates from the target by a certain percentage, the machine replicas change. The default value is 60 (representing 60%) if not provided.
    #[builder(into, default)]
    #[serde(rename = "target")]
    pub r#target: Box<Option<i32>>,
}
