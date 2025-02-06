#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct ClusterClientAuthentication {
    /// Configuration block for specifying SASL client authentication. See below.
    #[builder(into, default)]
    #[serde(rename = "sasl")]
    pub r#sasl: Box<Option<super::super::types::msk::ClusterClientAuthenticationSasl>>,
    /// Configuration block for specifying TLS client authentication. See below.
    #[builder(into, default)]
    #[serde(rename = "tls")]
    pub r#tls: Box<Option<super::super::types::msk::ClusterClientAuthenticationTls>>,
    /// Enables unauthenticated access.
    #[builder(into, default)]
    #[serde(rename = "unauthenticated")]
    pub r#unauthenticated: Box<Option<bool>>,
}
