#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectionProfilePostgresql {
    /// If the connected database is an AlloyDB instance, use this field to provide the AlloyDB cluster ID.
    #[builder(into, default)]
    #[serde(rename = "alloydbClusterId")]
    pub r#alloydb_cluster_id: Box<Option<String>>,
    /// If the source is a Cloud SQL database, use this field to provide the Cloud SQL instance ID of the source.
    #[builder(into, default)]
    #[serde(rename = "cloudSqlId")]
    pub r#cloud_sql_id: Box<Option<String>>,
    /// The IP or hostname of the source MySQL database.
    #[builder(into, default)]
    #[serde(rename = "host")]
    pub r#host: Box<Option<String>>,
    /// (Output)
    /// Output only. If the source is a Cloud SQL database, this field indicates the network architecture it's associated with.
    #[builder(into, default)]
    #[serde(rename = "networkArchitecture")]
    pub r#network_architecture: Box<Option<String>>,
    /// Input only. The password for the user that Database Migration Service will be using to connect to the database.
    /// This field is not returned on request, and the value is encrypted when stored in Database Migration Service.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into, default)]
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// (Output)
    /// Output only. Indicates If this connection profile password is stored.
    #[builder(into, default)]
    #[serde(rename = "passwordSet")]
    pub r#password_set: Box<Option<bool>>,
    /// The network port of the source MySQL database.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    /// SSL configuration for the destination to connect to the source database.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "ssl")]
    pub r#ssl: Box<Option<super::super::types::databasemigrationservice::ConnectionProfilePostgresqlSsl>>,
    /// The username that Database Migration Service will use to connect to the database. The value is encrypted when stored in Database Migration Service.
    #[builder(into, default)]
    #[serde(rename = "username")]
    pub r#username: Box<Option<String>>,
}
