#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct FleetDefaultClusterConfig {
    /// Enable/Disable binary authorization features for the cluster.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "binaryAuthorizationConfig")]
    pub r#binary_authorization_config: Box<Option<super::super::types::gkehub::FleetDefaultClusterConfigBinaryAuthorizationConfig>>,
    /// Enable/Disable Security Posture features for the cluster.
    /// Structure is documented below.
    #[builder(into, default)]
    #[serde(rename = "securityPostureConfig")]
    pub r#security_posture_config: Box<Option<super::super::types::gkehub::FleetDefaultClusterConfigSecurityPostureConfig>>,
}
