#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct IntentFulfillmentActivityCodeHook {
    /// The version of the request-response that you want Amazon Lex to use
    /// to invoke your Lambda function. For more information, see
    /// [Using Lambda Functions](https://docs.aws.amazon.com/lex/latest/dg/using-lambda.html). Must be less than or equal to 5 characters in length.
    #[builder(into)]
    #[serde(rename = "messageVersion")]
    pub r#message_version: Box<String>,
    /// The Amazon Resource Name (ARN) of the Lambda function.
    #[builder(into)]
    #[serde(rename = "uri")]
    pub r#uri: Box<String>,
}
