#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DistributionOrderedCacheBehaviorFunctionAssociation {
    /// Specific event to trigger this function. Valid values: `viewer-request` or `viewer-response`.
    #[builder(into)]
    #[serde(rename = "eventType")]
    pub r#event_type: Box<String>,
    /// ARN of the CloudFront function.
    #[builder(into)]
    #[serde(rename = "functionArn")]
    pub r#function_arn: Box<String>,
}