#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectionProfileOracle {
    /// Required. Database service for the Oracle connection.
    #[builder(into)]
    #[serde(rename = "databaseService")]
    pub r#database_service: Box<String>,
    /// SSL configuration for the destination to connect to the source database.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "forwardSshConnectivity")]
    pub r#forward_ssh_connectivity: Box<Option<super::super::types::databasemigrationservice::ConnectionProfileOracleForwardSshConnectivity>>,
    /// Required. The IP or hostname of the source Oracle database.
    #[builder(into)]
    #[serde(rename = "host")]
    pub r#host: Box<String>,
    /// Required. Input only. The password for the user that Database Migration Service will be using to connect to the database.
    /// This field is not returned on request, and the value is encrypted when stored in Database Migration Service.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// (Output)
    /// Output only. Indicates If this connection profile password is stored.
    #[builder(into, default)]
    #[serde(rename = "passwordSet")]
    pub r#password_set: Box<Option<bool>>,
    /// Required. The network port of the source Oracle database.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
    /// Configuration for using a private network to communicate with the source database
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "privateConnectivity")]
    pub r#private_connectivity: Box<Option<super::super::types::databasemigrationservice::ConnectionProfileOraclePrivateConnectivity>>,
    /// SSL configuration for the destination to connect to the source database.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "ssl")]
    pub r#ssl: Box<Option<super::super::types::databasemigrationservice::ConnectionProfileOracleSsl>>,
    /// This object has no nested fields.
    /// Static IP address connectivity configured on service project.
    #[builder(into, default)]
    #[serde(rename = "staticServiceIpConnectivity")]
    pub r#static_service_ip_connectivity: Box<Option<super::super::types::databasemigrationservice::ConnectionProfileOracleStaticServiceIpConnectivity>>,
    /// Required. The username that Database Migration Service will use to connect to the database. The value is encrypted when stored in Database Migration Service.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
