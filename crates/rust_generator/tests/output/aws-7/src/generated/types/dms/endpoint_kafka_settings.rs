#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct EndpointKafkaSettings {
    /// Kafka broker location. Specify in the form broker-hostname-or-ip:port.
    #[builder(into)]
    #[serde(rename = "broker")]
    pub r#broker: Box<String>,
    /// Shows detailed control information for table definition, column definition, and table and column changes in the Kafka message output. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "includeControlDetails")]
    pub r#include_control_details: Box<Option<bool>>,
    /// Include NULL and empty columns for records migrated to the endpoint. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "includeNullAndEmpty")]
    pub r#include_null_and_empty: Box<Option<bool>>,
    /// Shows the partition value within the Kafka message output unless the partition type is `schema-table-type`. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "includePartitionValue")]
    pub r#include_partition_value: Box<Option<bool>>,
    /// Includes any data definition language (DDL) operations that change the table in the control data, such as `rename-table`, `drop-table`, `add-column`, `drop-column`, and `rename-column`. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "includeTableAlterOperations")]
    pub r#include_table_alter_operations: Box<Option<bool>>,
    /// Provides detailed transaction information from the source database. This information includes a commit timestamp, a log position, and values for `transaction_id`, previous `transaction_id`, and `transaction_record_id` (the record offset within a transaction). Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "includeTransactionDetails")]
    pub r#include_transaction_details: Box<Option<bool>>,
    /// Output format for the records created on the endpoint. Message format is `JSON` (default) or `JSON_UNFORMATTED` (a single line with no tab).
    #[builder(into, default)]
    #[serde(rename = "messageFormat")]
    pub r#message_format: Box<Option<String>>,
    /// Maximum size in bytes for records created on the endpoint Default is `1,000,000`.
    #[builder(into, default)]
    #[serde(rename = "messageMaxBytes")]
    pub r#message_max_bytes: Box<Option<i32>>,
    /// Set this optional parameter to true to avoid adding a '0x' prefix to raw data in hexadecimal format. For example, by default, AWS DMS adds a '0x' prefix to the LOB column type in hexadecimal format moving from an Oracle source to a Kafka target. Use the `no_hex_prefix` endpoint setting to enable migration of RAW data type columns without adding the `'0x'` prefix.
    #[builder(into, default)]
    #[serde(rename = "noHexPrefix")]
    pub r#no_hex_prefix: Box<Option<bool>>,
    /// Prefixes schema and table names to partition values, when the partition type is `primary-key-type`. Doing this increases data distribution among Kafka partitions. For example, suppose that a SysBench schema has thousands of tables and each table has only limited range for a primary key. In this case, the same primary key is sent from thousands of tables to the same partition, which causes throttling. Default is `false`.
    #[builder(into, default)]
    #[serde(rename = "partitionIncludeSchemaTable")]
    pub r#partition_include_schema_table: Box<Option<bool>>,
    /// Secure password you created when you first set up your MSK cluster to validate a client identity and make an encrypted connection between server and client using SASL-SSL authentication.
    #[builder(into, default)]
    #[serde(rename = "saslPassword")]
    pub r#sasl_password: Box<Option<String>>,
    /// Secure user name you created when you first set up your MSK cluster to validate a client identity and make an encrypted connection between server and client using SASL-SSL authentication.
    #[builder(into, default)]
    #[serde(rename = "saslUsername")]
    pub r#sasl_username: Box<Option<String>>,
    /// Set secure connection to a Kafka target endpoint using Transport Layer Security (TLS). Options include `ssl-encryption`, `ssl-authentication`, and `sasl-ssl`. `sasl-ssl` requires `sasl_username` and `sasl_password`.
    #[builder(into, default)]
    #[serde(rename = "securityProtocol")]
    pub r#security_protocol: Box<Option<String>>,
    /// ARN for the private certificate authority (CA) cert that AWS DMS uses to securely connect to your Kafka target endpoint.
    #[builder(into, default)]
    #[serde(rename = "sslCaCertificateArn")]
    pub r#ssl_ca_certificate_arn: Box<Option<String>>,
    /// ARN of the client certificate used to securely connect to a Kafka target endpoint.
    #[builder(into, default)]
    #[serde(rename = "sslClientCertificateArn")]
    pub r#ssl_client_certificate_arn: Box<Option<String>>,
    /// ARN for the client private key used to securely connect to a Kafka target endpoint.
    #[builder(into, default)]
    #[serde(rename = "sslClientKeyArn")]
    pub r#ssl_client_key_arn: Box<Option<String>>,
    /// Password for the client private key used to securely connect to a Kafka target endpoint.
    #[builder(into, default)]
    #[serde(rename = "sslClientKeyPassword")]
    pub r#ssl_client_key_password: Box<Option<String>>,
    /// Kafka topic for migration. Default is `kafka-default-topic`.
    #[builder(into, default)]
    #[serde(rename = "topic")]
    pub r#topic: Box<Option<String>>,
}
