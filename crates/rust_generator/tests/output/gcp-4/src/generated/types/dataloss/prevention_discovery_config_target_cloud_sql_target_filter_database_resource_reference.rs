#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct PreventionDiscoveryConfigTargetCloudSqlTargetFilterDatabaseResourceReference {
    /// Required. Name of a database within the instance.
    #[builder(into)]
    #[serde(rename = "database")]
    pub r#database: Box<String>,
    /// Required. Name of a database resource, for example, a table within the database.
    #[builder(into)]
    #[serde(rename = "databaseResource")]
    pub r#database_resource: Box<String>,
    /// Required. The instance where this resource is located. For example: Cloud SQL instance ID.
    #[builder(into)]
    #[serde(rename = "instance")]
    pub r#instance: Box<String>,
    /// Required. If within a project-level config, then this must match the config's project ID.
    #[builder(into)]
    #[serde(rename = "projectId")]
    pub r#project_id: Box<String>,
}
