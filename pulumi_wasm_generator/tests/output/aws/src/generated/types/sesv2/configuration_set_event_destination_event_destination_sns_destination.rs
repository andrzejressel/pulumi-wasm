#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConfigurationSetEventDestinationEventDestinationSnsDestination {
    /// The Amazon Resource Name (ARN) of the Amazon SNS topic to publish email events to.
    #[builder(into)]
    #[serde(rename = "topicArn")]
    pub r#topic_arn: Box<String>,
}