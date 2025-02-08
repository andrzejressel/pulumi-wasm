#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct ConnectionProfileCloudsql {
    /// (Output)
    /// Output only. The Cloud SQL instance ID that this connection profile is associated with.
    #[builder(into, default)]
    #[serde(rename = "cloudSqlId")]
    pub r#cloud_sql_id: Box<Option<String>>,
    /// (Output)
    /// Output only. The Cloud SQL database instance's private IP.
    #[builder(into, default)]
    #[serde(rename = "privateIp")]
    pub r#private_ip: Box<Option<String>>,
    /// (Output)
    /// Output only. The Cloud SQL database instance's public IP.
    #[builder(into, default)]
    #[serde(rename = "publicIp")]
    pub r#public_ip: Box<Option<String>>,
    /// Immutable. Metadata used to create the destination Cloud SQL database.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "settings")]
    pub r#settings: Box<Option<super::super::types::databasemigrationservice::ConnectionProfileCloudsqlSettings>>,
}
