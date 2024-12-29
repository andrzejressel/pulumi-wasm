#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FirehoseDeliveryStreamIcebergConfigurationDestinationTableConfiguration {
    /// The name of the Apache Iceberg database.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: Box<String>,
    /// The table specific S3 error output prefix. All the errors that occurred while delivering to this table will be prefixed with this value in S3 destination.
    #[builder(into, default)]
    #[serde(rename = "s3ErrorOutputPrefix")]
    pub r#s_3_error_output_prefix: Box<Option<String>>,
    /// The name of the Apache Iceberg Table.
    #[builder(into)]
    #[serde(rename = "tableName")]
    pub r#table_name: Box<String>,
    /// A list of unique keys for a given Apache Iceberg table. Firehose will use these for running Create, Update, or Delete operations on the given Iceberg table.
    #[builder(into, default)]
    #[serde(rename = "uniqueKeys")]
    pub r#unique_keys: Box<Option<Vec<String>>>,
}
