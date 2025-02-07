#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ConnectionProfileOracleForwardSshConnectivity {
    /// Required. Hostname for the SSH tunnel.
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: Box<String>,
    /// Input only. SSH password. Only one of `password` and `private_key` can be configured.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into, default)]
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// Port for the SSH tunnel, default value is 22.
    #[builder(into)]
    #[serde(rename = "port")]
    pub r#port: Box<i32>,
    /// Input only. SSH private key. Only one of `password` and `private_key` can be configured.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into, default)]
    #[serde(rename = "privateKey")]
    pub r#private_key: Box<Option<String>>,
    /// Required. Username for the SSH tunnel.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
