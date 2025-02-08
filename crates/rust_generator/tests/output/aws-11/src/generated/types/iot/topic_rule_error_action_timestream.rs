#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TopicRuleErrorActionTimestream {
    /// The name of an Amazon Timestream database.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: Box<String>,
    /// Configuration blocks with metadata attributes of the time series that are written in each measure record. Nested arguments below.
    #[builder(into)]
    #[serde(rename = "dimensions")]
    pub r#dimensions: Box<Vec<super::super::types::iot::TopicRuleErrorActionTimestreamDimension>>,
    /// The ARN of the role that grants permission to write to the Amazon Timestream database table.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// The name of the database table into which to write the measure records.
    #[builder(into)]
    #[serde(rename = "tableName")]
    pub r#table_name: Box<String>,
    /// Configuration block specifying an application-defined value to replace the default value assigned to the Timestream record's timestamp in the time column. Nested arguments below.
    #[builder(into, default)]
    #[serde(rename = "timestamp")]
    pub r#timestamp: Box<Option<super::super::types::iot::TopicRuleErrorActionTimestreamTimestamp>>,
}
