#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct CassandraTableSchema {
    /// One or more `cluster_key` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "clusterKeys")]
    pub r#cluster_keys: Box<Option<Vec<super::super::types::cosmosdb::CassandraTableSchemaClusterKey>>>,
    /// One or more `column` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "columns")]
    pub r#columns: Box<Vec<super::super::types::cosmosdb::CassandraTableSchemaColumn>>,
    /// One or more `partition_key` blocks as defined below.
    #[builder(into)]
    #[serde(rename = "partitionKeys")]
    pub r#partition_keys: Box<Vec<super::super::types::cosmosdb::CassandraTableSchemaPartitionKey>>,
}
