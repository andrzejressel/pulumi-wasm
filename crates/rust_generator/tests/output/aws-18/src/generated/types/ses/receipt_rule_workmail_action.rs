#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ReceiptRuleWorkmailAction {
    /// The ARN of the WorkMail organization
    #[builder(into)]
    #[serde(rename = "organizationArn")]
    pub r#organization_arn: Box<String>,
    /// The position of the action in the receipt rule
    #[builder(into)]
    #[serde(rename = "position")]
    pub r#position: Box<i32>,
    /// The ARN of an SNS topic to notify
    #[builder(into, default)]
    #[serde(rename = "topicArn")]
    pub r#topic_arn: Box<Option<String>>,
}
