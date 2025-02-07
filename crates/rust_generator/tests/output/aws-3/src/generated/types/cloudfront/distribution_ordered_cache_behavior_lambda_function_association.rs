#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DistributionOrderedCacheBehaviorLambdaFunctionAssociation {
    /// Specific event to trigger this function. Valid values: `viewer-request`, `origin-request`, `viewer-response`, `origin-response`.
    #[builder(into)]
    #[serde(rename = "eventType")]
    pub r#event_type: Box<String>,
    /// When set to true it exposes the request body to the lambda function. Defaults to false. Valid values: `true`, `false`.
    #[builder(into, default)]
    #[serde(rename = "includeBody")]
    pub r#include_body: Box<Option<bool>>,
    /// ARN of the Lambda function.
    #[builder(into)]
    #[serde(rename = "lambdaArn")]
    pub r#lambda_arn: Box<String>,
}
