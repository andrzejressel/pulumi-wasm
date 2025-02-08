#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct EndpointKinesisSettings {
    /// Shows detailed control information for table definition, column definition, and table and column changes in the Kinesis message output. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "includeControlDetails")]
    pub r#include_control_details: Box<Option<bool>>,
    /// Include NULL and empty columns in the target. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "includeNullAndEmpty")]
    pub r#include_null_and_empty: Box<Option<bool>>,
    /// Shows the partition value within the Kinesis message output, unless the partition type is schema-table-type. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "includePartitionValue")]
    pub r#include_partition_value: Box<Option<bool>>,
    /// Includes any data definition language (DDL) operations that change the table in the control data. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "includeTableAlterOperations")]
    pub r#include_table_alter_operations: Box<Option<bool>>,
    /// Provides detailed transaction information from the source database. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "includeTransactionDetails")]
    pub r#include_transaction_details: Box<Option<bool>>,
    /// Output format for the records created. Default is `json`. Valid values are `json` and `json-unformatted` (a single line with no tab).
    #[builder(into, default)]
    #[serde(rename = "messageFormat")]
    pub r#message_format: Box<Option<String>>,
    /// Prefixes schema and table names to partition values, when the partition type is primary-key-type. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "partitionIncludeSchemaTable")]
    pub r#partition_include_schema_table: Box<Option<bool>>,
    /// ARN of the IAM Role with permissions to write to the Kinesis data stream.
    #[builder(into, default)]
    #[serde(rename = "serviceAccessRoleArn")]
    pub r#service_access_role_arn: Box<Option<String>>,
    /// ARN of the Kinesis data stream.
    #[builder(into, default)]
    #[serde(rename = "streamArn")]
    pub r#stream_arn: Box<Option<String>>,
}
