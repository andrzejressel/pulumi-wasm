#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ConnectionProfileForwardSshConnectivity {
    /// Hostname for the SSH tunnel.
    #[builder(into)]
    #[serde(rename = "hostname")]
    pub r#hostname: Box<String>,
    /// SSH password.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into, default)]
    #[serde(rename = "password")]
    pub r#password: Box<Option<String>>,
    /// Port for the SSH tunnel.
    #[builder(into, default)]
    #[serde(rename = "port")]
    pub r#port: Box<Option<i32>>,
    /// SSH private key.
    /// **Note**: This property is sensitive and will not be displayed in the plan.
    #[builder(into, default)]
    #[serde(rename = "privateKey")]
    pub r#private_key: Box<Option<String>>,
    /// Username for the SSH tunnel.
    #[builder(into)]
    #[serde(rename = "username")]
    pub r#username: Box<String>,
}
