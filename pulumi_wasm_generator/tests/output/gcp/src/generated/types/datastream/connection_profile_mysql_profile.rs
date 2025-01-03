#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectionProfileMysqlProfile {
    /// Hostname for the MySQL connection.
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: Box<String>,
    /// Password for the MySQL connection.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into)]
    #[serde(rename = "password")]
    pub r#password: Box<String>,
    /// Port for the MySQL connection.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    /// SSL configuration for the MySQL connection.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "sslConfig")]
    pub r#ssl_config: Box<Option<super::super::types::datastream::ConnectionProfileMysqlProfileSslConfig>>,
    /// Username for the MySQL connection.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
