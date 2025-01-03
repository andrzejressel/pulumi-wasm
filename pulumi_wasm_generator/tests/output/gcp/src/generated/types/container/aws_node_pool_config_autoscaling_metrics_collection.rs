#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AwsNodePoolConfigAutoscalingMetricsCollection {
    /// The frequency at which EC2 Auto Scaling sends aggregated data to AWS CloudWatch. The only valid value is "1Minute".
    #[builder(into)]
    #[serde(rename = "granularity")]
    pub r#granularity: Box<String>,
    /// The metrics to enable. For a list of valid metrics, see https://docs.aws.amazon.com/autoscaling/ec2/APIReference/API_EnableMetricsCollection.html. If you specify granularity and don't specify any metrics, all metrics are enabled.
    #[builder(into, default)]
    #[serde(rename = "metrics")]
    pub r#metrics: Box<Option<Vec<String>>>,
}
