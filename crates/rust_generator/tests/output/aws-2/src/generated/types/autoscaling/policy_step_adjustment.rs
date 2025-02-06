#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PolicyStepAdjustment {
    /// Lower bound for the
    /// difference between the alarm threshold and the CloudWatch metric.
    /// Without a value, AWS will treat this bound as negative infinity.
    #[builder(into, default)]
    #[serde(rename = "metricIntervalLowerBound")]
    pub r#metric_interval_lower_bound: Box<Option<String>>,
    /// Upper bound for the
    /// difference between the alarm threshold and the CloudWatch metric.
    /// Without a value, AWS will treat this bound as positive infinity. The upper bound
    /// must be greater than the lower bound.
    /// 
    /// Notice the bounds are **relative** to the alarm threshold, meaning that the starting point is not 0%, but the alarm threshold. Check the official [docs](https://docs.aws.amazon.com/autoscaling/ec2/userguide/as-scaling-simple-step.html#as-scaling-steps) for a detailed example.
    /// 
    /// The following arguments are only available to "TargetTrackingScaling" type policies:
    #[builder(into, default)]
    #[serde(rename = "metricIntervalUpperBound")]
    pub r#metric_interval_upper_bound: Box<Option<String>>,
    /// Number of members by which to
    /// scale, when the adjustment bounds are breached. A positive value scales
    /// up. A negative value scales down.
    #[builder(into)]
    #[serde(rename = "scalingAdjustment")]
    pub r#scaling_adjustment: Box<i32>,
}
