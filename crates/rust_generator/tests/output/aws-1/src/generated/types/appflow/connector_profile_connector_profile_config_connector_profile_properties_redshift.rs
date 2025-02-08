#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectorProfileConnectorProfileConfigConnectorProfilePropertiesRedshift {
    #[builder(into)]
    #[serde(rename = "bucketName")]
    pub r#bucket_name: Box<String>,
    #[builder(into, default)]
    #[serde(rename = "bucketPrefix")]
    pub r#bucket_prefix: Box<Option<String>>,
    /// The unique ID that's assigned to an Amazon Redshift cluster.
    #[builder(into, default)]
    #[serde(rename = "clusterIdentifier")]
    pub r#cluster_identifier: Box<Option<String>>,
    /// ARN of the IAM role that permits AppFlow to access the database through Data API.
    #[builder(into, default)]
    #[serde(rename = "dataApiRoleArn")]
    pub r#data_api_role_arn: Box<Option<String>>,
    /// The name of an Amazon Redshift database.
    #[builder(into, default)]
    #[serde(rename = "databaseName")]
    pub r#database_name: Box<Option<String>>,
    /// The JDBC URL of the Amazon Redshift cluster.
    #[builder(into, default)]
    #[serde(rename = "databaseUrl")]
    pub r#database_url: Box<Option<String>>,
    /// ARN of the IAM role.
    #[builder(into)]
    #[serde(rename = "roleArn")]
    pub r#role_arn: Box<String>,
}
