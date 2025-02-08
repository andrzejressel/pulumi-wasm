#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct TableSchemaDefinition {
    /// The columns that are part of the clustering key of the table.
    #[builder(into, default)]
    #[serde(rename = "clusteringKeys")]
    pub r#clustering_keys: Box<Option<Vec<super::super::types::keyspaces::TableSchemaDefinitionClusteringKey>>>,
    /// The regular columns of the table.
    #[builder(into)]
    #[serde(rename = "columns")]
    pub r#columns: Box<Vec<super::super::types::keyspaces::TableSchemaDefinitionColumn>>,
    /// The columns that are part of the partition key of the table .
    #[builder(into)]
    #[serde(rename = "partitionKeys")]
    pub r#partition_keys: Box<Vec<super::super::types::keyspaces::TableSchemaDefinitionPartitionKey>>,
    /// The columns that have been defined as `STATIC`. Static columns store values that are shared by all rows in the same partition.
    #[builder(into, default)]
    #[serde(rename = "staticColumns")]
    pub r#static_columns: Box<Option<Vec<super::super::types::keyspaces::TableSchemaDefinitionStaticColumn>>>,
}
