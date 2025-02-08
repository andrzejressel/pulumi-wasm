#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct DataSourceRelationalDatabaseConfigHttpEndpointConfig {
    /// AWS secret store ARN for database credentials.
    #[builder(into)]
    #[serde(rename = "awsSecretStoreArn")]
    pub r#aws_secret_store_arn: Box<String>,
    /// Logical database name.
    #[builder(into, default)]
    #[serde(rename = "databaseName")]
    pub r#database_name: Box<Option<String>>,
    /// Amazon RDS cluster identifier.
    #[builder(into)]
    #[serde(rename = "dbClusterIdentifier")]
    pub r#db_cluster_identifier: Box<String>,
    /// AWS Region for RDS HTTP endpoint. Defaults to current region.
    #[builder(into, default)]
    #[serde(rename = "region")]
    pub r#region: Box<Option<String>>,
    /// Logical schema name.
    #[builder(into, default)]
    #[serde(rename = "schema")]
    pub r#schema: Box<Option<String>>,
}
