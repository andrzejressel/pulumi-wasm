#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct FlowMetadataCatalogConfigGlueDataCatalog {
    /// The name of an existing Glue database to store the metadata tables that Amazon AppFlow creates.
    #[builder(into)]
    #[serde(rename = "databaseName")]
    pub r#database_name: Box<String>,
    /// The ARN of an IAM role that grants AppFlow the permissions it needs to create Data Catalog tables, databases, and partitions.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
    /// A naming prefix for each Data Catalog table that Amazon AppFlow creates
    #[builder(into)]
    #[serde(rename = "tablePrefix")]
    pub r#table_prefix: Box<String>,
}
