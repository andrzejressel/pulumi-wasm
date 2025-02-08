#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct TopicRuleErrorActionDynamodb {
    /// The hash key name.
    #[builder(into)]
    #[serde(rename = "hashKeyField")]
    pub r#hash_key_field: Box<String>,
    /// The hash key type. Valid values are "STRING" or "NUMBER".
    #[builder(into, default)]
    #[serde(rename = "hashKeyType")]
    pub r#hash_key_type: Box<Option<String>>,
    /// The hash key value.
    #[builder(into)]
    #[serde(rename = "hashKeyValue")]
    pub r#hash_key_value: Box<String>,
    /// The operation. Valid values are "INSERT", "UPDATE", or "DELETE".
    #[builder(into, default)]
    #[serde(rename = "operation")]
    pub r#operation: Box<Option<String>>,
    /// The action payload.
    #[builder(into, default)]
    #[serde(rename = "payloadField")]
    pub r#payload_field: Box<Option<String>>,
    /// The range key name.
    #[builder(into, default)]
    #[serde(rename = "rangeKeyField")]
    pub r#range_key_field: Box<Option<String>>,
    /// The range key type. Valid values are "STRING" or "NUMBER".
    #[builder(into, default)]
    #[serde(rename = "rangeKeyType")]
    pub r#range_key_type: Box<Option<String>>,
    /// The range key value.
    #[builder(into, default)]
    #[serde(rename = "rangeKeyValue")]
    pub r#range_key_value: Box<Option<String>>,
    /// The ARN of the IAM role that grants access to the DynamoDB table.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// The name of the DynamoDB table.
    #[builder(into)]
    #[serde(rename = "tableName")]
    pub r#table_name: Box<String>,
}
