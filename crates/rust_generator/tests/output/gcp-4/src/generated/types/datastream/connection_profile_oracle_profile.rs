#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectionProfileOracleProfile {
    /// Connection string attributes
    #[builder(into, default)]
    #[serde(rename = "connectionAttributes")]
    pub r#connection_attributes: Box<Option<std::collections::HashMap<String, String>>>,
    /// Database for the Oracle connection.
    #[builder(into)]
    #[serde(rename = "databaseService")]
    pub r#database_service: Box<String>,
    /// Hostname for the Oracle connection.
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: Box<String>,
    /// Password for the Oracle connection.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// Port for the Oracle connection.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    /// Username for the Oracle connection.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
