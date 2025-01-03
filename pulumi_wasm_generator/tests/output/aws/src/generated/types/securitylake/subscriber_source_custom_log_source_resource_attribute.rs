#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct SubscriberSourceCustomLogSourceResourceAttribute {
    /// The ARN of the AWS Glue crawler.
    #[builder(into)]
    #[serde(rename = "crawlerArn")]
    pub r#crawler_arn: Box<String>,
    /// The ARN of the AWS Glue database where results are written.
    #[builder(into)]
    #[serde(rename = "databaseArn")]
    pub r#database_arn: Box<String>,
    /// The ARN of the AWS Glue table.
    #[builder(into)]
    #[serde(rename = "tableArn")]
    pub r#table_arn: Box<String>,
}
