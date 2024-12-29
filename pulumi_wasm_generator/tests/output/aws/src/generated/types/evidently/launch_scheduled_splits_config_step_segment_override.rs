#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct LaunchScheduledSplitsConfigStepSegmentOverride {
    /// Specifies a number indicating the order to use to evaluate segment overrides, if there are more than one. Segment overrides with lower numbers are evaluated first.
    #[builder(into)]
    #[serde(rename = "evaluationOrder")]
    pub r#evaluation_order: Box<i32>,
    /// The name or ARN of the segment to use.
    #[builder(into)]
    #[serde(rename = "segment")]
    pub r#segment: Box<String>,
    /// The traffic allocation percentages among the feature variations to assign to this segment. This is a set of key-value pairs. The keys are variation names. The values represent the amount of traffic to allocate to that variation for this segment. This is expressed in thousandths of a percent, so a weight of 50000 represents 50% of traffic.
    #[builder(into)]
    #[serde(rename = "weights")]
    pub r#weights: Box<std::collections::HashMap<String, i32>>,
}
